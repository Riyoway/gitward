//! Git command entry points exposed to the frontend.

use crate::models::result::CommandResult;
use crate::services::command::SystemRunner;
use crate::services::git::{self, GitIdentity, GitStatus};

#[tauri::command]
pub fn git_is_repo(path: String) -> bool {
    git::is_repo(&path)
}

#[tauri::command]
pub fn git_read_config(path: String) -> CommandResult<GitIdentity> {
    CommandResult::from_result(git::read_config(&SystemRunner, &path))
}

#[tauri::command]
pub fn git_set_config(path: String, user_name: String, email: String) -> CommandResult<()> {
    CommandResult::from_result(git::set_config(&SystemRunner, &path, &user_name, &email))
}

#[tauri::command]
pub fn git_status(path: String) -> CommandResult<GitStatus> {
    CommandResult::from_result(git::status(&SystemRunner, &path))
}
