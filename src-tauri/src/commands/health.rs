//! Health-check command entry point.

use crate::models::result::{run_blocking, CommandResult};
use crate::services::command::SystemRunner;
use crate::services::health::{self, HealthReport};

#[tauri::command]
pub async fn health_check() -> CommandResult<HealthReport> {
    run_blocking(|| CommandResult::ok(health::check(&SystemRunner))).await
}
