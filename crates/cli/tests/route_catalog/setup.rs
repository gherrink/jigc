//! The route catalog's cases for `jigc setup` — the family's table, declared by
//! `crates/cli/tests/route_catalog.rs`, which says what a case is held to.
//!
//! **What stands here.** The hook's link refusal; and the two refusals beside it that the
//! install raises before its first write — over an entry in the way of a file it
//! **replaces** (`replaced_path_refusal`, six codes) and over a link at a file it **merges
//! into** that leads where it will not follow (`merged_through_links`, four codes). Then
//! the refusals a run meets before any of those: no repository to install into, or no
//! `$HOME` (`locate_finding`, two sites); and the adapter profile — not loaded, without a
//! reference, with a launch template or a guide target a clause refuses (four sites, every
//! one reached through `JIGC_ADAPTERS_DIR` and no other way). Then the last question before
//! the first write — whether the settings file can take the merge — and the first refusal
//! of the write span, the marker's write into `packs.yaml`. And one behind them all: a git
//! step of the install commit that ran and refused. Each of those twenty sites has a row of
//! [`READINGS`]: the functions that were read to count its causes, the count, every cause
//! that has no case with why, and every way out of its route that is not followed with
//! why. A twenty-first has a row of [`REASONS`] and no case.
//!
//! **A cause is one way through the callee to the constructor**: which entry is in the way
//! — the file's own path, or a directory on the way to it — what that entry is, and, for a
//! link the install would follow, which of the ends it refuses. Two cases of one cause
//! carry the same [`Case::cause`] to the letter — that is how [`READINGS`] is held to the
//! cases.
//!
//! **A cause is followed every way its route offers.** A route here offers up to two acts
//! (*remove the link, or point it at a regular file inside this repository*) and a clause
//! that turns on git (*committing the change where git tracks the link*); neither is a
//! branch of a callee, and every red case of the first ten sites was on one of them. So the
//! cases of a cause are a small table of ways — one macro a shape of route, below — and a
//! way that cannot be followed at a site is a row of that site's reading.

use super::{Reading, Reason, Why};
use crate::support::route_case::{Act, Case, Check, Operand, Place, SCRIPT, Step, Var};

/// The hook `jigc setup` installs, from the repository root.
pub const HOOK: &str = ".git/hooks/pre-commit";

/// The refusal a `pre-commit` hook draws when it is a link that leads out of the
/// repository and its hooks directory, or to nothing.
pub const HOOK_LINK_SITE: &str = "crates/cli/src/setup.rs::hook_link_refusal::setup.install-hook#1";

// The six codes `replaced_path_refusal` raises — one a member of the install whose writer
// replaces what it finds, in the order the install writes them.
const BOOTSTRAP: &str =
    "crates/cli/src/setup.rs::replaced_path_refusal::setup.write-bootstrap#dynamic-1";
const STAMP: &str =
    "crates/cli/src/setup.rs::replaced_path_refusal::setup.version-stamp#dynamic-1";
const LAYER: &str =
    "crates/cli/src/setup.rs::replaced_path_refusal::setup.init-project-layer#dynamic-1";
const MARKER: &str =
    "crates/cli/src/setup.rs::replaced_path_refusal::setup.compose-marker#dynamic-1";
const RECORD: &str =
    "crates/cli/src/setup.rs::replaced_path_refusal::setup.inject-allowlist#dynamic-1";
const GUIDE: &str = "crates/cli/src/setup.rs::replaced_path_refusal::setup.write-guide#dynamic-1";

// The four codes `merged_through_links` raises — one a member whose writer keeps what it
// finds.
const HOST: &str =
    "crates/cli/src/setup.rs::merged_through_links::setup.inject-reference#dynamic-1";
const SETTINGS: &str =
    "crates/cli/src/setup.rs::merged_through_links::setup.inject-allowlist#dynamic-1";
const SEED: &str =
    "crates/cli/src/setup.rs::merged_through_links::setup.secrets-gitignore#dynamic-1";
const IGNORE: &str =
    "crates/cli/src/setup.rs::merged_through_links::setup.init-project-layer#dynamic-1";

// The install's members, from the repository root.
const AGENT_MD: &str = ".jigc/AGENT.md";
const VERSION: &str = ".jigc/version";
const GUIDE_MD: &str = ".claude/skills/jigc/SKILL.md";
const CLAUDE_MD: &str = "CLAUDE.md";
const SETTINGS_JSON: &str = ".claude/settings.json";
const ROOT_GITIGNORE: &str = ".gitignore";

/// What every case of `jigc setup` shares. A case states its acts itself: one that leaves
/// them empty under a printed route is refused by the runner.
const SETUP: Case = Case {
    name: "",
    site: "",
    cause: "",
    state: &[],
    refusing: &["setup"],
    from: Place::Repo(""),
    env: &[],
    acts: &[],
    holds: &[],
    witnesses: &[],
    doubt: None,
};

// **The acts.** Every route here prints the entry in the way as its first span and the
// re-run as the one after the entries — `--force`, the span after that, is named to say it
// changes nothing, and no case runs it. `…_AND_COMMIT` follows the clause *committing the
// change where git tracks the entry*, and stands where the case's state committed it.
const REMOVE: &[Act] = &[Act::Remove(Operand::Span(0)), Act::Run(1)];
const REMOVE_AND_COMMIT: &[Act] = &[
    Act::Remove(Operand::Span(0)),
    Act::Commit(Operand::Span(0)),
    Act::Run(1),
];
const MOVE_AWAY: &[Act] = &[Act::MoveAway(Operand::Span(0)), Act::Run(1)];
const MOVE_AWAY_AND_COMMIT: &[Act] = &[
    Act::MoveAway(Operand::Span(0)),
    Act::Commit(Operand::Span(0)),
    Act::Run(1),
];
const MAKE_DIRECTORY: &[Act] = &[Act::ReplaceWithDirectory(Operand::Span(0)), Act::Run(1)];
const MAKE_DIRECTORY_AND_COMMIT: &[Act] = &[
    Act::ReplaceWithDirectory(Operand::Span(0)),
    Act::Commit(Operand::Span(0)),
    Act::Run(1),
];
const REMOVE_BOTH: &[Act] = &[
    Act::Remove(Operand::Span(0)),
    Act::Remove(Operand::Span(1)),
    Act::Run(2),
];
const POINT_AT_COMMITTED: &[Act] = &[Act::PointAtCommitted(Operand::Span(0)), Act::Run(1)];
const POINT_AT_NEW: &[Act] = &[Act::PointAtNew(Operand::Span(0)), Act::Run(1)];
const POINT_AT_COMMITTED_AND_COMMIT: &[Act] = &[
    Act::PointAtCommitted(Operand::Span(0)),
    Act::Commit(Operand::Span(0)),
    Act::Run(1),
];
const POINT_AT_NEW_AND_COMMIT: &[Act] = &[
    Act::PointAtNew(Operand::Span(0)),
    Act::Commit(Operand::Span(0)),
    Act::Run(1),
];
const POINT_AT_SCRIPT: &[Act] = &[Act::PointAtScript(Operand::Span(0)), Act::Run(1)];

// The same, where one route joins the acts of two entries: two operands, then the re-run.
const REMOVE_BOTH_AND_COMMIT: &[Act] = &[
    Act::Remove(Operand::Span(0)),
    Act::Remove(Operand::Span(1)),
    Act::Commit(Operand::Span(0)),
    Act::Commit(Operand::Span(1)),
    Act::Run(2),
];

/// Commit everything the steps before it planted.
const COMMIT_IT: [Step; 2] = [
    Step::Git(&["add", "--all"]),
    Step::Git(&["commit", "-q", "-m", "what was planted"]),
];

/// Take the sandbox's one commit away again: `HEAD` is unborn, which is where `jigc setup`
/// seeds the root `.gitignore`.
const UNBORN: Step = Step::Git(&["update-ref", "-d", "HEAD"]);

/// A script of the repository's own, where the runner points a link that a route says to
/// point at *a script inside this repository* — committed by the steps behind it.
const A_SCRIPT: Step = Step::Script(Place::Repo(SCRIPT), "#!/bin/sh\nexit 0\n");

/// The two readings of *a regular file inside this repository*.
const WHICH_FILE: &str = "*point it at a regular file inside this repository*: (a) any regular \
                          file in the checkout, one written a moment ago and in no commit \
                          included — followed here; (b) a file the repository already holds \
                          committed — the case beside this one that ends \
                          `-committed-and-pointed-at-a-committed-file`";

/// The two readings of *a script inside this repository*.
const WHICH_SCRIPT: &str = "*point it at a script inside this repository*: (a) any script in \
                            the checkout, one written a moment ago and in no commit included \
                            — followed here; (b) a script the repository already holds \
                            committed — the case beside this one that ends \
                            `-pointed-at-a-committed-script`";

// ---------------------------------------------------------------------------
// The shapes of route, each followed every way it offers
// ---------------------------------------------------------------------------

/// **A hook that is a link, followed the three ways its route can be read**: the link
/// removed; pointed at a script inside the repository that is new; pointed at one the
/// repository holds committed. `installed` is what stands once the first way is followed.
macro_rules! hook_ways {
    (
        $prefix:literal, $cause:expr,
        plant: [$($plant:expr),* $(,)?],
        installed: [$($installed:expr),* $(,)?],
        witnesses: [$($witness:literal),* $(,)?],
        doubt: $doubt:expr $(,)?
    ) => {
        [
            Case {
                name: concat!($prefix, "-removed"),
                site: HOOK_LINK_SITE,
                cause: $cause,
                state: &[$($plant,)*],
                acts: REMOVE,
                holds: &[$($installed,)*],
                witnesses: &[$($witness,)*],
                ..SETUP
            },
            Case {
                name: concat!($prefix, "-pointed-at-a-new-script"),
                site: HOOK_LINK_SITE,
                cause: $cause,
                state: &[$($plant,)*],
                acts: POINT_AT_SCRIPT,
                holds: &[Check::RegularFile(Place::Repo(AGENT_MD))],
                witnesses: &[$($witness,)*],
                doubt: $doubt,
                ..SETUP
            },
            Case {
                name: concat!($prefix, "-pointed-at-a-committed-script"),
                site: HOOK_LINK_SITE,
                cause: $cause,
                state: &[A_SCRIPT, COMMIT_IT[0], COMMIT_IT[1], $($plant,)*],
                acts: POINT_AT_SCRIPT,
                holds: &[Check::RegularFile(Place::Repo(AGENT_MD))],
                witnesses: &[$($witness,)*],
                ..SETUP
            },
        ]
    };
}

/// **An entry to remove, where git does not track it and where it is committed** — a link
/// or a special file at a replaced member's path, or a link at the one merged-into member
/// the install never follows one at. `$member` is a regular file once the route is followed.
macro_rules! removed_ways {
    (
        $prefix:literal, $site:expr, $cause:expr, $member:literal,
        plant: [$($plant:expr),* $(,)?],
        witnesses: [$($witness:literal),* $(,)?] $(,)?
    ) => {
        [
            Case {
                name: concat!($prefix, "-removed"),
                site: $site,
                cause: $cause,
                state: &[$($plant,)*],
                acts: REMOVE,
                holds: &[Check::RegularFile(Place::Repo($member))],
                witnesses: &[$($witness,)*],
                ..SETUP
            },
            Case {
                name: concat!($prefix, "-committed-and-removed"),
                site: $site,
                cause: $cause,
                state: &[$($plant,)* COMMIT_IT[0], COMMIT_IT[1]],
                acts: REMOVE_AND_COMMIT,
                holds: &[Check::RegularFile(Place::Repo($member))],
                witnesses: &[$($witness,)*],
                ..SETUP
            },
        ]
    };
}

/// **A link at a replaced member's path**, to a file of somebody's outside the repository
/// with this body.
macro_rules! replaced_link {
    ($prefix:literal, $site:expr, $cause:expr, $member:literal, $body:literal) => {
        removed_ways!(
            $prefix, $site, $cause, $member,
            plant: [
                Step::File(Place::Outside("theirs"), $body),
                Step::Link(Place::Repo($member), Place::Outside("theirs")),
            ],
            witnesses: ["theirs"],
        )
    };
}

