use prusti_contracts::*;

// 5.5.dfy
// function Reduce(m: nat, x: int): int
// lemma {:induction false} ReduceUpperBound(m:nat, x: int)
// lemma {:induction false} ReduceUpperBound1(m:nat, x: int)
// lemma {:induction false} ReduceUpperBound2(m:nat, x: int)
// lemma {:induction false} ReduceUpperBound3(m:nat, x: int)

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

// FUTURE: termination_check
// FUTURE: lemma_syntax
// FUTURE: calc_block
// FUTURE: prusti_assert_eq
#[allow(clippy::only_used_in_recursion)]
#[pure]
#[requires(m >= 0)]
#[ensures(reduce(m, x) <= x)]
pub fn lemma_reduce_upper_bound(m: i64, x: i64) {
    if m == 0 {
    } else {
        prusti_assert!(reduce(m, x) == reduce(m / 2, x + 1) - m);
        lemma_reduce_upper_bound(m / 2, x + 1);
        prusti_assert!(reduce(m / 2, x + 1) <= x + 1);
        prusti_assert!(reduce(m / 2, x + 1) - m <= x + 1 - m);
        prusti_assert!(0 < m);
        prusti_assert!(x + 1 - m <= x);
    }
}

// FUTURE: termination_check
// FUTURE: lemma_syntax
// FUTURE: calc_block
// FUTURE: prusti_assert_eq
#[allow(clippy::only_used_in_recursion)]
#[pure]
#[requires(m >= 0)]
#[ensures(reduce(m, x) <= x)]
pub fn lemma_reduce_upper_bound_1(m: i64, x: i64) {
    if m == 0 {
    } else {
        prusti_assert!(reduce(m, x) == reduce(m / 2, x + 1) - m);
        lemma_reduce_upper_bound_1(m / 2, x + 1);
        prusti_assert!(reduce(m / 2, x + 1) <= x + 1);
        prusti_assert!(x + 1 - m <= x);
    }
}

// FUTURE: termination_check
// FUTURE: lemma_syntax
// FUTURE: calc_block
// FUTURE: prusti_assert_eq
#[allow(clippy::only_used_in_recursion)]
#[pure]
#[requires(m >= 0)]
#[ensures(reduce(m, x) <= x)]
pub fn lemma_reduce_upper_bound_2(m: i64, x: i64) {
    if m == 0 {
    } else {
        prusti_assert!(x >= x + 1 - m);
        lemma_reduce_upper_bound_2(m / 2, x + 1);
        prusti_assert!(x + 1 - m >= reduce(m / 2, x + 1) - m);
        prusti_assert!(reduce(m / 2, x + 1) - m == reduce(m, x));
    }
}

// FUTURE: termination_check
// FUTURE: lemma_syntax
// FUTURE: calc_block
// FUTURE: prusti_assert_eq
#[allow(clippy::only_used_in_recursion)]
#[pure]
#[requires(m >= 0)]
#[ensures(reduce(m, x) <= x)]
pub fn lemma_reduce_upper_bound_3(m: i64, x: i64) {
    if m != 0 {
        prusti_assert!(reduce(m, x) == reduce(m / 2, x + 1) - m);
        lemma_reduce_upper_bound_3(m / 2, x + 1);
        prusti_assert!(reduce(m / 2, x + 1) <= x + 1);
        prusti_assert!(reduce(m / 2, x + 1) - m <= x + 1 - m);
        prusti_assert!(0 < m);
        prusti_assert!(x + 1 - m <= x);
    }
}
