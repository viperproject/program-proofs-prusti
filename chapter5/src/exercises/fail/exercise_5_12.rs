use prusti_contracts::*;

// 5.12.dfy
// lemma Sublist(args: List<Expr>, n: string, c: nat)
// lemma SubstituteIdempotent(e: Expr, n: string, c: nat)

use super::super::super::examples::{Expr, List};
use super::exercise_5_11::*;

// FUTURE: termination_check
// FUTURE: lemma_syntax
// FUTURE: non_copy_types_in_pure_fn
#[pure]
#[ensures(Expr::substitute_list(Expr::substitute_list(snap(args), n, c), n, c)
    === Expr::substitute_list(snap(args), n, c))]
pub fn lemma_sublist(args: &List<Expr>, n: &str, c: u64) {
    if let List::Cons(e, tail) = args {
        lemma_substitute_idempotent(e, n, c);
        lemma_sublist(tail, n, c);
    }
}

// FUTURE: termination_check
// FUTURE: lemma_syntax
#[pure]
#[ensures(snap(e).substitute(n, c).substitute(n, c)
    === snap(e).substitute(n, c))]
pub fn lemma_substitute_idempotent(e: &Expr, n: &str, c: u64) {
    if let Expr::Node(op, args) = e {
        // FUTURE: calc_block
        // FUTURE: allocation_in_pure_fn
        // FUTURE: non_copy_types_in_pure_fn
        let b0 = snap(e).substitute(n, c).substitute(n, c); // Note: snap should not be called in normal code
        let b1 =
            Expr::Node(*op, Box::new(Expr::substitute_list(snap(args), n, c))).substitute(n, c);
        let b2 = Expr::Node(
            *op,
            Box::new(Expr::substitute_list(
                Expr::substitute_list(snap(args), n, c),
                n,
                c,
            )),
        );
        let b3 = snap(e).substitute(n, c); // Note: snap should not be called in normal code

        prusti_assert!(b0 === b1);
        prusti_assert!(b1 === b2);
        lemma_sublist(args, n, c);
        prusti_assert!(b2 === b3);
    }
}
