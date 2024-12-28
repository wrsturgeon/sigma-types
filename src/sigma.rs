//! Type that maintains a given invariant.

use {
    crate::{NonNegative, OnUnit, One, Positive, Zero},
    core::{borrow::Borrow, fmt, marker::PhantomData, ops},
};

#[cfg(feature = "std")]
use std::env;

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
#[derive(Copy, Clone, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Sigma<Raw: fmt::Debug, Invariant: crate::Test<Raw>> {
    /// Only to silence compiler errors.
    phantom: PhantomData<Invariant>,
    /// Internal type (to which this type will reduce in release builds).
    raw: Raw,
}

impl<Raw: fmt::Debug, Invariant: crate::Test<Raw>> Sigma<Raw, Invariant> {
    /// Without changing its internal value,
    /// view one sigma-typed value as implementing another sigma type
    /// by checking the latter invariant at runtime (iff debug assertions are enabled).
    /// # Panics
    /// If the latter invariant does not hold.
    #[inline]
    #[cfg(debug_assertions)]
    pub fn also<OtherInvariant: crate::Test<Raw>>(&self) -> &Sigma<Raw, OtherInvariant> {
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
    pub const fn also<OtherInvariant: crate::Test<Raw>>(&self) -> &Sigma<Raw, OtherInvariant> {
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
        OtherInvariant: crate::Test<OtherRaw>,
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
        OtherInvariant: crate::Test<OtherRaw>,
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
    pub fn try_also<OtherInvariant: crate::Test<Raw>>(
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
}

impl<Raw: fmt::Debug, Invariant: crate::Test<Raw>> AsRef<Raw> for Sigma<Raw, Invariant> {
    #[inline(always)]
    fn as_ref(&self) -> &Raw {
        &self.raw
    }
}

impl<Raw: fmt::Debug, Invariant: crate::Test<Raw>> Borrow<Raw> for Sigma<Raw, Invariant> {
    #[inline(always)]
    fn borrow(&self) -> &Raw {
        &self.raw
    }
}

impl<Raw: fmt::Debug, Invariant: crate::Test<Raw>> fmt::Debug for Sigma<Raw, Invariant> {
    #[inline(always)]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        #[cfg(feature = "std")]
        if env::var("DEBUG_SIGMA_TYPES").is_ok_and(|s| s != "0") {
            write!(f, "({}) ", Invariant::ADJECTIVE)?;
        }
        fmt::Debug::fmt(&self.raw, f)
    }
}

impl<Raw: fmt::Debug + fmt::Display, Invariant: crate::Test<Raw>> fmt::Display
    for Sigma<Raw, Invariant>
{
    #[inline(always)]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&self.raw, f)
    }
}

impl<Raw: fmt::Debug, Invariant: crate::Test<Raw>> ops::Deref for Sigma<Raw, Invariant> {
    type Target = Raw;

    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.raw
    }
}

#[cfg(feature = "serde")]
#[expect(clippy::missing_trait_methods, reason = "I'm no expert")]
impl<'de, Raw: fmt::Debug + serde::Deserialize<'de>, Invariant: crate::Test<Raw>>
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
impl<Raw: fmt::Debug + serde::Serialize, Invariant: crate::Test<Raw>> serde::Serialize
    for Sigma<Raw, Invariant>
{
    #[inline(always)]
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        self.raw.serialize(serializer)
    }
}
