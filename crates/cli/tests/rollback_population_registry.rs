//! M52 Increment 5 / T1 — **the capture/restore class is a code-side registry, and membership
//! is a counted source scan** (`completions/artifacts/M52/settle-record.md` → D1.1 as amended
//! by §1, §2, §14 and §19;
//! [baseline-rollback.md](../../../completions/artifacts/M52/baseline-rollback.md) §1).
//!
//! **The gap this closes.** Nobody owned the enumeration of the places jigc puts bytes back,
//! so the populations disagreed with each other about the same cell inside one binary — the
//! config layer's absent-pre-image arm deletes only while the file still holds jigc's bytes,
//! the milestone record's identical arm deletes unconditionally — and the review that looked
//! at the class counted **four** where the source has **nine**. A fix cut against a
//! hand-drawn list is a fix cut short, which is this wave's subject.
//!
//! **Two arms, each naming the kind of set it iterates.**
//!
//!   * **(a) the counted source scan** — a *counted* scan on the `repo_relative_paths.rs`
//!     mold, because **no clap tree bijects this class**: a restore is a property of a
//!     function body, not of a verb. Every production **restore unit** (a function whose name
//!     carries the family's vocabulary, plus every production `Drop` body that restores —
//!     the two shapes a name-only grep misses; `impl Drop for RecordFlipGuard` was a claimed
//!     unit under that rule until M52 Increment 5 / T5 moved its restore onto the shared
//!     compare-and-swap, which left the destructor with no byte-restoring call to count) is
//!     claimed by a registry row or sits in a counted remainder with a reason; and each
//!     claimed unit's byte-restoring **calls** are counted against the rows that own them, so
//!     a unit hosting two populations (`rollback_promotions`: promote **and** retire) fences
//!     each of them separately. Mutant-proven both ways — a restore added without a row
//!     reddens, a row whose site is deleted reddens, and a row deleted from a unit it shares
//!     reddens on the count.
//!   * **(b) the discipline leg** — every row's discipline is one of the four, and the two
//!     that name something (`DoorGuard`'s guard code, `Declared`'s reason **and** its
//!     reopening trigger) name something the source actually has. A row's doors are its
//!     `cli::VERB_KINDS` leaf paths, so a door that is not a verb cannot be written down.
//!
//! **Declared bounds.** (i) The scan reads **names and bodies**, not semantics: a production
//! function that restores bytes under a name carrying none of the family's vocabulary is
//! outside its reach, and no static reader can close that — what closes it is the driven
//! acceptance of T3–T8. (ii) `cli::rollback`'s `park` writes bytes and is deliberately *not*
//! a restore unit: it parks a pre-image into the gitignored workbench, which is the opposite
//! act. (iii) The count leg is a **partition of calls**, not a proof that a call restores;
//! the disposition tables carry the reason for every call the rows do not own.

use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};

use cli::rollback::{Discipline, Population, ROLLBACK_POPULATIONS, Site};

use super::support::rust_source;

// ---------------------------------------------------------------------------
// the scan
// ---------------------------------------------------------------------------

/// Both crate source trees — the class spans them (`CreatedDoc::rollback` is the engine's).
const CRATE_SRC: &[&str] = &["crates/engine/src", "crates/cli/src"];

/// What makes a production function a **restore unit**: the vocabulary the family is written
/// in. Deliberately narrower than the baseline's exploratory grep, which also swept `capture`,
/// `pre_image` and `displace` — a capture is not a restore, and a displacement is a move *out*.
const RESTORE_FN_MARKERS: &[&str] = &["rollback", "restore", "unwind"];

/// **What "puts bytes back" is, mechanically** — the calls the count leg partitions. The last
/// member is git's own restore verb as it appears in an argv array, which is how the two
/// HEAD-sourced arms restore; it is read over string-preserving source for that reason.
const RESTORE_PRIMITIVES: &[&str] = &[
    "fs::write(",
    "fs::remove_file(",
    "fs::remove_dir_all(",
    "fs::remove_dir(",
    "state::persist(",
    "\"restore\"",
];

/// One production restore unit: where it is and how many byte-restoring calls it holds.
#[derive(Debug)]
struct Unit {
    file: String,
    name: String,
    line: usize,
    prims: usize,
}

fn workspace_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("..").join("..")
}

/// The `{…}` body span of the **function** declared at `decl` — the parameter list is skipped
/// by paren-matching first, so a `->` return type carrying braces cannot be mistaken for it.
fn body_span(code: &str, decl: usize) -> Option<(usize, usize)> {
    let open_paren = code[decl..].find('(').map(|i| decl + i)?;
    let close_paren = matching(code, open_paren, b'(', b')')?;
    let open = code[close_paren..].find('{').map(|i| close_paren + i)?;
    let close = matching(code, open, b'{', b'}')?;
    Some((open, close))
}

