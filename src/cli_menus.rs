use crate::cli::{self, UserResponse};
use crate::settings::Config;
use crate::utils;

use colored::Colorize;
use console;
use dialoguer::{theme::ColorfulTheme, Editor, Input};

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
        cli::UserResponse::Some(choice) => cli::git_add(choice),
        cli::UserResponse::All => cli::git_add(utils::strip_vec_colors(choices)),
    };
}

pub fn git_reset_cli(config: &Config) -> Result<(), String> {
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
                    'M' | 'A' | 'C' | 'D' => choices.push(line[3..].yellow().to_string()),
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

pub fn git_branches_cli(_config: &Config) -> Result<(), String> {
    let stdout = console::Term::stdout();
    stdout.clear_last_lines(3).map_err(|e| e.to_string())?;

    let choices = vec!["Switch HEAD".to_string(), "Create new branch".to_string()];

    let choice = cli::choice_single(choices, String::from("Select action"), false, false)?;
    stdout.clear_last_lines(1).map_err(|e| e.to_string())?;

    match choice {
        UserResponse::Some(val) => match val {
            0 => {
                match cli::git_get_branches()? {
                    None => println!("{}", "You have no branches here".bright_red()),
                    Some(mut branches) => {
                        let target = cli::choice_single(branches.clone(), "Select branch you wish to switch to".bright_yellow().to_string(), false, false)?;

                        match target {
                            UserResponse::Some(index) => {
                                let branch = utils::strip_colors(branches[index].clone());
                                if branch.contains("remotes/") {
                                    cli::git_fetch()?;
                                }

                                branches.clear();
                                branches.push(branch);
                                cli::git_checkout(branches)?;
                                return Ok(())
                            },
                            _ => return Err("How did you do that one!".to_string())
                        }
                   }
                }
            }
            1 => {
                let name = cli::get_input("Enter new branch name: ".to_string())?;
                cli::git_create_branch(name)?;
            }
            _ => return Err("Wow, I don't even know what to say...\n Goodbye".to_string()),
        },
        _ => return Err("You achieved the impossible".to_string()),
    }

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

pub fn git_checkout_cli() -> Result<(), String> {
    let choice_checkout_prompt: String = String::from("Select files to checkout:");
    let status_opt = cli::git_status_short()?;

    match status_opt {
        None => {
            println!("{}", "All files are current with HEAD".bright_green());
            Ok(())
        }
        Some(status_output) => {
            let mut choices = Vec::<String>::new();
            // for every line that is unstaged changes,
            // add to choices
            for line in status_output.lines() {
                match line.chars().nth(1).unwrap() {
                    ' ' => continue,
                    _ => choices.push(line[3..].yellow().to_string()),
                }
            }
            // if there is an output to 'git status', but no unstaged changes,
            // ask user if they want to checkout any staged files
            if choices.len() == 0 {
                if status_output.len() != 0 {
                    if cli::ask_choice_cli(
                        "All changes are staged, would you still like to checkout?".to_string(),
                    )? {
                        for line in status_output.lines() {
                            choices.push(line[3..].yellow().to_string());
                        }
                        let choices = match cli::choice_no_limit(
                            choices.clone(),
                            choice_checkout_prompt,
                            true,
                            true,
                        )? {
                            cli::UserResponse::All => utils::strip_vec_colors(choices),
                            cli::UserResponse::Some(selected) => utils::strip_vec_colors(selected),
                            cli::UserResponse::None => Vec::<String>::new(),
                        };
                        if choices.len() != 0 {
                            cli::git_reset(choices.clone())?;
                            cli::git_checkout(choices)?;
                        }
                    }
                }
                return Ok(());
            } else {
                // If there are unstaged chnages, ask the user to choose from them
                let choices = match cli::choice_no_limit(
                    choices.clone(),
                    choice_checkout_prompt,
                    true,
                    true,
                )? {
                    cli::UserResponse::All => Some(utils::strip_vec_colors(choices)),
                    cli::UserResponse::Some(selected) => Some(utils::strip_vec_colors(selected)),
                    cli::UserResponse::None => None,
                };
                if choices.is_some() {
                    cli::git_checkout(choices.unwrap())?;
                }
                Ok(())
            }
        }
    }
}