/// **A directory at a replaced member's path**, holding a file: moved out of the way, where
/// git does not track it and where it is committed.
macro_rules! replaced_directory {
    ($prefix:literal, $site:expr, $cause:expr, $member:literal) => {
        [
            Case {
                name: concat!($prefix, "-moved"),
                site: $site,
                cause: $cause,
                state: &[Step::File(Place::Repo(concat!($member, "/kept")), "kept\n")],
                acts: MOVE_AWAY,
                holds: &[
                    Check::RegularFile(Place::Repo($member)),
                    Check::RegularFile(Place::Moved(concat!($member, "/kept"))),
                ],
                ..SETUP
            },
            Case {
                name: concat!($prefix, "-committed-and-moved"),
                site: $site,
                cause: $cause,
                state: &[
                    Step::File(Place::Repo(concat!($member, "/kept")), "kept\n"),
                    COMMIT_IT[0],
                    COMMIT_IT[1],
                ],
                acts: MOVE_AWAY_AND_COMMIT,
                holds: &[
                    Check::RegularFile(Place::Repo($member)),
                    Check::RegularFile(Place::Moved(concat!($member, "/kept"))),
                ],
                ..SETUP
            },
        ]
    };
}

/// **A link standing in for a directory on the way to a member**: replaced by a real
/// directory, where git does not track the link and where it is committed. `installed` is
/// what stands afterwards beside the directory itself.
macro_rules! linked_way {
    (
        $prefix:literal, $site:expr, $cause:expr, $link:literal,
        plant: [$($plant:expr),* $(,)?],
        installed: [$($installed:expr),* $(,)?],
        witnesses: [$($witness:literal),* $(,)?] $(,)?
    ) => {
        [
            Case {
                name: concat!($prefix, "-replaced"),
                site: $site,
                cause: $cause,
                state: &[$($plant,)*],
                acts: MAKE_DIRECTORY,
                holds: &[Check::Directory(Place::Repo($link)), $($installed,)*],
                witnesses: &[$($witness,)*],
                ..SETUP
            },
            Case {
                name: concat!($prefix, "-committed-and-replaced"),
                site: $site,
                cause: $cause,
                state: &[$($plant,)* COMMIT_IT[0], COMMIT_IT[1]],
                acts: MAKE_DIRECTORY_AND_COMMIT,
                holds: &[Check::Directory(Place::Repo($link)), $($installed,)*],
                witnesses: &[$($witness,)*],
                ..SETUP
            },
        ]
    };
}

/// **A link at a merged-into member on a born `HEAD`, followed the six ways its route can
/// be read**: removed, or pointed at a regular file inside the repository that is committed
/// or new — each where git does not track the link, and where it is committed and the
/// change is committed as the route's clause says. `kept` is what the link led to inside
/// the repository, which must still stand.
macro_rules! link_ways {
    (
        $prefix:literal, $site:expr, $cause:expr, $member:expr,
        plant: [$($plant:expr),* $(,)?],
        kept: [$($kept:expr),* $(,)?],
        witnesses: [$($witness:literal),* $(,)?] $(,)?
    ) => {
        [
            Case {
                name: concat!($prefix, "-removed"),
                site: $site,
                cause: $cause,
                state: &[$($plant,)*],
                acts: REMOVE,
                holds: &[Check::RegularFile(Place::Repo($member)), $($kept,)*],
                witnesses: &[$($witness,)*],
                ..SETUP
            },
            Case {
                name: concat!($prefix, "-committed-and-removed"),
                site: $site,
                cause: $cause,
                state: &[$($plant,)* COMMIT_IT[0], COMMIT_IT[1]],
                acts: REMOVE_AND_COMMIT,
                holds: &[Check::RegularFile(Place::Repo($member)), $($kept,)*],
                witnesses: &[$($witness,)*],
                ..SETUP
            },
            Case {
                name: concat!($prefix, "-pointed-at-a-committed-file"),
                site: $site,
                cause: $cause,
                state: &[$($plant,)*],
                acts: POINT_AT_COMMITTED,
                holds: &[Check::RegularFile(Place::Repo(AGENT_MD)), $($kept,)*],
                witnesses: &[$($witness,)*],
                ..SETUP
            },
            Case {
                name: concat!($prefix, "-pointed-at-a-new-file"),
                site: $site,
                cause: $cause,
                state: &[$($plant,)*],
                acts: POINT_AT_NEW,
                holds: &[Check::RegularFile(Place::Repo(AGENT_MD)), $($kept,)*],
                witnesses: &[$($witness,)*],
                ..SETUP
            },
            Case {
                name: concat!($prefix, "-committed-and-pointed-at-a-committed-file"),
                site: $site,
                cause: $cause,
                state: &[$($plant,)* COMMIT_IT[0], COMMIT_IT[1]],
                acts: POINT_AT_COMMITTED_AND_COMMIT,
                holds: &[Check::RegularFile(Place::Repo(AGENT_MD)), $($kept,)*],
                witnesses: &[$($witness,)*],
                ..SETUP
            },
            Case {
                name: concat!($prefix, "-committed-and-pointed-at-a-new-file"),
                site: $site,
                cause: $cause,
                state: &[$($plant,)* COMMIT_IT[0], COMMIT_IT[1]],
                acts: POINT_AT_NEW_AND_COMMIT,
                holds: &[Check::RegularFile(Place::Repo(AGENT_MD)), $($kept,)*],
                witnesses: &[$($witness,)*],
                doubt: Some(WHICH_FILE),
                ..SETUP
            },
        ]
    };
}

/// **A link at the root `.gitignore` on an unborn `HEAD`, followed the three ways its route
/// can be there**: removed; pointed at a new regular file inside the repository; and the
/// same where the link is staged and the change committed, as the route's clause says of a
/// link git tracks. Each holds only if git then ignores `.env` — the floor the member is
/// seeded for.
macro_rules! seeded_link_ways {
    (
        $prefix:literal, $cause:expr,
        plant: [$($plant:expr),* $(,)?],
        kept: [$($kept:expr),* $(,)?],
        witnesses: [$($witness:literal),* $(,)?] $(,)?
    ) => {
        [
            Case {
                name: concat!($prefix, "-removed"),
                site: SEED,
                cause: $cause,
                state: &[UNBORN, $($plant,)*],
                acts: REMOVE,
                holds: &[
                    Check::RegularFile(Place::Repo(ROOT_GITIGNORE)),
                    Check::GitIgnores(".env"),
                    $($kept,)*
                ],
                witnesses: &[$($witness,)*],
                ..SETUP
            },
            Case {
                name: concat!($prefix, "-pointed-at-a-new-file"),
                site: SEED,
                cause: $cause,
                state: &[UNBORN, $($plant,)*],
                acts: POINT_AT_NEW,
                holds: &[
                    Check::RegularFile(Place::Repo(AGENT_MD)),
                    Check::GitIgnores(".env"),
                    $($kept,)*
                ],
                witnesses: &[$($witness,)*],
                ..SETUP
            },
            Case {
                name: concat!($prefix, "-staged-and-pointed-at-a-new-file"),
                site: SEED,
                cause: $cause,
                state: &[UNBORN, $($plant,)* Step::Git(&["add", ".gitignore"])],
                acts: POINT_AT_NEW_AND_COMMIT,
                holds: &[
                    Check::RegularFile(Place::Repo(AGENT_MD)),
                    Check::GitIgnores(".env"),
                    $($kept,)*
                ],
                witnesses: &[$($witness,)*],
                ..SETUP
            },
        ]
    };
}

// ---------------------------------------------------------------------------
// `hook_link_refusal` · `setup.install-hook`
// ---------------------------------------------------------------------------

/// The hook's own script, outside the repository — what a route must not take.
const OUR_HOOK: Step = Step::Script(Place::Outside("pre-commit"), "#!/bin/sh\necho ours\n");

/// `core.hooksPath` as a directory inside the repository, and as one outside it.
const HOOKS_IN_THE_REPOSITORY: Step = Step::Git(&["config", "core.hooksPath", ".githooks"]);
const HOOKS_OUTSIDE: Step = Step::Git(&["config", "core.hooksPath", "../outside/hooks"]);

/// The first case of the catalog — the hook a link to an existing script outside the
/// repository — and the two other ways out of its route.
pub const HOOK_OUT: [Case; 3] = hook_ways!(
    "setup-hook-link-out-to-a-script",
    "the `pre-commit` hook is a link to an existing script outside the repository",
    plant: [OUR_HOOK, Step::Link(Place::Repo(HOOK), Place::Outside("pre-commit"))],
    installed: [Check::RegularFile(Place::Repo(HOOK))],
    witnesses: ["pre-commit"],
    doubt: None,
);
const HOOK_TO_NOTHING: [Case; 3] = hook_ways!(
    "setup-hook-link-to-nothing",
    "the `pre-commit` hook is a link to a path outside the repository that holds nothing",
    plant: [Step::Link(Place::Repo(HOOK), Place::Outside("pre-commit"))],
    installed: [Check::RegularFile(Place::Repo(HOOK))],
    witnesses: ["pre-commit"],
    doubt: Some(WHICH_SCRIPT),
);
const HOOK_OUT_RELATIVELY: [Case; 3] = hook_ways!(
    "setup-hook-relative-link-out-to-a-script",
    "the `pre-commit` hook is a link, spelled relatively, to an existing script outside the \
     repository",
    plant: [OUR_HOOK, Step::LinkTo(Place::Repo(HOOK), "../../../outside/pre-commit")],
    installed: [Check::RegularFile(Place::Repo(HOOK))],
    witnesses: ["pre-commit"],
    doubt: None,
);
const HOOK_OUT_TO_A_DIRECTORY: [Case; 3] = hook_ways!(
    "setup-hook-link-out-to-a-directory",
    "the `pre-commit` hook is a link to a directory outside the repository",
    plant: [
        Step::File(Place::Outside("hooks/theirs"), "theirs\n"),
        Step::Link(Place::Repo(HOOK), Place::Outside("hooks")),
    ],
    installed: [Check::RegularFile(Place::Repo(HOOK))],
    witnesses: ["hooks/theirs"],
    doubt: None,
);
const HOOK_TO_A_DEVICE: [Case; 3] = hook_ways!(
    "setup-hook-link-to-dev-null",
    "the `pre-commit` hook is a link to `/dev/null`",
    plant: [Step::LinkTo(Place::Repo(HOOK), "/dev/null")],
    installed: [Check::RegularFile(Place::Repo(HOOK))],
    witnesses: [],
    doubt: None,
);
const HOOKS_PATH_INSIDE_TO_NOTHING: [Case; 3] = hook_ways!(
    "setup-hooks-path-inside-link-to-nothing",
    "`core.hooksPath` names a directory inside the repository, and the hook there is a link \
     to a path that holds nothing",
    plant: [
        HOOKS_IN_THE_REPOSITORY,
        Step::Link(Place::Repo(".githooks/pre-commit"), Place::Outside("pre-commit")),
    ],
    installed: [Check::RegularFile(Place::Repo(".githooks/pre-commit"))],
    witnesses: ["pre-commit"],
    doubt: Some(WHICH_SCRIPT),
);
const HOOKS_PATH_INSIDE_OUT: [Case; 3] = hook_ways!(
    "setup-hooks-path-inside-link-out-to-a-script",
    "`core.hooksPath` names a directory inside the repository, and the hook there is a link \
     to an existing script outside the repository",
    plant: [
        HOOKS_IN_THE_REPOSITORY,
        OUR_HOOK,
        Step::Link(Place::Repo(".githooks/pre-commit"), Place::Outside("pre-commit")),
    ],
    installed: [Check::RegularFile(Place::Repo(".githooks/pre-commit"))],
    witnesses: ["pre-commit"],
    doubt: None,
);
const HOOKS_PATH_OUTSIDE_TO_NOTHING: [Case; 3] = hook_ways!(
    "setup-hooks-path-outside-link-to-nothing",
    "`core.hooksPath` names a directory outside the repository, and the hook there is a link \
     to a path that holds nothing",
    plant: [
        HOOKS_OUTSIDE,
        Step::Link(Place::Outside("hooks/pre-commit"), Place::Outside("nothing")),
    ],
    installed: [Check::RegularFile(Place::Outside("hooks/pre-commit"))],
    witnesses: ["nothing"],
    doubt: Some(WHICH_SCRIPT),
);
const HOOKS_PATH_OUTSIDE_OUT: [Case; 3] = hook_ways!(
    "setup-hooks-path-outside-link-out-of-it-to-a-script",
    "`core.hooksPath` names a directory outside the repository, and the hook there is a link \
     to an existing script in neither",
    plant: [
        HOOKS_OUTSIDE,
        OUR_HOOK,
        Step::Link(Place::Outside("hooks/pre-commit"), Place::Outside("pre-commit")),
    ],
    installed: [Check::RegularFile(Place::Outside("hooks/pre-commit"))],
    witnesses: ["pre-commit"],
    doubt: None,
);

