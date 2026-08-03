//! M47 Increment 3 / T5 — **the record pathspec joins the captured-pre-image discipline as
//! the fifth axis** (`DECISIONS.md` → 2026-07-26 M47 Increment 3 halt resolution, call (d);
//! `design/finalize.md` → Rollback discipline).
//!
//! Both fan-out arms reach ONE seam — `overlay_docs_commit_and_ff` — which, after building
//! the aggregate commit off the live checkout, `git add`s jigc's own contributions into the
//! **LIVE index** so the following `git merge --ff-only` sees them as matching. The milestone
//! record's pathspec rides that `git add`. But the executor's failure rollback captures only
//! the promotions, the owner-artifacts, and the config layer — the record path is in **none**
//! of them — and `RecordFlipGuard` restores only the record's *worktree* bytes.
//!
//! So a boundary that refuses **after** that stage left a **`joined` blob staged for a
//! milestone that never finalized**, with the worktree file back at `active`: a subsequent
//! plain `git commit` would land a lying record, and the staged residue is a live
//! counter-example to Increment 2's shipped *"no staged residue"* clause.
//!
//! **The driving cause is the hookless one** — ordinary untracked WIP in the main checkout at
//! a path the fan-out commits, so `git merge --ff-only` refuses (the carry-or-refuse
//! contract). It is the only cause that reaches the live-index `git add` *before* failing:
//! a hook rejection aborts inside the dedicated worktree, upstream of it. **The axis this
//! iterates is the seam's two callers** — `combine_commit` (`squash: true`) and `chain_commit`
//! (`squash: false`) — because the defect is in the shared seam and a fix proven on one arm
//! proves nothing about the other.
//!
//! Per arm: the finalize exits non-zero, `git show :<record>` is byte-identical to the
//! pre-finalize index entry (and **absent stays absent**), `git diff --cached --name-only`
//! names no record path, the record's worktree bytes are back at `active`, and a subsequent
//! **plain `git commit`** cannot land a `joined` record.

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

/// A throwaway directory that removes itself on drop.
struct TempDir(PathBuf);

