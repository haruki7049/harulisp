use crate::parser;
use crate::parser::{Program, Token};
use anyhow::Context as _;
use std::collections::HashMap;

/// Evaluates a String as Harulisp
pub fn eval(str: String) -> anyhow::Result<()> {
    let program: Program = match parser::parse(str.as_str()) {
        Ok(v) => v,
        Err(e) => anyhow::bail!(e),
    };

    HarulispMachine::load(program)?.run()?;

    Ok(())
}

#[derive(Debug)]
struct HarulispMachine {
    variables: HashMap<String, Value>,
    entrypoint: String,
}

impl Machine for HarulispMachine {
    /// Run Harulisp
    fn run(&self) -> anyhow::Result<()> {
        Ok(())
    }

    /// Load scripts
    fn load(program: Program) -> anyhow::Result<HarulispMachine> {
        let mut machine: HarulispMachine = HarulispMachine {
            variables: HashMap::new(),
            entrypoint: String::from("main"),
        };

        for token in program.statements {
            match token {
                Token::SExpression(s) => {
                    let (n, v): (String, Value) = eval_sexpr(s)?;
                    machine.append(n, v);
                }
                _ => {}
            }
        }

        Ok(machine)
    }

    /// Append variables
    fn append(&mut self, name: String, value: Value) {
        &self.variables.insert(name, value);
    }
}

fn eval_sexpr(mut tokens: Vec<Token>) -> anyhow::Result<(String, Value)> {
    // Reverse tokens
    tokens.reverse();

    let function: String = tokens
        .pop()
        .context("EVAL ERROR: No function name to construct SExpression")?
        .to_string();

    let value: String = tokens
        .pop()
        .context("EVAL ERROR: No value to construct SExpression")?;

    todo!();
    //Ok((function, value))
}

trait Machine {
    /// Run Harulisp
    fn run(&self) -> anyhow::Result<()>;

    /// Load scripts
    fn load(program: Program) -> anyhow::Result<HarulispMachine>;

    /// Append variables
    fn append(&mut self, name: String, value: Value);
}

#[derive(Debug, PartialEq, Eq)]
enum Value {
    String(String),
    Int(i32),
    List(List),
    SExpression(Statement),
}

impl std::convert::From<Token> for Value {
    fn from(t: Token) -> Self {
        match t {
            Token::String(s) => Value::String(s),
            Token::Int(i) => Value::Int(i),
            Token::Word(_) | Token::SExpression(_) | Token::List(_) => todo!(),
        }
    }
}

type List = Vec<Value>;
type Statements = Vec<Statement>;

#[derive(Debug, PartialEq, Eq)]
enum Statement {
    Def((String, Box<Value>)),
    Progn(Statements),
    Println(Box<Value>),
    Lambda((List, Statements)),
}

impl std::str::FromStr for Statement {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "def" => Ok(Statement::Def((
                String::new(),
                Box::new(Value::String(String::new())),
            ))),
            "progn" => Ok(Statement::Progn(vec![])),
            "println" => Ok(Statement::Println(Box::new(Value::String(String::new())))),
            "lambda" => Ok(Statement::Lambda((vec![], vec![]))),
            _ => anyhow::bail!("EVAL ERROR: Failed to parse function word"),
        }
    }
}

/// This implementation generates the following Statement:
/// Statement::Lambda((vec![], vec![]))
/// (lambda '() ())
impl Default for Statement {
    fn default() -> Self {
        Statement::Lambda((vec![], vec![]))
    }
}

#[cfg(test)]
mod tests {
    use crate::eval::{Statement, Value};

    #[test]
    fn statement_fromstr() -> anyhow::Result<()> {
        assert_eq!(
            "def".parse::<Statement>()?,
            Statement::Def((String::new(), Box::new(Value::String(String::new()))))
        );
        assert_eq!("progn".parse::<Statement>()?, Statement::Progn(vec![]));
        assert_eq!(
            "println".parse::<Statement>()?,
            Statement::Println(Box::new(Value::String(String::new())))
        );
        assert_eq!(
            "lambda".parse::<Statement>()?,
            Statement::Lambda((vec![], vec![]))
        );
        Ok(())
    }
}
