//! Non-negative types (defined by comparison to zero).

use {
    crate::{Sigma, Test, Zero},
    core::{fmt, marker::PhantomData},
};

/// A term expected to be non-negative was, in fact, negative.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Negative<'i, Input: fmt::Debug + PartialOrd + Zero>(&'i Input);

impl<Input: fmt::Debug + PartialOrd + Zero> fmt::Display for Negative<'_, Input> {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        #![expect(
            clippy::use_debug,
            reason = "Intentional and informative, not just forgotten print-debugging"
        )]

        let Self(z) = *self;
        write!(f, "{z:#?} < {:#?}", Input::ZERO)
    }
}

/// Non-negative terms (defined by comparison to zero).
pub type NonNegative<Input> = Sigma<Input, NonNegativeInvariant<Input>>;

/// Non-negative terms (defined by comparison to zero).
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct NonNegativeInvariant<Input: fmt::Debug + PartialOrd + Zero>(PhantomData<Input>);

impl<Input: fmt::Debug + PartialOrd + Zero> Test<Input, 1> for NonNegativeInvariant<Input> {
    const ADJECTIVE: &str = "non-negative";
    type Error<'i>
        = Negative<'i, Input>
    where
        Input: 'i;

    #[inline(always)]
    fn test([input]: [&Input; 1]) -> Result<(), Self::Error<'_>> {
        if *input >= Input::ZERO {
            Ok(())
        } else {
            Err(Negative(input))
        }
    }
}
