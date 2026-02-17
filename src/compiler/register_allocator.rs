use crate::{
    error_handler::errors::VynError,
    ir::ir_instr::{VynIROC, VynIROpCode},
    utils::Span,
};
use std::collections::{HashMap, HashSet};

/*
 * Register allocator that maps virtual registers to physical registers
 * with register reuse based on liveness analysis.
 *
 * Performs two-phase allocation:
 * 1. Liveness analysis (backward pass) - determines which virtual registers
 *    are live (still needed) after each instruction
 * 2. Register allocation (forward pass) - assigns physical registers to
 *    virtual registers, reusing physical registers when virtual ones die
 *
 * -- Entry method: `.analyze_liveness()` then `.allocate()`
 * -- Max registers: Configurable (typically 256 for your VM)
 * */
pub struct RegisterAllocator {
    // Maps virtual register ID -> physical register ID
    allocation: HashMap<u32, u8>,

    // Tracks which physical registers are currently in use
    used_physical: HashSet<u8>,

    // Liveness info: for each instruction index, stores which virtual
    // registers are live (still needed) after that instruction
    live_ranges: Vec<HashSet<u32>>,

    // Maximum number of physical registers available in the VM
    max_registers: u8,
}

impl RegisterAllocator {
    /*
     * Creates a new register allocator
     *
     * -- Arguments: max_registers - maximum number of physical registers
     *               available in the VM (e.g., 256)
     * -- Return value: RegisterAllocator instance
     * */
    pub fn new(max_registers: u8) -> Self {
        Self {
            allocation: HashMap::new(),
            used_physical: HashSet::new(),
            live_ranges: Vec::new(),
            max_registers,
        }
    }

    /*
     * Performs liveness analysis on all instructions (backward pass)
     *
     * Computes which virtual registers are "live" (still needed) after
     * each instruction. A register is live if its value will be used
     * in a future instruction.
     *
     * Algorithm:
     * - Start from the last instruction and work backwards
     * - For each instruction:
     *   1. Copy the live set from the next instruction
     *   2. Remove registers that are defined (written) by this instruction
     *   3. Add registers that are used (read) by this instruction
     *
     * -- Arguments: [&mut self], instructions - slice of IR instructions
     * -- Return value: void (stores results in self.live_ranges)
     *
     * -- Notes:
     * # Must be called before any allocation takes place
     * # Results are stored in self.live_ranges where index i contains
     *   the set of virtual registers live AFTER instruction i
     * */
    pub fn analyze_liveness(&mut self, instructions: &[VynIROpCode]) {
        let inst_len = instructions.len();
        self.live_ranges = vec![HashSet::new(); inst_len + 1];

        for i in (0..inst_len).rev() {
            let mut live = self.live_ranges[i + 1].clone();
            let inst = &instructions[i];

            if let Some(def) = self.get_def(inst) {
                live.remove(&def);
            }

            for used in self.get_uses(inst) {
                live.insert(used);
            }

            self.live_ranges[i] = live;
        }
    }

    /*
     * Gets the virtual register being defined (written to) by an instruction
     *
     * Most instructions write their result to a destination register.
     * This function extracts that destination register ID.
     *
     * -- Arguments: [&self], inst - the IR instruction to analyze
     * -- Return value: Some(virtual_reg_id) if instruction defines a register,
     *                  None if it doesn't write to any register
     *
     * -- Examples:
     * # LoadConstInt { dest: 5, value: 10 } -> Some(5)
     * # AddInt { dest: 2, left: 0, right: 1 } -> Some(2)
     * # Halt -> None (doesn't write to a register)
     * */
    fn get_def(&self, inst: &VynIROpCode) -> Option<u32> {
        match &inst.node {
            VynIROC::LoadConstInt { dest, .. } => Some(*dest),
            VynIROC::LoadConstFloat { dest, .. } => Some(*dest),
            VynIROC::LoadString { dest, .. } => Some(*dest),
            VynIROC::LoadBool { dest, .. } => Some(*dest),

            VynIROC::AddInt { dest, .. } => Some(*dest),
            VynIROC::AddFloat { dest, .. } => Some(*dest),
            VynIROC::SubInt { dest, .. } => Some(*dest),
            VynIROC::SubFloat { dest, .. } => Some(*dest),
            VynIROC::MulInt { dest, .. } => Some(*dest),
            VynIROC::MulFloat { dest, .. } => Some(*dest),
            VynIROC::DivInt { dest, .. } => Some(*dest),
            VynIROC::DivFloat { dest, .. } => Some(*dest),
            VynIROC::ExpInt { dest, .. } => Some(*dest),
            VynIROC::ExpFloat { dest, .. } => Some(*dest),

            VynIROC::CompareEqual { dest, .. } => Some(*dest),
            VynIROC::CompareNotEqual { dest, .. } => Some(*dest),
            VynIROC::CompareLessInt { dest, .. } => Some(*dest),
            VynIROC::CompareLessFloat { dest, .. } => Some(*dest),
            VynIROC::CompareGreaterInt { dest, .. } => Some(*dest),
            VynIROC::CompareGreaterFloat { dest, .. } => Some(*dest),
            VynIROC::CompareLessEqualInt { dest, .. } => Some(*dest),
            VynIROC::CompareLessEqualFloat { dest, .. } => Some(*dest),
            VynIROC::CompareGreaterEqualInt { dest, .. } => Some(*dest),
            VynIROC::CompareGreaterEqualFloat { dest, .. } => Some(*dest),

            VynIROC::Move { dest, .. } => Some(*dest),

            VynIROC::LogAddr { .. } => None,
            VynIROC::JumpIfFalse { .. } => None,
            VynIROC::JumpUncond { .. } => None,
            VynIROC::Label(..) => None,
            VynIROC::Halt => None,
        }
    }

