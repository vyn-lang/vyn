use crate::utils::Span;
use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub struct TokenInfo {
    pub token: Token,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    // Literals
    Integer(i32),
    Float(f64),
    String(String),
    Identifier(String),

    // Operators
    Assign,
    Plus,
    Minus,
    Asterisk,
    Slash,
    Caret,
    Bang,
    At,
    Hashtag,

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

    // Keywords
    Function,
    Let,
    True,
    False,
    If,
    Else,
    Nil,
    Or,
    And,
    Not,
    Type,
    Return,
    Stdout,
    Static,
    Loop,
    Break,
    Continue,
    For,
    When,
    Every,
    In,
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            // Literals - show the actual value
            Token::Integer(n) => write!(f, "{}", n),
            Token::Float(fl) => write!(f, "{}", fl),
            Token::String(s) => write!(f, "\"{}\"", s),
            Token::Identifier(name) => write!(f, "{}", name),

            // For everything else, delegate to TokenType's Display
            _ => write!(f, "{}", self.get_token_type()),
        }
    }
}

#[derive(Eq, PartialEq, Hash, Debug, Clone)]
pub enum TokenType {
    // Literals
    Integer,
    Float,
    String,
    Identifier,

    // Operators
    Assign,
    Plus,
    Minus,
    Asterisk,
    Slash,
    Caret,
    Bang,
    At,
    Hashtag,

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
    Illegal,

    // Keywords
    Function,
    Let,
    True,
    False,
    If,
    Else,
    Nil,
    Type,
    Or,
    And,
    Not,
    Return,
    Stdout,
    Static,
    Loop,
    Break,
    Continue,
    For,
    When,
    Every,
    In,
}

impl TokenType {
    pub fn is_delimiter(&self) -> bool {
        match self {
            Self::Semicolon | Self::Newline | Self::EndOfFile => true,
            _ => false,
        }
    }
}

impl fmt::Display for TokenType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            // Literals
            TokenType::Integer => write!(f, "Integer"),
            TokenType::Float => write!(f, "Float"),
            TokenType::String => write!(f, "String"),
            TokenType::Identifier => write!(f, "Identifier"),

            // Operators
            TokenType::Assign => write!(f, "="),
            TokenType::Plus => write!(f, "+"),
            TokenType::Minus => write!(f, "-"),
            TokenType::Asterisk => write!(f, "*"),
            TokenType::Slash => write!(f, "/"),
            TokenType::Caret => write!(f, "^"),
            TokenType::Bang => write!(f, "!"),
            TokenType::At => write!(f, "@"),
            TokenType::Hashtag => write!(f, "#"),

            // Comparison
            TokenType::LessThan => write!(f, "<"),
            TokenType::GreaterThan => write!(f, ">"),
            TokenType::Equal => write!(f, "=="),
            TokenType::NotEqual => write!(f, "!="),
            TokenType::LessThanEqual => write!(f, "<="),
            TokenType::GreaterThanEqual => write!(f, ">="),

            // Delimiters
            TokenType::Comma => write!(f, ","),
            TokenType::Semicolon => write!(f, ";"),
            TokenType::Colon => write!(f, ":"),
            TokenType::Newline => write!(f, "newline"),
            TokenType::Dot => write!(f, "."),
            TokenType::BoxColon => write!(f, "::"),

            // Grouping
            TokenType::LeftParenthesis => write!(f, "("),
            TokenType::RightParenthesis => write!(f, ")"),
            TokenType::LeftBrace => write!(f, "{{"),
            TokenType::RightBrace => write!(f, "}}"),
            TokenType::LeftBracket => write!(f, "["),
            TokenType::RightBracket => write!(f, "]"),

            // Special
            TokenType::EndOfFile => write!(f, "EOF"),
            TokenType::Illegal => write!(f, "illegal"),

            // Keywords
            TokenType::Function => write!(f, "fn"),
            TokenType::Let => write!(f, "let"),
            TokenType::True => write!(f, "true"),
            TokenType::False => write!(f, "false"),
            TokenType::If => write!(f, "if"),
            TokenType::Else => write!(f, "else"),
            TokenType::Nil => write!(f, "nil"),
            TokenType::Or => write!(f, "or"),
            TokenType::Type => write!(f, "type"),
            TokenType::And => write!(f, "and"),
            TokenType::Not => write!(f, "not"),
            TokenType::Return => write!(f, "return"),
            TokenType::Stdout => write!(f, "stdout"),
            TokenType::Static => write!(f, "static"),
            TokenType::Loop => write!(f, "loop"),
            TokenType::Break => write!(f, "break"),
            TokenType::Continue => write!(f, "continue"),
            TokenType::For => write!(f, "for"),
            TokenType::When => write!(f, "when"),
            TokenType::Every => write!(f, "every"),
            TokenType::In => write!(f, "in"),
        }
    }
}

