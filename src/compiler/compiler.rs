use std::mem;

use crate::{
    error_handler::error_collector::ErrorCollector,
    ir::{builder::VynIR, ir_instr::VynIROpCode},
};

/*
 * Collects bytecode information such as instructions, constants, etc.
 * and exports a bytecode out of it
 *
 * -- Entry method: `.compile_ir()`
 * -- Return value: Result<Bytecode, ErrorCollector>,
 * */
pub struct VynCompiler {
    instructions: Vec<u8>,
    error_collector: ErrorCollector,
}

/// Compiler's output data
pub struct Bytecode {
    instructions: Vec<u8>,
}

impl VynCompiler {
    pub fn new() -> Self {
        Self {
            instructions: Vec::new(),
            error_collector: ErrorCollector::new(),
        }
    }

    /*
     * Loops through every instructions from given IR
     * and returns a Bytecode
     *
     * -- Arguments: [&mut self], ir from ir builder
     * -- Return value; Result<Bytecode, ErrorCollector>
     * */
    pub fn compile_ir(&mut self, ir: &VynIR) -> Result<Bytecode, ErrorCollector> {
        for inst in &ir.instructions {
            self.compile_inst(inst);
        }
        if self.error_collector.has_errors() {
            Err(mem::take(&mut self.error_collector))
        } else {
            Ok(self.finish())
        }
    }

    /*
     * Converts the IR OpCode to a valid Bytecode
     * instruction
     *
     * -- Arguments: [&mut self], IR OpCode
     * -- Return value: void
     * */
    pub(crate) fn compile_inst(&mut self, inst: &VynIROpCode) {
        match &inst {
            VynIROpCode::LoadConstInt { dest, value } => {}
            unknown => todo!("Implement inst {unknown:?}"),
        }
    }

    /*
     * Extracts the bytecode information and compiles it
     * to Bytecode
     *
     * -- Arguments: [&mut self]
     * -- Result value: Bytecode
     *
     * -- Notes:
     * # This *TAKES* the value from the compiler itself.
     * This should only be called when the compiler finished
     * compiling
     * */
    pub fn finish(&mut self) -> Bytecode {
        Bytecode {
            instructions: mem::take(&mut self.instructions),
        }
    }
}
