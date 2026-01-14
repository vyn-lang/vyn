#[derive(Debug, Clone, PartialEq)]
pub struct TokenInfo {
    pub token: Token,
    pub line: usize,
    pub column: usize,
    pub end_column: usize,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    // Literals
    Integer(i64),
    Float(f64),
    String(String),
    Identifier(String),

    // Keywords
    Function,
    Let,
    True,
    False,
    If,
    Else,
    Return,

    // Operators
    Assign,
    Plus,
    Minus,
    Bang,
    Asterisk,
    Slash,

    // Comparison
    LessThan,
    GreaterThan,
    Equal,
    NotEqual,
    LessThanEqual,
    GreaterThanEqual,

    // Delimiters
    Comma,
    Semicolon,
    Colon,
    Newline,
    Dot,
    BoxColon, // ::

    // Grouping
    LeftParenthesis,
    RightParenthesis,
    LeftBrace,
    RightBrace,
    LeftBracket,
    RightBracket,

    // Special
    EndOfFile,
    Illegal(char),
}

impl Token {
    pub fn lookup_identifier(identifier: &str) -> Token {
        match identifier {
            "fn" => Token::Function,
            "let" => Token::Let,
            "true" => Token::True,
            "false" => Token::False,
            "if" => Token::If,
            "else" => Token::Else,
            "return" => Token::Return,
            _ => Token::Identifier(identifier.to_string()),
        }
    }
}
