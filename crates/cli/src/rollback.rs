//! **Every place jigc puts bytes back**, as one registry with a stated discipline per row —
//! M52 Increment 5 / T1 (`completions/artifacts/M52/settle-record.md` → **D1.1** as amended by
//! §1, §2, §14 and §19; [baseline-rollback.md](../../../completions/artifacts/M52/baseline-rollback.md)
//! §1; `design/finalize.md` → Rollback discipline).
//!
//! # The gap this closes
//!
//! The M51 per-axis review stated the class as **four** worktree-restore populations. Driven,
//! it is **nine**, of which exactly **one** — the config layer's — is compare-and-swap; five of
//! the nine sit outside the finalize executor entirely and are reached by doors the review's
//! axis never listed. Nobody owned the enumeration, so at the wave's base the populations
//! disagreed about the same cell in one binary: the config layer's absent-pre-image arm deleted
//! a file **only while it still held jigc's bytes**, while `rollback_record_pre_image`'s
//! identical arm deleted it unconditionally — so a third party who wrote at the record path
//! during a rejected `jigc milestone create` lost the file at exit 1, named by nothing. (Closed
//! at T4, which is what the `milestone-record` row's discipline now records rather than
//! declares.)
//!
//! A class nobody enumerates is a class a fix is cut short of, which is this wave's whole
//! subject. So the enumeration is code, and `crates/cli/tests/rollback_population_registry.rs`
//! is the reason it can be trusted: membership is a **counted source scan** over the production
//! restore units, in both directions — a restore site added without a row reddens, a row whose
//! site is deleted reddens.
//!
//! # The four disciplines
//!
//! Every row answers *what keeps this restore from overwriting a third party's bytes*:
//!
//!   * [`Discipline::FileCas`] — compare-and-swap: restore **only while the file still holds
//!     the bytes jigc wrote**; otherwise park the pre-image and name both copies with the
//!     door's `<door>.rollback-conflict`.
//!   * [`Discipline::MintedSet`] — nothing is compared because nothing is overwritten: the
//!     unwind removes **the door's own jigc-written area set** (`engine::state::WorkArea`,
//!     M52 Increment 4's registry) and then a non-recursive `remove_dir`, so a third party's
//!     file inside the area survives and is named rather than destroyed.
//!   * [`Discipline::DoorGuard`] — there is nothing to compare because the door **refused
//!     before the write**, or refused a state in which the restore could lose anything. The
//!     row names the guard's finding code, and the fence checks the code exists.
//!   * [`Discipline::Declared`] — no mechanism, by decision, with the reason **and the
//!     condition that reopens it** on the row itself (§19).
//!
//! # Two honest notes, both discharged by name
//!
//! **(1) Five rows declare a discipline the source does not yet bind.** This module was the
//! increment's *first* task, and each later task retires one of its declarations: the three
//! `FileCas` rows that are not yet compare-and-swap (the fan-out flip, `rename`'s unguarded
//! arms, and the config relocation, which has no rollback at all) and
//! **both** `MintedSet` rows (still `remove_dir_all`) are declarations of what the increment
//! binds, not readings of what the binary does today. **T10** is the task whose totality
//! fence makes every row a checked fact and strikes this paragraph. *(It read **eight** when
//! this module landed; **T3 bound `promote-destination` and `retired-original`** and **T4
//! bound `milestone-record`** — whose five doors now mint two door-keyed identities,
//! `milestone.rollback-conflict` and `task-discard.rollback-conflict` — so the
//! count moves with the source rather than standing as a stale number. The plan's
//! decomposition says "six" for the `FileCas` half alone; `unwind_mint` and
//! `unwind_unrecorded_seeds` are equally unbound until T7 — the datum is `unwind_mint`'s
//! single `remove_dir_all`, which both `MintedSet` rows reach and which no area set governs
//! at HEAD.)*
//!
//! **(2) The rows are twelve, and the plan's "ten" is struck with its arithmetic.** The plan
//! derives ten as *baseline §1 table A's nine worktree-restore populations + the rollback-less
//! `config set <root-knob>` relocation*, and that derivation is right about both halves. It
//! cannot, however, also carry the two `DoorGuard` classifications D1.1 states by name:
//!
//!   * `rollback_rename`'s HEAD-sourced `tracked_restore` arm is **inside** table A's row 6,
//!     whose other two arms the increment binds to `FileCas` — and a row carries **one**
//!     discipline, because T10's fence reads them per row (*"every `FileCas` row… every
//!     `DoorGuard` row…"*). Two disciplines, two rows.
//!   * `jigc setup`'s pre-write refusal is not in table A at all — it is baseline §1 group
//!     **C**, a declared no-rollback population — so the ten never had a slot for it.
//!
//! Twelve is 9 + 1 + 1 + 1, every addend cited. Nothing is dropped and nothing is invented.
//!
//! # The site-less rows are an enumerated exception, not a loophole
//!
//! Membership is the source scan, so a row with **no restore site** would be unfalsifiable.
//! Exactly two are admitted, each because the record names it: the `config set <root-knob>`
//! relocation (rollback-**less**, the defect T8 closes) and `jigc setup`'s install (guard-by-
//! refusal, M51 Increment 3, decided rather than unfinished). [`Site::NoRestore`] carries the
//! citation, and the fence asserts there are no others.

