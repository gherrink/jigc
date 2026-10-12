//! The route catalog's cases for `jigc uninstall` — the family's table, declared by
//! `crates/cli/tests/route_catalog.rs`, which says what a case is held to.
//!
//! **What stands here** is every blocking site of `jigc uninstall`, eighteen. The
//! refusals a teardown meets before it removes anything: no repository to tear down in, or
//! no `$HOME` (`locate_finding`, two sites); the adapter profile not loaded (reached
//! through `JIGC_ADAPTERS_DIR` and no other way); and the four guards over what `.jigc/`
//! holds the only copy of, each with the refusal it raises where it could not look — a
//! path under `.jigc/worktrees/` that holds content, a byte jigc did not write inside a
//! working area or the parking home, an open task's staged doc, and any other file under
//! `.jigc/` that no index has a copy of. Then the seven steps that remove, each a site:
//! `.jigc/` itself, the reference in `CLAUDE.md`, the three kinds of entry in the settings
//! file, the `pre-commit` hook, the guide. Each of the eighteen has a row of [`READINGS`].
//!
//! **A step that removes has one route for every error of its callee** — *ensure … is
//! writable* over one entry it names — so its causes are the callee's errors, and a case
//! follows the one act the route offers. **And the first step removes `.jigc/`**: a
//! refusal at any step behind it finds the workbench — and the install's record of the
//! settings entries it added — gone, and the route's re-run is a second teardown over what
//! the first left.
//!
//! **Every case starts from somebody's repository with an install that stands**
//! ([`installed!`]): their own `CLAUDE.md`, their own settings file with a permit of their
//! own, their own `pre-commit` hook, a file jigc never wrote — all committed — and then
//! `jigc setup`, run by the binary under test. So the install is that release's own, and
//! what a teardown must leave is something a case can name.
//!
//! **`jigc uninstall` removes things, so every case says what must not change.** Outside
//! the repository, [`OUTSIDE`]. Inside it, the files the command has no business touching
//! ([`THEIRS`], and [`STANDING`] where the case's route leads to no teardown at all — there
//! the install's own files are witnesses too: a refused teardown removes nothing). And
//! where a route leads to a teardown, what must hold is all of it ([`torn_down!`]): the
//! workbench and the guide gone, jigc's lines out of the three files it was merged into —
//! and their owner's lines still in them.
//!
//! **A guard's route is followed every way it offers**, and each prints the consent last:
//! `jigc uninstall --force`, which deletes what the guard refused over. A case that ends
//! `-forced` follows that; what it destroys was the refusal's subject and is no witness.
//!
//! **A route here often prints a line with a word in angle brackets** —
//! `jigc task discard <task-id> --force` — where the id is in the refusal's listing and not
//! in the route. Such a line is a shape the reader fills (the surface contract's span
//! fence), and a case fills it with the value the refusal lists, by index
//! ([`Act::RunFilled`]). One route prints `<milestone-id>` over a refusal that lists no
//! milestone: that line cannot be filled, and its case is red as it stands — **the route
//! owes every value it needs that is not the reader's own to choose**, the human's ruling
//! of 2026-10-12 (`DECISIONS.md` → *The route owes every value it needs that is not the reader's own to choose*).

use super::{Reading, Reason, Why};
use crate::support::route_case::{Act, Case, Check, Filled, Operand, Over, Place, Step, Var};

// The sites, as the committed list of refusal sites and the suite's claims name them.
const NOT_IN_A_REPOSITORY: &str =
    "crates/cli/src/locate.rs::locate_finding::uninstall.repo-root#dynamic-1";
const NO_HOME_SITE: &str =
    "crates/cli/src/locate.rs::locate_finding::uninstall.repo-root#dynamic-2";
const PROFILE_LOAD: &str = "crates/cli/src/setup.rs::run_uninstall::uninstall.profile-load#1";
const DIRTY_WORKTREE: &str =
    "crates/cli/src/setup.rs::dirty_worktree_finding::uninstall.dirty-worktree#1";
const UNVERIFIED_WORKTREES: &str =
    "crates/cli/src/setup.rs::unverified_worktrees_finding::uninstall.dirty-worktree#1";
const FOREIGN_BYTES: &str =
    "crates/cli/src/setup.rs::foreign_bytes_finding::uninstall.foreign-bytes#1";
const UNVERIFIED_FOREIGN: &str =
    "crates/cli/src/setup.rs::unverified_foreign_finding::uninstall.foreign-bytes#1";
const STAGED_PROSE: &str =
    "crates/cli/src/setup.rs::staged_prose_finding::uninstall.staged-prose#1";
const UNVERIFIED_PROSE: &str =
    "crates/cli/src/task.rs::unverified_prose_finding::uninstall.staged-prose#1";
const UNTRACKED_WORKBENCH: &str = "crates/cli/src/setup.rs::untracked_workbench_finding::uninstall.untracked-workbench-file#1";
const UNVERIFIED_WORKBENCH: &str = "crates/cli/src/setup.rs::unverified_workbench_finding::uninstall.untracked-workbench-file#1";

// The files the install is merged into or adds, from the repository root.
const CLAUDE_MD: &str = "CLAUDE.md";
const SETTINGS_JSON: &str = ".claude/settings.json";
const HOOK: &str = ".git/hooks/pre-commit";
const GUIDE_MD: &str = ".claude/skills/jigc/SKILL.md";
const WORKBENCH: &str = ".jigc";

/// Their settings file: one permit, and theirs.
const THEIR_SETTINGS: &str =
    "{\n  \"permissions\": {\n    \"allow\": [\n      \"Bash(mine:*)\"\n    ]\n  }\n}\n";

/// **Somebody's repository, and then an install that stands**: their instruction file,
/// their settings — this file — their `pre-commit` hook, a file jigc never writes, and a
/// file outside the repository — committed — and `jigc setup` over it, run by the binary
/// under test. The steps a case adds follow.
macro_rules! installed_over {
    ($settings:expr $(, $plant:expr)* $(,)?) => {
        &[
            Step::File(Place::Repo(CLAUDE_MD), "mine\n"),
            Step::File(Place::Repo(SETTINGS_JSON), $settings),
            Step::Script(Place::Repo(HOOK), "#!/bin/sh\necho mine\n"),
            Step::File(Place::Repo("notes.md"), "mine\n"),
            Step::File(Place::Outside("theirs"), "theirs\n"),
            Step::Git(&["add", "--all"]),
            Step::Git(&["commit", "-q", "-m", "theirs"]),
            Step::Jigc(&["setup"]),
            $($plant,)*
        ]
    };
}

/// [`installed_over!`] their settings with a permit of their own and nothing of jigc's —
/// the state every case starts from but the two that need a record of another shape.
macro_rules! installed {
    ($($plant:expr),* $(,)?) => {
        installed_over![THEIR_SETTINGS $(, $plant)*]
    };
}

/// The file outside the repository that no teardown has any business with.
const OUTSIDE: &[&str] = &["theirs"];

/// The files inside the repository that jigc did not write and a teardown does not edit:
/// the sandbox's own committed file, and their notes.
const THEIRS: &[&str] = &["README.md", "notes.md"];

/// [`THEIRS`], and every file of the install — the witnesses of a case whose route leads
/// to no teardown: a refused `jigc uninstall` removes nothing and edits nothing.
const STANDING: &[&str] = &[
    "README.md",
    "notes.md",
    ".jigc/AGENT.md",
    ".jigc/settings-entries.json",
    CLAUDE_MD,
    SETTINGS_JSON,
    HOOK,
    GUIDE_MD,
];

/// **What stands once a teardown is through**: the workbench and the guide are gone, jigc's
/// lines are out of the three files the install was merged into, and their owner's lines
/// are still in them. The checks a case adds follow.
macro_rules! torn_down {
    ($($also:expr),* $(,)?) => {
        &[
            Check::Absent(Place::Repo(WORKBENCH)),
            Check::Absent(Place::Repo(GUIDE_MD)),
            Check::Lacks(Place::Repo(CLAUDE_MD), ".jigc/AGENT.md"),
            Check::Contains(Place::Repo(CLAUDE_MD), "mine\n"),
            Check::Lacks(Place::Repo(SETTINGS_JSON), "jigc"),
            Check::Contains(Place::Repo(SETTINGS_JSON), "\"Bash(mine:*)\""),
            Check::Lacks(Place::Repo(HOOK), "jigc"),
            Check::Contains(Place::Repo(HOOK), "echo mine\n"),
            $($also,)*
        ]
    };
}

/// What every case of `jigc uninstall` shares.
const UNINSTALL: Case = Case {
    name: "",
    site: "",
    cause: "",
    state: &[],
    refusing: &["uninstall"],
    from: Place::Repo(""),
    env: &[],
    acts: &[],
    holds: &[],
    witnesses: OUTSIDE,
    untouched: THEIRS,
    doubt: None,
    over: None,
};

/// Take a permission away: a state has no step for one, so a `git` alias does it — a shell
/// line git runs at the repository's root.
macro_rules! shut {
    ($mode:literal, $path:literal) => {
        Step::Git(&[
            "-c",
            concat!("alias.shut=!chmod ", $mode, " ", $path),
            "shut",
        ])
    };
}

/// The two readings of *make sure … is readable*.
const WHICH_READABLE: &str = "*make sure `.jigc/…` is readable*: (a) the entry the route \
                              names, given its owner's read bit — followed here: it had it, \
                              or the bit is not what the walk was refused for; (b) every \
                              directory under it, readable and searchable — the frame has \
                              no act for a tree, and the entry at fault is named by the \
                              refusal's message where it is named at all";

// ---------------------------------------------------------------------------
// `locate_finding` · `uninstall.repo-root`
// ---------------------------------------------------------------------------

/// **A teardown run where there is no repository, followed the two ways its route
/// offers**: run from inside the target repository — the sandbox's, where the install
/// stands — and `git init` here first, then the command again, which finds nothing to
/// remove here and must leave the install over there as it is. `$made` is where the
/// route's `git init` puts git's directory.
macro_rules! outside_ways {
    ($prefix:literal, $cause:expr, $made:literal, plant: [$($plant:expr),* $(,)?] $(,)?) => {
        [
            Case {
                name: concat!($prefix, "-run-inside-the-repository"),
                site: NOT_IN_A_REPOSITORY,
                cause: $cause,
                state: installed![$($plant,)*],
                from: Place::Outside(""),
                acts: &[Act::RerunInTheRepository],
                holds: torn_down![],
                ..UNINSTALL
            },
            Case {
                name: concat!($prefix, "-a-repository-made-here"),
                site: NOT_IN_A_REPOSITORY,
                cause: $cause,
                state: installed![$($plant,)*],
                from: Place::Outside(""),
                acts: &[Act::Run(0), Act::Rerun],
                holds: &[Check::Directory(Place::Outside($made))],
                untouched: STANDING,
                ..UNINSTALL
            },
        ]
    };
}

