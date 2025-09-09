use std::{
    fmt,
    ops::{Index, Range},
};

use serde::{Deserialize, Serialize};

// Kw -> Keyword
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Hash, Copy, Clone)]
pub enum TokenKind {
    KwInt,
    KwFloat,
    KwChar,
    KwDouble,
    KwBool,

    Reference,
    Asterisk,
    New,
    Delete,
    Null,

    Eq,
    Underscore,
    SemiColon,

    Bool,
    Float,
    Char,
    Int,
    Identifier,

    Comment,
    Error,
    Whitespace,
    EOF,
}

impl fmt::Display for TokenKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TokenKind::KwInt => write!(f, "int"),
            TokenKind::KwFloat => write!(f, "float"),
            TokenKind::KwChar => write!(f, "char"),
            TokenKind::KwDouble => write!(f, "double"),
            TokenKind::KwBool => write!(f, "bool"),
            TokenKind::Reference => write!(f, "&"),
            TokenKind::Asterisk => write!(f, "*"),
            TokenKind::New => write!(f, "new"),
            TokenKind::Delete => write!(f, "delete"),
            TokenKind::Null => write!(f, "null"),
            TokenKind::Eq => write!(f, "="),
            TokenKind::Underscore => write!(f, "_"),
            TokenKind::SemiColon => write!(f, ";"),
            TokenKind::Comment => write!(f, "comment"),
            TokenKind::Int => write!(f, "int"),
            TokenKind::Float => write!(f, "float"),
            TokenKind::Bool => write!(f, "bool"),
            TokenKind::Char => write!(f, "char"),
            TokenKind::Identifier => write!(f, "identifier"),
            TokenKind::Error => write!(f, "error"),
            TokenKind::Whitespace => write!(f, "whitespace"),
            TokenKind::EOF => write!(f, "EOF"),
        }
    }
}

#[derive(Eq, PartialEq, Clone, Copy, Hash, Default, Debug, Serialize)]
pub(crate) struct Span {
    pub(crate) start: u32,
    pub(crate) end: u32,
}

impl Span {
    fn get_line_number(&self, input: &str) -> usize {
        input[..(self.start + self.end) as usize].lines().count()
    }

    fn get_column_number(&self, input: &str) -> usize {
        let mut column = 0;

        for c in input[..(self.start + self.end) as usize].chars() {
            if c == '\n' {
                column = 0;
            } else {
                column += 1;
            }
        }

        column
    }
}

impl From<Span> for Range<usize> {
    fn from(span: Span) -> Self {
        span.start as usize..span.end as usize
    }
}

impl From<Range<usize>> for Span {
    fn from(range: Range<usize>) -> Self {
        Self {
            start: range.start as u32,
            end: range.end as u32,
        }
    }
}

impl Index<Span> for str {
    type Output = str;

    fn index(&self, index: Span) -> &Self::Output {
        let end = (index.start + index.end) as usize;
        &self[index.start as usize..end]
    }
}

#[derive(PartialEq, Eq, Clone, Copy, Hash, Serialize)]
pub struct Token {
    pub(crate) kind: TokenKind,
    pub(crate) span: Span,
}

impl Token {
    #[allow(dead_code)]
    pub(crate) fn len(&self) -> usize {
        (self.span.end - self.span.start) as usize
    }

    pub(crate) fn text<'input>(&self, input: &'input str) -> &'input str {
        &input[self.span]
    }

    pub(crate) fn get_column_number(&self, input: &str) -> usize {
        self.span.get_column_number(input)
    }

    pub(crate) fn get_line_number(&self, input: &str) -> usize {
        self.span.get_line_number(input)
    }
}

impl fmt::Debug for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?} - <{}, {}>", self.kind, self.span.start, self.span.end)
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.kind)
    }
}
