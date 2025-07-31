//! Showcase of each documentation attribute.
//!
//! [`Default`] is generated with no documentation attributes.
//!
//! [`Description`] is generated with the `description` attribute.
//!
//! Rust uses the first line of documentation on a struct as a summary in the
//! module documentation. Stated will not show the description as a summary. If
//! the description is the only documentation on the struct, the summary will be
//! left blank.
//!
//! [`DescriptionDocumented`] is generated with the `description` attribute and
//! some documentation comments.
//!
//! Notice how you can now see a summary ("Hello, World!") on the struct in the
//! module documentation.
//!
//! [`Ugly`] is generated with the `ugly` attribute.
//!
//! The documentation is the actual typestate code.
//!
//! [`DescriptionUgly`] is generated with the `description` and `ugly`
//! attributes.

use stated::stated;

macro_rules! define {
    ($name:ident @ $($docs:ident),* @ $($comments:literal),*) => {
        $(#[doc = $comments])*
        #[stated(states(StateA, StateB, StateC), preset(StateC), docs($($docs),*))]
        pub struct $name<#[stated] S> {
            whatever: (),
        }

        #[stated]
        impl<#[stated] S> $name<S> {
            $(#[doc = $comments])*
            #[stated]
            pub fn new() -> $name<_> {
                $name {}
            }

            $(#[doc = $comments])*
            #[stated(assert(StateA))]
            pub fn foo(self) -> $name<_> {
                _
            }

            $(#[doc = $comments])*
            #[stated(reject(StateA, StateB))]
            pub fn bar(self, x: i32) -> $name<_> {
                _
            }

            $(#[doc = $comments])*
            #[stated(assert(StateA), reject(StateB), assign(StateB))]
            pub fn baz(self, y: impl Into<String>) -> $name<_> {
                _
            }

            $(#[doc = $comments])*
            #[stated(delete(StateB, StateC))]
            pub fn qux<T>(self, z: T) -> $name<_>
            where
                T: ToString,
            {
                _
            }
        }
    };
}

define!(Default @ @ );
define!(Description @ description @ );
define!(DescriptionDocumented @ description @ "Hello, World!");
define!(Ugly @ ugly @ );
define!(DescriptionUgly @ description, ugly @ );
