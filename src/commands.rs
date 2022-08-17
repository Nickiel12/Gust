use strum::IntoEnumIterator;
use strum_macros::EnumIter;

use colored::Colorize;

#[derive(EnumIter)]
pub enum Commands {
    Add,
    Reset,
    Commit,
    Push,
    Pull,
    Settings,
}

impl Commands {
    pub fn from_string(input: String) -> Result<Self, String> {
        match input.to_lowercase().as_str() {
            "add" => Ok(Commands::Add),
            "reset" => Ok(Commands::Reset),
            "commit" => Ok(Commands::Commit),
            "push" => Ok(Commands::Push),
            "pull" => Ok(Commands::Pull),
            "settings" => Ok(Commands::Settings),
            _ => Err(format!("{} {}", "Unrecognized command: {}".red(), input)),
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            Commands::Add => String::from("add"),
            Commands::Reset => String::from("reset"),
            Commands::Commit => String::from("commit"),
            Commands::Push => String::from("push"),
            Commands::Pull => String::from("pull"),
            Commands::Settings => String::from("settings"),
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
