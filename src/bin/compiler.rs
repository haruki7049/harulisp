use std::env;
use clap;

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);
}

#[derive(clap::Parser, Debug)]
#[command(name = env!("CARGO_PKG_AUTHORS"), author = env!("CARGO_PKG_AUTHORS"), version = env!("CARGO_PKG_AUTHORS"), about = env!("CARGO_PKG_AUTHORS"), long_about = None)]
struct Args {
    #[clap(subcommand)]
    subcommand: clap::Subcommand,
}