const OUTSIDE_A_REPOSITORY: [Case; 2] = outside_ways!(
    "uninstall-outside-a-repository",
    "the command is run in a directory that is in no repository: neither it nor a directory \
     above it holds a `.git`",
    ".git",
    plant: [],
);
const BESIDE_A_GIT_LINK_TO_NOTHING: [Case; 2] = outside_ways!(
    "uninstall-beside-a-git-link-to-nothing",
    "the directory's `.git` is a link that leads to nothing, and no directory above it holds \
     a `.git`",
    "gits-directory",
    plant: [Step::LinkTo(Place::Outside(".git"), "gits-directory")],
);

const HOME_NOT_SET: [Case; 1] = [Case {
    name: "uninstall-home-not-set-run-as-the-route-prints",
    site: NO_HOME_SITE,
    cause: "`$HOME` is not set, and the command is run inside a repository",
    state: installed![],
    env: &[Var::Unset("HOME")],
    acts: &[Act::Run(0)],
    holds: torn_down![],
    untouched: STANDING,
    ..UNINSTALL
}];

// ---------------------------------------------------------------------------
// `run_uninstall` · `uninstall.profile-load`
// ---------------------------------------------------------------------------

/// The one road to a profile that is not the binary's own.
const ADAPTERS: &[Var] = &[Var::Path("JIGC_ADAPTERS_DIR", Place::Outside("adapters"))];
const PROFILE: Place = Place::Outside("adapters/claude-code.yaml");

/// **The profile not loaded, followed the one way its route leaves**: the command again —
/// *reinstall jigc* puts the same binary where the binary is, and the route prints no
/// command line. The install was made by the binary's own profile (a state's step does not
/// run in the case's environment); the profile's bytes are a witness, and so is every file
/// of the install.
macro_rules! profile_refused {
    ($name:literal, $cause:expr, plant: [$($plant:expr),* $(,)?] $(,)?) => {
        [Case {
            name: $name,
            site: PROFILE_LOAD,
            cause: $cause,
            state: installed![$($plant,)*],
            env: ADAPTERS,
            acts: &[Act::Rerun],
            holds: torn_down![],
            witnesses: &["theirs", "adapters/claude-code.yaml"],
            untouched: STANDING,
            ..UNINSTALL
        }]
    };
}

const NO_PROFILE_IN_THE_DIRECTORY: [Case; 1] = profile_refused!(
    "uninstall-profile-not-in-the-adapters-directory",
    "the directory `JIGC_ADAPTERS_DIR` names holds no profile to read",
    plant: [Step::Dir(Place::Outside("adapters"))],
);
const PROFILE_NOT_TEXT: [Case; 1] = profile_refused!(
    "uninstall-profile-not-utf-8",
    "the profile in that directory is not UTF-8",
    plant: [
        Step::Dir(Place::Outside("adapters")),
        Step::Git(&[
            "-c",
            "alias.plant=!printf '\\377\\376' > ../outside/adapters/claude-code.yaml",
            "plant",
        ]),
    ],
);
const PROFILE_MALFORMED: [Case; 1] = profile_refused!(
    "uninstall-profile-malformed",
    "the profile in that directory is YAML that is no profile",
    plant: [Step::File(PROFILE, "assistant: claude-code\n")],
);

// ---------------------------------------------------------------------------
// `dirty_worktree_finding` · `uninstall.dirty-worktree` — the first guard
// ---------------------------------------------------------------------------

/// A fan-out, made by the binary under test: a milestone, one sub-task, and its worktree
/// under `.jigc/worktrees/` — registered with git, and on the milestone's roster.
const FAN_OUT: [Step; 3] = [
    Step::Jigc(&["milestone", "create", "Fan out"]),
    Step::Jigc(&["milestone", "add-task", "fan-out", "do a thing"]),
    Step::Jigc(&["milestone", "provision", "fan-out"]),
];
const SUB_TASK: &str = ".jigc/worktrees/do-a-thing";
const SUB_TASK_WORK: &str = ".jigc/worktrees/do-a-thing/work.txt";
const WORK: Step = Step::File(Place::Repo(SUB_TASK_WORK), "work\n");
const WORK_STAGED: Step = Step::Git(&["-C", SUB_TASK, "add", "work.txt"]);

/// A directory under `.jigc/worktrees/` that no git vouches for — what a copied repository
/// holds where the original had a worktree.
const COPIED: &str = ".jigc/worktrees/copied";
const COPIED_WORK: Step = Step::File(Place::Repo(".jigc/worktrees/copied/work.txt"), "work\n");

/// **A path under `.jigc/worktrees/` this repository has registered nowhere, followed the
/// two ways its route offers**: the work got out of the path — moved, the path being the
/// case's to name where the route says *those paths* and prints none — and the consent.
macro_rules! unregistered_ways {
    (
        $prefix:literal, $cause:expr, $out:expr,
        plant: [$($plant:expr),* $(,)?],
        moved: [$($moved:expr),* $(,)?] $(,)?
    ) => {
        [
            Case {
                name: concat!($prefix, "-moved-out"),
                site: DIRTY_WORKTREE,
                cause: $cause,
                state: installed![$($plant,)*],
                acts: &[$out, Act::Run(0)],
                holds: torn_down![$($moved,)*],
                ..UNINSTALL
            },
            Case {
                name: concat!($prefix, "-forced"),
                site: DIRTY_WORKTREE,
                cause: $cause,
                state: installed![$($plant,)*],
                acts: &[Act::Run(1)],
                holds: torn_down![],
                ..UNINSTALL
            },
        ]
    };
}

const A_DIRECTORY_NOTHING_VOUCHES_FOR: [Case; 2] = unregistered_ways!(
    "uninstall-worktree-path-a-directory-nothing-vouches-for",
    "a directory under `.jigc/worktrees/` holds content, and git reports no worktree of its \
     own there: registered nowhere in this repository",
    Act::MoveAway(Operand::Unprinted(COPIED)),
    plant: [COPIED_WORK],
    moved: [Check::RegularFile(Place::Moved(".jigc/worktrees/copied/work.txt"))],
);
const A_FILE_WHERE_A_WORKTREE_GOES: [Case; 2] = unregistered_ways!(
    "uninstall-worktree-path-a-file",
    "a plain file stands under `.jigc/worktrees/`",
    Act::MoveOut(Operand::Unprinted(".jigc/worktrees/stray")),
    plant: [Step::File(Place::Repo(".jigc/worktrees/stray"), "work\n")],
    moved: [Check::RegularFile(Place::Moved(".jigc/worktrees/stray"))],
);
/// Nothing can be read at the path: the route's own parenthesis says to *look at it and
/// move it aside or delete it yourself*, and neither can be done to a directory that may
/// be neither read nor written.
const A_PATH_NOTHING_CAN_BE_READ_AT: [Case; 2] = unregistered_ways!(
    "uninstall-worktree-path-unreadable",
    "a directory under `.jigc/worktrees/` may be neither read nor searched, so nothing \
     could be read there",
    Act::MoveAway(Operand::Unprinted(".jigc/worktrees/shut")),
    plant: [
        Step::File(Place::Repo(".jigc/worktrees/shut/kept"), "work\n"),
        shut!("000", ".jigc/worktrees/shut"),
    ],
    moved: [],
);

/// **A fan-out worktree with work in no commit** — registered here, so the route offers a
/// third way: the milestone abandoned, by a line that holds `<milestone-id>`. The refusal
/// lists the worktree's path and no milestone, and the id is the product's to give: the
/// line is run as printed, which no shell splits, and the case is a finding of the product
/// (the human's ruling of 2026-10-12, `DECISIONS.md` → *The route owes every value it needs that is not the reader's own to choose*).
const A_LIVE_WORKTREE: &str = "a fan-out sub-task's worktree, registered with this \
                               repository, holds a file in no commit";
const A_LIVE_FAN_OUT_WORKTREE: [Case; 3] = [
    Case {
        name: "uninstall-fan-out-worktree-uncommitted-work-moved-out",
        site: DIRTY_WORKTREE,
        cause: A_LIVE_WORKTREE,
        state: installed![FAN_OUT[0], FAN_OUT[1], FAN_OUT[2], WORK],
        acts: &[Act::MoveAway(Operand::Unprinted(SUB_TASK)), Act::Run(0)],
        holds: torn_down![Check::RegularFile(Place::Moved(SUB_TASK_WORK))],
        ..UNINSTALL
    },
    Case {
        name: "uninstall-fan-out-worktree-uncommitted-work-milestone-abandoned-as-printed",
        site: DIRTY_WORKTREE,
        cause: A_LIVE_WORKTREE,
        state: installed![FAN_OUT[0], FAN_OUT[1], FAN_OUT[2], WORK],
        acts: &[Act::Run(1), Act::Run(2)],
        holds: torn_down![],
        untouched: &["README.md", "notes.md", SUB_TASK_WORK],
        ..UNINSTALL
    },
    Case {
        name: "uninstall-fan-out-worktree-uncommitted-work-forced",
        site: DIRTY_WORKTREE,
        cause: A_LIVE_WORKTREE,
        state: installed![FAN_OUT[0], FAN_OUT[1], FAN_OUT[2], WORK],
        acts: &[Act::Run(3)],
        holds: torn_down![],
        ..UNINSTALL
    },
];

/// **What git's registration of a path holds, where the checkout is in order or gone** —
/// a staged path with no directory left, a commit no ref reaches, an operation git left
/// un-concluded. The route says each *has nothing to move out* and that the command for it
/// *is on its line* — a line of the refusal's listing, which is no part of the route: the
/// one way a case can follow is the consent.
macro_rules! registration_forced {
    ($name:literal, $cause:expr, plant: [$($plant:expr),* $(,)?] $(,)?) => {
        [Case {
            name: $name,
            site: DIRTY_WORKTREE,
            cause: $cause,
            state: installed![FAN_OUT[0], FAN_OUT[1], FAN_OUT[2], $($plant,)*],
            acts: &[Act::Run(3)],
            holds: torn_down![],
            ..UNINSTALL
        }]
    };
}

const A_REGISTRATION_WITH_NO_DIRECTORY: [Case; 1] = registration_forced!(
    "uninstall-fan-out-registration-without-a-directory-forced",
    "a fan-out worktree's directory is gone, and git's registration of the path still holds \
     a staged path in no commit",
    plant: [
        WORK,
        WORK_STAGED,
        Step::Git(&[
            "-c",
            "alias.gone=!mv .jigc/worktrees/do-a-thing ../outside/gone",
            "gone",
        ]),
    ],
);
const A_COMMIT_NO_REF_REACHES: [Case; 1] = registration_forced!(
    "uninstall-fan-out-worktree-commit-no-ref-reaches-forced",
    "a fan-out worktree is clean, and its `HEAD` holds a commit no ref reaches",
    plant: [
        WORK,
        WORK_STAGED,
        Step::Git(&["-C", SUB_TASK, "commit", "-q", "-m", "work"]),
    ],
);
const AN_OPERATION_LEFT_OPEN: [Case; 1] = registration_forced!(
    "uninstall-fan-out-worktree-mid-merge-forced",
    "a fan-out worktree is clean, and git has left a merge un-concluded in it",
    plant: [Step::Git(&[
        "-c",
        "alias.merging=!git -C .jigc/worktrees/do-a-thing rev-parse HEAD > \
         \"$(git -C .jigc/worktrees/do-a-thing rev-parse --absolute-git-dir)/MERGE_HEAD\"",
        "merging",
    ])],
);

