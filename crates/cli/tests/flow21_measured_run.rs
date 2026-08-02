//! M17 Increment 5 / T5 — flow 21, the measured run, driven end-to-end through
//! the REAL binary on a throwaway repo (`design/measurement.md` → Acceptance
//! flows, flow 21 + The seeded-failure obligation).
//!
//! The review-reordered seed sequence: the first promoting finalize lands the
//! committed managed docs FIRST (a fresh twin has no committed managed doc to
//! edit), then the seeds plant mid-run:
//!
//! 1. **First promoting finalize** — `jigc start --workflow record-dogfood`
//!    composes off-router; the emitted authoring lines are extracted from the
//!    composed bytes and run VERBATIM (placeholders agent-substituted — never a
//!    reconstructed equivalent); the landed finalize promotes the calibration
//!    record to `dogfood/`.
//! 2. **The seeded OOB edit** — a conformant sed-class edit on the committed
//!    record, made OUTSIDE the observed tools (no Write/Edit tool, no jigc —
//!    pinned to the absorb channel). It must yield `reconciliation.absorb`
//!    exactly once in the next LANDED finalize envelope (increments 1+2: the
//!    landed emission + the absorb baseline-advance), and ZERO on a subsequent
//!    validate (the re-fire defect stays fixed).
//! 3. **The seeded bad finalize** — a staged integrity violation on a later
//!    task; finalize exits **3** (increment 1's validation-blocked code).
//! 4. **The recording task** — the record blocks at finalize on a missing fact
//!    field and on an omitted owner-artifact (increment 3's #5 gate +
//!    `required-field-present`); fully authored + durably staged, ONE landed
//!    commit promotes the record to `dogfood/` and the artifact under
//!    `completions/artifacts/`.
//! 5. **The capture loop closes** — every jigc invocation this test makes is
//!    also fed through the REAL hook logger (`implementation/dogfood/
//!    log-event.py`) as the PostToolUse observation a measured run would
//!    produce (the command string + the REAL emitted streams + the REAL exit
//!    code), and the REAL tally (`tally.py`) derives the facts: the seeded
//!    path counted ONCE post-dedup (the corroborating absorb sightings across
//!    blocked + landed envelopes collapse to one (path × window) event) and
//!    the seeded block keyed via exit 3.
//!
//! Seeded attribution stays transcription protocol — the tally never opines;
//! this test attributes the seeded entries against its own staged sequence,
//! exactly as the run orchestrator would.

use std::fs;
use std::io::Write as _;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

/// A throwaway directory that removes itself on drop.
struct TempDir(PathBuf);

