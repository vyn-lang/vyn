use std::mem;

use crate::{
    ast::{Expr, Expression, Node, Program, Statement, Stmt},
    bytecode::bytecode::{Instructions, OpCode},
    errors::{ErrorCollector, HydorError},
    runtime_value::RuntimeValue,
    utils::Spanned,
};

pub struct Compiler {
    instructions: Instructions,
    constants: Vec<RuntimeValue>,

    errors: ErrorCollector,
}

pub struct Bytecode {
    pub instructions: Instructions,
    pub constants: Vec<RuntimeValue>,
}

impl Compiler {
    pub fn new() -> Self {
        Self {
            instructions: Vec::new(),
            constants: Vec::new(),

            errors: ErrorCollector::new(),
        }
    }

    /// Main entry point
    pub fn compile_program(&mut self, program: Program) -> Result<Bytecode, ErrorCollector> {
        for stmt in program.statements {
            match self.try_compile_node(stmt.as_node()) {
                None => break,
                Some(()) => (),
            };
        }

        self.emit(OpCode::Halt, vec![]);

        if self.errors.has_errors() {
            Err(mem::take(&mut self.errors))
        } else {
            Ok(self.bytecode())
        }
    }

    fn try_compile_node(&mut self, node: Node) -> Option<()> {
        match node {
            Node::Statement(stmt) => self.try_compile_statement(stmt)?,
            Node::Expression(expr) => self.try_compile_expression(expr)?,
        }

        Some(()) // Success
    }

    fn try_compile_statement(&mut self, stmt: Statement) -> Option<()> {
        let span = stmt.span;

        match stmt.node {
            Stmt::Expression { expression } => self.try_compile_expression(expression)?,
            unknown => {
                self.throw_error(HydorError::UnknownAST {
                    node: Node::Statement(Spanned {
                        node: unknown,
                        span,
                    }),
                    span,
                });
                return None;
            }
        }

        Some(()) // Success
    }

    fn try_compile_expression(&mut self, expr: Expression) -> Option<()> {
        let span = expr.span;

        match expr.node {
            Expr::IntegerLiteral(v) => {
                let value = RuntimeValue::IntegerLiteral(v);
                let constant_index = self.add_constant(value);
                self.emit(OpCode::LoadConstant, vec![constant_index]);
            }

            Expr::FloatLiteral(v) => {
                let value = RuntimeValue::FloatLiteral(v);
                let constant_index = self.add_constant(value);
                self.emit(OpCode::LoadConstant, vec![constant_index]);
            }

            Expr::BooleanLiteral(v) => {
                let value = RuntimeValue::BooleanLiteral(v);
                let constant_index = self.add_constant(value);
                self.emit(OpCode::LoadConstant, vec![constant_index]);
            }

            Expr::StringLiteral(v) => {
                let value = RuntimeValue::StringLiteral(v);
                let constant_index = self.add_constant(value);
                self.emit(OpCode::LoadConstant, vec![constant_index]);
            }

            unknown => {
                self.throw_error(HydorError::UnknownAST {
                    node: Node::Expression(Spanned {
                        node: unknown,
                        span,
                    }),
                    span,
                });
                return None;
            }
        }

        Some(()) // Success
    }

    fn bytecode(&mut self) -> Bytecode {
        Bytecode {
            instructions: mem::take(&mut self.instructions),
            constants: mem::take(&mut self.constants),
        }
    }

    fn emit(&mut self, opcode: OpCode, operands: Vec<usize>) -> usize {
        let instruction = OpCode::make(opcode, operands);
        let position = self.add_instruction(instruction);

        position
    }

    fn add_constant(&mut self, value: RuntimeValue) -> usize {
        self.constants.push(value);
        self.constants.len() - 1 // Returns the constant's "address"
    }

    fn throw_error(&mut self, error: HydorError) {
        self.errors.add(error);
    }

    fn add_instruction(&mut self, instruction: Instructions) -> usize {
        let position = self.instructions.len();

        for byte in instruction {
            self.instructions.push(byte);
        }

        position
    }
}
