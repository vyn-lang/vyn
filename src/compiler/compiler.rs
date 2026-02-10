use std::mem;

use crate::{
    bytecode::bytecode::OpCode,
    compiler::{debug_info::DebugInfo, register_allocator::RegisterAllocator},
    error_handler::error_collector::ErrorCollector,
    ir::{
        builder::VynIR,
        ir_instr::{VynIROC, VynIROpCode},
    },
    runtime_value::values::RuntimeValue,
    utils::Span,
    vyn_vm::vm::MAX_REGISTERS,
};

/*
 * Collects bytecode information such as instructions, constants, debug info, etc.
 * and exports a bytecode out of it
 *
 * -- Entry method: `.compile_ir()`
 * -- Return value: Result<Bytecode, ErrorCollector>
 * */
pub struct VynCompiler {
    instructions: Vec<u8>,
    constants: Vec<RuntimeValue>,
    debug_info: DebugInfo,
    string_table: Vec<String>,

    register_allocator: RegisterAllocator,
    error_collector: ErrorCollector,
}

/*
 * Compiler's output data
 *
 * Contains:
 * - instructions: Raw bytecode bytes
 * - constants: Constant pool (integers, floats, etc.)
 * - debug_info: Source location mapping for debugging/disassembly
 * */
pub struct Bytecode {
    pub instructions: Vec<u8>,
    pub constants: Vec<RuntimeValue>,
    pub string_table: Vec<String>,
    pub debug_info: DebugInfo,
}

impl VynCompiler {
    /*
     * Creates a new bytecode compiler
     *
     * -- Arguments: [none]
     * -- Return value: VynCompiler instance
     * */
    pub fn new() -> Self {
        Self {
            instructions: Vec::new(),
            constants: Vec::new(),
            string_table: Vec::new(),
            debug_info: DebugInfo::new(),
            register_allocator: RegisterAllocator::new(MAX_REGISTERS),
            error_collector: ErrorCollector::new(),
        }
    }

    /*
     * Loops through every instruction from given IR and compiles to bytecode
     *
     * Performs two-phase compilation:
     * 1. Liveness analysis (via register allocator)
     * 2. Code generation with register allocation
     *
     * -- Arguments: [&mut self], ir - intermediate representation from IR builder
     * -- Return value: Result<Bytecode, ErrorCollector>
     *                  Ok(Bytecode) if compilation succeeds
     *                  Err(ErrorCollector) if compilation fails
     * */
    pub fn compile_ir(&mut self, ir: &VynIR) -> Result<Bytecode, ErrorCollector> {
        self.register_allocator.analyze_liveness(&ir.instructions);

        for (i, inst) in ir.instructions.iter().enumerate() {
            if self.compile_inst(inst, i).is_none() {
                break;
            }
        }

        if self.error_collector.has_errors() {
            Err(mem::take(&mut self.error_collector))
        } else {
            Ok(self.finish())
        }
    }

