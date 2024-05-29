use prusti_contracts::*;

// 1.0.dfy
// method Min(x: int, y: int) returns (m: int)

#[ensures(result <= x && result <= y)]
pub fn min(x: i64, y: i64) -> i64 {
    if x < y {
        x - 1
    } else {
        y - 1
    }
    // FUTURE: std_lib_extern_spec_enhancement
    // x.min(y)
}
