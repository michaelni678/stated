use proc_macro2::TokenStream as TokenStream2;
use syn::{ItemImpl, ItemStruct, Meta, Result, Token, punctuated::Punctuated};

use crate::utils::squote::squote;

pub fn expand_item_struct(
    metas: Punctuated<Meta, Token![,]>,
    item_struct: ItemStruct,
) -> Result<TokenStream2> {
    Ok(squote! {})
}

pub fn expand_item_impl(
    metas: Punctuated<Meta, Token![,]>,
    item_impl: ItemImpl,
) -> Result<TokenStream2> {
    Ok(squote! {})
}
