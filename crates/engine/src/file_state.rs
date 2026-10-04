//! The `file-state` hash record — the raw-byte drift hash, the
//! `.jigc/state/file-state.json` `path → hex-hash` map (load/save), the engine-native
//! `file-state` probe surface, and the full OOB reconciliation classifier.
//!
//! The store layer is a raw-byte [`blake3`] hash and byte-stable JSON I/O over the
//! record. Built atop it: [`staged_copy_finding`], the per-staged-instance advisory
//! the task sweep emits at a persisted instance's repo-real destination (M43 A14 —
//! staged copies never key against the record); [`reconcile_committed`], the inc-5 OOB
//! reconciliation state machine over a *committed* managed doc — absorb /
//! conformance-block / conflict-block (`reconciliation.md` → The state machine;
//! `validation.md` → the `file-state` probe); and [`detect_rename`], the separate
//! rename classifier for a tracked path gone *missing* — strong signal (a
//! content-hash-matching untracked path → suspected `git mv`) / weak signal
//! (restore), routed to human-side revert, never auto-rewriting referrer refs
//! (`reconciliation.md` → Rename detection / No silent rename).
//!
//! - **Raw-byte hash** (`parsing.md` → Round-trip guarantees → Drift hash): the
//!   `file ↔ CLI-state` check hashes the file's *raw bytes*, so a conformant
//!   no-op read/write doesn't drift and first-touch canonicalizations re-baseline
//!   it. `blake3` is the chosen algorithm (`DECISIONS.md` 2026-05-31 →
//!   Cross-cutting build crates → Hashing = blake3; inc-4 planning pin).
//! - **The record** (`storage.md` → `.jigc/state/` hash records): one JSON map
//!   `path → hex-hash` at `<jigc_root>/state/file-state.json`, gitignored and
//!   rebuildable. Keyed to "last-known-good committed state"; the sites the recorded
//!   hash updates at are `reconciliation.md` → Hash re-baselining's list, not a count
//!   here. All but one are *callers* of [`FileStateRecord::save`]; the one this module
//!   owns is the **copy-in door** ([`read_for_copy_in`]), which records a doc's first
//!   baseline as a task copies it in.

use crate::finding::{Finding, Location, Severity};
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};
use std::path::{Path, PathBuf};

/// The `file-state.json` filename inside `<jigc_root>/state/`.
const FILE_STATE_FILE: &str = "file-state.json";

/// Hash a file's **raw bytes** to its lowercase-hex `blake3` digest.
///
/// The drift hash is over raw bytes, never the canonicalized form, so a
/// conformant no-op read/write does not register as drift (`parsing.md` → Drift
/// hash). The output is the 64-char lowercase hex of the 32-byte `blake3` digest.
pub fn hash_bytes(bytes: &[u8]) -> String {
    blake3::hash(bytes).to_hex().to_string()
}

/// The `file-state` record: a map from managed-doc path to its recorded raw-byte
/// hex hash (`storage.md` → `.jigc/state/` hash records).
///
/// A [`BTreeMap`] so serialization is **deterministic** (path-sorted) — the JSON
/// bytes are stable across runs regardless of insertion order, which the
/// byte-stable golden depends on. Paths are stored as their string form; the
/// record carries no schema version of its own (it is a rebuildable cache,
/// re-derivable from the committed docs at any time).
///
/// `base` is the **loaded** map, stashed by [`load`](Self::load) so
/// [`save`](Self::save) can tell *our* per-key deltas from the keys we merely
/// carried along — the substrate of the base-relative three-way merge (M46
/// Increment 1). It is `#[serde(skip)]`: [`to_bytes`](Self::to_bytes) serializes
/// `self`, and the on-disk byte form is golden-locked, so the stash must not reach
/// the wire.
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct FileStateRecord {
    /// `path → hex-hash`, path-sorted for deterministic output.
    pub hashes: BTreeMap<String, String>,
    /// The map as loaded from disk — the merge base. Never serialized.
    #[serde(skip)]
    base: BTreeMap<String, String>,
}

/// Equality is over [`hashes`](FileStateRecord::hashes) **alone**: `base` is
/// bookkeeping for the merge, not part of the record's identity. Two records with
/// the same map are the same record whether one was loaded and the other built —
/// which is exactly what the save → load round-trip assertion means.
impl PartialEq for FileStateRecord {
    fn eq(&self, other: &Self) -> bool {
        self.hashes == other.hashes
    }
}

impl Eq for FileStateRecord {}

impl FileStateRecord {
    /// An empty record.
    pub fn new() -> Self {
        Self::default()
    }

    /// The record's on-disk location under a `.jigc/` home.
    pub fn path_in(jigc_root: &Path) -> PathBuf {
        jigc_root.join("state").join(FILE_STATE_FILE)
    }

    /// Record `hash` for `path` (overwriting any prior hash for that path).
    pub fn record(&mut self, path: impl Into<String>, hash: impl Into<String>) {
        self.hashes.insert(path.into(), hash.into());
    }

    /// The recorded hash for `path`, if any.
    pub fn get(&self, path: &str) -> Option<&str> {
        self.hashes.get(path).map(String::as_str)
    }

    /// **Forget the recorded hash for `path`** — the inverse of
    /// [`record`](Self::record), the un-manage primitive (M21 Increment 4;
    /// `project-setup.md` → Flow 2 hardening → Teardown / cleanup (G5), un-manage a
    /// doc). Returns `true` iff a hash was actually removed, so a re-run on an
    /// already-forgotten path is a clean no-op (idempotency). Drops only jigc's own
    /// index entry; the file on disk is never touched (this record is byte-only state).
    pub fn forget(&mut self, path: &str) -> bool {
        self.hashes.remove(path).is_some()
    }

    /// Serialize to the frozen on-disk byte form: pretty JSON, path-sorted, one
    /// trailing newline (golden-locked, matching the `base.json` convention).
    pub fn to_bytes(&self) -> String {
        let mut s = serde_json::to_string_pretty(self).expect("FileStateRecord serializes");
        s.push('\n');
        s
    }

    /// Save the record to `<jigc_root>/state/file-state.json`, **merged** against
    /// whatever is on disk now, creating the `state/` dir if absent.
    ///
    /// Routes through [`crate::state::persist`] (temp + `rename`) rather than a
    /// direct `std::fs::write`: `.jigc/state/*` is **not** task-isolated, so two
    /// writers can hit this file at once, and a direct write lets a concurrent
    /// reader observe a truncated file. The atomic temp+rename — with a
    /// process-unique temp sibling — guarantees every reader sees a complete,
    /// parseable file (M45 Increment 7, Decision 9). `persist` creates the parent
    /// dir on demand, so no separate `create_dir_all` is needed here.
    ///
    /// Atomicity alone still lost data: the whole record was written verbatim, so a
    /// writer that had held its copy across another writer's save silently discarded
    /// that writer's per-key delta and *reported success* — a merge that never
    /// happened, which is what `CLAUDE.md`'s "never silently merged" forbids. So the
    /// save is now **base-relative and three-way** (M46 Increment 1 — `DECISIONS.md`
    /// 2026-08-18 M46 planned, N-3): the disk is re-read as `theirs` and merged with
    /// `ours` against the `base` [`load`](Self::load) stashed. See
    /// [`merge_onto`](Self::merge_onto) for the rule.
    ///
    /// Takes `&self` and keeps its signature, so no call site changes and the base
    /// stash is *not* advanced by a save: a record saved twice merges against the
    /// same base both times, which is stable (our second save's deltas are still our
    /// deltas) — see the `migrate-corpus` per-doc save loop.
    ///
    /// An **unreadable or unparseable** file on disk degrades to `theirs = ours`,
    /// which the rule reduces to writing `ours` — the pre-merge behaviour. A
    /// rebuildable cache must not fail a save because its own bytes went bad.
    ///
    /// The merge closes the *sequential* interleave. The concurrent read-modify-write
    /// window between the re-read of `theirs` and the `rename` — a sibling whose
    /// `rename` lands inside it is merged against a disk state that no longer exists,
    /// and its delta is overwritten by a save that returned `Ok` — is closed by running
    /// the whole re-read + merge + persist under [`crate::state::with_save_lock`] (M46
    /// Increment 1, T2). The critical section spawns no subprocess, so it cannot
    /// deadlock against the `pre-commit` hook's nested `jigc` process. The save **never
    /// runs unlocked**: a lock not taken within [`crate::state::SAVE_LOCK_BUDGET`] fails
    /// it with nothing written (2026-10-01), because an unlocked save on this record is
    /// exactly the lost concurrent delta the lock exists to prevent.
    pub fn save(&self, jigc_root: &Path) -> std::io::Result<()> {
        let path = Self::path_in(jigc_root);
        crate::state::with_save_lock(&path, || self.save_locked(jigc_root))
    }

    /// [`save`](Self::save)'s critical section — the re-read of *theirs*, the merge and
    /// the persist — for a caller that **already holds** the save lock.
    ///
    /// The lock is per open file description and is not re-entrant, so a critical section
    /// that has to do more than save (the copy-in door reads the doc it is about to stage
    /// and decides whether to record it, all under one lock — [`read_for_copy_in`]) cannot
    /// call `save` from inside it. It calls this instead, so there is still one merge and
    /// one persist, in one place.
    fn save_locked(&self, jigc_root: &Path) -> std::io::Result<()> {
        let theirs = Self::load(jigc_root).unwrap_or_else(|_| {
            crate::state::tally_save_degrade(crate::state::SaveDegrade::UnreadableTheirs);
            self.clone()
        });
        let merged = self.merge_onto(&theirs);
        crate::state::persist(&Self::path_in(jigc_root), merged.to_bytes().as_bytes())
    }

    /// The base-relative three-way merge: over `base ∪ ours ∪ theirs`, per key —
    ///
    /// - **`ours == base`** (including *both absent*) ⇒ take **theirs**, value *or*
    ///   absence. We never touched this key, so the other writer's decision stands —
    ///   this is the half that stops a save from resurrecting a concurrent
    ///   [`forget`](Self::forget) or dropping a concurrent [`record`](Self::record).
    /// - **otherwise** ⇒ take **ours**, value *or* absence. We recorded or forgot it
    ///   deliberately, so our decision stands — including our own deletion, against a
    ///   concurrent re-record.
    ///
    /// Both sides touching one key is the conflict cell, and it resolves to the
    /// **later saver's** value: deterministic, and the only choice available without
    /// a semantics for "combine two hashes of the same path", which does not exist
    /// (a path has exactly one last-known-good hash).
    fn merge_onto(&self, theirs: &Self) -> Self {
        let keys: BTreeSet<&String> = self
            .base
            .keys()
            .chain(self.hashes.keys())
            .chain(theirs.hashes.keys())
            .collect();
        let mut merged = Self::new();
        for key in keys {
            let ours = self.hashes.get(key);
            let winner = if ours == self.base.get(key) {
                theirs.hashes.get(key)
            } else {
                ours
            };
            if let Some(hash) = winner {
                merged.hashes.insert(key.clone(), hash.clone());
            }
        }
        merged
    }

    /// Load the record from `<jigc_root>/state/file-state.json`. A missing file
    /// is the *first-encounter* case (`reconciliation.md` → Absent-hash is not
    /// drift) and yields an empty record, never an error.
    ///
    /// **The loaded map is stashed as the merge base** — this is the *only* place a
    /// base is minted, and it is what makes [`save`](Self::save)'s three-way merge
    /// possible. A record built by [`new`](Self::new) has an empty base, so every key
    /// it carries reads as its own delta: a from-scratch record still wins its own
    /// keys and adopts everything else on disk.
    ///
    /// **The base travels with the moved value, never a fresh load at save time.**
    /// The wave's most important path is the finalize hand-off: the record loaded in
    /// `cli/task.rs` → `Task::validate` is carried **by value** through staging, the
    /// `git commit`, and the pre-commit hook's whole separate `jigc` process, and is
    /// only saved in phase 7 (`post_commit` → `advance_file_state`). Re-loading it
    /// there would compute our delta against a base that already contains the hook's
    /// write — so our copy would read as "unchanged" for the hook's keys and, worse,
    /// as a *deliberate* delta for keys the hook retired, silently voiding the merge
    /// exactly where it matters most. A refactor that replaces a carried record with
    /// a fresh `load` breaks the merge without breaking a type.
    pub fn load(jigc_root: &Path) -> std::io::Result<Self> {
        let path = Self::path_in(jigc_root);
        let mut record = match std::fs::read(&path) {
            Ok(bytes) => serde_json::from_slice::<Self>(&bytes)?,
            Err(err) if err.kind() == std::io::ErrorKind::NotFound => Self::new(),
            Err(err) => return Err(err),
        };
        record.base = record.hashes.clone();
        Ok(record)
    }
}

/// The per-staged-instance `file-state` advisory (M43 A14 — `DECISIONS.md`
/// 2026-07-16 Settle item 9; `surface-contract.md` → law 1): a task's staged
/// **persisted** instance, reported at `dest` — the repo-relative repo-real
/// committed destination its finalize will promote it to — never the
/// `docs/<type>:<slug>.md` working-area fiction the pre-M43 sweep printed.
///
/// Purely informational, emitted directly by the sweep **without consulting or
/// mutating the record**: a staged copy has no committed baseline of its own to
/// drift against (an in-flight edit of a copied-in committed doc legitimately
/// differs from the committed baseline, so keying `dest` against the record would
/// mint false blocking drift), and the baseline is adopted only when the finalize
/// lands. A transient-sink or unknown-type instance has no committed destination,
/// so the sweep emits **no** `file-state.*` finding for it at all — the A14 root
/// cause. Distinct from `file-state.baseline-adopt` so a task touching a committed
/// doc the store sweep baseline-adopts in the same run cannot collide two findings
/// on one `(code, target)` key.
pub fn staged_copy_finding(dest: &str) -> Finding {
    Finding::graded(
        Severity::Advisory,
        "file-state.staged-copy",
        format!("staged copy of `{dest}` — this task's in-flight version of the doc"),
        Some(Location::addressed(dest, 1, 1)),
        Some(crate::finding::Route::informational(
            "no action needed — the staged copy is validated in-task and baselined when \
             its finalize lands",
        )),
    )
}

/// How a caller presents a `DRIFTED + TOUCHED` **conflict-block** — the clause naming the
/// *CLI-side* mover plus the route out of it (M47 inc-2 / T4).
///
/// The conflict route **belongs to the caller, not the classifier**. The classifier sees a
/// path, a hash and a `task_touched` flag; it has no task id, and at one of its two callers
/// there is no task at all — so it cannot name the mover or the way out without lying. It
/// used to hard-code both (`this task's staged writes` + `jigc task discard <task-id>`),
/// which put an **inapplicable verb** carrying an **unsubstituted placeholder** on a
/// *blocking* finding at the milestone-record door — exactly what the M43 route floor exists
/// to prevent (`design/surface-contract.md` → The route fence; `DECISIONS.md` 2026-07-26 the
/// Settle, item 8/P6: a placeholder derivable by the caller must be substituted, and
/// `<task-id>` is not derivable here — it needs a different source).
///
/// Each caller supplies its own, from what it actually holds: the task-scope sweep the real
/// task id ([`ConflictBlock::task`]), the milestone-record preflight a record-shaped block
/// whose route is a human revert. Fields are private, so the value exists only through its
/// constructors and the classifier can add nothing of its own.
///
/// A caller may also supply a **second, path-keyed** presentation (M46 inc-5 / T2), used in
/// place of the general one when the conflict is on exactly that path — a migration task's
/// own recorded source is the one path whose general exits both fail it. The classifier
/// still adds nothing: it picks between two caller-composed pairs on the one fact it holds,
/// the conflicting path ([`ConflictBlock::presentation`]).
///
/// Because the keyed pair replaces the general one **wholesale**, a keyed route owes the
/// operator every exit the general route gave them and it is still true of: the migration
/// arm re-names the whole-task discard as the way to keep the on-disk bytes, since its own
/// exit replaces them (M46 inc-5, validate→fix).
#[derive(Clone, Debug)]
pub struct ConflictBlock {
    /// The clause after ``conflict on `<path>`: `` — names what moved on the CLI side.
    detail: String,
    /// The way out, already substituted by the caller.
    route: crate::finding::Route,
    /// The caller's **path-keyed** second presentation: `(path, detail, route)`, used in
    /// place of the pair above when the conflict is on exactly that path
    /// ([`ConflictBlock::presentation`]).
    keyed: Option<(String, String, crate::finding::Route)>,
}

impl ConflictBlock {
    /// A caller-composed conflict presentation: `detail` is the message clause after
    /// ``conflict on `<path>`: ``, `route` the (already-substituted) way out.
    pub fn new(detail: impl Into<String>, route: crate::finding::Route) -> Self {
        Self {
            detail: detail.into(),
            route,
            keyed: None,
        }
    }

    /// The presentation for a conflict at `path` — the caller's path-keyed pair when it
    /// names this path, its general pair otherwise.
    ///
    /// Still nothing of the classifier's own: both pairs were composed by the caller, and
    /// this only picks between them on the one fact the classifier does hold — which path
    /// conflicted. The keying is what keeps a *narrower* exit from being offered on the
    /// wider domain it is not true of ([surface-contract.md](../../../design/surface-contract.md)
    /// → A route offered on a wider domain must be gated on that domain).
    fn presentation(&self, path: &str) -> (&str, &crate::finding::Route) {
        match &self.keyed {
            Some((keyed_path, detail, route)) if keyed_path == path => (detail, route),
            _ => (&self.detail, &self.route),
        }
    }

    /// Whether `path` is the caller's **path-keyed** subject — a migration task's own
    /// recorded source ([`ConflictBlock::task`]).
    ///
    /// The base-pin backstop asks it ([`reconcile_committed`], the `UNKNOWN` arm): the keyed
    /// route's exit is `jigc unmanage <source>`, which drops that path's baseline *so that*
    /// the next finalize takes the `UNKNOWN` arm and lands the migration's rewrite over the
    /// source. A backstop that blocked that path too would turn the one sanctioned exit into
    /// a loop — unmanage, finalize, blocked on the same path, routed at unmanage again.
    fn keys(&self, path: &str) -> bool {
        matches!(&self.keyed, Some((keyed_path, _, _)) if keyed_path == path)
    }

    /// The **task-scope** preset — the sweep runs inside a named task, so the route names
    /// that task's id outright. The honest resolution pair is unchanged from M43's
    /// ghost-verb repair: discard the **whole task** (no per-doc discard exists) or revert
    /// the external edit on disk; only the `<task-id>` placeholder is gone. The jigc span
    /// rides the checked [`crate::finding::Route::mechanical`] constructor, so a verb that
    /// does not parse cannot be taught here again.
    pub fn task(task_id: &str, migration_source: Option<&str>) -> Self {
        let mut block = Self::new(
            "an external edit and this task's staged writes both changed it",
            crate::finding::Route::mechanical(
                ["jigc", "task", "discard", task_id, "--force"],
                format!(
                    " to drop this task's staged writes (discard retires the whole task — no \
                     per-doc discard exists), or revert the external edit on disk to keep \
                     them — {OUT_OF_BAND_SANCTION}"
                ),
            ),
        );
        if let Some(source) = migration_source {
            block.keyed = Some((
                source.to_string(),
                "an external edit and this migration's staged rewrite both changed it — this \
                 path is the source the task is migrating, so replacing it is the point"
                    .to_string(),
                crate::finding::Route::mechanical(
                    ["jigc", "unmanage", &crate::finding::shell_token(source)],
                    format!(
                        " to drop the stale baseline on that path, then run this finalize \
                         again — the guard is dropped for that path only and the bytes stay \
                         on disk; they are not merged, this task's staged rewrite replaces \
                         them, and that rewrite was authored against the source as this task \
                         recorded it at mint, so an edit made to the file since is replaced \
                         without appearing in the `--approve` fidelity diff (which renders \
                         the recorded source, not what is on disk now). To keep the file as \
                         it stands, `jigc task discard {task_id} --force` retires the migration \
                         instead and leaves it untouched"
                    ),
                ),
            ));
        }
        block
    }
}

/// The **live work-unit record** the validated subject belongs to — the one recorded
/// managed-doc path whose absence from this checkout is not a stale baseline (M52 Inc 10 /
/// T6).
///
/// The rename detector's dangling-baseline arm reads *absent on disk × no HEAD history* and
/// concludes the recorded baseline outlived its checkout — left behind by a branch switch.
/// That conclusion is wrong for exactly one path: the committed record of the milestone the
/// task under validation belongs to. A sub-task is pinned to its milestone's **base**, which
/// by construction predates the record commit, so wherever the checkout stands at that pin —
/// the provisioned worktree does by construction, the shared checkout whenever it is put
/// there — the record reads history-less while it is live, current, and the state the
/// milestone is run from. The carve-out was cut when that arm still routed prune-first at
/// `jigc unmanage <path>`, which would have unmanaged exactly that; the arm routes at the
/// branch switch since M55 (Increment 5 / T1), and — where no branch carries the path — at
/// `jigc unmanage` again since the M55 completion triage (CR2), which the carve-out still
/// precedes: it says the one true thing about this path — it is the live record, neither a
/// doc left on another branch nor one gone from every branch.
///
/// Like [`ConflictBlock`], the fact belongs to the **caller, not the classifier**: the engine
/// sees a path and a hash, never which work unit owns the task whose area it is sweeping. So
/// the caller that knows hands the record's path and its work-unit id in, and the classifier
/// picks on the one fact it holds — which path is missing. A caller with no such record
/// ([`LiveRecord::none`]) leaves every dangling baseline graded by history alone, byte-
/// identically to the shipped classification.
///
/// This is **not** a new observable of the git oracle (see [`detect_rename`]'s op-axis
/// argument): no git operation produces or removes it, so the op table that argument rests on
/// is unchanged. It is a membership fact about `.jigc/` state, supplied from outside.
///
/// No `Default`, deliberately: the absent case is a caller's **statement** that it swept no
/// owned work unit ([`LiveRecord::none`], which says so and carries the reason), not a value
/// that falls out of a struct literal.
#[derive(Clone, Debug)]
pub struct LiveRecord {
    /// `(repo-relative record path, the work-unit id the message and route name)`.
    keyed: Option<(String, String)>,
}

impl LiveRecord {
    /// No live record — every dangling baseline grades by history alone (the shipped
    /// classification). What a caller sweeping no task, or a task belonging to no milestone,
    /// supplies.
    pub fn none() -> Self {
        Self { keyed: None }
    }

    /// The committed record of the **milestone the validated task belongs to**: `path` is its
    /// repo-relative file-state key, `milestone_id` the id the message and route name.
    pub fn milestone(path: impl Into<String>, milestone_id: impl Into<String>) -> Self {
        Self {
            keyed: Some((path.into(), milestone_id.into())),
        }
    }

    /// The work-unit id when `path` **is** the live record, `None` otherwise — the one fact
    /// the classifier asks of this value.
    fn unit_at(&self, path: &str) -> Option<&str> {
        match &self.keyed {
            Some((keyed_path, unit)) if keyed_path == path => Some(unit.as_str()),
            _ => None,
        }
    }
}

/// The full **OOB reconciliation classifier** for a single *committed* managed doc —
/// the state machine [`file_state`] only baseline-adopted in inc-4
/// (`reconciliation.md` → The state machine; `DECISIONS.md` 2026-05-31 → inc-5
/// Reconciliation OOB classifier; `storage.md` → Edge index lifecycle, site 3).
///
/// Reads the `(committed-state, task-state)` pair for the doc at `path` (its
/// file-state key and `<type>:<slug>` identity `from`) and routes deterministically:
///
/// - **`UNKNOWN`** (no recorded hash) → **baseline-adopt**: record the current hash,
///   emit the advisory `file-state.baseline-adopt` finding (absent-hash is not drift).
///   A **non-conformant** file here is not adopted; which advisory it draws is the
///   **managed-vs-foreign discriminator's** call (M48 Inc 4 / T1,
///   [`crate::validate::AdoptionInputs::unadopted`]): a **never-adopted foreign** squatter
///   converges on the store family's `schema-conformance.unadopted-instance` with the
///   adoption route naming its own path, a **managed** doc keeps
///   `reconciliation.conformance-block` — routed on **its schema-version stamp** (T2): the
///   hand-repair sanction when it is at the current version, the corpus-migration route when
///   it is stale, unstamped or ahead. Neither is recorded, so either re-fires until the human
///   resolves it. **Except a touched doc off its base pin** (the rc.24 fix pass, `(R3, F7)`):
///   `UNKNOWN` + `task_touched` + `pinned` carries a blob for the path + the on-disk bytes
///   differ from it → the caller's **conflict-block**, nothing recorded. That is the
///   **base-pin backstop**: with no recorded hash, the pin is the only remaining witness of
///   what the task can have started from, and bytes that differ from it are an edit the
///   staged copy may not carry — adopting them is how a hand edit made after a task's first
///   write was overwritten at exit 0. A `None` lookup (an untracked doc, no pin, a git
///   failure, the record door) and the caller's path-keyed migration source
///   ([`ConflictBlock::keys`]) keep the adoption above.
/// - **`IN_SYNC`** (recorded hash matches) → no finding (clean / task-only change —
///   the working-area writes are reconciled elsewhere, not here).
/// - **`DRIFTED + TOUCHED`** (`task_touched`) → **conflict-block**: both sides moved.
///   A blocking `reconciliation.conflict-block` finding carrying the **caller-supplied**
///   [`ConflictBlock`] presentation (the classifier has no task and no verb of its own);
///   no silent merge, the hash and edge index are left untouched. **Except a pulled edit**
///   (M55 Increment 4, L1): when the on-disk bytes equal the doc's blob at the caller's
///   base pin (`pinned`, [`crate::validate::PinnedBlob`]), the drift predates the task, so
///   the `DRIFTED + UNTOUCHED` absorb below runs whole; a pinned edit that fails the
///   conformance gate keeps the caller's conflict-block unchanged, never a
///   conformance-block. A `None` lookup — no pin, absent blob, git failure, the record
///   door — keeps the conflict-block.
/// - **`DRIFTED + UNTOUCHED`** → the **parse classifier**: re-parse + schema-validate
///   the on-disk bytes against `schema`.
///   - clean → **absorb**: re-hash the recorded baseline forward, incrementally
///     update the committed `index` for the doc's edges (lifecycle site 3), and emit
///     the advisory `reconciliation.absorb` "external edit absorbed" finding.
///   - parse/schema fail → **conformance-block**: a blocking
///     `reconciliation.conformance-block` finding naming the file + the first precise
///     conformance error; the hash is **not** advanced and the index is **not**
///     touched (no auto-repair).
///
/// Mutating: re-hash advances `record` on baseline-adopt and absorb; absorb mutates
/// `index` in place. No I/O of its own — the caller supplies the on-disk `bytes` and
/// persists the advanced record / index after the run.
#[allow(clippy::too_many_arguments)]
pub fn reconcile_committed(
    record: &mut FileStateRecord,
    index: &mut crate::index::EdgeIndex,
    schema: &crate::schema::Schema,
    path: &str,
    from: &str,
    bytes: &[u8],
    task_touched: bool,
    pinned: &crate::validate::PinnedBlob<'_>,
    conflict: &ConflictBlock,
    adoption: &crate::validate::AdoptionInputs<'_>,
) -> Vec<Finding> {
    let current = hash_bytes(bytes);
    match record.get(path) {
        // UNKNOWN → the G4 conformance gate (M21; `project-setup.md` → Flow 2 hardening):
        // a fresh-checkout doc is baseline-adopted **only if it classifies conformant** —
        // a non-conformant `.md` sitting in a `location:` dir is routed as an advisory and
        // **not** recorded (so it re-fires every sweep until the human resolves it), never
        // silently absorbed.
        //
        // *Which* advisory is the managed-vs-foreign discriminator's call (M48 Inc 4 / T1).
        // The M42 sweep reached the store family and stopped there, so this door graded a
        // never-adopted **foreign** file — a file the user never handed to jigc — as an
        // unvetted *managed* one: a different code from the store door's over the same
        // bytes, carrying a route that names no verb and no path. A foreign file converges
        // here on the shipped `schema-conformance.unadopted-instance` and its adoption
        // route; everything the discriminator adjudicates **managed** (the commonest case,
        // and every doc of an unversioned doctype) keeps `conformance_advisory_finding` —
        // whose route, since T2, is decided by the doc's own schema-version stamp rather
        // than by adoption prose that is now wrong for its whole population. Neither
        // branch records.
        //
        // **The base-pin backstop comes first** (the rc.24 fix pass, `(R3, F7)`). A doc the
        // task has staged normally has a recorded hash by now — the copy-in door records
        // one ([`read_for_copy_in`]) — so reaching this arm *touched* means the key was lost
        // after the copy-in, was never written (a binary older than that door, a doc that
        // did not conform when it was copied in, a second task's copy-in over a first one's
        // unrecorded staging), or was dropped on purpose. Adopting the on-disk bytes here
        // is the defect: the promote that follows replaces them with a staged copy that was
        // never compared against them. The pin is the one witness left — bytes equal to its
        // blob are what the task started from; bytes that differ are an edit, and the
        // caller's conflict-block is the answer both orders of that edit get, because the
        // staged copy cannot say which side of the copy-in it fell on.
        None if task_touched
            && !conflict.keys(path)
            && pinned(path).is_some_and(|blob| hash_bytes(&blob) != current) =>
        {
            vec![conflict_block_finding(path, conflict)]
        }
        None => match conformance_gate(schema, bytes) {
            Ok(_) => {
                record.record(path, current);
                vec![baseline_adopt_finding(path)]
            }
            Err(cause) => {
                let source = String::from_utf8_lossy(bytes);
                // The `<slug>` half of the identity this doc is reconciled under — the
                // caller hands the identity in `from`, so it is read off that rather than
                // re-derived from the path (M50 Inc 2 / T1).
                let slug = from.split_once(':').map_or("", |(_, slug)| slug);
                match adoption.unadopted(&schema.ty, slug, schema, &source, path) {
                    Some(finding) => vec![finding],
                    None => vec![conformance_advisory_finding(
                        path,
                        cause,
                        &source,
                        adoption.current(&schema.ty),
                    )],
                }
            }
        },
        // IN_SYNC → clean / task-only change: nothing to reconcile here.
        Some(recorded) if recorded == current => Vec::new(),
        // DRIFTED + TOUCHED → conflict-block (both sides moved; no silent merge) — unless
        // the on-disk bytes equal the doc's blob at the caller's base pin (M55 Increment 4,
        // L1): then the drift predates the task (a pull), only the task moved since, and the
        // UNTOUCHED arm's whole absorb body runs. A pinned edit that does not conform is
        // never baselined: it keeps the caller's conflict-block unchanged (P1), so a
        // migration's path-keyed exit survives. The seam is asked here and in the `UNKNOWN`
        // arm's backstop above — of a touched path, both times.
        Some(_) if task_touched => {
            let at_pin = pinned(path).is_some_and(|blob| hash_bytes(&blob) == current);
            match at_pin
                .then(|| conformance_gate(schema, bytes).ok())
                .flatten()
            {
                Some(doc) => absorb(record, index, schema, path, from, current, &doc),
                None => vec![conflict_block_finding(path, conflict)],
            }
        }
        // DRIFTED + UNTOUCHED → the parse classifier (the same conformance gate the
        // UNKNOWN arm above runs; here a fail is **blocking**, not advisory).
        Some(_) => match conformance_gate(schema, bytes) {
            // Clean → absorb: re-hash + incrementally update the index.
            Ok(doc) => absorb(record, index, schema, path, from, current, &doc),
            // Schema-invalid / parse fail → conformance-block, naming the first error.
            Err(cause) => vec![conformance_block_finding(path, cause)],
        },
    }
}

