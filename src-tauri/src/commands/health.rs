//! Health-check command entry point.

use crate::models::result::CommandResult;
use crate::services::command::SystemRunner;
use crate::services::health::{self, HealthReport};

#[tauri::command]
pub fn health_check() -> CommandResult<HealthReport> {
    CommandResult::ok(health::check(&SystemRunner))
}
