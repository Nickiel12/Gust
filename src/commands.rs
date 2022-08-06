use strum::IntoEnumIterator;
use strum_macros::EnumIter;

use colored::Colorize;

#[derive(EnumIter)]
pub enum Commands {
    Add,
    Reset,
    Commit,
}

impl Commands {
    pub fn from_string(input: String) -> Result<Self, String> {
        match input.to_lowercase().as_str() {
            "add" => Ok(Commands::Add),
            "reset" => Ok(Commands::Reset),
            "commit" => Ok(Commands::Commit),
            _ => Err(format!("{} {}", "Unrecognized command: {}".red(), input)),
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            Commands::Add => String::from("add"),
            Commands::Reset => String::from("reset"),
            Commands::Commit => String::from("commit"),
        }
    }

    pub fn get_gum_string() -> String {
        let mut output = String::new();
        for item in Commands::iter() {
            output += "\n";
            output += item.to_string().as_str();
        }
        return output;
    }
}
