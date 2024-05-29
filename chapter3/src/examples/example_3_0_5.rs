use prusti_contracts::*;

// 3.0_5.dfy
// function method Dubious(): int

// Adding this to decrease the amount of warnings
#[allow(unconditional_recursion)]
// FUTURE: termination_check
#[pure]
pub fn dubious() -> i64 {
    1 + dubious()
}
