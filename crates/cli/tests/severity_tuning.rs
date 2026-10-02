//! End-to-end severity-tuning + demotion-lock (M6 inc-2, T5) — both tiers of the
//! two-tier rule driven **verbatim through the emitted `jigc` binary** against a
//! throwaway git repo, asserted on the emitted bytes (`design/validation.md` → The
//! two-tier rule; `implementation/roadmap.md` → M6 inc-2 grouped-scope bullet 5;
//! `increment-workflow.md` → Validation hardening #4 + #5):
//!
//! - **(a) a real-key tunable demotion re-grades a check.** `jigc config set
//!   validation.file-state.hash-matches.severity advisory` + an out-of-band edit to a
//!   committed, baselined ADR → the store-scope `jigc validate` surfaces the drift as
//!   an **advisory** finding rather than a blocking one. `hash-matches` is a *tunable*
//!   check (no floor), so the demotion applies. (Its live surface is the store sweep:
//!   since M43 A14 the task-scope staged loop never keys an instance against the
//!   record, so `task validate` mints no `hash-matches` — the task-scope demotion
//!   proof with exit-code observability lives in flow 13's tunable `doc-code` walk.)
//! - **(b) an intrinsic demotion is floor-rejected.** `jigc config set
//!   validation.workflow-refs.placeholder-resolves.severity advisory` targets a
//!   *floored* intrinsic check. The below-floor `scalar-set` is **soft-rejected at
//!   cascade resolution** — the delta is *not applied* (the resolved knob stays
//!   `blocking`, so the check still blocks where it fires: the JSON `--explain`
//!   carries the key in `rejected_demotions`, never in `scalar_overrides`, and it is
//!   not folded into `overrides applied: N`), and `jigc start --explain` shows the
//!   rejected-demotion line (attempted `advisory` · floor `blocking` · `project`).
//!
//! These two are the **supporting** half of the flow-8 trio (`worked-examples.md` →
//! flow 8, Supporting): (a) the *tunable demote stops blocking* and (b) the *intrinsic
//! demote is floor-rejected + shown in `--explain`*. The flow-8 **headline** — the M5
//! `override-default` conflict demoted blocking→warning so `jigc upgrade` warns instead
//! of blocks (exit 0) — lives in `flow8_override_default_warning.rs`; the *no-override
//! byte-identical* supporting leg lives in `upgrade.rs`'s no-delta byte-identity
//! goldens (cross-referenced so the flow is tracked complete — hardening #1 / #6).
//!
//! No external test crates: the binary path comes from `CARGO_BIN_EXE_jigc`, the temp
//! repo is a real `git init`, and a self-cleaning `TempDir` keeps the test off the
//! dev's repo.

use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

/// A throwaway directory that removes itself on drop.
struct TempDir(PathBuf);

