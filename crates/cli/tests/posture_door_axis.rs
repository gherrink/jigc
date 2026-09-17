//! **No repository posture reaches a door that commits or moves on the user's behalf**
//! (M51 Increment 2 / T3) — the posture family × the on-behalf classification, driven
//! through the real binary.
//!
//! The two registries this suite crosses are production tables, not lists typed here:
//! [`cli::cli::BEHALF_DOORS`] classifies **every** clap leaf into commit-on-behalf /
//! move-on-behalf / neither, and [`cli::repo::PostureMember::ALL`] is the family's
//! defining case-set. Each acting row is driven **from its own argv** — the row carries a
//! runnable one, fenced against the clap tree by
//! `cli::cli::tests::every_acting_row_carries_a_runnable_argv` — so a row whose cell this
//! suite cannot build is a hard `panic!`, never a skip: a skipped row is a door nobody
//! adjudicated, which is the exact hole the increment exists to close.
//!
//! **The two classes take different postures** (`settle-record.md` → Review amendments
//! §3). A **committing** door refuses the full family, minus whatever its own row states
//! as an exemption (`jigc setup` is exempt from *unborn*, quoted from the M30 audit
//! rationale). A **mover** refuses **only** an operation in progress — a `git mv` lands in
//! the index and which commit it joins stays the user's to decide, so a detached or unborn
//! HEAD is none of the mover's business, while moving a tracked file out from under a
//! half-finished merge is.
//!
//! **Posture refusals carry no override.** The route names the git command that resolves
//! the state, because a posture is a repository state the user can resolve — not bytes
//! only they can value. No cell here passes a flag that makes a refusal proceed.
//!
//! Three named cells carry the damage the M51 baseline drove, rather than only the
//! guard's identity: `jigc setup` on an unborn HEAD **still installs and still commits**
//! (the exemption honoured, the QUICKSTART on-ramp intact), `jigc milestone create` on an
//! unborn HEAD writes **no record and no `4b825dc` base**, and
//! `jigc task finalize --carry-staged` under a live `MERGE_HEAD` refuses and **leaves
//! `MERGE_HEAD` in place** — the flag's consent is *carry my staged work*, never
//! *conclude someone else's merge*.

use cli::cli::{ActsOnBehalf, BEHALF_DOORS, WORK_UNIT_ID_SLOT};
use cli::repo::PostureMember;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};

/// The canonical git empty-tree SHA — the sentinel an unborn HEAD pinned into a
/// **committed** milestone record at exit 0 before this guard.
const EMPTY_TREE_SHA: &str = "4b825dc642cb6eb9a060e54bf8d69288fbee4904";

/// The well-formed work-unit id every `<id>` slot in a row's argv is filled with. The
/// guard answers before any id resolution, so the id need only be well-formed.
const AXIS_ID: &str = "axis-unit";

// ---------------------------------------------------------------------------
// The family's defining case-set, as five repository states.
// ---------------------------------------------------------------------------

/// One state of the posture family, and the breaches `cli::repo::posture` answers in it —
/// each fact driven by `tests/repo_posture.rs` (T1) against a real `git`, in
/// [`PostureMember::ALL`] order.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum State {
    /// `git checkout --detach`.
    Detached,
    /// A fresh `git init` with no commits.
    Unborn,
    /// A conflicting `git merge --no-commit` — `MERGE_HEAD` present, HEAD attached.
    Merge,
    /// A rebase stopped on a conflict. Driven: git **also** detaches HEAD for the
    /// duration, so this state carries two breaches and the two classes answer
    /// *different* ones — which is why it is the sharpest cell in the axis.
    Rebase,
    /// `git bisect start` — `BISECT_LOG` present, HEAD attached.
    Bisect,
}

impl State {
    /// The whole case-set. A state added to the family without a cell here cannot be
    /// silently dropped: the cross below iterates this constant.
    const ALL: [State; 5] = [
        State::Detached,
        State::Unborn,
        State::Merge,
        State::Rebase,
        State::Bisect,
    ];

