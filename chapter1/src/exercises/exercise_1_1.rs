use prusti_contracts::*;

// 1.1.dfy
// method MaxSum(x: int, y: int) returns (s: int, m: int)
// method CallMaxSum() returns (s: int, m: int)

#[ensures(result.0 == x + y)]
#[ensures(result.1 >= x && result.1 >= y)]
#[ensures(result.1 == x || result.1 == y)]
pub fn max_sum(x: i64, y: i64) -> (i64, i64) {
    let s = x + y;
    let m = if x > y { x } else { y };
    (s, m)
    // FUTURE: std_lib_extern_spec_enhancement
    // (x + y, x.max(y))
}

pub fn call_max_sum() -> (i64, i64) {
    let (s, m) = max_sum(1928, 1);
    prusti_assert!(s == 1929);
    prusti_assert!(m == 1928);
    (s, m)
}
