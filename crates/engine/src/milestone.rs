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

use crate::finding::{Finding, Location, Severity};
use crate::state::BasePin;
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

/// The base-pin filename inside a milestone's area — the **single shared base**
/// every sub-task inherits, in the same frozen byte form as a task's `base.json`.
const BASE_PIN_FILE: &str = "base.json";

/// The task-list filename inside a milestone's area — the sub-task ids the
/// by-task-id join will enumerate, persisted as deterministic engine state.
const TASKS_FILE: &str = "tasks.json";

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

/// The `status` value a freshly materialized record (and each fresh sub-task item)
/// carries until `join` transitions it to `joined`.
const RECORD_STATUS_ACTIVE: &str = "active";

/// The `status` value the `join` in-place-mutate write arm transitions the record's
/// header **and** every committed sub-task item to (the `set: on-transition` target).
const RECORD_STATUS_JOINED: &str = "joined";

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
    std::fs::write(&pin_path, pin_body)
        .map_err(|err| io_finding(&id, "write the base pin", &err))?;

    // The empty task list — no sub-task minted yet.
    std::fs::write(dir.join(TASKS_FILE), TaskList::default().to_bytes())
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
    std::fs::write(dir.join(TASKS_FILE), list.to_bytes())
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
/// **Unknown milestone** → the [`add_task`] unknown-milestone block, before any
/// spec read or mint. **Unknown / transient / unparseable spec** → the existing
/// [`crate::store`]-shaped routed blocking finding (`store.unknown-type`,
/// `store.transient-type`, `store.not-found`, `store.unparseable`). **Zero
/// `criteria` items** → a blocking `milestone.no-criteria` finding ("nothing to
/// seed from"); nothing is minted. A within-spec slug collision between two
/// criteria surfaces the [`add_task`] `milestone.sub-task-collision` block
/// unchanged (the suffix is the join's, never incremental seed).
pub fn add_from_spec(
    jigc_root: &Path,
    repo_root: &Path,
    schemas: &std::collections::BTreeMap<String, crate::schema::Schema>,
    milestone_id: &str,
    spec_addr: &str,
    workflow_id: &str,
) -> Result<Vec<AddedTask>, Finding> {
    // Unknown milestone → reject before any spec read or mint.
    if !milestone_dir(jigc_root, milestone_id).is_dir() {
        return Err(unknown_milestone_finding(milestone_id));
    }

    // Read the committed spec and enumerate its `criteria` items (parse-items path).
    let criteria = read_spec_criteria(repo_root, schemas, spec_addr)?;

    // Zero criteria → "nothing to seed from"; mint nothing.
    if criteria.is_empty() {
        return Err(no_criteria_finding(spec_addr));
    }

    // One sub-task per criterion, the criterion text as intent, in physical order.
    let mut added = Vec::with_capacity(criteria.len());
    for intent in &criteria {
        added.push(add_task(jigc_root, milestone_id, intent, workflow_id)?);
    }
    Ok(added)
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
            "list the available doctypes with `jigc doc types`".to_string(),
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
        return Err(store_block(
            "store.no-such-section",
            format!("`{spec_addr}` names no `{CRITERIA_SECTION}` section to seed from"),
            spec_addr,
            format!("the spec must declare a `{CRITERIA_SECTION}` section"),
        ));
    };

    Ok(section.items.iter().map(|i| i.title.clone()).collect())
}

/// Build a blocking store-shaped finding (the same code/route shape
/// [`crate::store`] surfaces) for a spec-read failure during seeding.
fn store_block(code: &str, message: String, address: &str, route: String) -> Finding {
    Finding::graded(
        Severity::Blocking,
        code,
        message,
        Some(Location::addressed(address, 1, 1)),
        Some(route),
    )
}

