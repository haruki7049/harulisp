#[derive(Debug, Default, PartialEq, Eq)]
pub struct Tokens(Vec<Token>);

#[derive(Debug, PartialEq, Eq)]
pub enum Token {
    String(String),
    Reserved(ReservedWord),
    Integer(i32),
}

#[derive(Debug, PartialEq, Eq)]
pub enum ReservedWord {
    Def,
    Main,
    Print,
    Lambda,
    LeftParenthesis,
    RightParenthesis,
    Dash,
}

impl Into<Token> for ReservedWord {
    fn into(self) -> Token {
        match self {
            Self::Def => Token::Reserved(Self::Def),
            Self::Main => Token::Reserved(Self::Main),
            Self::Print => Token::Reserved(Self::Print),
            Self::Lambda => Token::Reserved(Self::Lambda),
            Self::LeftParenthesis => Token::Reserved(Self::LeftParenthesis),
            Self::RightParenthesis => Token::Reserved(Self::RightParenthesis),
            Self::Dash => Token::Reserved(Self::Dash),
        }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum TokenizeError {
    #[error("Failed to tokenize it: {str:?}")]
    InvalidProgram { str: String },
}

pub fn tokenize(s: String) -> Result<Tokens, TokenizeError> {
    if s == "" {
        return Ok(Tokens(vec![]));
    }

    if s == "()" {
        return Ok(Tokens(vec![
            ReservedWord::LeftParenthesis.into(),
            ReservedWord::RightParenthesis.into(),
        ]));
    }

    Err(TokenizeError::InvalidProgram { str: s })
}

#[cfg(test)]
mod tests {
    use super::tokenize;
    use super::Tokens;

    #[test]
    fn tokenize_empty_str() -> Result<(), Box<dyn std::error::Error>> {
        let program: String = String::new();
        let result = tokenize(program)?;

        assert_eq!(result, Tokens(vec![]));
        Ok(())
    }

    mod parentheses {
        use crate::tokenizer::ReservedWord;

        use super::super::tokenize;
        use super::super::Tokens;

        #[test]
        fn tokenize_parentheses() -> Result<(), Box<dyn std::error::Error>> {
            let program: String = String::from("()");
            let result = tokenize(program)?;

            assert_eq!(
                result,
                Tokens(vec![
                    ReservedWord::LeftParenthesis.into(),
                    ReservedWord::RightParenthesis.into()
                ])
            );
            Ok(())
        }
    }
}
