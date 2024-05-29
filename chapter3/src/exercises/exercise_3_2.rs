use prusti_contracts::*;

// 3.2.dfy
// function H(x: int): int

#[pure]
// #[decreases(x + 60)] // FUTURE: termination_check
pub fn h(x: i64) -> i64 {
    if x < -60 {
        x
    } else {
        h(x - 1)
    }
}
