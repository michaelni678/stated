use std::{collections::HashMap, ops::Deref};

use syn::{Ident, Meta, Result};

/// A map of state attributes to state identifiers.
#[derive(Default, Clone)]
pub struct Stateset(HashMap<String, Vec<Ident>>);

impl Deref for Stateset {
    type Target = HashMap<String, Vec<Ident>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Stateset {
    /// Add support for an attribute.
    pub fn support(mut self, attribute: &str) -> Self {
        self.0.insert(attribute.to_string(), Vec::new());
        self
    }

    /// Extend the map with the given metas. 
    /// 
    /// Skips unsupported attributes.
    pub fn extend_with_metas<'a, M>(&mut self, metas: M) -> Result<()>
    where
        M: IntoIterator<Item = &'a Meta>,
    {
        metas.into_iter().try_for_each(|meta| self.extend_with_meta(meta))
    }

    /// Extend the map with the given meta.
    ///
    /// Skips if the attribute is unsupported.
    pub fn extend_with_meta(&mut self, meta: &Meta) -> Result<()> {
        let attribute = meta.path().require_ident()?.to_string();

        if let Some(states) = self.0.get_mut(&attribute) {
            meta.require_list()?.parse_nested_meta(|meta| {
                let state = meta.path.require_ident().cloned()?;
                states.push(state);
                Ok(())
            })?;
        }

        Ok(())
    }
}
