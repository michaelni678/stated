//! How to use Stated.
//!
//! # Managing States
//!
//! A state is an identifier that can be either [disabled](`stated::N`) or
//! [enabled](`stated::Y`).
//!
//! States are [declared](#declare-and-preset) and [preset](#declare-and-preset)
//! where the struct is defined. States are [asserted](#assert-and-reject),
//! [rejected](#assert-and-reject), [assigned](#assign-and-delete), and
//! [deleted](#assign-and-delete) where an associated function is defined.
//!
//! ## Declare and Preset
//!
//! Use the `states` attribute to declare states. All states that are
//! [preset](#declare-and-preset), [asserted](#assert-and-reject),
//! [rejected](#assert-and-reject), [assigned](#assign-and-delete), and
//! [deleted](#assign-and-delete) must be declared.
//!
//! States default to being disabled when constructed. Use the `preset`
//! attribute to enable the specified states when constructed.
//!
//! ### Example
//!
//! In the code below, states `A`, `B`, and `C` are declared, with state `C`
//! being preset. When `new` is called, states `A` and `B` will default to
//! disabled, and state `C` will default to enabled.
//!
//! ```
//! # {} /*
//! #[stated(states(A, B, C), preset(C))]
//! struct Example<#[stated] S> {
//!     ...
//! }
//!
//! #[stated]
//! impl<#[stated] S> Example<S> {
//!     #[stated]
//!     fn new() -> Example<_> {
//!         ...
//!     }
//! }
//! # */
//! ```
//!
//! ## Assert and Reject
//!
//! When calling an associated function, ensure a state is enabled with the
//! `assert` attribute or disabled with the `reject` attribute.
//!
//! ## Assign and Delete
//!
//! When calling an associated function, transition a state to be enabled with
//! the `assign` attribute or disabled with the `delete` attribute.
//!
//! A state can be assigned even if it is already enabled, and it can be deleted
//! even if it is already disabled.
//!
//! ## Example
//!
//! In the code below, `foo` can only be called if state `A` is enabled. The
//! `bar` method requires states `B` and `C` to be disabled, and it transitions
//! state `A` to enabled.
//!
//! ```
//! # {} /*
//! #[stated]
//! impl<#[stated] S> Example<S> {
//!     #[stated(assert(A))]
//!     fn foo(self) -> Example<_> {
//!         ...
//!     }
//!
//!     #[stated(reject(B, C), assign(A))]
//!     fn bar(self) -> Example<_> {
//!         ...
//!     }
//! }
//! # */
//! ```
//!
//! Associated functions with a receiver can assert and reject states.
//! Associated functions that return the struct can assign and delete states.
//!
//! In the code below, `baz` requires states `A` and `C` to be enabled. State
//! `B` transitions to enabled when `new2` is called.
//!
//! ```
//! # {} /*
//! #[stated]
//! impl<#[stated] S> Example<S> {
//!     #[stated(assert(A, C))]
//!     fn baz(self) {
//!         ...
//!     }
//!
//!     #[stated(assign(B))]
//!     fn new2() -> Example<_> {
//!         ...
//!     }
//! }
//! # */
//! ```
//!
//! # Syntax
//!
//! Stated has some special syntax.
//!
//! ## Designate
//!
//! A generic parameter on struct definitions and impl blocks must be
//! "designated" by marking it with the `stated` attribute. Parameters
//! designated on struct definitions are used to track states. Parameters
//! designated on impl blocks are replaced by generic parameters used by Stated.
//!
//! ### Example
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
//! ## Infer
//!
//! In the return type of an associated function, the inferred type (`_`) is
//! replaced with the outgoing state type.
//!
//! In the body of a method, the inferred expression (`_`)
//! [reconstructs](#reconstruct-method) `self` with the outgoing state type.
//!
//! ### Example
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
//!
//! # Documentation
//!
//! Since the typestate pattern uses lots of generics, documentation can be
//! nearly incomprehensible. Stated automatically generates cleaner
//! documentation. However, this documentation is **inaccurate** as it hides all
//! the generics Stated uses. This may be misleading!
//!
//! The documentation Stated generates for the example in the [README](https://github.com/michaelni678/stated/blob/main/README.md)
//! can be found in the [examples::read_me module](super::examples::read_me).
//!
//! The generated documentation can be configured with documentation attributes.
//!
//! ## Description
//!
//! Stated can generate a description of the states on the struct definition and
//! associated functions with the `description` documentation attribute.
//!
//! Documentation generated using the `description` documentation attribute can
//! be found in the [examples::description
//! module](super::examples::description).
//!
//! ### Example
//!
//! ```
//! # {} /*
//! #[stated(states(A, B, C), docs(description))]
//! struct ...
//! # */
//! ```
//!
//! ## Ugly
//!
//! The `ugly` documentation attribute generates the actual documentation that
//! makes the typestate pattern possible.
//!
//! Documentation generated using the `ugly` documentation attribute can be
//! found in the [examples::ugly module](super::examples::ugly).
//!
//! ### Example
//!
//! ```
//! # {} /*
//! #[stated(states(A, B, C), docs(ugly))]
//! struct ...
//! # */
//! ```
//!
//! # Limitations
//!
//! The typestate pattern has several limitations, some of which Stated aims to
//! address.
//!
//! ## Boilerplate
//!
//! The typestate pattern has a lot of boilerplate. The whole point of Stated is
//! to solve this limitation!
//!
//! ## Bad Documentation
//!
//! Since the typestate pattern uses lots of generics, documentation can be
//! nearly incomprehensible. Stated generates [cleaner but inaccurate
//! documentation](#documentation).
//!
//! ## Worse Compile Times
//!
//! The typestate pattern uses a lot of generics, which increases compile time.
//! Stated does not attempt to solve this limitation.
//!
//! ## Self Type
//!
//! The typestate pattern doesn't work well with `Self`. `Self` refers to the
//! current type being implemented, but the generics of this type may be changed
//! for the typestate pattern to work. Stated does not attempt to solve this
//! limitation.
//!
//! # Expansion Behavior
//!
//! Overview of the modifications and additions the Stated macro makes to your
//! code behind the scenes.
//!
//! ## Phantom Field
//!
//! Stated adds a [`PhantomData`](std::marker::PhantomData) field to your struct
//! to track states. When instantiating the struct, Stated will fill this field
//! for you.
//!
//! ### Example
//!
//! In the code below, Stated secretly adds a phantom field to `Example`.
//! Despite this, you don’t need to manually set the field inside the `new`
//! function!
//!
//! ```
//! # {} /*
//! #[stated(...)]
//! struct Example<#[stated] S> {
//!     number: i32,
//!     string: String,
//! }
//!
//! #[stated]
//! impl<#[stated] S> Example<S> {
//!     #[stated]
//!     fn new() -> Example<_> {
//!         Example {
//!             number: 0,
//!             string: String::new(),
//!         }
//!     }
//! }
//! # */
//! ```
//!
//! ## Reconstruct Method
//!
//! Stated adds a private method to assist with transitioning states. This
//! method replaces [inferred expressions](#infer) in the method body.
//!
//! ## Token Export Macro
//!
//! Stated defines a private macro to export declared and preset states from the
//! struct definition to the impl block.
