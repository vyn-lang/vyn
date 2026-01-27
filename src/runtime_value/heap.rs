use crate::runtime_value::values::RuntimeValue;

pub enum HeapObject {
    String(String),
    Sequence {
        elements: Vec<RuntimeValue>,
    },
    Array {
        elements: Vec<RuntimeValue>,
        size: usize,
    },
}
