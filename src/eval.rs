mod value;
use crate::parser;
use crate::parser::{Program, Token};
use anyhow::Context as _;
use std::collections::HashMap;
use value::{Action, Atom, BuiltinFunction, Type};

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
        dbg!(entrypoint);

        //match entrypoint {
        //    //Type::BuiltinFunction(BuiltinFunction::Lambda) => run_action(args, action),
        //    v => anyhow::bail!(
        //        "EVAL ERROR: The entrypoint cannot receive other types instead of Lambda Type. The value -> {:?}",
        //        v
        //    ),
        //};

        todo!()
    }

    /// Load scripts
    fn load(program: Program) -> anyhow::Result<HarulispMachine> {
        let mut machine: HarulispMachine = HarulispMachine {
            variables: HashMap::new(),
            entrypoint: String::from("main"),
        };

        for statement in &program.statements {
            dbg!(Type::try_from(statement)?);

            //machine.append();
        }

        todo!()

        //Ok(machine)
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

    fn deref_variable(&self, token: &Token) -> anyhow::Result<&Type> {
        match token {
            Token::Word(s) => todo!(),
            v => anyhow::bail!("EVAL ERROR: Cannot dereference {:?}", v),
        }
    }
}

fn run_action(args: &Vec<Type>, action: &Action) -> anyhow::Result<()> {
    if args.len() != 0 {
        todo!("");
    }

    match action {
        Action::Progn(v) => {
            for action in v {
                run_action(args, action)?;
            }
        }
        Action::Print(v) => println!("{}", v),
    }

    Ok(())
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

    /// Resolves the variable's value
    fn deref_variable(&self, token: &Token) -> anyhow::Result<&Type>;
}
