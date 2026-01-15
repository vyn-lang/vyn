use hydor::{lexer::Lexer, parser::parser::Parser};

fn main() {
    let source = r#"
not not 10+0   "random string"
+10 // incomplete arithmetic expr
        "#;
    let mut lexer = Lexer::new(source);
    let mut parser = Parser::new(lexer.tokenize());
    let result = parser.parse_program();

    if result.is_ok() {
        println!("{:#?}", result.program);
    } else {
        result.errors.report_all(source);
    }
}
