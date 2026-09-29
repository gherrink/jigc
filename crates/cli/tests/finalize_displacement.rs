//! M52 Increment 4 / T3 — **`jigc task finalize` keeps what it cannot commit**
//! (`completions/artifacts/M52/settle-record.md` → D3.3 as amended by §8;
//! `design/team-ready-state.md` → The working area's two populations;
//! `design/storage.md` → the per-task working area).
//!
//! Driven at `256b3e0` on the debug binary, the exact cell below: a `NOTES.md` at a task
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
//! **T3 closes the gap arm 3 opened** (M53 Increment 2 / T3). At T2 the entry whose move
//! failed was still taken by the teardown a statement later, and this doc said so rather than
//! asserting over it, because an assertion there would have pinned the loss as expected
//! output. The removal is now conditioned on the move, and the cells below are the axis of
//! that claim:
//!
//!   4. **the `{all · some · none} × {clean · fault on the pin · fault on a later member}`
//!      axis**, seven reachable coordinates driven one named test each, with the two a
//!      filesystem cannot produce declared in [`UNREACHABLE`] with their reason. Every planted
//!      byte's survival is claimed by a real `grep` over the repository against a
//!      before-control, and each cell asserts exactly one advisory per area left standing, on
//!      the landed envelope's `findings`, with the pinned envelope shape unmoved.
//!   5. **the hook-writes-during-the-commit cell**, which is why the advisory's subject is a
//!      re-read taken after the unwind and not the move's failure set: there the displacement
//!      reports **zero** failures and the area survives anyway.
//!   6. **§13's zero-false-fire controls, end to end** — an ordinary task lifecycle and a
//!      migrate task land, unwind whole and mint nothing.

use std::path::{Path, PathBuf};
use std::process::Output;

use crate::support::trial_corpus::{
    FOREIGN_VISION, FOREIGN_VISION_PATH, MIGRATED_VISION_PAYLOAD, State, TrialCorpus,
};

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
/// Driven at `d84ad037`, that cell announced itself as *the working area held 1 entry* over an
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

// ---------------------------------------------------------------------------
// M53 — the pre-v1 usability batch, row 6: the cwd is not the root these paths
// are spelled against
// ---------------------------------------------------------------------------

