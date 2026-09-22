//! **No repository posture reaches a door that commits or moves on the user's behalf**
//! (M51 Increment 2 / T3, widened at M52 Increment 3 / T2) — the posture family × the
//! on-behalf classification, driven through the real binary.
//!
//! # Three sets, none of them typed here
//!
//! * [`cli::cli::BEHALF_DOORS`] classifies **every** clap leaf into commit-on-behalf /
//!   move-on-behalf / neither, and is fenced ⇔ against the clap tree. Each acting row is
//!   driven **from its own argv** — the row carries a runnable one, fenced by
//!   `cli::cli::tests::every_acting_row_carries_a_runnable_argv` — so a row whose cell
//!   this suite cannot build is a hard `panic!`, never a skip: a skipped row is a door
//!   nobody adjudicated, which is the exact hole the increment exists to close.
//! * [`GitState::ALL`] is the **git-state axis** — one member per operation a user can
//!   leave un-concluded, plus the two postures — built only by driving real git
//!   (`tests/support/git_state.rs`, M52 Increment 2).
//! * [`PostureMember::ALL`] is the family's defining case-set.
//!
//! The cross is every acting row × every git state. Until M52 Increment 3 this suite
//! crossed a **hand-written five-state table** of its own (detached · unborn · merge ·
//! rebase · bisect), and three of the eight states it could not name are exactly where
//! the M52 baseline found the damage: a clean `git merge --squash`, a conflicted index
//! with no marker at all, and a `git am` the shipped probe called *a rebase*. The table
//! is gone; the axis is the enum.
//!
//! # What one cell claims
//!
//! A **refusing** cell: non-zero exit, the adjudicated member's code, **exactly one**
//! route naming **that operation's own** command — the operation arm reads
//! `InProgress::abandon` from the production enum, and only the two postures name a
//! command written here, because that is where their shipped findings carry one — the
//! operation's noun in the message,
//! and **nothing concluded** — the marker set, HEAD and the commit count over all refs
//! all unmoved. A **proceeding** cell raises no member of the family at all.
//!
//! # Why one repository serves a state's refusing cells
//!
//! A refusal mutates nothing — which is itself one of the per-cell assertions — so a
//! state's refusing doors share one built fixture, and the sweep is
//! self-fencing: a door that *did* mutate reddens at its own cell and again at every
//! cell after it. The **proceeding** cells get a fresh fixture each, because a door
//! that proceeds may write, commit or move (`jigc setup` on an unborn HEAD does all
//! three), and the state it left behind is not the state the next cell means to drive.
//!
//! The cost is why it matters: at the registry as it stands — 12 acting rows × 13
//! states = **156 cells** — the sweep runs in **~63 s** through the real binary
//! (measured warm, this test alone, macOS / git 2.54.0) over **18** built fixtures: 13
//! shared, plus one per proceeding cell. A fixture per cell would be 156 of them, and
//! the bound the increment declares is that a cell is never dropped to buy time.
//!
//! # The route is run, once per state, from the door's own emitted bytes
//!
//! A route that cannot be followed is a route that does not exist (the shipped binary
//! named `git rebase --abort` under a `git am`, which git refuses at exit 128). So the
//! abandoning command is **extracted from the door's printed route** and run verbatim.
//! It is run once per state rather than once per cell, and the claim still covers every
//! cell: every refusing door's route is asserted **byte-identical** first — one producer,
//! so the doors cannot disagree — and running the command a second time is impossible
//! anyway, since the first run concludes the operation the state *is*.
//!
//! # The two postures keep their shipped cells
//!
//! A mover refuses **only** an operation in progress — a `git mv` lands in the index and
//! which commit it joins stays the user's to decide, so a detached or unborn HEAD is none
//! of the mover's business, while moving a tracked file out from under a half-finished
//! merge is. `jigc setup` is exempt from *unborn* and from nothing else, asserted here
//! rather than re-decided.
//!
//! **Posture refusals carry no override.** The route names the git command that resolves
//! the state, because a posture is a repository state the user can resolve — not bytes
//! only they can value. No cell here passes a flag that makes a refusal proceed.
//!
//! Five named cells carry driven damage rather than only the guard's identity: a
//! **resolved and staged `git revert`** (§2.5 / L2 — git's own partial-commit guard reads
//! `MERGE_HEAD` and `CHERRY_PICK_HEAD` and never `REVERT_HEAD`, so every commit door
//! concluded the user's revert at exit 0), a **clean `git merge --squash`** (§2.6 / L3 —
//! the squashed payload landed inside jigc's own commit and the authored message died
//! with `SQUASH_MSG`), `jigc setup` on an unborn HEAD **still installing and still
//! committing**, `jigc milestone create` on an unborn HEAD writing **no record and no
//! `4b825dc` base**, and `jigc task finalize --carry-staged` under a live `MERGE_HEAD`
//! refusing and **leaving `MERGE_HEAD` in place** — the flag's consent is *carry my
//! staged work*, never *conclude someone else's merge*.

