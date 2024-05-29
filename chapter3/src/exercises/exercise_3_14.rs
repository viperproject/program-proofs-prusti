use prusti_contracts::*;

// 3.14.dfy
// function F(n: nat): nat
// function M(n: nat): nat

// BUG: f() encoded as both a Viper Function and a Method
// FUTURE: mutually_rec_pure_fn
// See: https://github.com/viperproject/prusti-dev/issues/1214

#[pure]
// #[decreases(n, 1)] // FUTURE: termination_check
pub fn f(n: u64) -> u64 {
    if n == 0 {
        1
    } else {
        n - m(f(n - 1))
    }
}

#[pure]
// #[decreases(n, 0)] // FUTURE: termination_check
pub fn m(n: u64) -> u64 {
    if n == 0 {
        0
    } else {
        n - f(m(n - 1))
    }
}