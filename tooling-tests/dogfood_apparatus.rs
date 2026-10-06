//! M17 Increment 5 / T4 — the measurement apparatus at `implementation/dogfood/`
//! proven by running the REAL scripts (`design/measurement.md` → The capture
//! substrate, item 1): the hook logger over synthetic PostToolUse JSON and the
//! tally over the produced log + a committed v1 fixture.
//!
//! The apparatus is harness-side, zero engine surface: a PostToolUse hook script
//! appends one pinned-v1 JSONL event per observation (Bash → the jigc invocation's
//! argv + exit code + finding codes from output; Write|Edit → the path), and the
//! tally script derives the mechanized facts from the log:
//!
//! - **logical-mutation grouping** — one adapter-write per (doc address × finalize
//!   window); raw write-verb invocations are telemetry, never the headline,
//! - **OOB dedup** — one event per (path × window); the Write|Edit channel and the
//!   `reconciliation.absorb` finding channel corroborate, never sum,
//! - **exit-3 keying** — a `finalize` exit 3 keys the drift bucket, a `validate`
//!   exit 3 keys the paired validate-blocks count,
//! - **per-event detail** — emitted so seeded attribution stays transcription
//!   protocol, never tally-inferred.
//!
//! Every assertion runs the REAL scripts as a subprocess — never a reimplementation
//! of their rules in test code (the masking-test discipline: the scripts are the
//! artifact a dogfood run executes). The committed fixture pins the v1 log schema:
//! a tally that stops reading v1 lines goes red here.
//!
//! **Declared dependency: `python3` on `PATH`.** The apparatus under test is
//! Python (`log-event.py`, `tally.py`, `jrun`), so every test here spawns the
//! interpreter [`PYTHON`] names — the one place a reader or a CI image author
//! finds it. A host without it fails these tests at spawn (`NotFound`);
//! `dev/runner-faithful` installs it for exactly that reason. Whether a missing
//! interpreter should instead skip with a reason, or the apparatus be rewritten
//! in something the toolchain guarantees, is still owed to M57
//! (`implementation/decisions-pending.md` → row (I), the `dogfood_apparatus` one).

use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

use super::placed_executable;

/// The interpreter every apparatus script runs under — the suite's one declared
/// external dependency (see the module doc).
const PYTHON: &str = "python3";

/// A throwaway directory that removes itself on drop.
struct TempDir(PathBuf);

impl TempDir {
    fn new(tag: &str) -> Self {
        let mut path = std::env::temp_dir();
        path.push(format!(
            "jigc-dogfood-apparatus-{tag}-{}-{:?}",
            std::process::id(),
            engine::tempname::unique_nanos(),
        ));
        fs::create_dir_all(&path).expect("create temp dir");
        TempDir(path)
    }

    fn path(&self) -> &Path {
        &self.0
    }
}

impl Drop for TempDir {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}

/// The apparatus home: `implementation/dogfood/` at the repo root — the versioned
/// measurement artifact, consciously placed (`design/measurement.md` → the home note).
fn apparatus_dir() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../implementation/dogfood")
        .canonicalize()
        .unwrap_or_else(|e| {
            panic!("implementation/dogfood/ does not exist (the apparatus home): {e}")
        })
}

/// Run the REAL hook script with one synthetic PostToolUse JSON payload on stdin,
/// the log path env-configured (outside any repo — `$JIGC_DOGFOOD_LOG`). The hook
/// must always exit 0: a measurement hook never perturbs the run it observes.
fn run_hook(log: &Path, payload: &str) {
    run_hook_with(log, payload, None);
}

/// `run_hook` with `CLAUDE_PROJECT_DIR` set — the env Claude Code provides to every
/// hook command in a real run (the repo root the hook relativizes file_ops against).
fn run_hook_in_repo(log: &Path, payload: &str, repo_root: &Path) {
    run_hook_with(log, payload, Some(repo_root));
}

