use crate::{data::{ControlFlow, ControlFlowParser, Output, Parser, State, TupleParser}, system::{Constant, Lazy}};

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
        self.until_terminator = Some(ControlFlowParser::terminate_if_ok(Constant::wrap(terminator)));
        self
    }
}

impl<A> Parser<A> where A: Clone + 'static {
    pub fn sequence(self, settings: SequenceSettings) -> Parser<Vec<A>> {
        Parser::<Vec<A>>::init(move |state| {
            let mut leading = Vec::<A>::default();
            let mut trailing: State = state.clone();
            'trials : while !trailing.text.is_empty() {
                if let Some(terminator) = settings.until_terminator.as_ref() {
                    match (terminator.binder.as_ref())(trailing.clone()) {
                        Output::Ok { value: ControlFlow::Terminate, state } => {
                            trailing = state;
                            break 'trials;
                        }
                        _ => ()
                    }
                }
                match (self.binder.as_ref())(trailing.clone()) {
                    Output::Ok { value, state } => {
                        trailing = state;
                        leading.push(value);
                        continue 'trials;
                    }
                    Output::Fail { .. } => break 'trials,
                }
            }
            if leading.is_empty() && !settings.allow_empty.unwrap_or(false) {
                return trailing.fail()
            }
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
    pub fn many_unless<B>(
        self,
        other: impl Lazy<Item = Parser<B>>
    ) -> TupleParser<Vec<A>, Option<B>> where B: 'static + Clone {
        let settings = SequenceSettings::default()
            .allow_empty(true)
            .until_terminator(ControlFlowParser::terminate_if_ok(other.clone()));
        self.sequence(settings).and(other.map(|o| o.optional()))
    }
    pub fn some_unless<B>(
        self,
        other: impl Lazy<Item = Parser<B>>
    ) -> TupleParser<Vec<A>, Option<B>> where B: 'static + Clone {
        let settings = SequenceSettings::default()
            .allow_empty(false)
            .until_terminator(ControlFlowParser::terminate_if_ok(other.clone()));
        self.sequence(settings).and(other.map(|o| o.optional()))
    }
    pub fn many_till<B>(
        self,
        other: impl Lazy<Item = Parser<B>>
    ) -> TupleParser<Vec<A>, B> where B: 'static + Clone {
        let settings = SequenceSettings::default()
            .allow_empty(true)
            .until_terminator(ControlFlowParser::terminate_if_ok(other.clone()));
        self.sequence(settings).and(other)
    }
    pub fn some_till<B>(
        self,
        other: impl Lazy<Item = Parser<B>>
    ) -> TupleParser<Vec<A>, B> where B: 'static + Clone {
        let settings = SequenceSettings::default()
            .allow_empty(false)
            .until_terminator(ControlFlowParser::terminate_if_ok(other.clone()));
        self.sequence(settings).and(other)
    }
}
