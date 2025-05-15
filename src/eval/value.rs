use crate::parser::Token;
use std::ops::Deref;

#[derive(Debug, PartialEq, Eq)]
pub enum Type {
    Atom(Atom),
    IO,
    List(Vec<Type>),
    Variable(String),
    BuiltinFunction(BuiltinFunction),
}

#[derive(Debug, PartialEq, Eq)]
pub enum BuiltinFunction {
    Def,
    Lambda,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Atom {
    String(String),
    Int(i32),
}

#[derive(Debug, PartialEq, Eq)]
/// Action
/// HarulispMachine reads the entrypoint's `Action`
pub enum Action {
    Print(Atom),
    Progn(Vec<Action>),
}

impl TryFrom<&Token> for Type {
    type Error = anyhow::Error;

    fn try_from(v: &Token) -> Result<Self, Self::Error> {
        let t: Token = v.clone();

        match t {
            Token::String(s) => Ok(Type::Atom(Atom::String(s.clone()))),
            Token::Int(i) => Ok(Type::Atom(Atom::Int(i))),
            Token::List(l) => todo!(),
            Token::Word(w) => match w.as_str() {
                "def" => Ok(Type::BuiltinFunction(BuiltinFunction::Def)),
                "lambda" => Ok(Type::BuiltinFunction(BuiltinFunction::Lambda)),
                s => Ok(Type::Variable(s.to_string())),
            },
            Token::SExpression(mut sexpr) => {
                let function: Token = sexpr.pop().unwrap();
                match function {
                    Token::Word(s) => todo!(),
                    Token::SExpression(mut sexpr) => {
                        let function: Token = sexpr.pop().unwrap();
                        dbg!(&function);

                        match function {
                            Token::SExpression(mut sexpr) => {
                                dbg!(&sexpr);

                                match function {
                                    _ => todo!(),
                                }
                            }
                            _ => todo!(),
                        }
                    }
                    v => {
                        dbg!(v);
                        unreachable!()
                    }
                }

                todo!();
            }
        }
    }
}

impl std::fmt::Display for Atom {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Atom::String(s) => write!(f, "{}", s),
            Atom::Int(i) => write!(f, "{}", i),
        }
    }
}

impl std::str::FromStr for BuiltinFunction {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "def" => Ok(BuiltinFunction::Def),
            v => anyhow::bail!("EVAL ERROR: Cannot find {} for BuiltinFunction", v),
        }
    }
}