/// **A registered path and an unregistered one at once** — the route's arm that says the
/// second *still blocks the teardown* after the abandon: both moved out, and the consent.
const BOTH_KINDS: &str = "a fan-out worktree registered here holds a file in no commit, and \
                          beside it stands a directory registered nowhere";
const A_REGISTERED_AND_AN_UNREGISTERED_PATH: [Case; 2] = [
    Case {
        name: "uninstall-worktree-paths-of-both-kinds-moved-out",
        site: DIRTY_WORKTREE,
        cause: BOTH_KINDS,
        state: installed![FAN_OUT[0], FAN_OUT[1], FAN_OUT[2], WORK, COPIED_WORK],
        acts: &[
            Act::MoveAway(Operand::Unprinted(SUB_TASK)),
            Act::MoveAway(Operand::Unprinted(COPIED)),
            Act::Run(0),
        ],
        holds: torn_down![
            Check::RegularFile(Place::Moved(SUB_TASK_WORK)),
            Check::RegularFile(Place::Moved(".jigc/worktrees/copied/work.txt")),
        ],
        ..UNINSTALL
    },
    Case {
        name: "uninstall-worktree-paths-of-both-kinds-forced",
        site: DIRTY_WORKTREE,
        cause: BOTH_KINDS,
        state: installed![FAN_OUT[0], FAN_OUT[1], FAN_OUT[2], WORK, COPIED_WORK],
        acts: &[Act::Run(3)],
        holds: torn_down![],
        ..UNINSTALL
    },
];

// ---------------------------------------------------------------------------
// `unverified_worktrees_finding` · `uninstall.dirty-worktree` — the guard could not look
// ---------------------------------------------------------------------------

const WORKTREES_ROOT_UNREADABLE: &str = "`.jigc/worktrees/` is there and cannot be listed";
const THE_WORKTREES_ROOT_UNREADABLE: [Case; 3] = [
    Case {
        name: "uninstall-worktrees-root-unreadable-made-readable",
        site: UNVERIFIED_WORKTREES,
        cause: WORKTREES_ROOT_UNREADABLE,
        state: installed![
            Step::Dir(Place::Repo(".jigc/worktrees")),
            shut!("000", ".jigc/worktrees"),
        ],
        acts: &[Act::MakeReadable(Operand::Span(0)), Act::Run(2)],
        holds: torn_down![],
        ..UNINSTALL
    },
    Case {
        name: "uninstall-worktrees-root-unreadable-worktrees-removed-as-printed",
        site: UNVERIFIED_WORKTREES,
        cause: WORKTREES_ROOT_UNREADABLE,
        state: installed![
            Step::Dir(Place::Repo(".jigc/worktrees")),
            shut!("000", ".jigc/worktrees"),
        ],
        // `git worktree list`, a line that is complete, as printed. `git worktree
        // remove` names a command, and the operand the route speaks of is a worktree
        // the list names: here it names the main checkout and no other, so there is none
        // to hand it, and a name the case names no operand for is no act. Then the
        // printed re-run.
        acts: &[Act::Run(3), Act::Run(2)],
        holds: torn_down![],
        untouched: STANDING,
        ..UNINSTALL
    },
    Case {
        name: "uninstall-worktrees-root-unreadable-forced",
        site: UNVERIFIED_WORKTREES,
        cause: WORKTREES_ROOT_UNREADABLE,
        state: installed![
            Step::Dir(Place::Repo(".jigc/worktrees")),
            shut!("000", ".jigc/worktrees"),
        ],
        acts: &[Act::Run(5)],
        holds: torn_down![],
        ..UNINSTALL
    },
];

// ---------------------------------------------------------------------------
// `foreign_bytes_finding` · `uninstall.foreign-bytes` — the second guard
// ---------------------------------------------------------------------------

/// An open task, minted by the binary under test: its working area is
/// `.jigc/tasks/write-a-note/`, with its staged `commit:write-a-note` doc.
const A_TASK: Step = Step::Jigc(&["start", "--workflow", "decided-task", "write a note"]);

/// **A byte jigc did not write, where the route prints no path of it** — the route says
/// *the paths above* and prints `.jigc/displaced/`, so the one way a case can follow is the
/// consent.
macro_rules! foreign_forced {
    ($name:literal, $cause:expr, plant: [$($plant:expr),* $(,)?] $(,)?) => {
        [Case {
            name: $name,
            site: FOREIGN_BYTES,
            cause: $cause,
            state: installed![$($plant,)*],
            acts: &[Act::Run(3)],
            holds: torn_down![],
            ..UNINSTALL
        }]
    };
}

const A_STRAY_IN_A_TASKS_AREA: [Case; 1] = foreign_forced!(
    "uninstall-foreign-file-in-a-task-area-forced",
    "a file jigc did not write stands inside an open task's working area",
    plant: [
        A_TASK,
        Step::File(Place::Repo(".jigc/tasks/write-a-note/notes.txt"), "mine\n"),
    ],
);
const A_STRAY_IN_A_MILESTONES_AREA: [Case; 1] = foreign_forced!(
    "uninstall-foreign-file-in-a-milestone-area-forced",
    "a file jigc did not write stands inside a milestone's working area",
    plant: [
        FAN_OUT[0],
        Step::File(Place::Repo(".jigc/milestones/fan-out/notes.txt"), "mine\n"),
    ],
);
const A_STRAY_BESIDE_THE_AREAS: [Case; 1] = foreign_forced!(
    "uninstall-foreign-file-beside-the-task-areas-forced",
    "a plain file stands in `.jigc/tasks/` beside the working areas",
    plant: [Step::File(Place::Repo(".jigc/tasks/stray"), "mine\n")],
);
const A_STRAY_IN_A_TRANSIENT_DIRECTORY: [Case; 1] = foreign_forced!(
    "uninstall-foreign-file-in-the-state-directory-forced",
    "a file that is none of jigc's own stands in a transient directory no working area \
     lives in — `.jigc/state/`",
    plant: [Step::File(Place::Repo(".jigc/state/mine.txt"), "mine\n")],
);

/// The one cause whose path the route prints: the parking home, moved out whole.
const PARKED: &str = "a file is parked under `.jigc/displaced/`";
const A_PARKED_FILE: [Case; 2] = [
    Case {
        name: "uninstall-parked-file-parking-home-moved-out",
        site: FOREIGN_BYTES,
        cause: PARKED,
        state: installed![Step::File(
            Place::Repo(".jigc/displaced/write-a-note/kept.md"),
            "mine\n"
        )],
        acts: &[Act::MoveAway(Operand::Span(1)), Act::Run(2)],
        holds: torn_down![Check::RegularFile(Place::Moved(
            ".jigc/displaced/write-a-note/kept.md"
        ))],
        ..UNINSTALL
    },
    Case {
        name: "uninstall-parked-file-forced",
        site: FOREIGN_BYTES,
        cause: PARKED,
        state: installed![Step::File(
            Place::Repo(".jigc/displaced/write-a-note/kept.md"),
            "mine\n"
        )],
        acts: &[Act::Run(3)],
        holds: torn_down![],
        ..UNINSTALL
    },
];

// ---------------------------------------------------------------------------
// `unverified_foreign_finding` · `uninstall.foreign-bytes` — the guard could not look
// ---------------------------------------------------------------------------

/// **The walk for foreign bytes refused, followed the two ways its route offers**: the
/// `.jigc/` the route names made readable, and the consent.
macro_rules! unlisted_ways {
    ($prefix:literal, $cause:expr, plant: [$($plant:expr),* $(,)?] $(,)?) => {
        [
            Case {
                name: concat!($prefix, "-made-readable"),
                site: UNVERIFIED_FOREIGN,
                cause: $cause,
                state: installed![$($plant,)*],
                acts: &[Act::MakeReadable(Operand::Span(0)), Act::Run(1)],
                holds: torn_down![],
                doubt: Some(WHICH_READABLE),
                ..UNINSTALL
            },
            Case {
                name: concat!($prefix, "-forced"),
                site: UNVERIFIED_FOREIGN,
                cause: $cause,
                state: installed![$($plant,)*],
                acts: &[Act::Run(2)],
                holds: torn_down![],
                ..UNINSTALL
            },
        ]
    };
}

const A_DIRECTORY_OF_THE_WALK_UNLISTABLE: [Case; 2] = unlisted_ways!(
    "uninstall-task-areas-unlistable",
    "a directory the walk lists — `.jigc/tasks/` — is there and cannot be listed",
    plant: [
        Step::Dir(Place::Repo(".jigc/tasks")),
        shut!("000", ".jigc/tasks"),
    ],
);
const THE_WORKBENCH_UNSEARCHABLE: [Case; 2] = unlisted_ways!(
    "uninstall-workbench-unsearchable",
    "`.jigc` itself may not be searched, so nothing under it can be asked about",
    plant: [shut!("000", ".jigc")],
);

// ---------------------------------------------------------------------------
// `staged_prose_finding` · `uninstall.staged-prose` — the third guard
// ---------------------------------------------------------------------------

/// **An open task's staged doc, followed the three ways its route offers**: read and
/// thrown away, by the two lines the route prints for that; landed, by the line it prints
/// for that; and the consent. The first two hold `<task-id>` — and the first of them
/// `<address>` — where the refusal's listing has the id and the doc's address: `<task>:
/// <address>`, its first and its second listed value.
///
/// *Landed* is the route's way **once its doc is complete**, and it says the line refuses
/// while a required slot is empty. So that case's state is the task with its doc complete
/// and a change of its own staged — the one state the way's words are about; a state that
/// left the doc as minted would follow a way the route does not offer it.
const A_STAGED_DOC: &str = "an open task stages a doc that no commit has a copy of";
/// The open task's doc made complete, and a change of the task's own staged: the type and
/// the summary the `commit` doc requires, set through the binary under test, and a new file.
const ITS_DOC_COMPLETE_AND_A_CHANGE_STAGED: [Step; 5] = [
    Step::Jigc(&[
        "doc",
        "set-field",
        "commit:write-a-note#header/type",
        "--task",
        "write-a-note",
        "--value",
        "docs",
    ]),
    Step::File(Place::Outside("summary"), "write a note\n"),
    Step::Jigc(&[
        "doc",
        "set-slot",
        "commit:write-a-note#summary",
        "--task",
        "write-a-note",
        "--from-file",
        "../outside/summary",
    ]),
    Step::File(Place::Repo("note.md"), "a note\n"),
    Step::Git(&["add", "note.md"]),
];
const AN_OPEN_TASKS_STAGED_DOC: [Case; 3] = [
    Case {
        name: "uninstall-staged-doc-read-and-discarded-as-printed",
        site: STAGED_PROSE,
        cause: A_STAGED_DOC,
        state: installed![A_TASK],
        acts: &[
            Act::RunFilled(Filled {
                line: 0,
                with: &[1, 0],
            }),
            Act::RunFilled(Filled {
                line: 1,
                with: &[0],
            }),
            Act::Run(3),
        ],
        holds: torn_down![],
        ..UNINSTALL
    },
    Case {
        name: "uninstall-staged-doc-finalized-as-printed",
        site: STAGED_PROSE,
        cause: A_STAGED_DOC,
        state: installed![
            A_TASK,
            ITS_DOC_COMPLETE_AND_A_CHANGE_STAGED[0],
            ITS_DOC_COMPLETE_AND_A_CHANGE_STAGED[1],
            ITS_DOC_COMPLETE_AND_A_CHANGE_STAGED[2],
            ITS_DOC_COMPLETE_AND_A_CHANGE_STAGED[3],
            ITS_DOC_COMPLETE_AND_A_CHANGE_STAGED[4],
        ],
        acts: &[
            Act::RunFilled(Filled {
                line: 2,
                with: &[0],
            }),
            Act::Run(3),
        ],
        holds: torn_down![Check::RegularFile(Place::Repo("note.md"))],
        ..UNINSTALL
    },
    Case {
        name: "uninstall-staged-doc-forced",
        site: STAGED_PROSE,
        cause: A_STAGED_DOC,
        state: installed![A_TASK],
        acts: &[Act::Run(4)],
        holds: torn_down![],
        ..UNINSTALL
    },
];

