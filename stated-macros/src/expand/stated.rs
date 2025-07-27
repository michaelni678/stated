use proc_macro2::TokenStream as TokenStream2;
use quote::format_ident;
use syn::{ItemImpl, ItemStruct, Meta, Result, Token, punctuated::Punctuated};

use crate::{
    extensions::ty::{TypeExt, TypePathExt},
    utilities::squote::squote,
};

pub fn expand_item_struct(
    metas: Punctuated<Meta, Token![,]>,
    item_struct: ItemStruct,
) -> Result<TokenStream2> {
    let macro_name = format_ident!("__{}", item_struct.ident);

    Ok(squote! {
        // Re-emit the struct with the internal macro.
        #[::stated::stated_internal(#metas)]
        #item_struct

        #[doc(hidden)]
        macro_rules! #macro_name {
            ($($tt:tt)*) => {
                // Re-emit the input, but with the metas attached.
                #[::stated::stated_internal(#metas)]
                $($tt)*
            }
        }

        // Make the macro public with the same name as the struct.
        pub(crate) use #macro_name as #{item_struct.ident};
    })
}

pub fn expand_item_impl(item_impl: ItemImpl) -> Result<TokenStream2> {
    // Expect a macro at the impl type path with the same name.
    let mut macro_path = item_impl.self_ty.require_path()?.clone();

    // Strip the generic arguments from the macro path.
    macro_path.strip_generics();

    Ok(squote! {
        #macro_path!(#item_impl);
    })
}