/// The `{…}` body span of the **item** declared at `decl`, taken from its first brace — an
/// `impl` block has no parameter list, so [`body_span`]'s paren skip would walk straight past
/// the impl's own brace into the first method's and report that instead.
fn item_body_span(code: &str, decl: usize) -> Option<(usize, usize)> {
    let open = code[decl..].find('{').map(|i| decl + i)?;
    let close = matching(code, open, b'{', b'}')?;
    Some((open, close))
}

/// The index of the delimiter closing the one at `open`, over blanked code.
fn matching(code: &str, open: usize, lhs: u8, rhs: u8) -> Option<usize> {
    let bytes = code.as_bytes();
    let mut depth = 0i32;
    for (i, b) in bytes.iter().enumerate().skip(open) {
        if *b == lhs {
            depth += 1;
        } else if *b == rhs {
            depth -= 1;
            if depth == 0 {
                return Some(i);
            }
        }
    }
    None
}

/// Every production restore unit in `path`.
///
/// Two readings of the same bytes, because the two questions live in opposite places: the
/// declarations and the brace matching are **code** (a doc comment naming `rollback_rename`
/// is not a declaration), and the primitive count needs the **literals** kept (`"restore"`
/// exists only as one). Both blankings preserve byte offsets, so the spans line up.
fn restore_units(path: &Path) -> Vec<Unit> {
    let body = fs::read_to_string(path).expect("read source");
    let code = rust_source::code_only(&body);
    let with_strings = rust_source::code_and_strings(&body);
    let regions = rust_source::cfg_test_regions(&code);
    let file = path
        .strip_prefix(workspace_root())
        .unwrap_or(path)
        .to_string_lossy()
        .replace('\\', "/");

    let mut units = Vec::new();
    let push = |name: String, decl: usize, units: &mut Vec<Unit>| {
        let Some((open, close)) = body_span(&code, decl) else {
            return;
        };
        let region = &with_strings[open..close];
        let prims = RESTORE_PRIMITIVES
            .iter()
            .map(|marker| region.matches(marker).count())
            .sum();
        units.push(Unit {
            file: file.clone(),
            name,
            line: body[..decl].lines().count(),
            prims,
        });
    };

    // Named restore functions.
    for (at, _) in code.match_indices("fn ") {
        if rust_source::is_test_domain(path, &regions, at) {
            continue;
        }
        let tail = &code[at + "fn ".len()..];
        let name: String = tail
            .chars()
            .take_while(|c| c.is_alphanumeric() || *c == '_')
            .collect();
        if name.is_empty() || !RESTORE_FN_MARKERS.iter().any(|m| name.contains(m)) {
            continue;
        }
        push(name, at, &mut units);
    }

    // `Drop` bodies that restore — the shape a name-only scan is blind to, because the
    // function is always called `drop`. A teardown that removes jigc's own scratch matches
    // too, and is dispositioned rather than excluded by a cleverer predicate: fail-closed is
    // the direction a fence over a data-loss class has to fail in.
    for (at, _) in code.match_indices("impl Drop for ") {
        if rust_source::is_test_domain(path, &regions, at) {
            continue;
        }
        let tail = &code[at + "impl Drop for ".len()..];
        let ty: String = tail
            .chars()
            .take_while(|c| c.is_alphanumeric() || *c == '_')
            .collect();
        let Some((open, close)) = item_body_span(&code, at) else {
            continue;
        };
        let Some(drop_at) = code[open..close].find("fn drop").map(|i| open + i) else {
            continue;
        };
        let before = units.len();
        push(format!("{ty}::drop"), drop_at, &mut units);
        // A `Drop` that restores nothing is not a restore site.
        if units.len() > before && units[before].prims == 0 {
            units.pop();
        }
    }

    units
}

/// Every production restore unit across both crates, keyed `(file, unit)`.
///
/// **A key collision is a fence failure, not a last-writer-wins.** `(file, name)` is the
/// coordinate a registry row writes down, so two restore units sharing a name in one file
/// would make the key ambiguous and the second would silently replace the first — a
/// restore the fence then reports as claimed while nothing claims it. Fail-open is the one
/// direction a fence over a data-loss class may not fail in, so the collision panics with
/// both lines rather than being resolved.
fn scanned_units() -> BTreeMap<(String, String), Unit> {
    let mut all: BTreeMap<(String, String), Unit> = BTreeMap::new();
    for dir in CRATE_SRC {
        for path in rust_source::rust_files(&workspace_root().join(dir)) {
            for unit in restore_units(&path) {
                let key = (unit.file.clone(), unit.name.clone());
                if let Some(first) = all.get(&key) {
                    panic!(
                        "{}::{} is declared twice (lines {} and {}) — the scan keys a unit \
                         by `(file, name)`, so the second would silently replace the first \
                         and its restore would be claimed by a row that means the other \
                         one. Give one of them a distinguishing name, or teach the scan a \
                         key that separates them.",
                        key.0, key.1, first.line, unit.line,
                    );
                }
                all.insert(key, unit);
            }
        }
    }
    all
}

