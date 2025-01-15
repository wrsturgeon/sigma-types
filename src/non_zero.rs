//! Nonzero terms (defined by comparison to zero).

use {
    crate::{Sigma, Test, Zero},
    core::{fmt, marker::PhantomData},
};

/// Nonzero terms (defined by comparison to zero).
pub type NonZero<Input> = Sigma<Input, NonZeroInvariant<Input>>;

/// Nonzero terms (defined by comparison to zero).
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct NonZeroInvariant<Input: fmt::Debug + PartialEq + Zero>(PhantomData<Input>);

impl<Input: fmt::Debug + PartialEq + Zero> Test<Input, 1> for NonZeroInvariant<Input> {
    const ADJECTIVE: &str = "nonzero";
    type Error<'i>
        = NotNonZero<'i, Input>
    where
        Input: 'i;

    #[inline(always)]
    fn test([input]: [&Input; 1]) -> Result<(), Self::Error<'_>> {
        if *input == Input::ZERO {
            Err(NotNonZero(input))
        } else {
            Ok(())
        }
    }
}

/// A term expected to be nonzero was, in fact, zero.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct NotNonZero<'i, Input: fmt::Debug + PartialEq + Zero>(&'i Input);

impl<Input: fmt::Debug + PartialEq + Zero> fmt::Display for NotNonZero<'_, Input> {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        #![expect(
            clippy::use_debug,
            reason = "Intentional and informative, not just forgotten print-debugging"
        )]

        let Self(z) = *self;
        write!(f, "{z:#?} == {:#?}", Input::ZERO)
    }
}
