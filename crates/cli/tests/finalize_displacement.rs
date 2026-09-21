//! M52 Increment 4 / T3 — **`jigc task finalize` keeps what it cannot commit**
//! (`completions/artifacts/M52/settle-record.md` → D3.3 as amended by §8;
//! `design/team-ready-state.md` → The working area's two populations;
//! `design/storage.md` → the per-task working area).
//!
//! Driven at `8fdda36` on the debug binary, the exact cell below: a `NOTES.md` at a task
//! area's root, an `analysis/perf.txt` under it and a `docs/notes.txt` beside the staged
//! prose all died at **exit 0** when the commit landed, with an **empty stderr** and no
//! `.jigc/displaced/` ever created. Phase 7 removed the working area whole
//! (`remove_dir_all(cleanup_dir)`), and the area is the one place an agent's own scratch
//! sits: gitignored, so no commit has a copy and `git status` never mentions it.
//!
//! This is the door the Settle refuses to give a consent flag: a `task finalize --force`
//! would be a new capability, and the landed-boundary warrant for *narrating* the loss
//! (*the staged set is already in git*) is structurally unavailable here — the commit takes
//! the promoted docs and the index, and nothing at all out of the working area. So the
//! disposition is **displace**: the complement moves to `.jigc/displaced/<task-id>/`, with
//! its relative path preserved, before the teardown runs. The teardown itself is never
//! skipped — a left-over area makes `jigc task list` report an active task that no longer
//! exists.
//!
//! **Three arms. The second catches a subject cut too wide; the third catches a subject
//! reported too narrow** (M53 Increment 2 / T2 —
//! `completions/artifacts/M53/settle-record.md` → D2.1/D2.2):
//!
//! 1. **the populated cell** — the three plants above, driven through a real landed
//!    finalize on both surfaces: they survive byte-intact under `.jigc/displaced/<id>/`,
//!    each move is named on **stderr** (the side channel, so the `--format json` document
//!    still owns stdout undiluted) and carried on the envelope's `committed.displaced`
//!    array as `{from, to}` repo-relative pairs sorted by `from`. The nested plant is
//!    displaced as the **directory** `analysis/` — the complement's unit is the entry, not
//!    the file, and moving the top of a subtree preserves it — so the pair names
//!    `analysis` while `analysis/perf.txt` is what survives inside it.
//! 2. **the omitting cell** — a task whose area holds nothing but jigc's own files.
//!    `displaced` is `[]` (always present: an absent key would make a driver guess) and the
//!    text surface says nothing at all. A subject drawn one member too wide — the registry
//!    missing `renames.json`, say — would show here as jigc moving its own files aside on
//!    the ordinary path, which is the failure mode the writer registry exists to prevent.
//! 3. **the partial cell** — the parking home occupied so one entry's move cannot land while
//!    the other's can. The move axis is `{all · some · none}`, not `{succeeds · fails}`, and
//!    the middle value is where the narration's count decides whether the door answers for
//!    the **area** or only for what it managed to do with it.
//!
//! **What this suite does not yet assert, said here rather than left to be inferred:** at T2
//! the entry whose move failed is still taken by the teardown that follows — nothing is kept
//! that was destroyed before, and an assertion over that outcome would pin the loss as
//! expected output. T3 conditions the removal on the move and owns those cells.

use std::path::{Path, PathBuf};
use std::process::Output;

use crate::support::trial_corpus::{State, TrialCorpus};

/// The plants: one file at the area root, one nested a directory deep, one beside the
/// staged prose under `docs/`. Each is a distinct cell of the complement's shape — a root
/// file, a foreign **directory** (returned whole, never walked), and an entry inside the
/// one registry member that is a tree with its own rule.
const PLANTS: &[(&str, &str)] = &[
    ("NOTES.md", "keep me — the agent's own scratch\n"),
    ("analysis/perf.txt", "p99 = 41ms\n"),
    ("docs/notes.txt", "not a staged doc id\n"),
];

/// The complement's **entries**, as a door names them: the nested plant's unit is its
/// directory, because `engine::state::foreign_area_paths` returns a foreign directory whole.
const ENTRIES: &[&str] = &["NOTES.md", "analysis", "docs/notes.txt"];

