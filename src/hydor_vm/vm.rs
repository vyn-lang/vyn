use crate::{
    bytecode::bytecode::{Instructions, OpCode, ToOpcode, read_uint16},
    compiler::compiler::{Bytecode, DebugInfo},
    errors::HydorError,
    runtime_value::RuntimeValue,
    utils::Span,
};

const MAX_STACK: usize = 10_000;
const GLOBAL_STACK: usize = 4028;

pub struct HydorVM {
    stack: Vec<StackValue>,
    globals: Vec<StackValue>,
    last_pop: Option<RuntimeValue>,

    pub instructions: Instructions,
    pub ip: usize,

    pub string_table: Vec<String>,
    pub constants: Vec<RuntimeValue>,

    debug_info: DebugInfo,
}

#[derive(Debug, Clone, Copy)]
struct StackValue {
    value: RuntimeValue,
    span: Span,
}

// SINGLETON ---
pub const BOOLEAN_TRUE: RuntimeValue = RuntimeValue::BooleanLiteral(true);
pub const BOOLEAN_FALSE: RuntimeValue = RuntimeValue::BooleanLiteral(false);
pub const NIL_LITERAL: RuntimeValue = RuntimeValue::NilLiteral;

impl HydorVM {
    pub fn new(bytecode: Bytecode) -> Self {
        Self {
            stack: Vec::with_capacity(MAX_STACK),
            globals: Vec::new(), // Start empty
            last_pop: None,

            string_table: bytecode.string_table,
            instructions: bytecode.instructions,
            ip: 0,

            constants: bytecode.constants,
            debug_info: bytecode.debug_info,
        }
    }

    /// Main entry point
    pub fn execute_bytecode(&mut self) -> Result<(), HydorError> {
        while self.ip < self.instructions.len() {
            let opcode = self.instructions[self.ip].to_opcode();
            let span = self.debug_info.get_span(self.ip);

            match opcode {
                OpCode::LoadConstant => self.load_constant(span)?,
                OpCode::LoadString => self.load_string(span)?,
                OpCode::LoadNil => self.push(NIL_LITERAL, span)?,
                OpCode::LoadBoolTrue => self.push(BOOLEAN_TRUE, span)?,
                OpCode::LoadBoolFalse => self.push(BOOLEAN_FALSE, span)?,

                OpCode::AddInt | OpCode::AddFloat => self.binary_op(opcode, span)?,
                OpCode::SubtractInt | OpCode::SubtractFloat => self.binary_op(opcode, span)?,
                OpCode::MultiplyInt | OpCode::MultiplyFloat => self.binary_op(opcode, span)?,
                OpCode::DivideInt | OpCode::DivideFloat => self.binary_op(opcode, span)?,
                OpCode::ExponentInt | OpCode::ExponentFloat => self.binary_op(opcode, span)?,

                OpCode::ConcatString => self.string_concat(span)?,

                OpCode::UnaryNegateInt | OpCode::UnaryNegateFloat => {
                    self.unary_operation(opcode, span)?
                }
                OpCode::UnaryNot => self.unary_operation(opcode, span)?,

                OpCode::CompareLessInt
                | OpCode::CompareLessFloat
                | OpCode::CompareLessEqualInt
                | OpCode::CompareLessEqualFloat
                | OpCode::CompareGreaterInt
                | OpCode::CompareGreaterFloat
                | OpCode::CompareGreaterEqualInt
                | OpCode::CompareGreaterEqualFloat
                | OpCode::CompareEqual
                | OpCode::CompareNotEqual => self.compare_operation(opcode, span)?,

                OpCode::LoadGlobal => {
                    let global_index = read_uint16(&self.instructions, self.ip + 1);
                    self.ip += 2;

                    let global = self.get_global(global_index as usize);
                    self.push(global.value, span)?;
                }

                OpCode::DeclareGlobal => {
                    let global_idx = read_uint16(&self.instructions, self.ip + 1);
                    self.ip += 2;

                    let (value, span) = self.pop_with_span()?;
                    self.add_global(global_idx as usize, StackValue { value, span })?;
                }

                OpCode::Pop => {
                    self.last_pop = Some(self.pop_value()?);
                }
                OpCode::Halt => return Ok(()),
                _ => unreachable!(),
            }

            self.ip += 1;
        }

        unreachable!()
    }

    #[inline(always)]
    pub(crate) fn push(&mut self, value: RuntimeValue, span: Span) -> Result<(), HydorError> {
        if self.stack.len() >= MAX_STACK {
            return Err(HydorError::OperandStackOverflow {
                stack_length: self.stack.len(),
                span,
            });
        }

        self.stack.push(StackValue { value, span });
        Ok(())
    }

    #[inline(always)]
    pub(crate) fn add_global(&mut self, idx: usize, value: StackValue) -> Result<(), HydorError> {
        if idx >= GLOBAL_STACK {
            return Err(HydorError::GlobalStackOverflow {
                stack_length: self.globals.len(),
                max: GLOBAL_STACK,
                span: value.span,
            });
        }

        // Grow lazily only when needed
        if idx >= self.globals.len() {
            self.globals.resize(
                idx + 1,
                StackValue {
                    value: NIL_LITERAL,
                    span: Span::default(),
                },
            );
        }

        self.globals[idx] = value;
        Ok(())
    }

    #[inline(always)]
    pub(crate) fn get_global(&self, idx: usize) -> StackValue {
        self.globals[idx]
    }

    pub(crate) fn peek_offset(&self, n: usize) -> Result<RuntimeValue, HydorError> {
        let size = self.stack.len();
        if n >= size {
            return Err(HydorError::OperandStackUnderflow {
                stack_length: MAX_STACK,
                span: Span::default(),
            });
        }

        Ok(self.stack[size - 1 - n].value)
    }

    pub(crate) fn peek_span(&self, n: usize) -> Result<Span, HydorError> {
        let size = self.stack.len();
        if n >= size {
            return Err(HydorError::OperandStackUnderflow {
                stack_length: size,
                span: Span::default(),
            });
        }

        Ok(self.stack[size - 1 - n].span)
    }

    pub(crate) fn set_offset_value(
        &mut self,
        n: usize,
        new_value: RuntimeValue,
    ) -> Result<(), HydorError> {
        let size = self.stack.len();
        if n >= size {
            return Err(HydorError::OperandStackUnderflow {
                stack_length: size,
                span: Span::default(),
            });
        }

        self.stack[size - 1 - n].value = new_value;
        Ok(())
    }

    #[inline(always)]
    pub(crate) fn pop_value(&mut self) -> Result<RuntimeValue, HydorError> {
        self.stack
            .pop()
            .map(|sv| sv.value)
            .ok_or(HydorError::OperandStackUnderflow {
                stack_length: 0,
                span: Span::default(),
            })
    }

    #[inline(always)]
    pub(crate) fn pop_with_span(&mut self) -> Result<(RuntimeValue, Span), HydorError> {
        self.stack
            .pop()
            .map(|sv| (sv.value, sv.span))
            .ok_or(HydorError::OperandStackUnderflow {
                stack_length: 0,
                span: Span::default(),
            })
    }

    pub fn resolve_string(&self, index: usize) -> &str {
        &self.string_table[index]
    }

    pub fn last_popped(&self) -> Option<RuntimeValue> {
        self.last_pop
    }
}
