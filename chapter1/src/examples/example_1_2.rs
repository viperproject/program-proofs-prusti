// 1.2.dfy
// method Triple(x: int) returns (r: int)

// Disable clippy warning for assert!(false);
#[allow(clippy::assertions_on_constants)]

// Note: This file correctly fails verification

// FUTURE: numberOfErrorsToReport
//   Here Prusti should show that (r < 5) is correct, since (r == 10 * x) was checked a line before.
//   See: https://github.com/viperproject/prusti-dev/issues/1213
pub fn triple(x: i64) -> i64 {
    let y = 2 * x;
    let r = x + y;
    assert!(r == 10 * x); // Prusti finds error here
    assert!(r < 5); // not checked, but should be correct
    assert!(false); // not checked, but should be shown as incorrect
    r
}
