use crate::{
    ast::type_annotation::TypeAnnotation,
    tokens::Token,
    utils::{Span, Spanned},
};
use std::fmt::{Display, Formatter, Result as FmtResult};

#[derive(Debug, Clone, PartialEq)]
pub struct Program {
    pub statements: Vec<Statement>,
}

pub type Expression = Spanned<Expr>;
pub type Statement = Spanned<Stmt>;

// Used fot error handling
#[derive(Debug, Clone, PartialEq)]
pub enum Node {
    Statement(Stmt),
    Expression(Expr),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    IntegerLiteral(i32),
    FloatLiteral(f64),
    BooleanLiteral(bool),
    StringLiteral(String),
    Identifier(String),
    NilLiteral,
    ArrayLiteral {
        elements: Vec<Box<Expression>>,
    },

    Unary {
        operator: Token,
        right: Box<Expression>,
    },

    BinaryOperation {
        left: Box<Expression>,
        operator: Token,
        right: Box<Expression>,
    },
    VariableAssignment {
        identifier: Box<Expression>,
        new_value: Box<Expression>,
    },
    Index {
        target: Box<Expression>,
        property: Box<Expression>,
    },
    IndexAssignment {
        target: Box<Expression>,
        property: Box<Expression>,
        new_value: Box<Expression>,
    },
}

impl Display for Expression {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}", self.node)
    }
}

impl Display for Expr {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            Expr::IntegerLiteral(n) => write!(f, "{}", n),
            Expr::FloatLiteral(fl) => write!(f, "{}", fl),
            Expr::BooleanLiteral(b) => write!(f, "{}", b),
            Expr::StringLiteral(s) => write!(f, "\"{}\"", s),
            Expr::Identifier(name) => write!(f, "{}", name),
            Expr::NilLiteral => write!(f, "nil"),
            Expr::ArrayLiteral { elements } => {
                let v = elements
                    .iter()
                    .map(|e| format!("{}", e))
                    .collect::<Vec<_>>()
                    .join(", ");

                write!(f, "[{}]", v)
            }

            Expr::Unary { operator, right } => {
                write!(f, "({}{})", operator, right)
            }

            Expr::BinaryOperation {
                left,
                operator,
                right,
            } => {
                write!(f, "({} {} {})", left, operator, right)
            }

            Expr::VariableAssignment {
                identifier,
                new_value,
            } => {
                write!(f, "{} = {}", identifier, new_value)
            }

            Expr::Index { target, property } => {
                write!(f, "{}::{}", target, property)
            }
            Expr::IndexAssignment {
                target,
                property,
                new_value,
            } => {
                write!(f, "{}::{} = {}", target, property, new_value)
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Stmt {
    Expression {
        expression: Expression,
    },
    VariableDeclaration {
        identifier: Expression,
        value: Expression,
        annotated_type: TypeAnnotation,
        mutable: bool,
    },
    StaticVariableDeclaration {
        identifier: Expression,
        value: Expression,
        annotated_type: TypeAnnotation,
    },
    TypeAliasDeclaration {
        identifier: Expression,
        value: TypeAnnotation,
    },
    StdoutLog {
        log_value: Expression,
    },
    Scope {
        // This only creates a seperate symbol table at comp time
        // whilst block creates a its own instruction area
        statements: Vec<Statement>,
    },
    Block {
        statements: Vec<Statement>,
    },
    IfDeclaration {
        condition: Expression,
        consequence: Box<Statement>,
        alternate: Box<Option<Statement>>,
    },
    Loop {
        body: Box<Statement>,
    },
    Continue,
    Break,
}

impl Expr {
    pub fn spanned(self, span: Span) -> Spanned<Self> {
        Spanned { node: self, span }
    }

    pub fn to_node(self) -> Node {
        Node::Expression(self.clone())
    }
}

impl Stmt {
    pub fn spanned(self, span: Span) -> Spanned<Self> {
        Spanned { node: self, span }
    }

    pub fn to_node(self) -> Node {
        Node::Statement(self.clone())
    }
}