impl TempDir {
    fn new(tag: &str) -> Self {
        let mut path = std::env::temp_dir();
        path.push(format!(
            "jigc-record-ff-rollback-{tag}-{}-{:?}",
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

/// Run `git <args>` in `cwd`, asserting success; **raw** stdout (never trimmed — the
/// index-entry assertion is a byte comparison).
fn git(cwd: &Path, args: &[&str]) -> String {
    let out = Command::new("git")
        .args(args)
        .current_dir(cwd)
        .output()
        .expect("run git");
    assert!(
        out.status.success(),
        "git {args:?} in {cwd:?} failed: {}",
        String::from_utf8_lossy(&out.stderr),
    );
    String::from_utf8(out.stdout).expect("utf-8 git stdout")
}

/// Run `git <args>` in `cwd` **without** asserting success — the "absent stays absent"
/// probes (`git show :<path>` is fatal when the path is not in the index).
fn git_try(cwd: &Path, args: &[&str]) -> Option<Vec<u8>> {
    let out = Command::new("git")
        .args(args)
        .current_dir(cwd)
        .output()
        .expect("run git");
    out.status.success().then_some(out.stdout)
}

/// Initialize a real git repo with one commit (the milestone mint reads HEAD).
fn init_repo(repo: &Path) {
    git(repo, &["init", "-q"]);
    git(repo, &["config", "user.email", "test@example.com"]);
    git(repo, &["config", "user.name", "Test"]);
    fs::write(repo.join("README.md"), "hello\n").expect("write README");
    git(repo, &["add", "README.md"]);
    git(repo, &["commit", "-q", "-m", "initial"]);
}

/// Write the `[dev ▸ methodology]` compose marker and commit it — the composition that
/// resolves the `milestone-record` doctype (dev-only ships no record, so no pathspec rides
/// the live `git add` and the axis is inert).
fn write_compose_marker(repo: &Path) {
    fs::create_dir_all(repo.join(".jigc").join("config")).expect("mk project config");
    fs::write(
        repo.join(".jigc").join("config").join("packs.yaml"),
        "compose-embedded-methodology: true\n",
    )
    .expect("write compose marker");
    git(repo, &["add", "."]);
    git(repo, &["commit", "-q", "-m", "compose marker"]);
}

/// `.jigc/config/manifest.yaml` opting the project into per-sub-task commits
/// (`squash: false`).
fn set_squash_false(repo: &Path) {
    fs::write(
        repo.join(".jigc")
            .join("config")
            .join("manifest.yaml")
            .as_path(),
        "scalar:\n  finalize.fan-out.squash: false\n",
    )
    .expect("write manifest");
}

/// Run `jigc <args>` with `cwd = repo`, `$HOME = home`, against the embedded packs (never
/// inheriting a harness `JIGC_PACK_DIR` — the compose-marker path requires it ABSENT).
fn jigc(repo: &Path, home: &Path, args: &[&str]) -> std::process::Output {
    Command::new(env!("CARGO_BIN_EXE_jigc"))
        .args(args)
        .current_dir(repo)
        .env("HOME", home)
        .env_remove("JIGC_PACK_DIR")
        .output()
        .expect("run the jigc binary")
}

/// Run `jigc <args>`, asserting exit 0.
fn ok(repo: &Path, home: &Path, args: &[&str], what: &str) {
    let out = jigc(repo, home, args);
    assert!(
        out.status.success(),
        "`{what}` must exit 0; stdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr),
    );
}

const MILESTONE_TITLE: &str = "Cache rework";
const MILESTONE_ID: &str = "cache-rework";

/// The committed record's repo-relative path (docs-root-nested by the dev `docs-root` knob
/// the composition resolves).
const RECORD_SPEC: &str = "docs/milestone-records/cache-rework.md";

/// Stage a doc body + its provenance bit into a sub-task's `tasks/<sub>/docs/` area.
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

/// A plain, ref-free ADR body — the merged doc each sub-task promotes.
fn adr_plain(title: &str) -> String {
    format!(
        "---\nstatus: accepted\ndate: 2026-06-04\n---\n\n# {title}\n\n## Context\n\nForces.\n\n## Options\n\nAlternatives were weighed and rejected.\n\n## Decision\n\nDo the thing.\n\n## Consequences\n\nTradeoffs.\n"
    )
}

/// Stage a sub-task's authored `commit:<sub>` doc — the prose the per-sub-task render reads.
fn stage_subtask_commit(repo: &Path, sub: &str, summary: &str) {
    let body = format!(
        "---\ntype: feat\n---\n\n# {sub}\n\n## Summary\n\n{summary}\n\n## Body\n\n\n\n## Trailers\n"
    );
    stage_doc(repo, sub, &format!("commit:{sub}"), &body);
}

/// Write + `git add` a code file **in** a provisioned fan-out worktree.
fn stage_worktree_code(repo: &Path, sub: &str, rel: &str, body: &str) {
    let wt = repo.join(".jigc").join("worktrees").join(sub);
    let p = wt.join(rel);
    if let Some(parent) = p.parent() {
        fs::create_dir_all(parent).expect("mkdir worktree code parent");
    }
    fs::write(&p, body).expect("write worktree code");
    git(&wt, &["add", rel]);
}

/// The `[dev ▸ methodology]` fan-out fixture: a milestone with a **committed record**, two
/// sub-tasks each holding a disjoint persisted ADR + its authored commit doc, and a
/// provisioned worktree holding that sub-task's staged code.
fn setup_fanout(repo: &Path, home: &Path, squash: bool) {
    init_repo(repo);
    write_compose_marker(repo);
    if !squash {
        set_squash_false(repo);
    }
    ok(
        repo,
        home,
        &["milestone", "create", MILESTONE_TITLE],
        "milestone create",
    );
    for intent in ["Area zed", "Area low"] {
        ok(
            repo,
            home,
            &["milestone", "add-task", MILESTONE_ID, intent],
            "milestone add-task",
        );
    }
    stage_doc(repo, "area-low", "adr:low-policy", &adr_plain("Low policy"));
    stage_subtask_commit(repo, "area-low", "rework the low cache path");
    stage_doc(repo, "area-zed", "adr:zed-policy", &adr_plain("Zed policy"));
    stage_subtask_commit(repo, "area-zed", "rework the zed cache path");
    ok(
        repo,
        home,
        &["milestone", "provision", MILESTONE_ID],
        "milestone provision",
    );
    stage_worktree_code(repo, "area-low", "src/low.rs", "pub fn low() {}\n");
    stage_worktree_code(repo, "area-zed", "src/zed.rs", "pub fn zed() {}\n");
}

/// Seed ordinary **untracked** WIP in the main checkout at a path the fan-out commits, so
/// `git merge --ff-only` refuses to overwrite it — the hookless rejection cause, and the
/// only one that reaches the seam's live-index `git add` before failing.
fn seed_colliding_wip(repo: &Path) {
    let p = repo.join("src").join("low.rs");
    fs::create_dir_all(p.parent().expect("src parent")).expect("mk src/");
    fs::write(&p, "// untracked human WIP\n").expect("seed colliding untracked WIP");
}

/// The record's staged blob — `git show :<record>`, `None` when the path is **absent** from
/// the index (the "absent stays absent" half the shared primitive already models).
fn staged_record(repo: &Path) -> Option<Vec<u8>> {
    git_try(repo, &["show", &format!(":{RECORD_SPEC}")])
}

/// The record path's raw index entry (`git ls-files --stage`) — the assertion is on the
/// **entry**, never the weaker "not staged".
fn record_index_entry(repo: &Path) -> String {
    git(repo, &["ls-files", "--stage", "--", RECORD_SPEC])
}

#[test]
fn an_ff_refused_fan_out_finalize_leaves_no_joined_record_staged() {
    // The axis: the two callers of the one seam that stages the record into the LIVE index.
    for squash in [true, false] {
        let label = if squash {
            "squash: true"
        } else {
            "squash: false"
        };
        let repo = TempDir::new(if squash {
            "squash-true"
        } else {
            "squash-false"
        });
        let home = TempDir::new("home");
        setup_fanout(repo.path(), home.path(), squash);
        seed_colliding_wip(repo.path());

        let head_before = git(repo.path(), &["rev-parse", "HEAD"]);
        let record_index_before = record_index_entry(repo.path());
        let record_staged_before = staged_record(repo.path());
        let record_bytes_before =
            fs::read(repo.path().join(RECORD_SPEC)).expect("the committed record exists");
        assert!(
            String::from_utf8_lossy(&record_bytes_before).contains("active"),
            "[{label}] the fixture's record must be `active` before the finalize",
        );

        // (1) The boundary refuses — `git merge --ff-only` will not overwrite the WIP.
        let aborted = jigc(
            repo.path(),
            home.path(),
            &["milestone", "finalize", MILESTONE_ID],
        );
        assert!(
            !aborted.status.success(),
            "[{label}] the finalize must exit non-zero; stdout:\n{}\nstderr:\n{}",
            String::from_utf8_lossy(&aborted.stdout),
            String::from_utf8_lossy(&aborted.stderr),
        );
        assert_eq!(
            git(repo.path(), &["rev-parse", "HEAD"]),
            head_before,
            "[{label}] nothing landed, so HEAD must be unchanged",
        );

        // (2) The record's index entry is byte-identical to the pre-finalize one — the blob
        // AND the raw entry, with "absent" preserved as absent.
        assert_eq!(
            staged_record(repo.path()),
            record_staged_before,
            "[{label}] `git show :{RECORD_SPEC}` must be byte-identical to the pre-finalize \
             index entry after a refused finalize",
        );
        assert_eq!(
            record_index_entry(repo.path()),
            record_index_before,
            "[{label}] the record's raw `git ls-files --stage` entry must be unchanged",
        );

        // (3) No record path is left staged.
        let staged_names = git(repo.path(), &["diff", "--cached", "--name-only"]);
        assert!(
            !staged_names.lines().any(|l| l == RECORD_SPEC),
            "[{label}] a refused finalize must leave no record path staged; \
             `git diff --cached --name-only` gave:\n{staged_names}",
        );

        // (4) The record's worktree bytes are back at `active`.
        let record_bytes_after =
            fs::read(repo.path().join(RECORD_SPEC)).expect("the record survives the refusal");
        assert_eq!(
            record_bytes_after, record_bytes_before,
            "[{label}] the record's worktree bytes must be restored to their pre-flip state",
        );

        // (5) A subsequent PLAIN `git commit` cannot land a `joined` record — the whole point
        // of the axis: the staged residue is what a later ordinary commit would carry.
        let _ = Command::new("git")
            .args(["commit", "-q", "-m", "operator follow-up"])
            .current_dir(repo.path())
            .output()
            .expect("run git commit");
        let committed = git(repo.path(), &["show", &format!("HEAD:{RECORD_SPEC}")]);
        assert_eq!(
            committed.as_bytes(),
            record_bytes_before.as_slice(),
            "[{label}] a plain `git commit` after the refused finalize must not land a \
             `joined` record; HEAD's record reads:\n{committed}",
        );
    }
}
