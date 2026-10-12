//! The route catalog's cases for the sites the list of refusal sites assigns to no
//! family: the *dynamic* ones, whose code is handed in and not spelled at the call. The
//! table for them, declared by `crates/cli/tests/route_catalog.rs`, which says what a case
//! is held to.
//!
//! **What stands here** is the three of them that the human's ruling of 2026-10-11 takes
//! in (`DECISIONS.md` → *Which families the route catalog takes in*), and of each only the
//! codes whose path the fix pass changed — a dynamic site words every refusal of its door,
//! a code a kind, and the catalog claims a code there one by one
//! (`route_catalog.rs` → `CLAIMS`, `IN_PART`): the site keeps its own row of the scheduled
//! list for the codes it mints that nothing changed.
//!
//!   * `jigc rename`'s `refuse` — *no such doc* and *the destination is taken*: both
//!     conditions are asked of the home's own entry since the fix pass, and no longer
//!     through a link. Cases.
//!   * `jigc relocate`'s `refuse` — *the destination is occupied by a managed instance*: a
//!     link to nothing at a home the record holds a key for reaches it now. A reason: no
//!     door of the shipped packs prints it as a finding of its own; it is followed where
//!     a re-point carries it, in `config.rs`.
//!   * `jigc migrate-corpus`'s `blocked_finding` — *the relocation's destination is
//!     taken*. A reason: it takes a pack of the reader's own.

use super::{Reading, Reason, Why};
use crate::support::route_case::{
    Act, Case, Check, Chosen, Cited, Own, Place, Promise, Says, Step,
};

// The sites, as a claim names each: the dynamic site's function, and the code minted there.
const RENAME_NO_SUCH_DOC: &str = "crates/cli/src/rename.rs::refuse::store.not-found#dynamic-1";
const RENAME_DESTINATION_TAKEN: &str =
    "crates/cli/src/rename.rs::refuse::write.already-present#dynamic-1";
const RELOCATE_DESTINATION_MANAGED: &str =
    "crates/cli/src/relocate.rs::refuse::write.already-present#dynamic-1";
const CORPUS_DESTINATION_COLLISION: &str =
    "crates/cli/src/migrate_corpus.rs::blocked_finding::migrate-corpus.destination-collision#dynamic-1";

const SETUP: Step = Step::Jigc(&["setup"]);

/// What every case here shares: `jigc rename` of the committed ADR, at the repository's
/// root.
const U: Case = Case {
    name: "",
    site: "",
    cause: "",
    state: &[],
    refusing: &["rename", "adr:use-tabs", "--to", "Use spaces"],
    from: Place::Repo(""),
    env: &[],
    acts: &[],
    holds: &[],
    witnesses: &[],
    untouched: &[],
    doubt: None,
    over: None,
};

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
/// An ADR whole in one payload, by its title.
macro_rules! an_adr_titled {
    ($file:literal, $title:literal) => {
        Step::File(
            Place::Outside($file),
            concat!(
                "title: \"",
                $title,
                "\"\nsections:\n  - id: context\n    set:\n      context: |-\n        \
                 <<Because.>>\n  - id: decision\n    set:\n      decision: |-\n        \
                 <<Tabs.>>\n  - id: consequences\n    set:\n      consequences: |-\n        \
                 <<None.>>\n"
            ),
        )
    };
}
const PICK_TABS_COMMIT_DOC: [Step; 3] = commit_doc!("pick-tabs");
const NEXT_COMMIT_DOC: [Step; 3] = commit_doc!("next");
/// **An install and an ADR titled *Use tabs* that an earlier task committed**, and what the
/// case plants.
macro_rules! an_adr_committed {
    ($($plant:expr),* $(,)?) => {
        &[
            SETUP,
            Step::Jigc(&["start", "--workflow", "record-decision", "pick tabs"]),
            an_adr_titled!("adr.yaml", "Use tabs"),
            Step::Jigc(&[
                "doc",
                "author",
                "adr",
                "--from-file",
                "../outside/adr.yaml",
                "--task",
                "pick-tabs",
            ]),
            PICK_TABS_COMMIT_DOC[0],
            PICK_TABS_COMMIT_DOC[1],
            PICK_TABS_COMMIT_DOC[2],
            Step::Jigc(&["task", "finalize", "pick-tabs"]),
            $($plant,)*
        ]
    };
}
const THE_ADR: &str = "docs/decisions/use-tabs.md";
const THE_DESTINATION: &str = "docs/decisions/use-spaces.md";

// ---------------------------------------------------------------------------
// `jigc rename` · `store.not-found` — no doc to rename at the id's home
// ---------------------------------------------------------------------------

/// What the route promises of the command it cites — *`jigc describe` lists the doctype
/// surface — check the id you typed against it*: the surface, with the doctype of the id
/// that was typed in it. Held to its words (the human's ruling of 2026-10-12): the line
/// that opens the doc-types `jigc describe` lists, with `adr`, the first of them, on it.
const THE_DOCTYPE_SURFACE_WITH_THE_IDS_DOCTYPE: &[Promise] = &[Promise::Line(&[Says::Text(
    "The doc-types you can author. adr is ",
)])];

