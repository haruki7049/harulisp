use clap::Parser;
use harulisp::eval;

fn main() -> anyhow::Result<()> {
    let args: CLIArgs = CLIArgs::parse();

    // Evaluation
    eval::eval(args.script)?;

    Ok(())
}

#[derive(Parser)]
#[command(version, about)]
struct CLIArgs {
    #[arg(short, long)]
    script: String,
}
