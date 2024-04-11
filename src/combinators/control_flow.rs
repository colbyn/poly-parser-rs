use std::path::Iter;

use crate::{data::{ControlFlow, ControlFlowParser, Output, Parser}, system::Lazy};

impl ControlFlowParser {
    pub fn terminate_if_ok<T>(parser: impl Lazy<Item=Parser<T>> + 'static) -> Self where T: 'static + Clone {
        Self::init(move |state| {
            match (parser.clone().evaluate().binder)(state.clone()) {
                Output::Ok { .. } => state.ok(ControlFlow::Terminate),
                Output::Fail { .. } => state.ok(ControlFlow::NoOp),
            }
        })
    }
    pub fn terminate_if_ok_<T>(parser: Parser<T>) -> Self where T: 'static + Clone {
        Self::init(move |state| {
            match (parser.clone().binder)(state.clone()) {
                Output::Ok { .. } => state.ok(ControlFlow::Terminate),
                Output::Fail { .. } => state.ok(ControlFlow::NoOp),
            }
        })
    }
}
