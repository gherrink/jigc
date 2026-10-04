//! Task/staging state, the per-task working area, base pinning, and the
//! `finalize` transaction.
//!
//! See `design/write-commands.md` (task origination, staging), `design/storage.md`
//! (`.jigc/` layout), and `design/finalize.md` (the seven phases).
//!
//! This module lands **task origination** (`write-commands.md` → Task
//! origination; `storage.md` → The per-task working area / base pinning): minting
//! a task slugs the intent into a frozen content-slug ([`crate::slug::slugify`],
//! empty → the type name), opens the gitignored working area at
//! `.jigc/tasks/<id>/`, and writes a **base-pin** file recording the commit the
//! task started against (full + short SHA). A serial collision — an active task
//! dir of that id already exists — **rejects** (`write-commands.md` → Task-id
//! collision & resume: *never silently suffixed, never silently reused*), surfacing
//! the existing task's status as a routed blocking [`Finding`]; nothing new is
//! created. The numeric `-2`/`-3` suffix is reserved for the post-MVP parallel
//! `fan-out`/`join` case only.
//!
//! The base SHA is supplied by the caller (the CLI reads HEAD via `git rev-parse`
//! — "CLI orchestrates, git executes"); the engine performs no I/O beyond the
//! working-area filesystem and never shells out, so minting is a pure function of
//! (jigc-root, intent, type-name, workflow-id, base) → on-disk effect,
//! golden-testable.

use crate::finding::{Finding, Location, Severity};
use crate::schema::Schema;
use crate::write::{self, Instance, SectionContent};
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

/// The base-pin filename inside a task's working area.
const BASE_PIN_FILE: &str = "base.json";

/// The bound-roles filename inside a task's working area (`DECISIONS.md`
/// 2026-05-31 → inc-5 `as:` role binding at create: the bound context roles live
/// in `.jigc/tasks/<id>/roles.json`, read back on resume).
const ROLES_FILE: &str = "roles.json";

/// The rename-log filename inside a task's working area — the durable record of the
/// **pre-rename titles** this task has moved a staged doc away from
/// ([`RenameRecord`]).
const RENAMES_FILE: &str = "renames.json";

/// The intent filename inside a task's working area — the original human intent
/// the task was minted from, persisted verbatim so `jigc start --task <id>`
/// resume re-composes with the same `{{task.intent}}` (the id is a *lossy* slug
/// of the intent, so the intent itself must be stored to survive a resume;
/// `storage.md` → The per-task working area: "it persists on disk across
/// sessions, so work resumes"). Plain text, no trailing-newline normalization
/// (read back byte-for-byte).
const INTENT_FILE: &str = "intent";

/// The minting-workflow filename inside a task's working area — the id of the
/// workflow the task was minted from (`single-task`, `quick-fix`, …), persisted
/// verbatim so `jigc start --task <id>` resume composes the task's **own**
/// workflow, never the cascade default (`DECISIONS.md` 2026-06-01 → M2 Increment 3
/// re-cut: persist the minting workflow id at mint; resume composes that; a
/// missing id is a clear error). Plain text, the same diff-friendly style as
/// `intent` (read back byte-for-byte).
const WORKFLOW_FILE: &str = "workflow";

/// The working-area file that makes a task an **amend** task (`jigc task amend`, F-10) —
/// the marker `finalize` reads to take its second commit model, `git commit --amend` with
/// the committed tree untouched, instead of the ordinary stage-and-commit
/// (`design/finalize.md` → The amend arm).
///
/// It holds the **full sha of the commit the amend was minted against**, which is also what
/// `base.json` pins. The duplication is deliberate and it is not redundancy: `base.json`'s
/// pin answers *what has this task been working from*, and every task has one, so a
/// marker keyed on the pin's presence would make every task an amend. The marker's own
/// content is what the finalize arm re-reads to name the superseded sha in its ack and its
/// `amended` envelope key after `git commit --amend` has already moved HEAD — by which
/// point neither HEAD nor git carries it anywhere an ack can reach without a reflog parse.
///
/// Absent on every other task, so the ordinary commit model is the default and no existing
/// task's behaviour moves. Public so the CLI `task amend` verb writes it under the same
/// name the engine-side reader ([`read_amend_pin`]) takes.
pub const AMEND_PIN_FILE: &str = "amend";

/// The working-area file recording a migration task's repo-relative foreign source
/// path (`jigc migrate <path>`), read back at finalize to retire the foreign original
/// (`design/auto-migration.md` → Retire-the-foreign-original). Absent on every
/// non-migration task — the retire set is then empty. Public so the CLI `migrate`
/// verb writes it under the same name the engine planner reads
/// ([`read_migration_source`]).
pub const SOURCE_PATH_FILE: &str = "source-path";

/// The working-area file recording a migration task's `--slug` override
/// (`jigc migrate <path> --as <doctype> --slug <s>`), read back by `doc author`
/// to drive the created target doc's id verbatim (the gated-create slug
/// override — M43 Inc 6 T2). Absent on a slug-less migrate and on every
/// non-migration task — the author then derives the slug from the payload
/// title, byte-identical to before. Public so the CLI `migrate` verb writes it
/// under the same name the author read-back uses ([`read_slug_override`]).
pub const SLUG_OVERRIDE_FILE: &str = "slug-override";

/// The working-area file a migration task's **foreign source bytes** are staged at
/// (`jigc migrate <path>`) — the read-only source artifact the compose-time source seam
/// surfaces. Plain bytes, read back verbatim (the same diff-friendly style as `intent` and
/// `workflow`).
///
/// It is declared here rather than beside the CLI verb that writes it because it is a
/// member of [`TASK_AREA_FILES`], and a registry whose members are declared in two crates
/// is a registry one crate can be wrong about (M52 Increment 4 / T2).
pub const SOURCE_FILE: &str = "source";

/// The working-area file the finalize transaction renders the commit message into before
/// handing it to `git commit -F` (`design/finalize.md` → 6. Commit). Written at the top of
/// `cli::task::try_execute_finalize_plan` and removed one statement after the commit returns,
/// so on every ordinary path it is gone before the teardown ever reads the area.
///
/// **It is a member of BOTH rows, and it is on them because *ordinary* is not the axis
/// membership is decided on** (M53 completion audit, fix 1). `msg_tmp_dir` **is**
/// `cleanup_dir` at all three of that executor's call sites — the task door's own area, and
/// the milestone area at both boundary arms — and its removal is best-effort
/// (`let _ = std::fs::remove_file(…)`). One in-transaction fault defeats it: a `pre-commit`
/// hook that `chmod 0555`s the area (M53 Increment 2's own racer) makes the removal *and* the
/// teardown fail, and the transient then sits in the complement. Driven at `f664863a`, that
/// made `jigc task finalize` land at exit 0 printing a `finalize.foreign-bytes` advisory that
/// named jigc's own transient as *a path jigc did not write*, and made every later
/// `jigc task discard` / `jigc uninstall` refuse at **exit 1** over it, demanding `--force`.
///
/// That is G-13's failure mode a second time, and for the reason [`TASK_AREA_FILES`] already
/// records it the first (`record-commit-msg.txt`): a name whose *ordinary* lifetime ends
/// inside the transaction still outlives it on the arm where the transaction faults, and the
/// registry is read by doors that run **after** that arm. M53 planning disposed this name on
/// the removal statement (`DECISIONS.md` → 2026-09-16, and this row's own struck reason in
/// `crates/cli/tests/task_area_writer_registry.rs` → `NON_AREA_JOINS`); the fault model
/// falsifies the disposition, not the statement.
///
/// It is declared here rather than beside the CLI verb that writes it for [`SOURCE_FILE`]'s
/// reason — a registry whose members are declared in two crates is a registry one crate can
/// be wrong about.
pub const FINALIZE_MESSAGE_FILE: &str = "finalize-message.tmp";

/// The working-area sub-directory holding a task's staged doc instances
/// (`DECISIONS.md` 2026-05-31 → Task working-area on-disk layout: a staged instance
/// lives at `.jigc/tasks/<id>/docs/<type>:<slug>.md`).
pub(crate) const DOCS_DIR: &str = "docs";

/// The bare filename of a staged doc instance: the `:`-joined address slug
/// (`<type>:<slug>.md`), the on-disk form the working-area layout pins
/// (`DECISIONS.md` 2026-05-31 → Task working-area on-disk layout).
fn instance_filename(type_name: &str, slug: &str) -> String {
    format!("{type_name}:{slug}.md")
}

/// **Everything jigc itself writes into a task's working area**, in one home both crates
/// read — the registry whose *complement* is the subject of every door that destroys one
/// (`design/team-ready-state.md` → The working area's two populations).
///
/// Before M52 Increment 4 nobody owned this set: thirteen filename constants sat scattered
/// across three modules, eleven of them private, and the doors that remove a working area
/// took the whole directory. A guard cut from a hand-listed subset is worse than no guard —
/// the list drove short of `renames.json` (written at the ordinary `jigc doc rename --task`
/// door) and of the two doc↔code probe snapshots, so it would have called three of jigc's
/// own files foreign and refused every task that had renamed a doc.
///
/// **The members are the declarations themselves, never copies of their values**, so the
/// registry cannot drift from the writer; and `crates/cli/tests/task_area_writer_registry.rs`
/// counts the production `<…dir>.join(<name>)` sites in **both** crates against it, so a
/// fourteenth writer reddens rather than silently joining the complement.
///
/// **The `docs/` member is a tree, not a file** — its own rule is [`TASK_DOCS_FILES`].
///
/// **`record-commit-msg.txt` is on BOTH rows** (M52 Increment 4 / T5). It reads as a
/// milestone-area name and it is one — and `jigc task discard <sub-task-id>` also writes it
/// into the **task** area, because a sub-task discard runs its own record-only commit and
/// hands that door's `msg_dir` the task dir. On the ordinary path it is gone a moment later
/// with the area; on a **hook-rejected** discard the area survives *with the file in it*, and
/// that is a state jigc's own survivable frame promises is re-runnable. Missing from this row
/// it made the re-run refuse over jigc's own file — G-13's exact failure mode, found by
/// driving (`flow47_acceptance::every_committing_door_leaves_the_repo_recoverable`) rather
/// than by the source fence, which counts names against the **union** of the two rows and so
/// cannot see a member on the wrong one.
///
/// **`finalize-message.tmp` is on BOTH rows too** (M53 completion audit, fix 1), and for the
/// same shape of reason one layer over: it is jigc's own transient, it is *ordinarily* gone
/// before any door reads the area, and the arm where it is not is exactly the arm a door then
/// answers for. [`FINALIZE_MESSAGE_FILE`] carries the driven cell.
/// **`base.json` stays at index 0** ([`WorkArea::base_pin`] reads the row's first member),
/// so the rest of the row is alphabetical by constant name and [`AMEND_PIN_FILE`] sits
/// second rather than first. The order is a lookup convenience everywhere else; only member
/// 0 is load-bearing, and its own rule is fenced by
/// [`tests::both_area_rows_lead_with_the_base_pin`].
pub const TASK_AREA_FILES: &[&str] = &[
    BASE_PIN_FILE,
    AMEND_PIN_FILE,
    DOCS_DIR,
    FINALIZE_MESSAGE_FILE,
    INTENT_FILE,
    RENAMES_FILE,
    ROLES_FILE,
    SLUG_OVERRIDE_FILE,
    SOURCE_FILE,
    SOURCE_PATH_FILE,
    STAGED_SNAPSHOT_FILE,
    WORKFLOW_FILE,
    crate::milestone::RECORD_COMMIT_MSG_FILE,
    crate::validate::BASE_SNAPSHOT_FILE,
    crate::validate::SNAPSHOT_FILE,
];

/// **Jigc's own set inside a task area's `docs/`**: the provenance manifest, plus the
/// staged doc bodies — every entry whose name [`staged_doc_id`] reads as an identity
/// *and* whose shape is a regular file. Nothing else under `docs/` is jigc's.
///
/// The bodies cannot be listed here because their names are the task's own staged
/// addresses, so the rule is carried by [`foreign_area_paths`] rather than by a constant;
/// this names the one entry that is fixed.
///
/// **Why the instance-name set and not `provenance.json`'s key set**, which is the more
/// precise answer to *"did jigc stage this?"*: a stager that failed to record provenance
/// would make every doc it staged foreign, and the doors this feeds *destroy* or *move* the
/// complement — so the exact discriminator's failure mode is losing authored prose, and the
/// looser one's is keeping a file too many. The name rule is [`staged_doc_id`], and it is
/// the writer's own form rather than *any* `.md`: see that function for the cell where
/// *any* `.md` was the loss it was chosen to avoid.
pub const TASK_DOCS_FILES: &[&str] = &[PROVENANCE_FILE];

/// **Everything jigc itself writes into a milestone's working area** — [`TASK_AREA_FILES`]'
/// sibling row, over `.jigc/milestones/<id>/` (`settle-record.md` → D3.1, §6).
///
/// **The `merged/` member is a tree, not a file** — its own rule is in
/// [`foreign_area_paths`]' milestone arm, and [`unwind_merged`] removes by the identical
/// walk: under `merged/`, jigc's set is the `docs/` directory
/// [`crate::milestone::materialize`] writes, and inside that it is every entry
/// [`staged_doc_id`] recognises. Nothing else.
///
/// It shipped from M52 as *"jigc's wholesale, and nothing inside it is walked"*, which is an
/// exclusion resting on the predicate **nothing but jigc's bytes are in `merged/`**. That
/// predicate is false four ways (M53 Increment 1 / T2; `settle-record.md` → D1.2 as amended
/// by §2): a finalize blocked at exit 3 leaves the area standing with its materialized bodies
/// in it; a succeeding `pre-commit` hook writes into the area during the commit (the writer
/// M52 recorded at [`staged_doc_id`]); `materialize` took an editor `.swp` on a finalize that
/// committed nothing; and the carve-out **defeated the consent gate** at the two doors that
/// have one — driven, `jigc milestone discard` with no `--force` over `merged/top.txt` and
/// `merged/docs/deep.txt` exited 0, printed *"workbench removed"* and took both, while the
/// identical byte at the area **root** refused with `milestone.foreign-bytes`. One state, two
/// answers, from one carve-out.
///
/// **`finalize-message.tmp` is on this row for the same reason it is on the task row**: the
/// shared finalize executor's `msg_tmp_dir` is the *milestone* area at both boundary arms, so
/// the transient lands here too and survives here too on the arm where the transaction faults
/// ([`FINALIZE_MESSAGE_FILE`]).
pub const MILESTONE_AREA_FILES: &[&str] = &[
    BASE_PIN_FILE,
    FINALIZE_MESSAGE_FILE,
    STAGED_SNAPSHOT_FILE,
    crate::milestone::MERGED_AREA,
    crate::milestone::RECORD_COMMIT_MSG_FILE,
    crate::milestone::TASKS_FILE,
];

/// Which working area a path is — the two rows of the writer registry.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WorkArea {
    /// `.jigc/tasks/<task-id>/` — [`TASK_AREA_FILES`] plus the `docs/` tree rule.
    Task,
    /// `.jigc/milestones/<milestone-id>/` — [`MILESTONE_AREA_FILES`].
    Milestone,
}

impl WorkArea {
    /// The registry row for this area — the names jigc writes at its root.
    pub fn jigc_written(self) -> &'static [&'static str] {
        match self {
            WorkArea::Task => TASK_AREA_FILES,
            WorkArea::Milestone => MILESTONE_AREA_FILES,
        }
    }

    /// The row's **base-pin** member — the file whose presence makes a directory a work
    /// unit ([`carries_base_pin`]).
    ///
    /// Read off the registry's member 0 rather than spelled out, so this and
    /// [`unwind_area`]'s first removal cannot drift: the pin is what the unwind takes
    /// first, which is what makes a half-unwound area a residual rather than a work unit
    /// jigc could not tear down. `crates/cli/tests/mint_door_base_pin.rs` and this module's
    /// own [`tests::both_area_rows_lead_with_the_base_pin`] hold the position.
    fn base_pin(self) -> &'static str {
        self.jigc_written()[0]
    }

    /// Whether `name` is the row's **directory** member — the one entry whose shape is a
    /// tree rather than a file, so a plain file wearing that name is not jigc's.
    fn tree_member(self, name: &str) -> bool {
        match self {
            WorkArea::Task => name == DOCS_DIR,
            WorkArea::Milestone => name == crate::milestone::MERGED_AREA,
        }
    }
}

/// **Is `area` a work unit at all?** — the residual rule, in one home
/// (`completions/artifacts/M53/settle-record.md` → D3): a directory under `.jigc/tasks/`
/// (or `.jigc/milestones/`) is a task, or a milestone, **iff it carries its base pin**.
/// Everything else living there is a *residual*: a leftover a faulted teardown left, or a
/// directory somebody made.
///
/// Until M53 the question was `is_dir()`, so a bare `mkdir .jigc/tasks/anything` was an
/// active task at every door that enumerates or resolves one — listed by `jigc task list`,
/// rendered by `jigc start` as work in progress with a resume route that then failed, and
/// counted by the mid-fan-out guard that refuses `jigc rename` while work is live.
///
/// **Existence, never a parse.** [`mint_task`] writes the pin with a plain `fs::write`, so
/// a predicate keyed on the pin's *contents* would have a torn-read window the milestone
/// side (which persists atomically) does not, and would answer *no such task* over a live
/// one on a bad read. A legitimate area whose pin is torn, corrupt or unreadable therefore
/// passes this and fails at the read, where the producers that report exactly that still
/// live — deliberately kept (`settle-record.md` → §9).
///
/// **Shape is part of it**, read through `symlink_metadata` so a symlink is not followed:
/// jigc writes the pin as a regular file, and [`unwind_area`] leaves an entry it did not
/// write standing — including a *directory* named `base.json`, which an existence-keyed
/// predicate would call a live task for ever.
///
/// **The converse is the load-bearing half**, and it is fenced rather than assumed: every
/// production mint door writes the pin as its first content write, immediately after
/// `create_dir_all`, so no legitimate area is pin-less at a moment a door can observe it
/// (`crates/cli/tests/mint_door_base_pin.rs`, per [`MINT_DOORS`] row). The remaining window
/// — between those two syscalls — narrows rather than closes, and flickers in the safe
/// direction: a just-minted area is briefly absent from the roster, never a settled one
/// briefly present.
#[must_use]
pub fn carries_base_pin(area: &Path, kind: WorkArea) -> bool {
    std::fs::symlink_metadata(area.join(kind.base_pin())).is_ok_and(|shape| shape.is_file())
}

/// **What a residual is**, as the one home every door that names one reads it from
/// (`settle-record.md` → D3, *What a by-id door answers* + *What the mint answers*).
///
/// `listed` is the area, already rendered repo-relative; `unit_noun` is the kind of work
/// unit the *calling door* was asked about — `task` at the by-id seams and at
/// [`mint_task`], `milestone` at [`crate::milestone::mint_milestone`]. Only that noun
/// varies: *leftover and not a work unit* is true of both, and the two causes are the same
/// two causes.
///
/// It is a shared **fragment** rather than a shared finding because the three doors differ
/// in what they were asked — *there is no task `<id>`* is not *`<id>` cannot be minted* —
/// while what they found is one thing. Sharing the finding would force one message to
/// answer two questions; sharing nothing would put three spellings of one condition on the
/// surface, which is the failure `settle-record.md` → §5 names.
#[must_use]
pub fn residual_area_note(listed: &str, unit_noun: &str) -> String {
    format!(
        "`{listed}` is a directory carrying no base pin, so it is a leftover and not a \
         work unit — either jigc never minted a {unit_noun} there, or a teardown stopped \
         partway and left the directory behind"
    )
}

/// **The one recovery a residual has** — byte-identical at every door that names one,
/// because there is exactly one act, and a second spelling would make one state answer two
/// ways (`settle-record.md` → §5's rule, applied to this family).
///
/// It names a path and **no command**. A mechanical route here would have to be a jigc verb
/// that deletes a directory jigc did not fill, and D3's standing bound is that no verb
/// resolves a residual: what is in there is not jigc's to judge. `nothing was changed` is
/// true at all three doors — the by-id seams refuse before they resolve, and both mints
/// refuse before `create_dir_all`.
#[must_use]
pub fn residual_area_route(listed: &str) -> crate::finding::Route {
    crate::finding::Route::human(format!(
        "nothing was changed. Keep anything you need from `{listed}` and delete the \
         rest by hand — jigc mints no verb that clears a leftover working area, because \
         what is in there is not jigc's to judge"
    ))
}

/// The **repo-relative** spelling of a working area, for a door that holds the `.jigc/`
/// root rather than the checkout it hangs off.
///
/// [`crate::path::repo_relative`] renders against the repository root, and the mints take
/// `jigc_root` (= `<jigc_home>/.jigc`), so the home is its parent — which is **jigc_home**,
/// the main checkout, never the worktree a fan-out sub-agent runs in. Taking the worktree
/// root would print a host-absolute path at every door reached from a fan-out, the law-1
/// breach T3 closed on the by-id side.
pub(crate) fn area_repo_path(jigc_root: &Path, area: &Path) -> String {
    crate::path::repo_relative(jigc_root.parent().unwrap_or(jigc_root), area)
}

/// The staged-doc identity a working-area `docs/` entry **name** stands for — the inverse
/// of [`instance_filename`], and the one home that rule has.
///
/// **It is the inverse, not `.md`-anything** (M52 Increment 5, the audit fix). Every site
/// that writes a staged body goes through [`instance_path`], which composes
/// `<type>:<slug>.md` — so the `:` is not decoration, it is what makes a name one jigc's own
/// writer could have produced, and `engine::finalize`'s two enumerations (`plan_promotions`,
/// `plan_owner_artifacts`) have always required it before treating a staged file as an
/// instance. This asked `strip_suffix(".md")` alone, so *every* `.md` under `docs/` was
/// jigc's — and this rule is the **destroying** doors' subject, not only a read probe's:
/// driven at `6f4a975a`, a `pre-commit` hook that wrote `docs/agent-notes.md` into the area
/// a `jigc milestone add-task` had just minted had that file removed by [`unwind_docs`] at
/// exit 1, out of a gitignored tree with no second copy, while the identical bytes at the
/// area *root* survived and were named — one run, two answers, because the root's membership
/// is the registry row and `docs/`'s was this predicate.
///
/// The narrowing keeps the reason the rule is **not** `provenance.json`'s key set
/// ([`TASK_DOCS_FILES`]): a stager that failed to record provenance still has every doc it
/// staged recognised here, because the name it wrote is the name this reads. What it stops
/// recognising is a name jigc's writer cannot emit.
///
/// It answers about the *name* only. The **shape** question is the caller's, and the callers
/// ask it two ways on purpose: [`foreign_area_paths`] and `cli::task::TaskArea::staged_docs`
/// ask whether jigc wrote the entry, so they do not follow a symlink;
/// `cli::task::staged_doc_ids` asks what `jigc doc show <id> --task` can open, so it does.
pub fn staged_doc_id(file_name: &str) -> Option<&str> {
    file_name
        .strip_suffix(".md")
        .filter(|identity| identity.contains(':'))
}

/// **The bytes jigc did not write into `area`** — every entry of the working area that is
/// not a member of its registry row, relative to the area, sorted.
///
/// This is the subject every door that destroys a working area takes
/// (`design/team-ready-state.md` → The working area's two populations). The doors differ in
/// what they *do* with it — refuse without a consent, narrate under one, or move it aside —
/// and each records its own disposition; what none of them may do is take it silently,
/// which is what all five did before M52 Increment 4.
///
/// **A foreign directory is returned whole and never walked**: the entry is the unit a door
/// names, refuses over or moves, and moving the top of a subtree preserves it. The walk
/// descends only into the row's **tree** member, and exactly as far as that member's own rule
/// needs: one level into a task's `docs/`, whose rule is [`TASK_DOCS_FILES`] plus
/// [`staged_doc_id`]; two into a milestone's `merged/`, where jigc's set is the `docs/`
/// directory [`crate::milestone::materialize`] writes and, inside it, [`staged_doc_id`]
/// **alone** — the task branch's wider rule is deliberately not inherited, because
/// `materialize` writes only `<type>:<slug>.md` bodies, so a `merged/docs/provenance.json` is
/// a third party's file (`settle-record.md` → §2).
///
/// **Shape is part of membership.** A member's name on the wrong kind of entry — a
/// *directory* called `base.json`, a plain file called `docs/`, a symlink wearing either —
/// is not something jigc wrote, and the shape is read **without following symlinks**
/// ([`std::fs::DirEntry::file_type`]) because jigc writes regular files and real
/// directories and never a link. This is L-3's lesson at the door's own question: a
/// directory named `<type>:<slug>.md` was a staged identity at four surfaces until M52
/// Increment 4 / T1.
///
/// **Fail-closed, like every other probe a destroying door reads** (`staged_task_prose`'s
/// discipline): an **absent** area is `Ok(vec![])` — and absence is `NotFound` and nothing
/// else — while a present area that cannot be read, or an entry whose shape cannot be
/// stat'd, is an `Err`. Enumerating nothing and finding nothing are the same empty vector
/// to a caller about to delete, and only one of them is safe.
pub fn foreign_area_paths(area: &Path, kind: WorkArea) -> std::io::Result<Vec<PathBuf>> {
    match std::fs::symlink_metadata(area) {
        Ok(_) => {}
        Err(err) if err.kind() == std::io::ErrorKind::NotFound => return Ok(Vec::new()),
        Err(err) => return Err(err),
    }

    let mut foreign: Vec<PathBuf> = Vec::new();
    for entry in std::fs::read_dir(area)? {
        let entry = entry?;
        let raw = entry.file_name();
        let name = raw.to_string_lossy();
        let shape = entry.file_type()?;
        let mine = kind.jigc_written().contains(&name.as_ref())
            && if kind.tree_member(&name) {
                shape.is_dir()
            } else {
                shape.is_file()
            };
        if !mine {
            foreign.push(PathBuf::from(name.as_ref()));
            continue;
        }
        if !kind.tree_member(&name) {
            continue;
        }
        match kind {
            // `docs/` holds the task's staged instances beside whatever else was put there,
            // so its own rule decides entry by entry ([`TASK_DOCS_FILES`]).
            WorkArea::Task => {
                for staged in std::fs::read_dir(entry.path())? {
                    let staged = staged?;
                    let raw = staged.file_name();
                    let name = raw.to_string_lossy();
                    let shape = staged.file_type()?;
                    let mine = shape.is_file()
                        && (TASK_DOCS_FILES.contains(&name.as_ref())
                            || staged_doc_id(&name).is_some());
                    if !mine {
                        foreign.push(Path::new(DOCS_DIR).join(name.as_ref()));
                    }
                }
            }
            // `merged/` is the join's staging tree, and the only entry
            // [`crate::milestone::materialize`] writes into it is `docs/` — so that name on a
            // directory is the one member, and everything else at this level is a third
            // party's, returned whole.
            WorkArea::Milestone => {
                let merged = Path::new(crate::milestone::MERGED_AREA);
                for child in std::fs::read_dir(entry.path())? {
                    let child = child?;
                    let raw = child.file_name();
                    let name = raw.to_string_lossy();
                    let shape = child.file_type()?;
                    if !(shape.is_dir() && name == DOCS_DIR) {
                        foreign.push(merged.join(name.as_ref()));
                        continue;
                    }
                    for staged in std::fs::read_dir(child.path())? {
                        let staged = staged?;
                        let raw = staged.file_name();
                        let name = raw.to_string_lossy();
                        let shape = staged.file_type()?;
                        // **[`staged_doc_id`] alone**, never the task branch's wider rule:
                        // `materialize` writes only `<type>:<slug>.md` bodies, so inheriting
                        // [`TASK_DOCS_FILES`] would call a foreign
                        // `merged/docs/provenance.json` jigc's own and hand it to the doors
                        // that destroy this set (`settle-record.md` → §2).
                        if !(shape.is_file() && staged_doc_id(&name).is_some()) {
                            foreign.push(merged.join(DOCS_DIR).join(name.as_ref()));
                        }
                    }
                }
            }
        }
    }
    foreign.sort();
    Ok(foreign)
}

/// What [`unwind_area`] left behind — the three answers a caller about to report on a
/// working area needs to tell apart.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AreaUnwind {
    /// There was no area at that path to unwind.
    Absent,
    /// Every jigc-written entry is gone, and the directory with it.
    Removed,
    /// **The area survives**, because it holds bytes jigc did not write and removing it would
    /// have taken them. Two shapes answer this: the ordinary one, a minted directory with a
    /// foreign entry still in it, and the degenerate one, a non-directory squatting the area's
    /// own path — which jigc's mint never produced, so nothing there is jigc's to remove.
    Foreign,
}

/// A path [`unwind_area`] could neither remove nor account for — carried with the path
/// because the caller's whole job is to name it: an unwind that failed leaves a workbench
/// the operator may have to clear by hand, and *"could not unwind"* without a path is not
/// something anyone can act on (`design/surface-contract.md` → law 1).
#[derive(Debug)]
pub struct AreaUnwindError {
    /// The exact path whose removal (or whose read) failed.
    pub path: PathBuf,
    /// What the filesystem said.
    pub source: std::io::Error,
}

impl AreaUnwindError {
    fn at(path: &Path, source: std::io::Error) -> Self {
        Self {
            path: path.to_path_buf(),
            source,
        }
    }
}

impl std::fmt::Display for AreaUnwindError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.path.display(), self.source)
    }
}

impl std::error::Error for AreaUnwindError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        Some(&self.source)
    }
}

