//! **`jigc setup` refuses its install commit over bytes it did not write** (M51
//! Increment 3 / T2; `completions/artifacts/M51/settle-record.md` → D3 with its §1
//! amendment and §5 · §10; `implementation/roadmap.md` → Milestone 51 Increment 3).
//!
//! The first command an adopter runs used to commit the user's uncommitted work into
//! `chore(jigc): install jigc workspace config` at exit 0. Driven at `61f05c01` on a
//! repo whose `CLAUDE.md` carried an edit that was **never staged at all**: `jigc setup`
//! exited 0, `git show HEAD:CLAUDE.md` carried the user's line, and `git status --short`
//! was **empty** — so nothing prompted recovery.
//!
//! **The predicate, stated:** for each path in `setup`'s would-be pathspec, asked
//! **before the first write** — dirty iff its **index or worktree** bytes differ from
//! `HEAD`, git holding no copy at all included. Mechanically one `git status --porcelain
//! --untracked-files=all` (`=no` on an unborn `HEAD`, where the exemption below applies)
//! whose answer is intersected with the pathspec the door actually settles on.
//!
//! **Both axes, not worktree-only.** A pathspec commit takes the **worktree** contents,
//! which is why `decide_carryover`'s index-vs-HEAD snapshot pair is not this door's
//! predicate (§1) — but an **index-only** difference (`MM` with worktree == `HEAD`) was
//! *committed away* at exit 0 too, and the staged blob then ended up named by no commit
//! and no index entry. Cell (3) is that loss.
//!
//! **Untracked is a subject on a born `HEAD`** (cells 8, 17 — the M51 completion audit):
//! git holds no copy of an untracked file at all, so it is dirty relative to `HEAD` under
//! the predicate as settled, and `setup` *merges into* `CLAUDE.md`, so the adopter's prose
//! is still there when the commit is made. The `??` exclusion this suite first shipped
//! read M30 audit finding 1 — *`setup` owns making its install footprint **tracked*** —
//! past its subject: that is about `setup`'s own files. **On an unborn `HEAD` it stays a
//! green cell** (cells 4, 18), where it is the M30 rationale rather than a status flag that
//! carries it — there is no `HEAD` to be dirty against, and `setup` owns minting the first
//! commit — so Increment 2's `SETUP_UNBORN_EXEMPTION` is not re-closed through the back
//! door.
//!
//! **The refusal is asked before the first write, so it installs nothing** (corrected by
//! the M51 completion audit; the roadmap's *"the eight install paths stay written and
//! staged"* sentence is struck with it). The guard first shipped as a *conjunction* —
//! dirty before the install **and** still dirty after it — which is structurally blind to
//! every path the install **rewrites whole**: `.jigc/AGENT.md` matched `HEAD` again by the
//! time the second leg was asked *because* the adopter's prose had just been destroyed, so
//! the set came back empty at exit 0 with `git status` empty and the bytes in no git
//! object. Asking once, before any write, is what makes the refusal's own sentence — *every
//! path listed above still has its pre-run bytes there* — true (cells 19–20).
//!
//! **The commit-time ask survives as a backstop** over the one pathspec member the
//! pre-write enumeration cannot name (the hook, whose home the install itself resolves).
//! Where it fires, the install *is* written and staged and the finding says so; it also
//! does not stage a path it has just decided not to commit, since staging the `MM` cell's
//! path would replace exactly the index blob the guard exists to save.
//!
//! **The default is now *refuse*.** An install path is exempt only when an oracle says the
//! bytes there are **jigc's own** — the guide's recorded digest, the version stamp's shape
//! (`cli::setup::InstallPathDisposition`). The old derivation classified a forgotten path
//! as *exempt*, which is how two of eleven paths swept authored bytes; this one classifies
//! it as a refusal, so forgetting costs a loud false alarm and never a loss. Cell 21 says
//! what `--force` buys, and cell 22 fences the class member by member.
//!
//! **And what it leaves behind is its own, on the next run too** (cells 13–16, the
//! completion audit's fix). Every arm that writes the install and commits none of it —
//! this refusal, a `git add` or `git commit` git declines, the posture re-probe — leaves
//! `setup`'s bytes staged and on disk, and the pre-write probe read them on the next run
//! as the adopter's work: the re-run refused over jigc's *own* install files, the emitted
//! route could not clear it, and the only exits were `--force` (which then swept the very
//! bytes the guard had refused) or a hand `git reset`. The uncommitted footprint is
//! recorded where the pathspec settles and re-verified — bytes **and** index — where the
//! question is asked, so the exemption can only ever cover bytes `setup` still wrote.
//!
//! Twenty-two cells, all through the real binary (`CARGO_BIN_EXE_jigc`) over throwaway
//! `git init` repos.

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

/// The blocking code the door refuses with — `CarryoverBoundary::Setup`'s own identity
/// (`crates/engine/src/finalize.rs` → `setup_dirty_install_finding`).
const DIRTY_CODE: &str = "setup.dirty-install-path";

/// A throwaway directory that removes itself on drop.
struct TempDir(PathBuf);