/// The one way the hook's route does not name and a repository can still be in: the link
/// under an in-repository hooks directory is committed. The route has no clause for it.
const HOOKS_PATH_INSIDE_COMMITTED: [Case; 1] = [Case {
    name: "setup-hooks-path-inside-committed-link-to-nothing-removed",
    site: HOOK_LINK_SITE,
    cause: "`core.hooksPath` names a directory inside the repository, and the hook there is a \
            link to a path that holds nothing",
    state: &[
        HOOKS_IN_THE_REPOSITORY,
        Step::Link(
            Place::Repo(".githooks/pre-commit"),
            Place::Outside("pre-commit"),
        ),
        COMMIT_IT[0],
        COMMIT_IT[1],
    ],
    acts: REMOVE,
    holds: &[Check::RegularFile(Place::Repo(".githooks/pre-commit"))],
    witnesses: &["pre-commit"],
    ..SETUP
}];

// ---------------------------------------------------------------------------
// `replaced_path_refusal`
// ---------------------------------------------------------------------------

const AGENT_A_LINK: [Case; 2] = replaced_link!(
    "setup-bootstrap-file-a-link",
    BOOTSTRAP,
    "`.jigc/AGENT.md` is a link",
    ".jigc/AGENT.md",
    "theirs\n"
);
const AGENT_A_DIRECTORY: [Case; 2] = replaced_directory!(
    "setup-bootstrap-file-a-directory",
    BOOTSTRAP,
    "`.jigc/AGENT.md` is a directory",
    ".jigc/AGENT.md"
);
const WORKBENCH_A_LINK: [Case; 2] = linked_way!(
    "setup-workbench-a-link",
    BOOTSTRAP,
    "`.jigc`, on the way to `.jigc/AGENT.md`, is a link to a directory outside the repository",
    ".jigc",
    plant: [
        Step::File(Place::Outside("store/theirs"), "theirs\n"),
        Step::Link(Place::Repo(".jigc"), Place::Outside("store")),
    ],
    installed: [Check::RegularFile(Place::Repo(AGENT_MD))],
    witnesses: ["store/theirs", "store/AGENT.md", "store/version"],
);
const AGENT_A_FIFO: [Case; 1] = [Case {
    name: "setup-bootstrap-file-a-special-file",
    site: BOOTSTRAP,
    cause: "`.jigc/AGENT.md` is a FIFO",
    state: &[Step::Fifo(Place::Repo(AGENT_MD))],
    acts: REMOVE,
    holds: &[Check::RegularFile(Place::Repo(AGENT_MD))],
    ..SETUP
}];

/// Several entries in the way at once: one finding, one route that joins their acts.
const TWO_REPLACED: &str = "two replaced files are in the way at once — `.jigc/AGENT.md` and \
                            `.jigc/version` are both links — and the route joins their acts";
const TWO_REPLACED_LINKS: [Case; 2] = [
    Case {
        name: "setup-two-replaced-files-links-removed",
        site: BOOTSTRAP,
        cause: TWO_REPLACED,
        state: &[
            Step::File(Place::Outside("theirs"), "theirs\n"),
            Step::File(Place::Outside("also-theirs"), "also theirs\n"),
            Step::Link(Place::Repo(AGENT_MD), Place::Outside("theirs")),
            Step::Link(Place::Repo(VERSION), Place::Outside("also-theirs")),
        ],
        acts: REMOVE_BOTH,
        holds: &[
            Check::RegularFile(Place::Repo(AGENT_MD)),
            Check::RegularFile(Place::Repo(VERSION)),
        ],
        witnesses: &["theirs", "also-theirs"],
        ..SETUP
    },
    Case {
        name: "setup-two-replaced-files-links-committed-and-removed",
        site: BOOTSTRAP,
        cause: TWO_REPLACED,
        state: &[
            Step::File(Place::Outside("theirs"), "theirs\n"),
            Step::File(Place::Outside("also-theirs"), "also theirs\n"),
            Step::Link(Place::Repo(AGENT_MD), Place::Outside("theirs")),
            Step::Link(Place::Repo(VERSION), Place::Outside("also-theirs")),
            COMMIT_IT[0],
            COMMIT_IT[1],
        ],
        acts: REMOVE_BOTH_AND_COMMIT,
        holds: &[
            Check::RegularFile(Place::Repo(AGENT_MD)),
            Check::RegularFile(Place::Repo(VERSION)),
        ],
        witnesses: &["theirs", "also-theirs"],
        ..SETUP
    },
];

const STAMP_A_LINK: [Case; 2] = replaced_link!(
    "setup-version-stamp-a-link",
    STAMP,
    "`.jigc/version` is a link",
    ".jigc/version",
    "theirs\n"
);
const STAMP_A_DIRECTORY: [Case; 2] = replaced_directory!(
    "setup-version-stamp-a-directory",
    STAMP,
    "`.jigc/version` is a directory",
    ".jigc/version"
);
const STAMP_A_FIFO: [Case; 1] = [Case {
    name: "setup-version-stamp-a-special-file",
    site: STAMP,
    cause: "`.jigc/version` is a FIFO",
    state: &[Step::Fifo(Place::Repo(VERSION))],
    acts: REMOVE,
    holds: &[Check::RegularFile(Place::Repo(VERSION))],
    ..SETUP
}];

const LAYER_UNDER_A_LINK: [Case; 2] = linked_way!(
    "setup-project-layer-a-link",
    LAYER,
    "`.jigc/config`, on the way to `.jigc/config/.gitkeep`, is a link to a directory outside \
     the repository",
    ".jigc/config",
    plant: [
        Step::File(Place::Outside("config/theirs"), "theirs\n"),
        Step::Link(Place::Repo(".jigc/config"), Place::Outside("config")),
    ],
    installed: [Check::RegularFile(Place::Repo(".jigc/config/.gitkeep"))],
    witnesses: ["config/theirs", "config/.gitkeep", "config/packs.yaml"],
);
const LAYER_A_LINK: [Case; 2] = replaced_link!(
    "setup-layer-marker-a-link",
    LAYER,
    "`.jigc/config/.gitkeep` is a link",
    ".jigc/config/.gitkeep",
    "theirs\n"
);
const LAYER_A_DIRECTORY: [Case; 2] = replaced_directory!(
    "setup-layer-marker-a-directory",
    LAYER,
    "`.jigc/config/.gitkeep` is a directory",
    ".jigc/config/.gitkeep"
);
const LAYER_A_FIFO: [Case; 1] = [Case {
    name: "setup-layer-marker-a-special-file",
    site: LAYER,
    cause: "`.jigc/config/.gitkeep` is a FIFO",
    state: &[Step::Fifo(Place::Repo(".jigc/config/.gitkeep"))],
    acts: REMOVE,
    holds: &[Check::RegularFile(Place::Repo(".jigc/config/.gitkeep"))],
    ..SETUP
}];

const MARKER_A_LINK: [Case; 2] = replaced_link!(
    "setup-compose-marker-a-link",
    MARKER,
    "`.jigc/config/packs.yaml` is a link",
    ".jigc/config/packs.yaml",
    "packs: []\n"
);
const MARKER_A_DIRECTORY: [Case; 2] = replaced_directory!(
    "setup-compose-marker-a-directory",
    MARKER,
    "`.jigc/config/packs.yaml` is a directory",
    ".jigc/config/packs.yaml"
);
/// **Held.** `jigc setup` reads this path before it reaches the refusal, and the read of a
/// FIFO does not return.
const MARKER_A_FIFO: [Case; 1] = [Case {
    name: "setup-compose-marker-a-special-file",
    site: MARKER,
    cause: "`.jigc/config/packs.yaml` is a FIFO",
    state: &[Step::Fifo(Place::Repo(".jigc/config/packs.yaml"))],
    acts: REMOVE,
    holds: &[Check::RegularFile(Place::Repo(".jigc/config/packs.yaml"))],
    ..SETUP
}];

const RECORD_A_LINK: [Case; 2] = replaced_link!(
    "setup-settings-record-a-link",
    RECORD,
    "`.jigc/settings-entries.json` is a link",
    ".jigc/settings-entries.json",
    "{}\n"
);
const RECORD_A_DIRECTORY: [Case; 2] = replaced_directory!(
    "setup-settings-record-a-directory",
    RECORD,
    "`.jigc/settings-entries.json` is a directory",
    ".jigc/settings-entries.json"
);
const RECORD_A_FIFO: [Case; 1] = [Case {
    name: "setup-settings-record-a-special-file",
    site: RECORD,
    cause: "`.jigc/settings-entries.json` is a FIFO",
    state: &[Step::Fifo(Place::Repo(".jigc/settings-entries.json"))],
    acts: REMOVE,
    holds: &[Check::RegularFile(Place::Repo(".jigc/settings-entries.json"))],
    ..SETUP
}];

const GUIDE_UNDER_A_LINKED_ASSISTANT_DIRECTORY: [Case; 2] = linked_way!(
    "setup-guide-under-a-linked-assistant-directory",
    GUIDE,
    "`.claude`, on the way to the guide, is a link to a directory outside the repository \
     that holds no guide",
    ".claude",
    plant: [
        Step::File(Place::Outside("dotclaude/theirs"), "theirs\n"),
        Step::Link(Place::Repo(".claude"), Place::Outside("dotclaude")),
    ],
    installed: [Check::RegularFile(Place::Repo(GUIDE_MD))],
    witnesses: [
        "dotclaude/theirs",
        "dotclaude/settings.json",
        "dotclaude/skills/jigc/SKILL.md",
    ],
);
const GUIDE_UNDER_A_LINKED_SKILLS_DIRECTORY: [Case; 2] = linked_way!(
    "setup-guide-under-a-linked-skills-directory",
    GUIDE,
    "`.claude/skills`, on the way to the guide, is a link to a directory outside the \
     repository",
    ".claude/skills",
    plant: [
        Step::File(Place::Outside("skills/theirs"), "theirs\n"),
        Step::Link(Place::Repo(".claude/skills"), Place::Outside("skills")),
    ],
    installed: [Check::RegularFile(Place::Repo(GUIDE_MD))],
    witnesses: ["skills/theirs", "skills/jigc/SKILL.md"],
);
const GUIDE_UNDER_A_LINKED_SKILL_DIRECTORY: [Case; 2] = linked_way!(
    "setup-guide-under-a-linked-skill-directory",
    GUIDE,
    "`.claude/skills/jigc`, on the way to the guide, is a link to a directory outside the \
     repository",
    ".claude/skills/jigc",
    plant: [
        Step::File(Place::Outside("skill/theirs"), "theirs\n"),
        Step::Link(Place::Repo(".claude/skills/jigc"), Place::Outside("skill")),
    ],
    installed: [Check::RegularFile(Place::Repo(GUIDE_MD))],
    witnesses: ["skill/theirs", "skill/SKILL.md"],
);
/// **Held.** `guide_ownership` reads the guide's path before the refusal is reached.
const GUIDE_A_FIFO: [Case; 1] = [Case {
    name: "setup-guide-a-special-file",
    site: GUIDE,
    cause: "the guide's own path is a FIFO",
    state: &[Step::Fifo(Place::Repo(GUIDE_MD))],
    acts: REMOVE,
    holds: &[Check::RegularFile(Place::Repo(GUIDE_MD))],
    ..SETUP
}];

// ---------------------------------------------------------------------------
// `merged_through_links`
// ---------------------------------------------------------------------------

