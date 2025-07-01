use proc_macro::TokenStream;
use syn::{
    Error, Item, Meta, Token, parse::Nothing, parse_macro_input, punctuated::Punctuated,
    spanned::Spanned,
};

use crate::expand::{stated, stated_internal};

mod expand;
mod exts;
mod utils;

#[proc_macro_attribute]
pub fn stated(args: TokenStream, input: TokenStream) -> TokenStream {
    let metas = parse_macro_input!(args with Punctuated<Meta, Token![,]>::parse_terminated);
    let item = parse_macro_input!(input as Item);

    let result = match item {
        Item::Struct(item_struct) => stated::expand_item_struct(metas, item_struct),
        Item::Impl(item_impl) => stated::expand_item_impl(metas, item_impl),
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
            stated_internal::expand_item_struct(item_struct)
        }
        Item::Impl(item_impl) => {
            let metas = parse_macro_input!(args with Punctuated<Meta, Token![,]>::parse_terminated);
            stated_internal::expand_item_impl(metas, item_impl)
        }
        other => Err(Error::new(other.span(), "expected a struct or impl")),
    };

    result.unwrap_or_else(Error::into_compile_error).into()
}
