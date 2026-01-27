use crate::{
    ast::{
        ast::{Expr, Expression, Program, Statement, Stmt},
        type_annotation::TypeAnnotation,
    },
    error_handler::{error_collector::ErrorCollector, errors::VynError},
    tokens::TokenType,
    type_checker::symbol_type_table::SymbolTypeTable,
    utils::throw_error,
};
use core::fmt;
use std::mem;

#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    Integer,
    Float,
    Bool,
    String,
    Nil,
    Identifier,
    Array(Box<Type>, usize),
    Sequence(Box<Type>),
}

impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Type::Integer => write!(f, "Int"),
            Type::Float => write!(f, "Float"),
            Type::Bool => write!(f, "Bool"),
            Type::String => write!(f, "String"),
            Type::Nil => write!(f, "Nil"),
            Type::Identifier => write!(f, "Identifier"),
            Type::Array(t, s) => {
                write!(f, "[{}]{}", s, t)
            }
            Type::Sequence(t) => {
                write!(f, "[]{}", t)
            }
        }
    }
}

impl Type {
    pub fn from_anotated_type(an_type: &TypeAnnotation) -> Self {
        match an_type {
            TypeAnnotation::StringType => Self::String,
            TypeAnnotation::IntegerType => Self::Integer,
            TypeAnnotation::FloatType => Self::Float,
            TypeAnnotation::BooleanType => Self::Bool,
            TypeAnnotation::ArrayType(ta, size_expr) => {
                let t = Self::from_anotated_type(ta.as_ref());
                let size = Self::evaluate_const_expr(size_expr)
                    .and_then(|v| if v >= 0 { Some(v as usize) } else { None })
                    .unwrap_or(0);
                Type::Array(Box::new(t), size)
            }
            TypeAnnotation::SequenceType(ta) => {
                let t = Type::from_anotated_type(ta);
                Type::Sequence(Box::new(t))
            }
        }
    }

    /// Evaluates a constant expression to an i64 value
    /// Returns None if the expression cannot be evaluated at compile time
    fn evaluate_const_expr(expr: &Expression) -> Option<i32> {
        match &expr.node {
            Expr::IntegerLiteral(n) => Some(*n),

            Expr::Unary { operator, right } => {
                let right_val = Self::evaluate_const_expr(right)?;

                match operator.get_token_type() {
                    TokenType::Minus => Some(-right_val),
                    TokenType::Plus => Some(right_val),
                    TokenType::Bang => Some(if right_val == 0 { 1 } else { 0 }),
                    _ => None,
                }
            }

            Expr::BinaryOperation {
                left,
                operator,
                right,
            } => {
                let left_val = Self::evaluate_const_expr(left)?;
                let right_val = Self::evaluate_const_expr(right)?;

                match operator.get_token_type() {
                    TokenType::Plus => Some(left_val.checked_add(right_val)?),
                    TokenType::Minus => Some(left_val.checked_sub(right_val)?),
                    TokenType::Asterisk => Some(left_val.checked_mul(right_val)?),
                    TokenType::Slash => {
                        if right_val != 0 {
                            Some(left_val.checked_div(right_val)?)
                        } else {
                            None
                        }
                    }
                    TokenType::Caret => {
                        if right_val < 0 {
                            return None;
                        }
                        Some(left_val.checked_pow(right_val as u32)?)
                    }

                    _ => None,
                }
            }

            _ => None,
        }
    }
}

pub struct TypeChecker {
    symbol_type_table: SymbolTypeTable,
    errors: ErrorCollector,
}

impl TypeChecker {
    pub fn new() -> Self {
        Self {
            symbol_type_table: SymbolTypeTable::new(),
            errors: ErrorCollector::new(),
        }
    }

    /// Main entry point
    pub fn check_program(&mut self, program: &Program) -> Result<(), ErrorCollector> {
        for stmt in &program.statements {
            let _ = self.check_statement(stmt);
        }

        if self.errors.has_errors() {
            Err(mem::take(&mut self.errors))
        } else {
            Ok(())
        }
    }

