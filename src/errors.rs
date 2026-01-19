use crate::{
    ast::ast::Node, runtime_value::RuntimeType, tokens::TokenType,
    type_checker::type_checker::Type, utils::Span,
};
use colored::*;

#[derive(Debug, Clone)]
pub enum HydorError {
    // ----- Parser -----
    UnexpectedToken {
        token: TokenType,
        span: Span,
    },
    ExpectedToken {
        expected: TokenType,
        got: TokenType,
        span: Span,
    },
    KeywordTypeError {
        got: TokenType,
        span: Span,
    },
    InvalidTypeName {
        got: String,
        span: Span,
    },
    ExpectedType {
        got: TokenType,
        span: Span,
    },

    // ----- Type Checker -----
    TypeMismatch {
        expected: Vec<Type>, // Can be one of the expected
        found: Type,
        span: Span,
    },
    DeclarationTypeMismatch {
        got: Type,
        expected: Type,
        span: Span,
    },
    InvalidUnaryOp {
        operator: TokenType,
        operand_type: Type,
        span: Span,
    },
    InvalidBinaryOp {
        operator: TokenType,
        left_type: Type,
        right_type: Type,
        span: Span,
    },
    UndefinedVariable {
        name: String,
        span: Span,
    },
    VariableRedeclaration {
        name: String,
        original_span: Span,
        redeclaration_span: Span,
    },

    // ----- Compiler -----
    RegisterOverflow {
        span: Span,
    },
    NotImplemented {
        feature: String,
        span: Span,
    },
    UnknownAST {
        node: Node,
        span: Span,
    },
    UndefinedIdentifier {
        ident_name: String,
        span: Span,
    },

    // ----- HydorVM -----
    OperandStackUnderflow {
        stack_length: usize,
        span: Span,
    },
    OperandStackOverflow {
        stack_length: usize,
        span: Span,
    },
    GlobalStackOverflow {
        stack_length: usize,
        max: usize,
        span: Span,
    },

    // ----- Runtime Arithmetic Errors -----
    ArithmeticError {
        operation: TokenType,
        left_type: RuntimeType,
        right_type: RuntimeType,
        span: Span,
    },
    UnaryOperationError {
        operation: TokenType,
        operand_type: RuntimeType,
        span: Span,
    },
    ComparisonOperationError {
        operation: TokenType,
        blame_type: RuntimeType,
        span: Span,
    },
}

impl HydorError {
    pub fn span(&self) -> Span {
        match self {
            HydorError::UnexpectedToken { span, .. } => *span,
            HydorError::ExpectedToken { span, .. } => *span,
            HydorError::KeywordTypeError { span, .. } => *span,
            HydorError::InvalidTypeName { span, .. } => *span,
            HydorError::ExpectedType { span, .. } => *span,
            HydorError::RegisterOverflow { span, .. } => *span,
            HydorError::NotImplemented { span, .. } => *span,

            HydorError::TypeMismatch { span, .. } => *span,
            HydorError::InvalidUnaryOp { span, .. } => *span,
            HydorError::InvalidBinaryOp { span, .. } => *span,
            HydorError::DeclarationTypeMismatch { span, .. } => *span,
            HydorError::UndefinedVariable { span, .. } => *span,
            HydorError::VariableRedeclaration {
                redeclaration_span, ..
            } => *redeclaration_span,

            HydorError::UnknownAST { span, .. } => *span,
            HydorError::UndefinedIdentifier { span, .. } => *span,

            HydorError::OperandStackUnderflow { span, .. } => *span,
            HydorError::OperandStackOverflow { span, .. } => *span,
            HydorError::GlobalStackOverflow { span, .. } => *span,
            HydorError::ArithmeticError { span, .. } => *span,
            HydorError::UnaryOperationError { span, .. } => *span,
            HydorError::ComparisonOperationError { span, .. } => *span,
        }
    }

    pub fn category(&self) -> &str {
        match self {
            HydorError::UnexpectedToken { .. } => "Syntax",
            HydorError::ExpectedToken { .. } => "Syntax",
            HydorError::KeywordTypeError { .. } => "Syntax",
            HydorError::InvalidTypeName { .. } => "Syntax",
            HydorError::ExpectedType { .. } => "Syntax",
            HydorError::RegisterOverflow { .. } => "Compiler",
            HydorError::NotImplemented { .. } => "Compiler",

            HydorError::TypeMismatch { .. } => "Type",
            HydorError::InvalidUnaryOp { .. } => "Type",
            HydorError::InvalidBinaryOp { .. } => "Type",
            HydorError::DeclarationTypeMismatch { .. } => "Type",
            HydorError::UndefinedVariable { .. } => "Type",
            HydorError::VariableRedeclaration { .. } => "Type",

            HydorError::UnknownAST { .. } => "Compiler",
            HydorError::UndefinedIdentifier { .. } => "Compiler",

            HydorError::OperandStackUnderflow { .. } => "Runtime",
            HydorError::OperandStackOverflow { .. } => "Runtime",
            HydorError::GlobalStackOverflow { .. } => "Runtime",
            HydorError::ArithmeticError { .. } => "Runtime",
            HydorError::UnaryOperationError { .. } => "Runtime",
            HydorError::ComparisonOperationError { .. } => "Runtime",
        }
    }

