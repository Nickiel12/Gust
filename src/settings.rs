use colored::Colorize;

use confy;
use dirs_next;
use serde::{Deserialize, Serialize};
use serde_json;

const APP_NAME: &str = "gust";

fn get_config(file_name: &str) -> std::path::PathBuf {
    let mut dir = dirs_next::config_dir().unwrap();
    dir.push(APP_NAME);
    dir.push(file_name);
    return dir;
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub show_welcome: bool,
    pub show_all_in_add_menu: bool,

    pub verbose_commit: bool,
    pub use_git_commit_message_dialog: bool,

    pub use_commit_convention: bool,
    pub convention: String,
    pub use_types: bool,
    pub use_scope: bool,
    pub use_important: bool,
    pub use_footers: bool,
    pub allow_none_convention: bool,
}

impl std::default::Default for Config {
    fn default() -> Self {
        Self {
            show_welcome: false,
            show_all_in_add_menu: true,

            verbose_commit: false,
            use_git_commit_message_dialog: false,

            use_commit_convention: true,
            convention: "ConventionalCommits".to_string(),
            use_types: true,
            use_scope: true,
            use_important: true,
            use_footers: false,
            allow_none_convention: true,
        }
    }
}

pub fn load_config() -> Result<Config, confy::ConfyError> {
    return confy::load_path(get_config((APP_NAME.to_string() + ".toml").as_str()));
}

pub fn save_config(config: Config) -> Result<(), confy::ConfyError> {
    return confy::store(
        get_config((APP_NAME.to_string() + ".toml").as_str())
            .into_os_string()
            .into_string()
            .unwrap()
            .as_str(),
        config,
    );
}

#[derive(Serialize, Deserialize)]
pub struct ScopeDelims {
    pub opening: String,
    pub closing: String,
}

#[derive(Serialize, Deserialize)]
pub struct ConventionSettings {
    pub types: Vec<String>,
    pub scopes: Vec<String>,
    pub scope_delimeters: ScopeDelims,
    pub important_symbol: String,
    pub separator: String,
    pub footers: Vec<String>,
}

pub fn load_convention(convention_name: String) -> ConventionSettings {
    let json: ConventionSettings = serde_json::from_reader(
        std::fs::File::open(get_config((convention_name + ".json").as_str())).unwrap(),
    )
    .unwrap();
    return json;
}

pub fn cli() -> Result<(), String> {
    println!("{}", "No settings implemented yet".bright_cyan());
    println!("Will eventually be used to handle everything from colors, to git ignore, etc");

    Ok(())
}
