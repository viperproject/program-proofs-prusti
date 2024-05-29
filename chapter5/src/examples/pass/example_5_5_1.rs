use prusti_contracts::*;

// 5.5.1.dfy
// function Reduce(m: nat, x: int): int
// lemma {:induction false} ReduceLowerBound(m:nat, x: int)

// FUTURE: termination_check
#[pure]
#[requires(m >= 0)]
pub fn reduce(m: i64, x: i64) -> i64 {
    if m == 0 {
        x
    } else {
        reduce(m / 2, x + 1) - m
    }
}

// FUTURE: lemma_syntax
// FUTURE: termination_check
#[allow(clippy::only_used_in_recursion)]
#[pure]
#[requires(m >= 0)]
#[ensures(x - 2 * m <= reduce(m, x))]
pub fn lemma_reduce_lower_bound(m: i64, x: i64) {
    if m == 0 {
    } else {
        // FUTURE: calc_block
        prusti_assert!(reduce(m, x) == reduce(m / 2, x + 1) - m);
        lemma_reduce_lower_bound(m / 2, x + 1);
        prusti_assert!(x + 1 - 2 * (m / 2) <= reduce(m / 2, x + 1));
        prusti_assert!(reduce(m / 2, x + 1) - m >= x + 1 - 2 * (m / 2) - m);
        prusti_assert!(2 * (m / 2) <= m);
        prusti_assert!(x + 1 - m - m > x - 2 * m);
    }
}
