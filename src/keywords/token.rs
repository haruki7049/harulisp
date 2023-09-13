#[derive(Debug, PartialEq)]
#[allow(dead_code)]
pub enum Token {
    Integer(isize),
    Float(f64),
    String(String),
    Symbol(String),
    LParen,
    RParen,
}

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Token::Integer(number) => write!(f, "{}", number),
            Token::Float(number) => write!(f, "{}", number),
            Token::String(string) => write!(f, "{}", string),
            Token::Symbol(symbol) => write!(f, "{}", symbol),
            Token::LParen => write!(f, "("),
            Token::RParen => write!(f, ")"),
        }
    }
}

#[derive(Debug)]
pub struct TokenError {
    ch: char,
}

impl std::fmt::Display for TokenError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "unexpected character: {}", self.ch)
    }
}
