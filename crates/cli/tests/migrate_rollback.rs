//! M23 Increment 3, T3 — extend `rollback_promotions` to restore retired paths
//! (review B1).
//!
//! The retire step ([auto-migration.md](../../../design/auto-migration.md) →
//! Retire-the-foreign-original) is the first byte-destructive write, run **inside** the
//! commit closure so the deletion stages into the same commit as the promoted managed
//! doc. The transaction is all-or-nothing: if the `git commit` is **rejected** (a
//! seeded `pre-commit` hook that exits non-zero — the M19 hook-rejection-rollback
//! idiom), nothing may land AND no byte may be lost. This drives the failure branch:
//!   - the commit fails non-zero and **no** commit lands (HEAD unchanged);
//!   - the foreign original is **restored byte-intact** on disk (never left
//!     deleted-with-no-commit — the B1 defect this task closes);
//!   - the promoted canonical copy is **also** rolled back (gone from disk).
//!
//! Drives the built `jigc` binary against a throwaway `git init` temp repo over the
//! shipped dev pack.

use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

/// A throwaway directory that removes itself on drop.
struct TempDir(PathBuf);

impl TempDir {
    fn new(tag: &str) -> Self {
        let mut path = std::env::temp_dir();
        let unique = format!(
            "jigc-rollback-{tag}-{}-{:?}",
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

/// The embedded dev pack tree on disk — selected via `JIGC_PACK_DIR`.
fn dev_pack() -> PathBuf {
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

/// Initialize a real git repo with one commit plus the `.jigc/config/` project layer.
fn init_repo(root: &Path) {
    git(root, &["init", "-q"]);
    git(root, &["config", "user.email", "test@example.com"]);
    git(root, &["config", "user.name", "Test"]);
    fs::write(root.join("README.md"), "hello\n").expect("write file");
    git(root, &["add", "."]);
    git(root, &["commit", "-q", "-m", "initial"]);
    fs::create_dir_all(root.join(".jigc").join("config")).expect("create project layer");
}

/// Run a `jigc` subcommand with `cwd = repo`, `$HOME = home`, `JIGC_PACK_DIR = pack`.
fn run_jigc(repo: &Path, home: &Path, pack: &Path, args: &[&str]) -> std::process::Output {
    let mut command = Command::new(env!("CARGO_BIN_EXE_jigc"));
    command
        .args(args)
        .current_dir(repo)
        .env("HOME", home)
        .env("JIGC_PACK_DIR", pack)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());
    command
        .spawn()
        .expect("spawn jigc")
        .wait_with_output()
        .expect("wait for jigc")
}

/// Run a `jigc` subcommand piping `stdin`.
fn run_jigc_stdin(
    repo: &Path,
    home: &Path,
    pack: &Path,
    args: &[&str],
    stdin: &[u8],
) -> std::process::Output {
    let mut child = Command::new(env!("CARGO_BIN_EXE_jigc"))
        .args(args)
        .current_dir(repo)
        .env("HOME", home)
        .env("JIGC_PACK_DIR", pack)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("spawn jigc");
    child
        .stdin
        .take()
        .expect("stdin piped")
        .write_all(stdin)
        .expect("write stdin");
    child.wait_with_output().expect("wait for jigc")
}

/// Assert a `jigc` invocation exits 0.
fn ok(out: std::process::Output, what: &str) {
    assert!(
        out.status.success(),
        "`{what}` must exit 0; stdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr),
    );
}

/// The off-router migration task id — `migrate` mints a per-file `migrate-<doctype>-<slug(path)>` (the empty
/// intent slugs the `migrate-` id-source fallback), keeping the bare `changelog`
/// namespace free (`auto-migration.md` -> Hardening #9).
const TASK: &str = "migrate-changelog-history-3268e06b69e1";

/// The single-release foreign file (the cold/empty spike input).
const FOREIGN: &str = "\
# Changelog

## [0.1.0] - 2021-03-09
### Added
- First public release.
";

/// Drive the full migrate + author spine to a conformant staged migration task over a
/// **committed** foreign `HISTORY.md`, plus a conformant commit doc.
fn committed_staged_migration(repo: &Path, home: &Path, pack: &Path) {
    staged_migration(repo, home, pack, true);
}

/// Drive the migrate + author spine over a foreign `HISTORY.md`. When `track_foreign`
/// the foreign is committed first (the design's "foreign committed first" assumption);
/// otherwise it is left **untracked** — the review-F3 case where `git restore` cannot
/// recover it on a rolled-back commit.
fn staged_migration(repo: &Path, home: &Path, pack: &Path, track_foreign: bool) {
    fs::write(repo.join("HISTORY.md"), FOREIGN).expect("write foreign HISTORY.md");
    if track_foreign {
        git(repo, &["add", "HISTORY.md"]);
        git(repo, &["commit", "-q", "-m", "track foreign changelog"]);
    }

    ok(run_jigc(repo, home, pack, &["setup"]), "jigc setup");
    ok(
        run_jigc(
            repo,
            home,
            pack,
            &["migrate", "HISTORY.md", "--as", "changelog"],
        ),
        "jigc migrate",
    );
    author_migrated_changelog(repo, home, pack, TASK);
}

/// Author the migrated changelog (the canonical single-release rewrite of [`FOREIGN`])
/// plus a conformant commit doc, all into `task`'s working area — the shared authoring
/// half of every rollback case, parameterized so a differently-sourced migration task
/// (the same-path case, whose id embeds its own path hash) can reuse it.
fn author_migrated_changelog(repo: &Path, home: &Path, pack: &Path, task: &str) {
    ok(
        run_jigc(
            repo,
            home,
            pack,
            &[
                "doc",
                "create",
                "changelog",
                "--title",
                "Changelog",
                "--task",
                task,
            ],
        ),
        "doc create changelog",
    );
    let release = String::from_utf8(
        run_jigc(
            repo,
            home,
            pack,
            &[
                "doc",
                "add-item",
                "changelog:changelog#releases",
                "--title",
                "0.1.0",
                "--task",
                task,
            ],
        )
        .stdout,
    )
    .expect("utf-8")
    .trim()
    .to_owned();
    ok(
        run_jigc(
            repo,
            home,
            pack,
            &[
                "doc",
                "set-field",
                &format!("{release}/date"),
                "--value",
                "2021-03-09",
                "--task",
                task,
            ],
        ),
        "set-field date",
    );
    let group = String::from_utf8(
        run_jigc(
            repo,
            home,
            pack,
            &[
                "doc",
                "add-item",
                &format!("{release}/changes"),
                "--title",
                "Added",
                "--task",
                task,
            ],
        )
        .stdout,
    )
    .expect("utf-8")
    .trim()
    .to_owned();
    ok(
        run_jigc_stdin(
            repo,
            home,
            pack,
            &[
                "doc",
                "set-slot",
                &format!("{group}/notes"),
                "--from-file",
                "-",
                "--task",
                task,
            ],
            b"First public release.\n",
        ),
        "set-slot notes",
    );
    // The provisioned commit doc.
    ok(
        run_jigc(
            repo,
            home,
            pack,
            &[
                "doc",
                "set-field",
                &format!("commit:{task}#type"),
                "--value",
                "feat",
                "--task",
                task,
            ],
        ),
        "set-field commit type",
    );
    ok(
        run_jigc(
            repo,
            home,
            pack,
            &[
                "doc",
                "set-field",
                &format!("commit:{task}#scope"),
                "--value",
                "changelog",
                "--task",
                task,
            ],
        ),
        "set-field commit scope",
    );
    ok(
        run_jigc_stdin(
            repo,
            home,
            pack,
            &[
                "doc",
                "set-slot",
                &format!("commit:{task}#summary"),
                "--from-file",
                "-",
                "--task",
                task,
            ],
            b"adopt the migrated changelog\n",
        ),
        "set-slot commit summary",
    );
    ok(
        run_jigc_stdin(
            repo,
            home,
            pack,
            &[
                "doc",
                "set-slot",
                &format!("commit:{task}#body"),
                "--from-file",
                "-",
                "--task",
                task,
            ],
            b"Migrate the foreign HISTORY.md into managed shape.\n",
        ),
        "set-slot commit body",
    );
}

/// Seed a `pre-commit` hook that always rejects (exits non-zero), made executable —
/// the M19 hook-rejection idiom. Overwrites any hook `jigc setup` installed.
fn seed_rejecting_precommit(repo: &Path) {
    let hooks = repo.join(".git").join("hooks");
    fs::create_dir_all(&hooks).expect("mk hooks dir");
    let hook = hooks.join("pre-commit");
    fs::write(&hook, "#!/bin/sh\necho REJECTING-HOOK >&2\nexit 1\n").expect("write hook");
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        fs::set_permissions(&hook, fs::Permissions::from_mode(0o755)).expect("chmod hook");
    }
}

#[test]
fn approved_migration_commit_rejection_rolls_back_retire() {
    let repo = TempDir::new("repo");
    let home = TempDir::new("home");
    let pack = dev_pack();
    init_repo(repo.path());
    committed_staged_migration(repo.path(), home.path(), &pack);

    let head_before = git(repo.path(), &["rev-parse", "HEAD"]);
    let count_before: u32 = git(repo.path(), &["rev-list", "--count", "HEAD"])
        .parse()
        .unwrap();

    // The rejecting hook fires on the finalize's `git commit` (never `--no-verify`).
    seed_rejecting_precommit(repo.path());

    let out = run_jigc(
        repo.path(),
        home.path(),
        &pack,
        &["task", "finalize", TASK, "--approve"],
    );

    // The commit is rejected: non-zero exit, nothing landed.
    assert!(
        !out.status.success(),
        "a hook-rejected --approve finalize must exit non-zero; stdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr),
    );
    assert_eq!(
        git(repo.path(), &["rev-parse", "HEAD"]),
        head_before,
        "HEAD must be unchanged — no commit landed",
    );
    assert_eq!(
        git(repo.path(), &["rev-list", "--count", "HEAD"])
            .parse::<u32>()
            .unwrap(),
        count_before,
        "no new commit",
    );

    // The B1 contract: the foreign original is restored BYTE-INTACT — never left
    // deleted-with-no-commit.
    let restored = repo.path().join("HISTORY.md");
    assert!(
        restored.exists(),
        "the foreign original must be restored on a rolled-back commit",
    );
    assert_eq!(
        fs::read_to_string(&restored).expect("read restored foreign"),
        FOREIGN,
        "the foreign original must be restored byte-intact",
    );
    // And its restoration reaches the index too (the staged deletion is undone), so the
    // worktree is clean of any pending HISTORY.md change.
    assert!(
        !git(repo.path(), &["status", "--porcelain", "HISTORY.md"]).contains("HISTORY.md"),
        "the foreign original's staged deletion must be rolled back (index + worktree)",
    );

    // The promoted canonical copy is also rolled back — gone from disk.
    assert!(
        !repo.path().join("changelog").join("changelog.md").exists(),
        "the promoted canonical copy must be rolled back on a failed commit",
    );
}

/// M40 F7 (design/finalize.md → M40 refinement item 1): a user who pre-staged the
/// retirement themselves (`git rm <foreign>` before finalize) must land clean. `git rm`
/// removes the path from the worktree AND the index while HEAD still carries it, so the
/// old HEAD discriminator kept the pathspec and the stage-phase `git add` fataled with
/// "did not match any files" (exit 1). Discriminated on the index instead, the pathspec
/// is skipped and nothing is lost: the commit is whole-index, so the user's staged
/// deletion rides the one migration commit alongside the promoted doc.
#[test]
fn approved_migration_with_pre_staged_git_rm_lands_one_commit() {
    let repo = TempDir::new("prestaged");
    let home = TempDir::new("home");
    let pack = dev_pack();
    init_repo(repo.path());
    committed_staged_migration(repo.path(), home.path(), &pack);

    let count_before: u32 = git(repo.path(), &["rev-list", "--count", "HEAD"])
        .parse()
        .unwrap();

    // The user pre-stages the retirement: worktree file gone, deletion in the index.
    git(repo.path(), &["rm", "-q", "HISTORY.md"]);

    let out = run_jigc(
        repo.path(),
        home.path(),
        &pack,
        &["task", "finalize", TASK, "--approve"],
    );
    ok(out, "jigc task finalize --approve (pre-staged git rm)");

    // Exactly one commit landed.
    assert_eq!(
        git(repo.path(), &["rev-list", "--count", "HEAD"])
            .parse::<u32>()
            .unwrap(),
        count_before + 1,
        "the pre-staged migration finalize must land exactly one commit",
    );

    // The whole-index commit carries the user's staged deletion AND the promoted doc.
    let name_status = git(repo.path(), &["show", "--name-status", "--format=", "HEAD"]);
    assert!(
        name_status.contains("D\tHISTORY.md"),
        "the landed commit must carry the foreign deletion; name-status:\n{name_status}",
    );
    assert!(
        name_status.contains("A\tCHANGELOG.md"),
        "the landed commit must carry the promoted canonical doc; name-status:\n{name_status}",
    );
}

/// The plain tracked-retire shape (no pre-staging — [`retire`] deletes the worktree file
/// and `stage_migration` stages the deletion) must not regress under the index
/// discriminator: the foreign is tracked-and-unmodified, so it is in the index either
/// way, and the deletion still lands in the one migration commit.
#[test]
fn approved_migration_with_tracked_foreign_still_stages_the_deletion() {
    let repo = TempDir::new("tracked");
    let home = TempDir::new("home");
    let pack = dev_pack();
    init_repo(repo.path());
    committed_staged_migration(repo.path(), home.path(), &pack);

    let count_before: u32 = git(repo.path(), &["rev-list", "--count", "HEAD"])
        .parse()
        .unwrap();

    let out = run_jigc(
        repo.path(),
        home.path(),
        &pack,
        &["task", "finalize", TASK, "--approve"],
    );
    ok(out, "jigc task finalize --approve (tracked foreign)");

    assert_eq!(
        git(repo.path(), &["rev-list", "--count", "HEAD"])
            .parse::<u32>()
            .unwrap(),
        count_before + 1,
        "the tracked-retire migration finalize must land exactly one commit",
    );
    let name_status = git(repo.path(), &["show", "--name-status", "--format=", "HEAD"]);
    assert!(
        name_status.contains("D\tHISTORY.md"),
        "the landed commit must carry the retired foreign deletion; name-status:\n{name_status}",
    );
    assert!(
        name_status.contains("A\tCHANGELOG.md"),
        "the landed commit must carry the promoted canonical doc; name-status:\n{name_status}",
    );
}

/// M40 F7 — the two-axis scoped rollback (`design/finalize.md` → Rollback discipline,
/// phase 6 commit (migration — the retire, M40)): a commit failure must restore **what
/// jigc deleted or staged — never what the user deleted**. Here the user pre-staged the
/// retirement (`git rm HISTORY.md`) and a rejecting `pre-commit` hook fails the commit:
/// the retire captured nothing (the worktree file was already gone) and the stage
/// skipped the pathspec (not in the index), so the rollback must leave the user's
/// deletion fully intact — worktree absent AND the deletion still staged (porcelain
/// X column `D`). Pre-fix, the unconditional per-planned-retirement `git restore
/// --staged --worktree` resurrected the file on both axes.
#[test]
fn pre_staged_git_rm_survives_a_hook_rejection_rollback() {
    let repo = TempDir::new("prestaged-rollback");
    let home = TempDir::new("home");
    let pack = dev_pack();
    init_repo(repo.path());
    committed_staged_migration(repo.path(), home.path(), &pack);

    // The user pre-stages the retirement: worktree file gone, deletion in the index.
    git(repo.path(), &["rm", "-q", "HISTORY.md"]);
    let head_before = git(repo.path(), &["rev-parse", "HEAD"]);

    seed_rejecting_precommit(repo.path());

    let out = run_jigc(
        repo.path(),
        home.path(),
        &pack,
        &["task", "finalize", TASK, "--approve"],
    );

    assert!(
        !out.status.success(),
        "a hook-rejected --approve finalize must exit non-zero; stdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr),
    );
    assert_eq!(
        git(repo.path(), &["rev-parse", "HEAD"]),
        head_before,
        "HEAD must be unchanged — no commit landed",
    );

    // The scoped-rollback contract: the user's pre-staged deletion is never restored.
    assert!(
        !repo.path().join("HISTORY.md").exists(),
        "the rollback must not resurrect the user's deleted worktree file",
    );
    let porcelain = git(repo.path(), &["status", "--porcelain", "HISTORY.md"]);
    assert!(
        porcelain.starts_with("D "),
        "the user's staged deletion must stay staged (porcelain X column D); \
         porcelain: {porcelain:?}",
    );

    // jigc's own acts ARE rolled back: the promoted canonical copy is gone.
    assert!(
        !repo.path().join("CHANGELOG.md").exists(),
        "the promoted canonical copy must be rolled back on a failed commit",
    );
}

/// M40 F7 dry-run/landed symmetry on the pre-staged shape: `predict_manifest` keeps the
/// HEAD discriminator, and its forecast stays outcome-accurate *because the commit is
/// whole-index* — the user's staged deletion lands regardless of the skipped stage
/// pathspec. This pins that claim as a test rather than trusting it: the `--dry-run`
/// forecast names the foreign as deleted, and the landed manifest carries the identical
/// entry.
#[test]
fn pre_staged_dry_run_forecast_matches_the_landed_manifest() {
    let repo = TempDir::new("dryrun");
    let home = TempDir::new("home");
    let pack = dev_pack();
    init_repo(repo.path());
    committed_staged_migration(repo.path(), home.path(), &pack);

    git(repo.path(), &["rm", "-q", "HISTORY.md"]);

    // A migration dry-run needs no `--approve` — it commits nothing.
    let dry = run_jigc(
        repo.path(),
        home.path(),
        &pack,
        &["task", "finalize", TASK, "--dry-run"],
    );
    let dry_stdout = String::from_utf8_lossy(&dry.stdout).to_string();
    ok(dry, "jigc task finalize --dry-run (pre-staged git rm)");
    assert!(
        dry_stdout.contains("  deleted HISTORY.md"),
        "the dry-run forecast must name the pre-staged foreign as deleted; stdout:\n{dry_stdout}",
    );

    let landed = run_jigc(
        repo.path(),
        home.path(),
        &pack,
        &["task", "finalize", TASK, "--approve"],
    );
    let landed_stdout = String::from_utf8_lossy(&landed.stdout).to_string();
    ok(landed, "jigc task finalize --approve (pre-staged git rm)");
    assert!(
        landed_stdout.contains("  deleted HISTORY.md"),
        "the landed manifest must carry the same deleted entry the forecast named; stdout:\n{landed_stdout}",
    );
    assert!(
        git(repo.path(), &["show", "--name-status", "--format=", "HEAD"]).contains("D\tHISTORY.md"),
        "the landed commit must actually carry the forecast deletion",
    );
}

/// M40 F7 (`design/finalize.md` → M40 refinement item 2): a git failure during jigc's
/// OWN stage phase (`stage_migration`'s `git add`) surfaces as a **routed blocking
/// finding** through the findings envelope — a `finalize.*` code + a route + the
/// verbatim git stderr embedded in the message — never the raw enveloped operational
/// error (exit 1, no code, no route) it was pre-fix. The rollback still runs before the
/// routed surface: no commit, the promoted copy gone, the retire-deleted foreign
/// restored byte-intact. A seeded stale `.git/index.lock` makes the stage `git add`
/// fail deterministically (the preflight git commands are read-only and tolerate it).
/// Item 3's counterpart — the commit-phase hook rejection staying verbatim-raw — is
/// pinned by the existing rejection tests above, unchanged.
#[test]
fn stage_phase_git_failure_yields_a_routed_finding_and_rolls_back() {
    let repo = TempDir::new("stagefail");
    let home = TempDir::new("home");
    let pack = dev_pack();
    init_repo(repo.path());
    committed_staged_migration(repo.path(), home.path(), &pack);

    let head_before = git(repo.path(), &["rev-parse", "HEAD"]);

    // Seed the stale lock: the stage-phase `git add` needs `.git/index.lock` and fails.
    fs::write(repo.path().join(".git").join("index.lock"), "").expect("seed index.lock");

    let out = run_jigc(
        repo.path(),
        home.path(),
        &pack,
        &["task", "finalize", TASK, "--approve"],
    );
    let stderr = String::from_utf8_lossy(&out.stderr).to_string();

    // The routed surface: the validation-blocked exit (3, the findings envelope) —
    // not the raw operational exit (1) the pre-fix path took.
    assert_eq!(
        out.status.code(),
        Some(3),
        "a stage-phase git failure must exit via the findings envelope (3); stdout:\n{}\nstderr:\n{stderr}",
        String::from_utf8_lossy(&out.stdout),
    );
    assert!(
        stderr.contains("finalize.stage-failed"),
        "the finding must carry the finalize.* code; stderr:\n{stderr}",
    );
    assert!(
        stderr.contains("route:"),
        "the finding must carry a route; stderr:\n{stderr}",
    );
    assert!(
        stderr.contains("index.lock"),
        "the git stderr must be embedded verbatim in the finding; stderr:\n{stderr}",
    );

    // The rollback ran before the routed surface.
    assert_eq!(
        git(repo.path(), &["rev-parse", "HEAD"]),
        head_before,
        "no commit landed",
    );
    assert!(
        !repo.path().join("CHANGELOG.md").exists(),
        "the promoted canonical copy must be rolled back",
    );
    let restored = repo.path().join("HISTORY.md");
    assert!(
        restored.exists(),
        "the retire-deleted foreign must be restored on the stage-failure rollback",
    );
    assert_eq!(
        fs::read_to_string(&restored).expect("read restored foreign"),
        FOREIGN,
        "the foreign original must be restored byte-intact",
    );
}

/// Review F3: an **untracked** foreign original (never committed at HEAD) + a seeded
/// commit failure must still leave the foreign file byte-intact on disk. `git restore`
/// alone cannot recover an untracked file — there are no committed bytes — so the retire
/// must capture the bytes pre-deletion and the rollback must rewrite them. Pre-fix the
/// foreign was lost permanently.
#[test]
fn approved_migration_commit_rejection_restores_an_untracked_foreign() {
    let repo = TempDir::new("untracked");
    let home = TempDir::new("home");
    let pack = dev_pack();
    init_repo(repo.path());
    // The foreign HISTORY.md is staged for migration but never committed — untracked.
    staged_migration(repo.path(), home.path(), &pack, false);

    // Sanity: the foreign is genuinely untracked at HEAD (the precondition under test).
    assert!(
        git(repo.path(), &["status", "--porcelain", "HISTORY.md"]).starts_with("??"),
        "the foreign HISTORY.md must be untracked for this case",
    );
    let head_before = git(repo.path(), &["rev-parse", "HEAD"]);

    seed_rejecting_precommit(repo.path());

    let out = run_jigc(
        repo.path(),
        home.path(),
        &pack,
        &["task", "finalize", TASK, "--approve"],
    );

    assert!(
        !out.status.success(),
        "a hook-rejected --approve finalize must exit non-zero; stdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr),
    );
    assert_eq!(
        git(repo.path(), &["rev-parse", "HEAD"]),
        head_before,
        "HEAD must be unchanged — no commit landed",
    );

    // The F3 contract: the untracked foreign original is restored BYTE-INTACT — never
    // permanently lost just because it was never committed.
    let restored = repo.path().join("HISTORY.md");
    assert!(
        restored.exists(),
        "the untracked foreign original must be restored on a rolled-back commit",
    );
    assert_eq!(
        fs::read_to_string(&restored).expect("read restored foreign"),
        FOREIGN,
        "the untracked foreign original must be restored byte-intact",
    );

    // The promoted canonical copy is rolled back — gone from disk.
    assert!(
        !repo.path().join("changelog").join("changelog.md").exists(),
        "the promoted canonical copy must be rolled back on a failed commit",
    );
}

/// Confidence-audit minor item 10 — the promotions rollback's **worktree** half has a
/// same-path sibling: a **same-path** migration (`destination == source`, the M43
/// carve-out) of an **untracked** foreign plans **no retirement**
/// (`plan_retirements` skips `source == destination`), so the retire byte-capture that
/// saves an untracked foreign in the two-path case never runs — and the promotion
/// destination IS the foreign original. On a hook-rejected `--approve` finalize the
/// worktree rollback saw an untracked destination and **deleted** it, destroying the
/// user's only worktree copy of the foreign bytes. The rollback must instead restore
/// the displaced pre-promote bytes — the same capture discipline as the retire axis.
#[test]
fn same_path_untracked_foreign_survives_a_hook_rejection_rollback() {
    let repo = TempDir::new("same-path");
    let home = TempDir::new("home");
    let pack = dev_pack();
    init_repo(repo.path());

    // The foreign sits AT the canonical destination (`changelog`'s placement is root
    // `CHANGELOG.md`) and is never committed — the same-path × untracked cell.
    fs::write(repo.path().join("CHANGELOG.md"), FOREIGN).expect("write foreign CHANGELOG.md");
    ok(
        run_jigc(repo.path(), home.path(), &pack, &["setup"]),
        "jigc setup",
    );
    let composed = run_jigc(
        repo.path(),
        home.path(),
        &pack,
        &["migrate", "CHANGELOG.md", "--as", "changelog"],
    );
    assert!(
        composed.status.success(),
        "`jigc migrate CHANGELOG.md` must compose; stderr:\n{}",
        String::from_utf8_lossy(&composed.stderr),
    );
    // The task id embeds this source path's own hash — parse it from the mint line.
    let task = String::from_utf8_lossy(&composed.stdout)
        .lines()
        .find_map(|l| l.strip_prefix("task minted: ").map(str::to_owned))
        .expect("the migrate compose announces the minted task id");
    author_migrated_changelog(repo.path(), home.path(), &pack, &task);

    // Sanity: the foreign is genuinely untracked (the cell under test).
    assert!(
        git(repo.path(), &["status", "--porcelain", "CHANGELOG.md"]).starts_with("??"),
        "the foreign CHANGELOG.md must be untracked for this case",
    );
    let head_before = git(repo.path(), &["rev-parse", "HEAD"]);

    seed_rejecting_precommit(repo.path());

    let out = run_jigc(
        repo.path(),
        home.path(),
        &pack,
        &["task", "finalize", &task, "--approve"],
    );
    assert!(
        !out.status.success(),
        "a hook-rejected --approve finalize must exit non-zero; stdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr),
    );
    assert_eq!(
        git(repo.path(), &["rev-parse", "HEAD"]),
        head_before,
        "HEAD must be unchanged — no commit landed",
    );

    // The contract: the untracked foreign original — displaced in place by the
    // promotion — is restored BYTE-INTACT, never deleted by the worktree rollback.
    let restored = repo.path().join("CHANGELOG.md");
    assert!(
        restored.exists(),
        "the same-path untracked foreign must survive a rolled-back commit — the \
         worktree rollback must restore the displaced bytes, not delete the destination",
    );
    assert_eq!(
        fs::read_to_string(&restored).expect("read restored foreign"),
        FOREIGN,
        "the same-path untracked foreign must be restored byte-intact",
    );
}
