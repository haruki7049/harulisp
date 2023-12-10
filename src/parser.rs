use crate::data::objects::{
    Object, ASTERISK, DEFINE, EQUAL, GREATER, IF, MINUS, NOT_EQUAL, PLUS, SHORTER, SLASH,
};
use crate::data::tokens::Token;
use crate::data::tokens::TokenError;
use crate::lexer::tokenize;

/// parse function, return Result<Object, ParseError>.
pub fn parse(program: &str) -> Result<Object, ParseError> {
    let token_result: Result<Vec<Token>, TokenError> = tokenize(program);
    if token_result.is_err() {
        return Err(ParseError {
            err: format!("{}", token_result.err().unwrap()),
        });
    }

    let mut tokens: Vec<Token> = token_result
        .unwrap()
        .into_iter()
        .rev()
        .collect::<Vec<Token>>();
    let parsed_list: Object = parse_list(&mut tokens)?;
    Ok(parsed_list)
}

/// parse token's vector to Object, if this failed return Err(ParseError).
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
        if token.is_none() {
            return Err(ParseError {
                err: "Did not find enough tokens".to_string(),
            });
        }

        let t = token.unwrap();
        match t {
            Token::Integer(integer) => list.push(Object::Integer(integer)),
            Token::Float(float) => list.push(Object::Float(float)),
            Token::String(string) => list.push(parse_string(string)),
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

/// parse String to Object, if there is no symbol return Raw string wrapped by Object.
fn parse_string(string: String) -> Object {
    match &string[..] {
        DEFINE => Object::Define,
        IF => Object::If,

        PLUS => Object::Plus,
        MINUS => Object::Minus,
        ASTERISK => Object::Asterisk,
        SLASH => Object::Slash,

        SHORTER => Object::Shorter,
        GREATER => Object::Greater,
        EQUAL => Object::Equal,
        NOT_EQUAL => Object::NotEqual,

        _ => Object::String(string),
    }
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

/// Parser test
#[cfg(test)]
mod test {
    use crate::data::objects::Object;
    use crate::parser::parse;

    /// test for not recursed list, whether parse function is correctly return normal list.
    #[test]
    fn test_simple_add() {
        const PROGRAM: &str = "(+ 1 2)";
        let list = parse(PROGRAM).unwrap();
        assert_eq!(
            list,
            Object::List(vec![Object::Plus, Object::Integer(1), Object::Integer(2),])
        );
    }

    /// test for recursive list, whether parse function is correctly ignore new_line and whitespace or not, and correctly return recursed list or not.
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
                    Object::Define,
                    Object::String("x".to_string()),
                    Object::Integer(12),
                ]),
                Object::List(vec![
                    Object::Define,
                    Object::String("y".to_string()),
                    Object::Integer(3),
                ]),
                Object::List(vec![
                    Object::Plus,
                    Object::String("x".to_string()),
                    Object::String("y".to_string()),
                ]),
            ])
        );
    }

    /// capital character test, whether this process is correctly run or not. the test program is mixed CAPITAL charactor in define, as DeFinE.
    #[should_panic]
    #[test]
    fn test_capital_character() {
        const PROGRAM: &str = "(
          (DeFinE x 23)
        )";
        let list = parse(PROGRAM).unwrap();
        assert_eq!(
            list,
            Object::List(vec![
                Object::Define,
                Object::String("x".to_string()),
                Object::Integer(23),
            ]),
        );
    }

    #[should_panic]
    #[test]
    fn no_parentheses() {
        const PROGRAM: &str = "define x 15";
        let _list = parse(PROGRAM).unwrap();
    }
}