use crate::support::git_state::{self, GitState, GitStateRepo, Head, MARKER_UNIVERSE};
use cli::cli::{ActsOnBehalf, BEHALF_DOORS, WORK_UNIT_ID_SLOT};
use cli::repo::{PostureMember, posture};
use std::collections::BTreeSet;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};

/// The canonical git empty-tree SHA — the sentinel an unborn HEAD pinned into a
/// **committed** milestone record at exit 0 before this guard.
const EMPTY_TREE_SHA: &str = "4b825dc642cb6eb9a060e54bf8d69288fbee4904";

/// The well-formed work-unit id every `<id>` slot in a row's argv is filled with. The
/// guard answers before any id resolution, so the id need only be well-formed.
const AXIS_ID: &str = "axis-unit";

/// The exit code every posture refusal takes — `cli::task::EXIT_ERROR`, the operational
/// failure seam's code. Asserted on the two named damage cells, where *"exit 1 rather
/// than exit 0"* is the whole difference between a refusal and the baseline's loss.
const EXIT_REFUSED: i32 = 1;

/// The phrase the route mold puts in front of the command that **abandons** the
/// operation — the one command runnable from the state as jigc found it, since every
/// operation fixture holds an un-concluded operation and concluding one needs the user's
/// own resolution first.
///
/// `tests/repo_posture.rs` parses the same lead-in out of the **probe's** finding; here it
/// is read out of the **door's** printed bytes. What is duplicated is the parse hint, not
/// a claim: the command itself is never written down on either side.
const ABANDON_LEAD: &str = "abandon it with `";

// ---------------------------------------------------------------------------
// The family, as the fixture's own driven facts declare it.
// ---------------------------------------------------------------------------

/// The breaches [`posture`] answers in `state`, **in probe order** — derived from the
/// fixture's two driven declarations ([`GitState::in_progress`] and the
/// [`git_state::expectation`] row's HEAD shape), never hand-listed here.
///
/// The order is the product's: the **operation** first, then the HEAD it detached (M52
/// Increment 3). Both rebase cells carry two breaches, and the two door classes answer
/// the same one — which is why they are the sharpest cells in the axis.
fn breaches(state: GitState) -> Vec<PostureMember> {
    let mut members = Vec::new();
    if state.in_progress().is_some() {
        members.push(PostureMember::OperationInProgress);
    }
    match git_state::expectation(state).head {
        Head::Detached => members.push(PostureMember::HeadDetached),
        Head::Unborn => members.push(PostureMember::HeadUnborn),
        Head::Attached => {}
    }
    members
}

/// The member a door of `acts` refuses with in `state` — the **first** breach of the
/// state the class adjudicates, or `None` when the class adjudicates none of them (the
/// mover's proceed cells, and `jigc setup`'s exempt cell).
fn adjudicated_member(acts: &ActsOnBehalf, state: GitState) -> Option<PostureMember> {
    breaches(state).into_iter().find(|member| match acts {
        ActsOnBehalf::CommitsOnBehalf { exempt, .. } => {
            !exempt.iter().any(|row| row.member == *member)
        }
        ActsOnBehalf::MovesOnBehalf { .. } => *member == PostureMember::OperationInProgress,
        ActsOnBehalf::Neither => false,
    })
}

