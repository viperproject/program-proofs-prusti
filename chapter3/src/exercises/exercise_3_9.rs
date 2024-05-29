use prusti_contracts::*;

// 3.9.dfy
// function N(x: int, y: int, b: bool): int

#[pure]
// #[decreases(x, b)] // FUTURE: termination_check
pub fn n(x: i64, y: i64, b: bool) -> i64 {
    if x <= 0 || y <= 0 {
        x + y
    } else if b {
        n(x, y + 3, !b)
    } else {
        n(x - 1, y, true)
    }
}