// ---------------------------------------------------------------------------
// the counted remainder
// ---------------------------------------------------------------------------

/// Scanned units that are **not** rollback populations — by file, unit, the byte-restoring
/// calls they hold, and the reason. A unit may not sit here and be claimed by a row.
///
/// Four kinds sit here, and each is its own answer rather than one word for four: the
/// **index axis** (five populations, all `git update-index`-keyed — no byte of a worktree
/// file is written, so none of them can take a third party's edit); the **conflict
/// surface** the `FileCas` discipline produces; a `Drop` that tears down **jigc's own
/// throwaway scratch**, which the fail-closed `Drop` rule sweeps in on purpose — a teardown
/// that removes a file jigc created seconds earlier in a temp dir restores nothing; and the
/// **`MintedSet` sink** (M52 Increment 5 / T7), which is an *implementation* of a discipline
/// rather than a population of its own — the two rows that reach it are `milestone-mint-area`
/// and `unrecorded-seed-areas`, and it lives in the engine because that is where the registry
/// deciding which files are jigc's already lives.
const NOT_A_POPULATION: &[(&str, &str, usize, &str)] = &[
    (
        "crates/engine/src/state.rs",
        "unwind_area",
        3,
        "the `MintedSet` sink: it removes the area's own `engine::state::WorkArea` row and \
         then the directory non-recursively, so a third party's file survives by \
         construction. It puts no byte back — the two rows that call it do, and their \
         restores are counted at their own units",
    ),
    (
        "crates/engine/src/state.rs",
        "unwind_docs",
        2,
        "the same sink's `docs/` arm, where the staged instances and the provenance manifest \
         are removed entry by entry under `TASK_DOCS_FILES` + `staged_doc_id` — the one \
         member of a `WorkArea` row whose shape is a tree",
    ),
    (
        "crates/cli/src/task.rs",
        "rollback_owner_artifact_index",
        0,
        "the index axis: `git update-index --cacheinfo` / `--force-remove` per captured \
         entry, and the worktree is never touched",
    ),
    (
        "crates/cli/src/task.rs",
        "rollback_config_layer_index",
        0,
        "the index axis, plus the set difference that drops what the stage added — again \
         `git update-index` only",
    ),
    (
        "crates/cli/src/rollback.rs",
        "rollback_conflict_finding",
        0,
        "the `FileCas` discipline's *output*: the blocking finding a raced restore raises, \
         carrying the door's own injected code and noun since T2. It composes a message; it \
         puts no bytes anywhere",
    ),
    (
        "crates/cli/src/task.rs",
        "carry_rollback_conflicts",
        0,
        "the carrier that prints those findings beside the door's frame and folds their \
         codes into the invocation log",
    ),
    (
        "crates/cli/src/task.rs",
        "fold_rollback_conflicts",
        0,
        "the carrier's document half, which puts the same findings into the `--format json` \
         document",
    ),
    (
        "crates/cli/src/combine.rs",
        "TempIndex::drop",
        1,
        "teardown of jigc's own throwaway `GIT_INDEX_FILE` under the system temp dir — a \
         file this type created, removed by the type that created it",
    ),
    (
        "crates/cli/src/task.rs",
        "CombineIndex::drop",
        1,
        "the same throwaway-index teardown at the finalize-side combine overlay",
    ),
    (
        "crates/cli/src/task.rs",
        "ScratchTree::drop",
        1,
        "teardown of the scratch worktree the combine materializes — jigc's own tree, \
         wholesale, under a process-unique temp path",
    ),
];

