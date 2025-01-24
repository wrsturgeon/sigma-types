//! Type that maintains a given invariant.

#![expect(
    clippy::arbitrary_source_item_ordering,
    reason = "macros must be defined before they're used"
)]

use {
    crate::{
        CanBeInfinite, Finite, Negative, NonNegative, NonPositive, NonZero, OnUnit, One, Positive,
        Zero,
    },
    core::{
        borrow::Borrow,
        cmp::Ordering,
        fmt,
        hash::{Hash, Hasher},
        marker::PhantomData,
        ops,
    },
};

#[cfg(test)]
use {paste::paste, quickcheck::TestResult, quickcheck_macros::quickcheck};

#[cfg(feature = "std")]
use std::env;

#[cfg(any(test, feature = "quickcheck"))]
use quickcheck::{Arbitrary, Gen};

#[cfg(all(any(test, feature = "quickcheck"), not(feature = "std")))]
use alloc::boxed::Box;

/// Implement a unary operation.
macro_rules! impl_op_1 {
    ($op:ident, $fn:ident, $lhs:ident, $out:ident $(, $($bound:ident),* $(,)?)?) => {
        impl<
            L: $($($bound +)*)? fmt::Debug + ops::$op<Output: $($($bound +)*)? fmt::Debug>,
        > ops::$op for $lhs<L>
        {
            type Output = $out<<L as ops::$op>::Output>;

            #[inline]
            fn $fn(self) -> Self::Output {
                self.map(|lhs| lhs.$fn())
            }
        }

        #[cfg(test)]
        paste! {
            #[cfg(test)]
            #[expect(trivial_casts, clippy::as_conversions, reason = "must be an implementation detail of `quickcheck`")]
            mod [< $lhs:snake _ $fn >] {
                use super::*;

                #[quickcheck]
                fn doesnt_panic(lhs: $lhs</* L */ f64>) -> TestResult {
                    if lhs.abs() > 1e64_f64 { return TestResult::discard() }
                    _ = <$lhs</* L */ f64> as ops::$op>::$fn(lhs);
                    TestResult::passed()
                }
            }
        }
    };
}

