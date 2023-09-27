use crate::data::tokens::Token;
use crate::data::objects::Object;
use crate::lexer::tokenize;

fn parse(program: &str) -> Result<Object, ParseError> {
    let token_result = tokenize(program);
    if token_result.is_err() {
        return Err(ParseError {
            err: format!("{}", token_result.err().unwrap()),
        });
    }

    let mut tokens = token_result.unwrap()
        .into_iter()
        .rev()
        .collect::<Vec<_>>();
    let parsed_list = parse_list(&mut tokens)?;
}

fn parse_list(tokens: &mut Vec<Token>) -> Result<Object, ParseError> {
    let token = tokens.pop();
    if tokens != Some(Token::LParen) {
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
            Token::Integer(integer) => list.push(Object::Integer(integer)),
            //Token::String(boolean) => list.push(Object::Bool(boolean)),
            // TODO: other match patern about Token to Object
        }
    }

    Ok(Object::List(list))
}

impl std::error::Error for ParseError {}

#[derive(Debug)]
pub struct ParseError {
    err: String,
}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Parse error: {}", self.err)
    }
}

#[cfg(test)]
mod test {
}
