use serde::{Deserialize, Serialize};
use std::fmt;

use crate::error::{Error, Result};
use crate::lexer::token::TokenKind;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum Expr {
    Literal(Lit),
    Ident(String),
    AddressOf(Box<Expr>),
    Dereference(Box<Expr>),
    PrefixOp {
        op: TokenKind,
        expr: Box<Expr>,
    },
    InfixOp {
        op: TokenKind,
        lhs: Box<Expr>,
        rhs: Box<Expr>,
    },
    PostfixOp {
        op: TokenKind,
        expr: Box<Expr>,
    },
}

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Expr::Literal(lit) => write!(f, "{}", lit),
            Expr::Ident(ident) => write!(f, "{}", ident),
            Expr::AddressOf(expr) => write!(f, "&{}", expr),
            Expr::Dereference(expr) => write!(f, "*{}", expr),
            Expr::PrefixOp { op, expr } => write!(f, "{}{}", op, expr),
            Expr::InfixOp { op, lhs, rhs } => write!(f, "{} {} {}", lhs, op, rhs),
            Expr::PostfixOp { op, expr } => write!(f, "{}{}", expr, op),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum Lit {
    Int(i64),
    Float(f64),
    Char(char),
    Bool(bool),
}

impl Lit {
    pub(crate) fn to_string(&self) -> String {
        match self {
            Lit::Int(i) => i.to_string(),
            Lit::Float(fl) => fl.to_string(),
            Lit::Char(c) => c.to_string(),
            Lit::Bool(b) => b.to_string(),
        }
    }

    pub(crate) fn from_str(s: &str) -> Result<Lit> {
        if let Ok(i) = s.parse::<i64>() {
            return Ok(Lit::Int(i));
        }

        if let Ok(fl) = s.parse::<f64>() {
            return Ok(Lit::Float(fl));
        }

        if s.starts_with('\'') && s.ends_with('\'') {
            return Ok(Lit::Char(s.chars().nth(1).unwrap()));
        }

        if s == "true" || s == "false" {
            return Ok(Lit::Bool(s == "true"));
        }

        Err(Error::ParserError("Invalid literal".to_string(), 0, 0))
    }
}

impl fmt::Display for Lit {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Lit::Int(i) => write!(f, "{}", i),
            Lit::Float(fl) => write!(f, "{}", fl),
            Lit::Char(c) => write!(f, "{}", c),
            Lit::Bool(b) => write!(f, "{}", b),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum Statement {
    VariableDeclaration {
        var_type: TokenKind,
        var_name: String,
        value: Box<Expr>,
        line: usize,
        var_ident_column: usize,
    },

    VariableDeclarationWithoutAssignment {
        var_type: TokenKind,
        var_name: String,
        line: usize,
        var_ident_column: usize,
    },

    VariableAssignment {
        var_name: String,
        new_value: Box<Expr>,
        line: usize,
        var_ident_column: usize,
        assignment_column: usize,
    },

    PointerDeclaration {
        base_type: TokenKind,
        pointer_name: String,
        value: Box<Expr>,
        line: usize,
        pointer_ident_column: usize,
    },

    PointerDeclarationHeap {
        base_type: TokenKind,
        pointer_name: String,
        line: usize,
        pointer_ident_column: usize,
    },

    PointerDeclarationNull {
        base_type: TokenKind,
        pointer_name: String,
        line: usize,
        pointer_ident_column: usize,
    },

    PointerAssignment {
        pointer_name: String,
        new_value: Box<Expr>,
        line: usize,
        pointer_ident_column: usize,
    },

    PointerAssignmentHeap {
        pointer_name: String,
        new_type: TokenKind,
        line: usize,
        pointer_ident_column: usize,
        new_type_column: usize,
    },

    PointerAssignmentNull {
        pointer_name: String,
        line: usize,
        pointer_ident_column: usize,
    },

    Deref {
        pointer_name: String,
        new_value: Box<Expr>,
        line: usize,
        pointer_ident_column: usize,
        new_value_column: usize,
    },

    Delete {
        pointer_name: String,
        line: usize,
        pointer_ident_column: usize,
    },
}
