use hydor::{lexer::Lexer, parser::parser::Parser};

fn main() {
    let mut lexer = Lexer::new("-----20");
    let mut parser = Parser::new(lexer.tokenize());

    let ast = parser.parse_program();

    println!("{ast:#?}");
}
