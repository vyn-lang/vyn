use crate::utils::{Span, Spanned};

pub type VReg = u32;

pub type VynIROpCode = Spanned<VynIROC>;

#[derive(Debug, Clone)]
pub enum VynIROC {
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

    Halt,
}

impl VynIROC {
    pub fn spanned(self, span: Span) -> Spanned<Self> {
        Spanned { node: self, span }
    }
}
