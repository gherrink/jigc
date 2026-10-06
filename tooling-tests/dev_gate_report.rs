//! The fence over `dev/gate`'s **report layer** — the part of the gate tool that is
//! pure text over a log file, and therefore testable in milliseconds.
//!
//! `dev/gate` runs four commands and then *reports* what it measured: the aggregate
//! `passed=`/`failed=` totals, the number of test binaries those totals came from, and
//! on failure the named failing tests. The report is the whole reason to prefer the
//! tool over retyping the gate, and it is also the half that had never been checked —
//! because checking it appeared to cost a five-minute gate run.
//!
//! It does not. The totals, the failing-name extraction, the error-line extraction and
//! (since 2026-10-05) the heaviest-suites listing are `awk`/`grep` passes over a log;
//! `dev/gate --report <log>` runs exactly those four and nothing else, so a fixture log
//! fences them for the cost of a subprocess. That flag exists for this suite, and is not
//! a second implementation: the live run calls the same four functions.
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
//! **Two of the lines this tool prints are read by a machine, and neither had a fence.**
//! `.claude/workflows/milestone-build.js`'s `gateProven` predicate is how the build
//! harness decides an increment's gate is a *fact* rather than a claim: it requires the
//! validator's pasted `gate_evidence` to carry **both** the bare `GATE: PASS` verdict and
//! the full-run `tests   passed=… failed=0  (over N test binaries)` totals line, each
//! matched **whole** — from the start of a line to the end of it, with only leading
//! whitespace and list/quote decoration allowed in front (evidence arrives pasted into
//! markdown), and nothing trailing.
//!
//! What is pinned here is the **rendered text** those patterns must match — the `tests`
//! label included, which the assertions below had never covered — and deliberately not
//! the patterns themselves: the harness has already widened them once (the decoration
//! allowance), so a regex literal copied into this file would have rotted on that edit
//! while still reading as fact. The literals are a **contract with that consumer**, not
//! incidental formatting; the patterns are that consumer's business.
//!
//! The failure lands in the expensive direction and is silent from here: reword the
//! summary label (`tests` → `test`, or drop it) and this suite stays green, `dev/gate`
//! stays correct, and every subsequent increment runs the validator three times — a
//! full gate each — before the harness halts with *"gate green could not be verified"*.
//! So the arms below pin the rendered totals line **whole**, and pin the `GATE: PASS`
//! verdict at its producer.
//!
//! **Why the verdict is fenced against the source and not against a run.** `--report`
//! cannot reach it: that path calls `gate_report` and `exit 0`, and `gate_report` prints
//! no `GATE:` line at all. The only producers are the live tail — `GATE: FAIL` when a
//! step failed, `GATE: PASS` after every step exited 0 — and reaching the PASS branch
//! means running the real gate, which is the five minutes this suite exists to avoid.
//! A source scan is the honest fence for a single-source literal here, on the
//! [`dev_rig_parity`](../crates/cli/tests/dev_rig_parity.rs) precedent (which scans `dev/` for a removal shape
//! it must never contain). The same scan carries the totals line's *other* producer:
//! `gate_report` and the live tail each render that line today, and only the live one is
//! what the harness ever reads, so pinning the report layer's rendering alone would leave
//! the consumed copy free to drift. The scan therefore holds *every* producer to one
//! format rather than pinning how many there are — collapsing the two into one helper is
//! the cleanup this arm should welcome, not punish.
//!
//! The second arm is the other way a report can be false while looking true: a step
//! that exits **127** did not run at all — the command was not found — and reporting
//! that as `FAILED 127 (0s)` with no error detail is indistinguishable from a red
//! gate, which is the one thing the tool exists to make distinguishable.

use std::path::{Path, PathBuf};
use std::process::Command;

use crate::support::scratch::ScratchDir;

use super::placed_executable;

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

/// `dev/gate`, run from the repo root with its `$TMPDIR` inside `tmp`.
///
/// A real run keeps its log, its step file and its hygiene logs in `$TMPDIR` on purpose —
/// the log path is what it prints — so every run this suite drives would leave them
/// behind; pointed into a [`ScratchDir`], they go when the guard does.
fn gate_in(tmp: &ScratchDir) -> Command {
    let mut cmd = Command::new(gate());
    cmd.current_dir(repo_root()).env("TMPDIR", tmp.path());
    cmd
}

/// Write `body` to a throwaway log and hand `dev/gate --report` its path.
fn report_over(label: &str, body: &str) -> String {
    let dir = ScratchDir::new(label);
    let log = dir.path().join("gate.log");
    std::fs::write(&log, body).expect("write the fixture log");
    let out = gate_in(&dir)
        .arg("--report")
        .arg(&log)
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

/// The `printf` statements in `dev/gate` that render `needle`.
///
/// Producers, never raw occurrences — and that distinction is the whole point. `dev/gate`'s
/// leading comment block **is** its `--help` text (`-h` prints the block verbatim), so
/// writing `# On success it prints GATE: PASS.` there is an ordinary, good edit: a comment
/// emits nothing, and a prose help line does not match the harness's line-whole patterns
/// anyway. Counting substrings would redden this suite for that edit and tell the reader
/// something false about why. Full-line `#` comments are dropped for exactly that reason.
fn printf_producers<'a>(src: &'a str, needle: &str) -> Vec<&'a str> {
    src.lines()
        .map(str::trim)
        .filter(|line| !line.starts_with('#') && line.contains("printf") && line.contains(needle))
        .collect()
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
    let tmp = ScratchDir::new("gate-did-not-run");
    let out = gate_in(&tmp)
        .arg("--quick")
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

// ---------------------------------------------------------------------------
// The nextest arm: what the test step prints since the gate runs `cargo nextest`.
// ---------------------------------------------------------------------------
//
// `dev/gate` runs the suite under `cargo nextest run` — twice since 2026-10-05, a fast
// tier and then its complement (the section after this one) — and then
// `cargo test --workspace --doc` (nextest does not run doctests). nextest prints no
// per-target `test result:` line: per run it prints one `Starting N tests across B
// binaries` header and one `Summary [ … ] R tests run: P passed, …` footer, and names each failure
// on a `FAIL [ … ] (i/n) <binary> <test>` status line — once as it happens, once more in
// the final summary. And it **echoes each failing test's captured libtest output,
// indented** — `    test result: FAILED. 0 passed; 1 failed; …` included — so a totals
// pass that matched that line by its fields rather than at column 0 would count every
// failing test as one more binary. The fixture below is a real nextest 0.9 run's shape
// (two failing tests over two binaries, one ignored), followed by the doctest step's
// two column-0 summaries.

/// A nextest run over two binaries — `passed` of `run` passing — then the doctest step.
fn nextest_log(run: &str, passed: u32, failed_lines: &[&str]) -> String {
    let mut log = vec![
        "    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.66s".to_string(),
        "────────────".to_string(),
        " Nextest run ID 441ae73b-fdc3-4816-9a0e-c486f071cbfe with nextest profile: default"
            .to_string(),
        "    Starting 4 tests across 2 binaries (1 test skipped)".to_string(),
    ];
    for (i, name) in failed_lines.iter().enumerate() {
        log.push(format!("        FAIL [   0.029s] ({}/4) {name}", i + 2));
        log.push("  stdout ───".to_string());
        log.push(String::new());
        log.push("    running 1 test".to_string());
        log.push(format!(
            "    test {} ... FAILED",
            name.rsplit(' ').next().unwrap()
        ));
        log.push(String::new());
        log.push("    failures:".to_string());
        log.push(String::new());
        log.push("    failures:".to_string());
        log.push(format!("        {}", name.rsplit(' ').next().unwrap()));
        log.push(String::new());
        log.push(
            "    test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 1 filtered \
             out; finished in 0.00s"
                .to_string(),
        );
        log.push(String::new());
        log.push("  stderr ───".to_string());
        log.push(String::new());
        log.push("    thread 'x' panicked at tests/result.rs:1:56:".to_string());
        log.push(String::new());
    }
    log.push("────────────".to_string());
    log.push(format!(
        "     Summary [   0.042s] {run} tests run: {passed} passed, {} failed, 1 skipped",
        failed_lines.len()
    ));
    for (i, name) in failed_lines.iter().enumerate() {
        log.push(format!("        FAIL [   0.029s] ({}/4) {name}", i + 2));
    }
    if !failed_lines.is_empty() {
        log.push("error: test run failed".to_string());
    }
    log.push(String::new());
    log.push("===== STEP doctest =====".to_string());
    for krate in ["jigc_engine", "cli"] {
        log.push(format!("   Doc-tests {krate}"));
        log.push(String::new());
        log.push("running 0 tests".to_string());
        log.push(String::new());
        log.push(summary_line(0, 0).trim_end().to_string());
        log.push(String::new());
    }
    log.join("\n") + "\n"
}

#[test]
fn a_nextest_run_is_totalled_from_its_summary_and_never_from_echoed_output() {
    let log = nextest_log(
        "4",
        2,
        &["nxprobe::result result::beta", "nxprobe tests::breaks"],
    );
    let report = report_over("gate-report-nextest-totals", &log);
    // 2 nextest binaries + the doctest step's 2 column-0 summaries; the two INDENTED
    // `test result: FAILED.` lines are a failing test's echoed stdout, not a binary.
    assert!(
        report
            .lines()
            .any(|l| l == "tests   passed=2 failed=2  (over 4 test binaries)"),
        "nextest's `Summary [` line carries the run's totals and its `Starting … across B \
         binaries` line its binary count; the indented `test result:` lines it echoes from \
         each failing test's captured output are neither.\nreport:\n{report}",
    );
}

#[test]
fn a_green_nextest_run_renders_the_line_the_build_harness_matches() {
    // `(10 slow)` is a real green run's shape: nextest annotates the passed count.
    let log = nextest_log("4222", 4222, &[])
        .replace(
            "Starting 4 tests across 2 binaries",
            "Starting 4222 tests across 15 binaries",
        )
        .replace("4222 passed,", "4222 passed (10 slow),");
    let report = report_over("gate-report-nextest-green", &log);
    let line = "tests   passed=4222 failed=0  (over 17 test binaries)";
    assert!(
        report.lines().any(|l| l == line),
        "a green nextest run plus the doctest step must render the whole-line totals \
         `.claude/workflows/milestone-build.js` accepts.\nexpected: {line}\nreport:\n{report}",
    );
    assert!(
        !report.contains("failing tests:"),
        "a green run names no failures.\nreport:\n{report}",
    );
}

#[test]
fn every_test_nextest_ran_and_did_not_pass_counts_as_failed() {
    // nextest splits the not-passed across `failed`, `timed out`, `exec failed`, … and a
    // run cut short prints `R/T tests run`. Failed is run minus passed, whatever the split.
    let log = "    Starting 6 tests across 1 binary\n\
               \x20    Summary [   9.001s] 5/6 tests run: 2 passed, 1 failed, 1 timed out, \
               1 exec failed, 0 skipped\n";
    let report = report_over("gate-report-nextest-split", log);
    assert!(
        report
            .lines()
            .any(|l| l == "tests   passed=2 failed=3  (over 1 test binaries)"),
        "a timed-out or un-executable test did not pass; it must not vanish from \
         `failed=`.\nreport:\n{report}",
    );
}

