use tree_formatter::{PrettyTree, ToPrettyTree};

use super::{Output, Text};

#[derive(Debug, Clone)]
pub struct State {
    pub text: Text,
}

impl State {
    pub(crate) fn ok<T>(self, value: T) -> Output<T> {
        Output::Ok { value, state: self }
    }
    pub(crate) fn fail<T>(self) -> Output<T> {
        Output::Fail { state: self }
    }
    pub(crate) fn set_text(&self, text: Text) -> Self {
        Self { text }
    }
}

impl ToPrettyTree for State {
    fn to_pretty_tree(&self) -> PrettyTree {
        PrettyTree::branch_of("State", vec![
            PrettyTree::key_value("text", &self.text)
        ])
    }
}