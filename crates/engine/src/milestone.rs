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

/// The on-disk directory a milestone's state lives in: `<jigc_root>/milestones/<id>/`.
pub fn milestone_dir(jigc_root: &Path, id: &str) -> PathBuf {
    jigc_root.join("milestones").join(id)
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
    let task = crate::state::mint_task(jigc_root, intent, SUB_TASK_TYPE, workflow_id, base)?;

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

/// One staged doc folded into the parent working overlay at the by-task-id join
/// (`design/storage.md` → The by-task-id join, algorithm step 2: "disjoint union of
/// staged docs, classified by provenance"). Carries the discriminator the clash rule
/// (T2) and cross-area rule (T3) will key on, plus the sub-task it came from (so a
/// later rule can name the offending area) and the doc's forward edges (the per-area
/// `overlay_working` derivation's output for this `from`, reused unchanged).
#[derive(Clone, Debug, PartialEq, Eq)]
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
#[derive(Clone, Debug, Default, PartialEq, Eq)]
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
        milestone_id,
        &list.enumerate(),
        schemas,
        committed,
    )
}

/// Fold the named sub-task areas — in the **given** order — into the merged overlay,
/// classified by provenance (`design/storage.md` → The by-task-id join, step 2). The
/// only accumulator is the address-keyed `BTreeMap`, so the result is a pure function
/// of the *set* of `sub_ids`, independent of their iteration order (the order-invariance
/// the join's contract rests on; the public [`join`] always feeds the id-sorted order).
fn fold_areas(
    jigc_root: &Path,
    milestone_id: &str,
    sub_ids: &[String],
    schemas: &std::collections::BTreeMap<String, crate::schema::Schema>,
    committed: &crate::index::EdgeIndex,
) -> Result<JoinOutcome, Finding> {
    let mut overlay: std::collections::BTreeMap<String, MergedDoc> =
        std::collections::BTreeMap::new();

    for sub_id in sub_ids {
        let sub_dir = jigc_root.join("tasks").join(sub_id);

        // The per-area basis: the single-area working-overlay derivation, reused
        // unchanged — its `task_edges` are the staged docs' forward edges, keyed by
        // `from`; its `task_froms` are this area's staged doc addresses.
        let area = crate::index::overlay_working(committed, &sub_dir, schemas);
        let provenance = crate::state::ProvenanceRecord::load(&sub_dir)
            .map_err(|err| io_finding(milestone_id, "read a sub-task provenance manifest", &err))?;

        // Step 2: disjoint union of this area's staged docs, classified by provenance.
        for from in &area.task_froms {
            // The forward edges this staged doc contributes (already sorted in `area`).
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
            overlay.insert(
                from.clone(),
                MergedDoc {
                    provenance: prov,
                    source_task: sub_id.clone(),
                    edges,
                },
            );
        }
    }

    Ok(JoinOutcome {
        overlay,
        findings: Vec::new(),
    })
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
            crate::schema::load_schema(SPEC_YAML).expect("spec.yaml loads"),
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
            crate::schema::load_schema(ADR_YAML).expect("adr.yaml loads"),
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
        crate::state::provision_doc(&alpha_dir, &schemas["commit"], "alpha-area")
            .expect("provision created doc");
        let zebra_dir = root.path().join("tasks").join("zebra-area");
        let adr_source = "---\nstatus: accepted\ndate: 2026-06-04\n---\n\n# Zebra decision\n\n## Context\n\nForces.\n\n## Decision\n\nDo the thing.\n";
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
        let outcome =
            join(root.path(), &milestone.id, &schemas, &committed).expect("the join folds");

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
        let forward = fold_areas(root.path(), &milestone.id, &ids, &schemas, &committed)
            .expect("forward fold");
        let mut reversed = ids.clone();
        reversed.reverse();
        let backward = fold_areas(root.path(), &milestone.id, &reversed, &schemas, &committed)
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
            "no-such-milestone",
            &join_schemas(),
            &crate::index::EdgeIndex::default(),
        )
        .expect_err("an unknown milestone rejects");
        assert_eq!(err.severity, Severity::Blocking);
        assert_eq!(err.code, "milestone.unknown");
        assert!(err.route.is_some());
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
}