const HOST_TO_NOTHING: [Case; 6] = link_ways!(
    "setup-host-link-to-nothing",
    HOST,
    "`CLAUDE.md` is a link to a path that holds nothing",
    CLAUDE_MD,
    plant: [Step::Link(Place::Repo(CLAUDE_MD), Place::Outside("nothing"))],
    kept: [],
    witnesses: ["nothing"],
);
const HOST_OUT: [Case; 6] = link_ways!(
    "setup-host-link-out-of-the-repository",
    HOST,
    "`CLAUDE.md` is a link to a file outside the repository",
    CLAUDE_MD,
    plant: [
        Step::File(Place::Outside("theirs"), "theirs\n"),
        Step::Link(Place::Repo(CLAUDE_MD), Place::Outside("theirs")),
    ],
    kept: [],
    witnesses: ["theirs"],
);
const HOST_TO_A_DIRECTORY: [Case; 6] = link_ways!(
    "setup-host-link-to-a-directory",
    HOST,
    "`CLAUDE.md` is a link to a directory inside the repository",
    CLAUDE_MD,
    plant: [
        Step::Dir(Place::Repo("docs")),
        Step::Link(Place::Repo(CLAUDE_MD), Place::Repo("docs")),
    ],
    kept: [Check::Directory(Place::Repo("docs"))],
    witnesses: [],
);
const HOST_INTO_GITS_DIRECTORY: [Case; 6] = link_ways!(
    "setup-host-link-into-gits-directory",
    HOST,
    "`CLAUDE.md` is a link to a file inside git's own directory",
    CLAUDE_MD,
    plant: [
        Step::File(Place::Repo(".git/theirs"), "theirs\n"),
        Step::Link(Place::Repo(CLAUDE_MD), Place::Repo(".git/theirs")),
    ],
    kept: [Check::RegularFile(Place::Repo(".git/theirs"))],
    witnesses: [],
);
/// Two links the install will not follow at once: one finding, one route that joins
/// their acts — followed by its first way, where git tracks neither link and where both
/// are committed.
const TWO_MERGED: &str = "two merged-into files are links the install will not follow at once \
                          — `CLAUDE.md` and `.claude/settings.json` — and the route joins \
                          their acts";
const TWO_MERGED_LINKS: [Case; 2] = [
    Case {
        name: "setup-two-merged-files-links-removed",
        site: HOST,
        cause: TWO_MERGED,
        state: &[
            Step::Link(Place::Repo(CLAUDE_MD), Place::Outside("nothing")),
            Step::Link(Place::Repo(SETTINGS_JSON), Place::Outside("nothing-either")),
        ],
        acts: REMOVE_BOTH,
        holds: &[
            Check::RegularFile(Place::Repo(CLAUDE_MD)),
            Check::RegularFile(Place::Repo(SETTINGS_JSON)),
        ],
        witnesses: &["nothing", "nothing-either"],
        ..SETUP
    },
    Case {
        name: "setup-two-merged-files-links-committed-and-removed",
        site: HOST,
        cause: TWO_MERGED,
        state: &[
            Step::Link(Place::Repo(CLAUDE_MD), Place::Outside("nothing")),
            Step::Link(Place::Repo(SETTINGS_JSON), Place::Outside("nothing-either")),
            COMMIT_IT[0],
            COMMIT_IT[1],
        ],
        acts: REMOVE_BOTH_AND_COMMIT,
        holds: &[
            Check::RegularFile(Place::Repo(CLAUDE_MD)),
            Check::RegularFile(Place::Repo(SETTINGS_JSON)),
        ],
        witnesses: &["nothing", "nothing-either"],
        ..SETUP
    },
];

const SETTINGS_TO_NOTHING: [Case; 6] = link_ways!(
    "setup-settings-link-to-nothing",
    SETTINGS,
    "`.claude/settings.json` is a link to a path that holds nothing",
    SETTINGS_JSON,
    plant: [Step::Link(Place::Repo(SETTINGS_JSON), Place::Outside("nothing"))],
    kept: [],
    witnesses: ["nothing"],
);
const SETTINGS_OUT: [Case; 6] = link_ways!(
    "setup-settings-link-out-of-the-repository",
    SETTINGS,
    "`.claude/settings.json` is a link to a file outside the repository",
    SETTINGS_JSON,
    plant: [
        Step::File(Place::Outside("theirs"), "{}\n"),
        Step::Link(Place::Repo(SETTINGS_JSON), Place::Outside("theirs")),
    ],
    kept: [],
    witnesses: ["theirs"],
);
const SETTINGS_TO_A_DIRECTORY: [Case; 6] = link_ways!(
    "setup-settings-link-to-a-directory",
    SETTINGS,
    "`.claude/settings.json` is a link to a directory inside the repository",
    SETTINGS_JSON,
    plant: [
        Step::Dir(Place::Repo("conf")),
        Step::Link(Place::Repo(SETTINGS_JSON), Place::Repo("conf")),
    ],
    kept: [Check::Directory(Place::Repo("conf"))],
    witnesses: [],
);
const SETTINGS_INTO_GITS_DIRECTORY: [Case; 6] = link_ways!(
    "setup-settings-link-into-gits-directory",
    SETTINGS,
    "`.claude/settings.json` is a link to a file inside git's own directory",
    SETTINGS_JSON,
    plant: [
        Step::File(Place::Repo(".git/theirs"), "{}\n"),
        Step::Link(Place::Repo(SETTINGS_JSON), Place::Repo(".git/theirs")),
    ],
    kept: [Check::RegularFile(Place::Repo(".git/theirs"))],
    witnesses: [],
);

/// A guide that is not jigc's, at the end of a `.claude` that is a link: the install then
/// leaves the guide alone, and the link is `.claude/settings.json`'s to answer for.
const SETTINGS_UNDER_A_LINK_THAT_HOLDS_NONE: [Case; 2] = linked_way!(
    "setup-settings-under-a-link-that-holds-none",
    SETTINGS,
    "`.claude`, on the way to `.claude/settings.json`, is a link to a directory outside the \
     repository that holds a guide of somebody's and no settings file",
    ".claude",
    plant: [
        Step::File(Place::Outside("dotclaude/skills/jigc/SKILL.md"), "theirs\n"),
        Step::Link(Place::Repo(".claude"), Place::Outside("dotclaude")),
    ],
    installed: [Check::RegularFile(Place::Repo(SETTINGS_JSON))],
    witnesses: ["dotclaude/skills/jigc/SKILL.md", "dotclaude/settings.json"],
);
const SETTINGS_UNDER_A_LINK_OUT: [Case; 2] = linked_way!(
    "setup-settings-under-a-link-out-of-the-repository",
    SETTINGS,
    "`.claude`, on the way to `.claude/settings.json`, is a link to a directory outside the \
     repository that holds a guide of somebody's and a settings file",
    ".claude",
    plant: [
        Step::File(Place::Outside("dotclaude/skills/jigc/SKILL.md"), "theirs\n"),
        Step::File(Place::Outside("dotclaude/settings.json"), "{}\n"),
        Step::Link(Place::Repo(".claude"), Place::Outside("dotclaude")),
    ],
    installed: [Check::RegularFile(Place::Repo(SETTINGS_JSON))],
    witnesses: ["dotclaude/skills/jigc/SKILL.md", "dotclaude/settings.json"],
);
const SETTINGS_UNDER_A_LINK_TO_A_DIRECTORY_THERE: [Case; 2] = linked_way!(
    "setup-settings-under-a-link-to-a-directory-there",
    SETTINGS,
    "`.claude`, on the way to `.claude/settings.json`, is a link to a directory inside the \
     repository where `settings.json` is a directory",
    ".claude",
    plant: [
        Step::File(Place::Repo("dotclaude/skills/jigc/SKILL.md"), "theirs\n"),
        Step::Dir(Place::Repo("dotclaude/settings.json")),
        Step::Link(Place::Repo(".claude"), Place::Repo("dotclaude")),
    ],
    installed: [
        Check::RegularFile(Place::Repo(SETTINGS_JSON)),
        Check::RegularFile(Place::Repo("dotclaude/skills/jigc/SKILL.md")),
    ],
    witnesses: [],
);
const SETTINGS_UNDER_A_LINK_INTO_GITS_DIRECTORY: [Case; 2] = linked_way!(
    "setup-settings-under-a-link-into-gits-directory",
    SETTINGS,
    "`.claude`, on the way to `.claude/settings.json`, is a link to a directory inside git's \
     own directory",
    ".claude",
    plant: [
        Step::File(Place::Repo(".git/dotclaude/skills/jigc/SKILL.md"), "theirs\n"),
        Step::File(Place::Repo(".git/dotclaude/settings.json"), "{}\n"),
        Step::Link(Place::Repo(".claude"), Place::Repo(".git/dotclaude")),
    ],
    installed: [
        Check::RegularFile(Place::Repo(SETTINGS_JSON)),
        Check::RegularFile(Place::Repo(".git/dotclaude/settings.json")),
    ],
    witnesses: [],
);

const SEED_TO_NOTHING: [Case; 3] = seeded_link_ways!(
    "setup-seeded-ignore-link-to-nothing",
    "on an unborn `HEAD`, the root `.gitignore` is a link to a path that holds nothing",
    plant: [Step::Link(Place::Repo(ROOT_GITIGNORE), Place::Outside("nothing"))],
    kept: [],
    witnesses: ["nothing"],
);
const SEED_OUT: [Case; 3] = seeded_link_ways!(
    "setup-seeded-ignore-link-out-of-the-repository",
    "on an unborn `HEAD`, the root `.gitignore` is a link to a file outside the repository",
    plant: [
        Step::File(Place::Outside("theirs"), "theirs\n"),
        Step::Link(Place::Repo(ROOT_GITIGNORE), Place::Outside("theirs")),
    ],
    kept: [],
    witnesses: ["theirs"],
);
const SEED_TO_A_DIRECTORY: [Case; 3] = seeded_link_ways!(
    "setup-seeded-ignore-link-to-a-directory",
    "on an unborn `HEAD`, the root `.gitignore` is a link to a directory inside the \
     repository",
    plant: [
        Step::Dir(Place::Repo("docs")),
        Step::Link(Place::Repo(ROOT_GITIGNORE), Place::Repo("docs")),
    ],
    kept: [Check::Directory(Place::Repo("docs"))],
    witnesses: [],
);
const SEED_INTO_GITS_DIRECTORY: [Case; 3] = seeded_link_ways!(
    "setup-seeded-ignore-link-into-gits-directory",
    "on an unborn `HEAD`, the root `.gitignore` is a link to a file inside git's own \
     directory",
    plant: [
        Step::File(Place::Repo(".git/theirs"), "theirs\n"),
        Step::Link(Place::Repo(ROOT_GITIGNORE), Place::Repo(".git/theirs")),
    ],
    kept: [Check::RegularFile(Place::Repo(".git/theirs"))],
    witnesses: [],
);

const WORKBENCH_IGNORE_A_LINK: [Case; 2] = removed_ways!(
    "setup-workbench-ignore-a-link",
    IGNORE,
    "`.jigc/.gitignore` is a link",
    ".jigc/.gitignore",
    plant: [
        Step::File(Place::Outside("theirs"), "theirs\n"),
        Step::Link(Place::Repo(".jigc/.gitignore"), Place::Outside("theirs")),
    ],
    witnesses: ["theirs"],
);

// ---------------------------------------------------------------------------
// `locate_finding` · `setup.repo-root`
// ---------------------------------------------------------------------------

/// Outside a repository.
const NOT_IN_A_REPOSITORY: &str =
    "crates/cli/src/locate.rs::locate_finding::setup.repo-root#dynamic-1";
/// Inside one, where the team layer's path cannot be resolved.
const NO_HOME_SITE: &str = "crates/cli/src/locate.rs::locate_finding::setup.repo-root#dynamic-2";

/// **A command run where there is no repository, followed the two ways its route offers**:
/// run from inside the target repository — the sandbox's, since the route names it and
/// prints none — and `git init` here first, then the command again. The route prints no
/// command line for either re-run.
macro_rules! outside_ways {
    ($prefix:literal, $cause:expr, plant: [$($plant:expr),* $(,)?] $(,)?) => {
        [
            Case {
                name: concat!($prefix, "-run-inside-the-repository"),
                site: NOT_IN_A_REPOSITORY,
                cause: $cause,
                state: &[$($plant,)*],
                from: Place::Outside(""),
                acts: &[Act::RerunInTheRepository],
                holds: &[Check::RegularFile(Place::Repo(AGENT_MD))],
                ..SETUP
            },
            Case {
                name: concat!($prefix, "-a-repository-made-here"),
                site: NOT_IN_A_REPOSITORY,
                cause: $cause,
                state: &[$($plant,)*],
                from: Place::Outside(""),
                acts: &[Act::Run(0), Act::Rerun],
                holds: &[Check::RegularFile(Place::Outside(".jigc/AGENT.md"))],
                ..SETUP
            },
        ]
    };
}

