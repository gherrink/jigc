//! M43 Increment 7 / T1 — **create-or-update for committed non-singletons** through the
//! real binary (`DECISIONS.md` → 2026-07-16 M43 planning: the Settle, review-baked:
//! "`doc create` over a committed **non-singleton** ships copy-in + `existed` ack";
//! `design/command-output-contract.md` §2 → the create-ack `existed` discriminator).
//!
//! Pre-M43 the copy-in branch of `state::create` was gated to `singleton`, so a
//! `doc create adr` whose slug already had a committed instance seeded a **blank**
//! working copy — and the blank Created doc then ambushed the agent at the finalize
//! clobber gate (`finalize.promote-clobber`), the M16 create-or-update record's intent
//! broken for every non-singleton. Now the copy-in is doctype-blind:
//!
//! - (i) a fresh `doc create` acks **`existed: false`** (the key is always present) and
//!   its agent text stays the bare minted address;
//! - (ii) a `doc create` over a committed same-slug adr **copies the committed body in**
//!   (`existed: true` on JSON; the agent text says so), the working copy carries the
//!   committed prose, and an edited re-promote lands as an ordinary **`M`** — no
//!   `finalize.promote-clobber`;
//! - (iii) the guard's remaining reachable set — **post-create drift** (a file appears at
//!   the destination only *after* the Created mint) — still blocks.

use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

/// A throwaway directory that removes itself on drop.
struct TempDir(PathBuf);

