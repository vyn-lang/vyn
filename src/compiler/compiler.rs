use std::{collections::HashSet, mem, vec};

use crate::{
    ast::{
        ast::{Expr, Expression, Program, Statement, Stmt},
        type_annotation::TypeAnnotation,
    },
    bytecode::bytecode::{Instructions, OpCode},
    compiler::{debug_info::DebugInfo, symbol_table::SymbolTable},
    error_handler::{error_collector::ErrorCollector, errors::VynError},
    runtime_value::values::RuntimeValue,
    type_checker::{static_evaluator::StaticEvaluator, type_checker::Type},
    utils::Span,
};

pub struct Compiler<'a> {
    pub instructions: Instructions,
    pub constants: Vec<RuntimeValue>,
    pub string_table: Vec<String>,
    pub debug_info: DebugInfo,
    pub symbol_table: SymbolTable,

    next_register: u8,
    free_registers: Vec<u8>,
    pinned_registers: HashSet<u8>,
    errors: ErrorCollector,

    static_eval: &'a StaticEvaluator,
    loop_stack: Vec<LoopContext>,
}

#[derive(Debug, Clone)]
struct LoopContext {
    start_offset: usize,       // Where the loop begins (for continue)
    break_patches: Vec<usize>, // Positions of break jumps to patch later
}

#[derive(Debug)]
pub struct Bytecode {
    pub instructions: Instructions,
    pub constants: Vec<RuntimeValue>,
    pub string_table: Vec<String>,
    pub debug_info: DebugInfo,
}

