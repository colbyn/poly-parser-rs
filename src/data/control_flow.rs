use tree_formatter::{PrettyTree, ToPrettyTree};

#[derive(Debug, Clone)]
pub enum ControlFlow {
    NoOp,
    Terminate,
}

impl Default for ControlFlow {
    fn default() -> Self {
        ControlFlow::NoOp
    }
}

impl ToPrettyTree for ControlFlow {
    fn to_pretty_tree(&self) -> tree_formatter::PrettyTree {
        match self {
            ControlFlow::NoOp => PrettyTree::value("ControlFlow::NoOp"),
            ControlFlow::Terminate => PrettyTree::value("ControlFlow::Terminate"),
        }
    }
}