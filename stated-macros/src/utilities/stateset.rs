use std::{collections::HashMap, ops::Deref};

use syn::{Ident, Meta, Result};

/// A map of state properties to state identifiers.
#[derive(Default, Clone)]
pub struct Stateset(HashMap<String, Vec<Ident>>);

impl Deref for Stateset {
    type Target = HashMap<String, Vec<Ident>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Stateset {
    /// Add support for a property.
    pub fn support(mut self, property: &str) -> Self {
        self.0.insert(property.to_string(), Vec::new());
        self
    }

    /// Extend the map with the given metas.
    ///
    /// Panics if a property is not supported.
    pub fn extend_with_metas<'a, M>(&mut self, metas: M) -> Result<()>
    where
        M: IntoIterator<Item = &'a Meta>,
    {
        metas
            .into_iter()
            .try_for_each(|meta| self.extend_with_meta(meta))
    }

    /// Extend the map with the given meta.
    ///
    /// Panics if a property is not supported.
    pub fn extend_with_meta(&mut self, meta: &Meta) -> Result<()> {
        let property = meta.path().require_ident()?.to_string();
        let states = self.0.get_mut(&property).expect("unsupported property");

        meta.require_list()?.parse_nested_meta(|meta| {
            let state = meta.path.require_ident().cloned()?;
            states.push(state);
            Ok(())
        })
    }
}
