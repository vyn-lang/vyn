use crate::error_handler::errors::VynError;

impl VynError {
    pub fn category(&self) -> &str {
        match self {
            VynError::UnexpectedToken { .. } => "Syntax",
            VynError::ExpectedToken { .. } => "Syntax",
            VynError::KeywordTypeError { .. } => "Syntax",
            VynError::InvalidTypeName { .. } => "Syntax",
            VynError::ExpectedType { .. } => "Syntax",
            VynError::RegisterOverflow { .. } => "Compiler",
            VynError::NotImplemented { .. } => "Compiler",

            VynError::TypeMismatch { .. } => "Type",
            VynError::InvalidUnaryOp { .. } => "Type",
            VynError::InvalidBinaryOp { .. } => "Type",
            VynError::DeclarationTypeMismatch { .. } => "Type",
            VynError::UndefinedVariable { .. } => "Type",
            VynError::VariableRedeclaration { .. } => "Type",
            VynError::TypeAliasRedeclaration { .. } => "Type",
            VynError::ImmutableMutation { .. } => "Type",
            VynError::LeftHandAssignment { .. } => "Type",
            VynError::InvalidIndexing { .. } => "Type",
            VynError::TypeInfer { .. } => "Type",
            VynError::ArrayLengthMismatch { .. } => "Type",

            VynError::IndexOutOfBounds { .. } => "Index",

            VynError::UnknownAST { .. } => "Compiler",
            VynError::UndefinedIdentifier { .. } => "Compiler",

            VynError::ArithmeticError { .. } => "Runtime",
            VynError::UnaryOperationError { .. } => "Runtime",
            VynError::ComparisonOperationError { .. } => "Runtime",

            VynError::DivisionByZero { .. } => "Math",
        }
    }
}
