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
//! axis never listed. Nobody owned the enumeration, so the populations disagreed about the same
//! cell in one binary: `ConfigLayerWorktree::restore`'s absent-pre-image arm deletes a file
//! **only while it still holds jigc's bytes**, while `rollback_record_pre_image`'s identical arm
//! deletes it unconditionally — so a third party who wrote at the record path during a rejected
//! `jigc milestone create` lost the file at exit 1, named by nothing.
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
//! **(1) Eight rows declare a discipline the source does not yet bind.** This module is the
//! increment's *first* task: the six `FileCas` rows that are not yet compare-and-swap (promote,
//! retire, the milestone record, the fan-out flip, `rename`'s unguarded arms, and the config
//! relocation, which has no rollback at all) and **both** `MintedSet` rows (still
//! `remove_dir_all`) are declarations of what the increment binds, not readings of what the
//! binary does today. **T10** is the task whose totality fence makes every row a checked fact
//! and strikes this paragraph. *(The plan's decomposition says "six"; it counted the `FileCas`
//! half. `unwind_mint` and `unwind_unrecorded_seeds` are equally unbound until T7, so the
//! honest number is eight — the datum is `unwind_mint`'s single `remove_dir_all`, which both
//! `MintedSet` rows reach and which no area set governs at HEAD.)*
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

use engine::state::WorkArea;

/// What keeps one population's restore from overwriting bytes that are not jigc's.
///
/// One per row: a population whose arms want two disciplines is two rows (see the module
/// header's note 2), because every fence over this registry reads the discipline *per row*.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Discipline {
    /// Compare-and-swap against the bytes jigc wrote: restore while they stand, park the
    /// pre-image and name both copies when they do not (`task.rs`'s `ConfigLayerWorktree`,
    /// the shipped M51 restore this increment generalizes).
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
            file: "crates/cli/src/task.rs",
            // `ConfigLayerWorktree::restore` — the scan names a method by its own name, and
            // this file declares exactly one `fn restore`.
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
            // The displaced-bytes rewrite, the `--source=HEAD` fallback for a tracked
            // destination with no capture, the new destination's removal, and the empty-parent
            // sweep that removal opens.
            restores: 4,
        },
        discipline: Discipline::FileCas,
    },
    Population {
        id: "retired-original",
        subject: "the bytes a retirement deleted, rewritten from the retire's own capture",
        doors: &[&["task", "finalize"]],
        site: Site::Source {
            file: "crates/cli/src/task.rs",
            unit: "rollback_promotions",
            restores: 1,
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
            restores: 2,
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
