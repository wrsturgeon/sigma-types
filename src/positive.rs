//! Positive types (defined by comparison to zero).

use {
    crate::{Sigma, Test, Zero},
    core::{fmt, marker::PhantomData},
};

/// A term expected to be positive was not.
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct NonPositive<'i, Input: fmt::Debug + PartialOrd + Zero>(&'i Input);

impl<Input: fmt::Debug + PartialOrd + Zero> fmt::Display for NonPositive<'_, Input> {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        #![expect(
            clippy::use_debug,
            reason = "Intentional and informative, not just forgotten print-debugging"
        )]

        let Self(z) = *self;
        write!(f, "{z:#?} <= {:#?}", Input::ZERO)
    }
}

/// Positive terms (defined by comparison to zero).
pub type Positive<Input> = Sigma<Input, PositiveInvariant<Input>>;

/// Positive terms (defined by comparison to zero).
#[derive(Copy, Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct PositiveInvariant<Input: fmt::Debug + PartialOrd + Zero>(PhantomData<Input>);

impl<Input: fmt::Debug + PartialOrd + Zero> Test<Input> for PositiveInvariant<Input> {
    const ADJECTIVE: &str = "positive";
    type Error<'i>
        = NonPositive<'i, Input>
    where
        Input: 'i;

    #[inline(always)]
    fn test([input]: [&Input; 1]) -> Result<(), Self::Error<'_>> {
        if *input > Input::ZERO {
            Ok(())
        } else {
            Err(NonPositive(input))
        }
    }
}