use std::path::{Path, PathBuf};

use engine::finding::{Finding, Location, Severity};
use engine::state::WorkArea;

/// What keeps one population's restore from overwriting bytes that are not jigc's.
///
/// One per row: a population whose arms want two disciplines is two rows (see the module
/// header's note 2), because every fence over this registry reads the discipline *per row*.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Discipline {
    /// Compare-and-swap against the bytes jigc wrote: restore while they stand, park the
    /// pre-image and name both copies when they do not ([`PreImageFamily`], the shipped M51
    /// restore generalized at T2 onto an entry every `FileCas` row can carry).
    FileCas,
    /// Remove **the door's own jigc-written area set** and then the directory, non-recursively
    /// — the rows whose subject is a minted working area rather than a file's pre-image. The
    /// sets are `engine::state`'s, never a hand-list: a hand-list drove short at
    /// `milestone create`, where the door writes two more files into the area *after* the mint.
    MintedSet(&'static [WorkArea]),
    /// No restore is needed because the door refused first. The payload is the guard's
    /// **finding code**, which the fence resolves in production source.
    DoorGuard(&'static str),
    /// No mechanism, by decision — with the condition that makes the reason false written
    /// beside it, so the row states its own reopening trigger.
    Declared {
        /// Why no mechanism is owed.
        reason: &'static str,
        /// The condition under which `reason` becomes false and the row changes discipline.
        reopens_when: &'static str,
    },
}

/// Where a population's restore lives — the coordinate the source scan fences membership
/// against.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Site {
    /// A production restore unit: a function whose name carries the restore family's
    /// vocabulary, or a `Drop` body that restores. `restores` is how many byte-restoring
    /// calls of that unit belong to **this** row — the counted half of the scan, and what
    /// makes a unit hosting two populations (`rollback_promotions`) fence each of them.
    Source {
        /// Workspace-relative source file.
        file: &'static str,
        /// The unit as the scan names it — `rollback_promotions`, or `<Type>::drop`.
        unit: &'static str,
        /// Byte-restoring calls in that unit owned by this row.
        restores: usize,
    },
    /// **No restore site in the source at all.** Admitted only for the two populations the
    /// record names; see the module header.
    NoRestore {
        /// Why there is none.
        reason: &'static str,
        /// Where that is decided — the citation a reader follows.
        cited: &'static str,
    },
}

