//! The repository-posture family, driven (M51 Increment 2 / T1; the operation axis
//! M52 Increment 3 / T1).
//!
//! `cli::repo::posture` is the one home for the three states a door that commits or moves
//! on the user's behalf must adjudicate before it acts — **HEAD detached** · **HEAD
//! unborn** · **an operation in progress**.
//! [`design/finalize.md`](../../../design/finalize.md) → 1. Preflight has
//! promised the third verbatim (*"No in-progress merge/rebase/bisect"*) since it was
//! written, with **zero** probes behind it in either crate, and the M51 baseline drove the
//! other two as live damage at exit 0.
//!
//! **The third member is a set of operations, not a set of markers** (M52 Increment 3).
//! `cli::repo::InProgress` names every operation git can leave un-concluded — one
//! variant each — because two of those states (a `git merge --squash` and a conflicted
//! index with no operation) write no marker any widening could have reached, and one
//! marker (`rebase-apply/`) is written by **two** operations git itself discriminates.
//!
//! **What this suite drives.** A real `git` repository is driven into **every member of
//! the fixture builder's `GitState`** — the operation axis, one construction per state,
//! built by running the command a user runs — plus a **clean control** and a
//! **linked-worktree control**, and each row asserts the breach identity, its code,
//! **exactly one `route:` line**, the operation's noun in the message, and the git
//! command that resolves the state named **verbatim** inside it — then **runs that
//! command** and requires git to accept it. Two further rows carry the probe's two stated
//! non-answers: the **unanswerable** cell (a git that cannot be spawned · a path that is
//! no work tree · a fake `.git`) reads as **no breach**, and the **`GIT_DIR` redirect is
//! declared out** — present here as a row rather than as a silence.
//!
//! **The assertions read the emitted bytes.** Every route assertion renders the breach's
//! finding through the shipped `cli::render::finding_error` carrier — the same
//! `BlockedFinding` a door's error funnel prints and the invocation log keys on — never a
//! hand-built equivalent, so a finding that renders two routes or drops the git command
//! cannot pass here while the printed surface is broken.
//!
//! **The probe reports the operation before the HEAD it detached** (M52 Increment 3):
//! a stopped rebase is *a rebase*, and calling it a detached HEAD routed the user at a
//! `git switch` git refuses at exit 128 while the operation that caused it went unnamed.
//!
//! **One arm here drives git and not jigc** (M53 Increment 4 / T1, at the foot of the
//! file): before a member is minted for an uncommitted `git cherry-pick --no-commit`, the
//! command its route will name is driven in every state that member will answer, plus a
//! linked worktree — because a route is a claim about git, and a red cell there falsifies
//! the route rather than the code built on it.

use crate::support::git_state::{GitState, GitStateRepo};
use cli::render::finding_error;
use cli::repo::{InProgress, PostureBreach, PostureMember, posture};
use std::collections::BTreeSet;
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
            PostureMember::OperationInProgress,
            PostureMember::HeadDetached,
        ],
        "a stopped rebase detaches HEAD as well — both members are real, and the OPERATION \
         is answered first: the detachment is the rebase's own doing, so a consumer taking \
         the first breach must be handed the cause and not the symptom (M52 Increment 3)",
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

// ───────── the operation axis: every git state a user can leave un-concluded ─────────

