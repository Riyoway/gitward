//! GitHub CLI command entry points exposed to the frontend.

use crate::models::result::{run_blocking, CommandResult};
use crate::services::command::SystemRunner;
use crate::services::github_cli::{self, GhAccount};

#[tauri::command]
pub async fn gh_auth_status() -> CommandResult<Vec<GhAccount>> {
    run_blocking(|| CommandResult::from_result(github_cli::auth_status(&SystemRunner))).await
}

#[tauri::command]
pub async fn gh_auth_switch(username: String) -> CommandResult<()> {
    run_blocking(move || {
        CommandResult::from_result(github_cli::auth_switch(&SystemRunner, &username))
    })
    .await
}

#[tauri::command]
pub async fn gh_auth_setup_git() -> CommandResult<()> {
    run_blocking(|| CommandResult::from_result(github_cli::setup_git(&SystemRunner))).await
}
