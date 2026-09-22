//! **A destroying door refuses a worktree git has left mid-operation, even when its tree is
//! clean** — the leftover classifier's second leg (the independent review of `986d5e0a`, the
//! HIGH; 2026-09-22).
//!
//! `986d5e0a` gave `jigc milestone finalize` the posture probe for every checkout it commits
//! **from**. The sibling it carried was the *destroying* half of the same boundary, and the
//! warrant it was carried on — *"a destroying door whose guard refuses over content and whose
//! `--force` is its consent"* — is true only where the operation leaves content.
//! `cli::milestone::dirty_worktrees` asks `git status --porcelain`, a question about
//! **working-tree bytes**, where the door's exposure is **repository state git holds**. In the
//! clean-tree cell no guard fired at all, so the consent the disposition pointed at was never
//! asked for and `--force` was never reached.
//!
//! Driven on the debug binary at `986d5e0a`, all three refusing doors, exit **0**, nothing
//! printed about the operation:
//!
//!   * `jigc milestone discard <id>` over a provisioned sub-task worktree paused mid-bisect —
//!     worktree gone, `.git/worktrees/<sub>/` gone with it, so `BISECT_LOG`, `ORIG_HEAD` and
//!     that checkout's reflog went too and a commit reachable only from its detached HEAD
//!     dangles until `git gc`;
//!   * `jigc uninstall` over the same state — the checkout removed, the admin dir left behind
//!     (weaker: the objects stay ref-reachable until `git worktree prune`), and still silent;
//!   * `jigc milestone provision <id>` over an **unregistered** worktree parked at a
//!     sub-task's path — the cell the review did not drive, and the worst of the three,
//!     because that path can belong to a **different repository**: driven, a second repo's
//!     linked worktree sitting mid-bisect at `.jigc/worktrees/area-low` was deleted and
//!     replaced by jigc's own checkout at exit 0, leaving the other repository with a stale
//!     registration and no working tree.
//!
//! # The class, and what iterates it
//!
//! The subject is **`cli::milestone::probe_leftover`'s `OwnWorktree` arm** — the one verdict
//! where git can vouch for the path — and every door that asks it inherits the leg. Two axes
//! cross there, and each arm below names which one it iterates:
//!
//!   * **the operation axis** — `support::git_state::GitState::ALL`, minus the members a
//!     worktree cannot hold (each excluded by its own `worktree_refusal`, never by a skip
//!     written here), driven inside a worktree jigc itself provisioned. `GitState::Detached`
//!     is the **control**: jigc provisions every fan-out worktree `--detach`, so a door that
//!     refused there would refuse jigc's own provisioning.
//!   * **the door axis** — `cli::milestone::WORKTREE_DOORS` filtered to its refusing members
//!     by `DestroyingDoor::consent`, read code-side, so a fifth worktree door lands here
//!     rather than being remembered.
//!
//! The two are crossed at one planting rather than fully, and the planting is the reason: a
//! worktree jigc **registered** is reused untouched by `provision` (driven — the idempotent
//! re-provision every fan-out depends on), so the only planting all three doors reach is an
//! **unregistered** worktree at the sub-task path. That is the foreign-repository cell above,
//! and it is what the door axis runs over; the operation axis runs over the provisioned
//! planting, which is the reported repro.
//!
//! Every cell drives the real binary and asserts on the **bytes git left**: the operation's
//! whole marker residue, read back byte-for-byte across the door.

use cli::milestone::{DestroyingDoor, WORKTREE_DOORS};

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

use crate::support::git_state::{self, GitState};

/// The milestone every fixture mints.
const MILESTONE: &str = "cache-rework";
/// Its one sub-task — and therefore the worktree path every planting occupies.
const SUB: &str = "area-low";

/// A throwaway directory that removes itself on drop (the project's no-tempfile pattern).
struct TempDir(PathBuf);

