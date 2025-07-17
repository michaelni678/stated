use proc_macro2::TokenStream as TokenStream2;
use syn::{
    Error, Ident, ItemImpl, ItemStruct, Meta, Result, Token, Type, TypePath,
    punctuated::Punctuated, spanned::Spanned,
};

use crate::{
    exts::punctuated::PunctuatedExt,
    utils::squote::{parse_squote, squote},
};

pub fn expand_item_struct(
    mut metas: Punctuated<Meta, Token![,]>,
    item_struct: ItemStruct,
) -> Result<TokenStream2> {
    // Extract all export names.
    let mut export_names = metas
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

    // Get the first export name, or default to the struct name.
    let export_name = export_names
        .next()
        .unwrap_or_else(|| item_struct.ident.clone());

    // Validate that only one export name was specified.
    if let Some(export) = export_names.next() {
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
        macro_rules! #export_name {
            ($($tt:tt)*) => {
                // Emit the input, but with the metas attached.
                #[::stated::stated_internal(#metas)]
                $($tt)*
            }
        }
    })
}

pub fn expand_item_impl(
    mut metas: Punctuated<Meta, Token![,]>,
    item_impl: ItemImpl,
) -> Result<TokenStream2> {
    // Extract all the import names.
    let mut import_names = metas
        .call(|metas| {
            metas
                .extract_if(.., |meta| meta.path().is_ident("import"))
                .map(|meta| match meta {
                    Meta::NameValue(name_value) => Ok(parse_squote!(#{name_value.value})),
                    other => Err(Error::new(other.span(), "import name format is incorrect")),
                })
                .collect::<Result<Vec<Ident>>>()
        })?
        .into_iter();

    // Get the first import name, or default to the impl type name.
    let import_name = match import_names.next() {
        Some(import_name) => import_name,
        None => {
            let Type::Path(TypePath { path, .. }) = item_impl.self_ty.as_ref() else {
                return Err(Error::new(item_impl.self_ty.span(), "expected a path"));
            };

            path.segments
                .last()
                .ok_or_else(|| Error::new(path.span(), "path is invalid"))?
                .ident
                .clone()
        }
    };

    // Validate that only one import name was specified.
    if let Some(import_name) = import_names.next() {
        return Err(Error::new(
            import_name.span(),
            "import name can only be specified once",
        ));
    }

    Ok(squote! {
        #import_name!(#item_impl);
    })
}
