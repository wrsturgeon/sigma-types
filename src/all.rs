//! Iterable data structure in which each element satisfies a given invariant.

use core::{fmt, marker::PhantomData};

/// Iterable data structure in which each element satisfies a given invariant.
#[derive(Copy, Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct All<Invariant: crate::Test<Input::Item, 1>, Input: IntoIterator + fmt::Debug>(
    PhantomData<Invariant>,
    PhantomData<Input>,
)
where
    Input::Item: fmt::Debug,
    for<'i> &'i Input: IntoIterator<Item = &'i Input::Item>;

impl<Invariant: crate::Test<Input::Item, 1>, Input: IntoIterator + fmt::Debug> crate::Test<Input, 1>
    for All<Invariant, Input>
where
    Input::Item: fmt::Debug,
    for<'i> &'i Input: IntoIterator<Item = &'i Input::Item>,
{
    const ADJECTIVE: &str = "all valid";

    type Error<'i>
        = NotAll<'i, Input::Item, Invariant>
    where
        Input: 'i;

    #[inline]
    fn test([input]: [&Input; 1]) -> Result<(), Self::Error<'_>> {
        for (index, element) in input.into_iter().enumerate() {
            match Invariant::test([element]) {
                Ok(()) => {}
                Err(error) => {
                    return Err(NotAll {
                        element,
                        error,
                        index,
                    });
                }
            }
        }
        Ok(())
    }
}

/// At least one element in an iterator did not satisfy the given invariant.
#[non_exhaustive]
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct NotAll<'i, Item: fmt::Debug, Invariant: crate::Test<Item, 1>> {
    /// Invalid element in the iterator.
    element: &'i Item,
    /// Error indicating why this element wasn't valid.
    error: Invariant::Error<'i>,
    /// After how many other elements
    /// did we see the this element?
    index: usize,
}

impl<Item: fmt::Debug, Invariant: crate::Test<Item, 1>> fmt::Display
    for NotAll<'_, Item, Invariant>
{
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        #![expect(
            clippy::use_debug,
            reason = "Intentional and informative, not just forgotten print-debugging"
        )]

        let Self {
            element,
            ref error,
            index,
        } = *self;
        write!(
            f,
            "Element #{index} ({element:#?}) was not {}: {error}",
            Invariant::ADJECTIVE,
        )
    }
}
