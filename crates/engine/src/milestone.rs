//! The `milestone` work-unit — a higher-level fan-out container in the work-unit
//! family (`milestone > task` for M7; see `design/structural-grammar.md` →
//! Work-units and runtime identity). A milestone shares the work-unit minting
//! discipline (frozen content-slug from its title, empty → the type-name
//! fallback) but has no internal section/leaf structure; its address is
//! `milestone:<slug>` with **no fragment**.
//!
//! This module lands the **mint site** (`design/write-commands.md` → Minting a
//! milestone): `jigc milestone create "<title>"` slugs the title into a frozen
//! id, opens the milestone's gitignored area at `.jigc/milestones/<id>/`, pins
//! **one shared base** (the commit every sub-task will inherit) and an **empty
//! task list**. The single shared base is what makes the by-task-id join's
//! "present at the milestone base" a deterministic lookup against a frozen commit
//! rather than a live-filesystem race (`design/storage.md` → The by-task-id
//! join). A serial collision — an active milestone dir of that id already exists
//! — **rejects** with a routed blocking [`Finding`]; nothing new is created (the
//! deterministic `-2`/`-3` collision suffix runs only at the join, never at the
//! mint site).
//!
//! The base SHA is supplied by the caller (the CLI reads HEAD via
//! `git rev-parse` — "CLI orchestrates, git executes"); the engine performs no
//! git I/O and never shells out, so minting is a pure function of
//! (jigc-root, title, base) → on-disk effect, golden-testable. No sub-task is
//! minted here — `add_task` (the next task) is the incremental populator.

use crate::finding::{Finding, Findings, Location, Severity};
use crate::state::BasePin;
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

/// The base-pin filename inside a milestone's area — the **single shared base**
/// every sub-task inherits, in the same frozen byte form as a task's `base.json`.
///
/// A shared-workbench state file like its co-located [`TASKS_FILE`], and written under the
/// same rule: [`read_base_pin`] serves every milestone door, so both its writers
/// ([`mint_milestone`], [`reseed_cache_from_record`]) persist through
/// [`crate::state::persist`]. See [`TASKS_FILE`] → *Shared state* for the class and why the
/// rule is stated over it rather than over a writer list.
const BASE_PIN_FILE: &str = "base.json";

/// The task-list filename inside a milestone's area — the sub-task ids the
/// by-task-id join will enumerate, persisted as deterministic engine state.
///
/// `pub` for one consumer: the CLI `add-task` door captures this file's pre-append bytes so a
/// rejected record commit can restore them (M47 Inc 2 T2). The name is decided **here**, beside
/// every writer of it, so the door cannot drift onto a second spelling of it.
///
/// # Shared state: atomic, and deliberately un-merged
///
/// This file is one member of the **shared, non-task-isolated `.jigc/` workbench**: every
/// sub-task of a fan-out lives under one milestone area, and the whole worktree set shares one
/// `.jigc/` (`design/team-ready-state.md` → The `.jigc` layer is shared across worktrees). So
/// M45 Settle Decision 9's rationale for `file-state.json`
/// ([`crate::file_state::FileStateRecord::save`]) applies verbatim.
///
/// **The rule is a property of the FILE CLASS, not of a writer list** (M46 completion-audit
/// F4). Stated over the class: *every write of a gitignored, shared `.jigc/` workbench file
/// that another door parses goes through [`crate::state::persist`]* — temp + `rename`, so the
/// path only ever appears complete. A plain `std::fs::write` truncates (or creates) the target
/// and *then* fills it, so a concurrent reader can read zero or partial bytes and fail to parse
/// what is on disk (measured: the real `add_task` door raising `milestone.area-io`, *"EOF while
/// parsing a value at line 1 column 0"*).
///
/// The class is derivable rather than remembered: `.jigc/` splits into a **committed** config
/// surface (`config/`, `AGENT.md`, `version`) and a **gitignored** workbench (`index/`,
/// `state/`, `milestones/`, `worktrees/`, `tasks/` — `design/storage.md` → the `.jigc` layout),
/// of which `tasks/<id>/` is task-isolated single-writer by construction. What remains is
/// `state/file-state.json` (merge + lock + persist), `index/edges.json` (lock + persist), and
/// this area's `tasks.json` **and its co-located [`BASE_PIN_FILE`]** — read by
/// [`read_base_pin`] from every milestone door.
///
/// *"That another door parses"* is the load-bearing clause, and it is what leaves this area's
/// `merged/docs/*.md` out: [`materialize`] writes those bodies and `milestone finalize`'s
/// promote sweep reads them back **inside the same call**, which clears the whole staging area
/// first — no second door reads them, so there is no cross-door window to close (a second
/// concurrent finalize destroys that area at directory granularity, a coarser problem than a
/// byte-tear and not this one).
///
/// The first statement of this rule enumerated *four writers of `tasks.json`* and shipped the
/// class un-swept: `base.json` was left truncating in two of the very functions that
/// enumeration named, and a **fifth** writer of `tasks.json` — the CLI `add-task` door's
/// rejected-commit restore (`cli/milestone.rs` → `unwind_mint`) — was outside the count
/// entirely. Which is why the claim is now keyed to the class and pinned by a test that
/// iterates it ([`shared_area_writers_replace_rather_than_truncate`]), never to a count.
///
/// **The base-relative merge that guards `file-state.json` (M46 Increment 1) does NOT extend
/// here — a decision on evidence, not an omission.** Three facts:
///
/// 1. **Every production caller is an orchestrator door**, never a sub-agent-time verb: the mint
///    at `cli/milestone.rs` → `run_create`, the append at `run_add_task` and inside
///    [`add_from_spec`], the drop at `run_add_from_spec`'s rollback (`unwind_unrecorded_seeds`),
///    and the re-seed through `cli/milestone.rs` → `reseed_cache`, reached from the eight
///    `milestone <verb>` doors (`add-task`, `add-from-spec`, `list-tasks`, `provision`,
///    `discard`, `execute`, `join`, `finalize`). Nothing a fanned sub-agent runs in its worktree
///    reaches any of them, so the concurrent read-modify-write the merge exists to survive has
///    no producer here.
/// 2. **The one door a fan-out *can* run while sub-agents are live re-seeds byte-idempotently**:
///    [`reseed_cache_from_record`] returns early when both cache files exist, and otherwise writes
///    bytes derived from the committed record — so two of them race to write the same bytes.
/// 3. **[`TaskList`] is a registry, not a flat map.** The merge is defined per *key* over
///    `base ∪ ours ∪ theirs`; a `Vec<String>` of ids has no keys, and its recorded order is an
///    audit trail the drop's rewrite depends on. The rule does not transfer, and inventing a
///    set-union in its place would silently resurrect exactly the ids `drop_sub_tasks` exists to
///    retire.
///
/// The exclusion is from the **merge**, never from temp + `rename`.
pub const TASKS_FILE: &str = "tasks.json";

/// The `milestone-record` doctype's **header (front-matter) section id** — the
/// `meta` block, the `completion-record` sibling convention
/// (`packs/methodology/schemas/milestone-record.yaml`;
/// `design/team-ready-state.md` → The milestone-record doctype). The record's
/// machine-maintained state lives on this doctype; the ids below name its leaves,
/// so the create/materialize write arm can build the canonical instance the pack
/// schema declares.
const RECORD_HEADER_SECTION: &str = "meta";

/// The header leaf carrying the milestone's shared **base-SHA pin** (`set: on-create`).
const RECORD_BASE_FIELD: &str = "base";

/// The header leaf carrying the record's **status** (active / joined,
/// `set: on-transition` — the M39 seam); seeded `active` at create.
const RECORD_STATUS_FIELD: &str = "status";

/// The repeatable section id holding one item per sub-task — empty at create,
/// appended to by the `add-task` write arm.
const RECORD_TASKS_SECTION: &str = "tasks";

/// The `tasks` item leaf carrying a sub-task's **intent** (`set: on-transition`),
/// materialized by the CLI at `add-task` from the sub-task's recorded intent.
const RECORD_TASK_INTENT_FIELD: &str = "intent";

/// The `tasks` item leaf carrying the **workflow the sub-task was minted against**
/// (`set: on-transition`, schema-version 3 — M49 Increment 9). Written by the `add-task`
/// append arm ([`append_task_item`]) and read back by the fresh-clone re-seed
/// ([`reseed_sub_task_areas`]), which is what makes the recorded workflow durable rather
/// than workbench-local. A record committed **before** the bump carries no such bullet on
/// any item; the re-seed's fall-back is what keeps it resumable.
const RECORD_TASK_WORKFLOW_FIELD: &str = "workflow";

/// The `status` value a freshly materialized record (and each fresh sub-task item)
/// carries until `join` transitions it to `joined`.
const RECORD_STATUS_ACTIVE: &str = "active";

/// The `status` value the `join` in-place-mutate write arm transitions the record's
/// header **and** every committed sub-task item to (the `set: on-transition` target).
const RECORD_STATUS_JOINED: &str = "joined";

/// The `status` value the `discard` in-place-mutate write arm settles an **abandoned**
/// milestone's record to — the terminal that makes an abandoned milestone observable
/// (milestone-record schema-version 2, M42; `design/team-ready-state.md` → The
/// lifecycle). Flipped onto the header and every **non-joined** sub-task item; a
/// genuinely [`joined`](RECORD_STATUS_JOINED) item keeps its value (it really did land).
const RECORD_STATUS_DISCARDED: &str = "discarded";

/// The lifecycle's **terminal** statuses — the two values a settled record's header can
/// read (`design/team-ready-state.md` → The lifecycle): [`joined`](RECORD_STATUS_JOINED),
/// written by `milestone finalize` when the work landed, and
/// [`discarded`](RECORD_STATUS_DISCARDED), written by `milestone discard` when it was
/// abandoned. Both mean the same operational thing — **the milestone is over, and it has
/// no workbench** — which is the whole content of [`is_terminal_status`].
const RECORD_TERMINAL_STATUSES: [&str; 2] = [RECORD_STATUS_JOINED, RECORD_STATUS_DISCARDED];

/// A milestone's persisted task list — the sub-task ids appended by `add_task`,
/// the collection the by-task-id join enumerates (`design/storage.md` → The
/// by-task-id join). Serialized as a JSON list in a stable, golden-locked byte
/// form; created **empty** at mint (`design/write-commands.md` → Minting a
/// milestone: "opens its area with an empty task list … No sub-task minted yet").
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct TaskList {
    /// The sub-task ids belonging to this milestone, in the order written.
    /// Deterministic *enumeration* by id is a later task's concern; this field is
    /// the recorded backing state.
    pub tasks: Vec<String>,
}

impl TaskList {
    /// The sub-task ids in **canonical id-sorted order** — the order the
    /// by-task-id join enumerates (`design/storage.md` → The by-task-id join;
    /// `structural-grammar.md` → ordering lives in a separate ordered list).
    /// Insertion order (and any `read_dir` order behind it) is recorded in
    /// [`Self::tasks`] as the audit trail, but enumeration is a pure function of
    /// the id *set* — sorted at this boundary so no insertion/iteration order can
    /// leak into the join's output (Validation hardening #7).
    pub fn enumerate(&self) -> Vec<String> {
        let mut ids = self.tasks.clone();
        ids.sort();
        ids
    }

    /// Serialize to the frozen on-disk byte form: pretty JSON, one trailing
    /// newline (golden-locked, matching the `base.json` convention in
    /// [`crate::state`]).
    fn to_bytes(&self) -> String {
        let mut s = serde_json::to_string_pretty(self).expect("TaskList serializes");
        s.push('\n');
        s
    }
}

/// A freshly minted milestone: its frozen slug `id`, its area `dir`, and the
/// single shared `base` every sub-task inherits.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct MintedMilestone {
    /// The frozen content-slug id (slugged from the title, type-name fallback).
    pub id: String,
    /// The milestone's area, `<jigc_root>/milestones/<id>/`.
    pub dir: PathBuf,
    /// The single shared base every sub-task is pinned to.
    pub base: BasePin,
}

/// Synthesize the **one** milestone-finalize commit message as a pure
/// **structural projection** of the milestone `id` + its **id-ordered** sub-task
/// list (`DECISIONS.md` 2026-06-04 → the inc-4 fork resolution; `finalize.md` →
/// `fan-out` finalize). No authored prose, no commit doc, no slot — this is
/// *structure* the CLI owns, like git's auto-generated merge message, so it stays
/// on the CLI's side of the determinism boundary.
///
/// The body lines are the sub-task ids in canonical id-sorted order
/// ([`TaskList::enumerate`], never insertion / `read_dir` / feed order), so the
/// message is **byte-identical** across feed orders — the property flow-9's
/// determinism assertion requires (Validation hardening #7). The layout is
/// golden-locked: a subject naming the milestone and its sub-task count, a blank
/// line, then one `- <sub-task-id>` line per sub-task, and a trailing newline.
pub fn synthesized_message(milestone_id: &str, tasks: &TaskList) -> String {
    let ids = tasks.enumerate();
    let mut msg = format!(
        "Finalize milestone {milestone_id} ({} sub-task{})\n\n",
        ids.len(),
        if ids.len() == 1 { "" } else { "s" }
    );
    for id in &ids {
        msg.push_str("- ");
        msg.push_str(id);
        msg.push('\n');
    }
    msg
}

/// The on-disk directory a milestone's state lives in: `<jigc_root>/milestones/<id>/`.
pub fn milestone_dir(jigc_root: &Path, id: &str) -> PathBuf {
    jigc_root.join("milestones").join(id)
}

/// The on-disk location of a fanned sub-agent's ephemeral code worktree, **relative
/// to jigc_home**: `.jigc/worktrees/<sub-task-id>` (`design/storage.md` → repository
/// layout). A **pure function of the sub-task id** so the `Spawn:` line can render it
/// at compose time with no jigc_home in hand (the relative token the sub-agent `cd`s
/// into); the provisioning verb joins it onto jigc_home for the absolute worktree path.
/// One shared convention for the engine emit (`emit_fan_out_spawns`) and the CLI
/// (`render_spawn` / the provision verb) — a doc-elaboration pin (`DECISIONS.md`
/// 2026-06-21 → M31 Increment 3).
pub fn worktree_path(id: &str) -> PathBuf {
    Path::new(".jigc").join("worktrees").join(id)
}

/// Mint a milestone: slug the `title` (empty → the `milestone` type-name
/// fallback, the same [`crate::slug::slugify`] + fallback discipline as
/// `state::mint_task`), reject on a serial collision with an existing milestone
/// dir, else open `<jigc_root>/milestones/<id>/`, write the **single shared
/// base-pin** capturing `base`, and write an **empty task list**.
///
/// `jigc_root` is the project's `.jigc/` home (a temp root under test). On
/// success the milestone area, its `base.json`, and an empty `tasks.json` exist
/// on disk. On a serial collision the returned [`Finding`] is
/// [`Severity::Blocking`], names the existing milestone, and carries a route —
/// nothing new is created.
pub fn mint_milestone(
    jigc_root: &Path,
    title: &str,
    base: BasePin,
) -> Result<MintedMilestone, Finding> {
    let id = mint_id(title);
    let dir = milestone_dir(jigc_root, &id);

    // Serial collision: an active milestone dir of that id already exists →
    // reject, never silently suffixed or reused (the `-2` suffix is the join's).
    if dir.exists() {
        return Err(collision_finding(&id));
    }

    std::fs::create_dir_all(&dir)
        .map_err(|err| io_finding(&id, "open the milestone area", &err))?;

    let pin_path = dir.join(BASE_PIN_FILE);
    let pin_body = render_base_pin(&base);
    crate::state::persist(&pin_path, pin_body.as_bytes())
        .map_err(|err| io_finding(&id, "write the base pin", &err))?;

    // The empty task list — no sub-task minted yet.
    crate::state::persist(
        &dir.join(TASKS_FILE),
        TaskList::default().to_bytes().as_bytes(),
    )
    .map_err(|err| io_finding(&id, "write the task list", &err))?;

    Ok(MintedMilestone { id, dir, base })
}

/// A sub-task freshly added under a milestone: the [`crate::state::MintedTask`]
/// returned by the reused mint, plus the milestone-relative bookkeeping the join
/// will read — the milestone `id` it was appended to.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AddedTask {
    /// The milestone the sub-task was appended to.
    pub milestone_id: String,
    /// The minted sub-task (its frozen id, isolated `tasks/<sub>/` area, and the
    /// **milestone's** base it inherited).
    pub task: crate::state::MintedTask,
}

/// Mint a sub-task under an existing milestone and append it to the milestone's
/// task list (`design/write-commands.md` → `jigc milestone add-task`;
/// `design/storage.md` → The by-task-id join). The sub-task is a task work-unit:
/// its id is the frozen slug of `intent`, it lives in the **shared**
/// `<jigc_root>/tasks/<sub>/` namespace in its own isolated area, and it inherits
/// the milestone's **single shared base** (read back from the milestone area, not
/// a fresh HEAD) so the join's "present at the milestone base" stays a lookup
/// against a frozen commit. `workflow_id` is the workflow the sub-task is minted
/// from (supplied by the caller), persisted by [`crate::state::mint_task`].
///
/// **Unknown milestone** — no area at `<jigc_root>/milestones/<milestone_id>/` →
/// reject with a blocking [`Finding`]; nothing is minted. **Within-milestone
/// serial collision** — the minted sub-task id is already in *this milestone's*
/// task list → reject with a routed blocking [`Finding`], appending nothing (the
/// `-2`/`-3` suffix is the join's, never incremental add). A slug colliding with a
/// *non-milestone* task is the existing `task.serial-collision` path inside
/// [`crate::state::mint_task`], surfaced unchanged.
///
/// **No staged snapshot is written here, and that is a stated disposition rather than
/// an omission** — the `Exempt` member of [`crate::state::MINT_DOORS`] carries the
/// reason, and `crates/cli/tests/mint_doors.rs` drives it: the per-task finalize
/// refuses a sub-task first, so no door consumes a snapshot this mint could write.
pub fn add_task(
    jigc_root: &Path,
    milestone_id: &str,
    intent: &str,
    workflow_id: &str,
) -> Result<AddedTask, Finding> {
    let dir = milestone_dir(jigc_root, milestone_id);

    // Unknown milestone → reject before anything is minted.
    if !dir.is_dir() {
        return Err(unknown_milestone_finding(milestone_id));
    }

    // The single shared base every sub-task inherits — read back from the
    // milestone area, never a fresh HEAD.
    let base = read_base_pin(&dir)
        .map_err(|err| io_finding(milestone_id, "read the shared base pin", &err))?;

    // The id the sub-task will mint to — checked against *this milestone's* list
    // before the mint, so a within-milestone collision rejects without side effect.
    let sub_id = mint_sub_id(intent);
    let mut list =
        read_task_list(&dir).map_err(|err| io_finding(milestone_id, "read the task list", &err))?;
    if list.tasks.iter().any(|t| t == &sub_id) {
        return Err(sub_task_collision_finding(milestone_id, &sub_id));
    }

    // Reuse the task mint so the sub-task inherits the milestone's base and the
    // standard working-area files (base/intent/workflow), in the shared namespace.
    let task = crate::state::mint_task(jigc_root, intent, SUB_TASK_TYPE, workflow_id, base, None)?;

    // Append to the milestone's task list and persist it.
    list.tasks.push(task.id.clone());
    crate::state::persist(&dir.join(TASKS_FILE), list.to_bytes().as_bytes())
        .map_err(|err| io_finding(milestone_id, "append to the task list", &err))?;

    Ok(AddedTask {
        milestone_id: milestone_id.to_string(),
        task,
    })
}

/// Seed a milestone's task list from a committed spec: enumerate the spec's
/// repeatable **`criteria`** items via the parse-items read path
/// (`parse_sections` → the section's `items`, *not* the `jigc task bind` slice)
/// and mint one sub-task per criterion, the criterion's **text** (`ParsedItem.title`,
/// the same value the slug mints from) as that sub-task's intent
/// (`design/write-commands.md` → Minting a milestone, `jigc milestone
/// add-from-spec`). Sub-tasks are appended in **physical item order** (the recorded
/// audit trail); the milestone's enumeration stays id-sorted (Increment 1).
///
/// Reuses [`add_task`] for the per-criterion mint (no second minting discipline),
/// so every sub-task inherits the milestone's single shared base in its own
/// isolated `tasks/<sub>/` area and lands in the milestone's task list.
///
/// **Resumable** (M47 Inc 2 T3): a criterion whose sub-task the milestone **already
/// carried at entry** is *skipped* and reported in [`SeededFromSpec::already_seeded`],
/// never collided — the seeding pass is one record-only commit per sub-task on the CLI
/// side, so a rejected k-th commit leaves k−1 seeded and the recovery is a re-run of the
/// same call.
///
/// **The skip set is what the COMMITTED RECORD names** (`recorded`), never the demoted
/// `.jigc` cache (M47 Inc 2 T3 fix). The committed `.md` record is the source of truth and
/// `tasks.json` is a rebuildable cache (`design/team-ready-state.md`), so keying the skip on
/// the cache made *any* id that reached the workbench but never the record — a mint aborted
/// mid-loop, an unwind that could not finish, a process killed between the mint and its
/// record commit — silently absorbed as "already seeded": the criterion is then absent from
/// the committed, team-ready record **forever**, at exit 0 with a positive ack. Keyed on the
/// record, an un-recorded id is not skipped; it is re-minted, or it surfaces [`add_task`]'s
/// loud `milestone.sub-task-collision`. `recorded` is `None` **only** where the milestone has
/// no committed record home at all (a dev-only project, whose caller lands no record commit),
/// and there the cache is the only home and therefore is the truth.
///
/// The set is read **once, at entry**, so a duplicate *within one spec* (two criteria slugging
/// alike) still surfaces the [`add_task`] `milestone.sub-task-collision` block — a resume and
/// an authoring collision stay distinguishable.
///
/// **An abort mid-loop hands back what it minted** ([`SeedingAborted::minted`]) so the door
/// unwinds those areas and ids through the same primitive a rejected record commit uses —
/// the cache/record divergence this call could produce is never produced.
///
/// **Unknown milestone** → the [`add_task`] unknown-milestone block, before any
/// spec read or mint. **Unknown / transient / unparseable spec** → the existing
/// [`crate::store`]-shaped routed blocking finding (`store.unknown-type`,
/// `store.transient-type`, `store.not-found`, `store.unparseable`). **Zero
/// `criteria` items** → a blocking `milestone.no-criteria` finding ("nothing to
/// seed from"); nothing is minted.
pub fn add_from_spec(
    jigc_root: &Path,
    repo_root: &Path,
    schemas: &std::collections::BTreeMap<String, crate::schema::Schema>,
    milestone_id: &str,
    spec_addr: &str,
    workflow_id: &str,
    recorded: Option<&[String]>,
) -> Result<SeededFromSpec, SeedingAborted> {
    let dir = milestone_dir(jigc_root, milestone_id);

    // Unknown milestone → reject before any spec read or mint.
    if !dir.is_dir() {
        return Err(unknown_milestone_finding(milestone_id).into());
    }

    // Read the committed spec and enumerate its `criteria` items (parse-items path).
    let criteria = read_spec_criteria(repo_root, schemas, spec_addr)?;

    // Zero criteria → "nothing to seed from"; mint nothing.
    if criteria.is_empty() {
        return Err(no_criteria_finding(spec_addr).into());
    }

    // The resume set — what the **committed record** names at entry (the source of truth),
    // falling back to the demoted cache only where there is no record home at all. Read once,
    // so a criterion minted by *this* call is not in it and a within-spec duplicate still
    // collides through `add_task`.
    let seeded_at_entry: std::collections::BTreeSet<String> = match recorded {
        Some(ids) => ids.iter().cloned().collect(),
        None => read_task_list(&dir)
            .map_err(|err| io_finding(milestone_id, "read the task list", &err))?
            .tasks
            .into_iter()
            .collect(),
    };

    // One sub-task per not-yet-seeded criterion, the criterion text as intent, in
    // physical order. An abort mid-loop carries this call's mints out with the finding,
    // so the door unwinds them and the workbench never names what the record does not.
    let mut added = Vec::with_capacity(criteria.len());
    let mut already_seeded = Vec::new();
    for intent in &criteria {
        let sub_id = mint_sub_id(intent);
        if seeded_at_entry.contains(&sub_id) {
            already_seeded.push(sub_id);
            continue;
        }
        match add_task(jigc_root, milestone_id, intent, workflow_id) {
            Ok(task) => added.push(task),
            Err(finding) => {
                return Err(SeedingAborted {
                    finding,
                    minted: added,
                });
            }
        }
    }
    Ok(SeededFromSpec {
        added,
        already_seeded,
    })
}

/// One [`add_from_spec`] seeding pass's outcome — **both halves**, because the pass is
/// resumable (M47 Inc 2 T3): the caller lands one record-only commit per newly minted
/// sub-task, and a re-run after a rejected k-th commit must be able to say what it seeded
/// *and* what was already there.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SeededFromSpec {
    /// The sub-tasks **this call** minted, in physical criterion order.
    pub added: Vec<AddedTask>,
    /// The sub-task ids whose criterion the **committed record** already named at entry —
    /// skipped, never collided.
    pub already_seeded: Vec<String>,
}

/// What an aborted [`add_from_spec`] pass leaves behind — the blocking [`Finding`] **and the
/// sub-tasks this call had already minted** when it aborted (M47 Inc 2 T3 fix).
///
/// The pass mints N sub-tasks into the demoted cache before its caller lands a single record
/// commit, so an abort part-way through the loop (an [`add_task`] collision from two criteria
/// slugging alike, an I/O fault) leaves mints the record will never name. Handing them back
/// makes the door's existing mid-loop unwind — the very one a rejected k-th record commit
/// runs — cover this producer too, at one seam rather than two: the workbench names exactly
/// what the record names, whichever half failed.
///
/// `minted` is empty for every failure raised **before** the loop (unknown milestone,
/// unreadable spec, zero criteria), which is why [`Finding`] converts into this for free.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SeedingAborted {
    /// The blocking finding the pass failed with — propagated to the caller **unchanged**.
    pub finding: Finding,
    /// The sub-tasks this call minted before it aborted, in mint order — none of which the
    /// record names, all of which the door unwinds.
    pub minted: Vec<AddedTask>,
}

impl From<Finding> for SeedingAborted {
    /// A failure raised before the first mint — nothing to unwind.
    fn from(finding: Finding) -> Self {
        SeedingAborted {
            finding,
            minted: Vec::new(),
        }
    }
}

/// Drop `ids` from the milestone's persisted task list — the **list half** of a door's
/// mid-loop unwind (M47 Inc 2 T3).
///
/// `add-from-spec` mints N sub-tasks up front while its caller lands one record-only commit
/// per sub-task, so a rejected k-th commit leaves k−1 recorded and the rest minted but
/// un-recorded. The door removes those areas and drops their ids here, so the demoted cache
/// names exactly the sub-tasks the committed record does and the resume mints them afresh.
///
/// The list is re-rendered through the same writer [`add_task`] appends with, over the
/// surviving ids **in their recorded order** — so the result is byte-identical to the list as
/// of the k−1 appends that did land. Ids the list does not carry are ignored: the drop is
/// idempotent, which is what a best-effort unwind needs.
pub fn drop_sub_tasks(jigc_root: &Path, milestone_id: &str, ids: &[String]) -> std::io::Result<()> {
    let dir = milestone_dir(jigc_root, milestone_id);
    let mut list = read_task_list(&dir)?;
    list.tasks
        .retain(|id| !ids.iter().any(|dropped| dropped == id));
    crate::state::persist(&dir.join(TASKS_FILE), list.to_bytes().as_bytes())
}

/// The block-section id the spec's repeatable acceptance criteria live in
/// (`cli/pack/schemas/spec.yaml`); the seed substrate `add_from_spec` enumerates.
const CRITERIA_SECTION: &str = "criteria";

/// Read a committed spec named by `spec_addr` and return its `criteria` items'
/// **titles** in physical order (the criterion text the slug mints from).
///
/// Resolves the address's type to a [`Schema`](crate::schema::Schema), computes the
/// canonical committed path, reads + parses it via [`crate::parse::parse_sections`]
/// (the parse-items read path), and collects the `criteria` section's
/// [`ParsedItem.title`](crate::parse::ParsedItem)s. Every failure — unknown type,
/// transient (location-less) type, missing file, unparseable file, or no `criteria`
/// section — is a routed blocking store-shaped [`Finding`].
fn read_spec_criteria(
    repo_root: &Path,
    schemas: &std::collections::BTreeMap<String, crate::schema::Schema>,
    spec_addr: &str,
) -> Result<Vec<String>, Finding> {
    let address = crate::address::Address::parse(spec_addr).map_err(|err| {
        store_block(
            "store.unparseable",
            format!("could not parse spec address `{spec_addr}`: {err}"),
            spec_addr,
            "supply a valid `<type>:<slug>` spec address".to_string(),
        )
    })?;
    let type_name = address.r#type.as_str();
    let slug = address.slug.as_str();

    let Some(schema) = schemas.get(type_name) else {
        return Err(store_block(
            "store.unknown-type",
            format!("unknown doctype `{type_name}` for `{spec_addr}`"),
            spec_addr,
            "list the available doctypes with `jigc describe`".to_string(),
        ));
    };

    let Some(path) = crate::store::canonical_path(repo_root, schema, slug) else {
        return Err(store_block(
            "store.transient-type",
            format!(
                "doctype `{type_name}` is transient (no `location:`); `{spec_addr}` is not committed"
            ),
            spec_addr,
            "the referenced doctype has no committed location".to_string(),
        ));
    };

    let mut source = std::fs::read_to_string(&path).map_err(|err| {
        store_block(
            "store.not-found",
            format!(
                "could not read `{spec_addr}` at `{}`: {err}",
                path.display()
            ),
            spec_addr,
            "create the referenced spec, or fix the address to an existing one".to_string(),
        )
    })?;
    crate::parse::strip_leading_bom(&mut source);

    let doc = crate::parse::parse_sections(schema, &source).map_err(|findings| {
        let why = findings
            .first()
            .map(|f| f.message.clone())
            .unwrap_or_else(|| "unparseable".to_string());
        store_block(
            "store.unparseable",
            format!(
                "`{spec_addr}` at `{}` does not parse: {why}",
                path.display()
            ),
            spec_addr,
            "fix the committed spec so it conforms to its schema".to_string(),
        )
    })?;

    let Some(section) = doc.sections.iter().find(|s| s.id == CRITERIA_SECTION) else {
        // `add-from-spec` takes any `<type>:<slug>`, so this fires on a real, clean doc
        // of a doctype that simply declares no `criteria` — a mistyped address, not a
        // malformed spec. The route hands over a runnable read of the doc actually
        // addressed and names the sections it does carry, rather than restating the
        // requirement (M47 inc-10, the P2-6 residue swept over this code's third
        // producer).
        let carried: Vec<String> = doc.sections.iter().map(|s| format!("`{}`", s.id)).collect();
        return Err(store_block(
            "store.no-such-section",
            format!("`{spec_addr}` names no `{CRITERIA_SECTION}` section to seed from"),
            spec_addr,
            crate::finding::Route::mechanical(
                ["jigc", "doc", "show", spec_addr],
                if carried.is_empty() {
                    " — it carries no sections at all; seeding reads a `criteria` section's items"
                        .to_string()
                } else {
                    format!(
                        " — it carries {}; seeding reads a `{CRITERIA_SECTION}` section's items",
                        carried.join(", "),
                    )
                },
            ),
        ));
    };

    Ok(section.items.iter().map(|i| i.title.clone()).collect())
}