/// **Arm 4 — every Displace surface stays repo-relative from a branch-attached linked
/// worktree** (the rc.19 per-axis review's `(3, F-A)`).
///
/// `render::repo_relative` falls back to the honest absolute when its strip fails, so a
/// workbench path spelled against the *standing* checkout does not error — it silently
/// prints the host filesystem. Driven at `609da011`, running the identical populated cell
/// from a linked worktree gave `"from": "/private/var/folders/…/repo/.jigc/tasks/…"` on the
/// **1.0-pinned** `committed.displaced` key, the same absolutes in the stderr note, and the
/// same again inside `finalize.foreign-bytes`' message and route. `milestone finalize` was
/// clean in the identical cell — the rc.18 post-review MEDIUM 1 swept that door and this one
/// was left — which is why the class here is *the arguments the calls are written with* and
/// the source scan in `commit_seam_posture.rs` is its other half.
///
/// The worktree is **branch-attached** on purpose: a provisioned fan-out worktree is
/// detached by construction and the door refuses `repo.head-detached` before any
/// displacement runs, so that cwd cannot reach this surface at all.
///
/// Both cells are driven, because they are two different renderers over the same paths: the
/// all-move cell owns the envelope key and the *moved aside* note, the none-move cell owns
/// `finalize.foreign-bytes`' message **and** its route. The control is the same fixture from
/// the repository root, asserted in the same run — so a green here cannot come from a
/// scan that would pass anywhere.
#[test]
fn every_displace_surface_is_repo_relative_from_a_linked_worktree() {
    for occupy in [None, Some("")] {
        let corpus = TrialCorpus::build(State::Fresh);
        let task = corpus.start_workflow("quick-fix", "Cap the retry budget");
        let area = corpus.repo().join(".jigc").join("tasks").join(&task);
        std::fs::write(area.join("notes.txt"), "planted\n").expect("write the plant");

        // `Some("")` occupies `.jigc/displaced/<task>` itself with a regular file, so the
        // parking home cannot be created and NOTHING moves — the arm that raises
        // `finalize.foreign-bytes` and leaves the area standing.
        if occupy.is_some() {
            let displaced = corpus.repo().join(".jigc").join("displaced");
            std::fs::create_dir_all(&displaced).expect("the displaced root");
            std::fs::write(displaced.join(&task), "not a directory\n").expect("occupy");
        }

        corpus.jigc_ok(&[
            "doc",
            "set-field",
            &format!("commit:{task}#type"),
            "--value",
            "fix",
            "--task",
            &task,
        ]);
        corpus.set_slot(&format!("commit:{task}#summary"), &task, "cap the retries");

        // A branch-attached linked worktree, with the commit's own diff staged in ITS index
        // (a linked worktree carries its own index — staging in the main checkout would
        // leave this finalize with nothing to commit).
        let linked = corpus.repo().join("..").join("linked");
        corpus.git(&[
            "worktree",
            "add",
            "-q",
            "-b",
            "feat",
            linked.to_str().expect("utf-8 worktree path"),
        ]);
        std::fs::write(linked.join("work.txt"), "y\n").expect("write the staged change");
        std::process::Command::new("git")
            .args(["add", "work.txt"])
            .current_dir(&linked)
            .env("HOME", corpus.home())
            .output()
            .expect("stage in the linked worktree");

        let out = corpus.jigc_stdin_from(
            &linked,
            &["--format", "json", "task", "finalize", &task],
            "",
        );
        let stdout = String::from_utf8_lossy(&out.stdout).to_string();
        let stderr = String::from_utf8_lossy(&out.stderr).to_string();
        assert!(
            out.status.success(),
            "the finalize must LAND from the linked worktree;\nstdout:\n{stdout}\n\
             stderr:\n{stderr}",
        );

        // The host prefix this corpus lives under — the string that must appear on NO
        // surface. Asserted as a before-control: if the fixture's own root were not in
        // either stream to begin with, the scan below would be vacuous.
        let host = corpus
            .repo()
            .canonicalize()
            .expect("canonicalize the repo")
            .to_string_lossy()
            .to_string();
        assert!(
            !host.is_empty() && host.starts_with('/'),
            "the control needs a real absolute prefix; got `{host}`",
        );

        for (surface, printed) in [("the envelope", &stdout), ("the narration", &stderr)] {
            assert!(
                !printed.contains(&host),
                "`{surface}` spells a workbench path against the cwd's checkout — the \
                 host-absolute leak `(3, F-A)` names (occupy: {occupy:?});\n{printed}",
            );
        }
        assert!(
            stderr.contains(".jigc/tasks/") && stderr.contains("notes.txt"),
            "…and it still NAMES the entry, repo-relative (occupy: {occupy:?});\n{stderr}",
        );

        let value: serde_json::Value =
            serde_json::from_str(&stdout).expect("the landed document is one JSON value");
        match occupy {
            None => {
                let rows = value["committed"]["displaced"]
                    .as_array()
                    .expect("`committed.displaced` is always present")
                    .clone();
                assert_eq!(rows.len(), 1, "the one plant moved;\n{stdout}");
                for key in ["from", "to"] {
                    let path = rows[0][key].as_str().expect("a string");
                    assert!(
                        path.starts_with(".jigc/"),
                        "`committed.displaced[].{key}` is repo-relative on the 1.0-pinned \
                         envelope; got `{path}`",
                    );
                }
            }
            Some(_) => {
                let finding = value["findings"]
                    .as_array()
                    .and_then(|rows| {
                        rows.iter()
                            .find(|f| f["code"] == "finalize.foreign-bytes")
                            .cloned()
                    })
                    .unwrap_or_else(|| {
                        panic!("the none-move cell raises `finalize.foreign-bytes`;\n{stdout}")
                    });
                for key in ["message", "route"] {
                    let text = finding[key].as_str().unwrap_or_default();
                    assert!(
                        !text.contains(&host),
                        "`finalize.foreign-bytes`'s `{key}` carries a host path;\n{text}",
                    );
                }
                assert!(
                    finding["message"]
                        .as_str()
                        .unwrap_or_default()
                        .contains(".jigc/tasks/"),
                    "…and still names the area, repo-relative;\n{finding:#}",
                );
            }
        }
    }
}

