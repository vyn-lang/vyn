use crate::runtime_value::values::RuntimeValue;

pub enum HeapObject {
    String(String),
    FixedArray {
        elements: Vec<RuntimeValue>,
        size: usize,
    },
}