/// **Arm 3's plants** (M53 Increment 2 / T2) — exactly two entries, one at the area root and
/// one under `docs/`, so the number the narration prints is unambiguous: the area holds 2 and
/// the move takes 1. Their names say which side of the cell each is on.
const PARTIAL_PLANTS: &[(&str, &str)] = &[
    ("root-foreign.txt", "the entry whose move lands\n"),
    ("docs/non-md.txt", "the entry whose move cannot\n"),
];

/// A landed finalize with the plants in place, driven at `format`.
struct Landed {
    corpus: TrialCorpus,
    task: String,
    out: Output,
}

impl Landed {
    fn stderr(&self) -> String {
        String::from_utf8_lossy(&self.out.stderr).into_owned()
    }

    fn stdout(&self) -> String {
        String::from_utf8_lossy(&self.out.stdout).into_owned()
    }

    fn repo(&self) -> PathBuf {
        self.corpus.repo()
    }
}

/// Mint a task, optionally plant the foreign bytes in its working area, give the commit a
/// real diff to land, and finalize at `format`.
///
/// The code change is deliberate: a commit-only task with nothing staged blocks at
/// `finalize.empty-commit`, so without it this suite would assert about an arm that never
/// lands.
fn landed_finalize(plant: bool, format: &str) -> Landed {
    landed_finalize_with(if plant { PLANTS } else { &[] }, None, format)
}

/// [`landed_finalize`]'s core, with the parking home's state as a parameter: `occupy` is a
/// path under `.jigc/displaced/<task-id>/` to put a **regular file** at before the finalize
/// runs, which is how the partial cell is manufactured without permission games — the entry
/// whose parking parent it occupies cannot land, every other entry can.
fn landed_finalize_with(plants: &[(&str, &str)], occupy: Option<&str>, format: &str) -> Landed {
    let corpus = TrialCorpus::build(State::Fresh);
    let task = corpus.start_workflow("quick-fix", "Cap the retry budget");
    let area = corpus.repo().join(".jigc").join("tasks").join(&task);

    for (rel, bytes) in plants {
        let at = area.join(rel);
        std::fs::create_dir_all(at.parent().expect("a plant has a parent"))
            .expect("create the plant's parent");
        std::fs::write(&at, bytes).expect("write the plant");
    }

    if let Some(rel) = occupy {
        let at = corpus
            .repo()
            .join(".jigc")
            .join("displaced")
            .join(&task)
            .join(rel);
        std::fs::create_dir_all(at.parent().expect("the occupant has a parent"))
            .expect("create the parking home");
        std::fs::write(&at, "not a directory\n").expect("occupy the parking path");
    }

    // A real diff for the commit to carry — the index the agent stages, not the area.
    std::fs::write(corpus.repo().join("README.md"), "hello\nretry cap = 3\n")
        .expect("edit the tracked file");
    corpus.git(&["add", "README.md"]);

    corpus.jigc_ok(&[
        "doc",
        "set-field",
        &format!("commit:{task}#type"),
        "--value",
        "fix",
        "--task",
        &task,
    ]);
    corpus.jigc_ok(&[
        "doc",
        "set-field",
        &format!("commit:{task}#scope"),
        "--value",
        "cli",
        "--task",
        &task,
    ]);
    corpus.set_slot(&format!("commit:{task}#summary"), &task, "cap the retries");
    corpus.set_slot(
        &format!("commit:{task}#body"),
        &task,
        "Driven by the displacement suite.",
    );

    let out = corpus.jigc(&["task", "finalize", &task, "--format", format]);
    assert!(
        out.status.success(),
        "the finalize must LAND — this suite asserts about the landed arm;\n\
         --- stdout ---\n{}\n--- stderr ---\n{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr),
    );
    Landed { corpus, task, out }
}

/// The `committed.displaced` array of a landed `--format json` document.
fn displaced_key(landed: &Landed) -> Vec<(String, String)> {
    let document = landed.stdout();
    let value: serde_json::Value =
        serde_json::from_str(&document).expect("the landed document is one JSON value");
    let rows = value["committed"]["displaced"]
        .as_array()
        .unwrap_or_else(|| {
            panic!(
                "`committed.displaced` must be present ALWAYS — a driver cannot read a key \
                 that appears only when something moved;\ngot:\n{document}"
            )
        })
        .clone();
    rows.iter()
        .map(|row| {
            (
                row["from"].as_str().expect("`from` is a string").to_owned(),
                row["to"].as_str().expect("`to` is a string").to_owned(),
            )
        })
        .collect()
}

