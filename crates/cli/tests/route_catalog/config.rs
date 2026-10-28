//! The route catalog's cases for the family the list of refusal sites calls `config`: the
//! refusals of the `jigc config` verbs. The family's table, declared by
//! `crates/cli/tests/route_catalog.rs`, which says what a case is held to.
//!
//! **What stands here** is not the family: it is the five sites of it that the fix pass
//! since the previous release touched, as the human ruled on 2026-10-11 (`DECISIONS.md` →
//! *Which families the route catalog takes in*) — the rest are rows of the scheduled list.
//! All five are `jigc config set`'s: the four refusals at its door — a value the knob's
//! type does not take, and for the two root knobs a root git cannot record, a root inside
//! jigc's own workbench, and a root the store cannot describe — and the refusal behind
//! them, where a re-point that began could not be completed and was undone.
//!
//! **The three root refusals name one act and print no line for it** — *re-run with a
//! root …* — and beside it cite a command for what it prints: `jigc config list`. The
//! root is the reader's own and an argument of the refused command; the frame runs a
//! refusing command again only as it was typed, and only where a route prints no command
//! line. So a case runs the one line the route prints — cited for what it prints, and held
//! to its words (the human's ruling of 2026-10-12): the knob's value in force and the layer
//! it wins from, on the knob's own line ([`in_force`]). The re-run under another root is a
//! way with a reason in [`READINGS`], driven by hand.
//!
//! **And the re-point's refusal says *fix what this message names*.** What that is stands
//! in the refusal's message and not in its route, so the object is the case's, in the
//! marked form — a file moved out of a path, a lock removed, a manifest mended — and the
//! case comes out no better than *holds, operand implied*. Where the route itself prints a
//! path — the file it parked on its way — the frame refuses the marked form, and the
//! cause has a reason and no case. **And where the doc's own home is a link, the cause it
//! carries is the store's whole refusal, route and all**: the finding then prints two
//! `route:` lines, the frame lifts the first, and the case follows that one — the file
//! itself put in the link's place, committed, and the refused command again.

use super::{Reading, Reason, Why};
use crate::support::route_case::{
    Act, Case, Check, Cited, Named, Operand, Place, Promise, Says, Step, Word,
};

// The sites, as the committed list of refusal sites names them.
const REPOINT_FAILED: &str = "crates/cli/src/config.rs::repoint_failed::config.repoint-failed#1";
const UNTRACKABLE_ROOT: &str = "crates/cli/src/config.rs::run_set::config.untrackable-root#1";
const UNUSABLE_ROOT: &str = "crates/cli/src/config.rs::run_set::config.unusable-root#1";
const VALUE_REJECTED: &str = "crates/cli/src/config.rs::run_set::config.value-rejected#1";
const WORKBENCH_ROOT: &str = "crates/cli/src/config.rs::run_set::config.workbench-root#1";

const SETUP: Step = Step::Jigc(&["setup"]);

/// The project layer's manifest, from the repository root — where a knob that is set
/// lands. A refused `jigc config set` leaves it as it was: absent, after an install.
const MANIFEST: &str = ".jigc/config/manifest.yaml";

/// What every case of the family shares: it stands at the repository's root.
const C: Case = Case {
    name: "",
    site: "",
    cause: "",
    state: &[],
    refusing: &[],
    from: Place::Repo(""),
    env: &[],
    acts: &[],
    holds: &[],
    witnesses: &[],
    untouched: &[],
    doubt: None,
    over: None,
};

// ---------------------------------------------------------------------------
// `run_set` · `config.untrackable-root` · `config.workbench-root` ·
// `config.unusable-root` — a root knob's value refused at the door
// ---------------------------------------------------------------------------

/// **What the route promises of the list it cites** — *`jigc config list` shows the value
/// in force and the layer it wins from* — for the knob the refused command named: after a
/// set that was refused, the pack's default and the layer `pack-default`, on that knob's
/// line and no other's.
macro_rules! in_force {
    ("docs-root") => {
        &[Promise::Line(&[Says::Text(
            "docs-root = docs/  (pack-default)",
        )])]
    };
    ("placement-root") => {
        &[Promise::Line(&[Says::Text(
            "placement-root =   (pack-default)",
        )])]
    };
}

/// **A root refused at the door, followed the one way the frame can follow it**: the
/// command the route cites, run as printed — its span by index, which differs by refusal —
/// and held to what the route says it shows.
macro_rules! root {
    ($site:expr, $name:literal, $cause:literal, state: $state:expr, $knob:tt, $value:expr, list: $list:literal $(,)?) => {
        [Case {
            name: concat!($name, "-config-list-read-as-printed"),
            site: $site,
            cause: $cause,
            state: $state,
            refusing: &["config", "set", $knob, $value],
            acts: &[Act::RunCited(Cited {
                line: $list,
                promised: in_force!($knob),
            })],
            untouched: &[MANIFEST],
            ..C
        }]
    };
}

