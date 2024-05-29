use prusti_contracts::*;

// First parts identical to Example 4.6 abd Exercise 4.7

// 4.9.dfy
// datatype Expr = Const(nat) | Var(string) |Node(op: Op, args: List<Expr>)
// datatype Op = Add | Mul
// datatype List<T> = Nil | Cons(head: T, tail: List<T>)
// function method Eval(e: Expr, env: map<string,nat>): nat
// function method EvalList(args: List<Expr>, op: Op, env: map<string, nat>): nat
// predicate GoodEnv(e: Expr, env: map<string, nat>)
// predicate helper(args: List<Expr>, env: map<string, nat>)

pub enum Op {
    Add,
    Mul,
}

pub enum List<T> {
    Nil,
    Cons(T, Box<List<T>>),
}

pub enum Expr {
    Const(u64),
    Var(String),
    Node(Op, Box<List<Expr>>),
}

use std::collections::HashMap;
type Environment = HashMap<String, u64>;

use Expr::*;
use List::*;
use Op::*;

impl Expr {
    // FUTURE: termination_check
    // FUTURE: allocation_in_pure_fn
    // FUTURE: reference_typed_structures
    #[pure]
    #[requires(self.good_env(env))]
    pub fn eval(&self, env: &Environment) -> u64 {
        match self {
            Const(c) => *c,
            Var(s) => {
                if let Some(&val) = env.get(s) {
                    val
                } else {
                    0
                }
            }
            Node(op, args) => Expr::eval_list(args, op, env),
        }
    }

    #[pure] // FUTURE: allocation_in_pure_fn
    // #[decreases(args)] // FUTURE: termination_check
    #[requires(args.helper(env))]
    fn eval_list(args: &List<Expr>, op: &Op, env: &Environment) -> u64 {
        match args {
            Nil => match op {
                Add => 0,
                Mul => 1,
            },
            Cons(e, tail) => {
                let v0 = e.eval(env);
                let v1 = Expr::eval_list(tail, op, env);
                match op {
                    Add => v0 + v1,
                    Mul => v0 * v1,
                }
            }
        }
    }

    // New parts of Exercise 4.8 start here:
    // FUTURE: mutually_rec_pure_fn
    // FUTURE: termination_check
    #[pure]
    pub fn good_env(&self, env: &Environment) -> bool {
        match self {
            Const(_) => true,
            Var(s) => env.contains_key(s), // FUTURE: map_support, std_lib_extern_spec_enhancement
            Node(_, args) => args.helper(env),
        }
    }
}

impl List<Expr> {
    #[pure]
    fn helper(&self, env: &Environment) -> bool {
        match self {
            Nil => true,
            Cons(head, tail) => head.good_env(env) && tail.helper(env),
        }
    }
}