    pub(crate) fn check_statement(&mut self, stmt: &Statement) -> Result<(), ()> {
        let span = stmt.span;

        match &stmt.node {
            Stmt::Expression { expression } => {
                self.check_expression(expression, None)?;
                Ok(())
            }

            Stmt::VariableDeclaration {
                identifier,
                value,
                annotated_type,
                mutable,
            } => {
                let expected_type = Type::from_anotated_type(annotated_type);
                let value_type = self.check_expression(value, Some(expected_type.clone()))?;

                let var_name = match &identifier.node {
                    Expr::Identifier(name) => name.clone(),
                    _ => unreachable!("Variable name must be an identifier"),
                };

                self.symbol_type_table.declare_identifier(
                    var_name,
                    expected_type.clone(),
                    span,
                    *mutable,
                    &mut self.errors,
                )?;

                if expected_type != value_type {
                    self.throw_error(VynError::DeclarationTypeMismatch {
                        expected: expected_type.clone(),
                        got: value_type,
                        span,
                    });
                    return Err(());
                }

                Ok(())
            }

            Stmt::TypeAliasDeclaration { identifier, value } => {
                let name = match &identifier.node {
                    Expr::Identifier(n) => n.clone(),
                    _ => unreachable!("Type alias identifier must be an identifier"),
                };

                let resolved_type = Type::from_anotated_type(value);

                if let Err(err) =
                    self.symbol_type_table
                        .enroll_type_alias(name, resolved_type, span)
                {
                    self.throw_error(err);
                    return Err(());
                }

                Ok(())
            }

            Stmt::Block { statements } => {
                let parent_table =
                    mem::replace(&mut self.symbol_type_table, SymbolTypeTable::new());

                self.symbol_type_table = parent_table.enter_scope();

                for stmt in statements {
                    let _ = self.check_statement(stmt);
                }

                self.symbol_type_table =
                    mem::replace(&mut self.symbol_type_table, SymbolTypeTable::new()).exit_scope();

                Ok(())
            }

            Stmt::IfDeclaration {
                condition,
                consequence,
                alternate,
            } => {
                let condition_type = self.check_expression(condition, None)?;

                // Verify condition is boolean
                if condition_type != Type::Bool {
                    self.throw_error(VynError::TypeMismatch {
                        expected: vec![Type::Bool],
                        found: condition_type,
                        span: condition.span,
                    });
                    return Err(());
                }

                self.check_statement(&consequence)?;

                if let Some(alt) = alternate.as_ref() {
                    self.check_statement(alt)?;
                }
                Ok(())
            }

            Stmt::StdoutLog { log_value } => {
                self.check_expression(log_value, None)?;
                Ok(())
            }

            _ => throw_error(&format!("unknown ast:\n\n{:#?}", stmt.node), 1),
        }
    }