/// **Every git state, its own operation, and a route git accepts** (M52 Increment 3 /
/// T1) — the arm that turns the family's third member from three markers into the set
/// of operations git can leave behind.
///
/// The set is [`GitState::ALL`], the fixture builder's own case-set, every member
/// **built by driving real git** and asserting the marker set back at the build
/// (`support/git_state.rs`). Four claims per member, and the last is the one the
/// M51 per-axis review could not make:
///
///   1. the probe answers the [`InProgress`] variant the fixture's own
///      [`GitState::in_progress`] map declares — so `git am` is an `am` and not *a
///      rebase*, and a dangling `sequencer/` is not a cherry-pick that has already
///      been concluded;
///   2. the rendered finding **names that operation's noun** — the message is where a
///      user learns which operation they are in the middle of;
///   3. it prints **exactly one** route (`validation.md`'s *never a menu of three*);
///   4. **the abandoning command in that route, extracted from the emitted bytes and
///      run verbatim in that repository, is accepted by git** (exit 0) and leaves no
///      operation behind. This is the whole point: the shipped binary names `git
///      rebase --abort` under a `git am`, where git answers *"It looks like 'git am'
///      is in progress. Cannot rebase."* at exit **128** — a route that cannot be
///      followed is a route that does not exist (baseline-posture.md §2.2).
///
/// The two **postures** — detached and unborn — carry no operation and are asserted as
/// such rather than skipped: a member that answered an operation there would be the
/// family's own ordering defect in the other direction.
///
/// Red at this task's start on every cell but merge, rebase and bisect: five states
/// answered **no operation at all** (a clean squash merge, a cherry-pick, a revert, a
/// dangling sequencer and a conflicted index with no marker), and `am` answered
/// `Rebase` with a route git refuses.
#[test]
fn every_git_state_names_its_own_operation_and_a_route_git_accepts() {
    let mut covered: BTreeSet<String> = BTreeSet::new();
    for state in GitState::ALL {
        let fixture = GitStateRepo::build(*state);
        let repo = fixture.repo();
        let label = state.name();
        let breaches = posture(&repo);

        let Some(expected) = state.in_progress() else {
            assert!(
                !breaches
                    .iter()
                    .any(|breach| breach.member() == PostureMember::OperationInProgress),
                "`{label}` is a POSTURE, not an operation — there is nothing to conclude \
                 and no git command to route at, so the family must answer \
                 `{:?}` and no operation; got {:?}",
                state,
                breaches,
            );
            continue;
        };
        covered.insert(format!("{expected:?}"));

        let breach = only(&breaches, PostureMember::OperationInProgress);
        assert_eq!(
            breach.operation(),
            Some(expected),
            "the `{label}` fixture must be answered as `{expected:?}` — the fixture built \
             it by running the command a user runs and asserted git's own markers back, \
             so a different variant here is the probe's discrimination, not the state's",
        );

        let text = rendered(breach);
        assert!(
            text.contains(expected.noun()),
            "the `{label}` finding must NAME the operation (`{}`) — a user in the middle \
             of one of these operations learns which one here or nowhere:\n{text}",
            expected.noun(),
        );
        assert_identity_and_route(breach, "repo.operation-in-progress", expected.abandon());

        // (4) The emitted bytes, run verbatim. The command is read out of the rendered
        // route — never rebuilt from `abandon()` — because the route is the artifact a
        // user copies, and a test that rebuilt it could pass while the printed line was
        // broken.
        let argv = abandoning_argv(&text, label);
        let out = std::process::Command::new(&argv[0])
            .args(&argv[1..])
            .current_dir(&repo)
            .env("HOME", fixture.home())
            .output()
            .unwrap_or_else(|err| panic!("`{}` (the `{label}` route): {err}", argv.join(" ")));
        assert!(
            out.status.success(),
            "the `{label}` route must be a command git ACCEPTS in the repository it is \
             printed in — `{}` exited {}:\n{}",
            argv.join(" "),
            out.status,
            String::from_utf8_lossy(&out.stderr),
        );
        let after = posture(&repo);
        assert!(
            !after
                .iter()
                .any(|breach| breach.member() == PostureMember::OperationInProgress),
            "…and it must leave NO operation behind — after `{}` the `{label}` repository \
             still answers {:?}",
            argv.join(" "),
            after,
        );
    }

    // Completeness in the other direction: an operation the probe can answer and no
    // fixture can produce is a route nobody has ever run.
    let unreached: Vec<String> = InProgress::ALL
        .iter()
        .map(|operation| format!("{operation:?}"))
        .filter(|name| !covered.contains(name))
        .collect();
    assert!(
        unreached.is_empty(),
        "every member of `InProgress::ALL` must be produced by at least one `GitState` \
         cell, or its noun and its route are asserted by nothing: {unreached:?}",
    );
}

/// The phrase the route mold puts in front of the command that **abandons** the
/// operation — the one command that is runnable from the state the fixture is in, since
/// every fixture holds an un-concluded operation by construction and concluding one
/// needs the user's own resolution first.
const ABANDON_LEAD: &str = "abandon it with `";

/// The abandoning command, read out of the **rendered** route and split into an argv.
///
/// It panics rather than falling back: a route that does not carry a runnable command
/// in the shape the mold declares is the defect this arm exists to catch, and a lenient
/// parse here would turn it into a skip.
fn abandoning_argv(text: &str, label: &str) -> Vec<String> {
    let route = text
        .lines()
        .find(|line| line.trim_start().starts_with("route: "))
        .unwrap_or_else(|| panic!("the `{label}` finding must print a route:\n{text}"));
    let tail = route.split_once(ABANDON_LEAD).unwrap_or_else(|| {
        panic!(
            "the `{label}` route must name the command that abandons the operation, after \
             `{ABANDON_LEAD}` — the mold every member shares, so a user can copy one \
             runnable command out of it: {route}"
        )
    });
    let command = tail
        .1
        .split_once('`')
        .unwrap_or_else(|| panic!("the `{label}` route's command must be closed: {route}"))
        .0;
    assert!(
        command.starts_with("git "),
        "the `{label}` route abandons an operation with GIT, not with jigc — a posture is \
         a repository state the user resolves themselves: {command}",
    );
    command.split_whitespace().map(str::to_owned).collect()
}

