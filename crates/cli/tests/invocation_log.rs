//! Acceptance — the M36 **opt-in invocation log** (`design/measurement.md` → The
//! in-repo invocation log; `roadmap.md` → M36 Increment 1, bullet 3).
//!
//! A `bool` cascade knob `invocation-log` (default **OFF**) turns on an append-only JSONL
//! log at `.jigc/logs/invocations.jsonl` — one record per `jigc` invocation, carrying the
//! key set `cli::invocation_log`'s `Record` declares and this list no longer re-spells (it
//! said five keys of eight until M51; the shape is fenced against the emitted bytes by that
//! module's own exhaustive-destructure unit test). The record is written by a `main()`
//! wrapper that straddles `Cli::try_parse()` (so a clap-rejected usage error is logged too)
//! and resolves the knob from the project cascade independent of argv. Knob OFF writes
//! nothing; a run outside a jigc repo writes nothing; `.jigc/logs/` is git-ignored while
//! `.jigc/version` (the T2 provenance stamp) stays committed.
//!
//! This drives the built `jigc` binary end-to-end with **no** `JIGC_PACK_DIR` (the real
//! embedded pack) in a self-cleaning `TempDir`.

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

/// A throwaway directory that removes itself on drop.
struct TempDir(PathBuf);

impl TempDir {
    fn new(tag: &str) -> Self {
        let mut path = std::env::temp_dir();
        path.push(format!(
            "jigc-invocation-log-{tag}-{}-{:?}",
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
    String::from_utf8(out.stdout)
        .expect("utf-8")
        .trim()
        .to_string()
}

/// Run `jigc <args>` with `cwd = repo` and `$HOME = home` against the real embedded pack.
fn jigc(repo: &Path, home: &Path, args: &[&str]) -> std::process::Output {
    Command::new(env!("CARGO_BIN_EXE_jigc"))
        .args(args)
        .current_dir(repo)
        .env("HOME", home)
        .output()
        .expect("run the jigc binary")
}

/// Make `root` a real git repo with identity, then run `jigc setup` over it.
fn setup_repo(repo: &Path, home: &Path) {
    git(repo, &["init", "-q"]);
    git(repo, &["config", "user.email", "test@example.com"]);
    git(repo, &["config", "user.name", "Test"]);
    fs::write(repo.join("README.md"), "hello\n").expect("write file");
    git(repo, &["add", "."]);
    git(repo, &["commit", "-q", "-m", "initial"]);

    let out = jigc(repo, home, &["setup"]);
    assert!(
        out.status.success(),
        "`jigc setup` must succeed; stderr:\n{}",
        String::from_utf8_lossy(&out.stderr),
    );
}

/// Turn the invocation-log knob ON in the project cascade.
fn enable_log(repo: &Path, home: &Path) {
    let out = jigc(repo, home, &["config", "set", "invocation-log", "true"]);
    assert!(
        out.status.success(),
        "`jigc config set invocation-log true` must succeed; stderr:\n{}",
        String::from_utf8_lossy(&out.stderr),
    );
}

/// The parsed JSONL records at `.jigc/logs/invocations.jsonl` (empty when the file is absent).
fn log_records(repo: &Path) -> Vec<serde_json::Value> {
    let path = repo.join(".jigc").join("logs").join("invocations.jsonl");
    match fs::read_to_string(&path) {
        Ok(body) => body
            .lines()
            .filter(|l| !l.trim().is_empty())
            .map(|l| serde_json::from_str(l).expect("each log line is valid JSON"))
            .collect(),
        Err(_) => Vec::new(),
    }
}

/// The last record whose `argv` array contains `needle`.
fn record_with_arg<'a>(
    records: &'a [serde_json::Value],
    needle: &str,
) -> Option<&'a serde_json::Value> {
    records.iter().rev().find(|r| {
        r["argv"]
            .as_array()
            .is_some_and(|a| a.iter().any(|v| v.as_str() == Some(needle)))
    })
}

/// Knob ON: a successful invocation appends exactly one well-formed JSONL record carrying
/// every field of the record shape.
#[test]
fn knob_on_appends_one_well_formed_record() {
    let repo = TempDir::new("on");
    let home = TempDir::new("home");
    setup_repo(repo.path(), home.path());
    enable_log(repo.path(), home.path());

    let before = log_records(repo.path()).len();
    let out = jigc(repo.path(), home.path(), &["describe"]);
    assert!(out.status.success(), "`jigc describe` must succeed");

    let records = log_records(repo.path());
    assert_eq!(
        records.len(),
        before + 1,
        "exactly one record is appended per invocation; got {records:#?}",
    );
    let rec = record_with_arg(&records, "describe").expect("the describe invocation is logged");
    assert!(
        rec["timestamp"].as_str().is_some(),
        "record carries a string timestamp; got {rec}",
    );
    assert_eq!(rec["exit_code"].as_u64(), Some(0), "describe exits 0");
    assert!(
        rec["duration_ms"].as_u64().is_some(),
        "record carries a numeric duration_ms; got {rec}",
    );
    assert!(
        rec["finding_codes"].as_array().is_some(),
        "record carries a finding_codes array; got {rec}",
    );
}

/// Knob ON: a clap-rejected invocation (unknown subcommand → exit 2) is logged too — the
/// wrapper straddles `Cli::try_parse()`, so the usage error never escapes uncaptured.
#[test]
fn clap_usage_error_is_logged() {
    let repo = TempDir::new("usage");
    let home = TempDir::new("home");
    setup_repo(repo.path(), home.path());
    enable_log(repo.path(), home.path());

    let out = jigc(repo.path(), home.path(), &["frobnicate-nonexistent"]);
    assert_eq!(
        out.status.code(),
        Some(2),
        "an unknown subcommand is a clap usage error (exit 2)",
    );

    let records = log_records(repo.path());
    let rec = record_with_arg(&records, "frobnicate-nonexistent")
        .expect("the clap-rejected invocation is logged");
    assert_eq!(
        rec["exit_code"].as_u64(),
        Some(2),
        "the logged usage error records exit_code 2; got {rec}",
    );
}

/// Knob ON: a failing verb records its finding codes — a validation-blocked `task validate`
/// (exit 3, unfilled required commit slots) logs a record whose `finding_codes` is non-empty.
#[test]
fn failing_verb_records_finding_codes() {
    let repo = TempDir::new("findings");
    let home = TempDir::new("home");
    setup_repo(repo.path(), home.path());
    enable_log(repo.path(), home.path());

    // Mint a task; its provisioned commit form has empty required slots/fields.
    let out = jigc(
        repo.path(),
        home.path(),
        &["start", "--workflow", "single-task", "block me"],
    );
    assert!(
        out.status.success(),
        "`jigc start` must mint the task; stderr:\n{}",
        String::from_utf8_lossy(&out.stderr),
    );

    let out = jigc(repo.path(), home.path(), &["task", "validate", "block-me"]);
    assert_eq!(
        out.status.code(),
        Some(3),
        "an unfilled task validates as blocking (exit 3); stdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr),
    );

    let records = log_records(repo.path());
    let rec = records
        .iter()
        .rev()
        .find(|r| r["exit_code"].as_u64() == Some(3))
        .expect("the blocked validate is logged with exit_code 3");
    let codes = rec["finding_codes"]
        .as_array()
        .expect("finding_codes is an array");
    assert!(
        !codes.is_empty(),
        "a failing verb records its finding_codes; got {rec}",
    );
}

/// Knob ON: every record carries `binary_version` — the running binary's own
/// `CARGO_PKG_VERSION` (M40 A3) — so a log spanning an upgrade attributes each record to the
/// binary that wrote it. The integration test shares the `cli` package's version, so the
/// expectation is the same `env!` the binary stamps.
#[test]
fn record_carries_binary_version() {
    let repo = TempDir::new("version");
    let home = TempDir::new("home");
    setup_repo(repo.path(), home.path());
    enable_log(repo.path(), home.path());

    let out = jigc(repo.path(), home.path(), &["describe"]);
    assert!(out.status.success(), "`jigc describe` must succeed");

    let records = log_records(repo.path());
    let rec = record_with_arg(&records, "describe").expect("the describe invocation is logged");
    assert_eq!(
        rec["binary_version"].as_str(),
        Some(env!("CARGO_PKG_VERSION")),
        "the record carries the running binary's CARGO_PKG_VERSION; got {rec}",
    );
}

/// Knob ON: the recorded `output_bytes` equals the exact stdout+stderr byte count the driven
/// binary emitted for a known-output verb — the fd-level tee counts the true total, not a
/// per-emitter tally. A swallowing tee (counts but never forwards) fails here too: `Command`
/// would capture 0 bytes while `output_bytes` stayed non-zero.
#[test]
fn output_bytes_equals_emitted_stdout_plus_stderr() {
    let repo = TempDir::new("outsize");
    let home = TempDir::new("home");
    setup_repo(repo.path(), home.path());
    enable_log(repo.path(), home.path());

    let out = jigc(repo.path(), home.path(), &["describe"]);
    assert!(out.status.success(), "`jigc describe` must succeed");
    assert!(
        !out.stdout.is_empty(),
        "`jigc describe` emits a non-empty report (guards against a trivial 0 == 0 pass)",
    );
    let emitted = out.stdout.len() + out.stderr.len();

    let records = log_records(repo.path());
    let rec = record_with_arg(&records, "describe").expect("the describe invocation is logged");
    assert_eq!(
        rec["output_bytes"].as_u64(),
        Some(emitted as u64),
        "logged output_bytes equals the exact stdout+stderr byte count the binary emitted; \
         emitted={emitted}, record={rec}",
    );
}

/// Knob ON: `jigc --help` — a clap-arm emission that exits before dispatch — records
/// `output_bytes` == its emitted-stdout length. This is the fd-tee's differentiator over the
/// rejected per-emitter tally (which records `--help`/`--version` as size 0): it forces the
/// tee to span the `Err(err) => err.print()` arm and be torn down only *after* that emission.
#[test]
fn help_output_bytes_equals_emitted_help_length() {
    let repo = TempDir::new("help");
    let home = TempDir::new("home");
    setup_repo(repo.path(), home.path());
    enable_log(repo.path(), home.path());

    let out = jigc(repo.path(), home.path(), &["--help"]);
    assert_eq!(
        out.status.code(),
        Some(0),
        "`--help` prints to stdout and exits 0"
    );
    assert!(out.stderr.is_empty(), "`--help` writes nothing to stderr");
    assert!(
        !out.stdout.is_empty(),
        "`--help` emits a non-empty help text"
    );
    let emitted = out.stdout.len();

    let records = log_records(repo.path());
    let rec = record_with_arg(&records, "--help").expect("the --help invocation is logged");
    assert_eq!(rec["exit_code"].as_u64(), Some(0), "--help exits 0");
    assert_eq!(
        rec["output_bytes"].as_u64(),
        Some(emitted as u64),
        "`--help` records output_bytes == its emitted stdout length (the clap-arm output a \
         per-emitter tally would record as 0); emitted={emitted}, record={rec}",
    );
}

/// Knob ON (M42, T7): a **hook-rejected finalize** is identifiable in the log — its record
/// carries the route-exempt error identity `finalize.commit-rejected`, while
/// `jigc task finalize <absent-id>` (the other exit-1, no-findings failure) carries none.
/// Before this, both bailed as unstructured `anyhow` errors logging `finding_codes: []` at
/// exit 1 — **byte-identical in the log** (`design/finalize.md` → "A failed finalize must be
/// legible in the invocation log"). The two records must be distinguishable from the log
/// alone, so the assertion pins exactly that: same exit code, same (empty) `finding_codes`,
/// different `error_code`.
#[test]
fn hook_rejected_finalize_is_identifiable_absent_task_is_not() {
    let repo = TempDir::new("rejected");
    let home = TempDir::new("home");
    setup_repo(repo.path(), home.path());
    enable_log(repo.path(), home.path());

    // Mint a task and fill its commit doc, so finalize reaches the commit phase (phase 6)
    // rather than blocking at validate (phase 2).
    let intent = "reject me";
    let task = "reject-me";
    let out = jigc(
        repo.path(),
        home.path(),
        &["start", "--workflow", "single-task", intent],
    );
    assert!(
        out.status.success(),
        "`jigc start` must mint the task; stderr:\n{}",
        String::from_utf8_lossy(&out.stderr),
    );
    for (addr, value) in [
        (format!("commit:{task}#type"), "feat"),
        (format!("commit:{task}#scope"), "cache"),
    ] {
        let out = jigc(
            repo.path(),
            home.path(),
            &["doc", "set-field", &addr, "--value", value],
        );
        assert!(
            out.status.success(),
            "`jigc doc set-field {addr}` must succeed; stderr:\n{}",
            String::from_utf8_lossy(&out.stderr),
        );
    }
    for (addr, prose) in [
        (format!("commit:{task}#summary"), "name the rejection\n"),
        (format!("commit:{task}#body"), "A log-legibility change.\n"),
    ] {
        // `set-slot` reads prose from a file (or `-`); the source lives outside the repo so it
        // never enters the tree the finalize commits.
        let source = home.path().join("prose.md");
        fs::write(&source, prose).expect("write the slot prose");
        let out = jigc(
            repo.path(),
            home.path(),
            &[
                "doc",
                "set-slot",
                &addr,
                "--from-file",
                source.to_str().expect("utf-8 path"),
            ],
        );
        assert!(
            out.status.success(),
            "`jigc doc set-slot {addr}` must succeed; stderr:\n{}",
            String::from_utf8_lossy(&out.stderr),
        );
    }
    fs::write(repo.path().join("code.txt"), "the task's work\n").expect("write code.txt");
    git(repo.path(), &["add", "code.txt"]);

    // A `pre-commit` hook that rejects the commit — the failure class the RC adoption trial
    // hit, and the one the log could not name.
    install_rejecting_hook(repo.path());

    let rejected = jigc(repo.path(), home.path(), &["task", "finalize", task]);
    assert!(
        !rejected.status.success(),
        "a hook-rejected finalize exits non-zero; stdout:\n{}",
        String::from_utf8_lossy(&rejected.stdout),
    );

    let absent = jigc(
        repo.path(),
        home.path(),
        &["task", "finalize", "no-such-task"],
    );
    assert!(
        !absent.status.success(),
        "finalizing an absent task exits non-zero",
    );

    let records = log_records(repo.path());
    let rejected_rec =
        record_with_arg(&records, task).expect("the hook-rejected finalize is logged");
    let absent_rec =
        record_with_arg(&records, "no-such-task").expect("the absent-task finalize is logged");

    // The two failures share an exit code, so `error_code` is the discriminator for the
    // half that has no `Finding` at all. The hook rejection is that half **by design**:
    // git's stderr stays verbatim and unwrapped, so it mints no finding and could only
    // ever name itself through `error_code`.
    assert_eq!(
        rejected_rec["exit_code"], absent_rec["exit_code"],
        "the two finalize failures share an exit code (that is why the log needs the error \
         identity); rejected={rejected_rec}, absent={absent_rec}",
    );
    assert_eq!(
        rejected_rec["finding_codes"].as_array().map(Vec::len),
        Some(0),
        "a hook rejection is an operational error, not a Finding — no finding codes (git's \
         stderr stays verbatim and unwrapped); got {rejected_rec}",
    );
    // The absent-task half **does** name itself, and since M51 Increment 6 / T1 it names
    // itself twice over: the refusal is the typed `finalize.no-task` finding, so the code
    // the surface prints is the code the log records (the M50 completion-audit rule), and
    // `error_code` stays null because it is a `Finding`, not an error identity.
    assert_eq!(
        absent_rec["finding_codes"],
        serde_json::json!(["finalize.no-task"]),
        "an absent task records the refusal it printed; got {absent_rec}",
    );

    assert_eq!(
        rejected_rec["error_code"].as_str(),
        Some("finalize.commit-rejected"),
        "the hook-rejected finalize NAMES its rejection in the log; got {rejected_rec}",
    );
    assert!(
        rejected_rec["error_code"] != absent_rec["error_code"],
        "the two failures are distinguishable from the log alone; rejected={rejected_rec}, \
         absent={absent_rec}",
    );
    assert!(
        absent_rec["error_code"].is_null(),
        "a refusal that travels as a `Finding` carries no error identity (null, not a \
         borrowed one — the identity rides `finding_codes`); got {absent_rec}",
    );
}

/// Knob ON (M43, T4): the **exit-4 migration review hold** names itself in the log — a
/// migration finalize held without `--approve` writes a record with `exit_code` 4 carrying
/// the route-exempt error identity `migrate.review-pending`. Before this, the hold logged
/// `error_code: null` at exit 4 — a coded stop with no *why*, the log-opacity sibling of
/// the hook-rejected case above. The record must be distinguishable from BOTH the
/// hook-rejected finalize (`finalize.commit-rejected`, exit 1) and the absent-id failure
/// (no `error_code`, exit 1): different identity than the former, an identity at all vs the
/// latter.
#[test]
fn migration_review_hold_is_identifiable_in_the_log() {
    let repo = TempDir::new("hold");
    let home = TempDir::new("home");
    setup_repo(repo.path(), home.path());
    enable_log(repo.path(), home.path());

    // The off-router migration task id `jigc migrate` mints for HISTORY.md.
    let task = "migrate-changelog-history-3268e06b69e1";

    // Assert-success runner returning trimmed stdout (the add-item verbs print the minted
    // address as a bare line).
    let ok = |args: &[&str], what: &str| -> String {
        let out = jigc(repo.path(), home.path(), args);
        assert!(
            out.status.success(),
            "`jigc {what}` must succeed; stdout:\n{}\nstderr:\n{}",
            String::from_utf8_lossy(&out.stdout),
            String::from_utf8_lossy(&out.stderr),
        );
        String::from_utf8(out.stdout)
            .expect("utf-8 stdout")
            .trim_end_matches('\n')
            .to_owned()
    };
    // `set-slot` reads prose from a file; the source lives outside the repo so it never
    // enters the tree the finalize would commit.
    let set_slot = |addr: &str, prose: &str| {
        let source = home.path().join("prose.md");
        fs::write(&source, prose).expect("write the slot prose");
        ok(
            &[
                "doc",
                "set-slot",
                addr,
                "--from-file",
                source.to_str().expect("utf-8 path"),
                "--task",
                task,
            ],
            "doc set-slot",
        );
    };

    // Drive the migrate + author spine to the state the review gate holds: a staged
    // conformant canonical changelog over a foreign HISTORY.md, plus a conformant commit
    // doc (so finalize reaches the review gate rather than blocking at validate).
    fs::write(
        repo.path().join("HISTORY.md"),
        "# Changelog\n\n## [0.1.0] - 2021-03-09\n### Added\n- First public release.\n",
    )
    .expect("write foreign HISTORY.md");
    // M51 Inc 1 / T2 — the migrate door takes only a source git holds a copy of.
    git(repo.path(), &["add", "--", "HISTORY.md"]);
    ok(&["migrate", "HISTORY.md", "--as", "changelog"], "migrate");
    ok(
        &[
            "doc",
            "create",
            "changelog",
            "--title",
            "Changelog",
            "--task",
            task,
        ],
        "doc create",
    );
    let release = ok(
        &[
            "doc",
            "add-item",
            "changelog:changelog#releases",
            "--title",
            "0.1.0",
            "--task",
            task,
        ],
        "doc add-item release",
    );
    ok(
        &[
            "doc",
            "set-field",
            &format!("{release}/date"),
            "--value",
            "2021-03-09",
            "--task",
            task,
        ],
        "doc set-field date",
    );
    let group = ok(
        &[
            "doc",
            "add-item",
            &format!("{release}/changes"),
            "--title",
            "Added",
            "--task",
            task,
        ],
        "doc add-item change-group",
    );
    set_slot(&format!("{group}/notes"), "First public release.\n");
    ok(
        &[
            "doc",
            "set-field",
            &format!("commit:{task}#type"),
            "--value",
            "feat",
            "--task",
            task,
        ],
        "doc set-field type",
    );
    ok(
        &[
            "doc",
            "set-field",
            &format!("commit:{task}#scope"),
            "--value",
            "changelog",
            "--task",
            task,
        ],
        "doc set-field scope",
    );
    set_slot(
        &format!("commit:{task}#summary"),
        "adopt the migrated changelog\n",
    );
    set_slot(
        &format!("commit:{task}#body"),
        "Migrate the foreign HISTORY.md into managed shape.\n",
    );

    // The review hold: a migration finalize without `--approve` exits 4, committing nothing.
    let held = jigc(repo.path(), home.path(), &["task", "finalize", task]);
    assert_eq!(
        held.status.code(),
        Some(4),
        "a migration finalize without --approve holds at exit 4; stdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&held.stdout),
        String::from_utf8_lossy(&held.stderr),
    );

    // The other coded-stop-free failure in the same log: an absent task id.
    let absent = jigc(
        repo.path(),
        home.path(),
        &["task", "finalize", "no-such-task"],
    );
    assert!(
        !absent.status.success(),
        "finalizing an absent task exits non-zero",
    );

    let records = log_records(repo.path());
    // `record_with_arg` reads the LAST record carrying the needle — the finalize, not the
    // earlier `--task`-scoped authoring writes.
    let held_rec = record_with_arg(&records, task).expect("the held finalize is logged");
    let absent_rec =
        record_with_arg(&records, "no-such-task").expect("the absent-task finalize is logged");

    assert_eq!(
        held_rec["exit_code"].as_u64(),
        Some(4),
        "the review hold records its distinct exit code; got {held_rec}",
    );
    assert_eq!(
        held_rec["finding_codes"].as_array().map(Vec::len),
        Some(0),
        "the review hold is a human-fidelity gate, not a Finding — no finding codes \
         (the measurement.md:62 rationale); got {held_rec}",
    );
    assert_eq!(
        held_rec["error_code"].as_str(),
        Some("migrate.review-pending"),
        "the held finalize NAMES its hold in the log; got {held_rec}",
    );

    // Three-way distinguishability, from the log alone, by why: the hold's identity is not
    // the hook-rejection's, and the absent-id record carries none at all.
    assert_ne!(
        held_rec["error_code"].as_str(),
        Some("finalize.commit-rejected"),
        "the hold and a hook rejection carry different identities; got {held_rec}",
    );
    assert!(
        absent_rec["error_code"].is_null(),
        "an unstructured operational error carries no borrowed identity; got {absent_rec}",
    );
}

/// Install a `pre-commit` hook that rejects every commit — the commit-phase (phase 6)
/// failure class the RC adoption trial hit, and the one the log could not name.
fn install_rejecting_hook(repo: &Path) {
    let hook = repo.join(".git").join("hooks").join("pre-commit");
    fs::create_dir_all(hook.parent().expect("hooks dir")).expect("create hooks dir");
    fs::write(
        &hook,
        "#!/bin/sh\necho 'lint: trailing whitespace' 1>&2\nexit 1\n",
    )
    .expect("write pre-commit hook");
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        fs::set_permissions(&hook, fs::Permissions::from_mode(0o755)).expect("chmod the hook");
    }
}

