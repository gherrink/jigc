//! The fence over [`support::run_then_parse`] — the helper every suite that reads JSON
//! off a finished process goes through (M54 Increment 1, S1's first part).
//!
//! **Why it exists.** `validate_command::validate_stale_anchor_surfaces_finding_without_flipping_the_exit_itself`
//! failed once in a full gate with *EOF while parsing a value*: its process had produced
//! empty stdout, and the test parsed before asking whether the process ran, so the
//! failure named the parser instead of the exit code and the stderr. The race behind it
//! was later reproduced at 5.5 % under load ([DECISIONS.md](../../../DECISIONS.md) → *M54
//! settled*, S1). These arms feed the helper **real processes** — `sh -c`, never a
//! hand-built `Output` — and assert on the panic text a failing suite would print.

use crate::support::run_then_parse::{Stream, parse_json, stderr_json, stdout_json};
use std::panic::{AssertUnwindSafe, catch_unwind};
use std::process::{Command, Output};

/// Run `script` under `sh -c` to completion.
fn sh(script: &str) -> Output {
    Command::new("sh")
        .args(["-c", script])
        .output()
        .expect("spawn sh")
}

/// The panic message of `f`, which must panic.
fn panic_text(f: impl FnOnce()) -> String {
    let payload = catch_unwind(AssertUnwindSafe(f)).expect_err("the helper must panic here");
    if let Some(s) = payload.downcast_ref::<String>() {
        s.clone()
    } else if let Some(s) = payload.downcast_ref::<&str>() {
        (*s).to_string()
    } else {
        panic!("a non-string panic payload")
    }
}

#[test]
fn a_process_that_exits_other_than_expected_is_reported_by_exit_and_both_streams() {
    let out = sh("printf 'OUT-MARKER\\n'; printf 'ERR-MARKER\\n' >&2; exit 3");
    let text = panic_text(|| {
        let _: serde_json::Value = stdout_json(&out, &[0], "the failing label");
    });
    for needle in ["the failing label", "exit 3", "ERR-MARKER", "OUT-MARKER"] {
        assert!(
            text.contains(needle),
            "the panic must name `{needle}`; it said:\n{text}"
        );
    }
    assert!(
        !text.contains("expected value") && !text.contains("EOF while parsing"),
        "an exit mismatch is reported before any parse is attempted; it said:\n{text}"
    );
}

#[test]
fn an_empty_stdout_under_a_wrong_exit_names_the_exit_not_the_parser() {
    // The shape of the recorded flake: nothing on stdout, a process that did not succeed.
    let out = sh("printf 'could not start\\n' >&2; exit 1");
    let text = panic_text(|| {
        let _: serde_json::Value = stdout_json(&out, &[0], "the sweep");
    });
    assert!(
        text.contains("exit 1"),
        "the exit is named; it said:\n{text}"
    );
    assert!(
        text.contains("could not start"),
        "the stderr is shown; it said:\n{text}"
    );
    assert!(
        !text.contains("EOF while parsing"),
        "the parser is not the reported cause; it said:\n{text}"
    );
}

#[test]
fn an_exit_matching_process_with_non_json_output_names_both_streams() {
    let out = sh("printf 'not json at all\\n'; printf 'a warning\\n' >&2; exit 0");
    let text = panic_text(|| {
        let _: serde_json::Value = stdout_json(&out, &[0], "the listing");
    });
    for needle in ["the listing", "not json at all", "a warning", "exit 0"] {
        assert!(
            text.contains(needle),
            "the panic must name `{needle}`; it said:\n{text}"
        );
    }
}

#[test]
fn a_signal_is_named_as_a_signal() {
    let out = sh("kill -9 $$");
    let text = panic_text(|| {
        let _: serde_json::Value = stdout_json(&out, &[0], "the killed run");
    });
    assert!(
        text.contains("signal 9"),
        "a process killed by a signal is named by it; it said:\n{text}"
    );
}

#[test]
fn the_stderr_arm_parses_a_stderr_envelope() {
    let out =
        sh("printf 'human text on stdout\\n'; printf '{\"error\":\"refused\"}\\n' >&2; exit 1");
    let envelope: serde_json::Value = stderr_json(&out, &[1], "the refusal envelope");
    assert_eq!(envelope["error"], "refused");
}

#[test]
fn a_conforming_process_returns_the_parsed_value() {
    let out = sh("printf '{\"findings\":[{\"code\":\"a.b\"}]}\\n'");
    let report: serde_json::Value = stdout_json(&out, &[0], "the report");
    assert_eq!(report["findings"][0]["code"], "a.b");
}

#[test]
fn an_expected_set_accepts_any_of_its_members() {
    // A site whose caller asserts the exit itself names every exit it tolerates.
    let out = sh("printf '[1, 2]\\n'; exit 1");
    let parsed: Vec<u8> = parse_json(&out, &[0, 1], Stream::Stdout, "the sweep");
    assert_eq!(parsed, vec![1, 2]);
}