/// The repo-relative spelling of `path` under `repo`.
fn rel(repo: &Path, path: &Path) -> String {
    path.strip_prefix(repo)
        .expect("under the repo")
        .to_string_lossy()
        .replace('\\', "/")
}

/// **Arm 1, the populated cell on the machine surface** — the three plants survive the
/// landed finalize under `.jigc/displaced/<task-id>/`, byte-intact and with their relative
/// paths preserved; every move is on the envelope and on stderr; the working area is gone
/// and no task is left active.
#[test]
fn the_foreign_bytes_survive_the_landed_finalize_and_the_envelope_names_each_move() {
    let landed = landed_finalize(true, "json");
    let repo = landed.repo();
    let task = &landed.task;
    let area = repo.join(".jigc").join("tasks").join(task);
    let home = repo.join(".jigc").join("displaced").join(task);

    // The bytes: every plant is where it was planted, relative path preserved.
    for (plant, bytes) in PLANTS {
        assert!(
            !area.join(plant).exists(),
            "the working area is torn down, so `{plant}` is not there any more",
        );
        let kept = home.join(plant);
        assert_eq!(
            std::fs::read_to_string(&kept).unwrap_or_else(|err| panic!(
                "`{plant}` must survive at `.jigc/displaced/{task}/{plant}` — the door that \
                 cannot ask for consent moves the bytes aside instead of taking them ({err})"
            )),
            *bytes,
            "`{plant}` survives BYTE-intact, never re-rendered",
        );
    }
    assert!(
        !area.exists(),
        "the teardown still runs — a left-over working area makes `jigc task list` report \
         an active task that finalized",
    );

    // The envelope: one `{from, to}` pair per complement ENTRY, sorted by `from`.
    let rows = displaced_key(&landed);
    let expected: Vec<(String, String)> = {
        let mut pairs: Vec<(String, String)> = ENTRIES
            .iter()
            .map(|entry| (rel(&repo, &area.join(entry)), rel(&repo, &home.join(entry))))
            .collect();
        pairs.sort();
        pairs
    };
    assert_eq!(
        rows,
        expected,
        "`committed.displaced` carries one repo-relative `{{from, to}}` per moved ENTRY, \
         sorted by `from` — the nested plant rides its own directory, which is the unit a \
         door names;\nstdout:\n{}",
        landed.stdout(),
    );

    // The side channel: the document owns stdout undiluted, and the moves are named on
    // stderr — each pair, both halves.
    let stderr = landed.stderr();
    for (from, to) in &expected {
        assert!(
            stderr.contains(from) && stderr.contains(to),
            "stderr names every move it made ({from} → {to}); stderr:\n{stderr}",
        );
    }

    // Nothing is left active: the id the finalize landed is not a task any more.
    let listed = landed.corpus.jigc_ok(&["task", "list"]);
    assert!(
        !listed.contains(task.as_str()),
        "the landed task is gone from `jigc task list`; got:\n{listed}",
    );
}

/// **Arm 1b, the same cell on the text surface** — the narration is the *same* channel on
/// both formats, because it is a loss-shaped side channel and not part of either document.
#[test]
fn the_text_surface_names_the_same_moves_on_stderr() {
    let landed = landed_finalize(true, "agent");
    let repo = landed.repo();
    let area = repo.join(".jigc").join("tasks").join(&landed.task);
    let home = repo.join(".jigc").join("displaced").join(&landed.task);
    let stderr = landed.stderr();

    for entry in ENTRIES {
        assert!(
            stderr.contains(&rel(&repo, &area.join(entry)))
                && stderr.contains(&rel(&repo, &home.join(entry))),
            "the agent-text run names `{entry}`'s move on stderr too; stderr:\n{stderr}",
        );
    }
    for (plant, bytes) in PLANTS {
        assert_eq!(
            std::fs::read_to_string(home.join(plant)).unwrap_or_default(),
            *bytes,
            "`{plant}` survives on the text surface as well",
        );
    }
}