const OUTSIDE_A_REPOSITORY: [Case; 2] = outside_ways!(
    "setup-outside-a-repository",
    "the command is run in a directory that is in no repository: neither it nor a directory \
     above it holds a `.git`",
    plant: [],
);
const BESIDE_A_GIT_LINK_TO_NOTHING: [Case; 2] = outside_ways!(
    "setup-beside-a-git-link-to-nothing",
    "the directory's `.git` is a link that leads to nothing, and no directory above it holds \
     a `.git`",
    plant: [Step::LinkTo(Place::Outside(".git"), "gits-directory")],
);

const HOME_NOT_SET: [Case; 1] = [Case {
    name: "setup-home-not-set-run-as-the-route-prints",
    site: NO_HOME_SITE,
    cause: "`$HOME` is not set, and the command is run inside a repository",
    env: &[Var::Unset("HOME")],
    acts: &[Act::Run(0)],
    holds: &[Check::RegularFile(Place::Repo(AGENT_MD))],
    ..SETUP
}];

// ---------------------------------------------------------------------------
// The four profile refusals — `run` and `install`, before any install path is asked about
// ---------------------------------------------------------------------------

const PROFILE_LOAD: &str = "crates/cli/src/setup.rs::run::setup.profile-load#1";
const PROFILE_INCOMPLETE: &str = "crates/cli/src/setup.rs::install::setup.profile-incomplete#1";
const LAUNCH_TEMPLATE: &str = "crates/cli/src/setup.rs::install::setup.spawn-template#1";
const GUIDE_TARGET: &str = "crates/cli/src/setup.rs::install::setup.guide-target#1";

/// **The one road to a profile that is not the binary's own**: `JIGC_ADAPTERS_DIR` names a
/// directory, and the profile is read from `<directory>/claude-code.yaml`. Every case of
/// the four sites stands in the sandbox's repository with the variable set to this
/// directory of the sandbox.
const ADAPTERS: &[Var] = &[Var::Path("JIGC_ADAPTERS_DIR", Place::Outside("adapters"))];
const PROFILE: Place = Place::Outside("adapters/claude-code.yaml");

/// The one act a route here leaves a reader: the command again. None of the four prints a
/// command line, and none names a file.
const AGAIN: &[Act] = &[Act::Rerun];

/// A profile — the shipped one's shape in small, and valid where every piece is the one
/// [`template!`] and [`guide!`] pass for the others: the `inject:` list, the launch
/// template, the guide's file and its one front-matter key.
macro_rules! profile {
    ($inject:literal, $template:literal, $file:literal, $key:literal) => {
        concat!(
            "assistant: claude-code\ninject:",
            $inject,
            "\nallowlist:\n  file: .claude/settings.json\n  permit: [\"Bash(jigc:*)\"]\n  \
             deny: []\nspawn:\n  template: ",
            $template,
            "\nguide:\n  file: ",
            $file,
            "\n  front-matter:\n    ",
            $key,
            ": jigc\n"
        )
    };
}

/// A profile whose launch template is this YAML scalar, and nothing else wrong.
macro_rules! template {
    ($template:literal) => {
        profile!(
            "\n  - reference: { file: CLAUDE.md, to: .jigc/AGENT.md, syntax: at-import }",
            $template,
            ".claude/skills/jigc/SKILL.md",
            "name"
        )
    };
}

/// A profile whose guide target is this file under this front-matter key, and nothing else
/// wrong.
macro_rules! guide {
    ($file:literal, $key:literal) => {
        profile!(
            "\n  - reference: { file: CLAUDE.md, to: .jigc/AGENT.md, syntax: at-import }",
            "\"Use `jigc workflow {{workflow}} --task {{task_id}}`\"",
            $file,
            $key
        )
    };
}

/// **One refusal over the profile, followed the one way its route leaves**: the command
/// again. The profile's own bytes are the case's witness — nothing a route here says
/// touches them.
macro_rules! profile_refused {
    ($name:literal, $site:expr, $cause:expr, plant: [$($plant:expr),* $(,)?] $(,)?) => {
        [Case {
            name: $name,
            site: $site,
            cause: $cause,
            state: &[$($plant,)*],
            env: ADAPTERS,
            acts: AGAIN,
            holds: &[Check::RegularFile(Place::Repo(AGENT_MD))],
            witnesses: &["adapters/claude-code.yaml"],
            ..SETUP
        }]
    };
}

const NO_PROFILE_IN_THE_DIRECTORY: [Case; 1] = profile_refused!(
    "setup-profile-not-in-the-adapters-directory",
    PROFILE_LOAD,
    "the directory `JIGC_ADAPTERS_DIR` names holds no profile to read",
    plant: [Step::Dir(Place::Outside("adapters"))],
);
/// A state's file is text, so the two bytes are written by a `git` alias — a shell line
/// git runs at the repository's root.
const PROFILE_NOT_TEXT: [Case; 1] = profile_refused!(
    "setup-profile-not-utf-8",
    PROFILE_LOAD,
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
    "setup-profile-malformed",
    PROFILE_LOAD,
    "the profile in that directory is YAML that is no profile",
    plant: [Step::File(PROFILE, "assistant: claude-code\n")],
);

const PROFILE_WITHOUT_A_REFERENCE: [Case; 1] = profile_refused!(
    "setup-profile-declares-no-reference",
    PROFILE_INCOMPLETE,
    "the profile declares no reference among its inject targets",
    plant: [Step::File(
        PROFILE,
        profile!(
            " []",
            "\"Use `jigc workflow {{workflow}} --task {{task_id}}`\"",
            ".claude/skills/jigc/SKILL.md",
            "name"
        ),
    )],
);

const TEMPLATE_WITHOUT_A_TASK: [Case; 1] = profile_refused!(
    "setup-launch-template-without-the-task-placeholder",
    LAUNCH_TEMPLATE,
    "the launch template holds no `{{task_id}}`",
    plant: [Step::File(PROFILE, template!("\"Use `jigc workflow {{workflow}} --task x`\""))],
);
const TEMPLATE_WITHOUT_A_WORKFLOW: [Case; 1] = profile_refused!(
    "setup-launch-template-without-the-workflow-placeholder",
    LAUNCH_TEMPLATE,
    "the launch template holds no `{{workflow}}`",
    plant: [Step::File(PROFILE, template!("\"Use `jigc workflow x --task {{task_id}}`\""))],
);
/// **The one route of the four sites that prints a `jigc` span** — the shape the template
/// lacks, `jigc workflow … --task …`. The frame takes a `jigc` span for a command line and
/// refuses a re-run beside it, so the case runs the span.
const TEMPLATE_WITHOUT_THE_INVOCATION: [Case; 1] = [Case {
    name: "setup-launch-template-without-the-invocation",
    site: LAUNCH_TEMPLATE,
    cause: "the launch template holds no `jigc workflow … --task …` line",
    state: &[Step::File(PROFILE, template!("\"Use `{{workflow}} {{task_id}}`\""))],
    env: ADAPTERS,
    acts: &[Act::Run(0)],
    holds: &[Check::RegularFile(Place::Repo(AGENT_MD))],
    witnesses: &["adapters/claude-code.yaml"],
    doubt: Some(
        "the route is the violated clause and nothing else — *spawn template missing the \
         required `jigc workflow … --task …` invocation*: (a) the backticked `jigc` line is a \
         command to run — followed here, as printed; (b) it is the shape the profile's \
         template must hold, and the route prints nothing to run and names no file to change",
    ),
    ..SETUP
}];
const TEMPLATE_OF_THREE_LINES: [Case; 1] = profile_refused!(
    "setup-launch-template-of-three-lines",
    LAUNCH_TEMPLATE,
    "the launch template holds more than one newline",
    plant: [Step::File(
        PROFILE,
        template!("\"a\\nb\\nc `jigc workflow {{workflow}} --task {{task_id}}`\""),
    )],
);
const TEMPLATE_WITHOUT_A_SPAN: [Case; 1] = profile_refused!(
    "setup-launch-template-without-a-backticked-span",
    LAUNCH_TEMPLATE,
    "the launch template does not hold exactly one backticked span",
    plant: [Step::File(
        PROFILE,
        template!("\"Use jigc workflow {{workflow}} --task {{task_id}}\""),
    )],
);
const TEMPLATE_WITH_A_MARKER: [Case; 1] = profile_refused!(
    "setup-launch-template-with-a-forbidden-marker",
    LAUNCH_TEMPLATE,
    "the launch template holds a forbidden marker",
    plant: [Step::File(
        PROFILE,
        template!("\"# `jigc workflow {{workflow}} --task {{task_id}}`\""),
    )],
);
const TEMPLATE_WITH_OTHER_SHELL: [Case; 1] = profile_refused!(
    "setup-launch-template-with-other-shell",
    LAUNCH_TEMPLATE,
    "the launch template's span holds shell beyond the one `cd` it may",
    plant: [Step::File(
        PROFILE,
        template!("\"Use `ls && jigc workflow {{workflow}} --task {{task_id}}`\""),
    )],
);

const GUIDE_WITHOUT_A_PATH: [Case; 1] = profile_refused!(
    "setup-guide-target-without-a-path",
    GUIDE_TARGET,
    "the guide target's file is empty",
    plant: [Step::File(PROFILE, guide!("\"\"", "name"))],
);
const GUIDE_AT_AN_ABSOLUTE_PATH: [Case; 1] = profile_refused!(
    "setup-guide-target-absolute",
    GUIDE_TARGET,
    "the guide target's file is an absolute path",
    plant: [Step::File(PROFILE, guide!("/elsewhere/SKILL.md", "name"))],
);
const GUIDE_OUT_OF_THE_REPOSITORY: [Case; 1] = profile_refused!(
    "setup-guide-target-climbing-out",
    GUIDE_TARGET,
    "the guide target's file holds a `..` component",
    plant: [Step::File(PROFILE, guide!("../SKILL.md", "name"))],
);
const GUIDE_WITH_A_STAMP_KEY: [Case; 1] = profile_refused!(
    "setup-guide-target-declares-a-stamp-key",
    GUIDE_TARGET,
    "the guide target's front matter declares one of jigc's own stamp keys",
    plant: [Step::File(PROFILE, guide!(".claude/skills/jigc/SKILL.md", "jigc-version"))],
);
const GUIDE_WITH_A_KEY_THAT_IS_NONE: [Case; 1] = profile_refused!(
    "setup-guide-target-malformed-key",
    GUIDE_TARGET,
    "the guide target's front matter has a key that is no plain YAML key",
    plant: [Step::File(PROFILE, guide!(".claude/skills/jigc/SKILL.md", "\"a key\""))],
);

// ---------------------------------------------------------------------------
// `InstallCommitRejection::finding` · `setup.install-commit`
// ---------------------------------------------------------------------------

/// A git step of the install commit that ran and refused — after the install's writes.
const COMMIT_REFUSED: &str = "crates/cli/src/setup.rs::finding::setup.install-commit#1";

/// The route of an identity refusal prints the two `git config` lines and the re-run.
const TELL_GIT_AND_RUN_AGAIN: &[Act] = &[Act::Run(0), Act::Run(1), Act::Run(2)];

/// The sandbox's identity, taken out of the commands under test.
const NO_IDENTITY: &[Var] = &[Var::Unset("GIT_CONFIG_COUNT")];

