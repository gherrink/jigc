//! `jigc start "<intent>"` — task minting + composition (the front door).
//!
//! The CLI reads HEAD (`git rev-parse`) and hands the engine an explicit base
//! SHA; the engine slugs the intent, opens `.jigc/tasks/<id>/`, and writes the
//! base pin (`design/write-commands.md` → Task origination; `design/storage.md`
//! → The per-task working area / base pinning). "CLI orchestrates, git executes"
//! ([storage.md](../../../design/storage.md) → CLI and git): the git invocation
//! lives here, never in the engine, so the engine stays a pure function of its
//! inputs.
//!
//! After minting, [`compose_in_repo`] composes the cascade's default workflow
//! (post-flip, the `creates-task: false` `router`; `DECISIONS.md` 2026-06-01)
//! with `{{task.intent}}` = the intent — locating the cascade
//! (CLI locates, engine resolves), running the `workflow-refs` gate at
//! compose-time, then emitting the four-class composed view
//! ([write-commands.md](../../../design/write-commands.md) → Task origination;
//! [workflow-dialect.md](../../../design/workflow-dialect.md) → The composed
//! output is a view). Composition is deterministic and makes no LLM call (same
//! resolved cascade in → same workflow out).

use crate::pack::EmbeddedPack;
use anyhow::{Context, Result, bail};
use engine::address::Address;
use engine::cascade::{self, OverrideLayer, PackDefaultLayer};
use engine::compose::{
    self, CommandCatalog, ComposedWorkflow, StepDef, StepSource, StoreContext, WorkflowDef,
    load_command_catalog, load_step_def, load_workflow_def,
};
use engine::data_value::{ComposeContext, TaskRoot};
use engine::finding::{Finding, Severity};
use engine::index;
use engine::packsource::{PackResourceKind, PackSource, ResourceId};
use engine::result::CatalogEntry;
use engine::schema::{Schema, load_schema};
use engine::state::{self, BasePin, MintedTask, RolesRecord};
use std::collections::BTreeMap;
use std::path::Path;
use std::process::Command;

/// The default doc-type name minting falls back to when the intent slugs to
/// nothing. `single-task` is a `commit`-producing work-workflow, so its task-id
/// fallback is the `commit` type name (`write-commands.md` → Instance
/// provisioning: the task's commit doc).
const FALLBACK_TYPE: &str = "commit";

/// Mint a task in the repo containing `start`: read HEAD, then open the working
/// area + base pin under `<repo_root>/.jigc/`.
///
/// Repo-root discovery reuses the same `.git`-ancestor walk as cascade location
/// ([`crate::locate`]). A serial collision (an active task of the slugged id
/// already exists) surfaces as the engine's routed blocking finding, mapped here
/// to an `anyhow` error carrying that route.
pub fn mint_in_repo(start: &Path, intent: &str, workflow_id: &str) -> Result<MintedTask> {
    let repo_root = discover_repo_root(start)
        .with_context(|| format!("not inside a git repository (from {})", start.display()))?;
    let jigc_root = repo_root.join(".jigc");
    let base = read_head(&repo_root)?;

    state::mint_task(&jigc_root, intent, FALLBACK_TYPE, workflow_id, base).map_err(finding_to_err)
}

/// Provision the task's workflow-provisioned **commit** doc into the working
/// area as a **fillable form**: the empty skeleton with every schema header field
/// pre-stamped as an empty `key:` line, materialized at
/// `<task_dir>/docs/commit:<id>.md`. This is the deterministic structural act the
/// workflow owns — the agent then fills the `summary`/`body` slots (`set-slot`)
/// and splices the `type`/`scope` fields (`set-field`) into the present lines.
///
/// The header-field lines are pre-stamped because the engine's `set-field` is a
/// **surgical splice** of a present value line ([`engine::write::set_field`]):
/// front-matter has no generate-a-new-line path (the header is not a generatable
/// body section), so the form must carry the line for the agent to fill. (Body
/// slots provision empty and `set-slot` generates the section's home on demand —
/// `write-commands.md` → Instance provisioning; `DECISIONS.md` 2026-05-31 → inc-4
/// fillable-form provisioning.)
fn provision_commit_doc(pack: &dyn PackSource, minted: &MintedTask) -> Result<()> {
    let bytes = read_pack(pack, PackResourceKind::Schemas, FALLBACK_TYPE)?;
    let schema = engine::schema::load_schema(&bytes)
        .map_err(|e| anyhow::anyhow!("the `{FALLBACK_TYPE}` schema is malformed: {e}"))?;
    let instance = fillable_form(&schema, &minted.id);
    let path = state::instance_path(&minted.dir, &schema.ty, &minted.id);
    let rendered = engine::write::render(&schema, &instance);
    state::persist(&path, rendered.as_bytes())
        .with_context(|| format!("could not provision the commit doc for `{}`", minted.id))?;
    Ok(())
}

/// Build the **fillable** empty instance for `schema`: the H1 title is the task id,
/// every body section is content-free, and the header section's declared fields are
/// pre-stamped as empty `key:` lines (the surface `set-field` splices). The empty
/// value is `Value::Scalar("")`, which renders the canonical `key:` line.
///
/// **Exception — an optional `ref` is not pre-stamped.** An optional ref (forward
/// `card:` minimum 0, e.g. `commit.implements`'s `0..1`) carries no author
/// obligation: a spec-less task leaves it unset, so a pre-stamped empty `key:` line
/// would survive to finalize as a malformed-empty value + a dangling edge (both
/// blocking) — yet the design mandates a spec-less `commit` finalizes cleanly
/// without it (`implementation/doctype-map.md` → `commit —implements→ spec`, `0..1`).
/// So an optional ref is **absent** in the fillable form, exactly as `validate.rs`'s
/// `required-field-present` exempts it (the two views agree on the same cardinality
/// predicate). The spec-driven workflow that *does* set it materializes the line
/// when it lands (a later M3 increment), not here.
fn fillable_form(schema: &engine::schema::Schema, slug: &str) -> engine::write::Instance {
    use engine::field_block::{Field, Value};
    use engine::schema::SectionBody;
    use engine::write::SectionContent;

    let sections = schema
        .sections
        .iter()
        .map(|section| {
            let fields = if section.header {
                match &section.body {
                    SectionBody::Simple { fields, .. } => fields
                        .iter()
                        .filter(|f| !is_optional_ref(f))
                        .map(|f| Field {
                            key: f.id.clone(),
                            value: Value::Scalar(String::new()),
                        })
                        .collect(),
                    SectionBody::Repeatable { .. } => Vec::new(),
                }
            } else {
                Vec::new()
            };
            SectionContent {
                id: section.id.clone(),
                fields,
                ..Default::default()
            }
        })
        .collect();
    engine::write::Instance {
        title: slug.to_string(),
        sections,
    }
}

/// Whether `field` is an **optional `ref`** — a `ref` whose forward cardinality has a
/// minimum of 0 (`card:` absent ⇒ the `"0..1"` default, or an explicit form starting
/// with `0`). Such a ref carries no author obligation, so the fillable form omits its
/// line. Mirrors `engine::validate`'s `required-field-present` optional-ref exemption
/// so provisioning and validation agree on the same predicate.
fn is_optional_ref(field: &engine::schema::Field) -> bool {
    field.ty == engine::schema::FieldType::Ref
        && field
            .card
            .as_deref()
            .is_none_or(|card| card.trim_start().starts_with('0'))
}

/// The cascade knob the default-workflow id is read from
/// (`design/overrides.md`; `design/write-commands.md` → Task origination: the
/// `default-workflow: <id>` cascade knob the front door composes). Post-flip the
/// pack ships `default-workflow: router` in its `config/defaults` resource
/// (`DECISIONS.md` 2026-06-01 → M2 flips `default-workflow` to `router`).
const DEFAULT_WORKFLOW_KEY: &str = "default-workflow";

