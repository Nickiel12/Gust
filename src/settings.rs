use colored::Colorize;
use confy;
use dirs_next;
use serde::{Deserialize, Serialize};
use serde_json;

use crate::cli;

const APP_NAME: &str = "gust";
pub const DEFAULT_COMMIT_CONV: &str = "ConventionalCommits";

pub fn get_config(file_name: &str) -> std::path::PathBuf {
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
            convention: DEFAULT_COMMIT_CONV.to_string(),
            use_types: true,
            use_scope: true,
            use_important: true,
            use_footers: false,
            allow_none_convention: true,
        }
    }
}

pub fn load_config() -> Result<Config, confy::ConfyError> {
    return confy::load(APP_NAME);
}

pub fn save_config(config: Config) -> Result<(), confy::ConfyError> {
    return confy::store(APP_NAME, config);
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

pub fn check_for_convention_file(config: &Config) -> Result<(), String> {
    let expected_path = get_config((DEFAULT_COMMIT_CONV.to_string() + ".json").as_str());

    if !expected_path.exists() {
        println!("{}", "Convention file not found!".bright_red());

        if config.convention == DEFAULT_COMMIT_CONV {
            if cli::ask_choice_cli(
                "Would you like to copy the default convention file?".to_string(),
            )? {
                let mut cwd_path = std::env::current_dir().map_err(|e| e.to_string())?;
                cwd_path.push(DEFAULT_COMMIT_CONV.to_string() + ".json");

                let mut exe_path = std::env::current_exe().map_err(|e| e.to_string())?;
                exe_path.push("..");
                exe_path.push(DEFAULT_COMMIT_CONV.to_string() + ".json");

                if cwd_path.exists() {
                    std::fs::copy(cwd_path, expected_path).map_err(|e| e.to_string())?;
                } else if exe_path.exists() {
                    std::fs::copy(exe_path, expected_path).map_err(|e| e.to_string())?;
                } else {
                    return Err("Default convention file found! Make sure that the default convention file is next to the executable, then create an issue".to_string());
                }
            }
        } else {
            return Err(format!(
                "{}: '{}'",
                "Please put the convention file at".bright_yellow(),
                expected_path.to_str().unwrap(),
            ));
        }
    }
    return Ok(());
}