/// Byte-restoring calls **inside a claimed unit** that belong to a different axis, so the
/// unit's count and its rows' counts can agree without a row pretending to own them.
///
/// Both members are the **index** axis reached through git's own `restore` verb rather than
/// through `update-index` — `--staged` with no `--worktree`, which moves an index entry and
/// no byte on disk.
const OTHER_AXIS_CALLS: &[(&str, &str, usize, &str)] = &[
    (
        "crates/cli/src/relocate.rs",
        "rollback_relocations",
        1,
        "`git restore --staged` un-stages both sides of each landed `git mv`; the bytes at \
         those two paths are the compare-and-swap's subject",
    ),
    (
        "crates/cli/src/task.rs",
        "rollback_promotions",
        1,
        "`git restore --staged` un-stages a retirement's deletion, and only when jigc's own \
         stage staged it",
    ),
    (
        "crates/cli/src/rename.rs",
        "rollback_rename",
        1,
        "`git restore --staged <new path>` un-stages the move's landing; the landing's \
         worktree copy is `rename-worktree`'s",
    ),
];

// ---------------------------------------------------------------------------
// (a) membership
// ---------------------------------------------------------------------------

#[test]
fn every_restore_unit_is_a_registry_row_or_a_counted_remainder() {
    let scan = scanned_units();
    assert!(
        !scan.is_empty(),
        "the scan found no restore unit at all — it has stopped reading the source",
    );

    let mut offenders: Vec<String> = Vec::new();

    // Rows → source: a row that names a site the source does not have is a row whose
    // population has been deleted or renamed out from under it.
    let mut claimed: BTreeMap<(String, String), (usize, Vec<&str>)> = BTreeMap::new();
    for row in ROLLBACK_POPULATIONS {
        let Site::Source {
            file,
            unit,
            restores,
        } = row.site
        else {
            continue;
        };
        let key = (file.to_string(), unit.to_string());
        if !scan.contains_key(&key) {
            offenders.push(format!(
                "  row `{}` names {file}::{unit}, which the scan does not find",
                row.id,
            ));
            continue;
        }
        let entry = claimed.entry(key).or_default();
        entry.0 += restores;
        entry.1.push(row.id);
    }

    // Source → rows: every scanned unit is claimed, or counted out with a reason.
    for (key, unit) in &scan {
        let remainder = NOT_A_POPULATION
            .iter()
            .find(|(file, name, _, _)| *file == key.0 && *name == key.1);
        match (claimed.get(key), remainder) {
            (Some(_), Some(_)) => offenders.push(format!(
                "  {}::{} is both a registry row and a counted remainder — one of the two is \
                 a lie",
                key.0, key.1,
            )),
            (None, None) => offenders.push(format!(
                "  {}::{} (line {}) restores in {} place(s) and no row claims it — add a \
                 `ROLLBACK_POPULATIONS` row, or a `NOT_A_POPULATION` entry with the reason \
                 it is not one",
                key.0, key.1, unit.line, unit.prims,
            )),
            (None, Some((_, _, prims, reason))) => {
                if reason.trim().is_empty() {
                    offenders.push(format!("  {}::{} carries no reason", key.0, key.1));
                }
                if unit.prims != *prims {
                    offenders.push(format!(
                        "  {}::{} is counted out at {prims} restoring call(s), the source has \
                         {}",
                        key.0, key.1, unit.prims,
                    ));
                }
            }
            (Some((owned, ids)), None) => {
                let other: usize = OTHER_AXIS_CALLS
                    .iter()
                    .filter(|(file, name, _, _)| *file == key.0 && *name == key.1)
                    .map(|(_, _, count, _)| *count)
                    .sum();
                if owned + other != unit.prims {
                    offenders.push(format!(
                        "  {}::{} holds {} byte-restoring call(s); its row(s) {ids:?} own \
                         {owned} and {other} are counted onto another axis — the sum must be \
                         the source's, or a restore is unaccounted for",
                        key.0, key.1, unit.prims,
                    ));
                }
            }
        }
    }

    // A remainder row whose unit is gone is a stale disposition.
    for (file, name, _, _) in NOT_A_POPULATION {
        if !scan.contains_key(&(file.to_string(), name.to_string())) {
            offenders.push(format!(
                "  the counted remainder names {file}::{name}, which the scan does not find",
            ));
        }
    }
    // The other-axis count reads only inside a **claimed** unit, so an entry whose unit is
    // gone, or one sitting on a unit the remainder already counts whole, is a count nothing
    // reads — and a reason nothing measures is a sentence (`repo_relative_paths.rs`).
    for (file, name, _, reason) in OTHER_AXIS_CALLS {
        let key = (file.to_string(), name.to_string());
        if !scan.contains_key(&key) {
            offenders.push(format!(
                "  the other-axis count names {file}::{name}, which the scan does not find",
            ));
        } else if !claimed.contains_key(&key) {
            offenders.push(format!(
                "  the other-axis count names {file}::{name}, which no registry row \
                 claims — its calls are already counted whole by the remainder, so \
                 this entry is read by nothing",
            ));
        }
        if reason.trim().is_empty() {
            offenders.push(format!(
                "  the other-axis count at {file}::{name} carries no reason"
            ));
        }
    }

    assert!(
        offenders.is_empty(),
        "the rollback class and the source disagree — every place jigc puts bytes back is a \
         registry row with a discipline, or a counted remainder with a reason:\n{}",
        offenders.join("\n"),
    );
}

