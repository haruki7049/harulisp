use crate::data::objects::{
    Comparison, Object, Operator, Symbol, ASTERISK, DEFINE, EQUAL, GREATER, IF, LAMBDA, MINUS,
    NOT_EQUAL, PLUS, SHORTER, SLASH,
};
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

#[derive(Debug, PartialEq, Default)]
pub struct Env {
    parent: Option<Rc<RefCell<Env>>>,
    variables: HashMap<String, Object>,
}

impl Env {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn extend(parent: Rc<RefCell<Self>>) -> Env {
        Env {
            variables: HashMap::new(),
            parent: Some(parent),
        }
    }

    pub fn get(&self, name: &str) -> Option<Object> {
        match self.variables.get(name) {
            Some(value) => Some(value.clone()),
            None => self
                .parent
                .as_ref()
                .and_then(|o| o.borrow().get(name).clone()),
        }
    }

    pub fn set(&mut self, name: &str, value: Object) {
        self.variables.insert(name.to_string(), value);
    }
}

/// get_string_from_symbol function which is convert Symbol to String
pub fn convert_string_from_symbol(symbol: &Symbol) -> String {
    match symbol {
        Symbol::Define => String::from(DEFINE),
        Symbol::If => String::from(IF),
        Symbol::Lambda => String::from(LAMBDA),
        Symbol::Operator(operator) => match operator {
            Operator::Plus => String::from(PLUS),
            Operator::Minus => String::from(MINUS),
            Operator::Asterisk => String::from(ASTERISK),
            Operator::Slash => String::from(SLASH),
        },
        Symbol::Comparison(comparison) => match comparison {
            Comparison::Shorter => String::from(SHORTER),
            Comparison::Greater => String::from(GREATER),
            Comparison::Equal => String::from(EQUAL),
            Comparison::NotEqual => String::from(NOT_EQUAL),
        },
    }
}