/// Implement a binary operation.
macro_rules! impl_op_2 {
    ($op:ident, $fn:ident, $lhs:ident, $rhs:ident, $out:ident, $reject:expr $(, $($bound:ident),* $(,)?)?) => {
        impl<
            L: $($($bound +)*)? fmt::Debug + ops::$op<R, Output: $($($bound +)*)? fmt::Debug>,
            R: $($($bound +)*)? fmt::Debug,
        > ops::$op<$rhs<R>> for $lhs<L>
        {
            type Output = $out<<L as ops::$op<R>>::Output>;

            #[inline]
            fn $fn(self, rhs: $rhs<R>) -> Self::Output {
                self.map(|lhs| lhs.$fn(rhs.get()))
            }
        }

        #[cfg(test)]
        paste! {
            #[cfg(test)]
            #[expect(trivial_casts, clippy::as_conversions, reason = "must be an implementation detail of `quickcheck`")]
            mod [< $lhs:snake _ $fn _ $rhs:snake >] {
                use super::*;

                #[quickcheck]
                fn doesnt_panic(lhs: $lhs</* L */ f64>, rhs: $rhs</* R */ f64>) -> TestResult {
                    if lhs.abs() > 1e64_f64 { return TestResult::discard() }
                    if rhs.abs() > 1e64_f64 { return TestResult::discard() }
                    let toss: fn(&$lhs</* L */ f64>, &$rhs</* R */ f64>) -> bool = $reject;
                    if toss(&lhs, &rhs) { return TestResult::discard() }
                    _ = <$lhs</* L */ f64> as ops::$op<$rhs</* R */ f64>>>::$fn(lhs, rhs);
                    TestResult::passed()
                }
            }
        }

        impl<
            'rhs,
            L: $($($bound +)*)? fmt::Debug + ops::$op<&'rhs R, Output: $($($bound +)*)? fmt::Debug>,
            R: $($($bound +)*)? fmt::Debug,
        > ops::$op<&'rhs $rhs<R>> for $lhs<L>
        {
            type Output = $out<<L as ops::$op<&'rhs R>>::Output>;

            #[inline]
            fn $fn(self, rhs: &'rhs $rhs<R>) -> Self::Output {
                self.map(|lhs| lhs.$fn(rhs.get_ref()))
            }
        }

        #[cfg(test)]
        paste! {
            #[cfg(test)]
            #[expect(trivial_casts, clippy::as_conversions, reason = "must be an implementation detail of `quickcheck`")]
            mod [< $lhs:snake _ $fn _ $rhs:snake _ref >] {
                use super::*;

                #[quickcheck]
                fn doesnt_panic(lhs: $lhs</* L */ f64>, rhs: $rhs</* R */ f64>) -> TestResult {
                    if lhs.abs() > 1e64_f64 { return TestResult::discard() }
                    if rhs.abs() > 1e64_f64 { return TestResult::discard() }
                    let toss: fn(&$lhs</* L */ f64>, &$rhs</* R */ f64>) -> bool = $reject;
                    if toss(&lhs, &rhs) { return TestResult::discard() }
                    _ = <$lhs</* L */ f64> as ops::$op<&$rhs</* R */ f64>>>::$fn(lhs, &rhs);
                    TestResult::passed()
                }
            }
        }

        /*
        impl<
            'lhs,
            L: $($($bound +)*)? fmt::Debug,
            R: $($($bound +)*)? fmt::Debug,
        > ops::$op<$rhs<R>> for &'lhs $lhs<L>
        where
            &'lhs L: ops::$op<R, Output: $($($bound +)*)? fmt::Debug>,
        {
            type Output = $out<<&'lhs L as ops::$op<R>>::Output>;

            #[inline]
            fn $fn(self, rhs: $rhs<R>) -> Self::Output {
                self.map_ref(|lhs| lhs.$fn(rhs.get()))
            }
        }

        #[cfg(test)]
        #[quickcheck]
        fn [< doesnt_panic_ $lhs:snake _ $fn _ $rhs:snake >] {
            fn doesnt_panic(lhs: $lhs</* L */ f64>, rhs: $rhs</* R */ f64>) -> TestResult {
            _ = <$lhs</* L */ f64> as ops::$ops>::$fn(lhs, rhs);
            TestResult::passed()
        }

        impl<
            'lhs,
            'rhs,
            L: $($($bound +)*)? fmt::Debug,
            R: $($($bound +)*)? fmt::Debug,
        > ops::$op<&'rhs $rhs<R>> for &'lhs $lhs<L>
        where
            &'lhs L: ops::$op<&'rhs R, Output: $($($bound +)*)? fmt::Debug>,
        {
            type Output = $out<<&'lhs L as ops::$op<&'rhs R>>::Output>;

            #[inline]
            fn $fn(self, rhs: &'rhs $rhs<R>) -> Self::Output {
                self.map_ref(|lhs| lhs.$fn(rhs.get_ref()))
            }
        }

        #[cfg(test)]
        #[quickcheck]
        fn [< doesnt_panic_ $lhs:snake _ $fn _ $rhs:snake _ref >] {
            fn doesnt_panic(lhs: $lhs</* L */ f64>, rhs: $rhs</* R */ f64>) -> TestResult {
            _ = <$lhs</* L */ f64> as ops::$ops>::$fn(lhs, rhs);
            TestResult::passed()
        }
        */
    };
}

/// Implement a unary assignment operator.
macro_rules! impl_op_assign {
    ($op:ident, $fn:ident, $lhs:ident, $rhs:ident, $reject:expr $(, $($bound:ident),* $(,)?)?) => {
        impl<
            L: $($($bound +)*)? fmt::Debug + ops::$op<R>,
            R: $($($bound +)*)? fmt::Debug,
        > ops::$op<$rhs<R>> for $lhs<L>
        {
            #[inline]
            fn $fn(&mut self, rhs: $rhs<R>) {
                self.map_mut(|lhs| lhs.$fn(rhs.get()))
            }
        }

        #[cfg(test)]
        paste! {
            #[cfg(test)]
            #[expect(trivial_casts, clippy::as_conversions, reason = "must be an implementation detail of `quickcheck`")]
            mod [< $lhs:snake _ $fn _ $rhs:snake >] {
                use super::*;

                #[quickcheck]
                fn doesnt_panic(mut lhs: $lhs</* L */ f64>, rhs: $rhs</* R */ f64>) -> TestResult {
                    if lhs.abs() > 1e64_f64 { return TestResult::discard() }
                    if rhs.abs() > 1e64_f64 { return TestResult::discard() }
                    let toss: fn(&$lhs</* L */ f64>, &$rhs</* R */ f64>) -> bool = $reject;
                    if toss(&lhs, &rhs) { return TestResult::discard() }
                    _ = <$lhs</* L */ f64> as ops::$op<$rhs</* R */ f64>>>::$fn(&mut lhs, rhs);
                    TestResult::passed()
                }
            }
        }

        impl<
            'rhs,
            L: $($($bound +)*)? fmt::Debug + ops::$op<&'rhs R>,
            R: $($($bound +)*)? fmt::Debug,
        > ops::$op<&'rhs $rhs<R>> for $lhs<L>
        {
            #[inline]
            fn $fn(&mut self, rhs: &'rhs $rhs<R>) {
                self.map_mut(|lhs| lhs.$fn(rhs.get_ref()))
            }
        }

        #[cfg(test)]
        paste! {
            #[cfg(test)]
            #[expect(trivial_casts, clippy::as_conversions, reason = "must be an implementation detail of `quickcheck`")]
            mod [< $lhs:snake _ $fn _ $rhs:snake _ref >] {
                use super::*;

                #[quickcheck]
                fn doesnt_panic(mut lhs: $lhs</* L */ f64>, rhs: $rhs</* R */ f64>) -> TestResult {
                    if lhs.abs() > 1e64_f64 { return TestResult::discard() }
                    if rhs.abs() > 1e64_f64 { return TestResult::discard() }
                    let toss: fn(&$lhs</* L */ f64>, &$rhs</* R */ f64>) -> bool = $reject;
                    if toss(&lhs, &rhs) { return TestResult::discard() }
                    _ = <$lhs</* L */ f64> as ops::$op<&$rhs</* R */ f64>>>::$fn(&mut lhs, &rhs);
                    TestResult::passed()
                }
            }
        }
    };
}

