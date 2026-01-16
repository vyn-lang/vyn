use crate::{
    bytecode::bytecode::{Instructions, OpCode, ToOpcode, read_uint16},
    compiler::compiler::{Bytecode, DebugInfo},
    errors::HydorError,
    runtime_value::RuntimeValue,
    utils::Span,
};

const MAX_STACK: usize = 10_000;

pub struct HydorVM {
    stack: Vec<StackValue>,
    last_pop: Option<RuntimeValue>,

    instructions: Instructions,
    string_table: Vec<String>,
    constants: Vec<RuntimeValue>,
    debug_info: DebugInfo,
}

#[derive(Debug, Clone, Copy)]
struct StackValue {
    value: RuntimeValue,
    span: Span,
}

// SINGLETON ---
const BOOLEAN_TRUE: RuntimeValue = RuntimeValue::BooleanLiteral(true);
const BOOLEAN_FALSE: RuntimeValue = RuntimeValue::BooleanLiteral(false);
const NIL_LITERAL: RuntimeValue = RuntimeValue::NilLiteral;

impl HydorVM {
    pub fn new(bytecode: Bytecode) -> Self {
        Self {
            stack: Vec::with_capacity(MAX_STACK),
            last_pop: None,

            string_table: bytecode.string_table,
            instructions: bytecode.instructions,
            constants: bytecode.constants,
            debug_info: bytecode.debug_info,
        }
    }

    /// Main entry point
    pub fn execute_bytecode(&mut self) -> Result<(), HydorError> {
        let mut ip: usize = 0;

        while ip < self.instructions.len() {
            let opcode = self.instructions[ip].to_opcode();
            let span = self.debug_info.get_span(ip);

            match opcode {
                OpCode::LoadConstant => {
                    let const_index = read_uint16(&self.instructions, ip + 1);
                    ip += 2;

                    let constant = self.constants[const_index as usize];
                    self.push(constant, span)?;
                }

                OpCode::LoadString => {
                    let str_index = read_uint16(&self.instructions, ip + 1);
                    ip += 2;

                    self.push(RuntimeValue::StringLiteral(str_index as usize), span)?;
                }

                OpCode::LoadNil => {
                    self.push(NIL_LITERAL, span)?;
                }
                OpCode::LoadBoolTrue => {
                    self.push(BOOLEAN_TRUE, span)?;
                }
                OpCode::LoadBoolFalse => {
                    self.push(BOOLEAN_FALSE, span)?;
                }

                OpCode::Add => {
                    self.binary_op_add()?;
                }
                OpCode::Subtract => {
                    self.binary_op_numeric("subtraction", |a, b| a - b)?;
                }
                OpCode::Multiply => {
                    self.binary_op_numeric("multiplication", |a, b| a * b)?;
                }
                OpCode::Divide => {
                    self.binary_op_numeric("division", |a, b| a / b)?;
                }
                OpCode::Exponent => {
                    self.binary_op_numeric("exponentiation", |a, b| a.powf(b))?;
                }

                OpCode::UnaryNegate => {
                    let target = self.peek_offset(0)?;
                    let operand_span = self.peek_span(0)?;

                    if !target.is_number() {
                        // This does a direct stack modification
                        // which is faster than popping and pushing
                        // a value into the stack
                        return Err(HydorError::UnaryOperationError {
                            operation: "negation".to_string(),
                            operand_type: target.get_type(),
                            span: operand_span,
                        });
                    }

                    if target.is_float() {
                        let literal = target.as_float().unwrap();
                        self.set_offset_value(0, RuntimeValue::FloatLiteral(-literal))?;
                    } else {
                        let literal = target.as_int().unwrap();
                        self.set_offset_value(0, RuntimeValue::IntegerLiteral(-literal))?;
                    }
                }

                OpCode::UnaryNot => {
                    let target = self.peek_offset(0)?;
                    let operand_span = self.peek_span(0)?;

                    match target {
                        // This does a direct stack modification
                        // which is faster than popping and pushing
                        // a value into the stack
                        RuntimeValue::BooleanLiteral(b) => {
                            self.set_offset_value(0, RuntimeValue::BooleanLiteral(!b))?;
                        }
                        _ => {
                            return Err(HydorError::UnaryOperationError {
                                operation: "logical not".to_string(),
                                operand_type: target.get_type(),
                                span: operand_span,
                            });
                        }
                    }
                }

                OpCode::Pop => {
                    self.last_pop = Some(self.pop_value()?);
                }
                OpCode::Halt => {
                    return Ok(());
                }
            }

            ip += 1; // Advance opcode
        }

        unreachable!()
    }

    fn push(&mut self, value: RuntimeValue, span: Span) -> Result<(), HydorError> {
        if self.stack.len() >= MAX_STACK {
            return Err(HydorError::StackOverflow {
                stack_length: self.stack.len(),
                span,
            });
        }

        self.stack.push(StackValue { value, span });
        Ok(())
    }

    fn peek_offset(&self, n: usize) -> Result<RuntimeValue, HydorError> {
        let size = self.stack.len();
        if n >= size {
            return Err(HydorError::StackUnderflow {
                stack_length: size,
                span: Span::default(),
            });
        }

        Ok(self.stack[size - 1 - n].value)
    }

    fn peek_span(&self, n: usize) -> Result<Span, HydorError> {
        let size = self.stack.len();
        if n >= size {
            return Err(HydorError::StackUnderflow {
                stack_length: size,
                span: Span::default(),
            });
        }

        Ok(self.stack[size - 1 - n].span)
    }

