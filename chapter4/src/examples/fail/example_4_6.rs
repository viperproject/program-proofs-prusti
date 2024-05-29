use prusti_contracts::*;

// 4.6.dfy
// datatype Expr = Const(nat) | Var(string) |Node(op: Op, args: List<Expr>)
// datatype Op = Add | Mul
// datatype List<T> = Nil | Cons(head: T, tail: List<T>)
// function method Eval(e: Expr, env: map<string,nat>): nat
// function method EvalList(args: List<Expr>, op: Op, env: map<string, nat>): nat

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

// FUTURE: map_support
use std::collections::HashMap;
type Environment = HashMap<String, u64>;

use Expr::*;
use List::*;
use Op::*;

impl Expr {
    // FUTURE: reference_typed_structures
    #[pure]
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

    #[pure]
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
}