/// Addition only (and -assignment).
macro_rules! impl_add {
    ($lhs:ident, $rhs:ident, $out:ident $(, $($bound:ident),* $(,)?)?) => {
        impl_op_2!(Add, add, $lhs, $rhs, $out, |_, _| false $(, $($bound,)*)?);
        impl_op_assign!(AddAssign, add_assign, $lhs, $rhs, |_, _| false $(, $($bound,)*)?);
    };
}

/// Subtraction only (and -assignment).
macro_rules! impl_sub {
    ($lhs:ident, $rhs:ident, $out:ident $(, $($bound:ident),* $(,)?)?) => {
        impl_op_2!(Sub, sub, $lhs, $rhs, $out, |_, _| false $(, $($bound,)*)?);
        impl_op_assign!(SubAssign, sub_assign, $lhs, $rhs, |_, _| false $(, $($bound,)*)?);
    };
}

/// All multiplicative traits (e.g. multiplication, division, ...).
macro_rules! impl_mul {
    ($lhs:ident, $rhs:ident, $out:ident $(, $($bound:ident),* $(,)?)?) => {
        impl_op_2!(Div, div, $lhs, $rhs, $out, |_, rhs| rhs.get() == 0_f64 $(, $($bound,)*)?);
        impl_op_2!(Mul, mul, $lhs, $rhs, $out, |_, _| false $(, $($bound,)*)?);
    };
}

/// All multiplicative traits (e.g. multiplication, division, ...).
macro_rules! impl_mul_assign {
    ($lhs:ident, $rhs:ident $(, $($bound:ident),* $(,)?)?) => {
        impl_op_assign!(DivAssign, div_assign, $lhs, $rhs, |_, rhs| rhs.get() == 0_f64 $(, $($bound,)*)?);
        impl_op_assign!(MulAssign, mul_assign, $lhs, $rhs, |_, _| false $(, $($bound,)*)?);
    };
}

impl<Z: CanBeInfinite + One + fmt::Debug> One for Finite<Z> {
    const ONE: Self = Self {
        phantom: PhantomData,
        raw: Z::ONE,
    };
}

impl<Z: CanBeInfinite + Zero + fmt::Debug> Zero for Finite<Z> {
    const ZERO: Self = Self {
        phantom: PhantomData,
        raw: Z::ZERO,
    };
}

impl_add!(Finite, Finite, Finite, CanBeInfinite);
impl_sub!(Finite, Finite, Finite, CanBeInfinite);
impl_mul!(Finite, Finite, Finite, CanBeInfinite);
impl_mul_assign!(Finite, Finite, CanBeInfinite);
impl_op_1!(Neg, neg, Finite, Finite, CanBeInfinite);

