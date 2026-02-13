use crate::{
    ast::ast::Expression,
    error_handler::errors::VynError,
    tokens::{Token, TokenType},
    type_checker::type_checker::{Type, TypeChecker},
};

impl TypeChecker<'_> {
    pub(crate) fn check_binary_expr(
        &mut self,
        operator: &crate::tokens::Token,
        left: &Expression,
        right: &Expression,
        span: crate::utils::Span,
    ) -> Result<Type, ()> {
        let left_type = self.check_expression(left, None)?;
        let right_type = self.check_expression(right, None)?;

        match operator.get_token_type() {
            TokenType::Plus
            | TokenType::Minus
            | TokenType::Asterisk
            | TokenType::Slash
            | TokenType::Caret => {
                if left_type != right_type {
                    self.throw_error(VynError::TypeMismatch {
                        expected: vec![left_type.clone()],
                        found: right_type,
                        span,
                    });
                    return Err(());
                }

                if left_type != Type::Integer && left_type != Type::Float {
                    self.throw_error(VynError::TypeMismatch {
                        expected: vec![Type::Integer, Type::Float],
                        found: left_type,
                        span,
                    });
                    return Err(());
                }

                Ok(left_type)
            }
            TokenType::Equal
            | TokenType::NotEqual
            | TokenType::GreaterThan
            | TokenType::GreaterThanEqual
            | TokenType::LessThan
            | TokenType::LessThanEqual => {
                if left_type != right_type {
                    self.throw_error(VynError::TypeMismatch {
                        expected: vec![left_type.clone()],
                        found: right_type,
                        span,
                    });
                    return Err(());
                }
                Ok(Type::Bool)
            }
            TokenType::And | TokenType::Or => {
                if left_type != Type::Bool || right_type != Type::Bool {
                    self.throw_error(VynError::TypeMismatch {
                        expected: vec![Type::Bool],
                        found: if left_type != Type::Bool {
                            left_type
                        } else {
                            right_type
                        },
                        span,
                    });
                    return Err(());
                }
                Ok(Type::Bool)
            }
            _ => {
                self.throw_error(VynError::InvalidBinaryOperator {
                    operator: operator.clone(),
                    span,
                });
                Err(())
            }
        }
    }
}