fn run_hook_with(log: &Path, payload: &str, project_dir: Option<&Path>) {
    let script = apparatus_dir().join("log-event.py");
    assert!(
        script.is_file(),
        "hook script missing: {}",
        script.display()
    );
    let mut cmd = Command::new(PYTHON);
    cmd.arg(&script).env("JIGC_DOGFOOD_LOG", log);
    match project_dir {
        Some(root) => {
            cmd.env("CLAUDE_PROJECT_DIR", root);
        }
        None => {
            cmd.env_remove("CLAUDE_PROJECT_DIR");
        }
    }
    let mut child = cmd
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("spawn hook script");

    crate::support::child_stdin::feed(&mut child, payload.as_bytes());
    let out = child.wait_with_output().expect("hook exits");
    assert!(
        out.status.success(),
        "hook must exit 0 (never blocks the run): {}\n{}",
        out.status,
        String::from_utf8_lossy(&out.stderr)
    );
}

/// Run the REAL tally script over `log`, returning its parsed JSON output.
fn run_tally(log: &Path, managed_prefixes: &[&str]) -> serde_json::Value {
    let raw = run_tally_raw(log, managed_prefixes);
    serde_json::from_str(&raw).expect("tally emits valid JSON")
}

fn run_tally_raw(log: &Path, managed_prefixes: &[&str]) -> String {
    let out = run_tally_unchecked(log, managed_prefixes);
    assert!(
        out.status.success(),
        "tally exits 0: {}\n{}",
        out.status,
        String::from_utf8_lossy(&out.stderr)
    );
    String::from_utf8(out.stdout).expect("tally output is UTF-8")
}

/// Run the REAL tally script and hand back the raw process output — for the
/// paths where the tally is REQUIRED to refuse (non-zero exit), not report.
fn run_tally_unchecked(log: &Path, managed_prefixes: &[&str]) -> std::process::Output {
    let script = apparatus_dir().join("tally.py");
    assert!(
        script.is_file(),
        "tally script missing: {}",
        script.display()
    );
    let mut cmd = Command::new(PYTHON);
    cmd.arg(&script).arg(log);
    for prefix in managed_prefixes {
        cmd.arg("--managed-prefix").arg(prefix);
    }
    cmd.output().expect("spawn tally script")
}

/// A synthetic PostToolUse payload for a Bash invocation, carrying the command,
/// its exit code, and its captured stdout — the Claude Code hook-input shape.
fn bash_event(command: &str, exit: i64, stdout: &str) -> String {
    serde_json::json!({
        "hook_event_name": "PostToolUse",
        "tool_name": "Bash",
        "tool_input": { "command": command },
        "tool_response": { "stdout": stdout, "stderr": "", "exitCode": exit },
    })
    .to_string()
}

/// A synthetic PostToolUse payload for a Write/Edit tool operation.
fn file_event(tool: &str, path: &str) -> String {
    serde_json::json!({
        "hook_event_name": "PostToolUse",
        "tool_name": tool,
        "tool_input": { "file_path": path },
        "tool_response": {},
    })
    .to_string()
}

/// The real-binary JSON findings envelope (the `render::json` projection) carrying
/// one finding — what a `--format json` jigc invocation prints, fed to the hook as
/// captured stdout so finding extraction runs over the genuine envelope shape.
fn envelope(severity: &str, code: &str, message: &str, address: &str) -> String {
    serde_json::json!({
        "findings": [{
            "severity": severity,
            "probe": code.split('.').next().unwrap(),
            "check": code.split_once('.').map(|(_, c)| c).unwrap_or(""),
            "code": code,
            "message": message,
            "location": { "address": address, "line": 1, "col": 1 },
            "route": null,
        }],
    })
    .to_string()
}

