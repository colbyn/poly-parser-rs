use crate::data::{CharParser, State};

impl CharParser {
    pub fn next() -> Self {
        Self::init(|state| {
            match state.text.uncons() {
                Some((l, r)) => {
                    state.set_text(r).ok(l)
                }
                None => state.fail()
            }
        })
    }
    pub fn char(value: impl Into<char>) -> Self {
        let value = value.into();
        Self::init(move |state| {
            let result = state.text
                .uncons()
                .filter(|(head, rest)| {
                    head.value == value
                });
            if let Some((head, rest)) = result {
                return State { text: rest }.ok(head)
            }
            return state.fail()
        })
    }
    pub fn char_if(predicate: impl Fn(char) -> bool + 'static) -> Self {
        Self::init(move |state| {
            let result = state.text
                .uncons()
                .filter(|(head, rest)| {
                    predicate(head.value)
                });
            if let Some((head, rest)) = result {
                return State { text: rest }.ok(head)
            }
            return state.fail()
        })
    }
}