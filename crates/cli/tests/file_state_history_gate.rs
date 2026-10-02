//! M45 Increment 7 / T1 — **file-state history-gating** (`DECISIONS.md` → 2026-07-23
//! M45 Settle, Decision 7; `design/storage.md` → Derived caches). The file↔state hashes
//! carry no stamp and no rebuild path, so a checkout that moves underneath the gitignored
//! cache — `git reset --hard` / branch switch / rebase past a doc's creating commit —
//! leaves a recorded baseline pointing at a path that no longer exists in the working
//! tree, which the drift probe reads today as a **blocking** dangling baseline that wedges
//! every subsequent task.
//!
//! The fix history-gates the weak-signal severity: a dangling baseline downgrades to an
//! **advisory** with a route to the existing `jigc unmanage` **only when `git log HEAD -1
//! -- <path>` is empty** (HEAD has no history for the path — nothing was deleted).
//! Every genuine-deletion case (the path *has* history and is now gone) keeps blocking,
//! and the strong-signal (content-preserving `git mv`) arm is untouched.
//!
//! Three proofs, driving the REAL binary against throwaway git repos, each red before the
//! change:
//!
//!   (a) **reset --hard past the creating commit → advisory, no block.** An ADR is
//!       committed and baselined, then HEAD is moved back past its creating commit
//!       (`git reset --hard <root>`, so `git log HEAD -1 -- <path>` is empty). The next
//!       task's `jigc task validate` reports the dangling baseline **Advisory** with a
//!       `jigc unmanage` prune route and **exits 0** (red today: Blocking, exit 3).
//!
//!   (b) **`git rm` + commit (history present) → still blocks.** The same ADR is deleted
//!       via `git rm` and committed, so HEAD carries history for the path. The weak
//!       finding **still blocks** (`reconciliation.rename`, exit 3) — a genuine deletion
//!       is detected and routed, never silently downgraded.
//!
//!   (c) **bare content-preserving `git mv` → the strong finding is unchanged.** The ADR
//!       is `git mv`'d to a new path with identical content; the strong-signal
//!       `reconciliation.rename` still surfaces (names both paths, routes to `jigc
//!       rename`), untouched by the history gate.
//!
//! # The op axis (the 2026-07-24 confidence audit, sibling-hunt finding 7)
//!
//! The oracle collapse argument lives at the oracle itself
//! ([`engine::file_state::detect_rename`]'s doc): the shipped classification consults
//! exactly three observables — file absent × strong-candidate hash match ×
//! `git log HEAD -1 -- <path>` non-empty — so every orphaning git *operation* projects onto
//! one row of that table. This suite iterates the **distinct rows as real git ops** so that
//! an oracle *choice* change (consulting any further observable: object existence,
//! other-ref reachability, reflog, sparse state) reddens here. The blind-derived op table
//! (`completions/artifacts/M45/sibling-hunt.md` → Appendix) maps to tests as:
//!
//! | op | classification | test |
//! |---|---|---|
//! | `reset --hard` past creation | advisory | (a) above |
//! | `reset --hard` to after a committed delete | block | `reset_hard_to_after_committed_delete_still_blocks` |
//! | branch switch to a pre-creation point | advisory | `branch_switch_to_pre_creation_and_branch_delete_stay_advisory` |
//! | branch switch to a branch where the doc was deleted | block | `branch_switch_to_deletion_branch_still_blocks` |
//! | rebase dropping the creating commit | advisory | `rebase_dropping_creating_commit_downgrades_to_advisory` |
//! | `commit --amend` removing the file from its only creating commit | advisory | `amend_removing_from_creating_commit_downgrades_to_advisory` |
//! | `commit --amend` turning the tip into a deletion | block | `amend_turning_tip_into_deletion_still_blocks` |
//! | `gc` after reset (object existence never consulted) | unchanged | `gc_pruning_the_objects_never_changes_the_classification` |
//! | committed `git mv`, content unchanged | strong block | `committed_git_mv_still_surfaces_the_strong_finding` |
//! | `stash -u` sweeping an untracked rename candidate | history decides | `stash_u_sweeping_the_candidate_flips_strong_to_history_graded_weak` |
//! | branch delete orphaning the sole ref | advisory | folded into the branch-switch advisory test (second act) |
//! | history rewrite dropping the doc (`filter-repo`-shaped) | advisory | `history_rewrite_dropping_the_doc_downgrades_to_advisory` |
//! | sparse-checkout excluding the path | **block (declared bound)** | `sparse_checkout_absence_classifies_as_weak_deletion_block` |
//!
//! **Rows collapsed, with the argument** (each names the row it collapses onto — the
//! collapse holds *for the shipped oracle only*, which is why the rows above are driven as
//! ops, not observables):
//!
//! - **detached checkout to a pre-creation point** — byte-identical observables to the
//!   branch-switch advisory row: the oracle reads HEAD, never *how* HEAD moved (the
//!   branch-delete second act pins that other-ref reachability is never consulted).
//! - **force-pull (fetch + reset to a rewritten remote)** — locally indistinguishable from
//!   `reset --hard` past creation; the remote's involvement leaves no extra local
//!   observable the oracle could read.
//! - **history rewrite renaming the doc / leaving a deletion** — the rename arm lands the
//!   committed-`git mv` row (same-hash candidate → strong), the deletion arm the
//!   `git rm`-with-history row (b); only the drop arm is a distinct row and is driven.
//! - **submodule replacing a former tracked path** — the superproject's history for the
//!   path decides, i.e. the same two observables as the branch rows; no submodule-specific
//!   observable exists in the oracle.
//!
//! **The M52 live-record carve-out is not a new row** (Increment 10 / T6). The advisory
//! arm's *route* now has one exception — the committed record of the milestone the swept
//! task belongs to ([`engine::file_state::LiveRecord`]) — but the exception's input is not a
//! git observable: no git operation creates, removes or moves a task's milestone membership,
//! so every op above still projects onto the same (candidate × history) cell it did, and the
//! table is carried unchanged rather than re-derived. What the carve-out changes is which of
//! two caller-composed presentations one already-classified cell renders, for one
//! caller-named path; `sub_task_at_its_milestone_base_pin_is_never_routed_to_unmanage_its_own_record`
//! pins both sides of that split in one state.
//!
//! Also driven here (the audit's remaining holes): the **milestone-create baseline
//! writer** — the trial's actual §1.3 repro writer
//! (`milestone_create_baseline_reset_downgrades_to_advisory`) — and the **store/task
//! severity split** (`store_scope_stays_blocking_where_task_scope_is_advisory`): the
//! read-only store twin passes an always-history-present predicate, so `jigc validate`
//! keeps the blocking weak finding (and its rename exit-flip) over the exact state the
//! task gate downgrades. Decided at `DECISIONS.md` → 2026-07-24 M45 Increment 7 planning
//! (the deliberate-boundary verified base) — not in Decision 7's own text.

