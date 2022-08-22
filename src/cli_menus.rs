use crate::cli;
use crate::settings::Config;
use crate::utils;

use colored::Colorize;
use console;
use dialoguer::{theme::ColorfulTheme, Editor, Input};

pub fn git_add_cli(config: &Config) -> Result<(), String> {
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
                choices.push(line[3..].bright_green().to_string());
            }
            // Modified from head, but not staged
            'M' => {
                // https://www.tutorialspoint.com/what-is-the-short-status-in-git
                match line.chars().nth(0).unwrap() {
                    // No staged changes, Added
                    ' ' | 'A' => {
                        choices.push(line[3..].green().to_string());
                    }
                    // Modified, Deleted, Renamed, Updated but merged
                    'M' | 'D' | 'R' | 'U' => {
                        choices.push(line[3..].yellow().to_string());
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
                choices.push(line[3..].bright_yellow().to_string());
            }
            // Added
            'A' => {
                choices.push(line[3..].bright_red().to_string());
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

    let usr_selected = cli::choice_no_limit(choices.clone(), true, config.show_all_in_add_menu)?;
    return match usr_selected {
        cli::UserResponse::None => {
            println!("None selected, returning");
            Ok(())
        }
        cli::UserResponse::Some(choice) => cli::git_add(choice),
        cli::UserResponse::All => cli::git_add(utils::strip_vec_colors(choices)),
    };
}

pub fn git_reset_cli(config: &Config) -> Result<(), String> {
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
                    'M' | 'A' | 'C' | 'D' => choices.push(line[3..].yellow().to_string()),
                    _ => {}
                }
            }
            return match cli::choice_no_limit(choices, true, false)? {
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

pub fn git_commit_cli(config: &Config) -> Result<(), String> {
    println!(
        "{} {} {}",
        "Opening".green(),
        "Commit".bold().green(),
        "menu".green()
    );
    let status_opt = cli::git_status_short()?;

    let do_commit = match status_opt {
        None => {
            println!("{}", "No files changed since last commit".bright_yellow());
            false
        }
        Some(status) => {
            // Sort the return from git status
            let mut choices = Vec::<String>::new();
            for line in status.lines() {
                match line.chars().nth(0).unwrap() {
                    'M' | 'A' | 'C' | 'D' => choices.push(line[3..].yellow().to_string()),
                    _ => {}
                }
            }
            // check for staged changes
            if choices.len() == 0 {
                if cli::ask_choice_cli("No files staged, would you like to add some?".to_string())?
                {
                    git_add_cli(&config)?;
                    true
                } else {
                    false
                }
            } else {
                if cli::ask_choice_cli(format!(
                    "{}:\n{}",
                    "Commit the following files?",
                    choices.join("\n")
                ))? {
                    true
                } else {
                    false
                }
            }
        }
    };

    if do_commit {
        if config.use_git_commit_message_dialog {
            if do_commit {
                // Get commit type
                // Get commit message
                // Get advanced description
                cli::git_commit(None, config)?;
            }
        } else {
            let conventions = crate::settings::load_convention(config.convention.clone());
            let short_form: String = {
                let mut commit_msg = Vec::<String>::new();
                if config.use_types {
                    commit_msg.push(
                        cli::filter_choice_cli(conventions.types, true)?.unwrap_or("".to_string()),
                    );
                    if config.use_scope {
                        commit_msg.push(conventions.scope_delimeters.opening.clone());
                        commit_msg.push(
                            cli::filter_choice_cli(conventions.scopes, true)?
                                .unwrap_or("".to_string()),
                        );
                        commit_msg.push(conventions.scope_delimeters.closing.clone());
                    }
                    if config.use_important {
                        if cli::ask_choice_cli(format!(
                            "Flag this commit as important with: '{}'",
                            conventions.important_symbol
                        ))? {
                            commit_msg.push(conventions.important_symbol);
                        }
                    }
                    commit_msg.push(conventions.separator);
                    commit_msg.push(" ".to_string());
                }

                let usr_selection: String = Input::with_theme(&ColorfulTheme::default())
                    .allow_empty(false)
                    .with_initial_text(commit_msg.join(""))
                    .with_prompt(" Enter Commit Message, Shouldn't exceed this ---> |\n")
                    .interact_text_on(&console::Term::stderr())
                    .unwrap();

                usr_selection
            };

            let description: String = Editor::new()
                .edit("Enter a commit description")
                .unwrap()
                .unwrap();

            cli::git_commit(
                Some(vec![
                    "-m".to_string(),
                    short_form,
                    "-m".to_string(),
                    description,
                ]),
                config,
            )?;
        }
    }
    println!("{}", "Changes committed!".bright_green());
    Ok(())
}

pub fn git_pull_cli() -> Result<(), String> {
    cli::git_pull()?;
    Ok(())
}

pub fn git_push_cli() -> Result<(), String> {
    cli::git_push()?;
    Ok(())
}
