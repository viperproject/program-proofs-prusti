use prusti_contracts::*;

// 5.2_2.dfy
// function method More(x: int): int
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
#[pure]
#[ensures(x < more(x))]
pub fn lemma_increasing(x: i64) {
    if 0 < x {
        lemma_increasing(x - 2)
    }
}
