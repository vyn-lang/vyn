use crate::{
    ast::ast::{Expr, Expression, Program, Statement, Stmt},
    error_handler::{error_collector::ErrorCollector, errors::VynError},
    tokens::TokenType,
    utils::Span,
};
use std::collections::HashMap;

#[derive(Clone, Debug, PartialEq)]
pub enum StaticValue {
    Int(i32),
    Float(f64),
    Bool(bool),
    String(String),
    Nil,
}

impl StaticValue {
    pub fn as_int(&self) -> Option<i32> {
        match self {
            StaticValue::Int(n) => Some(*n),
            _ => None,
        }
    }

    pub fn as_float(&self) -> Option<f64> {
        match self {
            StaticValue::Float(f) => Some(*f),
            _ => None,
        }
    }

    pub fn as_bool(&self) -> Option<bool> {
        match self {
            StaticValue::Bool(b) => Some(*b),
            _ => None,
        }
    }
}

pub struct StaticEvaluator {
    // Store evaluated statics
    statics: HashMap<String, (StaticValue, Span)>,
    // Track dependencies to detect cycles
    evaluating: Vec<String>,
}

impl StaticEvaluator {
    pub fn new() -> Self {
        Self {
            statics: HashMap::new(),
            evaluating: Vec::new(),
        }
    }

    /// Main entry point - evaluates all static declarations in the program
    pub fn evaluate_program(
        &mut self,
        program: &Program,
        errors: &mut ErrorCollector,
    ) -> Result<(), ()> {
        // First pass: collect all static declarations
        let mut static_decls = Vec::new();
        self.collect_static_decls(program, &mut static_decls);

        // Second pass: evaluate each static
        for (name, value_expr, span) in static_decls {
            if let Err(_) = self.evaluate_and_store_static(&name, &value_expr, span, errors) {
                // Continue evaluating other statics even if one fails
                continue;
            }
        }

        if errors.has_errors() { Err(()) } else { Ok(()) }
    }

    /// Collect all static variable declarations from the program
    fn collect_static_decls(&self, program: &Program, decls: &mut Vec<(String, Expression, Span)>) {
        for stmt in &program.statements {
            self.collect_static_decls_from_stmt(stmt, decls);
        }
    }

    fn collect_static_decls_from_stmt(
        &self,
        stmt: &Statement,
        decls: &mut Vec<(String, Expression, Span)>,
    ) {
        match &stmt.node {
            Stmt::StaticVariableDeclaration {
                identifier, value, ..
            } => {
                let name = match &identifier.node {
                    Expr::Identifier(n) => n.clone(),
                    _ => return,
                };
                decls.push((name, value.clone(), stmt.span));
            }
            Stmt::Block { statements } => {
                for s in statements {
                    self.collect_static_decls_from_stmt(s, decls);
                }
            }
            // Add other statement types that can contain nested static decls
            _ => {}
        }
    }

    /// Evaluate a static and store it
    fn evaluate_and_store_static(
        &mut self,
        name: &str,
        expr: &Expression,
        span: Span,
        errors: &mut ErrorCollector,
    ) -> Result<(), ()> {
        // Check for duplicate declarations
        if self.statics.contains_key(name) {
            errors.add(VynError::VariableRedeclaration {
                name: name.to_string(),
                original_span: self.statics[name].1,
                redeclaration_span: span,
            });
            return Err(());
        }

        // Check for circular dependencies
        if self.evaluating.contains(&name.to_string()) {
            errors.add(VynError::CircularStaticDependency {
                name: name.to_string(),
                span,
            });
            return Err(());
        }

        // Mark as being evaluated
        self.evaluating.push(name.to_string());

        // Evaluate the expression
        let result = self.evaluate_static_expr(expr, errors);

        // Remove from evaluation stack
        self.evaluating.pop();

        match result {
            Ok(value) => {
                self.statics.insert(name.to_string(), (value, span));
                Ok(())
            }
            Err(_) => {
                errors.add(VynError::StaticEvaluationFailed {
                    name: name.to_string(),
                    span: expr.span,
                });
                Err(())
            }
        }
    }

