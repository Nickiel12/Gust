use crate::commands::Commands;
use std::io::Write;
use std::process::{Command, Stdio};

pub fn get_git_action() -> Result<Commands, String> {
    let mut prompt_cmd = Command::new("gum")
        .arg("filter")
        .stdout(Stdio::piped())
        .stdin(Stdio::piped())
        .spawn()
        .expect("I'm out of gum!!!");

    let mut stdin = prompt_cmd.stdin.take().expect("failed to open stdin");
    std::thread::spawn(move || {
        stdin
            .write_all(Commands::get_gum_string().as_bytes())
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
