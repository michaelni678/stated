use proc_macro2::TokenStream as TokenStream2;
use syn::{
    Error, ItemImpl, ItemStruct, Meta, Result, Token, Type, TypePath, punctuated::Punctuated,
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
    let mut export_name = None;

    // Try to find and remove the first export name meta.
    if let Some(meta) = metas.find_remove(|meta| meta.path().is_ident("export")) {
        let name_value = meta.require_name_value()?;
        export_name = Some(parse_squote!(#{name_value.value}));
    }

    // Validate there are no more export name metas.
    if let Some(meta) = metas.find_remove(|meta| meta.path().is_ident("export")) {
        // NOTE: This probably shouldn't emit an error, a warning makes more sense.
        return Err(Error::new(
            meta.span(),
            "export name can only be specified once",
        ));
    }

    let export_name = export_name.unwrap_or_else(|| item_struct.ident.clone());

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
    let mut import_name = None;

    // Try to find and remove the first import name meta.
    if let Some(meta) = metas.find_remove(|meta| meta.path().is_ident("import")) {
        let name_value = meta.require_name_value()?;
        import_name = Some(parse_squote!(#{name_value.value}));
    }

    // Validate there are no more import name metas.
    if let Some(meta) = metas.find_remove(|meta| meta.path().is_ident("import")) {
        // NOTE: This probably shouldn't emit an error, a warning makes more sense.
        return Err(Error::new(
            meta.span(),
            "import name can only be specified once",
        ));
    }

    // Unwrap the import name or default to the impl type name.
    let import_name = match import_name {
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

    Ok(squote! {
        #import_name!(#item_impl);
    })
}