/// Drive the REAL hook script through a synthetic measured-run slice and the REAL
/// tally over the log it produced — the four done-criterion contracts in one walk:
/// logical-mutation grouping, OOB dedup across corroborating channels, exit-3
/// keying split finalize vs validate, and the telemetry invocation count.
#[test]
fn hook_log_and_tally_derive_the_mechanized_facts() {
    let tmp = TempDir::new("e2e");
    let log = tmp.path().join("capture/hook-log.jsonl");

    // -- Window 1: one logical mutation assembled from THREE set-field calls -----
    let addr = "dogfood-record:pilot-run";
    for (field, value) in [
        ("case", "pilot"),
        ("verdict", "green"),
        ("binary-sha", "abc123"),
    ] {
        run_hook(
            &log,
            &bash_event(
                &format!("jigc doc set-field {addr}#{field} --value {value} --task t1"),
                0,
                "field set\n",
            ),
        );
    }

    // A second doc address in the same window: a create — its own logical mutation.
    run_hook(
        &log,
        &bash_event(
            "jigc doc create adr \"Pick Storage\" --task t1",
            0,
            "created\n",
        ),
    );

    // A non-jigc Bash command must NOT be logged (the log records jigc invocations).
    run_hook(&log, &bash_event("cargo build", 0, ""));

    // The OOB pair on ONE path in ONE window — two corroborating channels:
    // (a) a hook-observed Write on a managed path,
    run_hook(
        &log,
        &file_event("Write", "docs/decisions/0001-pick-storage.md"),
    );
    // (b) the absorb finding on the SAME path, riding the landed finalize's
    //     emitted envelope (the inc-1 success emission). The landed finalize also
    //     closes window 1.
    run_hook(
        &log,
        &bash_event(
            "jigc task finalize t1 --format json",
            0,
            &envelope(
                "advisory",
                "reconciliation.absorb",
                "external edit absorbed: `docs/decisions/0001-pick-storage.md`",
                "docs/decisions/0001-pick-storage.md",
            ),
        ),
    );

    // -- Window 2: the exit-3 split ----------------------------------------------
    // A `validate` exit 3 keys validate-blocks — finding code extracted from the
    // AGENT-TEXT rendering (the second extraction channel).
    run_hook(
        &log,
        &bash_event(
            "jigc task validate t2",
            3,
            "blocking · schema-conformance.required-field-present — required field `case` missing\n",
        ),
    );
    // A `finalize` exit 3 keys the drift bucket.
    run_hook(
        &log,
        &bash_event(
            "jigc task finalize t2 --format json",
            3,
            &envelope(
                "blocking",
                "schema-conformance.required-field-present",
                "required field `owner-artifact` missing",
                "dogfood/pilot-run.md",
            ),
        ),
    );

    // -- The pinned v1 log: append-only JSONL, one event per line ----------------
    let raw_log = fs::read_to_string(&log).expect("hook log written");
    let lines: Vec<&str> = raw_log.lines().collect();
    assert_eq!(
        lines.len(),
        8,
        "8 observed events — 7 jigc + 1 file_op (the non-jigc Bash command is not logged):\n{raw_log}"
    );
    for line in &lines {
        let event: serde_json::Value = serde_json::from_str(line).expect("each line is JSON");
        assert_eq!(
            event["v"], 1,
            "every event carries the pinned schema version"
        );
        assert!(
            event["event"] == "jigc" || event["event"] == "file_op",
            "pinned event kinds only: {line}"
        );
    }

    // -- The REAL tally over the produced log ------------------------------------
    let tally = run_tally(&log, &["docs/decisions/"]);
    let totals = &tally["totals"];

    // 3 set-field calls to one address in one window → 1 logical mutation; the
    // create is the second. Raw invocations ride as telemetry, never the headline.
    assert_eq!(totals["adapter-writes"], 2, "tally: {tally:#}");
    assert_eq!(
        totals["write-verb-invocations"], 4,
        "telemetry rides the output"
    );
    assert_eq!(
        totals["jigc-invocations"], 7,
        "every jigc invocation is telemetry"
    );

    // One OOB post-dedup: hook-observed Write + absorb finding, same path, same
    // window — the channels corroborate, never sum.
    assert_eq!(totals["oob-edits"], 1, "tally: {tally:#}");

    // finalize exit 3 → drift bucket; validate exit 3 → validate-blocks.
    assert_eq!(totals["drift-caught"], 1, "tally: {tally:#}");
    assert_eq!(totals["validate-blocks"], 1, "tally: {tally:#}");

    // -- Per-event grouped detail (seeded attribution stays protocol) ------------
    let mutations = tally["detail"]["logical-mutations"]
        .as_array()
        .expect("grouped mutation detail");
    let grouped = mutations
        .iter()
        .find(|m| m["address"] == addr)
        .expect("the set-field address appears in the detail");
    assert_eq!(grouped["window"], 1);
    assert_eq!(
        grouped["invocations"], 3,
        "3 raw calls grouped into 1 mutation"
    );

    let oob = tally["detail"]["oob-edits"].as_array().expect("oob detail");
    assert_eq!(oob.len(), 1);
    assert_eq!(oob[0]["path"], "docs/decisions/0001-pick-storage.md");
    assert_eq!(oob[0]["window"], 1);
    assert_eq!(
        oob[0]["channels"],
        serde_json::json!(["absorb", "write-edit"]),
        "both corroborating channels named, counted once"
    );

    let drift = tally["detail"]["drift-caught"]
        .as_array()
        .expect("drift detail");
    assert_eq!(drift.len(), 1);
    assert_eq!(drift[0]["window"], 2);
    assert_eq!(
        drift[0]["findings"],
        serde_json::json!(["schema-conformance.required-field-present"]),
        "the qualitative what-it-caught record rides the drift bucket"
    );

    let blocks = tally["detail"]["validate-blocks"]
        .as_array()
        .expect("validate-block detail");
    assert_eq!(blocks.len(), 1);
    assert_eq!(
        blocks[0]["findings"],
        serde_json::json!(["schema-conformance.required-field-present"]),
        "agent-text finding extraction feeds the paired count"
    );

    // The tally never opines on seeded-ness — attribution is transcription protocol.
    assert!(
        tally["note"]
            .as_str()
            .expect("the honesty note rides the output")
            .contains("transcription protocol"),
    );

    // Determinism: same log in → byte-identical tally out.
    assert_eq!(
        run_tally_raw(&log, &["docs/decisions/"]),
        run_tally_raw(&log, &["docs/decisions/"]),
        "the tally is deterministic over the same log"
    );
}