const NO_DOC_OF_THAT_ID: [Case; 1] = [Case {
    name: "rename-not-found-no-doc-of-that-id-surface-read-as-printed",
    site: RENAME_NO_SUCH_DOC,
    cause: "the id names a doc the store does not hold: nothing stands at its home",
    state: an_adr_committed![],
    refusing: &["rename", "adr:no-such-doc", "--to", "Use spaces"],
    acts: &[Act::RunCited(Cited {
        line: 0,
        promised: THE_DOCTYPE_SURFACE_WITH_THE_IDS_DOCTYPE,
    })],
    untouched: &[THE_ADR],
    ..U
}];
const THE_DOC_GONE_FROM_THE_WORKING_TREE: [Case; 1] = [Case {
    name: "rename-not-found-the-doc-gone-from-the-working-tree-surface-read-as-printed",
    site: RENAME_NO_SUCH_DOC,
    cause: "the id is a committed doc's, and the doc is gone from the working tree — \
            deleted, the deletion in no commit",
    state: an_adr_committed![Step::Git(&[
        "-c",
        "alias.plant=!rm docs/decisions/use-tabs.md",
        "plant"
    ])],
    acts: &[Act::RunCited(Cited {
        line: 0,
        promised: THE_DOCTYPE_SURFACE_WITH_THE_IDS_DOCTYPE,
    })],
    holds: &[Check::Absent(Place::Repo(THE_DESTINATION))],
    ..U
}];

// ---------------------------------------------------------------------------
// `jigc rename` · `write.already-present` — a regular file at the destination
// ---------------------------------------------------------------------------

/// **Another slug, where the route says *give this doc an id nothing else answers to —
/// re-run `jigc rename … --slug <other-slug>`*** — the human's ruling of 2026-10-12
/// (`DECISIONS.md` → *A printed line that carries a placeholder for a value of the
/// reader's own is followed with the placeholder filled*): the printed line is followed
/// with the reader's own value in the placeholder. The title of the rename, *Use spaces*,
/// slugs to `use-spaces`, which is the id that is taken.
const THE_RENAME_UNDER_ANOTHER_SLUG: &[Act] = &[Act::RunChosen(Chosen {
    line: 0,
    own: &[Own {
        placeholder: "<other-slug>",
        value: "spaces",
        why: "an id nothing else answers to, as the route asks: other than `use-spaces`, \
              which the title slugs to and something else holds — no doc is `adr:spaces`, \
              and nothing stands at its home",
    }],
    promised: &[],
})];
/// What must hold behind it: the ADR at the home of the id that was named, under the
/// title of the rename, and gone from the home it had.
const RENAMED_UNDER_ANOTHER_SLUG: &[Check] = &[
    Check::RegularFile(Place::Repo("docs/decisions/spaces.md")),
    Check::Contains(Place::Repo("docs/decisions/spaces.md"), "# Use spaces"),
    Check::Absent(Place::Repo(THE_ADR)),
];

const ANOTHER_DOC_AT_THE_DESTINATION: &str = "the id the doc is renamed to is another \
    committed doc's";
macro_rules! another_adr_committed {
    () => {
        an_adr_committed![
            Step::Jigc(&["start", "--workflow", "record-decision", "next"]),
            an_adr_titled!("next.yaml", "Use spaces"),
            Step::Jigc(&[
                "doc",
                "author",
                "adr",
                "--from-file",
                "../outside/next.yaml",
                "--task",
                "next",
            ]),
            NEXT_COMMIT_DOC[0],
            NEXT_COMMIT_DOC[1],
            NEXT_COMMIT_DOC[2],
            Step::Jigc(&["task", "finalize", "next"]),
        ]
    };
}
const A_COMMITTED_DOC_AT_THE_DESTINATION: [Case; 2] = [
    Case {
        name: "rename-destination-taken-by-a-committed-doc-another-slug-line-run-as-printed",
        site: RENAME_DESTINATION_TAKEN,
        cause: ANOTHER_DOC_AT_THE_DESTINATION,
        state: another_adr_committed!(),
        acts: THE_RENAME_UNDER_ANOTHER_SLUG,
        holds: RENAMED_UNDER_ANOTHER_SLUG,
        untouched: &[THE_DESTINATION],
        ..U
    },
    Case {
        name: "rename-destination-taken-by-a-committed-doc-that-doc-read-as-printed",
        site: RENAME_DESTINATION_TAKEN,
        cause: ANOTHER_DOC_AT_THE_DESTINATION,
        state: another_adr_committed!(),
        acts: &[Act::Run(2)],
        untouched: &[THE_ADR, THE_DESTINATION],
        ..U
    },
];
const A_FILE_IN_NO_COMMIT_AT_THE_DESTINATION: &str = "a file somebody wrote, in no commit and \
    no doc, stands at the home of the id the doc is renamed to";
