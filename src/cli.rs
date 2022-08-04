use crate::commands::Commands;
use std::process::Command;

pub fn get_git_action() -> Option<Commands> {
    let prompt_cmd = Command::new("gum")
        .arg("filter")
        .arg(Commands::Add.to_string())
        .arg(Commands::Reset.to_string())
        .arg(Commands::Commit.to_string())
        .spawn()
        .unwrap();
        //.expect("I'm out of gum!!!");

    let user_response = prompt_cmd.wait_with_output().unwrap();    

    if user_response.status.success() {
        let response = String::from_utf8_lossy(&user_response.stdout);
        print!("{}", response);
        //return Commands::get_from_string(response);
    } else {
        print!("choose command failed");
    }
    return None;
}