/// The **absorb** body both drifted arms of [`reconcile_committed`] share: re-hash the
/// recorded baseline forward to `current`, incrementally update the committed `index` with the
/// conformant `doc`'s edges (lifecycle site 3), and emit the advisory `reconciliation.absorb`.
fn absorb(
    record: &mut FileStateRecord,
    index: &mut crate::index::EdgeIndex,
    schema: &crate::schema::Schema,
    path: &str,
    from: &str,
    current: String,
    doc: &crate::parse::Document,
) -> Vec<Finding> {
    record.record(path, current);
    index.absorb_doc(schema, from, doc);
    vec![absorb_finding(path)]
}

/// The shared **conformance gate** — re-parse the on-disk `bytes` against `schema` and
/// schema-validate the result. Returns the parsed [`Document`](crate::parse::Document) on a
/// clean classification, or the **first** precise conformance error ([`Finding`]) on a
/// parse / schema failure. Both the G4 `UNKNOWN` baseline-adopt gate (M21) and the
/// `DRIFTED + UNTOUCHED` absorb classifier route through it; they differ only in how a
/// failure is graded (the `UNKNOWN` arm advisory, the `DRIFTED` arm blocking).
fn conformance_gate(
    schema: &crate::schema::Schema,
    bytes: &[u8],
) -> Result<crate::parse::Document, Option<Finding>> {
    let source = String::from_utf8_lossy(bytes);
    match crate::parse::parse_sections(schema, &source) {
        Ok(doc) => {
            let conformance = crate::validate::schema_conformance(schema, &source, &doc);
            match conformance
                .into_iter()
                .find(|f| f.severity == Severity::Blocking)
            {
                Some(first) => Err(Some(first)),
                None => Ok(doc),
            }
        }
        Err(parse_findings) => Err(parse_findings.into_iter().next()),
    }
}

/// What the **copy-in door** did about a doc's baseline ([`read_for_copy_in`]).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CopyInBaseline {
    /// The record already carried the doc's key. Nothing was written; the doc reads
    /// `IN_SYNC` or `DRIFTED` against the hash it had.
    Held,
    /// The key was absent and the bytes just read were recorded as the doc's baseline —
    /// its first encounter, at the door that stages it.
    Adopted,
    /// The key was absent and **stays** absent: the doc remains `UNKNOWN`, and the
    /// committing door's base-pin backstop decides ([`reconcile_committed`]).
    Unrecorded(Unrecorded),
}

/// Why a copy-in recorded no baseline for a doc that had none.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Unrecorded {
    /// The bytes do not pass the conformance gate. A non-conformant doc is never baselined
    /// — the rule the `UNKNOWN` arm and the post-commit re-hash already keep
    /// ([`committed_path_recordable`]).
    NonConformant,
    /// **Another open task already has this doc staged, and there is no record of what it
    /// started from.** The record is per path, not per task: recording *these* bytes would
    /// make that other task read `IN_SYNC` against a file it never saw — and if a hand edit
    /// fell between the two copy-ins, promote over it at exit 0. Left `UNKNOWN`, both tasks
    /// are decided by the pin.
    StagedElsewhere,
}

impl CopyInBaseline {
    /// The advisory the write door owes when it **adopted** the baseline — the same
    /// `file-state.baseline-adopt` the sweep emits on a first encounter, keyed at the same
    /// path, because it is the same event: *"every absorb surfaces"* has no door exemption
    /// (`reconciliation.md` → What reconciliation does NOT do), and a doc baselined at its
    /// copy-in is `IN_SYNC` by the time any sweep sees it, so the write is the only surface
    /// left to say so. `None` for every other outcome: nothing was adopted.
    pub fn finding(self, key: &str) -> Option<Finding> {
        (self == CopyInBaseline::Adopted).then(|| baseline_adopt_finding(key))
    }
}

/// A doc read for a copy-in: the bytes to stage, and what was done about its baseline.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CopyInSource {
    /// The doc's bytes at its home, exactly as read — the caller stages these.
    pub body: String,
    /// What the read did about the doc's `file-state` baseline.
    pub baseline: CopyInBaseline,
}

/// **The copy-in door** — read the doc at `home` for staging into a task, and record its
/// baseline if it has none (`reconciliation.md` → Detection timing, the *write through the
/// CLI* row; → Baseline adoption; the rc.24 fix pass, `(R3, F7)`).
///
/// A task that copies a committed doc in will, at its finalize, **replace** the file at the
/// doc's home with its staged copy. Whether that is safe is a comparison between what is on
/// disk then and what was on disk *now* — and until this door existed nothing remembered
/// *now*: the baseline was written only by a landed finalize, so through the whole of a
/// clone's first task the doc was `UNKNOWN`, the sweep adopted whatever it found, and a hand
/// edit made after the copy-in was overwritten at exit 0. So the read that feeds the staging
/// is also the doc's first encounter, and it records what it read.
///
/// - **`key`** is the doc's file-state key — its repo-relative home, the string the store
///   sweep reads it under ([`crate::finalize::promote_destination`], the same key a landed
///   finalize records). A different spelling would be a silent no-op.
/// - **The hash is of the raw bytes read**, never of the staged copy: the copy-in applies
///   the first-touch canonicalization (BOM strip, single final newline), and a hash of
///   *that* would make every doc it changed read `DRIFTED` from the moment it was staged.
/// - **Record-if-absent, never overwrite.** A key the record already holds is left alone
///   ([`CopyInBaseline::Held`]) — the bytes read are then either what it names or a drift
///   the sweep will classify. Task writes still never *update* a committed-state hash.
/// - **A non-conformant doc is not baselined**, and **neither is a doc another open task
///   holds staged with no record** (`staged_elsewhere`; [`Unrecorded`]).
///
/// **One lock, the read inside it.** The read, the re-read of the record, the decision and
/// the save run under the record's save lock ([`crate::state::with_save_lock`]), so two
/// tasks' first writes cannot both find the key absent and both record: the second one in
/// reads the first one's key and records nothing. It spawns no subprocess, which is the
/// lock's standing rule. A record that already holds the key takes no lock at all — there is
/// nothing to write, and whatever the read returns is classified against the held hash.
///
/// **It fails closed.** A lock not taken within [`crate::state::SAVE_LOCK_BUDGET`], an
/// unreadable record, an unreadable doc: each is the caller's error, returned **before**
/// anything is staged. A copy-in that staged the doc and skipped the record would be exactly
/// the unrecorded staging this door exists to end.
pub fn read_for_copy_in(
    jigc_root: &Path,
    home: &Path,
    key: &str,
    schema: &crate::schema::Schema,
    staged_elsewhere: impl FnOnce() -> bool,
) -> std::io::Result<CopyInSource> {
    if FileStateRecord::load(jigc_root)?.get(key).is_some() {
        return Ok(CopyInSource {
            body: std::fs::read_to_string(home)?,
            baseline: CopyInBaseline::Held,
        });
    }
    crate::state::with_save_lock(&FileStateRecord::path_in(jigc_root), || {
        let body = std::fs::read_to_string(home)?;
        let mut record = FileStateRecord::load(jigc_root)?;
        let baseline = if record.get(key).is_some() {
            CopyInBaseline::Held
        } else if conformance_gate(schema, body.as_bytes()).is_err() {
            CopyInBaseline::Unrecorded(Unrecorded::NonConformant)
        } else if staged_elsewhere() {
            CopyInBaseline::Unrecorded(Unrecorded::StagedElsewhere)
        } else {
            record.record(key, hash_bytes(body.as_bytes()));
            record.save_locked(jigc_root)?;
            CopyInBaseline::Adopted
        };
        Ok(CopyInSource { body, baseline })
    })
}

/// Whether a **just-committed** `path` (its committed `bytes`) may be recorded into the
/// `file-state` baseline — the finalize post-commit twin of the [`reconcile_committed`]
/// `UNKNOWN` arm's G4 gate (M21; `project-setup.md` → Flow 2 hardening).
///
/// A committed path that is a **managed-doc slot** — a *direct child* `<location>/<slug>.md`
/// of some persisted schema's `location:`, exactly the non-recursive `<location>/*.md`
/// namespace [`reconcile_committed_store`] walks — is recordable **only if it classifies
/// conformant**: a foreign non-conformant `.md` that `git add --all` swept into the
/// aggregate commit must **not** be baseline-adopted, so it stays `UNKNOWN` and the
/// advisory re-fires every finalize until the human resolves it (the routed-but-not-
/// recorded recurrence).
///
/// **Direct-child only.** A path *under* a `location:` prefix but *nested* (a `/` in the
/// slug remainder — e.g. a promoted owner-artifact `completions/artifacts/<run>/audit.md`)
/// is **not** a managed doc the store sweep ever reaches, so it is always recordable — the
/// gate guards exactly the managed-doc namespace, never the artifacts the sweep leaves
/// baselined-as-is. Any path no persisted schema owns (code, configs) is likewise always
/// recordable.
///
/// **Placement doctypes** (`location: None`, one instance at an exact literal
/// `placement.file`) are owned by **exact-path equality** against that literal — matching the
/// [`reconcile_committed_store`] placement arm — so a foreign non-conformant file squatting the
/// managed literal path is conformance-gated (stays `UNKNOWN`, not baseline-adopted), while a
/// sibling root `.md` the schema does not name is always recordable (a literal file is not a
/// root glob; `storage.md` → Placement census: `committed_path_recordable`).
pub fn committed_path_recordable(
    schemas: &std::collections::BTreeMap<String, crate::schema::Schema>,
    path: &str,
    bytes: &[u8],
) -> bool {
    let managed_doc = schemas.values().find_map(|s| {
        // A placement doctype (`location: None`) owns its one instance at the exact literal
        // `placement.file` — exact-path equality, matching the store sweep's placement arm,
        // so a foreign non-conformant file squatting that literal path is conformance-gated
        // (not always-recordable), while a sibling root `.md` is not this doctype's instance
        // (a literal file is not a root glob; `storage.md` → Placement census:
        // `committed_path_recordable`).
        if let Some(placement) = &s.placement {
            return (placement.file == path).then_some(s);
        }
        let loc = s.location.as_deref()?;
        let slug = path.strip_prefix(loc)?.strip_suffix(".md")?;
        // The store walk is a non-recursive `<location>/*.md` glob: only a single-segment
        // slug (no `/`) is a managed doc. A nested path is an unwalked artifact.
        (!slug.contains('/')).then_some(s)
    });
    match managed_doc {
        Some(schema) => conformance_gate(schema, bytes).is_ok(),
        None => true,
    }
}

/// Sweep the **committed store** for out-of-band drift and route it — the command-
/// surface wiring of [`reconcile_committed`] + [`detect_rename`] over every tracked
/// committed managed doc (`reconciliation.md` → Detection timing: the `task validate`
/// full sweep, shared by `finalize`'s preflight).
///
/// For each persisted (`location:`-bearing) schema in `schemas`, walks
/// `<repo_root>/<location>/*.md`, derives each doc's `<type>:<slug>` identity and its
/// `<location>/<slug>.md` record key, and:
///
/// - **present on disk** → [`reconcile_committed`] against the recorded baseline.
///   `task_touched` is true iff this task's working area stages the same identity at
///   `<task_dir>/docs/<type>:<slug>.md` (a `DRIFTED + TOUCHED` conflict). Absorb
///   mutates `record` + `index` in place; a block leaves both pinned.
/// - **recorded but now absent on disk** → [`detect_rename`] over the untracked
///   candidates (the on-disk `.md` files of that type with no recorded hash), routing
///   a suspected `git mv` (strong signal) or a restore (weak signal). `history` grades
///   the weak signal (M45, Decision 7): a path with no HEAD history is a dangling
///   baseline (advisory — routed at the branch switch when `other_refs` finds a branch
///   carrying it, at `jigc unmanage` when none does), a path with history is a genuine
///   deletion (block).
///
/// `conflict` is the caller's [`ConflictBlock`] — the sweep knows the working area's
/// *path*, never which task (or join) owns it, so the naming and the way out come from the
/// caller that does (M47 inc-2 / T4). `pinned` is the caller's
/// [`PinnedBlob`](crate::validate::PinnedBlob) — a doc's committed bytes at the caller's base
/// pin, which turns a touched doc's pulled drift into an absorb (M55 Increment 4). `adoption`
/// is the caller's
/// [`AdoptionInputs`](crate::validate::AdoptionInputs) — three pack facts the engine cannot
/// produce, feeding [`reconcile_committed`]'s `UNKNOWN` + non-conformant arm so a foreign
/// squatter draws the same code and route here it draws at store scope (M48 Inc 4 / T1).
/// `live` is the caller's [`LiveRecord`] — the committed record of the work unit the swept
/// task belongs to, if any, so the one history-less path that is the milestone's own state,
/// and no doc left on another branch, is graded as the live record it is (M52 Inc 10 / T6).
///
/// Mutating: `record` (baseline-adopt / absorb) and `index` (absorb) advance in place;
/// the caller persists them. Findings aggregate in a stable order: persisted schemas
/// by type, then committed docs by path-sorted slug, then rename findings for each
/// recorded-but-missing path (also type-then-path sorted). No I/O beyond reading the
/// committed `.md` bytes — the engine stays shell-free.
#[allow(clippy::too_many_arguments)]
pub fn reconcile_committed_store(
    record: &mut FileStateRecord,
    index: &mut crate::index::EdgeIndex,
    schemas: &std::collections::BTreeMap<String, crate::schema::Schema>,
    repo_root: &Path,
    task_dir: &Path,
    history: &crate::validate::HistoryPredicate<'_>,
    other_refs: &crate::validate::OtherRefsPredicate<'_>,
    pinned: &crate::validate::PinnedBlob<'_>,
    conflict: &ConflictBlock,
    adoption: &crate::validate::AdoptionInputs<'_>,
    live: &LiveRecord,
) -> Vec<Finding> {
    let mut findings = Vec::new();
    // Snapshot the recorded committed paths *before* the reconcile loop mutates the
    // record (baseline-adopt records fresh paths), so rename detection below can tell a
    // genuinely-tracked path from one this same sweep just adopted.
    let recorded_at_entry: std::collections::BTreeSet<String> = record
        .hashes
        .keys()
        .filter(|p| persisted_committed_path(p, schemas))
        .cloned()
        .collect();
    // Track which recorded paths we saw on disk, so a recorded-but-missing path can be
    // routed to rename detection afterward.
    let mut seen_on_disk: std::collections::BTreeSet<String> = std::collections::BTreeSet::new();

    for (ty, schema) in schemas {
        // A placement doctype (`location: None`) owns its one instance at the exact literal
        // `placement.file` — visit it by that path, never a dir-glob, so an OOB edit to a
        // root `FOO.md`/`CHANGELOG.md` is detected + routed instead of silently skipped by
        // the `location: None` `continue` below (`storage.md` → Placement census:
        // `reconcile_committed_store`). Its record key is `placement.file`; its identity is
        // the fixed `<type>:<type>` slug (the singleton carries no title-derived slug). A
        // missing file is left out of `seen_on_disk`, so the recorded-but-absent arm below
        // routes it through rename detection exactly as a location-keyed doc.
        if let Some(placement) = &schema.placement {
            if let Ok(bytes) = std::fs::read(repo_root.join(&placement.file)) {
                seen_on_disk.insert(placement.file.clone());
                let from = format!("{ty}:{ty}");
                let task_touched = task_dir.join("docs").join(format!("{from}.md")).exists();
                findings.extend(reconcile_committed(
                    record,
                    index,
                    schema,
                    &placement.file,
                    &from,
                    &bytes,
                    task_touched,
                    pinned,
                    conflict,
                    adoption,
                ));
            }
            continue; // a placement doctype has no location dir to glob.
        }
        let Some(location) = schema.location.as_deref() else {
            continue; // a transient (location-less) type has no committed docs.
        };
        let dir = repo_root.join(location);
        let mut slugs: Vec<String> = match std::fs::read_dir(&dir) {
            Ok(entries) => entries
                .flatten()
                .map(|e| e.path())
                .filter(|p| p.extension().and_then(|x| x.to_str()) == Some("md"))
                .filter_map(|p| p.file_stem().and_then(|s| s.to_str()).map(str::to_owned))
                .collect(),
            Err(_) => Vec::new(), // no committed docs of this type yet.
        };
        slugs.sort();

        for slug in &slugs {
            let path = format!("{location}{slug}.md");
            seen_on_disk.insert(path.clone());
            let from = format!("{ty}:{slug}");
            let Ok(bytes) = std::fs::read(repo_root.join(location).join(format!("{slug}.md")))
            else {
                continue; // read race: skip; the next sweep re-checks.
            };
            let task_touched = task_dir.join("docs").join(format!("{from}.md")).exists();
            findings.extend(reconcile_committed(
                record,
                index,
                schema,
                &path,
                &from,
                &bytes,
                task_touched,
                pinned,
                conflict,
                adoption,
            ));
        }
    }

    // Rename detection: a recorded committed path of a persisted type that is no longer
    // on disk is a suspected rename/deletion (`reconciliation.md` → Rename detection).
    // "Missing" means absent from DISK, not absent from the walk (the 2026-06-12
    // amendment): the walk is a non-recursive `<location>/*.md` glob, so a baselined
    // path outside it (e.g. a promoted owner-artifact under `completions/artifacts/`)
    // is checked for genuine disk presence — present-but-unwalked yields no finding and
    // stays baselined as-is. The untracked candidates are the on-disk `.md` files (of
    // any persisted type) that carry no recorded hash, paired with their raw-byte hash.
    let recorded_missing: Vec<String> = recorded_at_entry
        .iter()
        .filter(|p| !seen_on_disk.contains(*p))
        .filter(|p| !repo_root.join(p).exists())
        .cloned()
        .collect();
    if !recorded_missing.is_empty() {
        let untracked = untracked_committed(&recorded_at_entry, schemas, repo_root);
        let untracked_refs: Vec<(&str, String)> = untracked
            .iter()
            .map(|(p, h)| (p.as_str(), h.clone()))
            .collect();
        for path in recorded_missing {
            let recorded_hash = record.get(&path).unwrap_or("").to_string();
            // `jigc rename` post-commit crash-window self-heal (M35): the verb committed
            // the move (the renamed doc tracked at its new path, every referrer already
            // repointed) but crashed *before* re-baselining file-state, leaving the OLD
            // path recorded-missing. When the landing is present in the committed tree — a
            // new same-`location:` doc whose content differs from the recorded baseline
            // (the H1 was rewritten, so it is NOT a strong-signal `git mv` match) — the
            // committed tree is authoritative: drop the stale old key (the landing was
            // baseline-adopted in the walk above, completing the old→new re-key) and emit
            // no finding, never the weak-signal restore that would resurrect a
            // deliberately-renamed-away doc (`reconciliation.md` → Rename detection,
            // self-healing; `write-commands.md` → `jigc rename` step 5). The strong signal
            // (content-preserving bare `git mv`) and the genuine-deletion weak signal
            // (no landing) both still route through detect_rename below.
            let strong = untracked_refs.iter().any(|(_, h)| *h == recorded_hash);
            if !strong && rename_landing_present(&path, &untracked_refs, schemas) {
                record.forget(&path);
                continue;
            }
            let from = identity_of(&path, schemas).unwrap_or_else(|| path.clone());
            findings.extend(detect_rename(
                &path,
                &from,
                &recorded_hash,
                &untracked_refs,
                history,
                other_refs,
                live,
                repo_root,
            ));
        }
    }

    findings
}

/// The **read-only file↔CLI-state store twin** — `jigc validate`'s store-scope
/// file-state target (`validation.md` → Completing the envelope → read-only
/// file↔CLI-state at store scope; M20). The *detect-without-absorb* complement of
/// [`reconcile_committed_store`]: it compares each committed managed doc's on-disk
/// content hash to its recorded [`FileStateRecord`] and **reports** drift, opening
/// **no** record write and **never** routing through the mutating reconcile path
/// (no absorb, no re-baseline, no edge-index touch) — so it cannot silently
/// re-baseline the very drift the sweep exists to surface.
///
/// Enumerates every committed instance through [`crate::index::committed_instances`] —
/// a located type's `<location>/*.md` files **and** a **placement** type's single literal
/// `placement.file` — the same placement-aware enumerator its mutating twin's walk covers
/// by hand (`file_state.rs` → the placement arm) and the store's conformance/hollow
/// families already share. Before M42 it walked only `location:`-bearing schemas, so an
/// out-of-band edit to a baselined `CHANGELOG.md`/`VISION.md` was **silently invisible** to
/// `jigc validate` — the placement class fell out of the *"out-of-band edits are detected
/// and routed"* invariant (`design/storage.md` → The census: `detect_committed_store`).
/// Each instance's record key is its **repo-relative path** (a placement doc's is its
/// literal `placement.file`, matching the key the mutating twin records), and each
/// **present** doc routes by [`FileStateRecord::get`] vs [`hash_bytes`] of the on-disk
/// bytes:
///
/// - **content drift** (recorded hash ≠ on-disk hash) → exactly one blocking
///   `file-state.hash-matches` finding (the reused check id) carrying a
///   **store-scope route** (review / re-author through the owning workflow), never
///   the task-scope `reconcile <path>` route the mutating path emits — **unless the
///   baseline merely lags `HEAD`** (M55 Increment 4, L1's store arm): when the on-disk
///   bytes equal the doc's blob at `HEAD` (`head`, the CLI-supplied
///   [`PinnedBlob`](crate::validate::PinnedBlob) bound to `HEAD`) **and** pass the
///   conformance gate, the drift is committed — a pull, or a conformant edit committed with
///   plain git — so the same finding is **advisory**, routed informationally *"the baseline
///   lags `HEAD`; absorbed at the next finalize"*. A committed non-conformant edit keeps the
///   blocking finding (and family 5 its conformance finding); nothing is baselined either
///   way. `head` is asked only about a drifted doc, so a clean store shells out zero times.
/// - **un-baselined** (no recorded hash) → exactly one **advisory**
///   `file-state.un-baselined` finding — a distinct *not-yet-tracked* outcome,
///   neither drift nor silent-clean (informational on a fresh / pre-baseline
///   store, so it does not flip the exit code) — **unless the doc is a never-adopted
///   foreign squatter**, below.
/// - **in-sync** (recorded hash matches) → no finding.
///
/// **A foreign squatter is not un-baselined (M42).** Since the enumerator reaches
/// **placement** homes, this twin also lands on a brownfield repo's own Keep-a-Changelog
/// `CHANGELOG.md` — a file the user never handed to jigc — where it has **no record** and
/// so read as *"un-baselined … no action needed — the doc is baselined on its next author
/// or finalize"*: a promise about a file **jigc will never author**. The un-baselined
/// advisory is a claim about a **managed** doc, so it now runs the same committed-bytes
/// discriminator family 5 does ([`crate::validate::is_unadopted_foreign`]) and stays silent
/// on a foreign instance — which is **never left silent overall**: family 5 gives it the one
/// finding it earns, the `schema-conformance.unadopted-instance` adoption advisory
/// (`design/validation.md` → *the same discriminator applies to family 3's `un-baselined`*).
/// The suppression is scoped to the **un-baselined arm alone**: a *recorded* doc keeps both
/// outcomes unchanged, so an OOB edit that mangles a managed doc past parsing still reports
/// its drift and never escapes as "foreign".
///
/// The discriminator keys on the doc's **committed bytes** (stamp, else a parse against the
/// current or a shipped prior shape) — never on `record` membership, which lives in
/// gitignored `.jigc/state/` and is **empty on a fresh clone**, where every managed doc would
/// otherwise read foreign and lose its un-baselined advisory.
///
/// **Content drift on *present* docs only.** Recorded-but-now-missing docs
/// (rename / deletion — [`detect_rename`]'s task-scope concern) are **out of scope**:
/// the twin walks on-disk docs and never enumerates recorded-but-absent paths.
///
/// Read-only by construction: `record` is borrowed `&` (no mutation possible) and
/// the only I/O is reading the committed `.md` bytes (and the caller's `head` lookup, on a
/// drifted doc) — it never routes through the mutating [`reconcile_committed_store`]. Findings aggregate in a stable order —
/// persisted schemas by type, then that type's instances in the enumerator's sorted order.
pub fn detect_committed_store(
    record: &FileStateRecord,
    schemas: &std::collections::BTreeMap<String, crate::schema::Schema>,
    repo_root: &Path,
    head: &crate::validate::PinnedBlob<'_>,
    versions: &std::collections::BTreeMap<String, u32>,
    priors: &std::collections::BTreeMap<String, Vec<crate::schema::Schema>>,
) -> Vec<Finding> {
    let mut findings = Vec::new();
    for (ty, schema) in schemas {
        for (identity, path) in crate::index::committed_instances(repo_root, ty, schema) {
            // The record key is the repo-relative path the mutating twin records the doc
            // under — for a placement doc, its case-preserved literal `placement.file`
            // (never re-derived from the path, which does not round-trip through slug
            // derivation).
            let key = path
                .strip_prefix(repo_root)
                .unwrap_or(&path)
                .to_string_lossy()
                .into_owned();
            let Ok(bytes) = std::fs::read(&path) else {
                continue; // read race: skip; the next sweep re-checks.
            };
            let current = hash_bytes(&bytes);
            match record.get(&key) {
                // No record — *not-yet-baselined*, but only if the doc is jigc's to baseline.
                // A never-adopted **foreign** file squatting at a managed home has no record
                // for the same reason it has no stamp, and calling it un-baselined would
                // promise a baseline on an author/finalize that will never come. Classify from
                // the committed bytes (family 5's discriminator) and stay silent on it — it is
                // family 5's `schema-conformance.unadopted-instance` adoption case, never a
                // silent one.
                None => {
                    let source = String::from_utf8_lossy(&bytes);
                    let slug = identity.split_once(':').map_or("", |(_, slug)| slug);
                    if !crate::validate::is_unadopted_foreign(
                        ty, slug, schema, &source, versions, priors,
                    ) {
                        findings.push(unbaselined_finding(&key));
                    }
                }
                // A **recorded** doc is managed by construction (jigc baselined it), so both
                // outcomes stand unclassified: an OOB edit that mangles it past parsing is
                // drift to route, never a file to "adopt".
                Some(recorded) if recorded == current => {}
                // Drifted. When the on-disk bytes equal the doc's blob at `HEAD` **and** conform,
                // the baseline merely lags `HEAD` — a pull, or a conformant edit committed with
                // plain git — and the next landed finalize absorbs it (M55 Increment 4, L1's
                // store arm): advisory, the same `(code, target)` key. Every other drift keeps
                // the blocking finding, a committed non-conformant edit included. The seam is
                // asked only here, so a clean store shells out zero times.
                Some(_) => {
                    let lags_head = head(&key).is_some_and(|blob| hash_bytes(&blob) == current)
                        && conformance_gate(schema, &bytes).is_ok();
                    findings.push(if lags_head {
                        lagging_baseline_finding(&key)
                    } else {
                        drift_store_finding(&key)
                    });
                }
            }
        }
    }
    findings
}