/// One capture/restore population: what it puts back, which doors reach it, where its
/// restore lives, and the discipline that keeps it from destroying a third party's bytes.
#[derive(Clone, Copy, Debug)]
pub struct Population {
    /// Stable identity, used by every fence and finding over this registry.
    pub id: &'static str,
    /// The bytes this population puts back, in one line.
    pub subject: &'static str,
    /// The doors that reach it, each as its `crate::cli::VERB_KINDS` leaf path.
    pub doors: &'static [&'static [&'static str]],
    /// Where the restore is, or the stated reason there is none.
    pub site: Site,
    /// What keeps the restore honest.
    pub discipline: Discipline,
}

/// **The class**: every capture/restore population in the binary, one row each.
///
/// Ordered as the transaction meets them — the finalize executor's three families, then the
/// milestone record and the fan-out flip, then `rename`'s two arms, then the staged-write
/// rollback one layer down, then the two minted-area unwinds, and last the two rows that have
/// no restore site at all.
pub const ROLLBACK_POPULATIONS: &[Population] = &[
    Population {
        id: "config-layer-worktree",
        subject: "the `.jigc/.gitignore` amend and the `.jigc/version` stamp `finalize` \
                  rewrites in the worktree",
        doors: &[&["task", "finalize"], &["milestone", "finalize"]],
        site: Site::Source {
            // [`PreImageFamily::restore`], the generic compare-and-swap this module ships —
            // the scan names a method by its own name, and this file declares exactly one
            // `fn restore`. It moved here at T2: the shipped body was typed over the config
            // layer's own row type and hard-coded one door's code, noun and park name, so
            // what transfers to the other `FileCas` rows is the LOGIC, carried by a generic
            // entry with the door's metadata injected (`settle-record.md` → §1).
            file: "crates/cli/src/rollback.rs",
            unit: "restore",
            restores: 2,
        },
        // The one population that already is what this registry declares: the M51 restore
        // every other `FileCas` row is being generalized *from*.
        discipline: Discipline::FileCas,
    },
    Population {
        id: "promote-destination",
        subject: "the pre-promote bytes a promotion displaced at its destination, or the \
                  removal of a destination the promotion created",
        doors: &[&["task", "finalize"], &["milestone", "finalize"]],
        site: Site::Source {
            file: "crates/cli/src/task.rs",
            unit: "rollback_promotions",
            // Two, since T3 bound this row to the discipline it declares: the destination's
            // own restore — the displaced-bytes rewrite, and the removal of a destination the
            // promotion created — is the shared compare-and-swap at `rollback.rs::restore`,
            // counted once at the row whose site that unit is. What stays here is the pair of
            // arms keyed on *a destination this promotion created*: the `--source=HEAD`
            // fallback for one tracked at HEAD with no pre-image, and the empty-parent sweep
            // the removal opens. The fallback is not a pre-image population and could not be
            // one — its bytes come out of the object DB, so there is no third party's edit for
            // it to take.
            restores: 2,
        },
        discipline: Discipline::FileCas,
    },
    Population {
        id: "retired-original",
        subject: "the bytes a retirement deleted, rewritten from the retire's own capture \
                  while the path it deleted is still absent",
        doors: &[&["task", "finalize"]],
        site: Site::Source {
            file: "crates/cli/src/task.rs",
            unit: "rollback_promotions",
            // None of its own since T3: this population's whole restore is the shared
            // compare-and-swap at `rollback.rs::restore` — its post-image being *absence*
            // rather than bytes is a value, not a second code path — and those calls are
            // counted at `config-layer-worktree`'s row, the one whose site that unit is. What
            // remains of this population in this unit is its **index** arm, counted onto
            // `OTHER_AXIS_CALLS`. A row owning zero is still fenced: deleting it leaves this
            // unit's count unclaimed.
            restores: 0,
        },
        discipline: Discipline::FileCas,
    },
    Population {
        id: "milestone-record",
        subject: "the committed `milestone-record`'s pre-write image — rewritten, or the \
                  record deleted when the pre-image was absent",
        doors: &[
            &["milestone", "create"],
            &["milestone", "add-task"],
            &["milestone", "add-from-spec"],
            &["milestone", "discard"],
            // The fifth door `design/finalize.md`'s four-door enumeration omits: a sub-task
            // discard runs its own record-only commit (§19 — five doors through four capture
            // sites, `append_and_commit_record` serving two of them).
            &["task", "discard"],
        ],
        site: Site::Source {
            file: "crates/cli/src/milestone.rs",
            unit: "rollback_record_pre_image",
            // None of its own since T4: this population's whole worktree restore is the
            // shared compare-and-swap at `rollback.rs::restore`, counted once at the row
            // whose site that unit is. What stays in this unit is its **index** arm, and
            // that is the third axis's primitive — counted onto `NOT_A_POPULATION`'s
            // `rollback_owner_artifact_index` row, which is where it has always been. A row
            // owning zero is still fenced: deleting it leaves this unit's count unclaimed.
            restores: 0,
        },
        discipline: Discipline::FileCas,
    },
    Population {
        id: "fan-out-record-flip",
        subject: "the pre-flip record bytes the join's active → joined mutation replaced",
        doors: &[&["milestone", "finalize"]],
        site: Site::Source {
            file: "crates/cli/src/milestone.rs",
            // A destructor, not a function the family's vocabulary names — one of the two
            // restores a name-only grep is blind to.
            unit: "RecordFlipGuard::drop",
            restores: 1,
        },
        discipline: Discipline::FileCas,
    },
    Population {
        id: "rename-worktree",
        subject: "the landing path the rename wrote, and the gitignored \
                  `.jigc/state/file-state.json` it re-keyed",
        doors: &[&["rename"]],
        site: Site::Source {
            file: "crates/cli/src/rename.rs",
            unit: "rollback_rename",
            // The landing path's removal, and the file-state record's write-or-remove.
            restores: 3,
        },
        discipline: Discipline::FileCas,
    },
    Population {
        id: "rename-head-restore",
        subject: "the old doc and every structured referrer, restored from HEAD",
        doors: &[&["rename"]],
        site: Site::Source {
            file: "crates/cli/src/rename.rs",
            unit: "rollback_rename",
            restores: 1,
        },
        // Sharing a unit with `rename-worktree` and **not** its discipline, which is why the
        // two are two rows: this arm restores from HEAD, and the door refuses to run at all
        // over a dirty tree — so HEAD is what the worktree held, and there are no third-party
        // bytes for the restore to take.
        discipline: Discipline::DoorGuard("rename.dirty-tree"),
    },
    Population {
        id: "created-doc-staged-write",
        subject: "a staged instance a multi-step write created in the task working area, \
                  restored to what the create found",
        doors: &[&["doc", "author"]],
        site: Site::Source {
            file: "crates/engine/src/state.rs",
            unit: "rollback",
            restores: 2,
        },
        discipline: Discipline::Declared {
            reason: "intra-process — no subprocess runs in the window. The create → \
                     leaf-chain → rollback span spawns no git call and no hook, so the racer \
                     every other `FileCas` row exists for cannot enter it, and a \
                     `doc-author.rollback-conflict` would register a claim the binary can \
                     never make (settle-record §19, the human's Arm B).",
            reopens_when: "a door spawns a subprocess inside the create → leaf-chain → \
                           rollback span; the reason is then false and this row becomes \
                           `FileCas`.",
        },
    },
    Population {
        id: "milestone-mint-area",
        subject: "the working area this call minted, and `tasks.json`'s pre-append bytes",
        doors: &[&["milestone", "create"], &["milestone", "add-task"]],
        site: Site::Source {
            file: "crates/cli/src/milestone.rs",
            unit: "unwind_mint",
            restores: 2,
        },
        // Both rows, because one unwind serves two doors over two different areas — and the
        // `create` door writes `staged-snapshot.json` and `record-commit-msg.txt` into the
        // area *after* the mint, which is exactly what a hand-list of "the mint's own files"
        // drove short of.
        discipline: Discipline::MintedSet(&[WorkArea::Milestone, WorkArea::Task]),
    },
    Population {
        id: "unrecorded-seed-areas",
        subject: "each sub-task area seeded in a mid-loop failure the record never recorded",
        doors: &[&["milestone", "add-from-spec"]],
        site: Site::Source {
            file: "crates/cli/src/milestone.rs",
            unit: "unwind_unrecorded_seeds",
            // None of its own: it unwinds each seed through `unwind_mint`, so the calls are
            // counted there. A row owning zero is still fenced — deleting it leaves the unit
            // unclaimed.
            restores: 0,
        },
        discipline: Discipline::MintedSet(&[WorkArea::Task]),
    },
    Population {
        id: "config-root-relocation",
        subject: "the committed docs a `docs-root` / `placement-root` re-point moved to their \
                  new home before the knob write that justifies the move",
        doors: &[&["config", "set"]],
        site: Site::NoRestore {
            reason: "there is no rollback of any kind: the two move floors run before \
                     `write_scalar`, so a failed knob write leaves every `git mv` staged and \
                     every file-state key re-pointed against a knob that never landed. The \
                     row is in the registry *because* it is missing — an absent discipline \
                     that nothing enumerates is how this class stayed invisible.",
            cited: "settle-record.md → D1.5; baseline-rollback.md §1 table A's last row; \
                    built at M52 Increment 5 / T8",
        },
        discipline: Discipline::FileCas,
    },
    Population {
        id: "setup-install-path",
        subject: "the adopter's own bytes at a path `jigc setup` would install over",
        doors: &[&["setup"]],
        site: Site::NoRestore {
            reason: "the door refuses **before** the first write rather than restoring after \
                     one: staging a path and then refusing re-enacts the loss one step \
                     earlier, because `git add` on a worktree-equal-to-HEAD path replaces the \
                     adopter's staged blob. A restore here is not missing; it is declined.",
            cited: "M51 Increment 3 (`setup.dirty-install-path`); baseline-rollback.md §1 \
                    group C",
        },
        discipline: Discipline::DoorGuard("setup.dirty-install-path"),
    },
];

// ---------------------------------------------------------------------------
// The generic pre-image entry, and the compare-and-swap every `FileCas` row runs
// ---------------------------------------------------------------------------

/// **The door metadata a `FileCas` population injects** into the shared entry — M52
/// Increment 5 / T2 (`completions/artifacts/M52/settle-record.md` → **§1**, which struck
/// D1.2's *"the restore body transfers unchanged"*).
///
/// The **logic** transfers; the body could not, because the shipped one was typed over the
/// config layer's own row type, hard-coded `finalize.rollback-conflict` with *"this
/// finalize"* in its route, and parked by the **last path component** — so two populations
/// sharing a basename landed indistinguishably in one flat `.jigc/displaced/`. Those three
/// hard-codings are exactly this struct: the code, the noun the prose and the park
/// sub-directory are both named by, and the clause naming what the door's failure left
/// undone.
///
/// `noun` is deliberately one fact serving two places — `.jigc/displaced/<noun>/` and *"this
/// `<noun>`"* — because a door whose park directory and whose prose disagreed would make the
/// route name a path the reader cannot find.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ConflictDoor {
    /// The `<door>.rollback-conflict` identity this door's raced restores raise.
    pub code: &'static str,
    /// The door's noun: the park sub-directory under `.jigc/displaced/`, and the word the
    /// route prose uses (*"this finalize"*). One path component, so it can never escape the
    /// workbench — and **space-free**, because the route tells the reader to delete the
    /// parked copy at a path it names, and a path jigc itself chose with a space in it is
    /// the shape M51's `ShellUnsafeName` class exists to stop jigc minting. A door whose
    /// natural noun is two words is hyphenated here rather than spaced (`milestone-op`,
    /// `task-discard`).
    pub noun: &'static str,
    /// What the door's failure left undone, as the route's lead clause — *"nothing was
    /// committed"* at `finalize`.
    pub undone: &'static str,
}

