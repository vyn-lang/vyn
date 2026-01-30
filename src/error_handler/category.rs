use crate::error_handler::errors::VynError;

impl VynError {
    pub fn category(&self) -> &str {
        match self {
            // Syntax errors
            VynError::UnexpectedToken { .. } => "Syntax",
            VynError::ExpectedToken { .. } => "Syntax",
            VynError::KeywordTypeError { .. } => "Syntax",
            VynError::InvalidTypeName { .. } => "Syntax",
            VynError::ExpectedType { .. } => "Syntax",
            VynError::StaticRequiresConstant { .. } => "Syntax",
            VynError::IllegalLoopInterruptToken { .. } => "Syntax",

            // Type errors
            VynError::TypeMismatch { .. } => "Type",
            VynError::InvalidUnaryOp { .. } => "Type",
            VynError::InvalidBinaryOp { .. } => "Type",
            VynError::DeclarationTypeMismatch { .. } => "Type",
            VynError::UndefinedVariable { .. } => "Type",
            VynError::VariableRedeclaration { .. } => "Type",
            VynError::TypeAliasRedeclaration { .. } => "Type",
            VynError::ImmutableMutation { .. } => "Type",
            VynError::StaticMutation { .. } => "Type",
            VynError::LeftHandAssignment { .. } => "Type",
            VynError::InvalidIndexing { .. } => "Type",
            VynError::TypeInfer { .. } => "Type",
            VynError::ArrayLengthMismatch { .. } => "Type",
            VynError::InvalidUnaryOperator { .. } => "Type",
            VynError::InvalidBinaryOperator { .. } => "Type",

            // Static evaluation errors
            VynError::CircularStaticDependency { .. } => "StaticEval",
            VynError::UndefinedStatic { .. } => "StaticEval",
            VynError::StaticEvaluationFailed { .. } => "StaticEval",
            VynError::NotStaticExpression { .. } => "StaticEval",
            VynError::InvalidStaticOperation { .. } => "StaticEval",
            VynError::StaticOverflow { .. } => "StaticEval",
            VynError::NegativeExponent { .. } => "StaticEval",
            VynError::NegativeArraySize { .. } => "StaticEval",
            VynError::ArraySizeNotStatic { .. } => "StaticEval",

            // Index errors
            VynError::IndexOutOfBounds { .. } => "Index",

            // Compiler errors
            VynError::RegisterOverflow { .. } => "Compiler",
            VynError::NotImplemented { .. } => "Compiler",
            VynError::UnknownAST { .. } => "Compiler",
            VynError::UndefinedIdentifier { .. } => "Compiler",

            // Runtime errors
            VynError::ArithmeticError { .. } => "Runtime",
            VynError::UnaryOperationError { .. } => "Runtime",
            VynError::ComparisonOperationError { .. } => "Runtime",
            VynError::DivisionByZero { .. } => "Runtime",
        }
    }
}
