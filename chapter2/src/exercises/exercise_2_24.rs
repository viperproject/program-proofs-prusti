use prusti_contracts::*;

// 2.24.dfy
// method exercise (u: int) returns (t: int)

#[requires(u < 0)]
#[ensures(u < result)]
pub fn exercise(u: i64) -> i64 {
    abs(7 * u)
}

#[requires(x != i64::MIN)] // Fix for `-x` potentially overflowing (not from the book)
#[ensures(0 <= result && (x == result || x == -result))]
pub fn abs(x: i64) -> i64 {
    if x >= 0 {
        x
    } else {
        // FUTURE: negate_no_overflow: https://github.com/viperproject/prusti-dev/issues/881
        -x
    }
}
