use hydor::lexer::Lexer;

fn main() {
    let mut lexer = Lexer::new("let x = \"Hello world\"");
    println!("{:#?}", lexer.tokenize())
}
