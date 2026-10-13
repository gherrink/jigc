//! **The byte table** — one writing command between two pictures of a sandbox, and what
//! the difference is (the exit rule's first clause, `DECISIONS.md` → *2026-10-04 — The exit
//! rule, revised*; the plan is
//! `completions/artifacts/M55/stabilization-build/reshape/byte-table-plan.md`).
//!
//! The frame is [`support::byte_cell`](crate::support::byte_cell), which says what a
//! manifest, a row and a classification are, takes a manifest off a sandbox and runs a
//! cell. **The table has four rows and twelve cells — the pilot's acceptance**: this file
//! is the plan's tasks `B1` to `B3` — the arms that hold the comparison, the reader and
//! the runner to their rules, the fences, and the rows and the cells of the four
//! commands the acceptance needs, a family a file under `byte_table/` ([`FAMILIES`]).
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
//! **The leaf fence** ([`every_write_leaf_has_a_row_a_scheduled_row_or_an_owed_row`]):
//! every `Write` row of the product's own registry (`cli::cli::VERB_KINDS`) has exactly one
//! answer — a row of the table ([`rows`]), a row of the scheduled list
//! (`byte_table/scheduled.tsv`: the leaves the pilot leaves for before the port, each with
//! its reason) or a row of the owed list ([`OWED`], `byte_table/owed.rs`: the leaves the pilot's tasks owe a row,
//! each under the task that owes it, a list the later tasks empty). So a writing command
//! the product gains tomorrow is on no list, and is named until it has its answer.
//!
//! **The cell fence** ([`every_need_of_a_row_has_a_cell_a_scheduled_row_or_is_owed`], the
//! plan's task `B3`): what a row needs a cell for is read off the row — its refusing cell,
//! its base cell in its home state and in each other prepared state, and a planted cell
//! for every line and every kind of plant that targets it
//! ([`byte_cell::aimed`](crate::support::byte_cell::aimed), which also gives the reason a
//! kind does not target a line, so that nobody types one) — and each need has exactly one
//! answer: a cell, a line of `byte_table/scheduled-cells.tsv` with its reason, or its
//! leaf's cells owed whole by a task of the plan ([`CELLS_OWED`]). A cell says which line
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

#[path = "byte_table/owed.rs"]
mod owed;

/// One family's file under `byte_table/`, as the suite reads it.
struct Family {
    /// Its rows, a leaf each.
    rows: &'static [Row],
    /// Its cells.
    cells: &'static [byte_cell::Cell],
}