/// Stage a sub-task's authored doc body in its isolated working area
/// (`.jigc/tasks/<sub>/docs/<address>.md` + its provenance bit) — what a fanned-out
/// sub-agent leaves for the by-task-id join to merge, so the milestone boundary has a real
/// tree diff to commit (the `milestone.rs` suite's `stage_doc` precedent).
fn stage_subtask_doc(repo: &Path, sub: &str, address: &str, body: &str) {
    let docs = repo.join(".jigc").join("tasks").join(sub).join("docs");
    fs::create_dir_all(&docs).expect("mk the sub-area docs/");
    fs::write(docs.join(format!("{address}.md")), body).expect("write the staged body");
    let manifest = docs.join("provenance.json");
    let mut record: serde_json::Value = match fs::read_to_string(&manifest) {
        Ok(s) => serde_json::from_str(&s).expect("the provenance manifest parses"),
        Err(_) => serde_json::json!({ "docs": {} }),
    };
    record["docs"][address] = serde_json::Value::String("created".to_string());
    fs::write(
        &manifest,
        serde_json::to_string_pretty(&record).expect("serialize the manifest"),
    )
    .expect("write the provenance manifest");
}

/// Knob ON: the **fan-out / milestone** finalize arm names itself in the log while
/// `jigc milestone finalize <absent-id>` names nothing. Without an identity the two are
/// byte-identical on `(exit_code, finding_codes, error_code)` — the L1 defect
/// (`design/finalize.md` → "A failed finalize must be legible in the invocation log": the
/// requirement is *a finalize that did not commit*, not *a per-task finalize*).
///
/// Run over **both** commit modes: `squash: true` (the default single aggregate commit, whose
/// rejection is raised inside the shared executor) and `squash: false` (the per-sub-task commit
/// chain, whose rejection aborts the chain in its dedicated worktree) — a rejection must be
/// identifiable whichever knob is set.
///
/// **M47 Inc 3 T7 revises what "identifiable" means here.** M42 gave both arms the *task*
/// door's `finalize.commit-rejected`, which made them legible at the cost of naming the wrong
/// verb — a law-1 lie on the surface built to stop the log lying. Each commit model now carries
/// its **own** registry identity, so the knob is exactly what the two arms differ on and the
/// log says which construction refused. The identities are read from
/// `cli::invocation_log::COMMITTING_DOORS`, the one code-side axis table, never re-spelled here.
#[test]
fn hook_rejected_milestone_finalize_is_identifiable_absent_milestone_is_not() {
    for squash in ["true", "false"] {
        let repo = TempDir::new("ms-rejected");
        let home = TempDir::new("home");
        setup_repo(repo.path(), home.path());
        enable_log(repo.path(), home.path());

        let out = jigc(
            repo.path(),
            home.path(),
            &["config", "set", "finalize.fan-out.squash", squash],
        );
        assert!(
            out.status.success(),
            "`jigc config set finalize.fan-out.squash {squash}` must succeed; stderr:\n{}",
            String::from_utf8_lossy(&out.stderr),
        );

        // Mint a milestone + one sub-task, and stage a created ADR in the sub-area so the
        // boundary reaches the commit phase (phase 6) with a real diff rather than blocking
        // earlier on the empty-diff backstop.
        for args in [
            vec!["milestone", "create", "Cache rework"],
            vec!["milestone", "add-task", "cache-rework", "Area low"],
        ] {
            let out = jigc(repo.path(), home.path(), &args);
            assert!(
                out.status.success(),
                "`jigc {}` must succeed; stderr:\n{}",
                args.join(" "),
                String::from_utf8_lossy(&out.stderr),
            );
        }
        stage_subtask_doc(
            repo.path(),
            "area-low",
            "adr:eviction-policy",
            "---\nstatus: accepted\ndate: 2026-07-14\n---\n\n# Eviction policy\n\n## Context\n\nForces.\n\n## Options\n\nAlternatives were weighed and rejected.\n\n## Decision\n\nDo the thing.\n\n## Consequences\n\nTradeoffs.\n",
        );

        install_rejecting_hook(repo.path());

        let rejected = jigc(
            repo.path(),
            home.path(),
            &["milestone", "finalize", "cache-rework"],
        );
        assert!(
            !rejected.status.success(),
            "a hook-rejected milestone finalize exits non-zero (squash: {squash}); stdout:\n{}",
            String::from_utf8_lossy(&rejected.stdout),
        );
        assert!(
            String::from_utf8_lossy(&rejected.stderr).contains("`git commit` was rejected"),
            "the rejection surfaces git's hook channel verbatim (squash: {squash}); stderr:\n{}",
            String::from_utf8_lossy(&rejected.stderr),
        );

        let absent = jigc(
            repo.path(),
            home.path(),
            &["milestone", "finalize", "no-such-milestone"],
        );
        assert!(
            !absent.status.success(),
            "finalizing an absent milestone exits non-zero (squash: {squash})",
        );

        let records = log_records(repo.path());
        // `record_with_arg` reads the LAST record carrying the needle — the finalize, not the
        // earlier create/add-task.
        let rejected_rec = record_with_arg(&records, "cache-rework")
            .expect("the hook-rejected milestone finalize is logged");
        let absent_rec = record_with_arg(&records, "no-such-milestone")
            .expect("the absent-milestone finalize is logged");

        // The two failures are indistinguishable on every OTHER field — same exit code, same
        // (empty) finding codes. So `error_code` is the discriminator, or nothing is.
        assert_eq!(
            rejected_rec["exit_code"], absent_rec["exit_code"],
            "the two milestone-finalize failures share an exit code (squash: {squash}); \
             rejected={rejected_rec}, absent={absent_rec}",
        );
        assert_eq!(
            rejected_rec["finding_codes"].as_array().map(Vec::len),
            Some(0),
            "a hook rejection is an operational error, not a Finding (squash: {squash}); got \
             {rejected_rec}",
        );
        let expected = cli::invocation_log::COMMITTING_DOORS
            .iter()
            .find(|door| door.verb == format!("jigc milestone finalize (squash: {squash})"))
            .unwrap_or_else(|| panic!("the axis table must carry the squash: {squash} arm"))
            .error_code;
        assert_eq!(
            rejected_rec["error_code"].as_str(),
            Some(expected),
            "the hook-rejected milestone finalize NAMES ITSELF in the log — its own commit \
             model's identity, not the task door's (squash: {squash}); got {rejected_rec}",
        );
        assert_ne!(
            rejected_rec["error_code"].as_str(),
            Some("finalize.commit-rejected"),
            "a milestone finalize must not log the TASK door's identity (squash: {squash}); \
             got {rejected_rec}",
        );
        assert!(
            absent_rec["error_code"].is_null(),
            "an unstructured operational error carries no borrowed identity (squash: {squash}); \
             got {absent_rec}",
        );
    }
}

