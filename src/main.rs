use std::time::Instant;

use hydor::{
    compiler::{compiler::Compiler, disassembler::disassemble},
    hydor_vm::vm::HydorVM,
    lexer::Lexer,
    parser::parser::Parser,
};

fn main() {
    let source = r#"
    (10+10)^2
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
                    // disassemble(&r);
                    let mut vm = HydorVM::new(r.instructions, r.constants, r.string_table);

                    let start = Instant::now();
                    vm.run();
                    let dur = start.elapsed();

                    println!("Program took {dur:?}");
                }
                Err(ec) => ec.report_all(source),
            }
        }
        Err(e) => e.report_all(source),
    }
}