impl TempDir {
    fn new(tag: &str) -> Self {
        let mut path = std::env::temp_dir();
        path.push(format!(
            "jigc-leftover-op-{tag}-{}-{:?}",
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

fn git_ok(cwd: &Path, args: &[&str]) {
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
}

fn git_stdout(cwd: &Path, args: &[&str]) -> String {
    let out = Command::new("git")
        .args(args)
        .current_dir(cwd)
        .output()
        .expect("run git");
    String::from_utf8_lossy(&out.stdout).trim().to_string()
}

/// A repo carrying `milestone:cache-rework` with one sub-task, plus whatever sits at that
/// sub-task's worktree path.
struct Fixture {
    _root: TempDir,
    home: TempDir,
    repo: PathBuf,
}

impl Fixture {
    /// The repo + milestone + one sub-task, with nothing at the worktree path yet.
    fn mint(tag: &str) -> Self {
        let root = TempDir::new(tag);
        let repo = root.path().join("repo");
        fs::create_dir_all(&repo).expect("mk repo");
        git_ok(&repo, &["init", "-q"]);
        git_ok(&repo, &["config", "user.email", "test@example.com"]);
        git_ok(&repo, &["config", "user.name", "Test"]);
        git_ok(&repo, &["config", "commit.gpgsign", "false"]);
        fs::write(repo.join("README.md"), "hello\n").expect("write README");
        git_ok(&repo, &["add", "."]);
        git_ok(&repo, &["commit", "-q", "-m", "initial"]);
        // The project cascade layer — `jigc milestone`'s door-top precondition (M52 Inc 8 / T1).
        crate::support::mint_project_layer(&repo);

        let fx = Fixture {
            _root: root,
            home: TempDir::new("home"),
            repo,
        };
        fx.jigc_ok(&["milestone", "create", "Cache rework"]);
        fx.jigc_ok(&["milestone", "add-task", MILESTONE, "Area low"]);
        fx
    }

    /// The worktree path every planting occupies.
    fn worktree(&self) -> PathBuf {
        self.repo.join(".jigc").join("worktrees").join(SUB)
    }

    /// That path as every surface prints it — repo-relative (law 1).
    fn printed(&self) -> String {
        format!(".jigc/worktrees/{SUB}")
    }

    /// The worktree's **own** git dir — where `MERGE_HEAD`, `rebase-merge/` and `BISECT_LOG`
    /// live for that checkout, and therefore where the operation's whole record is.
    fn worktree_git_dir(&self) -> PathBuf {
        PathBuf::from(git_stdout(
            &self.worktree(),
            &["rev-parse", "--absolute-git-dir"],
        ))
    }

    fn run(&self, args: &[&str]) -> std::process::Output {
        Command::new(env!("CARGO_BIN_EXE_jigc"))
            .args(args)
            .current_dir(&self.repo)
            .env("HOME", self.home.path())
            .env_remove("JIGC_PACK_DIR")
            .output()
            .expect("run the jigc binary")
    }

    fn jigc_ok(&self, args: &[&str]) {
        let out = self.run(args);
        assert!(
            out.status.success(),
            "`jigc {}` must exit 0; stderr:\n{}",
            args.join(" "),
            String::from_utf8_lossy(&out.stderr),
        );
    }
}

/// Every marker `support::git_state` knows about that is present in `git_dir`, paired with
/// its bytes — the whole of what an un-concluded operation left behind, read back so a cell
/// can compare it across the door.
///
/// The **bytes**, not the presence: the loss this class is about is a record destroyed, and a
/// cell that checked only `BISECT_LOG` existed would pass over a truncated one. The
/// `commit_seam_posture` convention, reused.
fn operation_residue(git_dir: &Path) -> Vec<(String, Option<Vec<u8>>)> {
    git_state::MARKER_UNIVERSE
        .iter()
        .filter(|marker| git_dir.join(marker).exists())
        .map(|marker| ((*marker).to_string(), fs::read(git_dir.join(marker)).ok()))
        .collect()
}

/// The code `door` refuses with over a **worktree-shaped** leftover — selected by the subject
/// it answers for, never by its position in the set (the `leftover_probe_fail_closed`
/// convention; `DestroyingDoor::codes` has been a set since M52 Increment 4).
fn worktree_code(door: &DestroyingDoor) -> &'static str {
    door.codes
        .iter()
        .copied()
        .find(|code| code.ends_with(".leftover-holds-work") || code.ends_with(".dirty-worktree"))
        .unwrap_or_else(|| {
            panic!(
                "`{}` removes a worktree-shaped path, so it owes a refusal code over that \
                 subject; got {:?}",
                door.verb, door.codes,
            )
        })
}

/// The argv that stands at `door`, **derived from the door's own `verb`** rather than
/// hand-listed (the `leftover_probe_fail_closed` convention).
fn argv_at(door: &DestroyingDoor, force: bool) -> Vec<String> {
    let mut argv: Vec<String> = door
        .verb
        .split_whitespace()
        .skip(1)
        .map(str::to_owned)
        .collect();
    if argv.first().map(String::as_str) == Some("milestone") {
        argv.push(MILESTONE.to_owned());
    }
    if force {
        argv.push("--force".to_owned());
    }
    argv
}

/// Assert that `stderr` refused over the operation `operation`, at the worktree `printed`,
/// with a route a reader can run from where they are standing.
fn assert_named_the_operation(
    stderr: &str,
    cell: &str,
    code: &str,
    printed: &str,
    operation: cli::repo::InProgress,
) {
    assert!(
        stderr.contains(&format!("blocking · {code}")),
        "{cell}: the refusal must carry the door's own code `{code}`; stderr:\n{stderr}",
    );
    assert!(
        stderr.contains(operation.noun()),
        "{cell}: the refusal must name THIS operation (`{}`) — a door that calls a paused \
         rebase *uncommitted work* has told the reader to commit or stash it, which resolves \
         nothing; stderr:\n{stderr}",
        operation.noun(),
    );
    assert!(
        stderr.contains(printed),
        "{cell}: the refusal must name WHICH path holds it; stderr:\n{stderr}",
    );
    let aimed = format!(
        "git -C {printed} {}",
        operation
            .abandon()
            .strip_prefix("git ")
            .expect("every routed command of the posture family is a git invocation"),
    );
    assert!(
        stderr.contains(&aimed),
        "{cell}: the refusal must name the command that clears the operation, aimed at the \
         checkout holding it (`{aimed}`) — the caller is not standing in it; stderr:\n{stderr}",
    );
}

// ---------------------------------------------------------------------------
// The operation axis, at the reported door, over the provisioned planting.
// ---------------------------------------------------------------------------

/// One cell of the operation axis: `state`, built inside the worktree jigc provisioned, then
/// `jigc milestone discard <id>` with **no consent**.
///
/// Returns whether the worktree's tree was **clean** when the door arrived — the cell that was
/// live, and the one the count below asserts is not empty.
fn discard_cell(state: GitState) -> bool {
    let fx = Fixture::mint(&format!("op-{}", state.name()));
    fx.jigc_ok(&["milestone", "provision", MILESTONE]);
    let worktree = fx.worktree();
    git_state::overlay_worktree(&worktree, fx.home.path(), state)
        .expect("the axis skips the states a worktree cannot hold");

    let git_dir = fx.worktree_git_dir();
    let residue = operation_residue(&git_dir);
    let clean = git_stdout(&worktree, &["status", "--porcelain"]).is_empty();
    let cell = format!("milestone discard × {}", state.name());

    let out = fx.run(&["milestone", "discard", MILESTONE]);
    let stderr = String::from_utf8_lossy(&out.stderr).into_owned();

    let Some(operation) = state.in_progress() else {
        assert!(
            out.status.success(),
            "{cell}: `{}` is the posture every fan-out worktree is provisioned IN — the \
             abandon must still settle over it, or the guard refuses jigc's own \
             provisioning; got {:?}\nstderr:\n{stderr}",
            state.name(),
            out.status,
        );
        return clean;
    };

    assert!(
        !out.status.success(),
        "{cell}: the teardown removes this worktree and every byte of the `{}` it holds, so \
         it must refuse without `--force` — it settled the record at {:?} instead\n\
         stderr:\n{stderr}",
        operation.noun(),
        out.status,
    );
    assert_named_the_operation(
        &stderr,
        &cell,
        "milestone.dirty-worktree",
        &fx.printed(),
        operation,
    );
    assert!(
        worktree.join(".git").exists(),
        "{cell}: a refused abandon must leave the worktree standing",
    );
    assert_eq!(
        operation_residue(&git_dir),
        residue,
        "{cell}: a refused abandon must leave the operation exactly as git left it — every \
         marker present and every byte of its authored message intact",
    );
    clean
}

/// **The whole operation axis at `jigc milestone discard`**, over a worktree jigc itself
/// provisioned — the reported repro, iterated rather than pinned.
///
/// The axis is `GitState::ALL`, minus the members a worktree cannot hold (excluded by their
/// own `worktree_refusal`), and `GitState::Detached` is the proceeding control.
#[test]
fn the_abandon_refuses_every_operation_left_in_a_worktree_it_would_remove() {
    let mut refused = 0;
    let mut excluded = 0;
    let mut clean_tree_cells = 0;
    for state in GitState::ALL {
        if state.worktree_refusal().is_some() {
            excluded += 1;
            continue;
        }
        let clean = discard_cell(*state);
        if state.in_progress().is_some() {
            refused += 1;
            if clean {
                clean_tree_cells += 1;
            }
        }
    }
    assert_eq!(
        refused + excluded + 1,
        GitState::ALL.len(),
        "every member of the axis is a refusing cell, an excluded one with a stated reason, \
         or the one proceeding posture (`detached`) — {refused} refused, {excluded} \
         excluded, out of {}",
        GitState::ALL.len(),
    );
    assert!(
        clean_tree_cells > 0,
        "at least one refusing cell must have had a CLEAN `git status --porcelain` — those \
         are the cells no shipped guard saw, and without one this arm would pass over the \
         defect it exists for",
    );
}

// ---------------------------------------------------------------------------
// The door axis, over the one planting all three refusing doors reach.
// ---------------------------------------------------------------------------

/// Park a **second repository's** linked worktree at the sub-task's worktree path, paused
/// mid-bisect over a clean tree, and return the bytes only that repository holds.
///
/// This is the planting every refusing door reaches: `jigc milestone provision` probes only
/// the paths this repository has **not** registered (a registered worktree is reused
/// untouched — the idempotent re-provision every fan-out depends on), while `discard` probes
/// every sub-task path and `uninstall` every child of `.jigc/worktrees/`. It is also the
/// cell with the sharpest loss: the bytes belong to a repository jigc was never pointed at.
fn park_foreign_worktree(fx: &Fixture, other: &Path) -> PathBuf {
    git_ok(other, &["init", "-q"]);
    git_ok(other, &["config", "user.email", "other@example.com"]);
    git_ok(other, &["config", "user.name", "Other"]);
    git_ok(other, &["config", "commit.gpgsign", "false"]);
    fs::write(other.join("elsewhere.md"), "another repository's file\n").expect("write");
    git_ok(other, &["add", "."]);
    git_ok(other, &["commit", "-q", "-m", "one"]);

    let at = fx.worktree();
    fs::create_dir_all(at.parent().expect("worktrees root")).expect("mk .jigc/worktrees/");
    let out = Command::new("git")
        .args(["worktree", "add", "-q", "--detach"])
        .arg(&at)
        .arg("HEAD")
        .current_dir(other)
        .output()
        .expect("run git worktree add");
    assert!(
        out.status.success(),
        "parking the foreign worktree failed: {}",
        String::from_utf8_lossy(&out.stderr),
    );
    git_ok(&at, &["bisect", "start"]);
    git_ok(&at, &["bisect", "bad"]);
    assert!(
        git_stdout(&at, &["status", "--porcelain"]).is_empty(),
        "the planted worktree must be CLEAN — a dirty one is refused by the shipped guard \
         and this arm would prove nothing",
    );
    at.join("elsewhere.md")
}

/// **The door axis** — every refusing member of `WORKTREE_DOORS`, read code-side, over the
/// one planting all of them reach, without its consent and then with it.
#[test]
fn every_refusing_worktree_door_refuses_a_live_operation_and_takes_it_only_on_consent() {
    let refusing: Vec<&DestroyingDoor> = WORKTREE_DOORS
        .into_iter()
        .filter(|door| door.consent().is_some())
        .collect();
    assert!(
        !refusing.is_empty(),
        "the worktree door table must carry at least one refusing member",
    );

    for door in refusing {
        let code = worktree_code(door);
        // Without the consent: refuse, name the operation, leave every byte.
        {
            let fx = Fixture::mint("door-block");
            let other = TempDir::new("other");
            let witness = park_foreign_worktree(&fx, other.path());
            let git_dir = fx.worktree_git_dir();
            let residue = operation_residue(&git_dir);
            let cell = format!("{} × bisect × force=false", door.verb);

            let argv = argv_at(door, false);
            let args: Vec<&str> = argv.iter().map(String::as_str).collect();
            let out = fx.run(&args);
            let stderr = String::from_utf8_lossy(&out.stderr).into_owned();
            assert!(
                !out.status.success(),
                "{cell}: the door would take a checkout another repository is mid-bisect in, \
                 so it must refuse without `--force`; got {:?}\nstderr:\n{stderr}",
                out.status,
            );
            assert_named_the_operation(
                &stderr,
                &cell,
                code,
                &fx.printed(),
                cli::repo::InProgress::Bisect,
            );
            assert_eq!(
                fs::read_to_string(&witness).ok().as_deref(),
                Some("another repository's file\n"),
                "{cell}: a refusal must leave the other repository's bytes exactly as they \
                 were",
            );
            assert_eq!(
                operation_residue(&git_dir),
                residue,
                "{cell}: a refusal must leave the bisect exactly as git left it",
            );
        }
        // With the consent: proceed — and **narrated ⇔ taken**, the law every destroying
        // door already obeys for bytes (`cli::milestone::PendingLoss`), now owed for the
        // operation too. Which doors take this checkout is not uniform and is not guessed:
        // `discard`'s teardown removes the worktrees this repository **registered** and
        // leaves every other path on disk (its own refusal says so per path), while
        // `uninstall`'s `remove_dir_all(.jigc)` and `provision`'s clear-then-add both take
        // it. So the cell asks the filesystem which happened and holds the door to that.
        {
            let fx = Fixture::mint("door-force");
            let other = TempDir::new("other");
            let witness = park_foreign_worktree(&fx, other.path());
            let cell = format!("{} × bisect × force=true", door.verb);

            let argv = argv_at(door, true);
            let args: Vec<&str> = argv.iter().map(String::as_str).collect();
            let out = fx.run(&args);
            let stderr = String::from_utf8_lossy(&out.stderr).into_owned();
            assert!(
                out.status.success(),
                "{cell}: `--force` is the one consent past this guard, so it must proceed; \
                 got {:?}\nstderr:\n{stderr}",
                out.status,
            );
            // Whether the door took the checkout the bisect was running in. The witness is
            // the other repository's **own tracked file**, not the worktree path or its
            // `.git`: `jigc milestone provision --force` clears the path and immediately
            // `git worktree add`s a fresh checkout at it, so the path existing again says
            // nothing about whether the planted checkout survived (the
            // `leftover_probe_fail_closed` witness rule).
            let taken = !witness.exists();
            let named = stderr.contains(cli::repo::InProgress::Bisect.noun());
            if taken {
                assert!(
                    named,
                    "{cell}: the door destroyed the checkout another repository was \
                     mid-bisect in, so it must SAY so — the consent buys the removal, not \
                     the silence; stderr:\n{stderr}",
                );
            } else {
                assert!(
                    !named,
                    "{cell}: this door leaves an unregistered path on disk, so it must not \
                     claim a destruction it did not perform; stderr:\n{stderr}",
                );
                assert_eq!(
                    fs::read_to_string(&witness).ok().as_deref(),
                    Some("another repository's file\n"),
                    "{cell}: …and the bytes must actually still be there",
                );
            }
        }
    }
}
