use crate::data::env::Env;
use crate::data::objects::{
    Object,
    Symbol,
};
use crate::parser::parse;
use std::cell::RefCell;
use std::rc::Rc;

fn evaluate(program: &str, env: &mut Rc<RefCell<Env>>) -> Result<Object, String> {
    let parsed_list = parse(program);
    if parsed_list.is_err() {
        return Err(format!("{}", parsed_list.err().unwrap()));
    }
    evaluate_object(&parsed_list.unwrap(), env)
}

fn evaluate_object(object: &Object, env: &mut Rc<RefCell<Env>>) -> Result<Object, String> {
    match object {
        Object::List(list) => evaluate_list(list, env),
        Object::Symbol(symbol) => evaluate_symbol(symbol, env),
        Object::Void => Ok(Object::Void),
        Object::Lambda(_parameters, _body) => Ok(Object::Void),
        Object::Bool(_) => Ok(object.clone()),
        Object::Integer(integer) => Ok(Object::Integer(*integer)),
        Object::Float(float) => Ok(Object::Float(*float)),
        Object::String(string) => Ok(Object::String(*string)),
    }
}

fn evaluate_symbol(symbol: &Symbol, env: &mut Rc<RefCell<Env>>) -> Result<Object, String> {
    let val = env.borrow_mut().get(symbol); // TODO: convert from Object::Symbol to &str
    if val.is_none() {
        return Err(format!("Unbound symbol: {}", symbol));
    }
    Ok(val.unwrap().clone())
}

fn evaluate_list(list: &Vec<Object>) -> Result<Object, String> {
    let head = &list[0];
    match head {
        Object::Symbol(symbol) => match symbol.as_str() {
        }
        _ => {
            let mut new_list = Vec::new();
            Ok(Object::List(new_list))
        }
    }
}
