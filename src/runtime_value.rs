#[derive(Debug, Clone, Copy)]
pub enum RuntimeValue {
    IntegerLiteral(i32),
    FloatLiteral(f64),
    BooleanLiteral(bool),
    StringLiteral(usize), // Accessed via string table
}

impl RuntimeValue {
    pub fn type_name(&self) -> &'static str {
        match self {
            RuntimeValue::IntegerLiteral(_) => "integer",
            RuntimeValue::FloatLiteral(_) => "float",
            RuntimeValue::BooleanLiteral(_) => "boolean",
            RuntimeValue::StringLiteral(_) => "string",
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
}
