//! The sync command entry point.

use crate::models::result::{run_blocking, CommandResult};
use crate::services::command::SystemRunner;
use crate::services::sync::{self, SyncReport};

#[tauri::command]
pub async fn sync_repository(
    path: String,
    user_name: String,
    email: String,
    gh_username: Option<String>,
) -> CommandResult<SyncReport> {
    run_blocking(move || {
        let report = sync::sync_repository(
            &SystemRunner,
            &path,
            &user_name,
            &email,
            gh_username.as_deref(),
        );
        CommandResult::ok(report)
    })
    .await
}
