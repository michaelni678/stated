//! Special syntax.
//!
//! # Designate
//!
//! A generic parameter on struct definitions and impl blocks must be
//! "designated" by marking it with the `stated` attribute. Parameters
//! designated on struct definitions are used to track states. Parameters
//! designated on impl blocks are replaced by generic parameters used by Stated.
//!
//! ## Example
//!
//! In the code below, generic parameter `U` is designated.
//!
//! ```
//! # {} /*
//! #[stated(...)]
//! struct Example<T, #[stated] U, V> {
//!     ...
//! }
//!
//! #[stated]
//! impl<T, #[stated] U, V> Example<T, U, V> {
//!     ...
//! }
//! # */
//! ```
//!
//! # Infer
//!
//! In the return type of an associated function, the inferred type (`_`) is
//! replaced with the outgoing state type.
//!
//! In the body of a method, the inferred expression (`_`)
//! [reconstructs](#reconstruct-method) `self` with the outgoing state type.
//!
//! ## Example
//!
//! In the code below, the inferred type in `Example<T, _, V>` is replaced with
//! the outgoing state type, and the inferred expression in the method body will
//! return this modified return type.
//!
//! ```
//! # {} /*
//! #[stated]
//! impl<T, #[stated] U, V> Example<T, U, V> {
//!     #[stated(...)]
//!     fn foo(self) -> Example<T, _, V> {
//!         _
//!     }
//!
//!     #[stated(...)]
//!     fn bar(self) -> Result<Example<T, _, V>, &'static str> {
//!         if ... {
//!             return Err("error!"),
//!         } else if ... {
//!             return Ok(_);
//!         }
//!
//!         ...
//!         Ok(_)
//!     }
//! }
//! # */
//! ```
