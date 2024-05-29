use prusti_contracts::*;

// 4.1.dfy
// datatype BYTree = BlueLeaf | YellowLeaf| Node(BYTree, BYTree)
// function ReverseColors(t: BYTree): BYTree
// 4.2.dfy
// function Oceanize(t: BYTree): BYTree
// 4.3.dfy
// function LeftLinks(t: BYTree): nat
// 4.4.dfy
// predicate HasLeftTree1(t: BYTree, u: BYTree)
// predicate HasLeftTree2(t: BYTree, u: BYTree)

// #[derive(Debug)] // FUTURE: derive_debug
// #[derive(PartialEq, Eq)] // FUTURE: derive_partial_eq
#[derive(Clone)]
pub enum BYTree {
    BlueLeaf,
    YellowLeaf,
    Node(Box<BYTree>, Box<BYTree>),
}

// // FUTURE: allocation_in_pure_fn
// #[extern_spec]
// impl<T> Box<T> {
//     #[pure]
//     #[trusted]
//     fn new(t: T) -> Self;
// }

use BYTree::*;

// FUTURE: non_copy_types_in_pure_fn
// FUTURE: allocation_in_pure_fn
#[pure]
pub fn reverse_colors(t: &BYTree) -> BYTree {
    match t {
        BlueLeaf => BlueLeaf,
        YellowLeaf => YellowLeaf,
        Node(left, right) => Node(
            Box::new(reverse_colors(left)),
            Box::new(reverse_colors(right)),
        ),
    }
}

impl BYTree {
    // Exercise 4.1
    // FUTURE: non_copy_types_in_pure_fn
    // FUTURE: allocation_in_pure_fn
    #[pure]
    pub fn reverse_colors(&self) -> BYTree {
        match self {
            BlueLeaf => YellowLeaf,
            YellowLeaf => BlueLeaf,
            Node(left, right) => Node(
                Box::new(right.reverse_colors()),
                Box::new(left.reverse_colors()),
            ),
        }
    }

    // Exercise 4.2
    // FUTURE: allocation_in_pure_fn
    // FUTURE: allocation_in_pure_fn
    #[pure]
    pub fn oceanize(&self) -> BYTree {
        match self {
            BlueLeaf => BlueLeaf,
            YellowLeaf => BlueLeaf,
            Node(left, right) => Node(Box::new(left.oceanize()), Box::new(right.oceanize())),
        }
    }

    // Exercise 4.3
    #[pure] // FUTURE: allocation_in_pure_fn
    pub fn left_links(&self) -> u64 {
        match self {
            BlueLeaf => 1,
            YellowLeaf => 0,
            Node(left, _) => left.left_links(),
        }
    }

    // // Exercise 4.4
    // #[pure] // FUTURE: allocation_in_pure_fn
    // pub fn has_left_tree(&self, u: &BYTree) -> bool {
    //     if let Node(left, _) = self {
    //         left.as_ref() == u // FUTURE: derive_partial_eq
    //     } else {
    //         false
    //     }
    // }

    // // Exercise 4.4
    // #[pure] // FUTURE: allocation_in_pure_fn
    pub fn has_left_tree(&self, u: &BYTree) -> bool {
        fn eq(a: &BYTree, b: &BYTree) -> bool {
            match (a, b) {
                (BlueLeaf, BlueLeaf) | (YellowLeaf, YellowLeaf) => true,
                (Node(a_left, a_right), Node(b_left, b_right)) => {
                    eq(a_left, b_left) && eq(a_right, b_right)
                }
                _ => false,
            }
        }
        if let Node(left, _) = self {
            eq(left.as_ref(), u)
        } else {
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // these tests are ignored by Prusti (due to cfg(test))

    #[test]
    fn test_has_left_tree_0() {
        use BYTree::*;
        let left = Node(Box::new(BlueLeaf), Box::new(YellowLeaf));
        let u = left.clone();
        let right = Node(Box::new(YellowLeaf), Box::new(BlueLeaf));
        let tree = Node(Box::new(left), Box::new(right));
        assert!(tree.has_left_tree(&u));
    }

    #[test]
    fn test_has_left_tree_1() {
        use BYTree::*;
        let left = Node(Box::new(BlueLeaf), Box::new(YellowLeaf));
        let right = Node(Box::new(YellowLeaf), Box::new(BlueLeaf));
        let u = right.clone();
        let tree = Node(Box::new(left), Box::new(right));
        assert!(!tree.has_left_tree(&u));
    }
}