#[test]
fn nextest_failures_are_named_by_binary_and_test_once_each() {
    let log = nextest_log(
        "4",
        2,
        &["nxprobe::result result::beta", "nxprobe tests::breaks"],
    )
    .replace(
        "        FAIL [   0.029s] (3/4) nxprobe tests::breaks\n  stdout",
        "     TIMEOUT [ 480.003s] (3/4) nxprobe tests::breaks\n  stdout",
    );
    let report = report_over("gate-report-nextest-names", &log);
    let names: Vec<&str> = report
        .lines()
        .skip_while(|l| *l != "failing tests:")
        .skip(1)
        .take_while(|l| !l.is_empty())
        .collect();
    assert_eq!(
        names,
        vec!["  nxprobe tests::breaks", "  nxprobe::result result::beta"],
        "each failing test is named once — nextest prints it twice — as `<binary> <test>`, \
         without the duration or the progress counter, whatever its failure status \
         (FAIL, TIMEOUT, …).\nreport:\n{report}",
    );
}

// ---------------------------------------------------------------------------
// The live run under a fake `cargo`: which commands, in which order, and where it stops.
// ---------------------------------------------------------------------------
//
// The real commands are the suite, so the run's *shape* is fenced with a `cargo` that
// records each argv it is handed and does nothing else. Since 2026-10-05 the shape has
// three properties the arms below pin, each of them a way a red gate used to cost its
// full ten minutes (completions/artifacts/M55/fix-pass-rc25/perf/run-performance.md
// §3.6: 23 red gates, every failing test under 6.3 s, each reported after 8–10 min):
//
// * a red `fmt`, `clippy` or `build` launches no test;
// * the suite runs as a **fast tier** and then its **complement** — the second filter is
//   `not (<the first>)`, so the two cover every test whatever the first one holds — and a
//   red fast tier launches neither the second tier nor the doctests;
// * the fast tier alone is a pre-check, `--fast`, and that mode prints **neither** line
//   `.claude/workflows/milestone-build.js` accepts as a passed gate.
//
// The tier is derived, never listed: it is every test the last run on this target
// directory did not measure at the threshold or more, read from the record the gate
// itself writes out of nextest's JUnit report. The last arm of this section drives that
// loop twice, so a test nobody named moves to the second tier because a run timed it.

/// A fake `cargo` first on `PATH`, with the scratch directory that is the run's
/// `CARGO_TARGET_DIR` and `TMPDIR`.
///
/// The fake appends each argv to `argv.log`. For `nextest run` it also prints a green
/// three-test summary over two binaries and, when [`FakeCargo::junit`] staged a report for
/// the profile it was handed, writes that report where nextest would — under the store the
/// run's tool config names — and, when
/// `FAKE_CARGO_SKIP` holds a line, appends it to the file the gate names in
/// `JIGC_GATE_SKIPS`, as a test of that run does when it passes without running what it
/// tests. It exits 1 for a step named in `FAKE_CARGO_RED` — by its first word (`clippy`),
/// by its nextest profile (`gate-tier1`), or by a flag of its own without the dashes
/// (`no-run`: the build of the tests, and no other `cargo test`).
///
/// **And it refuses what nextest refuses.** A `nextest` call whose tool config names
/// `binary_id(=<FAKE_CARGO_GONE>)` in any profile exits 96 with nextest's own words and
/// does nothing else — `list` and `run` alike, as nextest 0.9.143 was probed to: every
/// profile of the config is compiled before anything is listed or run.
///
/// **And another gate may be running.** When [`FakeCargo::junit_of_another_gate`] staged
/// a report for the profile, a `nextest run` also leaves it at
/// `<target>/nextest/<profile>/junit.xml` — where a gate building into the same target
/// directory wrote its own until each run had a store of its own.
struct FakeCargo {
    scratch: ScratchDir,
}

/// What one gate run under the fake printed and launched.
struct FakeRun {
    text: String,
    /// Each cargo argv, in order, the per-run tier config's path replaced by `<tiers>`.
    argvs: Vec<String>,
    /// The body of the tier config the run wrote, empty when it wrote none.
    tiers: String,
    ok: bool,
}

impl FakeRun {
    /// The `default-filter` the run gave `profile`.
    fn filter(&self, profile: &str) -> &str {
        let header = format!("[profile.{profile}]");
        self.tiers
            .lines()
            .skip_while(|l| *l != header)
            .nth(1)
            .and_then(|l| l.strip_prefix("default-filter = '"))
            .and_then(|l| l.strip_suffix('\''))
            .unwrap_or_else(|| {
                panic!(
                    "the tier config gives `{profile}` no default-filter:\n{}",
                    self.tiers
                )
            })
    }
}

impl FakeCargo {
    fn new(label: &str, nextest: bool) -> Self {
        let scratch = ScratchDir::new(label);
        let dir = scratch.path();
        let bin = dir.join("bin");
        std::fs::create_dir_all(&bin).expect("create the fake cargo's dir");
        placed_executable::write(
            &bin.join("cargo"),
            format!(
                "#!/bin/sh\n\
                 here='{here}'\n\
                 if [ \"$1\" = nextest ] && [ {nextest} = 0 ]; then\n\
                 \x20 echo 'error: no such command: `nextest`' >&2\n\
                 \x20 exit 101\n\
                 fi\n\
                 printf '%s\\n' \"$*\" >> \"$here/argv.log\"\n\
                 profile=\n\
                 config=\n\
                 prev=\n\
                 for a in \"$@\"; do\n\
                 \x20 if [ \"$prev\" = --profile ]; then profile=$a; fi\n\
                 \x20 if [ \"$prev\" = --tool-config-file ]; then config=${{a#jigc-gate:}}; fi\n\
                 \x20 prev=$a\n\
                 done\n\
                 if [ \"$1\" = nextest ] && [ -n \"$FAKE_CARGO_GONE\" ] && [ -n \"$config\" ] \\\n\
                 \x20   && grep -q \"binary_id(=$FAKE_CARGO_GONE)\" \"$config\"; then\n\
                 \x20 echo \"error: for config file \\`$config\\` provided by tool \\`jigc-gate\\`, failed to parse profile.gate-tier1.default-filter\" >&2\n\
                 \x20 echo \"  error: operator didn't match any binary IDs\" >&2\n\
                 \x20 exit 96\n\
                 fi\n\
                 if [ \"$1 $2\" = 'nextest run' ]; then\n\
                 \x20 echo '    Starting 3 tests across 2 binaries'\n\
                 \x20 echo '     Summary [   0.100s] 3 tests run: 3 passed, 0 skipped'\n\
                 \x20 store=$(sed -n 's/^dir = \"\\(.*\\)\"$/\\1/p' \"$config\")\n\
                 \x20 if [ -f \"$here/junit-$profile.xml\" ]; then\n\
                 \x20   mkdir -p \"$store/$profile\"\n\
                 \x20   cp \"$here/junit-$profile.xml\" \"$store/$profile/junit.xml\"\n\
                 \x20 fi\n\
                 \x20 if [ -f \"$here/other-junit-$profile.xml\" ]; then\n\
                 \x20   mkdir -p \"$CARGO_TARGET_DIR/nextest/$profile\"\n\
                 \x20   cp \"$here/other-junit-$profile.xml\" \"$CARGO_TARGET_DIR/nextest/$profile/junit.xml\"\n\
                 \x20 fi\n\
                 \x20 if [ -n \"$FAKE_CARGO_SKIP\" ]; then\n\
                 \x20   printf '%s\\n' \"$FAKE_CARGO_SKIP\" >> \"$JIGC_GATE_SKIPS\"\n\
                 \x20 fi\n\
                 fi\n\
                 for red in $FAKE_CARGO_RED; do\n\
                 \x20 if [ \"$red\" = \"$1\" ] || [ \"$red\" = \"$profile\" ]; then exit 1; fi\n\
                 \x20 case \" $* \" in *\" --$red \"*) exit 1 ;; esac\n\
                 done\n\
                 exit 0\n",
                here = dir.display(),
                nextest = u8::from(nextest),
            ),
        );
        FakeCargo { scratch }
    }

    fn dir(&self) -> &Path {
        self.scratch.path()
    }

    /// Stage the JUnit report the fake writes for a `nextest run` under `profile`: one
    /// `(binary, test, seconds)` per test case, in nextest's own one-tag-per-line shape.
    fn junit(&self, profile: &str, cases: &[(&str, &str, &str)]) {
        self.stage_junit(&format!("junit-{profile}.xml"), cases);
    }

    /// Stage the report ANOTHER gate, running in the same target directory at the same
    /// time, leaves for `profile` while this run's tier runs.
    fn junit_of_another_gate(&self, profile: &str, cases: &[(&str, &str, &str)]) {
        self.stage_junit(&format!("other-junit-{profile}.xml"), cases);
    }

    fn stage_junit(&self, file: &str, cases: &[(&str, &str, &str)]) {
        let mut xml = String::from("<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n<testsuites>\n");
        for (binary, test, seconds) in cases {
            xml.push_str(&format!(
                "    <testsuite name=\"{binary}\" tests=\"1\">\n        <testcase \
                 name=\"{test}\" classname=\"{binary}\" \
                 timestamp=\"2026-10-05T19:41:23.446+02:00\" time=\"{seconds}\"/>\n    \
                 </testsuite>\n"
            ));
        }
        xml.push_str("</testsuites>\n");
        std::fs::write(self.dir().join(file), xml).expect("stage the fixture JUnit report");
    }

    /// The timing record of the target directory the fake's runs build into.
    fn record_path(&self) -> PathBuf {
        self.dir().join("jigc-gate/test-seconds.tsv")
    }

    /// Leave `text` as the timing record, as an earlier run — or whatever made it stale —
    /// left it.
    fn record(&self, text: &str) {
        let path = self.record_path();
        std::fs::create_dir_all(path.parent().expect("the record has a directory"))
            .expect("create the record's directory");
        std::fs::write(path, text).expect("write the fixture record");
    }

    /// The record as it now stands, row by row.
    fn recorded(&self) -> Vec<String> {
        std::fs::read_to_string(self.record_path())
            .expect("read the record")
            .lines()
            .map(str::to_owned)
            .collect()
    }

    /// Run `dev/gate <flags>` with the steps in `red` exiting 1.
    fn run(&self, flags: &[&str], red: &[&str]) -> FakeRun {
        self.run_where(flags, red, "")
    }

    /// [`FakeCargo::run`], with every test step of the run leaving `skip` as the line of a
    /// test that passed without running what it tests (none when empty).
    fn run_where(&self, flags: &[&str], red: &[&str], skip: &str) -> FakeRun {
        self.run_env(flags, red, &[("FAKE_CARGO_SKIP", skip)])
    }

    /// [`FakeCargo::run`], with the test binary `gone` no longer among those nextest has:
    /// a filter that names it is refused, as nextest refuses it.
    fn run_without(&self, flags: &[&str], red: &[&str], gone: &str) -> FakeRun {
        self.run_env(flags, red, &[("FAKE_CARGO_GONE", gone)])
    }

