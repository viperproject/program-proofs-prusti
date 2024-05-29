use prusti_contracts::*;

// 2.19.dfy
// method exercise (x: int) returns (y: int)

#[requires(x >= 4 && x < 8)]
#[ensures(result % 2 == 0)]
pub fn exercise(x: i64) -> i64 {
    if x < 8 {
        if x < 4 {
            let x_new = x + 1;
            unreachable!();
        } else {
            2
        }
    } else if x < 32 {
        1
    } else {
        unreachable!();
    }
}
