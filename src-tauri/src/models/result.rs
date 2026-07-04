//! The common result type crossing the IPC boundary.

use serde::Serialize;

use crate::error::AppError;

/// Every `#[tauri::command]` returns this, so the frontend can branch on
/// `success` and, on failure, surface `stdout`/`stderr`/`exitCode` to the log.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CommandResult<T> {
    pub success: bool,
    pub data: Option<T>,
    pub stdout: String,
    pub stderr: String,
    pub exit_code: Option<i32>,
    pub error: Option<String>,
}

impl<T> CommandResult<T> {
    pub fn ok(data: T) -> Self {
        Self {
            success: true,
            data: Some(data),
            stdout: String::new(),
            stderr: String::new(),
            exit_code: None,
            error: None,
        }
    }

    /// Convert a service result into the wire type, unpacking command failures
    /// so their output reaches the frontend.
    pub fn from_result(result: crate::error::AppResult<T>) -> Self {
        match result {
            Ok(data) => Self::ok(data),
            Err(err) => Self::from(err),
        }
    }
}

/// Run a blocking command body on the blocking thread pool so it never stalls
/// the main (UI) thread. Tauri runs non-async commands on the main thread, so
/// every command that shells out or touches the network must go through this.
pub async fn run_blocking<T, F>(f: F) -> CommandResult<T>
where
    F: FnOnce() -> CommandResult<T> + Send + 'static,
    T: Send + 'static,
{
    match tauri::async_runtime::spawn_blocking(f).await {
        Ok(result) => result,
        Err(err) => CommandResult {
            success: false,
            data: None,
            stdout: String::new(),
            stderr: String::new(),
            exit_code: None,
            error: Some(format!("task failed: {err}")),
        },
    }
}

impl<T> From<AppError> for CommandResult<T> {
    fn from(err: AppError) -> Self {
        let message = err.to_string();
        if let AppError::CommandFailed {
            stdout,
            stderr,
            code,
            ..
        } = err
        {
            Self {
                success: false,
                data: None,
                stdout,
                stderr,
                exit_code: code,
                error: Some(message),
            }
        } else {
            Self {
                success: false,
                data: None,
                stdout: String::new(),
                stderr: String::new(),
                exit_code: None,
                error: Some(message),
            }
        }
    }
}
