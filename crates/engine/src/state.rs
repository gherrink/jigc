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

/// The working-area file recording a migration task's repo-relative foreign source
/// path (`jigc migrate <path>`), read back at finalize to retire the foreign original
/// (`design/auto-migration.md` → Retire-the-foreign-original). Absent on every
/// non-migration task — the retire set is then empty. Public so the CLI `migrate`
/// verb writes it under the same name the engine planner reads ([`read_source_path`]).
pub const SOURCE_PATH_FILE: &str = "source-path";

/// The working-area file recording a migration task's `--slug` override
/// (`jigc migrate <path> --as <doctype> --slug <s>`), read back by `doc author`
/// to drive the created target doc's id verbatim (the gated-create slug
/// override — M43 Inc 6 T2). Absent on a slug-less migrate and on every
/// non-migration task — the author then derives the slug from the payload
/// title, byte-identical to before. Public so the CLI `migrate` verb writes it
/// under the same name the author read-back uses ([`read_slug_override`]).
pub const SLUG_OVERRIDE_FILE: &str = "slug-override";

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
    let bytes = write::render(schema, &seeded_instance(schema, title, on_create));
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
/// **process-unique** `.<pid>.<nanos>.tmp` suffix (same directory, so `rename` is
/// intra-filesystem and atomic). The pid+nanos disambiguator is what makes
/// concurrent writers to a *shared* target (e.g. `.jigc/state/file-state.json`,
/// which is not task-isolated) each own a distinct temp: without it two writers
/// would share one `<name>.tmp` and interleave their bytes, so a reader could
/// observe a file that parses as neither writer's record (M45 Increment 7,
/// Decision 9).
fn temp_sibling(path: &Path) -> PathBuf {
    let mut name = path.file_name().unwrap_or_default().to_os_string();
    let nanos = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_nanos())
        .unwrap_or(0);
    name.push(format!(".{}.{}.tmp", std::process::id(), nanos));
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

/// Enumerate the **active task ids** under a project's `.jigc/` home — the sorted
/// directory names of `<jigc_root>/tasks/<id>/`. This is the single enumeration
/// source of truth the active-task resolution (`jigc doc`), the `jigc task list`
/// roster, and the ambiguous-task error all read, so they never disagree on which
/// tasks are live. A missing `tasks/` directory yields an empty list (no task minted
/// yet), never an error.
pub fn list_active_task_ids(jigc_root: &Path) -> Vec<String> {
    let tasks = jigc_root.join("tasks");
    let mut ids: Vec<String> = match std::fs::read_dir(&tasks) {
        Ok(entries) => entries
            .filter_map(std::result::Result::ok)
            .filter(|entry| entry.path().is_dir())
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

/// Read the persisted **foreign source path** of a migration task from its working
/// area (`<task_dir>/source-path`) — the repo-relative path of the foreign original
/// `jigc migrate` recorded at mint, read back at finalize to name the
/// retire-the-foreign-original target (`design/auto-migration.md` →
/// Retire-the-foreign-original). A missing file yields [`None`] — the clear
/// non-migration case (the retire set is then empty), never an error.
pub fn read_source_path(task_dir: &Path) -> std::io::Result<Option<String>> {
    match std::fs::read_to_string(task_dir.join(SOURCE_PATH_FILE)) {
        Ok(path) => Ok(Some(path)),
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

/// Write the staged snapshot into a working area (`<dir>/staged-snapshot.json`)
/// — the base-pin mold: pretty JSON, key-sorted (the `BTreeMap`/`BTreeSet` field
/// types), one trailing newline (golden-locked frozen on-disk form). Written at
/// every task-minting door (`jigc start` compose forms, `jigc migrate`,
/// `jigc milestone create` into the milestone area); a clean index writes an
/// **empty** snapshot, distinct from the absent pre-M43 case [`read_staged_snapshot`]
/// maps to `None`.
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
        Some(
            format!(
                "resume with `jigc start --task {id}` or abandon with `jigc task discard {id}`"
            )
            .into(),
        ),
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
    /// The create-or-update discriminator (M43 inc-7 T1): `true` when a committed
    /// instance already occupied the slug's canonical home and was **copied in for
    /// update** ([`copy_in`], `edited-from-base`); `false` on a fresh mint. Feeds the
    /// `doc create` ack's always-present `existed` key
    /// (`design/command-output-contract.md` §2).
    pub existed: bool,
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
    // `singleton` has no free title, so its H1 is the fixed type id (= slug) unless the
    // schema declares a `display-title:` knob (`vision` → `# Vision`). An id-source that
    // slugs empty keeps H1 == slug — the type-name fallback fired, so the slug stands in.
    let title = if schema.singleton {
        schema.display_title.clone().unwrap_or_else(|| slug.clone())
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
    let MintedInstance {
        slug,
        title,
        address,
        path,
    } = mint_instance(task_dir, schema, type_name, id_source, slug_override)?;

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
    if !migration_squatter
        && let Some(committed) = crate::store::canonical_path(repo_root, schema, &slug)
        && committed.is_file()
    {
        let body = std::fs::read_to_string(&committed)
            .map_err(|err| io_finding(&address, "read the committed instance", &err))?;
        let path = copy_in(task_dir, type_name, &slug, &body)
            .map_err(|err| io_finding(&address, "copy in the committed instance", &err))?;
        return Ok(CreatedDoc {
            address,
            path,
            existed: true,
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
    })
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
) -> Result<CreatedDoc, Finding> {
    // Step 3: unknown doctype rejects before the gate is consulted.
    let Some(schema) = schemas.get(type_name) else {
        return Err(unknown_doctype_finding(type_name, schemas));
    };
    // Step 5: a known-but-disallowed doctype is gate-blocked.
    let Some(entry) = gate.iter().find(|e| e.doc_type == type_name) else {
        return Err(gate_blocked_finding(type_name, gate));
    };
    // Step 4: admitted → mint + provision (or copy-in a committed instance) — the
    // `slug_override` (the front door's `--slug`) drives the minted id verbatim. But
    // first probe for a same-identity **staged** copy: if the minted slug's working-area
    // file already exists, `create` would reject it with `create.serial-collision`, which
    // routes the agent away from the only repairing action. Ack `existed` and fall
    // through to the role-bind instead (the shared `mint_instance` keeps the slug
    // derivation identical to `create`'s, so the probe can never diverge from the reject).
    let minted = mint_instance(task_dir, schema, type_name, id_source, slug_override)?;
    let created = if minted.path.exists() {
        CreatedDoc {
            address: minted.address,
            path: minted.path,
            existed: true,
        }
    } else {
        create(
            task_dir,
            schemas,
            type_name,
            id_source,
            repo_root,
            on_create,
            slug_override,
        )?
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

/// Does a **migration** task target this slug's own canonical destination? — the
/// create-side half of the in-location-squatter discriminator (`design/auto-migration.md`
/// → Path-collision guard / Hardening #8). Reads the task's recorded `source-path`
/// ([`read_source_path`]); when present and **canonically equal** — via the shared
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
    let Some(source) = read_source_path(task_dir)? else {
        return Ok(false);
    };
    let source = source.trim();
    if source.is_empty() {
        return Ok(false);
    }
    Ok(crate::store::lexical_normalize(Path::new(source))
        == crate::store::lexical_normalize(Path::new(&destination)))
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
    use proptest::prelude::*;

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
        .expect_err("an unknown type rejects before the gate");
        assert_eq!(unknown.code, "create.unknown-doctype");
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