const A_ROOT_ABOVE_THE_REPOSITORY: [Case; 1] = root!(
    UNTRACKABLE_ROOT,
    "untrackable-root-above-the-repository",
    "the root climbs out of the repository: it resolves outside the repository root",
    state: &[SETUP],
    "docs-root",
    "../elsewhere",
    list: 1,
);
const A_ROOT_IN_GITS_DIRECTORY: [Case; 2] = [
    root!(
        UNTRACKABLE_ROOT,
        "untrackable-root-in-gits-directory",
        "the root has a `.git` component: git refuses to track any path under one",
        state: &[SETUP],
        "docs-root",
        ".git/docs",
        list: 1,
    )[0],
    root!(
        UNTRACKABLE_ROOT,
        "untrackable-root-in-gits-directory-the-placement-knob",
        "the root has a `.git` component: git refuses to track any path under one",
        state: &[SETUP],
        "placement-root",
        ".git",
        list: 1,
    )[0],
];
const A_ROOT_IN_A_GIT_DIRECTORY_OF_ANOTHER_NAME: [Case; 1] = root!(
    UNTRACKABLE_ROOT,
    "untrackable-root-in-a-git-directory-of-another-name",
    "the repository keeps its git directory under another name, inside the work tree, and \
     the root lies in it",
    state: &[
        Step::Git(&["init", "-q", "--separate-git-dir", "gitdata"]),
        SETUP,
    ],
    "docs-root",
    "gitdata/docs",
    list: 1,
);
const A_ROOT_IN_ANOTHER_REPOSITORY: [Case; 1] = root!(
    UNTRACKABLE_ROOT,
    "untrackable-root-in-another-repository",
    "the root lies in a directory that is a repository of its own — an embedded one, whose \
     paths this index cannot record",
    state: &[
        SETUP,
        Step::Git(&["init", "-q", "vendor"]),
    ],
    "docs-root",
    "vendor/docs",
    list: 1,
);

const A_ROOT_IN_THE_WORKBENCH: [Case; 1] = root!(
    WORKBENCH_ROOT,
    "workbench-root-typed",
    "the root is typed inside jigc's own workbench",
    state: &[SETUP],
    "docs-root",
    ".jigc/docs",
    list: 1,
);
const A_ROOT_THAT_REACHES_THE_WORKBENCH: [Case; 1] = root!(
    WORKBENCH_ROOT,
    "workbench-root-reached-by-a-hop",
    "the root reaches jigc's own workbench only once its `..` hop is applied",
    state: &[SETUP],
    "placement-root",
    "docs/../.jigc",
    list: 1,
);

const AN_ABSOLUTE_ROOT: [Case; 1] = root!(
    UNUSABLE_ROOT,
    "unusable-root-absolute",
    "the root is an absolute path",
    state: &[SETUP],
    "docs-root",
    "/elsewhere/docs",
    list: 5,
);
const A_ROOT_PADDED_WITH_WHITESPACE: [Case; 1] = root!(
    UNUSABLE_ROOT,
    "unusable-root-padded-with-whitespace",
    "a component of the root carries whitespace at its edge",
    state: &[SETUP],
    "docs-root",
    " docs",
    list: 5,
);
const A_ROOT_GIT_READS_AS_PATHSPEC_MAGIC: [Case; 1] = root!(
    UNUSABLE_ROOT,
    "unusable-root-a-leading-colon",
    "the root begins with `:`, which git reads as pathspec magic and not as a name",
    state: &[SETUP],
    "docs-root",
    ":docs",
    list: 5,
);
const A_ROOT_GIT_READS_AS_A_PATTERN: [Case; 1] = root!(
    UNUSABLE_ROOT,
    "unusable-root-a-wildmatch-character",
    "the root contains `*`, which git reads as a pattern and not as part of a name",
    state: &[SETUP],
    "placement-root",
    "do*cs",
    list: 5,
);
/// A component of 300 bytes — over the 255 a path component may carry.
const A_COMPONENT_OF_300_BYTES: &str = concat!(
    "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
    "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
    "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
    "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
    "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
);
const A_ROOT_OVER_THE_NAME_CEILING: [Case; 1] = root!(
    UNUSABLE_ROOT,
    "unusable-root-a-component-over-the-name-ceiling",
    "a component of the root is longer than a path component may be",
    state: &[SETUP],
    "docs-root",
    A_COMPONENT_OF_300_BYTES,
    list: 5,
);
// A component of the root that is a symbolic link is no cause of this site since
// 2026-10-28: it is the link-on-the-way state, refused under `store.link-on-the-way` ahead
// of this check, and its case stands with that site's (`doc_read.rs`).
const A_ROOT_THAT_IS_A_FILE: [Case; 1] = root!(
    UNUSABLE_ROOT,
    "unusable-root-a-file",
    "the root names a file, not a directory",
    state: &[SETUP],
    "docs-root",
    "README.md",
    list: 5,
);
const A_ROOT_UNDER_THE_INSTALLS_OWN_ARTIFACTS: [Case; 1] = root!(
    UNUSABLE_ROOT,
    "unusable-root-under-the-installs-own-artifacts",
    "the root lies where jigc's own install writes the assistant adapter's artifacts",
    state: &[SETUP],
    "docs-root",
    ".claude/docs",
    list: 5,
);

