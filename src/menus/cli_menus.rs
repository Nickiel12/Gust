use crate::cli;
use crate::commands::{BasicCommands, Commands};

pub fn advanced_menu() -> Result<Commands, String> {
    let command = match cli::filter_choice_cli(Commands::get_commands_vec(), false) {
        Ok(com) => Commands::from_string(com.unwrap()),
        Err(val) => Err(val),
    };

    return command;
}

pub fn basic_menu() -> Result<Commands, String> {
    let choice = cli::filter_choice_cli(BasicCommands::get_commands_vec(), false)?;
    let command = BasicCommands::from_string(choice.unwrap())?;

    let result = match command {
        BasicCommands::Add => Ok(Commands::Add),
        BasicCommands::Reset => Ok(Commands::Reset),
        BasicCommands::Commit => Ok(Commands::Commit),
        BasicCommands::AdvancedOptions => advanced_menu(),
    };
    return result;
}

pub fn git_pull_cli() -> Result<(), String> {
    cli::git_pull()?;
    Ok(())
}

pub fn git_push_cli() -> Result<(), String> {
    cli::git_push()?;
    Ok(())
}
