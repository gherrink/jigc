//! Acceptance spine (M21 Increment 4, T1) — `jigc unmanage <path>` drops one managed
//! doc from jigc's index/state, the inverse of `ingest`'s register-only `adopt`,
//! end-to-end through the built binary.
//!
//! Drives the real `jigc` binary against a throwaway git repo: `setup` → seed a
//! conformant `adr` carrying a `supersedes` edge → `ingest` (which adopts it: index +
//! baseline gain it) → `jigc unmanage decisions/<slug>.md`, then asserts:
//!
//!   (a) the verb exits 0,
//!   (b) the path is gone from `.jigc/state/file-state.json`,
//!   (c) the doc's forward edges are gone from `.jigc/index/edges.json`,
//!   (d) the managed doc file's on-disk bytes are byte-identical to before, and
//!   (e) a second `jigc unmanage` exits 0 as a clean no-op (record + index unchanged).
//!
//! See `design/project-setup.md` → Flow 2 hardening → Teardown / cleanup (G5).
//!
//! No external test crates: the binary path comes from `CARGO_BIN_EXE_jigc`, the temp
//! repo is a real `git init`, and a self-cleaning `TempDir` keeps the test off the
//! developer's repo.

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

/// A throwaway directory that removes itself on drop.
struct TempDir(PathBuf);

impl TempDir {
    fn new(tag: &str) -> Self {
        let mut path = std::env::temp_dir();
        let unique = format!(
            "jigc-unmanage-{tag}-{}-{:?}",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos(),
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

/// A conformant ADR carrying a `supersedes` ref — the adopt fixture whose forward edge
/// must enter the index on adoption (and must vanish on un-manage).
const CONFORMANT_ADR: &str = "\
---
status: accepted
date: 2026-05-23
supersedes: adr:naive-throttle
---

# Rate limiting

## Context
The gateway must shed load under burst traffic.

## Decision
A token bucket per client keeps the gateway fair under burst.

## Consequences
A misbehaving client is throttled, not the whole gateway.
";

#[test]
fn unmanage_drops_one_doc_from_index_and_state_leaving_bytes_and_is_idempotent() {
    let repo = TempDir::new("drop");
    let home = TempDir::new("home");
    init_repo(repo.path());

    // ── on-ramp: wire jigc into the fresh repo ───────────────────────────────────
    let out = jigc(repo.path(), home.path(), &["setup"]);
    assert!(
        out.status.success(),
        "`jigc setup` must succeed; stderr:\n{}",
        String::from_utf8_lossy(&out.stderr),
    );

    // ── seed a conformant adr and adopt it via ingest ────────────────────────────
    let rel = "decisions/rate-limit.md";
    let doc_path = repo.path().join(rel);
    fs::create_dir_all(doc_path.parent().unwrap()).expect("mk decisions/");
    fs::write(&doc_path, CONFORMANT_ADR).expect("write adr");

    let out = jigc(repo.path(), home.path(), &["ingest"]);
    assert!(
        out.status.success(),
        "`jigc ingest` must succeed; stdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr),
    );

    // Precondition: the adopt landed — the path is baselined + its edge is indexed.
    let file_state_path = repo.path().join(".jigc/state/file-state.json");
    let edges_path = repo.path().join(".jigc/index/edges.json");
    let baseline_before =
        fs::read_to_string(&file_state_path).expect("file-state record persisted by adopt");
    let edges_before = fs::read_to_string(&edges_path).expect("edge index persisted by adopt");
    assert!(
        baseline_before.contains(rel),
        "precondition: the adopted doc is baselined:\n{baseline_before}",
    );
    assert!(
        edges_before.contains("\"from\": \"adr:rate-limit\""),
        "precondition: the adopted doc's forward edge is indexed:\n{edges_before}",
    );

    // Snapshot the managed doc's on-disk bytes — un-manage must leave them untouched.
    let doc_bytes_before = fs::read(&doc_path).expect("read doc before un-manage");

    // ── un-manage the doc ────────────────────────────────────────────────────────
    let out = jigc(repo.path(), home.path(), &["unmanage", rel]);
    assert!(
        out.status.success(),
        "(a) `jigc unmanage` must exit 0; stdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr),
    );

    // (b) the path is gone from file-state.json.
    let baseline_after = fs::read_to_string(&file_state_path).expect("file-state still readable");
    assert!(
        !baseline_after.contains(rel),
        "(b) the un-managed path must be gone from file-state.json:\n{baseline_after}",
    );

    // (c) the doc's forward edges are gone from edges.json.
    let edges_after = fs::read_to_string(&edges_path).expect("edge index still readable");
    assert!(
        !edges_after.contains("\"from\": \"adr:rate-limit\""),
        "(c) the un-managed doc's forward edge must be gone from edges.json:\n{edges_after}",
    );

    // (d) the managed doc file's on-disk bytes are byte-identical to before.
    let doc_bytes_after = fs::read(&doc_path).expect("read doc after un-manage");
    assert_eq!(
        doc_bytes_before, doc_bytes_after,
        "(d) un-manage must leave the managed doc's bytes on disk byte-identical",
    );

    // ── (e) a second un-manage is a clean no-op ──────────────────────────────────
    let baseline_post = fs::read_to_string(&file_state_path).expect("file-state readable");
    let edges_post = fs::read_to_string(&edges_path).expect("edge index readable");

    let out = jigc(repo.path(), home.path(), &["unmanage", rel]);
    assert!(
        out.status.success(),
        "(e) a second `jigc unmanage` must exit 0 (clean no-op); stdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr),
    );

    // The no-op leaves both surfaces byte-identical to the post-drop state.
    assert_eq!(
        fs::read_to_string(&file_state_path).expect("file-state readable"),
        baseline_post,
        "(e) a no-op un-manage leaves file-state.json byte-identical",
    );
    assert_eq!(
        fs::read_to_string(&edges_path).expect("edge index readable"),
        edges_post,
        "(e) a no-op un-manage leaves edges.json byte-identical",
    );
}