/// `jigc task finalize` / `jigc milestone finalize` — the shipped M51 population's door, and
/// the first caller of the generic entry. Its code, severity, key form and route sentence are
/// unchanged by the generalization; only the park path moved.
pub const FINALIZE_DOOR: ConflictDoor = ConflictDoor {
    code: "finalize.rollback-conflict",
    noun: "finalize",
    undone: "nothing was committed",
};

/// **Every door that raises a `<door>.rollback-conflict`**, one row each.
///
/// It exists because the finding's code is now a *parameter* rather than a string literal
/// inside the constructor call: `crates/cli/tests/finalize_family_registry.rs`' producer scan
/// is a lexer, so it cannot see a code that arrives this way, and its stated remedy for that
/// shape is to complete the derived set **from the registry that decides membership** rather
/// than from a hand-written exception (the `COMMITTING_DOORS` precedent, M52 Increment 1).
/// A door added here whose identity lies in the `finalize.` namespace therefore grows that
/// expected set and reddens the family table, which is the property the literal shape bought
/// for free.
/// The **four milestone record-only doors** — `create` · `add-task` · `add-from-spec` ·
/// `discard` — which write the committed `milestone-record` and commit only it (M52
/// Increment 5 / T4).
///
/// One door value for four verbs, because the *subject* is one file and the operator's act
/// is one comparison: whichever milestone op raced, the two copies to reconcile are the
/// record as it now stands and the pre-image of the record this op rewrote. The
/// discrimination a reader needs is the **path**, which the finding is keyed at, not a
/// fourth spelling of `milestone` in the code.
pub const MILESTONE_DOOR: ConflictDoor = ConflictDoor {
    code: "milestone.rollback-conflict",
    noun: "milestone-op",
    undone: "nothing was committed",
};