impl TempDir {
    fn new(tag: &str) -> Self {
        let mut path = std::env::temp_dir();
        path.push(format!(
            "jigc-setup-pathspec-{tag}-{}-{:?}",
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

/// Run `git` in `repo`, asserting success, returning trimmed stdout.
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
    String::from_utf8_lossy(&out.stdout).trim().to_string()
}

/// Run `git` in `repo` without asserting — for the queries whose failure is the answer.
fn git_try(repo: &Path, args: &[&str]) -> std::process::Output {
    Command::new("git")
        .args(args)
        .current_dir(repo)
        .output()
        .expect("run git")
}

/// A real `git init` with a per-repo identity and **no** commit yet.
fn unborn_repo(tag: &str) -> (TempDir, TempDir) {
    let repo = TempDir::new(tag);
    let home = TempDir::new(&format!("{tag}-home"));
    git(repo.path(), &["init", "-q"]);
    git(repo.path(), &["config", "user.email", "test@example.com"]);
    git(repo.path(), &["config", "user.name", "Test"]);
    (repo, home)
}

/// [`unborn_repo`] plus a seed commit — a **born** `HEAD`, the state every cell but (4)
/// starts from.
fn born_repo(tag: &str) -> (TempDir, TempDir) {
    let (repo, home) = unborn_repo(tag);
    write(repo.path(), "README.md", "hello\n");
    git(repo.path(), &["add", "README.md"]);
    git(repo.path(), &["commit", "-q", "-m", "initial"]);
    (repo, home)
}

/// Write `contents` at `relative` under `repo`, creating parent directories.
fn write(repo: &Path, relative: &str, contents: &str) {
    let path = repo.join(relative);
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).expect("create parent");
    }
    fs::write(path, contents).expect("write file");
}

/// Read `relative` under `repo`.
fn read(repo: &Path, relative: &str) -> String {
    fs::read_to_string(repo.join(relative)).unwrap_or_default()
}

/// Run `jigc <args>` with `cwd = repo` and a temp `$HOME`, with any developer-shell
/// `JIGC_PACK_DIR` removed so the run composes the **embedded** packs.
fn jigc(repo: &Path, home: &Path, args: &[&str]) -> std::process::Output {
    Command::new(env!("CARGO_BIN_EXE_jigc"))
        .args(args)
        .current_dir(repo)
        .env("HOME", home)
        .env_remove("JIGC_PACK_DIR")
        .output()
        .expect("run jigc")
}

/// stdout + stderr of an invocation, joined for message assertions.
fn said(out: &std::process::Output) -> String {
    format!(
        "{}{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr)
    )
}

/// The repo-relative paths currently staged (index vs `HEAD`).
fn staged(repo: &Path) -> Vec<String> {
    git(repo, &["diff", "--cached", "--name-only"])
        .lines()
        .map(str::to_string)
        .collect()
}

/// (1) A **tracked-and-modified** install path refuses the install commit: exit 1 with
/// `setup.dirty-install-path`, `HEAD` unmoved, **nothing written** (the question is asked
/// before the first write), and the adopter's bytes still on disk and still visible in
/// `git status` — the recovery prompt the driven exit-0 sweep left nowhere.
#[test]
fn a_tracked_modified_install_path_refuses_the_install_commit() {
    let (repo, home) = born_repo("dirty");
    let (repo, home) = (repo.path(), home.path());
    write(repo, "CLAUDE.md", "user line one\n");
    git(repo, &["add", "CLAUDE.md"]);
    git(repo, &["commit", "-q", "-m", "add CLAUDE.md"]);
    // The driven cell: an edit that was never staged at all.
    write(repo, "CLAUDE.md", "user line one\nUNSTAGED WIP SECRET\n");
    let head_before = git(repo, &["rev-parse", "HEAD"]);

    let out = jigc(repo, home, &["setup"]);
    let said = said(&out);
    assert_eq!(
        out.status.code(),
        Some(1),
        "a dirty install path refuses the install commit at exit 1: {said}"
    );
    assert!(
        said.contains(DIRTY_CODE),
        "the refusal carries the door's own code `{DIRTY_CODE}`: {said}"
    );
    assert!(
        said.contains("CLAUDE.md"),
        "the refusal names the path it refuses over: {said}"
    );
    assert!(
        said.contains("jigc setup --force"),
        "the route names `--force` as the single consent: {said}"
    );

    assert_eq!(
        git(repo, &["rev-parse", "HEAD"]),
        head_before,
        "no install commit was made — HEAD is where it was"
    );
    // The question is asked before the first write, so the refusal installed nothing —
    // which is what lets the message claim every named path still has its pre-run bytes.
    assert!(
        !repo.join(".jigc/AGENT.md").exists(),
        "the refusal is asked BEFORE any write, so no install file was written"
    );
    let staged = staged(repo);
    assert!(
        staged.is_empty(),
        "and nothing was staged either: {staged:?}"
    );
    // The adopter's bytes: on disk, and visible.
    assert!(
        read(repo, "CLAUDE.md").contains("UNSTAGED WIP SECRET"),
        "the user's bytes are still on disk"
    );
    let status = git(repo, &["status", "--short"]);
    assert!(
        status.lines().any(|line| line.ends_with("CLAUDE.md")),
        "the user's bytes are still visible in `git status` — something prompts recovery: {status}"
    );
    assert!(
        !git(repo, &["show", "HEAD:CLAUDE.md"]).contains("UNSTAGED WIP SECRET"),
        "the user's line rode no commit"
    );
}

/// (2) `--force` is the **single consent**: the same repo installs and exits 0, with the
/// adopter's bytes deliberately in the install commit.
#[test]
fn force_is_the_single_consent_and_installs() {
    let (repo, home) = born_repo("forced");
    let (repo, home) = (repo.path(), home.path());
    write(repo, "CLAUDE.md", "user line one\n");
    git(repo, &["add", "CLAUDE.md"]);
    git(repo, &["commit", "-q", "-m", "add CLAUDE.md"]);
    write(repo, "CLAUDE.md", "user line one\nUNSTAGED WIP SECRET\n");
    let head_before = git(repo, &["rev-parse", "HEAD"]);

    let out = jigc(repo, home, &["setup", "--force"]);
    assert_eq!(
        out.status.code(),
        Some(0),
        "`--force` consents and installs: {}",
        said(&out)
    );
    assert_ne!(
        git(repo, &["rev-parse", "HEAD"]),
        head_before,
        "the install commit landed"
    );
    assert!(
        git(repo, &["show", "HEAD:CLAUDE.md"]).contains("UNSTAGED WIP SECRET"),
        "`--force` commits those paths as they stand — that is what the consent spends"
    );
}

/// (3) The **index-only** cell (`MM`, worktree == `HEAD`): the staged blob exists in no
/// commit and in no worktree file, so committing it away loses it outright. The door
/// refuses — and does not restage the path, so the blob is still in the index.
#[test]
fn an_index_only_difference_refuses_and_keeps_the_staged_blob() {
    let (repo, home) = born_repo("index-only");
    let (repo, home) = (repo.path(), home.path());
    write(repo, "CLAUDE.md", "base\n");
    git(repo, &["add", "CLAUDE.md"]);
    git(repo, &["commit", "-q", "-m", "add CLAUDE.md"]);
    // Stage bytes that exist nowhere else, then put the worktree back to HEAD.
    write(repo, "CLAUDE.md", "base\nSTAGED ONLY\n");
    git(repo, &["add", "CLAUDE.md"]);
    write(repo, "CLAUDE.md", "base\n");
    assert_eq!(
        git(repo, &["status", "--porcelain", "--", "CLAUDE.md"]),
        "MM CLAUDE.md",
        "the cell is index-differs AND worktree-differs-from-index"
    );
    let head_before = git(repo, &["rev-parse", "HEAD"]);

    let out = jigc(repo, home, &["setup"]);
    assert_eq!(
        out.status.code(),
        Some(1),
        "an index-only difference is dirty relative to HEAD too: {}",
        said(&out)
    );
    assert!(said(&out).contains(DIRTY_CODE));
    assert_eq!(
        git(repo, &["rev-parse", "HEAD"]),
        head_before,
        "the staged blob was not committed away"
    );
    assert!(
        git(repo, &["cat-file", "-p", ":CLAUDE.md"]).contains("STAGED ONLY"),
        "the staged blob is still the index entry — the refusal did not restage over it"
    );
}

/// (4) A **fresh install on an unborn `HEAD`** is clean: there is no `HEAD` for a
/// pre-existing file to be dirty against, and `setup` owns minting the repo's first commit
/// with its install footprint (Increment 2's `SETUP_UNBORN_EXEMPTION`, untouched). Cell
/// (18) is the same claim with the plant at an **install** path.
#[test]
fn a_fresh_install_on_an_unborn_head_is_clean() {
    let (repo, home) = unborn_repo("unborn");
    let (repo, home) = (repo.path(), home.path());
    write(repo, "notes.md", "pre-existing, untracked\n");

    let out = jigc(repo, home, &["setup"]);
    assert_eq!(
        out.status.code(),
        Some(0),
        "a fresh install on an unborn HEAD installs: {}",
        said(&out)
    );
    assert!(
        git_try(repo, &["rev-parse", "--verify", "-q", "HEAD"])
            .status
            .success(),
        "setup minted the repo's first commit"
    );
}

/// (5) A **re-run over an unchanged install** is a clean no-op: exit 0 and no second
/// commit — the re-run from a **committed** install, which is the case *"the question is
/// asked before any write"* covers on its own.
///
/// It is deliberately not the whole re-run story, and saying so is the point: this cell
/// once carried that premise as a universal (*"nothing `setup` itself wrote can enter the
/// answer"*), which is true within one invocation and **false across** them — the re-run
/// an adopter actually reaches after the guard fires is cell (13)'s, and it was bricked
/// while this cell was green.
#[test]
fn a_rerun_over_an_unchanged_install_makes_no_second_commit() {
    let (repo, home) = born_repo("rerun");
    let (repo, home) = (repo.path(), home.path());
    assert_eq!(
        jigc(repo, home, &["setup"]).status.code(),
        Some(0),
        "the first install"
    );
    let head_after_install = git(repo, &["rev-parse", "HEAD"]);

    let out = jigc(repo, home, &["setup"]);
    assert_eq!(
        out.status.code(),
        Some(0),
        "the re-run is a clean no-op: {}",
        said(&out)
    );
    assert_eq!(
        git(repo, &["rev-parse", "HEAD"]),
        head_after_install,
        "no second install commit"
    );
}

/// (6) An **upgrade** — the install is committed and `.jigc/version` records a different
/// binary — installs clean: the stamp matched `HEAD` when the question was asked, and
/// the rewrite that follows is `setup`'s own.
#[test]
fn an_upgrade_over_a_committed_install_is_clean() {
    let (repo, home) = born_repo("upgrade");
    let (repo, home) = (repo.path(), home.path());
    assert_eq!(jigc(repo, home, &["setup"]).status.code(), Some(0));
    // A prior binary's stamp, committed — the state a later binary meets.
    write(repo, ".jigc/version", "0.0.1-earlier\n");
    git(repo, &["add", ".jigc/version"]);
    git(repo, &["commit", "-q", "-m", "an earlier binary's stamp"]);
    let head_before = git(repo, &["rev-parse", "HEAD"]);

    let out = jigc(repo, home, &["setup"]);
    assert_eq!(
        out.status.code(),
        Some(0),
        "the upgrade installs: {}",
        said(&out)
    );
    assert_ne!(
        git(repo, &["rev-parse", "HEAD"]),
        head_before,
        "the refreshed stamp landed in an install commit"
    );
    assert!(
        !read(repo, ".jigc/version").contains("0.0.1-earlier"),
        "the stamp is this binary's"
    );
}

/// (7) An **unrelated staged file** is not in the pathspec, so it is neither a subject of
/// the refusal nor swept into the install commit — the pathspec scoping this door has
/// always had, pinned against the new guard.
#[test]
fn an_unrelated_staged_file_is_untouched() {
    let (repo, home) = born_repo("unrelated");
    let (repo, home) = (repo.path(), home.path());
    write(repo, "feature.txt", "work in progress\n");
    git(repo, &["add", "feature.txt"]);

    let out = jigc(repo, home, &["setup"]);
    assert_eq!(
        out.status.code(),
        Some(0),
        "an unrelated staged path raises no refusal: {}",
        said(&out)
    );
    assert!(
        !git(repo, &["show", "--name-only", "--format=", "HEAD"])
            .lines()
            .any(|p| p == "feature.txt"),
        "it is not in the install commit"
    );
    assert!(
        staged(repo).iter().any(|p| p == "feature.txt"),
        "it is still staged, exactly as the user left it"
    );
}

/// (8) An **untracked** pre-existing install-path file on a born `HEAD` **refuses** — the
/// third member of the dirtiness axis, closed with its two siblings below (cells 17, 18).
/// An untracked file is dirty relative to `HEAD` — git holds no copy of it at all — and
/// `setup` merges into `CLAUDE.md` rather than rewriting it, so the adopter's bytes are
/// still there at commit time. The `??` exclusion this cell used to pin was a narrowing of
/// the settled predicate (`settle-record.md` §1 — *"any path in `setup`'s pathspec is
/// dirty relative to HEAD before `setup` writes"*) that the M30 rationale does not reach:
/// *setup owns making its install footprint tracked* is about `setup`'s **own** files, and
/// an adopter's prose at a tracked-capable path in a born repo is not one of them.
#[test]
fn an_untracked_install_path_refuses_the_install_commit() {
    let (repo, home) = born_repo("untracked");
    let (repo, home) = (repo.path(), home.path());
    write(repo, "CLAUDE.md", "the adopter's own prose\n");
    let head_before = git(repo, &["rev-parse", "HEAD"]);

    let out = jigc(repo, home, &["setup"]);
    let first = said(&out);
    assert_eq!(
        out.status.code(),
        Some(1),
        "an untracked install path carries bytes setup did not write: {first}"
    );
    assert!(
        first.contains(DIRTY_CODE) && first.contains("CLAUDE.md"),
        "the refusal names the code and the path: {first}"
    );
    assert_eq!(
        git(repo, &["rev-parse", "HEAD"]),
        head_before,
        "HEAD did not move"
    );
    assert!(
        !git(repo, &["log", "--all", "-p"]).contains("the adopter's own prose"),
        "the adopter's prose is in no commit"
    );
    assert!(
        read(repo, "CLAUDE.md").contains("the adopter's own prose"),
        "and is still on disk — the guard binds the commit, not the install"
    );
    assert!(
        staged(repo).is_empty(),
        "and nothing was written or staged — the question is asked before the first write"
    );

    // And the re-run over the unresolved refusal names the adopter's one path, never an
    // install file (cell 14's claim on this axis).
    let out = jigc(repo, home, &["setup"]);
    let second = said(&out);
    assert_eq!(out.status.code(), Some(1), "still refused: {second}");
    assert!(
        second.contains("1 path(s)") && second.contains("CLAUDE.md"),
        "the subject is the adopter's one path, as before: {second}"
    );
    assert!(
        !second.contains(".jigc/AGENT.md"),
        "and never setup's own install files: {second}"
    );
    assert!(
        !repo.join(".jigc/AGENT.md").exists(),
        "still nothing installed after the second refusal"
    );

    // The route, verbatim — and it must *run* on this class: a bare `git stash push --
    // <path>` exits 1 over a path git holds no copy of, which is why the route names `-u`.
    let stash = git_try(repo, &["stash", "push", "-u", "-q", "--", "CLAUDE.md"]);
    assert!(
        stash.status.success(),
        "the act the route names runs: {}",
        String::from_utf8_lossy(&stash.stderr)
    );
    let out = jigc(repo, home, &["setup"]);
    assert_eq!(
        out.status.code(),
        Some(0),
        "and it restores a runnable `jigc setup`: {}",
        said(&out)
    );
    assert!(
        git(repo, &["show", "--name-only", "--format=", "HEAD"])
            .lines()
            .any(|p| p == ".jigc/AGENT.md"),
        "the install commit landed"
    );
    assert!(
        git(repo, &["show", "stash@{0}^3:CLAUDE.md"]).contains("the adopter's own prose"),
        "and the adopter's stashed prose is still theirs to pop (`^3` — the untracked \
         commit a `-u` stash carries)"
    );
}

/// (9) A **gitignored** install path is not a subject: the pathspec already drops it, and
/// the answer never reported it either.
#[test]
fn a_gitignored_install_path_is_not_a_subject() {
    let (repo, home) = born_repo("ignored");
    let (repo, home) = (repo.path(), home.path());
    write(repo, ".gitignore", "CLAUDE.md\n");
    git(repo, &["add", ".gitignore"]);
    git(repo, &["commit", "-q", "-m", "ignore CLAUDE.md"]);
    write(repo, "CLAUDE.md", "ignored, and the user's\n");

    let out = jigc(repo, home, &["setup"]);
    assert_eq!(
        out.status.code(),
        Some(0),
        "a gitignored install path raises no refusal: {}",
        said(&out)
    );
    assert!(
        !git(repo, &["show", "--name-only", "--format=", "HEAD"])
            .lines()
            .any(|p| p == "CLAUDE.md"),
        "and it is in no install commit"
    );
}

/// (10) The **rejecting-hook** cell — §5's falsifying datum, closed. `setup` preserves a
/// pre-existing `pre-commit` hook and then commits with `--no-verify`, so the hook that
/// would have refused the adopter's secret never ran. The refusal closes the cell the
/// struck half of that rationale was covering: the secret rides no commit.
#[test]
fn a_rejecting_hook_no_longer_has_a_secret_swept_past_it() {
    let (repo, home) = born_repo("hook");
    let (repo, home) = (repo.path(), home.path());
    write(repo, "CLAUDE.md", "project notes\n");
    git(repo, &["add", "CLAUDE.md"]);
    git(repo, &["commit", "-q", "-m", "add CLAUDE.md"]);
    // The adopter's own policy hook, present before jigc.
    write(
        repo,
        ".git/hooks/pre-commit",
        "#!/bin/sh\nif git diff --cached | grep -q AWS_SECRET; then\n  echo 'refusing: secret'\n  exit 1\nfi\nexit 0\n",
    );
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let hook = repo.join(".git/hooks/pre-commit");
        let mut perms = fs::metadata(&hook).expect("hook metadata").permissions();
        perms.set_mode(0o755);
        fs::set_permissions(&hook, perms).expect("chmod hook");
    }
    write(repo, "CLAUDE.md", "project notes\nAWS_SECRET=hunter2\n");
    let head_before = git(repo, &["rev-parse", "HEAD"]);

