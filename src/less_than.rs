//! Terms less than a constant (defined by `PartialOrd` comparison).

#![expect(
    clippy::arbitrary_source_item_ordering,
    reason = "macros need to be defined before they're used"
)]

/// Make a type-specific module, since (unfortunately) we can't use dependent types.
macro_rules! mk_mod {
    ($t:ident) => {
        pub mod $t {
            //! Terms of type `$t` less than a constant (defined by `PartialOrd` comparison).

            use {
                crate::{Sigma, Test},
                core::fmt,
            };

            /// Terms less than a constant (defined by `PartialOrd` comparison).
            pub type LessThan<const N: $t> = Sigma<$t, LessThanInvariant<N>>;

            /// Terms less than a constant (defined by `PartialOrd` comparison).
            #[expect(clippy::exhaustive_structs, reason = "no fields")]
            #[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
            pub struct LessThanInvariant<const N: $t>;

            impl<const N: $t> Test<$t, 1> for LessThanInvariant<N> {
                const ADJECTIVE: &str = "positive";
                type Error<'i>
                    = NotLessThan<'i, N>
                where
                    $t: 'i;

                #[inline(always)]
                fn test([input]: [&$t; 1]) -> Result<(), Self::Error<'_>> {
                    if *input < N {
                        Ok(())
                    } else {
                        Err(NotLessThan(input))
                    }
                }
            }

            /// A term expected to be positive was not.
            #[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
            pub struct NotLessThan<'i, const N: $t>(&'i $t);

            impl<const N: $t> fmt::Display for NotLessThan<'_, N> {
                #[inline]
                fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                    /*
                    #![expect(
                        clippy::use_debug,
                        reason = "Intentional and informative, not just forgotten print-debugging"
                    )]
                    */

                    let Self(z) = *self;
                    write!(f, "{z:#?} >= {N:#?}")
                }
            }
        }
    };
}

mk_mod!(i8);
mk_mod!(i16);
mk_mod!(i32);
mk_mod!(i64);
mk_mod!(i128);
mk_mod!(isize);
mk_mod!(u8);
mk_mod!(u16);
mk_mod!(u32);
mk_mod!(u64);
mk_mod!(u128);
mk_mod!(usize);
