use crate::data::objects::Object;
use crate::data::tokens::Token;
use crate::data::tokens::TokenError;
use crate::lexer::tokenize;

pub fn parse(program: &str) -> Result<Object, ParseError> {
    let token_result: Result<Vec<Token>, TokenError> = tokenize(program);
    if token_result.is_err() {
        return Err(ParseError {
            err: format!("{}", token_result.err().unwrap()),
        });
    }

    let mut tokens: Vec<Token> = token_result.unwrap().into_iter().rev().collect::<Vec<Token>>();
    let parsed_list: Object = parse_list(&mut tokens)?;
    Ok(parsed_list)
}

fn parse_list(tokens_vec: &mut Vec<Token>) -> Result<Object, ParseError> {
    let t = tokens_vec.pop();
    if t != Some(Token::LParen) {
        return Err(ParseError {
            err: format!("Expected LParen, found {:?}", t),
        });
    }

    let mut list: Vec<Object> = Vec::new();
    while !tokens_vec.is_empty() {
        let token = tokens_vec.pop();
        if token == None {
            return Err(ParseError {
                err: format!("Did not find enough tokens"),
            });
        }

        let t = token.unwrap();
        match t {
            Token::Integer(integer) => list.push(Object::Integer(integer)),
            Token::Float(float) => list.push(Object::Float(float)),
            Token::String(string) => list.push(Object::String(string)),
            Token::LParen => {
                tokens_vec.push(Token::LParen);
                let sub_list = parse_list(tokens_vec)?;
                list.push(sub_list);
            }
            Token::RParen => {
                return Ok(Object::List(list));
            }
        }
    }

    Ok(Object::List(list))
}

#[derive(Debug)]
pub struct ParseError {
    err: String,
}

impl std::error::Error for ParseError {}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Parse error: {}", self.err)
    }
}

#[cfg(test)]
mod test {
    use crate::data::objects::Object;
    use crate::parser::parse;

    #[test]
    fn test_simple_add() {
        const PROGRAM: &str = "(+ 1 2)";
        let list = parse(PROGRAM).unwrap();
        assert_eq!(
            list,
            Object::List(vec![
                Object::String("+".to_string()),
                Object::Integer(1),
                Object::Integer(2),
            ])
        );
    }

    #[test]
    fn test_recursive_list() {
        const PROGRAM: &str = "(
            (define x 12)
            (define y 3)
            (+ x y)
        )";
        let list = parse(PROGRAM).unwrap();
        assert_eq!(
            list,
            Object::List(vec![
                Object::List(vec![
                    Object::String("define".to_string()),
                    Object::String("x".to_string()),
                    Object::Integer(12),
                ]),
                Object::List(vec![
                    Object::String("define".to_string()),
                    Object::String("y".to_string()),
                    Object::Integer(3),
                ]),
                Object::List(vec![
                    Object::String("+".to_string()),
                    Object::String("x".to_string()),
                    Object::String("y".to_string()),
                ]),
            ])
        );
    }

    #[test]
    fn test_capital_character() {
        const PROGRAM: &str = "(
          (Define x 23)
        )";
        let list = parse(PROGRAM).unwrap();
        assert_eq!(
            list,
            Object::List(vec![Object::List(vec![
                Object::String("Define".to_string()),
                Object::String("x".to_string()),
                Object::Integer(23),
            ])])
        );
    }
}