impl_add!(Negative, Negative, Negative, PartialOrd, Zero);
impl_sub!(Negative, NonNegative, Negative, PartialOrd, Zero);
impl_add!(Negative, NonPositive, Negative, PartialOrd, Zero);
impl_sub!(Negative, Positive, Negative, PartialOrd, Zero);
impl_mul!(Negative, Negative, Positive, PartialOrd, Zero);
impl_mul!(Negative, NonNegative, NonPositive, PartialOrd, Zero);
impl_mul!(Negative, NonPositive, NonNegative, PartialOrd, Zero);
impl_mul!(Negative, Positive, Negative, PartialOrd, Zero);
impl_mul_assign!(Negative, Positive, PartialOrd, Zero);
impl_op_1!(Neg, neg, Negative, Positive, PartialOrd, Zero);

impl<T: One + PartialOrd + Zero + fmt::Debug> One for NonNegative<T> {
    const ONE: Self = Self {
        phantom: PhantomData,
        raw: T::ONE,
    };
}

impl<Z: PartialOrd + Zero + fmt::Debug> Zero for NonNegative<Z> {
    const ZERO: Self = Self {
        phantom: PhantomData,
        raw: Z::ZERO,
    };
}

impl_sub!(NonNegative, Negative, Positive, PartialOrd, Zero);
impl_add!(NonNegative, NonNegative, NonNegative, PartialOrd, Zero);
impl_sub!(NonNegative, NonPositive, NonNegative, PartialOrd, Zero);
impl_add!(NonNegative, Positive, Positive, PartialOrd, Zero);
impl_mul!(NonNegative, Negative, NonPositive, PartialOrd, Zero);
impl_mul!(NonNegative, NonNegative, NonNegative, PartialOrd, Zero);
impl_mul!(NonNegative, NonPositive, NonPositive, PartialOrd, Zero);
impl_mul!(NonNegative, Positive, NonNegative, PartialOrd, Zero);
impl_mul_assign!(NonNegative, NonNegative, PartialOrd, Zero);
impl_mul_assign!(NonNegative, Positive, PartialOrd, Zero);
impl_op_1!(Neg, neg, NonNegative, NonPositive, PartialOrd, Zero);

impl_add!(NonPositive, Negative, Negative, PartialOrd, Zero);
impl_sub!(NonPositive, NonNegative, NonPositive, PartialOrd, Zero);
impl_add!(NonPositive, NonPositive, NonPositive, PartialOrd, Zero);
impl_sub!(NonPositive, Positive, Negative, PartialOrd, Zero);
impl_mul!(NonPositive, Negative, NonNegative, PartialOrd, Zero);
impl_mul!(NonPositive, NonNegative, NonPositive, PartialOrd, Zero);
impl_mul!(NonPositive, NonPositive, NonNegative, PartialOrd, Zero);
impl_mul!(NonPositive, Positive, NonPositive, PartialOrd, Zero);
impl_mul_assign!(NonPositive, NonNegative, PartialOrd, Zero);
impl_mul_assign!(NonPositive, Positive, PartialOrd, Zero);
impl_op_1!(Neg, neg, NonPositive, NonNegative, PartialOrd, Zero);

impl_mul!(NonZero, NonZero, NonZero, PartialEq, Zero);
impl_op_1!(Neg, neg, NonZero, NonZero, PartialEq, Zero);

impl<T: One + PartialOrd + Zero + fmt::Debug, const INCLUSIVE_AT_ZERO: bool> One
    for OnUnit<T, INCLUSIVE_AT_ZERO, true>
{
    const ONE: Self = Self {
        phantom: PhantomData,
        raw: T::ONE,
    };
}

impl<Z: One + PartialOrd + Zero + fmt::Debug, const INCLUSIVE_AT_ONE: bool> Zero
    for OnUnit<Z, true, INCLUSIVE_AT_ONE>
{
    const ZERO: Self = Self {
        phantom: PhantomData,
        raw: Z::ZERO,
    };
}

impl_sub!(Positive, Negative, Positive, PartialOrd, Zero);
impl_add!(Positive, NonNegative, Positive, PartialOrd, Zero);
impl_sub!(Positive, NonPositive, Positive, PartialOrd, Zero);
impl_add!(Positive, Positive, Positive, PartialOrd, Zero);
impl_mul!(Positive, Negative, Negative, PartialOrd, Zero);
impl_mul!(Positive, NonNegative, NonNegative, PartialOrd, Zero);
impl_mul!(Positive, NonPositive, NonPositive, PartialOrd, Zero);
impl_mul!(Positive, Positive, Positive, PartialOrd, Zero);
impl_mul_assign!(Positive, Positive, PartialOrd, Zero);
impl_op_1!(Neg, neg, Positive, Negative, PartialOrd, Zero);

