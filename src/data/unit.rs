use tree_formatter::{PrettyTree, ToPrettyTree};

#[derive(Debug, Clone, Copy)]
pub struct Unit;

impl ToPrettyTree for Unit {
    fn to_pretty_tree(&self) -> tree_formatter::PrettyTree {
        PrettyTree::leaf("Unit")
    }
}