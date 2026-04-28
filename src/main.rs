use clap::Parser;
use harulisp::tokenizer;

fn main() -> anyhow::Result<()> {
    let args: CLIArgs = CLIArgs::parse();

    // Tokenizer
    tokenizer::tokenize(args.script)?;

    Ok(())
}

#[derive(Parser)]
#[command(version, about)]
struct CLIArgs {
    #[arg(short, long)]
    script: String,
}
