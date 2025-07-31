//! Documentation generation.
//!
//! Since the typestate pattern uses lots of generics, documentation can be
//! nearly incomprehensible. Stated automatically generates cleaner
//! documentation. However, this documentation is **inaccurate** as it hides all
//! the generics Stated uses. This may be misleading! See the [ugly
//! attribute](#ugly) attribute to opt out.
//!
//! # Attributes
//!
//! Attributes can be added to modify the generated documentation.
//!
//! ## Description
//!
//! Stated can generate a description of the states on the struct definition and
//! associated functions with the `description` attribute.
//!
//! ## Ugly
//!
//! The `ugly` documentation attribute generates the actual documentation that
//! makes the typestate pattern possible.
//!
//! ### Example
//!
//! In the code below, the generated documentation is modified with the
//! `description` and `ugly` attributes.
//!
//! ```
//! # {} /*
//! #[stated(states(A, B, C), docs(description, ugly))]
//! struct ...
//! # */
//! ```

pub mod examples;