/// **Read-only store-scope rename detection** — the *recorded-but-missing* arm of
/// [`reconcile_committed_store`] lifted into the read-only file↔CLI-state family
/// ([`detect_committed_store`]'s sibling; `validation.md` → file↔CLI-state: recorded-but-
/// missing rename detection added at M35; `reconciliation.md` → Out-of-band rename,
/// Component A). It enumerates every recorded committed path **absent on disk** and runs the
/// **pure, non-mutating** [`detect_rename`] over the untracked candidates, so a bare `git mv`
/// committed without `jigc rename` is diagnosed as *a rename to adopt/revert* — including the
/// **referrer-less** move no dangling-ref check can catch.
///
/// Returns the `reconciliation.rename` findings **and** the set of missing `<type>:<slug>`
/// identities, so the caller can **scope-subtract** those identities' inbound edges from
/// [`ref_resolves_store`](crate::index::ref_resolves_store) for the same sweep — the dedup
/// that keeps one OOB event from emitting both a rename finding *and* N competing dangling-ref
/// findings, while `jigc validate` still reports every *other* dangling ref (no short-circuit).
///
/// **Read-only by construction** — `record` is borrowed `&`: the `jigc rename` post-commit
/// crash-window landing (a same-`location:` doc whose content *differs* from the recorded
/// baseline, so it is no strong-signal `git mv` match) is **suppressed** (skipped, no finding)
/// but **never** self-healed here, because that heal is a `record.forget` write — the mutating
/// self-heal stays the task-scope [`reconcile_committed_store`]'s job. So the read-only twin
/// never resurrects a deliberately-renamed-away doc via the weak-signal restore, and never
/// mutates the record. The only I/O is reading the committed `.md` bytes (for the untracked
/// candidates' hashes). Findings + identities aggregate in path-sorted order.
///
/// `history` is the CLI-supplied [`HistoryPredicate`](crate::validate::HistoryPredicate) the
/// task gate's twin consults too (M55 Increment 5 / T2): a missing path with no content-
/// matching candidate and no history at `HEAD` is the **advisory** dangling baseline, one with
/// history the **blocking** weak deletion. It is asked only of such a path, so a store with no
/// missing baseline never consults it. `other_refs`
/// ([`OtherRefsPredicate`](crate::validate::OtherRefsPredicate)) routes that advisory exactly as
/// at task scope (M55 completion triage, CR2): switch back when a branch carries the doc,
/// `jigc unmanage` when none does.
pub fn detect_committed_store_renames(
    record: &FileStateRecord,
    schemas: &std::collections::BTreeMap<String, crate::schema::Schema>,
    repo_root: &Path,
    history: &crate::validate::HistoryPredicate<'_>,
    other_refs: &crate::validate::OtherRefsPredicate<'_>,
) -> (Vec<Finding>, std::collections::BTreeSet<String>) {
    let mut findings = Vec::new();
    let mut renamed: std::collections::BTreeSet<String> = std::collections::BTreeSet::new();

    // The recorded committed managed-doc paths (excludes staged working-area keys / code
    // paths via `persisted_committed_path`), and which of them are now absent on disk
    // ("missing" means absent from DISK — the 2026-06-12 amendment; present-but-unwalked is
    // not missing).
    let recorded: std::collections::BTreeSet<String> = record
        .hashes
        .keys()
        .filter(|p| persisted_committed_path(p, schemas))
        .cloned()
        .collect();
    let recorded_missing: Vec<String> = recorded
        .iter()
        .filter(|p| !repo_root.join(p).exists())
        .cloned()
        .collect();
    if recorded_missing.is_empty() {
        return (findings, renamed);
    }

    let untracked = untracked_committed(&recorded, schemas, repo_root);
    let untracked_refs: Vec<(&str, String)> = untracked
        .iter()
        .map(|(p, h)| (p.as_str(), h.clone()))
        .collect();
    for path in recorded_missing {
        let recorded_hash = record.get(&path).unwrap_or("").to_string();
        // The post-commit crash-window landing is suppressed read-only — skip without
        // `forget` (the heal is the mutating reconcile's job), so the weak-signal restore
        // never resurrects a renamed-away doc. The strong signal (content-preserving bare
        // `git mv`) and the genuine-deletion weak signal both still route below.
        let strong = untracked_refs.iter().any(|(_, h)| *h == recorded_hash);
        if !strong && rename_landing_present(&path, &untracked_refs, schemas) {
            continue;
        }
        let from = identity_of(&path, schemas).unwrap_or_else(|| path.clone());
        // The store twin grades the weak signal by the caller's history predicate, exactly as
        // the task gate does (M55 Increment 5 / T2, revising the M45 Increment 7 store/task
        // split): a path with no history at `HEAD` is the advisory dangling baseline, its route
        // picked by the same `other_refs` (switch back, or `jigc unmanage` when no branch
        // carries it — M55 completion triage, CR2), so both scopes report one key at one
        // severity under one route.
        // Pinned by `file_state_history_gate::store_scope_agrees_with_task_scope_on_a_history_less_baseline`.
        // No live record at store scope: this twin sweeps no task, so no work unit owns the
        // sweep and the caller-supplied carve-out has no subject (`LiveRecord::none`).
        let detected = detect_rename(
            &path,
            &from,
            &recorded_hash,
            &untracked_refs,
            history,
            other_refs,
            &LiveRecord::none(),
            repo_root,
        );
        if !detected.is_empty() {
            renamed.insert(from);
            findings.extend(detected);
        }
    }
    (findings, renamed)
}

/// The store-scope drift finding: the on-disk content no longer matches the
/// recorded hash, surfaced read-only by [`detect_committed_store`]. Reuses the
/// `file-state.hash-matches` check id (no new id / knob) but carries the
/// **store-scope route** — there is no task and the mutating reconcile is barred, so
/// it routes to *review / re-author through the owning workflow*, never
/// `reconcile <path>` (`validation.md` → Completing the envelope: the reused
/// `hash-matches` id carries a store-scope route, not `reconcile <path>`).
fn drift_store_finding(path: &str) -> Finding {
    Finding::graded(
        Severity::Blocking,
        "file-state.hash-matches",
        format!("on-disk content of `{path}` differs from the recorded state"),
        Some(Location::addressed(path, 1, 1)),
        Some(format!(
            "review the out-of-band edit to `{path}` and re-author it through the owning workflow"
        ).into()),
    )
}

/// The store-scope drift finding **graded by L1's store arm** (M55 Increment 4): a recorded
/// doc whose on-disk bytes differ from its baseline but equal its blob at `HEAD`, and conform.
/// The drift is committed, so it is not an out-of-band edit waiting in the worktree — the
/// baseline lags `HEAD`, and the next landed finalize's sweep absorbs it through the
/// conformance gate this arm has already passed. It **is** the [`drift_store_finding`] —
/// the `file-state.hash-matches` id, message and location, so the `(code, target)` key does
/// not move (the shipped no-new-id pattern) — with only the severity (advisory) and the route
/// (informational: it directs nothing) changed.
fn lagging_baseline_finding(path: &str) -> Finding {
    Finding {
        severity: Severity::Advisory,
        route: Some(crate::finding::Route::informational(
            "the baseline lags `HEAD`; absorbed at the next finalize",
        )),
        ..drift_store_finding(path)
    }
}

/// The advisory **un-baselined** finding: a committed **managed** doc with no recorded
/// hash, surfaced by [`detect_committed_store`] (its route promises a baseline *"on its next
/// author or finalize"*, so it is emitted only for a doc jigc will in fact author — never for
/// a foreign squatter, M42). A distinct *not-yet-tracked*
/// outcome — neither drift nor silent-clean — so a read-only twin that cannot adopt
/// a baseline (the mutating path's UNKNOWN resolution) never falsely reports an
/// untracked doc as clean. Advisory + an **informational route** (the advisory-route
/// floor — every finding routes, never `null`; the no-op is reified as an explicit
/// route value, not an absence): informational on a fresh / pre-baseline store, it
/// does not flip the exit code (`validation.md` → Completing the envelope: UNKNOWN
/// (un-baselined) ≠ clean; the advisory-route floor).
fn unbaselined_finding(path: &str) -> Finding {
    Finding::graded(
        Severity::Advisory,
        "file-state.un-baselined",
        format!("committed doc `{path}` is not yet baselined in the file-state record"),
        Some(Location::addressed(path, 1, 1)),
        Some("no action needed — the doc is baselined on its next author or finalize".into()),
    )
}

/// **The absorb question, asked once for the two doors that answer it outside the
/// classifier** (M52 Increment 8 / T3): would advancing `path`'s recorded baseline to
/// `bytes` carry an **out-of-band edit** forward — and if so, the advisory that says so.
///
/// [`reconcile_committed`] classifies drift and emits `reconciliation.absorb` on the arm
/// that absorbs it, so the task/finalize sweep already obeys
/// [reconciliation.md](../../../design/reconciliation.md)'s *"every absorb surfaces"*. Two
/// **register-only** doors advance a committed doc's baseline without going through it —
/// `jigc ingest` (via [`crate::ingest::adopt`]) and `jigc rename` (via the move primitive's
/// re-key and its referrer re-baseline) — and through `1.0.0-rc.15` both did it in silence,
/// so a blocking `file-state.hash-matches` simply vanished from the next `jigc validate`
/// with no line on any surface saying an external edit had been taken
/// (`completions/artifacts/M52/baseline-freeze.md` §2.2 F1/F3 and §4 L-1, both driven).
///
/// The predicate is the *same comparison* the classifier's `DRIFTED` arm makes — a recorded
/// hash that differs from the bytes in hand — asked here rather than re-derived at each
/// door, so the two doors cannot disagree with each other or with the sweep about what
/// counts as an out-of-band edit. **`None` is the ordinary answer** on every clean corpus:
/// no recorded baseline is not drift (that is the `UNKNOWN` arm's `file-state.un-baselined`
/// business), and a matching one is `IN_SYNC`.
///
/// It **classifies nothing and records nothing**: the caller passes the bytes it is about to
/// baseline, so this reports what the door did rather than deciding it. Which drift a door is
/// willing to absorb at all is that door's own gate, and the two differ — `adopt` re-gates
/// parse + conformance and refuses a non-conformant edit, while `rename` has no such gate and
/// absorbs one (driven at M52 Increment 8 / T3: the blocking `conformance.section-renamed`
/// survives into the next `jigc validate`, so nothing is greened). Hence the message makes
/// **no** conformance claim — only the baseline claim, which is true at both doors.
pub fn absorbed_drift(record: &FileStateRecord, path: &str, bytes: &[u8]) -> Option<Finding> {
    match record.get(path) {
        Some(recorded) if recorded != hash_bytes(bytes) => Some(absorbed_baseline_finding(path)),
        _ => None,
    }
}

/// The advisory **`file-state.absorbed`** finding — the surface
/// [reconciliation.md](../../../design/reconciliation.md) → *What reconciliation does NOT do*
/// has always owed at a register-only absorb.
///
/// Informational, like its `reconciliation.absorb` sibling: a clean external edit is honored,
/// not a problem to repair. It carries an **`Informational` route naming the finding it
/// retired**, because the reader's question at this line is *why did `jigc validate` stop
/// telling me about this file* — and the answer is that this run is what stopped it.
fn absorbed_baseline_finding(path: &str) -> Finding {
    Finding::graded(
        Severity::Advisory,
        "file-state.absorbed",
        format!(
            "out-of-band edit to `{path}` absorbed — its file-state baseline now records the \
             on-disk bytes"
        ),
        Some(Location::addressed(path, 1, 1)),
        // `Route::informational` explicitly, never `String::into` — that `From` impl maps to
        // `RouteKind::Human`, so an *informs-only* route written as a bare string is silently
        // filed as a direction a human must take. The registration says `Informational` and
        // the constructed kind has to agree with it.
        Some(crate::finding::Route::informational(format!(
            "no action needed — this retires the `file-state.hash-matches` finding for \
             `{path}`; review the edit in git history if it was not yours"
        ))),
    )
}

/// Whether `path` (a `file-state` record key like `decisions/x.md`) lives under a
/// persisted schema's `location:` — i.e. it is a committed managed doc, not a code
/// path. The `:` exclusion is a structural guard on the staged working-area namespace
/// (`docs/<type>:<slug>.md` — no committed path can carry a `:`: committed docs are
/// `<location>/<slug>.md`, slugs `[a-z0-9-]` per [`crate::slug::slugify`]); since M43
/// A14 staged keys never enter the record at all (the task sweep is record-silent for
/// staged instances), so no live record should ever hit it.
fn persisted_committed_path(
    path: &str,
    schemas: &std::collections::BTreeMap<String, crate::schema::Schema>,
) -> bool {
    !path.contains(':')
        && (schemas
            .values()
            .filter_map(|s| s.location.as_deref())
            .any(|loc| path.starts_with(loc) && path.ends_with(".md"))
            || schemas
                .values()
                .filter_map(|s| s.placement.as_ref())
                .any(|p| p.file == path))
}

/// The `<type>:<slug>` identity for a committed record path (`decisions/x.md` → its
/// persisted type's `adr:x`), or `None` when no persisted schema owns it.
fn identity_of(
    path: &str,
    schemas: &std::collections::BTreeMap<String, crate::schema::Schema>,
) -> Option<String> {
    for (ty, schema) in schemas {
        // A placement doctype (`location: None`) owns its one literal `placement.file`
        // by exact-path equality; its slug is fixed = the type id (`storage.md` →
        // Placement census: identity stays explicit, never filename-derived).
        if let Some(placement) = &schema.placement
            && placement.file == path
        {
            return Some(format!("{ty}:{ty}"));
        }
        let Some(location) = schema.location.as_deref() else {
            continue;
        };
        if let Some(rest) = path.strip_prefix(location)
            && let Some(slug) = rest.strip_suffix(".md")
        {
            return Some(format!("{ty}:{slug}"));
        }
    }
    None
}

/// The on-disk committed `.md` files (under any persisted `location:`) that carry **no**
/// recorded hash — the untracked rename candidates, each paired with its raw-byte hash
/// (`reconciliation.md` → Rename detection: an untracked path with a content-hash match
/// is a suspected `git mv`).
fn untracked_committed(
    recorded_at_entry: &std::collections::BTreeSet<String>,
    schemas: &std::collections::BTreeMap<String, crate::schema::Schema>,
    repo_root: &Path,
) -> Vec<(String, String)> {
    let mut out = Vec::new();
    for schema in schemas.values() {
        // A placement doctype's one on-disk instance is a candidate too: an untracked
        // (no recorded hash) `placement.file` present on disk is a rename candidate,
        // enumerated by exact literal path — never a root dir-glob, so a sibling root
        // `README.md` is not swept in (`storage.md` → Placement census: `untracked_committed`).
        //
        // **Unless the declared home is EMPTY** (M53 — the pre-v1 usability batch, row 1 /
        // the rc.19 review's `(2, N-2)`). A bare `git mv` of a placement singleton lands the
        // file at a path that is, by definition, **not** the declared one — so the
        // literal-path-only census could never produce a candidate for it, `detect_rename`
        // could only ever reach its **weak** arm, and the whole placement family
        // (`VISION.md`, `CHANGELOG.md`, `docs/roadmap.md`, `docs/decisions-log.md` — every
        // managed singleton a stock corpus has) was told *"is missing"* and routed to
        // `jigc unmanage`, the one act that would drop the identity of a doc sitting right
        // there under a new name. The installed pre-commit hook's blocking backstop keys on
        // the strong arm's `mv` pair, so it was structurally inert over that whole family.
        //
        // The widen is conditioned on the declared home being absent — the one state in
        // which a rename of this singleton is possible at all — so in the ordinary case the
        // census is byte-identical to before and no sibling is read. What a widened
        // candidate can *become* is still gated by an exact recorded-content-hash match, so
        // an unrelated sibling `README.md` contributes a candidate and never a finding.
        if let Some(placement) = &schema.placement {
            let declared = repo_root.join(&placement.file);
            if declared.exists() {
                if !recorded_at_entry.contains(&placement.file)
                    && let Ok(bytes) = std::fs::read(&declared)
                {
                    out.push((placement.file.clone(), hash_bytes(&bytes)));
                }
                continue;
            }
            out.extend(placement_home_siblings(
                &placement.file,
                recorded_at_entry,
                repo_root,
            ));
            continue;
        }
        let Some(location) = schema.location.as_deref() else {
            continue;
        };
        let dir = repo_root.join(location);
        let Ok(entries) = std::fs::read_dir(&dir) else {
            continue;
        };
        for path in entries
            .flatten()
            .map(|e| e.path())
            .filter(|p| p.extension().and_then(|x| x.to_str()) == Some("md"))
        {
            let Some(slug) = path.file_stem().and_then(|s| s.to_str()) else {
                continue;
            };
            let key = format!("{location}{slug}.md");
            if recorded_at_entry.contains(&key) {
                continue; // tracked at entry, not an untracked candidate.
            }
            if let Ok(bytes) = std::fs::read(&path) {
                out.push((key, hash_bytes(&bytes)));
            }
        }
    }
    out.sort();
    out.dedup();
    out
}

/// The untracked `.md` siblings of a placement doctype's **vacated** declared home — the
/// rename candidates for a singleton whose one literal path is gone
/// (`untracked_committed`'s placement arm; M53 — the pre-v1 usability batch, row 1).
///
/// The home's own directory is the search space, and it is derived from the declared file
/// itself rather than named: a root-declared home (`VISION.md`) searches the repository
/// root, a directory-declared one (`docs/roadmap.md`) searches that directory — the two
/// shapes a `placement.file` can take. Keys are rebuilt with the home's own directory
/// prefix, so a candidate is addressed exactly as a recorded path is.
///
/// Recorded paths are excluded (they are tracked, not candidates). Nothing else is: the
/// *content-hash match* in [`detect_rename`] is what turns a candidate into a finding, and
/// narrowing here by name or by doctype would only re-introduce the assumption this
/// function exists to drop — that a renamed file is still findable at its declared path.
fn placement_home_siblings(
    declared_file: &str,
    recorded_at_entry: &std::collections::BTreeSet<String>,
    repo_root: &Path,
) -> Vec<(String, String)> {
    let mut out = Vec::new();
    let (dir, prefix) = match Path::new(declared_file).parent() {
        Some(parent) if !parent.as_os_str().is_empty() => (
            repo_root.join(parent),
            format!("{}/", parent.to_string_lossy()),
        ),
        _ => (repo_root.to_path_buf(), String::new()),
    };
    let Ok(entries) = std::fs::read_dir(&dir) else {
        return out;
    };
    for path in entries
        .flatten()
        .map(|e| e.path())
        .filter(|p| p.extension().and_then(|x| x.to_str()) == Some("md"))
    {
        let Some(name) = path.file_name().and_then(|s| s.to_str()) else {
            continue;
        };
        let key = format!("{prefix}{name}");
        if recorded_at_entry.contains(&key) {
            continue; // tracked at entry, not an untracked candidate.
        }
        if let Ok(bytes) = std::fs::read(&path) {
            out.push((key, hash_bytes(&bytes)));
        }
    }
    out
}

/// Whether a `jigc rename` **landing** is present in the committed tree for a
/// recorded-but-missing old `path` — the signal that distinguishes the verb's
/// post-commit crash window (a committed rename whose file-state re-baseline was
/// interrupted) from a genuine deletion (`reconciliation.md` → Rename detection,
/// self-healing; `write-commands.md` → `jigc rename` step 5).
///
/// True iff some `untracked` candidate (an on-disk `.md` carrying no recorded hash —
/// the renamed doc's new path, baseline-adopted in the same sweep) lives under the old
/// path's own `location:` (a rename preserves the doctype, so the landing shares the
/// directory). A content-hash *match* is the strong-signal bare-`git mv` case the caller
/// already excludes; here the landing's content differs (its H1 was rewritten).
fn rename_landing_present(
    path: &str,
    untracked: &[(&str, String)],
    schemas: &std::collections::BTreeMap<String, crate::schema::Schema>,
) -> bool {
    let Some(location) = schemas
        .values()
        .filter_map(|s| s.location.as_deref())
        .find(|loc| path.starts_with(*loc))
    else {
        return false;
    };
    untracked.iter().any(|(p, _)| p.starts_with(location))
}

/// **Rename detection** — the separate classifier for a tracked managed-doc path
/// gone *missing* on disk (`reconciliation.md` → Rename detection; `storage.md` →
/// Identity: a path change is an identity change). Runs at the same trigger points
/// as [`reconcile_committed`], but only when the recorded path is **absent** — the
/// state machine there sees the old path as missing and a moved file as a fresh
/// untracked file, so neither catches a `git mv`.
///
/// Given the missing tracked `path`, its `<type>:<slug>` identity `from`, its
/// `recorded_hash`, and the on-disk `untracked` candidates (`(path, raw-byte hash)`
/// pairs the caller collected), it emits exactly one blocking `reconciliation.rename`
/// finding routed for human-side revert:
///
/// - **Strong signal** — some untracked path carries the **same** `recorded_hash`: a
///   suspected `git mv`. The finding names both paths and routes to the owned op first
///   (`jigc rename <from> --to "<New Title>"`, which re-points every referrer
///   atomically), revert (`git mv <suspect> <tracked>`) second. The first hash-matching
///   candidate in `untracked` order is named.
/// - **Weak signal** — no untracked path matches: the file is simply gone. `history`
///   grades it (M45, Decision 7): a path that **has** HEAD history was genuinely deleted
///   — the finding **blocks** and routes to restore (or confirm the deletion via `jigc
///   unmanage`). A path with **no** history is a **dangling baseline** — the recorded
///   baseline pointing at a path the checkout moved out from under the gitignored
///   file↔state cache (`git reset --hard` / branch switch / rebase past the creating
///   commit), or an ingested doc deleted before it was ever committed — which downgrades
///   to an **advisory**, so a moved checkout no longer wedges every subsequent task. Its
///   route is picked by the CLI-supplied `other_refs` (M55 completion triage, CR2): a doc
///   some branch still carries is the branch switch, routed informationally at switching
///   back, never an index drop (M55 Increment 5 / T1); a doc no branch carries is gone
///   from everything a checkout can switch to, routed at `jigc unmanage <path>`
///   ([`rename_orphaned_baseline_finding`]). The one exception is the
///   caller's [`LiveRecord`] (M52 Inc 10 / T6): when the history-less path **is** the
///   committed record of the work unit the swept task belongs to, the same id and severity
///   carry the live-record claim and its own informational route instead
///   ([`live_record_finding`]).
///
/// **No auto-rewrite.** A path rename is an identity change; the MVP blocks and routes
/// to revert, and **never** rewrites referrer refs or mutates the edge index
/// (`reconciliation.md` → No silent rename). Dropping a baseline is likewise **never**
/// automatic: silently forgetting a genuinely deleted managed doc would regress *"detected
/// and routed, never silently absorbed."* This function is pure of I/O and of any
/// edge/referrer mutation by construction — it reads its inputs (including the
/// CLI-supplied `history` predicate) and returns findings.
///
/// **The op-axis collapse argument** (the 2026-07-24 confidence audit, sibling-hunt
/// finding 7). This oracle consults exactly four observables — the path is absent from
/// disk (the caller's precondition), an untracked candidate carries the recorded hash,
/// `history(path)`, and (in the history-less cell only, where it picks the route and never
/// the severity) `other_refs(path)`, whether some branch tip carries the path — and nothing
/// else: no git object existence (a `gc` changes nothing), no tag, stash or reflog, no
/// sparse state. So every orphaning git *operation* projects onto one row of the
/// (candidate × history × branch-carried) table, and testing the observables covers the
/// ops — **for this oracle**. (`other_refs` arrived at the M55 completion triage, CR2: a
/// branch delete now moves its row from the switch-back route to the `unmanage` route.) The collapse does NOT survive an oracle change: the ops are exactly where
/// oracle *choice* diverges (e.g. `git log --all` re-blocks the branch-switch row Decision
/// 7 chose HEAD-scoping to downgrade), which is why
/// `crates/cli/tests/file_state_history_gate.rs` iterates the distinct **ops as real git
/// operations** (its module doc carries the row table and the per-row collapse notes).
/// Changing what this oracle consults requires re-deriving that table — the op suite will
/// redden on any divergence. The M52 [`LiveRecord`] carve-out does **not** touch it: it is
/// not a git observable at all — no git operation creates, removes or moves it — so every
/// row still projects exactly as before, and the carve-out only re-presents one already-
/// classified cell (history-less × no candidate) for one caller-named path.
///
/// **Declared conservative bound — sparse-checkout (a false-deletion shape).** A
/// sparse-checkout that excludes a baselined doc's path reads (absent from the worktree ×
/// history present) and classifies as the blocking weak deletion, though nothing was
/// deleted — the file is merely unmaterialized. Pinned as-is
/// (`file_state_history_gate::sparse_checkout_absence_classifies_as_weak_deletion_block`):
/// blocking is the safe direction (it over-blocks, never false-prunes), and in-oracle
/// sparse detection is deliberately not built — re-weigh only if a sparse-checkout user
/// exists (`DECISIONS.md` → 2026-07-23 M45 Settle, Decision 7, the bracketed 2026-07-24
/// note).
// Each parameter is a distinct observable or caller fact the classification reads (the
// path, its identity and hash, the candidates, the two CLI-supplied git predicates, the
// caller's live record, the checkout root); bundling them would only relocate the arity.
#[allow(clippy::too_many_arguments)]
pub fn detect_rename(
    path: &str,
    from: &str,
    recorded_hash: &str,
    untracked: &[(&str, String)],
    history: &crate::validate::HistoryPredicate<'_>,
    other_refs: &crate::validate::OtherRefsPredicate<'_>,
    live: &LiveRecord,
    home: &Path,
) -> Vec<Finding> {
    match untracked.iter().find(|(_, hash)| hash == recorded_hash) {
        Some((suspect, _)) => vec![rename_strong_finding(path, from, suspect, home)],
        // History-gate the weak signal (M45, Decision 7). `history(path)` is `git log HEAD
        // -1 -- <path>` non-empty: present → a genuine deletion keeps blocking; empty → a
        // dangling baseline the checkout moved underneath the cache, downgraded to advisory.
        //
        // Accepted bound (`design/storage.md` → Derived caches): a working copy truncated to
        // a shallow clone can false-prune — at a shallow depth the deleting commit reads as a
        // root and its deletion diff never materializes, so the path reads history-less and
        // this arm downgrades a genuine deletion. It under-blocks, never over-blocks.
        None if history(path) => vec![rename_weak_finding(path, from)],
        // The caller's live-record carve-out (M52 Inc 10 / T6): the one history-less path
        // that is no other branch's doc but the record the milestone is run from. Same id,
        // same advisory severity — only what it says about the state, and what it tells the
        // reader, differ.
        None => match live.unit_at(path) {
            Some(unit) => vec![live_record_finding(path, from, unit)],
            // The cause split (M55 completion triage, CR2): a branch that still carries the
            // doc is the branch switch, whose route never drops the index; no branch
            // carrying it means nothing the checkout can switch to brings it back, and the
            // route names the one way to clear the baseline.
            None if other_refs(path) => vec![rename_dangling_baseline_finding(path, from)],
            None => vec![rename_orphaned_baseline_finding(path, from)],
        },
    }
}

