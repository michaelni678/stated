use proc_macro2::TokenStream as TokenStream2;
use syn::{ItemImpl, ItemStruct, Meta, Result, Token, punctuated::Punctuated};

pub fn expand_item_struct_internal(item_struct: ItemStruct) -> Result<TokenStream2> {
    Ok(TokenStream2::new())
}

pub fn expand_item_impl_internal(
    metas: Punctuated<Meta, Token![,]>,
    item_impl: ItemImpl,
) -> Result<TokenStream2> {
    Ok(TokenStream2::new())
}
