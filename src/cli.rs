use crate::commands::Commands;
use std::{process::Command, str};

pub fn get_git_action() -> Option<Commands> {
    let user_response = Command::new("gum")
        .arg("filter")
        .arg(Commands::Add.to_string())
        .arg(Commands::Reset.to_string())
        .arg(Commands::Commit.to_string())
        .output()
        .expect("I'm out of gum!!!");

    match (str::from_utf8(&user_response.stdout)) {
        Ok(value) => return Commands::get_from_string(value.to_string()),
        Err(_) => return None,
    }
}
