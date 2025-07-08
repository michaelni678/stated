#![cfg_attr(docsrs, feature(doc_auto_cfg))]

//! Stated simplifies working with the typestate pattern.

extern crate self as stated;

pub use stated_macros::{stated, stated_internal};

#[cfg(feature = "guide")]
pub mod guide;

/// Indicates a disabled state.
#[derive(Debug, Default, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct N;

/// Indicates an enabled state.
#[derive(Debug, Default, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Y;

/// Placeholder for a stateless type.
pub struct __;
