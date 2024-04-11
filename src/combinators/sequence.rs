use std::fmt::Debug;

use crate::{data::{ControlFlow, ControlFlowParser, Output, Parser, State, TupleParser}, system::{Constant, Lazy, Thunk}};

#[derive(Default)]
pub struct SequenceSettings {
    allow_empty: Option<bool>,
    until_terminator: Option<ControlFlowParser>,
}

impl SequenceSettings {
    pub fn allow_empty(mut self, flag: bool) -> Self {
        self.allow_empty = Some(flag);
        self
    }
    pub fn until_terminator(mut self, terminator: ControlFlowParser) -> Self {
        self.until_terminator = Some(terminator);
        self
    }
    pub fn terminate_if_ok<T>(mut self, terminator: impl Lazy<Item = Parser<T>>) -> Self where T: Clone + 'static {
        self.until_terminator = Some(ControlFlowParser::terminate_if_ok(terminator));
        self
    }
    pub fn terminate_if_ok_<T>(mut self, terminator: Parser<T>) -> Self where T: Clone + 'static {
        self.until_terminator = Some(ControlFlowParser::terminate_if_ok_(terminator));
        self
    }
}

impl<A: Debug> Parser<A> where A: Clone + 'static + Debug {
    pub fn sequence(self, settings: SequenceSettings) -> Parser<Vec<A>> {
        Parser::<Vec<A>>::init(move |original| {
            let mut leading = Vec::<A>::default();
            let mut trailing: State = original.clone();
            let mut trailing_text_length: usize = trailing.text.len();
            // let mut counter = 0usize;
            'trials : while !trailing.text.is_empty() {
                // counter += 1;
                if let Some(terminator) = settings.until_terminator.as_ref() {
                    if let Output::Ok { value: ControlFlow::Terminate, .. } = (terminator.binder)(trailing.clone()) {
                        break 'trials;
                    }
                }
                if let Output::Ok { value, state } = (self.binder)(trailing.clone()) {
                    println!("Parser.sequence {value:?} {state:?}");
                    if trailing_text_length == state.text.len() {
                        // DON'T LOOP FOREVER
                        break 'trials
                    }
                    trailing_text_length = state.text.len();
                    leading.push(value);
                    trailing = state;
                    continue 'trials;
                }
                break 'trials
            }
            if leading.is_empty() && !settings.allow_empty.unwrap_or(false) {
                return original.fail()
            }
            println!("Parser.sequence FINAL STATE: {leading:?} {trailing:?}");
            trailing.ok(leading)
        })
    }
    pub fn many(self) -> Parser<Vec<A>> {
        let settings = SequenceSettings::default().allow_empty(true);
        self.sequence(settings)
    }
    pub fn some(self) -> Parser<Vec<A>> {
        let settings = SequenceSettings::default().allow_empty(false);
        self.sequence(settings)
    }
    pub fn many_unless<B: Debug>(
        self,
        other: impl Lazy<Item = Parser<B>>
    ) -> TupleParser<Vec<A>, Option<B>> where B: 'static + Clone {
        let settings = SequenceSettings::default()
            .allow_empty(true)
            .until_terminator(ControlFlowParser::terminate_if_ok(other.clone()));
        self.sequence(settings).and(other.map(|o| o.optional()))
    }
    pub fn some_unless<B: Debug>(
        self,
        other: impl Lazy<Item = Parser<B>>
    ) -> TupleParser<Vec<A>, Option<B>> where B: 'static + Clone {
        let settings = SequenceSettings::default()
            .allow_empty(false)
            .until_terminator(ControlFlowParser::terminate_if_ok(other.clone()));
        self.sequence(settings).and(other.map(|o| o.optional()))
    }
    pub fn many_till<B: Debug>(
        self,
        other: impl Lazy<Item = Parser<B>>
    ) -> TupleParser<Vec<A>, B> where B: 'static + Clone {
        let settings = SequenceSettings::default()
            .allow_empty(true)
            .until_terminator(ControlFlowParser::terminate_if_ok(other.clone()));
        self.sequence(settings).and(other)
    }
    pub fn some_till<B: Debug>(
        self,
        other: impl Lazy<Item = Parser<B>>
    ) -> TupleParser<Vec<A>, B> where B: 'static + Clone {
        let settings = SequenceSettings::default()
            .allow_empty(false)
            .until_terminator(ControlFlowParser::terminate_if_ok(other.clone()));
        self.sequence(settings).and(other)
    }
    pub fn many_unless_<B: Debug>(
        self,
        other: Parser<B>,
    ) -> TupleParser<Vec<A>, Option<B>> where B: 'static + Clone {
        let settings = SequenceSettings::default()
            .allow_empty(true)
            .until_terminator(ControlFlowParser::terminate_if_ok_(other.clone()));
        self.sequence(settings).and_(other.clone().optional())
    }
    pub fn some_unless_<B: Debug>(
        self,
        other: Parser<B>,
    ) -> TupleParser<Vec<A>, Option<B>> where B: 'static + Clone {
        let settings = SequenceSettings::default()
            .allow_empty(false)
            .until_terminator(ControlFlowParser::terminate_if_ok_(other.clone()));
        self.sequence(settings).and_(other.clone().optional())
    }
    pub fn many_till_<B: Debug>(
        self,
        other: Parser<B>,
    ) -> TupleParser<Vec<A>, B> where B: 'static + Clone {
        let settings = SequenceSettings::default()
            .allow_empty(true)
            .until_terminator(ControlFlowParser::terminate_if_ok_(other.clone()));
        self.sequence(settings).and_(other.clone())
    }
    pub fn some_till_<B: Debug>(
        self,
        other: Parser<B>,
    ) -> TupleParser<Vec<A>, B> where B: 'static + Clone {
        let settings = SequenceSettings::default()
            .allow_empty(false)
            .until_terminator(ControlFlowParser::terminate_if_ok_(other.clone()));
        self.sequence(settings).and_(other.clone())
    }
}
