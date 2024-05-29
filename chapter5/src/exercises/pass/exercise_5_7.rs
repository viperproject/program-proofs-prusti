use prusti_contracts::*;

// 5.7.dfy
// function Mult(x: nat, y: nat): nat
// lemma {: induction false} MultCommutative(x: nat, y: nat)

// FUTURE: termination_check
#[pure]
pub fn mult(x: u64, y: u64) -> u64 {
    if y == 0 {
        0
    } else {
        x + mult(x, y - 1)
    }
}

// FUTURE: lemma_syntax
// FUTURE: calc_block
#[pure]
#[ensures(mult(x, y) == mult(y, x))]
// #[decreases(x + y)] // FUTURE: termination_check
pub fn lemma_mult_commutative(x: u64, y: u64) {
    if x == y {
    } else if x == 0 {
        lemma_mult_commutative(x, y - 1);
    } else if y == 0 {
        lemma_mult_commutative(x - 1, y);
    } else if y < x {
        lemma_mult_commutative(y, x);
    } else {
        prusti_assert!(mult(x, y) == x + mult(x, y - 1));
        lemma_mult_commutative(x, y - 1);
        prusti_assert!(x + mult(x, y - 1) == x + mult(y - 1, x));
        prusti_assert!(x + mult(y - 1, x) == x + y - 1 + mult(y - 1, x - 1));
        lemma_mult_commutative(x - 1, y - 1);
        prusti_assert!(x + y - 1 + mult(y - 1, x - 1) == x + y - 1 + mult(x - 1, y - 1));
        prusti_assert!(x + y - 1 + mult(x - 1, y - 1) == y + mult(x - 1, y));
        lemma_mult_commutative(x - 1, y);
        prusti_assert!(y + mult(x - 1, y) == y + mult(y, x - 1));
        prusti_assert!(y + mult(y, x - 1) == mult(y, x));
    }
}