    fn set_offset_value(&mut self, n: usize, new_value: RuntimeValue) -> Result<(), HydorError> {
        let size = self.stack.len();
        if n >= size {
            return Err(HydorError::StackUnderflow {
                stack_length: size,
                span: Span::default(),
            });
        }

        self.stack[size - 1 - n].value = new_value;
        Ok(())
    }

    fn pop_value(&mut self) -> Result<RuntimeValue, HydorError> {
        self.stack
            .pop()
            .map(|sv| sv.value)
            .ok_or(HydorError::StackUnderflow {
                stack_length: 0,
                span: Span::default(),
            })
    }

    fn pop_with_span(&mut self) -> Result<(RuntimeValue, Span), HydorError> {
        self.stack
            .pop()
            .map(|sv| (sv.value, sv.span))
            .ok_or(HydorError::StackUnderflow {
                stack_length: 0,
                span: Span::default(),
            })
    }

    // For reading only
    pub fn resolve_string(&self, index: usize) -> &str {
        &self.string_table[index]
    }

    pub fn last_popped(&self) -> Option<RuntimeValue> {
        self.last_pop
    }
}

// Binary operations
impl HydorVM {
    /// General binary addition
    fn binary_op_add(&mut self) -> Result<(), HydorError> {
        let (right, right_span) = self.pop_with_span()?;
        let (left, left_span) = self.pop_with_span()?;

        // String concatenation
        if matches!(left, RuntimeValue::StringLiteral(_))
            && matches!(right, RuntimeValue::StringLiteral(_))
        {
            return self.string_concat(left, left_span, right, right_span);
        }

        // Numeric addition
        if !left.is_number() {
            return Err(HydorError::ArithmeticError {
                operation: "addition".to_string(),
                left_type: left.get_type(),
                right_type: right.get_type(),
                span: left_span,
            });
        }

        if !right.is_number() {
            return Err(HydorError::ArithmeticError {
                operation: "addition".to_string(),
                left_type: left.get_type(),
                right_type: right.get_type(),
                span: right_span,
            });
        }

        let result = self.compute_numeric(left, right, |a, b| a + b);
        let result_span = Span {
            line: left_span.line,
            start_column: left_span.start_column,
            end_column: right_span.end_column,
        };

        self.push(result, result_span)?;
        Ok(())
    }

    /// Generic numeric binary operation
    fn binary_op_numeric<F>(&mut self, op_name: &str, f: F) -> Result<(), HydorError>
    where
        F: Fn(f64, f64) -> f64,
    {
        let (right, right_span) = self.pop_with_span()?;
        let (left, left_span) = self.pop_with_span()?;

        if !left.is_number() {
            return Err(HydorError::ArithmeticError {
                operation: op_name.to_string(),
                left_type: left.get_type(),
                right_type: right.get_type(),
                span: left_span,
            });
        }

        if !right.is_number() {
            return Err(HydorError::ArithmeticError {
                operation: op_name.to_string(),
                left_type: left.get_type(),
                right_type: right.get_type(),
                span: right_span,
            });
        }

        let result = self.compute_numeric(left, right, f);
        let result_span = Span {
            line: left_span.line,
            start_column: left_span.start_column,
            end_column: right_span.end_column,
        };

        self.push(result, result_span)?;
        Ok(())
    }

    /// Compute numeric operation and preserve int/float types when possible
    fn compute_numeric<F>(&self, left: RuntimeValue, right: RuntimeValue, f: F) -> RuntimeValue
    where
        F: Fn(f64, f64) -> f64,
    {
        let a = match left {
            RuntimeValue::IntegerLiteral(n) => n as f64,
            RuntimeValue::FloatLiteral(n) => n,
            _ => unreachable!(),
        };

        let b = match right {
            RuntimeValue::IntegerLiteral(n) => n as f64,
            RuntimeValue::FloatLiteral(n) => n,
            _ => unreachable!(),
        };

        let result = f(a, b);

        // If both operands were integers and result is whole, keep as integer
        if !left.is_float() && !right.is_float() && result.fract() == 0.0 {
            RuntimeValue::IntegerLiteral(result as i32)
        } else {
            RuntimeValue::FloatLiteral(result)
        }
    }

    /// String concatenation
    fn string_concat(
        &mut self,
        left: RuntimeValue,
        left_span: Span,
        right: RuntimeValue,
        right_span: Span,
    ) -> Result<(), HydorError> {
        let left_idx = match left {
            RuntimeValue::StringLiteral(v) => v,
            _ => unreachable!(),
        };

        let right_idx = match right {
            RuntimeValue::StringLiteral(v) => v,
            _ => unreachable!(),
        };

        let left_str = self.resolve_string(left_idx);
        let right_str = self.resolve_string(right_idx);

        let concatenated = format!("{}{}", left_str, right_str);

        // Intern the new string
        let str_index = self.intern_string(concatenated);

        let result_span = Span {
            line: left_span.line,
            start_column: left_span.start_column,
            end_column: right_span.end_column,
        };

        self.push(RuntimeValue::StringLiteral(str_index), result_span)?;
        Ok(())
    }

    /// Intern a string into the string table (with deduplication)
    fn intern_string(&mut self, s: String) -> usize {
        // Check if string already exists
        if let Some(pos) = self.string_table.iter().position(|existing| existing == &s) {
            return pos;
        }

        // Add new string
        self.string_table.push(s);
        self.string_table.len() - 1
    }
}
