use std::{collections::HashSet, mem};

use crate::{
    ast::ast::{Expr, Expression, Program, Statement, Stmt},
    bytecode::bytecode::{Instructions, OpCode},
    compiler::symbol_table::SymbolTable,
    errors::{ErrorCollector, VynError},
    runtime_value::RuntimeValue,
    tokens::TokenType,
    type_checker::type_checker::{Type, TypeChecker},
    utils::Span,
};

pub struct Compiler {
    instructions: Instructions,
    constants: Vec<RuntimeValue>,
    string_table: Vec<String>,
    debug_info: DebugInfo,

    next_register: u8,
    free_registers: Vec<u8>, // TODO: These might need a seperate struct
    pinned_registers: HashSet<u8>,
    symbol_table: SymbolTable,
    errors: ErrorCollector,
}

#[derive(Debug)]
pub struct Bytecode {
    pub instructions: Instructions,
    pub constants: Vec<RuntimeValue>,
    pub string_table: Vec<String>,
    pub debug_info: DebugInfo,
}

/// Run-length encoded debug information
#[derive(Default, Debug)]
pub struct DebugInfo {
    pub line_changes: Vec<(usize, u32)>,
    pub start_col_changes: Vec<(usize, u32)>,
    pub end_col_changes: Vec<(usize, u32)>,
}

impl DebugInfo {
    pub fn new() -> Self {
        Self {
            line_changes: Vec::new(),
            start_col_changes: Vec::new(),
            end_col_changes: Vec::new(),
        }
    }

    pub fn get_span(&self, ip: usize) -> Span {
        let line = self.find_value(&self.line_changes, ip);
        let start_column = self.find_value(&self.start_col_changes, ip);
        let end_column = self.find_value(&self.end_col_changes, ip);

        Span {
            line,
            start_column,
            end_column,
        }
    }

    fn find_value(&self, changes: &Vec<(usize, u32)>, ip: usize) -> u32 {
        if changes.is_empty() {
            return 0;
        }

        let idx = changes
            .binary_search_by_key(&ip, |&(offset, _)| offset)
            .unwrap_or_else(|i| i.saturating_sub(1));

        changes[idx].1
    }
}

impl Compiler {
    pub fn new() -> Self {
        Self {
            instructions: Vec::new(),
            constants: Vec::new(),
            string_table: Vec::new(),
            free_registers: Vec::new(),
            pinned_registers: HashSet::new(),
            debug_info: DebugInfo::new(),
            next_register: 0,

            symbol_table: SymbolTable::new(),
            errors: ErrorCollector::new(),
        }
    }

    /// Main entry point
    pub fn compile_program(&mut self, program: Program) -> Result<Bytecode, ErrorCollector> {
        // Type check the entire program before compiling
        let mut type_checker = TypeChecker::new();
        type_checker.check_program(&program)?;

        for stmt in program.statements {
            let result = self.try_compile_statement(stmt);

            if result.is_none() {
                break;
            }
        }

        self.emit(OpCode::Halt, vec![], Span::default());

        if self.errors.has_errors() {
            Err(mem::take(&mut self.errors))
        } else {
            Ok(self.bytecode())
        }
    }

    pub(crate) fn try_compile_statement(&mut self, stmt: Statement) -> Option<()> {
        let span = stmt.span;

        match stmt.node {
            Stmt::Expression { expression } => {
                self.compile_expression(expression)?;
                Some(())
            }

            Stmt::VariableDeclaration {
                identifier,
                value,
                span,
                annotated_type,
                ..
            } => {
                let var_name = match identifier.node {
                    Expr::Identifier(n) => n,
                    _ => unreachable!("Variable name must be identifier"),
                };

                let value_reg = self.compile_expression(value)?;
                self.pin_register(value_reg);

                match self.symbol_table.declare_identifier(
                    var_name.clone(),
                    span,
                    Type::from_anotated_type(&annotated_type),
                    value_reg,
                ) {
                    Ok(_) => {}
                    Err(he) => {
                        self.throw_error(he);
                        return None;
                    }
                };

                Some(())
            }

            unknown => {
                self.throw_error(VynError::UnknownAST {
                    node: unknown.to_node(),
                    span,
                });
                None
            }
        }
    }

