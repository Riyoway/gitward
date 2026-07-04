//! Detecting installed developer tools and launching them at a repo path.

use serde::Serialize;

use crate::error::{AppError, AppResult};
use crate::services::command::{CommandOutput, CommandRunner};

/// A known tool and whether it is installed on this machine.
#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Tool {
    pub id: String,
    pub name: String,
    pub category: String,
    pub installed: bool,
}

struct ToolDef {
    id: &'static str,
    name: &'static str,
    category: &'static str,
    /// Executable on PATH (a `.cmd` shim on Windows for editors).
    program: &'static str,
    /// Launch arguments; `{path}` is replaced with the repository path.
    launch_args: &'static [&'static str],
}

const TOOLS: &[ToolDef] = &[
    ToolDef {
        id: "vscode",
        name: "VS Code",
        category: "editor",
        program: "code",
        launch_args: &["{path}"],
    },
    ToolDef {
        id: "cursor",
        name: "Cursor",
        category: "editor",
        program: "cursor",
        launch_args: &["{path}"],
    },
    ToolDef {
        id: "idea",
        name: "IntelliJ IDEA",
        category: "editor",
        program: "idea",
        launch_args: &["{path}"],
    },
    ToolDef {
        id: "sublime",
        name: "Sublime Text",
        category: "editor",
        program: "subl",
        launch_args: &["{path}"],
    },
    ToolDef {
        id: "wt",
        name: "Windows Terminal",
        category: "terminal",
        program: "wt",
        launch_args: &["-d", "{path}"],
    },
];

/// A resolved command to spawn.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Invocation {
    pub program: String,
    pub args: Vec<String>,
}

/// Probe each known tool with `--version` and report which are installed.
pub fn detect_tools(runner: &dyn CommandRunner) -> Vec<Tool> {
    TOOLS
        .iter()
        .map(|t| Tool {
            id: t.id.to_string(),
            name: t.name.to_string(),
            category: t.category.to_string(),
            installed: probe(runner, t.program)
                .map(|o| o.is_success())
                .unwrap_or(false),
        })
        .collect()
}

/// Build the launch invocation for a tool, substituting the repo path.
pub fn resolve_launch(tool_id: &str, path: &str) -> AppResult<Invocation> {
    let tool = TOOLS
        .iter()
        .find(|t| t.id == tool_id)
        .ok_or_else(|| AppError::ToolNotFound(tool_id.to_string()))?;
    Ok(Invocation {
        program: tool.program.to_string(),
        args: tool
            .launch_args
            .iter()
            .map(|a| a.replace("{path}", path))
            .collect(),
    })
}

// Detect by resolving the executable on PATH, never by launching it — running a
// tool (e.g. `wt --version`) can pop a GUI dialog and block. `where`/`command -v`
// just report whether the program exists and exit fast.
#[cfg(windows)]
fn probe(runner: &dyn CommandRunner, program: &str) -> AppResult<CommandOutput> {
    runner.run("where", &[program], None)
}

#[cfg(not(windows))]
fn probe(runner: &dyn CommandRunner, program: &str) -> AppResult<CommandOutput> {
    runner.run("sh", &["-c", &format!("command -v {program}")], None)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn resolve_launch_substitutes_path() {
        let inv = resolve_launch("vscode", "C:/repo").unwrap();
        assert_eq!(inv.program, "code");
        assert_eq!(inv.args, vec!["C:/repo"]);
    }

    #[test]
    fn resolve_launch_substitutes_in_flagged_args() {
        let inv = resolve_launch("wt", "/home/ada/repo").unwrap();
        assert_eq!(inv.args, vec!["-d", "/home/ada/repo"]);
    }

    #[test]
    fn resolve_launch_rejects_unknown_tool() {
        assert!(matches!(
            resolve_launch("nope", "/x"),
            Err(AppError::ToolNotFound(_))
        ));
    }
}
