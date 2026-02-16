use crate::utils::Span;

pub type VReg = u32;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Label(pub usize);

#[derive(Debug, Clone)]
pub struct VynIROpCode {
    pub node: VynIROC,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub enum VynIROC {
    // Load constants
    LoadConstInt { dest: VReg, value: i32 },
    LoadConstFloat { dest: VReg, value: f64 },
    LoadString { dest: VReg, value: String },
    LoadBool { dest: VReg, value: bool },

    // Arithmetic
    AddInt { dest: VReg, left: VReg, right: VReg },
    AddFloat { dest: VReg, left: VReg, right: VReg },
    SubInt { dest: VReg, left: VReg, right: VReg },
    SubFloat { dest: VReg, left: VReg, right: VReg },
    MulInt { dest: VReg, left: VReg, right: VReg },
    MulFloat { dest: VReg, left: VReg, right: VReg },
    DivInt { dest: VReg, left: VReg, right: VReg },
    DivFloat { dest: VReg, left: VReg, right: VReg },
    ExpInt { dest: VReg, left: VReg, right: VReg },
    ExpFloat { dest: VReg, left: VReg, right: VReg },

    // Comparisons
    CompareEqual { dest: VReg, left: VReg, right: VReg },
    CompareNotEqual { dest: VReg, left: VReg, right: VReg },
    CompareLessInt { dest: VReg, left: VReg, right: VReg },
    CompareLessFloat { dest: VReg, left: VReg, right: VReg },
    CompareLessEqualInt { dest: VReg, left: VReg, right: VReg },
    CompareLessEqualFloat { dest: VReg, left: VReg, right: VReg },
    CompareGreaterInt { dest: VReg, left: VReg, right: VReg },
    CompareGreaterFloat { dest: VReg, left: VReg, right: VReg },
    CompareGreaterEqualInt { dest: VReg, left: VReg, right: VReg },
    CompareGreaterEqualFloat { dest: VReg, left: VReg, right: VReg },

    // Control flow
    JumpIfFalse { condition_reg: VReg, label: Label },
    JumpUncond { label: Label },
    Label(Label),

    // Register operations
    Move { dest: VReg, src: VReg }, // For variable assignment

    // I/O
    LogAddr { addr: VReg },

    // Control
    Halt,
}

impl VynIROC {
    pub fn spanned(self, span: Span) -> VynIROpCode {
        VynIROpCode { node: self, span }
    }
}
