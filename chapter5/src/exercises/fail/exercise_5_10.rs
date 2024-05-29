use prusti_contracts::*;

// 5.10.dfy
// lemma EvalEnv(e: Expr, n: string, env: map<string,nat>)
// lemma {:induction false} EvalEnvList(args: List<Expr>,  op: Op, n: string, c: nat, env: map<string, nat>)

// Like example_5_8_0.rs + eval_env + eval_env_list
use super::super::super::examples::{Environment, Expr, List, Op};

// FUTURE: termination_check
// FUTURE: lemma_syntax
#[pure]
#[requires(env.contains_key(n))] // FUTURE: std_lib_extern_spec_requirement, map_support
#[ensures(snap(e).eval(env) === snap(e).substitute(n, env[n]).eval(env))]
pub fn lemma_eval_env(e: &Expr, n: &str, env: &Environment) {
    if let Expr::Node(op, args) = e {
        lemma_eval_env_list(args, *op, n, env[n], env); // FUTURE: map_support
    }
}

// FUTURE: termination_check
// FUTURE: lemma_syntax
#[allow(clippy::only_used_in_recursion)]
#[pure]
#[requires(env.contains_key(n))] // FUTURE: std_lib_extern_spec_requirement, map_support
#[ensures(Expr::eval_list(snap(args), op, env) === Expr::eval_list(Expr::substitute_list(snap(args), n, env[n]), op, env))]
pub fn lemma_eval_env_list(args: &List<Expr>, op: Op, n: &str, c: u64, env: &Environment) {
    match args {
        List::Nil => {}
        List::Cons(e, tail) => {
            lemma_eval_env(e, n, env);
            lemma_eval_env_list(tail, op, n, env[n], env); // FUTURE: map_support
        }
    }
}