// ---------------------------------------------------------------------------
// `unverified_prose_finding` · `uninstall.staged-prose` — the guard could not look
// ---------------------------------------------------------------------------

/// The one state that fails this walk and not the one before it: a task's `docs/` that
/// may be listed and not searched. The foreign-byte guard reads the names and asks nothing
/// of the entries; this one asks what each staged entry is.
const DOCS_UNSEARCHABLE: &str = "an open task's `docs/` may be listed and not searched, so \
                                 what a staged entry is cannot be asked";
const A_STAGED_ENTRY_THAT_CANNOT_BE_ASKED_ABOUT: [Case; 2] = [
    Case {
        name: "uninstall-staged-docs-unsearchable-made-readable",
        site: UNVERIFIED_PROSE,
        cause: DOCS_UNSEARCHABLE,
        state: installed![A_TASK, shut!("600", ".jigc/tasks/write-a-note/docs")],
        acts: &[Act::MakeReadable(Operand::Span(0)), Act::Run(1)],
        holds: torn_down![],
        doubt: Some(WHICH_READABLE),
        ..UNINSTALL
    },
    Case {
        name: "uninstall-staged-docs-unsearchable-forced",
        site: UNVERIFIED_PROSE,
        cause: DOCS_UNSEARCHABLE,
        state: installed![A_TASK, shut!("600", ".jigc/tasks/write-a-note/docs")],
        acts: &[Act::Run(2)],
        holds: torn_down![],
        ..UNINSTALL
    },
];

// ---------------------------------------------------------------------------
// `untracked_workbench_finding` · `uninstall.untracked-workbench-file` — the fourth guard
// ---------------------------------------------------------------------------

const LOCAL_CONFIG: &str = ".jigc/config/local.yaml";
const A_LOCAL_CONFIG: Step = Step::File(Place::Repo(LOCAL_CONFIG), "mine: 1\n");
const PACKS_YAML: &str = ".jigc/config/packs.yaml";

/// **A file under `.jigc/` that git takes and no index has, followed the four ways its
/// route offers**: put where it can be recovered — by the `git add` line the route prints,
/// which holds `<path>`, and by a commit, which is where *recovered* is beyond doubt —
/// deleted, and the consent. The route prints no path, so the file is the case's to name.
const NEVER_TRACKED: &str = "a file under `.jigc/`, outside the transient directories, is in \
                             no index, and git does not ignore it";
const A_FILE_NO_INDEX_HAS: [Case; 4] = [
    Case {
        name: "uninstall-untracked-workbench-file-added-as-printed",
        site: UNTRACKED_WORKBENCH,
        cause: NEVER_TRACKED,
        state: installed![A_LOCAL_CONFIG],
        acts: &[
            Act::RunFilled(Filled {
                line: 0,
                with: &[0],
            }),
            Act::Run(2),
        ],
        holds: torn_down![],
        ..UNINSTALL
    },
    Case {
        name: "uninstall-untracked-workbench-file-committed",
        site: UNTRACKED_WORKBENCH,
        cause: NEVER_TRACKED,
        state: installed![A_LOCAL_CONFIG],
        acts: &[Act::Commit(Operand::Unprinted(LOCAL_CONFIG)), Act::Run(2)],
        holds: torn_down![],
        ..UNINSTALL
    },
    Case {
        name: "uninstall-untracked-workbench-file-deleted",
        site: UNTRACKED_WORKBENCH,
        cause: NEVER_TRACKED,
        state: installed![A_LOCAL_CONFIG],
        acts: &[Act::Remove(Operand::Unprinted(LOCAL_CONFIG)), Act::Run(2)],
        holds: torn_down![],
        ..UNINSTALL
    },
    Case {
        name: "uninstall-untracked-workbench-file-forced",
        site: UNTRACKED_WORKBENCH,
        cause: NEVER_TRACKED,
        state: installed![A_LOCAL_CONFIG],
        acts: &[Act::Run(3)],
        holds: torn_down![],
        ..UNINSTALL
    },
];

/// The classifier's second leg: the index lists the path and carries other bytes.
const EDITED: &str = "a file of the install that git tracks under `.jigc/` is edited, and \
                      the edit is not staged";
const A_TRACKED_FILE_EDITED: [Case; 2] = [
    Case {
        name: "uninstall-edited-workbench-file-committed",
        site: UNTRACKED_WORKBENCH,
        cause: EDITED,
        state: installed![Step::File(Place::Repo(PACKS_YAML), "# mine\n")],
        acts: &[Act::Commit(Operand::Unprinted(PACKS_YAML)), Act::Run(2)],
        holds: torn_down![],
        ..UNINSTALL
    },
    Case {
        name: "uninstall-edited-workbench-file-forced",
        site: UNTRACKED_WORKBENCH,
        cause: EDITED,
        state: installed![Step::File(Place::Repo(PACKS_YAML), "# mine\n")],
        acts: &[Act::Run(3)],
        holds: torn_down![],
        ..UNINSTALL
    },
];

/// Their own ignore rule, committed, and a file it covers.
const AN_IGNORE_RULE_OF_THEIRS: [Step; 3] = [
    Step::File(Place::Repo(".jigc/config/.gitignore"), "local.yaml\n"),
    Step::Git(&["add", "--all"]),
    Step::Git(&["commit", "-q", "-m", "their ignore rule"]),
];

/// **Every listed path is one git ignores** — the route prints `.jigc/` and no path of the
/// file, so the one way a case can follow is the consent.
const AN_IGNORED_FILE: [Case; 1] = [Case {
    name: "uninstall-ignored-workbench-file-forced",
    site: UNTRACKED_WORKBENCH,
    cause: "a file under `.jigc/` is in no index, and a rule of the repository's own \
            ignores it — every listed path is ignored",
    state: installed![
        AN_IGNORE_RULE_OF_THEIRS[0],
        AN_IGNORE_RULE_OF_THEIRS[1],
        AN_IGNORE_RULE_OF_THEIRS[2],
        A_LOCAL_CONFIG,
    ],
    acts: &[Act::Run(3)],
    holds: torn_down![],
    ..UNINSTALL
}];

/// **An ignored path and one git takes, together** — the one arm that prints a listed
/// path: the ignored file, moved out by its span; the other is left to the `git add` line,
/// which holds `<path>`.
const BOTH_LISTED: &str = "two files under `.jigc/` are in no index, and git ignores one of \
                           them";
const AN_IGNORED_FILE_AND_ONE_GIT_TAKES: [Case; 2] = [
    Case {
        name: "uninstall-workbench-files-one-ignored-moved-out-and-one-added-as-printed",
        site: UNTRACKED_WORKBENCH,
        cause: BOTH_LISTED,
        state: installed![
            AN_IGNORE_RULE_OF_THEIRS[0],
            AN_IGNORE_RULE_OF_THEIRS[1],
            AN_IGNORE_RULE_OF_THEIRS[2],
            A_LOCAL_CONFIG,
            Step::File(Place::Repo(".jigc/config/other.yaml"), "mine: 2\n"),
        ],
        acts: &[
            Act::MoveOut(Operand::Span(3)),
            Act::RunFilled(Filled {
                line: 1,
                with: &[1],
            }),
            Act::Run(0),
        ],
        holds: torn_down![Check::RegularFile(Place::Moved(LOCAL_CONFIG))],
        ..UNINSTALL
    },
    Case {
        name: "uninstall-workbench-files-one-ignored-forced",
        site: UNTRACKED_WORKBENCH,
        cause: BOTH_LISTED,
        state: installed![
            AN_IGNORE_RULE_OF_THEIRS[0],
            AN_IGNORE_RULE_OF_THEIRS[1],
            AN_IGNORE_RULE_OF_THEIRS[2],
            A_LOCAL_CONFIG,
            Step::File(Place::Repo(".jigc/config/other.yaml"), "mine: 2\n"),
        ],
        acts: &[Act::Run(6)],
        holds: torn_down![],
        ..UNINSTALL
    },
];

/// **The invocation log, with its knob still on** — the route opens with an order: switch
/// it off, stage the config change that makes, and then the paths. The log is one git
/// ignores and the route prints no path of it, so the order is followed by its two printed
/// lines and ended by the consent.
const THE_LOG_WITH_ITS_KNOB_ON: [Case; 1] = [Case {
    name: "uninstall-invocation-log-switched-off-as-printed-and-forced",
    site: UNTRACKED_WORKBENCH,
    cause: "the opt-in invocation log is listed, and the knob that writes it is still on",
    state: installed![
        Step::Jigc(&["config", "set", "invocation-log", "true"]),
        Step::Git(&["add", ".jigc/config"]),
        Step::Git(&["commit", "-q", "-m", "the log on"]),
        Step::Jigc(&["validate"]),
    ],
    acts: &[Act::Run(0), Act::Run(1), Act::Run(5)],
    holds: torn_down![],
    ..UNINSTALL
}];

// ---------------------------------------------------------------------------
// `unverified_workbench_finding` · `uninstall.untracked-workbench-file` — could not look
// ---------------------------------------------------------------------------

/// **The fourth guard could not look, followed the two ways its route offers**: the
/// `.jigc/` the route names made readable, and the consent.
macro_rules! unasked_ways {
    ($prefix:literal, $cause:expr, plant: [$($plant:expr),* $(,)?] $(,)?) => {
        [
            Case {
                name: concat!($prefix, "-made-readable"),
                site: UNVERIFIED_WORKBENCH,
                cause: $cause,
                state: installed![$($plant,)*],
                acts: &[Act::MakeReadable(Operand::Span(1)), Act::Run(2)],
                holds: torn_down![],
                ..UNINSTALL
            },
            Case {
                name: concat!($prefix, "-forced"),
                site: UNVERIFIED_WORKBENCH,
                cause: $cause,
                state: installed![$($plant,)*],
                acts: &[Act::Run(3)],
                holds: torn_down![],
                ..UNINSTALL
            },
        ]
    };
}

