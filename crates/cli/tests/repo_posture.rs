//! The repository-posture family, driven (M51 Increment 2 / T1).
//!
//! `cli::repo::posture` is the one home for the three states a door that commits or moves
//! on the user's behalf must adjudicate before it acts — **HEAD detached** · **HEAD
//! unborn** · **an operation in progress** (`MERGE_HEAD` · `rebase-merge`/`rebase-apply` ·
//! `BISECT_LOG`). [`design/finalize.md`](../../../design/finalize.md) → 1. Preflight has
//! promised the third verbatim (*"No in-progress merge/rebase/bisect"*) since it was
//! written, with **zero** probes behind it in either crate, and the M51 baseline drove the
//! other two as live damage at exit 0.
//!
//! **What this suite drives.** A real `git` repository is driven into **each of the five
//! states** — detached · unborn · merge · rebase · bisect — plus a **clean control** and a
//! **linked-worktree control**, and each row asserts the breach identity, its code,
//! **exactly one `route:` line**, and the git command that resolves the state named
//! **verbatim** inside it. Two further rows carry the probe's two stated non-answers: the
//! **unanswerable** cell (a git that cannot be spawned · a path that is no work tree · a
//! fake `.git`) reads as **no breach**, and the **`GIT_DIR` redirect is declared out** —
//! present here as a row rather than as a silence.
//!
//! **The assertions read the emitted bytes.** Every route assertion renders the breach's
//! finding through the shipped `cli::render::finding_error` carrier — the same
//! `BlockedFinding` a door's error funnel prints and the invocation log keys on — never a
//! hand-built equivalent, so a finding that renders two routes or drops the git command
//! cannot pass here while the printed surface is broken.
//!
//! **No door consumes the probe yet** (T3 wires it), so nothing else in the suite moves.

use cli::render::finding_error;
use cli::repo::{InProgress, PostureBreach, PostureMember, posture};
use std::path::{Path, PathBuf};

/// A throwaway git repository that removes itself on drop (the project's no-tempfile
/// pattern; `remove_dir_all` flattens linked worktrees too).
struct TempRepo(PathBuf);

impl TempRepo {
    /// A bare directory — no `git init`, no `.git` at all.
    fn empty() -> Self {
        let mut path = std::env::temp_dir();
        path.push(format!(
            "jigc-repo-posture-{}-{:?}",
            std::process::id(),
            engine::tempname::unique_nanos(),
        ));
        std::fs::create_dir_all(&path).expect("create temp dir");
        TempRepo(path)
    }

    /// A real repository with one commit on `main` — the base every posture row starts from.
    fn with_one_commit() -> Self {
        let repo = TempRepo::empty();
        git(repo.path(), &["init", "-q", "-b", "main", "."]);
        repo.commit("one", "a\n");
        repo
    }

    fn path(&self) -> &Path {
        &self.0
    }

    /// Write `f.txt` and commit it.
    fn commit(&self, message: &str, body: &str) {
        std::fs::write(self.path().join("f.txt"), body).expect("write f.txt");
        git(self.path(), &["add", "."]);
        git(self.path(), &["commit", "-q", "-m", message]);
    }
}

impl Drop for TempRepo {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.0);
    }
}

/// Run git in `dir`, asserting success. Identity and init defaults are passed as `-c`
/// overrides with the ambient config neutralized, so the row does not depend on whoever
/// runs the suite.
fn git(dir: &Path, args: &[&str]) {
    let out = git_try(dir, args);
    assert!(
        out.status.success(),
        "git {args:?} failed: {}",
        String::from_utf8_lossy(&out.stderr),
    );
}

/// Run git in `dir`, returning the output — for the steps whose *failure* is the state
/// being built (a conflicting merge, a conflicting rebase).
fn git_try(dir: &Path, args: &[&str]) -> std::process::Output {
    std::process::Command::new("git")
        .args(["-c", "user.email=t@t", "-c", "user.name=t"])
        .args(args)
        .current_dir(dir)
        .env("GIT_CONFIG_GLOBAL", "/dev/null")
        .env("GIT_CONFIG_SYSTEM", "/dev/null")
        .output()
        .expect("run git")
}

/// The breach members a probe answered, in order — the handle the door classes filter on.
fn members(breaches: &[PostureBreach]) -> Vec<PostureMember> {
    breaches.iter().map(PostureBreach::member).collect()
}

