//! Types that maintain a given invariant.

#![cfg_attr(not(feature = "std"), no_std)]

mod all;
mod all_pairs;

#[cfg(all(not(feature = "std"), feature = "quickcheck"))]
extern crate alloc;

mod invariant;
mod negative;
mod non_negative;
mod non_positive;
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
    all::{All, NotAll},
    all_pairs::{AllPairs, NotAllPairs},
    invariant::Test,
    negative::{Negative, NegativeInvariant, NotNegative},
    non_negative::{NonNegative, NonNegativeInvariant, NotNonNegative},
    non_positive::{NonPositive, NonPositiveInvariant, NotNonPositive},
    on_unit::{NotOnUnit, OnUnit, OnUnitInvariant},
    positive::{NotPositive, Positive, PositiveInvariant},
    sigma::Sigma,
    sorted::{OutOfOrder, Sorted, SortedInvariant, SortedPair},
};

#[cfg(feature = "malachite")]
pub use malachite_base::num::basic::traits::{One, Zero};

#[cfg(not(feature = "malachite"))]
pub use {one::One, zero::Zero};