impl<T: One + PartialOrd + Zero + fmt::Debug> One for Positive<T> {
    const ONE: Self = Self {
        phantom: PhantomData,
        raw: T::ONE,
    };
}

/// Type that maintains a given invariant.
#[repr(transparent)]
pub struct Sigma<Raw: fmt::Debug, Invariant: crate::Test<Raw, 1>> {
    /// Only to silence compiler errors.
    phantom: PhantomData<Invariant>,
    /// Internal type (to which this type will reduce in release builds).
    raw: Raw,
}

impl<Raw: fmt::Debug, Invariant: crate::Test<Raw, 1>> Sigma<Raw, Invariant> {
    /// Check all elements of an array.
    #[inline]
    pub fn all<const N: usize>(array: &[Raw; N]) -> &[Self; N] {
        let pointer: *const [Raw; N] = array;
        let cast: *const [Self; N] = pointer.cast();
        // SAFETY:
        // `repr(transparent)`
        let provisional = unsafe { &*cast };
        for element in provisional {
            element.check();
        }
        provisional
    }

    /// Without changing its internal value,
    /// view one sigma-typed value as implementing another sigma type
    /// by checking the latter invariant at runtime (iff debug assertions are enabled).
    /// # Panics
    /// If the latter invariant does not hold.
    #[inline(always)]
    pub fn also<OtherInvariant: crate::Test<Raw, 1>>(self) -> Sigma<Raw, OtherInvariant> {
        Sigma::new(self.get())
    }

    /// Without changing its internal value,
    /// view one sigma-typed value as implementing another sigma type
    /// by checking the latter invariant at runtime (iff debug assertions are enabled).
    /// # Panics
    /// If the latter invariant does not hold.
    #[inline]
    #[cfg(debug_assertions)]
    pub fn also_ref<OtherInvariant: crate::Test<Raw, 1>>(&self) -> &Sigma<Raw, OtherInvariant> {
        let ptr: *const Self = self;
        // SAFETY:
        // Pointer reinterpretation. See `repr(transparent)` above.
        // All non-zero-sized fields are identical across the cast.
        let transmuted: &Sigma<Raw, OtherInvariant> = unsafe { &*ptr.cast() };
        transmuted.check();
        transmuted
    }

    /// Without changing its internal value,
    /// view one sigma-typed value as implementing another sigma type
    /// by checking the latter invariant at runtime (iff debug assertions are enabled).
    #[inline]
    #[cfg(not(debug_assertions))]
    pub const fn also_ref<OtherInvariant: crate::Test<Raw, 1>>(
        &self,
    ) -> &Sigma<Raw, OtherInvariant> {
        let ptr: *const Self = self;
        // SAFETY:
        // Pointer reinterpretation. See `repr(transparent)` above.
        // All non-zero-sized fields are identical across the cast.
        unsafe { &*ptr.cast() }
    }

