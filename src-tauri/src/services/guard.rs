//! Push Guard: a self-contained `pre-push` hook that blocks a push when the
//! repository's identity doesn't match its assigned account.
//!
//! The hook is a plain POSIX shell script with the expected identity baked in,
//! so it works from any terminal or AI agent without Gitward running. The
//! expected git name/email and gh account come from the repo's assignment; the
//! expected remote (and its SSH host) are captured from `origin` at install.

use std::fs;
use std::path::{Path, PathBuf};

use serde::Serialize;

use crate::error::{AppError, AppResult};
use crate::services::command::CommandRunner;
use crate::services::git;

/// Appears in every hook we write, so we can recognise (and refuse to clobber)
/// hooks that aren't ours.
const MARKER: &str = "gitward-push-guard";

/// Whether a repository has our hook, someone else's, or none.
#[derive(Debug, Clone, Copy, Serialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum GuardState {
    /// No `pre-push` hook present.
    None,
    /// Our hook is installed.
    Gitward,
    /// A `pre-push` hook exists that Gitward did not write — never touched.
    Foreign,
}

/// Report the current guard state. Unresolvable layouts (worktrees, submodules,
/// bare repos) read as `None`; installing there surfaces the real reason.
pub fn state(repo: &str) -> GuardState {
    let Some(hook) = hook_path(repo) else {
        return GuardState::None;
    };
    match fs::read_to_string(&hook) {
        Ok(body) if body.contains(MARKER) => GuardState::Gitward,
        Ok(_) => GuardState::Foreign,
        Err(_) => GuardState::None,
    }
}

/// Install (or refresh) the guard hook with the given expected identity. Empty
/// fields are not enforced. Refuses to overwrite a foreign `pre-push` hook.
pub fn install(
    runner: &dyn CommandRunner,
    repo: &str,
    name: &str,
    email: &str,
    gh: &str,
) -> AppResult<()> {
    let hook = hook_path(repo).ok_or_else(|| {
        AppError::Guard(
            "Push Guard supports standard repositories only \
             (worktrees and submodules are not supported yet)."
                .to_string(),
        )
    })?;

    if let Ok(existing) = fs::read_to_string(&hook) {
        if !existing.contains(MARKER) {
            return Err(AppError::Guard(
                "A pre-push hook already exists and was not created by Gitward. \
                 Remove or rename it first."
                    .to_string(),
            ));
        }
    }

    // Capture the current origin as the expected remote baseline.
    let remote = git::origin_url(runner, repo).unwrap_or_default();
    let ssh_host = ssh_host_of(&remote);
    let script = render_hook(name, email, gh, &remote, &ssh_host);

    if let Some(parent) = hook.parent() {
        fs::create_dir_all(parent)?;
    }
    fs::write(&hook, script)?;

    // Git only runs a hook that is executable on Unix; Windows ignores the bit.
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        fs::set_permissions(&hook, fs::Permissions::from_mode(0o755))?;
    }

    Ok(())
}

/// Remove the guard hook. Idempotent when absent; refuses to touch a foreign hook.
pub fn uninstall(repo: &str) -> AppResult<()> {
    let Some(hook) = hook_path(repo) else {
        return Ok(());
    };
    match fs::read_to_string(&hook) {
        Ok(body) if body.contains(MARKER) => {
            fs::remove_file(&hook)?;
            Ok(())
        }
        Ok(_) => Err(AppError::Guard(
            "Refusing to remove a pre-push hook Gitward did not create.".to_string(),
        )),
        Err(_) => Ok(()),
    }
}

/// `<repo>/.git/hooks/pre-push`, but only for a normal `.git` directory. When
/// `.git` is a file (worktree/submodule), return `None` so callers fail loudly
/// rather than install a hook that silently never runs.
fn hook_path(repo: &str) -> Option<PathBuf> {
    let git = Path::new(repo).join(".git");
    git.is_dir().then(|| git.join("hooks").join("pre-push"))
}

/// Extract the SSH host from a remote URL, or `""` for non-SSH (https) remotes.
/// Handles `ssh://[user@]host[:port]/path` and scp-like `user@host:path`,
/// including host aliases like `git@github-work:owner/repo`.
fn ssh_host_of(url: &str) -> String {
    let url = url.trim();
    if let Some(rest) = url.strip_prefix("ssh://") {
        let after_at = rest.rsplit('@').next().unwrap_or(rest);
        after_at.split(['/', ':']).next().unwrap_or("").to_string()
    } else if !url.starts_with("http://")
        && !url.starts_with("https://")
        && url.contains('@')
        && url.contains(':')
    {
        let after_at = url.rsplit('@').next().unwrap_or(url);
        after_at.split(':').next().unwrap_or("").to_string()
    } else {
        String::new()
    }
}

