use prusti_contracts::*;

// 1.6_1.dfy
// function Average(a: int, b: int): int
// method Triple'(x: int) returns (r: int)

#[pure]
pub fn average(a: i64, b: i64) -> i64 {
    (a + b) / 2
}

#[ensures(average(result, x * 3) == x * 3)]
pub fn triple_wrong(x: i64) -> i64 {
    x * 3 + 1
}

#[ensures(average(result, x * 3) == x * 3)]
#[ensures(result % 2 == (x * 3) % 2)]
pub fn triple_correct(x: i64) -> i64 {
    x * 3
}
