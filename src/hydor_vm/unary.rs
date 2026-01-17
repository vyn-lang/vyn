use crate::{
    bytecode::bytecode::OpCode,
    errors::HydorError,
    hydor_vm::vm::{BOOLEAN_FALSE, BOOLEAN_TRUE, HydorVM},
    runtime_value::RuntimeValue,
    utils::Span,
};

impl HydorVM {
    pub(crate) fn unary_operation(&mut self, opcode: OpCode, span: Span) -> Result<(), HydorError> {
        match opcode {
            OpCode::UnaryNegateInt | OpCode::UnaryNegateFloat => {
                self.unary_negation_operation(span, opcode)
            }
            OpCode::UnaryNot => self.unary_not_operation(),

            _ => unreachable!(),
        }
    }

    pub(crate) fn unary_negation_operation(
        &mut self,
        span: Span,
        opcode: OpCode,
    ) -> Result<(), HydorError> {
        // This does a direct stack modification
        // which is faster than popping and pushing
        // a value into the stack
        let target = self.peek_offset(0)?;
        let target_span = self.peek_span(0)?;

        match opcode {
            OpCode::UnaryNegateInt => {
                let int = target.as_int().unwrap();
                self.set_offset_value(0, RuntimeValue::IntegerLiteral(-int))?;
            }
            OpCode::UnaryNegateFloat => {
                let float = target.as_float().unwrap();
                self.set_offset_value(0, RuntimeValue::FloatLiteral(-float))?;
            }

            _ => {
                unreachable!("Missing opcode for unary negation should be catched by type checker")
            }
        }

        Ok(())
    }

    pub(crate) fn unary_not_operation(&mut self) -> Result<(), HydorError> {
        let target = self.peek_offset(0)?;

        // Just flip return type
        if self.is_truthy(target) {
            self.set_offset_value(0, BOOLEAN_FALSE)?;
        } else {
            self.set_offset_value(0, BOOLEAN_TRUE)?;
        }

        Ok(())
    }
}
