use std::mem;

use crate::{
    ast::ast::{Expr, Expression, Program, Statement, Stmt},
    bytecode::bytecode::OpCode,
    error_handler::error_collector::ErrorCollector,
    ir::{
        ir_instr::{Label, VReg, VynIROC, VynIROpCode},
        symbol_ir_table::{SymbolScope, SymbolTable},
    },
    type_checker::{
        static_evaluator::StaticEvaluator, symbol_type_table::SymbolTypeTable, type_checker::Type,
    },
    utils::Span,
};

pub struct VynIRBuilder<'a> {
    instructions: Vec<VynIROpCode>,
    next_register: VReg,
    label_counter: usize,
    pub(crate) error_collector: ErrorCollector,
    pub(crate) static_eval: &'a StaticEvaluator,
    pub(crate) symbol_type_table: &'a SymbolTypeTable,
    pub(crate) symbol_table: SymbolTable,

    // Loop context
    break_jump_pos: Option<Label>,
    continue_jump_pos: Option<Label>,
}

pub struct VynIR {
    pub instructions: Vec<VynIROpCode>,
}

impl<'a> VynIRBuilder<'a> {
    pub fn new(static_eval: &'a StaticEvaluator, symbol_type_table: &'a SymbolTypeTable) -> Self {
        Self {
            instructions: Vec::new(),
            error_collector: ErrorCollector::new(),
            next_register: 0,
            label_counter: 0,
            static_eval,
            break_jump_pos: None,
            continue_jump_pos: None,
            symbol_type_table,
            symbol_table: SymbolTable::new(),
        }
    }

    pub fn build_ir(&mut self, program: &Program) -> Result<VynIR, ErrorCollector> {
        for stmt in &program.statements {
            self.build_stmt(stmt, stmt.span);
        }

        self.emit(VynIROC::Halt.spanned(Span::default()));

        if self.error_collector.has_errors() {
            Err(mem::take(&mut self.error_collector))
        } else {
            Ok(self.finish())
        }
    }

    fn build_stmt(&mut self, stmt: &Statement, span: Span) -> Option<()> {
        match &stmt.node {
            Stmt::Expression { expression } => {
                self.build_expr(expression);
            }

            Stmt::VariableDeclaration {
                identifier,
                value,
                annotated_type,
                mutable,
            } => {
                let var_name = match identifier.node.clone() {
                    Expr::Identifier(n) => n,
                    _ => unreachable!(),
                };

                let symbol_type = Type::from_anotated_type(
                    annotated_type,
                    &mut self.static_eval,
                    &mut self.error_collector,
                );

                let value_vreg = if let Some(val) = value {
                    self.build_expr(val)?
                } else {
                    let val_type = Type::from_anotated_type(
                        annotated_type,
                        &self.static_eval,
                        &mut self.error_collector,
                    );
                    let value = Type::get_type_default_value(&val_type);
                    self.build_expr(&value)?
                };

                self.symbol_table.declare_ident_with_register(
                    symbol_type,
                    var_name.clone(),
                    *mutable,
                    value_vreg as u8,
                    span,
                    &mut self.error_collector,
                );
            }

            Stmt::Loop { body } => {
                let loop_start = self.next_label();
                let loop_end = self.next_label();

                let prev_break_jump_pos = self.break_jump_pos;
                self.break_jump_pos = Some(loop_end);

                let prev_continue_jump_pos = self.continue_jump_pos;
                self.continue_jump_pos = Some(loop_start);

                self.emit_label(loop_start);
                self.build_stmt(body, span)?;

                self.emit(VynIROC::JumpUncond { label: loop_start }.spanned(span));
                self.emit_label(loop_end);

                self.break_jump_pos = prev_break_jump_pos;
                self.continue_jump_pos = prev_continue_jump_pos;
            }

            Stmt::WhenLoop { body, condition } => {
                let loop_start = self.next_label();
                let loop_end = self.next_label();

                let prev_break_jump_pos = self.break_jump_pos;
                self.break_jump_pos = Some(loop_end);

                let prev_continue_jump_pos = self.continue_jump_pos;
                self.continue_jump_pos = Some(loop_start);

                self.emit_label(loop_start);
                let cond_reg = self.build_expr(condition)?;

                self.emit(
                    VynIROC::JumpIfFalse {
                        condition_reg: cond_reg,
                        label: loop_end,
                    }
                    .spanned(span),
                );

                self.build_stmt(body, span)?;

                self.emit(VynIROC::JumpUncond { label: loop_start }.spanned(span));
                self.emit_label(loop_end);

                self.break_jump_pos = prev_break_jump_pos;
                self.continue_jump_pos = prev_continue_jump_pos;
            }

            Stmt::Break => {
                let jmp_pos = self.break_jump_pos.unwrap();
                self.emit(VynIROC::JumpUncond { label: jmp_pos }.spanned(span));
            }

            Stmt::Continue => {
                let jmp_pos = self.continue_jump_pos.unwrap();
                self.emit(VynIROC::JumpUncond { label: jmp_pos }.spanned(span));
            }

            Stmt::Scope { statements } => {
                self.symbol_table.enter_scope();
                for stmt in statements {
                    self.build_stmt(stmt, stmt.span);
                }
                self.symbol_table.exit_scope();
            }

            Stmt::IfDeclaration {
                condition,
                consequence,
                alternate,
            } => {
                let condition_reg = self.build_expr(condition)?;
                let else_label = self.next_label();
                let if_end_label = self.next_label();

                self.emit(
                    VynIROC::JumpIfFalse {
                        condition_reg,
                        label: else_label,
                    }
                    .spanned(span),
                );

                self.build_stmt(&consequence, span)?;
                if !self.is_terminating_stmt(consequence) {
                    self.emit(
                        VynIROC::JumpUncond {
                            label: if_end_label,
                        }
                        .spanned(span),
                    );
                }

                self.emit_label(else_label);
                if let Some(else_block) = alternate.as_ref() {
                    self.build_stmt(else_block, span)?;
                }
                self.emit_label(if_end_label);
            }

            Stmt::StdoutLog { log_value } => {
                let vreg = self.build_expr(log_value)?;
                self.emit(VynIROC::LogAddr { addr: vreg }.spanned(span));
            }

            unknown => todo!("Implement stmt {:?} at IR", unknown),
        }

        Some(())
    }

