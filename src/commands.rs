use strum::IntoEnumIterator;
use strum_macros::EnumIter;

use colored::Colorize;

#[derive(EnumIter)]
pub enum BasicCommands {
    Add,
    Reset,
    Commit,
    AdvancedOptions,
}

impl BasicCommands {
    pub fn from_string(input: String) -> Result<Self, String> {
        match input.to_lowercase().as_str() {
            "add" => Ok(BasicCommands::Add),
            "reset" => Ok(BasicCommands::Reset),
            "commit" => Ok(BasicCommands::Commit),
            "advanced options" => Ok(BasicCommands::AdvancedOptions),
            _ => Err(format!("{} {}", "Unrecognized command: {}".red(), input)),
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            BasicCommands::Add => String::from("Add"),
            BasicCommands::Reset => String::from("Reset"),
            BasicCommands::Commit => String::from("Commit"),
            BasicCommands::AdvancedOptions => String::from("Advanced Options"),
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
    Checkout,
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
            "checkout" => Ok(Commands::Checkout),
            _ => Err(format!("{} {}", "Unrecognized command: {}".red(), input)),
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            Commands::Add => String::from("add"),
            Commands::Reset => String::from("reset"),
            Commands::Commit => String::from("commit"),
            Commands::UndoCommit => String::from("undo_commit"),
            Commands::Branches => String::from("branches"),
            Commands::Push => String::from("push"),
            Commands::Pull => String::from("pull"),
            Commands::Checkout => String::from("checkout"),
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
