use clap::Parser;
use std::str::FromStr;

fn main() {
    let app = Args::parse();
    if app.sub_command == SubCommands::Repl {
        println!("Sample code. You typed: {}", app.sub_command);
    }
}

#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Args {
    sub_command: SubCommands,
}

#[derive(Debug, Clone, PartialEq)]
enum SubCommands {
    Repl,
}

impl FromStr for SubCommands {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "repl" => Ok(SubCommands::Repl),
            _ => Err("Unknown command".to_string()),
        }
    }
}

impl std::fmt::Display for SubCommands {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            Self::Repl => write!(f, "repl"),
        }
    }
}
