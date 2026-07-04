//! Git command entry points exposed to the frontend.

use crate::models::result::{run_blocking, CommandResult};
use crate::services::command::SystemRunner;
use crate::services::git::{self, GitIdentity, GitStatus};

#[tauri::command]
pub async fn git_is_repo(path: String) -> bool {
    tauri::async_runtime::spawn_blocking(move || git::is_repo(&path))
        .await
        .unwrap_or(false)
}

#[tauri::command]
pub async fn git_read_config(path: String) -> CommandResult<GitIdentity> {
    run_blocking(move || CommandResult::from_result(git::read_config(&SystemRunner, &path))).await
}

#[tauri::command]
pub async fn git_set_config(path: String, user_name: String, email: String) -> CommandResult<()> {
    run_blocking(move || {
        CommandResult::from_result(git::set_config(&SystemRunner, &path, &user_name, &email))
    })
    .await
}

#[tauri::command]
pub async fn git_status(path: String) -> CommandResult<GitStatus> {
    run_blocking(move || CommandResult::from_result(git::status(&SystemRunner, &path))).await
}
