use std::io::{self, Write};

use crate::{
    bytecode::bytecode::{Instructions, OpCode, ToOpcode, read_uint8, read_uint16},
    errors::VynError,
    runtime_value::RuntimeValue,
};

// Singletons for common values
pub const NIL: RuntimeValue = RuntimeValue::NilLiteral;
pub const TRUE: RuntimeValue = RuntimeValue::BooleanLiteral(true);
pub const FALSE: RuntimeValue = RuntimeValue::BooleanLiteral(false);

const MAX_REGISTERS: usize = 256;
type Registers = [RuntimeValue; MAX_REGISTERS];

pub struct VynVM {
    // Registers store actual RuntimeValues
    pub(crate) registers: Registers,
    // Constant pool
    pub(crate) constants: Vec<RuntimeValue>,
    // String table (since strings are stored by index)
    pub(crate) string_table: Vec<String>,
    // Program bytecode
    pub(crate) instructions: Instructions,
    // Instruction pointer
    pub(crate) ip: usize,
}

impl VynVM {
    pub fn new(
        instructions: Instructions,
        constants: Vec<RuntimeValue>,
        string_table: Vec<String>,
    ) -> Self {
        Self {
            registers: [NIL; MAX_REGISTERS], // Initialize all to Nil singleton
            constants,
            string_table,
            instructions,
            ip: 0,
        }
    }

    pub fn execute(&mut self) -> Result<(), VynError> {
        loop {
            let opcode = self.instructions[self.ip];

            match opcode {
                OpCode::HALT => {
                    return Ok(());
                }

                OpCode::LOAD_CONST_INT => {
                    self.load_constant()?;
                }
                OpCode::LOAD_CONST_FLOAT => {
                    self.load_constant()?;
                }
                OpCode::LOAD_STRING => {
                    self.load_string()?;
                }
                OpCode::LOAD_NIL => {
                    self.load_static(NIL)?;
                }
                OpCode::LOAD_TRUE => {
                    self.load_static(TRUE)?;
                }
                OpCode::LOAD_FALSE => {
                    self.load_static(FALSE)?;
                }

                OpCode::ADD_INT => {
                    self.arith_int(opcode)?;
                }
                OpCode::SUBTRACT_INT => {
                    self.arith_int(opcode)?;
                }
                OpCode::MULTIPLY_INT => {
                    self.arith_int(opcode)?;
                }
                OpCode::DIVIDE_INT => {
                    self.arith_int(opcode)?;
                }
                OpCode::EXPONENT_INT => {
                    self.arith_int(opcode)?;
                }

                OpCode::ADD_FLOAT => {
                    self.arith_float(opcode)?;
                }
                OpCode::SUBTRACT_FLOAT => {
                    self.arith_float(opcode)?;
                }
                OpCode::MULTIPLY_FLOAT => {
                    self.arith_float(opcode)?;
                }
                OpCode::DIVIDE_FLOAT => {
                    self.arith_float(opcode)?;
                }
                OpCode::EXPONENT_FLOAT => {
                    self.arith_float(opcode)?;
                }

                OpCode::CONCAT_STRING => {
                    self.concat_string()?;
                }

                OpCode::NEGATE_INT => {
                    self.negate_int()?;
                }
                OpCode::NEGATE_FLOAT => {
                    self.negate_float()?;
                }
                OpCode::NOT => {
                    self.bool_not()?;
                }

                OpCode::LESS_INT => {
                    self.compare_int(opcode)?;
                }
                OpCode::LESS_EQUAL_INT => {
                    self.compare_int(opcode)?;
                }
                OpCode::GREATER_INT => {
                    self.compare_int(opcode)?;
                }
                OpCode::GREATER_EQUAL_INT => {
                    self.compare_int(opcode)?;
                }

                OpCode::LESS_FLOAT => {
                    self.compare_float(opcode)?;
                }
                OpCode::LESS_EQUAL_FLOAT => {
                    self.compare_float(opcode)?;
                }
                OpCode::GREATER_FLOAT => {
                    self.compare_float(opcode)?;
                }
                OpCode::GREATER_EQUAL_FLOAT => {
                    self.compare_float(opcode)?;
                }

                OpCode::EQUAL => {
                    self.compare_equality(opcode)?;
                }
                OpCode::NOT_EQUAL => {
                    self.compare_equality(opcode)?;
                }

                OpCode::JUMP_IF_FALSE => {
                    let cond_reg_idx = read_uint8(&self.instructions, self.ip + 1);
                    let jump_idx = read_uint16(&self.instructions, self.ip + 2);
                    self.ip += 3;

                    let cond_reg = self.get_register(cond_reg_idx as usize);

                    if !self.is_truthy(cond_reg) {
                        self.ip = jump_idx as usize;
                        continue;
                    }
                }

                OpCode::JUMP_UNCOND => {
                    let jump_idx = read_uint16(&self.instructions, self.ip + 1);
                    self.ip = jump_idx as usize;
                    continue;
                }

                OpCode::STORE_GLOBAL => {
                    unreachable!() // no scopes yet
                }
                OpCode::LOAD_GLOBAL => {
                    unreachable!() // no scopes yet
                }

                OpCode::MOVE => {
                    let dest = read_uint8(&self.instructions, self.ip + 1) as usize;
                    let src = read_uint8(&self.instructions, self.ip + 2) as usize;
                    self.ip += 2;

                    let value = self.get_register(src);
                    self.set_register(dest, value);
                }

                OpCode::LOG_ADDR => {
                    let src = read_uint8(&self.instructions, self.ip + 1) as usize;
                    self.ip += 1;

                    let value = self.get_register(src);
                    let stdout = io::stdout();
                    let mut out = stdout.lock();

                    value.write_to(&mut out, &self.strings).unwrap();
                    out.write_all(b"\n").unwrap();
                }

                _ => unreachable!("Unknown opcode byte {}", opcode.to_opcode()),
            }

            self.ip += 1; // Advance past opcode
        }
    }

    #[inline(always)]
    pub(crate) fn set_register(&mut self, reg: usize, value: RuntimeValue) {
        self.registers[reg] = value
    }

    #[inline(always)]
    pub(crate) fn get_register(&self, reg: usize) -> RuntimeValue {
        self.registers[reg]
    }

    pub(crate) fn intern_string(&mut self, str: String) -> usize {
        self.strings.push(str);
        self.strings.len() - 1
    }

    // For debugging
    pub fn get_registers(&self) -> Vec<RuntimeValue> {
        // TODO: Maybe reformat this
        // this is kind of slow, but i guess its okay
        // for now since its just for debugging
        let mut occupied = Vec::with_capacity(MAX_REGISTERS);

        for reg in &self.registers {
            if reg.is_nil() {
                continue;
            }

            occupied.push(*reg);
        }

        occupied
    }

    pub fn get_string(&self, idx: usize) -> &str {
        &self.strings[idx]
    }
}