/// **A family's file is wired in by its declaration inside this macro's one call, and
/// nowhere else** — the `#[path]` line the registration fence reads
/// (`test_target_registration.rs`), which the macro makes the file's row of [`FAMILIES`].
/// The file exports `ROWS` and `CELLS`. It makes no arm of its own: a cell runs in the arm
/// of its prepared state ([`ARMS`]).
macro_rules! families {
    ($(#[path = $file:literal] mod $name:ident;)*) => {
        $(
            #[path = $file]
            mod $name;
        )*

        /// Every family's file, in the order declared.
        const FAMILIES: &[Family] = &[$(Family {
            rows: $name::ROWS,
            cells: $name::CELLS,
        }),*];
    };
}

families! {
    #[path = "byte_table/install.rs"]
    mod install;
    #[path = "byte_table/task.rs"]
    mod task;
    #[path = "byte_table/milestone.rs"]
    mod milestone;
}

use owed::{CELLS_OWED, OWED, OWING};

use crate::support::byte_cell::{
    self, Aim, Allows, Built, Checkout, Classification, Commit, Condition, Ended, Entry, Holding,
    Kind, Layer, Lead, Line, Listed, Manifest, May, Plant, Planted, Prepared, Quote, REPO, Row,
    Seen, Staged, Stands, When, Where, blob_of,
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

/// A line of [`ON_A_REFUSAL`]: a cache at `place`, rebuilt, when the command refused.
const fn rebuilt_on_a_refusal(id: &'static str, place: Where, quotes: &'static [Quote]) -> Line {
    Line {
        id,
        allows: Allows::Files(place, May::Rebuilt),
        when: When::Refused,
        on_refusal: true,
        quotes,
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

/// **Every way the three lists fail to answer for `leaves`, once each** — empty when every
/// write leaf has exactly one answer and every answer is a write leaf's.
fn unanswered(
    leaves: &[Vec<&str>],
    rows: &[&[&str]],
    scheduled: &[(Vec<&str>, &str)],
    owed: &[(&[&str], &str)],
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
            (
                owed.iter()
                    .filter(|(row, _)| *row == leaf.as_slice())
                    .count(),
                "a row of the owed list",
            ),
        ]
        .into_iter()
        .flat_map(|(times, what)| std::iter::repeat_n(what, times))
        .collect();
        match answers.as_slice() {
            [_] => {}
            [] => out.push(format!(
                "{} writes, and has no row, no scheduled row and no owed row",
                named(leaf)
            )),
            several => out.push(format!(
                "{} is answered more than once: {}",
                named(leaf),
                several.join(" and ")
            )),
        }
    }
    let listed = rows
        .iter()
        .map(|row| (row.to_vec(), "the table"))
        .chain(
            scheduled
                .iter()
                .map(|(row, _)| (row.clone(), "the scheduled list")),
        )
        .chain(owed.iter().map(|(row, _)| (row.to_vec(), "the owed list")));
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
    for (leaf, task) in owed {
        if !OWING.contains(task) {
            out.push(format!(
                "{} is owed by `{task}`, which is no task of the plan that owes a row",
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

/// **Every write leaf of the product has exactly one answer**: a row of the table, a row
/// of the scheduled list, or a row of the owed list under the task that owes it.
///
/// On a fixture: a write leaf nobody answers for is named; so is one answered twice, an
/// answer for a leaf the registry does not hold as a write, a scheduled row with no
/// reason, and an owed row under no task. Then this tree, against the lists this suite
/// holds — the registry has 36 write leaves, and none is unanswered — and this tree with
/// one write leaf more than it has: **a writing command the product gains tomorrow is on
/// no list, and is named until it has its answer**.
#[test]
fn every_write_leaf_has_a_row_a_scheduled_row_or_an_owed_row() {
    let leaves = vec![vec!["setup"], vec!["doc", "create"], vec!["relocate"]];
    let (setup, create, relocate): (&[&str], &[&str], &[&str]) =
        (&["setup"], &["doc", "create"], &["relocate"]);
    let waits = |leaf: &[&'static str], why: &'static str| (leaf.to_vec(), why);
    says(
        &unanswered(&leaves, &[], &[], &[]),
        &[
            ("`jigc setup`", "no row, no scheduled row and no owed row"),
            (
                "`jigc doc create`",
                "no row, no scheduled row and no owed row",
            ),
            (
                "`jigc relocate`",
                "no row, no scheduled row and no owed row",
            ),
        ],
    );
    let in_order = (
        vec![setup],
        vec![waits(relocate, "the count holds it untouched")],
        vec![(create, "B6")],
    );
    says(
        &unanswered(&leaves, &in_order.0, &in_order.1, &in_order.2),
        &[],
    );
    says(
        &unanswered(
            &leaves,
            &in_order.0,
            &in_order.1,
            &[(create, "B6"), (setup, "B4")],
        ),
        &[(
            "`jigc setup`",
            "a row of the table and a row of the owed list",
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
            &in_order.2,
        ),
        &[("`jigc doc show`", "no write leaf of that name")],
    );
    says(
        &unanswered(&leaves, &in_order.0, &[waits(relocate, " ")], &in_order.2),
        &[("`jigc relocate`", "does not say why it waits")],
    );
    says(
        &unanswered(&leaves, &in_order.0, &in_order.1, &[(create, "B1")]),
        &[("`jigc doc create`", "no task of the plan that owes a row")],
    );

    let here = write_leaves();
    assert_eq!(
        here.len(),
        36,
        "the product's registry holds 36 write leaves — the number the pilot's plan counts \
         from; a leaf more or less is a row, a scheduled row or an owed row to write or to \
         take away, and this number with it"
    );
    let rows: Vec<&[&str]> = rows().iter().map(|row| row.leaf).collect();
    assert_eq!(
        rows.len(),
        4,
        "the table has the four rows of the pilot's acceptance: {rows:?}"
    );
    let wrong = unanswered(&here, &rows, &scheduled(SCHEDULED), OWED);
    assert!(
        wrong.is_empty(),
        "the byte table does not answer for the product's write leaves as it must:\n\n  \
         {}\n\na write leaf has a row in a file under `byte_table/`, a line of \
         `byte_table/scheduled.tsv` with its reason, or a row of `OWED` under the task of \
         the plan that owes it — exactly one of the three",
        wrong.join("\n  ")
    );

    // Tomorrow's leaf.
    let mut tomorrow = here;
    tomorrow.push(vec!["doc", "merge"]);
    says(
        &unanswered(&tomorrow, &rows, &scheduled(SCHEDULED), OWED),
        &[(
            "`jigc doc merge`",
            "no row, no scheduled row and no owed row",
        )],
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
    Planted(Aim),
}

impl Need {
    /// The need as the scheduled list of cells names it.
    fn named(&self) -> String {
        match self {
            Need::Refusing => "refusing".to_string(),
            Need::Base => "base".to_string(),
            Need::BaseIn(state) => format!("base, in `{}`", state.name()),
            Need::Planted(aim) => format!("{} × {}", aim.line, aim.kind.tag()),
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
    for line in row.lines {
        for kind in Kind::ALL {
            if byte_cell::aimed(*kind, line).is_ok() {
                out.push(Need::Planted(Aim {
                    line: line.id,
                    kind: *kind,
                }));
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
    let aim = match (layer, cell.aim) {
        (Layer::Planted, Some(aim)) => aim,
        (Layer::Planted, None) => return Err("puts a plant in and aims it at no line".to_string()),
        (_, Some(_)) => return Err("aims at a line and puts no plant in".to_string()),
        (Layer::Refusing, None) => return Ok(Need::Refusing),
        (Layer::Base, None) => return Ok(Need::Base),
        (Layer::BaseElsewhere, None) => return Ok(Need::BaseIn(cell.state)),
    };
    let Some(line) = row.lines.iter().find(|line| line.id == aim.line) else {
        return Err(format!(
            "aims at the line `{}`, which its row does not have",
            aim.line
        ));
    };
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
    Ok(Need::Planted(aim))
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

/// **Every way the cells, the scheduled list of cells and the list of leaves whose cells
/// are owed fail to answer for what the rows need** — empty when every need of every row
/// has exactly one answer: a cell, or a scheduled row with its reason; or its leaf's
/// cells are owed whole, by a task of the plan. A cell that cannot be read as an answer
/// is named too ([`answers`]).
fn unmet(
    rows: &[&Row],
    cells: &[&byte_cell::Cell],
    scheduled: &[(Vec<&str>, &str, &str)],
    owed: &[(&[&str], &str)],
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
        let owing: Vec<&str> = owed
            .iter()
            .filter(|(leaf, _)| *leaf == row.leaf)
            .map(|(_, task)| *task)
            .collect();
        let waiting: Vec<&(Vec<&str>, &str, &str)> = scheduled
            .iter()
            .filter(|(leaf, _, _)| leaf.as_slice() == row.leaf)
            .collect();
        let needs = needs(row);
        if !owing.is_empty() {
            if owing.len() > 1 {
                out.push(format!(
                    "the cells of {} are owed more than once",
                    named(row.leaf)
                ));
            }
            if !waiting.is_empty() {
                out.push(format!(
                    "{} is answered more than once: its cells are owed by `{}`, and it has \
                     rows of the scheduled list of cells",
                    named(row.leaf),
                    owing[0]
                ));
            }
        } else {
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
    for (leaf, task) in owed {
        if !rows.iter().any(|row| row.leaf == *leaf) {
            out.push(format!(
                "the cells of {} are owed, and it has no row",
                named(leaf)
            ));
        }
        if !OWING.contains(task) {
            out.push(format!(
                "the cells of {} are owed by `{task}`, which is no task of the plan that owes \
                 a row",
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
    owed: &[(&[&str], &str)],
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
            let owing = owed.iter().find(|(leaf, _)| *leaf == row.leaf);
            let answer = match (by.is_empty(), waits, owing) {
                (false, _, _) => by.join(" · "),
                (true, Some((_, _, reason)), _) => format!("scheduled: {reason}"),
                (true, None, Some((_, task))) => format!("owed by `{task}`"),
                (true, None, None) => "nothing".to_string(),
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

/// **Every row has, for each thing it needs a cell for, a cell, a scheduled row, or its
/// leaf's cells owed whole by a task of the plan** — so a line a row gains tomorrow needs
/// its planted cells that day, and a cell cannot be born aimed at nothing.
///
/// On a fixture: a row with no cell is named need by need — its refusing cell, its base
/// cell, the base cell of each other prepared state, and each plant that targets one of
/// its lines; a cell, a scheduled row, or the owed list answers each; a need answered
/// twice is named, and so are a scheduled row for nothing the row needs, one with no
/// reason, a plant aimed at no line, an aim with no plant, an aim at a line the row does
/// not have, an aim the line's own vocabulary turns away — with **the reason the frame
/// derives** — and a plant that is not of the kind the cell says. Then this tree: nothing
/// is unmet; and **the fence is not vacuous** — with the owed list taken away, the four
/// rows of the acceptance are named for what the twelve cells do not answer.
#[test]
fn every_need_of_a_row_has_a_cell_a_scheduled_row_or_is_owed() {
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
        unmet(&[&REPLACES], &[], &[], &[]).len(),
        names.len(),
        "a row nobody answers for is named need by need"
    );
    says(
        &unmet(&[&REPLACES], &[], &[], &[(&["task", "finalize"], "B4")]),
        &[],
    );

    const fn cell(
        name: &'static str,
        state: Prepared,
        leads: &'static [Lead],
        argv: &'static [&'static str],
        aim: Option<Aim>,
    ) -> byte_cell::Cell {
        byte_cell::Cell {
            name,
            leaf: &["task", "finalize"],
            state,
            leads,
            argv,
            listed: Listed::Acts,
            beside: None,
            aim,
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
        Ok(Need::Planted(Aim {
            line: "F1",
            kind: Kind::Untracked
        }))
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
        unmet(&[&REPLACES], &the_four, &[], &[]).len(),
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
    says(&unmet(&[&REPLACES], &the_four, &rest, &[]), &[]);

    // Answered twice; a row for nothing; a row with no reason; owed and scheduled at once.
    let mut twice = rest.clone();
    twice.push(waits("F1 × U", "before the port"));
    twice.push(waits("F3 × U", "before the port"));
    twice.push(waits("F2 × H", " "));
    says(
        &unmet(&[&REPLACES], &the_four, &twice, &[]),
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
    says(
        &unmet(
            &[&REPLACES],
            &[],
            &[
                waits("refusing", "before the port"),
                (vec!["doc", "create"], "base", "before the port"),
            ],
            &[
                (&["task", "finalize"], "B4"),
                (&["task", "finalize"], "B5"),
                (&["relocate"], "B1"),
            ],
        ),
        &[
            ("`jigc task finalize`", "owed more than once"),
            (
                "`jigc task finalize`",
                "its cells are owed by `B4`, and it has rows of the scheduled list",
            ),
            ("`jigc doc create`", "has no row"),
            ("the cells of `jigc relocate` are owed", "it has no row"),
            ("`jigc relocate`", "no task of the plan that owes a row"),
        ],
    );

    // A cell that cannot be read as an answer.
    let owed_whole: &[(&[&str], &str)] = &[(&["task", "finalize"], "B4")];
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
        says(
            &unmet(&[&REPLACES], &[&cell], &[], owed_whole),
            &[(cell.name, said)],
        );
    }

    // This tree.
    let (rows, cells) = (rows(), cells());
    let wrong = unmet(&rows, &cells, &scheduled_cells(SCHEDULED_CELLS), CELLS_OWED);
    assert!(
        wrong.is_empty(),
        "the byte table's cells do not answer for what its rows need:\n\n  {}\n\na row needs \
         its refusing cell, its base cell in its home state and in each other prepared state, \
         and a planted cell for every line and every kind of plant that targets it — each a \
         cell in a file under `byte_table/`, or a line of `byte_table/scheduled-cells.tsv` \
         with its reason; or the leaf's cells are owed whole in `CELLS_OWED`, by the task of \
         the plan that owes them",
        wrong.join("\n  ")
    );
    // Not vacuous: with nothing owed, the rows of the acceptance are named for what the
    // acceptance's cells do not answer — and the cells that are there answer something.
    let bare = unmet(&rows, &cells, &[], &[]);
    let needed: usize = rows.iter().map(|row| needs(row).len()).sum();
    assert!(
        bare.len() > 100 && bare.len() < needed,
        "with nothing owed, {} of {needed} needs are unmet",
        bare.len()
    );
    assert!(
        bare.iter().any(|line| line
            == "`jigc uninstall` needs a cell for *refusing*, and has no cell and no scheduled \
                row for it")
            && !bare.iter().any(|line| line.contains("*U1 × D*")),
        "{bare:#?}"
    );
    assert_eq!(
        needs_table(&rows, &cells, &scheduled_cells(SCHEDULED_CELLS), CELLS_OWED)
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
    (
        &["task", "finalize"],
        "C1",
        &[
            Prepared::Of(State::Fresh),
            Prepared::Of(State::CommittedSingletons),
        ],
    ),
    (
        &["milestone", "finalize"],
        "M1",
        &[Prepared::Of(State::Fresh)],
    ),
    (
        &["milestone", "finalize"],
        "M3",
        &[Prepared::Of(State::Fresh)],
    ),
    (
        &["milestone", "finalize"],
        "M5",
        &[Prepared::Of(State::Fresh)],
    ),
    (
        &["milestone", "finalize"],
        "M6, the worktrees",
        &[Prepared::Of(State::Fresh)],
    ),
    (
        &["milestone", "finalize"],
        "M6, the sub-tasks' areas",
        &[Prepared::Of(State::Fresh)],
    ),
    (
        &["milestone", "finalize"],
        "M6, the milestone's area",
        &[Prepared::Of(State::Fresh)],
    ),
    (
        &["milestone", "finalize"],
        "M7",
        &[Prepared::Of(State::Fresh)],
    ),
    (
        &["milestone", "finalize"],
        "M10, the edge index",
        &[Prepared::Of(State::Fresh)],
    ),
    (
        &["milestone", "finalize"],
        "M10, the file-state hashes",
        &[Prepared::Of(State::Fresh)],
    ),
];

/// What would use every line of a row that no cell of the gate excuses a difference
/// under: the cells of its family, which a later task of the plan owes.
const ITS_FAMILYS_CELLS: &str = "the cells of its family, which are owed";

/// **The lines that excuse nothing in any cell the gate runs** — named, each with what
/// would use it, and not removed: a line is its doc's sentence, and stands whether or not
/// a cell has met it yet. A line that is on neither list, or on both, is named
/// ([`unaccounted`]).
const EXCUSES_NOTHING: &[(&[&str], &str, &str)] = &[
    (&["setup"], "S1", ITS_FAMILYS_CELLS),
    (&["setup"], "S2", ITS_FAMILYS_CELLS),
    (&["setup"], "S3", ITS_FAMILYS_CELLS),
    (&["setup"], "S4", ITS_FAMILYS_CELLS),
    (&["setup"], "S5", ITS_FAMILYS_CELLS),
    (&["setup"], "S6, written", ITS_FAMILYS_CELLS),
    (&["setup"], "S6, removed", ITS_FAMILYS_CELLS),
    (&["setup"], "S7", ITS_FAMILYS_CELLS),
    (&["setup"], "S8", ITS_FAMILYS_CELLS),
    (&["setup"], "S9", ITS_FAMILYS_CELLS),
    (&["setup"], "S10", ITS_FAMILYS_CELLS),
    (&["setup"], "S11", ITS_FAMILYS_CELLS),
    (&["setup"], "C1", A_REFUSAL_THAT_REBUILDS_IT),
    (&["setup"], "C2", A_REFUSAL_THAT_REBUILDS_IT),
    (&["setup"], "C3, the base pin", NO_CELL_OF_THIS_LEAF),
    (&["setup"], "C3, the task list", NO_CELL_OF_THIS_LEAF),
    (&["uninstall"], "U1", ITS_FAMILYS_CELLS),
    (&["uninstall"], "U2", ITS_FAMILYS_CELLS),
    (&["uninstall"], "U3", ITS_FAMILYS_CELLS),
    (&["uninstall"], "U4", ITS_FAMILYS_CELLS),
    (&["uninstall"], "U4, removed", ITS_FAMILYS_CELLS),
    (&["uninstall"], "U5", ITS_FAMILYS_CELLS),
    (&["uninstall"], "U6", ITS_FAMILYS_CELLS),
    (&["uninstall"], "C1", A_REFUSAL_THAT_REBUILDS_IT),
    (&["uninstall"], "C2", A_REFUSAL_THAT_REBUILDS_IT),
    (&["uninstall"], "C3, the base pin", NO_CELL_OF_THIS_LEAF),
    (&["uninstall"], "C3, the task list", NO_CELL_OF_THIS_LEAF),
    (&["task", "finalize"], "T1", ITS_FAMILYS_CELLS),
    (&["task", "finalize"], "T2", ITS_FAMILYS_CELLS),
    (&["task", "finalize"], "T3", ITS_FAMILYS_CELLS),
    (&["task", "finalize"], "T4", ITS_FAMILYS_CELLS),
    (
        &["task", "finalize"],
        "T5, the edge index",
        ITS_FAMILYS_CELLS,
    ),
    (
        &["task", "finalize"],
        "T5, the file-state hashes",
        ITS_FAMILYS_CELLS,
    ),
    (&["task", "finalize"], "T6", ITS_FAMILYS_CELLS),
    (&["task", "finalize"], "T6, moved aside", ITS_FAMILYS_CELLS),
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
    (&["milestone", "finalize"], "M2", ITS_FAMILYS_CELLS),
    (
        &["milestone", "finalize"],
        "M6, moved aside",
        ITS_FAMILYS_CELLS,
    ),
    (&["milestone", "finalize"], "C1", A_REFUSAL_THAT_REBUILDS_IT),
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
];

/// What would use a line of the ruling on a refusal's caches that no refusal of the
/// leaf's cells has met.
const A_REFUSAL_THAT_REBUILDS_IT: &str =
    "a refusal of this leaf that leaves that cache rebuilt: none of its cells' refusals does";

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
                 *{reading}*: a touched leaf is the pilot's, and is owed its row",
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
         and a leaf it holds touched under the reading the pilot is built under has a row or \
         is owed one",
        wrong.join("\n  ")
    );
}

// ---------------------------------------------------------------------------
// The quote fence
// ---------------------------------------------------------------------------

/// The design docs an allowance line may quote, by their path from the repository's root.
const DOCS: &[(&str, &str)] = &[
    (
        "design/assistant-adapter.md",
        include_str!("../../../design/assistant-adapter.md"),
    ),
    (
        "design/finalize.md",
        include_str!("../../../design/finalize.md"),
    ),
    (
        "design/project-setup.md",
        include_str!("../../../design/project-setup.md"),
    ),
    (
        "design/storage.md",
        include_str!("../../../design/storage.md"),
    ),
    (
        "design/team-ready-state.md",
        include_str!("../../../design/team-ready-state.md"),
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
                    stored: manifest.objects.iter().any(|id| holds(id.as_bytes())),
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
    }
}

const fn row(lines: &'static [Line]) -> Row {
    Row {
        leaf: &["probe"],
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
    const LANDS: Row = row(&[
        line(
            "L1",
            Allows::Files(Where::AStagedDocsHome, May::CopyOfAStagedDoc),
        ),
        line("L2", Allows::Commit(Holding::TheIndex)),
    ]);
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
            ..Cell::of(LANDS, &before, &landed)
        },
        Classification::GitMoved,
    );
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
/// rewrite of a file the prepared state held — nobody's lead-in wrote it — is *lost*; and
/// where the store still has those bytes and nothing reaches them, the verdict says that
/// it hangs on what *held* means.
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
    assert!(!prepared.verdict().expect("a cell").hangs_on_reach);
    let stored = then(&after, |m| {
        m.objects.insert(blob_of("{\"stamp\":\"c0\"}\n"));
    });
    let hangs = Cell::of(NOTHING, &before, &stored)
        .verdict()
        .expect("a cell");
    assert_eq!(hangs.classification, Classification::Lost);
    assert!(
        hangs.hangs_on_reach,
        "the store has the bytes and nothing reaches them"
    );
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
        line("K2", Allows::Commit(Holding::TheWorktrees)),
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
    const FINALIZES: Row = row(&[
        line(
            "F1",
            Allows::Files(Where::AStagedDocsHome, May::CopyOfAStagedDoc),
        ),
        line("F2", Allows::Commit(Holding::TheIndex)),
        line(
            "F3",
            Allows::Files(Where::Under(".jigc/tasks/*"), May::Removed),
        ),
    ]);
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
    comes_out(
        &Cell::of(FINALIZES, &before, &landed),
        Classification::Holds,
    );
    let nowhere = then(&before, area_gone);
    comes_out(
        &Cell::of(FINALIZES, &before, &nowhere),
        Classification::Lost,
    );
    let elsewhere = then(&before, |m| {
        area_gone(m);
        put(m, "repo/docs/nowhere.md", PROSE);
    });
    comes_out(
        &Cell::of(FINALIZES, &before, &elsewhere),
        Classification::GitMoved,
    );
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
    let (got, why) = Cell::of(FINALIZES, &before, &through_a_link).came_to();
    assert_eq!(got, Classification::GitMoved, "{why:#?}");
    assert!(
        why.iter()
            .any(|why| why.contains("holds a link at `docs/use-tabs.md`")),
        "{why:#?}"
    );
    // The area still there: nothing is asked of the doc yet.
    comes_out(
        &Cell::of(FINALIZES, &before, &before),
        Classification::Holds,
    );
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
        &Cell::of(FINALIZES, &with_its_commit_doc, &landed),
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
        &Cell::of(FINALIZES, &with_a_lookalike, &landed),
        Classification::Lost,
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
            }],
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
writes-its-own) mkdir -p .jigc && echo own > .jigc/own.txt ;;
rewrites-its-own) echo other > .jigc/own.txt ;;
overwrites-the-committed-plant) echo other > notes.txt ;;
writes-through-the-link) echo prose > a-link.md ;;
rewrites-the-edited-file) echo rewritten > README.md ;;
prunes-the-worktrees) git worktree prune ;;
seeds-the-ignore-file) echo secrets > .gitignore ;;
writes-a-baseline) mkdir -p .jigc/state && echo '{}' > .jigc/state/hashes.json ;;
sleeps) exec sleep 30 ;;
*) echo "the stand-in knows no '$1'" >&2; exit 64 ;;
esac
"#;

/// One cell on the stand-in: the row it is compared under, what it must come out as, and
/// a piece of why. `hangs` is whether the verdict must say it hangs on what *held* means.
struct OnAStandIn {
    cell: byte_cell::Cell,
    row: Row,
    expected: Classification,
    why: &'static str,
    hangs: bool,
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
            name: argv[0],
            leaf: &["probe"],
            state: Prepared::Bare,
            leads,
            argv,
            listed: Listed::Acts,
            beside: None,
            aim: None,
        },
        row,
        expected,
        why,
        hangs: false,
    }
}

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
    // --- the plants: a committed file, a link to nothing, an edit, the user's worktree ---
    on_a_stand_in(
        &["overwrites-the-committed-plant"],
        &[Lead::Plant(Plant::Committed("notes.txt"))],
        NOTHING,
        Classification::GitMoved,
        "the planted `repo/notes.txt` is gone from every path of the sandbox, and an object \
         within reach still has its bytes",
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
    OnAStandIn {
        hangs: true,
        ..on_a_stand_in(
            &["stores-then-removes"],
            A_PLANT,
            REMOVES,
            Classification::Lost,
            "in no object within reach",
        )
    },
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
            name: "refuses",
            leaf: &["probe"],
            state: Prepared::Bare,
            leads: &[],
            argv: &["refuses"],
            listed: Listed::Refuses,
            beside: None,
            aim: None,
        },
        row: NOTHING,
        expected: Classification::RefusesNothingChanged,
        why: "",
        hangs: false,
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
/// cell wrote, rewritten.
///
/// Then the faults of a cell, which no classification covers: a lead-in that is refused,
/// a plant where something stands, a state that is not the one handed in, a row of
/// another leaf.
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
                if verdict.classification != on.expected
                    || !says
                    || verdict.hangs_on_reach != on.hangs
                {
                    wrong.push(format!(
                        "{named} came out *{}* (hangs on reach: {}), and *{}* with `{}` was \
                         expected (hangs on reach: {}):\n{}",
                        verdict.classification,
                        verdict.hangs_on_reach,
                        on.expected,
                        on.why,
                        on.hangs,
                        ran.detail
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

    // The faults of a cell.
    let cell = |name, state, leads, leaf| byte_cell::Cell {
        name,
        leaf,
        state,
        leads,
        argv: &["refuses"],
        listed: Listed::Refuses,
        beside: None,
        aim: None,
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
    let started = std::time::Instant::now();
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
    assert!(
        started.elapsed() < Duration::from_secs(25),
        "the command was ended at the limit, and not waited out: {:?}",
        started.elapsed()
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
        name,
        leaf: &["probe"],
        state,
        leads,
        argv: &["--version"],
        listed: Listed::Acts,
        beside: None,
        aim: None,
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

// ---------------------------------------------------------------------------
// The cells — an arm a prepared state, and the expected-red list
// ---------------------------------------------------------------------------

/// Every cell of every family's file.
fn cells() -> Vec<&'static byte_cell::Cell> {
    FAMILIES.iter().flat_map(|family| family.cells).collect()
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
const EXPECTED_RED: &[(&str, Classification, &[&str])] = &[];

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
                    if ran.verdict.hangs_on_reach {
                        " (it hangs on what *held* means)"
                    } else {
                        ""
                    },
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
        cells.len(),
        12,
        "the pilot's acceptance is twelve cells (the plan's §5)"
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
    exit: String,
    hangs: bool,
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
            exit: "-".to_string(),
            hangs: false,
            seconds: 0.0,
            excused: Vec::new(),
            detail: why.to_string(),
        }
    }

    fn shown(&self) -> String {
        match &self.came {
            Ok(classification) if self.hangs => {
                format!("{classification} (hangs on what *held* means)")
            }
            Ok(classification) => classification.to_string(),
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
                            exit: ran.exit,
                            hangs: ran.verdict.hangs_on_reach,
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
    let mut out = "cell\tcommand\tstate\tlisted as\tkey\tthe named binary\tits exit\tseconds\t\
                   this tree's binary\tits exit\tseconds\n"
        .to_string();
    for ((cell, there), here) in cells.iter().zip(there).zip(here) {
        let key = expected
            .iter()
            .find(|entry| entry.cell == cell.name)
            .map_or("-", |entry| entry.key);
        out.push_str(&format!(
            "{}\tjigc {}\t{}\t{}\t{key}\t{}\t{}\t{:.1}\t{}\t{}\t{:.1}\n",
            cell.name,
            cell.argv.join(" "),
            cell.state.name(),
            match cell.listed {
                Listed::Acts => "acts",
                Listed::Refuses => "refuses",
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
        name: "a-cell-in-fresh",
        leaf: &["setup"],
        state: Prepared::Of(State::Fresh),
        leads: &[],
        argv: &["setup"],
        listed: Listed::Acts,
        beside: None,
        aim: None,
    };
    let (there, here) = (on(&refuses, &[&CELL]), on(&does_nothing, &[&CELL]));
    let printed = table(&[&CELL], &there, &here, &[]);
    let line = printed.lines().nth(1).unwrap_or_default();
    assert!(
        line.starts_with(
            "a-cell-in-fresh\tjigc setup\tfresh\tacts\t-\tnot run: `fresh` could not be built \
             on that binary: "
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
    println!(
        "== needs ==\n{}",
        needs_table(
            &rows(),
            &cells,
            &scheduled_cells(SCHEDULED_CELLS),
            CELLS_OWED
        )
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
            match cell.listed {
                Listed::Acts => "acts",
                Listed::Refuses => "refuses",
            },
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
