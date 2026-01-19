use crate::{
    ast::{
        ast::{Expr, Expression, Program, Statement, Stmt},
        type_annotation::TypeAnnotation,
    },
    errors::{ErrorCollector, HydorError},
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
            // Ignore individual errors, keep checking all statements
            let _ = self.check_statement(stmt);
        }

        if self.errors.has_errors() {
            Err(mem::take(&mut self.errors))
        } else {
            Ok(())
        }
    }

    pub(crate) fn check_statement(&mut self, stmt: &Statement) -> Result<(), ()> {
        match &stmt.node {
            Stmt::Expression { expression } => {
                self.check_expression(expression)?;
                Ok(())
            }
            Stmt::VariableDeclaration {
                identifier,
                value,
                annotated_type,
                span,
                ..
            } => {
                let an_type = Type::from_anotated_type(annotated_type);
                let value_type = self.check_expression(value)?;

                let var_name = match &identifier.node {
                    Expr::Identifier(name) => name.clone(),
                    _ => unreachable!("Var names are always identifiers"),
                };

                // Check type match first
                if an_type != value_type {
                    self.throw_error(HydorError::DeclarationTypeMismatch {
                        expected: an_type.clone(),
                        got: value_type,
                        span: *span,
                    });

                    return Err(());
                }

                // Declare in symbol table (no register assignment during type checking)
                self.symbol_type_table.declare_identifier(
                    var_name,
                    an_type,
                    *span,
                    &mut self.errors,
                )?;

                Ok(())
            }

            _ => throw_error(&format!("unknown ast: \n\n{:#?}", stmt.node), 1),
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
                self.symbol_type_table
                    .resolve_identifier(name, span, &mut self.errors)
            }

            Expr::Unary { operator, right } => self.check_unary(operator, right, span),

            Expr::BinaryOperation {
                left,
                operator,
                right,
            } => self.check_binary_expr(operator, left, right, span),

            _ => unreachable!("Unknown expression type {:?}", expr.node),
        }
    }

    pub(crate) fn throw_error(&mut self, error: HydorError) {
        self.errors.add(error);
    }
}
