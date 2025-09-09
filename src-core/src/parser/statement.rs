use crate::lexer::token::{Token, TokenKind};

use super::{ast, Parser};

use crate::error::{Error::ParserError, Result};

impl<'input, I> Parser<'input, I>
where
    I: Iterator<Item = Token>,
{
    pub(crate) fn statement(&mut self) -> Result<ast::Statement> {
        let line_number = self.tokens.peek().map_or(0, |token| token.get_line_number(&self.input));

        let column_number =
            self.tokens.peek().map_or(0, |token| token.get_column_number(&self.input));

        match self.peek() {
            var_type @ TokenKind::KwInt
            | var_type @ TokenKind::KwChar
            | var_type @ TokenKind::KwFloat
            | var_type @ TokenKind::KwDouble
            | var_type @ TokenKind::KwBool => {
                self.consume(var_type)?;

                let mut pointer = false;

                if self.peek() == TokenKind::Asterisk {
                    pointer = true;
                    self.consume(TokenKind::Asterisk)?;
                }

                let ident = if let Some(token) = self.next() {
                    token
                } else {
                    return Err(ParserError(
                        "Expected identifier but found none".to_string(),
                        line_number,
                        column_number,
                    ));
                };

                if ident.kind != TokenKind::Identifier {
                    return Err(ParserError(
                        format!("Expected identifier but found `{}`", ident.kind),
                        line_number,
                        column_number,
                    ));
                }

                let name = self.text(ident).to_string();

                if pointer {
                    self.consume(TokenKind::Eq)?;

                    let pointer_ident_column = ident.get_column_number(&self.input);

                    if self.peek() == TokenKind::New {
                        // Heap allocation

                        self.consume(TokenKind::New)?;

                        match self.peek() {
                            TokenKind::KwBool => {
                                self.consume(TokenKind::KwBool)?;
                                if var_type != TokenKind::Bool {
                                    return Err(ParserError(
                                        format!("Expected a pointer to {}", var_type),
                                        line_number,
                                        column_number,
                                    ));
                                }
                            }
                            TokenKind::KwChar => {
                                self.consume(TokenKind::KwChar)?;
                                if var_type != TokenKind::KwChar {
                                    return Err(ParserError(
                                        format!("Expected a pointer to {}", var_type),
                                        line_number,
                                        column_number,
                                    ));
                                }
                            }
                            TokenKind::KwFloat => {
                                self.consume(TokenKind::KwFloat)?;
                                if var_type != TokenKind::KwFloat {
                                    return Err(ParserError(
                                        format!("Expected a pointer to {}", var_type),
                                        line_number,
                                        column_number,
                                    ));
                                }
                            }

                            TokenKind::KwInt => {
                                self.consume(TokenKind::KwInt)?;
                                if var_type != TokenKind::KwInt {
                                    return Err(ParserError(
                                        format!("Expected a pointer to {}", var_type),
                                        line_number,
                                        column_number,
                                    ));
                                }
                            }

                            TokenKind::KwDouble => {
                                self.consume(TokenKind::KwDouble)?;
                                if var_type != TokenKind::KwDouble {
                                    return Err(ParserError(
                                        format!("Expected a pointer to {}", var_type),
                                        line_number,
                                        column_number,
                                    ));
                                }
                            }

                            _ => {
                                return Err(ParserError(
                                    format!(
                                        "Expected type after `new` but found `{}`",
                                        self.peek()
                                    ),
                                    line_number,
                                    column_number,
                                ));
                            }
                        }

                        self.consume(TokenKind::SemiColon)?;

                        return Ok(ast::Statement::PointerDeclarationHeap {
                            base_type: var_type,
                            pointer_name: name,
                            line: line_number,
                            pointer_ident_column,
                        });
                    }

                    if self.peek() == TokenKind::Null {
                        self.consume(TokenKind::Null)?;
                        self.consume(TokenKind::SemiColon)?;

                        return Ok(ast::Statement::PointerDeclarationNull {
                            base_type: var_type,
                            pointer_name: name,
                            line: line_number,
                            pointer_ident_column,
                        });
                    }

                    let expression = self.parse_expression()?;

                    match expression {
                        ast::Expr::AddressOf(inner_expr) => {
                            if let ast::Expr::Ident(ident) = *inner_expr {
                                self.consume(TokenKind::SemiColon)?;

                                return Ok(ast::Statement::PointerDeclaration {
                                    base_type: var_type,
                                    pointer_name: name,
                                    value: Box::new(ast::Expr::Ident(ident)),
                                    line: line_number,
                                    pointer_ident_column,
                                });
                            } else {
                                return Err(ParserError(
                                    "Expected identifier after reference operator".to_string(),
                                    line_number,
                                    column_number,
                                ));
                            }
                        }

                        expression => {
                            return Err(ParserError(
                                format!("Expected reference operator but found `{}`", expression),
                                line_number,
                                column_number,
                            ));
                        }
                    }
                }

                if self.peek() == TokenKind::SemiColon {
                    self.consume(TokenKind::SemiColon)?;
                    return Ok(ast::Statement::VariableDeclarationWithoutAssignment {
                        var_type,
                        var_name: name,
                        line: line_number,
                        var_ident_column: ident.get_column_number(&self.input),
                    });
                }

                self.consume(TokenKind::Eq)?;

                let value = self.parse_expression()?;

                self.consume(TokenKind::SemiColon)?;

                Ok(ast::Statement::VariableDeclaration {
                    var_type,
                    var_name: name,
                    value: Box::new(value),
                    line: line_number,
                    var_ident_column: ident.get_column_number(&self.input),
                })
            }

            TokenKind::Asterisk => {
                self.consume(TokenKind::Asterisk)?;
                let ident = if let Some(token) = self.next() {
                    token
                } else {
                    return Err(ParserError(
                        "Expected identifier after dereference operator but found none".to_string(),
                        line_number,
                        column_number,
                    ));
                };

                let pointer_ident_column = ident.get_column_number(&self.input);

                if ident.kind != TokenKind::Identifier {
                    return Err(ParserError(
                        format!(
                            "Expected identifier after dereference operator `*`, but found `{}`",
                            ident.kind
                        ),
                        line_number,
                        column_number,
                    ));
                }

                let name = self.text(ident).to_string();
                self.consume(TokenKind::Eq)?;

                let new_value_column =
                    self.tokens.peek().map_or(0, |token| token.get_column_number(&self.input));

                let expression = self.parse_expression()?;

                match expression {
                    ast::Expr::Ident(ident) => {
                        self.consume(TokenKind::SemiColon)?;

                        return Ok(ast::Statement::Deref {
                            pointer_name: name,
                            new_value: Box::new(ast::Expr::Ident(ident)),
                            line: line_number,
                            pointer_ident_column,
                            new_value_column,
                        });
                    }

                    ast::Expr::Literal(lit) => {
                        self.consume(TokenKind::SemiColon)?;

                        return Ok(ast::Statement::Deref {
                            pointer_name: name,
                            new_value: Box::new(ast::Expr::Literal(lit)),
                            line: line_number,
                            pointer_ident_column,
                            new_value_column,
                        });
                    }

                    expression => {
                        return Err(ParserError(
                            format!("Expected identifier but found `{}`", expression),
                            line_number,
                            column_number,
                        ));
                    }
                }
            }

            TokenKind::Identifier => {
                let ident = self.next().unwrap();
                let pointer_ident_column = ident.get_column_number(&self.input);

                let name = self.text(ident).to_string();
                let mut assignment_column = 0;

                if self.peek() == TokenKind::Eq {
                    assignment_column =
                        self.tokens.peek().map_or(0, |token| token.get_column_number(&self.input));

                    self.consume(TokenKind::Eq)?;
                }

                if self.peek() == TokenKind::New {
                    // Heap allocation
                    self.consume(TokenKind::New)?;

                    let new_type;
                    let new_type_column =
                        self.tokens.peek().map_or(0, |token| token.get_column_number(&self.input));

                    match self.peek() {
                        TokenKind::KwBool => {
                            self.consume(TokenKind::KwBool)?;
                            new_type = TokenKind::KwBool;
                        }
                        TokenKind::KwChar => {
                            self.consume(TokenKind::KwChar)?;
                            new_type = TokenKind::KwChar;
                        }
                        TokenKind::KwFloat => {
                            self.consume(TokenKind::KwFloat)?;
                            new_type = TokenKind::KwFloat;
                        }
                        TokenKind::KwInt => {
                            self.consume(TokenKind::KwInt)?;
                            new_type = TokenKind::KwInt;
                        }
                        TokenKind::KwDouble => {
                            self.consume(TokenKind::KwDouble)?;
                            new_type = TokenKind::KwDouble;
                        }
                        _ => {
                            return Err(ParserError(
                                format!("Expected type after `new` but found `{}`", self.peek()),
                                line_number,
                                column_number,
                            ));
                        }
                    }

                    self.consume(TokenKind::SemiColon)?;

                    return Ok(ast::Statement::PointerAssignmentHeap {
                        pointer_name: name,
                        new_type,
                        line: line_number,
                        pointer_ident_column,
                        new_type_column,
                    });
                }

                if self.peek() == TokenKind::Null {
                    self.consume(TokenKind::Null)?;
                    self.consume(TokenKind::SemiColon)?;

                    return Ok(ast::Statement::PointerAssignmentNull {
                        pointer_name: name,
                        line: line_number,
                        pointer_ident_column,
                    });
                }

                let expr = self.parse_expression()?;

                if let ast::Expr::AddressOf(inner_expr) = expr {
                    if let ast::Expr::Ident(ident) = *inner_expr {
                        self.consume(TokenKind::SemiColon)?;

                        return Ok(ast::Statement::PointerAssignment {
                            pointer_name: name,
                            new_value: Box::new(ast::Expr::Ident(ident)),
                            line: line_number,
                            pointer_ident_column,
                        });
                    } else {
                        return Err(ParserError(
                            "Expected identifier after reference operator".to_string(),
                            line_number,
                            column_number,
                        ));
                    }
                }

                self.consume(TokenKind::SemiColon)?;

                Ok(ast::Statement::VariableAssignment {
                    var_name: name,
                    new_value: Box::new(expr),
                    line: line_number,
                    var_ident_column: ident.get_column_number(&self.input),
                    assignment_column,
                })
            }

            TokenKind::Delete => {
                self.consume(TokenKind::Delete)?;

                let ident = if let Some(token) = self.next() {
                    token
                } else {
                    return Err(ParserError(
                        "Expected identifier after delete operator but found none".to_string(),
                        line_number,
                        column_number,
                    ));
                };

                if ident.kind != TokenKind::Identifier {
                    return Err(ParserError(
                        format!(
                            "Expected identifier after delete operator `delete`, but found `{}`",
                            ident.kind
                        ),
                        line_number,
                        column_number,
                    ));
                }

                let name = self.text(ident).to_string();
                self.consume(TokenKind::SemiColon)?;

                Ok(ast::Statement::Delete {
                    pointer_name: name,
                    line: line_number,
                    pointer_ident_column: ident.get_column_number(&self.input),
                })
            }
            _ => Err(ParserError(
                format!("Expected statement but found `{}`", self.peek()),
                line_number,
                column_number,
            )),
        }
    }
}