    /// Evaluate a static expression
    pub fn evaluate_static_expr(
        &mut self,
        expr: &Expression,
        errors: &mut ErrorCollector,
    ) -> Result<StaticValue, ()> {
        match &expr.node {
            Expr::IntegerLiteral(n) => Ok(StaticValue::Int(*n)),

            Expr::FloatLiteral(f) => Ok(StaticValue::Float(*f)),

            Expr::BooleanLiteral(b) => Ok(StaticValue::Bool(*b)),

            Expr::StringLiteral(s) => Ok(StaticValue::String(s.clone())),

            Expr::NilLiteral => Ok(StaticValue::Nil),

            Expr::Identifier(name) => {
                // Look up in already-evaluated statics
                if let Some((value, _)) = self.statics.get(name) {
                    return Ok(value.clone());
                }

                // If not found, it might be a forward reference
                // Try to evaluate it now
                errors.add(VynError::UndefinedStatic {
                    name: name.clone(),
                    span: expr.span,
                });
                Err(())
            }

            Expr::Unary { operator, right } => {
                let right_val = self.evaluate_static_expr(right, errors)?;

                match operator.get_token_type() {
                    TokenType::Minus => match right_val {
                        StaticValue::Int(n) => Ok(StaticValue::Int(-n)),
                        StaticValue::Float(f) => Ok(StaticValue::Float(-f)),
                        _ => {
                            errors.add(VynError::InvalidStaticOperation {
                                operation: "unary minus".to_string(),
                                span: expr.span,
                            });
                            Err(())
                        }
                    },
                    TokenType::Plus => match right_val {
                        StaticValue::Int(_) | StaticValue::Float(_) => Ok(right_val),
                        _ => {
                            errors.add(VynError::InvalidStaticOperation {
                                operation: "unary plus".to_string(),
                                span: expr.span,
                            });
                            Err(())
                        }
                    },
                    TokenType::Bang => match right_val {
                        StaticValue::Bool(b) => Ok(StaticValue::Bool(!b)),
                        StaticValue::Int(n) => Ok(StaticValue::Bool(n == 0)),
                        _ => {
                            errors.add(VynError::InvalidStaticOperation {
                                operation: "logical not".to_string(),
                                span: expr.span,
                            });
                            Err(())
                        }
                    },
                    _ => {
                        errors.add(VynError::InvalidStaticOperation {
                            operation: "unknown unary".to_string(),
                            span: expr.span,
                        });
                        Err(())
                    }
                }
            }

            Expr::BinaryOperation {
                left,
                operator,
                right,
            } => {
                let left_val = self.evaluate_static_expr(left, errors)?;
                let right_val = self.evaluate_static_expr(right, errors)?;

                self.evaluate_binary_op(
                    left_val,
                    operator.get_token_type(),
                    right_val,
                    expr.span,
                    errors,
                )
            }

            _ => {
                errors.add(VynError::NotStaticExpression { span: expr.span });
                Err(())
            }
        }
    }

    fn evaluate_binary_op(
        &self,
        left: StaticValue,
        op: TokenType,
        right: StaticValue,
        span: Span,
        errors: &mut ErrorCollector,
    ) -> Result<StaticValue, ()> {
        match (left, op.clone(), right) {
            // Integer operations
            (StaticValue::Int(l), TokenType::Plus, StaticValue::Int(r)) => {
                l.checked_add(r).map(StaticValue::Int).ok_or_else(|| {
                    errors.add(VynError::StaticOverflow { span });
                    ()
                })
            }
            (StaticValue::Int(l), TokenType::Minus, StaticValue::Int(r)) => {
                l.checked_sub(r).map(StaticValue::Int).ok_or_else(|| {
                    errors.add(VynError::StaticOverflow { span });
                    ()
                })
            }
            (StaticValue::Int(l), TokenType::Asterisk, StaticValue::Int(r)) => {
                l.checked_mul(r).map(StaticValue::Int).ok_or_else(|| {
                    errors.add(VynError::StaticOverflow { span });
                    ()
                })
            }
            (StaticValue::Int(l), TokenType::Slash, StaticValue::Int(r)) => {
                if r == 0 {
                    errors.add(VynError::DivisionByZero { span });
                    Err(())
                } else {
                    Ok(StaticValue::Int(l / r))
                }
            }
            (StaticValue::Int(l), TokenType::Caret, StaticValue::Int(r)) => {
                if r < 0 {
                    errors.add(VynError::NegativeExponent { span });
                    Err(())
                } else {
                    l.checked_pow(r as u32)
                        .map(StaticValue::Int)
                        .ok_or_else(|| {
                            errors.add(VynError::StaticOverflow { span });
                            ()
                        })
                }
            }

            // Float operations
            (StaticValue::Float(l), TokenType::Plus, StaticValue::Float(r)) => {
                Ok(StaticValue::Float(l + r))
            }
            (StaticValue::Float(l), TokenType::Minus, StaticValue::Float(r)) => {
                Ok(StaticValue::Float(l - r))
            }
            (StaticValue::Float(l), TokenType::Asterisk, StaticValue::Float(r)) => {
                Ok(StaticValue::Float(l * r))
            }
            (StaticValue::Float(l), TokenType::Slash, StaticValue::Float(r)) => {
                if r == 0.0 {
                    errors.add(VynError::DivisionByZero { span });
                    Err(())
                } else {
                    Ok(StaticValue::Float(l / r))
                }
            }

            // Boolean operations
            (StaticValue::Bool(l), TokenType::And, StaticValue::Bool(r)) => {
                Ok(StaticValue::Bool(l && r))
            }
            (StaticValue::Bool(l), TokenType::Or, StaticValue::Bool(r)) => {
                Ok(StaticValue::Bool(l || r))
            }

            _ => {
                errors.add(VynError::InvalidStaticOperation {
                    operation: format!("{:?}", op),
                    span,
                });
                Err(())
            }
        }
    }

    /// Get a static value by name (used by type checker)
    pub fn get_static(&self, name: &str) -> Option<&StaticValue> {
        self.statics.get(name).map(|(value, _)| value)
    }

    /// Get a static as an integer (helper for array sizes)
    pub fn get_static_int(&self, name: &str) -> Option<i32> {
        self.get_static(name)?.as_int()
    }
}
