#[derive(Debug, PartialEq, Eq)]
pub enum Type {
    Atom(Atom),
    List(Vec<Type>),
    Variable(Box<Type>),
    Lambda((Vec<Type>, Box<Action>)),
    BuiltinFunction(BuiltinFunction),
}

#[derive(Debug, PartialEq, Eq)]
pub enum BuiltinFunction {
    Def,
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

impl std::str::FromStr for BuiltinFunction {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "def" => Ok(BuiltinFunction::Def),
            v => anyhow::bail!("EVAL ERROR: Cannot find {} for BuiltinFunction", v),
        }
    }
}
