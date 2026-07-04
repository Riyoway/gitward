//! Minimal file read/write for import/export of user data.
//!
//! Paths come from the OS file dialog (user-chosen), so these back the
//! Export/Import buttons rather than exposing arbitrary filesystem access.

use std::fs;

use crate::error::AppError;
use crate::models::result::{run_blocking, CommandResult};

#[tauri::command]
pub async fn write_text_file(path: String, contents: String) -> CommandResult<()> {
    run_blocking(move || {
        CommandResult::from_result(fs::write(&path, contents).map_err(AppError::from))
    })
    .await
}

#[tauri::command]
pub async fn read_text_file(path: String) -> CommandResult<String> {
    run_blocking(move || {
        CommandResult::from_result(fs::read_to_string(&path).map_err(AppError::from))
    })
    .await
}
