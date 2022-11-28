use crate::settings::Config;
use crate::utils;

use colored::Colorize;
use console;
use dialoguer::{theme, Confirm, FuzzySelect, Input, MultiSelect, Select};
use std::process::{Command, Stdio};

#[derive(Debug)]
pub enum UserResponse<T> {
    All,
    Some(T),
    None,
}

pub fn get_input(prompt: String) -> Result<String, String> {
    Input::<String>::new()
        .with_prompt(prompt)
        .interact_text_on(&console::Term::stderr())
        .map_err(|e| e.to_string())
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

pub fn ask_yes_no(prompt: String, default_yes: bool) -> Result<bool, String> {
    let default = if default_yes { "Y" } else { "N" };
    let user_input: String = Input::new()
        .with_prompt(format!("{} (y/n)", prompt).to_string())
        .default(default.into())
        .interact_text()
        .expect("Couldn't ask a yes or no question");

    match user_input.chars().nth(0).unwrap() {
        'y' | 'Y' => Ok(true),
        _ => Ok(false),
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

pub fn choice_single(
    mut choices: Vec<String>,
    prompt: String,
    has_all: bool,
    has_none: bool,
) -> Result<UserResponse<usize>, String> {
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
            if has_none {
                if index == choices.len() - 1 {
                    return Ok(UserResponse::None);
                }
            }
            return Ok(UserResponse::Some(index));
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

pub fn git_get_branches() -> Result<Option<Vec<String>>, String> {
    let cmd = Command::new("git")
        .arg("branch")
        .arg("-a")
        .stdout(Stdio::piped())
        .spawn()
        .expect("Couldn't call git add!");

    let output = cmd.wait_with_output().map_err(|e| e.to_string())?;

    if output.status.success() {
        let branches: Vec<String> = String::from_utf8_lossy(&output.stdout)
            .to_string()
            .split("\n")
            .filter_map(|i| {
                if i.contains("HEAD") {
                    None
                } else {
                    if i.len() > 2 {
                        Some(i[2..].to_string())
                    } else {
                        None
                    }
                }
            })
            .collect();

        Ok(Some(utils::strip_vec_colors(branches)))
    } else {
        Err(String::from_utf8_lossy(&output.stderr).to_string())
    }
}

pub fn git_fetch() -> Result<(), String> {
    Command::new("git")
        .arg("fetch")
        .arg("--all")
        .spawn()
        .map_err(|e| e.to_string())?
        .wait()
        .map_err(|e| e.to_string())?;

    Ok(())
}

pub fn git_log(branch: Option<String>) -> Result<Option<String>, String> {
    let mut args = vec![];
    
    if branch.is_some() {
        args.push(branch.unwrap());
    }

    let git_log_cmd = Command::new("git")
        .arg("log")
        .arg("--oneline")
        .args(args)
        .stdout(Stdio::piped())
        .spawn()
        .expect("Couldn't call git log --oneline!");
    
    let git_log_cmd = Command::new("git")
        .arg("log")
        .arg("--oneline")
        .args(args)
        .stdout(Stdio::piped())
        .spawn()
        .expect("Couldn't call git log --oneline!");

    let git_log = git_log_cmd.wait_with_output().map_err(|e| e.to_string())?;

    if git_log.status.success() {
        let log_output = String::from_utf8_lossy(&git_log.stdout).to_string();
        if log_output.len() == 1 || log_output.len() == 0 {
            Ok(None)
        } else {
            Ok(Some(log_output))
        }
    } else {
        Err(String::from_utf8_lossy(&git_log.stderr).to_string())
    }
}

pub fn git_create_branch(
    new_branch: String,
    starting_commit_hash: Option<String>,
) -> Result<(), String> {
    let mut args = vec![];

    if starting_commit_hash.is_some() {
        args.push(starting_commit_hash.unwrap());
    }

    let git_create_branch_cmd = Command::new("git")
        .arg("branch")
        .args(args)
        .spawn()
        .expect("Coun't create new branch");

    let git_create_branch_cmd = Command::new("git")
        .arg("branch")
        .args(args)
        .spawn()
        .expect("Coun't create new branch");

    let output = git_create_branch_cmd
        .wait_with_output()
        .map_err(|e| e.to_string())?;

    if !output.status.success() {
        return Err(String::from_utf8_lossy(&output.stderr).to_string());
    } else {
        Ok(())
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

pub fn git_revert(commit_hash: String) -> Result<(), String> {
    let git_revert_cmd = Command::new("git")
        .arg("revert")
        .arg(commit_hash)
        .spawn()
        .expect("Couldn't run `git reset`");

    let output = git_revert_cmd
        .wait_with_output()
        .map_err(|e| e.to_string())?;

    if !output.status.success() {
        return Err(String::from_utf8_lossy(&output.stderr).to_string());
    } else {
        Ok(())
    }
}

pub fn git_ls_tree() -> Result<Option<String>, String> {
    let git_ls_tree_cmd = Command::new("git")
        .arg("ls-tree")
        .arg("--full-tree")
        .arg("-r")
        .arg("--name-only")
        .arg("HEAD")
        .stdout(Stdio::piped())
        .spawn()
        .expect("Couldn't run `git ls-tree`");

    let git_output = git_ls_tree_cmd
        .wait_with_output()
        .map_err(|e| e.to_string())?;

    if git_output.status.success() {
        let output = String::from_utf8_lossy(&git_output.stdout).to_string();
        if output.len() <= 1 {
            Ok(None)
        } else {
            Ok(Some(output))
        }
    } else {
        Err(String::from_utf8_lossy(&git_output.stderr).to_string())
    }
}

pub fn git_rm(files: Vec<String>, as_cached: bool) -> Result<(), String> {
    let mut args = vec![];

    if as_cached {
        args.push(String::from("--cached"));
    }

    args.append(&mut files.clone());

    let git_rm_cmd = Command::new("git")
        .arg("rm")
        .args(args)
        .spawn()
        .expect("Couldn't run `git add`");

    let output = git_rm_cmd.wait_with_output().map_err(|e| e.to_string())?;

    if !output.status.success() {
        return Err(String::from_utf8_lossy(&output.stderr).to_string());
    } else {
        println!("{}", "Files no longer tracking!".bright_green());
        Ok(())
    }
}