/// The blocking **strong-signal** rename finding (`reconciliation.md` → Rename
/// detection → strong signal): the missing tracked doc and the content-matching
/// suspect, routed to **adopt the move as `jigc rename`** (the owned op that re-points
/// every referrer atomically) or revert the suspected `git mv`. The detector itself
/// leaves referrer refs untouched — it hands the identity change to the owned op or the
/// human.
fn rename_strong_finding(path: &str, from: &str, suspect: &str, home: &Path) -> Finding {
    // `git mv` takes paths, not pathspecs — `:/`-prefixing is refused outright
    // (`fatal: bad source`) — so the revert names the checkout it runs in and keeps both
    // operands repo-relative to it (M53 — the cwd census, C1-09).
    let revert = crate::finding::git_at(
        home,
        &format!(
            "mv {suspect_token} {path_token}",
            suspect_token = crate::finding::shell_token(suspect),
            path_token = crate::finding::shell_token(path)
        ),
    );
    Finding::graded(
        Severity::Blocking,
        "reconciliation.rename",
        format!(
            "tracked managed doc {from} ({path}) is missing; {suspect} has the same content hash — likely renamed via `git mv`"
        ),
        Some(Location::addressed(path, 1, 1)),
        Some(format!(
            "adopt it as a CLI-owned rename (re-points every referrer atomically): `jigc rename {from} --to \"<New Title>\"`; or revert the move: `{revert}`"
        ).into()),
    )
}

/// The blocking **weak-signal** rename finding (`reconciliation.md` → Rename
/// detection → weak signal): the tracked doc is simply gone, no content-matching
/// suspect, routed to restore the file.
fn rename_weak_finding(path: &str, from: &str) -> Finding {
    Finding::graded(
        Severity::Blocking,
        "reconciliation.rename",
        format!("tracked managed doc {from} ({path}) is missing"),
        Some(Location::addressed(path, 1, 1)),
        Some(format!(
            "restore {path}, or confirm the deletion by dropping it from the index: `jigc unmanage {token}`",
            token = crate::finding::shell_token(path)
        ).into()),
    )
}

/// The **advisory** dangling-baseline finding (M45, Decision 7; `design/storage.md` →
/// Derived caches): the recorded baseline points at a path with **no HEAD history**, so
/// nothing was deleted — the baseline is an artifact of the checkout moving underneath the
/// gitignored file↔state cache (a `git reset --hard` / branch switch / rebase past the
/// creating commit). Reuses the `reconciliation.rename` check id at [`Severity::Advisory`]
/// (the established no-new-id advisory pattern in this file, cf.
/// [`conformance_advisory_finding`]), so a moved checkout no longer wedges every subsequent
/// task.
///
/// **One route at both scopes, and never an index drop** (M55 Increment 5 / T1;
/// `design/findings-channel.md` → §6 L2, R5). The route names the branch switch and offers
/// switching back to the branch that carries the path, and says nothing on this checkout
/// needs to change. It is selected only where that is true — some branch, local or
/// remote-tracking, still carries the path (the CLI's `other_refs`; M55 completion triage,
/// CR2) — so pruning here would drop the identity of a doc that is alive on its branch. A
/// path no branch carries takes [`rename_orphaned_baseline_finding`] instead. Like
/// [`live_record_finding`], the route is
/// [`Informational`](crate::finding::RouteKind::Informational) and carries no backticked
/// command, so neither the route fence nor the pre-commit hook's `mv` scan reads anything
/// in it. The message, code, severity and location are M45's, so the `(code, target)` key
/// does not move.
fn rename_dangling_baseline_finding(path: &str, from: &str) -> Finding {
    Finding::graded(
        Severity::Advisory,
        "reconciliation.rename",
        format!(
            "tracked managed doc {from} ({path}) is missing, but the path has no history — the checkout moved underneath the file-state cache, not a deletion"
        ),
        Some(Location::addressed(path, 1, 1)),
        // `Route::informational` explicitly, never `String::into` — that `From` impl files a
        // route as a direction a human must take, and nothing on this checkout is owed.
        Some(crate::finding::Route::informational(format!(
            "nothing on this checkout needs to change — a branch switch left this baseline \
             behind, and {path} lives on a branch this checkout does not carry: switch back \
             to that branch to work on it again"
        ))),
    )
}

/// The **advisory orphaned-baseline** finding (M55 completion triage, CR2;
/// `design/findings-channel.md` → §6 L2): the recorded path is missing, has no history at
/// `HEAD`, and **no branch, local or remote-tracking, carries it** — so no checkout this
/// repository can switch to brings the doc back. Its causes are a `git reset --hard` or a
/// rebase past the creating commit, or a doc ingested and then deleted before it was ever
/// committed; the message and route claim none of them as fact.
///
/// The same `reconciliation.rename` id, [`Severity::Advisory`] and location as
/// [`rename_dangling_baseline_finding`], so the `(code, target)` key and the exit behaviour
/// are the branch-switch arm's; what differs is the claim, and the route, which names the
/// one way to clear the baseline — `jigc unmanage <path>`, the index drop M55 Increment 5
/// retired for the branch-switch case and which is true here, where no branch holds the
/// doc whose identity it drops. A human route (the default `String` route kind), exactly
/// as [`rename_weak_finding`] carries the same command.
fn rename_orphaned_baseline_finding(path: &str, from: &str) -> Finding {
    Finding::graded(
        Severity::Advisory,
        "reconciliation.rename",
        format!(
            "tracked managed doc {from} ({path}) is missing, has no history at HEAD, and no \
             branch carries it — the baseline outlived the doc"
        ),
        Some(Location::addressed(path, 1, 1)),
        Some(
            format!(
                "no branch, local or remote-tracking, can bring {path} back — a hard reset or a \
             rebase past its creating commit, or deleting it before it was ever committed, \
             leaves this baseline behind: drop it with `jigc unmanage {token}`",
                token = crate::finding::shell_token(path)
            )
            .into(),
        ),
    )
}

/// The **live-record** dangling arm (M52 Inc 10 / T6): the recorded path has no HEAD
/// history *and* is the committed record of the work unit the validated task belongs to
/// ([`LiveRecord`]), so the baseline is current and nothing was deleted — this checkout
/// simply predates the record commit (a sub-task is pinned to its milestone's base, which
/// by construction sits before it).
///
/// Reuses the `reconciliation.rename` check id at [`Severity::Advisory`] — the same
/// no-new-id pattern [`rename_dangling_baseline_finding`] already uses, so the
/// `(code, target)` key and the exit behaviour are unchanged; what moves is the claim and
/// the route. The route is **[`Informational`](crate::finding::RouteKind::Informational)**
/// and names no verb: there is nothing to do. (When this carve-out was cut the sibling
/// routed prune-first at `jigc unmanage`, which here would have unmanaged the milestone's
/// own state; since M55 Increment 5 / T1 the sibling routes at the branch switch, which is
/// not true of this path either — the record is current on this branch's milestone.)
fn live_record_finding(path: &str, from: &str, unit: &str) -> Finding {
    Finding::graded(
        Severity::Advisory,
        "reconciliation.rename",
        format!(
            "tracked managed doc {from} ({path}) is absent from this checkout, which carries \
             no history for it — it is the live record of milestone {unit}, the milestone \
             this task belongs to, so nothing was deleted and the baseline is current"
        ),
        Some(Location::addressed(path, 1, 1)),
        // `Route::informational` explicitly, never `String::into` — that `From` impl files a
        // route as a direction a human must take, and there is no act owed here.
        Some(crate::finding::Route::informational(format!(
            "no action needed — do not prune this baseline: it is the live record of \
             milestone {unit}, and it is on disk in any checkout that carries its commit"
        ))),
    )
}

/// The advisory **absorb** finding (`reconciliation.md` → OOB edit → absorb: "external
/// edit absorbed: `<doc>`"). Informational — a clean external edit is honored, not a
/// problem to repair; the absorb already re-hashed + updated the edge index. Carries an
/// **informational route** (the advisory-route floor — every finding routes, never
/// `null`; the no-op is an explicit route value, not an absence).
fn absorb_finding(path: &str) -> Finding {
    Finding::graded(
        Severity::Advisory,
        "reconciliation.absorb",
        format!("external edit absorbed: `{path}`"),
        Some(Location::addressed(path, 1, 1)),
        Some("no action needed — the external edit was absorbed into the baseline".into()),
    )
}

/// The **hand-repair sanction** — the route over a **managed** doc that is at the schema-version
/// this binary knows and still does not conform (round-2 D7): the adapter rule bans hand-editing
/// managed files, but out-of-band damage is repaired where it happened, so this is the one case
/// the file is explicitly yours to hand-repair.
///
/// **One source, two producers** (M48 Inc 4 / T2). The blocking `DRIFTED` twin
/// ([`conformance_block_finding`]) and the advisory `UNKNOWN` arm
/// ([`conformance_advisory_finding`]) say the same thing about the same fact — an at-version
/// managed doc that does not conform — and a second inline copy of these words is a drift
/// waiting to happen: the acceptance suite lifts one producer's emitted route and compares it
/// to the other's, which proves they *agree*, not that they come from the same place. This
/// const is what makes them the same place.
fn hand_repair_sanction() -> String {
    format!(
        "fix the file to restore conformance, or revert the edit — this is the one case a \
         managed file is yours to hand-edit: {OUT_OF_BAND_SANCTION}"
    )
}

/// The **sanction clause itself** — the half of [`hand_repair_sanction`] that says why a
/// hand edit of a managed file is legitimate at all, and the half a *third* producer needs
/// (M46 inc-5 / T2).
///
/// The `DRIFTED + TOUCHED` conflict route's second exit is *"revert the external edit on
/// disk"* — over a managed doc, that is precisely the act `.jigc/AGENT.md`'s routing
/// sentence forbids ("never read or edit managed docs directly"). The route was right and
/// the prohibition is right; what was missing is the sentence that reconciles them, and it
/// already exists here. So it is lifted out of its one consumer rather than retyped into
/// the second — the M48 Inc 4 / T2 lesson, applied to its own const: two producers agreeing
/// is not two producers sharing a source.
///
/// A **fourth** producer joined at M49 Inc 1 / T3, one layer down: the parser's
/// `conformance.duplicate-field` route ([`crate::parse`]) — a repeated declared field line
/// is repaired by deleting it, which is a hand edit of a managed file for exactly this
/// reason.
pub(crate) const OUT_OF_BAND_SANCTION: &str =
    "the damage was made out-of-band, so it is repaired where it happened";

/// The blocking **conformance-block** finding (`reconciliation.md` → OOB edit →
/// conformance-block: "a precise conformance error — file, line, expected shape"). The
/// underlying parse/schema `cause` (re-located onto the file) names exactly what is
/// wrong; the MVP never auto-repairs (`reconciliation.md` → Auto-repair scope).
fn conformance_block_finding(path: &str, cause: Option<Finding>) -> Finding {
    let (detail, line) = match &cause {
        Some(f) => (
            f.message.clone(),
            f.location.as_ref().map(|l| l.line).unwrap_or(1),
        ),
        None => ("the edit is not schema-conformant".to_string(), 1),
    };
    Finding::graded(
        Severity::Blocking,
        "reconciliation.conformance-block",
        format!("nonconformant edit on `{path}`: {detail}"),
        Some(Location::addressed(path, line, 1)),
        Some(hand_repair_sanction().into()),
    )
}

/// The **advisory** conformance-block finding for the G4 baseline-adopt gate (M21;
/// `project-setup.md` → Flow 2 hardening → G4 conformance gate). A non-conformant `.md`
/// sitting in a `location:` dir with no recorded baseline is **routed, not recorded** — so
/// it re-fires every sweep until the human resolves it. Reuses the existing
/// `reconciliation.conformance-block` check id at [`Severity::Advisory`] (the M21 "no new
/// check ids" invariant; the id is not knob-remapped, so Advisory stays advisory). The
/// underlying parse/schema `cause` (re-located onto the file) names exactly what is wrong.
///
/// **The route reads the doc's schema-version stamp** (M48 Inc 4 / T2). Until T1 this arm
/// served foreign ∪ managed and routed *"ingest, migrate, or move `<path>` out of the managed
/// location"* — advice that was right for the foreign majority. T1 sent every file the
/// discriminator adjudicates **foreign** to the adoption advisory, so what is left here is
/// **jigc's own doc**, and adoption-or-removal advice about it is wrong for the whole
/// population. What is right depends on one fact — which schema the doc was written against —
/// and both answers are already shipped strings:
///
/// - **at-version** (or an **unversioned** doctype, where `current` is `None` and the question
///   does not arise) ⇒ [`hand_repair_sanction`], the blocking twin's route: nothing to migrate,
///   so the file really is yours to hand-repair;
/// - **below-version**, **stamp-absent** (the v0-era corpus) or **above-current** ⇒
///   [`crate::validate::route_schema_conformance`]'s version-aware route, the same one the store
///   door labels this doc's findings with. Hand-repair advice over a **stale** doc is itself
///   wrong: it tells the operator to hand-fix what `jigc migrate-corpus` must rewrite.
///
/// The sanction is the **default**, overridden only where the stamp says otherwise — so a
/// doctype the caller supplies no manifest version for keeps the pre-M48 blocking-twin wording
/// by construction, never by a second branch.
fn conformance_advisory_finding(
    path: &str,
    cause: Option<Finding>,
    source: &str,
    current: Option<u32>,
) -> Finding {
    let (detail, line) = match &cause {
        Some(f) => (
            f.message.clone(),
            f.location.as_ref().map(|l| l.line).unwrap_or(1),
        ),
        None => ("the file is not schema-conformant".to_string(), 1),
    };
    let mut finding = Finding::graded(
        Severity::Advisory,
        "reconciliation.conformance-block",
        format!("unvetted file `{path}` in a managed location is not schema-conformant: {detail}"),
        Some(Location::addressed(path, line, 1)),
        Some(hand_repair_sanction().into()),
    );
    // Read the stamp from the RAW front matter, exactly as the discriminator does: a
    // below-version doc of a structurally-changed doctype does not parse under the current
    // schema — which is why it is in this arm at all.
    let stamp = crate::validate::schema_version_from_front_matter(source);
    if stamp != current {
        crate::validate::route_schema_conformance(
            std::slice::from_mut(&mut finding),
            stamp,
            current,
            path,
        );
    }
    finding
}

/// The blocking **conflict-block** finding (`reconciliation.md` → Conflict — block at
/// file level): both the on-disk file and the CLI side moved. File granularity, an
/// explicit resolution route, never a silent merge (three-way merge is deferred).
///
/// The message clause and the route are the **caller's** ([`ConflictBlock`], M47 inc-2 /
/// T4) — the classifier composes only the invariant `` conflict on `<path>`: `` frame and
/// the location. What the mover is, and how to get out of it, differ per caller and are
/// unknowable here: the task-scope sweep names the real task id, the milestone-record
/// preflight names the record (there is no task at that door at all). The route the caller
/// hands in is verbatim, so a mechanical one has already passed the checked
/// [`crate::finding::Route::mechanical`] constructor (`surface-contract.md` → The route
/// fence) at its producer.
fn conflict_block_finding(path: &str, conflict: &ConflictBlock) -> Finding {
    let (detail, route) = conflict.presentation(path);
    Finding::graded(
        Severity::Blocking,
        "reconciliation.conflict-block",
        format!("conflict on `{path}`: {detail}"),
        Some(Location::addressed(path, 1, 1)),
        Some(route.clone()),
    )
}