/// Mint a task from `intent`, then compose the cascade's default workflow over
/// it — the `jigc start "<intent>"` front door.
///
/// Locates the cascade (a missing project layer routes to `jigc setup`), resolves
/// `default-workflow` **through the cascade** (so a project `scalar:` delta
/// flips it — [`resolve_default_workflow`]), mints the task (seq 12),
/// builds the [`ComposeContext`] binding `{{task.intent}}`/`{{task.id}}` and
/// declaring the workflow's `allows-create` roles unbound, runs the
/// `workflow-refs` gate at compose-time, and on a clean gate emits the
/// deterministic four-class composed view. A blocking gate finding short-circuits
/// with that finding's message + route — nothing is emitted past the gate.
///
/// Returns the composed view; the caller renders it through the selected format.
pub fn compose_in_repo(start: &Path, intent: &str) -> Result<ComposedWorkflow> {
    let repo_root = discover_repo_root(start)
        .with_context(|| format!("not inside a git repository (from {})", start.display()))?;
    let project_config = repo_root.join(".jigc").join("config");
    if !project_config.is_dir() {
        bail!(
            "this project isn't set up — run `jigc setup` (no `.jigc/config/` cascade layer found)"
        );
    }

    let pack = EmbeddedPack::new();
    let workflow_id = resolve_default_workflow(&pack, &project_config)?;
    compose_core(&repo_root, intent, &pack, &workflow_id)
}

/// Compose the workflow named by `workflow_id` from `intent` — the explicit
/// `jigc start --workflow <X> "<intent>"` front door (Form D, `write-commands.md`
/// → Task origination). Unlike [`compose_in_repo`] it bypasses the cascade
/// `default-workflow` knob and composes the *named* workflow, minting iff `X`
/// declares `creates-task: true`; an unknown `<X>` is rejected with a routed
/// finding before any mint. The cascade-presence guard (a missing project layer
/// routes to `jigc setup`) is identical to the default front door.
pub fn compose_named_in_repo(
    start: &Path,
    intent: &str,
    workflow_id: &str,
) -> Result<ComposedWorkflow> {
    let repo_root = discover_repo_root(start)
        .with_context(|| format!("not inside a git repository (from {})", start.display()))?;
    let project_config = repo_root.join(".jigc").join("config");
    if !project_config.is_dir() {
        bail!(
            "this project isn't set up — run `jigc setup` (no `.jigc/config/` cascade layer found)"
        );
    }

    let pack = EmbeddedPack::new();
    compose_core(&repo_root, intent, &pack, workflow_id)
}

/// Compose the workflow named by `workflow_id` from `intent`, branching on the
/// workflow's `creates-task` flag — the engine spine both front-door forms drive
/// (the cascade-default `jigc start "<intent>"` passes the cascade-resolved
/// [`resolve_default_workflow`]; the explicit `--workflow <X>` form passes `X`),
/// with the pack injected so the no-task arm is reachable under test.
///
/// An unknown `workflow_id` — one the pack does not provide — is **rejected with a
/// routed finding before any mint** (`write-commands.md` → Form D): the membership
/// check is the pack read, a `PackError::NotFound` mapped to a routed
/// `workflow-refs.unknown-workflow` block, so a typo'd id never strands a task dir.
///
/// A `creates-task: true` work-workflow (e.g. `single-task`) **mints**: it opens
/// the task working area (reads HEAD), provisions the task's commit doc, and
/// binds the `task` context root, then composes with `{{task.intent}}` bound. A
/// `creates-task: false` workflow (the router and its kind) composes with **no
/// task context** — no mint, no working area, no commit doc, no commit-role bind:
/// it carries `task = None` and feeds only the selectable-workflow `catalog` to
/// composition (`write-commands.md` → Task origination, the `creates-task: false`
/// compose contract; `workflow-dialect.md` → Workflow selection). Either arm feeds
/// the cascade catalog **filtered to the selectable (`creates-task: true`) work-
/// workflows** into the context, so `{{catalog}}` resolves the same list the
/// router lists and never names itself.
fn compose_core(
    repo_root: &Path,
    intent: &str,
    pack: &dyn PackSource,
    workflow_id: &str,
) -> Result<ComposedWorkflow> {
    let workflow_bytes = read_workflow(pack, workflow_id)?;
    let def = load_workflow_def(&workflow_bytes).map_err(finding_to_err)?;
    let commands = load_catalog(pack)?;
    // The selectable-workflow list both arms feed to composition — the router's
    // `{{catalog}}` input, filtered to `creates-task: true` so it never lists
    // itself or any other `creates-task: false` workflow.
    let selectable = selectable_workflows(pack)?;
    // The committed managed store both arms feed — the `{{store.<doctype>}}` input,
    // enumerated from the committed `<location>/<slug>.md` instances (CLI locates,
    // engine resolves).
    let schemas = all_schemas(pack)?;
    let store = committed_store(repo_root, &schemas);

    let ctx = if def.creates_task {
        // Mint the task (reads HEAD). Minting after the definition loads so a
        // malformed pack never leaves a task dir behind.
        let minted = mint_in_repo(repo_root, intent, workflow_id)?;
        // Workflow-provisioned instance — a `creates-task` work-workflow
        // provisions the task's commit doc (the sink of its
        // `<<author: {{task.commit#summary}}>>` slot), so the agent only fills
        // slots (`write-commands.md` → Instance provisioning → Workflow-
        // provisioned). The empty skeleton stages here, ready for the
        // `jigc doc set-field`/`set-slot` write loop.
        provision_commit_doc(pack, &minted)?;
        // At mint there are no bound context roles yet (the agent binds them
        // in-task, e.g. an ADR via the create-gate); resume re-reads them.
        build_context(
            &minted.id,
            intent,
            &def,
            &RolesRecord::new(),
            selectable,
            store,
        )
    } else {
        // The `creates-task: false` compose contract: no mint, no working area,
        // no commit doc, no role binding — `task = None`. Intent threads forward
        // only via the re-run command's agent marker, never a resolved data-value;
        // any `task.*` reference here is the blocking
        // `workflow-refs.task-ref-in-no-task-workflow` conformance error.
        ComposeContext {
            task: None,
            catalog: selectable,
            store,
        }
    };
    let source = PackStepSource { pack };

    // Compose-time `workflow-refs` gate: validate every placeholder / include /
    // command-ref / marker before any output reaches the agent. A blocking
    // finding short-circuits.
    let findings = compose::workflow_refs(&workflow_bytes, &source, &commands, &ctx);
    if let Some(finding) = findings
        .into_iter()
        .find(|f| f.severity == Severity::Blocking)
    {
        return Err(finding_to_err(finding));
    }

    compose::compose(&def, &source, &commands, &ctx).map_err(finding_to_err)
}

/// Build the selectable-workflow catalog from `pack`: every workflow the pack
/// provides, parsed for its front-matter, **filtered to `creates-task: true`**
/// (the work-workflows a router selects among), each paired with its `when`
/// selection hint, in the pack's `list` order — the deterministic `catalog`
/// data-value root (`workflow-dialect.md` → data-value roots / Workflow
/// selection). A `creates-task: false` workflow (the router itself) is never a
/// selectable entry, so the router never lists itself. A selectable workflow that
/// declares no `when` is a definition bug, surfaced as a clear, id-bearing error.
pub(crate) fn selectable_workflows(pack: &dyn PackSource) -> Result<Vec<CatalogEntry>> {
    let mut entries = Vec::new();
    for id in pack.list(PackResourceKind::Workflows) {
        let bytes = read_pack(pack, PackResourceKind::Workflows, id.as_str())?;
        let def = load_workflow_def(&bytes).map_err(finding_to_err)?;
        if !def.creates_task {
            continue;
        }
        let when = def.when.filter(|w| !w.trim().is_empty()).with_context(|| {
            format!("selectable workflow `{id}` is missing the required `when` selection hint")
        })?;
        entries.push(CatalogEntry::new(id.as_str(), when));
    }
    Ok(entries)
}

