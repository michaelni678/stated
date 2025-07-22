use extend::ext;
use syn::{Error, PathArguments, Result, Type, TypePath, spanned::Spanned};

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

#[ext]
pub impl TypePath {
    /// Strip all generic arguments.
    fn strip_generics(&mut self) {
        for seg in self.path.segments.iter_mut() {
            seg.arguments = PathArguments::None;
        }
    }
}
