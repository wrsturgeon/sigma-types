//! Type that maintains a given invariant.

use {
    crate::{NonNegative, OnUnit, One, Positive, Zero},
    core::{
        borrow::Borrow,
        cmp::Ordering,
        fmt,
        hash::{Hash, Hasher},
        marker::PhantomData,
        ops,
    },
};

#[cfg(feature = "std")]
use std::env;

#[cfg(feature = "quickcheck")]
use quickcheck::{Arbitrary, Gen};

#[cfg(all(feature = "quickcheck", not(feature = "std")))]
use alloc::boxed::Box;

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
    /// Without changing its internal value,
    /// view one sigma-typed value as implementing another sigma type
    /// by checking the latter invariant at runtime (iff debug assertions are enabled).
    /// # Panics
    /// If the latter invariant does not hold.
    #[inline]
    #[cfg(debug_assertions)]
    pub fn also<OtherInvariant: crate::Test<Raw, 1>>(&self) -> &Sigma<Raw, OtherInvariant> {
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
    pub const fn also<OtherInvariant: crate::Test<Raw, 1>>(&self) -> &Sigma<Raw, OtherInvariant> {
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

    /// Apply a function that mutates this value,
    /// then check that the operation maintained this invariant.
    #[inline]
    pub fn modify<Y, F: FnOnce(&mut Raw) -> Y>(&mut self, f: F) -> Y {
        let raw = self.get_mut();
        let y = f(raw);
        self.check();
        y
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
    #[inline]
    pub fn try_new(raw: Raw) -> Option<Self> {
        let provisional = Self {
            phantom: PhantomData,
            raw,
        };
        provisional.try_check().ok()?;
        Some(provisional)
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

#[cfg(feature = "quickcheck")]
impl<Raw: Arbitrary + fmt::Debug, Invariant: 'static + crate::Test<Raw, 1>> Arbitrary
    for Sigma<Raw, Invariant>
{
    #[inline]
    fn arbitrary(g: &mut Gen) -> Self {
        loop {
            if let Some(sigma) = Self::try_new(Arbitrary::arbitrary(g)) {
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
        Box::new(raw.shrink().filter_map(Self::try_new))
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
