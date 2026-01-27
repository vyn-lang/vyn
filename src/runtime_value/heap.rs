use crate::runtime_value::values::RuntimeValue;

pub enum HeapObject {
    String(String),
    DynamicArray {
        elements: Vec<RuntimeValue>,
    },
    FixedArray {
        elements: Vec<RuntimeValue>,
        size: usize,
    },
}
