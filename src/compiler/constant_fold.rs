use crate::{
    ast::ast::{Expr, Expression},
    compiler::compiler::Compiler,
    runtime_value::values::RuntimeValue,
    tokens::{Token, TokenType},
};

impl Compiler {
    pub(crate) fn try_fold_expr(&mut self, expr: &Expression) -> Option<RuntimeValue> {
        match &expr.node {
            Expr::IntegerLiteral(v) => Some(RuntimeValue::IntegerLiteral(*v)),
            Expr::FloatLiteral(v) => Some(RuntimeValue::FloatLiteral(*v)),
            Expr::BooleanLiteral(v) => Some(RuntimeValue::BooleanLiteral(*v)),
            Expr::StringLiteral(v) => {
                let idx = self.intern_string(v.clone());
                Some(RuntimeValue::StringLiteral(idx))
            }
            Expr::NilLiteral => Some(RuntimeValue::NilLiteral),

            Expr::Unary { operator, right } => {
                let folded_operand = self.try_fold_expr(right)?;
                self.try_fold_unary_value(operator, &folded_operand)
            }

            Expr::BinaryOperation {
                left,
                operator,
                right,
            } => {
                let left_val = self.try_fold_expr(left)?;
                let right_val = self.try_fold_expr(right)?;
                self.try_fold_binary(&left_val, operator, &right_val)
            }

            _ => None,
        }
    }

    pub fn try_fold_unary(
        &mut self,
        operator: &Token,
        operand: &Expression,
    ) -> Option<RuntimeValue> {
        let folded_operand = self.try_fold_expr(operand)?;
        self.try_fold_unary_value(operator, &folded_operand)
    }

    fn try_fold_unary_value(
        &self,
        operator: &Token,
        operand: &RuntimeValue,
    ) -> Option<RuntimeValue> {
        match operator.get_token_type() {
            TokenType::Minus => match operand {
                RuntimeValue::IntegerLiteral(v) => Some(RuntimeValue::IntegerLiteral(-v)),
                RuntimeValue::FloatLiteral(v) => Some(RuntimeValue::FloatLiteral(-v)),
                _ => None,
            },
            TokenType::Not => match operand {
                RuntimeValue::BooleanLiteral(v) => Some(RuntimeValue::BooleanLiteral(!v)),
                _ => None,
            },
            _ => None,
        }
    }

    pub fn try_fold_binary(
        &mut self,
        left: &RuntimeValue,
        operator: &crate::tokens::Token,
        right: &RuntimeValue,
    ) -> Option<RuntimeValue> {
        match operator.get_token_type() {
            TokenType::Plus => match (left, right) {
                (RuntimeValue::IntegerLiteral(l), RuntimeValue::IntegerLiteral(r)) => {
                    Some(RuntimeValue::IntegerLiteral(l + r))
                }
                (RuntimeValue::FloatLiteral(l), RuntimeValue::FloatLiteral(r)) => {
                    Some(RuntimeValue::FloatLiteral(l + r))
                }
                (RuntimeValue::StringLiteral(l), RuntimeValue::StringLiteral(r)) => {
                    let left = self.get_intern_string(*l);
                    let right = self.get_intern_string(*r);

                    let mut new_str = String::with_capacity(left.len() + right.len());
                    new_str.push_str(&left);
                    new_str.push_str(&right);
                    let new_str_idx = self.intern_string(new_str);
                    Some(RuntimeValue::StringLiteral(new_str_idx))
                }
                _ => None,
            },
            TokenType::Minus => match (left, right) {
                (RuntimeValue::IntegerLiteral(l), RuntimeValue::IntegerLiteral(r)) => {
                    Some(RuntimeValue::IntegerLiteral(l - r))
                }
                (RuntimeValue::FloatLiteral(l), RuntimeValue::FloatLiteral(r)) => {
                    Some(RuntimeValue::FloatLiteral(l - r))
                }
                _ => None,
            },
            TokenType::Asterisk => match (left, right) {
                (RuntimeValue::IntegerLiteral(l), RuntimeValue::IntegerLiteral(r)) => {
                    Some(RuntimeValue::IntegerLiteral(l * r))
                }
                (RuntimeValue::FloatLiteral(l), RuntimeValue::FloatLiteral(r)) => {
                    Some(RuntimeValue::FloatLiteral(l * r))
                }
                _ => None,
            },
            TokenType::Slash => match (left, right) {
                (RuntimeValue::IntegerLiteral(l), RuntimeValue::IntegerLiteral(r)) => {
                    if *r != 0 {
                        Some(RuntimeValue::IntegerLiteral(l / r))
                    } else {
                        None // Don't fold division by zero
                    }
                }
                (RuntimeValue::FloatLiteral(l), RuntimeValue::FloatLiteral(r)) => {
                    Some(RuntimeValue::FloatLiteral(l / r))
                }
                _ => None,
            },
            TokenType::Caret => match (left, right) {
                (RuntimeValue::IntegerLiteral(l), RuntimeValue::IntegerLiteral(r)) => {
                    if *r >= 0 {
                        Some(RuntimeValue::IntegerLiteral(l.pow(*r as u32)))
                    } else {
                        None
                    }
                }
                (RuntimeValue::FloatLiteral(l), RuntimeValue::FloatLiteral(r)) => {
                    Some(RuntimeValue::FloatLiteral(l.powf(*r)))
                }
                _ => None,
            },
            TokenType::Equal => Some(RuntimeValue::BooleanLiteral(left == right)),
            TokenType::NotEqual => Some(RuntimeValue::BooleanLiteral(left != right)),
            TokenType::LessThan => match (left, right) {
                (RuntimeValue::IntegerLiteral(l), RuntimeValue::IntegerLiteral(r)) => {
                    Some(RuntimeValue::BooleanLiteral(l < r))
                }
                (RuntimeValue::FloatLiteral(l), RuntimeValue::FloatLiteral(r)) => {
                    Some(RuntimeValue::BooleanLiteral(l < r))
                }
                _ => None,
            },
            TokenType::LessThanEqual => match (left, right) {
                (RuntimeValue::IntegerLiteral(l), RuntimeValue::IntegerLiteral(r)) => {
                    Some(RuntimeValue::BooleanLiteral(l <= r))
                }
                (RuntimeValue::FloatLiteral(l), RuntimeValue::FloatLiteral(r)) => {
                    Some(RuntimeValue::BooleanLiteral(l <= r))
                }
                _ => None,
            },
            TokenType::GreaterThan => match (left, right) {
                (RuntimeValue::IntegerLiteral(l), RuntimeValue::IntegerLiteral(r)) => {
                    Some(RuntimeValue::BooleanLiteral(l > r))
                }
                (RuntimeValue::FloatLiteral(l), RuntimeValue::FloatLiteral(r)) => {
                    Some(RuntimeValue::BooleanLiteral(l > r))
                }
                _ => None,
            },
            TokenType::GreaterThanEqual => match (left, right) {
                (RuntimeValue::IntegerLiteral(l), RuntimeValue::IntegerLiteral(r)) => {
                    Some(RuntimeValue::BooleanLiteral(l >= r))
                }
                (RuntimeValue::FloatLiteral(l), RuntimeValue::FloatLiteral(r)) => {
                    Some(RuntimeValue::BooleanLiteral(l >= r))
                }
                _ => None,
            },
            _ => None,
        }
    }
}
