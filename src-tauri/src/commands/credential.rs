//! Credential diagnosis command entry points.

use crate::models::result::CommandResult;
use crate::services::command::SystemRunner;
use crate::services::credential::{self, CredentialDiagnosis};

#[tauri::command]
pub fn credential_diagnose() -> CommandResult<CredentialDiagnosis> {
    CommandResult::from_result(credential::diagnose(&SystemRunner))
}
