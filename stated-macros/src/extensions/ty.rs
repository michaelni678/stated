use extend::ext;
use syn::{Error, Result, Type, TypePath, spanned::Spanned};

#[ext]
pub impl Type {
    /// Get the type as a path.
    fn require_path(&self) -> Result<&TypePath> {
        match self {
            Self::Path(path) => Ok(path),
            _ => Err(Error::new(self.span(), "expected a path")),
        }
    }
}
