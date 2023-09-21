/// Token, which is tokenized value. tokenize function is in lexer module.
/// this has some element as under.
/// - LParen
/// - RParen
/// - Integer
/// - Float
/// - String
#[derive(Debug, PartialEq)]
pub enum Token {
    LParen,
    RParen,
    StringQuotation,
    Integer(isize),
    Float(f64),
    String(String),
}

#[derive(Debug)]
pub struct TokenError {
    char: char,
}

/// Display implementation of Token.
impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Token::LParen => write!(f, "("),
            Token::RParen => write!(f, ")"),
            Token::StringQuotation => write!(f, "\""),
            Token::Integer(integer) => write!(f, "{}", integer),
            Token::Float(float) => write!(f, "{}", float),
            Token::String(string) => write!(f, "{}", string),
        }
    }
}

/// Display implementation for TokenError.
impl std::fmt::Display for TokenError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "unexpected character: {}", self.char)
    }
}
