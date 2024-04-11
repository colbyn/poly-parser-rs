use crate::{data::{Output, Parser, QuadrupleParser, TripleParser, TupleParser}, system::Lazy};

impl<A: Clone + 'static> Parser<A> {
    pub fn and<B>(
        self,
        next: impl Lazy<Item = Parser<B>>
    ) -> TupleParser<A, B> where B: 'static + Clone {
        TupleParser::<A, B>::init(move |state| {
            match (self.binder)(state) {
                Output::Ok { value: t, state } => {
                    match (next.clone().evaluate().binder)(state) {
                        Output::Ok { value: u, state } => {
                            return state.ok((t, u))
                        }
                        Output::Fail { state } => state.fail()
                    }
                }
                Output::Fail { state } => state.fail()
            }
        })
    }
    pub fn and2<B, C>(
        self,
        f: impl Lazy<Item = Parser<B>>,
        g: impl Lazy<Item = Parser<C>>,
    ) -> TripleParser<A, B, C> where B: Clone + 'static, C: Clone + 'static {
        self.and(f).and(g).map(|((a, b), c)| {
            (a, b, c)
        })
    }
    pub fn and3<B, C, D>(
        self,
        f: impl Lazy<Item = Parser<B>>,
        g: impl Lazy<Item = Parser<C>>,
        h: impl Lazy<Item = Parser<D>>,
    ) -> QuadrupleParser<A, B, C, D> where B: Clone + 'static, C: Clone + 'static, D: Clone + 'static {
        self.and2(f, g).and(h).map(|((a, b, c), d)| {
            (a, b, c, d)
        })
    }
}