/// The shape a REAL run produces: Claude Code's Write|Edit tool supplies an
/// ABSOLUTE `file_path`, and the hook command runs with `CLAUDE_PROJECT_DIR` (the
/// repo root) in its environment. The hook must record the file_op repo-relative
/// so the tally classifies it as managed AND the (path × window) dedup key is
/// byte-identical with the absorb channel's repo-relative finding path — the two
/// OOB channels corroborate, never sum (`design/measurement.md` →
/// adapter-adherence).
#[test]
fn absolute_path_write_corroborates_with_absorb_not_sums() {
    let tmp = TempDir::new("abs");
    let log = tmp.path().join("hook-log.jsonl");
    let repo = tmp.path().join("twin");
    fs::create_dir_all(repo.join("decisions")).expect("twin repo dir");

    // (a) The hook-observed Write — ABSOLUTE path under the repo root.
    let abs_path = repo.join("docs/decisions/0002-pick-storage.md");
    run_hook_in_repo(
        &log,
        &file_event("Write", abs_path.to_str().expect("utf-8 path")),
        &repo,
    );

    // (b) The absorb finding on the SAME doc rides the landed finalize
    //     repo-RELATIVE — the path shape jigc findings always carry.
    run_hook_in_repo(
        &log,
        &bash_event(
            "jigc task finalize t1 --format json",
            0,
            &envelope(
                "advisory",
                "reconciliation.absorb",
                "external edit absorbed: `docs/decisions/0002-pick-storage.md`",
                "docs/decisions/0002-pick-storage.md",
            ),
        ),
        &repo,
    );

    let tally = run_tally(&log, &["docs/decisions/"]);

    // ONE OOB event post-dedup — not zero (the write-edit channel fired for the
    // absolute path) and not two (the channels did not sum).
    assert_eq!(tally["totals"]["oob-edits"], 1, "tally: {tally:#}");
    let oob = tally["detail"]["oob-edits"].as_array().expect("oob detail");
    assert_eq!(oob.len(), 1, "one entry, not one per channel: {tally:#}");
    assert_eq!(
        oob[0]["path"], "docs/decisions/0002-pick-storage.md",
        "the dedup key is the repo-relative path"
    );
    assert_eq!(
        oob[0]["channels"],
        serde_json::json!(["absorb", "write-edit"]),
        "both channels corroborate on the byte-identical key: {tally:#}"
    );

    // A path OUTSIDE the repo root stays verbatim and unmanaged — never counted.
    run_hook_in_repo(&log, &file_event("Write", "/etc/elsewhere/notes.md"), &repo);
    let tally = run_tally(&log, &["docs/decisions/"]);
    assert_eq!(
        tally["totals"]["oob-edits"], 1,
        "an out-of-repo absolute path is not a managed-doc edit: {tally:#}"
    );
}

