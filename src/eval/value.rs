#[derive(Debug, PartialEq, Eq)]
pub enum Value {
    String(String),
    Int(i32),
    List(Vec<Value>),
    IO(Box<Action>),
    Variable(Box<Value>),
}

#[derive(Debug, PartialEq, Eq)]
/// Action
/// HarulispMachine reads the entrypoint's `Vec<Action>`
pub enum Action {
    Print(Value),
    Progn(Vec<Action>),
}

impl Value {
    pub fn is_atom(&self) -> bool {
        match self {
            Value::String(_) => true,
            Value::Int(_) => true,
            Value::List(_) => false,
            Value::IO(_) => false,
            Value::Variable(_) => false,
        }
    }
}
