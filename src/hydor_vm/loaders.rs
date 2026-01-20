use crate::{
    bytecode::bytecode::{read_uint8, read_uint16},
    errors::HydorError,
    hydor_vm::vm::HydorVM,
    runtime_value::RuntimeValue,
};

impl HydorVM {
    #[inline(always)]
    pub(crate) fn load_constant(&mut self) -> Result<(), HydorError> {
        let dest = read_uint8(&self.instructions, self.ip + 1) as usize;
        let const_idx = read_uint16(&self.instructions, self.ip + 2) as usize;
        self.ip += 3;

        let constant = self.constants[const_idx];
        self.set_register(dest, constant);
        Ok(())
    }

    #[inline(always)]
    pub(crate) fn load_string(&mut self) -> Result<(), HydorError> {
        let dest = read_uint8(&self.instructions, self.ip + 1) as usize;
        let str_idx = read_uint16(&self.instructions, self.ip + 2) as usize;
        self.ip += 3;

        let str = RuntimeValue::StringLiteral(str_idx);
        self.set_register(dest, str);
        Ok(())
    }

    #[inline(always)]
    pub(crate) fn load_static(&mut self, static_val: RuntimeValue) -> Result<(), HydorError> {
        let dest = read_uint8(&self.instructions, self.ip + 1) as usize;
        self.ip += 1;

        self.set_register(dest, static_val);
        Ok(())
    }
}
