use syn::{Error, GenericParam, Result, Token, punctuated::Punctuated, spanned::Spanned};

use crate::extensions::generics::GenericParamExt;

pub fn get_designated_indices(
    params: &Punctuated<GenericParam, Token![,]>,
) -> Result<(usize, usize)> {
    let mut d_indices = None;

    for (param_index, param) in params.iter().enumerate() {
        // Only type parameters can be designated, so skip the other variants.
        let Ok(ty_param) = param.require_type_param() else {
            continue;
        };

        let mut d_attr_index = None;

        for (attr_index, attr) in ty_param.attrs.iter().enumerate() {
            if !attr.path().is_ident("stated") {
                continue;
            }

            // Validate the attribute is just a path.
            attr.meta.require_path_only()?;

            if d_attr_index.replace(attr_index).is_some() {
                // TODO: Emit a warning once that feature is stabilized.
                return Err(Error::new(attr.span(), "parameter is already designated"));
            }
        }

        // If there is no designated attribute, continue to the next parameter.
        let Some(d_attr_index) = d_attr_index else {
            continue;
        };

        // Validate that the parameter wasn't designated more than once.
        if d_indices.replace((param_index, d_attr_index)).is_some() {
            return Err(Error::new(
                param.span(),
                "cannot designate more than one parameter",
            ));
        }
    }

    d_indices.ok_or_else(|| Error::new(params.span(), "no parameter is designated"))
}