    /// The breaches the probe answers in this state, in probe order — T1's driven facts.
    fn breaches(self) -> &'static [PostureMember] {
        match self {
            State::Detached => &[PostureMember::HeadDetached],
            State::Unborn => &[PostureMember::HeadUnborn],
            State::Merge => &[PostureMember::OperationInProgress],
            // The OPERATION first, then the HEAD it detached (M52 Increment 3): both
            // members are real, and a consumer taking the first adjudicated breach must
            // be handed the cause. Before the flip, every commit door answered this cell
            // `repo.head-detached` and routed at a `git switch` git refuses at exit 128,
            // while the rebase that caused the detachment went unnamed.
            State::Rebase => &[
                PostureMember::OperationInProgress,
                PostureMember::HeadDetached,
            ],
            State::Bisect => &[PostureMember::OperationInProgress],
        }
    }

    fn label(self) -> &'static str {
        match self {
            State::Detached => "a detached HEAD",
            State::Unborn => "an unborn HEAD",
            State::Merge => "a merge in progress",
            State::Rebase => "a rebase in progress",
            State::Bisect => "a bisect in progress",
        }
    }
}

/// A throwaway repository driven into `state`, with a jigc project layer present so no
/// door can answer *"no project layer"* instead of answering the posture.
fn repo_in(state: State) -> TempDir {
    let repo = TempDir::new("posture-door");
    git(repo.path(), &["init", "-q", "-b", "main", "."]);
    // The project layer marker — created by hand rather than by `jigc setup`, whose
    // install commit would birth HEAD and destroy the unborn state.
    std::fs::create_dir_all(repo.path().join(".jigc").join("config")).expect("mk config layer");
    if state == State::Unborn {
        return repo;
    }
    commit(repo.path(), "one", "a\n");
    match state {
        State::Unborn => unreachable!("returned above"),
        State::Detached => git(repo.path(), &["checkout", "-q", "--detach", "HEAD"]),
        State::Merge | State::Rebase => {
            git(repo.path(), &["checkout", "-q", "-b", "side"]);
            commit(repo.path(), "side", "side\n");
            git(repo.path(), &["checkout", "-q", "main"]);
            commit(repo.path(), "main2", "main\n");
            // The conflict IS the state being built, so git's non-zero exit is expected.
            let _ = git_try(
                repo.path(),
                if state == State::Merge {
                    &["merge", "--no-commit", "side"]
                } else {
                    &["rebase", "side"]
                },
            );
            let marker = if state == State::Merge {
                "MERGE_HEAD"
            } else {
                "rebase-merge"
            };
            assert!(
                repo.path().join(".git").join(marker).exists(),
                "{} must leave `{marker}` behind",
                state.label(),
            );
        }
        State::Bisect => {
            commit(repo.path(), "two", "b\n");
            git(repo.path(), &["bisect", "start"]);
            assert!(
                repo.path().join(".git").join("BISECT_LOG").exists(),
                "a bisect must leave BISECT_LOG behind",
            );
        }
    }
    repo
}

// ---------------------------------------------------------------------------
// The cross: every acting row × every state.
// ---------------------------------------------------------------------------

/// The code a door of `acts` refuses with in `state` — the **first** breach of the state
/// the class adjudicates, or `None` when the class adjudicates none of them (the mover's
/// proceed cells, and `jigc setup`'s exempt cell).
fn expected_code(acts: &ActsOnBehalf, state: State) -> Option<&'static str> {
    state
        .breaches()
        .iter()
        .find(|member| match acts {
            ActsOnBehalf::CommitsOnBehalf { exempt, .. } => {
                !exempt.iter().any(|row| row.member == **member)
            }
            ActsOnBehalf::MovesOnBehalf { .. } => **member == PostureMember::OperationInProgress,
            ActsOnBehalf::Neither => false,
        })
        .map(|member| member.code())
}

