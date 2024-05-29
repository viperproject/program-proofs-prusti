use prusti_contracts::*;

// 1.6_2.dfy
// predicate IsEven(x: int)
// function IsEven2(x: int): bool

#[pure]
#[ensures(result == (x % 2 == 0))] // technically not needed due to #[pure]
pub fn is_even(x: i64) -> bool {
    x % 2 == 0
}
