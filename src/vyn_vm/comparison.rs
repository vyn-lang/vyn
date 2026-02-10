use crate::{
    bytecode::bytecode::{OpCode, read_uint8},
    error_handler::errors::VynError,
    vyn_vm::vm::VynVM,
};

impl VynVM {
    #[inline]
    pub(crate) fn compare_int(&mut self, opcode: u8) -> Result<(), VynError> {
        let dest = read_uint8(&self.instructions, self.ip + 1) as usize;
        let left_reg_idx = read_uint8(&self.instructions, self.ip + 2) as usize;
        let right_reg_idx = read_uint8(&self.instructions, self.ip + 3) as usize;
        self.ip += 3;

        let left_reg = self.get_register(left_reg_idx);
        let right_reg = self.get_register(right_reg_idx);

        let l = left_reg.as_int().unwrap();
        let r = right_reg.as_int().unwrap();
        let res = match opcode {
            OpCode::LESS_INT => l < r,
            OpCode::LESS_EQUAL_INT => l <= r,
            OpCode::GREATER_INT => l > r,
            OpCode::GREATER_EQUAL_INT => l >= r,

            _ => unreachable!("Invalid compare_int opcode"),
        };

        self.set_register(dest, self.runtime_bool(res));
        Ok(())
    }

    #[inline]
    pub(crate) fn compare_float(&mut self, opcode: u8) -> Result<(), VynError> {
        let dest = read_uint8(&self.instructions, self.ip + 1) as usize;
        let left_reg_idx = read_uint8(&self.instructions, self.ip + 2) as usize;
        let right_reg_idx = read_uint8(&self.instructions, self.ip + 3) as usize;
        self.ip += 3;

        let left_reg = self.get_register(left_reg_idx);
        let right_reg = self.get_register(right_reg_idx);

        let l = left_reg.as_float().unwrap();
        let r = right_reg.as_float().unwrap();
        let res = match opcode {
            OpCode::LESS_FLOAT => l < r,
            OpCode::LESS_EQUAL_FLOAT => l <= r,
            OpCode::GREATER_FLOAT => l > r,
            OpCode::GREATER_EQUAL_FLOAT => l >= r,

            _ => unreachable!("Invalid compare_int opcode"),
        };

        self.set_register(dest, self.runtime_bool(res));
        Ok(())
    }

    #[inline]
    pub(crate) fn compare_equality(&mut self, opcode: u8) -> Result<(), VynError> {
        let dest = read_uint8(&self.instructions, self.ip + 1) as usize;
        let left_reg_idx = read_uint8(&self.instructions, self.ip + 2) as usize;
        let right_reg_idx = read_uint8(&self.instructions, self.ip + 3) as usize;
        self.ip += 3;

        let left_reg = self.get_register(left_reg_idx);
        let right_reg = self.get_register(right_reg_idx);

        let res = match opcode {
            OpCode::EQUAL => left_reg == right_reg,
            OpCode::NOT_EQUAL => left_reg != right_reg,

            _ => unreachable!("Invalid compare_equality opcode"),
        };

        self.set_register(dest, self.runtime_bool(res));
        Ok(())
    }
}
