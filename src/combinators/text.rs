use crate::data::{State, TextParser};



impl TextParser {
    pub fn token(value: impl Into<String>) -> Self {
        let value = value.into();
        Self::init(move |state| {
            if let Some((prefix, rest)) = state.text.pop_prefix(&value) {
                assert!(prefix.to_string() == value);
                println!("TextParser.token {prefix} => {rest:?}");
                return State { text: rest }.ok(prefix)
            }
            state.fail()
        })
    }
}