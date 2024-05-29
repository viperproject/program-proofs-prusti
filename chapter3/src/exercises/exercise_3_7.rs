use prusti_contracts::*;

// 3.7.dfy
// function G(n: nat): nat

#[pure]
// #[decreases(n)] // FUTURE: termination_check
// Note: This is required since the decreases notation doesn't exist (yet),
//       otherwise there is an error for potentially negative results of unsigned integer subtraction
//       `result <= n` can be derived from `decreases(n)`
#[ensures(result <= n)]
pub fn g(n: u64) -> u64 {
    if n == 0 {
        0
    } else {
        n - g(g(n - 1))
    }
}
