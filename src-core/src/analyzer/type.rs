//! This module contains the `Type` enum which is used to represent the different types that are supported by the language
//! We use this instead of the [TokenKind](crate::lexer::token::TokenKind) enum to make the code more readable and easier to work with when checking for types

use serde::Serialize;

use crate::error::Result;
use crate::lexer::token::TokenKind;
use crate::parser::ast;

/// Represents the different types that are supported by the language
#[derive(Debug, Clone, Copy, Serialize, PartialEq)]
pub enum Type {
    Integer,
    Float,
    Char,
    Double,
    Bool,
}

impl Type {
    /// Converts a [TokenKind](crate::lexer::token::TokenKind) to a [Type](crate::analyzer::type::Type)
    ///
    /// # Arguments
    /// - `token_type`: A [TokenKind](crate::lexer::token::TokenKind) representing the type
    ///
    /// # Returns
    /// - [Result](crate::error::Result): A result containing either:
    ///    - [Type](crate::analyzer::type::Type): The converted type
    ///    - [Error](crate::error::Error): An error if the type is invalid
    pub(crate) fn from_token(token_type: TokenKind) -> Result<Type> {
        match token_type {
            TokenKind::KwInt => Ok(Type::Integer),
            TokenKind::KwFloat => Ok(Type::Float),
            TokenKind::KwChar => Ok(Type::Char),
            TokenKind::KwDouble => Ok(Type::Double),
            TokenKind::KwBool => Ok(Type::Bool),
            _ => Err("Invalid Type".into()),
        }
    }

    /// Checks if the current type matches the type associated with a given [TokenKind](crate::lexer::token::TokenKind)
    ///
    /// # Arguments
    /// - `value`: A [TokenKind](crate::lexer::token::TokenKind) representing the type
    ///
    /// # Returns
    /// - `bool`: `true` if the types match, `false` otherwise
    pub(crate) fn is_type(&self, value: TokenKind) -> bool {
        match value {
            TokenKind::KwInt => self == &Type::Integer,
            TokenKind::KwFloat => self == &Type::Float,
            TokenKind::KwChar => self == &Type::Char,
            TokenKind::KwDouble => self == &Type::Double,
            TokenKind::KwBool => self == &Type::Bool,
            _ => false,
        }
    }

    /// Checks if the current type matches the type associated with a given [ast::Lit](crate::parser::ast::Lit)
    ///
    /// # Arguments
    /// - `value`: A [ast::Lit](crate::parser::ast::Lit) representing the literal value
    ///
    /// # Returns
    /// - `bool`: `true` if the types match, `false` otherwise
    pub(crate) fn is_correct_literal(&self, value: &ast::Lit) -> bool {
        match value {
            ast::Lit::Int(_) => self == &Type::Integer,
            ast::Lit::Bool(_) => self == &Type::Bool,
            ast::Lit::Float(_) => self == &Type::Float || self == &Type::Double,
            ast::Lit::Char(_) => self == &Type::Char,
        }
    }

    /// Gets the size of the type in bytes
    ///
    /// # Returns
    /// - `usize`: The size of the type in bytes
    pub(crate) fn get_size(&self) -> usize {
        match self {
            Type::Integer => 4,
            Type::Float => 4,
            Type::Char => 1,
            Type::Double => 8,
            Type::Bool => 1,
        }
    }

    /// Gets the default value for the type
    /// This is used when we declare a heap pointer and we need to initialize it with a default value
    ///
    /// # Returns
    /// - `String`: The default value for the type
    pub(crate) fn get_garbage_value(&self) -> String {
        match self {
            Type::Integer => "0".to_owned(),
            Type::Float => "0.0".to_owned(),
            Type::Char => "'\\0'".to_owned(),
            Type::Double => "0.0".to_owned(),
            Type::Bool => "false".to_owned(),
        }
    }
}
