//! M42 Increment 7 / T5 — `jigc milestone discard <id> [--force]`: the milestone family's
//! terminal verb (`design/team-ready-state.md` → `jigc milestone discard <id>` — the three
//! properties; `design/write-commands.md` → Abandoning a milestone). It settles the committed
//! record to the `discarded` terminal (T1's schema-version-2 enum member, through T4's per-item
//! flip: a **genuinely joined** sub-task stays `joined`), lands ONE record-only commit, and tears
//! the workbench down — the sub-task areas, the registered fan-out worktrees, **and**
//! `.jigc/milestones/<id>/`, which had no reachable remover at all.
//!
//! Four proofs, driving the REAL binary against throwaway git repos:
//!
//!   (RED-i)   **The dirty-worktree refusal.** A file written into a provisioned sub-task
//!             worktree makes `discard` REFUSE (exit non-zero, naming the worktree and the dirty
//!             path); the record, the worktree, and the file are all intact, and no commit lands.
//!             Red today: teardown's `git worktree remove --force` — safe at *finalize*, where
//!             the commit lands first — destroys the file silently at exit 0 on the abandon path,
//!             where the work is by definition uncommitted (the M31 WIP-safety shape).
//!
//!   (RED-ii)  **`--force` settles + tears down.** Exit 0; the committed record's header reads
//!             `status: discarded`, a genuinely joined sub-task still reads `joined`, the active
//!             one reads `discarded`; EXACTLY ONE record-only commit lands (unrelated staged and
//!             untracked WIP in the main checkout untouched); and `.jigc/milestones/<id>/`, the
//!             sub-task areas, and the registered worktrees are all gone.
//!
//!   (RED-iii) **An unknown milestone id routes and removes nothing** — a live milestone's
//!             workbench and record survive a discard aimed at an id that does not exist.
//!
//!   (RED-iv)  **Dev-only degrades** (the omitting context): with no methodology pack there is no
//!             `milestone-record` schema, so `discard` settles no record and lands NO commit —
//!             and still tears the whole workbench down, exit 0.

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

/// A throwaway directory that removes itself on drop.
struct TempDir(PathBuf);

