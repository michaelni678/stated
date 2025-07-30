//! Drawbacks to consider.
//!
//! The typestate pattern has several limitations, some of which Stated
//! addresses.
//!
//! # Boilerplate
//!
//! The typestate pattern has a lot of boilerplate. The whole point of Stated is
//! to reduce boilerplate!
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
//! Stated does not address this limitation.
//!
//! # Bad Compiler Errors
//!
//! The complex generics used by the typestate pattern often lead to confusing
//! compiler errors. Stated does not address this limitation.
//!
//! # Self Type
//!
//! The typestate pattern doesn't work well with `Self`. `Self` refers to the
//! current type being implemented, but the generics of this type may be changed
//! for the typestate pattern to work. Stated does not address this limitation.
