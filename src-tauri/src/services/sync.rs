//! The core action: align a repository's git and GitHub identity in one pass.
//!
//! Not atomic. Steps run in order and stop at the first failure; every attempted
//! step is recorded so the frontend can show exactly what happened.

use serde::Serialize;

use crate::error::{AppError, AppResult};
use crate::services::command::CommandRunner;
use crate::services::{git, github_cli};

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct SyncStep {
    pub name: String,
    pub success: bool,
    pub stdout: String,
    pub stderr: String,
    pub exit_code: Option<i32>,
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct SyncReport {
    pub steps: Vec<SyncStep>,
    pub overall_success: bool,
}

/// Set the repo's local identity, then (if a gh account is given) switch the
/// active gh account and re-point git's credential helper at it.
pub fn sync_repository(
    runner: &dyn CommandRunner,
    path: &str,
    user_name: &str,
    email: &str,
    gh_username: Option<&str>,
) -> SyncReport {
    let mut steps = Vec::new();

    if record(&mut steps, "git_config", || {
        git::set_config(runner, path, user_name, email)
    }) {
        if let Some(user) = gh_username {
            if record(&mut steps, "gh_switch", || {
                github_cli::auth_switch(runner, user)
            }) {
                record(&mut steps, "gh_setup_git", || github_cli::setup_git(runner));
            }
        }
    }

    let overall_success = steps.iter().all(|s| s.success);
    SyncReport {
        steps,
        overall_success,
    }
}

/// Run one step, append its outcome, and report whether it succeeded.
fn record(steps: &mut Vec<SyncStep>, name: &str, step: impl FnOnce() -> AppResult<()>) -> bool {
    let (success, stdout, stderr, exit_code) = match step() {
        Ok(()) => (true, String::new(), String::new(), Some(0)),
        Err(AppError::CommandFailed {
            stdout,
            stderr,
            code,
            ..
        }) => (false, stdout, stderr, code),
        Err(err) => (false, String::new(), err.to_string(), None),
    };
    steps.push(SyncStep {
        name: name.to_string(),
        success,
        stdout,
        stderr,
        exit_code,
    });
    success
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::services::command::mock::MockRunner;

    fn temp_repo() -> (tempfile::TempDir, String) {
        let dir = tempfile::tempdir().unwrap();
        std::fs::create_dir(dir.path().join(".git")).unwrap();
        let path = dir.path().to_string_lossy().into_owned();
        (dir, path)
    }

    #[test]
    fn syncs_git_and_gh_when_all_steps_pass() {
        let (_dir, path) = temp_repo();
        let runner = MockRunner::new()
            .on(
                &format!("git -C {path} config --local user.name Ada"),
                "",
                0,
            )
            .on(
                &format!("git -C {path} config --local user.email ada@example.com"),
                "",
                0,
            )
            .on("gh auth switch --user riyoway", "", 0)
            .on("gh auth setup-git", "", 0);

        let report = sync_repository(&runner, &path, "Ada", "ada@example.com", Some("riyoway"));
        assert!(report.overall_success);
        assert_eq!(report.steps.len(), 3);
        assert_eq!(report.steps[0].name, "git_config");
        assert_eq!(report.steps[2].name, "gh_setup_git");
    }

    #[test]
    fn stops_after_a_failing_step() {
        let (_dir, path) = temp_repo();
        // git_config succeeds, gh_switch fails -> gh_setup_git never runs.
        let runner = MockRunner::new()
            .on(
                &format!("git -C {path} config --local user.name Ada"),
                "",
                0,
            )
            .on(
                &format!("git -C {path} config --local user.email ada@example.com"),
                "",
                0,
            )
            .on("gh auth switch --user ghost", "no such account", 1);

        let report = sync_repository(&runner, &path, "Ada", "ada@example.com", Some("ghost"));
        assert!(!report.overall_success);
        assert_eq!(report.steps.len(), 2);
        assert!(!report.steps[1].success);
        assert_eq!(report.steps[1].name, "gh_switch");
    }

    #[test]
    fn skips_gh_steps_when_no_account_assigned() {
        let (_dir, path) = temp_repo();
        let runner = MockRunner::new()
            .on(
                &format!("git -C {path} config --local user.name Ada"),
                "",
                0,
            )
            .on(
                &format!("git -C {path} config --local user.email ada@example.com"),
                "",
                0,
            );

        let report = sync_repository(&runner, &path, "Ada", "ada@example.com", None);
        assert!(report.overall_success);
        assert_eq!(report.steps.len(), 1);
    }
}
