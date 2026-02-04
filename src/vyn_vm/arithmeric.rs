use crate::{
    bytecode::bytecode::{OpCode, read_uint8},
    error_handler::errors::VynError,
    runtime_value::values::RuntimeValue,
    vyn_vm::vm::VynVM,
};

impl VynVM {
    #[inline]
    pub(crate) fn arith_int(&mut self, operator: u8) -> Result<(), VynError> {
        let dest = read_uint8(&self.instructions, self.ip + 1) as usize;
        let left_reg_idx = read_uint8(&self.instructions, self.ip + 2) as usize;
        let right_reg_idx = read_uint8(&self.instructions, self.ip + 3) as usize;
        self.ip += 3;

        let left_reg = self.get_register(left_reg_idx);
        let right_reg = self.get_register(right_reg_idx);

        let l = left_reg.as_int().unwrap();
        let r = right_reg.as_int().unwrap();
        let result: i32 = match operator {
            OpCode::ADD_INT => l + r,
            OpCode::SUBTRACT_INT => l - r,
            OpCode::MULTIPLY_INT => l * r,
            OpCode::DIVIDE_INT => l / r,
            OpCode::EXPONENT_INT => l.pow(r as u32),

            _ => unreachable!("Invalid arith int opcode"),
        };

        self.set_register(dest, RuntimeValue::IntegerLiteral(result));

        Ok(())
    }

    #[inline]
    pub(crate) fn arith_float(&mut self, operator: u8) -> Result<(), VynError> {
        let dest = read_uint8(&self.instructions, self.ip + 1) as usize;
        let left_reg_idx = read_uint8(&self.instructions, self.ip + 2) as usize;
        let right_reg_idx = read_uint8(&self.instructions, self.ip + 3) as usize;
        self.ip += 3;

        let left_reg = self.get_register(left_reg_idx);
        let right_reg = self.get_register(right_reg_idx);

        let l = left_reg.as_float().unwrap();
        let r = right_reg.as_float().unwrap();
        let result = match operator {
            OpCode::ADD_FLOAT => l + r,
            OpCode::SUBTRACT_FLOAT => l - r,
            OpCode::MULTIPLY_FLOAT => l * r,
            OpCode::DIVIDE_FLOAT => l / r,
            OpCode::EXPONENT_FLOAT => l.powf(r),

            _ => unreachable!("Invalid arith float opcode"),
        };

        self.set_register(dest, RuntimeValue::FloatLiteral(result));

        Ok(())
    }

    #[inline]
    pub(crate) fn concat_string(&mut self) -> Result<(), VynError> {
        let dest = read_uint8(&self.instructions, self.ip + 1) as usize;
        let left_reg_idx = read_uint8(&self.instructions, self.ip + 2) as usize;
        let right_reg_idx = read_uint8(&self.instructions, self.ip + 3) as usize;
        self.ip += 3;

        let left_str_idx = self.get_register(left_reg_idx).as_string_index().unwrap();
        let right_str_idx = self.get_register(right_reg_idx).as_string_index().unwrap();

        let l = self.get_string(left_str_idx);
        let r = self.get_string(right_str_idx);

        let mut concat = String::with_capacity(l.len() + r.len());
        concat.push_str(&l);
        concat.push_str(&r);

        let idx = self.intern_string(concat);

        self.set_register(dest, RuntimeValue::StringLiteral(idx));
        Ok(())
    }
}
