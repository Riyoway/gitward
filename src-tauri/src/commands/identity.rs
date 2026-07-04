//! App Identity command entry points.

use crate::models::result::{run_blocking, CommandResult};
use crate::services::identity::{self, IdentityField, IdentityFieldValue};

#[tauri::command]
pub async fn detect_framework(path: String) -> CommandResult<Vec<String>> {
    run_blocking(move || CommandResult::ok(identity::detect_frameworks(&path))).await
}

#[tauri::command]
pub async fn read_identity(path: String) -> CommandResult<Vec<IdentityField>> {
    run_blocking(move || CommandResult::ok(identity::read_identity(&path))).await
}

#[tauri::command]
pub async fn write_identity(path: String, fields: Vec<IdentityFieldValue>) -> CommandResult<()> {
    run_blocking(move || CommandResult::from_result(identity::write_identity(&path, &fields))).await
}