/// Resume an existing task by `id` and re-compose its **own** minting workflow —
/// the `jigc start --task <id>` form (`design/write-commands.md` → Task-id
/// collision & resume: `--task` "resumes an existing task … where it is in *its*
/// workflow"; `DECISIONS.md` 2026-05-31 → Four `jigc start` forms; `DECISIONS.md`
/// 2026-06-01 → M2 Increment 3 re-cut: resume composes the persisted minting
/// workflow id, not the cascade default — so a `single-task` task still composes
/// `single-task` after the default flips to `router`).
///
/// Unlike [`compose_in_repo`] this **mints nothing**: it resolves the existing
/// `.jigc/tasks/<id>/` working area, reads its persisted state (base pin, the
/// original intent, the recorded minting workflow id, the bound context roles in
/// `roles.json`), verifies the task's
/// base still matches the current checkout (the CLI never operates a task off its
/// pinned base — `storage.md` → A task is pinned to its base), then re-composes
/// with the bound roles in scope. A role bound since the task was minted (e.g. an
/// ADR created in-task through the create-gate) now resolves in the composed view
/// — the surface the superseding-decision context-slice reads
/// (`worked-examples.md` → Superseding decision).
///
/// A nonexistent id rejects with `no task \`<id>\``; a base mismatch rejects with
/// the divergence-routing prompt (`write-commands.md` → Base mismatch on an
/// existing task).
pub fn resume_in_repo(start: &Path, id: &str) -> Result<ComposedWorkflow> {
    let repo_root = discover_repo_root(start)
        .with_context(|| format!("not inside a git repository (from {})", start.display()))?;
    let project_config = repo_root.join(".jigc").join("config");
    if !project_config.is_dir() {
        bail!(
            "this project isn't set up — run `jigc setup` (no `.jigc/config/` cascade layer found)"
        );
    }

    let task_dir = repo_root.join(".jigc").join("tasks").join(id);
    if !task_dir.is_dir() {
        bail!("no task `{id}` — list live tasks with `jigc start`");
    }

    // The task is pinned to its base; never operate it off its pinned commit.
    let pinned = state::read_base_pin(&task_dir)
        .with_context(|| format!("could not read the base pin for task `{id}`"))?;
    let head = read_head(&repo_root)?;
    if pinned.sha != head.sha {
        bail!(
            "task `{id}` is pinned to base {} but you're on {} — switch back with `git checkout {}` or `jigc task discard {id}`",
            pinned.short,
            head.short,
            pinned.short,
        );
    }

    let intent = state::read_intent(&task_dir)
        .with_context(|| format!("could not read intent for `{id}`"))?;
    let bound =
        RolesRecord::load(&task_dir).with_context(|| format!("could not read roles for `{id}`"))?;

    // Resume composes the task's **own** minting workflow, never the cascade
    // default (`DECISIONS.md` 2026-06-01 → M2 Increment 3 re-cut): a `single-task`
    // task resumed after the default flips to `router` must still compose
    // `single-task`. A working area with no recorded workflow id is a clear fault,
    // not a silent fall-through to the default.
    let pack = EmbeddedPack::new();
    let workflow_id = state::read_workflow_id(&task_dir)
        .with_context(|| format!("could not read the recorded workflow for `{id}`"))?
        .with_context(|| {
            format!(
                "task `{id}` has no recorded workflow — discard it with `jigc task discard {id}` and re-start with `jigc start`"
            )
        })?;
    let workflow_bytes = read_pack(&pack, PackResourceKind::Workflows, &workflow_id)?;
    let def = load_workflow_def(&workflow_bytes).map_err(finding_to_err)?;
    let commands = load_catalog(&pack)?;
    let selectable = selectable_workflows(&pack)?;
    // The committed store feed (`{{store.<doctype>}}`), enumerated from the committed
    // `<location>/<slug>.md` instances; the same `schemas` set the edge overlay below
    // resolves `<type>` prefixes against.
    let schemas = all_schemas(&pack)?;
    let store_feed = committed_store(&repo_root, &schemas);

    let ctx = build_context(id, &intent, &def, &bound, selectable, store_feed);
    let source = PackStepSource { pack: &pack };

    let findings = compose::workflow_refs(&workflow_bytes, &source, &commands, &ctx);
    if let Some(finding) = findings
        .into_iter()
        .find(|f| f.severity == Severity::Blocking)
    {
        return Err(finding_to_err(finding));
    }

    // Wire the committed store + edge overlay so a context-slice over a persisted ADR
    // (`{{@task.decision.supersedes#decision}}`) dereferences to the prior decision's
    // prose — the superseding-decision read path (`worked-examples.md` → Task 2). The
    // CLI locates the layers (committed-store root + the `.jigc/` index home), the
    // engine resolves through the `ContentStore` trait (`VISION.md` principle #4).
    let jigc_root = repo_root.join(".jigc");
    let committed = index::load_committed(&repo_root, &jigc_root, &schemas, &head.sha);
    let overlay = index::overlay_working(&committed, &task_dir, &schemas);
    let store = StoreContext {
        repo_root: &repo_root,
        schemas: &schemas,
        overlay: &overlay,
    };

    compose::compose_with_store(&def, &source, &commands, &ctx, Some(&store))
        .map_err(finding_to_err)
}

/// Enumerate the committed managed store into the `store` data-value feed: for every
/// schema that declares a `location:`, list its committed `<location>/<slug>.md`
/// instances and key them by the location's path-facing **collection name** (the
/// location stem, `specs/` → `specs`) — exactly the `store.specs` the
/// `locate-from-spec` step interpolates (`workflow-dialect.md` → data-value roots;
/// `DECISIONS.md` 2026-06-01 → the `store` root is keyed by the location stem). Each
/// value is the committed instances as `<type>:<slug>` addresses, in sorted (stable)
/// order.
///
/// This is the **CLI-locates** half of the determinism split (`VISION.md` principle
/// #4): the CLI walks the committed working tree (the same `<location>/<slug>.md`
/// surface `index::rebuild_committed` walks); the engine resolver consumes the fed
/// map and does no committed-store I/O. A transient (location-less) doctype — e.g.
/// `commit` — contributes nothing (it is never persisted as a repo file).
fn committed_store(
    repo_root: &Path,
    schemas: &BTreeMap<String, Schema>,
) -> BTreeMap<String, Vec<Address>> {
    let mut store: BTreeMap<String, Vec<Address>> = BTreeMap::new();
    for schema in schemas.values() {
        let Some(location) = schema.location.as_deref() else {
            continue; // a transient (location-less) type has no committed instances.
        };
        // The location's path-facing collection name — the trimmed final path
        // component (`specs/` → `specs`), the key authors write as `store.<name>`.
        let key = location.trim_matches('/');
        if key.is_empty() {
            continue;
        }
        let dir = repo_root.join(location);
        let Ok(entries) = std::fs::read_dir(&dir) else {
            continue; // no committed instances of this type yet.
        };
        let mut slugs: Vec<String> = entries
            .flatten()
            .map(|e| e.path())
            .filter(|p| p.extension().and_then(|x| x.to_str()) == Some("md"))
            .filter_map(|p| p.file_stem().and_then(|s| s.to_str()).map(str::to_owned))
            .collect();
        slugs.sort();
        let addresses: Vec<Address> = slugs
            .iter()
            .filter_map(|slug| Address::parse(&format!("{}:{slug}", schema.ty)).ok())
            .collect();
        if !addresses.is_empty() {
            store.insert(key.to_owned(), addresses);
        }
    }
    store
}

