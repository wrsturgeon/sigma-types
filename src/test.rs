#[cfg(not(feature = "std"))]
extern crate alloc;

#[cfg(not(feature = "std"))]
extern crate std;

#[cfg(not(feature = "serde"))]
use serde_json as _;

use {
    crate::{NonNegative, Zero as _},
    core::cmp::Ordering,
    quickcheck::TestResult,
    std::panic::catch_unwind,
};

#[cfg(feature = "std")]
use std::format;

#[cfg(not(feature = "std"))]
use alloc::{format, vec::Vec};

const _CHECK_ZERO_IMPL_FOR_NON_NEGATIVE: NonNegative<u8> = NonNegative::ZERO;

quickcheck::quickcheck! {
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

    fn try_sorted_vec_strict(v: Vec<u8>) -> TestResult {
        type Sorted = crate::Sorted<Vec<u8>, false>;
        let actually_sorted = v.is_sorted_by(|a,b| matches!(a.cmp(b), Ordering::Less));
        match Sorted::try_new(v) {
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
}
