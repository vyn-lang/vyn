use crate::{
    ast::{
        ast::{Expr, Expression, Program, Statement, Stmt},
        type_annotation::TypeAnnotation,
    },
    error_handler::{error_collector::ErrorCollector, errors::VynError},
    tokens::TokenType,
    type_checker::{static_evaluator::StaticEvaluator, symbol_type_table::SymbolTypeTable},
    utils::{Span, throw_error},
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
    pub fn from_anotated_type(
        an_type: &TypeAnnotation,
        static_eval: &StaticEvaluator,
        errors: &mut ErrorCollector,
    ) -> Self {
        match an_type {
            TypeAnnotation::StringType => Self::String,
            TypeAnnotation::IntegerType => Self::Integer,
            TypeAnnotation::FloatType => Self::Float,
            TypeAnnotation::BooleanType => Self::Bool,
            TypeAnnotation::ArrayType(ta, size_expr) => {
                let t = Self::from_anotated_type(ta.as_ref(), static_eval, errors);

                // Try to evaluate the size expression
                let size = Self::evaluate_array_size(size_expr, static_eval, errors).unwrap_or(0);

                Type::Array(Box::new(t), size)
            }
            TypeAnnotation::SequenceType(ta) => {
                let t = Type::from_anotated_type(ta, static_eval, errors);
                Type::Sequence(Box::new(t))
            }
        }
    }

    pub fn get_type_default_value(t: &Type) -> Expression {
        let expr = match t {
            Self::String => Expr::StringLiteral(String::new()),
            Self::Integer => Expr::IntegerLiteral(0),
            Self::Float => Expr::FloatLiteral(0.0),
            Self::Bool => Expr::BooleanLiteral(false),
            Self::Array(ta, size) => {
                let default_elem = Self::get_type_default_value(ta);

                Expr::ArrayLiteral {
                    elements: vec![Box::new(default_elem); *size],
                }
            }
            Self::Sequence(_) => Expr::ArrayLiteral { elements: vec![] },

            _ => unreachable!(),
        };

        Expression {
            node: expr,
            span: Span::default(),
        }
    }

    /// Evaluate array size expression using the static evaluator
    fn evaluate_array_size(
        expr: &Expression,
        static_eval: &StaticEvaluator,
        errors: &mut ErrorCollector,
    ) -> Option<usize> {
        match &expr.node {
            // Direct integer literal
            Expr::IntegerLiteral(n) => {
                if *n >= 0 {
                    Some(*n as usize)
                } else {
                    errors.add(VynError::NegativeArraySize {
                        size: *n,
                        span: expr.span,
                    });
                    None
                }
            }

            // Identifier reference to a static
            Expr::Identifier(name) => {
                if let Some(n) = static_eval.get_static_int(name) {
                    if n >= 0 {
                        Some(n as usize)
                    } else {
                        errors.add(VynError::NegativeArraySize {
                            size: n,
                            span: expr.span,
                        });
                        None
                    }
                } else {
                    errors.add(VynError::ArraySizeNotStatic { span: expr.span });
                    None
                }
            }

            // For complex expressions, we could try to evaluate them
            // but for now, just report an error
            _ => {
                errors.add(VynError::ArraySizeNotStatic { span: expr.span });
                None
            }
        }
    }
}

pub struct TypeChecker<'a> {
    pub(crate) symbol_type_table: SymbolTypeTable,
    pub(crate) errors: ErrorCollector,
    static_eval: &'a StaticEvaluator,
    loop_depth: usize,
}

