//! GitHub CLI command entry points exposed to the frontend.

use crate::models::result::CommandResult;
use crate::services::command::SystemRunner;
use crate::services::github_cli::{self, GhAccount};

#[tauri::command]
pub fn gh_auth_status() -> CommandResult<Vec<GhAccount>> {
    CommandResult::from_result(github_cli::auth_status(&SystemRunner))
}

#[tauri::command]
pub fn gh_auth_switch(username: String) -> CommandResult<()> {
    CommandResult::from_result(github_cli::auth_switch(&SystemRunner, &username))
}

#[tauri::command]
pub fn gh_auth_setup_git() -> CommandResult<()> {
    CommandResult::from_result(github_cli::setup_git(&SystemRunner))
}
