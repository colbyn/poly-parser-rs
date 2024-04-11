use std::fmt::Debug;

use itertools::Itertools;

use crate::{data::{Either, EitherParser, Output, Parser, QuadrupleParser, TripleParser, TupleParser}, system::{Lazy, Thunk}};

impl<T> Parser<T> where T: Clone + 'static {
    pub fn pure(value: T) -> Self {
        Self::init(move |s| s.ok(value.clone()))
    }
}

impl<T> Parser<T> {
    pub fn fail() -> Self {
        Self::init(|state| state.fail())
    }
}

impl<A: Debug> Parser<A> where A: Clone + 'static {
    pub fn and_then<B: Debug>(
        self,
        right: impl Fn(A) -> Parser<B> + 'static
    ) -> Parser<B> where B: Clone + 'static {
        let left = self.binder.clone();
        Parser::<B>::init(move |s1| {
            match left(s1) {
                Output::Ok { value: t, state: s2 } => (right(t).binder)(s2),
                Output::Fail { state } => state.fail()
            }
        })
    }
    pub fn map<B: Debug>(
        self,
        right: impl Fn(A) -> B + 'static
    ) -> Parser<B> where B: Clone + 'static {
        self.and_then(move |t| {
            Parser::<B>::pure(right(t))
        })
    }
    pub fn optional(self) -> Parser<Option<A>> {
        Parser::<Option<A>>::init(move |state| {
            match (self.binder)(state) {
                Output::Ok { value, state } => state.ok(Some(value)),
                Output::Fail { state } => state.ok(None),
            }
        })
    }
    pub fn options<P: Lazy<Item = Self>>(options: Vec<P>) -> Self {    
        Self::init(move |state| {
            for op in options.clone() {
                match (op.evaluate().binder)(state.clone()) {
                    Output::Ok { value, state } => return state.ok(value),
                    Output::Fail { .. } => continue,
                }
            }
            return state.fail()
        })
    }
    pub fn options_(options: Vec<Self>) -> Self {    
        Self::init(move |state| {
            for op in options.clone() {
                match (op.binder)(state.clone()) {
                    Output::Ok { value, state } => return state.ok(value),
                    Output::Fail { .. } => continue,
                }
            }
            return state.fail()
        })
    }
    pub fn or(self, next: Self) -> Self {
        Self::init(move |state| {
            match (self.binder)(state) {
                Output::Ok { value, state } => state.ok(value),
                Output::Fail { state } => (next.binder)(state),
            }
        })
    }
    pub fn either_or<B: Clone + 'static + Debug>(self, other: impl Lazy<Item = Parser<B>>) -> EitherParser<A, B> {
        EitherParser::<A, B>::init(move |state| {
            match (self.binder)(state) {
                Output::Ok { value, state } => state.ok(Either::Left(value)),
                Output::Fail { state } => {
                    match (other.clone().evaluate().binder)(state) {
                        Output::Ok { value, state } => state.ok(Either::Right(value)),
                        Output::Fail { state } => state.fail(),
                    }
                },
            }
        })
    }
    pub fn between_both_ends<B: Debug>(self, end: Parser<B>) -> TripleParser<B, A, B> where B: Clone + 'static {
        end.clone().and2(Thunk::wrap(move || self.clone()), Thunk::wrap(move || end.clone()))
    }
    pub fn between<Left: Debug, Right: Debug>(
        self,
        left: Parser<Left>,
        right: Parser<Right>,
    ) -> TripleParser<Left, A, Right> where Left: Clone + 'static, Right: Clone + 'static {
        left.and2(
            Thunk::wrap(move || self.clone()),
            Thunk::wrap(move || right.clone()),
        )
    }
}