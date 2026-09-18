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
//! **Four arms, each naming the kind of set it iterates.** The first two landed with the
//! registry at T1; the last two are M52 Increment 5 / **T10**, which turns every row's
//! discipline from a classification a reader has to trust into a fact the source binds, and
//! closes the `<door>.rollback-conflict` family at the five doors §19 decided.
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
//!   * **(b) the row shape** — every row says what it puts back, which doors reach it, and
//!     where its restore lives. A row's doors are its `cli::VERB_KINDS` leaf paths, so a door
//!     that is not a verb cannot be written down, and exactly one row may carry no restore
//!     site at all.
//!   * **(c) the binding totality fence (T10)** — every row's **discipline** is bound by the
//!     source, per discipline and over the whole registry: a `FileCas` row's restore reaches
//!     T2's generic entry (`cli::rollback::PreImageFamily::restore`), a `MintedSet` row's
//!     unwind reaches T7's sink (`engine::state::unwind_area`), a `DoorGuard` row names a
//!     guard production source mints, and a `Declared` row carries a reason **and** its
//!     reopening trigger. The discipline match is exhaustive by the compiler, so a fifth
//!     kind cannot compile until someone says what would bind it. Mutant-proven: a row's
//!     restore switched to an unconditional `fs::write` reddens, an unwind that stops reading
//!     the area registry reddens both `MintedSet` rows — the second through the one hop the
//!     walk allows — and a `Declared` row stripped of its trigger reddens.
//!   * **(d) the `<door>.rollback-conflict` family (T10)** — a **four-way ⇔** over the five
//!     doors: `cli::rollback::ROLLBACK_DOORS` ⇔ the `ConflictDoor` constants the registry
//!     home declares ⇔ the whole-literal `*.rollback-conflict` codes production source mints
//!     ⇔ the codes `design/validation.md`'s registration tables name — plus the floor,
//!     **driven** once per door at the production seam: the racer's bytes survive, the
//!     pre-image is parked under that door's own noun, and the finding is blocking, located
//!     at the raced path, `Human`-routed, serializable (where the route floor asserts) and
//!     **not** an `ERROR_CODE_REGISTRY` member. A sixth producer reddens wherever it is
//!     written, which is what makes §19's refusal of `doc-author.rollback-conflict` hold in
//!     code.
//!
//! **Declared bounds.** (i) The scan reads **names and bodies**, not semantics: a production
//! function that restores bytes under a name carrying none of the family's vocabulary is
//! outside its reach, and no static reader can close that — what closes it is the driven
//! acceptance of T3–T8. (ii) `cli::rollback`'s `park` writes bytes and is deliberately *not*
//! a restore unit: it parks a pre-image into the gitignored workbench, which is the opposite
//! act. (iii) The count leg is a **partition of calls**, not a proof that a call restores;
//! the disposition tables carry the reason for every call the rows do not own. (iv) Arm (d)
//! separates a **mint** from a **mention**: a code is minted when a production literal's
//! *whole value* is `<door>.rollback-conflict`, and named into the family when a registration
//! table's first column carries it. Prose naming a refused spelling is neither — which is how
//! the registry's own `Declared` row explains why `doc-author.rollback-conflict` is not
//! minted, and how the roadmap records that amendment, without either being a violation.

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

