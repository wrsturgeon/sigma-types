//! Function-like type that checks an invariant and optionally provides an error message.

use core::fmt;

/// Function-like type that checks an invariant and optionally provides an error message.
///
/// Takes an arity parameter useful e.g. for comparing pairs, since
/// you want to take references for each element of the pair,
/// but the pair *itself* has to be a temporary value.
pub trait Test<Input, const ARITY: usize = 1> {
    /// Adjective to describe this test:
    /// for example, if we're testing A,
    /// then this is B in "A is not B."
    const ADJECTIVE: &str;

    /// An error implementing `::core::fmt::Display`.
    /// If no error is ever provided, please use `::core::convert::Infallible`.
    type Error<'i>: fmt::Display
    where
        Input: 'i;

    /// Check whether a given term satisfies this invariant.
    /// # Errors
    /// If it doesn't.
    fn test(input: [&Input; ARITY]) -> Result<(), Self::Error<'_>>;
}
