use prusti_contracts::*;

// 5.0.dfy
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

// FUTURE: termination_check
// FUTURE: lemma_syntax
// FUTURE: lemma_induction_proof
#[pure]
#[ensures(x < more(x))]
pub fn lemma_increasing(x: i64) {}

pub fn example_lemma_use(a: i64) {
    lemma_increasing(a);
    let b = more(a);
    let c = more(b);
    if a < 1000 {
        lemma_increasing(more(a));
    }
    prusti_assert!(2 <= c - a || 200 <= a);
}
