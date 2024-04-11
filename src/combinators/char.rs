use crate::data::CharParser;

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
            match state.text.uncons() {
                Some((l, r)) if l.value == value => {
                    state.set_text(r).ok(l)
                }
                _ => state.fail()
            }
        })
    }
    pub fn char_if(predicate: impl Fn(char) -> bool + 'static) -> Self {
        Self::init(move |state| {
            match state.text.uncons() {
                Some((l, r)) if predicate(l.value) => {
                    state.set_text(r).ok(l)
                }
                _ => state.fail()
            }
        })
    }
}