use prusti_contracts::*;

// 3.0_4.dfy
// method Impossible(x: int) returns (y: int)

// Adding this to decrease the amount of warnings
#[allow(unconditional_recursion)]
#[allow(clippy::only_used_in_recursion)]
// FUTURE: termination_check
#[ensures(result % 2 == 0 && result == 10 * x - 3)]
pub fn impossible(x: i64) -> i64 {
    impossible(x)
}
