//! Push Guard command entry points exposed to the frontend.

use crate::models::result::{run_blocking, CommandResult};
use crate::services::command::SystemRunner;
use crate::services::guard::{self, GuardState};

#[tauri::command]
pub async fn guard_status(path: String) -> CommandResult<GuardState> {
    run_blocking(move || CommandResult::ok(guard::state(&path))).await
}

#[tauri::command]
pub async fn guard_install(
    path: String,
    user_name: String,
    email: String,
    gh_username: Option<String>,
) -> CommandResult<()> {
    run_blocking(move || {
        CommandResult::from_result(guard::install(
            &SystemRunner,
            &path,
            &user_name,
            &email,
            gh_username.as_deref().unwrap_or(""),
        ))
    })
    .await
}

#[tauri::command]
pub async fn guard_uninstall(path: String) -> CommandResult<()> {
    run_blocking(move || CommandResult::from_result(guard::uninstall(&path))).await
}