// ───────── the route's two qualifiers are the member's own ─────────

/// The **rendered** route line of every member that supplies a concluding command, as
/// bytes (M53 Increment 4 / T3).
///
/// These five are the shipped set, and this table is here to assert that giving the
/// qualifiers a per-member home **changed none of them**: the clause
/// *"once its conflicts are resolved"* was one hard-coded `format!` shared by the mold,
/// and every one of the five is detected only in a state the user has stopped in, so the
/// clause is true of each and its bytes must survive the move unaltered
/// (`settle-record.md` → §10).
///
/// Pinned as whole lines rather than as the qualifier alone on purpose: a qualifier that
/// arrived correct but landed on the wrong side of a backtick, or a separator that grew
/// a space, is exactly the breakage a substring check reads as fine.
const SHIPPED_ROUTE_LINES: [(InProgress, &str); 5] = [
    (
        InProgress::Merge,
        "conclude it with `git merge --continue` once its conflicts are resolved, or \
         abandon it with `git merge --abort`, then re-run this command",
    ),
    (
        InProgress::Rebase,
        "conclude it with `git rebase --continue` once its conflicts are resolved, or \
         abandon it with `git rebase --abort`, then re-run this command",
    ),
    (
        InProgress::Am,
        "conclude it with `git am --continue` once its conflicts are resolved, or abandon \
         it with `git am --abort`, then re-run this command",
    ),
    (
        InProgress::CherryPick,
        "conclude it with `git cherry-pick --continue` once its conflicts are resolved, or \
         abandon it with `git cherry-pick --abort`, then re-run this command",
    ),
    (
        InProgress::Revert,
        "conclude it with `git revert --continue` once its conflicts are resolved, or \
         abandon it with `git revert --abort`, then re-run this command",
    ),
];

/// The route line the mold owes **this** member — composed from that member's own two
/// qualifiers, which is the whole of what this task moved.
///
/// The composition is the axis half of the arm, not its anchor: what it can say that a
/// byte literal cannot is that **each member's** qualifiers reach the rendered line, so a
/// tenth member declaring one and never seeing it printed reddens here. The anchor is
/// [`SHIPPED_ROUTE_LINES`], which is not composed from anything.
fn expected_route_line(operation: InProgress) -> String {
    let abandon = format!(
        "abandon it with `{}`{}",
        operation.abandon(),
        operation.abandon_qualifier(),
    );
    match operation.conclude() {
        Some((conclude, qualifier)) => format!(
            "conclude it with `{conclude}`{qualifier}, or {abandon}, then re-run this \
             command"
        ),
        None => format!("conclude it, or {abandon}, then re-run this command"),
    }
}

/// The `route:` line of a rendered finding, without the house prefix.
fn route_line(text: &str, label: &str) -> String {
    text.lines()
        .find_map(|line| line.trim_start().strip_prefix("route: "))
        .unwrap_or_else(|| panic!("the `{label}` finding must print a route:\n{text}"))
        .to_string()
}

