use crate::{
    ast::ast::{Expr, Expression},
    bytecode::bytecode::OpCode,
    compiler::compiler::Compiler,
    errors::VynError,
    runtime_value::values::RuntimeValue,
    tokens::{Token, TokenType},
    type_checker::type_checker::Type,
    utils::Span,
};

impl Compiler {
    pub(crate) fn compile_binary_expr(
        &mut self,
        left_type: Type,
        left: Expression,
        _right_type: Type,
        right: Expression,
        operator: Token,
        span: Span,
    ) -> Option<u8> {
        // Zero division check
        if operator.get_token_type() == TokenType::Slash {
            let right_val = match &right.node {
                Expr::IntegerLiteral(n) => Some(*n as f64),
                Expr::FloatLiteral(n) => Some(*n),
                _ => None,
            };

            if let Some(val) = right_val {
                if val == 0.0 {
                    self.throw_error(VynError::DivisionByZero { span });
                    return None;
                }
            }
        }

        // Try constant folding
        if let (Some(left_val), Some(right_val)) =
            (self.try_fold_expr(&left), self.try_fold_expr(&right))
        {
            if let Some(folded_result) = self.try_fold_binary(&left_val, &operator, &right_val) {
                return self.emit_constant_value(folded_result, span);
            }
        }

        let left_reg = self.compile_expression(left, None)?;
        let right_reg = self.compile_expression(right.clone(), None)?;

        let dest_reg = self.allocate_register()?;

        let op_type = operator.get_token_type();

        // Choose correct opcode based on operator and type
        let opcode = match (op_type, &left_type) {
            // Integer arithmetic
            (TokenType::Plus, Type::Integer) => OpCode::AddInt,
            (TokenType::Minus, Type::Integer) => OpCode::SubtractInt,
            (TokenType::Asterisk, Type::Integer) => OpCode::MultiplyInt,
            (TokenType::Slash, Type::Integer) => OpCode::DivideInt,
            (TokenType::Caret, Type::Integer) => OpCode::ExponentInt,

            // Float arithmetic
            (TokenType::Plus, Type::Float) => OpCode::AddFloat,
            (TokenType::Minus, Type::Float) => OpCode::SubtractFloat,
            (TokenType::Asterisk, Type::Float) => OpCode::MultiplyFloat,
            (TokenType::Slash, Type::Float) => OpCode::DivideFloat,
            (TokenType::Caret, Type::Float) => OpCode::ExponentFloat,

            // Integer comparisons
            (TokenType::LessThan, Type::Integer) => OpCode::LessInt,
            (TokenType::LessThanEqual, Type::Integer) => OpCode::LessEqualInt,
            (TokenType::GreaterThan, Type::Integer) => OpCode::GreaterInt,
            (TokenType::GreaterThanEqual, Type::Integer) => OpCode::GreaterEqualInt,

            // Float comparisons
            (TokenType::LessThan, Type::Float) => OpCode::LessFloat,
            (TokenType::LessThanEqual, Type::Float) => OpCode::LessEqualFloat,
            (TokenType::GreaterThan, Type::Float) => OpCode::GreaterFloat,
            (TokenType::GreaterThanEqual, Type::Float) => OpCode::GreaterEqualFloat,

            // Equality (works on any type)
            (TokenType::Equal, _) => OpCode::Equal,
            (TokenType::NotEqual, _) => OpCode::NotEqual,

            // String concatenation
            (TokenType::Plus, Type::String) => OpCode::ConcatString,

            _ => {
                self.throw_error(VynError::TypeMismatch {
                    expected: vec![Type::Integer, Type::Float],
                    found: left_type,
                    span,
                });
                return None;
            }
        };

        self.emit(
            opcode,
            vec![dest_reg as usize, left_reg as usize, right_reg as usize],
            span,
        );

        self.free_register(left_reg);
        self.free_register(right_reg);

        Some(dest_reg)
    }

    fn emit_constant_value(&mut self, value: RuntimeValue, span: Span) -> Option<u8> {
        let dest_reg = self.allocate_register()?;

        match value {
            RuntimeValue::IntegerLiteral(v) => {
                let const_idx = self.add_constant(RuntimeValue::IntegerLiteral(v));
                self.emit(
                    OpCode::LoadConstInt,
                    vec![dest_reg as usize, const_idx],
                    span,
                );
            }
            RuntimeValue::FloatLiteral(v) => {
                let const_idx = self.add_constant(RuntimeValue::FloatLiteral(v));
                self.emit(
                    OpCode::LoadConstFloat,
                    vec![dest_reg as usize, const_idx],
                    span,
                );
            }
            RuntimeValue::BooleanLiteral(true) => {
                self.emit(OpCode::LoadTrue, vec![dest_reg as usize], span);
            }
            RuntimeValue::BooleanLiteral(false) => {
                self.emit(OpCode::LoadFalse, vec![dest_reg as usize], span);
            }
            RuntimeValue::StringLiteral(idx) => {
                self.emit(OpCode::LoadString, vec![dest_reg as usize, idx], span);
            }
            RuntimeValue::NilLiteral => {
                self.emit(OpCode::LoadNil, vec![dest_reg as usize], span);
            }
            _ => return None,
        }

        Some(dest_reg)
    }
}
