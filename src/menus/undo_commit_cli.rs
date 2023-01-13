use crate::cli;
use crate::settings::Config;

use colored::Colorize;

pub fn git_undo_commit_cli(_config: &Config) -> Result<(), String> {
    let choice_undo_prompt: String = String::from("Select a commit to revert:");

    let log_output: String;
    match cli::git_log(None)? {
        None => {
            println!("{}", "No commits found! returning to menu".bright_yellow());
            return Ok(());
        }
        Some(log_string) => log_output = log_string,
    }

    let mut choices = Vec::<String>::new();
    for line in log_output.lines() {
        choices.push(line.to_string());
    }

    let usr_selected = cli::choice_single(choices.clone(), choice_undo_prompt, false, true)?;

    match usr_selected {
        cli::UserResponse::None => {
            println!("'None' selected, returning to menu");
            return Ok(());
        }
        cli::UserResponse::Some(choice) => {
            println!("{}", choice);
            let hash = choices[choice][..7].to_string();
            cli::git_revert(hash.clone())?;
            println!(
                "{}",
                format!("Commit '{}' reverted. Returning to menu", hash)
                    .bright_green()
                    .to_string()
            );
            return Ok(());
        }
        _ => panic!("This is impossible git undo commit"),
    }
}