    /*
     * Converts a single IR instruction to bytecode
     *
     * Handles:
     * - Physical register allocation for virtual registers
     * - Constant pool management
     * - Bytecode emission
     * - Register freeing after use
     *
     * -- Arguments: [&mut self],
     *               inst - IR instruction to compile
     *               inst_idx - instruction index (for liveness analysis)
     * -- Return value: Some(()) if compilation succeeds
     *                  None if compilation fails (errors added to error_collector)
     * */
    pub(crate) fn compile_inst(&mut self, inst: &VynIROpCode, inst_idx: usize) -> Option<()> {
        match &inst.node {
            /*
             * Emits a LoadConstInt OpCode
             * -- Operands: [dest, const_idx]
             * */
            VynIROC::LoadConstInt { dest, value } => {
                let dest = self.allocate(*dest, inst_idx, inst.span)?;
                let const_idx = self.add_constant(RuntimeValue::IntegerLiteral(*value));

                self.emit(
                    OpCode::LoadConstInt,
                    vec![dest as usize, const_idx as usize],
                    inst.span,
                );
            }

            /*
             * Emits a LoadConstFloat OpCode
             * -- Operands: [dest, const_idx]
             * */
            VynIROC::LoadConstFloat { dest, value } => {
                let dest = self.allocate(*dest, inst_idx, inst.span)?;
                let const_idx = self.add_constant(RuntimeValue::FloatLiteral(*value));

                self.emit(
                    OpCode::LoadConstFloat,
                    vec![dest as usize, const_idx as usize],
                    inst.span,
                );
            }

            VynIROC::LoadString { dest, value } => {
                let dest = self.allocate(*dest, inst_idx, inst.span)?;
                let string_idx = self.intern_string(value.clone());
                self.emit(
                    OpCode::LoadString,
                    vec![dest as usize, string_idx],
                    inst.span,
                );
            }

            /*
             * Compiles a binary expression OpCode
             * -- Operands: [dest, left_reg, right_reg]
             * */
            VynIROC::AddInt { dest, left, right }
            | VynIROC::AddFloat { dest, left, right }
            | VynIROC::SubInt { dest, left, right }
            | VynIROC::SubFloat { dest, left, right }
            | VynIROC::MulInt { dest, left, right }
            | VynIROC::MulFloat { dest, left, right }
            | VynIROC::DivInt { dest, left, right }
            | VynIROC::DivFloat { dest, left, right }
            | VynIROC::ExpInt { dest, left, right }
            | VynIROC::ExpFloat { dest, left, right } => {
                // Get physical registers for operands (already allocated)
                let left_reg = self.get(*left, inst.span)?;
                let right_reg = self.get(*right, inst.span)?;

                // Allocate new physical register for destination
                let dest_reg = self.allocate(*dest, inst_idx, inst.span)?;

                // Determine opcode based on instruction type
                let opcode = match &inst.node {
                    VynIROC::AddInt { .. } => OpCode::AddInt,
                    VynIROC::AddFloat { .. } => OpCode::AddFloat,
                    VynIROC::SubInt { .. } => OpCode::SubtractInt,
                    VynIROC::SubFloat { .. } => OpCode::SubtractFloat,
                    VynIROC::MulInt { .. } => OpCode::MultiplyInt,
                    VynIROC::MulFloat { .. } => OpCode::MultiplyFloat,
                    VynIROC::DivInt { .. } => OpCode::DivideInt,
                    VynIROC::DivFloat { .. } => OpCode::DivideFloat,
                    VynIROC::ExpInt { .. } => OpCode::ExponentInt,
                    VynIROC::ExpFloat { .. } => OpCode::ExponentFloat,
                    _ => unreachable!(),
                };

                self.emit(
                    opcode,
                    vec![dest_reg as usize, left_reg as usize, right_reg as usize],
                    inst.span,
                );

                // Free operands if no longer live
                self.free(*left, inst_idx + 1);
                self.free(*right, inst_idx + 1);
            }

            /*
             * Compiles to an stdout printer
             * -- Operands: [addr]
             * */
            VynIROC::LogAddr { addr } => {
                let val = self.get(*addr, inst.span)?;

                self.emit(OpCode::LogAddr, vec![val as usize], inst.span);

                // Free address register after use
                self.free(*addr, inst_idx + 1);
            }

            /*
             * Emits Halt at the end of the instruction
             * -- Operands: []
             * */
            VynIROC::Halt => {
                self.emit(OpCode::Halt, vec![], inst.span);
            }

            unknown => todo!("Implement inst {unknown:?}"),
        }

        Some(())
    }

    /*
     * Allocates a physical register for a virtual register
     *
     * -- Arguments: [&mut self],
     *               virtual_reg - virtual register ID to allocate
     *               inst_idx - current instruction index (for liveness analysis)
     *               span - source location (for error reporting)
     * -- Return value: Some(physical_register) if allocation succeeds
     *                  None if allocation fails (error added to collector)
     * */
    fn allocate(&mut self, virtual_reg: u32, inst_idx: usize, span: Span) -> Option<u8> {
        let result = self
            .register_allocator
            .allocate(virtual_reg, inst_idx, span);

        if let Err(error) = result {
            self.error_collector.add(error);
            return None;
        }

        Some(result.unwrap())
    }