// ---------------------------------------------------------------------------
// M53 Increment 2 / T3 — the removal is conditioned on the move
// ---------------------------------------------------------------------------

/// The marker every planted byte carries, so the survival claim is made by `grep` over the
/// whole repository rather than by looking where the test expects the bytes to be — a
/// before-control runs the identical scan before the finalize, so a green cannot come from a
/// scan that finds nothing anywhere.
const KEEP_MARKER: &str = "JIGC-M53-KEEP";

/// A plant at the **area root**: the depth every door has reached since M52.
const ROOT_PLANT: (&str, &str) = ("root-kept.txt", "JIGC-M53-KEEP root\n");

/// A plant one directory down, whose complement **entry** is `analysis` — a foreign
/// directory moves whole, so its subtree rides its parent's move.
const NESTED_PLANT: (&str, &str) = ("analysis/perf.txt", "JIGC-M53-KEEP nested\n");

/// A plant inside the registry's one tree member, whose own rule decides entry by entry.
const DOCS_PLANT: (&str, &str) = ("docs/non-md.txt", "JIGC-M53-KEEP under docs\n");

/// How much of the area's complement reached `.jigc/displaced/<unit-id>/`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Moves {
    /// Every entry moved.
    All,
    /// Some moved and some did not — the value between *all* and *none*, and the one the
    /// door's count was blind to until T2.
    Partial,
    /// Not one entry could be parked.
    None,
}

/// What [`engine::state::unwind_area`] did with jigc's **own** members.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Unwind {
    /// Every member removed; the area survives only if something else is still in it.
    Clean,
    /// The removal failed on `TASK_AREA_FILES[0]`, the base pin — so the area stands
    /// **whole**, pin included, and is still a task at every by-id door.
    FaultOnPin,
    /// The removal failed on a later member, after the pin was already gone.
    FaultOnLater,
}

/// One coordinate of the manufactured axis `{all · partial · none} × {clean · fault on the
/// pin · fault on a later member}`, with the manufacture it needs.
///
/// **The space is manufactured and says so**: the two failure points are decided (the pin is
/// member 0 of the registry row; `docs/` is member 1), and the move axis is a property of the
/// parking home's state, which no registry enumerates.
struct Cell {
    /// `<moves> × <unwind>`, the coordinate's own name.
    name: &'static str,
    moves: Moves,
    unwind: Unwind,
    /// What this cell plants into the working area.
    plants: &'static [(&'static str, &'static str)],
    /// A path under `.jigc/displaced/` to occupy with a **regular file** before the finalize
    /// runs — `""` is the unit's parking home itself (so no entry's parent can be created),
    /// `"docs"` is one entry's parent (so exactly that entry's move cannot land). `None`
    /// leaves the parking home free.
    occupy: Option<&'static str>,
}

/// **The reachable coordinates.** Seven of the nine: see [`UNREACHABLE`] for the two that a
/// filesystem cannot be made to produce and why.
const CELLS: &[Cell] = &[
    Cell {
        name: "all move × unwind clean",
        moves: Moves::All,
        unwind: Unwind::Clean,
        plants: &[ROOT_PLANT, NESTED_PLANT],
        occupy: None,
    },
    Cell {
        name: "some move × unwind clean",
        moves: Moves::Partial,
        unwind: Unwind::Clean,
        plants: &[ROOT_PLANT, DOCS_PLANT],
        occupy: Some("docs"),
    },
    Cell {
        name: "none move × unwind clean",
        moves: Moves::None,
        unwind: Unwind::Clean,
        plants: &[ROOT_PLANT, NESTED_PLANT],
        occupy: Some(""),
    },
    Cell {
        name: "all move × fault on a later member",
        moves: Moves::All,
        unwind: Unwind::FaultOnLater,
        plants: &[ROOT_PLANT, NESTED_PLANT],
        occupy: None,
    },
    Cell {
        name: "some move × fault on a later member",
        moves: Moves::Partial,
        unwind: Unwind::FaultOnLater,
        // No parking game: the `docs/` plant's move fails because `docs/` is the directory
        // the fault is made in, and a rename needs the SOURCE directory writable.
        plants: &[ROOT_PLANT, DOCS_PLANT],
        occupy: None,
    },
    Cell {
        name: "none move × fault on a later member",
        moves: Moves::None,
        unwind: Unwind::FaultOnLater,
        plants: &[ROOT_PLANT, NESTED_PLANT],
        occupy: Some(""),
    },
    Cell {
        name: "none move × fault on the pin",
        moves: Moves::None,
        unwind: Unwind::FaultOnPin,
        // No parking game either: an area that cannot be written to is an area no entry can
        // be renamed OUT of, which is the same bit the pin's removal needs.
        plants: &[ROOT_PLANT, NESTED_PLANT],
        occupy: None,
    },
];

