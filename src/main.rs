use std::time::Instant;

use vyn::{
    compiler::{compiler::Compiler, disassembler::disassemble},
    lexer::Lexer,
    parser::parser::Parser,
    vyn_vm::vm::VynVM,
};

fn main() {
    let source = r#" 
    let @person_1: String = "Bob";
    let @person_2: String = "Alice";

    person_1 = person_2 = "None";
        "#;
    let mut lexer = Lexer::new(source);
    let mut parser = Parser::new(lexer.tokenize());
    let program = parser.parse_program();

    // TODO: A cli to hsndle this abomination
    match program {
        Ok(p) => {
            let mut compiler = Compiler::new();
            let result = compiler.compile_program(p);

            match result {
                Ok(r) => {
                    disassemble(&r);
                    let mut vm = VynVM::new(r.instructions, r.constants, r.string_table);

                    let start = Instant::now();
                    vm.execute().unwrap();
                    let dur = start.elapsed();

                    println!(); // newline

                    for (i, reg) in vm.get_registers().iter().enumerate() {
                        if reg.is_string() {
                            let str = vm.get_string(reg.as_string_index().unwrap());
                            println!("r{i:?}: {str}");
                            continue;
                        }

                        println!("r{i:?} {reg:?}");
                    }

                    println!("Program took {dur:?}");
                }
                Err(ec) => ec.report_all(source),
            }
        }
        Err(e) => e.report_all(source),
    }
}
