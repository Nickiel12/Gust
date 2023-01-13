use crate::cli;
use crate::utils;

use colored::Colorize;

pub fn git_remove_cli() -> Result<(), String> {
    let choice_remove_prompt: String = String::from("Select files to remove from tracking:");

    match cli::git_ls_tree()? {
        None => println!(
            "{}",
            "There were no files found in the git repo".bright_green()
        ),
        Some(options) => {
            let mut choices = Vec::<String>::new();
            for line in options.lines() {
                choices.push(line.yellow().to_string());
            }

            let as_cached = cli::ask_yes_no(
                String::from("Would you like to delete the selected files from the disk?"),
                false,
            )?;

            let user_choices =
                match cli::choice_no_limit(choices.clone(), choice_remove_prompt, true, false)? {
                    cli::UserResponse::All => utils::strip_vec_colors(choices),
                    cli::UserResponse::Some(selected) => utils::strip_vec_colors(selected),
                    cli::UserResponse::None => vec![],
                };

            if user_choices.len() == 0 {
                println!("{}", "Nothing selected".bright_yellow());
            } else {
                cli::git_rm(user_choices, as_cached)?;
            }
        }
    }

    Ok(())
}
