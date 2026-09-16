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
//! [`dev_rig_parity`](dev_rig_parity) precedent (which scans `dev/` for a removal shape
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
    let mut cmd = Command::new(gate());
    cmd.arg("--quick")
        .current_dir(repo_root())
        .env("PATH", "/usr/bin:/bin")
        .env("CARGO_TARGET_DIR", target_dir);
    if let Some(n) = warn_at {
        cmd.env("JIGC_GATE_DEPS_WARN", n);
    }
    let out = cmd.output().expect("spawn dev/gate --quick");
    String::from_utf8_lossy(&out.stdout).to_string()
}

/// A `<root>/debug/deps` holding `n` files, and the root to hand `CARGO_TARGET_DIR`.
fn target_dir_with_deps(label: &str, n: usize) -> PathBuf {
    let root = support::trial_corpus::unique_root(label);
    let deps = root.join("debug/deps");
    std::fs::create_dir_all(&deps).expect("create the fixture deps dir");
    for i in 0..n {
        std::fs::write(deps.join(format!("litter-{i}.rcgu.o")), b"").expect("write a fixture file");
    }
    root
}

#[test]
fn the_gate_counts_the_deps_directory_it_is_about_to_build_into() {
    let root = target_dir_with_deps("gate-deps-count", 3);
    let text = quick_gate_over(&root, None);
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
            .any(|l| l == format!("gate: target {}", root.display())),
        "the header must name the target dir the run will actually use, including one \
         inherited from `CARGO_TARGET_DIR`.\n{text}",
    );
}

#[test]
fn a_deps_directory_over_the_threshold_warns_and_names_the_cleanup() {
    let root = target_dir_with_deps("gate-deps-warn", 3);
    let text = quick_gate_over(&root, Some("2"));
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
    // whose five commands all passed. With cargo absent all three quick steps exit 127,
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
    let root = support::trial_corpus::unique_root("gate-deps-absent");
    std::fs::create_dir_all(&root).expect("create the fixture target dir");
    let text = quick_gate_over(&root, None);
    assert!(
        text.lines().any(|l| l == "gate: deps   0 entries"),
        "a target dir with no `debug/deps` is the post-clean state, not an error.\n{text}",
    );
}
