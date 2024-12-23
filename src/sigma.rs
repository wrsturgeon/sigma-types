//! Type that maintains a given invariant.

use {
    crate::{Zero, non_negative::NonNegative},
    core::{fmt, ops},
};

#[cfg(debug_assertions)]
use crate::non_negative::NonNegativeInvariant;

#[cfg(not(debug_assertions))]
use core::marker::PhantomData;

impl<Z: fmt::Debug + PartialOrd + Zero> Zero for NonNegative<Z> {
    const ZERO: Self = Self {
        raw: Z::ZERO,
        #[cfg(debug_assertions)]
        test: NonNegativeInvariant::new(),
        #[cfg(not(debug_assertions))]
        phantom: PhantomData,
    };
}

/// Type that maintains a given invariant.
#[derive(Copy, Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Sigma<Raw: fmt::Debug, Invariant: crate::Test<Raw>> {
    /// Internal type (to which this type will reduce in release builds).
    raw: Raw,
    /// Function-like type that checks the raw type for a specified invariant.
    #[cfg(debug_assertions)]
    test: Invariant,
    #[cfg(not(debug_assertions))]
    phantom: PhantomData<Invariant>,
}

impl<Raw: fmt::Debug, Invariant: crate::Test<Raw>> Sigma<Raw, Invariant> {
    /// Create a new sigma type instance by checking an invariant
    /// if and only if debug assertions are enabled.
    /// # Panics
    /// If the invariant does not hold ***and*** debug assertions are enabled.
    #[inline]
    pub fn check(&self) {
        #[expect(
            clippy::panic,
            reason = "Returning a result would break API in release builds"
        )]
        match Invariant::test(&self.raw) {
            Ok(()) => {}
            Err(message) => {
                panic!("{:#?} is not {}: {message}", self.raw, Invariant::ADJECTIVE);
            }
        }
    }

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
        f(self.get_ref())
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
    pub fn map<
        OtherRaw: fmt::Debug,
        OtherInvariant: crate::Test<OtherRaw>,
        F: FnOnce(Raw) -> OtherRaw,
    >(
        self,
        f: F,
    ) -> Sigma<OtherRaw, OtherInvariant> {
        let raw = self.get();
        let other_raw = f(raw);
        Sigma::new(other_raw)
    }

    /// Apply a function to a term that implements a given invariant (say, A),
    /// then check the output for a (possibly different) invariant (say, B).
    #[inline]
    pub fn map_ref<
        OtherRaw: fmt::Debug,
        OtherInvariant: crate::Test<OtherRaw>,
        F: FnOnce(&Raw) -> OtherRaw,
    >(
        &self,
        f: F,
    ) -> Sigma<OtherRaw, OtherInvariant> {
        let raw = self.get_ref();
        let other_raw = f(raw);
        Sigma::new(other_raw)
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
    pub fn new(raw: Raw) -> Self {
        let provisional = Self {
            raw,
            #[cfg(debug_assertions)]
            test: Default::default(),
            #[cfg(not(debug_assertions))]
            phantom: PhantomData,
        };
        provisional.check();
        provisional
    }
}

impl<Raw: fmt::Debug, Invariant: crate::Test<Raw>> ops::Deref for Sigma<Raw, Invariant> {
    type Target = Raw;

    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        self.get_ref()
    }
}
