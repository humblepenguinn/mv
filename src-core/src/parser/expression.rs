use super::{ast, Parser};
use crate::error::{Error::ParserError, Result};
use crate::lexer::token::{Token, TokenKind};

impl<'input, I> Parser<'input, I>
where
    I: Iterator<Item = Token>,
{
    pub(crate) fn parse_expression(&mut self) -> Result<ast::Expr> {
        let line_number = self.tokens.peek().map_or(0, |token| token.get_line_number(&self.input));

        let column_number =
            self.tokens.peek().map_or(0, |token| token.get_column_number(&self.input));

        match self.peek() {
            lit @ TokenKind::Char
            | lit @ TokenKind::Float
            | lit @ TokenKind::Int
            | lit @ TokenKind::Bool => {
                let literal_text = {
                    let literal_token = self.next().unwrap();
                    self.text(literal_token)
                };

                let lit = match lit {
                    TokenKind::Int => {
                        let literal: i64 = match literal_text.parse() {
                            Ok(literal) => literal,
                            Err(_) => {
                                return Err(ParserError(
                                    format!("invalid integer literal: `{}`", literal_text),
                                    line_number,
                                    column_number,
                                ));
                            }
                        };

                        ast::Lit::Int(literal)
                    }

                    TokenKind::Float => {
                        let literal: f64 = match literal_text.parse() {
                            Ok(literal) => literal,
                            Err(_) => {
                                return Err(ParserError(
                                    format!("invalid float literal: `{}`", literal_text),
                                    line_number,
                                    column_number,
                                ));
                            }
                        };

                        ast::Lit::Float(literal)
                    }

                    TokenKind::Bool => {
                        let literal: bool = match literal_text.parse() {
                            Ok(literal) => literal,
                            Err(_) => {
                                return Err(ParserError(
                                    format!("invalid boolean literal: `{}`", literal_text),
                                    line_number,
                                    column_number,
                                ));
                            }
                        };

                        ast::Lit::Bool(literal)
                    }

                    TokenKind::Char => {
                        let literal = match literal_text.chars().nth(1) {
                            Some(literal) => literal,
                            None => {
                                return Err(ParserError(
                                    format!("invalid char literal: `{}`", literal_text),
                                    line_number,
                                    column_number,
                                ));
                            }
                        };

                        ast::Lit::Char(literal)
                    }
                    _ => unreachable!(),
                };

                Ok(ast::Expr::Literal(lit))
            }

            TokenKind::Identifier => {
                let ident_text = {
                    let ident_token = self.next().unwrap();
                    self.text(ident_token)
                };

                Ok(ast::Expr::Ident(ident_text.to_string()))
            }

            op @ TokenKind::Reference => {
                self.consume(op)?;

                Ok(ast::Expr::AddressOf(Box::new(self.parse_expression()?)))
            }

            TokenKind::Asterisk => {
                self.consume(TokenKind::Asterisk)?;
                Ok(ast::Expr::Dereference(Box::new(self.parse_expression()?)))
            }

            _ => Err(ParserError(
                format!("Expected expression but found `{}`", self.peek()),
                line_number,
                column_number,
            )),
        }
    }
}