    fn run_env(&self, flags: &[&str], red: &[&str], more: &[(&str, &str)]) -> FakeRun {
        let dir = self.dir();
        let argv_log = dir.join("argv.log");
        std::fs::write(&argv_log, "").expect("start the run with an empty argv log");
        let out = gate_in(&self.scratch)
            .args(flags)
            .env(
                "PATH",
                format!("{}:/usr/bin:/bin", dir.join("bin").display()),
            )
            .env("CARGO_TARGET_DIR", dir)
            .env("JIGC_GATE_HYGIENE", "off")
            .env("FAKE_CARGO_RED", red.join(" "))
            .env_remove("FAKE_CARGO_SKIP")
            .env_remove("FAKE_CARGO_GONE")
            .envs(more.iter().copied())
            .output()
            .expect("spawn dev/gate");
        let mut tiers = String::new();
        let mut argvs = Vec::new();
        for line in std::fs::read_to_string(&argv_log)
            .expect("read the argv log")
            .lines()
        {
            let mut argv = line.to_string();
            if let Some(at) = line.find("jigc-gate:") {
                let path = line[at + "jigc-gate:".len()..]
                    .split(' ')
                    .next()
                    .expect("split yields a first field");
                tiers = std::fs::read_to_string(path).expect("read the run's tier config");
                argv = line.replace(path, "<tiers>");
            }
            argvs.push(argv);
        }
        FakeRun {
            text: String::from_utf8_lossy(&out.stdout).to_string(),
            argvs,
            tiers,
            ok: out.status.success(),
        }
    }
}

/// The three steps every mode opens with.
const LINT_AND_BUILD: [&str; 3] = [
    "fmt --check",
    "clippy --all-targets -- -D warnings",
    "build",
];
const TIER_1: &str = "nextest run --workspace --no-fail-fast --tool-config-file \
                      jigc-gate:<tiers> --profile gate-tier1";
const TIER_2: &str = "nextest run --workspace --no-fail-fast --no-tests=pass \
                      --tool-config-file jigc-gate:<tiers> --profile gate-tier2";
/// The question a run asks nextest before any tier, where a record gives it tiers to ask
/// about: does it take them? It lists the test binaries and executes none.
const PROBE: &str = "nextest list --workspace --list-type binaries-only --tool-config-file \
                     jigc-gate:<tiers> --profile gate-tier1";

/// The cargo argvs a run launched, without the `nextest --version` probe.
fn launched(run: &FakeRun) -> Vec<&str> {
    run.argvs
        .iter()
        .map(String::as_str)
        .filter(|a| *a != "nextest --version")
        .collect()
}

/// Does `text` carry either line the build harness accepts as a passed gate?
///
/// By substring, deliberately wider than the harness's whole-line patterns: a pre-check
/// must not be readable as a gate by an agent skimming it either.
fn reads_as_a_passed_gate(text: &str) -> bool {
    text.contains("GATE: PASS") || text.contains("passed=")
}

#[test]
fn the_full_gate_runs_the_fast_tier_then_its_complement_then_the_doctests() {
    let run = FakeCargo::new("gate-steps-nextest", true).run(&[], &[]);
    let text = &run.text;
    let mut expected = LINT_AND_BUILD.to_vec();
    expected.extend([TIER_1, TIER_2, "test --workspace --doc"]);
    assert_eq!(
        launched(&run),
        expected,
        "the gate's commands, in order — the suite as two nextest runs, then the doctests, \
         which nextest does not run.\n{text}",
    );
    let fast = run.filter("gate-tier1");
    assert_eq!(
        run.filter("gate-tier2"),
        format!("not ({fast})"),
        "the second tier is the complement of the first BY CONSTRUCTION — `not (<the fast \
         tier's filter>)` — so the two runs cover every test whatever the first one holds, \
         and never a second list that can drift from it.\n{}",
        run.tiers,
    );
    assert!(
        text.lines()
            .any(|l| l == "tests   passed=6 failed=0  (over 2 test binaries)"),
        "two tier runs sum their passed counts, and their binaries are the largest count \
         either run printed, never the sum: a binary with a test in each tier is in both \
         runs' counts.\n{text}",
    );
    assert!(text.lines().any(|l| l == "GATE: PASS"), "{text}");
}

#[test]
fn a_red_lint_or_build_step_launches_no_test() {
    for red in ["fmt", "clippy", "build"] {
        let run = FakeCargo::new(&format!("gate-red-{red}"), true).run(&[], &[red]);
        let text = &run.text;
        assert_eq!(
            run.argvs, LINT_AND_BUILD,
            "a red `{red}` ends the run after the three cheap steps: nothing is probed and \
             no test is launched — the suite is ten minutes spent to report what three \
             seconds already said.\n{text}",
        );
        assert!(
            text.lines()
                .any(|l| l == format!("GATE: FAIL (step: {red})")),
            "{text}",
        );
        assert!(!run.ok && !text.contains("GATE: PASS"), "{text}");
    }
}

#[test]
fn a_red_fast_tier_launches_neither_the_second_tier_nor_the_doctests() {
    let run = FakeCargo::new("gate-red-tier1", true).run(&[], &["gate-tier1"]);
    let text = &run.text;
    let mut expected = LINT_AND_BUILD.to_vec();
    expected.push(TIER_1);
    assert_eq!(launched(&run), expected, "{text}");
    assert!(
        text.lines().any(|l| l == "GATE: FAIL (step: tier1)"),
        "{text}",
    );
    assert!(
        text.contains("the second tier and the doctests were not run"),
        "a red that stops the run must say what it left unmeasured, or a reader takes the \
         named failures for the whole list.\n{text}",
    );
    assert!(!run.ok && !text.contains("GATE: PASS"), "{text}");
}

#[test]
fn a_red_second_tier_still_runs_the_doctests() {
    let run = FakeCargo::new("gate-red-tier2", true).run(&[], &["gate-tier2"]);
    let mut expected = LINT_AND_BUILD.to_vec();
    expected.extend([TIER_1, TIER_2, "test --workspace --doc"]);
    assert_eq!(launched(&run), expected, "{}", run.text);
    assert!(
        run.text.lines().any(|l| l == "GATE: FAIL (step: tier2)"),
        "{}",
        run.text,
    );
}

// ---------------------------------------------------------------------------
// `--keep-going`: the full gate, and no red step stops it.
// ---------------------------------------------------------------------------
//
// A plain gate stops at its first red stage, which is what a builder wants of a red and
// the wrong thing for a reader who holds one run's red against another's: a stabilization
// run accepts a record commit when its gate shows nothing red that the candidate's own
// gate did not, and under a candidate that is red early neither gate ran what lies behind
// the stop. So the gate can be asked to keep going. Three properties, each pinned below:
//
// * nothing red stops it — but a red build, after which there is no test to run: the
//   binary's, or the tests', which this mode builds as a step of its own (`test-build`),
//   so that tests that do not compile are a red build and never a red suite that names
//   no test;
// * the suite is ONE nextest run, the step `test`, never two tiers: a test's tier is this
//   machine's last measurement of it, so a failing test that took two seconds is in the
//   second tier on the next run, and the step it is red in would differ between two runs
//   that are red in the same test. It reads no timing record, so none can touch it;
// * green, it is a full gate like any other — the same tests, the same two lines — and
//   a plain `dev/gate` is exactly what it was.

/// The build of the tests, which a run that keeps going makes a step of its own.
const TEST_BUILD: &str = "test --workspace --no-run";
/// The whole suite, as a run that keeps going launches it.
const WHOLE_SUITE: &str = "nextest run --workspace --no-fail-fast --tool-config-file \
                           jigc-gate:<tiers> --profile gate-all";

#[test]
fn a_gate_that_keeps_going_runs_the_suite_as_one_step_and_green_is_a_full_gate() {
    let fake = FakeCargo::new("gate-keep-going", true);
    // A record nextest would refuse the tiers of: this mode reads none.
    fake.record("12.500\tjigc::g_gone\tslow_suite::drives_the_binary_a_lot\n");
    fake.junit(
        "gate-all",
        &[
            ("jigc::g_doc", "quick_suite::reads_a_registry", "0.020"),
            (
                "jigc::g_doc",
                "slow_suite::drives_the_binary_a_lot",
                "12.500",
            ),
        ],
    );
    let run = fake.run_without(&["--keep-going"], &[], "jigc::g_gone");
    let text = &run.text;
    let mut expected = LINT_AND_BUILD.to_vec();
    expected.extend([TEST_BUILD, WHOLE_SUITE, "test --workspace --doc"]);
    assert_eq!(
        launched(&run),
        expected,
        "the tests built, the suite as ONE nextest run, then the doctests — no tier, and \
         no question to ask nextest about one.\n{text}",
    );
    assert_eq!(
        run.filter("gate-all"),
        "all()",
        "the one run is every test: the filter names no test and no binary.\n{}",
        run.tiers,
    );
    assert!(
        !run.tiers.contains("gate-tier") && !run.tiers.contains("binary_id"),
        "its config holds nothing a timing record gave.\n{}",
        run.tiers,
    );
    assert!(
        run.ok
            && text
                .lines()
                .any(|l| l == "tests   passed=3 failed=0  (over 2 test binaries)")
            && text.lines().any(|l| l == "GATE: PASS"),
        "green, it prints the two lines of a full gate — it ran every test.\n{text}",
    );
    assert!(
        text.lines()
            .any(|l| l.starts_with("gate: mode   full, keep-going (")),
        "and its mode line says which full gate it is — the line `dev/stabilize-record` \
         reads before it takes a red gate as evidence.\n{text}",
    );
    assert_eq!(
        fake.recorded(),
        vec![
            "0.020\tjigc::g_doc\tquick_suite::reads_a_registry",
            "12.500\tjigc::g_doc\tslow_suite::drives_the_binary_a_lot",
        ],
        "one green run of every test measured every test: its rows are the record.",
    );

    // A plain gate is what it was: its mode line, and (the arms above) its commands.
    let plain = FakeCargo::new("gate-keep-going-plain", true).run(&[], &[]);
    assert!(
        plain.text.lines().any(|l| l == "gate: mode   full"),
        "{}",
        plain.text
    );
}