impl TempDir {
    fn new(tag: &str) -> Self {
        let mut path = std::env::temp_dir();
        let unique = format!(
            "jigc-severity-tuning-{tag}-{}-{:?}",
            std::process::id(),
            engine::tempname::unique_nanos(),
        );
        path.push(unique);
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

/// Run a `git` command in `repo`, asserting success.
fn git(repo: &Path, args: &[&str]) {
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
}

/// Initialize a real git repo with one commit + the `.jigc/config/` project layer.
fn init_repo(repo: &Path) {
    git(repo, &["init", "-q"]);
    git(repo, &["config", "user.email", "test@example.com"]);
    git(repo, &["config", "user.name", "Test"]);
    fs::write(repo.join("README.md"), "hello\n").expect("write file");
    git(repo, &["add", "."]);
    git(repo, &["commit", "-q", "-m", "initial"]);
    fs::create_dir_all(repo.join(".jigc").join("config")).expect("create project layer");
}

/// Run `jigc <args>` with `cwd = repo`, `$HOME = home`, capturing output.
fn jigc(repo: &Path, home: &Path, args: &[&str]) -> std::process::Output {
    Command::new(env!("CARGO_BIN_EXE_jigc"))
        .args(args)
        .current_dir(repo)
        .env("HOME", home)
        .output()
        .expect("run the jigc binary")
}

/// Run `jigc doc <args>`, optionally piping `stdin`, capturing output.
fn jigc_doc(repo: &Path, home: &Path, args: &[&str], stdin: Option<&[u8]>) -> std::process::Output {
    let mut command = Command::new(env!("CARGO_BIN_EXE_jigc"));
    command.arg("doc").args(args);
    command.current_dir(repo).env("HOME", home);
    if stdin.is_some() {
        command.stdin(Stdio::piped());
    }
    command.stdout(Stdio::piped()).stderr(Stdio::piped());
    let mut child = command.spawn().expect("spawn jigc");
    if let Some(bytes) = stdin {
        crate::support::child_stdin::feed(&mut child, bytes);
    }
    child.wait_with_output().expect("wait for jigc")
}

/// Assert a `jigc` invocation succeeded, surfacing its streams on failure.
fn assert_ok(out: &std::process::Output, what: &str) {
    assert!(
        out.status.success(),
        "{what} must succeed; stdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr),
    );
}

/// The combined stdout+stderr of a `jigc` invocation (findings render to either).
fn streams(out: &std::process::Output) -> String {
    format!(
        "{}{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr),
    )
}

/// Fill every author-required field/slot of the provisioned commit doc so a validate
/// over it is clean of `schema-conformance` findings (isolating the `file-state` drift).
fn fill_commit(repo: &Path, home: &Path, task: &str) {
    let set_field = |addr: &str, value: &str| {
        let out = jigc_doc(repo, home, &["set-field", addr, "--value", value], None);
        assert_ok(&out, &format!("set-field {addr}"));
    };
    let set_slot = |addr: &str, prose: &[u8]| {
        let out = jigc_doc(
            repo,
            home,
            &["set-slot", addr, "--from-file", "-"],
            Some(prose),
        );
        assert_ok(&out, &format!("set-slot {addr}"));
    };
    set_field(&format!("commit:{task}#type"), "feat");
    set_field(&format!("commit:{task}#scope"), "limiter");
    set_slot(&format!("commit:{task}#summary"), b"add a rate limiter\n");
    set_slot(&format!("commit:{task}#body"), b"A rate limiter.\n");
}

/// One whole task that creates + finalizes `adr:single-node-cache`, committing it to
/// `docs/decisions/single-node-cache.md` and baselining it in the file-state record
/// (the `validate_envelope.rs` idiom) — the committed managed doc the OOB edit drifts.
fn commit_baselined_adr(repo: &Path, home: &Path) {
    let start = jigc(
        repo,
        home,
        &[
            "start",
            "--workflow",
            "single-task",
            "cache sessions in a single in-memory node",
        ],
    );
    assert_ok(&start, "`jigc start`");
    let task = "cache-sessions-in-a-single";

    let create = jigc_doc(
        repo,
        home,
        &["create", "adr", "--title", "Single-node cache"],
        None,
    );
    assert_ok(&create, "`jigc doc create adr`");
    for (slot, prose) in [
        (
            "context",
            &b"Session lookups must stay sub-millisecond.\n"[..],
        ),
        ("decision", b"Keep sessions in a single in-memory node.\n"),
        ("consequences", b"A cold node loses its sessions.\n"),
    ] {
        let out = jigc_doc(
            repo,
            home,
            &[
                "set-slot",
                &format!("adr:single-node-cache#{slot}"),
                "--from-file",
                "-",
            ],
            Some(prose),
        );
        assert_ok(&out, &format!("set-slot #{slot}"));
    }
    fill_commit(repo, home, task);
    let out = jigc(repo, home, &["task", "finalize", task]);
    assert_ok(&out, "`jigc task finalize`");
}

// ─────────────── (a) tunable demotion re-grades a check ───────────────

/// The committed ADR's canonical path (default `docs-root`: `docs/`).
const COMMITTED_ADR: &str = "docs/decisions/single-node-cache.md";

#[test]
fn tunable_severity_demotion_regrades_a_file_state_drift() {
    let repo = TempDir::new("tunable");
    let home = TempDir::new("home");
    init_repo(repo.path());
    // Set up the project layer the store sweep reads.
    let setup = jigc(repo.path(), home.path(), &["setup"]);
    assert_ok(&setup, "`jigc setup`");

    // A committed, baselined ADR, then an out-of-band edit so its on-disk bytes
    // diverge from the recorded baseline — the store-scope drift.
    commit_baselined_adr(repo.path(), home.path());
    let committed = repo.path().join(COMMITTED_ADR);
    let mut body = fs::read_to_string(&committed).expect("read the committed ADR");
    body.push_str("\nAn out-of-band human edit appended after baseline.\n");
    fs::write(&committed, &body).expect("apply the out-of-band edit");

    // Before the demotion: `file-state.hash-matches` is blocking by default →
    // `jigc validate` surfaces the drift as a *blocking* finding (report-only, exit 0).
    let before = jigc(repo.path(), home.path(), &["validate"]);
    assert_ok(
        &before,
        "`jigc validate` (report-only, before the demotion)",
    );
    assert!(
        streams(&before).contains("blocking (gates at finalize) · file-state.hash-matches"),
        "the default drift finding must render as `blocking`; got:\n{}",
        streams(&before)
    );

    // Demote the *tunable* check to advisory — a real-key `scalar-set` on the closed
    // surface (`hash-matches` carries no floor, so the demotion applies).
    let set = jigc(
        repo.path(),
        home.path(),
        &[
            "config",
            "set",
            "validation.file-state.hash-matches.severity",
            "advisory",
        ],
    );
    assert_ok(&set, "`jigc config set` (tunable demotion)");

    // After the demotion: the SAME drift is surfaced as an *advisory* finding — files
    // are truth, the drift is still reported, just re-graded. The drift state itself
    // is unchanged (the read-only sweep never advances the record).
    let after = jigc(repo.path(), home.path(), &["validate"]);
    assert_ok(&after, "`jigc validate` (report-only, after the demotion)");
    let surfaced = streams(&after);
    assert!(
        surfaced.contains("advisory · file-state.hash-matches"),
        "the demoted drift must render as `advisory`; got:\n{surfaced}"
    );
    assert!(
        !surfaced.contains("blocking (gates at finalize) · file-state.hash-matches"),
        "the demoted drift must NOT render as `blocking`; got:\n{surfaced}"
    );
}

// ─────────────── (b) intrinsic demotion is floor-rejected ───────────────

/// The exact rejected-demotion line the `--explain` agent text must emit for a
/// floor-rejected intrinsic demotion (key · attempted · floor · source layer).
const REJECTED_LINE: &str = "rejected demotion: \
     validation.workflow-refs.placeholder-resolves.severity = advisory below floor blocking    (project)";

#[test]
fn intrinsic_severity_demotion_is_floor_rejected_and_shown_in_explain() {
    let repo = TempDir::new("intrinsic");
    let home = TempDir::new("home");
    init_repo(repo.path());

    // `config set` records the scalar-set into the project layer (the closed-surface
    // check passes — the key is a declared knob; the floor is adjudicated at *resolve*
    // time, not write time).
    let set = jigc(
        repo.path(),
        home.path(),
        &[
            "config",
            "set",
            "validation.workflow-refs.placeholder-resolves.severity",
            "advisory",
        ],
    );
    assert_ok(&set, "`jigc config set` (intrinsic, below-floor)");

    // `jigc start --explain` resolves the cascade and renders its layer-1 provenance.
    let explain = jigc(
        repo.path(),
        home.path(),
        &[
            "start",
            "--explain",
            "--workflow",
            "single-task",
            "add rate limiter",
        ],
    );
    assert_ok(&explain, "`jigc start --explain`");
    let stdout = String::from_utf8(explain.stdout).expect("utf-8 stdout");

    // The agent-text body shows the rejected-demotion line on the emitted bytes
    // (hardening #4 — assert the emitted artifact, not a reconstruction).
    assert!(
        stdout.contains(REJECTED_LINE),
        "the floor-rejected intrinsic demotion must render its rejected-demotion line \
         (attempted advisory · floor blocking · project); got:\n{stdout}"
    );
    // The rejected demotion is NOT applied: it does not appear as an applied override,
    // and it is not folded into the `overrides applied: N` count.
    assert!(
        !stdout.contains(
            "validation.workflow-refs.placeholder-resolves.severity = advisory    (project)"
        ),
        "a floor-rejected demotion must NOT render as an applied override; got:\n{stdout}"
    );
    assert!(
        stdout.contains("overrides applied: none"),
        "a floor-rejected demotion is the only delta — it is logged, not applied, so no \
         override is counted; got:\n{stdout}"
    );

    // The structured JSON projection is the strongest "the delta is not applied" proof:
    // the key rides `rejected_demotions`, NEVER `scalar_overrides`, so the resolved knob
    // stays `blocking` and the check still blocks where it fires.
    let json_out = jigc(
        repo.path(),
        home.path(),
        &[
            "start",
            "--explain",
            "--format",
            "json",
            "--workflow",
            "single-task",
            "add rate limiter",
        ],
    );
    assert_ok(&json_out, "`jigc start --explain --format json`");
    let value: serde_json::Value =
        serde_json::from_str(&String::from_utf8(json_out.stdout).expect("utf-8"))
            .expect("--format json must emit parseable JSON");

    // Not applied: absent from scalar_overrides, present in rejected_demotions.
    let scalars = value["scalar_overrides"]
        .as_array()
        .cloned()
        .unwrap_or_default();
    assert!(
        scalars
            .iter()
            .all(|s| s["key"] != "validation.workflow-refs.placeholder-resolves.severity"),
        "the floored key must NOT appear in scalar_overrides (not applied); got:\n{value:#}"
    );
    let rejected = value["rejected_demotions"]
        .as_array()
        .expect("rejected_demotions is an array");
    let entry = rejected
        .iter()
        .find(|r| r["key"] == "validation.workflow-refs.placeholder-resolves.severity")
        .expect("the floored key appears in rejected_demotions");
    assert_eq!(entry["attempted"], "advisory", "got:\n{value:#}");
    assert_eq!(entry["floor"], "blocking", "got:\n{value:#}");
    assert_eq!(entry["layer"], "project", "got:\n{value:#}");
    // Not counted as an applied override.
    assert_eq!(
        value["overrides_applied"], 0,
        "a floor-rejected demotion is not folded into overrides_applied; got:\n{value:#}"
    );
}
