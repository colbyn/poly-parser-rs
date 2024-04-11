use std::fmt::Debug;

use itertools::Itertools;

use crate::{combinators::SequenceSettings, data::{CharParser, Parser, Text, TextParser, TupleParser, VecParser}, formats::{InRoundBrackets, InSquareBrackets}, system::{Lazy, Thunk}};

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
        Parser::<Self>::options_(vec![
            inline::PlainText::parser(env.clone()).map(Inline::PlainText),
            // inline::Link::parser(env.clone()).map(Inline::Link),
            // inline::Image::parser(env.clone()).map(Inline::Image),
            // inline::Emphasis::parser(env.clone()).map(Inline::Emphasis),
            // inline::Highlight::parser(env.clone()).map(Inline::Highlight),
            // inline::Strikethrough::parser(env.clone()).map(Inline::Strikethrough),
            // inline::Subscript::parser(env.clone()).map(Inline::Subscript),
            // inline::Superscript::parser(env.clone()).map(Inline::Superscript),
            // inline::InlineCode::parser(env.clone()).map(Inline::InlineCode),
            // inline::Latex::parser(env.clone()).map(Inline::Latex),
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
        let reserved_tokens = Inline::reserved_tokens();
        let settings = SequenceSettings::default()
            .allow_empty(false);
        CharParser::char_if(move |x| !reserved_tokens.contains(&x))
            .sequence(settings)
            .map(|x| Text::from_iter(x))
    }
}
impl InlineSequence {
    pub fn some(env: Environment) -> Parser<Self> {
        let settings = SequenceSettings::default().allow_empty(false);
        Inline::parser(env).sequence(settings).map(InlineSequence)
    }
    pub fn many(env: Environment) -> Parser<Self> {
        let settings = SequenceSettings::default().allow_empty(true);
        Inline::parser(env).sequence(settings).map(InlineSequence)
    }
    pub fn some_unless<End: Debug>(env: Environment, terminator: Parser<End>) -> TupleParser<Self, Option<End>> where End: Clone + 'static {
        Inline::parser(env).some_unless_(terminator)
            .map(|(l, r)| (InlineSequence(l), r))
    }
    pub fn many_unless<End: Debug>(env: Environment, terminator: Parser<End>) -> TupleParser<Self, Option<End>> where End: Clone + 'static {
        Inline::parser(env).many_unless_(terminator)
            .map(|(l, r)| (InlineSequence(l), r))
    }
    pub fn some_till<End: Debug>(env: Environment, terminator: Parser<End>) -> TupleParser<Self, End> where End: Clone + 'static {
        Inline::parser(env).some_till_(terminator)
            .map(|(l, r)| (InlineSequence(l), r))
    }
    pub fn many_till<End: Debug>(env: Environment, terminator: Parser<End>) -> TupleParser<Self, End> where End: Clone + 'static {
        Inline::parser(env).many_till_(terminator)
            .map(|(l, r)| (InlineSequence(l), r))
    }
}
impl inline::PlainText {
    pub fn parser(env: Environment) -> Parser<Self> {
        Inline::plain_text(env).map(|x| inline::PlainText {value: x})
    }
}
impl inline::Link {
    pub fn parser(env: Environment) -> Parser<Self> {
        let text = InSquareBrackets::parser(crate::thunk!{
            (env) => Inline::many(env)
        });
        let url = InRoundBrackets::parser(crate::thunk!{
            (env) => inline::Url::parser(env)
        });
        return text
            .and(Thunk::constant(url))
            .map(|(x, y)| {
                Self { text: x, url: y }
            })
    }
}
impl inline::Url {
    pub fn parser(env: Environment) -> Parser<Self> {
        Inline::plain_text(env).map(|x| {
            Self { destination: x, title: None }
        })
    }
}
impl inline::Image {
    pub fn parser(env: Environment) -> Parser<Self> {
        CharParser::char('!')
            .and(crate::thunk!{
                (env) => inline::Link::parser(env)
            })
            .map(|(x, y)| {
                Self { bang: x, link: y }
            })
    }
}
impl inline::Emphasis {
    pub fn parser(env: Environment) -> Parser<Self> {
        // let pack = |token: &'static str| -> Parser<Self> {
        //     TextParser::token(token)
        //         .and(crate::thunk!{
        //             (env) => InlineSequence::many_till(env, TextParser::token(token))
        //         })
        //         .map(|(l, (c, r))| {
        //             Self { start_delimiter: l, content: c.0, end_delimiter: r }
        //         })
        // };
        // Parser::options_(vec![
        //     pack("***"),
        //     pack("**"),
        //     pack("*"),
        //     pack("___"),
        //     pack("__"),
        //     pack("_"),
        // ])
        // TextParser::token("*")
        //     .and_(Inline::plain_text(env).map(|x| vec![Inline::Raw(x)]).and_(TextParser::token("*")))
        //     .map(|(l, (c, r))| {
        //         Self { start_delimiter: l, content: c, end_delimiter: r }
        //     })
        // let main = 
        //     Inline::plain_text(env)
        //         .map(|x| inline::PlainText {value: x})
        //         .map(Inline::PlainText);
        TextParser::token("*")
            .and2(
                crate::thunk!{
                    (env) => Inline::plain_text(env)
                        .map(|x| inline::PlainText {value: x})
                        .map(Inline::PlainText)
                },
                crate::thunk! {
                    TextParser::token("*")
                }
            )
            .map(|(l, c, r)| Self { start_delimiter: l, content: vec![c], end_delimiter: r })
    }
}
impl inline::Highlight {
    pub fn parser(env: Environment) -> Parser<Self> {
        Inline::many(env.clone())
            .between_both_ends(TextParser::token("=="))
            .map(|(l, c, r)| Self { start_delimiter: l, content: c, end_delimiter: r })
    }
}
impl inline::Strikethrough {
    pub fn parser(env: Environment) -> Parser<Self> {
        Inline::many(env.clone())
            .between_both_ends(TextParser::token("~~"))
            .map(|(l, c, r)| Self { start_delimiter: l, content: c, end_delimiter: r })
    }
}
impl inline::Subscript {
    pub fn parser(env: Environment) -> Parser<Self> {
        Inline::many(env.clone())
            .between_both_ends(TextParser::token("~"))
            .map(|(l, c, r)| Self { start_delimiter: l, content: c, end_delimiter: r })
    }
}
impl inline::Superscript {
    pub fn parser(env: Environment) -> Parser<Self> {
        Inline::many(env.clone())
            .between_both_ends(TextParser::token("~"))
            .map(|(l, c, r)| Self { start_delimiter: l, content: c, end_delimiter: r })
    }
}
impl inline::InlineCode {
    pub fn parser(env: Environment) -> Parser<Self> {
        let terminator1 = TextParser::token("``");
        let terminator2 = TextParser::token("`");
        let options = Parser::options_(vec![
            terminator1.clone()
                .and(
                    Thunk::wrap(move || {
                        CharParser::next().many_till_(terminator1.clone()).map(|(l, r)| {
                            (Text::from_iter(l), r)
                        })
                    })
                ),
            terminator2.clone()
                .and(
                    Thunk::wrap(move || {
                        CharParser::next().many_till_(terminator2.clone()).map(|(l, r)| {
                            (Text::from_iter(l), r)
                        })
                    })
                ),
        ]);
        return options.map(|(l, (c, r))| {
            Self { start_delimiter: l, content: c, end_delimiter: r }
        })
    }
}
impl inline::Latex {
    pub fn parser(env: Environment) -> Parser<Self> {
        return Parser::fail() // TODO
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