#[test]
fn a_gate_that_keeps_going_is_stopped_by_no_red_step_but_a_red_build() {
    let mut whole = LINT_AND_BUILD.to_vec();
    whole.extend([TEST_BUILD, WHOLE_SUITE, "test --workspace --doc"]);

    // A red format check and a red lint: every test still runs.
    let run =
        FakeCargo::new("gate-keep-going-lint", true).run(&["--keep-going"], &["fmt", "clippy"]);
    assert_eq!(launched(&run), whole, "{}", run.text);
    assert!(
        !run.ok
            && run
                .text
                .lines()
                .any(|l| l == "GATE: FAIL (step: fmt clippy)"),
        "{}",
        run.text,
    );

    // A red suite: the doctests still run, and the verdict names every red step.
    let run = FakeCargo::new("gate-keep-going-suite", true)
        .run(&["--keep-going"], &["clippy", "gate-all"]);
    assert_eq!(launched(&run), whole, "{}", run.text);
    assert!(
        !run.ok
            && run
                .text
                .lines()
                .any(|l| l == "GATE: FAIL (step: clippy test)"),
        "{}",
        run.text,
    );
    assert!(
        !run.text.contains("not run") && !run.text.contains("GATE: PASS"),
        "a run that kept going left nothing unmeasured to warn of, and a red one is no \
         passed gate.\n{}",
        run.text,
    );

    // Tests that do not build: a red build like the other, and no test is launched.
    let run = FakeCargo::new("gate-keep-going-test-build", true)
        .run(&["--keep-going"], &["clippy", "no-run"]);
    let mut expected = LINT_AND_BUILD.to_vec();
    expected.push(TEST_BUILD);
    assert_eq!(launched(&run), expected, "{}", run.text);
    assert!(
        run.text
            .lines()
            .any(|l| l == "GATE: FAIL (step: clippy test-build)")
            && run.text.contains("the tests do not build: none was run"),
        "tests that do not compile are a red BUILD step — never a red suite that names no \
         failing test, which says nothing about which tests are red.\n{}",
        run.text,
    );

    // A red build: there is nothing to run a test from, in any mode — and the run says so.
    let run = FakeCargo::new("gate-keep-going-build", true).run(&["--keep-going"], &["build"]);
    assert_eq!(run.argvs, LINT_AND_BUILD, "{}", run.text);
    assert!(
        run.text.lines().any(|l| l == "GATE: FAIL (step: build)")
            && run
                .text
                .contains("the build is red: no test can be built, so none was run"),
        "{}",
        run.text,
    );
}

#[test]
fn keeping_going_is_a_mode_of_the_full_gate_and_of_nothing_else() {
    for pre_check in ["--fast", "--quick"] {
        let fake = FakeCargo::new("gate-keep-going-pre-check", true);
        let out = gate_in(&fake.scratch)
            .args(["--keep-going", pre_check])
            .env(
                "PATH",
                format!("{}:/usr/bin:/bin", fake.dir().join("bin").display()),
            )
            .env("CARGO_TARGET_DIR", fake.dir())
            .output()
            .expect("spawn dev/gate");
        assert_eq!(
            out.status.code(),
            Some(2),
            "`--keep-going {pre_check}`: a pre-check that kept going is neither a gate nor \
             a quick answer.",
        );
        assert!(
            String::from_utf8_lossy(&out.stderr)
                .contains("--keep-going is the full gate with no stop"),
            "{}",
            String::from_utf8_lossy(&out.stderr),
        );
        assert!(
            !fake.dir().join("argv.log").exists(),
            "a usage error launches nothing.",
        );
    }

    // Without nextest the suite is one step already — `cargo test`, doctests included.
    let run = FakeCargo::new("gate-keep-going-fallback", false).run(&["--keep-going"], &[]);
    let mut expected = LINT_AND_BUILD.to_vec();
    expected.extend([TEST_BUILD, "test --workspace --no-fail-fast"]);
    assert_eq!(run.argvs, expected, "{}", run.text);
    assert!(run.ok && run.text.contains("GATE: PASS"), "{}", run.text);
}

/// `--fast` is the fast tier as a pre-check, and it must never read as a gate.
///
/// `.claude/workflows/milestone-build.js` accepts an increment's gate on two lines —
/// `GATE: PASS` and the `tests   passed=… failed=0  (over N test binaries)` totals — and
/// `--quick` already prints the first of them. A pre-check that ran thousands of green
/// tests would print both unless it is built not to, so a green `--fast` prints neither,
/// in any form, and its own verdict says what it is not.
#[test]
fn the_fast_pre_check_runs_the_fast_tier_only_and_never_reads_as_a_gate() {
    let fake = FakeCargo::new("gate-fast", true);
    let run = fake.run(&["--fast"], &[]);
    let text = &run.text;
    let mut expected = LINT_AND_BUILD.to_vec();
    expected.push(TIER_1);
    assert_eq!(
        launched(&run),
        expected,
        "the pre-check is lint, build and the fast tier — never the second tier, never \
         the doctests.\n{text}",
    );
    assert!(run.ok, "a green pre-check exits 0.\n{text}");
    assert!(
        !reads_as_a_passed_gate(text),
        "a green pre-check prints neither line the build harness accepts as a passed \
         gate.\n{text}",
    );
    assert!(
        text.lines().any(|l| l.starts_with("PRE-CHECK: PASS")
            && l.contains("NOT a gate")
            && l.contains("never commit on it")),
        "its verdict is its own, and says what it is not.\n{text}",
    );
    assert!(
        text.lines()
            .any(|l| l.starts_with("fast tier  3 passed, 0 failed")),
        "it still says how much it measured.\n{text}",
    );

    let red = fake.run(&["--fast"], &["gate-tier1"]);
    assert!(
        red.text
            .lines()
            .any(|l| l == "PRE-CHECK: FAIL (step: tier1)"),
        "{}",
        red.text,
    );
    assert!(
        !red.ok && !red.text.contains("GATE:"),
        "a red pre-check is not a gate verdict either.\n{}",
        red.text,
    );
}

#[test]
fn without_nextest_the_full_gate_still_runs_the_full_suite_and_says_why() {
    let fake = FakeCargo::new("gate-steps-fallback", false);
    let run = fake.run(&[], &[]);
    let text = &run.text;
    let mut expected = LINT_AND_BUILD.to_vec();
    expected.push("test --workspace --no-fail-fast");
    assert_eq!(
        run.argvs, expected,
        "with nextest absent the test step is `cargo test` — the whole suite, doctests \
         included — never a skipped step and never a smaller suite.\n{text}",
    );
    assert!(
        text.contains("cargo-nextest is not installed")
            && text.contains("cargo install cargo-nextest --locked"),
        "the fallback is announced, with the line that ends it.\n{text}",
    );
    assert!(text.contains("GATE: PASS"), "{text}");

    // The tiers are nextest filters, so without nextest there is no fast tier to run: the
    // pre-check says so and stays a pre-check, rather than running the whole suite.
    let fast = fake.run(&["--fast"], &[]);
    assert_eq!(fast.argvs, LINT_AND_BUILD, "{}", fast.text);
    assert!(
        fast.text.contains("cargo-nextest is not installed")
            && fast.text.contains("the fast tier did not run")
            && !reads_as_a_passed_gate(&fast.text),
        "{}",
        fast.text,
    );
}

/// The tier is derived from what the gate measured, and a new test needs no one to place it.
///
/// Run 1 has no record: every test is in the fast tier. Its fast tier's JUnit report
/// times three tests, one of them at 12.5 s. Run 2, on the same target directory, holds
/// that test — by binary and by name — out of the fast tier, and by construction in the
/// second. Nothing here lists a suite, which is the point: the list this replaces would
/// have rotted the day a suite was added.
#[test]
fn the_fast_tier_is_every_test_the_last_run_did_not_measure_as_slow() {
    let fake = FakeCargo::new("gate-tier-derivation", true);
    fake.junit(
        "gate-tier1",
        &[
            (
                "jigc::g_doc",
                "slow_suite::drives_the_binary_a_lot",
                "12.500",
            ),
            ("jigc::g_doc", "quick_suite::reads_a_registry", "0.020"),
            ("jigc-engine", "parse::tests::round_trips", "1.999"),
        ],
    );

    let first = fake.run(&[], &[]);
    assert_eq!(
        first.filter("gate-tier1"),
        "not (none())",
        "with no record every test is in the fast tier — a test nobody has timed is never \
         assumed slow.\n{}",
        first.text,
    );
    assert!(
        first.text.contains("no timing record"),
        "a run whose fast tier is the whole suite must say so.\n{}",
        first.text,
    );
    let record = std::fs::read_to_string(fake.dir().join("jigc-gate/test-seconds.tsv"))
        .expect("the run writes the record under the target directory it built into");
    assert_eq!(
        record.lines().collect::<Vec<_>>(),
        vec![
            "12.500\tjigc::g_doc\tslow_suite::drives_the_binary_a_lot",
            "0.020\tjigc::g_doc\tquick_suite::reads_a_registry",
            "1.999\tjigc-engine\tparse::tests::round_trips",
        ],
        "one row per test the run measured: seconds, binary, test.",
    );

    let second = fake.run(&[], &[]);
    assert_eq!(
        second.filter("gate-tier1"),
        "not ((binary_id(=jigc::g_doc) & test(/^(slow_suite::drives_the_binary_a_lot)$/)))",
        "the one test measured at 2 s or more leaves the fast tier, named by binary and \
         by test; 1.999 s is under the threshold and stays.\n{}",
        second.text,
    );
    assert_eq!(
        second.filter("gate-tier2"),
        format!("not ({})", second.filter("gate-tier1")),
        "{}",
        second.tiers,
    );
}

// ---------------------------------------------------------------------------
// The timing record, stale against the tree: it costs time, never a red gate.
// ---------------------------------------------------------------------------
//
// The record is this machine's last measurement, and nothing keeps it true of the tree.
// The review of the stabilization build probed one way that bites (its M11): a row for a
// test binary that no longer exists puts `binary_id(=<gone>)` in the fast tier's filter,
// nextest refuses the filter before it runs a test (exit 96), a tier that never runs
// writes no row, and the gate is red until somebody deletes the record. That is one member
// of a class — every way the record can be stale against the tree — and each is decided:
//
// | the record…                                   | what the gate does |
// |---|---|
// | names a binary the tree no longer has (removed, renamed, another branch or checkout built into this target directory) | asks nextest before any tier; refused, the run goes on with no record — the fast tier is the whole suite — and its timings replace the record |
// | names a test the tree no longer has           | the name matches nothing; the row leaves when a run whose tiers are all green rewrites the record |
// | holds a line that is no row (cut short, two run together) | read by nobody; a line shaped like a row is one of the two above |
// | holds seconds that are no longer true         | a test moves between the tiers: time |
// | was written by another target directory       | not read: the record lives in the target directory it measures (pinned above) |
// | is being written by another gate              | replaced by a rename: a reader has the old one or the new one, whole |
// | — and that gate's JUnit reports               | each run has a store of its own |

