use prusti_contracts::*;

// 4.2_3.dfy
// datatype BYTree = BlueLeaf | YellowLeaf| Node(left: BYTree, right: BYTree)
// 4.2_4.dfy
// function BlueCount(t: BYTree): nat

// example_4.2_3
pub enum BYTree {
    BlueLeaf,
    YellowLeaf,
    Node {
        left: Box<BYTree>,
        right: Box<BYTree>,
    },
}

// example_4.2_4
impl BYTree {
    // FUTURE: termination_check
    #[pure]
    pub fn blue_count(&self) -> u64 {
        match self {
            BYTree::BlueLeaf => 1,
            BYTree::YellowLeaf => 0,
            BYTree::Node { left, right } => left.blue_count() + right.blue_count(),
        }
    }
}
