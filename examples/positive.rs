//! Fail if and only if debug assertions are enabled
//! (that is, if this is not a release build):

#![expect(
    unused_crate_dependencies,
    reason = "not every example uses each dev-dependency"
)]

use sigma_types::Positive;

fn main() {
    // This will fail if and only if debug assertions are enabled
    // (that is, if this is not a release build):
    _ = Positive::new(0_i8);
}
