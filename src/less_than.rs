//! Terms less than a constant (defined by `PartialOrd` comparison).

pub mod usize {
    //! Terms of type `usize` less than a constant (defined by `PartialOrd` comparison).

    use {
        crate::{Sigma, Test},
        core::fmt,
    };

    /// Terms less than a constant (defined by `PartialOrd` comparison).
    pub type LessThan<const N: usize> = Sigma<usize, LessThanInvariant<N>>;

    /// Terms less than a constant (defined by `PartialOrd` comparison).
    #[expect(clippy::exhaustive_structs, reason = "no fields")]
    #[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
    pub struct LessThanInvariant<const N: usize>;

    impl<const N: usize> Test<usize, 1> for LessThanInvariant<N> {
        const ADJECTIVE: &str = "positive";
        type Error<'i>
            = NotLessThan<'i, N>
        where
            usize: 'i;

        #[inline(always)]
        fn test([input]: [&usize; 1]) -> Result<(), Self::Error<'_>> {
            if *input < N {
                Ok(())
            } else {
                Err(NotLessThan(input))
            }
        }
    }

    /// A term expected to be positive was not.
    #[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    pub struct NotLessThan<'i, const N: usize>(&'i usize);

    impl<const N: usize> fmt::Display for NotLessThan<'_, N> {
        #[inline]
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            #![expect(
                clippy::use_debug,
                reason = "Intentional and informative, not just forgotten print-debugging"
            )]

            let Self(z) = *self;
            write!(f, "{z:#?} >= {N:#?}")
        }
    }
}
