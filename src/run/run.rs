use crate::{
    compiler::{compiler::Compiler, disassembler::disassemble},
    lexer::Lexer,
    parser::parser::Parser,
};

pub fn run_file(source: String) {
    let mut lexer = Lexer::new(&source);
    let tokens = lexer.tokenize();
    let mut parser = Parser::new(tokens);

    // TODO: Reformat this nest
    match parser.parse_program() {
        Ok(program) => {
            let mut compiler = Compiler::new();
            match compiler.compile_program(program) {
                Ok(bytecode) => disassemble(&bytecode),
                Err(ec) => ec.report_all(&source),
            };
        }
        Err(ec) => {
            ec.report_all(&source);
        }
    }
}
