use prusti_contracts::*;

// 3.3.dfy
// function I(x: nat, y: nat): int

#[pure]
// #[decreases(x + y)] // FUTURE: termination_check
pub fn i(x: u64, y: u64) -> u64 {
    if x == 0 || y == 0 {
        12
    } else if x % 2 == y % 2 {
        i(x - 1, y)
    } else {
        i(x, y - 1)
    }
}