/// **Remove exactly what jigc wrote into `area`, and then the area — non-recursively** — the
/// removal half of [`foreign_area_paths`]' partition, and the sink of
/// `cli::rollback::ROLLBACK_POPULATIONS`' two `MintedSet` rows (M52 Increment 5 / T7;
/// `settle-record.md` → §2; `design/finalize.md` → Rollback discipline).
///
/// **Why it is not `remove_dir_all`.** A door that minted a working area and then failed to
/// record it must unwind the mint, or the identical re-run blocks on the id it already minted
/// forever (M47 Inc 2). Until M52 that unwind took the **directory**, and the interval it runs
/// in is exactly the interval that contains the door's rejecting hook — arbitrary code, with
/// the workbench in front of it. Driven at `8f0fb833`: a `pre-commit` hook that wrote a file
/// into `.jigc/tasks/<id>/docs/` and exited 1 had that file destroyed at exit 1, named by
/// nothing, and `.jigc/` is gitignored whole, so it had no second copy.
///
/// **The removal is keyed on the registry, never on a hand-list.** The set is the area's own
/// [`WorkArea::jigc_written`] row plus its tree member's own rule ([`unwind_docs`] for a
/// task's `docs/`, [`unwind_merged`] for a milestone's `merged/`) — the same membership
/// [`foreign_area_paths`] reports the complement of, so the two cannot disagree about a file.
/// A hand-list of *"what the mint wrote"* drove short at `milestone create`, whose door writes
/// `staged-snapshot.json` and `record-commit-msg.txt` into the area **after** the mint: every
/// rejected run would have left the area standing over jigc's own bytes.
///
/// **The safety is structural, not a check.** Nothing is enumerated and then deleted: each
/// removal names a registry member, and the directory itself goes through `remove_dir`, which
/// refuses a non-empty directory. So a third party's file — including one written *after* this
/// walk read the directory — survives by construction, and the caller is told
/// ([`AreaUnwind::Foreign`]) rather than the loss being discovered later.
///
/// **Shape is part of membership**, exactly as in [`foreign_area_paths`]: a *directory* named
/// `base.json`, a plain file named `docs/`, a symlink wearing either — none of those is
/// something jigc wrote, so none is removed, and the area then survives as `Foreign`. Shapes
/// are read without following symlinks.
///
/// `Err` is the fail-loud arm: a removal or a read that failed for any reason other than the
/// member being absent (or the directory being non-empty) stops the unwind **where it is**,
/// with the path, leaving the rest of the area as found.
pub fn unwind_area(area: &Path, kind: WorkArea) -> Result<AreaUnwind, AreaUnwindError> {
    match std::fs::symlink_metadata(area) {
        Ok(shape) if shape.is_dir() => {}
        // A plain file or a symlink wearing the area's name is not an area any mint of
        // jigc's produced, so nothing here is jigc's to remove.
        Ok(_) => return Ok(AreaUnwind::Foreign),
        Err(err) if err.kind() == std::io::ErrorKind::NotFound => return Ok(AreaUnwind::Absent),
        Err(err) => return Err(AreaUnwindError::at(area, err)),
    }

    for name in kind.jigc_written() {
        let path = area.join(name);
        let shape = match std::fs::symlink_metadata(&path) {
            Ok(shape) => shape,
            // Not every member is written on every path through a door — an absent one is
            // the ordinary case, not a fault.
            Err(err) if err.kind() == std::io::ErrorKind::NotFound => continue,
            Err(err) => return Err(AreaUnwindError::at(&path, err)),
        };
        if kind.tree_member(name) {
            if !shape.is_dir() {
                continue;
            }
            match kind {
                // `docs/` holds the task's staged instances next to whatever else was put
                // there, so its own rule decides entry by entry ([`TASK_DOCS_FILES`]).
                WorkArea::Task => unwind_docs(&path)?,
                // `merged/` is the join's staging tree, and what jigc wrote into it is the
                // `docs/` directory `materialize` writes and, inside that, the bodies
                // [`staged_doc_id`] recognises — so it is walked, by the rule
                // [`foreign_area_paths`] reports the complement of ([`unwind_merged`]).
                WorkArea::Milestone => unwind_merged(&path)?,
            }
        } else if shape.is_file() {
            std::fs::remove_file(&path).map_err(|err| AreaUnwindError::at(&path, err))?;
        }
    }

    match std::fs::remove_dir(area) {
        Ok(()) => Ok(AreaUnwind::Removed),
        // Something jigc did not write is still in there. That is the answer, not an error:
        // the bytes survive and the caller names the area.
        Err(err) if err.kind() == std::io::ErrorKind::DirectoryNotEmpty => Ok(AreaUnwind::Foreign),
        Err(err) => Err(AreaUnwindError::at(area, err)),
    }
}

/// [`unwind_area`]'s `merged/` arm: [`unwind_docs`]' twin, one level deeper — remove the
/// materialized bodies [`staged_doc_id`] recognises, then `merged/docs`, then `merged`, each
/// removal non-recursive and each tolerating a directory somebody else still has something in
/// (M53 Increment 1 / T3; `settle-record.md` → D1.4).
///
/// **It shipped as `remove_dir_all(merged/)`**, on the registry row's own statement that
/// `merged/` is *"jigc's wholesale, and nothing inside it is walked"*. That is an exclusion
/// resting on the predicate **nothing but jigc's bytes are in `merged/`**, and the predicate
/// is false four ways — [`MILESTONE_AREA_FILES`] carries the four with their data. The
/// removal half inherited the exclusion from the probe half, so the two halves of one
/// membership rule disagreed about a file: [`foreign_area_paths`] now calls
/// `merged/docs/provenance.json` a third party's, and this arm used to take it anyway.
///
/// The membership asked here is [`staged_doc_id`] **alone**, never [`TASK_DOCS_FILES`], for
/// the reason the probe states: [`crate::milestone::materialize`] writes only
/// `<type>:<slug>.md` bodies into this tree.
///
/// A non-empty `merged/docs` or `merged/` is **not** an error: what is left is somebody
/// else's, and the area's own `remove_dir` two frames up is where that becomes
/// [`AreaUnwind::Foreign`] — one file, one answer.
fn unwind_merged(merged: &Path) -> Result<(), AreaUnwindError> {
    let docs = merged.join(DOCS_DIR);
    match std::fs::symlink_metadata(&docs) {
        // Shape is part of membership here too: a plain file or a symlink wearing `docs`'
        // name is not the tree `materialize` writes, so nothing under it is jigc's.
        Ok(shape) if shape.is_dir() => {
            for entry in std::fs::read_dir(&docs).map_err(|err| AreaUnwindError::at(&docs, err))? {
                let entry = entry.map_err(|err| AreaUnwindError::at(&docs, err))?;
                let raw = entry.file_name();
                let name = raw.to_string_lossy();
                let shape = entry
                    .file_type()
                    .map_err(|err| AreaUnwindError::at(&entry.path(), err))?;
                if !(shape.is_file() && staged_doc_id(&name).is_some()) {
                    continue;
                }
                let path = entry.path();
                std::fs::remove_file(&path).map_err(|err| AreaUnwindError::at(&path, err))?;
            }
            match std::fs::remove_dir(&docs) {
                Ok(()) => {}
                Err(err) if err.kind() == std::io::ErrorKind::DirectoryNotEmpty => {}
                Err(err) => return Err(AreaUnwindError::at(&docs, err)),
            }
        }
        Ok(_) => {}
        Err(err) if err.kind() == std::io::ErrorKind::NotFound => {}
        Err(err) => return Err(AreaUnwindError::at(&docs, err)),
    }
    match std::fs::remove_dir(merged) {
        Ok(()) => Ok(()),
        Err(err) if err.kind() == std::io::ErrorKind::DirectoryNotEmpty => Ok(()),
        Err(err) => Err(AreaUnwindError::at(merged, err)),
    }
}

/// [`unwind_area`]'s `docs/` arm: remove the staged instances and the provenance manifest —
/// [`TASK_DOCS_FILES`] plus the [`staged_doc_id`] rule, the identical membership
/// [`foreign_area_paths`] descends one level to ask — then the directory, non-recursively.
///
/// A non-empty `docs/` is **not** an error here: what is left is somebody else's, and the
/// area's own `remove_dir` one frame up is where that becomes [`AreaUnwind::Foreign`]. Saying
/// it twice would make one third-party file two answers.
fn unwind_docs(docs: &Path) -> Result<(), AreaUnwindError> {
    for entry in std::fs::read_dir(docs).map_err(|err| AreaUnwindError::at(docs, err))? {
        let entry = entry.map_err(|err| AreaUnwindError::at(docs, err))?;
        let raw = entry.file_name();
        let name = raw.to_string_lossy();
        let shape = entry
            .file_type()
            .map_err(|err| AreaUnwindError::at(&entry.path(), err))?;
        let mine = shape.is_file()
            && (TASK_DOCS_FILES.contains(&name.as_ref()) || staged_doc_id(&name).is_some());
        if !mine {
            continue;
        }
        let path = entry.path();
        std::fs::remove_file(&path).map_err(|err| AreaUnwindError::at(&path, err))?;
    }
    match std::fs::remove_dir(docs) {
        Ok(()) => Ok(()),
        Err(err) if err.kind() == std::io::ErrorKind::DirectoryNotEmpty => Ok(()),
        Err(err) => Err(AreaUnwindError::at(docs, err)),
    }
}

/// The on-disk path a staged `<type>:<slug>` instance lives at within `task_dir`:
/// `<task_dir>/docs/<type>:<slug>.md`.
pub fn instance_path(task_dir: &Path, type_name: &str, slug: &str) -> PathBuf {
    task_dir
        .join(DOCS_DIR)
        .join(instance_filename(type_name, slug))
}

/// Build the **empty** in-memory instance for `schema` — the canonical template the
/// workflow provisions: the H1 title is the caller-supplied `title` (the human
/// id-source text, **not** the kebab slug — `design/write-commands.md` → the H1
/// renders the title, the id is `slugify(title)`), every body section is left
/// content-free (no slot prose, no items), and every **header** section pre-stamps
/// its **author-required** fields as empty `key:` lines in schema-declared order
/// (M40 F1 — the commit doc's fillable-form precedent generalized through the shared
/// [`crate::validate::is_author_required`] predicate, so a required field is never
/// invisible at create; `design/write-commands.md` → Instance provisioning;
/// `DECISIONS.md` 2026-07-10 → M40 Settle #4). Rendered through
/// [`crate::write::render`], this yields the full skeleton — front-matter (the
/// stamped empty keys), the `# <title>` H1, and every `## Heading` with an empty
/// slot — the bytes the agent then fills slot-by-slot (`design/write-commands.md` →
/// Instance provisioning: "the agent only fills slots").
fn empty_instance(schema: &Schema, title: &str) -> Instance {
    Instance {
        title: title.to_string(),
        sections: schema
            .sections
            .iter()
            .map(|s| SectionContent {
                id: s.id.clone(),
                fields: stamped_header_fields(s),
                ..Default::default()
            })
            .collect(),
    }
}

/// The pre-stamped empty `key:` lines for one schema section: every
/// **author-required** field of a **header** simple section, in schema-declared
/// order ([`empty_instance`]'s M40 F1 skeleton stamp). A non-header section, a
/// repeatable, and every exempt field (`default:`/`set:`/optional/`optional ref`/
/// pack-typed — [`crate::validate::is_author_required`]) stamp nothing.
fn stamped_header_fields(section: &crate::schema::Section) -> Vec<crate::field_block::Field> {
    if !section.header {
        return Vec::new();
    }
    let crate::schema::SectionBody::Simple { fields, .. } = &section.body else {
        return Vec::new();
    };
    fields
        .iter()
        .filter(|f| crate::validate::is_author_required(f))
        .map(|f| crate::field_block::Field {
            key: f.id.clone(),
            value: crate::field_block::Value::Scalar(String::new()),
        })
        .collect()
}

/// The empty template ([`empty_instance`]) with the caller-supplied **on-create**
/// field values seeded into its **header** section — the clock-free seam half of
/// the doc-level `set: on-create` / `default:` materialization (`design/changelog.md`
/// → engine work #4). The engine places the bytes the caller hands it verbatim,
/// **never** reading `set:`/`default:` itself: the CLI owns the clock and computes
/// the slice (mirroring the item-level [`crate::write::add_item`] `on_create` path).
///
/// An **empty** `on_create` slice leaves the instance byte-identical to
/// [`empty_instance`] — the additive-neutrality guard every existing caller relies
/// on (which, since M40 F1, includes the pre-stamped author-required header lines).
/// Each non-empty seed field is routed to the simple section whose `fields`
/// **declares a leaf of that id** (mirroring the item-level model where a field lands
/// in its own block), **merged with that section's skeleton stamps in the section's
/// own declared field order** — a stamped author-required line and a seeded
/// default/on-create value interleave exactly as the schema declares them. A field
/// declared by no simple section is dropped (no home to seed) — but the CLI collects
/// seeds *from* the schema's simple sections, so every seed has a declaring section.
/// (Before this routing the slice was written wholesale into the single header
/// section, silently misplacing a seed declared in a non-header body section — inert
/// for the shipped header-only doctypes, but a latent gap.)
fn seeded_instance(
    schema: &Schema,
    title: &str,
    on_create: &[crate::field_block::Field],
) -> Instance {
    let mut instance = empty_instance(schema, title);
    if on_create.is_empty() {
        return instance;
    }
    for section in &schema.sections {
        let crate::schema::SectionBody::Simple { fields, .. } = &section.body else {
            continue;
        };
        // The section's declared fields, in declared order: a seed where the caller
        // supplied one, else the skeleton's author-required empty stamp (header only).
        let merged: Vec<crate::field_block::Field> = fields
            .iter()
            .filter_map(|decl| {
                if let Some(seed) = on_create.iter().find(|f| f.key == decl.id) {
                    return Some(seed.clone());
                }
                if section.header && crate::validate::is_author_required(decl) {
                    return Some(crate::field_block::Field {
                        key: decl.id.clone(),
                        value: crate::field_block::Value::Scalar(String::new()),
                    });
                }
                None
            })
            .collect();
        if merged.is_empty() {
            continue;
        }
        if let Some(content) = instance.sections.iter_mut().find(|c| c.id == section.id) {
            content.fields = merged;
        }
    }
    instance
}

/// The provenance-manifest filename inside a task's `docs/` area — the on-disk record
/// the by-task-id join reads to classify each staged doc (`storage.md` → The by-task-id
/// join → classification by provenance; `DECISIONS.md` 2026-06-04 → M7 Increment 2 T1).
/// It rides **beside** the `.md` bodies (one manifest per `docs/` area), so the doc body
/// bytes the round-trip writer owns stay byte-for-byte unchanged.
const PROVENANCE_FILE: &str = "provenance.json";

/// How a doc came to be staged in a task's `docs/` area — the discriminator the
/// by-task-id join's clash rule needs (`storage.md` → The by-task-id join → classification
/// by provenance). Recorded at stage time, *before* the join depends on it.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum Provenance {
    /// The doc was **minted in this sub-task** ([`provision_doc`]) — a brand-new
    /// instance, not present in the committed store at the milestone base.
    Created,
    /// The doc **existed in the committed store at the milestone base** and was copied
    /// in for editing ([`copy_in`]).
    EditedFromBase,
}

/// The per-`docs/`-area **provenance manifest**: a map from a staged doc's
/// `<type>:<slug>` address to its [`Provenance`] (`storage.md` → The by-task-id join →
/// classification by provenance). Written atomically beside the `.md` bodies by the two
/// staging primitives ([`provision_doc`] → [`Provenance::Created`], [`copy_in`] →
/// [`Provenance::EditedFromBase`]) and read back by the Increment-3 join.
///
/// A [`BTreeMap`](std::collections::BTreeMap) so the serialized JSON is **key-sorted and
/// deterministic** — the byte form is golden-stable, the same convention as
/// [`RolesRecord`] / `base.json`. The record is task-local working-area state, disposable
/// with the task, so it carries no schema version of its own.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProvenanceRecord {
    /// `<type>:<slug>` address → provenance, address-sorted for deterministic output.
    pub docs: std::collections::BTreeMap<String, Provenance>,
}

impl ProvenanceRecord {
    /// The manifest's on-disk location inside a task's `docs/` area.
    pub fn path_in(task_dir: &Path) -> PathBuf {
        task_dir.join(DOCS_DIR).join(PROVENANCE_FILE)
    }

    /// Record `address`'s provenance **write-once**: the first provenance recorded for
    /// an address sticks; a later `record` of the same address is a no-op
    /// (`write-commands.md` → copy-on-first-touch: "provenance is recorded once, at
    /// first touch, keyed on whether the slug existed in the committed store at the
    /// milestone base — `created` is sticky across later edits in the same area").
    /// So a `created` doc later copied-in for editing stays `created`, never flipping
    /// to `edited-from-base`, keeping the join's mixed-case clash discriminator stable.
    pub fn record(&mut self, address: impl Into<String>, provenance: Provenance) {
        self.docs.entry(address.into()).or_insert(provenance);
    }

    /// The provenance recorded for `address`, if any.
    pub fn get(&self, address: &str) -> Option<Provenance> {
        self.docs.get(address).copied()
    }

    /// Serialize to the frozen on-disk byte form: pretty JSON, address-sorted, one
    /// trailing newline (golden-locked, matching the `base.json` / `roles.json` convention).
    pub fn to_bytes(&self) -> String {
        let mut s = serde_json::to_string_pretty(self).expect("ProvenanceRecord serializes");
        s.push('\n');
        s
    }

    /// Load the manifest from `<task_dir>/docs/provenance.json`. A missing file is the
    /// *nothing-staged-yet* case and yields an empty record, never an error (mirrors
    /// [`RolesRecord::load`]'s absent-is-empty contract).
    pub fn load(task_dir: &Path) -> std::io::Result<Self> {
        match std::fs::read(Self::path_in(task_dir)) {
            Ok(bytes) => Ok(serde_json::from_slice(&bytes)?),
            Err(err) if err.kind() == std::io::ErrorKind::NotFound => Ok(Self::default()),
            Err(err) => Err(err),
        }
    }
}

/// Load the `docs/` provenance manifest, record `address` → `provenance`, and persist it
/// atomically — the shared stage-time provenance write the two staging primitives perform
/// **after** the `.md` body lands (so the body bytes stay byte-for-byte unchanged).
fn record_provenance(
    task_dir: &Path,
    address: &str,
    provenance: Provenance,
) -> std::io::Result<()> {
    let mut record = ProvenanceRecord::load(task_dir)?;
    record.record(address, provenance);
    write_atomic(
        &ProvenanceRecord::path_in(task_dir),
        record.to_bytes().as_bytes(),
    )
}

/// Record `address` → `provenance` in the task's `docs/` provenance manifest — the
/// write-once stage-time record the by-task-id join reads (`storage.md` → The
/// by-task-id join → classification by provenance). The public companion of the
/// staging primitives' internal write, for callers that persist a staged body
/// outside [`provision_doc`] / [`copy_in`] and must still record its provenance —
/// the CLI's `provision_commit_doc` records `commit:<id>` → [`Provenance::Created`]
/// this way (`write-commands.md` → copy-on-first-touch: "`provision_commit_doc`
/// likewise records `created` provenance under M8"). Write-once: a re-entry that
/// re-records the same address never overwrites the first ([`ProvenanceRecord::record`]).
pub fn record_doc_provenance(
    task_dir: &Path,
    address: &str,
    provenance: Provenance,
) -> std::io::Result<()> {
    record_provenance(task_dir, address, provenance)
}

/// **Provision** a workflow-provisioned empty doc instance into the task working area
/// (`design/write-commands.md` → Instance provisioning → Workflow-provisioned;
/// `implementation/parsing.md` → The write pipeline → "Writes land in the task working
/// area"). Materializes `<task_dir>/docs/<type>:<slug>.md` with the canonical
/// empty-template bytes ([`crate::write::render`] of [`empty_instance`]), atomically
/// (temp + rename), and returns its path. The `docs/` dir is created on demand.
///
/// Pure working-area filesystem effect — no verbs, no git. The `type` name is read
/// from the schema; the `slug` is the task-derived id (`commit:<task-id>`).
///
/// The `slug` drives the **filename/address** (`<location>/<slug>.md`); the `title`
/// drives the **`# H1` display text** — the two are deliberately separate so a
/// title-slugged doctype's H1 reads as the human title (`# Use MySQL`) while the id
/// stays the kebab slug (`use-mysql`), `slugify(title) == slug` keeping the address
/// unambiguous (`design/write-commands.md` → the H1 renders the title).
///
/// `on_create` is the additive clock-free **seed seam**: the caller-computed
/// doc-level `set: on-create` / `default:` header field values, written into the
/// header section before render ([`seeded_instance`]). An **empty** slice is
/// byte-neutral — the provisioned bytes equal `write::render` of the unseeded
/// [`empty_instance`], the form every existing caller passes.
pub fn provision_doc(
    task_dir: &Path,
    schema: &Schema,
    slug: &str,
    title: &str,
    on_create: &[crate::field_block::Field],
) -> std::io::Result<PathBuf> {
    let path = instance_path(task_dir, &schema.ty, slug);
    let bytes = provisioned_bytes(schema, title, on_create);
    write_atomic(&path, bytes.as_bytes())?;
    // A minted-here instance: record `created` beside the body for the join's clash rule.
    record_provenance(
        task_dir,
        &format!("{}:{slug}", schema.ty),
        Provenance::Created,
    )?;
    Ok(path)
}

/// The exact bytes [`provision_doc`] materializes for a fresh mint — the **pristine
/// skeleton** of `schema` under `title`, seeded with the caller-computed `on_create`
/// header fields.
///
/// Split out of [`provision_doc`] (which is its only writer) so a caller that needs to
/// know what a create *would have* staged can ask for it without staging anything: the
/// CLI's changelog-gate advisory reconstructs a freshly-created doc's **un-authored
/// baseline** this way, and must compare against the same bytes the mint wrote, never a
/// second rendering of the same idea (`design/validation.md` → The changelog-gate
/// advisory).
pub fn provisioned_bytes(
    schema: &Schema,
    title: &str,
    on_create: &[crate::field_block::Field],
) -> String {
    write::render(schema, &seeded_instance(schema, title, on_create))
}

/// **Copy-in on first touch** of a pre-existing managed doc into the task working area
/// (`implementation/parsing.md` → The write pipeline → "an existing doc is copied in
/// (base-pinned) on first touch"; `DECISIONS.md` 2026-05-31 → copy-in-on-first-touch).
/// Applies the *only* permitted first-touch canonicalization
/// ([`crate::write::first_touch_canonicalize`] — BOM strip + single trailing newline,
/// EOL-preserving, everything else byte-for-byte) and persists the result atomically at
/// `<task_dir>/docs/<type>:<slug>.md`, returning its path. The committed source file is
/// untouched (this writes only the working copy).
pub fn copy_in(
    task_dir: &Path,
    type_name: &str,
    slug: &str,
    source: &str,
) -> std::io::Result<PathBuf> {
    let path = instance_path(task_dir, type_name, slug);
    let canonical = write::first_touch_canonicalize(source);
    write_atomic(&path, canonical.as_bytes())?;
    // A base-existing instance copied in for editing: record `edited-from-base`.
    record_provenance(
        task_dir,
        &format!("{type_name}:{slug}"),
        Provenance::EditedFromBase,
    )?;
    Ok(path)
}

/// **Atomic persist** of an edited buffer to a working-area doc path
/// (`implementation/parsing.md` → The write pipeline → "Atomic on disk — write temp +
/// rename"). Writes `bytes` to a sibling temp file, then `rename`s it over `path`, so a
/// reader never observes a partial write and no temp residue survives a successful
/// persist. The parent dir is created on demand.
pub fn persist(path: &Path, bytes: &[u8]) -> std::io::Result<()> {
    write_atomic(path, bytes)
}

/// Write `bytes` to `path` via the temp-file + `rename` dance (the atomic-on-disk
/// primitive shared by [`provision_doc`], [`copy_in`], and [`persist`]). The temp file
/// is a process-unique sibling (`<filename>.<pid>.<nanos>.tmp`) so the `rename` stays on
/// the same filesystem (atomic) and concurrent writers never share one temp;
/// it is removed on a write failure and consumed by the rename on success — never left
/// behind. The parent dir is created on demand.
fn write_atomic(path: &Path, bytes: &[u8]) -> std::io::Result<()> {
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)?;
    }
    let tmp = temp_sibling(path);
    if let Err(err) = std::fs::write(&tmp, bytes) {
        let _ = std::fs::remove_file(&tmp);
        return Err(err);
    }
    if let Err(err) = std::fs::rename(&tmp, path) {
        let _ = std::fs::remove_file(&tmp);
        return Err(err);
    }
    Ok(())
}

/// The sibling temp path for an atomic write of `path` — its filename with a
/// **globally-unique** `.<pid>.<nanos>.tmp` suffix (same directory, so `rename` is
/// intra-filesystem and atomic). The disambiguator is what makes concurrent writers
/// to a *shared* target (e.g. `.jigc/state/file-state.json`, which is not
/// task-isolated) each own a distinct temp: without it two writers would share one
/// `<name>.tmp` and interleave their bytes, so a reader could observe a file that
/// parses as neither writer's record (M45 Increment 7, Decision 9).
///
/// The two components fence the two collision axes. `pid` separates concurrent
/// *processes*. The raw clock does **not** separate the calls inside one — two
/// threads routinely read the same value where the OS resolution is coarser than a
/// nanosecond, mint the same temp path, and then writer A's `rename` consumes it and
/// writer B's fails `NotFound`. So `nanos` comes from [`crate::tempname::unique_nanos`],
/// which is strictly increasing per process and therefore never repeats however coarse
/// the clock is.
fn temp_sibling(path: &Path) -> PathBuf {
    let mut name = path.file_name().unwrap_or_default().to_os_string();
    name.push(format!(
        ".{}.{}.tmp",
        std::process::id(),
        crate::tempname::unique_nanos(),
    ));
    match path.parent() {
        Some(parent) => parent.join(name),
        None => PathBuf::from(name),
    }
}

/// Whether `name` is a temp sibling [`write_atomic`] could have minted for a target named
/// `target` — `<target>.<pid>.<nanos>.tmp`, both components all digits. The inverse of
/// [`temp_sibling`], and the one home that rule has.
///
/// A successful persist consumes its temp and a failed one removes it, so on every path
/// the writer finishes there is none. The path it does **not** finish — a process killed
/// between the write and the `rename` — leaves one beside the shared caches, and a door
/// that asks *"did jigc write this?"* over those directories has to be able to say yes
/// (`cli::setup`'s teardown, whose foreign-byte guard would otherwise refuse over jigc's
/// own residue and call it a path jigc did not write).
#[must_use]
pub fn is_temp_sibling(name: &str, target: &str) -> bool {
    let digits = |part: &str| !part.is_empty() && part.bytes().all(|byte| byte.is_ascii_digit());
    name.strip_prefix(target)
        .and_then(|rest| rest.strip_prefix('.'))
        .and_then(|rest| rest.strip_suffix(".tmp"))
        .and_then(|middle| middle.split_once('.'))
        .is_some_and(|(pid, nanos)| digits(pid) && digits(nanos))
}

/// How long a save waits for the save-scoped lock before it **fails** — wall-clock, not
/// a count of sleeps, so the ceiling is the time it states however late the scheduler
/// wakes the waiter (2026-10-01; `DECISIONS.md` → that date).
///
/// **Ten seconds, from the measured contention.** A waiter's wait is the sum of the
/// critical sections queued ahead of it — one small read, one merge, one write +
/// `rename`, a millisecond or two each — and `try_lock` polling is not fair, so a waiter
/// can lose several releases in a row. Driven: the two survival cells of
/// `file_state_concurrency.rs`, run as twelve concurrent copies beside 24 CPU hogs on a
/// 10-core host (unoptimized build): of the 12,585 acquisitions that waited 20 ms or
/// more, the median waited 169 ms, p99 917 ms and the worst 1.37 s. The 1,000 × 1 ms
/// attempt budget this replaces ran out on a loaded 4-vCPU CI runner. Ten seconds is
/// seven times the worst wait observed and ten times the budget that expired.
///
/// **What a long budget costs, and why that is the only cost.** The lock is an OS
/// advisory lock (`flock` on Unix), which the kernel releases when its holder's process
/// exits, crash or `kill -9` included — so a *dead* holder never blocks a save, and the
/// budget only ever runs out behind a holder that is alive and stuck. Generosity costs
/// that case ten seconds before it reports; it costs every other case nothing.
pub const SAVE_LOCK_BUDGET: std::time::Duration = std::time::Duration::from_secs(10);
/// The pause between two `try_lock` attempts inside [`SAVE_LOCK_BUDGET`].
pub const SAVE_LOCK_SPIN: std::time::Duration = std::time::Duration::from_millis(1);

/// The **stable** lock sibling for a shared `.jigc/` cache file: `<filename>.lock` in
/// the same directory.
///
/// It must be a sibling and never the target's own fd. [`write_atomic`] persists by
/// `rename`ing a temp over `path`, so the target's inode is **replaced on every write**:
/// an advisory lock taken on that fd would guard an inode the very next persist orphans,
/// and the mutual exclusion would silently be no exclusion at all. The lock file's own
/// inode is never replaced — nothing ever writes to it — so every holder locks the same
/// object.
///
/// Both current targets (`.jigc/state/file-state.json`, `.jigc/index/edges.json`) live
/// under `state/` and `index/`, which `cli::gitignore::ENTRIES` already ignores, so the
/// sibling needs no gitignore change and cannot reach a commit.
pub fn lock_sibling(path: &Path) -> PathBuf {
    let mut name = path.file_name().unwrap_or_default().to_os_string();
    name.push(".lock");
    match path.parent() {
        Some(parent) => parent.join(name),
        None => PathBuf::from(name),
    }
}

