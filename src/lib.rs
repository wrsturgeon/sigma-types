//! Types that maintain a given invariant.

#![cfg_attr(not(feature = "std"), no_std)]

mod all_pairs;
mod invariant;
mod non_negative;
mod on_unit;

#[cfg(not(feature = "malachite"))]
mod one;

mod positive;
mod sigma;
mod sorted;

#[cfg(test)]
mod test;

#[cfg(not(feature = "malachite"))]
mod zero;

pub use {
    all_pairs::{AllPairs, NotAllPairs},
    invariant::Test,
    non_negative::{Negative, NonNegative, NonNegativeInvariant},
    on_unit::{NotOnUnit, OnUnit, OnUnitInvariant},
    positive::{NonPositive, Positive, PositiveInvariant},
    sigma::Sigma,
    sorted::{OutOfOrder, Sorted, SortedInvariant, SortedPair},
};

#[cfg(feature = "malachite")]
pub use malachite_base::num::basic::traits::{One, Zero};

#[cfg(not(feature = "malachite"))]
pub use {one::One, zero::Zero};