// ---------------------------------------------------------------------------
// `repoint_failed` · `config.repoint-failed` — a re-point that began, could not be
// completed and was undone
// ---------------------------------------------------------------------------

/// The two writes a `commit` doc requires, for the task `$id`.
macro_rules! commit_doc {
    ($id:literal) => {
        [
            Step::Jigc(&[
                "doc",
                "set-field",
                concat!("commit:", $id, "#header/type"),
                "--task",
                $id,
                "--value",
                "docs",
            ]),
            Step::File(Place::Outside("summary"), "write a note\n"),
            Step::Jigc(&[
                "doc",
                "set-slot",
                concat!("commit:", $id, "#summary"),
                "--task",
                $id,
                "--from-file",
                "../outside/summary",
            ]),
        ]
    };
}
const THE_ADR_TASKS_COMMIT_DOC: [Step; 3] = commit_doc!("pick-tabs");
const THE_LOG_TASKS_COMMIT_DOC: [Step; 3] = commit_doc!("write-a-note");

/// **An install and an ADR an earlier task committed** — `docs/decisions/use-tabs.md`, a
/// doc under a `location:` directory, which a `docs-root` re-point moves — and what the
/// case plants.
macro_rules! a_committed_adr {
    ($($plant:expr),* $(,)?) => {
        &[
            SETUP,
            Step::Jigc(&["start", "--workflow", "record-decision", "pick tabs"]),
            Step::File(
                Place::Outside("adr.yaml"),
                "title: \"Use tabs\"\nsections:\n  - id: context\n    set:\n      context: \
                 |-\n        <<Because.>>\n  - id: decision\n    set:\n      decision: |-\n        \
                 <<Tabs.>>\n  - id: consequences\n    set:\n      consequences: |-\n        \
                 <<None.>>\n",
            ),
            Step::Jigc(&[
                "doc",
                "author",
                "adr",
                "--from-file",
                "../outside/adr.yaml",
                "--task",
                "pick-tabs",
            ]),
            THE_ADR_TASKS_COMMIT_DOC[0],
            THE_ADR_TASKS_COMMIT_DOC[1],
            THE_ADR_TASKS_COMMIT_DOC[2],
            Step::Jigc(&["task", "finalize", "pick-tabs"]),
            $($plant,)*
        ]
    };
}
/// **An install and the decisions log an earlier task committed** — `docs/decisions-log.md`,
/// a doc at a placement home, which a `placement-root` re-point moves — and what the case
/// plants.
macro_rules! a_committed_log {
    ($($plant:expr),* $(,)?) => {
        &[
            SETUP,
            Step::Jigc(&["start", "--workflow", "decided-task", "write a note"]),
            THE_LOG_TASKS_COMMIT_DOC[0],
            THE_LOG_TASKS_COMMIT_DOC[1],
            THE_LOG_TASKS_COMMIT_DOC[2],
            Step::Jigc(&[
                "doc",
                "create",
                "decisions-log",
                "--title",
                "Decisions Log",
                "--task",
                "write-a-note",
            ]),
            Step::Jigc(&["task", "finalize", "write-a-note"]),
            $($plant,)*
        ]
    };
}
const THE_ADR: &str = "docs/decisions/use-tabs.md";
const THE_ADR_MOVED: &str = "handbook/decisions/use-tabs.md";
const THE_LOG: &str = "docs/decisions-log.md";
const THE_LOG_MOVED: &str = "handbook/decisions-log.md";
const DOCS_ROOT: &[&str] = &["config", "set", "docs-root", "handbook"];
const PLACEMENT_ROOT: &[&str] = &["config", "set", "placement-root", "handbook"];
/// What must hold once a `docs-root` re-point has gone through: the ADR at its new home
/// and gone from its prior one, the knob landed.
const THE_ADR_AT_ITS_NEW_HOME: &[Check] = &[
    Check::RegularFile(Place::Repo(THE_ADR_MOVED)),
    Check::Absent(Place::Repo(THE_ADR)),
    Check::Contains(Place::Repo(MANIFEST), "docs-root: handbook"),
];
const THE_LOG_AT_ITS_NEW_HOME: &[Check] = &[
    Check::RegularFile(Place::Repo(THE_LOG_MOVED)),
    Check::Absent(Place::Repo(THE_LOG)),
    Check::Contains(Place::Repo(MANIFEST), "placement-root: handbook"),
];
/// A shell line a `git` alias runs at the repository's root — a state has no step that
/// removes.
macro_rules! sh {
    ($line:literal) => {
        Step::Git(&["-c", concat!("alias.plant=!", $line), "plant"])
    };
}

