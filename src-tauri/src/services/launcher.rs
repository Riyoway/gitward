//! Detecting installed developer tools and launching them at a repo path.
//!
//! Cross-platform (Windows/macOS/Linux). Detection resolves an executable on
//! PATH (`where` / `command -v`) or, on macOS, an `.app` bundle; it NEVER runs
//! the tool, since some (e.g. `wt --version`) pop a GUI dialog. Launch is
//! per-OS: GUI apps open with the path, terminals open at the directory, and AI
//! CLIs open in a new terminal at the repo.

use serde::Serialize;

use crate::error::{AppError, AppResult};
use crate::services::command::CommandRunner;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Os {
    Windows,
    Macos,
    Linux,
}

#[cfg(target_os = "windows")]
const CURRENT_OS: Os = Os::Windows;
#[cfg(target_os = "macos")]
const CURRENT_OS: Os = Os::Macos;
#[cfg(all(unix, not(target_os = "macos")))]
const CURRENT_OS: Os = Os::Linux;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum LaunchKind {
    /// A GUI app or a terminal opened at a directory (launch_args carry {path}).
    Spawn,
    /// A CLI tool run inside a new terminal window opened at the repo.
    InTerminal,
}

struct ToolDef {
    id: &'static str,
    name: &'static str,
    category: &'static str,
    /// Executables to look for on PATH, in priority order.
    commands: &'static [&'static str],
    /// macOS `.app` bundle name (without `.app`) for GUI apps not on PATH.
    #[cfg_attr(not(target_os = "macos"), allow(dead_code))]
    mac_app: Option<&'static str>,
    /// Operating systems this tool is relevant to.
    platforms: &'static [Os],
    launch_kind: LaunchKind,
    /// For `Spawn`: arguments with `{path}` substituted to the repo path.
    launch_args: &'static [&'static str],
}

