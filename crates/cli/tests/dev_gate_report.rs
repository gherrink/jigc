//! The fence over `dev/gate`'s **report layer** — the part of the gate tool that is
//! pure text over a log file, and therefore testable in milliseconds.
//!
//! `dev/gate` runs five commands and then *reports* what it measured: the aggregate
//! `passed=`/`failed=` totals, the number of test binaries those totals came from, and
//! on failure the named failing tests. The report is the whole reason to prefer the
//! tool over retyping the gate, and it is also the half that had never been checked —
//! because checking it appeared to cost a five-minute gate run.
//!
//! It does not. The totals, the failing-name extraction and the error-line extraction
//! are three `awk`/`grep` passes over a log; `dev/gate --report <log>` runs exactly
//! those three and nothing else, so a fixture log fences them for the cost of a
//! subprocess. That flag exists for this suite, and is not a second implementation:
//! the live run calls the same three functions.
//!
//! **The defect this suite exists for.** The totals matched `/^test result:/`
//! *undelimited*, so every test whose path begins `result::` — `engine/src/result.rs`
//! has a `mod tests` — counted as a test **binary**: cargo prints
//! `test result::tests::alpha ... ok` per test and `test result: ok. N passed; …` per
//! target, and the pattern could not tell them apart. A 17-binary run reported **33**,
//! and the wrong figure reached a commit message before anything noticed. The count
//! exists to prove the run measured something, so inflating it is precisely the lie it
//! was added to prevent. `passed=`/`failed=` were never wrong: a per-test line carries
//! no `passed;` field.
//!
//! The second arm is the other way a report can be false while looking true: a step
//! that exits **127** did not run at all — the command was not found — and reporting
//! that as `FAILED 127 (0s)` with no error detail is indistinguishable from a red
//! gate, which is the one thing the tool exists to make distinguishable.

use std::path::{Path, PathBuf};
use std::process::Command;

use crate::support;

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .ancestors()
        .nth(2)
        .expect("crates/cli has a repo root two levels up")
        .to_path_buf()
}

fn gate() -> PathBuf {
    repo_root().join("dev/gate")
}

/// Write `body` to a throwaway log and hand `dev/gate --report` its path.
fn report_over(label: &str, body: &str) -> String {
    let dir = support::trial_corpus::unique_root(label);
    std::fs::create_dir_all(&dir).expect("create the fixture log dir");
    let log = dir.join("gate.log");
    std::fs::write(&log, body).expect("write the fixture log");
    let out = Command::new(gate())
        .arg("--report")
        .arg(&log)
        .current_dir(repo_root())
        .output()
        .expect("spawn dev/gate --report");
    assert!(
        out.status.success(),
        "dev/gate --report must succeed over a readable log ({}):\n{}",
        out.status,
        String::from_utf8_lossy(&out.stderr),
    );
    String::from_utf8(out.stdout).expect("utf-8 report")
}

/// One `test result:` summary line, as cargo prints it per test target.
fn summary_line(passed: u32, failed: u32) -> String {
    let verdict = if failed == 0 { "ok." } else { "FAILED." };
    format!(
        "test result: {verdict} {passed} passed; {failed} failed; 0 ignored; 0 measured; \
         0 filtered out; finished in 0.01s\n"
    )
}

#[test]
fn a_test_named_result_is_not_counted_as_a_test_binary() {
    // The exact shape that inflated the count: a test whose path starts `result::`,
    // beside one real summary line. One binary, not two.
    let log = format!(
        "test result::tests::alpha ... ok\n\
         test result::tests::beta ... ok\n\
         {}",
        summary_line(2, 1),
    );
    let report = report_over("gate-report-named-result", &log);
    assert!(
        report.contains("passed=2 failed=1  (over 1 test binaries)"),
        "a test NAMED `result::…` is a test, not a test binary — the log holds one \
         summary line, so the count is 1.\nreport:\n{report}",
    );
}

#[test]
fn the_totals_sum_over_every_summary_line() {
    let mut log = String::new();
    log.push_str("test result::tests::alpha ... ok\n");
    log.push_str(&summary_line(5, 0));
    log.push_str("test resulting_shapes::wide ... ok\n");
    log.push_str("test result::tests::gamma ... ok\n");
    log.push_str(&summary_line(7, 2));
    let report = report_over("gate-report-totals", &log);
    assert!(
        report.contains("passed=12 failed=2  (over 2 test binaries)"),
        "the totals sum the per-target summary lines and count exactly those lines.\n\
         report:\n{report}",
    );
}