/// **The route's two qualifiers are the member's own, and the shipped five are unmoved**
/// (M53 Increment 4 / T3).
///
/// The mold hard-coded *"once its conflicts are resolved"* after the concluding command
/// and nothing at all after the abandoning one. The first half is true of every member
/// git detects only in a **stopped** state — which is all five that carry a concluding
/// command today — and false of a member git can also leave behind with nothing
/// conflicted; the second half is a claim a bare command name cannot make, and D4's
/// abandoning command needs one (*it keeps the picked changes, it does not discard
/// them*). So the clause that makes naming a command true is carried **per member**
/// beside the command itself (`settle-record.md` → §10).
///
/// Two assertions per member, and they are not the same assertion:
///
///   1. the **rendered** route line equals the line composed from that member's own
///      [`InProgress::conclude`] qualifier and [`InProgress::abandon_qualifier`] — the
///      axis, over `InProgress::ALL`, so a member whose declared qualifier never reaches
///      the printed bytes reddens;
///   2. for the five members that carry a concluding command, the rendered line equals a
///      **byte literal** written out in full — the anchor, so this task's own move cannot
///      change a shipped user-facing line while both sides of a composition agree.
///
/// Every line asserted here is read out of a breach `posture` built in a repository the
/// fixture drove into that state with real git, rendered through the shipped
/// `finding_error` carrier — never a hand-built `Finding`.
///
/// **Declared bound, measured rather than assumed.** A renderer that dropped the
/// **abandoning** qualifier reddens nothing here today, and cannot: every shipped member
/// declares it empty, so the mutation changes no byte (driven — the conclude qualifier's
/// two mutations, a reword and a drop, each redden). The half that closes it is the
/// member whose abandoning command keeps the work it applied, and it is the composed
/// assertion above that will do the closing.
///
/// Red at this task's start: `InProgress::conclude` and `InProgress::abandon_qualifier`
/// do not exist, the qualifier being a literal inside the renderer's `format!`.
#[test]
fn every_members_route_line_is_composed_from_its_own_two_qualifiers() {
    let mut reached: BTreeSet<String> = BTreeSet::new();
    for state in GitState::ALL {
        let Some(expected) = state.in_progress() else {
            continue;
        };
        let label = state.name();
        let fixture = GitStateRepo::build(*state);
        let breaches = posture(&fixture.repo());
        let breach = only(&breaches, PostureMember::OperationInProgress);
        assert_eq!(
            breach.operation(),
            Some(expected),
            "the `{label}` fixture must be answered as `{expected:?}`",
        );
        reached.insert(format!("{expected:?}"));

        let line = route_line(&rendered(breach), label);
        assert_eq!(
            line,
            expected_route_line(expected),
            "the `{label}` route must be the line `{expected:?}`'s OWN qualifiers compose \
             — a qualifier declared on the member and dropped by the mold is a phrase \
             nobody reads",
        );
        if let Some((_, pinned)) = SHIPPED_ROUTE_LINES
            .iter()
            .find(|(member, _)| *member == expected)
        {
            assert_eq!(
                line, *pinned,
                "`{expected:?}` shipped this line before the qualifiers had a per-member \
                 home, and moving them was not licence to reword it",
            );
        }
    }

    let unreached: Vec<String> = InProgress::ALL
        .iter()
        .map(|operation| format!("{operation:?}"))
        .filter(|name| !reached.contains(name))
        .collect();
    assert!(
        unreached.is_empty(),
        "every member of `InProgress::ALL` must have its route line composed and compared \
         here, or its qualifiers are declared and asserted by nothing: {unreached:?}",
    );

    let unpinned: Vec<String> = SHIPPED_ROUTE_LINES
        .iter()
        .map(|(member, _)| format!("{member:?}"))
        .filter(|name| !reached.contains(name))
        .collect();
    assert!(
        unpinned.is_empty(),
        "…and every byte literal must have been compared against a rendered line, or the \
         pin is a string this suite reads and never checks: {unpinned:?}",
    );
}

// ───────── the owed spike: `git reset` abandons an uncommitted cherry-pick ─────────

