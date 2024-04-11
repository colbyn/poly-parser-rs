use std::fmt::Debug;

use crate::{data::{CharParser, FatChar, Parser}, system::{Constant, Lazy, Parsable, Thunk}};

use super::*;

impl<Content: Debug> InDoubleQuotes<Content> where Content: Clone + 'static {
    pub fn parser(content: impl Lazy<Item=Parser<Content>>) -> Parser<Self> {
        let double_quote = CharParser::char('\"');
        double_quote
            .clone()
            .and2(
                content,
                Thunk::constant(double_quote.clone()),
            )
            .map(|(open, content, close)| {
                Self {
                    start_delimiter: open,
                    content,
                    end_delimiter: close,
                }
            })
    }
}
impl<Content: Debug> InSingleQuotes<Content> where Content: Clone + 'static {
    pub fn parser(content: impl Lazy<Item=Parser<Content>>) -> Parser<Self> {
        let double_quote = CharParser::char('\'');
        double_quote
            .clone()
            .and2(
                content,
                Thunk::constant(double_quote.clone()),
            )
            .map(|(open, content, close)| {
                Self {
                    start_delimiter: open,
                    content,
                    end_delimiter: close,
                }
            })
    }
}
impl<Content: Debug> InSquareBrackets<Content> where Content: Clone + 'static {
    pub fn parser(content: impl Lazy<Item=Parser<Content>>) -> Parser<Self> {
        let open = CharParser::char(token::bracket::OpenSquareBracket);
        let close = Constant::wrap(CharParser::char(token::bracket::CloseSquareBracket));
        open
            .and2(content, close)
            .map(|(l, c, r)| {
                Self {
                    open_delimiter: l,
                    content: c,
                    close_delimiter: r,
                }
            })
    }
}
impl<Content: Debug> InRoundBrackets<Content> where Content: Clone + 'static {
    pub fn parser(content: impl Lazy<Item=Parser<Content>>) -> Parser<Self> {
        let open = CharParser::char(token::bracket::OpenRoundBracket);
        let close = Constant::wrap(CharParser::char(token::bracket::CloseRoundBracket));
        open
            .and2(content, close)
            .map(|(l, c, r)| {
                Self {
                    open_delimiter: l,
                    content: c,
                    close_delimiter: r,
                }
            })
    }
}


impl Parsable for token::bracket::OpenRoundBracket {
    type Item=FatChar;
    fn parser() -> CharParser {
        CharParser::char(Self::TOKEN)
    }
}
impl Parsable for token::bracket::OpenSquareBracket {
    type Item=FatChar;
    fn parser() -> CharParser {
        CharParser::char(Self::TOKEN)
    }
}
impl Parsable for token::bracket::OpenCurlyBracket {
    type Item=FatChar;
    fn parser() -> CharParser {
        CharParser::char(Self::TOKEN)
    }
}
impl Parsable for token::bracket::OpenAngleBracket {
    type Item=FatChar;
    fn parser() -> CharParser {
        CharParser::char(Self::TOKEN)
    }
}
impl Parsable for token::bracket::CloseRoundBracket {
    type Item=FatChar;
    fn parser() -> CharParser {
        CharParser::char(Self::TOKEN)
    }
}
impl Parsable for token::bracket::CloseSquareBracket {
    type Item=FatChar;
    fn parser() -> CharParser {
        CharParser::char(Self::TOKEN)
    }
}
impl Parsable for token::bracket::CloseCurlyBracket {
    type Item=FatChar;
    fn parser() -> CharParser {
        CharParser::char(Self::TOKEN)
    }
}
impl Parsable for token::bracket::CloseAngleBracket {
    type Item=FatChar;
    fn parser() -> CharParser {
        CharParser::char(Self::TOKEN)
    }
}