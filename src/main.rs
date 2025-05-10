use clap::Parser;

fn main() -> anyhow::Result<()> {
    let args: CLIArgs = CLIArgs::parse();
    dbg!(args.script);

    Ok(())
}

#[derive(Parser)]
#[command(version, about)]
struct CLIArgs {
    #[arg(short, long)]
    script: String,
}
