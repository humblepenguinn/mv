pub(crate) mod rule;
pub(crate) mod token;

use rule::{get_rules, unambiguous_single_char, Rule};
use token::{Span, Token, TokenKind};

pub(crate) struct Lexer<'input> {
    input: &'input str,
    cursor: u32,
    eof: bool,
    rules: Vec<Rule>,
}

impl<'input> Lexer<'input> {
    pub(crate) fn new(input: &'input str) -> Self {
        Self {
            input,
            cursor: 0,
            eof: false,
            rules: get_rules(),
        }
    }

    pub(crate) fn next_token(&mut self, input: &str) -> Token {
        self.valid_token(input).unwrap_or_else(|| self.invalid_token(input))
    }

    fn valid_token(&mut self, input: &str) -> Option<Token> {
        let next = input.chars().next().unwrap();

        let (len, kind) = if next.is_whitespace() {
            (
                input
                    .char_indices()
                    .take_while(|(_, c)| c.is_whitespace())
                    .last()
                    .unwrap() // we know there is at least one whitespace character
                    .0 as u32
                    + 1,
                TokenKind::Whitespace,
            )
        } else if let Some(kind) = unambiguous_single_char(next) {
            (1, kind)
        } else {
            self.rules
                .iter()
                .rev()
                .filter_map(|rule| Some(((rule.matches)(input)?, rule.kind)))
                .max_by_key(|&(len, _)| len)?
        };

        let start = self.cursor;
        self.cursor += len as u32;

        Some(Token {
            kind,
            span: Span {
                start,
                end: len as u32,
            },
        })
    }

    fn invalid_token(&mut self, input: &str) -> Token {
        let start = self.cursor;

        let len = input
            .char_indices()
            .find(|(pos, _)| self.valid_token(&input[*pos..]).is_some())
            .map(|(pos, _)| pos)
            .unwrap_or_else(|| input.len());

        let len = len as u32;
        self.cursor = start + len;

        Token {
            kind: TokenKind::Error,
            span: Span { start, end: len },
        }
    }
}

impl<'input> Iterator for Lexer<'input> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        if self.cursor as usize >= self.input.len() {
            if self.eof {
                return None;
            }

            self.eof = true;

            Some(Token {
                kind: TokenKind::EOF,
                span: Span {
                    start: self.cursor,
                    end: 0,
                },
            })
        } else {
            Some(self.next_token(&self.input[self.cursor as usize..]))
        }
    }
}
