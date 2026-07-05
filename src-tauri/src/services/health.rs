//! A quick environment check: are the tools Gitward relies on available?

use std::net::{TcpStream, ToSocketAddrs};
use std::time::Duration;

use serde::Serialize;

use crate::services::command::CommandRunner;

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct HealthReport {
    pub git: bool,
    pub gh: bool,
    pub ssh: bool,
    pub internet: bool,
}

/// Probe the local environment. Network reachability is checked with a short
/// TCP connect to github.com so an offline machine reports quickly. Tool
/// detection is a separate command so the UI can show it without waiting on the
/// network probe.
pub fn check(runner: &dyn CommandRunner) -> HealthReport {
    HealthReport {
        git: version_ok(runner, "git"),
        gh: version_ok(runner, "gh"),
        ssh: version_ok(runner, "ssh"),
        internet: internet_reachable(),
    }
}

/// `<program> --version`, or `ssh -V` (which prints to stderr, exit 0).
fn version_ok(runner: &dyn CommandRunner, program: &str) -> bool {
    let arg = if program == "ssh" { "-V" } else { "--version" };
    runner
        .run(program, &[arg], None)
        .map(|o| o.is_success())
        .unwrap_or(false)
}

fn internet_reachable() -> bool {
    "github.com:443"
        .to_socket_addrs()
        .ok()
        .and_then(|mut addrs| addrs.next())
        .map(|addr| TcpStream::connect_timeout(&addr, Duration::from_secs(3)).is_ok())
        .unwrap_or(false)
}