const A_FILE_WHERE_THE_NEW_HOMES_DIRECTORY_MUST_BE: [Case; 1] = [Case {
    name: "repoint-failed-a-file-where-the-new-homes-directory-must-be-moved-out-and-re-run-as-printed",
    site: REPOINT_FAILED,
    cause: "`docs-root`: a file stands where a directory of the new home must be made, so \
            the destination cannot be read before the move",
    state: a_committed_adr![Step::File(Place::Repo("handbook/decisions"), "a file\n")],
    refusing: DOCS_ROOT,
    acts: &[
        Act::MoveOut(Operand::Unprinted("handbook/decisions")),
        Act::Run(1),
    ],
    holds: THE_ADR_AT_ITS_NEW_HOME,
    ..C
}];
const A_FILE_AT_THE_DOCS_NEW_HOME: [Case; 1] = [Case {
    name: "repoint-failed-a-file-at-the-docs-new-home-moved-out-and-re-run-as-printed",
    site: REPOINT_FAILED,
    cause: "`docs-root`: a file somebody wrote stands at the doc's new home, and `git mv` \
            refuses a destination that exists",
    state: a_committed_adr![Step::File(Place::Repo(THE_ADR_MOVED), "mine\n")],
    refusing: DOCS_ROOT,
    acts: &[Act::MoveOut(Operand::Unprinted(THE_ADR_MOVED)), Act::Run(1)],
    holds: THE_ADR_AT_ITS_NEW_HOME,
    ..C
}];
const GITS_INDEX_LOCKED_AT_THE_MOVE: [Case; 1] = [Case {
    name: "repoint-failed-a-stale-index-lock-at-the-move-removed-and-re-run-as-printed",
    site: REPOINT_FAILED,
    cause: "`docs-root`: a stale `.git/index.lock` stands, so `git mv` cannot stage the move",
    state: a_committed_adr![Step::File(Place::Repo(".git/index.lock"), "")],
    refusing: DOCS_ROOT,
    acts: &[
        Act::Remove(Operand::Unprinted(".git/index.lock")),
        Act::Run(1),
    ],
    holds: THE_ADR_AT_ITS_NEW_HOME,
    ..C
}];
const THE_MANIFEST_A_DIRECTORY: [Case; 1] = [Case {
    name: "repoint-failed-the-manifest-a-directory-moved-away-and-re-run-as-printed",
    site: REPOINT_FAILED,
    cause: "a directory stands where the project layer's manifest is read and written, so \
            the knob cannot land",
    state: a_committed_adr![Step::Dir(Place::Repo(MANIFEST))],
    refusing: DOCS_ROOT,
    acts: &[Act::MoveAway(Operand::Unprinted(MANIFEST)), Act::Run(1)],
    holds: THE_ADR_AT_ITS_NEW_HOME,
    ..C
}];
const THE_MANIFEST_NOT_YAML: [Case; 1] = [Case {
    name: "repoint-failed-the-manifest-not-yaml-mended-and-re-run-as-printed",
    site: REPOINT_FAILED,
    cause: "the project layer's manifest holds text that is no YAML, so the knob cannot land",
    state: a_committed_log![Step::File(Place::Repo(MANIFEST), ": : [\n")],
    refusing: PLACEMENT_ROOT,
    acts: &[Act::Mend(Operand::Unprinted(MANIFEST)), Act::Run(1)],
    holds: THE_LOG_AT_ITS_NEW_HOME,
    ..C
}];
const GITS_INDEX_LOCKED_AT_THE_DISPLACEMENT: [Case; 1] = [Case {
    name: "repoint-failed-a-stale-index-lock-over-a-file-in-the-way-removed-and-re-run-as-printed",
    site: REPOINT_FAILED,
    cause: "`placement-root`: a file somebody wrote stands at the doc's new home and a stale \
            `.git/index.lock` stands, so the file cannot be moved out of the way",
    state: a_committed_log![
        Step::File(Place::Repo(THE_LOG_MOVED), "mine\n"),
        Step::File(Place::Repo(".git/index.lock"), ""),
    ],
    refusing: PLACEMENT_ROOT,
    acts: &[
        Act::Remove(Operand::Unprinted(".git/index.lock")),
        Act::Run(1),
    ],
    holds: &[
        Check::RegularFile(Place::Repo(THE_LOG_MOVED)),
        Check::Absent(Place::Repo(THE_LOG)),
        Check::Contains(Place::Repo(MANIFEST), "placement-root: handbook"),
        Check::Contains(Place::Repo(".jigc/displaced/decisions-log.md"), "mine"),
    ],
    ..C
}];
const THE_DOC_GONE_FROM_THE_WORKING_TREE: [Case; 1] = [Case {
    name: "repoint-failed-the-doc-gone-from-the-working-tree-brought-back-and-re-run-as-printed",
    site: REPOINT_FAILED,
    cause: "`placement-root`: the committed doc is gone from the working tree — deleted, the \
            deletion in no commit — so the stranded doc cannot be read",
    state: a_committed_log![sh!("rm docs/decisions-log.md")],
    refusing: PLACEMENT_ROOT,
    acts: &[Act::Revert(Operand::Unprinted(THE_LOG)), Act::Run(1)],
    holds: THE_LOG_AT_ITS_NEW_HOME,
    ..C
}];

