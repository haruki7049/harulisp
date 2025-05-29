mod value;

use crate::parser;
use crate::parser::{Program, Token};
use anyhow::Context as _;
use std::collections::HashMap;
use value::{Action, Atom, Type};

/// Evaluates a String as Harulisp
pub fn eval(str: String) -> anyhow::Result<()> {
    let program: Program = match parser::parse(str.as_str()) {
        Ok(v) => v,
        Err(e) => anyhow::bail!(e),
    };

    HarulispMachine::default()
        .run(program)?;

    Ok(())
}

#[derive(Debug, Default)]
struct HarulispMachine {
    variables: HashMap<String, Type>,
    entrypoint: String,
}

impl Machine for HarulispMachine {
    /// Run Harulisp
    fn run(&self, program: Program) -> anyhow::Result<()> {
        let entrypoint: Type = Type::Atom(Atom::String(String::from("main")));

        for statement in program.statements {
            let value: Type = statement.eval();

            if value.is_print() {
                println!("{}", statement.value);
            }
        }

        todo!()
    }
}

trait Machine {
    /// Run Harulisp
    fn run(&self, program: Program) -> anyhow::Result<()>;
}