/// **Every acting row of the on-behalf registry, in every state of the posture family.**
///
/// A commit-on-behalf cell must refuse: non-zero exit, the member's code, **exactly one**
/// route naming the git command that resolves the state, and **nothing committed** — the
/// commit count over all refs and HEAD itself both unmoved. A move-on-behalf cell must
/// refuse an operation in progress and **proceed** past a detached or unborn HEAD.
///
/// A row this suite cannot drive is a hard failure, not a skip.
#[test]
fn every_acting_door_adjudicates_the_posture_family() {
    let mut acting = 0usize;
    for row in BEHALF_DOORS {
        let argv: Vec<String> = match &row.acts {
            ActsOnBehalf::Neither => continue,
            ActsOnBehalf::CommitsOnBehalf { argv, .. } | ActsOnBehalf::MovesOnBehalf { argv } => {
                argv.iter()
                    .map(|token| {
                        if *token == WORK_UNIT_ID_SLOT {
                            AXIS_ID.to_string()
                        } else {
                            (*token).to_string()
                        }
                    })
                    .collect()
            }
        };
        acting += 1;
        let shown = row.door.join(" ");
        for state in State::ALL {
            let repo = repo_in(state);
            let home = TempDir::new("home");
            let commits_before = commit_count(repo.path());
            let head_before = head_sha(repo.path());

            let out = run_jigc(repo.path(), home.path(), &argv);
            let text = format!(
                "{}{}",
                String::from_utf8_lossy(&out.stdout),
                String::from_utf8_lossy(&out.stderr),
            );

            match expected_code(&row.acts, state) {
                Some(code) => {
                    assert!(
                        !out.status.success(),
                        "`jigc {shown}` under {} must refuse; output:\n{text}",
                        state.label(),
                    );
                    assert!(
                        text.contains(&format!("blocking · {code}")),
                        "`jigc {shown}` under {} must refuse with `{code}`; output:\n{text}",
                        state.label(),
                    );
                    let routes: Vec<&str> = text
                        .lines()
                        .filter(|line| line.trim_start().starts_with("route: "))
                        .collect();
                    assert_eq!(
                        routes.len(),
                        1,
                        "`jigc {shown}` under {} must print exactly one route; output:\n{text}",
                        state.label(),
                    );
                    assert!(
                        routes[0].contains(git_command_for(code, state)),
                        "`jigc {shown}` under {}: the route must name `{}`; got: {}",
                        state.label(),
                        git_command_for(code, state),
                        routes[0],
                    );
                    assert_eq!(
                        commit_count(repo.path()),
                        commits_before,
                        "`jigc {shown}` under {} must commit nothing; output:\n{text}",
                        state.label(),
                    );
                    assert_eq!(
                        head_sha(repo.path()),
                        head_before,
                        "`jigc {shown}` under {} must leave HEAD unmoved; output:\n{text}",
                        state.label(),
                    );
                }
                None => {
                    for member in PostureMember::ALL {
                        assert!(
                            !text.contains(member.code()),
                            "`jigc {shown}` under {} must not raise `{}` — this door \
                             adjudicates no member of the family here; output:\n{text}",
                            state.label(),
                            member.code(),
                        );
                    }
                }
            }
        }
    }
    assert!(
        acting > 0,
        "the on-behalf registry must carry acting rows for this axis to mean anything",
    );
}

/// The git command a route must name, per code — and, for a rebase, the *rebase's* abort
/// rather than the merge's: the state decides which operation is in progress.
fn git_command_for(code: &str, state: State) -> &'static str {
    match (code, state) {
        ("repo.head-detached", _) => "git switch <branch>",
        ("repo.head-unborn", _) => "git commit",
        ("repo.operation-in-progress", State::Merge) => "git merge --abort",
        ("repo.operation-in-progress", State::Rebase) => "git rebase --abort",
        ("repo.operation-in-progress", State::Bisect) => "git bisect reset",
        other => panic!("no git command declared for {other:?}"),
    }
}

// ---------------------------------------------------------------------------
// The three named cells — the baseline's driven damage, and the stated exemption.
// ---------------------------------------------------------------------------

