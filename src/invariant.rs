//! Function-like type that checks an invariant and optionally provides an error message.

use core::fmt;

/// Function-like type that checks an invariant and optionally provides an error message.
pub trait Test<Input: fmt::Debug> {
    /// Adjective to describe this test:
    /// for example, if we're testing A,
    /// then this is B in "A is not B."
    const ADJECTIVE: &str;

    /// An error implementing `::core::fmt::Display`.
    /// If none is ever provided, please use `::core::convert::Infallible`.
    type Error: fmt::Display;

    /// Check whether a given term satisfies this invariant.
    /// # Errors
    /// If it doesn't.
    fn test(input: &Input) -> Result<(), Self::Error>;
}