/// Load every shipped schema from the embedded pack, keyed by doctype — the
/// cascade-resolved schema set the committed store + edge overlay resolve `<type>`
/// prefixes against (`crate::task` loads the same set for the finalize sweep).
fn all_schemas(pack: &dyn PackSource) -> Result<BTreeMap<String, Schema>> {
    let mut out = BTreeMap::new();
    for id in pack.list(PackResourceKind::Schemas) {
        let bytes = read_pack(pack, PackResourceKind::Schemas, id.as_str())?;
        let schema = load_schema(&bytes)
            .with_context(|| format!("the `{}` schema is malformed", id.as_str()))?;
        out.insert(schema.ty.clone(), schema);
    }
    Ok(out)
}

/// Build the [`ComposeContext`] for the composed workflow.
///
/// Binds the engine-native `task.id`/`task.intent` scalars, the task's **commit**
/// role to `commit:<id>` (the always-present sink of a `creates-task` work-
/// workflow — its `<<author: {{task.commit#summary}}>>` slot must resolve to that
/// address), and declares each `allows-create` **and** `reads` role from the
/// workflow front-matter. A role **bound** in `roles` (persisted in the task's
/// `roles.json` once the agent created the doc through the create-gate or ran
/// `jigc task bind` for a `reads` role) binds to its recorded address; an
/// **unbound** declared role declares as `None`, so `{{@task.<role>.…}}` resolves
/// to absent/empty (`design/workflow-dialect.md` → Empty vs unresolvable). At mint
/// `roles` is empty (nothing is bound yet); on `--task <id>` resume it carries
/// the binds recorded since (`DECISIONS.md` 2026-05-31 → inc-5 `as:` role binding
/// at create).
fn build_context(
    id: &str,
    intent: &str,
    def: &WorkflowDef,
    bound: &RolesRecord,
    catalog: Vec<CatalogEntry>,
    store: BTreeMap<String, Vec<Address>>,
) -> ComposeContext {
    let mut roles: BTreeMap<String, Option<Address>> = BTreeMap::new();
    // The task's commit doc — the engine-native sink, bound to `commit:<id>`.
    let commit =
        Address::parse(&format!("commit:{id}")).expect("`commit:<slug>` is a valid address");
    roles.insert("commit".to_owned(), Some(commit));
    // Each `allows-create` role is declared; bound iff `roles.json` recorded it.
    for entry in &def.allows_create {
        let binding = bound
            .get(&entry.as_role)
            .and_then(|addr| Address::parse(addr).ok());
        roles.entry(entry.as_role.clone()).or_insert(binding);
    }
    // Each `reads` role — the dual of `allows-create` — is declared the same way:
    // bound iff `roles.json` recorded a `jigc task bind` for it, else `None` so
    // `{{@task.<role>.…}}` resolves to absent/empty until bound
    // (`workflow-dialect.md` → `reads`: declared-but-unbound resolves to empty).
    for entry in &def.reads {
        let binding = bound
            .get(&entry.role)
            .and_then(|addr| Address::parse(addr).ok());
        roles.entry(entry.role.clone()).or_insert(binding);
    }
    ComposeContext {
        task: Some(TaskRoot {
            id: id.to_owned(),
            intent: intent.to_owned(),
            roles,
        }),
        // The selectable-workflow catalog, fed identically to the no-task arm so a
        // `creates-task: true` workflow that interpolates `{{catalog}}` resolves
        // the same list (it never lists itself — the filter excludes it only when
        // it is `creates-task: false`).
        catalog,
        // The committed managed store, fed by the CLI so `{{store.<doctype>}}`
        // resolves to the committed instances (the determinism boundary; the
        // engine resolver does no committed-store I/O).
        store,
    }
}

/// A [`StepSource`] that resolves step ids against the embedded pack. The MVP
/// cascade has no project step overrides, so the pack-default layer owns every
/// step — the engine consumes this mapping and stays a pure function of it.
struct PackStepSource<'a> {
    pack: &'a dyn PackSource,
}

impl StepSource for PackStepSource<'_> {
    fn step(&self, id: &str) -> Option<StepDef> {
        let bytes = self
            .pack
            .read(PackResourceKind::Steps, &ResourceId::from(id))
            .ok()?;
        load_step_def(id, &bytes).ok()
    }
}

/// Resolve the `default-workflow` id **through the cascade** — the live read the
/// bare front door composes (`overrides.md` → Scalar knobs / Resolution
/// algorithm). Seeds the [`PackDefaultLayer`] scalar surface from the pack's
/// `config/knobs` (closed key set + materialized defaults), loads the project
/// [`OverrideLayer`] from `<project_config>/manifest.yaml`, resolves the cascade,
/// and reads the key via [`cascade::Resolved::scalar_required`] — so a project
/// `scalar: default-workflow: …` delta *flips* which workflow mints, while a
/// no-override cascade reads the pack default unchanged.
///
/// The read goes through `scalar_required`, not [`cascade::Resolved::scalar`]:
/// the compose path is byte-safe only over the declared, seeded surface, so an
/// undeclared read-key is a hard [`cascade::CascadeError::UndeclaredComposeRead`]
/// here, never a silent `None`/raw fallback (the read-side determinism invariant).
fn resolve_default_workflow(pack: &dyn PackSource, project_config: &Path) -> Result<String> {
    let knobs_bytes = read_pack(pack, PackResourceKind::Config, "knobs")?;
    let knobs = engine::knobs::load_knobs(&knobs_bytes).context("`config/knobs` is malformed")?;
    let pack_default = PackDefaultLayer::new(
        pack_id_from_config(pack)?,
        pack.pack_version(),
        knobs.base_scalars(),
        Vec::new(),
    );
    let project = load_project_layer(project_config)?;
    let resolved = cascade::resolve(&pack_default, None, Some(&project))?;
    resolved
        .scalar_required(DEFAULT_WORKFLOW_KEY)
        .map(str::to_owned)
        .map_err(anyhow::Error::from)
}

/// Read the pack's own cascade id from its `config/defaults` `pack-id` field —
/// the identity the [`PackDefaultLayer`] carries (`overrides.md`: the pack-default
/// layer names itself; `pack-id` is identity, never a knob).
fn pack_id_from_config(pack: &dyn PackSource) -> Result<String> {
    let bytes = read_pack(pack, PackResourceKind::Config, "defaults")?;
    let text = String::from_utf8(bytes).context("`config/defaults` is not UTF-8")?;
    let value: serde_yaml_ng::Value =
        serde_yaml_ng::from_str(&text).context("`config/defaults` is not valid YAML")?;
    value
        .get("pack-id")
        .and_then(serde_yaml_ng::Value::as_str)
        .map(str::to_owned)
        .context("`config/defaults` declares no `pack-id`")
}