    /*
     * Gets the physical register assigned to a virtual register
     *
     * -- Arguments: [&mut self],
     *               virtual_reg - virtual register ID to look up
     *               _span - source location (unused, kept for consistency)
     * -- Return value: Some(physical_register) if found
     *
     * -- Notes:
     * # Panics if virtual register was never allocated (compiler bug)
     * # This should only be called for registers that were previously allocated
     * */
    fn get(&mut self, virtual_reg: u32, _span: Span) -> Option<u8> {
        match self.register_allocator.get(virtual_reg) {
            Ok(phys_reg) => Some(phys_reg),
            _ => unreachable!("COMPILER BUG: v{} was never allocated", virtual_reg),
        }
    }

    /*
     * Frees a virtual register if it's no longer live
     *
     * Marks the physical register as available for reuse if the virtual
     * register is not needed after the specified instruction.
     *
     * -- Arguments: [&mut self],
     *               virtual_reg - virtual register to potentially free
     *               inst_idx - instruction index to check liveness after
     * -- Return value: void
     * */
    fn free(&mut self, virtual_reg: u32, inst_idx: usize) {
        self.register_allocator.free(virtual_reg, inst_idx);
    }

    /*
     * Emits bytecode instruction with debug info
     *
     * Generates bytecode from opcode and operands, and records source
     * location mapping for debugging/disassembly.
     *
     * -- Arguments: [&mut self],
     *               opcode - VM opcode to emit
     *               operands - operand values (registers, constants, etc.)
     *               span - source location for this instruction
     * -- Return value: usize - bytecode offset where instruction was emitted
     * */
    fn emit(&mut self, opcode: OpCode, operands: Vec<usize>, span: Span) -> usize {
        let pos = self.instructions.len();

        // Record span for instruction start
        self.debug_info.add_span(pos, span);

        let inst = OpCode::make(opcode, operands);
        for data in inst {
            self.instructions.push(data);
            // Record span for each byte
            self.debug_info.add_span(self.instructions.len() - 1, span);
        }

        pos
    }

    /*
     * Adds a constant to the constant pool
     *
     * -- Arguments: [&mut self], value - runtime value to add
     * -- Return value: u16 - index of constant in pool
     *
     * -- Notes:
     * # Returns index as u16 (supports up to 65536 constants)
     * */
    fn add_constant(&mut self, value: RuntimeValue) -> u16 {
        if let Some(idx) = self.constants.iter().position(|c| c == &value) {
            return idx as u16;
        }

        self.constants.push(value);
        (self.constants.len() - 1) as u16
    }

    /*
     * Registers a string to the string_table
     *
     * -- Arguments: [&mut self], value - string to be registered
     *
     * -- Notes:
     * # If the string already exists, it'll return the index
     * # If not, it will register the string AND return the index
     * */
    fn intern_string(&mut self, string: String) -> usize {
        if let Some(index) = self.string_table.iter().position(|s| s == &string) {
            return index;
        }

        self.string_table.push(string);
        self.string_table.len() - 1
    }

    /*
     * Extracts the bytecode information and compiles it to Bytecode
     *
     * -- Arguments: [&mut self]
     * -- Return value: Bytecode
     *
     * -- Notes:
     * # This *TAKES* the value from the compiler itself
     * # Should only be called when the compiler finished compiling
     * */
    pub fn finish(&mut self) -> Bytecode {
        Bytecode {
            instructions: mem::take(&mut self.instructions),
            constants: mem::take(&mut self.constants),
            string_table: mem::take(&mut self.string_table),
            debug_info: mem::take(&mut self.debug_info),
        }
    }
}
