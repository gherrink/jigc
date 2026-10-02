//! **Feed a child its stdin** — the one way a suite writes a spawned process's piped
//! stdin (fenced by `child_stdin_feed.rs`).
//!
//! Every site used to write it inline as `.write_all(payload).expect("write stdin")`.
//! When the child exits before reading — `jigc` refusing an invocation before it consumes
//! the `--from-file -` payload — the pipe's read end is gone and the write fails with
//! `BrokenPipe` (Rust ignores `SIGPIPE`, so it is an error, not a signal). Whether the
//! write lands before the exit is a scheduling race, so that `expect` panicked a test that
//! was about to assert on the refusal itself, rarely and more often as the gate got
//! faster. A payload nobody read is not a failure; what the process did is the verdict,
//! and the caller still waits for it and asserts on its exit and output.

use std::io::{ErrorKind, Write};
use std::process::Child;

/// Write `bytes` to `child`'s piped stdin, then close it so the child reads EOF.
///
/// A `BrokenPipe` — the child exited, or closed its stdin, without reading all of it — is
/// tolerated: the caller's `wait_with_output` and its assertions judge the run. Any other
/// write error panics, as does a child spawned without `Stdio::piped()` stdin.
pub fn feed(child: &mut Child, bytes: impl AsRef<[u8]>) {
    let mut stdin = child
        .stdin
        .take()
        .expect("feed: the child was spawned without a piped stdin");
    match stdin.write_all(bytes.as_ref()) {
        Ok(()) => {}
        Err(e) if e.kind() == ErrorKind::BrokenPipe => {}
        Err(e) => panic!("feed: write the child's stdin: {e}"),
    }
    // `stdin` drops here, closing the pipe.
}
