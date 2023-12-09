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

    Define,
    If,

    Plus,
    Minus,
    Asterisk,
    Slash,

    Greater,
    Shorter,
    Equal,
    NotEqual,

    List(Vec<Object>),
    Lambda(Vec<String>, Vec<Object>),
}

impl std::fmt::Display for Object {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Object::Void => write!(f, "Void"),
            Object::Integer(integer) => write!(f, "{}", integer),
            Object::Bool(boolean) => write!(f, "{}", boolean),
            Object::Float(float) => write!(f, "{}", float),
            Object::String(string) => write!(f, "{}", string),

            Object::Define => write!(f, "{}", DEFINE),
            Object::If => write!(f, "{}", IF),

            Object::Plus => write!(f, "{}", PLUS),
            Object::Minus => write!(f, "{}", MINUS),
            Object::Asterisk => write!(f, "{}", ASTERISK),
            Object::Slash => write!(f, "{}", SLASH),

            Object::Shorter => write!(f, "{}", SHORTER),
            Object::Greater => write!(f, "{}", GREATER),
            Object::Equal => write!(f, "{}", EQUAL),
            Object::NotEqual => write!(f, "{}", NOT_EQUAL),

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
