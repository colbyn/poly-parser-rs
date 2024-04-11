use std::fmt::Debug;

use crate::{data::{Output, Parser, QuadrupleParser, TripleParser, TupleParser}, system::Lazy};

impl<A: Clone + 'static + Debug> Parser<A> {
    pub fn and<B: Debug>(
        self,
        next: impl Lazy<Item = Parser<B>>
    ) -> TupleParser<A, B> where B: 'static + Clone {
        TupleParser::<A, B>::init(move |original| {
            if let Output::Ok { value: t, state } = (self.binder)(original.clone()) {
                if let Output::Ok { value: u, state } = (next.clone().evaluate().binder)(state) {
                    return state.ok((t, u))
                }
            }
            original.fail()
        })
    }
    pub fn and2<B: Debug, C: Debug>(
        self,
        f: impl Lazy<Item = Parser<B>>,
        g: impl Lazy<Item = Parser<C>>,
    ) -> TripleParser<A, B, C> where B: Clone + 'static, C: Clone + 'static {
        self.and(f).and(g).map(|((a, b), c)| {
            (a, b, c)
        })
    }
    pub fn and3<B: Debug, C: Debug, D: Debug>(
        self,
        f: impl Lazy<Item = Parser<B>>,
        g: impl Lazy<Item = Parser<C>>,
        h: impl Lazy<Item = Parser<D>>,
    ) -> QuadrupleParser<A, B, C, D> where B: Clone + 'static, C: Clone + 'static, D: Clone + 'static {
        self.and2(f, g).and(h).map(|((a, b, c), d)| {
            (a, b, c, d)
        })
    }



    pub fn and_<B: Debug>(
        self,
        next: Parser<B>
    ) -> TupleParser<A, B> where B: 'static + Clone {
        TupleParser::<A, B>::init(move |original| {
            if let Output::Ok { value: t, state } = (self.binder)(original.clone()) {
                if let Output::Ok { value: u, state } = (next.binder)(state) {
                    return state.ok((t, u))
                }
            }
            original.fail()
        })
    }
    pub fn and2_<B: Debug, C: Debug>(
        self,
        f: Parser<B>,
        g: Parser<C>,
    ) -> TripleParser<A, B, C> where B: Clone + 'static, C: Clone + 'static {
        self.and_(f).and_(g).map(|((a, b), c)| {
            (a, b, c)
        })
    }
    pub fn and3_<B: Debug, C: Debug, D: Debug>(
        self,
        f: Parser<B>,
        g: Parser<C>,
        h: Parser<D>,
    ) -> QuadrupleParser<A, B, C, D> where B: Clone + 'static, C: Clone + 'static, D: Clone + 'static {
        self.and2_(f, g).and_(h).map(|((a, b, c), d)| {
            (a, b, c, d)
        })
    }
}