/// The committed fixture pins the v1 JSONL schema: the REAL tally over a
/// hand-written v1 log must keep deriving the same facts — window boundaries
/// (the same doc address in two finalize windows = two logical mutations), the
/// built-in `.jigc/tasks/<id>/docs/` working-area rule, the unmanaged-path
/// exclusion, and finalize-exit-3 keying. A schema drift in either script that
/// stops reading committed v1 capture goes red here.
#[test]
fn tally_reads_the_committed_v1_fixture() {
    let fixture = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../tooling-tests/fixtures/dogfood-hooklog-v1.jsonl");
    assert!(
        fixture.is_file(),
        "committed fixture: {}",
        fixture.display()
    );

    let tally = run_tally(&fixture, &[]);
    let totals = &tally["totals"];

    // The same address mutated in window 1 and window 2 → TWO logical mutations
    // (the per-window grouping, not a per-run collapse).
    assert_eq!(totals["adapter-writes"], 2, "tally: {tally:#}");

    // One OOB: the `.jigc/tasks/t2/docs/` working-area edit (built-in managed
    // rule); the `src/main.rs` Write is unmanaged and never counted.
    assert_eq!(totals["oob-edits"], 1, "tally: {tally:#}");
    let oob = tally["detail"]["oob-edits"].as_array().expect("oob detail");
    assert_eq!(oob[0]["path"], ".jigc/tasks/t2/docs/draft.md");
    assert_eq!(oob[0]["window"], 2);

    // The fixture's `task finalize t9` exit 3 keys the drift bucket; no validate
    // blocks anywhere in the fixture.
    assert_eq!(totals["drift-caught"], 1);
    assert_eq!(totals["validate-blocks"], 0);
    assert_eq!(totals["jigc-invocations"], 4);

    // The detail names each window so a transcriber can attribute seeded events.
    let mutations = tally["detail"]["logical-mutations"]
        .as_array()
        .expect("mutation detail");
    let windows: Vec<&serde_json::Value> = mutations
        .iter()
        .filter(|m| m["address"] == "roadmap:roadmap")
        .collect();
    assert_eq!(windows.len(), 2, "one entry per (address × window)");
}

/// A harness payload that carries NO exit code — the historically documented
/// Claude Code Bash `tool_response` shape (`{stdout, stderr, interrupted,
/// isImage}`). The hook must log `exit: null` (never invent a value), and the
/// tally must then REFUSE LOUDLY: with exit null, every exit-keyed fact
/// (adapter-writes, drift-caught, validate-blocks, the finalize-window advance)
/// silently collapses to zero while telemetry still accrues — a plausible-looking
/// report that measured nothing. Refusal, not zeros.
#[test]
fn tally_refuses_a_log_whose_jigc_exits_are_null() {
    let tmp = TempDir::new("null-exit");
    let log = tmp.path().join("hook-log.jsonl");

    // The real hook over the no-exit-code payload shape: logs exit: null.
    let payload = serde_json::json!({
        "hook_event_name": "PostToolUse",
        "tool_name": "Bash",
        "tool_input": { "command": "jigc task finalize t1" },
        "tool_response": {
            "stdout": "ok\n",
            "stderr": "",
            "interrupted": false,
            "isImage": false,
        },
    })
    .to_string();
    run_hook(&log, &payload);

    let raw = fs::read_to_string(&log).expect("hook log written");
    let event: serde_json::Value =
        serde_json::from_str(raw.lines().next().expect("one logged event"))
            .expect("logged event is JSON");
    assert!(
        event["exit"].is_null(),
        "no exit code in the payload → exit: null, never an invented value: {event}"
    );

    // A telemetry-only file_op alongside — null exit is fine for non-jigc events.
    run_hook(&log, &file_event("Write", "docs/decisions/0001-x.md"));

    let out = run_tally_unchecked(&log, &["docs/decisions/"]);
    assert!(
        !out.status.success(),
        "the tally must refuse a log whose jigc exits are null, not emit zeros:\n{}",
        String::from_utf8_lossy(&out.stdout)
    );
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert!(
        stderr.contains("1 jigc invocation"),
        "the refusal names the null-exit count: {stderr}"
    );
    assert!(
        stderr.contains("smoke"),
        "the refusal points at the pre-pilot smoke check: {stderr}"
    );
    assert!(
        out.stdout.is_empty(),
        "no plausible-looking report rides a refusal: {}",
        String::from_utf8_lossy(&out.stdout)
    );
}

