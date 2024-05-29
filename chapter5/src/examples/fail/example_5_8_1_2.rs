use prusti_contracts::*;

// 5.8.1_1.dfy
// datatype List<T>
// datatype Op
// datatype Expr
// function method Substitute(e: Expr, n: string, c: nat): Expr
// function method SubstituteList(es: List<Expr>, n: string, c: nat): List<Expr>
// function method Eval(e: Expr, env: map<string,nat>): nat
// function method EvalList(args: List<Expr>, op: Op, env: map<string, nat>): nat
// lemma EvalSubstitute(e: Expr, n: string, c: nat, env: map<string, nat>)
// lemma {:induction false} EvalSubstituteList(args: List<Expr>,  op: Op, n: string, c: nat, env: map<string, nat>)
// function method Unit(op: Op): nat
// function method Optimize(e: Expr): Expr
// function method Shorten(op: Op, args: List<Expr>): Expr
// function method OptimizeAndFilter(es: List<Expr>, unit: nat): List<Expr>
// lemma OptimizeCorrect(e: Expr, env: map<string,nat>)
// lemma ShortenCorrect(op:Op, args: List<Expr>, env: map<string,nat>)
// lemma OptimizeAndFilterCorrect(args: List<Expr>, op: Op, env: map<string,nat>)

pub enum List<T> {
    Nil,
    Cons(T, Box<List<T>>),
}

impl<T: Clone> Clone for List<T> {
    fn clone(&self) -> Self {
        match self {
            List::Nil => List::Nil,
            List::Cons(h, tail) => List::Cons(h.clone(), tail.clone()),
        }
    }
}

#[derive(Clone, Copy)]
pub enum Op {
    Add,
    Mul,
}

#[derive(Clone)]
pub enum Expr {
    Const(u64),
    Var(String),
    Node(Op, Box<List<Expr>>),
}

type Environment = std::collections::HashMap<String, u64>;

impl Op {
    #[pure]
    pub fn unit(self) -> u64 {
        match self {
            Op::Add => 0,
            Op::Mul => 1,
        }
    }
}

impl Expr {
    // FUTURE: termination_check
    // FUTURE: allocation_in_pure_fn
    // FUTURE: non_copy_types_in_pure_fn (Box::new())
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
    // FUTURE: allocation_in_pure_fn (Box::new())
    // FUTURE: non_copy_types_in_pure_fn
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

    // FUTURE: termination_check
    // FUTURE: non_copy_types_in_pure_fn
    #[pure]
    pub fn optimize(self) -> Self {
        if let Expr::Node(op, args) = self {
            let args = Self::optimize_and_filter(&args, op.unit());
            Self::shorten(op, args)
        } else {
            self
        }
    }

    // FUTURE: termination_check
    // FUTURE: non_copy_types_in_pure_fn
    #[pure]
    pub fn shorten(op: Op, args: List<Expr>) -> Self {
        match args {
            List::Nil => Expr::Const(op.unit()),
            _ => todo!(), // See example_5_8_1_2.rs
                          // List::Cons =>
        }
    }

    // FUTURE: non_copy_types_in_pure_fn
    #[pure]
    pub fn optimize_and_filter(es: &List<Expr>, unit: u64) -> List<Expr> {
        todo!()
    }
}

// FUTURE: lemma_syntax
#[pure]
// #[ensures(Expr::shorten(op, args).eval(env)
//     === Expr::Node(op, args).eval(env)]
pub fn lemma_shorten_correct(op: Op, args: List<Expr>, env: &mut Environment) {
    todo!()
}

// FUTURE: lemma_syntax
#[pure]
pub fn lemma_eval_substitute_list() {}

// FUTURE: lemma_syntax
#[pure]
// Cannot encode, because other functions cannot be made pure
// #[ensures(self.optimize().eval(env) == self.eval(env))]
pub fn lemma_optimize_correct(e: &Expr, env: &Environment) {
    todo!()
}

// FUTURE: lemma_syntax
#[allow(clippy::only_used_in_recursion)]
#[pure]
#[ensures(Expr::Node(op, Box::new(Expr::optimize_and_filter(args, op.unit()))).eval(env)
    === Expr::Node(op, Box::new((*args).clone())).eval(env))]
pub fn lemma_optimize_and_filter_correct(args: &List<Expr>, op: Op, env: &Environment) {
    match args {
        List::Nil => {}
        List::Cons(e, tail) => {
            lemma_optimize_correct(e, env);
            lemma_optimize_and_filter_correct(tail, op, env);
        }
    }
}