const THE_WORKBENCH_UNLISTABLE: [Case; 2] = unasked_ways!(
    "uninstall-workbench-unlistable",
    "`.jigc` may be searched and not listed",
    plant: [shut!("300", ".jigc")],
);
const GITS_INDEX_UNREADABLE: [Case; 2] = unasked_ways!(
    "uninstall-git-index-unreadable",
    "`git ls-files` fails — here over an index that is not one",
    plant: [Step::File(Place::Repo(".git/index"), "no index\n")],
);
/// An index entry whose name is not UTF-8 exists on this platform only through git's
/// plumbing: the name is written by `git update-index --cacheinfo`.
const GITS_LISTING_UNDECODABLE: [Case; 2] = unasked_ways!(
    "uninstall-git-listing-undecodable",
    "git lists a path under `.jigc/` whose name is not UTF-8",
    plant: [Step::Git(&[
        "-c",
        "alias.plant=!git update-index --add --cacheinfo \
         \"100644,$(git hash-object -w README.md),.jigc/config/$(printf '\\377').yaml\"",
        "plant",
    ])],
);

// ---------------------------------------------------------------------------
// `uninstall` — the seven steps that remove, each a site
// ---------------------------------------------------------------------------

const REMOVE_JIGC: &str = "crates/cli/src/setup.rs::uninstall::uninstall.remove-jigc#1";
const UNWIRE: &str = "crates/cli/src/setup.rs::uninstall::uninstall.unwire-reference#1";
const REMOVE_ALLOWLIST: &str =
    "crates/cli/src/setup.rs::uninstall::uninstall.remove-allowlist#1";
const REMOVE_HOOK: &str = "crates/cli/src/setup.rs::uninstall::uninstall.remove-hook#1";
const REMOVE_DENY: &str = "crates/cli/src/setup.rs::uninstall::uninstall.remove-deny#1";
const REMOVE_PRECOMMIT: &str =
    "crates/cli/src/setup.rs::uninstall::uninstall.remove-precommit#1";
const REMOVE_GUIDE: &str = "crates/cli/src/setup.rs::uninstall::uninstall.remove-guide#1";

/// **A step of the teardown that refuses, followed the one way its route offers**: the
/// entry the route names made writable, and the command again. Each step words every
/// error of its callee under that one route. **The first step removes `.jigc/`**, so
/// every refusal behind it finds the workbench gone: the witnesses are their files alone.
macro_rules! step_refused {
    ($name:literal, $site:expr, $cause:expr, plant: [$($plant:expr),* $(,)?] $(,)?) => {
        [Case {
            name: $name,
            site: $site,
            cause: $cause,
            state: installed![$($plant,)*],
            acts: &[Act::MakeWritable(Operand::Span(0)), Act::Run(1)],
            holds: torn_down![],
            ..UNINSTALL
        }]
    };
}

/// **The bit, and not the link** — which of the two the refusal is over in the two cases
/// whose state is a link to a file outside the repository that may not be written.
const THE_BIT_NOT_THE_LINK: Option<(Over, &str)> = Some((
    Over::TheBound,
    "with the file outside the owner's to write, the teardown writes through the link at \
     exit 0 and refuses nothing — driven by hand at both paths (the findings file, *Seen, \
     and no case*)",
));

/// Run a shell line at the repository's root — a `git` alias, where a state has no step
/// for what it needs: an entry swapped for another kind, bytes that are no text.
macro_rules! sh {
    ($line:literal) => {
        Step::Git(&["-c", concat!("alias.plant=!", $line), "plant"])
    };
}

// --- `uninstall.remove-jigc`

const WORKBENCH_READ_ONLY: [Case; 1] = step_refused!(
    "uninstall-workbench-read-only",
    REMOVE_JIGC,
    "`.jigc` itself may not be written, so nothing can be taken out of it",
    plant: [shut!("555", ".jigc")],
);
const A_WORKBENCH_DIRECTORY_READ_ONLY: [Case; 1] = step_refused!(
    "uninstall-workbench-directory-read-only",
    REMOVE_JIGC,
    "a directory under `.jigc` may not be written — `.jigc/config` — while `.jigc` may",
    plant: [shut!("555", ".jigc/config")],
);
const ROOT_READ_ONLY: [Case; 1] = step_refused!(
    "uninstall-workbench-under-a-read-only-root",
    REMOVE_JIGC,
    "the repository root may not be written, so the emptied `.jigc` cannot be removed \
     from it",
    plant: [shut!("555", ".")],
);
/// The one cause the guards keep from every run but a consenting one: the refusing command
/// is `jigc uninstall --force`, and the route's re-run is `jigc uninstall`.
const FORCED_OVER_AN_UNREADABLE_DIRECTORY: [Case; 1] = [Case {
    name: "uninstall-forced-workbench-directory-unreadable-made-writable",
    site: REMOVE_JIGC,
    cause: "a directory under `.jigc` may not be read, and the teardown was told \
            `--force` — the guards refuse it from every other run",
    state: installed![
        Step::Dir(Place::Repo(".jigc/worktrees")),
        shut!("000", ".jigc/worktrees"),
    ],
    refusing: &["uninstall", "--force"],
    acts: &[Act::MakeWritable(Operand::Span(0)), Act::Run(1)],
    holds: torn_down![],
    ..UNINSTALL
}];

// --- `uninstall.unwire-reference`

const HOST_READ_ONLY: [Case; 1] = step_refused!(
    "uninstall-reference-host-read-only",
    UNWIRE,
    "`CLAUDE.md` holds the reference and may not be written",
    plant: [shut!("444", "CLAUDE.md")],
);
const HOST_A_DIRECTORY: [Case; 1] = step_refused!(
    "uninstall-reference-host-a-directory",
    UNWIRE,
    "a directory stands at `CLAUDE.md`",
    plant: [sh!("rm CLAUDE.md && mkdir CLAUDE.md")],
);
const HOST_NOT_TEXT: [Case; 1] = step_refused!(
    "uninstall-reference-host-not-utf-8",
    UNWIRE,
    "`CLAUDE.md` is not UTF-8",
    plant: [sh!("printf '\\377\\376' > CLAUDE.md")],
);
const HOST_UNREADABLE: [Case; 1] = step_refused!(
    "uninstall-reference-host-unreadable",
    UNWIRE,
    "`CLAUDE.md` may not be read",
    plant: [shut!("000", "CLAUDE.md")],
);
/// **Held.** The step reads `CLAUDE.md` with no limit, behind the removal of `.jigc/`.
const HOST_A_FIFO: [Case; 1] = step_refused!(
    "uninstall-reference-host-a-special-file",
    UNWIRE,
    "a special file stands at `CLAUDE.md`",
    plant: [sh!("rm CLAUDE.md"), Step::Fifo(Place::Repo(CLAUDE_MD))],
);
/// The reference's host is a link to a file of somebody's outside the repository — a copy
/// of the host as the install left it — that may not be written. The file is a witness.
const HOST_A_LINK_OUT: [Case; 1] = [Case {
    name: "uninstall-reference-host-a-link-to-a-read-only-file-outside",
    site: UNWIRE,
    cause: "`CLAUDE.md` is a link to a file outside the repository that holds the \
            reference and may not be written",
    state: installed![sh!(
        "cp CLAUDE.md ../outside/CLAUDE.md && rm CLAUDE.md && ln -s \
         \"$PWD/../outside/CLAUDE.md\" CLAUDE.md && chmod 444 ../outside/CLAUDE.md"
    )],
    acts: &[Act::MakeWritable(Operand::Span(0)), Act::Run(1)],
    holds: torn_down![],
    witnesses: &["theirs", "CLAUDE.md"],
    over: THE_BIT_NOT_THE_LINK,
    ..UNINSTALL
}];
// --- `uninstall.remove-allowlist`

const SETTINGS_READ_ONLY: [Case; 1] = step_refused!(
    "uninstall-settings-read-only",
    REMOVE_ALLOWLIST,
    "the settings file holds the permits and may not be written",
    plant: [shut!("444", ".claude/settings.json")],
);
const SETTINGS_A_DIRECTORY: [Case; 1] = step_refused!(
    "uninstall-settings-a-directory",
    REMOVE_ALLOWLIST,
    "a directory stands at the settings file's path",
    plant: [sh!("rm .claude/settings.json && mkdir .claude/settings.json")],
);
const SETTINGS_NOT_TEXT: [Case; 1] = step_refused!(
    "uninstall-settings-not-utf-8",
    REMOVE_ALLOWLIST,
    "the settings file is not UTF-8",
    plant: [sh!("printf '\\377\\376' > .claude/settings.json")],
);
const SETTINGS_NOT_JSON: [Case; 1] = step_refused!(
    "uninstall-settings-not-json",
    REMOVE_ALLOWLIST,
    "the settings file is no JSON",
    plant: [Step::File(Place::Repo(SETTINGS_JSON), "{\n")],
);
const SETTINGS_UNREADABLE: [Case; 1] = step_refused!(
    "uninstall-settings-unreadable",
    REMOVE_ALLOWLIST,
    "the settings file may not be read",
    plant: [shut!("000", ".claude/settings.json")],
);
/// **Held.** The step reads the settings file with no limit, behind the removal of
/// `.jigc/` and the unwiring of the reference.
const SETTINGS_A_FIFO: [Case; 1] = step_refused!(
    "uninstall-settings-a-special-file",
    REMOVE_ALLOWLIST,
    "a special file stands at the settings file's path",
    plant: [
        sh!("rm .claude/settings.json"),
        Step::Fifo(Place::Repo(SETTINGS_JSON)),
    ],
);

// --- `uninstall.remove-hook` and `uninstall.remove-deny`

/// **Their settings, where they held both of jigc's permits before `jigc setup`**: the
/// install's record then lists no permit, the allowlist step has nothing to take and writes
/// nothing, and the session hook's step is the first to write the settings file.
const THEIR_PERMITS: &str = "{\n  \"permissions\": {\n    \"allow\": [\n      \"Bash(mine:*)\",\n      \
                             \"Bash(jigc:*)\",\n      \"Bash(git add:*)\"\n    ]\n  }\n}\n";
/// **And where they held the session hook as well**: the record lists neither permit nor
/// hook, and the deny floor's step is the first to write.
const THEIR_PERMITS_AND_HOOK: &str = "{\n  \"hooks\": {\n    \"SessionStart\": [\n      {\n        \
                                      \"hooks\": [\n          {\n            \"command\": \
                                      \"jigc start\",\n            \"type\": \"command\"\n          \
                                      }\n        ]\n      }\n    ]\n  },\n  \"permissions\": {\n    \
                                      \"allow\": [\n      \"Bash(mine:*)\",\n      \
                                      \"Bash(jigc:*)\",\n      \"Bash(git add:*)\"\n    ]\n  }\n}\n";

