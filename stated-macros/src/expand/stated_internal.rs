use std::mem;

use itertools::Itertools;
use proc_macro2::TokenStream as TokenStream2;
use syn::{
    Error, Fields, FieldsNamed, FieldsUnnamed, ImplItem, ItemImpl, ItemStruct, Meta, MetaList,
    Result, Token, Type, parse::Parser, punctuated::Punctuated, spanned::Spanned,
    visit_mut::VisitMut,
};

use crate::{
    extensions::{
        generics::{GenericParamExt, PathArgumentsExt},
        item::ImplItemExt,
        meta::MetaExt,
        punctuated::PunctuatedExt,
        ty::{TypeExt, TypePathExt},
    },
    utilities::{
        designated::{find_designated_arg, get_designated_indices},
        documentation::{Description, DescriptionLine, Documentation},
        squote::{parse_squote, squote},
        stateset::Stateset,
        visit::{AddFieldInStructConstruction, ReplaceExprInfer, ReplaceTypeInfer},
    },
};

pub fn expand_item_struct_internal(
    metas: Punctuated<Meta, Token![,]>,
    mut item_struct: ItemStruct,
) -> Result<TokenStream2> {
    // Validate all attributes in the metas are supported.
    if let Some(meta) = metas
        .iter()
        .filter(|meta| !meta.path().is_ident("states"))
        .filter(|meta| !meta.path().is_ident("preset"))
        .find(|meta| !meta.path().is_ident("docs"))
    {
        return Err(Error::new(meta.path().span(), "invalid attribute"));
    }

    let mut documentation = Documentation::default();
    documentation.configure_with_metas(&metas)?;

    let mut stateset = Stateset::default().support("states").support("preset");
    stateset.extend_with_metas(&metas)?;

    if documentation.description {
        item_struct.attrs.push(
            Description::new(&stateset)
                .line(DescriptionLine::new("states").label("States"))
                .line(DescriptionLine::new("preset").label("Preset"))
                .generate(),
        );
    }

    let (designated_param_index, designating_attr_index) =
        get_designated_indices(&item_struct.generics.params)?;
    let designated_param =
        item_struct.generics.params[designated_param_index].require_type_param_mut()?;

    // Remove the designating attribute from the designated parameter.
    designated_param.attrs.remove(designating_attr_index);

    // Add a phantom field for the generic parameter.
    let phantom_ty: Type = parse_squote!(::std::marker::PhantomData<#{designated_param.ident}>);

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
    generics_clone.params[designated_param_index] = parse_squote!(__Re);
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

pub fn expand_item_impl_internal(
    metas: Punctuated<Meta, Token![,]>,
    mut item_impl: ItemImpl,
) -> Result<TokenStream2> {
    // Validate all attributes in the metas are supported.
    if let Some(meta) = metas
        .iter()
        .filter(|meta| !meta.path().is_ident("states"))
        .filter(|meta| !meta.path().is_ident("preset"))
        .find(|meta| !meta.path().is_ident("docs"))
    {
        return Err(Error::new(meta.path().span(), "invalid attribute"));
    }

    // Validate the implementation isn't for a trait.
    if let Some((_, trait_, _)) = item_impl.trait_.as_ref() {
        return Err(Error::new(trait_.span(), "trait impls are not supported"));
    }

    let mut documentation = Documentation::default();
    documentation.configure_with_metas(&metas)?;

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

    // Get the designated indices.
    let (designated_param_index, designating_attr_index) =
        get_designated_indices(&item_impl.generics.params)?;
    let designated_param =
        item_impl.generics.params[designated_param_index].require_type_param_mut()?;
    let designated_param_ident = designated_param.ident.clone();

    // Remove the designating attribute from the designated parameter.
    designated_param.attrs.remove(designating_attr_index);

    let impl_items = mem::take(&mut item_impl.items);

    // Rename the variable for clarity. This will act as a template when needed. It
    // has no impl items and the designating attribute was removed, making it a
    // completely valid impl block.
    let item_impl_template = item_impl;

    let mut pretty_item_impl = item_impl_template.clone();

    let mut expansions = Vec::new();

    for mut impl_item in impl_items {
        let ruleset_attrs = impl_item
            .require_fn_mut()?
            .attrs
            .extract_if(.., |attr| attr.path().is_ident("stated"))
            .collect_vec();

        if ruleset_attrs.is_empty() {
            return Err(Error::new(impl_item.span(), "no ruleset is specified"));
        }

        for ruleset_attr in ruleset_attrs {
            let mut ruleset = Stateset::default()
                .support("assert")
                .support("reject")
                .support("assign")
                .support("delete");

            // Validate the ruleset attribute is not a name-value.
            ruleset_attr.meta.forbid_name_value()?;

            if let Meta::List(MetaList { tokens, .. }) = ruleset_attr.meta {
                let metas = Punctuated::<Meta, Token![,]>::parse_terminated.parse2(tokens)?;

                // Validate all attributes in the metas are supported.
                if let Some(meta) = metas
                    .iter()
                    .filter(|meta| !meta.path().is_ident("assert"))
                    .filter(|meta| !meta.path().is_ident("reject"))
                    .filter(|meta| !meta.path().is_ident("assign"))
                    .find(|meta| !meta.path().is_ident("delete"))
                {
                    return Err(Error::new(meta.path().span(), "invalid attribute"));
                }

                ruleset.extend_with_metas(&metas)?;
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

            // Validate the asserted and assigned states are disjoint.
            if let Some(state) = ruleset["assign"]
                .iter()
                .find(|state| ruleset["assert"].contains(state))
            {
                // TODO(blocked): Emit a warning once procedural macro diagnostics is
                // stabilized. Tracking issue: https://github.com/rust-lang/rust/issues/54140.
                return Err(Error::new(
                    state.span(),
                    "asserted state doesn't need to be assigned",
                ));
            }

            // Validate the rejected and deleted states are disjoint.
            if let Some(state) = ruleset["delete"]
                .iter()
                .find(|state| ruleset["reject"].contains(state))
            {
                // TODO(blocked): Emit a warning once procedural macro diagnostics is
                // stabilized. Tracking issue: https://github.com/rust-lang/rust/issues/54140.
                return Err(Error::new(
                    state.span(),
                    "rejected state doesn't need to be deleted",
                ));
            }

            // Clone the impl block. Each function will go in its own block due to differing
            // generics.
            let mut item_impl = item_impl_template.clone();
            let mut impl_item = impl_item.clone();
            let associated_fn = impl_item.require_fn_mut()?;

            if documentation.description {
                associated_fn.attrs.push(
                    Description::new(&ruleset)
                        .line(DescriptionLine::new("assert").label("Assert"))
                        .line(DescriptionLine::new("reject").label("Reject"))
                        .line(DescriptionLine::new("assign").label("Assign"))
                        .line(DescriptionLine::new("delete").label("Delete"))
                        .generate(),
                );
            }

            let item_impl_path = item_impl.self_ty.require_path_mut()?;

            let args = &mut item_impl_path
                .last_mut()?
                .arguments
                .require_angle_bracketed_mut()?
                .args;

            let designated_arg_index = find_designated_arg(args, &designated_param_ident)?;

            if !documentation.ugly {
                let mut pretty_associated_fn = associated_fn.clone();

                // Replace `_` in the return type with the designated parameter's ident.
                ReplaceTypeInfer(parse_squote!(#designated_param_ident))
                    .visit_return_type_mut(&mut pretty_associated_fn.sig.output);
                pretty_associated_fn.block = parse_squote!({ unreachable!() });

                pretty_item_impl
                    .items
                    .push(ImplItem::Fn(pretty_associated_fn));
            }

            if let Some(receiver) = associated_fn.sig.receiver() {
                let receiver_span = receiver.span();

                let replace_with = stateset["states"]
                    .iter()
                    .filter(|state| !ruleset["assert"].contains(state))
                    .filter(|state| !ruleset["reject"].contains(state))
                    .map(|state| parse_squote!(#state));

                item_impl.generics.params.call(|params| {
                    params.splice(
                        designated_param_index..(designated_param_index + 1),
                        replace_with,
                    );
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

                // Replace the designated argument with the ingoing type.
                args[designated_arg_index] = parse_squote!((#(#states_in_ty),*));

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

                // Replace the designated argument with the outgoing type.
                ReplaceTypeInfer(parse_squote!((#(#states_out_ty),*)))
                    .visit_return_type_mut(&mut associated_fn.sig.output);

                ReplaceExprInfer(parse_squote!(@receiver_span=> self.__reconstruct()))
                    .visit_block_mut(&mut associated_fn.block);
            } else {
                // Replace the designated argument with the stateless type.
                args[designated_arg_index] = parse_squote!(::stated::__);

                // Remove the designated parameter.
                item_impl
                    .generics
                    .params
                    .call(|params| params.remove(designated_param_index));

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
                ReplaceTypeInfer(parse_squote!((#(#states_out_ty),*)))
                    .visit_return_type_mut(&mut associated_fn.sig.output);
            }

            AddFieldInStructConstruction {
                path: &item_impl_path.path,
                field_member: parse_squote!(__states),
                field_expr: parse_squote!(::std::marker::PhantomData),
            }
            .visit_block_mut(&mut associated_fn.block);

            item_impl.items.push(impl_item);

            if documentation.ugly {
                expansions.push(squote!(#item_impl));
            } else {
                expansions.push(squote! {
                    #[cfg(not(doc))]
                    #item_impl
                });
            }
        }
    }

    if !documentation.ugly {
        expansions.push(squote! {
            #[cfg(doc)]
            #pretty_item_impl
        });
    }

    Ok(squote! {
        #(#expansions)*
    })
}