/// **The two coordinates no filesystem produces**, named rather than silently absent — the
/// axis is only honest if its gaps are stated.
const UNREACHABLE: &[(&str, &str)] = &[
    (
        "all move × fault on the pin",
        "the pin's `remove_file` and every entry's `fs::rename` out of the area draw on the \
         SAME bit — write permission on the area — so an area whose pin cannot be removed is \
         an area no entry can leave",
    ),
    (
        "some move × fault on the pin",
        "the same bit, for the same reason: with the area writable the pin goes, and without \
         it no entry moves, so `some` and `fault on the pin` cannot hold at once",
    ),
];

/// Every file under `repo` whose contents carry [`KEEP_MARKER`], repo-relative and sorted —
/// the survival scan, run through the real `grep` so the claim is not made by the same code
/// that placed the bytes.
fn marker_files(repo: &Path) -> Vec<String> {
    let out = std::process::Command::new("grep")
        .args(["-rlE", KEEP_MARKER, "."])
        .current_dir(repo)
        .output()
        .expect("run grep");
    let mut found: Vec<String> = String::from_utf8_lossy(&out.stdout)
        .lines()
        .map(|line| line.trim_start_matches("./").to_owned())
        .filter(|line| !line.is_empty())
        .collect();
    found.sort();
    found
}

/// Set `path`'s mode, for the permission games the fault cells need.
fn set_mode(path: &Path, mode: u32) {
    let mut perms = std::fs::metadata(path)
        .unwrap_or_else(|err| panic!("stat {}: {err}", path.display()))
        .permissions();
    std::os::unix::fs::PermissionsExt::set_mode(&mut perms, mode);
    std::fs::set_permissions(path, perms)
        .unwrap_or_else(|err| panic!("chmod {}: {err}", path.display()));
}

/// One driven cell: the corpus, the task, the landed run, and the marker scan taken
/// **before** it.
struct Driven {
    corpus: TrialCorpus,
    task: String,
    out: Output,
    before: Vec<String>,
}

/// Drive `cell` through a real landed `jigc task finalize --format json`.
///
/// The fault is made by a **`pre-commit` hook**, because it has to happen inside the
/// transaction: the finalize writes its message temp file into this very area before the
/// commit, so a mode set beforehand would fail the run instead of the teardown.
fn drive_task_cell(cell: &Cell) -> Driven {
    let corpus = TrialCorpus::build(State::Fresh);
    let task = corpus.start_workflow("quick-fix", "Cap the retry budget");
    let repo = corpus.repo();
    let area = repo.join(".jigc").join("tasks").join(&task);

    for (rel, bytes) in cell.plants {
        let at = area.join(rel);
        std::fs::create_dir_all(at.parent().expect("a plant has a parent"))
            .expect("create the plant's parent");
        std::fs::write(&at, bytes).expect("write the plant");
    }

    if let Some(rel) = cell.occupy {
        let home = repo.join(".jigc").join("displaced").join(&task);
        let at = if rel.is_empty() { home } else { home.join(rel) };
        std::fs::create_dir_all(at.parent().expect("the occupant has a parent"))
            .expect("create the parking home");
        std::fs::write(&at, "not a directory\n").expect("occupy the parking path");
    }

    let faulted = match cell.unwind {
        Unwind::Clean => None,
        Unwind::FaultOnPin => Some(area.clone()),
        Unwind::FaultOnLater => Some(area.join("docs")),
    };
    if let Some(target) = &faulted {
        let hook = repo.join(".git").join("hooks").join("pre-commit");
        std::fs::write(
            &hook,
            format!(
                "#!/bin/sh\nchmod 0555 {}\nexit 0\n",
                target.to_string_lossy()
            ),
        )
        .expect("install the faulting hook");
        set_mode(&hook, 0o755);
    }

    std::fs::write(repo.join("README.md"), "hello\nretry cap = 3\n").expect("edit the file");
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
        "Driven by the displacement axis.",
    );

    let before = marker_files(&repo);
    assert_eq!(
        before.len(),
        cell.plants.len(),
        "[{}] the before-control must FIND the plants — a scan that finds nothing before \
         proves nothing after; got {before:?}",
        cell.name,
    );

    let out = corpus.jigc(&["task", "finalize", &task, "--format", "json"]);
    // Hand the write bits back before anything else reads or removes the tree.
    if faulted.is_some() {
        set_mode(&area, 0o755);
        let docs = area.join("docs");
        if docs.exists() {
            set_mode(&docs, 0o755);
        }
    }
    Driven {
        corpus,
        task,
        out,
        before,
    }
}