/// jrun mode — the documented fallback for the verified real-harness fact that
/// the Claude Code Bash `PostToolUse` payload carries NO exit code (so the Bash
/// hook logs `exit: null` and the tally refuses). `jrun` shadows `jigc`, runs the
/// real pinned binary, captures its REAL exit, and feeds `log-event.py` the
/// payload the harness omitted — so the canonical extraction produces the `jigc`
/// event with a real exit, and the tally KEYS (does not refuse) the log.
#[test]
fn jrun_captures_real_exit_into_the_log() {
    let tmp = TempDir::new("jrun");
    let log = tmp.path().join("hook-log.jsonl");

    // A stub standing in for the real pinned jigc: a `finalize` that blocks —
    // prints an agent-text finding line on stdout and exits 3.
    let stub = tmp.path().join("jigc.real");
    placed_executable::write(
        &stub,
        "#!/usr/bin/env bash\n\
         echo \"blocking · schema-conformance.required-field-present — required field case missing\"\n\
         exit 3\n",
    );

    let jrun = apparatus_dir().join("jrun");
    assert!(jrun.is_file(), "jrun ships: {}", jrun.display());
    let out = Command::new(PYTHON)
        .arg(&jrun)
        .args(["task", "finalize", "t1"])
        .env("JIGC_REAL_BIN", &stub)
        .env("JIGC_DOGFOOD_LOG", &log)
        .env("JIGC_DOGFOOD_HOME", apparatus_dir())
        .output()
        .expect("run jrun");

    // Passthrough: jrun exits with the real code and reproduces the binary's stdout.
    assert_eq!(
        out.status.code(),
        Some(3),
        "jrun forwards the real exit code"
    );
    assert!(
        String::from_utf8_lossy(&out.stdout).contains("required field case missing"),
        "jrun passes the binary's stdout through"
    );

    // The log carries ONE jigc event with the REAL exit + the extracted finding.
    let raw = fs::read_to_string(&log).expect("hook log written");
    let lines: Vec<&str> = raw.lines().collect();
    assert_eq!(lines.len(), 1, "one jigc event per jrun invocation:\n{raw}");
    let event: serde_json::Value = serde_json::from_str(lines[0]).expect("v1 JSON line");
    assert_eq!(event["event"], "jigc");
    assert_eq!(event["cmd"], "task finalize t1", "the reconstructed argv");
    assert_eq!(event["exit"], 3, "the REAL exit, not null");
    assert_eq!(
        event["findings"][0]["code"], "schema-conformance.required-field-present",
        "finding extraction runs over the captured stdout"
    );

    // The tally now KEYS the drift bucket (a finalize exit 3) — never refuses.
    let tally = run_tally(&log, &[]);
    assert_eq!(tally["totals"]["drift-caught"], 1, "tally: {tally:#}");

    // With JIGC_DOGFOOD_LOG unset, jrun is a transparent passthrough: it runs the
    // binary and logs nothing (any non-measured session).
    let off_log = tmp.path().join("unused.jsonl");
    let off = Command::new(PYTHON)
        .arg(&jrun)
        .args(["--version"])
        .env("JIGC_REAL_BIN", &stub)
        .env_remove("JIGC_DOGFOOD_LOG")
        .output()
        .expect("run jrun with logging off");
    assert_eq!(
        off.status.code(),
        Some(3),
        "passthrough exit still forwarded"
    );
    assert!(!off_log.exists(), "logging off writes nothing");

    // Misconfiguration is LOUD, never silent: log requested but the apparatus
    // home has no log-event.py → a stderr warning, the exit still forwarded, and
    // nothing logged (a blind measured run must be visible, not a quiet zero).
    let blind_log = tmp.path().join("blind.jsonl");
    let empty_home = tmp.path().join("empty-home");
    fs::create_dir_all(&empty_home).expect("empty home dir");
    let misconfig = Command::new(PYTHON)
        .arg(&jrun)
        .args(["task", "finalize", "t1"])
        .env("JIGC_REAL_BIN", &stub)
        .env("JIGC_DOGFOOD_LOG", &blind_log)
        .env("JIGC_DOGFOOD_HOME", &empty_home)
        .output()
        .expect("run jrun with bad home");
    assert_eq!(misconfig.status.code(), Some(3), "exit still forwarded");
    assert!(
        String::from_utf8_lossy(&misconfig.stderr).contains("JIGC_DOGFOOD_HOME"),
        "a missing log-event.py warns loudly: {}",
        String::from_utf8_lossy(&misconfig.stderr)
    );
    assert!(
        !blind_log.exists(),
        "no event logged when the hook is unreachable"
    );
}

