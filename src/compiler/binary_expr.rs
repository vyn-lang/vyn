use crate::{
    ast::ast::{Expr, Expression},
    bytecode::bytecode::OpCode,
    compiler::compiler::Compiler,
    runtime_value::RuntimeValue,
    tokens::{Token, TokenType},
    type_checker::type_checker::Type,
    utils::Span,
};

impl Compiler {
    fn get_binary_opcode(
        left_type: &Type,
        right_type: &Type,
        operator: &TokenType,
    ) -> Option<OpCode> {
        match operator {
            TokenType::Plus => match (left_type, right_type) {
                (Type::Integer, Type::Integer) => Some(OpCode::AddInt),
                (Type::Float, Type::Float) => Some(OpCode::AddFloat),
                (Type::String, Type::String) => Some(OpCode::ConcatString),
                _ => unreachable!("Type mismatch should be caught in type checker"),
            },

            TokenType::Minus => match (left_type, right_type) {
                (Type::Integer, Type::Integer) => Some(OpCode::SubtractInt),
                (Type::Float, Type::Float) => Some(OpCode::SubtractFloat),
                _ => unreachable!("Type mismatch should be caught in type checker"),
            },

            TokenType::Asterisk => match (left_type, right_type) {
                (Type::Integer, Type::Integer) => Some(OpCode::MultiplyInt),
                (Type::Float, Type::Float) => Some(OpCode::MultiplyFloat),
                _ => unreachable!("Type mismatch should be caught in type checker"),
            },

            TokenType::Slash => match (left_type, right_type) {
                (Type::Integer, Type::Integer) => Some(OpCode::DivideInt),
                (Type::Float, Type::Float) => Some(OpCode::DivideFloat),
                _ => unreachable!("Type mismatch should be caught in type checker"),
            },

            TokenType::Caret => match (left_type, right_type) {
                (Type::Integer, Type::Integer) => Some(OpCode::ExponentInt),
                (Type::Float, Type::Float) => Some(OpCode::ExponentFloat),
                _ => unreachable!("Type mismatch should be caught in type checker"),
            },

            TokenType::LessThan => match (left_type, right_type) {
                (Type::Integer, Type::Integer) => Some(OpCode::CompareLessInt),
                (Type::Float, Type::Float) => Some(OpCode::CompareLessFloat),
                _ => unreachable!("Type mismatch should be caught in type checker"),
            },

            TokenType::LessThanEqual => match (left_type, right_type) {
                (Type::Integer, Type::Integer) => Some(OpCode::CompareLessEqualInt),
                (Type::Float, Type::Float) => Some(OpCode::CompareLessEqualFloat),
                _ => unreachable!("Type mismatch should be caught in type checker"),
            },

            TokenType::GreaterThan => match (left_type, right_type) {
                (Type::Integer, Type::Integer) => Some(OpCode::CompareGreaterInt),
                (Type::Float, Type::Float) => Some(OpCode::CompareGreaterFloat),
                _ => unreachable!("Type mismatch should be caught in type checker"),
            },

            TokenType::GreaterThanEqual => match (left_type, right_type) {
                (Type::Integer, Type::Integer) => Some(OpCode::CompareGreaterEqualInt),
                (Type::Float, Type::Float) => Some(OpCode::CompareGreaterEqualFloat),
                _ => unreachable!("Type mismatch should be caught in type checker"),
            },

            TokenType::Equal => Some(OpCode::CompareEqual),
            TokenType::NotEqual => Some(OpCode::CompareNotEqual),

            _ => unreachable!("Unhandled binary operator type"),
        }
    }

    pub(crate) fn compile_binary_expr(
        &mut self,
        left_type: Type,
        left: Expression,
        right_type: Type,
        right: Expression,
        operator: Token,
        span: Span,
    ) -> Option<()> {
        if self
            .try_fold_binary(&left, &right, &operator, span)
            .is_some()
        {
            return Some(());
        }

        self.compile_expression(left)?;
        self.compile_expression(right)?;

        let opcode = Self::get_binary_opcode(&left_type, &right_type, &operator.get_token_type())?;
        self.emit(opcode, vec![], span);

        Some(())
    }

    pub(crate) fn try_fold_binary(
        &mut self,
        left: &Expression,
        right: &Expression,
        operator: &Token,
        span: Span,
    ) -> Option<()> {
        let left_val = self.eval_to_constant(left)?;
        let right_val = self.eval_to_constant(right)?;
        let constant = self.compute_binary_constant(left_val, right_val, operator)?;

        self.emit_constant(constant, span);
        Some(())
    }

    pub(crate) fn eval_to_constant(&mut self, expr: &Expression) -> Option<RuntimeValue> {
        match &expr.node {
            Expr::IntegerLiteral(v) => Some(RuntimeValue::IntegerLiteral(*v)),
            Expr::FloatLiteral(v) => Some(RuntimeValue::FloatLiteral(*v)),
            Expr::BooleanLiteral(v) => Some(RuntimeValue::BooleanLiteral(*v)),

            Expr::StringLiteral(v) => {
                let idx = self.intern_string(v.clone());
                Some(RuntimeValue::StringLiteral(idx))
            }

            Expr::Unary { operator, right } => {
                let right_val = self.eval_to_constant(right)?;
                self.compute_unary_constant(right_val, operator)
            }

            Expr::BinaryOperation {
                left,
                operator,
                right,
            } => {
                let left_val = self.eval_to_constant(left)?;
                let right_val = self.eval_to_constant(right)?;
                self.compute_binary_constant(left_val, right_val, operator)
            }

            _ => None,
        }
    }