/// Wrap a value as a shell single-quoted literal, escaping embedded quotes, so a
/// crafted name/email can't break out of the script.
fn sh_quote(s: &str) -> String {
    format!("'{}'", s.replace('\'', "'\\''"))
}

fn render_hook(name: &str, email: &str, gh: &str, remote: &str, ssh_host: &str) -> String {
    let header = format!(
        "expected_name={}\n\
         expected_email={}\n\
         expected_gh={}\n\
         expected_remote={}\n\
         expected_ssh_host={}\n",
        sh_quote(name),
        sh_quote(email),
        sh_quote(gh),
        sh_quote(remote),
        sh_quote(ssh_host),
    );
    format!("{HOOK_PRELUDE}\n{header}{HOOK_BODY}")
}

const HOOK_PRELUDE: &str = "#!/bin/sh\n\
    # gitward-push-guard — managed by Gitward. Regenerated on change; do not edit by hand.\n\
    # Blocks a push when this repository's identity does not match its assigned account.\n\
    # Bypass once (not recommended): git push --no-verify";

const HOOK_BODY: &str = r#"
ssh_host_of() {
  case "$1" in
    ssh://*) r="${1#ssh://}"; r="${r##*@}"; printf '%s' "${r%%[:/]*}" ;;
    *@*:*)   r="${1##*@}"; printf '%s' "${r%%:*}" ;;
    *)       printf '' ;;
  esac
}

cur_name=$(git config user.name 2>/dev/null)
cur_email=$(git config user.email 2>/dev/null)
cur_remote=$(git remote get-url origin 2>/dev/null)
cur_ssh_host=$(ssh_host_of "$cur_remote")

cur_gh=''
if command -v gh >/dev/null 2>&1; then
  cur_gh=$(gh auth status 2>&1 | grep -B1 'Active account: true' | grep 'Logged in to' | head -n1 | sed -n 's/.*account \([^ ]*\).*/\1/p')
fi

fail=0
if [ -n "$expected_name" ]     && [ "$cur_name" != "$expected_name" ];         then fail=1; fi
if [ -n "$expected_email" ]    && [ "$cur_email" != "$expected_email" ];       then fail=1; fi
if [ -n "$expected_remote" ]   && [ "$cur_remote" != "$expected_remote" ];     then fail=1; fi
if [ -n "$expected_ssh_host" ] && [ "$cur_ssh_host" != "$expected_ssh_host" ]; then fail=1; fi
if [ -n "$expected_gh" ] && [ -n "$cur_gh" ] && [ "$cur_gh" != "$expected_gh" ]; then fail=1; fi

if [ "$fail" -ne 0 ]; then
  {
    echo ''
    echo 'ERROR: Gitward blocked this push.'
    echo ''
    echo 'Expected'
    [ -n "$expected_gh" ]       && printf '  GitHub Account : %s\n' "$expected_gh"
    [ -n "$expected_email" ]    && printf '  Git Email      : %s\n' "$expected_email"
    [ -n "$expected_name" ]     && printf '  Git Name       : %s\n' "$expected_name"
    [ -n "$expected_remote" ]   && printf '  Remote origin  : %s\n' "$expected_remote"
    [ -n "$expected_ssh_host" ] && printf '  SSH Host       : %s\n' "$expected_ssh_host"
    echo ''
    echo 'Current'
    printf '  GitHub Account : %s\n' "${cur_gh:-<none>}"
    printf '  Git Email      : %s\n' "${cur_email:-<unset>}"
    printf '  Git Name       : %s\n' "${cur_name:-<unset>}"
    printf '  Remote origin  : %s\n' "${cur_remote:-<none>}"
    [ -n "$expected_ssh_host" ] && printf '  SSH Host       : %s\n' "${cur_ssh_host:-<none>}"
    echo ''
    echo 'Push cancelled. Run Sync in Gitward to fix the identity, or bypass once with: git push --no-verify'
    echo ''
  } >&2
  exit 1
fi

