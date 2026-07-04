//! Cross-checking the active `gh` account against Git Credential Manager.
//!
//! A common failure: `gh` is switched to account A, but the credential manager
//! still hands git account B's token — so pushes authenticate as B.

use serde::Serialize;

use crate::error::AppResult;
use crate::services::command::CommandRunner;
use crate::services::github_cli;

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct CredentialDiagnosis {
    /// The currently-active `gh` account, if any.
    pub gh_user: Option<String>,
    /// GitHub accounts stored in Git Credential Manager.
    pub gcm_users: Vec<String>,
    /// True when the active gh account is not among the stored credentials.
    pub mismatch: bool,
}

/// List GitHub accounts stored in Git Credential Manager.
pub fn list(runner: &dyn CommandRunner) -> AppResult<Vec<String>> {
    let out = runner.run("git", &["credential-manager", "github", "list"], None)?;
    Ok(parse_list(&out.stdout))
}

/// Compare the active gh account with the stored credentials.
pub fn diagnose(runner: &dyn CommandRunner) -> AppResult<CredentialDiagnosis> {
    let gh_user = github_cli::auth_status(runner)?
        .into_iter()
        .find(|a| a.active)
        .map(|a| a.username);

    // If GCM is absent or errors, treat it as "nothing stored" — not a mismatch.
    let gcm_users = list(runner).unwrap_or_default();

    let mismatch = match &gh_user {
        Some(user) => {
            !gcm_users.is_empty() && !gcm_users.iter().any(|g| g.eq_ignore_ascii_case(user))
        }
        None => false,
    };

    Ok(CredentialDiagnosis {
        gh_user,
        gcm_users,
        mismatch,
    })
}

/// Parse `git credential-manager github list`. Keep only lines that look like
/// usernames (no spaces, not a URL/host header), so header lines are ignored.
fn parse_list(stdout: &str) -> Vec<String> {
    stdout
        .lines()
        .map(str::trim)
        .filter(|l| !l.is_empty() && !l.contains(' ') && !l.contains("://"))
        .map(String::from)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::services::command::mock::MockRunner;

    const GH_ACTIVE_RIYOWAY: &str =
        "github.com\n  \u{2713} Logged in to github.com account riyoway (keyring)\n  - Active account: true\n";

    #[test]
    fn parse_list_keeps_usernames_only() {
        assert_eq!(parse_list("riyoway\nynom1\n"), vec!["riyoway", "ynom1"]);
        // Header/URL lines are dropped.
        assert_eq!(
            parse_list("https://github.com\n  riyoway\n"),
            vec!["riyoway"]
        );
        assert!(parse_list("").is_empty());
    }

    #[test]
    fn no_mismatch_when_active_account_is_stored() {
        let runner = MockRunner::new()
            .on("gh auth status", GH_ACTIVE_RIYOWAY, 0)
            .on("git credential-manager github list", "riyoway\n", 0);
        let d = diagnose(&runner).unwrap();
        assert_eq!(d.gh_user.as_deref(), Some("riyoway"));
        assert_eq!(d.gcm_users, vec!["riyoway"]);
        assert!(!d.mismatch);
    }

    #[test]
    fn mismatch_when_active_account_is_not_stored() {
        let runner = MockRunner::new()
            .on("gh auth status", GH_ACTIVE_RIYOWAY, 0)
            .on("git credential-manager github list", "someone-else\n", 0);
        let d = diagnose(&runner).unwrap();
        assert!(d.mismatch);
    }

    #[test]
    fn no_mismatch_when_gcm_is_empty() {
        let runner = MockRunner::new()
            .on("gh auth status", GH_ACTIVE_RIYOWAY, 0)
            .on("git credential-manager github list", "", 0);
        let d = diagnose(&runner).unwrap();
        assert!(d.gcm_users.is_empty());
        assert!(!d.mismatch);
    }
}
