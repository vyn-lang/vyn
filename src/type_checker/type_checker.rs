use crate::{
    ast::{
        ast::{Expr, Expression, Program, Statement, Stmt},
        type_annotation::TypeAnnotation,
    },
    errors::{ErrorCollector, VynError},
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
}

impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Type::Integer => write!(f, "Integer"),
            Type::Float => write!(f, "Float"),
            Type::Bool => write!(f, "Bool"),
            Type::String => write!(f, "String"),
            Type::Nil => write!(f, "Nil"),
            Type::Identifier => write!(f, "Identifier"),
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
                self.check_expression(expression)?;
                Ok(())
            }

            Stmt::VariableDeclaration {
                identifier,
                value,
                annotated_type,
                mutable,
            } => {
                let expected_type = Type::from_anotated_type(annotated_type);
                let value_type = self.check_expression(value)?;

                let var_name = match &identifier.node {
                    Expr::Identifier(name) => name.clone(),
                    _ => unreachable!("Variable name must be an identifier"),
                };

                if expected_type != value_type {
                    self.throw_error(VynError::DeclarationTypeMismatch {
                        expected: expected_type.clone(),
                        got: value_type,
                        span,
                    });
                    return Err(());
                }

                self.symbol_type_table.declare_identifier(
                    var_name,
                    expected_type,
                    span,
                    *mutable,
                    &mut self.errors,
                )?;

                Ok(())
            }

            Stmt::TypeAliasDeclaration { identifier, value } => {
                let name = match &identifier.node {
                    Expr::Identifier(n) => n.clone(),
                    _ => unreachable!("Type alias identifier must be an identifier"),
                };

                if let Err(err) = self.symbol_type_table.enroll_type_alias(
                    name,
                    Type::from_anotated_type(value),
                    span,
                ) {
                    self.throw_error(err);
                }

                Ok(())
            }

            Stmt::Block { statements } => {
                let parent_table = mem::replace(
                    &mut self.symbol_type_table,
                    SymbolTypeTable::new(), // temporary placeholder
                );

                // Create child scope from parent
                self.symbol_type_table = parent_table.enter_scope();

                for stmt in statements {
                    let _ = self.check_statement(stmt);
                }

                // Exit scope - restore parent
                self.symbol_type_table = mem::replace(
                    &mut self.symbol_type_table,
                    SymbolTypeTable::new(), // temporary placeholder
                )
                .exit_scope();

                Ok(())
            }

            Stmt::IfDeclaration {
                condition,
                consequence,
                alternate,
            } => {
                self.check_expression(condition)?;
                self.check_statement(&consequence)?;

                if let Some(alt) = alternate.as_ref() {
                    self.check_statement(alt)?;
                }
                Ok(())
            }

            Stmt::StdoutLog { log_value } => {
                self.check_expression(log_value)?;
                Ok(())
            }

            _ => throw_error(&format!("unknown ast:\n\n{:#?}", stmt.node), 1),
        }
    }

    pub(crate) fn check_expression(&mut self, expr: &Expression) -> Result<Type, ()> {
        let span = expr.span;

        match &expr.node {
            Expr::IntegerLiteral(_) => Ok(Type::Integer),
            Expr::FloatLiteral(_) => Ok(Type::Float),
            Expr::BooleanLiteral(_) => Ok(Type::Bool),
            Expr::StringLiteral(_) => Ok(Type::String),
            Expr::NilLiteral => Ok(Type::Nil),

            Expr::Identifier(name) => {
                let ident =
                    self.symbol_type_table
                        .resolve_identifier(name, span, &mut self.errors)?;

                Ok(ident.symbol_type.clone())
            }

            Expr::Unary { operator, right } => self.check_unary(operator, right, span),

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

                let value_type = self.check_expression(new_value)?;
                let ident_symbol = self.symbol_type_table.resolve_identifier(
                    &ident_name,
                    span,
                    &mut self.errors,
                )?;

                if !ident_symbol.mutable {
                    self.throw_error(VynError::ImmutableMutation {
                        identifier: ident_name,
                        span: ident_symbol.span,
                        mutation_span: span,
                    });
                    return Err(());
                }

                if ident_symbol.symbol_type != value_type {
                    self.throw_error(VynError::TypeMismatch {
                        expected: vec![ident_symbol.symbol_type.clone()],
                        found: value_type,
                        span,
                    });
                    return Err(());
                }

                Ok(value_type)
            }

            _ => unreachable!("Unknown expression type {:#?}", expr.node),
        }
    }

    pub(crate) fn throw_error(&mut self, error: VynError) {
        self.errors.add(error);
    }
}
