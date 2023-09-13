extern crate harulisp;
use harulisp::env::Env;
use harulisp::eval::eval;
use harulisp::keywords::object::Object;

use linefeed::Interface;
use linefeed::ReadResult;

use std::rc::Rc;
use std::cell::RefCell;

const PROMPT: &str = "harulisp:> ";

fn main() -> Result<(), Box<dyn std::error::Error>>{
    let reader = Interface::new(PROMPT).unwrap();
    let mut env = Rc::new(RefCell::new(Env::new()));

    reader.set_prompt(format!("{}", PROMPT).as_ref()).unwrap();

    while let ReadResult::Input(input) = reader.read_line().unwrap() {
        if input.eq("exit") {
            break;
        }

        let value = eval(input.as_ref(), &mut env)?;
        match value {
            Object::Void => {},
            Object::Integer(n) => println!("{}", n),
            Object::Bool(b) => println!("{}", b),
            Object::Symbol(s) => println!("{}", s),
            Object::Lambda(parameters, body) => {
                println!("Lambda(");
                for parameter in parameters {
                    println!("{}", parameter);
                }

                println!(")");
                for expression in body {
                    println!(" {}", expression);
                }
            },
            _ => println!("{}", value),
        }
    }

    println!("Bye.");
    Ok(())
}