    let out = jigc(repo, home, &["setup"]);
    assert_eq!(
        out.status.code(),
        Some(1),
        "the secret's path is dirty, so the install commit refuses: {}",
        said(&out)
    );
    assert!(said(&out).contains(DIRTY_CODE));
    assert_eq!(
        git(repo, &["rev-parse", "HEAD"]),
        head_before,
        "HEAD did not move"
    );
    let every_commit = git(repo, &["log", "--all", "-p"]);
    assert!(
        !every_commit.contains("AWS_SECRET"),
        "the adopter's secret is in no commit — the `--no-verify` install commit can no \
         longer sweep past the hook that would have refused it"
    );
}

/// (11) A **regenerated artifact is not a subject**, via the second leg: jigc's own guide
/// copy, stamped by an earlier build, is dirty against `HEAD` before the install — and the
/// install rewrites it whole, so what would be committed is this build's canonical bytes
/// and nothing of the user's rides along. Refusing here would block an upgrade over jigc's
/// own file, and it would contradict M48 Increment 10's shipped decision that a copy jigc
/// owns is replaced. The guard composed into a context that omits its target: inert.
#[test]
fn an_owned_guide_artifact_is_not_a_subject_of_the_guard() {
    const GUIDE: &str = ".claude/skills/jigc/SKILL.md";
    let (repo, home) = born_repo("guide");
    let (repo, home) = (repo.path(), home.path());
    assert_eq!(jigc(repo, home, &["setup"]).status.code(), Some(0));
    assert!(
        git(repo, &["ls-files", GUIDE]).contains("SKILL.md"),
        "the premise: the first install committed the guide artifact"
    );
    // jigc's own bytes, stamped by an earlier build — tracked and modified against HEAD.
    let owned = read(repo, GUIDE).replacen("jigc-version:", "jigc-version: 0.0.1 #", 1);
    write(repo, GUIDE, &owned);

    let out = jigc(repo, home, &["setup"]);
    assert_eq!(
        out.status.code(),
        Some(0),
        "an owned guide artifact is jigc's to replace, not a refusal: {}",
        said(&out)
    );
    assert!(
        !read(repo, GUIDE).contains("0.0.1 #"),
        "and the install replaced it with this build's artifact"
    );
}