/// **D4's route, driven before anything is built on it** (M53 Increment 4 / T1).
///
/// `git cherry-pick --no-commit` leaves a state no member of this family answers today:
/// git writes `MERGE_MSG` (and `AUTO_MERGE`) and **no** `CHERRY_PICK_HEAD`, so the
/// marker-keyed members miss it, and on the conflicted cell only the unmerged index is
/// left to notice — which routes the user at `git reset --merge`, the one command that
/// throws the picked bytes away. D4 mints a member for it whose **abandon** route is
/// plain `git reset`.
///
/// That route is a **claim about git**, not about jigc, and the increment is built on
/// top of it — so it is driven here first, in the four states D4 names plus a linked
/// worktree, and a red cell in this arm falsifies the route rather than the code.
///
/// Four claims per cell, all of them git-level:
///
///   1. the state is the one it claims to be — `MERGE_MSG` present and **every** marker
///      D4's predicate negates absent, read through `git rev-parse --git-path` so the
///      worktree cell is asked the same question as the plain ones;
///   2. `git reset` **exits 0** — the whole route, run verbatim;
///   3. it leaves `MERGE_MSG` gone **in the worktree it was run in, and only there**, and
///      `git ls-files -u` empty;
///   4. **the picked payload is still in the working tree** — unstaged on the clean
///      cells, conflict markers intact on the conflicted one. This is the leg that
///      distinguishes the route from `git reset --merge` and from `git cherry-pick
///      --abort` — which, driven on both the clean and the conflicted cell, answers
///      *"error: no cherry-pick or revert in progress"* at exit **128**,
///      `CHERRY_PICK_HEAD` never having been written.
///
/// The command is hand-built rather than read out of an emitted route on purpose: the
/// member that would emit it lands later in this increment, and **that** arm — the
/// existing `every_git_state_names_its_own_operation_and_a_route_git_accepts` — is where
/// the emitted bytes are extracted and run. This one pins the git behaviour underneath
/// it, so a route that renders correctly over a command git does not accept cannot pass
/// both.
///
/// Driven on git 2.54.0, the version the family's existing deferral names.
#[test]
fn an_uncommitted_cherry_pick_is_abandoned_by_git_reset_in_every_cell() {
    for cell in UncommittedPick::ALL {
        let repo = cell.build();
        let at = repo.path();

        assert_uncommitted_pick_state(at, cell.label());
        assert_eq!(
            unmerged_paths(at),
            cell.unmerged_before(),
            "the `{}` fixture's index must carry the unmerged paths the cell is named for \
             — a conflicted pick that merged cleanly is a different state under the same \
             label",
            cell.label(),
        );

        abandon_with_git_reset(at, cell.label());
        cell.assert_payload_survived(at);
    }

    // ── the linked-worktree cell ──
    //
    // `MERGE_MSG` is **per worktree** (`.git/worktrees/<name>/MERGE_MSG`), and jigc's own
    // fan-out runs `milestone finalize` in a linked worktree — which is why the charter's
    // `rm .git/MERGE_MSG` is false here and `git reset` is the route. Two worktrees are
    // put into the same conflicted state so that *untouched* has something to be measured
    // against: a route that reached for the main checkout's path would leave this cell's
    // own state standing and destroy a sibling's.
    let main = TempRepo::with_one_commit();
    git(main.path(), &["checkout", "-q", "-b", "side"]);
    main.commit("side", "side\n");
    git(main.path(), &["checkout", "-q", "main"]);

    let mut worktrees = Vec::new();
    for name in ["wt-a", "wt-b"] {
        git(main.path(), &["worktree", "add", "-q", "-b", name, name]);
        let at = main.path().join(name);
        std::fs::write(at.join("f.txt"), format!("mainline-{name}\n")).expect("diverge");
        git(&at, &["commit", "-qam", "diverge"]);
        // The conflict is the state being built: git exits 1 here.
        let out = git_try(&at, &["cherry-pick", "-n", "side"]);
        assert!(
            !out.status.success(),
            "the `{name}` worktree cell needs a CONFLICTING pick — git accepted it, so the \
             fixture is a clean pick wearing the conflicted label",
        );
        assert_uncommitted_pick_state(&at, name);
        worktrees.push(at);
    }
    assert!(
        !main.path().join(".git/MERGE_MSG").exists(),
        "the charter's `rm .git/MERGE_MSG` is false in a linked worktree, and this is the \
         datum: both worktrees are mid-pick while the main checkout's own path holds \
         nothing — a route naming that path would remove the wrong file, or none",
    );

    // The helper asserts `wt-a`'s own MERGE_MSG is gone; what only this cell can say is
    // that the sibling's is not.
    abandon_with_git_reset(&worktrees[0], "wt-a");
    assert!(
        merge_msg_path(&worktrees[1]).exists(),
        "`git reset` in `wt-a` must leave the sibling worktree's MERGE_MSG alone — it acts \
         on the worktree it runs in, which is the property that makes it worktree-correct \
         where a path literal is not",
    );
    assert_eq!(
        unmerged_paths(&worktrees[1]),
        3,
        "the sibling's index is untouched too, not merely its MERGE_MSG",
    );
    let text = std::fs::read_to_string(worktrees[0].join("f.txt")).expect("read f.txt");
    for wanted in ["<<<<<<<", "mainline-wt-a", "side"] {
        assert!(
            text.contains(wanted),
            "the picked bytes must survive the abandon in a worktree exactly as they do in \
             a plain checkout — `{wanted}` is gone from:\n{text}",
        );
    }
}

/// The four states D4 names for the member, each built by running the command a user
/// runs — `git cherry-pick --no-commit`, clean or conflicting.
///
/// They are spelled here rather than taken from `GitState`: the variants land later in
/// this increment, and a spike that waited for the fixture builder would be asserting
/// the builder rather than git.
#[derive(Clone, Copy, Debug)]
enum UncommittedPick {
    /// One commit, applied cleanly — the cell with no conflict anywhere, where a
    /// conflict-shaped route would be a lie.
    CleanOne,
    /// A range, applied cleanly — several commits in one pick, and **no** `sequencer/`,
    /// so nothing distinguishes it from the single-commit cell on disk.
    CleanRange,
    /// A conflicting pick, left as git left it — the cell today's family answers as a
    /// bare unmerged index.
    Conflicted,
    /// …and the same pick after the user resolved it with `git add`: the index is clean
    /// again while `MERGE_MSG` still stands, so nothing but that file says the pick is
    /// un-concluded.
    ConflictedThenAdded,
}

impl UncommittedPick {
    const ALL: [UncommittedPick; 4] = [
        UncommittedPick::CleanOne,
        UncommittedPick::CleanRange,
        UncommittedPick::Conflicted,
        UncommittedPick::ConflictedThenAdded,
    ];

