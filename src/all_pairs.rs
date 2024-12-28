//! Iterable data structure in which each adjacent pair of elements satisfies a given invariant.

use core::{fmt, marker::PhantomData};

/// Iterable data structure in which each adjacent pair of elements satisfies a given invariant.
#[derive(Copy, Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct AllPairs<Invariant: crate::Test<Input::Item, 2>, Input: IntoIterator + fmt::Debug>(
    PhantomData<Invariant>,
    PhantomData<Input>,
)
where
    Input::Item: fmt::Debug,
    for<'i> &'i Input: IntoIterator<Item = &'i Input::Item>;

impl<Invariant: crate::Test<Input::Item, 2>, Input: IntoIterator + fmt::Debug> crate::Test<Input>
    for AllPairs<Invariant, Input>
where
    Input::Item: fmt::Debug,
    for<'i> &'i Input: IntoIterator<Item = &'i Input::Item>,
{
    const ADJECTIVE: &str = "all pairwise valid";

    type Error<'i>
        = NotAllPairs<'i, Input::Item, Invariant>
    where
        Input: 'i;

    #[inline]
    fn test([input]: [&Input; 1]) -> Result<(), Self::Error<'_>> {
        let mut iter = input.into_iter();

        // Since Rust can't reassign references,
        // get a pointer for each item and reassign that instead:
        let mut last_ptr: *const _ = {
            let Some(last_ref) = iter.next() else {
                return Ok(());
            };
            last_ref
        };
        for (index_of_fst, current) in iter.enumerate() {
            // SAFETY:
            // Just dereferencing a known reference
            // whose referencee may be changed.
            let last = unsafe { &*last_ptr };
            match Invariant::test([last, current]) {
                Ok(()) => {}
                Err(error) => {
                    return Err(NotAllPairs {
                        elem_fst: last,
                        elem_snd: current,
                        error,
                        index_of_fst,
                    });
                }
            }
            last_ptr = current;
        }
        Ok(())
    }
}

/// At least one pair in an iterator did not satisfy the given invariant.
#[non_exhaustive]
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct NotAllPairs<'i, Item: fmt::Debug, Invariant: crate::Test<Item, 2>> {
    /// First element of the pair.
    elem_fst: &'i Item,
    /// Second element of the pair.
    elem_snd: &'i Item,
    /// Error indicating why this pair wasn't valid.
    error: Invariant::Error<'i>,
    /// After how many other elements
    /// did we see the first element of this pair?
    index_of_fst: usize,
}

impl<Item: fmt::Debug, Invariant: crate::Test<Item, 2>> fmt::Display
    for NotAllPairs<'_, Item, Invariant>
{
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        #![expect(
            clippy::use_debug,
            reason = "Intentional and informative, not just forgotten print-debugging"
        )]

        let Self {
            elem_fst,
            elem_snd,
            ref error,
            index_of_fst,
        } = *self;
        write!(f, "Elements #{index_of_fst} and #")?;
        if let Some(index_of_snd) = index_of_fst.checked_add(1) {
            fmt::Display::fmt(&index_of_snd, f)
        } else {
            write!(f, "[`usize` overflow]")
        }?;
        write!(
            f,
            " ({elem_fst:#?} and {elem_snd:#?}) were not {}: {error}",
            Invariant::ADJECTIVE,
        )
    }
}
