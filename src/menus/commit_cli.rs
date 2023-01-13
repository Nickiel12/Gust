use crate::cli;
use crate::settings::Config;

use crate::menus;

use colored::Colorize;
use console;
use dialoguer::{theme::ColorfulTheme, Editor, Input};

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
                    'M' | 'A' | 'C' | 'D' => {
                        if line.chars().nth(3).unwrap() == '"' {
                            choices.push(line[4..line.len() - 1].yellow().to_string());
                        } else {
                            choices.push(line[3..].yellow().to_string());
                        }
                    }
                    _ => {}
                }
            }
            // check for staged changes
            if choices.len() == 0 {
                if cli::ask_choice_cli("No files staged, would you like to add some?".to_string())?
                {
                    menus::git_add_cli(&config)?;
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
                    ); // The check against none is to skip all of the convention options
                       // if the first is skipped
                    if commit_msg[0] != "None" {
                        if config.use_scope {
                            let convention_scope =
                                cli::filter_choice_cli(conventions.scopes, true)?
                                    .unwrap_or("".to_string());
                            if convention_scope != "None" {
                                commit_msg.push(conventions.scope_delimeters.opening.clone());
                                commit_msg.push(convention_scope);
                                commit_msg.push(conventions.scope_delimeters.closing.clone());
                            }
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
                }

                commit_msg = commit_msg
                    .iter()
                    .filter(|x| x != &&String::from("None"))
                    .cloned()
                    .collect();

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
