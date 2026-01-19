use hydor::{compiler::compiler::Compiler, lexer::Lexer, parser::parser::Parser};

fn main() {
    let source = r#"
    10+10
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
                    println!("{r:#?}");
                }
                Err(ec) => ec.report_all(source),
            }
        }
        Err(e) => e.report_all(source),
    }
}