/// The advisory's code — the one identity this pass mints.
const KEPT_AREA_CODE: &str = "finalize.foreign-bytes";

/// How many `finalize.foreign-bytes` findings the landed `--format json` envelope carries.
fn kept_area_findings(stdout: &str) -> usize {
    let value: serde_json::Value =
        serde_json::from_str(stdout).expect("the landed document is one JSON value");
    value["findings"]
        .as_array()
        .expect("the landed envelope carries a `findings` array")
        .iter()
        .filter(|finding| finding["code"].as_str() == Some(KEPT_AREA_CODE))
        .count()
}

/// The landed envelope's top-level keys, sorted — the pinned `ENVELOPE_ARMS` shape.
fn envelope_keys(stdout: &str) -> Vec<String> {
    let value: serde_json::Value =
        serde_json::from_str(stdout).expect("the landed document is one JSON value");
    let mut keys: Vec<String> = value
        .as_object()
        .expect("the landed document is an object")
        .keys()
        .cloned()
        .collect();
    keys.sort();
    keys
}

/// The shared assertion every task-door cell makes.
fn assert_task_cell(cell: &Cell) {
    let driven = drive_task_cell(cell);
    let repo = driven.corpus.repo();
    let stdout = String::from_utf8_lossy(&driven.out.stdout).into_owned();
    let stderr = String::from_utf8_lossy(&driven.out.stderr).into_owned();
    let area = repo.join(".jigc").join("tasks").join(&driven.task);

    assert!(
        driven.out.status.success(),
        "[{}] the commit is truth and the teardown is best-effort, so the run lands at exit \
         0 whatever the area does;\nstdout:\n{stdout}\nstderr:\n{stderr}",
        cell.name,
    );

    // The survival claim, by `grep` over the whole repository, against the before-control.
    let after = marker_files(&repo);
    assert_eq!(
        after.len(),
        driven.before.len(),
        "[{}] every planted byte is still on disk after the landed commit — moved aside or \
         left standing, never taken; before {:?}, after {after:?}\nstderr:\n{stderr}",
        cell.name,
        driven.before,
    );

    // The cell's own MOVE coordinate, asserted rather than assumed: a manufacture that
    // silently stopped working (an occupancy that no longer blocks, a mode that no longer
    // binds) would otherwise let a fault cell pass while testing a different coordinate.
    let moved: usize = serde_json::from_str::<serde_json::Value>(&stdout)
        .expect("the landed document is one JSON value")["committed"]["displaced"]
        .as_array()
        .expect("`committed.displaced` is present always")
        .len();
    let expected_moves = match cell.moves {
        // Each plant is exactly one complement ENTRY here — the nested one rides its own
        // directory — so the pair count is the plant count.
        Moves::All => cell.plants.len(),
        // The root plant lands; the one inside the tree member does not.
        Moves::Partial => 1,
        Moves::None => 0,
    };
    assert_eq!(
        moved, expected_moves,
        "[{}] the cell's move coordinate must really be the one it claims;\nstdout:\n{stdout}",
        cell.name,
    );

    // Exactly one advisory per area left standing, and none for an area that went.
    let standing = !(cell.moves == Moves::All && cell.unwind == Unwind::Clean);
    assert_eq!(
        area.exists(),
        standing,
        "[{}] an area jigc could not empty of a third party's bytes is LEFT; one it emptied \
         is gone;\nstderr:\n{stderr}",
        cell.name,
    );
    assert_eq!(
        kept_area_findings(&stdout),
        usize::from(standing),
        "[{}] exactly one `{KEPT_AREA_CODE}` per area left standing, on the landed \
         envelope's `findings`;\nstdout:\n{stdout}",
        cell.name,
    );

    // The pinned envelope shape does not move for the advisory.
    assert_eq!(
        envelope_keys(&stdout),
        vec![
            "committed".to_string(),
            "findings".to_string(),
            "schema_version".to_string()
        ],
        "[{}] the landed `task finalize` envelope stays `Object(&[\"committed\", \
         \"findings\", \"schema_version\"])`;\nstdout:\n{stdout}",
        cell.name,
    );

    // The pin's fate is the difference between the two fault cells, and it is what a by-id
    // door reads: whole area with its pin, or a residual.
    if standing {
        let pin = area.join("base.json").exists();
        assert_eq!(
            pin,
            cell.unwind == Unwind::FaultOnPin,
            "[{}] a fault ON the pin leaves the area whole; a clean unwind and a fault on a \
             LATER member both take the pin first",
            cell.name,
        );
    }
}