/// The review's M11, and the member of the class that reddens a gate.
///
/// Whatever makes nextest refuse the tiers a record gives — here a binary that is gone —
/// the run goes on as if there were no record, says so, and heals the record: the next
/// run is tiered again. The question it asks nextest is no step: it is in no verdict.
#[test]
fn a_record_that_names_a_binary_the_tree_no_longer_has_never_reddens_the_gate() {
    let stale = "12.500\tjigc::g_gone\tslow_suite::drives_the_binary_a_lot\n\
                 9.000\tjigc::g_doc\tslow_suite::still_here\n";
    let measured = [
        ("jigc::g_doc", "slow_suite::still_here", "9.100"),
        ("jigc::g_doc", "quick_suite::reads_a_registry", "0.020"),
    ];
    let rows = vec![
        "9.100\tjigc::g_doc\tslow_suite::still_here".to_owned(),
        "0.020\tjigc::g_doc\tquick_suite::reads_a_registry".to_owned(),
    ];

    let fake = FakeCargo::new("gate-stale-binary", true);
    fake.record(stale);
    fake.junit("gate-tier1", &measured);
    let run = fake.run_without(&[], &[], "jigc::g_gone");
    let text = &run.text;
    assert!(
        run.ok && text.lines().any(|l| l == "GATE: PASS"),
        "a row for a test binary that is gone is a stale measurement, never a verdict \
         about the tree: the gate is as green as its steps.\n{text}",
    );
    let mut expected = LINT_AND_BUILD.to_vec();
    expected.extend([PROBE, TIER_1, TIER_2, "test --workspace --doc"]);
    assert_eq!(
        launched(&run),
        expected,
        "the run asks nextest whether it takes the record's tiers before any tier, and \
         then runs every step of a full gate.\n{text}",
    );
    assert_eq!(
        (run.filter("gate-tier1"), run.filter("gate-tier2")),
        ("not (none())", "not (not (none()))"),
        "refused, the run goes on as if there were no record: the fast tier is the whole \
         suite, and the second tier is still its complement.\n{}",
        run.tiers,
    );
    assert!(
        text.contains("nextest does not take the tiers the timing record gives (exit 96")
            && text.contains("the fast tier is the WHOLE suite this run")
            && text.contains("replace the record"),
        "a run that sets its record aside says so, and why.\n{text}",
    );
    assert_eq!(
        fake.recorded(),
        rows,
        "its timings ARE the record afterwards — the row for the binary that is gone went \
         with the record it was in.",
    );

    let healed = fake.run_without(&[], &[], "jigc::g_gone");
    assert_eq!(
        healed.filter("gate-tier1"),
        "not ((binary_id(=jigc::g_doc) & test(/^(slow_suite::still_here)$/)))",
        "and the run after it is tiered again: one slow run, nothing to delete by hand.\n{}",
        healed.text,
    );
    assert!(
        healed.ok && !healed.text.contains("does not take"),
        "{}",
        healed.text
    );

    // The pre-check heals the same record the same way — a red tier that ran nothing
    // would otherwise be all `--fast` could ever print on this target directory.
    let fake = FakeCargo::new("gate-stale-binary-fast", true);
    fake.record(stale);
    fake.junit("gate-tier1", &measured);
    let fast = fake.run_without(&["--fast"], &[], "jigc::g_gone");
    let mut expected = LINT_AND_BUILD.to_vec();
    expected.extend([PROBE, TIER_1]);
    assert_eq!(launched(&fast), expected, "{}", fast.text);
    assert!(
        fast.ok && fast.text.lines().any(|l| l.starts_with("PRE-CHECK: PASS")),
        "{}",
        fast.text,
    );
    assert_eq!(fake.recorded(), rows);

    // Whatever else makes nextest refuse the question — here the tier's own step is red
    // too — the question is in no verdict: the run names the step that is red.
    let fake = FakeCargo::new("gate-stale-probe-red", true);
    fake.record(stale);
    let red = fake.run(&[], &["gate-tier1"]);
    assert!(
        !red.ok && red.text.lines().any(|l| l == "GATE: FAIL (step: tier1)"),
        "the probe is no step: a red gate names the steps that are red, and only \
         those.\n{}",
        red.text,
    );
    assert_eq!(
        fake.recorded(),
        stale.lines().collect::<Vec<_>>(),
        "a run that measured nothing leaves the record as it found it.",
    );
}

/// A line that is no row is read by nobody.
///
/// A record cut short, or two writers' rows run together, leaves lines no run wrote
/// whole. Each is skipped by every reader of the record — the filter, the count the
/// header prints, the merge — so its test is a test without a row, which runs in the
/// fast tier; and a row whose test is gone names nothing. The run after rewrites the
/// record from what it ran.
#[test]
fn a_line_of_the_record_that_is_no_row_is_read_by_nobody() {
    let fake = FakeCargo::new("gate-record-garbled", true);
    fake.record(
        "12.500\tjigc::g_doc\tslow_suite::whole\n\
         9.000\tjigc::g_d\n\
         soon\tjigc::g_doc\tslow_suite::not_a_number\n\
         7.000\tjigc::g_doc\tslow_suite::run\t3.000\tjigc::g_doc\tslow_suite::together\n\
         \n\
         8.000\tjigc::g_doc\tslow_suite::a_test_that_is_go\n",
    );
    fake.junit(
        "gate-tier1",
        &[("jigc::g_doc", "quick_suite::reads_a_registry", "0.020")],
    );
    fake.junit(
        "gate-tier2",
        &[("jigc::g_doc", "slow_suite::whole", "12.700")],
    );
    let run = fake.run(&[], &[]);
    assert_eq!(
        run.filter("gate-tier1"),
        "not ((binary_id(=jigc::g_doc) & \
         test(/^(slow_suite::a_test_that_is_go|slow_suite::whole)$/)))",
        "only a line of exactly three fields whose first is a number of seconds is a \
         row. The name that was cut is a row and names no test, which is harmless.\n{}",
        run.text,
    );
    assert!(
        run.text
            .contains("2 of 2 recorded tests wait for the second tier"),
        "the header counts rows, not lines.\n{}",
        run.text,
    );
    assert!(run.ok, "{}", run.text);
    assert_eq!(
        fake.recorded(),
        vec![
            "0.020\tjigc::g_doc\tquick_suite::reads_a_registry",
            "12.700\tjigc::g_doc\tslow_suite::whole",
        ],
        "two green tiers measured every test there is, so their rows are the record: \
         what was no row is gone, and so is the row of the test that is gone.",
    );

    // A run that did not measure every test keeps the rows it did not measure — the
    // rows, never the lines that are none.
    let fake = FakeCargo::new("gate-record-garbled-fast", true);
    fake.record("12.500\tjigc::g_doc\tslow_suite::whole\n9.000\tjigc::g_d\n");
    fake.junit(
        "gate-tier1",
        &[("jigc::g_doc", "quick_suite::reads_a_registry", "0.020")],
    );
    let fast = fake.run(&["--fast"], &[]);
    assert!(fast.ok, "{}", fast.text);
    assert_eq!(
        fake.recorded(),
        vec![
            "0.020\tjigc::g_doc\tquick_suite::reads_a_registry",
            "12.500\tjigc::g_doc\tslow_suite::whole",
        ],
    );
}

/// Two gates on one target directory: a gate reading the record while another replaces
/// it reads one record, whole.
///
/// The record is replaced by a rename, never rewritten in place — so a reader that has it
/// open keeps the record it opened. Held here by exactly that: the file this test opened
/// before the run still reads as the old record after it.
#[test]
fn a_run_replaces_the_record_by_a_rename_so_a_reader_has_one_record_whole() {
    use std::io::Read;
    let before = "12.500\tjigc::g_doc\tslow_suite::before\n";
    for flags in [&[][..], &["--fast"][..]] {
        let fake = FakeCargo::new("gate-record-rename", true);
        fake.record(before);
        fake.junit(
            "gate-tier1",
            &[("jigc::g_doc", "quick_suite::after", "0.020")],
        );
        let mut reader = std::fs::File::open(fake.record_path()).expect("open the record");
        let run = fake.run(flags, &[]);
        assert!(run.ok, "{}", run.text);
        let mut seen = String::new();
        reader
            .read_to_string(&mut seen)
            .expect("read the record that was opened before the run");
        assert_eq!(
            seen, before,
            "{flags:?}: a gate that opened the record before this run replaced it reads \
             the record it opened — rewritten in place, it would read whatever part of \
             the new one was there.",
        );
        assert!(
            fake.recorded()
                .contains(&"0.020\tjigc::g_doc\tquick_suite::after".to_owned()),
            "{flags:?}: and the record is the new one: {:?}",
            fake.recorded(),
        );
        let litter: Vec<_> = std::fs::read_dir(fake.dir().join("jigc-gate"))
            .expect("list the record's directory")
            .map(|entry| entry.expect("a directory entry").file_name())
            .collect();
        assert_eq!(
            litter,
            ["test-seconds.tsv"],
            "{flags:?}: the file the new record was written to is the record now.",
        );
    }
}

/// Two gates on one target directory: neither reads the other's JUnit report.
///
/// nextest writes a profile's report at one path of its store, so two gates with one store
/// — the target directory's, as it was — are two writers of one file: a run folded the
/// other's seconds into its log and into the record. Each run's store is its own.
#[test]
fn each_run_keeps_its_junit_reports_in_a_store_of_its_own() {
    let fake = FakeCargo::new("gate-store-per-run", true);
    fake.junit(
        "gate-tier1",
        &[("jigc::g_doc", "quick_suite::this_runs_own", "0.020")],
    );
    fake.junit_of_another_gate(
        "gate-tier1",
        &[("jigc::g_other", "elsewhere::another_gates_test", "44.000")],
    );
    let run = fake.run(&[], &[]);
    assert!(run.ok, "{}", run.text);
    assert_eq!(
        fake.recorded(),
        vec!["0.020\tjigc::g_doc\tquick_suite::this_runs_own"],
        "the report a gate running beside this one left in the target directory is not \
         this run's, and none of its rows is.\n{}",
        run.text,
    );
    let store_of = |run: &FakeRun| -> String {
        run.tiers
            .lines()
            .skip_while(|l| *l != "[store]")
            .nth(1)
            .unwrap_or_else(|| panic!("the tier config names a store:\n{}", run.tiers))
            .to_owned()
    };
    let again = fake.run(&[], &[]);
    assert_ne!(
        store_of(&run),
        store_of(&again),
        "two runs, two stores — whatever target directory they share.",
    );
}

#[test]
fn a_threshold_that_is_not_a_number_is_refused_before_anything_runs() {
    let fake = FakeCargo::new("gate-bad-threshold", true);
    let out = gate_in(&fake.scratch)
        .env(
            "PATH",
            format!("{}:/usr/bin:/bin", fake.dir().join("bin").display()),
        )
        .env("CARGO_TARGET_DIR", fake.dir())
        .env("JIGC_GATE_FAST_UNDER", "soon")
        .output()
        .expect("spawn dev/gate");
    assert_eq!(
        out.status.code(),
        Some(2),
        "awk reads a non-number as zero, and at zero every recorded test is slow: the \
         fast tier would silently be the unrecorded tests alone.",
    );
    assert!(
        String::from_utf8_lossy(&out.stderr)
            .contains("JIGC_GATE_FAST_UNDER must be a number of seconds, got: soon"),
        "{}",
        String::from_utf8_lossy(&out.stderr),
    );
    assert!(
        !fake.dir().join("argv.log").exists(),
        "a usage error launches nothing.",
    );
}