    /// Check an invariant if and only if debug assertions are enabled.
    /// # Panics
    /// If the invariant does not hold ***and*** debug assertions are enabled.
    #[inline]
    #[cfg(debug_assertions)]
    pub fn check(&self) {
        #[expect(
            clippy::panic,
            reason = "Returning a result would break API in release builds"
        )]
        match Invariant::test([&self.raw]) {
            Ok(()) => {}
            Err(message) => {
                panic!("{:#?} is not {}: {message}", self.raw, Invariant::ADJECTIVE);
            }
        }
    }

    /// Do nothing (since debug assertions are disabled).
    #[inline]
    #[cfg(not(debug_assertions))]
    pub const fn check(&self) {}

    /// Unwrap the internal value that satisfies the invariant.
    /// If you're using this to create another value that should
    /// also maintain an invariant, use `map` instead.
    #[inline(always)]
    pub fn get(self) -> Raw {
        self.raw
    }

    /// Unwrap the internal value that satisfies the invariant.
    /// If you're using this to create another value that should
    /// also maintain an invariant, use `map` instead.
    #[inline(always)]
    #[expect(clippy::allow_attributes, reason = "Edition 2021 only")]
    #[allow(tail_expr_drop_order, reason = "just for miri")]
    pub fn get_by<Y, F: FnOnce(Raw) -> Y>(self, f: F) -> Y {
        f(self.get())
    }

    /// Unwrap the internal value that satisfies the invariant.
    /// If you're using this to create another value that should
    /// also maintain an invariant, use `map` instead.
    #[inline(always)]
    pub fn get_by_mut<Y, F: FnOnce(&mut Raw) -> Y>(&mut self, f: F) -> Y {
        f(self.get_mut())
    }

    /// Unwrap the internal value that satisfies the invariant.
    /// If you're using this to create another value that should
    /// also maintain an invariant, use `map` instead.
    #[inline(always)]
    pub fn get_by_ref<Y, F: FnOnce(&Raw) -> Y>(&self, f: F) -> Y {
        f(self)
    }

    /// Unwrap the internal value that satisfies the invariant.
    /// If you're using this to create another value that should
    /// also maintain an invariant, use `map` instead.
    #[inline(always)]
    pub const fn get_mut(&mut self) -> &mut Raw {
        &mut self.raw
    }

    /// Unwrap the internal value that satisfies the invariant.
    /// If you're using this to create another value that should
    /// also maintain an invariant, use `map` instead.
    #[inline(always)]
    pub const fn get_ref(&self) -> &Raw {
        &self.raw
    }

    /// Apply a function to a term that implements a given invariant (say, A),
    /// then check the output for a (possibly different) invariant (say, B).
    #[inline]
    #[expect(clippy::allow_attributes, reason = "Edition 2021 only")]
    #[allow(tail_expr_drop_order, reason = "just for miri")]
    pub fn map<
        OtherRaw: fmt::Debug,
        OtherInvariant: crate::Test<OtherRaw, 1>,
        F: FnOnce(Raw) -> OtherRaw,
    >(
        self,
        f: F,
    ) -> Sigma<OtherRaw, OtherInvariant> {
        Sigma::new(f(self.get()))
    }

    /// Apply a function that mutates this value,
    /// then check that the operation maintained this invariant.
    #[inline]
    pub fn map_mut<Y, F: FnOnce(&mut Raw) -> Y>(&mut self, f: F) -> Y {
        let raw = self.get_mut();
        let y = f(raw);
        self.check();
        y
    }

    /// Apply a function to a term that implements a given invariant (say, A),
    /// then check the output for a (possibly different) invariant (say, B).
    #[inline]
    #[expect(clippy::allow_attributes, reason = "Edition 2021 only")]
    #[allow(tail_expr_drop_order, reason = "just for miri")]
    pub fn map_ref<
        OtherRaw: fmt::Debug,
        OtherInvariant: crate::Test<OtherRaw, 1>,
        F: FnOnce(&Raw) -> OtherRaw,
    >(
        &self,
        f: F,
    ) -> Sigma<OtherRaw, OtherInvariant> {
        Sigma::new(f(self))
    }

    /// Create a new sigma type instance by checking an invariant.
    /// # Panics
    /// If the invariant does not hold ***and*** debug assertions are enabled.
    #[inline]
    #[cfg(debug_assertions)]
    pub fn new(raw: Raw) -> Self {
        let provisional = Self {
            phantom: PhantomData,
            raw,
        };
        provisional.check();
        provisional
    }

    /// Create a new sigma type instance by checking an invariant.
    /// # Panics
    /// If the invariant does not hold ***and*** debug assertions are enabled.
    #[inline]
    #[cfg(not(debug_assertions))]
    pub const fn new(raw: Raw) -> Self {
        Self {
            phantom: PhantomData,
            raw,
        }
    }

    /// Without changing its internal value,
    /// try to view one sigma-typed value as implementing another sigma type
    /// by checking the latter invariant at runtime.
    /// # Errors
    /// If the latter invariant does not hold.
    #[inline]
    pub fn try_also<OtherInvariant: crate::Test<Raw, 1>>(
        self,
    ) -> Result<Sigma<Raw, OtherInvariant>, Self> {
        Sigma::try_new(self.get()).map_err(|raw| Self {
            phantom: PhantomData,
            raw,
        })
    }

    /// Without changing its internal value,
    /// try to view one sigma-typed value as implementing another sigma type
    /// by checking the latter invariant at runtime.
    /// # Errors
    /// If the latter invariant does not hold.
    #[inline]
    pub fn try_also_ref<OtherInvariant: crate::Test<Raw, 1>>(
        &self,
    ) -> Result<&Sigma<Raw, OtherInvariant>, OtherInvariant::Error<'_>> {
        let ptr: *const Self = self;
        // SAFETY:
        // Pointer reinterpretation. See `repr(transparent)` above.
        // All non-zero-sized fields are identical across the cast.
        let transmuted: &Sigma<Raw, OtherInvariant> = unsafe { &*ptr.cast() };
        transmuted.try_check()?;
        Ok(transmuted)
    }

    /// Check an invariant without panicking.
    /// # Errors
    /// If the invariant does not hold.
    #[inline(always)]
    pub fn try_check(&self) -> Result<(), Invariant::Error<'_>> {
        Invariant::test([&self.raw])
    }

    /// Create a new sigma type instance by checking an invariant.
    /// # Errors
    /// If the invariant does not hold.
    /// In this case, return the original input unchanged.
    #[inline]
    pub fn try_new(raw: Raw) -> Result<Self, Raw> {
        let provisional = Self {
            phantom: PhantomData,
            raw,
        };

        if provisional.try_check().is_ok() {
            Ok(provisional)
        } else {
            Err(provisional.raw)
        }
    }

    /// Wrap a reference through pointer reinterpretation magic.
    #[inline(always)]
    #[cfg(debug_assertions)]
    pub fn wrap(reference: &Raw) -> &Self {
        let raw_pointer: *const _ = reference;
        let sigma_pointer = raw_pointer.cast::<Self>();
        // SAFETY:
        // `repr(transparent)`
        let wrapped = unsafe { &*sigma_pointer };
        wrapped.check();
        wrapped
    }

    /// Wrap a reference through pointer reinterpretation magic.
    #[inline(always)]
    #[cfg(not(debug_assertions))]
    pub const fn wrap(reference: &Raw) -> &Self {
        let raw_pointer: *const _ = reference;
        let sigma_pointer = raw_pointer.cast::<Self>();
        // SAFETY:
        // `repr(transparent)`
        unsafe { &*sigma_pointer }
    }

    /// Wrap a reference through pointer reinterpretation magic.
    #[inline(always)]
    pub fn wrap_mut(reference: &mut Raw) -> &mut Self {
        let raw_pointer: *mut _ = reference;
        let sigma_pointer = raw_pointer.cast::<Self>();
        // SAFETY:
        // `repr(transparent)`
        let wrapped = unsafe { &mut *sigma_pointer };
        wrapped.check();
        wrapped
    }
}

