use prusti_contracts::*;

// 3.6.dfy
// function L(x: int): int

#[pure]
// #[decreases(100 - x)] // FUTURE: termination_check
pub fn l(x: i64) -> i64 {
    if x < 100 {
        l(x + 1) + 10
    } else {
        x
    }
}
