use std::time::Instant;

use hydor::{
    compiler::{compiler::Compiler, disassembler::disassemble},
    hydor_vm::vm::HydorVM,
    lexer::Lexer,
    parser::parser::Parser,
};

fn main() {
    let source = r#"
    10==10
    not "" != "hello"
    "hello" == "hello"
        "#;
    let mut lexer = Lexer::new(source);
    let mut parser = Parser::new(lexer.tokenize());
    let program = parser.parse_program();

    // NOTE: This isnt the file's final product
    // This is just for debugging purposes
    match program {
        Ok(p) => {
            let mut compiler = Compiler::new();
            let result = compiler.compile_program(p);

            match result {
                Ok(r) => {
                    disassemble(&r);
                    let mut vm = HydorVM::new(r.instructions, r.constants, r.string_table);

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
