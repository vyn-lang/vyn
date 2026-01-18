use core::fmt;

use byteorder::{BigEndian, ByteOrder};
use num_enum::IntoPrimitive;

#[derive(IntoPrimitive, Clone, Copy, Debug)]
#[repr(u8)]
pub enum OpCode {
    Halt = 0x01,

    // Load operations
    LoadConstInt = 0x03,   // LoadConstInt dest_reg, const_index
    LoadConstFloat = 0x04, // LoadConstFloat dest_reg, const_index
    LoadString = 0x05,     // LoadString dest_reg, string_index
    LoadNil = 0x06,        // LoadNil dest_reg
    LoadTrue = 0x07,       // LoadTrue dest_reg
    LoadFalse = 0x08,      // LoadFalse dest_reg

    // Integer arithmetic
    AddInt = 0x10, // AddInt dest_reg, left_reg, right_reg
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
    ConcatString = 0x1A, // ConcatString dest_reg, left_reg, right_reg

    // Unary operations
    NegateInt = 0x20, // NegateInt dest_reg, src_reg
    NegateFloat = 0x21,
    Not = 0x22, // Not dest_reg, src_reg

    // Integer comparisons (stores bool as 0 or 1)
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
    Equal = 0x38, // Equal dest_reg, left_reg, right_reg
    NotEqual = 0x39,

    // Variable operations
    StoreGlobal = 0x40, // StoreGlobal var_index, src_reg
    LoadGlobal = 0x41,  // LoadGlobal dest_reg, var_index

    // Move operation
    Move = 0x50, // Move dest_reg, src_reg
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
    ///
    /// Format depends on instruction type:
    /// - 3-register ops (Add, Sub, etc): [opcode, dest, left, right]
    /// - 2-register ops (Negate, Not, Move): [opcode, dest, src]
    /// - Load constant: [opcode, dest, const_index_hi, const_index_lo]
    /// - Load immediate: [opcode, dest]
    pub fn make(opcode: OpCode, operands: Vec<usize>) -> Instructions {
        let definition = OpCode::get_definition(opcode);
        let mut instruction_length = 1; // 1 for opcode

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
                operands_width: vec![1, 2], // dest_reg (u8), const_index (u16)
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
                operands_width: vec![1], // dest_reg
            },
            OpCode::LoadTrue => Definition {
                name: "LOAD_TRUE",
                operands_width: vec![1],
            },
            OpCode::LoadFalse => Definition {
                name: "LOAD_FALSE",
                operands_width: vec![1],
            },

            // Integer arithmetic (3 registers: dest, left, right)
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

            // Unary operations (2 registers: dest, src)
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
                operands_width: vec![2, 1], // var_index (u16), src_reg (u8)
            },
            OpCode::LoadGlobal => Definition {
                name: "LOAD_GLOBAL",
                operands_width: vec![1, 2], // dest_reg (u8), var_index (u16)
            },

            // Move
            OpCode::Move => Definition {
                name: "MOVE",
                operands_width: vec![1, 1], // dest_reg, src_reg
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

            // Integer arithmetic
            0x10 => OpCode::AddInt,
            0x11 => OpCode::SubtractInt,
            0x12 => OpCode::MultiplyInt,
            0x13 => OpCode::DivideInt,
            0x14 => OpCode::ExponentInt,

            // Float arithmetic
            0x15 => OpCode::AddFloat,
            0x16 => OpCode::SubtractFloat,
            0x17 => OpCode::MultiplyFloat,
            0x18 => OpCode::DivideFloat,
            0x19 => OpCode::ExponentFloat,

            // String operations
            0x1A => OpCode::ConcatString,

            // Unary operations
            0x20 => OpCode::NegateInt,
            0x21 => OpCode::NegateFloat,
            0x22 => OpCode::Not,

            // Integer comparisons
            0x30 => OpCode::LessInt,
            0x31 => OpCode::LessEqualInt,
            0x32 => OpCode::GreaterInt,
            0x33 => OpCode::GreaterEqualInt,

            // Float comparisons
            0x34 => OpCode::LessFloat,
            0x35 => OpCode::LessEqualFloat,
            0x36 => OpCode::GreaterFloat,
            0x37 => OpCode::GreaterEqualFloat,

            // General equality
            0x38 => OpCode::Equal,
            0x39 => OpCode::NotEqual,

            // Variable operations
            0x40 => OpCode::StoreGlobal,
            0x41 => OpCode::LoadGlobal,

            // Move
            0x50 => OpCode::Move,

            _ => unreachable!("Cannot convert byte '0x{:02X}' to an opcode", self),
        }
    }
}

pub fn read_uint8(instructions: &Instructions, offset: usize) -> u8 {
    instructions[offset]
}

pub fn read_uint16(instructions: &Instructions, offset: usize) -> u16 {
    BigEndian::read_u16(&instructions[offset..offset + 2])
}
