//! Positive types (defined by comparison to zero).

use {
    crate::{Sigma, Test, Zero},
    core::{fmt, marker::PhantomData},
};

/// A term expected to be positive was not.
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct NonPositive<Z: Clone + fmt::Debug + PartialOrd + Zero>(Z);

impl<Z: Clone + fmt::Debug + PartialOrd + Zero> fmt::Display for NonPositive<Z> {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        #![expect(
            clippy::use_debug,
            reason = "Intentional and informative, not just forgotten print-debugging"
        )]

        let Self(ref z) = *self;
        write!(f, "{z:#?} <= {:#?}", Z::ZERO)
    }
}

/// Positive types (defined by comparison to zero).
pub type Positive<Z> = Sigma<Z, PositiveInvariant<Z>>;

/// Positive types (defined by comparison to zero).
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct PositiveInvariant<Z: Clone + fmt::Debug + PartialOrd + Zero>(PhantomData<Z>);

impl<Z: Clone + fmt::Debug + PartialOrd + Zero> Default for PositiveInvariant<Z> {
    #[inline(always)]
    fn default() -> Self {
        Self(PhantomData)
    }
}

impl<Z: Clone + fmt::Debug + PartialOrd + Zero> Test<Z> for PositiveInvariant<Z> {
    const ADJECTIVE: &str = "positive";
    type Error = NonPositive<Z>;

    #[inline(always)]
    fn test(input: &Z) -> Result<(), Self::Error> {
        if *input > Z::ZERO {
            Ok(())
        } else {
            Err(NonPositive(input.clone()))
        }
    }
}
