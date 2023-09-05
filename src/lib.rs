mod input {
    #[derive(PartialEq)]
    enum TerminalStatement {
        REPL,
        //Stdout,
    }

    #[allow(dead_code)]
    fn check_in_repl(terminal_statement: TerminalStatement) -> bool {
        if terminal_statement == TerminalStatement::REPL {
            true
        } else {
            false
        }
    }
}

mod lexer {
    use std::fmt;

    #[derive(Debug, PartialEq)]
    #[allow(dead_code)]
    pub enum Token {
        Integer(isize),
        Symbol(String),
        LParen,
        RParen,
    }

    impl fmt::Display for Token {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            match self {
                Token::Integer(n) => write!(f, "{}", n),
                Token::Symbol(s) => write!(f, "{}", s),
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

    fn tokenize(program: &str) -> Result<Vec<Token>, TokenError> {
        let p = program.replace("(", " ( ").replace(")", " ) ");
        let words = p.split_whitespace();
        let mut tokens: Vec<Token> = Vec::new();

        for word in words {
            match word {
                "(" => tokens.push(Token::LParen),
                ")" => tokens.push(Token::RParen),
                _ => {
                    let i = word.parse::<isize>();
                    if i.is_ok() {
                        tokens.push(Token::Integer(i.unwrap()));
                    } else {
                        tokens.push(Token::Symbol(word.to_string()));
                    }
                }
            }
        }

        Ok(tokens)
    }

    #[cfg(test)]
    mod test_lexer {
        use crate::lexer;
        use crate::lexer::Token;

        #[test]
        fn test_add() {
            let tokens = lexer::tokenize("(+ 1 2)").unwrap_or(vec![]);

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
            let program: &str = "
            (
              (define r 10)
              (define pi 314)
              (* pi (* r r))
            )
            ";
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
}

mod object {
    use std::fmt;

    enum Object {
        Void,
        Integer(isize),
        Bool(bool),
        Symbol(String),
        Lambda(Vec<String>, Vec<Object>),
        List(Vec<Object>),
    }

    impl fmt::Display for Object {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            match self {
                Object::Void => write!(f, "Void"),
                Object::Integer(n) => write!(f, "{}", n),
                Object::Bool(b) => write!(f, "{}", b),
                Object::Symbol(s) => write!(f, "{}", s),
                Object::Lambda(parameters, body) => {
                    write!(f, "Lambda(")?;
                    for parameter in parameters {
                        write!(f, "{}", parameter)?;
                    }
                    write!(f, ")")?;

                    for expression in body {
                        write!(f, "{}", expression)?;
                    }
                    Ok(())
                },

                Object::List(list) => {
                    write!(f, "(")?;

                    for (i, obj) in list.iter().enumerate() {
                        if i > 0 {
                            write!(f, "")?;
                        }
                        write!(f, "{}", obj)?;
                    }

                    write!(f, ")")
                }
            }
        }
    }
}