/// `jigc task discard <sub-task>` — the **fifth** record-only door, and the one
/// `design/finalize.md`'s four-door enumeration never named (`settle-record.md` → §19).
///
/// It takes its own identity rather than the milestone one because the stable `(code,
/// target)` key is what a driver branches on: a raced sub-task discard and a raced
/// `milestone discard` rewrite the *same record file*, so sharing a code would make the two
/// indistinguishable at exactly the path where they collide.
pub const TASK_DISCARD_DOOR: ConflictDoor = ConflictDoor {
    code: "task-discard.rollback-conflict",
    noun: "task-discard",
    undone: "nothing was committed",
};

pub const ROLLBACK_DOORS: &[ConflictDoor] = &[FINALIZE_DOOR, MILESTONE_DOOR, TASK_DISCARD_DOOR];

/// One file's **worktree pre-image**: what it held before the transaction, and the exact
/// bytes jigc wrote over it.
///
/// It carries **four** values rather than a pre-image and a path, and each one is
/// load-bearing:
///
/// - `identity` — the population's own key, and the name the park is written under. A
///   repo-relative path wherever the population has one, because a *basename* does not
///   discriminate two files with the same name in different directories.
/// - `path` — the absolute path the writer actually writes, which is not derivable from the
///   identity: the config layer's two writers hang their files off **different roots**.
/// - `pre` — the bytes before the transaction, `None` when the file was **absent**. Absent
///   means absent, never *unreadable*: only `NotFound` yields `None`, because that value is
///   what makes the restore a **delete**, and swallowing a permission fault into it would
///   delete a file this run never created.
/// - `post` — **what jigc left here** ([`PostWrite`]). Not cosmetic: *"jigc wrote identical
///   bytes"*, *"jigc wrote nothing"* and *"jigc removed the file"* are three different facts,
///   and only the second may never be rolled back.
pub struct PreImage {
    identity: String,
    path: PathBuf,
    pre: Option<Vec<u8>>,
    post: PostWrite,
}

