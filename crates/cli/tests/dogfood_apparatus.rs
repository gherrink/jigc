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

use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

/// A throwaway directory that removes itself on drop.
struct TempDir(PathBuf);

impl TempDir {
    fn new(tag: &str) -> Self {
        let mut path = std::env::temp_dir();
        path.push(format!(
            "jigc-dogfood-apparatus-{tag}-{}-{:?}",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos(),
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
    let script = apparatus_dir().join("log-event.py");
    assert!(
        script.is_file(),
        "hook script missing: {}",
        script.display()
    );
    let mut child = Command::new("python3")
        .arg(&script)
        .env("JIGC_DOGFOOD_LOG", log)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("spawn hook script");
    use std::io::Write as _;
    child
        .stdin
        .take()
        .expect("hook stdin")
        .write_all(payload.as_bytes())
        .expect("write payload");
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
    let script = apparatus_dir().join("tally.py");
    assert!(
        script.is_file(),
        "tally script missing: {}",
        script.display()
    );
    let mut cmd = Command::new("python3");
    cmd.arg(&script).arg(log);
    for prefix in managed_prefixes {
        cmd.arg("--managed-prefix").arg(prefix);
    }
    let out = cmd.output().expect("spawn tally script");
    assert!(
        out.status.success(),
        "tally exits 0: {}\n{}",
        out.status,
        String::from_utf8_lossy(&out.stderr)
    );
    String::from_utf8(out.stdout).expect("tally output is UTF-8")
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
    run_hook(&log, &file_event("Write", "decisions/0001-pick-storage.md"));
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
                "external edit absorbed: `decisions/0001-pick-storage.md`",
                "decisions/0001-pick-storage.md",
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
            "blocking · conformance.required-field-present — required field `case` missing\n",
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
                "conformance.required-field-present",
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
    let tally = run_tally(&log, &["decisions/"]);
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
    assert_eq!(oob[0]["path"], "decisions/0001-pick-storage.md");
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
        serde_json::json!(["conformance.required-field-present"]),
        "the qualitative what-it-caught record rides the drift bucket"
    );

    let blocks = tally["detail"]["validate-blocks"]
        .as_array()
        .expect("validate-block detail");
    assert_eq!(blocks.len(), 1);
    assert_eq!(
        blocks[0]["findings"],
        serde_json::json!(["conformance.required-field-present"]),
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
        run_tally_raw(&log, &["decisions/"]),
        run_tally_raw(&log, &["decisions/"]),
        "the tally is deterministic over the same log"
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
    let fixture =
        Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/dogfood-hooklog-v1.jsonl");
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
