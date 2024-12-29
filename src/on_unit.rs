//! Types on the unit interval (between 0 and 1),
//! either inclusive or exclusive at each extreme.

use {
    crate::{One, Sigma, Test, Zero},
    core::{cmp::Ordering, fmt, marker::PhantomData},
};

/// Term expected to be on the unit interval (between 0 and 1) was not.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct NotOnUnit<
    'i,
    Input: One + PartialOrd + Zero + fmt::Debug,
    const INCLUSIVE_AT_ZERO: bool,
    const INCLUSIVE_AT_ONE: bool,
>(&'i Input);

impl<
    Input: One + PartialOrd + Zero + fmt::Debug,
    const INCLUSIVE_AT_ZERO: bool,
    const INCLUSIVE_AT_ONE: bool,
> fmt::Display for NotOnUnit<'_, Input, INCLUSIVE_AT_ZERO, INCLUSIVE_AT_ONE>
{
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        #![expect(
            clippy::use_debug,
            reason = "Intentional and informative, not just forgotten print-debugging"
        )]

        let Self(u) = *self;
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
pub type OnUnit<Input, const INCLUSIVE_AT_ZERO: bool, const INCLUSIVE_AT_ONE: bool> =
    Sigma<Input, OnUnitInvariant<Input, INCLUSIVE_AT_ZERO, INCLUSIVE_AT_ONE>>;

/// Terms on the unit interval (between 0 and 1),
/// either inclusive or exclusive at each extreme.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct OnUnitInvariant<
    Input: One + PartialOrd + Zero + fmt::Debug,
    const INCLUSIVE_AT_ZERO: bool,
    const INCLUSIVE_AT_ONE: bool,
>(PhantomData<Input>);

impl<
    Input: One + PartialOrd + Zero + fmt::Debug,
    const INCLUSIVE_AT_ZERO: bool,
    const INCLUSIVE_AT_ONE: bool,
> Test<Input, 1> for OnUnitInvariant<Input, INCLUSIVE_AT_ZERO, INCLUSIVE_AT_ONE>
{
    const ADJECTIVE: &str = "on the unit interval";
    type Error<'i>
        = NotOnUnit<'i, Input, INCLUSIVE_AT_ZERO, INCLUSIVE_AT_ONE>
    where
        Input: 'i;

    #[inline]
    fn test([input]: [&Input; 1]) -> Result<(), Self::Error<'_>> {
        match input.partial_cmp(&Input::ZERO) {
            None | Some(Ordering::Less) => return Err(NotOnUnit(input)),
            Some(Ordering::Equal) => {
                if !INCLUSIVE_AT_ZERO {
                    return Err(NotOnUnit(input));
                }
            }
            Some(Ordering::Greater) => {}
        }
        match input.partial_cmp(&Input::ONE) {
            None | Some(Ordering::Greater) => return Err(NotOnUnit(input)),
            Some(Ordering::Equal) => {
                if !INCLUSIVE_AT_ONE {
                    return Err(NotOnUnit(input));
                }
            }
            Some(Ordering::Less) => {}
        }
        Ok(())
    }
}
