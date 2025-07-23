use std::mem;

use itertools::Itertools;
use proc_macro2::TokenStream as TokenStream2;
use syn::{
    Error, Fields, FieldsNamed, FieldsUnnamed, ItemImpl, ItemStruct, Meta, Result, Token, Type,
    punctuated::Punctuated, spanned::Spanned,
};

use crate::{
    extensions::{generics::GenericParamExt, item::ImplItemExt},
    utilities::{
        designated::get_designated_indices,
        squote::{parse_squote, squote},
        stateset::Stateset,
    },
};

pub fn expand_item_struct_internal(mut item_struct: ItemStruct) -> Result<TokenStream2> {
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

    // Take the impl items temporarily and loop through them.
    for mut impl_item in mem::take(&mut item_impl.items) {
        let associated_fn = impl_item.require_fn_mut()?;

        let ruleset_attrs = associated_fn
            .attrs
            .extract_if(.., |attr| attr.path().is_ident("stated"))
            .collect_vec();

        if ruleset_attrs.is_empty() {
            return Err(Error::new(associated_fn.span(), "no ruleset is specified"));
        }
    }

    Ok(TokenStream2::new())
}
