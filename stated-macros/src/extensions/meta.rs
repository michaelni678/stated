use extend::ext;
use syn::{Error, Meta, MetaNameValue, Result, spanned::Spanned};

#[ext]
pub impl Meta {
    /// Returns an error if the meta is a name-value.
    fn forbid_name_value(&self) -> Result<&Self> {
        match self {
            Self::NameValue(MetaNameValue { eq_token, .. }) => {
                return Err(Error::new(eq_token.span(), "did not expect `=`"));
            }
            _ => Ok(self),
        }
    }
}
