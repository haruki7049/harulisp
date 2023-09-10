use std::fmt;

#[derive(Debug, PartialEq)]
#[allow(dead_code)]
pub enum Token {
    Integer(isize),
    Float(f64),
    String(String),
    Symbol(String),
    LParen,
    RParen,
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Token::Integer(number) => write!(f, "{}", number),
            Token::Float(number) => write!(f, "{}", number),
            Token::String(string) => write!(f, "{}", string),
            Token::Symbol(symbol) => write!(f, "{}", symbol),
            Token::LParen => write!(f, "("),
            Token::RParen => write!(f, ")"),
        }
    }
}

#[derive(Debug)]
pub struct TokenError {
    ch: char,
}

impl fmt::Display for TokenError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "unexpected character: {}", self.ch)
    }
}

pub fn tokenize(program: String) -> Result<Vec<Token>, TokenError> {
    let mut string_literal: Vec<String> = vec![];
    let splited_program_by_whitespace: String = replace_parenthese_by_whitespace(program);
    let p: String = replace_quatation_by_whitespace(splited_program_by_whitespace);
    let words_iter = p.split_whitespace();
    let mut tokens: Vec<Token> = Vec::new();

    for word in words_iter {
        match word {
            "(" => tokens.push(Token::LParen),
            ")" => tokens.push(Token::RParen),
            _ => {
                if word == "\"" || word == "\'" {
                }
                tokens.push(check_number_or_symbol(word.to_string()));
            }
        }
    }

    Ok(tokens)
}

fn check_number_or_symbol(word: String) -> Token {
    let i = word.parse::<isize>();
    if i.is_ok() {
        Token::Integer(i.unwrap())
    } else {
        let f = word.parse::<f64>();
        if f.is_ok() {
            Token::Float(f.unwrap())
        } else {
            Token::Symbol(word.to_string())
        }
    }
}

fn replace_quatation_by_whitespace(program: String) -> String {
    program.replace("\"", " \" ").replace("\'", " \' ")
}

fn replace_parenthese_by_whitespace(program: String) -> String {
    program.replace("(", " ( ").replace(")", " ) ")
}

#[cfg(test)]
mod test_lexer {
    use crate::lexer;
    use crate::lexer::tokenize;
    use crate::lexer::check_number_or_symbol;
    use crate::lexer::Token;

    #[test]
    fn test_float_token() {
        let program: String = "(
              (define i 3.0)
            )".to_string();
        let tokens: Vec<Token> = tokenize(program).unwrap_or(vec![]);

        assert_eq!(
            tokens,
            vec![
                Token::LParen,

                Token::LParen,
                Token::Symbol("define".to_string()),
                Token::Symbol("i".to_string()),
                Token::Float(3.0),
                Token::RParen,

                Token::RParen,
            ]
        );
    }

    #[test]
    fn test_string_token_printing() {
        let program: String = "hi haruki!".to_string();
        println!("{}", Token::String(program));
    }

    #[test]
    fn test_quatation_literal() {
        let program: String = "(
              (define i \"hoge haruki\")
            )".to_string();
        let tokens = tokenize(program).unwrap_or(vec![]);

        assert_eq!(
            tokens,
            vec![
                Token::LParen,
                Token::LParen,
                Token::Symbol("define".to_string()),
                Token::Symbol("i".to_string()),
                Token::String("hoge haruki".to_string()),
                Token::RParen,
                Token::RParen,
            ]
        );
    }

    #[test]
    fn test_check_integer() {
        let word: String = "haruki".to_string();
        let token: Token = check_number_or_symbol(word);
        assert_eq!(token, Token::Symbol("haruki".to_string()));
    }

    #[test]
    fn test_add() {
        let tokens = lexer::tokenize("(+ 1 2)".to_string()).unwrap_or(vec![]);

        assert_eq!(
            tokens,
            vec![
                Token::LParen,
                Token::Symbol("+".to_string()),
                Token::Integer(1),
                Token::Integer(2),
                Token::RParen,
            ]
        );
    }

    #[test]
    fn test_area_of_a_circle() {
        let program: String = "
        (
          (define r 10)
          (define pi 314)
          (* pi (* r r))
        )
        ".to_string();
        let tokens = lexer::tokenize(program).unwrap_or(vec![]);

        assert_eq!(
            tokens,
            vec![
                Token::LParen,
                Token::LParen,
                Token::Symbol("define".to_string()),
                Token::Symbol("r".to_string()),
                Token::Integer(10),
                Token::RParen,
                Token::LParen,
                Token::Symbol("define".to_string()),
                Token::Symbol("pi".to_string()),
                Token::Integer(314),
                Token::RParen,
                Token::LParen,
                Token::Symbol("*".to_string()),
                Token::Symbol("pi".to_string()),
                Token::LParen,
                Token::Symbol("*".to_string()),
                Token::Symbol("r".to_string()),
                Token::Symbol("r".to_string()),
                Token::RParen,
                Token::RParen,
                Token::RParen,
            ]
        );
    }
}