/// Load the project cascade layer from `<project_config>/manifest.yaml`, reading
/// **only** its top-level `scalar:` block this increment (the `deltas:` list is
/// increments 2–5). Each `scalar:` entry becomes one
/// [`OverrideLayer::scalar_set`], recorded **in manifest order** so within-layer
/// application order is preserved (`overrides.md` → Delta representation; phase 3
/// / Within-layer manifest order). The resulting layer, resolved over the base,
/// applies the project's `scalar-set` deltas — flipping a knob like
/// `default-workflow` the cascade reads on the compose path.
///
/// A **missing** manifest file, or a present manifest with a **missing or blank**
/// `scalar:` block, yields [`OverrideLayer::empty()`] — present-but-empty is the
/// no-override path the determinism invariant keeps byte-identical to today. A
/// non-string scalar value (`true`, `3`) is recorded as its YAML scalar string;
/// the engine treats scalar values as opaque strings here, with typed adjudication
/// owned by the `jigc config set` write path's `check_value` (T6).
///
/// The closed-surface check (an undeclared key is an error) is **not** done here:
/// it is the resolver's job (`cascade::resolve` rejects an undeclared
/// `scalar-set`), so a hand-edited manifest and a `config set` write are
/// adjudicated by the one path.
pub(crate) fn load_project_layer(project_config: &Path) -> Result<OverrideLayer> {
    let manifest = project_config.join("manifest.yaml");
    // Always stamp the committed-config path (the provenance-header segment, the
    // same path the orient view shows), whichever arm builds the layer.
    let with_path = |layer: OverrideLayer| layer.config_path(project_config.display().to_string());

    let text = match std::fs::read_to_string(&manifest) {
        Ok(text) => text,
        // No manifest file → the no-override path (present-but-empty layer).
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
            return Ok(with_path(OverrideLayer::empty()));
        }
        Err(e) => {
            return Err(e).with_context(|| format!("could not read {}", manifest.display()));
        }
    };
    let doc: serde_yaml_ng::Value = serde_yaml_ng::from_str(&text)
        .with_context(|| format!("{} is not valid YAML", manifest.display()))?;

    let mut layer = OverrideLayer::empty();
    // Only the `scalar:` block this increment. A **blank** block (`scalar:` with no
    // children, parsed as null) is the no-override path — not an error — so the key
    // present-but-empty leaves the layer empty exactly like an absent block.
    match doc.get("scalar") {
        None | Some(serde_yaml_ng::Value::Null) => {}
        Some(scalar) => {
            let map = scalar.as_mapping().with_context(|| {
                format!(
                    "{}: `scalar:` must be a map of knob keys",
                    manifest.display()
                )
            })?;
            for (key, value) in map {
                let key = key.as_str().with_context(|| {
                    format!("{}: `scalar:` keys must be strings", manifest.display())
                })?;
                layer = layer.scalar_set(key, scalar_to_string(value));
            }
        }
    }
    Ok(with_path(layer))
}

/// Render a YAML scalar `value` to the opaque string the cascade stores. A string
/// scalar passes through verbatim; a bool / int / other scalar renders to its YAML
/// text (`true`, `3`). A non-scalar (map / sequence) is not a valid knob value and
/// renders to its YAML form, which the resolver / `check_value` then rejects.
fn scalar_to_string(value: &serde_yaml_ng::Value) -> String {
    match value {
        serde_yaml_ng::Value::String(s) => s.clone(),
        serde_yaml_ng::Value::Bool(b) => b.to_string(),
        serde_yaml_ng::Value::Number(n) => n.to_string(),
        other => serde_yaml_ng::to_string(other)
            .unwrap_or_default()
            .trim_end()
            .to_owned(),
    }
}

/// Load and parse the pack's command catalog (`config/commands`).
fn load_catalog(pack: &dyn PackSource) -> Result<CommandCatalog> {
    let bytes = read_pack(pack, PackResourceKind::Config, "commands")?;
    load_command_catalog(&bytes).map_err(finding_to_err)
}

/// Read a named workflow's bytes, mapping a **missing** workflow to a routed
/// blocking finding — the Form-D unknown-`<X>` rejection (`write-commands.md`: an
/// unknown `<X>` is rejected with a routed finding). Membership is the pack read:
/// a `PackError::NotFound` is the "not in the catalog" rejection. Fires before any
/// mint, so a typo'd id strands no task dir.
fn read_workflow(pack: &dyn PackSource, id: &str) -> Result<Vec<u8>> {
    pack.read(PackResourceKind::Workflows, &ResourceId::from(id))
        .map_err(|_| {
            finding_to_err(Finding::block(
                "workflow-refs.unknown-workflow",
                format!("no workflow `{id}` — list the selectable work-workflows with `jigc start`"),
                "run `jigc start` to see the selectable work-workflows, then re-run `jigc start --workflow <id> \"<intent>\"`",
            ))
        })
}

/// Read a pack resource by kind + id, mapping a missing resource to an error.
fn read_pack(pack: &dyn PackSource, kind: PackResourceKind, id: &str) -> Result<Vec<u8>> {
    pack.read(kind, &ResourceId::from(id))
        .with_context(|| format!("the embedded pack is missing `{id}`"))
}

/// Map an engine [`Finding`] to an `anyhow` error carrying its message + route —
/// the same envelope minting already uses (a hard block is a blocking-severity
/// finding carrying a route, `DECISIONS.md` 2026-05-31).
fn finding_to_err(finding: Finding) -> anyhow::Error {
    let route = finding
        .route
        .map(|r| format!("\n  route: {r}"))
        .unwrap_or_default();
    anyhow::anyhow!("{}{route}", finding.message)
}

/// Read HEAD as a [`BasePin`] (full + short SHA) by shelling out to the user's
/// `git` (`DECISIONS.md` 2026-05-31 → Git invocation: shell out to the `git`
/// binary, not git2/gitoxide).
fn read_head(repo_root: &Path) -> Result<BasePin> {
    let sha = git_rev_parse(repo_root, &["rev-parse", "HEAD"])?;
    let short = git_rev_parse(repo_root, &["rev-parse", "--short", "HEAD"])?;
    Ok(BasePin::new(sha, short))
}

/// Run `git <args>` in `repo_root` and return the single trimmed line of stdout.
fn git_rev_parse(repo_root: &Path, args: &[&str]) -> Result<String> {
    let out = Command::new("git")
        .args(args)
        .current_dir(repo_root)
        .output()
        .context("could not run `git` (is it on PATH?)")?;
    if !out.status.success() {
        anyhow::bail!(
            "`git {}` failed: {}",
            args.join(" "),
            String::from_utf8_lossy(&out.stderr).trim(),
        );
    }
    let line = String::from_utf8(out.stdout)
        .context("`git` produced non-UTF-8 output")?
        .trim()
        .to_string();
    Ok(line)
}

