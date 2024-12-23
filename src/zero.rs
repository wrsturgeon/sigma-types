//! Types equipped with an additive identity (i.e., zero).

impl Zero for u8 {
    const ZERO: Self = 0;
}

impl Zero for i8 {
    const ZERO: Self = 0;
}

impl Zero for u16 {
    const ZERO: Self = 0;
}

impl Zero for i16 {
    const ZERO: Self = 0;
}

impl Zero for u32 {
    const ZERO: Self = 0;
}

impl Zero for i32 {
    const ZERO: Self = 0;
}

impl Zero for u64 {
    const ZERO: Self = 0;
}

impl Zero for i64 {
    const ZERO: Self = 0;
}

impl Zero for u128 {
    const ZERO: Self = 0;
}

impl Zero for i128 {
    const ZERO: Self = 0;
}

impl Zero for usize {
    const ZERO: Self = 0;
}

impl Zero for isize {
    const ZERO: Self = 0;
}

/// Types equipped with an additive identity (i.e., zero).
pub trait Zero {
    /// Additive identity (i.e., zero).
    const ZERO: Self;
}
