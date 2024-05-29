use prusti_contracts::*;

// 5.0.dfy
// function method More(x: int): int
// lemma Increasing(x: int)

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
// FUTURE: lemma_induction_proof
#[pure]
#[ensures(x < more(x))]
pub fn lemma_increasing_automatic(x: i64) {}