/// The one breach of `member` in `breaches`, or a hard failure naming what was found —
/// a row that silently matched nothing would be a green over an absent probe.
fn only(breaches: &[PostureBreach], member: PostureMember) -> &PostureBreach {
    let matching: Vec<&PostureBreach> = breaches.iter().filter(|b| b.member() == member).collect();
    assert_eq!(
        matching.len(),
        1,
        "expected exactly one {member:?} breach, found {:?}",
        members(breaches),
    );
    matching[0]
}

/// The bytes a door would print for this breach — the shipped `BlockedFinding` carrier's
/// own `Display`, which is the house finding line.
fn rendered(breach: &PostureBreach) -> String {
    format!("{}", finding_error(&breach.finding()))
}

/// Assert the emitted finding carries `code`, **exactly one** `route:` line, and names
/// `git_command` verbatim inside that route.
fn assert_identity_and_route(breach: &PostureBreach, code: &str, git_command: &str) {
    let finding = breach.finding();
    assert_eq!(finding.code, code, "the breach's code");
    let text = rendered(breach);
    let routes: Vec<&str> = text
        .lines()
        .filter(|line| line.trim_start().starts_with("route: "))
        .collect();
    assert_eq!(routes.len(), 1, "exactly one route line in:\n{text}");
    assert!(
        text.contains(&format!("blocking · {code}")),
        "the emitted line leads with the blocking identity:\n{text}",
    );
    assert!(
        routes[0].contains(git_command),
        "the route must name `{git_command}` verbatim, got: {}",
        routes[0],
    );
}

/// The clean control: a repository with a commit, HEAD on a branch, nothing in progress —
/// **no breach**, so the family never fires on the ordinary posture every door runs in.
#[test]
fn a_clean_repository_breaches_nothing() {
    let repo = TempRepo::with_one_commit();
    assert_eq!(posture(repo.path()), Vec::new());
}

/// Detached HEAD — the member with **no** shipped probe before this task, and the one the
/// baseline drove as a branchless commit at exit 0.
#[test]
fn a_detached_head_breaches_and_the_route_names_git_switch() {
    let repo = TempRepo::with_one_commit();
    git(repo.path(), &["checkout", "-q", "--detach", "HEAD"]);

    let breaches = posture(repo.path());
    assert_eq!(members(&breaches), vec![PostureMember::HeadDetached]);
    assert_identity_and_route(
        only(&breaches, PostureMember::HeadDetached),
        "repo.head-detached",
        "git switch <branch>",
    );
}

/// Unborn HEAD — the member that reuses `task::head_is_unborn`'s discriminator. An unborn
/// HEAD is a *symbolic* ref whose branch does not exist yet, so it is **not** also
/// detached: the two members are disjoint, asserted here rather than assumed.
#[test]
fn an_unborn_head_breaches_and_the_route_names_a_first_commit() {
    let repo = TempRepo::empty();
    git(repo.path(), &["init", "-q", "-b", "main", "."]);

    let breaches = posture(repo.path());
    assert_eq!(members(&breaches), vec![PostureMember::HeadUnborn]);
    assert_identity_and_route(
        only(&breaches, PostureMember::HeadUnborn),
        "repo.head-unborn",
        "git commit",
    );
}

/// A merge the user started and has not concluded — `MERGE_HEAD` present, HEAD still on
/// its branch. This is the cell `jigc task finalize --carry-staged` concluded under jigc's
/// own subject at exit 0 in the baseline.
#[test]
fn a_merge_in_progress_breaches_and_the_route_names_git_merge_abort() {
    let repo = TempRepo::with_one_commit();
    git(repo.path(), &["checkout", "-q", "-b", "side"]);
    repo.commit("side", "side\n");
    git(repo.path(), &["checkout", "-q", "main"]);
    repo.commit("main2", "main\n");
    // A conflicting merge: git exits non-zero and leaves MERGE_HEAD — the failure IS the
    // state being built, so this step is driven through the non-asserting runner.
    let _ = git_try(repo.path(), &["merge", "--no-commit", "side"]);
    assert!(repo.path().join(".git/MERGE_HEAD").exists(), "MERGE_HEAD");

    let breaches = posture(repo.path());
    assert_eq!(members(&breaches), vec![PostureMember::OperationInProgress]);
    let breach = only(&breaches, PostureMember::OperationInProgress);
    assert_eq!(breach.operation(), Some(InProgress::Merge));
    assert_identity_and_route(breach, "repo.operation-in-progress", "git merge --abort");
}