use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

/// The committed ADR at its canonical path / record key.
const ADR_PATH: &str = "docs/decisions/single-node-cache.md";

/// A throwaway directory that removes itself on drop.
struct TempDir(PathBuf);

impl TempDir {
    fn new(tag: &str) -> Self {
        let mut path = std::env::temp_dir();
        path.push(format!(
            "jigc-fshistory-{tag}-{}-{:?}",
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

/// Run a `git` command in `repo`, asserting success and returning trimmed stdout.
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

/// Run `jigc <args>` with `cwd = repo`, `$HOME = home`, capturing output. Never inherits
/// a harness `JIGC_PACK_DIR` — the milestone-writer test composes via the `packs.yaml`
/// marker, which requires the env pack absent; the dev-pack tests are unaffected.
fn jigc(repo: &Path, home: &Path, args: &[&str]) -> std::process::Output {
    Command::new(env!("CARGO_BIN_EXE_jigc"))
        .args(args)
        .current_dir(repo)
        .env("HOME", home)
        .env_remove("JIGC_PACK_DIR")
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
        .env_remove("JIGC_PACK_DIR");
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

/// Fill every author-required field/slot of the provisioned commit doc.
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
    set_field(&format!("commit:{task}#scope"), "cache");
    set_slot(&format!("commit:{task}#summary"), b"change the cache\n");
    set_slot(&format!("commit:{task}#body"), b"A cache change.\n");
}

/// Task 0 — create + finalize `adr:single-node-cache`, the committed managed doc whose
/// landed finalize posts its file-state baseline (finalize phase 7).
fn commit_prior_adr(repo: &Path, home: &Path) {
    let out = jigc(
        repo,
        home,
        &[
            "start",
            "--workflow",
            "single-task",
            "cache sessions in a single in-memory node",
        ],
    );
    assert_ok(&out, "`jigc start` (task 0)");
    let task = "cache-sessions-in-a-single";

    let create = jigc_doc(
        repo,
        home,
        &["create", "adr", "--title", "Single-node cache"],
        None,
    );
    assert_ok(&create, "`jigc doc create adr` (task 0)");

    let set_slot = |addr: &str, prose: &[u8]| {
        let out = jigc_doc(
            repo,
            home,
            &["set-slot", addr, "--from-file", "-"],
            Some(prose),
        );
        assert_ok(&out, &format!("set-slot {addr}"));
    };
    set_slot(
        "adr:single-node-cache#context",
        b"Session lookups must stay sub-millisecond.\n",
    );
    set_slot(
        "adr:single-node-cache#decision",
        b"A single in-memory node keeps lookups fast.\n",
    );
    set_slot(
        "adr:single-node-cache#consequences",
        b"A cold node loses its sessions.\n",
    );
    fill_commit(repo, home, task);

    let out = jigc(repo, home, &["task", "finalize", task, "--format", "json"]);
    assert_ok(&out, "`jigc task finalize` (task 0)");
    assert!(
        repo.join(ADR_PATH).exists(),
        "task 0 must promote {ADR_PATH}"
    );
}

/// Mint a commit-only task with one staged code file + a filled commit doc.
fn stage_commit_only(repo: &Path, home: &Path, task: &str, intent: &str) {
    let out = jigc(repo, home, &["start", "--workflow", "single-task", intent]);
    assert_ok(&out, &format!("`jigc start` ({task})"));
    fs::write(repo.join(format!("{task}.txt")), "the code change\n").expect("write code change");
    git(repo, &["add", &format!("{task}.txt")]);
    fill_commit(repo, home, task);
}

/// The findings array of a parsed JSON report envelope.
fn parse_envelope(stdout: &str, what: &str) -> Vec<serde_json::Value> {
    let value: serde_json::Value = serde_json::from_str(stdout).unwrap_or_else(|err| {
        panic!("{what}: stdout must parse as the report envelope ({err}); got:\n{stdout}")
    });
    value["findings"]
        .as_array()
        .unwrap_or_else(|| {
            panic!("{what}: the envelope must carry a `findings` array; got:\n{stdout}")
        })
        .clone()
}

/// The single `reconciliation.rename` finding naming `path`, or a panic if absent.
fn rename_finding(findings: &[serde_json::Value], path: &str, what: &str) -> serde_json::Value {
    let matches: Vec<&serde_json::Value> = findings
        .iter()
        .filter(|f| {
            f["code"] == "reconciliation.rename"
                && f["message"].as_str().is_some_and(|m| m.contains(path))
        })
        .collect();
    assert_eq!(
        matches.len(),
        1,
        "{what}: exactly one `reconciliation.rename` finding must name {path}; got:\n{findings:#?}",
    );
    matches[0].clone()
}

/// Run `jigc task validate <task> --format json`, returning the raw output + findings.
fn validate_task_json(
    repo: &Path,
    home: &Path,
    task: &str,
    what: &str,
) -> (std::process::Output, Vec<serde_json::Value>) {
    let out = jigc(repo, home, &["task", "validate", task, "--format", "json"]);
    let stdout = String::from_utf8_lossy(&out.stdout).into_owned();
    let findings = parse_envelope(&stdout, what);
    (out, findings)
}

/// Assert the **advisory dangling-baseline** classification for `path`: `task validate`
/// exits 0, the single rename finding is advisory and routes prune-first to `jigc
/// unmanage <path>`. Returns the finding (for byte-wise re-comparison across ops).
fn assert_dangling_advisory(
    out: &std::process::Output,
    findings: &[serde_json::Value],
    path: &str,
    what: &str,
) -> serde_json::Value {
    assert!(
        out.status.success(),
        "{what}: a history-less dangling baseline must not block; got {:?}\nstdout:\n{}\nstderr:\n{}",
        out.status,
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr),
    );
    let rename = rename_finding(findings, path, what);
    assert_eq!(
        rename["severity"], "advisory",
        "{what}: the history-less dangling baseline downgrades to advisory; got:\n{rename:#?}",
    );
    assert!(
        rename["route"]
            .as_str()
            .expect("the advisory carries a route")
            .contains(&format!("jigc unmanage {path}")),
        "{what}: the advisory routes prune-first to `jigc unmanage {path}`; got:\n{rename:#?}",
    );
    rename
}

/// Assert the **blocking weak-deletion** classification for `path`: `task validate` exits
/// 3 and the single rename finding is blocking with the weak (no-suspect) restore route —
/// never the strong `git mv` shape. Returns the finding.
fn assert_weak_deletion_block(
    out: &std::process::Output,
    findings: &[serde_json::Value],
    path: &str,
    what: &str,
) -> serde_json::Value {
    assert_eq!(
        out.status.code(),
        Some(3),
        "{what}: a genuine deletion (history present) must block with exit 3; got {:?}\nstdout:\n{}",
        out.status,
        String::from_utf8_lossy(&out.stdout),
    );
    let rename = rename_finding(findings, path, what);
    assert_eq!(
        rename["severity"], "blocking",
        "{what}: a genuine deletion keeps blocking; got:\n{rename:#?}",
    );
    assert!(
        !rename["message"]
            .as_str()
            .expect("the weak finding carries a message")
            .contains("git mv"),
        "{what}: the weak finding must not claim a `git mv` suspect; got:\n{rename:#?}",
    );
    rename
}

/// Whether a `git` command fails in `repo` (for object-absence probes).
fn git_fails(repo: &Path, args: &[&str]) -> bool {
    !Command::new("git")
        .args(args)
        .current_dir(repo)
        .output()
        .expect("run git")
        .status
        .success()
}

/// (a) A `git reset --hard` past the ADR's creating commit — the path has no HEAD history,
/// so the dangling baseline downgrades to an **advisory** with a `jigc unmanage` prune
/// route, and the next task's `task validate` does **not** block.
#[test]
fn reset_hard_past_creating_commit_downgrades_to_advisory() {
    let repo = TempDir::new("reset");
    let home = TempDir::new("home");
    init_repo(repo.path());
    let root = git(repo.path(), &["rev-parse", "HEAD"]).trim().to_string();

    // Land + baseline the ADR (its finalize is the only commit on top of the root).
    commit_prior_adr(repo.path(), home.path());

    // Move HEAD back past the creating commit: the ADR leaves disk AND leaves HEAD's
    // history, while the gitignored file-state baseline survives the reset.
    git(repo.path(), &["reset", "--hard", &root]);
    assert!(
        !repo.path().join(ADR_PATH).exists(),
        "the reset removes the ADR from the working tree",
    );
    assert!(
        git(repo.path(), &["log", "HEAD", "-1", "--", ADR_PATH])
            .trim()
            .is_empty(),
        "HEAD carries no history for the ADR path after the reset",
    );
    let record = repo
        .path()
        .join(".jigc")
        .join("state")
        .join("file-state.json");
    assert!(
        fs::read_to_string(&record)
            .expect("the file-state baseline survives the reset")
            .contains(ADR_PATH),
        "the dangling baseline is still recorded",
    );

    // The next task previews the store sweep: the dangling baseline is advisory, not a block.
    let task = "warm-the-read-cache";
    stage_commit_only(repo.path(), home.path(), task, "warm the read cache");
    let out = jigc(
        repo.path(),
        home.path(),
        &["task", "validate", task, "--format", "json"],
    );
    let stdout = String::from_utf8_lossy(&out.stdout);
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert!(
        out.status.success(),
        "a history-less dangling baseline must not block `task validate`; \
         got {:?}\nstdout:\n{stdout}\nstderr:\n{stderr}",
        out.status,
    );
    let findings = parse_envelope(&stdout, "history-less validate");
    let rename = rename_finding(&findings, ADR_PATH, "history-less validate");
    assert_eq!(
        rename["severity"], "advisory",
        "the history-less dangling baseline downgrades to advisory; got:\n{rename:#?}",
    );
    assert!(
        rename["route"]
            .as_str()
            .expect("the advisory carries a route")
            .contains(&format!("jigc unmanage {ADR_PATH}")),
        "the advisory routes prune-first to `jigc unmanage {ADR_PATH}`; got:\n{rename:#?}",
    );
}

/// (b) A `git rm <ADR>` + commit leaves the path with HEAD history — the weak-signal
/// finding still **blocks** (`reconciliation.rename`, exit 3), never silently downgraded.
#[test]
fn git_rm_with_history_still_blocks() {
    let repo = TempDir::new("gitrm");
    let home = TempDir::new("home");
    init_repo(repo.path());
    commit_prior_adr(repo.path(), home.path());

    // Delete the committed ADR through git — the deletion commit gives the path history.
    git(repo.path(), &["rm", "-q", ADR_PATH]);
    git(
        repo.path(),
        &["commit", "-q", "-m", "docs: drop the cache ADR"],
    );
    assert!(
        !git(repo.path(), &["log", "HEAD", "-1", "--", ADR_PATH])
            .trim()
            .is_empty(),
        "HEAD carries history for the deleted ADR path",
    );

    let task = "warm-the-read-cache";
    stage_commit_only(repo.path(), home.path(), task, "warm the read cache");
    let out = jigc(
        repo.path(),
        home.path(),
        &["task", "validate", task, "--format", "json"],
    );
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(
        !out.status.success(),
        "a genuine deletion (history present) must still block; got success\nstdout:\n{stdout}",
    );
    assert_eq!(
        out.status.code(),
        Some(3),
        "a blocking validate exits 3; got {:?}",
        out.status,
    );
    let findings = parse_envelope(&stdout, "genuine-deletion validate");
    let rename = rename_finding(&findings, ADR_PATH, "genuine-deletion validate");
    assert_eq!(
        rename["severity"], "blocking",
        "a genuine deletion keeps blocking; got:\n{rename:#?}",
    );
}

/// (c) A bare content-preserving `git mv` is the strong-signal arm the history gate leaves
/// untouched — the `reconciliation.rename` finding names both paths and routes to `jigc
/// rename`, and still blocks.
#[test]
fn bare_git_mv_still_surfaces_the_strong_finding() {
    const MOVED_PATH: &str = "docs/decisions/renamed-cache.md";
    let repo = TempDir::new("gitmv");
    let home = TempDir::new("home");
    init_repo(repo.path());
    commit_prior_adr(repo.path(), home.path());

    // A content-preserving move: the moved file carries the recorded hash (strong signal).
    git(repo.path(), &["mv", ADR_PATH, MOVED_PATH]);
    assert!(
        repo.path().join(MOVED_PATH).exists() && !repo.path().join(ADR_PATH).exists(),
        "the git mv relocates the ADR with its content intact",
    );

    let task = "warm-the-read-cache";
    stage_commit_only(repo.path(), home.path(), task, "warm the read cache");
    let out = jigc(
        repo.path(),
        home.path(),
        &["task", "validate", task, "--format", "json"],
    );
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(
        !out.status.success(),
        "a strong-signal rename still blocks; got success\nstdout:\n{stdout}",
    );
    let findings = parse_envelope(&stdout, "git-mv validate");
    let rename = rename_finding(&findings, ADR_PATH, "git-mv validate");
    assert_eq!(
        rename["severity"], "blocking",
        "the strong-signal rename is untouched by the history gate; got:\n{rename:#?}",
    );
    assert!(
        rename["message"]
            .as_str()
            .expect("the strong finding carries a message")
            .contains("git mv"),
        "the strong finding names the suspected `git mv`; got:\n{rename:#?}",
    );
    assert!(
        rename["route"]
            .as_str()
            .expect("the strong finding carries a route")
            .contains("jigc rename"),
        "the strong finding routes to `jigc rename`; got:\n{rename:#?}",
    );
}

/// Op row — **`reset --hard` to after a committed delete → block.** The deletion commit is
/// reachable from the reset target, so arriving at the state *via reset* changes nothing:
/// the gate keys on `git log HEAD -1 -- <path>`, not on whether a checkout moved.
#[test]
fn reset_hard_to_after_committed_delete_still_blocks() {
    let repo = TempDir::new("reset-after-del");
    let home = TempDir::new("home");
    init_repo(repo.path());
    commit_prior_adr(repo.path(), home.path());

    // Delete + commit (the deletion commit D), then one more commit E, then reset to D.
    git(repo.path(), &["rm", "-q", ADR_PATH]);
    git(repo.path(), &["commit", "-q", "-m", "docs: drop the ADR"]);
    let deletion = git(repo.path(), &["rev-parse", "HEAD"]).trim().to_string();
    fs::write(repo.path().join("extra.txt"), "later work\n").expect("write extra");
    git(repo.path(), &["add", "extra.txt"]);
    git(repo.path(), &["commit", "-q", "-m", "later work"]);
    git(repo.path(), &["reset", "--hard", "-q", &deletion]);
    assert!(
        !git(repo.path(), &["log", "HEAD", "-1", "--", ADR_PATH])
            .trim()
            .is_empty(),
        "the deletion commit is reachable after the reset",
    );

    let task = "warm-the-read-cache";
    stage_commit_only(repo.path(), home.path(), task, "warm the read cache");
    let (out, findings) = validate_task_json(repo.path(), home.path(), task, "reset-after-delete");
    assert_weak_deletion_block(&out, &findings, ADR_PATH, "reset-after-delete");
}

/// Op rows — **branch switch to a pre-creation point → advisory**, and (second act)
/// **branch delete orphaning the sole ref → the classification is byte-identical.** The
/// ADR lands on a feature branch; switching back to the default branch leaves the path
/// history-less at HEAD → advisory. Deleting the feature branch (the only ref reaching the
/// creating commit) changes nothing — the oracle reads HEAD only, never other-ref
/// reachability, which the equality of the two findings pins.
#[test]
fn branch_switch_to_pre_creation_and_branch_delete_stay_advisory() {
    let repo = TempDir::new("branch-switch");
    let home = TempDir::new("home");
    init_repo(repo.path());
    let default_branch = git(repo.path(), &["rev-parse", "--abbrev-ref", "HEAD"])
        .trim()
        .to_string();

    git(repo.path(), &["checkout", "-q", "-b", "feature"]);
    commit_prior_adr(repo.path(), home.path());
    git(repo.path(), &["checkout", "-q", &default_branch]);
    assert!(
        !repo.path().join(ADR_PATH).exists(),
        "the branch switch removes the ADR from the working tree",
    );

    let task = "warm-the-read-cache";
    stage_commit_only(repo.path(), home.path(), task, "warm the read cache");
    let (out, findings) = validate_task_json(repo.path(), home.path(), task, "branch-switch");
    let before = assert_dangling_advisory(&out, &findings, ADR_PATH, "branch-switch");

    // Second act: delete the sole ref reaching the creating commit. HEAD is unchanged, so
    // the shipped oracle must classify byte-identically (other refs are never consulted).
    git(repo.path(), &["branch", "-q", "-D", "feature"]);
    let (out, findings) = validate_task_json(repo.path(), home.path(), task, "branch-delete");
    let after = assert_dangling_advisory(&out, &findings, ADR_PATH, "branch-delete");
    assert_eq!(
        before, after,
        "deleting the orphaned branch must not change the classification (HEAD-only oracle)",
    );
}

/// Op row — **branch switch to a branch where the doc was deleted → block.** The deletion
/// branch carries HEAD history for the path, so arriving there *by switch* (the same
/// checkout motion the advisory rows use) still reads as a genuine deletion: the gate keys
/// history, not the fact that a checkout moved underneath the cache.
#[test]
fn branch_switch_to_deletion_branch_still_blocks() {
    let repo = TempDir::new("branch-del-switch");
    let home = TempDir::new("home");
    init_repo(repo.path());
    let default_branch = git(repo.path(), &["rev-parse", "--abbrev-ref", "HEAD"])
        .trim()
        .to_string();
    commit_prior_adr(repo.path(), home.path());

    // A branch that deletes the ADR; then switch away and back onto it.
    git(repo.path(), &["checkout", "-q", "-b", "drop-the-adr"]);
    git(repo.path(), &["rm", "-q", ADR_PATH]);
    git(repo.path(), &["commit", "-q", "-m", "docs: drop the ADR"]);
    git(repo.path(), &["checkout", "-q", &default_branch]);
    git(repo.path(), &["checkout", "-q", "drop-the-adr"]);
    assert!(
        !repo.path().join(ADR_PATH).exists()
            && !git(repo.path(), &["log", "HEAD", "-1", "--", ADR_PATH])
                .trim()
                .is_empty(),
        "the deletion branch lacks the file but carries its history",
    );

    let task = "warm-the-read-cache";
    stage_commit_only(repo.path(), home.path(), task, "warm the read cache");
    let (out, findings) =
        validate_task_json(repo.path(), home.path(), task, "deletion-branch switch");
    assert_weak_deletion_block(&out, &findings, ADR_PATH, "deletion-branch switch");
}

/// Op row — **rebase dropping the creating commit → advisory.** `git rebase --onto <root>
/// <creating>` replays the later commit without the ADR's creating commit; the rebased
/// HEAD carries no history for the path.
#[test]
fn rebase_dropping_creating_commit_downgrades_to_advisory() {
    let repo = TempDir::new("rebase-drop");
    let home = TempDir::new("home");
    init_repo(repo.path());
    let root = git(repo.path(), &["rev-parse", "HEAD"]).trim().to_string();
    commit_prior_adr(repo.path(), home.path());
    let creating = git(repo.path(), &["rev-parse", "HEAD"]).trim().to_string();
    fs::write(repo.path().join("extra.txt"), "later work\n").expect("write extra");
    git(repo.path(), &["add", "extra.txt"]);
    git(repo.path(), &["commit", "-q", "-m", "later work"]);

    // Drop the creating commit: replay everything after it onto the root.
    git(repo.path(), &["rebase", "-q", "--onto", &root, &creating]);
    assert!(
        !repo.path().join(ADR_PATH).exists()
            && git(repo.path(), &["log", "HEAD", "-1", "--", ADR_PATH])
                .trim()
                .is_empty(),
        "the rebase drops the creating commit: file gone, history empty",
    );

    let task = "warm-the-read-cache";
    stage_commit_only(repo.path(), home.path(), task, "warm the read cache");
    let (out, findings) = validate_task_json(repo.path(), home.path(), task, "rebase-drop");
    assert_dangling_advisory(&out, &findings, ADR_PATH, "rebase-drop");
}

/// Op row — **`commit --amend` removing the file from its only creating commit →
/// advisory.** The amended tip never touches the path (its parent lacks it too), so the
/// path reads history-less.
#[test]
fn amend_removing_from_creating_commit_downgrades_to_advisory() {
    let repo = TempDir::new("amend-remove");
    let home = TempDir::new("home");
    init_repo(repo.path());
    commit_prior_adr(repo.path(), home.path());

    git(repo.path(), &["rm", "-q", ADR_PATH]);
    git(repo.path(), &["commit", "-q", "--amend", "--no-edit"]);
    assert!(
        git(repo.path(), &["log", "HEAD", "-1", "--", ADR_PATH])
            .trim()
            .is_empty(),
        "the amended history never touches the ADR path",
    );

    let task = "warm-the-read-cache";
    stage_commit_only(repo.path(), home.path(), task, "warm the read cache");
    let (out, findings) = validate_task_json(repo.path(), home.path(), task, "amend-remove");
    assert_dangling_advisory(&out, &findings, ADR_PATH, "amend-remove");
}

/// Op row — **`commit --amend` turning the tip into a deletion → block.** The amended tip
/// sits on top of the creating commit and now carries the deletion diff, so the path has
/// history — a genuine deletion, not a moved checkout.
#[test]
fn amend_turning_tip_into_deletion_still_blocks() {
    let repo = TempDir::new("amend-delete");
    let home = TempDir::new("home");
    init_repo(repo.path());
    commit_prior_adr(repo.path(), home.path());
    fs::write(repo.path().join("extra.txt"), "later work\n").expect("write extra");
    git(repo.path(), &["add", "extra.txt"]);
    git(repo.path(), &["commit", "-q", "-m", "later work"]);

    git(repo.path(), &["rm", "-q", ADR_PATH]);
    git(repo.path(), &["commit", "-q", "--amend", "--no-edit"]);
    assert!(
        !git(repo.path(), &["log", "HEAD", "-1", "--", ADR_PATH])
            .trim()
            .is_empty(),
        "the amended tip carries the deletion diff for the ADR path",
    );

    let task = "warm-the-read-cache";
    stage_commit_only(repo.path(), home.path(), task, "warm the read cache");
    let (out, findings) = validate_task_json(repo.path(), home.path(), task, "amend-delete");
    assert_weak_deletion_block(&out, &findings, ADR_PATH, "amend-delete");
}

/// Op row — **`gc` after the reset changes nothing: object existence is never
/// consulted.** After the reset-past-creation advisory, expiring the reflog and pruning
/// makes the ADR's blob genuinely unreachable-and-gone — and the classification is
/// byte-identical. This is the corrected-oracle pin: the rejected `git cat-file -e`
/// alternative would answer differently before and after the prune.
#[test]
fn gc_pruning_the_objects_never_changes_the_classification() {
    let repo = TempDir::new("gc");
    let home = TempDir::new("home");
    init_repo(repo.path());
    let root = git(repo.path(), &["rev-parse", "HEAD"]).trim().to_string();
    commit_prior_adr(repo.path(), home.path());
    let blob = git(repo.path(), &["rev-parse", &format!("HEAD:{ADR_PATH}")])
        .trim()
        .to_string();
    git(repo.path(), &["reset", "--hard", "-q", &root]);

    let task = "warm-the-read-cache";
    stage_commit_only(repo.path(), home.path(), task, "warm the read cache");
    let (out, findings) = validate_task_json(repo.path(), home.path(), task, "pre-gc");
    let before = assert_dangling_advisory(&out, &findings, ADR_PATH, "pre-gc");

    git(repo.path(), &["reflog", "expire", "--expire=now", "--all"]);
    git(repo.path(), &["gc", "--prune=now", "-q"]);
    assert!(
        git_fails(repo.path(), &["cat-file", "-e", &blob]),
        "the prune removes the ADR blob — the object is genuinely gone",
    );

    let (out, findings) = validate_task_json(repo.path(), home.path(), task, "post-gc");
    let after = assert_dangling_advisory(&out, &findings, ADR_PATH, "post-gc");
    assert_eq!(
        before, after,
        "pruning the objects must not change the classification (object existence never consulted)",
    );
}

/// Op row — **committed `git mv`, content unchanged → the strong finding.** Committing the
/// move (vs. test (c)'s staged-only move) changes nothing: the landing is still an on-disk
/// same-hash candidate with no recorded baseline, so the strong signal fires and blocks.
#[test]
fn committed_git_mv_still_surfaces_the_strong_finding() {
    const MOVED_PATH: &str = "docs/decisions/renamed-cache.md";
    let repo = TempDir::new("committed-mv");
    let home = TempDir::new("home");
    init_repo(repo.path());
    commit_prior_adr(repo.path(), home.path());

    git(repo.path(), &["mv", ADR_PATH, MOVED_PATH]);
    git(repo.path(), &["commit", "-q", "-m", "docs: move the ADR"]);

    let task = "warm-the-read-cache";
    stage_commit_only(repo.path(), home.path(), task, "warm the read cache");
    let (out, findings) = validate_task_json(repo.path(), home.path(), task, "committed-mv");
    assert!(
        !out.status.success(),
        "a committed strong-signal rename still blocks; stdout:\n{}",
        String::from_utf8_lossy(&out.stdout),
    );
    let rename = rename_finding(&findings, ADR_PATH, "committed-mv");
    assert_eq!(rename["severity"], "blocking");
    assert!(
        rename["message"]
            .as_str()
            .expect("message")
            .contains(MOVED_PATH)
            && rename["message"]
                .as_str()
                .expect("message")
                .contains("git mv"),
        "the strong finding names the committed landing; got:\n{rename:#?}",
    );
}

/// Op row — **`stash -u` sweeping an untracked rename candidate flips strong → weak, and
/// history then grades the weak arm.** With a same-hash untracked candidate present the
/// strong finding fires; after `git stash push -u` sweeps the candidate the same state
/// re-reads as the weak arm, which the (empty) history downgrades to the advisory. The
/// stash is pathspec-scoped to the candidate so it never sweeps the gitignore-less test
/// repo's `.jigc/` workbench (which a real setup gitignores) or the task's staged file.
#[test]
fn stash_u_sweeping_the_candidate_flips_strong_to_history_graded_weak() {
    const CANDIDATE_PATH: &str = "docs/decisions/restored-cache.md";
    let repo = TempDir::new("stash");
    let home = TempDir::new("home");
    init_repo(repo.path());
    let root = git(repo.path(), &["rev-parse", "HEAD"]).trim().to_string();

    commit_prior_adr(repo.path(), home.path());
    let adr_bytes = fs::read(repo.path().join(ADR_PATH)).expect("read the promoted ADR");
    git(repo.path(), &["reset", "--hard", "-q", &root]);

    // An untracked same-content candidate: the strong signal.
    fs::create_dir_all(repo.path().join("docs/decisions")).expect("mk decisions dir");
    fs::write(repo.path().join(CANDIDATE_PATH), &adr_bytes).expect("write candidate");

    let task = "warm-the-read-cache";
    stage_commit_only(repo.path(), home.path(), task, "warm the read cache");
    let (out, findings) = validate_task_json(repo.path(), home.path(), task, "pre-stash");
    assert!(
        !out.status.success(),
        "the same-hash candidate makes the strong signal, which blocks; stdout:\n{}",
        String::from_utf8_lossy(&out.stdout),
    );
    let strong = rename_finding(&findings, ADR_PATH, "pre-stash");
    assert!(
        strong["message"]
            .as_str()
            .expect("message")
            .contains("git mv"),
        "with the candidate present the finding is the strong shape; got:\n{strong:#?}",
    );

    // The stash sweeps the untracked candidate.
    git(
        repo.path(),
        &["stash", "push", "-q", "-u", "--", CANDIDATE_PATH],
    );
    assert!(
        !repo.path().join(CANDIDATE_PATH).exists(),
        "the stash sweeps the untracked candidate",
    );

    let (out, findings) = validate_task_json(repo.path(), home.path(), task, "post-stash");
    assert_dangling_advisory(&out, &findings, ADR_PATH, "post-stash");
}

/// Op row — **history rewrite dropping the doc (the `filter-repo` shape, cheapest
/// equivalent) → advisory.** The whole post-root history is squashed into one commit that
/// omits the ADR (`reset --soft` + `git rm` + commit — the same end state a
/// `filter-repo --invert-paths` run leaves), so the rewritten HEAD never touches the path.
/// The rewrite's rename / leave-a-deletion arms collapse onto the committed-mv and
/// `git rm` rows (see the module doc).
#[test]
fn history_rewrite_dropping_the_doc_downgrades_to_advisory() {
    let repo = TempDir::new("rewrite-drop");
    let home = TempDir::new("home");
    init_repo(repo.path());
    let root = git(repo.path(), &["rev-parse", "HEAD"]).trim().to_string();
    commit_prior_adr(repo.path(), home.path());
    fs::write(repo.path().join("extra.txt"), "later work\n").expect("write extra");
    git(repo.path(), &["add", "extra.txt"]);
    git(repo.path(), &["commit", "-q", "-m", "later work"]);

    // The rewrite: squash everything since the root into one commit without the ADR
    // (`-f`: after the soft reset the ADR is index-staged content new to HEAD).
    git(repo.path(), &["reset", "--soft", "-q", &root]);
    git(repo.path(), &["rm", "-q", "-f", ADR_PATH]);
    git(
        repo.path(),
        &["commit", "-q", "-m", "rewrite: history without the ADR"],
    );
    assert!(
        !repo.path().join(ADR_PATH).exists()
            && git(repo.path(), &["log", "HEAD", "-1", "--", ADR_PATH])
                .trim()
                .is_empty(),
        "the rewritten history never touches the ADR path",
    );

    let task = "warm-the-read-cache";
    stage_commit_only(repo.path(), home.path(), task, "warm the read cache");
    let (out, findings) = validate_task_json(repo.path(), home.path(), task, "rewrite-drop");
    assert_dangling_advisory(&out, &findings, ADR_PATH, "rewrite-drop");
}

/// Op row — **sparse-checkout excluding the path → the weak-deletion block (the declared
/// conservative bound).** The file is absent from the worktree but tracked in HEAD, so the
/// oracle reads (absent × history-present) → block — a **false-deletion shape**: nothing
/// was deleted, the file is merely unmaterialized. Pinned as-is and declared at the oracle
/// ([`engine::file_state::detect_rename`]) + Decision 7's entry: blocking is the safe
/// direction, and no sparse-checkout user exists to warrant in-oracle detection.
#[test]
fn sparse_checkout_absence_classifies_as_weak_deletion_block() {
    let repo = TempDir::new("sparse");
    let home = TempDir::new("home");
    init_repo(repo.path());
    commit_prior_adr(repo.path(), home.path());

    git(
        repo.path(),
        &["sparse-checkout", "set", "--no-cone", "/*", "!docs/"],
    );
    assert!(
        !repo.path().join(ADR_PATH).exists()
            && !git(repo.path(), &["log", "HEAD", "-1", "--", ADR_PATH])
                .trim()
                .is_empty(),
        "sparse-checkout unmaterializes the tracked ADR: absent from the worktree, history present",
    );

    let task = "warm-the-read-cache";
    stage_commit_only(repo.path(), home.path(), task, "warm the read cache");
    let (out, findings) = validate_task_json(repo.path(), home.path(), task, "sparse-checkout");
    assert_weak_deletion_block(&out, &findings, ADR_PATH, "sparse-checkout");
}

/// The **milestone-create baseline writer** — the trial's actual §1.3 repro writer. `jigc
/// milestone create` (under the `[dev ▸ methodology]` compose marker) materializes +
/// path-scoped-commits the `milestone-record` AND baselines it in the file-state record;
/// a `git reset --hard` past that commit then leaves the baseline dangling with no HEAD
/// history — advisory + `jigc unmanage` route, not the wedge the trial hit.
#[test]
fn milestone_create_baseline_reset_downgrades_to_advisory() {
    const RECORD_PATH: &str = "docs/milestone-records/cache-rework.md";
    let repo = TempDir::new("milestone");
    let home = TempDir::new("home");
    init_repo(repo.path());
    fs::write(
        repo.path().join(".jigc").join("config").join("packs.yaml"),
        "compose-embedded-methodology: true\n",
    )
    .expect("write compose marker");

    let created = jigc(
        repo.path(),
        home.path(),
        &["milestone", "create", "Cache rework"],
    );
    assert_ok(&created, "`jigc milestone create`");
    assert!(
        repo.path().join(RECORD_PATH).exists(),
        "create materializes the committed milestone record",
    );
    let record = repo
        .path()
        .join(".jigc")
        .join("state")
        .join("file-state.json");
    assert!(
        fs::read_to_string(&record)
            .expect("the file-state record exists after create")
            .contains(RECORD_PATH),
        "milestone create baselines the committed record (the trial's §1.3 writer)",
    );

    // The reset moves HEAD past the record's creating commit; the gitignored baseline survives.
    git(repo.path(), &["reset", "--hard", "-q", "HEAD~1"]);
    assert!(
        !repo.path().join(RECORD_PATH).exists()
            && git(repo.path(), &["log", "HEAD", "-1", "--", RECORD_PATH])
                .trim()
                .is_empty(),
        "the reset orphans the milestone-record baseline",
    );

    let task = "warm-the-read-cache";
    stage_commit_only(repo.path(), home.path(), task, "warm the read cache");
    let (out, findings) = validate_task_json(repo.path(), home.path(), task, "milestone-writer");
    assert_dangling_advisory(&out, &findings, RECORD_PATH, "milestone-writer");
}

/// The **store/task severity split** — over the exact state the task gate downgrades
/// (reset past creation, history-less), the read-only store twin passes an
/// always-history-present predicate: `jigc validate` keeps the **blocking** weak finding
/// and its `reconciliation.rename` exit-flip (exit 1). Decided at `DECISIONS.md` →
/// 2026-07-24 M45 Increment 7 planning (the deliberate-boundary verified base).
#[test]
fn store_scope_stays_blocking_where_task_scope_is_advisory() {
    let repo = TempDir::new("scope-split");
    let home = TempDir::new("home");
    init_repo(repo.path());
    let root = git(repo.path(), &["rev-parse", "HEAD"]).trim().to_string();
    commit_prior_adr(repo.path(), home.path());
    git(repo.path(), &["reset", "--hard", "-q", &root]);

    // Task scope: advisory, exit 0.
    let task = "warm-the-read-cache";
    stage_commit_only(repo.path(), home.path(), task, "warm the read cache");
    let (out, findings) = validate_task_json(repo.path(), home.path(), task, "split/task-scope");
    assert_dangling_advisory(&out, &findings, ADR_PATH, "split/task-scope");

    // Store scope: the same state keeps the blocking weak finding and flips the exit.
    let out = jigc(repo.path(), home.path(), &["validate", "--format", "json"]);
    let stdout = String::from_utf8_lossy(&out.stdout);
    let findings = parse_envelope(&stdout, "split/store-scope");
    let rename = rename_finding(&findings, ADR_PATH, "split/store-scope");
    assert_eq!(
        rename["severity"], "blocking",
        "store scope stays blocking (always-history-present twin); got:\n{rename:#?}",
    );
    assert_eq!(
        out.status.code(),
        Some(1),
        "a store-scope rename finding flips the exit; stdout:\n{stdout}",
    );
}

/// The **conservative default is provoked, not just written** (confidence-audit minor
/// item 8): when `git log HEAD -1 -- <path>` itself FAILS — here a real unborn HEAD,
/// reached by `git checkout --orphan` after the baseline landed — the history
/// predicate's `unwrap_or(true)` treats the path as history-present, so the dangling
/// baseline keeps the **blocking** weak-deletion shape rather than silently
/// downgrading a possible deletion to the advisory. Flipping the default to `false`
/// (advisory) reddens this test.
///
/// **The door moved at M52 Increment 3 / T6, and the arm asserts why.** An unborn HEAD is
/// a member of the repository-posture family, so `jigc task finalize` refuses in this
/// fixture — and since T6 `jigc task validate` previews that refusal, byte-identically,
/// instead of reporting the task's content over a state the commit boundary will not act
/// in. That is asserted here first, because this fixture is the **only** live task on an
/// unborn HEAD in the suite set and `validate_previews_posture.rs` cannot build one (the
/// git-state axis refuses to overlay `unborn` on a corpus `jigc setup` has committed
/// into) — so this arm covers that axis's thirteenth cell.
///
/// The history predicate is then reached through **bare `jigc start`**, which runs the
/// identical task-scope sweep (`crate::task::sweep_for_orientation`, `GatePreview::On`)
/// and is classified `ActsOnBehalf::Neither`, so no posture stands between the fixture
/// and the classification under test. What it cannot carry is `task validate`'s exit
/// code — orientation is a reader and reports at exit 0 — so the discriminating
/// assertion is the **severity**, which is the whole of what the conservative default
/// decides: flipped to `false`, this finding is advisory and the arm reddens.
#[test]
fn unborn_head_keeps_the_conservative_weak_deletion_block() {
    let repo = TempDir::new("unborn");
    let home = TempDir::new("home");
    init_repo(repo.path());
    commit_prior_adr(repo.path(), home.path());

    // An orphan checkout leaves HEAD pointing at an unborn ref: worktree and index
    // survive, but `git log HEAD -1 -- <path>` now fails outright ("unknown
    // revision") — the exact failure the conservative default guards.
    git(repo.path(), &["checkout", "-q", "--orphan", "orphaned"]);
    fs::remove_file(repo.path().join(ADR_PATH)).expect("remove the ADR worktree copy");
    assert!(
        !Command::new("git")
            .args(["log", "HEAD", "-1", "--", ADR_PATH])
            .current_dir(repo.path())
            .output()
            .expect("run git")
            .status
            .success(),
        "`git log HEAD` must fail under the unborn HEAD (the provoked arm)",
    );

    let task = "warm-the-read-cache";
    stage_commit_only(repo.path(), home.path(), task, "warm the read cache");

    // The posture cell: the preview refuses what the commit boundary refuses.
    let preview = jigc(repo.path(), home.path(), &["task", "validate", task]);
    assert_eq!(
        preview.status.code(),
        Some(1),
        "an unborn HEAD is a posture `task finalize` refuses, so its preview refuses too; \
         stdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&preview.stdout),
        String::from_utf8_lossy(&preview.stderr),
    );
    let refusal = String::from_utf8_lossy(&preview.stderr);
    assert!(
        refusal.contains("repo.head-unborn"),
        "and names the member; got:\n{refusal}",
    );

    // The classification under test, through the door the posture does not guard.
    let out = jigc(repo.path(), home.path(), &["start", "--format", "json"]);
    let stdout = String::from_utf8_lossy(&out.stdout).into_owned();
    let orientation: serde_json::Value = serde_json::from_str(&stdout).unwrap_or_else(|err| {
        panic!("unborn-head: bare `jigc start` must emit its envelope ({err}); got:\n{stdout}")
    });
    let findings: Vec<serde_json::Value> = orientation["tasks"]
        .as_array()
        .unwrap_or_else(|| panic!("unborn-head: the envelope carries `tasks`; got:\n{stdout}"))
        .iter()
        .find(|row| row["id"] == task)
        .unwrap_or_else(|| panic!("unborn-head: `{task}` is active; got:\n{stdout}"))["findings"]
        .as_array()
        .unwrap_or_else(|| {
            panic!("unborn-head: the sweep ran and its findings ride the row; got:\n{stdout}")
        })
        .clone();
    let rename = rename_finding(&findings, ADR_PATH, "unborn-head");
    assert_eq!(
        rename["severity"], "blocking",
        "unborn-head: a failing history probe stays conservative — history PRESENT, so the \
         dangling baseline keeps blocking; got:\n{rename:#?}",
    );
    assert!(
        !rename["message"]
            .as_str()
            .expect("the weak finding carries a message")
            .contains("git mv"),
        "unborn-head: the weak finding must not claim a `git mv` suspect; got:\n{rename:#?}",
    );
}

/// **M52 Increment 10 / T6 — the live milestone record.** A sub-task standing in the shared
/// checkout at its milestone's **base pin** reads its own milestone record as an absent,
/// history-less baseline: the pin predates the record commit `jigc milestone create` lands,
/// by construction. That is the dangling-baseline cell exactly — and the shipped route,
/// followed, runs `jigc unmanage` over the record the milestone is run from.
///
/// The carve-out is **path-keyed on the task's own milestone**, never blanket, so the state
/// is built to discriminate in one repo: two milestones are created in sequence, the
/// sub-task is added to the **first**, and the checkout returns to that first milestone's
/// base pin — which predates **both** record commits, so both baselines dangle identically
/// and the only difference between them is which milestone owns the task being swept. The
/// other milestone's record must keep the shipped advisory and its prune route
/// **byte-identically** (asserted as whole strings, not as a substring probe); the task's
/// own record must say what it is and route at no destructive verb.
///
/// The history gate above this arm is untouched: a record with HEAD history — a genuine
/// deletion — still reaches the blocking weak finding, which
/// `branch_switch_to_deletion_branch_still_blocks` and proof (b) above pin over the ADR
/// path.
#[test]
fn sub_task_at_its_milestone_base_pin_is_never_routed_to_unmanage_its_own_record() {
    const OWN_RECORD: &str = "docs/milestone-records/cache-rework.md";
    const OTHER_RECORD: &str = "docs/milestone-records/ledger-rework.md";
    let repo = TempDir::new("live-record");
    let home = TempDir::new("home");
    init_repo(repo.path());
    fs::write(
        repo.path().join(".jigc").join("config").join("packs.yaml"),
        "compose-embedded-methodology: true\n",
    )
    .expect("write compose marker");

    // HEAD before either record commit — the FIRST milestone's base pin, and therefore
    // the sub-task's. (The second milestone's own base is the first's record commit; the
    // pin the checkout returns to is this one, and it predates both records.)
    let base = git(repo.path(), &["rev-parse", "HEAD"]).trim().to_string();

    for title in ["Cache rework", "Ledger rework"] {
        let created = jigc(repo.path(), home.path(), &["milestone", "create", title]);
        assert_ok(&created, &format!("`jigc milestone create {title}`"));
    }
    let task = "warm-the-read-cache";
    let added = jigc(
        repo.path(),
        home.path(),
        &[
            "milestone",
            "add-task",
            "cache-rework",
            "warm the read cache",
        ],
    );
    assert_ok(&added, "`jigc milestone add-task`");

    // Stand in the shared checkout at the sub-task's base pin: both records leave disk and
    // neither has HEAD history here, while the gitignored baselines survive.
    git(
        repo.path(),
        &["checkout", "-q", "-b", "at-the-base-pin", &base],
    );
    for path in [OWN_RECORD, OTHER_RECORD] {
        assert!(
            !repo.path().join(path).exists()
                && git(repo.path(), &["log", "HEAD", "-1", "--", path])
                    .trim()
                    .is_empty(),
            "the base pin predates {path}: absent on disk, no HEAD history",
        );
    }

    let (out, findings) = validate_task_json(repo.path(), home.path(), task, "live-record");

    // The control — a record of a milestone this task does NOT belong to is an ordinary
    // dangling baseline and keeps the shipped advisory byte-for-byte.
    let other = assert_dangling_advisory(&out, &findings, OTHER_RECORD, "live-record/other");
    assert_eq!(
        other["message"]
            .as_str()
            .expect("the control has a message"),
        format!(
            "tracked managed doc milestone-record:ledger-rework ({OTHER_RECORD}) is missing, \
             but the path has no history — the checkout moved underneath the file-state \
             cache, not a deletion"
        ),
        "live-record/other: the shipped dangling-baseline message is unchanged",
    );
    assert_eq!(
        other["route"].as_str().expect("the control has a route"),
        format!(
            "prune the stale baseline: `jigc unmanage {OTHER_RECORD}`; or restore \
             {OTHER_RECORD} if it should still exist"
        ),
        "live-record/other: the shipped prune route is unchanged",
    );

    // The subject — the task's OWN milestone record.
    let own = rename_finding(&findings, OWN_RECORD, "live-record/own");
    assert_eq!(
        own["severity"], "advisory",
        "live-record/own: the live record stays advisory (no new code, no exit flip); \
         got:\n{own:#?}",
    );
    let route = own["route"]
        .as_str()
        .expect("the live record carries a route");
    assert!(
        !route.contains("jigc unmanage"),
        "live-record/own: the advisory must not route at `jigc unmanage` over the record the \
         milestone is run from; got:\n{own:#?}",
    );
    assert!(
        route.contains("do not prune") && route.contains("cache-rework"),
        "live-record/own: the route says what not to do and names the milestone; \
         got:\n{own:#?}",
    );
    assert!(
        own["message"]
            .as_str()
            .expect("the live record carries a message")
            .contains("live record of milestone cache-rework"),
        "live-record/own: the message states what the path is, not that it went missing; \
         got:\n{own:#?}",
    );
}
