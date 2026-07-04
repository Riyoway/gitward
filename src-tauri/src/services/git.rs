//! Reading and writing a repository's local git identity and status.

use std::path::Path;

use serde::Serialize;

use crate::error::{AppError, AppResult};
use crate::services::command::{run_checked, CommandRunner};

/// A repository's local `user.name` / `user.email`. Either may be unset.
#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct GitIdentity {
    pub user_name: Option<String>,
    pub email: Option<String>,
}

/// A snapshot of a repository's working tree and branch state.
#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct GitStatus {
    pub branch: Option<String>,
    pub ahead: u32,
    pub behind: u32,
    pub is_dirty: bool,
    pub remote_url: Option<String>,
}

/// Cheap check for whether `path` is (inside) a git repository.
pub fn is_repo(path: &str) -> bool {
    // `.git` is a directory in a normal clone and a file in a worktree/submodule.
    Path::new(path).join(".git").exists()
}

/// Read the local `user.name` and `user.email`.
pub fn read_config(runner: &dyn CommandRunner, path: &str) -> AppResult<GitIdentity> {
    ensure_repo(path)?;
    Ok(GitIdentity {
        user_name: read_key(runner, path, "user.name")?,
        email: read_key(runner, path, "user.email")?,
    })
}

/// Set the local `user.name` and `user.email`.
pub fn set_config(
    runner: &dyn CommandRunner,
    path: &str,
    user_name: &str,
    email: &str,
) -> AppResult<()> {
    ensure_repo(path)?;
    run_checked(
        runner,
        "git",
        &["-C", path, "config", "--local", "user.name", user_name],
        None,
    )?;
    run_checked(
        runner,
        "git",
        &["-C", path, "config", "--local", "user.email", email],
        None,
    )?;
    Ok(())
}

/// Read branch, ahead/behind, dirtiness, and the origin remote URL.
pub fn status(runner: &dyn CommandRunner, path: &str) -> AppResult<GitStatus> {
    ensure_repo(path)?;
    let out = run_checked(
        runner,
        "git",
        &["-C", path, "status", "--porcelain=v2", "--branch"],
        None,
    )?;
    let mut status = parse_status(&out.stdout);
    status.remote_url = read_remote_url(runner, path);
    Ok(status)
}

fn ensure_repo(path: &str) -> AppResult<()> {
    if is_repo(path) {
        Ok(())
    } else {
        Err(AppError::NotAGitRepo(path.to_string()))
    }
}

/// `git config --get` exits 1 when a key is unset (that is "no value", not an
/// error). Any other non-zero exit is a real failure.
fn read_key(runner: &dyn CommandRunner, path: &str, key: &str) -> AppResult<Option<String>> {
    let out = runner.run(
        "git",
        &["-C", path, "config", "--local", "--get", key],
        None,
    )?;
    if out.is_success() {
        Ok(non_empty(out.stdout))
    } else if out.code == Some(1) {
        Ok(None)
    } else {
        Err(AppError::CommandFailed {
            cmd: format!("git config --get {key}"),
            stdout: out.stdout,
            stderr: out.stderr,
            code: out.code,
        })
    }
}

fn read_remote_url(runner: &dyn CommandRunner, path: &str) -> Option<String> {
    let out = runner
        .run("git", &["-C", path, "remote", "get-url", "origin"], None)
        .ok()?;
    out.is_success().then(|| non_empty(out.stdout)).flatten()
}

fn non_empty(s: String) -> Option<String> {
    let trimmed = s.trim();
    if trimmed.is_empty() {
        None
    } else {
        Some(trimmed.to_string())
    }
}

/// Parse the header lines of `git status --porcelain=v2 --branch`.
fn parse_status(stdout: &str) -> GitStatus {
    let mut status = GitStatus {
        branch: None,
        ahead: 0,
        behind: 0,
        is_dirty: false,
        remote_url: None,
    };

    for line in stdout.lines() {
        if let Some(head) = line.strip_prefix("# branch.head ") {
            status.branch = (head != "(detached)").then(|| head.to_string());
        } else if let Some(ab) = line.strip_prefix("# branch.ab ") {
            // Format: "+<ahead> -<behind>"
            for token in ab.split_whitespace() {
                if let Some(n) = token.strip_prefix('+') {
                    status.ahead = n.parse().unwrap_or(0);
                } else if let Some(n) = token.strip_prefix('-') {
                    status.behind = n.parse().unwrap_or(0);
                }
            }
        } else if !line.starts_with('#') && !line.trim().is_empty() {
            // Any non-header entry means a changed/untracked path exists.
            status.is_dirty = true;
        }
    }

    status
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::services::command::mock::MockRunner;

    /// Create a temp dir that looks like a repo (has a `.git` entry) and return
    /// its path as a string.
    fn temp_repo() -> (tempfile::TempDir, String) {
        let dir = tempfile::tempdir().unwrap();
        std::fs::create_dir(dir.path().join(".git")).unwrap();
        let path = dir.path().to_string_lossy().into_owned();
        (dir, path)
    }

    #[test]
    fn parses_branch_ahead_behind_and_dirty() {
        let out = "# branch.oid abcdef\n\
                   # branch.head feature/x\n\
                   # branch.ab +3 -2\n\
                   1 .M N... 100644 100644 100644 aaa bbb file.rs\n";
        let s = parse_status(out);
        assert_eq!(s.branch.as_deref(), Some("feature/x"));
        assert_eq!(s.ahead, 3);
        assert_eq!(s.behind, 2);
        assert!(s.is_dirty);
    }

    #[test]
    fn clean_repo_is_not_dirty() {
        let out = "# branch.head main\n# branch.ab +0 -0\n";
        let s = parse_status(out);
        assert_eq!(s.branch.as_deref(), Some("main"));
        assert!(!s.is_dirty);
        assert_eq!(s.ahead, 0);
        assert_eq!(s.behind, 0);
    }

    #[test]
    fn detached_head_has_no_branch() {
        let s = parse_status("# branch.head (detached)\n");
        assert!(s.branch.is_none());
    }

    #[test]
    fn read_config_returns_identity() {
        let (_dir, path) = temp_repo();
        let runner = MockRunner::new()
            .on(
                &format!("git -C {path} config --local --get user.name"),
                "Ada Lovelace\n",
                0,
            )
            .on(
                &format!("git -C {path} config --local --get user.email"),
                "ada@example.com\n",
                0,
            );
        let id = read_config(&runner, &path).unwrap();
        assert_eq!(id.user_name.as_deref(), Some("Ada Lovelace"));
        assert_eq!(id.email.as_deref(), Some("ada@example.com"));
    }

    #[test]
    fn read_config_treats_unset_key_as_none() {
        let (_dir, path) = temp_repo();
        // MockRunner returns exit 1 for any unregistered invocation, which is
        // exactly how `git config --get` signals an unset key.
        let runner = MockRunner::new();
        let id = read_config(&runner, &path).unwrap();
        assert!(id.user_name.is_none());
        assert!(id.email.is_none());
    }

    #[test]
    fn non_repo_path_errors() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().to_string_lossy().into_owned();
        let runner = MockRunner::new();
        let err = read_config(&runner, &path).unwrap_err();
        assert!(matches!(err, AppError::NotAGitRepo(_)));
    }
}