/// The informational baseline-adopt finding (`reconciliation.md` → Baseline
/// adoption: "baseline adopted: `<doc>`"). Advisory — first encounter is the normal
/// case, not a problem to repair. Carries an **informational route** (the advisory-route
/// floor — every finding routes, never `null`; the no-op is an explicit route value,
/// not an absence).
fn baseline_adopt_finding(path: &str) -> Finding {
    Finding::graded(
        Severity::Advisory,
        "file-state.baseline-adopt",
        format!("baseline adopted: `{path}`"),
        Some(Location::addressed(path, 1, 1)),
        Some("no action needed — the baseline was adopted on first encounter".into()),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::index::{Edge, EdgeIndex};
    use crate::schema::Schema;

    const ADR_YAML: &[u8] = include_bytes!(pack_path!(dev, "schemas/adr.yaml"));

    fn adr_schema() -> Schema {
        crate::schema::load_schema_with_types(ADR_YAML, &crate::schema::dev_pack_field_types())
            .expect("adr.yaml loads")
    }

    /// The caller-supplied conflict presentation a task-scope caller hands the classifier
    /// (M47 inc-2 / T4) — a real task id, never a placeholder.
    fn test_conflict() -> ConflictBlock {
        ConflictBlock::task("drift-the-cache", None)
    }

    /// A committed ADR `B` (superseding nothing) — the recorded baseline before any
    /// out-of-band edit.
    const ADR_B_BASE: &str = "\
---
status: accepted
date: 2026-05-30
---

# Distributed session cache

## Context
A single node is a single point of failure.

## Options
Alternatives were weighed and rejected.

## Decision
Replicate the session cache across nodes.

## Consequences
Slightly higher write latency for resilience.
";

    /// The same ADR `B`, edited out-of-band to *add* a `supersedes` ref — a clean,
    /// schema-conformant edit (the absorb case).
    const ADR_B_EDITED_SUPERSEDES: &str = "\
---
status: accepted
date: 2026-05-30
supersedes: adr:single-node-cache
---

# Distributed session cache

## Context
A single node is a single point of failure.

## Options
Alternatives were weighed and rejected.

## Decision
Replicate the session cache across nodes.

## Consequences
Slightly higher write latency for resilience.
";

    /// ADR `B`, edited out-of-band to *malform* the `date` field value — a
    /// non-conformant edit (the conformance-block case).
    const ADR_B_EDITED_BAD_DATE: &str = "\
---
status: accepted
date: 2026/13/01
---

# Distributed session cache

## Context
A single node is a single point of failure.

## Options
Alternatives were weighed and rejected.

## Decision
Replicate the session cache across nodes.

## Consequences
Slightly higher write latency for resilience.
";

    const ADR_B_PATH: &str = "decisions/distributed-cache.md";
    const ADR_B_FROM: &str = "adr:distributed-cache";

    /// A referrer ADR that `supersedes` the (pre-rename) `adr:single-node-cache` —
    /// the pre-repoint committed baseline. Its H1 slugs to `cache-strategy`, distinct
    /// from the renamed doc's `distributed-cache`.
    const ADR_REFERRER_BASE: &str = "\
---
status: accepted
date: 2026-06-01
supersedes: adr:single-node-cache
---

# Cache strategy revision

## Context
The single-node cache decision needs revisiting.

## Options
Alternatives were weighed and rejected.

## Decision
Adopt the distributed cache instead.

## Consequences
Referrers must point at the new decision.
";

    /// [`absorbed_drift`] — the predicate the two **register-only** doors ask, over all
    /// three of its cells (M52 Increment 8 / T3). The positive cell's **route kind** is the
    /// load-bearing assertion here and nowhere else: the wire projects a route as its flat
    /// text, so no integration suite driving the binary can see that the registration's
    /// `Informational` was not silently filed as `Human` by a `String::into`.
    #[test]
    fn absorbed_drift_answers_only_over_a_baseline_the_bytes_do_not_match() {
        let mut record = FileStateRecord::new();
        let bytes = b"the bytes in hand";

        // UNKNOWN — no recorded baseline is not drift (that is `un-baselined`'s business).
        assert!(
            absorbed_drift(&record, ADR_B_PATH, bytes).is_none(),
            "an absent baseline is not an absorb"
        );

        // IN_SYNC — a matching baseline is nothing to report.
        record.record(ADR_B_PATH, hash_bytes(bytes));
        assert!(
            absorbed_drift(&record, ADR_B_PATH, bytes).is_none(),
            "a matching baseline is not an absorb"
        );

        // DRIFTED — the one cell that answers.
        record.record(ADR_B_PATH, hash_bytes(b"what the record still remembers"));
        let finding = absorbed_drift(&record, ADR_B_PATH, bytes).expect("a differing baseline");
        assert_eq!(finding.code, "file-state.absorbed");
        assert_eq!(finding.severity, Severity::Advisory);
        assert!(
            finding.message.contains(ADR_B_PATH),
            "the message names the absorbed path: {finding:?}"
        );
        let route = finding.route.as_ref().expect("the advisory-route floor");
        assert!(
            matches!(route.kind(), crate::finding::RouteKind::Informational),
            "the registration says `Informational`, so the constructed kind must be: {route:?}"
        );
        assert!(
            route.as_str().contains("file-state.hash-matches"),
            "the route names the finding this retires: {route:?}"
        );
    }

    /// The DRIFTED+UNTOUCHED clean-reparse branch: a committed ADR with a recorded
    /// baseline hash, edited on disk to add a `supersedes`, reconciles to **absorb** —
    /// the recorded hash advances to the new content's hash, the committed edge index
    /// gains the new forward edge, and the finding is an **advisory**
    /// `reconciliation.absorb` carrying the "external edit absorbed" message.
    #[test]
    fn oob_clean_edit_absorbs_and_updates_index() {
        let schema = adr_schema();
        let mut record = FileStateRecord::new();
        // Baseline: the committed bytes before the OOB edit.
        record.record(ADR_B_PATH, hash_bytes(ADR_B_BASE.as_bytes()));

        // The committed index has no edge for B yet (it superseded nothing).
        let mut index = EdgeIndex::default();

        let edited = ADR_B_EDITED_SUPERSEDES.as_bytes();
        let findings = reconcile_committed(
            &mut record,
            &mut index,
            &schema,
            ADR_B_PATH,
            ADR_B_FROM,
            edited,
            /* task_touched */ false,
            &|_| None,
            &test_conflict(),
            &crate::validate::AdoptionInputs::inert(),
        );

        // Exactly one advisory absorb finding carrying the message.
        assert_eq!(findings.len(), 1, "absorb emits exactly one finding");
        let f = &findings[0];
        assert_eq!(f.code, "reconciliation.absorb");
        assert_eq!(f.severity, Severity::Advisory, "absorb is informational");
        assert!(
            f.message.contains("external edit absorbed") && f.message.contains(ADR_B_PATH),
            "absorb names the absorbed doc: {f:?}"
        );

        // Re-hash: the recorded baseline advanced to the new content's hash.
        assert_eq!(
            record.get(ADR_B_PATH),
            Some(hash_bytes(edited).as_str()),
            "absorb re-hashes the recorded baseline to the edited content"
        );

        // The committed edge index gained the new forward edge (incremental update).
        assert_eq!(
            index.edges,
            vec![Edge {
                from: ADR_B_FROM.to_string(),
                relation: "supersedes".to_string(),
                to: "adr:single-node-cache".to_string(),
            }],
            "absorb incrementally adds the edited doc's new forward edge"
        );
    }

    /// The DRIFTED+UNTOUCHED parse/schema-fail branch: an edit malforming the `date`
    /// value **conformance-blocks** — a blocking `reconciliation.conformance-block`
    /// finding naming the file (and an expected-shape hint), the recorded hash is
    /// **not** advanced (no re-baseline on a block), and the edge index is **not**
    /// touched (no auto-repair).
    #[test]
    fn oob_nonconformant_edit_blocks() {
        let schema = adr_schema();
        let mut record = FileStateRecord::new();
        let baseline = hash_bytes(ADR_B_BASE.as_bytes());
        record.record(ADR_B_PATH, baseline.clone());

        let mut index = EdgeIndex::default();
        let edges_before = index.edges.clone();

        let edited = ADR_B_EDITED_BAD_DATE.as_bytes();
        let findings = reconcile_committed(
            &mut record,
            &mut index,
            &schema,
            ADR_B_PATH,
            ADR_B_FROM,
            edited,
            /* task_touched */ false,
            &|_| None,
            &test_conflict(),
            &crate::validate::AdoptionInputs::inert(),
        );

        assert!(
            findings
                .iter()
                .any(|f| f.code == "reconciliation.conformance-block"
                    && f.severity == Severity::Blocking),
            "a nonconformant edit conformance-blocks: {findings:?}"
        );
        let block = findings
            .iter()
            .find(|f| f.code == "reconciliation.conformance-block")
            .expect("a conformance-block finding");
        assert!(
            block.message.contains(ADR_B_PATH),
            "the block names the offending file: {block:?}"
        );
        assert!(
            block.route.is_some(),
            "the conformance-block carries a route"
        );

        // The recorded hash is NOT advanced — a block is not a re-baseline site.
        assert_eq!(
            record.get(ADR_B_PATH),
            Some(baseline.as_str()),
            "conformance-block does not advance the recorded hash (no auto-repair)"
        );
        // The edge index is untouched.
        assert_eq!(
            index.edges, edges_before,
            "conformance-block does not update the edge index"
        );
    }

    /// The DRIFTED+TOUCHED branch: the same committed doc both drifted on disk **and**
    /// touched by the CLI side **conflict-blocks** — a blocking
    /// `reconciliation.conflict-block` finding carrying the **caller-supplied** route, no
    /// silent merge, the recorded hash unadvanced and the edge index untouched.
    ///
    /// The task-scope caller's preset is asserted here; the round-trip of an arbitrary
    /// caller's presentation (the milestone-record door's shape) is
    /// [`conflict_presentation_is_the_callers`].
    #[test]
    fn oob_conflict_blocks() {
        let schema = adr_schema();
        let mut record = FileStateRecord::new();
        let baseline = hash_bytes(ADR_B_BASE.as_bytes());
        record.record(ADR_B_PATH, baseline.clone());

        let mut index = EdgeIndex::default();

        // The on-disk content drifted (a clean edit, even) — but the task also touched it.
        let edited = ADR_B_EDITED_SUPERSEDES.as_bytes();
        let findings = reconcile_committed(
            &mut record,
            &mut index,
            &schema,
            ADR_B_PATH,
            ADR_B_FROM,
            edited,
            /* task_touched */ true,
            &|_| None,
            &test_conflict(),
            &crate::validate::AdoptionInputs::inert(),
        );

        assert_eq!(findings.len(), 1, "conflict emits exactly one finding");
        let f = &findings[0];
        assert_eq!(f.code, "reconciliation.conflict-block");
        assert_eq!(f.severity, Severity::Blocking);
        let route = f.route.as_ref().expect("conflict carries a discard route");
        // The M43 ghost-verb repair (`DECISIONS.md` 2026-07-16 Settle) stands — the route
        // names only real verbs: the whole-task discard (honest that no per-write discard
        // exists) or the human on-disk revert. What M47 inc-2 / T4 changed is its SOURCE:
        // the task-scope caller supplies it, so the argv carries the REAL task id and the
        // `<task-id>` placeholder the classifier used to mint is gone.
        assert_eq!(
            route.as_str(),
            "`jigc task discard drift-the-cache --force` to drop this task's staged writes \
             (discard retires the whole task — no per-doc discard exists), or revert the external \
             edit on disk to keep them — the damage was made out-of-band, so it is repaired \
             where it happened",
            "the task-scope conflict route names real verbs AND the real task id, and the \
             revert it orders carries the sanction that makes it legal"
        );
        assert!(
            !route.as_str().contains("<task-id>"),
            "no unsubstituted placeholder survives on a blocking finding's route"
        );
        assert!(
            matches!(route.kind(), crate::finding::RouteKind::Mechanical { .. }),
            "the jigc span rides the checked mechanical constructor"
        );
        // The negative control for the migration exit (M46 inc-5 / T2): this caller declared
        // no migration source, so the `jigc unmanage` clause is not on offer here. A route
        // that dropped a baseline guard on a plain task's conflict would be a data-loss
        // affordance handed to a state that does not need it.
        assert!(
            !route.as_str().contains("unmanage"),
            "a non-migration task's conflict route must not offer the baseline drop: {route:?}"
        );

        // No silent merge: neither the recorded hash nor the edge index moved.
        assert_eq!(
            record.get(ADR_B_PATH),
            Some(baseline.as_str()),
            "conflict-block does not advance the recorded hash (no silent merge)"
        );
        assert!(
            index.edges.is_empty(),
            "conflict-block does not update the edge index"
        );
    }

    /// The conflict presentation **round-trips from the caller** (M47 inc-2 / T4): the
    /// classifier composes only the invariant `` conflict on `<path>`: `` frame + the
    /// location, and adds nothing of its own to the message clause or the route.
    ///
    /// Driven with the shape the **milestone-record** door supplies — a record-naming clause
    /// and a `Human` route (no task exists at that door, so no `jigc task discard` and no
    /// `<task-id>` placeholder may appear). This is the seam that made the reported defect
    /// possible: with the presentation hard-coded, that caller could not say anything true.
    #[test]
    fn conflict_presentation_is_the_callers() {
        let schema = adr_schema();
        let mut record = FileStateRecord::new();
        record.record(ADR_B_PATH, hash_bytes(ADR_B_BASE.as_bytes()));
        let mut index = EdgeIndex::default();

        let detail = "the milestone record is machine-maintained and was edited out of band";
        let route_text = "restore the record to what jigc last wrote, then re-run";
        let findings = reconcile_committed(
            &mut record,
            &mut index,
            &schema,
            ADR_B_PATH,
            ADR_B_FROM,
            ADR_B_EDITED_SUPERSEDES.as_bytes(),
            /* task_touched */ true,
            &|_| None,
            &ConflictBlock::new(detail, crate::finding::Route::human(route_text)),
            &crate::validate::AdoptionInputs::inert(),
        );

        assert_eq!(findings.len(), 1, "conflict emits exactly one finding");
        let f = &findings[0];
        assert_eq!(
            f.message,
            format!("conflict on `{ADR_B_PATH}`: {detail}"),
            "the message is the invariant frame + the caller's clause, verbatim"
        );
        let route = f.route.as_ref().expect("the caller's route rides through");
        assert_eq!(route.as_str(), route_text, "the route round-trips verbatim");
        assert!(
            matches!(route.kind(), crate::finding::RouteKind::Human),
            "the caller's route KIND survives too (a human revert is not mechanical)"
        );
        assert!(
            !f.message.contains("this task's staged writes")
                && !route.as_str().contains("jigc task discard"),
            "the classifier contributes no task language of its own: {f:?}"
        );
    }

    /// **L1 pull absorption, (i)** (M55 Increment 4, P1): a committed doc drifted from its
    /// recorded baseline **and** touched by the task, whose on-disk bytes equal its blob at the
    /// task's base pin, was moved by a pull *before* the task began — so the touched arm runs
    /// the untouched arm's whole absorb body, never only the re-hash: the conformance gate
    /// passes, the record advances, the committed index gains the doc's edges, and the one
    /// finding is the advisory `reconciliation.absorb`. The seam is asked about exactly the
    /// drifted path.
    #[test]
    fn a_touched_doc_whose_drift_equals_the_pin_absorbs() {
        let schema = adr_schema();
        let mut record = FileStateRecord::new();
        record.record(ADR_B_PATH, hash_bytes(ADR_B_BASE.as_bytes()));
        let mut index = EdgeIndex::default();
        let asked = std::cell::RefCell::new(Vec::new());

        let pulled = ADR_B_EDITED_SUPERSEDES.as_bytes();
        let findings = reconcile_committed(
            &mut record,
            &mut index,
            &schema,
            ADR_B_PATH,
            ADR_B_FROM,
            pulled,
            /* task_touched */ true,
            &|path: &str| {
                asked.borrow_mut().push(path.to_string());
                Some(pulled.to_vec())
            },
            &test_conflict(),
            &crate::validate::AdoptionInputs::inert(),
        );

        assert_eq!(
            findings,
            vec![absorb_finding(ADR_B_PATH)],
            "a pulled edit at the pin is absorbed, advisory, with no conflict-block"
        );
        assert_eq!(findings[0].severity, Severity::Advisory);
        assert_eq!(
            record.get(ADR_B_PATH),
            Some(hash_bytes(pulled).as_str()),
            "the absorb re-hashes the recorded baseline to the pulled bytes"
        );
        assert_eq!(
            index.edges,
            vec![Edge {
                from: ADR_B_FROM.to_string(),
                relation: "supersedes".to_string(),
                to: "adr:single-node-cache".to_string(),
            }],
            "the absorb folds the pulled doc's edges into the committed index"
        );
        assert_eq!(
            asked.into_inner(),
            vec![ADR_B_PATH.to_string()],
            "the seam is asked about the drifted path, once"
        );
    }

    /// **L1 pull absorption, (ii)** (M55 Increment 4, P1): bytes that equal the pin but do
    /// **not** conform are never baselined — the arm returns the **caller's** conflict-block,
    /// byte-identical to what it returned before the pin existed, never a conformance-block.
    /// Driven with a migration task's `ConflictBlock` keyed on this very path, so the M46
    /// path-keyed third exit (`jigc unmanage <source>`) is the route that survives: a break
    /// committed before `jigc migrate` equals the pin, and grading it a conformance-block would
    /// silently retire that exit.
    #[test]
    fn a_touched_doc_at_the_pin_that_does_not_conform_keeps_the_callers_conflict_block() {
        let schema = adr_schema();
        let mut record = FileStateRecord::new();
        let baseline = hash_bytes(ADR_B_BASE.as_bytes());
        record.record(ADR_B_PATH, baseline.clone());
        let mut index = EdgeIndex::default();
        let conflict = ConflictBlock::task("migrate-adr-distributed-cache", Some(ADR_B_PATH));

        let broken = ADR_B_EDITED_BAD_DATE.as_bytes();
        let findings = reconcile_committed(
            &mut record,
            &mut index,
            &schema,
            ADR_B_PATH,
            ADR_B_FROM,
            broken,
            /* task_touched */ true,
            &|_| Some(broken.to_vec()),
            &conflict,
            &crate::validate::AdoptionInputs::inert(),
        );

        assert_eq!(
            findings,
            vec![conflict_block_finding(ADR_B_PATH, &conflict)],
            "a non-conformant pulled edit keeps the caller's conflict-block, unchanged"
        );
        let route = findings[0].route.as_ref().expect("the conflict routes");
        assert!(
            route
                .as_str()
                .starts_with(&format!("`jigc unmanage {ADR_B_PATH}`")),
            "the migration source's path-keyed exit survives: {route:?}"
        );
        assert_eq!(
            record.get(ADR_B_PATH),
            Some(baseline.as_str()),
            "a non-conformant edit is never baselined"
        );
        assert!(index.edges.is_empty(), "nor is it folded into the index");
    }

    /// **L1 pull absorption, (iii)** (M55 Increment 4): bytes that differ from the pin's blob
    /// moved **during** the task — the pin predates them — so the touched arm conflict-blocks
    /// exactly as before, the record and index untouched.
    #[test]
    fn a_touched_doc_whose_drift_differs_from_the_pin_conflict_blocks() {
        let schema = adr_schema();
        let mut record = FileStateRecord::new();
        let baseline = hash_bytes(ADR_B_BASE.as_bytes());
        record.record(ADR_B_PATH, baseline.clone());
        let mut index = EdgeIndex::default();

        let findings = reconcile_committed(
            &mut record,
            &mut index,
            &schema,
            ADR_B_PATH,
            ADR_B_FROM,
            ADR_B_EDITED_SUPERSEDES.as_bytes(),
            /* task_touched */ true,
            &|_| Some(ADR_B_BASE.as_bytes().to_vec()),
            &test_conflict(),
            &crate::validate::AdoptionInputs::inert(),
        );

        assert_eq!(
            findings,
            vec![conflict_block_finding(ADR_B_PATH, &test_conflict())],
            "an edit made after the pin conflict-blocks"
        );
        assert_eq!(record.get(ADR_B_PATH), Some(baseline.as_str()));
        assert!(index.edges.is_empty());
    }

    /// **L1 pull absorption, (iv)** (M55 Increment 4, P2/P3): a `None` lookup — no pin, an
    /// absent blob, a git failure, the record door — leaves **every** arm's verdict as it was,
    /// and only a **touched** path ever consults the seam — drifted (the L1 arm) or with no
    /// record (the base-pin backstop, `(R3, F7)`) — so a clean sweep shells out zero times.
    #[test]
    fn a_none_lookup_keeps_every_arm_and_only_a_touched_path_asks() {
        let schema = adr_schema();
        let base = ADR_B_BASE.as_bytes();
        let edited = ADR_B_EDITED_SUPERSEDES.as_bytes();
        let broken = ADR_B_EDITED_BAD_DATE.as_bytes();
        let asked = std::cell::Cell::new(0usize);
        let none = |_: &str| {
            asked.set(asked.get() + 1);
            None
        };
        // (recorded baseline, on-disk bytes, touched) → the codes today's classifier emits,
        // and how many times the seam is asked.
        type Cell<'a> = (Option<&'a [u8]>, &'a [u8], bool, &'a [&'a str], usize);
        let cells: [Cell<'_>; 7] = [
            (None, base, false, &["file-state.baseline-adopt"], 0),
            (None, base, true, &["file-state.baseline-adopt"], 1),
            (Some(base), base, true, &[], 0),
            (Some(base), edited, false, &["reconciliation.absorb"], 0),
            (
                Some(base),
                broken,
                false,
                &["reconciliation.conformance-block"],
                0,
            ),
            (
                Some(base),
                edited,
                true,
                &["reconciliation.conflict-block"],
                1,
            ),
            (
                Some(base),
                broken,
                true,
                &["reconciliation.conflict-block"],
                1,
            ),
        ];
        for (recorded, bytes, touched, codes, asks) in cells {
            asked.set(0);
            let mut record = FileStateRecord::new();
            if let Some(recorded) = recorded {
                record.record(ADR_B_PATH, hash_bytes(recorded));
            }
            let mut index = EdgeIndex::default();
            let findings = reconcile_committed(
                &mut record,
                &mut index,
                &schema,
                ADR_B_PATH,
                ADR_B_FROM,
                bytes,
                touched,
                &none,
                &test_conflict(),
                &crate::validate::AdoptionInputs::inert(),
            );
            let got: Vec<&str> = findings.iter().map(|f| f.code.as_str()).collect();
            assert_eq!(got, codes, "touched={touched}: {findings:?}");
            assert_eq!(asked.get(), asks, "touched={touched}: the seam's asks");
            if touched && recorded.is_some() && bytes != base {
                assert_eq!(
                    findings,
                    vec![conflict_block_finding(ADR_B_PATH, &test_conflict())],
                    "a `None` lookup keeps the conflict-block byte-identically"
                );
                assert_eq!(record.get(ADR_B_PATH), Some(hash_bytes(base).as_str()));
            }
        }
    }

    /// The **migration-source exit is scoped to the one path it is true of** (M46 inc-5 / T2).
    ///
    /// A migration task's conflict on **its own recorded source** is the one conflict whose
    /// general route cannot be followed: discarding retires the migration, and the alternative
    /// it names — reverting the external edit on disk — is the act the adapter's read rule
    /// forbids, over a file that was *already* hand-broken out of band. So that path, and only
    /// that path, is offered the baseline drop.
    ///
    /// This is the scope half: the **same** `ConflictBlock`, asked about a *different* managed
    /// doc the same migration task also touched, answers the general presentation. A route that
    /// offered `jigc unmanage` for every conflicting path would hand a data-loss affordance to
    /// docs the migration is not replacing (the widen-a-guard rule,
    /// `surface-contract.md` → A route offered on a wider domain must be gated on that domain).
    #[test]
    fn the_migration_source_exit_is_keyed_on_the_source_path() {
        let source = ADR_B_PATH;
        let conflict = ConflictBlock::task("migrate-adr-decisions-cache", Some(source));

        let on_source = conflict_block_finding(source, &conflict);
        let route = on_source.route.as_ref().expect("the source arm routes");
        assert_eq!(
            route.as_str(),
            format!(
                "`jigc unmanage {source}` to drop the stale baseline on that path, then run \
                 this finalize again — the guard is dropped for that path only and the bytes \
                 stay on disk; they are not merged, this task's staged rewrite replaces them, \
                 and that rewrite was authored against the source as this task recorded it at \
                 mint, so an edit made to the file since is replaced without appearing in the \
                 `--approve` fidelity diff (which renders the recorded source, not what is on \
                 disk now). To keep the file as it stands, \
                 `jigc task discard migrate-adr-decisions-cache --force` retires the \
                 migration instead and leaves it untouched"
            ),
            "the source arm routes at the baseline drop, with the path substituted, the cost \
             stated, and the exit that keeps the on-disk bytes still named"
        );
        assert!(
            matches!(route.kind(), crate::finding::RouteKind::Mechanical { .. }),
            "the single-argv exit rides the checked mechanical constructor"
        );
        assert!(
            !route.as_str().contains('<'),
            "no unsubstituted placeholder survives on a blocking finding's route: {route:?}"
        );

        // The omitting context: another doc the same migration task touched. The exit is not
        // on offer there — that path is not what the task is replacing.
        let elsewhere = conflict_block_finding("decisions/unrelated.md", &conflict);
        let general = elsewhere.route.as_ref().expect("the general arm routes");
        assert!(
            !general.as_str().contains("unmanage"),
            "a conflict away from the migration source keeps the general route: {general:?}"
        );
        assert_eq!(
            general.as_str(),
            conflict_block_finding(
                "decisions/unrelated.md",
                &test_conflict_for("migrate-adr-decisions-cache")
            )
            .route
            .expect("the source-less caller routes")
            .as_str(),
            "and it is byte-identical to the route a source-less caller would have supplied"
        );
    }

    /// **The source arm's argv is emitted bytes, so the source is quoted** (M51 completion
    /// audit). `ConflictBlock::task` is built **eagerly for every migration task**, on the
    /// finalize path, before anything knows whether a conflict will be raised at all —
    /// driven at `c2faae6b` a migration of `my notes.md` panicked `jigc task finalize
    /// --approve` at exit 101 on the route fence's token check, the crash arriving after the
    /// authoring and at the one door that commits. In release the fence is compiled out and
    /// the same argv would have printed `jigc unmanage my notes.md`, dropping the baseline
    /// of a path that is not the one named.
    ///
    /// The quoting is at the **producer**, not at the fence: the fence is debug-posture and
    /// a release binary never consults it, so a fence firing is a report that the producer
    /// is wrong — never the fix.
    #[test]
    fn the_migration_source_exit_quotes_a_source_a_shell_would_re_lex() {
        let source = "my notes.md";
        let conflict = ConflictBlock::task("migrate-adr-my-notes", Some(source));
        let finding = conflict_block_finding(source, &conflict);
        let route = finding.route.as_ref().expect("the source arm routes");

        assert!(
            route.as_str().starts_with("`jigc unmanage 'my notes.md'`"),
            "the operand is the bytes a shell re-lexes as the path: {route:?}"
        );
        assert!(
            crate::finding::command_spans_are_shell_safe(route.as_str()),
            "and every command span in it is runnable: {route:?}"
        );
    }

    /// A task-scope conflict presentation for `task_id` with **no** migration source — the
    /// source-less caller the scope assertion above compares against.
    fn test_conflict_for(task_id: &str) -> ConflictBlock {
        ConflictBlock::task(task_id, None)
    }

    /// The **at-version advisory and its blocking twin read one shared sanction** (M48 Inc 4 /
    /// T2). The two producers sit side by side and once carried the clause as **two inline
    /// literals** — a drift the acceptance suite could not catch, because a suite that lifts one
    /// producer's bytes and compares them to the other's stays green only while both come from
    /// the same place. Pinned here at the seam: same route, character for character, and no
    /// migration verb over a doc that is already at the current schema-version.
    #[test]
    fn the_at_version_advisory_and_its_blocking_twin_share_one_sanction() {
        let path = "docs/decisions/cache-sessions-in-memory.md";
        let at_version = conformance_advisory_finding(
            path,
            None,
            "---\nschema-version: 2\n---\n\n# Cache sessions in memory\n",
            Some(2),
        );
        let blocking = conformance_block_finding(path, None);

        assert_eq!(
            at_version.route, blocking.route,
            "the at-version advisory serves the blocking twin's hand-repair sanction from ONE \
             source — not a second copy of the same words"
        );
        let route = at_version.route.as_deref().expect("the advisory routes");
        assert!(
            !route.contains("migrate"),
            "and a doc at the current schema-version is never told to migrate itself: {route}"
        );
        assert_eq!(
            at_version.severity,
            Severity::Advisory,
            "the un-baselined arm stays advisory — only the route moves"
        );
    }

    /// The G4 baseline-adopt gate (M21; `design/project-setup.md` → Flow 2 hardening →
    /// G4 conformance gate): a foreign **non-conformant** `.md` with **no** recorded
    /// hash (the `UNKNOWN` arm) is gated by `parse_sections` + `schema_conformance`
    /// **before** recording — it routes as **exactly one advisory**
    /// `reconciliation.conformance-block` (route `Some`), the record is **not** advanced
    /// (`get(path) == None`), and a second call **re-emits** the advisory (the
    /// routed-but-not-recorded re-fire). Advisory, never blocking.
    #[test]
    fn unknown_nonconformant_routes_advisory_and_does_not_baseline() {
        let schema = adr_schema();
        let mut record = FileStateRecord::new();
        let mut index = EdgeIndex::default();

        // A freeform notes file squatting in `decisions/` — no recorded hash (UNKNOWN),
        // and not a conformant ADR (missing the required header + sections).
        let foreign = b"# notes\n\nrandom thoughts, not an ADR\n";
        let path = "decisions/notes.md";
        let from = "adr:notes";

        let findings = reconcile_committed(
            &mut record,
            &mut index,
            &schema,
            path,
            from,
            foreign,
            false,
            &|_| None,
            &test_conflict(),
            &crate::validate::AdoptionInputs::inert(),
        );

        assert_eq!(
            findings.len(),
            1,
            "the non-conformant UNKNOWN doc routes exactly one finding: {findings:?}"
        );
        let f = &findings[0];
        assert_eq!(f.code, "reconciliation.conformance-block");
        assert_eq!(
            f.severity,
            Severity::Advisory,
            "the G4 gate routes advisory, never blocking: {f:?}"
        );
        assert!(
            f.route.is_some(),
            "the advisory names the corrective verb (route Some): {f:?}"
        );
        assert!(
            f.message.contains(path),
            "the advisory names the squatting file: {f:?}"
        );

        // The record is NOT advanced — a routed-but-not-recorded outcome.
        assert_eq!(
            record.get(path),
            None,
            "the non-conformant doc is not baseline-adopted (record not advanced)"
        );
        // The edge index is untouched (no absorb of a non-conformant doc).
        assert!(
            index.edges.is_empty(),
            "no edges absorbed from a routed doc"
        );

        // Re-fire: a second call (still UNKNOWN, since the first did not record) emits
        // the same advisory again — the recurrence the human resolves by ingest/move.
        let again = reconcile_committed(
            &mut record,
            &mut index,
            &schema,
            path,
            from,
            foreign,
            false,
            &|_| None,
            &test_conflict(),
            &crate::validate::AdoptionInputs::inert(),
        );
        assert_eq!(
            again.len(),
            1,
            "the advisory re-fires on a second sweep (routed-but-not-recorded): {again:?}"
        );
        assert_eq!(again[0].code, "reconciliation.conformance-block");
        assert_eq!(again[0].severity, Severity::Advisory);
        assert_eq!(
            record.get(path),
            None,
            "the record still is not advanced after the re-fire"
        );
    }

    /// The G4 gate preserves the legitimate `UNKNOWN` case: a **conformant**
    /// fresh-checkout doc with no recorded hash still **baseline-adopts** cleanly —
    /// exactly one advisory `file-state.baseline-adopt` and the record **is** advanced
    /// (the M20 clean-store guarantee holds; `design/project-setup.md` → Flow 2
    /// hardening → G4 conformance gate).
    #[test]
    fn unknown_conformant_doc_still_baselines() {
        let schema = adr_schema();
        let mut record = FileStateRecord::new();
        let mut index = EdgeIndex::default();

        let findings = reconcile_committed(
            &mut record,
            &mut index,
            &schema,
            ADR_B_PATH,
            ADR_B_FROM,
            ADR_B_BASE.as_bytes(),
            false,
            &|_| None,
            &test_conflict(),
            &crate::validate::AdoptionInputs::inert(),
        );

        assert_eq!(
            findings.len(),
            1,
            "the conformant UNKNOWN doc emits exactly one finding: {findings:?}"
        );
        let f = &findings[0];
        assert_eq!(f.code, "file-state.baseline-adopt");
        assert_eq!(f.severity, Severity::Advisory);
        assert!(
            f.route
                .as_deref()
                .is_some_and(|r| r.contains("no action needed")),
            "baseline adoption carries an informational route (the advisory-route floor, \
             never null); a first encounter is not a repair: {f:?}"
        );

        // The record IS advanced to the conformant doc's hash.
        assert_eq!(
            record.get(ADR_B_PATH),
            Some(hash_bytes(ADR_B_BASE.as_bytes()).as_str()),
            "a conformant fresh-checkout doc still baselines (record advances)"
        );
    }

    /// **Strong signal** — a tracked managed-doc path is missing **and** an untracked
    /// path carries the **same recorded content hash**: a suspected `git mv`. The
    /// detector emits exactly one blocking `reconciliation.rename` finding naming both
    /// the missing tracked path and the suspect path, routed to a `git mv … revert`,
    /// and rewrites **no** referrer refs / edges (`reconciliation.md` → Rename
    /// detection → strong signal).
    #[test]
    fn rename_strong_signal_routes_to_git_mv_revert() {
        const TRACKED: &str = "decisions/rate-limit.md";
        const MOVED: &str = "decisions/gateway-rate-limit.md";
        const FROM: &str = "adr:rate-limit";

        // The recorded baseline hash for the (now-missing) tracked doc.
        let recorded = hash_bytes(ADR_B_BASE.as_bytes());

        // The untracked candidates on disk: the moved file carries the *same* content,
        // so its raw-byte hash matches the recorded baseline (a suspected `git mv`).
        let untracked: Vec<(&str, String)> = vec![(MOVED, hash_bytes(ADR_B_BASE.as_bytes()))];

        let findings = detect_rename(
            TRACKED,
            FROM,
            &recorded,
            &untracked,
            &|_| true,
            &|_| true,
            &LiveRecord::none(),
            std::path::Path::new("/repo"),
        );

        assert_eq!(findings.len(), 1, "strong signal emits exactly one finding");
        let f = &findings[0];
        assert_eq!(f.code, "reconciliation.rename");
        assert_eq!(f.severity, Severity::Blocking);
        assert!(
            f.message.contains(TRACKED) && f.message.contains(MOVED),
            "the strong-signal finding names both the missing tracked path and the suspect path: {f:?}"
        );
        let route = f
            .route
            .as_deref()
            .expect("strong signal carries a revert route");
        assert!(
            route.contains("git -C /repo mv") && route.contains(MOVED) && route.contains(TRACKED),
            "the route directs a `git mv … revert` of the moved file back to the tracked \
             path, aimed at the checkout it runs in (M53 — the cwd census, C1-09; `git mv` \
             refuses a pathspec, so `-C` is the only spelling that runs from anywhere): \
             {route:?}"
        );
        assert!(
            route.contains("jigc rename") && !route.contains("jigc doc rename"),
            "the route advertises the now-shipped top-level `jigc rename` adopt op (not the stale `jigc doc rename` placeholder): {route:?}"
        );
    }

    /// **Weak signal** — a tracked managed-doc path is missing and **no** untracked
    /// path carries the recorded content hash: the file is simply gone (deleted). The
    /// detector emits exactly one blocking `reconciliation.rename` finding routed to
    /// **restore**, naming the missing path and rewriting **no** referrer refs / edges
    /// (`reconciliation.md` → Rename detection → weak signal).
    #[test]
    fn rename_weak_signal_routes_to_restore() {
        const TRACKED: &str = "decisions/rate-limit.md";
        const FROM: &str = "adr:rate-limit";

        let recorded = hash_bytes(ADR_B_BASE.as_bytes());

        // Untracked candidates exist, but none matches the recorded hash (different
        // content) — so there is no rename suspect, only a missing file.
        let untracked: Vec<(&str, String)> =
            vec![("decisions/unrelated.md", hash_bytes(b"some other body\n"))];

        let findings = detect_rename(
            TRACKED,
            FROM,
            &recorded,
            &untracked,
            &|_| true,
            &|_| true,
            &LiveRecord::none(),
            std::path::Path::new("/repo"),
        );

        assert_eq!(findings.len(), 1, "weak signal emits exactly one finding");
        let f = &findings[0];
        assert_eq!(f.code, "reconciliation.rename");
        assert_eq!(f.severity, Severity::Blocking);
        assert!(
            f.message.contains(TRACKED) && f.message.contains("missing"),
            "the weak-signal finding names the missing tracked path: {f:?}"
        );
        let route = f
            .route
            .as_deref()
            .expect("weak signal carries a restore route");
        assert!(
            route.contains("restore"),
            "the weak-signal route directs a restore of the missing file: {route:?}"
        );
        assert!(
            route.contains("jigc unmanage") && !route.contains("jigc delete"),
            "the weak-signal route names the shipped top-level `jigc unmanage` op (not the nonexistent `jigc delete`): {route:?}"
        );
    }

    /// **Weak signal, history-less** — the same missing tracked path with **no** content-
    /// matching suspect, but the `history` predicate reports no HEAD history for it: a
    /// dangling baseline the checkout moved out from under the gitignored cache (M45,
    /// Decision 7). The detector downgrades to an **advisory** `reconciliation.rename`,
    /// never the blocking weak finding — so a moved checkout no longer wedges the gate.
    ///
    /// With a branch still carrying the path (`other_refs` true), the route names the
    /// branch switch and offers switching back, as an **informational** route with no
    /// backticked command and no `jigc unmanage` (M55 Increment 5 / T1, R5): one producer,
    /// one route at task and store scope, and never an index drop.
    #[test]
    fn rename_weak_signal_history_less_downgrades_to_advisory() {
        const TRACKED: &str = "decisions/rate-limit.md";
        const FROM: &str = "adr:rate-limit";

        let recorded = hash_bytes(ADR_B_BASE.as_bytes());
        let untracked: Vec<(&str, String)> =
            vec![("decisions/unrelated.md", hash_bytes(b"some other body\n"))];

        // `history` empty for the path: nothing was deleted, the baseline is stale.
        let findings = detect_rename(
            TRACKED,
            FROM,
            &recorded,
            &untracked,
            &|_| false,
            &|_| true,
            &LiveRecord::none(),
            std::path::Path::new("/repo"),
        );

        assert_eq!(
            findings.len(),
            1,
            "history-less weak signal emits one finding"
        );
        let f = &findings[0];
        assert_eq!(f.code, "reconciliation.rename");
        assert_eq!(
            f.severity,
            Severity::Advisory,
            "a history-less dangling baseline downgrades to advisory: {f:?}"
        );
        let route = f
            .route
            .as_ref()
            .expect("the advisory-route floor: the dangling baseline carries a route");
        assert!(
            matches!(route.kind(), crate::finding::RouteKind::Informational),
            "nothing on this checkout is owed, so the route is informational: {route:?}"
        );
        assert_eq!(
            route.as_str(),
            format!(
                "nothing on this checkout needs to change — a branch switch left this \
                 baseline behind, and {TRACKED} lives on a branch this checkout does not \
                 carry: switch back to that branch to work on it again"
            ),
            "the route names the branch switch and offers switching back"
        );
        assert!(
            !route.contains("jigc unmanage") && !route.contains('`'),
            "the route never offers an index drop and carries no command span: {route:?}"
        );
    }

    /// (M55 completion triage, CR2) **No branch carries the path — the `unmanage` route.**
    /// The same history-less cell, but `other_refs` finds no branch, local or
    /// remote-tracking, carrying it (a hard reset or rebase past the creating commit, or a
    /// doc ingested and deleted before it was ever committed). The severity, code and
    /// location are the branch-switch arm's, so the `(code, target)` key does not move; the
    /// message claims no branch switch, and the route offers `jigc unmanage <path>`, the one
    /// way to clear a baseline that outlived its doc.
    #[test]
    fn rename_weak_signal_history_less_on_no_branch_routes_at_unmanage() {
        const TRACKED: &str = "decisions/rate-limit.md";
        const FROM: &str = "adr:rate-limit";

        let recorded = hash_bytes(ADR_B_BASE.as_bytes());
        let findings = detect_rename(
            TRACKED,
            FROM,
            &recorded,
            &[],
            &|_| false,
            &|_| false,
            &LiveRecord::none(),
            std::path::Path::new("/repo"),
        );

        assert_eq!(findings.len(), 1, "one finding for the one baseline");
        let f = &findings[0];
        let switch = rename_dangling_baseline_finding(TRACKED, FROM);
        assert_eq!(
            (&f.code, f.severity, &f.location),
            (&switch.code, switch.severity, &switch.location),
            "the orphaned arm keeps the branch-switch arm's key and advisory severity: {f:?}"
        );
        assert!(
            !f.message.contains("branch switch") && !f.message.contains("checkout moved"),
            "the message claims no cause it did not observe: {f:?}"
        );
        let route = f.route.as_ref().expect("the advisory carries a route");
        assert_eq!(
            route.as_str(),
            format!(
                "no branch, local or remote-tracking, can bring {TRACKED} back — a hard reset \
                 or a rebase past its creating commit, or deleting it before it was ever \
                 committed, leaves this baseline behind: drop it with `jigc unmanage {TRACKED}`"
            ),
            "the route offers the index drop, cause-neutrally"
        );
        assert!(
            !route.contains("switch back"),
            "no branch carries the doc, so the route offers no switch back: {route:?}"
        );
    }

    /// (M55 completion triage, CR2) **`other_refs` is asked only in the history-less cell.**
    /// A genuine deletion (history present) and a strong `git mv` never consult it, so a
    /// gate over either pays no extra git read and its finding is unchanged.
    #[test]
    fn rename_other_refs_is_asked_only_when_history_is_empty() {
        const TRACKED: &str = "decisions/rate-limit.md";
        let recorded = hash_bytes(ADR_B_BASE.as_bytes());
        let strong: Vec<(&str, String)> = vec![("decisions/moved.md", recorded.clone())];
        for (what, untracked, history) in [
            ("weak, history present", Vec::new(), true),
            ("strong", strong.clone(), true),
            ("strong, history-less", strong, false),
        ] {
            let asked = std::cell::Cell::new(false);
            let other_refs = |_: &str| {
                asked.set(true);
                false
            };
            detect_rename(
                TRACKED,
                "adr:rate-limit",
                &recorded,
                &untracked,
                &|_| history,
                &other_refs,
                &LiveRecord::none(),
                std::path::Path::new("/repo"),
            );
            assert!(!asked.get(), "{what}: `other_refs` must not be asked");
        }
    }

    /// A committed store whose record baselines `decisions/gone.md` (absent on disk) at
    /// [`ADR_B_BASE`]'s hash, and — when `moved` — carries those same bytes at the untracked
    /// `decisions/moved.md`, the strong-signal `git mv` shape. The store twin's fixture.
    fn store_with_missing_baseline(tag: &str, moved: bool) -> (TempRoot, FileStateRecord) {
        let root = TempRoot::new(tag);
        let decisions = root.path().join("decisions");
        std::fs::create_dir_all(&decisions).expect("mk decisions/");
        if moved {
            std::fs::write(decisions.join("moved.md"), ADR_B_BASE).expect("write moved ADR");
        }
        let mut record = FileStateRecord::new();
        record.record("decisions/gone.md", hash_bytes(ADR_B_BASE.as_bytes()));
        (root, record)
    }

    fn adr_schemas() -> std::collections::BTreeMap<String, Schema> {
        std::collections::BTreeMap::from([("adr".to_string(), adr_schema())])
    }

    /// (M55 Increment 5 / T2, i) **The store twin grades by history, as the task gate does.**
    /// A recorded doc missing on disk, no content-matching candidate, and **no** history at
    /// `HEAD` (a branch switch left the baseline behind): [`detect_committed_store_renames`]
    /// emits the advisory dangling-baseline finding — the task scope's own producer, so the
    /// route is the branch switch's and the key is the same — and still reports the identity
    /// as missing, so its inbound edges are subtracted from `ref-resolves` exactly as before.
    #[test]
    fn store_twin_history_less_baseline_is_the_advisory_dangling_finding() {
        let (root, record) = store_with_missing_baseline("store-twin-history-less", false);
        let (findings, renamed) = detect_committed_store_renames(
            &record,
            &adr_schemas(),
            root.path(),
            &|_| false,
            &|_| true,
        );

        assert_eq!(
            findings.len(),
            1,
            "one finding for the one baseline: {findings:?}"
        );
        let f = &findings[0];
        assert_eq!(f.code, "reconciliation.rename");
        assert_eq!(
            f.severity,
            Severity::Advisory,
            "a history-less dangling baseline is advisory at store scope too: {f:?}"
        );
        assert_eq!(
            f,
            &rename_dangling_baseline_finding("decisions/gone.md", "adr:gone"),
            "one producer at both scopes: the store row is the task row, byte for byte"
        );
        assert!(
            !f.route
                .as_deref()
                .unwrap_or_default()
                .contains("jigc unmanage"),
            "the store row never offers an index drop: {f:?}"
        );
        assert_eq!(
            renamed,
            std::collections::BTreeSet::from(["adr:gone".to_string()]),
            "the identity is still reported missing, so `ref-resolves` subtracts its edges"
        );
    }

    /// (M55 Increment 5 / T2, ii) **A path with history keeps the blocking weak finding.**
    /// The same state with `HEAD` history for the path is a genuine deletion: the store
    /// twin's emission is unchanged from before the predicate was threaded.
    #[test]
    fn store_twin_history_present_baseline_keeps_the_blocking_weak_finding() {
        let (root, record) = store_with_missing_baseline("store-twin-history-present", false);
        let (findings, _) = detect_committed_store_renames(
            &record,
            &adr_schemas(),
            root.path(),
            &|_| true,
            &|_| true,
        );

        assert_eq!(
            findings,
            vec![rename_weak_finding("decisions/gone.md", "adr:gone")],
            "a genuine deletion keeps the blocking weak finding at store scope"
        );
        assert_eq!(findings[0].severity, Severity::Blocking);
    }

    /// (M55 completion triage, CR2) **The store twin routes a branchless baseline at
    /// `unmanage`, as the task gate does.** History-less and carried by no branch: the
    /// store row is the orphaned arm's producer, byte for byte, still advisory, and the
    /// identity is still reported missing.
    #[test]
    fn store_twin_history_less_baseline_on_no_branch_is_the_orphaned_finding() {
        let (root, record) = store_with_missing_baseline("store-twin-orphaned", false);
        let (findings, renamed) = detect_committed_store_renames(
            &record,
            &adr_schemas(),
            root.path(),
            &|_| false,
            &|_| false,
        );
        assert_eq!(
            findings,
            vec![rename_orphaned_baseline_finding(
                "decisions/gone.md",
                "adr:gone"
            )],
            "one producer at both scopes for the orphaned arm too"
        );
        assert_eq!(findings[0].severity, Severity::Advisory);
        assert_eq!(
            renamed,
            std::collections::BTreeSet::from(["adr:gone".to_string()]),
        );
    }

    /// (M55 Increment 5 / T2, iii) **The strong signal never consults history.** A
    /// content-preserving bare `git mv` is the same blocking strong finding whatever the
    /// predicate answers, and the predicate is not even asked.
    #[test]
    fn store_twin_strong_signal_is_unchanged_under_either_history() {
        let (root, record) = store_with_missing_baseline("store-twin-strong", true);
        for history in [false, true] {
            let asked = std::cell::Cell::new(false);
            let predicate = |_: &str| {
                asked.set(true);
                history
            };
            let (findings, _) = detect_committed_store_renames(
                &record,
                &adr_schemas(),
                root.path(),
                &predicate,
                &|_| true,
            );
            assert_eq!(
                findings,
                vec![rename_strong_finding(
                    "decisions/gone.md",
                    "adr:gone",
                    "decisions/moved.md",
                    root.path(),
                )],
                "history {history}: the strong `git mv` finding is unchanged"
            );
            assert!(
                !asked.get(),
                "history {history}: the strong arm never asks the history predicate"
            );
        }
    }

    /// The command-surface sweep [`reconcile_committed_store`] routes committed-store
    /// drift end-to-end: it walks `<repo_root>/decisions/*.md`, reconciles each against
    /// the recorded baseline, and aggregates the classifier's findings — a conformant
    /// OOB edit **absorbs** (advisory, record advances) while a nonconformant one
    /// **conformance-blocks** (record pinned). This is the contract `validate_task`
    /// relies on (`reconciliation.md` → Detection timing: the `task validate` full sweep).
    #[test]
    fn reconcile_committed_store_sweeps_and_routes_drift() {
        let schema = adr_schema();
        let mut schemas: std::collections::BTreeMap<String, Schema> =
            std::collections::BTreeMap::new();
        schemas.insert("adr".to_string(), schema);

        // A committed store with two ADRs at their canonical paths.
        let root = TempRoot::new("sweep");
        let decisions = root.path().join("decisions");
        std::fs::create_dir_all(&decisions).expect("mk decisions/");

        // `clean`: a conformant OOB edit (a `## Decision` prose change) → absorb.
        let clean_path = "decisions/clean.md";
        std::fs::write(
            decisions.join("clean.md"),
            ADR_B_EDITED_SUPERSEDES, // differs from ADR_B_BASE (adds a supersedes)
        )
        .expect("write clean ADR");
        // `broken`: a structural nonconformance (renamed heading) → conformance-block.
        let broken_path = "decisions/broken.md";
        let broken = ADR_B_BASE.replace("## Decision", "## Decisionz");
        std::fs::write(decisions.join("broken.md"), &broken).expect("write broken ADR");

        // Record both baselines as the pre-edit content (so both DRIFT).
        let mut record = FileStateRecord::new();
        record.record(clean_path, hash_bytes(ADR_B_BASE.as_bytes()));
        record.record(broken_path, hash_bytes(ADR_B_BASE.as_bytes()));

        let mut index = EdgeIndex::default();
        // No active task touches either doc (a separate empty task dir).
        let task = TempRoot::new("sweep-task");

        let findings = reconcile_committed_store(
            &mut record,
            &mut index,
            &schemas,
            root.path(),
            task.path(),
            &|_| true,
            &|_| true,
            &|_| None,
            &test_conflict(),
            &crate::validate::AdoptionInputs::inert(),
            &LiveRecord::none(),
        );

        // The clean ADR absorbed (advisory) and its baseline advanced + edge folded in.
        assert!(
            findings
                .iter()
                .any(|f| f.code == "reconciliation.absorb" && f.message.contains(clean_path)),
            "the conformant edit absorbs: {findings:?}"
        );
        assert_eq!(
            record.get(clean_path),
            Some(hash_bytes(ADR_B_EDITED_SUPERSEDES.as_bytes()).as_str()),
            "absorb advances the clean doc's recorded baseline"
        );
        assert!(
            index.edges.contains(&Edge {
                from: "adr:clean".to_string(),
                relation: "supersedes".to_string(),
                to: "adr:single-node-cache".to_string(),
            }),
            "absorb folds the clean doc's new forward edge into the index: {:?}",
            index.edges
        );

        // The broken ADR conformance-blocked and its baseline stayed pinned.
        let block = findings
            .iter()
            .find(|f| f.code == "reconciliation.conformance-block")
            .expect("the nonconformant edit conformance-blocks");
        assert_eq!(block.severity, Severity::Blocking);
        assert!(
            block.message.contains(broken_path),
            "the block names the offending file: {block:?}"
        );
        assert_eq!(
            record.get(broken_path),
            Some(hash_bytes(ADR_B_BASE.as_bytes()).as_str()),
            "conformance-block does not advance the recorded hash"
        );
    }

    /// [`reconcile_committed_store`] routes a recorded committed doc that has gone
    /// **missing** on disk to rename detection: a content-hash-matching untracked file
    /// is a strong-signal `git mv` suspect (`reconciliation.md` → Rename detection).
    #[test]
    fn reconcile_committed_store_routes_a_missing_recorded_doc_to_rename() {
        let schema = adr_schema();
        let mut schemas: std::collections::BTreeMap<String, Schema> =
            std::collections::BTreeMap::new();
        schemas.insert("adr".to_string(), schema);

        let root = TempRoot::new("rename-sweep");
        let decisions = root.path().join("decisions");
        std::fs::create_dir_all(&decisions).expect("mk decisions/");

        // The tracked path is recorded but absent on disk; a content-matching file
        // exists at a new (untracked) path — a suspected `git mv`.
        let tracked = "decisions/old-name.md";
        let moved_slug = "new-name";
        std::fs::write(decisions.join(format!("{moved_slug}.md")), ADR_B_BASE)
            .expect("write moved ADR");

        let mut record = FileStateRecord::new();
        record.record(tracked, hash_bytes(ADR_B_BASE.as_bytes()));

        let mut index = EdgeIndex::default();
        let task = TempRoot::new("rename-task");

        let findings = reconcile_committed_store(
            &mut record,
            &mut index,
            &schemas,
            root.path(),
            task.path(),
            &|_| true,
            &|_| true,
            &|_| None,
            &test_conflict(),
            &crate::validate::AdoptionInputs::inert(),
            &LiveRecord::none(),
        );

        let rename = findings
            .iter()
            .find(|f| f.code == "reconciliation.rename")
            .expect("the missing recorded doc routes to rename detection");
        assert_eq!(rename.severity, Severity::Blocking);
        assert!(
            rename.message.contains(tracked) && rename.message.contains("new-name"),
            "the strong-signal rename names both the missing tracked path and the suspect: {rename:?}"
        );
    }

    /// The 2026-06-12 rename amendment (`reconciliation.md` → Rename detection:
    /// "missing means absent from disk, not absent from the walk"): a baselined path
    /// under a persisted `location:` prefix but outside the non-recursive
    /// `<location>/*.md` walk — the promoted owner-artifact case,
    /// `completions/artifacts/<run>/x.md`, baselined by its finalize — is **not**
    /// missing while it exists on disk: zero `reconciliation.rename`, hash untouched.
    /// Genuinely deleted from disk, the weak-signal blocking rename fires.
    #[test]
    fn baselined_path_outside_walk_is_not_missing() {
        let mut schema = adr_schema();
        schema.location = Some("completions/".to_string());
        let mut schemas: std::collections::BTreeMap<String, Schema> =
            std::collections::BTreeMap::new();
        schemas.insert("completion-record".to_string(), schema);

        let root = TempRoot::new("outside-walk");
        let artifact_rel = "completions/artifacts/run-1/x.md";
        let artifact_abs = root.path().join(artifact_rel);
        std::fs::create_dir_all(artifact_abs.parent().expect("artifact has a parent"))
            .expect("mk completions/artifacts/run-1/");
        let artifact_bytes: &[u8] = b"the genuine audit transcript\n";
        std::fs::write(&artifact_abs, artifact_bytes).expect("write the owner-artifact");

        // The artifact was baselined at its finalize (git_commit_files re-hash).
        let mut record = FileStateRecord::new();
        let baseline = hash_bytes(artifact_bytes);
        record.record(artifact_rel, baseline.clone());

        let mut index = EdgeIndex::default();
        let task = TempRoot::new("outside-walk-task");

        // Present on disk but never walked → NOT missing: zero rename, hash untouched.
        let findings = reconcile_committed_store(
            &mut record,
            &mut index,
            &schemas,
            root.path(),
            task.path(),
            &|_| true,
            &|_| true,
            &|_| None,
            &test_conflict(),
            &crate::validate::AdoptionInputs::inert(),
            &LiveRecord::none(),
        );
        assert!(
            findings.iter().all(|f| f.code != "reconciliation.rename"),
            "a baselined path present on disk outside the walk is not missing: {findings:?}"
        );
        assert_eq!(
            record.get(artifact_rel),
            Some(baseline.as_str()),
            "the present-outside-the-walk path stays baselined as-is (hash untouched)"
        );

        // Genuinely deleted from disk → the weak-signal blocking rename fires.
        std::fs::remove_file(&artifact_abs).expect("delete the owner-artifact");
        let findings = reconcile_committed_store(
            &mut record,
            &mut index,
            &schemas,
            root.path(),
            task.path(),
            &|_| true,
            &|_| true,
            &|_| None,
            &test_conflict(),
            &crate::validate::AdoptionInputs::inert(),
            &LiveRecord::none(),
        );
        let rename = findings
            .iter()
            .find(|f| f.code == "reconciliation.rename")
            .expect("a genuinely deleted baselined path still routes to rename detection");
        assert_eq!(rename.severity, Severity::Blocking);
        assert!(
            rename.message.contains(artifact_rel) && rename.message.contains("missing"),
            "the weak-signal rename names the missing artifact path: {rename:?}"
        );
        assert!(
            rename
                .route
                .as_deref()
                .is_some_and(|r| r.contains("restore")),
            "no content-matching suspect exists, so the weak signal routes to restore: {rename:?}"
        );
    }

    /// **Post-commit crash-window self-heal** (M35; `reconciliation.md` → Rename
    /// detection, the transaction, self-healing; `write-commands.md` → `jigc rename`
    /// step 5). A `jigc rename` that committed the move — the renamed doc is tracked at
    /// its **new** path and every referrer is **already repointed** — but crashed
    /// *before* re-baselining `file-state` leaves the record still keyed on the **OLD**
    /// path. Driving that interrupted state through [`reconcile_committed_store`] must
    /// **self-heal**: re-key the record old→new and emit **no** `reconciliation.rename`
    /// finding. The committed tree is authoritative, so the verb's own interrupted
    /// landing is never routed to the weak-signal "restore the old file" (which would
    /// resurrect a deliberately-renamed-away doc). The heal is **idempotent** — a second
    /// sweep is a clean no-op. RED obligation: the recovery is *driven* through the real
    /// sweep, not asserted.
    #[test]
    fn rename_crash_window_self_heals_without_restore_finding() {
        let schema = adr_schema();
        let mut schemas: BTreeMap<String, Schema> = BTreeMap::new();
        schemas.insert("adr".to_string(), schema);

        let root = TempRoot::new("rename-crash");
        let decisions = root.path().join("decisions");
        std::fs::create_dir_all(&decisions).expect("mk decisions/");

        // The committed tree AFTER the rename landed: the renamed doc lives at its NEW
        // path with the rewritten H1 (content differs from the old recorded baseline, so
        // it is NOT a strong-signal `git mv` content match), and the referrer's
        // `supersedes` ref is already repointed to the new identity.
        let old_path = "decisions/single-node-cache.md";
        let new_path = "decisions/distributed-cache.md";
        let renamed_landing = ADR_B_BASE; // H1 "Distributed session cache" → distributed-cache
        std::fs::write(decisions.join("distributed-cache.md"), renamed_landing)
            .expect("write the renamed (landed) doc");

        let referrer_path = "decisions/cache-strategy.md";
        let referrer_repointed = ADR_REFERRER_BASE.replace(
            "supersedes: adr:single-node-cache",
            "supersedes: adr:distributed-cache",
        );
        std::fs::write(decisions.join("cache-strategy.md"), &referrer_repointed)
            .expect("write the repointed referrer");

        // file-state STILL keyed on the OLD state (the crash interrupted the re-baseline):
        // the OLD path is recorded (now absent on disk; its recorded hash is the
        // pre-rename content, which does NOT match the landing) and the referrer carries
        // its PRE-repoint hash (so it DRIFTs and absorbs in the walk).
        let mut record = FileStateRecord::new();
        record.record(
            old_path,
            hash_bytes(b"the pre-rename single-node-cache body\n"),
        );
        record.record(referrer_path, hash_bytes(ADR_REFERRER_BASE.as_bytes()));

        let mut index = EdgeIndex::default();
        let task = TempRoot::new("rename-crash-task");

        let findings = reconcile_committed_store(
            &mut record,
            &mut index,
            &schemas,
            root.path(),
            task.path(),
            &|_| true,
            &|_| true,
            &|_| None,
            &test_conflict(),
            &crate::validate::AdoptionInputs::inert(),
            &LiveRecord::none(),
        );

        // (i) NO rename finding — neither the weak-signal restore nor a strong block.
        assert!(
            findings.iter().all(|f| f.code != "reconciliation.rename"),
            "the crash-window landing self-heals; it never routes to rename/restore: {findings:?}"
        );
        // (ii) the record is re-keyed old→new: the stale OLD key is forgotten and the
        // NEW landing is baselined to its on-disk content.
        assert_eq!(
            record.get(old_path),
            None,
            "the stale old-path key is forgotten (re-keyed old→new)"
        );
        assert_eq!(
            record.get(new_path),
            Some(hash_bytes(renamed_landing.as_bytes()).as_str()),
            "the renamed doc's landing is baselined at its new path"
        );

        // (iii) idempotent: a second sweep is a clean no-op (no rename, record stable).
        let record_after_first = record.clone();
        let again = reconcile_committed_store(
            &mut record,
            &mut index,
            &schemas,
            root.path(),
            task.path(),
            &|_| true,
            &|_| true,
            &|_| None,
            &test_conflict(),
            &crate::validate::AdoptionInputs::inert(),
            &LiveRecord::none(),
        );
        assert!(
            again.iter().all(|f| f.code != "reconciliation.rename"),
            "the self-heal is idempotent — a second sweep emits no rename finding: {again:?}"
        );
        assert_eq!(
            record, record_after_first,
            "the second sweep does not further mutate the re-keyed record"
        );
    }

    /// The read-only file↔CLI-state **store twin** ([`detect_committed_store`])
    /// over a committed store carrying a baselined-and-drifted doc, an
    /// un-baselined doc, and an in-sync doc (`validation.md` → Completing the
    /// envelope → read-only file↔CLI-state at store scope):
    ///
    /// - the drifted doc → exactly one `file-state.hash-matches` finding whose
    ///   route is the **store-scope variant** (NOT `reconcile <path>`);
    /// - the un-baselined committed doc → exactly one distinct **advisory**
    ///   un-baselined finding (not drift, not silent-clean);
    /// - the in-sync doc → no finding;
    /// - the passed `record` is **byte-identical before and after** (the twin
    ///   takes it by `&` and opens no record write — mutation-free).
    #[test]
    fn detect_committed_store_reports_drift_and_unbaselined_without_absorb() {
        let schema = adr_schema();
        let mut schemas: std::collections::BTreeMap<String, Schema> =
            std::collections::BTreeMap::new();
        schemas.insert("adr".to_string(), schema);

        let root = TempRoot::new("detect-twin");
        let decisions = root.path().join("decisions");
        std::fs::create_dir_all(&decisions).expect("mk decisions/");

        // `drifted`: recorded baseline differs from on-disk bytes → hash-matches.
        let drifted_path = "decisions/drifted.md";
        std::fs::write(decisions.join("drifted.md"), ADR_B_EDITED_SUPERSEDES)
            .expect("write drifted ADR");
        // `synced`: on-disk bytes equal the recorded baseline → no finding.
        let synced_path = "decisions/synced.md";
        std::fs::write(decisions.join("synced.md"), ADR_B_BASE).expect("write synced ADR");
        // `fresh`: a committed doc with NO recorded hash → un-baselined advisory.
        let fresh_path = "decisions/fresh.md";
        std::fs::write(decisions.join("fresh.md"), ADR_B_BASE).expect("write fresh ADR");

        let mut record = FileStateRecord::new();
        record.record(drifted_path, hash_bytes(ADR_B_BASE.as_bytes())); // pre-edit baseline
        record.record(synced_path, hash_bytes(ADR_B_BASE.as_bytes())); // matches on-disk
        // (no record for fresh_path)

        // Clone the record to assert byte-for-byte mutation-freedom after the call.
        let record_before = record.clone();

        // No `versions` ⇒ the doctype is unversioned ⇒ the managed-vs-foreign discriminator
        // is inert here (its precondition: it answers only where a stamp can exist). These two
        // pin the drift / un-baselined / in-sync arms, unchanged by M42.
        let findings = detect_committed_store(
            &record,
            &schemas,
            root.path(),
            &|_| None,
            &std::collections::BTreeMap::new(),
            &std::collections::BTreeMap::new(),
        );

        // (i) the drifted doc → exactly one hash-matches finding, store-scope route.
        let drift: Vec<&Finding> = findings
            .iter()
            .filter(|f| f.code == "file-state.hash-matches")
            .collect();
        assert_eq!(
            drift.len(),
            1,
            "exactly one hash-matches finding (the drifted doc): {findings:?}"
        );
        let drift = drift[0];
        assert_eq!(drift.severity, Severity::Blocking);
        assert!(
            drift.message.contains(drifted_path),
            "the drift finding names the drifted doc: {drift:?}"
        );
        let route = drift
            .route
            .as_deref()
            .expect("the store-scope drift carries a route");
        assert!(
            route != format!("reconcile {drifted_path}") && !route.starts_with("reconcile"),
            "the store-scope route is NOT the task-scope `reconcile <path>` variant: {route:?}"
        );

        // (iii) the un-baselined doc → exactly one distinct advisory finding.
        let unbaselined: Vec<&Finding> = findings
            .iter()
            .filter(|f| f.message.contains(fresh_path))
            .collect();
        assert_eq!(
            unbaselined.len(),
            1,
            "exactly one finding for the un-baselined doc: {findings:?}"
        );
        let unbaselined = unbaselined[0];
        assert_eq!(
            unbaselined.severity,
            Severity::Advisory,
            "un-baselined is informational, neither drift nor a block: {unbaselined:?}"
        );
        assert_ne!(
            unbaselined.code, "file-state.hash-matches",
            "un-baselined is a DISTINCT outcome from drift, not silent-clean"
        );

        // (iv) the in-sync doc → no finding.
        assert!(
            findings.iter().all(|f| !f.message.contains(synced_path)),
            "the in-sync doc emits no finding: {findings:?}"
        );

        // (ii) mutation-free: the record is byte-identical before and after.
        assert_eq!(
            record, record_before,
            "detect_committed_store must not mutate the record (no absorb, no re-baseline)"
        );
    }

    /// (M42 inc-3 T3) The **un-baselined advisory is a claim about a MANAGED doc** — and both
    /// halves of that sentence are pinned here, over an **empty record**: the state of a
    /// **fresh clone** of a managed repo, where `.jigc/state/` (gitignored, rebuildable) does
    /// not exist at all.
    ///
    /// - the **managed** doc (unstamped, but it parses against the current shape — the v0-era
    ///   corpus) → its `file-state.un-baselined` advisory **still fires**. This is the guard:
    ///   the M42 suppression must reach **foreign docs only**, never a managed corpus on a
    ///   clean checkout. Suppressing on *record membership* — the discriminator the Settle
    ///   first reached for — would silence exactly this doc, on exactly this checkout.
    /// - the **foreign** doc (parses against no known version of the doctype, the squatter's
    ///   signature) → **no** un-baselined finding: its route (*"baselined on its next author
    ///   or finalize"*) would promise an authoring that will never happen. It is not left
    ///   silent — family 5 gives it the one finding it earns, the
    ///   `schema-conformance.unadopted-instance` adoption advisory
    ///   (`crates/cli/tests/managed_vs_foreign.rs`, through the real binary).
    ///
    /// The doctype is **versioned** here (`versions` carries it) — the discriminator's
    /// precondition, without which the classifier is inert and the foreign arm would not be
    /// exercised at all.
    #[test]
    fn detect_committed_store_suppresses_unbaselined_for_the_foreign_squatter_only() {
        let mut schemas: std::collections::BTreeMap<String, Schema> =
            std::collections::BTreeMap::new();
        schemas.insert("adr".to_string(), adr_schema());
        // The doctype is versioned (the CLI stamps it) — so "unstamped" is evidence, and the
        // classifier is willing to answer.
        let versions: std::collections::BTreeMap<String, u32> =
            [("adr".to_string(), 1u32)].into_iter().collect();
        // No shipped prior shapes needed: the managed doc below parses against the current one.
        let priors: std::collections::BTreeMap<String, Vec<Schema>> =
            std::collections::BTreeMap::new();

        let root = TempRoot::new("detect-twin-foreign");
        let decisions = root.path().join("decisions");
        std::fs::create_dir_all(&decisions).expect("mk decisions/");

        // A genuinely MANAGED, un-baselined ADR: no stamp (the v0-era corpus), but it parses.
        let managed_path = "decisions/distributed-session-cache.md";
        std::fs::write(decisions.join("distributed-session-cache.md"), ADR_B_BASE)
            .expect("write managed ADR");
        // A FOREIGN file squatting at the `adr` home: parses against no known `adr` version.
        let foreign_path = "decisions/notes.md";
        std::fs::write(
            decisions.join("notes.md"),
            "# Meeting notes\n\n## Attendees\n\n- Ada\n",
        )
        .expect("write foreign note");

        // A FRESH CLONE: the file-state record is empty — neither doc is recorded.
        let record = FileStateRecord::new();

        let findings = detect_committed_store(
            &record,
            &schemas,
            root.path(),
            &|_| None,
            &versions,
            &priors,
        );

        let managed: Vec<&Finding> = findings
            .iter()
            .filter(|f| f.message.contains(managed_path))
            .collect();
        assert_eq!(
            managed.len(),
            1,
            "the MANAGED un-baselined doc still fires on a fresh clone (the guard): {findings:?}"
        );
        assert_eq!(managed[0].code, "file-state.un-baselined");
        assert_eq!(managed[0].severity, Severity::Advisory);

        assert!(
            findings.iter().all(|f| !f.message.contains(foreign_path)),
            "the never-adopted foreign squatter is NOT un-baselined — it is family 5's \
             adoption case, and this advisory would promise a baseline on an author or \
             finalize that will never touch it: {findings:?}"
        );
    }

    /// (M42 inc-2 T2) The read-only twin [`detect_committed_store`] sees a **placement**
    /// doctype's committed instance at its literal home — the *12th census site*
    /// (`design/storage.md` → The census: `file_state::detect_committed_store`). Its
    /// `location: None` skip dropped the whole placement class, so **`jigc validate` could
    /// not see an out-of-band edit to `CHANGELOG.md`/`VISION.md`** — voiding
    /// [CLAUDE.md](../../../CLAUDE.md)'s *"out-of-band edits are detected and routed"*
    /// invariant for that class, while the *mutating* twin ([`reconcile_committed_store`])
    /// had carried its placement branch since M38.
    ///
    /// Both outcomes are pinned on placement docs at their literal homes:
    ///
    /// - a **recorded-then-drifted** root `FOO.md` → one blocking `file-state.hash-matches`
    ///   naming the literal path, carrying the **store-scope** route (never `reconcile …`);
    /// - an **un-baselined** `docs/bar.md` → one advisory `file-state.un-baselined`;
    /// - a sibling root `README.md` declared by nothing → **no** finding (exact-path
    ///   ownership, not a root dir-glob — the non-vacuous half: the sweep visited the
    ///   literals, and only them);
    /// - the `record` stays **byte-identical** — detect-without-absorb survives the new
    ///   branch (the twin must never re-baseline the drift it exists to surface).
    #[test]
    fn detect_committed_store_sees_placement_docs_at_their_literal_homes() {
        let mut schemas: BTreeMap<String, Schema> = BTreeMap::new();
        schemas.insert("foo".to_string(), adr_placement_schema("FOO.md"));
        schemas.insert("bar".to_string(), adr_placement_schema("docs/bar.md"));

        let root = TempRoot::new("detect-placement");
        // `FOO.md`: recorded baseline ≠ on-disk bytes → drift.
        std::fs::write(root.path().join("FOO.md"), ADR_B_EDITED_SUPERSEDES)
            .expect("write the drifted placement doc");
        // `docs/bar.md`: committed with no recorded hash → un-baselined.
        std::fs::create_dir_all(root.path().join("docs")).expect("mk docs/");
        std::fs::write(root.path().join("docs").join("bar.md"), ADR_B_BASE)
            .expect("write the un-baselined placement doc");
        // A sibling root `.md` no schema declares — must stay unmanaged.
        std::fs::write(root.path().join("README.md"), "# readme\n\nnot managed\n")
            .expect("write the README.md sibling");

        let mut record = FileStateRecord::new();
        record.record("FOO.md", hash_bytes(ADR_B_BASE.as_bytes())); // the pre-edit baseline
        let record_before = record.clone();

        // No `versions` ⇒ the doctype is unversioned ⇒ the managed-vs-foreign discriminator
        // is inert here (its precondition: it answers only where a stamp can exist). These two
        // pin the drift / un-baselined / in-sync arms, unchanged by M42.
        let findings = detect_committed_store(
            &record,
            &schemas,
            root.path(),
            &|_| None,
            &std::collections::BTreeMap::new(),
            &std::collections::BTreeMap::new(),
        );

        // (i) the drifted placement doc → one blocking hash-matches, store-scope route.
        let drift: Vec<&Finding> = findings
            .iter()
            .filter(|f| f.code == "file-state.hash-matches")
            .collect();
        assert_eq!(
            drift.len(),
            1,
            "the OOB edit to the placement doc at its literal home is detected: {findings:?}"
        );
        let drift = drift[0];
        assert_eq!(drift.severity, Severity::Blocking);
        assert!(
            drift.message.contains("FOO.md"),
            "the drift names the literal placement home: {drift:?}"
        );
        let route = drift
            .route
            .as_deref()
            .expect("the store-scope drift carries a route");
        assert!(
            !route.starts_with("reconcile"),
            "the store-scope route is NOT the task-scope `reconcile <path>` variant: {route:?}"
        );

        // (ii) the un-baselined placement doc → one advisory un-baselined finding.
        let unbaselined: Vec<&Finding> = findings
            .iter()
            .filter(|f| f.code == "file-state.un-baselined")
            .collect();
        assert_eq!(
            unbaselined.len(),
            1,
            "the un-baselined placement doc is surfaced, not silently clean: {findings:?}"
        );
        let unbaselined = unbaselined[0];
        assert_eq!(unbaselined.severity, Severity::Advisory);
        assert!(
            unbaselined.message.contains("docs/bar.md"),
            "the advisory names the literal placement home: {unbaselined:?}"
        );

        // (iii) the undeclared sibling root .md is not swept (exact-path, never a glob).
        assert!(
            findings.iter().all(|f| !f.message.contains("README.md")),
            "an undeclared sibling root .md is not swept as an instance: {findings:?}"
        );

        // (iv) detect-without-absorb: the record is byte-identical across the sweep.
        assert_eq!(
            record, record_before,
            "the read-only twin must not re-baseline the drift it exists to surface"
        );
    }

    /// **L1's store arm** (M55 Increment 4, P4): the twin asks one more fact of a recorded doc
    /// that has drifted — its committed bytes at `HEAD` — and grades the drift by it.
    ///
    /// - **(i)** bytes equal the `HEAD` blob **and** conform → the one finding is
    ///   `file-state.hash-matches` at **advisory**, keyed and messaged as the blocking drift,
    ///   with the informational route *"the baseline lags `HEAD`; absorbed at the next
    ///   finalize"* — a pull, or a conformant edit committed with plain git;
    /// - **(ii)** bytes equal the `HEAD` blob but do **not** conform → today's blocking drift,
    ///   unchanged: matching bytes are not on their own a clean doc;
    /// - **(iii)** bytes that differ from the `HEAD` blob (an uncommitted edit) → today's
    ///   blocking drift, unchanged;
    /// - **(iv)** the record is borrowed `&` and byte-identical across the sweep, and the seam
    ///   is asked only about the drifted docs — never the in-sync one.
    #[test]
    fn detect_committed_store_grades_a_drift_that_equals_head_advisory_behind_conformance() {
        let mut schemas: BTreeMap<String, Schema> = BTreeMap::new();
        schemas.insert("adr".to_string(), adr_schema());

        let root = TempRoot::new("detect-lag");
        let decisions = root.path().join("decisions");
        std::fs::create_dir_all(&decisions).expect("mk decisions/");
        // Each doc's on-disk bytes, and what `HEAD` carries for it.
        let docs: [(&str, &str, &str); 4] = [
            // (i) pulled: the conformant edit is on disk and at HEAD.
            ("pulled", ADR_B_EDITED_SUPERSEDES, ADR_B_EDITED_SUPERSEDES),
            // (ii) committed-broken: the non-conformant edit is on disk and at HEAD.
            ("broken", ADR_B_EDITED_BAD_DATE, ADR_B_EDITED_BAD_DATE),
            // (iii) uncommitted: the edit is on disk, HEAD still carries the baseline.
            ("local", ADR_B_EDITED_SUPERSEDES, ADR_B_BASE),
            // in-sync: no drift, so the seam must not be asked.
            ("synced", ADR_B_BASE, ADR_B_BASE),
        ];
        let mut record = FileStateRecord::new();
        let mut at_head: BTreeMap<String, Vec<u8>> = BTreeMap::new();
        for (slug, on_disk, committed) in docs {
            std::fs::write(decisions.join(format!("{slug}.md")), on_disk).expect("write doc");
            let key = format!("decisions/{slug}.md");
            record.record(&key, hash_bytes(ADR_B_BASE.as_bytes()));
            at_head.insert(key, committed.as_bytes().to_vec());
        }
        let record_before = record.clone();
        let asked = std::cell::RefCell::new(Vec::new());
        let head = |path: &str| {
            asked.borrow_mut().push(path.to_string());
            at_head.get(path).cloned()
        };

        let findings = detect_committed_store(
            &record,
            &schemas,
            root.path(),
            &head,
            &BTreeMap::new(),
            &BTreeMap::new(),
        );

        let at = |path: &str| -> Vec<&Finding> {
            findings
                .iter()
                .filter(|f| f.location.as_ref().and_then(|l| l.address.as_deref()) == Some(path))
                .collect()
        };
        // (i) HEAD-equal and conformant → one advisory hash-matches, the lag route.
        let pulled = at("decisions/pulled.md");
        assert_eq!(
            pulled.len(),
            1,
            "one finding for the pulled doc: {findings:?}"
        );
        let lag = pulled[0];
        assert_eq!(lag.code, "file-state.hash-matches", "no new id: {lag:?}");
        assert_eq!(
            lag.severity,
            Severity::Advisory,
            "the lag is advisory: {lag:?}"
        );
        assert_eq!(
            lag.message,
            drift_store_finding("decisions/pulled.md").message,
            "the message stays the drift's: {lag:?}"
        );
        let route = lag.route.as_ref().expect("the lag routes");
        assert_eq!(
            route.as_str(),
            "the baseline lags `HEAD`; absorbed at the next finalize"
        );
        assert!(
            matches!(route.kind(), crate::finding::RouteKind::Informational),
            "the lag informs and directs nothing: {route:?}"
        );
        // (ii) HEAD-equal but non-conformant, and (iii) uncommitted → today's blocking drift.
        for path in ["decisions/broken.md", "decisions/local.md"] {
            assert_eq!(
                at(path),
                vec![&drift_store_finding(path)],
                "`{path}` keeps today's blocking drift: {findings:?}"
            );
        }
        assert!(at("decisions/synced.md").is_empty(), "{findings:?}");
        // (iv) read-only, and the seam is asked about the drifted docs only.
        assert_eq!(record, record_before, "the twin never baselines the lag");
        assert_eq!(
            asked.into_inner(),
            vec![
                "decisions/broken.md".to_string(),
                "decisions/local.md".to_string(),
                "decisions/pulled.md".to_string(),
            ],
            "only a drifted doc consults the seam"
        );
    }

    /// (M38 inc-1 T3) The two path↔identity sites recognize a **placement** doctype's
    /// committed path by **exact-path equality** against `placement.file` (`design/storage.md`
    /// → Placement — census sites `identity_of`, `persisted_committed_path`):
    ///
    /// - [`identity_of`] maps the literal file to `<type>:<type>` — the fixed slug **is** the
    ///   type id (a placement singleton carries no title-derived slug), for both a root
    ///   `FOO.md` and a `docs/bar.md`;
    /// - [`persisted_committed_path`] returns `true` for either literal (a placement doc is a
    ///   committed managed doc, so the store sweep / rename detection must own it);
    /// - a **sibling non-declared** root path (`README.md`) is **unowned** by both (a literal
    ///   file is not a dir-glob — every other root `.md` stays unmanaged);
    /// - **regression**: the existing `location:`-keyed behavior is byte-unchanged — a
    ///   `decisions/x.md` still maps to `adr:x` and is still a persisted committed path.
    #[test]
    fn placement_doctype_path_is_owned_by_exact_file_equality() {
        let foo = crate::schema::load_schema(
            b"\
type: foo
placement: { file: FOO.md }
sections: []
",
        )
        .expect("root-placement schema loads");
        let bar = crate::schema::load_schema(
            b"\
type: bar
placement: { file: docs/bar.md }
sections: []
",
        )
        .expect("docs-placement schema loads");
        let mut schemas: BTreeMap<String, Schema> = BTreeMap::new();
        schemas.insert("foo".to_string(), foo);
        schemas.insert("bar".to_string(), bar);
        schemas.insert("adr".to_string(), adr_schema());

        // identity_of: literal file → <type>:<type> (fixed slug = type id).
        assert_eq!(
            identity_of("FOO.md", &schemas).as_deref(),
            Some("foo:foo"),
            "a root placement file's identity is <type>:<type>, not filename-derived",
        );
        assert_eq!(
            identity_of("docs/bar.md", &schemas).as_deref(),
            Some("bar:bar"),
            "a docs/ placement file's identity is <type>:<type>, exact-path matched",
        );

        // persisted_committed_path: a placement literal is a committed managed doc.
        assert!(
            persisted_committed_path("FOO.md", &schemas),
            "a root placement literal is a persisted committed path",
        );
        assert!(
            persisted_committed_path("docs/bar.md", &schemas),
            "a docs/ placement literal is a persisted committed path",
        );

        // A sibling non-declared root path is unowned by both — a literal is not a glob.
        assert_eq!(
            identity_of("README.md", &schemas),
            None,
            "an undeclared sibling root .md is not owned by any placement doctype",
        );
        assert!(
            !persisted_committed_path("README.md", &schemas),
            "an undeclared sibling root .md is not a persisted committed path",
        );

        // Regression: the existing location-keyed behavior is byte-unchanged.
        assert_eq!(
            identity_of("decisions/x.md", &schemas).as_deref(),
            Some("adr:x"),
            "a location-keyed doc still maps to <type>:<slug> unchanged",
        );
        assert!(
            persisted_committed_path("decisions/x.md", &schemas),
            "a location-keyed committed doc is still a persisted committed path",
        );
    }

    /// A placement fixture: an ADR schema re-homed as a literal-file placement doctype
    /// (`location: None`, `placement.file = file`) — so its committed instance lives at
    /// one exact path, exactly the shape a real `VISION.md`/`CHANGELOG.md` singleton uses.
    fn adr_placement_schema(file: &str) -> Schema {
        let mut schema = adr_schema();
        schema.location = None;
        schema.placement = Some(crate::schema::Placement {
            file: file.to_string(),
        });
        schema
    }

    /// (M38 inc-2 T1) A **placement** doctype's committed instance is swept by
    /// [`reconcile_committed_store`] at its exact literal `placement.file` — closing the
    /// headline gap where the `location: None` `continue` skipped it and an OOB edit went
    /// silently undetected (`design/storage.md` → Placement census: `reconcile_committed_store`).
    ///
    /// - a recorded placement doc whose on-disk `FOO.md` **drifted** (here nonconformantly)
    ///   is **detected + routed**: a blocking `reconciliation.conformance-block` naming the
    ///   literal file, carrying a route;
    /// - a sibling root `README.md` — declared by no schema — is **not** swept as an instance
    ///   (a literal file is not a dir-glob): no finding names it, and it is not baseline-adopted.
    #[test]
    fn placement_doc_oob_drift_is_detected_and_sibling_not_swept() {
        let mut schemas: BTreeMap<String, Schema> = BTreeMap::new();
        schemas.insert("foo".to_string(), adr_placement_schema("FOO.md"));

        let root = TempRoot::new("placement-drift");
        // The managed placement instance drifted out-of-band, nonconformantly (a renamed
        // heading) — recorded baseline is the pre-edit conformant content.
        let broken = ADR_B_BASE.replace("## Decision", "## Decisionz");
        std::fs::write(root.path().join("FOO.md"), &broken).expect("write drifted FOO.md");
        // A sibling root .md declared by nothing — must stay unmanaged.
        std::fs::write(root.path().join("README.md"), "# readme\n\nnot managed\n")
            .expect("write README.md sibling");

        let mut record = FileStateRecord::new();
        record.record("FOO.md", hash_bytes(ADR_B_BASE.as_bytes()));

        let mut index = EdgeIndex::default();
        let task = TempRoot::new("placement-drift-task");

        let findings = reconcile_committed_store(
            &mut record,
            &mut index,
            &schemas,
            root.path(),
            task.path(),
            &|_| true,
            &|_| true,
            &|_| None,
            &test_conflict(),
            &crate::validate::AdoptionInputs::inert(),
            &LiveRecord::none(),
        );

        // (a) the OOB edit to the managed placement file is detected + routed.
        let block = findings
            .iter()
            .find(|f| f.code == "reconciliation.conformance-block")
            .expect("the placement doc's OOB drift is detected (not silently skipped)");
        assert_eq!(block.severity, Severity::Blocking);
        assert!(
            block.message.contains("FOO.md"),
            "the block names the drifted placement file: {block:?}"
        );
        assert!(block.route.is_some(), "the block carries a route");

        // (c) the sibling root README.md is not swept as an instance.
        assert!(
            findings.iter().all(|f| !f.message.contains("README.md")),
            "an undeclared sibling root .md is not swept: {findings:?}"
        );
        assert_eq!(
            record.get("README.md"),
            None,
            "the sibling is not baseline-adopted (a literal file is not a dir-glob)"
        );
    }

    /// (M38 inc-2 T1) A recorded **placement** doc gone **missing** on disk routes through
    /// the existing rename arm — Inc-1 taught `identity_of`/`persisted_committed_path` the
    /// literal path, so a missing `FOO.md` with no content-matching candidate is the
    /// weak-signal `reconciliation.rename` routed to restore (`design/reconciliation.md` →
    /// Rename detection → weak signal). Guards that the new placement sweep arm does not
    /// shadow the recorded-but-missing routing.
    #[test]
    fn placement_doc_missing_routes_through_rename_arm() {
        let mut schemas: BTreeMap<String, Schema> = BTreeMap::new();
        schemas.insert("foo".to_string(), adr_placement_schema("FOO.md"));

        let root = TempRoot::new("placement-missing");
        // FOO.md is recorded but absent on disk (deleted out of band).
        let mut record = FileStateRecord::new();
        record.record("FOO.md", hash_bytes(ADR_B_BASE.as_bytes()));

        let mut index = EdgeIndex::default();
        let task = TempRoot::new("placement-missing-task");

        let findings = reconcile_committed_store(
            &mut record,
            &mut index,
            &schemas,
            root.path(),
            task.path(),
            &|_| true,
            &|_| true,
            &|_| None,
            &test_conflict(),
            &crate::validate::AdoptionInputs::inert(),
            &LiveRecord::none(),
        );

        let rename = findings
            .iter()
            .find(|f| f.code == "reconciliation.rename")
            .expect("a missing recorded placement doc routes to rename detection");
        assert_eq!(rename.severity, Severity::Blocking);
        assert!(
            rename.message.contains("FOO.md") && rename.message.contains("missing"),
            "the rename names the missing placement path: {rename:?}"
        );
        assert!(
            rename
                .route
                .as_deref()
                .is_some_and(|r| r.contains("restore")),
            "no content-matching suspect exists, so the weak signal routes to restore: {rename:?}"
        );
    }

    /// (M38 inc-2 T1) [`untracked_committed`] enumerates an **untracked** on-disk placement
    /// file (no recorded hash) as a rename candidate — the location-dir-only scan is taught
    /// the literal `placement.file` (`design/storage.md` → Placement census: `untracked_committed`).
    /// A sibling root `README.md` declared by nothing is **not** a candidate (exact-path, not a
    /// root glob).
    #[test]
    fn untracked_committed_includes_an_untracked_placement_file() {
        let mut schemas: BTreeMap<String, Schema> = BTreeMap::new();
        schemas.insert("foo".to_string(), adr_placement_schema("FOO.md"));

        let root = TempRoot::new("placement-untracked");
        let foo_bytes = ADR_B_BASE.as_bytes();
        std::fs::write(root.path().join("FOO.md"), foo_bytes).expect("write untracked FOO.md");
        std::fs::write(root.path().join("README.md"), "# readme\n")
            .expect("write README.md sibling");

        // Nothing recorded → FOO.md is an untracked candidate.
        let recorded: std::collections::BTreeSet<String> = std::collections::BTreeSet::new();
        let untracked = untracked_committed(&recorded, &schemas, root.path());

        assert!(
            untracked
                .iter()
                .any(|(p, h)| p == "FOO.md" && h == &hash_bytes(foo_bytes)),
            "the untracked placement file is a candidate with its raw-byte hash: {untracked:?}"
        );
        assert!(
            untracked.iter().all(|(p, _)| p != "README.md"),
            "an undeclared sibling root .md is not an untracked candidate: {untracked:?}"
        );
    }

    /// (M53 — the pre-v1 usability batch, row 1 / the rc.19 review's `(2, N-2)`) An
    /// out-of-band `git mv` of a **placement** singleton reaches
    /// [`detect_rename`]'s **strong** arm — the arm that names the pair and emits the
    /// `git -C <home> mv <new> <old>` revert route the installed pre-commit hook's blocking
    /// backstop keys on.
    ///
    /// **The class this iterates is the two shapes a `placement.file` can take**, not the
    /// shipped doctype ids: a **root-declared** home (`VISION.md`, `CHANGELOG.md`) and a
    /// **directory-declared** one (`docs/roadmap.md`, `docs/decisions-log.md`). Every
    /// shipped placement doctype is one shape or the other, and the census arm under test
    /// branches on exactly that — `Path::parent()` — so a per-doctype list would have
    /// iterated one axis five times.
    ///
    /// Before the fix both cells produced **no candidate at all**: the census enumerated
    /// the declared literal path only, which is precisely the path a rename vacates. The
    /// negative control rides in the same cells — an unrelated sibling of different content
    /// is a candidate and contributes **no** finding, because the strong arm is gated on an
    /// exact recorded-content-hash match.
    #[test]
    fn a_renamed_placement_singleton_reaches_the_strong_rename_arm_at_both_home_shapes() {
        for (declared, moved_to, unrelated) in [
            ("FOO.md", "FOO-OOB.md", "README.md"),
            ("docs/FOO.md", "docs/FOO-OOB.md", "docs/NOTES.md"),
        ] {
            let mut schemas: BTreeMap<String, Schema> = BTreeMap::new();
            schemas.insert("foo".to_string(), adr_placement_schema(declared));

            let root = TempRoot::new("placement-renamed");
            let bytes = ADR_B_BASE.as_bytes();
            for rel in [moved_to, unrelated] {
                let abs = root.path().join(rel);
                if let Some(parent) = abs.parent() {
                    std::fs::create_dir_all(parent).expect("mk home dir");
                }
                std::fs::write(&abs, if rel == moved_to { bytes } else { b"# other\n" })
                    .expect("write sibling");
            }

            // The declared home is vacated; the bytes sit beside it under a new name.
            let recorded: std::collections::BTreeSet<String> =
                std::iter::once(declared.to_string()).collect();
            let untracked = untracked_committed(&recorded, &schemas, root.path());
            assert!(
                untracked
                    .iter()
                    .any(|(p, h)| p == moved_to && h == &hash_bytes(bytes)),
                "[{declared}] the vacated home's sibling is a rename candidate: {untracked:?}"
            );

            let untracked_refs: Vec<(&str, String)> = untracked
                .iter()
                .map(|(p, h)| (p.as_str(), h.clone()))
                .collect();
            let findings = detect_rename(
                declared,
                "foo:foo",
                &hash_bytes(bytes),
                &untracked_refs,
                &|_| true,
                &|_| true,
                &LiveRecord::none(),
                root.path(),
            );
            let rename = findings
                .iter()
                .find(|f| f.code == "reconciliation.rename")
                .unwrap_or_else(|| panic!("[{declared}] a rename finding is emitted"));
            assert_eq!(rename.severity, Severity::Blocking);
            assert!(
                rename.message.contains(moved_to) && rename.message.contains("same content hash"),
                "[{declared}] the strong arm names the suspect: {rename:?}"
            );
            let route = rename.route.as_deref().unwrap_or_default();
            assert!(
                route.contains(&format!("mv {moved_to} {declared}")),
                "[{declared}] the revert route carries the `mv` pair the hook's backstop \
                 keys on: {route}"
            );
            assert!(
                !route.contains(unrelated) && !rename.message.contains(unrelated),
                "[{declared}] the content-unmatched sibling reaches no finding: {rename:?}"
            );
        }
    }

    /// (M38 inc-2 T2) The finalize post-commit baseline-adopt gate
    /// [`committed_path_recordable`] learns a **placement** doctype's exact-path ownership
    /// (`design/storage.md` → Placement census: `committed_path_recordable`). Before this
    /// arm the gate found a managed doc by `location:` prefix only, so a placement file
    /// (`location: None`) matched nothing → `managed_doc = None` → **always recordable**,
    /// wrongly baseline-adopting a foreign non-conformant `FOO.md`. With the exact-path
    /// branch it mirrors the location arm's conformance gate:
    ///
    /// - (a) a **conformant** placement file at `FOO.md` is recordable;
    /// - (b) a **non-conformant** one at the same literal path is **not** recordable — it
    ///   stays `UNKNOWN` so the advisory re-fires every finalize until the human resolves it;
    /// - (c) an **undeclared** sibling root `.md` (owned by no schema) is recordable — the
    ///   unowned `None` arm is unchanged (a literal file is not a root glob).
    #[test]
    fn committed_path_recordable_learns_placement_exact_path() {
        let mut schemas: BTreeMap<String, Schema> = BTreeMap::new();
        schemas.insert("foo".to_string(), adr_placement_schema("FOO.md"));

        // (a) a conformant placement file at its exact literal path is recordable.
        assert!(
            committed_path_recordable(&schemas, "FOO.md", ADR_B_BASE.as_bytes()),
            "a conformant placement file at its literal path is recordable"
        );

        // (b) a non-conformant file at the same literal path is NOT recordable — it stays
        // UNKNOWN (the routed-but-not-recorded re-fire), never silently baseline-adopted.
        let broken = ADR_B_BASE.replace("## Decision", "## Decisionz");
        assert!(
            !committed_path_recordable(&schemas, "FOO.md", broken.as_bytes()),
            "a non-conformant placement file at its literal path is NOT recordable"
        );

        // (c) an undeclared sibling root .md — owned by no schema — is recordable (the
        // unowned None arm, unchanged; a placement literal is exact-path, not a root glob).
        assert!(
            committed_path_recordable(&schemas, "README.md", b"# readme\n\nnot managed\n"),
            "an undeclared sibling root .md is always recordable (unowned)"
        );
    }

    /// A throwaway directory that removes itself on drop.
    struct TempRoot(PathBuf);

    impl TempRoot {
        fn new(tag: &str) -> Self {
            let mut path = std::env::temp_dir();
            let unique = format!(
                "jigc-file-state-{tag}-{}-{:?}",
                std::process::id(),
                crate::tempname::unique_nanos(),
            );
            path.push(unique);
            std::fs::create_dir_all(&path).expect("create temp root");
            TempRoot(path)
        }

        fn path(&self) -> &Path {
            &self.0
        }
    }

    impl Drop for TempRoot {
        fn drop(&mut self) {
            let _ = std::fs::remove_dir_all(&self.0);
        }
    }

    /// The done-criterion: (a) the hash of a fixed *raw-byte* input equals a
    /// pinned `blake3` hex (golden), proving raw-byte (non-canonicalized) hashing
    /// over a non-ASCII, multi-byte input; and (b) save→load of a multi-entry
    /// record round-trips byte-stably (path-sorted, identical bytes).
    #[test]
    fn file_state_hash_is_raw_byte_blake3_and_round_trips() {
        // (a) Raw-byte blake3 golden. The input carries a UTF-8 check mark (✓)
        // and an é plus a trailing newline — bytes that a canonicalizer would
        // touch; the drift hash must not, so the digest is pinned to the raw
        // bytes verbatim.
        let input: &[u8] = b"jigc file-state drift hash \xe2\x9c\x93 caf\xc3\xa9\n";
        assert_eq!(
            hash_bytes(input),
            "4c4d3fd9e60ea710965b8f8c2b6bbc63974af6f803baf098cd10d84e55c5aceb",
            "raw-byte blake3 hex of the fixed input must match the pinned golden",
        );

        // (b) Multi-entry record, recorded out of path order, saved and loaded.
        let mut record = FileStateRecord::new();
        record.record("decisions/rate-limit.md", hash_bytes(b"adr body\n"));
        record.record("specs/gateway.md", hash_bytes(b"spec body\n"));
        record.record(
            "decisions/auth.md",
            hash_bytes(b"another adr\xc3\xa9 body\n"),
        );

        let root = TempRoot::new("roundtrip");
        record.save(root.path()).expect("save the record");

        // The serialized bytes are deterministic: path-sorted, pretty, one
        // trailing newline. Golden-pin the exact on-disk byte form.
        let on_disk =
            std::fs::read_to_string(FileStateRecord::path_in(root.path())).expect("record written");
        insta::assert_snapshot!(on_disk, @r#"
        {
          "hashes": {
            "decisions/auth.md": "f1217111b9f36d9883a5924a7c7cbc216bd3062fb25282d31a0e47654976871d",
            "decisions/rate-limit.md": "874feccab2c6ac7ce7e7c0fe96442ad9144871796370127452bd7b21bfc78ee3",
            "specs/gateway.md": "b2eec3f1ab19c18fe2324d109745dd543f0e7d68a772f9b0162cf71e34303ae2"
          }
        }
        "#);

        // save → load round-trips the in-memory record exactly.
        let loaded = FileStateRecord::load(root.path()).expect("load the record");
        assert_eq!(loaded, record, "save → load must round-trip byte-stably");
    }

    /// **The base-relative three-way merge, iterated over the mutation-kind axis**
    /// (M46 Increment 1 / T1 — `DECISIONS.md` 2026-08-18 M46 planned, N-3).
    ///
    /// `.jigc/state/file-state.json` is **shared**, not task-isolated, so two writers
    /// legitimately hold the same record at once. Before this fix `save` wrote `self`
    /// verbatim: the later saver silently discarded every per-key delta the other
    /// writer had landed in between — a *reported success* for a write that was
    /// thrown away, which is what `CLAUDE.md`'s "never silently merged" forbids.
    ///
    /// The rule, over `base ∪ ours ∪ theirs`: **`ours == base` ⇒ take theirs** (their
    /// value *or* its absence — we did not touch this key, so the other writer's
    /// decision stands); **otherwise ⇒ ours** (we recorded or forgot it deliberately,
    /// so our decision stands, including a deletion).
    ///
    /// The axis is the **mutation kind on each side** — `record` and `forget`, both
    /// ways — enumerated as a code-side table so a third mutation kind cannot be added
    /// to [`FileStateRecord`] without a cell here. (The post-sweep hand-off, the third
    /// axis member, is a real-binary cell: `cli/tests/file_state_merge_hand_off.rs`.)
    /// Every cell is a **sequential interleave** (A loads · B loads-mutates-saves · A
    /// mutates-saves), never true concurrency: the read-modify-write window this leaves
    /// open is T2's lock, not this merge's claim.
    #[test]
    fn save_merges_a_concurrent_writers_delta_over_the_mutation_kind_axis() {
        /// One cell of the axis: the seeded base, what the *other* writer did to it,
        /// what *we* then did to our own loaded copy, and the map that must be on disk
        /// after we save last.
        struct Cell {
            name: &'static str,
            base: &'static [(&'static str, &'static str)],
            theirs: fn(&mut FileStateRecord),
            ours: fn(&mut FileStateRecord),
            expect: &'static [(&'static str, &'static str)],
        }

        const HA: &str = "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa";
        const HB: &str = "bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb";
        const H0: &str = "0000000000000000000000000000000000000000000000000000000000000000";
        const HX: &str = "1111111111111111111111111111111111111111111111111111111111111111";
        const HY: &str = "2222222222222222222222222222222222222222222222222222222222222222";

        let axis: &[Cell] = &[
            // record × record — two writers touching disjoint keys: both survive.
            Cell {
                name: "record: disjoint keys both survive",
                base: &[("docs/decisions/p.md", H0)],
                theirs: |r| r.record("docs/decisions/x.md", HX),
                ours: |r| r.record("docs/decisions/y.md", HY),
                expect: &[
                    ("docs/decisions/p.md", H0),
                    ("docs/decisions/x.md", HX),
                    ("docs/decisions/y.md", HY),
                ],
            },
            // forget × record — their deletion of a key we never touched stands, and
            // our own addition lands. `jigc unmanage` racing any recorder.
            Cell {
                name: "forget: their forget of an untouched key stays gone",
                base: &[("docs/decisions/p.md", H0)],
                theirs: |r| {
                    r.forget("docs/decisions/p.md");
                },
                ours: |r| r.record("docs/decisions/q.md", HY),
                expect: &[("docs/decisions/q.md", HY)],
            },
            // record × record on ONE key — the conflict cell. The later saver's value
            // wins, deterministically; the bystander keys still merge.
            Cell {
                name: "conflict: the later saver's value wins, bystanders still merge",
                base: &[("docs/decisions/k.md", H0), ("docs/decisions/keep.md", HX)],
                theirs: |r| {
                    r.record("docs/decisions/k.md", HB);
                    r.record("docs/decisions/extra.md", HY);
                },
                ours: |r| r.record("docs/decisions/k.md", HA),
                expect: &[
                    ("docs/decisions/extra.md", HY),
                    ("docs/decisions/k.md", HA),
                    ("docs/decisions/keep.md", HX),
                ],
            },
            // record × forget on ONE key — our deletion is a deliberate decision, so it
            // beats their concurrent re-record; their unrelated addition still lands.
            Cell {
                name: "forget: our forget beats a concurrent re-record",
                base: &[("docs/decisions/p.md", H0)],
                theirs: |r| {
                    r.record("docs/decisions/p.md", HB);
                    r.record("docs/decisions/other.md", HX);
                },
                ours: |r| {
                    r.forget("docs/decisions/p.md");
                },
                expect: &[("docs/decisions/other.md", HX)],
            },
        ];

        // Every cell is exercised and its mismatch collected, so one red run names
        // *every* uncovered cell of the axis rather than only the first.
        let mut failures: Vec<String> = Vec::new();
        for cell in axis {
            let root = TempRoot::new("merge");

            let mut seed = FileStateRecord::new();
            for (path, hash) in cell.base {
                seed.record(*path, *hash);
            }
            seed.save(root.path()).expect("seed the shared record");

            // Writer A loads — and stashes the base it loaded.
            let mut ours = FileStateRecord::load(root.path()).expect("A loads");
            // Writer B loads, mutates and saves in between.
            let mut theirs = FileStateRecord::load(root.path()).expect("B loads");
            (cell.theirs)(&mut theirs);
            theirs.save(root.path()).expect("B saves");
            // A mutates its long-held copy and saves last.
            (cell.ours)(&mut ours);
            ours.save(root.path()).expect("A saves");

            let merged = FileStateRecord::load(root.path()).expect("the merged record loads");
            let expected: BTreeMap<String, String> = cell
                .expect
                .iter()
                .map(|(p, h)| ((*p).to_string(), (*h).to_string()))
                .collect();
            if merged.hashes != expected {
                failures.push(format!(
                    "[{}] expected {:?}, on disk {:?}",
                    cell.name, expected, merged.hashes,
                ));
            }
        }
        assert!(
            failures.is_empty(),
            "the later save must merge the other writer's delta, never discard it:\n{}",
            failures.join("\n"),
        );
    }

    /// A missing `file-state.json` is the first-encounter case: an empty record,
    /// never an error (`reconciliation.md` → Absent-hash is not drift).
    #[test]
    fn load_missing_record_is_empty_not_error() {
        let root = TempRoot::new("missing");
        let loaded = FileStateRecord::load(root.path()).expect("missing file loads empty");
        assert_eq!(loaded, FileStateRecord::new());
    }

    /// The staged-copy advisory (M43 A14): keyed at the repo-real destination
    /// (the file-path target form, value corrected), advisory severity, and an
    /// informational route — the shape [`crate::validate::validate_task`]'s staged
    /// loop emits for a persisted instance without touching the record.
    #[test]
    fn staged_copy_finding_is_an_informational_advisory_at_the_destination() {
        let f = staged_copy_finding("decisions/rate-limit.md");
        assert_eq!(f.code, "file-state.staged-copy");
        assert_eq!(f.severity, Severity::Advisory);
        assert!(
            f.message.contains("`decisions/rate-limit.md`"),
            "the message names the repo-real destination: {f:?}",
        );
        assert_eq!(
            f.location.as_ref().and_then(|l| l.address.as_deref()),
            Some("decisions/rate-limit.md"),
            "the target is the repo-real destination: {f:?}",
        );
        assert!(
            f.route
                .as_deref()
                .is_some_and(|r| r.starts_with("no action needed")),
            "an ignorable advisory says so in the first clause (the style guide): {f:?}",
        );
    }

    /// A doctype declaring `location:` **without** a trailing slash must key its
    /// file-state baseline at a path that exists on disk. The sweep reads through
    /// `repo_root.join(location).join(...)` (which inserts the separator) but keys the
    /// record by the concatenation `{location}{slug}.md`, so a slashless declaration
    /// used to record `findingsfindings-log.md` — a path that does not exist and never
    /// will, so the store sweep never matched the finalize-written baseline, re-adopted
    /// every run, and printed a nonexistent path at the reader (a law-1 lie on a
    /// permanent advisory). Every *shipped* schema spells the slash, which is why
    /// nothing hit it until M49 Increment 6 made project-authored packs shippable.
    ///
    /// The fix is the load-time normalization in
    /// [`crate::schema`](../schema/index.html) — this test drives the real authoring
    /// door (`load_schema_with_types` over a slashless `location:`), so it fails at the
    /// key the way an adopter's pack does, not at a hand-built `Schema`.
    #[test]
    fn a_slashless_location_keys_its_baseline_at_a_path_that_exists() {
        let yaml = String::from_utf8(ADR_YAML.to_vec())
            .expect("adr.yaml is utf-8")
            .replace("location: decisions/", "location: findings");
        let schema = crate::schema::load_schema_with_types(
            yaml.as_bytes(),
            &crate::schema::dev_pack_field_types(),
        )
        .expect("a slashless-location adr fixture loads");
        let mut schemas: std::collections::BTreeMap<String, Schema> =
            std::collections::BTreeMap::new();
        schemas.insert("adr".to_string(), schema);

        let root = TempRoot::new("slashless");
        let findings_dir = root.path().join("findings");
        std::fs::create_dir_all(&findings_dir).expect("mk findings/");
        std::fs::write(findings_dir.join("findings-log.md"), ADR_B_BASE)
            .expect("write the committed doc");

        let mut record = FileStateRecord::new();
        let mut index = EdgeIndex::default();
        let task = TempRoot::new("slashless-task");

        let findings = reconcile_committed_store(
            &mut record,
            &mut index,
            &schemas,
            root.path(),
            task.path(),
            &|_| true,
            &|_| true,
            &|_| None,
            &test_conflict(),
            &crate::validate::AdoptionInputs::inert(),
            &LiveRecord::none(),
        );

        let real = "findings/findings-log.md";
        assert_eq!(
            record.get(real),
            Some(hash_bytes(ADR_B_BASE.as_bytes()).as_str()),
            "the adopted baseline is keyed at the doc's real path: {:?}",
            record.hashes.keys().collect::<Vec<_>>()
        );
        assert!(
            root.path().join(real).exists(),
            "the recorded key names a path that exists on disk"
        );
        for key in record.hashes.keys() {
            assert!(
                root.path().join(key).exists(),
                "every recorded key names an existing path; `{key}` does not"
            );
        }
        // And the advisory the reader sees names that same real path — not the
        // glued `findingsfindings-log.md` the concatenation used to print.
        let adopt = findings
            .iter()
            .find(|f| f.code == "file-state.baseline-adopt")
            .expect("the fresh doc draws a baseline-adopt advisory");
        assert!(
            adopt.message.contains(real) && !adopt.message.contains("findingsfindings"),
            "the advisory names the real path: {adopt:?}"
        );

        // Idempotence — the symptom that made it permanent: a second sweep over an
        // unchanged store re-adopts nothing.
        let again = reconcile_committed_store(
            &mut record,
            &mut index,
            &schemas,
            root.path(),
            task.path(),
            &|_| true,
            &|_| true,
            &|_| None,
            &test_conflict(),
            &crate::validate::AdoptionInputs::inert(),
            &LiveRecord::none(),
        );
        assert!(
            !again.iter().any(|f| f.code == "file-state.baseline-adopt"),
            "the adopted baseline matches on the next sweep — no forever-re-adopt: {again:?}"
        );
    }

    /// **The base-pin backstop** (the rc.24 fix pass, `(R3, F7)`), over the whole `UNKNOWN`
    /// arm: `(on-disk bytes, touched, what the pin answers, whose conflict presentation)` →
    /// the codes emitted, and whether the bytes were recorded.
    ///
    /// Only one cell blocks — *touched, a blob at the pin, bytes that differ from it* — and
    /// it blocks a non-conformant edit too, ahead of the advisory that used to say *fix the
    /// file* one statement before the promote overwrote it. Every other cell is the arm as
    /// it was: untouched never asks the pin; bytes at the pin adopt; a pin with no blob
    /// adopts; and the migration task's own source keeps the arm whole, so its
    /// `jigc unmanage <source>` exit stays an exit.
    #[test]
    fn unknown_and_touched_is_decided_by_the_base_pin() {
        let schema = adr_schema();
        let base = ADR_B_BASE.as_bytes();
        let edited = ADR_B_EDITED_SUPERSEDES.as_bytes();
        let broken = ADR_B_EDITED_BAD_DATE.as_bytes();
        let at_base = |_: &str| Some(ADR_B_BASE.as_bytes().to_vec());
        let no_blob = |_: &str| None;
        let general = test_conflict();
        let migrating_this = ConflictBlock::task("migrate-it", Some(ADR_B_PATH));
        let migrating_other = ConflictBlock::task("migrate-it", Some("docs/elsewhere.md"));

        struct Cell<'a> {
            name: &'a str,
            bytes: &'a [u8],
            touched: bool,
            pinned: &'a crate::validate::PinnedBlob<'a>,
            conflict: &'a ConflictBlock,
            codes: &'a [(&'a str, Severity)],
            recorded: bool,
        }
        const ADOPT: (&str, Severity) = ("file-state.baseline-adopt", Severity::Advisory);
        const CONFLICT: (&str, Severity) = ("reconciliation.conflict-block", Severity::Blocking);
        const UNVETTED: (&str, Severity) = ("reconciliation.conformance-block", Severity::Advisory);
        let cells = [
            Cell {
                name: "touched, off the pin → the backstop blocks",
                bytes: edited,
                touched: true,
                pinned: &at_base,
                conflict: &general,
                codes: &[CONFLICT],
                recorded: false,
            },
            Cell {
                name: "touched, off the pin, non-conformant → blocks, never the advisory",
                bytes: broken,
                touched: true,
                pinned: &at_base,
                conflict: &general,
                codes: &[CONFLICT],
                recorded: false,
            },
            Cell {
                name: "touched, at the pin → adopts",
                bytes: base,
                touched: true,
                pinned: &at_base,
                conflict: &general,
                codes: &[ADOPT],
                recorded: true,
            },
            Cell {
                name: "touched, no blob at the pin → adopts",
                bytes: edited,
                touched: true,
                pinned: &no_blob,
                conflict: &general,
                codes: &[ADOPT],
                recorded: true,
            },
            Cell {
                name: "touched, no blob at the pin, non-conformant → the advisory, unrecorded",
                bytes: broken,
                touched: true,
                pinned: &no_blob,
                conflict: &general,
                codes: &[UNVETTED],
                recorded: false,
            },
            Cell {
                name: "untouched, off the pin → adopts (a first encounter)",
                bytes: edited,
                touched: false,
                pinned: &at_base,
                conflict: &general,
                codes: &[ADOPT],
                recorded: true,
            },
            Cell {
                name: "the migration source, off the pin → keeps the arm (the unmanage exit)",
                bytes: edited,
                touched: true,
                pinned: &at_base,
                conflict: &migrating_this,
                codes: &[ADOPT],
                recorded: true,
            },
            Cell {
                name: "a migration task's OTHER doc, off the pin → blocks",
                bytes: edited,
                touched: true,
                pinned: &at_base,
                conflict: &migrating_other,
                codes: &[CONFLICT],
                recorded: false,
            },
        ];
        for cell in cells {
            let mut record = FileStateRecord::new();
            let mut index = EdgeIndex::default();
            let findings = reconcile_committed(
                &mut record,
                &mut index,
                &schema,
                ADR_B_PATH,
                ADR_B_FROM,
                cell.bytes,
                cell.touched,
                cell.pinned,
                cell.conflict,
                &crate::validate::AdoptionInputs::inert(),
            );
            let got: Vec<(&str, Severity)> = findings
                .iter()
                .map(|f| (f.code.as_str(), f.severity))
                .collect();
            assert_eq!(got, cell.codes, "{}: {findings:?}", cell.name);
            assert_eq!(
                record.get(ADR_B_PATH).is_some(),
                cell.recorded,
                "{}: the record",
                cell.name,
            );
            assert!(
                index.edges.is_empty(),
                "{}: the `UNKNOWN` arm never touches the index",
                cell.name,
            );
        }

        // The block is the caller's own presentation, byte for byte — the same finding the
        // `DRIFTED + TOUCHED` arm raises, so one route serves both.
        let mut record = FileStateRecord::new();
        let findings = reconcile_committed(
            &mut record,
            &mut EdgeIndex::default(),
            &schema,
            ADR_B_PATH,
            ADR_B_FROM,
            edited,
            true,
            &at_base,
            &general,
            &crate::validate::AdoptionInputs::inert(),
        );
        assert_eq!(findings, vec![conflict_block_finding(ADR_B_PATH, &general)]);
    }

    /// The ADR home under a temp repo root, and the bytes written there.
    fn adr_at_home(root: &Path, body: &str) -> PathBuf {
        let home = root.join(ADR_B_PATH);
        std::fs::create_dir_all(home.parent().expect("the location dir")).expect("mk decisions/");
        std::fs::write(&home, body).expect("write the committed ADR");
        home
    }

    /// **The copy-in door** ([`read_for_copy_in`]; the rc.24 fix pass, `(R3, F7)`), over its
    /// four outcomes. The hash it records is of the **raw bytes read** — the fixture has no
    /// final newline, which the first-touch canonicalization would add, so a hash of the
    /// staged form would be a different one — and the body it returns is those same bytes.
    #[test]
    fn the_copy_in_door_records_an_absent_baseline_and_only_that() {
        let schema = adr_schema();
        let raw = ADR_B_BASE.trim_end_matches('\n');
        assert_ne!(
            hash_bytes(raw.as_bytes()),
            hash_bytes(crate::write::first_touch_canonicalize(raw).as_bytes()),
            "the premise: the copy-in canonicalizes this body",
        );

        // Absent key, conformant, nobody else holds it → adopted, durably, at the raw hash.
        let root = TempRoot::new("copy-in-adopt");
        let jigc_root = root.path().join(".jigc");
        let home = adr_at_home(root.path(), raw);
        let source = read_for_copy_in(&jigc_root, &home, ADR_B_PATH, &schema, || false)
            .expect("the copy-in read");
        assert_eq!(source.body, raw, "the bytes to stage are the bytes on disk");
        assert_eq!(source.baseline, CopyInBaseline::Adopted);
        assert_eq!(
            FileStateRecord::load(&jigc_root)
                .expect("load the record")
                .get(ADR_B_PATH),
            Some(hash_bytes(raw.as_bytes()).as_str()),
            "the recorded hash is the raw file's",
        );
        let said = source
            .baseline
            .finding(ADR_B_PATH)
            .expect("an adoption is stated");
        assert_eq!(said, baseline_adopt_finding(ADR_B_PATH));
        assert_eq!(said.severity, Severity::Advisory);

        // The key is held → nothing is written, even over bytes it does not name.
        std::fs::write(&home, ADR_B_EDITED_SUPERSEDES).expect("an out-of-band edit");
        let before = std::fs::read(FileStateRecord::path_in(&jigc_root)).expect("the record");
        let held = read_for_copy_in(&jigc_root, &home, ADR_B_PATH, &schema, || {
            panic!("a held key asks nothing about other tasks")
        })
        .expect("the copy-in read");
        assert_eq!(held.body, ADR_B_EDITED_SUPERSEDES);
        assert_eq!(held.baseline, CopyInBaseline::Held);
        assert_eq!(held.baseline.finding(ADR_B_PATH), None);
        assert_eq!(
            std::fs::read(FileStateRecord::path_in(&jigc_root)).expect("the record"),
            before,
            "a copy-in never moves a baseline that exists",
        );

        // Non-conformant → not baselined.
        let root = TempRoot::new("copy-in-nonconformant");
        let jigc_root = root.path().join(".jigc");
        let home = adr_at_home(root.path(), ADR_B_EDITED_BAD_DATE);
        let source = read_for_copy_in(&jigc_root, &home, ADR_B_PATH, &schema, || {
            panic!("a non-conformant doc is refused before the question is asked")
        })
        .expect("the copy-in read");
        assert_eq!(
            source.baseline,
            CopyInBaseline::Unrecorded(Unrecorded::NonConformant)
        );
        assert_eq!(
            source.body, ADR_B_EDITED_BAD_DATE,
            "the body is still handed over"
        );
        assert_eq!(source.baseline.finding(ADR_B_PATH), None);
        assert!(
            FileStateRecord::load(&jigc_root)
                .expect("load the record")
                .hashes
                .is_empty(),
            "a non-conformant doc is never baselined",
        );

        // Another open task holds it unrecorded → not baselined.
        let root = TempRoot::new("copy-in-elsewhere");
        let jigc_root = root.path().join(".jigc");
        let home = adr_at_home(root.path(), ADR_B_BASE);
        let source = read_for_copy_in(&jigc_root, &home, ADR_B_PATH, &schema, || true)
            .expect("the copy-in read");
        assert_eq!(
            source.baseline,
            CopyInBaseline::Unrecorded(Unrecorded::StagedElsewhere)
        );
        assert!(
            FileStateRecord::load(&jigc_root)
                .expect("load the record")
                .hashes
                .is_empty(),
            "the path stays `UNKNOWN` for the backstop to decide",
        );

        // An unreadable doc is the caller's error, with nothing recorded.
        let root = TempRoot::new("copy-in-missing");
        let jigc_root = root.path().join(".jigc");
        let missing = root.path().join(ADR_B_PATH);
        assert!(
            read_for_copy_in(&jigc_root, &missing, ADR_B_PATH, &schema, || false).is_err(),
            "a doc that cannot be read fails the copy-in",
        );
        assert!(!FileStateRecord::path_in(&jigc_root).exists());
    }

    /// **The key the copy-in records is the key the sweep reads** — for a `location:` doc
    /// and for a `placement:` one. Recorded under [`crate::finalize::promote_destination`],
    /// each doc reads `IN_SYNC` at the next sweep (no finding at all); under any other
    /// spelling it would read `UNKNOWN` and the record would be a silent no-op.
    #[test]
    fn the_copy_in_key_is_the_key_the_sweep_reads() {
        let foo = crate::schema::load_schema(
            b"\
type: foo
placement: { file: FOO.md }
sections: []
",
        )
        .expect("the placement schema loads");
        let mut schemas = adr_schemas();
        schemas.insert("foo".to_string(), foo);

        let root = TempRoot::new("copy-in-key");
        let jigc_root = root.path().join(".jigc");
        let task = TempRoot::new("copy-in-key-task");
        adr_at_home(root.path(), ADR_B_BASE);
        std::fs::write(root.path().join("FOO.md"), "# Foo\n").expect("write the placement doc");

        for (ty, slug) in [("adr", "distributed-cache"), ("foo", "foo")] {
            let schema = &schemas[ty];
            let key = crate::finalize::promote_destination(schema, slug).expect("a home");
            let home = crate::store::canonical_path(root.path(), schema, slug).expect("a home");
            let source = read_for_copy_in(&jigc_root, &home, &key, schema, || false)
                .expect("the copy-in read");
            assert_eq!(
                source.baseline,
                CopyInBaseline::Adopted,
                "{ty}: adopted at `{key}`"
            );
        }

        let mut record = FileStateRecord::load(&jigc_root).expect("load the record");
        let findings = reconcile_committed_store(
            &mut record,
            &mut EdgeIndex::default(),
            &schemas,
            root.path(),
            task.path(),
            &|_| true,
            &|_| true,
            &|_| None,
            &test_conflict(),
            &crate::validate::AdoptionInputs::inert(),
            &LiveRecord::none(),
        );
        assert!(
            findings.is_empty(),
            "both docs read in-sync under the keys the copy-in recorded: {findings:?}",
        );
    }
}