    pub(crate) fn compile_expression(&mut self, expr: Expression) -> Option<u8> {
        let span = expr.span;

        match expr.node {
            Expr::IntegerLiteral(v) => {
                let dest = self.allocate_register()?;
                let const_idx = self.add_constant(RuntimeValue::IntegerLiteral(v));

                self.emit(OpCode::LoadConstInt, vec![dest as usize, const_idx], span);

                Some(dest)
            }

            Expr::FloatLiteral(v) => {
                let dest = self.allocate_register()?;
                let const_idx = self.add_constant(RuntimeValue::FloatLiteral(v));

                self.emit(OpCode::LoadConstFloat, vec![dest as usize, const_idx], span);

                Some(dest)
            }

            Expr::BooleanLiteral(truethy) => {
                let dest = self.allocate_register()?;

                if truethy {
                    self.emit(OpCode::LoadTrue, vec![dest as usize], span);
                } else {
                    self.emit(OpCode::LoadFalse, vec![dest as usize], span);
                }

                Some(dest)
            }

            Expr::StringLiteral(v) => {
                let dest = self.allocate_register()?;
                let str_idx = self.intern_string(v);

                self.emit(OpCode::LoadString, vec![dest as usize, str_idx], span);

                Some(dest)
            }

            Expr::NilLiteral => {
                let dest = self.allocate_register()?;

                self.emit(OpCode::LoadNil, vec![dest as usize], span);

                Some(dest)
            }

            Expr::Identifier(name) => {
                let symbol = match self.symbol_table.resolve_identifier(&name, span) {
                    Ok(s) => s,
                    Err(he) => {
                        self.throw_error(he);
                        return None;
                    }
                };

                // Just return the register where this variable lives
                Some(symbol.register)
            }

            Expr::Unary {
                ref operator,
                ref right,
            } => {
                let right_expr = (**right).clone();
                let operand_type = self.get_expr_type(right)?;

                let src_reg = self.compile_expression(right_expr)?;
                let dest_reg = self.allocate_register()?;

                match operator.get_token_type() {
                    TokenType::Minus => match operand_type {
                        Type::Integer => {
                            self.emit(
                                OpCode::NegateInt,
                                vec![dest_reg as usize, src_reg as usize],
                                span,
                            );
                        }
                        Type::Float => {
                            self.emit(
                                OpCode::NegateFloat,
                                vec![dest_reg as usize, src_reg as usize],
                                span,
                            );
                        }
                        _ => {
                            self.throw_error(VynError::TypeMismatch {
                                expected: vec![Type::Integer, Type::Float],
                                found: operand_type,
                                span,
                            });
                            return None;
                        }
                    },
                    TokenType::Not => {
                        self.emit(OpCode::Not, vec![dest_reg as usize, src_reg as usize], span);
                    }
                    _ => unreachable!("Unhandled unary operator type"),
                };

                Some(dest_reg)
            }

            Expr::BinaryOperation {
                left,
                operator,
                right,
            } => {
                let left_type = self.get_expr_type(&left)?;
                let right_type = self.get_expr_type(&right)?;

                self.compile_binary_expr(left_type, *left, right_type, *right, operator, span)
            }

            unknown => {
                self.throw_error(VynError::UnknownAST {
                    node: unknown.to_node(),
                    span,
                });
                None
            }
        }
    }

