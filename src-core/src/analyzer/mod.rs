//! # Analyzer
//! Responsible for analyzing the parsed source code and generating a visualization of the stack and the heap

mod heap_allocator;
mod helpers;
mod random_heap_allocator;
mod r#type;

use async_trait::async_trait;
use heap_allocator::HeapBlock;
use helpers::{validate_pointer_assignment, validate_variable_assignment};
use indexmap::IndexMap;
use serde::Serialize;

use self::random_heap_allocator::HeapAllocator;
use self::r#type::Type;
use crate::{
    error::{Error::AnalyzerError, Result},
    parser::ast::{self, Statement},
};

/// Represents the type of memory allocation for a symbol.
///
/// The `AllocationType` enum categorizes memory allocation into three types:
///
/// - `Stack`: Represents memory allocated on the stack.
/// - `Heap`: Represents memory allocated on the heap.
/// - `None`: Represents no allocation or undefined allocation type.
#[derive(Debug, Clone, Serialize, PartialEq)]
pub enum AllocationType {
    Stack,
    Heap,
    Dangling,
    Null,
}

/// Represents different types of symbols used in the language.
///
/// The `Symbol` enum defines three main categories of symbols:
///
/// - **Variable**:
///   - `vtype`: Type of the variable.
///   - `name`: Variable's name.
///   - `value`: Optional value of the variable.
///   - `size`: Size of the variable.
///
/// - **Pointer**:
///   - `ptype`: Type of the pointer.
///   - `name`: Pointer's name.
///   - `value`: Optional value, pointing to another `Symbol` (if applicable).
///   - `heap_pointer`: Optional pointer to a location in the heap.
///   - `allocation_type`: Type of memory allocation (e.g., `Stack`, `Heap`).
///   - `pointer_size`: Size of the pointer.
///   - `value_size`: Size of the value pointed to.
///
/// - **Literal**:
///   - `value`: The literal's value as a string.
///
/// This enum is used to manage and categorize symbols in various contexts such as variable declarations,
/// pointer management, and literal values.
#[derive(Debug, Clone, Serialize, PartialEq)]
pub enum Symbol {
    Variable {
        vtype: Type,
        name: String,
        value: Option<String>,
        size: usize,
    },

    Pointer {
        ptype: Type,
        name: String,
        value: Option<Box<Symbol>>,
        heap_pointer: Option<usize>,
        allocation_type: AllocationType,
        pointer_size: usize,
        value_size: usize,
    },

    Literal {
        value: String,
    },
}

#[async_trait]
pub trait AnalyzerState {
    async fn get_starting_pointers(&mut self) -> IndexMap<String, usize>;
    async fn set_starting_pointers(&mut self, pointers: IndexMap<String, usize>);
}

#[derive(Default)]
pub struct Analyzer {}

impl Analyzer {
    /// Analyzes statements produced by the parser and generates a visualization of the stack and heap.
    ///
    /// This function processes a vector of statements to generate a visual representation of the stack and heap.
    /// It uses the application state to determine the heap size, which is adjusted based on the user's screen size.
    ///
    /// # Arguments
    ///
    /// - `state`: A reference to the application state, which provides information on heap size.
    /// - `statements`: A vector of statements to be analyzed.
    ///
    /// # Returns
    ///
    /// - `Result<(Vec<Symbol>, Vec<HeapBlock>), Error>`: A result containing either:
    ///   - A tuple with:
    ///     - `Vec<Symbol>`: A vector of symbols representing the stack and heap data.
    ///     - `Vec<HeapBlock>`: A vector of heap blocks representing memory allocations.
    ///
    ///   Or:
    ///   - An `Error` if the analysis fails.
    ///
    /// There are two versions of this function, one for the WASM target and one for the Tauri target
    pub async fn analyze_statements<S: AnalyzerState>(
        &self,
        statements: Vec<Statement>,
        state: &mut S,
    ) -> Result<(Vec<Symbol>, Vec<HeapBlock>)> {
        let mut starting_pointers = state.get_starting_pointers().await;

        let mut stack_symbols: IndexMap<String, Symbol> = IndexMap::new();
        let mut allocator = HeapAllocator::new_infinite(20, 2.0, None);

        for statement in statements {
            self.analyze_statement(
                statement,
                &mut stack_symbols,
                &mut allocator,
                &mut starting_pointers,
            )?;
        }

        let stack_symbols_vec: Vec<Symbol> = stack_symbols.into_iter().map(|(_, v)| v).collect();

        self.clean_starting_pointers(&mut starting_pointers, &stack_symbols_vec);

        state.set_starting_pointers(starting_pointers.clone()).await;

        Ok((stack_symbols_vec, allocator.get_heap()))
    }

