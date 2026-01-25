use crate::{
    ast::ast::{Expr, Expression},
    compiler::compiler::Compiler,
    type_checker::type_checker::Type,
};

impl Compiler {
    pub(crate) fn get_expr_type(&mut self, expr: &Box<Expression>) -> Option<Type> {
        match expr.node {
            Expr::IntegerLiteral(_) => Some(Type::Integer),
            Expr::FloatLiteral(_) => Some(Type::Float),
            Expr::BooleanLiteral(_) => Some(Type::Bool),
            Expr::StringLiteral(_) => Some(Type::String),
            Expr::Identifier(_) => Some(Type::Identifier),
            Expr::NilLiteral => Some(Type::Nil),

            _ => None,
        }
    }

    // NOTE: This will be used when preallocate syntax is created [v; n] ---
    // pub(crate) fn is_default_value(&self, expr: &Expression, ty: &Type) -> bool {
    //     match (&expr.node, ty) {
    //         (Expr::IntegerLiteral(0), Type::Integer) => true,
    //         (Expr::FloatLiteral(f), Type::Float) if *f == 0.0 => true,
    //         (Expr::BooleanLiteral(false), Type::Bool) => true,
    //         (Expr::NilLiteral, Type::Nil) => true,
    //         (Expr::StringLiteral(s), Type::String) if s.is_empty() => true,
    //         _ => false,
    //     }
    // }
}
