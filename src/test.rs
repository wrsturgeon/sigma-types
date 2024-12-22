extern crate alloc;
extern crate std;

use {
    alloc::{format, vec::Vec},
    core::cmp::Ordering,
    quickcheck::TestResult,
    std::panic::catch_unwind,
};

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
}
