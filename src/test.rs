#![allow(
    clippy::allow_attributes,
    reason = "`alloc::format` only warns on some feature combinations"
)]

#[cfg(not(feature = "std"))]
extern crate alloc;

#[cfg(not(feature = "std"))]
extern crate std;

use {
    crate::{NonNegative, Zero as _},
    core::cmp::Ordering,
    quickcheck::TestResult,
};

#[cfg(not(feature = "serde"))]
use serde_json as _;

#[cfg(feature = "quickcheck")]
use {
    crate::Positive,
    quickcheck::{Arbitrary, Gen},
};

#[cfg(not(feature = "std"))]
#[allow(unused_imports, reason = "complicated namespace resolution")]
use alloc::{format, vec::Vec};

#[cfg(debug_assertions)]
use std::panic::catch_unwind;

const _CHECK_ZERO_IMPL_FOR_NON_NEGATIVE: NonNegative<u8> = NonNegative::ZERO;

quickcheck::quickcheck! {
    #[cfg(feature = "quickcheck")]
    fn positive_f64_doesnt_panic_arbitrary(size: usize) -> TestResult {
        let mut g = Gen::new(size);
        let dropped: Positive<f64> = Arbitrary::arbitrary(&mut g);
        _ = dropped;
        TestResult::passed()
    }

    #[cfg(feature = "quickcheck")]
    fn positive_f64_doesnt_panic_shrink(size: usize) -> TestResult {
        let mut g = Gen::new(size);
        let init: Positive<f64> = Arbitrary::arbitrary(&mut g);
        for _ in init.shrink() {}
        TestResult::passed()
    }

    #[cfg(debug_assertions)]
    fn sorted_vec_non_strict(v: Vec<u8>) -> TestResult {
        type Sorted = crate::Sorted<Vec<u8>, true>;
        let actually_sorted = v.is_sorted();
        match catch_unwind(|| Sorted::new(v)) {
            Ok(ok) => {
                if actually_sorted {
                    // Testing `Deref` method syntax:
                    if ok.is_sorted() {
                        for _ in ok {}
                        TestResult::passed()
                    } else {
                        TestResult::error("sigma type not sorted")
                    }
                } else {
                    TestResult::error("not sorted but passed")
                }
            }
            Err(e) => {
                if actually_sorted {
                    TestResult::error(format!("sorted but failed: {e:#?}"))
                } else {
                    TestResult::passed()
                }
            }
        }
    }

    #[cfg(debug_assertions)]
    fn sorted_vec_strict(v: Vec<u8>) -> TestResult {
        type Sorted = crate::Sorted<Vec<u8>, false>;
        let actually_sorted = v.is_sorted_by(|a,b| matches!(a.cmp(b), Ordering::Less));
        match catch_unwind(|| Sorted::new(v)) {
            Ok(ok) => {
                if actually_sorted {
                    // Testing `Deref` method syntax:
                    if ok.is_sorted() {
                        for _ in ok {}
                        TestResult::passed()
                    } else {
                        TestResult::error("sigma type not sorted")
                    }
                } else {
                    TestResult::error("not sorted but passed")
                }
            }
            Err(e) => {
                if actually_sorted {
                    TestResult::error(format!("sorted but failed: {e:#?}"))
                } else {
                    TestResult::passed()
                }
            }
        }
    }

    #[cfg(debug_assertions)]
    fn unit_closed(f: f32) -> TestResult {
        type OnUnit = crate::OnUnit<f32, true, true>;
        let actually_on_unit = ((0.)..=1.).contains(&f);
        match catch_unwind(|| OnUnit::new(f)) {
            Ok(..) => {
                if actually_on_unit {
                    TestResult::passed()
                } else {
                    TestResult::error("not on unit but passed")
                }
            }
            Err(e) => {
                if actually_on_unit {
                    TestResult::error(format!("on unit but failed: {e:#?}"))
                } else {
                    TestResult::passed()
                }
            }
        }
    }

    #[cfg(debug_assertions)]
    fn unit_open(f: f32) -> TestResult {
        type OnUnit = crate::OnUnit<f32, false, false>;
        let actually_on_unit = (f > 0_f32) && (f < 1_f32);
        match catch_unwind(|| OnUnit::new(f)) {
            Ok(..) => {
                if actually_on_unit {
                    TestResult::passed()
                } else {
                    TestResult::error("not on unit but passed")
                }
            }
            Err(e) => {
                if actually_on_unit {
                    TestResult::error(format!("on unit but failed: {e:#?}"))
                } else {
                    TestResult::passed()
                }
            }
        }
    }

    fn try_sorted_vec_strict(v: Vec<u8>) -> TestResult {
        type Sorted = crate::Sorted<Vec<u8>, false>;
        let actually_sorted = v.is_sorted_by(|a,b| matches!(a.cmp(b), Ordering::Less));
        Sorted::try_new(v).map_or_else(
            || {
                if actually_sorted {
                    TestResult::error("sorted but failed")
                } else {
                    TestResult::passed()
                }
            },
            |some| {
                if actually_sorted {
                    // Testing `Deref` method syntax:
                    if some.is_sorted() {
                        for _ in some {}
                        TestResult::passed()
                    } else {
                        TestResult::error("sigma type not sorted")
                    }
                } else {
                    TestResult::error("not sorted but passed")
                }
            },
        )
    }

    #[cfg(debug_assertions)]
    fn i64_non_negative(i: i64) -> TestResult {
        type NonNegative = crate::NonNegative<i64>;
        let actually_non_negative = i >= 0;
        match catch_unwind(|| NonNegative::new(i)) {
            Ok(..) => {
                if actually_non_negative {
                    TestResult::passed()
                } else {
                    TestResult::error("negative but passed")
                }
            }
            Err(e) => {
                if actually_non_negative {
                    TestResult::error(format!("non-negative but failed: {e:#?}"))
                } else {
                    TestResult::passed()
                }
            }
        }
    }

    #[cfg(debug_assertions)]
    fn i64_positive(i: i64) -> TestResult {
        type Positive = crate::Positive<i64>;
        let actually_positive = i > 0;
        match catch_unwind(|| Positive::new(i)) {
            Ok(..) => {
                if actually_positive {
                    TestResult::passed()
                } else {
                    TestResult::error("non-positive but passed")
                }
            }
            Err(e) => {
                if actually_positive {
                    TestResult::error(format!("positive but failed: {e:#?}"))
                } else {
                    TestResult::passed()
                }
            }
        }
    }

    #[cfg(debug_assertions)]
    fn i64_positive_wrap(i: i64) -> TestResult {
        type Positive = crate::Positive<i64>;
        let actually_positive = i > 0;
        match catch_unwind(|| Positive::wrap(&i)) {
            Ok(..) => {
                if actually_positive {
                    TestResult::passed()
                } else {
                    TestResult::error("non-positive but passed")
                }
            }
            Err(e) => {
                if actually_positive {
                    TestResult::error(format!("positive but failed: {e:#?}"))
                } else {
                    TestResult::passed()
                }
            }
        }
    }

    #[cfg(debug_assertions)]
    fn i64_positive_wrap_mut(i: i64) -> TestResult {
        type Positive = crate::Positive<i64>;
        let actually_positive = i > 0;
        match catch_unwind(|| { let mut j = i; *Positive::wrap_mut(&mut j) }) {
            Ok(..) => {
                if actually_positive {
                    TestResult::passed()
                } else {
                    TestResult::error("non-positive but passed")
                }
            }
            Err(e) => {
                if actually_positive {
                    TestResult::error(format!("positive but failed: {e:#?}"))
                } else {
                    TestResult::passed()
                }
            }
        }
    }

    #[cfg(debug_assertions)]
    fn positive_also_non_negative(i: i64) -> () {
        type Positive = crate::Positive<i64>;
        type NonNegative = crate::NonNegative<i64>;
        let Some(positive) = Positive::try_new(i) else {
            return;
        };
        let _: NonNegative = positive.also();
    }

    #[cfg(debug_assertions)]
    fn positive_also_ref_non_negative(i: i64) -> () {
        type Positive = crate::Positive<i64>;
        type NonNegative = crate::NonNegative<i64>;
        let Some(positive) = Positive::try_new(i) else {
            return;
        };
        let _: &NonNegative = positive.also_ref();
    }

    #[cfg(debug_assertions)]
    fn non_negative_try_also_positive(i: i64) -> TestResult {
        type NonNegative = crate::NonNegative<i64>;
        type Positive = crate::Positive<i64>;
        let Some(non_negative) = NonNegative::try_new(i) else {
            return TestResult::discard();
        };
        let maybe_also: Option<Positive> = non_negative.try_also();
        match maybe_also {
            Some(..) => if i == 0 { TestResult::error("Zero but passed") } else { TestResult::passed() },
            None => if i == 0 { TestResult::passed() } else { TestResult::error("Positive but failed") },
        }
    }

    #[cfg(debug_assertions)]
    fn non_negative_try_also_ref_positive(i: i64) -> TestResult {
        type NonNegative = crate::NonNegative<i64>;
        type Positive = crate::Positive<i64>;
        let Some(non_negative) = NonNegative::try_new(i) else {
            return TestResult::discard();
        };
        let maybe_also: Result<&Positive, _> = non_negative.try_also_ref();
        match maybe_also {
            Ok(..) => if i == 0 { TestResult::error("Zero but passed") } else { TestResult::passed() },
            Err(e) => if i == 0 { TestResult::passed() } else { TestResult::error(format!("Positive but failed: {e}")) },
        }
    }

    #[cfg(debug_assertions)]
    #[cfg(feature = "serde")]
    fn serde_roundtrip_positive_i64(i: i64) -> TestResult {
        type Positive = crate::Positive<i64>;
        let Ok(positive) = catch_unwind(|| Positive::new(i)) else {
            return TestResult::discard();
        };
        let json = match serde_json::to_string(&positive) {
            Ok(ok) => ok,
            Err(e) => return TestResult::error(format!("Couldn't serialize {positive:#?}: {e}"))
        };
        let rust: Positive = match serde_json::from_str(&json) {
            Ok(ok) => ok,
            Err(e) => return TestResult::error(format!("Couldn't deserialize {json:#?}: {e}"))
        };
        if positive == rust {
            TestResult::passed()
        } else {
            TestResult::error(format!("{positive:#?} -> {json:#?} -> {rust:#?} =/= {positive:#?}"))
        }
    }

    #[cfg(feature = "serde")]
    fn serde_fallible_positive_i64(i: i64) -> TestResult {
        type Positive = crate::Positive<i64>;
        let actually_positive = i > 0;
        let json = match serde_json::to_string(&i) {
            Ok(ok) => ok,
            Err(e) => return TestResult::error(format!("Couldn't serialize {i:#?}: {e}"))
        };
        let rust: Positive = match serde_json::from_str(&json) {
            Ok(ok) => {
                if actually_positive {
                    ok
                } else {
                    return TestResult::error(format!("Not positive, but deserialization succeeded: {i}"));
                }
            }
            Err(e) => {
                return if actually_positive {
                    TestResult::error(format!("Couldn't deserialize {json:#?}: {e}"))
                } else {
                    TestResult::passed()
                };
            }
        };
        if i == *rust.as_ref() {
            TestResult::passed()
        } else {
            TestResult::error(format!("{i:#?} -> {json:#?} -> {rust:#?} =/= {i:#?}"))
        }
    }

    #[cfg(debug_assertions)]
    fn i64_negative(i: i64) -> TestResult {
        type Negative = crate::Negative<i64>;
        let actually_negative = i < 0;
        match catch_unwind(|| Negative::new(i)) {
            Ok(..) => {
                if actually_negative {
                    TestResult::passed()
                } else {
                    TestResult::error("non-negative but passed")
                }
            }
            Err(e) => {
                if actually_negative {
                    TestResult::error(format!("negative but failed: {e:#?}"))
                } else {
                    TestResult::passed()
                }
            }
        }
    }

    #[cfg(debug_assertions)]
    fn negative_also_non_positive(i: i64) -> () {
        type Negative = crate::Negative<i64>;
        type NonPositive = crate::NonPositive<i64>;
        let Some(negative) = Negative::try_new(i) else {
            return;
        };
        let _: NonPositive = negative.also();
    }

    #[cfg(debug_assertions)]
    fn negative_also_ref_non_positive(i: i64) -> () {
        type Negative = crate::Negative<i64>;
        type NonPositive = crate::NonPositive<i64>;
        let Some(negative) = Negative::try_new(i) else {
            return;
        };
        let _: &NonPositive = negative.also_ref();
    }

    #[cfg(debug_assertions)]
    fn non_positive_try_also_negative(i: i64) -> TestResult {
        type NonPositive = crate::NonPositive<i64>;
        type Negative = crate::Negative<i64>;
        let Some(non_positive) = NonPositive::try_new(i) else {
            return TestResult::discard();
        };
        let maybe_also: Option<Negative> = non_positive.try_also();
        match maybe_also {
            Some(..) => if i == 0 { TestResult::error("Zero but passed") } else { TestResult::passed() },
            None => if i == 0 { TestResult::passed() } else { TestResult::error("Negative but failed") },
        }
    }

    #[cfg(debug_assertions)]
    fn non_positive_try_also_ref_negative(i: i64) -> TestResult {
        type NonPositive = crate::NonPositive<i64>;
        type Negative = crate::Negative<i64>;
        let Some(non_positive) = NonPositive::try_new(i) else {
            return TestResult::discard();
        };
        let maybe_also: Result<&Negative, _> = non_positive.try_also_ref();
        match maybe_also {
            Ok(..) => if i == 0 { TestResult::error("Zero but passed") } else { TestResult::passed() },
            Err(e) => if i == 0 { TestResult::passed() } else { TestResult::error(format!("Negative but failed: {e}")) },
        }
    }

    #[cfg(debug_assertions)]
    #[cfg(feature = "serde")]
    fn serde_roundtrip_negative_i64(i: i64) -> TestResult {
        type Negative = crate::Negative<i64>;
        let Ok(negative) = catch_unwind(|| Negative::new(i)) else {
            return TestResult::discard();
        };
        let json = match serde_json::to_string(&negative) {
            Ok(ok) => ok,
            Err(e) => return TestResult::error(format!("Couldn't serialize {negative:#?}: {e}"))
        };
        let rust: Negative = match serde_json::from_str(&json) {
            Ok(ok) => ok,
            Err(e) => return TestResult::error(format!("Couldn't deserialize {json:#?}: {e}"))
        };
        if negative == rust {
            TestResult::passed()
        } else {
            TestResult::error(format!("{negative:#?} -> {json:#?} -> {rust:#?} =/= {negative:#?}"))
        }
    }

    #[cfg(feature = "serde")]
    fn serde_fallible_negative_i64(i: i64) -> TestResult {
        type Negative = crate::Negative<i64>;
        let actually_negative = i < 0;
        let json = match serde_json::to_string(&i) {
            Ok(ok) => ok,
            Err(e) => return TestResult::error(format!("Couldn't serialize {i:#?}: {e}"))
        };
        let rust: Negative = match serde_json::from_str(&json) {
            Ok(ok) => {
                if actually_negative {
                    ok
                } else {
                    return TestResult::error(format!("Not negative, but deserialization succeeded: {i}"));
                }
            }
            Err(e) => {
                return if actually_negative {
                    TestResult::error(format!("Couldn't deserialize {json:#?}: {e}"))
                } else {
                    TestResult::passed()
                };
            }
        };
        if i == *rust.as_ref() {
            TestResult::passed()
        } else {
            TestResult::error(format!("{i:#?} -> {json:#?} -> {rust:#?} =/= {i:#?}"))
        }
    }
}
