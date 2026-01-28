use crate::{
    ast::ast::Expression,
    error_handler::errors::VynError,
    tokens::{Token, TokenType},
    type_checker::type_checker::{Type, TypeChecker},
    utils::Span,
};

impl TypeChecker<'_> {
    pub(crate) fn check_unary(
        &mut self,
        operator: &crate::tokens::Token,
        right: &Expression,
        span: crate::utils::Span,
    ) -> Result<Type, ()> {
        let right_type = self.check_expression(right, None)?;

        match operator.get_token_type() {
            TokenType::Minus | TokenType::Plus => {
                if right_type != Type::Integer && right_type != Type::Float {
                    self.throw_error(VynError::TypeMismatch {
                        expected: vec![Type::Integer, Type::Float],
                        found: right_type,
                        span,
                    });
                    return Err(());
                }
                Ok(right_type)
            }
            TokenType::Bang => {
                if right_type != Type::Bool {
                    self.throw_error(VynError::TypeMismatch {
                        expected: vec![Type::Bool],
                        found: right_type,
                        span,
                    });
                    return Err(());
                }
                Ok(Type::Bool)
            }
            _ => {
                self.throw_error(VynError::InvalidUnaryOperator {
                    operator: operator.clone(),
                    span,
                });
                Err(())
            }
        }
    }
}