const INDEX_LOCKED: [Case; 1] = [Case {
    name: "setup-install-staging-refused-over-a-locked-index",
    site: COMMIT_REFUSED,
    cause: "`git add` refuses the install's paths — here over a lock on the index",
    state: &[Step::File(Place::Repo(".git/index.lock"), "")],
    acts: &[
        Act::Remove(Operand::Unprinted(".git/index.lock")),
        Act::Run(1),
    ],
    holds: &[Check::RegularFile(Place::Repo(AGENT_MD))],
    ..SETUP
}];
const NOBODY_TO_COMMIT_AS: [Case; 1] = [Case {
    name: "setup-install-commit-refused-with-no-identity",
    site: COMMIT_REFUSED,
    cause: "`git commit` refuses for want of an identity: git has no name and no address",
    env: NO_IDENTITY,
    acts: TELL_GIT_AND_RUN_AGAIN,
    holds: &[Check::RegularFile(Place::Repo(AGENT_MD))],
    ..SETUP
}];
const AN_EMPTY_NAME: [Case; 1] = [Case {
    name: "setup-install-commit-refused-over-an-empty-name",
    site: COMMIT_REFUSED,
    cause: "`git commit` refuses for want of an identity: the name git is given is empty",
    state: &[
        Step::Git(&["config", "user.email", "someone@example.com"]),
        Step::Git(&["config", "user.name", ""]),
    ],
    env: NO_IDENTITY,
    acts: TELL_GIT_AND_RUN_AGAIN,
    holds: &[Check::RegularFile(Place::Repo(AGENT_MD))],
    ..SETUP
}];
/// `--no-verify` passes over `pre-commit` and `commit-msg`, and not over this hook.
const A_HOOK_REFUSES_THE_COMMIT: [Case; 1] = [Case {
    name: "setup-install-commit-refused-by-a-hook",
    site: COMMIT_REFUSED,
    cause: "`git commit` refuses for another reason — here a `prepare-commit-msg` hook that \
            exits 1",
    state: &[Step::Script(
        Place::Repo(".git/hooks/prepare-commit-msg"),
        "#!/bin/sh\necho refused by a hook >&2\nexit 1\n",
    )],
    acts: &[
        Act::Remove(Operand::Unprinted(".git/hooks/prepare-commit-msg")),
        Act::Run(1),
    ],
    holds: &[Check::RegularFile(Place::Repo(AGENT_MD))],
    ..SETUP
}];

// ---------------------------------------------------------------------------
// `install` · `setup.inject-allowlist` — the settings file, asked before the first write
// ---------------------------------------------------------------------------

/// The merge into the settings file, run in memory before anything is written.
const SETTINGS_ASKED: &str = "crates/cli/src/setup.rs::install::setup.inject-allowlist#1";

/// The route of a settings file the merge cannot take: fix it, commit the fix, run again.
const MEND_COMMIT_AND_RUN_AGAIN: &[Act] = &[
    Act::Mend(Operand::Span(0)),
    Act::Commit(Operand::Span(0)),
    Act::Run(1),
];

/// **A settings file the merge cannot take, in the two states that reach the refusal**:
/// committed, on a born `HEAD` — uncommitted there, the file is refused first as work in no
/// commit — and untracked on an unborn one, where a file the install merges into rides the
/// first commit. The route is the same and is followed the same: the file mended, the fix
/// committed, the command again.
macro_rules! unmergeable_ways {
    ($prefix:literal, $cause:expr, plant: [$($plant:expr),* $(,)?] $(,)?) => {
        [
            Case {
                name: concat!($prefix, "-committed"),
                site: SETTINGS_ASKED,
                cause: $cause,
                state: &[$($plant,)* COMMIT_IT[0], COMMIT_IT[1]],
                acts: MEND_COMMIT_AND_RUN_AGAIN,
                holds: &[Check::RegularFile(Place::Repo(AGENT_MD))],
                ..SETUP
            },
            Case {
                name: concat!($prefix, "-on-an-unborn-head"),
                site: SETTINGS_ASKED,
                cause: $cause,
                state: &[UNBORN, $($plant,)*],
                acts: MEND_COMMIT_AND_RUN_AGAIN,
                holds: &[Check::RegularFile(Place::Repo(AGENT_MD))],
                ..SETUP
            },
        ]
    };
}

/// The same, where the cause is the file's text.
macro_rules! unmergeable {
    ($prefix:literal, $cause:expr, $body:literal) => {
        unmergeable_ways!(
            $prefix,
            $cause,
            plant: [Step::File(Place::Repo(SETTINGS_JSON), $body)],
        )
    };
}

const SETTINGS_NOT_JSON: [Case; 2] = unmergeable!(
    "setup-settings-not-json",
    "the settings file is no JSON",
    "not json\n"
);
/// Two bytes a `git` alias writes, as for the profile: a state's file is text.
const SETTINGS_NOT_TEXT: [Case; 2] = unmergeable_ways!(
    "setup-settings-not-utf-8",
    "the settings file is not UTF-8",
    plant: [
        Step::Dir(Place::Repo(".claude")),
        Step::Git(&[
            "-c",
            "alias.plant=!printf '\\377\\376' > .claude/settings.json",
            "plant",
        ]),
    ],
);
const SETTINGS_ROOT_NO_OBJECT: [Case; 2] = unmergeable!(
    "setup-settings-root-no-object",
    "the settings file's root is no JSON object",
    "[]\n"
);
const SETTINGS_PERMISSIONS_NO_OBJECT: [Case; 2] = unmergeable!(
    "setup-settings-permissions-no-object",
    "the settings file's `permissions` is no JSON object",
    "{\"permissions\": []}\n"
);
const SETTINGS_ALLOW_NO_ARRAY: [Case; 2] = unmergeable!(
    "setup-settings-allow-no-array",
    "the settings file's `permissions.allow` is no JSON array",
    "{\"permissions\": {\"allow\": {}}}\n"
);
const SETTINGS_HOOKS_NO_OBJECT: [Case; 2] = unmergeable!(
    "setup-settings-hooks-no-object",
    "the settings file's `hooks` is no JSON object",
    "{\"hooks\": []}\n"
);
const SETTINGS_EVENT_NO_ARRAY: [Case; 2] = unmergeable!(
    "setup-settings-hook-event-no-array",
    "the settings file's `hooks.SessionStart` is no JSON array",
    "{\"hooks\": {\"SessionStart\": {}}}\n"
);
const SETTINGS_DENY_NO_ARRAY: [Case; 2] = unmergeable!(
    "setup-settings-deny-no-array",
    "the settings file's `permissions.deny` is no JSON array",
    "{\"permissions\": {\"deny\": {}}}\n"
);
/// The one cause with the other route: the read fails, and not over the file's bytes.
const SETTINGS_A_DIRECTORY: [Case; 1] = [Case {
    name: "setup-settings-a-directory-made-readable",
    site: SETTINGS_ASKED,
    cause: "the settings file cannot be read — here a directory stands at its path",
    state: &[Step::Dir(Place::Repo(SETTINGS_JSON))],
    acts: &[Act::MakeReadable(Operand::Span(0)), Act::Run(1)],
    holds: &[Check::RegularFile(Place::Repo(SETTINGS_JSON))],
    ..SETUP
}];

// ---------------------------------------------------------------------------
// `write_install_span` · `setup.compose-marker` — the first of the write span's refusals
// ---------------------------------------------------------------------------

/// The marker's write into `.jigc/config/packs.yaml` — the read before it, the parse, and
/// the write itself, every error of the three under one route.
const MARKER_WRITE: &str = "crates/cli/src/setup.rs::write_install_span::setup.compose-marker#1";

const PACKS_YAML: &str = ".jigc/config/packs.yaml";

/// The route's one act — *ensure `.jigc/config/` is writable* — and the re-run.
const MAKE_WRITABLE_AND_RUN_AGAIN: &[Act] = &[Act::MakeWritable(Operand::Span(0)), Act::Run(1)];

/// **A committed `packs.yaml` the marker cannot be written into**, followed the one way the
/// route offers. Committed, because a file there that git does not hold is refused first as
/// work in no commit, on a born `HEAD` and on an unborn one.
macro_rules! marker_refused {
    ($name:literal, $cause:expr, plant: [$($plant:expr),* $(,)?] $(,)?) => {
        [Case {
            name: $name,
            site: MARKER_WRITE,
            cause: $cause,
            state: &[$($plant,)*],
            acts: MAKE_WRITABLE_AND_RUN_AGAIN,
            holds: &[Check::RegularFile(Place::Repo(AGENT_MD))],
            ..SETUP
        }]
    };
}

const PACKS_NOT_TEXT: [Case; 1] = marker_refused!(
    "setup-compose-marker-packs-file-not-utf-8",
    "the committed `packs.yaml` cannot be read as text",
    plant: [
        Step::Dir(Place::Repo(".jigc/config")),
        Step::Git(&[
            "-c",
            "alias.plant=!printf '\\377\\376' > .jigc/config/packs.yaml",
            "plant",
        ]),
        COMMIT_IT[0],
        COMMIT_IT[1],
    ],
);
const PACKS_NOT_YAML: [Case; 1] = marker_refused!(
    "setup-compose-marker-packs-file-not-yaml",
    "the committed `packs.yaml` is no YAML",
    plant: [Step::File(Place::Repo(PACKS_YAML), "packs: [\n"), COMMIT_IT[0], COMMIT_IT[1]],
);
const PACKS_NO_MAPPING: [Case; 1] = marker_refused!(
    "setup-compose-marker-packs-file-no-mapping",
    "the committed `packs.yaml` is YAML and no mapping",
    plant: [Step::File(Place::Repo(PACKS_YAML), "- a list\n"), COMMIT_IT[0], COMMIT_IT[1]],
);
const PACKS_READ_ONLY: [Case; 1] = marker_refused!(
    "setup-compose-marker-packs-file-read-only",
    "the committed `packs.yaml` may not be written, in a directory that may",
    plant: [
        Step::File(Place::Repo(PACKS_YAML), "packs: []\n"),
        COMMIT_IT[0],
        COMMIT_IT[1],
        Step::Git(&["-c", "alias.shut=!chmod 444 .jigc/config/packs.yaml", "shut"]),
    ],
);
/// The one state the route's words are true of: the directory itself may not be written,
/// and there is no `packs.yaml` in it yet. `.gitkeep` is committed there, so the step
/// before this one has nothing to write.
const CONFIG_READ_ONLY: [Case; 1] = marker_refused!(
    "setup-compose-marker-config-directory-read-only",
    "`.jigc/config/` may not be written, and holds no `packs.yaml` yet",
    plant: [
        Step::File(Place::Repo(".jigc/config/.gitkeep"), ""),
        COMMIT_IT[0],
        COMMIT_IT[1],
        Step::Git(&["-c", "alias.shut=!chmod 555 .jigc/config", "shut"]),
    ],
);