    pub(crate) fn build_expr(&mut self, expr: &Expression) -> Option<VReg> {
        let dest = match &expr.node {
            Expr::IntegerLiteral(i) => {
                let dest = self.allocate_vreg();
                self.emit(VynIROC::LoadConstInt { dest, value: *i }.spanned(expr.span));
                dest
            }

            Expr::FloatLiteral(f) => {
                let dest = self.allocate_vreg();
                self.emit(VynIROC::LoadConstFloat { dest, value: *f }.spanned(expr.span));
                dest
            }

            Expr::BooleanLiteral(b) => {
                let dest = self.allocate_vreg();
                self.emit(VynIROC::LoadBool { dest, value: *b }.spanned(expr.span));
                dest
            }

            Expr::StringLiteral(s) => {
                let dest = self.allocate_vreg();
                self.emit(
                    VynIROC::LoadString {
                        dest,
                        value: s.clone(),
                    }
                    .spanned(expr.span),
                );
                dest
            }

            Expr::VariableAssignment {
                identifier,
                new_value,
            } => {
                let new_value_vreg = self.build_expr(&new_value)?;

                let var_name = match &identifier.node {
                    Expr::Identifier(n) => n,
                    _ => unreachable!(),
                };

                let symbol = self.symbol_table.resolve_symbol(
                    var_name,
                    expr.span,
                    &mut self.error_collector,
                )?;

                match symbol.scope {
                    SymbolScope::Register(dest_reg) => {
                        self.emit(
                            VynIROC::Move {
                                dest: dest_reg as VReg,
                                src: new_value_vreg,
                            }
                            .spanned(expr.span),
                        );

                        dest_reg as VReg
                    }
                }
            }

            Expr::Identifier(name) => {
                let symbol =
                    self.symbol_table
                        .resolve_symbol(name, expr.span, &mut self.error_collector)?;

                match symbol.scope {
                    SymbolScope::Register(reg) => reg as VReg,
                }
            }

            Expr::BinaryOperation {
                left,
                operator,
                right,
            } => self.build_binary_expr(left, operator, right, expr)?,

            unknown => todo!("Implement expr {:?} at IR", unknown),
        };

        Some(dest)
    }

    pub(crate) fn allocate_vreg(&mut self) -> VReg {
        let reg = self.next_register;
        self.next_register += 1;
        reg
    }

    fn is_terminating_stmt(&self, stmt: &Statement) -> bool {
        match &stmt.node {
            Stmt::Break | Stmt::Continue => true,
            Stmt::Scope { statements } => statements
                .last()
                .map(|s| self.is_terminating_stmt(s))
                .unwrap_or(false),
            _ => false,
        }
    }

    pub(crate) fn next_label(&mut self) -> Label {
        let label = Label(self.label_counter);
        self.label_counter += 1;
        label
    }

    pub(crate) fn emit_label(&mut self, label: Label) {
        self.instructions
            .push(VynIROC::Label(label).spanned(Span::default()));
    }

    pub(crate) fn emit(&mut self, opcode: VynIROpCode) {
        self.instructions.push(opcode);
    }

    fn finish(&mut self) -> VynIR {
        VynIR {
            instructions: mem::take(&mut self.instructions),
        }
    }
}
