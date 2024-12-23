//! Types that maintain a given invariant.

#![no_std]

mod invariant;
mod non_negative;
mod positive;
mod sigma;
mod sorted;

#[cfg(test)]
mod test;

mod zero;

pub use {
    invariant::Test,
    non_negative::{Negative, NonNegative, NonNegativeInvariant},
    positive::{NonPositive, Positive, PositiveInvariant},
    sigma::Sigma,
    sorted::{OutOfOrder, Sorted, SortedInvariant},
    zero::Zero,
};