/// The cases of `jigc setup`, a group a cause — and, within a group, a case a way out of
/// the cause's route.
pub const CASES: &[&[Case]] = &[
    &HOOK_OUT,
    &HOOK_TO_NOTHING,
    &HOOK_OUT_RELATIVELY,
    &HOOK_OUT_TO_A_DIRECTORY,
    &HOOK_TO_A_DEVICE,
    &HOOKS_PATH_INSIDE_TO_NOTHING,
    &HOOKS_PATH_INSIDE_COMMITTED,
    &HOOKS_PATH_INSIDE_OUT,
    &HOOKS_PATH_OUTSIDE_TO_NOTHING,
    &HOOKS_PATH_OUTSIDE_OUT,
    &AGENT_A_LINK,
    &AGENT_A_DIRECTORY,
    &WORKBENCH_A_LINK,
    &AGENT_A_FIFO,
    &TWO_REPLACED_LINKS,
    &STAMP_A_LINK,
    &STAMP_A_DIRECTORY,
    &STAMP_A_FIFO,
    &LAYER_UNDER_A_LINK,
    &LAYER_A_LINK,
    &LAYER_A_DIRECTORY,
    &LAYER_A_FIFO,
    &MARKER_A_LINK,
    &MARKER_A_DIRECTORY,
    &MARKER_A_FIFO,
    &RECORD_A_LINK,
    &RECORD_A_DIRECTORY,
    &RECORD_A_FIFO,
    &GUIDE_UNDER_A_LINKED_ASSISTANT_DIRECTORY,
    &GUIDE_UNDER_A_LINKED_SKILLS_DIRECTORY,
    &GUIDE_UNDER_A_LINKED_SKILL_DIRECTORY,
    &GUIDE_A_FIFO,
    &HOST_TO_NOTHING,
    &HOST_OUT,
    &HOST_TO_A_DIRECTORY,
    &HOST_INTO_GITS_DIRECTORY,
    &TWO_MERGED_LINKS,
    &SETTINGS_TO_NOTHING,
    &SETTINGS_OUT,
    &SETTINGS_TO_A_DIRECTORY,
    &SETTINGS_INTO_GITS_DIRECTORY,
    &SETTINGS_UNDER_A_LINK_THAT_HOLDS_NONE,
    &SETTINGS_UNDER_A_LINK_OUT,
    &SETTINGS_UNDER_A_LINK_TO_A_DIRECTORY_THERE,
    &SETTINGS_UNDER_A_LINK_INTO_GITS_DIRECTORY,
    &SEED_TO_NOTHING,
    &SEED_OUT,
    &SEED_TO_A_DIRECTORY,
    &SEED_INTO_GITS_DIRECTORY,
    &WORKBENCH_IGNORE_A_LINK,
    &OUTSIDE_A_REPOSITORY,
    &BESIDE_A_GIT_LINK_TO_NOTHING,
    &HOME_NOT_SET,
    &NO_PROFILE_IN_THE_DIRECTORY,
    &PROFILE_NOT_TEXT,
    &PROFILE_MALFORMED,
    &PROFILE_WITHOUT_A_REFERENCE,
    &TEMPLATE_WITHOUT_A_TASK,
    &TEMPLATE_WITHOUT_A_WORKFLOW,
    &TEMPLATE_WITHOUT_THE_INVOCATION,
    &TEMPLATE_OF_THREE_LINES,
    &TEMPLATE_WITHOUT_A_SPAN,
    &TEMPLATE_WITH_A_MARKER,
    &TEMPLATE_WITH_OTHER_SHELL,
    &GUIDE_WITHOUT_A_PATH,
    &GUIDE_AT_AN_ABSOLUTE_PATH,
    &GUIDE_OUT_OF_THE_REPOSITORY,
    &GUIDE_WITH_A_STAMP_KEY,
    &GUIDE_WITH_A_KEY_THAT_IS_NONE,
    &INDEX_LOCKED,
    &NOBODY_TO_COMMIT_AS,
    &AN_EMPTY_NAME,
    &A_HOOK_REFUSES_THE_COMMIT,
    &SETTINGS_NOT_JSON,
    &SETTINGS_NOT_TEXT,
    &SETTINGS_ROOT_NO_OBJECT,
    &SETTINGS_PERMISSIONS_NO_OBJECT,
    &SETTINGS_ALLOW_NO_ARRAY,
    &SETTINGS_HOOKS_NO_OBJECT,
    &SETTINGS_EVENT_NO_ARRAY,
    &SETTINGS_DENY_NO_ARRAY,
    &SETTINGS_A_DIRECTORY,
    &PACKS_NOT_TEXT,
    &PACKS_NOT_YAML,
    &PACKS_NO_MAPPING,
    &PACKS_READ_ONLY,
    &CONFIG_READ_ONLY,
];

/// The sites of `jigc setup` that have no case, each with why.
pub const REASONS: &[Reason] = &[Reason {
    site: "crates/cli/src/setup.rs::commit_install::setup.install-commit#1",
    why: Why::NotConstructible(
        "read: `commit_install`, `repo::SeamSubject::live_exempt` and `verify`, \
         `render::blocked_finding`. The site is the fallback for a posture re-probe that \
         fails without a finding of its own, and `verify` has two such arms — the git \
         directory is no longer the one the subject was built against, and `HEAD` is on \
         another ref — while a posture breach carries its finding and its own code. The \
         subject is built and verified in one expression, so either arm needs the \
         repository to change between two reads a few instructions apart: no state stands \
         still in it",
    ),
}];

/// What both callees share: the question each asks of a member's path, and the table whose
/// order decides the code.
const SHARED: [&str; 4] = [
    "crates/cli/src/setup.rs::install — the order of the three refusals before the first write",
    "crates/cli/src/setup.rs::install_tracked_paths — the members, their writers and their order",
    "crates/cli/src/regular_file.rs::blocker and Blocker::clearing_act — what is in the way, and the act a route names for it",
    "crates/engine/src/store.rs::home_entry — the three shapes of an entry that is no regular file",
];

/// Why a link at `.jigc` is no case of a site but the first replacing member's.
const WORKBENCH_LINK_IS_THE_BOOTSTRAP_FILES: Why = Why::NotConstructible(
    "a link at `.jigc` is in the way of `.jigc/AGENT.md` first, and the refusal is raised \
     under the first blocked member's code — `setup.write-bootstrap`, the cases \
     `setup-workbench-a-link-…`",
);

/// Why the clause *committing the removal where git tracks the entry* is not followed over
/// a special file.
const GIT_TRACKS_NO_FIFO: (&str, Why) = (
    "the special file committed, and its removal committed",
    Why::NotConstructible("git tracks regular files and links: a FIFO is in no index"),
);

