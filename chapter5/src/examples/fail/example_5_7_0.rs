use prusti_contracts::*;

// 5.7.0.dfy
// datatype Tree<T>
// function Mirror<T>(t: Tree<T>): Tree<T>
// lemma {:induction false} MirrorMirror<T>(t: Tree<T>)

#[derive(Clone, PartialEq, Eq)] // FUTURE: derive_partial_eq
pub enum Tree<T: Clone + Eq> {
    Leaf(T),
    Node(Box<Tree<T>>, Box<Tree<T>>),
}

// impl<T> Eq for Tree<T>
//     where T: Clone + Eq
// {}

// impl<T> PartialEq for Tree<T>
//     where T: Clone + Eq {
//     #[pure]
//     fn eq(&self, other: &Self) -> bool {
//         // FUTURE: reference_typed_structures
//         // FUTURE: std_lib_extern_spec_requirement (PartialEq::eq and Box dereferencing)
//         match (self, other) {
//             (Tree::Leaf(x), Tree::Leaf(y)) => x == y,
//             (Tree::Node(left, right), Tree::Node(other_left, other_right)) => {
//                 left == right && other_left == other_right
//             },
//             _ => false
//         }
//     }
//     // Note: This is a workaround for references in structures not being allowed
//     //       Here another problem stops the verification: The trait function PartialEq::Eq is not marked
//     //       as pure in Prusti. Extern spec for the Box dereferenciation are also still needed here.
//     // FUTURE: std_lib_extern_spec_requirement
//     // #[pure]
//     // fn eq(&self, other: &Self) -> bool {
//     //     match self {
//     //         Tree::Leaf(x) => match other {
//     //             Tree::Leaf(y) => x == y,
//     //             _ => false
//     //         },
//     //         Tree::Node(left, right) => {
//     //             match other {
//     //                 Tree::Node(other_left, other_right) => {
//     //                     other_left == left &&
//     //                     other_right == right
//     //                 },
//     //                 _ => false
//     //             }
//     //         }
//     //     }
//     // }
// }

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
    // FUTURE: lemma_syntax
    // FUTURE: calc_block
    // FUTURE: allocation_in_pure_fn (box::new())
    // FUTURE: std_lib_extern_spec_requirement (for PartialEq::eq)
    #[pure]
    #[ensures(snap(self) === snap(&self).mirror().mirror())]
    fn lemma_mirror_mirror(&self) {
        use Tree::*;
        match self {
            Leaf(l) => {} // trivial
            Node(left, right) => {
                let a = Node(snap(left), snap(right)).mirror().mirror();
                let b = Node(Box::new(right.mirror()), Box::new(left.mirror())).mirror();
                let c = Node(
                    Box::new(left.mirror().mirror()),
                    Box::new(right.mirror().mirror()),
                );
                let d = Node(snap(left), snap(right));
                prusti_assert!(a === b);
                prusti_assert!(b === c);
                left.lemma_mirror_mirror();
                right.lemma_mirror_mirror();
                prusti_assert!(c === d);
            }
        }
    }
}
