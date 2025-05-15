mod value;
use crate::parser;
use crate::parser::Program;
use anyhow::Context as _;
use std::collections::HashMap;
use value::{Action, Atom, Type};

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
    variables: HashMap<String, Type>,
    entrypoint: String,
}

impl Machine for HarulispMachine {
    /// Run Harulisp
    fn run(&self) -> anyhow::Result<()> {
        let entrypoint: &Type = self.get(self.entrypoint())?;
        dbg!(&entrypoint);

        match entrypoint {
            Type::IO(action) => run_action(action),
            v => anyhow::bail!(
                "EVAL ERROR: The entrypoint cannot receive other types instead of IO Type. The value -> {:?}",
                v
            ),
        }

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
            Type::IO(Box::new(Action::Progn(vec![
                Action::Print(Atom::Int(1)),
                Action::Print(Atom::Int(33)),
            ]))),
        );
        machine.append(
            String::from("main"),
            Type::IO(Box::new(Action::Print(Atom::Int(3)))),
        );
        //machine.append(
        //    String::from("main"),
        //    Type::List(vec![Type::Atom(Atom::Int(3))]),
        //);

        Ok(machine)
    }

    /// Append variables
    fn append(&mut self, n: String, t: Type) {
        let _ = &self.variables.insert(n, t);
    }

    fn get(&self, name: &String) -> anyhow::Result<&Type> {
        Ok(self
            .variables
            .get(name)
            .context(format!("EVAL ERROR: No value found on {}", name))?)
    }

    /// Reads the entrypoint's name
    fn entrypoint(&self) -> &String {
        &self.entrypoint
    }
}

fn run_action(action: &Action) {
    match action {
        Action::Progn(v) => {
            for action in v {
                run_action(action);
            }
        }
        Action::Print(v) => println!("{}", v),
    }
}

trait Machine {
    /// Run Harulisp
    fn run(&self) -> anyhow::Result<()>;

    /// Load scripts
    fn load(program: Program) -> anyhow::Result<HarulispMachine>;

    /// Append variables
    fn append(&mut self, n: String, t: Type);

    /// Gets a variable from name: String
    fn get(&self, name: &String) -> anyhow::Result<&Type>;

    /// Reads the entrypoint's name
    fn entrypoint(&self) -> &String;
}
