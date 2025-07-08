//! Stated simplifies working with the typestate pattern.
//!
//! # Managing States
//!
//! A state is an identifier that can be either [disabled](`N`) or
//! [enabled](`Y`).
//!
//! States are [declared](#declare--preset) and [preset](#declare--preset) where
//! the struct is defined. States are [asserted](#assert--reject),
//! [rejected](#assert--reject), [assigned](#assign--delete), and
//! [deleted](#assign--delete) where an associated function is defined.
//!
//! ## Declare / Preset
//!
//! Use the `states` property to declare states. All states that are
//! [preset](#declare--preset), [asserted](#assert--reject),
//! [rejected](#assert--reject), [assigned](#assign--delete), and
//! [deleted](#assign--delete) must be declared.
//!
//! States default to being disabled when constructed. Use the `preset`
//! property to enable the specified states when constructed.
//!
//! ## Assert / Reject
//!
//! When calling an associated function, ensure a state is enabled with the
//! `assert` property or disabled with the `reject` property.
//!
//! ## Assign / Delete
//!
//! When calling an associated function, transition a state to be enabled with
//! the `assign` property or disabled with the `delete` property.
//!
//! # Syntax
//!
//! Stated has some special syntax.
//!
//! ## Designate
//!
//! A generic parameter must be marked with the `stated` attribute on struct
//! definitions and impl blocks.
//!
//! Parameters designated on struct definitions are used to track states.
//! Parameters designated on impl blocks are replaced by generic parameters not
//! used by Stated functions.
//!
//! ## Infer
//!
//! In the return type of an associated function, `_` is replaced with the
//! outgoing state type.
//!
//! In the body of a method, `_` reconstructs `self` with the outgoing state
//! type.
//!
//! # Example
//!
//! Let's dissect the example from the [README](https://github.com/michaelni678/stated/blob/main/README.md).
//! This example is also available in the [examples module](examples).
//!
//! The `MessageBuilder` struct is defined with states `HasRecipient` and
//! `HasBody` [declared](#declare--preset). No states are
//! [preset](#declare--preset). The generic parameter `S` is
//! [designated](#designate).
//!
//! ```
//! # /*
//! #[stated(states(HasRecipient, HasBody))]
//! pub struct MessageBuilder<#[stated] S> {
//!     recipients: Vec<String>,
//!     body: String,
//! }
//! # */
//! #
//! # mod __ {} // Suppresses `rustdoc::invalid_rust_codeblocks` on this block.
//! ```
//!
//! When `MessageBuilder` is constructed with `MessageBuilder::new`, states
//! `HasRecipient` and `HasBody` are disabled. The outgoing state type is
//! automatically [inferred](#infer).
//!
//! ```
//! # /*
//! #[stated]
//! impl<#[stated] S> MessageBuilder<S> {
//!     #[stated]
//!     pub fn new() -> MessageBuilder<_> {
//!         MessageBuilder {
//!             recipients: Vec::new(),
//!             body: String::new(),
//!         }
//!     }
//!
//!     /* ... */
//! }
//! # */
//! #
//! # mod __ {} // Suppresses `rustdoc::invalid_rust_codeblocks` on this block.
//! ```
//!
//! When `MessageBuilder::recipient` is called, it [assigns](#assign--delete)
//! the `HasRecipient` state. Subsequent calls will see this state is enabled.
//! In the return type, the outgoing state type is automatically
//! [inferred](#infer). In the function body, `_` is replaced to return the
//! proper `MessageBuilder`.
//!
//! ```
//! # /*
//! #[stated]
//! impl<#[stated] S> MessageBuilder<S> {
//!     /* ... */
//!
//!     #[stated(assign(HasRecipient))]
//!     pub fn recipient(mut self, recipient: impl Into<String>) -> MessageBuilder<_> {
//!         self.recipients.push(recipient.into());
//!         _
//!     }
//!
//!     /* ... */
//! }
//! # */
//! #
//! # mod __ {} // Suppresses `rustdoc::invalid_rust_codeblocks` on this block.
//! ```
//!
//! When `MessageBuilder::body` is called, it both [rejects](#assert--reject)
//! and [assigns](#assign--delete) the `HasBody` state. Since `MessageBuilder`
//! does not have a method that [deletes](#assign--delete) the `HasBody` state,
//! `MessageBuilder::body` can only be called once.
//!
//! In the return type, the outgoing state type is automatically
//! [inferred](#infer). In the function body, `_` is replaced to return the
//! proper `MessageBuilder`.
//!
//! ```
//! # /*
//! #[stated]
//! impl<#[stated] S> MessageBuilder<S> {
//!     /* ... */
//!
//!     #[stated(reject(HasBody), assign(HasBody))]
//!     pub fn body(mut self, body: impl Into<String>) -> Result<MessageBuilder<_>, &'static str> {
//!         let body = body.into();
//!         if !body.is_ascii() {
//!             return Err("Body contains non-ASCII characters");
//!         }
//!
//!         self.body = body;
//!         Ok(_)
//!     }
//!
//!     /* ... */
//! }
//! # */
//! #
//! # mod __ {} // Suppresses `rustdoc::invalid_rust_codeblocks` on this block.
//! ```
//!
//! `MessageBuilder::build` [asserts](#assert--reject) that a recipient was
//! added. It returns a `Message` (a type-aliased `String`).
//!
//! ```
//! pub type Message = String;
//!
//! /* ... */
//!
//! # /*
//! #[stated]
//! impl<#[stated] S> MessageBuilder<S> {
//!     /* ... */
//!
//!     #[stated(assert(HasRecipient))]
//!     pub fn build(self) -> Message {
//!         let to = self.recipients.join(" & ");
//!         let mut body = self.body;
//!
//!         if body.is_empty() {
//!             body.push_str("<empty body>");
//!         }
//!
//!         format!("To: {to}\n{body}")
//!     }
//! }
//! # */
//! ```
//!
//! `MessageBuilder` can be used like any other builder...
//!
//! ```
//! # use stated::{stated, examples::read_me::MessageBuilder};
//! #
//! let message = MessageBuilder::new()
//!     .recipient("Bob")
//!     .recipient("Rob")
//!     .body("Hello, World!")?
//!     .build();
//!
//! assert_eq!(message, "To: Bob & Rob\nHello, World!");
//! ```
//!
//! ```
//! # use stated::{stated, examples::read_me::MessageBuilder};
//! #
//! let message = MessageBuilder::new().recipient("Bob").build();
//!
//! assert_eq!(message, "To: Bob\n<empty body>");
//! #
//! # Ok::<_, &'static str>(())
//! ```
//!
//! ... but improper usage results in a compilation error!
//!
//! ```compile_fail,E0599
//! # use stated::{stated, examples::read_me::MessageBuilder};
//! #
//! let message = MessageBuilder::new()
//!     .body("Hello, World!")?
//!     .build(); // HasRecipient assertion fails.
//! #
//! # Ok::<_, &'static str>(())
//! ```
//!
//! ```compile_fail,E0599
//! # use stated::{stated, examples::read_me::MessageBuilder};
//! #
//! let message = MessageBuilder::new()
//!     .recipient("Bob")
//!     .body("Hello, World!")?
//!     .body("Hello, again...")? // HasBody rejection fails.
//!     .build();
//! #
//! # Ok::<_, &'static str>(())
//! ```
//!
//! # Other Stuff
//!
//! ## Export and Import
//!
//! Stated creates a macro to export tokens from the struct definition. These
//! tokens are imported by the impl block. This is to allow the states to be
//! declared alongside the struct definition, even though the impl block is what
//! needs those declared states. There are some edge cases where this won't
//! work, such as when more than one Stated struct have the same name.
//!
//! ```compile_fail,E0428
//! mod a {
//!     # use stated::stated;
//!     #
//!     #[stated(states(A, B, C))]
//!     pub struct Example<#[stated] S>;
//!
//!     /* ... */
//! }
//!
//! mod b {
//!     # use stated::stated;
//!     #
//!     #[stated(states(X, Y, Z))]
//!     pub struct Example<#[stated] S>;
//!
//!     #[stated]
//!     impl<#[stated] S> Example<S> {
//!         /* ... */
//!     }
//! }
//! ```
//!
//! To get around this, you can specify an export and import name. The import
//! name must match the export name of the struct to import tokens from.
//!
//! ```
//! mod a {
//!     # use stated::stated;
//!     #
//!     #[stated(states(A, B, C))]
//!     pub struct Example<#[stated] S>;
//!
//!     /* ... */
//! }
//!
//! mod b {
//!     # use stated::stated;
//!     #
//!     #[stated(states(X, Y, Z), export = OtherExample)]
//!     pub struct Example<#[stated] S>;
//!
//!     #[stated(import = OtherExample)]
//!     impl<#[stated] S> Example<S> {
//!         /* ... */
//!     }
//! }
//! ```
//!
//! ## Internal Macro
//!
//! If you don't want to rely on the impl block importing tokens from the struct
//! definition, you can declare states directly on the impl block. Stated
//! exposes the [`stated_internal`] macro for this purpose.
//!
//! ```
//! use stated::stated_internal;
//!
//! #[stated_internal]
//! pub struct Example<#[stated] S>;
//!
//! #[stated_internal(states(A, B, C), preset(C))]
//! impl<#[stated] S> Example<S> {
//!     /* ... */
//! }
//! ```
//!
//! At first glance, this may look like you can have different states declared
//! for each impl block, but that won't work as intended. To avoid making
//! mistakes, use the normal [`macro@stated`] macro when possible.

extern crate self as stated;

pub use stated_macros::{stated, stated_internal};

#[cfg(any(docsrs, doctest))]
pub mod examples {
    /// The example on the [README](https://github.com/michaelni678/stated/blob/main/README.md).
    pub mod read_me;
}

/// Indicates a disabled state.
#[derive(Debug, Default, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct N;

/// Indicates an enabled state.
#[derive(Debug, Default, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Y;

/// Placeholder for a stateless type.
pub struct __;
