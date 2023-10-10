pub const IF: &str = "if";
pub const DEFINE: &str = "define";
pub const LAMBDA: &str = "lambda";

pub const PLUS: &str = "+";
pub const MINUS: &str = "-";
pub const ASTERISK: &str = "*";
pub const SLASH: &str = "/";

pub const SHORTER: &str = "<";
pub const GREATER: &str = ">";
pub const EQUAL: &str = "=";
pub const NOT_EQUAL: &str = "!=";

#[derive(Debug, Clone, PartialEq)]
pub enum Object {
    Void,
    Integer(isize),
    Bool(bool),
    Float(f64),
    String(String),
    Symbol(Symbol),
    List(Vec<Object>),
    Lambda(Vec<String>, Vec<Object>),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Symbol {
    Define,
    If,
    Lambda,
    Operator(Operator),
    Comparison(Comparison),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Operator {
    Plus,
    Minus,
    Asterisk,
    Slash,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Comparison {
    Shorter,
    Greater,
    Equal,
    NotEqual,
}

impl std::fmt::Display for Object {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Object::Void => write!(f, "Void"),
            Object::Integer(integer) => write!(f, "{}", integer),
            Object::Bool(boolean) => write!(f, "{}", boolean),
            Object::Float(float) => write!(f, "{}", float),
            Object::String(string) => write!(f, "{}", string),
            Object::Symbol(symbol) => write!(f, "{}", symbol),
            Object::Lambda(parameters, body) => {
                write!(f, "Lambda(")?;

                for parameter in parameters {
                    write!(f, "{} ", parameter)?;
                }

                for expression in body {
                    write!(f, " {}", expression)?;
                }

                Ok(())
            }
            Object::List(list) => {
                write!(f, "(")?;
                for (i, object) in list.iter().enumerate() {
                    if i > 0 {
                        write!(f, " ")?;
                    }
                    write!(f, "{}", object)?;
                }
                write!(f, ")")
            }
        }
    }
}

impl std::fmt::Display for Symbol {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Symbol::Define => write!(f, "{}", DEFINE),
            Symbol::If => write!(f, "{}", IF),
            Symbol::Lambda => write!(f, "{}", LAMBDA),
            Symbol::Operator(operator) => write!(f, "{}", operator),
            Symbol::Comparison(comparison) => write!(f, "{}", comparison),
        }
    }
}

impl std::fmt::Display for Operator {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Operator::Plus => write!(f, "{}", PLUS),
            Operator::Minus => write!(f, "{}", MINUS),
            Operator::Asterisk => write!(f, "{}", ASTERISK),
            Operator::Slash => write!(f, "{}", SLASH),
        }
    }
}

impl std::fmt::Display for Comparison {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Comparison::Shorter => write!(f, "{}", SHORTER),
            Comparison::Greater => write!(f, "{}", GREATER),
            Comparison::Equal => write!(f, "{}", EQUAL),
            Comparison::NotEqual => write!(f, "{}", NOT_EQUAL),
        }
    }
}