#[test]
fn a_log_with_no_test_output_reports_zero_rather_than_guessing() {
    let report = report_over(
        "gate-report-empty",
        "   Compiling engine v0.1.0\n    Finished dev profile\n",
    );
    assert!(
        report.contains("passed=0 failed=0  (over 0 test binaries)"),
        "a run that compiled nothing must be distinguishable from a run that passed — \
         that is what the binary count is for.\nreport:\n{report}",
    );
}

#[test]
fn the_failing_names_come_from_the_failures_list_block_only() {
    // cargo prints the per-test stdout under `---- <name> stdout ----` first, then the
    // `failures:` list block. Only the list block carries the indented bare names.
    //
    // The indentation of the two names is load-bearing (the extractor reads
    // `^    <name>`), so the fixture is built line by line rather than as one
    // continued literal — a `\` continuation eats the next line's leading whitespace
    // and would silently hand the extractor un-indented names.
    let summary = summary_line(1, 2);
    let log = [
        "running 3 tests",
        "test suite_a::holds ... ok",
        "test suite_a::breaks ... FAILED",
        "",
        "failures:",
        "",
        "---- suite_a::breaks stdout ----",
        "thread 'suite_a::breaks' panicked at tests/suite_a.rs:12:5:",
        "assertion failed: the emitted route must run",
        "",
        "failures:",
        "    suite_a::breaks",
        "    suite_b::also_breaks",
        "",
        summary.trim_end(),
        "",
    ]
    .join("\n");
    let report = report_over("gate-report-names", &log);
    assert!(
        report.contains("failing tests:"),
        "a log with failures must name them.\nreport:\n{report}",
    );
    for name in ["suite_a::breaks", "suite_b::also_breaks"] {
        assert!(
            report.contains(name),
            "the failing test `{name}` must be named, so the reader has something to \
             reach for instead of a blind tail.\nreport:\n{report}",
        );
    }
    assert!(
        !report.contains("panicked at"),
        "the name list is the names, not the panic body — the log path is printed for \
         that.\nreport:\n{report}",
    );
}

#[test]
fn compiler_error_lines_are_surfaced_distinctly() {
    let log = "error[E0308]: mismatched types\n\
               error[E0308]: mismatched types\n\
               Diff in /repo/src/lib.rs at line 4:\n\
               warning: unused variable: `x`\n";
    let report = report_over("gate-report-errors", log);
    assert!(
        report.contains("errors (first 20 distinct):"),
        "report:\n{report}",
    );
    assert!(report.contains("error[E0308]"), "report:\n{report}");
    assert!(report.contains("Diff in "), "report:\n{report}");
    assert!(
        !report.contains("warning: unused variable"),
        "warnings are not the failure detail; clippy's own step reports them.\n\
         report:\n{report}",
    );
}

/// A step that could not run is not a verdict about the code.
///
/// Driven rather than asserted structurally: `PATH=/usr/bin:/bin` removes `cargo`, so
/// every step exits 127 in ~0s and nothing is built. The gate still exits non-zero —
/// it did not pass — but it must say the steps *did not run* and name what was
/// missing, or the operator reads a vacuous red as a real one.
#[test]
fn a_step_that_could_not_run_says_so_and_names_the_missing_binary() {
    let out = Command::new(gate())
        .arg("--quick")
        .current_dir(repo_root())
        .env("PATH", "/usr/bin:/bin")
        .output()
        .expect("spawn dev/gate --quick");
    let text = String::from_utf8_lossy(&out.stdout).to_string();
    assert!(
        !out.status.success(),
        "a gate whose steps never ran has not passed.\n{text}",
    );
    assert!(
        text.contains("DID NOT RUN"),
        "exit 127 is `command not found`, not a failing check — say so.\n{text}",
    );
    assert!(
        text.contains("`cargo` not found on PATH"),
        "name the missing binary; that is the whole actionable content.\n{text}",
    );
    assert!(
        text.contains("not a verdict about the code"),
        "the summary must separate `did not run` from `failed`.\n{text}",
    );
}
