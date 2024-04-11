use tree_formatter::ToPrettyTree;

use super::*;

impl<Content> ToPrettyTree for InDoubleQuotes<Content> where Content: ToPrettyTree {
    fn to_pretty_tree(&self) -> tree_formatter::PrettyTree {
        tree_formatter::PrettyTree::branch_of("InDoubleQuotes", vec![
            tree_formatter::PrettyTree::key_value("start_delimiter", self.start_delimiter.to_pretty_tree()),
            tree_formatter::PrettyTree::key_value("content", self.content.to_pretty_tree()),
            tree_formatter::PrettyTree::key_value("end_delimiter", self.end_delimiter.to_pretty_tree()),
        ])
    }
}
impl<Content> ToPrettyTree for InSingleQuotes<Content> where Content: ToPrettyTree {
    fn to_pretty_tree(&self) -> tree_formatter::PrettyTree {
        tree_formatter::PrettyTree::branch_of("InSingleQuotes", vec![
            tree_formatter::PrettyTree::key_value("start_delimiter", self.start_delimiter.to_pretty_tree()),
            tree_formatter::PrettyTree::key_value("content", self.content.to_pretty_tree()),
            tree_formatter::PrettyTree::key_value("end_delimiter", self.end_delimiter.to_pretty_tree()),
        ])
    }
}
impl<Content> ToPrettyTree for InSquareBrackets<Content> where Content: ToPrettyTree {
    fn to_pretty_tree(&self) -> tree_formatter::PrettyTree {
        tree_formatter::PrettyTree::branch_of("InSquareBrackets", vec![
            tree_formatter::PrettyTree::key_value("open_delimiter", self.open_delimiter.to_pretty_tree()),
            tree_formatter::PrettyTree::key_value("content", self.content.to_pretty_tree()),
            tree_formatter::PrettyTree::key_value("close_delimiter", self.close_delimiter.to_pretty_tree()),
        ])
    }
}
impl<Content> ToPrettyTree for InRoundBrackets<Content> where Content: ToPrettyTree {
    fn to_pretty_tree(&self) -> tree_formatter::PrettyTree {
        tree_formatter::PrettyTree::branch_of("InRoundBrackets", vec![
            tree_formatter::PrettyTree::key_value("open_delimiter", self.open_delimiter.to_pretty_tree()),
            tree_formatter::PrettyTree::key_value("content", self.content.to_pretty_tree()),
            tree_formatter::PrettyTree::key_value("close_delimiter", self.close_delimiter.to_pretty_tree()),
        ])
    }
}