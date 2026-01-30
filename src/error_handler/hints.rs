use crate::{error_handler::errors::VynError, tokens::TokenType};

impl VynError {
    pub fn hint(&self) -> Option<String> {
        match self {
            VynError::UnexpectedToken { .. } => {
                Some("Remove this token or check for missing syntax".to_string())
            }
            VynError::ExpectedToken { expected, .. } => {
                Some(format!("Insert '{}' at this location", expected))
            }
            VynError::KeywordTypeError { .. } => {
                Some("Keywords are reserved and cannot be used as type names".to_string())
            }
            VynError::ExpectedType { got, .. } => Some(format!(
                "Insert a valid type before '{got}' based on the assigned value"
            )),
            VynError::RegisterOverflow { .. } => Some(
                "Split this expression into multiple smaller expressions or statements".to_string(),
            ),
            VynError::NotImplemented { feature, .. } => Some(format!(
                "'{}' is planned but not yet available in this version",
                feature
            )),
            VynError::InvalidTypeName { .. } => {
                Some("Available types: Int, Float, Bool, String".to_string())
            }
            VynError::TypeInfer { expr, .. } => {
                Some(format!("Annotate a type for expression '{expr}'"))
            }
            VynError::StaticRequiresConstant { .. } => Some(format!(
                "Consider changing the variable signiture to be a 'let' variable or change the value to a static value",
            )),
            VynError::CircularStaticDependency { name, .. } => {
            Some(format!(
                "Static variable '{}' depends on itself directly or indirectly. Break the circular reference",
                name
            ))
        }
        VynError::UndefinedStatic { name, .. } => {
            Some(format!(
                "Declare static variable '{}' before using it in constant expressions",
                name
            ))
        }
        VynError::StaticEvaluationFailed { name, .. } => {
            Some(format!(
                "Ensure static variable '{}' has a valid compile-time constant value",
                name
            ))
        }
        VynError::NotStaticExpression { .. } => {
            Some("Use only literals, static variables, and compile-time operations".to_string())
        }
        VynError::InvalidStaticOperation { .. } => {
            Some("Only basic arithmetic operations (+, -, *, /, ^) are allowed in static expressions".to_string())
        }
        VynError::StaticOverflow { .. } => {
            Some("Use smaller values or change the operation to prevent overflow".to_string())
        }
        VynError::NegativeExponent { .. } => {
            Some("Exponents in compile-time expressions must be non-negative integers".to_string())
        }
        VynError::NegativeArraySize { .. } => {
            Some("Array size must be a positive integer".to_string())
        }
        VynError::ArraySizeNotStatic { .. } => {
            Some("Use a literal number or static variable for array size".to_string())
        }
        VynError::InvalidUnaryOperator { .. } => {
            Some("Only '+', '-', and '!' operators are allowed in constant expressions".to_string())
        }
        VynError::InvalidBinaryOperator { .. } => {
            Some("Only arithmetic operators (+, -, *, /, ^) are allowed in constant expressions".to_string())
        }
            VynError::StaticMutation { .. } => Some(format!(
                "Consider changing the variable signiture to be a 'let' variable or remove the assignment expression",
            )),
            VynError::ArrayLengthMismatch { expected, got, .. } => Some(format!(
                "Consider changing the array length to '{expected}' or adjust the annotated length to '{got}'"
            )),
            VynError::DeclarationTypeMismatch { got, expected, .. } => Some(format!(
                "Either change the declared type to '{}' or provide a value of type '{}'",
                got, expected
            )),
            VynError::TypeMismatch { expected, .. } => {
                if expected.len() > 1 {
                    Some("Ensure the value matches one of the expected types".to_string())
                } else {
                    Some(format!("Convert the value to type '{}'", expected[0]))
                }
            }
            VynError::TypeAliasRedeclaration { .. } => {
                Some("Remove the redeclaration and use it".to_string())
            }
            VynError::LeftHandAssignment { .. } => None,
            VynError::InvalidIndexing { .. } => None,
            VynError::IndexOutOfBounds { .. } => None,
            VynError::ImmutableMutation { identifier, .. } => {
                Some(format!("Prefix identifier '{identifier}' with '@'"))
            }
            VynError::InvalidUnaryOp { operator, .. } => match operator {
                TokenType::Not => Some("Logical negation requires a boolean operand".to_string()),
                TokenType::Minus => {
                    Some("Numeric negation requires an integer or float operand".to_string())
                }
                _ => Some("This operator is not supported for the given type".to_string()),
            },
            VynError::InvalidBinaryOp {
                operator,
                left_type,
                right_type,
                ..
            } => {
                if left_type != right_type {
                    Some("Both operands must have the same type".to_string())
                } else {
                    match operator {
                        TokenType::Plus
                        | TokenType::Minus
                        | TokenType::Asterisk
                        | TokenType::Slash
                        | TokenType::Caret => Some(
                            "Arithmetic operators require integer or float operands".to_string(),
                        ),
                        TokenType::LessThan
                        | TokenType::LessThanEqual
                        | TokenType::GreaterThan
                        | TokenType::GreaterThanEqual => Some(
                            "Comparison operators require integer or float operands".to_string(),
                        ),
                        _ => Some("This operator is not supported for the given types".to_string()),
                    }
                }
            }
            VynError::UndefinedVariable { name, .. } => {
                Some(format!("Declare the variable '{}' before using it", name))
            }
            VynError::VariableRedeclaration { name, .. } => Some(format!(
                "Remove this declaration or rename the variable to a different name than '{}'",
                name
            )),

            VynError::UnknownAST { .. } => {
                Some("This is a compiler bug. Please report this issue".to_string())
            }
            VynError::UndefinedIdentifier { .. } => Some(
                "This is a compiler bug. The type checker should have caught this error"
                    .to_string(),
            ),

            VynError::ArithmeticError {
                left_type,
                right_type,
                ..
            } => {
                if left_type == right_type {
                    Some(format!(
                        "Type '{}' does not support arithmetic operations",
                        left_type
                    ))
                } else {
                    Some(
                        "Arithmetic operations require both operands to be the same numeric type"
                            .to_string(),
                    )
                }
            }

            VynError::IllegalLoopInterruptToken { token_type, .. } => {
                Some(format!("Remove the '{token_type}' statement"))
            }

            VynError::UnaryOperationError { operation, .. } => match operation {
                TokenType::Minus => {
                    Some("Negation requires an integer or float operand".to_string())
                }
                TokenType::Not => Some("Logical negation requires a boolean operand".to_string()),
                _ => Some("This operation is not supported for the given type".to_string()),
            },

            VynError::ComparisonOperationError { .. } => {
                Some("Comparison operators require integer or float operands".to_string())
            }

            VynError::DivisionByZero { .. } => None,
        }
    }
}
