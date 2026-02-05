// use crate::{bytecode::bytecode::read_uint8, error_handler::errors::VynError, vyn_vm::vm::VynVM};
//
// impl VynVM {
//     #[inline]
//     pub(crate) fn bool_not(&mut self) -> Result<(), VynError> {
//         let dest = read_uint8(&self.instructions, self.ip + 1) as usize;
//         let src = read_uint8(&self.instructions, self.ip + 2) as usize;
//         self.ip += 2;
//
//         let src_reg = self.get_register(src);
//         self.set_register(dest, self.runtime_bool(self.is_truthy(src_reg)));
//         Ok(())
//     }
// }
