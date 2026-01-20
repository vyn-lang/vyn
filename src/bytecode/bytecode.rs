use core::fmt;

use byteorder::{BigEndian, ByteOrder};
use num_enum::IntoPrimitive;

#[derive(IntoPrimitive, Clone, Copy, Debug)]
#[repr(u8)]
pub enum OpCode {
    Halt = 0x01,

    // Load operations
    LoadConstInt = 0x03,
    LoadConstFloat = 0x04,
    LoadString = 0x05,
    LoadNil = 0x06,
    LoadTrue = 0x07,
    LoadFalse = 0x08,

    // Integer arithmetic
    AddInt = 0x10,
    SubtractInt = 0x11,
    MultiplyInt = 0x12,
    DivideInt = 0x13,
    ExponentInt = 0x14,

    // Float arithmetic
    AddFloat = 0x15,
    SubtractFloat = 0x16,
    MultiplyFloat = 0x17,
    DivideFloat = 0x18,
    ExponentFloat = 0x19,

    // String operations
    ConcatString = 0x1A,

    // Unary operations
    NegateInt = 0x20,
    NegateFloat = 0x21,
    Not = 0x22,

    // Integer comparisons
    LessInt = 0x30,
    LessEqualInt = 0x31,
    GreaterInt = 0x32,
    GreaterEqualInt = 0x33,

    // Float comparisons
    LessFloat = 0x34,
    LessEqualFloat = 0x35,
    GreaterFloat = 0x36,
    GreaterEqualFloat = 0x37,

    // General equality
    Equal = 0x38,
    NotEqual = 0x39,

    // Variable operations
    StoreGlobal = 0x40,
    LoadGlobal = 0x41,

    // Move operation
    Move = 0x50,
}

impl OpCode {
    // Opcode byte constants for fast VM dispatch
    pub const HALT: u8 = 0x01;

    pub const LOAD_CONST_INT: u8 = 0x03;
    pub const LOAD_CONST_FLOAT: u8 = 0x04;
    pub const LOAD_STRING: u8 = 0x05;
    pub const LOAD_NIL: u8 = 0x06;
    pub const LOAD_TRUE: u8 = 0x07;
    pub const LOAD_FALSE: u8 = 0x08;

    pub const ADD_INT: u8 = 0x10;
    pub const SUBTRACT_INT: u8 = 0x11;
    pub const MULTIPLY_INT: u8 = 0x12;
    pub const DIVIDE_INT: u8 = 0x13;
    pub const EXPONENT_INT: u8 = 0x14;

    pub const ADD_FLOAT: u8 = 0x15;
    pub const SUBTRACT_FLOAT: u8 = 0x16;
    pub const MULTIPLY_FLOAT: u8 = 0x17;
    pub const DIVIDE_FLOAT: u8 = 0x18;
    pub const EXPONENT_FLOAT: u8 = 0x19;

    pub const CONCAT_STRING: u8 = 0x1A;

    pub const NEGATE_INT: u8 = 0x20;
    pub const NEGATE_FLOAT: u8 = 0x21;
    pub const NOT: u8 = 0x22;

    pub const LESS_INT: u8 = 0x30;
    pub const LESS_EQUAL_INT: u8 = 0x31;
    pub const GREATER_INT: u8 = 0x32;
    pub const GREATER_EQUAL_INT: u8 = 0x33;

    pub const LESS_FLOAT: u8 = 0x34;
    pub const LESS_EQUAL_FLOAT: u8 = 0x35;
    pub const GREATER_FLOAT: u8 = 0x36;
    pub const GREATER_EQUAL_FLOAT: u8 = 0x37;

    pub const EQUAL: u8 = 0x38;
    pub const NOT_EQUAL: u8 = 0x39;

    pub const STORE_GLOBAL: u8 = 0x40;
    pub const LOAD_GLOBAL: u8 = 0x41;

    pub const MOVE: u8 = 0x50;
}