const TOOLS: &[ToolDef] = &[
    ToolDef {
        id: "vscode",
        name: "Visual Studio Code",
        category: "editor",
        commands: &["code"],
        mac_app: Some("Visual Studio Code"),
        platforms: &[Os::Windows, Os::Macos, Os::Linux],
        launch_kind: LaunchKind::Spawn,
        launch_args: &["{path}"],
    },
    ToolDef {
        id: "vscode-insiders",
        name: "Visual Studio Code Insiders",
        category: "editor",
        commands: &["code-insiders"],
        mac_app: Some("Visual Studio Code - Insiders"),
        platforms: &[Os::Windows, Os::Macos, Os::Linux],
        launch_kind: LaunchKind::Spawn,
        launch_args: &["{path}"],
    },
    ToolDef {
        id: "vscodium",
        name: "VSCodium",
        category: "editor",
        commands: &["codium"],
        mac_app: Some("VSCodium"),
        platforms: &[Os::Windows, Os::Macos, Os::Linux],
        launch_kind: LaunchKind::Spawn,
        launch_args: &["{path}"],
    },
    ToolDef {
        id: "cursor",
        name: "Cursor",
        category: "editor",
        commands: &["cursor"],
        mac_app: Some("Cursor"),
        platforms: &[Os::Windows, Os::Macos, Os::Linux],
        launch_kind: LaunchKind::Spawn,
        launch_args: &["{path}"],
    },
    ToolDef {
        id: "windsurf",
        name: "Windsurf",
        category: "editor",
        commands: &["windsurf"],
        mac_app: Some("Windsurf"),
        platforms: &[Os::Windows, Os::Macos, Os::Linux],
        launch_kind: LaunchKind::Spawn,
        launch_args: &["{path}"],
    },
    ToolDef {
        id: "zed",
        name: "Zed",
        category: "editor",
        commands: &["zed"],
        mac_app: Some("Zed"),
        platforms: &[Os::Windows, Os::Macos, Os::Linux],
        launch_kind: LaunchKind::Spawn,
        launch_args: &["{path}"],
    },
    ToolDef {
        id: "sublime-text",
        name: "Sublime Text",
        category: "editor",
        commands: &["subl"],
        mac_app: Some("Sublime Text"),
        platforms: &[Os::Windows, Os::Macos, Os::Linux],
        launch_kind: LaunchKind::Spawn,
        launch_args: &["{path}"],
    },
    ToolDef {
        id: "intellij-idea",
        name: "IntelliJ IDEA",
        category: "editor",
        commands: &["idea"],
        mac_app: Some("IntelliJ IDEA"),
        platforms: &[Os::Windows, Os::Macos, Os::Linux],
        launch_kind: LaunchKind::Spawn,
        launch_args: &["{path}"],
    },
    ToolDef {
        id: "pycharm",
        name: "PyCharm",
        category: "editor",
        commands: &["pycharm"],
        mac_app: Some("PyCharm"),
        platforms: &[Os::Windows, Os::Macos, Os::Linux],
        launch_kind: LaunchKind::Spawn,
        launch_args: &["{path}"],
    },
    ToolDef {
        id: "webstorm",
        name: "WebStorm",
        category: "editor",
        commands: &["webstorm"],
        mac_app: Some("WebStorm"),
        platforms: &[Os::Windows, Os::Macos, Os::Linux],
        launch_kind: LaunchKind::Spawn,
        launch_args: &["{path}"],
    },
    ToolDef {
        id: "goland",
        name: "GoLand",
        category: "editor",
        commands: &["goland"],
        mac_app: Some("GoLand"),
        platforms: &[Os::Windows, Os::Macos, Os::Linux],
        launch_kind: LaunchKind::Spawn,
        launch_args: &["{path}"],
    },
    ToolDef {
        id: "clion",
        name: "CLion",
        category: "editor",
        commands: &["clion"],
        mac_app: Some("CLion"),
        platforms: &[Os::Windows, Os::Macos, Os::Linux],
        launch_kind: LaunchKind::Spawn,
        launch_args: &["{path}"],
    },
    ToolDef {
        id: "phpstorm",
        name: "PhpStorm",
        category: "editor",
        commands: &["phpstorm"],
        mac_app: Some("PhpStorm"),
        platforms: &[Os::Windows, Os::Macos, Os::Linux],
        launch_kind: LaunchKind::Spawn,
        launch_args: &["{path}"],
    },
    ToolDef {
        id: "rubymine",
        name: "RubyMine",
        category: "editor",
        commands: &["rubymine"],
        mac_app: Some("RubyMine"),
        platforms: &[Os::Windows, Os::Macos, Os::Linux],
        launch_kind: LaunchKind::Spawn,
        launch_args: &["{path}"],
    },
    ToolDef {
        id: "rider",
        name: "Rider",
        category: "editor",
        commands: &["rider"],
        mac_app: Some("Rider"),
        platforms: &[Os::Windows, Os::Macos, Os::Linux],
        launch_kind: LaunchKind::Spawn,
        launch_args: &["{path}"],
    },
    ToolDef {
        id: "rustrover",
        name: "RustRover",
        category: "editor",
        commands: &["rustrover"],
        mac_app: Some("RustRover"),
        platforms: &[Os::Windows, Os::Macos, Os::Linux],
        launch_kind: LaunchKind::Spawn,
        launch_args: &["{path}"],
    },
    ToolDef {
        id: "datagrip",
        name: "DataGrip",
        category: "editor",
        commands: &["datagrip"],
        mac_app: Some("DataGrip"),
        platforms: &[Os::Windows, Os::Macos, Os::Linux],
        launch_kind: LaunchKind::Spawn,
        launch_args: &["{path}"],
    },
    ToolDef {
        id: "android-studio",
        name: "Android Studio",
        category: "editor",
        commands: &["studio"],
        mac_app: Some("Android Studio"),
        platforms: &[Os::Windows, Os::Macos, Os::Linux],
        launch_kind: LaunchKind::Spawn,
        launch_args: &["{path}"],
    },
    ToolDef {
        id: "xcode",
        name: "Xcode",
        category: "editor",
        commands: &["xed"],
        mac_app: Some("Xcode"),
        platforms: &[Os::Macos],
        launch_kind: LaunchKind::Spawn,
        launch_args: &["{path}"],
    },
    ToolDef {
        id: "neovide",
        name: "Neovide (Neovim GUI)",
        category: "editor",
        commands: &["neovide"],
        mac_app: Some("Neovide"),
        platforms: &[Os::Windows, Os::Macos, Os::Linux],
        launch_kind: LaunchKind::Spawn,
        launch_args: &["{path}"],
    },
    ToolDef {
        id: "emacs",
        name: "Emacs",
        category: "editor",
        commands: &["emacs"],
        mac_app: Some("Emacs"),
        platforms: &[Os::Windows, Os::Macos, Os::Linux],
        launch_kind: LaunchKind::Spawn,
        launch_args: &["{path}"],
    },
    ToolDef {
        id: "positron",
        name: "Positron",
        category: "editor",
        commands: &["positron"],
        mac_app: Some("Positron"),
        platforms: &[Os::Windows, Os::Macos, Os::Linux],
        launch_kind: LaunchKind::Spawn,
        launch_args: &["{path}"],
    },
    ToolDef {
        id: "trae",
        name: "Trae",
        category: "editor",
        commands: &["trae"],
        mac_app: Some("Trae"),
        platforms: &[Os::Windows, Os::Macos],
        launch_kind: LaunchKind::Spawn,
        launch_args: &["{path}"],
    },
    ToolDef {
        id: "kiro",
        name: "Kiro",
        category: "editor",
        commands: &["kiro"],
        mac_app: Some("Kiro"),
        platforms: &[Os::Windows, Os::Macos, Os::Linux],
        launch_kind: LaunchKind::Spawn,
        launch_args: &["{path}"],
    },
    ToolDef {
        id: "lapce",
        name: "Lapce",
        category: "editor",
        commands: &["lapce"],
        mac_app: Some("Lapce"),
        platforms: &[Os::Windows, Os::Macos, Os::Linux],
        launch_kind: LaunchKind::Spawn,
        launch_args: &["{path}"],
    },
    ToolDef {
        id: "pulsar",
        name: "Pulsar",
        category: "editor",
        commands: &["pulsar"],
        mac_app: Some("Pulsar"),
        platforms: &[Os::Windows, Os::Macos, Os::Linux],
        launch_kind: LaunchKind::Spawn,
        launch_args: &["{path}"],
    },
    ToolDef {
        id: "nova",
        name: "Nova",
        category: "editor",
        commands: &["nova"],
        mac_app: Some("Nova"),
        platforms: &[Os::Macos],
        launch_kind: LaunchKind::Spawn,
        launch_args: &["{path}"],
    },
    ToolDef {
        id: "bbedit",
        name: "BBEdit",
        category: "editor",
        commands: &["bbedit"],
        mac_app: Some("BBEdit"),
        platforms: &[Os::Macos],
        launch_kind: LaunchKind::Spawn,
        launch_args: &["{path}"],
    },
    ToolDef {
        id: "textmate",
        name: "TextMate",
        category: "editor",
        commands: &["mate"],
        mac_app: Some("TextMate"),
        platforms: &[Os::Macos],
        launch_kind: LaunchKind::Spawn,
        launch_args: &["{path}"],
    },
    ToolDef {
        id: "kate",
        name: "Kate",
        category: "editor",
        commands: &["kate"],
        mac_app: None,
        platforms: &[Os::Windows, Os::Linux],
        launch_kind: LaunchKind::Spawn,
        launch_args: &["{path}"],
    },
    ToolDef {
        id: "windows-terminal",
        name: "Windows Terminal",
        category: "terminal",
        commands: &["wt"],
        mac_app: None,
        platforms: &[Os::Windows],
        launch_kind: LaunchKind::Spawn,
        launch_args: &["-d", "{path}"],
    },
    ToolDef {
        id: "git-bash",
        name: "Git Bash",
        category: "terminal",
        commands: &["git-bash"],
        mac_app: None,
        platforms: &[Os::Windows],
        launch_kind: LaunchKind::Spawn,
        launch_args: &["--cd={path}"],
    },
    ToolDef {
        id: "iterm2",
        name: "iTerm2",
        category: "terminal",
        commands: &[],
        mac_app: Some("iTerm"),
        platforms: &[Os::Macos],
        launch_kind: LaunchKind::Spawn,
        launch_args: &["{path}"],
    },
    ToolDef {
        id: "apple-terminal",
        name: "Terminal (macOS)",
        category: "terminal",
        commands: &[],
        mac_app: Some("Terminal"),
        platforms: &[Os::Macos],
        launch_kind: LaunchKind::Spawn,
        launch_args: &["{path}"],
    },
    ToolDef {
        id: "ghostty",
        name: "Ghostty",
        category: "terminal",
        commands: &["ghostty"],
        mac_app: Some("Ghostty"),
        platforms: &[Os::Macos, Os::Linux],
        launch_kind: LaunchKind::Spawn,
        launch_args: &["--working-directory={path}"],
    },
    ToolDef {
        id: "wezterm",
        name: "WezTerm",
        category: "terminal",
        commands: &["wezterm"],
        mac_app: Some("WezTerm"),
        platforms: &[Os::Windows, Os::Macos, Os::Linux],
        launch_kind: LaunchKind::Spawn,
        launch_args: &["start", "--cwd", "{path}"],
    },
    ToolDef {
        id: "alacritty",
        name: "Alacritty",
        category: "terminal",
        commands: &["alacritty"],
        mac_app: Some("Alacritty"),
        platforms: &[Os::Windows, Os::Macos, Os::Linux],
        launch_kind: LaunchKind::Spawn,
        launch_args: &["--working-directory", "{path}"],
    },
    ToolDef {
        id: "kitty",
        name: "kitty",
        category: "terminal",
        commands: &["kitty"],
        mac_app: Some("kitty"),
        platforms: &[Os::Macos, Os::Linux],
        launch_kind: LaunchKind::Spawn,
        launch_args: &["--directory", "{path}"],
    },
    ToolDef {
        id: "rio",
        name: "Rio",
        category: "terminal",
        commands: &["rio"],
        mac_app: Some("Rio"),
        platforms: &[Os::Windows, Os::Macos, Os::Linux],
        launch_kind: LaunchKind::Spawn,
        launch_args: &["--working-dir", "{path}"],
    },
    ToolDef {
        id: "tabby",
        name: "Tabby",
        category: "terminal",
        commands: &["tabby"],
        mac_app: Some("Tabby"),
        platforms: &[Os::Windows, Os::Macos, Os::Linux],
        launch_kind: LaunchKind::Spawn,
        launch_args: &["open", "{path}"],
    },
    ToolDef {
        id: "hyper",
        name: "Hyper",
        category: "terminal",
        commands: &["hyper"],
        mac_app: Some("Hyper"),
        platforms: &[Os::Windows, Os::Macos, Os::Linux],
        launch_kind: LaunchKind::Spawn,
        launch_args: &["{path}"],
    },
    ToolDef {
        id: "gnome-terminal",
        name: "GNOME Terminal",
        category: "terminal",
        commands: &["gnome-terminal"],
        mac_app: None,
        platforms: &[Os::Linux],
        launch_kind: LaunchKind::Spawn,
        launch_args: &["--working-directory={path}"],
    },
    ToolDef {
        id: "konsole",
        name: "Konsole",
        category: "terminal",
        commands: &["konsole"],
        mac_app: None,
        platforms: &[Os::Linux],
        launch_kind: LaunchKind::Spawn,
        launch_args: &["--workdir", "{path}"],
    },
    ToolDef {
        id: "xfce4-terminal",
        name: "Xfce Terminal",
        category: "terminal",
        commands: &["xfce4-terminal"],
        mac_app: None,
        platforms: &[Os::Linux],
        launch_kind: LaunchKind::Spawn,
        launch_args: &["--working-directory={path}"],
    },
    ToolDef {
        id: "tilix",
        name: "Tilix",
        category: "terminal",
        commands: &["tilix"],
        mac_app: None,
        platforms: &[Os::Linux],
        launch_kind: LaunchKind::Spawn,
        launch_args: &["--working-directory={path}"],
    },
    ToolDef {
        id: "terminator",
        name: "Terminator",
        category: "terminal",
        commands: &["terminator"],
        mac_app: None,
        platforms: &[Os::Linux],
        launch_kind: LaunchKind::Spawn,
        launch_args: &["--working-directory={path}"],
    },
    ToolDef {
        id: "ptyxis",
        name: "Ptyxis",
        category: "terminal",
        commands: &["ptyxis"],
        mac_app: None,
        platforms: &[Os::Linux],
        launch_kind: LaunchKind::Spawn,
        launch_args: &["--working-directory={path}"],
    },
    ToolDef {
        id: "foot",
        name: "foot",
        category: "terminal",
        commands: &["foot"],
        mac_app: None,
        platforms: &[Os::Linux],
        launch_kind: LaunchKind::Spawn,
        launch_args: &["--working-directory={path}"],
    },
    ToolDef {
        id: "urxvt",
        name: "rxvt-unicode (urxvt)",
        category: "terminal",
        commands: &["urxvt"],
        mac_app: None,
        platforms: &[Os::Linux],
        launch_kind: LaunchKind::Spawn,
        launch_args: &["-cd", "{path}"],
    },
    ToolDef {
        id: "xterm",
        name: "xterm",
        category: "terminal",
        commands: &["xterm"],
        mac_app: None,
        platforms: &[Os::Linux],
        launch_kind: LaunchKind::Spawn,
        launch_args: &[],
    },
    ToolDef {
        id: "claude-code",
        name: "Claude Code",
        category: "ai",
        commands: &["claude"],
        mac_app: None,
        platforms: &[Os::Windows, Os::Macos, Os::Linux],
        launch_kind: LaunchKind::InTerminal,
        launch_args: &[],
    },
    ToolDef {
        id: "openai-codex",
        name: "OpenAI Codex CLI",
        category: "ai",
        commands: &["codex"],
        mac_app: None,
        platforms: &[Os::Windows, Os::Macos, Os::Linux],
        launch_kind: LaunchKind::InTerminal,
        launch_args: &[],
    },
    ToolDef {
        id: "github-copilot-cli",
        name: "GitHub Copilot CLI",
        category: "ai",
        commands: &["copilot"],
        mac_app: None,
        platforms: &[Os::Windows, Os::Macos, Os::Linux],
        launch_kind: LaunchKind::InTerminal,
        launch_args: &[],
    },
    ToolDef {
        id: "gemini-cli",
        name: "Gemini CLI",
        category: "ai",
        commands: &["gemini"],
        mac_app: None,
        platforms: &[Os::Windows, Os::Macos, Os::Linux],
        launch_kind: LaunchKind::InTerminal,
        launch_args: &[],
    },
    ToolDef {
        id: "cursor-agent",
        name: "Cursor CLI",
        category: "ai",
        commands: &["cursor-agent"],
        mac_app: None,
        platforms: &[Os::Windows, Os::Macos, Os::Linux],
        launch_kind: LaunchKind::InTerminal,
        launch_args: &[],
    },
    ToolDef {
        id: "aider",
        name: "Aider",
        category: "ai",
        commands: &["aider"],
        mac_app: None,
        platforms: &[Os::Windows, Os::Macos, Os::Linux],
        launch_kind: LaunchKind::InTerminal,
        launch_args: &[],
    },
    ToolDef {
        id: "opencode",
        name: "opencode",
        category: "ai",
        commands: &["opencode"],
        mac_app: None,
        platforms: &[Os::Windows, Os::Macos, Os::Linux],
        launch_kind: LaunchKind::InTerminal,
        launch_args: &[],
    },
    ToolDef {
        id: "amazon-q-cli",
        name: "Amazon Q Developer CLI",
        category: "ai",
        commands: &["q"],
        mac_app: None,
        platforms: &[Os::Macos, Os::Linux],
        launch_kind: LaunchKind::InTerminal,
        launch_args: &[],
    },
    ToolDef {
        id: "amp",
        name: "Amp",
        category: "ai",
        commands: &["amp"],
        mac_app: None,
        platforms: &[Os::Windows, Os::Macos, Os::Linux],
        launch_kind: LaunchKind::InTerminal,
        launch_args: &[],
    },
    ToolDef {
        id: "crush",
        name: "Crush",
        category: "ai",
        commands: &["crush"],
        mac_app: None,
        platforms: &[Os::Windows, Os::Macos, Os::Linux],
        launch_kind: LaunchKind::InTerminal,
        launch_args: &[],
    },
    ToolDef {
        id: "goose",
        name: "Goose",
        category: "ai",
        commands: &["goose"],
        mac_app: None,
        platforms: &[Os::Windows, Os::Macos, Os::Linux],
        launch_kind: LaunchKind::InTerminal,
        launch_args: &[],
    },
    ToolDef {
        id: "qwen-code",
        name: "Qwen Code",
        category: "ai",
        commands: &["qwen"],
        mac_app: None,
        platforms: &[Os::Windows, Os::Macos, Os::Linux],
        launch_kind: LaunchKind::InTerminal,
        launch_args: &[],
    },
    ToolDef {
        id: "continue-cli",
        name: "Continue CLI",
        category: "ai",
        commands: &["cn"],
        mac_app: None,
        platforms: &[Os::Windows, Os::Macos, Os::Linux],
        launch_kind: LaunchKind::InTerminal,
        launch_args: &[],
    },
    ToolDef {
        id: "cline",
        name: "Cline",
        category: "ai",
        commands: &["cline"],
        mac_app: None,
        platforms: &[Os::Windows, Os::Macos, Os::Linux],
        launch_kind: LaunchKind::InTerminal,
        launch_args: &[],
    },
    ToolDef {
        id: "cody-cli",
        name: "Cody CLI",
        category: "ai",
        commands: &["cody"],
        mac_app: None,
        platforms: &[Os::Windows, Os::Macos, Os::Linux],
        launch_kind: LaunchKind::InTerminal,
        launch_args: &[],
    },
    ToolDef {
        id: "auggie",
        name: "Auggie",
        category: "ai",
        commands: &["auggie"],
        mac_app: None,
        platforms: &[Os::Windows, Os::Macos, Os::Linux],
        launch_kind: LaunchKind::InTerminal,
        launch_args: &[],
    },
    ToolDef {
        id: "plandex",
        name: "Plandex",
        category: "ai",
        commands: &["plandex", "pdx"],
        mac_app: None,
        platforms: &[Os::Windows, Os::Macos, Os::Linux],
        launch_kind: LaunchKind::InTerminal,
        launch_args: &[],
    },
];

