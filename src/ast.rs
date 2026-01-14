use crate::{
    tokens::Token,
    utils::{Span, Spanned},
};

#[derive(Debug, Clone, PartialEq)]
pub struct Program {
    pub statements: Vec<Statement>,
}

pub type Expression = Spanned<Expr>;
pub type Statement = Spanned<Stmt>;

#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    IntegerLiteral(i32),
    FloatLiteral(f64),
    BooleanLiteral(bool),
    Identifier(String),
    StringLiteral(String),

    Unary {
        operator: Token,
        right: Box<Expression>,
    },

    BinaryOperation {
        left: Box<Expression>,
        operator: Token,
        right: Box<Expression>,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub enum Stmt {
    Expression {
        expression: Expression,
    },
    VariableDeclaration {
        identifier: Expression,
        value: Expression,
    },
}

impl Expr {
    pub fn spanned(self, from: Span) -> Spanned<Self> {
        Spanned {
            node: self,
            span: from,
        }
    }
}

impl Stmt {
    pub fn spanned(self, from: Span) -> Spanned<Self> {
        Spanned {
            node: self,
            span: from,
        }
    }
}
