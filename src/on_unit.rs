//! Types on the unit interval (between 0 and 1),
//! either inclusive or exclusive at each extreme.

use {
    crate::{One, Sigma, Test, Zero},
    core::{cmp::Ordering, fmt, marker::PhantomData},
};

/// Term expected to be on the unit interval (between 0 and 1) was not.
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct NotOnUnit<
    U: Clone + One + PartialOrd + Zero + fmt::Debug,
    const INCLUSIVE_AT_ZERO: bool,
    const INCLUSIVE_AT_ONE: bool,
>(U);

impl<
    U: Clone + One + PartialOrd + Zero + fmt::Debug,
    const INCLUSIVE_AT_ZERO: bool,
    const INCLUSIVE_AT_ONE: bool,
> fmt::Display for NotOnUnit<U, INCLUSIVE_AT_ZERO, INCLUSIVE_AT_ONE>
{
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        #![expect(
            clippy::use_debug,
            reason = "Intentional and informative, not just forgotten print-debugging"
        )]

        let Self(ref u) = *self;
        write!(
            f,
            "Not on {}0, 1{}: {u:#?}",
            if INCLUSIVE_AT_ZERO { '[' } else { '(' },
            if INCLUSIVE_AT_ONE { ']' } else { ')' },
        )
    }
}

/// Terms on the unit interval (between 0 and 1),
/// either inclusive or exclusive at each extreme.
pub type OnUnit<U, const INCLUSIVE_AT_ZERO: bool, const INCLUSIVE_AT_ONE: bool> =
    Sigma<U, OnUnitInvariant<U, INCLUSIVE_AT_ZERO, INCLUSIVE_AT_ONE>>;

/// Terms on the unit interval (between 0 and 1),
/// either inclusive or exclusive at each extreme.
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct OnUnitInvariant<
    U: Clone + One + PartialOrd + Zero + fmt::Debug,
    const INCLUSIVE_AT_ZERO: bool,
    const INCLUSIVE_AT_ONE: bool,
>(PhantomData<U>);

impl<
    U: Clone + One + PartialOrd + Zero + fmt::Debug,
    const INCLUSIVE_AT_ZERO: bool,
    const INCLUSIVE_AT_ONE: bool,
> Default for OnUnitInvariant<U, INCLUSIVE_AT_ZERO, INCLUSIVE_AT_ONE>
{
    #[inline(always)]
    fn default() -> Self {
        Self(PhantomData)
    }
}

impl<
    U: Clone + One + PartialOrd + Zero + fmt::Debug,
    const INCLUSIVE_AT_ZERO: bool,
    const INCLUSIVE_AT_ONE: bool,
> Test<U> for OnUnitInvariant<U, INCLUSIVE_AT_ZERO, INCLUSIVE_AT_ONE>
{
    const ADJECTIVE: &str = "on the unit interval";
    type Error = NotOnUnit<U, INCLUSIVE_AT_ZERO, INCLUSIVE_AT_ONE>;

    #[inline]
    fn test(input: &U) -> Result<(), Self::Error> {
        match input.partial_cmp(&U::ZERO) {
            None | Some(Ordering::Less) => return Err(NotOnUnit(input.clone())),
            Some(Ordering::Equal) => {
                if !INCLUSIVE_AT_ZERO {
                    return Err(NotOnUnit(input.clone()));
                }
            }
            Some(Ordering::Greater) => {}
        }
        match input.partial_cmp(&U::ONE) {
            None | Some(Ordering::Greater) => return Err(NotOnUnit(input.clone())),
            Some(Ordering::Equal) => {
                if !INCLUSIVE_AT_ONE {
                    return Err(NotOnUnit(input.clone()));
                }
            }
            Some(Ordering::Less) => {}
        }
        Ok(())
    }
}
