//! Types that maintain a given invariant.

#![no_std]

mod invariant;
mod sigma;
mod sorted;

#[cfg(test)]
mod test;

pub use {
    invariant::Test,
    sigma::Sigma,
    sorted::{OutOfOrder, Sorted, SortedInvariant},
};