    fn label(self) -> &'static str {
        match self {
            UncommittedPick::CleanOne => "clean one-commit",
            UncommittedPick::CleanRange => "clean range",
            UncommittedPick::Conflicted => "conflicted",
            UncommittedPick::ConflictedThenAdded => "conflicted then added",
        }
    }

    /// The unmerged paths the cell's index carries **before** the abandon.
    fn unmerged_before(self) -> usize {
        match self {
            // The three stages git leaves for one conflicting path.
            UncommittedPick::Conflicted => 3,
            _ => 0,
        }
    }

    fn build(self) -> TempRepo {
        let repo = TempRepo::empty();
        let at = repo.path();
        git(at, &["init", "-q", "-b", "main", "."]);
        match self {
            UncommittedPick::CleanOne | UncommittedPick::CleanRange => {
                repo.commit("base", "a\n");
                git(at, &["checkout", "-q", "-b", "side"]);
                repo.commit("side1", "a\nside1\n");
                repo.commit("side2", "a\nside1\nside2\n");
                git(at, &["checkout", "-q", "main"]);
                // Clean picks: git exits 0, so these go through the asserting runner.
                match self {
                    UncommittedPick::CleanOne => git(at, &["cherry-pick", "-n", "side~1"]),
                    _ => git(at, &["cherry-pick", "-n", "main..side"]),
                }
            }
            UncommittedPick::Conflicted | UncommittedPick::ConflictedThenAdded => {
                repo.commit("base", "base\n");
                git(at, &["checkout", "-q", "-b", "side"]);
                repo.commit("side", "side\n");
                git(at, &["checkout", "-q", "main"]);
                repo.commit("main2", "mainline\n");
                // The conflict IS the state being built, so this step is driven through
                // the non-asserting runner and its failure asserted instead.
                let out = git_try(at, &["cherry-pick", "-n", "side"]);
                assert!(
                    !out.status.success(),
                    "the `{}` fixture needs a CONFLICTING pick — git accepted it",
                    self.label(),
                );
                if matches!(self, UncommittedPick::ConflictedThenAdded) {
                    std::fs::write(at.join("f.txt"), "resolved\n").expect("resolve f.txt");
                    git(at, &["add", "f.txt"]);
                }
            }
        }
        repo
    }

    /// **The picked payload is still in the working tree, and unstaged** — the leg that
    /// makes `git reset` an abandon of the *operation* rather than of the user's work.
    fn assert_payload_survived(self, at: &Path) {
        let text = std::fs::read_to_string(at.join("f.txt")).expect("read f.txt");
        match self {
            // Exact bytes where the cell has them: the one-commit pick must carry the
            // first commit's line and NOT the second, or the fixture is the range cell.
            UncommittedPick::CleanOne => assert_eq!(text, "a\nside1\n", "the picked bytes"),
            UncommittedPick::CleanRange => {
                assert_eq!(text, "a\nside1\nside2\n", "both picked commits' bytes")
            }
            UncommittedPick::ConflictedThenAdded => {
                assert_eq!(text, "resolved\n", "the user's own resolution")
            }
            // The conflicted cell's bytes carry a short sha, so the markers are asserted
            // rather than the whole file. Driven, `git reset --merge` — the route today's
            // family prints at this state, via `UnmergedIndex` — leaves f.txt back at
            // `mainline` and the tree CLEAN: markers and picked side both gone.
            UncommittedPick::Conflicted => {
                for wanted in ["<<<<<<<", "mainline", "side"] {
                    assert!(
                        text.contains(wanted),
                        "the conflict markers must survive the abandon — `{wanted}` is gone \
                         from:\n{text}",
                    );
                }
            }
        }
        assert_eq!(
            git_stdout(at, &["diff", "--cached", "--name-only"]),
            "",
            "…and the surviving bytes are UNSTAGED after the abandon ({})",
            self.label(),
        );
        assert_eq!(
            git_stdout(at, &["diff", "--name-only"]),
            "f.txt",
            "…and they are still a working-tree change ({})",
            self.label(),
        );
    }
}

/// `git rev-parse --git-path <name>`, absolutized — the **per-worktree** answer.
///
/// A linked worktree's `MERGE_MSG` lives under `.git/worktrees/<name>/`, so
/// `<repo>/.git/MERGE_MSG` answers about the main checkout no matter which worktree is
/// asking. Every marker in this arm is resolved through git for that reason.
fn merge_msg_path(at: &Path) -> PathBuf {
    git_path(at, "MERGE_MSG")
}

fn git_path(at: &Path, name: &str) -> PathBuf {
    let raw = PathBuf::from(git_stdout(at, &["rev-parse", "--git-path", name]));
    if raw.is_absolute() { raw } else { at.join(raw) }
}

/// Run git in `dir`, asserting success, and return its trimmed stdout.
fn git_stdout(dir: &Path, args: &[&str]) -> String {
    let out = git_try(dir, args);
    assert!(
        out.status.success(),
        "git {args:?} failed: {}",
        String::from_utf8_lossy(&out.stderr),
    );
    String::from_utf8_lossy(&out.stdout).trim().to_string()
}

/// The number of unmerged index entries — `git ls-files -u`, the same question the
/// shipped probe's `index_has_unmerged_paths` asks.
fn unmerged_paths(at: &Path) -> usize {
    git_stdout(at, &["ls-files", "-u"]).lines().count()
}