/// **`jigc setup` on an unborn HEAD still installs and still commits** — the one stated
/// `Exempt` row in the registry, honoured end to end. Setup owns committing its own
/// install footprint regardless of HEAD state (the M30 audit rationale the row quotes),
/// and it is the first command an adopter runs: refusing here would route the QUICKSTART
/// on-ramp at `jigc setup --force` and train the very reflex the wave prices as its own
/// weakness.
///
/// The row's other two members still apply, asserted in the same test so the exemption
/// cannot quietly widen into *setup adjudicates nothing*.
#[test]
fn setup_is_exempt_from_the_unborn_member_and_from_nothing_else() {
    let repo = TempDir::new("posture-setup-unborn");
    git(repo.path(), &["init", "-q", "-b", "main", "."]);
    let home = TempDir::new("home");
    assert_eq!(commit_count(repo.path()), 0, "the repo starts unborn");

    let out = run_jigc(repo.path(), home.path(), &["setup".to_string()]);
    assert!(
        out.status.success(),
        "`jigc setup` on an unborn HEAD must still install; stderr:\n{}",
        String::from_utf8_lossy(&out.stderr),
    );
    assert_eq!(
        commit_count(repo.path()),
        1,
        "`jigc setup` on an unborn HEAD must still land its install commit",
    );

    // …and the two members it is NOT exempt from still refuse.
    for state in [State::Detached, State::Merge] {
        let repo = repo_in(state);
        let home = TempDir::new("home");
        let before = commit_count(repo.path());
        let out = run_jigc(repo.path(), home.path(), &["setup".to_string()]);
        let text = String::from_utf8_lossy(&out.stderr).into_owned();
        assert!(
            !out.status.success(),
            "`jigc setup` under {} must refuse; stderr:\n{text}",
            state.label(),
        );
        assert!(
            text.contains(&format!(
                "blocking · {}",
                expected_code(setup_acts(), state).expect("setup adjudicates this member"),
            )),
            "`jigc setup` under {} must name the member it adjudicates; stderr:\n{text}",
            state.label(),
        );
        assert_eq!(
            commit_count(repo.path()),
            before,
            "`jigc setup` under {} must commit nothing",
            state.label(),
        );
    }
}

/// The `jigc setup` row's classification, read back from the production registry — so the
/// exemption this test honours is the one the binary carries, never a copy.
fn setup_acts() -> &'static ActsOnBehalf {
    &BEHALF_DOORS
        .iter()
        .find(|row| row.door == ["setup"])
        .expect("the registry carries a `jigc setup` row")
        .acts
}

/// **`jigc milestone create` on an unborn HEAD writes no record and no `4b825dc` base** —
/// the M51 baseline's driven damage, which landed git's **empty tree** as the shared base
/// pin inside a *committed* milestone record, a base `git worktree add` cannot take.
#[test]
fn milestone_create_on_an_unborn_head_writes_no_record_and_no_empty_tree_base() {
    let repo = repo_in(State::Unborn);
    let home = TempDir::new("home");

    let out = run_jigc(
        repo.path(),
        home.path(),
        &[
            "milestone".to_string(),
            "create".to_string(),
            "Axis milestone".to_string(),
        ],
    );
    let text = format!(
        "{}{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr),
    );
    assert!(
        !out.status.success() && text.contains("blocking · repo.head-unborn"),
        "`jigc milestone create` on an unborn HEAD must refuse; output:\n{text}",
    );
    assert_eq!(
        commit_count(repo.path()),
        0,
        "the refusal must leave the repository unborn",
    );
    let milestones = repo.path().join(".jigc").join("milestones");
    assert!(
        !milestones.exists(),
        "the refusal must write no milestone area at {milestones:?}",
    );
    let mut with_empty_tree = Vec::new();
    collect_files_containing(
        repo.path().join(".jigc"),
        EMPTY_TREE_SHA,
        &mut with_empty_tree,
    );
    assert!(
        with_empty_tree.is_empty(),
        "no file may pin the empty-tree base after the refusal; got: {with_empty_tree:?}",
    );
}

/// **`--carry-staged` stops concluding a user's merge.** The flag's documented consent is
/// *carry my staged work*; it never covered *conclude someone else's merge*, and the
/// baseline drove exactly that at exit 0 under jigc's own commit subject. The
/// operation-in-progress member refuses first, so the flag is a consequence of the
/// ordering rather than a second mechanism — and `MERGE_HEAD` survives, because a refusal
/// that swallowed the merge state would be the same damage with a better message.
#[test]
fn carry_staged_does_not_conclude_a_live_merge() {
    let repo = repo_in(State::Merge);
    let home = TempDir::new("home");
    let merge_head = repo.path().join(".git").join("MERGE_HEAD");
    let before = commit_count(repo.path());

    let out = run_jigc(
        repo.path(),
        home.path(),
        &[
            "task".to_string(),
            "finalize".to_string(),
            AXIS_ID.to_string(),
            "--carry-staged".to_string(),
        ],
    );
    let text = format!(
        "{}{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr),
    );
    assert!(
        !out.status.success() && text.contains("blocking · repo.operation-in-progress"),
        "`--carry-staged` must not carry a door past a live merge; output:\n{text}",
    );
    assert!(
        merge_head.exists(),
        "the refusal must leave MERGE_HEAD in place — it refuses the merge, never concludes it",
    );
    assert_eq!(
        commit_count(repo.path()),
        before,
        "the refusal must commit nothing; output:\n{text}",
    );
}

