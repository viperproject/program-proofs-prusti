use prusti_contracts::*;

// 3.1.dfy
// function G(x: int): int

#[pure]
// #[decreases(x)] // FUTURE: termination_check
pub fn g(x: i64) -> i64 {
    if 0 <= x {
        g(x - 2)
    } else {
        x
    }
}