/// Build a blocking store-shaped finding (the same code/route shape
/// [`crate::store`] surfaces) for a spec-read failure during seeding.
fn store_block(
    code: &str,
    message: String,
    address: &str,
    route: impl Into<crate::finding::Route>,
) -> Finding {
    Finding::graded(
        Severity::Blocking,
        code,
        message,
        Some(Location::addressed(address, 1, 1)),
        Some(route.into()),
    )
}

/// The zero-criteria block: a blocking finding naming the empty spec, routing the
/// agent to add criteria or seed differently — never a silent empty milestone
/// (`design/write-commands.md` → Minting a milestone: "nothing to seed from").
///
/// The first route arm names the **executable** path (round-2 D6h, verified on the
/// real binary): `jigc doc add-item <spec>#criteria --title … --task <id>` under a
/// task copies the committed spec in on first touch, and that task's finalize
/// re-promotes it — so the criteria repair is a normal in-task write, not a
/// hand-edit.
fn no_criteria_finding(spec_addr: &str) -> Finding {
    Finding::graded(
        Severity::Blocking,
        "milestone.no-criteria",
        format!("spec `{spec_addr}` has no criteria to seed from"),
        Some(Location::addressed(spec_addr, 1, 1)),
        Some(
            format!(
                "add criteria to the spec in a task — `jigc doc add-item {spec_addr}#criteria \
                 --title \"<criterion>\" --task <task-id>` copies the committed spec in on \
                 first touch, and that task's finalize re-promotes it — then re-run; or add \
                 sub-tasks directly with `jigc milestone add-task`"
            )
            .into(),
        ),
    )
}

/// The type-name a milestone sub-task mints under — the empty-intent fallback for
/// [`crate::state::mint_task`]. A sub-task is a `task` work-unit; the workflow it
/// runs is supplied separately and the fallback only fires on an empty slug.
const SUB_TASK_TYPE: &str = "task";

/// Slug `intent` into the sub-task id with the empty → `task` type-name fallback —
/// the *same* result [`crate::state::mint_task`] computes, recomputed here only to
/// key the within-milestone collision check before the mint (no separate
/// minting discipline).
fn mint_sub_id(intent: &str) -> String {
    let slug = crate::slug::slugify(intent);
    if slug.is_empty() {
        crate::slug::slugify(SUB_TASK_TYPE)
    } else {
        slug
    }
}

/// The unknown-milestone block: a blocking finding naming the missing milestone,
/// routing the agent to create it first (`write-commands.md` → add-task mints
/// *under* an existing milestone).
fn unknown_milestone_finding(milestone_id: &str) -> Finding {
    Finding::graded(
        Severity::Blocking,
        "milestone.unknown",
        format!("milestone `{milestone_id}` does not exist"),
        Some(Location::addressed(
            format!("milestone:{milestone_id}"),
            1,
            1,
        )),
        Some("create it first with `jigc milestone create \"<title>\"`".into()),
    )
}

/// The **stale-base block**: a blocking finding for a milestone whose recorded
/// base-SHA pin names a commit that **no longer exists** — a history rewrite orphaned
/// it (`design/team-ready-state.md` → Stale-base edge (history rewrite)). Routes the
/// human to restore the base commit or re-pin the milestone, never a silent git-op
/// against the missing commit (a partial worktree, a corrupt combine). The CLI owns the
/// git I/O that *detects* the missing commit; the engine owns only this finding shape
/// (no git I/O), the [`unknown_milestone_finding`] sibling.
pub fn stale_base_finding(milestone_id: &str, base_short: &str) -> Finding {
    Finding::graded(
        Severity::Blocking,
        "milestone.stale-base",
        format!(
            "milestone `{milestone_id}` is pinned to base `{base_short}`, which no longer exists (the base commit was rewritten away)"
        ),
        Some(Location::addressed(
            format!("milestone:{milestone_id}"),
            1,
            1,
        )),
        Some("restore the base commit, or re-pin the milestone's base, then re-run the op".into()),
    )
}

/// The within-milestone serial-collision block: a blocking finding naming the
/// already-listed sub-task id, routing the agent to a distinct intent (the `-2`
/// suffix is the join's, never incremental add; `write-commands.md` → add-task).
fn sub_task_collision_finding(milestone_id: &str, sub_id: &str) -> Finding {
    Finding::graded(
        Severity::Blocking,
        "milestone.sub-task-collision",
        format!("sub-task `{sub_id}` is already in milestone `{milestone_id}`"),
        Some(Location::addressed(format!("task:{sub_id}"), 1, 1)),
        Some("add the sub-task with a distinct intent".into()),
    )
}

/// Read the persisted [`TaskList`] of a milestone from its area
/// (`<jigc_root>/milestones/<id>/tasks.json`) — the companion of the mint write,
/// read back to enumerate the sub-tasks. A missing or malformed file is an error
/// (the list is written at mint, so its absence is a real fault).
pub fn read_task_list(milestone_dir: &Path) -> std::io::Result<TaskList> {
    let bytes = std::fs::read(milestone_dir.join(TASKS_FILE))?;
    serde_json::from_slice(&bytes)
        .map_err(|err| std::io::Error::new(std::io::ErrorKind::InvalidData, err))
}

/// The milestone whose task list carries `task_id` as a sub-task, if any — the
/// **membership discriminator** the per-task finalize guard keys on (round-2 D1;
/// the batch-C C0 verdict: the parent milestone's finalize is the *only* commit
/// boundary for a sub-task, so `jigc task finalize <sub-id>` has zero legitimate
/// use and is destructive in a fan-out worktree).
///
/// Scans `<jigc_root>/milestones/*/tasks.json` in **sorted directory order** (a
/// deterministic answer if an id ever appeared in two lists — the mint's
/// within-milestone collision check makes that unreachable in practice). A missing
/// or malformed area is skipped, never an error: a terminal milestone's torn-down
/// workbench simply no longer claims its sub-tasks (its `.jigc/tasks/<id>/` areas
/// are gone with it).
pub fn owning_milestone(jigc_root: &Path, task_id: &str) -> Option<String> {
    let milestones = jigc_root.join("milestones");
    let mut ids: Vec<String> = std::fs::read_dir(&milestones)
        .ok()?
        .filter_map(|entry| {
            let entry = entry.ok()?;
            entry
                .file_type()
                .ok()?
                .is_dir()
                .then(|| entry.file_name().to_string_lossy().into_owned())
        })
        .collect();
    ids.sort();
    ids.into_iter().find(|id| {
        read_task_list(&milestone_dir(jigc_root, id))
            .is_ok_and(|list| list.tasks.iter().any(|t| t == task_id))
    })
}

/// The **sub-task finalize refusal** (round-2 D1, the promote-clobber refusal
/// class — protecting an always-wrong destructive op, not preventing a legitimate
/// one): `jigc task finalize <sub-id>` on a milestone sub-task lands a commit on
/// the fan-out worktree's detached HEAD, empties the staged index the milestone
/// combine folds, and the later `jigc milestone finalize` silently lands WITHOUT
/// the work while the record claims the sub-task joined. The route names the one
/// real commit boundary, riding the checked [`Route`](crate::finding::Route)
/// constructor so the CLI-seam parse fence proves the argv.
pub fn sub_task_finalize_finding(milestone_id: &str, task_id: &str) -> Finding {
    Finding::graded(
        Severity::Blocking,
        "finalize.milestone-sub-task",
        format!(
            "task `{task_id}` is a sub-task of milestone `{milestone_id}` — the parent \
             milestone's finalize is the only commit boundary; a per-sub-task finalize \
             would land a commit outside it and strand this sub-task's work"
        ),
        Some(Location::addressed(format!("task:{task_id}"), 1, 1)),
        Some(crate::finding::Route::mechanical(
            ["jigc", "milestone", "finalize", milestone_id],
            " — the milestone finalize folds every sub-task's staged work into the one aggregate commit",
        )),
    )
}

/// Read the persisted single shared [`BasePin`] of a milestone from its area
/// (`<jigc_root>/milestones/<id>/base.json`) — the base every sub-task inherits.
/// A missing or malformed pin is an error (it is written at mint).
pub fn read_base_pin(milestone_dir: &Path) -> std::io::Result<BasePin> {
    let bytes = std::fs::read(milestone_dir.join(BASE_PIN_FILE))?;
    serde_json::from_slice(&bytes)
        .map_err(|err| std::io::Error::new(std::io::ErrorKind::InvalidData, err))
}

/// Render the record's `base` header field value — the shared base pin as the full
/// SHA and its abbreviated short SHA, space-joined (`<sha> <short>`). **Both** are
/// stored because the short is *not* recoverable from the full SHA: git's `--short`
/// is a variable-length unambiguous abbreviation, not a fixed-length prefix, so a
/// sha-only record could not round-trip a [`BasePin`] losslessly on a fresh-clone
/// re-derive (`design/team-ready-state.md` → Engine capability 2 (read-back);
/// `DECISIONS.md` 2026-07-07 → M39 Increment 3 T4). Mirrors `base.json`, which
/// likewise persists both `sha` and `short`.
fn render_base_field(base: &BasePin) -> String {
    format!("{} {}", base.sha, base.short)
}

/// Parse the record's `base` header field value back into a [`BasePin`] — the exact
/// inverse of [`render_base_field`]. The value is `<sha> <short>`; the short is read
/// from the **second** token, *never* re-derived as a prefix of the sha (the short's
/// length is not recoverable from the sha, so trusting `short = sha[..n]` would be
/// lossy). A single-token value (a legacy or hand-edited record that never stored the
/// short) degrades to `short == sha` rather than panicking; the read-back is lossless
/// only for records this module wrote.
fn parse_base_field(value: &str) -> BasePin {
    let mut parts = value.split_whitespace();
    let sha = parts.next().unwrap_or_default().to_string();
    let short = parts
        .next()
        .map(str::to_string)
        .unwrap_or_else(|| sha.clone());
    BasePin { sha, short }
}

/// **Re-derive a milestone's operational state from its committed record** — the
/// read-back half of Engine capability 2 (`design/team-ready-state.md` → Engine
/// capability 2 (read-back): the committed `.md` record is the source of truth and
/// the `.jigc/milestones/<id>/{base,tasks}.json` cache is rebuildable from it; M39
/// Increment 3 T4). Parses the record `source` against its pack `schema` and
/// reconstructs the `(BasePin, TaskList)` the JSON cache holds: the [`BasePin`] from
/// the `meta` header's `base` field (full + short SHA, [`parse_base_field`]) and the
/// [`TaskList`] from the `tasks` section's item ids — each item's frozen `{#id}`
/// anchor **is** the sub-task id (the `id-from: task-id` heading slug), the same ids
/// [`add_task`] appends to `tasks.json`.
///
/// Pure over the bytes: no `.jigc/` I/O, no git — the inverse of
/// [`render_fresh_record`] + [`append_task_item`], so re-deriving a record this
/// module wrote yields the exact `BasePin` (sha **and** short) and task-id set the
/// cache was seeded with. A record that does not conform to `schema`, or one missing
/// its `base` header field, is a routed blocking [`Finding`] (a real fault — the
/// record is this module's own materialized output).
pub fn read_back_record(
    schema: &crate::schema::Schema,
    source: &str,
) -> Result<(BasePin, TaskList), Finding> {
    let doc = crate::parse::parse_sections(schema, source)
        .map_err(|findings| read_back_finding(findings.first()))?;
    let base_value = doc
        .sections
        .iter()
        .find(|s| s.id == RECORD_HEADER_SECTION)
        .and_then(|s| s.fields.iter().find(|f| f.key == RECORD_BASE_FIELD))
        .map(|f| f.value.render())
        .ok_or_else(|| read_back_finding(None))?;
    let base = parse_base_field(&base_value);
    let tasks = doc
        .sections
        .iter()
        .find(|s| s.id == RECORD_TASKS_SECTION)
        .map(|s| s.items.iter().map(|i| i.id.clone()).collect())
        .unwrap_or_default();
    Ok((base, TaskList { tasks }))
}

/// **Re-seed the `.jigc` cache from the committed record when absent** — the
/// fresh-clone half of Engine capability 2 (`design/team-ready-state.md` → Engine
/// capability 2 (read-back): "on a fresh clone (no `.jigc/` working state) the first
/// milestone op parses the record back into `BasePin` + `TaskList` and re-seeds the
/// cache"; M39 Increment 3 T4). When either cache file (`base.json` / `tasks.json`)
/// is **absent** under `milestone_dir`, re-derives both from `record_source`
/// ([`read_back_record`]) and writes them in the same frozen byte form the mint
/// wrote — so the demoted cache is rebuilt from the source-of-truth record with no
/// loss. When both cache files are already present this is a **no-op** (the live
/// cache is authoritative for the session; staleness is not this arm's concern).
///
/// **A record in a terminal state does not re-seed a workbench** ([`terminal_status`];
/// `design/team-ready-state.md` → The lifecycle). Continuation is for a milestone that is
/// still *in flight*: once the record has settled — `discarded` (abandoned) or `joined`
/// (landed) — the milestone is over, there is nothing to continue, and the cache has
/// nothing to rebuild *for*. Re-seeding it would resurrect the very `.jigc/milestones/<id>/`
/// the terminal op tore down, from which `provision` re-provisions worktrees at the settled
/// base and `add-task` appends an `active` sub-task to a settled record — the lying committed
/// record back again. So a terminal record is a routed blocking [`Finding`] here, and because
/// **every** milestone verb passes through this primitive before its own guards, that one
/// refusal is what makes both terminals terminal (M42 completion-audit HIGH).
///
/// The live cache-resolve wiring (the CLI reading through this on every milestone op)
/// and the full fresh-clone `.jigc` delete are Inc 4; this is the engine primitive
/// they call.
pub fn reseed_cache_from_record(
    milestone_dir: &Path,
    schema: &crate::schema::Schema,
    record_source: &str,
) -> Result<(), Finding> {
    let id = milestone_dir
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("milestone-record");
    // **The terminal predicate — a record in a terminal state does not re-seed a workbench.**
    // Checked FIRST, ahead of the cache-present no-op: the refusal is a property of the
    // *record*, not of the cache's absence, so an interrupted teardown (record settled, the
    // area's removal never reached) refuses here too rather than running on a live cache.
    if let Some(status) = terminal_status(schema, record_source) {
        return Err(terminal_milestone_finding(id, &status));
    }
    if milestone_dir.join(BASE_PIN_FILE).exists() && milestone_dir.join(TASKS_FILE).exists() {
        return Ok(());
    }
    let (base, tasks) = read_back_record(schema, record_source)?;
    std::fs::create_dir_all(milestone_dir)
        .map_err(|err| io_finding(id, "open the milestone cache area", &err))?;
    crate::state::persist(
        &milestone_dir.join(BASE_PIN_FILE),
        render_base_pin(&base).as_bytes(),
    )
    .map_err(|err| io_finding(id, "re-seed the base pin cache", &err))?;
    crate::state::persist(&milestone_dir.join(TASKS_FILE), tasks.to_bytes().as_bytes())
        .map_err(|err| io_finding(id, "re-seed the task list cache", &err))?;
    Ok(())
}

/// **Rebuild every sub-task's WORKING AREA from the committed record** — the second half of
/// the fresh-clone reseed ([`reseed_cache_from_record`] is the first;
/// `design/team-ready-state.md` → Engine capability 2 (read-back), whose CLI site already
/// promises the teammate "resumes its un-joined sub-tasks from scratch"). M47 Increment 3 T3.
///
/// The milestone-cache half rebuilt only `.jigc/milestones/<id>/{base,tasks}.json`, so a fresh
/// clone knew *which* sub-tasks the milestone carries and nothing else: `.jigc/tasks/<sub>/`
/// stayed absent, and the milestone-execution workflow's own emitted
/// `` Spawn: `cd .jigc/worktrees/<sub> && jigc workflow <W> --task <sub>` `` line dead-ended on
/// *"no task"* — routed back at `jigc milestone list-tasks`, which names that same sub-task. A
/// loop, and the one T2's `milestone.zero-contribution` refusal routes a fresh-clone operator
/// into. This closes it: for every sub-task the record names, mint the standard working area
/// ([`crate::state::mint_task`], never a second minting discipline) at the recorded id, with the
/// milestone's shared `base` pin and the item's **verbatim** `intent`.
///
/// The recorded id is used as the slug **override**, never re-derived from the intent: the
/// record's `{#id}` anchor *is* the sub-task id the join, the worktree path and the spawn line
/// all key on, and re-slugging the intent would silently fork them the moment the slug rule
/// generation moves.
///
/// **Idempotent and never destructive**: a sub-task whose area already exists is skipped
/// untouched, so a live session's working area — including the `--workflow` it actually
/// recorded, and any staged docs under it — is never clobbered by a later milestone op.
///
/// **Each item's own recorded `workflow` is the minting workflow** (M49 Increment 9, T3). The
/// record used to carry `task-id`, `intent` and `status` and nothing about the minting
/// workflow, so this site rebuilt every area under the single caller-supplied default: an
/// `add-task --workflow <other>` override was workbench-local, did not survive a clone, and
/// the operator's own `jigc workflow <other> --task <id>` re-entry was then refused by the
/// re-entry W-equality guard **naming a workflow this function had itself invented** — false
/// provenance asserted with full confidence. Schema-version 3 put the leaf in the record, so
/// each area is rebuilt under the workflow its own item records.
///
/// **`default_workflow_id` is the caller's fall-back, for an item with no leaf** (the engine
/// ships empty of pack content, so the pack's default sub-task workflow is a CLI fact). Every
/// record committed before the bump carries no `workflow` bullet on any item — the migration
/// adds none, the leaf being machine-maintained and `default`-less — so the fall-back is what
/// keeps those milestones resumable. It is the value `add-task`/`add-from-spec` recorded
/// absent an override, so a pre-bump record resumes exactly as it did.
///
/// **An item the record has settled is not rebuilt** ([`item_is_settled`], the shared predicate
/// over the item's own `status` leaf). The rule the premise here used to rest on — *every item the record names is
/// rebuilt, because both terminals flip the header and every item in one write, and a terminal
/// header never reaches here* — was falsified the moment a **per-item** terminal existed: M49's
/// `jigc task discard <sub-id>` settles one item to `discarded` while the milestone stays
/// `active`, so filtering on the area's absence alone made every door reaching this site
/// resurrect the area the discard had just removed. So the skip keys on the *item's* recorded
/// status, through the same terminal vocabulary the header guard uses — an item that is over is
/// over on both loci. An item with **no** status leaf is rebuilt, as before: a record that never
/// claimed the sub-task settled cannot be read as claiming it.
/// **No staged snapshot is written here either** — the second `Exempt` member of
/// [`crate::state::MINT_DOORS`], on the same premise, plus this being a *rebuild* of an
/// area the committed record already names rather than a door a user staged work in
/// front of.
pub fn reseed_sub_task_areas(
    jigc_root: &Path,
    schema: &crate::schema::Schema,
    record_source: &str,
    default_workflow_id: &str,
) -> Result<(), Finding> {
    let doc = crate::parse::parse_sections(schema, record_source)
        .map_err(|findings| read_back_finding(findings.first()))?;
    let base = doc
        .sections
        .iter()
        .find(|s| s.id == RECORD_HEADER_SECTION)
        .and_then(|s| s.fields.iter().find(|f| f.key == RECORD_BASE_FIELD))
        .map(|f| parse_base_field(&f.value.render()))
        .ok_or_else(|| read_back_finding(None))?;
    let Some(tasks) = doc.sections.iter().find(|s| s.id == RECORD_TASKS_SECTION) else {
        return Ok(());
    };
    for item in &tasks.items {
        if jigc_root.join("tasks").join(&item.id).exists() {
            continue;
        }
        if item_is_settled(item) {
            continue;
        }
        let leaf = |key: &str| {
            item.fields
                .iter()
                .find(|f| f.key == key)
                .map(|f| f.value.render())
        };
        let intent = leaf(RECORD_TASK_INTENT_FIELD).unwrap_or_default();
        // The item's own recorded workflow, never the caller's default — that default is
        // reached only by a pre-bump item, which carries no leaf at all.
        let workflow = leaf(RECORD_TASK_WORKFLOW_FIELD)
            .filter(|value| !value.trim().is_empty())
            .unwrap_or_else(|| default_workflow_id.to_string());
        crate::state::mint_task(
            jigc_root,
            &intent,
            SUB_TASK_TYPE,
            &workflow,
            base.clone(),
            Some(&item.id),
        )?;
    }
    Ok(())
}

/// The **header `status`** a committed record reads — whatever it says (`active`, `joined`,
/// `discarded`), read from the record itself, which is the source of truth for a milestone's
/// lifecycle ([Engine capability 2]; the `.jigc` JSON cache carries no status at all, which is
/// exactly why the workbench's presence or absence can never answer this question —
/// `design/team-ready-state.md` → The lifecycle).
///
/// `None` when the record does not conform or carries no header `status` leaf. A *missing*
/// status is deliberately **not** read as terminal: the callers that gate on this refuse work,
/// and refusing on an unreadable record would strand a milestone whose real fault is a
/// malformed record — that fault surfaces through the ordinary read-back path
/// ([`read_back_record`]), which routes it to reconcile.
///
/// [Engine capability 2]: reseed_cache_from_record
pub fn record_status(schema: &crate::schema::Schema, source: &str) -> Option<String> {
    let doc = crate::parse::parse_sections(schema, source).ok()?;
    doc.sections
        .iter()
        .find(|s| s.id == RECORD_HEADER_SECTION)
        .and_then(|s| s.fields.iter().find(|f| f.key == RECORD_STATUS_FIELD))
        .map(|f| f.value.render())
}

/// Whether a record's header `status` has **settled** — [`joined`](RECORD_STATUS_JOINED) or
/// [`discarded`](RECORD_STATUS_DISCARDED). The two differ in *why* the milestone is over
/// (its work landed / it was abandoned) and the record says which, but they are **one state**
/// operationally: the milestone is over and has no workbench. Every guard in this family keys
/// on that one predicate rather than on either member, so neither terminal can be secured
/// while the other leaks.
pub fn is_terminal_status(status: &str) -> bool {
    RECORD_TERMINAL_STATUSES.contains(&status)
}

/// The record's header `status` **when it is terminal** — [`record_status`] filtered through
/// [`is_terminal_status`]; the guard predicate itself.
pub fn terminal_status(schema: &crate::schema::Schema, source: &str) -> Option<String> {
    record_status(schema, source).filter(|s| is_terminal_status(s))
}

/// The blocking finding a **settled** milestone's verbs refuse with (`design/team-ready-state.md`
/// → The lifecycle): the milestone is over — `discarded` (abandoned) or `joined` (landed) — so it
/// has no workbench, and none may be rebuilt for it.
///
/// The route names the surface that *can* still serve the operator: the committed record is the
/// source of truth and stays fully readable through `jigc doc show`
/// (`design/doc-read-surface.md`) — the settled record, its sub-tasks and their statuses, all of
/// it. What is refused is *operating* on a milestone that is over, never *reading* it. New work
/// starts as a new milestone (the settled record keeps the id).
pub fn terminal_milestone_finding(milestone_id: &str, status: &str) -> Finding {
    Finding::graded(
        Severity::Blocking,
        "milestone.terminal",
        format!(
            "milestone `{milestone_id}` is `{status}` — a settled milestone is over and has no workbench"
        ),
        Some(Location::addressed(
            format!("milestone:{milestone_id}"),
            1,
            1,
        )),
        Some(format!(
            "read the settled record with `jigc doc show milestone-record:{milestone_id}`; new work starts a new milestone (`jigc milestone create \"<title>\"`)"
        ).into()),
    )
}

/// The blocking finding `milestone create` refuses an **already-owned id** with: a record
/// already lives at the minted slug's canonical home (`design/team-ready-state.md` → The
/// lifecycle; M42 completion-audit HIGH).
///
/// The mint's own collision check reads the **workbench** (`.jigc/milestones/<id>/`), which is
/// gitignored, disposable, and *absent by design* in two legitimate states — a fresh clone of an
/// in-flight milestone, and a settled one whose terminal op tore it down. So the workbench cannot
/// answer *"is this id taken?"*; only the record's home can, and it is the record — never the
/// cache — that a re-mint would **overwrite**. `status` is the existing record's header status
/// (`None` if it does not read), which decides the route: a **live** record is *continued*, a
/// **settled** one is over and its id is spent.
///
/// **The message says "a record", never "a committed record"** (M47 Inc 2 T2; law 1 —
/// `design/surface-contract.md`). The caller probes the record path's **existence on disk** and
/// runs no git read, so it cannot know the file ever landed: a hand-drafted record, or (before
/// the record-commit transaction shipped) the residue of a rejected `create`, is present and
/// uncommitted, and claiming a commit that never happened is exactly the class of lie the
/// caller's own wave exists to remove. The refusal and its route are unchanged — only the
/// unfounded provenance claim goes.
pub fn record_exists_finding(milestone_id: &str, status: Option<&str>) -> Finding {
    let reads = status
        .map(|s| format!(" (its record reads `{s}`)"))
        .unwrap_or_default();
    let route = match status {
        Some(s) if is_terminal_status(s) => format!(
            "that milestone is over — its record is settled at `{s}` and re-minting the id would overwrite it; create this one under a different title (or read the settled record with `jigc doc show milestone-record:{milestone_id}`)"
        ),
        _ => format!(
            "continue it with `jigc milestone add-task {milestone_id} \"<intent>\"`, or create this one under a different title"
        ),
    };
    Finding::graded(
        Severity::Blocking,
        "milestone.record-exists",
        format!("milestone `{milestone_id}` already has a record{reads}"),
        Some(Location::addressed(
            format!("milestone:{milestone_id}"),
            1,
            1,
        )),
        Some(route.into()),
    )
}

/// A blocking finding for a record that could not be read back into operational
/// state — it did not conform to its schema, or it carried no `base` header field (a
/// real fault: the record is this module's own materialized output). Routed to
/// reconcile the record, then re-run the milestone op.
fn read_back_finding(cause: Option<&Finding>) -> Finding {
    let why = cause
        .map(|f| f.message.clone())
        .unwrap_or_else(|| "no `base` header field".to_string());
    Finding::graded(
        Severity::Blocking,
        "milestone.record-read-back",
        format!("could not re-derive milestone state from its record: {why}"),
        Some(Location::addressed("milestone-record", 1, 1)),
        Some("reconcile the milestone record, then re-run the milestone op".into()),
    )
}

/// **Materialize a fresh `milestone-record` doc body** from a milestone's shared
/// base pin — the create/materialize write arm (`design/team-ready-state.md` →
/// Engine capability 1 (write), the `set: on-create` materialization; M39
/// Increment 3). Builds the canonical byte form against the pack-supplied `schema`:
/// the `meta` header carrying the `base` SHA (`set: on-create`), `status: active`
/// (`set: on-transition`, seeded active at create), and — when the schema declares
/// the injected stamp field — the `schema-version` stamp, the `# <milestone_id>` H1, and
/// an **empty** repeatable `tasks` section — no sub-task appended yet (`add_task` is
/// the incremental populator, the mint-site precedent). The result is the committed
/// record's source-of-truth bytes; the CLI writes + path-scoped-commits them.
///
/// The engine stays **clock-free and LLM-free**: `base` is the caller-supplied SHA
/// (the CLI read HEAD, "CLI orchestrates, git executes"), `status: active` is a
/// structural constant, and `schema_version` is the caller-supplied manifest version
/// (the CLI's `set: schema-version` deriver value — the manifest lives CLI-side), so
/// this is a pure function of (`schema`, `milestone_id`, `base`, `schema_version`) →
/// bytes — golden-testable and the parser's inverse (`render → parse` round-trips
/// the novel all-machine-set header + empty-repeatable shape).
///
/// The stamp is materialized **only when `schema` declares it** — a stamp-injected
/// schema (the manifest-frozen production load, `load_pack_schema`) renders
/// `schema-version: <v>` in the header so the fresh mint satisfies the store-scope
/// stamp demand; a stamp-free schema (a manifest-less pack, the bare engine load)
/// renders exactly the prior shape, mirroring the load-time injection gate.
pub fn render_fresh_record(
    schema: &crate::schema::Schema,
    milestone_id: &str,
    base: &BasePin,
    schema_version: u32,
) -> String {
    use crate::field_block::{Field, Value};
    use crate::write::{Instance, SectionContent};

    let mut header_fields = vec![
        Field {
            key: RECORD_BASE_FIELD.to_string(),
            // Both the full SHA and the short — the short is not a fixed
            // prefix of the sha, so a sha-only record could not round-trip
            // a `BasePin` losslessly ([`render_base_field`]).
            value: Value::Scalar(render_base_field(base)),
        },
        Field {
            key: RECORD_STATUS_FIELD.to_string(),
            value: Value::Scalar(RECORD_STATUS_ACTIVE.to_string()),
        },
    ];
    let declares_stamp = schema.sections.iter().any(|s| {
        s.header
            && match &s.body {
                crate::schema::SectionBody::Simple { fields, .. } => fields
                    .iter()
                    .any(|f| f.id == crate::schema::SCHEMA_VERSION_FIELD),
                crate::schema::SectionBody::Repeatable { .. } => false,
            }
    });
    if declares_stamp {
        header_fields.push(Field {
            key: crate::schema::SCHEMA_VERSION_FIELD.to_string(),
            value: Value::Scalar(schema_version.to_string()),
        });
    }

    let instance = Instance {
        title: milestone_id.to_string(),
        sections: vec![
            SectionContent {
                id: RECORD_HEADER_SECTION.to_string(),
                fields: header_fields,
                ..Default::default()
            },
            // The empty repeatable `tasks` section — no sub-task materialized yet.
            SectionContent {
                id: RECORD_TASKS_SECTION.to_string(),
                ..Default::default()
            },
        ],
    };
    crate::write::render(schema, &instance)
}

