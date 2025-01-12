//! Negative terms (defined by comparison to zero).

use {
    crate::{Sigma, Test, Zero},
    core::{fmt, marker::PhantomData},
};

/// Negative terms (defined by comparison to zero).
pub type Negative<Input> = Sigma<Input, NegativeInvariant<Input>>;

/// Negative terms (defined by comparison to zero).
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct NegativeInvariant<Input: fmt::Debug + PartialOrd + Zero>(PhantomData<Input>);

impl<Input: fmt::Debug + PartialOrd + Zero> Test<Input, 1> for NegativeInvariant<Input> {
    const ADJECTIVE: &str = "positive";
    type Error<'i>
        = NotNegative<'i, Input>
    where
        Input: 'i;

    #[inline(always)]
    fn test([input]: [&Input; 1]) -> Result<(), Self::Error<'_>> {
        if *input < Input::ZERO {
            Ok(())
        } else {
            Err(NotNegative(input))
        }
    }
}

/// A term expected to be positive was not.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct NotNegative<'i, Input: fmt::Debug + PartialOrd + Zero>(&'i Input);

impl<Input: fmt::Debug + PartialOrd + Zero> fmt::Display for NotNegative<'_, Input> {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        #![expect(
            clippy::use_debug,
            reason = "Intentional and informative, not just forgotten print-debugging"
        )]

        let Self(z) = *self;
        write!(f, "{z:#?} >= {:#?}", Input::ZERO)
    }
}
