#[derive(Debug, Default, PartialEq, Eq)]
pub struct Program {
    statements: Vec<Token>,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Token {
    String(String),
    Word(String),
    Int(i32),
    SExpression(Vec<Token>),
    List(Vec<Token>),
}

pub fn parse(s: &str) -> anyhow::Result<Program> {
    todo!()
}

#[cfg(test)]
mod tests {
    use crate::parser;
    use crate::parser::{Program, Token};

    #[test]
    fn parse_sexpr() -> anyhow::Result<()> {
        let result = parser::parse("(hoge fuga)")?;

        assert_eq!(
            result,
            Program {
                statements: vec![Token::SExpression(vec![
                    Token::Word(String::from("hoge")),
                    Token::Word(String::from("fuga")),
                ])]
            }
        );
        Ok(())
    }

    #[test]
    fn parse_main_func() -> anyhow::Result<()> {
        let result =
            parser::parse("(def main (lambda '() (println \"This is the example text. FOO!!\")))")?;

        assert_eq!(
            result,
            Program {
                statements: vec![Token::SExpression(vec![
                    Token::Word(String::from("def")),
                    Token::Word(String::from("main")),
                    Token::SExpression(vec![
                        Token::Word(String::from("lambda")),
                        Token::List(vec![]),
                        Token::SExpression(vec![
                            Token::Word(String::from("println")),
                            Token::String(String::from("\"This is the example text. FOO!!\"")),
                        ])
                    ])
                ])]
            }
        );
        Ok(())
    }
}
