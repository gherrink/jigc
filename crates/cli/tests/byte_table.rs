//! **The byte table** — one writing command between two pictures of a sandbox, and what
//! the difference is (the exit rule's first clause, `DECISIONS.md` → *2026-10-04 — The exit
//! rule, revised*; the plan is
//! `completions/artifacts/M55/stabilization-build/reshape/byte-table-plan.md`).
//!
//! The frame is [`support::byte_cell`](crate::support::byte_cell), which says what a
//! manifest, a row and a classification are, takes a manifest off a sandbox and runs a
//! cell. **This file holds no cell of the table yet**: it is the plan's tasks `B1` to
//! `B2b` — the arms that hold the comparison, the reader and the runner to their rules,
//! and the two fences that stand before the first row and the first cell are born.
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
//! under the heading it names. No row is data yet, so the fence holds what the rows will
//! be lifted from — **every quotation of the committed record**
//! (`…/reshape/byte-table-acceptance-rows.md`) — and a design doc that moves under one
//! reddens here from this commit on.
//!
//! **The leaf fence** ([`every_write_leaf_has_a_row_a_scheduled_row_or_an_owed_row`]):
//! every `Write` row of the product's own registry (`cli::cli::VERB_KINDS`) has exactly one
//! answer — a row of the table ([`ROWS`]), a row of the scheduled list
//! (`byte_table/scheduled.tsv`: the leaves the pilot leaves for before the port, each with
//! its reason) or a row of the owed list ([`OWED`], `byte_table/owed.rs`: the leaves the pilot's tasks owe a row,
//! each under the task that owes it, a list the later tasks empty). So a writing command
//! the product gains tomorrow is on no list, and is named until it has its answer.
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

use owed::{OWED, OWING};

use crate::support::byte_cell::{
    self, Allows, Built, Checkout, Classification, Commit, Ended, Entry, Holding, Lead, Line,
    Listed, Manifest, May, Plant, Planted, Prepared, Quote, REPO, Row, Seen, Staged, Stands, When,
    Where, blob_of,
};
use crate::support::route_case::{self, Place, Step};
use crate::support::scratch::ScratchDir;
use crate::support::trial_corpus::{State, TrialCorpus};

// ---------------------------------------------------------------------------
// The table — its rows, and the two lists beside them
// ---------------------------------------------------------------------------

/// **The rows of the table**, a leaf each. None yet: the four of the acceptance become
/// data in the plan's task `B2c`, lifted quote for quote from the committed record.
const ROWS: &[Row] = &[];

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
            &[(create, "B6"), (setup, "B2c")],
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
    let rows: Vec<&[&str]> = ROWS.iter().map(|row| row.leaf).collect();
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

/// **An allowance line's quote is held to its doc's text**: the words stand, byte for
/// byte, under the heading the line names.
///
/// On a fixture: a quote as the doc has it holds; one word off does not, nor the right
/// words under another heading — a later one included: a section ends where the next
/// heading of its level opens — a heading the doc does not have, a doc the fence does not
/// read, or a quote of no words; and a `#` line inside a fenced block ends no section. Then
/// the committed record: it holds quotations, every one names a doc and a heading, and
/// every one stands in its doc — so a design doc that moves under a line reddens here.
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
    let lines: BTreeSet<&str> = ROWS
        .iter()
        .flat_map(|row| row.lines)
        .flat_map(|line| line.quotes)
        .map(|quote| quote.words)
        .collect();
    let wrong: Vec<String> = quoted
        .iter()
        .chain(
            ROWS.iter()
                .flat_map(|row| row.lines)
                .flat_map(|line| line.quotes),
        )
        .filter_map(|quote| byte_cell::unquoted(quote, doc))
        .chain(
            lines
                .iter()
                .filter(|words| !quoted.iter().any(|quote| quote.words == **words))
                .map(|words| format!("a line of a row quotes what the record does not: `{words}`")),
        )
        .collect();
    assert!(
        wrong.is_empty(),
        "an allowance line rests on a sentence its design doc no longer has, or never \
         had:\n\n  {}\n\na line is written from the doc's sentence and quotes it; where the \
         doc moved, the line is read against the doc again — it is never bent to what the \
         command does",
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
/// link at the home in the doc's place, *git's state moved*.
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

/// **The cells on the stand-in**, a layer of the manifest each — what the reader must
/// read for the comparison to come out as it did over manifests built by hand.
const ON_A_STAND_IN: &[OnAStandIn] = &[
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

    // The faults of a cell.
    let cell = |name, state, leads, leaf| byte_cell::Cell {
        name,
        leaf,
        state,
        leads,
        argv: &["refuses"],
        listed: Listed::Refuses,
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
