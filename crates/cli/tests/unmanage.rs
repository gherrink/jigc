//! Acceptance spine (M21 Increment 4, T1) — `jigc unmanage <path>` drops one managed
//! doc from jigc's index/state, the inverse of `ingest`'s register-only `adopt`,
//! end-to-end through the built binary.
//!
//! Drives the real `jigc` binary against a throwaway git repo: `setup` → seed a
//! conformant `adr` carrying a `supersedes` edge → `ingest` (which adopts it: index +
//! baseline gain it) → `jigc unmanage docs/decisions/<slug>.md`, then asserts:
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

## Options
Alternatives were weighed and rejected.

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
    let rel = "docs/decisions/rate-limit.md";
    let doc_path = repo.path().join(rel);
    fs::create_dir_all(doc_path.parent().unwrap()).expect("mk docs/decisions/");
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

/// **Round-2 D2+D3+D4 — the operator-surface honesty triple** (the r2a:451-458
/// capture as a test). The `docs-root` relocation sweep walks **committed truth**
/// (`git ls-files` under the prior resolved root) — deliberately index-blind, so a
/// fresh clone still relocates — which means a just-`unmanage`d file at the managed
/// home is still carried by the re-point. That behavior is by design; these are the
/// honesty obligations around it:
///
/// - **D2 (unmanage says so)**: the unmanage report states the file still sits at
///   the managed home and home-wide ops still carry it.
/// - **D2+D3 (the relocation says what it sweeps and what git state it leaves)**:
///   the header names the class (every committed doc under the prior root, managed
///   or not) and the git state (staged `git mv`, not committed) — and the moves
///   really are staged renames with HEAD untouched.
/// - **D4 (`jigc upgrade` names what it checked)**: the clean line is the
///   config-delta noun — "no recorded config deltas" before the set, "1 recorded
///   config delta(s) re-apply clean" after — never the task-scoped wording.
#[test]
fn docs_root_repoint_carries_the_unmanaged_file_and_the_surfaces_say_so() {
    let repo = TempDir::new("repoint-honesty");
    let home = TempDir::new("home");
    init_repo(repo.path());

    let out = jigc(repo.path(), home.path(), &["setup"]);
    assert!(out.status.success(), "`jigc setup` must succeed");

    // Seed a conformant adr, adopt it, and COMMIT it (the relocation sweep walks
    // `git ls-files` — committed truth).
    let rel = "docs/decisions/rate-limit.md";
    let doc_path = repo.path().join(rel);
    fs::create_dir_all(doc_path.parent().unwrap()).expect("mk docs/decisions/");
    fs::write(&doc_path, CONFORMANT_ADR).expect("write adr");
    let out = jigc(repo.path(), home.path(), &["ingest"]);
    assert!(out.status.success(), "`jigc ingest` must succeed");
    git(repo.path(), &["add", "."]);
    git(repo.path(), &["commit", "-q", "-m", "adopt the adr"]);

    // D4, zero-delta arm: the clean line names the config-delta noun, not "the task".
    let out = jigc(repo.path(), home.path(), &["upgrade"]);
    let stdout = String::from_utf8_lossy(&out.stdout).into_owned();
    assert!(out.status.success(), "`jigc upgrade` must exit 0");
    assert!(
        stdout.contains("no recorded config deltas to check"),
        "the zero-delta clean line must say no deltas were recorded; got:\n{stdout}",
    );
    assert!(
        !stdout.contains("the task validates clean"),
        "`jigc upgrade` must not claim a task validated; got:\n{stdout}",
    );

    // D2 (unmanage says so): drop the doc from the index — the report must state it
    // still sits at the managed home and home-wide ops still carry it.
    let out = jigc(repo.path(), home.path(), &["unmanage", rel]);
    let stdout = String::from_utf8_lossy(&out.stdout).into_owned();
    assert!(out.status.success(), "`jigc unmanage` must exit 0");
    assert!(
        stdout.contains("still sits at the managed home"),
        "the unmanage report must state the file is still at the managed home; got:\n{stdout}",
    );

    // D2+D3: the `docs-root` re-point relocates the just-unmanaged committed file
    // too, and the header says the sweep class + the git state of the moves.
    let head_before = {
        let out = std::process::Command::new("git")
            .args(["rev-parse", "HEAD"])
            .current_dir(repo.path())
            .output()
            .expect("git rev-parse");
        String::from_utf8(out.stdout).unwrap().trim().to_string()
    };
    let out = jigc(
        repo.path(),
        home.path(),
        &["config", "set", "docs-root", "docs2"],
    );
    let stderr = String::from_utf8_lossy(&out.stderr).into_owned();
    assert!(
        out.status.success(),
        "`jigc config set docs-root` must exit 0"
    );
    // EC-16 (M51 Increment 9 / T5): the header names the set the code actually walks —
    // `orphan::docs_root_would_orphan`'s stranded set, every committed doc under a
    // doctype's resolved `location:` directory — and states the placement exclusion
    // positively, because a placement doctype's file can sit under the prior resolved
    // root and is never carried.
    assert!(
        stderr.contains("under a doctype's prior resolved `location:` directory, managed or not"),
        "the relocation header must name the set the sweep walks; got:\n{stderr}",
    );
    assert!(
        stderr.contains("a placement doctype's file is not carried"),
        "the relocation header must state the placement exclusion positively; got:\n{stderr}",
    );
    assert!(
        !stderr.contains("every committed doc under the prior resolved root"),
        "the falsified universal must be gone from the header; got:\n{stderr}",
    );
    assert!(
        stderr.contains("staged `git mv`") && stderr.contains("next commit"),
        "the relocation header must state the git state of the moves; got:\n{stderr}",
    );
    assert!(
        stderr.contains("docs/decisions/rate-limit.md → docs2/decisions/rate-limit.md"),
        "the just-unmanaged committed file is still carried by the sweep (the honest, \
         tested behavior); got:\n{stderr}",
    );
    assert!(
        repo.path().join("docs2/decisions/rate-limit.md").is_file() && !doc_path.exists(),
        "the move really happened on disk",
    );

    // The stated git state is true: a staged rename, no commit landed.
    let head_after = {
        let out = std::process::Command::new("git")
            .args(["rev-parse", "HEAD"])
            .current_dir(repo.path())
            .output()
            .expect("git rev-parse");
        String::from_utf8(out.stdout).unwrap().trim().to_string()
    };
    assert_eq!(head_before, head_after, "the relocation commits nothing");
    let status = {
        let out = std::process::Command::new("git")
            .args(["status", "--porcelain"])
            .current_dir(repo.path())
            .output()
            .expect("git status");
        String::from_utf8(out.stdout).unwrap()
    };
    assert!(
        status
            .lines()
            .any(|l| l.starts_with('R') && l.contains("rate-limit.md")),
        "the move is a STAGED rename awaiting the operator's commit; status:\n{status}",
    );

    // D4, recorded-delta arm: the clean line now counts the recorded delta.
    let out = jigc(repo.path(), home.path(), &["upgrade"]);
    let stdout = String::from_utf8_lossy(&out.stdout).into_owned();
    assert!(
        out.status.success(),
        "`jigc upgrade` must exit 0 with the delta"
    );
    assert!(
        stdout.contains("1 recorded config delta(s) re-apply clean against the current pack"),
        "the clean line must count the recorded delta(s); got:\n{stdout}",
    );
}
