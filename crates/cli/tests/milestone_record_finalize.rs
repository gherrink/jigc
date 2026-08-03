//! M39 Increment 4 / T4 — `milestone finalize` folds the `join` status-flip into the SINGLE
//! finalize commit (`design/team-ready-state.md` → Engine capability 1 (write), the `join` —
//! in-place-mutate arm; The commit model — join folds). Under a `[dev ▸ methodology]` project
//! the finalize commit boundary flips the committed record's every `tasks` item AND the header
//! `status` active → joined and **path-adds the flipped record into the ONE finalize commit**
//! (never a second commit). The `.jigc/` WIP area is removed; the committed record (outside
//! `.jigc/`) survives.
//!
//! One proof, driving the REAL binary against a throwaway `[dev ▸ methodology]` git repo:
//!
//!   (RED-iii) create → add-task ×2 → **a sub-task merges a doc** → finalize. The committed
//!             record is `joined` on every item + the header, **byte-identical modulo exactly
//!             the flipped `status` values** (the byte-stability obligation of the in-place
//!             flip when folded into the commit), landed in the SINGLE finalize commit; the
//!             `.jigc/milestones/<id>/` WIP area is gone and the record survives on disk.
//!
//! **The merged doc is load-bearing, not decoration (M47 Inc 3 T2).** The fixture used to
//! finalize an *empty* milestone — no docs, no provisioned worktrees — because that was the
//! cheapest way to reach the flip. That is exactly the **zero-contribution** shape now refused
//! (`design/finalize.md` → The zero-contribution refusal): a boundary whose only change is the
//! record's own `joined` flip lands no work and burns the milestone's terminal status. So the
//! fixture stages one real sub-task doc; the flip it proves is the same flip, folded into the
//! same single commit, and every byte-stability assertion below is untouched.

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

/// A throwaway directory that removes itself on drop.
struct TempDir(PathBuf);

