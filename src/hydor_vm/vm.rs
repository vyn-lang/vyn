use crate::{
    bytecode::bytecode::{Instructions, OpCode, ToOpcode, read_uint16},
    compiler::compiler::{Bytecode, DebugInfo},
    errors::HydorError,
    runtime_value::RuntimeValue,
    utils::Span,
};

const MAX_STACK: usize = 10_000;

pub struct HydorVM {
    stack: Vec<RuntimeValue>,
    last_pop: Option<RuntimeValue>,

    instructions: Instructions,
    string_table: Vec<String>,
    constants: Vec<RuntimeValue>,
    debug_info: DebugInfo,
}

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

                OpCode::Add => {
                    self.binary_op_numeric("addition", |a, b| a + b, span)?;
                }
                OpCode::Subtract => {
                    self.binary_op_numeric("subtraction", |a, b| a - b, span)?;
                }
                OpCode::Multiply => {
                    self.binary_op_numeric("multiplication", |a, b| a * b, span)?;
                }
                OpCode::Divide => {
                    self.binary_op_numeric("division", |a, b| a / b, span)?;
                }
                OpCode::Exponent => {
                    self.binary_op_numeric("exponentiation", |a, b| a.powf(b), span)?;
                }

                OpCode::UnaryNegate => {
                    let value = self.pop(span)?;
                    match value {
                        RuntimeValue::IntegerLiteral(n) => {
                            self.push(RuntimeValue::IntegerLiteral(-n), span)?;
                        }
                        RuntimeValue::FloatLiteral(n) => {
                            self.push(RuntimeValue::FloatLiteral(-n), span)?;
                        }
                        _ => {
                            return Err(HydorError::TypeError {
                                operation: "unary negation".to_string(),
                                expected: "number".to_string(),
                                got: value.type_name().to_string(),
                                span,
                            });
                        }
                    }
                }
                OpCode::UnaryNot => {
                    let value = self.pop(span)?;
                    match value {
                        RuntimeValue::BooleanLiteral(b) => {
                            self.push(RuntimeValue::BooleanLiteral(!b), span)?;
                        }
                        _ => {
                            return Err(HydorError::TypeError {
                                operation: "logical not".to_string(),
                                expected: "boolean".to_string(),
                                got: value.type_name().to_string(),
                                span,
                            });
                        }
                    }
                }

                OpCode::Pop => {
                    self.last_pop = Some(self.pop(span)?);
                }
                OpCode::Halt => {
                    return Ok(());
                }
            }

            ip += 1; // Advance opcode
        }

        Ok(())
    }

    fn push(&mut self, value: RuntimeValue, span: Span) -> Result<(), HydorError> {
        if self.stack.len() >= MAX_STACK {
            return Err(HydorError::StackOverflow {
                stack_length: self.stack.len(),
                span,
            });
        }

        self.stack.push(value);
        Ok(())
    }

    fn pop(&mut self, span: Span) -> Result<RuntimeValue, HydorError> {
        self.stack.pop().ok_or(HydorError::StackUnderflow {
            stack_length: self.stack.len(),
            span,
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

impl HydorVM {
    fn binary_op_numeric<F>(&mut self, op_name: &str, f: F, span: Span) -> Result<(), HydorError>
    where
        F: Fn(f64, f64) -> f64,
    {
        let right = self.pop(span)?;
        let left = self.pop(span)?;

        if !left.is_number() {
            return Err(HydorError::TypeError {
                operation: op_name.to_string(),
                expected: "number".to_string(),
                got: left.type_name().to_string(),
                span,
            });
        }

        if !right.is_number() {
            return Err(HydorError::TypeError {
                operation: op_name.to_string(),
                expected: "number".to_string(),
                got: right.type_name().to_string(),
                span,
            });
        }

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
        let is_float = left.is_float() || right.is_float();

        if is_float {
            self.push(RuntimeValue::FloatLiteral(result), span)?;
        } else {
            self.push(RuntimeValue::IntegerLiteral(result as i32), span)?;
        }

        Ok(())
    }
}
