//! Drawbacks to consider.
//!
//! The typestate pattern has several limitations. Stated helps with some of
//! them, but not all.
//!
//! # Boilerplate
//!
//! The typestate pattern has a lot of boilerplate. Stated reduces the amount of
//! code you'll need to write significantly.
//!
//! # Bad Documentation
//!
//! Since the typestate pattern uses lots of generics, documentation can be
//! nearly incomprehensible. Stated generates [cleaner but inaccurate
//! documentation](super::documentation).
//!
//! # Increased Compile Times
//!
//! The typestate pattern uses a lot of generics, which increases compile time.
//! Stated does not attempt to decrease compile time.
//!
//! # Bad Compiler Errors
//!
//! The complex generics used by the typestate pattern often lead to confusing
//! compiler errors. Stated does not address this limitation.
//!
//! # No Self Type
//!
//! The typestate pattern doesn't work well with `Self`. This is due to generic
//! state changes altering the type that `Self` refers to. Stated does not
//! attempt to fix this.