/// Run `critical` under an advisory lock on [`lock_sibling`] of `path` — the
/// **save-scoped exclusion** that closes the read-modify-write window a base-relative
/// merge alone leaves open (M46 Increment 1; `DECISIONS.md` 2026-08-18 M46 planned,
/// N-3). Returns whatever `critical` returns.
///
/// **Scope, deliberately narrow — this wraps [`crate::file_state::FileStateRecord::save`]
/// and [`crate::index::EdgeIndex::save`] only, never [`persist`] itself**, which is also
/// the task-area / base-pin / roles writer and has no shared-target problem to solve. The
/// critical section **spawns no subprocess**: that is what dissolves the deadlock that
/// made a coarse lock unaffordable here — jigc's own `pre-commit` hook runs a nested
/// `jigc validate`, and `File::lock` is per-open-file-description, so a lock held across
/// a `git commit` would have the parent waiting on a child that waits on the parent. The
/// two locks are **per target and never nested**: the paired call sites (`cli/ingest.rs`,
/// `cli/unmanage.rs`) save the index and the record sequentially.
///
/// **A save never runs unlocked** (2026-10-01). If the lock is not taken within
/// [`SAVE_LOCK_BUDGET`], or the lock sibling cannot be opened or locked at all,
/// `critical` does not run and the save fails with an error naming the lock file and a
/// route — nothing is written. Until then the save ran its critical section anyway (the
/// M46 degrade); on `file-state.json` that loses a concurrent writer's key, and a lost
/// baseline silently switches off `reconciliation.conflict-block` (`storage.md` →
/// Concurrent writers). The budget is what keeps "never unlocked" from becoming "may
/// hang".
pub fn with_save_lock<T>(
    path: &Path,
    critical: impl FnOnce() -> std::io::Result<T>,
) -> std::io::Result<T> {
    let _guard = SaveLock::acquire(path)?;
    critical()
}

/// A tally of the save path's **silent degrade** — the arm that runs a shared-cache save
/// weaker than designed while still reporting `Ok`. **A diagnostic counter, and nothing
/// else:** tallying changes no return value, no output and no control flow, and nothing
/// in the product reads it. It exists so a test that observes a lost concurrent delta can
/// say in its own failure message whether any of its saves clobbered an unreadable record
/// (`crates/cli/tests/file_state_concurrency.rs`).
///
/// **The lock arms are retired, not zero (2026-10-01).** It also counted the saves that
/// ran *unlocked* — a spin that ran out, or a lock that could not be opened — and the
/// first CI red that counted one (PR #5's run `36927086957`: *1 ran unlocked (lock
/// timeout 1)*) is what retired them: both arms now fail the save instead
/// ([`with_save_lock`]), so there is no silent outcome left to count, and a survival cell
/// whose save fails says so in its own panic.
///
/// **Per thread, deliberately.** A save's critical section runs inline on the calling
/// thread, so a thread's tally is exactly the degrades of the saves it made — and a
/// test that forces a degrade by design cannot pollute a sibling test's count in the same
/// process, which a process-global counter would.
///
/// **Not part of `jigc-engine`'s API.** It is `pub` only so `jigc`'s integration suite
/// can read it across the crate boundary; it is a test diagnostic, hidden from the docs,
/// and carries no stability promise of any kind.
#[doc(hidden)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct SaveDegrades {
    /// [`crate::file_state::FileStateRecord::save`]'s re-read of the record on disk
    /// failed, so the merge degraded to `theirs = ours` — the clobbering pre-merge write.
    pub unreadable_theirs: usize,
}

/// Which [`SaveDegrades`] arm a degrade is tallied on.
#[derive(Clone, Copy, Debug)]
pub(crate) enum SaveDegrade {
    UnreadableTheirs,
}

thread_local! {
    static SAVE_DEGRADES: std::cell::Cell<SaveDegrades> = const {
        std::cell::Cell::new(SaveDegrades {
            unreadable_theirs: 0,
        })
    };
}

/// The calling thread's [`SaveDegrades`] tally since it started. Monotone; read it before
/// and after the work under measurement and subtract.
///
/// **Not part of `jigc-engine`'s API**, like [`SaveDegrades`]: a test diagnostic, `pub`
/// only for `jigc`'s integration suite, hidden from the docs, with no stability promise.
#[doc(hidden)]
pub fn save_degrades() -> SaveDegrades {
    SAVE_DEGRADES.with(std::cell::Cell::get)
}

/// Tally one degrade on the calling thread. Saturating, so the diagnostic can never
/// panic its way into the save path's behaviour.
pub(crate) fn tally_save_degrade(arm: SaveDegrade) {
    SAVE_DEGRADES.with(|cell| {
        let mut tally = cell.get();
        let count = match arm {
            SaveDegrade::UnreadableTheirs => &mut tally.unreadable_theirs,
        };
        *count = count.saturating_add(1);
        cell.set(tally);
    });
}

/// An acquired advisory lock, released on drop (including on an unwinding panic out of
/// the critical section).
struct SaveLock(std::fs::File);

impl SaveLock {
    /// Take the lock, polling every [`SAVE_LOCK_SPIN`] until [`SAVE_LOCK_BUDGET`] has
    /// passed. Every failure is an error the save returns — there is no unlocked arm:
    ///
    /// - the budget runs out behind a live holder → [`std::io::ErrorKind::TimedOut`];
    /// - the lock sibling (or its directory) cannot be created or opened, or `try_lock`
    ///   fails with anything but *would block* → that error's own kind.
    ///
    /// Each message names the lock file, says nothing was written, and carries the route.
    fn acquire(path: &Path) -> std::io::Result<Self> {
        let lock_path = lock_sibling(path);
        let failed = |doing: &str, err: std::io::Error| {
            std::io::Error::new(
                err.kind(),
                format!(
                    "could not {doing} the save lock `{}`: {err}, so nothing was written — \
                     resolve the I/O condition (a disk or permissions problem under \
                     `.jigc/`), then retry",
                    lock_path.display(),
                ),
            )
        };
        if let Some(parent) = lock_path.parent() {
            std::fs::create_dir_all(parent)
                .map_err(|err| failed("create the directory of", err))?;
        }
        let file = std::fs::OpenOptions::new()
            .create(true)
            .read(true)
            .write(true)
            .truncate(false)
            .open(&lock_path)
            .map_err(|err| failed("open", err))?;
        let started = std::time::Instant::now();
        loop {
            match file.try_lock() {
                Ok(()) => return Ok(SaveLock(file)),
                Err(std::fs::TryLockError::WouldBlock) => {}
                Err(std::fs::TryLockError::Error(err)) => return Err(failed("take", err)),
            }
            if started.elapsed() >= SAVE_LOCK_BUDGET {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::TimedOut,
                    format!(
                        "timed out after {}s waiting for the save lock `{}`: another jigc \
                         process holds it and has not released it, so nothing was written — \
                         retry once that process has finished",
                        SAVE_LOCK_BUDGET.as_secs(),
                        lock_path.display(),
                    ),
                ));
            }
            std::thread::sleep(SAVE_LOCK_SPIN);
        }
    }
}

impl Drop for SaveLock {
    fn drop(&mut self) {
        let _ = self.0.unlock();
    }
}

/// The base commit a task was started against: the full 40-char SHA and the
/// abbreviated short SHA, both as `git rev-parse` reports them.
///
/// Operating a task whose base ≠ the current checkout is detected and routed
/// (`storage.md` → A task is pinned to its base); this is the pinned value the
/// CLI compares HEAD against on resume.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct BasePin {
    /// The full commit SHA HEAD pointed at when the task was minted.
    pub sha: String,
    /// The abbreviated short SHA (for human-facing divergence messages).
    pub short: String,
}

impl BasePin {
    /// A base pin from a full and short SHA.
    pub fn new(sha: impl Into<String>, short: impl Into<String>) -> Self {
        Self {
            sha: sha.into(),
            short: short.into(),
        }
    }
}

/// A freshly minted task: its frozen slug `id`, its working-area `dir`, and the
/// `base` it was pinned to.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct MintedTask {
    /// The frozen content-slug id (slugged from the intent, type-name fallback).
    pub id: String,
    /// The per-task working area, `<jigc_root>/tasks/<id>/`.
    pub dir: PathBuf,
    /// The base commit the task is pinned to.
    pub base: BasePin,
}

/// Mint a task: slug the `intent` (empty → `type_name`), reject on a serial
/// collision with an existing active task, else open `<jigc_root>/tasks/<id>/`
/// and write the base-pin file capturing `base`, the verbatim `intent`, and the
/// `workflow_id` the task was minted from (so resume composes the task's *own*
/// workflow — [`read_workflow_id`]).
///
/// `slug_override` **drives the minted id verbatim** when `Some` — the front door's
/// `--slug` override (`DECISIONS.md` 2026-07-06 M39 planning → Slug (G6);
/// `design/write-commands.md` → `jigc rename`'s `--slug` precedent): the intent is
/// still persisted verbatim (so resume re-composes with the same `{{task.intent}}`),
/// but the id is the caller-supplied slug, not the slugged intent. `None` is
/// today's behavior — the slugged intent with the empty→type-name fallback ([`mint_id`]).
/// The override is validated at the CLI boundary (via [`crate::slug::is_slug`]) and
/// used as-is here, so the serial-collision guard below rejects a colliding override
/// through the same route a colliding slugged-intent takes.
///
/// `jigc_root` is the project's `.jigc/` home (a temp root under test). On
/// success the working-area directory and its `base.json` exist on disk. On a
/// serial collision the returned [`Finding`] is `Severity::Blocking`, carries the
/// existing task's status in its message, and a `route` directing the agent to
/// resume or discard — nothing new is created.
pub fn mint_task(
    jigc_root: &Path,
    intent: &str,
    type_name: &str,
    workflow_id: &str,
    base: BasePin,
    slug_override: Option<&str>,
) -> Result<MintedTask, Finding> {
    let id = match slug_override {
        Some(slug) => slug.to_string(),
        None => mint_id(intent, type_name),
    };
    let dir = jigc_root.join("tasks").join(&id);

    // Serial collision: something of that id already exists at the area path → reject,
    // never silently suffixed or reused. **Which** refusal splits on the residual rule
    // ([`carries_base_pin`]): a live task gets its status surfaced and a resume route; a
    // pin-less *directory* is no task at any door, so claiming it is "already active" would
    // be a law-1 lie routed at two verbs that both answer `finalize.no-task` over it.
    //
    // A **non-directory** squatting the area path keeps today's answer, deliberately: the
    // residual sentence describes a directory and its two causes, the increment's axis is
    // three directory shapes, and a regular file there is a different condition owed a
    // different sentence — carried as a stated bound rather than answered with a sentence
    // that would be false (`DECISIONS.md` 2026-09-22 → M53 Increment 3 / T4).
    if dir.exists() {
        return Err(if dir.is_dir() && !carries_base_pin(&dir, WorkArea::Task) {
            residual_collision_finding(&id, &area_repo_path(jigc_root, &dir))
        } else {
            collision_finding(&id)
        });
    }

    std::fs::create_dir_all(&dir).map_err(|err| io_finding(&id, "open the working area", &err))?;

    let pin_path = dir.join(BASE_PIN_FILE);
    let body = render_base_pin(&base);
    std::fs::write(&pin_path, body).map_err(|err| io_finding(&id, "write the base pin", &err))?;

    // Persist the original intent verbatim so resume re-composes with the same
    // `{{task.intent}}` (the id is a lossy slug; the intent must be stored).
    std::fs::write(dir.join(INTENT_FILE), intent)
        .map_err(|err| io_finding(&id, "write the task intent", &err))?;

    // Persist the minting workflow id so resume composes the task's *own* workflow,
    // not the cascade default (`DECISIONS.md` 2026-06-01 → M2 Increment 3 re-cut).
    std::fs::write(dir.join(WORKFLOW_FILE), workflow_id)
        .map_err(|err| io_finding(&id, "write the task workflow id", &err))?;

    Ok(MintedTask { id, dir, base })
}

/// Enumerate the **active task ids** under a project's `.jigc/` home — the sorted ids of
/// those `<jigc_root>/tasks/<id>/` directories that **carry their base pin**
/// ([`carries_base_pin`]). This is the single enumeration source of truth the active-task
/// resolution (`jigc doc`), the `jigc task list` roster, `jigc start`'s orientation and its
/// `also open:` block, and the ambiguous-task error all read, so they never disagree on
/// which tasks are live. A missing `tasks/` directory yields an empty list (no task minted
/// yet), never an error.
///
/// **A directory is not a task** (M53 Increment 3 / T2; `settle-record.md` → D3). Until
/// then the filter was `entry.path().is_dir()`, so a leftover a faulted teardown left — or
/// a bare `mkdir` — was reported as live work by every surface above, each of them offering
/// a resume route into a task that does not exist. The pin is what a mint writes first and
/// what an unwind removes first, so it is the one file whose presence means *a work unit
/// lives here*.
pub fn list_active_task_ids(jigc_root: &Path) -> Vec<String> {
    let tasks = jigc_root.join("tasks");
    let mut ids: Vec<String> = match std::fs::read_dir(&tasks) {
        Ok(entries) => entries
            .filter_map(std::result::Result::ok)
            .filter(|entry| carries_base_pin(&entry.path(), WorkArea::Task))
            .filter_map(|entry| entry.file_name().into_string().ok())
            .collect(),
        Err(_) => Vec::new(),
    };
    ids.sort();
    ids
}

/// Read the persisted original intent of a task from its working area
/// (`<task_dir>/intent`). The companion of the [`mint_task`] write — resume reads
/// it back to re-compose with the same `{{task.intent}}`. A missing file yields
/// an empty intent (a task minted before this file existed, or an empty intent),
/// never an error.
pub fn read_intent(task_dir: &Path) -> std::io::Result<String> {
    match std::fs::read_to_string(task_dir.join(INTENT_FILE)) {
        Ok(intent) => Ok(intent),
        Err(err) if err.kind() == std::io::ErrorKind::NotFound => Ok(String::new()),
        Err(err) => Err(err),
    }
}

/// Read the persisted **minting workflow id** of a task from its working area
/// (`<task_dir>/workflow`) — the companion of the [`mint_task`] write, read back
/// on resume to compose the task's *own* workflow rather than the cascade default
/// (`DECISIONS.md` 2026-06-01 → M2 Increment 3 re-cut). A missing file yields
/// [`None`] (the clear absent case the CLI resume site maps to a routed "no
/// recorded workflow" error — never a silent fall-through to the default).
pub fn read_workflow_id(task_dir: &Path) -> std::io::Result<Option<String>> {
    match std::fs::read_to_string(task_dir.join(WORKFLOW_FILE)) {
        Ok(id) => Ok(Some(id)),
        Err(err) if err.kind() == std::io::ErrorKind::NotFound => Ok(None),
        Err(err) => Err(err),
    }
}

/// The **foreign source a migration task recorded** at mint (`jigc migrate`), read back
/// from its working area — and the only shape that read comes out in
/// (`design/auto-migration.md` → Retire-the-foreign-original; M51 Increment 1 / T3,
/// `completions/artifacts/M51/settle-record.md` → §2, the sink).
///
/// **It is a type rather than a `String` because of what the value *is*:** the path
/// `jigc task finalize --approve` **deletes**. It reaches that deletion from a plain file
/// in a mutable, gitignored working area (`.jigc/tasks/<id>/source-path`) one commit
/// closure after the door that adjudicated it, so at the sink it is **caller-supplied
/// again** — a door-only guard guards the typing, not the unlink. Keeping the raw read
/// private to [`read_migration_source`] is what makes that structural instead of
/// remembered: no consumer can hand a bare string to a destructive op, because no consumer
/// can obtain one.
///
/// What this type asserts is **provenance, not admissibility**: these bytes were recorded
/// as a migration source and are non-empty. The admissibility question — *may this
/// repository unlink that path?* — is asked CLI-side, immediately before the unlink and in
/// the same function as it, and its answer is `cli::task::ValidatedRetirement`. The engine
/// cannot ask it: the predicate needs `git` and a symlink syscall, and the engine hosts
/// neither (`crates/engine` contains no `Command::new` and no `symlink_metadata`).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MigrationSource {
    /// The recorded spelling, trimmed.
    recorded: String,
}

impl MigrationSource {
    /// The recorded spelling — the form the carryover gate's retire exemption and the
    /// conflict route name, and the form the CLI sink re-adjudicates. A *reference* to a
    /// path, never a licence to act on one.
    #[must_use]
    pub fn recorded(&self) -> &str {
        &self.recorded
    }

    /// Its [`crate::store::lexical_normalize`]d form — the comparison form both retire-side
    /// guards use, so a `./`-prefixed or redundant-component spelling still matches the
    /// canonical destination (review C1/F2). Derived here rather than at each guard, so the
    /// two cannot normalize differently.
    #[must_use]
    pub fn normalized(&self) -> PathBuf {
        crate::store::lexical_normalize(Path::new(&self.recorded))
    }
}

/// Read the persisted foreign source path of a migration task from its working area
/// (`<task_dir>/source-path`) as a [`MigrationSource`].
///
/// A missing file yields [`None`] — the clear non-migration case (the retire set is then
/// empty), never an error. So does a **blank** recording: an empty or whitespace-only value
/// names no file, and every consumer already spelled that test inline. Folding it here is
/// what makes "is this a migration task?" one question with one answer.
pub fn read_migration_source(task_dir: &Path) -> std::io::Result<Option<MigrationSource>> {
    match std::fs::read_to_string(task_dir.join(SOURCE_PATH_FILE)) {
        Ok(path) => Ok(Some(path.trim().to_string())
            .filter(|recorded| !recorded.is_empty())
            .map(|recorded| MigrationSource { recorded })),
        Err(err) if err.kind() == std::io::ErrorKind::NotFound => Ok(None),
        Err(err) => Err(err),
    }
}

/// Read the persisted **slug override** of a migration task from its working
/// area (`<task_dir>/slug-override`) — the `--slug` value `jigc migrate`
/// recorded at mint, fed to the gated create so the target doc's id is the
/// override verbatim, decoupled from the authored title (M43 Inc 6 T2). A
/// missing file yields [`None`] — the slug-less / non-migration case (the id
/// derives from the title as ever), never an error.
pub fn read_slug_override(task_dir: &Path) -> std::io::Result<Option<String>> {
    match std::fs::read_to_string(task_dir.join(SLUG_OVERRIDE_FILE)) {
        Ok(slug) => Ok(Some(slug)),
        Err(err) if err.kind() == std::io::ErrorKind::NotFound => Ok(None),
        Err(err) => Err(err),
    }
}

/// Mark a freshly minted task as an **amend** task, recording the full sha of the commit
/// its `git commit --amend` will rewrite ([`AMEND_PIN_FILE`]).
///
/// Written by `jigc task amend` immediately after the mint, so an area either carries the
/// marker from the moment it exists or never does — there is no window in which a
/// half-minted amend area reads as an ordinary task that `jigc task finalize` would commit
/// normally.
pub fn write_amend_pin(task_dir: &Path, sha: &str) -> std::io::Result<()> {
    std::fs::write(task_dir.join(AMEND_PIN_FILE), sha)
}

/// Read a task's **amend marker** — the sha of the commit `finalize`'s amend arm rewrites,
/// or [`None`] for every ordinary task ([`AMEND_PIN_FILE`]).
///
/// The single source of truth for *is this an amend task?*: `finalize` branches on it, and
/// so does the carryover gate's exemption. A **blank** recording yields `None` on
/// [`read_migration_source`]'s precedent — an empty marker names no commit, and reading it
/// as "amend, of nothing" would take the amend arm with no sha to compare HEAD against.
pub fn read_amend_pin(task_dir: &Path) -> std::io::Result<Option<String>> {
    match std::fs::read_to_string(task_dir.join(AMEND_PIN_FILE)) {
        Ok(sha) => Ok(Some(sha.trim().to_string()).filter(|sha| !sha.is_empty())),
        Err(err) if err.kind() == std::io::ErrorKind::NotFound => Ok(None),
        Err(err) => Err(err),
    }
}

/// Read the persisted [`BasePin`] of a task from its working area
/// (`<task_dir>/base.json`) — the companion of [`mint_task`]'s pin write, read
/// back on resume to compare against the current checkout. A missing or malformed
/// pin is an error (the pin is written at mint, so its absence is a real fault).
pub fn read_base_pin(task_dir: &Path) -> std::io::Result<BasePin> {
    let bytes = std::fs::read(task_dir.join(BASE_PIN_FILE))?;
    serde_json::from_slice(&bytes)
        .map_err(|err| std::io::Error::new(std::io::ErrorKind::InvalidData, err))
}

/// The staged-snapshot filename inside a task's (or milestone's) working area —
/// the pre-task staged state the carryover gate compares finalize's index against
/// (`design/surface-contract.md` → The carryover gate; M43 T1).
const STAGED_SNAPSHOT_FILE: &str = "staged-snapshot.json";

/// The staged state of the repo's index **at the moment a task-minting door ran**
/// — what was `git add`ed / `git rm`ed *before this task existed*, snapshotted so
/// finalize can refuse to let a foreign pre-staged change silently ride the task's
/// commit (`design/surface-contract.md` → The carryover gate).
///
/// Two halves, because an entry-only snapshot is structurally blind to a staged
/// deletion (the trial's A7 case): `entries` carries the index entries that differ
/// from HEAD (adds + modifications) as `path → staged blob hash`, and `deletions`
/// carries the HEAD paths absent from the index (a pre-task `git rm`).
///
/// `BTreeMap`/`BTreeSet` so the serialized JSON is key-sorted and deterministic —
/// the byte form is golden-stable, the same convention as `base.json` /
/// `roles.json`. A **missing** snapshot file reads as [`None`] and the gate fails
/// open (a task minted pre-M43 finalizes as today — the declared bound), so the
/// record carries no schema version: it is working-area state, disposable with
/// the task.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct StagedSnapshot {
    /// `path → staged blob hash` for every index entry differing from HEAD
    /// (staged adds + modifications).
    pub entries: std::collections::BTreeMap<String, String>,
    /// The staged-deletion set: HEAD paths absent from the index (`git rm`).
    pub deletions: std::collections::BTreeSet<String>,
}

/// What a [`MintDoor`] does about the carryover gate's staged snapshot.
pub enum Snapshot {
    /// The door writes `staged-snapshot.json` into the area it mints
    /// ([`write_staged_snapshot`]), so a committing boundary downstream of it can
    /// tell *staged before this work-unit existed* from the unit's own staging.
    Written,
    /// The door writes **no** snapshot, carrying the reason it does not owe one —
    /// asserted through the binary by the driven arm, never taken on trust.
    Exempt(&'static str),
}

/// One production **working-area mint** — a call site of [`mint_task`] or
/// [`crate::milestone::mint_milestone`], paired with the door an operator reaches it
/// by and its snapshot disposition.
pub struct MintDoor {
    /// The door as an operator names it — the argv shape that reaches this mint.
    pub door: &'static str,
    /// The production call site, `<workspace-relative path>::<enclosing fn>`. This is
    /// the key the source-level completeness fence matches on, so a mint added
    /// anywhere in either crate is a red test rather than a silent sixth door.
    pub site: &'static str,
    /// The mint this site calls — `mint_task` or `mint_milestone`.
    pub mint: &'static str,
    /// Whether this door snapshots the pre-mint staged state, or is exempt with a
    /// stated reason.
    pub snapshot: Snapshot,
}

/// The **mint-door axis** — every production call that opens a working area, with its
/// staged-snapshot disposition (M49 Increment 12 / T1).
///
/// It exists because the disposition was a **remembered list** and the memory was
/// wrong: [`write_staged_snapshot`]'s doc-comment said *"written at every task-minting
/// door"* and then named three, while five production sites mint an area. The two it
/// never named ([`crate::milestone::add_task`],
/// [`crate::milestone::reseed_sub_task_areas`]) are not a hole — they are **exempt**,
/// and the exemption is stated here rather than left as an absence, which is the
/// difference between a disposition and an oversight
/// (`completions/artifacts/M49/settle-record.md` → the carryover refutation's residue).
///
/// One list, two consumers, both in `crates/cli/tests/mint_doors.rs`: a **source-level
/// completeness fence** over both crates' production code — the call-site set of
/// [`mint_task`] ∪ [`crate::milestone::mint_milestone`] must equal the `site` set below
/// — and **one driven cell per member** through the real binary, where a member with no
/// cell is a hard panic rather than a skip. A grep is not a fence
/// (`implementation/dev-workflow.md`): the sweep that *found* these five cannot stop the
/// sixth, so membership is checked where membership is decided.
pub const MINT_DOORS: &[MintDoor] = &[
    MintDoor {
        door: "jigc start \"<intent>\"",
        site: "crates/cli/src/start.rs::mint_in_repo",
        mint: "mint_task",
        snapshot: Snapshot::Written,
    },
    MintDoor {
        door: "jigc migrate <path> --as <doctype>",
        site: "crates/cli/src/start.rs::mint_migration_in_repo",
        mint: "mint_task",
        snapshot: Snapshot::Written,
    },
    MintDoor {
        door: "jigc task amend [\"<intent>\"]",
        site: "crates/cli/src/start.rs::mint_amend_in_repo",
        mint: "mint_task",
        snapshot: Snapshot::Written,
    },
    MintDoor {
        door: "jigc milestone create \"<title>\"",
        site: "crates/cli/src/milestone.rs::run_create",
        mint: "mint_milestone",
        snapshot: Snapshot::Written,
    },
    MintDoor {
        door: "jigc milestone add-task <milestone> \"<intent>\"",
        site: "crates/engine/src/milestone.rs::add_task",
        mint: "mint_task",
        snapshot: Snapshot::Exempt(
            "a sub-task's own area is never a committing boundary: `jigc task finalize \
             <sub>` refuses a milestone sub-task first (`finalize.milestone-sub-task`), \
             so no door consumes a snapshot written here, and the aggregate boundary \
             gates on the MILESTONE area's snapshot instead.",
        ),
    },
    MintDoor {
        door: "any operating milestone op on a fresh clone \
               (`jigc milestone add-task` / `provision` / `execute` / `join` / \
               `finalize` / `discard`) — the record-driven re-seed",
        site: "crates/engine/src/milestone.rs::reseed_sub_task_areas",
        mint: "mint_task",
        snapshot: Snapshot::Exempt(
            "the same premise as `add_task` — the area it rebuilds is a sub-task's, and \
             the per-task finalize refuses one before any gate runs. It is also a \
             REBUILD of an area the committed record already names, not a door a user \
             staged work in front of, so there is no pre-mint index state that belongs \
             to it.",
        ),
    },
];

/// Write the staged snapshot into a working area (`<dir>/staged-snapshot.json`)
/// — the base-pin mold: pretty JSON, key-sorted (the `BTreeMap`/`BTreeSet` field
/// types), one trailing newline (golden-locked frozen on-disk form). A clean index
/// writes an **empty** snapshot, distinct from the absent pre-M43 case
/// [`read_staged_snapshot`] maps to `None`.
///
/// **Which doors call this is [`MINT_DOORS`], not a sentence here.** This comment
/// used to read *"written at every task-minting door"* and then name three, while
/// five production sites mint a working area — the two it omitted being exempt, but
/// nowhere stated as such (M49 Increment 12 / T1).
pub fn write_staged_snapshot(dir: &Path, snapshot: &StagedSnapshot) -> std::io::Result<()> {
    let mut body = serde_json::to_string_pretty(snapshot).expect("StagedSnapshot serializes");
    body.push('\n');
    std::fs::write(dir.join(STAGED_SNAPSHOT_FILE), body)
}

/// Read the persisted [`StagedSnapshot`] of a working area, the companion of
/// [`write_staged_snapshot`]. A missing file yields [`None`] — the declared
/// fail-open bound (a task/milestone minted before the carryover gate existed
/// finalizes as today); a present-but-malformed file is a real fault and errors.
pub fn read_staged_snapshot(dir: &Path) -> std::io::Result<Option<StagedSnapshot>> {
    match std::fs::read(dir.join(STAGED_SNAPSHOT_FILE)) {
        Ok(bytes) => serde_json::from_slice(&bytes)
            .map(Some)
            .map_err(|err| std::io::Error::new(std::io::ErrorKind::InvalidData, err)),
        Err(err) if err.kind() == std::io::ErrorKind::NotFound => Ok(None),
        Err(err) => Err(err),
    }
}

/// Slug the id-source into the task id, applying the empty → type-name fallback.
///
/// The pure normalization is [`crate::slug::slugify`]; the fallback is the mint
/// site's concern (only the caller knows the type name), per the slug rule
/// (`DECISIONS.md` 2026-05-31 → Slug / minting normalization).
///
/// **The fallback is production-dead since M53 Increment 5, and it stays — a decision,
/// not an oversight** (`completions/artifacts/M53/settle-record.md` → **D5** as amended
/// by **§11**). Both call sites are guarded ahead of it, so no production path reaches
/// the empty branch: [`mint_task`] is called by `jigc start`'s mint and by
/// [`crate::milestone::add_task`], each of which asks [`reject_unslugable_title`] first
/// — that second site covering `jigc milestone add-task` and `add-from-spec` alike — and
/// by the two [`MINT_DOORS`] rows that pass a `slug_override` (the migrate mint's path
/// hash, the re-seed's recorded id), which never ask this at all; [`mint_instance`]
/// refuses its own empty-slugging title one branch earlier, under `create.empty-title`.
///
/// **It stays because removing it costs a contract and buys no behaviour.** `mint_id`
/// is *total* — every `&str` yields an id — and that totality is what its two tests pin,
/// in halves: `whitespace_intent_falls_back_to_type_name` the empty input, the
/// `mint_id_is_slugify_for_non_empty` proptest every other. Dropping the branch makes the
/// function partial and pushes an `Option` into two call sites that, behind the guards,
/// can no longer be handed the input it would report. A defensive floor under a guarded
/// door is the cheaper shape.
fn mint_id(intent: &str, type_name: &str) -> String {
    let slug = crate::slug::slugify(intent);
    if slug.is_empty() {
        crate::slug::slugify(type_name)
    } else {
        slug
    }
}

/// The **mint class's** refusal: the caller's title carries nothing an id can be built
/// from, so the door refuses before it writes rather than minting the work unit at a
/// fabricated identity (M53 Increment 5 / D5; `completions/artifacts/M53/settle-record.md`
/// → **D5**, as amended by **§11**).
///
/// **One producer for the class, because the class is one condition.** Every [`MINT_DOORS`]
/// row but two derives its id from the caller's prose through `mint_id`, whose empty→
/// type-name fallback would otherwise hand the work a `task` / `milestone` id nobody typed —
/// an identity a second such call then serial-collides. The exempt pair derives its id from
/// something other than a title (a path hash, a recorded id) and never asks. `jigc start`'s
/// mint is the door raising it here; each other door asks the same question at its own seam,
/// before its first write, and gets this same sentence for it.
///
/// **Parameterised on the work-unit type token alone** (`task` / `milestone`) — which is
/// also the **target**. There is no `milestone:<?>` to address: no unit exists and none is
/// going to, exactly as for a doctype-scoped `create` block, so the key is the **bare
/// token**, the form `create.empty-title` takes
/// ([command-output-contract.md](../../../design/command-output-contract.md) → the form
/// table, the doctype-scoped-blocks row). A synthesized `<type>:<invented-slug>` would name
/// a unit a driver could try to read back.
///
/// **The code is `write.unslugable-title`**, the class's shipped identity — its third
/// producer, beside `jigc rename`'s `--to` guard and the item-title write. The charter's
/// `work-unit.malformed-id` is *inert* here (`mint_id("")` yields a well-formed slug) and
/// *false* as a repair (an id grammar is no answer to free prose).
///
/// **The sentence is true for a title in any script**, which is the half the pre-D5 bail got
/// wrong: it said *"intent must contain at least one letter or digit"* of `"日本語"`. The
/// cell is ordinary, not adversarial — every non-Latin title and every stopword-only title
/// reaches it.
///
/// **Private, and that is the one-producer rule expressed in the type system** (M53
/// completion audit, finding 6). It shipped `pub` while its only caller anywhere in the
/// workspace was [`reject_unslugable_title`], one screen below — measured, not assumed: of
/// the 31 `pub fn` this module declares, it is the **only** one with no code reference
/// outside `state.rs`. A `pub` constructor beside a `pub` guard advertises a second way to
/// raise this code that does not ask the condition, which is exactly the shape D5 exists to
/// prevent; the guard stays `pub` because `cli` calls it, and the identity it raises is now
/// unreachable except through it.
fn unslugable_title_finding(work_unit: &str, also: Option<&str>) -> Finding {
    let mut route = format!(
        "re-run with a title carrying ASCII letters or digits — the {work_unit} id is \
         slugged from it"
    );
    if let Some(also) = also {
        route.push_str(", or ");
        route.push_str(also);
    }
    Finding::graded(
        Severity::Blocking,
        "write.unslugable-title",
        format!(
            "cannot mint a {work_unit}: its id is slugged from the title, and this title \
             slugs to nothing — ids are built from ASCII letters and digits, so a title in \
             another script, or of stopwords only, yields none"
        ),
        Some(Location::addressed(work_unit, 1, 1)),
        Some(route.into()),
    )
}

/// The mint class's **guard** — the one question every door whose id is slugged from the
/// caller's prose asks, at its own seam, before its first write (M53 Increment 5 / T3;
/// `completions/artifacts/M53/settle-record.md` → **D5** as amended by **§11**, **§12**).
///
/// **One predicate, not four**, because the condition is one condition: the caller's title
/// carries nothing [`crate::slug::slugify`] keeps, so [`mint_id`]'s empty→type-name
/// fallback would hand the work unit a `task` / `milestone` id nobody typed. Driven at
/// `a53bc0a1~`, `jigc milestone create "日本語"` minted `milestone:milestone` and committed
/// a record for it at exit 0; the sibling door committed `task:task`. Each door asks this
/// here and raises `unslugable_title_finding`'s single sentence for it, so the class
/// cannot acquire a second wording the way the pre-D5 `jigc start` bail did. That producer
/// is **private to this module** — a plain code span above rather than an intra-doc link,
/// because it is deliberately not part of the crate's surface — so this guard is the only
/// way the identity can be raised at all.
///
/// **Where a door asks it is the door's own decision, and it is not "first".** The seam is
/// *before the first write*, which at `jigc milestone create` is after the two reads that
/// establish where jigc is standing — otherwise a caller outside a repository is told about
/// their title instead of getting M49's converged not-in-repo answer (§12).
///
/// **`also` is the door's own extra exit, and it exists because one door has one** (M53,
/// the rc.20 per-axis review `(3, F-C)`). The shared advice — *re-run with a title carrying
/// ASCII letters or digits* — is the whole exit set at every [`MINT_DOORS`] row whose
/// registry spelling makes the title **required**. `jigc task amend ["<intent>"]` brackets
/// it: omit the intent and the task is named after the commit it rewrites
/// (`amend-<sha7>`), a mint the shipped route never named, because `MINT_DOORS` grew a
/// sixth member with a wider exit set and the shared route was not re-derived over it.
/// Passing it here rather than editing the finding at the door keeps the class's **one
/// sentence, one producer** rule — a second wording site is exactly what the pre-D5 `jigc
/// start` bail was — while letting a door state an exit only it has. `None` is the answer
/// for every door that has none, and it is stated rather than defaulted.
pub fn reject_unslugable_title(
    work_unit: &str,
    title: &str,
    also: Option<&str>,
) -> Result<(), Finding> {
    if crate::slug::slugify(title).is_empty() {
        return Err(unslugable_title_finding(work_unit, also));
    }
    Ok(())
}

/// Render the base-pin file body — the frozen on-disk form (golden-locked).
fn render_base_pin(base: &BasePin) -> String {
    // Pretty JSON with a trailing newline; field order is the struct order
    // (`sha` then `short`), pinned by the golden.
    let mut s = serde_json::to_string_pretty(base).expect("BasePin serializes");
    s.push('\n');
    s
}

/// The serial-collision block: a blocking finding naming the existing task and
/// routing the agent to resume or discard (`write-commands.md` → Task-id collision
/// & resume).
fn collision_finding(id: &str) -> Finding {
    Finding::graded(
        Severity::Blocking,
        "task.serial-collision",
        format!("task `{id}` is already active"),
        Some(Location::addressed(format!("task:{id}"), 1, 1)),
        Some(
            format!(
                "resume with `jigc start --task {id}` or abandon with \
                 `jigc task discard {id} --force`"
            )
            .into(),
        ),
    )
}

/// The serial-collision block over a **residual**: the same code and the same stable key as
/// [`collision_finding`], because the state is the same state — *the id is taken on disk* —
/// and a driver keying on `(task.serial-collision, task:<id>)` must not learn a second pair
/// to see this cell (M53 Increment 3 / T4; `settle-record.md` → D3, *What the mint answers*).
///
/// What differs is what is *there*, so what differs is the sentence and the route. The mint
/// keeps **refusing**: adopting the directory would write jigc's files in beside bytes
/// nobody has looked at, at a door that could not do that before.
fn residual_collision_finding(id: &str, listed: &str) -> Finding {
    Finding::graded(
        Severity::Blocking,
        "task.serial-collision",
        format!(
            "cannot mint task `{id}`: {}",
            residual_area_note(listed, "task")
        ),
        Some(Location::addressed(format!("task:{id}"), 1, 1)),
        Some(residual_area_route(listed)),
    )
}

/// The task's **bound context roles** — a map from a workflow-declared role name
/// (e.g. `decision`) to the `<type>:<slug>` address the agent bound to it
/// (`write-commands.md` → Task origination: "A task carries context roles its
/// workflow declares; the agent binds them explicitly … the CLI never infers a
/// binding"). Persisted at `.jigc/tasks/<id>/roles.json` so a re-composed
/// `jigc start --task <id>` resolves `task.<role>` to the bound doc
/// (`DECISIONS.md` 2026-05-31 → inc-5 `as:` role binding at create).
///
/// A [`BTreeMap`](std::collections::BTreeMap) so the serialized JSON is
/// **key-sorted and deterministic** — the byte form is golden-stable, the same
/// convention as `base.json` / `file-state.json`. The record carries no schema
/// version of its own: it is task-local working-area state, disposable with the
/// task, not a committed contract surface.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct RolesRecord {
    /// `role → <type>:<slug>` address, role-sorted for deterministic output.
    pub roles: std::collections::BTreeMap<String, String>,
}

