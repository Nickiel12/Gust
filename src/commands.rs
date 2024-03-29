use strum::IntoEnumIterator;
use strum_macros::EnumIter;

use colored::Colorize;

#[derive(EnumIter)]
pub enum BasicCommands {
    Add,
    Reset,
    Commit,
    AdvancedOptions,
    Quit,
}

impl BasicCommands {
    pub fn from_string(input: String) -> Result<Self, String> {
        match input.to_lowercase().as_str() {
            "add" => Ok(BasicCommands::Add),
            "reset" => Ok(BasicCommands::Reset),
            "commit" => Ok(BasicCommands::Commit),
            "advanced options" => Ok(BasicCommands::AdvancedOptions),
            "quit" => Ok(BasicCommands::Quit),
            _ => Err(format!("{} {}", "Unrecognized command: {}".red(), input)),
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            BasicCommands::Add => String::from("Add"),
            BasicCommands::Reset => String::from("Reset"),
            BasicCommands::Commit => String::from("Commit"),
            BasicCommands::AdvancedOptions => String::from("Advanced Options"),
            BasicCommands::Quit => String::from("Quit"),
        }
    }

    pub fn get_commands_vec() -> Vec<String> {
        let mut output = Vec::<String>::new();
        for item in BasicCommands::iter() {
            output.push(item.to_string())
        }
        return output;
    }
}

#[derive(EnumIter)]
pub enum Commands {
    Add,
    Reset,
    Commit,
    UndoCommit,
    Branches,
    Push,
    Pull,
    Remove,
    Quit,
}

impl Commands {
    pub fn from_string(input: String) -> Result<Self, String> {
        match input.to_lowercase().as_str() {
            "add" => Ok(Commands::Add),
            "reset" => Ok(Commands::Reset),
            "commit" => Ok(Commands::Commit),
            "undo_commit" => Ok(Commands::UndoCommit),
            "branches" => Ok(Commands::Branches),
            "push" => Ok(Commands::Push),
            "pull" => Ok(Commands::Pull),
            "remove" => Ok(Commands::Remove),
            "quit" => Ok(Commands::Quit),
            _ => Err(format!("{} {}", "Unrecognized command: {}".red(), input)),
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            Commands::Add => String::from("Add"),
            Commands::Reset => String::from("Reset"),
            Commands::Commit => String::from("Commit"),
            Commands::UndoCommit => String::from("Undo_commit"),
            Commands::Branches => String::from("Branches"),
            Commands::Push => String::from("Push"),
            Commands::Pull => String::from("Pull"),
            Commands::Remove => String::from("Remove"),
            Commands::Quit => String::from("Quit"),
        }
    }

    pub fn get_commands_vec() -> Vec<String> {
        let mut output = Vec::<String>::new();
        for item in Commands::iter() {
            output.push(item.to_string())
        }
        return output;
    }
}
