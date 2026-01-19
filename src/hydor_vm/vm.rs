use crate::{
    bytecode::bytecode::{Instructions, OpCode, ToOpcode, read_uint8, read_uint16},
    runtime_value::RuntimeValue,
};
/*
 * TODO: Refactor VM
 * */

// Singletons for common values
const NIL: RuntimeValue = RuntimeValue::NilLiteral;
const TRUE: RuntimeValue = RuntimeValue::BooleanLiteral(true);
const FALSE: RuntimeValue = RuntimeValue::BooleanLiteral(false);

pub struct HydorVM {
    // Registers store actual RuntimeValues
    registers: [RuntimeValue; 256],
    // Constant pool
    constants: Vec<RuntimeValue>,
    // String table (since strings are stored by index)
    strings: Vec<String>,
    // Program bytecode
    instructions: Instructions,
    // Instruction pointer
    ip: usize,
}

impl HydorVM {
    pub fn new(
        instructions: Instructions,
        constants: Vec<RuntimeValue>,
        strings: Vec<String>,
    ) -> Self {
        Self {
            registers: [NIL; 256], // Initialize all to Nil singleton
            constants,
            strings,
            instructions,
            ip: 0,
        }
    }

    pub fn run(&mut self) -> Result<(), String> {
        loop {
            let opcode = self.instructions[self.ip].to_opcode();

            match opcode {
                OpCode::Halt => {
                    break;
                }

                // Load operations
                OpCode::LoadConstInt => {
                    let dest = read_uint8(&self.instructions, self.ip + 1);
                    let const_index = read_uint16(&self.instructions, self.ip + 2) as usize;

                    let value = self
                        .constants
                        .get(const_index)
                        .ok_or(format!("Invalid constant index {}", const_index))?;

                    if let Some(val) = value.as_int() {
                        self.registers[dest as usize] = RuntimeValue::IntegerLiteral(val);
                    } else {
                        return Err(format!(
                            "Expected integer constant at index {}",
                            const_index
                        ));
                    }

                    self.ip += 4; // opcode + dest(1) + index(2)
                }

                OpCode::LoadConstFloat => {
                    let dest = read_uint8(&self.instructions, self.ip + 1);
                    let const_index = read_uint16(&self.instructions, self.ip + 2) as usize;

                    let value = self
                        .constants
                        .get(const_index)
                        .ok_or(format!("Invalid constant index {}", const_index))?;

                    if let Some(val) = value.as_float() {
                        self.registers[dest as usize] = RuntimeValue::FloatLiteral(val);
                    } else {
                        return Err(format!("Expected float constant at index {}", const_index));
                    }

                    self.ip += 4;
                }

                OpCode::LoadString => {
                    let dest = read_uint8(&self.instructions, self.ip + 1);
                    let string_index = read_uint16(&self.instructions, self.ip + 2) as usize;

                    if string_index < self.strings.len() {
                        self.registers[dest as usize] = RuntimeValue::StringLiteral(string_index);
                    } else {
                        return Err(format!("Invalid string index {}", string_index));
                    }

                    self.ip += 4;
                }

                OpCode::LoadNil => {
                    let dest = read_uint8(&self.instructions, self.ip + 1);
                    self.registers[dest as usize] = NIL; // Singleton!
                    self.ip += 2;
                }

                OpCode::LoadTrue => {
                    let dest = read_uint8(&self.instructions, self.ip + 1);
                    self.registers[dest as usize] = TRUE; // Singleton!
                    self.ip += 2;
                }

                OpCode::LoadFalse => {
                    let dest = read_uint8(&self.instructions, self.ip + 1);
                    self.registers[dest as usize] = FALSE; // Singleton!
                    self.ip += 2;
                }

                // Integer arithmetic
                OpCode::AddInt => {
                    let dest = read_uint8(&self.instructions, self.ip + 1);
                    let left = read_uint8(&self.instructions, self.ip + 2);
                    let right = read_uint8(&self.instructions, self.ip + 3);

                    let a = self.registers[left as usize]
                        .as_int()
                        .ok_or("AddInt: left operand is not an integer")?;
                    let b = self.registers[right as usize]
                        .as_int()
                        .ok_or("AddInt: right operand is not an integer")?;

                    self.registers[dest as usize] = RuntimeValue::IntegerLiteral(a + b);
                    self.ip += 4;
                }

                OpCode::SubtractInt => {
                    let dest = read_uint8(&self.instructions, self.ip + 1);
                    let left = read_uint8(&self.instructions, self.ip + 2);
                    let right = read_uint8(&self.instructions, self.ip + 3);

                    let a = self.registers[left as usize]
                        .as_int()
                        .ok_or("SubtractInt: left operand is not an integer")?;
                    let b = self.registers[right as usize]
                        .as_int()
                        .ok_or("SubtractInt: right operand is not an integer")?;

                    self.registers[dest as usize] = RuntimeValue::IntegerLiteral(a - b);
                    self.ip += 4;
                }

                OpCode::MultiplyInt => {
                    let dest = read_uint8(&self.instructions, self.ip + 1);
                    let left = read_uint8(&self.instructions, self.ip + 2);
                    let right = read_uint8(&self.instructions, self.ip + 3);

                    let a = self.registers[left as usize]
                        .as_int()
                        .ok_or("MultiplyInt: left operand is not an integer")?;
                    let b = self.registers[right as usize]
                        .as_int()
                        .ok_or("MultiplyInt: right operand is not an integer")?;

                    self.registers[dest as usize] = RuntimeValue::IntegerLiteral(a * b);
                    self.ip += 4;
                }

                OpCode::DivideInt => {
                    let dest = read_uint8(&self.instructions, self.ip + 1);
                    let left = read_uint8(&self.instructions, self.ip + 2);
                    let right = read_uint8(&self.instructions, self.ip + 3);

                    let a = self.registers[left as usize]
                        .as_int()
                        .ok_or("DivideInt: left operand is not an integer")?;
                    let b = self.registers[right as usize]
                        .as_int()
                        .ok_or("DivideInt: right operand is not an integer")?;

                    if b == 0 {
                        return Err("Division by zero".to_string());
                    }

                    self.registers[dest as usize] = RuntimeValue::IntegerLiteral(a / b);
                    self.ip += 4;
                }

                OpCode::ExponentInt => {
                    let dest = read_uint8(&self.instructions, self.ip + 1);
                    let left = read_uint8(&self.instructions, self.ip + 2);
                    let right = read_uint8(&self.instructions, self.ip + 3);

                    let a = self.registers[left as usize]
                        .as_int()
                        .ok_or("ExponentInt: left operand is not an integer")?;
                    let b = self.registers[right as usize]
                        .as_int()
                        .ok_or("ExponentInt: right operand is not an integer")?;

                    if b < 0 {
                        return Err("Negative exponent for integer".to_string());
                    }

                    self.registers[dest as usize] = RuntimeValue::IntegerLiteral(a.pow(b as u32));
                    self.ip += 4;
                }

                // Float arithmetic
                OpCode::AddFloat => {
                    let dest = read_uint8(&self.instructions, self.ip + 1);
                    let left = read_uint8(&self.instructions, self.ip + 2);
                    let right = read_uint8(&self.instructions, self.ip + 3);

                    let a = self.registers[left as usize]
                        .as_float()
                        .ok_or("AddFloat: left operand is not a float")?;
                    let b = self.registers[right as usize]
                        .as_float()
                        .ok_or("AddFloat: right operand is not a float")?;

                    self.registers[dest as usize] = RuntimeValue::FloatLiteral(a + b);
                    self.ip += 4;
                }

                OpCode::SubtractFloat => {
                    let dest = read_uint8(&self.instructions, self.ip + 1);
                    let left = read_uint8(&self.instructions, self.ip + 2);
                    let right = read_uint8(&self.instructions, self.ip + 3);

                    let a = self.registers[left as usize]
                        .as_float()
                        .ok_or("SubtractFloat: left operand is not a float")?;
                    let b = self.registers[right as usize]
                        .as_float()
                        .ok_or("SubtractFloat: right operand is not a float")?;

                    self.registers[dest as usize] = RuntimeValue::FloatLiteral(a - b);
                    self.ip += 4;
                }

                OpCode::MultiplyFloat => {
                    let dest = read_uint8(&self.instructions, self.ip + 1);
                    let left = read_uint8(&self.instructions, self.ip + 2);
                    let right = read_uint8(&self.instructions, self.ip + 3);

                    let a = self.registers[left as usize]
                        .as_float()
                        .ok_or("MultiplyFloat: left operand is not a float")?;
                    let b = self.registers[right as usize]
                        .as_float()
                        .ok_or("MultiplyFloat: right operand is not a float")?;

                    self.registers[dest as usize] = RuntimeValue::FloatLiteral(a * b);
                    self.ip += 4;
                }

                OpCode::DivideFloat => {
                    let dest = read_uint8(&self.instructions, self.ip + 1);
                    let left = read_uint8(&self.instructions, self.ip + 2);
                    let right = read_uint8(&self.instructions, self.ip + 3);

                    let a = self.registers[left as usize]
                        .as_float()
                        .ok_or("DivideFloat: left operand is not a float")?;
                    let b = self.registers[right as usize]
                        .as_float()
                        .ok_or("DivideFloat: right operand is not a float")?;

                    self.registers[dest as usize] = RuntimeValue::FloatLiteral(a / b);
                    self.ip += 4;
                }

                OpCode::ExponentFloat => {
                    let dest = read_uint8(&self.instructions, self.ip + 1);
                    let left = read_uint8(&self.instructions, self.ip + 2);
                    let right = read_uint8(&self.instructions, self.ip + 3);

                    let a = self.registers[left as usize]
                        .as_float()
                        .ok_or("ExponentFloat: left operand is not a float")?;
                    let b = self.registers[right as usize]
                        .as_float()
                        .ok_or("ExponentFloat: right operand is not a float")?;

                    self.registers[dest as usize] = RuntimeValue::FloatLiteral(a.powf(b));
                    self.ip += 4;
                }

                // Unary operations
                OpCode::NegateInt => {
                    let dest = read_uint8(&self.instructions, self.ip + 1);
                    let src = read_uint8(&self.instructions, self.ip + 2);

                    let val = self.registers[src as usize]
                        .as_int()
                        .ok_or("NegateInt: operand is not an integer")?;

                    self.registers[dest as usize] = RuntimeValue::IntegerLiteral(-val);
                    self.ip += 3;
                }

                OpCode::NegateFloat => {
                    let dest = read_uint8(&self.instructions, self.ip + 1);
                    let src = read_uint8(&self.instructions, self.ip + 2);

                    let val = self.registers[src as usize]
                        .as_float()
                        .ok_or("NegateFloat: operand is not a float")?;

                    self.registers[dest as usize] = RuntimeValue::FloatLiteral(-val);
                    self.ip += 3;
                }

                OpCode::Not => {
                    let dest = read_uint8(&self.instructions, self.ip + 1);
                    let src = read_uint8(&self.instructions, self.ip + 2);

                    let val = self.registers[src as usize]
                        .as_bool()
                        .ok_or("Not: operand is not a boolean")?;

                    // Use singletons!
                    self.registers[dest as usize] = if val { FALSE } else { TRUE };
                    self.ip += 3;
                }

                // Integer comparisons
                OpCode::LessInt => {
                    let dest = read_uint8(&self.instructions, self.ip + 1);
                    let left = read_uint8(&self.instructions, self.ip + 2);
                    let right = read_uint8(&self.instructions, self.ip + 3);

                    let a = self.registers[left as usize]
                        .as_int()
                        .ok_or("LessInt: left operand is not an integer")?;
                    let b = self.registers[right as usize]
                        .as_int()
                        .ok_or("LessInt: right operand is not an integer")?;

                    // Use singletons!
                    self.registers[dest as usize] = if a < b { TRUE } else { FALSE };
                    self.ip += 4;
                }

                OpCode::LessEqualInt => {
                    let dest = read_uint8(&self.instructions, self.ip + 1);
                    let left = read_uint8(&self.instructions, self.ip + 2);
                    let right = read_uint8(&self.instructions, self.ip + 3);

                    let a = self.registers[left as usize]
                        .as_int()
                        .ok_or("LessEqualInt: left operand is not an integer")?;
                    let b = self.registers[right as usize]
                        .as_int()
                        .ok_or("LessEqualInt: right operand is not an integer")?;

                    self.registers[dest as usize] = if a <= b { TRUE } else { FALSE };
                    self.ip += 4;
                }

                OpCode::GreaterInt => {
                    let dest = read_uint8(&self.instructions, self.ip + 1);
                    let left = read_uint8(&self.instructions, self.ip + 2);
                    let right = read_uint8(&self.instructions, self.ip + 3);

                    let a = self.registers[left as usize]
                        .as_int()
                        .ok_or("GreaterInt: left operand is not an integer")?;
                    let b = self.registers[right as usize]
                        .as_int()
                        .ok_or("GreaterInt: right operand is not an integer")?;

                    self.registers[dest as usize] = if a > b { TRUE } else { FALSE };
                    self.ip += 4;
                }

                OpCode::GreaterEqualInt => {
                    let dest = read_uint8(&self.instructions, self.ip + 1);
                    let left = read_uint8(&self.instructions, self.ip + 2);
                    let right = read_uint8(&self.instructions, self.ip + 3);

                    let a = self.registers[left as usize]
                        .as_int()
                        .ok_or("GreaterEqualInt: left operand is not an integer")?;
                    let b = self.registers[right as usize]
                        .as_int()
                        .ok_or("GreaterEqualInt: right operand is not an integer")?;

                    self.registers[dest as usize] = if a >= b { TRUE } else { FALSE };
                    self.ip += 4;
                }

                // Float comparisons
                OpCode::LessFloat => {
                    let dest = read_uint8(&self.instructions, self.ip + 1);
                    let left = read_uint8(&self.instructions, self.ip + 2);
                    let right = read_uint8(&self.instructions, self.ip + 3);

                    let a = self.registers[left as usize]
                        .as_float()
                        .ok_or("LessFloat: left operand is not a float")?;
                    let b = self.registers[right as usize]
                        .as_float()
                        .ok_or("LessFloat: right operand is not a float")?;

                    self.registers[dest as usize] = if a < b { TRUE } else { FALSE };
                    self.ip += 4;
                }

                OpCode::LessEqualFloat => {
                    let dest = read_uint8(&self.instructions, self.ip + 1);
                    let left = read_uint8(&self.instructions, self.ip + 2);
                    let right = read_uint8(&self.instructions, self.ip + 3);

                    let a = self.registers[left as usize]
                        .as_float()
                        .ok_or("LessEqualFloat: left operand is not a float")?;
                    let b = self.registers[right as usize]
                        .as_float()
                        .ok_or("LessEqualFloat: right operand is not a float")?;

                    self.registers[dest as usize] = if a <= b { TRUE } else { FALSE };
                    self.ip += 4;
                }

                OpCode::GreaterFloat => {
                    let dest = read_uint8(&self.instructions, self.ip + 1);
                    let left = read_uint8(&self.instructions, self.ip + 2);
                    let right = read_uint8(&self.instructions, self.ip + 3);

                    let a = self.registers[left as usize]
                        .as_float()
                        .ok_or("GreaterFloat: left operand is not a float")?;
                    let b = self.registers[right as usize]
                        .as_float()
                        .ok_or("GreaterFloat: right operand is not a float")?;

                    self.registers[dest as usize] = if a > b { TRUE } else { FALSE };
                    self.ip += 4;
                }

                OpCode::GreaterEqualFloat => {
                    let dest = read_uint8(&self.instructions, self.ip + 1);
                    let left = read_uint8(&self.instructions, self.ip + 2);
                    let right = read_uint8(&self.instructions, self.ip + 3);

                    let a = self.registers[right as usize]
                        .as_float()
                        .ok_or("GreaterEqualFloat: left operand is not a float")?;
                    let b = self.registers[right as usize]
                        .as_float()
                        .ok_or("GreaterEqualFloat: right operand is not a float")?;

                    self.registers[dest as usize] = if a >= b { TRUE } else { FALSE };
                    self.ip += 4;
                }

                // General equality (works on any type)
                OpCode::Equal => {
                    let dest = read_uint8(&self.instructions, self.ip + 1);
                    let left = read_uint8(&self.instructions, self.ip + 2);
                    let right = read_uint8(&self.instructions, self.ip + 3);

                    let a = &self.registers[left as usize];
                    let b = &self.registers[right as usize];

                    self.registers[dest as usize] = if a == b { TRUE } else { FALSE };
                    self.ip += 4;
                }

                OpCode::NotEqual => {
                    let dest = read_uint8(&self.instructions, self.ip + 1);
                    let left = read_uint8(&self.instructions, self.ip + 2);
                    let right = read_uint8(&self.instructions, self.ip + 3);

                    let a = &self.registers[left as usize];
                    let b = &self.registers[right as usize];

                    self.registers[dest as usize] = if a != b { TRUE } else { FALSE };
                    self.ip += 4;
                }

                // String operations
                OpCode::ConcatString => {
                    let dest = read_uint8(&self.instructions, self.ip + 1);
                    let left = read_uint8(&self.instructions, self.ip + 2);
                    let right = read_uint8(&self.instructions, self.ip + 3);

                    let a_idx = self.registers[left as usize]
                        .as_string_index()
                        .ok_or("ConcatString: left operand is not a string")?;
                    let b_idx = self.registers[right as usize]
                        .as_string_index()
                        .ok_or("ConcatString: right operand is not a string")?;

                    let a_str = self
                        .strings
                        .get(a_idx)
                        .ok_or(format!("Invalid string index {}", a_idx))?;
                    let b_str = self
                        .strings
                        .get(b_idx)
                        .ok_or(format!("Invalid string index {}", b_idx))?;

                    let result = format!("{}{}", a_str, b_str);
                    self.strings.push(result);
                    let new_index = self.strings.len() - 1;

                    self.registers[dest as usize] = RuntimeValue::StringLiteral(new_index);
                    self.ip += 4;
                }

                // Move
                OpCode::Move => {
                    let dest = read_uint8(&self.instructions, self.ip + 1);
                    let src = read_uint8(&self.instructions, self.ip + 2);

                    self.registers[dest as usize] = self.registers[src as usize];
                    self.ip += 3;
                }

                _ => {
                    return Err(format!("Unimplemented opcode: {:?}", opcode));
                }
            }
        }

        Ok(())
    }

    // Public accessor methods for debugging/testing
    pub fn get_register(&self, reg: u8) -> &RuntimeValue {
        &self.registers[reg as usize]
    }

    pub fn get_register_as_int(&self, reg: u8) -> Option<i32> {
        self.registers[reg as usize].as_int()
    }

    pub fn get_register_as_float(&self, reg: u8) -> Option<f64> {
        self.registers[reg as usize].as_float()
    }

    pub fn get_register_as_bool(&self, reg: u8) -> Option<bool> {
        self.registers[reg as usize].as_bool()
    }

    pub fn get_string(&self, index: usize) -> Option<&String> {
        self.strings.get(index)
    }
}
