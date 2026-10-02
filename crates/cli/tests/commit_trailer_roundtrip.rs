//! On-disk round-trip for the `commit` doctype's **trailers**, driven purely through
//! the write verbs (M45 Increment 4 T1 — `pinning.md` §2; `finalize.md` → Commit-doc
//! rendering / trailers).
//!
//! The `commit` schema's `trailers` block is `id-from: key`, so an item authored via
//! `jigc doc add-item …#trailers --title <Key>` carries the key as its **heading**,
//! never as a `- key:` field bullet (`store.rs` → `resolve_leaf`: the `id-from` leaf
//! resolves to the item heading). The commit renderer's `trailer_lines` therefore has
//! to read the key from `item.title`; reading a `key` field returns `None` for every
//! CLI-authored trailer, so the trailer silently never reaches the git message.
//!
//! This is the first test to author a `commit` trailer **through the verbs** (not a
//! hand-built `Instance` that pre-sets both a `title` and a synthetic `key` field) and
//! assert the finalized commit's `%(trailers)`. It closes the round-trip hole the
//! `commit` doctype was exempted from ("no on-disk round-trip to fence" — false: the
//! git message *is* the on-disk artifact, and the verbs are the only way to reach it).
//!
//! No external test crates: the binary path comes from `CARGO_BIN_EXE_jigc`, the temp
//! repo is a real `git init`, and a self-cleaning `TempDir` keeps the test off the repo.
//!
//! **The title-write door axis** (the confidence-audit wave — sibling-hunt finding 6):
//! the trailer key-shape rule is scoped `doctype == "commit" && id-from == "key"`, and
//! `commit` is the only trailer-carrying doctype, so the axis is the write doors that
//! can set an item title on a `commit` trailers item. Each member's disposition:
//!
//! - `jigc doc add-item` — **covered**: wired via `id_from_enum_block`
//!   (`doc.rs`), tested here (`a_whitespace_trailer_key_is_rejected_at_add_item`).
//! - `jigc doc author` — **covered by the same door**: its lowered `add-item` leaves
//!   run through `apply_leaf`, which calls the same `id_from_enum_block`.
//! - `jigc doc retitle-item` — **covered**: the sibling-hunt's finding-6 door,
//!   unguarded as shipped by M45 inc-4; guarded + tested here
//!   (`a_malformed_trailer_key_is_rejected_at_retitle_item`).
//! - `jigc doc set-field <item>/key` — **already-safe**: the set-field id-from guard
//!   refuses every id-from leaf write and routes a string id-from to `retitle-item`
//!   — which is exactly why the retitle door carrying the rule is load-bearing.
//! - `jigc doc set-slot` — **excluded**: slots are prose, an item heading is not
//!   slot-addressable, and the item-region validate-after rejects reserved-depth
//!   headings smuggled into slot prose (the M45 item-slot ceiling sweep).
//! - migration transforms / `migrate-corpus` — **excluded**: `commit` is transient
//!   (its sink is the git message, never a persisted corpus file), so no corpus
//!   migration path writes a trailer heading.
//! - out-of-band file edit — **excluded by design** (the honest boundary): caught
//!   origin-independently at the task gate, tested here
//!   (`a_whitespace_trailer_key_is_blocked_at_the_task_gate`).

use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

/// A throwaway directory that removes itself on drop.
struct TempDir(PathBuf);