/// The git command the route must name for `member` in `state`.
///
/// The operation arm reads [`cli::repo::InProgress::abandon`] — the production enum, so a
/// route corrected in the binary is corrected here, and a route that drifts from the
/// enum reddens. The two postures name the commands their own shipped findings carry.
fn resolving_command(state: GitState, member: PostureMember) -> String {
    match member {
        PostureMember::OperationInProgress => state
            .in_progress()
            .unwrap_or_else(|| {
                panic!(
                    "`{}` is adjudicated as an operation in progress and declares none",
                    state.name(),
                )
            })
            .abandon()
            .to_string(),
        PostureMember::HeadDetached => "git switch <branch>".to_string(),
        PostureMember::HeadUnborn => "git commit".to_string(),
    }
}

/// A runnable argv for an **acting** row, with [`WORK_UNIT_ID_SLOT`] filled; `None` for
/// [`ActsOnBehalf::Neither`], the control class that stays silent.
fn runnable_argv(acts: &ActsOnBehalf) -> Option<Vec<String>> {
    let argv = match acts {
        ActsOnBehalf::Neither => return None,
        ActsOnBehalf::CommitsOnBehalf { argv, .. } | ActsOnBehalf::MovesOnBehalf { argv } => *argv,
    };
    Some(
        argv.iter()
            .map(|token| {
                if *token == WORK_UNIT_ID_SLOT {
                    AXIS_ID.to_string()
                } else {
                    (*token).to_string()
                }
            })
            .collect(),
    )
}

// ---------------------------------------------------------------------------
// The cross: every acting row × every git state.
// ---------------------------------------------------------------------------