impl RolesRecord {
    /// An empty record.
    pub fn new() -> Self {
        Self::default()
    }

    /// The record's on-disk location inside a task's working area.
    pub fn path_in(task_dir: &Path) -> PathBuf {
        task_dir.join(ROLES_FILE)
    }

    /// Bind `role` to `address` (overwriting any prior binding for that role).
    pub fn bind(&mut self, role: impl Into<String>, address: impl Into<String>) {
        self.roles.insert(role.into(), address.into());
    }

    /// The address bound to `role`, if any.
    pub fn get(&self, role: &str) -> Option<&str> {
        self.roles.get(role).map(String::as_str)
    }

    /// Serialize to the frozen on-disk byte form: pretty JSON, role-sorted, one
    /// trailing newline (golden-locked, matching the `base.json` convention).
    pub fn to_bytes(&self) -> String {
        let mut s = serde_json::to_string_pretty(self).expect("RolesRecord serializes");
        s.push('\n');
        s
    }

    /// Save the record atomically to `<task_dir>/roles.json` (temp + rename, the
    /// shared working-area atomic-write primitive).
    pub fn save(&self, task_dir: &Path) -> std::io::Result<()> {
        write_atomic(&Self::path_in(task_dir), self.to_bytes().as_bytes())
    }

    /// Load the record from `<task_dir>/roles.json`. A missing file is the
    /// *no-roles-bound-yet* case — a task that has bound nothing — and yields an
    /// empty record, never an error (mirrors [`FileStateRecord::load`]'s
    /// absent-is-empty contract).
    pub fn load(task_dir: &Path) -> std::io::Result<Self> {
        match std::fs::read(Self::path_in(task_dir)) {
            Ok(bytes) => Ok(serde_json::from_slice(&bytes)?),
            Err(err) if err.kind() == std::io::ErrorKind::NotFound => Ok(Self::new()),
            Err(err) => Err(err),
        }
    }
}

/// The task's **pre-rename title log** — every `# H1` this task has renamed a staged
/// doc *away from*, oldest first (`jigc doc rename <addr> --to … --task <id>`).
/// Persisted at `.jigc/tasks/<id>/renames.json`, beside [`RolesRecord`]'s `roles.json`.
///
/// **Why it has to be durable.** A rename moves a doc's title; the task's staged
/// `commit` doc may already carry the old one in its summary. The producer that notices
/// that runs in a **later invocation** — the task-scope sweep at `jigc task validate
/// <id>` and `task finalize --dry-run` — and, for jigc's own `pre-commit` hook's nested
/// `jigc validate`, in a **different process**. So the pre-rename title cannot live in
/// the renaming process's memory, and the ack that prints it once is a cue card of the
/// shape the trials measured failing (`completions/artifacts/RC-1.0-gate/cue-card-postmortem.md`).
///
/// **The log is history, so it is a `Vec`, not a key-sorted map** — the sibling records
/// ([`RolesRecord`], [`ProvenanceRecord`]) are `BTreeMap`s because a *key* has one
/// current value; here the order is the content. A title renamed away from twice is
/// recorded twice. Only the **titles** are recorded, never the pre-rename address: an
/// in-task re-slug's own fence is that nothing under the task area still names the old
/// identity (`crates/cli/tests/doc_rename_in_task.rs` → the derived-set fence).
///
/// **Concurrent-writer disposition** (`design/storage.md` → Concurrent writers): this
/// is **task-area** state, isolated by task id exactly as `roles.json` and the staged
/// `.md` bodies are, so it is **not** a fourth row of that section's shared-writer table
/// (`file-state.json` / `edges.json` / `tasks.json`) — a fan-out's sub-agents each own
/// their own `tasks/<sub>/` area and no two of them write this file. It therefore needs
/// neither the base-relative merge nor the save-scoped lock: it is written through the
/// shared temp-sibling + `rename` primitive ([`persist`]) purely so a reader in another
/// process never observes a partial file, and nothing more. The record is disposable
/// with the task, so it carries no schema version of its own.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct RenameRecord {
    /// Every `# H1` this task has renamed a staged doc away from, in the order the
    /// renames happened.
    pub pre_rename_titles: Vec<String>,
}

impl RenameRecord {
    /// The record's on-disk location inside a task's working area.
    pub fn path_in(task_dir: &Path) -> PathBuf {
        task_dir.join(RENAMES_FILE)
    }

    /// Serialize to the on-disk byte form: pretty JSON, one trailing newline (the
    /// `roles.json` / `base.json` convention).
    fn to_bytes(&self) -> String {
        let mut s = serde_json::to_string_pretty(self).expect("RenameRecord serializes");
        s.push('\n');
        s
    }

    /// Load the record from `<task_dir>/renames.json`. A missing file is the
    /// *nothing-renamed-yet* case — the overwhelmingly common one — and yields an empty
    /// record, never an error (the absent-is-empty contract [`RolesRecord::load`] and
    /// [`ProvenanceRecord::load`] both keep).
    pub fn load(task_dir: &Path) -> std::io::Result<Self> {
        match std::fs::read(Self::path_in(task_dir)) {
            Ok(bytes) => Ok(serde_json::from_slice(&bytes)?),
            Err(err) if err.kind() == std::io::ErrorKind::NotFound => Ok(Self::default()),
            Err(err) => Err(err),
        }
    }
}

/// Append `pre_rename_title` to the task's rename log and persist it atomically — the
/// stage-time write `jigc doc rename` performs **after** the retitled body lands, so a
/// refused rename logs nothing.
///
/// The caller owns the *whether*: a rename that moved no `# H1` records nothing (see
/// `cli::doc::run_doc_rename`, the one producer). This primitive owns only the append,
/// mirroring [`record_doc_provenance`].
pub fn record_pre_rename_title(task_dir: &Path, pre_rename_title: &str) -> std::io::Result<()> {
    let mut record = RenameRecord::load(task_dir)?;
    record.pre_rename_titles.push(pre_rename_title.to_string());
    write_atomic(
        &RenameRecord::path_in(task_dir),
        record.to_bytes().as_bytes(),
    )
}

/// A freshly created doc instance: its minted `address` (`<type>:<slug>`) and the
/// on-disk `path` of the staged instance in the working area.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct CreatedDoc {
    /// The minted address — `<type>:<slug>` (no fragment; the whole container).
    pub address: String,
    /// The staged instance path, `<task_dir>/docs/<type>:<slug>.md`.
    pub path: PathBuf,
    /// The create-or-update discriminator (M43 inc-7 T1): `true` when a committed
    /// instance already occupied the slug's canonical home and was **copied in for
    /// update** ([`copy_in`], `edited-from-base`); `false` on a fresh mint. Feeds the
    /// `doc create` ack's always-present `existed` key
    /// (`design/command-output-contract.md` §2).
    pub existed: bool,
    /// The **staged pre-image** — the working-area file's bytes as this call found them,
    /// or `None` when `path` did not exist before the call. It is what makes a create
    /// undoable *correctly* ([`CreatedDoc::rollback`]), and it is **not** the same
    /// discriminator as [`existed`](Self::existed): a committed copy-in reports
    /// `existed: true` while still *provisioning* the staged file, so only the
    /// same-identity **staged** copy (M45 Inc 5 T2 — `create_gated` hands back the file
    /// it found and writes nothing) carries a pre-image.
    ///
    /// The capture lives here, at the seam that decides which branch ran, rather than in
    /// each caller: a caller cannot re-derive the minted path without duplicating
    /// [`mint_instance`]'s slug precedence, and "did this call create the file?" is
    /// exactly the question a caller has no other way to answer.
    pub staged_pre_image: Option<Vec<u8>>,
}

impl CreatedDoc {
    /// **Undo what this create did to the working area** — restore the captured
    /// [`staged_pre_image`](Self::staged_pre_image), or remove `path` when the create
    /// provisioned it (a fresh mint or a committed copy-in, whose pre-image is "absent"
    /// and whose removal therefore restores exactly the pre-call state — the committed
    /// source is untouched either way, [`copy_in`] never writes it).
    ///
    /// The caller is a multi-step write that persisted the create and then failed
    /// (`doc author`'s leaf chain): the batch promises "persisted nothing", so the empty
    /// doc it minted must not leak. **But an unconditional `remove_file` here is data
    /// loss**: over a same-identity staged copy the create provisioned nothing, so the
    /// removal deletes the editing session's prior work — silently, on the very path the
    /// reject's route invites the agent to re-run (M47 Increment 6, the triage fix). It is
    /// the same **captured-pre-image** discipline finalize's staged-path families follow
    /// (`design/finalize.md` → Rollback discipline), applied one layer down, to the task
    /// working area: restore what was found, never assume the call created it.
    ///
    /// Best-effort, like every sibling rollback: the write did **not** land, so a restore
    /// failure must not replace the caller's real (blocking) finding.
    pub fn rollback(&self) {
        match &self.staged_pre_image {
            Some(bytes) => {
                let _ = std::fs::write(&self.path, bytes);
            }
            None => {
                let _ = std::fs::remove_file(&self.path);
            }
        }
    }
}

/// The minted identity of a create — the frozen `slug`, the display `title`, the
/// `<type>:<slug>` `address`, and the working-area `path`. Factored out of
/// [`create`] so the slug/title/path derivation has **one** source shared with
/// [`create_gated`], which probes `path` to ack a **same-identity staged copy for
/// update** rather than reject it with `create.serial-collision` (M45 Inc 5 T2;
/// `DECISIONS.md` 2026-07-23 M45 planning → Fork 2). Applies the empty-title guard,
/// so both callers reject a slugs-to-nothing title identically.
struct MintedInstance {
    slug: String,
    title: String,
    address: String,
    path: PathBuf,
}

/// Derive the [`MintedInstance`] for a create against `task_dir` — the shared
/// slug/title/address/path computation (`create` step 2). The empty-title guard,
/// the `singleton` / `slug_override` / `mint_id` slug precedence, and the H1 display
/// text all live here so [`create`] and [`create_gated`] never diverge.
fn mint_instance(
    task_dir: &Path,
    schema: &Schema,
    type_name: &str,
    id_source: &str,
    slug_override: Option<&str>,
) -> Result<MintedInstance, Finding> {
    // A non-singleton create derives its stable id from the title; a title that
    // slugs to nothing would fall to `mint_id`'s type-name fallback and mint a
    // degenerate `<ty>:<ty>` (e.g. `adr:adr` from `--title ""`). Reject up front,
    // routing to a non-empty title — the engine-side mirror of `rename`'s
    // slug-derivation guard. A `singleton` fixes its slug to the type id (no title
    // to derive), so it is untouched; a `slug_override` supplies the id explicitly,
    // so the guard is inert then.
    if !schema.singleton && slug_override.is_none() && crate::slug::slugify(id_source).is_empty() {
        return Err(empty_title_finding(type_name));
    }
    // Mint the frozen content-slug. A `singleton` doctype fixes the slug to the type
    // id unconditionally (so a re-create targets the same `<location>/<ty>.md`); a
    // non-singleton with a `slug_override` takes it **verbatim** (the front door's
    // `--slug`, validated at the CLI boundary via `is_slug`); otherwise it slugs the
    // (now guaranteed non-empty-slugging) id-source.
    let slug = if schema.singleton {
        schema.ty.clone()
    } else if let Some(slug) = slug_override {
        slug.to_string()
    } else {
        mint_id(id_source, type_name)
    };
    // The `# H1` display text — the **human** id-source verbatim (`Use MySQL`, not the
    // `use-mysql` slug), so the committed artifact reads as a title, not a filename. A
    // `singleton` has no free title, so its H1 is the schema's [`Schema::fixed_title`]
    // (the `display-title:` knob when declared — `vision` → `# Vision` — else the fixed
    // type id, which is the slug). An id-source that slugs empty keeps H1 == slug — the
    // type-name fallback fired, so the slug stands in.
    let title = if let Some(fixed) = schema.fixed_title() {
        fixed
    } else if crate::slug::slugify(id_source).is_empty() {
        slug.clone()
    } else {
        id_source.to_string()
    };
    let address = format!("{type_name}:{slug}");
    let path = instance_path(task_dir, type_name, &slug);
    Ok(MintedInstance {
        slug,
        title,
        address,
        path,
    })
}

/// **The `create`/provisioning verb** against the task working area — the
/// structural act the CLI always owns (`design/write-commands.md` → Instance
/// provisioning: "mint the id … and place it at the schema-defined location";
/// `design/structural-grammar.md` → IDs: provenance and minting). This is the
/// **workflow-provisioned** path (the gate is bypassed; see [`create_gated`] for
/// the agent-initiated path that consults `allows-create` — and which, under an entry
/// carrying `new: true`, refuses an occupied home instead of reaching step 4 here):
///
/// 1. **Unknown doctype** — `type_name` not in the cascade-resolved `schemas` set
///    → reject with a blocking `create.unknown-doctype` [`Finding`]
///    (`write-commands.md` → The create-gate, enforcement step 3: "fires before the
///    gate check"). Nothing is created.
/// 2. **Mint** the frozen content-slug from `id_source` (empty → the type-name
///    fallback, the same [`mint_id`] discipline as a task), yielding the address
///    `<type>:<slug>`.
/// 3. **Serial collision** — an instance of that id already exists in the working
///    area → reject with a blocking `create.serial-collision` [`Finding`] carrying a
///    route, never silently suffixed (`structural-grammar.md` → minting: the numeric
///    suffix is the post-MVP `fan-out`/`join` case only; serial mints reject).
/// 4. **Idempotent create-or-update for a committed instance** (`methodology-docs.md`
///    → The engine work, item 2; review findings B-5/I-2; doctype-blind since M43
///    inc-7 T1 — `DECISIONS.md` → 2026-07-16 M43 planning: the Settle, review-baked).
///    When the minted slug's **committed** canonical file already exists under
///    `repo_root`, `create` **copies that committed body in** via [`copy_in`]
///    (recording `edited-from-base`, [`CreatedDoc::existed`] `true`) instead of
///    minting blank — killing the clobber-on-blank landmine and giving
///    create-or-update for singleton and non-singleton alike (pre-M43 the branch was
///    `singleton`-gated, so a non-singleton create-over-committed seeded blank and
///    ambushed at the finalize clobber gate). Copy-in does **not** reconcile
///    (review I-1) — OOB drift over the committed doc is caught at
///    finalize-preflight, not here. **Exception:** the in-location squatter
///    ([`migration_targets_canonical_destination`], `auto-migration.md` →
///    Hardening #8) — a migration task whose recorded `source-path` is this slug's
///    canonical destination seeds **blank** (skips the copy-in) so the author
///    sequence builds onto a clean skeleton, not the non-conformant foreign body.
/// 5. **Provision** the empty instance at `docs/<type>:<slug>.md` via
///    [`provision_doc`] and return its [`CreatedDoc`] address + path.
pub fn create(
    task_dir: &Path,
    schemas: &std::collections::BTreeMap<String, Schema>,
    type_name: &str,
    id_source: &str,
    repo_root: &Path,
    on_create: &[crate::field_block::Field],
    slug_override: Option<&str>,
) -> Result<CreatedDoc, Finding> {
    // 1. Unknown doctype → reject before anything is minted or placed.
    let Some(schema) = schemas.get(type_name) else {
        return Err(unknown_doctype_finding(type_name, schemas));
    };

    // 2. Mint the identity (slug/title/address/path) through the shared derivation,
    //    which applies the empty-title guard (a title that slugs to nothing rejects).
    let minted = mint_instance(task_dir, schema, type_name, id_source, slug_override)?;

    // The one probe of the minted identity's home ([`occupied_home`]); steps 3–5 act on
    // its answer and never ask again.
    let home = occupied_home(repo_root, schema, &minted.slug);
    stage_minted(task_dir, schema, type_name, minted, home, on_create)
}

/// **The minted identity's canonical home, when it is already a file on disk** — the one
/// predicate every create-side question about *"is there a doc there already?"* is
/// answered by: [`create`]'s copy-in, [`create_gated`]'s create-only refusal,
/// [`create_occupied`] (the CLI pre-check's ranked early ask of the same refusal) and
/// [`create_incumbent`]'s committed arm. `None` when the home is free, or the doctype has
/// none (a transient sink type). *On disk* is wider than *committed* — an untracked file
/// at the home is occupied too (`design/write-commands.md` → The create-gate).
///
/// **A create asks it once and acts on the answer it got** (the rc.24 review's
/// `(R6, K-1)`): [`create`] and [`create_gated`] call this a single time and hand the
/// *result* down to [`stage_minted`], so the refusal a create-only entry owes and the
/// copy-in a plain entry performs are two readings of **one** observation, never two
/// observations a writer can land between.
fn occupied_home(repo_root: &Path, schema: &Schema, slug: &str) -> Option<PathBuf> {
    crate::store::canonical_path(repo_root, schema, slug).filter(|home| home.is_file())
}

/// [`create`]'s steps 3–5 over an identity already minted and a home **already probed**
/// (`home` is [`occupied_home`]'s answer — `Some` when the canonical home is a file on
/// disk). It never probes the home itself: the caller that decided what an occupied home
/// means (copy in, or — under a create-only entry — refuse and never call this) hands its
/// one observation down, so that decision and this copy-in cannot disagree.
fn stage_minted(
    task_dir: &Path,
    schema: &Schema,
    type_name: &str,
    minted: MintedInstance,
    home: Option<PathBuf>,
    on_create: &[crate::field_block::Field],
) -> Result<CreatedDoc, Finding> {
    let MintedInstance {
        slug,
        title,
        address,
        path,
    } = minted;

    // 3. Serial collision in the working area → reject, never suffixed, nothing
    //    created. This is the **ungated** serial-mint reject (fan-out): the agent path
    //    never reaches here on a same-identity staged copy — [`create_gated`] acks
    //    `existed` and binds the role before calling `create` (M45 Inc 5 T2).
    if path.exists() {
        return Err(instance_collision_finding(&address));
    }

    // 4. Idempotent create-or-update: a committed instance at the slug's canonical
    //    path under `repo_root` is copied in for editing rather than minted blank
    //    (the B-5 clobber fix; doctype-blind since M43 inc-7 T1). Copy-in records
    //    `edited-from-base`; drift is finalize-preflight's concern, not copy-in's.
    //    **Exception — the in-location squatter** (`design/auto-migration.md` →
    //    Path-collision guard / Hardening #8): when a migration task's recorded
    //    `source-path` IS this slug's canonical destination, the committed body is the
    //    non-conformant foreign file being replaced, so seed **blank** (skip the
    //    copy-in) and let the author sequence build onto a clean skeleton. The
    //    discriminator is the source-path match — a non-migration create, and an
    //    off-canonical migration, both still copy in.
    let migration_squatter = migration_targets_canonical_destination(task_dir, schema, &slug)
        .map_err(|err| io_finding(&address, "read the migration source path", &err))?;
    if !migration_squatter && let Some(committed) = home {
        let body = std::fs::read_to_string(&committed)
            .map_err(|err| io_finding(&address, "read the committed instance", &err))?;
        let path = copy_in(task_dir, type_name, &slug, &body)
            .map_err(|err| io_finding(&address, "copy in the committed instance", &err))?;
        return Ok(CreatedDoc {
            address,
            path,
            existed: true,
            // Step 3 above rejected a pre-existing working-area path, so this copy-in
            // provisioned the staged file itself: its pre-image is **absent**, and an
            // undo is the removal of what this call wrote (the committed source is
            // never touched by `copy_in`).
            staged_pre_image: None,
        });
    }

    // 5. Provision the (optionally on-create-seeded) instance and return its
    //    address + path. The seam is additive: an empty `on_create` slice provisions
    //    the unchanged empty template.
    let path = provision_doc(task_dir, schema, &slug, &title, on_create)
        .map_err(|err| io_finding(&address, "provision the instance", &err))?;
    Ok(CreatedDoc {
        address,
        path,
        existed: false,
        // A fresh mint: step 3 rejected a pre-existing path, so this call wrote the file
        // and its pre-image is absent — an undo removes it.
        staged_pre_image: None,
    })
}

/// **Why a gated create did not happen** — [`create_gated`]'s error.
///
/// Two arms because the two kinds of refusal are completed in different places. Every
/// admission, minting and IO refusal is a finished [`Finding`], route included. The
/// **create-only** refusal is not: its route is the caller's, because the correction
/// differs by verb and by what the task already holds (`doc create` takes a `--slug`,
/// `doc author` does not; a task that already holds its doc has no distinct identity to
/// be routed at) — so the engine hands back the occupied address and the caller finishes
/// it through [`already_exists_finding`]. A separate arm rather than a route-less
/// `Finding`: a caller cannot map this error without saying what the occupied-home
/// refusal's route is, so a new minting door cannot acquire the gate and forget it.
#[derive(Clone, Debug, PartialEq)]
pub enum CreateRefusal {
    /// A complete blocking finding — unknown doctype, gate-blocked, empty title, a
    /// working-area IO failure.
    Blocked(Box<Finding>),
    /// The gate entry carries `new: true` and the minted identity's home is already a
    /// file on disk ([`occupied_home`]): nothing was copied in, staged or bound. `address`
    /// is that identity's `<type>:<slug>` — [`already_exists_finding`]'s first argument.
    AlreadyExists {
        /// The occupied identity, `<type>:<slug>`.
        address: String,
    },
}

impl From<Finding> for CreateRefusal {
    fn from(finding: Finding) -> Self {
        CreateRefusal::Blocked(Box::new(finding))
    }
}

