use crate::cli;
use crate::commands::Commands;
use crate::utils;

use colored::Colorize;
use std::process::{Command, Stdio};

pub fn git_add_cli() -> Result<(), String> {
    let git_status_cmd = Command::new("git")
        .arg("status")
        .arg("--short")
        .stdout(Stdio::piped())
        .spawn()
        .expect("Couldn't call git add!");

    let git_status = git_status_cmd.wait_with_output().unwrap();

    if git_status.status.success() {
        let status_output = String::from_utf8_lossy(&git_status.stdout).to_string();
        if status_output == "" {
            println!("nothing returned by git add");
            return Ok(());
        }

        let mut choices = String::new();
        for line in status_output.lines() {
            //println!("debug line: {}", line);
            match line.chars().nth(1).unwrap() {
                // No changes from HEAD or unstaged changes
                ' ' => {
                    continue;
                }
                // Not tracked
                '?' => {
                    choices += "\n";
                    choices += line[3..].bright_green().to_string().as_str();
                }
                // Modified from head, but not staged
                'M' => {
                    // https://www.tutorialspoint.com/what-is-the-short-status-in-git
                    match line.chars().nth(0).unwrap() {
                        // No staged changes, Added
                        ' ' | 'A' => {
                            choices += "\n";
                            choices += line[3..].green().to_string().as_str();
                        }
                        // Modified, Deleted, Renamed, Updated but merged
                        'M' | 'D' | 'R' | 'U' => {
                            choices += "\n";
                            choices += line[3..].yellow().to_string().as_str();
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
                    choices += "\n";
                    choices += line[3..].bright_yellow().to_string().as_str();
                }
                // Added
                'A' => {
                    choices += "\n";
                    choices += line[3..].bright_red().to_string().as_str();
                }
                _ => {
                    println!("ding: {}", line);
                }
            }
        }
        println!("your choices sir: \n{}", choices);
        let usr_selected = utils::strip_colors(cli::choice_no_limit(choices, true).unwrap());
        println!("You decided on: {}", usr_selected);
        Ok(())
    } else {
        return Err(String::from_utf8_lossy(&git_status.stderr).to_string());
    }
}