impl<'a> Compiler<'a> {
    pub fn new(static_eval: &'a StaticEvaluator) -> Self {
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

            static_eval,
            loop_stack: Vec::new(),
        }
    }

    /// Main entry point
    pub fn compile_program(&mut self, program: Program) -> Result<Bytecode, ErrorCollector> {
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
                self.compile_expression(expression, None)?;
                Some(())
            }

            Stmt::VariableDeclaration {
                identifier,
                value,
                annotated_type,
                ..
            } => {
                let var_name = match identifier.node {
                    Expr::Identifier(n) => n,
                    _ => unreachable!("Variable name must be identifier"),
                };

                let expected_type =
                    Type::from_anotated_type(&annotated_type, self.static_eval, &mut self.errors);

                // Pass the expected type down to compile_expression

                let exported_value = match value {
                    Some(v) => v,
                    _ => Type::get_type_default_value(&expected_type),
                };
                let value_reg = self.compile_expression(exported_value, Some(&expected_type))?;
                self.pin_register(value_reg);

                match self.symbol_table.declare_identifier(
                    var_name.clone(),
                    span,
                    expected_type,
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

            Stmt::StaticVariableDeclaration {
                identifier,
                value,
                annotated_type,
            } => {
                let var_name = match identifier.node {
                    Expr::Identifier(n) => n,
                    _ => unreachable!("Variable name must be identifier"),
                };

                let expected_type =
                    Type::from_anotated_type(&annotated_type, self.static_eval, &mut self.errors);

                let value_reg = self.compile_expression(value, Some(&expected_type))?;
                self.pin_register(value_reg);

                match self.symbol_table.declare_identifier(
                    var_name.clone(),
                    span,
                    expected_type,
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

            Stmt::TypeAliasDeclaration { .. } => Some(()),

            Stmt::StdoutLog { log_value } => {
                let src = self.compile_expression(log_value, None)?;

                self.emit(OpCode::LogAddr, vec![src as usize], span);
                self.free_register(src);
                Some(())
            }

            Stmt::Scope { statements } => {
                self.enter_scope();

                let pinned_before = self.pinned_registers.clone();

                for stmt in statements {
                    self.try_compile_statement(stmt)?;
                }

                let mut newly_pinned = Vec::new();
                for reg in &self.pinned_registers {
                    if !pinned_before.contains(reg) {
                        newly_pinned.push(*reg);
                    }
                }

                for reg in newly_pinned {
                    self.unpin_register(reg);
                }

                self.exit_scope();
                Some(())
            }

            Stmt::IfDeclaration {
                condition,
                consequence,
                alternate,
            } => {
                let cond_idx = self.compile_expression(condition, None)?;

                let jump_false_pos =
                    self.emit(OpCode::JumpIfFalse, vec![cond_idx as usize, 9999], span);

                self.free_register(cond_idx);

                self.try_compile_statement(*consequence)?;

                let end_block = self.instructions.len();
                OpCode::change_operand(
                    &mut self.instructions,
                    jump_false_pos,
                    vec![cond_idx as usize, end_block],
                );

                if let Some(alt) = alternate.as_ref() {
                    let jump_pos = self.emit(OpCode::JumpUncond, vec![9999], span);

                    let end_block = self.instructions.len();
                    OpCode::change_operand(
                        &mut self.instructions,
                        jump_false_pos,
                        vec![cond_idx as usize, end_block],
                    );

                    self.try_compile_statement(alt.clone())?;

                    let end_jmp_block = self.instructions.len();
                    OpCode::change_operand(&mut self.instructions, jump_pos, vec![end_jmp_block]);
                }
                Some(())
            }

            Stmt::WhenLoop { body, condition } => {
                let start_loop = self.instructions.len();

                let comp_condition = self.compile_expression(condition, Some(&Type::Bool))?;
                let jump_false = self.emit(OpCode::JumpIfFalse, vec![9999, 9999], span);

                self.try_compile_statement(*body)?;

                self.emit(OpCode::JumpUncond, vec![start_loop], span);
                let end_loop = self.instructions.len();
                OpCode::change_operand(
                    &mut self.instructions,
                    jump_false,
                    vec![comp_condition as usize, end_loop],
                );

                self.free_register(comp_condition);

                Some(())
            }

            Stmt::Loop { body } => {
                let loop_start = self.instructions.len();

                self.loop_stack.push(LoopContext {
                    start_offset: loop_start,
                    break_patches: Vec::new(),
                });

                self.try_compile_statement(*body)?;

                self.emit(OpCode::JumpUncond, vec![loop_start], span);

                let loop_ctx = self.loop_stack.pop().unwrap();
                let loop_end = self.instructions.len();

                for patch_pos in loop_ctx.break_patches {
                    OpCode::change_operand(&mut self.instructions, patch_pos, vec![loop_end]);
                }

                Some(())
            }

            Stmt::Continue => {
                // Jump directly back to loop start
                let loop_start = self.loop_stack.last().unwrap().start_offset;
                self.emit(OpCode::JumpUncond, vec![loop_start], span);

                Some(())
            }

            Stmt::Break => {
                // Emit jump with placeholder target
                let jump_pos = self.emit(OpCode::JumpUncond, vec![9999], span);

                // Record this position for later patching
                self.loop_stack
                    .last_mut()
                    .unwrap()
                    .break_patches
                    .push(jump_pos);

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

    pub(crate) fn compile_expression(
        &mut self,
        expr: Expression,
        expected_type: Option<&Type>,
    ) -> Option<u8> {
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
                let symbol = match self.symbol_table.resolve_identifier(&name, span) {
                    Ok(s) => s,
                    Err(he) => {
                        self.throw_error(he);
                        return None;
                    }
                };

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

                let src_reg = self.compile_expression(*new_value, None)?;

                self.emit(
                    OpCode::Move,
                    vec![dest_reg as usize, src_reg as usize],
                    span,
                );

                self.free_register(src_reg);
                Some(dest_reg)
            }

            Expr::ArrayLiteral { elements } => {
                self.compile_array_literal(elements, expected_type, span)
            }

            Expr::Index { target, property } => {
                // Get the type BEFORE consuming target
                let target_type = self.get_expr_type(&target)?;

                let dest = self.allocate_register()?;
                let target_reg = self.compile_expression(*target, None)?;
                let property_reg = self.compile_expression(*property, None)?;

                match target_type {
                    Type::Array(_, _) | Type::Sequence(_) => {
                        self.emit(
                            OpCode::ArrayGet,
                            vec![dest as usize, target_reg as usize, property_reg as usize],
                            span,
                        );
                    }

                    _ => {
                        unreachable!()
                    }
                }

                self.free_register(target_reg);
                self.free_register(property_reg);

                Some(dest)
            }

            Expr::IndexAssignment {
                target,
                property,
                new_value,
            } => {
                // Get type BEFORE consuming target
                let target_type = self.get_expr_type(&target)?;

                let target_reg = self.compile_expression(*target, None)?;
                let index_reg = self.compile_expression(*property, Some(&Type::Integer))?;
                let value_reg = self.compile_expression(*new_value, None)?;

                match target_type {
                    Type::Array(_, _) | Type::Sequence(_) => {
                        self.emit(
                            OpCode::ArraySet,
                            vec![target_reg as usize, index_reg as usize, value_reg as usize],
                            span,
                        );
                    }

                    _ => unreachable!(),
                }

                self.free_register(target_reg);
                self.free_register(index_reg);
                self.free_register(value_reg);
                Some(target_reg)
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

    fn compile_array_literal(
        &mut self,
        elements: Vec<Box<Expression>>,
        expected_type: Option<&Type>,
        span: Span,
    ) -> Option<u8> {
        match expected_type? {
            Type::Array(t, size) => {
                let dest = self.allocate_register()?;

                self.emit(OpCode::ArrayNewFixed, vec![dest as usize, *size], span);

                for (i, elem) in elements.iter().enumerate() {
                    let elem_reg = self.compile_expression(*elem.clone(), Some(t))?;

                    self.emit(
                        OpCode::ArraySet,
                        vec![dest as usize, i, elem_reg as usize],
                        span,
                    );

                    self.free_register(elem_reg);
                }

                Some(dest)
            }

            Type::Sequence(t) => {
                let dest = self.allocate_register()?;

                self.emit(
                    OpCode::ArrayNewDynamic,
                    vec![dest as usize, elements.len()],
                    span,
                );
                for elem in elements.iter() {
                    let elem_reg = self.compile_expression(*elem.clone(), Some(t))?;

                    self.emit(
                        OpCode::ArrayPush,
                        vec![dest as usize, elem_reg as usize],
                        span,
                    );

                    self.free_register(elem_reg);
                }

                Some(dest)
            }

            _ => unreachable!(),
        }
    }

    pub(crate) fn allocate_register(&mut self) -> Option<u8> {
        if let Some(reg) = self.free_registers.pop() {
            return Some(reg);
        }

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

    pub(crate) fn throw_error(&mut self, error: VynError) {
        self.errors.add(error);
    }

    pub(crate) fn emit(&mut self, opcode: OpCode, operands: Vec<usize>, span: Span) -> usize {
        let instruction = OpCode::make(opcode, operands);
        let position = self.add_instruction(instruction, span);
        position
    }

    pub(crate) fn enter_scope(&mut self) {
        self.symbol_table = self.symbol_table.enter_scope();
    }

    pub(crate) fn exit_scope(&mut self) {
        self.symbol_table = mem::take(&mut self.symbol_table).exit_scope();
    }

    pub(crate) fn add_instruction(&mut self, instruction: Instructions, span: Span) -> usize {
        let position = self.instructions.len();

        for byte in instruction {
            let offset = self.instructions.len();

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
