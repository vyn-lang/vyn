use crate::{
    ast::ast::{Expr, Expression},
    compiler::compiler::Compiler,
    type_checker::type_checker::Type,
};

impl Compiler<'_> {
    pub(crate) fn get_expr_type(&mut self, expr: &Box<Expression>) -> Option<Type> {
        match &expr.node {
            Expr::IntegerLiteral(_) => Some(Type::Integer),
            Expr::FloatLiteral(_) => Some(Type::Float),
            Expr::BooleanLiteral(_) => Some(Type::Bool),
            Expr::StringLiteral(_) => Some(Type::String),
            Expr::Identifier(name) => match self.symbol_table.resolve_identifier(name, expr.span) {
                Ok(symbol) => Some(symbol.symbol_type.clone()),
                Err(ve) => {
                    self.throw_error(ve);
                    return None;
                }
            },
            Expr::NilLiteral => Some(Type::Nil),

            Expr::Index { target, .. } => {
                let target_type = self.get_expr_type(target)?;

                match target_type {
                    Type::Array(element_type, _) | Type::Sequence(element_type) => {
                        Some(*element_type)
                    }
                    _ => {
                        unreachable!()
                    }
                }
            }

            Expr::ArrayLiteral { elements } => {
                // Type checker ensures this never happens without expected_type in compile_expression
                // If we're here, something went wrong in type checking
                if elements.is_empty() {
                    unreachable!("Empty array literal should have been caught by type checker");
                }

                // Infer element type from first element
                let elem_type = self.get_expr_type(&elements[0])?;

                // We can't know if it's Array or Sequence without expected_type,
                // but this is only used for intermediate expressions, not the literal itself
                // Default to Sequence as it's more general
                // The compiler should never be here anyways
                Some(Type::Sequence(Box::new(elem_type)))
            }

            Expr::BinaryOperation { left, .. } => self.get_expr_type(left),

            Expr::Unary { right, .. } => self.get_expr_type(right),

            _ => None,
        }
    }
}
