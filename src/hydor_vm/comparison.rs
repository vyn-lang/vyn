use crate::{
    bytecode::bytecode::OpCode,
    errors::HydorError,
    hydor_vm::vm::{BOOLEAN_FALSE, BOOLEAN_TRUE, HydorVM},
    runtime_value::RuntimeValue,
    utils::Span,
};

impl HydorVM {
    pub(crate) fn compare_operation(
        &mut self,
        opcode: OpCode,
        span: Span,
    ) -> Result<(), HydorError> {
        let (right, _) = self.pop_with_span()?;
        let (left, _) = self.pop_with_span()?;

        let result = match opcode {
            OpCode::CompareLessInt => {
                let a = left.as_int().unwrap();
                let b = right.as_int().unwrap();
                a < b
            }
            OpCode::CompareLessFloat => {
                let a = left.as_float().unwrap();
                let b = right.as_float().unwrap();
                a < b
            }
            OpCode::CompareLessEqualInt => {
                let a = left.as_int().unwrap();
                let b = right.as_int().unwrap();
                a <= b
            }
            OpCode::CompareLessEqualFloat => {
                let a = left.as_float().unwrap();
                let b = right.as_float().unwrap();
                a <= b
            }
            OpCode::CompareGreaterInt => {
                let a = left.as_int().unwrap();
                let b = right.as_int().unwrap();
                a > b
            }
            OpCode::CompareGreaterFloat => {
                let a = left.as_float().unwrap();
                let b = right.as_float().unwrap();
                a > b
            }
            OpCode::CompareGreaterEqualInt => {
                let a = left.as_int().unwrap();
                let b = right.as_int().unwrap();
                a >= b
            }
            OpCode::CompareGreaterEqualFloat => {
                let a = left.as_float().unwrap();
                let b = right.as_float().unwrap();
                a >= b
            }
            OpCode::CompareEqual => self.values_equal(left, right),
            OpCode::CompareNotEqual => !self.values_equal(left, right),
            _ => unreachable!("Type checker should catch invalid comparison operations"),
        };

        self.push(if result { BOOLEAN_TRUE } else { BOOLEAN_FALSE }, span)?;
        Ok(())
    }

    fn values_equal(&self, left: RuntimeValue, right: RuntimeValue) -> bool {
        match (left, right) {
            (RuntimeValue::IntegerLiteral(a), RuntimeValue::IntegerLiteral(b)) => a == b,
            (RuntimeValue::FloatLiteral(a), RuntimeValue::FloatLiteral(b)) => a == b,
            (RuntimeValue::BooleanLiteral(a), RuntimeValue::BooleanLiteral(b)) => a == b,
            (RuntimeValue::StringLiteral(a), RuntimeValue::StringLiteral(b)) => {
                self.resolve_string(a) == self.resolve_string(b)
            }
            (RuntimeValue::NilLiteral, RuntimeValue::NilLiteral) => true,
            _ => false,
        }
    }
}
