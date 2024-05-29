use prusti_contracts::*;

// 3.8.dfy
// function M(x: int, b: bool): int

#[pure]
// #[decreases(!b)] // FUTURE: termination_check
pub fn m(x: i64, b: bool) -> i64 {
    if b {
        x
    } else {
        m(x + 25, true)
    }
}