#[cfg(any(test, feature = "quickcheck"))]
impl<Raw: Arbitrary + fmt::Debug, Invariant: 'static + crate::Test<Raw, 1>> Arbitrary
    for Sigma<Raw, Invariant>
{
    #[inline]
    fn arbitrary(g: &mut Gen) -> Self {
        loop {
            let raw: Raw = Arbitrary::arbitrary(g);
            if let Ok(sigma) = Self::try_new(raw) {
                return sigma;
            }
        }
    }

    #[inline]
    fn shrink(&self) -> Box<dyn Iterator<Item = Self>> {
        let Self {
            phantom: PhantomData,
            ref raw,
        } = *self;
        Box::new(raw.shrink().filter_map(|shrunk| Self::try_new(shrunk).ok()))
    }
}

impl<Raw: fmt::Debug, Invariant: crate::Test<Raw, 1>> AsRef<Raw> for Sigma<Raw, Invariant> {
    #[inline(always)]
    fn as_ref(&self) -> &Raw {
        &self.raw
    }
}

impl<Raw: fmt::Debug, Invariant: crate::Test<Raw, 1>> Borrow<Raw> for Sigma<Raw, Invariant> {
    #[inline(always)]
    fn borrow(&self) -> &Raw {
        &self.raw
    }
}

impl<Raw: CanBeInfinite + fmt::Debug, Invariant: crate::Test<Raw>> CanBeInfinite
    for Sigma<Raw, Invariant>
{
    #[inline(always)]
    fn check_finite(&self) -> bool {
        self.raw.check_finite()
    }
}

impl<Raw: Clone + fmt::Debug, Invariant: crate::Test<Raw, 1>> Clone for Sigma<Raw, Invariant> {
    #[inline(always)]
    fn clone(&self) -> Self {
        Self::new(self.raw.clone())
    }

    #[inline(always)]
    fn clone_from(&mut self, source: &Self) {
        self.raw.clone_from(&source.raw);
        self.check();
    }
}

impl<Raw: Copy + fmt::Debug, Invariant: crate::Test<Raw, 1>> Copy for Sigma<Raw, Invariant> {}

impl<Raw: Default + fmt::Debug, Invariant: crate::Test<Raw, 1>> Default for Sigma<Raw, Invariant> {
    #[inline(always)]
    fn default() -> Self {
        Self::new(Raw::default())
    }
}

impl<Raw: Eq + fmt::Debug, Invariant: crate::Test<Raw, 1>> Eq for Sigma<Raw, Invariant> {
    #[inline(always)]
    fn assert_receiver_is_total_eq(&self) {
        self.raw.assert_receiver_is_total_eq();
    }
}

