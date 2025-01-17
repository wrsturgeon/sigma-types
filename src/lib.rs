//! Types that maintain a given invariant.

#![cfg_attr(not(feature = "std"), no_std)]

mod all;
mod all_pairs;

#[cfg(all(not(feature = "std"), any(test, feature = "quickcheck")))]
extern crate alloc;

mod finite;
mod invariant;
mod less_than;
mod negative;
mod non_negative;
mod non_positive;
mod non_zero;
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
    finite::{CanBeInfinite, Finite, FiniteInvariant, NotFinite},
    invariant::Test,
    less_than::{i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize},
    negative::{Negative, NegativeInvariant, NotNegative},
    non_negative::{NonNegative, NonNegativeInvariant, NotNonNegative},
    non_positive::{NonPositive, NonPositiveInvariant, NotNonPositive},
    non_zero::{NonZero, NonZeroInvariant, NotNonZero},
    on_unit::{NotOnUnit, OnUnit, OnUnitInvariant},
    positive::{NotPositive, Positive, PositiveInvariant},
    sigma::Sigma,
    sorted::{OutOfOrder, Sorted, SortedInvariant, SortedPair},
};

#[cfg(feature = "malachite")]
pub use malachite_base::num::basic::traits::{One, Zero};

#[cfg(not(feature = "malachite"))]
pub use {one::One, zero::Zero};
