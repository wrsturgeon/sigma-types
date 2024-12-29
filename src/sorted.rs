//! Iterable data structure guaranteed to be sorted (optionally with or without duplicates).

use {
    crate::AllPairs,
    core::{cmp::Ordering, fmt},
};

/// Some elements in a supposedly sorted iterator were not sorted.
#[expect(
    clippy::exhaustive_enums,
    reason = "Partial comparison is, in fact, an exhaustive relation"
)]
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum OutOfOrder {
    /// Two adjacent elements compared as equal (iff this was explicitly disallowed).
    Duplicate,
    /// Two adjacent elements could not be compared.
    NoDefinedComparison,
    /// Two adjacent elements compared in decreasing order.
    Reversed,
}

impl fmt::Display for OutOfOrder {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Self::Duplicate => {
                writeln!(
                    f,
                    "duplicate element (not allowed since `ALLOW_DUPLICATES = false`)",
                )
            }
            Self::NoDefinedComparison => writeln!(f, "no defined comparison"),
            Self::Reversed => writeln!(f, "reversed"),
        }
    }
}

/// Iterable data structure guaranteed to be sorted (optionally with or without duplicates).
pub type Sorted<Input, const ALLOW_DUPLICATES: bool> =
    crate::Sigma<Input, SortedInvariant<Input, ALLOW_DUPLICATES>>;

impl<Input: IntoIterator + fmt::Debug, const ALLOW_DUPLICATES: bool> IntoIterator
    for Sorted<Input, ALLOW_DUPLICATES>
where
    Input::Item: PartialOrd + fmt::Debug,
    for<'i> &'i Input: IntoIterator<Item = &'i Input::Item>,
{
    type IntoIter = <Input as IntoIterator>::IntoIter;
    type Item = <Input as IntoIterator>::Item;

    #[inline(always)]
    fn into_iter(self) -> Self::IntoIter {
        let iter: Input = self.get();
        <Input as IntoIterator>::into_iter(iter)
    }
}

/// Iterable data structure guaranteed to be sorted (optionally with or without duplicates).
pub type SortedInvariant<Input, const ALLOW_DUPLICATES: bool> =
    AllPairs<SortedPair<ALLOW_DUPLICATES>, Input>;

/// Pair guaranteed to be sorted left-to-right (optionally permitted to be equal).
#[expect(clippy::exhaustive_structs, reason = "are you fucking kidding me")]
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct SortedPair<const ALLOW_DUPLICATES: bool>;

impl<const ALLOW_DUPLICATES: bool, Input: PartialOrd> crate::Test<Input, 2>
    for SortedPair<ALLOW_DUPLICATES>
{
    const ADJECTIVE: &str = "sorted";

    type Error<'i>
        = OutOfOrder
    where
        Input: 'i;

    #[inline]
    fn test([fst, snd]: [&Input; 2]) -> Result<(), Self::Error<'_>> {
        match fst.partial_cmp(snd) {
            None => Err(OutOfOrder::NoDefinedComparison),
            Some(Ordering::Less) => Ok(()),
            Some(Ordering::Equal) => {
                if ALLOW_DUPLICATES {
                    Ok(())
                } else {
                    Err(OutOfOrder::Duplicate)
                }
            }
            Some(Ordering::Greater) => Err(OutOfOrder::Reversed),
        }
    }
}
