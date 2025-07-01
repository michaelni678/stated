use proc_macro2::TokenStream as TokenStream2;
use syn::{
    Error, Ident, ItemImpl, ItemStruct, Meta, Result, Token, punctuated::Punctuated,
    spanned::Spanned,
};

use crate::{
    exts::punctuated::PunctuatedExt,
    utils::squote::{parse_squote, squote},
};

pub fn expand_item_struct(
    mut metas: Punctuated<Meta, Token![,]>,
    item_struct: ItemStruct,
) -> Result<TokenStream2> {
    let mut exports = metas
        .call(|metas| {
            metas
                .extract_if(.., |meta| meta.path().is_ident("export"))
                .map(|meta| match meta {
                    Meta::NameValue(name_value) => Ok(parse_squote!(#{name_value.value})),
                    other => Err(Error::new(other.span(), "export name format is incorrect")),
                })
                .collect::<Result<Vec<Ident>>>()
        })?
        .into_iter();

    let export = exports
        .next()
        .unwrap_or(parse_squote!(#{item_struct.ident}));

    if let Some(export) = exports.next() {
        return Err(Error::new(
            export.span(),
            "export name can only be specified once",
        ));
    }

    Ok(squote! {
        #[::stated::stated_internal]
        #item_struct

        #[macro_export]
        #[doc(hidden)]
        macro_rules! #{export}! {
            ($($tt:tt)*) => {
                #[::stated::stated_internal(#metas)]
                $($tt)*
            }
        }
    })
}

pub fn expand_item_impl(
    metas: Punctuated<Meta, Token![,]>,
    item_impl: ItemImpl,
) -> Result<TokenStream2> {
    Ok(squote! {})
}