    pub(crate) fn compile_binary_expr(
        &mut self,
        left_type: Type,
        left: Expression,
        _right_type: Type,
        right: Expression,
        operator: crate::tokens::Token,
        span: Span,
    ) -> Option<u8> {
        let left_reg = self.compile_expression(left)?;
        let right_reg = self.compile_expression(right)?;
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

    pub(crate) fn get_expr_type(&mut self, expr: &Expression) -> Option<Type> {
        match &expr.node {
            Expr::IntegerLiteral(_) => Some(Type::Integer),
            Expr::FloatLiteral(_) => Some(Type::Float),
            Expr::BooleanLiteral(_) => Some(Type::Bool),
            Expr::StringLiteral(_) => Some(Type::String),
            Expr::NilLiteral => Some(Type::Nil),
            Expr::Identifier(n) => {
                let symbol = self.symbol_table.resolve_identifier(n, expr.span);
                match symbol {
                    Ok(s) => Some(s.symbol_type.clone()),
                    Err(he) => {
                        self.throw_error(he);
                        None
                    }
                }
            }

            Expr::Unary { right, operator } => match operator.get_token_type() {
                TokenType::Minus => self.get_expr_type(right),
                TokenType::Not => Some(Type::Bool),
                _ => unreachable!(),
            },

            Expr::BinaryOperation { left, operator, .. } => match operator.get_token_type() {
                TokenType::Plus
                | TokenType::Minus
                | TokenType::Asterisk
                | TokenType::Slash
                | TokenType::Caret => self.get_expr_type(left),

                TokenType::LessThan
                | TokenType::LessThanEqual
                | TokenType::GreaterThan
                | TokenType::GreaterThanEqual
                | TokenType::Equal
                | TokenType::NotEqual => Some(Type::Bool),

                _ => unreachable!(),
            },

            _ => unreachable!("Unknown expression type\n\n{:#?}", expr.node),
        }
    }

    fn allocate_register(&mut self) -> Option<u8> {
        // First, try to reuse a freed register
        if let Some(reg) = self.free_registers.pop() {
            return Some(reg);
        }

        // Otherwise allocate a new one
        if self.next_register >= u8::MAX {
            self.throw_error(VynError::RegisterOverflow {
                span: Span::default(),
            });
            return None;
        }

        let reg = self.next_register;
        self.next_register += 1;
        Some(reg)
    }

    fn free_register(&mut self, reg: u8) {
        if !self.pinned_registers.contains(&reg) {
            self.free_registers.push(reg);
        }
    }

    fn allocate_pinned_register(&mut self) -> Option<u8> {
        let reg = self.allocate_register()?;
        self.pin_register(reg);
        Some(reg)
    }

    fn pin_register(&mut self, reg: u8) {
        self.pinned_registers.insert(reg);
    }

    fn unpin_register(&mut self, reg: u8) {
        self.pinned_registers.remove(&reg);
        self.free_register(reg);
    }

    /// Add a string to the string table (with deduplication)
    pub(crate) fn intern_string(&mut self, s: String) -> usize {
        if let Some(pos) = self.string_table.iter().position(|existing| existing == &s) {
            return pos;
        }

        self.string_table.push(s);
        self.string_table.len() - 1
    }

    pub(crate) fn get_intern_string(&self, idx: usize) -> String {
        self.string_table[idx].clone()
    }

    pub fn bytecode(&mut self) -> Bytecode {
        Bytecode {
            instructions: mem::take(&mut self.instructions),
            constants: mem::take(&mut self.constants),
            string_table: mem::take(&mut self.string_table),
            debug_info: mem::take(&mut self.debug_info),
        }
    }

    /// Add a constant to the constants table
    pub(crate) fn add_constant(&mut self, value: RuntimeValue) -> usize {
        if let Some(pos) = self
            .constants
            .iter()
            .position(|existing| existing == &value)
        {
            return pos;
        }

        self.constants.push(value);
        self.constants.len() - 1
    }

    /// Record a compilation error
    pub(crate) fn throw_error(&mut self, error: VynError) {
        self.errors.add(error);
    }

    /// Emit an instruction with span tracking
    pub(crate) fn emit(&mut self, opcode: OpCode, operands: Vec<usize>, span: Span) -> usize {
        let instruction = OpCode::make(opcode, operands);
        let position = self.add_instruction(instruction, span);
        position
    }

    /// Add instruction bytes and track their span
    pub(crate) fn add_instruction(&mut self, instruction: Instructions, span: Span) -> usize {
        let position = self.instructions.len();

        for byte in instruction {
            let offset = self.instructions.len();

            // Compressed span tracking (only record when values change)
            if self.should_add_line_change(span.line) {
                self.debug_info.line_changes.push((offset, span.line));
            }

            if self.should_add_col_change(&self.debug_info.start_col_changes, span.start_column) {
                self.debug_info
                    .start_col_changes
                    .push((offset, span.start_column));
            }

            if self.should_add_col_change(&self.debug_info.end_col_changes, span.end_column) {
                self.debug_info
                    .end_col_changes
                    .push((offset, span.end_column));
            }

            self.instructions.push(byte);
        }

        position
    }

    fn should_add_line_change(&self, line: u32) -> bool {
        self.debug_info.line_changes.is_empty()
            || self.debug_info.line_changes.last().unwrap().1 != line
    }

    fn should_add_col_change(&self, changes: &Vec<(usize, u32)>, col: u32) -> bool {
        changes.is_empty() || changes.last().unwrap().1 != col
    }
}
