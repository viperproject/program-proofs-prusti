use prusti_contracts::*;

// 3.0_1.dfy
// method BadDouble(x: int) returns (d: int)

// Adding this to decrease the amount of warnings
#[allow(unconditional_recursion)]
#[allow(clippy::only_used_in_recursion)]
// FUTURE: termination_check
#[ensures(result == 2*x)]
pub fn bad_double(x: i64) -> i64 {
    let y = bad_double(x - 1);
    y + 2
}