const HOOK_FIRST_TO_WRITE: [Case; 1] = [Case {
    name: "uninstall-session-hook-settings-read-only",
    site: REMOVE_HOOK,
    cause: "the settings file may not be written, and the install's record lists no \
            permit: the session hook's removal is the first write",
    state: installed_over![THEIR_PERMITS, shut!("444", ".claude/settings.json")],
    acts: &[Act::MakeWritable(Operand::Span(0)), Act::Run(1)],
    holds: &[
        Check::Absent(Place::Repo(WORKBENCH)),
        Check::Lacks(Place::Repo(SETTINGS_JSON), "jigc start"),
        Check::Lacks(Place::Repo(SETTINGS_JSON), "jigc uninstall"),
        Check::Contains(Place::Repo(SETTINGS_JSON), "\"Bash(jigc:*)\""),
        Check::Contains(Place::Repo(SETTINGS_JSON), "\"Bash(mine:*)\""),
    ],
    ..UNINSTALL
}];
const DENY_FIRST_TO_WRITE: [Case; 1] = [Case {
    name: "uninstall-deny-floor-settings-read-only",
    site: REMOVE_DENY,
    cause: "the settings file may not be written, and the install's record lists neither \
            permit nor hook: the deny floor's removal is the first write",
    state: installed_over![
        THEIR_PERMITS_AND_HOOK,
        shut!("444", ".claude/settings.json")
    ],
    acts: &[Act::MakeWritable(Operand::Span(0)), Act::Run(1)],
    holds: &[
        Check::Absent(Place::Repo(WORKBENCH)),
        Check::Lacks(Place::Repo(SETTINGS_JSON), "jigc uninstall"),
        Check::Contains(Place::Repo(SETTINGS_JSON), "jigc start"),
        Check::Contains(Place::Repo(SETTINGS_JSON), "\"Bash(mine:*)\""),
    ],
    ..UNINSTALL
}];

// --- `uninstall.remove-precommit`

/// **The hook's removal refused, followed the one way its route offers.** The route names
/// *the repo's git hooks directory* and prints no path, so the directory is the case's to
/// name: the one git resolves.
macro_rules! hook_refused {
    ($name:literal, $cause:expr, $hooks:literal, plant: [$($plant:expr),* $(,)?] $(,)?) => {
        [Case {
            name: $name,
            site: REMOVE_PRECOMMIT,
            cause: $cause,
            state: installed![$($plant,)*],
            acts: &[Act::MakeWritable(Operand::Unprinted($hooks)), Act::Run(0)],
            holds: torn_down![],
            ..UNINSTALL
        }]
    };
}

const HOOKS_PATH_EMPTY: [Case; 1] = hook_refused!(
    "uninstall-hook-hooks-path-empty",
    "`core.hooksPath` is the empty string, which git refuses to resolve",
    ".git/hooks",
    plant: [Step::Git(&["config", "core.hooksPath", ""])],
);
const HOOKS_PATH_A_FILE: [Case; 1] = hook_refused!(
    "uninstall-hook-hooks-path-a-file",
    "`core.hooksPath` names a regular file",
    "../outside/hooks-file",
    plant: [
        Step::File(Place::Outside("hooks-file"), "theirs\n"),
        Step::Git(&["config", "core.hooksPath", "../outside/hooks-file"]),
    ],
);
const HOOKS_PATH_A_DEVICE: [Case; 1] = hook_refused!(
    "uninstall-hook-hooks-path-dev-null",
    "`core.hooksPath` names `/dev/null`",
    "/dev/null",
    plant: [Step::Git(&["config", "core.hooksPath", "/dev/null"])],
);
const HOOK_A_DIRECTORY: [Case; 1] = hook_refused!(
    "uninstall-hook-a-directory",
    "a directory stands at the hook's path",
    ".git/hooks",
    plant: [sh!("rm .git/hooks/pre-commit && mkdir .git/hooks/pre-commit")],
);
const HOOK_NOT_TEXT: [Case; 1] = hook_refused!(
    "uninstall-hook-not-utf-8",
    "the hook is not UTF-8",
    ".git/hooks",
    plant: [sh!("printf '\\377\\376' > .git/hooks/pre-commit")],
);
const HOOK_UNREADABLE: [Case; 1] = hook_refused!(
    "uninstall-hook-unreadable",
    "the hook may not be read",
    ".git/hooks",
    plant: [shut!("000", ".git/hooks/pre-commit")],
);
/// **Held.** The step reads the hook with no limit, behind every other removal but the
/// guide's.
const HOOK_A_FIFO: [Case; 1] = hook_refused!(
    "uninstall-hook-a-special-file",
    "a special file stands at the hook's path",
    ".git/hooks",
    plant: [sh!("rm .git/hooks/pre-commit"), Step::Fifo(Place::Repo(HOOK))],
);
const HOOK_READ_ONLY: [Case; 1] = hook_refused!(
    "uninstall-hook-read-only",
    "the hook holds their lines beside jigc's block and may not be written, in a hooks \
     directory that may",
    ".git/hooks",
    plant: [shut!("444", ".git/hooks/pre-commit")],
);
/// The hook is a link to a script of somebody's outside the repository — a copy of the hook
/// as the install left it — that may not be written. The script is a witness.
const HOOK_A_LINK_OUT: [Case; 1] = [Case {
    name: "uninstall-hook-a-link-to-a-read-only-script-outside",
    site: REMOVE_PRECOMMIT,
    cause: "the hook is a link to a script outside the repository that holds jigc's block \
            and may not be written",
    state: installed![sh!(
        "cp .git/hooks/pre-commit ../outside/pre-commit && rm .git/hooks/pre-commit && ln -s \
         \"$PWD/../outside/pre-commit\" .git/hooks/pre-commit && chmod 444 \
         ../outside/pre-commit"
    )],
    acts: &[Act::MakeWritable(Operand::Unprinted(".git/hooks")), Act::Run(0)],
    holds: torn_down![],
    witnesses: &["theirs", "pre-commit"],
    over: THE_BIT_NOT_THE_LINK,
    ..UNINSTALL
}];
/// **The one state the route's words are true of**: an install over a repository with no
/// hook of its own, so the hook is jigc's whole and the step removes the file — from a
/// hooks directory that may not be written.
const A_HOOK_OF_JIGCS_IN_A_READ_ONLY_HOOKS_DIRECTORY: [Case; 1] = [Case {
    name: "uninstall-hook-of-jigcs-own-hooks-directory-read-only",
    site: REMOVE_PRECOMMIT,
    cause: "the hook is jigc's whole, so the step removes the file, and the hooks \
            directory may not be written",
    state: &[Step::Jigc(&["setup"]), shut!("555", ".git/hooks")],
    acts: &[Act::MakeWritable(Operand::Unprinted(".git/hooks")), Act::Run(0)],
    holds: &[
        Check::Absent(Place::Repo(WORKBENCH)),
        Check::Absent(Place::Repo(HOOK)),
        Check::Absent(Place::Repo(GUIDE_MD)),
    ],
    witnesses: &[],
    untouched: &["README.md"],
    ..UNINSTALL
}];

// --- `uninstall.remove-guide`

const GUIDE_IN_A_READ_ONLY_DIRECTORY: [Case; 1] = step_refused!(
    "uninstall-guide-directory-read-only",
    REMOVE_GUIDE,
    "the guide is jigc's, and the directory that holds it may not be written",
    plant: [shut!("555", ".claude/skills/jigc")],
);

/// The cases of `jigc uninstall`, a group a cause — and, within a group, a case a way out
/// of the cause's route.
pub const CASES: &[&[Case]] = &[
    &OUTSIDE_A_REPOSITORY,
    &BESIDE_A_GIT_LINK_TO_NOTHING,
    &HOME_NOT_SET,
    &NO_PROFILE_IN_THE_DIRECTORY,
    &PROFILE_NOT_TEXT,
    &PROFILE_MALFORMED,
    &A_DIRECTORY_NOTHING_VOUCHES_FOR,
    &A_FILE_WHERE_A_WORKTREE_GOES,
    &A_PATH_NOTHING_CAN_BE_READ_AT,
    &A_LIVE_FAN_OUT_WORKTREE,
    &A_REGISTRATION_WITH_NO_DIRECTORY,
    &A_COMMIT_NO_REF_REACHES,
    &AN_OPERATION_LEFT_OPEN,
    &A_REGISTERED_AND_AN_UNREGISTERED_PATH,
    &THE_WORKTREES_ROOT_UNREADABLE,
    &A_STRAY_IN_A_TASKS_AREA,
    &A_STRAY_IN_A_MILESTONES_AREA,
    &A_STRAY_BESIDE_THE_AREAS,
    &A_STRAY_IN_A_TRANSIENT_DIRECTORY,
    &A_PARKED_FILE,
    &A_DIRECTORY_OF_THE_WALK_UNLISTABLE,
    &THE_WORKBENCH_UNSEARCHABLE,
    &AN_OPEN_TASKS_STAGED_DOC,
    &A_STAGED_ENTRY_THAT_CANNOT_BE_ASKED_ABOUT,
    &A_FILE_NO_INDEX_HAS,
    &A_TRACKED_FILE_EDITED,
    &AN_IGNORED_FILE,
    &AN_IGNORED_FILE_AND_ONE_GIT_TAKES,
    &THE_LOG_WITH_ITS_KNOB_ON,
    &THE_WORKBENCH_UNLISTABLE,
    &GITS_INDEX_UNREADABLE,
    &GITS_LISTING_UNDECODABLE,
    &WORKBENCH_READ_ONLY,
    &A_WORKBENCH_DIRECTORY_READ_ONLY,
    &ROOT_READ_ONLY,
    &FORCED_OVER_AN_UNREADABLE_DIRECTORY,
    &HOST_READ_ONLY,
    &HOST_A_DIRECTORY,
    &HOST_NOT_TEXT,
    &HOST_UNREADABLE,
    &HOST_A_FIFO,
    &HOST_A_LINK_OUT,
    &SETTINGS_READ_ONLY,
    &SETTINGS_A_DIRECTORY,
    &SETTINGS_NOT_TEXT,
    &SETTINGS_NOT_JSON,
    &SETTINGS_UNREADABLE,
    &SETTINGS_A_FIFO,
    &HOOK_FIRST_TO_WRITE,
    &DENY_FIRST_TO_WRITE,
    &HOOKS_PATH_EMPTY,
    &HOOKS_PATH_A_FILE,
    &HOOKS_PATH_A_DEVICE,
    &HOOK_A_DIRECTORY,
    &HOOK_NOT_TEXT,
    &HOOK_UNREADABLE,
    &HOOK_A_FIFO,
    &HOOK_READ_ONLY,
    &HOOK_A_LINK_OUT,
    &A_HOOK_OF_JIGCS_IN_A_READ_ONLY_HOOKS_DIRECTORY,
    &GUIDE_IN_A_READ_ONLY_DIRECTORY,
];