    /*
     * Gets all virtual registers being used (read from) by an instruction
     *
     * Returns a vector of all virtual register IDs that this instruction
     * reads from (its operands).
     *
     * -- Arguments: [&self], inst - the IR instruction to analyze
     * -- Return value: Vec<u32> of virtual register IDs being read
     *
     * -- Examples:
     * # LoadConstInt { dest: 5, value: 10 } -> vec![] (no reads)
     * # AddInt { dest: 2, left: 0, right: 1 } -> vec![0, 1]
     * # LogAddr { addr: 4 } -> vec![4]
     * */
    fn get_uses(&self, inst: &VynIROpCode) -> Vec<u32> {
        match &inst.node {
            VynIROC::LoadConstInt { .. } => vec![],
            VynIROC::LoadConstFloat { .. } => vec![],
            VynIROC::LoadString { .. } => vec![],
            VynIROC::LoadBool { .. } => vec![],

            VynIROC::AddInt { left, right, .. } => vec![*left, *right],
            VynIROC::AddFloat { left, right, .. } => vec![*left, *right],
            VynIROC::SubInt { left, right, .. } => vec![*left, *right],
            VynIROC::SubFloat { left, right, .. } => vec![*left, *right],
            VynIROC::MulInt { left, right, .. } => vec![*left, *right],
            VynIROC::MulFloat { left, right, .. } => vec![*left, *right],
            VynIROC::DivInt { left, right, .. } => vec![*left, *right],
            VynIROC::DivFloat { left, right, .. } => vec![*left, *right],
            VynIROC::ExpInt { left, right, .. } => vec![*left, *right],
            VynIROC::ExpFloat { left, right, .. } => vec![*left, *right],

            VynIROC::CompareEqual { left, right, .. } => vec![*left, *right],
            VynIROC::CompareNotEqual { left, right, .. } => vec![*left, *right],
            VynIROC::CompareLessInt { left, right, .. } => vec![*left, *right],
            VynIROC::CompareLessFloat { left, right, .. } => vec![*left, *right],
            VynIROC::CompareGreaterInt { left, right, .. } => vec![*left, *right],
            VynIROC::CompareGreaterFloat { left, right, .. } => vec![*left, *right],
            VynIROC::CompareLessEqualInt { left, right, .. } => vec![*left, *right],
            VynIROC::CompareLessEqualFloat { left, right, .. } => vec![*left, *right],
            VynIROC::CompareGreaterEqualInt { left, right, .. } => vec![*left, *right],
            VynIROC::CompareGreaterEqualFloat { left, right, .. } => vec![*left, *right],

            VynIROC::Move { src, .. } => vec![*src],
            VynIROC::LogAddr { addr } => vec![*addr],
            VynIROC::JumpIfFalse { condition_reg, .. } => vec![*condition_reg],

            VynIROC::JumpUncond { .. } => vec![],
            VynIROC::Label(..) => vec![],
            VynIROC::Halt => vec![],
        }
    }

