use prusti_contracts::*;

// 3.0.dfy
// function F(x: int): int

#[pure]
// #[decreases(x)] // FUTURE: termination_check
pub fn f(x: i64) -> i64 {
    if x < 10 {
        x
    } else {
        f(x - 1)
    }
}
