//! External-process execution, behind a trait so services stay unit-testable.

use std::process::Command;

use crate::error::{AppError, AppResult};

/// Raw output of a finished process.
#[derive(Debug, Clone)]
pub struct CommandOutput {
    pub stdout: String,
    pub stderr: String,
    pub code: Option<i32>,
}

impl CommandOutput {
    pub fn is_success(&self) -> bool {
        self.code == Some(0)
    }
}

/// Runs external programs. Services depend on this trait, not on
/// [`std::process`], so tests can inject canned output instead of shelling out
/// to a real `git` or `gh`.
pub trait CommandRunner: Send + Sync {
    fn run(&self, program: &str, args: &[&str], cwd: Option<&str>) -> AppResult<CommandOutput>;
}

/// Runs a program and returns its output only when it exits zero; otherwise
/// maps the failure to [`AppError::CommandFailed`] with the captured output.
pub fn run_checked(
    runner: &dyn CommandRunner,
    program: &str,
    args: &[&str],
    cwd: Option<&str>,
) -> AppResult<CommandOutput> {
    let out = runner.run(program, args, cwd)?;
    if out.is_success() {
        Ok(out)
    } else {
        Err(AppError::CommandFailed {
            cmd: format!("{program} {}", args.join(" ")),
            stdout: out.stdout,
            stderr: out.stderr,
            code: out.code,
        })
    }
}

/// The production runner, backed by [`std::process::Command`].
#[derive(Default, Clone, Copy)]
pub struct SystemRunner;

impl CommandRunner for SystemRunner {
    fn run(&self, program: &str, args: &[&str], cwd: Option<&str>) -> AppResult<CommandOutput> {
        let mut cmd = Command::new(program);
        cmd.args(args);
        if let Some(dir) = cwd {
            cmd.current_dir(dir);
        }

        // Don't flash a console window when spawning CLI tools on Windows.
        #[cfg(windows)]
        {
            use std::os::windows::process::CommandExt;
            const CREATE_NO_WINDOW: u32 = 0x0800_0000;
            cmd.creation_flags(CREATE_NO_WINDOW);
        }

        let output = cmd.output().map_err(|source| AppError::Spawn {
            program: program.to_string(),
            source,
        })?;

        Ok(CommandOutput {
            stdout: String::from_utf8_lossy(&output.stdout).into_owned(),
            stderr: String::from_utf8_lossy(&output.stderr).into_owned(),
            code: output.status.code(),
        })
    }
}

#[cfg(test)]
pub mod mock {
    //! A [`CommandRunner`] that replays scripted output, for tests.

    use std::collections::HashMap;

    use super::{CommandOutput, CommandRunner};
    use crate::error::AppResult;

    #[derive(Default)]
    pub struct MockRunner {
        responses: HashMap<String, CommandOutput>,
    }

    impl MockRunner {
        pub fn new() -> Self {
            Self::default()
        }

        /// Register output for `"<program> <args…>"`.
        pub fn on(mut self, invocation: &str, stdout: &str, code: i32) -> Self {
            self.responses.insert(
                invocation.to_string(),
                CommandOutput {
                    stdout: stdout.to_string(),
                    stderr: String::new(),
                    code: Some(code),
                },
            );
            self
        }
    }

    impl CommandRunner for MockRunner {
        fn run(
            &self,
            program: &str,
            args: &[&str],
            _cwd: Option<&str>,
        ) -> AppResult<CommandOutput> {
            let key = format!("{program} {}", args.join(" "));
            Ok(self.responses.get(&key).cloned().unwrap_or(CommandOutput {
                stdout: String::new(),
                stderr: format!("unexpected invocation: {key}"),
                code: Some(1),
            }))
        }
    }
}
