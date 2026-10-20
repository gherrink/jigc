//! **The byte table** — one writing command between two pictures of a sandbox, and what
//! the difference is (the exit rule's first clause, `DECISIONS.md` → *2026-10-04 — The exit
//! rule, revised*; the plan is
//! `completions/artifacts/M55/stabilization-build/reshape/byte-table-plan.md`).
//!
//! The frame is [`support::byte_cell`](crate::support::byte_cell), which says what a
//! manifest, a row and a classification are, takes a manifest off a sandbox and runs a
//! cell. **The table has thirty rows — the four of the pilot's acceptance, `jigc task
//! discard`'s, `jigc ingest`'s and `jigc unmanage`'s, the doc family's eight, the seven
//! other leaves of the `milestone` family, the movers family's four (`rename`, `relocate`,
//! `migrate-corpus`, `config set`) and the mint family's four (`start`, `workflow`,
//! `migrate`, `task amend`) — and 739 cells: the acceptance's twelve, the task family's
//! seventy-one** (`jigc task finalize`, `jigc task discard`; four of them `task finalize`'s
//! spelling `doc-only`), **`milestone finalize`'s spelling `squash: false` four, the install
//! family's hundred and thirty-one** (`setup`, `uninstall`, `ingest`, `unmanage`), **the doc
//! family's hundred and thirty-nine** (its eight leaves), **the milestone family's record
//! doors' seventy** (`create`, `add-task`, `add-from-spec`), **its fan-out doors'
//! thirty-six** (`provision`, `execute`, `join`) **and its terminals' sixty-nine**
//! (`discard`, and `milestone finalize`'s plain spelling, beside the carryover gate's cell
//! under `squash: false`), **the movers family's seventy-six and eighty** (`rename`,
//! `migrate-corpus`; `relocate`, `config set`) **and the mint family's fifty-one**, of which
//! a hundred and eighty run on demand. This file is the plan's tasks `B1` to `B9`: the arms
//! that hold the comparison, the reader and the runner to their rules, the fences, and the
//! rows and the cells, a family a file under
//! `byte_table/` ([`FAMILIES`]).
//!
//! **A cell of the table names the refusal it expects** — the exit code, and words its
//! output holds — and one that refuses otherwise is *not the listed outcome*
//! ([`a_refusal_is_the_refusal_the_cell_names`]); **a red cell shows every class it is red
//! in** ([`a_red_cell_shows_every_class_it_is_red_in`]); and **a cell that turns on an open
//! question, or stands on a declared bound, carries a mark**, which the committed list of
//! readings names back ([`a_mark_names_a_question_the_list_of_readings_has`]).
//!
//! **The cells run in the arm of their prepared state** ([`ARMS`],
//! [`run_the_cells_of`]): the state is built once on the binary this suite was built
//! with, every cell runs on a copy, and **the expected-red list is exactly the cells that
//! came out red** ([`EXPECTED_RED`], [`differences`]) — a red cell no entry lists, a
//! listed cell that is not red, and a listed cell whose classification moved are each
//! named. The arms are pools as wide as the machine, so under the gate's runner they run
//! one at a time with the route catalog's
//! ([`the_arms_that_run_cells_run_one_at_a_time_under_the_gates_runner`]). And every cell
//! has its row, its arm, a name of its own and — where it names one — the test of
//! another suite that holds the same state, ordering by ordering
//! ([`every_cell_has_its_row_its_arm_and_a_name_of_its_own`]).
//!
//! **And the same cells run on a binary a variable names**
//! ([`the_cells_are_run_on_the_binary_a_variable_names`], the plan's task `B2e`) —
//! ignored, run by hand: every cell on the named binary and on this tree's, each on a
//! copy of the state that binary built, side by side with what each line of each row
//! excused and the cell's seconds in the frame. It is how the table is read on the
//! previous release, where the acceptance must show the losses the pilot was built to
//! catch. An arm of a prepared state also leaves its cells' seconds in the target
//! directory's `tmp/` ([`SECONDS`]), so that what a cell costs inside a gate can be read
//! after one.
//!
//! **The arms of the comparison** drive [`byte_cell::compare`] over manifests built as
//! data: a repository with one commit, changed by hand to what a command would have left.
//! Each was red against a comparison that declared its classifications and answered
//! *holds* to everything, and each names the change to the built comparison it catches
//! (`DECISIONS.md`, the entry of the task `B1`, has both tables). What they cannot hold is
//! the half that reads a sandbox.
//!
//! **The arms of that half** run cells on a **stand-in** — a script handed to the frame as
//! the binary under test, which does one known act between two manifests
//! ([`the_reader_reads_what_the_comparison_reads`], over [`ON_A_STAND_IN`]): that *held*
//! is asked through git's filters, that a registration whose directory is gone is read
//! from its admin files, that a modification time and a plain `git status` are in no
//! layer and one changed byte is, and each other layer the comparison reads. A command
//! that does not end is ended ([`a_command_that_does_not_end_is_ended`]). And a prepared
//! state brought into a sandbox is the state the frame's own steps would have made
//! ([`a_state_brought_in_and_the_state_made_in_the_sandbox_agree_in_what_can_agree`]), on
//! the binary this suite was built with — the one arm here that hands a real one in.
//!
//! **The quote fence** ([`a_lines_quote_is_held_to_its_docs_text`]): an allowance line
//! rests on a sentence of a design doc, quoted, and the quote is held to the doc's text
//! under the heading it names. It holds **every quotation of the committed record**
//! (`…/reshape/byte-table-acceptance-rows.md`) and every quote of a row; a row's quote
//! must be one the record has, its line's number one the record has, and **a line that
//! rests on no sentence is none** — so a design doc that moves under a line reddens here,
//! and a line cannot be written from what a command did.
//!
//! **The leaf fence** ([`every_write_leaf_has_a_row_or_a_scheduled_row`]): every `Write`
//! row of the product's own registry (`cli::cli::VERB_KINDS`) has exactly one answer — a
//! row of the table ([`rows`]) or a row of the scheduled list (`byte_table/scheduled.tsv`:
//! the leaves the pilot leaves for before the port, each with its reason). The owed list,
//! the leaves the pilot's tasks owed a row, was emptied by the last of them, `B9`, and went
//! with it. So a writing command the product gains tomorrow is on no list, and is named
//! until it has its answer.
//!
//! **The cell fence** ([`every_need_of_a_row_has_a_cell_or_a_scheduled_row`], the
//! plan's task `B3`): what a row needs a cell for is read off the row — its refusing cell,
//! its base cell in its home state and in each other prepared state, and a planted cell
//! for every line and every kind of plant that targets it
//! ([`byte_cell::aimed`](crate::support::byte_cell::aimed), which also gives the reason a
//! kind does not target a line, so that nobody types one) — and each need has exactly one
//! answer: a cell, or a line of `byte_table/scheduled-cells.tsv` with its reason. A cell
//! says which line
//! its plant is aimed at, and never a path. **A base cell in a state that is not its
//! leaf's home runs on demand** ([`the_cells_on_demand_are_run_and_printed`]), and has
//! its line in the committed table of that arm
//! ([`every_cell_on_demand_has_its_line_in_the_committed_table`]) — held by no gate.
//! **Every line of a row excuses a difference in some cell the gate runs, or is named**
//! ([`EXCUSED_IN`], [`EXCUSES_NOTHING`]), exactly, in the arm of every state. And **every
//! scheduled leaf is one the committed count holds untouched**
//! ([`every_scheduled_leaf_is_one_the_count_holds_untouched`]): the count of the write
//! leaves the fix pass touched, each read against its handler, is
//! `…/reshape/byte-table-leaves-touched-since-rc24.md`.
//!
//! **A prepared state is built on the binary handed in**
//! ([`a_prepared_state_is_built_on_the_binary_handed_in`], the plan's task `B2a`): the
//! table runs a cell on the state the binary under test built, so the builder of the
//! prepared states takes its binary as a value
//! ([`TrialCorpus::build_on`](crate::support::trial_corpus::TrialCorpus::build_on)).

use std::collections::BTreeSet;
use std::fs;
use std::os::unix::fs::PermissionsExt;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::time::Duration;

use cli::cli::{VERB_KINDS, VerbKind};

/// One family's file under `byte_table/`, as the suite reads it.
struct Family {
    /// Its rows, a leaf each.
    rows: &'static [Row],
    /// Its cells, a group after another.
    cells: &'static [&'static [byte_cell::Cell]],
}

#[path = "byte_table/doc.rs"]
mod doc;
#[path = "byte_table/install.rs"]
mod install;
#[path = "byte_table/milestone.rs"]
mod milestone;
#[path = "byte_table/mint.rs"]
mod mint;
#[path = "byte_table/movers.rs"]
mod movers;
#[path = "byte_table/task.rs"]
mod task;

/// **A family's file is wired in by its `#[path]` declaration above and its name in this
/// macro's one call, and nowhere else** — the declaration is the line the registration
/// fence reads (`test_target_registration.rs`), and the name makes the file's row of
/// [`FAMILIES`]; a file declared and not named is dead code, which the gate's `clippy` step
/// refuses. The declaration stands beside the macro and never inside it: rustfmt reads
/// no item a macro's tokens hold, so a file declared there is formatted by nothing and the
/// gate's `fmt` step passes over any diff in it (`tooling-tests/fmt_reach.rs`). The file
/// exports `ROWS` and `CELLS`. It makes no arm of its own: a cell runs in the arm of its
/// prepared state ([`ARMS`]).
macro_rules! families {
    ($($name:ident),* $(,)?) => {
        /// Every family's file, in the order named.
        const FAMILIES: &[Family] = &[$(Family {
            rows: $name::ROWS,
            cells: $name::CELLS,
        }),*];
    };
}

families![install, task, milestone, doc, movers, mint];

use crate::support::byte_cell::{
    self, Aim, Allows, Built, Checkout, Classification, Commit, Condition, Ended, Entry, Holding,
    Kind, Layer, Lead, Line, Listed, Manifest, Mark, May, Plant, Planted, Prepared, Quote, REPO,
    Refusal, Row, Seen, Staged, Stands, Unit, When, Where, blob_of,
};
use crate::support::route_case::{self, Place, Step};
use crate::support::scratch::ScratchDir;
use crate::support::trial_corpus::{State, TrialCorpus};

// ---------------------------------------------------------------------------
// The table — its rows, and the two lists beside them
// ---------------------------------------------------------------------------

/// **The rows of the table**, a leaf each: every family's, in the order declared.
fn rows() -> Vec<&'static Row> {
    FAMILIES.iter().flat_map(|family| family.rows).collect()
}

/// **A refusal a cell names**: the exit code, and words the command's output holds — the
/// finding's code, or a run of the sentence that says why.
const fn refuses(exit: i32, says: &'static str) -> Listed {
    Listed::RefusesWith(Refusal {
        exit,
        says,
        moved: &[],
    })
}

/// **A refusal of an acting sweep that moved what it could first** — the exit code, the
/// words, and each doc it moved, from its place to its home: the human's rulings of
/// 2026-10-20, R1 (*does all it can, then exits non-zero*) and the rows' record's §6, entry
/// 23 (`jigc relocate` *blocks that doc, moves the others*). A refusal of the whole sweep
/// is not this outcome ([`Refusal`]).
const fn refuses_having_moved(
    exit: i32,
    says: &'static str,
    moved: &'static [(&'static str, &'static str)],
) -> Listed {
    Listed::RefusesWith(Refusal { exit, says, moved })
}

/// A line of [`ON_A_REFUSAL`]: a cache at `place`, rebuilt, when the command refused.
const fn rebuilt_on_a_refusal(id: &'static str, place: Where, quotes: &'static [Quote]) -> Line {
    Line {
        id,
        allows: Allows::Files(place, May::Rebuilt),
        when: When::Refused,
        on_refusal: true,
        quotes,
        spelling: None,
    }
}

/// **What a refusal may do to a cache** — the lines `C1` to `C3` of the committed record
/// (its §4a), which stand in every row: the human's ruling of 2026-10-12 on the record's
/// first silence (`DECISIONS.md` → *A refusal may rebuild a cache*). The places are the
/// ones `design/storage.md` declares rebuildable derived state — the edge index, the
/// file↔state hashes, and a milestone's `base.json` and `tasks.json` — and no other. A
/// line here holds **when the command refused, and only then**; what a command that exits
/// 0 may write to a cache is its own row's to say.
const ON_A_REFUSAL: [Line; 4] = [
    rebuilt_on_a_refusal(
        "C1",
        Where::Under(".jigc/index"),
        &[
            Quote {
                doc: "design/storage.md",
                heading: "Derived caches",
                words: "The edge index and the file↔state hashes are **rebuildable from the committed docs**, so they are gitignored, never committed",
            },
            Quote {
                doc: "design/storage.md",
                heading: "Repository layout & state location",
                words: "gitignored — edge index (rebuildable cache)",
            },
        ],
    ),
    rebuilt_on_a_refusal(
        "C2",
        Where::Under(".jigc/state"),
        &[
            Quote {
                doc: "design/storage.md",
                heading: "Derived caches",
                words: "The edge index and the file↔state hashes are **rebuildable from the committed docs**, so they are gitignored, never committed",
            },
            Quote {
                doc: "design/storage.md",
                heading: "Repository layout & state location",
                words: "gitignored — file↔CLI-state hashes (rebuildable)",
            },
        ],
    ),
    rebuilt_on_a_refusal(
        "C3, the base pin",
        Where::Path(".jigc/milestones/*/base.json"),
        &[
            Quote {
                doc: "design/storage.md",
                heading: "Source of truth",
                words: "**demotes** that JSON to exactly a *rebuildable cache* (re-derived from the record when absent or stale)",
            },
            Quote {
                doc: "design/storage.md",
                heading: "Repository layout & state location",
                words: "gitignored — milestone WIP (merge staging, worktrees) + the {base,tasks}.json cache re-derived from the committed milestone-record (M39; team-ready-state.md)",
            },
        ],
    ),
    rebuilt_on_a_refusal(
        "C3, the task list",
        Where::Path(".jigc/milestones/*/tasks.json"),
        &[
            Quote {
                doc: "design/storage.md",
                heading: "Source of truth",
                words: "**demotes** that JSON to exactly a *rebuildable cache* (re-derived from the record when absent or stale)",
            },
            Quote {
                doc: "design/storage.md",
                heading: "Repository layout & state location",
                words: "gitignored — milestone WIP (merge staging, worktrees) + the {base,tasks}.json cache re-derived from the committed milestone-record (M39; team-ready-state.md)",
            },
        ],
    ),
];

/// **The edge index's file is the stamp's, and its next read rebuilds it** — the sentences
/// beside a finalize's own *"Invalidate the edge-index stamp"* that its lines `T5` and
/// `M10` rest on for the word [`May::Invalidated`]: the file the stamp is kept in, a cache
/// whose loss costs one rebuild, and the lifecycle site that leaves the rebuild to the next
/// read. The review of the task `B7`'s first part, its first finding.
const THE_INDEX_IS_REBUILT_BY_ITS_NEXT_READ: [Quote; 3] = [
    Quote {
        doc: "design/storage.md",
        heading: "Concurrent writers",
        words: "**`index/edges.json` — locked, not merged.** A decision, not an omission: the index is **stamp-rebuildable by construction** (a stamp mismatch rebuilds it from the committed `.md`s, per Derived caches above), so a lost delta costs one rebuild and never a wrong answer",
    },
    Quote {
        doc: "design/storage.md",
        heading: "Edge index lifecycle",
        words: "| **finalize commit** | phase 7 invalidates the stamp; next read rebuilds against the new HEAD ([finalize.md](finalize.md)) | best-effort; cache-stamp absorbs failure |",
    },
    Quote {
        doc: "design/storage.md",
        heading: "Edge index lifecycle",
        words: "only `finalize` writes them through (indirectly, via stamp invalidation → next read rebuilds against the new HEAD). This is what makes \"delete `.jigc/`'s gitignored caches, rebuild from `.md`s, nothing lost\" hold for the index too",
    },
];

/// **The project config layer, on its first commit** — the sentence both finalize rows
/// rest their config lines on, `T3, the config` and `M3, the config`, each under
/// [`When::FirstCommitOfItsPlace`], so that the two rows read it one way. The review of
/// the task `B7`'s second part, its first finding: the milestone's row had carried the
/// layer into any commit, and the task's into none.
const THE_CONFIG_LAYERS_FIRST_COMMIT: Quote = Quote {
    doc: "design/finalize.md",
    heading: "5. Stage",
    words: "on a first commit, the project config layer (`.jigc/config`, `.jigc/.gitignore`),",
};

/// **The scheduled list** — the write leaves the pilot leaves for before the port: one
/// file of data, a line a leaf with its reason.
const SCHEDULED: &str = include_str!("byte_table/scheduled.tsv");

/// The rows of a scheduled list: every line that is no comment and not empty, as a leaf —
/// its words, space-separated — and a reason, tab-separated.
fn scheduled(list: &'static str) -> Vec<(Vec<&'static str>, &'static str)> {
    list.lines()
        .filter(|line| !line.is_empty() && !line.starts_with('#'))
        .map(|line| {
            let (leaf, reason) = line.split_once('\t').unwrap_or((line, ""));
            (leaf.split(' ').collect(), reason)
        })
        .collect()
}

/// **Every way the two lists fail to answer for `leaves`, once each** — empty when every
/// write leaf has exactly one answer and every answer is a write leaf's.
fn unanswered(
    leaves: &[Vec<&str>],
    rows: &[&[&str]],
    scheduled: &[(Vec<&str>, &str)],
) -> Vec<String> {
    let mut out = Vec::new();
    let named = |leaf: &[&str]| format!("`jigc {}`", leaf.join(" "));
    for leaf in leaves {
        let answers: Vec<&str> = [
            (
                rows.iter().filter(|row| **row == leaf.as_slice()).count(),
                "a row of the table",
            ),
            (
                scheduled.iter().filter(|(row, _)| row == leaf).count(),
                "a row of the scheduled list",
            ),
        ]
        .into_iter()
        .flat_map(|(times, what)| std::iter::repeat_n(what, times))
        .collect();
        match answers.as_slice() {
            [_] => {}
            [] => out.push(format!(
                "{} writes, and has no row and no scheduled row",
                named(leaf)
            )),
            several => out.push(format!(
                "{} is answered more than once: {}",
                named(leaf),
                several.join(" and ")
            )),
        }
    }
    let listed = rows.iter().map(|row| (row.to_vec(), "the table")).chain(
        scheduled
            .iter()
            .map(|(row, _)| (row.clone(), "the scheduled list")),
    );
    for (leaf, list) in listed {
        if !leaves.contains(&leaf) {
            out.push(format!(
                "{} is on {list}, and the product's registry holds no write leaf of that name",
                named(&leaf)
            ));
        }
    }
    for (leaf, reason) in scheduled {
        if reason.trim().is_empty() {
            out.push(format!(
                "{} is on the scheduled list and does not say why it waits",
                named(leaf)
            ));
        }
    }
    out
}

/// The write leaves of the product's registry.
fn write_leaves() -> Vec<Vec<&'static str>> {
    VERB_KINDS
        .iter()
        .filter(|(_, kind)| *kind == VerbKind::Write)
        .map(|(leaf, _)| leaf.to_vec())
        .collect()
}

/// `said` is one line for each of `expected`, in order, and each line holds its two
/// pieces: what it is about, and what it says of it.
fn says(said: &[String], expected: &[(&str, &str)]) {
    assert_eq!(said.len(), expected.len(), "{said:#?}");
    for (line, (about, what)) in said.iter().zip(expected) {
        assert!(
            line.contains(about) && line.contains(what),
            "a line about `{about}` that says `{what}`: {said:#?}",
        );
    }
}

/// **Every write leaf of the product has exactly one answer**: a row of the table, or a row
/// of the scheduled list. The owed list of leaves whose rows the plan's tasks owed was
/// emptied by the last of them, the task `B9`, and went with it.
///
/// On a fixture: a write leaf nobody answers for is named; so is one answered twice, an
/// answer for a leaf the registry does not hold as a write, and a scheduled row with no
/// reason. Then this tree, against the lists this suite holds — the registry has 36 write
/// leaves, and none is unanswered — and this tree with one write leaf more than it has: **a
/// writing command the product gains tomorrow is on no list, and is named until it has its
/// answer**.
#[test]
fn every_write_leaf_has_a_row_or_a_scheduled_row() {
    let leaves = vec![vec!["setup"], vec!["doc", "create"], vec!["relocate"]];
    let (setup, create, relocate): (&[&str], &[&str], &[&str]) =
        (&["setup"], &["doc", "create"], &["relocate"]);
    let waits = |leaf: &[&'static str], why: &'static str| (leaf.to_vec(), why);
    says(
        &unanswered(&leaves, &[], &[]),
        &[
            ("`jigc setup`", "no row and no scheduled row"),
            ("`jigc doc create`", "no row and no scheduled row"),
            ("`jigc relocate`", "no row and no scheduled row"),
        ],
    );
    let in_order = (
        vec![setup, create],
        vec![waits(relocate, "the count holds it untouched")],
    );
    says(&unanswered(&leaves, &in_order.0, &in_order.1), &[]);
    says(
        &unanswered(
            &leaves,
            &in_order.0,
            &[
                waits(relocate, "the count holds it untouched"),
                waits(setup, "the count holds it untouched"),
            ],
        ),
        &[(
            "`jigc setup`",
            "a row of the table and a row of the scheduled list",
        )],
    );
    says(
        &unanswered(
            &leaves,
            &in_order.0,
            &[
                waits(relocate, "the count holds it untouched"),
                waits(&["doc", "show"], "-"),
            ],
        ),
        &[("`jigc doc show`", "no write leaf of that name")],
    );
    says(
        &unanswered(&leaves, &in_order.0, &[waits(relocate, " ")]),
        &[("`jigc relocate`", "does not say why it waits")],
    );

    let here = write_leaves();
    assert_eq!(
        here.len(),
        36,
        "the product's registry holds 36 write leaves — the number the pilot's plan counts \
         from; a leaf more or less is a row or a scheduled row to write or to take away, and \
         this number with it"
    );
    let rows: Vec<&[&str]> = rows().iter().map(|row| row.leaf).collect();
    assert_eq!(
        rows.len(),
        30,
        "the table has the four rows of the pilot's acceptance, `jigc task discard`'s, \
         `jigc ingest`'s and `jigc unmanage`'s, the doc family's eight, the milestone \
         family's seven beside `milestone finalize`, the movers family's four and the mint \
         family's four: {rows:?}"
    );
    let wrong = unanswered(&here, &rows, &scheduled(SCHEDULED));
    assert!(
        wrong.is_empty(),
        "the byte table does not answer for the product's write leaves as it must:\n\n  \
         {}\n\na write leaf has a row in a file under `byte_table/`, or a line of \
         `byte_table/scheduled.tsv` with its reason — exactly one of the two",
        wrong.join("\n  ")
    );

    // Tomorrow's leaf.
    let mut tomorrow = here;
    tomorrow.push(vec!["doc", "merge"]);
    says(
        &unanswered(&tomorrow, &rows, &scheduled(SCHEDULED)),
        &[("`jigc doc merge`", "no row and no scheduled row")],
    );
}

// ---------------------------------------------------------------------------
// The cell fence — what a row needs a cell for, and the count its lists stand on
// ---------------------------------------------------------------------------

/// **One thing a row needs a cell for** (the plan's §3, *How the cross is kept from
/// exploding*): the leaf's refusing cell; its base cell in its home state; its base cell
/// in each other prepared state, which runs on demand; and a planted cell for every line
/// of the row and every kind of plant that targets it ([`byte_cell::aimed`]).
#[derive(Clone, Debug, PartialEq, Eq)]
enum Need {
    Refusing,
    Base,
    BaseIn(Prepared),
    /// The base cell of a second spelling of the leaf, in its home state.
    BaseAs(&'static str),
    /// A plant aimed at a line, **under a spelling of the leaf** — `None` for the plain
    /// one: a need is keyed by the spelling as well as the line, so a cell of a spelling
    /// aimed at a line of every spelling never answers the plain spelling's need of it.
    Planted(Aim, Option<&'static str>),
}

impl Need {
    /// The need as the scheduled list of cells names it.
    fn named(&self) -> String {
        match self {
            Need::Refusing => "refusing".to_string(),
            Need::Base => "base".to_string(),
            Need::BaseIn(state) => format!("base, in `{}`", state.name()),
            Need::BaseAs(spelling) => format!("base, as `{spelling}`"),
            Need::Planted(aim, None) => format!("{} × {}", aim.line, aim.kind.tag()),
            Need::Planted(aim, Some(spelling)) => {
                format!("{} × {}, as `{spelling}`", aim.line, aim.kind.tag())
            }
        }
    }
}

/// The seven prepared states (the plan's §3).
const STATES: [Prepared; 7] = [
    Prepared::Bare,
    Prepared::Of(State::Fresh),
    Prepared::Of(State::CommittedSingletons),
    Prepared::Of(State::Migrated),
    Prepared::Of(State::RefsPostHoc),
    Prepared::Of(State::ChattyHooks),
    Prepared::Of(State::Vendored),
];

/// Everything `row` needs a cell for, in order: the two layers that have no plant, the
/// states that are not its home, then its lines, each with the kinds that target it.
fn needs(row: &Row) -> Vec<Need> {
    let mut out = vec![Need::Refusing, Need::Base];
    out.extend(
        STATES
            .iter()
            .filter(|state| **state != row.home)
            .map(|state| Need::BaseIn(*state)),
    );
    out.extend(
        row.spellings
            .iter()
            .map(|spelling| Need::BaseAs(spelling.name)),
    );
    for line in row.lines {
        for kind in Kind::ALL {
            if byte_cell::aimed(*kind, line).is_ok() {
                out.push(Need::Planted(
                    Aim {
                        line: line.id,
                        kind: *kind,
                    },
                    line.spelling,
                ));
            }
        }
    }
    out
}

/// **The need `cell` answers under `row`** — its layer, and for a planted cell what its
/// plant is aimed at. `Err` is a fault of the cell as written: a plant aimed at nothing,
/// an aim with no plant, an aim at a line the row does not have or a line the kind does
/// not target — with the reason the frame derives — or a plant that is not of the kind
/// the cell says.
fn answers(cell: &byte_cell::Cell, row: &Row) -> Result<Need, String> {
    let layer = cell.layer(row);
    if let Some(name) = cell.spelling
        && !row.spellings.iter().any(|spelling| spelling.name == name)
    {
        return Err(format!(
            "names the spelling `{name}`, which its row does not have"
        ));
    }
    let aim = match (layer, cell.aim, cell.spelling) {
        (Layer::Planted, Some(aim), _) => aim,
        (Layer::Planted, None, _) => {
            return Err("puts a plant in and aims it at no line".to_string());
        }
        (_, Some(_), _) => return Err("aims at a line and puts no plant in".to_string()),
        (Layer::Base, None, Some(name)) => {
            return Ok(Need::BaseAs(
                row.spellings
                    .iter()
                    .find(|spelling| spelling.name == name)
                    .map(|spelling| spelling.name)
                    .expect("the spelling is the row's"),
            ));
        }
        (Layer::Refusing | Layer::BaseElsewhere, None, Some(name)) => {
            return Err(format!(
                "names the spelling `{name}` where a spelling's base cell stands in its row's \
                 home state, with the leads that make its state"
            ));
        }
        (Layer::Refusing, None, None) => return Ok(Need::Refusing),
        (Layer::Base, None, None) => return Ok(Need::Base),
        (Layer::BaseElsewhere, None, None) => return Ok(Need::BaseIn(cell.state)),
    };
    let Some(line) = row.lines.iter().find(|line| line.id == aim.line) else {
        return Err(format!(
            "aims at the line `{}`, which its row does not have",
            aim.line
        ));
    };
    let replaced = row.spellings.iter().any(|spelling| {
        cell.spelling == Some(spelling.name) && spelling.replaces.contains(&line.id)
    });
    if line
        .spelling
        .is_some_and(|name| cell.spelling != Some(name))
        || replaced
    {
        return Err(format!(
            "aims at the line `{}`, which is not in force under the spelling it runs",
            aim.line
        ));
    }
    if let Err(why) = byte_cell::aimed(aim.kind, line) {
        return Err(format!(
            "aims a plant of the kind *{}* at the line `{}`, and {why}",
            aim.kind.tag(),
            aim.line
        ));
    }
    let of_the_kind = cell.leads.iter().any(|lead| match lead {
        Lead::Plant(plant) => plant.is_of(aim.kind),
        _ => false,
    });
    if !of_the_kind {
        return Err(format!(
            "says its plant is of the kind *{}*, and puts in none that is",
            aim.kind.tag()
        ));
    }
    Ok(Need::Planted(aim, cell.spelling))
}

/// **The scheduled list of cells** — what a row needs a cell for and the pilot leaves for
/// before the port: one file of data, a line a need with its reason.
const SCHEDULED_CELLS: &str = include_str!("byte_table/scheduled-cells.tsv");

/// The rows of a scheduled list of cells: every line that is no comment and not empty, as
/// a leaf — its words, space-separated — a need as [`Need::named`] spells it, and a
/// reason, tab-separated.
fn scheduled_cells(list: &'static str) -> Vec<(Vec<&'static str>, &'static str, &'static str)> {
    list.lines()
        .filter(|line| !line.is_empty() && !line.starts_with('#'))
        .map(|line| {
            let mut columns = line.splitn(3, '\t');
            let leaf = columns.next().unwrap_or_default();
            (
                leaf.split(' ').collect(),
                columns.next().unwrap_or_default(),
                columns.next().unwrap_or_default(),
            )
        })
        .collect()
}

/// **Every way the cells and the scheduled list of cells fail to answer for what the rows
/// need** — empty when every need of every row has exactly one answer: a cell, or a
/// scheduled row with its reason. A cell that cannot be read as an answer is named too
/// ([`answers`]).
fn unmet(
    rows: &[&Row],
    cells: &[&byte_cell::Cell],
    scheduled: &[(Vec<&str>, &str, &str)],
) -> Vec<String> {
    let mut out = Vec::new();
    let named = |leaf: &[&str]| format!("`jigc {}`", leaf.join(" "));
    let mut met: Vec<(&[&str], Need)> = Vec::new();
    for cell in cells {
        let Some(row) = rows.iter().find(|row| row.leaf == cell.leaf) else {
            continue;
        };
        match answers(cell, row) {
            Ok(need) => met.push((row.leaf, need)),
            Err(fault) => out.push(format!("`{}` {fault}", cell.name)),
        }
    }
    for row in rows {
        let waiting: Vec<&(Vec<&str>, &str, &str)> = scheduled
            .iter()
            .filter(|(leaf, _, _)| leaf.as_slice() == row.leaf)
            .collect();
        let needs = needs(row);
        for need in &needs {
            let cell = met
                .iter()
                .any(|(leaf, answer)| *leaf == row.leaf && answer == need);
            let rows = waiting
                .iter()
                .filter(|(_, what, _)| *what == need.named())
                .count();
            match (cell, rows) {
                (true, 0) | (false, 1) => {}
                (false, 0) => out.push(format!(
                    "{} needs a cell for *{}*, and has no cell and no scheduled row for it",
                    named(row.leaf),
                    need.named()
                )),
                _ => out.push(format!(
                    "{} is answered more than once for *{}*",
                    named(row.leaf),
                    need.named()
                )),
            }
        }
        for (_, what, reason) in waiting {
            if !needs.iter().any(|need| need.named() == *what) {
                out.push(format!(
                    "{} has a scheduled row for *{what}*, which is nothing its row needs a \
                     cell for",
                    named(row.leaf)
                ));
            }
            if reason.trim().is_empty() {
                out.push(format!(
                    "{} has a scheduled row for *{what}* that does not say why it waits",
                    named(row.leaf)
                ));
            }
        }
    }
    for (leaf, _, _) in scheduled {
        if !rows.iter().any(|row| row.leaf == leaf.as_slice()) {
            out.push(format!(
                "{} is on the scheduled list of cells, and has no row",
                named(leaf)
            ));
        }
    }
    out
}

/// A line a need, tab-separated: the leaf, the need, and what answers it — the cells, a
/// scheduled row with its reason, or the task that owes the leaf's cells.
fn needs_table(
    rows: &[&Row],
    cells: &[&byte_cell::Cell],
    scheduled: &[(Vec<&str>, &str, &str)],
) -> String {
    let mut out = "command\tneeds a cell for\tanswered by\n".to_string();
    for row in rows {
        for need in needs(row) {
            let by: Vec<&str> = cells
                .iter()
                .filter(|cell| cell.leaf == row.leaf && answers(cell, row).as_ref() == Ok(&need))
                .map(|cell| cell.name)
                .collect();
            let waits = scheduled
                .iter()
                .find(|(leaf, what, _)| leaf.as_slice() == row.leaf && *what == need.named());
            let answer = match (by.is_empty(), waits) {
                (false, _) => by.join(" · "),
                (true, Some((_, _, reason))) => format!("scheduled: {reason}"),
                (true, None) => "nothing".to_string(),
            };
            out.push_str(&format!(
                "jigc {}\t{}\t{answer}\n",
                row.leaf.join(" "),
                need.named()
            ));
        }
    }
    out
}

/// **Every row has, for each thing it needs a cell for, a cell or a scheduled row** — so a
/// line a row gains tomorrow needs its planted cells that day, and a cell cannot be born
/// aimed at nothing. The list of leaves whose cells a task of the plan owed whole was
/// emptied by the last of them, the task `B9`, and went with it.
///
/// On a fixture: a row with no cell is named need by need — its refusing cell, its base
/// cell, the base cell of each other prepared state, and each plant that targets one of
/// its lines; a cell or a scheduled row answers each; a need answered
/// twice is named, and so are a scheduled row for nothing the row needs, one with no
/// reason, a plant aimed at no line, an aim with no plant, an aim at a line the row does
/// not have, an aim the line's own vocabulary turns away — with **the reason the frame
/// derives** — and a plant that is not of the kind the cell says. Then this tree: nothing
/// is unmet; and **the fence is not vacuous** — with the scheduled list taken away, every
/// scheduled need is named, and with the `milestone` family's cells taken away, its needs.
#[test]
fn every_need_of_a_row_has_a_cell_or_a_scheduled_row() {
    const REPLACES: Row = Row {
        leaf: &["task", "finalize"],
        home: Prepared::Of(State::Fresh),
        ..row(&[
            line("F1", Allows::Files(Where::Path("a-file"), May::WriteWhole)),
            line("F2", Allows::Commit(Holding::TheIndex)),
            Line {
                when: When::Refused,
                on_refusal: true,
                ..line(
                    "F3",
                    Allows::Files(Where::Under(".jigc/index"), May::Rebuilt),
                )
            },
        ])
    };
    let names: Vec<String> = needs(&REPLACES).iter().map(Need::named).collect();
    assert_eq!(
        names,
        [
            "refusing",
            "base",
            "base, in `bare`",
            "base, in `committed-singletons`",
            "base, in `migrated`",
            "base, in `refs-post-hoc`",
            "base, in `chatty-hooks`",
            "base, in `vendored`",
            "F1 × U",
            "F1 × T",
            "F1 × L, to nothing",
            "F1 × L, to a file in the repository",
            "F1 × L, to a file outside it",
            "F1 × K, a directory in a file's place",
            "F1 × K, a file in a directory's place",
            "F1 × E, after the first write",
            "F1 × S, where the command writes",
            "F2 × S, where it does not",
            "F2 × N",
            "F2 × H",
        ],
        "what a row needs a cell for is read off its lines"
    );
    assert_eq!(
        unmet(&[&REPLACES], &[], &[]).len(),
        names.len(),
        "a row nobody answers for is named need by need"
    );

    const fn cell(
        name: &'static str,
        state: Prepared,
        leads: &'static [Lead],
        argv: &'static [&'static str],
        aim: Option<Aim>,
    ) -> byte_cell::Cell {
        byte_cell::Cell {
            spelling: None,
            name,
            leaf: &["task", "finalize"],
            state,
            leads,
            argv,
            listed: Listed::Acts,
            beside: None,
            aim,
            marks: &[],
            at: None,
        }
    }
    const FRESH: Prepared = Prepared::Of(State::Fresh);
    const A_STEP: &[Lead] = &[Lead::Step(Step::Jigc(&["start", "an intent"]))];
    const A_FILE: &[Lead] = &[Lead::Plant(Plant::File("a-file"))];

    const fn at(line: &'static str, kind: Kind) -> Option<Aim> {
        Some(Aim { line, kind })
    }
    const UNRESOLVED: &[&str] = &["task", "finalize", "a-task"];
    const ACTING: &[&str] = &["task", "finalize", "an-intent"];
    let refusing = cell(
        "refusing",
        FRESH,
        &[],
        &["task", "finalize", "a-task"],
        None,
    );
    let base = cell("base", FRESH, A_STEP, ACTING, None);
    let elsewhere = cell("elsewhere", Prepared::Bare, &[], ACTING, None);
    let planted = cell("planted", FRESH, A_FILE, ACTING, at("F1", Kind::Untracked));
    assert_eq!(answers(&refusing, &REPLACES), Ok(Need::Refusing));
    assert_eq!(answers(&base, &REPLACES), Ok(Need::Base));
    assert_eq!(
        answers(&elsewhere, &REPLACES),
        Ok(Need::BaseIn(Prepared::Bare))
    );
    assert_eq!(
        answers(&planted, &REPLACES),
        Ok(Need::Planted(
            Aim {
                line: "F1",
                kind: Kind::Untracked
            },
            None
        ))
    );
    // The argv that resolves nothing is the refusing cell's in `fresh` and with no lead,
    // and nowhere else.
    assert_eq!(
        answers(
            &cell("unresolved-in-bare", Prepared::Bare, &[], UNRESOLVED, None),
            &REPLACES
        ),
        Ok(Need::BaseIn(Prepared::Bare))
    );
    assert_eq!(
        answers(
            &cell("unresolved-after-a-lead", FRESH, A_STEP, UNRESOLVED, None),
            &REPLACES
        ),
        Ok(Need::Base)
    );
    let the_four = [&refusing, &base, &elsewhere, &planted];
    assert_eq!(
        unmet(&[&REPLACES], &the_four, &[]).len(),
        names.len() - 4,
        "a cell answers the need of its layer, and no other"
    );
    let waits = |what: &'static str, why: &'static str| (vec!["task", "finalize"], what, why);
    let rest: Vec<(Vec<&str>, &str, &str)> = needs(&REPLACES)
        .iter()
        .map(Need::named)
        .filter(|what| !["refusing", "base", "base, in `bare`", "F1 × U"].contains(&what.as_str()))
        .map(|what| waits(what.leak(), "before the port"))
        .collect();
    says(&unmet(&[&REPLACES], &the_four, &rest), &[]);

    // Answered twice; a row for nothing; a row with no reason; a row of a leaf with none.
    let mut twice = rest.clone();
    twice.push(waits("F1 × U", "before the port"));
    twice.push(waits("F3 × U", "before the port"));
    twice.push(waits("F2 × H", " "));
    says(
        &unmet(&[&REPLACES], &the_four, &twice),
        &[
            ("`jigc task finalize`", "more than once for *F1 × U*"),
            ("`jigc task finalize`", "more than once for *F2 × H*"),
            (
                "a scheduled row for *F3 × U*",
                "nothing its row needs a cell for",
            ),
            ("a scheduled row for *F2 × H*", "does not say why it waits"),
        ],
    );
    let mut stray = rest.clone();
    stray.push((vec!["doc", "create"], "base", "before the port"));
    says(
        &unmet(&[&REPLACES], &the_four, &stray),
        &[("`jigc doc create`", "has no row")],
    );

    // A cell that cannot be read as an answer.
    for (cell, said) in [
        (
            cell("a-plant-aimed-at-nothing", FRESH, A_FILE, ACTING, None),
            "puts a plant in and aims it at no line",
        ),
        (
            cell(
                "an-aim-with-no-plant",
                FRESH,
                A_STEP,
                ACTING,
                at("F1", Kind::Untracked),
            ),
            "aims at a line and puts no plant in",
        ),
        (
            cell(
                "a-line-the-row-does-not-have",
                FRESH,
                A_FILE,
                ACTING,
                at("F9", Kind::Untracked),
            ),
            "aims at the line `F9`, which its row does not have",
        ),
        (
            cell(
                "a-line-the-kind-does-not-target",
                FRESH,
                A_FILE,
                ACTING,
                at("F1", Kind::Foreign),
            ),
            "at the line `F1`, and the line removes no directory",
        ),
        (
            cell(
                "a-line-that-holds-on-a-refusal-only",
                FRESH,
                A_FILE,
                ACTING,
                at("F3", Kind::Untracked),
            ),
            "the line holds on a refusal only",
        ),
        (
            cell(
                "a-plant-of-another-kind",
                FRESH,
                A_FILE,
                ACTING,
                at("F1", Kind::Committed),
            ),
            "says its plant is of the kind *T*, and puts in none that is",
        ),
    ] {
        let mut with = the_four.to_vec();
        with.push(&cell);
        says(&unmet(&[&REPLACES], &with, &rest), &[(cell.name, said)]);
    }

    // A plant in the place of a file that stands is of its inner plant's kind, but for the
    // two that stand where no file does — in a directory the command removes (**D**), and
    // in a directory's place (**K**): a file of the user's put there is of neither, where
    // the same file put on its own is of both.
    const REMOVES: Row = Row {
        leaf: &["task", "finalize"],
        home: Prepared::Of(State::Fresh),
        ..row(&[
            line("F1", Allows::Files(Where::Path("a-file"), May::WriteWhole)),
            line(
                "F4",
                Allows::Files(Where::Under(".jigc/tasks"), May::Removed),
            ),
        ])
    };
    const IN_A_FILES_PLACE: &[Lead] = &[Lead::Plant(Plant::InPlaceOf(&Plant::File("a-file")))];
    for (line, kind) in [("F4", Kind::Foreign), ("F1", Kind::FileForADir)] {
        assert_eq!(
            answers(
                &cell(
                    "in-a-files-place",
                    FRESH,
                    IN_A_FILES_PLACE,
                    ACTING,
                    at(line, kind)
                ),
                &REMOVES
            ),
            Err(format!(
                "says its plant is of the kind *{}*, and puts in none that is",
                kind.tag()
            )),
            "a file of the user's in the place of a file that stands is no plant of *{}*",
            kind.tag()
        );
        assert_eq!(
            answers(
                &cell("on-its-own", FRESH, A_FILE, ACTING, at(line, kind)),
                &REMOVES
            ),
            Ok(Need::Planted(Aim { line, kind }, None)),
            "the same file put on its own is a plant of *{}*",
            kind.tag()
        );
    }
    assert_eq!(
        answers(
            &cell(
                "in-a-files-place",
                FRESH,
                IN_A_FILES_PLACE,
                ACTING,
                at("F1", Kind::Untracked)
            ),
            &REMOVES
        ),
        Ok(Need::Planted(
            Aim {
                line: "F1",
                kind: Kind::Untracked
            },
            None
        )),
        "a file of the user's in the place of a file that stands is a plant of *U*"
    );

    // A file simply deleted, the index left as it stands, is a plant of **E** — git lists
    // it among the changes not staged, beside an edit — at either of E's positions, and of
    // no other kind: not **S**, which a deletion staged with it would be, and not **U**.
    for kind in Kind::ALL {
        assert_eq!(
            Plant::Deleted("a-file").is_of(*kind),
            matches!(kind, Kind::EditBefore | Kind::EditAfter),
            "a file deleted is a plant of *{}* only where that is *E*",
            kind.tag()
        );
    }
    const DELETED: &[Lead] = &[Lead::Plant(Plant::Deleted("a-file"))];
    assert_eq!(
        answers(
            &cell("deleted", FRESH, DELETED, ACTING, at("F1", Kind::EditAfter)),
            &REMOVES
        ),
        Ok(Need::Planted(
            Aim {
                line: "F1",
                kind: Kind::EditAfter
            },
            None
        )),
        "a file deleted answers *E* at a line that lets the command rewrite it"
    );
    assert_eq!(
        answers(
            &cell(
                "deleted",
                FRESH,
                DELETED,
                ACTING,
                at("F1", Kind::StagedThere)
            ),
            &REMOVES
        ),
        Err(
            "says its plant is of the kind *S, where the command writes*, and puts in none that \
             is"
            .to_string()
        ),
        "a file deleted, the index left as it stands, is no staged change"
    );

    // The home a write copies a committed doc in from — read, and nothing written there
    // (`May::CopiedFrom`, the `doc` family's `W3`): a file of the user's, a link, a
    // directory in the doc's place, and the doc edited or deleted on either side of the
    // task's first write are each aimed at it — and nothing that is put at a place the
    // command writes: a committed file is the doc the line reads, a staged change and a
    // line of the user's are held to a write, a file in the home's directory's place
    // takes every doc beside it.
    const READS: Row = Row {
        leaf: &["doc", "set-slot"],
        home: Prepared::Of(State::CommittedSingletons),
        ..row(&[line(
            "F5",
            Allows::Files(Where::AStagedDocsHome, May::CopiedFrom),
        )])
    };
    let read: Vec<String> = needs(&READS)
        .iter()
        .map(Need::named)
        .filter(|need| need.starts_with("F5"))
        .collect();
    assert_eq!(
        read,
        [
            "F5 × U",
            "F5 × L, to nothing",
            "F5 × L, to a file in the repository",
            "F5 × L, to a file outside it",
            "F5 × K, a directory in a file's place",
            "F5 × E, before the first write",
            "F5 × E, after the first write",
        ],
        "the home a write copies in from takes the plants that stand in a doc's place"
    );

    // This tree.
    let (rows, cells) = (rows(), cells());
    let wrong = unmet(&rows, &cells, &scheduled_cells(SCHEDULED_CELLS));
    assert!(
        wrong.is_empty(),
        "the byte table's cells do not answer for what its rows need:\n\n  {}\n\na row needs \
         its refusing cell, its base cell in its home state and in each other prepared state, \
         and a planted cell for every line and every kind of plant that targets it — each a \
         cell in a file under `byte_table/`, or a line of `byte_table/scheduled-cells.tsv` \
         with its reason",
        wrong.join("\n  ")
    );
    // Not vacuous: with nothing scheduled, every scheduled need is named and no other — the
    // cells that are there answer the rest; and with a family's cells taken away, its needs
    // are named.
    let bare = unmet(&rows, &cells, &[]);
    let needed: usize = rows.iter().map(|row| needs(row).len()).sum();
    let waiting = scheduled_cells(SCHEDULED_CELLS).len();
    assert!(
        bare.len() == waiting && bare.len() < needed,
        "with nothing scheduled, {} of {needed} needs are unmet, where {waiting} are scheduled",
        bare.len()
    );
    assert!(
        bare.iter()
            .any(|line| line.contains("*S2 × K, a file in a directory's place*"))
            && !bare.iter().any(|line| line.contains("*U1 × D*")),
        "{bare:#?}"
    );
    let without: Vec<&byte_cell::Cell> = cells
        .iter()
        .copied()
        .filter(|cell| cell.leaf.first() != Some(&"milestone"))
        .collect();
    let taken = unmet(&rows, &without, &scheduled_cells(SCHEDULED_CELLS));
    assert!(
        taken.iter().any(|line| line
            == "`jigc milestone finalize` needs a cell for *refusing*, and has no cell and no \
                scheduled row for it"),
        "{taken:#?}"
    );
    assert_eq!(
        needs_table(&rows, &cells, &scheduled_cells(SCHEDULED_CELLS))
            .lines()
            .count(),
        needed + 1,
        "the table of needs has a line a need, under its header"
    );
}

/// **The cells that run on demand, as their arm last printed them** — the base cells in a
/// prepared state that is not their leaf's home ([`Layer::BaseElsewhere`]): committed,
/// dated, and **held by no gate** — a cell here that starts to lose bytes is seen when
/// somebody runs the arm ([`the_cells_on_demand_are_run_and_printed`]).
const ON_DEMAND: &str = include_str!(
    "../../../completions/artifacts/M55/stabilization-build/reshape/byte-table-on-demand-cells.tsv"
);

/// The header of that table, as the arm prints it.
const ON_DEMAND_HEADER: &str = "cell\tcommand\tstate\tlisted as\tcame to\tits exit";

/// The cells that run on demand, of `cells`.
fn on_demand(cells: &[&'static byte_cell::Cell]) -> Vec<&'static byte_cell::Cell> {
    cells
        .iter()
        .copied()
        .filter(|cell| {
            rows()
                .iter()
                .find(|row| row.leaf == cell.leaf)
                .is_some_and(|row| cell.layer(row) == Layer::BaseElsewhere)
        })
        .collect()
}

/// **Every way a committed table of the cells on demand and the cells themselves
/// disagree**: a cell with no line, a line for no cell, a cell with two lines, a table
/// that does not open with the arm's header.
fn unlisted(cells: &[&str], table: &str) -> Vec<String> {
    let mut out = Vec::new();
    let mut lines = table
        .lines()
        .filter(|line| !line.is_empty() && !line.starts_with('#'));
    if lines.next() != Some(ON_DEMAND_HEADER) {
        out.push(
            "the table of the cells on demand does not open with the arm's header".to_string(),
        );
    }
    let listed: Vec<&str> = lines
        .map(|line| line.split('\t').next().unwrap_or_default())
        .collect();
    for cell in cells {
        match listed.iter().filter(|name| *name == cell).count() {
            1 => {}
            0 => out.push(format!(
                "`{cell}` runs on demand, and the committed table of its arm has no line for it"
            )),
            _ => out.push(format!(
                "`{cell}` has more than one line in the committed table"
            )),
        }
    }
    for name in listed {
        if !cells.contains(&name) {
            out.push(format!(
                "the committed table has a line for `{name}`, which is no cell that runs on demand"
            ));
        }
    }
    out
}

/// **Every cell that runs on demand has its line in the committed table of that arm**,
/// and the table has no line for anything else — on a fixture, fault by fault; then this
/// tree's cells against the committed table.
#[test]
fn every_cell_on_demand_has_its_line_in_the_committed_table() {
    let table = |lines: &str| format!("# printed on a day\n{ON_DEMAND_HEADER}\n{lines}");
    says(
        &unlisted(&["a", "b"], &table("a\tjigc x\nb\tjigc y\n")),
        &[],
    );
    says(&unlisted(&[], &table("")), &[]);
    says(
        &unlisted(&["a", "b"], &table("a\tjigc x\na\tjigc x\nc\tjigc z\n")),
        &[
            ("`a`", "more than one line"),
            ("`b` runs on demand", "has no line for it"),
            ("a line for `c`", "no cell that runs on demand"),
        ],
    );
    says(
        &unlisted(&["a"], "cell\tcommand\na\tjigc x\n"),
        &[("the table of the cells on demand", "the arm's header")],
    );

    let cells = cells();
    let names: Vec<&str> = on_demand(&cells).iter().map(|cell| cell.name).collect();
    let wrong = unlisted(&names, ON_DEMAND);
    assert!(
        wrong.is_empty(),
        "the cells that run on demand and their committed table:\n\n  {}\n\nrun \
         `byte_table::{ON_DEMAND_ARM}` and commit what it prints under `== cells on demand ==` \
         as `…/reshape/byte-table-on-demand-cells.tsv`, under a line that says when",
        wrong.join("\n  ")
    );
}

/// **What each line of each row excuses, by the prepared state in whose cells it excuses
/// a difference** — the cells the gate runs. Exact both ways in every arm
/// ([`excused_otherwise`]): a line that excuses a difference in a state it is not listed
/// for is named, and so is one listed for a state in whose cells it excuses nothing. A
/// line no cell of any state excuses a difference under stands on [`EXCUSES_NOTHING`].
const EXCUSED_IN: &[(&[&str], &str, &[Prepared])] = &[
    (&["setup"], "S1", &[Prepared::Bare]),
    (&["setup"], "S2", &[Prepared::Bare]),
    (&["setup"], "S3", &[Prepared::Bare]),
    (&["setup"], "S4", &[Prepared::Bare]),
    (&["setup"], "S5", &[Prepared::Bare]),
    (&["setup"], "S6, written", &[Prepared::Bare]),
    (&["setup"], "S7", &[Prepared::Bare]),
    (&["setup"], "S8", &[Prepared::Bare]),
    (&["setup"], "S9", &[Prepared::Bare]),
    (&["setup"], "S10", &[Prepared::Bare]),
    (&["setup"], "S11", &[Prepared::Bare]),
    (&["setup"], "S16, the file-state hashes", &[Prepared::Bare]),
    (&["setup"], "C2", &[Prepared::Bare]),
    (&["uninstall"], "U1", &[Prepared::Of(State::Fresh)]),
    (&["uninstall"], "U4", &[Prepared::Of(State::Fresh)]),
    (&["uninstall"], "U4, removed", &[Prepared::Of(State::Fresh)]),
    (&["ingest"], "I1", &[Prepared::Of(State::Fresh)]),
    (&["ingest"], "I2", &[Prepared::Of(State::Fresh)]),
    (
        &["unmanage"],
        "X1",
        &[Prepared::Of(State::CommittedSingletons)],
    ),
    (
        &["unmanage"],
        "X2",
        &[
            Prepared::Of(State::Fresh),
            Prepared::Of(State::CommittedSingletons),
        ],
    ),
    (
        &["task", "finalize"],
        "C1",
        &[
            Prepared::Of(State::Fresh),
            Prepared::Of(State::CommittedSingletons),
        ],
    ),
    (
        &["task", "finalize"],
        "T1",
        &[
            Prepared::Of(State::Fresh),
            Prepared::Of(State::CommittedSingletons),
        ],
    ),
    (&["task", "finalize"], "T2", &[Prepared::Of(State::Fresh)]),
    (&["task", "finalize"], "T9", &[Prepared::Of(State::Fresh)]),
    (
        &["task", "finalize"],
        "T4",
        &[
            Prepared::Of(State::Fresh),
            Prepared::Of(State::CommittedSingletons),
        ],
    ),
    (
        &["task", "finalize"],
        "T5, the edge index",
        &[Prepared::Of(State::Fresh)],
    ),
    (
        &["task", "finalize"],
        "T5, the file-state hashes",
        &[
            Prepared::Of(State::Fresh),
            Prepared::Of(State::CommittedSingletons),
        ],
    ),
    (
        &["task", "finalize"],
        "T6",
        &[
            Prepared::Of(State::Fresh),
            Prepared::Of(State::CommittedSingletons),
        ],
    ),
    (
        &["task", "finalize"],
        "T6, moved aside",
        &[Prepared::Of(State::Fresh)],
    ),
    (&["task", "discard"], "A1", &[Prepared::Of(State::Fresh)]),
    (&["task", "discard"], "A5", &[Prepared::Of(State::Fresh)]),
    (&["task", "discard"], "A6", &[Prepared::Of(State::Fresh)]),
    (&["task", "discard"], "A7", &[Prepared::Of(State::Fresh)]),
    (
        &["milestone", "finalize"],
        "M1",
        &[
            Prepared::Of(State::Fresh),
            Prepared::Of(State::CommittedSingletons),
        ],
    ),
    (
        &["milestone", "finalize"],
        "M4",
        &[Prepared::Of(State::Fresh)],
    ),
    (
        &["milestone", "finalize"],
        "M3",
        &[
            Prepared::Of(State::Fresh),
            Prepared::Of(State::CommittedSingletons),
        ],
    ),
    (
        &["milestone", "finalize"],
        "M5",
        &[
            Prepared::Of(State::Fresh),
            Prepared::Of(State::CommittedSingletons),
        ],
    ),
    (
        &["milestone", "finalize"],
        "M6, the worktrees",
        &[
            Prepared::Of(State::Fresh),
            Prepared::Of(State::CommittedSingletons),
        ],
    ),
    (
        &["milestone", "finalize"],
        "M6, the sub-tasks' areas",
        &[
            Prepared::Of(State::Fresh),
            Prepared::Of(State::CommittedSingletons),
        ],
    ),
    (
        &["milestone", "finalize"],
        "M6, the milestone's area",
        &[
            Prepared::Of(State::Fresh),
            Prepared::Of(State::CommittedSingletons),
        ],
    ),
    (
        &["milestone", "finalize"],
        "M7",
        &[
            Prepared::Of(State::Fresh),
            Prepared::Of(State::CommittedSingletons),
        ],
    ),
    (
        &["milestone", "finalize"],
        "M10, the edge index",
        &[
            Prepared::Of(State::Fresh),
            Prepared::Of(State::CommittedSingletons),
        ],
    ),
    (
        &["milestone", "finalize"],
        "M10, the file-state hashes",
        &[
            Prepared::Of(State::Fresh),
            Prepared::Of(State::CommittedSingletons),
        ],
    ),
    (
        &["milestone", "create"],
        "MC1",
        &[Prepared::Of(State::Fresh)],
    ),
    (
        &["milestone", "create"],
        "MC2",
        &[Prepared::Of(State::Fresh)],
    ),
    (
        &["milestone", "create"],
        "MC3",
        &[Prepared::Of(State::Fresh)],
    ),
    (
        &["milestone", "create"],
        "MC4",
        &[Prepared::Of(State::Fresh)],
    ),
    (
        &["milestone", "add-task"],
        "MA1",
        &[Prepared::Of(State::Fresh)],
    ),
    (
        &["milestone", "add-task"],
        "MA2",
        &[Prepared::Of(State::Fresh)],
    ),
    (
        &["milestone", "add-task"],
        "MA3",
        &[Prepared::Of(State::Fresh)],
    ),
    (
        &["milestone", "add-task"],
        "MA4",
        &[Prepared::Of(State::Fresh)],
    ),
    (
        &["milestone", "add-task"],
        "MA5",
        &[Prepared::Of(State::Fresh)],
    ),
    (
        &["milestone", "add-from-spec"],
        "MS1",
        &[Prepared::Of(State::Vendored)],
    ),
    (
        &["milestone", "add-from-spec"],
        "MS2",
        &[Prepared::Of(State::Vendored)],
    ),
    (
        &["milestone", "add-from-spec"],
        "MS3",
        &[Prepared::Of(State::Vendored)],
    ),
    (
        &["milestone", "add-from-spec"],
        "MS4",
        &[Prepared::Of(State::Vendored)],
    ),
    (
        &["milestone", "add-from-spec"],
        "MS5",
        &[Prepared::Of(State::Vendored)],
    ),
    (
        &["milestone", "provision"],
        "MP1",
        &[Prepared::Of(State::Fresh)],
    ),
    (
        &["milestone", "provision"],
        "MP2",
        &[Prepared::Of(State::Fresh)],
    ),
    (
        &["milestone", "execute"],
        "ME1",
        &[Prepared::Of(State::Fresh)],
    ),
    (
        &["milestone", "execute"],
        "ME2",
        &[Prepared::Of(State::Fresh)],
    ),
    (&["milestone", "join"], "C1", &[Prepared::Of(State::Fresh)]),
    (
        &["milestone", "finalize"],
        "M12",
        &[Prepared::Of(State::Fresh)],
    ),
    (
        &["milestone", "finalize"],
        "C1",
        &[Prepared::Of(State::Fresh)],
    ),
    (
        &["milestone", "join"],
        "MJ4, the edge index",
        &[Prepared::Of(State::Fresh)],
    ),
    (
        &["milestone", "finalize"],
        "M2",
        &[
            Prepared::Of(State::Fresh),
            Prepared::Of(State::CommittedSingletons),
        ],
    ),
    (
        &["milestone", "finalize"],
        "M6, moved aside",
        &[Prepared::Of(State::Fresh)],
    ),
    (
        &["milestone", "finalize"],
        "M6, the milestone's area moved aside",
        &[Prepared::Of(State::Fresh)],
    ),
    (
        &["milestone", "discard"],
        "MD1",
        &[Prepared::Of(State::Fresh)],
    ),
    (
        &["milestone", "discard"],
        "MD2",
        &[Prepared::Of(State::Fresh)],
    ),
    (
        &["milestone", "discard"],
        "MD3",
        &[Prepared::Of(State::Fresh)],
    ),
    (
        &["milestone", "discard"],
        "MD4, the milestone's area",
        &[Prepared::Of(State::Fresh)],
    ),
    (
        &["milestone", "discard"],
        "MD4, the sub-tasks' areas",
        &[Prepared::Of(State::Fresh)],
    ),
    (
        &["milestone", "discard"],
        "MD4, the worktrees",
        &[Prepared::Of(State::Fresh)],
    ),
    (
        &["milestone", "discard"],
        "MD6",
        &[Prepared::Of(State::Fresh)],
    ),
    (
        &["doc", "add-item"],
        "W1",
        &[Prepared::Of(State::CommittedSingletons)],
    ),
    (
        &["doc", "add-item"],
        "W2",
        &[Prepared::Of(State::CommittedSingletons)],
    ),
    (
        &["doc", "remove-item"],
        "W1",
        &[Prepared::Of(State::CommittedSingletons)],
    ),
    (
        &["doc", "remove-item"],
        "W2",
        &[Prepared::Of(State::CommittedSingletons)],
    ),
    (
        &["doc", "retitle-item"],
        "W1",
        &[Prepared::Of(State::CommittedSingletons)],
    ),
    (
        &["doc", "retitle-item"],
        "W2",
        &[Prepared::Of(State::CommittedSingletons)],
    ),
    (
        &["doc", "rename"],
        "W1",
        &[Prepared::Of(State::CommittedSingletons)],
    ),
    (
        &["doc", "rename"],
        "W2",
        &[Prepared::Of(State::CommittedSingletons)],
    ),
    (
        &["doc", "rename"],
        "W4",
        &[Prepared::Of(State::CommittedSingletons)],
    ),
    (
        &["doc", "set-field"],
        "W1",
        &[Prepared::Of(State::CommittedSingletons)],
    ),
    (
        &["doc", "set-field"],
        "W2",
        &[Prepared::Of(State::CommittedSingletons)],
    ),
    (
        &["doc", "set-slot"],
        "W1",
        &[Prepared::Of(State::CommittedSingletons)],
    ),
    (
        &["doc", "set-slot"],
        "W2",
        &[Prepared::Of(State::CommittedSingletons)],
    ),
    (
        &["doc", "create"],
        "W1",
        &[Prepared::Of(State::CommittedSingletons)],
    ),
    (
        &["doc", "create"],
        "W2",
        &[Prepared::Of(State::CommittedSingletons)],
    ),
    (
        &["doc", "author"],
        "W1",
        &[Prepared::Of(State::CommittedSingletons)],
    ),
    (
        &["doc", "author"],
        "W2",
        &[Prepared::Of(State::CommittedSingletons)],
    ),
    (&["rename"], "RN1", &[Prepared::Of(State::Fresh)]),
    (
        &["rename"],
        "RN1, at its new home",
        &[Prepared::Of(State::Fresh)],
    ),
    (&["rename"], "RN2", &[Prepared::Of(State::Fresh)]),
    (&["rename"], "RN3", &[Prepared::Of(State::Fresh)]),
    (&["rename"], "RN4", &[Prepared::Of(State::Fresh)]),
    (&["rename"], "RN5", &[Prepared::Of(State::Fresh)]),
    (&["rename"], "C1", &[Prepared::Of(State::Fresh)]),
    (&["rename"], "C2", &[Prepared::Of(State::Fresh)]),
    (&["migrate-corpus"], "MG1", &[Prepared::Of(State::Fresh)]),
    (
        &["migrate-corpus"],
        "MG1, from a prior home",
        &[Prepared::Of(State::Fresh)],
    ),
    (&["migrate-corpus"], "MG2", &[Prepared::Of(State::Fresh)]),
    (&["migrate-corpus"], "MG3", &[Prepared::Of(State::Fresh)]),
    (&["relocate"], "RL1", &[Prepared::Of(State::Fresh)]),
    (
        &["relocate"],
        "RL1, at its new home",
        &[Prepared::Of(State::Fresh)],
    ),
    (&["relocate"], "RL4", &[Prepared::Of(State::Fresh)]),
    (&["config", "set"], "CS1", &[Prepared::Of(State::Fresh)]),
    (&["config", "set"], "CS2", &[Prepared::Of(State::Fresh)]),
    (
        &["config", "set"],
        "CS2, at its new home",
        &[Prepared::Of(State::Fresh)],
    ),
    (&["config", "set"], "CS4", &[Prepared::Of(State::Fresh)]),
    (&["start"], "ST1", &[Prepared::Of(State::Fresh)]),
    (&["start"], "ST2", &[Prepared::Of(State::Fresh)]),
    (
        &["start"],
        "ST2, at a resume",
        &[Prepared::Of(State::Fresh)],
    ),
    (&["workflow"], "WF1", &[Prepared::Of(State::Fresh)]),
    (&["workflow"], "WF2", &[Prepared::Of(State::Fresh)]),
    (&["migrate"], "MI1", &[Prepared::Of(State::Fresh)]),
    (&["task", "amend"], "AM1", &[Prepared::Of(State::Fresh)]),
];

/// Why the doc family's `W3` excuses nothing, and never will: it lets nothing change at
/// its place. It names the home a write copies in from, which the plants in a doc's place
/// are aimed at.
const READ_AND_NEVER_WRITTEN: &str = "nothing can: the line lets no difference happen at the home a write copies in from — it \
     names the place, which is what the plants in a doc's place are aimed at";

/// **The lines that excuse nothing in any cell the gate runs** — named, each with what
/// would use it, and not removed: a line is its doc's sentence, and stands whether or not
/// a cell has met it yet. A line that is on neither list, or on both, is named
/// ([`unaccounted`]).
const EXCUSES_NOTHING: &[(&[&str], &str, &str)] = &[
    (&["rename"], "C3, the base pin", NO_CELL_OF_THIS_LEAF),
    (&["rename"], "C3, the task list", NO_CELL_OF_THIS_LEAF),
    (&["migrate-corpus"], "MG4", STAGED_UNDER_ANOTHER_LINE),
    (&["migrate-corpus"], "MG5", A_DRY_RUN_WRITES_NOTHING),
    (&["relocate"], "RL2", STAGED_UNDER_ANOTHER_LINE),
    (&["start"], "C1", A_REFUSAL_THAT_REBUILDS_IT),
    (&["start"], "C2", A_REFUSAL_THAT_REBUILDS_IT),
    (&["start"], "C3, the base pin", NO_CELL_OF_THIS_LEAF),
    (&["start"], "C3, the task list", NO_CELL_OF_THIS_LEAF),
    (&["workflow"], "C1", A_REFUSAL_THAT_REBUILDS_IT),
    (&["workflow"], "C2", A_REFUSAL_THAT_REBUILDS_IT),
    (&["workflow"], "C3, the base pin", NO_CELL_OF_THIS_LEAF),
    (&["workflow"], "C3, the task list", NO_CELL_OF_THIS_LEAF),
    (&["migrate"], "C1", A_REFUSAL_THAT_REBUILDS_IT),
    (&["migrate"], "C2", A_REFUSAL_THAT_REBUILDS_IT),
    (&["migrate"], "C3, the base pin", NO_CELL_OF_THIS_LEAF),
    (&["migrate"], "C3, the task list", NO_CELL_OF_THIS_LEAF),
    (&["task", "amend"], "C1", A_REFUSAL_THAT_REBUILDS_IT),
    (&["task", "amend"], "C2", A_REFUSAL_THAT_REBUILDS_IT),
    (&["task", "amend"], "C3, the base pin", NO_CELL_OF_THIS_LEAF),
    (
        &["task", "amend"],
        "C3, the task list",
        NO_CELL_OF_THIS_LEAF,
    ),
    (&["migrate"], "MI2", READ_AND_NEVER_WRITTEN),
    (&["relocate"], "C1", A_REFUSAL_THAT_REBUILDS_IT),
    (&["relocate"], "C2", A_REFUSAL_THAT_REBUILDS_IT),
    (&["relocate"], "C3, the base pin", NO_CELL_OF_THIS_LEAF),
    (&["relocate"], "C3, the task list", NO_CELL_OF_THIS_LEAF),
    (&["config", "set"], "CS5", STAGED_UNDER_ANOTHER_LINE),
    (&["config", "set"], "C1", A_REFUSAL_THAT_REBUILDS_IT),
    (&["config", "set"], "C2", A_REFUSAL_THAT_REBUILDS_IT),
    (&["config", "set"], "C3, the base pin", NO_CELL_OF_THIS_LEAF),
    (
        &["config", "set"],
        "C3, the task list",
        NO_CELL_OF_THIS_LEAF,
    ),
    (&["migrate-corpus"], "C1", A_REFUSAL_THAT_REBUILDS_IT),
    (&["migrate-corpus"], "C2", A_REFUSAL_THAT_REBUILDS_IT),
    (
        &["migrate-corpus"],
        "C3, the base pin",
        NO_CELL_OF_THIS_LEAF,
    ),
    (
        &["migrate-corpus"],
        "C3, the task list",
        NO_CELL_OF_THIS_LEAF,
    ),
    (
        &["setup"],
        "S6, removed",
        A_RECORD_WHERE_THE_COMMIT_CANNOT_CARRY_IT,
    ),
    (
        &["setup"],
        "S16, the edge index",
        NO_SETUP_LAYS_THE_INDEX_OUT,
    ),
    (&["setup"], "C1", A_REFUSAL_THAT_REBUILDS_IT),
    (&["setup"], "C3, the base pin", NO_CELL_OF_THIS_LEAF),
    (&["setup"], "C3, the task list", NO_CELL_OF_THIS_LEAF),
    (&["uninstall"], "U2", NOT_UNTIL_THE_TEARDOWN_COMMITS),
    (&["uninstall"], "U3", NOT_UNTIL_THE_TEARDOWN_COMMITS),
    (&["uninstall"], "U5", NOT_UNTIL_THE_TEARDOWN_COMMITS),
    (&["uninstall"], "U6", A_REGISTRATION_UNDER_THE_WORKBENCH),
    (&["uninstall"], "U9", NOT_UNTIL_THE_TEARDOWN_COMMITS),
    (&["ingest"], "C1", A_REFUSAL_THAT_REBUILDS_IT),
    (&["ingest"], "C2", A_REFUSAL_THAT_REBUILDS_IT),
    (&["ingest"], "C3, the base pin", NO_CELL_OF_THIS_LEAF),
    (&["ingest"], "C3, the task list", NO_CELL_OF_THIS_LEAF),
    (&["unmanage"], "C1", A_REFUSAL_THAT_REBUILDS_IT),
    (&["unmanage"], "C2", A_REFUSAL_THAT_REBUILDS_IT),
    (&["unmanage"], "C3, the base pin", NO_CELL_OF_THIS_LEAF),
    (&["unmanage"], "C3, the task list", NO_CELL_OF_THIS_LEAF),
    (&["uninstall"], "C1", A_REFUSAL_THAT_REBUILDS_IT),
    (&["uninstall"], "C2", A_REFUSAL_THAT_REBUILDS_IT),
    (&["uninstall"], "C3, the base pin", NO_CELL_OF_THIS_LEAF),
    (&["uninstall"], "C3, the task list", NO_CELL_OF_THIS_LEAF),
    (&["task", "finalize"], "T3", STAGED_UNDER_ANOTHER_LINE),
    (
        &["task", "finalize"],
        "T3, the config",
        NO_FIRST_COMMIT_OF_THE_CONFIG_LAYER,
    ),
    (
        &["task", "finalize"],
        "T3, the config's ignore file",
        NO_FIRST_COMMIT_OF_THE_CONFIG_LAYER,
    ),
    (&["task", "finalize"], "C2", A_REFUSAL_THAT_REBUILDS_IT),
    (
        &["task", "finalize"],
        "C3, the base pin",
        NO_CELL_OF_THIS_LEAF,
    ),
    (
        &["task", "finalize"],
        "C3, the task list",
        NO_CELL_OF_THIS_LEAF,
    ),
    (&["task", "discard"], "C1", A_REFUSAL_THAT_REBUILDS_IT),
    (&["task", "discard"], "C2", A_REFUSAL_THAT_REBUILDS_IT),
    (
        &["task", "discard"],
        "C3, the base pin",
        A_REFUSAL_THAT_REBUILDS_IT,
    ),
    (
        &["task", "discard"],
        "C3, the task list",
        A_REFUSAL_THAT_REBUILDS_IT,
    ),
    (&["milestone", "finalize"], "C2", A_REFUSAL_THAT_REBUILDS_IT),
    (
        &["milestone", "finalize"],
        "C3, the base pin",
        A_REFUSAL_THAT_REBUILDS_IT,
    ),
    (
        &["milestone", "finalize"],
        "C3, the task list",
        A_REFUSAL_THAT_REBUILDS_IT,
    ),
    (&["milestone", "create"], "C1", A_REFUSAL_THAT_REBUILDS_IT),
    (&["milestone", "create"], "C2", A_REFUSAL_THAT_REBUILDS_IT),
    (
        &["milestone", "create"],
        "C3, the base pin",
        A_REFUSAL_THAT_REBUILDS_IT,
    ),
    (
        &["milestone", "create"],
        "C3, the task list",
        A_REFUSAL_THAT_REBUILDS_IT,
    ),
    (&["milestone", "add-task"], "MA6", NO_AREA_TO_RE_SEED),
    (&["milestone", "add-task"], "C1", A_REFUSAL_THAT_REBUILDS_IT),
    (&["milestone", "add-task"], "C2", A_REFUSAL_THAT_REBUILDS_IT),
    (
        &["milestone", "add-task"],
        "C3, the base pin",
        A_REFUSAL_THAT_REBUILDS_IT,
    ),
    (
        &["milestone", "add-task"],
        "C3, the task list",
        A_REFUSAL_THAT_REBUILDS_IT,
    ),
    (&["milestone", "add-from-spec"], "MS6", NO_AREA_TO_RE_SEED),
    (
        &["milestone", "add-from-spec"],
        "C1",
        A_REFUSAL_THAT_REBUILDS_IT,
    ),
    (
        &["milestone", "add-from-spec"],
        "C2",
        A_REFUSAL_THAT_REBUILDS_IT,
    ),
    (
        &["milestone", "add-from-spec"],
        "C3, the base pin",
        A_REFUSAL_THAT_REBUILDS_IT,
    ),
    (
        &["milestone", "add-from-spec"],
        "C3, the task list",
        A_REFUSAL_THAT_REBUILDS_IT,
    ),
    (&["milestone", "provision"], "MP3", NO_CACHE_TO_RE_DERIVE),
    (&["milestone", "provision"], "MP4", NO_AREA_TO_RE_SEED),
    (
        &["milestone", "provision"],
        "C1",
        A_REFUSAL_THAT_REBUILDS_IT,
    ),
    (
        &["milestone", "provision"],
        "C2",
        A_REFUSAL_THAT_REBUILDS_IT,
    ),
    (
        &["milestone", "provision"],
        "C3, the base pin",
        A_REFUSAL_THAT_REBUILDS_IT,
    ),
    (
        &["milestone", "provision"],
        "C3, the task list",
        A_REFUSAL_THAT_REBUILDS_IT,
    ),
    (&["milestone", "execute"], "C1", A_REFUSAL_THAT_REBUILDS_IT),
    (&["milestone", "execute"], "C2", A_REFUSAL_THAT_REBUILDS_IT),
    (
        &["milestone", "execute"],
        "C3, the base pin",
        A_REFUSAL_THAT_REBUILDS_IT,
    ),
    (
        &["milestone", "execute"],
        "C3, the task list",
        A_REFUSAL_THAT_REBUILDS_IT,
    ),
    (&["milestone", "join"], "MJ1", NO_CACHE_TO_RE_DERIVE),
    (&["milestone", "join"], "MJ2", NO_AREA_TO_RE_SEED),
    (&["milestone", "join"], "C2", A_REFUSAL_THAT_REBUILDS_IT),
    (
        &["milestone", "join"],
        "C3, the base pin",
        A_REFUSAL_THAT_REBUILDS_IT,
    ),
    (
        &["milestone", "join"],
        "C3, the task list",
        A_REFUSAL_THAT_REBUILDS_IT,
    ),
    (&["milestone", "discard"], "C1", A_REFUSAL_THAT_REBUILDS_IT),
    (&["milestone", "discard"], "C2", A_REFUSAL_THAT_REBUILDS_IT),
    (
        &["milestone", "discard"],
        "C3, the base pin",
        A_REFUSAL_THAT_REBUILDS_IT,
    ),
    (
        &["milestone", "discard"],
        "C3, the task list",
        A_REFUSAL_THAT_REBUILDS_IT,
    ),
    (
        &["milestone", "finalize"],
        "M3, the config",
        NO_FIRST_COMMIT_OF_THE_CONFIG_LAYER,
    ),
    (
        &["milestone", "finalize"],
        "M11",
        A_TERMINAL_TEARS_DOWN_WHAT_IT_RE_SEEDS,
    ),
    (
        &["milestone", "discard"],
        "MD7",
        A_TERMINAL_TEARS_DOWN_WHAT_IT_RE_SEEDS,
    ),
    (
        &["milestone", "discard"],
        "MD8",
        NO_DISCARD_OVER_AN_ABSENT_AREA,
    ),
    (
        &["milestone", "finalize"],
        "M3, the config's ignore file",
        NO_FIRST_COMMIT_OF_THE_CONFIG_LAYER,
    ),
    (
        &["milestone", "finalize"],
        "M6, the worktrees moved aside",
        NOT_UNTIL_THE_BOUNDARY_MOVES_THE_WORKTREES_LEFTOVERS,
    ),
    (&["doc", "add-item"], "W3", READ_AND_NEVER_WRITTEN),
    (&["doc", "add-item"], "C1", A_REFUSAL_THAT_REBUILDS_IT),
    (&["doc", "add-item"], "C2", A_REFUSAL_THAT_REBUILDS_IT),
    (
        &["doc", "add-item"],
        "C3, the base pin",
        NO_CELL_OF_THIS_LEAF,
    ),
    (
        &["doc", "add-item"],
        "C3, the task list",
        NO_CELL_OF_THIS_LEAF,
    ),
    (&["doc", "remove-item"], "W3", READ_AND_NEVER_WRITTEN),
    (&["doc", "remove-item"], "C1", A_REFUSAL_THAT_REBUILDS_IT),
    (&["doc", "remove-item"], "C2", A_REFUSAL_THAT_REBUILDS_IT),
    (
        &["doc", "remove-item"],
        "C3, the base pin",
        NO_CELL_OF_THIS_LEAF,
    ),
    (
        &["doc", "remove-item"],
        "C3, the task list",
        NO_CELL_OF_THIS_LEAF,
    ),
    (&["doc", "retitle-item"], "W3", READ_AND_NEVER_WRITTEN),
    (&["doc", "retitle-item"], "C1", A_REFUSAL_THAT_REBUILDS_IT),
    (&["doc", "retitle-item"], "C2", A_REFUSAL_THAT_REBUILDS_IT),
    (
        &["doc", "retitle-item"],
        "C3, the base pin",
        NO_CELL_OF_THIS_LEAF,
    ),
    (
        &["doc", "retitle-item"],
        "C3, the task list",
        NO_CELL_OF_THIS_LEAF,
    ),
    (&["doc", "rename"], "W3", READ_AND_NEVER_WRITTEN),
    (&["doc", "rename"], "C1", A_REFUSAL_THAT_REBUILDS_IT),
    (&["doc", "rename"], "C2", A_REFUSAL_THAT_REBUILDS_IT),
    (&["doc", "rename"], "C3, the base pin", NO_CELL_OF_THIS_LEAF),
    (
        &["doc", "rename"],
        "C3, the task list",
        NO_CELL_OF_THIS_LEAF,
    ),
    (&["doc", "set-field"], "W3", READ_AND_NEVER_WRITTEN),
    (&["doc", "set-field"], "C1", A_REFUSAL_THAT_REBUILDS_IT),
    (&["doc", "set-field"], "C2", A_REFUSAL_THAT_REBUILDS_IT),
    (
        &["doc", "set-field"],
        "C3, the base pin",
        NO_CELL_OF_THIS_LEAF,
    ),
    (
        &["doc", "set-field"],
        "C3, the task list",
        NO_CELL_OF_THIS_LEAF,
    ),
    (&["doc", "set-slot"], "W3", READ_AND_NEVER_WRITTEN),
    (&["doc", "set-slot"], "C1", A_REFUSAL_THAT_REBUILDS_IT),
    (&["doc", "set-slot"], "C2", A_REFUSAL_THAT_REBUILDS_IT),
    (
        &["doc", "set-slot"],
        "C3, the base pin",
        NO_CELL_OF_THIS_LEAF,
    ),
    (
        &["doc", "set-slot"],
        "C3, the task list",
        NO_CELL_OF_THIS_LEAF,
    ),
    (&["doc", "create"], "W3", READ_AND_NEVER_WRITTEN),
    (&["doc", "create"], "C1", A_REFUSAL_THAT_REBUILDS_IT),
    (&["doc", "create"], "C2", A_REFUSAL_THAT_REBUILDS_IT),
    (&["doc", "create"], "C3, the base pin", NO_CELL_OF_THIS_LEAF),
    (
        &["doc", "create"],
        "C3, the task list",
        NO_CELL_OF_THIS_LEAF,
    ),
    (&["doc", "author"], "W3", READ_AND_NEVER_WRITTEN),
    (&["doc", "author"], "C1", A_REFUSAL_THAT_REBUILDS_IT),
    (&["doc", "author"], "C2", A_REFUSAL_THAT_REBUILDS_IT),
    (&["doc", "author"], "C3, the base pin", NO_CELL_OF_THIS_LEAF),
    (
        &["doc", "author"],
        "C3, the task list",
        NO_CELL_OF_THIS_LEAF,
    ),
];

/// What would use a line of `uninstall`'s whose change lands at a tracked path, and the
/// commit line itself: the teardown's commit — none is made on this tree, so each such
/// change is outside the allowance (`U9`, the human's ruling of 2026-10-14), and the line
/// excuses nothing.
const NOT_UNTIL_THE_TEARDOWN_COMMITS: &str = "a teardown that lands its changes at a tracked path in a commit, as `U9` asks: on this \
     tree it commits nothing, so such a change is outside the allowance and no line excuses it";

/// What would use `U6`: a registration under `.jigc/worktrees/`.
const A_REGISTRATION_UNDER_THE_WORKBENCH: &str = "a registration under `.jigc/worktrees/` — a milestone's sub-task provisioned before the \
     teardown, which no cell of the install family makes: the milestone family's (`B7`)";

/// What would use `S6, removed`: a settings record standing where the install commit
/// cannot carry the settings file.
const A_RECORD_WHERE_THE_COMMIT_CANNOT_CARRY_IT: &str = "a settings record standing where the install commit cannot carry the settings file — an \
     ignored or untracked `.claude/settings.json`, which no cell of the family constructs: before \
     the port";

/// What would use the half of `S16` over the edge index: a `setup` that lays that
/// directory out as well.
const NO_SETUP_LAYS_THE_INDEX_OUT: &str = "a `setup` that lays out `.jigc/index/` as the ruling's sentence lets it: no cell's does \
     — each that acts lays out `.jigc/state/` alone";

/// Why the line that lets a command stage excuses nothing by itself.
const STAGED_UNDER_ANOTHER_LINE: &str = "no cell can: the comparison excuses a staged entry under the line that wrote the file, or \
     under the commit's — this line is what lets the row stage at all";

/// What would use a line of the ruling on a refusal's caches that no refusal of the
/// leaf's cells has met.
const A_REFUSAL_THAT_REBUILDS_IT: &str =
    "a refusal of this leaf that leaves that cache rebuilt: none of its cells' refusals does";

/// What would use a line of the re-seed of the sub-tasks' areas: an area the record names
/// that is absent before the command.
const NO_AREA_TO_RE_SEED: &str = "a sub-task's area the committed record names and the workbench lacks — a fresh clone, or the \
     area taken away by hand — which the family's cells construct at `execute` and at the \
     boundary's refusal alone: at this door, before the port";

/// What would use a line of the re-seed of the milestone's cache: the cache absent, or stale,
/// before the command.
const NO_CACHE_TO_RE_DERIVE: &str = "the milestone's cache absent or stale before the command — a fresh clone, or the cache \
     taken away by hand — which the family's cells construct at `execute` alone: at this door, \
     before the port";

/// What would use a line of the re-seed at a terminal — `finalize`'s and `discard`'s, added
/// after the review of the task `B7`'s second part (its tenth finding).
const A_TERMINAL_TEARS_DOWN_WHAT_IT_RE_SEEDS: &str = "the milestone's cache absent or stale before the command, which no cell of this door \
     constructs — and what the re-seed rebuilds there the door's own teardown removes where it \
     acts, and a refusal's cache is `C3`'s: before the port";

/// What would use `discard`'s line of the re-seed of the sub-tasks' areas, which holds on a
/// refusal too since the seventh fix after that review.
const NO_DISCARD_OVER_AN_ABSENT_AREA: &str = "a sub-task's area absent before `discard`, which no cell of the door constructs — where \
     the door acts its teardown removes the area the re-seed rebuilt, and where it refuses the \
     area stays, as at the boundary's cell of a refusal: before the port";

/// What would use a line of the config layer a finalize carries, `T3, the config` and
/// `M3, the config`, each with its ignore file's half: the layer's first commit.
const NO_FIRST_COMMIT_OF_THE_CONFIG_LAYER: &str = "a commit that is the config layer's first — `HEAD` holding nothing under `.jigc/config`, or no \
     `.jigc/.gitignore`, before the command — which no prepared state is: `setup`'s install \
     commit lands the layer, and a repository set up before it did is before the port";

/// What would use `M6, the worktrees moved aside`: a landed boundary that moves a live
/// sub-task worktree's unstaged and untracked files aside, as the human's ruling of
/// 2026-10-19 on the rows' record's §6, entry 18 has it.
const NOT_UNTIL_THE_BOUNDARY_MOVES_THE_WORKTREES_LEFTOVERS: &str = "a landed boundary that moves what git neither ignores nor holds out of a sub-task's \
     worktree into `.jigc/displaced/<sub-task-id>/`, as the human's ruling on entry 18 has it: on \
     this tree it takes the file with the checkout, and the cell of it is lost under its key";

/// What would use `migrate-corpus`'s `MG5`: nothing — the dry run's line names the docs it
/// reads, which is what the plants of its spelling's cells are aimed at, and lets no
/// difference happen there.
const A_DRY_RUN_WRITES_NOTHING: &str = "nothing can: the dry run's line lets no difference happen at the doc it reads — it names \
     the place, which is what the plants of the spelling's cells are aimed at";

/// What would use a line of the ruling that stands in a row whose leaf has no milestone.
const NO_CELL_OF_THIS_LEAF: &str = "no cell of this leaf is expected to: the ruling's lines stand in every row, and this \
     leaf reads no milestone's cache";

/// **Every way the two lists fail to account for the lines of `rows`**: a line on neither
/// list, a line on both, a line listed twice, an entry for a line no row has, an entry of
/// [`EXCUSED_IN`] that names no state, an entry of [`EXCUSES_NOTHING`] that does not say
/// what would use the line.
fn unaccounted(
    rows: &[&Row],
    excused_in: &[(&[&str], &str, &[Prepared])],
    nothing: &[(&[&str], &str, &str)],
) -> Vec<String> {
    let mut out = Vec::new();
    let named =
        |leaf: &[&str], line: &str| format!("the line `{line}` of `jigc {}`", leaf.join(" "));
    for row in rows {
        for line in row.lines {
            let somewhere = excused_in
                .iter()
                .filter(|(leaf, id, _)| *leaf == row.leaf && *id == line.id)
                .count();
            let nowhere = nothing
                .iter()
                .filter(|(leaf, id, _)| *leaf == row.leaf && *id == line.id)
                .count();
            match (somewhere, nowhere) {
                (1, 0) | (0, 1) => {}
                (0, 0) => out.push(format!(
                    "{} is on neither list: it excuses a difference in the cells of a state, \
                     or it is named as excusing nothing",
                    named(row.leaf, line.id)
                )),
                _ => out.push(format!(
                    "{} is listed more than once",
                    named(row.leaf, line.id)
                )),
            }
        }
    }
    let has = |leaf: &[&str], id: &str| {
        rows.iter()
            .any(|row| row.leaf == leaf && row.lines.iter().any(|line| line.id == id))
    };
    for (leaf, id, states) in excused_in {
        if !has(leaf, id) {
            out.push(format!("{} is listed, and no row has it", named(leaf, id)));
        }
        if states.is_empty() {
            out.push(format!(
                "{} is listed as excusing a difference, in the cells of no state",
                named(leaf, id)
            ));
        }
    }
    for (leaf, id, would) in nothing {
        if !has(leaf, id) {
            out.push(format!("{} is listed, and no row has it", named(leaf, id)));
        }
        if would.trim().is_empty() {
            out.push(format!(
                "{} is named as excusing nothing, and nothing says what would use it",
                named(leaf, id)
            ));
        }
    }
    out
}

/// What the lines of a cell's row excused: each line's `id`, and how many differences.
type Excused<'a> = &'a [(&'static str, usize)];

/// **Every way what the lines excused in the cells of `state` is not what [`EXCUSED_IN`]
/// says of that state** — over the rows whose leaf has a cell among `ran`, each cell with
/// what the lines of its row excused: a line that excused a difference there and is not
/// listed for the state, and a line listed for it that excused none.
fn excused_otherwise(
    state: Prepared,
    rows: &[&Row],
    ran: &[(&[&str], Excused<'_>)],
    excused_in: &[(&[&str], &str, &[Prepared])],
) -> Vec<String> {
    let mut out = Vec::new();
    for row in rows {
        if !ran.iter().any(|(leaf, _)| *leaf == row.leaf) {
            continue;
        }
        for line in row.lines {
            let times: usize = ran
                .iter()
                .filter(|(leaf, _)| *leaf == row.leaf)
                .flat_map(|(_, excused)| excused.iter())
                .filter(|(id, _)| *id == line.id)
                .map(|(_, times)| times)
                .sum();
            let listed = excused_in.iter().any(|(leaf, id, states)| {
                *leaf == row.leaf && *id == line.id && states.contains(&state)
            });
            let named = format!("the line `{}` of `jigc {}`", line.id, row.leaf.join(" "));
            match (times, listed) {
                (0, true) => out.push(format!(
                    "{named} is listed as excusing a difference in the cells of `{}`, and \
                     excused none",
                    state.name()
                )),
                (times, false) if times > 0 => out.push(format!(
                    "{named} excused {times} difference(s) in the cells of `{}`, and is not \
                     listed as excusing any there",
                    state.name()
                )),
                _ => {}
            }
        }
    }
    out
}

/// **Every line of a row excuses a difference in some cell the gate runs, or is named** —
/// and where it does is exact in every arm. On a fixture: a line on neither list, on
/// both, listed for no state, named with nothing that would use it, and an entry for a
/// line no row has are each named; and of what an arm's cells excused, a line that
/// excused a difference in a state it is not listed for, and a line listed for a state it
/// excused nothing in. Then this tree's rows against the two lists. What the cells
/// excuse is held where they run ([`run_the_cells_of`]).
#[test]
fn every_line_of_a_row_excuses_a_difference_in_some_cell_or_is_named() {
    const TWO: Row = Row {
        leaf: &["task", "finalize"],
        ..row(&[
            line("F1", Allows::Staged),
            line("F2", Allows::Commit(Holding::TheIndex)),
        ])
    };
    const FRESH: Prepared = Prepared::Of(State::Fresh);
    let leaf: &[&str] = &["task", "finalize"];
    says(
        &unaccounted(
            &[&TWO],
            &[(leaf, "F1", &[FRESH])],
            &[(leaf, "F2", "a cell")],
        ),
        &[],
    );
    says(
        &unaccounted(&[&TWO], &[], &[]),
        &[
            ("the line `F1` of `jigc task finalize`", "on neither list"),
            ("the line `F2` of `jigc task finalize`", "on neither list"),
        ],
    );
    says(
        &unaccounted(
            &[&TWO],
            &[(leaf, "F1", &[]), (leaf, "F3", &[FRESH])],
            &[(leaf, "F1", "a cell"), (leaf, "F2", " "), (leaf, "F4", "-")],
        ),
        &[
            ("the line `F1`", "listed more than once"),
            ("the line `F1`", "in the cells of no state"),
            ("the line `F3`", "no row has it"),
            ("the line `F2`", "nothing says what would use it"),
            ("the line `F4`", "no row has it"),
        ],
    );
    let listed: &[(&[&str], &str, &[Prepared])] = &[(leaf, "F1", &[FRESH])];
    let excused = |f1, f2| -> Vec<(&'static str, usize)> { vec![("F1", f1), ("F2", f2)] };
    let (once, never, other) = (excused(1, 0), excused(0, 0), excused(0, 2));
    says(
        &excused_otherwise(
            FRESH,
            &[&TWO],
            &[(leaf, once.as_slice()), (leaf, never.as_slice())],
            listed,
        ),
        &[],
    );
    says(
        &excused_otherwise(
            FRESH,
            &[&TWO],
            &[(leaf, never.as_slice()), (leaf, other.as_slice())],
            listed,
        ),
        &[
            (
                "the line `F1`",
                "is listed as excusing a difference in the cells of `fresh`, and excused none",
            ),
            (
                "the line `F2`",
                "excused 2 difference(s) in the cells of `fresh`, and is not listed",
            ),
        ],
    );
    // Another state's cells are held to their own listing, and a row with no cell in the
    // arm to nothing.
    says(
        &excused_otherwise(Prepared::Bare, &[&TWO], &[(leaf, once.as_slice())], listed),
        &[(
            "the line `F1`",
            "excused 1 difference(s) in the cells of `bare`",
        )],
    );
    says(&excused_otherwise(FRESH, &[&TWO], &[], listed), &[]);

    let wrong = unaccounted(&rows(), EXCUSED_IN, EXCUSES_NOTHING);
    assert!(
        wrong.is_empty(),
        "the lines of the rows and the two lists of what they excuse:\n\n  {}\n\na line \
         excuses a difference in the cells of a state (`EXCUSED_IN`), or it is named with \
         what would use it (`EXCUSES_NOTHING`) — exactly one of the two",
        wrong.join("\n  ")
    );
}

/// **The count the pilot's bound stands on** — which write leaves the fix pass touched
/// since the previous release, each leaf read against its handler, under each reading of
/// *touched* the plan names (its §1.4).
const COUNT: &str = include_str!(
    "../../../completions/artifacts/M55/stabilization-build/reshape/byte-table-leaves-touched-since-rc24.md"
);

/// The reading of *touched* the pilot is built under, as the count's table heads its
/// column: the plan's rule.
const THE_PLANS_RULE: &str = "the plan's rule";

/// One line of the count's table: a leaf's words, and under each reading whether the
/// count holds the leaf touched.
type Counted<'a> = (Vec<&'a str>, Vec<(&'a str, bool)>);

/// **The count's table**: every line that opens ``| `jigc `` — a leaf, and under each
/// reading whether the count holds it touched. The readings are the table's own column
/// heads, from its second column on.
fn counted(count: &'static str) -> Vec<Counted<'static>> {
    let cells_of = |line: &'static str| -> Vec<&'static str> {
        line.trim_matches('|').split('|').map(str::trim).collect()
    };
    let heads: Vec<&str> = count
        .lines()
        .find(|line| line.starts_with("| leaf |"))
        .map(cells_of)
        .unwrap_or_default();
    count
        .lines()
        .filter(|line| line.starts_with("| `jigc "))
        .map(|line| {
            let cells = cells_of(line);
            let leaf = cells[0]
                .trim_matches('`')
                .trim_start_matches("jigc ")
                .split(' ')
                .collect();
            let readings = heads
                .iter()
                .zip(&cells)
                .skip(1)
                .filter(|(_, cell)| ["touched", "untouched"].contains(*cell))
                .map(|(head, cell)| (*head, *cell == "touched"))
                .collect();
            (leaf, readings)
        })
        .collect()
}

/// **Every way a count and a scheduled list of leaves disagree**: a write leaf the count
/// has no line for, or two; a line for no write leaf; a line that does not say, under
/// `reading`, whether the leaf is touched; and **a scheduled leaf the count holds
/// touched** under it.
fn uncounted(
    leaves: &[Vec<&str>],
    counted: &[Counted<'_>],
    scheduled: &[(Vec<&str>, &str)],
    reading: &str,
) -> Vec<String> {
    let mut out = Vec::new();
    let named = |leaf: &[&str]| format!("`jigc {}`", leaf.join(" "));
    let under = |leaf: &[&str]| {
        counted
            .iter()
            .find(|(counted, _)| counted.as_slice() == leaf)
            .and_then(|(_, readings)| readings.iter().find(|(head, _)| *head == reading))
            .map(|(_, touched)| *touched)
    };
    for leaf in leaves {
        match counted
            .iter()
            .filter(|(counted, _)| counted == leaf)
            .count()
        {
            1 => {
                if under(leaf).is_none() {
                    out.push(format!(
                        "the count does not say whether {} is touched under *{reading}*",
                        named(leaf)
                    ));
                }
            }
            0 => out.push(format!("the count has no line for {}", named(leaf))),
            _ => out.push(format!(
                "the count has more than one line for {}",
                named(leaf)
            )),
        }
    }
    for (leaf, _) in counted {
        if !leaves.contains(leaf) {
            out.push(format!(
                "the count has a line for {}, which is no write leaf",
                named(leaf)
            ));
        }
    }
    for (leaf, _) in scheduled {
        if under(leaf) == Some(true) {
            out.push(format!(
                "{} is on the scheduled list, and the count holds it touched under \
                 *{reading}*: a touched leaf is the pilot's, and has its row",
                named(leaf)
            ));
        }
    }
    out
}

/// **Every scheduled leaf is one the count holds untouched**, and the count has one line
/// for every write leaf — on a fixture, fault by fault; then the committed count against
/// the product's registry and the scheduled list.
#[test]
fn every_scheduled_leaf_is_one_the_count_holds_untouched() {
    const FIXTURE: &str = "| leaf | the plan's rule | another reading | why |\n|---|---|---|---|\n\
        | `jigc setup` | touched | touched | - |\n\
        | `jigc doc create` | untouched | touched | - |\n\
        | `jigc relocate` | by reading | touched | - |\n\
        | `jigc doc show` | untouched | untouched | - |\n";
    let counted_fixture = counted(FIXTURE);
    assert_eq!(counted_fixture.len(), 4, "{counted_fixture:?}");
    assert_eq!(
        counted_fixture[1],
        (
            vec!["doc", "create"],
            vec![("the plan's rule", false), ("another reading", true)]
        )
    );
    let leaves = vec![
        vec!["setup"],
        vec!["doc", "create"],
        vec!["relocate"],
        vec!["rename"],
    ];
    let waits = |leaf: &[&'static str]| (leaf.to_vec(), "the count holds it untouched");
    says(
        &uncounted(
            &leaves,
            &counted_fixture,
            &[waits(&["doc", "create"]), waits(&["setup"])],
            THE_PLANS_RULE,
        ),
        &[
            (
                "whether `jigc relocate` is touched",
                "under *the plan's rule*",
            ),
            ("no line for `jigc rename`", ""),
            ("a line for `jigc doc show`", "which is no write leaf"),
            (
                "`jigc setup` is on the scheduled list",
                "the count holds it touched",
            ),
        ],
    );
    // Under another reading the same scheduled leaf is touched.
    says(
        &uncounted(
            &[vec!["doc", "create"]],
            &counted_fixture[1..2],
            &[waits(&["doc", "create"])],
            "another reading",
        ),
        &[("`jigc doc create` is on the scheduled list", "touched")],
    );

    let wrong = uncounted(
        &write_leaves(),
        &counted(COUNT),
        &scheduled(SCHEDULED),
        THE_PLANS_RULE,
    );
    assert!(
        wrong.is_empty(),
        "the count of the leaves the fix pass touched and the scheduled list:\n\n  {}\n\nthe \
         count is `…/reshape/byte-table-leaves-touched-since-rc24.md`: a line a write leaf, \
         and a leaf it holds touched under the reading the pilot is built under has a row",
        wrong.join("\n  ")
    );
}

// ---------------------------------------------------------------------------
// The quote fence
// ---------------------------------------------------------------------------

/// The docs an allowance line may quote, by their path from the repository's root: the
/// design docs, and the decisions log for a ruling that stands where a design doc is silent.
const DOCS: &[(&str, &str)] = &[
    // A ruling of the human's is a line's sentence where the design doc is silent and the
    // ruling says what the command does: `uninstall`'s `U9`, until the doc says it.
    ("DECISIONS.md", include_str!("../../../DECISIONS.md")),
    (
        "design/assistant-adapter.md",
        include_str!("../../../design/assistant-adapter.md"),
    ),
    (
        "design/auto-migration.md",
        include_str!("../../../design/auto-migration.md"),
    ),
    (
        "design/corpus-migration.md",
        include_str!("../../../design/corpus-migration.md"),
    ),
    (
        "design/finalize.md",
        include_str!("../../../design/finalize.md"),
    ),
    (
        "design/overrides.md",
        include_str!("../../../design/overrides.md"),
    ),
    (
        "design/project-setup.md",
        include_str!("../../../design/project-setup.md"),
    ),
    (
        "design/reconciliation.md",
        include_str!("../../../design/reconciliation.md"),
    ),
    (
        "design/storage.md",
        include_str!("../../../design/storage.md"),
    ),
    (
        "design/team-ready-state.md",
        include_str!("../../../design/team-ready-state.md"),
    ),
    (
        "design/validation.md",
        include_str!("../../../design/validation.md"),
    ),
    (
        "design/write-commands.md",
        include_str!("../../../design/write-commands.md"),
    ),
];

fn doc(path: &str) -> Option<&'static str> {
    DOCS.iter()
        .find(|(doc, _)| *doc == path)
        .map(|(_, text)| *text)
}

/// **The committed record the rows are lifted from** — the four acceptance rows, each line
/// over the sentence it rests on.
const RECORD: &str = include_str!(
    "../../../completions/artifacts/M55/stabilization-build/reshape/byte-table-acceptance-rows.md"
);

/// The quotations of a record: every line that opens `> `, with the source two lines under
/// it — `` — `doc` → *heading* ``. A quotation with no such line under it has no doc.
fn quotations(record: &'static str) -> Vec<Quote> {
    let lines: Vec<&str> = record.lines().collect();
    lines
        .iter()
        .enumerate()
        .filter_map(|(at, line)| Some((at, line.strip_prefix("> ")?)))
        .map(|(at, words)| {
            let source = lines.get(at + 2).and_then(|line| {
                let (doc, heading) = line.strip_prefix("— `")?.split_once("` → *")?;
                Some((doc, heading.strip_suffix('*')?))
            });
            let (doc, heading) = source.unwrap_or_default();
            Quote {
                doc,
                heading,
                words,
            }
        })
        .collect()
}

/// The numbers of a record's entries: every line that opens `**<number> — `.
fn numbers(record: &'static str) -> BTreeSet<&'static str> {
    record
        .lines()
        .filter_map(|line| line.strip_prefix("**")?.split_once(" — "))
        .map(|(number, _)| number)
        .collect()
}

/// **Every way a row's lines do not rest on the record**: a line that quotes no sentence,
/// a line whose number — its `id` up to a comma — is no entry of the record, a quote the
/// record does not have, a number two lines of one row share.
fn unrested(rows: &[&Row], numbers: &BTreeSet<&str>, quoted: &[Quote]) -> Vec<String> {
    let mut out = Vec::new();
    for row in rows {
        let leaf = row.leaf.join(" ");
        for (at, line) in row.lines.iter().enumerate() {
            let named = format!("the line `{}` of `jigc {leaf}`", line.id);
            if line.quotes.is_empty() {
                out.push(format!(
                    "{named} quotes no sentence: a line that rests on nothing is none"
                ));
            }
            let number = line.id.split(',').next().unwrap_or_default();
            if !numbers.contains(number) {
                out.push(format!(
                    "{named} has the number `{number}`, which is no entry of the record"
                ));
            }
            for quote in line.quotes {
                if !quoted.contains(quote) {
                    out.push(format!(
                        "{named} quotes what the record does not: `{}`",
                        quote.words
                    ));
                }
            }
            if row.lines[..at].iter().any(|other| other.id == line.id) {
                out.push(format!(
                    "{named} shares its `id` with another line of its row"
                ));
            }
        }
    }
    out
}

/// **An allowance line's quote is held to its doc's text**: the words stand, byte for
/// byte, under the heading the line names.
///
/// On a fixture: a quote as the doc has it holds; one word off does not, nor the right
/// words under another heading — a later one included: a section ends where the next
/// heading of its level opens — a heading the doc does not have, a doc the fence does not
/// read, or a quote of no words; and a `#` line inside a fenced block ends no section. Then
/// the committed record: it holds quotations, every one names a doc and a heading, and
/// every one stands in its doc — so a design doc that moves under a line reddens here.
///
/// Then the rows: on a fixture, a line that quotes nothing is named, and so is a line
/// whose number the record does not have, a quote the record does not have, and a number
/// two lines of a row share; and the table's own rows rest on the record, line by line.
#[test]
fn a_lines_quote_is_held_to_its_docs_text() {
    const FIXTURE: &str = "# A doc\n\nNot in a section of its own.\n\n## The first\n\n\
        Setup seeds the floor **iff the repo has zero commits**.\n\n```\n# a comment, and no \
        heading\n```\n\nStill the first.\n\n### Under the first\n\nDeeper words.\n\n## The \
        second\n\nOther words.\n";
    let fixture = |path: &str| (path == "design/fixture.md").then_some(FIXTURE);
    let quote = |heading, words| Quote {
        doc: "design/fixture.md",
        heading,
        words,
    };
    for held in [
        quote(
            "The first",
            "seeds the floor **iff the repo has zero commits**",
        ),
        quote("The first", "Still the first."),
        quote("The first", "Deeper words."),
        quote("Under the first", "Deeper words."),
        quote("A doc", "Other words."),
    ] {
        assert_eq!(byte_cell::unquoted(&held, fixture), None, "{held:?}");
    }
    for (off, says) in [
        (
            quote(
                "The first",
                "seeds the floor **iff the repo has no commits**",
            ),
            "does not hold the words quoted from it",
        ),
        (
            quote("The second", "Still the first."),
            "does not hold the words",
        ),
        (
            quote("Under the first", "Still the first."),
            "does not hold the words",
        ),
        // A section ends where the next heading of its level opens.
        (
            quote("The first", "Other words."),
            "does not hold the words",
        ),
        (
            quote("The third", "Other words."),
            "has no heading `The third`",
        ),
        (quote("The first", " "), "holds no words"),
        (
            Quote {
                doc: "design/elsewhere.md",
                heading: "The first",
                words: "Still the first.",
            },
            "is no doc the fence reads",
        ),
    ] {
        let said = byte_cell::unquoted(&off, fixture);
        assert!(
            said.as_deref().is_some_and(|said| said.contains(says)),
            "{off:?}: {said:?}"
        );
    }

    let quoted = quotations(RECORD);
    assert!(
        quoted.len() >= 50,
        "the committed record holds {} quotations — it held 57 when the fence was built",
        quoted.len()
    );
    let wrong: Vec<String> = quoted
        .iter()
        .filter_map(|quote| byte_cell::unquoted(quote, doc))
        .collect();
    assert!(
        wrong.is_empty(),
        "an allowance line rests on a sentence its design doc no longer has, or never \
         had:\n\n  {}\n\na line is written from the doc's sentence and quotes it; where the \
         doc moved, the line is read against the doc again — it is never bent to what the \
         command does",
        wrong.join("\n  ")
    );

    // The rows, on a fixture.
    const HELD: Quote = Quote {
        doc: "design/fixture.md",
        heading: "The first",
        words: "Still the first.",
    };
    const RESTS: Line = Line {
        quotes: &[HELD],
        ..line("F1", Allows::Staged)
    };
    const FIXTURE_ROW: Row = row(&[
        RESTS,
        line("F2", Allows::Staged),
        Line {
            id: "G1, a second place",
            ..RESTS
        },
        Line {
            quotes: &[Quote {
                words: "Other words.",
                ..HELD
            }],
            ..line("F3", Allows::Staged)
        },
        Line { id: "F1", ..RESTS },
    ]);
    let entries: BTreeSet<&str> = ["F1", "F2", "F3"].into();
    says(
        &unrested(&[&FIXTURE_ROW], &entries, &[HELD]),
        &[
            ("the line `F2` of `jigc probe`", "quotes no sentence"),
            (
                "the line `G1, a second place` of `jigc probe`",
                "has the number `G1`, which is no entry of the record",
            ),
            (
                "the line `F3` of `jigc probe`",
                "quotes what the record does not: `Other words.`",
            ),
            ("the line `F1` of `jigc probe`", "shares its `id`"),
        ],
    );

    // The table's own.
    let rows = rows();
    let lines = rows.iter().flat_map(|row| row.lines).count();
    assert!(
        lines >= 40,
        "the four rows hold {lines} lines — they held 54 when they became data"
    );
    let wrong = unrested(&rows, &numbers(RECORD), &quoted);
    assert!(
        wrong.is_empty(),
        "a line of a row does not rest on the committed record:\n\n  {}\n\na row is lifted \
         from the record quote for quote, and a line changes only with the record, in one \
         commit",
        wrong.join("\n  ")
    );
}

// ---------------------------------------------------------------------------
// The arms of the comparison — manifests as data
// ---------------------------------------------------------------------------

const README: &str = "README.md";

fn put(manifest: &mut Manifest, path: &str, bytes: &str) {
    manifest.files.insert(path.to_string(), Entry::file(bytes));
}

fn dir(manifest: &mut Manifest, path: &str) {
    manifest.files.insert(path.to_string(), Entry::Dir(0o755));
}

fn staged(bytes: &str) -> Staged {
    Staged {
        mode: 0o100_644,
        blob: blob_of(bytes),
        flags: String::new(),
    }
}

/// A sandbox as a manifest: a repository on `main` with one commit, which holds
/// `README.md`; `home/`, `outside/` and `tmp/` beside it.
fn sandbox() -> Manifest {
    let mut manifest = Manifest {
        hooks: format!("{REPO}/.git/hooks"),
        ..Manifest::default()
    };
    for place in [REPO, "home", "outside", "tmp"] {
        dir(&mut manifest, place);
    }
    put(&mut manifest, "repo/README.md", "readme\n");
    manifest.checkouts.insert(
        REPO.to_string(),
        Checkout {
            head: String::new(),
            branch: Some("main".to_string()),
            index: [(README.to_string(), staged("readme\n"))].into(),
        },
    );
    commit(
        &mut manifest,
        "c0",
        &[(README, Some((0o100_644, "readme\n")))],
    );
    manifest
}

/// A commit on the main checkout's branch: `id` over the `HEAD` that stands, changing
/// these paths — its objects in the store and within reach, the index at its paths as the
/// commit has them.
fn commit(manifest: &mut Manifest, id: &str, changed: &[(&str, Option<(u32, &str)>)]) {
    let main = manifest.checkouts.get_mut(REPO).expect("the main checkout");
    let parent = Some(main.head.clone()).filter(|head| !head.is_empty());
    main.head = id.to_string();
    for (path, entry) in changed {
        match entry {
            Some((mode, bytes)) => main.index.insert(
                path.to_string(),
                Staged {
                    mode: *mode,
                    ..staged(bytes)
                },
            ),
            None => main.index.remove(*path),
        };
    }
    let changed: Vec<(String, Option<(u32, String)>)> = changed
        .iter()
        .map(|(path, entry)| {
            (
                path.to_string(),
                entry.map(|(mode, bytes)| (mode, blob_of(bytes))),
            )
        })
        .collect();
    for (_, entry) in changed.iter() {
        if let Some((mode, blob)) = entry
            && mode & 0o170_000 == 0o100_000
        {
            let bytes = blob.strip_prefix("blob:").unwrap_or_default();
            manifest
                .blobs
                .insert(blob.clone(), bytes.as_bytes().to_vec());
        }
    }
    for id in changed
        .iter()
        .filter_map(|(_, entry)| Some(entry.as_ref()?.1.clone()))
        .chain([id.to_string()])
    {
        manifest.objects.insert(id.clone());
        manifest.held.insert(id);
    }
    manifest
        .refs
        .insert("refs/heads/main".to_string(), id.to_string());
    manifest.commits.insert(
        id.to_string(),
        Commit {
            parent,
            changed: changed.into_iter().collect(),
        },
    );
}

/// Milestone records, committed in one commit over the `HEAD` that stands: under `root`, a
/// directory from the repository's root (`""` for the root itself), one record a milestone,
/// each naming its sub-tasks as items `### <id>  {#<id>}` — and its shared base, as
/// [`a_records_head`] has it.
fn records(manifest: &mut Manifest, root: &str, milestones: &[(&str, &[&str])]) {
    let home = match root {
        "" => "milestone-records".to_string(),
        root => format!("{root}/milestone-records"),
    };
    let mut at = REPO.to_string();
    for component in home.split('/') {
        at = format!("{at}/{component}");
        dir(manifest, &at);
    }
    let head = a_records_head(manifest);
    let written: Vec<(String, String)> = milestones
        .iter()
        .map(|(id, tasks)| {
            let mut bytes = format!("{head}# {id}\n\n## Tasks\n");
            for task in *tasks {
                bytes.push_str(&format!(
                    "\n### {task}  {{#{task}}}\n\n<!-- fields -->\n- intent: an intent\n"
                ));
            }
            (format!("{home}/{id}.md"), bytes)
        })
        .collect();
    for (path, bytes) in &written {
        put(manifest, &format!("{REPO}/{path}"), bytes);
    }
    let changed: Vec<(&str, Option<(u32, &str)>)> = written
        .iter()
        .map(|(path, bytes)| (path.as_str(), Some((0o100_644, bytes.as_str()))))
        .collect();
    let id = format!("records-{}", manifest.commits.len());
    commit(manifest, &id, &changed);
}

/// **A record's front matter**, as `jigc milestone create` writes it: the shared base it
/// pins — the main checkout's `HEAD` where the record is written, the sandbox's first commit
/// over [`sandbox`] — then its status.
fn a_records_head(manifest: &Manifest) -> String {
    let head = manifest
        .checkouts
        .get(REPO)
        .map(|main| main.head.as_str())
        .unwrap_or_default();
    format!("---\nbase: {head} {head}\nstatus: active\n---\n\n")
}

/// `manifest` with where each marker stands read off it, as the frame's reader would: the
/// files whose bytes hold it, an object within reach, an object of the store — a
/// hand-built id carries its bytes.
fn read(mut manifest: Manifest, markers: &[&str]) -> Manifest {
    manifest.markers = markers
        .iter()
        .map(|marker| {
            let holds = |bytes: &[u8]| {
                !marker.is_empty()
                    && bytes
                        .windows(marker.len())
                        .any(|at| at == marker.as_bytes())
            };
            let files = manifest
                .files
                .iter()
                .filter(|(_, entry)| matches!(entry, Entry::File { bytes, .. } if holds(bytes)))
                .map(|(path, _)| path.clone())
                .collect();
            (
                marker.to_string(),
                Stands {
                    files,
                    held: manifest.held.iter().any(|id| holds(id.as_bytes())),
                },
            )
        })
        .collect();
    manifest
}

/// `manifest`, changed.
fn then(manifest: &Manifest, change: impl FnOnce(&mut Manifest)) -> Manifest {
    let mut after = manifest.clone();
    change(&mut after);
    after
}

/// One comparison, as an arm states it.
#[derive(Clone, Copy)]
struct Cell<'a> {
    row: Row,
    /// Before the lead-ins; `None` where the cell has none, and it is `before`.
    base: Option<&'a Manifest>,
    before: &'a Manifest,
    after: &'a Manifest,
    /// The plants: where each stands, and its marker.
    planted: &'a [(&'a str, &'a str)],
    code: Option<i32>,
    said: &'a str,
    consent: bool,
    listed: Listed,
    /// The command, as the row's [`Unit`] reads its work unit off it.
    argv: &'a [&'a str],
    /// The spelling of the leaf the command ran under.
    spelling: Option<&'a str>,
}

impl<'a> Cell<'a> {
    /// A cell with no plant and no lead-in, whose command exited 0 as listed.
    fn of(row: Row, before: &'a Manifest, after: &'a Manifest) -> Cell<'a> {
        Cell {
            row,
            base: None,
            before,
            after,
            planted: &[],
            code: Some(0),
            said: "",
            consent: false,
            listed: Listed::Acts,
            argv: &[],
            spelling: None,
        }
    }

    fn verdict(&self) -> Result<byte_cell::Verdict, String> {
        let markers: Vec<&str> = self.planted.iter().map(|(_, marker)| *marker).collect();
        let planted: Vec<Planted> = self
            .planted
            .iter()
            .map(|(at, marker)| Planted {
                at: at.to_string(),
                marker: marker.to_string(),
                staged: false,
            })
            .collect();
        let (before, after) = (
            read(self.before.clone(), &markers),
            read(self.after.clone(), &markers),
        );
        byte_cell::compare(
            &self.row,
            &Seen {
                base: self.base.unwrap_or(&before),
                before: &before,
                after: &after,
                planted: &planted,
                argv: self.argv,
                spelling: self.spelling,
                ended: Ended {
                    ended: true,
                    code: self.code,
                    said: self.said,
                },
                consent: self.consent,
                listed: self.listed,
            },
        )
    }

    /// What the cell came to, with why — a fault of the cell fails the arm.
    fn came_to(&self) -> (Classification, Vec<String>) {
        let verdict = self
            .verdict()
            .expect("the cell is one the frame can compare");
        (verdict.classification, verdict.why)
    }
}

/// `cell` comes out as `expected`.
#[track_caller]
fn comes_out(cell: &Cell<'_>, expected: Classification) {
    let (got, why) = cell.came_to();
    assert_eq!(got, expected, "{why:#?}");
}

const fn line(id: &'static str, allows: Allows) -> Line {
    Line {
        id,
        allows,
        when: When::Always,
        on_refusal: false,
        quotes: &[],
        spelling: None,
    }
}

const fn row(lines: &'static [Line]) -> Row {
    Row {
        leaf: &["probe"],
        unit: Unit::Nothing,
        spellings: &[],
        consent: Some("--force"),
        home: Prepared::Bare,
        lines,
    }
}

/// A row that lets its command do nothing.
const NOTHING: Row = row(&[]);

/// A row that lets its command remove everything under `.jigc/`.
const REMOVES: Row = row(&[line(
    "R1",
    Allows::Files(Where::Under(".jigc"), May::Removed),
)]);

const MARKER: &str = "byte-table-plant-0";

/// The sandbox with a workbench: `.jigc/`, and a file somebody planted under it.
fn planted_under_the_workbench() -> Manifest {
    then(&sandbox(), |m| {
        dir(m, "repo/.jigc");
        dir(m, "repo/.jigc/logs");
        put(m, "repo/.jigc/version", "jigc-version: 1\n");
        put(m, "repo/.jigc/logs/notes.md", MARKER);
    })
}

const PLANT: &[(&str, &str)] = &[("repo/.jigc/logs/notes.md", MARKER)];

/// **A plant that is gone is lost whatever the row says.** A file somebody planted at a
/// place a line lets the command remove is gone at exit 0, and its bytes stand nowhere:
/// *lost* — the row's line does not excuse it. Under the consent spelling, with the path
/// named in the command's output, the removal is the line's to judge and holds; a consent
/// whose output names nothing is no exemption; and neither is a flag on a row that
/// declares no consent.
#[test]
fn a_plant_that_is_gone_is_lost_whatever_the_row_says() {
    let before = planted_under_the_workbench();
    let after = then(&before, |m| {
        m.files.retain(|path, _| !path.starts_with("repo/.jigc"));
    });
    let lost = Cell {
        planted: PLANT,
        ..Cell::of(REMOVES, &before, &after)
    };
    comes_out(&lost, Classification::Lost);
    let named = Cell {
        planted: PLANT,
        consent: true,
        said: "removed .jigc/logs/notes.md, which no index has a copy of\n",
        ..Cell::of(REMOVES, &before, &after)
    };
    comes_out(&named, Classification::Holds);
    comes_out(
        &Cell {
            said: "removed the workbench\n",
            ..named
        },
        Classification::Lost,
    );
    const NO_CONSENT: Row = Row {
        consent: None,
        ..REMOVES
    };
    comes_out(
        &Cell {
            row: NO_CONSENT,
            ..named
        },
        Classification::Lost,
    );
    // The plant untouched, the rest removed under the line: nothing is lost.
    let spared = then(&before, |m| {
        m.files.remove("repo/.jigc/version");
    });
    comes_out(
        &Cell {
            planted: PLANT,
            ..Cell::of(REMOVES, &before, &spared)
        },
        Classification::Holds,
    );
}

/// **Bytes that stand elsewhere are not lost.** A planted file moved to another path is
/// *written outside its allowance* — the bytes are there; under a line that moves the
/// place aside to that directory it holds; and where the bytes are gone from every path
/// and a commit within reach still has them, something was put in the plant's place:
/// *git's state moved outside its allowance*.
#[test]
fn bytes_that_stand_elsewhere_are_not_lost() {
    const ASIDE: Row = row(&[
        line(
            "A1",
            Allows::Files(Where::Under(".jigc/tasks/*"), May::Removed),
        ),
        line(
            "A2",
            Allows::Files(
                Where::Under(".jigc/tasks/*"),
                May::MovedAsideTo(".jigc/displaced"),
            ),
        ),
    ]);
    let before = then(&sandbox(), |m| {
        for path in ["repo/.jigc", "repo/.jigc/tasks", "repo/.jigc/tasks/t1"] {
            dir(m, path);
        }
        put(m, "repo/.jigc/tasks/t1/base.json", "{}\n");
        put(m, "repo/.jigc/tasks/t1/notes.md", MARKER);
    });
    let planted: &[(&str, &str)] = &[("repo/.jigc/tasks/t1/notes.md", MARKER)];
    let moved = then(&before, |m| {
        m.files
            .retain(|path, _| !path.starts_with("repo/.jigc/tasks/t1"));
        for path in ["repo/.jigc/displaced", "repo/.jigc/displaced/t1"] {
            dir(m, path);
        }
        put(m, "repo/.jigc/displaced/t1/notes.md", MARKER);
    });
    comes_out(
        &Cell {
            planted,
            ..Cell::of(REMOVES, &before, &moved)
        },
        Classification::WrittenOutside,
    );
    let aside = Cell {
        planted,
        ..Cell::of(ASIDE, &before, &moved)
    };
    comes_out(&aside, Classification::Holds);
    let excused = aside.verdict().expect("a cell").excused;
    assert_eq!(
        excused,
        [("A1", 2), ("A2", 3)],
        "the area and its file; the plant and its way"
    );

    // A planted, committed file, and a commit that puts other bytes in its place.
    const LANDS: Row = Row {
        unit: Unit::TheWordAfterTheLeaf,
        ..row(&[
            line(
                "L1",
                Allows::Files(Where::AStagedDocsHome, May::CopyOfAStagedDoc),
            ),
            line("L2", Allows::Commit(Holding::TheIndex)),
        ])
    };
    let before = then(&sandbox(), |m| {
        put(m, "repo/docs.md", MARKER);
        commit(m, "c1", &[("docs.md", Some((0o100_644, MARKER)))]);
        for path in [
            "repo/.jigc",
            "repo/.jigc/tasks",
            "repo/.jigc/tasks/t1",
            "repo/.jigc/tasks/t1/docs",
        ] {
            dir(m, path);
        }
        put(
            m,
            "repo/.jigc/tasks/t1/docs/docs.md",
            "the unit's own doc\n",
        );
    });
    let landed = then(&before, |m| {
        put(m, "repo/docs.md", "the unit's own doc\n");
        commit(
            m,
            "c2",
            &[("docs.md", Some((0o100_644, "the unit's own doc\n")))],
        );
    });
    comes_out(
        &Cell {
            planted: &[("repo/docs.md", MARKER)],
            argv: &["probe", "t1"],
            ..Cell::of(LANDS, &before, &landed)
        },
        Classification::GitMoved,
    );
}

/// **A file moved aside lands where its line names, and a component of that place may stand
/// for a sub-task of the unit** — the human's ruling of 2026-10-19 on the rows' record's §6,
/// entry 18: before removing a landed sub-task's worktree, the boundary moves its unstaged
/// and untracked files that git does not ignore into `.jigc/displaced/<sub-task-id>/`. On
/// `milestone finalize`'s own row: a file of the user's in a sub-task's worktree, moved to
/// that sub-task's place under `.jigc/displaced/`, holds; moved under an id the record does
/// not name, or loose at the directory's root, it is *written outside its allowance*; gone,
/// it is *lost* — the line moves bytes and never lets them go. **And under its own unit's
/// place, not a sibling's** (the review of `B8`, its H10, red first: each came out
/// *holds*): a sub-task's worktree or area, the milestone's area and a task's area, each
/// moved under another unit's place there — a unit the record names — is written outside.
#[test]
fn a_file_moved_aside_lands_under_its_units_place() {
    let before = then(&sandbox(), |m| {
        dir(m, "repo/.jigc");
        records(m, "docs", &[("m", &["s1"])]);
        for path in ["repo/.jigc/worktrees", "repo/.jigc/worktrees/s1"] {
            dir(m, path);
        }
        put(m, "repo/.jigc/worktrees/s1/notes.txt", MARKER);
    });
    let moved_to = |to: Option<&str>| {
        then(&before, |m| {
            m.files
                .retain(|path, _| !path.starts_with("repo/.jigc/worktrees"));
            let Some(to) = to else {
                return;
            };
            let mut at = REPO.to_string();
            let parts: Vec<&str> = to.split('/').collect();
            for part in &parts[..parts.len() - 1] {
                at = format!("{at}/{part}");
                dir(m, &at);
            }
            put(m, &format!("{REPO}/{to}"), MARKER);
        })
    };
    let came = |after: &Manifest| {
        Cell {
            planted: &[("repo/.jigc/worktrees/s1/notes.txt", MARKER)],
            argv: &["milestone", "finalize", "m"],
            ..Cell::of(milestone::FINALIZE, &before, after)
        }
        .came_to()
    };
    let (class, why) = came(&moved_to(Some(".jigc/displaced/s1/notes.txt")));
    assert_eq!(
        class,
        Classification::Holds,
        "moved to its sub-task's place: {why:#?}"
    );
    for elsewhere in [".jigc/displaced/s2/notes.txt", ".jigc/displaced/notes.txt"] {
        let (class, why) = came(&moved_to(Some(elsewhere)));
        assert_eq!(
            class,
            Classification::WrittenOutside,
            "moved to `{elsewhere}`: {why:#?}"
        );
    }
    let (class, why) = came(&moved_to(None));
    assert_eq!(class, Classification::Lost, "gone: {why:#?}");

    // **Its own unit's place, not a sibling's** (the review of `B8`, its H10, red first:
    // each came out *holds*). A place under `displaced/` names the unit whose area or
    // worktree the bytes left — *"to `.jigc/displaced/<task-id>/` with its relative paths
    // intact"*, *"`.jigc/displaced/<unit-id>/<relative>`"* — so one unit's leftover under
    // another's place, a unit the record names, is written outside; and so is it on the
    // three lines that moved an area aside to `displaced/` at large: a task's area, a
    // sub-task's area, the milestone's own area.
    let moved_from = |before: &Manifest, from: &str, to: &str| {
        then(before, |m| {
            m.files.retain(|path, _| !path.starts_with(from));
            let mut at = REPO.to_string();
            let parts: Vec<&str> = to.split('/').collect();
            for part in &parts[..parts.len() - 1] {
                at = format!("{at}/{part}");
                if !m.files.contains_key(&at) {
                    dir(m, &at);
                }
            }
            put(m, &format!("{REPO}/{to}"), MARKER);
        })
    };
    let siblings = then(&sandbox(), |m| {
        dir(m, "repo/.jigc");
        records(m, "docs", &[("m", &["s1", "s2"])]);
        for path in [
            "repo/.jigc/worktrees",
            "repo/.jigc/worktrees/s1",
            "repo/.jigc/tasks",
            "repo/.jigc/tasks/s1",
            "repo/.jigc/milestones",
            "repo/.jigc/milestones/m",
        ] {
            dir(m, path);
        }
    });
    let task_area = then(&sandbox(), |m| {
        for path in ["repo/.jigc", "repo/.jigc/tasks", "repo/.jigc/tasks/t1"] {
            dir(m, path);
        }
        put(m, "repo/.jigc/tasks/t1/base.json", "{}\n");
    });
    let leftovers = [
        (
            "a sub-task's worktree",
            &milestone::FINALIZE,
            &["milestone", "finalize", "m"],
            &siblings,
            ".jigc/worktrees/s1/notes.txt",
            ".jigc/displaced/s1/notes.txt",
            ".jigc/displaced/s2/notes.txt",
        ),
        (
            "a sub-task's area",
            &milestone::FINALIZE,
            &["milestone", "finalize", "m"],
            &siblings,
            ".jigc/tasks/s1/notes.txt",
            ".jigc/displaced/s1/notes.txt",
            ".jigc/displaced/s2/notes.txt",
        ),
        (
            "the milestone's area",
            &milestone::FINALIZE,
            &["milestone", "finalize", "m"],
            &siblings,
            ".jigc/milestones/m/notes.txt",
            ".jigc/displaced/m/notes.txt",
            ".jigc/displaced/s1/notes.txt",
        ),
        (
            "a task's area",
            &task::FINALIZE,
            &["task", "finalize", "t1"],
            &task_area,
            ".jigc/tasks/t1/notes.txt",
            ".jigc/displaced/t1/notes.txt",
            ".jigc/displaced/t2/notes.txt",
        ),
    ];
    for (case, row, argv, before, from, own, sibling) in leftovers {
        let at = format!("{REPO}/{from}");
        let before = then(before, |m| put(m, &at, MARKER));
        let area = format!("{REPO}/{}", from.rsplit_once('/').expect("a file").0);
        let planted: &[(&str, &str)] = &[(at.as_str(), MARKER)];
        for (to, want) in [
            (own, Classification::Holds),
            (sibling, Classification::WrittenOutside),
        ] {
            let after = moved_from(&before, &area, to);
            let (class, why) = Cell {
                planted,
                argv,
                ..Cell::of(*row, &before, &after)
            }
            .came_to();
            assert_eq!(class, want, "{case}, moved to `{to}`: {why:#?}");
        }
        // **With its relative path intact** (the seventh fix's left-open, red first: each red
        // case came out *holds*) — *"to `.jigc/displaced/<task-id>/` with its relative paths
        // intact"*: under its own place at another name it is written outside; at its name
        // with a `.<n>` beside it, where nothing stood at the name, the same; where an earlier
        // displacement stood at the name — the product's no-clobber rule,
        // `free_displacement_path` — it holds.
        let (dir_of, _) = own.rsplit_once('/').expect("a file");
        let renamed = format!("{dir_of}/other.txt");
        let suffixed = format!("{own}.2");
        let earlier = then(&before, |m| written(m, own, "an earlier displacement\n"));
        for (before, to, want) in [
            (&before, renamed.as_str(), Classification::WrittenOutside),
            (&before, suffixed.as_str(), Classification::WrittenOutside),
            (&earlier, suffixed.as_str(), Classification::Holds),
        ] {
            let after = moved_from(before, &area, to);
            let (class, why) = Cell {
                planted,
                argv,
                ..Cell::of(*row, before, &after)
            }
            .came_to();
            assert_eq!(class, want, "{case}, moved to `{to}`: {why:#?}");
        }
    }
}

/// A sandbox with these committed files, under the directories they need — the movers
/// family's fixtures: `files` is `(path from the repository's root, bytes)`.
fn with_committed(files: &[(&str, &str)]) -> Manifest {
    then(&sandbox(), |m| {
        for (path, bytes) in files {
            let mut at = REPO.to_string();
            let parts: Vec<&str> = path.split('/').collect();
            for part in &parts[..parts.len() - 1] {
                at = format!("{at}/{part}");
                dir(m, &at);
            }
            put(m, &format!("{REPO}/{path}"), bytes);
        }
        let changed: Vec<(&str, Option<(u32, &str)>)> = files
            .iter()
            .map(|(path, bytes)| (*path, Some((0o100_644, *bytes))))
            .collect();
        commit(m, "c1", &changed);
    })
}

/// `manifest` with a file written in no commit, under the directories it needs.
fn written(manifest: &mut Manifest, path: &str, bytes: &str) {
    let mut at = REPO.to_string();
    let parts: Vec<&str> = path.split('/').collect();
    for part in &parts[..parts.len() - 1] {
        at = format!("{at}/{part}");
        if !manifest.files.contains_key(&at) {
            dir(manifest, &at);
        }
    }
    put(manifest, &format!("{REPO}/{path}"), bytes);
}

/// `manifest` with `from` moved to `to` in the working tree and git's index — a `git mv`,
/// staged and committed by nobody.
fn git_mv(manifest: &mut Manifest, from: &str, to: &str, bytes: &str) {
    manifest.files.remove(&format!("{REPO}/{from}"));
    written(manifest, to, bytes);
    let main = manifest.checkouts.get_mut(REPO).expect("the main checkout");
    main.index.remove(from);
    main.index.insert(to.to_string(), staged(bytes));
}

/// **A doc `jigc rename` moves is retitled where its home is, and every doc that referred
/// to it repointed** — `RN1`, `RN1, at its new home` and `RN2` of the movers family's row
/// ([`movers::RENAME`]), the words *retitled* and *repointed* (the task `B8`). On the row
/// itself: the doc moved to the slug its `--to` title gives, in its home directory, with its
/// `# ` title the `--to` title and every other line as it was, a referrer's ref-field
/// holding the new identity, all of it in one commit, holds; so does the retitle-only arm,
/// in place, a ref-field list re-emitted in its canonical form with its other element as
/// it was, and a `--slug` that names the slug. A body line changed, a home in another
/// doctype's directory, a title other than `--to`'s, **a slug other than the one the title
/// gives** (the review of `B8`, H11), two copies, the doc gone with no new home, a
/// referrer's other line changed, a file of no doctype's home rewritten for its mention,
/// **a ref to another doc whose id the old one prefixes rewritten, and a mention in a
/// referrer's prose rewritten** (H4) are each red; so is the doc landing over a file of the
/// user's at its new home. And the new home is a line's place (H7): what squats it — a
/// file of the user's, one committed, a link, a directory — is what the row needs a cell
/// for there.
#[test]
fn a_renamed_doc_is_retitled_and_its_referrers_repointed() {
    const ONE: &str =
        "---\nschema-version: 2\nstatus: accepted\n---\n\n# One way\n\n## Context\n\nWe chose.\n";
    const MOVED: &str = "---\nschema-version: 2\nstatus: accepted\n---\n\n# Single way\n\n## Context\n\nWe chose.\n";
    const TWO: &str = "---\nschema-version: 2\nstatus: accepted\nsupersedes: [adr:one-way]\n---\n\n# Two ways\n\nAn aside. See adr:one-way in passing.\n";
    const REPOINTED: &str = "---\nschema-version: 2\nstatus: accepted\nsupersedes: [adr:single-way]\n---\n\n# Two ways\n\nAn aside. See adr:one-way in passing.\n";
    const THREE: &str = "---\nschema-version: 2\nstatus: accepted\nsupersedes: [adr:one-way-two]\n---\n\n# Three ways\n\nNone of it.\n";
    const FOUR: &str = "---\nschema-version: 2\nstatus: accepted\nsupersedes: [adr:old,adr:one-way]\n---\n\n# Four ways\n\nBoth.\n";
    const README_SAYS: &str = "readme\nsee adr:one-way\n";
    let before = with_committed(&[
        ("docs/decisions/one-way.md", ONE),
        ("docs/decisions/two-ways.md", TWO),
        ("docs/decisions/three-ways.md", THREE),
        ("docs/decisions/four-ways.md", FOUR),
        ("README.md", README_SAYS),
    ]);
    let argv: &[&str] = &["rename", "adr:one-way", "--to", "Single way"];
    // `changes`: a path from the repository's root to its bytes after, or to nothing.
    let renamed = |changes: &[(&str, Option<&str>)]| {
        then(&before, |m| {
            for (path, bytes) in changes {
                match bytes {
                    Some(bytes) => written(m, path, bytes),
                    None => {
                        m.files.remove(&format!("{REPO}/{path}"));
                    }
                }
            }
            commit(
                m,
                "c2",
                &changes
                    .iter()
                    .map(|(path, bytes)| (*path, bytes.map(|bytes| (0o100_644, bytes))))
                    .collect::<Vec<_>>(),
            );
        })
    };
    let came = |after: &Manifest, argv: &[&str]| {
        Cell {
            argv,
            ..Cell::of(movers::RENAME, &before, after)
        }
        .came_to()
    };
    let (class, why) = came(
        &renamed(&[
            ("docs/decisions/one-way.md", None),
            ("docs/decisions/single-way.md", Some(MOVED)),
            ("docs/decisions/two-ways.md", Some(REPOINTED)),
        ]),
        argv,
    );
    assert_eq!(class, Classification::Holds, "the rename: {why:#?}");
    let retitled =
        "---\nschema-version: 2\nstatus: accepted\n---\n\n# One Way\n\n## Context\n\nWe chose.\n";
    let (class, why) = came(
        &renamed(&[("docs/decisions/one-way.md", Some(retitled))]),
        &["rename", "adr:one-way", "--to", "One Way"],
    );
    assert_eq!(
        class,
        Classification::Holds,
        "the retitle-only arm: {why:#?}"
    );
    // A list re-emitted in its canonical form — *"swaps the one element and re-emits the
    // canonical whole-list"* — its other element as it was.
    let (class, why) = came(
        &renamed(&[
            ("docs/decisions/one-way.md", None),
            ("docs/decisions/single-way.md", Some(MOVED)),
            ("docs/decisions/two-ways.md", Some(REPOINTED)),
            (
                "docs/decisions/four-ways.md",
                Some(
                    "---\nschema-version: 2\nstatus: accepted\nsupersedes: [adr:old, adr:single-way]\n---\n\n# Four ways\n\nBoth.\n",
                ),
            ),
        ]),
        argv,
    );
    assert_eq!(class, Classification::Holds, "a list re-emitted: {why:#?}");
    // `--slug` names the slug: *"unless `--slug` overrides"*.
    let (class, why) = came(
        &renamed(&[
            ("docs/decisions/one-way.md", None),
            ("docs/decisions/zzz.md", Some(MOVED)),
            (
                "docs/decisions/two-ways.md",
                Some(
                    "---\nschema-version: 2\nstatus: accepted\nsupersedes: [adr:zzz]\n---\n\n# Two ways\n\nAn aside. See adr:one-way in passing.\n",
                ),
            ),
        ]),
        &[
            "rename",
            "adr:one-way",
            "--to",
            "Single way",
            "--slug",
            "zzz",
        ],
    );
    assert_eq!(class, Classification::Holds, "a `--slug`: {why:#?}");
    // A title whose slug drops its edge word — *"a leading or trailing filler word … is
    // dropped"* — gives the doc's own slug: a retitle in place holds, and a move to the
    // slug with the word kept is red.
    const THE_ONE_WAY: &str = "---\nschema-version: 2\nstatus: accepted\n---\n\n# The one way\n\n## Context\n\nWe chose.\n";
    let the_one_way: &[&str] = &["rename", "adr:one-way", "--to", "The one way"];
    let (class, why) = came(
        &renamed(&[("docs/decisions/one-way.md", Some(THE_ONE_WAY))]),
        the_one_way,
    );
    assert_eq!(
        class,
        Classification::Holds,
        "an edge word dropped: {why:#?}"
    );
    let (class, why) = came(
        &renamed(&[
            ("docs/decisions/one-way.md", None),
            ("docs/decisions/the-one-way.md", Some(THE_ONE_WAY)),
            (
                "docs/decisions/two-ways.md",
                Some(
                    "---\nschema-version: 2\nstatus: accepted\nsupersedes: [adr:the-one-way]\n---\n\n# Two ways\n\nAn aside. See adr:one-way in passing.\n",
                ),
            ),
        ]),
        the_one_way,
    );
    assert!(
        class.is_red(),
        "the slug with its edge word kept: came out {class}, {why:#?}"
    );
    // The new home is a line's place, and what squats it is what the row needs a cell for.
    let at_the_new_home: Vec<String> = needs(&movers::RENAME)
        .iter()
        .map(Need::named)
        .filter(|need| need.starts_with("RN1, at its new home × "))
        .collect();
    assert_eq!(
        at_the_new_home,
        [
            "RN1, at its new home × U",
            "RN1, at its new home × T",
            "RN1, at its new home × L, to nothing",
            "RN1, at its new home × L, to a file in the repository",
            "RN1, at its new home × L, to a file outside it",
            "RN1, at its new home × K, a directory in a file's place",
        ],
        "what squats the doc's new home is what the row needs a cell for there"
    );
    // The doc landed over a file of the user's at its new home.
    let squatted = then(&before, |m| {
        written(m, "docs/decisions/single-way.md", "the user's own notes\n");
    });
    let over = then(&squatted, |m| {
        m.files.remove(&format!("{REPO}/docs/decisions/one-way.md"));
        written(m, "docs/decisions/single-way.md", MOVED);
        written(m, "docs/decisions/two-ways.md", REPOINTED);
        commit(
            m,
            "c2",
            &[
                ("docs/decisions/one-way.md", None),
                ("docs/decisions/single-way.md", Some((0o100_644, MOVED))),
                ("docs/decisions/two-ways.md", Some((0o100_644, REPOINTED))),
            ],
        );
    });
    let (class, why) = Cell {
        argv,
        ..Cell::of(movers::RENAME, &squatted, &over)
    }
    .came_to();
    assert!(
        class.is_red(),
        "the doc landed over a file of the user's: came out {class}, {why:#?}"
    );
    for (case, changes) in [
        (
            "a body line changed",
            vec![
                ("docs/decisions/one-way.md", None),
                (
                    "docs/decisions/single-way.md",
                    Some(
                        "---\nschema-version: 2\nstatus: accepted\n---\n\n# Single way\n\n## Context\n\nWe chose otherwise.\n",
                    ),
                ),
            ],
        ),
        (
            "another doctype's directory",
            vec![
                ("docs/decisions/one-way.md", None),
                ("docs/specs/single-way.md", Some(MOVED)),
            ],
        ),
        (
            "a title other than `--to`'s",
            vec![
                ("docs/decisions/one-way.md", None),
                (
                    "docs/decisions/single-way.md",
                    Some(
                        "---\nschema-version: 2\nstatus: accepted\n---\n\n# Another way\n\n## Context\n\nWe chose.\n",
                    ),
                ),
            ],
        ),
        (
            "two copies",
            vec![("docs/decisions/single-way.md", Some(MOVED))],
        ),
        ("the doc gone", vec![("docs/decisions/one-way.md", None)]),
        (
            "a referrer's other line changed",
            vec![
                ("docs/decisions/one-way.md", None),
                ("docs/decisions/single-way.md", Some(MOVED)),
                (
                    "docs/decisions/two-ways.md",
                    Some(
                        "---\nschema-version: 2\nstatus: accepted\nsupersedes: [adr:single-way]\n---\n\n# Two ways\n\nAnother aside.\n",
                    ),
                ),
            ],
        ),
        (
            "a file of no doctype's home rewritten",
            vec![
                ("docs/decisions/one-way.md", None),
                ("docs/decisions/single-way.md", Some(MOVED)),
                ("README.md", Some("readme\nsee adr:single-way\n")),
            ],
        ),
        (
            "a slug other than the one the title gives",
            vec![
                ("docs/decisions/one-way.md", None),
                ("docs/decisions/zzz.md", Some(MOVED)),
                (
                    "docs/decisions/two-ways.md",
                    Some(
                        "---\nschema-version: 2\nstatus: accepted\nsupersedes: [adr:zzz]\n---\n\n# Two ways\n\nAn aside. See adr:one-way in passing.\n",
                    ),
                ),
            ],
        ),
        (
            "a ref to another doc the old id prefixes rewritten",
            vec![
                ("docs/decisions/one-way.md", None),
                ("docs/decisions/single-way.md", Some(MOVED)),
                ("docs/decisions/two-ways.md", Some(REPOINTED)),
                (
                    "docs/decisions/three-ways.md",
                    Some(
                        "---\nschema-version: 2\nstatus: accepted\nsupersedes: [adr:single-way-two]\n---\n\n# Three ways\n\nNone of it.\n",
                    ),
                ),
            ],
        ),
        (
            "a mention in a referrer's prose rewritten",
            vec![
                ("docs/decisions/one-way.md", None),
                ("docs/decisions/single-way.md", Some(MOVED)),
                (
                    "docs/decisions/two-ways.md",
                    Some(
                        "---\nschema-version: 2\nstatus: accepted\nsupersedes: [adr:single-way]\n---\n\n# Two ways\n\nAn aside. See adr:single-way in passing.\n",
                    ),
                ),
            ],
        ),
        (
            "a list's other element changed",
            vec![
                ("docs/decisions/one-way.md", None),
                ("docs/decisions/single-way.md", Some(MOVED)),
                ("docs/decisions/two-ways.md", Some(REPOINTED)),
                (
                    "docs/decisions/four-ways.md",
                    Some(
                        "---\nschema-version: 2\nstatus: accepted\nsupersedes: [adr:older, adr:single-way]\n---\n\n# Four ways\n\nBoth.\n",
                    ),
                ),
            ],
        ),
    ] {
        let (class, why) = came(&renamed(&changes), argv);
        assert!(class.is_red(), "{case}: came out {class}, {why:#?}");
    }
}

/// **A doc a mover moves lands at its doctype's home, the same entry** — `RL1` of `jigc
/// relocate`'s row ([`movers::RELOCATE`]) and `CS2` of `jigc config set`'s
/// ([`movers::CONFIG_SET`]), the word *into its home* (the task `B8`), read at both ends since
/// the human's ruling of 2026-10-20 on the rows' record's §6, entry 23 (the task `B9`): the
/// departure at the line's place, the arrival at the destination, `<a-new-home>`, a line's
/// place of its own (`RL1, at its new home`, `CS2, at its new home`). On the rows themselves:
/// a stranded `note` under `--from` moved into the home a listed pack declares for it,
/// staged and committed by nobody, holds; moved anywhere else, with other bytes, or from
/// outside `--from`, it is red; a line of the user's planted in the doc moves with it, and
/// holds. **Nothing is moved aside from a destination, and what stands there blocks the
/// move**: a file of the user's at the destination parked — by its name, under another,
/// beside an earlier parking — a directory parked whole, an occupant parked whether the
/// file-state baseline records it or not, a committed doc no move concerns parked, and a
/// move that lands where git held a file the working tree did not, are each red; a row that
/// parks a destination's occupant is red over the parking; and a refusal that leaves the
/// note and its occupant as they stood changes nothing. **A sweep does all it can, then
/// refuses naming what it left** (the human's ruling R1 of that day): a refusal that moved
/// one note and left another as it stood, naming it or the path its move would have landed
/// at, changes nothing a refusal may not; naming nothing it left, or with the left note
/// rewritten, it is red. A `docs-root` re-point that parks the doc it should move is red. A
/// `placement-root` re-point moving the roadmap under the new root, the knob set, holds; to
/// another directory, it is red.
#[test]
fn a_moved_doc_lands_at_its_doctypes_home() {
    const NOTE: &str = "# Cache benchmarks\n\n## Body\n\nA single node caps throughput.\n";
    const SCHEMA: &str = "type: note\nlocation: notes/\nid-from: title\nsections:\n  - id: body\n    slot: { hint: \"The note.\" }\n";
    let before = then(
        &with_committed(&[
            ("docs/legacy-notes/cache-benchmarks.md", NOTE),
            ("docs/else/stays.md", NOTE),
        ]),
        |m| {
            written(
                m,
                ".jigc/config/packs.yaml",
                "packs:\n  - .jigc/note-pack\n",
            );
            written(m, ".jigc/note-pack/schemas/note.yaml", SCHEMA);
        },
    );
    let argv: &[&str] = &["relocate", "note", "--from", "docs/legacy-notes/"];
    let came = |after: &Manifest| {
        Cell {
            argv,
            ..Cell::of(movers::RELOCATE, &before, after)
        }
        .came_to()
    };
    let moved = |to: &str, bytes: &str| {
        then(&before, |m| {
            git_mv(m, "docs/legacy-notes/cache-benchmarks.md", to, bytes);
        })
    };
    let (class, why) = came(&moved("docs/notes/cache-benchmarks.md", NOTE));
    assert_eq!(class, Classification::Holds, "into its home: {why:#?}");
    for (case, after) in [
        (
            "another directory",
            moved("docs/other/cache-benchmarks.md", NOTE),
        ),
        (
            "other bytes",
            moved(
                "docs/notes/cache-benchmarks.md",
                "# Cache benchmarks\n\nOther.\n",
            ),
        ),
        (
            "from outside `--from`",
            then(&before, |m| {
                git_mv(m, "docs/else/stays.md", "docs/notes/stays.md", NOTE)
            }),
        ),
    ] {
        let (class, why) = came(&after);
        assert!(class.is_red(), "{case}: came out {class}, {why:#?}");
    }
    // A line of the user's in the stranded doc, in no commit, moves with it.
    let edited = then(&before, |m| {
        written(
            m,
            "docs/legacy-notes/cache-benchmarks.md",
            &format!("{NOTE}a line of the user's\n"),
        );
    });
    let carried = then(&edited, |m| {
        let bytes = format!("{NOTE}a line of the user's\n");
        m.files
            .remove(&format!("{REPO}/docs/legacy-notes/cache-benchmarks.md"));
        written(m, "docs/notes/cache-benchmarks.md", &bytes);
        let main = m.checkouts.get_mut(REPO).expect("the main checkout");
        main.index.remove("docs/legacy-notes/cache-benchmarks.md");
        main.index
            .insert("docs/notes/cache-benchmarks.md".to_string(), staged(NOTE));
    });
    let (class, why) = Cell {
        argv,
        planted: &[(
            "repo/docs/legacy-notes/cache-benchmarks.md",
            "a line of the user's",
        )],
        ..Cell::of(movers::RELOCATE, &edited, &carried)
    }
    .came_to();
    assert_eq!(
        class,
        Classification::Holds,
        "an edit moved with its doc: {why:#?}"
    );

    // **Nothing is moved aside from a destination, and what stands there blocks the move** —
    // the human's ruling of 2026-10-20 on the rows' record's §6, entry 23 (the task `B9`'s
    // first commit). A file of the user's at the note's new home parked under
    // `.jigc/displaced/` — by its name, under another, beside an earlier parking — is red,
    // and taken it is lost; a directory of theirs parked whole is red. A refusal that leaves
    // the note and the occupant as they stood, naming the path, changes nothing.
    let squatted = then(&before, |m| {
        written(
            m,
            "docs/notes/cache-benchmarks.md",
            "the user's own notes\n",
        );
    });
    let landed = then(&squatted, |m| {
        git_mv(
            m,
            "docs/legacy-notes/cache-benchmarks.md",
            "docs/notes/cache-benchmarks.md",
            NOTE,
        );
    });
    let parked_by_name = then(&landed, |m| {
        written(
            m,
            ".jigc/displaced/cache-benchmarks.md",
            "the user's own notes\n",
        );
    });
    let over = |after: &Manifest| {
        Cell {
            argv,
            planted: &[(
                "repo/docs/notes/cache-benchmarks.md",
                "the user's own notes",
            )],
            ..Cell::of(movers::RELOCATE, &squatted, after)
        }
        .came_to()
    };
    let (class, why) = over(&parked_by_name);
    assert!(
        class.is_red(),
        "the squatter parked: came out {class}, {why:#?}"
    );
    let (class, why) = over(&landed);
    assert_eq!(class, Classification::Lost, "the squatter taken: {why:#?}");
    let earlier = then(&squatted, |m| {
        written(m, ".jigc/displaced/cache-benchmarks.md", "parked before\n");
    });
    for (case, before, to) in [
        ("under another name", &squatted, ".jigc/displaced/other.md"),
        (
            "beside a name nothing held",
            &squatted,
            ".jigc/displaced/cache-benchmarks.md.2",
        ),
        (
            "beside an earlier parking",
            &earlier,
            ".jigc/displaced/cache-benchmarks.md.2",
        ),
    ] {
        let after = then(before, |m| {
            git_mv(
                m,
                "docs/legacy-notes/cache-benchmarks.md",
                "docs/notes/cache-benchmarks.md",
                NOTE,
            );
            written(m, to, "the user's own notes\n");
        });
        let (class, why) = Cell {
            argv,
            planted: &[(
                "repo/docs/notes/cache-benchmarks.md",
                "the user's own notes",
            )],
            ..Cell::of(movers::RELOCATE, before, &after)
        }
        .came_to();
        assert!(
            class.is_red(),
            "the squatter parked {case}: came out {class}, {why:#?}"
        );
    }
    let blocked = Cell {
        argv,
        planted: &[(
            "repo/docs/notes/cache-benchmarks.md",
            "the user's own notes",
        )],
        code: Some(1),
        said: "blocked · docs/notes/cache-benchmarks.md — clear it, then re-run\n",
        listed: Listed::Refuses,
        ..Cell::of(movers::RELOCATE, &squatted, &squatted)
    };
    comes_out(&blocked, Classification::RefusesNothingChanged);
    let directory = then(&before, |m| {
        written(
            m,
            "docs/notes/cache-benchmarks.md/notes.txt",
            "the user's own notes\n",
        );
    });
    let parked = then(&directory, |m| {
        m.files
            .remove(&format!("{REPO}/docs/notes/cache-benchmarks.md/notes.txt"));
        m.files
            .remove(&format!("{REPO}/docs/notes/cache-benchmarks.md"));
        git_mv(
            m,
            "docs/legacy-notes/cache-benchmarks.md",
            "docs/notes/cache-benchmarks.md",
            NOTE,
        );
        written(
            m,
            ".jigc/displaced/cache-benchmarks.md/notes.txt",
            "the user's own notes\n",
        );
    });
    let (class, why) = Cell {
        argv,
        planted: &[(
            "repo/docs/notes/cache-benchmarks.md/notes.txt",
            "the user's own notes",
        )],
        ..Cell::of(movers::RELOCATE, &directory, &parked)
    }
    .came_to();
    assert!(
        class.is_red(),
        "the squatting directory parked: came out {class}, {why:#?}"
    );

    // A committed decision no move concerns, parked beside the note's honest move, is red;
    // so is an occupant of the note's new home parked, the file-state baseline recording it
    // or not; and a move that lands where git held a file the working tree did not — a
    // destination nothing stands at on disk, which the place reads as git holds it too (red
    // first: it came out *holds*).
    const DECISION: &str =
        "---\nschema-version: 2\nstatus: accepted\n---\n\n# One way\n\n## Context\n\nWe chose.\n";
    const MANAGED: &str = "# Cache benchmarks\n\n## Body\n\nThe managed one.\n";
    let with_the_note_pack = |files: &[(&str, &str)], baseline: Option<&str>| {
        then(&with_committed(files), |m| {
            written(
                m,
                ".jigc/config/packs.yaml",
                "packs:\n  - .jigc/note-pack\n",
            );
            written(m, ".jigc/note-pack/schemas/note.yaml", SCHEMA);
            if let Some(path) = baseline {
                written(
                    m,
                    ".jigc/state/file-state.json",
                    &format!("{{\n  \"hashes\": {{\n    \"{path}\": \"0\"\n  }}\n}}\n"),
                );
            }
        })
    };
    let beside = with_the_note_pack(
        &[
            ("docs/legacy-notes/cache-benchmarks.md", NOTE),
            ("docs/decisions/one-way.md", DECISION),
        ],
        None,
    );
    let parked_beside = then(&beside, |m| {
        git_mv(
            m,
            "docs/legacy-notes/cache-benchmarks.md",
            "docs/notes/cache-benchmarks.md",
            NOTE,
        );
        m.files.remove(&format!("{REPO}/docs/decisions/one-way.md"));
        written(m, ".jigc/displaced/one-way.md", DECISION);
        let main = m.checkouts.get_mut(REPO).expect("the main checkout");
        main.index.remove("docs/decisions/one-way.md");
    });
    let (class, why) = Cell {
        argv,
        ..Cell::of(movers::RELOCATE, &beside, &parked_beside)
    }
    .came_to();
    assert!(
        class.is_red(),
        "a decision no move concerns, parked: came out {class}, {why:#?}"
    );
    for (case, baseline) in [
        ("an occupant with no baseline", None),
        ("a managed occupant", Some("docs/notes/cache-benchmarks.md")),
    ] {
        let occupied = with_the_note_pack(
            &[
                ("docs/legacy-notes/cache-benchmarks.md", NOTE),
                ("docs/notes/cache-benchmarks.md", MANAGED),
            ],
            baseline,
        );
        let parked = then(&occupied, |m| {
            git_mv(
                m,
                "docs/legacy-notes/cache-benchmarks.md",
                "docs/notes/cache-benchmarks.md",
                NOTE,
            );
            written(m, ".jigc/displaced/cache-benchmarks.md", MANAGED);
        });
        let (class, why) = Cell {
            argv,
            ..Cell::of(movers::RELOCATE, &occupied, &parked)
        }
        .came_to();
        assert!(class.is_red(), "{case}, parked: came out {class}, {why:#?}");
    }
    let held = then(
        &with_the_note_pack(
            &[
                ("docs/legacy-notes/cache-benchmarks.md", NOTE),
                ("docs/notes/cache-benchmarks.md", MANAGED),
            ],
            None,
        ),
        |m| {
            m.files
                .remove(&format!("{REPO}/docs/notes/cache-benchmarks.md"));
        },
    );
    let landed_over_it = then(&held, |m| {
        git_mv(
            m,
            "docs/legacy-notes/cache-benchmarks.md",
            "docs/notes/cache-benchmarks.md",
            NOTE,
        );
    });
    let (class, why) = Cell {
        argv,
        ..Cell::of(movers::RELOCATE, &held, &landed_over_it)
    }
    .came_to();
    assert!(
        class.is_red(),
        "a move that lands where git held a file: came out {class}, {why:#?}"
    );
    // **A line that moves aside from one file's place covers nothing**: a row that parks a
    // destination's occupant (`RL3` until this ruling) is red over the parking.
    const PARKS: Row = Row {
        lines: &[
            movers::RELOCATE.lines[0],
            movers::RELOCATE.lines[1],
            movers::RELOCATE.lines[2],
            line(
                "RL3",
                Allows::Files(
                    Where::Path(byte_cell::A_DOC),
                    May::MovedAsideTo(".jigc/displaced"),
                ),
            ),
        ],
        ..movers::RELOCATE
    };
    let (class, why) = Cell {
        argv,
        planted: &[(
            "repo/docs/notes/cache-benchmarks.md",
            "the user's own notes",
        )],
        ..Cell::of(PARKS, &squatted, &parked_by_name)
    }
    .came_to();
    assert!(
        class.is_red(),
        "a row that parks a destination's occupant: came out {class}, {why:#?}"
    );

    // **A sweep does all it can, then refuses naming what it left** — the human's rulings of
    // 2026-10-20, R1 and entry 23 (red first: the refusal that named the doc it left came
    // out *refused, and changed what a refusal may not*): two stranded notes, the second
    // moved into its home and the first left as it stood, at exit 1 — naming the first, or
    // the path its move would have landed at, it changes nothing a refusal may not; naming
    // nothing it left, or with the first rewritten, it is red.
    const SECOND: &str = "# Load tests\n\n## Body\n\nTwo nodes hold the load.\n";
    let two = with_the_note_pack(
        &[
            ("docs/legacy-notes/cache-benchmarks.md", NOTE),
            ("docs/legacy-notes/load-tests.md", SECOND),
        ],
        None,
    );
    let swept = then(&two, |m| {
        git_mv(
            m,
            "docs/legacy-notes/load-tests.md",
            "docs/notes/load-tests.md",
            SECOND,
        );
    });
    let rewritten = then(&swept, |m| {
        written(
            m,
            "docs/legacy-notes/cache-benchmarks.md",
            "# Cache benchmarks\n\nOther.\n",
        );
    });
    for (case, after, said, want) in [
        (
            "naming the doc it left",
            &swept,
            "blocked · docs/legacy-notes/cache-benchmarks.md: not a regular file\n",
            Classification::RefusesNothingChanged,
        ),
        (
            "naming the path to clear",
            &swept,
            "blocked · docs/notes/cache-benchmarks.md stands at the destination\n",
            Classification::RefusesNothingChanged,
        ),
        (
            "naming nothing it left",
            &swept,
            "store.unknown-type — no doctype `note`\n",
            Classification::RefusedAndChanged,
        ),
        (
            "the doc it named, rewritten",
            &rewritten,
            "blocked · docs/legacy-notes/cache-benchmarks.md: not a regular file\n",
            Classification::RefusedAndChanged,
        ),
    ] {
        let (class, why) = Cell {
            argv,
            code: Some(1),
            said,
            listed: Listed::Refuses,
            ..Cell::of(movers::RELOCATE, &two, after)
        }
        .came_to();
        assert_eq!(class, want, "{case}: {why:#?}");
    }

    // A `docs-root` re-point: the decision moved under the new root holds; parked in the
    // workbench in place of its move, the knob set, it is red.
    let before = with_committed(&[("docs/decisions/one-way.md", DECISION)]);
    let argv: &[&str] = &["config", "set", "docs-root", "notes"];
    let knob_set = |m: &mut Manifest| {
        written(
            m,
            ".jigc/config/manifest.yaml",
            "scalar:\n  docs-root: notes\n",
        );
    };
    let moved = then(&before, |m| {
        git_mv(
            m,
            "docs/decisions/one-way.md",
            "notes/decisions/one-way.md",
            DECISION,
        );
        knob_set(m);
    });
    let parked = then(&before, |m| {
        m.files.remove(&format!("{REPO}/docs/decisions/one-way.md"));
        written(m, ".jigc/displaced/one-way.md", DECISION);
        let main = m.checkouts.get_mut(REPO).expect("the main checkout");
        main.index.remove("docs/decisions/one-way.md");
        knob_set(m);
    });
    let came = |after: &Manifest| {
        Cell {
            argv,
            ..Cell::of(movers::CONFIG_SET, &before, after)
        }
        .came_to()
    };
    let (class, why) = came(&moved);
    assert_eq!(class, Classification::Holds, "the docs-root move: {why:#?}");
    let (class, why) = came(&parked);
    assert!(
        class.is_red(),
        "the doc to move parked instead: came out {class}, {why:#?}"
    );

    // A root knob's re-point: the roadmap under the new root, and the knob set.
    const ROADMAP: &str = "---\nschema-version: 1\n---\n\n# Roadmap\n";
    let before = then(&with_committed(&[("docs/roadmap.md", ROADMAP)]), |m| {
        written(
            m,
            ".jigc/config/packs.yaml",
            "compose-embedded-methodology: true\n",
        );
    });
    let argv: &[&str] = &["config", "set", "placement-root", "notes"];
    let re_pointed = |to: &str| {
        then(&before, |m| {
            git_mv(m, "docs/roadmap.md", to, ROADMAP);
            written(
                m,
                ".jigc/config/manifest.yaml",
                "scalar:\n  placement-root: notes\n",
            );
        })
    };
    let came = |after: &Manifest| {
        Cell {
            argv,
            ..Cell::of(movers::CONFIG_SET, &before, after)
        }
        .came_to()
    };
    let (class, why) = came(&re_pointed("notes/roadmap.md"));
    assert_eq!(class, Classification::Holds, "the re-point: {why:#?}");
    let (class, why) = came(&re_pointed("elsewhere/roadmap.md"));
    assert!(class.is_red(), "another root: came out {class}, {why:#?}");
}

/// **R1's door holds what the sweep must have done** — the human's rulings of 2026-10-20: *an
/// acting sweep does all it can, then exits non-zero*, and `jigc relocate` *blocks that doc,
/// moves the others, names the path to clear and exits non-zero* (the rows' record's §6,
/// entry 23). Judged under the listing of the two `relocate` cells that carry a second note
/// — one with a link in the first note's place, one with a file of the user's at the first
/// note's new home — a refusal that moved the second note into its home and left the first
/// as it stood, naming it, changes nothing a refusal may not; **a refusal of the whole sweep,
/// which moved nothing, is not the outcome they list** (red first, the review of the task
/// `B9`, H1: the whole refusal came out *refuses, nothing changed*); and the sweep that parks
/// the occupant and moves both notes at exit 0 is red.
#[test]
fn a_sweep_refused_whole_is_not_the_outcome_r1s_cells_list() {
    const NOTE: &str = "# Cache benchmarks\n\n## Body\n\nA single node caps throughput.\n";
    const SECOND: &str = "# Load tests\n\n## Body\n\nTwo nodes hold the load.\n";
    const SCHEMA: &str = "type: note\nlocation: notes/\nid-from: title\nsections:\n  - id: body\n    slot: { hint: \"The note.\" }\n";
    const OCCUPANT: &str = "the user's own notes\n";
    let cells = cells();
    let named = |name: &str| {
        *cells
            .iter()
            .find(|cell| cell.name == name)
            .unwrap_or_else(|| panic!("`{name}` is a cell of the table"))
    };
    let two = then(
        &with_committed(&[
            ("docs/legacy-notes/cache-benchmarks.md", NOTE),
            ("docs/legacy-notes/load-tests.md", SECOND),
        ]),
        |m| {
            written(
                m,
                ".jigc/config/packs.yaml",
                "packs:\n  - .jigc/note-pack\n",
            );
            written(m, ".jigc/note-pack/schemas/note.yaml", SCHEMA);
        },
    );
    let second_moved = |m: &mut Manifest| {
        git_mv(
            m,
            "docs/legacy-notes/load-tests.md",
            "docs/notes/load-tests.md",
            SECOND,
        );
    };

    // A link to nothing in the first note's place.
    let cell = named("relocate-a-link-to-nothing-in-one-stranded-notes-place-and-a-second-note");
    let linked = then(&two, |m| {
        m.files.insert(
            format!("{REPO}/docs/legacy-notes/cache-benchmarks.md"),
            Entry::Link("nowhere".to_string()),
        );
    });
    let said = "blocking · store.home-not-regular-file — docs/legacy-notes/cache-benchmarks.md \
                is a link\n";
    let refused = |after: &Manifest| {
        Cell {
            argv: cell.argv,
            code: Some(1),
            said,
            listed: cell.listed,
            ..Cell::of(movers::RELOCATE, &linked, after)
        }
        .came_to()
    };
    let (class, why) = refused(&then(&linked, second_moved));
    assert_eq!(
        class,
        Classification::RefusesNothingChanged,
        "the link's cell, the second note moved: {why:#?}"
    );
    let (class, why) = refused(&linked);
    assert_eq!(
        class,
        Classification::NotTheListedOutcome,
        "the link's cell, the whole sweep refused: {why:#?}"
    );
    // A copy is no move: the second note at its home, staged, and still at its place.
    let copied = then(&linked, |m| {
        written(m, "docs/notes/load-tests.md", SECOND);
        let main = m.checkouts.get_mut(REPO).expect("the main checkout");
        main.index
            .insert("docs/notes/load-tests.md".to_string(), staged(SECOND));
    });
    let (class, why) = refused(&copied);
    assert!(
        class.is_red(),
        "the link's cell, the second note copied: came out {class}, {why:#?}"
    );

    // A file of the user's at the first note's new home.
    let cell = named("relocate-a-file-of-the-users-at-the-notes-new-home-and-a-second-note");
    let occupied = then(&two, |m| {
        written(m, "docs/notes/cache-benchmarks.md", OCCUPANT);
    });
    let planted = [(
        "repo/docs/notes/cache-benchmarks.md",
        "the user's own notes",
    )];
    let said = "blocked · docs/notes/cache-benchmarks.md stands at the destination — clear it \
                or adopt it, then re-run\n";
    let came = |code: i32, after: &Manifest| {
        Cell {
            argv: cell.argv,
            planted: &planted,
            code: Some(code),
            said,
            listed: cell.listed,
            ..Cell::of(movers::RELOCATE, &occupied, after)
        }
        .came_to()
    };
    let (class, why) = came(1, &then(&occupied, second_moved));
    assert_eq!(
        class,
        Classification::RefusesNothingChanged,
        "the occupant's cell, the second note moved: {why:#?}"
    );
    let (class, why) = came(1, &occupied);
    assert_eq!(
        class,
        Classification::NotTheListedOutcome,
        "the occupant's cell, the whole sweep refused: {why:#?}"
    );
    let parked = then(&occupied, |m| {
        git_mv(
            m,
            "docs/legacy-notes/cache-benchmarks.md",
            "docs/notes/cache-benchmarks.md",
            NOTE,
        );
        written(m, ".jigc/displaced/cache-benchmarks.md", OCCUPANT);
        second_moved(m);
    });
    let (class, why) = came(0, &parked);
    assert!(
        class.is_red(),
        "the occupant's cell, the occupant parked and both notes moved: came out {class}, \
         {why:#?}"
    );
}

/// **A task a command mints is one, where nothing stood** — the word `<minted-task>` of the
/// mint family's rows (`jigc start`, `jigc migrate`, `jigc task amend`; the task `B9`): an
/// area under `.jigc/tasks/` that carries its base pin after the command, at an id where
/// nothing stood before it. A new area holds; an area that carries no pin, two areas, an
/// area made over a residual that stood at its id, and a sibling task's area rewritten
/// beside the mint are each red (red first: against the word declared and read by nobody,
/// the new area came out *written outside its allowance*).
#[test]
fn a_task_a_command_mints_is_one_nothing_stood_at() {
    const MINTS: Row = row(&[line(
        "N1",
        Allows::Files(Where::Under(".jigc/tasks/<minted-task>"), May::Rebuilt),
    )]);
    let area = |m: &mut Manifest, id: &str| {
        written(m, &format!(".jigc/tasks/{id}/base.json"), "a pin\n");
        written(
            m,
            &format!(".jigc/tasks/{id}/docs/commit:{id}.md"),
            "# a commit\n",
        );
    };
    let before = then(&sandbox(), |m| dir(m, "repo/.jigc"));
    let (class, why) =
        Cell::of(MINTS, &before, &then(&before, |m| area(m, "add-a-file"))).came_to();
    assert_eq!(class, Classification::Holds, "a task minted: {why:#?}");
    let residual = then(&before, |m| {
        written(m, ".jigc/tasks/add-a-file/notes.md", "the user's notes\n")
    });
    let sibling = then(&before, |m| area(m, "other"));
    for (case, before, after) in [
        (
            "an area that carries no pin",
            &before,
            then(&before, |m| {
                written(m, ".jigc/tasks/add-a-file/intent", "add a file\n")
            }),
        ),
        (
            "two areas",
            &before,
            then(&before, |m| {
                area(m, "one");
                area(m, "two");
            }),
        ),
        (
            "an area made over a residual",
            &residual,
            then(&residual, |m| area(m, "add-a-file")),
        ),
        (
            "a sibling's area rewritten beside the mint",
            &sibling,
            then(&sibling, |m| {
                area(m, "add-a-file");
                written(m, ".jigc/tasks/other/docs/commit:other.md", "# rewritten\n");
            }),
        ),
    ] {
        let (class, why) = Cell::of(MINTS, before, &after).came_to();
        assert!(class.is_red(), "{case}: came out {class}, {why:#?}");
    }
}

/// **A provisioned place makes, and rewrites nothing** — the word *provisioned* of `jigc
/// workflow`'s `WF1` (the task `B9`): what a re-entry provisions into its sub-task's area is
/// made where nothing stood. The commit doc and the provenance manifest made in the unit's
/// area hold; a staged doc that stood rewritten, the base pin gone, and a file made in a
/// sibling's area are each red (red first: against the word declared and read by nobody,
/// the made files came out *written outside its allowance*).
#[test]
fn a_provisioned_place_makes_and_rewrites_nothing() {
    const PROVISIONS: Row = Row {
        unit: Unit::TheTaskFlag,
        ..row(&[line(
            "P1",
            Allows::Files(Where::Under(".jigc/tasks/<unit>"), May::Provisioned),
        )])
    };
    let argv: &[&str] = &["workflow", "sub-task", "--task", "s1"];
    let before = then(&sandbox(), |m| {
        for id in ["s1", "s2"] {
            written(m, &format!(".jigc/tasks/{id}/base.json"), "a pin\n");
        }
    });
    let provisioned = |m: &mut Manifest| {
        written(m, ".jigc/tasks/s1/docs/commit:s1.md", "# a commit\n");
        written(m, ".jigc/tasks/s1/docs/provenance.json", "{}\n");
    };
    let came = |before: &Manifest, after: &Manifest| {
        Cell {
            argv,
            ..Cell::of(PROVISIONS, before, after)
        }
        .came_to()
    };
    let (class, why) = came(&before, &then(&before, provisioned));
    assert_eq!(class, Classification::Holds, "provisioned: {why:#?}");
    let authored = then(&before, |m| {
        written(m, ".jigc/tasks/s1/docs/commit:s1.md", "# authored\n")
    });
    for (case, before, after) in [
        (
            "a staged doc rewritten",
            &authored,
            then(&authored, provisioned),
        ),
        (
            "the base pin gone",
            &before,
            then(&before, |m| {
                provisioned(m);
                m.files.remove("repo/.jigc/tasks/s1/base.json");
            }),
        ),
        (
            "a file made in a sibling's area",
            &before,
            then(&before, |m| {
                written(m, ".jigc/tasks/s2/docs/commit:s2.md", "# a commit\n")
            }),
        ),
    ] {
        let (class, why) = came(before, &after);
        assert!(class.is_red(), "{case}: came out {class}, {why:#?}");
    }
}

/// **A line moves aside only from a place the command removes, never from a destination** —
/// the human's ruling of 2026-10-20 on the rows' record's §6, entry 23: *jigc moves a file
/// aside only from a place it is about to remove, never from a destination*. A line of the
/// word *moved aside* at one file's place — a mover's destination, as `RL3` and `CS3` were
/// until that ruling — is named: on a fixture, beside one under a directory, which is not;
/// and over the table's rows, which hold none (red first: the rows as they stood named
/// `RL3` and `CS3`).
#[test]
fn a_line_moves_aside_only_from_a_place_it_removes() {
    fn from_one_file(rows: &[&Row]) -> Vec<String> {
        rows.iter()
            .flat_map(|row| {
                row.lines
                    .iter()
                    .filter(|line| {
                        matches!(line.allows, Allows::Files(place, May::MovedAsideTo(_))
                            if !matches!(place, Where::Under(_)))
                    })
                    .map(move |line| {
                        format!("the line `{}` of `jigc {}`", line.id, row.leaf.join(" "))
                    })
            })
            .collect()
    }
    const FIXTURE: Row = row(&[
        line(
            "P1",
            Allows::Files(
                Where::Under(".jigc/tasks/<unit>"),
                May::MovedAsideTo(".jigc/displaced/<unit>"),
            ),
        ),
        line(
            "P2",
            Allows::Files(
                Where::Path(byte_cell::A_DOC),
                May::MovedAsideTo(".jigc/displaced"),
            ),
        ),
    ]);
    assert_eq!(
        from_one_file(&[&FIXTURE]),
        vec!["the line `P2` of `jigc probe`".to_string()]
    );
    let named = from_one_file(&rows());
    assert!(
        named.is_empty(),
        "a line moves aside from one file's place — a destination, whose occupant blocks the \
         move: {named:#?}"
    );
}

/// **A migrated doc keeps its prose, and its stamp is its doctype's current version** —
/// `MG1` and `MG1, from a prior home` of `jigc migrate-corpus`'s row
/// ([`movers::MIGRATE_CORPUS`]), the word *migrated* (the task `B8`). On the row itself: a
/// v1 decision given its `## Options` heading and stamped 2, committed, holds; so does the
/// v1 changelog moved from its earlier home to `CHANGELOG.md` with its title and stamp
/// moved. A prose line changed or added, the stamp left behind, a doc already current
/// rewritten, the changelog's prose dropped on its way, and a line of the user's in no
/// commit swept into the migration's commit, are each red — and under `--dry-run` any
/// write is. **Every byte the migration's sentences do not let it change** (the review of
/// `B8`, its H3, red first — each came out *holds*): a `- key:` bullet or a sub-heading the
/// author wrote in a slot dropped or changed, the `supersedes` edge dropped, the status
/// flipped, a field added, the title rewritten where no `display-title:` moved, a heading
/// no schema adds or the added one twice, the changelog's title not the one its schema
/// declares, a heading of its author's dropped — each is red; a doc with no stamp given one
/// holds, and given two is red. A line of the user's in the old changelog moves with it, and holds under
/// `--no-commit`; a commit that carries it is red, read against what `HEAD` held at the
/// old home (the task `B8`'s third commit, red first: the plant came out *written
/// outside*, then the commit *held*). A refusal that committed one doc holds where the
/// doc it refused stands as it did, and is red where that doc is migrated in the working
/// tree or staged — *"the doc joins `blocked`"* (the review of `B8`, its H8, red first:
/// each came out *refuses, nothing changed*).
#[test]
fn a_migrated_doc_keeps_its_prose() {
    const V1: &str = "---\nschema-version: 1\nstatus: accepted\n---\n\n# One way\n\n## Context\n\nWe had to choose.\n\n## Decision\n\nOne way.\n\n## Consequences\n\nIt holds.\n";
    const V2: &str = "---\nschema-version: 2\nstatus: accepted\n---\n\n# One way\n\n## Context\n\nWe had to choose.\n\n## Options\n\n\n\n## Decision\n\nOne way.\n\n## Consequences\n\nIt holds.\n";
    let before = with_committed(&[("docs/decisions/one-way.md", V1)]);
    let migrated = |path: &str, bytes: &str| {
        then(&before, |m| {
            written(m, path, bytes);
            commit(m, "c2", &[(path, Some((0o100_644, bytes)))]);
        })
    };
    let came = |before: &Manifest, after: &Manifest, spelling: Option<&'static str>| {
        Cell {
            argv: &["migrate-corpus"],
            spelling,
            ..Cell::of(movers::MIGRATE_CORPUS, before, after)
        }
        .came_to()
    };
    let (class, why) = came(&before, &migrated("docs/decisions/one-way.md", V2), None);
    assert_eq!(class, Classification::Holds, "the migration: {why:#?}");
    for (case, bytes) in [
        (
            "a prose line changed",
            V2.replace("It holds.", "It may hold."),
        ),
        (
            "the stamp left behind",
            V2.replace("schema-version: 2", "schema-version: 1"),
        ),
        (
            "a line of prose added",
            V2.replace("It holds.\n", "It holds.\nAnd more.\n"),
        ),
    ] {
        let (class, why) = came(
            &before,
            &migrated("docs/decisions/one-way.md", &bytes),
            None,
        );
        assert!(class.is_red(), "{case}: came out {class}, {why:#?}");
    }
    // Every byte the migration's sentences do not let it change: a `- key:` bullet and a
    // sub-heading the author wrote inside a slot, a ref in the front matter and a field's
    // value (the review of `B8`, its H3 — each came out *holds*).
    const AUTHORED: &str = "---\nschema-version: 1\nstatus: accepted\nsupersedes: [adr:old]\n---\n\n# One way\n\n## Context\n\nWe had to choose.\n\n### A caveat of the author's\n\n- risk: we may lose data\n\n## Decision\n\nOne way.\n\n## Consequences\n\nIt holds.\n";
    let authored = with_committed(&[("docs/decisions/one-way.md", AUTHORED)]);
    let into = |bytes: &str| {
        then(&authored, |m| {
            written(m, "docs/decisions/one-way.md", bytes);
            commit(
                m,
                "c2",
                &[("docs/decisions/one-way.md", Some((0o100_644, bytes)))],
            );
        })
    };
    let honest = AUTHORED
        .replace("schema-version: 1", "schema-version: 2")
        .replace("## Decision", "## Options\n\n\n\n## Decision");
    let (class, why) = came(&authored, &into(&honest), None);
    assert_eq!(
        class,
        Classification::Holds,
        "the authored migration: {why:#?}"
    );
    // A doc with no stamp, *"the v0 corpus state"*: the stamp is added, once.
    let unstamped = AUTHORED.replace("schema-version: 1\n", "");
    let v0 = with_committed(&[("docs/decisions/one-way.md", unstamped.as_str())]);
    let stamped = |bytes: &str| {
        then(&v0, |m| {
            written(m, "docs/decisions/one-way.md", bytes);
            commit(
                m,
                "c2",
                &[("docs/decisions/one-way.md", Some((0o100_644, bytes)))],
            );
        })
    };
    let (class, why) = came(&v0, &stamped(&honest), None);
    assert_eq!(class, Classification::Holds, "a v0 doc stamped: {why:#?}");
    let twice = honest.replace("---\n\n", "schema-version: 2\n---\n\n");
    let (class, why) = came(&v0, &stamped(&twice), None);
    assert!(
        class.is_red(),
        "the stamp added twice: came out {class}, {why:#?}"
    );
    for (case, bytes) in [
        (
            "a bullet of the author's dropped",
            honest.replace("- risk: we may lose data\n", ""),
        ),
        (
            "a bullet's value changed",
            honest.replace("- risk: we may lose data", "- risk: none"),
        ),
        (
            "a sub-heading of the author's dropped",
            honest.replace("### A caveat of the author's\n", ""),
        ),
        (
            "the `supersedes` edge dropped",
            honest.replace("supersedes: [adr:old]\n", ""),
        ),
        (
            "the status flipped",
            honest.replace("status: accepted", "status: superseded"),
        ),
        (
            "a front-matter field added",
            honest.replace("status: accepted\n", "status: accepted\ndate: 2026-10-19\n"),
        ),
        (
            "the title rewritten",
            honest.replace("# One way\n", "# The one way\n"),
        ),
        (
            "a heading no schema adds",
            honest.replace("## Consequences", "## Notes\n\n## Consequences"),
        ),
        (
            "the added heading twice",
            honest.replace("## Decision", "## Options\n\n## Decision"),
        ),
    ] {
        let (class, why) = came(&authored, &into(&bytes), None);
        assert!(class.is_red(), "{case}: came out {class}, {why:#?}");
    }
    // A line of the user's in the stale doc, in no commit: the migration keeps it on disk,
    // and a commit that carries it carries what `HEAD` never held.
    let edited = then(&before, |m| {
        written(
            m,
            "docs/decisions/one-way.md",
            &format!("{V1}A line of the user's.\n"),
        );
    });
    let swept = then(&edited, |m| {
        let bytes = format!("{V2}A line of the user's.\n");
        written(m, "docs/decisions/one-way.md", &bytes);
        commit(
            m,
            "c2",
            &[(
                "docs/decisions/one-way.md",
                Some((0o100_644, bytes.as_str())),
            )],
        );
    });
    let (class, why) = came(&edited, &swept, None);
    assert_eq!(
        class,
        Classification::GitMoved,
        "the user's uncommitted line swept into the commit: {why:#?}"
    );
    let current = with_committed(&[("docs/decisions/one-way.md", V2)]);
    let rewritten = then(&current, |m| {
        let bytes = V2.replace("status: accepted", "status: superseded");
        written(m, "docs/decisions/one-way.md", &bytes);
        commit(
            m,
            "c2",
            &[(
                "docs/decisions/one-way.md",
                Some((0o100_644, bytes.as_str())),
            )],
        );
    });
    let (class, why) = came(&current, &rewritten, None);
    assert!(
        class.is_red(),
        "a doc already current: came out {class}, {why:#?}"
    );
    // A refusal: one doc refused while another was migrated and committed holds — the
    // run's other docs still migrate and commit; a refusal that committed nothing and
    // leaves the migration written and staged is red (the rows' record's §6, entry 22, ruled
    // by the human on 2026-10-19: such a commit rolls back).
    let refused = |after: &Manifest| {
        Cell {
            argv: &["migrate-corpus"],
            code: Some(1),
            listed: Listed::Refuses,
            ..Cell::of(movers::MIGRATE_CORPUS, &before, after)
        }
        .came_to()
    };
    let (class, why) = refused(&migrated("docs/decisions/one-way.md", V2));
    assert_eq!(
        class,
        Classification::RefusesNothingChanged,
        "a refusal that committed the others: {why:#?}"
    );
    let unlanded = then(&before, |m| {
        written(m, "docs/decisions/one-way.md", V2);
        let main = m.checkouts.get_mut(REPO).expect("the main checkout");
        main.index
            .insert("docs/decisions/one-way.md".to_string(), staged(V2));
    });
    let (class, why) = refused(&unlanded);
    assert_eq!(
        class,
        Classification::RefusedAndChanged,
        "a refusal that committed nothing: {why:#?}"
    );
    // A refusal that committed one doc changes what git tracks only as its commit landed
    // it: the doc it refused — *"the doc joins `blocked`"* — is not written, in the
    // working tree or in the index (the review of `B8`, its H8, red first: each came out
    // *refuses, nothing changed*).
    let two = with_committed(&[
        ("docs/decisions/one-way.md", V1),
        ("docs/decisions/two-way.md", V1),
    ]);
    let refused_of_two = |after: &Manifest| {
        Cell {
            argv: &["migrate-corpus"],
            code: Some(1),
            listed: Listed::Refuses,
            ..Cell::of(movers::MIGRATE_CORPUS, &two, after)
        }
        .came_to()
    };
    let one_committed = |m: &mut Manifest| {
        written(m, "docs/decisions/one-way.md", V2);
        commit(
            m,
            "c2",
            &[("docs/decisions/one-way.md", Some((0o100_644, V2)))],
        );
    };
    let (class, why) = refused_of_two(&then(&two, one_committed));
    assert_eq!(
        class,
        Classification::RefusesNothingChanged,
        "one doc committed, the refused one as it stood: {why:#?}"
    );
    let blocked_written = then(&two, |m| {
        one_committed(m);
        written(m, "docs/decisions/two-way.md", V2);
    });
    let blocked_staged = then(&two, |m| {
        one_committed(m);
        written(m, "docs/decisions/two-way.md", V2);
        let main = m.checkouts.get_mut(REPO).expect("the main checkout");
        main.index
            .insert("docs/decisions/two-way.md".to_string(), staged(V2));
    });
    for (case, after) in [
        (
            "the refused doc migrated in the working tree",
            &blocked_written,
        ),
        ("the refused doc migrated and staged", &blocked_staged),
    ] {
        let (class, why) = refused_of_two(after);
        assert_eq!(class, Classification::RefusedAndChanged, "{case}: {why:#?}");
    }

    // Under `--dry-run` nothing is written.
    let (class, why) = came(&before, &before, Some("--dry-run"));
    assert_eq!(class, Classification::Holds, "a dry run: {why:#?}");
    let (class, why) = came(
        &before,
        &migrated("docs/decisions/one-way.md", V2),
        Some("--dry-run"),
    );
    assert!(
        class.is_red(),
        "a dry run that writes: came out {class}, {why:#?}"
    );
    // Under `--no-commit` the migration is written and staged, and nothing is committed — the
    // human's ruling of 2026-10-19 on the rows' record's §6, entry 21: *"stages exactly the
    // paths the migration touched — both halves of a move — and commits nothing"* (`MG4`,
    // red first: the staged migration came out *git's state moved*, no line staging it).
    // Written and left unstaged, as the product does, it holds too: a line lets a write
    // happen, and does not demand it.
    let unstaged = then(&before, |m| {
        written(m, "docs/decisions/one-way.md", V2);
    });
    let staged_too = then(&unstaged, |m| {
        let main = m.checkouts.get_mut(REPO).expect("the main checkout");
        main.index
            .insert("docs/decisions/one-way.md".to_string(), staged(V2));
    });
    for (case, after, want) in [
        ("staged", &staged_too, Classification::Holds),
        ("unstaged", &unstaged, Classification::Holds),
    ] {
        let (class, why) = came(&before, after, Some("--no-commit"));
        assert_eq!(class, want, "`--no-commit`, the migration {case}: {why:#?}");
    }
    let (class, why) = came(
        &before,
        &migrated("docs/decisions/one-way.md", V2),
        Some("--no-commit"),
    );
    assert!(
        class.is_red(),
        "`--no-commit` that commits: came out {class}, {why:#?}"
    );

    // The v1 changelog, at its earlier home.
    const OLD: &str = "---\nschema-version: 1\n---\n\n# changelog\n\n## Unreleased\n\nA change.\n";
    const NEW: &str = "---\nschema-version: 2\n---\n\n# Changelog\n\n## Unreleased\n\nA change.\n";
    let before = with_committed(&[("docs/changelog/changelog.md", OLD)]);
    let relocated = |bytes: &str| {
        then(&before, |m| {
            m.files
                .remove(&format!("{REPO}/docs/changelog/changelog.md"));
            written(m, "CHANGELOG.md", bytes);
            commit(
                m,
                "c2",
                &[
                    ("docs/changelog/changelog.md", None),
                    ("CHANGELOG.md", Some((0o100_644, bytes))),
                ],
            );
        })
    };
    let (class, why) = came(&before, &relocated(NEW), None);
    assert_eq!(class, Classification::Holds, "the relocation: {why:#?}");
    for (case, bytes) in [
        ("the prose dropped", NEW.replace("A change.\n", "")),
        (
            "the title not the one the schema declares",
            NEW.replace("# Changelog", "# The changelog"),
        ),
        (
            "a heading of the author's dropped",
            NEW.replace("## Unreleased\n\n", ""),
        ),
    ] {
        let (class, why) = came(&before, &relocated(&bytes), None);
        assert!(class.is_red(), "{case}: came out {class}, {why:#?}");
    }
    // A line of the user's in the old changelog, in no commit: the migration carries it to
    // the doc's new home — under `--no-commit`, it holds there; and a commit that carries it
    // carries what `HEAD` never held at the old home.
    const MINE: &str = "A line of the user's.\n";
    let edited = then(&before, |m| {
        written(m, "docs/changelog/changelog.md", &format!("{OLD}{MINE}"))
    });
    let moved = |committed: bool| {
        then(&edited, |m| {
            let bytes = format!("{NEW}{MINE}");
            m.files
                .remove(&format!("{REPO}/docs/changelog/changelog.md"));
            written(m, "CHANGELOG.md", &bytes);
            if committed {
                commit(
                    m,
                    "c2",
                    &[
                        ("docs/changelog/changelog.md", None),
                        ("CHANGELOG.md", Some((0o100_644, bytes.as_str()))),
                    ],
                );
            }
        })
    };
    let carried = |after: &Manifest, spelling: Option<&'static str>| {
        Cell {
            argv: &["migrate-corpus"],
            spelling,
            planted: &[("repo/docs/changelog/changelog.md", "A line of the user's.")],
            ..Cell::of(movers::MIGRATE_CORPUS, &edited, after)
        }
        .came_to()
    };
    let (class, why) = carried(&moved(false), Some("--no-commit"));
    assert_eq!(
        class,
        Classification::Holds,
        "the user's line carried with its doc: {why:#?}"
    );
    let (class, why) = carried(&moved(true), None);
    assert_eq!(
        class,
        Classification::GitMoved,
        "the user's line carried into the commit: {why:#?}"
    );
}

/// **A knob set keeps every other line of the project manifest** — `CS1` of `jigc config
/// set`'s row ([`movers::CONFIG_SET`]), the word *its knob set* (the task `B8`). On the row
/// itself: the knob's line added beside the user's own lines, the manifest made where
/// none stood, or the knob's value replaced, holds; the user's comment dropped, another
/// knob's value changed, or the manifest left with no line of the knob, is each red — and
/// so, since the review of `B8` (its H9, red first: each came out *holds*), is the knob set
/// to a value the command does not name or to none, its line outside the `scalar:` map or
/// twice, and the `scalar:` line dropped or written twice.
#[test]
fn a_knob_set_keeps_every_other_line() {
    const MINE: &str = "# a note of the user's\nscalar:\n  docs-root: docs\n";
    let before = then(&sandbox(), |m| {
        written(m, ".jigc/config/manifest.yaml", MINE)
    });
    let argv: &[&str] = &["config", "set", "finalize.fan-out.squash", "false"];
    let set = |bytes: &str| then(&before, |m| written(m, ".jigc/config/manifest.yaml", bytes));
    let came = |before: &Manifest, after: &Manifest| {
        Cell {
            argv,
            ..Cell::of(movers::CONFIG_SET, before, after)
        }
        .came_to()
    };
    let (class, why) = came(
        &before,
        &set(
            "# a note of the user's\nscalar:\n  docs-root: docs\n  finalize.fan-out.squash: 'false'\n",
        ),
    );
    assert_eq!(
        class,
        Classification::Holds,
        "the knob's line added: {why:#?}"
    );
    let none = sandbox();
    let made = then(&none, |m| {
        written(
            m,
            ".jigc/config/manifest.yaml",
            "scalar:\n  finalize.fan-out.squash: 'false'\n",
        )
    });
    let (class, why) = came(&none, &made);
    assert_eq!(class, Classification::Holds, "the manifest made: {why:#?}");
    let was_set = then(&sandbox(), |m| {
        written(
            m,
            ".jigc/config/manifest.yaml",
            "scalar:\n  finalize.fan-out.squash: 'true'\n",
        )
    });
    let reset = then(&was_set, |m| {
        written(
            m,
            ".jigc/config/manifest.yaml",
            "scalar:\n  finalize.fan-out.squash: 'false'\n",
        )
    });
    let (class, why) = came(&was_set, &reset);
    assert_eq!(
        class,
        Classification::Holds,
        "the knob's value replaced: {why:#?}"
    );
    for (case, bytes) in [
        // The value the command names, and the map that holds it (the review of `B8`, its
        // H9, red first: each came out *holds*).
        (
            "the knob set to another value",
            "# a note of the user's\nscalar:\n  docs-root: docs\n  finalize.fan-out.squash: 'true'\n",
        ),
        (
            "the knob set to no value",
            "# a note of the user's\nscalar:\n  docs-root: docs\n  finalize.fan-out.squash:\n",
        ),
        (
            "the knob's line outside the `scalar:` map",
            "# a note of the user's\nscalar:\n  docs-root: docs\nfinalize.fan-out.squash: 'false'\n",
        ),
        (
            "the `scalar:` line dropped",
            "# a note of the user's\n  docs-root: docs\n  finalize.fan-out.squash: 'false'\n",
        ),
        (
            "a second `scalar:` line",
            "# a note of the user's\nscalar:\n  docs-root: docs\nscalar:\n  finalize.fan-out.squash: 'false'\n",
        ),
        (
            "the knob's line twice",
            "# a note of the user's\nscalar:\n  docs-root: docs\n  finalize.fan-out.squash: 'false'\n  finalize.fan-out.squash: 'false'\n",
        ),
        (
            "the user's comment dropped",
            "scalar:\n  docs-root: docs\n  finalize.fan-out.squash: 'false'\n",
        ),
        (
            "another knob's value changed",
            "# a note of the user's\nscalar:\n  docs-root: notes\n  finalize.fan-out.squash: 'false'\n",
        ),
        (
            "no line of the knob",
            "# a note of the user's\nscalar:\n  docs-root: docs\n\n",
        ),
    ] {
        let (class, why) = came(&before, &set(bytes));
        assert!(class.is_red(), "{case}: came out {class}, {why:#?}");
    }
}

/// **A plant with no bytes is a fault of the cell**, and so are two plants with one
/// marker, and a plant the manifest taken before the command does not hold where the cell
/// says it stands: none is a classification.
#[test]
fn a_plant_with_no_bytes_is_a_fault_of_the_cell() {
    let before = then(&planted_under_the_workbench(), |m| {
        put(m, "repo/.jigc/logs/empty.md", "");
        put(m, "repo/.jigc/logs/twin.md", MARKER);
    });
    for (planted, says) in [
        (&[("repo/.jigc/logs/empty.md", "")][..], "has no bytes"),
        (
            &[
                ("repo/.jigc/logs/notes.md", MARKER),
                ("repo/.jigc/logs/twin.md", MARKER),
            ][..],
            "two plants carry the marker",
        ),
        (
            &[("repo/.jigc/elsewhere.md", MARKER)][..],
            "does not stand there",
        ),
    ] {
        let fault = Cell {
            planted,
            ..Cell::of(REMOVES, &before, &before)
        }
        .verdict();
        assert!(
            fault.as_ref().is_err_and(|fault| fault.contains(says)),
            "{planted:?}: {fault:?}"
        );
    }
}

/// **The product's own file rewritten is no loss.** A file a lead-in of the cell wrote is
/// rewritten: under a line that covers it the cell holds; with no line it is *written
/// outside its allowance* and never *lost*, though the old bytes stand nowhere. The same
/// rewrite of a file the prepared state held — nobody's lead-in wrote it — is *lost*;
/// where the store still has those bytes and nothing reaches them it is *lost* too — the
/// bare store is not held (the human's ruling of 2026-10-14) — and where an object within
/// reach has them it is *written outside its allowance*.
#[test]
fn the_products_own_file_rewritten_is_no_loss() {
    const CACHES: Row = row(&[line(
        "C1",
        Allows::Files(Where::Under(".jigc/index"), May::WriteWhole),
    )]);
    let base = then(&sandbox(), |m| dir(m, "repo/.jigc"));
    let before = then(&base, |m| {
        dir(m, "repo/.jigc/index");
        put(m, "repo/.jigc/index/edges.json", "{\"stamp\":\"c0\"}\n");
    });
    let after = then(&before, |m| {
        put(m, "repo/.jigc/index/edges.json", "{\"stamp\":\"\"}\n")
    });
    let own = |row| Cell {
        base: Some(&base),
        ..Cell::of(row, &before, &after)
    };
    comes_out(&own(CACHES), Classification::Holds);
    comes_out(&own(NOTHING), Classification::WrittenOutside);
    // The prepared state's own bytes: the same rewrite, and no lead-in wrote the file.
    let prepared = Cell::of(NOTHING, &before, &after);
    comes_out(&prepared, Classification::Lost);
    let stored = then(&after, |m| {
        m.objects.insert(blob_of("{\"stamp\":\"c0\"}\n"));
    });
    comes_out(&Cell::of(NOTHING, &before, &stored), Classification::Lost);
    let held = then(&stored, |m| {
        m.held.insert(blob_of("{\"stamp\":\"c0\"}\n"));
    });
    comes_out(
        &Cell::of(NOTHING, &before, &held),
        Classification::WrittenOutside,
    );
}

/// **A line's condition is read off the state before the command.** A file a line allows
/// only where the repository has no commit is *written outside its allowance* in a
/// repository that has one, and holds in one whose `HEAD` is unborn.
#[test]
fn a_lines_condition_is_read_off_the_state_before_the_command() {
    const SEEDS: Row = row(&[Line {
        when: When::NoCommitYet,
        ..line(
            "S9",
            Allows::Files(Where::Path(".gitignore"), May::MergeInto),
        )
    }]);
    let born = sandbox();
    let seeded = |before: &Manifest| then(before, |m| put(m, "repo/.gitignore", ".env\n"));
    comes_out(
        &Cell::of(SEEDS, &born, &seeded(&born)),
        Classification::WrittenOutside,
    );
    let unborn = then(&Manifest::default(), |m| {
        dir(m, REPO);
        m.checkouts.insert(
            REPO.to_string(),
            Checkout {
                branch: Some("main".to_string()),
                ..Checkout::default()
            },
        );
    });
    comes_out(
        &Cell::of(SEEDS, &unborn, &seeded(&unborn)),
        Classification::Holds,
    );
}

/// **A merge keeps every line, a take-out adds none, and a file written whole is a
/// regular file.** Merged into: a line added among the lines that stood there holds, and
/// one of those lines changed or dropped does not. Taken out of: a line gone holds, and a
/// line added does not. Written whole: any bytes hold, and a link in the file's place
/// does not.
#[test]
fn a_merge_keeps_every_line_and_a_take_out_adds_none() {
    const MERGES: Row = row(&[line(
        "M1",
        Allows::Files(Where::Path("CLAUDE.md"), May::MergeInto),
    )]);
    const TAKES: Row = row(&[line(
        "U1",
        Allows::Files(Where::Path("CLAUDE.md"), May::TakeOutOf),
    )]);
    const WRITES: Row = row(&[line(
        "W1",
        Allows::Files(Where::Path("CLAUDE.md"), May::WriteWhole),
    )]);
    let before = then(&sandbox(), |m| {
        put(m, "repo/CLAUDE.md", "# Mine\n\nmy own line\n");
        m.held.insert(blob_of("# Mine\n\nmy own line\n"));
    });
    let with = |bytes: &str| then(&before, |m| put(m, "repo/CLAUDE.md", bytes));
    let merged = with("# Mine\n\n@.jigc/AGENT.md\nmy own line\n");
    comes_out(&Cell::of(MERGES, &before, &merged), Classification::Holds);
    for not_a_merge in [
        "# Mine\n\n@.jigc/AGENT.md\n",
        "# Mine\n\nmy own line, edited\n@.jigc/AGENT.md\n",
    ] {
        comes_out(
            &Cell::of(MERGES, &before, &with(not_a_merge)),
            Classification::WrittenOutside,
        );
    }
    comes_out(&Cell::of(TAKES, &merged, &before), Classification::Holds);
    comes_out(
        &Cell::of(TAKES, &before, &merged),
        Classification::WrittenOutside,
    );
    comes_out(
        &Cell::of(WRITES, &before, &with("anything\n")),
        Classification::Holds,
    );
    let linked = then(&before, |m| {
        m.files.insert(
            "repo/CLAUDE.md".to_string(),
            Entry::Link("AGENTS.md".to_string()),
        );
    });
    comes_out(
        &Cell::of(WRITES, &before, &linked),
        Classification::WrittenOutside,
    );
}

/// **A commit is no way around a word that says what a file may become** — a line that lets
/// a file be *merged into*, under a row whose commit holds what its lines wrote: the file
/// merged and committed holds; a line of it dropped and committed is *git's state moved
/// outside its allowance*, as it is *written outside* when nothing commits it. Until
/// 2026-10-19 (the task `B8`) a file the working tree held as a commit made it was excused
/// under the commit's line, and the commit's path under any line placed there, so a merge
/// that dropped a line and committed it held.
#[test]
fn a_commit_is_no_way_around_a_words_content_rule() {
    const MERGES_AND_COMMITS: Row = row(&[
        line(
            "M1",
            Allows::Files(Where::Path("CLAUDE.md"), May::MergeInto),
        ),
        line("M2", Allows::Commit(Holding::OnlyTheLines)),
    ]);
    const MINE: &str = "# Mine\n\nmy own line\n";
    let before = then(&sandbox(), |m| {
        put(m, "repo/CLAUDE.md", MINE);
        commit(m, "c1", &[("CLAUDE.md", Some((0o100_644, MINE)))]);
    });
    let committed = |bytes: &str| {
        then(&before, |m| {
            put(m, "repo/CLAUDE.md", bytes);
            commit(m, "c2", &[("CLAUDE.md", Some((0o100_644, bytes)))]);
        })
    };
    comes_out(
        &Cell::of(
            MERGES_AND_COMMITS,
            &before,
            &committed("# Mine\n\n@.jigc/AGENT.md\nmy own line\n"),
        ),
        Classification::Holds,
    );
    comes_out(
        &Cell::of(
            MERGES_AND_COMMITS,
            &before,
            &committed("# Mine\n\n@.jigc/AGENT.md\n"),
        ),
        Classification::GitMoved,
    );
}

/// **A write outside the repository is seen**: a file made under `outside/`, under `home/`
/// and under `tmp/` is each *written outside its allowance*, under a row that lets its
/// command write anything anywhere in the repository. **No line reaches out of the
/// repository** — no row of the acceptance needed one — so the pair of this arm, the same
/// write covered, cannot be stated and is not.
#[test]
fn a_write_outside_the_repository_is_seen() {
    const ANYTHING: Row = row(&[line(
        "X1",
        Allows::Files(Where::Under("*"), May::WriteWhole),
    )]);
    let before = sandbox();
    for place in ["outside/left.txt", "home/.jigc-state", "tmp/commit-msg.txt"] {
        let after = then(&before, |m| put(m, place, "left behind\n"));
        let (got, why) = Cell::of(ANYTHING, &before, &after).came_to();
        assert_eq!(got, Classification::WrittenOutside, "{place}");
        assert!(why.iter().any(|why| why.contains(place)), "{why:?}");
    }
    let inside = then(&before, |m| put(m, "repo/anything.txt", "written\n"));
    comes_out(&Cell::of(ANYTHING, &before, &inside), Classification::Holds);
}

/// The sandbox with a second checkout — a linked worktree at `path`, registered as
/// `name`, with one staged file.
fn with_a_worktree(path: &str, name: &str) -> Manifest {
    then(&sandbox(), |m| {
        let mut at = String::new();
        for part in path.split('/') {
            at = if at.is_empty() {
                part.to_string()
            } else {
                format!("{at}/{part}")
            };
            if at != REPO {
                dir(m, &at);
            }
        }
        put(m, &format!("{path}/work.txt"), "a sub-task's work\n");
        m.registrations.insert(name.to_string(), path.to_string());
        m.checkouts.insert(
            path.to_string(),
            Checkout {
                head: "c0".to_string(),
                branch: None,
                index: [("work.txt".to_string(), staged("a sub-task's work\n"))].into(),
            },
        );
        m.held.insert(blob_of("a sub-task's work\n"));
    })
}

/// A row that lets its command tear a sub-task's worktree down.
const TEARS_DOWN: Row = row(&[
    line(
        "T1",
        Allows::Files(Where::Under(".jigc/worktrees"), May::Removed),
    ),
    line("T2", Allows::Unregistered(".jigc/worktrees")),
]);

/// **Another checkout is seen.** A file removed in a linked worktree is *written outside
/// its allowance*; its registration dropped is *git's state moved outside its allowance*;
/// and both hold where the worktree stands under the directory a line names — while the
/// same teardown of a worktree somebody made elsewhere does not.
#[test]
fn another_checkout_is_seen() {
    let before = with_a_worktree("repo/.jigc/worktrees/sub-1", "sub-1");
    let file_gone = then(&before, |m| {
        m.files.remove("repo/.jigc/worktrees/sub-1/work.txt");
    });
    comes_out(
        &Cell::of(NOTHING, &before, &file_gone),
        Classification::WrittenOutside,
    );
    let dropped = then(&before, |m| {
        m.registrations.clear();
    });
    comes_out(
        &Cell::of(NOTHING, &before, &dropped),
        Classification::GitMoved,
    );
    let torn_down = |before: &Manifest, path: &str| {
        then(before, |m| {
            m.files.retain(|at, _| !at.starts_with(path));
            m.registrations.clear();
            m.checkouts.retain(|at, _| !at.starts_with(path));
        })
    };
    let gone = torn_down(&before, "repo/.jigc/worktrees");
    let own = Cell::of(TEARS_DOWN, &before, &gone);
    comes_out(&own, Classification::Holds);
    assert_eq!(
        own.verdict().expect("a cell").excused,
        [("T1", 3), ("T2", 2)]
    );
    let theirs = with_a_worktree("outside/their-worktree", "theirs");
    let (got, why) = Cell::of(
        TEARS_DOWN,
        &theirs,
        &torn_down(&theirs, "outside/their-worktree"),
    )
    .came_to();
    assert_eq!(got, Classification::GitMoved, "{why:#?}");
    assert!(
        why.iter()
            .any(|why| why.contains("the registration `theirs`")),
        "{why:#?}"
    );
    // Its directory moved away and the registration dropped: the registration alone.
    let moved_away = then(&theirs, |m| {
        m.files
            .retain(|at, _| !at.starts_with("outside/their-worktree"));
        m.checkouts.remove("outside/their-worktree");
    });
    let dropped = then(&moved_away, |m| m.registrations.clear());
    comes_out(
        &Cell::of(TEARS_DOWN, &moved_away, &dropped),
        Classification::GitMoved,
    );
}

/// **A file of git's that nobody else reads is git's state.** The admin `index` of a
/// registration whose directory is gone, rewritten, is *git's state moved outside its
/// allowance* — no line covers a path inside a `.git` directory but the hook's.
#[test]
fn a_file_of_gits_is_gits_state() {
    const HOOKS: Row = row(&[line("H1", Allows::Files(Where::Hook, May::MergeInto))]);
    let before = then(&sandbox(), |m| {
        put(m, "repo/.git/worktrees/theirs/index", "staged: a.txt\n");
        put(m, "repo/.git/hooks/pre-commit", "#!/bin/sh\nnpm run lint\n");
    });
    let rewritten = then(&before, |m| {
        put(m, "repo/.git/worktrees/theirs/index", "empty\n")
    });
    comes_out(
        &Cell::of(HOOKS, &before, &rewritten),
        Classification::GitMoved,
    );
    let hooked = then(&before, |m| {
        put(
            m,
            "repo/.git/hooks/pre-commit",
            "#!/bin/sh\nnpm run lint\n# jigc\n",
        );
    });
    comes_out(&Cell::of(HOOKS, &before, &hooked), Classification::Holds);
    comes_out(
        &Cell::of(NOTHING, &before, &hooked),
        Classification::GitMoved,
    );
}

/// A row whose command writes a config file and commits it with the index as it stood.
const COMMITS_THE_INDEX: Row = row(&[
    line(
        "K1",
        Allows::Files(Where::Path(".jigc/version"), May::WriteWhole),
    ),
    line("K2", Allows::Commit(Holding::TheIndex)),
]);

/// The sandbox with one file staged by its user, and the file itself.
fn with_staged_code() -> Manifest {
    then(&sandbox(), |m| {
        put(m, "repo/code.rs", "fn main() {}\n");
        m.checkouts
            .get_mut(REPO)
            .expect("the main checkout")
            .index
            .insert("code.rs".to_string(), staged("fn main() {}\n"));
    })
}

/// **Git's state is seen.** A stash entry dropped, a branch moved with no line that
/// commits, a staged path unstaged, a ref made, and the local configuration changed are
/// each *git's state moved outside its allowance* — and a commit under a line that lets
/// the command commit holds.
#[test]
fn gits_state_is_seen() {
    let before = then(&with_staged_code(), |m| m.stash.push("s0".to_string()));
    type Change<'a> = &'a dyn Fn(&mut Manifest);
    let moved: [(&str, Change<'_>); 5] = [
        ("the stash", &|m| m.stash.clear()),
        ("`HEAD` moved", &|m| {
            commit(m, "c1", &[("code.rs", Some((0o100_644, "fn main() {}\n")))])
        }),
        ("the index entry at `code.rs`", &|m| {
            m.checkouts
                .get_mut(REPO)
                .expect("main")
                .index
                .remove("code.rs");
        }),
        ("the ref `refs/heads/kept`", &|m| {
            m.refs
                .insert("refs/heads/kept".to_string(), "c0".to_string());
        }),
        ("configuration", &|m| {
            m.config.push_str("core.hookspath=/elsewhere\n")
        }),
    ];
    for (what, change) in moved {
        let after = then(&before, change);
        let (got, why) = Cell::of(NOTHING, &before, &after).came_to();
        assert_eq!(got, Classification::GitMoved, "{what}: {why:#?}");
        assert!(why.iter().any(|why| why.contains(what)), "{what}: {why:#?}");
    }
    let committed = then(&before, |m| {
        commit(m, "c1", &[("code.rs", Some((0o100_644, "fn main() {}\n")))]);
    });
    comes_out(
        &Cell::of(COMMITS_THE_INDEX, &before, &committed),
        Classification::Holds,
    );
}

/// **A commit holds what its line declares, and nothing else.** Under a line whose commit
/// holds the index as it stood, the user's staged file rides and a file nobody staged does
/// not. Under one whose commit holds only what the row's lines wrote, the user's staged
/// file does not ride. Under one whose commit holds what the other checkouts had staged,
/// a sub-task's staged file rides and the main checkout's does not. A path a line of the
/// file layer wrote rides under all three — as a regular file: **a link there is what no
/// line declares**.
#[test]
fn a_commit_holds_what_its_line_declares() {
    const ONLY_THE_LINES: Row = row(&[
        line(
            "K1",
            Allows::Files(Where::Path(".jigc/version"), May::WriteWhole),
        ),
        line("K2", Allows::Commit(Holding::OnlyTheLines)),
    ]);
    const THE_WORKTREES: Row = row(&[
        line(
            "K1",
            Allows::Files(Where::Path(".jigc/version"), May::WriteWhole),
        ),
        line(
            "K2",
            Allows::Commit(Holding::TheWorktrees(".jigc/worktrees/*")),
        ),
    ]);
    let before = then(
        &with_a_worktree("repo/.jigc/worktrees/sub-1", "sub-1"),
        |m| {
            put(m, "repo/code.rs", "fn main() {}\n");
            put(m, "repo/untracked.rs", "nobody staged this\n");
            m.checkouts
                .get_mut(REPO)
                .expect("the main checkout")
                .index
                .insert("code.rs".to_string(), staged("fn main() {}\n"));
        },
    );
    let stamp = (".jigc/version", Some((0o100_644, "jigc-version: 1\n")));
    let with = |also: &[(&str, Option<(u32, &str)>)]| {
        then(&before, |m| {
            put(m, "repo/.jigc/version", "jigc-version: 1\n");
            let mut changed = vec![stamp];
            changed.extend_from_slice(also);
            commit(m, "c1", &changed);
        })
    };
    let code = ("code.rs", Some((0o100_644, "fn main() {}\n")));
    let untracked = ("untracked.rs", Some((0o100_644, "nobody staged this\n")));
    let work = ("work.txt", Some((0o100_644, "a sub-task's work\n")));
    for (row, rides, expected) in [
        (COMMITS_THE_INDEX, &[][..], Classification::Holds),
        (COMMITS_THE_INDEX, &[code][..], Classification::Holds),
        (
            COMMITS_THE_INDEX,
            &[untracked][..],
            Classification::GitMoved,
        ),
        (COMMITS_THE_INDEX, &[work][..], Classification::GitMoved),
        (ONLY_THE_LINES, &[][..], Classification::Holds),
        (ONLY_THE_LINES, &[code][..], Classification::GitMoved),
        (THE_WORKTREES, &[work][..], Classification::Holds),
        (THE_WORKTREES, &[code][..], Classification::GitMoved),
    ] {
        let (got, why) = Cell::of(row, &before, &with(rides)).came_to();
        assert_eq!(
            got, expected,
            "{:?} with {rides:?}: {why:#?}",
            row.lines[1].allows
        );
    }
    // The line's own path, as a link.
    let linked = then(&before, |m| {
        m.files.insert(
            "repo/.jigc/version".to_string(),
            Entry::Link("elsewhere".to_string()),
        );
        commit(
            m,
            "c1",
            &[(".jigc/version", Some((0o120_000, "elsewhere")))],
        );
    });
    let (got, why) = Cell::of(COMMITS_THE_INDEX, &before, &linked).came_to();
    assert_eq!(got, Classification::GitMoved, "{why:#?}");
    assert!(
        why.iter()
            .any(|why| why.contains("holds a link at `.jigc/version`")),
        "{why:#?}"
    );
}

/// **The home a write copies a committed doc in from is read, and never written**
/// (`May::CopiedFrom`, the `doc` family's `W3`). A write that stages into its task's area
/// and leaves the home as it stood holds. The home rewritten in the working tree is
/// *written outside its allowance* — the line lets nothing change there; and a commit
/// that holds the rewritten home is *git's state moved outside its allowance* — the line
/// lets no commit hold it, though the file the commit holds is the one that stands there.
#[test]
fn the_home_a_write_copies_in_from_is_read_and_never_written() {
    const COPIES_IN: Row = row(&[
        line("R1", Allows::Files(Where::AStagedDocsHome, May::CopiedFrom)),
        line("R2", Allows::Commit(Holding::OnlyTheLines)),
        line(
            "R3",
            Allows::Files(Where::Under(".jigc/tasks/*"), May::Rebuilt),
        ),
    ]);
    const VISION: &str = "# Vision\n\nwhat the project is for\n";
    const SHARPENED: &str = "# Vision\n\nwhat the project is for, sharpened\n";
    let before = then(&sandbox(), |m| {
        dir(m, "repo/docs");
        put(m, "repo/docs/vision.md", VISION);
        commit(m, "c1", &[("docs/vision.md", Some((0o100_644, VISION)))]);
    });
    let staged = then(&before, |m| {
        for path in [
            "repo/.jigc",
            "repo/.jigc/tasks",
            "repo/.jigc/tasks/t1",
            "repo/.jigc/tasks/t1/docs",
        ] {
            dir(m, path);
        }
        put(m, "repo/.jigc/tasks/t1/docs/vision.md", SHARPENED);
    });
    comes_out(
        &Cell::of(COPIES_IN, &before, &staged),
        Classification::Holds,
    );
    let rewritten = then(&staged, |m| put(m, "repo/docs/vision.md", SHARPENED));
    comes_out(
        &Cell::of(COPIES_IN, &before, &rewritten),
        Classification::WrittenOutside,
    );
    let committed = then(&rewritten, |m| {
        commit(m, "c2", &[("docs/vision.md", Some((0o100_644, SHARPENED)))]);
    });
    let (got, why) = Cell::of(COPIES_IN, &before, &committed).came_to();
    assert_eq!(got, Classification::GitMoved, "{why:#?}");
    assert!(
        why.iter().any(|why| why.contains(
            "a commit the command made holds at `docs/vision.md` what the row does not let it hold"
        )),
        "{why:#?}"
    );
}

/// **A commit that must carry what the lines changed** — the human's ruling of 2026-10-14
/// on `uninstall` (`Holding::AllTheLinesChanged`): the teardown lands as one path-scoped
/// commit of what it removed, and never carries the user's own uncommitted edits. A
/// tracked file taken out of and a tracked file removed, with no commit, are *written
/// outside its allowance* — the change is the commit's to land — and the same two in a
/// commit that carries them hold. A file nobody tracked goes with no commit, and holds.
/// And a commit that carries a line of the user's that no commit held — an edit swept in
/// at the path the command changed — is *git's state moved outside its allowance*; under
/// a row whose commit holds only the lines it is too, since 2026-10-19 (the task `B8`), by
/// the word's own content rule over what `HEAD` held: a take-out of `HEAD`'s file holds no
/// line `HEAD` did not.
#[test]
fn a_commit_that_must_carry_what_the_lines_changed() {
    const REMOVED: Line = line("U1", Allows::Files(Where::Under(".jigc"), May::Removed));
    const TAKEN: Line = line(
        "U2",
        Allows::Files(Where::Path("CLAUDE.md"), May::TakeOutOf),
    );
    const TEARS_DOWN_IN_A_COMMIT: Row = row(&[
        REMOVED,
        TAKEN,
        line("U9", Allows::Commit(Holding::AllTheLinesChanged)),
    ]);
    const TEARS_DOWN: Row = row(&[
        REMOVED,
        TAKEN,
        line("U9", Allows::Commit(Holding::OnlyTheLines)),
    ]);
    const INSTALLED: &str = "# the project\n@.jigc/AGENT.md\n";
    const TAKEN_OUT: &str = "# the project\n";
    const EDITED: &str =
        "# the project\nbyte-table-plant-0 a line of the user's\n@.jigc/AGENT.md\n";
    const EDITED_TAKEN_OUT: &str = "# the project\nbyte-table-plant-0 a line of the user's\n";
    let installed = then(&sandbox(), |m| {
        dir(m, "repo/.jigc");
        dir(m, "repo/.jigc/state");
        put(m, "repo/.jigc/AGENT.md", "agent\n");
        put(m, "repo/.jigc/state/hashes.json", "{}\n");
        put(m, "repo/CLAUDE.md", INSTALLED);
        commit(
            m,
            "c1",
            &[
                (".jigc/AGENT.md", Some((0o100_644, "agent\n"))),
                ("CLAUDE.md", Some((0o100_644, INSTALLED))),
            ],
        );
    });
    let torn = |claude: &str| {
        let claude = claude.to_string();
        move |m: &mut Manifest| {
            m.files.retain(|path, _| !path.starts_with("repo/.jigc"));
            put(m, "repo/CLAUDE.md", &claude);
        }
    };
    let in_the_tree = then(&installed, torn(TAKEN_OUT));
    let in_a_commit = then(&in_the_tree, |m| {
        commit(
            m,
            "c2",
            &[
                (".jigc/AGENT.md", None),
                ("CLAUDE.md", Some((0o100_644, TAKEN_OUT))),
            ],
        );
    });
    let (got, why) = Cell::of(TEARS_DOWN_IN_A_COMMIT, &installed, &in_the_tree).came_to();
    assert_eq!(got, Classification::WrittenOutside, "{why:#?}");
    for path in ["repo/.jigc/AGENT.md", "repo/CLAUDE.md"] {
        assert!(
            why.iter()
                .any(|why| why.contains(path) && why.contains("no commit the command made")),
            "{path}: {why:#?}"
        );
    }
    assert!(
        !why.iter().any(|why| why.contains("hashes.json")),
        "a file nobody tracked goes with no commit: {why:#?}"
    );
    comes_out(
        &Cell::of(TEARS_DOWN_IN_A_COMMIT, &installed, &in_a_commit),
        Classification::Holds,
    );
    comes_out(
        &Cell::of(TEARS_DOWN, &installed, &in_the_tree),
        Classification::Holds,
    );

    // The user's own line, in no commit, at the path the teardown takes a line out of.
    let edited = then(&installed, |m| put(m, "repo/CLAUDE.md", EDITED));
    let swept = then(&edited, |m| {
        torn(EDITED_TAKEN_OUT)(m);
        commit(
            m,
            "c2",
            &[
                (".jigc/AGENT.md", None),
                ("CLAUDE.md", Some((0o100_644, EDITED_TAKEN_OUT))),
            ],
        );
    });
    let edit: &[(&str, &str)] = &[("repo/CLAUDE.md", MARKER)];
    let (got, why) = Cell {
        planted: edit,
        ..Cell::of(TEARS_DOWN_IN_A_COMMIT, &edited, &swept)
    }
    .came_to();
    assert_eq!(got, Classification::GitMoved, "{why:#?}");
    assert!(
        why.iter().any(
            |why| why.contains("repo/CLAUDE.md") && why.contains("no object within reach held")
        ),
        "{why:#?}"
    );
    let (got, why) = Cell {
        planted: edit,
        ..Cell::of(TEARS_DOWN, &edited, &swept)
    }
    .came_to();
    assert_eq!(got, Classification::GitMoved, "{why:#?}");
    assert!(
        why.iter()
            .any(|why| why.contains("holds at `CLAUDE.md` what the row does not let it hold")),
        "{why:#?}"
    );
}

/// **An object that is gone is seen**: one that was in the store and is in it no longer is
/// *git's state moved outside its allowance*, whoever reached it. An object more is no
/// difference.
#[test]
fn an_object_that_is_gone_is_seen() {
    let before = then(&sandbox(), |m| {
        m.objects.insert("unreachable".to_string());
    });
    let pruned = then(&before, |m| {
        m.objects.remove("unreachable");
    });
    comes_out(
        &Cell::of(NOTHING, &before, &pruned),
        Classification::GitMoved,
    );
    let grown = then(&before, |m| {
        m.objects.insert("another".to_string());
    });
    comes_out(&Cell::of(NOTHING, &before, &grown), Classification::Holds);
}

/// **A staged doc is accounted for.** A task's working area is removed under the row's
/// line: where the doc staged there is committed at its home as a regular file, the cell
/// holds; where no commit has it and its bytes stand at another path, *git's state moved
/// outside its allowance*; where they stand nowhere, *lost*; and where the commit holds a
/// link at the home in the doc's place, *git's state moved*. **The unit's commit doc is
/// not prose to promote**: it is rendered into the commit's message, and its file goes
/// with the working area.
#[test]
fn a_staged_doc_is_accounted_for() {
    const FINALIZES: Row = Row {
        unit: Unit::TheWordAfterTheLeaf,
        ..row(&[
            line(
                "F1",
                Allows::Files(Where::AStagedDocsHome, May::CopyOfAStagedDoc),
            ),
            line("F2", Allows::Commit(Holding::TheIndex)),
            line(
                "F3",
                Allows::Files(Where::Under(".jigc/tasks/*"), May::Removed),
            ),
        ])
    };
    // The command finalizes `t1`: the unit whose staged docs `F1` lets land.
    fn finalizing<'a>(before: &'a Manifest, after: &'a Manifest) -> Cell<'a> {
        Cell {
            argv: &["probe", "t1"],
            ..Cell::of(FINALIZES, before, after)
        }
    }
    const PROSE: &str = "# Use tabs\n\nthe prose somebody handed to jigc\n";
    let before = then(&sandbox(), |m| {
        for path in [
            "repo/.jigc",
            "repo/.jigc/tasks",
            "repo/.jigc/tasks/t1",
            "repo/.jigc/tasks/t1/docs",
            "repo/docs",
        ] {
            dir(m, path);
        }
        put(m, "repo/.jigc/tasks/t1/docs/use-tabs.md", PROSE);
    });
    let area_gone = |m: &mut Manifest| {
        m.files
            .retain(|path, _| !path.starts_with("repo/.jigc/tasks/t1"))
    };
    let landed = then(&before, |m| {
        area_gone(m);
        put(m, "repo/docs/use-tabs.md", PROSE);
        commit(m, "c1", &[("docs/use-tabs.md", Some((0o100_644, PROSE)))]);
    });
    comes_out(&finalizing(&before, &landed), Classification::Holds);
    let nowhere = then(&before, area_gone);
    comes_out(&finalizing(&before, &nowhere), Classification::Lost);
    let elsewhere = then(&before, |m| {
        area_gone(m);
        put(m, "repo/docs/nowhere.md", PROSE);
    });
    comes_out(&finalizing(&before, &elsewhere), Classification::GitMoved);
    // The row's own case: a link at the doc's home, the prose written through it.
    let through_a_link = then(&before, |m| {
        area_gone(m);
        m.files.insert(
            "repo/docs/use-tabs.md".to_string(),
            Entry::Link("nowhere.md".to_string()),
        );
        put(m, "repo/docs/nowhere.md", PROSE);
        commit(
            m,
            "c1",
            &[("docs/use-tabs.md", Some((0o120_000, "nowhere.md")))],
        );
    });
    let (got, why) = finalizing(&before, &through_a_link).came_to();
    assert_eq!(got, Classification::GitMoved, "{why:#?}");
    assert!(
        why.iter()
            .any(|why| why.contains("holds a link at `docs/use-tabs.md`")),
        "{why:#?}"
    );
    // The area still there: nothing is asked of the doc yet.
    comes_out(&finalizing(&before, &before), Classification::Holds);
    // The unit's commit doc is rendered into the commit's message and promoted nowhere:
    // its file goes with the working area, and nothing asks where its bytes stand. A doc
    // whose name only begins like it is prose like any other.
    let with_its_commit_doc = then(&before, |m| {
        put(
            m,
            "repo/.jigc/tasks/t1/docs/commit:t1.md",
            "# docs: use tabs\n\nthe message somebody handed to jigc\n",
        );
    });
    let landed = then(&with_its_commit_doc, |m| {
        area_gone(m);
        put(m, "repo/docs/use-tabs.md", PROSE);
        commit(m, "c1", &[("docs/use-tabs.md", Some((0o100_644, PROSE)))]);
    });
    comes_out(
        &finalizing(&with_its_commit_doc, &landed),
        Classification::Holds,
    );
    let with_a_lookalike = then(&before, |m| {
        put(
            m,
            "repo/.jigc/tasks/t1/docs/commit-rules.md",
            "# Commit rules\n\nmore prose somebody handed to jigc\n",
        );
    });
    let landed = then(&with_a_lookalike, |m| {
        area_gone(m);
        put(m, "repo/docs/use-tabs.md", PROSE);
        commit(m, "c1", &[("docs/use-tabs.md", Some((0o100_644, PROSE)))]);
    });
    comes_out(
        &finalizing(&with_a_lookalike, &landed),
        Classification::Lost,
    );

    // **One move is the lines' to judge: a re-slug.** A doc re-slugged inside a task that
    // stands — `doc rename`, its body under a new name and a new `# H1` — holds under a row
    // that lets it (`May::ReSlugged`); the same move in an area whose pin is gone, a file
    // of another's keeping the directory, is a staged doc lost.
    const RE_SLUGS: Row = row(&[
        line(
            "F4",
            Allows::Files(Where::Under(".jigc/tasks/*"), May::Rebuilt),
        ),
        line(
            "F5",
            Allows::Files(Where::Path(".jigc/tasks/*/docs/*"), May::ReSlugged),
        ),
    ]);
    const RE_SLUGGED: &str = "# Use spaces\n\nthe prose somebody handed to jigc\n";
    let a_task = then(&before, |m| put(m, "repo/.jigc/tasks/t1/base.json", "{}\n"));
    let moved = |m: &mut Manifest| {
        m.files.remove("repo/.jigc/tasks/t1/docs/use-tabs.md");
        put(m, "repo/.jigc/tasks/t1/docs/use-spaces.md", RE_SLUGGED);
    };
    comes_out(
        &Cell::of(RE_SLUGS, &a_task, &then(&a_task, moved)),
        Classification::Holds,
    );
    let unpinned = then(&a_task, |m| {
        moved(m);
        m.files.remove("repo/.jigc/tasks/t1/base.json");
    });
    comes_out(
        &Cell::of(RE_SLUGS, &a_task, &unpinned),
        Classification::Lost,
    );

    // **And only the re-slug** (the review of the doc family, its second finding): a
    // standing pin is no excuse for a staged doc gone. Another unit's doc taken by a
    // finalize that removes its own area, under a line that lets an area go, its pin left
    // standing, is lost.
    let two_tasks = then(&a_task, |m| {
        for path in ["repo/.jigc/tasks/t2", "repo/.jigc/tasks/t2/docs"] {
            dir(m, path);
        }
        put(m, "repo/.jigc/tasks/t2/base.json", "{}\n");
        put(
            m,
            "repo/.jigc/tasks/t2/docs/use-colours.md",
            "# Use colours\n\nthe prose another task handed to jigc\n",
        );
    });
    let t1_landed = |m: &mut Manifest| {
        m.files
            .retain(|path, _| !path.starts_with("repo/.jigc/tasks/t1"));
        put(m, "repo/docs/use-tabs.md", PROSE);
        commit(m, "c1", &[("docs/use-tabs.md", Some((0o100_644, PROSE)))]);
    };
    comes_out(
        &finalizing(&two_tasks, &then(&two_tasks, t1_landed)),
        Classification::Holds,
    );
    let and_t2s_doc = then(&two_tasks, |m| {
        t1_landed(m);
        m.files.remove("repo/.jigc/tasks/t2/docs/use-colours.md");
    });
    comes_out(&finalizing(&two_tasks, &and_t2s_doc), Classification::Lost);
    // Under the line a re-slug is let go by: a doc gone with nothing gained in its unit, a
    // doc gone where its unit gained another doc's prose, and a doc gone where another
    // unit gained its prose under the new name are each a staged doc lost.
    let gone = then(&a_task, |m| {
        m.files.remove("repo/.jigc/tasks/t1/docs/use-tabs.md");
    });
    comes_out(&Cell::of(RE_SLUGS, &a_task, &gone), Classification::Lost);
    let another_doc = then(&gone, |m| {
        put(
            m,
            "repo/.jigc/tasks/t1/docs/use-spaces.md",
            "# Use spaces\n\nother prose, minted in its place\n",
        );
    });
    comes_out(
        &Cell::of(RE_SLUGS, &a_task, &another_doc),
        Classification::Lost,
    );
    let in_another_unit = then(&two_tasks, |m| {
        m.files.remove("repo/.jigc/tasks/t1/docs/use-tabs.md");
        put(m, "repo/.jigc/tasks/t2/docs/use-spaces.md", RE_SLUGGED);
    });
    comes_out(
        &Cell::of(RE_SLUGS, &two_tasks, &in_another_unit),
        Classification::Lost,
    );
}

/// **A line reaches its own unit's area and no sibling's** — the task `B7`'s first part, for
/// the doc family's review (its seventh finding): a line whose place reads `.jigc/tasks/*`
/// lets a command rewrite any task's area, where *"a sub-agent physically cannot stage into
/// a sibling's area"* (`design/write-commands.md` → *The write-time `--task`-scoped barrier
/// (M8)*). A place written with [`byte_cell::OWN`] is the area of the unit the row's
/// [`Unit`] reads off the command, and one with [`byte_cell::ITS_SUB_TASKS`] the areas of
/// the sub-tasks that unit's committed record names.
///
/// Under `--task t1`, t1's staged doc rewritten holds and t2's is *written outside its
/// allowance* — the product's own bytes, written by a lead-in, so no loss, and no line's.
/// With no flag the one task that stands is the unit; with two, neither is. And a
/// milestone's line lets its own sub-task's area go and not a task its record does not
/// name.
#[test]
fn a_line_reaches_its_own_units_area_and_no_siblings() {
    const STAGES: Row = Row {
        unit: Unit::TheTaskFlag,
        ..row(&[line(
            "O1",
            Allows::Files(Where::Under(".jigc/tasks/<unit>"), May::Rebuilt),
        )])
    };
    let base = then(&sandbox(), |m| dir(m, "repo/.jigc"));
    let a_task = |m: &mut Manifest, id: &str, doc: &str| {
        for path in ["", "/docs"] {
            dir(m, &format!("repo/.jigc/tasks/{id}{path}"));
        }
        put(m, &format!("repo/.jigc/tasks/{id}/base.json"), "{}\n");
        put(
            m,
            &format!("repo/.jigc/tasks/{id}/docs/{doc}.md"),
            &format!("# {doc}\n\nthe prose task {id} handed to jigc\n"),
        );
    };
    let one = then(&base, |m| {
        dir(m, "repo/.jigc/tasks");
        a_task(m, "t1", "use-tabs");
    });
    let two = then(&one, |m| a_task(m, "t2", "use-colours"));
    let rewritten = |before: &Manifest, path: &str| {
        then(before, |m| put(m, path, "# rewritten\n\nother prose\n"))
    };
    const T1: &str = "repo/.jigc/tasks/t1/docs/use-tabs.md";
    const T2: &str = "repo/.jigc/tasks/t2/docs/use-colours.md";
    let came_to = |before: &Manifest, after: &Manifest, argv: &[&str]| {
        Cell {
            base: Some(&base),
            argv,
            ..Cell::of(STAGES, before, after)
        }
        .came_to()
    };
    let flagged: &[&str] = &["doc", "add-item", "x:x#y", "--task", "t1"];
    let (class, why) = came_to(&two, &rewritten(&two, T1), flagged);
    assert_eq!(class, Classification::Holds, "its own area: {why:#?}");
    let (class, why) = came_to(&two, &rewritten(&two, T2), flagged);
    assert!(
        class == Classification::WrittenOutside
            && why.iter().any(|why| why.contains("use-colours.md")),
        "a sibling's staged doc rewritten is no line's: {class}, {why:#?}"
    );
    let plain: &[&str] = &["doc", "add-item", "x:x#y"];
    let (class, why) = came_to(&one, &rewritten(&one, T1), plain);
    assert_eq!(
        class,
        Classification::Holds,
        "the one task that stands: {why:#?}"
    );
    let (class, why) = came_to(&two, &rewritten(&two, T1), plain);
    assert_eq!(
        class,
        Classification::WrittenOutside,
        "two tasks and no flag name no unit: {why:#?}"
    );
    // A task stands where its area carries its base pin — *"A task is an area that carries
    // its base pin"* (`design/storage.md` → *The per-task working area (staging)*) — so a
    // leftover directory beside one task leaves that task the one that stands, and a
    // leftover alone is no unit.
    const LEFTOVER: &str = "repo/.jigc/tasks/t2/docs/use-colours.md";
    let one_and_a_leftover = then(&one, |m| {
        for path in ["repo/.jigc/tasks/t2", "repo/.jigc/tasks/t2/docs"] {
            dir(m, path);
        }
        put(
            m,
            LEFTOVER,
            "# use-colours\n\nbytes a faulted teardown left\n",
        );
    });
    let (class, why) = came_to(
        &one_and_a_leftover,
        &rewritten(&one_and_a_leftover, T1),
        plain,
    );
    assert_eq!(
        class,
        Classification::Holds,
        "the one task that stands beside a leftover: {why:#?}"
    );
    let a_leftover_alone = then(&one_and_a_leftover, |m| {
        m.files
            .retain(|path, _| !path.starts_with("repo/.jigc/tasks/t1"));
    });
    let (class, why) = came_to(
        &a_leftover_alone,
        &rewritten(&a_leftover_alone, LEFTOVER),
        plain,
    );
    assert!(
        class == Classification::WrittenOutside
            && why.iter().any(|why| why.contains("use-colours.md")),
        "a leftover alone is no task that stands: {class}, {why:#?}"
    );

    const ITS_OWN: Row = Row {
        leaf: &["milestone", "finalize"],
        unit: Unit::TheWordAfterTheLeaf,
        ..row(&[line(
            "O2",
            Allows::Files(Where::Under(".jigc/tasks/<sub-task>"), May::Removed),
        )])
    };
    let a_milestone = then(&base, |m| {
        dir(m, "repo/.jigc/tasks");
        for id in ["s1", "t9"] {
            dir(m, &format!("repo/.jigc/tasks/{id}"));
            put(m, &format!("repo/.jigc/tasks/{id}/intent"), "an intent\n");
        }
        records(m, "docs", &[("m", &["s1"])]);
    });
    let gone = |id: &str| {
        then(&a_milestone, |m| {
            let area = format!("repo/.jigc/tasks/{id}");
            m.files.retain(|path, _| !path.starts_with(&area));
        })
    };
    let argv: &[&str] = &["milestone", "finalize", "m"];
    let (class, why) = Cell {
        base: Some(&base),
        argv,
        ..Cell::of(ITS_OWN, &a_milestone, &gone("s1"))
    }
    .came_to();
    assert_eq!(
        class,
        Classification::Holds,
        "its own sub-task's area: {why:#?}"
    );
    let after = gone("t9");
    let (class, why) = Cell {
        base: Some(&base),
        argv,
        ..Cell::of(ITS_OWN, &a_milestone, &after)
    }
    .came_to();
    assert!(
        class == Classification::WrittenOutside && why.iter().any(|why| why.contains("t9")),
        "a task its record does not name: {class}, {why:#?}"
    );

    // **The three sets a line is read through, beside its place** (the review of the task
    // `B7`'s first part, its second finding): the docs staged in the unit, whose copies a
    // line lets land; every unit's staged docs, which the loss loop holds to a commit; and
    // the checkouts whose staged code a fan-out's commit may hold.
    const PROMOTES: Row = Row {
        leaf: &["task", "finalize"],
        unit: Unit::TheWordAfterTheLeaf,
        ..row(&[
            line(
                "O3",
                Allows::Files(Where::AStagedDocsHome, May::CopyOfAStagedDoc),
            ),
            line("O4", Allows::Commit(Holding::OnlyTheLines)),
            line(
                "O5",
                Allows::Files(Where::Under(".jigc/tasks/<unit>"), May::Removed),
            ),
        ])
    };
    let finalize_t1: &[&str] = &["task", "finalize", "t1"];
    let in_docs = |before: &Manifest| then(before, |m| dir(m, "repo/docs"));
    let promoted = |before: &Manifest, docs: &[(&str, &str)]| {
        then(before, |m| {
            m.files
                .retain(|path, _| !path.starts_with("repo/.jigc/tasks/t1"));
            let mut changed = Vec::new();
            for (name, bytes) in docs {
                put(m, &format!("repo/docs/{name}.md"), bytes);
                changed.push((format!("docs/{name}.md"), *bytes));
            }
            let changed: Vec<(&str, Option<(u32, &str)>)> = changed
                .iter()
                .map(|(path, bytes)| (path.as_str(), Some((0o100_644, *bytes))))
                .collect();
            if !changed.is_empty() {
                commit(m, "c1", &changed);
            }
        })
    };
    const TABS: &str = "# use-tabs\n\nthe prose task t1 handed to jigc\n";
    const COLOURS: &str = "# use-colours\n\nthe prose task t2 handed to jigc\n";
    let siblings = in_docs(&two);
    let came = |before: &Manifest, after: &Manifest, row: Row, argv: &[&str]| {
        Cell {
            base: Some(&base),
            argv,
            ..Cell::of(row, before, after)
        }
        .came_to()
    };
    let (class, why) = came(
        &siblings,
        &promoted(&siblings, &[("use-tabs", TABS)]),
        PROMOTES,
        finalize_t1,
    );
    assert_eq!(
        class,
        Classification::Holds,
        "its own doc promoted: {why:#?}"
    );
    // A sibling's staged doc promoted with it is no copy of a doc of the unit's.
    let (class, why) = came(
        &siblings,
        &promoted(&siblings, &[("use-tabs", TABS), ("use-colours", COLOURS)]),
        PROMOTES,
        finalize_t1,
    );
    assert!(
        class == Classification::GitMoved && why.iter().any(|why| why.contains("use-colours.md")),
        "a sibling's staged doc promoted is no line's: {class}, {why:#?}"
    );

    // A joined milestone stands in the repository: its merged set is no reason to leave a
    // task's own staged docs unread. t1's doc dropped is *lost*, and promoted it holds.
    let beside_a_joined_milestone = then(&siblings, |m| {
        for path in [
            "repo/.jigc/milestones",
            "repo/.jigc/milestones/m",
            "repo/.jigc/milestones/m/merged",
            "repo/.jigc/milestones/m/merged/docs",
        ] {
            dir(m, path);
        }
        put(
            m,
            "repo/.jigc/milestones/m/merged/docs/joined.md",
            "# joined\n\nthe prose a joined milestone holds\n",
        );
    });
    let (class, why) = came(
        &beside_a_joined_milestone,
        &promoted(&beside_a_joined_milestone, &[]),
        PROMOTES,
        finalize_t1,
    );
    assert!(
        class == Classification::Lost && why.iter().any(|why| why.contains("use-tabs.md")),
        "its own doc dropped beside a joined milestone: {class}, {why:#?}"
    );
    let (class, why) = came(
        &beside_a_joined_milestone,
        &promoted(&beside_a_joined_milestone, &[("use-tabs", TABS)]),
        PROMOTES,
        finalize_t1,
    );
    assert_eq!(
        class,
        Classification::Holds,
        "its own doc promoted beside a joined milestone: {why:#?}"
    );

    // A milestone's boundary lands its own merged set, and no other milestone's.
    const LANDS: Row = Row {
        leaf: &["milestone", "finalize"],
        unit: Unit::TheWordAfterTheLeaf,
        ..row(&[
            line(
                "O6",
                Allows::Files(Where::AStagedDocsHome, May::CopyOfAStagedDoc),
            ),
            line("O7", Allows::Commit(Holding::OnlyTheLines)),
        ])
    };
    let two_milestones = then(&in_docs(&base), |m| {
        dir(m, "repo/.jigc/milestones");
        for id in ["m", "m2"] {
            for path in ["", "/merged", "/merged/docs"] {
                dir(m, &format!("repo/.jigc/milestones/{id}{path}"));
            }
            put(
                m,
                &format!("repo/.jigc/milestones/{id}/merged/docs/joined-{id}.md"),
                &format!("# joined-{id}\n\nthe prose milestone {id} joined\n"),
            );
        }
    });
    let landed = |id: &str| {
        then(&two_milestones, |m| {
            let bytes = format!("# joined-{id}\n\nthe prose milestone {id} joined\n");
            let path = format!("docs/joined-{id}.md");
            put(m, &format!("repo/{path}"), &bytes);
            commit(
                m,
                "c1",
                &[(path.as_str(), Some((0o100_644, bytes.as_str())))],
            );
        })
    };
    let finalize_m: &[&str] = &["milestone", "finalize", "m"];
    let (class, why) = came(&two_milestones, &landed("m"), LANDS, finalize_m);
    assert_eq!(class, Classification::Holds, "its own merged doc: {why:#?}");
    let (class, why) = came(&two_milestones, &landed("m2"), LANDS, finalize_m);
    assert!(
        class == Classification::GitMoved && why.iter().any(|why| why.contains("joined-m2.md")),
        "another milestone's merged doc landed is no line's: {class}, {why:#?}"
    );

    // A fan-out's commit holds what its own sub-tasks' worktrees staged — a checkout at
    // `.jigc/worktrees/<sub-task>` — and not what another checkout did: a task's worktree
    // its record does not name, or the user's own.
    const FOLDS: Row = Row {
        leaf: &["milestone", "finalize"],
        unit: Unit::TheWordAfterTheLeaf,
        ..row(&[line(
            "O8",
            Allows::Commit(Holding::TheWorktrees(".jigc/worktrees/<sub-task>")),
        )])
    };
    let worktrees = then(&a_milestone, |m| {
        for (at, file) in [
            ("repo/.jigc/worktrees/s1", "s1.txt"),
            ("repo/.jigc/worktrees/t9", "t9.txt"),
            ("outside/own", "own.txt"),
        ] {
            let bytes = format!("the work staged at {at}\n");
            m.checkouts.insert(
                at.to_string(),
                Checkout {
                    head: "c0".to_string(),
                    branch: None,
                    index: [(file.to_string(), staged(&bytes))].into(),
                },
            );
            m.held.insert(blob_of(&bytes));
        }
    });
    for (at, file, expected) in [
        ("repo/.jigc/worktrees/s1", "s1.txt", Classification::Holds),
        (
            "repo/.jigc/worktrees/t9",
            "t9.txt",
            Classification::GitMoved,
        ),
        ("outside/own", "own.txt", Classification::GitMoved),
    ] {
        let bytes = format!("the work staged at {at}\n");
        let folded = then(&worktrees, |m| {
            put(m, &format!("repo/{file}"), &bytes);
            commit(m, "c1", &[(file, Some((0o100_644, bytes.as_str())))]);
        });
        let (class, why) = came(&worktrees, &folded, FOLDS, argv);
        assert_eq!(
            class, expected,
            "what the checkout at `{at}` staged: {why:#?}"
        );
    }
}

/// **A line reaches its own unit's record, at the home the docs root gives it, and reads the
/// record as committed** — the review of the task `B7`'s first part, its fifth, sixth and
/// seventh findings. `M5` and `A5` read `docs/milestone-records/*` and let a command write
/// any milestone's record, where each sentence names one: `M5`'s the record of the milestone
/// the command finalizes, `A5`'s the record whose item names the sub-task it discards. A
/// place's component may be [`byte_cell::OWN`] or [`byte_cell::ITS_MILESTONE`] with a fixed
/// suffix, and [`byte_cell::DOCS_ROOT`] leads it to the directory the cascade resolves
/// `docs-root` to — *"The resolved `docs-root` is prepended to every persisted doctype's
/// `location:` at schema load"* (`design/storage.md` → *Config layout*) — the project layer
/// over the team layer over the pack's default. The sub-tasks [`byte_cell::ITS_SUB_TASKS`]
/// stands for are read off the same home, and off the bytes `HEAD` holds there: a record
/// with a change nobody committed names no sub-task.
#[test]
fn a_line_reaches_its_units_record_where_the_docs_root_has_it() {
    let base = then(&sandbox(), |m| dir(m, "repo/.jigc"));
    let came = |row: Row, before: &Manifest, after: &Manifest, argv: &[&str]| {
        Cell {
            base: Some(&base),
            argv,
            ..Cell::of(row, before, after)
        }
        .came_to()
    };
    let flipped = |before: &Manifest, path: &str| {
        then(before, |m| {
            let Some(Entry::File { bytes, .. }) = m.files.get(path) else {
                panic!("no record at {path}");
            };
            let bytes = format!("{}- status: joined\n", String::from_utf8_lossy(bytes));
            put(m, path, &bytes);
        })
    };
    const M: &str = "repo/docs/milestone-records/m.md";
    const M2: &str = "repo/docs/milestone-records/m2.md";
    let two = then(&base, |m| {
        records(m, "docs", &[("m", &["s1"]), ("m2", &["t9"])]);
    });

    // `M5`: the record of the milestone the command finalizes, and no other.
    let finalize_m: &[&str] = &["milestone", "finalize", "m"];
    let (class, why) = came(milestone::FINALIZE, &two, &flipped(&two, M), finalize_m);
    assert_eq!(class, Classification::Holds, "its own record: {why:#?}");
    let (class, why) = came(milestone::FINALIZE, &two, &flipped(&two, M2), finalize_m);
    assert!(
        class == Classification::WrittenOutside && why.iter().any(|why| why.contains("m2.md")),
        "another milestone's record is no line's: {class}, {why:#?}"
    );

    // `A5`: the record whose item names the sub-task the command discards — none where no
    // record names it, and none where two do.
    let discard = |id: &'static str| -> Vec<&'static str> { vec!["task", "discard", id] };
    let (class, why) = came(task::DISCARD, &two, &flipped(&two, M), &discard("s1"));
    assert_eq!(
        class,
        Classification::Holds,
        "the record that names the sub-task: {why:#?}"
    );
    let (class, why) = came(task::DISCARD, &two, &flipped(&two, M2), &discard("s1"));
    assert!(
        class == Classification::WrittenOutside && why.iter().any(|why| why.contains("m2.md")),
        "a record that does not name the sub-task is no line's: {class}, {why:#?}"
    );
    let (class, why) = came(task::DISCARD, &two, &flipped(&two, M), &discard("t5"));
    assert!(
        class == Classification::WrittenOutside && why.iter().any(|why| why.contains("m.md")),
        "a task no record names reaches no record: {class}, {why:#?}"
    );
    let named_twice = then(&base, |m| {
        records(m, "docs", &[("m", &["s1"]), ("m2", &["s1"])]);
    });
    let (class, why) = came(
        task::DISCARD,
        &named_twice,
        &flipped(&named_twice, M),
        &discard("s1"),
    );
    assert!(
        class == Classification::WrittenOutside && why.iter().any(|why| why.contains("m.md")),
        "a task two records name has no one record: {class}, {why:#?}"
    );

    // The docs root, as the cascade resolves it: the record at `notes/` under a project
    // layer that says `notes`, and the one at `docs/` no record of the unit's.
    const AREAS: Row = Row {
        leaf: &["milestone", "finalize"],
        unit: Unit::TheWordAfterTheLeaf,
        ..row(&[line(
            "O9",
            Allows::Files(Where::Under(".jigc/tasks/<sub-task>"), May::Removed),
        )])
    };
    let layer = |m: &mut Manifest, at: &str, value: &str| {
        let mut path = String::new();
        let dirs: Vec<&str> = at.split('/').collect();
        for component in &dirs[..dirs.len() - 1] {
            path = if path.is_empty() {
                component.to_string()
            } else {
                format!("{path}/{component}")
            };
            dir(m, &path);
        }
        put(m, at, &format!("scalar:\n  docs-root: {value}\n"));
    };
    const PROJECT: &str = "repo/.jigc/config/manifest.yaml";
    const TEAM: &str = "home/.config/jigc/manifest.yaml";
    let with_areas = |m: &mut Manifest| {
        dir(m, "repo/.jigc/tasks");
        for id in ["s1", "t9"] {
            dir(m, &format!("repo/.jigc/tasks/{id}"));
            put(m, &format!("repo/.jigc/tasks/{id}/intent"), "an intent\n");
        }
    };
    let area_gone = |before: &Manifest, id: &str| {
        then(before, |m| {
            let area = format!("repo/.jigc/tasks/{id}");
            m.files.retain(|path, _| !path.starts_with(&area));
        })
    };
    const NOTES: &str = "repo/notes/milestone-records/m.md";
    for (layers, root, (holds, outside)) in [
        (&[(PROJECT, "notes")][..], "notes", ("s1", "t9")),
        (&[(TEAM, "notes")][..], "notes", ("s1", "t9")),
        (
            &[(TEAM, "notes"), (PROJECT, "docs/")][..],
            "docs",
            ("t9", "s1"),
        ),
    ] {
        let before = then(&base, |m| {
            for (at, value) in layers {
                layer(m, at, value);
            }
            with_areas(m);
            records(m, "notes", &[("m", &["s1"])]);
            records(m, "docs", &[("m", &["t9"])]);
        });
        let (record, other) = if root == "notes" {
            (NOTES, M)
        } else {
            (M, NOTES)
        };
        let (class, why) = came(
            milestone::FINALIZE,
            &before,
            &flipped(&before, record),
            finalize_m,
        );
        assert_eq!(
            class,
            Classification::Holds,
            "the record at `{root}`, under {layers:?}: {why:#?}"
        );
        let (class, why) = came(
            milestone::FINALIZE,
            &before,
            &flipped(&before, other),
            finalize_m,
        );
        assert_eq!(
            class,
            Classification::WrittenOutside,
            "the record at the other root, under {layers:?}: {why:#?}"
        );
        let (class, why) = came(AREAS, &before, &area_gone(&before, holds), finalize_m);
        assert_eq!(
            class,
            Classification::Holds,
            "`{holds}`, which the record at `{root}` names, under {layers:?}: {why:#?}"
        );
        let (class, why) = came(AREAS, &before, &area_gone(&before, outside), finalize_m);
        assert_eq!(
            class,
            Classification::WrittenOutside,
            "`{outside}`, which only the record at the other root names, under {layers:?}: \
             {why:#?}"
        );
    }
    // `.` is the repository's root: the record at `milestone-records/`.
    let flat = then(&base, |m| {
        layer(m, PROJECT, ".");
        records(m, "", &[("m", &["s1"])]);
    });
    let (class, why) = came(
        milestone::FINALIZE,
        &flat,
        &flipped(&flat, "repo/milestone-records/m.md"),
        finalize_m,
    );
    assert_eq!(
        class,
        Classification::Holds,
        "the record at the root, under `docs-root: .`: {why:#?}"
    );

    // The record as committed: an item added and never committed names no sub-task.
    let hand_edited = then(&two, |m| {
        with_areas(m);
        let Some(Entry::File { bytes, .. }) = m.files.get(M) else {
            panic!("no record at {M}");
        };
        let bytes = format!(
            "{}\n### t9  {{#t9}}\n\n<!-- fields -->\n- intent: an intent\n",
            String::from_utf8_lossy(bytes)
        );
        put(m, M, &bytes);
    });
    let (class, why) = came(
        AREAS,
        &hand_edited,
        &area_gone(&hand_edited, "t9"),
        finalize_m,
    );
    assert!(
        class == Classification::WrittenOutside && why.iter().any(|why| why.contains("t9")),
        "an item nobody committed names no sub-task: {class}, {why:#?}"
    );
}

/// **Under `squash: false` a boundary is a chain: a commit a code-carrying sub-task, in
/// task-id order, each holding its own worktree's code, then the aggregate** — the review of
/// the task `B7`'s first part, its fourth finding: `M4` held what `M3` holds, so a product
/// that ignored the knob kept every cell of the spelling green. On the row
/// `milestone::FINALIZE` itself, run as `squash: false`: a milestone `m` whose committed
/// record names `s1`, `s2` and `s3`, each with its worktree at `.jigc/worktrees/<id>` —
/// `s1` and `s2` staged a file of code there, `s3` nothing. The chain `s1`, `s2`, the
/// aggregate holds; so does the one combined commit under the plain spelling (`M3`). Under
/// `squash: false` that combined commit is *git's state moved outside its allowance*, and
/// so are the chain out of order, a sub-task's commit carrying its sibling's code, a chain
/// with no aggregate, the aggregate carrying code, a commit for the sub-task that staged
/// nothing, and no commit at all.
#[test]
fn a_chain_holds_a_commit_a_sub_task_in_id_order_then_the_aggregate() {
    const RECORD: &str = "docs/milestone-records/m.md";
    let before = then(&sandbox(), |m| {
        records(m, "docs", &[("m", &["s1", "s2", "s3"])]);
        let main = m.checkouts.get(REPO).expect("the main checkout").clone();
        for at in ["repo/.jigc", "repo/.jigc/worktrees"] {
            dir(m, at);
        }
        for id in ["s1", "s2", "s3"] {
            let at = format!("repo/.jigc/worktrees/{id}");
            dir(m, &at);
            let mut index = main.index.clone();
            if id != "s3" {
                let bytes = format!("the code {id} staged\n");
                put(m, &format!("{at}/{id}.txt"), &bytes);
                index.insert(format!("{id}.txt"), staged(&bytes));
                m.held.insert(blob_of(&bytes));
            }
            m.registrations.insert(id.to_string(), at.clone());
            m.checkouts.insert(
                at,
                Checkout {
                    head: main.head.clone(),
                    branch: None,
                    index,
                },
            );
        }
    });
    let code = |id: &str| (format!("{id}.txt"), format!("the code {id} staged\n"));
    let flipped = |m: &Manifest| {
        let Some(Entry::File { bytes, .. }) = m.files.get(&format!("repo/{RECORD}")) else {
            panic!("no record");
        };
        format!("{}- status: joined\n", String::from_utf8_lossy(bytes))
    };
    // The commits, oldest first, each the paths it lands: a sub-task's code by its id, the
    // record flipped by `RECORD`.
    let landed = |links: &[&[&str]]| {
        then(&before, |m| {
            let record = flipped(m);
            for (at, link) in links.iter().enumerate() {
                let changed: Vec<(String, String)> = link
                    .iter()
                    .map(|what| match *what {
                        RECORD => (RECORD.to_string(), record.clone()),
                        id => code(id),
                    })
                    .collect();
                for (path, bytes) in &changed {
                    put(m, &format!("repo/{path}"), bytes);
                }
                let changed: Vec<(&str, Option<(u32, &str)>)> = changed
                    .iter()
                    .map(|(path, bytes)| (path.as_str(), Some((0o100_644, bytes.as_str()))))
                    .collect();
                commit(m, &format!("chain-{at}"), &changed);
            }
        })
    };
    let came = |spelling: Option<&'static str>, after: &Manifest| {
        Cell {
            argv: &["milestone", "finalize", "m"],
            spelling,
            ..Cell::of(milestone::FINALIZE, &before, after)
        }
        .came_to()
    };
    const UNSQUASHED: Option<&str> = Some("squash: false");

    let (class, why) = came(UNSQUASHED, &landed(&[&["s1"], &["s2"], &[RECORD]]));
    assert_eq!(class, Classification::Holds, "the chain: {why:#?}");
    let combined = landed(&[&["s1", "s2", RECORD]]);
    let (class, why) = came(None, &combined);
    assert_eq!(
        class,
        Classification::Holds,
        "one combined commit, `M3`: {why:#?}"
    );
    for (what, after, says) in [
        ("one combined commit", combined.clone(), "made 1 commit"),
        (
            "the chain out of order",
            landed(&[&["s2"], &["s1"], &[RECORD]]),
            "commit 1 of the chain under the line `M4` is the one of `s1`",
        ),
        (
            "a sub-task's commit carrying its sibling's code",
            landed(&[&["s1", "s2"], &[], &[RECORD]]),
            "is the one of `s1`",
        ),
        (
            "no aggregate",
            landed(&[&["s1"], &["s2", RECORD]]),
            "made 2 commits",
        ),
        (
            "the aggregate carrying code",
            landed(&[&["s1"], &[], &["s2", RECORD]]),
            "is the one of `s2`",
        ),
        (
            "a commit for the sub-task that staged nothing",
            landed(&[&["s1"], &["s2"], &[], &[RECORD]]),
            "made 4 commits",
        ),
        ("no commit at all", before.clone(), "made no commit"),
    ] {
        let (class, why) = came(UNSQUASHED, &after);
        assert!(
            class == Classification::GitMoved && why.iter().any(|why| why.contains(says)),
            "{what}: {class}, {why:#?}"
        );
    }
}

/// **A place a command mints is one where nothing stood, at the id the command minted** —
/// the milestone family's rows, the task `B7`'s second part: `jigc milestone create` mints a
/// milestone whose id is its title's slug, `add-task` a sub-task whose id is its intent's,
/// `add-from-spec` one a criterion of a committed spec, so the argv names none of them by
/// its id. A place's component may be [`byte_cell::MINTED`], alone or with a fixed suffix:
/// the one id the command minted — the record `HEAD` holds after it gains it, a milestone's
/// record at the home or a sub-task's item in the unit's record, where it did not before —
/// at which nothing stood before the command, in the working tree or in what git held
/// there: *"both mints still refuse a pre-existing id before creating anything"*, and *"a
/// home with nothing on disk is a free id only when git holds nothing there either"*
/// (`design/team-ready-state.md` → *The commit model*, → *The lifecycle*). Or
/// [`byte_cell::EACH_MINTED`], any id of those, where the door mints one *"per criterion"*.
///
/// A new record and its area hold; a sibling's area that stood, rewritten or added to, and
/// a record that stood, rewritten, do not; nor does a file the suffix does not name. The
/// review of the task `B7`'s second part, its eighth finding, and these added after it: a
/// record deleted from the working tree that `HEAD` still holds, minted over, does not hold;
/// nor does a mint of two, a milestone's or a sub-task's, under the singular word — each
/// holds under [`byte_cell::EACH_MINTED`] — nor an area at an id no record gains.
#[test]
fn a_place_a_command_mints_is_one_nothing_stood_at() {
    const COMMITS: Line = line("N3", Allows::Commit(Holding::OnlyTheLines));
    const MINTS: Row = row(&[
        line(
            "N1",
            Allows::Files(Where::Under(".jigc/milestones/<minted>"), May::Rebuilt),
        ),
        line(
            "N2",
            Allows::Files(
                Where::Path("<docs-root>/milestone-records/<minted>.md"),
                May::WriteWhole,
            ),
        ),
        COMMITS,
    ]);
    const MINTS_EACH: Row = row(&[
        line(
            "N1",
            Allows::Files(Where::Under(".jigc/milestones/<each-minted>"), May::Rebuilt),
        ),
        line(
            "N2",
            Allows::Files(
                Where::Path("<docs-root>/milestone-records/<each-minted>.md"),
                May::WriteWhole,
            ),
        ),
        COMMITS,
    ]);
    let before = then(&sandbox(), |m| {
        dir(m, "repo/.jigc");
        dir(m, "repo/.jigc/milestones");
        dir(m, "repo/.jigc/milestones/m0");
        put(m, "repo/.jigc/milestones/m0/base.json", "a pin\n");
    });
    // What stands under `.jigc/` was the lead-ins': the product's own, never a loss.
    let base = then(&sandbox(), |m| dir(m, "repo/.jigc"));
    let came = |row: Row, before: &Manifest, after: &Manifest| {
        Cell {
            base: Some(&base),
            ..Cell::of(row, before, after)
        }
        .came_to()
    };
    let area = |m: &mut Manifest, id: &str| {
        dir(m, &format!("repo/.jigc/milestones/{id}"));
        put(
            m,
            &format!("repo/.jigc/milestones/{id}/base.json"),
            "a pin\n",
        );
    };

    let minted = then(&before, |m| {
        area(m, "m");
        records(m, "docs", &[("m", &[])]);
    });
    for row in [MINTS, MINTS_EACH] {
        let (class, why) = came(row, &before, &minted);
        assert_eq!(
            class,
            Classification::Holds,
            "a new record and its area: {why:#?}"
        );
    }
    for (what, after, says) in [
        (
            "the area that stood, rewritten",
            then(&minted, |m| {
                put(m, "repo/.jigc/milestones/m0/base.json", "another pin\n")
            }),
            "m0/",
        ),
        (
            "the area that stood, added to",
            then(&minted, |m| {
                put(m, "repo/.jigc/milestones/m0/intent", "an intent\n")
            }),
            "m0/",
        ),
        (
            "an area at an id no record gains",
            then(&minted, |m| area(m, "n")),
            "milestones/n/",
        ),
        (
            "a file the suffix does not name",
            then(&minted, |m| {
                put(m, "repo/docs/milestone-records/m.txt", "# m\n")
            }),
            "m.txt",
        ),
    ] {
        let (class, why) = came(MINTS, &before, &after);
        assert!(
            class == Classification::WrittenOutside && why.iter().any(|why| why.contains(says)),
            "{what}: {class}, {why:#?}"
        );
    }
    // A record that stood — in the working tree, or in `HEAD` alone, its file deleted —
    // rewritten and committed.
    let stood = then(&before, |m| records(m, "docs", &[("m", &[])]));
    let deleted = then(&stood, |m| {
        m.files.remove("repo/docs/milestone-records/m.md");
    });
    for (what, before) in [
        ("a record that stood, rewritten", &stood),
        (
            "a record deleted from the working tree that `HEAD` holds, minted over",
            &deleted,
        ),
    ] {
        let after = then(before, |m| {
            area(m, "m");
            records(m, "docs", &[("m", &["s1"])]);
        });
        for row in [MINTS, MINTS_EACH] {
            // The commit holds at the record what no line lets it hold.
            let (class, why) = came(row, before, &after);
            assert!(
                class == Classification::GitMoved && why.iter().any(|why| why.contains("m.md")),
                "{what}: {class}, {why:#?}"
            );
        }
    }
    // Two milestones minted by one command.
    let two = then(&before, |m| {
        area(m, "m");
        area(m, "n");
        records(m, "docs", &[("m", &[]), ("n", &[])]);
    });
    // The two records the commit holds, written where no line lets them be.
    let (class, why) = came(MINTS, &before, &two);
    assert!(
        class == Classification::GitMoved
            && why.iter().any(|why| why.contains("records/m.md"))
            && why.iter().any(|why| why.contains("records/n.md")),
        "a mint of two milestones, under the singular word: {class}, {why:#?}"
    );
    let (class, why) = came(MINTS_EACH, &before, &two);
    assert_eq!(
        class,
        Classification::Holds,
        "a mint of two milestones, under the word for each: {why:#?}"
    );

    // A sub-task's mint, by a door whose unit is the milestone: the item its record gains.
    const ADDS: Row = Row {
        unit: Unit::TheWordAfterTheLeaf,
        ..row(&[
            line(
                "A1",
                Allows::Files(Where::Under(".jigc/tasks/<minted>"), May::Rebuilt),
            ),
            line(
                "A2",
                Allows::Files(
                    Where::Path("<docs-root>/milestone-records/<unit>.md"),
                    May::WriteWhole,
                ),
            ),
            COMMITS,
        ])
    };
    const ADDS_EACH: Row = Row {
        unit: Unit::TheWordAfterTheLeaf,
        ..row(&[
            line(
                "A1",
                Allows::Files(Where::Under(".jigc/tasks/<each-minted>"), May::Rebuilt),
            ),
            line(
                "A2",
                Allows::Files(
                    Where::Path("<docs-root>/milestone-records/<unit>.md"),
                    May::WriteWhole,
                ),
            ),
            COMMITS,
        ])
    };
    let task = |m: &mut Manifest, id: &str| {
        dir(m, "repo/.jigc/tasks");
        dir(m, &format!("repo/.jigc/tasks/{id}"));
        put(m, &format!("repo/.jigc/tasks/{id}/base.json"), "a pin\n");
    };
    let before = then(&sandbox(), |m| {
        dir(m, "repo/.jigc");
        task(m, "s1");
        records(m, "docs", &[("m", &["s1"])]);
    });
    let came = |row: Row, after: &Manifest| {
        Cell {
            argv: &["probe", "m"],
            base: Some(&base),
            ..Cell::of(row, &before, after)
        }
        .came_to()
    };
    let added = then(&before, |m| {
        task(m, "s2");
        records(m, "docs", &[("m", &["s1", "s2"])]);
    });
    for row in [ADDS, ADDS_EACH] {
        let (class, why) = came(row, &added);
        assert_eq!(
            class,
            Classification::Holds,
            "a sub-task the record gains: {why:#?}"
        );
    }
    let (class, why) = came(
        ADDS,
        &then(&before, |m| {
            task(m, "s2");
            task(m, "s3");
            records(m, "docs", &[("m", &["s1", "s2", "s3"])]);
        }),
    );
    assert!(
        class == Classification::WrittenOutside
            && why.iter().any(|why| why.contains("tasks/s2/"))
            && why.iter().any(|why| why.contains("tasks/s3/")),
        "a mint of two sub-tasks, under the singular word: {class}, {why:#?}"
    );
    let (class, why) = came(
        ADDS_EACH,
        &then(&before, |m| {
            task(m, "s2");
            task(m, "s3");
            records(m, "docs", &[("m", &["s1", "s2", "s3"])]);
        }),
    );
    assert_eq!(
        class,
        Classification::Holds,
        "a mint of two sub-tasks, under the word for each: {why:#?}"
    );
    for (what, after) in [
        (
            "an area beside the one the record gains",
            then(&added, |m| task(m, "s3")),
        ),
        (
            "an area the record does not gain",
            then(&before, |m| task(m, "s3")),
        ),
    ] {
        for row in [ADDS, ADDS_EACH] {
            let (class, why) = came(row, &after);
            assert!(
                class == Classification::WrittenOutside
                    && why.iter().any(|why| why.contains("tasks/s3/")),
                "{what}: {class}, {why:#?}"
            );
        }
    }
}

/// **The re-seed rebuilds a sub-task's area only where none stood, and `provision` a
/// worktree only where no checkout stood** — the review of the task `B7`'s second part, its
/// second finding: the re-seed's lines and `MP1` let any rewrite happen under a sub-task's
/// area or worktree, where their own sentences say *"A sub-task whose area already exists is
/// skipped untouched, so a live session is never clobbered — **and so is one the record has
/// settled**"* (`design/team-ready-state.md` → *Engine capability 2 (read-back)*) and *"a
/// **registered** worktree is reused untouched and never probed at all"* (→ *`jigc
/// milestone discard <id>` — the record-settling op*).
///
/// At every door of the seven that passes through the re-seed — `finalize` and `discard`
/// among them, whose rows lacked the re-seed's lines — the area of a sub-task the record
/// names, live, where none stood, holds; a standing sub-task's staged prose or base pin
/// rewritten, or a file added to its area, is *written outside its allowance*, and so is
/// an area made for a sub-task the record has settled. At `provision`, a worktree made
/// where nothing stood, or into a leftover directory no checkout is (*"it clears a
/// leftover before `git worktree add`"*), holds; a file of a live checkout's rewritten does
/// not.
///
/// **And three more, by the seventh fix after that review.** A door that refuses at its own
/// guard after the re-seed leaves the area it made where none stood — *"all pass through
/// the reseed before their own guards"* — and that refusal changed nothing it may not; a
/// standing area added to on a refusal did. A worktree `provision` makes holds the pinned
/// base's tree and its own `.git` file and nothing else — *"populates from the pinned
/// base's *tree*"* — so a file the base does not hold there, or other bytes than the
/// base's, is written outside the allowance. And a worktree for a sub-task the record has
/// settled is no place of the line — the stricter reading of the record's §6, entry 20.
#[test]
fn a_re_seed_rebuilds_an_area_only_where_none_stood() {
    const DOORS: [Row; 7] = [
        milestone::ADD_TASK,
        milestone::ADD_FROM_SPEC,
        milestone::PROVISION,
        milestone::EXECUTE,
        milestone::JOIN,
        milestone::DISCARD,
        milestone::FINALIZE,
    ];
    const S1: &str = "repo/.jigc/tasks/s1";
    let before = then(&sandbox(), |m| {
        dir(m, "repo/.jigc");
        dir(m, "repo/.jigc/tasks");
        dir(m, S1);
        dir(m, &format!("{S1}/docs"));
        put(m, &format!("{S1}/base.json"), "a pin\n");
        put(
            m,
            &format!("{S1}/docs/commit:s1.md"),
            "# s1\n\nthe prose s1 handed to jigc\n",
        );
        settled_records(
            m,
            "docs",
            "m",
            &[("s1", "active"), ("s2", "active"), ("s3", "discarded")],
        );
    });
    let an_area = |id: &str| {
        then(&before, |m| {
            let area = format!("repo/.jigc/tasks/{id}");
            dir(m, &area);
            put(m, &format!("{area}/base.json"), "a pin\n");
            put(m, &format!("{area}/intent"), "an intent\n");
        })
    };
    for row in DOORS {
        let leaf = row.leaf.join(" ");
        let argv = [row.leaf[0], row.leaf[1], "m"];
        let came = |after: &Manifest| {
            Cell {
                argv: &argv,
                ..Cell::of(row, &before, after)
            }
            .came_to()
        };
        let (class, why) = came(&an_area("s2"));
        assert_eq!(
            class,
            Classification::Holds,
            "`jigc {leaf}`, a live sub-task's area where none stood: {why:#?}"
        );
        // A rewrite takes bytes no object holds, so no line covering it is a loss; an
        // addition is a write no line lets happen.
        for (what, after, says, expected) in [
            (
                "a standing sub-task's staged prose rewritten",
                then(&before, |m| {
                    put(
                        m,
                        &format!("{S1}/docs/commit:s1.md"),
                        "# s1\n\nother prose\n",
                    )
                }),
                "commit:s1.md",
                Classification::Lost,
            ),
            (
                "a standing sub-task's base pin rewritten",
                then(&before, |m| {
                    put(m, &format!("{S1}/base.json"), "another pin\n")
                }),
                "s1/base.json",
                Classification::Lost,
            ),
            (
                "a standing sub-task's area added to",
                then(&before, |m| put(m, &format!("{S1}/intent"), "an intent\n")),
                "s1/intent",
                Classification::WrittenOutside,
            ),
            (
                "an area made for a sub-task the record has settled",
                an_area("s3"),
                "s3/",
                Classification::WrittenOutside,
            ),
        ] {
            let (class, why) = came(&after);
            assert!(
                class == expected && why.iter().any(|why| why.contains(says)),
                "`jigc {leaf}`, {what}: {class}, {why:#?}"
            );
        }
        // Refused at the door's own guard, after the re-seed.
        let refused = |after: &Manifest| {
            Cell {
                argv: &argv,
                code: Some(3),
                listed: Listed::Refuses,
                ..Cell::of(row, &before, after)
            }
            .came_to()
        };
        let (class, why) = refused(&an_area("s2"));
        assert_eq!(
            class,
            Classification::RefusesNothingChanged,
            "`jigc {leaf}` refused, a live sub-task's area where none stood: {why:#?}"
        );
        let (class, why) = refused(&then(&before, |m| {
            put(m, &format!("{S1}/intent"), "an intent\n")
        }));
        assert!(
            class == Classification::RefusedAndChanged
                && why.iter().any(|why| why.contains("s1/intent")),
            "`jigc {leaf}` refused, a standing sub-task's area added to: {class}, {why:#?}"
        );
    }

    let argv: &[&str] = &["milestone", "provision", "m"];
    let came = |before: &Manifest, after: &Manifest| {
        Cell {
            argv,
            ..Cell::of(milestone::PROVISION, before, after)
        }
        .came_to()
    };
    const S2: &str = "repo/.jigc/worktrees/s2";
    let provisioned = then(&before, |m| worktree_added(m, S2, "s2"));
    let (class, why) = came(&before, &provisioned);
    assert_eq!(
        class,
        Classification::Holds,
        "a worktree where nothing stood: {why:#?}"
    );
    let a_leftover = then(&before, |m| {
        dir(m, "repo/.jigc/worktrees");
        dir(m, S2);
    });
    let (class, why) = came(&a_leftover, &provisioned);
    assert_eq!(
        class,
        Classification::Holds,
        "a worktree into an empty leftover no checkout is: {why:#?}"
    );
    let rewritten = then(&provisioned, |m| {
        put(m, &format!("{S2}/README.md"), "another readme\n")
    });
    let added = then(&provisioned, |m| {
        put(m, &format!("{S2}/more.txt"), "more\n")
    });
    for (what, after, says) in [
        (
            "a live checkout's file rewritten",
            &rewritten,
            "s2/README.md",
        ),
        ("a live checkout added to", &added, "more.txt"),
    ] {
        let (class, why) = came(&provisioned, after);
        assert!(
            class == Classification::WrittenOutside && why.iter().any(|why| why.contains(says)),
            "{what}: {class}, {why:#?}"
        );
    }

    // What the worktree holds: the pinned base's tree, and its own `.git` file.
    let (class, why) = came(
        &before,
        &then(&provisioned, |m| {
            put(m, &format!("{S2}/.git"), "gitdir: the admin directory\n")
        }),
    );
    assert_eq!(
        class,
        Classification::Holds,
        "a worktree with its own `.git` file: {why:#?}"
    );
    for (what, after, says) in [
        (
            "a file the pinned base's tree does not hold",
            then(&provisioned, |m| {
                put(m, &format!("{S2}/extra.txt"), "not the base's\n")
            }),
            "s2/extra.txt",
        ),
        (
            "a file of the base's, with other bytes",
            then(&provisioned, |m| {
                put(m, &format!("{S2}/{README}"), "not the base's readme\n")
            }),
            "s2/README.md",
        ),
    ] {
        let (class, why) = came(&before, &after);
        assert!(
            class == Classification::WrittenOutside && why.iter().any(|why| why.contains(says)),
            "a new worktree, {what}: {class}, {why:#?}"
        );
    }
    let (class, why) = came(
        &before,
        &then(&before, |m| {
            worktree_added(m, "repo/.jigc/worktrees/s3", "s3")
        }),
    );
    assert!(
        class == Classification::GitMoved && why.iter().any(|why| why.contains("`s3`")),
        "a worktree for a sub-task the record has settled: {class}, {why:#?}"
    );
}

/// **The rendered message's temporary file stands under no line** ([`byte_cell::DELETED_AFTER`])
/// — *"`git commit -F <rendered-message-file>` (a temp file, deleted after)"*
/// (`design/finalize.md` → *6. Commit*), and the human's ruling of 2026-10-14 on the
/// record's §6, entry 6: deleted whether or not the commit landed. A line that lets a
/// milestone's or a task's area be rebuilt covers a cache made there and not the message
/// file, made or rewritten, at exit 0 or on a refusal; the file gone is as the place's line
/// has it; and a file of that name outside `.jigc/` is no file of jigc's. The review of the
/// task `B7`'s second part, its tenth finding: the record doors' areas are lines whole, so
/// the file a landed record commit left stood unseen at exit 0.
#[test]
fn the_rendered_messages_file_stands_under_no_line() {
    const AREAS: Row = Row {
        unit: Unit::TheWordAfterTheLeaf,
        ..row(&[
            line(
                "A1",
                Allows::Files(Where::Under(".jigc/milestones/<unit>"), May::Rebuilt),
            ),
            line(
                "A2",
                Allows::Files(Where::Under(".jigc/tasks/<unit>"), May::Removed),
            ),
            line("A3", Allows::Files(Where::Under("docs"), May::WriteWhole)),
            Line {
                when: When::Refused,
                on_refusal: true,
                ..line(
                    "A4",
                    Allows::Files(Where::Under(".jigc/milestones/<unit>"), May::Rebuilt),
                )
            },
        ])
    };
    const AREA: &str = "repo/.jigc/milestones/m";
    const MESSAGE: &str = "repo/.jigc/milestones/m/record-commit-msg.txt";
    let before = then(&sandbox(), |m| {
        dir(m, "repo/.jigc");
        dir(m, "repo/.jigc/milestones");
        dir(m, AREA);
        dir(m, "repo/.jigc/tasks");
        dir(m, "repo/.jigc/tasks/m");
        dir(m, "repo/docs");
    });
    // The state before the lead-ins is `before`: a message file that stands before the
    // command is a lead-in's, the product's own.
    let came = |at: &Manifest, after: &Manifest, code: i32| {
        Cell {
            base: Some(&before),
            argv: &["probe", "m"],
            code: Some(code),
            listed: if code == 0 {
                Listed::Acts
            } else {
                Listed::Refuses
            },
            ..Cell::of(AREAS, at, after)
        }
        .came_to()
    };
    let (class, why) = came(
        &before,
        &then(&before, |m| put(m, &format!("{AREA}/tasks.json"), "[]\n")),
        0,
    );
    assert_eq!(class, Classification::Holds, "a cache made: {why:#?}");
    let (class, why) = came(
        &before,
        &then(&before, |m| {
            put(m, "repo/docs/record-commit-msg.txt", "a doc of that name\n")
        }),
        0,
    );
    assert_eq!(
        class,
        Classification::Holds,
        "a file of that name outside `.jigc/`: {why:#?}"
    );
    let left = then(&before, |m| put(m, MESSAGE, "chore: a message\n"));
    const TASKS_MESSAGE: &str = "repo/.jigc/tasks/m/finalize-message.tmp";
    let (class, why) = came(
        &then(&before, |m| put(m, TASKS_MESSAGE, "a message\n")),
        &before,
        0,
    );
    assert_eq!(
        class,
        Classification::Holds,
        "the finalize's message file gone under a line that removes its area: {why:#?}"
    );
    for (what, at, after, code, expected) in [
        (
            "made at exit 0",
            &before,
            &left,
            0,
            Classification::WrittenOutside,
        ),
        (
            "rewritten at exit 0",
            &left,
            &then(&left, |m| put(m, MESSAGE, "chore: another message\n")),
            0,
            Classification::WrittenOutside,
        ),
        (
            "made on a refusal",
            &before,
            &left,
            1,
            Classification::RefusedAndChanged,
        ),
    ] {
        let (class, why) = came(at, after, code);
        assert!(
            class == expected && why.iter().any(|why| why.contains("record-commit-msg.txt")),
            "the record's message file {what}: {class}, {why:#?}"
        );
    }
    let (class, why) = came(
        &before,
        &then(&before, |m| {
            put(m, &format!("{AREA}/finalize-message.tmp"), "a message\n")
        }),
        0,
    );
    assert!(
        class == Classification::WrittenOutside
            && why.iter().any(|why| why.contains("finalize-message.tmp")),
        "the finalize's message file made in a milestone's area: {class}, {why:#?}"
    );
}

/// **A line that merges into a file lets each commit the command made on the way hold the
/// file partway** — *"It mints one sub-task per spec criterion and commits the record once
/// per sub-task"* (`design/team-ready-state.md` → *The commit model — path-scoped commit at
/// each milestone op*): a commit a newer one of the command's changes the same path in
/// again holds there what keeps every line the file held before the command and has no
/// line the file does not hold after it ([`Manifest::blobs`]); one that drops a line of
/// before, or holds a line the file does not end with, is git's state moved, and so is the
/// newest commit holding the file partway. The review of the task `B7`'s second part, its
/// tenth finding: a spec of more than one criterion was a frame bound.
#[test]
fn a_merge_committed_step_by_step_holds_each_step() {
    const MERGES: Row = row(&[
        line(
            "G1",
            Allows::Files(Where::Path("docs/r.md"), May::MergeInto),
        ),
        line("G2", Allows::Commit(Holding::OnlyTheLines)),
    ]);
    const RECORD: &str = "docs/r.md";
    let at = format!("{REPO}/{RECORD}");
    let before = then(&sandbox(), |m| {
        dir(m, "repo/docs");
        put(m, &at, "a\n");
        commit(m, "c1", &[(RECORD, Some((0o100_644, "a\n")))]);
    });
    let stepped = |first: &str, second: Option<&str>| {
        then(&before, |m| {
            put(m, &at, "a\nb\nc\n");
            commit(m, "k1", &[(RECORD, Some((0o100_644, first)))]);
            if let Some(second) = second {
                commit(m, "k2", &[(RECORD, Some((0o100_644, second)))]);
            }
        })
    };
    let came = |after: &Manifest| Cell::of(MERGES, &before, after).came_to();
    let (class, why) = came(&stepped("a\nb\n", Some("a\nb\nc\n")));
    assert_eq!(
        class,
        Classification::Holds,
        "two commits, the first partway: {why:#?}"
    );
    for (what, after) in [
        (
            "the first drops a line of before",
            stepped("b\n", Some("a\nb\nc\n")),
        ),
        (
            "the first holds a line the file does not end with",
            stepped("a\nx\n", Some("a\nb\nc\n")),
        ),
        (
            "the only commit holds the file partway",
            stepped("a\nb\n", None),
        ),
    ] {
        let (class, why) = came(&after);
        assert!(
            class == Classification::GitMoved && why.iter().any(|why| why.contains(RECORD)),
            "{what}: {class}, {why:#?}"
        );
    }
}

/// A milestone's record, committed over the `HEAD` that stands: under `root`, as
/// [`records`] has it, each item with the `status` leaf the record carries.
fn settled_records(manifest: &mut Manifest, root: &str, id: &str, items: &[(&str, &str)]) {
    let home = match root {
        "" => "milestone-records".to_string(),
        root => format!("{root}/milestone-records"),
    };
    let mut at = REPO.to_string();
    for component in home.split('/') {
        at = format!("{at}/{component}");
        dir(manifest, &at);
    }
    let mut bytes = format!("{}# {id}\n\n## Tasks\n", a_records_head(manifest));
    for (task, status) in items {
        bytes.push_str(&format!(
            "\n### {task}  {{#{task}}}\n\n<!-- fields -->\n- intent: an intent\n- status: {status}\n"
        ));
    }
    let path = format!("{home}/{id}.md");
    put(manifest, &format!("{REPO}/{path}"), &bytes);
    let commit_id = format!("records-{}", manifest.commits.len());
    commit(
        manifest,
        &commit_id,
        &[(path.as_str(), Some((0o100_644, bytes.as_str())))],
    );
}

/// `manifest` with a linked worktree added at `path`, from the sandbox's root, registered
/// as `name`, as `git worktree add --detach` leaves one at the base [`records`] pins: its
/// directory with the base's one file, its `HEAD` detached at the main checkout's first
/// commit, its index that commit's tree.
fn worktree_added(manifest: &mut Manifest, path: &str, name: &str) {
    let mut at = String::new();
    for part in path.split('/') {
        at = if at.is_empty() {
            part.to_string()
        } else {
            format!("{at}/{part}")
        };
        if at != REPO {
            dir(manifest, &at);
        }
    }
    put(manifest, &format!("{path}/{README}"), "readme\n");
    manifest
        .registrations
        .insert(name.to_string(), path.to_string());
    manifest.checkouts.insert(
        path.to_string(),
        Checkout {
            head: "c0".to_string(),
            branch: None,
            index: [(README.to_string(), staged("readme\n"))].into(),
        },
    );
}

/// **A registration a command adds is at the place its line names** — `jigc milestone
/// provision`'s *"`git worktree add --detach` populates from the pinned base's *tree*"*
/// (`design/team-ready-state.md` → *What a fan-out worktree isolates*), the task `B7`'s
/// second part. A worktree registered, its checkout with it, at `.jigc/worktrees/<sub-task>`
/// — a sub-task the milestone's committed record names — holds under a line of
/// [`Allows::Registered`]; with no such line it is git's state moved, and so is one for a
/// task the record does not name, and one made outside the workbench. **And it is as `git
/// worktree add --detach` leaves one at the pinned base** — its `HEAD` detached at the base
/// the milestone's committed record pins, its index that commit's tree: the review of the
/// task `B7`'s second part, its ninth finding, found the line excused a new checkout by its
/// path alone, so one at another commit, on a branch, or with something staged held.
#[test]
fn a_registration_a_command_adds_is_at_the_place_its_line_names() {
    const WORKTREES: Line = line(
        "P1",
        Allows::Files(Where::Under(".jigc/worktrees/<sub-task>"), May::Rebuilt),
    );
    const PROVISIONS: Row = Row {
        unit: Unit::TheWordAfterTheLeaf,
        ..row(&[
            WORKTREES,
            line("P2", Allows::Registered(".jigc/worktrees/<sub-task>")),
        ])
    };
    const FILES_ALONE: Row = Row {
        unit: Unit::TheWordAfterTheLeaf,
        ..row(&[WORKTREES])
    };
    let before = then(&sandbox(), |m| {
        dir(m, "repo/.jigc");
        records(m, "docs", &[("m", &["s1"])]);
    });
    let came_to = |row: Row, after: &Manifest| {
        Cell {
            argv: &["probe", "m"],
            ..Cell::of(row, &before, after)
        }
        .came_to()
    };
    let came = |row: Row, path: &str, name: &str| {
        came_to(row, &then(&before, |m| worktree_added(m, path, name)))
    };
    let (class, why) = came(PROVISIONS, "repo/.jigc/worktrees/s1", "s1");
    assert_eq!(
        class,
        Classification::Holds,
        "its sub-task's worktree: {why:#?}"
    );
    for (what, row, path, name) in [
        (
            "with no line that lets it",
            FILES_ALONE,
            "repo/.jigc/worktrees/s1",
            "s1",
        ),
        (
            "for a task the record does not name",
            PROVISIONS,
            "repo/.jigc/worktrees/s9",
            "s9",
        ),
        ("outside the workbench", PROVISIONS, "outside/own", "own"),
    ] {
        let (class, why) = came(row, path, name);
        assert!(
            class == Classification::GitMoved
                && why.iter().any(|why| why.contains(&format!("`{name}`"))),
            "{what}: {class}, {why:#?}"
        );
    }

    // At the place its line names, and not as the add leaves one at the pinned base.
    const S1: &str = "repo/.jigc/worktrees/s1";
    let provisioned = then(&before, |m| worktree_added(m, S1, "s1"));
    let main_head = before.checkouts[REPO].head.clone();
    for (what, after) in [
        (
            "its `HEAD` at another commit than the pinned base",
            then(&provisioned, |m| {
                m.checkouts.get_mut(S1).expect("the worktree").head = main_head.clone()
            }),
        ),
        (
            "on a branch",
            then(&provisioned, |m| {
                m.checkouts.get_mut(S1).expect("the worktree").branch = Some("s1".to_string())
            }),
        ),
        (
            "with something staged in its index",
            then(&provisioned, |m| {
                put(m, &format!("{S1}/work.txt"), "a sub-task's work\n");
                m.checkouts
                    .get_mut(S1)
                    .expect("the worktree")
                    .index
                    .insert("work.txt".to_string(), staged("a sub-task's work\n"));
                m.held.insert(blob_of("a sub-task's work\n"));
            }),
        ),
    ] {
        let (class, why) = came_to(PROVISIONS, &after);
        assert!(
            class == Classification::GitMoved
                && why.iter().any(|why| why.contains("`s1`"))
                && why.iter().any(|why| why.contains("pinned base")),
            "{what}: {class}, {why:#?}"
        );
    }
}

/// **A commit carries the config layer as it stands, on the layer's first commit and on no
/// other** — [`May::Carried`] under [`When::FirstCommitOfItsPlace`]: *"on a first commit,
/// the project config layer (`.jigc/config`, `.jigc/.gitignore`)"* (`design/finalize.md` →
/// *5. Stage*), the milestone boundary's *"overlays the promoted docs + first-commit
/// config"* (→ *`fan-out` finalize*). Where `HEAD` holds nothing under the place, a
/// `.jigc/config` file in the working tree that a commit the command made holds, byte for
/// byte, holds; a commit that holds other bytes there is git's state moved, the command's
/// own rewrite of the place included; and the place rewritten while the commit holds what
/// stood is written outside the allowance — the word lets the commit carry what stood
/// before the command, and the command write nothing there. **Where `HEAD` holds the
/// layer** — every prepared state, since `setup`'s install commit lands it — the line is
/// out of force: a pending delta, or a file of the user's nobody staged under the place,
/// that a finalize commits is git's state moved, at both rows that quote the sentence, the
/// same way. The task `B7`'s second part; narrowed after its review (the first finding),
/// where the word held under any `HEAD` and a user's untracked file swept into a landed
/// boundary held.
#[test]
fn a_commit_carries_a_config_delta_as_it_stands() {
    const CARRIES: Row = row(&[
        Line {
            when: When::FirstCommitOfItsPlace,
            ..line(
                "K1",
                Allows::Files(Where::Under(".jigc/config"), May::Carried),
            )
        },
        line("K2", Allows::Commit(Holding::OnlyTheLines)),
    ]);
    const KNOBS: &str = ".jigc/config/manifest.yaml";
    let at = format!("{REPO}/{KNOBS}");
    let before = then(&sandbox(), |m| {
        dir(m, "repo/.jigc");
        dir(m, "repo/.jigc/config");
        put(m, &at, "scalar: {a: 1}\n");
    });
    let carried = then(&before, |m| {
        commit(m, "c1", &[(KNOBS, Some((0o100_644, "scalar: {a: 1}\n")))]);
    });
    comes_out(&Cell::of(CARRIES, &before, &carried), Classification::Holds);
    let other = then(&before, |m| {
        commit(m, "c1", &[(KNOBS, Some((0o100_644, "scalar: {a: 2}\n")))]);
    });
    let (class, why) = Cell::of(CARRIES, &before, &other).came_to();
    assert!(
        class == Classification::GitMoved && why.iter().any(|why| why.contains(KNOBS)),
        "other bytes than stand there: {class}, {why:#?}"
    );
    for (what, written, expected) in [
        (
            "the place rewritten and committed so",
            "scalar: {a: 2}\n",
            Classification::GitMoved,
        ),
        (
            "the place rewritten, and what stood committed",
            "scalar: {a: 1}\n",
            Classification::WrittenOutside,
        ),
    ] {
        let after = then(&before, |m| {
            put(m, &at, "scalar: {a: 2}\n");
            commit(m, "c1", &[(KNOBS, Some((0o100_644, written)))]);
        });
        let (class, why) = Cell::of(CARRIES, &before, &after).came_to();
        assert!(
            class == expected && why.iter().any(|why| why.contains(KNOBS)),
            "{what}: {class}, {why:#?}"
        );
    }
    // The same delta where `HEAD` holds the layer: the line is out of force.
    let layer = then(&sandbox(), |m| {
        dir(m, "repo/.jigc");
        dir(m, "repo/.jigc/config");
        put(m, &at, "scalar: {}\n");
        commit(m, "c1", &[(KNOBS, Some((0o100_644, "scalar: {}\n")))]);
    });
    let pending = then(&layer, |m| put(m, &at, "scalar: {a: 1}\n"));
    let swept = then(&pending, |m| {
        commit(m, "c2", &[(KNOBS, Some((0o100_644, "scalar: {a: 1}\n")))]);
    });
    let (class, why) = Cell::of(CARRIES, &pending, &swept).came_to();
    assert!(
        class == Classification::GitMoved && why.iter().any(|why| why.contains(KNOBS)),
        "a pending delta where `HEAD` holds the layer: {class}, {why:#?}"
    );
    the_config_layer_committed_is_carried_by_no_line_of_a_finalize();
}

/// The two rows that quote *"on a first commit, the project config layer"* read it one way:
/// the layer's first commit holds at each; where `HEAD` holds the layer, a pending delta, or
/// a file of the user's nobody staged under the place, that a finalize commits is git's
/// state moved at each — the exclusion.
fn the_config_layer_committed_is_carried_by_no_line_of_a_finalize() {
    const KNOBS: &str = ".jigc/config/manifest.yaml";
    const NOTES: &str = ".jigc/config/notes-of-the-user.txt";
    const FINALIZES: [(Row, &[&str]); 2] = [
        (task::FINALIZE, &["task", "finalize", "t1"]),
        (milestone::FINALIZE, &["milestone", "finalize", "m"]),
    ];
    let first = then(&sandbox(), |m| {
        dir(m, "repo/.jigc");
        dir(m, "repo/.jigc/config");
        put(m, &format!("{REPO}/{KNOBS}"), "scalar: {}\n");
    });
    let landed = then(&first, |m| {
        commit(m, "c1", &[(KNOBS, Some((0o100_644, "scalar: {}\n")))]);
    });
    for (row, argv) in FINALIZES {
        let (class, why) = Cell {
            argv,
            code: Some(0),
            ..Cell::of(row, &first, &landed)
        }
        .came_to();
        assert!(
            class == Classification::Holds,
            "`jigc {}`, the layer on its first commit: {class}, {why:#?}",
            row.leaf.join(" ")
        );
    }
    let committed = then(&sandbox(), |m| {
        dir(m, "repo/.jigc");
        dir(m, "repo/.jigc/config");
        put(m, &format!("{REPO}/{KNOBS}"), "scalar: {}\n");
        commit(m, "c1", &[(KNOBS, Some((0o100_644, "scalar: {}\n")))]);
    });
    for (what, path, bytes) in [
        ("a pending delta", KNOBS, "scalar: {a: 1}\n"),
        (
            "a file of the user's nobody staged",
            NOTES,
            "a note of the user\n",
        ),
    ] {
        let before = then(&committed, |m| put(m, &format!("{REPO}/{path}"), bytes));
        let after = then(&before, |m| {
            commit(m, "c2", &[(path, Some((0o100_644, bytes)))]);
        });
        for (row, argv) in FINALIZES {
            let (class, why) = Cell {
                argv,
                code: Some(0),
                ..Cell::of(row, &before, &after)
            }
            .came_to();
            assert!(
                class == Classification::GitMoved && why.iter().any(|why| why.contains(path)),
                "`jigc {}`, {what} committed where `HEAD` holds the layer: {class}, {why:#?}",
                row.leaf.join(" ")
            );
        }
    }
}

/// **Every way a row's spellings do not stand on its lines**: a line in force under a
/// spelling the row does not have; a spelling that replaces a line the row does not have
/// as a line of every spelling; a spelling with no line of its own that takes none out of
/// force; two spellings of one name.
fn unspelled(rows: &[&Row]) -> Vec<String> {
    let mut out = Vec::new();
    for row in rows {
        let leaf = row.leaf.join(" ");
        for line in row.lines {
            if let Some(name) = line.spelling
                && !row.spellings.iter().any(|spelling| spelling.name == name)
            {
                out.push(format!(
                    "the line `{}` of `jigc {leaf}` is in force under the spelling `{name}`, \
                     which its row does not have",
                    line.id
                ));
            }
        }
        for (at, spelling) in row.spellings.iter().enumerate() {
            let name = spelling.name;
            for id in spelling.replaces {
                if !row
                    .lines
                    .iter()
                    .any(|line| line.id == *id && line.spelling.is_none())
                {
                    out.push(format!(
                        "the spelling `{name}` of `jigc {leaf}` replaces `{id}`, which is no line \
                         of every spelling of its row"
                    ));
                }
            }
            // A spelling is a line of its own, or a line it takes out of force: one that only
            // takes a line away is a flag whose doc promises fewer writes.
            let takes_away = spelling.replaces.iter().any(|id| {
                row.lines
                    .iter()
                    .any(|line| line.id == *id && line.spelling.is_none())
            });
            if !takes_away && !row.lines.iter().any(|line| line.spelling == Some(name)) {
                out.push(format!(
                    "the spelling `{name}` of `jigc {leaf}` has no line of its own, and takes \
                     none out of force"
                ));
            }
            if row.spellings[..at].iter().any(|other| other.name == name) {
                out.push(format!(
                    "the spelling `{name}` of `jigc {leaf}` is named twice"
                ));
            }
        }
    }
    out
}

/// **A cell is compared under the spelling of its leaf it names** — the human's ruling of
/// 2026-10-18 on the two spellings: a command its doc has commit otherwise in a state no
/// repository's bytes show — the doc-only arm of `task finalize`, `squash: false` at
/// `milestone finalize` — is a spelling a cell names, and its row's lines as the spelling
/// has them are what it is held to: every line of no spelling but those the spelling
/// replaces, and the lines that name it.
///
/// A commit that sweeps a change the user staged elsewhere beside the doc it lands holds
/// under the plain spelling, whose commit is the index whole, and is *git's state moved
/// outside its allowance* under a spelling whose commit holds only its lines' paths; the
/// commit of the doc alone holds under both. A spelling's own line is in force under it
/// alone: the same commit under the plain spelling of a row whose only commit line is the
/// spelling's is no line's.
#[test]
fn a_cell_is_compared_under_the_spelling_it_names() {
    const DOC: &str = "docs/use-tabs.md";
    const SPELLED: Row = Row {
        spellings: &[byte_cell::Spelling {
            name: "path-scoped",
            replaces: &["P1"],
        }],
        ..row(&[
            line("P1", Allows::Commit(Holding::TheIndex)),
            line("P2", Allows::Files(Where::Path(DOC), May::WriteWhole)),
            Line {
                spelling: Some("path-scoped"),
                ..line("P3", Allows::Commit(Holding::OnlyTheLines))
            },
        ])
    };
    const PROSE: &str = "# Use tabs\n\nthe prose\n";
    let before = then(&sandbox(), |m| {
        dir(m, "repo/docs");
        put(m, "repo/staged.txt", "the user's own\n");
        m.checkouts
            .get_mut(REPO)
            .expect("the main checkout")
            .index
            .insert("staged.txt".to_string(), staged("the user's own\n"));
    });
    let swept = then(&before, |m| {
        put(m, &format!("repo/{DOC}"), PROSE);
        commit(
            m,
            "c1",
            &[
                (DOC, Some((0o100_644, PROSE))),
                ("staged.txt", Some((0o100_644, "the user's own\n"))),
            ],
        );
    });
    let alone = then(&before, |m| {
        put(m, &format!("repo/{DOC}"), PROSE);
        commit(m, "c1", &[(DOC, Some((0o100_644, PROSE)))]);
    });
    let under = |spelling: Option<&'static str>, after: &Manifest| {
        Cell {
            spelling,
            ..Cell::of(SPELLED, &before, after)
        }
        .came_to()
    };
    let (class, why) = under(None, &swept);
    assert_eq!(class, Classification::Holds, "the plain spelling: {why:#?}");
    let (class, why) = under(Some("path-scoped"), &swept);
    assert!(
        class == Classification::GitMoved && why.iter().any(|why| why.contains("staged.txt")),
        "a path-scoped commit that swept the index: {class}, {why:#?}"
    );
    for spelling in [None, Some("path-scoped")] {
        let (class, why) = under(spelling, &alone);
        assert_eq!(class, Classification::Holds, "{spelling:?}: {why:#?}");
    }
    const ITS_OWN_ONLY: Row = Row {
        spellings: &[byte_cell::Spelling {
            name: "path-scoped",
            replaces: &[],
        }],
        ..row(&[
            line("P2", Allows::Files(Where::Path(DOC), May::WriteWhole)),
            Line {
                spelling: Some("path-scoped"),
                ..line("P3", Allows::Commit(Holding::OnlyTheLines))
            },
        ])
    };
    let (class, why) = Cell::of(ITS_OWN_ONLY, &before, &alone).came_to();
    assert_eq!(
        class,
        Classification::GitMoved,
        "a spelling's line under the plain spelling: {why:#?}"
    );

    // The rows' spellings stand on their lines: on a fixture, fault by fault; then this
    // tree's rows.
    says(&unspelled(&[&SPELLED]), &[]);
    const ASTRAY: Row = Row {
        spellings: &[
            byte_cell::Spelling {
                name: "path-scoped",
                replaces: &["P9", "P3"],
            },
            byte_cell::Spelling {
                name: "path-scoped",
                replaces: &[],
            },
        ],
        ..row(&[Line {
            spelling: Some("elsewhere"),
            ..line("P3", Allows::Commit(Holding::OnlyTheLines))
        }])
    };
    says(
        &unspelled(&[&ASTRAY]),
        &[
            (
                "the line `P3`",
                "the spelling `elsewhere`, which its row does not have",
            ),
            ("replaces `P9`", "no line of every spelling"),
            ("replaces `P3`", "no line of every spelling"),
            ("the spelling `path-scoped`", "has no line of its own"),
            ("the spelling `path-scoped`", "has no line of its own"),
            ("the spelling `path-scoped`", "is named twice"),
        ],
    );
    // A spelling that only takes a line out of force — a flag whose doc promises fewer
    // writes, as `migrate-corpus --no-commit` was until the human's ruling on its question
    // gave it a staging line of its own — stands on the line it replaces.
    const TAKES_ONE_AWAY: Row = Row {
        spellings: &[byte_cell::Spelling {
            name: "no-commit",
            replaces: &["P3"],
        }],
        ..row(&[line("P3", Allows::Commit(Holding::OnlyTheLines))])
    };
    says(&unspelled(&[&TAKES_ONE_AWAY]), &[]);
    let wrong = unspelled(&rows());
    assert!(wrong.is_empty(), "{wrong:#?}");

    // And a cell answers its spelling's needs, and no other's: its base cell, a plant at a
    // line of its spelling — never one of another spelling, nor a line it replaces.
    let cell = |spelling: Option<&'static str>, aim: Option<Aim>, leads: &'static [Lead]| {
        byte_cell::Cell {
            name: "a-cell",
            leaf: &["probe"],
            state: Prepared::Bare,
            leads,
            argv: &["probe", "it"],
            listed: Listed::Acts,
            beside: None,
            aim,
            marks: &[],
            at: None,
            spelling,
        }
    };
    const A_STAGED_CHANGE: &[Lead] = &[Lead::Plant(Plant::Staged {
        at: "notes.txt",
        kept: true,
    })];
    let at = |line| {
        Some(Aim {
            line,
            kind: Kind::StagedElsewhere,
        })
    };
    assert_eq!(
        answers(&cell(Some("path-scoped"), None, &[]), &SPELLED),
        Ok(Need::BaseAs("path-scoped"))
    );
    assert_eq!(
        answers(
            &cell(Some("path-scoped"), at("P3"), A_STAGED_CHANGE),
            &SPELLED
        ),
        Ok(Need::Planted(
            at("P3").expect("an aim"),
            Some("path-scoped")
        ))
    );
    for (spelling, line, why) in [
        (None, "P3", "not in force under the spelling it runs"),
        (
            Some("path-scoped"),
            "P1",
            "not in force under the spelling it runs",
        ),
        (Some("elsewhere"), "P1", "which its row does not have"),
    ] {
        let fault = answers(&cell(spelling, at(line), A_STAGED_CHANGE), &SPELLED)
            .expect_err("a cell that answers no need of its spelling");
        assert!(fault.contains(why), "{spelling:?} at `{line}`: {fault}");
    }
    // A spelling's base cell stands in its row's home state, so a spelled cell with no
    // plant in another state, or in the refusing layer, answers nothing — the review of the
    // task `B7`'s first part, its tenth finding: each seen as a fault on its own.
    for (layer, spelled) in [
        (
            Layer::BaseElsewhere,
            byte_cell::Cell {
                state: Prepared::Of(State::Fresh),
                ..cell(Some("path-scoped"), None, &[])
            },
        ),
        (
            Layer::Refusing,
            byte_cell::Cell {
                state: Prepared::Of(State::Fresh),
                leaf: &["start"],
                argv: &["start"],
                ..cell(Some("path-scoped"), None, &[])
            },
        ),
    ] {
        assert_eq!(spelled.layer(&SPELLED), layer, "the fixture's layer");
        let fault = answers(&spelled, &SPELLED);
        assert!(
            fault.as_ref().is_err_and(|fault| fault.contains(
                "names the spelling `path-scoped` where a spelling's base cell stands in its \
                 row's home state"
            )),
            "a spelled cell in the layer {layer:?}: {fault:?}"
        );
    }

    // A need is keyed by the spelling as well as the line — the review of the task `B7`'s
    // first part, its ninth finding: a cell of a spelling aimed at a line of every spelling
    // is that line under the spelling, and never answers the plain spelling's need of it.
    const A_FILE_AT_THE_DOC: &[Lead] = &[Lead::Plant(Plant::File(DOC))];
    let at_the_doc = |spelling| {
        cell(
            spelling,
            Some(Aim {
                line: "P2",
                kind: Kind::Untracked,
            }),
            A_FILE_AT_THE_DOC,
        )
    };
    let plain_need = "`jigc probe` needs a cell for *P2 × U*, and has no cell";
    let spelled = at_the_doc(Some("path-scoped"));
    let unanswered = unmet(&[&SPELLED], &[&spelled], &[]);
    assert!(
        unanswered.iter().any(|line| line.starts_with(plain_need)),
        "a spelling's cell at a line of every spelling answers the plain spelling's need: \
         {unanswered:#?}"
    );
    let plain = at_the_doc(None);
    let unanswered = unmet(&[&SPELLED], &[&plain], &[]);
    assert!(
        !unanswered.iter().any(|line| line.starts_with(plain_need)),
        "the plain spelling's cell answers it: {unanswered:#?}"
    );
}

/// **A refusal may change only what a line that holds on a refusal covers.** A command
/// that exits non-zero and wrote one file is *refused, and changed what a refusal may
/// not* — under a line that covers the write at exit 0 too; it is *refuses, nothing
/// changed* under a line that holds on a refusal, and where it wrote nothing. A plant gone
/// on a refusal is no loss by the rule's words — the loss rule binds a command that exited
/// 0 — and is the refusal's own red.
#[test]
fn a_refusal_may_change_only_what_a_line_that_holds_on_a_refusal_covers() {
    const AT_EXIT_0: Row = row(&[line(
        "M1",
        Allows::Files(Where::Under(".jigc/milestones/*/merged"), May::WriteWhole),
    )]);
    const ON_A_REFUSAL: Row = row(&[Line {
        on_refusal: true,
        ..line(
            "M1",
            Allows::Files(Where::Under(".jigc/milestones/*/merged"), May::WriteWhole),
        )
    }]);
    let before = then(&sandbox(), |m| {
        for path in [
            "repo/.jigc",
            "repo/.jigc/milestones",
            "repo/.jigc/milestones/m1",
        ] {
            dir(m, path);
        }
        put(m, "repo/.jigc/milestones/m1/base.json", "{}\n");
        m.held.insert(blob_of("{}\n"));
    });
    let rebuilt = then(&before, |m| {
        dir(m, "repo/.jigc/milestones/m1/merged");
        put(m, "repo/.jigc/milestones/m1/merged/a.md", "# A\n");
    });
    fn refused<'a>(row: Row, before: &'a Manifest, after: &'a Manifest) -> Cell<'a> {
        Cell {
            code: Some(3),
            listed: Listed::Refuses,
            ..Cell::of(row, before, after)
        }
    }
    comes_out(
        &Cell::of(AT_EXIT_0, &before, &rebuilt),
        Classification::Holds,
    );
    comes_out(
        &refused(AT_EXIT_0, &before, &rebuilt),
        Classification::RefusedAndChanged,
    );
    comes_out(
        &refused(ON_A_REFUSAL, &before, &rebuilt),
        Classification::RefusesNothingChanged,
    );
    comes_out(
        &refused(AT_EXIT_0, &before, &before),
        Classification::RefusesNothingChanged,
    );
    // The line that holds on a refusal covers its own place and no other.
    let wider = then(&rebuilt, |m| {
        put(m, "repo/.jigc/milestones/m1/base.json", "{\"n\":1}\n")
    });
    comes_out(
        &refused(ON_A_REFUSAL, &before, &wider),
        Classification::RefusedAndChanged,
    );
    // A plant gone on a refusal.
    let planted = planted_under_the_workbench();
    let gone = then(&planted, |m| {
        m.files.remove("repo/.jigc/logs/notes.md");
    });
    comes_out(
        &Cell {
            planted: PLANT,
            code: Some(1),
            listed: Listed::Refuses,
            ..Cell::of(REMOVES, &planted, &gone)
        },
        Classification::RefusedAndChanged,
    );
}

/// **A refusal may rebuild a cache, and may not take one away** — the two words the
/// human's ruling of 2026-10-12 added to a line ([`May::Rebuilt`], [`When::Refused`]). A
/// command that refused and left a cache's file written whole, a file made under it, a
/// directory made on the way and one made with nothing in it yet *refuses, nothing
/// changed*, each difference counted under the cache's line. The same refusal with a file of the cache **gone** is red: that is
/// no rebuild. So is a link put in a cache file's place, and a write beside the cache.
/// And **the line holds on a refusal only**: the same file made by a command that exited
/// 0 is *written outside its allowance* — what an acting command may write to a cache is
/// its own row's to say.
#[test]
fn a_refusal_may_rebuild_a_cache_and_may_not_take_one_away() {
    const CACHE: Row = row(&[Line {
        when: When::Refused,
        on_refusal: true,
        ..line(
            "C1",
            Allows::Files(Where::Under(".jigc/index"), May::Rebuilt),
        )
    }]);
    let before = then(&sandbox(), |m| {
        dir(m, "repo/.jigc");
        dir(m, "repo/.jigc/index");
        put(m, "repo/.jigc/index/edges.json", "built against c0\n");
    });
    let refused = |after: &Manifest| {
        Cell {
            code: Some(1),
            listed: Listed::Refuses,
            ..Cell::of(CACHE, &before, after)
        }
        .verdict()
        .expect("a cell")
    };
    let rebuilt = refused(&then(&before, |m| {
        put(m, "repo/.jigc/index/edges.json", "built against the tree\n");
        dir(m, "repo/.jigc/index/by-doc");
        put(m, "repo/.jigc/index/by-doc/vision.json", "[]\n");
        dir(m, "repo/.jigc/index/by-task");
    }));
    assert_eq!(
        rebuilt.classification,
        Classification::RefusesNothingChanged,
        "{:#?}",
        rebuilt.why
    );
    assert_eq!(
        rebuilt.excused,
        [("C1", 4)],
        "the file written whole, the file made, the directory on its way, and a directory \
         made with nothing in it yet"
    );
    for (what, after) in [
        (
            "a file of the cache gone",
            then(&before, |m| {
                m.files.remove("repo/.jigc/index/edges.json");
            }),
        ),
        (
            "a link in a cache file's place",
            then(&before, |m| {
                m.files.insert(
                    "repo/.jigc/index/edges.json".to_string(),
                    Entry::Link("elsewhere".to_string()),
                );
            }),
        ),
        (
            "a write beside the cache",
            then(&before, |m| put(m, "repo/.jigc/notes.md", "notes\n")),
        ),
    ] {
        let verdict = refused(&after);
        assert_eq!(
            verdict.classification,
            Classification::RefusedAndChanged,
            "{what}: {:#?}",
            verdict.why
        );
    }
    // The same file made at exit 0: the line holds on a refusal, and only then.
    let made = then(&before, |m| {
        put(m, "repo/.jigc/index/stamp", "the tree\n");
    });
    comes_out(
        &Cell::of(CACHE, &before, &made),
        Classification::WrittenOutside,
    );
}

/// **The install-footprint record is no cache, and a directory laid out holds no file** —
/// the two words the human's rulings of 2026-10-15 on the install family's questions put
/// into the frame (the record's §6, entries 3 and 1). A refusal that leaves the record at
/// the path a run showed it under is *refused, and changed what a refusal may not*: a line
/// that lets a cache be rebuilt does not cover it ([`byte_cell::NO_CACHE`]), where a file
/// of the cache beside it still is. And a line that lays a directory out ([`May::LaidOut`])
/// lets the directory be made by a command that acts, and nothing in it: a file made there
/// is *written outside its allowance*, as is a directory made outside the line's place.
#[test]
fn the_footprint_record_is_no_cache_and_a_directory_laid_out_holds_no_file() {
    const CACHE: Row = row(&[Line {
        when: When::Refused,
        on_refusal: true,
        ..line(
            "C2",
            Allows::Files(Where::Under(".jigc/state"), May::Rebuilt),
        )
    }]);
    let before = then(&sandbox(), |m| dir(m, "repo/.jigc"));
    let refused = |after: &Manifest| {
        Cell {
            code: Some(1),
            listed: Listed::Refuses,
            ..Cell::of(CACHE, &before, after)
        }
        .came_to()
    };
    let with = |file: &str| {
        then(&before, |m| {
            dir(m, "repo/.jigc/state");
            put(m, &format!("repo/.jigc/state/{file}"), "a run's own\n");
        })
    };
    let (rebuilt, footprint) = (with("file-state.json"), with("setup-install-footprint"));
    let (class, why) = refused(&rebuilt);
    assert_eq!(class, Classification::RefusesNothingChanged, "{why:#?}");
    let (class, why) = refused(&footprint);
    assert!(
        class == Classification::RefusedAndChanged
            && why
                .iter()
                .any(|why| why.contains("`repo/.jigc/state/setup-install-footprint` was made")),
        "the footprint record left by a refusal is no rebuilt cache: {class}, {why:#?}"
    );

    const LAYS_OUT: Row = row(&[line(
        "L1",
        Allows::Files(Where::Under(".jigc/state"), May::LaidOut),
    )]);
    let laid_out = then(&before, |m| dir(m, "repo/.jigc/state"));
    let verdict = Cell::of(LAYS_OUT, &before, &laid_out)
        .verdict()
        .expect("a cell");
    assert_eq!(
        (verdict.classification, verdict.excused.as_slice()),
        (Classification::Holds, &[("L1", 1)][..]),
        "an empty directory laid out under the line: {:#?}",
        verdict.why
    );
    for (what, after) in [
        ("a file in the directory laid out", rebuilt.clone()),
        (
            "a directory beside the line's place",
            then(&before, |m| dir(m, "repo/.jigc/logs")),
        ),
    ] {
        let (class, why) = Cell::of(LAYS_OUT, &before, &after).came_to();
        assert_eq!(class, Classification::WrittenOutside, "{what}: {why:#?}");
    }
}

/// **A line lets a file under a directory go only where its sentence does** — the lines of
/// the table's own rows, each driven over a file of the product's own that stands in a
/// directory the row lets its command write into when it acts: the project layer `setup`
/// initializes, and the edge index and the file-state hashes a finalize writes after its
/// commit. No sentence of those lines lets a file go — *initializes … when absent*,
/// *invalidate the edge-index stamp*, *update `file-state` hashes* — so one that is gone
/// is *written outside its allowance*, and one written again holds. The one file of them
/// whose sentence lets it go is the stamped index's own, `.jigc/index/edges.json`, which
/// the next read rebuilds ([`a_finalize_may_take_the_edge_indexs_file_away_and_no_other_file`]);
/// the file here is another. Its pair is the one directory whose sentence says the command
/// **clears** it: a milestone's `merged/`.
///
/// Red first against the rows as they were lifted: each of the five lines said *written
/// whole* over a directory, which lets anything under it happen, a file gone included.
#[test]
fn a_directory_a_command_writes_into_is_not_one_it_may_empty() {
    let row_of = |leaf: &[&str]| {
        **rows()
            .iter()
            .find(|row| row.leaf == leaf)
            .expect("the leaf has a row")
    };
    let cache = |place: &str| format!("repo/{place}/a-file-of-the-products.json");
    let with = |place: &str| {
        then(&sandbox(), |m| {
            dir(m, "repo/.jigc");
            dir(m, &format!("repo/{place}"));
            put(m, &cache(place), "{}\n");
        })
    };
    let base = sandbox();
    let came = |leaf: &[&str], before: &Manifest, after: &Manifest| {
        Cell {
            base: Some(&base),
            ..Cell::of(row_of(leaf), before, after)
        }
        .came_to()
    };
    for (leaf, place) in [
        (&["setup"][..], ".jigc/config"),
        (&["task", "finalize"], ".jigc/index"),
        (&["task", "finalize"], ".jigc/state"),
        (&["milestone", "finalize"], ".jigc/index"),
        (&["milestone", "finalize"], ".jigc/state"),
    ] {
        let before = with(place);
        let gone = then(&before, |m| {
            m.files.remove(&cache(place));
        });
        let (class, why) = came(leaf, &before, &gone);
        assert_eq!(
            class,
            Classification::WrittenOutside,
            "`jigc {}`, a file gone from `{place}`: {why:#?}",
            leaf.join(" ")
        );
        let again = then(&before, |m| put(m, &cache(place), "{\"again\": true}\n"));
        let (class, why) = came(leaf, &before, &again);
        assert_eq!(
            class,
            Classification::Holds,
            "`jigc {}`, a file written again under `{place}`: {why:#?}",
            leaf.join(" ")
        );
    }
    // The pair: `materialize` *clears and rebuilds* a milestone's `merged/` every call —
    // the milestone the command names.
    let merged = ".jigc/milestones/a-milestone/merged";
    let before = then(&with(merged), |m| {
        dir(m, "repo/.jigc/milestones/a-milestone")
    });
    let cleared = then(&before, |m| {
        m.files.remove(&cache(merged));
    });
    let (class, why) = Cell {
        base: Some(&base),
        argv: &["milestone", "finalize", "a-milestone"],
        ..Cell::of(row_of(&["milestone", "finalize"]), &before, &cleared)
    }
    .came_to();
    assert_eq!(class, Classification::Holds, "{why:#?}");
}

/// **A finalize may take the edge index's file away, and no other file** — the review of the
/// task `B7`'s first part, its first finding. The docs say a finalize invalidates the
/// edge-index stamp after its commit and the **next read** rebuilds it (`design/finalize.md`
/// → *7. Post-commit (best-effort)*; `design/storage.md` → *Edge index lifecycle*), and
/// every door that invalidates takes `.jigc/index/edges.json` away; the line's word was
/// *rebuilt*, under which a file gone is none, so a finalize that met a built index read
/// red. On the rows `task::FINALIZE` (`T5`) and `milestone::FINALIZE` (`M10`) themselves,
/// over a built index — its file and its lock — beside the file-state hashes: the index's
/// file gone holds, and so does it written again or gone beside its lock written; its lock
/// gone, a link or a directory in its place, and the file-state hashes gone — a record with
/// *"no stamp and no rebuild path"* (`design/storage.md` → *Derived caches*) — are each red,
/// a difference no line covers; and on a refusal the index's file gone is red: what a
/// refusal may do to a cache is `C1`'s, which rebuilds and takes nothing away.
///
/// Red first against the line as it stood: the index's file gone came out *lost* on the
/// first row — the bytes of a cache stand nowhere once it is invalidated.
#[test]
fn a_finalize_may_take_the_edge_indexs_file_away_and_no_other_file() {
    const INDEX: &str = "repo/.jigc/index/edges.json";
    const LOCK: &str = "repo/.jigc/index/edges.json.lock";
    const HASHES: &str = "repo/.jigc/state/file-state.json";
    let before = then(&sandbox(), |m| {
        for at in ["repo/.jigc", "repo/.jigc/index", "repo/.jigc/state"] {
            dir(m, at);
        }
        put(m, INDEX, "{\"stamp\":\"c0\",\"edges\":[]}\n");
        put(m, LOCK, "");
        put(m, HASHES, "{\"hashes\":{}}\n");
    });
    let gone = |path: &'static str| {
        then(&before, move |m| {
            m.files.remove(path);
        })
    };
    for (row, argv) in [
        (task::FINALIZE, &["task", "finalize", "t1"][..]),
        (milestone::FINALIZE, &["milestone", "finalize", "m"][..]),
    ] {
        let leaf = row.leaf.join(" ");
        let came = |after: &Manifest, code: i32| {
            Cell {
                argv,
                code: Some(code),
                listed: if code == 0 {
                    Listed::Acts
                } else {
                    Listed::Refuses
                },
                ..Cell::of(row, &before, after)
            }
            .came_to()
        };
        for (what, after) in [
            ("the index's file taken away", gone(INDEX)),
            (
                "the index's file written again",
                then(&before, |m| {
                    put(m, INDEX, "{\"stamp\":\"c1\",\"edges\":[]}\n")
                }),
            ),
            (
                "the index's file taken away, its lock written",
                then(&gone(INDEX), |m| put(m, LOCK, "held\n")),
            ),
        ] {
            let (class, why) = came(&after, 0);
            assert_eq!(
                class,
                Classification::Holds,
                "`jigc {leaf}`, {what}: {why:#?}"
            );
        }
        for (what, after) in [
            ("the index's lock taken away", gone(LOCK)),
            (
                "a link in the index's file's place",
                then(&before, |m| {
                    m.files
                        .insert(INDEX.to_string(), Entry::Link("elsewhere".to_string()));
                }),
            ),
            (
                "a directory in the index's file's place",
                then(&before, |m| dir(m, INDEX)),
            ),
            ("the file-state hashes taken away", gone(HASHES)),
        ] {
            let (class, why) = came(&after, 0);
            assert!(
                class.is_red() && why.iter().any(|why| why.contains("no line covers it")),
                "`jigc {leaf}`, {what}: {class}, {why:#?}"
            );
        }
        let (class, why) = came(&gone(INDEX), 1);
        assert!(
            class.is_red() && why.iter().any(|why| why.contains(INDEX)),
            "`jigc {leaf}` refused, the index's file taken away: {class}, {why:#?}"
        );
    }
}

/// **A refusal is the refusal the cell names**: a cell of the table says which refusal it
/// expects — the exit code, and words the output holds — and one that refuses with another
/// code, or without saying so, is *not the listed outcome*. A refusal that also changed
/// something is that first, and shows the other class beside it.
#[test]
fn a_refusal_is_the_refusal_the_cell_names() {
    let before = sandbox();
    let named = Listed::RefusesWith(byte_cell::Refusal {
        exit: 3,
        says: "probe.refused",
        moved: &[],
    });
    let refused = |code, said| Cell {
        code: Some(code),
        said,
        listed: named,
        ..Cell::of(NOTHING, &before, &before)
    };
    comes_out(
        &refused(3, "blocking · probe.refused — no"),
        Classification::RefusesNothingChanged,
    );
    for (code, said, why) in [
        (
            1,
            "blocking · probe.refused — no",
            "ended with exit 1 and said so",
        ),
        (
            3,
            "blocking · probe.other — no",
            "ended with exit 3 and did not say so",
        ),
        (1, "could not do it", "ended with exit 1 and did not say so"),
    ] {
        let (class, said) = refused(code, said).came_to();
        assert_eq!(class, Classification::NotTheListedOutcome, "{said:#?}");
        assert!(
            said.iter().any(|line| line
                .contains("is listed as refusing with exit 3 and `probe.refused`")
                && line.contains(why)),
            "{said:#?}"
        );
    }
    // A cell that names its refusal and whose command exits 0 is not the listed outcome.
    let (class, said) = Cell {
        listed: named,
        ..Cell::of(NOTHING, &before, &before)
    }
    .came_to();
    assert_eq!(class, Classification::NotTheListedOutcome, "{said:#?}");
    // A refusal that is listed without a name is held to refusing, and to nothing more.
    comes_out(
        &Cell {
            listed: Listed::Refuses,
            ..refused(1, "could not do it")
        },
        Classification::RefusesNothingChanged,
    );
    // Another refusal that also wrote: the write outranks, and the other class stands
    // beside it.
    let wrote = then(&before, |m| put(m, "repo/left", "left\n"));
    let verdict = Cell {
        after: &wrote,
        ..refused(1, "could not do it")
    }
    .verdict()
    .expect("the cell is one the frame can compare");
    assert_eq!(verdict.classification, Classification::RefusedAndChanged);
    assert!(
        matches!(
            verdict.also.as_slice(),
            [(Classification::NotTheListedOutcome, why)] if why.len() == 1
        ),
        "{:#?}",
        verdict.also
    );
}

/// **A red cell shows every class it is red in**: its classification is the one that
/// outranks the others, and each lower class the comparison found stands beside it with
/// its reasons — so what else a command did in a cell where it lost bytes is read off the
/// cell, and not hidden behind the loss.
#[test]
fn a_red_cell_shows_every_class_it_is_red_in() {
    let before = planted_under_the_workbench();
    let after = then(&before, |m| {
        m.files.remove("repo/.jigc/logs/notes.md");
        put(m, "outside/left", "left\n");
        m.refs
            .insert("refs/heads/other".to_string(), "c0".to_string());
    });
    let verdict = Cell {
        planted: PLANT,
        listed: Listed::Refuses,
        ..Cell::of(NOTHING, &before, &after)
    }
    .verdict()
    .expect("the cell is one the frame can compare");
    assert_eq!(verdict.classification, Classification::Lost);
    let also: Vec<Classification> = verdict.also.iter().map(|(class, _)| *class).collect();
    assert_eq!(
        also,
        [
            Classification::GitMoved,
            Classification::WrittenOutside,
            Classification::NotTheListedOutcome
        ],
        "{:#?}",
        verdict.also
    );
    let shown = verdict.shown();
    assert!(
        shown.contains(
            "also *git's state moved outside its allowance*:\nthe ref `refs/heads/other`"
        ) && shown.contains("also *written outside its allowance*:\n`outside/left` was made"),
        "{shown}"
    );
    // A cell that is red in one class shows that one, and nothing beside it.
    let only = then(&before, |m| put(m, "outside/left", "left\n"));
    let verdict = Cell {
        planted: PLANT,
        ..Cell::of(NOTHING, &before, &only)
    }
    .verdict()
    .expect("the cell is one the frame can compare");
    assert_eq!(verdict.classification, Classification::WrittenOutside);
    assert!(verdict.also.is_empty(), "{:#?}", verdict.also);
}

/// **A staged doc that goes under the consent is one the command's output names**: the
/// rule for prose handed to jigc has one exemption, the consent's — and only where the
/// command said which doc went, by its identity, as *narrate under one* asks of a door
/// that destroys a working area.
#[test]
fn a_staged_doc_that_goes_under_the_consent_is_one_the_output_names() {
    const DISCARDS: Row = row(&[line(
        "D1",
        Allows::Files(Where::Under(".jigc/tasks/*"), May::Removed),
    )]);
    let base = sandbox();
    let before = then(&base, |m| {
        for at in [
            "repo/.jigc",
            "repo/.jigc/tasks",
            "repo/.jigc/tasks/t",
            "repo/.jigc/tasks/t/docs",
        ] {
            dir(m, at);
        }
        put(m, "repo/.jigc/tasks/t/docs/adr:one-way.md", "a decision\n");
        put(m, "repo/.jigc/tasks/t/docs/commit:t.md", "a message\n");
    });
    let after = then(&before, |m| {
        m.files
            .retain(|path, _| !path.starts_with("repo/.jigc/tasks"));
    });
    let discarded = |consent, said| Cell {
        base: Some(&base),
        consent,
        said,
        ..Cell::of(DISCARDS, &before, &after)
    };
    comes_out(
        &discarded(
            true,
            "discarded task t — dropped staged edits to: adr:one-way, commit:t",
        ),
        Classification::Holds,
    );
    for (consent, said) in [
        (true, "discarded task t"),
        (
            true,
            "discarded task t — dropped staged edits to: adr:another",
        ),
        (
            false,
            "discarded task t — dropped staged edits to: adr:one-way, commit:t",
        ),
    ] {
        let (class, why) = discarded(consent, said).came_to();
        assert_eq!(class, Classification::Lost, "{consent} `{said}`: {why:#?}");
        assert!(
            why.iter().any(|line| line
                .contains("the staged doc `repo/.jigc/tasks/t/docs/adr:one-way.md` is gone")),
            "{why:#?}"
        );
    }
    // A row with no consent has no exemption to give.
    const NO_CONSENT: Row = Row {
        consent: None,
        ..DISCARDS
    };
    comes_out(
        &Cell {
            row: NO_CONSENT,
            ..discarded(true, "dropped staged edits to: adr:one-way")
        },
        Classification::Lost,
    );
}

/// **The classifications, in their order.** A command that did not end is said first,
/// whatever it left. At exit 0 a loss outranks a write outside the allowance, and git's
/// state moved outranks it too; a cell that came out as listed holds, and one that did not
/// is *not the listed outcome* — read behind every other red. Six of the eight are red.
#[test]
fn the_classifications_in_their_order() {
    let before = planted_under_the_workbench();
    let lost_and_outside = then(&before, |m| {
        m.files.remove("repo/.jigc/logs/notes.md");
        put(m, "outside/left.txt", "left behind\n");
    });
    let both = Cell {
        planted: PLANT,
        ..Cell::of(NOTHING, &before, &lost_and_outside)
    };
    comes_out(&both, Classification::Lost);
    let not_ended = byte_cell::compare(
        &NOTHING,
        &Seen {
            base: &before,
            before: &read(before.clone(), &[MARKER]),
            after: &read(lost_and_outside.clone(), &[MARKER]),
            planted: &[Planted {
                at: PLANT[0].0.to_string(),
                marker: MARKER.to_string(),
                staged: false,
            }],
            argv: &[],
            spelling: None,
            ended: Ended {
                ended: false,
                code: None,
                said: "",
            },
            consent: false,
            listed: Listed::Acts,
        },
    )
    .expect("a cell");
    assert_eq!(not_ended.classification, Classification::DidNotEnd);
    let git_and_outside = then(&before, |m| {
        m.stash.push("s1".to_string());
        put(m, "outside/left.txt", "left behind\n");
    });
    comes_out(
        &Cell::of(NOTHING, &before, &git_and_outside),
        Classification::GitMoved,
    );
    let outside = then(&before, |m| put(m, "outside/left.txt", "left behind\n"));
    comes_out(
        &Cell {
            listed: Listed::Refuses,
            ..Cell::of(NOTHING, &before, &outside)
        },
        Classification::WrittenOutside,
    );
    comes_out(
        &Cell {
            listed: Listed::Refuses,
            ..Cell::of(NOTHING, &before, &before)
        },
        Classification::NotTheListedOutcome,
    );
    comes_out(
        &Cell {
            code: Some(1),
            ..Cell::of(NOTHING, &before, &before)
        },
        Classification::NotTheListedOutcome,
    );
    comes_out(&Cell::of(NOTHING, &before, &before), Classification::Holds);
    let all = [
        Classification::DidNotEnd,
        Classification::Lost,
        Classification::GitMoved,
        Classification::WrittenOutside,
        Classification::RefusedAndChanged,
        Classification::NotTheListedOutcome,
        Classification::Holds,
        Classification::RefusesNothingChanged,
    ];
    let red: Vec<&str> = all
        .iter()
        .filter(|c| c.is_red())
        .map(|c| c.as_str())
        .collect();
    assert_eq!(
        red,
        [
            "the command did not end",
            "lost",
            "git's state moved outside its allowance",
            "written outside its allowance",
            "refused, and changed what a refusal may not",
            "not the listed outcome",
        ]
    );
    assert_eq!(Classification::Holds.to_string(), "holds");
    assert_eq!(
        Classification::RefusesNothingChanged.to_string(),
        "refuses, nothing changed"
    );
}

/// **The excused are counted, line by line** — every line of the row, with how many
/// differences it excused: one that excused nothing reads 0, and a directory on the way to
/// a covered write counts with the line that covers the write.
#[test]
fn the_excused_are_counted_line_by_line() {
    const TWO: Row = row(&[
        line(
            "E1",
            Allows::Files(Where::Path(".jigc/AGENT.md"), May::WriteWhole),
        ),
        line(
            "E2",
            Allows::Files(Where::Under(".jigc/config"), May::WriteWhole),
        ),
        line(
            "E3",
            Allows::Files(Where::Path("CLAUDE.md"), May::MergeInto),
        ),
    ]);
    let before = sandbox();
    let after = then(&before, |m| {
        dir(m, "repo/.jigc");
        put(m, "repo/.jigc/AGENT.md", "the bootstrap\n");
        dir(m, "repo/.jigc/config");
        put(m, "repo/.jigc/config/.gitkeep", "");
        put(m, "repo/.jigc/config/packs.yaml", "packs: []\n");
    });
    let verdict = Cell::of(TWO, &before, &after).verdict().expect("a cell");
    assert_eq!(
        verdict.classification,
        Classification::Holds,
        "{:#?}",
        verdict.why
    );
    assert_eq!(verdict.excused, [("E1", 2), ("E2", 3), ("E3", 0)]);
    // A directory that is on the way to nothing a line covers is a write of its own.
    let stray = then(&after, |m| dir(m, "repo/.jigc/tasks"));
    comes_out(
        &Cell::of(TWO, &before, &stray),
        Classification::WrittenOutside,
    );
}

// ---------------------------------------------------------------------------
// A prepared state — built on the binary handed in
// ---------------------------------------------------------------------------

/// Place an executable script at `path`, as the Linux rule asks of a file that may be
/// executed (`implementation/dev-workflow.md` → Gate): written by a child process, so no
/// thread of this one ever holds its write handle and no sibling's fork can inherit one.
fn place_script(path: &Path, body: &str) {
    let staged = path.with_extension("staged");
    fs::write(&staged, body).expect("stage the script");
    let placed = Command::new("cp")
        .arg("--")
        .arg(&staged)
        .arg(path)
        .stdin(Stdio::null())
        .status()
        .expect("run cp");
    assert!(placed.success(), "cp to place {}", path.display());
    fs::set_permissions(path, fs::Permissions::from_mode(0o755))
        .expect("make the script executable");
}

/// **A prepared state is built on the binary handed in, and every later child of the
/// corpus is that binary** (the plan's task `B2a`). The binary here is a script that
/// writes down, under the `$HOME` it was given, the arguments it was started with — so
/// what the builder started is read off the corpus, and cargo's binary of this tree,
/// which the builder named itself until this task, leaves no line there. `fresh` is one
/// `jigc setup`: the builder started the script with `setup` and nothing else. Then the
/// two places a corpus starts its binary from — a plain child and one fed its stdin —
/// and a copy of the state, which keeps the binary its source was built on.
#[test]
fn a_prepared_state_is_built_on_the_binary_handed_in() {
    let scratch = ScratchDir::new("byte-table-a-binary-handed-in");
    let script = scratch.path().join("jigc");
    place_script(
        &script,
        "#!/bin/sh\nprintf '%s\\n' \"$*\" >> \"$HOME/started\"\n",
    );
    let started = |corpus: &TrialCorpus| {
        fs::read_to_string(corpus.home().join("started")).unwrap_or_default()
    };

    let corpus = TrialCorpus::build_on(&script, State::Fresh);
    assert_eq!(
        started(&corpus),
        "setup\n",
        "`fresh` is one `jigc setup`, started on the binary handed in"
    );
    corpus.jigc(&["doc", "list"]);
    corpus.jigc_stdin(&["doc", "author", "--from-file", "-"], "");
    assert_eq!(
        started(&corpus),
        "setup\ndoc list\ndoc author --from-file -\n",
        "every later child of the corpus is the binary it was built on"
    );
    let copy = corpus.copy_state();
    copy.jigc(&["validate"]);
    assert_eq!(
        started(&copy),
        "setup\ndoc list\ndoc author --from-file -\nvalidate\n",
        "a copy of a state keeps the binary its source was built on"
    );
}

// ---------------------------------------------------------------------------
// The half that reads a sandbox — the reader and the runner, on a stand-in
// ---------------------------------------------------------------------------

/// **The stand-in**: a script an arm hands the frame as the binary under test. It does
/// what its first word says and nothing else, in the directory it is started in — the
/// sandbox's repository — so that each cell of [`ON_A_STAND_IN`] is one known act between
/// two manifests, and what the reader and the comparison make of it is the arm's to hold.
const STAND_IN: &str = r#"#!/bin/sh
case "$1" in
removes-the-plant) rm .jigc/logs/notes.md ;;
removes-and-names) rm .jigc/logs/notes.md && echo "removed .jigc/logs/notes.md" ;;
moves-the-plant) mv .jigc/logs/notes.md elsewhere.md ;;
stages-then-removes) git add .jigc/logs/notes.md && rm .jigc/logs/notes.md ;;
stores-then-removes) git hash-object -w .jigc/logs/notes.md >/dev/null && rm .jigc/logs/notes.md ;;
writes-outside) echo left > ../outside/left ;;
writes-home) echo left > "$HOME/left" ;;
writes-tmp) echo left > "$TMPDIR/left" ;;
the-same-under-conversion) printf 'one\ntwo\n' > notes.txt ;;
one-byte-under-conversion) printf 'one\ntwO\n' > notes.txt ;;
a-gone-worktrees-index) GIT_INDEX_FILE=.git/worktrees/own/index git rm -q --cached README.md ;;
status-and-a-time) git status >/dev/null && touch -t 200001010000 README.md ;;
one-byte-size-and-time-back)
    touch -r README.md "$TMPDIR/as-it-was" && printf 'trial corpuS\n' > README.md &&
        touch -r "$TMPDIR/as-it-was" README.md && rm "$TMPDIR/as-it-was" ;;
removes-in-the-other-checkout) rm ../outside/own/README.md ;;
unstages-in-the-other-checkout) git -C ../outside/own rm -q --cached README.md ;;
drops-the-registration) git worktree remove --force ../outside/own ;;
drops-a-stash-entry) git stash drop -q ;;
moves-a-branch) git branch -f other HEAD ;;
unstages-a-path) git rm -q --cached staged.txt ;;
commits-a-link) ln -s nowhere link && git add link && git commit -q -m link ;;
commits-the-index) git commit -q -m staged ;;
commits-a-removal) git rm -q README.md && git commit -q -m gone ;;
prunes) git prune --expire=now ;;
configures) git config user.name Somebody ;;
writes-the-hook) mkdir -p .git/hooks && printf '#!/bin/sh\n' > .git/hooks/pre-commit ;;
changes-a-mode) chmod 755 README.md ;;
repoints-a-link) rm a-link && ln -s two a-link ;;
writes-the-exclude) mkdir -p .git/info && echo ignored >> .git/info/exclude ;;
marks-an-entry) git update-index --assume-unchanged README.md ;;
refuses-and-writes) echo left > left; exit 1 ;;
refuses) exit 1 ;;
refuses-showing-the-readme) cat README.md; exit 1 ;;
writes-its-own) mkdir -p .jigc && echo own > .jigc/own.txt ;;
rewrites-its-own) echo other > .jigc/own.txt ;;
overwrites-the-committed-plant) echo other > notes.txt ;;
drops-the-last-commit) git reset -q --hard HEAD~1 ;;
writes-through-the-link) echo prose > a-link.md ;;
rewrites-the-edited-file) echo rewritten > README.md ;;
prunes-the-worktrees) git worktree prune ;;
seeds-the-ignore-file) echo secrets > .gitignore ;;
writes-a-baseline) mkdir -p .jigc/state && echo '{}' > .jigc/state/hashes.json ;;
empties-the-directory) rm -r a-dir ;;
empties-the-readme) rm -r README.md ;;
drops-the-staged) git reset -q -- README.md ;;
commits-nothing) git commit -q --allow-empty -m nothing ;;
commits-what-is-staged) git commit -q -m staged ;;
writes-here) echo here > here.txt ;;
sleeps) exec sleep 30 ;;
*) echo "the stand-in knows no '$1'" >&2; exit 64 ;;
esac
"#;

/// One cell on the stand-in: the row it is compared under, what it must come out as, and
/// a piece of why.
struct OnAStandIn {
    cell: byte_cell::Cell,
    row: Row,
    expected: Classification,
    why: &'static str,
}

const fn on_a_stand_in(
    argv: &'static [&'static str],
    leads: &'static [Lead],
    row: Row,
    expected: Classification,
    why: &'static str,
) -> OnAStandIn {
    OnAStandIn {
        cell: byte_cell::Cell {
            spelling: None,
            name: argv[0],
            leaf: &["probe"],
            state: Prepared::Bare,
            leads,
            argv,
            listed: Listed::Acts,
            beside: None,
            aim: None,
            marks: &[],
            at: None,
        },
        row,
        expected,
        why,
    }
}

/// The stand-in writing `here.txt` where it runs, a directory `sub/` made first — run at
/// `sub/` by the case that names the place.
const WRITES_HERE: OnAStandIn = on_a_stand_in(
    &["writes-here"],
    &[Lead::Step(Step::Dir(Place::Repo("sub")))],
    HERE,
    Classification::Holds,
    "",
);

/// A row that lets its command write `sub/here.txt`, and nothing else.
const HERE: Row = row(&[line(
    "W1",
    Allows::Files(Where::Path("sub/here.txt"), May::WriteWhole),
)]);

/// A file somebody planted under the workbench — [`PLANT`]'s place, in a sandbox.
const A_PLANT: &[Lead] = &[Lead::Plant(Plant::File(".jigc/logs/notes.md"))];

/// A tracked file with CRLF line ends in a repository that converts them: the commit
/// holds it with LF.
const UNDER_CONVERSION: &[Lead] = &[
    Lead::Step(Step::Git(&["config", "core.autocrlf", "true"])),
    Lead::Step(Step::File(Place::Repo("notes.txt"), "one\r\ntwo\r\n")),
    Lead::Step(Step::Git(&["add", "notes.txt"])),
    Lead::Step(Step::Git(&["commit", "-q", "-m", "notes"])),
];

/// A linked worktree of the user's, outside the repository.
const A_WORKTREE: Lead = Lead::Step(Step::Git(&[
    "worktree",
    "add",
    "-q",
    "--detach",
    "../outside/own",
    "HEAD",
]));

/// A file the user staged.
const STAGED: &[Lead] = &[
    Lead::Step(Step::File(Place::Repo("staged.txt"), "staged\n")),
    Lead::Step(Step::Git(&["add", "staged.txt"])),
];

/// A row that lets its command write the `pre-commit` hook.
const HOOKS: Row = row(&[line("H1", Allows::Files(Where::Hook, May::MergeInto))]);

/// A row that lets its command write one file of its own.
const ITS_OWN: Row = row(&[line(
    "O1",
    Allows::Files(Where::Path(".jigc/own.txt"), May::WriteWhole),
)]);

/// A row that lets its command write the root `.gitignore` while the repository has no
/// commit.
const SEEDS: Row = row(&[Line {
    when: When::NoCommitYet,
    ..line(
        "G1",
        Allows::Files(Where::Path(".gitignore"), May::MergeInto),
    )
}]);

/// **The cells on the stand-in**, a layer of the manifest each — what the reader must
/// read for the comparison to come out as it did over manifests built by hand — and then
/// **a plant and a condition each**: what the frame puts into a state for a cell.
const ON_A_STAND_IN: &[OnAStandIn] = &[
    // --- where the command runs (the task `B9`, the mint family's cells: a sub-task is
    // re-entered at its worktree): a cell run at a place of the sandbox writes there ---
    OnAStandIn {
        cell: byte_cell::Cell {
            at: Some(Place::Repo("sub")),
            ..WRITES_HERE.cell
        },
        ..WRITES_HERE
    },
    // --- the plants: a committed file, a link to nothing, an edit, the user's worktree ---
    on_a_stand_in(
        &["overwrites-the-committed-plant"],
        &[Lead::Plant(Plant::Committed("notes.txt"))],
        NOTHING,
        Classification::GitMoved,
        "the planted `repo/notes.txt` is gone from every path of the sandbox, and an object \
         within reach still has its bytes",
    ),
    // The commit that holds the plant is reached by nothing but the reflogs that name it:
    // held, by the human's ruling of 2026-10-14 on what *held* means — reflogs included.
    on_a_stand_in(
        &["drops-the-last-commit"],
        &[Lead::Plant(Plant::Committed("notes.txt"))],
        NOTHING,
        Classification::GitMoved,
        "the planted `repo/notes.txt` is gone from every path of the sandbox, and an object \
         within reach still has its bytes",
    ),
    // A line of the user's own, in a file jigc merges into and in the hook (plant **M**).
    on_a_stand_in(
        &["rewrites-the-edited-file"],
        &[Lead::Plant(Plant::UsersLine("README.md"))],
        NOTHING,
        Classification::Lost,
        "the planted `repo/README.md` is gone",
    ),
    on_a_stand_in(
        &["writes-the-hook"],
        &[Lead::Plant(Plant::UsersHook)],
        HOOKS,
        Classification::Lost,
        "the planted `repo/.git/hooks/pre-commit` is gone",
    ),
    on_a_stand_in(
        &["writes-through-the-link"],
        &[Lead::Plant(Plant::LinkToNothing("a-link.md"))],
        NOTHING,
        Classification::WrittenOutside,
        "-only-of-its-cell.md` was made, and no line covers it",
    ),
    on_a_stand_in(
        &["rewrites-the-edited-file"],
        &[Lead::Plant(Plant::Edit("README.md"))],
        NOTHING,
        Classification::Lost,
        "the planted `repo/README.md` is gone, and its bytes stand at no path",
    ),
    on_a_stand_in(
        &["prunes-the-worktrees"],
        &[Lead::Plant(Plant::Worktree { moved_away: true })],
        NOTHING,
        Classification::GitMoved,
        "the registration `own`, of the worktree at `outside/own`, is gone",
    ),
    on_a_stand_in(
        &["prunes-the-worktrees"],
        &[Lead::Plant(Plant::Worktree { moved_away: false })],
        NOTHING,
        Classification::Holds,
        "",
    ),
    // --- the two conditions: `HEAD` unborn, and no file-state baseline ---
    on_a_stand_in(
        &["seeds-the-ignore-file"],
        &[Lead::Under(Condition::HeadUnborn)],
        SEEDS,
        Classification::Holds,
        "",
    ),
    on_a_stand_in(
        &["seeds-the-ignore-file"],
        &[],
        SEEDS,
        Classification::WrittenOutside,
        "`repo/.gitignore` was made, and no line covers it",
    ),
    on_a_stand_in(
        &["writes-a-baseline"],
        &[
            Lead::Step(Step::File(Place::Repo(".jigc/state/hashes.json"), "{}\n")),
            Lead::Under(Condition::NoBaseline),
        ],
        NOTHING,
        Classification::WrittenOutside,
        "`repo/.jigc/state/hashes.json` was made, and no line covers it",
    ),
    // --- a plant in the place of a file that stands: the committed `README.md` taken away,
    // and a file of the user's, one the user committed, a link or a directory put there ---
    on_a_stand_in(
        &["rewrites-the-edited-file"],
        &[Lead::Plant(Plant::InPlaceOf(&Plant::File(README)))],
        NOTHING,
        Classification::Lost,
        "the planted `repo/README.md` is gone, and its bytes stand at no path",
    ),
    on_a_stand_in(
        &["rewrites-the-edited-file"],
        &[Lead::Plant(Plant::InPlaceOf(&Plant::Committed(README)))],
        NOTHING,
        Classification::GitMoved,
        "the planted `repo/README.md` is gone from every path of the sandbox, and an object \
         within reach still has its bytes",
    ),
    on_a_stand_in(
        &["rewrites-the-edited-file"],
        &[Lead::Plant(Plant::InPlaceOf(&Plant::LinkToNothing(README)))],
        NOTHING,
        Classification::WrittenOutside,
        "-only-of-its-cell.md` was made, and no line covers it",
    ),
    on_a_stand_in(
        &["rewrites-the-edited-file"],
        &[Lead::Plant(Plant::InPlaceOf(&Plant::LinkToAFile {
            at: README,
            outside: true,
        }))],
        NOTHING,
        Classification::Lost,
        "the planted `outside/byte-table-plant-0-only-of-its-cell.md` is gone",
    ),
    on_a_stand_in(
        &["empties-the-readme"],
        &[Lead::Plant(Plant::InPlaceOf(&Plant::Dir(README)))],
        NOTHING,
        Classification::Lost,
        "the planted `repo/README.md/a-file-of-its-users.txt` is gone",
    ),
    // --- a file simply deleted (**E**, the file deleted): the committed `README.md` gone
    // from the tree, nothing in its place, and the index left as it stands — the deletion
    // is no difference of the command's; a file it puts back is one, which no line of
    // `NOTHING` covers; and the index's entry is still there for a `git rm` to take ---
    on_a_stand_in(
        &["prunes-the-worktrees"],
        &[Lead::Plant(Plant::Deleted(README))],
        NOTHING,
        Classification::Holds,
        "",
    ),
    on_a_stand_in(
        &["rewrites-the-edited-file"],
        &[Lead::Plant(Plant::Deleted(README))],
        NOTHING,
        Classification::WrittenOutside,
        "`repo/README.md` was made, and no line covers it",
    ),
    on_a_stand_in(
        &["commits-a-removal"],
        &[Lead::Plant(Plant::Deleted(README))],
        COMMITS_THE_INDEX,
        Classification::GitMoved,
        "a commit the command made removes `README.md`, which no line declares",
    ),
    // --- the plants of a family's cells: a link to a file, a directory, what is staged, a
    // hook that rejects — and a lead-in that must refuse ---
    on_a_stand_in(
        &["writes-through-the-link"],
        &[Lead::Plant(Plant::LinkToAFile {
            at: "a-link.md",
            outside: false,
        })],
        NOTHING,
        Classification::Lost,
        "the planted `repo/byte-table-plant-0-only-of-its-cell.md` is gone",
    ),
    on_a_stand_in(
        &["writes-through-the-link"],
        &[Lead::Plant(Plant::LinkToAFile {
            at: "a-link.md",
            outside: true,
        })],
        NOTHING,
        Classification::Lost,
        "the planted `outside/byte-table-plant-0-only-of-its-cell.md` is gone",
    ),
    on_a_stand_in(
        &["refuses"],
        &[Lead::Plant(Plant::LinkToAFile {
            at: "a-link.md",
            outside: true,
        })],
        NOTHING,
        Classification::NotTheListedOutcome,
        "the cell is listed as acting, and the command refused",
    ),
    on_a_stand_in(
        &["empties-the-directory"],
        &[Lead::Plant(Plant::Dir("a-dir"))],
        NOTHING,
        Classification::Lost,
        "the planted `repo/a-dir/a-file-of-its-users.txt` is gone",
    ),
    on_a_stand_in(
        &["drops-the-staged"],
        &[Lead::Plant(Plant::Staged {
            at: "README.md",
            kept: false,
        })],
        NOTHING,
        Classification::Lost,
        "what was staged at `repo/README.md` is in the index no longer",
    ),
    on_a_stand_in(
        &["commits-what-is-staged"],
        &[Lead::Plant(Plant::Staged {
            at: "README.md",
            kept: false,
        })],
        COMMITS_THE_INDEX,
        Classification::Holds,
        "",
    ),
    on_a_stand_in(
        &["refuses"],
        &[Lead::Plant(Plant::Staged {
            at: "notes/staged-by-its-user.txt",
            kept: false,
        })],
        NOTHING,
        Classification::NotTheListedOutcome,
        "the cell is listed as acting, and the command refused",
    ),
    on_a_stand_in(
        &["commits-what-is-staged"],
        &[Lead::Plant(Plant::Staged {
            at: "notes/staged-by-its-user.txt",
            kept: true,
        })],
        COMMITS_THE_INDEX,
        Classification::Holds,
        "",
    ),
    on_a_stand_in(
        &["commits-nothing"],
        &[Lead::Plant(Plant::RejectingHook)],
        NOTHING,
        Classification::NotTheListedOutcome,
        "the cell is listed as acting, and the command refused",
    ),
    on_a_stand_in(
        &["commits-nothing"],
        &[
            Lead::Step(Step::Script(
                Place::Repo(".git/hooks/pre-commit"),
                "#!/bin/sh\necho what the hook held\n",
            )),
            Lead::Plant(Plant::RejectingHook),
        ],
        NOTHING,
        Classification::NotTheListedOutcome,
        "the cell is listed as acting, and the command refused",
    ),
    on_a_stand_in(
        &["refuses"],
        &[Lead::Refused(&["refuses"])],
        NOTHING,
        Classification::NotTheListedOutcome,
        "the cell is listed as acting, and the command refused",
    ),
    // A write over a hand edit, which the human's ruling of 2026-10-19 refuses showing the
    // edit and this tree lets act: either way the cell goes on to its command.
    on_a_stand_in(
        &["refuses"],
        &[
            Lead::Plant(Plant::Edit("README.md")),
            Lead::OverAHandEdit {
                step: Step::Jigc(&["refuses-showing-the-readme"]),
                plant: 0,
            },
        ],
        NOTHING,
        Classification::NotTheListedOutcome,
        "the cell is listed as acting, and the command refused",
    ),
    on_a_stand_in(
        &["refuses"],
        &[
            Lead::Plant(Plant::Edit("README.md")),
            Lead::OverAHandEdit {
                step: Step::Jigc(&["status-and-a-time"]),
                plant: 0,
            },
        ],
        NOTHING,
        Classification::NotTheListedOutcome,
        "the cell is listed as acting, and the command refused",
    ),
    // --- where a plant's bytes stand: at a path, within reach, in the store, nowhere ---
    on_a_stand_in(
        &["removes-the-plant"],
        A_PLANT,
        REMOVES,
        Classification::Lost,
        "its bytes stand at no path of the sandbox and in no object within reach",
    ),
    on_a_stand_in(
        &["removes-and-names", "--force"],
        A_PLANT,
        REMOVES,
        Classification::Holds,
        "",
    ),
    on_a_stand_in(
        &["removes-and-names"],
        A_PLANT,
        REMOVES,
        Classification::Lost,
        "is gone",
    ),
    on_a_stand_in(
        &["moves-the-plant"],
        A_PLANT,
        REMOVES,
        Classification::WrittenOutside,
        "its bytes stand at {\"repo/elsewhere.md\"}",
    ),
    on_a_stand_in(
        &["stages-then-removes"],
        A_PLANT,
        REMOVES,
        Classification::GitMoved,
        "an object within reach still has its bytes",
    ),
    on_a_stand_in(
        &["stores-then-removes"],
        A_PLANT,
        REMOVES,
        Classification::Lost,
        "in no object within reach",
    ),
    // --- the file layer beside the repository, `tmp/` as every child's `TMPDIR` ---
    on_a_stand_in(
        &["writes-outside"],
        &[],
        NOTHING,
        Classification::WrittenOutside,
        "`outside/left` was made",
    ),
    on_a_stand_in(
        &["writes-home"],
        &[],
        NOTHING,
        Classification::WrittenOutside,
        "`home/left` was made",
    ),
    on_a_stand_in(
        &["writes-tmp"],
        &[],
        NOTHING,
        Classification::WrittenOutside,
        "`tmp/left` was made",
    ),
    on_a_stand_in(
        &["changes-a-mode"],
        &[],
        NOTHING,
        Classification::WrittenOutside,
        "`repo/README.md` is not what it was",
    ),
    on_a_stand_in(
        &["repoints-a-link"],
        &[Lead::Step(Step::LinkTo(Place::Repo("a-link"), "one"))],
        NOTHING,
        Classification::WrittenOutside,
        "`repo/a-link` is not what it was",
    ),
    // --- *held* is asked through git's filters ---
    on_a_stand_in(
        &["the-same-under-conversion"],
        UNDER_CONVERSION,
        NOTHING,
        Classification::Holds,
        "",
    ),
    on_a_stand_in(
        &["one-byte-under-conversion"],
        UNDER_CONVERSION,
        NOTHING,
        Classification::WrittenOutside,
        "`repo/notes.txt` is not what it was",
    ),
    // --- what is not compared stays quiet, and what is compared is not missed ---
    on_a_stand_in(
        &["status-and-a-time"],
        &[],
        NOTHING,
        Classification::Holds,
        "",
    ),
    on_a_stand_in(
        &["one-byte-size-and-time-back"],
        &[],
        NOTHING,
        Classification::WrittenOutside,
        "`repo/README.md` is not what it was",
    ),
    // --- another checkout, and a registration whose directory is gone ---
    on_a_stand_in(
        &["removes-in-the-other-checkout"],
        &[A_WORKTREE],
        NOTHING,
        Classification::WrittenOutside,
        "`outside/own/README.md` is gone",
    ),
    on_a_stand_in(
        &["unstages-in-the-other-checkout"],
        &[A_WORKTREE],
        NOTHING,
        Classification::GitMoved,
        "the index entry at `README.md`, in the checkout at `outside/own`, is not what it was",
    ),
    on_a_stand_in(
        &["drops-the-registration"],
        &[A_WORKTREE],
        NOTHING,
        Classification::GitMoved,
        "the registration `own`, of the worktree at `outside/own`, is gone",
    ),
    on_a_stand_in(
        &["a-gone-worktrees-index"],
        &[A_WORKTREE, Lead::MovedAway("outside/own")],
        NOTHING,
        Classification::GitMoved,
        "`repo/.git/worktrees/own/index`, a file of git's, is not what it was",
    ),
    // --- git's layer ---
    on_a_stand_in(
        &["drops-a-stash-entry"],
        &[
            Lead::Step(Step::File(Place::Repo("README.md"), "edited\n")),
            Lead::Step(Step::Git(&["stash"])),
        ],
        NOTHING,
        Classification::GitMoved,
        "the stash is not what it was",
    ),
    on_a_stand_in(
        &["moves-a-branch"],
        &[
            Lead::Step(Step::Git(&["branch", "other"])),
            Lead::Step(Step::File(Place::Repo("more.txt"), "more\n")),
            Lead::Step(Step::Git(&["add", "more.txt"])),
            Lead::Step(Step::Git(&["commit", "-q", "-m", "more"])),
        ],
        NOTHING,
        Classification::GitMoved,
        "the ref `refs/heads/other` is not what it was",
    ),
    on_a_stand_in(
        &["unstages-a-path"],
        STAGED,
        NOTHING,
        Classification::GitMoved,
        "the index entry at `staged.txt`, in the checkout at `repo`, is not what it was",
    ),
    on_a_stand_in(
        &["commits-a-link"],
        &[],
        COMMITS_THE_INDEX,
        Classification::GitMoved,
        "a commit the command made holds a link at `link`",
    ),
    on_a_stand_in(
        &["commits-the-index"],
        STAGED,
        COMMITS_THE_INDEX,
        Classification::Holds,
        "",
    ),
    on_a_stand_in(
        &["commits-a-removal"],
        &[],
        COMMITS_THE_INDEX,
        Classification::GitMoved,
        "a commit the command made removes `README.md`, which no line declares",
    ),
    on_a_stand_in(
        &["prunes"],
        &[Lead::Step(Step::Git(&["hash-object", "-w", "--stdin"]))],
        NOTHING,
        Classification::GitMoved,
        "was in the store and is gone",
    ),
    on_a_stand_in(
        &["configures"],
        &[],
        NOTHING,
        Classification::GitMoved,
        "the repository's configuration is not what it was",
    ),
    on_a_stand_in(
        &["writes-the-hook"],
        &[],
        NOTHING,
        Classification::GitMoved,
        "`repo/.git/hooks/pre-commit`, a file of git's, was made",
    ),
    on_a_stand_in(&["writes-the-hook"], &[], HOOKS, Classification::Holds, ""),
    on_a_stand_in(
        &["writes-the-exclude"],
        &[],
        NOTHING,
        Classification::GitMoved,
        "`repo/.git/info/exclude`, a file of git's,",
    ),
    on_a_stand_in(
        &["marks-an-entry"],
        &[],
        NOTHING,
        Classification::GitMoved,
        "the index entry at `README.md`, in the checkout at `repo`, is not what it was",
    ),
    // --- how the command ended ---
    on_a_stand_in(
        &["refuses-and-writes"],
        &[],
        NOTHING,
        Classification::RefusedAndChanged,
        "`repo/left` was made",
    ),
    OnAStandIn {
        cell: byte_cell::Cell {
            spelling: None,
            name: "refuses",
            leaf: &["probe"],
            state: Prepared::Bare,
            leads: &[],
            argv: &["refuses"],
            listed: Listed::Refuses,
            beside: None,
            aim: None,
            marks: &[],
            at: None,
        },
        row: NOTHING,
        expected: Classification::RefusesNothingChanged,
        why: "",
    },
    // --- what a lead-in of the cell wrote is the product's own ---
    on_a_stand_in(
        &["rewrites-its-own"],
        &[Lead::Step(Step::Jigc(&["writes-its-own"]))],
        NOTHING,
        Classification::WrittenOutside,
        "`repo/.jigc/own.txt` is not what it was, and no line covers it",
    ),
    on_a_stand_in(
        &["rewrites-its-own"],
        &[Lead::Step(Step::Jigc(&["writes-its-own"]))],
        ITS_OWN,
        Classification::Holds,
        "",
    ),
];

/// The stand-in, placed in `scratch` as a binary.
fn the_stand_in(scratch: &ScratchDir) -> PathBuf {
    let script = scratch.path().join("jigc");
    place_script(&script, STAND_IN);
    script
}

/// **The reader reads what the comparison reads, and the runner runs one command between
/// two manifests** — every cell of [`ON_A_STAND_IN`] on a script handed in as the binary,
/// in the never-adopted repository. Each cell is one known act: a plant removed, moved,
/// staged or stored and then removed; a write under `outside/`, `home/` and `tmp/` —
/// which is every child's `TMPDIR`; a permission bit; a file rewritten to the same content
/// under a line-ending conversion, which is no difference, and with one byte changed,
/// which is one; a plain `git status` and a modification time, which are in no layer, and
/// one byte changed with its size and time put back, which is seen; a file removed in
/// another checkout, a registration dropped, and the admin index of a registration whose
/// directory is gone rewritten, and an entry of another checkout's index dropped; a link
/// pointed elsewhere; a stash entry, a branch, an index entry and an index entry's flag, a
/// link committed, the index committed, a removal committed, an object pruned, the configuration, the hook and
/// `info/exclude`; a refusal that wrote and one that did not; and a file a lead-in of the
/// cell wrote, rewritten. And a file simply deleted, the index left as it stands: left
/// standing, written back, and its deletion committed.
///
/// Then the faults of a cell, which no classification covers: a lead-in that is refused,
/// a plant where something stands, a state that is not the one handed in, a row of
/// another leaf, a spelling its row does not have.
#[test]
fn the_reader_reads_what_the_comparison_reads() {
    let scratch = ScratchDir::new("byte-table-stand-in");
    let built = Built::on(&the_stand_in(&scratch), Prepared::Bare);
    let ran = route_case::pooled(route_case::width(), ON_A_STAND_IN, |on| {
        byte_cell::run(&built, &on.row, &on.cell)
    });
    let mut wrong = Vec::new();
    for (on, ran) in ON_A_STAND_IN.iter().zip(&ran) {
        let named = format!(
            "`{}` under {} line(s)",
            on.cell.argv.join(" "),
            on.row.lines.len()
        );
        match ran {
            Err(fault) => wrong.push(format!("{named} could not be run as written: {fault}")),
            Ok(ran) => {
                let verdict = &ran.verdict;
                // Why: the piece stands in a line of it — or it says nothing, as expected.
                let says = verdict.why.iter().any(|why| why.contains(on.why))
                    || on.why.is_empty() && verdict.why.is_empty();
                if verdict.classification != on.expected || !says {
                    wrong.push(format!(
                        "{named} came out *{}*, and *{}* with `{}` was expected:\n{}",
                        verdict.classification, on.expected, on.why, ran.detail
                    ));
                }
                if ran
                    .before
                    .files
                    .keys()
                    .any(|path| path.starts_with("staging"))
                {
                    wrong.push(format!(
                        "{named}: the frame's own `staging/` is in the manifest"
                    ));
                }
            }
        }
    }
    assert!(
        wrong.is_empty(),
        "the cells on the stand-in:\n\n{}",
        wrong.join("\n\n")
    );

    // The commit the stand-in made is excused by the line that lets it commit.
    let committed = ON_A_STAND_IN
        .iter()
        .zip(&ran)
        .find(|(on, _)| on.cell.argv == ["commits-the-index"])
        .and_then(|(_, ran)| ran.as_ref().ok())
        .expect("the cell that commits the index ran");
    assert_eq!(
        committed.verdict.excused,
        [("K1", 0), ("K2", 2)],
        "the branch that moved and the path the commit holds, both under the commit's line"
    );

    // An edit is a line added: what the file held stands above it.
    let edited = ON_A_STAND_IN
        .iter()
        .zip(&ran)
        .find(|(on, _)| on.cell.argv == ["rewrites-the-edited-file"])
        .and_then(|(_, ran)| ran.as_ref().ok())
        .expect("the cell of the edit ran");
    assert!(
        matches!(
            edited.before.files.get("repo/README.md"),
            Some(Entry::File { bytes, .. }) if bytes.starts_with(b"trial corpus\n")
        ),
        "the edited file keeps what it held",
    );

    // A committed plant is in a commit of its own, on the checked-out branch.
    let committed = ON_A_STAND_IN
        .iter()
        .zip(&ran)
        .find(|(on, _)| on.cell.argv == ["overwrites-the-committed-plant"])
        .and_then(|(_, ran)| ran.as_ref().ok())
        .expect("the cell of the committed plant ran");
    let head = &committed.before.checkouts[REPO].head;
    assert!(
        committed.before.commits.len() == 2
            && committed.before.commits[head]
                .changed
                .keys()
                .eq(["notes.txt"]),
        "the committed plant is the one path of the commit `HEAD` is: {:?}",
        committed.before.commits.get(head)
    );

    // The user's own worktree, moved away: git's registration names a directory that is
    // not there, its files stand under `moved/`, and its commit is one only its `HEAD`
    // reaches — the never-adopted repository has one commit, and the picture has two.
    let moved = ON_A_STAND_IN
        .iter()
        .zip(&ran)
        .find(|(on, _)| {
            matches!(
                on.cell.leads,
                [Lead::Plant(Plant::Worktree { moved_away: true })]
            )
        })
        .and_then(|(_, ran)| ran.as_ref().ok())
        .expect("the cell of the worktree that was moved away ran");
    let before = &moved.before;
    assert!(
        before.registrations.get("own").map(String::as_str) == Some("outside/own")
            && !before.checkouts.contains_key("outside/own")
            && before
                .files
                .contains_key("moved/own/staged-by-its-user.txt")
            && before.files.contains_key("repo/.git/worktrees/own/index")
            && before.commits.len() == 2
            && !before
                .refs
                .values()
                .any(|id| before.commits[id].parent.is_some()),
        "the worktree that was moved away, as the picture before the command has it: \
         registrations {:?}, checkouts {:?}, {} commits",
        before.registrations,
        before.checkouts.keys(),
        before.commits.len()
    );

    // A plant in the place of a file that stands: the file is gone from the picture the
    // command is held to, and the plant is where it stood — a directory, here.
    let in_place = ON_A_STAND_IN
        .iter()
        .zip(&ran)
        .find(|(on, _)| {
            matches!(
                on.cell.leads,
                [Lead::Plant(Plant::InPlaceOf(Plant::Dir(_)))]
            )
        })
        .and_then(|(_, ran)| ran.as_ref().ok())
        .expect("the cell of a directory in the file's place ran");
    assert!(
        matches!(
            in_place.before.files.get("repo/README.md"),
            Some(Entry::Dir(_))
        ) && in_place
            .before
            .files
            .contains_key("repo/README.md/a-file-of-its-users.txt"),
        "a directory stands where the committed file stood: {:?}",
        in_place.before.files.get("repo/README.md")
    );
    // A file of the user's in the place of a tracked file is what **U** says — an
    // untracked file: its bytes stand where the committed file stood, and the index holds
    // no entry at its path, so git sees a file it does not track and not an edit of one
    // it does (which is **E**'s).
    let untracked = ON_A_STAND_IN
        .iter()
        .zip(&ran)
        .find(|(on, _)| {
            matches!(
                on.cell.leads,
                [Lead::Plant(Plant::InPlaceOf(Plant::File(_)))]
            )
        })
        .and_then(|(_, ran)| ran.as_ref().ok())
        .expect("the cell of a file of the user's in the file's place ran");
    assert!(
        matches!(
            untracked.before.files.get("repo/README.md"),
            Some(Entry::File { bytes, .. })
                if String::from_utf8_lossy(bytes).contains("byte-table-plant-0-only-of-its-cell")
        ) && !untracked.before.checkouts[REPO]
            .index
            .contains_key("README.md"),
        "a file of the user's stands untracked where the committed file stood: {:?}, \
         and the index holds {:?} at its path",
        untracked.before.files.get("repo/README.md"),
        untracked.before.checkouts[REPO].index.get("README.md")
    );
    // A file simply deleted is gone from the picture the command is held to, nothing in
    // its place, and the index holds at its path what it held before the plant — the plain
    // `rm`, which git lists as a change not staged.
    let deleted = ON_A_STAND_IN
        .iter()
        .zip(&ran)
        .find(|(on, _)| {
            on.cell.argv == ["prunes-the-worktrees"]
                && matches!(on.cell.leads, [Lead::Plant(Plant::Deleted(_))])
        })
        .and_then(|(_, ran)| ran.as_ref().ok())
        .expect("the cell of a file deleted ran");
    // What the index holds at the path with nothing planted: the cell with no leads.
    let committed_entry = ON_A_STAND_IN
        .iter()
        .zip(&ran)
        .find(|(on, _)| on.cell.argv == ["refuses"] && on.cell.leads.is_empty())
        .and_then(|(_, ran)| ran.as_ref().ok())
        .and_then(|ran| ran.before.checkouts[REPO].index.get("README.md").cloned())
        .expect("the never-adopted repository's index holds `README.md`");
    assert!(
        !deleted
            .before
            .files
            .keys()
            .any(|path| path == "repo/README.md" || path.starts_with("repo/README.md/"))
            && deleted.before.checkouts[REPO].index.get("README.md") == Some(&committed_entry),
        "a file deleted is gone from the tree, and the index holds what it held: the tree has \
         {:?}, and the index {:?} where {committed_entry:?} stood",
        deleted.before.files.get("repo/README.md"),
        deleted.before.checkouts[REPO].index.get("README.md")
    );

    // A hook the user's lines are put into keeps whatever else it held.
    let hooked = ON_A_STAND_IN
        .iter()
        .zip(&ran)
        .find(|(on, _)| {
            matches!(
                on.cell.leads,
                [Lead::Step(_), Lead::Plant(Plant::RejectingHook)]
            )
        })
        .and_then(|(_, ran)| ran.as_ref().ok())
        .expect("the cell of the hook that held something ran");
    assert!(
        matches!(
            hooked.before.files.get("repo/.git/hooks/pre-commit"),
            Some(Entry::File { bytes, mode, .. })
                if bytes.starts_with(
                    b"#!/bin/sh\necho 'byte-table-plant-1-only-of-its-cell' >&2\nexit 1\n"
                ) && bytes.ends_with(b"echo what the hook held\n")
                    && mode & 0o111 != 0
        ),
        "the user's lines stand after the hook's first line, and what it held stands after them"
    );

    // The faults of a cell.
    let cell = |name, state, leads, leaf| byte_cell::Cell {
        spelling: None,
        name,
        leaf,
        state,
        leads,
        argv: &["refuses"],
        listed: Listed::Refuses,
        beside: None,
        aim: None,
        marks: &[],
        at: None,
    };
    for (cell, says) in [
        (
            cell(
                "a-lead-in-refused",
                Prepared::Bare,
                &[Lead::Step(Step::Jigc(&["refuses"]))],
                &["probe"],
            ),
            "was not done",
        ),
        (
            cell(
                "a-plant-over-a-file",
                Prepared::Bare,
                &[Lead::Plant(Plant::File("README.md"))],
                &["probe"],
            ),
            "something stands there already",
        ),
        (
            cell("another-state", Prepared::Of(State::Fresh), &[], &["probe"]),
            "the state handed in is `bare`",
        ),
        (
            cell("another-leaf", Prepared::Bare, &[], &["setup"]),
            "the row handed in is of `jigc probe`",
        ),
        (
            cell(
                "a-baseline-that-is-not-there",
                Prepared::Bare,
                &[Lead::Under(Condition::NoBaseline)],
                &["probe"],
            ),
            "holds no file-state baseline to take away",
        ),
        (
            cell(
                "an-edit-of-no-file",
                Prepared::Bare,
                &[Lead::Plant(Plant::Edit("no-such-file.md"))],
                &["probe"],
            ),
            "no regular file stands there",
        ),
        (
            cell(
                "a-lead-in-that-must-refuse-and-does-not",
                Prepared::Bare,
                &[Lead::Refused(&["status-and-a-time"])],
                &["probe"],
            ),
            "must refuse, and `jigc status-and-a-time` came to exit 0",
        ),
        (
            cell(
                "a-write-over-a-hand-edit-that-refuses-and-does-not-show-it",
                Prepared::Bare,
                &[
                    Lead::Plant(Plant::Edit("README.md")),
                    Lead::OverAHandEdit {
                        step: Step::Jigc(&["refuses"]),
                        plant: 0,
                    },
                ],
                &["probe"],
            ),
            "acts or refuses showing that edit, and `jigc refuses` came to exit 1",
        ),
        (
            cell(
                "a-write-over-a-hand-edit-that-shows-another-plant",
                Prepared::Bare,
                &[
                    Lead::Plant(Plant::Edit("README.md")),
                    Lead::OverAHandEdit {
                        step: Step::Jigc(&["refuses-showing-the-readme"]),
                        plant: 1,
                    },
                ],
                &["probe"],
            ),
            "a write over the hand edit of its lead 1, acts or refuses showing that edit",
        ),
        (
            cell(
                "a-write-over-a-hand-edit-that-is-no-step-of-the-binary",
                Prepared::Bare,
                &[
                    Lead::Plant(Plant::Edit("README.md")),
                    Lead::OverAHandEdit {
                        step: Step::Git(&["status"]),
                        plant: 0,
                    },
                ],
                &["probe"],
            ),
            "its step is not the binary under test",
        ),
        (
            cell(
                "a-directory-over-a-file",
                Prepared::Bare,
                &[Lead::Plant(Plant::Dir("README.md"))],
                &["probe"],
            ),
            "something stands there already",
        ),
        (
            cell(
                "a-plant-in-the-place-of-no-file",
                Prepared::Bare,
                &[Lead::Plant(Plant::InPlaceOf(&Plant::File(
                    "no-such-file.md",
                )))],
                &["probe"],
            ),
            "a plant goes in the place of the file at `no-such-file.md`, and no regular \
             file stands there",
        ),
        (
            cell(
                "a-plant-in-the-place-of-a-directory",
                Prepared::Bare,
                &[
                    Lead::Step(Step::Dir(Place::Repo("a-dir"))),
                    Lead::Plant(Plant::InPlaceOf(&Plant::Dir("a-dir"))),
                ],
                &["probe"],
            ),
            "a plant goes in the place of the file at `a-dir`, and no regular \
             file stands there",
        ),
        (
            cell(
                "an-edit-in-the-place-of-a-file",
                Prepared::Bare,
                &[Lead::Plant(Plant::InPlaceOf(&Plant::Edit(README)))],
                &["probe"],
            ),
            "one that makes an entry of its own",
        ),
        (
            cell(
                "a-deletion-of-no-file",
                Prepared::Bare,
                &[Lead::Plant(Plant::Deleted("no-such-file.md"))],
                &["probe"],
            ),
            "a deletion takes the file at `no-such-file.md`, and no regular file stands there",
        ),
        (
            cell(
                "a-deletion-of-a-directory",
                Prepared::Bare,
                &[
                    Lead::Step(Step::Dir(Place::Repo("a-dir"))),
                    Lead::Plant(Plant::Deleted("a-dir")),
                ],
                &["probe"],
            ),
            "a deletion takes the file at `a-dir`, and no regular file stands there",
        ),
        (
            cell(
                "a-deletion-in-the-place-of-a-file",
                Prepared::Bare,
                &[Lead::Plant(Plant::InPlaceOf(&Plant::Deleted(README)))],
                &["probe"],
            ),
            "one that makes an entry of its own",
        ),
        (
            cell(
                "a-staged-change-at-a-directory",
                Prepared::Bare,
                &[
                    Lead::Step(Step::Dir(Place::Repo("a-dir"))),
                    Lead::Plant(Plant::Staged {
                        at: "a-dir",
                        kept: false,
                    }),
                ],
                &["probe"],
            ),
            "what stands there is no regular file",
        ),
        (
            byte_cell::Cell {
                spelling: Some("absent"),
                ..cell("a-spelling-its-row-lacks", Prepared::Bare, &[], &["probe"])
            },
            "the cell names the spelling `absent`, which the row of `jigc probe` does not have",
        ),
    ] {
        let fault = byte_cell::run(&built, &NOTHING, &cell).err();
        assert!(
            fault.as_deref().is_some_and(|fault| fault.contains(says)),
            "`{}` is a fault of the cell that says `{says}`: {fault:?}",
            cell.name
        );
    }
}

/// **A command that does not end is ended, and says so.** The stand-in sleeps; the frame
/// waits for the command under test no longer than the limit the arm hands in and ends it
/// — the process itself, by its handle — and the cell comes out *the command did not end*.
#[test]
fn a_command_that_does_not_end_is_ended() {
    let scratch = ScratchDir::new("byte-table-stand-in-held");
    let built = Built::on(&the_stand_in(&scratch), Prepared::Bare);
    let held = on_a_stand_in(&["sleeps"], &[], NOTHING, Classification::DidNotEnd, "");
    let ran = byte_cell::run_within(&built, &held.row, &held.cell, Duration::from_secs(2))
        .expect("the cell is one the frame can run");
    assert_eq!(
        ran.verdict.classification,
        Classification::DidNotEnd,
        "{}",
        ran.detail
    );
    assert!(
        ran.exit.contains("did not end within 2s and was ended"),
        "{}",
        ran.exit
    );
    // The cell's own seconds, from its copy to its verdict — not the wait for a place in
    // the process's one gate, which the arms of the prepared states hold for as long as
    // their pools run where every test shares a process (`cargo test`).
    assert!(
        ran.seconds < 25.0,
        "the command was ended at the limit, and not waited out: {:.1}s",
        ran.seconds
    );
}

/// The ids of a manifest's commits, written `<commit>` in `bytes` — whole, or cut to the
/// seven characters git abbreviates one to.
fn without_commit_ids(manifest: &Manifest, bytes: &[u8]) -> String {
    let mut text = String::from_utf8_lossy(bytes).into_owned();
    for id in manifest.commits.keys() {
        text = text.replace(id.as_str(), "<commit>");
    }
    for id in manifest.commits.keys() {
        text = text.replace(&id[..7], "<commit>");
    }
    text
}

/// **A prepared state brought into the sandbox and the same state made in the sandbox
/// agree in what can agree** — `fresh` as the builder of the prepared states made it,
/// copied in, beside the never-adopted repository with one `jigc setup` run in the sandbox
/// on the same binary. The same paths, kinds, permission bits and bytes outside `.git` —
/// but for a commit's id where a file holds one; the same index, by path, mode and blob;
/// the same refs by name; the same hooks. **Not the commits' ids**: a commit carries its
/// time and its author, and the builder commits under its own identity where the sandbox
/// has the mask's. So a state built outside the frame is the state the frame's own steps
/// would have made, and a cell's lead-ins and its prepared state are one kind of thing.
#[test]
fn a_state_brought_in_and_the_state_made_in_the_sandbox_agree_in_what_can_agree() {
    let binary = Path::new(env!("CARGO_BIN_EXE_jigc"));
    let cell = |name, state, leads| byte_cell::Cell {
        spelling: None,
        name,
        leaf: &["probe"],
        state,
        leads,
        argv: &["--version"],
        listed: Listed::Acts,
        beside: None,
        aim: None,
        marks: &[],
        at: None,
    };
    let brought = byte_cell::run(
        &Built::on(binary, Prepared::Of(State::Fresh)),
        &NOTHING,
        &cell("fresh-brought-in", Prepared::Of(State::Fresh), &[]),
    )
    .expect("`fresh`, brought in");
    let made = byte_cell::run(
        &Built::on(binary, Prepared::Bare),
        &NOTHING,
        &cell(
            "bare-then-setup",
            Prepared::Bare,
            &[Lead::Step(Step::Jigc(&["setup"]))],
        ),
    )
    .expect("the never-adopted repository, then `jigc setup` in the sandbox");
    for ran in [&brought, &made] {
        assert_eq!(
            ran.verdict.classification,
            Classification::Holds,
            "`jigc --version` changes nothing: {}",
            ran.detail
        );
    }

    let (a, b) = (&brought.before, &made.before);
    let mut differ = Vec::new();
    let paths: BTreeSet<&String> = a.files.keys().chain(b.files.keys()).collect();
    for path in paths {
        let same = match (a.files.get(path), b.files.get(path)) {
            (
                Some(Entry::File { mode, bytes, .. }),
                Some(Entry::File {
                    mode: other,
                    bytes: others,
                    ..
                }),
            ) => mode == other && without_commit_ids(a, bytes) == without_commit_ids(b, others),
            (one, other) => one == other,
        };
        if !same {
            differ.push(format!("`{path}` is not the same entry in both"));
        }
    }
    let index = |manifest: &Manifest| -> Vec<(String, u32, String)> {
        manifest.checkouts[REPO]
            .index
            .iter()
            .map(|(path, staged)| (path.clone(), staged.mode, staged.blob.clone()))
            .collect()
    };
    if index(a) != index(b) {
        differ.push(format!("the index: {:#?}\nand {:#?}", index(a), index(b)));
    }
    let refs = |manifest: &Manifest| manifest.refs.keys().cloned().collect::<Vec<_>>();
    if refs(a) != refs(b) {
        differ.push(format!("the refs: {:?} and {:?}", refs(a), refs(b)));
    }
    if a.hooks != b.hooks {
        differ.push(format!(
            "the hooks directory: `{}` and `{}`",
            a.hooks, b.hooks
        ));
    }
    assert!(
        differ.is_empty(),
        "`fresh` brought in, and the never-adopted repository after `jigc setup` in the \
         sandbox:\n\n  {}",
        differ.join("\n  ")
    );
    assert!(
        a.files.len() > 10 && a.checkouts[REPO].index.len() > 3 && a.commits.len() == 2,
        "the arm compares a state that holds something: {} entries, {} index entries, {} \
         commits",
        a.files.len(),
        a.checkouts[REPO].index.len(),
        a.commits.len()
    );
}

/// **The questions of reading the pilot's cells turn on, and the bounds they stand on**, as
/// the committed list has them (`…/reshape/byte-table-readings.md`).
const READINGS: &str = include_str!(
    "../../../completions/artifacts/M55/stabilization-build/reshape/byte-table-readings.md"
);

/// A line a marked cell, as the readings file's last table holds it: the mark, and the
/// cell.
fn marked(cells: &[&byte_cell::Cell]) -> Vec<String> {
    let mut out: Vec<(Mark, &str)> = cells
        .iter()
        .flat_map(|cell| cell.marks.iter().map(|mark| (*mark, cell.name)))
        .collect();
    out.sort();
    out.into_iter()
        .map(|(mark, cell)| format!("| {} | `{cell}` |", mark.named()))
        .collect()
}

/// **The declared bounds a cell may stand on**, by their numbers in the plan's list (its
/// §1.1): the human's four of 2026-10-06 less the second, which the human retired on
/// 2026-10-19 — *"Bound 2 is retired, for tasks and sub-tasks alike"*: a `doc` write that
/// would replace a part holding an uncommitted hand edit the copy-in carried refuses at the
/// write (`DECISIONS.md`, the entry of that day). A cell that stood on it is plain red under
/// its key.
const DECLARED_BOUNDS: [u8; 3] = [1, 3, 4];

/// **Every way the marks of `cells` are not the ones a list of readings holds**: a mark
/// for a silence the record's list has no entry for, or a bound that is none of the
/// declared ones ([`DECLARED_BOUNDS`]);
/// a cell marked twice with one mark; a marked cell the list's table has no line for; a
/// line of the table for a cell that carries no such mark.
fn unmarked(cells: &[&byte_cell::Cell], silences: &[u8], readings: &str) -> Vec<String> {
    let mut out = Vec::new();
    for cell in cells {
        for (at, mark) in cell.marks.iter().enumerate() {
            let known = match mark {
                Mark::Silence(entry) => silences.contains(entry),
                Mark::Bound(number) => DECLARED_BOUNDS.contains(number),
            };
            if !known {
                out.push(format!(
                    "`{}` carries the mark *{}*, which is no entry of the record's list and \
                     none of the declared bounds",
                    cell.name,
                    mark.named()
                ));
            }
            if cell.marks[..at].contains(mark) {
                out.push(format!(
                    "`{}` carries the mark *{}* twice",
                    cell.name,
                    mark.named()
                ));
            }
        }
    }
    let lines = marked(cells);
    let listed: Vec<&str> = readings
        .lines()
        .filter(|line| line.starts_with("| ") && line.ends_with("` |"))
        .collect();
    for line in &lines {
        if !listed.contains(&line.as_str()) {
            out.push(format!(
                "the list of readings has no line `{line}` in its table of the marked cells"
            ));
        }
    }
    for line in listed {
        if !lines.iter().any(|marked| marked == line) {
            out.push(format!(
                "the list of readings has the line `{line}`, and no cell carries that mark"
            ));
        }
    }
    out
}

/// The numbers of the entries of the record's list of silences (its §6): every line that
/// opens `**<n>. `.
fn silences(record: &str) -> Vec<u8> {
    record
        .lines()
        .filter_map(|line| line.strip_prefix("**")?.split_once(". ")?.0.parse().ok())
        .collect()
}

/// **A mark names a question the list of readings has, and the list names every marked
/// cell** — a mark changes a list and a count, and never a cell's construction: when a
/// question is ruled, the mark leaves the cell and its line leaves the list, in one
/// commit. On a fixture, fault by fault; then this tree's cells against the committed
/// list.
#[test]
fn a_mark_names_a_question_the_list_of_readings_has() {
    const fn cell(name: &'static str, marks: &'static [Mark]) -> byte_cell::Cell {
        byte_cell::Cell {
            spelling: None,
            name,
            leaf: &["probe"],
            state: Prepared::Bare,
            leads: &[],
            argv: &["probe"],
            listed: Listed::Acts,
            beside: None,
            aim: None,
            marks,
            at: None,
        }
    }
    let (plain, one, two) = (
        cell("plain", &[]),
        cell("one", &[Mark::Silence(2)]),
        cell("two", &[Mark::Silence(1), Mark::Bound(1)]),
    );
    let list = "| silence 2 | `one` |\n| silence 1 | `two` |\n| bound 1 | `two` |\n";
    assert_eq!(
        marked(&[&plain, &one, &two]),
        [
            "| silence 1 | `two` |",
            "| silence 2 | `one` |",
            "| bound 1 | `two` |"
        ]
    );
    says(&unmarked(&[&plain, &one, &two], &[1, 2], list), &[]);
    // Bound 2 is no bound since the human's ruling of 2026-10-19: a mark of it is a mark of
    // nothing.
    let (lost, twice, retired) = (
        cell("lost", &[Mark::Silence(9), Mark::Bound(5)]),
        cell("twice", &[Mark::Bound(3), Mark::Bound(3)]),
        cell("retired", &[Mark::Bound(2)]),
    );
    says(
        &unmarked(&[&one, &lost, &twice, &retired], &[1, 2], list),
        &[
            (
                "`lost` carries the mark *silence 9*",
                "no entry of the record's list",
            ),
            (
                "`lost` carries the mark *bound 5*",
                "none of the declared bounds",
            ),
            ("`twice` carries the mark *bound 3* twice", ""),
            (
                "`retired` carries the mark *bound 2*",
                "none of the declared bounds",
            ),
            ("no line `| silence 9 | `lost` |`", ""),
            ("no line `| bound 2 | `retired` |`", ""),
            ("no line `| bound 3 | `twice` |`", ""),
            ("no line `| bound 3 | `twice` |`", ""),
            ("no line `| bound 5 | `lost` |`", ""),
            (
                "the line `| silence 1 | `two` |`",
                "no cell carries that mark",
            ),
            (
                "the line `| bound 1 | `two` |`",
                "no cell carries that mark",
            ),
        ],
    );
    assert_eq!(
        silences("**1. A thing.**\n**2. Another.** words\n**S1 — no**\n"),
        [1, 2]
    );

    let wrong = unmarked(&cells(), &silences(RECORD), READINGS);
    assert!(
        wrong.is_empty(),
        "the marks of the cells and the list of readings:\n\n  {}\n\na marked cell has its \
         line in the last table of `…/reshape/byte-table-readings.md`, and a line there is \
         a cell's mark",
        wrong.join("\n  ")
    );
}

// ---------------------------------------------------------------------------
// The cells — an arm a prepared state, and the expected-red list
// ---------------------------------------------------------------------------

/// Every cell of every family's file.
fn cells() -> Vec<&'static byte_cell::Cell> {
    FAMILIES
        .iter()
        .flat_map(|family| family.cells)
        .flat_map(|group| group.iter())
        .collect()
}

/// The cells the gate runs, of `cells`: every one but a base cell in a state that is not
/// its leaf's home, which runs on demand ([`on_demand`]).
fn in_the_gate(cells: &[&'static byte_cell::Cell]) -> Vec<&'static byte_cell::Cell> {
    let later = on_demand(cells);
    cells
        .iter()
        .copied()
        .filter(|cell| !later.iter().any(|other| other.name == cell.name))
        .collect()
}

/// The row a cell is compared under: its leaf's.
fn row_of(cell: &byte_cell::Cell) -> &'static Row {
    rows()
        .into_iter()
        .find(|row| row.leaf == cell.leaf)
        .unwrap_or_else(|| panic!("`{}` is of a leaf with no row", cell.name))
}

/// **The cells that are red, by the key of the finding each reproduces** — a row a key and
/// a classification, with the cells that come out so on the binary this suite was built
/// with. The keys are minted with the cells, in
/// `completions/artifacts/M55/stabilization-build/reshape/byte-table-findings.md`, in the
/// grammar the route catalog's have.
const EXPECTED_RED: &[(&str, Classification, &[&str])] = &[
    (
        "r1-setup-replaces-a-file-the-user-committed-at-a-path-it-writes-whole",
        Classification::GitMoved,
        &[
            "setup-at-the-bootstrap-a-committed-file",
            "setup-at-the-settings-record-a-committed-file",
            "setup-at-the-stamp-a-committed-file",
        ],
    ),
    (
        "r1-setup-commits-the-file-a-root-ignore-link-leads-to-and-the-link-on-an-unborn-head",
        Classification::GitMoved,
        &["setup-head-unborn-at-the-root-ignore-a-link-to-a-file-in-the-repository"],
    ),
    (
        "r1-setup-replaces-a-change-staged-at-a-path-it-writes-whole",
        Classification::Lost,
        &[
            "setup-at-the-guide-a-change-staged-and-not-in-the-tree",
            "setup-at-the-stamp-a-change-staged-and-not-in-the-tree",
        ],
    ),
    (
        "r1-setup-refuses-after-its-first-write-and-leaves-the-install-written",
        Classification::RefusedAndChanged,
        &[
            "setup-a-file-where-the-hooks-directory-goes",
            "setup-at-the-bootstrap-a-change-staged-and-not-in-the-tree",
            "setup-at-the-hook-a-directory",
            "setup-at-the-hook-a-link-to-a-file-in-the-repository",
            "setup-at-the-host-file-a-change-staged-and-not-in-the-tree",
            "setup-at-the-host-file-a-directory",
            "setup-at-the-settings-a-change-staged-and-not-in-the-tree",
            "setup-at-the-settings-record-a-change-staged-and-not-in-the-tree",
            "setup-at-the-workbench-ignore-a-change-staged-and-not-in-the-tree",
            "setup-at-the-workbench-ignore-a-directory",
            "setup-head-unborn-at-the-root-ignore-a-change-staged-and-not-in-the-tree",
            "setup-head-unborn-at-the-root-ignore-a-directory",
            "setup-head-unborn-the-staged-root-ignore-deleted",
        ],
    ),
    (
        "r1-setup-runs-no-hook-at-its-install-commit",
        Classification::NotTheListedOutcome,
        &["setup-a-hook-that-rejects"],
    ),
    (
        "r1-uninstall-leaves-its-teardown-in-the-working-tree-and-commits-nothing",
        Classification::WrittenOutside,
        &[
            "uninstall-a-change-staged-at-the-host-file-and-not-in-the-file",
            "uninstall-a-change-staged-at-the-settings-file-and-not-in-the-file",
            "uninstall-a-file-staged-elsewhere",
            "uninstall-a-file-staged-under-a-name-git-quotes",
            "uninstall-a-foreign-file-under-logs-by-consent",
            "uninstall-a-hook-that-rejects",
            "uninstall-a-line-of-the-users-in-the-host-file",
            "uninstall-after-a-task-landed",
            "uninstall-an-edit-to-the-host-file",
            "uninstall-run-as-typed",
            "uninstall-the-host-file-deleted",
            "uninstall-the-settings-file-deleted",
            "uninstall-the-users-own-lines-in-the-hook",
            "uninstall-the-users-own-worktree",
            "uninstall-the-users-own-worktree-moved-away",
        ],
    ),
    (
        "r1-uninstall-refuses-over-a-settings-file-it-cannot-parse-after-removing-the-workbench",
        Classification::RefusedAndChanged,
        &[
            "uninstall-a-line-of-the-users-in-the-settings-file",
            "uninstall-an-edit-to-the-settings-file",
        ],
    ),
    (
        "r1-task-finalize-a-change-staged-at-an-updated-doc-and-not-in-the-file-is-replaced",
        Classification::Lost,
        &["task-finalize-a-change-staged-at-the-updated-doc-and-not-in-the-file"],
    ),
    (
        "r1-task-finalize-a-hand-edit-before-the-tasks-first-write-is-replaced-by-its-write-to-the-same-part",
        Classification::Lost,
        &["task-finalize-a-hand-edit-before-the-first-write-to-the-same-part"],
    ),
    // The human's ruling of 2026-10-19, which retired their bound 2, at its own door (after
    // the review of the task `B8`, H6): the write to the part a hand edit stands in acts,
    // where the ruling has it refuse showing the edit — one mechanism with the key above
    // and the boundary's, one fix closes the three.
    (
        "r1-doc-write-acts-over-a-hand-edit-in-the-part-it-writes",
        Classification::NotTheListedOutcome,
        &["doc-set-the-visions-open-questions-a-hand-edit-to-the-same-part-before-the-first-write"],
    ),
    (
        "r1-task-finalize-a-change-staged-at-the-version-stamp-and-not-in-the-file-is-replaced",
        Classification::Lost,
        &["task-finalize-a-change-staged-at-the-version-stamp-and-not-in-the-file"],
    ),
    (
        "r1-task-discard-a-change-staged-at-the-record-and-not-in-the-file-is-replaced",
        Classification::Lost,
        &["task-discard-a-change-staged-at-the-record-and-not-in-the-file"],
    ),
    (
        "r1-task-discard-settles-no-record-and-discards-a-sub-task-whose-record-is-not-on-disk",
        Classification::NotTheListedOutcome,
        &[
            "task-discard-a-file-in-the-place-of-the-records-directory",
            "task-discard-the-milestones-record-deleted",
        ],
    ),
    (
        "r1-doc-write-acts-over-a-home-that-is-no-doc-of-its-type",
        Classification::NotTheListedOutcome,
        &[
            "doc-add-a-milestone-a-file-of-the-users-in-the-docs-place",
            "doc-add-a-milestone-a-link-to-a-file-in-the-repository-in-the-docs-place",
            "doc-add-a-milestone-a-link-to-a-file-outside-in-the-docs-place",
            "doc-create-the-vision-a-file-of-the-users-in-the-docs-place",
            "doc-create-the-vision-a-link-to-a-file-in-the-repository-in-the-docs-place",
            "doc-create-the-vision-a-link-to-a-file-outside-in-the-docs-place",
        ],
    ),
    (
        "r1-doc-author-refuses-over-a-home-that-is-no-doc-of-its-type-and-leaves-its-role-bound",
        Classification::RefusedAndChanged,
        &[
            "doc-author-the-vision-a-file-of-the-users-in-the-docs-place",
            "doc-author-the-vision-a-link-to-a-file-in-the-repository-in-the-docs-place",
            "doc-author-the-vision-a-link-to-a-file-outside-in-the-docs-place",
        ],
    ),
    (
        "r1-doc-write-refuses-over-a-home-that-is-no-doc-of-its-type-and-leaves-its-copy-staged",
        Classification::RefusedAndChanged,
        &[
            "doc-remove-a-milestone-a-file-of-the-users-in-the-docs-place",
            "doc-remove-a-milestone-a-link-to-a-file-in-the-repository-in-the-docs-place",
            "doc-remove-a-milestone-a-link-to-a-file-outside-in-the-docs-place",
            "doc-retitle-a-committed-decision-a-file-of-the-users-in-the-docs-place",
            "doc-retitle-a-committed-decision-a-link-to-a-file-in-the-repository-in-the-docs-place",
            "doc-retitle-a-committed-decision-a-link-to-a-file-outside-in-the-docs-place",
            "doc-retitle-a-milestone-a-file-of-the-users-in-the-docs-place",
            "doc-retitle-a-milestone-a-link-to-a-file-in-the-repository-in-the-docs-place",
            "doc-retitle-a-milestone-a-link-to-a-file-outside-in-the-docs-place",
            "doc-set-a-releases-link-a-file-of-the-users-in-the-docs-place",
            "doc-set-a-releases-link-a-link-to-a-file-in-the-repository-in-the-docs-place",
            "doc-set-a-releases-link-a-link-to-a-file-outside-in-the-docs-place",
            "doc-set-what-a-milestone-proves-a-file-of-the-users-in-the-docs-place",
            "doc-set-what-a-milestone-proves-a-link-to-a-file-in-the-repository-in-the-docs-place",
            "doc-set-what-a-milestone-proves-a-link-to-a-file-outside-in-the-docs-place",
        ],
    ),
    (
        "r1-task-discard-a-rejected-record-commit-leaves-its-message-file-in-the-working-area",
        Classification::RefusedAndChanged,
        &["task-discard-a-hook-that-rejects"],
    ),
    (
        "r1-milestone-record-doors-leave-their-message-file-in-the-milestones-area",
        Classification::RefusedAndChanged,
        &[
            "milestone-add-task-a-hook-that-rejects",
            "milestone-add-from-spec-a-hook-that-rejects",
            "milestone-discard-a-hook-that-rejects",
        ],
    ),
    // The same key at exit 0, since the seventh fix after the review of the task `B7`'s
    // second part: the rendered message's file stands under no line
    // (`byte_cell::DELETED_AFTER`), and every record door that lands leaves it in the
    // milestone's area.
    (
        "r1-milestone-record-doors-leave-their-message-file-in-the-milestones-area",
        Classification::WrittenOutside,
        &[
            "milestone-create-a-milestone-the-catalog-names",
            "milestone-create-ship-it",
            "milestone-create-a-file-staged-elsewhere",
            "milestone-create-a-staged-file-whose-name-git-quotes",
            "milestone-create-a-file-of-the-users-beside-the-baseline",
            "milestone-add-task-do-the-thing",
            "milestone-add-task-a-file-of-the-users-in-the-milestones-area",
            "milestone-add-task-a-file-staged-elsewhere",
            "milestone-add-task-a-staged-file-whose-name-git-quotes",
            "milestone-add-task-a-file-of-the-users-beside-the-baseline",
            "milestone-add-task-a-file-of-the-users-in-a-first-sub-tasks-area",
            "milestone-add-from-spec-padding",
            "milestone-add-from-spec-a-spec-of-two-criteria",
            "milestone-add-from-spec-a-file-of-the-users-in-the-milestones-area",
            "milestone-add-from-spec-a-file-staged-elsewhere",
            "milestone-add-from-spec-a-staged-file-whose-name-git-quotes",
            "milestone-add-from-spec-a-file-of-the-users-beside-the-baseline",
            "milestone-add-from-spec-a-file-of-the-users-in-a-first-sub-tasks-area",
        ],
    ),
    // A refusal of the boundary that leaves the directory its worktree provisioning made —
    // the seventh fix's cell of a refusal after the re-seed met it.
    (
        "r1-milestone-finalize-refuses-and-leaves-the-worktrees-directory-it-made",
        Classification::RefusedAndChanged,
        &["milestone-finalize-a-sub-tasks-area-gone-and-no-work-to-land"],
    ),
    (
        "r1-milestone-record-doors-refuse-and-leave-the-directories-they-made",
        Classification::RefusedAndChanged,
        &[
            "milestone-create-a-file-in-the-place-of-the-records-directory",
            "milestone-create-a-hook-that-rejects",
            "milestone-add-task-a-file-in-the-place-of-the-records-directory",
            "milestone-add-task-the-record-deleted",
        ],
    ),
    (
        "r1-milestone-record-doors-replace-a-change-staged-at-the-record",
        Classification::Lost,
        &[
            "milestone-add-task-a-change-staged-at-the-record",
            "milestone-add-from-spec-a-change-staged-at-the-record",
            "milestone-discard-a-change-staged-at-the-record",
            "milestone-finalize-a-change-staged-at-the-record",
        ],
    ),
    // What the human's bound 2 of 2026-10-06 reached until they retired it on 2026-10-19, at
    // the boundary: one mechanism with the task family's key above, the copy-in a sub-task's
    // first write makes from the main checkout's doc-store; the cell carries no mark.
    (
        "r1-milestone-finalize-a-hand-edit-before-a-sub-tasks-first-write-is-replaced-by-its-write-to-the-same-part",
        Classification::Lost,
        &["milestone-finalize-a-hand-edit-before-a-sub-tasks-first-write-to-the-same-part"],
    ),
    (
        "r1-milestone-finalize-takes-a-file-nobody-staged-from-a-landed-sub-tasks-worktree",
        Classification::Lost,
        &["milestone-finalize-a-file-of-the-users-in-a-sub-tasks-worktree"],
    ),
    // The rows' record's §6, entry 19 — one mechanism at two doors, the stage's `git add
    // -- .jigc/config .jigc/.gitignore`; ruled by the human on 2026-10-19, a finalize never
    // carries a change under the layer that nobody staged, so the cells carry no mark.
    (
        "r1-task-finalize-commits-a-change-under-the-config-layer-nobody-staged",
        Classification::GitMoved,
        &["task-finalize-a-pending-config-delta"],
    ),
    (
        "r1-milestone-finalize-commits-a-change-under-the-config-layer-nobody-staged",
        Classification::GitMoved,
        &["milestone-finalize-a-pending-config-delta"],
    ),
    // The movers family's doors that commit (the task `B8`, its third commit): `rename`'s
    // cells are none of them red; `migrate-corpus`'s four keys.
    (
        "r1-migrate-corpus-reads-past-a-link-or-a-directory-at-a-stale-docs-home-and-refuses-nothing",
        Classification::NotTheListedOutcome,
        &[
            "migrate-corpus-a-link-to-nothing-in-the-decisions-place",
            "migrate-corpus-a-link-to-a-file-in-the-repository-in-the-decisions-place",
            "migrate-corpus-a-link-to-a-file-outside-in-the-decisions-place",
            "migrate-corpus-a-directory-in-the-decisions-place",
            "migrate-corpus-a-link-to-nothing-in-the-old-changelogs-place",
            "migrate-corpus-a-link-to-a-file-in-the-repository-in-the-old-changelogs-place",
            "migrate-corpus-a-link-to-a-file-outside-in-the-old-changelogs-place",
            "migrate-corpus-a-directory-in-the-old-changelogs-place",
            "migrate-corpus-dry-run-a-link-to-nothing-in-the-decisions-place",
            "migrate-corpus-dry-run-a-link-to-a-file-in-the-repository-in-the-decisions-place",
            "migrate-corpus-dry-run-a-link-to-a-file-outside-in-the-decisions-place",
            "migrate-corpus-dry-run-a-directory-in-the-decisions-place",
        ],
    ),
    // The review's `(R4, F5)`, OPEN in the fix pass's ledger: the cells that hold it.
    (
        "r1-migrate-corpus-commits-an-uncommitted-edit-at-a-doc-it-migrates",
        Classification::GitMoved,
        &[
            "migrate-corpus-an-edit-to-the-decision",
            "migrate-corpus-an-edit-to-the-old-changelog",
        ],
    ),
    (
        "r1-migrate-corpus-replaces-a-change-staged-at-a-doc-it-migrates",
        Classification::Lost,
        &[
            "migrate-corpus-a-change-staged-at-the-decision",
            "migrate-corpus-a-change-staged-at-the-old-changelog",
        ],
    ),
    // The rows' record's §6, entry 22, ruled by the human on 2026-10-19: the commit a hook
    // rejects rolls back — a finding of the product.
    (
        "r1-migrate-corpus-refuses-at-its-commit-and-leaves-the-migration-written-and-staged",
        Classification::RefusedAndChanged,
        &["migrate-corpus-a-hook-that-rejects"],
    ),
    // The movers (the task `B8`, its fourth commit): `relocate`'s keys and `config set`'s.
    // `relocate`'s exit is the human's ruling R1 of 2026-10-20 — an acting sweep exits
    // non-zero where an act it was asked for did not happen — and the second note's cell
    // holds its *does all it can* (the task `B9`'s first commit).
    (
        "r1-relocate-refuses-a-doc-and-exits-0",
        Classification::NotTheListedOutcome,
        &[
            "relocate-a-link-to-nothing-in-the-stranded-notes-place",
            "relocate-a-link-to-a-file-in-the-repository-in-the-stranded-notes-place",
            "relocate-a-link-to-a-file-outside-in-the-stranded-notes-place",
            "relocate-a-directory-in-the-stranded-notes-place",
            "relocate-a-note-a-task-landed-at-the-notes-new-home",
            "relocate-a-link-to-nothing-in-one-stranded-notes-place-and-a-second-note",
        ],
    ),
    // The rows' record's §6, entry 23, ruled by the human on 2026-10-20: any file at a
    // mover's destination refuses — the committed one the entry asked of, and every other
    // occupant, a key of its own: one mechanism, one fix closes both.
    (
        "r1-relocate-parks-a-file-the-user-committed-at-a-new-home",
        Classification::GitMoved,
        &["relocate-a-file-the-user-committed-at-the-notes-new-home"],
    ),
    (
        "r1-relocate-parks-an-untracked-occupant-of-a-new-home",
        Classification::GitMoved,
        &[
            "relocate-a-file-of-the-users-at-the-notes-new-home",
            "relocate-a-link-to-nothing-at-the-notes-new-home",
            "relocate-a-link-to-a-file-in-the-repository-at-the-notes-new-home",
            "relocate-a-link-to-a-file-outside-at-the-notes-new-home",
            "relocate-a-directory-at-the-notes-new-home",
            "relocate-a-file-of-the-users-at-the-notes-new-home-and-a-second-note",
        ],
    ),
    (
        "r1-config-set-drops-a-comment-of-the-users-from-the-manifest",
        Classification::WrittenOutside,
        &["config-set-over-a-manifest-that-holds-the-users-comment"],
    ),
    (
        "r1-config-set-replaces-a-manifest-it-cannot-read-as-knobs",
        Classification::Lost,
        &[
            "config-set-a-file-of-the-users-in-the-manifests-place",
            "config-set-a-line-of-the-users-in-the-manifest",
        ],
    ),
    (
        "r1-config-set-writes-the-knob-through-a-link-at-the-manifest",
        Classification::Lost,
        &[
            "config-set-a-link-to-a-file-in-the-repository-in-the-manifests-place",
            "config-set-a-link-to-a-file-outside-in-the-manifests-place",
        ],
    ),
    // The same mechanism through a link to nothing: what it writes lands at the link's
    // target, a file of nobody's — no byte lost, and the re-point's moves under a knob the
    // frame cannot read through the link.
    (
        "r1-config-set-writes-the-knob-through-a-link-at-the-manifest",
        Classification::GitMoved,
        &["config-set-a-link-to-nothing-in-the-manifests-place"],
    ),
    (
        "r1-config-set-re-points-past-a-link-to-nothing-or-a-directory-at-a-doc-it-strands",
        Classification::NotTheListedOutcome,
        &[
            "config-set-a-link-to-nothing-in-the-decisions-place",
            "config-set-a-directory-in-the-decisions-place",
            // A file in the place of the decisions' directory strands the decision the same
            // way: the fifth of the fixes after the review of `B8`, its left-open; relisted
            // by the eighth, as the `placement-root` twin is.
            "config-set-a-file-in-the-place-of-the-decisions-directory",
        ],
    ),
    (
        "r1-config-set-refuses-and-leaves-the-directories-its-re-point-made",
        Classification::RefusedAndChanged,
        &[
            "config-set-a-link-to-a-file-in-the-repository-in-the-decisions-place",
            "config-set-a-link-to-a-file-outside-in-the-decisions-place",
        ],
    ),
    // The other root knob, `placement-root` (after the review of the task `B8`, H5): the
    // rows' record's §6, entry 23, ruled on 2026-10-20, at its second door — where
    // `docs-root`'s refusal of every occupant is what the ruling asks, its five cells
    // holding since.
    (
        "r1-config-set-parks-a-file-the-user-committed-at-a-new-home",
        Classification::GitMoved,
        &["config-set-placement-root-a-file-the-user-committed-at-the-roadmaps-new-home"],
    ),
    (
        "r1-config-set-parks-an-untracked-occupant-of-a-new-home",
        Classification::GitMoved,
        &[
            "config-set-placement-root-a-file-of-the-users-at-the-roadmaps-new-home",
            "config-set-placement-root-a-link-to-nothing-at-the-roadmaps-new-home",
            "config-set-placement-root-a-link-to-a-file-in-the-repository-at-the-roadmaps-new-home",
            "config-set-placement-root-a-link-to-a-file-outside-at-the-roadmaps-new-home",
            "config-set-placement-root-a-directory-at-the-roadmaps-new-home",
        ],
    ),
    // The mint family (the task `B9`, its third commit): a re-entry over a file of the
    // user's where the sub-task's commit doc is provisioned.
    (
        "r1-workflow-re-entry-takes-a-file-of-the-users-as-the-sub-tasks-commit-doc",
        Classification::NotTheListedOutcome,
        &["workflow-a-file-of-the-users-where-the-commit-doc-is-provisioned"],
    ),
];

/// One entry of the expected-red list.
struct ExpectedRed {
    cell: &'static str,
    key: &'static str,
    classification: Classification,
}

/// [`EXPECTED_RED`], an entry a cell.
fn expected_red() -> Vec<ExpectedRed> {
    EXPECTED_RED
        .iter()
        .flat_map(|(key, classification, cells)| {
            cells.iter().map(|cell| ExpectedRed {
                cell,
                key,
                classification: *classification,
            })
        })
        .collect()
}

/// **Every way what the cells came to and the expected-red list disagree.** The list is
/// exact three ways — a red cell no entry lists, a listed cell that is not red, a listed
/// cell whose classification moved — and its own faults are named with them: an entry for
/// a cell that was not run, a cell listed twice, an entry that expects a classification
/// that is not red, a key that is not in the grammar the findings' keys have.
fn differences(expected: &[ExpectedRed], results: &[(&str, Classification)]) -> Vec<String> {
    let mut out = Vec::new();
    for (cell, came) in results {
        let listed: Vec<&ExpectedRed> = expected
            .iter()
            .filter(|entry| entry.cell == *cell)
            .collect();
        match (listed.first(), came.is_red()) {
            (None, true) => out.push(format!(
                "`{cell}` came out *{came}*, and no entry of the expected-red list names it"
            )),
            (Some(entry), false) => out.push(format!(
                "`{cell}` is listed under `{}` as *{}*, and came out *{came}*, which is not red",
                entry.key, entry.classification
            )),
            (Some(entry), true) if entry.classification != *came => out.push(format!(
                "`{cell}` is listed under `{}` as *{}*, and came out *{came}*",
                entry.key, entry.classification
            )),
            _ => {}
        }
        if listed.len() > 1 {
            out.push(format!("`{cell}` is listed twice"));
        }
    }
    for entry in expected {
        if !results.iter().any(|(cell, _)| *cell == entry.cell) {
            out.push(format!(
                "`{}` is listed under `{}`, and names a cell that was not run",
                entry.cell, entry.key
            ));
        }
        if !entry.classification.is_red() {
            out.push(format!(
                "`{}` is listed as *{}*, which is not red",
                entry.cell, entry.classification
            ));
        }
        let in_the_grammar = entry.key.strip_prefix("r1-").is_some_and(|words| {
            !words.is_empty()
                && words.split('-').all(|word| {
                    !word.is_empty()
                        && word
                            .bytes()
                            .all(|byte| byte.is_ascii_lowercase() || byte.is_ascii_digit())
                })
        });
        if !in_the_grammar {
            out.push(format!(
                "`{}` is listed under `{}`, which is no key: `r1-`, then lowercase words joined by `-`",
                entry.cell, entry.key
            ));
        }
    }
    out
}

/// **The expected-red list is exact three ways, and its own faults are named** — on
/// planted lists and planted results, fault by fault.
#[test]
fn the_expected_red_list_is_exact_three_ways() {
    let entry = |cell, key, classification| ExpectedRed {
        cell,
        key,
        classification,
    };
    let results = [
        ("loses", Classification::Lost),
        ("holds", Classification::Holds),
        ("refuses", Classification::RefusesNothingChanged),
    ];
    says(
        &differences(
            &[entry("loses", "r1-setup-loses", Classification::Lost)],
            &results,
        ),
        &[],
    );
    says(
        &differences(&[], &results),
        &[(
            "`loses` came out *lost*",
            "no entry of the expected-red list names it",
        )],
    );
    says(
        &differences(
            &[
                entry("loses", "r1-setup-loses", Classification::GitMoved),
                entry("holds", "r1-setup-holds", Classification::WrittenOutside),
            ],
            &results,
        ),
        &[
            (
                "`loses` is listed under `r1-setup-loses`",
                "and came out *lost*",
            ),
            (
                "`holds` is listed under `r1-setup-holds`",
                "which is not red",
            ),
        ],
    );
    says(
        &differences(
            &[
                entry("loses", "r1-setup-loses", Classification::Lost),
                entry("loses", "r1-setup-loses", Classification::Lost),
                entry("gone", "r1-setup-gone", Classification::Lost),
                entry(
                    "refuses",
                    "r1-setup-refuses",
                    Classification::RefusesNothingChanged,
                ),
                entry("loses", "setup loses", Classification::Lost),
            ],
            &results,
        ),
        &[
            ("`loses`", "is listed twice"),
            (
                "`refuses` is listed under `r1-setup-refuses`",
                "which is not red",
            ),
            ("`gone`", "names a cell that was not run"),
            (
                "`refuses` is listed as *refuses, nothing changed*",
                "which is not red",
            ),
            ("`loses` is listed under `setup loses`", "which is no key"),
        ],
    );
}

fn binary() -> &'static Path {
    Path::new(env!("CARGO_BIN_EXE_jigc"))
}

/// **Run the cells of one prepared state on the binary this suite was built with, and
/// hold the expected-red list to exactly the ones that came out red.** The state is built
/// once, on that binary, and every cell runs on a copy of it; every difference of the
/// state's cells is named at once, with what each cell did and **how many differences
/// each line of its row excused** — which the arm prints for every cell whether or not
/// anything differs.
fn run_the_cells_of(state: Prepared) {
    let cells: Vec<&byte_cell::Cell> = in_the_gate(&cells())
        .into_iter()
        .filter(|cell| cell.state == state)
        .collect();
    assert!(
        !cells.is_empty(),
        "`{}` has an arm and no cell",
        state.name()
    );
    let started = std::time::Instant::now();
    let built = Built::on(binary(), state);
    let built_in = started.elapsed().as_secs_f64();
    let ran = route_case::pooled(route_case::width(), &cells, |cell| {
        byte_cell::run(&built, row_of(cell), cell)
    });
    let expected: Vec<ExpectedRed> = expected_red()
        .into_iter()
        .filter(|entry| cells.iter().any(|cell| cell.name == entry.cell))
        .collect();

    let (mut wrong, mut results, mut seen) = (Vec::new(), Vec::new(), Vec::new());
    for (cell, ran) in cells.iter().zip(&ran) {
        match ran {
            Ok(ran) => {
                results.push((cell.name, ran.verdict.classification));
                seen.push(format!(
                    "`{}` — {}{}\n{}\nexcused, line by line: {}",
                    cell.name,
                    ran.verdict.classification,
                    marks(cell),
                    ran.detail.trim_end(),
                    excused(&ran.verdict),
                ));
            }
            Err(fault) => wrong.push(format!("`{}` cannot be run as written: {fault}", cell.name)),
        }
    }
    wrong.extend(differences(&expected, &results));
    let excused_by: Vec<(&[&str], Excused<'_>)> = cells
        .iter()
        .zip(&ran)
        .filter_map(|(cell, ran)| Some((cell.leaf, ran.as_ref().ok()?.verdict.excused.as_slice())))
        .collect();
    wrong.extend(excused_otherwise(state, &rows(), &excused_by, EXCUSED_IN));
    assert!(
        wrong.is_empty(),
        "the cells of `{}`, the expected-red list and what each line is listed as excusing \
         disagree:\n\n  {}\n\nwhat each cell did:\n\n{}",
        state.name(),
        wrong.join("\n  "),
        seen.join("\n\n"),
    );
    // What each cell did, for a reader of the run — a harness shows it where it is asked to.
    println!("{}", seen.join("\n\n"));
    // And what each cost: the gate's runner keeps a passing test's output to itself.
    let seconds: String = cells
        .iter()
        .zip(&ran)
        .filter_map(|(cell, ran)| {
            Some(format!(
                "{}\t{:.1}\n",
                cell.name,
                ran.as_ref().ok()?.seconds
            ))
        })
        .collect();
    let _ = fs::write(
        Path::new(SECONDS).join(format!("byte-table-seconds-{}.tsv", state.name())),
        format!("cell\tseconds\nthe state, built\t{built_in:.1}\n{seconds}"),
    );
}

/// **Where an arm leaves its cells' seconds**: cargo's directory for what a test of this
/// target writes, `<target>/tmp` — a file a prepared state, `byte-table-seconds-<state>.tsv`,
/// written over by every run and read by nothing here. A cell's seconds inside a gate are
/// what the pilot's record asks for, and the gate's runner shows a passing test's output
/// to nobody.
const SECONDS: &str = env!("CARGO_TARGET_TMPDIR");

/// A cell's marks, as a line of what it did says them — nothing for a cell with none.
fn marks(cell: &byte_cell::Cell) -> String {
    if cell.marks.is_empty() {
        return String::new();
    }
    let named: Vec<String> = cell.marks.iter().map(|mark| mark.named()).collect();
    format!(" — marked: {}", named.join(" · "))
}

/// What a cell is listed as, in a table's words.
fn listed_as(cell: &byte_cell::Cell) -> String {
    match cell.listed {
        Listed::Acts => "acts".to_string(),
        Listed::Refuses => "refuses".to_string(),
        Listed::RefusesWith(refusal) => {
            let moved: Vec<String> = refusal
                .moved
                .iter()
                .map(|(from, to)| format!(", having moved `{from}` to `{to}`"))
                .collect();
            format!(
                "refuses: exit {}, `{}`{}",
                refusal.exit,
                refusal.says,
                moved.concat()
            )
        }
    }
}

/// What each line of a row excused, the lines that excused nothing left out.
fn excused(verdict: &byte_cell::Verdict) -> String {
    let counted: Vec<String> = verdict
        .excused
        .iter()
        .filter(|(_, times)| *times > 0)
        .map(|(line, times)| format!("{line} × {times}"))
        .collect();
    if counted.is_empty() {
        "nothing".to_string()
    } else {
        counted.join(" · ")
    }
}

/// **A prepared state gets its arm by its line inside this macro's one call**: the same
/// line is the state's row of [`ARMS`] and the test that runs its cells, so a state cannot
/// be given cells' arm by half.
macro_rules! arms {
    ($($name:ident => $state:expr,)*) => {
        /// The prepared states whose cells an arm runs, each with its arm's name.
        const ARMS: &[(&str, Prepared)] = &[$((stringify!($name), $state)),*];

        /// **The cells of a prepared state are run on the binary this suite was built
        /// with, and the expected-red list is exactly the ones that came out red** — one
        /// arm a state ([`run_the_cells_of`]).
        mod the_cells_are_run_and_the_expected_red_list_is_exact {
            use super::*;
            $(
                #[test]
                fn $name() {
                    run_the_cells_of($state);
                }
            )*
        }
    };
}

/// The module `arms!` puts the arms in, as the harness names it inside this suite.
const RUN: &str = "the_cells_are_run_and_the_expected_red_list_is_exact";

arms! {
    bare => Prepared::Bare,
    fresh => Prepared::Of(State::Fresh),
    committed_singletons => Prepared::Of(State::CommittedSingletons),
    vendored => Prepared::Of(State::Vendored),
}

/// The suite's own path inside the test binary, as the harness names its tests.
fn suite() -> &'static str {
    module_path!()
        .split_once("::")
        .map_or(module_path!(), |(_, suite)| suite)
}

/// Where the suites stand, for the arm that holds a cell's pointer to one.
const SUITES: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/tests");

/// **Every cell has its row, the arm of its state, a name of its own, and — where it
/// names a suite that holds the same state — a test that is there.** A cell of a leaf
/// with no row would be compared under nothing; a cell in a state no arm runs would be
/// run by nobody, at a green gate; and a pointer to a test that was renamed says a state
/// is held where nobody can find it. The expected-red list's own faults are held here
/// too, against every cell's name: an entry for a cell of no arm would be held by no arm.
/// And the harness is asked that each arm is a test it lists.
#[test]
fn every_cell_has_its_row_its_arm_and_a_name_of_its_own() {
    let cells = cells();
    let mut wrong = Vec::new();
    for (at, cell) in cells.iter().enumerate() {
        if !rows().iter().any(|row| row.leaf == cell.leaf) {
            wrong.push(format!(
                "`{}` is of `jigc {}`, which has no row",
                cell.name,
                cell.leaf.join(" ")
            ));
        }
        if !cell.argv.starts_with(cell.leaf) {
            wrong.push(format!(
                "`{}` runs `jigc {}`, which is not its leaf's command",
                cell.name,
                cell.argv.join(" ")
            ));
        }
        let runs_on_demand = on_demand(&cells)
            .iter()
            .any(|other| other.name == cell.name);
        if !runs_on_demand && !ARMS.iter().any(|(_, state)| *state == cell.state) {
            wrong.push(format!(
                "`{}` stands in `{}`, whose cells no arm runs",
                cell.name,
                cell.state.name()
            ));
        }
        if cells[..at].iter().any(|other| other.name == cell.name) {
            wrong.push(format!("the name `{}` is two cells'", cell.name));
        }
        if cell.listed == Listed::Refuses {
            wrong.push(format!(
                "`{}` is listed as refusing, and does not name its refusal",
                cell.name
            ));
        }
        // A write over a hand edit names the plant it writes over, and a refusal listed by a
        // plant's bytes names a plant of the cell: an index that moved would hold a cell to
        // another lead's bytes.
        let is_an_edit =
            |plant: usize| matches!(cell.leads.get(plant), Some(Lead::Plant(Plant::Edit(_))));
        for (lead_at, lead) in cell.leads.iter().enumerate() {
            if let Lead::OverAHandEdit { plant, .. } = lead
                && !(*plant < lead_at && is_an_edit(*plant))
            {
                wrong.push(format!(
                    "`{}`'s lead {lead_at} writes over the hand edit of its lead {plant}, which \
                     is no hand edit before it",
                    cell.name
                ));
            }
        }
        if let Listed::RefusesWith(refusal) = cell.listed
            && let Some(rest) = refusal.says.strip_prefix("byte-table-plant-")
        {
            let named = rest
                .strip_suffix("-only-of-its-cell")
                .and_then(|plant| plant.parse::<usize>().ok());
            if !named.is_some_and(is_an_edit) {
                wrong.push(format!(
                    "`{}` is listed as refusing with the bytes `{}`, which are no hand edit's \
                     of the cell",
                    cell.name, refusal.says
                ));
            }
        }
        if let Some(beside) = cell.beside {
            let held = beside.split_once("::").is_some_and(|(suite, test)| {
                fs::read_to_string(format!("{SUITES}/{suite}.rs"))
                    .is_ok_and(|source| source.contains(&format!("\nfn {test}() {{")))
            });
            if !held {
                wrong.push(format!(
                    "`{}` names `{beside}` as the test that holds its state, and no suite \
                     has that test",
                    cell.name
                ));
            }
        }
    }
    let as_listed: Vec<(&str, Classification)> = cells
        .iter()
        .map(|cell| {
            let listed = expected_red()
                .into_iter()
                .find(|entry| entry.cell == cell.name && entry.classification.is_red());
            (
                cell.name,
                listed.map_or(Classification::Holds, |entry| entry.classification),
            )
        })
        .collect();
    wrong.extend(differences(&expected_red(), &as_listed));
    let listed = route_case::harness_tests();
    for (arm, _) in ARMS {
        let test = format!("{}::{RUN}::{arm}", suite());
        if !listed.contains(&test) {
            wrong.push(format!("the harness lists no test `{test}`"));
        }
    }
    let on_demand_arm = format!("{}::{ON_DEMAND_ARM}", suite());
    if !listed.contains(&on_demand_arm) {
        wrong.push(format!("the harness lists no test `{on_demand_arm}`"));
    }
    assert!(
        wrong.is_empty(),
        "the cells of the byte table:\n\n  {}",
        wrong.join("\n  ")
    );
    assert_eq!(
        (
            cells.len(),
            in_the_gate(&cells).len(),
            on_demand(&cells).len()
        ),
        (739, 559, 180),
        "the pilot's acceptance is twelve cells (the plan's §5), the task family's are \
         seventy-one — fifty-nine in the gate, four of them its spelling `doc-only`, twelve \
         on demand — `milestone finalize`'s spelling `squash: false` four, in the gate — the \
         install family's are a \
         hundred and thirty-one: a hundred and seven in the gate, twenty-four on demand — the \
         doc family's a hundred and thirty-nine: ninety-one in the gate, forty-eight on \
         demand — and the milestone family's record doors seventy: fifty-two in the gate, \
         eighteen on demand — its fan-out doors thirty-six: eighteen in the gate, eighteen on \
         demand — and its terminals sixty-nine, `discard`'s twenty-eight and `milestone \
         finalize`'s plain spelling's forty-one: fifty-seven in the gate, twelve on demand — and \
         the movers family's doors that commit seventy-six, `rename`'s thirty-seven and \
         `migrate-corpus`'s thirty-nine: sixty-four in the gate, twelve on demand — and its \
         movers eighty, `relocate`'s twenty-seven and `config set`'s fifty-three: \
         sixty-eight in the gate, twelve on demand — and the mint family's fifty, `start`'s \
         fifteen, `workflow`'s eleven, `migrate`'s fifteen and `task amend`'s ten: twenty-seven \
         in the gate, twenty-four on demand"
    );
}

/// The configuration of the runner `dev/gate` runs the suite under, as committed.
const RUNNER: &str = include_str!("../../../.config/nextest.toml");

/// **Every way a nextest configuration leaves the tests `term` names free to run beside
/// the others of the one-at-a-time group.** What holds them is an override of
/// `[profile.default]` whose filter has `term` among its terms — joined by ` | ` and by
/// nothing else — and whose test group is one thread wide.
fn outside_the_group(config: &str, term: &str) -> Vec<String> {
    let config: toml::Table = match config.parse() {
        Ok(config) => config,
        Err(why) => return vec![format!("the runner's configuration is no TOML: {why}")],
    };
    let naming: Vec<&toml::Value> = config
        .get("profile")
        .and_then(|profiles| profiles.get("default"))
        .and_then(|default| default.get("overrides"))
        .and_then(toml::Value::as_array)
        .into_iter()
        .flatten()
        .filter(|over| {
            over.get("filter")
                .and_then(toml::Value::as_str)
                .is_some_and(|filter| filter.split(" | ").any(|part| part.trim() == term))
        })
        .collect();
    if naming.is_empty() {
        return vec![format!(
            "no override of `[profile.default]` has `{term}` as a term of its filter: the \
             arms that run cells are in no test group"
        )];
    }
    let mut out = Vec::new();
    for over in naming {
        let group = over.get("test-group").and_then(toml::Value::as_str);
        let wide = group
            .and_then(|group| config.get("test-groups")?.get(group)?.get("max-threads"))
            .and_then(toml::Value::as_integer);
        if wide != Some(1) {
            out.push(format!(
                "`{term}` is in the test group `{}`, which is not one thread wide",
                group.unwrap_or("<none>")
            ));
        }
    }
    out
}

/// **The arms that run cells run one at a time under the gate's runner**, with the route
/// catalog's: each is a pool as wide as the machine, and the runner gives each test a
/// process of its own, so nothing in the suite can bound two of them together. The term
/// names the module the arms stand in — an arm `arms!` makes for another prepared state
/// is in the bound the day it is made. On planted configurations; then the committed one,
/// which also holds the arm on a named binary, by its name.
#[test]
fn the_arms_that_run_cells_run_one_at_a_time_under_the_gates_runner() {
    let term = "test(/^s::run::/)";
    let config = |groups: &str, filter: &str, group: &str| {
        format!(
            "[test-groups]\n{groups}\n[profile.default]\ntest-threads = 16\n\
             [[profile.default.overrides]]\nfilter = '{filter}'\ntest-group = '{group}'\n"
        )
    };
    let one = "one-at-a-time = { max-threads = 1 }";
    says(
        &outside_the_group(
            &config(one, "test(=s::other) | test(/^s::run::/)", "one-at-a-time"),
            term,
        ),
        &[],
    );
    for (groups, filter, group, said) in [
        (
            one,
            "test(=s::run::fresh)",
            "one-at-a-time",
            "in no test group",
        ),
        (
            one,
            "test(/^s::run::/) & binary(g_other)",
            "one-at-a-time",
            "in no test group",
        ),
        (
            "one-at-a-time = { max-threads = 2 }",
            "test(/^s::run::/)",
            "one-at-a-time",
            "not one thread wide",
        ),
        (one, "test(/^s::run::/)", "another", "not one thread wide"),
    ] {
        says(
            &outside_the_group(&config(groups, filter, group), term),
            &[("`test(/^s::run::/)`", said)],
        );
    }
    says(
        &outside_the_group("[profile.default", term),
        &[("the runner's configuration", "is no TOML")],
    );

    // The arm that runs every cell on a named binary is in the group by its own name: it
    // is one test, ignored, and runs the whole table twice.
    let on_a_named_binary = format!("{}::{ON_A_NAMED_BINARY}", suite());
    assert!(
        route_case::harness_tests().contains(&on_a_named_binary),
        "the harness lists no test `{on_a_named_binary}`"
    );
    let mut wrong = outside_the_group(RUNNER, &format!("test(/^{}::{RUN}::/)", suite()));
    wrong.extend(outside_the_group(
        RUNNER,
        &format!("test(={on_a_named_binary})"),
    ));
    assert!(
        wrong.is_empty(),
        "`.config/nextest.toml` and the arms that run cells:\n\n  {}",
        wrong.join("\n  ")
    );
}

// ---------------------------------------------------------------------------
// The cells on a binary a variable names
// ---------------------------------------------------------------------------

/// The variable that names the binary [`the_cells_are_run_on_the_binary_a_variable_names`]
/// runs every cell on, by an absolute path. The table's own: under the route catalog's
/// variable one export would start both tables.
const NAMED_BINARY: &str = "JIGC_BYTE_TABLE_BINARY";

/// That arm, as the harness names it inside this suite.
const ON_A_NAMED_BINARY: &str = "the_cells_are_run_on_the_binary_a_variable_names";

/// What one cell came to on one binary.
struct CameTo {
    /// Its classification — or, where the frame could not run it on that binary, why: a
    /// prepared state that binary could not build, a lead-in it refused.
    came: Result<Classification, String>,
    /// The lower classes it is red in too.
    also: Vec<Classification>,
    exit: String,
    /// The cell's seconds in the frame: its sandbox, its leads, its three manifests, its
    /// command.
    seconds: f64,
    excused: Vec<(&'static str, usize)>,
    detail: String,
}

impl CameTo {
    fn not_run(why: &str) -> CameTo {
        CameTo {
            // The first line, with the machine's temporary directory written `<tmp>`.
            came: Err(why
                .lines()
                .next()
                .unwrap_or_default()
                .replace(&std::env::temp_dir().display().to_string(), "<tmp>/")),
            also: Vec::new(),
            exit: "-".to_string(),
            seconds: 0.0,
            excused: Vec::new(),
            detail: why.to_string(),
        }
    }

    fn shown(&self) -> String {
        let also: String = self
            .also
            .iter()
            .map(|class| format!("; also {class}"))
            .collect();
        match &self.came {
            Ok(classification) => format!("{classification}{also}"),
            Err(why) => format!("not run: {why}"),
        }
    }
}

/// What a panic said.
fn said(panic: Box<dyn std::any::Any + Send>) -> String {
    panic
        .downcast_ref::<String>()
        .cloned()
        .or_else(|| panic.downcast_ref::<&str>().map(|said| said.to_string()))
        .unwrap_or_else(|| "a panic that said nothing".to_string())
}

/// **Every one of `cells` on `binary`, each on a copy of the state that binary built** —
/// a state at a time, built once. A state that binary cannot build keeps its cells'
/// lines, which say so; so does a cell whose lead-in that binary refused.
fn on(binary: &Path, cells: &[&byte_cell::Cell]) -> Vec<CameTo> {
    let mut came: Vec<Option<CameTo>> = cells.iter().map(|_| None).collect();
    let mut states: Vec<Prepared> = Vec::new();
    for cell in cells {
        if !states.contains(&cell.state) {
            states.push(cell.state);
        }
    }
    for state in states {
        let of_state: Vec<usize> = (0..cells.len())
            .filter(|at| cells[*at].state == state)
            .collect();
        match std::panic::catch_unwind(|| Built::on(binary, state)) {
            Err(panic) => {
                let why = format!(
                    "`{}` could not be built on that binary: {}",
                    state.name(),
                    said(panic)
                );
                for at in of_state {
                    came[at] = Some(CameTo::not_run(&why));
                }
            }
            Ok(built) => {
                let ran = route_case::pooled(route_case::width(), &of_state, |at| {
                    byte_cell::run(&built, row_of(cells[*at]), cells[*at])
                });
                for (at, ran) in of_state.into_iter().zip(ran) {
                    came[at] = Some(match ran {
                        Ok(ran) => CameTo {
                            came: Ok(ran.verdict.classification),
                            also: ran.verdict.also.iter().map(|(class, _)| *class).collect(),
                            exit: ran.exit,
                            seconds: ran.seconds,
                            excused: ran.verdict.excused,
                            detail: ran.detail,
                        },
                        Err(fault) => CameTo::not_run(&fault),
                    });
                }
            }
        }
    }
    came.into_iter()
        .map(|came| came.expect("every cell stands in a state"))
        .collect()
}

/// The table: a line a cell, tab-separated — the cell, its command, its state, what it is
/// listed as, the key it is listed under on this tree, and what it came to on each of the
/// two binaries with how the command ended and the cell's seconds.
fn table(
    cells: &[&byte_cell::Cell],
    there: &[CameTo],
    here: &[CameTo],
    expected: &[ExpectedRed],
) -> String {
    let mut out = "cell\tcommand\tstate\tlisted as\tmarks\tkey\tthe named binary\tits exit\t\
                   seconds\tthis tree's binary\tits exit\tseconds\n"
        .to_string();
    for ((cell, there), here) in cells.iter().zip(there).zip(here) {
        let key = expected
            .iter()
            .find(|entry| entry.cell == cell.name)
            .map_or("-", |entry| entry.key);
        let marks: Vec<String> = cell.marks.iter().map(|mark| mark.named()).collect();
        out.push_str(&format!(
            "{}\tjigc {}\t{}\t{}\t{}\t{key}\t{}\t{}\t{:.1}\t{}\t{}\t{:.1}\n",
            cell.name,
            cell.argv.join(" "),
            cell.state.name(),
            listed_as(cell),
            if marks.is_empty() {
                "-".to_string()
            } else {
                marks.join(" · ")
            },
            there.shown(),
            there.exit,
            there.seconds,
            here.shown(),
            here.exit,
            here.seconds,
        ));
    }
    out
}

/// How many cells came to each pair of outcomes: the named binary's, then this tree's.
fn pairs(there: &[CameTo], here: &[CameTo]) -> String {
    let mut counted: std::collections::BTreeMap<(String, String), usize> = Default::default();
    for (there, here) in there.iter().zip(here) {
        *counted.entry((there.shown(), here.shown())).or_default() += 1;
    }
    let mut out = "the named binary\tthis tree's binary\tcells\n".to_string();
    for ((there, here), cells) in counted {
        out.push_str(&format!("{there}\t{here}\t{cells}\n"));
    }
    out
}

/// **What each line of each row excused, over every cell of its leaf** — on the named
/// binary, then on this tree's: a line that excuses much is seen, and so is one that
/// excused nothing in any cell.
fn excused_by_line(cells: &[&byte_cell::Cell], there: &[CameTo], here: &[CameTo]) -> String {
    let mut out = "command\tline\tthe named binary\tthis tree's binary\n".to_string();
    for row in rows() {
        for line in row.lines {
            let over = |came: &[CameTo]| -> usize {
                cells
                    .iter()
                    .zip(came)
                    .filter(|(cell, _)| cell.leaf == row.leaf)
                    .flat_map(|(_, came)| &came.excused)
                    .filter(|(id, _)| *id == line.id)
                    .map(|(_, times)| times)
                    .sum()
            };
            out.push_str(&format!(
                "jigc {}\t{}\t{}\t{}\n",
                row.leaf.join(" "),
                line.id,
                over(there),
                over(here)
            ));
        }
    }
    out
}

/// **The table runs each binary on the state that binary built, and a cell it cannot run
/// keeps its line** — on two stand-ins. One does nothing and exits 0: `fresh` builds on
/// it — the builder's one `setup` — and the cell comes out *holds*. The other refuses
/// everything: the builder's `setup` is refused, the state cannot be built **on that
/// binary**, and the cell's line says so where the first binary's says what it came to.
/// So neither column is the other's state.
#[test]
fn the_table_runs_each_binary_on_the_state_that_binary_built() {
    let scratch = ScratchDir::new("byte-table-two-stand-ins");
    let (does_nothing, refuses) = (
        scratch.path().join("does-nothing"),
        scratch.path().join("refuses"),
    );
    place_script(&does_nothing, "#!/bin/sh\nexit 0\n");
    place_script(&refuses, "#!/bin/sh\nexit 1\n");
    const CELL: byte_cell::Cell = byte_cell::Cell {
        spelling: None,
        name: "a-cell-in-fresh",
        leaf: &["setup"],
        state: Prepared::Of(State::Fresh),
        leads: &[],
        argv: &["setup"],
        listed: Listed::Acts,
        beside: None,
        aim: None,
        marks: &[],
        at: None,
    };
    let (there, here) = (on(&refuses, &[&CELL]), on(&does_nothing, &[&CELL]));
    let printed = table(&[&CELL], &there, &here, &[]);
    let line = printed.lines().nth(1).unwrap_or_default();
    assert!(
        line.starts_with(
            "a-cell-in-fresh\tjigc setup\tfresh\tacts\t-\t-\tnot run: `fresh` could not be \
             built on that binary: "
        ) && line.contains("\t-\t0.0\tholds\texit 0\t"),
        "{printed}"
    );
    assert!(
        here[0].seconds > 0.0 && !line.ends_with("\t0.0"),
        "a cell that ran has its seconds: {printed}"
    );
    assert_eq!(
        pairs(&there, &here).lines().count(),
        2,
        "one pair of outcomes, under its header"
    );
    let excused = excused_by_line(&[&CELL], &there, &here);
    assert!(
        excused.contains("jigc setup\tS1\t0\t0\n")
            && excused.contains("jigc uninstall\tU1\t0\t0\n"),
        "every line of every row has its line: {excused}"
    );

    let suite = include_str!("byte_table.rs");
    let arm = &suite[suite
        .find(&format!("fn {ON_A_NAMED_BINARY}() {{"))
        .expect("the ignored arm is there")..];
    assert!(
        arm.contains("std::env::var_os(NAMED_BINARY)") && arm.contains("on(named, &cells)"),
        "the ignored arm runs the cells on the binary the variable names"
    );
}

/// **Every cell of the table on the binary a variable names, beside what it came to on
/// this tree's** — a table, run by hand and by no gate:
///
/// ```text
/// JIGC_BYTE_TABLE_BINARY=<an absolute path> \
///     cargo test -p jigc --test g_migration byte_table::the_cells_are_run_on_the_binary_a_variable_names -- --ignored --exact --nocapture
/// ```
///
/// Every cell is run twice, once on each binary, **on a copy of the state that binary
/// built** ([`the_table_runs_each_binary_on_the_state_that_binary_built`]). It prints four
/// blocks, each tab-separated under a line `== <name> ==`: `cells`, the table; `pairs`,
/// how many cells came to each pair of outcomes; `excused`, what each line of each row
/// excused on each binary; `states`, which of the builder's named states the named binary
/// builds, with the seconds each took; and then what the frame saw of every cell whose two
/// outcomes are not the same words.
///
/// **It asserts that it ran** — a binary was named, and every cell has its line — and
/// nothing about what a cell came to on either binary: whether a difference between two
/// releases is anything is not this suite's to say.
#[test]
#[ignore = "a table, not a fence: every cell on the binary JIGC_BYTE_TABLE_BINARY names"]
fn the_cells_are_run_on_the_binary_a_variable_names() {
    let named = std::env::var_os(NAMED_BINARY).unwrap_or_else(|| {
        panic!("`{NAMED_BINARY}` names the binary to run the cells on, and is not set")
    });
    let named = Path::new(&named);
    assert!(
        named.is_absolute() && named.is_file(),
        "`{NAMED_BINARY}` is `{}`: the absolute path of a binary",
        named.display(),
    );
    let cells = cells();
    let (there, here) = (on(named, &cells), on(binary(), &cells));

    println!(
        "== cells ==\n{}",
        table(&cells, &there, &here, &expected_red())
    );
    println!("== pairs ==\n{}", pairs(&there, &here));
    println!("== excused ==\n{}", excused_by_line(&cells, &there, &here));
    println!("== marks ==\nmark\tcell\tthe named binary\tthis tree's binary");
    let mut by_mark: Vec<(Mark, String)> = Vec::new();
    for ((cell, there), here) in cells.iter().zip(&there).zip(&here) {
        for mark in cell.marks {
            by_mark.push((
                *mark,
                format!("{}\t{}\t{}", cell.name, there.shown(), here.shown()),
            ));
        }
    }
    by_mark.sort();
    for (mark, line) in by_mark {
        println!("{}\t{line}", mark.named());
    }
    println!(
        "\n== needs ==\n{}",
        needs_table(&rows(), &cells, &scheduled_cells(SCHEDULED_CELLS))
    );
    println!("== states ==\nstate\ton the named binary\tseconds");
    for state in State::ALL {
        let started = std::time::Instant::now();
        let built = std::panic::catch_unwind(|| Built::on(named, Prepared::Of(*state)));
        println!(
            "{}\t{}\t{:.1}",
            state.name(),
            match built {
                Ok(_) => "built".to_string(),
                Err(panic) => CameTo::not_run(&said(panic)).shown(),
            },
            started.elapsed().as_secs_f64()
        );
    }
    println!("== what the frame saw, where the two outcomes are not the same words ==");
    for ((cell, there), here) in cells.iter().zip(&there).zip(&here) {
        if there.shown() != here.shown() {
            println!(
                "\n`{}`\n-- the named binary: {}\n{}\n-- this tree's binary: {}\n{}",
                cell.name,
                there.shown(),
                there.detail.trim_end(),
                here.shown(),
                here.detail.trim_end(),
            );
        }
    }

    assert_eq!(there.len(), cells.len(), "every cell has its line");
}

/// The task cell and the fan-out cell that are timed alone.
const TIMED_ALONE: &[&str] = &[
    "task-finalize-a-baseline-a-hand-edit-after-the-first-write",
    "milestone-finalize-an-untracked-file-at-the-id-the-join-numbered-onto",
];

/// **One task cell and one fan-out cell, each alone** — a measurement, run by hand and by
/// no gate: the state built, then the one cell, nothing beside it in this process.
///
/// ```text
/// cargo test -p jigc --test g_migration byte_table::one_task_cell_and_one_fan_out_cell_are_timed_alone -- --ignored --exact --nocapture
/// ```
#[test]
#[ignore = "a measurement, not a fence: one task cell and one fan-out cell, each alone"]
fn one_task_cell_and_one_fan_out_cell_are_timed_alone() {
    println!("cell\tits state built in\tthe cell\tcame to");
    for name in TIMED_ALONE {
        let cells = cells();
        let cell = cells
            .iter()
            .find(|cell| cell.name == *name)
            .unwrap_or_else(|| panic!("`{name}` is a cell of the table"));
        let started = std::time::Instant::now();
        let built = Built::on(binary(), cell.state);
        let build = started.elapsed().as_secs_f64();
        let ran = byte_cell::run(&built, row_of(cell), cell).expect("the cell runs");
        println!(
            "{name}\t{build:.1}\t{:.1}\t{}",
            ran.seconds, ran.verdict.classification
        );
    }
}

/// The arm that runs the cells on demand, as the harness names it inside this suite.
const ON_DEMAND_ARM: &str = "the_cells_on_demand_are_run_and_printed";

/// **The cells that run on demand, run and printed** — the base cells in a prepared state
/// that is not their leaf's home (the plan's §9): run by hand and by no gate, on the
/// binary this suite was built with, each on a copy of its state.
///
/// ```text
/// cargo test -p jigc --test g_migration byte_table::the_cells_on_demand_are_run_and_printed -- --ignored --exact --nocapture
/// ```
///
/// It prints, under `== cells on demand ==`, the table that is committed as
/// `…/reshape/byte-table-on-demand-cells.tsv` under a line that says when it was printed
/// ([`every_cell_on_demand_has_its_line_in_the_committed_table`] holds a line a cell);
/// then each cell's seconds, and what the frame saw of every cell that came out red.
/// **It asserts that every cell ran**, and nothing about what one came to: a cell that
/// starts to lose bytes in a state that is not its leaf's home is seen here, by whoever
/// runs this, and by no gate.
#[test]
#[ignore = "a table, not a fence: the base cells in the states that are not their leaf's home"]
fn the_cells_on_demand_are_run_and_printed() {
    let cells = on_demand(&cells());
    let came = on(binary(), &cells);
    println!("== cells on demand ==\n{ON_DEMAND_HEADER}");
    for (cell, came) in cells.iter().zip(&came) {
        println!(
            "{}\tjigc {}\t{}\t{}\t{}\t{}",
            cell.name,
            cell.argv.join(" "),
            cell.state.name(),
            listed_as(cell),
            came.shown(),
            came.exit,
        );
    }
    println!("== seconds ==\ncell\tseconds");
    for (cell, came) in cells.iter().zip(&came) {
        println!("{}\t{:.1}", cell.name, came.seconds);
    }
    println!("== what the frame saw of every cell that is red, or was not run ==");
    for (cell, came) in cells.iter().zip(&came) {
        if !matches!(came.came, Ok(classification) if !classification.is_red()) {
            println!(
                "\n`{}` — {}\n{}",
                cell.name,
                came.shown(),
                came.detail.trim_end()
            );
        }
    }
    assert_eq!(came.len(), cells.len(), "every cell on demand has its line");
}
