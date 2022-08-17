use crate::cli;

use colored::Colorize;

pub fn git_add_cli() -> Result<(), String> {
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

    let usr_selected = cli::choice_no_limit(choices, true)?;
    return match usr_selected {
        None => {
            println!("None selected, returning");
            Ok(())
        }
        Some(choice) => cli::git_add(choice),
    };
}

pub fn git_reset_cli() -> Result<(), String> {
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
            return match cli::choice_no_limit(choices, true)? {
                None => {
                    println!("None selected, returning");
                    Ok(())
                }
                Some(choice) => return cli::git_reset(choice),
            };
        }
    };
}

pub fn git_commit_cli() -> Result<(), String> {
    println!(
        "{} {} {}",
        "Opening".green(),
        "Commit".bold().green(),
        "menu".green()
    );
    let status_opt = cli::git_status_short()?;

    fn no_staged() -> Result<(), String> {
        if cli::ask_choice_cli("No files staged, would you like to add some?".to_string())? {
            git_add_cli()
        } else {
            return Err("Git commit function failed at 'no_staged ask_choice_cli'".to_string());
        }
    }

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
            if choices.len() == 0 {
                match no_staged() {
                    Ok(_) => true,
                    Err(_err) => false,
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
        // Get commit type
        // Get commit message
        // Get advanced description
        cli::git_commit(None)?;
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

pub fn settings() -> Result<(), String> {
    
    println!("{}", "No settings implemented yet".bright_cyan());

    Ok(())
}