//! Application-level errors.
//!
//! Services return [`AppResult`]; the command layer converts these into a
//! [`CommandResult`](crate::models::result::CommandResult) so the frontend
//! always receives a structured payload instead of a thrown exception.

/// Errors raised while running external tools or parsing their output.
#[derive(Debug, thiserror::Error)]
pub enum AppError {
    /// An external command ran but exited non-zero.
    #[error("`{cmd}` exited with {code:?}")]
    CommandFailed {
        cmd: String,
        stdout: String,
        stderr: String,
        code: Option<i32>,
    },

    /// The given path is not a git repository.
    #[error("not a git repository: {0}")]
    NotAGitRepo(String),

    /// No launcher tool is registered under the given id.
    #[error("unknown tool: {0}")]
    ToolNotFound(String),

    /// A configuration file could not be parsed.
    #[error("failed to parse {file}: {reason}")]
    FileParse { file: String, reason: String },

    /// No identity field is registered under the given id.
    #[error("unknown identity field: {0}")]
    UnknownField(String),

    /// Push Guard could not be installed or removed (e.g. a foreign hook exists).
    #[error("{0}")]
    Guard(String),

    /// Failed to spawn a process at all (e.g. the program is not on PATH).
    #[error("failed to run `{program}`: {source}")]
    Spawn {
        program: String,
        #[source]
        source: std::io::Error,
    },

    #[error(transparent)]
    Io(#[from] std::io::Error),
}

pub type AppResult<T> = Result<T, AppError>;
