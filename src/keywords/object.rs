#[derive(Debug, Clone, PartialEq)]
pub enum Object {
    Void,
    Integer(isize),
    Float(f64),
    Bool(bool),
    Symbol(String),
    String(String),
    Lambda(Vec<String>, Vec<Object>),
    List(Vec<Object>),
}

impl std::fmt::Display for Object {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Object::Void => write!(f, "Void"),
            Object::Integer(number) => write!(f, "{}", number),
            Object::Float(number) => write!(f, "{}", number),
            Object::Bool(boolean) => write!(f, "{}", boolean),
            Object::Symbol(symbol) => write!(f, "{}", symbol),
            Object::String(string) => write!(f, "{}", string),
            Object::Lambda(parameters, body) => {
                write!(f, "Lambda(")?;
                for parameter in parameters {
                    write!(f, "{}", parameter)?;
                }
                write!(f, ")")?;

                for expression in body {
                    write!(f, "{}", expression)?;
                }
                Ok(())
            }

            Object::List(list) => {
                write!(f, "(")?;

                for (i, obj) in list.iter().enumerate() {
                    if i > 0 {
                        write!(f, "")?;
                    }
                    write!(f, "{}", obj)?;
                }

                write!(f, ")")
            }
        }
    }
}
