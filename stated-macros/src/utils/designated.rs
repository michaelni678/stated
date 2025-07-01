use syn::{
    Error, GenericArgument, GenericParam, Ident, Result, Token, Type, TypePath,
    punctuated::Punctuated, spanned::Spanned,
};

/// Finds the designated parameter in the given parameters.
pub fn find_designated_param(
    params: &Punctuated<GenericParam, Token![,]>,
) -> Result<(usize, usize)> {
    let mut d_indices = Vec::new();

    for (param_index, param) in params.iter().enumerate() {
        // Only type parameters can be designated. Skip the other variants.
        let GenericParam::Type(ty_param) = param else {
            continue;
        };

        let mut d_attr_indices = Vec::new();

        for (attr_index, attr) in ty_param.attrs.iter().enumerate() {
            // Skip attributes that don't have the "stated" path.
            if !attr.path().is_ident("stated") {
                continue;
            }

            // Validate the attribute is just a path.
            attr.meta.require_path_only()?;

            d_attr_indices.push(attr_index);
        }

        // If there is no designated attribute, continue to the next parameter.
        let Some(d_attr_index) = d_attr_indices.first().copied() else {
            continue;
        };

        // Validate that the parameter wasn't designated more than once.
        if let Some(d_attr_index) = d_attr_indices.get(1).copied() {
            // NOTE: This probably shouldn't emit an error, a warning makes more sense.
            return Err(Error::new(
                ty_param.attrs[d_attr_index].span(),
                "parameter is already designated",
            ));
        }

        d_indices.push((param_index, d_attr_index));
    }

    // Validate there is a designated parameter.
    let Some((d_param_index, d_attr_index)) = d_indices.first_mut().copied() else {
        return Err(Error::new(params.span(), "no parameter is designated"));
    };

    // Validate that there are not multiple designated parameters.
    if let Some((d_param_index, _)) = d_indices.get(1).copied() {
        return Err(Error::new(
            params[d_param_index].span(),
            "cannot designate more than one parameter",
        ));
    }

    Ok((d_param_index, d_attr_index))
}

/// Find the designated argument in the given arguments, which must match the
/// designated parameter.
pub fn find_designated_arg(
    args: &Punctuated<GenericArgument, Token![,]>,
    d_param_ident: &Ident,
) -> Result<usize> {
    let mut d_arg_indices = Vec::new();

    for (arg_index, arg) in args.iter().enumerate() {
        // Only type arguments can be designated. Skip the other variants.
        let GenericArgument::Type(Type::Path(TypePath { path, .. })) = arg else {
            continue;
        };

        if path.is_ident(d_param_ident) {
            d_arg_indices.push(arg_index);
        }
    }

    // Validate there is a designated argument.
    let Some(d_arg_index) = d_arg_indices.first_mut().copied() else {
        return Err(Error::new(
            args.span(),
            "no argument matches the designated parameter",
        ));
    };

    // Validate that there are not multiple designated arguments.
    if let Some(d_arg_index) = d_arg_indices.get(1).copied() {
        return Err(Error::new(
            args[d_arg_index].span(),
            "only one argument can match the designated parameter",
        ));
    }

    Ok(d_arg_index)
}