impl TempDir {
    fn new(tag: &str) -> Self {
        let mut path = std::env::temp_dir();
        path.push(format!(
            "jigc-flow21-{tag}-{}-{:?}",
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

/// The on-disk methodology pack home (`<root>/packs/methodology`).
fn methodology_pack_tree() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("..")
        .join("packs")
        .join("methodology")
}

/// The apparatus home: `implementation/dogfood/` at the repo root.
fn apparatus_dir() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../implementation/dogfood")
        .canonicalize()
        .expect("implementation/dogfood/ exists (the apparatus home)")
}

/// Run a `git` command in `repo`, asserting success, returning trimmed stdout.
fn git(repo: &Path, args: &[&str]) -> String {
    let out = Command::new("git")
        .args(args)
        .current_dir(repo)
        .output()
        .expect("run git");
    assert!(
        out.status.success(),
        "git {args:?} failed: {}",
        String::from_utf8_lossy(&out.stderr)
    );
    String::from_utf8(out.stdout).expect("utf-8 git stdout")
}

/// The repo's commit count (the one-landed-commit assertions).
fn rev_count(repo: &Path) -> u32 {
    git(repo, &["rev-list", "--count", "HEAD"])
        .trim()
        .parse()
        .expect("numeric rev count")
}

/// Both streams of an invocation, rendered for assertion messages.
fn streams(out: &std::process::Output) -> String {
    format!(
        "{}{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr),
    )
}

/// Assert a `jigc` invocation succeeded, surfacing its streams on failure.
fn assert_ok(out: &std::process::Output, what: &str) {
    assert!(
        out.status.success(),
        "{what} must succeed; got:\n{}",
        streams(out)
    );
}

/// The exit code of an outcome, with the streams surfaced on a missing code.
fn code_of(out: &std::process::Output, what: &str) -> i32 {
    out.status
        .code()
        .unwrap_or_else(|| panic!("{what} must exit with a code; got:\n{}", streams(out)))
}

/// Shell-quote one argv token for the logged command string (shlex-compatible:
/// the tally re-splits it; only whitespace-bearing titles/intents need quotes).
fn quote(token: &str) -> String {
    if token.is_empty() || token.chars().any(char::is_whitespace) {
        format!("\"{token}\"")
    } else {
        token.to_string()
    }
}

/// The measured-run twin: a throwaway git repo + `$HOME`, the hook log OUTSIDE
/// the repo (measurement apparatus, not project content), and the invocation
/// counter the tally's telemetry total is checked against.
struct Flow {
    repo: TempDir,
    home: TempDir,
    capture: TempDir,
    invocations: usize,
}

impl Flow {
    fn new() -> Self {
        let repo = TempDir::new("twin");
        git(repo.path(), &["init", "-q"]);
        git(repo.path(), &["config", "user.email", "test@example.com"]);
        git(repo.path(), &["config", "user.name", "Test"]);
        fs::write(repo.path().join("README.md"), "hello\n").expect("write file");
        git(repo.path(), &["add", "."]);
        git(repo.path(), &["commit", "-q", "-m", "initial"]);
        fs::create_dir_all(repo.path().join(".jigc").join("config")).expect("project layer");
        Flow {
            repo,
            home: TempDir::new("home"),
            capture: TempDir::new("capture"),
            invocations: 0,
        }
    }

    fn repo(&self) -> &Path {
        self.repo.path()
    }

    fn log(&self) -> PathBuf {
        self.capture.path().join("hook-log.jsonl")
    }

    /// Run `jigc <args>` against the twin AND feed the invocation through the
    /// REAL hook logger — the PostToolUse observation a measured run's harness
    /// would deliver: the command string, the REAL emitted streams, the REAL
    /// exit code. The capture rides the genuine bytes, never a synthetic
    /// re-rendering.
    fn jigc(&mut self, args: &[String], stdin: Option<&[u8]>) -> std::process::Output {
        let mut command = Command::new(env!("CARGO_BIN_EXE_jigc"));
        command
            .args(args)
            .current_dir(self.repo())
            .env("HOME", self.home.path())
            .env("JIGC_PACK_DIR", methodology_pack_tree())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped());
        if stdin.is_some() {
            command.stdin(Stdio::piped());
        }
        let mut child = command.spawn().expect("spawn jigc");
        if let Some(bytes) = stdin {
            child
                .stdin
                .take()
                .expect("stdin piped")
                .write_all(bytes)
                .expect("write stdin");
        }
        let out = child.wait_with_output().expect("wait for jigc");
        self.observe(args, &out);
        self.invocations += 1;
        out
    }

    fn jigc_strs(&mut self, args: &[&str], stdin: Option<&[u8]>) -> std::process::Output {
        let owned: Vec<String> = args.iter().map(|a| a.to_string()).collect();
        self.jigc(&owned, stdin)
    }

    /// Feed one observation through the REAL hook script (`log-event.py`).
    fn observe(&self, args: &[String], out: &std::process::Output) {
        let cmd = std::iter::once("jigc".to_string())
            .chain(args.iter().map(|a| quote(a)))
            .collect::<Vec<_>>()
            .join(" ");
        let payload = serde_json::json!({
            "hook_event_name": "PostToolUse",
            "tool_name": "Bash",
            "tool_input": { "command": cmd },
            "tool_response": {
                "stdout": String::from_utf8_lossy(&out.stdout),
                "stderr": String::from_utf8_lossy(&out.stderr),
                "exitCode": out.status.code(),
            },
        });
        let mut child = Command::new("python3")
            .arg(apparatus_dir().join("log-event.py"))
            .env("JIGC_DOGFOOD_LOG", self.log())
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .expect("spawn the hook script");
        child
            .stdin
            .take()
            .expect("hook stdin")
            .write_all(payload.to_string().as_bytes())
            .expect("write hook payload");
        let hook = child.wait_with_output().expect("hook exits");
        assert!(
            hook.status.success(),
            "the hook must exit 0 (never perturbs the run): {}",
            String::from_utf8_lossy(&hook.stderr)
        );
    }
}

/// Extract the one composed command line containing `needle` — the EMITTED
/// bytes, from the `jigc ` token on (trailing `Run:` backtick stripped).
fn extract_cmd(compose: &str, needle: &str) -> String {
    let mut hits = compose.lines().filter(|l| l.contains(needle));
    let line = hits
        .next()
        .unwrap_or_else(|| panic!("no composed line contains {needle:?}; got:\n{compose}"));
    assert!(
        hits.next().is_none(),
        "more than one composed line contains {needle:?}; got:\n{compose}"
    );
    let at = line
        .find("jigc ")
        .unwrap_or_else(|| panic!("composed line carries no jigc command: {line:?}"));
    line[at..].trim_end_matches('`').to_string()
}

/// Turn one emitted command line into argv: apply the authoring-placeholder
/// substitutions (`<slug>`/`<run>`/`<file>` — the runtime-minted values), then
/// fill any remaining `<…>` agent-value placeholder with `fill` as ONE argv
/// token (what shell quoting yields for a multi-word title).
fn argv_from(cmd: &str, subs: &[(&str, &str)], fill: Option<&str>) -> Vec<String> {
    cmd.split_whitespace()
        .skip(1) // the `jigc` token — the binary path supplies it
        .map(|token| {
            let mut t = token.to_string();
            for (from, to) in subs {
                t = t.replace(from, to);
            }
            if t.starts_with('<') {
                fill.unwrap_or_else(|| panic!("agent placeholder {t} needs a fill value"))
                    .to_string()
            } else {
                t
            }
        })
        .collect()
}

/// Extract the emitted line for `needle` and run it VERBATIM (placeholders
/// substituted) through the flow harness.
fn run_emitted(
    flow: &mut Flow,
    compose: &str,
    needle: &str,
    subs: &[(&str, &str)],
    fill: Option<&str>,
    stdin: Option<&[u8]>,
) -> std::process::Output {
    let argv = argv_from(&extract_cmd(compose, needle), subs, fill);
    flow.jigc(&argv, stdin)
}

/// The 13 directly-valued meta fields (run identity + 8 organic facts + the 2
/// seeded instrument checks + the judged verdict), protocol-plausible values.
/// `owner-artifact` is separate: its emitted line carries the owned-home path
/// (`completions/artifacts/<run>/<file>`) rather than an agent placeholder.
const META_FIELDS: [(&str, &str); 13] = [
    ("case", "pilot"),
    ("binary-sha", "0123abcd"),
    ("adapter-writes", "5"),
    ("oob-edits", "0"),
    ("drift-caught", "0"),
    ("validate-blocks", "0"),
    ("halts-expected", "1"),
    ("halts-unplanned", "0"),
    ("fix-rounds", "1"),
    ("audit-findings", "0"),
    ("seeded-oob", "1"),
    ("seeded-blocks", "1"),
    ("verdict", "green"),
];

/// The judgment prose — carries the phrase the sed-class seed later rewrites,
/// and states what the run did not test (the honesty obligation).
const JUDGMENT: &[u8] =
    b"The capture loop held firmly; the seeds registered once each. This run did not test ingest.\n";

/// Author the record at the run's minted `slug` by running the COMPOSED
/// authoring lines verbatim — every meta field, the judgment slot, and (unless
/// skipped) the durably-staged owner-artifact — EXCEPT any field named in
/// `skip`.
fn author_record(flow: &mut Flow, compose: &str, slug: &str, skip: &[&str]) {
    for (field, value) in META_FIELDS {
        if skip.contains(&field) {
            continue;
        }
        let needle = format!("set-field dogfood-record:<slug>#meta/{field}");
        let out = run_emitted(
            flow,
            compose,
            &needle,
            &[("<slug>", slug)],
            Some(value),
            None,
        );
        assert_ok(&out, &format!("emitted set-field {field} line"));
    }
    if !skip.contains(&"owner-artifact") {
        stage_artifact_and_set(flow, compose, slug);
    }
    let out = run_emitted(
        flow,
        compose,
        "set-slot dogfood-record:<slug>#judgment",
        &[("<slug>", slug)],
        None,
        Some(JUDGMENT),
    );
    assert_ok(&out, "emitted judgment set-slot line");
}

/// Write + git-stage the run's capture artifact at the owned home the emitted
/// line names (`completions/artifacts/<run>/<file>`), then run that line.
fn stage_artifact_and_set(flow: &mut Flow, compose: &str, slug: &str) {
    let artifact = format!("completions/artifacts/{slug}/capture.md");
    fs::create_dir_all(flow.repo().join("completions/artifacts").join(slug))
        .expect("mk owned artifact home");
    fs::write(
        flow.repo().join(&artifact),
        "transcript + raw hook log + tally output + manifest\n",
    )
    .expect("write owner-artifact");
    git(flow.repo(), &["add", &artifact]);
    let out = run_emitted(
        flow,
        compose,
        "set-field dogfood-record:<slug>#meta/owner-artifact",
        &[("<slug>", slug), ("<run>", slug), ("<file>", "capture.md")],
        None,
        None,
    );
    assert_ok(&out, "emitted owner-artifact set-field line");
}

/// Fill the task's commit doc by running the COMPOSED finalize-tail fill lines
/// verbatim.
fn fill_commit(flow: &mut Flow, compose: &str, task: &str) {
    let out = run_emitted(
        flow,
        compose,
        &format!("set-field commit:{task}#type"),
        &[],
        Some("chore"),
        None,
    );
    assert_ok(&out, "emitted commit type line");
    let out = run_emitted(
        flow,
        compose,
        &format!("set-field commit:{task}#scope"),
        &[],
        Some("dogfood"),
        None,
    );
    assert_ok(&out, "emitted commit scope line");
    let out = run_emitted(
        flow,
        compose,
        &format!("set-slot commit:{task}#summary"),
        &[],
        None,
        Some(b"record the measured run\n"),
    );
    assert_ok(&out, "emitted commit summary line");
    let out = run_emitted(
        flow,
        compose,
        &format!("set-slot commit:{task}#body"),
        &[],
        None,
        Some(b"One measured-run step of flow 21.\n"),
    );
    assert_ok(&out, "emitted commit body line");
}

/// `jigc start --workflow <workflow> "<intent>"`, returning the composed stdout
/// (the emitted bytes every authoring line is extracted from).
fn start(flow: &mut Flow, workflow: &str, intent: &str) -> String {
    let out = flow.jigc_strs(&["start", "--workflow", workflow, intent], None);
    assert_ok(&out, &format!("`jigc start --workflow {workflow}`"));
    String::from_utf8(out.stdout).expect("utf-8 compose stdout")
}

/// Run the REAL tally script over the flow's hook log, parsed.
fn run_tally(log: &Path, managed_prefixes: &[&str]) -> serde_json::Value {
    let mut cmd = Command::new("python3");
    cmd.arg(apparatus_dir().join("tally.py")).arg(log);
    for prefix in managed_prefixes {
        cmd.arg("--managed-prefix").arg(prefix);
    }
    let out = cmd.output().expect("spawn tally script");
    assert!(
        out.status.success(),
        "tally exits 0: {}",
        String::from_utf8_lossy(&out.stderr)
    );
    serde_json::from_slice(&out.stdout).expect("tally emits valid JSON")
}

/// The seeded OOB edit: rewrite the committed record's judgment prose through
/// REAL `sed` — the sed-class channel, OUTSIDE the observed tools (no Write/
/// Edit tool event, no jigc invocation; nothing reaches the hook log), so only
/// the absorb channel can register it. Conformant by construction (prose-only),
/// so the classifier routes to absorb, never to a conformance block.
fn seed_oob_edit(flow: &Flow, committed: &Path) {
    let before = fs::read_to_string(committed).expect("read the committed record");
    let sed = Command::new("sed")
        .arg("s/held firmly/held firmly throughout/")
        .arg(committed)
        .current_dir(flow.repo())
        .output()
        .expect("run sed");
    assert!(
        sed.status.success(),
        "sed must succeed: {}",
        String::from_utf8_lossy(&sed.stderr)
    );
    let edited = String::from_utf8(sed.stdout).expect("utf-8 sed output");
    assert_ne!(
        before, edited,
        "the sed-class seed must change the committed record"
    );
    fs::write(committed, edited).expect("apply the sed-class edit");
}

#[test]
fn flow21_measured_run_end_to_end() {
    let mut flow = Flow::new();

    // ── 1 · real work through the methodology: the first promoting finalize ──
    // Off-router by construction: record-dogfood is `selectable: false`, started
    // explicitly. Every authoring line below is the COMPOSED line run verbatim.
    let compose1 = start(&mut flow, "record-dogfood", "calibrate the capture loop");
    let task1 = "calibrate-the-capture-loop";
    let create = run_emitted(
        &mut flow,
        &compose1,
        "doc create dogfood-record",
        &[],
        Some("Calibration Run"),
        None,
    );
    assert_ok(&create, "emitted dogfood-record create line (calibration)");
    author_record(&mut flow, &compose1, "calibration-run", &[]);
    fill_commit(&mut flow, &compose1, task1);

    let before = rev_count(flow.repo());
    let fin1 = run_emitted(&mut flow, &compose1, "task finalize", &[], None, None);
    assert_ok(&fin1, "the first promoting finalize");
    assert_eq!(rev_count(flow.repo()), before + 1, "one landed commit");
    let committed = flow.repo().join("dogfood").join("calibration-run.md");
    assert!(
        !git(flow.repo(), &["ls-files", "dogfood/calibration-run.md"])
            .trim()
            .is_empty(),
        "the first promoting finalize commits dogfood/calibration-run.md",
    );
    assert!(
        !String::from_utf8_lossy(&fin1.stdout).contains("reconciliation.absorb"),
        "no absorb before any seed exists; got:\n{}",
        streams(&fin1),
    );

    // ── 2 · the seeds plant mid-run, AFTER the first promotion ───────────────
    // 2a — the seeded OOB edit on the now-committed managed doc.
    seed_oob_edit(&flow, &committed);

    // 2b — the seeded bad finalize: a staged integrity violation (the task's
    // commit doc deliberately left unauthored) on a later task. Exit 3 — the
    // validation-blocked code the drift bucket keys on; nothing lands.
    let compose2 = start(&mut flow, "dev-task", "seed a bad finalize");
    let before = rev_count(flow.repo());
    let fin2 = run_emitted(&mut flow, &compose2, "task finalize", &[], None, None);
    assert_eq!(
        code_of(&fin2, "the seeded bad finalize"),
        3,
        "the staged-violation finalize must exit 3; got:\n{}",
        streams(&fin2),
    );
    assert!(
        String::from_utf8_lossy(&fin2.stderr).contains("blocking · schema-conformance"),
        "the seeded block names the staged violation; got:\n{}",
        streams(&fin2),
    );
    assert_eq!(
        rev_count(flow.repo()),
        before,
        "a blocked finalize lands nothing"
    );

    // ── 3 · the recording task: blocks half-transcribed, promotes complete ───
    let compose3 = start(&mut flow, "record-dogfood", "record the pilot run");
    let task3 = "record-the-pilot-run";
    let create = run_emitted(
        &mut flow,
        &compose3,
        "doc create dogfood-record",
        &[],
        Some("Pilot Run"),
        None,
    );
    assert_ok(&create, "emitted dogfood-record create line (pilot)");
    author_record(
        &mut flow,
        &compose3,
        "pilot-run",
        &["oob-edits", "owner-artifact"],
    );
    fill_commit(&mut flow, &compose3, task3);

    // 3a — a missing fact field blocks: the record cannot land half-transcribed.
    let blocked = run_emitted(&mut flow, &compose3, "task finalize", &[], None, None);
    assert_eq!(
        code_of(&blocked, "finalize over a missing fact field"),
        3,
        "a missing fact field must exit 3; got:\n{}",
        streams(&blocked),
    );
    let stderr = String::from_utf8_lossy(&blocked.stderr).to_string();
    assert!(
        stderr.contains("field-value-conformant") && stderr.contains("oob-edits"),
        "the block names the un-transcribed `oob-edits` fact field (present-but-empty \
         since the M40 F1 create skeleton pre-stamps it, so it blocks at \
         field-value-conformant); got:\n{stderr}",
    );

    // 3b — fact transcribed; the omitted owner-artifact still blocks (#5 gate).
    let out = run_emitted(
        &mut flow,
        &compose3,
        "set-field dogfood-record:<slug>#meta/oob-edits",
        &[("<slug>", "pilot-run")],
        Some("0"),
        None,
    );
    assert_ok(&out, "transcribing the missing oob-edits fact");
    let blocked = run_emitted(&mut flow, &compose3, "task finalize", &[], None, None);
    assert_eq!(
        code_of(&blocked, "finalize over an omitted owner-artifact"),
        3,
        "an omitted owner-artifact must exit 3; got:\n{}",
        streams(&blocked),
    );
    let stderr = String::from_utf8_lossy(&blocked.stderr).to_string();
    assert!(
        stderr.contains("owner-artifact") && !stderr.contains("oob-edits"),
        "the block names the omitted `owner-artifact` and ONLY it (the fact fix took); \
         got:\n{stderr}",
    );

    // 3c — fully authored + durably staged → ONE landed commit promotes record
    // + artifact, and the landed envelope carries the seeded absorb EXACTLY
    // once (increment 1's success emission over increment 2's classifier).
    stage_artifact_and_set(&mut flow, &compose3, "pilot-run");

    // The designed preview first: `task validate` is a PURE READER — it sights
    // the seeded drift as the absorb advisory without advancing the baseline,
    // so the capture now holds corroborating sightings the tally must dedup.
    let preview = flow.jigc_strs(&["task", "validate", task3, "--format", "json"], None);
    assert_ok(&preview, "the pre-finalize preview validate");
    let envelope: serde_json::Value =
        serde_json::from_slice(&preview.stdout).expect("validate --format json envelope");
    assert_eq!(
        envelope["findings"]
            .as_array()
            .expect("findings array")
            .iter()
            .filter(|f| f["code"] == "reconciliation.absorb")
            .count(),
        1,
        "the preview validate sights the seeded drift once, without persisting; \
         got:\n{envelope:#}",
    );

    let before = rev_count(flow.repo());
    let fin3 = run_emitted(&mut flow, &compose3, "task finalize", &[], None, None);
    assert_ok(&fin3, "the promoting record finalize");
    assert_eq!(
        rev_count(flow.repo()),
        before + 1,
        "exactly one landed commit promotes record + artifact",
    );
    let stdout = String::from_utf8_lossy(&fin3.stdout).to_string();
    let absorbs: Vec<&str> = stdout
        .lines()
        .filter(|l| l.contains("reconciliation.absorb"))
        .collect();
    assert_eq!(
        absorbs.len(),
        1,
        "the landed envelope carries the seeded absorb EXACTLY once; got:\n{stdout}",
    );
    assert!(
        absorbs[0].contains("dogfood/calibration-run.md"),
        "the absorb names the seeded path; got:\n{stdout}",
    );
    for (path, what) in [
        ("dogfood/pilot-run.md", "the promoted record"),
        (
            "completions/artifacts/pilot-run/capture.md",
            "the promoted artifact",
        ),
    ] {
        assert!(
            !git(flow.repo(), &["ls-files", path]).trim().is_empty(),
            "{what} must be committed at {path}",
        );
    }
    let record = fs::read_to_string(flow.repo().join("dogfood/pilot-run.md"))
        .expect("read the promoted record");
    assert!(
        record.contains("seeded-oob: 1") && record.contains("seeded-blocks: 1"),
        "the instrument-check fields land durably transcribed; got:\n{record}",
    );

    // ── 4 · the baseline advanced: ZERO absorb on a subsequent validate ──────
    let compose4 = start(&mut flow, "dev-task", "verify the advanced baseline");
    let task4 = "verify-the-advanced-baseline";
    fill_commit(&mut flow, &compose4, task4);
    let validate = flow.jigc_strs(&["task", "validate", task4, "--format", "json"], None);
    assert_ok(&validate, "the post-absorb validate");
    let envelope: serde_json::Value =
        serde_json::from_slice(&validate.stdout).expect("validate --format json envelope");
    let re_fires = envelope["findings"]
        .as_array()
        .expect("findings array")
        .iter()
        .filter(|f| f["code"] == "reconciliation.absorb")
        .count();
    assert_eq!(
        re_fires, 0,
        "the absorbed baseline advanced at the landed finalize — a subsequent \
         validate re-fires NOTHING; got:\n{envelope:#}",
    );

    // ── 5 · the REAL tally over the REAL hook log closes the capture loop ────
    let raw_log = fs::read_to_string(flow.log()).expect("the hook log was written");
    let events: Vec<serde_json::Value> = raw_log
        .lines()
        .map(|l| serde_json::from_str(l).expect("each log line is one v1 JSON event"))
        .collect();
    assert!(
        events.iter().all(|e| e["v"] == 1 && e["event"] == "jigc"),
        "every event is a pinned-v1 jigc observation — the sed-class seed is \
         invisible to the observed tools (no file_op rides the log); got:\n{raw_log}",
    );

    // The corroborating absorb sightings: the blocked finalizes' stderr
    // envelopes AND the landed envelope all re-detect the same drift...
    let sightings = events
        .iter()
        .filter(|e| {
            e["findings"]
                .as_array()
                .is_some_and(|fs| fs.iter().any(|f| f["code"] == "reconciliation.absorb"))
        })
        .count();
    assert!(
        sightings >= 2,
        "the absorb is sighted across multiple envelopes (the dedup has work to \
         do); got {sightings} sighting(s) in:\n{raw_log}",
    );

    let tally = run_tally(&flow.log(), &["dogfood/", "completions/"]);
    let totals = &tally["totals"];

    // ...and the tally counts the seeded path ONCE post-dedup, on the absorb
    // channel ONLY (pinned outside the observed tools), in the window the seed
    // landed in (window 2 — after the first promoting finalize closed window 1).
    assert_eq!(totals["oob-edits"], 1, "tally: {tally:#}");
    let oob = tally["detail"]["oob-edits"].as_array().expect("oob detail");
    assert_eq!(oob.len(), 1);
    assert_eq!(oob[0]["path"], "dogfood/calibration-run.md");
    assert_eq!(oob[0]["window"], 2);
    assert_eq!(
        oob[0]["channels"],
        serde_json::json!(["absorb"]),
        "the seed is pinned to the absorb channel — no write-edit corroboration exists",
    );

    // The seeded block keys via exit 3: three finalize blocks total (the seeded
    // bad finalize + the two record blocks), each carrying its finding codes.
    assert_eq!(totals["drift-caught"], 3, "tally: {tally:#}");
    let drift = tally["detail"]["drift-caught"]
        .as_array()
        .expect("drift detail");
    let seeded: Vec<&serde_json::Value> = drift
        .iter()
        .filter(|d| {
            d["cmd"]
                .as_str()
                .is_some_and(|c| c.contains("seed-a-bad-finalize"))
        })
        .collect();
    assert_eq!(
        seeded.len(),
        1,
        "the seeded block keys exactly once: {tally:#}"
    );
    assert_eq!(seeded[0]["window"], 2);
    assert!(
        seeded[0]["findings"]
            .as_array()
            .is_some_and(|fs| fs.iter().any(|c| c
                .as_str()
                .is_some_and(|s| s.starts_with("schema-conformance.")))),
        "the seeded block carries its what-it-caught finding codes: {tally:#}",
    );

    // No validate exited 3 in this run — the paired count stays 0.
    assert_eq!(totals["validate-blocks"], 0, "tally: {tally:#}");

    // Logical-mutation grouping: 5 (doc address × window) mutations — two per
    // record window (the record + its commit doc), one in the validate window —
    // assembled from far more raw write-verb invocations (telemetry, never the
    // headline). Window 2's pilot record groups create + 13 set-field + the
    // post-block fact fix + owner-artifact + judgment = 16 raw invocations.
    assert_eq!(totals["adapter-writes"], 5, "tally: {tally:#}");
    let mutations = tally["detail"]["logical-mutations"]
        .as_array()
        .expect("mutation detail");
    let pilot = mutations
        .iter()
        .find(|m| m["address"] == "dogfood-record:pilot-run")
        .expect("the pilot record mutation appears in the detail");
    assert_eq!(pilot["window"], 2);
    assert_eq!(pilot["invocations"], 16, "tally: {tally:#}");
    assert_eq!(totals["write-verb-invocations"], 44, "tally: {tally:#}");

    // Every jigc invocation this test made was observed exactly once.
    assert_eq!(
        totals["jigc-invocations"], flow.invocations,
        "the hook logged every invocation, no drops, no dupes: {tally:#}",
    );
}