/// One production restore unit: where it is, how many byte-restoring calls it holds, and
/// **what its body says** — the last of those is what arm (c) reads, because *this row's
/// restore goes through the shared entry* is a fact about a call, not about a name.
#[derive(Debug)]
struct Unit {
    file: String,
    name: String,
    line: usize,
    prims: usize,
    body: String,
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
            body: region.to_string(),
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
// (b) the row shape
// ---------------------------------------------------------------------------

/// The registry's own home, **excluded** from the literal reader below.
///
/// A row may not be its own witness: the guard code a `DoorGuard` row carries is itself a
/// string literal in this file, so a reader that swept the whole crate would resolve
/// `DoorGuard("rename.no-such-guard")` against the row that invented it and pass. Caught by
/// running exactly that mutant, which the fence let through before this line existed.
///
/// The family arm (d) reads this home **deliberately**, for the opposite reason: the door
/// constants live here, so excluding it would leave that scan reading nothing. What keeps it
/// honest there is that its ⇔ runs against two things this file does not write — the codes
/// production source mints elsewhere, and the codes `design/validation.md` registers.
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

/// **A row says what it puts back, which doors reach it, and where its restore lives.**
///
/// The shape half of the registry; the *discipline* half — what each row's declared
/// discipline binds to in the source — is [`every_row_binds_the_discipline_it_declares`],
/// which is the one place it is checked (M52 Increment 5 / T10).
#[test]
fn every_row_names_a_subject_a_door_and_a_site() {
    let doors = door_paths();
    let mut offenders: Vec<String> = Vec::new();
    let mut ids: Vec<&str> = Vec::new();
    let mut siteless: Vec<&str> = Vec::new();

    for Population {
        id,
        subject,
        doors: row_doors,
        site,
        // The discipline's own payload is arm (c)'s subject — what the source **binds** —
        // and is deliberately checked in exactly one place.
        discipline: _,
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
        "a registry row must name what it puts back, the doors that reach it, and where its \
         restore lives:\n{}",
        offenders.join("\n"),
    );
}

// ---------------------------------------------------------------------------
// (c) the binding totality fence — every row's discipline is a checked fact
// ---------------------------------------------------------------------------

/// **The generic pre-image entry** every `FileCas` population's restore runs through (M52
/// Increment 5 / T2) — `cli::rollback::PreImageFamily::restore`, as the scan keys it.
const FILE_CAS_ENTRY: (&str, &str) = ("crates/cli/src/rollback.rs", "restore");

/// Its **call form**. Every caller holds a `PreImageFamily` value, so the compare-and-swap is
/// reached as a method call; the marker is unambiguous because [`the scan`](scanned_units)
/// asserts below that this workspace declares exactly one production unit named `restore`.
const FILE_CAS_CALL: &str = ".restore(";

/// **The `MintedSet` sink** (M52 Increment 5 / T7) — `engine::state::unwind_area`, which
/// removes the door's own area set and then the directory non-recursively, in its call form.
const MINTED_SET_SINK: &str = "unwind_area(";

/// Whether the restore unit at `from` **reaches** `marker` — in its own body, or through
/// another scanned restore unit it calls.
///
/// One hop is not a convenience: `unwind_unrecorded_seeds` unwinds each seed *through*
/// `unwind_mint`, so a body-only reader would report the row's discipline unbound while the
/// source binds it. The walk is closed over the scanned units alone (a call into anything
/// else is not a restore unit and cannot be the sink), and `seen` keeps a cycle finite.
fn reaches(
    units: &BTreeMap<(String, String), Unit>,
    from: &(String, String),
    marker: &str,
) -> bool {
    fn walk(
        units: &BTreeMap<(String, String), Unit>,
        at: &(String, String),
        marker: &str,
        seen: &mut BTreeMap<(String, String), ()>,
    ) -> bool {
        if seen.insert(at.clone(), ()).is_some() {
            return false;
        }
        let Some(unit) = units.get(at) else {
            return false;
        };
        if unit.body.contains(marker) {
            return true;
        }
        let callees: Vec<(String, String)> = units
            .keys()
            .filter(|key| key != &at && unit.body.contains(&format!("{}(", key.1)))
            .cloned()
            .collect();
        callees
            .iter()
            .any(|callee| walk(units, callee, marker, seen))
    }
    walk(units, from, marker, &mut BTreeMap::new())
}

/// **Every row's discipline is a fact the source binds, not a classification it declares**
/// (M52 Increment 5 / T10; `settle-record.md` → D1.1 as amended by §1, §2 and §19).
///
/// T1 shipped this registry with eight rows declaring a discipline the source did not yet
/// bind, and each later task retired one of those declarations. This arm is what makes the
/// last of them a *checked* fact — and what keeps a twelfth row from being added with a
/// discipline written down and nothing behind it:
///
///   * a **`FileCas`** row's restore provably reaches T2's generic entry — it *is* the entry,
///     or its unit calls it, so the compare-and-swap is the only way this population's bytes
///     go back. A restore switched to an unconditional `fs::write` reddens here (and again on
///     the count leg, which sees the new primitive);
///   * a **`MintedSet`** row's unwind provably reaches T7's sink, and names a non-empty area
///     set — a hand-list of "the mint's own files" is what drove short at `milestone create`;
///   * a **`DoorGuard`** row names a guard **production source mints**, resolved outside the
///     registry's own home so a row cannot be its own witness;
///   * a **`Declared`** row carries a reason **and** the condition that makes the reason
///     false (§19) — a declared row whose reason can go stale unnoticed is the shape the
///     Settle refused.
///
/// The match is exhaustive by the compiler, so a fifth discipline cannot compile until
/// someone says what binds it.
#[test]
fn every_row_binds_the_discipline_it_declares() {
    let scan = scanned_units();
    let literals = production_literals();
    let mut offenders: Vec<String> = Vec::new();

    // The two markers are only as good as their unambiguity, so both are asserted rather than
    // assumed: exactly one production unit is named `restore`, and it is the entry itself.
    let restores: Vec<&(String, String)> = scan.keys().filter(|key| key.1 == "restore").collect();
    assert_eq!(
        restores,
        vec![&(FILE_CAS_ENTRY.0.to_string(), FILE_CAS_ENTRY.1.to_string())],
        "`{FILE_CAS_CALL}` can only stand for the shared compare-and-swap while this \
         workspace declares exactly one production unit named `restore`; it declares \
         {restores:?}",
    );
    assert!(
        scan.contains_key(&(
            "crates/engine/src/state.rs".to_string(),
            "unwind_area".to_string()
        )),
        "the `MintedSet` sink `engine::state::unwind_area` is not in the scan — the marker \
         `{MINTED_SET_SINK}` would then be satisfiable by nothing, and every `MintedSet` row \
         would redden for the wrong reason",
    );

    for row in ROLLBACK_POPULATIONS {
        let site = match row.site {
            Site::Source { file, unit, .. } => Some((file.to_string(), unit.to_string())),
            // The one admitted site-less row (arm (b) fences that it is the only one): there
            // is no restore to bind, which is exactly what its `DoorGuard` discipline says.
            Site::NoRestore { .. } => None,
        };
        match row.discipline {
            Discipline::FileCas => match &site {
                Some(key)
                    if *key == (FILE_CAS_ENTRY.0.to_string(), FILE_CAS_ENTRY.1.to_string()) => {}
                Some(key) if reaches(&scan, key, FILE_CAS_CALL) => {}
                Some(key) => offenders.push(format!(
                    "  `{}` is a `FileCas` row whose unit {}::{} never reaches the shared \
                     compare-and-swap (`{}::{}`) — its bytes go back some other way, which is \
                     an unconditional restore however it is spelled",
                    row.id, key.0, key.1, FILE_CAS_ENTRY.0, FILE_CAS_ENTRY.1,
                )),
                None => offenders.push(format!(
                    "  `{}` is a `FileCas` row with no restore site at all",
                    row.id,
                )),
            },
            Discipline::MintedSet(areas) => {
                if areas.is_empty() {
                    offenders.push(format!(
                        "  `{}` is a `MintedSet` row naming no working area — the set it \
                         removes would be empty",
                        row.id,
                    ));
                }
                match &site {
                    Some(key) if reaches(&scan, key, MINTED_SET_SINK) => {}
                    Some(key) => offenders.push(format!(
                        "  `{}` is a `MintedSet` row whose unit {}::{} never reaches \
                         `engine::state::unwind_area` — a removal that does not read the \
                         area's own registry row is a removal that can take a third party's \
                         file",
                        row.id, key.0, key.1,
                    )),
                    None => offenders.push(format!(
                        "  `{}` is a `MintedSet` row with no unwind site at all",
                        row.id,
                    )),
                }
            }
            Discipline::DoorGuard(code) => {
                if !literals.contains(&format!("\"{code}\"")) {
                    offenders.push(format!(
                        "  `{}` is guarded by `{code}`, which no production source mints",
                        row.id,
                    ));
                }
            }
            Discipline::Declared {
                reason,
                reopens_when,
            } => {
                if reason.trim().is_empty() {
                    offenders.push(format!("  `{}` is declared with no reason", row.id));
                }
                if reopens_when.trim().is_empty() {
                    offenders.push(format!(
                        "  `{}` is declared with no reopening trigger — a declared row whose \
                         reason can go stale unnoticed is the shape §19 refused",
                        row.id,
                    ));
                }
            }
        }
    }

    assert!(
        offenders.is_empty(),
        "a row's discipline is what keeps its restore from destroying a third party's bytes, \
         so the source must bind it — declaring one is not carrying one:\n{}",
        offenders.join("\n"),
    );
}

// ---------------------------------------------------------------------------
// (d) the `<door>.rollback-conflict` family — five doors, and no sixth
// ---------------------------------------------------------------------------

/// The doc whose Severity inventory is where a minted finding is **registered**
/// (`completions/artifacts/M51/settle-record.md` → §10; the registration home
/// `config_layer_preimage.rs`' inventory arm already reads for the `finalize` member).
const INVENTORY_DOC: &str = "design/validation.md";

/// The suffix the family's identities share. A code is *minted* when a production string
/// literal's **whole value** is `<door>.rollback-conflict`; a code named inside a sentence is
/// a **mention**, which is how the registry's own `Declared` row explains why
/// `doc-author.rollback-conflict` is not minted (§19) and how the roadmap records that
/// amendment. Mint and mention are different acts, so the scan reads whole literals.
const CONFLICT_SUFFIX: &str = ".rollback-conflict";

/// A throwaway directory, removed on drop — the family arm drives the production seam against
/// real files, because *the racer's bytes survive* is a fact about a file and not about a type.
struct TempDir(PathBuf);

impl TempDir {
    fn new(tag: &str) -> Self {
        let mut path = std::env::temp_dir();
        path.push(format!(
            "jigc-rollback-family-{tag}-{}-{:?}",
            std::process::id(),
            engine::tempname::unique_nanos(),
        ));
        fs::create_dir_all(&path).expect("create temp dir");
        TempDir(path)
    }
}

impl Drop for TempDir {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}

/// Every whole-literal `*.rollback-conflict` code **production source mints**, both crates,
/// the registry's own home included — it is where the door constants live, so excluding it
/// would leave the scan reading nothing.
fn minted_conflict_codes() -> std::collections::BTreeSet<String> {
    let mut out = std::collections::BTreeSet::new();
    for dir in CRATE_SRC {
        for path in rust_source::rust_files(&workspace_root().join(dir)) {
            let body = fs::read_to_string(&path).expect("read source");
            let code = rust_source::code_only(&body);
            let regions = rust_source::cfg_test_regions(&code);
            for literal in rust_source::string_literals(&body) {
                if rust_source::is_test_domain(&path, &regions, literal.offset) {
                    continue;
                }
                if literal.value.ends_with(CONFLICT_SUFFIX)
                    && !literal.value[..literal.value.len() - CONFLICT_SUFFIX.len()]
                        .contains(|c: char| !c.is_ascii_lowercase() && c != '-')
                    && !literal.value.is_empty()
                {
                    out.insert(literal.value.clone());
                }
            }
        }
    }
    out
}

/// Every code a registration table in [`INVENTORY_DOC`] names in its **first column** — the
/// act of registering, as distinct from naming a code in prose.
fn registered_conflict_codes() -> std::collections::BTreeSet<String> {
    let doc = fs::read_to_string(workspace_root().join(INVENTORY_DOC))
        .unwrap_or_else(|err| panic!("read {INVENTORY_DOC}: {err}"));
    doc.lines()
        .map(str::trim)
        .filter(|line| line.starts_with('|'))
        .filter_map(|line| line.trim_start_matches('|').split('|').next())
        .flat_map(|cell| {
            cell.split('`')
                .skip(1)
                .step_by(2)
                .map(str::trim)
                .map(str::to_owned)
                .collect::<Vec<_>>()
        })
        .filter(|code| code.ends_with(CONFLICT_SUFFIX))
        .collect()
}

/// **The conflict family is the five doors the registry names — registered, route-floor
/// green, on the findings arm, and no sixth** (M52 Increment 5 / T10; `settle-record.md` →
/// §19, the human's Arm B).
///
/// Four legs, each closing a different way in:
///
///   1. **⇔ with the source's own door constants.** Every `ConflictDoor` declared at the
///      registry home is a `ROLLBACK_DOORS` member and every member is one of them, so a
///      sixth door cannot be declared and left out of the family — nor listed in the family
///      without being declared.
///   2. **⇔ with what production mints.** The whole-literal `*.rollback-conflict` codes the
///      binary carries are exactly the members' codes. A sixth producer reddens here
///      **wherever** it is written, which is what makes §19's refusal hold in code rather
///      than in a sentence: `doc-author.rollback-conflict` is mentioned by the `Declared`
///      row that explains why it is not minted, and a mention is not a mint.
///   3. **⇔ with the registration home.** Every member has a row in [`INVENTORY_DOC`]'s
///      registration tables and no non-member does — *named into the family* is a table row,
///      never a sentence, which is why the roadmap's own amendment bracket naming the refused
///      spelling is not a violation.
///   4. **Driven, per door: the floor.** The production seam is run over a raced path for
///      every member — the racer's bytes must survive, the pre-image must be parked under
///      that door's own noun, and the finding must be blocking, located at the raced path,
///      `Human`-routed and **serializable**, which is where the M43 route floor asserts. And
///      none of the five may be an `ERROR_CODE_REGISTRY` member: that registry mirrors door
///      **identities** derived from `COMMITTING_DOORS`, and a blocking `Finding` rides the
///      findings arm, never an `Outcome`'s flattened error identity (§10).
#[test]
fn the_rollback_conflict_family_is_five_doors_and_no_sixth() {
    use cli::rollback::{PreImage, PreImageFamily, ROLLBACK_DOORS};
    use engine::finding::{RouteKind, Severity};

    let codes: std::collections::BTreeSet<String> = ROLLBACK_DOORS
        .iter()
        .map(|door| door.code.to_string())
        .collect();
    assert_eq!(
        codes.len(),
        ROLLBACK_DOORS.len(),
        "two doors share a code — the family's whole point is that `(code, target)` \
         discriminates a raced door from its neighbour at the same path",
    );

    // 1 — the family and the source's door constants, in both directions.
    let home = fs::read_to_string(workspace_root().join(REGISTRY_HOME)).expect("read registry");
    let code_only = rust_source::code_only(&home);
    let declared: Vec<String> = code_only
        .match_indices(": ConflictDoor = ConflictDoor {")
        .filter_map(|(at, _)| {
            let head = &code_only[..at];
            let name_start = head.rfind(|c: char| c.is_whitespace())? + 1;
            Some(head[name_start..].to_string())
        })
        .collect();
    let listed_at = code_only
        .find("pub const ROLLBACK_DOORS: &[ConflictDoor] = &[")
        .expect("the registry home must declare `ROLLBACK_DOORS`");
    let listed_end = code_only[listed_at..]
        .find("];")
        .map(|off| listed_at + off)
        .expect("`ROLLBACK_DOORS` must be a closed array literal");
    let listed = &code_only[listed_at..listed_end];
    assert_eq!(
        declared.len(),
        ROLLBACK_DOORS.len(),
        "{REGISTRY_HOME} declares {} `ConflictDoor` constant(s) ({declared:?}) and \
         `ROLLBACK_DOORS` carries {} — a door declared outside the family is a door whose \
         code every fence over the family is blind to",
        declared.len(),
        ROLLBACK_DOORS.len(),
    );
    for name in &declared {
        assert!(
            listed.contains(name.as_str()),
            "`{name}` is a `ConflictDoor` the registry home declares and `ROLLBACK_DOORS` \
             does not list",
        );
    }

    // 2 — and what production actually mints.
    assert_eq!(
        minted_conflict_codes(),
        codes,
        "the `*.rollback-conflict` codes production source mints as whole literals must be \
         exactly the family's. A code minted with no door row is a sixth member nothing \
         fences; a member nothing mints is a claim the binary cannot make (settle-record \
         §19: `doc-author.rollback-conflict` is NOT minted — the `Declared` row mentions it \
         to say so, and a mention is not a mint)",
    );

    // 3 — and what the registration home registers.
    assert_eq!(
        registered_conflict_codes(),
        codes,
        "{INVENTORY_DOC}'s registration tables must name exactly the family's codes in their \
         first column — a member with no row is the silence §10 exists to end, and a row for \
         a code the binary never mints is how a refused spelling gets admitted by being \
         named into the family",
    );

    // 4 — the floor, driven at the production seam, once per door.
    for door in ROLLBACK_DOORS {
        assert!(
            !door.noun.is_empty()
                && !door.noun.contains(' ')
                && Path::new(door.noun).components().count() == 1,
            "`{}`'s park noun `{}` must be one space-free path component: it is both the \
             `.jigc/displaced/` sub-directory and the word the route tells the reader to \
             delete a copy at",
            door.code,
            door.noun,
        );
        assert!(
            !door.undone.trim().is_empty(),
            "`{}` must say what its failure left undone — it is the route's lead clause",
            door.code,
        );
        assert!(
            !cli::invocation_log::ERROR_CODE_REGISTRY.contains(&door.code),
            "`{}` must stay OUT of `ERROR_CODE_REGISTRY`: that registry mirrors door \
             identities derived from `COMMITTING_DOORS`, and this family rides the findings \
             arm (settle-record §10)",
            door.code,
        );

        let tmp = TempDir::new(door.noun);
        let repo = tmp.0.join("repo");
        let jigc = repo.join(".jigc");
        let path = repo.join("docs").join("raced.md");
        fs::create_dir_all(path.parent().expect("parent")).expect("mkdir");
        fs::create_dir_all(&jigc).expect("mkdir .jigc");
        fs::write(&path, b"the pre-image").expect("write pre-image");

        let mut entry = PreImage::capture("docs/raced.md", path.clone()).expect("capture");
        fs::write(&path, b"what jigc wrote").expect("jigc's own write");
        entry.wrote();
        // The race: a third party rewrites the path inside the transaction's window.
        fs::write(&path, b"THIRD PARTY PROSE").expect("the racer's write");
        let mut family = PreImageFamily::empty(*door);
        family.push(entry);
        let findings = family.restore(&repo, &jigc);

        assert_eq!(
            findings.len(),
            1,
            "`{}` must raise exactly one finding for one raced path",
            door.code,
        );
        let finding = &findings[0];
        assert_eq!(finding.code, door.code);
        assert_eq!(
            finding.severity,
            Severity::Blocking,
            "`{}` must block: nothing but a human can reconcile two versions of a file they \
             co-own",
            door.code,
        );
        let route = finding
            .route
            .as_ref()
            .unwrap_or_else(|| panic!("`{}` must carry a route (the route floor)", door.code));
        assert!(matches!(route.kind(), RouteKind::Human));
        assert!(route.as_str().contains(door.undone));
        assert_eq!(
            finding
                .location
                .as_ref()
                .and_then(|location| location.address.clone()),
            Some("docs/raced.md".to_string()),
            "`{}` keys at the raced file path — two raced paths are two findings, never one \
             `(code, null)`",
            door.code,
        );
        // The floor's machine half: the engine asserts the route rules on this seam.
        serde_json::to_string(finding)
            .unwrap_or_else(|err| panic!("`{}` must serialize: {err}", door.code));

        assert_eq!(
            fs::read(&path).expect("read the raced path"),
            b"THIRD PARTY PROSE",
            "`{}` must leave the racer's bytes exactly where they are — restoring over them \
             is the loss this family exists to prevent, in the other direction",
            door.code,
        );
        let parked = jigc.join("displaced").join(door.noun).join("docs");
        let copies: Vec<PathBuf> = fs::read_dir(&parked)
            .unwrap_or_else(|err| {
                panic!(
                    "`{}` must park its pre-image under `.jigc/displaced/{}/`: {err}",
                    door.code, door.noun,
                )
            })
            .map(|entry| entry.expect("dir entry").path())
            .collect();
        assert_eq!(copies.len(), 1, "one refused restore, one parked copy");
        assert_eq!(
            fs::read(&copies[0]).expect("read the parked copy"),
            b"the pre-image",
            "the parked copy must hold the bytes the transaction was going to put back",
        );
        assert!(
            route.as_str().contains(
                copies[0]
                    .strip_prefix(&repo)
                    .expect("the park is inside the repo")
                    .to_string_lossy()
                    .as_ref()
            ),
            "`{}`'s route must name the parked copy — both versions are on disk, and a route \
             that names only one of them is the half a reader cannot derive",
            door.code,
        );
    }
}
