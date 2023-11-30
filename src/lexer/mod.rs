use crate::data::tokens::Token;
use crate::data::tokens::TokenError;
use regex_lite::Regex;

/// tokenize function, convert from &str to Vec<Token>. If this function is failed, Return TokenError wrapped by Result's Error.
pub fn tokenize(program: &str) -> Result<Vec<Token>, TokenError> {
    let re: Regex = Regex::new(r#"[()]|\w+|-?\d|".*"|'.*'"#).unwrap();
    let tokens: Vec<&str> = re.find_iter(program).map(|m| m.as_str()).collect();

    let mut result: Vec<Token> = Vec::new();

    for token in tokens {
        match token {
            "(" => result.push(Token::LParen),
            ")" => result.push(Token::RParen),
            "\'" | "\"" => {}
            _ => {
                if let Ok(i) = token.parse::<isize>() {
                    result.push(Token::Integer(i));
                } else if let Ok(f) = token.parse::<f64>() {
                    result.push(Token::Float(f));
                } else if let Ok(s) = token.parse::<String>() {
                    result.push(Token::String(s));
                }
            }
        }
    }

    Ok(result)
}

/// lexer test.
#[cfg(test)]
mod test_lexer {
    use crate::data::tokens::Token;
    use crate::lexer::tokenize;

    /// test_one_sentence test, whether tokenize function correctly convert from &str to Token or not.
    #[test]
    fn test_one_sentence() {
        const PROGRAM: &str = "(define x 1)";
        let tokens: Vec<Token> = tokenize(PROGRAM).unwrap_or_default();
        assert_eq!(
            tokens,
            vec![
                Token::LParen,
                Token::String("define".to_string()),
                Token::String("x".to_string()),
                Token::Integer(1),
                Token::RParen,
            ]
        );
    }

    /// test_new_line test, whether tokenize function ignore new_line and space indent or not.
    #[test]
    fn test_new_line() {
        const PROGRAM: &str = "(
            define x 19
            )";
        let tokens: Vec<Token> = tokenize(PROGRAM).unwrap_or_default();
        assert_eq!(
            tokens,
            vec![
                Token::LParen,
                Token::String("define".to_string()),
                Token::String("x".to_string()),
                Token::Integer(19),
                Token::RParen,
            ]
        );
    }

    /// test_string_single_quotation test, whether tokenize function correctly handle single_quotation or not.
    #[test]
    fn test_string_single_quotation() {
        const PROGRAM: &str = "(define sample_string \'hoge fuga\')";
        let tokens: Vec<Token> = tokenize(PROGRAM).unwrap_or_default();
        assert_eq!(
            tokens,
            vec![
                Token::LParen,
                Token::String("define".to_string()),
                Token::String("sample_string".to_string()),
                Token::String("\'hoge fuga\'".to_string()),
                Token::RParen,
            ]
        );
    }

    /// test_string_double_quotation test, whether tokenize function correctly handle double_quotation or not.
    #[test]
    fn test_string_double_quotation() {
        const PROGRAM: &str = "(define sample_string \"hoge fuga\")";
        let tokens: Vec<Token> = tokenize(PROGRAM).unwrap_or_default();
        assert_eq!(
            tokens,
            vec![
                Token::LParen,
                Token::String("define".to_string()),
                Token::String("sample_string".to_string()),
                Token::String("\"hoge fuga\"".to_string()),
                Token::RParen,
            ]
        );
    }
}