/// (12) **The discriminator's own axis**, in one repo: `setup` regenerates `.jigc/version`
/// whole and **preserves** a `.jigc/.gitignore` that already carries its floor, so the same
/// pre-install dirtiness refuses over one and not the other — and the rule is *derived*
/// from asking git the same question twice (before the install, and over the settled
/// pathspec after it), never from a per-path ownership list that a new install path could
/// be forgotten out of.
///
/// This is the cell that keeps the guard honest in both directions: exempting too much
/// sweeps user bytes, exempting too little blocks the documented `jigc setup` re-stamp
/// recovery over a file jigc wrote itself.
#[test]
fn a_regenerated_artifact_is_exempt_and_a_preserved_one_is_not() {
    // (a) A stale stamp — jigc's own file, rewritten whole by the install.
    let (repo, home) = born_repo("regenerated");
    let (repo, home) = (repo.path(), home.path());
    assert_eq!(jigc(repo, home, &["setup"]).status.code(), Some(0));
    write(repo, ".jigc/version", "jigc-version: 0.9.0-elsewhere\n");
    let out = jigc(repo, home, &["setup"]);
    assert_eq!(
        out.status.code(),
        Some(0),
        "a regenerated artifact carries none of the user's bytes into the commit: {}",
        said(&out)
    );
    assert!(
        !read(repo, ".jigc/version").contains("0.9.0-elsewhere"),
        "the install re-stamped it — the documented recovery route still runs"
    );

    // (b) A `.jigc/.gitignore` already carrying the floor is left untouched, so an extra
    //     line the user added is still there when the commit would be made.
    let (repo, home) = born_repo("preserved");
    let (repo, home) = (repo.path(), home.path());
    assert_eq!(jigc(repo, home, &["setup"]).status.code(), Some(0));
    let mine = format!(
        "{}\n# my own ignore\n",
        read(repo, ".jigc/.gitignore").trim_end()
    );
    write(repo, ".jigc/.gitignore", &mine);
    let out = jigc(repo, home, &["setup"]);
    assert_eq!(
        out.status.code(),
        Some(1),
        "bytes that survive the install would ride the commit, so the door refuses: {}",
        said(&out)
    );
    let text = said(&out);
    assert!(text.contains(DIRTY_CODE) && text.contains(".jigc/.gitignore"));
    assert_eq!(
        read(repo, ".jigc/.gitignore"),
        mine,
        "and the user's line is still on disk"
    );
}

