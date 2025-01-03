//! Non-positive types (defined by comparison to zero).

use {
    crate::{Sigma, Test, Zero},
    core::{fmt, marker::PhantomData},
};

/// Non-positive terms (defined by comparison to zero).
pub type NonPositive<Input> = Sigma<Input, NonPositiveInvariant<Input>>;

/// Non-positive terms (defined by comparison to zero).
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct NonPositiveInvariant<Input: fmt::Debug + PartialOrd + Zero>(PhantomData<Input>);

impl<Input: fmt::Debug + PartialOrd + Zero> Test<Input, 1> for NonPositiveInvariant<Input> {
    const ADJECTIVE: &str = "non-positive";
    type Error<'i>
        = NotNonPositive<'i, Input>
    where
        Input: 'i;

    #[inline(always)]
    fn test([input]: [&Input; 1]) -> Result<(), Self::Error<'_>> {
        if *input <= Input::ZERO {
            Ok(())
        } else {
            Err(NotNonPositive(input))
        }
    }
}

/// A term expected to be non-positive was, in fact, positive.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct NotNonPositive<'i, Input: fmt::Debug + PartialOrd + Zero>(&'i Input);

impl<Input: fmt::Debug + PartialOrd + Zero> fmt::Display for NotNonPositive<'_, Input> {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        #![expect(
            clippy::use_debug,
            reason = "Intentional and informative, not just forgotten print-debugging"
        )]

        let Self(z) = *self;
        write!(f, "{z:#?} > {:#?}", Input::ZERO)
    }
}
