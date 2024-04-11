use std::collections::HashSet;

use crate::data::Text;
use crate::data::FatChar;
use crate::formats::common::*;

#[derive(Debug, Clone)]
pub enum Markdown {
    Block(Block),
    Inline(Inline),
}

#[derive(Debug, Clone)]
pub enum Inline {
    PlainText(inline::PlainText),
    Link(inline::Link),
    Image(inline::Image),
    Emphasis(inline::Emphasis),
    Highlight(inline::Highlight),
    Strikethrough(inline::Strikethrough),
    Subscript(inline::Subscript),
    Superscript(inline::Superscript),
    InlineCode(inline::InlineCode),
    Latex(inline::Latex),
    Raw(Text)
}

#[derive(Debug, Clone)]
pub struct InlineSequence(pub Vec<Inline>);

impl Inline {
    pub const RESERVED_TOKENS: &'static [char] = &[
        '[',
        ']',
        '(',
        ')',
        '*',
        '_',
        '=',
        '~',
        '`',
    ];
    pub fn reserved_tokens() -> HashSet<char> {
        HashSet::from_iter(Self::RESERVED_TOKENS.iter().map(|x| *x))
    }
}

pub mod inline {
    use super::*;
    #[derive(Debug, Clone)]
    pub struct PlainText {
        pub value: Text,
    }
    #[derive(Debug, Clone)]
    pub struct Link {
        pub text: InSquareBrackets<Vec<Inline>>,
        pub url: InRoundBrackets<Url>,
    }
    #[derive(Debug, Clone)]
    pub struct Url {
        pub destination: Text,
        pub title: Option<InDoubleQuotes<Text>>,
    }
    #[derive(Debug, Clone)]
    pub struct Image {
        pub bang: FatChar,
        pub link: Link,
    }
    #[derive(Debug, Clone)]
    pub struct Emphasis {
        /// Could be `*` or `_`, up to three repeating characters of such.
        pub start_delimiter: Token,
        pub content: Vec<Inline>,
        pub end_delimiter: Token,
    }
    #[derive(Debug, Clone)]
    pub struct Highlight {
        /// Assuming `==` for start
        pub start_delimiter: Token,
        pub content: Vec<Inline>,
        /// Assuming `==` for end
        pub end_delimiter: Token,
    }
    #[derive(Debug, Clone)]
    pub struct Strikethrough {
        /// Assuming `~~` for start
        pub start_delimiter: Token,
        pub content: Vec<Inline>,
        /// Assuming `~~` for end
        pub end_delimiter: Token,
    }
    #[derive(Debug, Clone)]
    pub struct Subscript {
        /// Assuming `~` for start
        pub start_delimiter: Token,
        pub content: Vec<Inline>,
        /// Assuming `~` for end
        pub end_delimiter: Token,
    }
    #[derive(Debug, Clone)]
    pub struct Superscript {
        /// Assuming `^` for start
        pub start_delimiter: Token,
        pub content: Vec<Inline>,
        /// Assuming `^` for end
        pub end_delimiter: Token,
    }
    #[derive(Debug, Clone)]
    pub struct InlineCode {
        /// One or more backticks.
        pub start_delimiter: Token,
        pub content: Text,
        /// One or more backticks; matching the `start_delimiter`.
        pub end_delimiter: Token,
    }
    #[derive(Debug, Clone)]
    pub struct Latex {
        /// Either a single dollar sign (`$`) or double dollar signs (`$$`).
        pub start_delimiter: Token,
        /// The Tex/LaTeX literal content.
        pub content: Text,
        /// Either a single dollar sign (`$`) or double dollar signs (`$$`) that matches the start delimiter.
        pub end_delimiter: Token,
    }
}

#[derive(Debug, Clone)]
pub enum Block {
    Heading(block::Heading),
    Paragraph(block::Paragraph),
    Blockquote(block::Blockquote),
    List(block::List),
    FencedCodeBlock(block::FencedCodeBlock),
    HorizontalRule(block::HorizontalRule),
    Table(block::Table),
    Newline(FatChar),
}

pub mod block {
    use super::*;
    #[derive(Debug, Clone)]
    pub struct Heading {
        /// Markdown allows for 1-6 `#` characters for headings
        pub hash_tokens: Token,
        pub content: Vec<Inline>,
    }
    #[derive(Debug, Clone)]
    pub struct Paragraph {
        /// A paragraph can contain multiple text elements
        pub content: Vec<Inline>,
    }
    #[derive(Debug, Clone)]
    pub struct Blockquote {
        /// The `>` character used to denote blockquotes
        pub start_delimiters: Vec<Token>,
        /// Blockquotes can contain multiple other Markdown elements
        pub content: Vec<Markdown>,
    }
    #[derive(Debug, Clone)]
    pub struct FencedCodeBlock {
        pub fence_start: Token,
        /// Optional language identifier for syntax highlighting
        pub info_string: Option<Text>,
        /// The actual code content
        pub content: Text,
        pub fence_end: Token,
    }
    #[derive(Debug, Clone)]
    pub struct HorizontalRule {
        /// The characters used to create a horizontal rule, e.g., `---`, `***`, `___`
        pub tokens: Token,
    }
    #[derive(Debug, Clone)]
    pub struct Table {
        pub header: table::Header,
        pub data: Vec<table::Row>,
    }
    #[derive(Debug, Clone)]
    pub enum List {
        Unordered { items: Vec<list::UnorderedItem> } ,
        Ordered { items: Vec<list::OrderedItem> } ,
        Task { items: Vec<list::TaskItem> } ,
    }

    pub mod list {
        use super::*;

        #[derive(Debug, Clone)]
        pub struct UnorderedItem {
            /// Either `*`, `-`, `+`, or a number followed by `.`
            pub bullet: FatChar,
            pub content: Vec<Markdown>,
        }
        #[derive(Debug, Clone)]
        pub struct OrderedItem {
            pub number: Token,
            pub dot: FatChar,
            pub content: Vec<Markdown>,
        }
        #[derive(Debug, Clone)]
        pub struct TaskItem {
            pub bullet: FatChar,
            /// Represents the `[ ]` or `[x]` for task list items
            pub header: InSquareBrackets<Option<Token>>,
            /// Task list items can contain multiple other Markdown elements
            pub content: Vec<Markdown>
        }
    }
    pub mod table {
        use super::*;
        #[derive(Debug, Clone)]
        pub struct Header {
            pub header: Row,
            pub separator: SeperatorRow,
        }
        #[derive(Debug, Clone)]
        pub struct SeperatorRow {
            /// Optionally, a table row might start with a delimiter if the table format specifies it.
            pub start_delimiter: Option<FatChar>,
            /// The cells within the row.
            pub columns: Vec<SeperatorRowCell>,
        }
        #[derive(Debug, Clone)]
        pub struct SeperatorRowCell {
            pub start_colon: Option<FatChar>,
            pub dashes: Token,
            pub end_colon: Option<FatChar>,
            pub end_delimiter: Option<FatChar>,
        }
        #[derive(Debug, Clone)]
        pub struct Row {
            /// Optionally, a table row might start with a delimiter if the table format specifies it.
            pub start_delimiter: Option<FatChar>,
            /// The cells within the row.
            pub cells: Vec<RowCell>,
        }
        #[derive(Debug, Clone)]
        pub struct RowCell {
            /// Content of the cell. This could include inline formatting, links, etc.
            pub content: InlineSequence,
            /// Delimiter token to separate this cell from the next. This could be considered optional,
            /// as the last cell in a row might not have a trailing delimiter in some Markdown formats.
            pub pipe_delimiter: Option<FatChar>,
        }
    }
}
