#[derive(Debug, PartialEq)]
pub enum Object {
    Void,
    Integer(isize),
    Bool(bool),
    Float(f64),
    String(String),
    Symbol(String),
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
