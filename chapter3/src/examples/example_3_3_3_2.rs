use prusti_contracts::*;

// 3.3.3_2.dfy
// function ExpLess1(n: nat): nat
// function ExpLess2(n: nat): nat

// FUTURE: mutually_rec_pure_fn
// Issue: https://github.com/viperproject/prusti-dev/issues/1214

#[pure]
// FUTURE: termination_check
pub fn exp_less1(n: u64) -> u64 {
    if n == 0 {
        0
    } else {
        2 * exp_less2(n - 1) + 1
    }
}

#[pure]
// FUTURE: termination_check
#[requires(1 <= n)]
pub fn exp_less2(n: u64) -> u64 {
    if n == 0 {
        0
    } else {
        2 * exp_less1(n - 1) + 1
    }
}