/// **The axis's gaps are named.** A coordinate a filesystem cannot produce is declared with
/// its reason, so the seven driven cells read as seven of nine rather than as the whole.
#[test]
fn the_unreachable_coordinates_are_named_with_their_reason() {
    assert_eq!(
        CELLS.len() + UNREACHABLE.len(),
        9,
        "the manufactured space is `{{all · partial · none}} × {{clean · fault on the pin · \
         fault on a later member}}` — nine coordinates, each either driven or declared \
         unreachable",
    );
    for (name, why) in UNREACHABLE {
        assert!(
            !why.trim().is_empty(),
            "`{name}` must carry the reason it cannot be manufactured",
        );
        assert!(
            !CELLS.iter().any(|cell| cell.name == *name),
            "`{name}` is declared unreachable and driven — it is one or the other",
        );
    }
}

#[test]
fn task_area_all_move_unwind_clean() {
    assert_task_cell(&CELLS[0]);
}

#[test]
fn task_area_some_move_unwind_clean() {
    assert_task_cell(&CELLS[1]);
}

#[test]
fn task_area_none_move_unwind_clean() {
    assert_task_cell(&CELLS[2]);
}

#[test]
fn task_area_all_move_fault_on_a_later_member() {
    assert_task_cell(&CELLS[3]);
}

#[test]
fn task_area_some_move_fault_on_a_later_member() {
    assert_task_cell(&CELLS[4]);
}

#[test]
fn task_area_none_move_fault_on_a_later_member() {
    assert_task_cell(&CELLS[5]);
}

#[test]
fn task_area_none_move_fault_on_the_pin() {
    assert_task_cell(&CELLS[6]);
}

