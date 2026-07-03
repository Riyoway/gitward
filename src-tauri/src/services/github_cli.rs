//! Wrapping the GitHub CLI (`gh`) for auth status and switching.

use serde::Serialize;

use crate::error::AppResult;
use crate::services::command::{run_checked, CommandRunner};

/// One account known to `gh` on a given host.
#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct GhAccount {
    pub username: String,
    pub host: String,
    pub active: bool,
    pub protocol: Option<String>,
    pub scopes: Vec<String>,
}

/// List accounts `gh` is logged into. Returns an empty list when logged out;
/// errors only when `gh` itself cannot be run.
pub fn auth_status(runner: &dyn CommandRunner) -> AppResult<Vec<GhAccount>> {
    // `gh auth status` has printed to stdout or stderr across versions, so we
    // parse both. A non-zero exit ("not logged in") is not an error here.
    let out = runner.run("gh", &["auth", "status"], None)?;
    let combined = format!("{}\n{}", out.stdout, out.stderr);
    Ok(parse_auth_status(&combined))
}

/// Switch the active `gh` account.
pub fn auth_switch(runner: &dyn CommandRunner, username: &str) -> AppResult<()> {
    run_checked(runner, "gh", &["auth", "switch", "--user", username], None)?;
    Ok(())
}

/// Configure git to use `gh` as its credential helper for its hosts.
pub fn setup_git(runner: &dyn CommandRunner) -> AppResult<()> {
    run_checked(runner, "gh", &["auth", "setup-git"], None)?;
    Ok(())
}

fn parse_auth_status(text: &str) -> Vec<GhAccount> {
    let mut accounts = Vec::new();
    let mut current: Option<GhAccount> = None;

    for line in text.lines() {
        let trimmed = line.trim();

        if let Some(idx) = trimmed.find("Logged in to ") {
            if let Some(acc) = current.take() {
                accounts.push(acc);
            }
            // e.g. "github.com account Riyoway (keyring)"
            let mut parts = trimmed[idx + "Logged in to ".len()..].split_whitespace();
            let host = parts.next().unwrap_or_default().to_string();
            let _ = parts.next(); // "account"
            let username = parts.next().unwrap_or_default().to_string();
            current = Some(GhAccount {
                username,
                host,
                active: false,
                protocol: None,
                scopes: Vec::new(),
            });
        } else if let Some(acc) = current.as_mut() {
            if let Some(v) = strip_field(trimmed, "Active account:") {
                acc.active = v.eq_ignore_ascii_case("true");
            } else if let Some(v) = strip_field(trimmed, "Git operations protocol:") {
                acc.protocol = Some(v.to_string());
            } else if let Some(v) = strip_field(trimmed, "Token scopes:") {
                acc.scopes = v
                    .split(',')
                    .map(|s| s.trim().trim_matches('\'').trim().to_string())
                    .filter(|s| !s.is_empty())
                    .collect();
            }
        }
    }

    if let Some(acc) = current.take() {
        accounts.push(acc);
    }
    accounts
}

/// Match a `gh` detail line like `- <field> <value>` (the bullet is optional).
fn strip_field<'a>(line: &'a str, field: &str) -> Option<&'a str> {
    line.strip_prefix("- ")
        .unwrap_or(line)
        .strip_prefix(field)
        .map(str::trim)
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "github.com\n\
        \x20 \u{2713} Logged in to github.com account Riyoway (keyring)\n\
        \x20 - Active account: true\n\
        \x20 - Git operations protocol: https\n\
        \x20 - Token: gho_************\n\
        \x20 - Token scopes: 'gist', 'read:org', 'repo', 'workflow'\n\
        \n\
        \x20 \u{2713} Logged in to github.com account YB-Software (keyring)\n\
        \x20 - Active account: false\n\
        \x20 - Git operations protocol: https\n";

    #[test]
    fn parses_multiple_accounts() {
        let accounts = parse_auth_status(SAMPLE);
        assert_eq!(accounts.len(), 2);

        let active = &accounts[0];
        assert_eq!(active.username, "Riyoway");
        assert_eq!(active.host, "github.com");
        assert!(active.active);
        assert_eq!(active.protocol.as_deref(), Some("https"));
        assert_eq!(active.scopes, vec!["gist", "read:org", "repo", "workflow"]);

        let other = &accounts[1];
        assert_eq!(other.username, "YB-Software");
        assert!(!other.active);
    }

    #[test]
    fn logged_out_yields_no_accounts() {
        assert!(parse_auth_status("You are not logged into any GitHub hosts.").is_empty());
    }
}
