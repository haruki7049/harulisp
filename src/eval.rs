mod value;
use value::{Action, Value};

use crate::parser;
use crate::parser::Program;
use std::collections::HashMap;

/// Evaluates a String as Harulisp
pub fn eval(str: String) -> anyhow::Result<()> {
    let program: Program = match parser::parse(str.as_str()) {
        Ok(v) => v,
        Err(e) => anyhow::bail!(e),
    };

    HarulispMachine::load(program)?.run()?;

    Ok(())
}

#[derive(Debug)]
struct HarulispMachine {
    variables: HashMap<String, Value>,
    entrypoint: String,
}

impl Machine for HarulispMachine {
    /// Run Harulisp
    fn run(&self) -> anyhow::Result<()> {
        Ok(())
    }

    /// Load scripts
    fn load(_program: Program) -> anyhow::Result<HarulispMachine> {
        let mut machine: HarulispMachine = HarulispMachine {
            variables: HashMap::new(),
            entrypoint: String::from("main"),
        };

        machine.append(
            String::from("main"),
            Value::IO(Box::new(Action::Print(Value::Int(3)))),
        );

        Ok(machine)
    }

    /// Append variables
    fn append(&mut self, name: String, value: Value) {
        let _ = &self.variables.insert(name, value);
    }
}

trait Machine {
    /// Run Harulisp
    fn run(&self) -> anyhow::Result<()>;

    /// Load scripts
    fn load(program: Program) -> anyhow::Result<HarulispMachine>;

    /// Append variables
    fn append(&mut self, name: String, value: Value);
}
