use clap::Parser;
use harulisp::tokenizer;

fn main() -> anyhow::Result<()> {
    let args: CLIArgs = CLIArgs::parse();

    // Tokenizer
    let tokens = tokenizer::tokenize(args.script)?;
    dbg!(tokens);

    Ok(())
}

#[derive(Parser)]
#[command(version, about)]
struct CLIArgs {
    #[arg(short, long)]
    script: String,
}