/// Knob OFF (the default): nothing is written — no `.jigc/logs/` file appears.
#[test]
fn knob_off_writes_nothing() {
    let repo = TempDir::new("off");
    let home = TempDir::new("home");
    setup_repo(repo.path(), home.path());

    let out = jigc(repo.path(), home.path(), &["describe"]);
    assert!(out.status.success(), "`jigc describe` must succeed");

    assert!(
        !repo
            .path()
            .join(".jigc")
            .join("logs")
            .join("invocations.jsonl")
            .exists(),
        "the default (knob OFF) writes no invocation log",
    );
}

/// Outside a jigc repo (a git repo with no `.jigc/` project layer): the wrapper no-ops —
/// nothing is written even though the knob's default read would be attempted.
#[test]
fn outside_jigc_repo_writes_nothing() {
    let repo = TempDir::new("bare");
    let home = TempDir::new("home");
    git(repo.path(), &["init", "-q"]);

    let out = jigc(repo.path(), home.path(), &["describe"]);
    // describe errors without a project layer, but the wrapper must still no-op silently.
    let _ = out;
    assert!(
        !repo.path().join(".jigc").join("logs").exists(),
        "a run outside a jigc project layer writes no invocation log",
    );
}

/// The store hygiene split: `.jigc/logs/` is git-ignored (never committed), while the T2
/// `.jigc/version` provenance stamp stays committed.
#[test]
fn logs_ignored_version_committed() {
    let repo = TempDir::new("ignore");
    let home = TempDir::new("home");
    setup_repo(repo.path(), home.path());
    enable_log(repo.path(), home.path());
    let _ = jigc(repo.path(), home.path(), &["describe"]);

    // `.jigc/logs/` is check-ignore-ignored (git prints the path + exits 0).
    let out = Command::new("git")
        .args(["check-ignore", ".jigc/logs/invocations.jsonl"])
        .current_dir(repo.path())
        .output()
        .expect("run git check-ignore");
    assert!(
        out.status.success(),
        "`.jigc/logs/` must be git-ignored; check-ignore stdout:\n{}",
        String::from_utf8_lossy(&out.stdout),
    );

    // `.jigc/version` is NOT ignored (exit 1) and IS tracked.
    let out = Command::new("git")
        .args(["check-ignore", ".jigc/version"])
        .current_dir(repo.path())
        .output()
        .expect("run git check-ignore");
    assert!(
        !out.status.success(),
        "`.jigc/version` must NOT be git-ignored (it is committed provenance)",
    );
    let tracked = git(repo.path(), &["ls-files"]);
    assert!(
        tracked.lines().any(|p| p == ".jigc/version"),
        "`.jigc/version` stays committed; tracked:\n{tracked}",
    );
}
