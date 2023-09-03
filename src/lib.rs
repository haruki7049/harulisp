mod input {
    #[derive(PartialEq)]
    enum TerminalStatement {
        REPL,
        //Stdout,
    }

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

    #[cfg(test)]
    mod test_lexer {
        //use super::Token;
    }
}