/// **The doc's own home is a link.** The cause the re-point's refusal carries is then the
/// store's whole refusal — its message and, on a line of its own, its route — and the
/// route the frame lifts from the finding is that one, the first `route:` line under it:
/// *put the doc itself at `<home>` as a regular file — for a link, a copy of the file it
/// points at, in the link's place — and commit that; then re-run this command*. It prints
/// no command line, so the re-run is the refused command, by the runner.
macro_rules! the_home_a_link {
    ($name:literal, $cause:literal, state: $state:expr, $refusing:expr, holds: $holds:expr $(,)?) => {
        [Case {
            name: concat!(
                "repoint-failed-",
                $name,
                "-the-file-itself-put-there-committed-and-re-run"
            ),
            site: REPOINT_FAILED,
            cause: $cause,
            state: $state,
            refusing: $refusing,
            acts: &[
                Act::PutTheFileItself(Operand::Span(0)),
                Act::Commit(Operand::Span(0)),
                Act::Rerun,
            ],
            holds: $holds,
            witnesses: &["copy.md"],
            ..C
        }]
    };
}
const THE_ADRS_HOME_A_LINK: [Case; 1] = the_home_a_link!(
    "the-docs-home-a-link-at-the-docs-root",
    "`docs-root`: the doc's own home is a committed link to a copy of it, so the move \
     primitive refuses to carry it",
    state: a_committed_adr![
        sh!("mv docs/decisions/use-tabs.md ../outside/copy.md"),
        Step::Link(Place::Repo(THE_ADR), Place::Outside("copy.md")),
        Step::Git(&["add", "-A"]),
        Step::Git(&["commit", "-q", "-m", "the doc's home is a link"]),
    ],
    DOCS_ROOT,
    holds: THE_ADR_AT_ITS_NEW_HOME,
);
const THE_LOGS_HOME_A_LINK: [Case; 1] = the_home_a_link!(
    "the-docs-home-a-link-at-the-placement-root",
    "`placement-root`: the doc's own home is a committed link to a copy of it, so the \
     sweep refuses to carry it",
    state: a_committed_log![
        sh!("mv docs/decisions-log.md ../outside/copy.md"),
        Step::Link(Place::Repo(THE_LOG), Place::Outside("copy.md")),
        Step::Git(&["add", "-A"]),
        Step::Git(&["commit", "-q", "-m", "the doc's home is a link"]),
    ],
    PLACEMENT_ROOT,
    holds: THE_LOG_AT_ITS_NEW_HOME,
);

/// **A link somebody made stands at the doc's new home** — to a directory outside the
/// repository, and no key for it in the record. Until 2026-10-28 the sweep parked it in the
/// workbench as the entry it is, and every door after refused over the link jigc had put
/// there; it is now refused where it stands, under the store's one code for a home that is
/// no regular file, and the route the frame lifts is that refusal's: *move the link at
/// `<home>` out of the doc's home — it is yours, and jigc moves no link — …, then re-run
/// this command* (the review's S1; the re-review's NS5).
const A_LINK_AT_THE_DOCS_NEW_HOME: [Case; 1] = [Case {
    name: "repoint-failed-a-link-at-the-docs-new-home-moved-out-and-re-run",
    site: REPOINT_FAILED,
    cause: "`placement-root`: a symbolic link to a directory outside the repository stands \
            at the doc's new home, with no key for it in the record — jigc moves no link, \
            so the relocation refuses over it",
    state: a_committed_log![
        Step::Dir(Place::Outside("elsewhere")),
        Step::Dir(Place::Repo("handbook")),
        Step::Link(Place::Repo(THE_LOG_MOVED), Place::Outside("elsewhere")),
    ],
    refusing: PLACEMENT_ROOT,
    acts: &[Act::MoveLinkOut(Operand::Span(0)), Act::Rerun],
    holds: THE_LOG_AT_ITS_NEW_HOME,
    ..C
}];

