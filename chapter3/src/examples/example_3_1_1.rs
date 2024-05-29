use prusti_contracts::*;

// 3.1_1.dfy
// function Fib(n: nat): nat

#[pure]
// #[decreases(n)] // FUTURE: termination_check
pub fn fib(n: u64) -> u64 {
    if n < 2 {
        n
    } else {
        fib(n - 2) + fib(n - 1)
    }
}