impl fmt::Display for OpCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            OpCode::AddInt | OpCode::AddFloat => "+",
            OpCode::SubtractInt | OpCode::SubtractFloat => "-",
            OpCode::MultiplyInt | OpCode::MultiplyFloat => "*",
            OpCode::DivideInt | OpCode::DivideFloat => "/",
            OpCode::ExponentInt | OpCode::ExponentFloat => "^",
            OpCode::NegateInt | OpCode::NegateFloat => "-",
            OpCode::Not => "not",
            OpCode::LessInt | OpCode::LessFloat => "<",
            OpCode::LessEqualInt | OpCode::LessEqualFloat => "<=",
            OpCode::GreaterInt | OpCode::GreaterFloat => ">",
            OpCode::GreaterEqualInt | OpCode::GreaterEqualFloat => ">=",
            OpCode::Equal => "==",
            OpCode::NotEqual => "!=",
            OpCode::ConcatString => "+",
            _ => return write!(f, "{:?}", self),
        };
        write!(f, "{}", s)
    }
}

pub struct Definition {
    pub name: &'static str,
    pub operands_width: Vec<usize>,
}

pub type Instructions = Vec<u8>;

impl OpCode {
    /// Create a new instruction with operands
    pub fn make(opcode: OpCode, operands: Vec<usize>) -> Instructions {
        let definition = OpCode::get_definition(opcode);
        let mut instruction_length = 1;

        for width in definition.operands_width.iter() {
            instruction_length += width;
        }

        let mut instructions: Instructions = vec![0; instruction_length];
        instructions[0] = opcode.into();

        let mut offset = 1;
        for (i, operand) in operands.iter().enumerate() {
            let width = definition.operands_width[i];

            match width {
                1 => instructions[offset] = *operand as u8,
                2 => BigEndian::write_u16(&mut instructions[offset..], *operand as u16),
                _ => unreachable!("Cannot make instruction operand with width {width}"),
            }

            offset += width;
        }

        instructions
    }

    pub fn get_definition(opcode: OpCode) -> Definition {
        match opcode {
            OpCode::Halt => Definition {
                name: "HALT",
                operands_width: vec![],
            },

            // Load operations
            OpCode::LoadConstInt => Definition {
                name: "LOAD_CONST_INT",
                operands_width: vec![1, 2],
            },
            OpCode::LoadConstFloat => Definition {
                name: "LOAD_CONST_FLOAT",
                operands_width: vec![1, 2],
            },
            OpCode::LoadString => Definition {
                name: "LOAD_STRING",
                operands_width: vec![1, 2],
            },
            OpCode::LoadNil => Definition {
                name: "LOAD_NIL",
                operands_width: vec![1],
            },
            OpCode::LoadTrue => Definition {
                name: "LOAD_TRUE",
                operands_width: vec![1],
            },
            OpCode::LoadFalse => Definition {
                name: "LOAD_FALSE",
                operands_width: vec![1],
            },

            // Integer arithmetic
            OpCode::AddInt => Definition {
                name: "ADD_INT",
                operands_width: vec![1, 1, 1],
            },
            OpCode::SubtractInt => Definition {
                name: "SUB_INT",
                operands_width: vec![1, 1, 1],
            },
            OpCode::MultiplyInt => Definition {
                name: "MUL_INT",
                operands_width: vec![1, 1, 1],
            },
            OpCode::DivideInt => Definition {
                name: "DIV_INT",
                operands_width: vec![1, 1, 1],
            },
            OpCode::ExponentInt => Definition {
                name: "EXP_INT",
                operands_width: vec![1, 1, 1],
            },

            // Float arithmetic
            OpCode::AddFloat => Definition {
                name: "ADD_FLOAT",
                operands_width: vec![1, 1, 1],
            },
            OpCode::SubtractFloat => Definition {
                name: "SUB_FLOAT",
                operands_width: vec![1, 1, 1],
            },
            OpCode::MultiplyFloat => Definition {
                name: "MUL_FLOAT",
                operands_width: vec![1, 1, 1],
            },
            OpCode::DivideFloat => Definition {
                name: "DIV_FLOAT",
                operands_width: vec![1, 1, 1],
            },
            OpCode::ExponentFloat => Definition {
                name: "EXP_FLOAT",
                operands_width: vec![1, 1, 1],
            },

            // String operations
            OpCode::ConcatString => Definition {
                name: "CONCAT_STRING",
                operands_width: vec![1, 1, 1],
            },

            // Unary operations
            OpCode::NegateInt => Definition {
                name: "NEGATE_INT",
                operands_width: vec![1, 1],
            },
            OpCode::NegateFloat => Definition {
                name: "NEGATE_FLOAT",
                operands_width: vec![1, 1],
            },
            OpCode::Not => Definition {
                name: "NOT",
                operands_width: vec![1, 1],
            },

            // Integer comparisons
            OpCode::LessInt => Definition {
                name: "LESS_INT",
                operands_width: vec![1, 1, 1],
            },
            OpCode::LessEqualInt => Definition {
                name: "LESS_EQUAL_INT",
                operands_width: vec![1, 1, 1],
            },
            OpCode::GreaterInt => Definition {
                name: "GREATER_INT",
                operands_width: vec![1, 1, 1],
            },
            OpCode::GreaterEqualInt => Definition {
                name: "GREATER_EQUAL_INT",
                operands_width: vec![1, 1, 1],
            },

            // Float comparisons
            OpCode::LessFloat => Definition {
                name: "LESS_FLOAT",
                operands_width: vec![1, 1, 1],
            },
            OpCode::LessEqualFloat => Definition {
                name: "LESS_EQUAL_FLOAT",
                operands_width: vec![1, 1, 1],
            },
            OpCode::GreaterFloat => Definition {
                name: "GREATER_FLOAT",
                operands_width: vec![1, 1, 1],
            },
            OpCode::GreaterEqualFloat => Definition {
                name: "GREATER_EQUAL_FLOAT",
                operands_width: vec![1, 1, 1],
            },

            // General equality
            OpCode::Equal => Definition {
                name: "EQUAL",
                operands_width: vec![1, 1, 1],
            },
            OpCode::NotEqual => Definition {
                name: "NOT_EQUAL",
                operands_width: vec![1, 1, 1],
            },

            // Variable operations
            OpCode::StoreGlobal => Definition {
                name: "STORE_GLOBAL",
                operands_width: vec![2, 1],
            },
            OpCode::LoadGlobal => Definition {
                name: "LOAD_GLOBAL",
                operands_width: vec![1, 2],
            },

            // Move
            OpCode::Move => Definition {
                name: "MOVE",
                operands_width: vec![1, 1],
            },
        }
    }
}

