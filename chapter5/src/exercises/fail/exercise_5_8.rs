use prusti_contracts::*;

// 5.8.dfy
// datatype Tree<T> = Leaf(data: T) | Node(left: Tree<T>, right: Tree<T>)
// function Size<T>(t: Tree<T>): nat
// function Mirror<T>(t: Tree<T>): Tree<T>
// lemma {:induction false} MirrorSize<T>(t: Tree<T>)

#[derive(Clone)]
pub enum Tree<T: Clone + Eq> {
    Leaf(T),
    Node(Box<Tree<T>>, Box<Tree<T>>),
}

impl<T: Clone + Eq> Tree<T> {
    // FUTURE: termination_check
    // FUTURE: std_lib_extern_spec_requirement (for clone)
    // FUTURE: allocation_in_pure_fn
    // FUTURE: non_copy_types_in_pure_fn
    #[pure]
    pub fn mirror(&self) -> Tree<T> {
        match self {
            Tree::Leaf(l) => Tree::Leaf(l.clone()),
            Tree::Node(left, right) => {
                Tree::Node(Box::new(right.mirror()), Box::new(left.mirror()))
            }
        }
    }

    // FUTURE: termination_check
    #[pure]
    pub fn size(&self) -> u64 {
        match self {
            Tree::Leaf(l) => 1,
            Tree::Node(left, right) => left.size() + right.size(),
        }
    }

    // FUTURE: lemma_syntax
    // FUTURE: non_copy_types_in_pure_fn
    // FUTURE: std_lib_extern_spec_requirement (for box deref)
    #[pure]
    #[ensures(self.size() == self.mirror().size())]
    pub fn lemma_mirror_size(&self) {
        match self {
            Tree::Leaf(_) => {} // trivial
            Tree::Node(left, right) => {
                left.lemma_mirror_size();
                right.lemma_mirror_size();
            }
        }
    }
}