/// The zero-criteria block: a blocking finding naming the empty spec, routing the
/// agent to add criteria or seed differently — never a silent empty milestone
/// (`design/write-commands.md` → Minting a milestone: "nothing to seed from").
fn no_criteria_finding(spec_addr: &str) -> Finding {
    Finding::graded(
        Severity::Blocking,
        "milestone.no-criteria",
        format!("spec `{spec_addr}` has no criteria to seed from"),
        Some(Location::addressed(spec_addr, 1, 1)),
        Some(
            "add `criteria` items to the spec, or add sub-tasks with `jigc milestone add-task`"
                .to_string(),
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
        Some("create it first with `jigc milestone create \"<title>\"`".to_string()),
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
        Some(
            "restore the base commit, or re-pin the milestone's base, then re-run the op"
                .to_string(),
        ),
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
        Some("add the sub-task with a distinct intent".to_string()),
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
    if milestone_dir.join(BASE_PIN_FILE).exists() && milestone_dir.join(TASKS_FILE).exists() {
        return Ok(());
    }
    let (base, tasks) = read_back_record(schema, record_source)?;
    std::fs::create_dir_all(milestone_dir)
        .map_err(|err| io_finding(id, "open the milestone cache area", &err))?;
    std::fs::write(milestone_dir.join(BASE_PIN_FILE), render_base_pin(&base))
        .map_err(|err| io_finding(id, "re-seed the base pin cache", &err))?;
    std::fs::write(milestone_dir.join(TASKS_FILE), tasks.to_bytes())
        .map_err(|err| io_finding(id, "re-seed the task list cache", &err))?;
    Ok(())
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
        Some("reconcile the milestone record, then re-run the milestone op".to_string()),
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
/// `set: on-transition` leaves `intent` and `status: active` (the CLI-supplied
/// machine-set values a freshly-added sub-task carries until `join`) — via the M16
/// [`crate::write::add_item`] primitive over the pack-supplied `schema`.
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
) -> Result<String, crate::write::GenerateError> {
    use crate::field_block::{Field, Value};

    // The `set: on-transition` leaves in schema block order (the id-from `task-id` is the
    // heading, not a bullet), materialized to the fresh-append values: the recorded
    // `intent`, seeded `status: active` until `join` transitions it to `joined`.
    let fields = vec![
        Field {
            key: RECORD_TASK_INTENT_FIELD.to_string(),
            value: Value::Scalar(intent.to_string()),
        },
        Field {
            key: RECORD_STATUS_FIELD.to_string(),
            value: Value::Scalar(RECORD_STATUS_ACTIVE.to_string()),
        },
    ];
    crate::write::add_item(schema, source, RECORD_TASKS_SECTION, task_id, None, &fields)
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
        .map_err(|err| record_flip_finding(milestone_id, err))?;
    std::fs::write(record_path, &flipped)
        .map_err(|err| io_finding(milestone_id, "write the joined milestone record", &err))?;
    Ok(flipped)
}

/// The pure in-place status flip behind [`join_record`]: read the committed `tasks` item
/// ids from the record itself (the source of truth), splice **each** item's `status`
/// value to `joined` via the byte-stable item-leaf splice threading the updated bytes
/// item by item, then splice the header `status` (front-matter, block-order-first). A
/// non-conformant record, or a `status` leaf the splice cannot locate, is a
/// [`SpliceError`](crate::write::SpliceError) the caller routes.
fn flip_record_status_to_joined(
    schema: &crate::schema::Schema,
    source: &str,
) -> Result<String, crate::write::SpliceError> {
    let doc = crate::parse::parse_sections(schema, source)
        .map_err(|_| crate::write::SpliceError::NotConformant)?;
    let item_ids: Vec<String> = doc
        .sections
        .iter()
        .find(|s| s.id == RECORD_TASKS_SECTION)
        .map(|s| s.items.iter().map(|i| i.id.clone()).collect())
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

/// A blocking finding for a `status` splice failure while flipping a milestone record to
/// `joined` — the record did not conform or a targeted `status` leaf vanished (a real
/// fault: the record is the record arm's own materialized output). Routed to reconcile.
fn record_flip_finding(milestone_id: &str, err: crate::write::SpliceError) -> Finding {
    Finding::graded(
        Severity::Blocking,
        "milestone.record-flip",
        format!("could not flip milestone record `{milestone_id}` to joined: {err:?}"),
        Some(Location::addressed(
            format!("milestone-record:{milestone_id}"),
            1,
            1,
        )),
        Some("reconcile the milestone record, then re-run the join".to_string()),
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
    /// The join's findings, in a stable order (empty in the skeleton increment).
    pub findings: Vec<Finding>,
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
            addresses.push(final_address);
        }
    }
    // Id-sorted addresses (the overlay-key order) — the audit trail of what landed.
    addresses.sort();

    Ok(MaterializeOutcome {
        docs_dir,
        addresses,
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

        for from in &area.task_froms {
            let edges: Vec<crate::index::Edge> = area
                .task_edges
                .iter()
                .filter(|e| &e.from == from)
                .cloned()
                .collect();
            // A staged doc with no recorded provenance is a real fault (the bit is
            // written at stage time beside every body); defaulting to a provenance is
            // wrong, so surface the absence as a blocking finding routed to re-stage.
            let Some(prov) = provenance.get(from) else {
                return Err(missing_provenance_finding(milestone_id, sub_id, from));
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

    Ok(JoinOutcome { overlay, findings })
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
                .to_string(),
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
                .to_string(),
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
        Some("re-stage the doc under a known doctype".to_string()),
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
            "could not rewrite the self-reference `{relation}` of colliding doc `{address}` in milestone `{milestone_id}`: {err:?}"
        ),
        Some(Location::addressed(address, 1, 1)),
        Some("re-stage the colliding doc so its self-reference is well-formed".to_string()),
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
        Some("re-stage the doc so its provenance is recorded".to_string()),
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
        Some(
            "re-stage the doc inside its own sub-task area, or drop the stray attribution"
                .to_string(),
        ),
    )
}

/// Slug the title into the milestone id, applying the empty → type-name
/// (`milestone`) fallback — the same discipline as `state::mint_id`.
fn mint_id(title: &str) -> String {
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
        )),
    )
}

/// A blocking finding for an area I/O failure during minting.
fn io_finding(id: &str, doing: &str, err: &std::io::Error) -> Finding {
    Finding::graded(
        Severity::Blocking,
        "milestone.area-io",
        format!("could not {doing} for milestone `{id}`: {err}"),
        Some(Location::addressed(format!("milestone:{id}"), 1, 1)),
        None,
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    /// A throwaway directory that removes itself on drop — keeps mint tests off
    /// any real `.jigc/` tree (mirrors `state.rs`'s `TempRoot`).
    struct TempRoot(PathBuf);

    impl TempRoot {
        fn new(tag: &str) -> Self {
            let mut path = std::env::temp_dir();
            let unique = format!(
                "jigc-milestone-{tag}-{}-{:?}",
                std::process::id(),
                std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap()
                    .as_nanos(),
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

        // The fixture's on-disk sub-task areas are not handed back pre-sorted by id
        // by the filesystem in at least one of the two runs — so a green
        // enumeration cannot be an accident of read_dir order. (If a filesystem
        // returns id order in BOTH runs we still have the insertion-order proof
        // above; this asserts the fixture genuinely exercises a divergent order.)
        let read_dir_diverges = |raw: &[String]| {
            let mut sorted = raw.to_vec();
            sorted.sort();
            raw != sorted.as_slice()
        };
        assert!(
            read_dir_diverges(&read_dir1) || read_dir_diverges(&read_dir2),
            "neither run's read_dir order diverged from id order: {read_dir1:?} / {read_dir2:?}"
        );
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
        )
        .expect("3-criteria spec seeds 3 sub-tasks");

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
        )
        .expect_err("a zero-criteria spec blocks");

        assert_eq!(err.severity, Severity::Blocking);
        assert_eq!(err.code, "milestone.no-criteria");
        assert!(
            err.message.contains("empty-plan"),
            "the block names the empty spec: {err:?}"
        );
        assert!(err.route.is_some(), "the no-criteria block carries a route");

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
        )
        .expect_err("an unknown milestone rejects");
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
        )
        .expect_err("an unknown spec rejects");
        assert_eq!(unknown_spec.severity, Severity::Blocking);
        assert_eq!(unknown_spec.code, "store.not-found");
        assert!(unknown_spec.route.is_some());
        assert_eq!(
            read_task_list(&milestone.dir).expect("read list").tasks,
            Vec::<String>::new(),
            "an unknown spec appends nothing"
        );
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
        add_task(root.path(), &milestone.id, "A area", "single-task").expect("a adds");

        let a_dir = root.path().join("tasks").join("a-area");
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
                == Some("adr:cache-strategy"),
            "the block is located at B's authoring doc: {:?}",
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
        //   [a-area, b-area, evict-area, low-strategy, zed-strategy]
        for intent in [
            "Zed strategy",
            "A area",
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

        // --- Overlap 2: a cross-area ref. `a-area` creates `adr:lru-eviction`;
        // `b-area` creates `adr:b-decision` whose `supersedes` GUESSES `a-area`'s
        // slug — resolvable only inside a sibling area → blocking ref-resolves.
        stage_doc(
            &root.path().join("tasks").join("a-area"),
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
                "a-area".to_string(),
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
    /// `tasks` item per sub-task over the fresh record — `task-id`/`intent`/`status:
    /// active` — **byte-stable** (the bytes outside each appended item's span are
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
        let after_one = append_task_item(&schema, &fresh, "warm-cache", "Warm the read cache")
            .expect("first sub-task appends");
        let after_two = append_task_item(&schema, &after_one, "evict-cold", "Evict cold entries")
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
- status: active

### evict-cold  {#evict-cold}

<!-- fields -->
- intent: Evict cold entries
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
        let seen: Vec<(&str, Option<String>, Option<String>)> = tasks
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
                    Some(RECORD_STATUS_ACTIVE.to_string()),
                ),
                (
                    "evict-cold",
                    Some("Evict cold entries".to_string()),
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
        let after_one = append_task_item(&schema, &fresh, "warm-cache", "Warm the read cache")
            .expect("first sub-task appends");
        let committed = append_task_item(&schema, &after_one, "evict-cold", "Evict cold entries")
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
        let r1 = append_task_item(&schema, &fresh, &a.task.id, "Warm the read cache")
            .expect("append sub-task 1");
        let record = append_task_item(&schema, &r1, &b.task.id, "Evict cold entries")
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
}