/// **The agent-initiated `create`** — [`create`] gated by the workflow's resolved
/// `allows-create` set (`design/write-commands.md` → The create-gate, enforcement
/// steps 3 then 5). Ordering matches the design: an **unknown** doctype rejects
/// *before* the gate (step 3); a **disallowed** doctype (known, but not in `gate`)
/// rejects with the structured `create.gate-blocked` [`Finding`] (step 5) carrying a
/// loosen route; an **admitted** doctype proceeds to [`create`].
///
/// On an **object-form** gate entry (`{type, as: <role>}`), the created instance
/// is additionally **bound to that context role** (`write-commands.md` → The
/// create-gate, step 4: "bind it to the entry's `as:` role if the entry declares
/// one"): the minted `<type>:<slug>` is recorded in `<task_dir>/roles.json`, so a
/// re-composed `jigc start --task <id>` resolves `task.<role>` to the created doc
/// (`DECISIONS.md` 2026-05-31 → inc-5 `as:` role binding at create). A
/// **bare-form** entry (an empty `as_role`) grants create permission without
/// declaring a role and binds nothing.
///
/// **Same-identity staged copy → ack `existed` and bind** (M45 Inc 5 T2;
/// `DECISIONS.md` 2026-07-23 M45 planning → Fork 2). When the minted slug already
/// has a **staged** file in the working area — an earlier edit verb copy-on-wrote it,
/// or a prior `create` staged it — the gated path returns [`CreatedDoc`] `existed`
/// (the ack "already existed — copied in for update") and binds the role, rather than
/// routing away with `create.serial-collision` from the **only** repairing action.
/// The **ungated** [`create`] keeps the serial-mint reject untouched (the fan-out
/// case): only the agent-initiated gate acks-existed.
///
/// **Create-only → refuse an occupied home, here** (M55's `new: true`; the rc.24 review's
/// `(R6, K-1)`). The minted identity's home is probed **once** ([`occupied_home`]), ahead
/// of both branches below, and that one answer decides both outcomes: under an entry
/// carrying `new: true` an occupied home is [`CreateRefusal::AlreadyExists`] — returned
/// before the staged-copy ack, the copy-in and the role bind, so nothing is staged and no
/// role is bound — and under any other entry the same answer is what [`stage_minted`]
/// copies in. So **under a `new: true` entry no create reaches the copy-in**, by
/// construction and not by a caller's earlier check: the CLI's title pre-check asks the
/// same predicate first ([`create_occupied`]) to *rank* the refusal ahead of its other
/// arms, but a home that becomes occupied after that ask is refused here. The home is
/// probed whatever the task has staged (M55 pin P3 — a committed doc another verb already
/// copied in is still on disk), and whatever a migration recorded as its source (the
/// blank-seed over an in-location squatter is create-or-update's exception, not a way to
/// write over a doc under an entry that only creates).
#[allow(clippy::too_many_arguments)]
pub fn create_gated(
    task_dir: &Path,
    schemas: &std::collections::BTreeMap<String, Schema>,
    gate: &[crate::compose::AllowsCreate],
    type_name: &str,
    id_source: &str,
    repo_root: &Path,
    on_create: &[crate::field_block::Field],
    slug_override: Option<&str>,
) -> Result<CreatedDoc, CreateRefusal> {
    // Steps 3 + 5: unknown doctype, then the gate — asked through the shared probe, so a
    // caller that must not out-rank them asks the identical question.
    let (schema, entry) = create_admission(schemas, gate, type_name)?;
    let minted = mint_instance(task_dir, schema, type_name, id_source, slug_override)?;
    // The one probe of the home. Create-only reads it first: an occupied home is refused
    // before anything below can hand back, copy in or bind.
    let home = occupied_home(repo_root, schema, &minted.slug);
    if entry.new && home.is_some() {
        return Err(CreateRefusal::AlreadyExists {
            address: minted.address,
        });
    }
    // Step 4: admitted → mint + provision (or copy-in a committed instance) — the
    // `slug_override` (the front door's `--slug`) drives the minted id verbatim. But
    // first probe for a same-identity **staged** copy: if the minted slug's working-area
    // file already exists, `create` would reject it with `create.serial-collision`, which
    // routes the agent away from the only repairing action. Ack `existed` and fall
    // through to the role-bind instead (the shared `mint_instance` keeps the slug
    // derivation identical to `create`'s, so the probe can never diverge from the reject).
    let created = if minted.path.exists() {
        // The staged copy is handed back **as found** — this call writes nothing — so its
        // bytes are captured as the pre-image: a caller undoing a failed multi-step write
        // must restore them, never remove a file it did not create (M47 Increment 6).
        // "Present" is read, never assumed: an unreadable working copy is an IO block
        // here, because swallowing it into "absent" would make the undo a **deletion** of
        // a doc that still exists — the failure mode the pre-image exists to prevent.
        let bytes = std::fs::read(&minted.path)
            .map_err(|err| io_finding(&minted.address, "read the staged instance", &err))?;
        CreatedDoc {
            address: minted.address,
            path: minted.path,
            existed: true,
            staged_pre_image: Some(bytes),
        }
    } else {
        stage_minted(task_dir, schema, type_name, minted, home, on_create)?
    };
    // … then bind it to the entry's `as:` role if the entry declares one. The create-gate
    // keeps **last-write-wins** (`RolesRecord::bind` overwrites): an explicit `doc create
    // --as` is an author act that may deliberately re-point a role. (The incidental
    // copy-on-write binding in the CLI's `read_or_copy_in` is bind-**if-unbound** instead —
    // it must never clobber an explicit binding; `DECISIONS.md` 2026-07-23 M45 Inc 5 T2.)
    if !entry.as_role.is_empty() {
        let mut roles = RolesRecord::load(task_dir)
            .map_err(|err| io_finding(&created.address, "read the bound roles", &err))?;
        roles.bind(entry.as_role.clone(), created.address.clone());
        roles
            .save(task_dir)
            .map_err(|err| io_finding(&created.address, "record the bound role", &err))?;
    }
    Ok(created)
}

/// **The create's two admission checks**, as a standalone probe: an *unknown* doctype
/// rejects before the gate is consulted (step 3), then a *known-but-disallowed* one is
/// gate-blocked (step 5) — `design/write-commands.md` → The create-gate.
///
/// Extracted because [`create_gated`] is not the only caller that has to ask: a
/// **pre-check** the CLI runs before the create persists must not out-rank these two —
/// telling an agent its `--title` is wrong for a doctype this workflow cannot create at
/// all is a misdirection, and the adjudication order is admission → title. One
/// implementation, so the pre-check and the create can never disagree about which
/// question fires first (M48 Increment 2, T2).
pub fn create_admission<'a>(
    schemas: &'a std::collections::BTreeMap<String, Schema>,
    gate: &'a [crate::compose::AllowsCreate],
    type_name: &str,
) -> Result<(&'a Schema, &'a crate::compose::AllowsCreate), Finding> {
    let Some(schema) = schemas.get(type_name) else {
        return Err(unknown_doctype_finding(type_name, schemas));
    };
    let Some(entry) = gate.iter().find(|e| e.doc_type == type_name) else {
        return Err(gate_blocked_finding(type_name, gate));
    };
    Ok((schema, entry))
}

/// What a [`create`] / [`create_gated`] call would **find** at the identity it is about
/// to mint — the probe the CLI's title pre-check reads, answered by the same predicates
/// `create` itself keys on rather than by a second copy of them.
pub struct CreateIncumbent {
    /// The identity the call mints (`<type>:<slug>`), from the shared [`mint_instance`]
    /// derivation — so the pre-check can never name a different doc than the create does.
    pub address: String,
    /// The existing body the call would hand back (a **staged** working copy) or copy in
    /// (a **committed** instance). `Some` means the supplied title will *not* become the
    /// doc's `# H1`: the create writes no title over an incumbent body. `None` — the call
    /// mints fresh and the title lands.
    pub incumbent: Option<PathBuf>,
}

/// Probe the identity a create is about to mint — see [`CreateIncumbent`]. Writes
/// nothing.
///
/// The committed arm reproduces [`create`]'s copy-in branch **including its
/// in-location-squatter exception**: a migration whose recorded `source-path` IS this
/// slug's canonical destination seeds **blank** over that foreign file, so the supplied
/// title *does* land there and reporting an incumbent would be a lie about the very case
/// the exception exists for (`design/auto-migration.md` → Hardening #8).
pub fn create_incumbent(
    task_dir: &Path,
    schema: &Schema,
    type_name: &str,
    id_source: &str,
    slug_override: Option<&str>,
    repo_root: &Path,
) -> Result<CreateIncumbent, Finding> {
    let MintedInstance {
        slug,
        address,
        path,
        ..
    } = mint_instance(task_dir, schema, type_name, id_source, slug_override)?;
    if path.exists() {
        return Ok(CreateIncumbent {
            address,
            incumbent: Some(path),
        });
    }
    let squatter = migration_targets_canonical_destination(task_dir, schema, &slug)
        .map_err(|err| io_finding(&address, "read the migration source path", &err))?;
    let committed = if squatter {
        None
    } else {
        occupied_home(repo_root, schema, &slug)
    };
    Ok(CreateIncumbent {
        address,
        incumbent: committed,
    })
}

/// **The create-only probe** (M55, an `allows-create` entry carrying `new: true`): the
/// `<type>:<slug>` a create is about to mint when that identity's canonical home is
/// already a **file on disk** — `None` when the home is free, or the doctype has none.
/// Writes nothing.
///
/// This is the CLI pre-check's **ranked early ask** of the refusal — what puts
/// `create.already-exists` ahead of the title arms — and it is the same predicate
/// ([`occupied_home`]) [`create_gated`] refuses on. It is *not* what keeps an occupied
/// home from being copied in: `create_gated` asks again, once, at the point it would
/// copy in, so an answer here that has gone stale cannot become a copy-in.
///
/// The home is probed **independently of any staged copy** (M55 pin P3), which is what
/// separates it from [`create_incumbent`]: that probe answers the staged copy first and
/// so hides a committed doc the task has already copied in (by `set-slot`, say) — the
/// very overwrite a create-only entry exists to refuse. A task's own fresh mint lives
/// only in its working area, never at the home, so re-running that create stays
/// idempotent. *On disk* is wider than *committed* — an untracked file at the home is
/// occupied too — which is the safe direction. The identity comes from the shared
/// [`mint_instance`], so the probed slug cannot differ from the one the create mints.
pub fn create_occupied(
    task_dir: &Path,
    schema: &Schema,
    type_name: &str,
    id_source: &str,
    slug_override: Option<&str>,
    repo_root: &Path,
) -> Result<Option<String>, Finding> {
    let MintedInstance { slug, address, .. } =
        mint_instance(task_dir, schema, type_name, id_source, slug_override)?;
    Ok(occupied_home(repo_root, schema, &slug).map(|_| address))
}

/// Does a **bound context role**'s recorded address still name a document this task can
/// act on? — the state probe the identity-divergence rank owes its own premise. Writes
/// nothing.
///
/// [`RolesRecord`] is a *record*; the rank's sentence — *"this task's `<role>` is already
/// `<addr>`"* — is a claim about *state*, and the two part company whenever a binding
/// outlives its document. [`create_gated`] binds the `as:` role **before** the CLI's
/// `doc author` applies its payload leaves, and a leaf failure rolls the staged `.md` back
/// but not the binding (the rollback is [`CreatedDoc::rollback`]'s, which owns the file and
/// not the record). Read as an incumbent, that orphan refuses the retry with a sentence
/// naming a doc that is not there and a `jigc doc rename` route that answers *"no staged
/// instance … provision it first"* — a blocking dead end whose only exits (re-author under
/// the wrong title, or `jigc task discard`) nothing names.
///
/// **"Can act on" is two homes, because a bound doc lives in either**: the task's
/// **staged** working copy (which `jigc doc rename` retitles in place), or the
/// **committed** store instance a create would copy in (whose divergence refusal routes at
/// the task-less `jigc rename`). Present in neither, the binding is stale — not an
/// incumbent — and the next mint re-points it, since [`RolesRecord::bind`] is
/// last-write-wins. The probe is deliberately **state-derived, not event-derived**: it
/// holds however the orphan arose, including the plainly-reachable one the working area
/// invites — a directory a human or an agent edits directly.
///
/// `address` is the `<type>:<slug>` form the record stores; `schema` is that type's
/// schema (the caller has already established the doctype matches).
pub fn bound_instance_present(
    task_dir: &Path,
    schema: &Schema,
    address: &str,
    repo_root: &Path,
) -> bool {
    let Some((type_name, slug)) = address.split_once(':') else {
        return false;
    };
    if instance_path(task_dir, type_name, slug).is_file() {
        return true;
    }
    crate::store::canonical_path(repo_root, schema, slug).is_some_and(|path| path.is_file())
}

/// Does a **migration** task target this slug's own canonical destination? — the
/// create-side half of the in-location-squatter discriminator (`design/auto-migration.md`
/// → Path-collision guard / Hardening #8). Reads the task's recorded `source-path`
/// ([`read_migration_source`]); when present and **canonically equal** — via the shared
/// [`crate::store::lexical_normalize`], so a `./`-prefixed or `..`-round-tripping spelling
/// still matches (review C1/F2) — to this slug's repo-relative canonical destination
/// `<location>/<slug>.md` (the same destination form the retire-side guard compares
/// against), the committed file *is* the foreign doc being replaced, so [`create`] seeds
/// the working area blank rather than copying that (non-conformant) squatter body in.
///
/// The discriminator is the **source-path match, never singleton-ness**: a non-migration
/// task has no `source-path` → `false` (the M16 clobber-fix copy-in stays intact for every
/// non-migration caller), and an off-canonical migration (e.g. a foreign `HISTORY.md` while
/// the canonical singleton lives at root `CHANGELOG.md`) → `false` (still copies in).
///
/// The canonical destination is the schema's **`placement.file`** literal when it declares
/// one (post-M38 `changelog` homes at root `CHANGELOG.md`, no `location`), else
/// `<location>/<slug>.md`. A type with **neither** (a transient sink type) has no canonical
/// destination → `false`.
fn migration_targets_canonical_destination(
    task_dir: &Path,
    schema: &Schema,
    slug: &str,
) -> std::io::Result<bool> {
    // The canonical destination is the **placement** literal when the schema declares one
    // (post-M38 `changelog` → root `CHANGELOG.md`), else `<location>/<slug>.md`; a type with
    // neither is transient and has no canonical home. This mirrors [`crate::store::canonical_path`]
    // but stays repo-relative (the recorded `source-path` and the retire-side
    // `promotions[].destination` are both repo-relative — review C1: create- and retire-side
    // guards must compare the same destination form).
    let destination = if let Some(placement) = &schema.placement {
        placement.file.clone()
    } else if let Some(location) = schema.location.as_deref() {
        format!("{}/{slug}.md", location.trim_end_matches('/'))
    } else {
        return Ok(false);
    };
    let Some(source) = read_migration_source(task_dir)? else {
        return Ok(false);
    };
    Ok(source.normalized() == crate::store::lexical_normalize(Path::new(&destination)))
}

/// The unknown-doctype block: a blocking finding naming the unrecognized type, the
/// **valid set** it should have picked from, and a route to the existing `jigc
/// describe` surface (`write-commands.md` → The create-gate, step 3). Mirrors the
/// proven-good `migrate --as <bad>` model — name the set, route to a real recovery,
/// never a nonexistent micro-verb. Route-bearing per the settled block-payload shape.
/// Keys at the [bare doctype id](doctype_scoped_location).
fn unknown_doctype_finding(
    type_name: &str,
    schemas: &std::collections::BTreeMap<String, Schema>,
) -> Finding {
    let set: Vec<&str> = schemas.keys().map(String::as_str).collect();
    Finding::graded(
        Severity::Blocking,
        "create.unknown-doctype",
        format!(
            "unknown doctype `{type_name}`; known doctypes: [{}]",
            set.join(", ")
        ),
        Some(doctype_scoped_location(type_name)),
        Some("run `jigc describe` to see the doctypes you can author".into()),
    )
}

/// The [`Location`] of a **doctype-scoped** create block — a block whose subject is a
/// *doctype*, not a doc: no instance exists and none is going to (the gate refused it /
/// the type is unknown / the title slugs to nothing), so its stable key targets the
/// **bare doctype id** (`adr`), never a synthesized `type:<slug>` URI that would address
/// nothing ([command-output-contract.md](../../../design/command-output-contract.md) →
/// the form table, the doctype-scoped-blocks row). Without it, two distinct blocked
/// creates in one gate-less task collide on one `(code, null)` key. It is the form the
/// read path already emits for a doctype-scoped block (`store.unknown-type`).
fn doctype_scoped_location(type_name: &str) -> Location {
    Location::addressed(type_name, 1, 1)
}

/// The serial-collision block for an existing instance id: a blocking finding naming
/// the colliding address, routing the agent to edit the existing instance instead
/// (`structural-grammar.md` → minting: serial mints reject, never silently reused).
fn instance_collision_finding(address: &str) -> Finding {
    Finding::graded(
        Severity::Blocking,
        "create.serial-collision",
        format!("instance `{address}` already exists in the working area"),
        Some(Location::addressed(address, 1, 1)),
        Some(format!("edit the existing `{address}` instead of re-creating it").into()),
    )
}

/// The **create-only refusal** (M55): the create-gate entry carries `new: true` and the
/// identity this create mints already exists on disk at its home ([`create_occupied`]
/// when the pre-check asks, [`CreateRefusal::AlreadyExists`] when the create does), so
/// the create is refused before anything is copied in — nothing staged, no role bound.
/// Instance-scoped like its sibling [`instance_collision_finding`]: an identity exists,
/// so the key is that doc's `<type>:<slug>` address
/// (`design/command-output-contract.md` → The stable finding key, the URI form). The
/// route is the caller's, because the correction differs by verb — `doc create` takes a
/// `--slug`, `doc author` does not.
pub fn already_exists_finding(address: &str, route: crate::finding::Route) -> Finding {
    let ty = address.split_once(':').map_or(address, |(ty, _)| ty);
    Finding::graded(
        Severity::Blocking,
        "create.already-exists",
        format!(
            "`{address}` already exists on disk at its home, and this workflow's \
             `allows-create` entry for `{ty}` carries `new: true` — it creates a new doc \
             only, so the existing one is never copied in for update"
        ),
        Some(Location::addressed(address, 1, 1)),
        Some(route),
    )
}

/// The empty-title block for a non-singleton `create` whose title slugs to nothing
/// (`--title ""`, `--title "!!!"`): left unguarded it mints a degenerate `<ty>:<ty>`.
/// Mirrors `rename`'s slug-derivation guard (`crates/cli/src/rename.rs`); routes to
/// supply a non-empty `--title` (its slug becomes the doc id). Keys at the
/// [bare doctype id](doctype_scoped_location).
fn empty_title_finding(type_name: &str) -> Finding {
    Finding::graded(
        Severity::Blocking,
        "create.empty-title",
        format!(
            "`jigc doc create {type_name}` needs a title that yields an id, but the given title is empty or slugs to nothing"
        ),
        Some(doctype_scoped_location(type_name)),
        Some("re-run with a non-empty `--title` (its slug becomes the doc id)".into()),
    )
}

/// The structured create-gate block (`write-commands.md` → The create-gate, step 5):
/// a blocking finding naming the disallowed type + the allowed set. The route is
/// honest about the real mechanism (round-2 D6f, verified on the real binary): the
/// gate is the minting **workflow's** `allows-create:` front-matter — pack
/// authoring, not project config. No config knob loosens it, and a project
/// workflow shadow does not reach this enforcement point — so the actionable route
/// is to run the create under a workflow that grants the type.
/// Keys at the [bare doctype id](doctype_scoped_location).
fn gate_blocked_finding(type_name: &str, gate: &[crate::compose::AllowsCreate]) -> Finding {
    let allowed: Vec<&str> = gate.iter().map(|e| e.doc_type.as_str()).collect();
    Finding::graded(
        Severity::Blocking,
        "create.gate-blocked",
        format!(
            "the workflow does not allow `jigc doc create {type_name}` in-task; allowed doctypes: [{}]",
            allowed.join(", ")
        ),
        Some(doctype_scoped_location(type_name)),
        Some(
            format!(
                "create `{type_name}` in a task minted from a workflow that grants it \
                 (`jigc start` lists the catalog) — the gate is the workflow's own \
                 `allows-create:` front-matter, pack authoring, not a project-config knob"
            )
            .into(),
        ),
    )
}