/// (13) **The emitted route, followed verbatim, restores a runnable `jigc setup`.** The
/// refusal names `commit or stash the work at those path(s) … then re-run` — so the re-run
/// after exactly that act must land the install commit. (Cell 8 drives the same route on
/// the untracked leg, where the act is the `-u` spelling the route names.) Until M51's fix it did not: the
/// refusal's own `git add` of the install paths it *was* willing to commit left seven
/// index entries that the next run's pre-write probe read as the adopter's work, so the
/// door refused over its own install files, forever, and the only exits were `--force`
/// (which then swept the very bytes the guard had refused) or a hand `git reset`.
#[test]
fn the_route_followed_verbatim_lands_the_install_commit() {
    let (repo, home) = born_repo("recover");
    let (repo, home) = (repo.path(), home.path());
    write(repo, "CLAUDE.md", "user line one\n");
    git(repo, &["add", "CLAUDE.md"]);
    git(repo, &["commit", "-q", "-m", "add CLAUDE.md"]);
    write(repo, "CLAUDE.md", "user line one\nUNSTAGED WIP SECRET\n");

    let out = jigc(repo, home, &["setup"]);
    assert_eq!(out.status.code(), Some(1), "the refusal: {}", said(&out));
    assert!(said(&out).contains(DIRTY_CODE));
    let head_after_refusal = git(repo, &["rev-parse", "HEAD"]);

    // The route, verbatim: stash the work at the path it named.
    git(repo, &["stash", "push", "-q", "--", "CLAUDE.md"]);

    let out = jigc(repo, home, &["setup"]);
    assert_eq!(
        out.status.code(),
        Some(0),
        "the act the route names restores a runnable `jigc setup`: {}",
        said(&out)
    );
    assert_ne!(
        git(repo, &["rev-parse", "HEAD"]),
        head_after_refusal,
        "the install commit landed on the re-run"
    );
    let committed = git(repo, &["show", "--name-only", "--format=", "HEAD"]);
    assert!(
        committed.lines().any(|p| p == ".jigc/AGENT.md"),
        "and it carries the install footprint: {committed}"
    );
    assert!(
        git(repo, &["show", "stash@{0}:CLAUDE.md"]).contains("UNSTAGED WIP SECRET"),
        "the adopter's stashed work is still theirs to pop"
    );
}

