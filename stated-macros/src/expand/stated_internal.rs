use std::mem;

use itertools::Itertools;
use proc_macro2::TokenStream as TokenStream2;
use syn::{
    Error, Expr, ExprCall, ExprPath, ExprStruct, Fields, FieldsNamed, FieldsUnnamed, ImplItem,
    ImplItemFn, ItemImpl, ItemStruct, Meta, Path, Result, Token, Type,
    parse::Parser,
    punctuated::Punctuated,
    spanned::Spanned,
    visit_mut::{
        VisitMut, visit_expr_call_mut, visit_expr_mut, visit_expr_struct_mut, visit_type_mut,
    },
};

use crate::{
    exts::{
        generics::{GenericParamExt, PathArgumentsExt},
        item::ImplItemExt,
        punctuated::PunctuatedExt,
    },
    utils::{
        designated::{find_designated_arg, find_designated_param},
        squote::{parse_squote, squote},
        stateset::Stateset,
    },
};

pub fn expand_item_struct(mut item_struct: ItemStruct) -> Result<TokenStream2> {
    // Get the designated parameter.
    let (d_param_index, d_attr_index) = find_designated_param(&item_struct.generics.params)?;
    let d_param = item_struct.generics.params[d_param_index].require_type_param_mut()?;

    // Remove the designating attribute from the designated parameter.
    d_param.attrs.remove(d_attr_index);

    // Add a phantom field for the generic parameter.
    let phantom_ty: Type = parse_squote!(::std::marker::PhantomData<#{d_param.ident}>);

    match &mut item_struct.fields {
        Fields::Named(FieldsNamed { named, .. }) => {
            named.push(parse_squote!(__states: #phantom_ty));
        }
        Fields::Unnamed(FieldsUnnamed { unnamed, .. }) => {
            unnamed.push(parse_squote!(#phantom_ty));
        }
        Fields::Unit => {
            // If the struct is a unit struct, it will be changed to a tuple struct.
            item_struct.fields = Fields::Unnamed(parse_squote!((#phantom_ty)));
        }
    }

    let (impl_generics, ty_generics, where_clause) = item_struct.generics.split_for_impl();

    // The generic parameter added to track states must be changed when
    // reconstructed.
    let mut generics_clone = item_struct.generics.clone();
    generics_clone.params[d_param_index] = parse_squote!(__Re);
    let re_generics = generics_clone.split_for_impl().1;

    // Collect the members and split off the phantom member, which is last.
    let members = item_struct.fields.members().collect_vec();
    let (member_phantom, members_rest) = members.split_last().unwrap();

    Ok(squote! {
        #item_struct

        impl #impl_generics #{item_struct.ident} #ty_generics #where_clause {
            pub(crate) fn __reconstruct<__Re>(self) -> #{item_struct.ident} #re_generics {
                #{item_struct.ident} {
                    #(#members_rest: self.#members_rest,)*
                    #member_phantom: ::std::marker::PhantomData,
                }
            }
        }
    })
}

pub fn expand_item_impl(
    metas: Punctuated<Meta, Token![,]>,
    mut item_impl: ItemImpl,
) -> Result<TokenStream2> {
    // Validate the implementation isn't for a trait.
    if let Some((_, trait_, _)) = item_impl.trait_.as_ref() {
        return Err(Error::new(trait_.span(), "trait impls are not supported"));
    }

    let mut stateset = Stateset::default().support("states").support("preset");

    stateset.extend_with_metas(&metas)?;

    // Validate at least one state was declared.
    if stateset["states"].is_empty() {
        return Err(Error::new(metas.span(), "no states were declared"));
    }

    // Validate there are no duplicate declared states.
    if let Some(state) = stateset["states"].iter().duplicates().next() {
        return Err(Error::new(state.span(), "state is already declared"));
    }

    // Validate there are no duplicate preset states.
    if let Some(state) = stateset["preset"].iter().duplicates().next() {
        return Err(Error::new(state.span(), "state is already preset"));
    }

    // Validate the preset states are a subset of the declared states.
    if let Some(state) = stateset["preset"]
        .iter()
        .find(|state| !stateset["states"].contains(state))
    {
        return Err(Error::new(
            state.span(),
            "preset state is not a declared state",
        ));
    }

    let mut impl_items_with_ruleset = Vec::new();
    let mut impl_items_without_ruleset = Vec::new();

    // Take the impl items temporarily and loop through them.
    for mut impl_item in mem::take(&mut item_impl.items) {
        // Validate the impl item is an associated function, otherwise it won't have a
        // ruleset.
        let ImplItem::Fn(ImplItemFn { attrs, .. }) = &mut impl_item else {
            impl_items_without_ruleset.push(impl_item);
            continue;
        };

        // Extract the ruleset attributes.
        let (r_attrs, other_attrs) = attrs
            .drain(..)
            .partition(|attr| attr.path().is_ident("stated"));
        *attrs = other_attrs;

        // Validate the associated function has at least one ruleset attribute.
        if r_attrs.is_empty() {
            impl_items_without_ruleset.push(impl_item);
            continue;
        }

        for r_attr in r_attrs {
            let mut ruleset = Stateset::default()
                .support("assert")
                .support("reject")
                .support("assign")
                .support("delete");

            match r_attr.meta {
                Meta::Path(_) => {}
                Meta::List(meta_list) => {
                    let metas = Parser::parse2(
                        Punctuated::<Meta, Token![,]>::parse_terminated,
                        meta_list.tokens,
                    )?;

                    ruleset.extend_with_metas(&metas)?;
                }
                other => {
                    return Err(Error::new(
                        other.span(),
                        "expected a list of states or nothing",
                    ));
                }
            }

            // Validate the asserted states contain no duplicates.
            if let Some(state) = ruleset["assert"].iter().duplicates().next() {
                return Err(Error::new(state.span(), "state is already asserted"));
            }

            // Validate the rejected states contain no duplicates.
            if let Some(state) = ruleset["reject"].iter().duplicates().next() {
                return Err(Error::new(state.span(), "state is already rejected"));
            }

            // Validate the assigned states contain no duplicates.
            if let Some(state) = ruleset["assign"].iter().duplicates().next() {
                return Err(Error::new(state.span(), "state is already assigned"));
            }

            // Validate the deleted states contain no duplicates.
            if let Some(state) = ruleset["delete"].iter().duplicates().next() {
                return Err(Error::new(state.span(), "state is already deleted"));
            }

            // Validate the asserted states are declared.
            if let Some(state) = ruleset["assert"]
                .iter()
                .find(|state| !stateset["states"].contains(state))
            {
                return Err(Error::new(state.span(), "asserted state is not declared"));
            }

            // Validate the rejected states are declared.
            if let Some(state) = ruleset["reject"]
                .iter()
                .find(|state| !stateset["states"].contains(state))
            {
                return Err(Error::new(state.span(), "rejected state is not declared"));
            }

            // Validate the asserted states are declared.
            if let Some(state) = ruleset["assign"]
                .iter()
                .find(|state| !stateset["states"].contains(state))
            {
                return Err(Error::new(state.span(), "assigned state is not declared"));
            }

            // Validate the asserted states are declared.
            if let Some(state) = ruleset["delete"]
                .iter()
                .find(|state| !stateset["states"].contains(state))
            {
                return Err(Error::new(state.span(), "deleted state is not declared"));
            }

            // Validate the asserted and rejected states are disjoint.
            if let Some(state) = ruleset["reject"]
                .iter()
                .find(|state| ruleset["assert"].contains(state))
            {
                return Err(Error::new(
                    state.span(),
                    "rejected state cannot also be asserted",
                ));
            }

            // Validate the assigned and deleted states are disjoint.
            if let Some(state) = ruleset["delete"]
                .iter()
                .find(|state| ruleset["assign"].contains(state))
            {
                return Err(Error::new(
                    state.span(),
                    "deleted state cannot also be assigned",
                ));
            }

            // Validatethe asserted and assigned states are disjoint.
            if let Some(state) = ruleset["assign"]
                .iter()
                .find(|state| ruleset["assert"].contains(state))
            {
                // NOTE: This probably shouldn't emit an error, a warning makes more sense.
                return Err(Error::new(
                    state.span(),
                    "assigned state doesn't need to be asserted",
                ));
            }

            // Validate the rejected and deleted states are disjoint.
            if let Some(state) = ruleset["delete"]
                .iter()
                .find(|state| ruleset["reject"].contains(state))
            {
                // NOTE: This probably shouldn't emit an error, a warning makes more sense.
                return Err(Error::new(
                    state.span(),
                    "deleted state doesn't need to be rejected",
                ));
            }

            impl_items_with_ruleset.push((impl_item.clone(), ruleset));
        }
    }

    // Get the designated parameter.
    let (d_param_index, d_attr_index) = find_designated_param(&item_impl.generics.params)?;
    let d_param = item_impl.generics.params[d_param_index].require_type_param_mut()?;

    // Remove the designating attribute from the designated parameter.
    d_param.attrs.remove(d_attr_index);

    let mut expansions = Vec::new();

    for (mut impl_item, ruleset) in impl_items_with_ruleset {
        let associated_fn = impl_item.require_fn_mut()?;

        let d_param = item_impl.generics.params[d_param_index].require_type_param()?;

        let mut item_impl = item_impl.clone();

        let Type::Path(ty_path) = item_impl.self_ty.as_mut() else {
            return Err(Error::new(
                item_impl.self_ty.span(),
                "unsupported impl type",
            ));
        };

        let Some(seg) = ty_path.path.segments.last_mut() else {
            return Err(Error::new(ty_path.span(), "unsupported impl type"));
        };

        let args = &mut seg.arguments.require_angle_bracketed_mut()?.args;
        let d_arg_index = find_designated_arg(args, &d_param.ident)?;

        let has_receiver = associated_fn.sig.receiver().is_some();

        struct ReplaceInferInReturnType(Type);

        impl VisitMut for ReplaceInferInReturnType {
            fn visit_type_mut(&mut self, ty: &mut Type) {
                let Type::Infer(_) = ty else {
                    visit_type_mut(self, ty);
                    return;
                };

                *ty = self.0.clone();
            }
        }

        if has_receiver {
            let replace_with = stateset["states"]
                .iter()
                .filter(|state| !ruleset["assert"].contains(state))
                .filter(|state| !ruleset["reject"].contains(state))
                .map(|state| parse_squote!(#state));

            item_impl.generics.params.call(|params| {
                params.splice(d_param_index..(d_param_index + 1), replace_with);
            });

            let states_in_ty = stateset["states"].iter().map(|state| -> Type {
                if ruleset["assert"].contains(state) {
                    parse_squote!(::stated::Y)
                } else if ruleset["reject"].contains(state) {
                    parse_squote!(::stated::N)
                } else {
                    parse_squote!(#state)
                }
            });

            // Replace the designated argument with the states-in type.
            args[d_arg_index] = parse_squote!((#(#states_in_ty),*));

            let states_out_ty = stateset["states"].iter().map(|state| -> Type {
                if ruleset["assign"].contains(state) {
                    parse_squote!(::stated::Y)
                } else if ruleset["delete"].contains(state) {
                    parse_squote!(::stated::N)
                } else if ruleset["assert"].contains(state) {
                    parse_squote!(::stated::Y)
                } else if ruleset["reject"].contains(state) {
                    parse_squote!(::stated::N)
                } else {
                    parse_squote!(#state)
                }
            });

            // Replace `_` in the return type with the states-out type.
            ReplaceInferInReturnType(parse_squote!((#(#states_out_ty),*)))
                .visit_return_type_mut(&mut associated_fn.sig.output);

            struct ReplaceInferInBlock;

            impl VisitMut for ReplaceInferInBlock {
                fn visit_expr_mut(&mut self, expr: &mut Expr) {
                    let Expr::Infer(_) = expr else {
                        visit_expr_mut(self, expr);
                        return;
                    };

                    *expr = parse_squote!(@expr.span()=> self.__reconstruct());
                }
            }

            ReplaceInferInBlock.visit_block_mut(&mut associated_fn.block);
        } else {
            // Replace the designated argument with the stateless type.
            args[d_arg_index] = parse_squote!(::stated::__);

            // Remove the designated parameter.
            item_impl
                .generics
                .params
                .call(|params| params.remove(d_param_index));

            let states_out_ty = stateset["states"].iter().map(|state| -> Type {
                if ruleset["assign"].contains(state) {
                    parse_squote!(::stated::Y)
                } else if ruleset["delete"].contains(state) {
                    parse_squote!(::stated::N)
                } else if ruleset["assert"].contains(state) {
                    parse_squote!(::stated::Y)
                } else if ruleset["reject"].contains(state) {
                    parse_squote!(::stated::N)
                } else if stateset["preset"].contains(state) {
                    parse_squote!(::stated::Y)
                } else {
                    parse_squote!(::stated::N)
                }
            });

            // Replace `_` in the return type with the states-out type.
            ReplaceInferInReturnType(parse_squote!((#(#states_out_ty),*)))
                .visit_return_type_mut(&mut associated_fn.sig.output);
        }

        struct ModifyStructConstructionInBlock<'a>(&'a Path);

        impl ModifyStructConstructionInBlock<'_> {
            fn should_modify(&self, other: &Path) -> bool {
                let other_idents = other.segments.iter().map(|seg| &seg.ident);

                self.0
                    .segments
                    .iter()
                    .map(|seg| &seg.ident)
                    .eq(other_idents)
            }
        }

        impl VisitMut for ModifyStructConstructionInBlock<'_> {
            fn visit_expr_mut(&mut self, expr: &mut Expr) {
                // Constructing a unit struct is considered a path expression. Since the
                // expression variant must be changed, capture it here.
                let Expr::Path(expr_path) = expr else {
                    visit_expr_mut(self, expr);
                    return;
                };

                // Check that the path of the struct being constructed is the impl type path.
                if !self.should_modify(&expr_path.path) {
                    visit_expr_mut(self, expr);
                    return;
                }

                *expr = parse_squote!(#expr_path(::std::marker::PhantomData));
            }

            // Constructing a tuple struct is considered a call expression.
            fn visit_expr_call_mut(&mut self, expr_call: &mut ExprCall) {
                let ExprCall { func, args, .. } = expr_call;

                let Expr::Path(ExprPath { path, .. }) = func.as_ref() else {
                    visit_expr_call_mut(self, expr_call);
                    return;
                };

                // Check that the path of the struct being constructed is the impl type path.
                if !self.should_modify(path) {
                    visit_expr_call_mut(self, expr_call);
                    return;
                }

                // Add an argument to the tuple struct construction.
                args.push(parse_squote!(::std::marker::PhantomData));
            }

            fn visit_expr_struct_mut(&mut self, expr_struct: &mut ExprStruct) {
                let ExprStruct { path, fields, .. } = expr_struct;

                // Check that the path of the struct being constructed is the impl type path.
                if !self.should_modify(path) {
                    visit_expr_struct_mut(self, expr_struct);
                    return;
                }

                fields.push(parse_squote!(__states: ::std::marker::PhantomData));
            }
        }

        ModifyStructConstructionInBlock(&ty_path.path).visit_block_mut(&mut associated_fn.block);

        item_impl.items.push(impl_item);
        expansions.push(squote!(#item_impl));
    }

    if !impl_items_without_ruleset.is_empty() {
        // Add back the impl items that don't have a ruleset.
        item_impl.items.extend(impl_items_without_ruleset);
        expansions.push(squote!(#item_impl));
    }

    Ok(squote! {
        #(#expansions)*
    })
}
