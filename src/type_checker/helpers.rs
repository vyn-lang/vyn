use crate::{
    ast::ast::{Expr, Expression},
    error_handler::error_collector::ErrorCollector,
    type_checker::type_checker::{Type, TypeChecker},
};

impl TypeChecker<'_> {
    pub(crate) fn is_value_static(&self, expr: &Expression, expected_type: Option<&Type>) -> bool {
        match &expr.node {
            // Literals are always static
            Expr::IntegerLiteral(_)
            | Expr::FloatLiteral(_)
            | Expr::BooleanLiteral(_)
            | Expr::StringLiteral(_)
            | Expr::NilLiteral => true,

            // Identifiers are static only if they refer to static variables
            Expr::Identifier(name) => {
                // Create a temporary error collector that we'll discard
                let mut temp_errors = ErrorCollector::new();
                if let Ok(symbol) =
                    self.symbol_type_table
                        .resolve_identifier(name, expr.span, &mut temp_errors)
                {
                    symbol.is_static()
                } else {
                    false
                }
            }

            // Array literals depend on the expected type
            Expr::ArrayLiteral { elements } => {
                if let Some(Type::Sequence(_)) = expected_type {
                    return false;
                }

                // check if all elements are static
                elements.iter().all(|elem| self.is_value_static(elem, None))
            }

            // Unary operations are static if the operand is static
            Expr::Unary { right, .. } => self.is_value_static(right, None),

            // Binary operations are static if both operands are static
            Expr::BinaryOperation { left, right, .. } => {
                self.is_value_static(left, None) && self.is_value_static(right, None)
            }

            // Index access is static if both target and property are static
            Expr::Index { target, property } => {
                self.is_value_static(target, None) && self.is_value_static(property, None)
            }

            // Any assignment operation is dynamic
            Expr::VariableAssignment { .. } | Expr::IndexAssignment { .. } => false,

            // Default to dynamic for unknown expressions
            _ => false,
        }
    }
}