// ---------------------------------------------------------------------------
// (b) the disciplines
// ---------------------------------------------------------------------------

/// The registry's own home, **excluded** from the literal reader below.
///
/// A row may not be its own witness: the guard code a `DoorGuard` row carries is itself a
/// string literal in this file, so a reader that swept the whole crate would resolve
/// `DoorGuard("rename.no-such-guard")` against the row that invented it and pass. Caught by
/// running exactly that mutant, which the fence let through before this line existed.
const REGISTRY_HOME: &str = "crates/cli/src/rollback.rs";

/// Every string literal in production source, both crates minus [`REGISTRY_HOME`] — the
/// reader that resolves a `DoorGuard` row's guard code to the place that **mints** it.
fn production_literals() -> String {
    let mut all = String::new();
    let home = workspace_root().join(REGISTRY_HOME);
    for dir in CRATE_SRC {
        for path in rust_source::rust_files(&workspace_root().join(dir)) {
            if path == home {
                continue;
            }
            let body = fs::read_to_string(&path).expect("read source");
            all.push_str(&rust_source::code_and_strings(&body));
            all.push('\n');
        }
    }
    all
}

fn door_paths() -> Vec<&'static [&'static str]> {
    cli::cli::VERB_KINDS.iter().map(|(path, _)| *path).collect()
}

#[test]
fn every_row_names_a_discipline_and_something_the_source_has() {
    let literals = production_literals();
    let doors = door_paths();
    let mut offenders: Vec<String> = Vec::new();
    let mut ids: Vec<&str> = Vec::new();
    let mut siteless: Vec<&str> = Vec::new();

    for Population {
        id,
        subject,
        doors: row_doors,
        site,
        discipline,
    } in ROLLBACK_POPULATIONS
    {
        ids.push(id);
        if subject.trim().is_empty() {
            offenders.push(format!("  `{id}` says nothing about what it puts back"));
        }
        if row_doors.is_empty() {
            offenders.push(format!("  `{id}` names no door"));
        }
        for door in *row_doors {
            if !doors.contains(door) {
                offenders.push(format!(
                    "  `{id}` names the door {door:?}, which is no `VERB_KINDS` leaf",
                ));
            }
        }
        match site {
            Site::Source { .. } => {}
            Site::NoRestore { reason, cited } => {
                siteless.push(id);
                if reason.trim().is_empty() || cited.trim().is_empty() {
                    offenders.push(format!(
                        "  `{id}` has no restore site and does not say why, or does not cite \
                         where that is decided",
                    ));
                }
            }
        }
        match discipline {
            Discipline::FileCas => {}
            Discipline::MintedSet(areas) => {
                if areas.is_empty() {
                    offenders.push(format!(
                        "  `{id}` is a `MintedSet` row naming no working area — the set it \
                         removes would be empty",
                    ));
                }
            }
            Discipline::DoorGuard(code) => {
                if !literals.contains(&format!("\"{code}\"")) {
                    offenders.push(format!(
                        "  `{id}` is guarded by `{code}`, which no production source mints",
                    ));
                }
            }
            Discipline::Declared {
                reason,
                reopens_when,
            } => {
                if reason.trim().is_empty() {
                    offenders.push(format!("  `{id}` is declared with no reason"));
                }
                if reopens_when.trim().is_empty() {
                    offenders.push(format!(
                        "  `{id}` is declared with no reopening trigger — a declared row \
                         whose reason can go stale unnoticed is the shape §19 refused",
                    ));
                }
            }
        }
    }

    let mut sorted = ids.clone();
    sorted.sort_unstable();
    sorted.dedup();
    if sorted.len() != ids.len() {
        offenders.push("  two rows share an id".to_string());
    }
    if siteless.len() != 1 {
        offenders.push(format!(
            "  {} row(s) carry no restore site ({siteless:?}); exactly ONE is admitted, \
             because the record names it — `jigc setup`'s guard-by-refusal. It was two until \
             M52 Increment 5 / T8, when the `config set <root-knob>` relocation stopped being \
             rollback-less and took a site like every other `FileCas` row",
            siteless.len(),
        ));
    }

    assert!(
        offenders.is_empty(),
        "a registry row must carry one of the four disciplines and name things the source \
         has:\n{}",
        offenders.join("\n"),
    );
}
