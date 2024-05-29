use prusti_contracts::*;

// 3.4.dfy
// function J(x: nat, y: nat): int

#[pure]
// #[decreases(4 * x + y)] // FUTURE: termination_check
pub fn j(x: u64, y: u64) -> u64 {
    if x == 0 {
        y
    } else if y == 0 {
        j(x - 1, 3)
    } else {
        j(x, y - 1)
    }
}
