#[derive(Debug, Clone, PartialEq)]
pub enum TypeAnnotation {
    StringType,
    IntegerType,
    FloatType,
    BooleanType,
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
