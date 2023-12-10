/// Token, which is tokenized value. tokenize function is in lexer module.
/// this has some element as under.
/// - LParen
/// - RParen
/// - Integer
/// - Float
/// - String
#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    LParen,
    RParen,
    Integer(isize),
    Float(f64),
    String(String),
}

#[derive(Debug)]
pub struct TokenError {
    pub char: char,
}

pub enum Alfabet {}

/// Display implementation of Token.
impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Token::LParen => write!(f, "("),
            Token::RParen => write!(f, ")"),
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

impl PartialEq<Token> for &mut Token {
    fn eq(&self, other: &Token) -> bool {
        self == other
    }
}

impl std::ops::Deref for Token {
    type Target = Token;
    fn deref(&self) -> &Self::Target {
        self
    }
}

impl std::ops::DerefMut for Token {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self
    }
}
