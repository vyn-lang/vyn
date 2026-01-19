// use std::fs::File;
// use std::io::{self, Read, Write};
// use std::path::Path;
//
// use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};
// use num_enum::{IntoPrimitive, TryFromPrimitive};
//
// use crate::compiler::compiler::{Bytecode, DebugInfo};
// use crate::runtime_value::RuntimeValue;
//
// const MAGIC_NUMBER: u32 = 0x48594452; // "HYDR" in hex
// const VERSION: u32 = 0x1;
//
// /// Type tags for serializing RuntimeValue variants
// #[derive(Debug, Clone, Copy, PartialEq, Eq, IntoPrimitive, TryFromPrimitive)]
// #[repr(u8)]
// enum ConstantType {
//     Integer = 0,
//     Float = 1,
//     Boolean = 2,
//     String = 3,
// }
//
// impl Bytecode {
//     pub fn save_to_file<P: AsRef<Path>>(&self, path: P) -> io::Result<()> {
//         let mut file = File::create(path)?;
//
//         // Write magic number
//         file.write_u32::<BigEndian>(MAGIC_NUMBER)?;
//
//         // Write version
//         file.write_u32::<BigEndian>(VERSION)?;
//
//         // Write global count (NEW)
//         file.write_u32::<BigEndian>(self.global_count as u32)?;
//
//         // Write instructions length + data
//         file.write_u32::<BigEndian>(self.instructions.len() as u32)?;
//         file.write_all(&self.instructions)?;
//
//         // Write string table
//         file.write_u32::<BigEndian>(self.string_table.len() as u32)?;
//         for string in &self.string_table {
//             file.write_u32::<BigEndian>(string.len() as u32)?;
//             file.write_all(string.as_bytes())?;
//         }
//
//         // Write constants count
//         file.write_u32::<BigEndian>(self.constants.len() as u32)?;
//
//         // Write each constant
//         for constant in &self.constants {
//             self.write_constant(&mut file, constant)?;
//         }
//
//         // Write debug info
//         self.write_debug_info(&mut file)?;
//
//         Ok(())
//     }
//
//     /// Load bytecode from a .hydc file
//     pub fn load_from_file<P: AsRef<Path>>(path: P) -> io::Result<Self> {
//         let mut file = File::open(path)?;
//
//         // Read and verify magic number
//         let magic = file.read_u32::<BigEndian>()?;
//         if magic != MAGIC_NUMBER {
//             return Err(io::Error::new(
//                 io::ErrorKind::InvalidData,
//                 format!(
//                     "Invalid magic number: expected {:#x}, got {:#x}",
//                     MAGIC_NUMBER, magic
//                 ),
//             ));
//         }
//
//         // Read and verify version
//         let file_version = file.read_u32::<BigEndian>()?;
//         if file_version != VERSION {
//             return Err(io::Error::new(
//                 io::ErrorKind::InvalidData,
//                 format!(
//                     "Version mismatch: expected {:#x}, got {:#x}",
//                     VERSION, file_version
//                 ),
//             ));
//         }
//
//         // Read global count
//         let global_count = file.read_u32::<BigEndian>()? as usize;
//
//         // Read instructions
//         let instructions_len = file.read_u32::<BigEndian>()? as usize;
//         let mut instructions = vec![0u8; instructions_len];
//         file.read_exact(&mut instructions)?;
//
//         // Read string table
//         let string_table_len = file.read_u32::<BigEndian>()? as usize;
//         let mut string_table = Vec::with_capacity(string_table_len);
//         for _ in 0..string_table_len {
//             let str_len = file.read_u32::<BigEndian>()? as usize;
//             let mut str_buf = vec![0u8; str_len];
//             file.read_exact(&mut str_buf)?;
//             let string = String::from_utf8(str_buf)
//                 .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
//             string_table.push(string);
//         }
//
//         // Read constants
//         let constants_count = file.read_u32::<BigEndian>()? as usize;
//         let mut constants = Vec::with_capacity(constants_count);
//         for _ in 0..constants_count {
//             constants.push(Self::read_constant(&mut file)?);
//         }
//
//         // Read debug info
//         let debug_info = Self::read_debug_info(&mut file)?;
//
//         Ok(Bytecode {
//             instructions,
//             constants,
//             string_table,
//             debug_info,
//             global_count,
//         })
//     }
//
//     fn write_constant(&self, file: &mut File, constant: &RuntimeValue) -> io::Result<()> {
//         match constant {
//             RuntimeValue::IntegerLiteral(v) => {
//                 file.write_u8(ConstantType::Integer.into())?;
//                 file.write_i32::<BigEndian>(*v)?;
//             }
//             RuntimeValue::FloatLiteral(v) => {
//                 file.write_u8(ConstantType::Float.into())?;
//                 file.write_f64::<BigEndian>(*v)?;
//             }
//
//             _ => unreachable!(),
//         }
//         Ok(())
//     }
//
//     fn read_constant(file: &mut File) -> io::Result<RuntimeValue> {
//         let type_tag = file.read_u8()?;
//         let constant_type = ConstantType::try_from(type_tag)
//             .map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "Unknown constant type"))?;
//
//         match constant_type {
//             ConstantType::Integer => {
//                 let value = file.read_i32::<BigEndian>()?;
//                 Ok(RuntimeValue::IntegerLiteral(value))
//             }
//             ConstantType::Float => {
//                 let value = file.read_f64::<BigEndian>()?;
//                 Ok(RuntimeValue::FloatLiteral(value))
//             }
//             ConstantType::Boolean => {
//                 let value = file.read_u8()?;
//                 Ok(RuntimeValue::BooleanLiteral(value != 0))
//             }
//             ConstantType::String => {
//                 let idx = file.read_u32::<BigEndian>()? as usize;
//                 Ok(RuntimeValue::StringLiteral(idx))
//             }
//         }
//     }
//
//     fn write_debug_info(&self, file: &mut File) -> io::Result<()> {
//         // Write line_changes
//         file.write_u32::<BigEndian>(self.debug_info.line_changes.len() as u32)?;
//         for (offset, line) in &self.debug_info.line_changes {
//             file.write_u32::<BigEndian>(*offset as u32)?;
//             file.write_u32::<BigEndian>(*line)?;
//         }
//
//         // Write start_col_changes
//         file.write_u32::<BigEndian>(self.debug_info.start_col_changes.len() as u32)?;
//         for (offset, col) in &self.debug_info.start_col_changes {
//             file.write_u32::<BigEndian>(*offset as u32)?;
//             file.write_u32::<BigEndian>(*col)?;
//         }
//
//         // Write end_col_changes
//         file.write_u32::<BigEndian>(self.debug_info.end_col_changes.len() as u32)?;
//         for (offset, col) in &self.debug_info.end_col_changes {
//             file.write_u32::<BigEndian>(*offset as u32)?;
//             file.write_u32::<BigEndian>(*col)?;
//         }
//
//         Ok(())
//     }
//
//     fn read_debug_info(file: &mut File) -> io::Result<DebugInfo> {
//         // Read line_changes
//         let line_changes_len = file.read_u32::<BigEndian>()? as usize;
//         let mut line_changes = Vec::with_capacity(line_changes_len);
//         for _ in 0..line_changes_len {
//             let offset = file.read_u32::<BigEndian>()? as usize;
//             let line = file.read_u32::<BigEndian>()?;
//             line_changes.push((offset, line));
//         }
//
//         // Read start_col_changes
//         let start_col_len = file.read_u32::<BigEndian>()? as usize;
//         let mut start_col_changes = Vec::with_capacity(start_col_len);
//         for _ in 0..start_col_len {
//             let offset = file.read_u32::<BigEndian>()? as usize;
//             let col = file.read_u32::<BigEndian>()?;
//             start_col_changes.push((offset, col));
//         }
//
//         // Read end_col_changes
//         let end_col_len = file.read_u32::<BigEndian>()? as usize;
//         let mut end_col_changes = Vec::with_capacity(end_col_len);
//         for _ in 0..end_col_len {
//             let offset = file.read_u32::<BigEndian>()? as usize;
//             let col = file.read_u32::<BigEndian>()?;
//             end_col_changes.push((offset, col));
//         }
//
//         Ok(DebugInfo {
//             line_changes,
//             start_col_changes,
//             end_col_changes,
//         })
//     }
// }
