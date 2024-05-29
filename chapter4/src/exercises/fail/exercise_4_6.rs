use prusti_contracts::*;

// 4.6.dfy
// datatype Tree<T> = Leaf(data: T) | Node(left: Tree<T>, right: Tree<T>)
// datatype Color = Blue | Yellow | Green | Red

pub enum Color {
    Blue,
    Yellow,
    Green,
    Red,
}

pub enum Tree<T> {
    Leaf(T),
    Node(Box<Tree<T>>, Box<Tree<T>>),
}

impl<T: Clone> Tree<T> {
    // FUTURE: termination_check
    // FUTURE: non_copy_types_in_pure_fn
    // FUTURE: allocation_in_pure_fn
    #[pure]
    pub fn mirror_clone(&self) -> Self {
        match self {
            Tree::Leaf(t) => Tree::Leaf(t.clone()),
            Tree::Node(l, r) => Tree::Node(Box::new(r.mirror_clone()), Box::new(l.mirror_clone())),
        }
    }
}

impl<T> Tree<T> {
    // FUTURE: termination_check
    // FUTURE: non_copy_types_in_pure_fn
    // FUTURE: allocation_in_pure_fn
    #[pure]
    pub fn mirror(self) -> Self {
        match self {
            Tree::Leaf(t) => Tree::Leaf(t),
            Tree::Node(l, r) => Tree::Node(Box::new(r.mirror()), Box::new(l.mirror())),
        }
    }
}