/// A known tool and whether it is installed on this machine.
#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Tool {
    pub id: String,
    pub name: String,
    pub category: String,
    pub installed: bool,
}

/// A resolved command to spawn.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Invocation {
    pub program: String,
    pub args: Vec<String>,
    /// Working directory for the spawned process (used for terminals so they
    /// open in the repo even without a cwd flag).
    pub cwd: Option<String>,
}

/// Detect which known tools (relevant to this OS) are installed.
pub fn detect_tools(runner: &dyn CommandRunner) -> Vec<Tool> {
    TOOLS
        .iter()
        .filter(|t| t.platforms.contains(&CURRENT_OS))
        .map(|t| Tool {
            id: t.id.to_string(),
            name: t.name.to_string(),
            category: t.category.to_string(),
            installed: is_installed(runner, t),
        })
        .collect()
}

fn is_installed(runner: &dyn CommandRunner, tool: &ToolDef) -> bool {
    if tool.commands.iter().any(|c| command_on_path(runner, c)) {
        return true;
    }
    #[cfg(target_os = "macos")]
    if let Some(app) = tool.mac_app {
        if mac_app_exists(app) {
            return true;
        }
    }
    #[cfg(target_os = "windows")]
    if windows_alias_exists(tool.id) {
        return true;
    }
    false
}

