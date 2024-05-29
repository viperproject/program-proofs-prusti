use prusti_contracts::*;

// 2.3.0.dfy
// method Swap0(x: int, y: int)
// method Swap1(x: int, y: int) returns (a: int, b: int)

#[ensures(result == (y, x))]
pub fn swap0(x: i64, y: i64) -> (i64, i64) {
    let tmp = x;
    let x = y;
    let y = tmp;
    (x, y)
}

#[ensures(result == (y, x))]
pub fn swap1(x: i64, y: i64) -> (i64, i64) {
    (y, x)
}
