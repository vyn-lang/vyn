use crate::{
    ast::ast::{Expr, Expression},
    compiler::compiler::Compiler,
    type_checker::type_checker::Type,
};

impl Compiler {
    pub(crate) fn get_expr_type(&mut self, expr: &Box<Expression>) -> Option<Type> {
        match expr.node.clone() {
            Expr::IntegerLiteral(_) => Some(Type::Integer),
            Expr::FloatLiteral(_) => Some(Type::Float),
            Expr::BooleanLiteral(_) => Some(Type::Bool),
            Expr::StringLiteral(_) => Some(Type::String),
            Expr::Identifier(name) => {
                match self.symbol_table.resolve_identifier(&name, expr.span) {
                    Ok(symbol) => Some(symbol.symbol_type.clone()), // Get the actual type from symbol table
                    Err(ve) => {
                        self.throw_error(ve);
                        return None;
                    }
                }
            }
            Expr::NilLiteral => Some(Type::Nil),

            Expr::Index { target, .. } => {
                let target_type = self.get_expr_type(&target)?;

                match target_type {
                    Type::Array(element_type, _) | Type::Sequence(element_type) => {
                        Some(*element_type)
                    }
                    _ => {
                        unreachable!()
                    }
                }
            }
            _ => None,
        }
    }
}
