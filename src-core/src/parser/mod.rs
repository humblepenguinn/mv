pub(crate) mod ast;
pub(crate) mod expression;
pub(crate) mod statement;

use std::iter::Peekable;

use super::error::{Error::ParserError, Result};

use crate::lexer::{
    token::{Token, TokenKind},
    Lexer,
};

pub struct TokenIter<'input> {
    lexer: Lexer<'input>,
}

impl<'input> TokenIter<'input> {
    fn new(input: &'input str) -> Self {
        Self {
            lexer: Lexer::new(input),
        }
    }
}

impl<'input> Iterator for TokenIter<'input> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let next_token = self.lexer.next()?;
            if !matches!(next_token.kind, TokenKind::Whitespace | TokenKind::Comment) {
                return Some(next_token);
            }
        }
    }
}

pub struct Parser<'input, I>
where
    I: Iterator<Item = Token>,
{
    input: &'input str,
    tokens: Peekable<I>,
}

impl<'input> Parser<'input, TokenIter<'input>> {
    pub fn new(input: &'input str) -> Parser<'input, TokenIter<'input>> {
        Parser {
            input,
            tokens: TokenIter::new(input).peekable(),
        }
    }
}

impl<'input, I> Parser<'input, I>
where
    I: Iterator<Item = Token>,
{
    pub fn parse(&mut self) -> Result<Vec<ast::Statement>> {
        let mut statements = Vec::new();

        while self.peek() != TokenKind::EOF {
            statements.push(self.statement()?);
        }

        Ok(statements)
    }

    pub(crate) fn text(&self, token: Token) -> &'input str {
        token.text(&self.input)
    }

    pub(crate) fn peek(&mut self) -> TokenKind {
        self.tokens.peek().map(|token| token.kind).unwrap_or(TokenKind::EOF)
    }

    #[allow(dead_code)]
    pub(crate) fn at(&mut self, kind: TokenKind) -> bool {
        self.peek() == kind
    }

    pub(crate) fn next(&mut self) -> Option<Token> {
        self.tokens.next()
    }

    pub(crate) fn consume(&mut self, expected: TokenKind) -> Result<()> {
        let line_number = self.tokens.peek().map_or(0, |token| token.get_line_number(&self.input));

        let column_number =
            self.tokens.peek().map_or(0, |token| token.get_column_number(&self.input));

        let token = self.next().ok_or_else(|| {
            ParserError(
                format!("Expected to consume `{}`, but found `EOF`", expected),
                line_number,
                column_number,
            )
        })?;

        if token.kind != expected {
            return Err(ParserError(
                format!("Expected to consume `{}`, but found `{}`", expected, token.kind),
                line_number,
                column_number,
            )
            .into());
        }

        Ok(())
    }
}
