pub type VReg = u32;

#[derive(Debug, Clone)]
pub enum VynIROpCode {
    // ===== Arithmetic - Integer =====
    AddInt { dest: VReg, left: VReg, right: VReg },
    SubInt { dest: VReg, left: VReg, right: VReg },
    MulInt { dest: VReg, left: VReg, right: VReg },
    DivInt { dest: VReg, left: VReg, right: VReg },
    ExpInt { dest: VReg, left: VReg, right: VReg },

    // ===== Arithmetic - Float =====
    AddFloat { dest: VReg, left: VReg, right: VReg },
    SubFloat { dest: VReg, left: VReg, right: VReg },
    MulFloat { dest: VReg, left: VReg, right: VReg },
    DivFloat { dest: VReg, left: VReg, right: VReg },
    ExpFloat { dest: VReg, left: VReg, right: VReg },

    // ===== Load Constants =====
    LoadConstInt { dest: VReg, value: i32 },
    LoadConstFloat { dest: VReg, value: f64 },
    LoadBoolTrue { dest: VReg },
    LoadBoolFalse { dest: VReg },
    LoadNil { dest: VReg },

    // Logging
    LogAddr { addr: VReg },
}
