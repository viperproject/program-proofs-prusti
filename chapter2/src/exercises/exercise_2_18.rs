use prusti_contracts::*;

// 2.18.dfy
// method exercise (x: int) returns (y: int)

#[requires(x >= 10)]
#[ensures(result % 2 == 0)]
pub fn exercise(x: i64) -> i64 {
    if x < 10 {
        if x < 20 {
            1
        } else {
            2
        }
    } else {
        4
    }
}
