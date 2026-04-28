#[derive(Debug, Default, PartialEq, Eq)]
pub struct Tokens(Vec<Token>);

impl From<Vec<Token>> for Tokens {
    fn from(value: Vec<Token>) -> Self {
        Self(value)
    }
}

impl Tokens {
    fn push(&mut self, token: Token, word_cache: &mut WordCache) {
        self.0.push(token);
        word_cache.clean();
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

#[derive(Debug, Default, Clone, PartialEq, Eq)]
struct WordCache {
    inner: String,
}

impl WordCache {
    fn push(&mut self, c: char) {
        self.inner.push(c);
    }

    fn clean(&mut self) {
        self.inner.clear();
    }

    fn as_str(&self) -> &str {
        &self.inner
    }
}

impl From<String> for WordCache {
    fn from(inner: String) -> Self {
        Self { inner }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum TokenizeError {
    #[error("Failed to tokenize it: {str:?}")]
    InvalidProgram { str: String },

    #[error("Invalid charactor for harulisp: {char:?}")]
    InvalidCharactor { char: char },
}

pub fn tokenize(program: String) -> Result<Tokens, TokenizeError> {
    let mut tokens = Tokens::default();
    let mut word_cache = WordCache::default();
    // dbg!(&word_cache); // for Debugging

    for c in program.chars() {
        word_cache.push(c);
        // dbg!(&word_cache); // for Debugging

        let actual_str = word_cache.as_str();
        match actual_str {
            "(" => tokens.push(ReservedWord::LeftParenthesis.into(), &mut word_cache),
            ")" => tokens.push(ReservedWord::RightParenthesis.into(), &mut word_cache),
            " " => (),
            "\n" => (),
            _ => return Err(TokenizeError::InvalidCharactor { char: c }),
        }
    }

    Ok(tokens)
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