/// **What was read for each site, and how many causes it holds** — a hand count, with its
/// derivation; the causes that have no case, each with why; and the ways out of its route
/// that are not followed, each with why.
///
/// The count is of ways through the callee, as this file's head says. One thing it leaves
/// to one site a function: **several entries in the way at once**, which either callee
/// answers with one finding under the first member's code and one route that joins the
/// acts. The join is the same code whichever member is first, so it is counted — and
/// followed — at the first member of each function (`.jigc/AGENT.md`, `CLAUDE.md`) and at no
/// other site.
///
/// **Where a replaced member's link leads is no input of its refusal or of its route** —
/// `replaced_path_refusal` asks what the entry is, and its one act removes it — so such a
/// link is followed at one end, the one with a file to lose. A merged-into member's link is
/// followed at each of the four ends `link_end` refuses, since there the end is asked.
pub const READINGS: &[Reading] = &[
    Reading {
        site: HOOK_LINK_SITE,
        read: &[
            "crates/cli/src/setup.rs::hook_link_refusal",
            "crates/cli/src/setup.rs::display_hook_path — which path the route names as the link",
            "crates/cli/src/setup.rs::resolve_hooks_dir and precommit_hook_in — where the hook is",
        ],
        causes: 9,
        derivation: "`hook_link_refusal` has two arms — the link leads to nothing, or leads \
                     out of the repository and its hooks directory — and `display_hook_path` \
                     words the path the route names by resolving the link: where it leads \
                     to nothing the route names the link, and where it leads anywhere it \
                     names **the end**. So what stands at the end decides what a reader is \
                     told to remove: a script (spelled absolutely, or relatively — the \
                     canary's second key), a directory, a device. `resolve_hooks_dir` asks \
                     git, so `core.hooksPath` moves the hook: into the repository, or out \
                     of it; each with the two arms. Five states of the default hooks \
                     directory and two of each `core.hooksPath`",
        without_case: &[],
        not_followed: &[(
            "the link committed, at any hooks directory but one inside the repository",
            Why::NotConstructible(
                "git tracks nothing under its own directory, where the default hooks \
                 directory is, and nothing outside the repository",
            ),
        )],
    },
    Reading {
        site: BOOTSTRAP,
        read: &[
            "crates/cli/src/setup.rs::replaced_path_refusal",
            SHARED[0],
            SHARED[1],
            SHARED[2],
            SHARED[3],
        ],
        causes: 5,
        derivation: "`.jigc/AGENT.md` is the first replacing member: one directory on its way \
                     can be a link (`.jigc`), its own path can be a link, a directory or a \
                     special file, and — counted here for the function — several entries can \
                     be in the way at once. Where a link leads is not asked",
        without_case: &[],
        not_followed: &[GIT_TRACKS_NO_FIFO],
    },
    Reading {
        site: STAMP,
        read: &[
            "crates/cli/src/setup.rs::replaced_path_refusal",
            SHARED[0],
            SHARED[1],
            SHARED[2],
            SHARED[3],
        ],
        causes: 4,
        derivation: "`.jigc/version`: the directory on its way (`.jigc`), and its own path as \
                     a link, a directory or a special file",
        without_case: &[(
            "`.jigc`, on the way to `.jigc/version`, is a link",
            WORKBENCH_LINK_IS_THE_BOOTSTRAP_FILES,
        )],
        not_followed: &[GIT_TRACKS_NO_FIFO],
    },
    Reading {
        site: LAYER,
        read: &[
            "crates/cli/src/setup.rs::replaced_path_refusal",
            SHARED[0],
            SHARED[1],
            SHARED[2],
            SHARED[3],
        ],
        causes: 5,
        derivation: "`.jigc/config/.gitkeep`: two directories on its way (`.jigc`, \
                     `.jigc/config`), and its own path as a link, a directory or a special \
                     file",
        without_case: &[(
            "`.jigc`, on the way to `.jigc/config/.gitkeep`, is a link",
            WORKBENCH_LINK_IS_THE_BOOTSTRAP_FILES,
        )],
        not_followed: &[GIT_TRACKS_NO_FIFO],
    },
    Reading {
        site: MARKER,
        read: &[
            "crates/cli/src/setup.rs::replaced_path_refusal",
            SHARED[0],
            SHARED[1],
            SHARED[2],
            SHARED[3],
        ],
        causes: 5,
        derivation: "`.jigc/config/packs.yaml`: two directories on its way (`.jigc`, \
                     `.jigc/config`), and its own path as a link, a directory or a special \
                     file",
        without_case: &[
            (
                "`.jigc`, on the way to `.jigc/config/packs.yaml`, is a link",
                WORKBENCH_LINK_IS_THE_BOOTSTRAP_FILES,
            ),
            (
                "`.jigc/config`, on the way to `.jigc/config/packs.yaml`, is a link",
                Why::NotConstructible(
                    "a link at `.jigc/config` is in the way of `.jigc/config/.gitkeep` \
                     first, the member written before this one — the refusal is raised \
                     under `setup.init-project-layer`, the cases \
                     `setup-project-layer-a-link-…`",
                ),
            ),
        ],
        not_followed: &[GIT_TRACKS_NO_FIFO],
    },
    Reading {
        site: RECORD,
        read: &[
            "crates/cli/src/setup.rs::replaced_path_refusal",
            SHARED[0],
            SHARED[1],
            SHARED[2],
            SHARED[3],
        ],
        causes: 4,
        derivation: "`.jigc/settings-entries.json`: the directory on its way (`.jigc`), and \
                     its own path as a link, a directory or a special file",
        without_case: &[(
            "`.jigc`, on the way to `.jigc/settings-entries.json`, is a link",
            WORKBENCH_LINK_IS_THE_BOOTSTRAP_FILES,
        )],
        not_followed: &[GIT_TRACKS_NO_FIFO],
    },
    Reading {
        site: GUIDE,
        read: &[
            "crates/cli/src/setup.rs::replaced_path_refusal",
            "crates/cli/src/setup.rs::guide_ownership — asked of the guide's path before the \
             member table is built; a copy that is not jigc's takes the guide out of the table",
            SHARED[0],
            SHARED[1],
            SHARED[2],
            SHARED[3],
        ],
        causes: 6,
        derivation: "`.claude/skills/jigc/SKILL.md`: three directories on its way (`.claude`, \
                     `.claude/skills`, `.claude/skills/jigc`), and its own path as a link, a \
                     directory or a special file",
        without_case: &[
            (
                "the guide's own path is a link",
                Why::NotConstructible(
                    "`guide_ownership` asks the entry first and reads a link as a copy that \
                     is not jigc's: the guide leaves the member table, `jigc setup` exits 0 \
                     and says so under the advisory `adapter-guide.user-modified` — driven, \
                     the link's end untouched",
                ),
            ),
            (
                "the guide's own path is a directory",
                Why::NotConstructible(
                    "`guide_ownership` cannot read a directory as jigc's text, so the guide \
                     leaves the member table as for a link — driven, exit 0",
                ),
            ),
        ],
        not_followed: &[GIT_TRACKS_NO_FIFO],
    },
    Reading {
        site: HOST,
        read: &[
            "crates/cli/src/setup.rs::merged_through_links",
            "crates/cli/src/setup.rs::link_end — the four ends a followed link is refused at",
            SHARED[0],
            SHARED[1],
            SHARED[2],
        ],
        causes: 5,
        derivation: "`CLAUDE.md` stands at the repository root, so only its own path can be \
                     a link; `link_end` refuses four ends — nothing, out of the checkout, no \
                     regular file, git's own directory — and, counted here for the function, \
                     several links can be refused at once. An entry there that is no link is \
                     not this function's",
        without_case: &[],
        not_followed: &[(
            "the joined route of two links, by its second way out",
            Why::NotConstructible(
                "the runner has one committed file and one new file to point a link at, and \
                 the join's two links are the reference host and the settings file: pointed \
                 at one file, the install's own writes meet in it — driven, the settings \
                 merge refuses the file the reference was appended to — which says nothing \
                 of the route. Each link's second way out is followed at its own site",
            ),
        )],
    },
    Reading {
        site: SETTINGS,
        read: &[
            "crates/cli/src/setup.rs::merged_through_links",
            "crates/cli/src/setup.rs::link_end — the four ends a followed link is refused at",
            "crates/cli/src/setup.rs::guide_ownership — what keeps `replaced_path_refusal` \
             from answering for a link at `.claude` first",
            SHARED[0],
            SHARED[1],
            SHARED[2],
        ],
        causes: 8,
        derivation: "`.claude/settings.json`: the link is at its own path or at `.claude`, \
                     the one directory on its way, and either is refused at each of \
                     `link_end`'s four ends",
        without_case: &[],
        not_followed: &[],
    },
    Reading {
        site: SEED,
        read: &[
            "crates/cli/src/setup.rs::merged_through_links",
            "crates/cli/src/setup.rs::link_end — the four ends a followed link is refused at",
            "crates/cli/src/setup.rs::is_fresh_repo — the root `.gitignore` is a member on an \
             unborn `HEAD` only",
            SHARED[0],
            SHARED[1],
            SHARED[2],
        ],
        causes: 4,
        derivation: "the root `.gitignore`, a member where `HEAD` is unborn: only its own \
                     path can be a link, refused at each of `link_end`'s four ends",
        without_case: &[],
        not_followed: &[
            (
                "the link pointed at a file the repository holds committed",
                Why::NotConstructible(
                    "the site is reached on an unborn `HEAD` only, where no file is in a \
                     commit",
                ),
            ),
            (
                "the link staged, removed, and the removal committed",
                Why::NotConstructible(
                    "a link that was staged and never committed leaves no removal to \
                     commit: `git commit` over its path answers that the pathspec matches \
                     nothing — driven",
                ),
            ),
        ],
    },
    Reading {
        site: IGNORE,
        read: &[
            "crates/cli/src/setup.rs::merged_through_links",
            SHARED[0],
            SHARED[1],
            SHARED[2],
        ],
        causes: 2,
        derivation: "`.jigc/.gitignore` is the one member a link is never followed at, so \
                     `link_end` is not asked: the link is at its own path, wherever it \
                     leads, or at `.jigc`, the directory on its way",
        without_case: &[(
            "`.jigc`, on the way to `.jigc/.gitignore`, is a link",
            Why::NotConstructible(
                "`replaced_path_refusal` is asked before this function and answers for a \
                 link at `.jigc` under `setup.write-bootstrap` — the cases \
                 `setup-workbench-a-link-…`",
            ),
        )],
        not_followed: &[],
    },
    Reading {
        site: NOT_IN_A_REPOSITORY,
        read: &[
            "crates/cli/src/locate.rs::locate_finding and not_in_repo_route",
            "crates/cli/src/locate.rs::locate and locate_from — the two ways `locate` fails",
            "crates/cli/src/repo.rs::discover_repo_root — the walk up, and what it asks of a `.git`",
            "crates/cli/src/cli.rs::run_setup — the start is the directory the command is run in",
        ],
        causes: 3,
        derivation: "`locate_finding` answers here where `discover_repo_root` finds nothing, \
                     and that walk asks one thing of each directory on the way up: whether \
                     `.git` exists there, following a link. Three ways to nothing: no entry \
                     of that name anywhere above; an entry that is a link leading to nothing; \
                     an entry that cannot be asked about. What the `.git` *is* where it \
                     exists — a file, an empty directory — is no cause of this site: the \
                     walk takes it, and a later refusal answers",
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
            "crates/cli/src/setup.rs::run — the route this arm is handed",
        ],
        causes: 1,
        derivation: "`locate` fails two ways, and the other is the site above, which is asked \
                     first: so this arm is `$HOME` not set, inside a repository. A `$HOME` \
                     that is set and empty is no cause — the variable is there",
        without_case: &[],
        not_followed: &[],
    },
    Reading {
        site: PROFILE_LOAD,
        read: &[
            "crates/cli/src/setup.rs::run",
            "crates/cli/src/adapter.rs::load_profile and load_profile_from",
            "crates/cli/src/adapter.rs::ProfileError — the four ways a load fails",
        ],
        causes: 4,
        derivation: "`load_profile_from` fails four ways: no profile of that name is embedded; \
                     the file `JIGC_ADAPTERS_DIR` selects cannot be read; its bytes are not \
                     UTF-8; its YAML is no profile. The second is one arm whatever the read's \
                     error was, and is followed where the directory holds no such file",
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
        site: PROFILE_INCOMPLETE,
        read: &[
            "crates/cli/src/setup.rs::install — the order of its refusals before any path is asked about",
            "crates/cli/src/adapter.rs::AdapterProfile::reference",
        ],
        causes: 1,
        derivation: "`reference` finds the first inject target that is a reference; there is \
                     none where the list is empty or holds hooks alone — one arm",
        without_case: &[],
        not_followed: &[],
    },
    Reading {
        site: LAUNCH_TEMPLATE,
        read: &[
            "crates/cli/src/setup.rs::install",
            "crates/cli/src/adapter.rs::validate_spawn_template, spawn_span_is_permitted and line_carries_forbidden_marker",
            "crates/cli/src/adapter.rs::SpawnTemplateReason — the route is its `Display`",
        ],
        causes: 7,
        derivation: "`validate_spawn_template` checks seven clauses in order and returns the \
                     first that fails, one reason a clause; the reason's words are the whole \
                     route",
        without_case: &[],
        not_followed: &[],
    },
    Reading {
        site: GUIDE_TARGET,
        read: &[
            "crates/cli/src/setup.rs::install",
            "crates/cli/src/adapter.rs::validate_guide_target",
            "crates/cli/src/adapter.rs::GuideTargetReason — the route is its `Display`",
        ],
        causes: 5,
        derivation: "`validate_guide_target` returns one of five reasons: no path, an \
                     absolute one, one with a `..` component, a front-matter key that is one \
                     of jigc's stamp keys, a key that is no plain YAML key",
        without_case: &[],
        not_followed: &[],
    },
    Reading {
        site: COMMIT_REFUSED,
        read: &[
            "crates/cli/src/setup.rs::InstallCommitRejection::finding — which step refused, and which of two routes",
            "crates/cli/src/setup.rs::is_git_identity_rejection — the three wordings of git's that choose the identity route",
            "crates/cli/src/setup.rs::commit_install — where each rejection is raised",
            "crates/cli/src/setup.rs::stage_paths",
        ],
        causes: 5,
        derivation: "the finding is built for a refused `git add` or a refused `git commit` \
                     (the two other rejections carry findings of their own). A refused \
                     commit takes the identity route where git's words match one of three \
                     wordings — *tell me who you are*, *unable to auto-detect email \
                     address*, *empty ident name* — and the other route where they match \
                     none; a refused `git add` never speaks of identity. One staging cause, \
                     three identity causes, one other",
        without_case: &[(
            "`git commit` refuses because git cannot make up an address for this machine",
            Why::Platform(
                "git says *unable to auto-detect email address* where it is allowed to \
                 invent an identity and the machine gives it none to invent one from — a \
                 host with no name it can qualify. Whether that is so is the machine's and \
                 not the sandbox's: the mask forbids the invention outright, and git then \
                 words the refusal the first way",
            ),
        )],
        not_followed: &[
            (
                "a refused `git add` over another refusal than a locked index — a path a \
                 sparse checkout excludes, a submodule that is registered and absent",
                Why::NotConstructible(
                    "one arm, followed over one refusal: `finding` words the route the same \
                     whatever git said, and what resolves the refusal is in git's words, \
                     never in the route — a second state would follow the same route by \
                     another act of the case's own",
                ),
            ),
            (
                "a refused `git commit` whose cause is a setting and no file — a signing \
                 key git cannot use",
                Why::NotConstructible(
                    "the route says *resolve the refusal* and prints no command for it; \
                     the frame has no act that changes a setting, and a `git config` line \
                     of the case's own would be a command the route did not print",
                ),
            ),
        ],
    },
    Reading {
        site: SETTINGS_ASKED,
        read: &[
            "crates/cli/src/setup.rs::install — where the question stands: behind the dirty gate, before the first write",
            "crates/cli/src/adapter.rs::check_settings_merge and read_settings",
            "crates/cli/src/adapter.rs::merge_allowlist, merge_hook and merge_deny — the shapes each refuses",
            "crates/cli/src/setup.rs::InstallSubject and DirtyPaths::rides_the_first_commit — which states of the file reach the question",
        ],
        causes: 9,
        derivation: "`read_settings` fails three ways — the read fails over the file's bytes \
                     (not UTF-8), the read fails any other way, the text is no JSON — and the \
                     three merges refuse six shapes between them: a root that is no object; \
                     `permissions`, `permissions.allow`, `hooks`, `hooks.<event>` and \
                     `permissions.deny` of the wrong kind (the later merges' checks of the \
                     root and of `permissions` cannot fail once the first merge passed). \
                     Eight of the nine draw one route, *fix it … commit the fix*; the read \
                     that fails otherwise draws *ensure it is readable*",
        without_case: &[],
        not_followed: &[
            (
                "the settings file changed and not committed, on a born `HEAD`",
                Why::NotConstructible(
                    "the dirty gate stands before this question and refuses the file as \
                     work in no commit, under `setup.dirty-install-path`",
                ),
            ),
            (
                "the read fails because the caller may not read the file",
                Why::NotConstructible(
                    "git cannot read it either and reports it changed: the dirty gate \
                     refuses first — driven, a committed settings file of mode 000. The \
                     arm is followed where a directory stands at the path, which git does \
                     not see",
                ),
            ),
        ],
    },
    Reading {
        site: MARKER_WRITE,
        read: &[
            "crates/cli/src/setup.rs::write_install_span — the step, and the three before it in the span",
            "crates/cli/src/setup.rs::write_compose_marker and compose_marker_rendering",
            "crates/cli/src/regular_file.rs::replace and open_no_follow — the write is in place: the file is opened, cut and written",
            SHARED[2],
        ],
        causes: 9,
        derivation: "`write_compose_marker` fails in four places. The entry is in the way (a \
                     link, a directory, a special file). The read fails otherwise than on \
                     an absent file. The rendering fails: the text is no YAML, is no \
                     mapping, or the mapping cannot be written back. The write fails: the \
                     directory cannot be made, the file cannot be opened for writing — \
                     because the file may not be written, or because there is none and the \
                     directory may not be — or the bytes cannot be written. Nine; every one \
                     draws the one route",
        without_case: &[
            (
                "an entry is in the way of `packs.yaml`",
                Why::NotConstructible(
                    "`replaced_path_refusal` asks the same question before the first write \
                     and answers under this code at its own site — the cases \
                     `setup-compose-marker-a-…`",
                ),
            ),
            (
                "the rendered mapping cannot be written back as YAML",
                Why::NotConstructible(
                    "what is written back is the mapping that was read, with one key set: \
                     a mapping YAML gave serializes",
                ),
            ),
            (
                "`.jigc/config/` cannot be made",
                Why::NotConstructible(
                    "the step before this one writes `.jigc/config/.gitkeep`, so the \
                     directory stands or that step has refused under \
                     `setup.init-project-layer`",
                ),
            ),
            (
                "the file is opened and its bytes cannot be written",
                Why::NotConstructible(
                    "a write that fails on an open file is a full disk or a failing one: no \
                     state of a sandbox",
                ),
            ),
        ],
        not_followed: &[
            (
                "`packs.yaml` in no commit — untracked, or changed — on a born `HEAD` and on \
                 an unborn one",
                Why::NotConstructible(
                    "the file is one the install replaces, so the dirty gate refuses it \
                     before the first write wherever git does not hold its bytes — driven \
                     on an unborn `HEAD`, `setup.dirty-install-path`",
                ),
            ),
            (
                "the read fails because the caller may not read the file",
                Why::NotConstructible(
                    "git cannot read it either and reports it changed: the dirty gate \
                     refuses first, as for the settings file",
                ),
            ),
        ],
    },
];
