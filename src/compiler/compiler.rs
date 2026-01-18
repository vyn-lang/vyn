use std::mem;

use crate::{
    ast::ast::{Expr, Expression, Program, Statement, Stmt},
    bytecode::bytecode::{Instructions, OpCode},
    errors::{ErrorCollector, HydorError},
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
    errors: ErrorCollector,
}

pub struct Bytecode {
    pub instructions: Instructions,
    pub constants: Vec<RuntimeValue>,
    pub string_table: Vec<String>,
    pub debug_info: DebugInfo,
}

/// Run-length encoded debug information
#[derive(Default)]
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
            debug_info: DebugInfo::new(),
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

        self.emit(
            OpCode::Halt,
            vec![],
            Span {
                line: 0,
                start_column: 0,
                end_column: 0,
            },
        );

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
                self.emit(OpCode::Pop, vec![], span);
                Some(())
            }

            unknown => {
                self.throw_error(HydorError::UnknownAST {
                    node: unknown.to_node(),
                    span,
                });
                return None;
            }
        }
    }

    pub(crate) fn compile_expression(&mut self, expr: Expression) -> Option<()> {
        let span = expr.span;

        match expr.node {
            Expr::IntegerLiteral(v) => {
                let idx = self.add_constant(RuntimeValue::IntegerLiteral(v));
                self.emit(OpCode::LoadConstant, vec![idx], span);
            }

            Expr::FloatLiteral(v) => {
                let idx = self.add_constant(RuntimeValue::FloatLiteral(v));
                self.emit(OpCode::LoadConstant, vec![idx], span);
            }

            Expr::BooleanLiteral(truethy) => {
                if truethy {
                    self.emit(OpCode::LoadBoolTrue, vec![], span);
                } else {
                    self.emit(OpCode::LoadBoolFalse, vec![], span);
                }
            }

            Expr::StringLiteral(v) => {
                let str_idx = self.intern_string(v);
                self.emit(OpCode::LoadString, vec![str_idx], span);
            }

            Expr::NilLiteral => {
                self.emit(OpCode::LoadNil, vec![], span);
            }

            Expr::Unary {
                ref operator,
                ref right,
            } => {
                // Try to fold unary expression first
                if let Some(folded) = self.eval_to_constant(&expr) {
                    self.emit_constant(folded, span);
                    return Some(());
                }

                // Can't fold, compile normally
                let right_expr = (**right).clone();
                let operand_type = self.get_expr_type(right);

                self.compile_expression(right_expr)?;

                match operator.get_token_type() {
                    TokenType::Minus => match operand_type {
                        Type::Integer => self.emit(OpCode::UnaryNegateInt, vec![], span),
                        Type::Float => self.emit(OpCode::UnaryNegateFloat, vec![], span),
                        _ => {
                            self.throw_error(HydorError::TypeMismatch {
                                expected: vec![Type::Integer, Type::Float],
                                found: operand_type,
                                span,
                            });
                            return None;
                        }
                    },
                    TokenType::Not => self.emit(OpCode::UnaryNot, vec![], span),
                    _ => unreachable!("Unhandled unary operator type"),
                };
            }

            Expr::BinaryOperation {
                left,
                operator,
                right,
            } => {
                if self
                    .try_fold_binary(&left, &right, &operator, expr.span)
                    .is_some()
                {
                    return Some(());
                }

                let left_type = self.get_expr_type(&left);
                let right_type = self.get_expr_type(&right);

                self.compile_binary_expr(
                    left_type, *left, right_type, *right, operator, expr.span,
                )?;
            }

            unknown => {
                self.throw_error(HydorError::UnknownAST {
                    node: unknown.to_node(),
                    span,
                });
                return None;
            }
        }

        Some(())
    }

    pub(crate) fn get_expr_type(&self, expr: &Expression) -> Type {
        match &expr.node {
            Expr::IntegerLiteral(_) => Type::Integer,
            Expr::FloatLiteral(_) => Type::Float,
            Expr::BooleanLiteral(_) => Type::Bool,
            Expr::StringLiteral(_) => Type::String,
            Expr::NilLiteral => Type::Nil,

            Expr::Unary { right, operator } => {
                match operator.get_token_type() {
                    TokenType::Minus => {
                        // -x is Integer if x is Integer
                        self.get_expr_type(right)
                    }
                    TokenType::Not => {
                        // !x is always Bool
                        Type::Bool
                    }
                    _ => unreachable!(),
                }
            }

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
                | TokenType::NotEqual => Type::Bool,

                _ => unreachable!(),
            },

            _ => unreachable!("Unknown expression type"),
        }
    }

    /// Add a string to the string table (with deduplication)
    pub(crate) fn intern_string(&mut self, s: String) -> usize {
        // Check if we already have this string
        if let Some(pos) = self.string_table.iter().position(|existing| existing == &s) {
            return pos;
        }

        // New string, add it
        self.string_table.push(s);
        self.string_table.len() - 1
    }

    pub(crate) fn get_intern_string(&mut self, idx: usize) -> String {
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

    pub(crate) fn extract_constant_value(&mut self, expr: &Expression) -> Option<RuntimeValue> {
        match &expr.node {
            Expr::IntegerLiteral(value) => Some(RuntimeValue::IntegerLiteral(*value)),
            Expr::FloatLiteral(value) => Some(RuntimeValue::FloatLiteral(*value)),
            Expr::StringLiteral(value) => {
                let intern_idx = self.intern_string(value.to_string());
                Some(RuntimeValue::StringLiteral(intern_idx))
            }
            Expr::BooleanLiteral(value) => Some(RuntimeValue::BooleanLiteral(*value)),
            Expr::NilLiteral => Some(RuntimeValue::NilLiteral),
            // Return None for non-constant expressions (identifiers, binary ops, etc.)
            _ => None,
        }
    }

    /// Emit an instruction with span tracking
    pub(crate) fn emit(&mut self, opcode: OpCode, operands: Vec<usize>, span: Span) -> usize {
        let instruction = OpCode::make(opcode, operands);
        let position = self.add_instruction(instruction, span);
        position
    }

    /// Add a constant to the constants table
    pub(crate) fn add_constant(&mut self, value: RuntimeValue) -> usize {
        self.constants.push(value);
        self.constants.len() - 1
    }

    /// Record a compilation error
    pub(crate) fn throw_error(&mut self, error: HydorError) {
        self.errors.add(error);
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
