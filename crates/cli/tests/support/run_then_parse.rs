//! **Run, then parse** — the one way a suite reads JSON off a finished process
//! (M54 Increment 1, S1's first part; [DECISIONS.md](../../../../DECISIONS.md) → *M54
//! settled*).
//!
//! A suite that parses a process's stdout *before* asking whether the process did what
//! it was driven to do reports every other outcome as bad JSON. That is how the
//! `doc-code` probe race first surfaced: a `jigc validate --format json` that produced
//! empty stdout failed its suite with *EOF while parsing a value*, which names the
//! parser and hides the exit code, the stderr and the fact that nothing was printed.
//!
//! So the helper **checks the exit first**, against the exit the *site* expects — never a
//! blanket success, because several sites parse a process that is meant to fail (a
//! blocked `task finalize`, a store sweep flipped non-zero, a refusal whose envelope
//! rides stderr) — and only then parses. On a mismatch **or** a parse failure it panics
//! with the site's label, the exit status (a signal named as a signal), stderr and
//! stdout, so the failure says what the process did instead.
//!
//! A site whose caller asserts the exit itself passes every exit it tolerates; the
//! helper still refuses anything outside that set, which is exactly the case that used
//! to masquerade as a parse error.

use serde::de::DeserializeOwned;
use std::process::{ExitStatus, Output};

/// Which of a process's two streams the JSON rides.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Stream {
    Stdout,
    Stderr,
}

/// Parse the JSON on `out`'s **stdout**, after asserting its exit is one of `expected`.
pub fn stdout_json<T: DeserializeOwned>(out: &Output, expected: &[i32], label: &str) -> T {
    parse_json(out, expected, Stream::Stdout, label)
}

/// Parse the JSON on `out`'s **stderr** (a refusal's `--format json` envelope), after
/// asserting its exit is one of `expected`.
pub fn stderr_json<T: DeserializeOwned>(out: &Output, expected: &[i32], label: &str) -> T {
    parse_json(out, expected, Stream::Stderr, label)
}

/// Assert `out` exited with one of `expected`, then parse the JSON on `stream`.
///
/// Panics — naming `label`, the exit, stderr and stdout — when the exit is outside
/// `expected` (checked first, so an unexpected outcome is never reported as bad JSON)
/// or when `stream` does not parse as `T`.
pub fn parse_json<T: DeserializeOwned>(
    out: &Output,
    expected: &[i32],
    stream: Stream,
    label: &str,
) -> T {
    if !out
        .status
        .code()
        .is_some_and(|code| expected.contains(&code))
    {
        panic!(
            "{label}: expected exit {expected:?}, the process ended with {}{}",
            describe(out.status),
            streams(out),
        );
    }
    let (name, bytes) = match stream {
        Stream::Stdout => ("stdout", &out.stdout),
        Stream::Stderr => ("stderr", &out.stderr),
    };
    serde_json::from_slice(bytes).unwrap_or_else(|err| {
        panic!(
            "{label}: {} as expected, but {name} is not the expected JSON ({err}){}",
            describe(out.status),
            streams(out),
        )
    })
}

/// `exit <code>`, or `signal <n>` for a process a signal ended.
fn describe(status: ExitStatus) -> String {
    if let Some(code) = status.code() {
        return format!("exit {code}");
    }
    #[cfg(unix)]
    {
        use std::os::unix::process::ExitStatusExt;
        if let Some(signal) = status.signal() {
            return format!("signal {signal}");
        }
    }
    format!("no exit code ({status})")
}

/// Both streams, labelled, for a panic message.
fn streams(out: &Output) -> String {
    format!(
        "\n--- stderr ---\n{}\n--- stdout ---\n{}",
        String::from_utf8_lossy(&out.stderr),
        String::from_utf8_lossy(&out.stdout),
    )
}