    /// Cleans up the starting pointers by removing any pointers that are not in the stack symbols vector.
    ///
    /// # Arguments
    ///
    /// - `starting_pointers`: A mutable reference to a `IndexMap<String, usize>` containing starting pointers.
    /// - `stack_symbols_vec`: A reference to a vector of `Symbol` representing the stack symbols.
    ///
    /// # Returns
    ///
    /// - Nothing
    fn clean_starting_pointers(
        &self,
        starting_pointers: &mut IndexMap<String, usize>,
        stack_symbols_vec: &Vec<Symbol>,
    ) {
        for entry in starting_pointers.clone() {
            if !stack_symbols_vec.iter().any(|symbol| {
                let symbol_name = match symbol {
                    Symbol::Variable { name, .. } => name,
                    Symbol::Pointer { name, .. } => name,
                    _ => &String::new(),
                };

                symbol_name == &entry.0
            }) {
                starting_pointers.shift_remove_entry(&entry.0);
            }
        }
    }

    /// Analyzes a single statement and updates the stack symbols and heap allocator accordingly.
    ///
    /// # Arguments
    ///
    /// - `statement`: The statement to be analyzed.
    /// - `stack_symbols`: A mutable reference to a `IndexMap<String, Symbol>` containing stack symbols.
    /// - `allocator`: A mutable reference to a `HeapAllocator` instance.
    /// - `starting_pointers`: A mutable reference to a `IndexMap<String, usize>` containing starting pointers.
    ///
    /// # Returns
    ///
    /// - `Result<(), Error>`: A result containing either:
    ///  - `Ok(())` if the analysis is successful.
    /// - An `Error` if the analysis fails.
    fn analyze_statement(
        &self,
        statement: Statement,
        stack_symbols: &mut IndexMap<String, Symbol>,
        allocator: &mut HeapAllocator,
        starting_pointers: &mut IndexMap<String, usize>,
    ) -> Result<()> {
        match statement {
            ast::Statement::VariableDeclaration {
                var_type,
                var_name,
                value,
                line,
                var_ident_column,
            } => {
                let value = validate_variable_assignment(
                    value,
                    &var_name,
                    &Type::from_token(var_type)?,
                    &stack_symbols,
                    line,
                    var_ident_column,
                )?;

                if stack_symbols.contains_key(&var_name) {
                    return Err(AnalyzerError(
                        format!("Variable `{}` already declared!", var_name),
                        line,
                        var_ident_column,
                    ));
                }

                let vtype = Type::from_token(var_type)?;
                stack_symbols.insert(
                    var_name.clone(),
                    Symbol::Variable {
                        vtype,
                        name: var_name,
                        value,
                        size: vtype.get_size(),
                    },
                );
            }

            ast::Statement::VariableDeclarationWithoutAssignment {
                var_type,
                var_name,
                line,
                var_ident_column,
            } => {
                if stack_symbols.contains_key(&var_name) {
                    return Err(AnalyzerError(
                        format!("Variable `{}` already declared!", var_name),
                        line,
                        var_ident_column,
                    ));
                }

                let vtype = Type::from_token(var_type)?;
                stack_symbols.insert(
                    var_name.clone(),
                    Symbol::Variable {
                        vtype,
                        name: var_name,
                        value: None,
                        size: vtype.get_size(),
                    },
                );
            }

            ast::Statement::VariableAssignment {
                var_name,
                new_value,
                line,
                var_ident_column,
                assignment_column,
            } => {
                let cloned_symbols = stack_symbols.clone();
                if let Some(symbol) = stack_symbols.get_mut(&var_name) {
                    if let Symbol::Variable { value, vtype, .. } = symbol {
                        let new_value = validate_variable_assignment(
                            new_value,
                            &var_name,
                            vtype,
                            &cloned_symbols,
                            line,
                            var_ident_column,
                        )?;
                        *value = new_value;
                    } else {
                        return Err(AnalyzerError(
                            format!(
                                "Invalid use case of assignment operator for symbol `{}`",
                                var_name
                            ),
                            line,
                            assignment_column,
                        ));
                    }
                } else {
                    return Err(AnalyzerError(
                        format!("Variable `{}` not found!", var_name),
                        line,
                        var_ident_column,
                    ));
                }
            }

            ast::Statement::PointerDeclaration {
                base_type,
                pointer_name,
                value,
                line,
                pointer_ident_column,
            } => {
                if stack_symbols.contains_key(&pointer_name) {
                    return Err(AnalyzerError(
                        format!("Pointer `{}` already declared!", pointer_name),
                        line,
                        pointer_ident_column,
                    ));
                }

                let value =
                    validate_pointer_assignment(value, &stack_symbols, line, pointer_ident_column)?;

                let ptype = Type::from_token(base_type)?;

                stack_symbols.insert(
                    pointer_name.clone(),
                    Symbol::Pointer {
                        ptype: Type::from_token(base_type)?,
                        name: pointer_name,
                        value,
                        allocation_type: AllocationType::Stack,
                        heap_pointer: None,
                        pointer_size: 4,
                        value_size: ptype.get_size(),
                    },
                );
            }

            ast::Statement::PointerDeclarationHeap {
                base_type,
                pointer_name,
                line,
                pointer_ident_column,
            } => {
                if stack_symbols.contains_key(&pointer_name) {
                    return Err(AnalyzerError(
                        format!("Pointer `{}` already declared!", &pointer_name),
                        line,
                        pointer_ident_column,
                    ));
                }

                let ptype = Type::from_token(base_type)?;

                let res = allocator.allocate_and_write(
                    &pointer_name,
                    ptype.get_size(),
                    starting_pointers,
                );

                if let Err(e) = res {
                    return Err(AnalyzerError(e.to_string(), line, pointer_ident_column));
                }

                stack_symbols.insert(
                    pointer_name.clone(),
                    Symbol::Pointer {
                        ptype,
                        name: pointer_name,
                        value: Some(Box::new(Symbol::Literal {
                            value: ptype.get_garbage_value(),
                        })),
                        heap_pointer: Some(res.unwrap()),
                        allocation_type: AllocationType::Heap,
                        pointer_size: 4,
                        value_size: ptype.get_size(),
                    },
                );
            }

            ast::Statement::PointerDeclarationNull {
                base_type,
                pointer_name,
                line,
                pointer_ident_column,
            } => {
                if stack_symbols.contains_key(&pointer_name) {
                    return Err(AnalyzerError(
                        format!("Pointer `{}` already declared!", &pointer_name),
                        line,
                        pointer_ident_column,
                    ));
                }

                let ptype = Type::from_token(base_type)?;

                stack_symbols.insert(
                    pointer_name.clone(),
                    Symbol::Pointer {
                        ptype,
                        name: pointer_name,
                        value: None,
                        allocation_type: AllocationType::Null,
                        heap_pointer: None,
                        pointer_size: 4,
                        value_size: ptype.get_size(),
                    },
                );
            }

            ast::Statement::PointerAssignment {
                pointer_name,
                new_value,
                line,
                pointer_ident_column,
            } => {
                let new_value = validate_pointer_assignment(
                    new_value,
                    &stack_symbols,
                    line,
                    pointer_ident_column,
                )?;

                if let Some(symbol) = stack_symbols.get_mut(&pointer_name) {
                    if let Symbol::Pointer {
                        name,
                        value,
                        allocation_type,
                        heap_pointer,
                        value_size,
                        ..
                    } = symbol
                    {
                        if *allocation_type != AllocationType::Dangling {
                            if let Some(heap_pointer) = heap_pointer {
                                allocator.leak(*heap_pointer, *value_size);
                            }
                        } else {
                            if let Some(heap_pointer) = heap_pointer {
                                allocator
                                    .remove_dangling_pointer(*heap_pointer, name.to_string())?;
                            }
                        }

                        *value = new_value;
                        *allocation_type = AllocationType::Stack;
                        *heap_pointer = None;
                    } else {
                        return Err(AnalyzerError(
                            format!(
                                "Invalid use case of assignment operator for symbol `{}`",
                                pointer_name
                            ),
                            line,
                            pointer_ident_column,
                        ));
                    }
                } else {
                    return Err(AnalyzerError(
                        format!("Pointer `{}` not found!", pointer_name),
                        line,
                        pointer_ident_column,
                    ));
                }
            }

            ast::Statement::PointerAssignmentHeap {
                pointer_name,
                new_type,
                line,
                pointer_ident_column,
                new_type_column,
            } => {
                if let Some(symbol) = stack_symbols.get_mut(&pointer_name) {
                    if let Symbol::Pointer {
                        ptype,
                        name,
                        value,
                        allocation_type,
                        heap_pointer,
                        value_size,
                        ..
                    } = symbol
                    {
                        if !ptype.is_type(new_type) {
                            return Err(AnalyzerError(
                                format!(
                                    "Cannot assign `{}` to pointer `{}` (incorrect type)",
                                    &new_type, &pointer_name
                                ),
                                line,
                                new_type_column,
                            ));
                        }

                        if *allocation_type != AllocationType::Dangling {
                            if let Some(heap_pointer) = heap_pointer {
                                allocator.leak(*heap_pointer, *value_size);
                            }
                        } else {
                            if let Some(heap_pointer) = heap_pointer {
                                allocator
                                    .remove_dangling_pointer(*heap_pointer, name.to_string())?;
                            }
                        }

                        let res = allocator.allocate_and_write(
                            &pointer_name,
                            *value_size,
                            starting_pointers,
                        );

                        if let Err(e) = res {
                            return Err(AnalyzerError(e.to_string(), line, pointer_ident_column));
                        }

                        *allocation_type = AllocationType::Heap;
                        *value = Some(Box::new(Symbol::Literal {
                            value: "".to_owned(),
                        }));
                        *heap_pointer = Some(res.unwrap());
                    }
                } else {
                    return Err(AnalyzerError(
                        format!("Pointer `{}` not found!", pointer_name),
                        line,
                        pointer_ident_column,
                    ));
                }
            }

            ast::Statement::PointerAssignmentNull {
                pointer_name,
                line,
                pointer_ident_column,
            } => {
                if let Some(symbol) = stack_symbols.get_mut(&pointer_name) {
                    if let Symbol::Pointer {
                        name,
                        value,
                        allocation_type,
                        heap_pointer,
                        value_size,
                        ..
                    } = symbol
                    {
                        if *allocation_type != AllocationType::Dangling {
                            if let Some(heap_pointer) = heap_pointer {
                                allocator.leak(*heap_pointer, *value_size);
                            }
                        } else {
                            if let Some(heap_pointer) = heap_pointer {
                                allocator
                                    .remove_dangling_pointer(*heap_pointer, name.to_string())?;
                            }
                        }

                        *value = None;
                        *allocation_type = AllocationType::Null;
                        *heap_pointer = None;
                    }
                } else {
                    return Err(AnalyzerError(
                        format!("Pointer `{}` not found!", pointer_name),
                        line,
                        pointer_ident_column,
                    ));
                }
            }

            ast::Statement::Deref {
                pointer_name,
                new_value,
                line,
                pointer_ident_column,
                new_value_column,
            } => {
                let cloned_symbols = stack_symbols.clone();

                if let Some(symbol) = stack_symbols.get_mut(&pointer_name) {
                    if let Symbol::Pointer {
                        value,
                        ptype,
                        allocation_type,
                        heap_pointer,
                        ..
                    } = symbol
                    {
                        let pointer_value = value;
                        let allocation_type = match *allocation_type {
                            AllocationType::Dangling => AllocationType::Heap,
                            _ => (*allocation_type).clone(),
                        };

                        if allocation_type == AllocationType::Null {
                            return Err(AnalyzerError(
                                format!("Cannot dereference null pointer `{}`", pointer_name),
                                line,
                                pointer_ident_column,
                            ));
                        }

                        match *new_value {
                            ast::Expr::Ident(new_ident) => {
                                if let Some(symbol) = cloned_symbols.get(&new_ident) {
                                    if let Symbol::Variable { value, .. } = symbol {
                                        if let Some(value) = value {
                                            if ptype
                                                .is_correct_literal(&ast::Lit::from_str(&value)?)
                                            {
                                                let new_value = value.to_string();
                                                let old_symbol = (*pointer_value).clone();

                                                if allocation_type == AllocationType::Heap {
                                                    *pointer_value =
                                                        Some(Box::new(Symbol::Literal {
                                                            value: new_value.clone(),
                                                        }));

                                                    if let Some(heap_pointer) = heap_pointer {
                                                        allocator.update_metadata(
                                                            *heap_pointer,
                                                            new_value.clone(),
                                                        )?;
                                                    } else {
                                                        return Err(AnalyzerError(
                                                            format!(
                                                                "Heap pointer not found for `{}`",
                                                                pointer_name
                                                            ),
                                                            line,
                                                            pointer_ident_column,
                                                        ));
                                                    }

                                                    return Ok(());
                                                }

                                                if let Some(old_symbol) = old_symbol {
                                                    if let Symbol::Variable { name, .. } =
                                                        *old_symbol
                                                    {
                                                        if let Some(symbol) =
                                                            stack_symbols.get_mut(&name)
                                                        {
                                                            if let Symbol::Variable {
                                                                value, ..
                                                            } = symbol
                                                            {
                                                                *value = Some(new_value);
                                                            }
                                                        }
                                                    }
                                                }
                                            } else {
                                                return Err(AnalyzerError(
                                                    format!(
                                                        "Cannot assign `{}` to pointer `{}` (incorrect type)",
                                                        value, pointer_name
                                                    ),
                                                    line,
                                                    new_value_column,
                                                ));
                                            }
                                        } else {
                                            return Err(AnalyzerError(
                                                format!(
                                                    "Variable `{}` not initialized!",
                                                    new_ident
                                                ),
                                                line,
                                                new_value_column,
                                            ));
                                        }
                                    } else {
                                        return Err(AnalyzerError(
                                            format!("Can only assign variables to pointers!",),
                                            line,
                                            new_value_column,
                                        ));
                                    }
                                } else {
                                    return Err(AnalyzerError(
                                        format!("Variable `{}` not found!", new_ident),
                                        line,
                                        new_value_column,
                                    ));
                                }
                            }
                            ast::Expr::Literal(lit) => {
                                if ptype.is_correct_literal(&lit) {
                                    let old_symbol = (*pointer_value).clone();

                                    if allocation_type == AllocationType::Heap {
                                        *pointer_value = Some(Box::new(Symbol::Literal {
                                            value: lit.to_string(),
                                        }));

                                        if let Some(heap_pointer) = heap_pointer {
                                            allocator
                                                .update_metadata(*heap_pointer, lit.to_string())?;
                                        } else {
                                            return Err(AnalyzerError(
                                                format!(
                                                    "Heap pointer not found for `{}`",
                                                    pointer_name
                                                ),
                                                line,
                                                pointer_ident_column,
                                            ));
                                        }

                                        return Ok(()); // continue;
                                    }

                                    if let Some(old_symbol) = old_symbol {
                                        if let Symbol::Variable { name, .. } = *old_symbol {
                                            if let Some(symbol) = stack_symbols.get_mut(&name) {
                                                if let Symbol::Variable { value, .. } = symbol {
                                                    *value = Some(lit.to_string());
                                                }
                                            }
                                        }
                                    }
                                } else {
                                    return Err(AnalyzerError(
                                        format!(
                                            "Cannot assign `{}` to pointer `{}` (incorrect type)",
                                            lit, pointer_name
                                        ),
                                        line,
                                        new_value_column,
                                    ));
                                }
                            }
                            _ => {}
                        };
                    }
                } else {
                    return Err(AnalyzerError(
                        format!("Pointer `{}` not found!", pointer_name),
                        line,
                        pointer_ident_column,
                    ));
                }
            }

            Statement::Delete {
                pointer_name,
                line,
                pointer_ident_column,
            } => {
                if let Some(symbol) = stack_symbols.get_mut(&pointer_name) {
                    if let Symbol::Pointer {
                        heap_pointer,
                        value_size,
                        allocation_type,
                        ..
                    } = symbol
                    {
                        if *allocation_type == AllocationType::Stack {
                            return Err(AnalyzerError(
                                format!("Cannot delete stack pointer `{}`", pointer_name),
                                line,
                                pointer_ident_column,
                            ));
                        }

                        if *allocation_type == AllocationType::Null {
                            return Err(AnalyzerError(
                                format!("Cannot delete null pointer `{}`", pointer_name),
                                line,
                                pointer_ident_column,
                            ));
                        }

                        if *allocation_type == AllocationType::Dangling {
                            return Err(AnalyzerError(
                                format!("Cannot delete dangling pointer `{}`", pointer_name),
                                line,
                                pointer_ident_column,
                            ));
                        }

                        *allocation_type = AllocationType::Dangling;

                        if let Some(heap_pointer) = heap_pointer {
                            allocator.free(*heap_pointer, *value_size);
                            allocator.insert_dangling_pointer(*heap_pointer, pointer_name)?;
                        }
                    }
                } else {
                    return Err(AnalyzerError(
                        format!("Pointer `{}` not found!", pointer_name),
                        line,
                        pointer_ident_column,
                    ));
                }
            }
        }

        Ok(())
    }
}