/// **The record of what jigc manages holds a key at the new home, and something stands
/// there.** Made by a re-point that went through, then undone by hand: the doc moved back
/// with `git mv`, the knob's manifest removed, both committed — the record still holds the
/// key the re-point wrote. The sweep then reads the new home as a managed doc's, and the
/// cause it carries is the relocation's own refusal, message and route; the frame lifts
/// that route, the first under the finding.
macro_rules! a_stale_key_at_the_new_home {
    ($($plant:expr),* $(,)?) => {
        a_committed_log![
            Step::Jigc(PLACEMENT_ROOT),
            Step::Git(&["add", "-A"]),
            Step::Git(&["commit", "-q", "-m", "re-pointed"]),
            Step::Dir(Place::Repo("docs")),
            Step::Git(&["mv", THE_LOG_MOVED, THE_LOG]),
            sh!("rm .jigc/config/manifest.yaml"),
            Step::Git(&["add", "-A"]),
            Step::Git(&["commit", "-q", "-m", "moved back by hand"]),
            $($plant,)*
        ]
    };
}
/// **The relocation's route, followed by each verb it names** — *two managed docs claim
/// `handbook/decisions-log.md` — decide which one keeps the home, then `jigc rename` the
/// other onto a free identity or `jigc unmanage` it; `jigc doc list` names what is managed
/// there*. `jigc rename` and `jigc unmanage` name a command (the human's rulings of
/// 2026-10-12, `DECISIONS.md` → *A span that names a command without its operand is followed with the operand the route speaks of*); `jigc doc list` is a line, and is run as printed. A case a verb, the reader
/// choosing *the other* as the route lets them: the one doc `jigc doc list` names, for the
/// rename onto a free identity — and, for the unmanage, the entry at the home the route
/// prints, which is what else claims it. Each is then held to the outcome: the refusing
/// command, run again.
macro_rules! over_a_stale_key {
    ($name:literal, $cause:literal, $($plant:expr),+ $(,)?) => {
        [
            Case {
                name: concat!(
                    "repoint-failed-a-stale-key-and-",
                    $name,
                    "-at-the-new-home-what-is-managed-listed-and-the-rename-run-as-printed"
                ),
                site: REPOINT_FAILED,
                cause: $cause,
                state: a_stale_key_at_the_new_home![$($plant,)+],
                refusing: PLACEMENT_ROOT,
                acts: &[
                    Act::Run(3),
                    Act::RunNamed(Named {
                        name: 1,
                        on: &[
                            Word::Named("decisions-log:decisions-log"),
                            Word::Named("--to"),
                            Word::Named("Decisions Log Two"),
                        ],
                        within: None,
                        means: "*the other*, renamed *onto a free identity*: the one doc \
                                `jigc doc list` names as managed, by the id that list \
                                prints for it — the refusal lists a path and no doc — and \
                                a title of the reader's own that nothing answers to",
                    }),
                    Act::Rerun,
                ],
                holds: THE_LOG_AT_ITS_NEW_HOME,
                ..C
            },
            Case {
                name: concat!(
                    "repoint-failed-a-stale-key-and-",
                    $name,
                    "-at-the-new-home-what-is-managed-listed-and-the-home-unmanaged"
                ),
                site: REPOINT_FAILED,
                cause: $cause,
                state: a_stale_key_at_the_new_home![$($plant,)+],
                refusing: PLACEMENT_ROOT,
                acts: &[
                    Act::Run(3),
                    Act::RunNamed(Named {
                        name: 2,
                        on: &[Word::Span(0)],
                        within: None,
                        means: "*the other*, unmanaged: the doc `jigc doc list` names keeps \
                                the home it is being moved to, and what else claims that \
                                home is the entry at the path the route prints",
                    }),
                    Act::Rerun,
                ],
                holds: THE_LOG_AT_ITS_NEW_HOME,
                ..C
            },
        ]
    };
}
const A_FILE_AT_A_HOME_THE_RECORD_HOLDS: [Case; 2] = over_a_stale_key!(
    "a-file",
    "`placement-root`: the record holds a key for the new home, where a file somebody wrote \
     stands — the relocation reads it as a managed doc's",
    Step::File(Place::Repo(THE_LOG_MOVED), "mine\n"),
);
const A_LINK_AT_A_HOME_THE_RECORD_HOLDS: [Case; 2] = over_a_stale_key!(
    "a-link-to-nothing",
    "`placement-root`: the record holds a key for the new home, where a symbolic link that \
     leads nowhere stands — the relocation reads it as a managed doc's",
    Step::Dir(Place::Repo("handbook")),
    Step::LinkTo(Place::Repo(THE_LOG_MOVED), "nowhere"),
);

