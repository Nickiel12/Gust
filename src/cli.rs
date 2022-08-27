use crate::settings::Config;
use crate::utils;

use colored::Colorize;
use console;
use dialoguer::{theme, Confirm, FuzzySelect, MultiSelect, Select};
use std::process::{Command, Stdio};

#[derive(Debug)]
pub enum UserResponse<T> {
    All,
    Some(T),
    None,
}

pub fn ask_choice_cli(prompt: String) -> Result<bool, String> {
    match Confirm::new()
        .with_prompt(prompt)
        .interact_on_opt(&console::Term::stderr())
        .expect("Couldn't confirm if user wanted to stage all changed files")
    {
        Some(choice) => return Ok(choice),
        None => Err("An error occured, and no confirmation was gotten".to_string()),
    }
}

pub fn filter_choice_cli(
    mut choices: Vec<String>,
    allow_none: bool,
) -> Result<Option<String>, String> {
    if allow_none {
        choices.push("None".to_string());
    }
    let selection = FuzzySelect::with_theme(&theme::ColorfulTheme::default())
        .items(&choices)
        .with_prompt("Please choose a menu:")
        .default(1)
        .interact_on_opt(&console::Term::stderr())
        .expect("Couldn't fuzzy search");

    match selection {
        Some(index) => {
            if index == choices.len() {
                return Ok(None);
            } else {
                return Ok(Some(choices[index].to_string()));
            }
        }
        None => Err("No item was selected!".green().to_string()),
    }
}

pub fn choice_single (
    mut choices: Vec<String>,
    prompt: String,
    has_all: bool,
    has_none: bool,
) -> Result<UserResponse<String>, String> {
    if has_all {
        choices.insert(0, "All".to_string());
    }
    if has_none {
        choices.push("None".to_string());
    }

    let selected = Select::new()
        .items(&choices)
        .with_prompt(prompt)
        .interact_on_opt(&console::Term::stderr())
        .expect("Couldn't start `select`");

    match selected {
        None => Err("No items were selected".to_string()),
        Some(index) => {
            if has_all {
                if index == 0 {
                    return Ok(UserResponse::All);
                }
            }
            if has_none{
                if index == choices.len()-1 {
                    return Ok(UserResponse::None);
                }
            }
            return Ok(UserResponse::Some(utils::strip_colors(choices[index].to_string().to_owned())));
        }
    }
}


pub fn choice_no_limit(
    mut choices: Vec<String>,
    prompt: String,
    has_none: bool,
    has_all: bool,
) -> Result<UserResponse<Vec<String>>, String> {
    if has_all {
        choices.insert(0, "All".to_string());
    }
    if has_none {
        choices.push("None".to_string());
    }

    let selected: Option<Vec<usize>> = MultiSelect::new()
        .items(&choices)
        .with_prompt(prompt)
        .interact_on_opt(&console::Term::stderr())
        .expect("Couldn't make a choice");

    match selected {
        None => Err("No items were selected".to_string()),
        Some(indexes) => {
            let mut all_choices = Vec::<String>::new();
            all_choices.reserve(indexes.len());
            for i in indexes.into_iter() {
                all_choices.push(utils::strip_colors(choices[i].to_string()).to_owned());
            }
            if all_choices.contains(&choices[choices.len() - 1].to_string()) {
                Ok(UserResponse::None)
            } else if all_choices.contains(&choices[0].to_string()) {
                Ok(UserResponse::All)
            } else {
                Ok(UserResponse::Some(all_choices))
            }
        }
    }
}

pub fn git_status_short() -> Result<Option<String>, String> {
    let git_status_cmd = Command::new("git")
        .arg("status")
        .arg("--short")
        .stdout(Stdio::piped())
        .spawn()
        .expect("Couldn't call git add!");

    let git_status = git_status_cmd
        .wait_with_output()
        .map_err(|e| e.to_string())?;

    if git_status.status.success() {
        let status_output = String::from_utf8_lossy(&git_status.stdout).to_string();
        if status_output.len() == 1 || status_output.len() == 0 {
            Ok(None)
        } else {
            Ok(Some(status_output))
        }
    } else {
        Err(String::from_utf8_lossy(&git_status.stderr).to_string())
    }
}

pub fn git_pull() -> Result<(), String> {
    let git_pull_cmd = Command::new("git")
        .arg("pull")
        .spawn()
        .expect("Couldn't run `git pull`");

    let output = git_pull_cmd.wait_with_output().map_err(|e| e.to_string())?;

    if !output.status.success() {
        return Err(String::from_utf8_lossy(&output.stderr).to_string());
    } else {
        Ok(())
    }
}

pub fn git_push() -> Result<(), String> {
    let git_push_cmd = Command::new("git")
        .arg("push")
        .spawn()
        .expect("Couldn't run `git push`");

    let output = git_push_cmd.wait_with_output().map_err(|e| e.to_string())?;

    if !output.status.success() {
        return Err(String::from_utf8_lossy(&output.stderr).to_string());
    } else {
        Ok(())
    }
}

pub fn git_checkout(files: Vec<String>) -> Result<(), String> {
    let git_checkout_cmd = Command::new("git")
        .arg("checkout")
        .args(files)
        .spawn()
        .expect("Couldn't run `git checkout`");

    let output = git_checkout_cmd
        .wait_with_output()
        .map_err(|e| e.to_string())?;

    if !output.status.success() {
        return Err(String::from_utf8_lossy(&output.stderr).to_string());
    } else {
        Ok(())
    }
}

pub fn git_add(input: Vec<String>) -> Result<(), String> {
    let git_add_cmd = Command::new("git")
        .arg("add")
        .args(input)
        .spawn()
        .expect("Couldn't run `git add`");

    let output = git_add_cmd.wait_with_output().map_err(|e| e.to_string())?;

    if !output.status.success() {
        return Err(String::from_utf8_lossy(&output.stderr).to_string());
    } else {
        println!("{}", "Files Staged!".bright_green());
        Ok(())
    }
}

pub fn git_reset(input: Vec<String>) -> Result<(), String> {
    let git_reset_cmd = Command::new("git")
        .arg("reset")
        .args(input)
        .spawn()
        .expect("Couldn't run `git reset`");

    let output = git_reset_cmd
        .wait_with_output()
        .map_err(|e| e.to_string())?;

    if !output.status.success() {
        return Err(String::from_utf8_lossy(&output.stderr).to_string());
    } else {
        Ok(())
    }
}

pub fn git_commit(passed_options: Option<Vec<String>>, config: &Config) -> Result<(), String> {
    let git_commit_cmd;
    let mut options: Vec<String> = vec![];

    if passed_options.is_some() {
        options = passed_options.unwrap();
    }

    if config.verbose_commit {
        options.push("-v".to_string());
    }

    // for debugging
    let opts = options.join(" ");

    git_commit_cmd = Command::new("git")
        .arg("commit")
        .args(options)
        .spawn()
        .expect(format!("Couldn't call `git commit {}`!", opts).as_str());

    let output = git_commit_cmd
        .wait_with_output()
        .map_err(|e| e.to_string())?;

    if !output.status.success() {
        return Err(String::from_utf8_lossy(&output.stderr).to_string());
    }
    Ok(())
}