impl TempDir {
    fn new(tag: &str) -> Self {
        let mut path = std::env::temp_dir();
        path.push(format!(
            "jigc-milestone-discard-{tag}-{}-{:?}",
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
        // The fan-out worktrees are ordinary directories under `.jigc/worktrees/` — a plain
        // recursive remove clears them along with the repo.
        let _ = fs::remove_dir_all(&self.0);
    }
}

/// The milestone under test, and its sub-tasks: two that genuinely **joined** (a real
/// `milestone finalize` flipped them) and one still **active** (added after that finalize).
const MILESTONE: &str = "cache-rework";
const JOINED_SUBS: [&str; 2] = ["warm-the-read-cache", "evict-cold-entries"];
const ACTIVE_SUB: &str = "purge-stale-keys";

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

/// Write the `[dev ▸ methodology]` compose marker — the exact key `make_pack` reads, so the
/// composed cascade resolves the methodology-pack `milestone-record` doctype (and dev `docs-root`).
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

/// The repo-relative paths touched by a commit.
fn commit_files(repo: &Path, rev: &str) -> Vec<String> {
    git(
        repo,
        &["diff-tree", "--no-commit-id", "--name-only", "-r", rev],
    )
    .lines()
    .map(str::to_string)
    .collect()
}

/// The committed record path (repo-relative) under the composed docs-root.
fn record_rel() -> String {
    format!("docs/milestone-records/{MILESTONE}.md")
}

/// The committed record's bytes.
fn read_record(repo: &Path) -> String {
    fs::read_to_string(repo.join(record_rel())).expect("read the committed milestone record")
}

/// The gitignored workbench paths the teardown must remove.
fn milestone_area(repo: &Path) -> PathBuf {
    repo.join(".jigc").join("milestones").join(MILESTONE)
}

fn subtask_area(repo: &Path, sub: &str) -> PathBuf {
    repo.join(".jigc").join("tasks").join(sub)
}

fn worktree_dir(repo: &Path, sub: &str) -> PathBuf {
    repo.join(".jigc").join("worktrees").join(sub)
}

/// The repo's registered git worktrees (the `worktree <path>` lines of the porcelain listing),
/// **excluding** the main checkout — exactly the fan-out worktrees `provision` registered.
fn registered_fanout_worktrees(repo: &Path) -> Vec<String> {
    git(repo, &["worktree", "list", "--porcelain"])
        .lines()
        .filter_map(|line| line.strip_prefix("worktree "))
        .filter(|path| path.contains(".jigc/worktrees/"))
        .map(str::to_string)
        .collect()
}

/// The `status` leaf of the record's `meta` header — read from the emitted committed bytes
/// (everything ahead of the H1).
fn header_status(body: &str) -> String {
    let end = body.find("\n# ").expect("the record carries an H1");
    body[..end]
        .lines()
        .find_map(|line| line.strip_prefix("status: "))
        .map(str::to_string)
        .unwrap_or_else(|| panic!("the record header carries a `status` field:\n{body}"))
}

/// The `status` leaf recorded for sub-task `sub` — sliced out of the item's own field block in
/// the emitted committed bytes (from its `{#<id>}` anchor to the next item heading).
fn item_status(body: &str, sub: &str) -> String {
    let anchor = format!("{{#{sub}}}");
    let start = body
        .find(&anchor)
        .unwrap_or_else(|| panic!("the record names sub-task `{sub}`:\n{body}"));
    let rest = &body[start..];
    let end = rest[1..]
        .find("\n### ")
        .map(|i| i + 1)
        .unwrap_or(rest.len());
    rest[..end]
        .lines()
        .find_map(|line| line.trim().strip_prefix("- status: "))
        .map(str::to_string)
        .unwrap_or_else(|| {
            panic!(
                "sub-task `{sub}` carries a `status` leaf:\n{}",
                &rest[..end]
            )
        })
}

/// A **partially-joined** milestone, built entirely through the production verbs on a
/// `[dev ▸ methodology]` repo: create → add-task ×2 → **finalize** (both sub-tasks and the header
/// flip to `joined`) → add-task (a third, `active`) → provision (one worktree per sub-task). The
/// committed record therefore carries exactly what the discard's per-item rule is about — landed
/// work alongside work that never landed — and the workbench (milestone area, the new sub-task's
/// area, three registered worktrees) is live.
fn setup_partially_joined_milestone(repo: &Path, home: &Path) {
    assert_ok(
        &run_milestone(repo, home, &["create", "Cache rework"]),
        "`jigc milestone create`",
    );
    assert_ok(
        &run_milestone(repo, home, &["add-task", MILESTONE, "Warm the read cache"]),
        "add-task #1",
    );
    assert_ok(
        &run_milestone(repo, home, &["add-task", MILESTONE, "Evict cold entries"]),
        "add-task #2",
    );
    assert_ok(
        &run_milestone(repo, home, &["finalize", MILESTONE]),
        "`jigc milestone finalize` (the two sub-tasks genuinely join)",
    );
    assert_ok(
        &run_milestone(repo, home, &["add-task", MILESTONE, "Purge stale keys"]),
        "add-task #3 (after the join — still active)",
    );
    assert_ok(
        &run_milestone(repo, home, &["provision", MILESTONE]),
        "`jigc milestone provision`",
    );

    // The pre-discard record: the header + both finalized sub-tasks read `joined`, the third
    // reads `active` (the fixture the per-item rule needs, minted by the production verbs).
    let body = read_record(repo);
    assert_eq!(
        header_status(&body),
        "joined",
        "pre-discard header:\n{body}"
    );
    for sub in JOINED_SUBS {
        assert_eq!(
            item_status(&body, sub),
            "joined",
            "pre-discard: `{sub}` genuinely joined:\n{body}",
        );
    }
    assert_eq!(
        item_status(&body, ACTIVE_SUB),
        "active",
        "pre-discard: `{ACTIVE_SUB}` never joined:\n{body}",
    );
    // The live workbench: the milestone area, the active sub-task's area, three worktrees.
    assert!(milestone_area(repo).is_dir(), "the milestone area is live");
    assert!(
        subtask_area(repo, ACTIVE_SUB).is_dir(),
        "the active sub-task's working area is live",
    );
    assert_eq!(
        registered_fanout_worktrees(repo).len(),
        3,
        "provision registered one worktree per sub-task",
    );
}

/// (RED-i) A dirty sub-task worktree REFUSES the discard — the abandon path's WIP-safety
/// property. Nothing is destroyed: the file, the worktree, the record, and the workbench all
/// survive, and no commit lands.
#[test]
fn a_dirty_subtask_worktree_refuses_the_discard() {
    let repo = TempDir::new("dirty");
    let home = TempDir::new("home");
    init_repo(repo.path());
    write_compose_marker(repo.path());
    setup_partially_joined_milestone(repo.path(), home.path());

    // Uncommitted work in the fanned sub-agent's worktree — by definition uncommitted on the
    // abandon path (nothing has been committed for it, and nothing will be).
    let scratch = worktree_dir(repo.path(), ACTIVE_SUB).join("scratch.rs");
    fs::write(&scratch, "fn wip() {}\n").expect("write the sub-agent's WIP");

    let before = read_record(repo.path());
    let pre_count = commit_count(repo.path());

    let out = run_milestone(repo.path(), home.path(), &["discard", MILESTONE]);
    assert!(
        !out.status.success(),
        "a dirty sub-task worktree must REFUSE the discard; got exit 0\nstdout:\n{}",
        String::from_utf8_lossy(&out.stdout),
    );
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert!(
        stderr.contains(ACTIVE_SUB) && stderr.contains("scratch.rs"),
        "the refusal names the dirty worktree and its path; stderr:\n{stderr}",
    );
    assert!(
        stderr.contains("--force"),
        "the refusal routes to `--force` (the explicit destroy-my-work confirmation); stderr:\n{stderr}",
    );

    // Nothing was destroyed — the file, the worktree, the workbench, the record, the history.
    assert_eq!(
        fs::read_to_string(&scratch).expect("the WIP file survives the refusal"),
        "fn wip() {}\n",
        "the refused discard must not touch the uncommitted work",
    );
    assert!(
        worktree_dir(repo.path(), ACTIVE_SUB).is_dir(),
        "the refused discard leaves the worktree registered and on disk",
    );
    assert_eq!(
        registered_fanout_worktrees(repo.path()).len(),
        3,
        "the refused discard removes no worktree",
    );
    assert!(milestone_area(repo.path()).is_dir(), "the area survives");
    assert!(
        subtask_area(repo.path(), ACTIVE_SUB).is_dir(),
        "the sub-task area survives",
    );
    assert_eq!(
        read_record(repo.path()),
        before,
        "the refused discard leaves the committed record byte-identical",
    );
    assert_eq!(
        commit_count(repo.path()),
        pre_count,
        "the refused discard commits nothing",
    );
}

/// (RED-ii) `--force` settles the record (header `discarded`, a genuinely joined sub-task still
/// `joined`, the active one `discarded`) in EXACTLY ONE record-only commit — unrelated staged and
/// untracked WIP untouched — and tears the whole workbench down.
#[test]
fn force_settles_the_record_and_tears_the_workbench_down() {
    let repo = TempDir::new("force");
    let home = TempDir::new("home");
    init_repo(repo.path());
    write_compose_marker(repo.path());
    setup_partially_joined_milestone(repo.path(), home.path());

    // The sub-agent's uncommitted work — `--force` is the explicit consent to destroy it.
    fs::write(
        worktree_dir(repo.path(), ACTIVE_SUB).join("scratch.rs"),
        "fn wip() {}\n",
    )
    .expect("write the sub-agent's WIP");
    // Unrelated in-flight WIP in the main checkout: one staged, one untracked. Neither may ride
    // the record commit (the M30/M31 path-scoped staging discipline).
    fs::write(repo.path().join("staged.rs"), "fn staged() {}\n").expect("write the staged WIP");
    git(repo.path(), &["add", "staged.rs"]);
    fs::write(repo.path().join("untracked.md"), "notes\n").expect("write the untracked WIP");

    let pre_count = commit_count(repo.path());

    let out = run_milestone(repo.path(), home.path(), &["discard", MILESTONE, "--force"]);
    assert_ok(&out, "`jigc milestone discard --force`");

    // Exactly ONE commit landed, and it touched ONLY the record.
    assert_eq!(
        commit_count(repo.path()),
        pre_count + 1,
        "discard lands exactly one record-only commit; stderr:\n{}",
        String::from_utf8_lossy(&out.stderr),
    );
    assert_eq!(
        commit_files(repo.path(), "HEAD"),
        vec![record_rel()],
        "the discard commit records ONLY the milestone record",
    );

    // The settled record — on disk and in the commit, the same bytes.
    let after = read_record(repo.path());
    assert_eq!(
        after,
        git(repo.path(), &["show", &format!("HEAD:{}", record_rel())]),
        "the on-disk record matches the bytes the discard committed",
    );
    assert_eq!(
        header_status(&after),
        "discarded",
        "the header settles to the abandon terminal:\n{after}",
    );
    for sub in JOINED_SUBS {
        assert_eq!(
            item_status(&after, sub),
            "joined",
            "a genuinely joined sub-task STAYS joined — it really did land:\n{after}",
        );
    }
    assert_eq!(
        item_status(&after, ACTIVE_SUB),
        "discarded",
        "the never-joined sub-task settles to discarded:\n{after}",
    );
    assert!(
        !after.contains("status: active"),
        "no recorded status survives un-settled:\n{after}",
    );

    // The unrelated WIP is untouched: the staged file is still staged (never committed), the
    // untracked file still untracked on disk.
    assert_eq!(
        git(repo.path(), &["diff", "--cached", "--name-only"]).trim(),
        "staged.rs",
        "the pre-staged unrelated file stays staged and out of the record commit",
    );
    assert!(
        repo.path().join("untracked.md").is_file(),
        "the untracked WIP survives the discard",
    );

    // The workbench is gone: the milestone area (which had NO reachable remover), the sub-task
    // areas, and every registered fan-out worktree.
    assert!(
        !milestone_area(repo.path()).exists(),
        "discard removes `.jigc/milestones/<id>/`",
    );
    assert!(
        !subtask_area(repo.path(), ACTIVE_SUB).exists(),
        "discard removes the sub-task working areas",
    );
    assert!(
        registered_fanout_worktrees(repo.path()).is_empty(),
        "discard removes every registered fan-out worktree; still registered: {:?}",
        registered_fanout_worktrees(repo.path()),
    );
    for sub in JOINED_SUBS.iter().chain([ACTIVE_SUB].iter()) {
        assert!(
            !worktree_dir(repo.path(), sub).exists(),
            "the `{sub}` worktree checkout is gone from disk",
        );
    }
}

/// (RED-iii) An unknown milestone id routes a block and removes nothing — the live milestone's
/// record, workbench, and worktrees all survive.
#[test]
fn an_unknown_milestone_id_routes_and_removes_nothing() {
    let repo = TempDir::new("unknown");
    let home = TempDir::new("home");
    init_repo(repo.path());
    write_compose_marker(repo.path());
    setup_partially_joined_milestone(repo.path(), home.path());

    let before = read_record(repo.path());
    let pre_count = commit_count(repo.path());

    let out = run_milestone(repo.path(), home.path(), &["discard", "no-such-milestone"]);
    assert!(
        !out.status.success(),
        "an unknown milestone id must block; got exit 0",
    );
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert!(
        stderr.contains("no-such-milestone") && stderr.contains("route:"),
        "the block names the id and carries a route; stderr:\n{stderr}",
    );

    // Nothing was removed or committed.
    assert_eq!(read_record(repo.path()), before, "the record is untouched");
    assert_eq!(commit_count(repo.path()), pre_count, "nothing committed");
    assert!(milestone_area(repo.path()).is_dir(), "the area survives");
    assert!(
        subtask_area(repo.path(), ACTIVE_SUB).is_dir(),
        "the sub-task area survives",
    );
    assert_eq!(
        registered_fanout_worktrees(repo.path()).len(),
        3,
        "no worktree was torn down",
    );
}

/// (RED-iv) **The omitting context** — dev-only (no methodology pack, so no `milestone-record`
/// schema resolves): `discard` settles no record and lands NO commit, and still tears the whole
/// workbench down at exit 0. The record arms degrade, they never error.
#[test]
fn dev_only_discard_tears_down_the_workbench_with_no_record_commit() {
    let repo = TempDir::new("devonly");
    let home = TempDir::new("home");
    init_repo(repo.path());
    // No compose marker — the dev pack alone, which ships no `milestone-record` doctype.

    assert_ok(
        &run_milestone(repo.path(), home.path(), &["create", "Cache rework"]),
        "dev-only `jigc milestone create`",
    );
    assert_ok(
        &run_milestone(
            repo.path(),
            home.path(),
            &["add-task", MILESTONE, "Purge stale keys"],
        ),
        "dev-only add-task",
    );
    assert_ok(
        &run_milestone(repo.path(), home.path(), &["provision", MILESTONE]),
        "dev-only provision",
    );
    let pre_count = commit_count(repo.path());

    let out = run_milestone(repo.path(), home.path(), &["discard", MILESTONE]);
    assert_ok(&out, "dev-only `jigc milestone discard`");

    // No record exists, so none is settled and nothing is committed.
    assert!(
        !repo.path().join("docs").join("milestone-records").exists(),
        "dev-only resolves no `milestone-record` doctype — no record home is created",
    );
    assert_eq!(
        commit_count(repo.path()),
        pre_count,
        "dev-only discard lands no record commit",
    );
    // The workbench is torn down all the same.
    assert!(
        !milestone_area(repo.path()).exists(),
        "dev-only discard removes `.jigc/milestones/<id>/`",
    );
    assert!(
        !subtask_area(repo.path(), ACTIVE_SUB).exists(),
        "dev-only discard removes the sub-task working area",
    );
    assert!(
        registered_fanout_worktrees(repo.path()).is_empty(),
        "dev-only discard removes the registered fan-out worktree",
    );
}