/// The sites of `jigc uninstall` that have no case, each with why: none — every blocking
/// site of the command is reached by a state the runner can plant.
pub const REASONS: &[Reason] = &[];

/// Why the consent is the one way a case follows where a route defers to the refusal's
/// listing.
const ON_ITS_LINE: Why = Why::NotConstructible(
    "the command is printed on the path's line of the refusal's listing, above the route, \
     and an act's operand is a span of the printed route: the frame runs no line of the \
     message",
);

/// Why a path the refusal lists cannot be named where the route prints another.
const PRINTS_ANOTHER_PATH: Why = Why::NotConstructible(
    "the route says *the paths above* of the listed file and prints the path of the \
     parking home, `.jigc/displaced/`. The frame refuses the marked form wherever the \
     printed route holds a path, so a case may not name the listed file there, and no act \
     on it can be written",
);

/// Why no step's failed write of bytes has a case.
const BYTES_CANNOT_BE_WRITTEN: (&str, Why) = (
    "the file is opened and its bytes cannot be written",
    Why::NotConstructible(
        "a write that fails on an open file is a full disk or a failing one: no state of a \
         sandbox",
    ),
);

/// Why a settings file that cannot be read is no cause of the two steps behind the first.
const THE_STEP_BEFORE_READS: Why = Why::NotConstructible(
    "the allowlist step reads the same file first, through the same reader, and refuses \
     under `uninstall.remove-allowlist` — the cases `uninstall-settings-…`. Nothing writes \
     the file between the steps but a step that has just read it",
);