/// Lever 6: the log carries the timings, so the next analysis need not re-run the suite.
#[test]
fn the_log_carries_each_steps_seconds_and_each_tests_seconds() {
    let fake = FakeCargo::new("gate-log-timings", true);
    fake.junit(
        "gate-tier1",
        &[("jigc::g_doc", "quick_suite::reads_a_registry", "0.020")],
    );
    fake.junit(
        "gate-tier2",
        &[
            (
                "jigc::g_doc",
                "slow_suite::drives_the_binary_a_lot",
                "12.500",
            ),
            ("jigc::g_doc", "slow_suite::drives_it_again", "30.300"),
        ],
    );
    let run = fake.run(&[], &[]);
    let text = &run.text;
    let store = run
        .tiers
        .lines()
        .skip_while(|l| *l != "[store]")
        .nth(1)
        .and_then(|l| l.strip_prefix("dir = \""))
        .and_then(|l| l.strip_suffix('"'))
        .unwrap_or_else(|| panic!("the run tells nextest where its store is.\n{}", run.tiers));
    assert!(
        Path::new(store).starts_with(fake.dir()) && Path::new(store).is_dir(),
        "the run tells nextest where its store is — a directory of the run's own under \
         `$TMPDIR` (this rig's scratch directory). Left to itself nextest writes its JUnit \
         report under the workspace's own `target/` whatever `CARGO_TARGET_DIR` says, where \
         this run would not find it and a gate running there would lose its own.\n{}",
        run.tiers,
    );
    let log_path = text
        .lines()
        .find_map(|l| l.strip_prefix("gate: log    "))
        .unwrap_or_else(|| panic!("the header names the log.\n{text}"));
    let log = std::fs::read_to_string(log_path).expect("read the run's log");
    for step in ["fmt", "clippy", "build", "tier1", "tier2", "doctest"] {
        let prefix = format!("step-seconds\t{step}\t0\t");
        assert!(
            log.lines().any(|l| l
                .strip_prefix(&prefix)
                .is_some_and(|s| s.parse::<u64>().is_ok())),
            "the log holds `{step}`'s exit code and elapsed seconds at column 0 — they were \
             on stdout only, which no log survives.\nlog:\n{log}",
        );
    }
    for row in [
        "test-seconds\t0.020\tjigc::g_doc\tquick_suite::reads_a_registry",
        "test-seconds\t12.500\tjigc::g_doc\tslow_suite::drives_the_binary_a_lot",
        "test-seconds\t30.300\tjigc::g_doc\tslow_suite::drives_it_again",
    ] {
        assert!(
            log.lines().any(|l| l == row),
            "the log holds every measured test's seconds, both tiers'.\nwant: {row}\nlog:\n{log}",
        );
    }
    assert!(
        text.lines()
            .any(|l| l == "    42.8s  jigc::g_doc slow_suite  (2 tests)")
            && text
                .lines()
                .any(|l| l == "    30.3s  jigc::g_doc slow_suite::drives_it_again"),
        "the run prints its heaviest suites and tests, so a suite's cost is seen when it \
         lands rather than by an audit.\n{text}",
    );
}

#[test]
fn a_green_two_tier_log_totals_what_one_run_over_the_same_tests_did() {
    // The same 4222 tests over the same 15 binaries as the one-run fixture above, cut in
    // two. A tier's `Starting` line counts the binaries that hold a test it selected; here
    // every binary holds a test of each tier, so both lines say fifteen.
    let one = nextest_log("4222", 4222, &[]).replace(
        "Starting 4 tests across 2 binaries (1 test skipped)",
        "Starting 4222 tests across 15 binaries",
    );
    let doctests_at = one
        .find("===== STEP doctest =====")
        .expect("the fixture ends with the doctest step");
    let (suite, doctests) = one.split_at(doctests_at);
    let tier = |run: u32, skipped: u32| {
        suite
            .replace(
                "Starting 4222 tests across 15 binaries",
                &format!("Starting {run} tests across 15 binaries ({skipped} tests skipped)"),
            )
            .replace(
                "4222 tests run: 4222 passed, 0 failed, 1 skipped",
                &format!("{run} tests run: {run} passed, {skipped} skipped"),
            )
    };
    let log = format!("{}{}{doctests}", tier(3100, 1122), tier(1122, 3100));
    let report = report_over("gate-report-two-tiers", &log);
    assert!(
        report
            .lines()
            .any(|l| l == "tests   passed=4222 failed=0  (over 17 test binaries)"),
        "a green gate's totals line reads as it did before the suite ran in two tiers: the \
         passed counts sum, and the fifteen binaries both runs print are fifteen, not \
         thirty.\nreport:\n{report}",
    );

    // The count is the LARGEST any run printed, so it can understate and never inflate:
    // with one binary all slow and another all fast, no run reaches all fifteen.
    let log = format!(
        "{}{}{doctests}",
        tier(3100, 1122).replace("across 15 binaries", "across 14 binaries"),
        tier(1122, 3100).replace("across 15 binaries", "across 9 binaries"),
    );
    let report = report_over("gate-report-two-tiers-apart", &log);
    assert!(
        report
            .lines()
            .any(|l| l == "tests   passed=4222 failed=0  (over 16 test binaries)"),
        "fourteen and nine are at least fourteen binaries and at most twenty-three; the \
         count says fourteen (and the doctest step's two), because a sum would count a \
         binary with a test in each tier twice.\nreport:\n{report}",
    );
}

#[test]
fn the_report_names_the_heaviest_suites_and_tests_of_a_log_that_carries_them() {
    let log = "test-seconds\t0.5\tjigc::g_doc\talpha::one\n\
               test-seconds\t3.26\tjigc::g_doc\talpha::two\n\
               test-seconds\t9\tjigc::g_flow\tbeta::only\n\
               \x20   test-seconds\t99\tjigc::g_flow\techoed::from_a_failing_tests_output\n\
               test-seconds\t0.01\tjigc-engine\tparse::tests::tiny\n";
    let report = report_over("gate-report-heaviest", log);
    let block: Vec<&str> = report
        .lines()
        .skip_while(|l| !l.starts_with("--- heaviest"))
        .collect();
    assert_eq!(
        block,
        vec![
            "--- heaviest of this run (seconds; every test is a `test-seconds` line in the log) ---",
            "suites:",
            "     9.0s  jigc::g_flow beta  (1 tests)",
            "     3.8s  jigc::g_doc alpha  (2 tests)",
            "     0.0s  jigc-engine parse  (1 tests)",
            "tests:",
            "     9.0s  jigc::g_flow beta::only",
            "     3.3s  jigc::g_doc alpha::two",
            "     0.5s  jigc::g_doc alpha::one",
            "     0.0s  jigc-engine parse::tests::tiny",
        ],
        "suites by summed seconds and tests by their own, heaviest first; an indented copy \
         of the row — a failing test's echoed output — is not a row.\nreport:\n{report}",
    );

    let bare = report_over("gate-report-no-timings", &summary_line(3, 0));
    assert!(
        !bare.contains("heaviest"),
        "a log with no timings prints no empty block.\nreport:\n{bare}",
    );
}

/// A test that passes without running what it tests is named by the gate itself.
///
/// The suites that run the stabilization harness need `node`; without it they fail under
/// CI and pass anywhere else, saying so on stderr — which nextest shows nobody for a test
/// that passed. So the gate names a file, such a test appends its line to it, and the
/// summary carries each line under a NOTE, beside the verdict it does not change.
#[test]
fn a_test_that_passed_without_running_is_named_beside_the_verdict() {
    let skip = "SKIPPED: `node` is not on PATH, so no stage was run";
    let fake = FakeCargo::new("gate-skips", true);
    let run = fake.run_where(&[], &[], skip);
    let text = &run.text;
    let note: Vec<&str> = text
        .lines()
        .skip_while(|l| !l.starts_with("==> NOTE    2 test(s) PASSED WITHOUT RUNNING"))
        .take(2)
        .collect();
    assert_eq!(
        note,
        vec![
            "==> NOTE    2 test(s) PASSED WITHOUT RUNNING what they test on this machine:",
            "               2  SKIPPED: `node` is not on PATH, so no stage was run",
        ],
        "each tier's test left the line: two tests, and the line once with its count.\n{text}",
    );
    assert!(
        run.ok && reads_as_a_passed_gate(text),
        "the NOTE is no step: the gate is as green as its steps, and its two lines stand.\n{text}",
    );
    assert!(
        text.find("==> NOTE    2 test(s)") < text.find("\nGATE: PASS"),
        "it stands above the verdict, in the lines a reader of the gate's end reads.\n{text}",
    );

    // A run in which no test said so prints no NOTE — and the pre-check prints one too.
    let quiet = fake.run(&[], &[]);
    assert!(
        !quiet.text.contains("PASSED WITHOUT RUNNING"),
        "{}",
        quiet.text
    );
    let fast = fake.run_where(&["--fast"], &[], skip);
    assert!(
        fast.text.contains("PASSED WITHOUT RUNNING"),
        "{}",
        fast.text
    );

    // The log keeps each line, so `--report` prints the same NOTE: every test counted,
    // each distinct line once, and a copy a failing test's output echoed — indented — is
    // not one.
    let log = format!(
        "{}test-skipped\tSKIPPED: one\ntest-skipped\tSKIPPED: one\n\
         test-skipped\tSKIPPED: two\twith a tab\n\x20   test-skipped\tSKIPPED: echoed\n",
        summary_line(3, 0)
    );
    let report = report_over("gate-report-skips", &log);
    let block: Vec<&str> = report
        .lines()
        .skip_while(|l| !l.starts_with("==> NOTE"))
        .collect();
    assert_eq!(
        block,
        vec![
            "==> NOTE    3 test(s) PASSED WITHOUT RUNNING what they test on this machine:",
            "               2  SKIPPED: one",
            "               1  SKIPPED: two\twith a tab",
        ],
        "report:\n{report}",
    );
}

/// Lever 9: a scoped run has a profile that turns a hung test into a named timeout —
/// and the gate never runs under it.
#[test]
fn the_scoped_profile_terminates_a_hung_test_and_the_gate_never_uses_it() {
    let config = std::fs::read_to_string(repo_root().join(".config/nextest.toml"))
        .expect(".config/nextest.toml is readable");
    let scoped: Vec<&str> = config
        .lines()
        .skip_while(|l| *l != "[profile.scoped]")
        .skip(1)
        .take_while(|l| !l.starts_with('['))
        .filter(|l| !l.trim_start().starts_with('#'))
        .collect();
    assert!(
        scoped
            .iter()
            .any(|l| l.starts_with("slow-timeout") && l.contains("terminate-after")),
        "`[profile.scoped]` must set a `slow-timeout` with a `terminate-after`: under \
         `cargo test` a test blocked on a FIFO hung a scoped run for ten minutes.\n{config}",
    );
    let src = std::fs::read_to_string(gate()).expect("dev/gate is readable");
    let naming: Vec<&str> = src
        .lines()
        .filter(|l| !l.trim_start().starts_with('#') && l.contains("scoped"))
        .collect();
    assert!(
        naming.is_empty(),
        "the gate runs under its own two profiles, which inherit `[profile.default]`; a \
         scoped run's timeout must not reach the full suite, where a legitimate test runs \
         for minutes under load.\n{}",
        naming.join("\n"),
    );
}

/// The exact totals line `.claude/workflows/milestone-build.js` matches, rendered.
///
/// Not `contains("passed=…")`: the harness matches the line **whole** and anchors on the
/// `tests` label and the `(over N test binaries)` phrasing, so the label and the spacing
/// are part of the contract. `failed=0` and a non-zero binary count are what `gateProven`
/// accepts, so the fixture is a green run's shape.
#[test]
fn the_totals_line_is_the_one_the_build_harness_matches() {
    let log = summary_line(3, 0);
    let report = report_over("gate-report-harness-totals", &log);
    let line = "tests   passed=3 failed=0  (over 1 test binaries)";
    assert!(
        report.lines().any(|l| l == line),
        "`.claude/workflows/milestone-build.js` matches this line WHOLE — start of line to \
         end of line, nothing trailing — to accept an increment's gate as proven. Reword \
         the label or the parenthetical and every later increment burns three full \
         validator rounds before halting on `gate green could not be verified`.\n\
         expected: {line}\nreport:\n{report}",
    );
}

