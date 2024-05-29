use prusti_contracts::*;

// 1.2.dfy
// method MaxSum(x: int, y: int) returns (s: int, m: int)
// method ReconstructFromMaxSum(s: int, m: int) returns (x: int, y: int)
// method TestMaxSum(x: int, y: int)

use super::exercise_1_1::max_sum;

#[ensures(s == result.0 + result.1)]
#[ensures((m == result.0 || m == result.1) && result.0 <= m && result.1 <= m)]
#[requires(s <= 2 * m)]
pub fn reconstruct_from_max_sum(s: i64, m: i64) -> (i64, i64) {
    let x = m;
    let y = s - m;
    (x, y)
}

pub fn test_max_sum(x: i64, y: i64) {
    let (s, m) = max_sum(x, y);
    let (x_, y_) = reconstruct_from_max_sum(s, m);
    prusti_assert!((x_ == x && y_ == y) || (x_ == y && y_ == x));
}