/// (14) **A re-run over an unresolved refusal refuses over the same paths — never over
/// `setup`'s own install files.** The second refusal naming `.jigc/AGENT.md` as *work
/// `jigc setup` did not write* is a law-1 lie about bytes the previous run wrote itself,
/// and it is what made the state permanent: the set grew with every run instead of
/// staying the adopter's one path.
#[test]
fn a_rerun_over_an_unresolved_refusal_names_the_same_paths() {
    let (repo, home) = born_repo("unresolved");
    let (repo, home) = (repo.path(), home.path());
    write(repo, "CLAUDE.md", "user line one\n");
    git(repo, &["add", "CLAUDE.md"]);
    git(repo, &["commit", "-q", "-m", "add CLAUDE.md"]);
    write(repo, "CLAUDE.md", "user line one\nUNSTAGED WIP SECRET\n");

    let first = said(&jigc(repo, home, &["setup"]));
    assert!(first.contains("1 path(s)"), "the first refusal: {first}");

    let out = jigc(repo, home, &["setup"]);
    let second = said(&out);
    assert_eq!(out.status.code(), Some(1), "still refused: {second}");
    assert!(
        second.contains("1 path(s)") && second.contains("CLAUDE.md"),
        "the subject is the adopter's one path, as before: {second}"
    );
    assert!(
        !second.contains(".jigc/AGENT.md"),
        "and never the install files the previous run wrote and staged itself: {second}"
    );
}

/// (15) **The exemption is keyed on the bytes, so a user edit after a leaving arm re-arms
/// the guard.** `setup` preserves a `.jigc/.gitignore` that already carries its floor (cell
/// 12b), so a line the adopter adds to the copy a **left-uncommitted** install wrote would
/// ride the next install commit — and does not: the recorded footprint describes the bytes
/// `setup` left, and these are no longer those bytes.
///
/// Driven through the **install-commit rejection** arm (cell 16's repo shape) rather than
/// the dirty refusal, because since the M51 completion audit the dirty refusal is asked
/// before the first write and therefore leaves no footprint at all. The three arms that
/// still leave one — a `git add` git declines, a `git commit` it declines, the posture
/// re-probe — are the class the footprint record exists for.
#[test]
fn a_user_edit_after_a_refusal_rearms_the_guard() {
    let (repo, home) = born_repo("rearm");
    let (repo, home) = (repo.path(), home.path());
    // A repo git will not commit in: the install is written and staged, and committed by
    // nothing — the footprint arm.
    git(repo, &["config", "--unset", "user.email"]);
    git(repo, &["config", "--unset", "user.name"]);
    git(repo, &["config", "user.useConfigOnly", "true"]);
    let out = jigc(repo, home, &["setup"]);
    assert_eq!(
        out.status.code(),
        Some(1),
        "git refuses the commit with no identity: {}",
        said(&out)
    );

    // The adopter's own line, added to a file that run wrote and staged.
    let mine = format!(
        "{}\n# my own ignore\n",
        read(repo, ".jigc/.gitignore").trim_end()
    );
    write(repo, ".jigc/.gitignore", &mine);
    // And the rejection's named act, verbatim.
    git(repo, &["config", "user.email", "test@example.com"]);
    git(repo, &["config", "user.name", "Test"]);

    let out = jigc(repo, home, &["setup"]);
    let said = said(&out);
    assert_eq!(
        out.status.code(),
        Some(1),
        "bytes that are no longer `setup`'s own refuse the commit: {said}"
    );
    assert!(
        said.contains(DIRTY_CODE) && said.contains(".jigc/.gitignore"),
        "and the refusal names them: {said}"
    );
    assert!(
        !said.contains(".jigc/AGENT.md"),
        "while the install files whose bytes are still `setup`'s own are exempt: {said}"
    );
    assert_eq!(
        read(repo, ".jigc/.gitignore"),
        mine,
        "the adopter's line is still on disk"
    );
}

/// (16) **The sibling arm, driven: the install-commit rejection's own route.** A repo git
/// will not commit in (`user.useConfigOnly` with no identity) leaves the install written
/// and staged and routes *"tell git who you are … then re-run `jigc setup` to commit the
/// staged install files"*. That re-run met the same dead end — over **eight** paths, in a
/// repo with no adopter changes at all — because the pre-write probe read `setup`'s own
/// staged install as work it did not write. Every arm that leaves the footprint
/// uncommitted is one class, so the exemption is recorded where the pathspec settles, not
/// on the refusal that happens to be reported.
#[test]
fn the_install_commit_rejections_route_also_lands_on_the_rerun() {
    let (repo, home) = born_repo("identity");
    let (repo, home) = (repo.path(), home.path());
    git(repo, &["config", "--unset", "user.email"]);
    git(repo, &["config", "--unset", "user.name"]);
    git(repo, &["config", "user.useConfigOnly", "true"]);

    let out = jigc(repo, home, &["setup"]);
    let first = said(&out);
    assert_eq!(
        out.status.code(),
        Some(1),
        "git refuses the commit with no identity: {first}"
    );
    assert!(
        first.contains("setup.install-commit"),
        "the rejection is the install-commit one: {first}"
    );

    // The route, verbatim.
    git(repo, &["config", "user.email", "test@example.com"]);
    git(repo, &["config", "user.name", "Test"]);

    let out = jigc(repo, home, &["setup"]);
    assert_eq!(
        out.status.code(),
        Some(0),
        "the staged install commits on the re-run: {}",
        said(&out)
    );
    let committed = git(repo, &["show", "--name-only", "--format=", "HEAD"]);
    assert!(
        committed.lines().any(|p| p == ".jigc/AGENT.md"),
        "the install footprint is in the commit: {committed}"
    );
}