exit 0
"#;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::services::command::mock::MockRunner;

    fn temp_repo() -> (tempfile::TempDir, String) {
        let dir = tempfile::tempdir().unwrap();
        fs::create_dir(dir.path().join(".git")).unwrap();
        let path = dir.path().to_string_lossy().into_owned();
        (dir, path)
    }

    fn runner_with_origin(path: &str, url: &str) -> MockRunner {
        MockRunner::new().on(
            &format!("git -C {path} remote get-url origin"),
            &format!("{url}\n"),
            0,
        )
    }

    #[test]
    fn ssh_host_extraction() {
        assert_eq!(ssh_host_of("git@github.com:owner/repo.git"), "github.com");
        assert_eq!(
            ssh_host_of("ssh://git@github.com/owner/repo.git"),
            "github.com"
        );
        assert_eq!(
            ssh_host_of("ssh://git@github.com:22/owner/repo.git"),
            "github.com"
        );
        assert_eq!(ssh_host_of("git@github-work:owner/repo.git"), "github-work");
        assert_eq!(ssh_host_of("https://github.com/owner/repo.git"), "");
    }

    #[test]
    fn rendered_hook_bakes_values_and_marker() {
        let script = render_hook(
            "Ada",
            "ada@example.com",
            "riyoway",
            "git@github.com:o/r.git",
            "github.com",
        );
        assert!(script.contains(MARKER));
        assert!(script.contains("expected_name='Ada'"));
        assert!(script.contains("expected_email='ada@example.com'"));
        assert!(script.contains("expected_gh='riyoway'"));
        assert!(script.contains("expected_remote='git@github.com:o/r.git'"));
        assert!(script.contains("expected_ssh_host='github.com'"));
        assert!(script.starts_with("#!/bin/sh"));
    }

    #[test]
    fn sh_quote_escapes_single_quotes() {
        // A name with a quote must not break out of the shell literal.
        assert_eq!(sh_quote("a'b"), "'a'\\''b'");
        let script = render_hook("a'b", "e@x.com", "", "", "");
        assert!(script.contains("expected_name='a'\\''b'"));
    }

    #[test]
    fn install_then_state_is_gitward_then_none_after_uninstall() {
        let (_dir, path) = temp_repo();
        let runner = runner_with_origin(&path, "git@github.com:o/r.git");

        assert_eq!(state(&path), GuardState::None);
        install(&runner, &path, "Ada", "ada@example.com", "riyoway").unwrap();
        assert_eq!(state(&path), GuardState::Gitward);

        // The captured origin is baked in.
        let body = fs::read_to_string(Path::new(&path).join(".git/hooks/pre-push")).unwrap();
        assert!(body.contains("expected_remote='git@github.com:o/r.git'"));
        assert!(body.contains("expected_ssh_host='github.com'"));

        uninstall(&path).unwrap();
        assert_eq!(state(&path), GuardState::None);
    }

    #[test]
    fn refuses_to_clobber_or_remove_a_foreign_hook() {
        let (_dir, path) = temp_repo();
        let hooks = Path::new(&path).join(".git/hooks");
        fs::create_dir_all(&hooks).unwrap();
        fs::write(
            hooks.join("pre-push"),
            "#!/bin/sh\necho someone elses hook\n",
        )
        .unwrap();

        assert_eq!(state(&path), GuardState::Foreign);

        let runner = runner_with_origin(&path, "");
        assert!(matches!(
            install(&runner, &path, "Ada", "ada@example.com", "").unwrap_err(),
            AppError::Guard(_)
        ));
        assert!(matches!(uninstall(&path).unwrap_err(), AppError::Guard(_)));
        // The foreign hook is untouched.
        assert_eq!(state(&path), GuardState::Foreign);
    }

    #[test]
    fn install_reinstall_is_idempotent_and_refreshes_values() {
        let (_dir, path) = temp_repo();
        let runner = runner_with_origin(&path, "git@github.com:o/r.git");
        install(&runner, &path, "Ada", "ada@example.com", "riyoway").unwrap();
        // Reinstalling over our own hook succeeds (used to refresh baked values).
        install(&runner, &path, "Bob", "bob@example.com", "bob").unwrap();
        let body = fs::read_to_string(Path::new(&path).join(".git/hooks/pre-push")).unwrap();
        assert!(body.contains("expected_email='bob@example.com'"));
        assert!(!body.contains("ada@example.com"));
    }
}
