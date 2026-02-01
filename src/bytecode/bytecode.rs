use byteorder::{BigEndian, ByteOrder};
use core::fmt;

macro_rules! define_opcodes {
    (
        $(
            $variant:ident, $constant:ident = $value:expr
        ),* $(,)?
    ) => {
        #[derive(Clone, Copy, Debug)]
        #[repr(u8)]
        pub enum OpCode {
            $(
                $variant = $value,
            )*
        }

        impl OpCode {
            $(
                pub const $constant: u8 = $value;
            )*
        }

        impl ToOpcode for u8 {
            fn to_opcode(self) -> OpCode {
                match self {
                    $(
                        $value => OpCode::$variant,
                    )*
                    _ => unreachable!("Cannot convert byte '0x{:02X}' to an opcode", self),
                }
            }
        }
    };
}

// Usage: EnumVariant, CONSTANT_NAME = value
// Enum variant is used at comp time
// whilst constant name is used in vm for faster
// bytecode dispatch
define_opcodes! {
    Halt, HALT = 0x01,

    LoadConstInt, LOAD_CONST_INT = 0x03,
    LoadConstFloat, LOAD_CONST_FLOAT = 0x04,
    LoadString, LOAD_STRING = 0x05,
    LoadNil, LOAD_NIL = 0x06,
    LoadTrue, LOAD_TRUE = 0x07,
    LoadFalse, LOAD_FALSE = 0x08,

    AddInt, ADD_INT = 0x10,
    SubtractInt, SUBTRACT_INT = 0x11,
    MultiplyInt, MULTIPLY_INT = 0x12,
    DivideInt, DIVIDE_INT = 0x13,
    ExponentInt, EXPONENT_INT = 0x14,

    AddFloat, ADD_FLOAT = 0x15,
    SubtractFloat, SUBTRACT_FLOAT = 0x16,
    MultiplyFloat, MULTIPLY_FLOAT = 0x17,
    DivideFloat, DIVIDE_FLOAT = 0x18,
    ExponentFloat, EXPONENT_FLOAT = 0x19,

    ConcatString, CONCAT_STRING = 0x1A,

    NegateInt, NEGATE_INT = 0x20,
    NegateFloat, NEGATE_FLOAT = 0x21,
    Not, NOT = 0x22,

    LessInt, LESS_INT = 0x30,
    LessEqualInt, LESS_EQUAL_INT = 0x31,
    GreaterInt, GREATER_INT = 0x32,
    GreaterEqualInt, GREATER_EQUAL_INT = 0x33,

    LessFloat, LESS_FLOAT = 0x34,
    LessEqualFloat, LESS_EQUAL_FLOAT = 0x35,
    GreaterFloat, GREATER_FLOAT = 0x36,
    GreaterEqualFloat, GREATER_EQUAL_FLOAT = 0x37,

    Equal, EQUAL = 0x38,
    NotEqual, NOT_EQUAL = 0x39,

    StoreGlobal, STORE_GLOBAL = 0x40,
    LoadGlobal, LOAD_GLOBAL = 0x41,

    Move, MOVE = 0x50,
    LogAddr, LOG_ADDR = 0x51,
    JumpIfFalse, JUMP_IF_FALSE = 0x52,
    JumpUncond, JUMP_UNCOND = 0x53,

    ArrayNewFixed, ARRAY_NEW_FIXED = 0x54,
    ArrayNewDynamic, ARRAY_NEW_DYNAMIC = 0x55,
    ArraySet, ARRAY_SET = 0x56,
    ArraySetReg, ARRAY_SET_REG = 0x57,
    ArrayGet, ARRAY_GET = 0x58,
    ArrayPush, ARRAY_PUSH = 0x59,
}

