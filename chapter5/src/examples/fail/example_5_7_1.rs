use prusti_contracts::*;

// 5.7.1.dfy
// datatype Tree<T>
// function Size<T>(t: Tree<T>): nat
// function Mirror<T>(t: Tree<T>): Tree<T>
// lemma {:induction false} MirrorSize<T>(t: Tree<T>)

#[derive(Clone)]
pub enum Tree<T: Clone> {
    Leaf(T),
    Node(Box<Tree<T>>, Box<Tree<T>>),
}

impl<T: Clone> Tree<T> {
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
    // FUTURE: termination_check
    #[pure]
    #[ensures(self.mirror().size() == self.size())]
    fn lemma_mirror_size(&self) {
        todo!();
    }
}
