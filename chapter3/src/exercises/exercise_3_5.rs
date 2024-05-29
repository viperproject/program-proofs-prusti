use prusti_contracts::*;

// 3.5.dfy
// function K(x: nat, y: nat, z: nat): int

#[allow(clippy::only_used_in_recursion)]
#[pure]
// #[decreases(x + y)] // FUTURE: termination_check
#[requires(x >= 0 && y >= 0 && z >= 0)]
pub fn k(x: i64, y: i64, z: i64) -> i64 {
    if x < 10 || y < 5 {
        x + y
    } else if z == 0 {
        k(x - 1, y, 5)
    } else {
        k(x, y - 1, z - 1)
    }
}
