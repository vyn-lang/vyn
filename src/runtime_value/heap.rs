use crate::runtime_value::values::RuntimeValue;

#[derive(Debug)]
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
