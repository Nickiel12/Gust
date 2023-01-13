use crate::cli::{self, UserResponse};
use crate::settings::Config;
use crate::utils;

use colored::Colorize;
use console;

pub fn git_branches_cli(_config: &Config) -> Result<(), String> {
    let stdout = console::Term::stdout();

    let choices = vec!["Switch HEAD".to_string(), "Create new branch".to_string()];

    let choice = cli::choice_single(choices, String::from("Select action"), false, false)?;

    stdout.clear_last_lines(1).map_err(|e| e.to_string())?;

    println!(
        "{} {}: {}",
        "✓".bright_green().to_string(),
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
