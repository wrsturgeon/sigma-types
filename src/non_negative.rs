//! Non-negative types (defined by comparison to zero).

use {
    crate::{Sigma, Test, Zero},
    core::{fmt, marker::PhantomData},
};

/// A term expected to be non-negative was, in fact, negative.
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Negative<Z: Clone + fmt::Debug + PartialOrd + Zero>(Z);

impl<Z: Clone + fmt::Debug + PartialOrd + Zero> fmt::Display for Negative<Z> {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        #![expect(
            clippy::use_debug,
            reason = "Intentional and informative, not just forgotten print-debugging"
        )]

        let Self(ref z) = *self;
        write!(f, "{z:#?} < {:#?}", Z::ZERO)
    }
}

/// Non-negative terms (defined by comparison to zero).
pub type NonNegative<Z> = Sigma<Z, NonNegativeInvariant<Z>>;

/// Non-negative terms (defined by comparison to zero).
#[derive(Copy, Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct NonNegativeInvariant<Z: Clone + fmt::Debug + PartialOrd + Zero>(PhantomData<Z>);

impl<Z: Clone + fmt::Debug + PartialOrd + Zero> Test<Z> for NonNegativeInvariant<Z> {
    const ADJECTIVE: &str = "non-negative";
    type Error = Negative<Z>;

    #[inline(always)]
    fn test(input: &Z) -> Result<(), Self::Error> {
        if *input >= Z::ZERO {
            Ok(())
        } else {
            Err(Negative(input.clone()))
        }
    }
}