impl<'a> TypeChecker<'a> {
    pub fn new(static_eval: &'a StaticEvaluator) -> Self {
        Self {
            symbol_type_table: SymbolTypeTable::new(),
            errors: ErrorCollector::new(),
            static_eval,
            loop_depth: 0,
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
                let expected_type =
                    Type::from_anotated_type(annotated_type, self.static_eval, &mut self.errors);

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

                if let Some(val) = value {
                    let value_type = self.check_expression(val, Some(expected_type.clone()))?;

                    if expected_type != value_type {
                        self.throw_error(VynError::DeclarationTypeMismatch {
                            expected: expected_type.clone(),
                            got: value_type,
                            span,
                        });
                        return Err(());
                    }
                }
                Ok(())
            }

            Stmt::WhenLoop { condition, body } => {
                let condition_type = self.check_expression(condition, Some(Type::Bool))?;

                if condition_type != Type::Bool {
                    self.throw_error(VynError::TypeMismatch {
                        expected: vec![Type::Bool],
                        found: condition_type,
                        span: condition.span,
                    });
                    return Err(());
                }

                self.loop_depth += 1;
                let stmt = self.check_statement(body.as_ref());
                self.loop_depth -= 1;

                stmt
            }

            Stmt::IndexLoop {
                body,
                iterator,
                range,
            } => {
                let iterator_name = match &iterator.node {
                    Expr::Identifier(name) => name.clone(),
                    _ => {
                        self.throw_error(VynError::TypeMismatch {
                            expected: vec![Type::Identifier],
                            found: Type::Identifier, // placeholder
                            span: iterator.span,
                        });
                        return Err(());
                    }
                };

                let range_type = self.check_expression(range, Some(Type::Integer))?;

                if range_type != Type::Integer {
                    self.throw_error(VynError::TypeMismatch {
                        expected: vec![Type::Integer],
                        found: range_type,
                        span: range.span,
                    });
                    return Err(());
                }

                // Enter a new scope for the loop
                let parent_table =
                    mem::replace(&mut self.symbol_type_table, SymbolTypeTable::new());
                self.symbol_type_table = parent_table.enter_scope();

                // Declare the iterator variable as an immutable Integer in the loop scope
                self.symbol_type_table.declare_identifier(
                    iterator_name,
                    Type::Integer,
                    iterator.span,
                    false,
                    &mut self.errors,
                )?;

                self.loop_depth += 1;
                let stmt = self.check_statement(body.as_ref());
                self.loop_depth -= 1;

                // Exit the loop scope
                self.symbol_type_table =
                    mem::replace(&mut self.symbol_type_table, SymbolTypeTable::new()).exit_scope();

                stmt
            }

            Stmt::Loop { body } => {
                self.loop_depth += 1;
                let stmt = self.check_statement(body.as_ref());
                self.loop_depth -= 1;

                stmt
            }

            Stmt::Break => {
                if self.loop_depth <= 0 {
                    self.throw_error(VynError::IllegalLoopInterruptToken {
                        token_type: TokenType::Break,
                        span,
                    });

                    return Err(());
                }

                Ok(())
            }

            Stmt::Continue => {
                if self.loop_depth <= 0 {
                    self.throw_error(VynError::IllegalLoopInterruptToken {
                        token_type: TokenType::Continue,
                        span,
                    });

                    return Err(());
                }

                Ok(())
            }

            Stmt::StaticVariableDeclaration {
                identifier,
                value,
                annotated_type,
            } => {
                let expected_type =
                    Type::from_anotated_type(annotated_type, self.static_eval, &mut self.errors);

                // Static values should already be validated by static_eval
                // Just check the type matches
                let value_type = self.check_expression(value, Some(expected_type.clone()))?;

                let var_name = match &identifier.node {
                    Expr::Identifier(name) => name.clone(),
                    _ => unreachable!("Variable name must be an identifier"),
                };

                self.symbol_type_table.declare_static_identifier(
                    var_name,
                    expected_type.clone(),
                    span,
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

                let resolved_type =
                    Type::from_anotated_type(value, self.static_eval, &mut self.errors);

                if let Err(err) =
                    self.symbol_type_table
                        .enroll_type_alias(name, resolved_type, span)
                {
                    self.throw_error(err);
                    return Err(());
                }

                Ok(())
            }

            Stmt::Scope { statements } => {
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

                let exp_type = if let Some(t) = expected_type {
                    t
                } else {
                    let first_elem_type = self.check_expression(&elements[0], None)?;
                    Type::Sequence(Box::new(first_elem_type))
                };

                match exp_type.clone() {
                    Type::Array(array_type, size) => {
                        if elements.len() != size {
                            self.throw_error(VynError::ArrayLengthMismatch {
                                expected: size,
                                got: elements.len(),
                                span,
                            });
                            return Err(());
                        }

                        for elem in elements {
                            let elem_type =
                                self.check_expression(elem.as_ref(), Some(*array_type.clone()))?;
                            if elem_type != *array_type {
                                self.throw_error(VynError::TypeMismatch {
                                    expected: vec![*array_type.clone()],
                                    found: elem_type,
                                    span: elem.span,
                                });
                                return Err(());
                            }
                        }

                        Ok(Type::Array(array_type, size))
                    }
                    Type::Sequence(seq_type) => {
                        if elements.is_empty() {
                            return Ok(Type::Sequence(seq_type));
                        }

                        for elem in elements {
                            let elem_type =
                                self.check_expression(elem.as_ref(), Some(*seq_type.clone()))?;
                            if elem_type != *seq_type {
                                self.throw_error(VynError::TypeMismatch {
                                    expected: vec![*seq_type.clone()],
                                    found: elem_type,
                                    span: elem.span,
                                });
                                return Err(());
                            }
                        }

                        Ok(Type::Sequence(seq_type))
                    }

                    unknown => {
                        self.throw_error(VynError::TypeMismatch {
                            expected: vec![exp_type],
                            found: unknown,
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

                if property_type != Type::Integer {
                    self.throw_error(VynError::TypeMismatch {
                        expected: vec![Type::Integer],
                        found: property_type,
                        span: property.span,
                    });
                    return Err(());
                }

                match target_type.clone() {
                    Type::Array(element_type, _size) => Ok(*element_type),
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
                if let Expr::Identifier(name) = &target.node {
                    let ident_symbol = self.symbol_type_table.resolve_identifier(
                        name,
                        target.span,
                        &mut self.errors,
                    )?;

                    if !ident_symbol.mutable {
                        self.throw_error(VynError::ImmutableMutation {
                            identifier: name.clone(),
                            span: ident_symbol.span,
                            mutation_span: span,
                        });
                        return Err(());
                    }

                    if ident_symbol.is_static() {
                        self.throw_error(VynError::StaticMutation {
                            identifier: name.clone(),
                            mutator_span: span,
                            span: ident_symbol.span,
                        });
                        return Err(());
                    }
                }

                let target_type = self.check_expression(target, None)?;
                let property_type = self.check_expression(property, None)?;

                if property_type != Type::Integer {
                    self.throw_error(VynError::TypeMismatch {
                        expected: vec![Type::Integer],
                        found: property_type,
                        span: property.span,
                    });
                    return Err(());
                }

                match target_type {
                    Type::Array(element_type, _size) => {
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
                let is_static = ident_symbol.is_static();
                let ident_span = ident_symbol.span;
                let expected_type = ident_symbol.symbol_type.clone();

                if is_static {
                    self.throw_error(VynError::StaticMutation {
                        identifier: ident_name,
                        mutator_span: span,
                        span: ident_span,
                    });

                    return Err(());
                }

                if !is_mutable {
                    self.throw_error(VynError::ImmutableMutation {
                        identifier: ident_name,
                        span: ident_span,
                        mutation_span: span,
                    });
                    return Err(());
                }

                let new_value_type =
                    self.check_expression(new_value, Some(expected_type.clone()))?;

                if expected_type != new_value_type {
                    self.throw_error(VynError::TypeMismatch {
                        expected: vec![expected_type.clone()],
                        found: new_value_type,
                        span: new_value.span,
                    });
                    return Err(());
                }

                Ok(expected_type)
            }

            _ => throw_error(&format!("unknown expr:\n\n{:#?}", expr.node), 1),
        }
    }

    pub(crate) fn throw_error(&mut self, error: VynError) {
        self.errors.add(error);
    }
}
