use crate::cli;
use crate::settings::Config;
use crate::utils;

use colored::Colorize;

pub fn git_add_cli(config: &Config) -> Result<(), String> {
    let choice_add_prompt: String = String::from("Select files to add:");

    let status_output: String;
    match cli::git_status_short()? {
        None => {
            println!("{}", "Nothing staged, returning to menu".bright_yellow());
            return Ok(());
        }
        Some(status_string) => status_output = status_string,
    }

    let mut choices = Vec::<String>::new();
    for line in status_output.lines() {
        //println!("debug line: {}", line);
        match line.chars().nth(1).unwrap() {
            // No changes from HEAD or unstaged changes
            ' ' => {
                continue;
            }
            // Not tracked
            '?' => {
                // Remove any quotation marks caused by spaces in filenames
                if line.chars().nth(3).unwrap() == '"' {
                    choices.push(line[4..line.len() - 1].bright_green().to_string());
                } else {
                    choices.push(line[3..].bright_green().to_string());
                }
            }
            // Modified from head, but not staged
            'M' => {
                // https://www.tutorialspoint.com/what-is-the-short-status-in-git
                match line.chars().nth(0).unwrap() {
                    // No staged changes, Added
                    ' ' | 'A' => {
                        // Remove any quotation marks caused by spaces in filenames
                        if line.chars().nth(3).unwrap() == '"' {
                            choices.push(line[4..line.len() - 1].green().to_string());
                        } else {
                            choices.push(line[3..].green().to_string());
                        }
                    }
                    // Modified, Deleted, Renamed, Updated but merged
                    'M' | 'D' | 'R' | 'U' => {
                        // Remove any quotation marks caused by spaces in filenames
                        if line.chars().nth(3).unwrap() == '"' {
                            choices.push(line[4..line.len() - 1].yellow().to_string());
                        } else {
                            choices.push(line[3..].yellow().to_string());
                        }
                    }
                    // Git was empty, but not?
                    _ => {
                        return Err(format!(
                            "git add cli menu recieved unknown first char: {}",
                            line
                        ))
                    }
                }
            }
            // Delete, Rename
            'D' | 'R' => {
                // Remove any quotation marks caused by spaces in filenames
                if line.chars().nth(3).unwrap() == '"' {
                    choices.push(line[4..line.len() - 1].bright_yellow().to_string());
                } else {
                    choices.push(line[3..].bright_yellow().to_string());
                }
            }
            // Added
            'A' => {
                // Remove any quotation marks caused by spaces in filenames
                if line.chars().nth(3).unwrap() == '"' {
                    choices.push(line[4..line.len() - 1].bright_red().to_string());
                } else {
                    choices.push(line[3..].bright_red().to_string());
                }
            }
            _ => {
                println!("ding: {}", line);
            }
        }
    }

    if choices.len() == 0 {
        println!("{}", "Nothing staged, returning to menu".bright_yellow());
        return Ok(());
    }

    let usr_selected = cli::choice_no_limit(
        choices.clone(),
        choice_add_prompt,
        true,
        config.show_all_in_add_menu,
    )?;
    return match usr_selected {
        cli::UserResponse::None => {
            println!("None selected, returning");
            Ok(())
        }
        cli::UserResponse::Some(choice) => {
            if choice.len() == 0 {
                println!("{}", "None selected".bright_yellow());
                Ok(())
            } else {
                cli::git_add(choice)
            }
        }
        cli::UserResponse::All => cli::git_add(utils::strip_vec_colors(choices)),
    };
}
