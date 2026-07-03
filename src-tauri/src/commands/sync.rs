//! The sync command entry point.

use crate::models::result::CommandResult;
use crate::services::command::SystemRunner;
use crate::services::sync::{self, SyncReport};

#[tauri::command]
pub fn sync_repository(
    path: String,
    user_name: String,
    email: String,
    gh_username: Option<String>,
) -> CommandResult<SyncReport> {
    let report = sync::sync_repository(
        &SystemRunner,
        &path,
        &user_name,
        &email,
        gh_username.as_deref(),
    );
    CommandResult::ok(report)
}