impl From<OpCode> for u8 {
    fn from(op: OpCode) -> u8 {
        op as u8
    }
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
    pub fn make(opcode: OpCode, operands: Vec<usize>) -> Instructions {
        let definition = OpCode::get_definition(opcode);
        let mut instruction_length = 1;

        for width in definition.operands_width.iter() {
            instruction_length += width;
        }

        let mut instructions: Instructions = vec![0; instruction_length];
        instructions[0] = opcode as u8;

        let mut offset = 1;
        for (i, operand) in operands.iter().enumerate() {
            let width = definition.operands_width[i];

            match width {
                1 => instructions[offset] = *operand as u8,
                2 => BigEndian::write_u16(&mut instructions[offset..], *operand as u16),
                4 => BigEndian::write_u32(&mut instructions[offset..], *operand as u32),
                _ => unreachable!("Cannot make instruction operand with width {width}"),
            }

            offset += width;
        }

        instructions
    }

    pub fn change_operand(
        instructions: &mut Instructions,
        position: usize,
        new_operands: Vec<usize>,
    ) {
        let opcode = instructions[position].to_opcode();
        let definition = OpCode::get_definition(opcode);

        let mut offset = position + 1;

        for (i, width) in definition.operands_width.iter().enumerate() {
            match width {
                1 => instructions[offset] = new_operands[i] as u8,
                2 => BigEndian::write_u16(&mut instructions[offset..], new_operands[i] as u16),
                4 => BigEndian::write_u32(&mut instructions[offset..], new_operands[i] as u32),
                _ => unreachable!("Cannot change operand with width {}", width),
            }
            offset += width;
        }
    }

    pub fn get_definition(opcode: OpCode) -> Definition {
        match opcode {
            OpCode::Halt => Definition {
                name: "HALT",
                operands_width: vec![],
            },
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
            OpCode::ConcatString => Definition {
                name: "CONCAT_STRING",
                operands_width: vec![1, 1, 1],
            },
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
            OpCode::Equal => Definition {
                name: "EQUAL",
                operands_width: vec![1, 1, 1],
            },
            OpCode::NotEqual => Definition {
                name: "NOT_EQUAL",
                operands_width: vec![1, 1, 1],
            },
            OpCode::StoreGlobal => Definition {
                name: "STORE_GLOBAL",
                operands_width: vec![2, 1],
            },
            OpCode::LoadGlobal => Definition {
                name: "LOAD_GLOBAL",
                operands_width: vec![1, 2],
            },
            OpCode::Move => Definition {
                name: "MOVE",
                operands_width: vec![1, 1],
            },
            OpCode::LogAddr => Definition {
                name: "LOG_ADDR",
                operands_width: vec![1],
            },
            OpCode::JumpIfFalse => Definition {
                name: "JUMP_IF_FALSE",
                operands_width: vec![1, 2],
            },
            OpCode::JumpUncond => Definition {
                name: "JUMP_UNCOND",
                operands_width: vec![2],
            },
            OpCode::ArrayNewFixed => Definition {
                name: "ARRAY_NEW_FIXED",
                operands_width: vec![1, 4],
            },
            OpCode::ArrayNewDynamic => Definition {
                name: "ARRAY_NEW_DYNAMIC",
                operands_width: vec![1, 4], // dest,
            },
            OpCode::ArraySet => Definition {
                name: "ARRAY_SET",
                operands_width: vec![1, 4, 1], // array_reg, index_u32, value_reg
            },
            OpCode::ArraySetReg => Definition {
                name: "ARRAY_SET_REG",
                operands_width: vec![1, 1, 1], // array_reg, index_reg, value_reg
            },
            OpCode::ArrayGet => Definition {
                name: "ARRAY_GET",
                operands_width: vec![1, 1, 1], // dest_reg, array_reg, index_reg
            },
            OpCode::ArrayPush => Definition {
                name: "ARRAY_PUSH",
                operands_width: vec![1, 1], // array_reg, value_reg
            },
        }
    }
}

pub trait ToOpcode {
    fn to_opcode(self) -> OpCode;
}

#[inline]
pub fn read_uint8(instructions: &Instructions, offset: usize) -> u8 {
    instructions[offset]
}

#[inline]
pub fn read_uint16(instructions: &Instructions, offset: usize) -> u16 {
    BigEndian::read_u16(&instructions[offset..offset + 2])
}

#[inline]
pub fn read_uint32(instructions: &Instructions, offset: usize) -> u32 {
    BigEndian::read_u32(&instructions[offset..offset + 4])
}
