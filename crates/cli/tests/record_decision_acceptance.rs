//! M44 Increment 5 (T1) acceptance — the `record-decision` from-knowledge ADR
//! authoring workflow, end-to-end through the built `jigc` binary against the
//! **on-disk** dev pack (`JIGC_PACK_DIR = crates/cli/pack`).
//!
//! The sole-channel hole for the flagship doctype: before this, an agent could
//! author an `adr` only inside a *code* task (`single-task`) or by *migrating* a
//! foreign source (`migrate-adr`). There was no honest front door for "I have made
//! a decision, record it, no code to write". `record-decision` is that door — a
//! selectable work-workflow that mirrors the `plan.yaml` / `architecture-documentation.yaml`
//! from-knowledge precedent (`allows-create: [{type: adr, as: decision}]`, plain
//! `finalize` — NOT `migration-finalize`, whose prose would lie about a review-hold
//! and a foreign-retire that never happen, S1).
//!
//! What this proves (`roadmap.md` → M44 Increment 5; `DECISIONS.md` → 2026-07-21):
//!
//! - the composed step names NO foreign source (the `author-migration-adr`
//!   foreign-mapping / transcribe-date / supersedes-from-foreign prose is absent);
//! - authoring the `context` / `decision` / `consequences` slots then **plain**
//!   `jigc task finalize <id>` (no `--approve`, no review-hold) commits **one** adr
//!   at its canonical `docs/decisions/<slug>.md`;
//! - the committed adr carries a **FRESH on-create date** (today's stamp, not a
//!   transcribed foreign date — the on-create stamp is kept, not suppressed);
//! - the workflow appears in the router catalog and in `jigc describe`.
//!
//! Everything drives the real binary; the temp repo is a real `git init`;
//! self-cleaning `TempDir`s keep the developer's repo clean.

use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

/// A throwaway directory that removes itself on drop.
struct TempDir(PathBuf);

impl TempDir {
    fn new(tag: &str) -> Self {
        let mut path = std::env::temp_dir();
        path.push(format!(
            "jigc-record-decision-{tag}-{}-{:?}",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos(),
        ));
        std::fs::create_dir_all(&path).expect("create temp dir");
        TempDir(path)
    }

    fn path(&self) -> &Path {
        &self.0
    }
}

impl Drop for TempDir {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.0);
    }
}

/// The on-disk dev pack tree — the faithful source the embedded pack mirrors.
fn pack_dir() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("pack")
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

/// Initialize a real git repo with one commit + the `.jigc/config/` project layer.
fn init_repo(repo: &Path) {
    git(repo, &["init", "-q"]);
    git(repo, &["config", "user.email", "test@example.com"]);
    git(repo, &["config", "user.name", "Test"]);
    std::fs::write(repo.join("README.md"), "# repo\n").expect("write readme");
    git(repo, &["add", "."]);
    git(repo, &["commit", "-q", "-m", "initial"]);
    std::fs::create_dir_all(repo.join(".jigc").join("config")).expect("create project layer");
}

/// Run `jigc <args>` with `cwd = repo`, `$HOME = home`, `JIGC_PACK_DIR = the on-disk pack`.
fn jigc(repo: &Path, home: &Path, args: &[&str]) -> std::process::Output {
    Command::new(env!("CARGO_BIN_EXE_jigc"))
        .args(args)
        .current_dir(repo)
        .env("HOME", home)
        .env("JIGC_PACK_DIR", pack_dir())
        .output()
        .expect("run the jigc binary")
}