/// A rebase stopped on a conflict — `rebase-merge/` present. Driven, git also **detaches**
/// HEAD for the duration, so this cell legitimately carries two breaches; the row asserts
/// the whole set rather than the one it came for, because a probe that answered only the
/// rebase would be under-reporting a real posture.
#[test]
fn a_rebase_in_progress_breaches_and_the_route_names_git_rebase_abort() {
    let repo = TempRepo::with_one_commit();
    git(repo.path(), &["checkout", "-q", "-b", "side"]);
    repo.commit("side", "side\n");
    git(repo.path(), &["checkout", "-q", "main"]);
    repo.commit("main2", "main\n");
    let _ = git_try(repo.path(), &["rebase", "side"]);
    assert!(
        repo.path().join(".git/rebase-merge").exists(),
        "rebase-merge",
    );

    let breaches = posture(repo.path());
    assert_eq!(
        members(&breaches),
        vec![
            PostureMember::HeadDetached,
            PostureMember::OperationInProgress,
        ],
        "a stopped rebase detaches HEAD as well — both members are real",
    );
    let breach = only(&breaches, PostureMember::OperationInProgress);
    assert_eq!(breach.operation(), Some(InProgress::Rebase));
    assert_identity_and_route(breach, "repo.operation-in-progress", "git rebase --abort");
}

/// A bisect the user started — `BISECT_LOG` present, HEAD still attached at `bisect start`,
/// so this cell isolates the member.
#[test]
fn a_bisect_in_progress_breaches_and_the_route_names_git_bisect_reset() {
    let repo = TempRepo::with_one_commit();
    repo.commit("two", "b\n");
    git(repo.path(), &["bisect", "start"]);
    assert!(repo.path().join(".git/BISECT_LOG").exists(), "BISECT_LOG");

    let breaches = posture(repo.path());
    assert_eq!(members(&breaches), vec![PostureMember::OperationInProgress]);
    let breach = only(&breaches, PostureMember::OperationInProgress);
    assert_eq!(breach.operation(), Some(InProgress::Bisect));
    assert_identity_and_route(breach, "repo.operation-in-progress", "git bisect reset");
}

/// The linked-worktree control, three claims in one row because they are one property —
/// *a worktree answers about itself*:
///
/// 1. a worktree on its own branch is a **clean** posture, even though its `.git` is a
///    file and its git dir lives under the main checkout;
/// 2. a merge started **in** the worktree breaches **there** and leaves the main checkout
///    clean — the per-worktree resolution, which a `dirname(git-common-dir)` answer would
///    get wrong;
/// 3. a `--detach` worktree — the shape jigc's own fan-out provisions — answers
///    `repo.head-detached`, **byte-identically to a user's detached HEAD**. That is the
///    measured un-sniffability behind §3's typed seam subject: the exemption cannot be
///    inferred from what the probe sees, so T4 carries it as a type.
#[test]
fn a_linked_worktree_answers_about_itself() {
    let main = TempRepo::with_one_commit();
    let attached = main.path().join("wt-attached");
    git(
        main.path(),
        &["worktree", "add", "-q", "-b", "feat", "wt-attached"],
    );
    assert!(attached.join(".git").is_file(), "a linked worktree's .git");
    assert_eq!(posture(&attached), Vec::new(), "(1) a clean worktree");

    // (2) Diverge the two, then start a conflicting merge INSIDE the worktree.
    main.commit("main2", "main\n");
    std::fs::write(attached.join("f.txt"), "feature\n").expect("write in the worktree");
    git(&attached, &["commit", "-qam", "feat side"]);
    let _ = git_try(&attached, &["merge", "--no-commit", "main"]);

    let in_worktree = posture(&attached);
    assert_eq!(
        members(&in_worktree),
        vec![PostureMember::OperationInProgress],
    );
    assert_identity_and_route(
        only(&in_worktree, PostureMember::OperationInProgress),
        "repo.operation-in-progress",
        "git merge --abort",
    );
    assert_eq!(
        posture(main.path()),
        Vec::new(),
        "(2) the main checkout is not in that merge and must not be told it is",
    );

    // (3) The fan-out shape: detached, and indistinguishable from a user's detached HEAD.
    let detached = main.path().join("wt-detached");
    git(
        main.path(),
        &["worktree", "add", "-q", "--detach", "wt-detached"],
    );
    let breaches = posture(&detached);
    assert_eq!(members(&breaches), vec![PostureMember::HeadDetached]);
    assert_identity_and_route(
        only(&breaches, PostureMember::HeadDetached),
        "repo.head-detached",
        "git switch <branch>",
    );
}