/// **Append one sub-task item to a `milestone-record`'s `tasks` section** — the
/// `add-task` write arm (`design/team-ready-state.md` → Engine capability 1 (write),
/// the `add-task` — append arm; M39 Increment 3). Materializes one `tasks` item —
/// `task-id` (the `id-from` heading, so `task_id` IS the item title) plus the
/// `set: on-transition` leaves `intent`, `workflow` and `status: active` (the CLI-supplied
/// machine-set values a freshly-added sub-task carries until `join`) — via the M16
/// [`crate::write::add_item`] primitive over the pack-supplied `schema`.
///
/// **`workflow` is the minting workflow, recorded because the record is the fresh-clone
/// continuation state** (schema-version 3, M49 Increment 9). It was the one thing the record
/// never carried, so it lived only in the gitignored `.jigc/tasks/<sub>/workflow` file and a
/// `--workflow` override did not survive a clone; [`reseed_sub_task_areas`] then rebuilt the
/// area under the caller's default and the re-entry W-equality guard refused the operator's
/// own override, naming a workflow the tool had itself invented. The caller supplies the id
/// it actually minted against — the engine ships empty of pack content and validates no
/// membership here (the CLI's `add-task`/`add-from-spec` doors check it against the loaded
/// packs before anything mints).
///
/// A **distinct operation** from the `join` in-place mutate (the design's F5 two-arm
/// census): this is a pure **append**, ordered and byte-stable — the primitive
/// re-renders any prior last item canonically, so the bytes outside the appended item's
/// span are byte-identical and the result re-parses. The engine stays clock-free and
/// LLM-free: every value is the caller-supplied structural state (`task_id`/`intent`)
/// or the `active` constant, so this is a pure function of
/// (`schema`, `source`, `task_id`, `intent`) → bytes.
///
/// A `task_id` with no slug-able content (an empty minted anchor) or one already present
/// in the section surfaces the primitive's [`GenerateError`](crate::write::GenerateError)
/// unchanged — the CLI wiring maps it to a routed finding.
pub fn append_task_item(
    schema: &crate::schema::Schema,
    source: &str,
    task_id: &str,
    intent: &str,
    workflow: &str,
) -> Result<String, crate::write::GenerateError> {
    use crate::field_block::{Field, Value};

    // The `set: on-transition` leaves in schema block order (the id-from `task-id` is the
    // heading, not a bullet), materialized to the fresh-append values: the recorded
    // `intent`, the minting `workflow`, seeded `status: active` until `join` transitions
    // it to `joined`.
    let fields = vec![
        Field {
            key: RECORD_TASK_INTENT_FIELD.to_string(),
            value: Value::Scalar(intent.to_string()),
        },
        Field {
            key: RECORD_TASK_WORKFLOW_FIELD.to_string(),
            value: Value::Scalar(workflow.to_string()),
        },
        Field {
            key: RECORD_STATUS_FIELD.to_string(),
            value: Value::Scalar(RECORD_STATUS_ACTIVE.to_string()),
        },
    ];
    // No `slug_override` and no `slot` pre-fill: a record item's `{#id}` IS the
    // sub-task's work-unit id (`id-from: task-id`), so the mint must slug it, never
    // take an override.
    crate::write::add_item(
        schema,
        source,
        RECORD_TASKS_SECTION,
        task_id,
        None,
        None,
        &fields,
    )
}

/// **Flip a `milestone-record` to `joined` in place** — the `join` write arm
/// (`design/team-ready-state.md` → Engine capability 1 (write), the `join` — in-place
/// mutate arm; M39 Increment 3). Reads the **committed record** at `record_path`,
/// rewrites the machine-set `status` field of **every already-committed `tasks` item**
/// (active → joined) and the header `status`, and writes the result back — the
/// direct-record-file read/edit/write plumbing (net-new vs. the task-scoped author
/// buffer path: the join arm writes the committed record directly, outside any task
/// working area). Returns the joined bytes (the CLI folds them into the join commit).
///
/// A **distinct operation** from the `add-task` append (the design's F5 two-arm census):
/// this is an in-place rewrite of N existing item-leaves, each routed through the
/// byte-stable [`crate::write::set_item_field`] value splice, plus the header
/// [`crate::write::set_field`] splice. The engine stays clock-free and LLM-free: the
/// only value written is the `joined` structural constant, so the transform is a pure
/// function of (`schema`, on-disk bytes) → bytes. Byte-stability of the in-place rewrite
/// is the red obligation — the joined record is byte-identical to the committed one
/// modulo exactly the flipped `status` values.
///
/// A record that does not read, does not conform, or lacks a targeted `status` leaf
/// surfaces a routed blocking [`Finding`]; nothing partial is committed (the write lands
/// only after every splice succeeds).
pub fn join_record(
    record_path: &Path,
    schema: &crate::schema::Schema,
    milestone_id: &str,
) -> Result<String, Finding> {
    let source = std::fs::read_to_string(record_path)
        .map_err(|err| io_finding(milestone_id, "read the milestone record", &err))?;
    let flipped = flip_record_status_to_joined(schema, &source)
        .map_err(|err| record_flip_finding(milestone_id, RECORD_STATUS_JOINED, "join", err))?;
    std::fs::write(record_path, &flipped)
        .map_err(|err| io_finding(milestone_id, "write the joined milestone record", &err))?;
    Ok(flipped)
}

/// The pure in-place status flip behind [`join_record`]: read the committed `tasks` item
/// ids from the record itself (the source of truth), splice each **unsettled** item's
/// `status` value to `joined` via the byte-stable item-leaf splice threading the updated
/// bytes item by item, then splice the header `status` (front-matter, block-order-first). A
/// non-conformant record, or a `status` leaf the splice cannot locate, is a
/// [`SpliceError`](crate::write::SpliceError) the caller routes.
///
/// **An item the record has already settled is left byte-untouched** ([`item_is_settled`]) —
/// the mirror of [`discard_sub_task_item`]'s joined carve-out, and the same rule stated on
/// the same predicate: a settled item is over, and a terminal the milestone boundary did not
/// produce is not the boundary's to overwrite. Without it a `jigc task discard <sub-id>` was
/// undone by the very next `milestone finalize`, which wrote `joined` over the abandoned
/// sub-task — the record lying about abandoned work, which is the defect the discard door
/// exists to close. An already-`joined` item is skipped for the same reason and to the same
/// bytes (its splice was a no-op).
fn flip_record_status_to_joined(
    schema: &crate::schema::Schema,
    source: &str,
) -> Result<String, crate::write::SpliceError> {
    let doc = crate::parse::parse_sections(schema, source)
        .map_err(|findings| crate::write::SpliceError::NotConformant { findings })?;
    let item_ids: Vec<String> = doc
        .sections
        .iter()
        .find(|s| s.id == RECORD_TASKS_SECTION)
        .map(|s| {
            s.items
                .iter()
                .filter(|i| !item_is_settled(i))
                .map(|i| i.id.clone())
                .collect()
        })
        .unwrap_or_default();

    let mut body = source.to_string();
    for item_id in &item_ids {
        body = crate::write::set_item_field(
            schema,
            &body,
            RECORD_TASKS_SECTION,
            item_id,
            RECORD_STATUS_FIELD,
            RECORD_STATUS_JOINED,
        )?;
    }
    crate::write::set_field(
        schema,
        &body,
        RECORD_HEADER_SECTION,
        RECORD_STATUS_FIELD,
        RECORD_STATUS_JOINED,
    )
}

/// **Settle a `milestone-record` to `discarded` in place** — the `discard` write arm
/// (`design/team-ready-state.md` → `jigc milestone discard <id>`, "Per-item semantics —
/// a joined sub-task stays joined"; M42 Increment 7). Reads the **committed record** at
/// `record_path`, settles it to the abandon terminal, and writes it back — the same
/// direct-record-file plumbing [`join_record`] uses, returning the settled bytes (the
/// CLI commits them record-only).
///
/// The [`join_record`] sibling with **one** semantic difference: the item flip is
/// **per-item conditional**. Every `tasks` item whose committed `status` is **not**
/// `joined` flips to `discarded`; a **genuinely joined** item is left **byte-untouched**
/// — it really did land, its code and docs are in the history, and flipping it would make
/// the record lie about **landed work**. The header always flips to `discarded`. So a
/// partially-joined milestone that was then abandoned reads as exactly what happened, on
/// both axes.
///
/// Each item's committed `status` is read from the record itself ([`crate::parse::ParsedItem`]'s
/// fields) — the committed record is its own source of truth ([Engine capability 2]:
/// the `.jigc` JSON is a rebuildable cache, never consulted here). The engine stays
/// clock-free and LLM-free: the only value written is the `discarded` structural
/// constant, so the transform is a pure function of (`schema`, on-disk bytes) → bytes,
/// spliced through the byte-stable [`crate::write::set_item_field`] / [`crate::write::set_field`]
/// path.
///
/// A record that does not read, does not conform, or lacks a targeted `status` leaf
/// surfaces a routed blocking [`Finding`]; nothing partial is written (the file lands
/// only after every splice succeeds).
///
/// [Engine capability 2]: join_record
pub fn discard_record(
    record_path: &Path,
    schema: &crate::schema::Schema,
    milestone_id: &str,
) -> Result<String, Finding> {
    let source = std::fs::read_to_string(record_path)
        .map_err(|err| io_finding(milestone_id, "read the milestone record", &err))?;
    let flipped = flip_record_status_to_discarded(schema, &source).map_err(|err| {
        record_flip_finding(milestone_id, RECORD_STATUS_DISCARDED, "discard", err)
    })?;
    std::fs::write(record_path, &flipped)
        .map_err(|err| io_finding(milestone_id, "write the discarded milestone record", &err))?;
    Ok(flipped)
}

/// **Settle ONE `tasks` item to `discarded` in place** — the per-sub-task abandon arm
/// (`design/team-ready-state.md` → The lifecycle; M49 Increment 2 / T3). Reads the
/// **committed record** at `record_path`, splices that one item's machine-set `status` leaf
/// to [`RECORD_STATUS_DISCARDED`], and writes it back — the same direct-record-file plumbing
/// [`join_record`] / [`discard_record`] use, returning the settled bytes (the CLI commits
/// them record-only).
///
/// The [`discard_record`] sibling narrowed from *the whole record* to **one item**: the
/// milestone is still in flight, so the header `status` and every other item are left
/// **byte-untouched**. `jigc task discard <sub-id>` removed the sub-task's working area while
/// the record went on calling it `active` forever — a committed record lying about abandoned
/// work, and (through [`reseed_sub_task_areas`]) an area the next milestone door happily
/// rebuilt.
///
/// The flip is **unconditional on the item's current value**, unlike [`discard_record`]'s
/// joined-item carve-out: this arm is reachable only while the sub-task has a live working
/// area, and a `joined` item's area no longer exists (both terminals are written at
/// `finalize`, whose teardown removes the workbench, after which the CLI door refuses at task
/// resolution). A guard here would be code no state can reach.
///
/// An unknown `task_id`, a record that does not conform, or a vanished `status` leaf surfaces
/// a routed blocking [`Finding`]; nothing is written unless the splice succeeds.
pub fn discard_sub_task_item(
    record_path: &Path,
    schema: &crate::schema::Schema,
    milestone_id: &str,
    task_id: &str,
) -> Result<String, Finding> {
    let source = std::fs::read_to_string(record_path)
        .map_err(|err| io_finding(milestone_id, "read the milestone record", &err))?;
    let settled = crate::write::set_item_field(
        schema,
        &source,
        RECORD_TASKS_SECTION,
        task_id,
        RECORD_STATUS_FIELD,
        RECORD_STATUS_DISCARDED,
    )
    .map_err(|err| {
        record_flip_finding(milestone_id, RECORD_STATUS_DISCARDED, "task discard", err)
    })?;
    std::fs::write(record_path, &settled).map_err(|err| {
        io_finding(
            milestone_id,
            "write the settled sub-task's milestone record",
            &err,
        )
    })?;
    Ok(settled)
}

/// The pure in-place settle behind [`discard_record`]: read the committed `tasks` items
/// from the record itself (the source of truth), select **only the non-joined** ones by
/// their committed `status` leaf, splice each one's `status` to `discarded` via the
/// byte-stable item-leaf splice (threading the updated bytes item by item), then splice
/// the header `status`. A **joined** item is never named to the splice, so not one of its
/// bytes moves. A non-conformant record, or a `status` leaf the splice cannot locate, is a
/// [`SpliceError`](crate::write::SpliceError) the caller routes.
fn flip_record_status_to_discarded(
    schema: &crate::schema::Schema,
    source: &str,
) -> Result<String, crate::write::SpliceError> {
    let doc = crate::parse::parse_sections(schema, source)
        .map_err(|findings| crate::write::SpliceError::NotConformant { findings })?;
    let item_ids: Vec<String> = doc
        .sections
        .iter()
        .find(|s| s.id == RECORD_TASKS_SECTION)
        .map(|s| {
            s.items
                .iter()
                .filter(|item| !item_is_joined(item))
                .map(|item| item.id.clone())
                .collect()
        })
        .unwrap_or_default();

    let mut body = source.to_string();
    for item_id in &item_ids {
        body = crate::write::set_item_field(
            schema,
            &body,
            RECORD_TASKS_SECTION,
            item_id,
            RECORD_STATUS_FIELD,
            RECORD_STATUS_DISCARDED,
        )?;
    }
    crate::write::set_field(
        schema,
        &body,
        RECORD_HEADER_SECTION,
        RECORD_STATUS_FIELD,
        RECORD_STATUS_DISCARDED,
    )
}

/// Whether a committed `tasks` item **genuinely joined** — its committed `status` leaf
/// reads [`RECORD_STATUS_JOINED`]. The one predicate that separates the `discard` arm
/// from its `join` sibling: a joined sub-task really did land, so `discard` leaves it
/// alone. A missing status leaf is **not** joined (a record that never claimed the work
/// landed cannot be read as claiming it), so it settles to `discarded` like any other
/// non-joined item.
fn item_is_joined(item: &crate::parse::ParsedItem) -> bool {
    item_status(item).as_deref() == Some(RECORD_STATUS_JOINED)
}

/// One committed `tasks` item's `status` leaf as the record renders it, `None` when the item
/// carries none. The single reader of a per-item status: the `discard` arm's joined carve-out
/// ([`item_is_joined`]) and the settled predicate every other locus keys on
/// ([`item_is_settled`]) ask the same question of the same leaf, so none can be secured while
/// another reads elsewhere.
fn item_status(item: &crate::parse::ParsedItem) -> Option<String> {
    item.fields
        .iter()
        .find(|f| f.key == RECORD_STATUS_FIELD)
        .map(|f| f.value.render())
}

/// Whether a committed `tasks` item has **settled** — its own `status` leaf reads a terminal
/// ([`is_terminal_status`]: `joined` or `discarded`). The per-item sibling of the header's
/// [`terminal_status`], and the single predicate every locus that must not treat a settled
/// sub-task as live asks: the reseed's rebuild skip ([`reseed_sub_task_areas`]), the operating
/// doors' enumeration ([`settled_sub_task_ids`]), and the `join` flip's carve-out
/// ([`flip_record_status_to_joined`]). An item with **no** status leaf is not settled — a
/// record that never claimed the sub-task was over cannot be read as claiming it.
fn item_is_settled(item: &crate::parse::ParsedItem) -> bool {
    item_status(item).as_deref().is_some_and(is_terminal_status)
}

/// **The sub-task ids the committed record has settled** — every `tasks` item whose own
/// `status` leaf is terminal ([`item_is_settled`]). The set the milestone's *operating* doors
/// subtract from their enumeration: a settled sub-task owns no working area (the discard
/// removed it and [`reseed_sub_task_areas`] never rebuilds it), so enumerating it as live is
/// how a door emits work nobody can do — the `Spawn:` line that dead-ends on *"no task"* and
/// routes straight back at `jigc milestone list-tasks`, which names it again. A **loop**, and
/// the same one the reseed skip exists to close, reached through the other seam.
///
/// Pure over the bytes, like its [`read_back_record`] sibling: a record that does not parse
/// carries no settled ids, so an unreadable record subtracts nothing and every caller's own
/// read-back guard still surfaces the fault.
///
/// The **teardown** doors deliberately do not subtract this set: a sub-task discard removes
/// the working area and leaves the provisioned worktree standing, so the milestone's abandon
/// and finalize teardowns stay the last doors able to remove it.
pub fn settled_sub_task_ids(
    schema: &crate::schema::Schema,
    source: &str,
) -> std::collections::BTreeSet<String> {
    let Ok(doc) = crate::parse::parse_sections(schema, source) else {
        return std::collections::BTreeSet::new();
    };
    doc.sections
        .iter()
        .find(|s| s.id == RECORD_TASKS_SECTION)
        .map(|s| {
            s.items
                .iter()
                .filter(|i| item_is_settled(i))
                .map(|i| i.id.clone())
                .collect()
        })
        .unwrap_or_default()
}

/// A blocking finding for a `status` splice failure while flipping a milestone record to
/// its `target` terminal (`joined` at the `join` op, `discarded` at `discard` — named by
/// `op`, so the route says which one to re-run) — the record did not conform or a targeted
/// `status` leaf vanished (a real fault: the record is the record arms' own materialized
/// output). Routed to reconcile.
fn record_flip_finding(
    milestone_id: &str,
    target: &str,
    op: &str,
    err: crate::write::SpliceError,
) -> Finding {
    Finding::graded(
        Severity::Blocking,
        "milestone.record-flip",
        format!("could not flip milestone record `{milestone_id}` to {target}: {err}"),
        Some(Location::addressed(
            format!("milestone-record:{milestone_id}"),
            1,
            1,
        )),
        Some(format!("reconcile the milestone record, then re-run the {op}").into()),
    )
}

/// One staged doc folded into the parent working overlay at the by-task-id join
/// (`design/storage.md` → The by-task-id join, algorithm step 2: "disjoint union of
/// staged docs, classified by provenance"). Carries the discriminator the clash rule
/// (T2) and cross-area rule (T3) will key on, plus the sub-task it came from (so a
/// later rule can name the offending area) and the doc's forward edges (the per-area
/// `overlay_working` derivation's output for this `from`, reused unchanged).
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct MergedDoc {
    /// How the doc came to be staged — `created` (minted here) vs `edited-from-base`
    /// (copied in from the committed store); the bit the join's clash rule needs.
    pub provenance: crate::state::Provenance,
    /// The sub-task id whose area contributed this staged doc.
    pub source_task: String,
    /// The doc's forward edges, sorted by `(from, relation, to)` — the same edges the
    /// single-area [`crate::index::overlay_working`] derivation emits for this `from`.
    pub edges: Vec<crate::index::Edge>,
}

/// The result of a by-task-id join (`design/storage.md` → The by-task-id join). The
/// `overlay` is the parent working overlay the merge produced — an **address-keyed
/// [`BTreeMap`](std::collections::BTreeMap)** so iteration is id-sorted and no
/// enumeration / `read_dir` / completion order can leak into the output (Validation
/// hardening #7). `findings` is the **ordered** finding list the join surfaces; in this
/// (skeleton) increment it is always empty — the provenance clash / collision-suffix
/// rules (T2) and the cross-area / isolation checks (T3) populate it later.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct JoinOutcome {
    /// The merged staged docs, keyed by `<type>:<slug>` address (id-sorted iteration).
    pub overlay: std::collections::BTreeMap<String, MergedDoc>,
    /// The join's findings, in a stable order (empty in the skeleton increment). A
    /// [`Findings`] — the sanctioned findings-collection projection, carrying the uniqueness
    /// half of the membership test on its `Serialize` (`command-output-contract.md` → The
    /// membership test); it derefs to `[Finding]` and its wire shape is the plain array.
    pub findings: Findings,
}

/// **The by-task-id join skeleton** (edge-index lifecycle **site 4**;
/// `design/storage.md` → The by-task-id join, algorithm steps 1–2). Enumerate the
/// milestone's sub-task areas by their **sorted task id** ([`TaskList::enumerate`],
/// never `read_dir` / filesystem / completion order) and fold each area's staged docs
/// into the parent working `overlay`, **classified by provenance**
/// ([`crate::state::ProvenanceRecord::load`]). The per-area basis is the single-area
/// working-overlay derivation reused unchanged ([`crate::index::overlay_working`]),
/// so each merged doc carries the same forward edges that derivation emits.
///
/// The merge is a **pure function of the set of sub-task areas**: every accumulator is
/// id-keyed (the address-keyed [`BTreeMap`](std::collections::BTreeMap) `overlay`), so
/// enumeration order cannot reach the output — the same set in yields the byte-identical
/// outcome out regardless of order, the property M7 exists to prove.
///
/// This is the **skeleton**: it assumes the sub-areas stage **disjoint** slugs (no
/// contention). The provenance clash rule + collision-suffix (T2) and the per-sub-area
/// cross-area / join-time isolation checks (T3) layer onto this fold; until then
/// [`JoinOutcome::findings`] is empty. **Unknown milestone** → the same blocking
/// [`unknown_milestone_finding`] the add path raises, before any area is read.
pub fn join(
    jigc_root: &Path,
    repo_root: &Path,
    milestone_id: &str,
    schemas: &std::collections::BTreeMap<String, crate::schema::Schema>,
    committed: &crate::index::EdgeIndex,
) -> Result<JoinOutcome, Finding> {
    let dir = milestone_dir(jigc_root, milestone_id);

    // Unknown milestone → reject before any sub-area is read.
    if !dir.is_dir() {
        return Err(unknown_milestone_finding(milestone_id));
    }

    let list =
        read_task_list(&dir).map_err(|err| io_finding(milestone_id, "read the task list", &err))?;

    // Step 1: enumerate the sub-task areas by sorted task id (never read_dir order),
    // then fold. The fold itself is order-invariant (its only accumulator is the
    // address-keyed `BTreeMap`), so feeding `enumerate()`'s sorted order is what the
    // contract requires while the result does not *depend* on it — hardening #7.
    fold_areas(
        jigc_root,
        repo_root,
        milestone_id,
        &list.enumerate(),
        schemas,
        committed,
    )
}

/// The result of [`materialize`] — the parent staging area whose `docs/` now holds every
/// merged, suffix-resolved doc body, the staging form `finalize`'s promote sweep reads
/// (`<area>/docs/<type>:<slug>.md`). `docs_dir` is the absolute path of that `docs/`
/// folder (`<jigc_root>/milestones/<id>/merged/docs/`); `addresses` are the materialized
/// final addresses in id-sorted order (the overlay keys), the audit trail of what landed.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MaterializeOutcome {
    /// The parent staging `docs/` folder the merged bodies were written into.
    pub docs_dir: PathBuf,
    /// The materialized final `<type>:<slug>` addresses, id-sorted (the overlay keys).
    pub addresses: Vec<String>,
    /// Each materialized final address → the sub-task id whose area contributed it
    /// (the overlay's `source_task`, carried through the suffix resolution) — the
    /// per-sub-task contribution facts the finalize landing manifest names (C2: the
    /// no-work sub-task becomes visible instead of silently credited). Id-sorted by
    /// construction ([`BTreeMap`](std::collections::BTreeMap)).
    pub sources: std::collections::BTreeMap<String, String>,
}

/// The parent staging-area subfolder a milestone's merged bodies materialize under,
/// relative to the milestone area: `<jigc_root>/milestones/<id>/merged/`. Its `docs/`
/// holds the suffix-resolved bodies in the same `<type>:<slug>.md` staging form a single
/// task's working area uses, so `finalize`'s promote sweep reads it unchanged.
const MERGED_AREA: &str = "merged";

/// **Materialize the join's suffix-rewritten doc bodies into the parent staging area**
/// (`design/storage.md` → The by-task-id join (M7): "the parent working overlay, already
/// suffix-resolved … finalize then commits the overlay"; `DECISIONS.md` 2026-06-04 → the
/// inc-4 fork resolution). Runs [`join`]; **blocks** if its [`JoinOutcome::findings`] holds
/// any blocking finding (same-doc clash / cross-area / isolation) — surfacing the **first**
/// such finding and writing **nothing**. Otherwise, iterating the **id-sorted** overlay,
/// re-produces each entry's final body bytes (the net-new byte production [`resolved_body`]
/// owns — a bare/lone instance's body is its sub-area body unchanged; a suffixed instance's
/// body is its sub-area body with its own self-references rewritten to the suffixed id) and
/// writes it to `<jigc_root>/milestones/<id>/merged/docs/<final-address>.md`, the staging
/// form `finalize`'s promote sweep already reads.
///
/// The written bytes are a **pure function of the area set**: the overlay is id-keyed and
/// each body is `resolved_body`'s deterministic output, so no enumeration / completion /
/// `read_dir` order can reach the materialized bytes (the order-invariance M7 proves). The
/// `merged/docs/` folder is **truncated** before writing (a re-materialize is a clean
/// rebuild, never a stale-body accretion).
pub fn materialize(
    jigc_root: &Path,
    repo_root: &Path,
    milestone_id: &str,
    schemas: &std::collections::BTreeMap<String, crate::schema::Schema>,
    committed: &crate::index::EdgeIndex,
) -> Result<MaterializeOutcome, Finding> {
    // Run the join. A blocking finding is surfaced before anything is materialized
    // (the `dispatch_join` precedent: the merge ran, then routed the contention).
    let outcome = join(jigc_root, repo_root, milestone_id, schemas, committed)?;
    if let Some(blocking) = outcome
        .findings
        .iter()
        .find(|f| f.severity == Severity::Blocking)
    {
        return Err(blocking.clone());
    }

    // Re-gather the same staged groups from the same areas (the shared gather, no
    // divergent re-walk) and re-produce each entry's final body via `resolved_body`.
    let dir = milestone_dir(jigc_root, milestone_id);
    let list =
        read_task_list(&dir).map_err(|err| io_finding(milestone_id, "read the task list", &err))?;
    let (groups, _findings) = gather_groups(
        jigc_root,
        repo_root,
        milestone_id,
        &list.enumerate(),
        schemas,
        committed,
    )?;

    // The parent staging docs/ — a clean rebuild on each materialize.
    let docs_dir = dir.join(MERGED_AREA).join(crate::state::DOCS_DIR);
    if docs_dir.exists() {
        std::fs::remove_dir_all(&docs_dir)
            .map_err(|err| io_finding(milestone_id, "clear the parent staging area", &err))?;
    }
    std::fs::create_dir_all(&docs_dir)
        .map_err(|err| io_finding(milestone_id, "open the parent staging area", &err))?;

    let mut addresses = Vec::new();
    let mut sources = std::collections::BTreeMap::new();
    for (_address, mut staged) in groups {
        // Resolve strictly by task id, the same order `fold_areas` resolves a group in,
        // so the materialized body for a given final address is order-invariant.
        staged.sort_by(|a, b| a.source_task.cmp(&b.source_task));
        for (nth, d) in staged.into_iter().enumerate() {
            let (final_address, body) = resolved_body(milestone_id, &d, nth + 1, schemas)?;
            let (ty, slug) = final_address
                .split_once(':')
                .unwrap_or((final_address.as_str(), ""));
            let path = crate::state::instance_path(dir.join(MERGED_AREA).as_path(), ty, slug);
            std::fs::write(&path, &body)
                .map_err(|err| io_finding(milestone_id, "write a materialized doc body", &err))?;
            sources.insert(final_address.clone(), d.source_task.clone());
            addresses.push(final_address);
        }
    }
    // Id-sorted addresses (the overlay-key order) — the audit trail of what landed.
    addresses.sort();

    Ok(MaterializeOutcome {
        docs_dir,
        addresses,
        sources,
    })
}

/// One staged doc gathered from a sub-area before the clash/suffix rules decide its
/// fate — its minted `address` (`<type>:<slug>`), contributing `source_task`,
/// recorded `provenance`, the area's forward `edges` for this `from`, and the
/// `sub_dir` (so a suffixed instance's body can be re-read for the self-ref rewrite).
/// Gathered in **task-id order** so the deterministic suffix is assigned in that order.
struct StagedDoc {
    address: String,
    source_task: String,
    provenance: crate::state::Provenance,
    edges: Vec<crate::index::Edge>,
    sub_dir: PathBuf,
}

