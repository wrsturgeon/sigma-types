//! Iterable data structure guaranteed to be sorted (optionally with or without duplicates).

use core::{cmp::Ordering, fmt, marker::PhantomData};

/// Some elements in a supposedly sorted iterator were not sorted.
#[expect(
    clippy::exhaustive_enums,
    reason = "Partial comparison is, in fact, an exhaustive relation"
)]
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum OutOfOrder<T: fmt::Debug> {
    /// Two adjacent elements compared as equal (iff this was explicitly disallowed).
    Duplicate {
        /// Element that compared equal to the one before it.
        current: T,
        /// Element that compared equal to the one after it.
        last: T,
    },
    /// Two adjacent elements could not be compared.
    NoDefinedComparison {
        /// Element that did not compared to the one before it.
        current: T,
        /// Element that did not compared to the one after it.
        last: T,
    },
    /// Two adjacent elements compared in decreasing order.
    Swapped {
        /// Element that compared less than the one before it.
        current: T,
        /// Element that compared greater than the one after it.
        last: T,
    },
}

impl<T: fmt::Debug> fmt::Display for OutOfOrder<T> {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        #[expect(
            clippy::use_debug,
            reason = "Intentional and informative, not just forgotten print-debugging"
        )]
        match *self {
            Self::Duplicate {
                ref current,
                ref last,
            } => {
                writeln!(
                    f,
                    "Duplicate element (not allowed since `ALLOW_DUPLICATES = false`):",
                )?;
                writeln!(f)?;
                writeln!(f, "current element:")?;
                writeln!(f, "{current:#?}")?;
                writeln!(f)?;
                writeln!(f, "last element:")?;
                writeln!(f, "{last:#?}")
            }
            Self::NoDefinedComparison {
                ref current,
                ref last,
            } => {
                writeln!(f, "No defined comparison:")?;
                writeln!(f)?;
                writeln!(f, "current element:")?;
                writeln!(f, "{current:#?}")?;
                writeln!(f)?;
                writeln!(f, "last element:")?;
                writeln!(f, "{last:#?}")
            }
            Self::Swapped {
                ref current,
                ref last,
            } => {
                writeln!(f, "Out of order:")?;
                writeln!(f)?;
                writeln!(f, "current element:")?;
                writeln!(f, "{current:#?}")?;
                writeln!(f)?;
                writeln!(f, "last element:")?;
                writeln!(f, "{last:#?}")
            }
        }
    }
}

/// Iterable data structure guaranteed to be sorted (optionally with or without duplicates).
pub type Sorted<Iter, const ALLOW_DUPLICATES: bool> =
    crate::Sigma<Iter, SortedInvariant<Iter, ALLOW_DUPLICATES>>;

impl<
    Iter: IntoIterator + fmt::Debug,
    const ALLOW_DUPLICATES: bool,
    Item: Clone + PartialOrd + fmt::Debug,
> IntoIterator for Sorted<Iter, ALLOW_DUPLICATES>
where
    for<'i> &'i Iter: IntoIterator<Item = &'i Item>,
{
    type IntoIter = <Iter as IntoIterator>::IntoIter;
    type Item = <Iter as IntoIterator>::Item;

    #[inline(always)]
    fn into_iter(self) -> Self::IntoIter {
        let iter: Iter = self.get();
        <Iter as IntoIterator>::into_iter(iter)
    }
}

/// Ensure that an iterable data structure is sorted (optionally with or without duplicates).
#[derive(Copy, Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct SortedInvariant<Iter: fmt::Debug, const ALLOW_DUPLICATES: bool>(PhantomData<Iter>)
where
    for<'i> &'i Iter: IntoIterator,
    for<'i> <&'i Iter as IntoIterator>::Item: fmt::Debug + PartialOrd;

impl<Iter: fmt::Debug, const ALLOW_DUPLICATES: bool, Item: Clone + PartialOrd + fmt::Debug>
    crate::Test<Iter> for SortedInvariant<Iter, ALLOW_DUPLICATES>
where
    for<'i> &'i Iter: IntoIterator<Item = &'i Item>,
{
    const ADJECTIVE: &str = "sorted";

    type Error = OutOfOrder<Item>;

    #[inline]
    fn test(input: &Iter) -> Result<(), Self::Error> {
        let mut iter = input.into_iter();
        if let Some(last_ref) = iter.next() {
            let mut last = last_ref.clone();
            for current_ref in iter {
                let current = current_ref.clone();
                match last.partial_cmp(&current) {
                    None => return Err(OutOfOrder::NoDefinedComparison { current, last }),
                    Some(Ordering::Less) => {}
                    Some(Ordering::Equal) => {
                        if !ALLOW_DUPLICATES {
                            return Err(OutOfOrder::Duplicate { current, last });
                        }
                    }
                    Some(Ordering::Greater) => {
                        return Err(OutOfOrder::Swapped { current, last });
                    }
                }
                last = current;
            }
        }
        Ok(())
    }
}