/// The measurement must survive the agent piping jrun's output into `head`/`grep -q`
/// and closing the pipe early — a real shape observed in a measured run
/// (`jigc start ... | head -60`). jrun logs BEFORE the passthrough and guards the
/// write, so a BrokenPipe costs neither the event nor a clean exit.
#[test]
fn jrun_logs_before_passthrough_survives_a_closed_pipe() {
    let tmp = TempDir::new("jrun-pipe");
    let log = tmp.path().join("hook-log.jsonl");
    let stub = tmp.path().join("jigc.real");
    // A stub that prints a finding line then far more than a pipe buffer of
    // output (so `head -1` closing forces a BrokenPipe on jrun's write), exit 3.
    placed_executable::write(
        &stub,
        "#!/usr/bin/env bash\n\
         echo \"blocking · schema-conformance.required-field-present — required field case missing\"\n\
         seq 1 200000\n\
         exit 3\n",
    );

    let jrun = apparatus_dir().join("jrun");
    let cmd = format!("{PYTHON} {jrun:?} task finalize t1 | head -1");
    let out = Command::new("bash")
        .arg("-c")
        .arg(&cmd)
        .env("JIGC_REAL_BIN", &stub)
        .env("JIGC_DOGFOOD_LOG", &log)
        .env("JIGC_DOGFOOD_HOME", apparatus_dir())
        .output()
        .expect("run jrun piped to head");

    // The event was logged despite the pipe closing mid-passthrough.
    let raw = fs::read_to_string(&log).expect("log written despite closed pipe");
    let event: serde_json::Value =
        serde_json::from_str(raw.lines().next().expect("one event")).expect("v1 JSON");
    assert_eq!(event["exit"], 3, "real exit logged before passthrough");
    assert_eq!(
        event["findings"][0]["code"], "schema-conformance.required-field-present",
        "findings extracted from the full captured output, not the truncated view"
    );
    // The BrokenPipe was swallowed — no Python traceback leaked to the run.
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert!(
        !stderr.contains("BrokenPipeError") && !stderr.contains("Traceback"),
        "closed pipe handled cleanly: {stderr}"
    );
}

/// Real-session command shapes the first cut of the hook missed: a PATH-QUALIFIED
/// jigc token (`/usr/local/bin/jigc`, `./jigc`) must be recognized, a COMPOUND
/// command must log one event PER invocation (not just the first), and prose that
/// merely mentions jigc inside quotes (`echo "run jigc ..."`) must log nothing —
/// the hook tokenizes with quotes respected (a bounded tokenizer, not a shell
/// parser; the bounds are pinned in the README).
#[test]
fn hook_extracts_path_qualified_and_every_compound_invocation() {
    let tmp = TempDir::new("compound");
    let log = tmp.path().join("hook-log.jsonl");

    // (a) Path-qualified at command start — zero events under the old extraction.
    run_hook(
        &log,
        &bash_event(
            "/usr/local/bin/jigc task validate t1",
            3,
            "blocking · schema-conformance.required-field-present — required field `case` missing\n",
        ),
    );
    run_hook(
        &log,
        &bash_event(
            "./jigc doc create adr \"Pick Storage\" --task t1",
            0,
            "created\n",
        ),
    );

    // (b) Compound: BOTH invocations log — the set-field and the landed finalize.
    run_hook(
        &log,
        &bash_event(
            "jigc doc set-field adr:pick-storage#status --value accepted --task t1 \
             && jigc task finalize t1",
            0,
            "ok\n",
        ),
    );

    // (c) Quoted prose mentioning jigc is NOT an invocation — nothing logged.
    run_hook(
        &log,
        &bash_event(
            "echo \"run jigc task validate next\"",
            0,
            "run jigc task validate next\n",
        ),
    );

    let raw_log = fs::read_to_string(&log).expect("hook log written");
    let cmds: Vec<String> = raw_log
        .lines()
        .map(|line| {
            let event: serde_json::Value = serde_json::from_str(line).expect("v1 JSON line");
            assert_eq!(event["event"], "jigc", "only jigc events here: {line}");
            event["cmd"].as_str().expect("cmd is a string").to_string()
        })
        .collect();
    assert_eq!(
        cmds,
        vec![
            "task validate t1",
            "doc create adr \"Pick Storage\" --task t1",
            "doc set-field adr:pick-storage#status --value accepted --task t1",
            "task finalize t1",
        ],
        "4 invocations: two path-qualified, two from one compound, prose skipped:\n{raw_log}"
    );

    // The REAL tally over the produced log: the path-qualified create and the
    // compound's set-field hit the SAME address in window 1 — ONE logical
    // mutation; the path-qualified validate exit 3 keys validate-blocks; the
    // compound's landed finalize closes the window.
    let tally = run_tally(&log, &["docs/decisions/"]);
    assert_eq!(tally["totals"]["jigc-invocations"], 4, "tally: {tally:#}");
    assert_eq!(tally["totals"]["adapter-writes"], 1, "tally: {tally:#}");
    assert_eq!(
        tally["totals"]["write-verb-invocations"], 2,
        "tally: {tally:#}"
    );
    assert_eq!(tally["totals"]["validate-blocks"], 1, "tally: {tally:#}");
    assert_eq!(tally["totals"]["drift-caught"], 0, "tally: {tally:#}");
    let mutations = tally["detail"]["logical-mutations"]
        .as_array()
        .expect("mutation detail");
    assert_eq!(mutations.len(), 1, "tally: {tally:#}");
    assert_eq!(mutations[0]["address"], "adr:pick-storage");
    assert_eq!(mutations[0]["window"], 1);
    assert_eq!(
        mutations[0]["invocations"], 2,
        "create + compound set-field grouped into one mutation"
    );
}

