//! Credential diagnosis command entry points.

use crate::models::result::{run_blocking, CommandResult};
use crate::services::command::SystemRunner;
use crate::services::credential::{self, CredentialDiagnosis};

#[tauri::command]
pub async fn credential_diagnose() -> CommandResult<CredentialDiagnosis> {
    run_blocking(|| CommandResult::from_result(credential::diagnose(&SystemRunner))).await
}