pub trait ToOpcode {
    fn to_opcode(self) -> OpCode;
}

impl ToOpcode for u8 {
    fn to_opcode(self) -> OpCode {
        match self {
            0x01 => OpCode::Halt,
            0x03 => OpCode::LoadConstInt,
            0x04 => OpCode::LoadConstFloat,
            0x05 => OpCode::LoadString,
            0x06 => OpCode::LoadNil,
            0x07 => OpCode::LoadTrue,
            0x08 => OpCode::LoadFalse,
            0x10 => OpCode::AddInt,
            0x11 => OpCode::SubtractInt,
            0x12 => OpCode::MultiplyInt,
            0x13 => OpCode::DivideInt,
            0x14 => OpCode::ExponentInt,
            0x15 => OpCode::AddFloat,
            0x16 => OpCode::SubtractFloat,
            0x17 => OpCode::MultiplyFloat,
            0x18 => OpCode::DivideFloat,
            0x19 => OpCode::ExponentFloat,
            0x1A => OpCode::ConcatString,
            0x20 => OpCode::NegateInt,
            0x21 => OpCode::NegateFloat,
            0x22 => OpCode::Not,
            0x30 => OpCode::LessInt,
            0x31 => OpCode::LessEqualInt,
            0x32 => OpCode::GreaterInt,
            0x33 => OpCode::GreaterEqualInt,
            0x34 => OpCode::LessFloat,
            0x35 => OpCode::LessEqualFloat,
            0x36 => OpCode::GreaterFloat,
            0x37 => OpCode::GreaterEqualFloat,
            0x38 => OpCode::Equal,
            0x39 => OpCode::NotEqual,
            0x40 => OpCode::StoreGlobal,
            0x41 => OpCode::LoadGlobal,
            0x50 => OpCode::Move,
            _ => unreachable!("Cannot convert byte '0x{:02X}' to an opcode", self),
        }
    }
}

#[inline]
pub fn read_uint8(instructions: &Instructions, offset: usize) -> u8 {
    instructions[offset]
}

#[inline]
pub fn read_uint16(instructions: &Instructions, offset: usize) -> u16 {
    BigEndian::read_u16(&instructions[offset..offset + 2])
}
