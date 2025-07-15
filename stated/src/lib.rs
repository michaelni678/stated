#![cfg_attr(docsrs, feature(doc_auto_cfg))]

//! Stated simplifies working with the typestate pattern.
//!
//! See the [guide] for more detailed explanation of Stated.
//!
//! # Quick Start
//!
//! On a struct definition, add the [`stated`](`macro@stated`) attribute.
//! [Declare](guide::tutorial#declare--preset) the states you need and
//! [preset](guide::tutorial#declare--preset) the states you want enabled by
//! default.
//!
//! Then, add a [designated parameter](guide::tutorial#designate) to track
//! states.
//!
//! ```
//! # /*
//! #[stated(states(A, B, C), preset(C))]
//! pub struct Example<#[stated] S> {
//! # // Stops automatic formatting, `rustfmt::skip` behaves strange here.
//!     /* ... */
//! }
//! # */
//! #
//! # mod __ {} // Suppresses `rustdoc::invalid_rust_codeblocks` on this block.
//! ```
//!
//! On the impl block, add the [`stated`](`macro@stated`) attribute.
//!
//! Then add a [designated parameter](guide::tutorial#designate).
//!
//! ```
//! # /*
//! #[stated]
//! impl<#[stated] S> Example<S> {
//!     /* ... */
//! }
//! # */
//! #
//! # mod __ {} // Suppresses `rustdoc::invalid_rust_codeblocks` on this block.
//! ```
//!
//! On an associated function, add the `stated` attribute. Require a state to be
//! either enabled or disabled by [asserting](guide::tutorial#assert--reject) or
//! [rejecting](guide::tutorial#assert--reject) them.
//! [Assign](guide::tutorial#assign--delete) or
//! [delete](guide::tutorial#assign--delete) states to transition them from
//! disabled to enabled or vice versa.
//!
//! Use `_` in the return type to [infer](guide::tutorial#infer) the outgoing
//! state type. Use it in the function body to reconstruct `self` to the correct
//! outgoing state type.
//!
//! ```
//! # /*
//! #[stated]
//! impl<#[stated] S> Example<S> {
//!     #[stated(assert(A), reject(C), assign(B))]
//!     pub fn foo(mut self, x: i32) -> Example<_> {
//!         /* ... */
//!         _
//!     }
//! }
//! # */
//! #
//! # mod __ {} // Suppresses `rustdoc::invalid_rust_codeblocks` on this block.
//! ```
//! 
//! See the [tutorial](guide::tutorial) for more help!

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
