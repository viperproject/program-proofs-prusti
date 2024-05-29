use prusti_contracts::*;

// 2.17.dfy
// method exercise (x: int) returns (y: int)

#[requires(x != 5)] // weakest precondition
#[ensures(result < 10)]
pub fn exercise(x: i64) -> i64 {
    if x < 8 {
        if x == 5 {
            10
        } else {
            2
        }
    } else {
        0
    }
}
