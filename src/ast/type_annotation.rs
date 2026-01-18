use core::fmt;
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, PartialEq)]
pub enum TypeAnnotation {
    StringType,
    IntegerType,
    FloatType,
    BooleanType,
}

impl Display for TypeAnnotation {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            TypeAnnotation::StringType => write!(f, "String"),
            TypeAnnotation::IntegerType => write!(f, "Integer"),
            TypeAnnotation::FloatType => write!(f, "Float"),
            TypeAnnotation::BooleanType => write!(f, "Bool"),
        }
    }
}

impl TypeAnnotation {
    pub fn from_identifier(name: &str) -> Option<Self> {
        match name {
            "Int" => Some(TypeAnnotation::IntegerType),
            "Float" => Some(TypeAnnotation::FloatType),
            "Bool" => Some(TypeAnnotation::BooleanType),
            "String" => Some(TypeAnnotation::StringType),
            _ => None,
        }
    }
}