    pub(crate) fn check_expression(
        &mut self,
        expr: &Expression,
        expected_type: Option<Type>,
    ) -> Result<Type, ()> {
        let span = expr.span;

        match &expr.node {
            Expr::IntegerLiteral(_) => Ok(Type::Integer),
            Expr::FloatLiteral(_) => Ok(Type::Float),
            Expr::BooleanLiteral(_) => Ok(Type::Bool),
            Expr::StringLiteral(_) => Ok(Type::String),
            Expr::NilLiteral => Ok(Type::Nil),

            Expr::ArrayLiteral { elements } => {
                if elements.is_empty() && expected_type.is_none() {
                    self.throw_error(VynError::TypeInfer {
                        expr: expr.node.clone(),
                        span,
                    });
                    return Err(());
                }

                // Handle empty arrays with expected type
                if elements.is_empty() {
                    return Ok(expected_type.unwrap_or(Type::Sequence(Box::new(Type::Nil))));
                }

                let exp_type = expected_type.clone().unwrap_or_else(|| {
                    // Infer type from first element if no expected type
                    Type::Sequence(Box::new(Type::Nil))
                });

                match &exp_type {
                    Type::Array(expected_element_type, size) => {
                        if elements.len() != *size {
                            self.throw_error(VynError::ArrayLengthMismatch {
                                expected: *size,
                                got: elements.len(),
                                span,
                            });
                            return Err(());
                        }

                        // Check each element against expected element type
                        for element in elements {
                            let actual_element_type = self.check_expression(
                                element,
                                Some((**expected_element_type).clone()),
                            )?;

                            if **expected_element_type != actual_element_type {
                                self.throw_error(VynError::TypeMismatch {
                                    expected: vec![(**expected_element_type).clone()],
                                    found: actual_element_type,
                                    span: element.span,
                                });
                                return Err(());
                            }
                        }

                        Ok(exp_type)
                    }
                    Type::Sequence(expected_element_type) => {
                        // Check each element against expected element type
                        for element in elements {
                            let actual_element_type = self.check_expression(
                                element,
                                Some((**expected_element_type).clone()),
                            )?;

                            if **expected_element_type != actual_element_type {
                                self.throw_error(VynError::TypeMismatch {
                                    expected: vec![(**expected_element_type).clone()],
                                    found: actual_element_type,
                                    span: element.span,
                                });
                                return Err(());
                            }
                        }

                        Ok(exp_type)
                    }
                    _ => {
                        self.throw_error(VynError::TypeMismatch {
                            expected: vec![exp_type],
                            found: Type::Sequence(Box::new(Type::Nil)),
                            span,
                        });
                        Err(())
                    }
                }
            }

            Expr::Identifier(name) => {
                let ident =
                    self.symbol_type_table
                        .resolve_identifier(name, span, &mut self.errors)?;

                Ok(ident.symbol_type.clone())
            }

            Expr::Unary { operator, right } => self.check_unary(operator, right, span),

            Expr::Index { target, property } => {
                let target_type = self.check_expression(target.as_ref(), None)?;
                let property_type = self.check_expression(property.as_ref(), None)?;

                // Property must be an integer
                if property_type != Type::Integer {
                    self.throw_error(VynError::TypeMismatch {
                        expected: vec![Type::Integer],
                        found: property_type,
                        span: property.span,
                    });
                    return Err(());
                }

                match target_type.clone() {
                    Type::Array(element_type, size) => {
                        // Check bounds if index is constant
                        if let Some(idx) = Type::evaluate_const_expr(property.as_ref()) {
                            if idx < 0 || idx >= size as i32 {
                                self.throw_error(VynError::IndexOutOfBounds {
                                    size,
                                    idx: idx as i64,
                                    span: property.span,
                                });
                                return Err(());
                            }
                        }

                        Ok(*element_type)
                    }
                    Type::Sequence(element_type) => Ok(*element_type),

                    _ => {
                        self.throw_error(VynError::InvalidIndexing {
                            target: target_type,
                            span,
                        });
                        Err(())
                    }
                }
            }

            Expr::IndexAssignment {
                target,
                property,
                new_value,
            } => {
                let target_type = self.check_expression(target, None)?;
                let property_type = self.check_expression(property, None)?;

                // Property must be an integer (index)
                if property_type != Type::Integer {
                    self.throw_error(VynError::TypeMismatch {
                        expected: vec![Type::Integer],
                        found: property_type,
                        span: property.span,
                    });
                    return Err(());
                }

                match target_type {
                    Type::Array(element_type, size) => {
                        // Check bounds if index is constant
                        if let Some(idx) = Type::evaluate_const_expr(property.as_ref()) {
                            if idx < 0 || idx >= size as i32 {
                                self.throw_error(VynError::IndexOutOfBounds {
                                    size,
                                    idx: idx as i64,
                                    span: property.span,
                                });
                                return Err(());
                            }
                        }

                        let new_value_type =
                            self.check_expression(new_value, Some((*element_type).clone()))?;

                        if *element_type != new_value_type {
                            self.throw_error(VynError::TypeMismatch {
                                expected: vec![*element_type.clone()],
                                found: new_value_type,
                                span: new_value.span,
                            });
                            return Err(());
                        }

                        Ok(*element_type)
                    }

                    Type::Sequence(element_type) => {
                        let new_value_type =
                            self.check_expression(new_value, Some((*element_type).clone()))?;

                        if *element_type != new_value_type {
                            self.throw_error(VynError::TypeMismatch {
                                expected: vec![*element_type.clone()],
                                found: new_value_type,
                                span: new_value.span,
                            });
                            return Err(());
                        }

                        Ok(*element_type)
                    }

                    _ => {
                        self.throw_error(VynError::InvalidIndexing {
                            target: target_type,
                            span,
                        });
                        Err(())
                    }
                }
            }

            Expr::BinaryOperation {
                left,
                operator,
                right,
            } => self.check_binary_expr(operator, left, right, span),

            Expr::VariableAssignment {
                identifier,
                new_value,
            } => {
                let ident_name = match &identifier.node {
                    Expr::Identifier(n) => n.clone(),
                    _ => {
                        self.throw_error(VynError::LeftHandAssignment { span });
                        return Err(());
                    }
                };

                let ident_symbol = self.symbol_type_table.resolve_identifier(
                    &ident_name,
                    span,
                    &mut self.errors,
                )?;

                let is_mutable = ident_symbol.mutable;
                let ident_span = ident_symbol.span;
                let expected_type = ident_symbol.symbol_type.clone();

                if !is_mutable {
                    self.throw_error(VynError::ImmutableMutation {
                        identifier: ident_name,
                        span: ident_span,
                        mutation_span: span,
                    });
                    return Err(());
                }

                let value_type = self.check_expression(new_value, Some(expected_type.clone()))?;

                if expected_type != value_type {
                    self.throw_error(VynError::TypeMismatch {
                        expected: vec![expected_type],
                        found: value_type,
                        span: new_value.span,
                    });
                    return Err(());
                }

                Ok(value_type)
            }

            _ => {
                self.throw_error(VynError::TypeInfer {
                    expr: expr.node.clone(),
                    span,
                });
                Err(())
            }
        }
    }

    pub(crate) fn throw_error(&mut self, error: VynError) {
        self.errors.add(error);
    }
}