/// A blocking finding for a working-area I/O failure during minting.
fn io_finding(id: &str, doing: &str, err: &std::io::Error) -> Finding {
    Finding::graded(
        Severity::Blocking,
        "task.working-area-io",
        format!("could not {doing} for task `{id}`: {err}"),
        Some(Location::addressed(format!("task:{id}"), 1, 1)),
        Some(
            "resolve the underlying I/O condition (a disk or permissions problem on the \
             `.jigc/` task working area), then re-run the command"
                .into(),
        ),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::finding::Route;
    use proptest::prelude::*;

    /// **The save-degrade tally is hidden from the published crate's docs.** [`SaveDegrades`]
    /// and [`save_degrades`] are `pub` only so `jigc`'s integration suite can read them
    /// across the crate boundary; they are a test diagnostic, not `jigc-engine` API. This
    /// reddens if either item loses its `#[doc(hidden)]` (M54 completion audit, finding 5).
    #[test]
    fn the_save_degrade_diagnostic_is_doc_hidden() {
        let source = include_str!("state.rs");
        for item in [
            "pub struct SaveDegrades {",
            "pub fn save_degrades() -> SaveDegrades {",
        ] {
            let at = source
                .find(item)
                .unwrap_or_else(|| panic!("`{item}` must be in state.rs"));
            let attrs: Vec<&str> = source[..at]
                .lines()
                .rev()
                .map(str::trim)
                .take_while(|line| line.starts_with("#[") || line.starts_with("///"))
                .collect();
            assert!(
                attrs.contains(&"#[doc(hidden)]"),
                "`{item}` is a test diagnostic and must carry `#[doc(hidden)]`; its attributes \
                 and doc lines read {attrs:?}",
            );
        }
    }

    /// **Both writer-registry rows lead with the base pin, and it is [`BASE_PIN_FILE`]** —
    /// so [`WorkArea::base_pin`]'s positional read is true rather than remembered, the task
    /// and milestone halves of the residual rule ask *the same question*, and
    /// [`unwind_area`]'s first removal is the file [`carries_base_pin`] keys on.
    ///
    /// The three-way equality is the point. A reorder of either row would silently move the
    /// residual predicate onto some other member — `docs/` on the task row, `merged/` on
    /// the milestone row — and both are entries a real area may legitimately lack, so the
    /// predicate would start answering *no such task* over live work. This reddens instead.
    #[test]
    fn both_area_rows_lead_with_the_base_pin() {
        assert_eq!(
            TASK_AREA_FILES[0], BASE_PIN_FILE,
            "the task row's member 0 is what `carries_base_pin` reads",
        );
        assert_eq!(
            MILESTONE_AREA_FILES[0], BASE_PIN_FILE,
            "and the milestone row's, so `jigc rename`'s twin asks the task seams' question",
        );
        assert_eq!(
            WorkArea::Task.base_pin(),
            WorkArea::Milestone.base_pin(),
            "one predicate over two rows, or it is two predicates",
        );
    }

    /// [`is_temp_sibling`] is the inverse of [`temp_sibling`]: it recognises what the writer
    /// mints — asked of the writer's own output, never of a spelling retyped here — and
    /// nothing a hand could plausibly have put beside the target instead.
    #[test]
    fn a_minted_temp_sibling_is_recognised_and_a_neighbour_is_not() {
        let target = Path::new("/somewhere/.jigc/state/file-state.json");
        let minted = temp_sibling(target);
        let name = minted.file_name().unwrap().to_string_lossy().into_owned();
        assert!(
            is_temp_sibling(&name, "file-state.json"),
            "the writer's own temp `{name}` must be recognised",
        );
        for neighbour in [
            "file-state.json",
            "file-state.json.lock",
            "file-state.json.tmp",
            "file-state.json.12.tmp",
            "file-state.json.12.x.tmp",
            "file-state.json.12.34.tmp.bak",
            "file-state.json..34.tmp",
            "notes.12.34.tmp",
            "edges.json.12.34.tmp",
        ] {
            assert!(
                !is_temp_sibling(neighbour, "file-state.json"),
                "`{neighbour}` is not a temp sibling of `file-state.json`",
            );
        }
    }

    /// The atomic-write temp sibling takes its disambiguator from the **shared**
    /// [`crate::tempname::unique_nanos`] mint, not a counter private to this module.
    ///
    /// Two claims, both load-bearing:
    ///
    /// 1. **The shape** is `<filename>.<pid>.<nanos>.tmp` in the target's own directory —
    ///    the `pid` keeps concurrent *processes* apart, the same-directory rule keeps the
    ///    `rename` intra-filesystem, and the trailing `.tmp` is the convention. There is no
    ///    fourth component: the private sequence nonce this site used to append is exactly
    ///    what the shared mint replaces.
    /// 2. **The value comes from the shared counter** — it lies strictly between two
    ///    readings of `unique_nanos` taken around the call. A private counter reading the
    ///    raw clock cannot satisfy this on any host whose clock is coarser than a
    ///    nanosecond (macOS truncates to microseconds), because its reading ties the one
    ///    before it. Claim 1 is the platform-independent half; claim 2 is the direct one.
    ///
    /// Given both sites draw from the one mint, their cross-site distinctness follows from
    /// the mint's own proven injectivity ([`crate::tempname`] tests) — it is not re-proven
    /// here.
    #[test]
    fn the_temp_sibling_draws_its_disambiguator_from_the_shared_mint() {
        let before = crate::tempname::unique_nanos();
        let sibling = temp_sibling(Path::new("/parent/dir/file-state.json"));
        let after = crate::tempname::unique_nanos();

        assert_eq!(
            sibling.parent(),
            Some(Path::new("/parent/dir")),
            "the temp sibling stays in the target's directory so the rename is atomic",
        );
        let name = sibling
            .file_name()
            .and_then(|n| n.to_str())
            .expect("the sibling has a UTF-8 filename");
        let suffix = name
            .strip_prefix("file-state.json.")
            .unwrap_or_else(|| panic!("{name} extends the target filename"));

        let parts: Vec<&str> = suffix.split('.').collect();
        assert_eq!(
            parts.len(),
            3,
            "{name} is <filename>.<pid>.<nanos>.tmp — the shared mint replaces the \
             private sequence nonce, so there is no fourth component",
        );
        assert_eq!(
            parts[0],
            std::process::id().to_string(),
            "{name} carries this process's pid, which separates concurrent processes",
        );
        assert_eq!(parts[2], "tmp", "{name} ends in the .tmp convention");

        let nanos: u128 = parts[1]
            .parse()
            .unwrap_or_else(|_| panic!("{name} carries a numeric nanos component"));
        assert!(
            before < nanos && nanos < after,
            "the disambiguator {nanos} must come from the shared mint (between \
             {before} and {after}), not a private clock read",
        );
    }

    /// The route-floor seam-sweep, exercised through the real `task.working-area-io` producer
    /// (M43 surface census): the mint I/O fault carries a recovery route and drives cleanly
    /// through the [`Findings`](crate::finding::Findings) serialization seam — the traffic
    /// whose absence let it ship route-less (`DECISIONS.md` 2026-07-17 → the seam-sweep rule).
    #[test]
    fn working_area_io_finding_carries_a_recovery_route_through_the_seam() {
        let err = std::io::Error::new(std::io::ErrorKind::PermissionDenied, "denied");
        let finding = io_finding("brighten-ui", "open the working area", &err);
        assert_eq!(finding.severity, Severity::Blocking);
        let route = finding
            .route
            .as_ref()
            .expect("an I/O fault names its recovery");
        assert!(
            route.as_str().contains("re-run the command"),
            "route: {route}"
        );
        // A route-less finding would panic the route-floor assert here.
        serde_json::to_string(&crate::finding::Findings::from(vec![finding])).expect("serializes");
    }

    /// A throwaway directory that removes itself on drop — keeps mint tests off
    /// any real `.jigc/` tree.
    struct TempRoot(PathBuf);

    impl TempRoot {
        fn new(tag: &str) -> Self {
            let mut path = std::env::temp_dir();
            let unique = format!(
                "jigc-mint-{tag}-{}-{:?}",
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

    /// The done-criterion: minting "Add rate limiter" opens
    /// `.jigc/tasks/add-rate-limiter/` with a base-pin recording the supplied
    /// HEAD; a second mint of the same slug rejects with a route-bearing blocking
    /// finding naming the existing task and creates nothing new.
    #[test]
    fn mint_creates_task_area_and_base_pin() {
        let root = TempRoot::new("create");
        let base = BasePin::new("0123456789abcdef0123456789abcdef01234567", "0123456");

        let minted = mint_task(
            root.path(),
            "Add rate limiter",
            "commit",
            "single-task",
            base.clone(),
            None,
        )
        .expect("first mint succeeds");

        assert_eq!(minted.id, "add-rate-limiter");
        let dir = root.path().join("tasks").join("add-rate-limiter");
        assert_eq!(minted.dir, dir);
        assert!(dir.is_dir(), "working area must exist");

        let pin = std::fs::read_to_string(dir.join(BASE_PIN_FILE)).expect("base pin written");
        // Golden over the frozen base-pin byte form for a fixed SHA.
        insta::assert_snapshot!(pin, @r#"
        {
          "sha": "0123456789abcdef0123456789abcdef01234567",
          "short": "0123456"
        }
        "#);
        // The pin round-trips back to the supplied base.
        let back: BasePin = serde_json::from_str(&pin).expect("pin parses");
        assert_eq!(back, base);

        // Second mint of the same slug → serial reject, nothing new created.
        let before = std::fs::read_dir(root.path().join("tasks"))
            .expect("tasks dir")
            .count();
        let err = mint_task(
            root.path(),
            "Add rate limiter",
            "commit",
            "single-task",
            base.clone(),
            None,
        )
        .expect_err("re-mint of the same slug rejects");
        assert_eq!(err.severity, Severity::Blocking);
        assert_eq!(err.code, "task.serial-collision");
        assert!(
            err.message.contains("add-rate-limiter"),
            "block must name the existing task: {err:?}"
        );
        assert!(
            err.route.is_some(),
            "serial collision carries a resume/discard route"
        );
        let after = std::fs::read_dir(root.path().join("tasks"))
            .expect("tasks dir")
            .count();
        assert_eq!(before, after, "no second dir created on collision");
    }

    /// Plant a residual working area at `<root>/tasks/<id>/` in one of the three shapes
    /// D3's axis names — a directory that carries **no** base pin. Returns the area.
    ///
    /// The shapes differ in what else is in there, which is what the *destroying* doors
    /// key on; the mint keys on the pin alone, so all three must answer identically here.
    fn plant_residual(root: &Path, id: &str, shape: &str) -> PathBuf {
        let area = root.join("tasks").join(id);
        std::fs::create_dir_all(&area).expect("plant the residual area");
        match shape {
            "empty dir" => {}
            "a foreign file" => {
                std::fs::write(area.join("notes.txt"), "a third party's bytes\n")
                    .expect("plant the foreign file");
            }
            "a foreign docs/<ty>:<slug>.md" => {
                let docs = area.join(DOCS_DIR);
                std::fs::create_dir_all(&docs).expect("plant the residual docs/ tree");
                std::fs::write(docs.join("adr:leftover.md"), "# Leftover\n")
                    .expect("plant the staged-looking body");
            }
            other => panic!("unknown residual shape `{other}`"),
        }
        assert!(
            !carries_base_pin(&area, WorkArea::Task),
            "a planted residual must carry no base pin, or this test proves nothing",
        );
        area
    }

    /// Every entry under `dir`, sorted — the "nothing was changed" witness.
    fn entries(dir: &Path) -> Vec<String> {
        let mut names: Vec<String> = std::fs::read_dir(dir)
            .expect("read the area")
            .filter_map(|entry| entry.ok()?.file_name().into_string().ok())
            .collect();
        names.sort();
        names
    }

    /// M53 Increment 3 / T4 (`completions/artifacts/M53/settle-record.md` → D3, *What the
    /// mint answers*) — **the mint names the leftover instead of claiming a live work
    /// unit.**
    ///
    /// Driven at `9ad95fd5`: a bare `mkdir .jigc/tasks/stray-alpha` made the next
    /// same-slug mint answer *"task `stray-alpha` is already active"* — a law-1 lie about a
    /// directory that is no task at any door, routed at `jigc start --task` and
    /// `jigc task discard --force`, **both of which T3 made answer `finalize.no-task`**. So
    /// the door stated a falsehood and then sent the agent two hops to find that out.
    ///
    /// The code and the key do not move: the state is still *the id is taken on disk*, and a
    /// driver keying on `(task.serial-collision, task:<id>)` must not learn a second pair to
    /// see it. What moves is the sentence and the route — one recovery, named once.
    #[test]
    fn mint_over_a_residual_area_names_the_leftover_not_a_live_task() {
        let root = TempRoot::new("residual-mint");
        let base = BasePin::new("0123456789abcdef0123456789abcdef01234567", "0123456");

        for shape in [
            "empty dir",
            "a foreign file",
            "a foreign docs/<ty>:<slug>.md",
        ] {
            let id = "stray-alpha";
            let area = plant_residual(root.path(), id, shape);
            let before = entries(&area);

            let err = mint_task(
                root.path(),
                "Stray alpha",
                "commit",
                "single-task",
                base.clone(),
                Some(id),
            )
            .expect_err("a mint onto a residual refuses");

            assert_eq!(err.severity, Severity::Blocking, "[{shape}] {err:?}");
            assert_eq!(
                err.code, "task.serial-collision",
                "[{shape}] the code is the state"
            );
            assert_eq!(
                err.location.as_ref().and_then(|l| l.address.clone()),
                Some(format!("task:{id}")),
                "[{shape}] the stable key's target is unmoved",
            );
            assert!(
                err.message.contains("carrying no base pin"),
                "[{shape}] the refusal says what the directory is; got: {}",
                err.message,
            );
            assert!(
                err.message.contains("tasks/stray-alpha"),
                "[{shape}] and names the path, repo-relative; got: {}",
                err.message,
            );
            assert!(
                !err.message.contains("is already active"),
                "[{shape}] the lie is gone; got: {}",
                err.message,
            );
            let route = err.route.as_ref().expect("a refusal carries a route");
            assert_eq!(
                route.as_str(),
                residual_area_route(&area_repo_path(root.path(), &area)).as_str(),
                "[{shape}] the residual route comes from the one home, not a third spelling",
            );
            // The old routes, asserted gone **by their argv** — a dead end that comes back
            // wearing new prose would still be a dead end.
            for dead in [
                &format!("jigc start --task {id}"),
                &format!("jigc task discard {id} --force"),
            ] {
                assert!(
                    !route.as_str().contains(dead.as_str()),
                    "[{shape}] `{dead}` answers `finalize.no-task` over a residual; got: {route}",
                );
            }
            assert_eq!(
                entries(&area),
                before,
                "[{shape}] the refusal precedes every write — nothing was changed",
            );
            assert!(
                !carries_base_pin(&area, WorkArea::Task),
                "[{shape}] and above all no pin was written into somebody else's directory",
            );

            std::fs::remove_dir_all(&area).expect("clear the shape before the next");
        }
    }

    /// The **over-firing control** for the cell above: a colliding mint over a *live* task
    /// still answers today's bytes, unchanged.
    ///
    /// This is the assertion that makes the new arm a narrowing rather than a rewrite. The
    /// predicate decides *which* sentence, and it must decide it the same way
    /// [`carries_base_pin`]'s other homes do — `crates/cli/tests/mint_door_base_pin.rs`
    /// holds the converse (every legitimate area carries the pin when a door can see it).
    #[test]
    fn mint_over_a_live_area_answers_the_shipped_collision_bytes() {
        let root = TempRoot::new("live-collision");
        let base = BasePin::new("0123456789abcdef0123456789abcdef01234567", "0123456");

        mint_task(
            root.path(),
            "Add rate limiter",
            "commit",
            "single-task",
            base.clone(),
            None,
        )
        .expect("the first mint lands a real area");

        let err = mint_task(
            root.path(),
            "Add rate limiter",
            "commit",
            "single-task",
            base,
            None,
        )
        .expect_err("the same slug over a live task still collides");

        assert_eq!(err.code, "task.serial-collision");
        assert_eq!(
            err.message, "task `add-rate-limiter` is already active",
            "the live cell's bytes are untouched",
        );
        assert_eq!(
            err.route.as_ref().map(Route::as_str),
            Some(
                "resume with `jigc start --task add-rate-limiter` or abandon with \
                 `jigc task discard add-rate-limiter --force`"
            ),
            "and so is its route — the resume it names really does resume",
        );
    }

    /// **One residual route, three doors** — the by-id seams' producer (T3) and the two
    /// mints (T4) hand the agent the *same bytes*, because there is exactly one recovery
    /// for a leftover directory and a second spelling would make one state answer two ways
    /// (`settle-record.md` → §5's rule, applied to this family).
    #[test]
    fn every_door_that_names_a_residual_names_the_same_recovery() {
        let root = TempRoot::new("route-identity");
        let base = BasePin::new("0123456789abcdef0123456789abcdef01234567", "0123456");

        let area = plant_residual(root.path(), "stray-alpha", "empty dir");
        let mint = mint_task(
            root.path(),
            "Stray alpha",
            "commit",
            "single-task",
            base,
            Some("stray-alpha"),
        )
        .expect_err("the mint refuses");

        // The by-id producer takes **jigc_home** (the checkout `.jigc/` hangs off), which
        // is what the mint's own [`area_repo_path`] derives from its `.jigc/` root — so
        // handing it the temp root's parent puts both on the same home and makes the
        // comparison about the *bytes*, not about two spellings of one path.
        let home = root.path().parent().expect("the temp root has a parent");
        let by_id = crate::finalize::residual_task_area_finding("stray-alpha", home, &area);

        assert_eq!(
            mint.route.as_ref().map(Route::as_str),
            by_id.route.as_ref().map(Route::as_str),
            "the mint and the by-id door route to the same act",
        );
        assert_eq!(
            by_id.route.as_ref().map(Route::as_str),
            Some(residual_area_route(&area_repo_path(root.path(), &area)).as_str()),
            "and both of them come from the shared home",
        );
    }

    /// M43 T1 (`design/surface-contract.md` → The carryover gate): the staged
    /// snapshot round-trips through its write/read companions on the base-pin
    /// mold, and the on-disk byte form is frozen — pretty JSON, key-sorted
    /// (`BTreeMap`/`BTreeSet`), one trailing newline.
    #[test]
    fn staged_snapshot_round_trips_the_frozen_on_disk_form() {
        let root = TempRoot::new("staged-snapshot");
        let mut snapshot = StagedSnapshot::default();
        snapshot.entries.insert(
            "mod.txt".to_owned(),
            "5ea2ed416fbd4a4cbe227b75fe255dd7fa6bd4d6".to_owned(),
        );
        snapshot.entries.insert(
            "added.txt".to_owned(),
            "3e757656cf36eca53338e520d134963a44f793f8".to_owned(),
        );
        snapshot.deletions.insert("del.txt".to_owned());

        write_staged_snapshot(root.path(), &snapshot).expect("snapshot writes");
        let body = std::fs::read_to_string(root.path().join(STAGED_SNAPSHOT_FILE))
            .expect("snapshot file exists");
        // Golden over the frozen on-disk form: entries key-sorted regardless of
        // insertion order, the deletion set alongside, trailing newline.
        insta::assert_snapshot!(body, @r#"
        {
          "entries": {
            "added.txt": "3e757656cf36eca53338e520d134963a44f793f8",
            "mod.txt": "5ea2ed416fbd4a4cbe227b75fe255dd7fa6bd4d6"
          },
          "deletions": [
            "del.txt"
          ]
        }
        "#);
        assert!(
            body.ends_with('\n'),
            "frozen form carries a trailing newline"
        );
        assert_eq!(
            read_staged_snapshot(root.path()).expect("snapshot reads back"),
            Some(snapshot),
            "the read companion returns the written value"
        );
    }

    /// M43 T1, the declared fail-open bound: an absent `staged-snapshot.json`
    /// reads as `None` (a task minted pre-M43 finalizes as today), never an error.
    /// A clean-index door writes an **empty** snapshot — `Some(empty)`, distinct
    /// from the absent case.
    #[test]
    fn absent_staged_snapshot_reads_none_and_empty_reads_some() {
        let root = TempRoot::new("staged-snapshot-none");
        assert_eq!(
            read_staged_snapshot(root.path()).expect("absent snapshot is not an error"),
            None,
            "a missing snapshot file is the fail-open None"
        );

        write_staged_snapshot(root.path(), &StagedSnapshot::default()).expect("empty writes");
        assert_eq!(
            read_staged_snapshot(root.path()).expect("empty snapshot reads back"),
            Some(StagedSnapshot::default()),
            "a clean-index snapshot is Some(empty), not None"
        );
    }

    /// The done-criterion for T3a (`DECISIONS.md` 2026-06-01 → M2 Increment 3
    /// re-cut): minting persists the **minting workflow id** alongside the intent,
    /// in the same plain-text style, so resume composes the task's *own* workflow,
    /// not the cascade default. The field round-trips through [`read_workflow_id`];
    /// a task working area without the file yields the clear absent case (`None`).
    #[test]
    fn mint_persists_the_workflow_id_read_back_verbatim() {
        let root = TempRoot::new("workflow-id");
        let base = BasePin::new("b".repeat(40), "bbbbbbb");

        let minted = mint_task(
            root.path(),
            "Add rate limiter",
            "commit",
            "quick-fix",
            base,
            None,
        )
        .expect("mint succeeds");

        // Persisted verbatim as a plain file next to `intent` (golden over the bytes).
        let on_disk = std::fs::read_to_string(minted.dir.join(WORKFLOW_FILE))
            .expect("workflow id file written");
        insta::assert_snapshot!(on_disk, @"quick-fix");

        // Read back through the reader — the resume-path companion of the write.
        assert_eq!(
            read_workflow_id(&minted.dir).expect("read succeeds"),
            Some("quick-fix".to_string()),
            "the persisted workflow id round-trips through the reader",
        );

        // A working area with no workflow file is the clear absent case (`None`),
        // which the CLI resume site maps to a routed error.
        let bare = root.path().join("tasks").join("bare");
        std::fs::create_dir_all(&bare).expect("create bare task dir");
        assert_eq!(
            read_workflow_id(&bare).expect("read succeeds"),
            None,
            "a task with no recorded workflow id reads as absent, never an error",
        );
    }

    /// An intent that normalizes to nothing falls back to the type name.
    #[test]
    fn empty_intent_falls_back_to_type_name() {
        let root = TempRoot::new("fallback");
        let base = BasePin::new("a".repeat(40), "aaaaaaa");

        let minted = mint_task(root.path(), "!!!___---", "adr", "single-task", base, None)
            .expect("fallback mint succeeds");
        assert_eq!(
            minted.id, "adr",
            "stripped-to-empty intent uses the type name"
        );
        assert!(root.path().join("tasks").join("adr").is_dir());
    }

    /// A stripped-whitespace intent also falls back.
    #[test]
    fn whitespace_intent_falls_back_to_type_name() {
        assert_eq!(mint_id("   ", "commit"), "commit");
        assert_eq!(mint_id("", "commit"), "commit");
    }

    proptest! {
        /// For any non-colliding, non-empty-slug intent, the minted id is exactly
        /// `slugify(intent)` — minting layers the fallback over the pure rule,
        /// nothing else.
        #[test]
        fn mint_id_is_slugify_for_non_empty(intent in "[A-Za-z0-9 _-]{1,40}") {
            let slug = crate::slug::slugify(&intent);
            prop_assume!(!slug.is_empty());
            prop_assert_eq!(mint_id(&intent, "commit"), slug);
        }
    }

    const COMMIT_YAML: &[u8] = include_bytes!(pack_path!(dev, "schemas/commit.yaml"));

    fn commit_schema() -> Schema {
        crate::schema::load_schema(COMMIT_YAML).expect("commit.yaml loads")
    }

    /// The done-criterion (`DECISIONS.md` 2026-05-31 → inc-4 working-area layout).
    /// Provisioning opens `.jigc/tasks/<id>/docs/commit:<id>.md` whose bytes equal
    /// `write::render` of the empty commit instance (golden); a subsequent atomic
    /// persist of an edited buffer replaces those bytes byte-for-byte, leaving **no**
    /// temp-file residue.
    #[test]
    fn working_area_provisions_and_persists_atomically() {
        let root = TempRoot::new("working-area");
        let schema = commit_schema();
        let task_dir = root.path().join("tasks").join("add-rate-limiter");

        // Provision: the empty commit template lands at docs/commit:<id>.md.
        let path = provision_doc(
            &task_dir,
            &schema,
            "add-rate-limiter",
            "add-rate-limiter",
            &[],
        )
        .expect("provision succeeds");
        assert_eq!(
            path,
            task_dir.join("docs").join("commit:add-rate-limiter.md"),
            "provisioned at the `:`-joined address slug under docs/"
        );
        assert!(path.is_file(), "provisioned file must exist");

        let provisioned = std::fs::read_to_string(&path).expect("read provisioned");
        // The provisioned bytes ARE `write::render` of the empty commit instance.
        let expected = write::render(&schema, &empty_instance(&schema, "add-rate-limiter"));
        assert_eq!(
            provisioned, expected,
            "provisioned bytes equal write::render of the empty commit instance"
        );
        // Golden over the provisioned empty-commit byte form.
        insta::assert_snapshot!("provisioned_empty_commit", provisioned);

        // No temp residue from the atomic provision.
        let tmp = task_dir.join("docs").join("commit:add-rate-limiter.md.tmp");
        assert!(!tmp.exists(), "no leftover temp path after provisioning");

        // Persist an edited buffer: atomic temp+rename replaces the file byte-for-byte.
        let edited = "---\ntype: feat\n---\n\n# add-rate-limiter\n\n## Summary\n\nLimit at the gateway.\n\n## Body\n\n## Trailers\n";
        persist(&path, edited.as_bytes()).expect("persist succeeds");

        let after = std::fs::read_to_string(&path).expect("read persisted");
        assert_eq!(after, edited, "persisted bytes equal the input buffer");
        assert!(
            !tmp.exists(),
            "no leftover temp path after the atomic persist"
        );
    }

    /// A doc-level seed field declared in a **non-header** simple body section lands
    /// in **that** section, not the header — the per-section routing fix. The CLI
    /// collects `set: on-create` / `default:` fields from *every* simple section, so a
    /// seed declared in a body section must be placed there; the prior wholesale
    /// header-only seed silently misplaced it (latent: inert for the shipped
    /// header-only doctypes, but a real correctness gap). The header section keeps its
    /// own header-declared seed; the body section gets its body-declared seed.
    #[test]
    fn seeded_instance_routes_each_field_to_its_declaring_section() {
        // A fixture with a header section (one seed) AND a non-header body simple
        // section that itself declares a `default:` field (the latent target).
        let yaml = br#"
type: brief
location: briefs/
id-from: title
description: A fixture with a non-header simple section carrying a default field.
usage: pin per-section seed routing.
sections:
  - id: meta
    header: true
    fields:
      - { id: status, type: enum, of: [draft, final], default: draft }
  - id: summary
    slot: { hint: "what" }
    fields:
      - { id: priority, type: enum, of: [low, high], default: low }
"#;
        let schema = crate::schema::load_schema(yaml).expect("fixture brief loads");

        // The CLI-collected seeds, from BOTH simple sections (header `status`, body
        // `priority`) — the exact slice the CLI's `on_create_doc_fields` hands in.
        let on_create = vec![
            crate::field_block::Field {
                key: "status".to_string(),
                value: crate::field_block::Value::Scalar("draft".to_string()),
            },
            crate::field_block::Field {
                key: "priority".to_string(),
                value: crate::field_block::Value::Scalar("low".to_string()),
            },
        ];

        let instance = seeded_instance(&schema, "a-brief", &on_create);
        let meta = instance
            .sections
            .iter()
            .find(|c| c.id == "meta")
            .expect("meta section present");
        let summary = instance
            .sections
            .iter()
            .find(|c| c.id == "summary")
            .expect("summary section present");

        assert_eq!(
            meta.fields.len(),
            1,
            "the header section carries only its own header-declared seed"
        );
        assert_eq!(meta.fields[0].key, "status");
        // The latent gap: the body-declared seed must land in the BODY section, not
        // be misplaced into the header (the pre-fix behavior).
        assert_eq!(
            summary.fields.len(),
            1,
            "the non-header body section carries its own body-declared seed"
        );
        assert_eq!(summary.fields[0].key, "priority");
    }

    /// Copy-in on first touch persists a pre-existing managed doc into the working
    /// area, applying the only first-touch canonicalization (BOM strip + single
    /// trailing newline, EOL-preserving) and leaving everything else byte-for-byte —
    /// the committed source is never the thing written. A BOM-prefixed, double-trailing
    /// -newline source lands canonicalized; an already-canonical source is byte-stable.
    #[test]
    fn copy_in_canonicalizes_on_first_touch() {
        let root = TempRoot::new("copy-in");
        let task_dir = root.path().join("tasks").join("supersede");

        let source =
            "\u{feff}---\nstatus: accepted\n---\n\n# Rate-limit\n\n## Context\n\nForces.\n\n\n";
        let path = copy_in(&task_dir, "adr", "rate-limit", source).expect("copy-in succeeds");

        assert_eq!(
            path,
            task_dir.join("docs").join("adr:rate-limit.md"),
            "copied in at the `:`-joined address slug under docs/"
        );
        let landed = std::fs::read_to_string(&path).expect("read copied-in");
        // BOM stripped, trailing newlines collapsed to exactly one; interior intact.
        assert_eq!(
            landed, "---\nstatus: accepted\n---\n\n# Rate-limit\n\n## Context\n\nForces.\n",
            "first-touch canonicalization: BOM strip + single trailing newline only"
        );
        assert_eq!(
            landed,
            write::first_touch_canonicalize(source),
            "copy-in IS first_touch_canonicalize of the source"
        );
    }

    /// The done-criterion (i) (`write-commands.md` → copy-on-first-touch:
    /// "provenance is recorded once, at first touch … `created` is sticky across
    /// later edits in the same area"; `DECISIONS.md` 2026-06-04 → M8 Increment 3 T3).
    /// [`ProvenanceRecord::record`] is **write-once**: the first provenance recorded
    /// for an address sticks, so a `created` doc later copied-in (recorded
    /// `edited-from-base`) stays `created` — the join's clash discriminator keys on
    /// base-membership, not on whether this area later edited the doc. A
    /// first-and-only `edited-from-base` record reads back `edited-from-base`.
    #[test]
    fn record_is_write_once_so_created_is_sticky() {
        let mut record = ProvenanceRecord::default();

        // First write of `created` sticks; a later `edited-from-base` is a no-op.
        record.record("commit:add-rate-limiter", Provenance::Created);
        record.record("commit:add-rate-limiter", Provenance::EditedFromBase);
        assert_eq!(
            record.get("commit:add-rate-limiter"),
            Some(Provenance::Created),
            "the first-recorded `created` is sticky — a later `edited-from-base` never flips it",
        );

        // A first-and-only `edited-from-base` reads back as `edited-from-base` —
        // write-once means the *first* write wins, whatever it is.
        record.record("adr:rate-limit", Provenance::EditedFromBase);
        assert_eq!(
            record.get("adr:rate-limit"),
            Some(Provenance::EditedFromBase),
            "a first-and-only `edited-from-base` record reads back `edited-from-base`",
        );
    }

    /// The done-criterion (`storage.md` → The by-task-id join → classification by
    /// provenance; `DECISIONS.md` 2026-06-04 → M7 Increment 2 T1). Staging one doc via
    /// [`provision_doc`] (a minted-here **`created`**) and one via [`copy_in`] (a
    /// base-existing **`edited-from-base`**) into a single task `docs/` area records
    /// **distinct** provenance for the two slugs in the provenance manifest the join
    /// reads — golden over the frozen on-disk bytes. The `.md` body bytes the writer
    /// owns stay byte-for-byte identical to the pre-change staging output (the bit rides
    /// beside the doc, never in it).
    #[test]
    fn staging_records_distinct_provenance_leaving_bodies_unchanged() {
        let root = TempRoot::new("provenance");
        let schema = commit_schema();
        let task_dir = root.path().join("tasks").join("add-rate-limiter");

        // `provision_doc` stages a minted-here `created` instance.
        let created_path = provision_doc(
            &task_dir,
            &schema,
            "add-rate-limiter",
            "add-rate-limiter",
            &[],
        )
        .expect("provision succeeds");
        // `copy_in` stages a base-existing `edited-from-base` instance.
        let source = "---\nstatus: accepted\n---\n\n# Rate-limit\n\n## Context\n\nForces.\n";
        let edited_path =
            copy_in(&task_dir, "adr", "rate-limit", source).expect("copy-in succeeds");

        // The `.md` body bytes are byte-for-byte the pre-change staging output —
        // the provenance bit rides beside the doc, never in it.
        let created_body = std::fs::read_to_string(&created_path).expect("read provisioned");
        assert_eq!(
            created_body,
            write::render(&schema, &empty_instance(&schema, "add-rate-limiter")),
            "provision_doc body is unchanged: write::render of the empty instance"
        );
        let edited_body = std::fs::read_to_string(&edited_path).expect("read copied-in");
        assert_eq!(
            edited_body,
            write::first_touch_canonicalize(source),
            "copy_in body is unchanged: first_touch_canonicalize of the source"
        );

        // The manifest records distinct provenance for the two slugs, read back.
        let record = ProvenanceRecord::load(&task_dir).expect("provenance manifest loads");
        assert_eq!(
            record.get("commit:add-rate-limiter"),
            Some(Provenance::Created),
            "provision_doc records `created`"
        );
        assert_eq!(
            record.get("adr:rate-limit"),
            Some(Provenance::EditedFromBase),
            "copy_in records `edited-from-base`"
        );

        // Golden over the frozen on-disk byte form of the provenance manifest.
        let bytes = std::fs::read_to_string(ProvenanceRecord::path_in(&task_dir))
            .expect("provenance manifest on disk");
        insta::assert_snapshot!("provenance_two_staged_docs", bytes);
    }

    /// A schema-set keyed type → `Schema`, the engine-domain-empty contract the
    /// CLI feeds in (mirrors `validate.rs` tests). Only `commit` is known here.
    fn schemas() -> std::collections::BTreeMap<String, Schema> {
        let mut m = std::collections::BTreeMap::new();
        m.insert("commit".to_string(), commit_schema());
        m
    }

    /// The done-criterion: the `create`/provisioning verb against the working area.
    /// Creating a **known** type (`commit`) mints `commit:<slug>` and lands the
    /// empty instance at `docs/commit:<slug>.md`; an **unknown** doctype returns the
    /// `create.unknown-doctype` blocking finding (message `"unknown doctype \`…\`"`)
    /// and creates nothing; a **serial collision** on an existing instance id rejects
    /// per the minting discipline (`write-commands.md` → Instance provisioning / The
    /// create-gate; `structural-grammar.md` → IDs: provenance and minting).
    #[test]
    fn create_provisions_and_rejects_unknown_type() {
        let root = TempRoot::new("create-verb");
        let task_dir = root.path().join("tasks").join("add-rate-limiter");
        let schemas = schemas();

        // Known type → mint `commit:<slug>` + land the empty instance on disk.
        let created = create(
            &task_dir,
            &schemas,
            "commit",
            "Add rate limiter",
            root.path(),
            &[],
            None,
        )
        .expect("create of a known type succeeds");
        assert_eq!(
            created.address, "commit:add-rate-limiter",
            "minted address is `<type>:<slug>`"
        );
        assert_eq!(
            created.path,
            task_dir.join("docs").join("commit:add-rate-limiter.md"),
            "instance placed at the `:`-joined address slug under docs/"
        );
        assert!(created.path.is_file(), "the instance file appears on disk");
        // The provisioned bytes are the empty commit template (provision_doc's contract).
        let on_disk = std::fs::read_to_string(&created.path).expect("read created");
        // The H1 is the human id-source ("Add rate limiter"), not the kebab slug —
        // the id/address/filename stay `add-rate-limiter`.
        let expected = write::render(
            &schemas["commit"],
            &empty_instance(&schemas["commit"], "Add rate limiter"),
        );
        assert_eq!(on_disk, expected, "created instance is the empty template");

        // Unknown doctype → blocking `create.unknown-doctype`, nothing created.
        let docs_before = std::fs::read_dir(task_dir.join("docs"))
            .expect("docs dir")
            .count();
        let err = create(
            &task_dir,
            &schemas,
            "spec",
            "whatever",
            root.path(),
            &[],
            None,
        )
        .expect_err("unknown doctype rejects");
        assert_eq!(err.severity, Severity::Blocking);
        assert_eq!(err.code, "create.unknown-doctype");
        assert!(
            err.message.contains("unknown doctype") && err.message.contains("spec"),
            "block names the unknown type: {err:?}"
        );
        let docs_after = std::fs::read_dir(task_dir.join("docs"))
            .expect("docs dir")
            .count();
        assert_eq!(
            docs_before, docs_after,
            "an unknown-type reject creates no instance"
        );

        // Serial collision: a second create of the same id rejects, nothing new.
        let collide = create(
            &task_dir,
            &schemas,
            "commit",
            "Add rate limiter",
            root.path(),
            &[],
            None,
        )
        .expect_err("a serial collision on an existing instance id rejects");
        assert_eq!(collide.severity, Severity::Blocking);
        assert_eq!(collide.code, "create.serial-collision");
        assert!(
            collide.message.contains("commit:add-rate-limiter"),
            "block names the existing instance: {collide:?}"
        );
        assert!(
            collide.route.is_some(),
            "a serial collision carries a route"
        );
    }

    /// (M36 inc-4 T2) A non-singleton `create` whose title slugs to nothing must be
    /// rejected up front — left unguarded, `mint_id`'s type-name fallback would mint a
    /// degenerate `<ty>:<ty>` (e.g. `adr:adr` from `--title ""`). This mirrors
    /// `rename`'s slug-derivation guard (`crates/cli/src/rename.rs`). A title that
    /// slugs empty a different way (`"!!!"`) rejects identically; a normal title and a
    /// `singleton` (fixed type-id slug, no title to derive) stay green.
    #[test]
    fn create_rejects_a_title_that_slugs_to_nothing() {
        let root = TempRoot::new("empty-title");
        let task_dir = root.path().join("tasks").join("t");
        let schemas = schemas(); // `commit` — a non-singleton.

        for bad in ["", "!!!", "   "] {
            let docs_before = std::fs::read_dir(task_dir.join("docs"))
                .map(|it| it.count())
                .unwrap_or(0);
            let err = create(&task_dir, &schemas, "commit", bad, root.path(), &[], None)
                .expect_err("a title that slugs to nothing rejects");
            assert_eq!(err.severity, Severity::Blocking);
            assert_eq!(err.code, "create.empty-title", "for title {bad:?}");
            assert!(
                err.route.is_some(),
                "the empty-title block routes to a non-empty title: {err:?}"
            );
            let docs_after = std::fs::read_dir(task_dir.join("docs"))
                .map(|it| it.count())
                .unwrap_or(0);
            assert_eq!(
                docs_before, docs_after,
                "an empty-title reject creates no instance (title {bad:?})"
            );
        }

        // A normal title still mints.
        let ok = create(
            &task_dir,
            &schemas,
            "commit",
            "Add cache",
            root.path(),
            &[],
            None,
        )
        .expect("a normal title still creates");
        assert_eq!(ok.address, "commit:add-cache");

        // A singleton with an empty id-source is untouched — its slug is the fixed
        // type id, so there is no title to derive and nothing to reject.
        let singleton_yaml = b"\
type: changelog
singleton: true
location: ./
sections: []
";
        let singleton = crate::schema::load_schema(singleton_yaml).expect("singleton loads");
        let mut singleton_schemas = std::collections::BTreeMap::new();
        singleton_schemas.insert("changelog".to_string(), singleton);
        let sing = create(
            &task_dir,
            &singleton_schemas,
            "changelog",
            "",
            root.path(),
            &[],
            None,
        )
        .expect("a singleton create with an empty id-source stays green");
        assert_eq!(sing.address, "changelog:changelog");
    }

    /// (M16 inc-2 T1) Fixed-slug minting for a `singleton` doctype. `create` on a
    /// `singleton: true` schema mints at slug **= the type id** unconditionally —
    /// a non-empty `id_source` (a title) does **not** change the slug, so a
    /// re-`create` deterministically targets the same `<location>/<ty>.md` (the
    /// premise idempotent-create rests on, review finding B-2). A **non-singleton**
    /// `create` still slugs the `id_source` (the unchanged mint discipline). See
    /// `design/methodology-docs.md` → The four doctypes.
    #[test]
    fn singleton_create_mints_at_the_type_id_regardless_of_id_source() {
        let root = TempRoot::new("singleton-slug");
        let task_dir = root.path().join("tasks").join("s");

        // A `singleton: true` schema (its location is irrelevant to the slug).
        let singleton_yaml = b"\
type: roadmap
singleton: true
location: roadmap/
sections: []
";
        let singleton = crate::schema::load_schema(singleton_yaml).expect("singleton loads");
        let mut singleton_schemas = std::collections::BTreeMap::new();
        singleton_schemas.insert("roadmap".to_string(), singleton);

        // A non-empty id_source (a title) does NOT move the slug off the type id.
        let created = create(
            &task_dir,
            &singleton_schemas,
            "roadmap",
            "Some Milestone Plan Title",
            root.path(),
            &[],
            None,
        )
        .expect("singleton create succeeds");
        assert_eq!(
            created.address, "roadmap:roadmap",
            "a singleton mints at slug = the type id, ignoring the id_source",
        );
        assert_eq!(
            created.path,
            task_dir.join("docs").join("roadmap:roadmap.md"),
            "the singleton instance lands at the fixed type-id slug",
        );

        // A non-singleton `create` still slugs the id_source (unchanged discipline).
        let non_singleton = create(
            &task_dir,
            &schemas(),
            "commit",
            "Add rate limiter",
            root.path(),
            &[],
            None,
        )
        .expect("non-singleton create succeeds");
        assert_eq!(
            non_singleton.address, "commit:add-rate-limiter",
            "a non-singleton still slugs the id_source",
        );
    }

    /// (M37 inc-1 T2) The `display-title` knob overrides a singleton's H1 display
    /// text. A throwaway singleton declaring `display-title: Vision`, once created,
    /// renders its H1 as `# Vision` (not `# <type-id>`) — the flagship-idiomatic H1.
    /// A singleton WITHOUT the field (the shipped-`changelog` shape) still renders
    /// `# <type-id>`, so the absent key leaves today's behavior. Drives the emitted
    /// artifact: the created `.md` on disk, reading its H1 line verbatim. Design:
    /// `design/design-altitude-doctypes.md` → §4 The vision surface.
    #[test]
    fn singleton_display_title_overrides_h1_absent_leaves_type_id() {
        let root = TempRoot::new("display-title-h1");
        // The single-`#` H1 line of a rendered doc body (`# X`, never `## X`).
        let h1_line = |body: &str| -> String {
            body.lines()
                .find(|l| l.starts_with("# "))
                .expect("rendered body has an H1")
                .to_string()
        };

        // A singleton declaring `display-title: Vision` → H1 reads `# Vision`,
        // regardless of the (ignored) id-source a singleton fixes to its type id.
        let with_title = b"\
type: vision
singleton: true
display-title: Vision
location: vision/
sections: []
";
        let schema = crate::schema::load_schema(with_title).expect("vision singleton loads");
        let mut vision_schemas = std::collections::BTreeMap::new();
        vision_schemas.insert("vision".to_string(), schema);
        let created = create(
            &root.path().join("tasks").join("v"),
            &vision_schemas,
            "vision",
            "some ignored id-source",
            root.path(),
            &[],
            None,
        )
        .expect("vision singleton create succeeds");
        let body = std::fs::read_to_string(&created.path).expect("read created vision");
        assert_eq!(
            h1_line(&body),
            "# Vision",
            "the display-title knob drives the singleton H1: {body:?}"
        );

        // A singleton WITHOUT `display-title` (the shipped-`changelog` shape) still
        // renders `# <type-id>` — the absent key leaves today's behavior untouched.
        let without_title = b"\
type: changelog
singleton: true
location: changelog/
sections: []
";
        let schema = crate::schema::load_schema(without_title).expect("changelog singleton loads");
        let mut changelog_schemas = std::collections::BTreeMap::new();
        changelog_schemas.insert("changelog".to_string(), schema);
        let created = create(
            &root.path().join("tasks").join("c"),
            &changelog_schemas,
            "changelog",
            "some ignored id-source",
            root.path(),
            &[],
            None,
        )
        .expect("changelog singleton create succeeds");
        let body = std::fs::read_to_string(&created.path).expect("read created changelog");
        assert_eq!(
            h1_line(&body),
            "# changelog",
            "a singleton with no display-title keeps the type-id H1: {body:?}"
        );
    }

    /// A fixture `singleton: true` schema with one prose slot section — the running-doc
    /// substrate shape (a fixed slug + real authorable content) the idempotent-create
    /// tests drive over.
    fn singleton_schema() -> Schema {
        let yaml = b"\
type: roadmap
singleton: true
location: roadmap/
sections:
  - id: overview
    slot: {}
";
        crate::schema::load_schema(yaml).expect("singleton schema loads")
    }

    fn singleton_schemas() -> std::collections::BTreeMap<String, Schema> {
        let mut m = std::collections::BTreeMap::new();
        m.insert("roadmap".to_string(), singleton_schema());
        m
    }

    /// A fixture **placement** singleton schema — `changelog`'s post-M38 shape: no
    /// `location`, its single instance homed at the literal root `CHANGELOG.md`
    /// (`design/storage.md` → Placement). The in-location-squatter discriminator must
    /// derive the canonical destination from `placement.file`, not the (absent)
    /// `location`.
    fn placement_singleton_schema() -> Schema {
        let yaml = b"\
type: changelog
singleton: true
placement: { file: CHANGELOG.md }
sections:
  - id: overview
    slot: {}
";
        crate::schema::load_schema(yaml).expect("placement singleton schema loads")
    }

    /// (M16 inc-2 T2 — cold) `create` of a `singleton` with **no committed instance**
    /// mints the empty template and round-trips byte-stable (`render(parse(.)) == .`).
    /// With no `<repo_root>/roadmap/roadmap.md` on disk, the copy-in branch is inert and
    /// the create falls through to the unchanged mint path (`provision_doc`), recording
    /// `created` provenance. See `design/methodology-docs.md` → The engine work (item 2),
    /// cold/warm spike.
    #[test]
    fn singleton_create_cold_mints_empty_and_round_trips_byte_stable() {
        let root = TempRoot::new("singleton-cold");
        let task_dir = root.path().join("tasks").join("plan");
        let schema = singleton_schema();
        let schemas = singleton_schemas();

        // No committed roadmap/roadmap.md under the repo root → cold create.
        let created = create(
            &task_dir,
            &schemas,
            "roadmap",
            "M16",
            root.path(),
            &[],
            None,
        )
        .expect("cold singleton create succeeds");
        assert_eq!(created.address, "roadmap:roadmap");
        assert_eq!(
            created.path,
            task_dir.join("docs").join("roadmap:roadmap.md"),
        );

        // The minted bytes ARE the empty template (the unchanged mint path).
        let minted = std::fs::read_to_string(&created.path).expect("read minted");
        assert_eq!(
            minted,
            write::render(&schema, &empty_instance(&schema, "roadmap")),
            "a cold singleton create mints the empty template (no copy-in)",
        );

        // Round-trips byte-stable: render(parse(minted)) == minted.
        let reparsed = write::render(
            &schema,
            &write::instance_from_source(&schema, &minted).expect("minted template parses"),
        );
        assert_eq!(reparsed, minted, "the cold mint round-trips byte-stable");

        // Cold provenance is `created`, not `edited-from-base`.
        let provenance = ProvenanceRecord::load(&task_dir).expect("provenance loads");
        assert_eq!(
            provenance.get("roadmap:roadmap"),
            Some(Provenance::Created),
            "a cold singleton create records `created` provenance",
        );
    }

    /// (M16 inc-2 T2 — warm) `create` of a `singleton` whose committed
    /// `<location>/<ty>.md` **exists** copies the committed body in (the B-5 clobber
    /// fix): the staged body is `first_touch_canonicalize` of the committed source —
    /// **no clobber, prior content preserved** — and provenance is recorded
    /// `edited-from-base`. An already-canonical committed doc copies in byte-for-byte
    /// (the `first_touch_canonicalize`-is-a-no-op confirmation the warm spike needs).
    /// See `design/methodology-docs.md` → The engine work (item 2).
    #[test]
    fn singleton_create_warm_copies_committed_body_in_no_clobber() {
        let root = TempRoot::new("singleton-warm");
        let task_dir = root.path().join("tasks").join("plan");
        let schema = singleton_schema();
        let schemas = singleton_schemas();

        // A committed roadmap with prior authored content (already canonical: one
        // trailing newline) at the singleton's fixed canonical path under the repo.
        let committed = "---\n---\n\n# roadmap\n\n## Overview\n\nMilestone M15 shipped the checkpoint step kind.\n";
        let committed_path = crate::store::canonical_path(root.path(), &schema, "roadmap")
            .expect("singleton has a committed path");
        std::fs::create_dir_all(committed_path.parent().unwrap()).expect("mk roadmap/");
        std::fs::write(&committed_path, committed).expect("commit the prior roadmap");

        // Warm create: copies the committed body in, does NOT mint blank.
        let created = create(
            &task_dir,
            &schemas,
            "roadmap",
            "M16",
            root.path(),
            &[],
            None,
        )
        .expect("warm singleton create succeeds");
        assert_eq!(created.address, "roadmap:roadmap");

        // The staged body preserves the prior content — it is the committed body,
        // first-touch-canonicalized (a no-op here: already-canonical in == out).
        let staged = std::fs::read_to_string(&created.path).expect("read staged");
        assert_eq!(
            staged,
            write::first_touch_canonicalize(committed),
            "the warm create copies the committed body in (first_touch_canonicalize)",
        );
        assert_eq!(
            staged, committed,
            "an already-canonical committed doc copies in byte-for-byte (no clobber, prior content preserved)",
        );

        // The committed source file is untouched (copy-in writes only the working copy).
        assert_eq!(
            std::fs::read_to_string(&committed_path).expect("re-read committed"),
            committed,
            "copy-in never touches the committed source",
        );

        // Warm provenance is `edited-from-base` — the base doc was copied in.
        let provenance = ProvenanceRecord::load(&task_dir).expect("provenance loads");
        assert_eq!(
            provenance.get("roadmap:roadmap"),
            Some(Provenance::EditedFromBase),
            "a warm singleton create records `edited-from-base` provenance",
        );
    }

    /// (M43 inc-7 T1 — create-or-update for committed non-singletons; `DECISIONS.md`
    /// → 2026-07-16 M43 planning: the Settle, review-baked) The committed-copy-in
    /// branch is **doctype-blind**: a **non-singleton** `create` on a slug whose
    /// committed `<location>/<slug>.md` exists copies that committed body in
    /// (`existed: true`, `edited-from-base` provenance) instead of seeding blank —
    /// the M16 create-or-update intent extended past `singleton`, dissolving the
    /// blank-Created clobber ambush at finalize. A fresh mint reports
    /// `existed: false`; a same-slug re-create still rejects with the unchanged
    /// `create.serial-collision` (the staged working copy survives).
    #[test]
    fn non_singleton_create_copies_a_committed_slug_in_for_update() {
        let root = TempRoot::new("non-singleton-copy-in");
        let task_dir = root.path().join("tasks").join("supersede");

        // An `adr` doctype is non-singleton with a committed `decisions/` location.
        let adr_yaml = b"\
type: adr
location: decisions/
id-from: title
sections:
  - id: decision
    slot: {}
";
        let adr = crate::schema::load_schema(adr_yaml).expect("adr fixture loads");
        let mut schemas = std::collections::BTreeMap::new();
        schemas.insert("adr".to_string(), adr.clone());

        // A committed adr at the slug's canonical path — the warm condition.
        let committed = "---\n---\n\n# Rate limit\n\n## Decision\n\nLimit at the gateway.\n";
        let committed_path = crate::store::canonical_path(root.path(), &adr, "rate-limit")
            .expect("adr has a committed path");
        std::fs::create_dir_all(committed_path.parent().unwrap()).expect("mk decisions/");
        std::fs::write(&committed_path, committed).expect("commit the prior adr");

        // The warm create copies the committed body in — never seeds blank.
        let warm = create(
            &task_dir,
            &schemas,
            "adr",
            "Rate limit",
            root.path(),
            &[],
            None,
        )
        .expect("a non-singleton create over a committed slug copies in");
        assert_eq!(warm.address, "adr:rate-limit");
        assert!(
            warm.existed,
            "the copy-in reports `existed: true` — the ack discriminator's source",
        );
        let staged = std::fs::read_to_string(&warm.path).expect("read staged");
        assert_eq!(
            staged, committed,
            "the working copy carries the committed body verbatim (copy-in)",
        );
        assert_eq!(
            ProvenanceRecord::load(&task_dir)
                .expect("provenance loads")
                .get("adr:rate-limit"),
            Some(Provenance::EditedFromBase),
            "the copy-in records `edited-from-base` — an ordinary re-promote at finalize",
        );
        // The committed source is untouched (copy-in writes only the working copy).
        assert_eq!(
            std::fs::read_to_string(&committed_path).expect("read committed"),
            committed,
            "copy-in never touches the committed source",
        );

        // A fresh mint (no committed instance at the slug's canonical path) reports
        // `existed: false` and provisions the empty template.
        let fresh = create(
            &task_dir,
            &schemas,
            "adr",
            "Burst limit",
            root.path(),
            &[],
            None,
        )
        .expect("a fresh non-singleton create still mints");
        assert!(
            !fresh.existed,
            "a fresh mint reports `existed: false` — the key's other arm",
        );
        assert_eq!(
            std::fs::read_to_string(&fresh.path).expect("read minted"),
            write::render(&adr, &empty_instance(&adr, "Burst limit")),
            "a fresh create still mints the empty template",
        );

        // A second create of the same slug rejects with the UNCHANGED blocking
        // serial-collision — the staged working copy survives (the steady-state guard).
        let collide = create(
            &task_dir,
            &schemas,
            "adr",
            "Rate limit",
            root.path(),
            &[],
            None,
        )
        .expect_err("a same-slug re-create rejects, unchanged");
        assert_eq!(collide.severity, Severity::Blocking);
        assert_eq!(collide.code, "create.serial-collision");
        assert!(
            collide.message.contains("adr:rate-limit"),
            "the block names the colliding instance: {collide:?}",
        );
        assert!(
            collide.route.is_some(),
            "the serial collision carries a route"
        );
    }

    /// **The create-only probe** (M55 pin P3): *occupied* means the minted identity's
    /// canonical home is a file on disk, probed independently of any staged copy. A
    /// committed home is occupied — and stays occupied after the task copied it in, which
    /// is the case [`create_incumbent`]'s staged-first answer would hide. The task's own
    /// fresh mint lives only in its working area, so it is not occupied (a re-run stays
    /// idempotent); an untracked file at the home is (on disk, not only committed).
    #[test]
    fn create_occupied_probes_the_home_on_disk_whatever_is_staged() {
        let root = TempRoot::new("create-occupied");
        let task_dir = root.path().join("tasks").join("probe");
        let adr = crate::schema::load_schema(
            b"type: adr\nlocation: decisions/\nid-from: title\nsections:\n  - id: decision\n    slot: {}\n",
        )
        .expect("adr fixture loads");
        let mut schemas = std::collections::BTreeMap::new();
        schemas.insert("adr".to_string(), adr.clone());
        let occupied = |title: &str, slug: Option<&str>| {
            create_occupied(&task_dir, &adr, "adr", title, slug, root.path())
                .expect("the probe answers")
        };

        // A committed home → occupied, at the address the create would mint.
        let home = crate::store::canonical_path(root.path(), &adr, "rate-limit").expect("a home");
        std::fs::create_dir_all(home.parent().unwrap()).expect("mk decisions/");
        std::fs::write(&home, "---\n---\n\n# Rate limit\n\n## Decision\n\nX.\n").expect("commit");
        assert_eq!(
            occupied("Rate limit", None).as_deref(),
            Some("adr:rate-limit")
        );
        // A different title slugging onto it, and a `--slug` naming it, are the same identity.
        assert_eq!(
            occupied("Rate  Limit!", None).as_deref(),
            Some("adr:rate-limit")
        );
        assert_eq!(
            occupied("Other", Some("rate-limit")).as_deref(),
            Some("adr:rate-limit")
        );

        // Copied in by an earlier write → the staged copy does not hide the home.
        create(
            &task_dir,
            &schemas,
            "adr",
            "Rate limit",
            root.path(),
            &[],
            None,
        )
        .expect("copy the committed adr in");
        assert_eq!(
            occupied("Rate limit", None).as_deref(),
            Some("adr:rate-limit")
        );

        // The task's own fresh mint is staged only → not occupied; a `--slug` beside the
        // committed doc is free too.
        create(
            &task_dir,
            &schemas,
            "adr",
            "Burst limit",
            root.path(),
            &[],
            None,
        )
        .expect("a fresh mint");
        assert_eq!(occupied("Burst limit", None), None);
        assert_eq!(occupied("Rate limit", Some("rate-limit-2")), None);

        // An untracked file hand-placed at a home is on disk → occupied.
        let placed = crate::store::canonical_path(root.path(), &adr, "hand-placed").expect("home");
        std::fs::write(&placed, "# Hand placed\n").expect("place");
        assert_eq!(
            occupied("Hand placed", None).as_deref(),
            Some("adr:hand-placed")
        );
    }

    /// The create-only refusal's constructor: its own code in the `create.*` family,
    /// **blocking**, keyed at the doc's `<type>:<slug>` URI (instance-scoped, beside
    /// `create.serial-collision`), carrying the caller's route verbatim.
    #[test]
    fn already_exists_finding_is_a_blocking_instance_scoped_create_member() {
        let finding = already_exists_finding(
            "idea:a-parked-thought",
            Route::human("choose a distinct `--title`"),
        );
        assert_eq!(finding.code, "create.already-exists");
        assert_eq!(finding.severity, Severity::Blocking);
        assert_eq!(
            finding.key().target.as_deref(),
            Some("idea:a-parked-thought"),
            "keyed at the doc's URI: {finding:?}",
        );
        assert!(
            finding.message.contains("new: true") && finding.message.contains("on disk"),
            "the message names the entry key and the on-disk reading: {}",
            finding.message,
        );
        assert_eq!(
            finding.route,
            Some(Route::human("choose a distinct `--title`")),
            "the route is the caller's, unchanged",
        );
    }

    /// The complete finding a [`create_gated`] refusal carries — every refusal but the
    /// create-only one, whose route is the caller's (so it is no finished finding yet).
    fn blocked_finding(refusal: CreateRefusal) -> Finding {
        match refusal {
            CreateRefusal::Blocked(finding) => *finding,
            CreateRefusal::AlreadyExists { address } => {
                panic!("expected a complete finding, got the create-only refusal at `{address}`")
            }
        }
    }

    /// Every file under `dir`, by relative path, with its bytes — the whole-area
    /// before/after a refusal is compared over, so *nothing staged, no role bound, no
    /// provenance written* is one assertion and cannot miss a file nobody listed.
    fn area_snapshot(dir: &Path) -> std::collections::BTreeMap<PathBuf, Vec<u8>> {
        fn walk(root: &Path, dir: &Path, out: &mut std::collections::BTreeMap<PathBuf, Vec<u8>>) {
            let Ok(entries) = std::fs::read_dir(dir) else {
                return;
            };
            for entry in entries.filter_map(Result::ok) {
                let path = entry.path();
                if path.is_dir() {
                    walk(root, &path, out);
                } else {
                    let rel = path
                        .strip_prefix(root)
                        .expect("under the root")
                        .to_path_buf();
                    out.insert(rel, std::fs::read(&path).expect("read an area file"));
                }
            }
        }
        let mut out = std::collections::BTreeMap::new();
        walk(dir, dir, &mut out);
        out
    }

    /// **Under a `new: true` entry no create reaches the copy-in** (the rc.24 review's
    /// `(R6, K-1)`; `design/findings-channel.md` §4 — *"the create-gate refuses it before
    /// copy-in, so nothing is staged"*). The refusal is the **engine's own**: the probe
    /// [`create_gated`] performs is the one that decides the copy-in, so an occupied home is
    /// refused there whatever a caller's pre-check concluded a moment earlier — which is the
    /// race-free statement of the gap. Before the fix `create_gated` took the entry and never
    /// read its key, so every arm below copied the occupant in (or handed back its staged
    /// copy and bound the role) at `Ok`.
    ///
    /// The axis is **how a create comes to mint an occupied identity** × **the entry form**:
    /// the title, a different title onto the same slug, a `--slug` override, a fixed-identity
    /// (`placement`) singleton, a home the task already **staged** through another verb (the
    /// staged copy must not hide the home — M55 pin P3), and a migration whose recorded
    /// source *is* the home (the blank-seed exception is a create-or-update rule, not a
    /// create-only one). Each is refused with the occupied address and leaves the working
    /// area byte-identical: nothing staged, no role bound, no provenance written.
    #[test]
    fn create_gated_under_a_new_entry_refuses_an_occupied_home_and_copies_nothing_in() {
        use crate::compose::AllowsCreate;

        let root = TempRoot::new("create-only-engine");
        let adr = crate::schema::load_schema(
            b"type: adr\nlocation: decisions/\nid-from: title\nsections:\n  - id: decision\n    slot: {}\n",
        )
        .expect("adr fixture loads");
        let changelog = placement_singleton_schema();
        let mut schemas = std::collections::BTreeMap::new();
        schemas.insert("adr".to_string(), adr.clone());
        schemas.insert("changelog".to_string(), changelog.clone());

        // The occupants: one `adr` and the placement singleton, both files on disk at
        // their homes (the engine reads the filesystem — committed and untracked are one
        // state here, which is the documented reading of *on disk*).
        let adr_body = "---\n---\n\n# Rate limit\n\n## Decision\n\nOCCUPANT.\n";
        let adr_home = crate::store::canonical_path(root.path(), &adr, "rate-limit").expect("home");
        std::fs::create_dir_all(adr_home.parent().unwrap()).expect("mk decisions/");
        std::fs::write(&adr_home, adr_body).expect("occupy the adr home");
        let changelog_body = "# Changelog\n\n## Overview\n\nOCCUPANT.\n";
        let changelog_home =
            crate::store::canonical_path(root.path(), &changelog, "changelog").expect("home");
        std::fs::write(&changelog_home, changelog_body).expect("occupy the placement home");

        /// How the working area is arranged before the create runs.
        #[derive(Clone, Copy, Debug)]
        enum Area {
            Empty,
            /// The occupant already copied in by another verb (the edit verbs' seam).
            StagedByAnotherVerb,
            /// A migration task whose recorded source IS the home (the squatter case).
            MigrationOntoTheHome,
        }
        /// One way a create comes to mint an occupied identity.
        struct Arm {
            what: &'static str,
            ty: &'static str,
            id_source: &'static str,
            slug: Option<&'static str>,
            /// The occupied address the refusal must name.
            address: &'static str,
            area: Area,
        }
        let arms = [
            Arm {
                what: "the occupant's title",
                ty: "adr",
                id_source: "Rate limit",
                slug: None,
                address: "adr:rate-limit",
                area: Area::Empty,
            },
            Arm {
                what: "a different title onto the same slug",
                ty: "adr",
                id_source: "Rate  Limit!",
                slug: None,
                address: "adr:rate-limit",
                area: Area::Empty,
            },
            Arm {
                what: "a `--slug` naming the occupant",
                ty: "adr",
                id_source: "Something Else",
                slug: Some("rate-limit"),
                address: "adr:rate-limit",
                area: Area::Empty,
            },
            Arm {
                what: "a fixed-identity singleton",
                ty: "changelog",
                id_source: "M56",
                slug: None,
                address: "changelog:changelog",
                area: Area::Empty,
            },
            Arm {
                what: "a home this task already staged through another verb",
                ty: "adr",
                id_source: "Rate limit",
                slug: None,
                address: "adr:rate-limit",
                area: Area::StagedByAnotherVerb,
            },
            Arm {
                what: "a migration whose source is the home",
                ty: "changelog",
                id_source: "M56",
                slug: None,
                address: "changelog:changelog",
                area: Area::MigrationOntoTheHome,
            },
        ];
        // Both entry forms: the role-binding object form, and the bare form.
        for role in ["filed", ""] {
            for (index, arm) in arms.iter().enumerate() {
                let what = format!("{} (entry role `{role}`)", arm.what);
                let task_dir = root
                    .path()
                    .join("tasks")
                    .join(format!("refused-{index}-{}", role.len()));
                std::fs::create_dir_all(&task_dir).expect("mk the task dir");
                match arm.area {
                    Area::Empty => {}
                    Area::StagedByAnotherVerb => {
                        copy_in(&task_dir, arm.ty, "rate-limit", adr_body).expect("the edit seam");
                    }
                    Area::MigrationOntoTheHome => {
                        persist(&task_dir.join(SOURCE_PATH_FILE), b"CHANGELOG.md")
                            .expect("record the in-location source path");
                    }
                }
                let gate = [AllowsCreate {
                    doc_type: arm.ty.to_string(),
                    as_role: role.to_string(),
                    new: true,
                }];
                let before = area_snapshot(&task_dir);

                let outcome = create_gated(
                    &task_dir,
                    &schemas,
                    &gate,
                    arm.ty,
                    arm.id_source,
                    root.path(),
                    &[],
                    arm.slug,
                );

                match outcome {
                    Err(CreateRefusal::AlreadyExists { address }) => {
                        assert_eq!(
                            address, arm.address,
                            "{what}: refused at the occupied address"
                        );
                    }
                    other => panic!(
                        "{what}: a `new: true` entry refuses an occupied home; got {other:?}"
                    ),
                }
                assert_eq!(
                    area_snapshot(&task_dir),
                    before,
                    "{what}: the working area is byte-identical — nothing staged, no role \
                     bound, no provenance written",
                );
            }
        }
        assert_eq!(
            std::fs::read_to_string(&adr_home).expect("re-read"),
            adr_body,
            "the occupant's bytes are untouched",
        );
        assert_eq!(
            std::fs::read_to_string(&changelog_home).expect("re-read"),
            changelog_body,
            "the placement occupant's bytes are untouched",
        );
    }

    /// The controls that keep the create-only refusal exactly as wide as its sentence
    /// (`design/findings-channel.md` §4 → Scope): under `new: true` a **free** home mints
    /// fresh and binds, and the task's **own** fresh mint is not *existing* — its re-run
    /// hands the staged copy back; and an entry **without** the key keeps create-or-update,
    /// copying the occupant in.
    #[test]
    fn create_gated_under_a_plain_entry_still_copies_in_and_a_free_home_still_mints() {
        use crate::compose::AllowsCreate;

        let root = TempRoot::new("create-only-controls");
        let adr = crate::schema::load_schema(
            b"type: adr\nlocation: decisions/\nid-from: title\nsections:\n  - id: decision\n    slot: {}\n",
        )
        .expect("adr fixture loads");
        let mut schemas = std::collections::BTreeMap::new();
        schemas.insert("adr".to_string(), adr.clone());
        let adr_body = "---\n---\n\n# Rate limit\n\n## Decision\n\nOCCUPANT.\n";
        let adr_home = crate::store::canonical_path(root.path(), &adr, "rate-limit").expect("home");
        std::fs::create_dir_all(adr_home.parent().unwrap()).expect("mk decisions/");
        std::fs::write(&adr_home, adr_body).expect("occupy the adr home");
        let entry = |new: bool| {
            [AllowsCreate {
                doc_type: "adr".to_string(),
                as_role: "decision".to_string(),
                new,
            }]
        };

        // `new: true`, a free home → a fresh mint, bound.
        let task_dir = root.path().join("tasks").join("fresh");
        let gate = entry(true);
        let fresh = create_gated(
            &task_dir,
            &schemas,
            &gate,
            "adr",
            "Burst limit",
            root.path(),
            &[],
            None,
        )
        .expect("a free home mints under `new: true`");
        assert!(!fresh.existed, "a fresh mint, not a copy-in");
        assert_eq!(
            RolesRecord::load(&task_dir).expect("roles").get("decision"),
            Some("adr:burst-limit"),
        );
        // … and its re-run is idempotent: the task's own mint is not at its home.
        let again = create_gated(
            &task_dir,
            &schemas,
            &gate,
            "adr",
            "Burst limit",
            root.path(),
            &[],
            None,
        )
        .expect("re-running the task's own fresh create stays idempotent");
        assert!(again.existed && again.staged_pre_image.is_some());

        // No `new` key, the occupied home → create-or-update: the occupant is copied in.
        let task_dir = root.path().join("tasks").join("update");
        let copied = create_gated(
            &task_dir,
            &schemas,
            &entry(false),
            "adr",
            "Rate limit",
            root.path(),
            &[],
            None,
        )
        .expect("an entry without `new` keeps create-or-update");
        assert!(copied.existed, "the occupant is copied in for update");
        assert!(
            std::fs::read_to_string(&copied.path)
                .expect("read the staged copy")
                .contains("OCCUPANT."),
            "the staged copy carries the occupant's body",
        );
        assert_eq!(
            ProvenanceRecord::load(&task_dir)
                .expect("provenance loads")
                .get("adr:rate-limit"),
            Some(Provenance::EditedFromBase),
        );
    }

    /// (M26 shakedown fix) An `id-from: title` create renders the **human title**
    /// in the `# H1`, while the id/address/filename stay the **slug** — the
    /// stable-id invariant is untouched, only the H1 display text gains its proper
    /// casing/spacing. The parser reads the H1 back as the title and
    /// `slugify(title) == the id` (the filename stem), so the round-trip holds and
    /// the address is unambiguous. Verified on a multi-word title with punctuation.
    #[test]
    fn create_renders_human_title_in_h1_id_stays_slug() {
        let root = TempRoot::new("h1-human-title");
        let task_dir = root.path().join("tasks").join("t");
        let adr_yaml = b"\
type: adr
location: decisions/
id-from: title
sections:
  - id: decision
    slot: {}
";
        let adr = crate::schema::load_schema(adr_yaml).expect("adr fixture loads");
        let mut schemas = std::collections::BTreeMap::new();
        schemas.insert("adr".to_string(), adr.clone());

        let created = create(
            &task_dir,
            &schemas,
            "adr",
            "Use MySQL: the choice",
            root.path(),
            &[],
            None,
        )
        .expect("create succeeds");

        // id / address / filename are UNCHANGED — they stay the slug.
        assert_eq!(created.address, "adr:use-mysql-the-choice");
        assert_eq!(
            created.path,
            task_dir.join("docs").join("adr:use-mysql-the-choice.md"),
        );

        // The H1 renders the human title verbatim, NOT the slug.
        let body = std::fs::read_to_string(&created.path).expect("read minted");
        assert!(
            body.lines().any(|l| l == "# Use MySQL: the choice"),
            "H1 is the human title, got:\n{body}",
        );
        assert!(
            !body.contains("# use-mysql"),
            "H1 must not be the kebab slug:\n{body}",
        );

        // The parser reads the H1 back as the title, and `slugify(title)` is the id
        // (the filename stem) — so the address is unambiguous and stable.
        let parsed = write::instance_from_source(&adr, &body).expect("minted parses");
        assert_eq!(parsed.title, "Use MySQL: the choice");
        assert_eq!(crate::slug::slugify(&parsed.title), "use-mysql-the-choice");

        // Round-trips byte-stable on the new H1 form.
        assert_eq!(
            write::render(&adr, &parsed),
            body,
            "the human-title H1 round-trips byte-stable",
        );
    }

    /// An adr-shaped fixture with a `status` **header** section carrying the
    /// `status` (default) + `date` (set-on-create) fields the M22 lift materializes
    /// — inline (no pack `code-anchor`) so it loads bare in this engine-only test.
    fn adr_header_schema() -> Schema {
        let yaml = b"\
type: adr
location: decisions/
id-from: title
sections:
  - id: status
    header: true
    fields:
      - { id: status, type: enum, of: [proposed, accepted, superseded], default: proposed }
      - { id: date, type: date, set: on-create }
  - id: decision
    slot: {}
";
        crate::schema::load_schema(yaml).expect("adr header fixture loads")
    }

    /// (M22 inc-4 T1) The clock-free **on-create seed seam**: `create` threads an
    /// additive `on_create` field slice into the provisioned instance's header. A
    /// freshly-created adr seeded with `[status: proposed, date: 2026-01-02]` (the
    /// values the CLI computes in T2) carries a non-empty front-matter fence in
    /// schema order and round-trips byte-stable (`render(parse(x)) == x`). The engine
    /// stays clock-free — it places the bytes the caller supplies, never reads `set:`
    /// itself (`design/changelog.md` → engine work #4).
    #[test]
    fn create_seeds_on_create_header_fields_and_round_trips() {
        use crate::field_block::{Field, Value};

        let root = TempRoot::new("on-create-seed");
        let task_dir = root.path().join("tasks").join("seed");
        let adr = adr_header_schema();
        let mut schemas = std::collections::BTreeMap::new();
        schemas.insert("adr".to_string(), adr.clone());

        let seed = [
            Field {
                key: "status".to_string(),
                value: Value::Scalar("proposed".to_string()),
            },
            Field {
                key: "date".to_string(),
                value: Value::Scalar("2026-01-02".to_string()),
            },
        ];

        let created = create(
            &task_dir,
            &schemas,
            "adr",
            "Rate limit",
            root.path(),
            &seed,
            None,
        )
        .expect("seeded create succeeds");
        let staged = std::fs::read_to_string(&created.path).expect("read staged");

        assert!(
            staged.starts_with("---\nstatus: proposed\ndate: 2026-01-02\n---"),
            "the staged bytes carry the seeded, non-empty header fence in schema order: {staged:?}",
        );

        // Round-trips byte-stable: `render(parse(staged)) == staged` (the retired
        // #1-risk byte-stability invariant holds over the seeded header).
        let reparsed = write::render(
            &adr,
            &write::instance_from_source(&adr, &staged).expect("staged adr parses"),
        );
        assert_eq!(reparsed, staged, "seeded header round-trips byte-stable");
    }

    /// (M22 inc-4 T1) **Empty-slice neutrality** — the additive seam is byte-neutral.
    /// `provision_doc` with `&[]` (the form every existing caller passes) renders
    /// byte-for-byte identically to `write::render` of the unseeded `empty_instance`,
    /// so no shipped golden shifts.
    #[test]
    fn provision_doc_empty_seed_is_byte_identical_to_empty_instance() {
        let root = TempRoot::new("on-create-empty");
        let task_dir = root.path().join("tasks").join("neutral");
        let adr = adr_header_schema();

        let path = provision_doc(&task_dir, &adr, "rate-limit", "rate-limit", &[])
            .expect("provision succeeds");
        let staged = std::fs::read_to_string(&path).expect("read staged");
        assert_eq!(
            staged,
            write::render(&adr, &empty_instance(&adr, "rate-limit")),
            "an empty seed renders byte-identically to the unseeded empty instance",
        );
    }

    /// The agent-initiated `create` consults the workflow's `allows-create` gate:
    /// a type **in** the gate proceeds; a type **not** in it is rejected with the
    /// structured gate-block finding carrying the loosen route; an **unknown** type
    /// is rejected *before* the gate (`write-commands.md` → The create-gate,
    /// enforcement steps 3 then 5). Commit-only scope drives the workflow-provisioned
    /// path above; this pins the gate edge the agent-initiated path consults.
    #[test]
    fn create_gated_enforces_the_allows_create_gate() {
        use crate::compose::AllowsCreate;

        let root = TempRoot::new("create-gated");
        let task_dir = root.path().join("tasks").join("g");
        // Pretend both `commit` and `adr` are known; the gate admits only `adr`.
        let mut all = schemas();
        all.insert("adr".to_string(), commit_schema()); // shape-irrelevant for the gate edge
        let gate = [AllowsCreate {
            doc_type: "adr".to_string(),
            as_role: "decision".to_string(),
            new: false,
        }];

        // In the gate → proceeds (mints + provisions).
        let ok = create_gated(
            &task_dir,
            &all,
            &gate,
            "adr",
            "Some Decision",
            root.path(),
            &[],
            None,
        )
        .expect("a gate-admitted type proceeds");
        assert_eq!(ok.address, "adr:some-decision");

        // Not in the gate → structured gate-block with the loosen route.
        let blocked = create_gated(
            &task_dir,
            &all,
            &gate,
            "commit",
            "x",
            root.path(),
            &[],
            None,
        )
        .map_err(blocked_finding)
        .expect_err("a disallowed type is gate-blocked");
        assert_eq!(blocked.severity, Severity::Blocking);
        assert_eq!(blocked.code, "create.gate-blocked");
        assert!(
            blocked.message.contains("commit") && blocked.message.contains("adr"),
            "the block names the disallowed type and the allowed set: {blocked:?}"
        );
        // Round-2 D6f: the route is honest about the real mechanism — the gate is the
        // workflow's own `allows-create:` front-matter (pack authoring); no
        // project-config knob loosens it, so the route must not claim one does.
        let route = blocked
            .route
            .as_ref()
            .expect("the gate-block carries a route");
        assert!(
            route.contains("allows-create") && route.contains("pack authoring"),
            "the route names the real mechanism (workflow front-matter, pack authoring): {route}"
        );
        assert!(
            !route.contains("in project config"),
            "the route must not claim a project-config loosening exists: {route}"
        );

        // Unknown type → unknown-doctype reject fires *before* the gate.
        let unknown = create_gated(
            &task_dir,
            &all,
            &gate,
            "wormhole",
            "x",
            root.path(),
            &[],
            None,
        )
        .map_err(blocked_finding)
        .expect_err("an unknown type rejects before the gate");
        assert_eq!(unknown.code, "create.unknown-doctype");
    }

    /// (M47 Increment 6, the triage fix) **A create's rollback restores exactly what it
    /// found** — the [`CreatedDoc::staged_pre_image`] axis iterated over *all three*
    /// branches the create seam can take, because the caller that undoes a failed
    /// multi-step write (`doc author`'s leaf chain) reaches every one of them:
    ///
    ///   1. **fresh mint** — pre-image absent; the undo removes the file this call wrote;
    ///   2. **committed copy-in** — `existed: true`, yet the call still *provisioned* the
    ///      staged file, so the pre-image is absent too and the undo restores "not
    ///      staged" while leaving the committed source untouched (the branch that shows
    ///      `existed` is **not** the discriminator a rollback may key on);
    ///   3. **same-identity staged copy** — the call hands back the file it found and
    ///      writes nothing, so the pre-image carries its bytes and the undo puts them
    ///      back. This is the destructive cell: an unconditional `remove_file` deletes an
    ///      editing session's prior work.
    ///
    /// Branch 3 rolls back over a **clobbered** file rather than an untouched one: the
    /// restore must be a real write-back, not "it happened to still be there".
    #[test]
    fn a_creates_rollback_restores_exactly_what_it_found() {
        use crate::compose::AllowsCreate;

        let root = TempRoot::new("create-rollback");
        let task_dir = root.path().join("tasks").join("r");
        let adr_yaml = b"\
type: adr
location: decisions/
id-from: title
sections:
  - id: decision
    slot: {}
";
        let adr = crate::schema::load_schema(adr_yaml).expect("adr fixture loads");
        let mut schemas = std::collections::BTreeMap::new();
        schemas.insert("adr".to_string(), adr.clone());
        // A bare-form gate entry: create permission, no role binding (the binding is
        // orthogonal to the pre-image and keeps this pin on one axis).
        let gate = [AllowsCreate {
            doc_type: "adr".to_string(),
            as_role: String::new(),
            new: false,
        }];
        let create_it = |id_source: &str| {
            create_gated(
                &task_dir,
                &schemas,
                &gate,
                "adr",
                id_source,
                root.path(),
                &[],
                None,
            )
            .expect("the gate admits `adr`")
        };

        // 1. Fresh mint → pre-image absent; the undo removes what this call provisioned.
        let fresh = create_it("Rate limit");
        assert!(
            fresh.staged_pre_image.is_none(),
            "a fresh mint provisioned the file itself — its pre-image is absent",
        );
        assert!(fresh.path.is_file(), "the fresh mint staged a file");
        fresh.rollback();
        assert!(
            !fresh.path.exists(),
            "the undo removes the file the fresh mint provisioned",
        );

        // 2. Committed copy-in → `existed: true`, pre-image STILL absent (the call
        //    provisioned the staged file from the committed body), and the committed
        //    source survives the undo untouched.
        let committed = "---\n---\n\n# Burst limit\n\n## Decision\n\nLimit at the gateway.\n";
        let committed_path = crate::store::canonical_path(root.path(), &adr, "burst-limit")
            .expect("adr has a committed path");
        std::fs::create_dir_all(committed_path.parent().unwrap()).expect("mk decisions/");
        std::fs::write(&committed_path, committed).expect("commit the prior adr");
        let warm = create_it("Burst limit");
        assert!(warm.existed, "a committed slug is copied in for update");
        assert!(
            warm.staged_pre_image.is_none(),
            "the copy-in provisioned the staged file — `existed` is not the rollback \
             discriminator, the pre-image is",
        );
        warm.rollback();
        assert!(
            !warm.path.exists(),
            "the undo restores `not staged` for a copy-in",
        );
        assert_eq!(
            std::fs::read_to_string(&committed_path).expect("read committed"),
            committed,
            "the undo never touches the committed source",
        );

        // 3. The same-identity staged copy → the pre-image carries the found bytes, and
        //    the undo writes them back over whatever the failed caller left behind.
        let prior = "---\n---\n\n# Rate limit\n\n## Decision\n\nThe prior work.\n";
        let seeded = create_it("Rate limit");
        std::fs::write(&seeded.path, prior).expect("seed the prior work");
        let over_staged = create_it("Rate limit");
        assert_eq!(
            over_staged.path, seeded.path,
            "the same title mints the same working-area path",
        );
        assert!(
            over_staged.existed,
            "a same-identity staged copy acks `existed` (M45 Inc 5 T2)",
        );
        assert_eq!(
            over_staged.staged_pre_image.as_deref(),
            Some(prior.as_bytes()),
            "the create captured the staged bytes it found",
        );
        std::fs::write(&over_staged.path, "clobbered\n").expect("simulate a partial write");
        over_staged.rollback();
        assert_eq!(
            std::fs::read_to_string(&over_staged.path).expect("read staged"),
            prior,
            "the undo restores the prior work byte-for-byte — it never removes a file the \
             create did not provision",
        );
    }

    /// (M42 inc-9 T1) The **doctype-scoped** create blocks key at the **bare doctype
    /// id** (`command-output-contract.md` → the form table, the doctype-scoped-blocks
    /// row). Their subject is a doctype, not a doc — no instance exists and none is
    /// going to — so the key's `target` is `adr`/`spec`, never `null` and never a
    /// synthesized `type:slug` URI that would address nothing. Without it, two
    /// distinct blocked creates in one gate-less task (`allows-create: []`) collide on
    /// one `(code, null)` key and a driver cannot tell them apart. The URI-addressed
    /// `create.serial-collision` is untouched (its subject *is* an instance).
    #[test]
    fn doctype_scoped_create_blocks_key_at_the_bare_doctype_id() {
        let root = TempRoot::new("doctype-key");
        let task_dir = root.path().join("tasks").join("k");
        // `commit` (from the shared helper) plus two more known doctypes — shape is
        // irrelevant to every edge below, only the *type name* is.
        let mut all = schemas();
        all.insert("adr".to_string(), commit_schema());
        all.insert("spec".to_string(), commit_schema());

        // The gate-less workflow (`allows-create: []`, e.g. `quick-fix`): two real,
        // distinct blocked creates must carry two distinct keys.
        let adr_blocked = create_gated(
            &task_dir,
            &all,
            &[],
            "adr",
            "Cache strategy",
            root.path(),
            &[],
            None,
        )
        .map_err(blocked_finding)
        .expect_err("a gate-less workflow blocks every create");
        let spec_blocked = create_gated(
            &task_dir,
            &all,
            &[],
            "spec",
            "Auth flow",
            root.path(),
            &[],
            None,
        )
        .map_err(blocked_finding)
        .expect_err("a gate-less workflow blocks every create");
        assert_eq!(adr_blocked.code, "create.gate-blocked");
        assert_eq!(spec_blocked.code, "create.gate-blocked");
        assert_eq!(
            adr_blocked.key().target.as_deref(),
            Some("adr"),
            "the gate-block keys at the bare doctype id: {adr_blocked:?}"
        );
        assert_eq!(
            spec_blocked.key().target.as_deref(),
            Some("spec"),
            "the gate-block keys at the bare doctype id: {spec_blocked:?}"
        );
        assert_ne!(
            adr_blocked.key(),
            spec_blocked.key(),
            "two blocked creates in one gate-less task must not collide on one key"
        );

        // `create.unknown-doctype` — the unknown type *is* the subject.
        let unknown = create_gated(
            &task_dir,
            &all,
            &[],
            "wormhole",
            "x",
            root.path(),
            &[],
            None,
        )
        .map_err(blocked_finding)
        .expect_err("an unknown type rejects before the gate");
        assert_eq!(unknown.code, "create.unknown-doctype");
        assert_eq!(
            unknown.key().target.as_deref(),
            Some("wormhole"),
            "the unknown-doctype block keys at the bare doctype id: {unknown:?}"
        );

        // `create.empty-title` — reached through the gate-admitting path (it fires
        // *after* the gate), so the gate must admit the type.
        let gate = [crate::compose::AllowsCreate {
            doc_type: "commit".to_string(),
            as_role: "commit".to_string(),
            new: false,
        }];
        let empty = create_gated(
            &task_dir,
            &all,
            &gate,
            "commit",
            "!!!",
            root.path(),
            &[],
            None,
        )
        .map_err(blocked_finding)
        .expect_err("a title that slugs to nothing rejects");
        assert_eq!(empty.code, "create.empty-title");
        assert_eq!(
            empty.key().target.as_deref(),
            Some("commit"),
            "the empty-title block keys at the bare doctype id: {empty:?}"
        );

        // The instance-scoped sibling is untouched: its subject *is* a doc, so it
        // keeps the doc-URI form.
        create(
            &task_dir,
            &all,
            "commit",
            "Add rate limiter",
            root.path(),
            &[],
            None,
        )
        .expect("the first create mints");
        let collide = create(
            &task_dir,
            &all,
            "commit",
            "Add rate limiter",
            root.path(),
            &[],
            None,
        )
        .expect_err("a serial collision rejects");
        assert_eq!(collide.code, "create.serial-collision");
        assert_eq!(
            collide.key().target.as_deref(),
            Some("commit:add-rate-limiter"),
            "the serial collision keeps the doc-URI target: {collide:?}"
        );
    }

    /// The done-criterion (`DECISIONS.md` 2026-05-31 → inc-5 `as:` role binding at
    /// create; `write-commands.md` → The create-gate, step 4: "bind it to the
    /// entry's `as:` role"). A `create_gated` admitting an entry carrying `as:
    /// <role>` records the minted `<type>:<slug>` as the task's bound role in
    /// `roles.json` (read back on resume); a bare-form entry (no `as:` role)
    /// writes **nothing**.
    #[test]
    fn create_gate_binds_admitted_adr_to_task_role() {
        use crate::compose::AllowsCreate;

        let root = TempRoot::new("create-gate-binds");
        let task_dir = root.path().join("tasks").join("supersede");
        let mut all = schemas();
        all.insert("adr".to_string(), commit_schema()); // shape-irrelevant for the bind

        // The object-form gate entry declares `as: decision`.
        let gate = [AllowsCreate {
            doc_type: "adr".to_string(),
            as_role: "decision".to_string(),
            new: false,
        }];

        // No roles.json before any create.
        assert!(
            !RolesRecord::path_in(&task_dir).exists(),
            "no roles.json exists before a bound create"
        );

        let created = create_gated(
            &task_dir,
            &all,
            &gate,
            "adr",
            "Shared Redis session cache",
            root.path(),
            &[],
            None,
        )
        .expect("the gate-admitted adr is created");
        assert_eq!(created.address, "adr:shared-redis-session-cache");

        // The bind landed: roles.json maps `decision -> adr:<slug>`, read back.
        let roles = RolesRecord::load(&task_dir).expect("roles.json loads");
        assert_eq!(
            roles.get("decision"),
            Some("adr:shared-redis-session-cache"),
            "the admitted instance binds to its `as:` role"
        );

        // Golden over the frozen roles.json byte form for one bound role.
        let bytes =
            std::fs::read_to_string(RolesRecord::path_in(&task_dir)).expect("roles.json on disk");
        insta::assert_snapshot!("roles_one_bound_role", bytes);

        // A bare-form entry (no `as:` role) for a *different* type writes nothing.
        let bare_dir = root.path().join("tasks").join("bare");
        let bare_gate = [AllowsCreate {
            doc_type: "commit".to_string(),
            as_role: String::new(), // the bare form: create permission, no role
            new: false,
        }];
        let _ = create_gated(
            &bare_dir,
            &all,
            &bare_gate,
            "commit",
            "Add rate limiter",
            root.path(),
            &[],
            None,
        )
        .expect("the bare-form-gated commit is created");
        assert!(
            !RolesRecord::path_in(&bare_dir).exists(),
            "a bare-form entry (no `as:` role) binds nothing — no roles.json written"
        );
    }

    /// (M43 inc-6 T2) The slug-override task-state round-trip: the `migrate` verb
    /// persists the `--slug` value under [`SLUG_OVERRIDE_FILE`]; [`read_slug_override`]
    /// reads it back verbatim, and an absent file (a slug-less migrate / any
    /// non-migration task) is `None`, never an error.
    #[test]
    fn read_slug_override_round_trips_and_absent_is_none() {
        let root = TempRoot::new("slug-override");
        let task_dir = root.path().join("tasks").join("migrate-adr-x");
        assert_eq!(
            read_slug_override(&task_dir).expect("an absent override is not an error"),
            None,
            "no override file reads back as None (the slug-less / non-migration case)",
        );
        persist(&task_dir.join(SLUG_OVERRIDE_FILE), b"pinned-decision").expect("persist");
        assert_eq!(
            read_slug_override(&task_dir).expect("the persisted override reads back"),
            Some("pinned-decision".to_string()),
            "the recorded override reads back verbatim",
        );
    }

    /// (M24 inc-5 T2 — seed-blank) The **in-location squatter** create-side guard. A
    /// **migration** task whose recorded `source-path` canonically equals the committed
    /// singleton's canonical destination seeds the working area **blank** (the empty
    /// template), NOT the committed non-conformant squatter body — so the author
    /// sequence builds onto a clean canonical skeleton, not a Frankenstein base, and the
    /// M23 e2e squatter FAIL now passes. The discriminator is the source-path match
    /// (extended from the retire side); provenance is `created` (a fresh mint, not an
    /// edit-from-base). See `design/auto-migration.md` → Path-collision guard / Hardening #8.
    #[test]
    fn migration_squatter_create_seeds_blank_not_the_committed_body() {
        let root = TempRoot::new("squatter-seed-blank");
        let task_dir = root.path().join("tasks").join("migrate-roadmap");
        let schema = singleton_schema();
        let schemas = singleton_schemas();

        // A non-conformant squatter committed AT the canonical path under the repo.
        let squatter = "# Whatever\n\nnon-conformant prior content\n";
        let committed_path = crate::store::canonical_path(root.path(), &schema, "roadmap")
            .expect("singleton has a canonical path");
        std::fs::create_dir_all(committed_path.parent().unwrap()).expect("mk roadmap/");
        std::fs::write(&committed_path, squatter).expect("commit the squatter");

        // This is a MIGRATION task whose source-path IS the canonical destination.
        persist(&task_dir.join(SOURCE_PATH_FILE), b"roadmap/roadmap.md")
            .expect("record the in-location source path");

        let created = create(
            &task_dir,
            &schemas,
            "roadmap",
            "M24",
            root.path(),
            &[],
            None,
        )
        .expect("squatter migration create succeeds");
        assert_eq!(created.address, "roadmap:roadmap");

        // Seeded BLANK — the empty template, never the committed squatter body.
        let staged = std::fs::read_to_string(&created.path).expect("read staged");
        assert_eq!(
            staged,
            write::render(&schema, &empty_instance(&schema, "roadmap")),
            "a migration squatter seeds the empty template, never the committed squatter body",
        );
        assert_ne!(
            staged,
            write::first_touch_canonicalize(squatter),
            "the non-conformant committed body is NOT copied in",
        );

        // Provenance is `created` (a fresh mint), not `edited-from-base`.
        assert_eq!(
            ProvenanceRecord::load(&task_dir)
                .expect("provenance loads")
                .get("roadmap:roadmap"),
            Some(Provenance::Created),
            "the squatter seed-blank records `created`, not `edited-from-base`",
        );
    }

    /// (M38 inc-5 T2 — placement squatter) The in-location squatter guard must fire for a
    /// **placement** singleton too. Post-M38 `changelog` carries no `location` — its home is
    /// the literal `placement.file` (root `CHANGELOG.md`), so a migration task whose recorded
    /// `source-path` is that same literal file IS the in-location squatter: `create` must seed
    /// the working area **blank**, never copy the non-conformant foreign body in. RED before
    /// the fix — [`migration_targets_canonical_destination`] derived the destination from
    /// `schema.location` alone and early-returned `false` for a location-less placement schema,
    /// so the copy-in branch read the foreign body in as the edit base (the M24-fixed
    /// Frankenstein failure, re-opened by the changelog root-relocation). See
    /// `design/auto-migration.md` → Path-collision guard.
    #[test]
    fn placement_migration_squatter_create_seeds_blank_not_the_committed_body() {
        let root = TempRoot::new("placement-squatter-seed-blank");
        let task_dir = root
            .path()
            .join("tasks")
            .join("migrate-changelog-changelog");
        let schema = placement_singleton_schema();
        let mut schemas = std::collections::BTreeMap::new();
        schemas.insert("changelog".to_string(), schema.clone());

        // A non-conformant foreign file committed AT the placement canonical destination
        // (root `CHANGELOG.md`), NOT under a `<location>/` folder.
        let squatter = "# Whatever\n\nnon-conformant prior content\n";
        let committed_path = crate::store::canonical_path(root.path(), &schema, "changelog")
            .expect("placement has a canonical path");
        if let Some(parent) = committed_path.parent() {
            std::fs::create_dir_all(parent).expect("mk parent");
        }
        std::fs::write(&committed_path, squatter).expect("commit the squatter");

        // A MIGRATION task whose source-path IS the placement canonical destination.
        persist(&task_dir.join(SOURCE_PATH_FILE), b"CHANGELOG.md")
            .expect("record the in-location source path");

        let created = create(
            &task_dir,
            &schemas,
            "changelog",
            "M38",
            root.path(),
            &[],
            None,
        )
        .expect("placement squatter migration create succeeds");
        assert_eq!(created.address, "changelog:changelog");

        // Seeded BLANK — the empty template, never the committed foreign body.
        let staged = std::fs::read_to_string(&created.path).expect("read staged");
        assert_eq!(
            staged,
            write::render(&schema, &empty_instance(&schema, "changelog")),
            "a placement migration squatter seeds the empty template, never the foreign body",
        );
        assert_ne!(
            staged,
            write::first_touch_canonicalize(squatter),
            "the non-conformant committed foreign body is NOT copied in",
        );

        // Provenance is `created` (a fresh mint), not `edited-from-base`.
        assert_eq!(
            ProvenanceRecord::load(&task_dir)
                .expect("provenance loads")
                .get("changelog:changelog"),
            Some(Provenance::Created),
            "the placement squatter seed-blank records `created`, not `edited-from-base`",
        );
    }

    /// (M24 inc-5 T2 — discriminator is path-match, not migration-ness) The seed-blank
    /// guard fires ONLY on a `source-path == canonical-destination` match: a migration
    /// whose recorded `source-path` is the **non-canonical** root `CHANGELOG.md`, with a
    /// committed body at the canonical singleton path, STILL copies the committed body in
    /// (`edited-from-base`) — the M16 clobber-fix intact. The discriminator is the path
    /// match, never the mere presence of a migration `source-path`; else the in-location
    /// guard would resurrect the clobber for every off-canonical migration. (The
    /// no-source-path M16 regression is
    /// `singleton_create_warm_copies_committed_body_in_no_clobber`.)
    #[test]
    fn migration_off_canonical_source_still_copies_committed_body_in() {
        let root = TempRoot::new("squatter-off-canonical");
        let task_dir = root.path().join("tasks").join("migrate-roadmap");
        let schema = singleton_schema();
        let schemas = singleton_schemas();

        let committed = "---\n---\n\n# roadmap\n\n## Overview\n\nPrior authored content.\n";
        let committed_path = crate::store::canonical_path(root.path(), &schema, "roadmap")
            .expect("singleton has a canonical path");
        std::fs::create_dir_all(committed_path.parent().unwrap()).expect("mk roadmap/");
        std::fs::write(&committed_path, committed).expect("commit the prior body");

        // A migration whose source-path is the NON-canonical root file.
        persist(&task_dir.join(SOURCE_PATH_FILE), b"CHANGELOG.md")
            .expect("record the off-canonical source path");

        let created = create(
            &task_dir,
            &schemas,
            "roadmap",
            "M24",
            root.path(),
            &[],
            None,
        )
        .expect("off-canonical migration create succeeds");

        let staged = std::fs::read_to_string(&created.path).expect("read staged");
        assert_eq!(
            staged,
            write::first_touch_canonicalize(committed),
            "an off-canonical migration still copies the committed body in (M16 clobber-fix intact)",
        );
        assert_eq!(
            ProvenanceRecord::load(&task_dir)
                .expect("provenance loads")
                .get("roadmap:roadmap"),
            Some(Provenance::EditedFromBase),
            "an off-canonical migration records `edited-from-base`",
        );
    }

    /// (M24 inc-5 T2 — agreement, review C1/F2) The create-side seed-blank and the
    /// retire-side skip share one path-normalization routine
    /// ([`crate::store::lexical_normalize`]), so a redundantly-spelled `source-path` —
    /// `./`-prefixed or carrying a `..` round-trip — is still recognized as the
    /// in-location squatter and seeds blank. (The retire side proves the same spellings
    /// at `finalize_plan_skips_retire_when_source_is_the_promote_destination`.)
    #[test]
    fn migration_squatter_seeds_blank_for_redundant_source_path_spellings() {
        let schema = singleton_schema();
        let schemas = singleton_schemas();
        for spelling in ["./roadmap/roadmap.md", "roadmap/../roadmap/roadmap.md"] {
            let root = TempRoot::new("squatter-spelling");
            let task_dir = root.path().join("tasks").join("migrate-roadmap");

            let committed = "# squatter\n\nnon-conformant\n";
            let committed_path = crate::store::canonical_path(root.path(), &schema, "roadmap")
                .expect("singleton has a canonical path");
            std::fs::create_dir_all(committed_path.parent().unwrap()).expect("mk roadmap/");
            std::fs::write(&committed_path, committed).expect("commit the squatter");

            persist(&task_dir.join(SOURCE_PATH_FILE), spelling.as_bytes())
                .expect("record a redundantly-spelled in-location source path");

            let created = create(
                &task_dir,
                &schemas,
                "roadmap",
                "M24",
                root.path(),
                &[],
                None,
            )
            .expect("squatter migration create succeeds");
            let staged = std::fs::read_to_string(&created.path).expect("read staged");
            assert_eq!(
                staged,
                write::render(&schema, &empty_instance(&schema, "roadmap")),
                "the `{spelling}` spelling is recognized as the squatter → seeds blank",
            );
        }
    }
}
