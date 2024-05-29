use prusti_contracts::*;

// 5.8.1_1.dfy
// datatype List<T>
// datatype Op
// datatype Expr
// function method Unit(op: Op): nat
// function method Optimize(e: Expr): Expr
// function method Shorten(op: Op, args: List<Expr>): Expr
// function method OptimizeAndFilter(es: List<Expr>, unit: nat): List<Expr>

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
    // FUTURE: non_copy_types_in_pure_fn
    #[pure]
    pub fn optimize(self) -> Self {
        if let Expr::Node(op, args) = self {
            let args = optimize_and_filter(*args, op.unit());
            shorten(op, args)
        } else {
            self
        }
    }
}

// FUTURE: non_copy_types_in_pure_fn
// FUTURE: box_syntax
#[pure]
pub fn shorten(op: Op, args: List<Expr>) -> Expr {
    use List::*;
    match args {
        Nil => Expr::Const(op.unit()),
        Cons(e, next_box) => {
            let next: &List<Expr> = &next_box;
            match next {
                Nil => e,
                Cons(..) => Expr::Node(op, Box::new(Cons(e, next_box))),
            }
        }
    }
}

// FUTURE: termination_check
// FUTURE: non_copy_types_in_pure_fn
// FUTURE: allocation_in_pure_fn (Box::new())
#[pure]
pub fn optimize_and_filter(es: List<Expr>, unit: u64) -> List<Expr> {
    use List::*;
    match es {
        Nil => Nil,
        Cons(e, tail) => {
            let e_ = e.optimize();
            let tail_ = optimize_and_filter(*tail, unit);
            match e_ {
                Expr::Const(u) if u == unit => tail_,
                _ => Cons(e_, Box::new(tail_)),
            }
            // if let Expr::Const(u) = e_ && u == unit {
            //     tail_
            // } else {
            //     Cons(e_, Box::new(tail_))
            // }
        }
    }
}
