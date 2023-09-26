use crate::data::tokens::Token;
use crate::data::tokens::TokenError;

/// tokenize function, convert from &str to Vec<Token>. If this function is failed, Return TokenError wrapped by Result's Error.
pub fn tokenize(program: &str) -> Result<Vec<Token>, TokenError> {
    // ")" の片端にスペースを追加
    let p: &str = &program.replace(")", " )");

    // '\'' と '\"' の

    let program_vector: Vec<char> = make_vector_char(p);
    let mut tokens: Vec<Token> = vec![];

    //
    let words: Vec<String> = wordnize(&program_vector);

    for term in words {
        let t: &str = &term.as_str();
        
        match t {
            "(" => tokens.push(Token::LParen),
            ")" => tokens.push(Token::RParen),
            "\'" | "\"" => tokens.push(Token::StringQuotation),
            _ => {
                let i = t.parse::<isize>();
                if i.is_ok() {
                    tokens.push(Token::Integer(i.unwrap()));
                } else {
                    let f = t.parse::<f64>();
                    if f.is_ok() {
                        tokens.push(Token::Float(f.unwrap()));
                    } else {
                        let s = t.parse::<String>();
                        if s.is_ok() {
                            tokens.push(Token::String(s.unwrap()));
                        }
                    }
                }
            }
        }
    }

    Ok(tokens)
}

fn make_vector_char(str: &str) -> Vec<char> {
    str.chars().collect()
}

fn wordnize(program_vector: &Vec<char>) -> Vec<String> {
    let mut result: _ = vec![];
    let mut word: Vec<char> = vec![];
    let mut literal_mode: bool = false; // さいしょはかならすリテラルはこないため、falseに設定

    for ch in program_vector {
        match ch {
            '(' => result.push('('.to_string()),
            ')' => result.push(')'.to_string()),
            '\"' => {
                result.push('\"'.to_string());
                literal_mode = switch_bool(literal_mode);
            }
            '\'' => {
                result.push('\''.to_string());
                literal_mode = switch_bool(literal_mode);
            }
            _ => {
                let w = create_word(*ch, &mut word, literal_mode);
                if w.is_ok() {
                    // 単語が完成したら、最後のスペースもしくはクオーテーションを取る
                    word.pop();
                    
                    result.push(word.iter().collect());
                    word = vec![];
                }
            }
        }
    }

    result
}

/// add char to Vec<char>, and return true if the char is ' '.
fn create_word(ch: char, word: &mut Vec<char>, literal_mode: bool) -> Result<Vec<char>, String> {
    // chからwordに一つずつ追加する
    word.push(ch);
    if literal_mode == true {
        if ch == '\"' || ch == '\'' {
            Ok(word.to_vec())
        } else {
            Err("not yet, this charactor is not quotation.".to_string())
        }
    } else {
        if ch == ' ' {
            // もしも空白がchに入っていたなら
            Ok(word.to_vec())
        } else {
            Err("not yet, this charactor is not whitespace.".to_string())
        }
    }

}

fn switch_bool(b: bool) -> bool {
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
        const PROGRAM: &str = "(define sample_string \'hoge fuga\')";
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

    //#[test]
    //fn test_string_error() {
    //    const ERROR_PROGRAM: &str = "(Define x 1)";
    //    let tokens: Vec<Token> = tokenize(ERROR_PROGRAM).unwrap_or(vec![]);
    //    assert_eq!(tokens, vec![Token::RParen]);
    //}
}