/// **Every acting row of the on-behalf registry, in every state of the git-state axis.**
///
/// Today that is 12 acting rows (10 commit-on-behalf + 2 move-on-behalf) × the 13
/// [`GitState::ALL`] members — but neither factor is a number this suite depends on: the
/// rows are counted off the registry, the states iterated off the enum, and the cell
/// total is asserted against their product. A verb classified as acting, or a thirteenth
/// git state, joins the sweep with no edit here.
#[test]
fn every_acting_door_adjudicates_the_posture_family() {
    let mut commit_rows = 0usize;
    let mut move_rows = 0usize;
    for row in BEHALF_DOORS {
        match &row.acts {
            ActsOnBehalf::CommitsOnBehalf { .. } => commit_rows += 1,
            ActsOnBehalf::MovesOnBehalf { .. } => move_rows += 1,
            ActsOnBehalf::Neither => {}
        }
    }
    assert!(
        commit_rows > 0 && move_rows > 0,
        "the sweep means nothing unless BOTH acting classes are represented — the claim \
         is that they adjudicate DIFFERENT families, not that one is a stronger flavour \
         of the other (got {commit_rows} commit-on-behalf, {move_rows} move-on-behalf)",
    );
    let acting_rows = commit_rows + move_rows;
    let mut cells = 0usize;

    for state in GitState::ALL.iter().copied() {
        let label = state.name();
        // One fixture for every REFUSING cell of this state: a refusal mutates nothing,
        // which each cell asserts, so a door that did mutate reddens at its own cell and
        // at every cell after it.
        let shared = GitStateRepo::build(state);
        let repo = shared.repo();
        let home = shared.home();
        let git_dir = git_dir_of(&repo);
        let untouched = Snapshot::of(&repo, &git_dir);
        let mut routes: BTreeSet<String> = BTreeSet::new();

        for row in BEHALF_DOORS {
            let Some(argv) = runnable_argv(&row.acts) else {
                continue;
            };
            cells += 1;
            let shown = row.door.join(" ");

            let Some(member) = adjudicated_member(&row.acts, state) else {
                // This cell PROCEEDS — and a door that proceeds may write, commit or
                // move, so it gets its own repository rather than the shared one.
                let fresh = GitStateRepo::build(state);
                let text = output_of(&run_jigc(&fresh.repo(), &fresh.home(), &argv));
                for member in PostureMember::ALL {
                    assert!(
                        !text.contains(member.code()),
                        "`jigc {shown}` under `{label}` must not raise `{}` — this door \
                         adjudicates no member of the family here; output:\n{text}",
                        member.code(),
                    );
                }
                continue;
            };

            let out = run_jigc(&repo, &home, &argv);
            let text = output_of(&out);
            assert!(
                !out.status.success(),
                "`jigc {shown}` under `{label}` must refuse; output:\n{text}",
            );
            assert!(
                text.contains(&format!("blocking · {}", member.code())),
                "`jigc {shown}` under `{label}` must refuse with `{}`; output:\n{text}",
                member.code(),
            );
            if let Some(operation) = state.in_progress() {
                assert!(
                    text.contains(operation.noun()),
                    "`jigc {shown}` under `{label}` must NAME the operation (`{}`) — a \
                     user in the middle of one of these operations learns which one here \
                     or nowhere; output:\n{text}",
                    operation.noun(),
                );
            }
            let route_lines: Vec<&str> = text
                .lines()
                .filter(|line| line.trim_start().starts_with("route: "))
                .collect();
            assert_eq!(
                route_lines.len(),
                1,
                "`jigc {shown}` under `{label}` must print exactly one route, never a \
                 menu; output:\n{text}",
            );
            let named = resolving_command(state, member);
            assert!(
                route_lines[0].contains(&named),
                "`jigc {shown}` under `{label}`: the route must name `{named}` — THIS \
                 operation's own command, not a sibling's; got: {}",
                route_lines[0],
            );
            assert_eq!(
                Snapshot::of(&repo, &git_dir),
                untouched,
                "`jigc {shown}` under `{label}` must CONCLUDE NOTHING — the marker set, \
                 HEAD and the commit count over all refs are the user's, and a refusal \
                 that swallowed the state would be the baseline's damage with a better \
                 message; output:\n{text}",
            );
            routes.insert(route_lines[0].trim().to_string());
        }

        // The fixture is still the fixture, by its own driven expectation — markers,
        // HEAD shape and the `git ls-files -u` count, after every acting door ran in it.
        git_state::assert_state(&repo, &home, state);

        assert_eq!(
            routes.len(),
            1,
            "every refusing door of `{label}` must print the SAME route — the finding has \
             one producer (`PostureBreach::finding`), so the acting doors cannot disagree \
             about how a user resolves one state; got: {routes:?}",
        );
        // …and that one route, read out of a door's emitted bytes, is a command git
        // accepts in the repository it was printed in.
        if state.in_progress().is_some() {
            let route = routes.iter().next().expect("the route set holds one line");
            let argv = abandoning_argv(route, label);
            let out = Command::new(&argv[0])
                .args(&argv[1..])
                .current_dir(&repo)
                .env("HOME", &home)
                .output()
                .unwrap_or_else(|err| panic!("`{}` (the `{label}` route): {err}", argv.join(" ")));
            assert!(
                out.status.success(),
                "the `{label}` route must be a command git ACCEPTS in the repository the \
                 door printed it in — `{}` exited {}:\n{}",
                argv.join(" "),
                out.status,
                String::from_utf8_lossy(&out.stderr),
            );
            let after = posture(&repo);
            assert!(
                !after
                    .iter()
                    .any(|breach| breach.member() == PostureMember::OperationInProgress),
                "…and it must leave NO operation behind — after `{}` the `{label}` \
                 repository still answers {after:?}",
                argv.join(" "),
            );
        }
    }

    assert_eq!(
        cells,
        acting_rows * GitState::ALL.len(),
        "every acting row must have been driven in every git state — a cell this suite \
         skipped is a door nobody adjudicated in a state a user can reach",
    );
}

