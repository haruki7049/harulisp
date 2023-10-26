use crate::data::env::{convert_string_from_symbol, Env};
use crate::data::objects::{Comparison, Object, Operator, Symbol};
use crate::parser::parse;
use std::cell::RefCell;
use std::rc::Rc;

/// evaluate function
pub fn evaluate(program: &str, env: &mut Rc<RefCell<Env>>) -> Result<Object, String> {
    let parsed_list = parse(program);
    if parsed_list.is_err() {
        return Err(format!("{}", parsed_list.err().unwrap()));
    }
    evaluate_object(&parsed_list.unwrap(), env)
}

/// evaluate_object function
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

/// evaluate_symbol function
fn evaluate_symbol(symbol: &Symbol, env: &mut Rc<RefCell<Env>>) -> Result<Object, String> {
    let val = env
        .borrow_mut()
        .get(convert_string_from_symbol(symbol).as_str());
    if val.is_none() {
        return Err(format!("Unbound symbol: {}", symbol));
    }
    Ok(val.unwrap().clone())
}

/// evaluate_list function
fn evaluate_list(list: &Vec<Object>, env: &mut Rc<RefCell<Env>>) -> Result<Object, String> {
    let head = &list[0];
    match head {
        Object::Symbol(symbol) => match symbol {
            Symbol::Define => evaluate_define(&list, env),
            Symbol::If => evaluate_if(&list, env),
            Symbol::Lambda => evaluate_function_definition(&list),
            Symbol::Operator(operator) => evaluate_operation(&operator, &list, env),
            Symbol::Comparison(comparison) => evaluate_comparison(&comparison, &list, env),
            _ => evaluate_function_call(&symbol, &list, env),
        },
        _ => {
            let mut new_list = Vec::new();
            Ok(Object::List(new_list))
        }
    }
}

/// evaluate_operation function
fn evaluate_operation(
    operator: &Operator,
    list: &Vec<Object>,
    env: &mut Rc<RefCell<Env>>,
) -> Result<Object, String> {
    if list.len() != 3 {
        return Err(format!("Invalid number of arguments for infix operator"));
    }

    let operator = list[0].clone();
    let left: Object = evaluate_object(&list[1].clone(), env)?;
}

fn is_integer_object(object: &Object) -> bool {
    let i: isize = object;
}

fn is_float_object(object: &Object) -> bool {
    object == Object::Float
}

/// evaluate_comparison function
fn evaluate_comparison(
    comparison: &Comparison,
    list: &Vec<Object>,
    env: &mut Rc<RefCell<Env>>,
) -> Result<Object, String> {
}

/// evaluate_function_definition function
fn evaluate_function_definition(list: &Vec<Object>) -> Result<Object, String> {
    let parameters = match &list[1] {
        Object::List(list) => {
            let mut parameters = Vec::new();
            for parameter in list {
                match parameter {
                    Object::String(string) => parameters.push(string.clone()),
                    _ => return Err(format!("Invalid lambda parameter")),
                }
            }
            parameters
        }
        _ => return Err(format!("Invalid lambda")),
    };

    let body = match &list[2] {
        Object::List(list) => list.clone(),
        _ => return Err(format!("Invalid lambda")),
    };
    Ok(Object::Lambda(parameters, body))
}

/// evaluate_define function
fn evaluate_define(list: &Vec<Object>, env: &mut Rc<RefCell<Env>>) -> Result<Object, String> {
    if list.len() != 3 {
        return Err(format!("Invalid number of arguments for define"));
    }

    let symbol = match &list[1] {
        Object::String(s) => s.clone(),
        _ => return Err(format!("Invalid define")),
    };
    let value = evaluate_object(&list[2], env)?;
    env.borrow_mut().set(&symbol, value);
    Ok(Object::Void)
}

/// evaluate_if function
fn evaluate_if(list: &Vec<Object>, env: &mut Rc<RefCell<Env>>) -> Result<Object, String> {
    if list.len() != 4 {
        return Err(format!("Invalid number of arguments for if statement"));
    }

    let cond_object = evaluate_object(&list[1], env)?;
    let cond = match cond_object {
        Object::Bool(boolean) => boolean,
        _ => return Err(format!("Condition must be a boolean")),
    };

    if cond == true {
        return evaluate_object(&list[2], env);
    } else {
        return evaluate_object(&list[3], env);
    }
}

fn evaluate_function_call(
    symbol: &Symbol,
    list: &Vec<Object>,
    env: &mut Rc<RefCell<Env>>,
) -> Result<Object, String> {
    let lambda = env
        .borrow_mut()
        .get(convert_string_from_symbol(symbol).as_str());
    if lambda.is_none() {
        return Err(format!("Unbound symbol: {}", symbol));
    }

    let function = lambda.unwrap();
    match function {
        Object::Lambda(parameters, body) => {
            let mut new_env = Rc::new(RefCell::new(Env::extend(env.clone())));
            for (i, parameter) in parameters.iter().enumerate() {
                let value: Object = evaluate_object(&list[i + 1], env)?;
                new_env.borrow_mut().set(parameter, value);
            }
            return evaluate_object(&Object::List(body), &mut new_env);
        }
        _ => return Err(format!("Not a lambda: {}", symbol)),
    }
}
