use colored::*;

use crate::{
    bytecode::bytecode::{Instructions, OpCode, ToOpcode, read_uint8, read_uint16},
    compiler::{compiler::Bytecode, debug_info::DebugInfo},
    runtime_value::RuntimeValue,
};

pub fn disassemble(bytecode: &Bytecode) {
    println!("{}", "--== Vyn Assembly ==--".bright_yellow().bold());
    disassemble_instructions(&bytecode.instructions, &bytecode.debug_info);
    println!();
    disassemble_constants(&bytecode.constants);
    println!();
    disassemble_string_table(&bytecode.string_table);
}

fn disassemble_instructions(instructions: &Instructions, debug_info: &DebugInfo) {
    let mut offset = 0;

    while offset < instructions.len() {
        let opcode_byte = instructions[offset];
        let opcode = opcode_byte.to_opcode();
        let definition = OpCode::get_definition(opcode);

        // Get span for this instruction
        let span = debug_info.get_span(offset);

        print!(
            "{} {} {} {}",
            format!("{:#04x}", offset).cyan(),
            format!("{}:{}-{}", span.line, span.start_column, span.end_column).bright_black(),
            definition.name.bright_white(),
            format!("({:#04x})", opcode_byte).cyan()
        );

        offset += 1;

        if !definition.operands_width.is_empty() {
            print!(" {}", "[".white().dimmed());

            for (i, &width) in definition.operands_width.iter().enumerate() {
                if i > 0 {
                    print!("{}", ", ".white().dimmed());
                }

                match width {
                    1 => {
                        let operand = read_uint8(instructions, offset);

                        // Pretty print register names
                        if is_register_operand(&opcode, i) {
                            print!("{}", format!("r{}", operand).green());
                        } else {
                            print!("{}", format!("{:#04x}", operand).white());
                        }
                        offset += 1;
                    }
                    2 => {
                        let operand = read_uint16(instructions, offset);

                        // Pretty print based on what the operand represents
                        if is_constant_index(&opcode, i) {
                            print!("{}", format!("const[{}]", operand).yellow());
                        } else if is_string_index(&opcode, i) {
                            print!("{}", format!("str[{}]", operand).magenta());
                        } else if is_global_index(&opcode, i) {
                            print!("{}", format!("global[{}]", operand).blue());
                        } else {
                            print!("{}", format!("{:#04x}", operand).white());
                        }
                        offset += 2;
                    }
                    _ => unreachable!("Unexpected operand width: {}", width),
                }
            }

            print!("{}", "]".white().dimmed());
        }

        println!();
    }
}

/// Check if an operand at a given position is a register
fn is_register_operand(opcode: &OpCode, operand_index: usize) -> bool {
    match opcode {
        // All register operands for each instruction type
        OpCode::LoadConstInt | OpCode::LoadConstFloat | OpCode::LoadString => {
            operand_index == 0 // dest_reg
        }
        OpCode::LoadNil | OpCode::LoadTrue | OpCode::LoadFalse => {
            operand_index == 0 // dest_reg
        }
        OpCode::AddInt
        | OpCode::SubtractInt
        | OpCode::MultiplyInt
        | OpCode::DivideInt
        | OpCode::ExponentInt
        | OpCode::AddFloat
        | OpCode::SubtractFloat
        | OpCode::MultiplyFloat
        | OpCode::DivideFloat
        | OpCode::ExponentFloat
        | OpCode::ConcatString => {
            true // All 3 operands are registers: dest, left, right
        }
        OpCode::NegateInt | OpCode::NegateFloat | OpCode::Not | OpCode::Move => {
            true // Both operands are registers: dest, src
        }
        OpCode::LessInt
        | OpCode::LessEqualInt
        | OpCode::GreaterInt
        | OpCode::GreaterEqualInt
        | OpCode::LessFloat
        | OpCode::LessEqualFloat
        | OpCode::GreaterFloat
        | OpCode::GreaterEqualFloat
        | OpCode::Equal
        | OpCode::NotEqual => {
            true // All 3 operands are registers: dest, left, right
        }
        OpCode::LoadGlobal | OpCode::LogAddr => {
            operand_index == 0 // dest_reg (operand 1 is global index)
        }
        OpCode::StoreGlobal => {
            operand_index == 1 // src_reg (operand 0 is global index)
        }

        OpCode::JumpIfFalse | OpCode::JumpUncond => true,

        OpCode::Halt => false,
    }
}

/// Check if an operand is a constant pool index
fn is_constant_index(opcode: &OpCode, operand_index: usize) -> bool {
    matches!(
        (opcode, operand_index),
        (OpCode::LoadConstInt, 1) | (OpCode::LoadConstFloat, 1)
    )
}

/// Check if an operand is a string table index
fn is_string_index(opcode: &OpCode, operand_index: usize) -> bool {
    matches!((opcode, operand_index), (OpCode::LoadString, 1))
}

/// Check if an operand is a global variable index
fn is_global_index(opcode: &OpCode, operand_index: usize) -> bool {
    matches!(
        (opcode, operand_index),
        (OpCode::LoadGlobal, 1) | (OpCode::StoreGlobal, 0)
    )
}

fn disassemble_constants(constants: &Vec<RuntimeValue>) {
    println!("{}", "--== Constants ==--".bright_yellow().bold());

    if constants.is_empty() {
        println!("{}", "No constants".white().dimmed())
    }

    for (i, constant) in constants.iter().enumerate() {
        println!(
            "{} {}",
            format!("{:04}", i).cyan(),
            format!("{:?}", constant).bright_white()
        );
    }
}

fn disassemble_string_table(strings: &Vec<String>) {
    println!("{}", "--== String Table ==--".bright_yellow().bold());

    if strings.is_empty() {
        println!("{}", "No strings".white().dimmed())
    }

    for (i, string) in strings.iter().enumerate() {
        println!(
            "{} {}",
            format!("{:04}", i).cyan(),
            format!("{:?}", string).bright_white()
        );
    }
}
