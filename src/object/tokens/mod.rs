/// Token, which is tokenized value. tokenize function is in lexer module.
#[derive(Debug, PartialEq)]
pub enum Token {
    Number(TokenNumber),
    String(String),
    LParen,
    RParen,
}

/// TokenNumber, which is tokenized number.
#[derive(Debug, PartialEq)]
pub enum TokenNumber {
    Integer(isize),
    Float(f64),
}

/// Display implementation of Token.
impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Token::Number(token_number) => write!(f, "{}", token_number),
            Token::String(string) => write!(f, "{}", string),
            Token::LParen => write!(f, "("),
            Token::RParen => write!(f, ")"),
        }
    }
}

/// Display implementation of TokenNumber.
impl std::fmt::Display for TokenNumber {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            TokenNumber::Integer(integer) => write!(f, "{}", integer),
            TokenNumber::Float(float) => write!(f, "{}", float),
        }
    }
}