/// A compound carrying TWO landed finalizes closes TWO windows — one advance per
/// landed finalize event, never a collapse into one and never more than two. A
/// write after the compound lands in window 3.
#[test]
fn compound_with_two_finalizes_closes_two_windows() {
    let tmp = TempDir::new("two-finalizes");
    let log = tmp.path().join("hook-log.jsonl");

    run_hook(
        &log,
        &bash_event("jigc task finalize t1 && jigc task finalize t2", 0, "ok\n"),
    );
    run_hook(
        &log,
        &bash_event(
            "jigc doc set-field roadmap:roadmap#status --value done --task t3",
            0,
            "ok\n",
        ),
    );

    let raw_log = fs::read_to_string(&log).expect("hook log written");
    assert_eq!(
        raw_log.lines().count(),
        3,
        "the compound logs one event per finalize:\n{raw_log}"
    );

    let tally = run_tally(&log, &[]);
    let mutations = tally["detail"]["logical-mutations"]
        .as_array()
        .expect("mutation detail");
    assert_eq!(mutations.len(), 1, "tally: {tally:#}");
    assert_eq!(
        mutations[0]["window"], 3,
        "two landed finalizes closed windows 1 and 2: {tally:#}"
    );

    // Determinism holds over the multi-event-per-command log.
    assert_eq!(run_tally_raw(&log, &[]), run_tally_raw(&log, &[]));
}

/// The shipped apparatus is complete and self-describing: the hooks config wires
/// BOTH matchers (Bash and Write|Edit) at the real scripts, and the README pins
/// the v1 schema + the env-configured log path.
#[test]
fn apparatus_ships_hooks_config_and_readme() {
    let dir = apparatus_dir();

    let hooks_raw = fs::read_to_string(dir.join("hooks.json")).expect("hooks.json ships");
    let hooks: serde_json::Value = serde_json::from_str(&hooks_raw).expect("hooks.json is JSON");
    let post = hooks["hooks"]["PostToolUse"]
        .as_array()
        .expect("PostToolUse hook entries");
    let matchers: Vec<&str> = post.iter().filter_map(|e| e["matcher"].as_str()).collect();
    assert!(
        matchers.contains(&"Bash"),
        "the Bash matcher ships: {matchers:?}"
    );
    assert!(
        matchers.contains(&"Write|Edit"),
        "the Write|Edit matcher ships: {matchers:?}"
    );
    assert!(
        hooks_raw.contains("log-event.py"),
        "both matchers route to the real hook script"
    );

    let readme = fs::read_to_string(dir.join("README.md")).expect("README ships");
    for needle in [
        "JIGC_DOGFOOD_LOG",
        "\"v\": 1",
        "finalize window",
        "tally.py",
    ] {
        assert!(readme.contains(needle), "README documents {needle:?}");
    }
}
