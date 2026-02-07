use std::mem;

use crate::{
    ast::ast::{Expr, Expression, Program, Statement, Stmt},
    error_handler::error_collector::ErrorCollector,
    ir::ir_instr::{VReg, VynIROpCode},
    tokens::Token,
    type_checker::{
        static_evaluator::StaticEvaluator, symbol_type_table::SymbolTypeTable, type_checker::Type,
    },
};

pub struct VynIRBuilder<'a> {
    instructions: Vec<VynIROpCode>,
    error_collector: ErrorCollector,
    next_register: VReg,
    static_eval: &'a StaticEvaluator,
    symbol_table: &'a SymbolTypeTable,
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
            self.build_stmt(stmt);
        }

        self.emit(VynIROpCode::Halt);

        if self.error_collector.has_errors() {
            Err(mem::take(&mut self.error_collector))
        } else {
            Ok(self.finish())
        }
    }

    fn build_stmt(&mut self, stmt: &Statement) {
        match &stmt.node {
            Stmt::Expression { expression } => {
                self.build_expr(expression);
            }

            Stmt::StdoutLog { log_value } => {
                let vreg = self.build_expr(log_value);
                self.emit(VynIROpCode::LogAddr { addr: vreg });
            }

            unknown => todo!("Implement stmt {:?} at IR", unknown),
        }
    }

    fn build_expr(&mut self, expr: &Expression) -> VReg {
        match &expr.node {
            Expr::IntegerLiteral(i) => {
                let dest = self.allocate_vreg();
                self.emit(VynIROpCode::LoadConstInt { dest, value: *i });
                dest
            }

            Expr::FloatLiteral(f) => {
                let dest = self.allocate_vreg();
                self.emit(VynIROpCode::LoadConstFloat { dest, value: *f });
                dest
            }

            Expr::BinaryOperation {
                left,
                operator,
                right,
            } => {
                let b_left = self.build_expr(left.as_ref());
                let b_right = self.build_expr(right.as_ref());
                let dest = self.allocate_vreg();

                // Get the type properly
                let expr_type = Type::from_ast(
                    left,
                    self.static_eval,
                    self.symbol_table,
                    &mut self.error_collector,
                );
                let is_op_int = matches!(expr_type, Type::Integer);

                let opcode = match operator {
                    Token::Plus => {
                        if is_op_int {
                            VynIROpCode::AddInt {
                                dest,
                                left: b_left,
                                right: b_right,
                            }
                        } else {
                            VynIROpCode::AddFloat {
                                dest,
                                left: b_left,
                                right: b_right,
                            }
                        }
                    }
                    Token::Minus => {
                        if is_op_int {
                            VynIROpCode::SubInt {
                                dest,
                                left: b_left,
                                right: b_right,
                            }
                        } else {
                            VynIROpCode::SubFloat {
                                dest,
                                left: b_left,
                                right: b_right,
                            }
                        }
                    }
                    Token::Asterisk => {
                        if is_op_int {
                            VynIROpCode::MulInt {
                                dest,
                                left: b_left,
                                right: b_right,
                            }
                        } else {
                            VynIROpCode::MulFloat {
                                dest,
                                left: b_left,
                                right: b_right,
                            }
                        }
                    }
                    Token::Slash => {
                        if is_op_int {
                            VynIROpCode::DivInt {
                                dest,
                                left: b_left,
                                right: b_right,
                            }
                        } else {
                            VynIROpCode::DivFloat {
                                dest,
                                left: b_left,
                                right: b_right,
                            }
                        }
                    }
                    Token::Caret => {
                        if is_op_int {
                            VynIROpCode::ExpInt {
                                dest,
                                left: b_left,
                                right: b_right,
                            }
                        } else {
                            VynIROpCode::ExpFloat {
                                dest,
                                left: b_left,
                                right: b_right,
                            }
                        }
                    }

                    _ => unreachable!(),
                };

                self.emit(opcode);

                dest
            }

            unknown => todo!("Implement expr {:?} at IR", unknown),
        }
    }

    fn allocate_vreg(&mut self) -> VReg {
        let reg = self.next_register;
        self.next_register += 1;
        reg
    }

    fn emit(&mut self, opcode: VynIROpCode) {
        self.instructions.push(opcode);
    }

    fn finish(&mut self) -> VynIR {
        VynIR {
            instructions: mem::take(&mut self.instructions),
        }
    }
}
