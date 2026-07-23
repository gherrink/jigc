//! M45 Increment 6 / T1 — **the milestone-boundary conformance gate**: `jigc milestone
//! finalize` validates the **merged effective state** before it commits, so corruption
//! authored across a fan-out no longer lands at exit 0 (`design/finalize.md` → 2. Validate;
//! `design/validation.md` → The milestone-boundary gate (M45); `DECISIONS.md` →
//! 2026-07-23 M45 Settle, Decision 2).
//!
//! Two classes were live-reproduced at the M45 baseline committing at exit 0 — a
//! non-conformant sub-area doc, and a **cross-worktree** code-anchor dangle no per-area
//! check can see (a merged doc's `doc-code` anchor whose target symbol a *different*
//! worktree removes). Both must now block at exit 3 committing nothing; a clean fan-out
//! still finalizes at exit 0. The whole flow drives the cargo-built `jigc` binary
//! (`CARGO_BIN_EXE_jigc`) against throwaway git repos with the **embedded** dev pack (the
//! flow33 real-binary fan-out idiom), the real `doc-code` probe pointed at via
//! `JIGC_DOC_CODE_PROBE` (the flow13 idiom).

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::OnceLock;

/// A throwaway directory that removes itself on drop.
struct TempDir(PathBuf);

impl TempDir {
    fn new(tag: &str) -> Self {
        let mut path = std::env::temp_dir();
        path.push(format!(
            "jigc-milestone-gate-{tag}-{}-{:?}",
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

/// Run `git <args>` in `dir`, asserting success and returning trimmed stdout.
fn git(dir: &Path, args: &[&str]) -> String {
    let out = Command::new("git")
        .args(args)
        .current_dir(dir)
        .output()
        .expect("run git");
    assert!(
        out.status.success(),
        "git {args:?} in {dir:?} failed: {}",
        String::from_utf8_lossy(&out.stderr)
    );
    String::from_utf8(out.stdout)
        .expect("utf-8 git stdout")
        .trim_end_matches('\n')
        .to_string()
}

/// Initialize a real git repo with one commit. `extra` files are committed into the base
/// too (so a sub-agent worktree can rewrite a base-tracked code file).
fn init_repo(root: &Path, extra: &[(&str, &str)]) {
    git(root, &["init", "-q"]);
    git(root, &["config", "user.email", "test@example.com"]);
    git(root, &["config", "user.name", "Test"]);
    fs::write(root.join("README.md"), "hello\n").expect("write README.md");
    for (rel, body) in extra {
        let p = root.join(rel);
        if let Some(parent) = p.parent() {
            fs::create_dir_all(parent).expect("mkdir extra base file parent");
        }
        fs::write(p, body).expect("write extra base file");
    }
    git(root, &["add", "."]);
    git(root, &["commit", "-q", "-m", "initial"]);
}

/// Build the pack's **real** `doc-code` probe once (process-wide) and return its path — the
/// tree-sitter subprocess the cross-worktree anchor arm drives (the flow13 idiom).
fn real_doc_code_probe() -> &'static Path {
    static PROBE: OnceLock<PathBuf> = OnceLock::new();
    PROBE.get_or_init(|| {
        let manifest = Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("probes")
            .join("doc-code")
            .join("Cargo.toml");
        let out = Command::new(env!("CARGO"))
            .args(["build", "--quiet", "--manifest-path"])
            .arg(&manifest)
            .output()
            .expect("invoke cargo build for doc-code");
        assert!(
            out.status.success(),
            "building the doc-code probe failed:\n{}",
            String::from_utf8_lossy(&out.stderr),
        );
        let bin = manifest
            .parent()
            .unwrap()
            .join("target")
            .join("debug")
            .join("doc-code");
        assert!(bin.is_file(), "doc-code binary missing at {bin:?}");
        bin
    })
}

/// Run `jigc milestone <args>` with `cwd = repo`, `$HOME = home`, the embedded pack, and the
/// real `doc-code` probe (so the merged-state `doc-code` arm resolves anchors for real).
fn run_milestone(repo: &Path, home: &Path, args: &[&str]) -> std::process::Output {
    let mut command = Command::new(env!("CARGO_BIN_EXE_jigc"));
    command.arg("milestone").args(args);
    command
        .current_dir(repo)
        .env("HOME", home)
        .env("JIGC_DOC_CODE_PROBE", real_doc_code_probe())
        .output()
        .expect("run the jigc binary")
}

/// `jigc --format json milestone <args>` — the pinned findings envelope (carrying the
/// finding **codes**) rides stdout on a block, so a blocking arm can assert on the family.
fn run_milestone_json(repo: &Path, home: &Path, args: &[&str]) -> std::process::Output {
    let mut command = Command::new(env!("CARGO_BIN_EXE_jigc"));
    command.args(["--format", "json", "milestone"]).args(args);
    command
        .current_dir(repo)
        .env("HOME", home)
        .env("JIGC_DOC_CODE_PROBE", real_doc_code_probe())
        .output()
        .expect("run the jigc binary")
}

/// Assert a `jigc milestone` invocation succeeded, surfacing both streams on failure.
fn expect_ok(out: &std::process::Output, what: &str) {
    assert!(
        out.status.success(),
        "{what} must exit 0; got {:?}\nstdout:\n{}\nstderr:\n{}",
        out.status,
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr),
    );
}

/// A conformant plain ADR body (no refs, no anchor) — distinct slug per area. Carries every
/// required section (`## Options` is optional but positional, so it is present here — the
/// flow33 conformant body).
fn adr_plain(title: &str) -> String {
    format!(
        "---\nstatus: accepted\ndate: 2026-06-04\n---\n\n# {title}\n\n## Context\n\nForces.\n\n## Options\n\nAlternatives were weighed and rejected.\n\n## Decision\n\nDo the thing.\n\n## Consequences\n\nTradeoffs.\n"
    )
}

/// A **non-conformant** ADR body — the required `## Decision` section is missing, so
/// `schema-conformance.*` blocks over the merged state.
fn adr_missing_decision(title: &str) -> String {
    format!(
        "---\nstatus: accepted\ndate: 2026-06-04\n---\n\n# {title}\n\n## Context\n\nForces.\n\n## Options\n\nAlternatives were weighed and rejected.\n\n## Consequences\n\nTradeoffs.\n"
    )
}

/// A conformant ADR body carrying a `cites-code` anchor into a code symbol — the
/// cross-worktree dangle probe (a *different* worktree removes the symbol).
fn adr_citing(title: &str, anchor: &str) -> String {
    format!(
        "---\nstatus: accepted\ndate: 2026-06-04\ncites-code: {anchor}\n---\n\n# {title}\n\n## Context\n\nForces.\n\n## Options\n\nAlternatives were weighed and rejected.\n\n## Decision\n\nDo the thing.\n\n## Consequences\n\nTradeoffs.\n"
    )
}

/// Stage a doc `body` into a milestone sub-task's `docs/` area (`.jigc/tasks/<sub>/docs/`)
/// with `provenance`, exactly as the staging primitives would (mirrors flow33's `stage_doc`).
fn stage_doc(repo: &Path, sub: &str, address: &str, body: &str, provenance: &str) {
    let docs = repo.join(".jigc").join("tasks").join(sub).join("docs");
    fs::create_dir_all(&docs).expect("mk docs/");
    fs::write(docs.join(format!("{address}.md")), body).expect("write staged body");
    let manifest = docs.join("provenance.json");
    let mut record: serde_json::Value = match fs::read_to_string(&manifest) {
        Ok(s) => serde_json::from_str(&s).expect("provenance manifest parses"),
        Err(_) => serde_json::json!({ "docs": {} }),
    };
    record["docs"][address] = serde_json::Value::String(provenance.to_string());
    fs::write(
        &manifest,
        serde_json::to_string_pretty(&record).expect("serialize manifest"),
    )
    .expect("write provenance manifest");
}

/// Write + `git add` a code file IN a provisioned fan-out worktree
/// (`.jigc/worktrees/<sub>/<rel>`) — the staged code a fanned-out sub-agent produces in its
/// isolated worktree (mirrors flow33's `stage_worktree_code`).
fn stage_worktree_code(repo: &Path, sub: &str, rel: &str, body: &str) {
    let wt = repo.join(".jigc").join("worktrees").join(sub);
    let p = wt.join(rel);
    if let Some(parent) = p.parent() {
        fs::create_dir_all(parent).expect("mkdir worktree code parent");
    }
    fs::write(&p, body).expect("write worktree code");
    git(&wt, &["add", rel]);
}

/// HEAD commit count — the no-commit witness the blocking arms assert is unchanged.
fn head_count(repo: &Path) -> u32 {
    git(repo, &["rev-list", "--count", "HEAD"]).parse().unwrap()
}

/// Any leaked worktree the gate's throwaway checkout would have left — the `.combine-*`
/// dirs `.jigc/worktrees/` gains at gate time must be gone (torn down on both exits).
fn combine_worktree_dirs(repo: &Path) -> Vec<PathBuf> {
    let dir = repo.join(".jigc").join("worktrees");
    let Ok(entries) = fs::read_dir(&dir) else {
        return Vec::new();
    };
    entries
        .filter_map(|e| e.ok())
        .map(|e| e.path())
        .filter(|p| {
            p.file_name()
                .and_then(|n| n.to_str())
                .is_some_and(|n| n.starts_with(".combine-"))
        })
        .collect()
}

/// Stand up a two-sub-task fan-out milestone (`cache-rework`) with the sub-tasks
/// `doc-area` + `code-area`, run `create` + `add-task` ×2, then `provision`. The caller
/// stages the docs before this (join reads the sub-task `docs/` areas) and the worktree
/// code after (the worktrees exist only post-`provision`).
fn create_and_add_tasks(repo: &Path, home: &Path) {
    expect_ok(
        &run_milestone(repo, home, &["create", "Cache rework"]),
        "milestone create",
    );
    // Added in non-id order so id order is not an accident of insertion.
    for intent in ["Doc area", "Code area"] {
        expect_ok(
            &run_milestone(repo, home, &["add-task", "cache-rework", intent]),
            "milestone add-task",
        );
    }
}

/// **(regression) A clean fan-out milestone still finalizes at exit 0.** Two sub-tasks
/// stage disjoint conformant ADRs + disjoint code — the merged state conforms, so the gate
/// is inert and the boundary commits exactly one aggregate.
#[test]
fn clean_fanout_finalizes_at_exit_0() {
    let repo = TempDir::new("clean");
    let home = TempDir::new("clean-home");
    init_repo(repo.path(), &[]);
    create_and_add_tasks(repo.path(), home.path());

    stage_doc(
        repo.path(),
        "doc-area",
        "adr:doc-policy",
        &adr_plain("Doc policy"),
        "edited-from-base",
    );
    stage_doc(
        repo.path(),
        "code-area",
        "adr:code-policy",
        &adr_plain("Code policy"),
        "edited-from-base",
    );
    expect_ok(
        &run_milestone(repo.path(), home.path(), &["provision", "cache-rework"]),
        "milestone provision",
    );
    stage_worktree_code(
        repo.path(),
        "doc-area",
        "src/alpha.rs",
        "pub fn alpha() {}\n",
    );
    stage_worktree_code(
        repo.path(),
        "code-area",
        "src/beta.rs",
        "pub fn beta() {}\n",
    );

    let before = head_count(repo.path());
    let out = run_milestone(repo.path(), home.path(), &["finalize", "cache-rework"]);
    expect_ok(&out, "clean milestone finalize");
    assert_eq!(
        head_count(repo.path()),
        before + 1,
        "a clean fan-out lands exactly one aggregate commit"
    );
    assert!(
        combine_worktree_dirs(repo.path()).is_empty(),
        "the gate's throwaway checkout is torn down"
    );
}

/// **(a) A non-conformant sub-area doc blocks at exit 3, committing nothing.** One area
/// stages an ADR missing its required `## Decision` section — a blocking
/// `schema-conformance.*` finding over the merged state.
#[test]
fn nonconformant_sub_area_doc_blocks_at_exit_3() {
    let repo = TempDir::new("conformance");
    let home = TempDir::new("conformance-home");
    init_repo(repo.path(), &[]);
    create_and_add_tasks(repo.path(), home.path());

    stage_doc(
        repo.path(),
        "doc-area",
        "adr:broken-policy",
        &adr_missing_decision("Broken policy"),
        "edited-from-base",
    );
    stage_doc(
        repo.path(),
        "code-area",
        "adr:code-policy",
        &adr_plain("Code policy"),
        "edited-from-base",
    );
    expect_ok(
        &run_milestone(repo.path(), home.path(), &["provision", "cache-rework"]),
        "milestone provision",
    );
    stage_worktree_code(
        repo.path(),
        "code-area",
        "src/beta.rs",
        "pub fn beta() {}\n",
    );

    let before = head_count(repo.path());
    let out = run_milestone_json(repo.path(), home.path(), &["finalize", "cache-rework"]);
    let output =
        String::from_utf8_lossy(&out.stdout).to_string() + &String::from_utf8_lossy(&out.stderr);
    assert_eq!(
        out.status.code(),
        Some(3),
        "a non-conformant merged doc must block finalize at exit 3; got {:?}\n{output}",
        out.status.code(),
    );
    // The `schema-conformance.*` family surfaces under the `conformance.*` probe codes
    // (`conformance.section-missing` / `.section-renamed`) — the merged doc's missing
    // required section blocks.
    assert!(
        output.contains("conformance.section"),
        "the block names the conformance family; got:\n{output}",
    );
    assert_eq!(
        head_count(repo.path()),
        before,
        "a blocked milestone finalize commits nothing"
    );
    assert!(
        combine_worktree_dirs(repo.path()).is_empty(),
        "the gate's throwaway checkout is torn down on the blocked exit"
    );
}

/// **(b) A cross-worktree code-anchor dangle blocks at exit 3, committing nothing.** A
/// merged ADR (`doc-area`) cites `src/target.rs#target`; a **different** worktree
/// (`code-area`) rewrites `src/target.rs`, removing the symbol — invisible to any per-area
/// check, visible only over the merged code tree. The gate's throwaway detached worktree is
/// torn down on the blocked exit.
#[test]
fn cross_worktree_anchor_dangle_blocks_at_exit_3() {
    let repo = TempDir::new("anchor");
    let home = TempDir::new("anchor-home");
    // The base carries the cited symbol — it resolves at base and dangles only at the
    // merged tree (the newly-dangled discriminator).
    init_repo(repo.path(), &[("src/target.rs", "pub fn target() {}\n")]);
    create_and_add_tasks(repo.path(), home.path());

    stage_doc(
        repo.path(),
        "doc-area",
        "adr:cited-decision",
        &adr_citing("Cited decision", "src/target.rs#target"),
        "edited-from-base",
    );
    expect_ok(
        &run_milestone(repo.path(), home.path(), &["provision", "cache-rework"]),
        "milestone provision",
    );
    // A DIFFERENT worktree removes the cited symbol (rewrites the file without `target`).
    stage_worktree_code(
        repo.path(),
        "code-area",
        "src/target.rs",
        "pub fn other() {}\n",
    );

    let before = head_count(repo.path());
    let out = run_milestone_json(repo.path(), home.path(), &["finalize", "cache-rework"]);
    let output =
        String::from_utf8_lossy(&out.stdout).to_string() + &String::from_utf8_lossy(&out.stderr);
    assert_eq!(
        out.status.code(),
        Some(3),
        "a cross-worktree anchor dangle must block finalize at exit 3; got {:?}\n{output}",
        out.status.code(),
    );
    assert!(
        output.contains("doc-code") && output.contains("src/target.rs#target"),
        "the block names the dangling anchor; got:\n{output}",
    );
    assert_eq!(
        head_count(repo.path()),
        before,
        "a blocked milestone finalize commits nothing"
    );
    assert!(
        combine_worktree_dirs(repo.path()).is_empty(),
        "the gate's throwaway detached worktree is torn down on the blocked exit"
    );
}