/// The two literals the build harness reads, pinned at their producers in `dev/gate`.
///
/// The rendered arm above proves `gate_report` prints the totals line; it cannot prove
/// the **live** tail prints the same one, and the live tail is the only output a
/// validator ever pastes. Nor can any `--report` run reach `GATE: PASS`, which that
/// path never prints. Both are literals in one shell script, so the script's own text
/// is where they are fenced.
///
/// Both assertions discriminate on **`printf` producers**, not on substring counts, so
/// the two edits that are improvements stay green: documenting either literal in the
/// leading comment block (which is the tool's `--help` text), and hoisting the duplicated
/// totals `printf` into one helper. What must still redden is a producer that renders a
/// *different* line — including the live tail alone, the copy the harness actually reads.
#[test]
fn dev_gate_produces_the_two_literals_the_build_harness_reads() {
    let src = std::fs::read_to_string(gate()).expect("dev/gate is readable");

    let totals_fmt = "printf 'tests   passed=%s failed=%s  (over %s test binaries)\\n'";
    let totals_producers = printf_producers(&src, "passed=");
    assert!(
        !totals_producers.is_empty(),
        "no `printf` in `dev/gate` renders a totals line at all. `gateProven` in \
         `.claude/workflows/milestone-build.js` accepts an increment's gate only when the \
         pasted evidence carries one, so with no producer every later increment burns three \
         full validator rounds before the harness halts on `gate green could not be \
         verified`.\nexpected:\n  {totals_fmt}",
    );
    let divergent: Vec<&str> = totals_producers
        .iter()
        .copied()
        .filter(|line| !line.contains(totals_fmt))
        .collect();
    assert!(
        divergent.is_empty(),
        "every `printf` that renders the totals line must render this EXACT format — it is \
         a contract with `.claude/workflows/milestone-build.js`, which matches the line \
         whole and reads only the LIVE tail's copy, so a producer that drifts leaves the \
         consumed copy unfenced while this suite stays green.\nexpected:\n  {totals_fmt}\n\
         divergent producer(s):\n  {}",
        divergent.join("\n  "),
    );

    let verdict = "printf '\\nGATE: PASS\\n'";
    let verdict_producers = printf_producers(&src, "GATE:");
    let pass_producers: Vec<&str> = verdict_producers
        .iter()
        .copied()
        .filter(|line| line.contains("GATE: PASS"))
        .collect();
    assert_eq!(
        pass_producers.len(),
        1,
        "exactly one `printf` in `dev/gate` may emit the `GATE: PASS` verdict — the tail \
         reached only when every step exited 0. Zero means the verdict the build harness \
         trusts has no producer (renamed? removed?) and every later increment halts \
         unverifiable; two would let a run that measured nothing emit it.\n\
         verdict-printing statements found:\n  {}",
        if verdict_producers.is_empty() {
            "(none)".to_string()
        } else {
            verdict_producers.join("\n  ")
        },
    );
    assert!(
        pass_producers[0].contains(verdict),
        "`.claude/workflows/milestone-build.js` accepts an increment's gate only when the \
         pasted evidence carries a BARE `GATE: PASS` line — matched whole, so the leading \
         newline and the absence of anything trailing are part of the contract.\n\
         expected:\n  {verdict}\nfound:\n  {}",
        pass_producers[0],
    );
}

// ---------------------------------------------------------------------------
// The deps-directory advisory: what the run is about to build *into*.
// ---------------------------------------------------------------------------
//
// `target/debug/deps` accumulates and is never garbage-collected. On 2026-09-14,
// mid-M51, it held **1,825,219** entries (26 GB), 1,823,960 of them `*.rcgu.o`
// split-debuginfo sidecars with a median age of 15 days: every gate run re-links the
// `cli` test binaries under a fresh hash and adds thousands more, and nothing removes
// the old ones. The cost is not disk — it is that macOS `syspolicyd` (Gatekeeper)
// rescans the tree, and a full `dev/gate` measured **~20 min** against **~9.5 min**
// on the same tree with the stale sidecars deleted mid-run (clippy 148→13 s, build
// 200→9 s, test 958→~540 s).
//
// `[profile.dev] split-debuginfo = "packed"` stops the litter being produced, but
// only for objects linked after that change; a tree that already carries it, or a
// profile someone flips back, is invisible. So the gate *counts* the directory it is
// about to build into and says so — one line always, a WARNING above a threshold —
// and the three arms below pin the two things that could make that advisory harmful:
// it must never become a gate step (a slow `find` or an absent directory would then
// fail a green gate), and it must survive a directory that does not exist at all,
// which is the state immediately after `cargo clean` and in every fresh clone.
//
// Driven, not asserted structurally, and cheap for the same reason the 127 arm above
// is: `PATH=/usr/bin:/bin` removes `cargo`, so every step exits 127 in ~0s and nothing
// is built, while the header — which the advisory is part of — is printed in full.

/// Run `dev/gate --quick` with `CARGO_TARGET_DIR` pointed at `target_dir` and cargo
/// absent from `PATH`, returning its stdout.
fn quick_gate_over(target_dir: &Path, warn_at: Option<&str>) -> String {
    let tmp = ScratchDir::new("gate-quick-tmp");
    let mut cmd = gate_in(&tmp);
    cmd.arg("--quick")
        .env("PATH", "/usr/bin:/bin")
        .env("CARGO_TARGET_DIR", target_dir);
    if let Some(n) = warn_at {
        cmd.env("JIGC_GATE_DEPS_WARN", n);
    }
    let out = cmd.output().expect("spawn dev/gate --quick");
    String::from_utf8_lossy(&out.stdout).to_string()
}

/// A `<root>/debug/deps` holding `n` files, and the root to hand `CARGO_TARGET_DIR`.
fn target_dir_with_deps(label: &str, n: usize) -> ScratchDir {
    let root = ScratchDir::new(label);
    let deps = root.path().join("debug/deps");
    std::fs::create_dir_all(&deps).expect("create the fixture deps dir");
    for i in 0..n {
        std::fs::write(deps.join(format!("litter-{i}.rcgu.o")), b"").expect("write a fixture file");
    }
    root
}

#[test]
fn the_gate_counts_the_deps_directory_it_is_about_to_build_into() {
    let root = target_dir_with_deps("gate-deps-count", 3);
    let text = quick_gate_over(root.path(), None);
    assert!(
        text.lines().any(|l| l == "gate: deps   3 entries"),
        "every run must say how much litter it is building on top of — the growth is \
         silent otherwise, which is exactly how the directory reached 1.8 M entries.\n\
         {text}",
    );
    assert!(
        !text.contains("WARNING"),
        "three entries is not a warning; the threshold exists so the line is noise-free \
         on a clean tree.\n{text}",
    );
    // The count is only readable against the directory it counted. `CARGO_TARGET_DIR`
    // in the environment redirects every cargo step, and the header said
    // `shared (<repo>/target)` regardless — a line that names the wrong tree beside a
    // number measured from the right one is worse than no line.
    assert!(
        text.lines()
            .any(|l| l == format!("gate: target {}", root.path().display())),
        "the header must name the target dir the run will actually use, including one \
         inherited from `CARGO_TARGET_DIR`.\n{text}",
    );
}

#[test]
fn a_deps_directory_over_the_threshold_warns_and_names_the_cleanup() {
    let root = target_dir_with_deps("gate-deps-warn", 3);
    let text = quick_gate_over(root.path(), Some("2"));
    assert!(
        text.contains("WARNING"),
        "above the threshold the count must be a warning, not a statistic.\n{text}",
    );
    assert!(
        text.contains("dev/clean-litter"),
        "a warning that does not name the command that fixes it is a complaint — \
         `dev/clean-litter` is the tool, and the route floor this repo applies to its \
         own product applies to its tooling.\n{text}",
    );
    // The advisory is an advisory: it is never a step, so it can never redden a gate
    // whose four commands all passed. With cargo absent all three quick steps exit 127,
    // and the verdict must name exactly those three.
    assert!(
        text.lines()
            .any(|l| l == "GATE: FAIL (step: fmt clippy build)"),
        "the deps advisory must not join the step list or the verdict — it measures the \
         tree, it does not judge the code.\n{text}",
    );
}

#[test]
fn an_absent_deps_directory_counts_zero_rather_than_erroring() {
    // The state immediately after `cargo clean`, and in every fresh clone.
    let root = ScratchDir::new("gate-deps-absent");
    let text = quick_gate_over(root.path(), None);
    assert!(
        text.lines().any(|l| l == "gate: deps   0 entries"),
        "a target dir with no `debug/deps` is the post-clean state, not an error.\n{text}",
    );
}

/// The git every suite drives, named in the header — and on macOS, never the trampoline.
///
/// `/usr/bin/git` on macOS is an `xcrun` shim that resolves the developer directory on
/// every call, ~11 ms each, and a full gate makes ~174 k git calls: the suite measured
/// 29 % faster with the real binary first on `PATH` (completions/artifacts/M55/
/// gate-speed-measurement.md). `PATH=/usr/bin:/bin` here is exactly the state that puts
/// the shim first, so on macOS the header must name the binary behind it — and export
/// `SDKROOT`, without which tree-sitter's C build under that `PATH` cannot find the SDK.
#[test]
fn the_gate_names_the_git_the_suite_runs_under() {
    let root = ScratchDir::new("gate-git-line");
    let text = quick_gate_over(root.path(), None);
    let git = text
        .lines()
        .find_map(|l| l.strip_prefix("gate: git    "))
        .unwrap_or_else(|| panic!("the header must name the git the suite runs under.\n{text}"));
    let trampoline = cfg!(target_os = "macos")
        && Command::new("xcrun")
            .args(["--find", "git"])
            .output()
            .is_ok_and(|o| o.status.success());
    if trampoline {
        assert!(
            !git.starts_with("/usr/bin/git"),
            "with `/usr/bin/git` first on PATH and xcrun able to resolve the real one, the \
             gate must run the suite under the real one.\n{text}",
        );
        assert!(
            text.lines().any(|l| l.starts_with("gate:        SDKROOT ")),
            "putting the real git first must export SDKROOT, and say so.\n{text}",
        );
    }
}

// ---------------------------------------------------------------------------
// The hygiene advisory: the local half of the public-hygiene guard.
// ---------------------------------------------------------------------------
//
// `dev/gate` scans the unpushed range and the tracked tree with a PRIVATE denylist
// (`implementation/public-hygiene.md`), through `dev/hygiene-scan` — the same tool CI
// runs as a hard step. Locally it is an advisory, exactly like the deps count, and the
// three arms below pin the properties that make an advisory safe to print on every run:
// a hit is reported by **where** it is and never by **what** matched (a scan that
// echoed the term would publish it in the next pasted gate log), a hit never joins the
// step list or the verdict, and a clean or absent list says so rather than printing
// nothing. The fixture denylist names a phrase from `dev/gate`'s own comments through a
// bracket class, so the pattern's own spelling in this file cannot match itself.
//
// `JIGC_GATE_HYGIENE_RANGE=HEAD^!` bounds the history half to one commit: the tree half
// is what these arms exercise, and a clone with no remote would otherwise scan its whole
// history on every arm.

