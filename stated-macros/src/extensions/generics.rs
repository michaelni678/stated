use extend::ext;
use syn::{
    AngleBracketedGenericArguments, Error, GenericParam, PathArguments, Result, TypeParam,
    spanned::Spanned,
};

#[ext]
pub impl GenericParam {
    /// Get the parameter as a type param.
    fn require_type_param(&self) -> Result<&TypeParam> {
        match self {
            Self::Type(ty_param) => Ok(ty_param),
            _ => Err(Error::new(self.span(), "expected a type parameter")),
        }
    }

    /// Get the parameter as a type param mutably.
    fn require_type_param_mut(&mut self) -> Result<&mut TypeParam> {
        match self {
            Self::Type(ty_param) => Ok(ty_param),
            _ => Err(Error::new(self.span(), "expected a type parameter")),
        }
    }
}

#[ext]
pub impl PathArguments {
    /// Get the arguments as angle bracketed arguments.
    fn require_angle_bracketed(&self) -> Result<&AngleBracketedGenericArguments> {
        match self {
            Self::AngleBracketed(angle_bracketed) => Ok(angle_bracketed),
            _ => Err(Error::new(
                self.span(),
                "expected angle bracketed arguments",
            )),
        }
    }

    /// Get the arguments as angle bracketed arguments mutably.
    fn require_angle_bracketed_mut(&mut self) -> Result<&mut AngleBracketedGenericArguments> {
        match self {
            Self::AngleBracketed(angle_bracketed) => Ok(angle_bracketed),
            _ => Err(Error::new(
                self.span(),
                "expected angle bracketed arguments",
            )),
        }
    }
}
