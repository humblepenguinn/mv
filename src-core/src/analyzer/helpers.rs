//! Helper functions that are used by the analyzer module

use indexmap::IndexMap;

use crate::{
    error::{Error::AnalyzerError, Result},
    parser::ast::{self, Expr},
};

use super::{r#type::Type, Symbol};

/// Validates a variable assignment.
///
/// This function checks if a value can be assigned to a variable based on the variable's type
/// and the current state of the symbol table.
///
/// # Arguments
/// - `value`: A boxed [Expr](crate::parser::ast::Expr) representing the value to be assigned to the variable. This can only be a literal
///   or an identifier.
/// - `var_name`: A string slice representing the name of the variable being assigned to.
/// - `var_type`: A reference to a [Type](crate::analyzer::type::Type) object representing the type of the variable.
/// - `symbols`: A reference to the symbol table
///
/// # Returns
/// - `Result<Option<String>>`: A result containing either:
///   - `Option<String>`: `Some(String)` where `String` is the value assigned to the variable.
///   - [AnalyzerError](crate::error::Error::AnalyzerError): returns an error if the assignment is invalid.
pub(crate) fn validate_variable_assignment(
    value: Box<Expr>,
    var_name: &str,
    var_type: &Type,
    symbols: &IndexMap<String, Symbol>,
    line: usize,
    var_ident_column: usize,
) -> Result<Option<String>> {
    match *value {
        ast::Expr::Literal(lit) => {
            if !var_type.is_correct_literal(&lit) {
                return Err(AnalyzerError(
                    format!("Cannot assign `{}` to variable `{}` (incorrect type)", lit, var_name),
                    line,
                    var_ident_column,
                ));
            }
            Ok(Some(lit.to_string()))
        }
        ast::Expr::Ident(ident_name) => {
            if let Some(symbol) = symbols.get(&ident_name) {
                if let Symbol::Variable { value, .. } = symbol {
                    if let Some(value) = value {
                        return Ok(Some(value.clone()));
                    } else {
                        return Err(AnalyzerError(
                            format!("Variable `{}` not initialized!", ident_name),
                            line,
                            var_ident_column,
                        ));
                    }
                } else {
                    return Err(AnalyzerError(
                        format!("Can only assign variables to variables!",),
                        line,
                        var_ident_column,
                    ));
                }
            } else {
                return Err(AnalyzerError(
                    format!("Variable `{}` not found!", ident_name),
                    line,
                    var_ident_column,
                ));
            }
        }
        expr => Err(AnalyzerError(
            format!("Expected a identifier or literal but found `{}`", expr),
            line,
            var_ident_column,
        )),
    }
}

/// Validates a pointer assignment.
///
/// This function checks if a value can be assigned to a pointer based on the symbol table. The value can
/// either be a literal or a variable.
///
/// # Arguments
/// - `value`: A boxed [Expr](crate::parser::ast::Expr) representing the value to be assigned to the pointer. This can be a literal
///   or an identifier.
/// - `symbols`: A reference to the symbol table
///
/// # Returns
/// - `Result<Option<Box<Symbol>>>`: A result containing either:
///   - `Option<Box<Symbol>>`: `Some(Box<Symbol>)` where `Symbol` is a boxed symbol representing the assigned value
///   - [AnalyzerError](crate::error::Error::AnalyzerError): returns an error if the assignment is invalid
pub(crate) fn validate_pointer_assignment<'a>(
    value: Box<Expr>,
    symbols: &'a IndexMap<String, Symbol>,
    line: usize,
    pointer_ident_column: usize,
) -> Result<Option<Box<Symbol>>> {
    match *value {
        ast::Expr::Literal(lit) => {
            return Ok(Some(Box::new(Symbol::Literal {
                value: lit.to_string(),
            })));
        }

        ast::Expr::Ident(ident_name) => {
            if let Some(symbol) = symbols.get(&ident_name) {
                if let Symbol::Variable { .. } = symbol {
                    return Ok(Some(Box::new(symbol.clone())));
                } else {
                    return Err(AnalyzerError(
                        format!("Pointers can only point to variables or literals!",),
                        line,
                        pointer_ident_column,
                    ));
                }
            } else {
                return Err(AnalyzerError(
                    format!("Variable `{}` not found!", ident_name),
                    line,
                    pointer_ident_column,
                ));
            }
        }
        expr => Err(AnalyzerError(
            format!("Expected a identifier or literal but found `{}`", expr),
            line,
            pointer_ident_column,
        )),
    }
}