    pub fn message(&self) -> String {
        match self {
            HydorError::UnexpectedToken { token, .. } => {
                format!("Unexpected token '{}'", token)
            }
            HydorError::ExpectedToken { expected, got, .. } => {
                format!("Expected '{}' but found '{}'", expected, got)
            }
            HydorError::KeywordTypeError { got, .. } => {
                format!("'{}' is a keyword and cannot be used as a type name", got)
            }
            HydorError::RegisterOverflow { .. } => {
                "Register overflow: expression is too complex".to_string()
            }
            HydorError::NotImplemented { feature, .. } => {
                format!("Feature not yet implemented: {}", feature)
            }
            HydorError::InvalidTypeName { got, .. } => {
                format!("'{}' is not a valid type", got)
            }
            HydorError::ExpectedType { got, .. } => {
                format!("Expected type annotation, got '{got}' instead")
            }
            HydorError::DeclarationTypeMismatch { got, expected, .. } => {
                format!(
                    "Type mismatch in variable declaration: expected '{}', got '{}'",
                    expected, got
                )
            }
            HydorError::TypeMismatch {
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
            HydorError::InvalidUnaryOp {
                operator,
                operand_type,
                ..
            } => {
                format!(
                    "Unary operator '{}' cannot be applied to type '{}'",
                    operator, operand_type
                )
            }
            HydorError::InvalidBinaryOp {
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
            HydorError::UndefinedVariable { name, .. } => {
                format!("Undefined variable '{}'", name)
            }
            HydorError::VariableRedeclaration {
                name,
                original_span,
                ..
            } => {
                format!(
                    "Variable '{}' is already declared at Ln {}:{}",
                    name, original_span.line, original_span.start_column
                )
            }

            HydorError::UnknownAST { node, .. } => match node {
                Node::Statement(s) => {
                    format!("Unimplemented statement:\n\n{:#?}", s)
                }
                Node::Expression(e) => {
                    format!("Unimplemented expression:\n\n{:#?}", e)
                }
            },
            HydorError::UndefinedIdentifier { ident_name, .. } => {
                format!(
                    "Internal compiler error: undefined identifier '{}' escaped type checking",
                    ident_name
                )
            }

            HydorError::OperandStackUnderflow { stack_length, .. } => {
                format!(
                    "Operand stack underflow: attempted to pop from stack with {} elements",
                    stack_length
                )
            }

            HydorError::OperandStackOverflow { stack_length, .. } => {
                format!(
                    "Operand stack overflow: expression stack exceeded maximum size (current size: {})",
                    stack_length
                )
            }

            HydorError::GlobalStackOverflow {
                stack_length, max, ..
            } => {
                format!(
                    "Global stack overflow: too many global variables ({} / max {})",
                    stack_length, max
                )
            }

            HydorError::ArithmeticError {
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

            HydorError::UnaryOperationError {
                operation,
                operand_type,
                ..
            } => {
                format!(
                    "Unary operation '{}' is not supported for type '{}'",
                    operation, operand_type
                )
            }

            HydorError::ComparisonOperationError {
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

    pub fn hint(&self) -> Option<String> {
        match self {
            HydorError::UnexpectedToken { .. } => {
                Some("Remove this token or check for missing syntax".to_string())
            }
            HydorError::ExpectedToken { expected, .. } => {
                Some(format!("Insert '{}' at this location", expected))
            }
            HydorError::KeywordTypeError { .. } => {
                Some("Keywords are reserved and cannot be used as type names".to_string())
            }
            HydorError::ExpectedType { got, .. } => Some(format!(
                "Insert a valid type before '{got}' based on the assigned value"
            )),
            HydorError::RegisterOverflow { .. } => {
                Some("Split this expression into multiple smaller expressions or statements".to_string())
            }
            HydorError::NotImplemented { feature, .. } => {
                Some(format!("'{}' is planned but not yet available in this version", feature))
            }
            HydorError::InvalidTypeName { .. } => {
                Some("Available types: Int, Float, Bool, String".to_string())
            }
            HydorError::DeclarationTypeMismatch { got, expected, .. } => Some(format!(
                "Either change the declared type to '{}' or provide a value of type '{}'",
                got, expected
            )),
            HydorError::TypeMismatch { expected, .. } => {
                if expected.len() > 1 {
                    Some("Ensure the value matches one of the expected types".to_string())
                } else {
                    Some(format!("Convert the value to type '{}'", expected[0]))
                }
            }
            HydorError::InvalidUnaryOp { operator, .. } => match operator {
                TokenType::Not => Some("Logical negation requires a boolean operand".to_string()),
                TokenType::Minus => {
                    Some("Numeric negation requires an integer or float operand".to_string())
                }
                _ => Some("This operator is not supported for the given type".to_string()),
            },
            HydorError::InvalidBinaryOp {
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
            HydorError::UndefinedVariable { name, .. } => {
                Some(format!("Declare the variable '{}' before using it", name))
            }
            HydorError::VariableRedeclaration { name, .. } => Some(format!(
                "Remove this declaration or rename the variable to a different name than '{}'",
                name
            )),

            HydorError::UnknownAST { .. } => {
                Some("This is a compiler bug. Please report this issue".to_string())
            }
            HydorError::UndefinedIdentifier { .. } => Some(
                "This is a compiler bug. The type checker should have caught this error"
                    .to_string(),
            ),

            HydorError::OperandStackUnderflow { .. } => {
                Some("This is a virtual machine bug. Please report this issue".to_string())
            }

            HydorError::OperandStackOverflow { .. } => {
                Some("Reduce expression complexity or split expressions into smaller statements".to_string())
            }

            HydorError::GlobalStackOverflow { .. } => {
                Some(
                    "Reduce the number of global variables, or move values into local scopes or functions"
                    .to_string(),
                )
            }

            HydorError::ArithmeticError {
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

            HydorError::UnaryOperationError { operation, .. } => match operation {
                TokenType::Minus => {
                    Some("Negation requires an integer or float operand".to_string())
                }
                TokenType::Not => Some("Logical negation requires a boolean operand".to_string()),
                _ => Some("This operation is not supported for the given type".to_string()),
            },

            HydorError::ComparisonOperationError { .. } => {
                Some("Comparison operators require integer or float operands".to_string())
            }
        }
    }

    pub fn report(&self, source: &str) {
        let span = self.span();

        // Header: Category::Error -> message
        eprintln!(
            "{}{}{}{}",
            self.category().bright_white().bold(),
            "::".white().dimmed(),
            "Error".red().dimmed().bold(),
            format!(" -> {}", self.message()).bright_red()
        );

        eprintln!();

        // Error caused by section
        eprintln!("{}", "Error caused by:".white().dimmed().bold());

        // Main error location
        self.print_code_snippet(source, span, true);

        // Additional context based on error type
        self.print_additional_context(source);

        eprintln!();

        // Hint section
        if let Some(hint_text) = self.hint() {
            eprintln!("{} {}", "Hint:".bright_yellow(), hint_text.bright_white());
        }

        eprintln!();
    }

    fn print_code_snippet(&self, source: &str, span: Span, highlight: bool) {
        let lines: Vec<&str> = source.lines().collect();

        if span.line == 0 || span.line > (lines.len() as u32) {
            eprintln!(
                "    {} {} {}",
                format!("Ln {}:{}", span.line, span.start_column).cyan(),
                "|".white(),
                "<source unavailable>".dimmed()
            );
            return;
        }

        let line_content = lines[(span.line - 1) as usize];
        let line_label = format!("Ln {}:{}", span.line, span.start_column);

        // Print the line
        if highlight {
            eprintln!(
                "    {} {} {}",
                line_label.cyan().bold(),
                "|".white(),
                line_content.bold().bright_white(),
            );
        } else {
            eprintln!(
                "    {} {} {}",
                line_label.cyan().bold(),
                "|".white(),
                line_content.dimmed(),
            );
        }

        // Print the pointer
        let line_prefix_len = line_label.len();
        let gutter_padding = " ".repeat(line_prefix_len + 3); // +3 for " | "
        let code_padding = " ".repeat(span.start_column.saturating_sub(1) as usize);
        let width = span.end_column.saturating_sub(span.start_column).max(1) as usize;
        let pointer = if width == 1 {
            "^".to_string()
        } else {
            "~".repeat(width)
        };

        if highlight {
            eprintln!(
                "    {}{}{}",
                gutter_padding,
                code_padding,
                pointer.bright_red().bold()
            );
        } else {
            eprintln!(
                "    {}{}{}",
                gutter_padding,
                code_padding,
                pointer.cyan().dimmed()
            );
        }
    }

    fn print_additional_context(&self, source: &str) {
        match self {
            HydorError::VariableRedeclaration { original_span, .. } => {
                eprintln!();
                eprintln!("{}", "Originally declared here:".white().dimmed());
                self.print_code_snippet(source, *original_span, false);
            }
            _ => {}
        }
    }
}

#[derive(Debug, Default)]
pub struct ErrorCollector {
    errors: Vec<HydorError>,
}

impl ErrorCollector {
    pub fn new() -> Self {
        Self { errors: Vec::new() }
    }

    pub fn add(&mut self, error: HydorError) {
        self.errors.push(error);
    }

    pub fn has_errors(&self) -> bool {
        !self.errors.is_empty()
    }

    pub fn len(&self) -> usize {
        self.errors.len()
    }

    pub fn report_all(&self, source: &str) {
        for error in &self.errors {
            error.report(source);
        }

        if !self.errors.is_empty() {
            let error_word = if self.errors.len() == 1 {
                "error"
            } else {
                "errors"
            };

            eprintln!(
                "{} Could not compile due to {} {}",
                "*".bright_red().bold(),
                self.errors.len().to_string().bright_red().bold(),
                error_word.bright_red()
            );
        }
    }

    pub fn errors(&self) -> &[HydorError] {
        &self.errors
    }

    pub fn clear(&mut self) {
        self.errors.clear()
    }
}
