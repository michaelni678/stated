use extend::ext;
use syn::{Error, ImplItem, ImplItemFn, Result, spanned::Spanned};

#[ext]
pub impl ImplItem {
    /// Get the impl item as an associated function.
    fn require_fn(&self) -> Result<&ImplItemFn> {
        match self {
            Self::Fn(associated_fn) => Ok(associated_fn),
            _ => Err(Error::new(self.span(), "expected an associated function")),
        }
    }

    /// Get the impl item as an associated function mutably.
    fn require_fn_mut(&mut self) -> Result<&mut ImplItemFn> {
        match self {
            Self::Fn(associated_fn) => Ok(associated_fn),
            _ => Err(Error::new(self.span(), "expected an associated function")),
        }
    }
}
