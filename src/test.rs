extern crate alloc;
extern crate std;

use {
    crate::{NonNegative, Zero as _},
    alloc::{format, vec::Vec},
    core::cmp::Ordering,
    quickcheck::TestResult,
    std::panic::catch_unwind,
};

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
}