const A_FILE_BY_HAND_AT_THE_DESTINATION: [Case; 2] = [
    Case {
        name: "rename-destination-taken-by-a-file-in-no-commit-another-slug-line-run-as-printed",
        site: RENAME_DESTINATION_TAKEN,
        cause: A_FILE_IN_NO_COMMIT_AT_THE_DESTINATION,
        state: an_adr_committed![Step::File(Place::Repo(THE_DESTINATION), "mine\n")],
        acts: THE_RENAME_UNDER_ANOTHER_SLUG,
        holds: RENAMED_UNDER_ANOTHER_SLUG,
        untouched: &[THE_DESTINATION],
        ..U
    },
    Case {
        name: "rename-destination-taken-by-a-file-in-no-commit-that-doc-read-as-printed",
        site: RENAME_DESTINATION_TAKEN,
        cause: A_FILE_IN_NO_COMMIT_AT_THE_DESTINATION,
        state: an_adr_committed![Step::File(Place::Repo(THE_DESTINATION), "mine\n")],
        acts: &[Act::Run(2)],
        untouched: &[THE_ADR, THE_DESTINATION],
        ..U
    },
];

/// The cases, a group a cause — and, within a group, a case a way out of the cause's
/// route.
pub const CASES: &[&[Case]] = &[
    &NO_DOC_OF_THAT_ID,
    &THE_DOC_GONE_FROM_THE_WORKING_TREE,
    &A_COMMITTED_DOC_AT_THE_DESTINATION,
    &A_FILE_BY_HAND_AT_THE_DESTINATION,
];

/// The claimed sites that have no case, each with why.
pub const REASONS: &[Reason] = &[
    Reason {
        site: RELOCATE_DESTINATION_MANAGED,
        why: Why::NotConstructible(
            "the state is constructible — a home the record holds a key for, with a file \
             or a link to nothing standing at it — and no door of the shipped packs prints \
             this refusal as a finding of its own: `jigc relocate` moves the instances of \
             a doctype no manifest governs, which takes a pack of the reader's own, and a \
             `placement-root` re-point carries the refusal inside the message of \
             `config.repoint-failed`. It is followed there, in `config.rs`, over a file \
             and over a link to nothing",
        ),
    },
    Reason {
        site: CORPUS_DESTINATION_COLLISION,
        why: Why::NotConstructible(
            "it takes a doctype whose home moved at a version bump, with an instance \
             still at the prior home — which no shipped schema has pending: a pack of the \
             reader's own. And the door prints a refused doc as a row of its report, \
             which is no finding the frame reads a route from",
        ),
    },
];

/// **What was read for each claimed site, and how many causes it holds** — a hand count,
/// with its derivation, and the ways out of its route that are not followed.
pub const READINGS: &[Reading] = &[
    Reading {
        site: RENAME_NO_SUCH_DOC,
        read: &[
            "crates/cli/src/rename.rs::run — the question of the doc's own home, asked of the entry itself: free, a regular file, or an entry that is neither",
            "crates/cli/src/rename.rs::run against the previous release (`git show 91834b5e:crates/cli/src/rename.rs`) — the condition was `!is_file()`, which follows a link: a link to nothing and a directory at the home answered here, and are the store's refusal now",
            "crates/cli/src/rename.rs::refuse and RefusalKind — the door's one constructor, a code a kind",
        ],
        causes: 2,
        derivation: "one predicate — nothing stands at the id's home — met where the id \
                     is nobody's, and where it is a committed doc's whose file is gone from \
                     the working tree. **The changed line takes two causes away and brings \
                     none**: a link to nothing and a directory at the home were answered \
                     here by the previous release's binary and are answered by the store's \
                     site on this tree, where each has its case",
        without_case: &[],
        not_followed: &[(
            "*check the id you typed against it* — the refused command with another id",
            Why::NotConstructible(
                "the id is the reader's own, an argument of the refused command, which \
                 the frame runs again only as it was typed and only where the route \
                 prints no command line",
            ),
        )],
    },
    Reading {
        site: RENAME_DESTINATION_TAKEN,
        read: &[
            "crates/cli/src/rename.rs::run — arm (f), the destination's own entry: a regular file refuses here, an entry that is none under the store's code",
            "crates/cli/src/rename.rs::run against the previous release — the condition was `is_file()`, which follows a link: a link to a file at the destination answered here, and is the store's refusal now",
        ],
        causes: 2,
        derivation: "one predicate — a regular file at the destination's home — met by \
                     another committed doc and by a file somebody wrote, in no commit; the \
                     message calls both *a different doc*. **The changed line takes one \
                     cause away and brings none**: a link that leads to a file",
        without_case: &[],
        not_followed: &[(
            "*and rename that one instead*",
            Why::NotConstructible(
                "the route names the act and prints no line for it: the title to give \
                 the other doc is the reader's own",
            ),
        )],
    },
];
