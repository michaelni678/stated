//! Procedural macros for [Stated](https://crates.io/crates/stated).

use proc_macro::TokenStream;
use syn::{
    Error, Item, Meta, Token, parse::Nothing, parse_macro_input, punctuated::Punctuated,
    spanned::Spanned,
};

use crate::expand::{
    stated::{expand_item_impl, expand_item_struct},
    stated_internal::{expand_item_impl_internal, expand_item_struct_internal},
};

mod expand;
mod extensions;
mod utilities;

#[proc_macro_attribute]
pub fn stated(args: TokenStream, input: TokenStream) -> TokenStream {
    let item = parse_macro_input!(input as Item);

    let result = match item {
        Item::Struct(item_struct) => {
            let metas = parse_macro_input!(args with Punctuated<Meta, Token![,]>::parse_terminated);
            expand_item_struct(metas, item_struct)
        }
        Item::Impl(item_impl) => {
            parse_macro_input!(args as Nothing);
            expand_item_impl(item_impl)
        }
        other => Err(Error::new(other.span(), "expected a struct or impl")),
    };

    result.unwrap_or_else(Error::into_compile_error).into()
}

#[proc_macro_attribute]
pub fn stated_internal(args: TokenStream, input: TokenStream) -> TokenStream {
    let item = parse_macro_input!(input as Item);

    let result = match item {
        Item::Struct(item_struct) => {
            parse_macro_input!(args as Nothing);
            expand_item_struct_internal(item_struct)
        }
        Item::Impl(item_impl) => {
            let metas = parse_macro_input!(args with Punctuated<Meta, Token![,]>::parse_terminated);
            expand_item_impl_internal(metas, item_impl)
        }
        other => Err(Error::new(other.span(), "expected a struct or impl")),
    };

    result.unwrap_or_else(Error::into_compile_error).into()
}
