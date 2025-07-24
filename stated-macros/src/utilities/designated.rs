use syn::{
    Error, GenericArgument, GenericParam, Ident, Result, Token, Type, TypePath,
    punctuated::Punctuated, spanned::Spanned,
};

use crate::extensions::generics::GenericParamExt;

pub fn get_designated_indices(
    params: &Punctuated<GenericParam, Token![,]>,
) -> Result<(usize, usize)> {
    let mut designated_indices = None;

    for (param_index, param) in params.iter().enumerate() {
        // Only type parameters can be designated, so skip the other variants.
        let Ok(ty_param) = param.require_type_param() else {
            continue;
        };

        let mut designating_attr_index = None;

        for (attr_index, attr) in ty_param.attrs.iter().enumerate() {
            if !attr.path().is_ident("stated") {
                continue;
            }

            // Validate the attribute is just a path.
            attr.meta.require_path_only()?;

            // Set the index.
            if designating_attr_index.replace(attr_index).is_some() {
                // TODO(blocked): Emit a warning once procedural macro diagnostics is
                // stabilized. Tracking issue: https://github.com/rust-lang/rust/issues/54140.
                return Err(Error::new(attr.span(), "parameter is already designated"));
            }
        }

        // If there is no designating attribute, continue to the next parameter.
        let Some(designating_attr_index) = designating_attr_index else {
            continue;
        };

        // Set the designated indices. Validate only one parameter was designated.
        if designated_indices
            .replace((param_index, designating_attr_index))
            .is_some()
        {
            return Err(Error::new(
                param.span(),
                "cannot designate more than one parameter",
            ));
        }
    }

    designated_indices.ok_or_else(|| Error::new(params.span(), "no parameter is designated"))
}

pub fn find_designated_arg(
    args: &Punctuated<GenericArgument, Token![,]>,
    designated_param_ident: &Ident,
) -> Result<usize> {
    let mut designated_arg_index = None;

    for (arg_index, arg) in args.iter().enumerate() {
        // Only type arguments can be designated. Skip the other variants.
        let GenericArgument::Type(Type::Path(TypePath { path, .. })) = arg else {
            continue;
        };

        // Skip to the next argument if it doesn't match the ident of the designated
        // parameter.
        if !path.is_ident(designated_param_ident) {
            continue;
        }

        // Set the designated argument index. Validate only one argument was designated.
        if designated_arg_index.replace(arg_index).is_some() {
            return Err(Error::new(
                arg.span(),
                "only one argument can match the designated parameter",
            ));
        }
    }

    designated_arg_index
        .ok_or_else(|| Error::new(args.span(), "no argument matches the designated parameter"))
}
