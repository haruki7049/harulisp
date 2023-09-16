#[derive(Debug, PartialEq)]
pub enum Token {
    Number(Number),
    String(String),
    LParen,
    RParen,
}

#[derive(Debug, PartialEq)]
pub enum Number {
    Integer(isize),
    Float(f64),
}

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Token::Number(number) => write!(f, "{}", number),
            Token::String(string) => write!(f, "{}", string),
            Token::LParen => write!(f, "("),
            Token::RParen => write!(f, ")"),
        }
    }
}

impl std::fmt::Display for Number {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Number::Integer(integer) => write!(f, "{}", integer),
            Number::Float(float) => write!(f, "{}", float),
        }
    }
}
