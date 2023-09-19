/// Token, which is tokenized value. tokenize function is in lexer module.
#[derive(Debug, PartialEq)]
pub enum Token {
    Number(TokenNumber),
    Symbol(ReservedWord),
    String(String),
    StringQuotation,
    LParen,
    RParen,
}

/// TokenNumber, which is tokenized number.
#[derive(Debug, PartialEq)]
pub enum TokenNumber {
    Integer(isize),
    Float(f64),
}

#[derive(Debug)]
pub struct TokenError {
    char: char,
}

/// ReservedWord, contains some words
#[derive(Debug, PartialEq)]
pub enum ReservedWord {
    Define,
    If,
    Lambda,
    Operation(Operation),
    Comparison(Comparison),
}

/// Operation, contains some words which is used by ReservedWord enum.
#[derive(Debug, PartialEq)]
pub enum Operation {
    Plus,
    Minus,
    Asterisk,
    Slash,
}

/// Comparison, contains some words which is used by ReservedWord enum.
#[derive(Debug, PartialEq)]
enum Comparison {
    Shorter,
    Greater,
    Equal,
    NotEqual,
}

/// Display implementation of Token.
impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Token::Number(token_number) => write!(f, "{}", token_number),
            Token::String(string) => write!(f, "{}", string),
            Token::Symbol(string) => write!(f, "{}", string),
            Token::StringQuotation => write!(f, "\""),
            Token::LParen => write!(f, "("),
            Token::RParen => write!(f, ")"),
        }
    }
}

/// Display implementation for TokenNumber.
impl std::fmt::Display for TokenNumber {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            TokenNumber::Integer(integer) => write!(f, "{}", integer),
            TokenNumber::Float(float) => write!(f, "{}", float),
        }
    }
}

/// Display implementation for TokenError.
impl std::fmt::Display for TokenError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "unexpected character: {}", self.char)
    }
}

/// Display implementation for ReservedWord.
impl std::fmt::Display for ReservedWord {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            ReservedWord::Define => write!(f, "Define"),
            ReservedWord::If => write!(f, "If"),
            ReservedWord::Lambda => write!(f, "Lambda"),
            ReservedWord::Operation(operation) => write!(f, "{}", operation),
            ReservedWord::Comparison(comparison) => write!(f, "{}", comparison),
        }
    }
}

/// Display implementation for Operation.
impl std::fmt::Display for Operation {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Operation::Plus => write!(f, "Plus"),
            Operation::Minus => write!(f, "Minus"),
            Operation::Asterisk => write!(f, "Asterisk"),
            Operation::Slash => write!(f, "Slash"),
        }
    }
}

impl std::fmt::Display for Comparison {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Comparison::Shorter => write!(f, "Shorter"),
            Comparison::Greater => write!(f, "Greater"),
            Comparison::Equal => write!(f, "Equal"),
            Comparison::NotEqual => write!(f, "NotEqual"),
        }
    }
}