impl TempDir {
    fn new(tag: &str) -> Self {
        let mut path = std::env::temp_dir();
        let unique = format!(
            "jigc-trailer-{tag}-{}-{:?}",
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

/// Initialize a real git repo with one commit and the `.jigc/config/` project layer,
/// then `jigc start --workflow single-task "<intent>"`. Returns the repo + `$HOME`.
fn started_repo(intent: &str) -> (TempDir, TempDir) {
    let repo = TempDir::new("repo");
    let home = TempDir::new("home");
    git(repo.path(), &["init", "-q"]);
    git(repo.path(), &["config", "user.email", "test@example.com"]);
    git(repo.path(), &["config", "user.name", "Test"]);
    fs::write(repo.path().join("README.md"), "hello\n").expect("write file");
    git(repo.path(), &["add", "."]);
    git(repo.path(), &["commit", "-q", "-m", "initial"]);
    fs::create_dir_all(repo.path().join(".jigc").join("config")).expect("create project layer");

    let out = Command::new(env!("CARGO_BIN_EXE_jigc"))
        .args(["start", "--workflow", "single-task", intent])
        .current_dir(repo.path())
        .env("HOME", home.path())
        .output()
        .expect("run jigc start");
    assert!(
        out.status.success(),
        "`jigc start` must provision the task; stderr:\n{}",
        String::from_utf8_lossy(&out.stderr)
    );
    (repo, home)
}

/// Run `jigc doc <args>` with `cwd = repo`, optionally piping `stdin`.
fn run_doc(repo: &Path, home: &Path, args: &[&str], stdin: Option<&[u8]>) -> std::process::Output {
    let mut command = Command::new(env!("CARGO_BIN_EXE_jigc"));
    command.arg("doc");
    command.args(args);
    command.current_dir(repo).env("HOME", home);
    if stdin.is_some() {
        command.stdin(Stdio::piped());
    }
    command.stdout(Stdio::piped()).stderr(Stdio::piped());
    let mut child = command.spawn().expect("spawn the jigc binary");
    if let Some(bytes) = stdin {
        crate::support::child_stdin::feed(&mut child, bytes);
    }
    child.wait_with_output().expect("wait for jigc")
}

/// Run `jigc task <args>` with `cwd = repo`.
fn run_task(repo: &Path, home: &Path, args: &[&str]) -> std::process::Output {
    Command::new(env!("CARGO_BIN_EXE_jigc"))
        .arg("task")
        .args(args)
        .current_dir(repo)
        .env("HOME", home)
        .output()
        .expect("run jigc task")
}

/// Set a commit-doc field/slot through the verbs, asserting success.
fn set_field(repo: &Path, home: &Path, addr: &str, value: &str) {
    let out = run_doc(repo, home, &["set-field", addr, "--value", value], None);
    assert!(
        out.status.success(),
        "set-field {addr}={value} must succeed; stderr:\n{}",
        String::from_utf8_lossy(&out.stderr)
    );
}

fn set_slot(repo: &Path, home: &Path, addr: &str, prose: &[u8]) {
    let out = run_doc(
        repo,
        home,
        &["set-slot", addr, "--from-file", "-"],
        Some(prose),
    );
    assert!(
        out.status.success(),
        "set-slot {addr} must succeed; stderr:\n{}",
        String::from_utf8_lossy(&out.stderr)
    );
}

/// Author a `commit` doc with a body and one trailer purely through the verbs, run
/// `finalize`, and assert the finalized commit's `%(trailers)` carries the authored
/// `<Key>: <value>` — the trailer authored via `add-item …#trailers --title <Key>`
/// (heading = key) + `set-field …/value` reaches the git message.
#[test]
fn a_trailer_authored_through_the_verbs_reaches_the_git_message() {
    let (repo, home) = started_repo("add a per-client rate limiter");
    let task = "add-a-per-client-rate";

    // A code change in the working tree (the task's work).
    fs::write(repo.path().join("limiter.rs"), "// rate limiter\n").expect("write code change");
    git(repo.path(), &["add", "limiter.rs"]);

    // The header + subject + body, through the verbs.
    set_field(
        repo.path(),
        home.path(),
        &format!("commit:{task}#type"),
        "feat",
    );
    set_field(
        repo.path(),
        home.path(),
        &format!("commit:{task}#scope"),
        "gateway",
    );
    set_slot(
        repo.path(),
        home.path(),
        &format!("commit:{task}#summary"),
        b"add a per-client rate limiter\n",
    );
    set_slot(
        repo.path(),
        home.path(),
        &format!("commit:{task}#body"),
        b"Throttle abusive clients at the gateway.\n",
    );

    // The trailer: `add-item …#trailers --title Refs` mints the item (heading = key),
    // then `set-field <emitted-addr>/value` fills the value. The emitted item address
    // is captured and fed verbatim (the emitted bytes are the contract).
    let add = run_doc(
        repo.path(),
        home.path(),
        &[
            "add-item",
            &format!("commit:{task}#trailers"),
            "--title",
            "Refs",
        ],
        None,
    );
    assert!(
        add.status.success(),
        "`jigc doc add-item …#trailers --title Refs` must exit 0; stderr:\n{}",
        String::from_utf8_lossy(&add.stderr)
    );
    let item_addr = String::from_utf8(add.stdout)
        .expect("utf-8 stdout")
        .trim_end_matches('\n')
        .to_owned();
    assert!(
        item_addr.starts_with(&format!("commit:{task}#trailers/")),
        "`add-item` emits the minted trailer item address; got {item_addr:?}"
    );
    set_field(
        repo.path(),
        home.path(),
        &format!("{item_addr}/value"),
        "#1242",
    );

    let log_before: u32 = git(repo.path(), &["rev-list", "--count", "HEAD"])
        .parse()
        .unwrap();

    let out = run_task(repo.path(), home.path(), &["finalize", task]);
    assert!(
        out.status.success(),
        "`jigc task finalize` must exit 0; stdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr)
    );

    let log_after: u32 = git(repo.path(), &["rev-list", "--count", "HEAD"])
        .parse()
        .unwrap();
    assert_eq!(
        log_after,
        log_before + 1,
        "finalize makes exactly ONE commit"
    );

    // The finalized commit's trailers, as git itself parses them — the round-trip
    // contract: an `add-item …#trailers` authored trailer is in `%(trailers)`.
    let trailers = git(repo.path(), &["log", "-1", "--format=%(trailers)"]);
    assert!(
        trailers.contains("Refs: #1242"),
        "the trailer authored through the verbs must reach the git message's %(trailers); \
         got trailers:\n{trailers}\nfull message:\n{}",
        git(repo.path(), &["log", "-1", "--format=%B"])
    );
}

/// Add a trailer whose key `title` is passed through `set-field`ing the value; returns
/// the emitted item address. Asserts `add-item` exits 0.
fn add_trailer(repo: &Path, home: &Path, task: &str, key: &str, value: &str) {
    let add = run_doc(
        repo,
        home,
        &[
            "add-item",
            &format!("commit:{task}#trailers"),
            "--title",
            key,
        ],
        None,
    );
    assert!(
        add.status.success(),
        "`add-item …#trailers --title {key}` must exit 0; stderr:\n{}",
        String::from_utf8_lossy(&add.stderr)
    );
    let item_addr = String::from_utf8(add.stdout)
        .expect("utf-8 stdout")
        .trim_end_matches('\n')
        .to_owned();
    set_field(repo, home, &format!("{item_addr}/value"), value);
}

/// The **commit-trailer key-shape rule at the add-item write door** (M45 inc-4 / T4):
/// a trailer key bearing internal whitespace (`BREAKING CHANGE`) would render as
/// `BREAKING CHANGE: <value>` and break git's `%(trailers)` block, so `add-item`
/// rejects it at the point of the mistake, naming the shared
/// `schema-conformance.field-value-conformant` (the named enum seam is blind to it —
/// the slug `breaking-change` is a well-shaped token). Driven over the real binary.
#[test]
fn a_whitespace_trailer_key_is_rejected_at_add_item() {
    let (repo, home) = started_repo("add a per-client rate limiter");
    let task = "add-a-per-client-rate";

    let blocked = run_doc(
        repo.path(),
        home.path(),
        &[
            "add-item",
            &format!("commit:{task}#trailers"),
            "--title",
            "BREAKING CHANGE",
            "--format",
            "json",
        ],
        None,
    );
    assert!(
        !blocked.status.success(),
        "a whitespace trailer key must block at the add-item verb (non-zero exit); \
         stdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&blocked.stdout),
        String::from_utf8_lossy(&blocked.stderr),
    );
    let stderr = String::from_utf8_lossy(&blocked.stderr);
    let report: serde_json::Value = serde_json::from_str(stderr.trim())
        .unwrap_or_else(|e| panic!("stderr is JSON: {e}; got:\n{stderr}"));
    assert_eq!(
        report["findings"][0]["code"], "schema-conformance.field-value-conformant",
        "the block carries the shared field-value-conformant code; got:\n{stderr}",
    );
}

/// A well-shaped hyphenated trailer key (`Co-Authored-By`) is **accepted** at the
/// write door and reaches the finalized commit's `%(trailers)` — the rule does not
/// over-reject the realistic multi-word-but-hyphenated key (T1's round-trip path,
/// re-proven for the named key in the done-criterion).
#[test]
fn a_hyphenated_trailer_key_is_accepted_and_reaches_the_git_message() {
    let (repo, home) = started_repo("wire the shared cache");
    let task = "wire-the-shared-cache";

    fs::write(repo.path().join("cache.rs"), "// cache\n").expect("write code change");
    git(repo.path(), &["add", "cache.rs"]);

    set_field(
        repo.path(),
        home.path(),
        &format!("commit:{task}#type"),
        "feat",
    );
    set_slot(
        repo.path(),
        home.path(),
        &format!("commit:{task}#summary"),
        b"wire the shared cache\n",
    );
    add_trailer(
        repo.path(),
        home.path(),
        task,
        "Co-Authored-By",
        "Ada <ada@example.com>",
    );

    let out = run_task(repo.path(), home.path(), &["finalize", task]);
    assert!(
        out.status.success(),
        "`jigc task finalize` must exit 0; stdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr)
    );
    let trailers = git(repo.path(), &["log", "-1", "--format=%(trailers)"]);
    assert!(
        trailers.contains("Co-Authored-By: Ada <ada@example.com>"),
        "a well-shaped hyphenated trailer key must reach %(trailers); got:\n{trailers}",
    );
}

/// The **task gate** blocks a whitespace trailer key too (M45 inc-4 / T4). `add-item`
/// refuses the malformed key, so it can only reach the staged doc via a direct
/// (out-of-band) file edit — the honest boundary: an agent can edit the staged
/// `.jigc/tasks/<id>/docs/commit:<slug>.md` directly. Authored a valid `Refs` trailer,
/// then rewrite its heading text to `BREAKING CHANGE` (keeping the `{#refs}` anchor, so
/// `item.title` reads the raw whitespace key), and `task validate` must exit non-zero
/// naming `schema-conformance.field-value-conformant`.
#[test]
fn a_whitespace_trailer_key_is_blocked_at_the_task_gate() {
    let (repo, home) = started_repo("throttle the ingest path");
    let task = "throttle-the-ingest-path";

    set_field(
        repo.path(),
        home.path(),
        &format!("commit:{task}#type"),
        "feat",
    );
    set_slot(
        repo.path(),
        home.path(),
        &format!("commit:{task}#summary"),
        b"throttle the ingest path\n",
    );
    add_trailer(repo.path(), home.path(), task, "Refs", "#42");

    // The gate is clean before the out-of-band corruption.
    let clean = run_task(repo.path(), home.path(), &["validate", task]);
    assert!(
        clean.status.success(),
        "the conformant commit doc must validate clean first; stdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&clean.stdout),
        String::from_utf8_lossy(&clean.stderr),
    );

    // Corrupt the staged doc's trailer heading text to a whitespace key, keeping the
    // frozen `{#refs}` anchor so `item.title` parses to the raw `BREAKING CHANGE`.
    let staged = repo
        .path()
        .join(".jigc")
        .join("tasks")
        .join(task)
        .join("docs")
        .join(format!("commit:{task}.md"));
    let source = fs::read_to_string(&staged).expect("read staged commit doc");
    assert!(
        source.contains("Refs"),
        "the staged doc carries the Refs heading; got:\n{source}"
    );
    let corrupted = source.replacen("Refs", "BREAKING CHANGE", 1);
    fs::write(&staged, corrupted).expect("write corrupted staged doc");

    let out = run_task(repo.path(), home.path(), &["validate", task]);
    assert!(
        !out.status.success(),
        "a whitespace trailer key must block at the task gate (non-zero exit); \
         stdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr),
    );
    let rendered = format!(
        "{}{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr)
    );
    assert!(
        rendered.contains("schema-conformance.field-value-conformant"),
        "the task-gate block must name field-value-conformant; got:\n{rendered}",
    );
}

/// The **commit-trailer key-shape rule at the retitle-item write door** (the
/// confidence-audit wave — sibling-hunt finding 6): as shipped by M45 inc-4 the rule
/// guarded `add-item` and the task gate but not `jigc doc retitle-item`, so a
/// well-shaped trailer could be retitled to `BREAKING CHANGE` at write time and the
/// user only found out later at the gate (bounded — no silent corruption, worse
/// route). The retitle door must refuse both trailer-token violations (internal
/// whitespace · colon) with the same shared
/// `schema-conformance.field-value-conformant` code the add-item door emits, while a
/// well-shaped retitle still passes (no over-rejection). Driven over the real binary.
#[test]
fn a_malformed_trailer_key_is_rejected_at_retitle_item() {
    let (repo, home) = started_repo("guard the retitle door");
    let task = "guard-the-retitle-door";

    // Mint a well-shaped trailer through the wired door; the emitted item address is
    // captured and fed verbatim (the emitted bytes are the contract).
    let add = run_doc(
        repo.path(),
        home.path(),
        &[
            "add-item",
            &format!("commit:{task}#trailers"),
            "--title",
            "Refs",
        ],
        None,
    );
    assert!(
        add.status.success(),
        "`add-item …#trailers --title Refs` must exit 0; stderr:\n{}",
        String::from_utf8_lossy(&add.stderr)
    );
    let item_addr = String::from_utf8(add.stdout)
        .expect("utf-8 stdout")
        .trim_end_matches('\n')
        .to_owned();
    assert!(
        item_addr.starts_with(&format!("commit:{task}#trailers/")),
        "`add-item` emits the minted trailer item address; got {item_addr:?}"
    );

    // Both trailer-token violation arms, refused at the retitle door with the shared
    // finding code — the same refusal shape the add-item door carries.
    for bad in ["BREAKING CHANGE", "Co:lon"] {
        let blocked = run_doc(
            repo.path(),
            home.path(),
            &[
                "retitle-item",
                &item_addr,
                "--title",
                bad,
                "--format",
                "json",
            ],
            None,
        );
        assert!(
            !blocked.status.success(),
            "retitling a commit trailer to {bad:?} must block at the retitle-item verb \
             (non-zero exit); stdout:\n{}\nstderr:\n{}",
            String::from_utf8_lossy(&blocked.stdout),
            String::from_utf8_lossy(&blocked.stderr),
        );
        let stderr = String::from_utf8_lossy(&blocked.stderr);
        let report: serde_json::Value = serde_json::from_str(stderr.trim())
            .unwrap_or_else(|e| panic!("stderr is JSON: {e}; got:\n{stderr}"));
        assert_eq!(
            report["findings"][0]["code"], "schema-conformance.field-value-conformant",
            "the retitle block carries the shared field-value-conformant code (the \
             wired door's code, no new check minted); got:\n{stderr}",
        );
    }

    // A well-shaped retitle still passes — the rule does not over-reject at the new
    // door (the anchor stays frozen; only the heading text changes).
    let ok = run_doc(
        repo.path(),
        home.path(),
        &["retitle-item", &item_addr, "--title", "Fixes"],
        None,
    );
    assert!(
        ok.status.success(),
        "retitling a commit trailer to the well-shaped `Fixes` must exit 0; stderr:\n{}",
        String::from_utf8_lossy(&ok.stderr)
    );
}