/// **A fan-out worktree is typed, never sniffed** — and the exemption is exactly **one**
/// member wide.
///
/// jigc's own `milestone provision` mints a `--detach` worktree per sub-task, and
/// `cli::repo::posture` answers `repo.head-detached` there **byte-identically** to a
/// user's detached HEAD (driven in `tests/repo_posture.rs`). So the exemption cannot be
/// read off the probe: it is carried by `cli::repo::posture_subject`, whose
/// `DedicatedWorktree` no caller can construct.
///
/// Two claims, because they are one property:
///
/// 1. inside a provisioned worktree, `jigc task finalize <sub-task>` still answers the
///    **routed** `finalize.milestone-sub-task` refusal it has answered since M31 — a
///    door-top that answered `repo.head-detached` here would have replaced a tested,
///    routed refusal with a posture the user cannot resolve (re-attaching a fan-out
///    worktree's HEAD is not a repair, it is a different bug);
/// 2. a merge left un-concluded **inside** that worktree still refuses, because a
///    dedicated worktree is exempt from *detached* and from nothing else.
#[test]
fn a_dedicated_worktree_is_exempt_from_the_detached_member_and_from_nothing_else() {
    let repo = TempDir::new("posture-worktree");
    git(repo.path(), &["init", "-q", "-b", "main", "."]);
    std::fs::create_dir_all(repo.path().join(".jigc").join("config")).expect("mk config layer");
    commit(repo.path(), "one", "a\n");
    // A divergent `side` branch, minted before the milestone so the base pin is main's
    // tip — the conflicting merge below needs something real to conflict with.
    git(repo.path(), &["checkout", "-q", "-b", "side"]);
    commit(repo.path(), "side", "side\n");
    git(repo.path(), &["checkout", "-q", "main"]);
    commit(repo.path(), "main2", "main\n");
    let home = TempDir::new("home");

    for argv in [
        vec!["milestone", "create", "Cache rework"],
        vec!["milestone", "add-task", "cache-rework", "Area low"],
        vec!["milestone", "provision", "cache-rework"],
    ] {
        let owned: Vec<String> = argv.iter().map(|token| (*token).to_string()).collect();
        let out = run_jigc(repo.path(), home.path(), &owned);
        assert!(
            out.status.success(),
            "`jigc {}` must exit 0 on a clean posture; stderr:\n{}",
            argv.join(" "),
            String::from_utf8_lossy(&out.stderr),
        );
    }
    let worktree = repo.path().join(".jigc").join("worktrees").join("area-low");
    assert!(
        worktree.is_dir(),
        "the sub-task worktree must be provisioned"
    );
    assert!(
        !git_try(&worktree, &["symbolic-ref", "-q", "HEAD"])
            .status
            .success(),
        "the provisioned worktree must be detached — the un-sniffability this arm rests on",
    );

    // (1) The routed sub-task refusal survives the door-top guard.
    let finalize = run_jigc(
        &worktree,
        home.path(),
        &[
            "task".to_string(),
            "finalize".to_string(),
            "area-low".to_string(),
        ],
    );
    let text = format!(
        "{}{}",
        String::from_utf8_lossy(&finalize.stdout),
        String::from_utf8_lossy(&finalize.stderr),
    );
    assert!(
        !text.contains("repo.head-detached"),
        "a dedicated worktree must not be refused for the HEAD jigc detached itself; \
         output:\n{text}",
    );
    assert!(
        text.contains("finalize.milestone-sub-task"),
        "the routed sub-task refusal must still be the answer here; output:\n{text}",
    );

    // (2) …and a merge left un-concluded inside that worktree still refuses.
    let _ = git_try(&worktree, &["merge", "--no-commit", "side"]);
    assert!(
        repo.path()
            .join(".git")
            .join("worktrees")
            .join("area-low")
            .join("MERGE_HEAD")
            .exists(),
        "the conflicting merge must leave MERGE_HEAD in the worktree's own git dir",
    );
    let blocked = run_jigc(
        &worktree,
        home.path(),
        &[
            "task".to_string(),
            "finalize".to_string(),
            "area-low".to_string(),
        ],
    );
    let text = format!(
        "{}{}",
        String::from_utf8_lossy(&blocked.stdout),
        String::from_utf8_lossy(&blocked.stderr),
    );
    assert!(
        !blocked.status.success() && text.contains("blocking · repo.operation-in-progress"),
        "the exemption covers the detached member and nothing else; output:\n{text}",
    );
}