/// **The state is the one the cell claims**: `MERGE_MSG` present and every marker D4's
/// predicate negates absent — the conjunction that makes an uncommitted cherry-pick
/// distinguishable from the members that already have one.
fn assert_uncommitted_pick_state(at: &Path, label: &str) {
    assert!(
        merge_msg_path(at).exists(),
        "the `{label}` cell must hold MERGE_MSG — that file is the only thing on disk an \
         uncommitted cherry-pick leaves that a concluded one does not",
    );
    for absent in [
        "MERGE_HEAD",
        "CHERRY_PICK_HEAD",
        "REVERT_HEAD",
        "SQUASH_MSG",
        "rebase-merge",
        "rebase-apply",
    ] {
        assert!(
            !git_path(at, absent).exists(),
            "the `{label}` cell must carry NO `{absent}` — every one of these is negated by \
             the member's predicate, and `CHERRY_PICK_HEAD` in particular is the one a \
             reader expects here and git does not write under `--no-commit`",
        );
    }
}

/// Run the route verbatim and assert git accepts it, then assert what it left behind —
/// `MERGE_MSG` gone in this worktree, and an index with nothing unmerged.
fn abandon_with_git_reset(at: &Path, label: &str) {
    let out = git_try(at, &["reset"]);
    assert!(
        out.status.success(),
        "`git reset` — D4's abandon route — must be accepted in the `{label}` cell; it \
         exited {}:\n{}",
        out.status,
        String::from_utf8_lossy(&out.stderr),
    );
    assert!(
        !merge_msg_path(at).exists(),
        "…and must clear MERGE_MSG in the `{label}` cell, or the state it abandons \
         survives it and the member fires again on the next command",
    );
    assert_eq!(
        unmerged_paths(at),
        0,
        "…and must leave no unmerged index entries in the `{label}` cell",
    );
}

// ───────── the finding inventory registers exactly this increment's codes ─────────

/// The design doc that owns the finding inventory — `validation.md`'s **Severity inventory**
/// is the one home a check's severity class, its intrinsic-or-tunable classification and its
/// keyed-or-unkeyed status are stated in, and Increment 1's sibling section directly above is
/// this wave's own precedent for registering mints that key no check id.
const INVENTORY_DOC: &str = "design/validation.md";

/// The section heading this increment's registration table sits under, verbatim.
///
/// **Keyed per increment, deliberately** — the same reason Increment 1's arm states: each
/// increment's arm asserts **equality** over its own mints, so one shared M51 section would
/// redden every such arm the moment a sibling increment registered its codes.
const INVENTORY_HEADING: &str =
    "### The M51 registrations — Increment 2: the repository-posture family";

/// **The three codes Increment 2 mints**, spelled here and deliberately **not** imported
/// from [`PostureMember::code`] — the `design/structural-grammar.md` literal-equality
/// precedent (`crates/cli/tests/malformed_work_unit_id.rs` →
/// `the_design_doc_states_the_shipped_grammar_verbatim`), where a test that derived both
/// sides from the same expression would pass over a renamed code and a stale doc alike.
///
/// The chain that makes this a fence rather than a spell-check has three links, and the arm
/// below asserts all three:
///
/// 1. **doc == these literals**, scraped from the registration table's first column and
///    compared as a **set** — a fourth row, a missing row or a re-spelled code reddens,
///    where `contains` alone would pass a table registering a code the binary never emits;
/// 2. **these literals == what the binary emits** — set equality against
///    [`PostureMember::ALL`], the family's defining case-set, mapped through the shipped
///    [`PostureMember::code`]. Equality in *both* directions, so a fourth member minted in
///    production with no inventory row reddens here as loudly as a doc row with nothing
///    behind it. On the wire, every row above drives these three codes through
///    `cli::render::finding_error`, and `posture_door_axis.rs` drives them through the real
///    binary at every door that commits or moves;
/// 3. **none of them is an [`cli::invocation_log::ERROR_CODE_REGISTRY`] member**
///    (`completions/artifacts/M51/settle-record.md` → §10). That registry mirrors **door
///    identities** derived from `COMMITTING_DOORS`, and a blocking `Finding` is not an
///    `Outcome` identity; registering one there would file a finding in a set whose own
///    fence is the doc mirror of something else.
const INCREMENT_CODES: [&str; 3] = [
    "repo.head-detached",
    "repo.head-unborn",
    "repo.operation-in-progress",
];

/// The repository root, two levels above `crates/cli`.
fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(Path::parent)
        .expect("the repo root sits two levels above crates/cli")
        .to_path_buf()
}

