use prusti_contracts::*;

// 5.11.dfy
// lemma EvalEnvDefault(e: Expr, n: string, env: map<string,nat>)
// lemma EvalEnvDefaultList(args: List<Expr>,  op: Op, n: string, c: nat, env: map<string, nat>)

use super::super::super::examples::{Environment, Expr, List, Op};
use super::exercise_5_10::*;

// FUTURE: termination_check
// FUTURE: lemma_syntax
#[pure]
#[requires(env.contains_key(n))] // FUTURE: std_lib_extern_spec_requirement, map_support
#[ensures(snap(e).eval(env) === snap(e).substitute(n, 0).eval(env))]
pub fn lemma_eval_env_default(e: &Expr, n: &str, env: &Environment) {
    if let Expr::Node(op, args) = e {
        lemma_eval_env_default_list(args, *op, n, env);
    }
}

// FUTURE: termination_check
// FUTURE: lemma_syntax
#[allow(clippy::only_used_in_recursion)]
#[pure]
#[requires(env.contains_key(n))] // FUTURE: std_lib_extern_spec_requirement, map_support
#[ensures(Expr::eval_list(snap(args), op, env)
    === Expr::eval_list(Expr::substitute_list(snap(args), n, 0), op, env))]
pub fn lemma_eval_env_default_list(args: &List<Expr>, op: Op, n: &str, env: &Environment) {
    if let List::Cons(e, tail) = args {
        lemma_eval_env_default(e, n, env);
        lemma_eval_env_default_list(tail, op, n, env);
    }
}
