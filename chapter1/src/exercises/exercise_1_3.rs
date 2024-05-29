use prusti_contracts::*;

// 1.3.dfy
// function Average(a: int, b: int): int
// method Triple'(x: int) returns (r: int)

#[pure]
pub fn average(a: i64, b: i64) -> i64 {
    (a + b) / 2
}

#[ensures(average(result, x * 3) == x * 3)]
pub fn triple_(x: i64) -> i64 {
    x * 3 + 1
}
