use prusti_contracts::*;

// 4.5.1.dfy
// datatype Tree<T> = Leaf(data: T) | Node(left: Tree<T>, right: Tree<T>)
// datatype Color = Blue | Yellow | Green | Red
// predicate AllBlue(t: Tree<Color>)
// 4.5.2.dfy
// function Size<T>(t: Tree<T>): nat
// function SizeOmit<T>(t: Tree<T>): nat

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

use Color::*;
use Tree::*;

// example_4.5_1
impl Tree<Color> {
    // FUTURE: termination_check
    #[pure]
    pub fn all_blue(&self) -> bool {
        match self {
            Leaf(Blue) => true,
            Node(left, right) => left.all_blue() && right.all_blue(),
            _ => false,
        }
    }
}

// example_4.5_2
impl<T> Tree<T> {
    // FUTURE: termination_check
    #[pure]
    pub fn size(&self) -> u64 {
        match self {
            Leaf(_) => 1,
            Node(left, right) => {
                // Tree::<T>::size(left) + Tree::<T>::size(right)
                left.size() + right.size()
            }
        }
    }
}