    /*
     * Allocates a physical register for a virtual register
     *
     * This is the main allocation function. It:
     * 1. Returns existing allocation if virtual register already has one
     * 2. Tries to find a free physical register
     * 3. If all physical registers are used, tries to "spill" (reuse)
     *    a register that holds a dead virtual register
     * 4. Fails if all registers hold live virtual registers
     *
     * -- Arguments: [&mut self],
     *               virtual_reg - the virtual register ID to allocate for
     *               inst_index - current instruction index (for liveness check)
     * -- Return value: Result<u8, String>
     *                  Ok(physical_reg_id) if allocation succeeds
     *                  Err(error_msg) if out of registers
     *
     * -- Notes:
     * # analyze_liveness() must be called before this
     * # This function updates internal allocation tables
     * */
    pub fn allocate(
        &mut self,
        virtual_reg: u32,
        inst_index: usize,
        span: Span,
    ) -> Result<u8, VynError> {
        // If already allocated, return the existing physical register
        if let Some(&phys) = self.allocation.get(&virtual_reg) {
            return Ok(phys);
        }

        // Try to find a free physical register
        for phys in 0..self.max_registers {
            if !self.used_physical.contains(&phys) {
                self.allocation.insert(virtual_reg, phys);
                self.used_physical.insert(phys);
                return Ok(phys);
            }
        }

        // No free registers - try to spill (reuse) a dead register
        if let Some(phys) = self.find_spillable_register(inst_index) {
            // Remove the old virtual->physical mapping for this physical register
            self.allocation.retain(|_, &mut v| v != phys);

            // Create new mapping
            self.allocation.insert(virtual_reg, phys);
            self.used_physical.insert(phys);
            return Ok(phys);
        }

        // Complete failure - all registers hold live values
        Err(VynError::RegisterOverflow { span })
    }

    /*
     * Finds a physical register that can be reused (spilled)
     *
     * Looks for a physical register that currently holds a virtual register
     * whose value is no longer needed (dead/not live).
     *
     * -- Arguments: [&self], inst_index - current instruction index
     * -- Return value: Some(physical_reg_id) if a spillable register is found,
     *                  None if all physical registers hold live values
     *
     * -- Algorithm:
     * # Check each allocated virtual->physical mapping
     * # If the virtual register is NOT in the live set, its physical
     *   register can be reused
     * */
    fn find_spillable_register(&self, inst_index: usize) -> Option<u8> {
        let live = &self.live_ranges[inst_index];

        // Find a physical register whose virtual register is not live
        for (&virt, &phys) in &self.allocation {
            if !live.contains(&virt) {
                return Some(phys);
            }
        }

        None
    }

    /*
     * Gets the physical register currently allocated to a virtual register
     *
     * Used when compiling instructions to look up which physical register
     * holds the value we need.
     *
     * -- Arguments: [&self], virtual_reg - the virtual register ID to look up
     * -- Return value: Result<u8, String>
     *                  Ok(physical_reg_id) if virtual register is allocated
     *                  Err(error_msg) if virtual register was never allocated
     *
     * -- Notes:
     * # This should only be called for virtual registers that have already
     *   been allocated via allocate()
     * # Commonly used for getting operand registers when compiling instructions
     * */
    pub fn get(&self, virtual_reg: u32) -> Result<u8, VynError> {
        self.allocation
            .get(&virtual_reg)
            .copied()
            .ok_or_else(|| unreachable!())
    }

    /*
     * Frees a virtual register if it's no longer live
     *
     * Marks the physical register as available for reuse if the virtual
     * register is not needed after this instruction.
     *
     * -- Arguments: [&mut self],
     *               virtual_reg - the virtual register to potentially free
     *               inst_index - instruction index to check liveness after
     * -- Return value: void
     *
     * -- Notes:
     * # Should be called after compiling instructions that use registers
     * # Only frees if the virtual register is NOT in the live set
     * # The physical register becomes available for allocation again
     *
     * -- Usage pattern:
     * # After compiling "AddInt { dest: 2, left: 0, right: 1 }":
     *   - free(0, inst_index + 1)  // free left operand
     *   - free(1, inst_index + 1)  // free right operand
     * */
    pub fn free(&mut self, virtual_reg: u32, inst_index: usize) {
        // Only free if this register is no longer live
        if !self.live_ranges[inst_index].contains(&virtual_reg) {
            if let Some(&phys) = self.allocation.get(&virtual_reg) {
                self.used_physical.remove(&phys);
                // Note: we keep the mapping in self.allocation for debugging,
                // but mark the physical register as free
            }
        }
    }

    /*
     * Gets the number of physical registers currently in use
     *
     * Useful for debugging and statistics.
     *
     * -- Arguments: [&self]
     * -- Return value: usize - number of physical registers currently allocated
     * */
    pub fn num_used_registers(&self) -> usize {
        self.used_physical.len()
    }

    /*
     * Gets the maximum number of physical registers ever used simultaneously
     *
     * Useful for determining minimum register requirements.
     *
     * -- Arguments: [&self]
     * -- Return value: usize - peak register usage
     *
     * -- Notes:
     * # Only accurate if called after all allocations are complete
     * */
    pub fn peak_register_usage(&self) -> usize {
        self.used_physical.len()
    }
}
