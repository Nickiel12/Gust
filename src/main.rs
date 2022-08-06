mod commands;
pub use commands::Commands;

mod cli;

use clap::Parser;
use colored::Colorize;

#[derive(Parser)]
struct CliArguments {
    command: Option<String>,
}

fn main() {
    let args = CliArguments::parse();

    let command;
    match args.command.clone() {
        Some(com) => {
            command = match Commands::from_string(com) {
                Ok(value) => value,
                Err(_) => {
                    panic!("Unknown Command: {:?}", args.command.unwrap().red())
                }
            }
        }
        None => {
            command = match cli::get_git_action() {
                Ok(com) => com,
                Err(val) => {
                    println!("{}", val);
                    std::process::exit(1);
                }
            }
        }
    }
    println!("{}", command.to_string());
}