/// Build the launch invocation for a tool, substituting the repo path.
pub fn resolve_launch(tool_id: &str, path: &str) -> AppResult<Invocation> {
    let tool = TOOLS
        .iter()
        .find(|t| t.id == tool_id)
        .ok_or_else(|| AppError::ToolNotFound(tool_id.to_string()))?;
    Ok(build_invocation(tool, path))
}

fn substitute(args: &[&str], path: &str) -> Vec<String> {
    args.iter().map(|a| a.replace("{path}", path)).collect()
}

fn first_command(tool: &ToolDef) -> &str {
    tool.commands.first().copied().unwrap_or(tool.id)
}

// --- Detection: PATH resolution never launches the tool -------------------

#[cfg(target_os = "windows")]
fn command_on_path(runner: &dyn CommandRunner, program: &str) -> bool {
    runner
        .run("where", &[program], None)
        .map(|o| o.is_success())
        .unwrap_or(false)
}

#[cfg(not(target_os = "windows"))]
fn command_on_path(runner: &dyn CommandRunner, program: &str) -> bool {
    runner
        .run("sh", &["-c", &format!("command -v {program}")], None)
        .map(|o| o.is_success())
        .unwrap_or(false)
}

#[cfg(target_os = "macos")]
fn mac_app_exists(app: &str) -> bool {
    use std::path::Path;
    let bundle = format!("{app}.app");
    if Path::new("/Applications").join(&bundle).exists() {
        return true;
    }
    std::env::var("HOME")
        .map(|home| Path::new(&home).join("Applications").join(&bundle).exists())
        .unwrap_or(false)
}

