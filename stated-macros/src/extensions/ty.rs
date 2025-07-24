use extend::ext;
use syn::{Error, PathArguments, PathSegment, Result, Type, TypePath, spanned::Spanned};

#[ext]
pub impl Type {
    /// Get the type as a path.
    fn require_path(&self) -> Result<&TypePath> {
        match self {
            Self::Path(path) => Ok(path),
            _ => Err(Error::new(self.span(), "expected a path")),
        }
    }

    /// Get the type as a path mutably.
    fn require_path_mut(&mut self) -> Result<&mut TypePath> {
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

    /// Get the last segment of the path.
    fn last(&self) -> Result<&PathSegment> {
        self.path
            .segments
            .last()
            .ok_or_else(|| Error::new(self.span(), "path is empty"))
    }

    /// Get the last segment of the path mutably.
    fn last_mut(&mut self) -> Result<&mut PathSegment> {
        let self_span = self.span();

        self.path
            .segments
            .last_mut()
            .ok_or_else(|| Error::new(self_span, "path is empty"))
    }
}