// ---------------------------------------------------------------------------
// Harness.
// ---------------------------------------------------------------------------

/// A throwaway directory that removes itself on drop (`remove_dir_all` flattens linked
/// worktrees too).
struct TempDir(PathBuf);

impl TempDir {
    fn new(tag: &str) -> Self {
        let mut path = std::env::temp_dir();
        path.push(format!(
            "jigc-{tag}-{}-{:?}",
            std::process::id(),
            engine::tempname::unique_nanos(),
        ));
        std::fs::create_dir_all(&path).expect("create temp dir");
        TempDir(path)
    }

    fn path(&self) -> &Path {
        &self.0
    }
}

impl Drop for TempDir {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.0);
    }
}

/// Run git in `dir`, asserting success, with the ambient config neutralized so a row does
/// not depend on whoever runs the suite.
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
fn git_try(dir: &Path, args: &[&str]) -> Output {
    Command::new("git")
        .args(["-c", "user.email=t@t", "-c", "user.name=t"])
        .args(args)
        .current_dir(dir)
        .env("GIT_CONFIG_GLOBAL", "/dev/null")
        .env("GIT_CONFIG_SYSTEM", "/dev/null")
        .output()
        .expect("run git")
}

/// Write `f.txt` and commit it.
fn commit(dir: &Path, message: &str, body: &str) {
    std::fs::write(dir.join("f.txt"), body).expect("write f.txt");
    git(dir, &["add", "."]);
    git(dir, &["commit", "-q", "-m", message]);
}

/// `git rev-list --count --all` — 0 on an unborn HEAD, and the count every "nothing
/// committed" assertion compares. It counts over **all** refs, so a commit landed on a
/// detached HEAD (reachable from no branch) is still seen.
fn commit_count(dir: &Path) -> u32 {
    String::from_utf8_lossy(&git_try(dir, &["rev-list", "--count", "--all"]).stdout)
        .trim()
        .parse()
        .unwrap_or(0)
}

/// `git rev-parse HEAD`, or the empty string when HEAD is unborn — the second half of
/// "nothing committed", since a commit on a detached HEAD moves HEAD without touching a
/// branch.
fn head_sha(dir: &Path) -> String {
    let out = git_try(dir, &["rev-parse", "HEAD"]);
    if out.status.success() {
        String::from_utf8_lossy(&out.stdout).trim().to_string()
    } else {
        String::new()
    }
}

/// Drive the real binary in `repo` with an isolated `HOME`.
fn run_jigc(repo: &Path, home: &Path, args: &[String]) -> Output {
    Command::new(env!("CARGO_BIN_EXE_jigc"))
        .args(args)
        .current_dir(repo)
        .env("HOME", home)
        .env("GIT_CONFIG_GLOBAL", "/dev/null")
        .env("GIT_CONFIG_SYSTEM", "/dev/null")
        .env("GIT_AUTHOR_NAME", "Test")
        .env("GIT_AUTHOR_EMAIL", "t@t")
        .env("GIT_COMMITTER_NAME", "Test")
        .env("GIT_COMMITTER_EMAIL", "t@t")
        .output()
        .expect("run jigc")
}

/// Every file under `dir` whose bytes contain `needle` — the "no `4b825dc` base" sweep,
/// which asks the whole workbench rather than the one file the damage was reported in.
fn collect_files_containing(dir: PathBuf, needle: &str, found: &mut Vec<PathBuf>) {
    let Ok(entries) = std::fs::read_dir(&dir) else {
        return;
    };
    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() {
            collect_files_containing(path, needle, found);
        } else if std::fs::read_to_string(&path).is_ok_and(|text| text.contains(needle)) {
            found.push(path);
        }
    }
}