impl TempDir {
    fn new(tag: &str) -> Self {
        let mut path = std::env::temp_dir();
        path.push(format!(
            "jigc-create-or-update-{tag}-{}-{:?}",
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

/// Run a `git` command in `repo`, asserting success.
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
        .trim_end_matches('\n')
        .to_string()
}

/// Initialize a real git repo with one commit (a tracked `README.md`).
fn init_repo(repo: &Path) {
    git(repo, &["init", "-q"]);
    git(repo, &["config", "user.email", "test@example.com"]);
    git(repo, &["config", "user.name", "Test"]);
    fs::write(repo.join("README.md"), "hello\n").expect("write README");
    git(repo, &["add", "README.md"]);
    git(repo, &["commit", "-q", "-m", "initial"]);
}

/// Run `jigc <args>` against the embedded packs.
fn jigc(repo: &Path, home: &Path, args: &[&str], stdin: Option<&[u8]>) -> std::process::Output {
    let mut command = Command::new(env!("CARGO_BIN_EXE_jigc"));
    command
        .args(args)
        .current_dir(repo)
        .env("HOME", home)
        .env_remove("JIGC_PACK_DIR")
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());
    if stdin.is_some() {
        command.stdin(Stdio::piped());
    }
    let mut child = command.spawn().expect("spawn the jigc binary");
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

/// Run `jigc <args>` (no stdin), asserting exit 0, returning stdout.
fn ok(repo: &Path, home: &Path, args: &[&str], what: &str) -> String {
    let out = jigc(repo, home, args, None);
    assert!(
        out.status.success(),
        "`{what}` must exit 0; stdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr),
    );
    String::from_utf8(out.stdout).expect("utf-8 stdout")
}

/// Set one slot from stdin, asserting success.
fn set_slot(repo: &Path, home: &Path, addr: &str, prose: &[u8]) {
    let out = jigc(
        repo,
        home,
        &["doc", "set-slot", addr, "--from-file", "-"],
        Some(prose),
    );
    assert!(
        out.status.success(),
        "set-slot {addr} must succeed; stderr:\n{}",
        String::from_utf8_lossy(&out.stderr),
    );
}

/// Fill every author-required field/slot of the task's provisioned commit doc.
fn fill_commit(repo: &Path, home: &Path, task: &str) {
    ok(
        repo,
        home,
        &[
            "doc",
            "set-field",
            &format!("commit:{task}#type"),
            "--value",
            "docs",
        ],
        "doc set-field commit#type",
    );
    set_slot(
        repo,
        home,
        &format!("commit:{task}#summary"),
        format!("{task}\n").as_bytes(),
    );
}

/// Fill the three author-required adr slots.
fn fill_adr_slots(repo: &Path, home: &Path, slug: &str) {
    set_slot(
        repo,
        home,
        &format!("adr:{slug}#context"),
        b"The cache is cold on every boot.\n",
    );
    set_slot(
        repo,
        home,
        &format!("adr:{slug}#decision"),
        b"Warm it on boot.\n",
    );
    set_slot(
        repo,
        home,
        &format!("adr:{slug}#consequences"),
        b"A slower boot.\n",
    );
}

/// (i) + (ii): a fresh `doc create adr` acks `existed: false`; a second task's
/// `doc create adr` over the now-committed same slug copies the committed body in,
/// acks `existed: true` (JSON) / says so (agent text), and an edited re-promote
/// finalizes as an ordinary `M` — no `finalize.promote-clobber`.
#[test]
fn create_over_a_committed_adr_copies_in_acks_existed_and_lands_m() {
    let repo = TempDir::new("copy-in");
    let home = TempDir::new("home");
    init_repo(repo.path());
    ok(repo.path(), home.path(), &["setup"], "jigc setup");

    // ── Task 1: fresh create → `existed: false` (the key is always present) ─────
    let first = "record-the-cache-decision";
    ok(
        repo.path(),
        home.path(),
        &[
            "start",
            "--workflow",
            "single-task",
            "record the cache decision",
        ],
        "jigc start (first)",
    );
    let stdout = ok(
        repo.path(),
        home.path(),
        &[
            "doc",
            "create",
            "adr",
            "--title",
            "Cache Strategy",
            "--format",
            "json",
        ],
        "jigc doc create adr (fresh, json)",
    );
    let ack: serde_json::Value =
        serde_json::from_str(stdout.trim()).expect("fresh create json ack parses");
    assert_eq!(ack["op"], "create");
    assert_eq!(
        ack["existed"],
        serde_json::json!(false),
        "a fresh create acks `existed: false` — the key is always present; got:\n{stdout}"
    );

    fill_adr_slots(repo.path(), home.path(), "cache-strategy");
    fill_commit(repo.path(), home.path(), first);
    ok(
        repo.path(),
        home.path(),
        &["task", "finalize", first],
        "jigc task finalize (first)",
    );
    let committed = repo.path().join("docs/decisions/cache-strategy.md");
    assert!(committed.is_file(), "the first finalize lands the adr");

    // ── Task 2: create over the committed slug → copy-in + `existed: true` ──────
    let second = "revise-the-cache-decision";
    ok(
        repo.path(),
        home.path(),
        &[
            "start",
            "--workflow",
            "single-task",
            "revise the cache decision",
        ],
        "jigc start (second)",
    );
    let stdout = ok(
        repo.path(),
        home.path(),
        &[
            "doc",
            "create",
            "adr",
            "--title",
            "Cache Strategy",
            "--format",
            "json",
        ],
        "jigc doc create adr (over committed, json)",
    );
    let ack: serde_json::Value =
        serde_json::from_str(stdout.trim()).expect("copy-in create json ack parses");
    assert_eq!(ack["op"], "create");
    assert_eq!(ack["target"]["slug"], "cache-strategy");
    assert_eq!(
        ack["existed"],
        serde_json::json!(true),
        "a create over a committed same-slug doc acks `existed: true`; got:\n{stdout}"
    );

    // The working copy carries the committed body — copied in, never seeded blank.
    let staged = fs::read_to_string(
        repo.path()
            .join(".jigc/tasks")
            .join(second)
            .join("docs")
            .join("adr:cache-strategy.md"),
    )
    .expect("read the staged working copy");
    assert!(
        staged.contains("The cache is cold on every boot."),
        "the working copy carries the committed body (copy-in); got:\n{staged}"
    );

    // Update the decision prose, then finalize: an ordinary re-promote, landing `M`.
    set_slot(
        repo.path(),
        home.path(),
        "adr:cache-strategy#decision",
        b"Warm it lazily on first hit.\n",
    );
    fill_commit(repo.path(), home.path(), second);
    let out = jigc(
        repo.path(),
        home.path(),
        &["task", "finalize", second],
        None,
    );
    let streams = format!(
        "{}{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr),
    );
    assert!(
        out.status.success(),
        "the copy-in re-promote finalizes clean (no clobber ambush); streams:\n{streams}"
    );
    assert!(
        !streams.contains("finalize.promote-clobber"),
        "an edited-from-base re-promote never trips the clobber guard; got:\n{streams}"
    );
    let name_status = git(repo.path(), &["show", "--name-status", "--format=", "HEAD"]);
    assert!(
        name_status.contains("M\tdocs/decisions/cache-strategy.md"),
        "the update lands as `M` on the committed adr; got:\n{name_status}"
    );
    let landed = fs::read_to_string(&committed).expect("read the updated adr");
    assert!(
        landed.contains("Warm it lazily on first hit.")
            && landed.contains("The cache is cold on every boot."),
        "the committed adr carries the update AND the preserved prior prose; got:\n{landed}"
    );

    // ── Task 3: the agent-text surface states the copy-in ───────────────────────
    ok(
        repo.path(),
        home.path(),
        &[
            "start",
            "--workflow",
            "single-task",
            "revisit the cache decision",
        ],
        "jigc start (third)",
    );
    let stdout = ok(
        repo.path(),
        home.path(),
        &["doc", "create", "adr", "--title", "Cache Strategy"],
        "jigc doc create adr (over committed, agent text)",
    );
    assert!(
        stdout.contains("adr:cache-strategy")
            && stdout.contains("already existed")
            && stdout.contains("copied in for update"),
        "the agent text states the copy-in (created vs already-existed, law 1); got:\n{stdout}"
    );
}

/// (iii) The guard's remaining reachable set: **post-create drift**. A doc minted while
/// its destination was empty (`Created` provenance) whose destination gains a file only
/// *after* the mint still blocks at `finalize.promote-clobber` — the copy-in narrows the
/// guard, it does not retire it.
#[test]
fn post_create_drift_still_blocks_at_the_clobber_guard() {
    let repo = TempDir::new("drift");
    let home = TempDir::new("home");
    init_repo(repo.path());
    ok(repo.path(), home.path(), &["setup"], "jigc setup");

    let task = "record-the-eviction-decision";
    ok(
        repo.path(),
        home.path(),
        &[
            "start",
            "--workflow",
            "single-task",
            "record the eviction decision",
        ],
        "jigc start",
    );
    // Minted while `docs/decisions/eviction-policy.md` does not exist → fresh, and the
    // fresh agent text stays the bare minted address (the next address an agent drives).
    let stdout = ok(
        repo.path(),
        home.path(),
        &["doc", "create", "adr", "--title", "Eviction Policy"],
        "jigc doc create adr (fresh, agent text)",
    );
    assert_eq!(
        stdout.trim(),
        "adr:eviction-policy",
        "a fresh create's agent text is the bare minted address, unchanged"
    );
    set_slot(
        repo.path(),
        home.path(),
        "adr:eviction-policy#context",
        b"Memory pressure.\n",
    );
    set_slot(
        repo.path(),
        home.path(),
        "adr:eviction-policy#decision",
        b"Evict LRU.\n",
    );
    set_slot(
        repo.path(),
        home.path(),
        "adr:eviction-policy#consequences",
        b"Cold entries churn.\n",
    );
    fill_commit(repo.path(), home.path(), task);

    // A file appears at the destination only AFTER the Created mint — the drift the
    // guard still owns.
    fs::create_dir_all(repo.path().join("docs/decisions")).expect("mk decisions dir");
    fs::write(
        repo.path().join("docs/decisions/eviction-policy.md"),
        "# Eviction Policy\n\nhand-authored, not jigc's.\n",
    )
    .expect("write the post-create squatter");

    let out = jigc(
        repo.path(),
        home.path(),
        &["task", "finalize", task, "--format", "json"],
        None,
    );
    assert!(
        !out.status.success(),
        "post-create drift must still block at finalize-promote"
    );
    let stdout = String::from_utf8_lossy(&out.stdout);
    let value: serde_json::Value =
        serde_json::from_str(&stdout).expect("the blocked finalize emits the findings envelope");
    let clobber = value["findings"]
        .as_array()
        .expect("findings array")
        .iter()
        .find(|f| f["code"] == "finalize.promote-clobber")
        .unwrap_or_else(|| panic!("a promote-clobber finding rides the envelope; got:\n{stdout}"))
        .clone();
    assert_eq!(
        clobber["key"]["target"], "docs/decisions/eviction-policy.md",
        "the guard keys at the destination it refused to overwrite; got:\n{stdout}"
    );
}
