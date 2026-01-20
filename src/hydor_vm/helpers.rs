use crate::{
    hydor_vm::vm::{FALSE, HydorVM, TRUE},
    runtime_value::RuntimeValue,
};

impl HydorVM {
    pub(crate) fn is_truthy(&self, val: RuntimeValue) -> bool {
        match val {
            RuntimeValue::IntegerLiteral(n) => n != 0,
            RuntimeValue::FloatLiteral(n) => n != 0.0,
            RuntimeValue::StringLiteral(i) => {
                let string = self.get_string(i);
                string != ""
            }
            RuntimeValue::BooleanLiteral(b) => b,
            RuntimeValue::NilLiteral => false,

            _ => true,
        }
    }

    pub(crate) fn runtime_bool(&self, b: bool) -> RuntimeValue {
        if b { TRUE } else { FALSE }
    }
}
