use itertools::Itertools;
use proc_macro2::TokenStream as TokenStream2;
use syn::{
    Fields, FieldsNamed, FieldsUnnamed, ItemImpl, ItemStruct, Meta, Result, Token, Type,
    punctuated::Punctuated,
};

use crate::{
    extensions::generics::GenericParamExt,
    utilities::{
        designated::get_designated_indices,
        squote::{parse_squote, squote},
    },
};

pub fn expand_item_struct_internal(mut item_struct: ItemStruct) -> Result<TokenStream2> {
    let (d_param_index, d_attr_index) = get_designated_indices(&item_struct.generics.params)?;
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

pub fn expand_item_impl_internal(
    metas: Punctuated<Meta, Token![,]>,
    item_impl: ItemImpl,
) -> Result<TokenStream2> {
    Ok(TokenStream2::new())
}
