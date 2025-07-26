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
    /// Returns `Ok(false)` if any of the properties are not supported.
    pub fn extend_with_metas<'a, M>(&mut self, metas: M) -> Result<bool>
    where
        M: IntoIterator<Item = &'a Meta>,
    {
        let mut supported = true;

        for meta in metas {
            supported |= self.extend_with_meta(meta)?;
        }

        Ok(supported)
    }

    /// Extend the map with the given meta.
    ///
    /// Returns `Ok(false)` if the property is not supported.
    pub fn extend_with_meta(&mut self, meta: &Meta) -> Result<bool> {
        let property = meta.path().require_ident()?.to_string();

        let Some(states) = self.0.get_mut(&property) else {
            return Ok(false);
        };

        meta.require_list()?.parse_nested_meta(|meta| {
            let state = meta.path.require_ident().cloned()?;
            states.push(state);
            Ok(())
        })?;

        Ok(true)
    }
}
