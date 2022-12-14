use crate::cli::{self, UserResponse};
use crate::settings::Config;
use crate::utils;

use crate::commands::{BasicCommands, Commands};

use colored::Colorize;
use console;
use dialoguer::{theme::ColorfulTheme, Editor, Input};

pub fn advanced_menu() -> Result<Commands, String> {
    let command = match cli::filter_choice_cli(Commands::get_commands_vec(), false) {
        Ok(com) => Commands::from_string(com.unwrap()),
        Err(val) => Err(val),
    };

    return command;
}

pub fn basic_menu() -> Result<Commands, String> {
    let choice = cli::filter_choice_cli(BasicCommands::get_commands_vec(), false)?;
    let command = BasicCommands::from_string(choice.unwrap())?;

    let result = match command {
        BasicCommands::Add => Ok(Commands::Add),
        BasicCommands::Reset => Ok(Commands::Reset),
        BasicCommands::Commit => Ok(Commands::Commit),
        BasicCommands::AdvancedOptions => advanced_menu(),
    };
    return result;
}

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

pub fn git_branches_cli(_config: &Config) -> Result<(), String> {
    let stdout = console::Term::stdout();

    let choices = vec!["Switch HEAD".to_string(), "Create new branch".to_string()];

    let choice = cli::choice_single(choices, String::from("Select action"), false, false)?;

    stdout.clear_last_lines(1).map_err(|e| e.to_string())?;

    println!(
        "{} {}: {}",
        "???".bright_green().to_string(),
        "Select action".bold().to_string(),
        {
            match choice {
                UserResponse::Some(val) => match val {
                    0 => "Switch HEAD",
                    1 => "Create New Branch",
                    _ => "Invalid Input",
                },
                _ => "Invalid Input",
            }
        }
    );

    match choice {
        UserResponse::Some(val) => match val {
            0 => match cli::git_get_branches()? {
                None => println!("{}", "You have no branches here".bright_red()),
                Some(mut branches) => {
                    let target = cli::choice_single(
                        branches.clone(),
                        "Select branch you wish to switch to"
                            .bright_yellow()
                            .to_string(),
                        false,
                        false,
                    )?;

                    match target {
                        UserResponse::Some(index) => {
                            let branch = utils::strip_colors(branches[index].clone());
                            if branch.contains("remotes/") {
                                cli::git_fetch()?;
                            }

                            branches.clear();
                            branches.push(branch);
                            cli::git_checkout(branches)?;
                            return Ok(());
                        }
                        _ => return Err("How did you do that one!".to_string()),
                    }
                }
            },
            1 => {
                let specific_commit_prompt =
                    String::from("Would you like to start the branch on a specific commit?");

                let select_commit_prompt =
                    String::from("Select a commit to base the new branch on:");

                // ask the user if they want to base the new branch on a specific commit
                if cli::ask_choice_cli(specific_commit_prompt)? {
                    let select_branch_prompt = "Select the branch the commit is on:"
                        .bright_yellow()
                        .to_string();

                    // Get the branch that the commit is on (for ease of use)
                    let branch = match cli::git_get_branches()? {
                        // Make sure there is a response from `git branch`
                        Some(branches) => {
                            // get the user to choose a branch
                            match cli::choice_single(
                                branches.clone(),
                                select_branch_prompt,
                                false,
                                false,
                            )? {
                                UserResponse::Some(index) => {
                                    let branch = utils::strip_colors(branches[index].clone());
                                    if branch.contains("remotes/") {
                                        cli::git_fetch()?;
                                    }

                                    branch
                                }
                                _ => panic!("You should not be in this specfics place 121212"),
                            }
                        }
                        None => {
                            panic!("{}", "You have no branches here".bright_red());
                        }
                    };

                    // Get the `git log` history on the chosen branch
                    let choices = match cli::git_log(Some(branch))? {
                        None => {
                            println!("{}", "No commits found! returning to menu".bright_yellow());
                            return Ok(());
                        }
                        Some(log_string) => {
                            let mut output = Vec::<String>::new();
                            for line in log_string.lines() {
                                output.push(line.to_string());
                            }
                            output
                        }
                    };

                    // Have the user select the commit to base the branch on
                    let usr_selected =
                        cli::choice_single(choices.clone(), select_commit_prompt, false, true)?;

                    match usr_selected {
                        UserResponse::None => {
                            println!("'None' selected, returning to menu");
                            return Ok(());
                        }
                        UserResponse::Some(choice) => {
                            let hash = choices[choice][..7].to_string();
                            println!("You have selected commit {}", hash);

                            let name = cli::get_input("Enter new branch name: ".to_string())?;

                            cli::git_create_branch(name, Some(hash))?;

                            return Ok(());
                        }
                        UserResponse::All => panic!(
                            "This code should be unreachable cli.rs - git-branch, usr_selected"
                        ),
                    }
                } else {
                    let name = cli::get_input("Enter new branch name: ".to_string())?;
                    cli::git_create_branch(name, None)?;
                }
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

            let as_cached = !cli::ask_yes_no(
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
