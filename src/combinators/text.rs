use crate::data::TextParser;



impl TextParser {
    pub fn token(value: impl ToString) -> Self {
        let value = value.to_string();
        Self::init(move |state| {
            match state.text.pop_prefix(&value) {
                Some((prefix, rest)) => state.set_text(rest).ok(prefix),
                None => state.fail()
            }
        })
    }
}