    fn compute_unary_constant(
        &self,
        operand: RuntimeValue,
        operator: &Token,
    ) -> Option<RuntimeValue> {
        match operator.get_token_type() {
            TokenType::Minus => match operand {
                RuntimeValue::IntegerLiteral(v) => Some(RuntimeValue::IntegerLiteral(-v)),
                RuntimeValue::FloatLiteral(v) => Some(RuntimeValue::FloatLiteral(-v)),
                _ => None,
            },
            TokenType::Not => match operand {
                RuntimeValue::BooleanLiteral(v) => Some(RuntimeValue::BooleanLiteral(!v)),
                _ => None,
            },
            _ => None,
        }
    }

    fn compute_binary_constant(
        &mut self,
        left: RuntimeValue,
        right: RuntimeValue,
        operator: &Token,
    ) -> Option<RuntimeValue> {
        match (left, right) {
            (RuntimeValue::IntegerLiteral(l), RuntimeValue::IntegerLiteral(r)) => {
                self.compute_integer_op(l, r, operator)
            }
            (RuntimeValue::FloatLiteral(l), RuntimeValue::FloatLiteral(r)) => {
                self.compute_float_op(l, r, operator)
            }
            (RuntimeValue::StringLiteral(l_idx), RuntimeValue::StringLiteral(r_idx)) => {
                self.compute_string_op(l_idx, r_idx, operator)
            }
            (RuntimeValue::BooleanLiteral(l), RuntimeValue::BooleanLiteral(r)) => {
                self.compute_boolean_op(l, r, operator)
            }
            _ => None,
        }
    }

    fn compute_integer_op(&self, l: i32, r: i32, operator: &Token) -> Option<RuntimeValue> {
        match operator.get_token_type() {
            TokenType::Plus => Some(RuntimeValue::IntegerLiteral(l + r)),
            TokenType::Minus => Some(RuntimeValue::IntegerLiteral(l - r)),
            TokenType::Asterisk => Some(RuntimeValue::IntegerLiteral(l * r)),
            TokenType::Slash => Some(RuntimeValue::IntegerLiteral(l / r)),
            TokenType::Caret => Some(RuntimeValue::IntegerLiteral(l.pow(r as u32))),
            TokenType::LessThan => Some(RuntimeValue::BooleanLiteral(l < r)),
            TokenType::LessThanEqual => Some(RuntimeValue::BooleanLiteral(l <= r)),
            TokenType::GreaterThan => Some(RuntimeValue::BooleanLiteral(l > r)),
            TokenType::GreaterThanEqual => Some(RuntimeValue::BooleanLiteral(l >= r)),
            TokenType::Equal => Some(RuntimeValue::BooleanLiteral(l == r)),
            TokenType::NotEqual => Some(RuntimeValue::BooleanLiteral(l != r)),
            _ => None,
        }
    }

    fn compute_float_op(&self, l: f64, r: f64, operator: &Token) -> Option<RuntimeValue> {
        match operator.get_token_type() {
            TokenType::Plus => Some(RuntimeValue::FloatLiteral(l + r)),
            TokenType::Minus => Some(RuntimeValue::FloatLiteral(l - r)),
            TokenType::Asterisk => Some(RuntimeValue::FloatLiteral(l * r)),
            TokenType::Slash => Some(RuntimeValue::FloatLiteral(l / r)),
            TokenType::Caret => Some(RuntimeValue::FloatLiteral(l.powf(r))),
            TokenType::LessThan => Some(RuntimeValue::BooleanLiteral(l < r)),
            TokenType::LessThanEqual => Some(RuntimeValue::BooleanLiteral(l <= r)),
            TokenType::GreaterThan => Some(RuntimeValue::BooleanLiteral(l > r)),
            TokenType::GreaterThanEqual => Some(RuntimeValue::BooleanLiteral(l >= r)),
            TokenType::Equal => Some(RuntimeValue::BooleanLiteral(l == r)),
            TokenType::NotEqual => Some(RuntimeValue::BooleanLiteral(l != r)),
            _ => None,
        }
    }

    fn compute_string_op(
        &mut self,
        l_idx: usize,
        r_idx: usize,
        operator: &Token,
    ) -> Option<RuntimeValue> {
        match operator.get_token_type() {
            TokenType::Plus => {
                let left_str = self.get_intern_string(l_idx);
                let right_str = self.get_intern_string(r_idx);
                let result = format!("{}{}", left_str, right_str);
                let result_idx = self.intern_string(result);
                Some(RuntimeValue::StringLiteral(result_idx))
            }
            TokenType::Equal => Some(RuntimeValue::BooleanLiteral(l_idx == r_idx)),
            TokenType::NotEqual => Some(RuntimeValue::BooleanLiteral(l_idx != r_idx)),
            _ => None,
        }
    }

    fn compute_boolean_op(&self, l: bool, r: bool, operator: &Token) -> Option<RuntimeValue> {
        match operator.get_token_type() {
            TokenType::Equal => Some(RuntimeValue::BooleanLiteral(l == r)),
            TokenType::NotEqual => Some(RuntimeValue::BooleanLiteral(l != r)),
            _ => None,
        }
    }

    pub(crate) fn emit_constant(&mut self, constant: RuntimeValue, span: Span) {
        match &constant {
            RuntimeValue::BooleanLiteral(true) => {
                self.emit(OpCode::LoadBoolTrue, vec![], span);
            }
            RuntimeValue::BooleanLiteral(false) => {
                self.emit(OpCode::LoadBoolFalse, vec![], span);
            }
            RuntimeValue::StringLiteral(idx) => {
                self.emit(OpCode::LoadString, vec![*idx], span);
            }
            _ => {
                let idx = self.add_constant(constant);
                self.emit(OpCode::LoadConstant, vec![idx], span);
            }
        }
    }
}
