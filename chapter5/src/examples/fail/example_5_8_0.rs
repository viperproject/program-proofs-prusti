use prusti_contracts::*;

// 5.8.0.dfy
// datatype List<T>
// datatype Op
// datatype Expr
// function method Substitute(e: Expr, n: string, c: nat): Expr
// function method SubstituteList(es: List<Expr>, n: string, c: nat): List<Expr>
// function method Eval(e: Expr, env: map<string,nat>): nat
// function method EvalList(args: List<Expr>, op: Op, env: map<string, nat>): nat
// lemma EvalSubstitute(e: Expr, n: string, c: nat, env: map<string, nat>)
// lemma {:induction false} EvalSubstituteList(args: List<Expr>,  op: Op, n: string, c: nat, env: map<string, nat>)

pub enum List<T> {
    Nil,
    Cons(T, Box<List<T>>),
}

#[derive(Clone, Copy)]
pub enum Op {
    Add,
    Mul,
}

pub enum Expr {
    Const(u64),
    Var(String),
    Node(Op, Box<List<Expr>>),
}

// FUTURE: map_support
// FUTURE: std_lib_extern_spec_requirement
pub type Environment = std::collections::HashMap<String, u64>;

impl Expr {
    // FUTURE: termination_check
    // FUTURE: non_copy_types_in_pure_fn
    // FUTURE: allocation_in_pure_fn (Box::new())
    #[pure]
    pub fn substitute(self, n: &str, c: u64) -> Expr {
        match self {
            Expr::Const(_) => self,
            Expr::Var(s) => {
                if s == n {
                    Expr::Const(c)
                } else {
                    Expr::Var(s)
                }
            }
            Expr::Node(op, args) => Expr::Node(op, Box::new(Self::substitute_list(*args, n, c))),
        }
    }

    // FUTURE: termination_check
    // FUTURE: non_copy_types_in_pure_fn
    // FUTURE: allocation_in_pure_fn (Box::new())
    #[pure]
    pub fn substitute_list(es: List<Expr>, n: &str, c: u64) -> List<Expr> {
        match es {
            List::Nil => es,
            List::Cons(e, tail) => List::Cons(
                e.substitute(n, c),
                Box::new(Self::substitute_list(*tail, n, c)),
            ),
        }
    }

    // FUTURE: termination_check
    // FUTURE: non_copy_types_in_pure_fn
    // FUTURE: allocation_in_pure_fn (Box::new())
    #[pure]
    pub fn eval(self, env: &Environment) -> u64 {
        match self {
            Expr::Const(c) => c,
            Expr::Var(s) => {
                let result = env.get(&s);
                match result {
                    Some(&v) => v, // FUTURE: reference_typed_structures
                    None => 0,
                }
                // if result.is_some() {
                //     *result.unwrap()
                // } else {
                //     0
                // }
                // *result.unwrap_or_else(|| &0)
                // if let Some(v) = env.get(&s) {
                //     *v
                // } else {
                //     0
                // }
            }
            Expr::Node(op, args) => Expr::eval_list(*args, op, env),
        }
    }

    // FUTURE: termination_check
    // FUTURE: non_copy_types_in_pure_fn
    #[pure]
    pub fn eval_list(args: List<Expr>, op: Op, env: &Environment) -> u64 {
        match args {
            List::Nil => match op {
                Op::Add => 0,
                Op::Mul => 1,
            },
            List::Cons(e, tail) => {
                let v0 = e.eval(env);
                let v1 = Self::eval_list(*tail, op, env);
                match op {
                    Op::Add => v0 + v1,
                    Op::Mul => v0 * v1,
                }
            }
        }
    }
}

// FUTURE: lemma_syntax
// FUTURE: map_support
#[pure]
// Cannot encode, because required functions cannot be made pure
// #[ensures(e.substitute(n, c).eval(env) == {env.set(n, c); e.eval()})]
pub fn lemma_eval_substitute(e: &Expr, n: &str, c: u64, env: &Environment) {}

// FUTURE: lemma_syntax
#[pure]
pub fn lemma_eval_substitute_list() {}
