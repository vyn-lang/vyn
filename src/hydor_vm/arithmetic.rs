use crate::{
    bytecode::bytecode::OpCode, errors::HydorError, hydor_vm::vm::HydorVM,
    runtime_value::RuntimeValue, utils::Span,
};

impl HydorVM {
    pub(crate) fn binary_op(&mut self, opcode: OpCode, span: Span) -> Result<(), HydorError> {
        let (right, right_span) = self.pop_with_span()?;
        let (left, left_span) = self.pop_with_span()?;

        let result = match opcode {
            OpCode::AddInt => {
                let a = left.as_int().unwrap();
                let b = right.as_int().unwrap();
                RuntimeValue::IntegerLiteral(a + b)
            }
            OpCode::AddFloat => {
                let a = left.as_float().unwrap();
                let b = right.as_float().unwrap();
                RuntimeValue::FloatLiteral(a + b)
            }
            OpCode::SubtractInt => {
                let a = left.as_int().unwrap();
                let b = right.as_int().unwrap();
                RuntimeValue::IntegerLiteral(a - b)
            }
            OpCode::SubtractFloat => {
                let a = left.as_float().unwrap();
                let b = right.as_float().unwrap();
                RuntimeValue::FloatLiteral(a - b)
            }
            OpCode::MultiplyInt => {
                let a = left.as_int().unwrap();
                let b = right.as_int().unwrap();
                RuntimeValue::IntegerLiteral(a * b)
            }
            OpCode::MultiplyFloat => {
                let a = left.as_float().unwrap();
                let b = right.as_float().unwrap();
                RuntimeValue::FloatLiteral(a * b)
            }
            OpCode::DivideInt => {
                let a = left.as_int().unwrap();
                let b = right.as_int().unwrap();
                RuntimeValue::IntegerLiteral(a / b)
            }
            OpCode::DivideFloat => {
                let a = left.as_float().unwrap();
                let b = right.as_float().unwrap();
                RuntimeValue::FloatLiteral(a / b)
            }
            OpCode::ExponentInt => {
                let a = left.as_int().unwrap() as f64;
                let b = right.as_int().unwrap() as f64;
                let result = a.powf(b);
                if result.fract() == 0.0 {
                    RuntimeValue::IntegerLiteral(result as i32)
                } else {
                    RuntimeValue::FloatLiteral(result)
                }
            }
            OpCode::ExponentFloat => {
                let a = left.as_float().unwrap();
                let b = right.as_float().unwrap();
                RuntimeValue::FloatLiteral(a.powf(b))
            }
            _ => unreachable!("Type checker should catch invalid binary operations"),
        };

        let result_span = Span {
            line: left_span.line,
            start_column: left_span.start_column,
            end_column: right_span.end_column,
        };

        self.push(result, result_span)?;
        Ok(())
    }

    pub(crate) fn string_concat(&mut self, span: Span) -> Result<(), HydorError> {
        let (right, right_span) = self.pop_with_span()?;
        let (left, left_span) = self.pop_with_span()?;

        let left_idx = left.as_string_index().unwrap();
        let right_idx = right.as_string_index().unwrap();

        let left_str = self.resolve_string(left_idx);
        let right_str = self.resolve_string(right_idx);

        let concatenated = format!("{}{}", left_str, right_str);
        let str_index = self.intern_string(concatenated);

        let result_span = Span {
            line: left_span.line,
            start_column: left_span.start_column,
            end_column: right_span.end_column,
        };

        self.push(RuntimeValue::StringLiteral(str_index), result_span)?;
        Ok(())
    }
}