/// (17) **The rejecting-hook cell on the untracked leg** — cell (10)'s sibling, and the
/// M51 completion audit's verbatim repro. `setup` preserves the adopter's `pre-commit`
/// hook and then commits with `--no-verify`, so an untracked `CLAUDE.md` carrying a secret
/// rode `chore(jigc): install jigc workspace config` at exit 0, past the very hook written
/// to refuse it, with `git status --short` then empty — the deliverable sentence word for
/// word, on the one dirtiness class cell (10) did not reach. It also restores the source's
/// stated basis for keeping `--no-verify`: after the refusal, this commit carries only
/// bytes `setup` itself wrote.
#[test]
fn a_rejecting_hook_no_longer_has_an_untracked_secret_swept_past_it() {
    let (repo, home) = born_repo("hook-untracked");
    let (repo, home) = (repo.path(), home.path());
    // The adopter's own policy hook, present before jigc.
    write(
        repo,
        ".git/hooks/pre-commit",
        "#!/bin/sh\nif git diff --cached | grep -q AWS_SECRET; then\n  echo 'POLICY: no secrets'\n  exit 1\nfi\nexit 0\n",
    );
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let hook = repo.join(".git/hooks/pre-commit");
        let mut perms = fs::metadata(&hook).expect("hook metadata").permissions();
        perms.set_mode(0o755);
        fs::set_permissions(&hook, perms).expect("chmod hook");
    }
    // Never committed, never staged — git holds no copy of these bytes anywhere.
    write(repo, "CLAUDE.md", "# my rules\nAWS_SECRET=hunter2\n");
    let head_before = git(repo, &["rev-parse", "HEAD"]);

    let out = jigc(repo, home, &["setup"]);
    assert_eq!(
        out.status.code(),
        Some(1),
        "the untracked secret's path is dirty, so the install commit refuses: {}",
        said(&out)
    );
    assert!(said(&out).contains(DIRTY_CODE));
    assert_eq!(
        git(repo, &["rev-parse", "HEAD"]),
        head_before,
        "HEAD did not move"
    );
    assert!(
        !git(repo, &["log", "--all", "-p"]).contains("AWS_SECRET"),
        "the adopter's secret is in no commit — the `--no-verify` install commit can no \
         longer sweep past the hook that would have refused it, on this leg either"
    );
    assert!(
        !git(repo, &["status", "--short"]).is_empty(),
        "and the tree does not come back empty, so the state prompts recovery"
    );
}

/// (18) **The unborn-`HEAD` exemption survives the untracked leg** — the sibling cell (8)
/// must not close by the back door. On an unborn `HEAD` there is no `HEAD` to be dirty
/// against and *every* pre-existing file is untracked, so refusing there would turn the
/// QUICKSTART on-ramp into a refusal: `setup` owns minting the repo's first commit with
/// its install footprint (M30 audit finding 1), which is Increment 2's
/// `SETUP_UNBORN_EXEMPTION` reading, and cell (4)'s green stays green with an
/// **install-path** plant rather than an unrelated one.
#[test]
fn an_untracked_install_path_on_an_unborn_head_is_the_stated_exemption() {
    let (repo, home) = unborn_repo("unborn-untracked");
    let (repo, home) = (repo.path(), home.path());
    write(repo, "CLAUDE.md", "the adopter's own prose\n");

    let out = jigc(repo, home, &["setup"]);
    assert_eq!(
        out.status.code(),
        Some(0),
        "an untracked install path on an unborn HEAD is the stated exemption: {}",
        said(&out)
    );
    assert!(
        git(repo, &["show", "HEAD:CLAUDE.md"]).contains("the adopter's own prose"),
        "setup minted the first commit with its install footprint, as M30 decided"
    );
}

/// (19) **A path `setup` rewrites whole is a subject too** (M51 completion audit). The
/// guard first shipped as a *conjunction* — dirty before the install **and** still dirty
/// after it — and `adapter::write_bootstrap_file` is an unconditional `fs::write` of the
/// canonical body, so `.jigc/AGENT.md` matched `HEAD` again by the time the second leg was
/// asked and the set came back empty. Driven at `b5ccd818`: the adopter's team rules were
/// gone at exit 0, `git status --short` was empty, and `git log --all -S` found the bytes
/// in no object — the exact signature the guard was chartered on, one path over.
#[test]
fn a_whole_rewrite_install_path_refuses_with_the_users_bytes_intact() {
    let (repo, home) = born_repo("whole-rewrite");
    let (repo, home) = (repo.path(), home.path());
    assert_eq!(jigc(repo, home, &["setup"]).status.code(), Some(0));
    let base = read(repo, ".jigc/AGENT.md");
    let mine = format!("{base}\n## TEAM RULES\n\nNEVER deploy on Friday.\n");
    write(repo, ".jigc/AGENT.md", &mine);
    let head_before = git(repo, &["rev-parse", "HEAD"]);

    let out = jigc(repo, home, &["setup"]);
    let said = said(&out);
    assert_eq!(
        out.status.code(),
        Some(1),
        "a path the install rewrites whole refuses over the adopter's bytes: {said}"
    );
    assert!(
        said.contains(DIRTY_CODE) && said.contains(".jigc/AGENT.md"),
        "the refusal carries the door's code and names the path: {said}"
    );
    assert_eq!(
        read(repo, ".jigc/AGENT.md"),
        mine,
        "the refusal is asked BEFORE the write, so the bytes are still there"
    );
    assert!(
        git(repo, &["status", "--short"])
            .lines()
            .any(|line| line.ends_with(".jigc/AGENT.md")),
        "and still visible in `git status` — something prompts recovery"
    );
    assert_eq!(
        git(repo, &["rev-parse", "HEAD"]),
        head_before,
        "HEAD did not move"
    );
}