/// **The hook-writes-during-the-commit cell** (M53 Increment 2 / T3;
/// `completions/artifacts/M53/settle-record.md` → §6) — the cell that makes the advisory's
/// subject a **post-unwind re-read** rather than the displacement's failure set.
///
/// A succeeding `pre-commit` hook writes into the working area *during* the commit and leaves
/// it unreadable. Phase 7's displacement then enumerates **nothing**, so it reports zero moves
/// and, decisively, **zero move failures** — and the area still survives its own teardown,
/// because `unwind_area` removes jigc's members by name and the hook's byte is not one of
/// them. A finding composed from `Displacement::unmoved` would be silent here about an area
/// full of somebody else's bytes; a finding composed from `AreaUnwind::Foreign` would name no
/// path at all, the variant carrying none.
///
/// Where the re-read cannot be made either — as here, the same permission denying both — the
/// message says **that**, rather than asserting an absence it did not check.
#[test]
fn a_hook_write_during_the_commit_survives_the_teardown_that_cannot_enumerate_it() {
    let corpus = TrialCorpus::build(State::Fresh);
    let task = corpus.start_workflow("quick-fix", "Cap the retry budget");
    let repo = corpus.repo();
    let area = repo.join(".jigc").join("tasks").join(&task);

    std::fs::write(area.join(ROOT_PLANT.0), ROOT_PLANT.1).expect("plant before the run");

    let hook = repo.join(".git").join("hooks").join("pre-commit");
    std::fs::write(
        &hook,
        // The marker is assembled at run time from two halves, so the hook's OWN source is
        // not a match for the survival scan below.
        format!(
            "#!/bin/sh\nprintf 'JIGC-M53%sKEEP hook\\n' '-' > {}/hook-wrote.txt\n\
             chmod 0333 {}\nexit 0\n",
            area.to_string_lossy(),
            area.to_string_lossy(),
        ),
    )
    .expect("install the writing hook");
    set_mode(&hook, 0o755);

    std::fs::write(repo.join("README.md"), "hello\nretry cap = 3\n").expect("edit the file");
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
        "Driven by the hook cell.",
    );

    let before = marker_files(&repo);
    assert_eq!(
        before.len(),
        1,
        "only the pre-planted byte is visible before the run — the hook's write happens \
         inside the commit; got {before:?}",
    );

    let out = corpus.jigc(&["task", "finalize", &task, "--format", "json"]);
    set_mode(&area, 0o755);
    let stdout = String::from_utf8_lossy(&out.stdout).into_owned();
    let stderr = String::from_utf8_lossy(&out.stderr).into_owned();

    assert!(
        out.status.success(),
        "the commit lands at exit 0;\nstdout:\n{stdout}\nstderr:\n{stderr}",
    );
    assert!(
        !stderr.contains("could not be moved"),
        "the displacement reports ZERO move failures here — it never enumerated an entry to \
         fail on, which is exactly why the advisory cannot be composed from that set;\n\
         stderr:\n{stderr}",
    );
    assert_eq!(
        kept_area_findings(&stdout),
        1,
        "…and the area survives its own teardown anyway, so one `{KEPT_AREA_CODE}` names \
         it;\nstdout:\n{stdout}",
    );
    assert!(
        area.exists() && area.join("hook-wrote.txt").exists(),
        "the hook's write is still on disk — `unwind_area` removes jigc's members by name, \
         so a byte it did not write survives by construction",
    );
    let after = marker_files(&repo);
    assert_eq!(
        after.len(),
        2,
        "both bytes are there: the one planted before the run and the one the hook wrote \
         inside it; got {after:?}",
    );
}

