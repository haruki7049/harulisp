#[derive(Debug, PartialEq, Eq)]
pub enum Type {
    Atom(Atom),
    List(Vec<Type>),
    Variable(Box<Type>),
    IO(Box<Action>),
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

impl std::fmt::Display for Atom {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Atom::String(s) => write!(f, "{}", s),
            Atom::Int(i) => write!(f, "{}", i),
        }
    }
}
