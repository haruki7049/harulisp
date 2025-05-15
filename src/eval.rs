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

    HarulispMachine::load(program)?.run()?;

    Ok(())
}

#[derive(Debug)]
struct HarulispMachine {
    variables: HashMap<String, Type>,
    entrypoint: String,
}

impl Token {
    fn to_atom(&self) -> Atom {
        match self {
            Token::String(s) => Atom::String(s.clone()),
            Token::Int(i) => Atom::Int(*i),
            _ => todo!(),
        }
    }
}

impl Machine for HarulispMachine {
    /// Run Harulisp
    fn run(&self) -> anyhow::Result<()> {
        let entrypoint: &Type = self.get(self.entrypoint())?;

        match entrypoint {
            Type::Lambda((args, action)) => run_action(args, action),
            v => anyhow::bail!(
                "EVAL ERROR: The entrypoint cannot receive other types instead of Lambda Type. The value -> {:?}",
                v
            ),
        }?;

        Ok(())
    }

    /// Load scripts
    fn load(program: Program) -> anyhow::Result<HarulispMachine> {
        let mut machine: HarulispMachine = HarulispMachine {
            variables: HashMap::new(),
            entrypoint: String::from("main"),
        };

        for statement in &program.statements {
            load_sexpr(&mut machine, statement)?;
        }

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

fn load_sexpr(machine: &mut HarulispMachine, token: &Token) -> anyhow::Result<()> {
    match token {
        Token::SExpression(tokens) => {
            // whether the function name is "def" or not
            match &tokens[0] {
                Token::Word(s) => match s.as_str() {
                    "def" => {
                        let name: String = tokens[1].to_string();
                        let value: &Token = &tokens[2];

                        //machine.append(name, value);
                        todo!()
                    }
                    "lambda" => {
                        //let arguments: Type = tokens[1].to_type();
                        //let sexpr: Type = tokens[2].to_type();

                        //Ok(())
                        todo!()
                    }
                    "println" => todo!(),
                    _ => todo!(),
                },

                _ => unreachable!(),
            }
        }
        _ => unreachable!(),
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
