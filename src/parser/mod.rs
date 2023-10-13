use crate::data::objects::{
    Object,
    Symbol,
    Operator,
    Comparison,
};
use crate::data::objects::{
    DEFINE,
    IF,
    LAMBDA,
    PLUS,
    MINUS,
    ASTERISK,
    SLASH,
    SHORTER,
    GREATER,
    EQUAL,
    NOT_EQUAL,
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

    let mut tokens: Vec<Token> = token_result.unwrap().into_iter().rev().collect::<Vec<Token>>();
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
        if token == None {
            return Err(ParseError {
                err: format!("Did not find enough tokens"),
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
        DEFINE => Object::Symbol(Symbol::Define),
        IF => Object::Symbol(Symbol::If),
        LAMBDA => Object::Symbol(Symbol::Lambda),

        PLUS => Object::Symbol(Symbol::Operator(Operator::Plus)),
        MINUS => Object::Symbol(Symbol::Operator(Operator::Minus)),
        ASTERISK => Object::Symbol(Symbol::Operator(Operator::Plus)),
        SLASH => Object::Symbol(Symbol::Operator(Operator::Plus)),
        
        SHORTER => Object::Symbol(Symbol::Comparison(Comparison::Shorter)),
        GREATER => Object::Symbol(Symbol::Comparison(Comparison::Greater)),
        EQUAL => Object::Symbol(Symbol::Comparison(Comparison::Equal)),
        NOT_EQUAL => Object::Symbol(Symbol::Comparison(Comparison::NotEqual)),
        
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
    use crate::data::objects::{
        Object,
        Symbol,
        Operator,
    };
    use crate::parser::parse;

    /// test for not recursed list, whether parse function is correctly return normal list.
    #[test]
    fn test_simple_add() {
        const PROGRAM: &str = "(+ 1 2)";
        let list = parse(PROGRAM).unwrap();
        assert_eq!(
            list,
            Object::List(vec![
                Object::Symbol(Symbol::Operator(Operator::Plus)),
                Object::Integer(1),
                Object::Integer(2),
            ])
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
                    Object::Symbol(Symbol::Define),
                    Object::String("x".to_string()),
                    Object::Integer(12),
                ]),
                Object::List(vec![
                    Object::Symbol(Symbol::Define),
                    Object::String("y".to_string()),
                    Object::Integer(3),
                ]),
                Object::List(vec![
                    Object::Symbol(Symbol::Operator(Operator::Plus)),
                    Object::String("x".to_string()),
                    Object::String("y".to_string()),
                ]),
            ])
        );
    }

    /// capital character test, whether this process is correctly run or not. the program is mixed CAPITAL charactor in define, as DeFinE.
    #[test]
    fn test_capital_character() {
        const PROGRAM: &str = "(
          (DeFinE x 23)
        )";
        let list = parse(PROGRAM).unwrap();
        assert_eq!(
            list,
            Object::List(vec![Object::List(vec![
                Object::String("DeFinE".to_string()),
                Object::String("x".to_string()),
                Object::Integer(23),
            ])])
        );
    }
}