/// The cases of the family, a group a cause — and, within a group, a case a way out of the
/// cause's route.
pub const CASES: &[&[Case]] = &[
    &A_ROOT_ABOVE_THE_REPOSITORY,
    &A_ROOT_IN_GITS_DIRECTORY,
    &A_ROOT_IN_A_GIT_DIRECTORY_OF_ANOTHER_NAME,
    &A_ROOT_IN_ANOTHER_REPOSITORY,
    &A_ROOT_IN_THE_WORKBENCH,
    &A_ROOT_THAT_REACHES_THE_WORKBENCH,
    &AN_ABSOLUTE_ROOT,
    &A_ROOT_PADDED_WITH_WHITESPACE,
    &A_ROOT_GIT_READS_AS_PATHSPEC_MAGIC,
    &A_ROOT_GIT_READS_AS_A_PATTERN,
    &A_ROOT_OVER_THE_NAME_CEILING,
    &A_ROOT_THAT_IS_A_FILE,
    &A_ROOT_UNDER_THE_INSTALLS_OWN_ARTIFACTS,
    &A_FILE_WHERE_THE_NEW_HOMES_DIRECTORY_MUST_BE,
    &A_FILE_AT_THE_DOCS_NEW_HOME,
    &GITS_INDEX_LOCKED_AT_THE_MOVE,
    &THE_MANIFEST_A_DIRECTORY,
    &THE_MANIFEST_NOT_YAML,
    &GITS_INDEX_LOCKED_AT_THE_DISPLACEMENT,
    &THE_DOC_GONE_FROM_THE_WORKING_TREE,
    &THE_ADRS_HOME_A_LINK,
    &THE_LOGS_HOME_A_LINK,
    &A_FILE_AT_A_HOME_THE_RECORD_HOLDS,
    &A_LINK_AT_A_HOME_THE_RECORD_HOLDS,
    &A_LINK_AT_THE_DOCS_NEW_HOME,
];

/// The sites of the family that have no case, each with why.
pub const REASONS: &[Reason] = &[Reason {
    site: VALUE_REJECTED,
    why: Why::NotConstructible(
        "the state is planted by anyone — a value outside the knob's declared type — and \
         the route cannot be followed: *set a value matching the knob's declared type* \
         names one act, the refused command with another value, and prints neither a line \
         nor a value. The value is the reader's own, and the frame runs a refusing command \
         again only as it was typed, which is the refusal again. Driven by hand over an \
         enum knob and a bool knob: `jigc config set default-workflow \"\"` and `jigc \
         config set finalize.fan-out.squash maybe` are refused, exit 1; `jigc config set \
         default-workflow router` and `jigc config set finalize.fan-out.squash false` \
         exit 0",
    ),
}];

/// The re-run under another root, which the three root refusals ask for in words.
const ANOTHER_ROOT_IS_THE_READERS: &str = "the root is the reader's own and an argument of the \
    refused command, which the frame runs again only as it was typed and only where the \
    route prints no command line — this one prints `jigc config list`. Driven by hand at \
    the first cause of each of the three refusals: `jigc config set docs-root handbook` \
    exits 0 and the knob lands";

