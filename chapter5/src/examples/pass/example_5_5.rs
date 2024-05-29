use prusti_contracts::*;

// 5.5.dfy
// function Reduce(m: nat, x: int): int

// FUTURE: termination_check
#[pure]
#[requires(m >= 0)]
pub fn reduce(m: i64, x: i64) -> i64 {
    if m == 0 {
        x
    } else {
        reduce(m / 2, x + 1) - m
    }
}