/// Gather every staged doc across the named sub-task areas — in the **given** order —
/// grouped by minted address, alongside the per-sub-area cross-area / isolation findings
/// (`design/storage.md` → The by-task-id join, steps 1–2 + step 5). The gather order *is*
/// the input order, so each group's instances are already in that order; the group
/// resolution then sorts every group strictly by task id, so the result is a pure function
/// of the *set* of areas (hardening #7). Shared by [`fold_areas`] (which resolves groups to
/// edges) and [`materialize`] (which resolves them to bodies), so the two paths gather the
/// **same** staged docs from the **same** areas with no divergent re-walk. **Unknown
/// provenance** for a staged body → the blocking [`missing_provenance_finding`] (a real
/// fault, the bit is written beside every body).
/// The `<type>:<slug>` addresses **physically staged** in a sub-area's `docs/` — every
/// `.md` body present, regardless of whether its doctype resolves or its bytes parse.
/// This is the join's materialization basis: a body physically staged in an area is
/// attributable to it (the same physical-body attribution the isolation check keys on).
/// It is deliberately **decoupled** from the overlay's `task_froms` — which, since M45
/// shadow-by-`from`, records a `from` only after a *successful parse* so an unparseable
/// staged doc cannot shadow the committed edges to empty on the compose path. Keying the
/// merge on `task_froms` would then silently drop an unparseable-but-staged body from the
/// commit; keying on the physical body preserves it (the join copies bodies, never parses
/// them). Sorted + deduped for a deterministic gather (`DECISIONS.md` 2026-07-23 → the
/// Settle, Decision 3, clause d).
fn staged_bodies(sub_dir: &Path) -> Vec<String> {
    let docs = sub_dir.join(crate::state::DOCS_DIR);
    let mut addrs: Vec<String> = match std::fs::read_dir(&docs) {
        Ok(entries) => entries
            .flatten()
            .map(|e| e.path())
            .filter(|p| p.extension().and_then(|x| x.to_str()) == Some("md"))
            .filter_map(|p| p.file_stem().and_then(|s| s.to_str()).map(str::to_string))
            .collect(),
        Err(_) => Vec::new(), // no docs/ yet: nothing staged.
    };
    addrs.sort();
    addrs.dedup();
    addrs
}

#[allow(clippy::type_complexity)]
fn gather_groups(
    jigc_root: &Path,
    repo_root: &Path,
    milestone_id: &str,
    sub_ids: &[String],
    schemas: &std::collections::BTreeMap<String, crate::schema::Schema>,
    committed: &crate::index::EdgeIndex,
) -> Result<
    (
        std::collections::BTreeMap<String, Vec<StagedDoc>>,
        Vec<Finding>,
    ),
    Finding,
> {
    let mut findings = Vec::new();
    let mut groups: std::collections::BTreeMap<String, Vec<StagedDoc>> =
        std::collections::BTreeMap::new();
    for sub_id in sub_ids {
        let sub_dir = jigc_root.join("tasks").join(sub_id);

        // The per-area basis: the single-area working-overlay derivation, reused
        // unchanged — its `task_edges` are the staged docs' forward edges, keyed by
        // `from`; its `task_froms` are this area's staged doc addresses.
        let area = crate::index::overlay_working(committed, &sub_dir, schemas);
        let provenance = crate::state::ProvenanceRecord::load(&sub_dir)
            .map_err(|err| io_finding(milestone_id, "read a sub-task provenance manifest", &err))?;

        // Step 5 — **cross-area refs, checked per sub-area** (`design/storage.md` → The
        // by-task-id join, step 5; `design/validation.md` → Fan-out cross-area refs). The
        // existing single-area forward-ref walk is reused **unchanged**, invoked once per
        // sub-area with *that area's own* `sub_dir`, so each outgoing edge resolves only
        // against `committed ∪ this one area` — never a merged multi-area overlay. A ref
        // resolving only inside a sibling area is unreachable here (its target is neither
        // committed nor in *this* `docs/`), so it surfaces as the already-floored,
        // intrinsic `schema-conformance.ref-resolves` block. No flattened union is ever
        // built, so a cross-area ref can never resolve clean by mere path-existence.
        findings.extend(crate::index::ref_resolves(
            &area, repo_root, &sub_dir, schemas,
        ));

        // Join-time isolation (`design/storage.md` → isolation is structural +
        // join-checked in M7; the write-time `--task` barrier is M8). A doc the area's
        // provenance manifest **attributes to itself** but whose body is not physically
        // staged in this area's `docs/` is not attributable to its own sub-area → a
        // blocking, route-bearing finding. Attribution keys on the **physical body**
        // (`docs/<addr>.md` exists), *not* on the schema-gated `task_froms`: a body whose
        // doctype is absent from the resolved schema set is skipped by `overlay_working`
        // (never reaches `task_froms`) yet is genuinely attributable to its own area, so
        // keying on `task_froms` would over-block it. Emitted directly with no
        // `knobs.yaml` row (the `reconciliation.*` blocking-but-untunable precedent).
        for claimed in provenance.docs.keys() {
            let body = sub_dir.join("docs").join(format!("{claimed}.md"));
            if !body.exists() {
                findings.push(isolation_finding(milestone_id, sub_id, claimed));
            }
        }

        // Group on the **physical bodies** staged in this area, not the schema-gated,
        // parse-gated `task_froms` (M45 shadow-by-`from` decoupled them — see
        // [`staged_bodies`]): a body physically staged here must materialize even if it
        // failed to parse, else the after-parse `task_froms` move would silently drop it
        // from the commit. Edges come from `area.task_edges` (empty for an unparseable or
        // unknown-type body — the join never re-parses, it copies bytes).
        for from in staged_bodies(&sub_dir) {
            let edges: Vec<crate::index::Edge> = area
                .task_edges
                .iter()
                .filter(|e| e.from == from)
                .cloned()
                .collect();
            // A staged doc with no recorded provenance is a real fault (the bit is
            // written at stage time beside every body); defaulting to a provenance is
            // wrong, so surface the absence as a blocking finding routed to re-stage.
            let Some(prov) = provenance.get(&from) else {
                return Err(missing_provenance_finding(milestone_id, sub_id, &from));
            };
            groups.entry(from.clone()).or_default().push(StagedDoc {
                address: from.clone(),
                source_task: sub_id.clone(),
                provenance: prov,
                edges,
                sub_dir: sub_dir.clone(),
            });
        }
    }
    Ok((groups, findings))
}

/// Fold the named sub-task areas — in the **given** order — into the merged overlay,
/// applying the provenance clash rule (block) vs the collision-suffix for distinct
/// `created` instances (`design/storage.md` → The by-task-id join, steps 2–4). The
/// only output accumulator is the address-keyed `BTreeMap`, and every collision group
/// is resolved by **task-id order** (the gather order), so the result is a pure
/// function of the *set* of `sub_ids`, independent of their iteration order (the
/// order-invariance the join's contract rests on; the public [`join`] always feeds the
/// id-sorted order).
fn fold_areas(
    jigc_root: &Path,
    repo_root: &Path,
    milestone_id: &str,
    sub_ids: &[String],
    schemas: &std::collections::BTreeMap<String, crate::schema::Schema>,
    committed: &crate::index::EdgeIndex,
) -> Result<JoinOutcome, Finding> {
    let (groups, mut findings) = gather_groups(
        jigc_root,
        repo_root,
        milestone_id,
        sub_ids,
        schemas,
        committed,
    )?;

    // Resolve each address group: a lone staged doc is a disjoint-union insert; a
    // collision is either a blocking same-doc clash or a deterministic suffix. Resolution
    // produces each group's **final** addresses (bare or suffixed) into `assignments`
    // *before* anything lands in the overlay, so a final address that two different groups
    // both claim (a suffixed `X-2` vs a pre-existing separate group already named `X-2`)
    // is detected and blocked rather than silently overwritten (the cross-group guard).
    let mut overlay: std::collections::BTreeMap<String, MergedDoc> =
        std::collections::BTreeMap::new();
    // Each resolved final address → its (group minted address, contributing source task,
    // merged doc). `BTreeMap`-keyed so the contributor listing for a collision is
    // id-sorted, independent of group iteration order (hardening #7).
    let mut assignments: std::collections::BTreeMap<String, (String, String, MergedDoc)> =
        std::collections::BTreeMap::new();
    // Final addresses claimed by ≥2 distinct groups — the cross-group collisions, each
    // recorded with every contending `(minted address, source task)` in id order.
    let mut collisions: std::collections::BTreeMap<String, Vec<(String, String)>> =
        std::collections::BTreeMap::new();
    let mut claim = |final_address: String, minted: &str, merged: MergedDoc| {
        match assignments.entry(final_address.clone()) {
            std::collections::btree_map::Entry::Vacant(v) => {
                v.insert((minted.to_string(), merged.source_task.clone(), merged));
            }
            std::collections::btree_map::Entry::Occupied(o) => {
                // A second claim on the same final address from a different group: this
                // is a cross-group collision, not a within-group suffix. Record both
                // contenders and keep the colliding address out of the overlay.
                let (prev_minted, prev_task, _) = o.get();
                let contenders = collisions
                    .entry(final_address)
                    .or_insert_with(|| vec![(prev_minted.clone(), prev_task.clone())]);
                contenders.push((minted.to_string(), merged.source_task));
            }
        }
    };
    for (address, mut staged) in groups {
        // Resolve every group strictly by **task id** — the suffix order and the clash
        // listing must not depend on the order `sub_ids` was fed in (the order-invariance
        // the join's contract rests on; the public [`join`] feeds id-sorted order, but a
        // permutation must still yield byte-identical output — hardening #7).
        staged.sort_by(|a, b| a.source_task.cmp(&b.source_task));
        if staged.len() == 1 {
            let d = staged.into_iter().next().expect("len == 1");
            claim(
                d.address.clone(),
                &address,
                MergedDoc {
                    provenance: d.provenance,
                    source_task: d.source_task,
                    edges: d.edges,
                },
            );
            continue;
        }
        // Step 3: any `edited-from-base` in a colliding group is a partition violation
        // — two edits to one committed-at-base slug, or the mixed (one created, one
        // edited) case. Blocking, never a blind merge; the clashing slug is kept out.
        if staged
            .iter()
            .any(|d| d.provenance == crate::state::Provenance::EditedFromBase)
        {
            findings.push(same_doc_clash_finding(&address, &staged));
            continue;
        }
        // Step 4: all `created` → distinct docs. The first (lowest task id) keeps the
        // bare slug; each later one takes the deterministic suffix, with its intra-doc
        // self-references rewritten to the suffixed id in lockstep.
        for (nth, d) in staged.into_iter().enumerate() {
            let merged = suffix_resolve(milestone_id, &d, nth + 1, schemas)?;
            claim(merged.0, &address, merged.1);
        }
    }

    // Land every uniquely-claimed final address; a final address claimed by ≥2 groups was
    // pulled from `assignments` into `collisions` and is kept out of the overlay, emitting
    // the same blocking, route-bearing clash the within-group rule raises.
    for (final_address, (_minted, _task, merged)) in assignments {
        if collisions.contains_key(&final_address) {
            continue;
        }
        overlay.insert(final_address, merged);
    }
    for (final_address, contenders) in collisions {
        findings.push(cross_group_collision_finding(&final_address, &contenders));
    }

    // The cross-area / isolation findings were gathered in the *input* sub-area order
    // (the clash findings already iterate the address-keyed `BTreeMap`, so they are
    // id-sorted); sort the whole list by `(code, message)` so the surfaced findings are
    // a pure function of the *set* of sub-areas, never their iteration order — the
    // order-invariance the join's contract rests on (hardening #7). The message carries
    // every finding's distinguishing identity (addresses, contending task ids), so this
    // total order is byte-stable across any input permutation.
    findings.sort_by(|a, b| (&a.code, &a.message).cmp(&(&b.code, &b.message)));

    Ok(JoinOutcome {
        overlay,
        findings: findings.into(),
    })
}

/// Resolve one `created` instance of a colliding slug at position `nth` (1-based, in
/// task-id order): apply the deterministic suffix to its address and, when suffixed,
/// **rewrite its own self-references** to the suffixed id via the `set_field`/splice
/// path (`design/storage.md` → step 4; `structural-grammar.md` → deterministic suffix
/// in task-id merge order), re-deriving its edges from the rewritten body so the
/// overlay and the bytes `finalize` will write agree. Returns the suffixed
/// `(address, MergedDoc)`. The first instance (`nth == 1`) keeps its bare slug and
/// edges unchanged.
fn suffix_resolve(
    milestone_id: &str,
    d: &StagedDoc,
    nth: usize,
    schemas: &std::collections::BTreeMap<String, crate::schema::Schema>,
) -> Result<(String, MergedDoc), Finding> {
    if nth <= 1 {
        return Ok((
            d.address.clone(),
            MergedDoc {
                provenance: d.provenance,
                source_task: d.source_task.clone(),
                edges: d.edges.clone(),
            },
        ));
    }

    // The suffixed address: `<type>:<slug>-<nth>`.
    let (ty, slug) = d
        .address
        .split_once(':')
        .unwrap_or((d.address.as_str(), ""));
    let new_address = format!("{ty}:{}", crate::slug::suffixed(slug, nth));

    // The self-references to rewrite: edges whose `to` is the doc's own (old) address.
    let self_refs: Vec<&crate::index::Edge> =
        d.edges.iter().filter(|e| e.to == d.address).collect();

    // No self-ref → only the identity changes; re-key the edges' `from` to the suffix.
    if self_refs.is_empty() {
        let edges = d
            .edges
            .iter()
            .map(|e| crate::index::Edge {
                from: new_address.clone(),
                ..e.clone()
            })
            .collect();
        return Ok((
            new_address.clone(),
            MergedDoc {
                provenance: d.provenance,
                source_task: d.source_task.clone(),
                edges,
            },
        ));
    }

    // Rewrite the self-ref field value(s) in the body via the set_field/splice path,
    // then re-derive the edges from the rewritten bytes so the overlay edges are the
    // bytes' truth (never hand-patched into divergence). The body production is the
    // shared [`resolved_body`] (the same bytes [`materialize`] writes), so the overlay
    // edges and the materialized bytes can never diverge.
    let (resolved_address, body) = resolved_body(milestone_id, d, nth, schemas)?;
    let schema = schemas
        .get(ty)
        .ok_or_else(|| missing_schema_finding(milestone_id, &d.source_task, &d.address))?;
    let edges = crate::index::edges_from_source(schema, &body, &resolved_address);
    Ok((
        resolved_address,
        MergedDoc {
            provenance: d.provenance,
            source_task: d.source_task.clone(),
            edges,
        },
    ))
}

/// Produce the **final body bytes** of one staged doc at position `nth` (1-based, in
/// task-id order) and its **final address** — the net-new byte production the
/// materialize step writes and [`suffix_resolve`] derives its edges from (so the
/// overlay edges and the committed bytes are always the same truth;
/// `DECISIONS.md` 2026-06-04 → the inc-4 fork resolution: the join keeps only edges,
/// materialize re-reads the body + re-applies the rewrite). The sub-area body is read
/// fresh and:
/// - `nth <= 1` (the first / lone instance) → the body is returned **byte-unchanged**
///   at its bare address;
/// - a suffixed instance with **no** self-reference → the body is unchanged (only its
///   identity / filename changes) at the `<type>:<slug>-<nth>` address;
/// - a suffixed instance **with** self-references → each self-ref field value is
///   rewritten to the suffixed address via the `set_field`/splice path, in lockstep with
///   the slug suffix, so the renamed doc never dangles or points at its sibling.
fn resolved_body(
    milestone_id: &str,
    d: &StagedDoc,
    nth: usize,
    schemas: &std::collections::BTreeMap<String, crate::schema::Schema>,
) -> Result<(String, String), Finding> {
    let (ty, slug) = d
        .address
        .split_once(':')
        .unwrap_or((d.address.as_str(), ""));
    let path = crate::state::instance_path(&d.sub_dir, ty, slug);
    let source = std::fs::read_to_string(&path)
        .map_err(|err| io_finding(milestone_id, "read a staged doc body", &err))?;

    // The first / lone instance keeps its bare address and byte-unchanged body.
    if nth <= 1 {
        return Ok((d.address.clone(), source));
    }

    let new_address = format!("{ty}:{}", crate::slug::suffixed(slug, nth));

    // The self-references to rewrite: edges whose `to` is the doc's own (old) address.
    let self_refs: Vec<&crate::index::Edge> =
        d.edges.iter().filter(|e| e.to == d.address).collect();
    if self_refs.is_empty() {
        // Only the identity changes — the body bytes are unchanged.
        return Ok((new_address, source));
    }

    let schema = schemas
        .get(ty)
        .ok_or_else(|| missing_schema_finding(milestone_id, &d.source_task, &d.address))?;
    let mut body = source;
    for edge in &self_refs {
        let Some(section_id) = ref_section(schema, &edge.relation) else {
            continue; // the relation is not a known schema ref field: nothing to splice.
        };
        body = crate::write::set_field(schema, &body, &section_id, &edge.relation, &new_address)
            .map_err(|err| splice_finding(milestone_id, &d.address, &edge.relation, err))?;
    }
    Ok((new_address, body))
}

/// The schema section id that declares the `ref` field `relation`, if any — used to
/// target [`crate::write::set_field`] when rewriting a suffixed instance's self-ref.
fn ref_section(schema: &crate::schema::Schema, relation: &str) -> Option<String> {
    schema.sections.iter().find_map(|s| {
        let crate::schema::SectionBody::Simple { fields, .. } = &s.body else {
            return None;
        };
        fields
            .iter()
            .any(|f| f.ty == crate::schema::FieldType::Ref && f.id == relation)
            .then(|| s.id.clone())
    })
}

/// The same-doc clash block (`design/storage.md` → The by-task-id join, step 3): two
/// sub-areas writing the same committed-at-base slug (two `edited-from-base`, or the
/// mixed `created` + `edited-from-base` case) is a partition violation — blocking,
/// routed to a human, never section-merged or last-writer-win. Names the contending
/// sub-tasks so the human can act. Emits `Severity::Blocking` directly with a route and
/// **no** knobs/inventory row (the verified `reconciliation.*`-shaped intrinsic-block
/// precedent; `DECISIONS.md` 2026-06-04 → M7 Increment 3 planning).
fn same_doc_clash_finding(address: &str, staged: &[StagedDoc]) -> Finding {
    let tasks: Vec<&str> = staged.iter().map(|d| d.source_task.as_str()).collect();
    Finding::graded(
        Severity::Blocking,
        "join.same-doc-clash",
        format!(
            "same-doc clash — sub-tasks [{}] each write `{address}` at the milestone base; \
             the join never blind-merges a shared managed doc",
            tasks.join(", ")
        ),
        Some(Location::addressed(address, 1, 1)),
        Some(
            "have the contending sub-tasks edit distinct docs, or merge their intent by hand"
                .into(),
        ),
    )
}

/// A blocking, route-bearing `join.same-doc-clash` for a **cross-group** final-address
/// collision: two *different* address groups resolve to the same final address (a
/// suffixed `X-2` and a separate group already named `X-2`). Without this the later
/// claim would silently overwrite the earlier in the overlay (and the materialize
/// `fs::write`) — a deterministic but lossy drop of a sub-task's doc with no finding.
/// `contenders` are the `(minted address, source task)` pairs in id order, so the
/// message is byte-stable across any group iteration order (hardening #7).
fn cross_group_collision_finding(final_address: &str, contenders: &[(String, String)]) -> Finding {
    let listing: Vec<String> = contenders
        .iter()
        .map(|(minted, task)| format!("`{minted}` (sub-task `{task}`)"))
        .collect();
    Finding::graded(
        Severity::Blocking,
        "join.same-doc-clash",
        format!(
            "same-doc clash — {} resolve to the same final address `{final_address}`; \
             the join never silently overwrites a managed doc",
            listing.join(", ")
        ),
        Some(Location::addressed(final_address, 1, 1)),
        Some(
            "have the contending sub-tasks write distinct docs, or rename one so the \
             suffixed and pre-existing addresses no longer collide"
                .into(),
        ),
    )
}

/// A blocking finding for a colliding `created` instance whose type has no schema in the
/// resolved set — the self-ref rewrite cannot run without it (should not arise: the
/// overlay derivation only stages known types, but defaulting is wrong).
fn missing_schema_finding(milestone_id: &str, sub_id: &str, address: &str) -> Finding {
    Finding::graded(
        Severity::Blocking,
        "join.unknown-type",
        format!(
            "colliding staged doc `{address}` in sub-task `{sub_id}` of milestone `{milestone_id}` has an unknown doctype"
        ),
        Some(Location::addressed(address, 1, 1)),
        Some("re-stage the doc under a known doctype".into()),
    )
}

/// A blocking finding for a self-ref splice failure during suffix resolution — the
/// colliding body did not conform or the located field vanished (a real fault, since the
/// edge was derived from this very body).
fn splice_finding(
    milestone_id: &str,
    address: &str,
    relation: &str,
    err: crate::write::SpliceError,
) -> Finding {
    Finding::graded(
        Severity::Blocking,
        "join.self-ref-rewrite",
        format!(
            "could not rewrite the self-reference `{relation}` of colliding doc `{address}` in milestone `{milestone_id}`: {err}"
        ),
        Some(Location::addressed(address, 1, 1)),
        Some("re-stage the colliding doc so its self-reference is well-formed".into()),
    )
}

/// A staged doc whose `docs/` area carries **no** provenance entry for it — a real
/// fault, since every staging primitive records the bit beside the body
/// (`design/storage.md` → classification by provenance). Blocking, routed to re-stage;
/// never defaulted to a provenance the clash rule would then mis-decide on.
fn missing_provenance_finding(milestone_id: &str, sub_id: &str, address: &str) -> Finding {
    Finding::graded(
        Severity::Blocking,
        "join.missing-provenance",
        format!(
            "staged doc `{address}` in sub-task `{sub_id}` of milestone `{milestone_id}` has no recorded provenance"
        ),
        Some(Location::addressed(address, 1, 1)),
        Some("re-stage the doc so its provenance is recorded".into()),
    )
}

/// The join-time isolation block (`design/storage.md` → The by-task-id join: isolation
/// is structural + join-checked in M7; the write-time `--task` barrier is M8). A
/// sub-area's provenance manifest attributes `address` to itself, yet no body for it is
/// physically staged in that area's `docs/` — the doc is **not attributable to its own
/// sub-area**, a violation of the locked "each sub-agent writes only to its own area"
/// invariant. Blocking, routed to re-stage; emitted directly with **no** `knobs.yaml`
/// row (the `reconciliation.*` blocking-but-untunable precedent;
/// `DECISIONS.md` 2026-06-04 → M7 Increment 3 planning).
fn isolation_finding(milestone_id: &str, sub_id: &str, address: &str) -> Finding {
    Finding::graded(
        Severity::Blocking,
        "join.area-isolation",
        format!(
            "isolation — sub-task `{sub_id}` of milestone `{milestone_id}` attributes `{address}` \
             to itself but stages no such doc in its own area; a staged doc must belong to its \
             own sub-area"
        ),
        Some(Location::addressed(address, 1, 1)),
        Some("re-stage the doc inside its own sub-task area, or drop the stray attribution".into()),
    )
}

/// Slug the title into the milestone id, applying the empty → type-name
/// (`milestone`) fallback — the same discipline as `state::mint_id`.
///
/// `pub` for the CLI's pre-mint id-is-taken guard, which must resolve the record home of the
/// **exact id this mint will produce** — deriving the slug a second way would guard a different
/// path than the mint writes (`design/team-ready-state.md` → The lifecycle).
pub fn mint_id(title: &str) -> String {
    let slug = crate::slug::slugify(title);
    if slug.is_empty() {
        crate::slug::slugify("milestone")
    } else {
        slug
    }
}

/// Render the base-pin file body — the frozen on-disk form (golden-locked,
/// identical to `state.rs`'s `base.json`).
fn render_base_pin(base: &BasePin) -> String {
    let mut s = serde_json::to_string_pretty(base).expect("BasePin serializes");
    s.push('\n');
    s
}

/// The serial-collision block: a blocking finding naming the existing milestone
/// and routing the agent to add tasks to it or discard it (`write-commands.md` →
/// Minting a milestone; the mint rejects, never suffixes at this site).
fn collision_finding(id: &str) -> Finding {
    Finding::graded(
        Severity::Blocking,
        "milestone.serial-collision",
        format!("milestone `{id}` already exists"),
        Some(Location::addressed(format!("milestone:{id}"), 1, 1)),
        Some(format!(
            "add tasks with `jigc milestone add-task {id} \"<intent>\"` or pick a different title"
        ).into()),
    )
}