/// Run `dev/gate --quick` with cargo absent, a fixture denylist, and a one-commit range.
fn quick_gate_with_denylist(label: &str, denylist: Option<&str>) -> String {
    let dir = ScratchDir::new(label);
    let list = dir.path().join("denylist");
    if let Some(body) = denylist {
        std::fs::write(&list, body).expect("write the fixture denylist");
    }
    let out = gate_in(&dir)
        .arg("--quick")
        .env("PATH", "/usr/bin:/bin")
        .env("CARGO_TARGET_DIR", dir.path())
        .env("JIGC_DENYLIST_FILE", &list)
        .env("JIGC_GATE_HYGIENE_RANGE", "HEAD^!")
        .output()
        .expect("spawn dev/gate --quick");
    String::from_utf8_lossy(&out.stdout).to_string()
}

#[test]
fn a_denylist_hit_is_named_by_location_and_never_by_content() {
    // `alarm wor[d]` matches the `Its alarm word is HYGIENE` comment in dev/gate.
    let text = quick_gate_with_denylist("gate-hygiene-hit", Some("# fixture\n\nalarm wor[d]\n"));
    assert!(
        text.contains("gate: HYGIENE  denylist hit(s)"),
        "a denylist term in the tracked tree must be reported.\n{text}",
    );
    assert!(
        text.lines()
            .any(|l| l.contains("tree") && l.contains("dev/gate:")),
        "the hit must be named by path and line — that is the whole actionable content.\n{text}",
    );
    assert!(
        !text.contains("alarm word"),
        "the matching text must never be printed: a gate log is pasted into commits and \
         reviews, and CI's copy of this scan runs in a public log.\n{text}",
    );
    assert!(
        text.lines()
            .any(|l| l == "GATE: FAIL (step: fmt clippy build)"),
        "the hygiene advisory must not join the step list or the verdict — with cargo \
         absent the verdict names exactly the three quick steps.\n{text}",
    );
}

#[test]
fn a_clean_denylist_says_clean() {
    // A bracket class keeps the pattern from matching its own spelling in this file.
    let text = quick_gate_with_denylist("gate-hygiene-clean", Some("zq[9]x7kw-no-such-term\n"));
    assert!(
        text.lines()
            .any(|l| l.starts_with("gate: hygiene  denylist clean (")),
        "a scan that found nothing must say it ran, or silence reads as skipped.\n{text}",
    );
    assert!(
        !text.lines().any(|l| l.starts_with("gate: HYGIENE")),
        "a clean scan raises no alarm.\n{text}",
    );
}

#[test]
fn an_absent_denylist_is_named_as_skipped() {
    let text = quick_gate_with_denylist("gate-hygiene-absent", None);
    assert!(
        text.lines()
            .any(|l| l.starts_with("gate: hygiene  no denylist at ")
                && l.ends_with("-- denylist scan skipped")),
        "without a denylist the scan did not run, and the header must say where the file \
         was looked for — the denylist is private, so every clone starts without one.\n{text}",
    );
}

#[test]
fn a_malformed_denylist_is_a_setup_error_never_a_clean_scan() {
    // `[` opens a bracket expression it never closes: no extended-regex matcher accepts
    // it. Before the fix every `grep -f` exited 2 with no output, the empty hits file
    // read as zero, and the gate printed `denylist clean` — one typo in the private list
    // switched the whole guard off.
    let text = quick_gate_with_denylist(
        "gate-hygiene-malformed",
        Some("zq[9]x7kw-no-such-term\nzq[unclosed\n"),
    );
    assert!(
        text.lines()
            .any(|l| l.starts_with("gate: HYGIENE  the denylist scan could not run (exit 2)")),
        "a denylist the scan cannot compile must read as a scan that did not run.\n{text}",
    );
    assert!(
        !text.contains("denylist clean"),
        "a scan that never ran must not say clean.\n{text}",
    );
}

// ---------------------------------------------------------------------------
// `dev/hygiene-scan` itself, over a throwaway history.
// ---------------------------------------------------------------------------
//
// The scan `cd`s to the repository its own file sits in, so each fixture repo carries a
// copy of the script at `dev/hygiene-scan` and runs that copy. Git runs with no ambient
// config (`GIT_CONFIG_NOSYSTEM`, a null global file, a fixture `HOME`), so a developer
// machine and a CI runner build the same history.

struct ScanRepo {
    dir: PathBuf,
    /// The fixture repo's root, removed on drop.
    _root: ScratchDir,
    /// Beside the repo, never in it: the denylist (so it is never part of the scanned
    /// history) and the scan's own `$TMPDIR` (its work dir stays there by design).
    aux: ScratchDir,
}

impl ScanRepo {
    fn new(label: &str) -> Self {
        let root = ScratchDir::new(label);
        let aux = ScratchDir::new(&format!("{label}-aux"));
        let dir = root.path().to_path_buf();
        std::fs::create_dir_all(dir.join("dev")).expect("create the fixture repo");
        placed_executable::copy(
            &repo_root().join("dev/hygiene-scan"),
            &dir.join("dev/hygiene-scan"),
        );
        let repo = ScanRepo {
            dir,
            _root: root,
            aux,
        };
        repo.git_ok(&["init", "-q", "-b", "main", "."]);
        repo.git_ok(&["config", "user.email", "fixture@example.invalid"]);
        repo.git_ok(&["config", "user.name", "fixture"]);
        repo
    }

    fn cmd(&self, program: &Path) -> Command {
        let mut c = Command::new(program);
        c.current_dir(&self.dir)
            .env("HOME", &self.dir)
            .env("TMPDIR", self.aux.path())
            .env("GIT_CONFIG_NOSYSTEM", "1")
            .env("GIT_CONFIG_GLOBAL", "/dev/null");
        c
    }

    fn git(&self, args: &[&str]) -> std::process::Output {
        self.cmd(Path::new("git"))
            .args(args)
            .output()
            .expect("spawn git")
    }

    fn git_ok(&self, args: &[&str]) -> String {
        let out = self.git(args);
        assert!(
            out.status.success(),
            "git {args:?} failed: {}",
            String::from_utf8_lossy(&out.stderr),
        );
        String::from_utf8_lossy(&out.stdout).trim().to_string()
    }

    fn write(&self, rel: &str, body: &str) {
        std::fs::write(self.dir.join(rel), body).expect("write a fixture file");
    }

    fn commit_all(&self, msg: &str) {
        self.git_ok(&["add", "-A"]);
        self.git_ok(&["commit", "-q", "-m", msg]);
    }

    /// Run the copied scan (`--tree` when `tree`) with a denylist body; return (exit
    /// code, stdout, stderr).
    /// The denylist sits outside the fixture repo, so it is never part of the scanned
    /// history.
    fn scan(&self, denylist: &str, tree: bool, args: &[&str]) -> (i32, String, String) {
        let list = self.aux.path().join("denylist");
        std::fs::write(&list, denylist).expect("write the fixture denylist");
        let mut scan = self.cmd(&self.dir.join("dev/hygiene-scan"));
        if tree {
            scan.arg("--tree");
        }
        let out = scan
            .arg(&list)
            .args(args)
            .output()
            .expect("spawn dev/hygiene-scan");
        (
            out.status.code().expect("the scan exits, not a signal"),
            String::from_utf8_lossy(&out.stdout).to_string(),
            String::from_utf8_lossy(&out.stderr).to_string(),
        )
    }
}

#[test]
fn hygiene_scan_rejects_a_malformed_pattern_by_line_and_never_prints_it() {
    let repo = ScanRepo::new("hygiene-scan-malformed");
    repo.write("f", "a zebrafixture line\n");
    repo.commit_all("base");

    // The well-formed half of this list hits: with the bad line it must still be exit 2,
    // because a scan that skipped one pattern is not a scan of the list.
    let list = "# fixture\n\nzebrafixture\n\nqx[unclosed\n";
    for tree in [false, true] {
        let (code, stdout, stderr) = repo.scan(list, tree, &["HEAD"]);
        let args = if tree { "--tree HEAD" } else { "HEAD" };
        assert_eq!(
            code, 2,
            "{args}: a pattern no matcher accepts is a setup error (exit 2).\n\
             stdout: {stdout}\nstderr: {stderr}",
        );
        assert!(
            !stdout.contains("clean"),
            "{args}: a scan that never ran must not say clean.\nstdout: {stdout}",
        );
        assert!(
            stderr.contains("line 5 is not a valid extended regex"),
            "{args}: the bad pattern is named by its line in the denylist, blank and \
             comment lines counted, so it can be found and fixed.\nstderr: {stderr}",
        );
        assert!(
            !stdout.contains("unclosed") && !stderr.contains("unclosed"),
            "{args}: the denylist is private and CI logs are public: no output may quote \
             a pattern, and a matcher's own diagnostic does (git grep's names it).\n\
             stdout: {stdout}\nstderr: {stderr}",
        );
    }

    let (code, stdout, _) = repo.scan("zebrafixture\n", false, &["HEAD"]);
    assert_eq!(
        code, 1,
        "the well-formed list alone hits.\nstdout: {stdout}"
    );
}

#[test]
fn hygiene_scan_reports_a_line_introduced_only_by_a_merge_resolution() {
    let repo = ScanRepo::new("hygiene-scan-merge");
    repo.write("f", "a\nb\nc\n");
    repo.commit_all("base");
    repo.git_ok(&["switch", "-q", "-c", "side"]);
    repo.write("f", "a\nside\nc\n");
    repo.commit_all("side");
    repo.git_ok(&["switch", "-q", "main"]);
    repo.write("f", "a\nmain\nc\n");
    repo.commit_all("main");
    let merge = repo.git(&["merge", "-q", "side"]);
    assert!(
        !merge.status.success(),
        "the fixture needs a conflict, so the resolution is the merge's own content",
    );
    // Neither parent carries the term: only the person resolving the conflict typed it.
    repo.write("f", "a\nresolved zebrafixture\nc\n");
    repo.commit_all("merge side");
    let merge_sha = repo.git_ok(&["rev-parse", "HEAD"]);

    let (code, stdout, stderr) = repo.scan("zebrafixture\n", false, &["HEAD"]);
    assert_eq!(
        code, 1,
        "a line typed while resolving a merge is new exposure like any added line; \
         `git log -p` shows no diff for a merge unless asked.\n\
         stdout: {stdout}\nstderr: {stderr}",
    );
    let want = format!("  added    {}  f:2", &merge_sha[..12]);
    assert!(
        stdout.lines().any(|l| l == want),
        "the hit is named at the merge commit, by path and line.\nwant: {want}\nstdout: {stdout}",
    );
    assert!(
        !stdout.contains("zebrafixture"),
        "a hit is reported by location only.\nstdout: {stdout}",
    );
}
