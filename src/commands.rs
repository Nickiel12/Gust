pub enum Commands {
    Add,
    Reset,
    Commit,
}

impl Commands {
    pub fn get_from_string(input: String) -> Option<Self> {
        match input.to_lowercase().as_str() {
            "add" => Some(Commands::Add),
            "reset" => Some(Commands::Reset),
            "commit" => Some(Commands::Commit),
            _ => None,
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            Commands::Add => String::from("add"),
            Commands::Reset => String::from("reset"),
            Commands::Commit => String::from("commit"),
        }
    }
}