/// A blocking finding for an area I/O failure during minting.
fn io_finding(id: &str, doing: &str, err: &std::io::Error) -> Finding {
    Finding::graded(
        Severity::Blocking,
        "milestone.area-io",
        format!("could not {doing} for milestone `{id}`: {err}"),
        Some(Location::addressed(format!("milestone:{id}"), 1, 1)),
        Some(
            "resolve the underlying I/O condition (a disk or permissions problem on the \
             `.jigc/` milestone area), then re-run the command"
                .into(),
        ),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    /// The route-floor seam-sweep, exercised through the real `milestone.area-io` producer
    /// (M43 surface census): the mint I/O fault carries a recovery route and drives cleanly
    /// through the [`Findings`] serialization seam — the traffic whose absence let it ship
    /// route-less (`DECISIONS.md` 2026-07-17 → the seam-sweep rule).
    #[test]
    fn area_io_finding_carries_a_recovery_route_through_the_seam() {
        let err = std::io::Error::new(std::io::ErrorKind::PermissionDenied, "denied");
        let finding = io_finding("cache-rework", "open the milestone area", &err);
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
        serde_json::to_string(&Findings::from(vec![finding])).expect("serializes");
    }

    /// A throwaway directory that removes itself on drop — keeps mint tests off
    /// any real `.jigc/` tree (mirrors `state.rs`'s `TempRoot`).
    struct TempRoot(PathBuf);

    impl TempRoot {
        fn new(tag: &str) -> Self {
            let mut path = std::env::temp_dir();
            let unique = format!(
                "jigc-milestone-{tag}-{}-{:?}",
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

    /// The done-criterion: minting "Cache rework" opens
    /// `.jigc/milestones/cache-rework/` with a single shared base-pin recording
    /// the supplied HEAD and an **empty** task list that reads back empty; a
    /// second mint of the same slug rejects with a route-bearing blocking
    /// finding naming the existing milestone and creates nothing new (golden
    /// over the frozen on-disk bytes).
    #[test]
    fn mint_creates_milestone_area_shared_base_and_empty_task_list() {
        let root = TempRoot::new("create");
        let base = BasePin::new("0123456789abcdef0123456789abcdef01234567", "0123456");

        let minted =
            mint_milestone(root.path(), "Cache rework", base.clone()).expect("first mint succeeds");

        assert_eq!(
            minted.id, "cache-rework",
            "address is `milestone:cache-rework`"
        );
        let dir = root.path().join("milestones").join("cache-rework");
        assert_eq!(minted.dir, dir);
        assert!(dir.is_dir(), "milestone area must exist");
        assert_eq!(minted.base, base);

        // Golden over the frozen single-shared-base byte form for a fixed SHA.
        let pin = std::fs::read_to_string(dir.join(BASE_PIN_FILE)).expect("base pin written");
        insta::assert_snapshot!("milestone_base_pin", pin);
        // The pin round-trips back to the supplied base.
        let back: BasePin = serde_json::from_str(&pin).expect("pin parses");
        assert_eq!(back, base);
        assert_eq!(
            read_base_pin(&dir).expect("read base pin"),
            base,
            "the shared base reads back through the reader"
        );

        // Golden over the frozen empty-task-list byte form.
        let tasks = std::fs::read_to_string(dir.join(TASKS_FILE)).expect("task list written");
        insta::assert_snapshot!("milestone_empty_task_list", tasks);
        // The task list reads back empty.
        assert_eq!(
            read_task_list(&dir).expect("read task list").tasks,
            Vec::<String>::new(),
            "a freshly minted milestone has an empty task list"
        );

        // Second mint of the same slug → serial reject, nothing new created.
        let before = std::fs::read_dir(root.path().join("milestones"))
            .expect("milestones dir")
            .count();
        let err = mint_milestone(root.path(), "Cache rework", base.clone())
            .expect_err("re-mint of the same slug rejects");
        assert_eq!(err.severity, Severity::Blocking);
        assert_eq!(err.code, "milestone.serial-collision");
        assert!(
            err.message.contains("cache-rework"),
            "block must name the existing milestone: {err:?}"
        );
        assert!(err.route.is_some(), "serial collision carries a route");
        let after = std::fs::read_dir(root.path().join("milestones"))
            .expect("milestones dir")
            .count();
        assert_eq!(before, after, "no second dir created on collision");
    }

    /// The done-criterion for T2 (`design/write-commands.md` → `jigc milestone
    /// add-task`; `design/storage.md` → The by-task-id join): two distinct
    /// sub-tasks each open an **isolated** `tasks/<sub>/` area whose `base.json`
    /// equals the **milestone's** shared base (not a fresh HEAD), and both ids
    /// land in the milestone's task list. A third add whose intent slugs to an
    /// already-listed id returns a routed blocking collision and appends nothing;
    /// an add against an unknown milestone rejects.
    #[test]
    fn add_task_mints_isolated_sub_tasks_on_the_shared_base_and_rejects_collision() {
        let root = TempRoot::new("add-task");
        let milestone_base = BasePin::new("1111111111111111111111111111111111111111", "1111111");

        let milestone = mint_milestone(root.path(), "Cache rework", milestone_base.clone())
            .expect("milestone mints");

        // First sub-task.
        let a = add_task(
            root.path(),
            &milestone.id,
            "Add rate limiter",
            "single-task",
        )
        .expect("first sub-task adds");
        assert_eq!(a.milestone_id, "cache-rework");
        assert_eq!(a.task.id, "add-rate-limiter");
        // Isolated area in the shared tasks/ namespace.
        let a_dir = root.path().join("tasks").join("add-rate-limiter");
        assert_eq!(a.task.dir, a_dir);
        assert!(a_dir.is_dir(), "sub-task opens its own isolated area");
        // Its base IS the milestone's shared base, not a fresh HEAD.
        assert_eq!(
            a.task.base, milestone_base,
            "sub-task inherits the shared base"
        );
        assert_eq!(
            crate::state::read_base_pin(&a_dir).expect("read sub base"),
            milestone_base,
            "the sub-task's base.json equals the milestone's base on disk"
        );

        // Second, distinct sub-task — its own isolated area, same shared base.
        let b = add_task(
            root.path(),
            &milestone.id,
            "Evict stale keys",
            "single-task",
        )
        .expect("second sub-task adds");
        assert_eq!(b.task.id, "evict-stale-keys");
        let b_dir = root.path().join("tasks").join("evict-stale-keys");
        assert!(
            b_dir.is_dir() && b_dir != a_dir,
            "the two areas are distinct"
        );
        assert_eq!(
            crate::state::read_base_pin(&b_dir).expect("read sub base"),
            milestone_base,
            "the second sub-task also inherits the milestone's shared base"
        );

        // Both ids are in the milestone's task list.
        let list = read_task_list(&milestone.dir).expect("read list");
        assert_eq!(
            list.tasks,
            vec![
                "add-rate-limiter".to_string(),
                "evict-stale-keys".to_string()
            ],
            "both sub-task ids land in the milestone's task list"
        );

        // A third add whose intent slugs to an already-listed id → routed block,
        // appends nothing, mints nothing new.
        let tasks_before = std::fs::read_dir(root.path().join("tasks"))
            .expect("tasks dir")
            .count();
        let collide = add_task(
            root.path(),
            &milestone.id,
            "Add rate limiter",
            "single-task",
        )
        .expect_err("a within-milestone slug collision rejects");
        assert_eq!(collide.severity, Severity::Blocking);
        assert_eq!(collide.code, "milestone.sub-task-collision");
        assert!(
            collide.message.contains("add-rate-limiter")
                && collide.message.contains("cache-rework"),
            "the block names the colliding sub-task and the milestone: {collide:?}"
        );
        assert!(collide.route.is_some(), "the collision carries a route");
        // Nothing appended: the list is unchanged.
        assert_eq!(
            read_task_list(&milestone.dir).expect("read list").tasks,
            list.tasks,
            "a collision appends nothing to the task list"
        );
        let tasks_after = std::fs::read_dir(root.path().join("tasks"))
            .expect("tasks dir")
            .count();
        assert_eq!(tasks_before, tasks_after, "a collision mints no new area");

        // Add against an unknown milestone → reject.
        let unknown = add_task(root.path(), "no-such-milestone", "Whatever", "single-task")
            .expect_err("an unknown milestone rejects");
        assert_eq!(unknown.severity, Severity::Blocking);
        assert_eq!(unknown.code, "milestone.unknown");
        assert!(
            unknown.route.is_some(),
            "the unknown-milestone block carries a route"
        );
    }

    /// Validation hardening #7 (`increment-workflow.md` → determinism by
    /// re-execution; `design/storage.md` → The by-task-id join): the task list the
    /// join enumerates returns a **canonical id-sorted** order, never insertion or
    /// `read_dir` order. The list is enumerated **twice** from divergent insertion
    /// orders — `[zebra-fix, alpha-fix]` and its **reverse** `[alpha-fix,
    /// zebra-fix]` — and both must yield the same id-sorted `[alpha-fix,
    /// zebra-fix]`; reverse is mandatory, since an id-ordered insertion would make
    /// insertion order trivially equal id order and hide the bug. The fixture is
    /// also built so the milestone's on-disk sub-task areas, listed in raw
    /// `read_dir` order, are not pre-sorted by id, so enumeration cannot be passing
    /// merely because the filesystem happened to hand back id order. The recorded
    /// backing list keeps insertion order (the audit trail); only enumeration is
    /// canonicalized.
    #[test]
    fn enumeration_is_id_sorted_under_divergent_insertion_and_read_dir_orders() {
        // Enumerate the same two sub-task ids under a given insertion order and
        // return both the recorded backing order and the canonical enumeration.
        fn enumerate_under(
            tag: &str,
            insertion: [(&str, &str); 2],
        ) -> (Vec<String>, Vec<String>, Vec<String>) {
            let root = TempRoot::new(tag);
            let base = BasePin::new("2222222222222222222222222222222222222222", "2222222");
            let milestone =
                mint_milestone(root.path(), "Cache rework", base).expect("milestone mints");
            for (intent, _id) in insertion {
                add_task(root.path(), &milestone.id, intent, "single-task")
                    .unwrap_or_else(|e| panic!("{intent} adds: {e:?}"));
            }
            let list = read_task_list(&milestone.dir).expect("read list");
            // Raw read_dir order of the sub-task areas, for the fixture-shape check.
            let read_dir_order: Vec<String> = std::fs::read_dir(root.path().join("tasks"))
                .expect("tasks dir")
                .map(|e| e.expect("entry").file_name().to_string_lossy().into_owned())
                .collect();
            (list.tasks.clone(), list.enumerate(), read_dir_order)
        }

        let id_sorted = vec!["alpha-fix".to_string(), "zebra-fix".to_string()];

        // Order 1: insertion is the reverse of id order.
        let (recorded1, enum1, read_dir1) = enumerate_under(
            "enum-fwd",
            [("Zebra fix", "zebra-fix"), ("Alpha fix", "alpha-fix")],
        );
        assert_eq!(
            recorded1,
            vec!["zebra-fix".to_string(), "alpha-fix".to_string()],
            "the recorded backing list keeps insertion order"
        );
        assert_eq!(
            enum1, id_sorted,
            "enumeration is id-sorted, not insertion order"
        );

        // Order 2: the reverse insertion — mandatory per #7, so an id-ordered
        // insertion cannot make insertion order trivially equal id order.
        let (recorded2, enum2, read_dir2) = enumerate_under(
            "enum-rev",
            [("Alpha fix", "alpha-fix"), ("Zebra fix", "zebra-fix")],
        );
        assert_eq!(
            recorded2,
            vec!["alpha-fix".to_string(), "zebra-fix".to_string()],
            "the reverse insertion is recorded in its own order"
        );
        assert_eq!(
            enum2, id_sorted,
            "the reverse insertion still enumerates id-sorted"
        );

        // Byte-identical enumeration across the two divergent insertion orders.
        assert_eq!(
            enum1, enum2,
            "enumeration is order-invariant across divergent insertion orders"
        );

        // The fixture's on-disk sub-task areas, listed in raw `read_dir` order.
        // Where the host hands them back in a divergent order, a green enumeration
        // demonstrably is not an accident of that order. Where it does not, the
        // fixture *cannot* produce divergence — an order-preserving filesystem
        // (APFS here) sorts every listing — so the demand is unattainable on the
        // host rather than unmet by the fixture, and asserting it would be a
        // machine-dependent red. Either way the load-bearing proof is the
        // divergent-insertion pair above, which is the axis `enumerate()` actually
        // consumes (it sorts the *recorded* list; `read_dir` is not on its path).
        let read_dir_diverges = |raw: &[String]| {
            let mut sorted = raw.to_vec();
            sorted.sort();
            raw != sorted.as_slice()
        };
        if !read_dir_diverges(&read_dir1) && !read_dir_diverges(&read_dir2) {
            assert_eq!(
                read_dir1, id_sorted,
                "neither run diverged, so the host must be order-preserving"
            );
            assert_eq!(
                read_dir2, id_sorted,
                "neither run diverged, so the host must be order-preserving"
            );
        }
    }

    use crate::schema::Schema;
    use std::collections::BTreeMap;

    const SPEC_YAML: &[u8] = include_bytes!("../../cli/pack/schemas/spec.yaml");

    /// The schema map `add_from_spec` resolves the spec address's type against —
    /// the shipped `spec` doctype (its repeatable `criteria` section is the seed
    /// substrate).
    fn schemas() -> BTreeMap<String, Schema> {
        let mut m = BTreeMap::new();
        m.insert(
            "spec".to_string(),
            crate::schema::load_schema_with_types(
                SPEC_YAML,
                &crate::schema::dev_pack_field_types(),
            )
            .expect("spec.yaml loads"),
        );
        m
    }

    /// A committed `spec` with **three** repeatable `criteria` items — the seed
    /// substrate `add-from-spec` enumerates. Human-editable, conformant bytes; the
    /// three titles are deliberately not in id-sorted physical order, so the mint
    /// (physical order) and the milestone's enumeration (id-sorted) are distinct.
    const THREE_CRITERIA_SPEC: &str = "\
# Gateway rate limiting

## Goal

Bound per-client request volume at the gateway.

## Context

Downstream services were each enforcing limits ad hoc.

## Criteria

### Rejects the 101st request  {#rejects-burst}

The gateway rejects the 101st request in a rolling 60s window.

### Admits within the window  {#admits-within}

Requests under the cap are admitted unchanged.

### Recovers after the window  {#recovers}

The next window admits requests again.
";

    /// A committed `spec` whose two criteria **slug alike** (case is not identity) — the
    /// authoring-collision fixture the resume must keep distinguishable from "already
    /// seeded".
    const DUPLICATE_CRITERIA_SPEC: &str = "\
# Duplicate criteria

## Goal

Two criteria that name the same thing.

## Context

An authoring slip the seed must refuse, not absorb.

## Criteria

### Cache eviction  {#first}

The cold entries are evicted.

### Cache Eviction  {#second}

The same criterion, capitalized differently.
";

    /// A committed `spec` whose `criteria` section has **zero** items — the
    /// "nothing to seed from" block fixture.
    const ZERO_CRITERIA_SPEC: &str = "\
# Empty plan

## Goal

A goal with no criteria yet.

## Context

Context without any acceptance criteria.

## Criteria
";

    /// Write a committed spec to its canonical path (`specs/<slug>.md`) under
    /// `repo_root`.
    fn write_committed_spec(repo_root: &Path, slug: &str, body: &str) {
        let path = repo_root.join("specs").join(format!("{slug}.md"));
        std::fs::create_dir_all(path.parent().unwrap()).expect("mk specs/");
        std::fs::write(&path, body).expect("write committed spec");
    }

    /// The done-criterion for T2 (`design/write-commands.md` → Minting a milestone,
    /// `jigc milestone add-from-spec`; `design/storage.md` → The by-task-id join):
    /// over a 3-criteria committed spec, `add_from_spec` mints EXACTLY 3 sub-tasks,
    /// each pinned to the milestone's shared base in its own isolated `tasks/<sub>/`
    /// area, with the 3 sub-task ids in the milestone's task list. Each sub-task's
    /// intent is the criterion's TEXT (`ParsedItem.title`), so its id is the slug of
    /// that title. The spec's physical criterion order is *not* id order, so the
    /// recorded task list keeps mint (physical) order while enumeration is id-sorted.
    #[test]
    fn add_from_spec_mints_one_sub_task_per_criterion() {
        let root = TempRoot::new("from-spec");
        let repo = TempRoot::new("from-spec-repo");
        let base = BasePin::new("3333333333333333333333333333333333333333", "3333333");

        let milestone =
            mint_milestone(root.path(), "Cache rework", base.clone()).expect("milestone mints");
        write_committed_spec(repo.path(), "gateway-rate-limiting", THREE_CRITERIA_SPEC);

        let added = add_from_spec(
            root.path(),
            repo.path(),
            &schemas(),
            &milestone.id,
            "spec:gateway-rate-limiting",
            "single-task",
            None,
        )
        .expect("3-criteria spec seeds 3 sub-tasks");
        // A first pass over an empty list seeds every criterion — nothing was already there.
        assert!(
            added.already_seeded.is_empty(),
            "a first pass finds nothing already seeded"
        );
        let added = added.added;

        // EXACTLY 3 sub-tasks, minted in the spec's physical criterion order, each
        // intent = the criterion title.
        assert_eq!(added.len(), 3, "one sub-task per criterion");
        let intent_ids: Vec<&str> = added.iter().map(|a| a.task.id.as_str()).collect();
        assert_eq!(
            intent_ids,
            vec![
                "rejects-the-101st-request",
                "admits-within-the-window",
                "recovers-after-the-window"
            ],
            "sub-tasks mint from the criterion text in physical order"
        );

        // Each sub-task: its own isolated area, pinned to the MILESTONE's shared base.
        for a in &added {
            assert_eq!(a.milestone_id, "cache-rework");
            let sub_dir = root.path().join("tasks").join(&a.task.id);
            assert_eq!(a.task.dir, sub_dir);
            assert!(sub_dir.is_dir(), "sub-task opens its own isolated area");
            assert_eq!(a.task.base, base, "sub-task inherits the shared base");
            assert_eq!(
                crate::state::read_base_pin(&sub_dir).expect("read sub base"),
                base,
                "the sub-task's base.json equals the milestone's shared base"
            );
            // Its intent is the criterion text verbatim.
            let intent = crate::state::read_intent(&sub_dir).expect("read intent");
            assert!(
                !intent.is_empty() && crate::slug::slugify(&intent) == a.task.id,
                "the sub-task intent is the criterion text: {intent:?}"
            );
        }

        // All 3 ids land in the milestone's task list (recorded = mint/physical order).
        let list = read_task_list(&milestone.dir).expect("read list");
        assert_eq!(
            list.tasks,
            vec![
                "rejects-the-101st-request".to_string(),
                "admits-within-the-window".to_string(),
                "recovers-after-the-window".to_string(),
            ],
            "all 3 sub-task ids are recorded in mint order"
        );
        // Enumeration (what the join reads) is id-sorted, distinct from mint order.
        assert_eq!(
            list.enumerate(),
            vec![
                "admits-within-the-window".to_string(),
                "recovers-after-the-window".to_string(),
                "rejects-the-101st-request".to_string(),
            ],
            "enumeration is id-sorted, not physical/mint order"
        );
    }

    /// **The resume half** (M47 Inc 2 T3): a criterion the **committed record** already
    /// names is *skipped*, never collided — so a second pass over a partially seeded
    /// milestone seeds exactly the remainder, and a fully seeded one is a true ack rather
    /// than a `milestone.sub-task-collision` dead end. The mid-loop unwind primitive
    /// [`drop_sub_tasks`] is what produces the partial state, so the two halves of a
    /// rejected k-th record commit are exercised together. The `recorded` argument is what
    /// the door reads back from the record, and it advances exactly as the record does.
    #[test]
    fn add_from_spec_skips_already_seeded_criteria_and_resumes_the_remainder() {
        let root = TempRoot::new("from-spec-resume");
        let repo = TempRoot::new("from-spec-resume-repo");
        let base = BasePin::new("6666666666666666666666666666666666666666", "6666666");

        let milestone = mint_milestone(root.path(), "Cache rework", base).expect("milestone mints");
        write_committed_spec(repo.path(), "gateway-rate-limiting", THREE_CRITERIA_SPEC);
        let seed = |root: &Path, recorded: &[String]| {
            add_from_spec(
                root,
                repo.path(),
                &schemas(),
                &milestone.id,
                "spec:gateway-rate-limiting",
                "single-task",
                Some(recorded),
            )
        };

        // First pass: the record names nothing, so all 3.
        let first = seed(root.path(), &[]).expect("the first pass seeds every criterion");
        assert_eq!(first.added.len(), 3);

        // The mid-loop state a rejected 2nd record commit leaves: the criteria after the
        // first are unwound — their areas removed and their ids dropped from the list.
        for a in &first.added[1..] {
            std::fs::remove_dir_all(&a.task.dir).expect("remove the un-recorded mint's area");
        }
        let unwound: Vec<String> = first.added[1..].iter().map(|a| a.task.id.clone()).collect();
        drop_sub_tasks(root.path(), &milestone.id, &unwound).expect("drop the un-recorded ids");
        assert_eq!(
            read_task_list(&milestone.dir).expect("read list").tasks,
            vec!["rejects-the-101st-request".to_string()],
            "the unwind leaves the list naming exactly the recorded sub-task"
        );

        // The resume: exactly the remaining 2 mint, the landed one is acked as already seeded.
        let landed = vec!["rejects-the-101st-request".to_string()];
        let resumed = seed(root.path(), &landed).expect("the resume seeds the remainder");
        assert_eq!(
            resumed
                .added
                .iter()
                .map(|a| a.task.id.as_str())
                .collect::<Vec<_>>(),
            vec!["admits-within-the-window", "recovers-after-the-window"],
            "the resume mints only the criteria the milestone does not carry"
        );
        assert_eq!(
            resumed.already_seeded,
            vec!["rejects-the-101st-request".to_string()],
            "the already-seeded criterion is reported, never collided"
        );

        // A fully seeded re-run: nothing to seed, everything acked — and the list is unchanged.
        let all: Vec<String> = read_task_list(&milestone.dir).expect("read list").tasks;
        let again =
            seed(root.path(), &all).expect("a fully seeded re-run acks rather than collides");
        assert!(again.added.is_empty(), "nothing left to seed");
        assert_eq!(
            again.already_seeded,
            vec![
                "rejects-the-101st-request".to_string(),
                "admits-within-the-window".to_string(),
                "recovers-after-the-window".to_string(),
            ],
            "every criterion is reported as already seeded, in physical order"
        );
        assert_eq!(
            read_task_list(&milestone.dir).expect("read list").tasks,
            vec![
                "rejects-the-101st-request".to_string(),
                "admits-within-the-window".to_string(),
                "recovers-after-the-window".to_string(),
            ],
            "an already-seeded pass appends nothing"
        );
    }

    /// **The skip set is the committed record's, never the demoted cache's** (the M47 Inc 2
    /// T3 fix). A sub-task id the gitignored `tasks.json` carries but the record does not —
    /// an unwind that could not finish, a process killed between the mint and its record
    /// commit — must **not** be absorbed as "already seeded": that criterion would be absent
    /// from the committed, team-ready record forever, at a positive ack. Keyed on the record,
    /// it is not skipped, so it surfaces [`add_task`]'s loud collision instead of vanishing.
    #[test]
    fn add_from_spec_keys_its_skip_set_on_the_record_not_the_demoted_cache() {
        let root = TempRoot::new("from-spec-cache-only");
        let repo = TempRoot::new("from-spec-cache-only-repo");
        let base = BasePin::new("8888888888888888888888888888888888888888", "8888888");

        let milestone = mint_milestone(root.path(), "Cache rework", base).expect("milestone mints");
        write_committed_spec(repo.path(), "gateway-rate-limiting", THREE_CRITERIA_SPEC);

        // The divergence: the cache names the first two criteria; the record names only the
        // first (the second's record commit never landed and its unwind never finished).
        let cache_only = "admits-within-the-window";
        std::fs::write(
            milestone.dir.join(TASKS_FILE),
            TaskList {
                tasks: vec![
                    "rejects-the-101st-request".to_string(),
                    cache_only.to_string(),
                ],
            }
            .to_bytes(),
        )
        .expect("plant the cache residue");

        let recorded = vec!["rejects-the-101st-request".to_string()];
        let aborted = add_from_spec(
            root.path(),
            repo.path(),
            &schemas(),
            &milestone.id,
            "spec:gateway-rate-limiting",
            "single-task",
            Some(&recorded),
        )
        .expect_err("a cache id the record does not name is never skipped");

        assert_eq!(
            aborted.finding.code, "milestone.sub-task-collision",
            "the un-recorded cache id blocks LOUDLY rather than passing as already-seeded"
        );
        assert!(
            aborted.finding.message.contains(cache_only),
            "the block names the diverged sub-task: {:?}",
            aborted.finding.message
        );
        assert!(
            aborted.minted.is_empty(),
            "the recorded criterion was skipped and the diverged one aborted — nothing minted"
        );
    }

    /// The resume must not swallow an **authoring** collision: two criteria of the *same*
    /// spec slugging alike still surface [`add_task`]'s `milestone.sub-task-collision`,
    /// because the skip set is read **once, at entry** — an id minted by this
    /// very call is not in it (M47 Inc 2 T3).
    #[test]
    fn add_from_spec_still_blocks_a_within_spec_duplicate_criterion() {
        let root = TempRoot::new("from-spec-dup");
        let repo = TempRoot::new("from-spec-dup-repo");
        let base = BasePin::new("7777777777777777777777777777777777777777", "7777777");

        let milestone = mint_milestone(root.path(), "Cache rework", base).expect("milestone mints");
        write_committed_spec(repo.path(), "duplicate-criteria", DUPLICATE_CRITERIA_SPEC);

        let err = add_from_spec(
            root.path(),
            repo.path(),
            &schemas(),
            &milestone.id,
            "spec:duplicate-criteria",
            "single-task",
            Some(&[]),
        )
        .expect_err("two criteria slugging alike collide");
        assert_eq!(err.finding.severity, Severity::Blocking);
        assert_eq!(err.finding.code, "milestone.sub-task-collision");
        // **The abort hands its mints back** (M47 Inc 2 T3 fix) — the first criterion did
        // mint, and the record will never name it, so the door must be able to unwind it.
        assert_eq!(
            err.minted
                .iter()
                .map(|a| a.task.id.as_str())
                .collect::<Vec<_>>(),
            vec!["cache-eviction"],
            "the aborted pass carries out what it minted, for the door to unwind"
        );
    }

    /// A spec whose `criteria` section has **zero** items returns a blocking,
    /// route-bearing `milestone.no-criteria` finding and mints nothing — never a
    /// silent empty milestone (`design/write-commands.md` → Minting a milestone:
    /// "A spec with zero criteria items is a blocking 'nothing to seed from'").
    #[test]
    fn add_from_spec_with_zero_criteria_blocks_and_mints_nothing() {
        let root = TempRoot::new("zero-crit");
        let repo = TempRoot::new("zero-crit-repo");
        let base = BasePin::new("4444444444444444444444444444444444444444", "4444444");

        let milestone = mint_milestone(root.path(), "Cache rework", base).expect("milestone mints");
        write_committed_spec(repo.path(), "empty-plan", ZERO_CRITERIA_SPEC);

        let err = add_from_spec(
            root.path(),
            repo.path(),
            &schemas(),
            &milestone.id,
            "spec:empty-plan",
            "single-task",
            Some(&[]),
        )
        .expect_err("a zero-criteria spec blocks");
        assert!(
            err.minted.is_empty(),
            "a failure before the loop minted nothing to unwind"
        );
        let err = err.finding;

        assert_eq!(err.severity, Severity::Blocking);
        assert_eq!(err.code, "milestone.no-criteria");
        assert!(
            err.message.contains("empty-plan"),
            "the block names the empty spec: {err:?}"
        );
        // Round-2 D6h: the first route arm names the EXECUTABLE repair path (the
        // in-task add-item copy-on-first-touch, verified live), never a hand-edit.
        let route = err
            .route
            .as_ref()
            .expect("the no-criteria block carries a route");
        assert!(
            route.contains("`jigc doc add-item spec:empty-plan#criteria")
                && route.contains("re-promotes"),
            "the route's first arm names the in-task add-item repair path: {route}"
        );

        // Nothing minted: the task list is still empty and no tasks/ areas exist.
        assert_eq!(
            read_task_list(&milestone.dir).expect("read list").tasks,
            Vec::<String>::new(),
            "a zero-criteria block appends nothing"
        );
        assert!(
            !root.path().join("tasks").exists(),
            "a zero-criteria block mints no sub-task area"
        );
    }

    /// An unknown milestone rejects before any spec read or mint; an unknown spec
    /// (no committed file) rejects with a routed store-shaped block — each reusing
    /// the existing milestone + store finding shapes.
    #[test]
    fn add_from_spec_rejects_unknown_milestone_and_unknown_spec() {
        let root = TempRoot::new("from-spec-rejects");
        let repo = TempRoot::new("from-spec-rejects-repo");
        let base = BasePin::new("5555555555555555555555555555555555555555", "5555555");

        // Unknown milestone → reject (reuses the add-task unknown-milestone shape).
        let unknown_ms = add_from_spec(
            root.path(),
            repo.path(),
            &schemas(),
            "no-such-milestone",
            "spec:whatever",
            "single-task",
            Some(&[]),
        )
        .expect_err("an unknown milestone rejects")
        .finding;
        assert_eq!(unknown_ms.severity, Severity::Blocking);
        assert_eq!(unknown_ms.code, "milestone.unknown");
        assert!(unknown_ms.route.is_some());

        // Unknown spec (no committed file) → routed store-not-found block, nothing minted.
        let milestone = mint_milestone(root.path(), "Cache rework", base).expect("milestone mints");
        let unknown_spec = add_from_spec(
            root.path(),
            repo.path(),
            &schemas(),
            &milestone.id,
            "spec:does-not-exist",
            "single-task",
            Some(&[]),
        )
        .expect_err("an unknown spec rejects")
        .finding;
        assert_eq!(unknown_spec.severity, Severity::Blocking);
        assert_eq!(unknown_spec.code, "store.not-found");
        assert!(unknown_spec.route.is_some());
        assert_eq!(
            read_task_list(&milestone.dir).expect("read list").tasks,
            Vec::<String>::new(),
            "an unknown spec appends nothing"
        );
    }

    /// (V13) The spec-slice read (`read_spec_criteria`) blocks `store.unknown-type`
    /// on an unknown doctype and its route names `jigc describe` — the real
    /// discovery verb — never a nonexistent `doc types` subcommand.
    #[test]
    fn read_spec_criteria_unknown_type_route_names_jigc_describe() {
        let root = TempRoot::new("spec-slice-unknown-type");
        let err = read_spec_criteria(root.path(), &schemas(), "wormhole:whatever")
            .expect_err("an unknown doctype blocks");

        assert_eq!(err.code, "store.unknown-type");
        let route = err.route.expect("the block carries a route");
        assert!(
            route.contains("jigc describe"),
            "route names the real discovery verb: {route}"
        );
        assert!(
            !route.contains(&["jigc doc", "types"].join(" ")),
            "route must not name the nonexistent subcommand: {route}"
        );
    }

    /// (M47 inc-10 T4 · the P2-6 residue, the **third** producer of
    /// `store.no-such-section`) `add-from-spec` accepts any `<type>:<slug>`, so
    /// pointing it at a doc whose doctype declares no `criteria` section — a
    /// committed ADR, parsed clean — reaches this producer in production. Its route
    /// used to restate the requirement (*"the spec must declare a `criteria`
    /// section"*) without naming the sections the addressed doc **does** declare or
    /// handing over a runnable read. It is now a [`Route::mechanical`] read of the
    /// addressed doc, with the real section ids in the tail.
    #[test]
    fn read_spec_criteria_no_criteria_section_route_reads_the_addressed_doc() {
        use crate::finding::RouteKind;

        let root = TempRoot::new("spec-slice-no-criteria");
        let path = root.path().join("decisions").join("single-node-cache.md");
        std::fs::create_dir_all(path.parent().unwrap()).expect("mk decisions/");
        std::fs::write(
            &path,
            "\
---
status: accepted
date: 2026-05-23
---

# Single-node session cache

## Context
Session lookups must stay sub-millisecond.

## Options
Alternatives were weighed and rejected.

## Decision
A single in-memory node keeps session lookups sub-millisecond.

## Consequences
A cold node loses its sessions; clients re-authenticate.
",
        )
        .expect("write committed ADR");

        let err = read_spec_criteria(root.path(), &join_schemas(), "adr:single-node-cache")
            .expect_err("a doc with no `criteria` section blocks");

        assert_eq!(err.code, "store.no-such-section");
        let route = err.route.expect("the block carries a route");
        assert!(
            matches!(route.kind(), RouteKind::Mechanical { .. }),
            "the reshaped route is a copy-runnable read, so it buys the parse fence: {route:?}"
        );
        assert!(
            route.starts_with("`jigc doc show adr:single-node-cache`"),
            "the route leads with the runnable read of the addressed doc: {route}"
        );
        for id in ["context", "options", "decision", "consequences"] {
            assert!(
                route.contains(&format!("`{id}`")),
                "the route names the section `{id}` the addressed doc really declares: {route}"
            );
        }
    }

    const COMMIT_YAML: &[u8] = include_bytes!("../../cli/pack/schemas/commit.yaml");
    const ADR_YAML: &[u8] = include_bytes!("../../cli/pack/schemas/adr.yaml");

    /// The schema set the join's per-area `overlay_working` basis resolves staged
    /// doc types against — `commit` (the `created` instance) and `adr` (the
    /// `edited-from-base` instance).
    fn join_schemas() -> BTreeMap<String, Schema> {
        let mut m = BTreeMap::new();
        m.insert(
            "commit".to_string(),
            crate::schema::load_schema(COMMIT_YAML).expect("commit.yaml loads"),
        );
        m.insert(
            "adr".to_string(),
            crate::schema::load_schema_with_types(ADR_YAML, &crate::schema::dev_pack_field_types())
                .expect("adr.yaml loads"),
        );
        m
    }

    /// The done-criterion for T1 (`design/storage.md` → The by-task-id join, algorithm
    /// steps 1–2: order by task id; disjoint union classified by provenance). Two
    /// **disjoint** sub-task areas — distinct slugs, one a `created` instance
    /// ([`crate::state::provision_doc`]), one an `edited-from-base` instance
    /// ([`crate::state::copy_in`]) — fold into a merged overlay that contains **both**
    /// docs with their **distinct** provenance and contributing sub-task. The fold is
    /// proven order-invariant: folding the **reverse** enumeration yields the
    /// **identical** merged overlay (the only accumulator is an address-keyed
    /// `BTreeMap`, never an unsorted `HashMap` reaching output; Validation hardening
    /// #7). The fixture is built so the milestone's on-disk sub-task areas, listed in
    /// raw `read_dir` order, are **not** pre-sorted by id, so a green fold cannot be an
    /// accident of filesystem order.
    #[test]
    fn join_folds_disjoint_sub_areas_classified_by_provenance_order_invariant() {
        let root = TempRoot::new("join-skeleton");
        let base = BasePin::new("6666666666666666666666666666666666666666", "6666666");
        let schemas = join_schemas();
        let committed = crate::index::EdgeIndex::default();

        let milestone = mint_milestone(root.path(), "Cache rework", base).expect("milestone mints");

        // Two disjoint sub-tasks, **inserted in reverse id order** (`zebra` before
        // `alpha`) so insertion/read_dir order diverges from id order.
        add_task(root.path(), &milestone.id, "Zebra area", "single-task").expect("zebra adds");
        add_task(root.path(), &milestone.id, "Alpha area", "single-task").expect("alpha adds");

        // Stage one `created` doc into the `alpha-area` sub-task and one
        // `edited-from-base` doc into the `zebra-area` sub-task — disjoint slugs.
        let alpha_dir = root.path().join("tasks").join("alpha-area");
        crate::state::provision_doc(
            &alpha_dir,
            &schemas["commit"],
            "alpha-area",
            "alpha-area",
            &[],
        )
        .expect("provision created doc");
        let zebra_dir = root.path().join("tasks").join("zebra-area");
        let adr_source = "---\nstatus: accepted\ndate: 2026-06-04\n---\n\n# Zebra decision\n\n## Context\n\nForces.\n\n## Options\n\nAlternatives were weighed and rejected.\n\n## Decision\n\nDo the thing.\n";
        crate::state::copy_in(&zebra_dir, "adr", "zebra-decision", adr_source)
            .expect("copy in edited-from-base doc");

        // The fixture's RECORDED task list is reverse-id insertion order, the
        // guaranteed divergence: if the fold keyed on recorded/insertion order instead
        // of the id-sorted set, it would observe `[zebra-area, alpha-area]` — yet the
        // overlay must come out id-sorted regardless (proven by the reverse-fold below).
        let recorded = read_task_list(&milestone.dir).expect("read list").tasks;
        assert_eq!(
            recorded,
            vec!["zebra-area".to_string(), "alpha-area".to_string()],
            "the recorded backing list is reverse-id insertion order"
        );
        // The on-disk areas' raw read_dir order is also captured; on filesystems that
        // do not pre-sort it diverges from id order (a green fold then cannot be an
        // accident of read_dir handing back id order). The recorded-order divergence
        // above and the reverse-fold below are the order-independent guarantee.
        let read_dir_order: Vec<String> = std::fs::read_dir(root.path().join("tasks"))
            .expect("tasks dir")
            .map(|e| e.expect("entry").file_name().to_string_lossy().into_owned())
            .collect();

        // The join folds by sorted task id and produces the merged overlay.
        let outcome = join(
            root.path(),
            root.path(),
            &milestone.id,
            &schemas,
            &committed,
        )
        .expect("the join folds");

        // The merged overlay contains BOTH staged docs with their distinct provenance
        // and contributing sub-task.
        assert_eq!(
            outcome.overlay.len(),
            2,
            "the disjoint union holds both staged docs"
        );
        let created = outcome
            .overlay
            .get("commit:alpha-area")
            .expect("the created doc is in the overlay");
        assert_eq!(created.provenance, crate::state::Provenance::Created);
        assert_eq!(created.source_task, "alpha-area");
        let edited = outcome
            .overlay
            .get("adr:zebra-decision")
            .expect("the edited-from-base doc is in the overlay");
        assert_eq!(edited.provenance, crate::state::Provenance::EditedFromBase);
        assert_eq!(edited.source_task, "zebra-area");

        // The skeleton increment surfaces no findings (clash/cross-area are T2/T3).
        assert!(
            outcome.findings.is_empty(),
            "the skeleton join surfaces no findings: {:?}",
            outcome.findings
        );

        // Order-invariance: folding the REVERSE enumeration yields the IDENTICAL
        // merged overlay. The id-sorted enumeration is `[alpha-area, zebra-area]`; its
        // reverse `[zebra-area, alpha-area]` must produce a byte-identical outcome.
        let ids = read_task_list(&milestone.dir)
            .expect("read list")
            .enumerate();
        assert_eq!(
            ids,
            vec!["alpha-area".to_string(), "zebra-area".to_string()],
            "enumeration is id-sorted"
        );
        let forward = fold_areas(
            root.path(),
            root.path(),
            &milestone.id,
            &ids,
            &schemas,
            &committed,
        )
        .expect("forward fold");
        let mut reversed = ids.clone();
        reversed.reverse();
        let backward = fold_areas(
            root.path(),
            root.path(),
            &milestone.id,
            &reversed,
            &schemas,
            &committed,
        )
        .expect("reverse fold");
        assert_eq!(
            forward, backward,
            "the fold is order-invariant: reversed enumeration yields the identical overlay"
        );
        assert_eq!(
            forward, outcome,
            "the public join's id-sorted fold equals the explicit forward fold"
        );

        // Feeding the raw read_dir order itself yields the IDENTICAL overlay — so even
        // if the filesystem hands back a non-id order, it cannot leak into the output
        // (the done-criterion's read_dir-order ≠ id-order obligation, proven directly:
        // the fold's only accumulator is an address-keyed `BTreeMap`).
        let by_read_dir = fold_areas(
            root.path(),
            root.path(),
            &milestone.id,
            &read_dir_order,
            &schemas,
            &committed,
        )
        .expect("read_dir-order fold");
        assert_eq!(
            by_read_dir, outcome,
            "folding in raw read_dir order yields the identical overlay: {read_dir_order:?}"
        );
    }

    /// An unknown milestone rejects before any sub-area is read (reuses the shared
    /// `milestone.unknown` block shape).
    #[test]
    fn join_rejects_unknown_milestone() {
        let root = TempRoot::new("join-unknown");
        let err = join(
            root.path(),
            root.path(),
            "no-such-milestone",
            &join_schemas(),
            &crate::index::EdgeIndex::default(),
        )
        .expect_err("an unknown milestone rejects");
        assert_eq!(err.severity, Severity::Blocking);
        assert_eq!(err.code, "milestone.unknown");
        assert!(err.route.is_some());
    }

    /// Stage a doc body into a sub-task's `docs/` area with a chosen [`Provenance`],
    /// the way the two staging primitives would have but with a caller-supplied body
    /// (so a `created` instance can carry a deliberate self-reference). Writes
    /// `<sub_dir>/docs/<type>:<slug>.md` verbatim and records the provenance in that
    /// area's manifest — the two inputs the join's clash rule reads.
    fn stage_doc(
        sub_dir: &Path,
        type_name: &str,
        slug: &str,
        body: &str,
        provenance: crate::state::Provenance,
    ) {
        let path = crate::state::instance_path(sub_dir, type_name, slug);
        std::fs::create_dir_all(path.parent().unwrap()).expect("mk docs/");
        std::fs::write(&path, body).expect("write staged body");
        let mut record =
            crate::state::ProvenanceRecord::load(sub_dir).expect("load provenance manifest");
        record.record(format!("{type_name}:{slug}"), provenance);
        std::fs::write(
            crate::state::ProvenanceRecord::path_in(sub_dir),
            record.to_bytes(),
        )
        .expect("write provenance manifest");
    }

    /// An ADR body whose `supersedes` ref points at `to` — used to build a `created`
    /// ADR that references its *own* slug on purpose (the self-ref the suffix rule must
    /// rewrite in lockstep with the slug suffix).
    fn adr_superseding(title: &str, to: &str) -> String {
        format!(
            "---\nstatus: accepted\ndate: 2026-06-04\nsupersedes: {to}\n---\n\n# {title}\n\n## Context\n\nForces.\n\n## Options\n\nAlternatives were weighed and rejected.\n\n## Decision\n\nDo the thing.\n\n## Consequences\n\nTradeoffs.\n"
        )
    }

    /// The done-criterion for T2 (`design/storage.md` → The by-task-id join, step 4:
    /// colliding new instances + intra-doc self-ref rewrite; `structural-grammar.md` →
    /// IDs: provenance and minting → deterministic suffix in task-id merge order). Two
    /// sub-tasks each `created` an `adr:cache-strategy` instance whose body references
    /// its **own** slug (`supersedes: adr:cache-strategy`). These are *distinct* docs,
    /// not a clash: the **lower-task-id** instance keeps the bare `adr:cache-strategy`;
    /// the **higher** takes the deterministic `-2` suffix → `adr:cache-strategy-2`, and
    /// the suffixed instance's OWN self-reference is rewritten in lockstep to
    /// `adr:cache-strategy-2` (so the renamed doc never dangles or points at its
    /// sibling). The merged overlay holds both, suffix-resolved; the join surfaces no
    /// findings (a collision of distinct `created` instances is not an error).
    #[test]
    fn join_suffixes_colliding_created_instances_and_rewrites_self_ref() {
        let root = TempRoot::new("join-suffix");
        let base = BasePin::new("7777777777777777777777777777777777777777", "7777777");
        let schemas = join_schemas();
        let committed = crate::index::EdgeIndex::default();

        let milestone = mint_milestone(root.path(), "Cache rework", base).expect("milestone mints");

        // Two sub-tasks, inserted in reverse-id order so insertion order diverges from
        // id order. The id-sorted enumeration is `[area-low, area-zed]`, so `area-low`
        // is the lower task id (keeps the bare slug) and `area-zed` the higher (suffixed).
        add_task(root.path(), &milestone.id, "Area zed", "single-task").expect("zed adds");
        add_task(root.path(), &milestone.id, "Area low", "single-task").expect("low adds");

        // Each area stages a `created` `adr:cache-strategy` that supersedes its OWN slug.
        let low_dir = root.path().join("tasks").join("area-low");
        stage_doc(
            &low_dir,
            "adr",
            "cache-strategy",
            &adr_superseding("Cache strategy", "adr:cache-strategy"),
            crate::state::Provenance::Created,
        );
        let zed_dir = root.path().join("tasks").join("area-zed");
        stage_doc(
            &zed_dir,
            "adr",
            "cache-strategy",
            &adr_superseding("Cache strategy", "adr:cache-strategy"),
            crate::state::Provenance::Created,
        );

        let outcome = join(
            root.path(),
            root.path(),
            &milestone.id,
            &schemas,
            &committed,
        )
        .expect("the join folds");

        // No findings: a collision of distinct `created` instances is a suffix, not a clash.
        assert!(
            outcome.findings.is_empty(),
            "colliding `created` instances suffix, never block: {:?}",
            outcome.findings
        );

        // The overlay holds both, suffix-resolved by task-id order.
        assert_eq!(
            outcome.overlay.len(),
            2,
            "both distinct created docs survive"
        );
        let bare = outcome
            .overlay
            .get("adr:cache-strategy")
            .expect("the lower-task-id instance keeps the bare slug");
        assert_eq!(bare.source_task, "area-low");
        assert_eq!(bare.provenance, crate::state::Provenance::Created);
        // The bare instance's self-ref stays its own (bare) slug.
        assert_eq!(
            bare.edges,
            vec![crate::index::Edge {
                from: "adr:cache-strategy".to_string(),
                relation: "supersedes".to_string(),
                to: "adr:cache-strategy".to_string(),
            }],
            "the bare instance references its own bare slug"
        );

        let suffixed = outcome
            .overlay
            .get("adr:cache-strategy-2")
            .expect("the higher-task-id instance takes the `-2` suffix");
        assert_eq!(suffixed.source_task, "area-zed");
        assert_eq!(suffixed.provenance, crate::state::Provenance::Created);
        // The suffixed instance's OWN self-reference is rewritten in lockstep: both the
        // edge's `from` (its identity) and its self-ref `to` are the suffixed slug.
        assert_eq!(
            suffixed.edges,
            vec![crate::index::Edge {
                from: "adr:cache-strategy-2".to_string(),
                relation: "supersedes".to_string(),
                to: "adr:cache-strategy-2".to_string(),
            }],
            "the suffixed instance's self-ref is rewritten to the suffixed slug"
        );

        // The suffix assignment is by **task id**, not by the order the areas are fed:
        // folding the enumeration and its reverse yields the byte-identical outcome (the
        // lower id always keeps the bare slug). A naïve fold keyed on input order would
        // hand the bare slug to whichever area came first.
        let ids = read_task_list(&milestone.dir)
            .expect("read list")
            .enumerate();
        let mut reversed = ids.clone();
        reversed.reverse();
        let forward = fold_areas(
            root.path(),
            root.path(),
            &milestone.id,
            &ids,
            &schemas,
            &committed,
        )
        .expect("fwd fold");
        let backward = fold_areas(
            root.path(),
            root.path(),
            &milestone.id,
            &reversed,
            &schemas,
            &committed,
        )
        .expect("rev fold");
        assert_eq!(
            forward, backward,
            "the suffix fold is order-invariant: the lower task id keeps the bare slug"
        );
        assert_eq!(forward, outcome, "the public join equals the explicit fold");
    }

    /// The done-criterion for T2 (`design/storage.md` → The by-task-id join, step 3:
    /// same-doc clash incl. the mixed case). Two sub-areas staging **`edited-from-base`**
    /// writes to the **same** committed-at-base slug is a partition violation → a
    /// blocking, route-bearing `join.same-doc-clash` (never a blind merge). The **mixed
    /// case** — one sub-area `created` a slug the other `edited-from-base` (same slug) —
    /// is *also* a blocking clash, not a suffix. Each clashing slug emits exactly one
    /// block and is kept out of the merged overlay.
    #[test]
    fn join_blocks_same_doc_clash_including_the_mixed_case() {
        let root = TempRoot::new("join-clash");
        let base = BasePin::new("8888888888888888888888888888888888888888", "8888888");
        let schemas = join_schemas();
        let committed = crate::index::EdgeIndex::default();

        let milestone = mint_milestone(root.path(), "Cache rework", base).expect("milestone mints");

        // Three sub-areas. `evict` + `purge` both edit the SAME base slug
        // `adr:eviction-policy` (the pure two-edited clash). `alpha` creates and `beta`
        // edits the SAME slug `adr:retention` (the mixed clash).
        add_task(root.path(), &milestone.id, "Purge area", "single-task").expect("purge adds");
        add_task(root.path(), &milestone.id, "Evict area", "single-task").expect("evict adds");
        add_task(root.path(), &milestone.id, "Beta area", "single-task").expect("beta adds");
        add_task(root.path(), &milestone.id, "Alpha area", "single-task").expect("alpha adds");

        let adr = |title: &str| {
            format!(
                "---\nstatus: accepted\ndate: 2026-06-04\n---\n\n# {title}\n\n## Context\n\nForces.\n\n## Options\n\nAlternatives were weighed and rejected.\n\n## Decision\n\nDo the thing.\n\n## Consequences\n\nTradeoffs.\n"
            )
        };

        // Two `edited-from-base` writes to one committed-at-base slug → clash.
        stage_doc(
            &root.path().join("tasks").join("evict-area"),
            "adr",
            "eviction-policy",
            &adr("Eviction policy"),
            crate::state::Provenance::EditedFromBase,
        );
        stage_doc(
            &root.path().join("tasks").join("purge-area"),
            "adr",
            "eviction-policy",
            &adr("Eviction policy"),
            crate::state::Provenance::EditedFromBase,
        );

        // Mixed case: one `created`, one `edited-from-base`, same slug → clash.
        stage_doc(
            &root.path().join("tasks").join("alpha-area"),
            "adr",
            "retention",
            &adr("Retention"),
            crate::state::Provenance::Created,
        );
        stage_doc(
            &root.path().join("tasks").join("beta-area"),
            "adr",
            "retention",
            &adr("Retention"),
            crate::state::Provenance::EditedFromBase,
        );

        let outcome = join(
            root.path(),
            root.path(),
            &milestone.id,
            &schemas,
            &committed,
        )
        .expect("the join folds");

        // Exactly two blocking clashes, one per clashing slug, each route-bearing.
        let clashes: Vec<&Finding> = outcome
            .findings
            .iter()
            .filter(|f| f.code == "join.same-doc-clash")
            .collect();
        assert_eq!(
            clashes.len(),
            2,
            "one clash per clashing slug (two-edited + mixed): {:?}",
            outcome.findings
        );
        for c in &clashes {
            assert_eq!(c.severity, Severity::Blocking);
            assert!(c.route.is_some(), "a same-doc clash carries a route: {c:?}");
        }
        // Both clashing slugs are named.
        assert!(
            clashes
                .iter()
                .any(|c| c.message.contains("adr:eviction-policy")),
            "the two-edited clash names its slug: {clashes:?}"
        );
        assert!(
            clashes.iter().any(|c| c.message.contains("adr:retention")),
            "the mixed clash names its slug: {clashes:?}"
        );
        // A clashing slug is kept OUT of the merged overlay — never blind-merged.
        assert!(
            !outcome.overlay.contains_key("adr:eviction-policy"),
            "the two-edited clash is not merged"
        );
        assert!(
            !outcome.overlay.contains_key("adr:retention"),
            "the mixed clash is not merged"
        );
    }

    /// Cross-group final-address collision (the M7-audit LOW finding): the suffix rule
    /// resolves a same-slug `created` group `adr:cache-strategy` to the pair
    /// `adr:cache-strategy` and `adr:cache-strategy-2`, while a **separate** group's slug
    /// is *already* `adr:cache-strategy-2` (a sub-task whose intent slugged that way), so
    /// the two **final** addresses collide. Without a cross-group guard, the `BTreeMap`
    /// insert (and the materialize `fs::write`) would silently let one win — a
    /// deterministic but lossy overwrite that drops a sub-task's doc with no finding. The
    /// join must instead emit a blocking, route-bearing `join.same-doc-clash` for the
    /// colliding final address and keep it out of the merged overlay (never overwrite).
    #[test]
    fn join_blocks_cross_group_final_address_collision() {
        let root = TempRoot::new("join-cross-group-collision");
        let base = BasePin::new("9999999999999999999999999999999999999999", "9999999");
        let schemas = join_schemas();
        let committed = crate::index::EdgeIndex::default();

        let milestone =
            mint_milestone(root.path(), "Cache strategy", base).expect("milestone mints");

        // Three sub-areas. Two `created` `adr:cache-strategy` (→ bare + `-2` suffix); a
        // third `created` whose slug IS already `cache-strategy-2` — so the suffixed
        // result of the first group and the bare slug of the third group are the SAME
        // final address `adr:cache-strategy-2`.
        add_task(root.path(), &milestone.id, "Low strategy", "single-task").expect("low adds");
        add_task(root.path(), &milestone.id, "Zed strategy", "single-task").expect("zed adds");
        add_task(root.path(), &milestone.id, "Pre strategy", "single-task").expect("pre adds");

        let adr = |title: &str| {
            format!(
                "---\nstatus: accepted\ndate: 2026-06-04\n---\n\n# {title}\n\n## Context\n\nForces.\n\n## Options\n\nAlternatives were weighed and rejected.\n\n## Decision\n\nDo the thing.\n\n## Consequences\n\nTradeoffs.\n"
            )
        };

        // Group A: two `created` `adr:cache-strategy` → `adr:cache-strategy` (lower id)
        // + `adr:cache-strategy-2` (higher id).
        stage_doc(
            &root.path().join("tasks").join("low-strategy"),
            "adr",
            "cache-strategy",
            &adr("Cache strategy"),
            crate::state::Provenance::Created,
        );
        stage_doc(
            &root.path().join("tasks").join("zed-strategy"),
            "adr",
            "cache-strategy",
            &adr("Cache strategy"),
            crate::state::Provenance::Created,
        );

        // Group B: a separate `created` whose slug is ALREADY `cache-strategy-2` — its
        // bare final address collides with group A's suffixed result.
        stage_doc(
            &root.path().join("tasks").join("pre-strategy"),
            "adr",
            "cache-strategy-2",
            &adr("Cache strategy two"),
            crate::state::Provenance::Created,
        );

        let outcome = join(
            root.path(),
            root.path(),
            &milestone.id,
            &schemas,
            &committed,
        )
        .expect("the join folds");

        // The colliding final address must surface a blocking, route-bearing clash —
        // never a silent overwrite.
        let collision: Vec<&Finding> = outcome
            .findings
            .iter()
            .filter(|f| {
                f.code == "join.same-doc-clash" && f.message.contains("adr:cache-strategy-2")
            })
            .collect();
        assert_eq!(
            collision.len(),
            1,
            "the cross-group final-address collision surfaces exactly one blocking clash: {:?}",
            outcome.findings
        );
        assert_eq!(collision[0].severity, Severity::Blocking);
        assert!(
            collision[0].route.is_some(),
            "the collision clash carries a route: {:?}",
            collision[0]
        );
    }

    /// The done-criterion for T3, part (a) (`design/storage.md` → The by-task-id join,
    /// step 5; `design/validation.md` → Fan-out cross-area refs: the per-`from`
    /// narrowing). Sub-area B authors `supersedes: adr:lru-eviction` while that slug
    /// **byte-exists only** in sibling area A's `docs/`. The join runs the existing
    /// single-area `ref_resolves` walk **once per sub-area** against `committed ∪ that
    /// one area` — so B's ref resolves in neither A (excluded) nor the committed store →
    /// a **blocking `schema-conformance.ref-resolves`**. The control assertion proves the
    /// per-area scoping is load-bearing, not incidental: a **naïve all-areas union
    /// overlay** (B's edges keyed `from`, the union of every area's `task_froms` as the
    /// reachable set) would resolve the very same ref *clean* — the silent-defeat bug the
    /// per-`from` narrowing exists to prevent.
    #[test]
    fn join_rejects_cross_area_ref_proving_per_area_scoping_is_load_bearing() {
        let root = TempRoot::new("join-cross-area");
        let repo = TempRoot::new("join-cross-area-repo");
        let base = BasePin::new("9999999999999999999999999999999999999999", "9999999");
        let schemas = join_schemas();
        let committed = crate::index::EdgeIndex::default();

        let milestone = mint_milestone(root.path(), "Cache rework", base).expect("milestone mints");

        // Two sub-areas. A creates `adr:lru-eviction`; B creates `adr:cache-strategy`
        // whose `supersedes` GUESSES A's slug — a cross-area ref. Inserted in reverse-id
        // order so insertion order diverges from id order.
        add_task(root.path(), &milestone.id, "B area", "single-task").expect("b adds");
        add_task(root.path(), &milestone.id, "Apex area", "single-task").expect("a adds");

        let a_dir = root.path().join("tasks").join("apex-area");
        stage_doc(
            &a_dir,
            "adr",
            "lru-eviction",
            &adr_superseding("LRU eviction", "adr:lru-eviction"),
            crate::state::Provenance::Created,
        );
        let b_dir = root.path().join("tasks").join("b-area");
        stage_doc(
            &b_dir,
            "adr",
            "cache-strategy",
            // B references A's slug — resolvable only inside A's sibling area.
            &adr_superseding("Cache strategy", "adr:lru-eviction"),
            crate::state::Provenance::Created,
        );

        let outcome = join(
            root.path(),
            repo.path(),
            &milestone.id,
            &schemas,
            &committed,
        )
        .expect("join folds");

        // B's cross-area ref is rejected with the existing, intrinsic, already-floored
        // `schema-conformance.ref-resolves` (no new check id).
        let cross: Vec<&Finding> = outcome
            .findings
            .iter()
            .filter(|f| f.code == "schema-conformance.ref-resolves")
            .collect();
        assert_eq!(
            cross.len(),
            1,
            "exactly one cross-area ref is rejected: {:?}",
            outcome.findings
        );
        assert_eq!(cross[0].severity, Severity::Blocking);
        assert!(
            cross[0].route.is_some(),
            "the cross-area block carries a route"
        );
        assert!(
            cross[0].message.contains("adr:lru-eviction"),
            "the block names the unreachable target: {:?}",
            cross[0]
        );
        assert!(
            cross[0]
                .location
                .as_ref()
                .and_then(|l| l.address.as_deref())
                == Some("adr:cache-strategy#supersedes/adr:lru-eviction"),
            "the block is located at B's authoring doc + the `#<relation>/<type>:<to-slug>` \
             fragment (the stable finding key): {:?}",
            cross[0]
        );

        // CONTROL — per-area scoping is load-bearing, not incidental. A naïve all-areas
        // union (B's outgoing edges over the union of every area's staged `from`
        // identities as the reachable set) would resolve the SAME ref clean. We feed
        // `ref_resolves` exactly that flattened overlay and assert ZERO findings — so the
        // green block above can only come from the per-`from` narrowing, never from the
        // target being genuinely unreachable.
        let b_area = crate::index::overlay_working(&committed, &b_dir, &schemas);
        let a_area = crate::index::overlay_working(&committed, &a_dir, &schemas);
        let mut union_froms = b_area.task_froms.clone();
        union_froms.extend(a_area.task_froms.clone()); // the flattened multi-area surface
        union_froms.sort();
        union_froms.dedup();
        let naive_union = crate::index::WorkingOverlay {
            committed: committed.edges.clone(),
            task_edges: b_area.task_edges.clone(),
            task_froms: union_froms,
        };
        // A naïve-union resolution writes A's body into B's reachable surface, so the
        // ref's target byte-exists on B's read path — mimic that flattened view.
        let naive_dir = TempRoot::new("join-cross-area-naive");
        for dir in [&a_dir, &b_dir] {
            for entry in std::fs::read_dir(dir.join("docs")).expect("docs/") {
                let p = entry.expect("entry").path();
                if p.extension().and_then(|x| x.to_str()) == Some("md") {
                    let dst = naive_dir.path().join("docs");
                    std::fs::create_dir_all(&dst).expect("mk docs/");
                    std::fs::copy(&p, dst.join(p.file_name().unwrap())).expect("copy");
                }
            }
        }
        let control =
            crate::index::ref_resolves(&naive_union, repo.path(), naive_dir.path(), &schemas);
        assert!(
            control.is_empty(),
            "CONTROL: the SAME ref resolves clean against a naïve all-areas union — \
             so the per-area scoping is what makes the rejection load-bearing: {control:?}"
        );
    }

    /// The done-criterion for T3, part (b) (`design/storage.md` → isolation is structural
    /// and join-checked in M7; the write-time `--task` barrier is M8). A sub-area whose
    /// provenance manifest **attributes a doc to itself** that it does not physically
    /// stage in its own `docs/` is not attributable to its own sub-area → a **blocking**
    /// `join.area-isolation` finding, route-bearing, emitted directly with no `knobs.yaml`
    /// row. The legitimately-staged doc still merges; only the stray attribution blocks.
    #[test]
    fn join_blocks_doc_not_attributable_to_its_own_sub_area() {
        let root = TempRoot::new("join-isolation");
        let repo = TempRoot::new("join-isolation-repo");
        let base = BasePin::new("0000000000000000000000000000000000000000", "0000000");
        let schemas = join_schemas();
        let committed = crate::index::EdgeIndex::default();

        let milestone = mint_milestone(root.path(), "Cache rework", base).expect("milestone mints");
        add_task(root.path(), &milestone.id, "Solo area", "single-task").expect("solo adds");

        let solo_dir = root.path().join("tasks").join("solo-area");
        // A legitimately-staged doc: body present in docs/ AND attributed in the manifest.
        stage_doc(
            &solo_dir,
            "commit",
            "solo-area",
            "# Subject\n\nBody.\n",
            crate::state::Provenance::Created,
        );
        // A stray attribution: the manifest claims `adr:elsewhere`, but no body for it is
        // staged in this area — a doc not attributable to its own sub-area.
        let mut record = crate::state::ProvenanceRecord::load(&solo_dir).expect("load manifest");
        record.record("adr:elsewhere", crate::state::Provenance::Created);
        std::fs::write(
            crate::state::ProvenanceRecord::path_in(&solo_dir),
            record.to_bytes(),
        )
        .expect("write manifest");

        let outcome = join(
            root.path(),
            repo.path(),
            &milestone.id,
            &schemas,
            &committed,
        )
        .expect("join folds");

        // Exactly one blocking isolation finding, naming the stray doc and the area.
        let iso: Vec<&Finding> = outcome
            .findings
            .iter()
            .filter(|f| f.code == "join.area-isolation")
            .collect();
        assert_eq!(
            iso.len(),
            1,
            "the stray attribution fires exactly one isolation block: {:?}",
            outcome.findings
        );
        assert_eq!(iso[0].severity, Severity::Blocking);
        assert!(
            iso[0].route.is_some(),
            "the isolation block carries a route"
        );
        assert!(
            iso[0].message.contains("adr:elsewhere") && iso[0].message.contains("solo-area"),
            "the block names the stray doc and its area: {:?}",
            iso[0]
        );
        // The legitimately-staged doc still merges — only the stray attribution blocks.
        assert!(
            outcome.overlay.contains_key("commit:solo-area"),
            "the genuinely-staged doc is unaffected by the stray attribution"
        );
    }

    /// Regression: the isolation check keys on **physical body presence**, not on the
    /// schema-gated `task_froms`. A doc whose doctype is **absent from the resolved
    /// schema set** is skipped by [`crate::index::overlay_working`] (so it never lands in
    /// `task_froms`), yet its body **is** physically staged in this area's `docs/` and is
    /// recorded in the provenance manifest — it is genuinely attributable to its own
    /// sub-area. The join must **not** fire `join.area-isolation` for it (no over-block).
    /// Pairs with the genuine-violation control above (a manifest address with no body in
    /// its area still blocks).
    #[test]
    fn join_does_not_over_block_a_staged_doc_of_an_unknown_doctype() {
        let root = TempRoot::new("join-isolation-unknown");
        let repo = TempRoot::new("join-isolation-unknown-repo");
        let base = BasePin::new("0000000000000000000000000000000000000000", "0000000");
        let schemas = join_schemas(); // `commit` + `adr` only — `prd` is unknown here.
        let committed = crate::index::EdgeIndex::default();

        let milestone = mint_milestone(root.path(), "Cache rework", base).expect("milestone mints");
        add_task(root.path(), &milestone.id, "Solo area", "single-task").expect("solo adds");

        let solo_dir = root.path().join("tasks").join("solo-area");
        // A body of an UNKNOWN doctype, physically staged in this area's docs/ AND
        // recorded in the manifest — genuinely attributable to its own sub-area, even
        // though `prd` resolves to no schema (so `overlay_working` skips it / it never
        // reaches `task_froms`).
        stage_doc(
            &solo_dir,
            "prd",
            "solo-area",
            "# Subject\n\nBody.\n",
            crate::state::Provenance::Created,
        );

        let outcome = join(
            root.path(),
            repo.path(),
            &milestone.id,
            &schemas,
            &committed,
        )
        .expect("join folds");

        let iso: Vec<&Finding> = outcome
            .findings
            .iter()
            .filter(|f| f.code == "join.area-isolation")
            .collect();
        assert!(
            iso.is_empty(),
            "a staged body of an unknown doctype is attributable to its own area and \
             must not fire `join.area-isolation`: {:?}",
            outcome.findings
        );
    }

    /// A pure, seeded Fisher–Yates shuffle (no `rand` dependency) — a tiny
    /// SplitMix64 PRNG drives the swaps so the permutation is **reproducible**
    /// from `seed` yet genuinely scrambles the order. Used by the T5 acceptance to
    /// feed the join a third divergent order beyond id and reverse.
    fn seeded_shuffle<T>(items: &mut [T], seed: u64) {
        let mut state = seed;
        let mut next = || {
            // SplitMix64 — a well-known minimal full-period generator.
            state = state.wrapping_add(0x9E37_79B9_7F4A_7C15);
            let mut z = state;
            z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
            z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
            z ^ (z >> 31)
        };
        let n = items.len();
        for i in (1..n).rev() {
            let j = (next() % (i as u64 + 1)) as usize;
            items.swap(i, j);
        }
    }

    /// **The #1-risk Proves — permutation determinism** (Validation hardening #7;
    /// `increment-workflow.md` → A second principle (M7): determinism by
    /// re-execution; `design/worked-examples.md` flow 9 → Headline). The SAME
    /// populated, **overlapping-by-design** sub-task area set is fed to the engine
    /// join under **three divergent feed orders** — task-id order, **reverse**
    /// task-id order, and a **seed-shuffled** order — and the merged outcome (the
    /// suffix-resolved overlay **and** the ordered finding list) is asserted
    /// **byte-identical** across all three: the join is a *pure function of the set
    /// of sub-task areas*, never their iteration / completion / `read_dir` order.
    ///
    /// The fixture forces **genuine overlap** across every contention path the
    /// disjoint partition is meant to make rare (flow 9's "deliberately exercise
    /// every contention path"):
    /// - **two same-slug `created`** `adr:cache-strategy` instances, **each
    ///   self-referential** (`supersedes` its own slug) — so the task-id-ordered
    ///   suffix *and* its lockstep self-ref rewrite are exercised, and a broken
    ///   self-rewrite (or an input-order-keyed suffix) would diverge across orders;
    /// - a **cross-area ref** — one area's `supersedes` guesses a sibling's slug,
    ///   rejected by the per-area `ref_resolves` walk;
    /// - an **`edited-from-base` clash** — two areas edit the same committed-at-base
    ///   slug, a blocking `join.same-doc-clash`.
    ///
    /// The on-disk areas' raw `read_dir` order is captured and asserted to diverge
    /// from id order in this run (a green result then cannot be a `read_dir`-order
    /// accident), and the seed-shuffled order is asserted to differ from **both** id
    /// and reverse order (so it is a genuinely third condition, not a relabelled
    /// reverse). Byte-identity is asserted over the serialized `JoinOutcome` (its
    /// only accumulators are address-keyed `BTreeMap`s + a sorted finding `Vec`, so
    /// no hash-container iteration can reach the bytes).
    #[test]
    fn join_is_byte_identical_across_id_reverse_and_shuffled_feed_orders() {
        let root = TempRoot::new("join-permutation");
        let repo = TempRoot::new("join-permutation-repo");
        let base = BasePin::new("cccccccccccccccccccccccccccccccccccccccc", "ccccccc");
        let schemas = join_schemas();
        let committed = crate::index::EdgeIndex::default();

        let milestone =
            mint_milestone(root.path(), "Cache hardening", base).expect("milestone mints");

        // Five sub-tasks, **inserted in an order that is neither id nor reverse-id**
        // order, so insertion / read_dir order diverges from the canonical id order.
        // Id-sorted, the areas are:
        //   [apex-area, b-area, evict-area, low-strategy, zed-strategy]
        for intent in [
            "Zed strategy",
            "Apex area",
            "Evict area",
            "Low strategy",
            "B area",
        ] {
            add_task(root.path(), &milestone.id, intent, "single-task")
                .unwrap_or_else(|e| panic!("{intent} adds: {e:?}"));
        }

        // --- Overlap 1: two same-slug `created` `adr:cache-strategy`, each
        // self-referential (supersedes its own slug). Lower id keeps the bare slug,
        // higher takes `-2` with its self-ref rewritten in lockstep.
        stage_doc(
            &root.path().join("tasks").join("low-strategy"),
            "adr",
            "cache-strategy",
            &adr_superseding("Cache strategy", "adr:cache-strategy"),
            crate::state::Provenance::Created,
        );
        stage_doc(
            &root.path().join("tasks").join("zed-strategy"),
            "adr",
            "cache-strategy",
            &adr_superseding("Cache strategy", "adr:cache-strategy"),
            crate::state::Provenance::Created,
        );

        // --- Overlap 2: a cross-area ref. `apex-area` creates `adr:lru-eviction`;
        // `b-area` creates `adr:b-decision` whose `supersedes` GUESSES `apex-area`'s
        // slug — resolvable only inside a sibling area → blocking ref-resolves.
        stage_doc(
            &root.path().join("tasks").join("apex-area"),
            "adr",
            "lru-eviction",
            &adr_superseding("LRU eviction", "adr:lru-eviction"),
            crate::state::Provenance::Created,
        );
        stage_doc(
            &root.path().join("tasks").join("b-area"),
            "adr",
            "b-decision",
            &adr_superseding("B decision", "adr:lru-eviction"),
            crate::state::Provenance::Created,
        );

        // --- Overlap 3: an `edited-from-base` clash. `evict-area` and `b-area` both
        // edit the same committed-at-base slug `adr:eviction-policy` → same-doc clash.
        let edited_adr = "---\nstatus: accepted\ndate: 2026-06-04\n---\n\n# Eviction policy\n\n## Context\n\nForces.\n\n## Options\n\nAlternatives were weighed and rejected.\n\n## Decision\n\nDo the thing.\n\n## Consequences\n\nTradeoffs.\n";
        stage_doc(
            &root.path().join("tasks").join("evict-area"),
            "adr",
            "eviction-policy",
            edited_adr,
            crate::state::Provenance::EditedFromBase,
        );
        stage_doc(
            &root.path().join("tasks").join("b-area"),
            "adr",
            "eviction-policy",
            edited_adr,
            crate::state::Provenance::EditedFromBase,
        );

        // The canonical id-sorted enumeration the join feeds.
        let id_order = read_task_list(&milestone.dir)
            .expect("read list")
            .enumerate();
        assert_eq!(
            id_order,
            vec![
                "apex-area".to_string(),
                "b-area".to_string(),
                "evict-area".to_string(),
                "low-strategy".to_string(),
                "zed-strategy".to_string(),
            ],
            "the enumeration is the canonical id-sorted order"
        );

        // The on-disk read_dir order must diverge from id order in this run — so a
        // green result cannot be a filesystem-handed-back-id-order accident.
        let read_dir_order: Vec<String> = std::fs::read_dir(root.path().join("tasks"))
            .expect("tasks dir")
            .map(|e| e.expect("entry").file_name().to_string_lossy().into_owned())
            .collect();
        assert_ne!(
            read_dir_order, id_order,
            "the read_dir order must diverge from id order: {read_dir_order:?}"
        );

        // Order A: canonical id order.
        let order_id = id_order.clone();
        // Order B: reverse id order — mandatory (an id-ordered fixture would let a
        // completion-ordered merge pass trivially).
        let mut order_reverse = id_order.clone();
        order_reverse.reverse();
        assert_ne!(
            order_reverse, order_id,
            "reverse genuinely diverges from id order"
        );
        // Order C: a seed-shuffled order — a third divergent condition, asserted to
        // differ from BOTH id and reverse (so it is genuinely a third feed order).
        let mut order_shuffled = id_order.clone();
        seeded_shuffle(&mut order_shuffled, 42);
        assert_ne!(
            order_shuffled, order_id,
            "the shuffled order must differ from id order: {order_shuffled:?}"
        );
        assert_ne!(
            order_shuffled, order_reverse,
            "the shuffled order must differ from reverse order too: {order_shuffled:?}"
        );

        // Feed the SAME area set under each order through the engine join's fold.
        let outcome_id = fold_areas(
            root.path(),
            repo.path(),
            &milestone.id,
            &order_id,
            &schemas,
            &committed,
        )
        .expect("id-order fold");
        let outcome_reverse = fold_areas(
            root.path(),
            repo.path(),
            &milestone.id,
            &order_reverse,
            &schemas,
            &committed,
        )
        .expect("reverse-order fold");
        let outcome_shuffled = fold_areas(
            root.path(),
            repo.path(),
            &milestone.id,
            &order_shuffled,
            &schemas,
            &committed,
        )
        .expect("shuffled-order fold");

        // The overlapping fixture must actually exercise every contention path — a
        // non-overlapping fixture would prove nothing. The suffix produced a `-2`
        // instance, the cross-area ref was rejected, and the clash blocked.
        assert!(
            outcome_id.overlay.contains_key("adr:cache-strategy")
                && outcome_id.overlay.contains_key("adr:cache-strategy-2"),
            "the two same-slug `created` instances suffix-resolved: {:?}",
            outcome_id.overlay.keys().collect::<Vec<_>>()
        );
        assert!(
            outcome_id
                .findings
                .iter()
                .any(|f| f.code == "schema-conformance.ref-resolves"),
            "the cross-area ref is rejected: {:?}",
            outcome_id.findings
        );
        assert!(
            outcome_id
                .findings
                .iter()
                .any(|f| f.code == "join.same-doc-clash"),
            "the edited-from-base clash blocks: {:?}",
            outcome_id.findings
        );

        // BYTE-IDENTICAL across all three feed orders — the #1-risk Proves. Compare
        // both the structural `JoinOutcome` and its serialized bytes (the bytes are
        // the contract `finalize` will later consume).
        let bytes =
            |o: &JoinOutcome| serde_json::to_string_pretty(o).expect("JoinOutcome serializes");
        let bytes_id = bytes(&outcome_id);
        assert_eq!(
            bytes_id,
            bytes(&outcome_reverse),
            "reverse-order join is byte-identical to id-order"
        );
        assert_eq!(
            bytes_id,
            bytes(&outcome_shuffled),
            "shuffled-order join is byte-identical to id-order"
        );
        // The structural equality matches the byte equality (no Serialize-only quirk).
        assert_eq!(
            outcome_id, outcome_reverse,
            "reverse outcome equals id outcome"
        );
        assert_eq!(
            outcome_id, outcome_shuffled,
            "shuffled outcome equals id outcome"
        );

        // The public `join` (which feeds id order internally) equals the explicit
        // id-order fold — the contract surface is the one proven order-invariant.
        let public = join(
            root.path(),
            repo.path(),
            &milestone.id,
            &schemas,
            &committed,
        )
        .expect("public join folds");
        assert_eq!(
            bytes(&public),
            bytes_id,
            "the public join is byte-identical to the explicit id-order fold"
        );
    }

    /// The done-criterion for T2 (`design/storage.md` → The by-task-id join (M7):
    /// "the parent working overlay, already suffix-resolved … finalize then commits
    /// the overlay"; `DECISIONS.md` 2026-06-04 → the inc-4 fork resolution:
    /// materialize the join's suffix-rewritten doc **bodies** into the parent area —
    /// the join keeps only edges today). Over a forced-overlap fixture — two `created`
    /// colliding `adr:cache-strategy` slugs, the **higher-id** one self-referential
    /// (`supersedes` its own slug), plus a **disjoint** `commit:alpha-area`, `materialize`
    /// blocks on nothing (a collision of distinct `created` instances suffix-resolves) and
    /// writes the **bare-slug** body (lower task id) **byte-unchanged**, the **`-2`-suffixed**
    /// body (higher task id) with its **OWN** self-ref rewritten to the suffixed id
    /// `adr:cache-strategy-2` (the fixture references its own slug, so a broken self-rewrite —
    /// leaving `adr:cache-strategy`, or pointing at the sibling — fails the body assertion)
    /// renamed to `adr:cache-strategy-2.md`, and the disjoint doc unchanged — all under the
    /// parent staging `docs/` (`.jigc/milestones/<id>/merged/docs/`), the staging form
    /// `finalize`'s promote sweep already reads. The materialized bytes are a pure function
    /// of the area set (the overlay is id-sorted; net-new byte production — re-read the
    /// sub-area body + re-apply the `write::set_field` self-ref rewrite).
    #[test]
    fn materialize_writes_bare_and_suffixed_bodies_with_self_ref_rewritten() {
        let root = TempRoot::new("materialize");
        let repo = TempRoot::new("materialize-repo");
        let base = BasePin::new("dddddddddddddddddddddddddddddddddddddddd", "ddddddd");
        let schemas = join_schemas();
        let committed = crate::index::EdgeIndex::default();

        let milestone = mint_milestone(root.path(), "Cache rework", base).expect("milestone mints");

        // Inserted in reverse-id order so insertion order diverges from id order. Id-sorted:
        // [alpha-area, area-low, area-zed]. `area-low` < `area-zed`, so `area-low` keeps the
        // bare slug and `area-zed` takes the `-2` suffix.
        add_task(root.path(), &milestone.id, "Area zed", "single-task").expect("zed adds");
        add_task(root.path(), &milestone.id, "Area low", "single-task").expect("low adds");
        add_task(root.path(), &milestone.id, "Alpha area", "single-task").expect("alpha adds");

        // Two colliding `created` `adr:cache-strategy`, each self-referential.
        let low_body = adr_superseding("Cache strategy", "adr:cache-strategy");
        let zed_body = adr_superseding("Cache strategy", "adr:cache-strategy");
        stage_doc(
            &root.path().join("tasks").join("area-low"),
            "adr",
            "cache-strategy",
            &low_body,
            crate::state::Provenance::Created,
        );
        stage_doc(
            &root.path().join("tasks").join("area-zed"),
            "adr",
            "cache-strategy",
            &zed_body,
            crate::state::Provenance::Created,
        );
        // A disjoint `created` doc — no collision, body materialized unchanged.
        let disjoint_body = "# Subject\n\nBody.\n";
        stage_doc(
            &root.path().join("tasks").join("alpha-area"),
            "commit",
            "alpha-area",
            disjoint_body,
            crate::state::Provenance::Created,
        );

        let outcome = materialize(
            root.path(),
            repo.path(),
            &milestone.id,
            &schemas,
            &committed,
        )
        .expect("materialize succeeds over a non-clashing fixture");

        // The parent staging docs/ lives under the milestone's merged area.
        let merged_docs = root
            .path()
            .join("milestones")
            .join("cache-rework")
            .join("merged")
            .join("docs");
        assert_eq!(
            outcome.docs_dir, merged_docs,
            "the materialized docs/ is the parent staging area"
        );

        // The bare-slug body (lower task id) is materialized byte-UNCHANGED.
        let bare_path = merged_docs.join("adr:cache-strategy.md");
        let bare = std::fs::read_to_string(&bare_path).expect("bare body materialized");
        assert_eq!(
            bare, low_body,
            "the bare-slug body is its sub-area body unchanged"
        );

        // The `-2`-suffixed body (higher task id) has its OWN self-ref rewritten to the
        // suffixed id — never left at the bare slug, never pointing at its sibling.
        let suffixed_path = merged_docs.join("adr:cache-strategy-2.md");
        let suffixed = std::fs::read_to_string(&suffixed_path).expect("suffixed body materialized");
        assert!(
            suffixed.contains("supersedes: adr:cache-strategy-2"),
            "the suffixed body's self-ref is rewritten to the suffixed id: {suffixed:?}"
        );
        assert!(
            !suffixed.contains("supersedes: adr:cache-strategy\n"),
            "the suffixed body's self-ref is NOT left at the bare slug: {suffixed:?}"
        );
        // The rewritten edges agree: a re-derive of the materialized bytes yields the
        // suffixed self-ref edge (the bytes are the truth, not a hand-patched divergence).
        let suffixed_edges =
            crate::index::edges_from_source(&schemas["adr"], &suffixed, "adr:cache-strategy-2");
        assert_eq!(
            suffixed_edges,
            vec![crate::index::Edge {
                from: "adr:cache-strategy-2".to_string(),
                relation: "supersedes".to_string(),
                to: "adr:cache-strategy-2".to_string(),
            }],
            "the materialized suffixed body re-derives the suffixed self-ref edge"
        );

        // The disjoint doc is materialized unchanged at its own address.
        let disjoint = std::fs::read_to_string(merged_docs.join("commit:alpha-area.md"))
            .expect("disjoint body materialized");
        assert_eq!(
            disjoint, disjoint_body,
            "the disjoint doc body is unchanged"
        );

        // Exactly the three expected `.md` bodies are present (no stray files).
        let mut names: Vec<String> = std::fs::read_dir(&merged_docs)
            .expect("merged docs/")
            .map(|e| e.expect("entry").file_name().to_string_lossy().into_owned())
            .filter(|n| n.ends_with(".md"))
            .collect();
        names.sort();
        assert_eq!(
            names,
            vec![
                "adr:cache-strategy-2.md".to_string(),
                "adr:cache-strategy.md".to_string(),
                "commit:alpha-area.md".to_string(),
            ],
            "exactly the three resolved bodies are materialized"
        );
    }

    /// M45 Inc-5 T1 regression guard (`DECISIONS.md` 2026-07-23 → the Settle, Decision 3,
    /// clause d): moving `task_froms` population to *after* a successful parse (so an
    /// unparseable staged doc cannot shadow the committed edges to empty on the compose
    /// path) must NOT silently drop an unparseable-but-staged sub-area body from the
    /// by-task-id merge. The join keys materialization on the **physical body**
    /// (mirroring the isolation check at `gather_groups`), never the schema-gated,
    /// parse-gated `task_froms`. A sub-area that stages an **edge-less** `commit` doc
    /// *and* an **unparseable** known-type `adr` still materializes BOTH bodies
    /// byte-unchanged (the join copies bodies, it never parses them). Green at baseline;
    /// red after the after-parse move alone; green with the physical-body-keyed grouping.
    #[test]
    fn materialize_keeps_an_edge_less_and_an_unparseable_staged_body() {
        let root = TempRoot::new("materialize-unparseable");
        let repo = TempRoot::new("materialize-unparseable-repo");
        let base = BasePin::new("cccccccccccccccccccccccccccccccccccccccc", "ccccccc");
        let schemas = join_schemas();
        let committed = crate::index::EdgeIndex::default();

        let milestone = mint_milestone(root.path(), "Cache rework", base).expect("milestone mints");
        add_task(root.path(), &milestone.id, "Sole area", "single-task").expect("area adds");
        let area_dir = root.path().join("tasks").join("sole-area");

        // A **parseable, edge-less** `commit` doc (proves normal bodies still land) and
        // an **unparseable** known-type `adr` (a valid type, so it reaches the parse —
        // which errs on the missing required sections; proves the physical-body keying),
        // both staged in the SAME sub-area with recorded provenance.
        crate::state::provision_doc(&area_dir, &schemas["commit"], "sole-area", "Sole area", &[])
            .expect("provision the edge-less commit doc");
        let edge_less = std::fs::read_to_string(crate::state::instance_path(
            &area_dir,
            "commit",
            "sole-area",
        ))
        .expect("read the provisioned commit body");
        let unparseable = "This staged body is not a valid ADR — it has no required sections.\n";
        stage_doc(
            &area_dir,
            "adr",
            "broken",
            unparseable,
            crate::state::Provenance::Created,
        );

        let outcome = materialize(
            root.path(),
            repo.path(),
            &milestone.id,
            &schemas,
            &committed,
        )
        .expect("materialize succeeds — the unparseable body is copied, never parsed at join");

        let merged_docs = root
            .path()
            .join("milestones")
            .join("cache-rework")
            .join("merged")
            .join("docs");
        let commit_body = std::fs::read_to_string(merged_docs.join("commit:sole-area.md"))
            .expect("the edge-less body materialized");
        assert_eq!(
            commit_body, edge_less,
            "the edge-less body is byte-unchanged"
        );
        let adr_body = std::fs::read_to_string(merged_docs.join("adr:broken.md"))
            .expect("the unparseable body materialized (physical-body keyed, not task_froms)");
        assert_eq!(
            adr_body, unparseable,
            "the unparseable body is byte-unchanged"
        );

        assert_eq!(
            outcome.addresses,
            vec!["adr:broken".to_string(), "commit:sole-area".to_string()],
            "both staged bodies land in the merge audit trail: {:?}",
            outcome.addresses
        );
    }

    /// The done-criterion for T2, the clash arm (`design/storage.md` → The by-task-id join,
    /// step 3). A same-doc clash fixture — two sub-areas each `edited-from-base` the SAME
    /// committed-at-base slug `adr:eviction-policy` — makes `materialize` **block** with the
    /// routed `join.same-doc-clash` and write **nothing**: no parent `docs/` body is
    /// produced, the clashing slug never materialized (never blind-merged).
    #[test]
    fn materialize_blocks_on_same_doc_clash_and_writes_nothing() {
        let root = TempRoot::new("materialize-clash");
        let repo = TempRoot::new("materialize-clash-repo");
        let base = BasePin::new("eeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeee0", "eeeeeee");
        let schemas = join_schemas();
        let committed = crate::index::EdgeIndex::default();

        let milestone = mint_milestone(root.path(), "Cache rework", base).expect("milestone mints");
        add_task(root.path(), &milestone.id, "Purge area", "single-task").expect("purge adds");
        add_task(root.path(), &milestone.id, "Evict area", "single-task").expect("evict adds");

        let adr = "---\nstatus: accepted\ndate: 2026-06-04\n---\n\n# Eviction policy\n\n## Context\n\nForces.\n\n## Options\n\nAlternatives were weighed and rejected.\n\n## Decision\n\nDo the thing.\n\n## Consequences\n\nTradeoffs.\n";
        stage_doc(
            &root.path().join("tasks").join("evict-area"),
            "adr",
            "eviction-policy",
            adr,
            crate::state::Provenance::EditedFromBase,
        );
        stage_doc(
            &root.path().join("tasks").join("purge-area"),
            "adr",
            "eviction-policy",
            adr,
            crate::state::Provenance::EditedFromBase,
        );

        let err = materialize(
            root.path(),
            repo.path(),
            &milestone.id,
            &schemas,
            &committed,
        )
        .expect_err("a same-doc clash blocks materialize");
        assert_eq!(err.severity, Severity::Blocking);
        assert_eq!(err.code, "join.same-doc-clash");
        assert!(err.route.is_some(), "the clash block carries a route");
        assert!(
            err.message.contains("adr:eviction-policy"),
            "the block names the clashing slug: {err:?}"
        );

        // NOTHING materialized: the parent staging docs/ holds no body (and need not exist).
        let merged_docs = root
            .path()
            .join("milestones")
            .join("cache-rework")
            .join("merged")
            .join("docs");
        let bodies = std::fs::read_dir(&merged_docs)
            .map(|rd| {
                rd.flatten()
                    .filter(|e| e.path().extension().and_then(|x| x.to_str()) == Some("md"))
                    .count()
            })
            .unwrap_or(0);
        assert_eq!(bodies, 0, "a blocked materialize writes no body");
    }

    /// A title that normalizes to nothing falls back to the `milestone` type
    /// name (the same fallback discipline as `state::mint_id`).
    #[test]
    fn empty_title_falls_back_to_type_name() {
        let root = TempRoot::new("fallback");
        let base = BasePin::new("a".repeat(40), "aaaaaaa");

        let minted =
            mint_milestone(root.path(), "!!!___---", base).expect("fallback mint succeeds");
        assert_eq!(
            minted.id, "milestone",
            "stripped-to-empty title uses the type name"
        );
        assert!(root.path().join("milestones").join("milestone").is_dir());
    }

    /// T1 done-criterion (`DECISIONS.md` 2026-06-04 → the inc-4 fork resolution;
    /// `finalize.md` → `fan-out` finalize): the milestone-finalize commit message
    /// is a **pure structural projection** of the milestone id + its
    /// **id-ordered** sub-task list — no authored prose, golden-locked. A
    /// `TaskList` carrying ≥2 sub-tasks renders the expected golden string; the
    /// **same task set** under two divergent insertion orders (incl. the reverse)
    /// renders a **byte-identical** message, since the projection sorts at the
    /// boundary ([`TaskList::enumerate`]), never reading insertion order
    /// (Validation hardening #7).
    #[test]
    fn synthesized_message_is_a_byte_stable_id_ordered_projection() {
        // Insertion order: reverse of id order.
        let forward = TaskList {
            tasks: vec![
                "zebra-fix".to_string(),
                "alpha-fix".to_string(),
                "mid-fix".to_string(),
            ],
        };
        // The reverse insertion order over the same id set.
        let reverse = TaskList {
            tasks: vec![
                "mid-fix".to_string(),
                "alpha-fix".to_string(),
                "zebra-fix".to_string(),
            ],
        };

        let msg = synthesized_message("cache-hardening", &forward);

        // Golden: subject names the milestone + count; one id-ordered line per
        // sub-task; the body is id-sorted, never insertion order.
        let expected = "\
Finalize milestone cache-hardening (3 sub-tasks)

- alpha-fix
- mid-fix
- zebra-fix
";
        assert_eq!(
            msg, expected,
            "the synthesized message is the golden projection"
        );

        // The reverse insertion order yields the byte-identical message — the
        // id-sort at the boundary, hardening #7 (reverse is mandatory: an
        // id-ordered insertion would make insertion order trivially equal id order
        // and hide an insertion-order leak).
        assert_eq!(
            synthesized_message("cache-hardening", &reverse),
            msg,
            "the message is byte-identical across divergent insertion orders"
        );
    }

    /// Load the **shipped** `milestone-record` schema from the methodology pack
    /// tree (`packs/methodology/schemas/milestone-record.yaml`, `../../` off the
    /// engine crate root) — so the golden pins the create arm against the real
    /// pack bytes, not an inlined stand-in. The schema-version stamp is injected
    /// exactly as the production load does (`load_pack_schema`: milestone-record is
    /// manifest-frozen since M40 A1), so the goldens pin the stamped shape.
    fn milestone_record_schema() -> crate::schema::Schema {
        let path = Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("..")
            .join("..")
            .join("packs")
            .join("methodology")
            .join("schemas")
            .join("milestone-record.yaml");
        let bytes = std::fs::read(&path).expect("read the shipped milestone-record schema");
        let mut schema =
            crate::schema::load_schema(&bytes).expect("the shipped milestone-record schema loads");
        crate::schema::inject_schema_version_stamp(&mut schema);
        schema
    }

    /// T1 done-criterion (`design/team-ready-state.md` → The milestone-record
    /// doctype; Engine capability 1 (write)): [`render_fresh_record`] materializes a
    /// fresh record from a [`BasePin`] — the `meta` header carries the `base` SHA +
    /// `status: active`, the H1 is the milestone id, and the `tasks` section is an
    /// **empty repeatable** — as **byte-golden** output that **re-parses** via
    /// [`crate::parse::parse_sections`]. This proves the novel all-machine-set
    /// header + empty-repeatable shape both materializes and round-trips (the
    /// claim-as-red: no shipped doctype expresses this shape today).
    #[test]
    fn fresh_milestone_record_is_byte_golden_and_reparses() {
        let schema = milestone_record_schema();
        let base = BasePin {
            sha: "1f2e3d4c5b6a7980a1b2c3d4e5f60718293a4b5c".to_string(),
            short: "1f2e3d4".to_string(),
        };

        let body = render_fresh_record(&schema, "cache-rework", &base, 1);

        // Golden: the `meta` front-matter (base SHA + seeded-active status), the
        // `# cache-rework` H1, and an EMPTY `## Tasks` repeatable — one trailing LF.
        let expected = "\
---
base: 1f2e3d4c5b6a7980a1b2c3d4e5f60718293a4b5c 1f2e3d4
status: active
schema-version: 1
---

# cache-rework

## Tasks
";
        assert_eq!(body, expected, "the fresh record is the golden byte form");

        // Re-parses via the schema-driven parse path: the header fields read back
        // opaque, and the repeatable `tasks` section carries zero items.
        let doc = crate::parse::parse_sections(&schema, &body)
            .expect("the fresh record re-parses against its schema");

        let header = doc
            .sections
            .iter()
            .find(|s| s.id == RECORD_HEADER_SECTION)
            .expect("the parsed doc carries the `meta` header section");
        let field = |key: &str| {
            header
                .fields
                .iter()
                .find(|f| f.key == key)
                .map(|f| f.value.render())
        };
        // The `base` field carries BOTH the full SHA and the short (space-joined), so
        // the record round-trips a `BasePin` losslessly on a fresh-clone re-derive.
        assert_eq!(
            field(RECORD_BASE_FIELD),
            Some(format!("{} {}", base.sha, base.short))
        );
        assert_eq!(
            field(RECORD_STATUS_FIELD).as_deref(),
            Some(RECORD_STATUS_ACTIVE)
        );

        let tasks = doc
            .sections
            .iter()
            .find(|s| s.id == RECORD_TASKS_SECTION)
            .expect("the parsed doc carries the `tasks` section");
        assert!(
            tasks.items.is_empty(),
            "a freshly materialized record stages no sub-task items"
        );
    }

    /// T2 done-criterion (`design/team-ready-state.md` → Engine capability 1 (write),
    /// the `add-task` — append arm; M39 Increment 3): [`append_task_item`] appends one
    /// `tasks` item per sub-task over the fresh record — `task-id`/`intent`/`workflow`/
    /// `status: active` — **byte-stable** (the bytes outside each appended item's span are
    /// byte-identical: the append never disturbs the `meta` header, the H1, the earlier
    /// item, or the `## Tasks` heading) and the twice-appended record **re-parses** with
    /// both items in append order carrying their machine-set values.
    #[test]
    fn add_task_appends_both_sub_tasks_byte_stable_and_reparses() {
        let schema = milestone_record_schema();
        let base = BasePin {
            sha: "1f2e3d4c5b6a7980a1b2c3d4e5f60718293a4b5c".to_string(),
            short: "1f2e3d4".to_string(),
        };
        let fresh = render_fresh_record(&schema, "cache-rework", &base, 1);

        // Append the first sub-task, then the second — each via the `add-task` append arm.
        let after_one = append_task_item(
            &schema,
            &fresh,
            "warm-cache",
            "Warm the read cache",
            "sub-task",
        )
        .expect("first sub-task appends");
        let after_two = append_task_item(
            &schema,
            &after_one,
            "evict-cold",
            "Evict cold entries",
            "sub-task",
        )
        .expect("second sub-task appends");

        // Golden: both items, in append order, under the untouched `meta`/H1/`## Tasks`.
        let expected = "\
---
base: 1f2e3d4c5b6a7980a1b2c3d4e5f60718293a4b5c 1f2e3d4
status: active
schema-version: 1
---

# cache-rework

## Tasks

### warm-cache  {#warm-cache}

<!-- fields -->
- intent: Warm the read cache
- workflow: sub-task
- status: active

### evict-cold  {#evict-cold}

<!-- fields -->
- intent: Evict cold entries
- workflow: sub-task
- status: active
";
        assert_eq!(
            after_two, expected,
            "the twice-appended record is the golden form"
        );

        // Byte-stability: the append is confined to the new item's span — every byte of
        // the prior record (header, H1, `## Tasks`, the first item) survives byte-identical.
        assert!(
            after_two.starts_with(after_one.trim_end_matches('\n')),
            "the second append leaves the first record's bytes untouched outside the appended span:\n\
             --- after_one ---\n{after_one}\n--- after_two ---\n{after_two}"
        );

        // Re-parses: the `tasks` section carries both items in append order, each with its
        // machine-set `task-id` (heading/id), `intent`, and `status: active`.
        let doc = crate::parse::parse_sections(&schema, &after_two)
            .expect("the twice-appended record re-parses against its schema");
        let tasks = doc
            .sections
            .iter()
            .find(|s| s.id == RECORD_TASKS_SECTION)
            .expect("the parsed doc carries the `tasks` section");
        #[allow(clippy::type_complexity)]
        let seen: Vec<(&str, Option<String>, Option<String>, Option<String>)> = tasks
            .items
            .iter()
            .map(|item| {
                let leaf = |key: &str| {
                    item.fields
                        .iter()
                        .find(|f| f.key == key)
                        .map(|f| f.value.render())
                };
                (
                    item.title.as_str(),
                    leaf(RECORD_TASK_INTENT_FIELD),
                    leaf(RECORD_TASK_WORKFLOW_FIELD),
                    leaf(RECORD_STATUS_FIELD),
                )
            })
            .collect();
        assert_eq!(
            seen,
            vec![
                (
                    "warm-cache",
                    Some("Warm the read cache".to_string()),
                    Some("sub-task".to_string()),
                    Some(RECORD_STATUS_ACTIVE.to_string()),
                ),
                (
                    "evict-cold",
                    Some("Evict cold entries".to_string()),
                    Some("sub-task".to_string()),
                    Some(RECORD_STATUS_ACTIVE.to_string()),
                ),
            ],
            "both sub-tasks re-parse in append order with their machine-set values"
        );
    }

    /// T3 done-criterion (`design/team-ready-state.md` → Engine capability 1 (write),
    /// the `join` — in-place mutate arm; M39 Increment 3): over a committed record
    /// (create → append ×2), [`join_record`] reads the record **directly from disk**,
    /// flips **every** committed `tasks` item's `status` AND the header `status` to
    /// `joined`, and writes it back — the doc **byte-identical modulo exactly the
    /// flipped status values** (the byte-stability red obligation) and re-parsing with
    /// every status now `joined`.
    #[test]
    fn join_flips_every_item_and_header_status_byte_stable() {
        let schema = milestone_record_schema();
        let base = BasePin {
            sha: "1f2e3d4c5b6a7980a1b2c3d4e5f60718293a4b5c".to_string(),
            short: "1f2e3d4".to_string(),
        };
        let fresh = render_fresh_record(&schema, "cache-rework", &base, 1);
        let after_one = append_task_item(
            &schema,
            &fresh,
            "warm-cache",
            "Warm the read cache",
            "sub-task",
        )
        .expect("first sub-task appends");
        let committed = append_task_item(
            &schema,
            &after_one,
            "evict-cold",
            "Evict cold entries",
            "sub-task",
        )
        .expect("second sub-task appends");

        // The committed record on disk — the join arm writes it directly (net-new
        // plumbing vs. the task-scoped author buffer path).
        let root = TempRoot::new("join");
        let record_path = root.path().join("cache-rework.md");
        std::fs::write(&record_path, &committed).expect("stage the committed record");

        let joined =
            join_record(&record_path, &schema, "cache-rework").expect("the join flips the record");

        // The write hit disk: the file bytes ARE the returned bytes.
        assert_eq!(
            std::fs::read_to_string(&record_path).expect("read back the joined record"),
            joined,
            "the join arm writes the committed record directly"
        );

        // Byte-stability — the red obligation: the joined record is byte-identical to the
        // committed one modulo EXACTLY the flipped `status` values. `active`/`joined` are
        // the same length, but nothing is assumed — every `status: active` (the header +
        // both items) becomes `status: joined`, and no other byte moves.
        assert_eq!(
            joined,
            committed.replace("status: active", "status: joined"),
            "the join flips only the status values; every other byte survives byte-identical"
        );
        assert!(
            !joined.contains("status: active"),
            "no committed status survives un-flipped after the join"
        );

        // Re-parses: the header and both items carry `status: joined`.
        let doc = crate::parse::parse_sections(&schema, &joined)
            .expect("the joined record re-parses against its schema");
        let header = doc
            .sections
            .iter()
            .find(|s| s.id == RECORD_HEADER_SECTION)
            .expect("the joined record carries the `meta` header");
        assert_eq!(
            header
                .fields
                .iter()
                .find(|f| f.key == RECORD_STATUS_FIELD)
                .map(|f| f.value.render())
                .as_deref(),
            Some(RECORD_STATUS_JOINED),
            "the header status flipped to joined"
        );
        let tasks = doc
            .sections
            .iter()
            .find(|s| s.id == RECORD_TASKS_SECTION)
            .expect("the joined record carries the `tasks` section");
        assert_eq!(tasks.items.len(), 2, "both sub-tasks survive the join");
        for item in &tasks.items {
            assert_eq!(
                item.fields
                    .iter()
                    .find(|f| f.key == RECORD_STATUS_FIELD)
                    .map(|f| f.value.render())
                    .as_deref(),
                Some(RECORD_STATUS_JOINED),
                "sub-task `{}` status flipped to joined",
                item.title
            );
        }
    }

    /// A committed record whose sub-task items carry the given statuses — the fixture
    /// the discard arm's per-item semantics need (the `join` arm can only produce
    /// all-`joined`). Builds it through the production write arms (create → append ×N)
    /// and then flips exactly the items that should read `joined` through the same
    /// byte-stable item-leaf splice the arms use, so the fixture bytes are the bytes a
    /// real partially-joined record would carry.
    fn record_with_item_statuses(
        schema: &crate::schema::Schema,
        milestone_id: &str,
        tasks: &[(&str, &str, &str)],
    ) -> String {
        let base = BasePin {
            sha: "1f2e3d4c5b6a7980a1b2c3d4e5f60718293a4b5c".to_string(),
            short: "1f2e3d4".to_string(),
        };
        let mut body = render_fresh_record(schema, milestone_id, &base, 1);
        for (task_id, intent, _) in tasks {
            body = append_task_item(schema, &body, task_id, intent, "sub-task")
                .expect("the sub-task appends");
        }
        for (task_id, _, status) in tasks {
            if *status != RECORD_STATUS_ACTIVE {
                body = crate::write::set_item_field(
                    schema,
                    &body,
                    RECORD_TASKS_SECTION,
                    task_id,
                    RECORD_STATUS_FIELD,
                    status,
                )
                .expect("the fixture item status splices");
            }
        }
        body
    }

    /// The record's `status` leaves, in document order: the header first, then one per
    /// `tasks` item (each paired with its item id) — read back through the parser, so
    /// the assertion reads what a consumer of the committed record would read.
    fn record_statuses(
        schema: &crate::schema::Schema,
        source: &str,
    ) -> (Option<String>, Vec<(String, String)>) {
        let doc = crate::parse::parse_sections(schema, source)
            .expect("the record re-parses against its schema");
        let header = doc
            .sections
            .iter()
            .find(|s| s.id == RECORD_HEADER_SECTION)
            .and_then(|s| s.fields.iter().find(|f| f.key == RECORD_STATUS_FIELD))
            .map(|f| f.value.render());
        let items = doc
            .sections
            .iter()
            .find(|s| s.id == RECORD_TASKS_SECTION)
            .map(|s| {
                s.items
                    .iter()
                    .map(|i| {
                        (
                            i.id.clone(),
                            i.fields
                                .iter()
                                .find(|f| f.key == RECORD_STATUS_FIELD)
                                .map(|f| f.value.render())
                                .expect("every sub-task item carries a status leaf"),
                        )
                    })
                    .collect()
            })
            .unwrap_or_default();
        (header, items)
    }

    /// T4 done-criterion (`design/team-ready-state.md` → `jigc milestone discard <id>`,
    /// "Per-item semantics — a joined sub-task stays joined"; M42 Increment 7): over a
    /// committed record whose items are `[joined, active]`, [`discard_record`] flips the
    /// header **and the non-joined item** to `discarded` and leaves the **genuinely
    /// joined** item's bytes **IDENTICAL** — it really did land, and flipping it would
    /// make the record lie about landed work.
    ///
    /// The red the `join` sibling would fail: its unconditional loop flips *every* item,
    /// so it would rewrite the joined sub-task too.
    #[test]
    fn discard_flips_non_joined_items_and_header_leaving_a_joined_item_byte_identical() {
        let schema = milestone_record_schema();
        let committed = record_with_item_statuses(
            &schema,
            "cache-rework",
            &[
                ("warm-cache", "Warm the read cache", RECORD_STATUS_JOINED),
                ("evict-cold", "Evict cold entries", RECORD_STATUS_ACTIVE),
            ],
        );

        // The committed record on disk — discard writes it directly, exactly as join does.
        let root = TempRoot::new("discard");
        let record_path = root.path().join("cache-rework.md");
        std::fs::write(&record_path, &committed).expect("stage the committed record");

        let discarded = discard_record(&record_path, &schema, "cache-rework")
            .expect("the discard settles the record");

        // The write hit disk: the file bytes ARE the returned bytes.
        assert_eq!(
            std::fs::read_to_string(&record_path).expect("read back the discarded record"),
            discarded,
            "the discard arm writes the committed record directly"
        );

        // Per-item semantics + byte-stability in one assertion: the settled record is the
        // committed one with EXACTLY the non-joined status values flipped. Every
        // `status: joined` byte survives untouched (there is one — the landed sub-task),
        // so the joined item's bytes are IDENTICAL.
        assert_eq!(
            discarded,
            committed.replace("status: active", "status: discarded"),
            "discard flips only the non-joined status values; every other byte survives"
        );
        assert_eq!(
            discarded.matches("status: joined").count(),
            1,
            "the genuinely joined sub-task still reads `joined` after the discard"
        );

        // Read back through the parser: header discarded, the joined item still joined,
        // the active item discarded.
        let (header, items) = record_statuses(&schema, &discarded);
        assert_eq!(
            header.as_deref(),
            Some(RECORD_STATUS_DISCARDED),
            "the header status settles to discarded"
        );
        assert_eq!(
            items,
            vec![
                ("warm-cache".to_string(), RECORD_STATUS_JOINED.to_string()),
                (
                    "evict-cold".to_string(),
                    RECORD_STATUS_DISCARDED.to_string()
                ),
            ],
            "a genuinely joined sub-task stays joined; every non-joined one is discarded"
        );

        // Byte-stability of the in-place rewrite: `render(parse(out)) == out`.
        let instance = crate::write::instance_from_source(&schema, &discarded)
            .expect("the discarded record re-parses into an instance");
        assert_eq!(
            crate::write::render(&schema, &instance),
            discarded,
            "the discarded record is byte-stable: render(parse(x)) == x"
        );
    }

    /// T4 done-criterion (same design section): an **all-joined** record — every
    /// sub-task landed before the milestone was abandoned — flips the **header alone**;
    /// not one item byte moves.
    #[test]
    fn discard_of_an_all_joined_record_flips_the_header_alone() {
        let schema = milestone_record_schema();
        let committed = record_with_item_statuses(
            &schema,
            "cache-rework",
            &[
                ("warm-cache", "Warm the read cache", RECORD_STATUS_JOINED),
                ("evict-cold", "Evict cold entries", RECORD_STATUS_JOINED),
            ],
        );

        let root = TempRoot::new("discard-all-joined");
        let record_path = root.path().join("cache-rework.md");
        std::fs::write(&record_path, &committed).expect("stage the committed record");

        let discarded = discard_record(&record_path, &schema, "cache-rework")
            .expect("the discard settles the record");

        // The header is the ONLY `status: active` in an all-joined record, so this is the
        // header-alone flip stated as bytes: both items survive byte-identical.
        assert_eq!(
            discarded,
            committed.replace("status: active", "status: discarded"),
            "an all-joined record flips the header alone; not one item byte moves"
        );
        let (header, items) = record_statuses(&schema, &discarded);
        assert_eq!(header.as_deref(), Some(RECORD_STATUS_DISCARDED));
        assert!(
            items.iter().all(|(_, s)| s == RECORD_STATUS_JOINED),
            "every landed sub-task still reads joined: {items:?}"
        );

        let instance = crate::write::instance_from_source(&schema, &discarded)
            .expect("the discarded record re-parses into an instance");
        assert_eq!(
            crate::write::render(&schema, &instance),
            discarded,
            "the discarded record is byte-stable: render(parse(x)) == x"
        );
    }

    /// T4 done-criterion (`design/team-ready-state.md` → Engine capability 2
    /// (read-back); M39 Increment 3): the committed record is the engine's source of
    /// truth — deleting the `.jigc` JSON cache and re-deriving from the record yields
    /// **byte-identical operational state**. Builds the cache (mint then add-task
    /// twice), captures the `BasePin`/`TaskList` it holds, builds the matching record
    /// (create then append twice), then reads the record back into the same
    /// `(BasePin, TaskList)` and — after deleting the JSON — re-seeds it from the
    /// record; both paths must yield an **identical `BasePin` (sha AND short)** and
    /// `TaskList::enumerate()`.
    ///
    /// The short is **asserted equal, not trusted to be a sha prefix**: the fixture
    /// short is *not* the git-default 7-char prefix, so a sha-only record (or a naive
    /// `sha[..7]` reconstruction) is lossy — the base field stores BOTH sha and short,
    /// and this test is what proves the round-trip is lossless.
    #[test]
    fn record_re_derives_base_pin_and_task_list_after_cache_delete() {
        let schema = milestone_record_schema();
        let root = TempRoot::new("read-back");
        // A short that is NOT the git-default 7-char prefix (8 chars) — so trusting
        // `short = sha[..7]` would reconstruct the WRONG short; the round-trip must
        // carry it explicitly.
        let base = BasePin {
            sha: "1f2e3d4c5b6a7980a1b2c3d4e5f60718293a4b5c".to_string(),
            short: "1f2e3d4c".to_string(),
        };
        assert_ne!(
            &base.sha[..7],
            base.short.as_str(),
            "the fixture short is deliberately not the default 7-char prefix"
        );

        // 1. Build the JSON cache: mint + two sub-tasks. Capture what the cache holds.
        let minted =
            mint_milestone(root.path(), "Cache rework", base.clone()).expect("mint milestone");
        let a = add_task(
            root.path(),
            &minted.id,
            "Warm the read cache",
            "single-task",
        )
        .expect("add sub-task 1");
        let b = add_task(root.path(), &minted.id, "Evict cold entries", "single-task")
            .expect("add sub-task 2");
        let cache_base = read_base_pin(&minted.dir).expect("read cache base");
        let cache_tasks = read_task_list(&minted.dir).expect("read cache tasks");

        // 2. Build the matching record: create + append the two minted sub-task ids.
        let fresh = render_fresh_record(&schema, &minted.id, &base, 1);
        let r1 = append_task_item(
            &schema,
            &fresh,
            &a.task.id,
            "Warm the read cache",
            "sub-task",
        )
        .expect("append sub-task 1");
        let record = append_task_item(&schema, &r1, &b.task.id, "Evict cold entries", "sub-task")
            .expect("append sub-task 2");

        // 3. Read-back parses the record into the SAME (BasePin, TaskList) — sha AND
        //    short. Asserting BasePin equality (not just sha) is the round-trip claim.
        let (rb_base, rb_tasks) = read_back_record(&schema, &record).expect("read back the record");
        assert_eq!(
            rb_base, cache_base,
            "the re-derived BasePin equals the cache's — sha AND short"
        );
        assert_eq!(
            rb_base.short, cache_base.short,
            "the short round-trips from the record, not re-derived as a sha prefix"
        );
        assert_eq!(
            rb_tasks.enumerate(),
            cache_tasks.enumerate(),
            "the re-derived task set equals the cache's"
        );

        // 4. Re-seed path: delete the JSON cache, re-derive it from the record, and the
        //    JSON readers return byte-identical operational state (the fresh-clone
        //    rebuild — the demoted cache is genuinely rebuilt from the `.md` record).
        std::fs::remove_file(minted.dir.join(BASE_PIN_FILE)).expect("delete base.json");
        std::fs::remove_file(minted.dir.join(TASKS_FILE)).expect("delete tasks.json");
        reseed_cache_from_record(&minted.dir, &schema, &record).expect("re-seed the cache");
        assert_eq!(
            read_base_pin(&minted.dir).expect("re-seeded base reads back"),
            cache_base,
            "the re-seeded base.json equals the original (sha AND short)"
        );
        assert_eq!(
            read_task_list(&minted.dir)
                .expect("re-seeded tasks reads back")
                .enumerate(),
            cache_tasks.enumerate(),
            "the re-seeded tasks.json enumerates identically"
        );
    }

    /// **The terminal predicate** (`design/team-ready-state.md` → The lifecycle — a terminal
    /// record has no workbench; M42 completion-audit HIGH): a record whose header `status` has
    /// settled — `discarded` **or** `joined` — **does not re-seed a workbench**. The reseed is
    /// the fresh-clone continuation path, and a settled milestone is not continuable: rebuilding
    /// its cache resurrects the very `.jigc/milestones/<id>/` the terminal ops tore down, from
    /// which `provision` re-provisions worktrees at the settled base and `add-task` appends an
    /// `active` sub-task to a settled record.
    ///
    /// Red before the fix: [`reseed_cache_from_record`] consulted only the two cache files and
    /// never the record's `status`, so it rebuilt the cache from a **discarded** record just as
    /// happily as from a live one — and the same for `joined` (the sibling terminal `milestone
    /// finalize` writes; the identical hole, not a second special case).
    ///
    /// **Both cache states** are asserted, because the refusal is a property of the *record*, not
    /// of the cache's absence: an interrupted teardown (record committed, `remove_dir_all` never
    /// reached) leaves a live cache under a settled record, and the verbs must refuse there too.
    #[test]
    fn a_terminal_record_never_reseeds_a_workbench() {
        let schema = milestone_record_schema();
        let active = record_with_item_statuses(
            &schema,
            "cache-rework",
            &[("warm-cache", "Warm the read cache", RECORD_STATUS_ACTIVE)],
        );

        for terminal in [RECORD_STATUS_DISCARDED, RECORD_STATUS_JOINED] {
            let settled = crate::write::set_field(
                &schema,
                &active,
                RECORD_HEADER_SECTION,
                RECORD_STATUS_FIELD,
                terminal,
            )
            .expect("the fixture header status splices");
            assert_eq!(
                terminal_status(&schema, &settled).as_deref(),
                Some(terminal),
                "the fixture record reads as settled at `{terminal}`"
            );
            assert_eq!(
                terminal_status(&schema, &active),
                None,
                "a live record is not terminal"
            );

            // (a) The fresh-clone shape: no cache at all. The reseed REFUSES and writes nothing —
            // the workbench the terminal op tore down stays torn down.
            let root = TempRoot::new(&format!("terminal-{terminal}"));
            let dir = root.path().join("milestones").join("cache-rework");
            let finding = reseed_cache_from_record(&dir, &schema, &settled)
                .expect_err("a settled record must refuse to re-seed a workbench");
            assert_eq!(finding.code, "milestone.terminal");
            assert_eq!(finding.severity, Severity::Blocking);
            assert!(
                finding.message.contains("cache-rework") && finding.message.contains(terminal),
                "the refusal names the milestone and its terminal: {}",
                finding.message,
            );
            assert!(
                finding
                    .route
                    .as_deref()
                    .is_some_and(|r| r.contains("jigc doc show milestone-record:cache-rework")),
                "the refusal routes to the committed record's read surface: {:?}",
                finding.route,
            );
            assert!(
                !dir.join(BASE_PIN_FILE).exists() && !dir.join(TASKS_FILE).exists(),
                "the refused reseed writes NO cache file — the workbench is not resurrected",
            );

            // (b) The interrupted-teardown shape: a live cache under a settled record. The
            // refusal is a property of the RECORD, so it fires here too (the cache-present
            // early-return must not slip past the terminal).
            std::fs::create_dir_all(&dir).expect("stage the leftover cache dir");
            std::fs::write(
                dir.join(BASE_PIN_FILE),
                render_base_pin(&BasePin {
                    sha: "1f2e3d4c5b6a7980a1b2c3d4e5f60718293a4b5c".to_string(),
                    short: "1f2e3d4".to_string(),
                }),
            )
            .expect("stage base.json");
            std::fs::write(dir.join(TASKS_FILE), TaskList::default().to_bytes())
                .expect("stage tasks.json");
            let finding = reseed_cache_from_record(&dir, &schema, &settled)
                .expect_err("a settled record refuses even when a stale cache survived");
            assert_eq!(finding.code, "milestone.terminal");
        }
    }

    /// **The shared-workbench atomicity axis** (M46 completion-audit F4): every writer of a
    /// file in the *gitignored, shared* `.jigc/` workbench that another door **parses** must
    /// replace it atomically (temp + `rename`, [`crate::state::persist`]) — never truncate it
    /// in place.
    ///
    /// The axis is derivable, not hand-picked: `.jigc/` splits into a **committed** config
    /// surface (`config/`, `AGENT.md`, `version`) and a **gitignored** workbench (`index/`,
    /// `state/`, `milestones/`, `worktrees/`, `tasks/` — `design/storage.md` → the `.jigc`
    /// layout), and `tasks/<id>/` is task-isolated single-writer by construction. What remains
    /// is the shared workbench, whose parsed state files are `state/file-state.json`
    /// (merge + lock + persist), `index/edges.json` (lock + persist), and this module's
    /// `milestones/<id>/{tasks,base}.json`.
    ///
    /// [`TASKS_FILE`]'s doc-comment stated the hazard and converted `tasks.json`'s writers —
    /// and left the **co-located `base.json` unconverted in two of the same functions**, one
    /// line above a converted `persist` in each. `base.json` is read by [`read_base_pin`] from
    /// every milestone door, so the truncation window that comment names was live on it.
    ///
    /// The witness is the write's **mechanism**, deterministically: a truncating
    /// `std::fs::write` refills the existing inode, while temp + `rename` swaps a new one in.
    /// So `ino` **must change** across the write — that is exactly the property that makes a
    /// concurrent reader see either the whole old file or the whole new one, and never a
    /// zero-length window. The table iterates every writer of a shared-area cache file that
    /// overwrites an existing target; [`mint_milestone`]'s fresh-create half is covered by
    /// [`a_concurrent_reader_never_parses_a_half_written_shared_cache`].
    #[test]
    fn shared_area_writers_replace_rather_than_truncate() {
        use std::os::unix::fs::MetadataExt;

        let schema = milestone_record_schema();
        let base = BasePin {
            sha: "1f2e3d4c5b6a7980a1b2c3d4e5f60718293a4b5c".to_string(),
            short: "1f2e3d4".to_string(),
        };

        // Each arm: (what it writes, the writer door, the target file, the parsing reader).
        type Arm = (
            &'static str,
            &'static str,
            fn(&Path, &str, &crate::schema::Schema, &str) -> PathBuf,
        );
        let arms: &[Arm] = &[
            (
                "base.json ← reseed_cache_from_record",
                BASE_PIN_FILE,
                |jigc_root, id, schema, record| {
                    let dir = milestone_dir(jigc_root, id);
                    // Defeat the both-present early return so the re-seed actually rewrites
                    // the base pin over the file already on disk.
                    std::fs::remove_file(dir.join(TASKS_FILE)).expect("clear the task list");
                    reseed_cache_from_record(&dir, schema, record).expect("re-seed the cache");
                    dir.join(BASE_PIN_FILE)
                },
            ),
            (
                "tasks.json ← reseed_cache_from_record",
                TASKS_FILE,
                |jigc_root, id, schema, record| {
                    let dir = milestone_dir(jigc_root, id);
                    std::fs::remove_file(dir.join(BASE_PIN_FILE)).expect("clear the base pin");
                    reseed_cache_from_record(&dir, schema, record).expect("re-seed the cache");
                    dir.join(TASKS_FILE)
                },
            ),
            (
                "tasks.json ← add_task",
                TASKS_FILE,
                |jigc_root, id, _schema, _record| {
                    add_task(jigc_root, id, "Warm the read cache", "single-task")
                        .expect("add a sub-task");
                    milestone_dir(jigc_root, id).join(TASKS_FILE)
                },
            ),
            (
                "tasks.json ← drop_sub_tasks",
                TASKS_FILE,
                |jigc_root, id, _schema, _record| {
                    let added = add_task(jigc_root, id, "Evict cold entries", "single-task")
                        .expect("add a sub-task");
                    drop_sub_tasks(jigc_root, id, &[added.task.id]).expect("drop the sub-task");
                    milestone_dir(jigc_root, id).join(TASKS_FILE)
                },
            ),
        ];

        for (label, _file, write) in arms {
            let root = TempRoot::new("atomic-shared-area");
            let minted = mint_milestone(root.path(), "Cache rework", base.clone())
                .expect("mint the milestone");
            let record = render_fresh_record(&schema, &minted.id, &base, 1);

            let target = minted.dir.join(_file);
            let before = std::fs::metadata(&target)
                .unwrap_or_else(|err| panic!("{label}: the target must exist first: {err}"))
                .ino();

            let written = write(root.path(), &minted.id, &schema, &record);
            assert_eq!(written, target, "{label}: the arm names its own target");

            let after = std::fs::metadata(&target)
                .unwrap_or_else(|err| panic!("{label}: the target survives the write: {err}"))
                .ino();
            assert_ne!(
                before, after,
                "{label}: the write must REPLACE the shared-area file (temp + rename), not \
                 truncate-and-refill the inode a concurrent reader may already have open",
            );

            // The replacement is also a correct one — the reader parses what landed.
            if *_file == BASE_PIN_FILE {
                read_base_pin(&minted.dir).expect("the replaced base pin parses");
            } else {
                read_task_list(&minted.dir).expect("the replaced task list parses");
            }
        }
    }

    /// The **consequence** the axis above exists to prevent, driven through a real door:
    /// while [`mint_milestone`] writes a milestone area, a concurrent [`read_base_pin`] must
    /// never observe bytes that do not parse.
    ///
    /// This is the shape [`TASKS_FILE`]'s doc-comment records as *measured* — the real
    /// `add_task` door raising `milestone.area-io`, *"EOF while parsing a value at line 1
    /// column 0"* — turned on `base.json`, whose two writers the same increment left
    /// truncating. A plain `std::fs::write` makes the path **exist before it has content**, so
    /// a reader that opens in that window reads zero bytes and fails `InvalidData`. Under
    /// temp + `rename` the path only ever appears complete.
    ///
    /// `NotFound` is a legitimate observation (the area is torn down between rounds) and is
    /// **not** counted; only a file that exists and does not parse is the defect. The loop is
    /// a race, so its *red* is probabilistic while its *green* is a guarantee (`rename` is
    /// atomic) — which is why the deterministic inode witness above is the standing fence and
    /// this test is the demonstration beside it.
    #[test]
    fn a_concurrent_reader_never_parses_a_half_written_shared_cache() {
        use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};

        let root = TempRoot::new("half-written-cache");
        let jigc_root = root.path().to_path_buf();
        let base = BasePin {
            sha: "1f2e3d4c5b6a7980a1b2c3d4e5f60718293a4b5c".to_string(),
            short: "1f2e3d4".to_string(),
        };
        let dir = milestone_dir(&jigc_root, &mint_id("Cache rework"));

        let done = AtomicBool::new(false);
        let torn = AtomicUsize::new(0);
        let reads = AtomicUsize::new(0);

        std::thread::scope(|scope| {
            let reader_dir = dir.clone();
            let (done_r, torn_r, reads_r) = (&done, &torn, &reads);
            scope.spawn(move || {
                while !done_r.load(Ordering::Relaxed) {
                    match read_base_pin(&reader_dir) {
                        Ok(_) => {
                            reads_r.fetch_add(1, Ordering::Relaxed);
                        }
                        // The area is legitimately absent between rounds.
                        Err(err) if err.kind() == std::io::ErrorKind::NotFound => {}
                        // The file EXISTS and does not parse — the truncation window.
                        Err(_) => {
                            torn_r.fetch_add(1, Ordering::Relaxed);
                        }
                    }
                }
            });

            for _ in 0..2_000 {
                let _ = std::fs::remove_dir_all(&dir);
                mint_milestone(&jigc_root, "Cache rework", base.clone()).expect("mint the area");
            }
            done.store(true, Ordering::Relaxed);
        });

        assert!(
            reads.load(Ordering::Relaxed) > 0,
            "the reader must actually have observed the pin — a run that read nothing proves \
             nothing (the vacuous pass)",
        );
        assert_eq!(
            torn.load(Ordering::Relaxed),
            0,
            "a concurrent reader parsed a half-written base.json: the shared-area write must \
             be temp + rename, never truncate-then-fill",
        );
    }
}
