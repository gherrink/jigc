//! The fence over [`support::child_stdin::feed`] — the one way a suite writes a child's
//! stdin.
//!
//! **The flake it exists for.** ~150 sites spawned `jigc` with a piped stdin and wrote the
//! payload with `.write_all(…).expect("write stdin")`. When `jigc` exits before reading
//! it — a refusal raised before the payload is consumed — the pipe's read end is gone and
//! the write fails with `BrokenPipe` (Rust ignores `SIGPIPE`, so it is an error rather than
//! a signal). Whether the write lands first is a scheduling race, so the `expect` panicked
//! a test that was about to assert on exactly that refusal — rarely, and more often as the
//! gate got faster. The process's exit and output are the verdict; a payload nobody read
//! is not.

use std::process::{Command, Stdio};

use crate::support::child_stdin::feed;

/// More than any pipe buffer holds, so the write cannot complete into the buffer and must
/// meet the closed read end — the race, made deterministic.
const PAYLOAD: usize = 1 << 20;

#[test]
fn a_child_that_exits_without_reading_its_stdin_is_judged_on_its_exit() {
    let mut child = Command::new("/bin/sh")
        .args(["-c", "echo refused >&2; exit 3"])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("spawn sh");
    feed(&mut child, vec![b'x'; PAYLOAD]);
    let out = child.wait_with_output().expect("wait for sh");
    assert_eq!(
        out.status.code(),
        Some(3),
        "the child's own exit is what the caller asserts on"
    );
    assert_eq!(String::from_utf8_lossy(&out.stderr), "refused\n");
}

#[test]
fn a_child_that_reads_its_stdin_gets_all_of_it_and_then_eof() {
    let mut child = Command::new("/usr/bin/wc")
        .arg("-c")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("spawn wc");
    feed(&mut child, b"twelve bytes");
    // `wc` ends only on EOF, so this returning at all proves `feed` closed the pipe.
    let out = child.wait_with_output().expect("wait for wc");
    assert!(out.status.success());
    assert_eq!(String::from_utf8_lossy(&out.stdout).trim(), "12");
}
