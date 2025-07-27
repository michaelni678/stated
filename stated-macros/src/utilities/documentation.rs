use syn::{Meta, Result};

#[derive(Default)]
pub struct Documentation {
    description: bool,
    ugly: bool,
}

impl Documentation {
    /// Configures the documentation with the given metas.
    ///
    /// Skips metas without the `docs` attribute.
    pub fn configure_with_metas<'a, M>(&mut self, metas: M) -> Result<()>
    where
        M: IntoIterator<Item = &'a Meta>,
    {
        let metas = metas
            .into_iter()
            .filter(|meta| meta.path().is_ident("docs"));

        for meta in metas {
            meta.require_list()?.parse_nested_meta(|meta| {
                if meta.path.is_ident("description") {
                    if self.description {
                        return Err(meta.error("redundant `description` attribute"));
                    }

                    self.description = true;
                } else if meta.path.is_ident("ugly") {
                    if self.ugly {
                        return Err(meta.error("redundant `ugly` attribute"));
                    }

                    self.ugly = true;
                } else {
                    return Err(meta.error("invalid attribute"));
                }

                Ok(())
            })?;
        }

        Ok(())
    }
}
