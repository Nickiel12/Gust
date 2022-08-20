use colored::Colorize;

use confy;
use serde::{Deserialize, Serialize};

const APP_NAME: &str = "gust";

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub show_welcome: bool,
    pub show_all_in_add_menu: bool,

    pub verbose_commit: bool,
}

impl std::default::Default for Config {
    fn default() -> Self {
        Self {
            show_welcome: false,
            show_all_in_add_menu: true,

            verbose_commit: false,
        }
    }
}

pub fn load_config() -> Result<Config, confy::ConfyError> {
    return confy::load(APP_NAME);
}

pub fn save_config(config: Config) -> Result<(), confy::ConfyError> {
    return confy::store(APP_NAME, config);
}

pub fn cli() -> Result<(), String> {
    println!("{}", "No settings implemented yet".bright_cyan());
    println!("Will eventually be used to handle everything from colors, to git ignore, etc");

    Ok(())
}
