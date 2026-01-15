use crate::{
    bytecode::bytecode::{Instructions, OpCode, ToOpcode, read_uint16},
    compiler::compiler::Bytecode,
    runtime_value::RuntimeValue,
};

pub fn disassemble(bytecode: &Bytecode) {
    println!("--== Hydor Assembly ==--");
    disassemble_instructions(&bytecode.instructions);
    println!();
    disassemble_constants(&bytecode.constants);
}

fn disassemble_instructions(instructions: &Instructions) {
    let mut offset = 0;

    while offset < instructions.len() {
        let opcode_byte = instructions[offset];
        let opcode = opcode_byte.to_opcode();
        let definition = OpCode::get_definition(opcode);

        print!("{:04} {} ({:#04x})", offset, definition.name, opcode_byte);

        offset += 1; // Move past opcode

        // Read operands
        if !definition.operands_width.is_empty() {
            print!(" [");

            for (i, &width) in definition.operands_width.iter().enumerate() {
                if i > 0 {
                    print!(", ");
                }

                match width {
                    2 => {
                        let operand = read_uint16(instructions, offset);
                        print!("{:#04x}", operand);
                        offset += 2;
                    }
                    _ => unreachable!(),
                }
            }

            print!("]");
        }

        println!();
    }
}

fn disassemble_constants(constants: &Vec<RuntimeValue>) {
    println!("--== Constants ==--");

    for (i, constant) in constants.iter().enumerate() {
        println!("{:#04x} {:?}", i, constant);
    }
}
