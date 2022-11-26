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
        "{}",
        "Gust - Git, with rust. A simple git helper for the average user.\n".bright_blue()
    );
    println!(
        "{}{}{}{}",
        "window 'edit' throws error when notepad exited early\n".bright_red(),
        "missing git functionality: `diff,` branches, conflict resolution, stashes, creation, setting origin\n"
            .bright_yellow(),
        "Add menu for adding to gitignore".bright_blue(),
        "\n"
            .bright_red(),
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
            if config.enable_basic_menu {
                command = cli_menus::basic_menu().unwrap();
            } else {
                command = cli_menus::advanced_menu().unwrap();
            }
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
        Commands::Remove => cli_menus::git_remove_cli().unwrap(),
    };

    settings::save_config(config).unwrap();
}
