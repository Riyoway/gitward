//! Launcher command entry points: detect tools and open a repo in one.

use std::process::Command;

use crate::error::{AppError, AppResult};
use crate::models::result::CommandResult;
use crate::services::command::SystemRunner;
use crate::services::launcher::{self, Invocation, Tool};

#[tauri::command]
pub fn detect_tools() -> CommandResult<Vec<Tool>> {
    CommandResult::ok(launcher::detect_tools(&SystemRunner))
}

#[tauri::command]
pub fn launch_tool(tool_id: String, path: String) -> CommandResult<()> {
    CommandResult::from_result(spawn_tool(&tool_id, &path))
}

fn spawn_tool(tool_id: &str, path: &str) -> AppResult<()> {
    let inv = launcher::resolve_launch(tool_id, path)?;
    // Fire and forget: don't wait, so GUI tools that stay open never block us.
    build_command(&inv)
        .spawn()
        .map_err(|source| AppError::Spawn {
            program: inv.program.clone(),
            source,
        })?;
    Ok(())
}

#[cfg(windows)]
fn build_command(inv: &Invocation) -> Command {
    use std::os::windows::process::CommandExt;
    const CREATE_NO_WINDOW: u32 = 0x0800_0000;
    let mut cmd = Command::new("cmd");
    cmd.arg("/C").arg(&inv.program).args(&inv.args);
    cmd.creation_flags(CREATE_NO_WINDOW);
    cmd
}

#[cfg(not(windows))]
fn build_command(inv: &Invocation) -> Command {
    let mut cmd = Command::new(&inv.program);
    cmd.args(&inv.args);
    cmd
}
