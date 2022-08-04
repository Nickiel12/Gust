mod commands;
pub use commands::Commands;

mod cli;

use clap::Parser;

#[derive(Parser)]
struct CliArguments {
    command: Option<String>,
}

fn main() {
    let args = CliArguments::parse();

    let command;
    match (args.command) {
        Some(com) => command = com,
        None => command = cli::get_git_action().unwrap().to_string(),
    }
    println!("{}", command);
}
