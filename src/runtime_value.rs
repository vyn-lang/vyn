use core::fmt;
use std::fmt::{Display, Formatter};

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RuntimeValue {
    IntegerLiteral(i32),
    FloatLiteral(f64),
    BooleanLiteral(bool),
    StringLiteral(usize), // Accessed via string table
    NilLiteral,
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum RuntimeType {
    Integer,
    Float,
    Boolean,
    String,
    Nil,
}

impl Display for RuntimeType {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            RuntimeType::Integer => write!(f, "Integer"),
            RuntimeType::Float => write!(f, "Float"),
            RuntimeType::Boolean => write!(f, "Boolean"),
            RuntimeType::String => write!(f, "String"),
            RuntimeType::Nil => write!(f, "Nil"),
        }
    }
}

impl RuntimeType {
    pub fn to_string(&self) -> &'static str {
        match self {
            RuntimeType::Integer => "integer",
            RuntimeType::Float => "float",
            RuntimeType::Boolean => "boolean",
            RuntimeType::String => "string",
            RuntimeType::Nil => "nil",
        }
    }
}

impl RuntimeValue {
    pub fn as_int(&self) -> Option<i32> {
        match self {
            RuntimeValue::IntegerLiteral(n) => Some(*n),
            _ => None,
        }
    }

    pub fn as_float(&self) -> Option<f64> {
        match self {
            RuntimeValue::FloatLiteral(n) => Some(*n),
            _ => None,
        }
    }

    pub fn as_bool(&self) -> Option<bool> {
        match self {
            RuntimeValue::BooleanLiteral(b) => Some(*b),
            _ => None,
        }
    }

    pub fn as_string_index(&self) -> Option<usize> {
        match self {
            RuntimeValue::StringLiteral(i) => Some(*i),
            _ => None,
        }
    }

    pub fn is_nil(&self) -> bool {
        matches!(self, RuntimeValue::NilLiteral)
    }

    pub fn as_number(&self) -> Option<f64> {
        match self {
            RuntimeValue::IntegerLiteral(n) => Some(*n as f64),
            RuntimeValue::FloatLiteral(n) => Some(*n),
            _ => None,
        }
    }

    pub fn get_type(&self) -> RuntimeType {
        match self {
            RuntimeValue::IntegerLiteral(_) => RuntimeType::Integer,
            RuntimeValue::FloatLiteral(_) => RuntimeType::Float,
            RuntimeValue::BooleanLiteral(_) => RuntimeType::Boolean,
            RuntimeValue::StringLiteral(_) => RuntimeType::String,
            RuntimeValue::NilLiteral => RuntimeType::Nil,
        }
    }

    pub fn is_number(&self) -> bool {
        matches!(
            self,
            RuntimeValue::IntegerLiteral(_) | RuntimeValue::FloatLiteral(_)
        )
    }

    pub fn is_float(&self) -> bool {
        matches!(self, RuntimeValue::FloatLiteral(_))
    }

    pub fn is_string(&self) -> bool {
        matches!(self, RuntimeValue::StringLiteral(_))
    }
}
