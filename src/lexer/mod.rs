use crate::data::tokens::Token;
use crate::data::tokens::TokenError;

/// tokenize function, convert from &str to Vec<Token>. If this function is failed, Return TokenError wrapped by Result's Error.
pub fn tokenize(program: &str) -> Result<Vec<Token>, TokenError> {
    let p: String = program.to_string();
    let mut program_vector: Vec<char> = p.chars().collect();
    let mut tokens: Vec<Token> = vec![];
    let mut is_string_literal: bool = false;

    // vectorがからになるまでしょりする
    // ひともじづつしょりする
    while is_not_empty(program_vector.clone()) {
        let char = program_vector.pop().unwrap();

        // もしもis_string_literalがtrueだったら、falseになるまでそれいこうのもじをToken::Stringとしてtokensにpushする
        if is_string_literal == true {
            let mut string_literal: Vec<char> = vec![];
        }

        match char {
            '(' => tokens.push(Token::LParen),
            ')' => tokens.push(Token::RParen),
            '\"' => {
                tokens.push(Token::StringQuotation);
                switch_boolean(is_string_literal);
            }
            '\'' => {
                tokens.push(Token::StringQuotation);
                switch_boolean(is_string_literal);
            }
            _ => {
                // ここでIntegerかFloatかのはんだんをする
            }
        }
    }

    // program_vectorがからになったらtokensをOkにつつんでかえす
    Ok(tokens)
}

fn is_not_empty(vector: Vec<char>) -> bool {
    !vector.is_empty()
}

fn switch_boolean(b: bool) -> bool {
    !b
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
        let tokens: Vec<Token> = tokenize(PROGRAM).unwrap_or(vec![]);
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

    /// test_string_quotation test, whether tokenize function correctly handle quotation or not.
    #[test]
    fn test_string_quotation() {
        const PROGRAM: &str = "(define sample_string \"hoge fuga\")";
        let tokens: Vec<Token> = tokenize(PROGRAM).unwrap_or(vec![]);
        assert_eq!(
            tokens,
            vec![
                Token::LParen,
                Token::String("define".to_string()),
                Token::String("sample_string".to_string()),
                Token::StringQuotation,
                Token::String("hoge fuga".to_string()),
                Token::StringQuotation,
                Token::RParen,
            ]
        );
    }
}
