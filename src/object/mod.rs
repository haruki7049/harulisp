use std::fmt;

enum Object {
    Void,
    Integer(isize),
    Bool(bool),
    Symbol(String),
    Lambda(Vec<String>, Vec<Object>),
    List(Vec<Object>),
}

impl fmt::Display for Object {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Object::Void => write!(f, "Void"),
            Object::Integer(n) => write!(f, "{}", n),
            Object::Bool(b) => write!(f, "{}", b),
            Object::Symbol(s) => write!(f, "{}", s),
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