/// The block of [`INVENTORY_DOC`] under [`INVENTORY_HEADING`], up to the next heading of the
/// same or a higher level — the section, never the rest of the file.
fn registration_section(doc: &str) -> &str {
    let start = doc.find(INVENTORY_HEADING).unwrap_or_else(|| {
        panic!(
            "{INVENTORY_DOC} must carry the section `{INVENTORY_HEADING}` — this increment \
             mints three blocking findings, and `validation.md`'s Severity inventory is the \
             home the wave's Settle named for registering them (settle-record.md → §10)",
        )
    });
    let body = &doc[start + INVENTORY_HEADING.len()..];
    let end = body
        .match_indices('\n')
        .map(|(at, _)| at + 1)
        .find(|at| body[*at..].starts_with("## ") || body[*at..].starts_with("### "))
        .unwrap_or(body.len());
    &body[..end]
}

/// Every code the registration table's **first column** names, as a set.
fn registered_codes(section: &str) -> BTreeSet<String> {
    section
        .lines()
        .map(str::trim)
        .filter(|line| line.starts_with('|'))
        // Drop the header row and the `|---|` separator: neither carries a backticked code.
        .filter_map(|line| line.trim_start_matches('|').split('|').next())
        .flat_map(|first_cell| {
            first_cell
                .split('`')
                .skip(1)
                .step_by(2)
                .map(str::trim)
                .map(str::to_owned)
                .collect::<Vec<_>>()
        })
        .collect()
}

/// **The inventory names exactly the codes this increment's binary emits, and none of them
/// joins the door-identity registry** (`settle-record.md` → §10; the roadmap's *Codes it
/// registers*).
///
/// Red at this task's start, and at the section-absent panic rather than the equality: T6
/// began with `validation.md` carrying no Increment 2 registration at all, so the three codes
/// T1 had already minted — and T3 had already wired to every committing and moving door —
/// were named in no inventory. It is the one mechanical check on a task whose deliverable is
/// otherwise prose: `finalize.md`'s preflight row states a **rule**, and a byte-assert over a
/// rule's wording would pin editorial phrasing rather than a contract, which is why that
/// locus is verified by reading against the command stated in `DECISIONS.md` instead (the
/// bound Increment 1 / T9 recorded, applied again).
#[test]
fn the_finding_inventory_registers_exactly_this_increments_codes() {
    let root = repo_root();
    let doc = std::fs::read_to_string(root.join(INVENTORY_DOC))
        .unwrap_or_else(|err| panic!("read {INVENTORY_DOC}: {err}"));
    let section = registration_section(&doc);

    // 1 — doc == these literals, as a SET.
    let registered = registered_codes(section);
    let expected: BTreeSet<String> = INCREMENT_CODES
        .iter()
        .map(|code| (*code).to_owned())
        .collect();
    assert_eq!(
        registered, expected,
        "{INVENTORY_DOC} → `{INVENTORY_HEADING}` must register EXACTLY the codes this \
         increment mints — one table row per code, the code alone in the first column. A row \
         for a code the binary never emits is a lie on the inventory that calls itself *the \
         single source of truth* for what the engine emits; a missing row is the silence §10 \
         exists to end.\nsection read:\n{section}",
    );

    // 2 — these literals == what the binary emits, in BOTH directions.
    let emitted: BTreeSet<String> = PostureMember::ALL
        .iter()
        .map(|member| member.code().to_owned())
        .collect();
    assert_eq!(
        expected, emitted,
        "the literals this arm spells must be exactly the codes `PostureMember::ALL` maps \
         through `PostureMember::code` — the family's defining case-set is the production \
         registry here, so a fourth member minted with no inventory row reddens as loudly as \
         a doc row with nothing behind it",
    );

    // 3 — and none of them joins the door-identity registry (§10).
    for code in INCREMENT_CODES {
        assert!(
            !cli::invocation_log::ERROR_CODE_REGISTRY.contains(&code),
            "`{code}` must stay OUT of `ERROR_CODE_REGISTRY`: that registry mirrors door \
             identities derived from `COMMITTING_DOORS`, and a blocking `Finding` is not an \
             `Outcome` identity (settle-record.md → §10)",
        );
    }
    assert!(
        section.contains("ERROR_CODE_REGISTRY"),
        "…and the section must SAY so — the reason a blocking finding is not a door identity \
         is the half a reader cannot derive from the table, and §10 decided it once for the \
         whole family",
    );

    // …and that these three carry no override, which is the half §3/S3 decided and the half a
    // reader would otherwise assume from the destroying doors' `--force`.
    assert!(
        section.contains("--force"),
        "…and the section must state that a posture refusal carries NO override, naming \
         `--force` — a posture is a repository state the user can resolve, so the route names \
         the git command; leaving it unsaid is how `--force` at `jigc setup` would drift into \
         carrying two meanings (settle-record.md → Review amendments §3 / S3)",
    );
}
