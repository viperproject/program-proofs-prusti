use prusti_contracts::*;

// 3.3.3_1.dfy
// function ExpLess1(n: nat): nat

// FUTURE: termination_check
pub fn exp_less1(n: u64) -> u64 {
    if n == 0 {
        0
    } else {
        2 * exp_less1(n - 1) + 1
    }
}
