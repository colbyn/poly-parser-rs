use std::rc::Rc;
use colored::Colorize;
use tree_formatter::{PrettyTree, PrettyTreePrinter, ToPrettyTree};
use crate::{combinators::SequenceSettings, data::{ControlFlow, FatChar, State, Text}};

//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
// SECTION NAME
//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
#[derive(Clone)]
pub struct Parser<A> {
    pub(crate) binder: Rc<dyn Fn(State) -> Output<A>>
}

pub type TextParser = Parser<Text>;
pub type CharParser = Parser<FatChar>;
pub type TupleParser<A, B> = Parser<(A, B)>;
pub type TripleParser<A, B, C> = Parser<(A, B, C)>;
pub type QuadrupleParser<A, B, C, D> = Parser<(A, B, C, D)>;
pub type ControlFlowParser = Parser<ControlFlow>;
pub type VecParser<A> = Parser<Vec<A>>;

impl<T> Parser<T> {
    pub fn evaluate(source: impl AsRef<str>, parser: Self) -> (Option<T>, State) {
        let snippet = State {
            text: Text::initialize_from(source),
        };
        match (parser.binder)(snippet) {
            Output::Ok { value, state } => (Some(value), state),
            Output::Fail { state } => (None, state)
        }
    }
    pub(crate) fn init(f: impl Fn(State) -> Output<T> + 'static) -> Self {
        Self { binder: Rc::new(f) }
    }
}


#[derive(Debug, Clone)]
pub(crate) enum Output<T> {
    Ok { value: T, state: State},
    Fail { state: State },
}
