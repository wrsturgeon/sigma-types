//! Finite types (e.g. for floating points, not `NaN`, infinite, etc.).

use {
    crate::{Sigma, Test},
    core::{fmt, marker::PhantomData},
};

/// Types that can represent infinite values.
pub trait CanBeInfinite {
    /// Check that this value is finite (i.e. not infinite, `NaN`, etc).
    fn check_finite(&self) -> bool;
}

/// Finite terms (e.g. for floating points, not `NaN`, infinite, etc.).
pub type Finite<Input> = Sigma<Input, FiniteInvariant<Input>>;

/// Finite terms (e.g. for floating points, not `NaN`, infinite, etc.).
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct FiniteInvariant<Input: fmt::Debug + CanBeInfinite>(PhantomData<Input>);

impl<Input: fmt::Debug + CanBeInfinite> Test<Input, 1> for FiniteInvariant<Input> {
    const ADJECTIVE: &str = "finite";
    type Error<'i>
        = NotFinite
    where
        Input: 'i;

    #[inline(always)]
    fn test([input]: [&Input; 1]) -> Result<(), Self::Error<'_>> {
        if input.check_finite() {
            Ok(())
        } else {
            Err(NotFinite)
        }
    }
}

/// A term expected to be finite was not.
#[non_exhaustive]
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct NotFinite;

impl fmt::Display for NotFinite {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "`CanBeInfinite::check_finite(..)` returned `false`")
    }
}

impl CanBeInfinite for f32 {
    #[inline(always)]
    fn check_finite(&self) -> bool {
        self.is_finite()
    }
}

impl CanBeInfinite for f64 {
    #[inline(always)]
    fn check_finite(&self) -> bool {
        self.is_finite()
    }
}