/// **What was read for each site, and how many causes it holds** — a hand count, with its
/// derivation; the causes that have no case, each with why; and the ways out of its route
/// that are not followed, each with why.
pub const READINGS: &[Reading] = &[
    Reading {
        site: UNTRACKABLE_ROOT,
        read: &[
            "crates/cli/src/config.rs::run_set — step 2b, for the two root knobs, on the folded value",
            "crates/cli/src/trackable.rs::untrackable_reason — the four reasons a root is one git cannot record",
            "crates/cli/src/config.rs::reject_repoint — the callee that changed since the previous release: one argument more to the re-point's refusal, at the function's end, behind this refusal",
        ],
        causes: 4,
        derivation: "one predicate, four reasons in its order: the root resolves outside \
                     the repository root; it has a `.git` component; it lies in this \
                     repository's git directory, where that has another name; it lies in \
                     another repository. Which of the two root knobs is set changes one \
                     word of the message: the second reason is followed at both",
        without_case: &[],
        not_followed: &[(
            "*re-run with a root git can record — a path under the repository root and outside `.git/`*",
            Why::NotConstructible(ANOTHER_ROOT_IS_THE_READERS),
        )],
    },
    Reading {
        site: WORKBENCH_ROOT,
        read: &[
            "crates/cli/src/config.rs::run_set — step 2c, for the two root knobs, on the folded value",
            "crates/cli/src/config.rs::is_workbench_root and first_normalized_component — the first component of the folded value, in any letter case",
        ],
        causes: 2,
        derivation: "one predicate — the first component of the folded value is the \
                     workbench — met by a root typed inside it, and by one that reaches it \
                     only once a `..` hop is applied; the message quotes what was typed",
        without_case: &[],
        not_followed: &[(
            "*re-run with a root outside `.jigc/`*",
            Why::NotConstructible(ANOTHER_ROOT_IS_THE_READERS),
        )],
    },
    Reading {
        site: UNUSABLE_ROOT,
        read: &[
            "crates/cli/src/config.rs::run_set — step 2d, for the two root knobs, on the folded value",
            "crates/cli/src/config.rs::unusable_root_reason — eight reasons, in its order, with the helpers each calls: whitespace_padded_component, pathspec_magic_root, over_name_ceiling_component, offending_component, installed_artifact_root",
        ],
        causes: 7,
        derivation: "one code and one route, a reason a shape: an absolute path; a \
                     component padded with whitespace; a leading `:`; a wildmatch \
                     character; a component over the name ceiling; a root that names a \
                     file; a root under a directory the install writes the adapter's \
                     artifacts in. A component that is a symbolic link was the eighth until \
                     2026-10-28: it is refused ahead of this check under \
                     `store.link-on-the-way`, the one code for the state",
        without_case: &[],
        not_followed: &[(
            "*re-run with a repo-relative directory — an existing one, or one jigc should create*",
            Why::NotConstructible(ANOTHER_ROOT_IS_THE_READERS),
        )],
    },
    Reading {
        site: REPOINT_FAILED,
        read: &[
            "crates/cli/src/config.rs::repoint_failed — the route, by whether the undo left a conflict and whether a file was parked on the way; and against the previous release, which had no clause for a parked file",
            "crates/cli/src/config.rs::reject_repoint and run_set's step 3 — the three producers of a cause: a doc that could not be moved, the sweep's own error, the knob's write",
            "crates/cli/src/config.rs::route_docs_root_repoint_orphans — the `docs-root` floor: the destination's capture and the move, a failure a doc",
            "crates/cli/src/config.rs::route_placement_root_repoint_strands, crates/cli/src/relocate.rs::relocate_stranded, relocate_one, displace_foreign_squatter, move_doc — the `placement-root` floor: the source's shape, the file in the way moved into the workbench, the read, the move",
            "crates/cli/src/config.rs::write_scalar — the knob's write: the manifest read, parsed, written",
        ],
        causes: 15,
        derivation: "the `docs-root` floor, a failure a doc: the destination cannot be \
                     captured — a file where a directory of the new home must be; the move \
                     fails — a file at the destination, git's index locked, the doc's own \
                     home no regular file — 4. The `placement-root` floor: the file in the \
                     way cannot be parked — git's index locked; the stranded doc cannot be \
                     read — gone from the working tree, with no file in the way and with \
                     one that was parked first; the doc's own home no regular file; the \
                     new home one the record holds a key for, with a file or with a link \
                     to nothing standing at it; the new home a link somebody made, with no \
                     key for it, which jigc moves no more (2026-10-28) — 7. The knob's write: the manifest a \
                     directory, no YAML, not writable — 3. And any of them where the undo \
                     could not put a path back — 1",
        without_case: &[
            (
                "the stranded doc is gone from the working tree, and a file in the way at its new home was parked first",
                Why::NotConstructible(
                    "the one cause whose route carries the clause the fix pass added, and \
                     it prints two paths — where the parked file stood and where it is \
                     now; the object to mend is the doc, which the message names and the \
                     route does not, and the frame refuses an object of a case's own \
                     wherever the route prints a path. Driven by hand: the refusal names \
                     the parked file both ways; with the doc brought back (`git checkout \
                     -- docs/decisions-log.md`) the printed re-run exits 0, the doc lands \
                     at its new home and the parked file stays in the workbench",
                ),
            ),
            (
                "the manifest cannot be written",
                Why::NotConstructible(
                    "a manifest that is read and parsed and then not written is one whose \
                     directory may not be written — a permission bit changed by hand, a \
                     declared bound of the catalog",
                ),
            ),
            (
                "the undo could not put a path back",
                Why::NotConstructible(
                    "a prior home that is taken again between the move and the undo, \
                     inside one command: a state has no step that writes between them",
                ),
            ),
        ],
        not_followed: &[(
            "*move one back if you want it where it stood* — the parked file",
            Why::NotConstructible(
                "the clause belongs to the cause that has no case; and the frame has no \
                 act that moves an entry to a path the route prints",
            ),
        )],
    },
];
