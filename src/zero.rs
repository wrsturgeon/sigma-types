//! Types equipped with an additive identity (i.e., zero).

/// Implementations using the `malachite_base` crate.
#[cfg(feature = "malachite")]
mod malachite {
    use {super::Zero, malachite_base::num::basic::traits};

    impl<Z: traits::Zero> Zero for Z {
        const ZERO: Self = <Z as traits::Zero>::ZERO;
    }
}

/// Implementations _not_ using the `malachite_base` crate.
#[cfg(not(feature = "malachite"))]
mod non_malachite {
    use super::Zero;

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
}

/// Types equipped with an additive identity (i.e., zero).
pub trait Zero {
    /// Additive identity (i.e., zero).
    const ZERO: Self;
}
