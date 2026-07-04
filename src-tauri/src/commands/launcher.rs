//! Launcher command entry points: detect tools and open a repo in one.

use std::process::Command;

use crate::error::{AppError, AppResult};
use crate::models::result::{run_blocking, CommandResult};
use crate::services::command::SystemRunner;
use crate::services::launcher::{self, Tool};

#[tauri::command]
pub async fn detect_tools() -> CommandResult<Vec<Tool>> {
    run_blocking(|| CommandResult::ok(launcher::detect_tools(&SystemRunner))).await
}

#[tauri::command]
pub async fn launch_tool(tool_id: String, path: String) -> CommandResult<()> {
    run_blocking(move || CommandResult::from_result(spawn_tool(&tool_id, &path))).await
}

fn spawn_tool(tool_id: &str, path: &str) -> AppResult<()> {
    let inv = launcher::resolve_launch(tool_id, path)?;

    let mut cmd = Command::new(&inv.program);
    cmd.args(&inv.args);
    if let Some(dir) = &inv.cwd {
        cmd.current_dir(dir);
    }

    // Hide the intermediate console on Windows; the launched GUI/terminal still
    // shows. Fire and forget so GUI tools that stay open never block us.
    #[cfg(windows)]
    {
        use std::os::windows::process::CommandExt;
        const CREATE_NO_WINDOW: u32 = 0x0800_0000;
        cmd.creation_flags(CREATE_NO_WINDOW);
    }

    cmd.spawn().map_err(|source| AppError::Spawn {
        program: inv.program.clone(),
        source,
    })?;
    Ok(())
}
