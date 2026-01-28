use crate::{
    ast::ast::{Expr, Node},
    runtime_value::values::RuntimeType,
    tokens::{Token, TokenType},
    type_checker::type_checker::Type,
    utils::Span,
};

#[derive(Debug, Clone)]
pub enum VynError {
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

    // ----- Static Evaluator -----
    CircularStaticDependency {
        name: String,
        span: Span,
    },

    UndefinedStatic {
        name: String,
        span: Span,
    },

    StaticEvaluationFailed {
        name: String,
        span: Span,
    },

    NotStaticExpression {
        span: Span,
    },

    InvalidStaticOperation {
        operation: String,
        span: Span,
    },

    StaticOverflow {
        span: Span,
    },

    NegativeExponent {
        span: Span,
    },

    NegativeArraySize {
        size: i32,
        span: Span,
    },

    ArraySizeNotStatic {
        span: Span,
    },

    InvalidUnaryOperator {
        operator: Token,
        span: Span,
    },

    InvalidBinaryOperator {
        operator: Token,
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
    InvalidIndexing {
        target: Type,
        span: Span,
    },
    TypeInfer {
        expr: Expr,
        span: Span,
    },
    ArrayLengthMismatch {
        expected: usize,
        got: usize,
        span: Span,
    },
    IndexOutOfBounds {
        size: usize,
        idx: i64,
        span: Span,
    },
    StaticRequiresConstant {
        span: Span,
    },
    StaticMutation {
        identifier: String,
        mutator_span: Span,
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
    LeftHandAssignment {
        span: Span,
    },
    ImmutableMutation {
        identifier: String,
        span: Span,
        mutation_span: Span,
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
    TypeAliasRedeclaration {
        name: String,
        span: Span,
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
    DivisionByZero {
        // This can also be compile time
        span: Span,
    },
}
