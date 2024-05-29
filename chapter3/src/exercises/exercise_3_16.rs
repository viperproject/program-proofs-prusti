use prusti_contracts::*;

// 3.16.dfy
// function F(x: nat, y: nat): int

#[pure]
// #[decreases(x % 2 == 1, if x % 2 == 1 {x} else {1000 - x})] // FUTURE: termination_check
pub fn f(x: u64, y: u64) -> u64 {
    if 1000 <= x {
        x + y
    } else if x % 2 == 0 {
        f(x + 2, y + 1)
    } else if x < 6 {
        f(2 * y, y)
    } else {
        f(x - 4, y + 3)
    }
}
