mod commands;
pub use commands::Commands;

mod utils;

mod cli;
mod cli_menus;

use clap::Parser;
use colored::Colorize;

#[derive(Parser)]
struct CliArguments {
    command: Option<String>,
}

fn main() {
    println!(
        "{}{}",
        "have you added support for merge conflicts yet?\n".bright_red(),
        "missing git functionality: `diff,` branches, conflict resolution, stashes,".bright_yellow(),
    );
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
            command = match cli::filter_choice_cli(Commands::get_commands_vec()) {
                Ok(com) => com,
                Err(val) => {
                    println!("{}", val);
                    std::process::exit(1);
                }
            }
        }
    }

    let result = match command {
        Commands::Add => cli_menus::git_add_cli().unwrap(),
        Commands::Reset => cli_menus::git_reset_cli().unwrap(),
        Commands::Commit => cli_menus::git_commit_cli().unwrap(),
        Commands::Push => cli_menus::git_push_cli().unwrap(),
        Commands::Pull => cli_menus::git_pull_cli().unwrap(),
        Commands::Settings => cli_menus::settings().unwrap(),
    };
}