impl Token {
    pub fn lookup_identifier(identifier: &str) -> Token {
        match identifier {
            "fn" => Token::Function,
            "let" => Token::Let,
            "true" => Token::True,
            "false" => Token::False,
            "if" => Token::If,
            "nil" => Token::Nil,
            "or" => Token::Or,
            "and" => Token::And,
            "not" => Token::Not,
            "type" => Token::Type,
            "else" => Token::Else,
            "return" => Token::Return,
            "stdout" => Token::Stdout,
            "static" => Token::Static,
            "loop" => Token::Loop,
            "continue" => Token::Continue,
            "break" => Token::Break,
            "for" => Token::For,
            "when" => Token::When,
            "every" => Token::Every,
            "in" => Token::In,
            _ => Token::Identifier(identifier.to_string()),
        }
    }

    pub fn get_token_type(&self) -> TokenType {
        match self {
            // Literals
            Token::Integer(_) => TokenType::Integer,
            Token::Float(_) => TokenType::Float,
            Token::String(_) => TokenType::String,
            Token::Identifier(_) => TokenType::Identifier,

            // Operators
            Token::Assign => TokenType::Assign,
            Token::Plus => TokenType::Plus,
            Token::Minus => TokenType::Minus,
            Token::Asterisk => TokenType::Asterisk,
            Token::Slash => TokenType::Slash,
            Token::Caret => TokenType::Caret,
            Token::Bang => TokenType::Bang,
            Token::At => TokenType::At,
            Token::Hashtag => TokenType::Hashtag,

            // Comparison
            Token::LessThan => TokenType::LessThan,
            Token::GreaterThan => TokenType::GreaterThan,
            Token::Equal => TokenType::Equal,
            Token::NotEqual => TokenType::NotEqual,
            Token::LessThanEqual => TokenType::LessThanEqual,
            Token::GreaterThanEqual => TokenType::GreaterThanEqual,

            // Delimiters
            Token::Comma => TokenType::Comma,
            Token::Semicolon => TokenType::Semicolon,
            Token::Colon => TokenType::Colon,
            Token::Newline => TokenType::Newline,
            Token::Dot => TokenType::Dot,
            Token::BoxColon => TokenType::BoxColon,

            // Grouping
            Token::LeftParenthesis => TokenType::LeftParenthesis,
            Token::RightParenthesis => TokenType::RightParenthesis,
            Token::LeftBrace => TokenType::LeftBrace,
            Token::RightBrace => TokenType::RightBrace,
            Token::LeftBracket => TokenType::LeftBracket,
            Token::RightBracket => TokenType::RightBracket,

            // Special
            Token::EndOfFile => TokenType::EndOfFile,
            Token::Illegal(_) => TokenType::Illegal,

            // Keywords
            Token::Function => TokenType::Function,
            Token::Let => TokenType::Let,
            Token::True => TokenType::True,
            Token::False => TokenType::False,
            Token::If => TokenType::If,
            Token::Else => TokenType::Else,
            Token::Nil => TokenType::Nil,
            Token::Or => TokenType::Or,
            Token::Type => TokenType::Type,
            Token::And => TokenType::And,
            Token::Not => TokenType::Not,
            Token::Return => TokenType::Return,
            Token::Stdout => TokenType::Stdout,
            Token::Static => TokenType::Static,
            Token::Loop => TokenType::Loop,
            Token::Break => TokenType::Break,
            Token::Continue => TokenType::Continue,
            Token::For => TokenType::For,
            Token::When => TokenType::When,
            Token::Every => TokenType::Every,
            Token::In => TokenType::In,
        }
    }
}
