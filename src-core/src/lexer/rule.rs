use lazy_static::lazy_static;
use regex::Regex;

use super::token::TokenKind;

pub(crate) struct Rule {
    pub(crate) kind: TokenKind,
    pub(crate) matches: fn(&str) -> Option<u32>,
}

fn match_keyword(input: &str, keyword: &str) -> Option<u32> {
    input.starts_with(keyword).then(|| keyword.len() as u32)
}

fn match_regex(input: &str, r: &Regex) -> Option<u32> {
    r.find(input).map(|regex_match| regex_match.end() as u32)
}

lazy_static! {
    static ref FLOAT_REGEX: Regex =
        Regex::new(r#"^((\d+(\.\d+)?)|(\.\d+))([Ee](\+|-)?\d+)?"#).unwrap();
    static ref BOOL_REGEX: Regex = Regex::new(r#"^(true|false)"#).unwrap();
    static ref COMMENT_REGEX: Regex = Regex::new(r#"^//[^\n]*\n"#).unwrap();
    static ref IDENTIFIER_REGEX: Regex = Regex::new(r##"^([A-Za-z]|_)([A-Za-z]|_|\d)*"##).unwrap();
}

pub(crate) fn get_rules() -> Vec<Rule> {
    vec![
        Rule {
            kind: TokenKind::KwInt,
            matches: |input| match_keyword(input, "int"),
        },
        Rule {
            kind: TokenKind::KwFloat,
            matches: |input| match_keyword(input, "float"),
        },
        Rule {
            kind: TokenKind::KwDouble,
            matches: |input| match_keyword(input, "double"),
        },
        Rule {
            kind: TokenKind::KwChar,
            matches: |input| match_keyword(input, "char"),
        },
        Rule {
            kind: TokenKind::KwBool,
            matches: |input| match_keyword(input, "bool"),
        },
        Rule {
            kind: TokenKind::New,
            matches: |input| match_keyword(input, "new"),
        },
        Rule {
            kind: TokenKind::Delete,
            matches: |input| match_keyword(input, "delete"),
        },
        Rule {
            kind: TokenKind::Null,
            matches: |input| match_keyword(input, "nullptr"),
        },
        Rule {
            kind: TokenKind::Comment,
            matches: move |input| match_regex(input, &COMMENT_REGEX),
        },
        Rule {
            kind: TokenKind::Int,
            matches: |input| {
                input
                    .char_indices()
                    .take_while(|(_, c)| c.is_ascii_digit())
                    .last()
                    .map(|(pos, _)| pos as u32 + 1)
            },
        },
        Rule {
            kind: TokenKind::Float,
            matches: |input| match_regex(input, &FLOAT_REGEX),
        },
        Rule {
            kind: TokenKind::Bool,
            matches: |input| match_regex(input, &BOOL_REGEX),
        },
        Rule {
            kind: TokenKind::Char,
            matches: |input| {
                if input.starts_with('\'') {
                    input[1..].char_indices().nth(1).map(|(pos, _)| pos as u32 + 2)
                } else {
                    None
                }
            },
        },
        Rule {
            kind: TokenKind::Identifier,
            matches: |input| match_regex(input, &IDENTIFIER_REGEX),
        },
    ]
}

pub(crate) fn unambiguous_single_char(c: char) -> Option<TokenKind> {
    Some(match c {
        '=' => TokenKind::Eq,
        '_' => TokenKind::Underscore,
        ';' => TokenKind::SemiColon,
        '&' => TokenKind::Reference,
        '*' => TokenKind::Asterisk,
        _ => return None,
    })
}