/// **§13's three zero-false-fire controls, driven end to end** — through a real landed
/// `jigc task finalize`, not only through `engine::state::unwind_area`
/// (`crates/cli/tests/finalize_area_unwind_controls.rs` owns that half, M53 Increment 2 / T1).
///
/// Two of them are here — an ordinary task lifecycle and a **migrate** task, whose
/// `source`/`source-path` members no other door writes. The third, an ordinary post-join
/// milestone area including `merged/`, is discharged where its fan-out fixture lives
/// (`crates/cli/tests/milestone_boundary_displacement.rs` →
/// `a_landed_boundary_with_no_foreign_byte_carries_an_empty_displaced_key`).
///
/// The regression they exist to catch is the opposite of every cell above: `unwind_area`
/// answering `Foreign` over an **ordinary** area would leave every finalized task's working
/// area standing and mint this advisory on every landed finalize.
#[test]
fn the_section_13_controls_mint_no_advisory_end_to_end() {
    // (a) an ordinary task lifecycle — the one that reaches `roles.json` (the create gate's
    //     binding) and `renames.json`, the member a hand-drawn registry missed.
    let corpus = TrialCorpus::build(State::Fresh);
    let task = corpus.start_workflow("record-decision", "record a decision about caching");
    corpus.jigc_ok(&[
        "doc",
        "create",
        "adr",
        "--title",
        "Cache the thing",
        "--task",
        &task,
    ]);
    corpus.jigc_ok(&[
        "doc",
        "rename",
        "adr:cache-the-thing",
        "--to",
        "Cache the other thing",
        "--task",
        &task,
    ]);
    // Report-only: an unfilled ADR has its own content findings, and that verdict is not
    // this control's subject — the snapshots the run lands in the area are.
    let _ = corpus.jigc(&["task", "validate", &task]);
    for slot in ["context", "options", "decision", "consequences"] {
        corpus.set_slot(
            &format!("adr:cache-the-other-thing#{slot}"),
            &task,
            "Weighed, and this is the call.",
        );
    }
    let area = corpus.repo().join(".jigc").join("tasks").join(&task);
    for member in ["roles.json", "renames.json", "docs"] {
        assert!(
            area.join(member).exists(),
            "the lifecycle must really reach `{member}`, else this control proves nothing",
        );
    }
    corpus.jigc_ok(&[
        "doc",
        "set-field",
        &format!("commit:{task}#type"),
        "--value",
        "docs",
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
    corpus.set_slot(
        &format!("commit:{task}#summary"),
        &task,
        "record the caching decision",
    );
    corpus.set_slot(
        &format!("commit:{task}#body"),
        &task,
        "Driven by the zero-false-fire control.",
    );
    let landed = corpus.jigc(&["task", "finalize", &task, "--format", "json"]);
    assert_control(&landed, &area, "an ordinary task lifecycle");
    assert!(
        !corpus.repo().join(".jigc").join("displaced").exists(),
        "an ordinary task lifecycle parks nothing: the home is not even created",
    );

    // (b) a migrate task — `engine::state::SOURCE_FILE` and `SOURCE_PATH_FILE` are written by
    //     no other door, so its area is a shape (a) never produces.
    let migrate = TrialCorpus::build(State::Fresh);
    let foreign = migrate.repo().join(FOREIGN_VISION_PATH);
    std::fs::create_dir_all(foreign.parent().expect("the source has a parent"))
        .expect("mk the source dir");
    std::fs::write(&foreign, FOREIGN_VISION).expect("write the foreign source");
    migrate.git(&["add", FOREIGN_VISION_PATH]);
    migrate.git(&["commit", "-q", "-m", "add the direction doc"]);
    migrate.jigc_ok(&["migrate", FOREIGN_VISION_PATH, "--as", "vision"]);
    let tasks = migrate.repo().join(".jigc").join("tasks");
    let minted: Vec<String> = std::fs::read_dir(&tasks)
        .expect("read the task root")
        .map(|entry| {
            entry
                .expect("a task dir")
                .file_name()
                .to_string_lossy()
                .into_owned()
        })
        .collect();
    assert_eq!(
        minted.len(),
        1,
        "the migrate mint is this corpus's only task"
    );
    let migrate_task = &minted[0];
    let migrate_area = tasks.join(migrate_task);
    for member in [engine::state::SOURCE_FILE, engine::state::SOURCE_PATH_FILE] {
        assert!(
            migrate_area.join(member).exists(),
            "a migration area must really carry `{member}`, else this is control (a) again",
        );
    }
    migrate.jigc_stdin_ok(
        &[
            "doc",
            "author",
            "vision",
            "--from-file",
            "-",
            "--task",
            migrate_task,
        ],
        MIGRATED_VISION_PAYLOAD,
    );
    migrate.jigc_ok(&[
        "doc",
        "set-field",
        &format!("commit:{migrate_task}#type"),
        "--value",
        "docs",
        "--task",
        migrate_task,
    ]);
    migrate.jigc_ok(&[
        "doc",
        "set-field",
        &format!("commit:{migrate_task}#scope"),
        "--value",
        "vision",
        "--task",
        migrate_task,
    ]);
    migrate.set_slot(
        &format!("commit:{migrate_task}#summary"),
        migrate_task,
        "migrate the direction doc",
    );
    migrate.set_slot(
        &format!("commit:{migrate_task}#body"),
        migrate_task,
        "Driven by the zero-false-fire control.",
    );
    let landed = migrate.jigc(&[
        "task",
        "finalize",
        migrate_task,
        "--approve",
        "--format",
        "json",
    ]);
    assert_control(&landed, &migrate_area, "a migrate task");
}

/// The control's shared predicate: the run landed, the area is **gone**, and no advisory was
/// minted about it.
fn assert_control(landed: &Output, area: &Path, control: &str) {
    let stdout = String::from_utf8_lossy(&landed.stdout).into_owned();
    let stderr = String::from_utf8_lossy(&landed.stderr).into_owned();
    assert!(
        landed.status.success(),
        "{control}: the control must LAND;\nstdout:\n{stdout}\nstderr:\n{stderr}",
    );
    assert!(
        !area.exists(),
        "{control}: an area holding nothing but jigc's own writes unwinds WHOLE — a `Foreign` \
         here leaves every finalized task standing;\nstderr:\n{stderr}",
    );
    assert_eq!(
        kept_area_findings(&stdout),
        0,
        "{control}: …and mints no `{KEPT_AREA_CODE}` at all;\nstdout:\n{stdout}",
    );
}
