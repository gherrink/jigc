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

/// The on-disk path a staged `<type>:<slug>` instance lives at within `task_dir`:
/// `<task_dir>/docs/<type>:<slug>.md`.
pub fn instance_path(task_dir: &Path, type_name: &str, slug: &str) -> PathBuf {
    task_dir
        .join(DOCS_DIR)
        .join(instance_filename(type_name, slug))
}

/// Build the **empty** in-memory instance for `schema` — the canonical template the
/// workflow provisions: the H1 title is the task-derived `slug`, and every schema
/// section is left content-free (no field values, no slot prose, no items). Rendered
/// through [`crate::write::render`], this yields the full skeleton — front-matter (no
/// values), the `# <slug>` H1, and every `## Heading` with an empty slot — the bytes
/// the agent then fills slot-by-slot (`design/write-commands.md` → Instance
/// provisioning: "the agent only fills slots").
fn empty_instance(schema: &Schema, slug: &str) -> Instance {
    Instance {
        title: slug.to_string(),
        sections: schema
            .sections
            .iter()
            .map(|s| SectionContent {
                id: s.id.clone(),
                ..Default::default()
            })
            .collect(),
    }
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
pub fn provision_doc(task_dir: &Path, schema: &Schema, slug: &str) -> std::io::Result<PathBuf> {
    let path = instance_path(task_dir, &schema.ty, slug);
    let bytes = write::render(schema, &empty_instance(schema, slug));
    write_atomic(&path, bytes.as_bytes())?;
    // A minted-here instance: record `created` beside the body for the join's clash rule.
    record_provenance(
        task_dir,
        &format!("{}:{slug}", schema.ty),
        Provenance::Created,
    )?;
    Ok(path)
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
/// is a sibling (`<filename>.tmp`) so the `rename` stays on the same filesystem (atomic);
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

/// The sibling temp path for an atomic write of `path` — its filename with a `.tmp`
/// suffix (same directory, so `rename` is intra-filesystem and atomic).
fn temp_sibling(path: &Path) -> PathBuf {
    let mut name = path.file_name().unwrap_or_default().to_os_string();
    name.push(".tmp");
    match path.parent() {
        Some(parent) => parent.join(name),
        None => PathBuf::from(name),
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
) -> Result<MintedTask, Finding> {
    let id = mint_id(intent, type_name);
    let dir = jigc_root.join("tasks").join(&id);

    // Serial collision: an active task dir of that id already exists → reject,
    // surfacing its status, never silently suffixed or reused.
    if dir.exists() {
        return Err(collision_finding(&id));
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

/// Read the persisted [`BasePin`] of a task from its working area
/// (`<task_dir>/base.json`) — the companion of [`mint_task`]'s pin write, read
/// back on resume to compare against the current checkout. A missing or malformed
/// pin is an error (the pin is written at mint, so its absence is a real fault).
pub fn read_base_pin(task_dir: &Path) -> std::io::Result<BasePin> {
    let bytes = std::fs::read(task_dir.join(BASE_PIN_FILE))?;
    serde_json::from_slice(&bytes)
        .map_err(|err| std::io::Error::new(std::io::ErrorKind::InvalidData, err))
}

/// Slug the id-source into the task id, applying the empty → type-name fallback.
///
/// The pure normalization is [`crate::slug::slugify`]; the fallback is the mint
/// site's concern (only the caller knows the type name), per the slug rule
/// (`DECISIONS.md` 2026-05-31 → Slug / minting normalization).
fn mint_id(intent: &str, type_name: &str) -> String {
    let slug = crate::slug::slugify(intent);
    if slug.is_empty() {
        crate::slug::slugify(type_name)
    } else {
        slug
    }
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
        Some(format!(
            "resume with `jigc start --task {id}` or abandon with `jigc task discard {id}`"
        )),
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

/// A freshly created doc instance: its minted `address` (`<type>:<slug>`) and the
/// on-disk `path` of the staged instance in the working area.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct CreatedDoc {
    /// The minted address — `<type>:<slug>` (no fragment; the whole container).
    pub address: String,
    /// The staged instance path, `<task_dir>/docs/<type>:<slug>.md`.
    pub path: PathBuf,
}

/// **The `create`/provisioning verb** against the task working area — the
/// structural act the CLI always owns (`design/write-commands.md` → Instance
/// provisioning: "mint the id … and place it at the schema-defined location";
/// `design/structural-grammar.md` → IDs: provenance and minting). This is the
/// **workflow-provisioned** path (the gate is bypassed; see [`create_gated`] for
/// the agent-initiated path that consults `allows-create`):
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
/// 4. **Provision** the empty instance at `docs/<type>:<slug>.md` via
///    [`provision_doc`] and return its [`CreatedDoc`] address + path.
pub fn create(
    task_dir: &Path,
    schemas: &std::collections::BTreeMap<String, Schema>,
    type_name: &str,
    id_source: &str,
) -> Result<CreatedDoc, Finding> {
    // 1. Unknown doctype → reject before anything is minted or placed.
    let Some(schema) = schemas.get(type_name) else {
        return Err(unknown_doctype_finding(type_name));
    };

    // 2. Mint the frozen content-slug. A `singleton` doctype fixes the slug to the
    //    type id unconditionally (so a re-create targets the same `<location>/<ty>.md`,
    //    review B-2); a non-singleton slugs the id-source (empty → type-name fallback).
    let slug = if schema.singleton {
        schema.ty.clone()
    } else {
        mint_id(id_source, type_name)
    };
    let address = format!("{type_name}:{slug}");
    let path = instance_path(task_dir, type_name, &slug);

    // 3. Serial collision → reject, never suffixed, nothing created.
    if path.exists() {
        return Err(instance_collision_finding(&address));
    }

    // 4. Provision the empty instance and return its address + path.
    let path = provision_doc(task_dir, schema, &slug)
        .map_err(|err| io_finding(&address, "provision the instance", &err))?;
    Ok(CreatedDoc { address, path })
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
pub fn create_gated(
    task_dir: &Path,
    schemas: &std::collections::BTreeMap<String, Schema>,
    gate: &[crate::compose::AllowsCreate],
    type_name: &str,
    id_source: &str,
) -> Result<CreatedDoc, Finding> {
    // Step 3: unknown doctype rejects before the gate is consulted.
    if !schemas.contains_key(type_name) {
        return Err(unknown_doctype_finding(type_name));
    }
    // Step 5: a known-but-disallowed doctype is gate-blocked.
    let Some(entry) = gate.iter().find(|e| e.doc_type == type_name) else {
        return Err(gate_blocked_finding(type_name, gate));
    };
    // Step 4: admitted → mint + provision …
    let created = create(task_dir, schemas, type_name, id_source)?;
    // … then bind it to the entry's `as:` role if the entry declares one.
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

/// The unknown-doctype block: a blocking finding naming the unrecognized type,
/// routing the agent to list the known types (`write-commands.md` → The create-gate,
/// step 3). Route-bearing per the settled block-payload shape.
fn unknown_doctype_finding(type_name: &str) -> Finding {
    Finding::graded(
        Severity::Blocking,
        "create.unknown-doctype",
        format!("unknown doctype `{type_name}`"),
        None,
        Some("list the available doctypes with `jigc doc types`".to_string()),
    )
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
        Some(format!(
            "edit the existing `{address}` instead of re-creating it"
        )),
    )
}

/// The structured create-gate block (`write-commands.md` → The create-gate, step 5):
/// a blocking finding naming the disallowed type + the allowed set, carrying the
/// loosen `run-command` route (the cascade-set config delta the agent can act on).
fn gate_blocked_finding(type_name: &str, gate: &[crate::compose::AllowsCreate]) -> Finding {
    let allowed: Vec<&str> = gate.iter().map(|e| e.doc_type.as_str()).collect();
    Finding::graded(
        Severity::Blocking,
        "create.gate-blocked",
        format!(
            "the workflow does not allow `jigc doc create {type_name}` in-task; allowed doctypes: [{}]",
            allowed.join(", ")
        ),
        None,
        Some(format!(
            "to loosen, add `{type_name}` to `allows-create` in project config"
        )),
    )
}

/// A blocking finding for a working-area I/O failure during minting.
fn io_finding(id: &str, doing: &str, err: &std::io::Error) -> Finding {
    Finding::graded(
        Severity::Blocking,
        "task.working-area-io",
        format!("could not {doing} for task `{id}`: {err}"),
        Some(Location::addressed(format!("task:{id}"), 1, 1)),
        None,
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;

    /// A throwaway directory that removes itself on drop — keeps mint tests off
    /// any real `.jigc/` tree.
    struct TempRoot(PathBuf);

    impl TempRoot {
        fn new(tag: &str) -> Self {
            let mut path = std::env::temp_dir();
            let unique = format!(
                "jigc-mint-{tag}-{}-{:?}",
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

    /// The done-criterion for T3a (`DECISIONS.md` 2026-06-01 → M2 Increment 3
    /// re-cut): minting persists the **minting workflow id** alongside the intent,
    /// in the same plain-text style, so resume composes the task's *own* workflow,
    /// not the cascade default. The field round-trips through [`read_workflow_id`];
    /// a task working area without the file yields the clear absent case (`None`).
    #[test]
    fn mint_persists_the_workflow_id_read_back_verbatim() {
        let root = TempRoot::new("workflow-id");
        let base = BasePin::new("b".repeat(40), "bbbbbbb");

        let minted = mint_task(root.path(), "Add rate limiter", "commit", "quick-fix", base)
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

        let minted = mint_task(root.path(), "!!!___---", "adr", "single-task", base)
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

    const COMMIT_YAML: &[u8] = include_bytes!("../../cli/pack/schemas/commit.yaml");

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
        let path =
            provision_doc(&task_dir, &schema, "add-rate-limiter").expect("provision succeeds");
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
        let created_path =
            provision_doc(&task_dir, &schema, "add-rate-limiter").expect("provision succeeds");
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
        let created = create(&task_dir, &schemas, "commit", "Add rate limiter")
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
        let expected = write::render(
            &schemas["commit"],
            &empty_instance(&schemas["commit"], "add-rate-limiter"),
        );
        assert_eq!(on_disk, expected, "created instance is the empty template");

        // Unknown doctype → blocking `create.unknown-doctype`, nothing created.
        let docs_before = std::fs::read_dir(task_dir.join("docs"))
            .expect("docs dir")
            .count();
        let err =
            create(&task_dir, &schemas, "spec", "whatever").expect_err("unknown doctype rejects");
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
        let collide = create(&task_dir, &schemas, "commit", "Add rate limiter")
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
        let non_singleton = create(&task_dir, &schemas(), "commit", "Add rate limiter")
            .expect("non-singleton create succeeds");
        assert_eq!(
            non_singleton.address, "commit:add-rate-limiter",
            "a non-singleton still slugs the id_source",
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
        }];

        // In the gate → proceeds (mints + provisions).
        let ok = create_gated(&task_dir, &all, &gate, "adr", "Some Decision")
            .expect("a gate-admitted type proceeds");
        assert_eq!(ok.address, "adr:some-decision");

        // Not in the gate → structured gate-block with the loosen route.
        let blocked = create_gated(&task_dir, &all, &gate, "commit", "x")
            .expect_err("a disallowed type is gate-blocked");
        assert_eq!(blocked.severity, Severity::Blocking);
        assert_eq!(blocked.code, "create.gate-blocked");
        assert!(
            blocked.message.contains("commit") && blocked.message.contains("adr"),
            "the block names the disallowed type and the allowed set: {blocked:?}"
        );
        assert!(
            blocked.route.is_some(),
            "the gate-block carries a loosen run-command route"
        );

        // Unknown type → unknown-doctype reject fires *before* the gate.
        let unknown = create_gated(&task_dir, &all, &gate, "wormhole", "x")
            .expect_err("an unknown type rejects before the gate");
        assert_eq!(unknown.code, "create.unknown-doctype");
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
        }];

        // No roles.json before any create.
        assert!(
            !RolesRecord::path_in(&task_dir).exists(),
            "no roles.json exists before a bound create"
        );

        let created = create_gated(&task_dir, &all, &gate, "adr", "Shared Redis session cache")
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
        }];
        let _ = create_gated(&bare_dir, &all, &bare_gate, "commit", "Add rate limiter")
            .expect("the bare-form-gated commit is created");
        assert!(
            !RolesRecord::path_in(&bare_dir).exists(),
            "a bare-form entry (no `as:` role) binds nothing — no roles.json written"
        );
    }
}