/// **What jigc left at a path** — the other half of the compare-and-swap, and the value the
/// live file is compared against.
///
/// The third variant is what makes a **deletion** a first-class write (M52 Increment 5 / T3):
/// the retire's rollback has the same question as every other `FileCas` row — *is what is
/// there now still what jigc left?* — except that what jigc left is **absence**. Modelled as
/// a missing post-image instead, a re-created path would read as *jigc wrote nothing here*
/// and the retire's own byte-capture would be dropped on the floor unnamed, which is exactly
/// the silent arm this row was built to close.
enum PostWrite {
    /// jigc wrote nothing at this path this run — an arm that never reached the write, a
    /// write that found nothing to change, or a read-back that failed. Never rolled back,
    /// however much the file has since changed.
    Untouched,
    /// The exact bytes jigc wrote.
    Bytes(Vec<u8>),
    /// jigc **removed** the file: what it left here is absence, and absence is what the swap
    /// compares against.
    Removed,
}

impl PostWrite {
    /// The bytes jigc left, `None` when what it left is **absence** — the shape both the
    /// `pre == post` no-op test and the live compare read, so a deletion and a write are one
    /// comparison rather than two branches. `None` for [`PostWrite::Untouched`] would say
    /// *jigc left absence here*, which is why that variant is answered before this is asked.
    fn left(&self) -> Option<&[u8]> {
        match self {
            PostWrite::Untouched | PostWrite::Removed => None,
            PostWrite::Bytes(bytes) => Some(bytes.as_slice()),
        }
    }
}

impl PreImage {
    /// Read the pre-image at `path` and key it under `identity`.
    ///
    /// `Err` on any read fault that is not `NotFound` — a transaction may not rewrite a file
    /// it cannot put back, and the caller is the one that knows how to say so about its own
    /// subject.
    pub fn capture(identity: impl Into<String>, path: PathBuf) -> std::io::Result<Self> {
        let pre = match std::fs::read(&path) {
            Ok(bytes) => Some(bytes),
            Err(err) if err.kind() == std::io::ErrorKind::NotFound => None,
            Err(err) => return Err(err),
        };
        Ok(Self {
            identity: identity.into(),
            path,
            pre,
            post: PostWrite::Untouched,
        })
    }

    /// The entry for a path jigc **removed**: `pre` is the bytes the caller read one statement
    /// before the unlink, and what jigc left is absence (M52 Increment 5 / T3, the retire row).
    ///
    /// The pre-image is handed in rather than read, because by the time a caller can say
    /// *"jigc removed this"* the bytes are gone — the read and the unlink are one step at the
    /// sink that owns them, and a second read here would find nothing.
    pub fn removed(identity: impl Into<String>, path: PathBuf, pre: Vec<u8>) -> Self {
        Self {
            identity: identity.into(),
            path,
            pre: Some(pre),
            post: PostWrite::Removed,
        }
    }

    /// The population's key for this entry.
    pub fn identity(&self) -> &str {
        &self.identity
    }

    /// Whether the file was **absent** before the transaction — so a restore of this entry is
    /// a *deletion*, and a caller with a second arm keyed on "the path jigc created" (the
    /// promote's HEAD-sourced fallback and its empty-parent sweep) can ask rather than keep a
    /// parallel list of its own.
    pub fn created(&self) -> bool {
        self.pre.is_none()
    }

    /// Record that jigc **just wrote** here, reading back the bytes it left.
    ///
    /// Called one statement after the write, which is what makes the recorded image jigc's
    /// own rather than a later reader's. A read that fails leaves `post` at
    /// [`PostWrite::Untouched`] — the entry is then treated as *never written* and the
    /// rollback leaves it alone: jigc cannot prove what it put there, and the safe direction
    /// is not to overwrite.
    pub fn wrote(&mut self) {
        self.post = match std::fs::read(&self.path) {
            Ok(bytes) => PostWrite::Bytes(bytes),
            Err(_) => PostWrite::Untouched,
        };
    }
}

