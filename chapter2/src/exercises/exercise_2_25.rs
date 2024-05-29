use prusti_contracts::*;

// 2.25.dfy
// method exerciseA (x: int, y: int)
// method exerciseB (x: int, y: int)
// method exerciseC (x: int, y: int)
// method exerciseD (x: int, y: int)
// method exerciseE (x: int, y: int)

#[ensures(x < 100)]
#[requires(x < 100 && y == 25)] // weakest precondition
pub fn exercise_a(x: i64, y: i64) {
    assert!(y == 25);
}

#[ensures(x < 100)]
#[requires(0 <= x && x < 100)] // weakest precondition
pub fn exercise_b(x: i64, y: i64) {
    assert!(0 <= x);
}

#[ensures(x < 100)]
#[requires(x < 100)] // weakest precondition
pub fn exercise_c(x: i64, y: i64) {
    assert!(x < 200);
}

#[ensures(x < 100)]
#[requires(x < 100)] // weakest precondition
pub fn exercise_d(x: i64, y: i64) {
    assert!(x <= 100);
}

#[allow(clippy::manual_range_contains)]
#[ensures(x < 100)]
#[requires(0 <= x && x < 100)] // weakest precondition
pub fn exercise_e(x: i64, y: i64) {
    // assert!((0..100).contains(&x)); // FUTURE: std_lib_extern_spec_enhancement (contains)
    assert!(0 <= x && x < 100);
}
