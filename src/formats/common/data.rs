use crate::data::{FatChar, Text};

pub type Token = Text;

#[derive(Debug, Clone)]
pub struct InDoubleQuotes<Content> {
    pub start_delimiter: FatChar,
    pub content: Content,
    pub end_delimiter: FatChar,
}
#[derive(Debug, Clone)]
pub struct InSingleQuotes<Content> {
    pub start_delimiter: FatChar,
    pub content: Content,
    pub end_delimiter: FatChar,
}
#[derive(Debug, Clone)]
pub struct InSquareBrackets<Content> {
    pub open_delimiter: FatChar,
    pub content: Content,
    pub close_delimiter: FatChar,
}
#[derive(Debug, Clone)]
pub struct InRoundBrackets<Content> {
    pub open_delimiter: FatChar,
    pub content: Content,
    pub close_delimiter: FatChar,
}

pub mod token {
    pub mod bracket {
        // - OPEN -
        pub struct OpenRoundBracket;
        pub struct OpenSquareBracket;
        pub struct OpenCurlyBracket;
        pub struct OpenAngleBracket;
        // - CLOSE -
        pub struct CloseRoundBracket;
        pub struct CloseSquareBracket;
        pub struct CloseCurlyBracket;
        pub struct CloseAngleBracket;
    }
}

impl token::bracket::OpenRoundBracket {
    pub const TOKEN: char = '(';
}
impl token::bracket::OpenSquareBracket {
    pub const TOKEN: char = '[';
}
impl token::bracket::OpenCurlyBracket {
    pub const TOKEN: char = '{';
}
impl token::bracket::OpenAngleBracket {
    pub const TOKEN: char = '<';
}
impl token::bracket::CloseRoundBracket {
    pub const TOKEN: char = ')';
}
impl token::bracket::CloseSquareBracket {
    pub const TOKEN: char = ']';
}
impl token::bracket::CloseCurlyBracket {
    pub const TOKEN: char = '}';
}
impl token::bracket::CloseAngleBracket {
    pub const TOKEN: char = '>';
}

impl From<token::bracket::OpenRoundBracket> for char {
    fn from(value: token::bracket::OpenRoundBracket) -> Self {
        token::bracket::OpenRoundBracket::TOKEN
    }
}
impl From<token::bracket::OpenSquareBracket> for char {
    fn from(value: token::bracket::OpenSquareBracket) -> Self {
        token::bracket::OpenSquareBracket::TOKEN
    }
}
impl From<token::bracket::OpenCurlyBracket> for char {
    fn from(value: token::bracket::OpenCurlyBracket) -> Self {
        token::bracket::OpenCurlyBracket::TOKEN
    }
}
impl From<token::bracket::OpenAngleBracket> for char {
    fn from(value: token::bracket::OpenAngleBracket) -> Self {
        token::bracket::OpenAngleBracket::TOKEN
    }
}
impl From<token::bracket::CloseRoundBracket> for char {
    fn from(value: token::bracket::CloseRoundBracket) -> Self {
        token::bracket::CloseRoundBracket::TOKEN
    }
}
impl From<token::bracket::CloseSquareBracket> for char {
    fn from(value: token::bracket::CloseSquareBracket) -> Self {
        token::bracket::CloseSquareBracket::TOKEN
    }
}
impl From<token::bracket::CloseCurlyBracket> for char {
    fn from(value: token::bracket::CloseCurlyBracket) -> Self {
        token::bracket::CloseCurlyBracket::TOKEN
    }
}
impl From<token::bracket::CloseAngleBracket> for char {
    fn from(value: token::bracket::CloseAngleBracket) -> Self {
        token::bracket::CloseAngleBracket::TOKEN
    }
}

impl std::fmt::Display for token::bracket::OpenRoundBracket {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", Self::TOKEN)
    }
}
impl std::fmt::Display for token::bracket::OpenSquareBracket {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", Self::TOKEN)
    }
}
impl std::fmt::Display for token::bracket::OpenCurlyBracket {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", Self::TOKEN)
    }
}
impl std::fmt::Display for token::bracket::OpenAngleBracket {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", Self::TOKEN)
    }
}
impl std::fmt::Display for token::bracket::CloseRoundBracket {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", Self::TOKEN)
    }
}
impl std::fmt::Display for token::bracket::CloseSquareBracket {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", Self::TOKEN)
    }
}
impl std::fmt::Display for token::bracket::CloseCurlyBracket {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", Self::TOKEN)
    }
}
impl std::fmt::Display for token::bracket::CloseAngleBracket {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", Self::TOKEN)
    }
}