/// **One `FileCas` population's entries plus the door they answer for** — captured before the
/// transaction's first write, restored **compare-and-swap** at its failure arm.
///
/// **Why compare-and-swap and not a rewrite** (M51 `settle-record.md` → §6, Codex 4): the
/// interval between jigc's write and the rollback runs arbitrary code — promotion,
/// retirement, staging, the user's own hooks. An unconditional restore over that interval
/// destroys a concurrent edit, *which is the same loss this family exists to prevent, in the
/// other direction*. So an entry restores **only while the file still holds the bytes jigc
/// wrote**; when it does not, nothing is overwritten — the pre-image is parked in the
/// gitignored `.jigc/displaced/<door>/` workbench and one blocking `<door>.rollback-conflict`
/// names both copies.
///
/// **An empty family is inert, and that is a property rather than a hope.** Unlike a rollback
/// keyed on a set difference — where an empty capture reads as *"the whole set was absent"* —
/// this one is keyed **per entry**: no entry, nothing restored. A capture that never ran and
/// a capture that found nothing are therefore the same safe value.
pub struct PreImageFamily {
    door: ConflictDoor,
    entries: Vec<PreImage>,
}

impl PreImageFamily {
    /// The value a caller holds **before** its capture point — no entries, so every later
    /// call is a no-op (see the type's note on inertness).
    pub fn empty(door: ConflictDoor) -> Self {
        Self {
            door,
            entries: Vec::new(),
        }
    }

    /// Add a captured entry.
    pub fn push(&mut self, entry: PreImage) {
        self.entries.push(entry);
    }

    /// The entry keyed `identity`, for a caller whose **own** arms depend on what the capture
    /// found — the promote's HEAD-sourced fallback and its empty-parent sweep both key on
    /// *"the destination jigc created"*, and reading that off the family keeps one answer
    /// where a parallel list beside it would be a second one that can disagree.
    pub fn entry(&self, identity: &str) -> Option<&PreImage> {
        self.entries.iter().find(|entry| entry.identity == identity)
    }

    /// Record that jigc just wrote the entry keyed `identity`. An identity this family does
    /// not carry is ignored — a write site may report a path the capture declared out.
    pub fn wrote(&mut self, identity: &str) {
        if let Some(entry) = self
            .entries
            .iter_mut()
            .find(|entry| entry.identity == identity)
        {
            entry.wrote();
        }
    }

    /// Restore the family on a failed transaction, compare-and-swap, returning one
    /// `<door>.rollback-conflict` per path whose bytes are no longer jigc's.
    ///
    /// Best-effort on the restore itself: the door's own failure is what the operator has to
    /// act on, so a write fault here must not replace it — the hook's stderr stays the
    /// correction signal.
    pub fn restore(&self, repo_root: &Path, jigc_root: &Path) -> Vec<Finding> {
        let mut conflicts = Vec::new();
        for entry in &self.entries {
            // jigc wrote nothing at this path this run (an arm that never reached the write,
            // a write that found nothing to change, a read-back that failed) — so there is
            // nothing to roll back, and nothing to report however much the file has changed.
            if matches!(entry.post, PostWrite::Untouched) {
                continue;
            }
            // What jigc left here — bytes, or absence where it removed the file. Both halves
            // of the comparison below are that one shape, so a deletion is the same swap as a
            // write rather than a second code path beside it.
            let post = entry.post.left();
            // jigc's write produced what was already there (identical bytes, or a removal of
            // a file that was already absent). Nothing changed, so nothing is restored — and
            // a concurrent edit here lost nothing to jigc, so it is not a conflict either.
            if entry.pre.as_deref() == post {
                continue;
            }
            let holds_jigcs_bytes = match std::fs::read(&entry.path) {
                Ok(bytes) => post == Some(bytes.as_slice()),
                // Absent now. That IS jigc's own act when jigc removed the file, and is
                // somebody else's removal when jigc wrote bytes here — in the second case the
                // swap fails and the pre-image is preserved rather than rewritten over a
                // deletion this transaction did not make.
                Err(err) if err.kind() == std::io::ErrorKind::NotFound => post.is_none(),
                // Unreadable is neither absent nor jigc's: a restore would overwrite bytes it
                // could not compare.
                Err(_) => false,
            };
            if holds_jigcs_bytes {
                match &entry.pre {
                    Some(bytes) => {
                        let _ = std::fs::write(&entry.path, bytes);
                    }
                    // The absent pre-image's restore is a DELETE, under the identical rule —
                    // jigc created the file, and a capture modelling only "present" would
                    // leave that creation behind.
                    None => {
                        let _ = std::fs::remove_file(&entry.path);
                    }
                }
                continue;
            }
            let parked = entry
                .pre
                .as_deref()
                .and_then(|bytes| park(jigc_root, self.door, &entry.identity, bytes));
            conflicts.push(rollback_conflict_finding(
                repo_root,
                self.door,
                entry,
                parked.as_deref(),
            ));
        }
        conflicts
    }
}

