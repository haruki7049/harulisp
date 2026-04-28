#[derive(Debug, Default, PartialEq, Eq)]
pub struct Tokens(Vec<Token>);

impl From<Vec<Token>> for Tokens {
    fn from(value: Vec<Token>) -> Self {
        Self(value)
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum Token {
    String(String),
    Reserved(ReservedWord),
    Integer(i32),
}

impl From<ReservedWord> for Token {
    fn from(value: ReservedWord) -> Self {
        match value {
            ReservedWord::Def => Self::Reserved(ReservedWord::Def),
            ReservedWord::Main => Self::Reserved(ReservedWord::Main),
            ReservedWord::Print => Self::Reserved(ReservedWord::Print),
            ReservedWord::Lambda => Self::Reserved(ReservedWord::Lambda),
            ReservedWord::LeftParenthesis => Self::Reserved(ReservedWord::LeftParenthesis),
            ReservedWord::RightParenthesis => Self::Reserved(ReservedWord::RightParenthesis),
            ReservedWord::Dash => Self::Reserved(ReservedWord::Dash),
        }
    }
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

#[derive(Debug, thiserror::Error)]
pub enum TokenizeError {
    #[error("Failed to tokenize it: {str:?}")]
    InvalidProgram { str: String },

    #[error("Invalid charactor for harulisp: {char:?}")]
    InvalidCharactor { char: char },
}

pub fn tokenize(program: String) -> Result<Tokens, TokenizeError> {
    let mut tokens_inner: Vec<Token> = Vec::new();

    for c in program.chars().into_iter() {
        match c {
            '(' => tokens_inner.push(ReservedWord::LeftParenthesis.into()),
            ')' => tokens_inner.push(ReservedWord::RightParenthesis.into()),
            ' ' => (),
            '\n' => (),
            _ => return Err(TokenizeError::InvalidCharactor { char: c }),
        }
    }

    Ok(tokens_inner.into())
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
            // Single parentheses
            let program: String = String::from("()");
            let result = tokenize(program)?;

            assert_eq!(
                result,
                Tokens(vec![
                    ReservedWord::LeftParenthesis.into(),
                    ReservedWord::RightParenthesis.into()
                ])
            );

            // Double parentheses
            let program: String = String::from("(())");
            let result = tokenize(program)?;

            assert_eq!(
                result,
                Tokens(vec![
                    ReservedWord::LeftParenthesis.into(),
                    ReservedWord::LeftParenthesis.into(),
                    ReservedWord::RightParenthesis.into(),
                    ReservedWord::RightParenthesis.into(),
                ])
            );

            // Return Ok(())
            Ok(())
        }
    }
}
