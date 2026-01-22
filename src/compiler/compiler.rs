use std::{collections::HashSet, mem};

use crate::{
    ast::ast::{Expr, Expression, Program, Statement, Stmt},
    bytecode::bytecode::{Instructions, OpCode},
    compiler::{debug_info::DebugInfo, symbol_table::SymbolTable},
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
                    // unreachable, the parser already checks this
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

            Stmt::TypeAliasDeclaration { .. } => {
                // Do nothing, this should only
                // be used while type checking
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

            Expr::BooleanLiteral(truthy) => {
                let dest = self.allocate_register()?;

                if truthy {
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
                // This does a direct mapping
                // it takes the value of the variable based on the
                // register and returns that value
                let symbol = match self.symbol_table.resolve_identifier(&name, span) {
                    Ok(s) => s,
                    Err(he) => {
                        self.throw_error(he);
                        return None;
                    }
                };

                // Just return the register where the value of variable lives
                Some(symbol.register)
            }

            Expr::Unary {
                ref operator,
                ref right,
            } => self.compile_unary_expr(operator.clone(), right, span),

            Expr::BinaryOperation {
                left,
                operator,
                right,
            } => {
                let left_type = self.get_expr_type(&left)?;
                let right_type = self.get_expr_type(&right)?;

                self.compile_binary_expr(left_type, *left, right_type, *right, operator, span)
            }

            Expr::VariableAssignment {
                identifier,
                new_value,
            } => {
                let name = match identifier.node {
                    Expr::Identifier(n) => n,
                    _ => unreachable!("Assignment target must be identifier"),
                };

                let dest_reg = match self.symbol_table.resolve_identifier(&name, span) {
                    Ok(symbol) => symbol.register,
                    Err(ve) => {
                        self.throw_error(ve);
                        return None;
                    }
                };

                let src_reg = self.compile_expression(*new_value)?;

                self.emit(
                    OpCode::Move,
                    vec![dest_reg as usize, src_reg as usize],
                    span,
                );

                self.free_register(src_reg);
                Some(dest_reg)
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

    pub(crate) fn allocate_register(&mut self) -> Option<u8> {
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

    pub(crate) fn free_register(&mut self, reg: u8) {
        if !self.pinned_registers.contains(&reg) {
            self.free_registers.push(reg);
        }
    }

    pub(crate) fn allocate_pinned_register(&mut self) -> Option<u8> {
        let reg = self.allocate_register()?;
        self.pin_register(reg);
        Some(reg)
    }

    pub(crate) fn pin_register(&mut self, reg: u8) {
        self.pinned_registers.insert(reg);
    }

    pub(crate) fn unpin_register(&mut self, reg: u8) {
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