/// Run `jigc doc <args>`, optionally piping `stdin`, capturing output.
fn jigc_doc(repo: &Path, home: &Path, args: &[&str], stdin: Option<&[u8]>) -> std::process::Output {
    let mut command = Command::new(env!("CARGO_BIN_EXE_jigc"));
    command.arg("doc").args(args);
    command
        .current_dir(repo)
        .env("HOME", home)
        .env("JIGC_PACK_DIR", pack_dir());
    if stdin.is_some() {
        command.stdin(Stdio::piped());
    }
    command.stdout(Stdio::piped()).stderr(Stdio::piped());
    let mut child = command.spawn().expect("spawn jigc");
    if let Some(bytes) = stdin {
        child
            .stdin
            .take()
            .expect("stdin piped")
            .write_all(bytes)
            .expect("write stdin");
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

/// Set a doc slot from piped stdin, asserting success.
fn set_slot(repo: &Path, home: &Path, addr: &str, prose: &[u8]) {
    let out = jigc_doc(
        repo,
        home,
        &["set-slot", addr, "--from-file", "-"],
        Some(prose),
    );
    assert_ok(&out, &format!("set-slot {addr}"));
}

/// Set a doc field, asserting success.
fn set_field(repo: &Path, home: &Path, addr: &str, value: &str) {
    let out = jigc_doc(repo, home, &["set-field", addr, "--value", value], None);
    assert_ok(&out, &format!("set-field {addr}"));
}

/// Today's date the same way the binary stamps it (system local date), so the
/// fresh-on-create assertion is exact, not just well-formed.
fn today() -> String {
    // The CLI's `set: on-create` date stamp derives from `SystemTime` days-since-epoch
    // in **UTC** (`doc::today_iso`, `secs / 86_400`). Read the same clock here — a local
    // `date +%Y-%m-%d` disagrees with the CLI's UTC stamp across the UTC/local midnight
    // boundary (a machine east of UTC flips a day early), which is a spurious failure.
    let out = Command::new("date")
        .args(["-u", "+%Y-%m-%d"])
        .output()
        .expect("run date");
    String::from_utf8(out.stdout)
        .expect("utf-8")
        .trim()
        .to_owned()
}

/// The one loop: `jigc start --workflow record-decision` → author the three adr
/// slots + the transient commit → plain `jigc task finalize` → one committed adr
/// carrying today's on-create date.
#[test]
fn record_decision_authors_an_adr_from_knowledge_with_a_fresh_date() {
    let repo = TempDir::new("loop-repo");
    let home = TempDir::new("loop-home");
    init_repo(repo.path());

    // Compose the from-knowledge workflow.
    let out = jigc(
        repo.path(),
        home.path(),
        &[
            "start",
            "--workflow",
            "record-decision",
            "adopt blake3 for content hashing",
        ],
    );
    assert_ok(&out, "`jigc start --workflow record-decision`");
    let stdout = String::from_utf8(out.stdout).expect("utf-8 stdout");

    // The task minted under its own working area, recording the workflow id.
    let task = "adopt-blake3-for-content-hashing";
    let task_dir = repo.path().join(".jigc").join("tasks").join(task);
    assert!(
        task_dir.join("base.json").is_file(),
        "record-decision is creates-task: true, so it must open .jigc/tasks/{task}/ with a base pin",
    );
    let recorded =
        std::fs::read_to_string(task_dir.join("workflow")).expect("read recorded workflow id");
    assert_eq!(
        recorded.trim(),
        "record-decision",
        "the minted task must record `workflow: record-decision`",
    );

    // The composed step frames the intent and offers the adr create-gate.
    assert!(
        stdout.contains("adopt blake3 for content hashing"),
        "the composed step must frame the task intent; got:\n{stdout}",
    );
    assert!(
        stdout.contains("jigc doc create adr"),
        "the composed step must offer the adr create-gate; got:\n{stdout}",
    );

    // The from-knowledge step names NO foreign source — none of the migration
    // prose (`author-migration-adr`) may survive into this door. (`supersedes` is
    // NOT migration prose — it is a legitimate adr ref-edge named via the
    // non-migration `superseded-context` step, M45 T3; the words below discriminate
    // migration vocabulary.)
    let lower = stdout.to_lowercase();
    for banned in ["foreign", "transcribe", "staged for you"] {
        assert!(
            !lower.contains(banned),
            "the from-knowledge step must not carry the migration prose word {banned:?}; got:\n{stdout}",
        );
    }
    // The create-gate is granted for the `decision` role.
    assert!(
        stdout.contains("adr"),
        "the compose must name the adr doctype; got:\n{stdout}",
    );

    // Author the three required slots (from knowledge — no source consulted).
    let create = jigc_doc(
        repo.path(),
        home.path(),
        &["create", "adr", "--title", "Adopt blake3"],
        None,
    );
    assert_ok(&create, "`jigc doc create adr`");
    let adr = String::from_utf8(create.stdout)
        .expect("utf-8")
        .trim()
        .to_string();
    assert_eq!(adr, "adr:adopt-blake3", "minted adr address");

    set_slot(
        repo.path(),
        home.path(),
        "adr:adopt-blake3#context",
        b"We need a fast, collision-resistant content hash.\n",
    );
    set_slot(
        repo.path(),
        home.path(),
        "adr:adopt-blake3#decision",
        b"Adopt blake3 for all content hashing.\n",
    );
    set_slot(
        repo.path(),
        home.path(),
        "adr:adopt-blake3#consequences",
        b"A vendored dependency; hashes are not sha-compatible.\n",
    );

    // Fill the transient commit doc so plain finalize renders a clean git message
    // (the workflow authors the record, not the commit — the arch-doc precedent).
    set_field(
        repo.path(),
        home.path(),
        &format!("commit:{task}#type"),
        "docs",
    );
    set_field(
        repo.path(),
        home.path(),
        &format!("commit:{task}#scope"),
        "adr",
    );
    set_slot(
        repo.path(),
        home.path(),
        &format!("commit:{task}#summary"),
        b"record the blake3 decision\n",
    );

    // PLAIN finalize — no `--approve`, no review-hold.
    let before: u32 = git(repo.path(), &["rev-list", "--count", "HEAD"])
        .parse()
        .unwrap();
    let fin = jigc(repo.path(), home.path(), &["task", "finalize", task]);
    let rendered = format!(
        "{}{}",
        String::from_utf8_lossy(&fin.stdout),
        String::from_utf8_lossy(&fin.stderr),
    );
    assert_ok(&fin, &format!("plain finalize; got:\n{rendered}"));

    // Exactly one commit landed.
    let after: u32 = git(repo.path(), &["rev-list", "--count", "HEAD"])
        .parse()
        .unwrap();
    assert_eq!(
        after,
        before + 1,
        "plain finalize must land exactly ONE commit"
    );

    // The adr committed at its canonical decisions path.
    let committed = Command::new("git")
        .args(["show", "HEAD:docs/decisions/adopt-blake3.md"])
        .current_dir(repo.path())
        .output()
        .expect("git show");
    assert!(
        committed.status.success(),
        "the adr must commit at docs/decisions/adopt-blake3.md; stderr:\n{}",
        String::from_utf8_lossy(&committed.stderr),
    );
    let body = String::from_utf8(committed.stdout).expect("utf-8");

    // The date is a FRESH on-create stamp: today's date, materialized by the CLI
    // (never transcribed from a foreign source — there is none). The migration
    // date-suppression prose is absent, so the on-create stamp survives.
    let today = today();
    assert!(
        body.contains(&format!("date: {today}")),
        "the committed adr must carry a fresh on-create date ({today}); got:\n{body}",
    );

    // The staged working area is gone (promoted, not left behind).
    assert!(
        !task_dir.exists(),
        "a finalized task's working area is cleaned up",
    );
}

/// The workflow is a first-class selectable surface: it appears in the router
/// catalog (`jigc start`, no workflow) and in the `jigc describe` menu — law 2,
/// nothing hides.
#[test]
fn record_decision_is_listed_in_the_router_catalog_and_describe() {
    let repo = TempDir::new("catalog-repo");
    let home = TempDir::new("catalog-home");
    init_repo(repo.path());

    // Bare `jigc start` composes the router catalog.
    let orient = jigc(repo.path(), home.path(), &["start"]);
    assert_ok(&orient, "bare `jigc start` (router catalog)");
    let catalog = String::from_utf8(orient.stdout).expect("utf-8");
    assert!(
        catalog.contains("record-decision"),
        "the router catalog must list the selectable record-decision workflow; got:\n{catalog}",
    );

    // `jigc describe` names it with its situation line.
    let describe = jigc(repo.path(), home.path(), &["describe"]);
    assert_ok(&describe, "`jigc describe`");
    let tour = String::from_utf8(describe.stdout).expect("utf-8");
    assert!(
        tour.contains("record-decision is"),
        "`jigc describe` must name the record-decision workflow; got:\n{tour}",
    );
}
