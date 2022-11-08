mod commands;
pub use commands::{BasicCommands, Commands};

mod utils;

mod cli;
mod cli_menus;
mod settings;

use clap::Parser;
use colored::Colorize;

#[derive(Parser)]
struct CliArguments {
    command: Option<String>,
    args: Option<String>,
}

fn main() {
    println!(
        "{}{}{}",
        "have you added support for merge conflicts yet?\nwindow 'edit' throws error when notepad exited early".bright_red(),
        "missing git functionality: `diff,` branches, conflict resolution, stashes,"
            .bright_yellow(),
        "Have you added `git rm` with `--cached` (not disk deletion) and non cached removal?".bright_red(),
    );
    let args = CliArguments::parse();

    let mut config = settings::load_config().unwrap();

    settings::check_for_convention_file(&config).unwrap();

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
            command = cli_menus::basic_menu().unwrap();
        }
    }

    let _result = match command {
        Commands::Add => cli_menus::git_add_cli(&config).unwrap(),
        Commands::Reset => cli_menus::git_reset_cli(&config).unwrap(),
        Commands::Commit => cli_menus::git_commit_cli(&config).unwrap(),
        Commands::UndoCommit => cli_menus::git_undo_commit_cli(&config).unwrap(),
        Commands::Branches => cli_menus::git_branches_cli(&config).unwrap(),
        Commands::Push => cli_menus::git_push_cli().unwrap(),
        Commands::Pull => cli_menus::git_pull_cli().unwrap(),
        Commands::Checkout => cli_menus::git_checkout_cli().unwrap(),
    };

    settings::save_config(config).unwrap();
}
