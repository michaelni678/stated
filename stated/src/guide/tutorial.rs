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
//! In the code below, states `A`, `B`, and `C` are declared, with state `C`
//! being preset. When `new` is called, states `A` and `B` will default to
//! disabled, and state `C` will default to enabled.
//!
//! ### Example
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
