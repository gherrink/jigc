//! The route catalog's cases for `jigc uninstall` — the family's table, declared by
//! `crates/cli/tests/route_catalog.rs`, which says what a case is held to.
//!
//! **What stands here.** The refusals a teardown meets before it removes anything: no
//! repository to tear down in, or no `$HOME` (`locate_finding`, two sites); the adapter
//! profile not loaded (reached through `JIGC_ADAPTERS_DIR` and no other way); and three of
//! the four guards over what `.jigc/` holds the only copy of, each with the refusal it
//! raises where it could not look: a path under `.jigc/worktrees/` that holds content, a
//! byte jigc did not write inside a working area or the parking home, and an open task's
//! staged doc. Each of those nine sites has a row of [`READINGS`].
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
//! in the route. Followed as printed, no shell splits such a line; filled in, it would be a
//! line of the case's own, which the frame does not run. Every such case is marked `doubt`
//! with the two readings ([`WHICH_LINE`]).

use super::{Reading, Reason, Why};
use crate::support::route_case::{Act, Case, Check, Operand, Place, Step, Var};

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

// The files the install is merged into or adds, from the repository root.
const CLAUDE_MD: &str = "CLAUDE.md";
const SETTINGS_JSON: &str = ".claude/settings.json";
const HOOK: &str = ".git/hooks/pre-commit";
const GUIDE_MD: &str = ".claude/skills/jigc/SKILL.md";
const WORKBENCH: &str = ".jigc";

/// **Somebody's repository, and then an install that stands**: their instruction file,
/// their settings with a permit of their own, their `pre-commit` hook, a file jigc never
/// writes, and a file outside the repository — committed — and `jigc setup` over it, run by
/// the binary under test. The steps a case adds follow.
macro_rules! installed {
    ($($plant:expr),* $(,)?) => {
        &[
            Step::File(Place::Repo(CLAUDE_MD), "mine\n"),
            Step::File(
                Place::Repo(SETTINGS_JSON),
                "{\n  \"permissions\": {\n    \"allow\": [\n      \"Bash(mine:*)\"\n    ]\n  }\n}\n",
            ),
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

/// The two readings of a printed line that holds a word in angle brackets.
const WHICH_LINE: &str = "a backticked line with a word in angle brackets — \
                          `jigc task discard <task-id> --force`, `jigc milestone discard \
                          <milestone-id> --force`: (a) a command to run as printed, like \
                          every line a route prints between backticks — followed here: no \
                          shell splits it, the bracketed word being a redirection from a \
                          file that is not there; (b) a shape to fill in with the id the \
                          refusal lists above its route — then the route prints no line a \
                          reader can run as it stands, and the frame has no case for it: a \
                          filled-in line would be the case's own";

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
/// third way: the milestone abandoned, by a line that holds `<milestone-id>`.
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
        doubt: Some(WHICH_LINE),
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

/// The two readings of the route's parenthesis.
const WHICH_GIT_LINES: &str = "*or remove the worktrees yourself (`git worktree list`, then \
                               `git worktree remove`) and re-run*: (a) two command lines to \
                               run as printed — followed here: git refuses the second, \
                               which wants a worktree's path; (b) the names of two git \
                               verbs, the second to be run once for each path the first \
                               lists — then the route prints no line to run, and here the \
                               first lists no worktree under `.jigc/worktrees/` at all";

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
        acts: &[Act::Run(3), Act::Run(4), Act::Run(2)],
        holds: torn_down![],
        untouched: STANDING,
        doubt: Some(WHICH_GIT_LINES),
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
const STAGED_DOC: &str = ".jigc/tasks/write-a-note/docs/commit:write-a-note.md";

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
/// for that; and the consent. The first two hold `<task-id>` where the id belongs.
const A_STAGED_DOC: &str = "an open task stages a doc that no commit has a copy of";
const AN_OPEN_TASKS_STAGED_DOC: [Case; 3] = [
    Case {
        name: "uninstall-staged-doc-read-and-discarded-as-printed",
        site: STAGED_PROSE,
        cause: A_STAGED_DOC,
        state: installed![A_TASK],
        acts: &[Act::Run(0), Act::Run(1), Act::Run(3)],
        holds: torn_down![],
        untouched: &["README.md", "notes.md", STAGED_DOC],
        doubt: Some(WHICH_LINE),
        ..UNINSTALL
    },
    Case {
        name: "uninstall-staged-doc-finalized-as-printed",
        site: STAGED_PROSE,
        cause: A_STAGED_DOC,
        state: installed![A_TASK],
        acts: &[Act::Run(2), Act::Run(3)],
        holds: torn_down![],
        untouched: &["README.md", "notes.md", STAGED_DOC],
        doubt: Some(WHICH_LINE),
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
];

/// The sites of `jigc uninstall` that have no case, each with why. None yet.
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
                     worktree, where no shell splits it",
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
];