/// **What was read for each site, and how many causes it holds** — a hand count, with its
/// derivation; the causes that have no case, each with why; and the ways out of its route
/// that are not followed, each with why.
pub const READINGS: &[Reading] = &[
    Reading {
        site: NOT_IN_A_REPOSITORY,
        read: &[
            "crates/cli/src/locate.rs::locate_finding and not_in_repo_route",
            "crates/cli/src/locate.rs::locate and locate_from — the two ways `locate` fails",
            "crates/cli/src/repo.rs::discover_repo_root — the walk up, and what it asks of a `.git`",
            "crates/cli/src/setup.rs::run_uninstall — the code and the second route this door hands over",
        ],
        causes: 3,
        derivation: "the function is the one `jigc setup` answers through, handed this \
                     door's code: it answers here where `discover_repo_root` finds nothing, \
                     and that walk asks one thing of each directory on the way up — whether \
                     `.git` exists there, following a link. Three ways to nothing: no entry \
                     of that name anywhere above; an entry that is a link leading to \
                     nothing; an entry that cannot be asked about",
        without_case: &[(
            "a directory above holds a `.git` that cannot be asked about",
            Why::NotConstructible(
                "the walk reads a failed `stat` as absent, so the cause is a directory on \
                 the way up that may not be searched — and a command cannot be started in a \
                 directory under one its caller may not search: the frame has no state for \
                 it, and a root caller, who may search anything, cannot reach it at all",
            ),
        )],
        not_followed: &[],
    },
    Reading {
        site: NO_HOME_SITE,
        read: &[
            "crates/cli/src/locate.rs::locate_finding",
            "crates/cli/src/locate.rs::locate — `$HOME` is read for the team layer's path",
            "crates/cli/src/setup.rs::run_uninstall — the route this arm is handed",
        ],
        causes: 1,
        derivation: "`locate` fails two ways, and the other is the site above, which is asked \
                     first: so this arm is `$HOME` not set, inside a repository",
        without_case: &[],
        not_followed: &[],
    },
    Reading {
        site: PROFILE_LOAD,
        read: &[
            "crates/cli/src/setup.rs::run_uninstall",
            "crates/cli/src/adapter.rs::load_profile and load_profile_from",
            "crates/cli/src/adapter.rs::ProfileError — the four ways a load fails",
        ],
        causes: 4,
        derivation: "`load_profile_from` fails four ways: no profile of that name is embedded; \
                     the file `JIGC_ADAPTERS_DIR` selects cannot be read; its bytes are not \
                     UTF-8; its YAML is no profile. The second is one arm whatever the read's \
                     error was, and is followed where the directory holds no such file. A \
                     profile that loads is asked nothing more by this door — the teardown \
                     has no clause refusals of its own",
        without_case: &[(
            "no profile of the assistant's name is embedded in the binary",
            Why::NotConstructible(
                "the name is a constant of `setup.rs` and the binary embeds the profile of \
                 that name: no state of a repository or of the environment takes it out",
            ),
        )],
        not_followed: &[],
    },
    Reading {
        site: DIRTY_WORKTREE,
        read: &[
            "crates/cli/src/setup.rs::dirty_worktree_finding — one route, grown by what the held paths are: registered here, registered nowhere, no worktree at all, mid-operation, anchored",
            "crates/cli/src/setup.rs::uninstall — the order of the four guards",
            "crates/cli/src/setup.rs::dirty_fanout_worktrees and fanout_worktree_paths — which paths are asked about, and whether this repository registered each",
            "crates/cli/src/milestone.rs::probe_leftover — what a path can be held for: the three legs",
            "crates/cli/src/milestone.rs::is_jigc_made_registration and admin_records — the registrations with no directory left",
        ],
        causes: 9,
        derivation: "`probe_leftover` holds a path for what stands there — a directory \
                     nothing vouches for, a file, a path nothing can be read at, a live \
                     worktree with work in no commit — or for what git's registration of it \
                     holds: staged paths with no directory left, a commit no ref reaches, an \
                     operation left un-concluded. Seven kinds of hold. The route then turns \
                     on whether this repository registered the held paths: all of them, none \
                     of them, some of them — the eighth cause is the two kinds at once — or \
                     git could not say, the ninth",
        without_case: &[(
            "git cannot list this repository's worktrees, so whether a held path is \
             registered is unknown",
            Why::NotConstructible(
                "reached — driven by hand, with a `.git/config` git cannot read — only \
                 where git itself fails in the main checkout, and there neither way out of \
                 the route can be followed to an end that is about this route: the re-run's \
                 fourth guard and the consent's hook step both ask git, and each refuses \
                 under a code of its own",
            ),
        )],
        not_followed: &[
            (
                "the work committed or stashed inside a live worktree",
                Why::NotConstructible(
                    "the route says *commit, stash, or copy it* and prints no line for any \
                     of the three; the frame's commit is one in the sandbox's own \
                     repository, where `.jigc/worktrees/` is ignored. The work is moved out \
                     — *copy it*",
                ),
            ),
            (
                "the command that keeps a commit, brings a checkout back, or abandons an \
                 operation",
                ON_ITS_LINE,
            ),
            (
                "the milestone abandoned, at the three causes that are about a registration",
                Why::NotConstructible(
                    "one arm, one line — `jigc milestone discard <milestone-id> --force` — \
                     whatever the registered path is held for: followed at the live \
                     worktree, where the refusal lists no milestone to fill it with and no \
                     shell splits it as printed",
                ),
            ),
            (
                "the work moved out of a path whose directory is gone, or whose checkout \
                 holds nothing in no commit",
                Why::NotConstructible(
                    "the route says of those paths that there is *nothing to move out*",
                ),
            ),
        ],
    },
    Reading {
        site: UNVERIFIED_WORKTREES,
        read: &[
            "crates/cli/src/setup.rs::unverified_worktrees_finding",
            "crates/cli/src/setup.rs::fanout_worktree_paths — the one `read_dir` whose failure has no path to name",
            "crates/cli/src/setup.rs::dirty_fanout_worktrees — a path that cannot be read is a hold, and another site's",
        ],
        causes: 2,
        derivation: "`fanout_worktree_paths` fails where `.jigc/worktrees/` is a directory \
                     and its listing does — at the `read_dir`, or at one of its entries",
        without_case: &[(
            "the listing of `.jigc/worktrees/` begins and one of its entries cannot be read",
            Why::NotConstructible(
                "a directory that can be opened for listing yields its entries: an entry \
                 that fails in the middle of a listing is a filesystem's fault and no state \
                 of a sandbox",
            ),
        )],
        not_followed: &[],
    },
    Reading {
        site: FOREIGN_BYTES,
        read: &[
            "crates/cli/src/setup.rs::foreign_bytes_finding — one route, whatever is listed",
            "crates/cli/src/setup.rs::workbench_foreign_subject, workbench_foreign_areas and prefix_owner — which directories are asked about, a row a transient prefix",
            "crates/cli/src/setup.rs::own_transient_file and own_transient_paths — what is jigc's own in the three directories no working area lives in",
            "crates/cli/src/task.rs::foreign_areas, leaves_under and stray_children — the walk of each row",
            "crates/engine/src/state.rs::foreign_area_paths — a working area's complement",
        ],
        causes: 5,
        derivation: "`workbench_foreign_areas` hands over one row a kind of area, and each \
                     has its own complement: a task's working area; a milestone's; the \
                     parking home, every leaf; `.jigc/tasks/` or `.jigc/milestones/` \
                     itself, every child that is no directory; and a transient directory no \
                     working area lives in, every leaf that is none of jigc's own. Five. \
                     The route is one for all of them, and several at once is a longer \
                     listing under the same words",
        without_case: &[],
        not_followed: &[
            (
                "what is needed moved out of a listed path, or a listed path deleted, at \
                 the four causes whose path is not the parking home",
                PRINTS_ANOTHER_PATH,
            ),
            (
                "the parking home deleted — *`rm -r` takes them*",
                Why::NotConstructible(
                    "`rm -r` is printed with no path, and it is no line of `jigc` or of \
                     `git`: the frame runs those two, and removes a file or a link, never \
                     a directory. The parking home is moved out whole instead",
                ),
            ),
        ],
    },
    Reading {
        site: UNVERIFIED_FOREIGN,
        read: &[
            "crates/cli/src/setup.rs::unverified_foreign_finding",
            "crates/cli/src/setup.rs::workbench_foreign_areas — the `stat` of each transient prefix, and the listing of the two that hold working areas",
            "crates/cli/src/task.rs::foreign_areas and leaves_under; crates/engine/src/state.rs::foreign_area_paths — every listing of the walk",
        ],
        causes: 2,
        derivation: "the walk fails two ways: a transient prefix cannot be asked about at \
                     all — `.jigc` itself may not be searched — or a directory the walk \
                     lists cannot be listed. The second is one arm, an `io::Error` handed \
                     up, whichever directory it was — a work-unit root, a working area, the \
                     parking home, a transient directory — and is followed at the first the \
                     walk reaches",
        without_case: &[],
        not_followed: &[],
    },
    Reading {
        site: STAGED_PROSE,
        read: &[
            "crates/cli/src/setup.rs::staged_prose_finding — the route, and the note it gains where a listed task is a milestone's",
            "crates/cli/src/task.rs::staged_task_prose and staged_doc_ids — which docs are staged",
            "crates/cli/src/setup.rs::uninstall — the guard before this one, and what it has refused first",
        ],
        causes: 2,
        derivation: "the finding lists every open task that stages a doc, and its route \
                     has two wordings: with no listed task a milestone's sub-task, and with \
                     one — where the route gains a note and the sub-task's row its own two \
                     exits",
        without_case: &[(
            "a milestone's sub-task stages a doc",
            Why::NotConstructible(
                "a sub-task's doc is staged by a `jigc doc` write run inside the \
                 sub-task's own worktree — the task is pinned there — and a state's step \
                 runs the binary under test at the repository's root: no state the runner \
                 can plant holds one. Not reached by hand either: `jigc start --task` in \
                 the worktree stages none",
            ),
        )],
        not_followed: &[],
    },
    Reading {
        site: UNVERIFIED_PROSE,
        read: &[
            "crates/cli/src/task.rs::unverified_prose_finding",
            "crates/cli/src/task.rs::staged_task_prose and staged_doc_ids — the three places the walk can fail",
            "crates/engine/src/state.rs::foreign_area_paths — the guard before this one lists the same directories and asks each entry's kind of the listing itself",
        ],
        causes: 3,
        derivation: "`staged_task_prose` fails where `.jigc/tasks/` cannot be listed, where \
                     a task's `docs/` cannot be listed, and where what a staged entry is \
                     cannot be asked — a `stat` through `docs/`, which the guard before \
                     does not make",
        without_case: &[
            (
                "`.jigc/tasks/` cannot be listed",
                Why::NotConstructible(
                    "the foreign-byte guard lists the same directory first and refuses \
                     under `uninstall.foreign-bytes` — the cases \
                     `uninstall-task-areas-unlistable-…`",
                ),
            ),
            (
                "an open task's `docs/` cannot be listed",
                Why::NotConstructible(
                    "the foreign-byte guard lists every working area's `docs/` first and \
                     refuses under `uninstall.foreign-bytes` — driven by hand",
                ),
            ),
        ],
        not_followed: &[],
    },
    Reading {
        site: UNTRACKED_WORKBENCH,
        read: &[
            "crates/cli/src/setup.rs::untracked_workbench_finding — four routes, by which listed paths git ignores, and the order it puts in front where the log's knob is on",
            "crates/cli/src/setup.rs::untracked_workbench_files, classify_workbench_paths and workbench_paths — which files are asked about, and the two legs of *no index has a copy*",
            "crates/cli/src/setup.rs::ignored_among — what is asked of git about each listed path",
            "crates/cli/src/setup.rs::own_transient_leaves — the invocation log, the one member inside a transient directory",
        ],
        causes: 6,
        derivation: "`classify_workbench_paths` lists a path on either of two legs — the \
                     index does not list it, or lists it with other bytes — and the route \
                     then turns on `ignored_among`: none of the listed paths ignored, all of \
                     them, some of them, or git could not say. With the invocation log \
                     listed and its knob on, an order is put in front. Two legs under the \
                     first route, then the three other answers, then the order: six",
        without_case: &[(
            "git could not say which of the listed paths it ignores",
            Why::NotConstructible(
                "`git check-ignore` is asked a moment after `git ls-files` and `git status` \
                 answered in the same repository: no state was found that fails the third \
                 question and neither of the two before it",
            ),
        )],
        not_followed: &[
            (
                "an ignored file moved out of `.jigc/`, or deleted, where every listed path \
                 is ignored — the log among them",
                Why::NotConstructible(
                    "the route says *move what you need out of `.jigc/`* and prints the \
                     workbench's path and not the file's. The frame refuses the marked form \
                     wherever the printed route holds a path, so a case may not name the \
                     file there — which is also why the order over the log is ended by the \
                     consent and not by the move",
                ),
            ),
            (
                "the file restored after the teardown — `git checkout -- <path>`",
                Why::NotConstructible(
                    "the line is printed to say what the index's copy is good for, behind \
                     the teardown: no step of the way out",
                ),
            ),
        ],
    },
    Reading {
        site: UNVERIFIED_WORKBENCH,
        read: &[
            "crates/cli/src/setup.rs::unverified_workbench_finding",
            "crates/cli/src/setup.rs::workbench_paths — the listings of the walk",
            "crates/cli/src/setup.rs::classify_workbench_paths and git_capture_untrimmed; crates/cli/src/task.rs::git_capture — the two questions asked of git, and the ways each has no answer",
        ],
        causes: 5,
        derivation: "the probe fails where a directory of the walk cannot be listed; where \
                     `git` cannot be run; where `git ls-files` fails; where `git status` \
                     fails; and where what git printed is not UTF-8. The first is one arm \
                     whichever directory it was, followed at `.jigc` itself",
        without_case: &[
            (
                "`git` cannot be run",
                Why::NotConstructible(
                    "reached with no `git` on `PATH`; the route's way out is to put it \
                     there, and a case's environment is one for all its commands — the \
                     frame has no act that changes a variable between the refusing command \
                     and the route's, and its own `git` children would not start",
                ),
            ),
            (
                "`git status` fails where `git ls-files` answered",
                Why::NotConstructible(
                    "both read the same index: no state was found that fails the second \
                     and not the first",
                ),
            ),
        ],
        not_followed: &[],
    },
    Reading {
        site: REMOVE_JIGC,
        read: &[
            "crates/cli/src/setup.rs::uninstall — step 1, and the guards that decide what state reaches it",
            "std::fs::remove_dir_all — every error of it is worded under one route",
        ],
        causes: 4,
        derivation: "the removal of `.jigc/` fails where an entry cannot be taken out of \
                     its directory: out of `.jigc` itself, out of a directory under it, or \
                     `.jigc` out of the repository's root — three places a write bit can be \
                     missing — and where a directory under it cannot be read at all, which \
                     the guards refuse first from every run but one told `--force`",
        without_case: &[],
        not_followed: &[],
    },
    Reading {
        site: UNWIRE,
        read: &[
            "crates/cli/src/setup.rs::uninstall — step 2, behind the removal of `.jigc/`",
            "crates/cli/src/adapter.rs::unwire_reference — the read, the two shapes of host it takes the reference out of, and the write or the removal",
        ],
        causes: 8,
        derivation: "`unwire_reference` reads `CLAUDE.md` through whatever stands there — a \
                     directory, bytes that are no text, a file that may not be read, a \
                     special file — then writes the host back without the reference, or \
                     removes a host that held nothing else. The write fails over a file \
                     that may not be written, the host's own or the one a link leads to, \
                     and over bytes a disk will not take; the removal over a root that may \
                     not be written",
        without_case: &[
            (
                "the host held the reference and nothing else, and the root it is to be \
                 removed from may not be written",
                Why::NotConstructible(
                    "the step before needs the same permission to take `.jigc/` out of the \
                     root, and refuses first under `uninstall.remove-jigc` — the case \
                     `uninstall-workbench-under-a-read-only-root`",
                ),
            ),
            BYTES_CANNOT_BE_WRITTEN,
        ],
        not_followed: &[],
    },
    Reading {
        site: REMOVE_ALLOWLIST,
        read: &[
            "crates/cli/src/setup.rs::uninstall — steps 0b and 3: which entries are taken, and the first read of the settings file",
            "crates/cli/src/setup.rs::SettingsClaim::read — what the install's record says, read before `.jigc/` goes",
            "crates/cli/src/adapter.rs::remove_allowlist, drop_entries, read_settings and write_settings",
        ],
        causes: 7,
        derivation: "`remove_allowlist` is the first step to read the settings file: a \
                     directory there, bytes that are no text, no JSON, a file that may not \
                     be read, a special file. It writes only where it took a permit out: \
                     over a file that may not be written, and over bytes a disk will not \
                     take. A settings file of another shape than the merge's is no error \
                     here — the step finds no permit and writes nothing",
        without_case: &[BYTES_CANNOT_BE_WRITTEN],
        not_followed: &[(
            "the write refused through a link to a file outside the repository",
            Why::NotConstructible(
                "one arm — the write fails — followed at the file that may not be written; \
                 a link out is followed at the reference's host, the step before",
            ),
        )],
    },
    Reading {
        site: REMOVE_HOOK,
        read: &[
            "crates/cli/src/setup.rs::uninstall — step 4",
            "crates/cli/src/adapter.rs::remove_hook, read_settings and write_settings",
            "crates/cli/src/adapter.rs::remove_allowlist — the step before reads the same file, and writes it only where it took a permit",
        ],
        causes: 2,
        derivation: "the step reads the settings file and writes it back. The read was the \
                     allowlist step's first; the write is this step's first only where that \
                     step took nothing out — an install whose record lists no permit",
        without_case: &[(
            "the settings file cannot be read, or is no JSON",
            THE_STEP_BEFORE_READS,
        )],
        not_followed: &[],
    },
    Reading {
        site: REMOVE_DENY,
        read: &[
            "crates/cli/src/setup.rs::uninstall — step 5",
            "crates/cli/src/adapter.rs::remove_deny, read_settings and write_settings",
            "crates/cli/src/adapter.rs::remove_allowlist and remove_hook — the two steps before read the same file, and each writes it only where it took something",
        ],
        causes: 2,
        derivation: "as the step before: the read was another's first, and the write is \
                     this step's first only where neither of the two before took anything \
                     out — an install whose record lists neither permit nor hook",
        without_case: &[(
            "the settings file cannot be read, or is no JSON",
            THE_STEP_BEFORE_READS,
        )],
        not_followed: &[],
    },
    Reading {
        site: REMOVE_PRECOMMIT,
        read: &[
            "crates/cli/src/setup.rs::uninstall — step 6",
            "crates/cli/src/setup.rs::remove_precommit_hook, resolve_hooks_dir and precommit_hook_in — where the hook is, the read, and the removal or the write",
            "crates/cli/src/setup.rs::classify_precommit_for_teardown — which of the two the step does",
        ],
        causes: 11,
        derivation: "`remove_precommit_hook` asks git for the hooks directory — refused \
                     where `core.hooksPath` is the empty string — and reads the hook \
                     through whatever stands there: a path under a `core.hooksPath` that is \
                     a regular file or a device, a directory, bytes that are no text, a \
                     file that may not be read, a special file. A hook that is jigc's whole \
                     is removed — refused by a hooks directory that may not be written; \
                     one that holds their lines too is written back without jigc's block — \
                     refused by a hook that may not be written, its own file or the one a \
                     link leads to, and by bytes a disk will not take. A hook jigc cannot \
                     find its block in is no error: it is left and named",
        without_case: &[BYTES_CANNOT_BE_WRITTEN],
        not_followed: &[],
    },
    Reading {
        site: REMOVE_GUIDE,
        read: &[
            "crates/cli/src/setup.rs::uninstall — step 7",
            "crates/cli/src/setup.rs::guide_ownership — a guide that is not jigc's is kept and named, not removed",
        ],
        causes: 1,
        derivation: "the step removes one regular file, and a file's removal is its \
                     directory's permission: the one error is a directory that may not be \
                     written. A special file or a directory at the guide's path is no file \
                     to the step, which passes it by",
        without_case: &[],
        not_followed: &[],
    },
];