/// **Arm 2, the omitting cell** — a working area holding nothing but jigc's own files
/// displaces nothing: the key is present and empty, stderr says nothing about moving
/// anything, and `.jigc/displaced/` is never even created.
#[test]
fn a_task_with_no_foreign_byte_displaces_nothing_and_says_nothing() {
    let json = landed_finalize(false, "json");
    assert_eq!(
        displaced_key(&json),
        Vec::<(String, String)>::new(),
        "an ordinary task moves nothing — `displaced` is present and EMPTY;\nstdout:\n{}",
        json.stdout(),
    );
    assert!(
        !json.repo().join(".jigc").join("displaced").exists(),
        "nothing foreign, nothing parked: the home is not even created",
    );

    let text = landed_finalize(false, "agent");
    let stderr = text.stderr();
    assert!(
        !stderr.contains("displaced") && !stderr.contains("moved"),
        "the ordinary path says nothing about displacement on either surface; stderr:\n{stderr}",
    );
}

/// **Arm 3, the partial cell** (M53 Increment 2 / T2;
/// `completions/artifacts/M53/settle-record.md` → D2.1/D2.2;
/// `completions/artifacts/M53/baseline-a3-2-failed-displacement.md` §4.1) — the move axis has
/// a **third** value between *all* and *none*: some entries move and some do not, reachable
/// with no permission game at all by occupying one entry's parking parent with a regular file.
///
/// Driven at `73b6ac0e`, that cell announced itself as *the working area held 1 entry* over an
/// area that held **two** — the count came from `moved.len()`, so the door under-reported its
/// own subject by exactly the entry whose move had failed. The narration's count is the
/// **complement's**, and the entry that did not move is named with the reason it did not,
/// beside the one that did.
///
/// `committed.displaced` is unmoved by this: the key is declared as what was **moved**, so it
/// still carries that pair alone.
#[test]
fn a_partial_move_names_what_it_could_not_move_and_counts_the_whole_area() {
    let landed = landed_finalize_with(PARTIAL_PLANTS, Some("docs"), "json");
    let repo = landed.repo();
    let task = &landed.task;
    let area = repo.join(".jigc").join("tasks").join(task);
    let home = repo.join(".jigc").join("displaced").join(task);

    let moved_from = rel(&repo, &area.join("root-foreign.txt"));
    let moved_to = rel(&repo, &home.join("root-foreign.txt"));
    let unmoved = rel(&repo, &area.join("docs").join("non-md.txt"));

    // The narration, read as the one block it is: both halves of the area's complement are
    // inside it, not scattered across notes a reader has to total up.
    let stderr = landed.stderr();
    let narration = stderr
        .find("note: the working area held")
        .map(|at| stderr[at..].to_owned())
        .unwrap_or_else(|| {
            panic!("a partial move still narrates the area it displaced from;\nstderr:\n{stderr}")
        });
    assert!(
        narration.contains("held 2 entries"),
        "the count is the AREA's complement — it held 2 — never `moved.len()`: a door that \
         reports one entry over an area that held two under-reports its own subject by the \
         entry whose move failed;\nstderr:\n{stderr}",
    );
    assert!(
        narration.contains(&moved_from) && narration.contains(&moved_to),
        "the narration still names the move it made ({moved_from} → {moved_to});\nstderr:\n{stderr}",
    );
    assert!(
        narration.contains(&unmoved),
        "the narration names the entry it could NOT move ({unmoved}) — the half the count was \
         hiding;\nstderr:\n{stderr}",
    );
    assert!(
        narration.contains("File exists"),
        "…with the reason that move failed, so the operator is not left to guess which of the \
         parking home's two failure points it hit;\nstderr:\n{stderr}",
    );

    // The envelope is keyed on what MOVED and says so, so it carries the one pair alone.
    assert_eq!(
        displaced_key(&landed),
        vec![(moved_from, moved_to)],
        "`committed.displaced` is declared as the moves that landed — a failed move is not a \
         `{{from, to}}` pair, and this key does not change;\nstdout:\n{}",
        landed.stdout(),
    );
}
