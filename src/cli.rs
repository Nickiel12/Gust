use crate::commands::Commands;
use colored::Colorize;
use std::io::Write;
use std::process::{Command, Stdio};

pub fn filter_choice_cli(choices: String) -> Result<Commands, String> {
    let mut prompt_cmd = Command::new("gum")
        .arg("filter")
        .stdout(Stdio::piped())
        .stdin(Stdio::piped())
        .spawn()
        .expect("I'm out of gum!!!");

    let mut stdin = prompt_cmd.stdin.take().expect("failed to open stdin");
    std::thread::spawn(move || {
        stdin
            .write_all(choices.as_bytes())
            .expect("Failed to write to stdin");
    });

    let user_response = prompt_cmd.wait_with_output().unwrap();

    if user_response.status.success() {
        let response = String::from_utf8_lossy(&user_response.stdout)
            .to_string()
            .replace("\n", "");
        Commands::from_string(response)
    } else {
        return Err(String::from_utf8_lossy(&user_response.stderr).to_string());
    }
}

pub fn choice_no_limit(mut choices: String, has_none: bool) -> Result<String, String> {
    let mut prompt_cmd = Command::new("gum")
        .arg("choose")
        .arg("--no-limit")
        .stdout(Stdio::piped())
        .stdin(Stdio::piped())
        .spawn()
        .expect("I couldn't chose what gum to have!");

    let mut stdin = prompt_cmd.stdin.take().expect("failed to open stdin");
    if has_none {
        choices += "\n";
        choices += "None".bright_green().to_string().as_str();
    }
    std::thread::spawn(move || {
        stdin
            .write_all(choices.as_bytes())
            .expect("Failed to write to stdin");
    });

    let user_response = prompt_cmd.wait_with_output().unwrap();

    if user_response.status.success() {
        let response = String::from_utf8_lossy(&user_response.stdout).to_string();
        Ok(response)
    } else {
        Err(String::from_utf8_lossy(&user_response.stderr).to_string())
    }
}
