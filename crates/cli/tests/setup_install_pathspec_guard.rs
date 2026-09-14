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
//! `HEAD`. Mechanically one `git status --porcelain --untracked-files=no` whose answer
//! is intersected with the pathspec the door actually settles on.
//!
//! **Both axes, not worktree-only.** A pathspec commit takes the **worktree** contents,
//! which is why `decide_carryover`'s index-vs-HEAD snapshot pair is not this door's
//! predicate (§1) — but an **index-only** difference (`MM` with worktree == `HEAD`) was
//! *committed away* at exit 0 too, and the staged blob then ended up named by no commit
//! and no index entry. Cell (3) is that loss.
//!
//! **Untracked is a stated green cell, not a silent narrowing** (cell 8): an untracked
//! `CLAUDE.md` carrying the adopter's own prose is committed, as it was before — M30
//! audit finding 1 gives `setup` the job of making its install footprint *tracked*, and
//! it is also what makes an unborn `HEAD` clean by construction (every pre-existing file
//! there reports `??`), so Increment 2's `SETUP_UNBORN_EXEMPTION` is not re-closed
//! through the back door.
//!
//! **What the refusal does NOT stage** (the one place this suite pins a behaviour the
//! roadmap's *"the eight install paths stay written and staged"* sentence reads past):
//! the door does not stage a path it has just decided not to commit. Staging the `MM`
//! cell's path would replace exactly the index blob the guard exists to save — the same
//! loss, one step earlier — so the refusal stages the install paths it *is* willing to
//! commit and leaves every path it names exactly as it found it. The named paths are
//! still **written** (the install ran), and the message lists them.
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
//! Sixteen cells, all through the real binary (`CARGO_BIN_EXE_jigc`) over throwaway
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
/// `setup.dirty-install-path`, `HEAD` unmoved, the install files written and staged, and
/// the adopter's bytes still on disk and still visible in `git status` — the recovery
/// prompt the driven exit-0 sweep left nowhere.
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
    // The install ran: the files are written, and the ones the door is willing to commit
    // are staged for the re-run.
    assert!(
        repo.join(".jigc/AGENT.md").is_file(),
        "the guard binds the COMMIT, not the install — the install files are written"
    );
    let staged = staged(repo);
    assert!(
        staged.iter().any(|p| p == ".jigc/AGENT.md"),
        "the install files the door will commit are staged: {staged:?}"
    );
    assert!(
        !staged.iter().any(|p| p == "CLAUDE.md"),
        "the door does not stage a path it has just decided not to commit: {staged:?}"
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

/// (4) A **fresh install on an unborn `HEAD`** is clean by construction: every
/// pre-existing file there reports `??`, so nothing is dirty and the install commit is
/// minted as the repo's first (Increment 2's `SETUP_UNBORN_EXEMPTION`, untouched).
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

/// (8) An **untracked** pre-existing install-path file on a born `HEAD` installs — the
/// stated green cell, with its reason: `setup` owns making its install footprint tracked
/// (M30 audit finding 1), and `??` exclusion is also what makes cell (4) clean with no
/// special case.
#[test]
fn an_untracked_install_path_still_installs() {
    let (repo, home) = born_repo("untracked");
    let (repo, home) = (repo.path(), home.path());
    write(repo, "CLAUDE.md", "the adopter's own prose\n");

    let out = jigc(repo, home, &["setup"]);
    assert_eq!(
        out.status.code(),
        Some(0),
        "an untracked install path is a green cell: {}",
        said(&out)
    );
    assert!(
        git(repo, &["show", "HEAD:CLAUDE.md"]).contains("the adopter's own prose"),
        "and it is committed — setup owns making its install footprint tracked"
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
/// refusal names `commit or stash the work at those path(s), then re-run` — so the re-run
/// after exactly that act must land the install commit. Until M51's fix it did not: the
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

/// (15) **The exemption is keyed on the bytes, so a user edit after a refusal re-arms the
/// guard.** `setup` preserves a `.jigc/.gitignore` that already carries its floor (cell
/// 12b), so a line the adopter adds to the copy a refused run left behind would ride the
/// next install commit — and does not: the recorded footprint describes the bytes `setup`
/// left, and these are no longer those bytes.
#[test]
fn a_user_edit_after_a_refusal_rearms_the_guard() {
    let (repo, home) = born_repo("rearm");
    let (repo, home) = (repo.path(), home.path());
    write(repo, "CLAUDE.md", "user line one\n");
    git(repo, &["add", "CLAUDE.md"]);
    git(repo, &["commit", "-q", "-m", "add CLAUDE.md"]);
    write(repo, "CLAUDE.md", "user line one\nUNSTAGED WIP SECRET\n");
    assert_eq!(jigc(repo, home, &["setup"]).status.code(), Some(1));

    // The adopter's own line, added to a file the refused run wrote and staged.
    let mine = format!(
        "{}\n# my own ignore\n",
        read(repo, ".jigc/.gitignore").trim_end()
    );
    write(repo, ".jigc/.gitignore", &mine);
    // And the refusal's named path, resolved exactly as the route says.
    git(repo, &["stash", "push", "-q", "--", "CLAUDE.md"]);

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
