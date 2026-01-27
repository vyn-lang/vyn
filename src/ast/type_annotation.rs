use core::fmt;
use std::fmt::{Display, Formatter};

use crate::ast::ast::Expression;

#[derive(Debug, Clone, PartialEq)]
pub enum TypeAnnotation {
    StringType,
    IntegerType,
    FloatType,
    BooleanType,
    FixedArrayType(Box<TypeAnnotation>, Expression),
    DynamicArrayType(Box<TypeAnnotation>),
}

impl Display for TypeAnnotation {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            TypeAnnotation::StringType => write!(f, "String"),
            TypeAnnotation::IntegerType => write!(f, "Integer"),
            TypeAnnotation::FloatType => write!(f, "Float"),
            TypeAnnotation::BooleanType => write!(f, "Bool"),

            TypeAnnotation::FixedArrayType(ta, s) => write!(f, "[{}]{}", s, ta),
            TypeAnnotation::DynamicArrayType(ta) => write!(f, "[]{}", ta),
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
