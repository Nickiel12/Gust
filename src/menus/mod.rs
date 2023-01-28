mod add_cli;
mod branches_cli;
mod commit_cli;
mod main_menu;
mod remove_cli;
mod reset_cli;
mod undo_commit_cli;

mod cli_menus;

pub use add_cli::git_add_cli;
pub use branches_cli::git_branches_cli;
pub use commit_cli::git_commit_cli;
pub use main_menu::main_menu;
pub use remove_cli::git_remove_cli;
pub use reset_cli::git_reset_cli;
pub use undo_commit_cli::git_undo_commit_cli;

pub use cli_menus::{advanced_menu, basic_menu, git_pull_cli, git_push_cli};