impl TempDir {
    fn new(tag: &str) -> Self {
        let mut path = std::env::temp_dir();
        path.push(format!(
            "jigc-milestone-record-finalize-{tag}-{}-{:?}",
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
    String::from_utf8(out.stdout).expect("utf-8 git stdout")
}

/// Initialize a real git repo with one commit (mint reads HEAD via `git rev-parse`).
fn init_repo(repo: &Path) {
    git(repo, &["init", "-q"]);
    git(repo, &["config", "user.email", "test@example.com"]);
    git(repo, &["config", "user.name", "Test"]);
    fs::write(repo.join("README.md"), "hello\n").expect("write file");
    git(repo, &["add", "README.md"]);
    git(repo, &["commit", "-q", "-m", "initial"]);
}

/// Write the `[dev ▸ methodology]` compose marker — the exact key `make_pack` reads to assemble
/// the composition dev-highest (so the `milestone-record` schema + dev `docs-root` apply).
fn write_compose_marker(repo: &Path) {
    fs::create_dir_all(repo.join(".jigc").join("config")).expect("mk project config");
    fs::write(
        repo.join(".jigc").join("config").join("packs.yaml"),
        "compose-embedded-methodology: true\n",
    )
    .expect("write compose marker");
}

/// Run `jigc milestone <args>` with `cwd = repo` and `$HOME = home`, never inheriting a harness
/// `JIGC_PACK_DIR` (the compose-marker path requires it ABSENT, else the env pack supersedes).
fn run_milestone(repo: &Path, home: &Path, args: &[&str]) -> std::process::Output {
    Command::new(env!("CARGO_BIN_EXE_jigc"))
        .arg("milestone")
        .args(args)
        .current_dir(repo)
        .env("HOME", home)
        .env_remove("JIGC_PACK_DIR")
        .output()
        .expect("run the jigc binary")
}

/// Assert a `jigc milestone` invocation exited 0, surfacing stderr on failure.
fn assert_ok(out: &std::process::Output, what: &str) {
    assert!(
        out.status.success(),
        "{what} must exit 0; got {:?}\nstderr:\n{}",
        out.status,
        String::from_utf8_lossy(&out.stderr),
    );
}

/// The number of commits reachable from HEAD.
fn commit_count(repo: &Path) -> u32 {
    git(repo, &["rev-list", "--count", "HEAD"])
        .trim()
        .parse()
        .expect("commit count parses")
}

/// The repo-relative paths touched by a commit (`git diff-tree` name-only over the commit).
fn commit_files(repo: &Path, rev: &str) -> Vec<String> {
    git(
        repo,
        &["diff-tree", "--no-commit-id", "--name-only", "-r", rev],
    )
    .lines()
    .map(str::to_string)
    .collect()
}

/// The committed record path under docs-root for milestone `cache-rework`.
fn record_rel() -> &'static str {
    "docs/milestone-records/cache-rework.md"
}

/// Stage a doc body + its provenance bit into a sub-task's `tasks/<sub>/docs/` area — the
/// managed doc a fanned-out sub-agent authors, and the boundary's real contribution (without
/// one the finalize is the refused zero-contribution shape, M47 Inc 3 T2).
fn stage_doc(repo: &Path, sub: &str, address: &str, body: &str) {
    let docs = repo.join(".jigc").join("tasks").join(sub).join("docs");
    fs::create_dir_all(&docs).expect("mk docs/");
    fs::write(docs.join(format!("{address}.md")), body).expect("write staged body");

    let manifest = docs.join("provenance.json");
    let mut record: serde_json::Value = match fs::read_to_string(&manifest) {
        Ok(s) => serde_json::from_str(&s).expect("provenance manifest parses"),
        Err(_) => serde_json::json!({ "docs": {} }),
    };
    record["docs"][address] = serde_json::Value::String("created".to_string());
    fs::write(
        &manifest,
        serde_json::to_string_pretty(&record).expect("serialize manifest"),
    )
    .expect("write provenance manifest");
}

/// A plain, ref-free ADR body.
fn adr_plain(title: &str) -> String {
    format!(
        "---\nstatus: accepted\ndate: 2026-06-04\n---\n\n# {title}\n\n## Context\n\nForces.\n\n## Options\n\nAlternatives were weighed and rejected.\n\n## Decision\n\nDo the thing.\n\n## Consequences\n\nTradeoffs.\n"
    )
}

/// create → add-task ×2 on a `[dev ▸ methodology]` repo — lands the milestone's own record-only
/// commits (create opens, each add-task appends), leaving every recorded status `active`.
fn setup_milestone(repo: &Path, home: &Path) {
    assert_ok(
        &run_milestone(repo, home, &["create", "Cache rework"]),
        "`[dev ▸ methodology]` `jigc milestone create`",
    );
    assert_ok(
        &run_milestone(
            repo,
            home,
            &["add-task", "cache-rework", "Warm the read cache"],
        ),
        "add-task #1",
    );
    assert_ok(
        &run_milestone(
            repo,
            home,
            &["add-task", "cache-rework", "Evict cold entries"],
        ),
        "add-task #2",
    );
}

/// (RED-iii) create → add-task ×2 → a merged sub-task doc → finalize — the committed record
/// flips to `joined` on every item + the header, byte-identical modulo exactly the flipped
/// `status` values, folded into the SINGLE finalize commit; the `.jigc/` WIP area is removed
/// and the record survives.
#[test]
fn finalize_folds_the_join_status_flip_into_the_single_commit_byte_stable() {
    let repo = TempDir::new("fold");
    let home = TempDir::new("home");
    init_repo(repo.path());
    write_compose_marker(repo.path());
    setup_milestone(repo.path(), home.path());
    // Real work for the boundary to land (M47 Inc 3 T2 — a record flip alone is refused).
    stage_doc(
        repo.path(),
        "warm-the-read-cache",
        "adr:warm-policy",
        &adr_plain("Warm policy"),
    );

    let record_abs = repo.path().join(record_rel());
    // The committed record after add-task ×2 — the base pin + both sub-tasks, every status
    // `active` (header + both items = three `status: active`).
    let before = fs::read_to_string(&record_abs).expect("record before finalize");
    assert_eq!(
        before.matches("status: active").count(),
        3,
        "before finalize: the header + both items are `status: active`; got:\n{before}",
    );
    let pre_count = commit_count(repo.path());

    // --- finalize -----------------------------------------------------------------------
    let out = run_milestone(repo.path(), home.path(), &["finalize", "cache-rework"]);
    assert_ok(&out, "`[dev ▸ methodology]` `jigc milestone finalize`");

    // Exactly ONE finalize commit landed (the flip folds in — never a second commit).
    assert_eq!(
        commit_count(repo.path()),
        pre_count + 1,
        "finalize lands exactly one commit (the join-flip folds into it); stderr:\n{}",
        String::from_utf8_lossy(&out.stderr),
    );
    // The flipped record rode THAT single finalize commit.
    assert!(
        commit_files(repo.path(), "HEAD")
            .iter()
            .any(|p| p == record_rel()),
        "the finalize commit path-adds the flipped record; touched: {:?}",
        commit_files(repo.path(), "HEAD"),
    );

    // The record survives on disk (it is outside `.jigc/` — the WIP-area cleanup must not touch
    // it) and its bytes ARE what landed in the finalize commit.
    let after = fs::read_to_string(&record_abs).expect("record survives finalize");
    let committed = git(repo.path(), &["show", &format!("HEAD:{}", record_rel())]);
    assert_eq!(
        after, committed,
        "the on-disk record matches the bytes committed at the finalize boundary",
    );

    // Byte-stability (the obligation): the joined record is byte-identical to the pre-finalize
    // one modulo EXACTLY the flipped `status` values — every `status: active` (header + both
    // items) becomes `status: joined`, and no other byte moves.
    assert_eq!(
        after,
        before.replace("status: active", "status: joined"),
        "the fold flips only the status values; every other byte survives byte-identical\n\
         --- before ---\n{before}\n--- after ---\n{after}",
    );
    assert!(
        !after.contains("status: active"),
        "no recorded status survives un-flipped after the join fold; got:\n{after}",
    );
    assert_eq!(
        after.matches("status: joined").count(),
        3,
        "the header + both items flipped to joined; got:\n{after}",
    );

    // The gitignored WIP area is gone (the milestone boundary removed it); the committed record
    // is the durable survivor.
    assert!(
        !repo
            .path()
            .join(".jigc")
            .join("milestones")
            .join("cache-rework")
            .exists(),
        "finalize removes the `.jigc/milestones/<id>/` WIP area",
    );
}

/// A **blocked** finalize must not persist the join-flip — the transactional guarantee ("Writes
/// are transactional"): with a foreign (non-record) commit in `base..HEAD` the base-guard blocks
/// the finalize, and the committed record stays `active` (the in-place flip is restored, not left
/// dangling in the working tree). The omitting-context proof for the flip's restore guard.
///
/// **It blocks for its OWN reason (re-verified M47 Inc 3 T2).** The milestone also carries a
/// real merged doc, so the zero-contribution refusal is inert here and the base-guard is what
/// speaks — asserted below rather than assumed, because two blocks that both exit 3 are
/// indistinguishable by exit code alone.
#[test]
fn a_blocked_finalize_restores_the_record_leaving_it_active() {
    let repo = TempDir::new("blocked");
    let home = TempDir::new("home");
    init_repo(repo.path());
    write_compose_marker(repo.path());
    setup_milestone(repo.path(), home.path());
    stage_doc(
        repo.path(),
        "warm-the-read-cache",
        "adr:warm-policy",
        &adr_plain("Warm policy"),
    );

    let record_abs = repo.path().join(record_rel());
    let before = fs::read_to_string(&record_abs).expect("record before the blocked finalize");

    // A foreign, non-record commit — external drift the base-guard must still catch (block).
    fs::write(repo.path().join("src.rs"), "fn main() {}\n").expect("write code file");
    git(repo.path(), &["add", "src.rs"]);
    git(repo.path(), &["commit", "-q", "-m", "foreign code change"]);

    let out = run_milestone(repo.path(), home.path(), &["finalize", "cache-rework"]);
    assert_eq!(
        out.status.code(),
        Some(3),
        "a foreign commit blocks finalize at the base-guard (exit 3); stderr:\n{}",
        String::from_utf8_lossy(&out.stderr),
    );
    let stderr = String::from_utf8_lossy(&out.stderr).to_string();
    assert!(
        stderr.contains("pinned to base") && stderr.contains("more than milestone-record"),
        "the block must be the BASE-GUARD's, not another refusal that also exits 3; got:\n{stderr}",
    );

    // The flip was restored — the working-tree record is byte-identical to its pre-finalize state
    // and still carries every `status: active`, so a blocked finalize left no dangling flip.
    let after = fs::read_to_string(&record_abs).expect("record survives the blocked finalize");
    assert_eq!(
        after, before,
        "a blocked finalize must restore the record to its pre-flip bytes (transactional)",
    );
    assert!(
        !after.contains("status: joined"),
        "a blocked finalize must not persist the join-flip; got:\n{after}",
    );
}
