use prusti_contracts::*;

// 4.1_1.dfy
// datatype BYTree = BlueLeaf | YellowLeaf| Node(BYTree, BYTree)
// function BlueCount(t: BYTree): nat
// 4.1_2.dfy
// function LeftLinks(t: BYTree): nat
// 4.2_1.dfy
// predicate IsNode(t: BYTree)
// 4.2_2.dfy
// function GetLeft(t: BYTree): BYTree

// #[derive(std::fmt::Debug)] // FUTURE: derive_debug
enum BYTree {
    BlueLeaf,
    YellowLeaf,
    Node(Box<BYTree>, Box<BYTree>),
}

impl BYTree {
    // example_4.1_1
    // FUTURE: termination_check
    #[pure]
    pub fn blue_count(&self) -> u64 {
        match self {
            BYTree::BlueLeaf => 1,
            BYTree::YellowLeaf => 0,
            BYTree::Node(left, right) => left.blue_count() + right.blue_count(),
        }
    }

    // example_4.1_2
    #[pure]
    pub fn left_links(&self) -> u64 {
        match self {
            BYTree::BlueLeaf => 1,
            BYTree::YellowLeaf => 0,
            BYTree::Node(left, _) => left.left_links(),
        }
    }

    // example_4.2_1
    #[pure]
    pub fn is_node(&self) -> bool {
        matches!(self, BYTree::Node(..))
    }

    // example_4.2_2
    #[pure]
    #[requires(self.is_node())]
    pub fn get_left(&self) -> &BYTree {
        match self {
            BYTree::Node(left, _) => left,
            _ => unreachable!(),
        }
    }
}

use std::fmt;

impl fmt::Debug for BYTree {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BYTree::BlueLeaf => f.write_str("BlueLeaf")?,
            BYTree::YellowLeaf => f.write_str("YellowLeaf")?,
            BYTree::Node(left, right) => {
                f.write_str("[")?;
                left.fmt(f)?;
                f.write_str(", ")?;
                right.fmt(f)?;
                f.write_str("]")?;
            }
        }
        Ok(())
    }
}

// Note: These tests are not part of the book
#[cfg(test)]
mod tests {
    use super::*;

    // these tests are ignored by Prusti (due to #[cfg(test)] and #[test])

    #[test]
    fn test_blue_count() {
        use BYTree::*;
        let left = Node(Box::new(BlueLeaf), Box::new(YellowLeaf));
        let right = Node(Box::new(YellowLeaf), Box::new(BlueLeaf));
        let tree = Node(Box::new(left), Box::new(right));
        let blue_count = tree.blue_count();
        assert_eq!(blue_count, 2);
    }

    #[test]
    fn test_debug_print() {
        use BYTree::*;
        let left = BlueLeaf;
        let right = YellowLeaf;
        let tree = Node(Box::new(left), Box::new(right));
        let result = format!("{tree:?}"); // Note: Can only debug print here if it is implemented/derived for the BYTree // FUTURE: derive_debug
        println!("{result}");
        assert_eq!(result, "[BlueLeaf, YellowLeaf]");
    }
}