#[cfg(target_os = "windows")]
fn windows_alias_exists(id: &str) -> bool {
    use std::path::Path;
    match id {
        // Windows Terminal is a WindowsApps execution alias that `where` may miss.
        "windows-terminal" => std::env::var("LOCALAPPDATA")
            .map(|dir| {
                Path::new(&dir)
                    .join("Microsoft/WindowsApps/wt.exe")
                    .exists()
            })
            .unwrap_or(false),
        "git-bash" => git_bash_path().is_some(),
        _ => false,
    }
}

#[cfg(target_os = "windows")]
fn git_bash_path() -> Option<String> {
    use std::path::Path;
    for var in ["ProgramFiles", "ProgramFiles(x86)"] {
        if let Ok(dir) = std::env::var(var) {
            let p = Path::new(&dir).join("Git").join("git-bash.exe");
            if p.exists() {
                return Some(p.to_string_lossy().into_owned());
            }
        }
    }
    None
}

// --- Launch: per-OS composition -------------------------------------------

#[cfg(target_os = "windows")]
fn build_invocation(tool: &ToolDef, path: &str) -> Invocation {
    match tool.launch_kind {
        LaunchKind::Spawn => {
            if tool.id == "git-bash" {
                if let Some(exe) = git_bash_path() {
                    return Invocation {
                        program: exe,
                        args: substitute(tool.launch_args, path),
                        cwd: None,
                    };
                }
            }
            // Run via `cmd /C <program> <args>` so PATH `.cmd` shims (code.cmd,
            // idea.cmd) and the `wt` alias resolve. The console is hidden by the
            // caller's CREATE_NO_WINDOW flag.
            let mut args = vec!["/C".to_string(), first_command(tool).to_string()];
            args.extend(substitute(tool.launch_args, path));
            Invocation {
                program: "cmd".into(),
                args,
                cwd: None,
            }
        }
        LaunchKind::InTerminal => {
            // Open a new console at the repo (inherited cwd), run the CLI, keep open.
            Invocation {
                program: "cmd".into(),
                args: vec![
                    "/C".into(),
                    "start".into(),
                    String::new(), // empty window title
                    "cmd".into(),
                    "/K".into(),
                    first_command(tool).to_string(),
                ],
                cwd: Some(path.to_string()),
            }
        }
    }
}

