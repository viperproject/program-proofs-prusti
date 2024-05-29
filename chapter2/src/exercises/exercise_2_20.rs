use prusti_contracts::*;

// 2.20.dfy
// method exercise (x: int) returns (y: int)

#[requires((x == 2) || (x >= 34 && x < 55))]
#[ensures(0 <= result && result < 100)]
// #[ensures((0..100).contains(&result))] // FUTURE: std_lib_extern_spec_enhancement (contains)
pub fn exercise(x: i64) -> i64 {
    if x < 34 {
        if x == 2 {
            x + 1
        } else {
            233
        }
    } else if x < 55 {
        21
    } else {
        144
    }
}