/// Walk up from `start` to the directory holding `.git` (the repo root) — the
/// same discovery [`crate::locate`] does, kept local so minting needs no
/// `RunContext`.
fn discover_repo_root(start: &Path) -> Option<std::path::PathBuf> {
    start
        .ancestors()
        .find(|dir| dir.join(".git").exists())
        .map(std::path::PathBuf::from)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::path::PathBuf;

    /// A throwaway directory that removes itself on drop.
    struct TempDir(PathBuf);

    impl TempDir {
        fn new(tag: &str) -> Self {
            let mut path = std::env::temp_dir();
            let unique = format!(
                "jigc-start-mint-{tag}-{}-{:?}",
                std::process::id(),
                std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap()
                    .as_nanos(),
            );
            path.push(unique);
            fs::create_dir_all(&path).expect("create temp dir");
            TempDir(path)
        }

        fn path(&self) -> &Path {
            &self.0
        }
    }

    impl Drop for TempDir {
        fn drop(&mut self) {
            let _ = fs::remove_dir_all(&self.0);
        }
    }

    /// Initialize a real git repo with one commit, returning its HEAD SHAs.
    fn init_repo_with_commit(root: &Path) -> (String, String) {
        let git = |args: &[&str]| {
            let out = Command::new("git")
                .args(args)
                .current_dir(root)
                .output()
                .expect("run git");
            assert!(
                out.status.success(),
                "git {args:?} failed: {}",
                String::from_utf8_lossy(&out.stderr)
            );
            String::from_utf8(out.stdout)
                .expect("utf-8")
                .trim()
                .to_string()
        };
        git(&["init", "-q"]);
        git(&["config", "user.email", "test@example.com"]);
        git(&["config", "user.name", "Test"]);
        fs::write(root.join("README.md"), "hello\n").expect("write file");
        git(&["add", "."]);
        git(&["commit", "-q", "-m", "initial"]);
        let sha = git(&["rev-parse", "HEAD"]);
        let short = git(&["rev-parse", "--short", "HEAD"]);
        (sha, short)
    }

    /// The CLI integration done-criterion: minting under a real temp git repo
    /// creates the working dir and a base pin recording the repo's actual HEAD.
    #[test]
    fn mint_opens_working_dir_and_base_pin_under_a_temp_git_repo() {
        let repo = TempDir::new("repo");
        let (sha, short) = init_repo_with_commit(repo.path());

        let minted =
            mint_in_repo(repo.path(), "Add rate limiter", "single-task").expect("mint succeeds");

        assert_eq!(minted.id, "add-rate-limiter");
        let dir = repo
            .path()
            .join(".jigc")
            .join("tasks")
            .join("add-rate-limiter");
        assert!(dir.is_dir(), "working dir must appear under .jigc/tasks/");
        assert_eq!(minted.dir, dir);

        let pin = fs::read_to_string(dir.join("base.json")).expect("base pin written");
        let base: BasePin = serde_json::from_str(&pin).expect("pin parses");
        assert_eq!(base.sha, sha, "base pin records HEAD's full SHA");
        assert_eq!(base.short, short, "base pin records HEAD's short SHA");

        // A serial re-mint of the same intent rejects, surfacing the route.
        let err = mint_in_repo(repo.path(), "Add rate limiter", "single-task")
            .expect_err("re-mint rejects");
        let msg = err.to_string();
        assert!(
            msg.contains("add-rate-limiter") && msg.contains("route:"),
            "serial collision must name the task and carry a route; got: {msg}"
        );
    }

    /// An in-memory [`PackSource`] seeded from `(kind, id, bytes)` triples — lets a
    /// unit test drive [`compose_core`] over a fixture cascade with no embedded
    /// pack and no filesystem.
    struct FixturePack(std::collections::HashMap<(PackResourceKind, ResourceId), Vec<u8>>);

    impl FixturePack {
        fn with(triples: Vec<(PackResourceKind, &str, &str)>) -> Self {
            let map = triples
                .into_iter()
                .map(|(kind, id, body)| ((kind, ResourceId::from(id)), body.as_bytes().to_vec()))
                .collect();
            FixturePack(map)
        }
    }

    impl PackSource for FixturePack {
        fn pack_version(&self) -> String {
            "0.0.0".to_owned()
        }

        fn list(&self, kind: PackResourceKind) -> Vec<ResourceId> {
            let mut ids: Vec<ResourceId> = self
                .0
                .keys()
                .filter(|(k, _)| *k == kind)
                .map(|(_, id)| id.clone())
                .collect();
            ids.sort();
            ids
        }

        fn read(
            &self,
            kind: PackResourceKind,
            id: &ResourceId,
        ) -> Result<Vec<u8>, engine::packsource::PackError> {
            self.0.get(&(kind, id.clone())).cloned().ok_or_else(|| {
                engine::packsource::PackError::NotFound {
                    kind,
                    id: id.clone(),
                }
            })
        }
    }

    /// The T4 done-criterion: composing a `creates-task: false` default workflow
    /// (a router whose body interpolates `{{catalog}}`) mints **nothing** — no
    /// `.jigc/tasks/<id>/` working area appears — and its output lists exactly the
    /// **selectable** (`creates-task: true`) work-workflows, never the router
    /// itself. The `repo_root` is an empty temp dir: the no-task arm never reads
    /// HEAD, so no git is needed (proof the arm carries no task context).
    #[test]
    fn no_task_workflow_composes_the_catalog_without_minting() {
        let repo = TempDir::new("no-task");

        let pack = FixturePack::with(vec![
            (
                PackResourceKind::Config,
                "defaults",
                "pack-id: dev\ndefault-workflow: router\n",
            ),
            (PackResourceKind::Config, "commands", "commands: []\n"),
            (
                PackResourceKind::Workflows,
                "router",
                "---\nwhen: help me pick a workflow\ncreates-task: false\n---\n{{ include: step:route }}\n",
            ),
            (
                PackResourceKind::Workflows,
                "single-task",
                "---\nwhen: implement one scoped change\ncreates-task: true\n---\n{{ include: step:noop }}\n",
            ),
            (
                PackResourceKind::Workflows,
                "quick-fix",
                "---\nwhen: a small focused fix\ncreates-task: true\n---\n{{ include: step:noop }}\n",
            ),
            (
                PackResourceKind::Steps,
                "route",
                "Pick one of the work-workflows below:\n\n{{ catalog }}\n",
            ),
            (PackResourceKind::Steps, "noop", "no-op body\n"),
        ]);

        // `compose_core` is driven with the workflow id directly here — the
        // cascade read (`resolve_default_workflow`) is exercised by the binary-
        // driven flip + byte-identical goldens (`tests/start_compose.rs`).
        let composed =
            compose_core(repo.path(), "anything", &pack, "router").expect("no-task compose");

        // (a) The no-task arm mints nothing: no working area is opened.
        assert!(
            !repo.path().join(".jigc").join("tasks").exists(),
            "a `creates-task: false` compose must not open any `.jigc/tasks/` dir",
        );

        // (b) `{{catalog}}` resolves to the selectable work-workflows, each as a
        // `- <id> — <when>` option line — and the router never lists itself.
        assert!(
            composed
                .text
                .contains("- single-task — implement one scoped change"),
            "the catalog must list single-task; got:\n{}",
            composed.text,
        );
        assert!(
            composed.text.contains("- quick-fix — a small focused fix"),
            "the catalog must list quick-fix; got:\n{}",
            composed.text,
        );
        assert!(
            !composed.text.contains("- router —"),
            "the router must not list itself (creates-task: false is filtered); got:\n{}",
            composed.text,
        );
    }

    /// A minimal but valid `commit` schema (`type: commit`, a header section + a
    /// `summary` slot) — enough for `provision_commit_doc` to render a fillable
    /// form when a `creates-task: true` workflow mints under Form D.
    const COMMIT_SCHEMA: &str = "type: commit\nsections:\n  - id: header\n    header: true\n    fields:\n      - { id: type, type: string }\n  - id: summary\n    slot: {}\n";

    /// The pack a Form-D test composes over: a `creates-task: true` `single-task`,
    /// a `creates-task: false` `router`, the `commit` schema minting provisions,
    /// and the no-op step both workflow bodies include. `default-workflow` points
    /// at the router so a stray cascade-default read would be observably wrong —
    /// Form D must compose the *named* workflow, not the default.
    fn form_d_pack() -> FixturePack {
        FixturePack::with(vec![
            (
                PackResourceKind::Config,
                "defaults",
                "pack-id: dev\ndefault-workflow: router\n",
            ),
            (PackResourceKind::Config, "commands", "commands: []\n"),
            (PackResourceKind::Schemas, "commit", COMMIT_SCHEMA),
            (
                PackResourceKind::Workflows,
                "router",
                "---\nwhen: help me pick a workflow\ncreates-task: false\n---\n{{ include: step:route }}\n",
            ),
            (
                PackResourceKind::Workflows,
                "single-task",
                "---\nwhen: implement one scoped change\ncreates-task: true\n---\n{{ include: step:noop }}\n",
            ),
            (
                PackResourceKind::Steps,
                "route",
                "Pick one of the work-workflows below:\n\n{{ catalog }}\n",
            ),
            (PackResourceKind::Steps, "noop", "no-op body\n"),
        ])
    }

    /// The T2 done-criterion (a): Form D over a **`creates-task: true`** named
    /// workflow opens `.jigc/tasks/<slug>/` and composes its body. The named
    /// workflow is `single-task` while `default-workflow` is the router, so a
    /// successful mint proves the *named* workflow composed, not the default.
    #[test]
    fn form_d_over_a_creates_task_workflow_mints_and_composes() {
        let repo = TempDir::new("form-d-mint");
        init_repo_with_commit(repo.path());

        let pack = form_d_pack();
        let composed = compose_core(repo.path(), "Add rate limiter", &pack, "single-task")
            .expect("Form-D compose of a creates-task workflow");

        let dir = repo
            .path()
            .join(".jigc")
            .join("tasks")
            .join("add-rate-limiter");
        assert!(
            dir.is_dir(),
            "a creates-task: true Form-D workflow must open .jigc/tasks/<slug>/",
        );
        assert!(
            composed.text.contains("no-op body"),
            "the named workflow's body must compose; got:\n{}",
            composed.text,
        );
    }

    /// The T2 done-criterion (b): a `<X>` the pack does not provide rejects with a
    /// routed finding, and **no** `.jigc/tasks/` dir is created — the rejection
    /// precedes minting. The repo is a bare temp dir: rejection fires before HEAD
    /// is ever read.
    #[test]
    fn form_d_unknown_workflow_rejects_before_minting() {
        let repo = TempDir::new("form-d-unknown");

        let pack = form_d_pack();
        let err = compose_core(repo.path(), "Add rate limiter", &pack, "does-not-exist")
            .expect_err("an unknown --workflow id must reject");

        let msg = err.to_string();
        assert!(
            msg.contains("does-not-exist") && msg.contains("route:"),
            "the rejection must name the unknown id and carry a route; got: {msg}",
        );
        assert!(
            !repo.path().join(".jigc").join("tasks").exists(),
            "rejection must precede minting — no .jigc/tasks/ dir may be created",
        );
    }

    /// The T2 done-criterion: a `reads`-declared role is **seeded into the compose
    /// role map** so `task.<role>` is a valid `workflow-refs` root that resolves to
    /// [`Resolution::Absent`] (empty text) when **unbound**, while an **undeclared**
    /// `task.<other>` still raises `workflow-refs.undeclared-role`
    /// (`workflow-dialect.md` → `reads`: declared-but-unbound resolves to empty,
    /// undeclared is a conformance error).
    #[test]
    fn reads_role_seeds_the_compose_map_and_resolves_empty_when_unbound() {
        use engine::compose::Reads;
        use engine::data_value::{Path, Resolution};

        let def = WorkflowDef {
            when: Some("implement from a spec".to_owned()),
            creates_task: true,
            allows_create: vec![],
            reads: vec![Reads {
                role: "spec".to_owned(),
                doc_type: "spec".to_owned(),
            }],
            includes: vec![],
        };
        // No role is bound yet — the task carries the declaration only.
        let ctx = build_context(
            "add-rate-limiter",
            "Add rate limiter",
            &def,
            &RolesRecord::new(),
            Vec::new(),
            BTreeMap::new(),
        );

        // The declared-but-unbound `reads` role resolves to absent/empty text — no
        // `undeclared-role` finding.
        let resolved = Path::parse("task.spec")
            .expect("valid path")
            .resolve(&ctx)
            .expect("a declared role must not raise a finding");
        assert_eq!(
            resolved,
            Resolution::Absent,
            "an unbound declared `reads` role must resolve to Absent (empty text)",
        );

        // An undeclared role under the same task still raises the conformance error.
        let undeclared = Path::parse("task.other")
            .expect("valid path")
            .resolve(&ctx)
            .expect_err("an undeclared role must raise a finding");
        assert_eq!(
            undeclared.code, "workflow-refs.undeclared-role",
            "an undeclared `task.<role>` must stay a conformance error; got {}",
            undeclared.code,
        );
    }

    /// `committed_store` enumerates the committed `<location>/<slug>.md` instances
    /// into the `store` feed, keyed by the location stem (`specs/` → `specs`) — the
    /// `store.specs` collection the `locate-from-spec` step interpolates. A transient
    /// (location-less) doctype (`commit`) contributes nothing; a doctype with a
    /// declared location but no committed files is omitted (the empty-store stance).
    #[test]
    fn committed_store_enumerates_by_location_stem() {
        let repo = TempDir::new("committed-store");
        let specs = repo.path().join("specs");
        fs::create_dir_all(&specs).expect("mk specs/");
        fs::write(specs.join("gateway-rate-limiting.md"), "# Gateway\n").expect("w");
        fs::write(specs.join("auth-token-rotation.md"), "# Auth\n").expect("w");
        // A non-`.md` file is ignored.
        fs::write(specs.join("notes.txt"), "ignore me").expect("w");

        // The real pack schemas: `spec` (location `specs/`), `adr` (`decisions/`,
        // no committed files here), `commit` (transient — no location).
        let pack = crate::pack::EmbeddedPack::new();
        let schemas = all_schemas(&pack).expect("schemas load");

        let store = committed_store(repo.path(), &schemas);

        assert_eq!(
            store.get("specs").map(Vec::as_slice),
            Some(
                [
                    Address::parse("spec:auth-token-rotation").expect("valid"),
                    Address::parse("spec:gateway-rate-limiting").expect("valid"),
                ]
                .as_slice()
            ),
            "`store.specs` lists the committed spec instances (sorted, `<type>:<slug>`)",
        );
        // A declared-location doctype with no committed files is absent (empty store).
        assert!(
            !store.contains_key("decisions"),
            "a doctype with no committed instances must be omitted; got {store:?}",
        );
        // The transient `commit` type (no `location:`) never appears.
        assert!(
            !store.keys().any(|k| k == "commit"),
            "a transient (location-less) doctype contributes nothing; got {store:?}",
        );
    }

    /// The fillable form pre-stamps required header fields but **omits** an optional
    /// `ref` (`card: 0..1`): a spec-less `commit` is provisioned without an empty
    /// `implements:` line, so it finalizes cleanly (no malformed-empty value, no
    /// dangling edge). Pins the provisioning side of the `commit.implements` optional
    /// ref against the shipped `commit.yaml`.
    #[test]
    fn fillable_form_omits_an_optional_ref_but_keeps_required_fields() {
        use engine::field_block::Value;
        use engine::schema::SectionBody;

        let schema = engine::schema::load_schema(
            crate::pack::EmbeddedPack::new()
                .read(PackResourceKind::Schemas, &ResourceId::from("commit"))
                .expect("commit schema")
                .as_slice(),
        )
        .expect("commit.yaml loads");

        let instance = fillable_form(&schema, "add-rate-limiter");
        let header = instance
            .sections
            .iter()
            .find(|s| s.id == "header")
            .expect("header section");
        let keys: Vec<&str> = header.fields.iter().map(|f| f.key.as_str()).collect();

        assert!(
            keys.contains(&"type") && keys.contains(&"scope"),
            "required header fields stay pre-stamped; got {keys:?}",
        );
        assert!(
            !keys.contains(&"implements"),
            "the optional `implements` ref must not be pre-stamped; got {keys:?}",
        );

        // The omission survives the render → no `implements:` front-matter line.
        let rendered = engine::write::render(&schema, &instance);
        assert!(
            !rendered.contains("implements:"),
            "the provisioned form must carry no empty `implements:` line; got:\n{rendered}",
        );

        // Sanity: the schema *does* declare `implements` as an optional ref, so the
        // test would catch a regression that pre-stamps it again.
        let SectionBody::Simple { fields, .. } = &schema
            .sections
            .iter()
            .find(|s| s.id == "header")
            .unwrap()
            .body
        else {
            panic!("header is a simple section");
        };
        let implements = fields.iter().find(|f| f.id == "implements").unwrap();
        assert!(is_optional_ref(implements));
        // The pre-stamped required fields carry an empty scalar (the fillable line).
        assert_eq!(header.fields[0].value, Value::Scalar(String::new()));
    }

    /// A pack-default base layer declaring `default-workflow: single-task` — the
    /// base a project `scalar-set` resolves over to prove the delta *applies*.
    fn base_layer() -> engine::cascade::PackDefaultLayer {
        let mut scalars = BTreeMap::new();
        scalars.insert("default-workflow".to_owned(), "single-task".to_owned());
        engine::cascade::PackDefaultLayer::new("dev", "0.0.0", scalars, Vec::new())
    }

    /// The T3 done-criterion: a hand-authored manifest with a top-level `scalar:`
    /// map loads into an `OverrideLayer` whose `scalar-set` delta, resolved over the
    /// base, **flips** the resolved value (`single-task` → `router`).
    #[test]
    fn manifest_scalar_block_flips_the_resolved_value() {
        let cfg = TempDir::new("manifest-flip");
        fs::write(
            cfg.path().join("manifest.yaml"),
            "scalar:\n  default-workflow: router\n",
        )
        .expect("write manifest");

        let layer = load_project_layer(cfg.path()).expect("manifest loads");
        let resolved =
            engine::cascade::resolve(&base_layer(), None, Some(&layer)).expect("resolves");

        assert_eq!(
            resolved.scalar("default-workflow"),
            Some("router"),
            "the project `scalar:` delta must flip the resolved value over the base",
        );
    }

    /// A manifest with **no** `scalar:` block yields the no-override path: resolved
    /// over the base, the base value is unchanged (`OverrideLayer::empty()`).
    #[test]
    fn manifest_without_scalar_block_yields_empty_layer() {
        let cfg = TempDir::new("manifest-no-scalar");
        fs::write(
            cfg.path().join("manifest.yaml"),
            "deltas: []\n", // a `deltas:`-only manifest carries no scalar block.
        )
        .expect("write manifest");

        let layer = load_project_layer(cfg.path()).expect("manifest loads");
        let resolved =
            engine::cascade::resolve(&base_layer(), None, Some(&layer)).expect("resolves");

        assert_eq!(
            resolved.scalar("default-workflow"),
            Some("single-task"),
            "a manifest with no `scalar:` block must leave the base value untouched",
        );
    }

    /// A **blank** `scalar:` block (the key present but empty) is the no-override
    /// path too — `serde_yaml_ng` parses `scalar:` with no children as null, which
    /// is not a mapping, so the layer stays empty and the base value survives.
    #[test]
    fn manifest_with_blank_scalar_block_yields_empty_layer() {
        let cfg = TempDir::new("manifest-blank-scalar");
        fs::write(cfg.path().join("manifest.yaml"), "scalar:\n").expect("write manifest");

        let layer = load_project_layer(cfg.path()).expect("manifest loads");
        let resolved =
            engine::cascade::resolve(&base_layer(), None, Some(&layer)).expect("resolves");

        assert_eq!(
            resolved.scalar("default-workflow"),
            Some("single-task"),
            "a blank `scalar:` block must leave the base value untouched",
        );
    }

    /// A **missing** manifest file (the project layer dir exists but holds no
    /// `manifest.yaml`) yields the no-override path — present-but-empty, not an
    /// error: the layer is the cascade's no-delta base reader.
    #[test]
    fn missing_manifest_file_yields_empty_layer() {
        let cfg = TempDir::new("manifest-missing");
        // No manifest.yaml written.

        let layer = load_project_layer(cfg.path()).expect("missing manifest is not an error");
        let resolved =
            engine::cascade::resolve(&base_layer(), None, Some(&layer)).expect("resolves");

        assert_eq!(
            resolved.scalar("default-workflow"),
            Some("single-task"),
            "a missing manifest must resolve to the base value (no-override path)",
        );
    }

    /// Every `scalar:` entry is recorded as a `scalar-set` delta: a manifest setting
    /// two declared knobs resolves both over the base. Pins that the loader walks the
    /// whole `scalar:` map (in manifest order, the iteration order `serde_yaml_ng`'s
    /// mapping preserves), not just the first entry.
    #[test]
    fn manifest_records_every_scalar_entry() {
        let mut scalars = BTreeMap::new();
        scalars.insert("default-workflow".to_owned(), "single-task".to_owned());
        scalars.insert(
            "validation.file-state.severity".to_owned(),
            "blocking".to_owned(),
        );
        let base = engine::cascade::PackDefaultLayer::new("dev", "0.0.0", scalars, Vec::new());

        let cfg = TempDir::new("manifest-multi");
        fs::write(
            cfg.path().join("manifest.yaml"),
            "scalar:\n  default-workflow: router\n  validation.file-state.severity: warning\n",
        )
        .expect("write manifest");

        let layer = load_project_layer(cfg.path()).expect("manifest loads");
        let resolved = engine::cascade::resolve(&base, None, Some(&layer)).expect("resolves");

        assert_eq!(resolved.scalar("default-workflow"), Some("router"));
        assert_eq!(
            resolved.scalar("validation.file-state.severity"),
            Some("warning"),
            "every `scalar:` entry must be recorded, not just the first",
        );
    }

    /// The read-side determinism invariant, end-to-end through the compose read:
    /// `resolve_default_workflow` reads `default-workflow` via `scalar_required`,
    /// so a pack whose `config/knobs` **does not declare** `default-workflow`
    /// fails **loudly** (the `UndeclaredComposeRead` hard error) rather than
    /// silently composing nothing. Pins that the wired compose read never falls
    /// back to a raw `None` for an undeclared key.
    #[test]
    fn undeclared_compose_read_key_fails_loudly() {
        let cfg = TempDir::new("undeclared-read");
        fs::create_dir_all(cfg.path()).expect("mk config dir");

        // A pack whose knob surface omits `default-workflow` entirely — so the
        // cascade-resolved surface never carries it.
        let pack = FixturePack::with(vec![
            (PackResourceKind::Config, "defaults", "pack-id: dev\n"),
            (
                PackResourceKind::Config,
                "knobs",
                "some-other-knob:\n  type: string\n  default: x\n",
            ),
        ]);

        let err = resolve_default_workflow(&pack, cfg.path())
            .expect_err("an undeclared compose-read key must fail loudly");
        let msg = err.to_string();
        assert!(
            msg.contains("default-workflow") && msg.contains("undeclared"),
            "the read of an undeclared key must name it and the closed-surface rule; got: {msg}",
        );
    }

    /// A non-string scalar value (`bool`) is recorded as its YAML scalar string
    /// (`true`) — the cascade stores opaque strings; typed adjudication is the
    /// `config set` write path (T6), not this loader.
    #[test]
    fn manifest_non_string_scalar_renders_to_its_yaml_string() {
        let mut scalars = BTreeMap::new();
        scalars.insert("some-flag".to_owned(), "false".to_owned());
        let base = engine::cascade::PackDefaultLayer::new("dev", "0.0.0", scalars, Vec::new());

        let cfg = TempDir::new("manifest-bool");
        fs::write(
            cfg.path().join("manifest.yaml"),
            "scalar:\n  some-flag: true\n",
        )
        .expect("write manifest");

        let layer = load_project_layer(cfg.path()).expect("manifest loads");
        let resolved = engine::cascade::resolve(&base, None, Some(&layer)).expect("resolves");

        assert_eq!(
            resolved.scalar("some-flag"),
            Some("true"),
            "a bool scalar must record as its YAML string form",
        );
    }
}
