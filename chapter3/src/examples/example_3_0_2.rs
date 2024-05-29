use prusti_contracts::*;

// 3.0_2.dfy
// method PartialId(x: int) returns (y: int)

// Note (2022-10-19): Neither rustc, clippy or Prusti generate any warnings/errors on this code

// FUTURE: termination_check
#[ensures(result == x)]
pub fn partial_id(x: i64) -> i64 {
    if x % 2 == 0 {
        x
    } else {
        partial_id(x)
    }
}
