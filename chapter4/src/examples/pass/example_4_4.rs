use prusti_contracts::*;

// 4.4.dfy
// datatype ColoredTree = Leaf(Color) | Node(ColoredTree, ColoredTree)
// datatype Color = Blue | Yellow | Green | Red
// predicate IsSwedishFlagColor(c: Color)
// predicate IsLithuanianFlagColor(c: Color)

pub enum Color {
    Blue,
    Yellow,
    Green,
    Red,
}

// #[derive(Debug)] // FUTURE: derive_debug
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

    #[pure]
    pub fn is_lithuanian_flag_color(&self) -> bool {
        !matches!(self, Blue)
    }
}