/// The abandoning command, read out of a **rendered** route and split into an argv.
///
/// It panics rather than falling back: a route that does not carry a runnable command in
/// the shape the mold declares is the defect this arm exists to catch, and a lenient
/// parse here would turn it into a skip.
fn abandoning_argv(route: &str, label: &str) -> Vec<String> {
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

// ---------------------------------------------------------------------------
// The named cells — the baseline's driven damage, and the stated exemptions.
// ---------------------------------------------------------------------------

/// **A `git revert` the user has resolved and staged is still a revert** — baseline
/// §2.5 / L2, the cell no marker-shaped guard reached.
///
/// git's own partial-commit guard reads `MERGE_HEAD` and `CHERRY_PICK_HEAD` and **not**
/// `REVERT_HEAD`, so once the conflict is resolved and staged there is nothing left for
/// git to object to: the index is clean, no unmerged path remains, and the next
/// `git commit` — jigc's included, under jigc's own subject — **concludes the user's
/// revert**, after which `git revert --continue` answers *"no cherry-pick or revert in
/// progress"*. That is the state driven here, and `jigc milestone create x` must exit 1
/// naming the revert, route at `git revert --abort`, and leave `REVERT_HEAD` where it
/// found it.
#[test]
fn a_resolved_and_staged_revert_is_still_a_revert() {
    let fixture = GitStateRepo::build(GitState::Revert);
    let repo = fixture.repo();
    let home = fixture.home();
    let git_dir = git_dir_of(&repo);
    let revert_head = git_dir.join("REVERT_HEAD");

    // The user resolves the conflict and stages it. The paths come from git rather than
    // from the fixture's private constants, so this cell states the USER'S act.
    let unmerged = String::from_utf8_lossy(
        &git_try(&repo, &["diff", "--name-only", "--diff-filter=U"]).stdout,
    )
    .into_owned();
    let unmerged: Vec<&str> = unmerged.lines().filter(|line| !line.is_empty()).collect();
    assert!(
        !unmerged.is_empty(),
        "the `revert` fixture must leave a conflict for the user to resolve",
    );
    for path in &unmerged {
        std::fs::write(repo.join(path), "resolved\n").expect("write the resolved file");
        git(&repo, &["add", "--", path]);
    }
    assert!(
        String::from_utf8_lossy(&git_try(&repo, &["ls-files", "-u"]).stdout)
            .trim()
            .is_empty(),
        "the resolved-and-staged cell must leave NO unmerged index entry — an unmerged \
         index is a different member of the family, and this cell is about the one git \
         itself stops objecting to",
    );
    assert!(
        revert_head.exists(),
        "…while `REVERT_HEAD` survives the resolution: that is the whole cell",
    );
    let before = commit_count(&repo);

    let out = run_jigc(
        &repo,
        &home,
        &[
            "milestone".to_string(),
            "create".to_string(),
            "x".to_string(),
        ],
    );
    let text = output_of(&out);
    assert_eq!(
        out.status.code(),
        Some(EXIT_REFUSED),
        "`jigc milestone create x` over a staged revert must exit {EXIT_REFUSED}; \
         output:\n{text}",
    );
    assert!(
        text.contains("blocking · repo.operation-in-progress") && text.contains("a revert"),
        "…and it must name the REVERT, not a generic conflict; output:\n{text}",
    );
    assert!(
        text.contains("git revert --abort"),
        "…and route at the revert's own command; output:\n{text}",
    );
    assert!(
        revert_head.exists(),
        "the refusal must leave `REVERT_HEAD` in place — it refuses the revert, never \
         concludes it",
    );
    assert_eq!(
        commit_count(&repo),
        before,
        "the refusal must commit nothing; output:\n{text}",
    );
    assert!(
        !repo.join(".jigc").join("milestones").exists(),
        "…and write no milestone area",
    );
}

/// **A clean `git merge --squash` keeps its payload and its message** — baseline
/// §2.6 / L3.
///
/// `--squash` never updates HEAD, so `MERGE_HEAD` is absent and `SQUASH_MSG` is the whole
/// of git's record: a marker-list guard sees a clean repository. Driven at the baseline,
/// the squashed payload landed **inside jigc's own commit** at exit 0 and the merge's
/// authored message was destroyed with `SQUASH_MSG`. Here the door refuses, the payload
/// stays staged, and the message's **bytes** are compared — a cell that only checked the
/// file's existence would pass over a truncated one.
#[test]
fn a_clean_squash_merge_keeps_its_payload_and_its_message() {
    let fixture = GitStateRepo::build(GitState::SquashMerge);
    let repo = fixture.repo();
    let home = fixture.home();
    let git_dir = git_dir_of(&repo);
    let squash_msg = git_dir.join("SQUASH_MSG");
    let message = std::fs::read(&squash_msg).expect("the squash merge parks its message");
    let staged =
        String::from_utf8_lossy(&git_try(&repo, &["diff", "--cached", "--name-only"]).stdout)
            .into_owned();
    assert!(
        !staged.trim().is_empty(),
        "the `squash-merge` fixture must leave the squashed payload STAGED — that is the \
         half the baseline folded into jigc's own commit",
    );
    let before = commit_count(&repo);

    let out = run_jigc(
        &repo,
        &home,
        &[
            "task".to_string(),
            "finalize".to_string(),
            AXIS_ID.to_string(),
        ],
    );
    let text = output_of(&out);
    assert_eq!(
        out.status.code(),
        Some(EXIT_REFUSED),
        "a commit door over a staged squash merge must exit {EXIT_REFUSED}; output:\n{text}",
    );
    assert!(
        text.contains("blocking · repo.operation-in-progress") && text.contains("a squash merge"),
        "…and name the squash merge — `MERGE_HEAD` is absent here, so a marker-list guard \
         sees a clean repository; output:\n{text}",
    );
    assert!(
        text.contains("git reset --merge"),
        "…and route at the command that actually clears it (`git merge --abort` answers \
         *there is no merge to abort*); output:\n{text}",
    );
    assert_eq!(
        std::fs::read(&squash_msg).ok().as_deref(),
        Some(message.as_slice()),
        "the refusal must leave `SQUASH_MSG` byte-identical — the authored message is the \
         byte at risk in this cell",
    );
    assert_eq!(
        String::from_utf8_lossy(&git_try(&repo, &["diff", "--cached", "--name-only"]).stdout),
        staged,
        "…and the squashed payload still staged, not folded into a jigc commit",
    );
    assert_eq!(
        commit_count(&repo),
        before,
        "…and nothing committed; output:\n{text}",
    );
}

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

    // …and the two members it is NOT exempt from still refuse — one posture cell and one
    // operation cell, both from the shared fixture builder.
    for state in [GitState::Detached, GitState::Merge] {
        let fixture = GitStateRepo::build(state);
        let repo = fixture.repo();
        let before = commit_count(&repo);
        let out = run_jigc(&repo, &fixture.home(), &["setup".to_string()]);
        let text = output_of(&out);
        let member =
            adjudicated_member(setup_acts(), state).expect("setup adjudicates this member");
        assert!(
            !out.status.success(),
            "`jigc setup` under `{}` must refuse; output:\n{text}",
            state.name(),
        );
        assert!(
            text.contains(&format!("blocking · {}", member.code())),
            "`jigc setup` under `{}` must name the member it adjudicates; output:\n{text}",
            state.name(),
        );
        assert_eq!(
            commit_count(&repo),
            before,
            "`jigc setup` under `{}` must commit nothing",
            state.name(),
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
    let fixture = GitStateRepo::build(GitState::Unborn);
    let repo = fixture.repo();

    let out = run_jigc(
        &repo,
        &fixture.home(),
        &[
            "milestone".to_string(),
            "create".to_string(),
            "Axis milestone".to_string(),
        ],
    );
    let text = output_of(&out);
    assert!(
        !out.status.success() && text.contains("blocking · repo.head-unborn"),
        "`jigc milestone create` on an unborn HEAD must refuse; output:\n{text}",
    );
    assert_eq!(
        commit_count(&repo),
        0,
        "the refusal must leave the repository unborn",
    );
    let milestones = repo.join(".jigc").join("milestones");
    assert!(
        !milestones.exists(),
        "the refusal must write no milestone area at {milestones:?}",
    );
    let mut with_empty_tree = Vec::new();
    collect_files_containing(repo.join(".jigc"), EMPTY_TREE_SHA, &mut with_empty_tree);
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
    let fixture = GitStateRepo::build(GitState::Merge);
    let repo = fixture.repo();
    let merge_head = git_dir_of(&repo).join("MERGE_HEAD");
    let before = commit_count(&repo);

    let out = run_jigc(
        &repo,
        &fixture.home(),
        &[
            "task".to_string(),
            "finalize".to_string(),
            AXIS_ID.to_string(),
            "--carry-staged".to_string(),
        ],
    );
    let text = output_of(&out);
    assert!(
        !out.status.success() && text.contains("blocking · repo.operation-in-progress"),
        "`--carry-staged` must not carry a door past a live merge; output:\n{text}",
    );
    assert!(
        merge_head.exists(),
        "the refusal must leave MERGE_HEAD in place — it refuses the merge, never concludes it",
    );
    assert_eq!(
        commit_count(&repo),
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
    let text = output_of(&finalize);
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
    let text = output_of(&blocked);
    assert!(
        !blocked.status.success() && text.contains("blocking · repo.operation-in-progress"),
        "the exemption covers the detached member and nothing else; output:\n{text}",
    );
}

// ---------------------------------------------------------------------------
// Harness.
// ---------------------------------------------------------------------------

/// What a refusing cell must leave exactly as it found it: the markers git wrote, HEAD,
/// and the commit count over **all** refs (so a commit landed on a detached HEAD, which
/// no branch contains, is still seen).
#[derive(Debug, PartialEq, Eq)]
struct Snapshot {
    markers: Vec<&'static str>,
    head: String,
    commits: u32,
}

impl Snapshot {
    /// Read the three facts back. The marker half asks [`MARKER_UNIVERSE`] — the fixture
    /// builder's own universe — rather than a list typed here, so a marker a new state
    /// introduces is carried into this comparison by adding it in one place.
    fn of(repo: &Path, git_dir: &Path) -> Snapshot {
        Snapshot {
            markers: MARKER_UNIVERSE
                .iter()
                .copied()
                .filter(|marker| git_dir.join(marker).exists())
                .collect(),
            head: head_sha(repo),
            commits: commit_count(repo),
        }
    }
}

/// The **worktree's own** git dir, absolute — asked of git rather than joined onto
/// `.git`, because a linked worktree's markers live in `.git/worktrees/<name>/`.
fn git_dir_of(repo: &Path) -> PathBuf {
    let out = git_try(repo, &["rev-parse", "--absolute-git-dir"]);
    assert!(
        out.status.success(),
        "git rev-parse --absolute-git-dir failed in {}: {}",
        repo.display(),
        String::from_utf8_lossy(&out.stderr),
    );
    PathBuf::from(String::from_utf8_lossy(&out.stdout).trim().to_string())
}

/// Everything the binary printed, both streams — a finding reaches stderr and the
/// surrounding prose can reach either, so a cell that read one stream could pass over a
/// refusal printed on the other.
fn output_of(out: &Output) -> String {
    format!(
        "{}{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr),
    )
}

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
/// being built, and for the read-back probes.
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

/// Drive the real binary in `repo` with an isolated `HOME` and no inherited
/// `JIGC_PACK_DIR` — a suite that inherited one would read a different pack than the one
/// it claims to drive.
fn run_jigc(repo: &Path, home: &Path, args: &[String]) -> Output {
    Command::new(env!("CARGO_BIN_EXE_jigc"))
        .args(args)
        .current_dir(repo)
        .env("HOME", home)
        .env_remove("JIGC_PACK_DIR")
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