/// (20) **The same class, second member: `.jigc/config/packs.yaml`.** `write_compose_marker`
/// is a parse-mutate-serialize round-trip, so a comment the adopter added is dropped by the
/// serializer — the file is *clean against `HEAD`* afterwards for the same structural reason
/// `.jigc/AGENT.md` is, and the conjunction could not see it either.
#[test]
fn a_comment_in_packs_yaml_refuses_rather_than_round_tripping_away() {
    let (repo, home) = born_repo("packs-comment");
    let (repo, home) = (repo.path(), home.path());
    assert_eq!(jigc(repo, home, &["setup"]).status.code(), Some(0));
    let mine = format!("# my note\n{}", read(repo, ".jigc/config/packs.yaml"));
    write(repo, ".jigc/config/packs.yaml", &mine);

    let out = jigc(repo, home, &["setup"]);
    let said = said(&out);
    assert_eq!(
        out.status.code(),
        Some(1),
        "the comment is the adopter's uncommitted bytes: {said}"
    );
    assert!(
        said.contains(DIRTY_CODE) && said.contains(".jigc/config/packs.yaml"),
        "named: {said}"
    );
    assert!(
        read(repo, ".jigc/config/packs.yaml").contains("# my note"),
        "and the comment is still on disk — the round-trip never ran"
    );
}

/// (21) **`--force` still commits, and says what it replaced.** The consent is *"commit
/// these paths as they stand"*, and at a path jigc regenerates whole *"as they stand"* is
/// jigc's canonical content, not the adopter's — so the consent is spent on bytes that are
/// gone, and the ack names every path it was spent on rather than letting the summary read
/// like an ordinary install.
#[test]
fn force_over_a_whole_rewrite_path_says_what_it_replaced() {
    let (repo, home) = born_repo("forced-rewrite");
    let (repo, home) = (repo.path(), home.path());
    assert_eq!(jigc(repo, home, &["setup"]).status.code(), Some(0));
    let base = read(repo, ".jigc/AGENT.md");
    write(
        repo,
        ".jigc/AGENT.md",
        &format!("{base}\n## TEAM RULES\n\nNEVER deploy on Friday.\n"),
    );
    let head_before = git(repo, &["rev-parse", "HEAD"]);

    let out = jigc(repo, home, &["setup", "--force"]);
    let said = said(&out);
    assert_eq!(out.status.code(), Some(0), "`--force` installs: {said}");
    assert!(
        said.contains("setup.forced-install-path") && said.contains(".jigc/AGENT.md"),
        "the ack names the path whose uncommitted bytes the consent was spent on: {said}"
    );
    assert!(
        !read(repo, ".jigc/AGENT.md").contains("NEVER deploy on Friday"),
        "and that is what the consent buys: the install regenerated the file"
    );
    // `HEAD` may not even move — the regenerated file is byte-identical to the committed
    // one, so there is nothing to commit. Which is precisely why this needs an ack: a
    // silent exit 0 would say nothing at all about bytes that are gone.
    assert!(
        git(repo, &["rev-parse", "HEAD"]) == head_before
            || !git(repo, &["show", "--name-only", "--format=", "HEAD"]).is_empty(),
        "the install either committed or had nothing to commit — both are exit 0"
    );
}

/// (22) **The class fence.** Every member of the install commit's pathspec, enumerated from
/// `install_tracked_paths` itself (with every conditional on) rather than from a hand list,
/// paired with the disposition the guard gives it. A **twelfth** install path joins this
/// answer the moment it joins the pathspec, so it reddens here until somebody decides
/// whether its writer can destroy authored bytes.
///
/// The count is **ten paths**, not the eleven a reading of the writer *occurrences* gives:
/// `.claude/settings.json` is one path with three writers (allowlist, SessionStart hook,
/// deny floor). Two of the ten are exempt, and each exemption is a content oracle rather
/// than a name: the guide's recorded body digest, the version stamp's one-line shape.
#[test]
fn the_install_path_class_is_dispositioned_member_by_member() {
    use cli::setup::InstallPathDisposition as D;
    let class = cli::setup::install_path_dispositions(
        "CLAUDE.md",
        ".claude/settings.json",
        Some(".claude/skills/jigc/SKILL.md"),
        Some(".githooks/pre-commit"),
    );
    let got: Vec<(&str, D)> = class.iter().map(|(p, d)| (p.as_str(), *d)).collect();
    assert_eq!(
        got,
        vec![
            // Merged into — `inject_reference` appends idempotently.
            ("CLAUDE.md", D::Refuses),
            // Merged into — allowlist + SessionStart hook + deny floor, structure-aware.
            (".claude/settings.json", D::Refuses),
            // **Rewritten whole** (`adapter::write_bootstrap_file`, an unconditional
            // `fs::write`) and authorable — the M51 completion audit's first loss cell.
            (".jigc/AGENT.md", D::Refuses),
            // Amended to the union (M51 Increment 4) — the user's extra lines survive.
            (".jigc/.gitignore", D::Refuses),
            // Machine-owned provenance stamp; re-stamping IS the documented
            // `store-version.binary-mismatch` recovery, so the oracle is its shape.
            (".jigc/version", D::ExemptWhenJigcOwned),
            // An empty marker file, rewritten — nothing to lose, and nothing that says so.
            (".jigc/config/.gitkeep", D::Refuses),
            // **Parse-mutate-serialize**, so a comment is dropped by the round-trip — the
            // audit's second loss cell, and the reason it cannot be found by looking at
            // whether the file is clean afterwards.
            (".jigc/config/packs.yaml", D::Refuses),
            // Merge-never-clobber, fresh-repo seed only.
            (".gitignore", D::Refuses),
            // M48's refuse-to-clobber already dropped a user-modified copy from the
            // pathspec, so a guide that reaches the guard is jigc's by construction.
            (".claude/skills/jigc/SKILL.md", D::ExemptWhenJigcOwned),
            // A foreign `pre-commit` is preserved verbatim and jigc's block spliced in.
            (".githooks/pre-commit", D::Refuses),
        ],
        "every install path carries a decided disposition — a new one lands as `Refuses`, \
         which is loud, and this fence is where it gets dispositioned on purpose"
    );
}