/// The unanswerable cells read as **no breach** — three of them, one per way the probe can
/// fail to get an answer:
///
/// - **git unspawnable**: the probe's `current_dir` does not exist, so the spawn itself
///   fails and nothing comes back (the `nothing_staged` precedent — a git that cannot be
///   spawned reads as `false`);
/// - **no work tree**: a real directory with no `.git`, where git answers 128 — an exit
///   code outside the two the discriminators name;
/// - **a fake `.git`** (the ~15 unit fixtures' shape) nested *inside a real repository*:
///   it resolves to itself and answers nothing, where a walk-up would answer about the
///   ancestor. Fail-closed here would let a test-harness fact decide a user-facing verdict.
#[test]
fn an_unanswerable_probe_reads_as_no_breach() {
    let absent = std::env::temp_dir().join(format!(
        "jigc-repo-posture-absent-{}-{:?}",
        std::process::id(),
        engine::tempname::unique_nanos(),
    ));
    assert!(
        !absent.exists(),
        "the unspawnable cell needs an absent path"
    );
    assert_eq!(posture(&absent), Vec::new(), "git unspawnable");

    let bare = TempRepo::empty();
    assert_eq!(posture(bare.path()), Vec::new(), "no work tree");

    // The fake-`.git` cell, nested in a real repository that IS in a posture — so a
    // walk-up would have something to wrongly report.
    let ancestor = TempRepo::with_one_commit();
    git(ancestor.path(), &["checkout", "-q", "--detach", "HEAD"]);
    git(ancestor.path(), &["bisect", "start"]);
    let fake = ancestor.path().join("nested");
    std::fs::create_dir_all(fake.join(".git")).expect("seed a fake .git dir");
    assert_eq!(
        posture(&fake),
        Vec::new(),
        "a fake `.git` must answer about itself, never about the ancestor repository",
    );
}

/// **The declared bound, stated as a row rather than as a silence.**
///
/// The `GIT_DIR` redirect is **out of this probe's scope** (D2; `crates/cli/src/repo.rs`'s
/// module header carries the rationale and the reopening condition). This row drives what
/// that costs, so the bound is measured rather than asserted: with `GIT_DIR` exported,
/// the *identical* git query the probe runs answers about a **different repository** — and
/// the probe, whose subject is the path it was handed and which passes the ambient
/// environment through untouched, answers about neither more nor less than that path.
///
/// It reopens when a repository-**identity** check exists that the fake-`.git` fixtures
/// survive; `dirname(git-common-dir) != toplevel` is not one, because a legitimate linked
/// worktree has the identical asymmetry.
#[test]
fn the_git_dir_redirect_is_declared_out_of_this_probe() {
    let here = TempRepo::with_one_commit();
    let elsewhere = TempRepo::with_one_commit();
    git(elsewhere.path(), &["checkout", "-q", "--detach", "HEAD"]);

    // The redirect is live in git: the probe's own query, run in `here` with GIT_DIR
    // pointing at `elsewhere`, answers about `elsewhere` (exit 1 — detached).
    let redirected = std::process::Command::new("git")
        .args(["symbolic-ref", "-q", "HEAD"])
        .current_dir(here.path())
        .env("GIT_DIR", elsewhere.path().join(".git"))
        .env("GIT_CONFIG_GLOBAL", "/dev/null")
        .env("GIT_CONFIG_SYSTEM", "/dev/null")
        .output()
        .expect("run git");
    assert_eq!(
        redirected.status.code(),
        Some(1),
        "GIT_DIR redirects git's answer to the other repository — this is the cost of the \
         declared bound, driven",
    );

    // The probe's subject is the path it is handed, and nothing else.
    assert_eq!(posture(here.path()), Vec::new());
    assert_eq!(
        members(&posture(elsewhere.path())),
        vec![PostureMember::HeadDetached],
    );
}
