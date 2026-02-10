use std::mem;

use crate::{
    ast::ast::{Expr, Expression, Program, Statement, Stmt},
    error_handler::error_collector::ErrorCollector,
    ir::ir_instr::{VReg, VynIROC, VynIROpCode},
    type_checker::{
        static_evaluator::StaticEvaluator, symbol_type_table::SymbolTypeTable, type_checker::Type,
    },
    utils::Span,
};

pub struct VynIRBuilder<'a> {
    instructions: Vec<VynIROpCode>,
    next_register: VReg,
    pub(crate) error_collector: ErrorCollector,
    pub(crate) static_eval: &'a StaticEvaluator,
    pub(crate) symbol_table: &'a SymbolTypeTable,
}

pub struct VynIR {
    pub instructions: Vec<VynIROpCode>,
}

impl<'a> VynIRBuilder<'a> {
    pub fn new(static_eval: &'a StaticEvaluator, symbol_table: &'a SymbolTypeTable) -> Self {
        Self {
            instructions: Vec::new(),
            error_collector: ErrorCollector::new(),
            next_register: 0,
            static_eval,
            symbol_table,
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

    fn build_stmt(&mut self, stmt: &Statement, span: Span) {
        match &stmt.node {
            Stmt::Expression { expression } => {
                self.build_expr(expression);
            }

            Stmt::StdoutLog { log_value } => {
                let vreg = self.build_expr(log_value);
                self.emit(VynIROC::LogAddr { addr: vreg }.spanned(span));
            }

            unknown => todo!("Implement stmt {:?} at IR", unknown),
        }
    }

    pub(crate) fn build_expr(&mut self, expr: &Expression) -> VReg {
        match &expr.node {
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

            Expr::BinaryOperation {
                left,
                operator,
                right,
            } => self.build_binary_expr(left, operator, right, expr),

            unknown => todo!("Implement expr {:?} at IR", unknown),
        }
    }

    pub(crate) fn allocate_vreg(&mut self) -> VReg {
        let reg = self.next_register;
        self.next_register += 1;
        reg
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
