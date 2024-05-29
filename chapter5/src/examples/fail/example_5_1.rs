use prusti_contracts::*;

// 5.1.dfy
// function method More(x: int): int
// lemma Increasing(x: int)
// method ExampleLemmaUse(a: int)

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
pub fn lemma_increasing(x: i64) {}

pub fn example_lemma_use(a: i64) {
    let b = more(a);
    lemma_increasing(a);
    lemma_increasing(b);
    prusti_assert!(a < b);
    let c = more(b);
    prusti_assert!(2 <= c - a);
}