impl<Raw: Hash + fmt::Debug, Invariant: crate::Test<Raw, 1>> Hash for Sigma<Raw, Invariant> {
    #[inline(always)]
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.raw.hash(state);
    }

    #[inline(always)]
    fn hash_slice<H: Hasher>(data: &[Self], state: &mut H) {
        let ptr: *const [Self] = data;
        #[expect(clippy::as_conversions, reason = "marked `repr(transparent)` above")]
        let transparent = ptr as *const [Raw];
        // SAFETY:
        // Marked `repr(transparent)` above
        let reinterpreted: &[Raw] = unsafe { &*transparent };
        Raw::hash_slice(reinterpreted, state);
    }
}

impl<Raw: Ord + fmt::Debug, Invariant: crate::Test<Raw, 1>> Ord for Sigma<Raw, Invariant> {
    #[inline(always)]
    fn clamp(self, min: Self, max: Self) -> Self {
        Self::new(self.raw.clamp(min.raw, max.raw))
    }

    #[inline(always)]
    fn cmp(&self, other: &Self) -> Ordering {
        self.raw.cmp(&other.raw)
    }

    #[inline(always)]
    fn max(self, other: Self) -> Self {
        Self::new(self.raw.max(other.raw))
    }

    #[inline(always)]
    fn min(self, other: Self) -> Self {
        Self::new(self.raw.min(other.raw))
    }
}

impl<Raw: PartialEq + fmt::Debug, Invariant: crate::Test<Raw, 1>> PartialEq
    for Sigma<Raw, Invariant>
{
    #[inline(always)]
    fn eq(&self, other: &Self) -> bool {
        self.raw.eq(&other.raw)
    }

    #[inline(always)]
    #[expect(
        clippy::partialeq_ne_impl,
        reason = "arbitrary choice between competing lints"
    )]
    fn ne(&self, other: &Self) -> bool {
        self.raw.ne(&other.raw)
    }
}

impl<Raw: PartialOrd + fmt::Debug, Invariant: crate::Test<Raw, 1>> PartialOrd
    for Sigma<Raw, Invariant>
{
    #[inline(always)]
    fn ge(&self, other: &Self) -> bool {
        self.raw.ge(&other.raw)
    }

    #[inline(always)]
    fn gt(&self, other: &Self) -> bool {
        self.raw.gt(&other.raw)
    }

    #[inline(always)]
    fn le(&self, other: &Self) -> bool {
        self.raw.le(&other.raw)
    }

    #[inline(always)]
    fn lt(&self, other: &Self) -> bool {
        self.raw.lt(&other.raw)
    }

    #[inline(always)]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.raw.partial_cmp(&other.raw)
    }
}

impl<Raw: fmt::Debug, Invariant: crate::Test<Raw, 1>> fmt::Debug for Sigma<Raw, Invariant> {
    #[inline(always)]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        #[cfg(feature = "std")]
        if env::var("DEBUG_SIGMA_TYPES").is_ok_and(|s| s != "0") {
            write!(f, "({}) ", Invariant::ADJECTIVE)?;
        }
        fmt::Debug::fmt(&self.raw, f)
    }
}

impl<Raw: fmt::Debug + fmt::Display, Invariant: crate::Test<Raw, 1>> fmt::Display
    for Sigma<Raw, Invariant>
{
    #[inline(always)]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&self.raw, f)
    }
}

impl<Raw: fmt::Debug, Invariant: crate::Test<Raw, 1>> ops::Deref for Sigma<Raw, Invariant> {
    type Target = Raw;

    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.raw
    }
}

#[cfg(feature = "serde")]
#[expect(clippy::missing_trait_methods, reason = "I'm no expert")]
impl<'de, Raw: fmt::Debug + serde::Deserialize<'de>, Invariant: crate::Test<Raw, 1>>
    serde::Deserialize<'de> for Sigma<Raw, Invariant>
{
    #[inline]
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use serde::de::Error;

        let raw = Raw::deserialize(deserializer)?;
        let provisional = Self {
            phantom: PhantomData,
            raw,
        };
        match provisional.try_check() {
            Ok(()) => {}
            Err(e) => return Err(Error::custom(e)),
        }
        Ok(provisional)
    }
}

#[cfg(feature = "serde")]
impl<Raw: fmt::Debug + serde::Serialize, Invariant: crate::Test<Raw, 1>> serde::Serialize
    for Sigma<Raw, Invariant>
{
    #[inline(always)]
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        self.raw.serialize(serializer)
    }
}
