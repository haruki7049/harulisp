use crate::env::Env;
use crate::object::Object;
use crate::parser;
use std::cell::RefCell;
use std::rc::Rc;

fn eval_obj(object: &Object, env: &mut Rc<RefCell<Env>>) -> Result<Object, String> {
    match object {
        Object::Void => Ok(Object::Void),
        Object::Lambda(_param, _body) => Ok(Object::Void),
        Object::Bool(_) => Ok(object.clone()),
        Object::String(string) => Ok(Object::String((*string).clone())),
        Object::Integer(n) => Ok(Object::Integer(*n)),
        Object::Float(number) => Ok(Object::Float(*number)),
        Object::Symbol(s) => eval_symbol(s, env),
        Object::List(list) => eval_list(list, env),
    }
}

fn eval_symbol(symbol: &str, env: &mut Rc<RefCell<Env>>) -> Result<Object, String> {
    let value = env.borrow_mut().get(symbol);
    if value.is_none() {
        return Err(format!("Unbound symbol: {}", symbol));
    }

    Ok(value.unwrap().clone())
}

fn eval_list(list: &Vec<Object>, env: &mut Rc<RefCell<Env>>) -> Result<Object, String> {
    let head = &list[0];
    match head {
        Object::Symbol(s) => match s.as_str() {
            "+" | "-" | "*" | "/" | "<" | ">" | "=" | "!=" => {
                return eval_binary_op(&list, env);
            }
            "define" => eval_define(&list, env),
            "if" => eval_if(&list, env),
            "lambda" => eval_function_definition(&list),
            _ => eval_function_call(&s, &list, env),
        },
        _ => {
            let mut new_list = Vec::new();
            for obj in list {
                let result = eval_obj(obj, env)?;
                match result {
                    Object::Void => {}
                    _ => new_list.push(result),
                }
            }
            Ok(Object::List(new_list))
        }
    }
}

/// eval function, needed &str of program and... what is env?
pub fn eval(program: &str, env: &mut Rc<RefCell<Env>>) -> Result<Object, String> {
    let parsed_list = parser::parse(program.to_string());
    if parsed_list.is_err() {
        return Err(format!("{}", parsed_list.err().unwrap()));
    }

    eval_obj(&parsed_list.unwrap(), env)
}

fn eval_binary_op(list: &Vec<Object>, env: &mut Rc<RefCell<Env>>) -> Result<Object, String> {
    if list.len() != 3 {
        return Err(format!("Invalid number of arguments for infix operator"));
    }

    let operator = list[0].clone();
    let left = eval_obj(&list[1].clone(), env)?;
    let right = eval_obj(&list[2].clone(), env)?;
    let left_val = match left {
        Object::Integer(n) => n,
        _ => return Err(format!("Left operand must be an integer {:?}", left)),
    };
    let right_val = match right {
        Object::Integer(n) => n,
        _ => return Err(format!("Right operand must be an integer {:?}", right)),
    };

    match operator {
        Object::Symbol(s) => match s.as_str() {
            "+" => Ok(Object::Integer(left_val + right_val)),
            "-" => Ok(Object::Integer(left_val - right_val)),
            "*" => Ok(Object::Integer(left_val * right_val)),
            "/" => Ok(Object::Integer(left_val / right_val)),
            "<" => Ok(Object::Bool(left_val < right_val)),
            ">" => Ok(Object::Bool(left_val > right_val)),
            "=" => Ok(Object::Bool(left_val == right_val)),
            "!=" => Ok(Object::Bool(left_val != right_val)),
            _ => Err(format!("Invalid infix operator: {}", s)),
        },
        _ => Err(format!("Operator must be a symbol")),
    }
}

fn eval_define(list: &Vec<Object>, env: &mut Rc<RefCell<Env>>) -> Result<Object, String> {
    if list.len() != 3 {
        return Err(format!("Invalid number of arguments for define"));
    }

    let sym = match &list[1] {
        Object::Symbol(s) => s.clone(),
        _ => return Err(format!("Invalid define")),
    };
    let val = eval_obj(&list[2], env)?;

    env.borrow_mut().set(&sym, val);
    Ok(Object::Void)
}

fn eval_if(list: &Vec<Object>, env: &mut Rc<RefCell<Env>>) -> Result<Object, String> {
    if list.len() != 4 {
        return Err(format!("Invalid number of arguments for if statement"));
    }

    let cond_object = eval_obj(&list[1], env)?;
    let cond = match cond_object {
        Object::Bool(b) => b,
        _ => return Err(format!("Condition must be a boolean")),
    };

    if cond == true {
        return eval_obj(&list[2], env);
    } else {
        return eval_obj(&list[3], env);
    }
}

fn eval_function_definition(list: &Vec<Object>) -> Result<Object, String> {
    let parameters = match &list[1] {
        Object::List(list) => {
            let mut parameters = Vec::new();
            for parameter in list {
                match parameter {
                    Object::Symbol(s) => parameters.push(s.clone()),
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

fn eval_function_call(
    symbol: &str,
    list: &Vec<Object>,
    env: &mut Rc<RefCell<Env>>,
) -> Result<Object, String> {
    let lambda = env.borrow_mut().get(symbol);
    if lambda.is_none() {
        return Err(format!("Unbound symbol: {}", symbol));
    }

    let function = lambda.unwrap();
    match function {
        Object::Lambda(parameters, body) => {
            let mut new_env = Rc::new(RefCell::new(Env::extend(env.clone())));
            for (i, param) in parameters.iter().enumerate() {
                let val = eval_obj(&list[i + 1], env)?;
                new_env.borrow_mut().set(param, val);
            }
            return eval_obj(&Object::List(body), &mut new_env);
        }
        _ => return Err(format!("Not a lambda: {}", symbol)),
    }
}

#[cfg(test)]
mod test_evaluator {
    use crate::env::Env;
    use crate::eval::eval;
    use crate::object::Object;
    use std::cell::RefCell;
    use std::rc::Rc;

    #[test]
    fn simple_add() {
        let mut env = Rc::new(RefCell::new(Env::new()));
        const PROGRAM: &str = "(+ 1 2)";
        let result: Object = eval(PROGRAM, &mut env).unwrap();

        assert_eq!(result, Object::Integer(3));
    }

    #[test]
    fn area_of_a_circle() {
        let mut env = Rc::new(RefCell::new(Env::new()));
        let program: &str = "(
              (define r 10)
              (define pi 314)
              (* pi (* r r))
            )";

        let result = eval(program, &mut env).unwrap();
        assert_eq!(
            result,
            Object::List(vec![Object::Integer((314 * 10 * 10) as isize)])
        );
    }
}