/// Park a conflicted pre-image in the **gitignored `.jigc/displaced/` workbench** — the home
/// the relocation arm already uses for bytes that must survive without becoming committable
/// (`crate::relocate`'s `WORKBENCH_SUBDIR`, a `crate::gitignore::ENTRIES` member).
///
/// The name is `<door noun>/<identity>.pre-image.<nanos>`, and each of the three parts is
/// there for a reason the flat `<basename>.pre-image.<nanos>` it replaces could not serve:
/// the **door** separates populations that would otherwise share one directory, the
/// **identity** keeps its directory structure so two files with the same name stay
/// distinguishable and each parked copy says which live path it came from, and the **nanos**
/// keeps a second refused run from overwriting the copy that is now the only one of those
/// bytes.
///
/// Only [`std::path::Component::Normal`] components of the identity are walked, so an
/// identity that is absolute or climbs with `..` parks nowhere rather than writing outside
/// the workbench — `None`, and the finding then says the copy is gone rather than naming a
/// path that is not there. The same `None` covers a write that simply failed.
fn park(jigc_root: &Path, door: ConflictDoor, identity: &str, bytes: &[u8]) -> Option<PathBuf> {
    let relative = Path::new(identity);
    let name = relative.file_name()?;
    let mut dir = jigc_root.join("displaced").join(door.noun);
    for component in relative.parent()?.components() {
        match component {
            std::path::Component::Normal(part) => dir.push(part),
            _ => return None,
        }
    }
    std::fs::create_dir_all(&dir).ok()?;
    let parked = dir.join(format!(
        "{}.pre-image.{}",
        name.to_string_lossy(),
        engine::tempname::unique_nanos(),
    ));
    std::fs::write(&parked, bytes).ok()?;
    Some(parked)
}

/// The refusal a **raced rollback** raises: jigc rewrote this file inside the transaction, the
/// transaction then failed, and by the time the rollback ran the bytes on disk were no longer
/// the ones jigc wrote (M51 `settle-record.md` → §6; `design/validation.md` → the M51
/// registrations).
///
/// On §10's mold — blocking, a [`engine::finding::Route::human`], exit 1 — because no `jigc`
/// argv reconciles two versions of a file a human co-owns; the act is a comparison only they
/// can make. It **keys at the file path** ([`crate::render::FinalizeSubject::FilePath`]), the
/// form that discriminates: two raced paths in one rollback are two findings, not one
/// `(code, null)`.
///
/// It is printed **beside** the door's own frame and never in place of it
/// ([`crate::task::carry_rollback_conflicts`]): the transaction's failure is still whatever
/// failed it, and a hook's stderr stays verbatim and unwrapped (`design/finalize.md` →
/// 6. Commit).
fn rollback_conflict_finding(
    repo_root: &Path,
    door: ConflictDoor,
    entry: &PreImage,
    parked: Option<&Path>,
) -> Finding {
    let live = crate::render::repo_relative(repo_root, &entry.path);
    let noun = door.noun;
    let both = match (parked, entry.pre.is_some()) {
        (Some(parked), _) => format!(
            "both versions are on disk: the file as it now stands at `{live}`, and this \
             {noun}'s pre-image at `{}`. Compare them, keep what you want, and delete the \
             parked copy",
            crate::render::repo_relative(repo_root, parked),
        ),
        (None, true) => format!(
            "the file as it now stands is at `{live}`; jigc could not park a copy of its \
             pre-image, so that version is gone. Recover it from git if the path is tracked"
        ),
        (None, false) => format!(
            "`{live}` did not exist before this {noun}, so the rollback would have deleted \
             the copy jigc created — it did not. Remove it by hand if you do not want it"
        ),
    };
    // What jigc left here, said in the operator's terms — because a retirement's rollback is
    // as raced as a write's and *"the bytes on disk are not the ones jigc wrote"* would be a
    // law-1 lie at a path jigc wrote no bytes to at all: it removed one.
    let left = match entry.post {
        PostWrite::Removed => "jigc removed it and something has since put a file back",
        PostWrite::Untouched | PostWrite::Bytes(_) => {
            "the bytes on disk are not the ones jigc wrote"
        }
    };
    Finding::graded(
        Severity::Blocking,
        door.code,
        format!(
            "`{live}` changed while this {noun} was running, so the rollback did not restore \
             it: {left}"
        ),
        Some(Location::addressed(live.clone(), 1, 1)),
        Some(engine::finding::Route::human(format!(
            "{} and {both}",
            door.undone,
        ))),
    )
}
