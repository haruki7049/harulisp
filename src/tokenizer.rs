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

#[derive(Debug, thiserror::Error)]
pub enum TokenizeError {
    #[error("Failed to tokenize it: {str:?}")]
    InvalidProgram { str: String },
}

pub fn tokenize(s: String) -> Result<Tokens, TokenizeError> {
    Ok(Tokens(vec![]))
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
}
