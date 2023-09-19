use crate::data::tokens::Token;
use crate::data::tokens::TokenError;

/// tokenize function, convert from &str to Vec<Token>. If this function is failed, Return TokenError wrapped by Result's Error.
fn tokenize(program: &str) -> Result<Vec<Token>, TokenError> {
    Ok(vec![Token::RParen])
}

/// lexer test.
#[cfg(test)]
mod test_lexer {
    use crate::data::tokens::Operation;
    use crate::data::tokens::ReservedWord;
    use crate::data::tokens::Token;
    use crate::data::tokens::TokenNumber;
    use crate::lexer::tokenize;

    /// test_one_sentence test, whether tokenize function correctly convert from &str to Token or not.
    #[test]
    fn test_one_sentence() {
        const PROGRAM: &str = "(define x 1)";
        let tokens: Vec<Token> = tokenize(PROGRAM).unwrap_or(vec![]);
        assert_eq!(
            tokens,
            vec![
                Token::LParen,
                Token::Symbol(ReservedWord::Define),
                Token::Symbol(ReservedWord::Operation(Operation::Plus)),
                Token::Number(TokenNumber::Integer(1)),
                Token::RParen,
            ]
        );
    }

    /// test_string_quotation test, whether tokenize function correctly handle quotation or not.
    #[test]
    fn test_string_quotation() {
        const PROGRAM: &str = "(define sample_string \"hoge fuga\")";
        let tokens: Vec<Token> = tokenize(PROGRAM).unwrap_or(vec![]);
        assert_eq!(
            tokens,
            vec![
                Token::LParen,
                Token::Symbol(ReservedWord::Define),
                Token::Number(TokenNumber::Integer(1)),
                Token::String("sample_string".to_string()),
                Token::RParen,
            ]
        );
    }
}
