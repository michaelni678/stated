use proc_macro2::TokenStream as TokenStream2;
use quote::format_ident;
use syn::{ItemImpl, ItemStruct, Macro, Meta, Result, Token, punctuated::Punctuated};

use crate::{
    extensions::ty::TypeExt,
    utilities::squote::{parse_squote, squote},
};

pub fn expand_item_struct(
    metas: Punctuated<Meta, Token![,]>,
    item_struct: ItemStruct,
) -> Result<TokenStream2> {
    let macro_name = format_ident!("__{}", item_struct.ident);

    Ok(squote! {
        // Re-emit the struct with the internal macro.
        #[::stated::stated_internal]
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
    let mut mac: Macro = parse_squote!(__!(#item_impl));
    mac.path = item_impl.self_ty.require_path()?.path.clone();

    Ok(squote! {
        #mac;
    })
}
