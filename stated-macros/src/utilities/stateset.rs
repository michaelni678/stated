use std::{collections::HashMap, ops::Deref};

use syn::{Ident, Meta, Result};

/// A map of state kinds to state identifiers.
#[derive(Default, Clone)]
pub struct Stateset(HashMap<String, Vec<Ident>>);

impl Deref for Stateset {
    type Target = HashMap<String, Vec<Ident>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Stateset {
    /// Add support for a state kind.
    pub fn support(mut self, kind: &str) -> Self {
        self.0.insert(kind.to_string(), Vec::new());
        self
    }

    /// Extend the map with the given metas. Skips metas that have an state kind
    /// that isn't supported.
    pub fn extend_with_metas<'a, M>(&mut self, metas: M) -> Result<()>
    where
        M: IntoIterator<Item = &'a Meta>,
    {
        metas
            .into_iter()
            .try_for_each(|meta| self.extend_with_meta(meta))
    }

    /// Extend the map with the given meta. Skips the meta if it has a state
    /// kind that isn't supported.
    pub fn extend_with_meta(&mut self, meta: &Meta) -> Result<()> {
        let kind = meta.path().require_ident()?.to_string();

        if let Some(states) = self.0.get_mut(&kind) {
            meta.require_list()?.parse_nested_meta(|meta| {
                let state = meta.path.require_ident().cloned()?;
                states.push(state);
                Ok(())
            })?;
        }

        Ok(())
    }
}
