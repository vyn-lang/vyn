use crate::{
    ast::ast::Expression,
    bytecode::bytecode::OpCode,
    compiler::compiler::Compiler,
    error_handler::errors::VynError,
    runtime_value::values::RuntimeValue,
    tokens::{Token, TokenType},
    type_checker::type_checker::Type,
    utils::Span,
};

impl Compiler<'_> {
    pub(crate) fn compile_unary_expr(
        &mut self,
        operator: Token,
        right: &Box<Expression>,
        span: Span,
    ) -> Option<u8> {
        // Try to fold at compile time
        if let Some(result) = self.try_fold_unary(&operator, right) {
            let dest_reg = self.allocate_register()?;
            let const_idx = self.add_constant(result.clone());

            match result {
                RuntimeValue::IntegerLiteral(_) => {
                    self.emit(
                        OpCode::LoadConstInt,
                        vec![dest_reg as usize, const_idx],
                        span,
                    );
                }
                RuntimeValue::FloatLiteral(_) => {
                    self.emit(
                        OpCode::LoadConstFloat,
                        vec![dest_reg as usize, const_idx],
                        span,
                    );
                }
                RuntimeValue::BooleanLiteral(true) => {
                    self.emit(OpCode::LoadTrue, vec![dest_reg as usize], span);
                }
                RuntimeValue::BooleanLiteral(false) => {
                    self.emit(OpCode::LoadFalse, vec![dest_reg as usize], span);
                }
                _ => unreachable!(),
            }

            return Some(dest_reg);
        }

        // If we couldn't fold, compile normally
        let right_expr = (**right).clone();
        let operand_type = self.get_expr_type(right)?;

        let src_reg = self.compile_expression(right_expr, None)?;
        let dest_reg = self.allocate_register()?;

        match operator.get_token_type() {
            TokenType::Minus => match operand_type {
                Type::Integer => {
                    self.emit(
                        OpCode::NegateInt,
                        vec![dest_reg as usize, src_reg as usize],
                        span,
                    );
                }
                Type::Float => {
                    self.emit(
                        OpCode::NegateFloat,
                        vec![dest_reg as usize, src_reg as usize],
                        span,
                    );
                }
                _ => {
                    self.throw_error(VynError::TypeMismatch {
                        expected: vec![Type::Integer, Type::Float],
                        found: operand_type,
                        span,
                    });
                    return None;
                }
            },
            TokenType::Not => {
                self.emit(OpCode::Not, vec![dest_reg as usize, src_reg as usize], span);
            }
            _ => unreachable!("Unhandled unary operator type"),
        };

        // Free the operand register
        self.free_register(src_reg);

        Some(dest_reg)
    }
}
