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
                writeln!(f, "    current element {current:#?}")?;
                writeln!(f, "       last element {last:#?}")
            }
            Self::NoDefinedComparison {
                ref current,
                ref last,
            } => {
                writeln!(f, "No defined comparison:")?;
                writeln!(f, "    current element {current:#?}")?;
                writeln!(f, "       last element {last:#?}")
            }
            Self::Swapped {
                ref current,
                ref last,
            } => {
                writeln!(f, "Out of order:")?;
                writeln!(f, "    current element {current:#?}")?;
                writeln!(f, "       last element {last:#?}")
            }
        }
    }
}

/// Iterable data structure guaranteed to be sorted (optionally with or without duplicates).
pub type Sorted<Iter, const ALLOW_DUPLICATES: bool> =
    crate::Sigma<Iter, SortedInvariant<Iter, ALLOW_DUPLICATES>>;

/*
impl<'iter, Iter: fmt::Debug, const ALLOW_DUPLICATES: bool> IntoIterator
    for &'iter Sorted<Iter, ALLOW_DUPLICATES>
where
    for<'i> &'i Iter: IntoIterator,
    for<'i> <&'i Iter as IntoIterator>::Item: fmt::Debug + PartialOrd,
{
    type Item = <&'iter Iter as IntoIterator>::Item;
    type IntoIter = <&'iter Iter as IntoIterator>::IntoIter;

    #[inline(always)]
    fn into_iter(self) -> Self::IntoIter {
        let iter: &'iter Iter = self.get_ref();
        <&'iter Iter as IntoIterator>::into_iter(iter)
    }
}
*/

impl<Iter: IntoIterator + fmt::Debug, const ALLOW_DUPLICATES: bool> IntoIterator
    for Sorted<Iter, ALLOW_DUPLICATES>
where
    for<'i> &'i Iter: IntoIterator,
    for<'i> <&'i Iter as IntoIterator>::Item: fmt::Debug + PartialOrd,
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
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct SortedInvariant<Iter: fmt::Debug, const ALLOW_DUPLICATES: bool>(PhantomData<Iter>)
where
    for<'i> &'i Iter: IntoIterator,
    for<'i> <&'i Iter as IntoIterator>::Item: fmt::Debug + PartialOrd;

impl<Iter: fmt::Debug, const ALLOW_DUPLICATES: bool> Default
    for SortedInvariant<Iter, ALLOW_DUPLICATES>
where
    for<'i> &'i Iter: IntoIterator,
    for<'i> <&'i Iter as IntoIterator>::Item: fmt::Debug + PartialOrd,
{
    #[inline(always)]
    fn default() -> Self {
        Self(PhantomData)
    }
}

impl<Iter: fmt::Debug, const ALLOW_DUPLICATES: bool> crate::Test<Iter>
    for SortedInvariant<Iter, ALLOW_DUPLICATES>
where
    for<'i> &'i Iter: IntoIterator,
    for<'i> <&'i Iter as IntoIterator>::Item: fmt::Debug + PartialOrd,
{
    const ADJECTIVE: &'static str = "sorted";

    type Error<'input>
        = OutOfOrder<<&'input Iter as IntoIterator>::Item>
    where
        Iter: 'input;

    #[inline]
    fn test(input: &Iter) -> Result<(), Option<Self::Error<'_>>> {
        let mut iter = input.into_iter();
        if let Some(mut last) = iter.next() {
            for current in iter {
                match last.partial_cmp(&current) {
                    None => return Err(Some(OutOfOrder::NoDefinedComparison { current, last })),
                    Some(Ordering::Less) => {}
                    Some(Ordering::Equal) => {
                        if !ALLOW_DUPLICATES {
                            return Err(Some(OutOfOrder::Duplicate { current, last }));
                        }
                    }
                    Some(Ordering::Greater) => {
                        return Err(Some(OutOfOrder::Swapped { current, last }));
                    }
                }
                last = current;
            }
        }
        Ok(())
    }
}
