use crate::{ast::ast::Node, error_handler::errors::VynError};

impl VynError {
    pub fn message(&self) -> String {
        match self {
            VynError::UnexpectedToken { token, .. } => {
                format!("Unexpected token '{}'", token)
            }
            VynError::ExpectedToken { expected, got, .. } => {
                format!("Expected '{}' but found '{}'", expected, got)
            }
            VynError::KeywordTypeError { got, .. } => {
                format!("'{}' is a keyword and cannot be used as a type name", got)
            }
            VynError::RegisterOverflow { .. } => {
                "Register overflow: expression is too complex".to_string()
            }
            VynError::NotImplemented { feature, .. } => {
                format!("Feature not yet implemented: {}", feature)
            }
            VynError::InvalidTypeName { got, .. } => {
                format!("'{}' is not a valid type", got)
            }
            VynError::ExpectedType { got, .. } => {
                format!("Expected type annotation, got '{got}' instead")
            }
            VynError::TypeInfer { expr, .. } => {
                format!("Cannot infer type of expression '{expr}'")
            }
            VynError::StaticMutation { identifier, .. } => {
                format!("Cannot mutate static identifier '{identifier}'")
            }
            VynError::StaticRequiresConstant { .. } => {
                format!("Cannot use value as a static value")
            }
            VynError::ArrayLengthMismatch { expected, got, .. } => {
                format!(
                    "Array length mismatch, expected length '[{expected}]' but got '[{got}]' instead"
                )
            }
            VynError::DivisionByZero { .. } => "Cannot divide by zero".to_string(),
            VynError::TypeAliasRedeclaration { name, .. } => {
                format!(
                    "Cannot redeclare type alias '{}' in the current scope",
                    name
                )
            }
            VynError::DeclarationTypeMismatch { got, expected, .. } => {
                format!(
                    "Type mismatch in variable declaration: expected '{}', got '{}'",
                    expected, got
                )
            }
            VynError::TypeMismatch {
                expected, found, ..
            } => {
                if expected.len() > 1 {
                    let expected_types = expected
                        .iter()
                        .map(|t| format!("'{}'", t))
                        .collect::<Vec<String>>()
                        .join(" or ");

                    format!(
                        "Type mismatch: expected {}, got '{}'",
                        expected_types, found
                    )
                } else {
                    format!("Type mismatch: expected '{}', got '{}'", expected[0], found)
                }
            }
            VynError::InvalidUnaryOp {
                operator,
                operand_type,
                ..
            } => {
                format!(
                    "Unary operator '{}' cannot be applied to type '{}'",
                    operator, operand_type
                )
            }
            VynError::InvalidBinaryOp {
                operator,
                left_type,
                right_type,
                ..
            } => {
                if left_type == right_type {
                    format!(
                        "Math operator '{}' is not supported for type '{}'",
                        operator, left_type
                    )
                } else {
                    format!(
                        "Math operator '{}' cannot be used between '{}' and '{}'",
                        operator, left_type, right_type
                    )
                }
            }
            VynError::CircularStaticDependency { name, .. } => {
                format!("Circular dependency detected in static variable '{}'", name)
            }
            VynError::UndefinedStatic { name, .. } => {
                format!(
                    "Undefined static variable '{}' in constant expression",
                    name
                )
            }
            VynError::StaticEvaluationFailed { name, .. } => {
                format!(
                    "Failed to evaluate static variable '{}' at compile time",
                    name
                )
            }
            VynError::NotStaticExpression { .. } => {
                "Expression is not a compile-time constant".to_string()
            }
            VynError::InvalidStaticOperation { operation, .. } => {
                format!(
                    "Operation '{}' is not allowed in static expressions",
                    operation
                )
            }
            VynError::StaticOverflow { .. } => {
                "Arithmetic overflow in static expression".to_string()
            }
            VynError::NegativeExponent { .. } => {
                "Cannot use negative exponent in compile-time expression".to_string()
            }
            VynError::NegativeArraySize { size, .. } => {
                format!("Array size cannot be negative, got '{}'", size)
            }
            VynError::ArraySizeNotStatic { .. } => {
                "Array size must be a compile-time constant expression".to_string()
            }
            VynError::InvalidUnaryOperator { operator, .. } => {
                format!("Unary operator '{}' is not valid in this context", operator)
            }
            VynError::InvalidBinaryOperator { operator, .. } => {
                format!(
                    "Binary operator '{}' is not valid in this context",
                    operator
                )
            }
            VynError::UndefinedVariable { name, .. } => {
                format!("Undefined variable '{}'", name)
            }

            VynError::IllegalLoopInterruptToken { token_type, .. } => {
                format!("Illegal '{token_type}' token found outside of loops")
            }

            VynError::IndexOutOfBounds { size, idx, .. } => {
                format!(
                    "Cannot index a value in index '{}' with a length of '{}'",
                    idx, size
                )
            }
            VynError::InvalidIndexing { target, .. } => {
                format!("Cannot index target type '{}'", target)
            }
            VynError::ImmutableMutation { identifier, .. } => {
                format!("Cannot mutate immutable identifier '{}'", identifier)
            }
            VynError::LeftHandAssignment { .. } => {
                format!("Cannot perform left-handed assignment")
            }
            VynError::VariableRedeclaration {
                name,
                original_span,
                ..
            } => {
                format!(
                    "Variable '{}' is already declared at Ln {}:{}",
                    name, original_span.line, original_span.start_column
                )
            }

            VynError::UnknownAST { node, .. } => match node {
                Node::Statement(s) => {
                    format!("Unimplemented statement:\n\n{:#?}", s)
                }
                Node::Expression(e) => {
                    format!("Unimplemented expression:\n\n{:#?}", e)
                }
            },
            VynError::UndefinedIdentifier { ident_name, .. } => {
                format!(
                    "Internal compiler error: undefined identifier '{}' escaped type checking",
                    ident_name
                )
            }

            VynError::ArithmeticError {
                operation,
                left_type,
                right_type,
                ..
            } => {
                if left_type == right_type {
                    format!(
                        "Arithmetic operation '{}' is not supported for type '{}'",
                        operation, left_type
                    )
                } else {
                    format!(
                        "Arithmetic operation '{}' cannot be performed between '{}' and '{}'",
                        operation, left_type, right_type
                    )
                }
            }

            VynError::UnaryOperationError {
                operation,
                operand_type,
                ..
            } => {
                format!(
                    "Unary operation '{}' is not supported for type '{}'",
                    operation, operand_type
                )
            }

            VynError::ComparisonOperationError {
                operation,
                blame_type,
                ..
            } => {
                format!(
                    "Comparison operator '{}' cannot be used with type '{}'",
                    operation, blame_type
                )
            }
        }
    }
}
