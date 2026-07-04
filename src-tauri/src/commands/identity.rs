//! App Identity command entry points.

use crate::models::result::CommandResult;
use crate::services::identity::{self, IdentityField, IdentityFieldValue};

#[tauri::command]
pub fn detect_framework(path: String) -> CommandResult<Vec<String>> {
    CommandResult::ok(identity::detect_frameworks(&path))
}

#[tauri::command]
pub fn read_identity(path: String) -> CommandResult<Vec<IdentityField>> {
    CommandResult::ok(identity::read_identity(&path))
}

#[tauri::command]
pub fn write_identity(path: String, fields: Vec<IdentityFieldValue>) -> CommandResult<()> {
    CommandResult::from_result(identity::write_identity(&path, &fields))
}
