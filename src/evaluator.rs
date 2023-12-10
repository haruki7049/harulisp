use crate::{
    data::{env::Env, objects::Object},
    parser,
    parser::ParseError,
};
use std::cell::RefCell;
use std::rc::Rc;

pub fn evaluate(program: &str, env: &mut Rc<RefCell<Env>>) -> Result<Object, String> {
    let parsed_list: Result<Object, ParseError> = parser::parse(program);
    if parsed_list.is_err() {
        return Err(format!("{}", parsed_list.err().unwrap()));
    }

    evaluate_object(&parsed_list.unwrap(), env)
}

fn evaluate_object(object: &Object, env: &mut Rc<RefCell<Env>>) -> Result<Object, String> {
    match object {
        Object::Void => Ok(Object::Void),
        Object::Lambda(_param, _body) => Ok(Object::Void),
        Object::Define => Ok(Object::Void),
        Object::If => Ok(Object::Void),
        Object::Plus => Ok(Object::Void),
        Object::Minus => Ok(Object::Void),
        Object::Asterisk => Ok(Object::Void),
        Object::Slash => Ok(Object::Void),
        Object::Greater => Ok(Object::Void),
        Object::Shorter => Ok(Object::Void),
        Object::Equal => Ok(Object::Void),
        Object::NotEqual => Ok(Object::Void),
        Object::Bool(_) => Ok(object.clone()),
        Object::Integer(integer) => Ok(Object::Integer(*integer)),
        Object::Float(float) => Ok(Object::Float(*float)),
        Object::String(string) => evaluate_variable_call(string, env),
        Object::List(list) => evaluate_list(list, env),
    }
}

fn evaluate_variable_call(variable: &str, env: &mut Rc<RefCell<Env>>) -> Result<Object, String> {
    let value: Option<Object> = env.borrow_mut().get(variable);
    if value.is_none() {
        return Err(format!("Unbound variable: {}", variable));
    }

    Ok(value.unwrap().clone())
}

fn evaluate_list(list: &Vec<Object>, env: &mut Rc<RefCell<Env>>) -> Result<Object, String> {
    let head: &Object = &list[0];
    match head {
        Object::Define => evaluate_define(&list, env),
        Object::If => evaluate_if(&list, env),
        Object::Lambda(_param, _body) => evaluate_function_definition(&list),
        Object::Plus
        | Object::Minus
        | Object::Asterisk
        | Object::Slash
        | Object::Shorter
        | Object::Greater
        | Object::Equal
        | Object::NotEqual => evaluate_operator(&list, env),
        Object::String(string) => evaluate_function_call(&string, &list, env),
        _ => {
            let mut new_list: Vec<Object> = Vec::new();
            for object in list {
                let result: Object = evaluate_object(object, env)?;
                match result {
                    Object::Void => {}
                    _ => new_list.push(result),
                }
            }
            Ok(Object::List(new_list))
        }
    }
}

fn evaluate_define(list: &Vec<Object>, env: &mut Rc<RefCell<Env>>) -> Result<Object, String> {
    if list.len() != 3 {
        return Err("Invalid number of arguments for DEFINE".to_string());
    }

    let sym: String = match &list[1] {
        Object::String(string) => string.clone(),
        _ => return Err("Invalid DEFINE".to_string()),
    };
    let val: Object = evaluate_object(&list[2], env)?;

    env.borrow_mut().set(&sym, val);
    Ok(Object::Void)
}

fn evaluate_if(list: &Vec<Object>, env: &mut Rc<RefCell<Env>>) -> Result<Object, String> {
    if list.len() != 4 {
        return Err("Invalid number of arguments for IF statement".to_string());
    }

    let cond_object: Object = evaluate_object(&list[1], env)?;
    let cond: bool = match cond_object {
        Object::Bool(bool) => bool,
        _ => return Err("Condition must be a boolean".to_string()),
    };

    if cond == true {
        return evaluate_object(&list[2], env);
    } else {
        return evaluate_object(&list[3], env);
    }
}

fn evaluate_function_definition(list: &Vec<Object>) -> Result<Object, String> {
    let parameters: Vec<String> = match &list[1] {
        Object::List(list) => {
            let mut parameters: Vec<String> = Vec::new();
            for parameter in list {
                match parameter {
                    Object::String(string) => parameters.push(string.clone()),
                    _ => return Err("Invalid lambda parameter".to_string()),
                }
            }
            parameters
        }
        _ => return Err("Invalid lambda".to_string()),
    };

    let body = match &list[2] {
        Object::List(list) => list.clone(),
        _ => return Err("Invalid lambda".to_string()),
    };

    Ok(Object::Lambda(parameters, body))
}

fn evaluate_operator(list: &Vec<Object>, env: &mut Rc<RefCell<Env>>) -> Result<Object, String> {
    if list.len() != 3 {
        return Err("Invalid number of arguments for infix operator".to_string());
    }

    let operator: Object = list[0].clone();
    let left: Object = evaluate_object(&list[1].clone(), env)?;
    let right: Object = evaluate_object(&list[2].clone(), env)?;
}

fn evaluate_function_call(
    variable: &String,
    list: &Vec<Object>,
    env: &mut Rc<RefCell<Env>>,
) -> Result<Object, String> {
    let lambda: Option<Object> = env.borrow_mut().get(variable);
    if lambda.is_none() {
        return Err(format!("Unbound variable: {}", variable));
    }

    let function: Object = lambda.unwrap();
    match function {
        Object::Lambda(parameters, body) => {
            let mut new_env = Rc::new(RefCell::new(Env::extend(env.clone())));
            for (i, parameter) in parameters.iter().enumerate() {
                let val: Object = evaluate_object(&list[i + 1], env)?;
                new_env.borrow_mut().set(parameter, val)
            }
            return evaluate_object(&Object::List(body), &mut new_env);
        }
        _ => return Err(format!("Not a lambda: {}", variable)),
    }
}

#[cfg(test)]
mod test_evaluator {
    use crate::{data::env::Env, evaluator::evaluate};
    use std::cell::RefCell;
    use std::rc::Rc;

    #[test]
    fn simple_add() {
        const PROGRAM: &str = "(+ 1 2)";
        let mut env = Rc::new(RefCell::new(Env::new()));
        let result = evaluate(PROGRAM, &mut env).unwrap();
    }
}