#[cfg(target_os = "macos")]
fn build_invocation(tool: &ToolDef, path: &str) -> Invocation {
    match tool.launch_kind {
        LaunchKind::Spawn => {
            if let Some(app) = tool.mac_app {
                // `open -a "<App>" <path>` reliably opens GUI apps and terminals
                // at the folder without needing a CLI on PATH.
                Invocation {
                    program: "open".into(),
                    args: vec!["-a".into(), app.into(), path.into()],
                    cwd: None,
                }
            } else {
                Invocation {
                    program: first_command(tool).into(),
                    args: substitute(tool.launch_args, path),
                    cwd: Some(path.into()),
                }
            }
        }
        LaunchKind::InTerminal => {
            // Drive Terminal.app via AppleScript: cd to the repo, run the CLI.
            let shell = format!("cd {} && {}", shell_quote(path), first_command(tool));
            let script = format!(
                "tell application \"Terminal\" to do script \"{}\"",
                applescript_escape(&shell)
            );
            Invocation {
                program: "osascript".into(),
                args: vec!["-e".into(), script],
                cwd: None,
            }
        }
    }
}

#[cfg(all(unix, not(target_os = "macos")))]
fn build_invocation(tool: &ToolDef, path: &str) -> Invocation {
    match tool.launch_kind {
        LaunchKind::Spawn => Invocation {
            program: first_command(tool).into(),
            args: substitute(tool.launch_args, path),
            cwd: Some(path.into()),
        },
        LaunchKind::InTerminal => {
            // Open the desktop's default terminal at the repo running the CLI.
            let inner = format!("{}; exec \"$SHELL\"", first_command(tool));
            Invocation {
                program: "x-terminal-emulator".into(),
                args: vec!["-e".into(), "sh".into(), "-c".into(), inner],
                cwd: Some(path.into()),
            }
        }
    }
}

