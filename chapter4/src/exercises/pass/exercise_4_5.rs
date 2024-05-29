use prusti_contracts::*;

// 4.5.dfy
// datatype ColoredTree = Leaf(Color) | Node(ColoredTree, ColoredTree)
// datatype Color = Blue | Yellow | Green | Red
// predicate IsSwedishFlagColor(c: Color)
// predicate IsSwedishColoredTree(t: ColoredTree)

pub enum Color {
    Blue,
    Yellow,
    Green,
    Red,
}

pub enum ColoredTree {
    Leaf(Color),
    Node(Box<ColoredTree>, Box<ColoredTree>),
}

use Color::*;
use ColoredTree::*;

impl Color {
    #[pure]
    pub fn is_swedish_flag_color(&self) -> bool {
        matches!(self, Blue | Yellow)
    }
}

impl ColoredTree {
    // FUTURE: termination_check
    #[pure]
    pub fn is_swedish_colored_tree(&self) -> bool {
        match self {
            // Leaf(c) if c.is_swedish_flag_color() => true, // FUTURE: shallow_borrow (not needed here, but it shows the missing feature)
            Leaf(c) => c.is_swedish_flag_color(),
            Node(l, r) => l.is_swedish_colored_tree() && r.is_swedish_colored_tree(),
            _ => false,
        }
    }
}
