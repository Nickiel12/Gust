use crate::cli;
use crate::settings::Config;

use colored::Colorize;

pub fn git_reset_cli(_config: &Config) -> Result<(), String> {
    let choice_reset_prompt: String = String::from("Select files to reset:");

    println!(
        "{} {} {}",
        "Opening".green(),
        "Reset".bold().green(),
        "menu".green()
    );
    let status_opt = cli::git_status_short()?;

    return match status_opt {
        None => {
            println!("{}", "No files staged".bright_green());
            Ok(())
        }
        Some(status) => {
            let mut choices = Vec::<String>::new();
            for line in status.lines() {
                match line.chars().nth(0).unwrap() {
                    'M' | 'A' | 'C' | 'D' => {
                        // remove the parenthesis from long file names
                        if line.chars().nth(3).unwrap() == '"' {
                            choices.push(line[4..line.len() - 1].yellow().to_string());
                        } else {
                            choices.push(line[3..].yellow().to_string());
                        }
                    }
                    _ => {}
                }
            }
            return match cli::choice_no_limit(choices, choice_reset_prompt, true, false)? {
                cli::UserResponse::None => {
                    println!("None selected, returning");
                    Ok(())
                }
                cli::UserResponse::Some(choice) => return cli::git_reset(choice),
                _ => Err("User shouldn't have been able to select 'all' in 'reset'".to_string()),
            };
        }
    };
}
