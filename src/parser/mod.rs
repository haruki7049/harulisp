use crate::lexer::tokenize;
use crate::keywords::token::Token;
use crate::keywords::object::Object;
use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub struct ParseError {
    err: String,
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Parse error: {}", self.err)
    }
}

impl Error for ParseError {}

pub fn parse(program: String) -> Result<Object, ParseError> {
    let token_result = tokenize(program);
    if token_result.is_err() {
        return Err(ParseError {
            err: format!("{}", token_result.err().unwrap()),
        });
    }

    let mut tokens = token_result.unwrap().into_iter().rev().collect::<Vec<_>>();
    let parsed_list = parse_list(&mut tokens)?;
    Ok(parsed_list)
}

fn parse_list(tokens: &mut Vec<Token>) -> Result<Object, ParseError> {
    let token = tokens.pop();
    if token != Some(Token::LParen) {
        return Err(ParseError {
            err: format!("Expected LParen, found {:?}", token),
        });
    }

    let mut list: Vec<Object> = Vec::new();
    while !tokens.is_empty() {
        let token = tokens.pop();
        if token == None {
            return Err(ParseError {
                err: format!("Did not find enough tokens"),
            });
        }

        let t = token.unwrap();
        match t {
            Token::Integer(number) => list.push(Object::Integer(number)),
            Token::Float(number) => list.push(Object::Float(number)),
            Token::Symbol(symbol) => list.push(Object::Symbol(symbol)),
            Token::String(string) => list.push(Object::String(string)),
            Token::LParen => {
                tokens.push(Token::LParen);
                let sub_list = parse_list(tokens)?;
                list.push(sub_list);
            }
            Token::RParen => {
                return Ok(Object::List(list));
            }
        }
    }

    Ok(Object::List(list))
}

#[cfg(test)]
mod test_parser {
    use crate::keywords::object::Object;
    use crate::parser::parse;

    #[test]
    fn test_parse() {
        let list = parse("(+ 1 2)".to_string()).unwrap();
        assert_eq!(
            list,
            Object::List(vec![
                Object::Symbol("+".to_string()),
                Object::Integer(1),
                Object::Integer(2),
            ])
        );
    }

    #[test]
    fn test_parse_multi_sentence() {
        let program = "(
            (define r 10)
            (define pi 314)
            (* pi (* r r))
            )";
        let list = parse(program.to_string()).unwrap();
        assert_eq!(
            list,
            Object::List(vec![
                Object::List(vec![
                    Object::Symbol("define".to_string()),
                    Object::Symbol("r".to_string()),
                    Object::Integer(10),
                ]),
                Object::List(vec![
                    Object::Symbol("define".to_string()),
                    Object::Symbol("pi".to_string()),
                    Object::Integer(314),
                ]),
                Object::List(vec![
                    Object::Symbol("*".to_string()),
                    Object::Symbol("pi".to_string()),
                    Object::List(vec![
                        Object::Symbol("*".to_string()),
                        Object::Symbol("r".to_string()),
                        Object::Symbol("r".to_string()),
                    ]),
                ]),
            ])
        );
    }
}
