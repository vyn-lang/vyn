use crate::utils::{Span, Spanned};

pub type VReg = u32;

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
pub struct Label(pub usize);

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

    // ===== Comparison - Int =====
    CompareLessInt { dest: VReg, left: VReg, right: VReg },
    CompareGreaterInt { dest: VReg, left: VReg, right: VReg },
    CompareLessEqualInt { dest: VReg, left: VReg, right: VReg },
    CompareGreaterEqualInt { dest: VReg, left: VReg, right: VReg },

    // ===== Comparison - Float =====
    CompareLessFloat { dest: VReg, left: VReg, right: VReg },
    CompareGreaterFloat { dest: VReg, left: VReg, right: VReg },
    CompareLessEqualFloat { dest: VReg, left: VReg, right: VReg },
    CompareGreaterEqualFloat { dest: VReg, left: VReg, right: VReg },

    // ===== Comparison - General =====
    CompareNotEqual { dest: VReg, left: VReg, right: VReg },
    CompareEqual { dest: VReg, left: VReg, right: VReg },

    // ===== Load Constants =====
    LoadConstInt { dest: VReg, value: i32 },
    LoadConstFloat { dest: VReg, value: f64 },
    LoadString { dest: VReg, value: String },
    LoadBool { dest: VReg, value: bool },
    LoadNil { dest: VReg },

    // ===== Globals Interaction ====
    LoadGlobal { dest: VReg, global_idx: usize },
    StoreGlobal { value_reg: VReg },

    // ===== Jumps =====
    JumpIfFalse { condition_reg: VReg, label: Label },
    JumpUncond { label: Label },

    // ===== Misc =====
    Label(Label),
    LogAddr { addr: VReg },

    Halt,
}

impl VynIROC {
    pub fn spanned(self, span: Span) -> Spanned<Self> {
        Spanned { node: self, span }
    }
}
