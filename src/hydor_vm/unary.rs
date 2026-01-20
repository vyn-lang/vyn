use crate::{
    bytecode::bytecode::read_uint8, errors::HydorError, hydor_vm::vm::HydorVM,
    runtime_value::RuntimeValue,
};

impl HydorVM {
    #[inline]
    pub(crate) fn negate_int(&mut self) -> Result<(), HydorError> {
        let dest = read_uint8(&self.instructions, self.ip + 1) as usize;
        let src = read_uint8(&self.instructions, self.ip + 2) as usize;
        self.ip += 2;

        let value = self.get_register(src).as_int().unwrap();
        self.set_register(dest, RuntimeValue::IntegerLiteral(-value));

        Ok(())
    }

    #[inline]
    pub(crate) fn negate_float(&mut self) -> Result<(), HydorError> {
        let dest = read_uint8(&self.instructions, self.ip + 1) as usize;
        let src = read_uint8(&self.instructions, self.ip + 2) as usize;
        self.ip += 2;

        let value = self.get_register(src).as_float().unwrap();
        self.set_register(dest, RuntimeValue::FloatLiteral(-value));

        Ok(())
    }
}
