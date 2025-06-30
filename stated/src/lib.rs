//! Stated simplifies working with the typestate pattern.

extern crate self as stated;

pub use stated_macros::{stated, stated_internal};

/// Indicates a disabled state.
#[derive(Debug, Default, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct N;

/// Indicates an enabled state.
#[derive(Debug, Default, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Y;

/// Placeholder for a stateless type.
pub struct __;
