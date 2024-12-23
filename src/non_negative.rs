//! Non-negative types (defined by comparison to zero).

use {
    crate::{Sigma, Test, Zero},
    core::{fmt, marker::PhantomData},
};

/// A term expected to be non-negative was, in fact, negative.
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Negative<'z, Z: fmt::Debug + PartialOrd + Zero>(&'z Z);

impl<Z: fmt::Debug + PartialOrd + Zero> fmt::Display for Negative<'_, Z> {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        #![expect(
            clippy::use_debug,
            reason = "Intentional and informative, not just forgotten print-debugging"
        )]

        let Self(z) = *self;
        write!(f, "{z:#?} < {:#?}", Z::ZERO)
    }
}

/// Non-negative types (defined by comparison to zero).
pub type NonNegative<Z> = Sigma<Z, NonNegativeInvariant<Z>>;

/// Non-negative types (defined by comparison to zero).
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct NonNegativeInvariant<Z: fmt::Debug + PartialOrd + Zero>(PhantomData<Z>);

impl<Z: fmt::Debug + PartialOrd + Zero> NonNegativeInvariant<Z> {
    /// `const` version of `Default::default`.
    #[inline(always)]
    #[cfg(debug_assertions)]
    #[expect(
        clippy::single_call_fn,
        reason = "Useful in another file with different visibility"
    )]
    pub(crate) const fn new() -> Self {
        Self(PhantomData)
    }
}

impl<Z: fmt::Debug + PartialOrd + Zero> Default for NonNegativeInvariant<Z> {
    #[inline(always)]
    fn default() -> Self {
        Self(PhantomData)
    }
}

impl<Z: fmt::Debug + PartialOrd + Zero> Test<Z> for NonNegativeInvariant<Z> {
    const ADJECTIVE: &str = "non-negative";
    type Error<'input>
        = Negative<'input, Z>
    where
        Z: 'input;

    #[inline(always)]
    fn test(input: &Z) -> Result<(), Self::Error<'_>> {
        if *input >= Z::ZERO {
            Ok(())
        } else {
            Err(Negative(input))
        }
    }
}
