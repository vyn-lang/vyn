use crate::{
    ast::ast::{Expr, Expression},
    ir::{builder::VynIRBuilder, ir_instr::VynIROC},
    tokens::Token,
    type_checker::type_checker::Type,
};

impl VynIRBuilder<'_> {
    pub(crate) fn build_binary_expr(
        &mut self,
        left: &Box<Expression>,
        operator: &Token,
        right: &Box<Expression>,
        expr: &Expression,
    ) -> Option<u32> {
        let b_left = self.build_expr(left.as_ref())?;
        let b_right = self.build_expr(right.as_ref())?;
        let dest = self.allocate_vreg();

        let expr_type = self.get_expr_type(expr)?;
        let opcode = match operator {
            // Arithmetic
            Token::Plus | Token::Minus | Token::Asterisk | Token::Slash | Token::Caret => {
                self.build_arith_expr(expr_type, b_left, operator, b_right, dest)
            }
            Token::LessThan
            | Token::GreaterThan
            | Token::LessThanEqual
            | Token::GreaterThanEqual
            | Token::Equal
            | Token::NotEqual => self.build_comp_expr(expr_type, b_left, operator, b_right, dest),

            _ => unreachable!(),
        };

        self.emit(opcode.spanned(expr.span));

        Some(dest)
    }

    fn get_expr_type(&mut self, expr: &Expression) -> Option<Type> {
        match &expr.node {
            Expr::IntegerLiteral(_) => Some(Type::Integer),
            Expr::FloatLiteral(_) => Some(Type::Float),
            Expr::BooleanLiteral(_) => Some(Type::Bool),
            Expr::StringLiteral(_) => Some(Type::String),

            Expr::Identifier(name) => {
                let symbol =
                    self.symbol_table
                        .resolve_symbol(name, expr.span, &mut self.error_collector)?;
                Some(symbol.symbol_type.clone())
            }

            Expr::BinaryOperation { left, .. } => {
                // Binary expr type = left operand type (type checker validated they match)
                self.get_expr_type(left)
            }

            _ => None, // Shouldn't reach here for arithmetic/comparison
        }
    }

    fn build_arith_expr(
        &mut self,
        expr_type: Type,
        b_left: u32,
        operator: &Token,
        b_right: u32,
        dest: u32,
    ) -> VynIROC {
        let is_op_int = matches!(expr_type, Type::Integer);

        match operator {
            Token::Plus => {
                if is_op_int {
                    VynIROC::AddInt {
                        dest,
                        left: b_left,
                        right: b_right,
                    }
                } else {
                    VynIROC::AddFloat {
                        dest,
                        left: b_left,
                        right: b_right,
                    }
                }
            }
            Token::Minus => {
                if is_op_int {
                    VynIROC::SubInt {
                        dest,
                        left: b_left,
                        right: b_right,
                    }
                } else {
                    VynIROC::SubFloat {
                        dest,
                        left: b_left,
                        right: b_right,
                    }
                }
            }
            Token::Asterisk => {
                if is_op_int {
                    VynIROC::MulInt {
                        dest,
                        left: b_left,
                        right: b_right,
                    }
                } else {
                    VynIROC::MulFloat {
                        dest,
                        left: b_left,
                        right: b_right,
                    }
                }
            }
            Token::Slash => {
                if is_op_int {
                    VynIROC::DivInt {
                        dest,
                        left: b_left,
                        right: b_right,
                    }
                } else {
                    VynIROC::DivFloat {
                        dest,
                        left: b_left,
                        right: b_right,
                    }
                }
            }
            Token::Caret => {
                if is_op_int {
                    VynIROC::ExpInt {
                        dest,
                        left: b_left,
                        right: b_right,
                    }
                } else {
                    VynIROC::ExpFloat {
                        dest,
                        left: b_left,
                        right: b_right,
                    }
                }
            }

            _ => unreachable!(),
        }
    }

    fn build_comp_expr(
        &mut self,
        expr_type: Type,
        b_left: u32,
        operator: &Token,
        b_right: u32,
        dest: u32,
    ) -> VynIROC {
        let is_op_int = matches!(expr_type, Type::Integer);

        match operator {
            Token::LessThan => {
                if is_op_int {
                    VynIROC::CompareLessInt {
                        dest,
                        left: b_left,
                        right: b_right,
                    }
                } else {
                    VynIROC::CompareLessFloat {
                        dest,
                        left: b_left,
                        right: b_right,
                    }
                }
            }
            Token::GreaterThan => {
                if is_op_int {
                    VynIROC::CompareGreaterInt {
                        dest,
                        left: b_left,
                        right: b_right,
                    }
                } else {
                    VynIROC::CompareGreaterFloat {
                        dest,
                        left: b_left,
                        right: b_right,
                    }
                }
            }
            Token::GreaterThanEqual => {
                if is_op_int {
                    VynIROC::CompareGreaterEqualInt {
                        dest,
                        left: b_left,
                        right: b_right,
                    }
                } else {
                    VynIROC::CompareGreaterEqualFloat {
                        dest,
                        left: b_left,
                        right: b_right,
                    }
                }
            }
            Token::LessThanEqual => {
                if is_op_int {
                    VynIROC::CompareLessEqualInt {
                        dest,
                        left: b_left,
                        right: b_right,
                    }
                } else {
                    VynIROC::CompareLessEqualFloat {
                        dest,
                        left: b_left,
                        right: b_right,
                    }
                }
            }
            Token::Equal => VynIROC::CompareEqual {
                dest,
                left: b_left,
                right: b_right,
            },
            Token::NotEqual => VynIROC::CompareNotEqual {
                dest,
                left: b_left,
                right: b_right,
            },

            _ => unreachable!(),
        }
    }
}
