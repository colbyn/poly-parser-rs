use itertools::Itertools;

use crate::{combinators::SequenceSettings, data::{CharParser, Parser, Text, TextParser, VecParser}, system::{Lazy, Thunk}};

use super::*;

//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
// SECTION NAME
//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
#[derive(Debug, Clone, Default)]
pub struct Environment {}

//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
// SECTION NAME
//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
impl Markdown {
    pub fn parser(env: Environment) -> Parser<Self> {
        Parser::options(vec![
            crate::thunk!{
                (env) => Block::parser(env).map(Markdown::Block)
            },
            crate::thunk!{
                (env) => Inline::parser(env).map(Markdown::Inline)
            }
        ])
    }
    pub fn some(env: Environment) -> VecParser<Self> {
        let settings = SequenceSettings::default().allow_empty(false);
        Self::parser(env).sequence(settings)
    }
    pub fn many(env: Environment) -> VecParser<Self> {
        let settings = SequenceSettings::default().allow_empty(true);
        Self::parser(env).sequence(settings)
    }
}

//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
// SECTION NAME
//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
impl Inline {
    pub fn parser(env: Environment) -> Parser<Self> {
        Parser::<Self>::options(vec![
            crate::thunk! {
                (env) => inline::PlainText::parser(env).map(Inline::PlainText)
            },
            crate::thunk! {
                (env) => inline::Link::parser(env).map(Inline::Link)
            },
            crate::thunk! {
                (env) => inline::Image::parser(env).map(Inline::Image)
            },
            crate::thunk! {
                (env) => inline::Emphasis::parser(env).map(Inline::Emphasis)
            },
            crate::thunk! {
                (env) => inline::Highlight::parser(env).map(Inline::Highlight)
            },
            crate::thunk! {
                (env) => inline::Strikethrough::parser(env).map(Inline::Strikethrough)
            },
            crate::thunk! {
                (env) => inline::Subscript::parser(env).map(Inline::Subscript)
            },
            crate::thunk! {
                (env) => inline::Superscript::parser(env).map(Inline::Superscript)
            },
            crate::thunk! {
                (env) => inline::InlineCode::parser(env).map(Inline::InlineCode)
            },
            crate::thunk! {
                (env) => inline::Latex::parser(env).map(Inline::Latex)
            },
        ])
    }
    pub fn some(env: Environment) -> VecParser<Self> {
        let settings = SequenceSettings::default().allow_empty(false);
        Self::parser(env).sequence(settings)
    }
    pub fn many(env: Environment) -> VecParser<Self> {
        let settings = SequenceSettings::default().allow_empty(true);
        Self::parser(env).sequence(settings)
    }
    pub fn plain_text(env: Environment) -> TextParser {
        let terminator = Inline::RESERVED_TOKENS
            .iter()
            .map(|x| CharParser::char(*x))
            .collect_vec();
        let terminator = CharParser::options_(terminator);
        let settings = SequenceSettings::default()
            .allow_empty(false)
            .terminate_if_ok_(terminator);
        CharParser::next().sequence(settings).map(Text::from_iter)
    }
}

impl inline::PlainText {
    pub fn parser(env: Environment) -> Parser<Self> {
        Inline::plain_text(env).map(|x| inline::PlainText {value: x})
    }
}
impl inline::Link {
    pub fn parser(env: Environment) -> Parser<Self> {
        unimplemented!()
    }
}
impl inline::Image {
    pub fn parser(env: Environment) -> Parser<Self> {
        unimplemented!()
    }
}
impl inline::Emphasis {
    pub fn parser(env: Environment) -> Parser<Self> {
        unimplemented!()
    }
}
impl inline::Highlight {
    pub fn parser(env: Environment) -> Parser<Self> {
        unimplemented!()
    }
}
impl inline::Strikethrough {
    pub fn parser(env: Environment) -> Parser<Self> {
        unimplemented!()
    }
}
impl inline::Subscript {
    pub fn parser(env: Environment) -> Parser<Self> {
        unimplemented!()
    }
}
impl inline::Superscript {
    pub fn parser(env: Environment) -> Parser<Self> {
        unimplemented!()
    }
}
impl inline::InlineCode {
    pub fn parser(env: Environment) -> Parser<Self> {
        unimplemented!()
    }
}
impl inline::Latex {
    pub fn parser(env: Environment) -> Parser<Self> {
        unimplemented!()
    }
}

//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
// SECTION NAME
//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
impl Block {
    pub fn parser(env: Environment) -> Parser<Self> {
        Parser::<Self>::options(vec![
            crate::thunk!{
                (env) => CharParser::char('\n').map(Block::Newline)
            },
            crate::thunk!{
                (env) => block::Heading::parser(env).map(Block::Heading)
            },
            crate::thunk!{
                (env) => block::Paragraph::parser(env).map(Block::Paragraph)
            },
            crate::thunk!{
                (env) => block::Blockquote::parser(env).map(Block::Blockquote)
            },
            crate::thunk!{
                (env) => block::List::parser(env).map(Block::List)
            },
            crate::thunk!{
                (env) => block::FencedCodeBlock::parser(env).map(Block::FencedCodeBlock)
            },
            crate::thunk!{
                (env) => block::HorizontalRule::parser(env).map(Block::HorizontalRule)
            },
            crate::thunk!{
                (env) => block::Table::parser(env).map(Block::Table)
            },
        ])
    }
    pub fn some(env: Environment) -> VecParser<Self> {
        let settings = SequenceSettings::default().allow_empty(false);
        Self::parser(env).sequence(settings)
    }
    pub fn many(env: Environment) -> VecParser<Self> {
        let settings = SequenceSettings::default().allow_empty(true);
        Self::parser(env).sequence(settings)
    }
}

impl block::Heading {
    pub fn parser(env: Environment) -> Parser<Self> {
        unimplemented!()
    }
}
impl block::Paragraph {
    pub fn parser(env: Environment) -> Parser<Self> {
        unimplemented!()
    }
}
impl block::Blockquote {
    pub fn parser(env: Environment) -> Parser<Self> {
        unimplemented!()
    }
}
impl block::FencedCodeBlock {
    pub fn parser(env: Environment) -> Parser<Self> {
        unimplemented!()
    }
}
impl block::HorizontalRule {
    pub fn parser(env: Environment) -> Parser<Self> {
        unimplemented!()
    }
}
impl block::List {
    pub fn parser(env: Environment) -> Parser<Self> {
        unimplemented!()
    }
}
impl block::Table {
    pub fn parser(env: Environment) -> Parser<Self> {
        unimplemented!()
    }
}
impl block::table::SeperatorRow {
    pub fn parser(env: Environment) -> Parser<Self> {
        unimplemented!()
    }
}
impl block::table::Row {
    pub fn parser(env: Environment) -> Parser<Self> {
        unimplemented!()
    }
}
