use prusti_contracts::*;

// 5.4.0_2.dfy
// function method More(x: int): int {
// lemma {:induction false} Increasing(x: int)

// FUTURE: termination_check
#[pure]
pub fn more(x: i64) -> i64 {
    if x <= 0 {
        1
    } else {
        more(x - 2) + 3
    }
}

// FUTURE: lemma_syntax
// FUTURE: termination_check
// FUTURE: calc_block
#[pure]
#[ensures(x < more(x))]
pub fn lemma_increasing(x: i64) {
    if x <= 0 {
        prusti_assert!(more(x) == 1);
    } else {
        prusti_assert!(more(x) == more(x - 2) + 3);
        lemma_increasing(x - 2);
        prusti_assert!(x - 2 < more(x - 2));
        prusti_assert!(x + 1 < more(x - 2) + 3);
        prusti_assert!(x + 1 < more(x));
    }
}