/// Single-quote a string for a POSIX shell (used only on macOS launch).
#[cfg_attr(not(target_os = "macos"), allow(dead_code))]
fn shell_quote(s: &str) -> String {
    format!("'{}'", s.replace('\'', "'\\''"))
}

/// Escape a string for an AppleScript string literal (used only on macOS).
#[cfg_attr(not(target_os = "macos"), allow(dead_code))]
fn applescript_escape(s: &str) -> String {
    s.replace('\\', "\\\\").replace('"', "\\\"")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn catalog_ids_are_unique() {
        let mut seen = std::collections::HashSet::new();
        for t in TOOLS {
            assert!(seen.insert(t.id), "duplicate id: {}", t.id);
        }
    }

    #[test]
    fn catalog_covers_ai_tools_and_categories() {
        assert!(TOOLS.iter().any(|t| t.id == "claude-code"));
        assert!(TOOLS.iter().any(|t| t.id == "openai-codex"));
        assert!(TOOLS.iter().filter(|t| t.category == "ai").count() >= 10);
        assert!(TOOLS.iter().filter(|t| t.category == "editor").count() >= 10);
        assert!(TOOLS.iter().filter(|t| t.category == "terminal").count() >= 5);
    }

    #[test]
    fn resolve_launch_references_the_repo_path() {
        for id in ["vscode", "claude-code", "wezterm"] {
            if TOOLS
                .iter()
                .find(|t| t.id == id)
                .unwrap()
                .platforms
                .contains(&CURRENT_OS)
            {
                let inv = resolve_launch(id, "/tmp/repo").unwrap();
                let in_args = inv.args.iter().any(|a| a.contains("/tmp/repo"));
                let in_cwd = inv.cwd.as_deref() == Some("/tmp/repo");
                assert!(in_args || in_cwd, "path missing for {id}: {inv:?}");
            }
        }
    }

    #[test]
    fn resolve_launch_rejects_unknown_tool() {
        assert!(matches!(
            resolve_launch("nope", "/x"),
            Err(AppError::ToolNotFound(_))
        ));
    }

    #[test]
    fn shell_quote_escapes_single_quotes() {
        assert_eq!(shell_quote("/a/b"), "'/a/b'");
        assert_eq!(shell_quote("it's"), "'it'\\''s'");
    }

    #[test]
    fn applescript_escape_escapes_specials() {
        assert_eq!(applescript_escape("a\"b\\c"), "a\\\"b\\\\c");
    }
}
