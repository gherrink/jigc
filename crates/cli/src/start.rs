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
use engine::cascade::{
    self, AnchorSpec, OverrideLayer, PackDefaultLayer, SlotFillDelta, SlotFillTarget,
    StructuralDelta, StructuralTarget, TrackedForkDelta,
};
use engine::compose::{
    self, CommandCatalog, ComposedWorkflow, ResolvedFills, StepDef, StepSource, StoreContext,
    WorkflowDef, apply_slot_fills, apply_structural_deltas, load_command_catalog, load_step_def,
    load_workflow_def,
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
/// flips it — [`resolve_cascade`] + [`cascade::Resolved::scalar_required`]), mints
/// the task (seq 12),
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
    // Resolve the cascade once: phase-3 scalars (the `default-workflow` read) +
    // phase-2 file owners + the manifest's phase-4 structural deltas (`overrides.md`
    // → Resolution algorithm). The same `Resolved` threads into the layer-aware
    // step source so a project step shadows the pack's.
    let (resolved, overrides) = resolve_cascade(&pack, &project_config)?;
    let workflow_id = resolved
        .scalar_required(DEFAULT_WORKFLOW_KEY)
        .map(str::to_owned)
        .map_err(anyhow::Error::from)?;
    let source = CascadeStepSource::new(&pack, &resolved, &project_config);
    compose_drained(&repo_root, intent, &pack, &workflow_id, &source, &overrides)
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
    // Form D bypasses the cascade `default-workflow` knob but still resolves the
    // cascade for phase-2 file owners + the phase-4/5 deltas, so a project step
    // override or slot-fill applies to a `--workflow <X>`-composed workflow too.
    let (resolved, overrides) = resolve_cascade(&pack, &project_config)?;
    let source = CascadeStepSource::new(&pack, &resolved, &project_config);
    compose_drained(&repo_root, intent, &pack, workflow_id, &source, &overrides)
}

/// Run [`compose_core`] over the layer-aware [`CascadeStepSource`], then prefer the
/// source's drained **located** fault over a generic compose error: a project step
/// the cascade owns but whose file is missing/malformed records a located finding
/// in the source's sink (the `Option<StepDef>` contract can't carry it), so a
/// failed compose surfaces that precise fault rather than the engine's generic
/// dangling-include message (`CascadeStepSource` doc → Located-error sink).
fn compose_drained(
    repo_root: &Path,
    intent: &str,
    pack: &dyn PackSource,
    workflow_id: &str,
    source: &CascadeStepSource<'_>,
    overrides: &ComposeOverrides,
) -> Result<ComposedWorkflow> {
    let result = compose_core(repo_root, intent, pack, workflow_id, source, overrides);
    if result.is_err()
        && let Some(located) = source.take_error()
    {
        return Err(finding_to_err(located));
    }
    result
}

/// Compose the workflow named by `workflow_id` from `intent`, branching on the
/// workflow's `creates-task` flag — the engine spine both front-door forms drive
/// (the cascade-default `jigc start "<intent>"` passes the cascade-resolved
/// `default-workflow`; the explicit `--workflow <X>` form passes `X`),
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
///
/// The `source` is the layer-aware [`StepSource`] (phase-2 by-id shadowing live);
/// `deltas` are the manifest's phase-4 `structural-op` deltas, applied to the
/// workflow's include id list **before** include expansion (`overrides.md` →
/// Resolution algorithm phase 4 / Why structural deltas precede expansion). The
/// no-delta / no-shadow path resolves the pack include list unchanged and reads
/// every step's pack body, so the composed bytes stay byte-identical to before
/// the wiring landed (the read-side determinism guard).
fn compose_core(
    repo_root: &Path,
    intent: &str,
    pack: &dyn PackSource,
    workflow_id: &str,
    source: &dyn StepSource,
    overrides: &ComposeOverrides,
) -> Result<ComposedWorkflow> {
    let workflow_bytes = read_workflow(pack, workflow_id)?;
    let mut def = load_workflow_def(&workflow_bytes).map_err(finding_to_err)?;
    // The manifest's `structural-op` deltas scoped to *this* workflow id — a
    // manifest may carry deltas for several workflows; only these apply here.
    let scoped = scoped_deltas(workflow_id, &overrides.deltas);
    // Phase 4 — apply the scoped deltas to the include id list, before include
    // expansion. A `replace-step` swaps a pack step id for a project-shadowed one,
    // so the layer-aware source resolves the new id to the project body. An
    // orphaned anchor (a delta whose anchor a same-manifest delta removed) surfaces
    // here as a blocking, routed finding (`overrides.md` → Within-layer manifest
    // order); the gate below re-runs the same pass to validate the post-phase-4
    // list for delta-introduced cycles/dangles.
    def.includes = apply_structural_deltas(&def.includes, &scoped).map_err(finding_to_err)?;
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
    // Compose-time `workflow-refs` gate over the **post-phase-4** include list +
    // the **post-phase-5** step bodies: validate every placeholder / include /
    // command-ref / marker, plus the two M4 fill checks (slot-fill-orphan +
    // fill-survivor), before any output reaches the agent — the scoped structural
    // deltas applied, the slot-fills + resolved fills fed in — so a delta-introduced
    // cycle / dangling include, an orphaned slot-fill, or a surviving nested
    // `{{fill:}}` all surface here at resolution time through the same machinery
    // (`overrides.md` → Resolution algorithm phases 4–7; The `{{fill:}}` placeholder).
    // The slot-fills thread unscoped (they carry no workflow id; the orphan check is
    // keyed on whether *this* workflow's composed bodies declare each point). A
    // blocking finding short-circuits.
    let findings = compose::workflow_refs_with_fills(
        &workflow_bytes,
        &scoped,
        &overrides.slot_fills,
        &overrides.fills,
        source,
        &commands,
        &ctx,
    );
    if let Some(finding) = findings
        .into_iter()
        .find(|f| f.severity == Severity::Blocking)
    {
        return Err(finding_to_err(finding));
    }

    // Phase 5 — apply the cascade's slot-fill content to each step body *before*
    // include expansion (phase 7): the [`FillStepSource`] runs `apply_slot_fills` as
    // each `StepDef` is fetched by id, so the filled content's own `{{include:}}` /
    // `{{cli.…}}` / `{{@…}}` resolve in the later phases as if the pack had written
    // them inline. A no-fill cascade (empty `fills`) is the identity — every point
    // resolves to its pack default, so the no-override path stays byte-identical.
    let filled = FillStepSource {
        inner: source,
        fills: &overrides.fills,
    };
    compose::compose(&def, &filled, &commands, &ctx).map_err(finding_to_err)
}

/// The manifest's `structural-op` deltas scoped to `workflow_id` — a
/// `structural-op` names its workflow in the target (`workflow:<id>#…`), so a
/// manifest may carry deltas for several workflows; only those targeting the
/// composed workflow apply at its phase 4 (`overrides.md` → Resolution algorithm
/// phase 4 / Delta targets). The scoped set threads into **both** the phase-4
/// `apply_structural_deltas` (the include list `compose` consumes) and the
/// post-phase-4 `workflow_refs_with_deltas` gate, so the gate validates the exact
/// list `compose` will expand. A no-delta cascade returns an empty set — the
/// byte-identity guard (`apply_structural_deltas` over `[]` is the identity).
pub(crate) fn scoped_deltas(workflow_id: &str, deltas: &[StructuralDelta]) -> Vec<StructuralDelta> {
    deltas
        .iter()
        .filter(|d| d.target().workflow_id == workflow_id)
        .cloned()
        .collect()
}

/// Build the `--explain` resolution tree (layers 1–2 — `workflow-dialect.md` →
/// `--explain` output contract) for `workflow_id` over the resolved cascade —
/// the seam the `--explain` dispatch (T3) renders.
///
/// Assembles the **exact** inputs [`compose_core`] feeds phase 4: the pack
/// workflow's include id list ([`read_workflow`] + [`load_workflow_def`]), the
/// manifest's `structural-op` `deltas` [`scoped_deltas`]-filtered to this
/// workflow, the layer the workflow definition file itself resolved to
/// ([`cascade::Resolved::file_owner`], defaulting to pack-default since the pack
/// always ships the workflow), and `file_owner` as the per-step layer lookup.
/// Delegates the position/annotation semantics to
/// [`compose::build_resolution_tree`], so the built tree's step order equals the
/// composed include order by construction — no re-resolution, no second algorithm.
///
/// Takes the manifest `deltas` directly (the same `ComposeOverrides.deltas` the
/// `--explain` dispatch already holds), keeping the seam free of the compose-only
/// override bundle.
///
/// Exercised by the unit tests; its live caller is the `--explain` flag dispatch
/// landed by T3 (`render::explain`), hence the `allow(dead_code)` until that wiring
/// arrives — the retained-seam pattern this crate already uses for
/// [`crate::render::orientation_agent_text`].
#[allow(dead_code)]
pub(crate) fn build_resolution_tree(
    pack: &dyn PackSource,
    resolved: &cascade::Resolved,
    deltas: &[StructuralDelta],
    workflow_id: &str,
) -> Result<engine::result::ResolutionTree> {
    let workflow_bytes = read_workflow(pack, workflow_id)?;
    let def = load_workflow_def(&workflow_bytes).map_err(finding_to_err)?;
    let scoped = scoped_deltas(workflow_id, deltas);
    let workflow_layer = resolved
        .file_owner(workflow_id)
        .unwrap_or(cascade::LayerKind::PackDefault);
    compose::build_resolution_tree(workflow_id, workflow_layer, &def.includes, &scoped, |id| {
        resolved.file_owner(id)
    })
    .map_err(finding_to_err)
}

/// The compose-relevant override surface a project layer carries beyond its
/// resolved scalar/file owners: the phase-4 `structural-op` deltas, the `slot-fill`
/// deltas (for the phase-5 fill pass + the orphan check), and the cascade-resolved
/// [`ResolvedFills`] map (`(step_id, fill_id)` → content body) those slot-fills load.
/// Bundled so both front doors thread one value through [`compose_drained`] rather
/// than a widening tuple (`overrides.md` → Resolution algorithm phases 4–5).
struct ComposeOverrides {
    /// The manifest's `structural-op` deltas (phase-4 include-list mutation).
    deltas: Vec<StructuralDelta>,
    /// The manifest's `slot-fill` deltas — the gate's orphan check walks them; they
    /// carry no workflow id, so they thread unscoped (the orphan check is keyed on
    /// whether the *composed* workflow's bodies declare each target's point).
    slot_fills: Vec<SlotFillDelta>,
    /// The cascade-resolved fill content the phase-5 pass splices, keyed
    /// `(step_id, fill_id)`. A no-fill cascade is the empty map (every point → its
    /// pack default), so the no-override path stays byte-identical.
    fills: ResolvedFills,
}

impl ComposeOverrides {
    /// A structural-only override surface (no slot-fills, empty resolved fills) —
    /// the phase-4-only shape the `compose_core` unit tests drive (the slot-fill
    /// apply path is exercised binary-driven in `tests/start_compose.rs`).
    #[cfg(test)]
    fn structural(deltas: Vec<StructuralDelta>) -> Self {
        ComposeOverrides {
            deltas,
            slot_fills: Vec::new(),
            fills: ResolvedFills::new(),
        }
    }
}

/// Resolve the cascade for a compose: seed the [`PackDefaultLayer`] scalar surface
/// from `config/knobs`, load the project [`OverrideLayer`] + its `structural-op` /
/// `slot-fill` deltas from `<project_config>/manifest.yaml`, and resolve — returning
/// the [`cascade::Resolved`] (phase-2 file owners + phase-3 scalars) paired with the
/// [`ComposeOverrides`] the compose applies at phases 4–5. The one cascade build both
/// front doors share (`overrides.md` → Resolution algorithm).
fn resolve_cascade(
    pack: &dyn PackSource,
    project_config: &Path,
) -> Result<(cascade::Resolved, ComposeOverrides)> {
    let knobs_bytes = read_pack(pack, PackResourceKind::Config, "knobs")?;
    let knobs = engine::knobs::load_knobs(&knobs_bytes).context("`config/knobs` is malformed")?;
    let pack_default = PackDefaultLayer::new(
        pack_id_from_config(pack)?,
        pack.pack_version(),
        knobs.base_scalars(),
        Vec::new(),
    );
    // The `tracked-fork` deltas are recording surface only — read by the (M5)
    // reconciliation, never by compose — so they are dropped here (a fork applies
    // as its phase-2 shadowed step file, no new resolution logic).
    let (project, deltas, slot_fills, _forks) = load_project_layer(project_config)?;
    let fills = load_fills(project_config, &slot_fills)?;
    let resolved = cascade::resolve(&pack_default, None, Some(&project))?;
    Ok((
        resolved,
        ComposeOverrides {
            deltas,
            slot_fills,
            fills,
        },
    ))
}

/// Load each `slot-fill` delta's native content file into the cascade-resolved
/// [`ResolvedFills`] map: for every delta, read `<project_config>/<content>` (its
/// `content_id` is the `fills/<id>.md` basename) and key the body by the target's
/// `(step_id, fill_id)` — the phase-5 input the pass splices (`overrides.md` →
/// Resolution algorithm phase 5; `storage.md` → Config layout, `fills/<id>.md`).
///
/// The project is the only override layer in M4, so a later layer's body never wins
/// here; when the team layer lands (M5), the same key would be overwritten in
/// precedence order (project last). A missing or unreadable fill file is a clear,
/// path-bearing error — the manifest references a native file that must exist.
fn load_fills(project_config: &Path, slot_fills: &[SlotFillDelta]) -> Result<ResolvedFills> {
    let mut fills = ResolvedFills::new();
    for delta in slot_fills {
        let path = project_config
            .join("fills")
            .join(format!("{}.md", delta.content_id));
        let body = std::fs::read_to_string(&path).with_context(|| {
            format!(
                "slot-fill on `step:{}#{}` references {}, which is unreadable",
                delta.target.step_id,
                delta.target.fill_id,
                path.display(),
            )
        })?;
        fills.insert(
            (delta.target.step_id.clone(), delta.target.fill_id.clone()),
            body,
        );
    }
    Ok(fills)
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

    // Resume composes the task's pinned workflow, but still over the *live* cascade —
    // a project `slot-fill` (or the pack's empty default for an unfilled
    // `{{fill:<id>}}` point) applies at phase 5 on re-compose exactly as on the fresh
    // front door, else a `{{fill:}}` point would survive resume to phase 8 unresolved.
    // The fill-aware gate runs the same M4 orphan + survivor checks (`overrides.md` →
    // The `{{fill:}}` placeholder); a no-fill cascade is the identity.
    let (_resolved, overrides) = resolve_cascade(&pack, &project_config)?;
    let findings = compose::workflow_refs_with_fills(
        &workflow_bytes,
        &[],
        &overrides.slot_fills,
        &overrides.fills,
        &source,
        &commands,
        &ctx,
    );
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

    // Phase 5 — apply the cascade's slot-fills to each fetched step body before
    // include expansion, identical to the fresh-compose path ([`compose_core`]): an
    // unfilled `{{fill:<id>}}` collapses to its empty pack default, a filled one
    // splices the project content. An empty `fills` is the identity, so the no-fill
    // resume stays byte-identical to the pre-`{{fill:}}`-point baseline.
    let filled = FillStepSource {
        inner: &source,
        fills: &overrides.fills,
    };
    compose::compose_with_store(&def, &filled, &commands, &ctx, Some(&store))
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

/// A phase-5 [`StepSource`] decorator: it fetches each step from `inner` (the
/// layer-aware cascade source) and applies the cascade's slot-fill content to the
/// body **before** returning it ([`compose::apply_slot_fills`]) — so include
/// expansion (phase 7) sees the filled body, and the filled content's own
/// `{{include:}}` / `{{cli.…}}` / `{{@…}}` resolve in the later phases as if the
/// pack had written them inline (`overrides.md` → Resolution algorithm phase 5; The
/// `{{fill:}}` placeholder).
///
/// `apply_slot_fills` is keyed `(step_id, fill_id)`, so applying it per fetched step
/// is exactly the right granularity. A no-fill cascade (`fills` empty) is the
/// identity: every `{{fill:}}` point resolves to its pack default (empty in M4), so
/// the body passes through byte-for-byte and the no-override path stays byte-
/// identical. Phase 5 does **not** re-run; a `{{fill:}}` nested in applied content
/// survives, already blocked by the `fill-survivor` gate check above.
struct FillStepSource<'a> {
    inner: &'a dyn StepSource,
    fills: &'a ResolvedFills,
}

impl StepSource for FillStepSource<'_> {
    fn step(&self, id: &str) -> Option<StepDef> {
        let def = self.inner.step(id)?;
        // The pass has no failure of its own in M4 (orphan/survivor is the gate's
        // job, already run); on the unreachable `Err` the unfilled body is kept.
        let body = apply_slot_fills(id, &def.body, self.fills).unwrap_or(def.body);
        Some(StepDef { id: def.id, body })
    }
}

/// A layer-aware [`StepSource`]: it consults the resolved cascade's phase-2 by-id
/// shadowing surface ([`cascade::Resolved::file_owner`]) to read each step from the
/// **highest-precedence layer that owns the id** — a project
/// `.jigc/config/steps/<id>.yaml` shadows the pack-default step, else the pack body
/// is read (`overrides.md` → Resolution algorithm phase 2). This replaces the
/// single-layer [`PackStepSource`] on the live compose read path: phase-2 by-id
/// shadowing wired in at last.
///
/// **Located-error sink.** The [`StepSource`] contract is `Option<StepDef>`, so a
/// fault cannot ride the return value. A `file_owner` of `Project` whose on-disk
/// file is **missing or malformed** is not a silent dangling-include nor a
/// fall-through to the pack body: `step()` records a clear located [`Finding`] into
/// an interior sink and returns `None`. The compose caller drains it via
/// [`CascadeStepSource::take_error`] after a `None` to surface the located fault.
/// This is the live compose step source (T4): the two front doors build it over
/// the resolved cascade + the project `steps/` dir and drain its sink after a
/// failed compose, so a missing/malformed project step surfaces its located fault
/// rather than the generic dangling-include message.
struct CascadeStepSource<'a> {
    pack: &'a dyn PackSource,
    resolved: &'a cascade::Resolved,
    /// The project layer's committed config dir — where `steps/<id>.yaml` lives.
    project_config: &'a Path,
    /// The located fault recorded by the most recent `step()` that returned `None`
    /// for a reason other than "no layer owns the id".
    error: std::cell::RefCell<Option<Finding>>,
}

impl<'a> CascadeStepSource<'a> {
    /// Build the layer-aware source over the resolved cascade + the project config
    /// dir the `steps/<id>.yaml` files live under.
    fn new(
        pack: &'a dyn PackSource,
        resolved: &'a cascade::Resolved,
        project_config: &'a Path,
    ) -> Self {
        Self {
            pack,
            resolved,
            project_config,
            error: std::cell::RefCell::new(None),
        }
    }

    /// Take the located fault recorded by the last failing `step()`, clearing the
    /// sink. `None` when the last `step()` returned `Some` or returned `None`
    /// because no layer owns the id (a plain dangling include the engine reports).
    fn take_error(&self) -> Option<Finding> {
        self.error.borrow_mut().take()
    }

    /// Read + parse a project-owned step file from `<project_config>/steps/<id>.yaml`,
    /// recording a located fault on a missing or malformed file (so the owning layer
    /// is never silently abandoned for the pack body).
    fn project_step(&self, id: &str) -> Option<StepDef> {
        let path = self.project_config.join("steps").join(format!("{id}.yaml"));
        let bytes = match std::fs::read(&path) {
            Ok(bytes) => bytes,
            Err(e) => {
                self.record(Finding::blocking(
                    "overrides.project-step-missing",
                    format!(
                        "project layer owns step `{id}` but its file {} is unreadable: {e}",
                        path.display()
                    ),
                    engine::finding::Location::at(1, 1),
                ));
                return None;
            }
        };
        match load_step_def(id, &bytes) {
            Ok(def) => Some(def),
            Err(finding) => {
                self.record(finding);
                None
            }
        }
    }

    fn record(&self, finding: Finding) {
        *self.error.borrow_mut() = Some(finding);
    }
}

impl StepSource for CascadeStepSource<'_> {
    fn step(&self, id: &str) -> Option<StepDef> {
        match self.resolved.file_owner(id) {
            // The project owns the id → read the project file (a missing/malformed
            // file is a located fault, never a fall-through to the pack body).
            Some(cascade::LayerKind::Project) => self.project_step(id),
            // Pack-default owns it, or no layer does — read the pack body. A
            // pack-unknown id is a plain dangling include the engine reports.
            _ => {
                let bytes = self
                    .pack
                    .read(PackResourceKind::Steps, &ResourceId::from(id))
                    .ok()?;
                load_step_def(id, &bytes).ok()
            }
        }
    }
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

/// The project cascade layer plus the three delta surfaces a manifest's `deltas:`
/// list carries: the [`OverrideLayer`], the phase-4 `structural-op`
/// [`StructuralDelta`]s, the phase-5 `slot-fill` [`SlotFillDelta`]s, and the
/// `tracked-fork` [`TrackedForkDelta`]s (recording surface, read only by the (M5)
/// reconciliation — never fed to compose).
type ProjectLayer = (
    OverrideLayer,
    Vec<StructuralDelta>,
    Vec<SlotFillDelta>,
    Vec<TrackedForkDelta>,
);

/// Load the project cascade layer from `<project_config>/manifest.yaml` plus the
/// project's native `steps/` dir, returning the [`OverrideLayer`] paired with the
/// manifest's phase-4 `structural-op` deltas, phase-5 `slot-fill` deltas, and
/// `tracked-fork` recording deltas (see [`ProjectLayer`]).
///
/// The layer carries three surfaces:
/// - **`scalar:`** — each entry becomes one [`OverrideLayer::scalar_set`], recorded
///   **in manifest order** (`overrides.md` → phase 3 / Within-layer manifest order),
///   flipping a knob like `default-workflow` the cascade reads on the compose path.
/// - **shadowed files** — every `steps/<id>.yaml` basename is declared via
///   [`OverrideLayer::shadow_file`], so [`cascade::Resolved::file_owner`] returns
///   `Project` for an id the project ships (phase-2 by-id shadowing). A native step
///   file shadows the pack step regardless of any delta (the `steps/` dir *is* the
///   project's step layer).
/// - **`deltas:`** — one entry per delta, split by `kind:`: the `structural-op`
///   kinds (`insert-step` / `replace-step` / `remove-step`) parse into
///   [`StructuralDelta`]s for the phase-4 pass; a `slot-fill` kind parses into a
///   [`SlotFillDelta`] for the phase-5 fill pass + the orphan check; a
///   `tracked-fork` kind parses into a [`TrackedForkDelta`] — **recording surface
///   only**, not fed to compose (at compose time a fork is just the shadowed step
///   file phase 2 already applies; the recorded `base-version` + `base-hash` are
///   read solely by the (M5) reconciliation — `overrides.md` → `tracked-fork` hash
///   basis). All three ride the one `deltas:` list (`storage.md` → Config layout)
///   and are returned separately because they mutate different surfaces at
///   different phases.
///
/// A **missing** manifest file (the dir may still hold `steps/` shadows), or a
/// present manifest with a **missing or blank** `scalar:` / `deltas:` block, yields
/// the no-override path — present-but-empty, byte-identical to today. A non-string
/// scalar value (`true`, `3`) is recorded as its YAML scalar string (opaque here).
///
/// The closed-surface check (an undeclared scalar key is an error) is **not** done
/// here: it is the resolver's job, so a hand-edited manifest and a `config set`
/// write are adjudicated by the one path.
pub(crate) fn load_project_layer(project_config: &Path) -> Result<ProjectLayer> {
    let manifest = project_config.join("manifest.yaml");
    // Every native `steps/<id>.yaml` basename shadows the pack step by id (phase 2),
    // independent of the manifest — the `steps/` dir is the project's step layer.
    let shadowed = project_step_ids(project_config);
    // Stamp the committed-config path (provenance header) + the shadowed step ids.
    let with_files = |mut layer: OverrideLayer| {
        layer = layer.config_path(project_config.display().to_string());
        for id in &shadowed {
            layer = layer.shadow_file(id);
        }
        layer
    };

    let text = match std::fs::read_to_string(&manifest) {
        Ok(text) => text,
        // No manifest file → no scalar/delta override (the `steps/` shadows still apply).
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
            return Ok((
                with_files(OverrideLayer::empty()),
                Vec::new(),
                Vec::new(),
                Vec::new(),
            ));
        }
        Err(e) => {
            return Err(e).with_context(|| format!("could not read {}", manifest.display()));
        }
    };
    let doc: serde_yaml_ng::Value = serde_yaml_ng::from_str(&text)
        .with_context(|| format!("{} is not valid YAML", manifest.display()))?;

    let mut layer = OverrideLayer::empty();
    // The `scalar:` block. A **blank** block (parsed as null) is the no-override
    // path — not an error — so present-but-empty leaves the layer empty.
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

    // The `deltas:` block — one list carrying the phase-4 `structural-op`, the
    // phase-5 `slot-fill`, and the (M5-read) `tracked-fork` recording kinds
    // (blank/absent → none of any).
    let (deltas, slot_fills, forks) = match doc.get("deltas") {
        None | Some(serde_yaml_ng::Value::Null) => (Vec::new(), Vec::new(), Vec::new()),
        Some(seq) => parse_deltas(seq, &manifest)?,
    };

    Ok((with_files(layer), deltas, slot_fills, forks))
}

/// Resolve one step's body **through the cascade** — the phase-2 file-owner read a
/// `CascadeStepSource` does, as a free function for the write-time `config fill`
/// point-exists check (`overrides.md` → the `jigc config` verbs: "the
/// `{{fill:<fill-id>}}` point exists in the resolved step body"). A project
/// `steps/<id>.yaml` shadows the pack step (phase-2 by-id shadowing); else the pack
/// body is read. A step id no layer owns yields `Ok(None)` (the caller routes it as
/// an absent point); a project-owned file that is missing/malformed is a clear,
/// path-bearing error (never a silent fall-through to the pack body).
///
/// This builds only the phase-2 [`cascade::Resolved`] (knobs + the project layer's
/// `steps/` shadows), not the full [`ComposeOverrides`] — the check needs the file
/// owner, not the resolved fills.
pub(crate) fn resolve_step_body(
    pack: &dyn PackSource,
    project_config: &Path,
    step_id: &str,
) -> Result<Option<engine::compose::StepDef>> {
    let knobs_bytes = read_pack(pack, PackResourceKind::Config, "knobs")?;
    let knobs = engine::knobs::load_knobs(&knobs_bytes).context("`config/knobs` is malformed")?;
    let pack_default = PackDefaultLayer::new(
        pack_id_from_config(pack)?,
        pack.pack_version(),
        knobs.base_scalars(),
        Vec::new(),
    );
    let (project, _deltas, _slot_fills, _forks) = load_project_layer(project_config)?;
    let resolved = cascade::resolve(&pack_default, None, Some(&project))?;

    match resolved.file_owner(step_id) {
        // The project owns the id → read the project file (missing/malformed is a
        // path-bearing error, never a fall-through to the pack body).
        Some(cascade::LayerKind::Project) => {
            let path = project_config.join("steps").join(format!("{step_id}.yaml"));
            let bytes = std::fs::read(&path).with_context(|| {
                format!(
                    "project layer owns step `{step_id}` but its file {} is unreadable",
                    path.display()
                )
            })?;
            Ok(Some(
                load_step_def(step_id, &bytes).map_err(finding_to_err)?,
            ))
        }
        // Pack-default owns it, or no layer does — read the pack body. A pack-unknown
        // id has no body to resolve (the caller routes the absent point).
        _ => match pack.read(PackResourceKind::Steps, &ResourceId::from(step_id)) {
            Ok(bytes) => Ok(Some(
                load_step_def(step_id, &bytes).map_err(finding_to_err)?,
            )),
            Err(_) => Ok(None),
        },
    }
}

/// List the native step ids the project layer ships — every `<project_config>/
/// steps/<id>.yaml` basename. These shadow the pack step of the same id at phase 2
/// (`overrides.md` → Native-file id = filename basename). A missing `steps/` dir is
/// the empty list (no project shadows), never an error.
pub(crate) fn project_step_ids(project_config: &Path) -> Vec<String> {
    let dir = project_config.join("steps");
    let Ok(entries) = std::fs::read_dir(&dir) else {
        return Vec::new();
    };
    let mut ids: Vec<String> = entries
        .flatten()
        .map(|e| e.path())
        .filter(|p| p.extension().and_then(|x| x.to_str()) == Some("yaml"))
        .filter_map(|p| p.file_stem().and_then(|s| s.to_str()).map(str::to_owned))
        .collect();
    ids.sort();
    ids
}

/// Parse the manifest's `deltas:` sequence, **partitioned by `kind:`** into the
/// phase-4 `structural-op` [`StructuralDelta`]s and the phase-5 `slot-fill`
/// [`SlotFillDelta`]s (`overrides.md` → Delta representation / Delta targets;
/// `storage.md` → Config layout: one `deltas:` list, every kind). A `structural-op`
/// entry carries a `kind` (`insert-step` / `replace-step` / `remove-step`), a
/// `target`, and — for insert/replace — a `with:` step reference; a `slot-fill`
/// entry carries `kind: slot-fill`, a `target: step:<id>#<fill-id>`, and a
/// `content: fills/<id>.md`; a `tracked-fork` entry carries `kind: tracked-fork`, a
/// `target: workflow:<id>#<step-id>`, a `base-version:`, and a `base-hash:`. A
/// malformed entry surfaces as its located finding.
fn parse_deltas(
    seq: &serde_yaml_ng::Value,
    manifest: &Path,
) -> Result<(
    Vec<StructuralDelta>,
    Vec<SlotFillDelta>,
    Vec<TrackedForkDelta>,
)> {
    let items = seq
        .as_sequence()
        .with_context(|| format!("{}: `deltas:` must be a list of deltas", manifest.display()))?;
    let mut structural = Vec::new();
    let mut slot_fills = Vec::new();
    let mut forks = Vec::new();
    for item in items {
        match parse_one_delta(item, manifest)? {
            ParsedDelta::Structural(delta) => structural.push(delta),
            ParsedDelta::SlotFill(delta) => slot_fills.push(delta),
            ParsedDelta::TrackedFork(delta) => forks.push(delta),
        }
    }
    Ok((structural, slot_fills, forks))
}

/// One parsed `deltas:` entry, partitioned by kind — a phase-4 structural-op, a
/// phase-5 slot-fill, or a `tracked-fork` recording (`overrides.md` → The ladder:
/// distinct kinds, distinct phases). The fork is recording-only — not fed to
/// compose; its basis is read solely by the (M5) reconciliation.
enum ParsedDelta {
    Structural(StructuralDelta),
    SlotFill(SlotFillDelta),
    TrackedFork(TrackedForkDelta),
}

/// Parse one `deltas:` entry into a [`ParsedDelta`], branching on `kind:`. A
/// `with:`/`target:` value may carry a `step:` prefix (the manifest spelling —
/// flow 3a's `with: step:project-implement`); the bare step id is what the include
/// list holds. A `target`'s `after:`/`before:` anchor is read from sibling keys. A
/// `slot-fill` entry's `content: fills/<id>.md` yields the native file's basename id.
fn parse_one_delta(item: &serde_yaml_ng::Value, manifest: &Path) -> Result<ParsedDelta> {
    let at = |key: &str| item.get(key).and_then(serde_yaml_ng::Value::as_str);
    let kind = at("kind")
        .with_context(|| format!("{}: a `deltas:` entry needs a `kind`", manifest.display()))?;
    let target_str = at("target")
        .with_context(|| format!("{}: a `deltas:` entry needs a `target`", manifest.display()))?;

    match kind {
        "replace-step" => {
            let target = StructuralTarget::parse(target_str, None).map_err(finding_to_err)?;
            let step = with_step(at("with"), kind, manifest)?;
            Ok(ParsedDelta::Structural(StructuralDelta::Replace {
                target,
                step,
            }))
        }
        "remove-step" => {
            let target = StructuralTarget::parse(target_str, None).map_err(finding_to_err)?;
            Ok(ParsedDelta::Structural(StructuralDelta::Remove { target }))
        }
        "insert-step" => {
            let anchor = match (at("after"), at("before")) {
                (Some(a), None) => AnchorSpec::After(a.to_owned()),
                (None, Some(b)) => AnchorSpec::Before(b.to_owned()),
                (Some(_), Some(_)) => bail!(
                    "{}: an `insert-step` needs exactly one of `after:`/`before:`",
                    manifest.display()
                ),
                (None, None) => bail!(
                    "{}: an `insert-step` needs an `after:` or `before:` anchor",
                    manifest.display()
                ),
            };
            let target =
                StructuralTarget::parse(target_str, Some(anchor)).map_err(finding_to_err)?;
            let step = with_step(at("with"), kind, manifest)?;
            Ok(ParsedDelta::Structural(StructuralDelta::Insert {
                target,
                step,
            }))
        }
        "slot-fill" => {
            let target = SlotFillTarget::parse(target_str).map_err(finding_to_err)?;
            let content_id = content_id(at("content"), manifest)?;
            Ok(ParsedDelta::SlotFill(SlotFillDelta { target, content_id }))
        }
        "tracked-fork" => {
            // `workflow:<id>#<step-id>` → an `Anchor::At` target naming the forked
            // unit (the native step file shadowed whole at phase 2). The recorded
            // basis (`base-version` + the blake3 `base-hash`) is M4's pinned
            // ancestor — read only by the (M5) compare; both are required, so a
            // missing field is a clear, located error here, never a panic.
            let target = StructuralTarget::parse(target_str, None).map_err(finding_to_err)?;
            let base_version = required_field(at("base-version"), "base-version", kind, manifest)?;
            let base_hash = required_field(at("base-hash"), "base-hash", kind, manifest)?;
            Ok(ParsedDelta::TrackedFork(TrackedForkDelta {
                target,
                base_version,
                base_hash,
            }))
        }
        other => bail!(
            "{}: unknown delta kind `{other}` (expected insert-step / replace-step / remove-step / slot-fill / tracked-fork)",
            manifest.display()
        ),
    }
}

/// The native fill file's basename id from a `slot-fill`'s `content:` value
/// (`content: fills/extra-guidance.md` → `extra-guidance`), the same
/// "id = filename basename" rule the `steps/` dir uses (`storage.md` → Config
/// layout, `fills/<id>.md`). A missing `content:` is a clear error.
fn content_id(raw: Option<&str>, manifest: &Path) -> Result<String> {
    let raw = raw.with_context(|| {
        format!(
            "{}: a `slot-fill` needs a `content:` file reference",
            manifest.display()
        )
    })?;
    let basename = Path::new(raw)
        .file_stem()
        .and_then(|s| s.to_str())
        .filter(|s| !s.is_empty())
        .with_context(|| {
            format!(
                "{}: a `slot-fill` `content:` must name a `fills/<id>.md` file; got `{raw}`",
                manifest.display()
            )
        })?;
    Ok(basename.to_owned())
}

/// A required string `field` off a `deltas:` entry (`tracked-fork`'s `base-version`
/// / `base-hash`) — a missing field is a clear, located error naming the field and
/// the entry's `kind`, never a panic (`overrides.md` → `tracked-fork` hash basis:
/// the recorded basis must be present to round-trip).
fn required_field(raw: Option<&str>, field: &str, kind: &str, manifest: &Path) -> Result<String> {
    let raw =
        raw.with_context(|| format!("{}: a `{kind}` needs a `{field}`", manifest.display()))?;
    Ok(raw.to_owned())
}

/// The bare step id from a `with:` reference, stripping the manifest's optional
/// `step:` prefix (`with: step:project-implement` → `project-implement`). A
/// missing `with:` on an insert/replace is a clear error.
fn with_step(raw: Option<&str>, kind: &str, manifest: &Path) -> Result<String> {
    let raw = raw.with_context(|| {
        format!(
            "{}: a `{kind}` needs a `with:` step reference",
            manifest.display()
        )
    })?;
    Ok(raw.strip_prefix("step:").unwrap_or(raw).to_owned())
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
pub(crate) fn read_workflow(pack: &dyn PackSource, id: &str) -> Result<Vec<u8>> {
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
    use engine::cascade::Anchor;
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
        // cascade read (`resolve_cascade` + `scalar_required`) is exercised by the binary-
        // driven flip + byte-identical goldens (`tests/start_compose.rs`). The
        // no-override step source + empty phase-4 deltas read the fixture pack
        // unchanged (the live `CascadeStepSource` path is the binary-driven tests').
        let source = PackStepSource { pack: &pack };
        let composed = compose_core(
            repo.path(),
            "anything",
            &pack,
            "router",
            &source,
            &ComposeOverrides::structural(Vec::new()),
        )
        .expect("no-task compose");

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

    /// Build an `insert` structural delta from a `workflow:<id>` + `after:`/`before:`
    /// anchor + the inserted step id (the CLI-side mirror of the engine test helper).
    fn insert_delta(workflow: &str, anchor: AnchorSpec, step: &str) -> StructuralDelta {
        StructuralDelta::Insert {
            target: StructuralTarget::parse(workflow, Some(anchor)).expect("valid target"),
            step: step.to_owned(),
        }
    }

    /// Build a `replace` structural delta from a `workflow:<id>#<step-id>` ref + the
    /// replacement step id.
    fn replace_delta(target: &str, step: &str) -> StructuralDelta {
        StructuralDelta::Replace {
            target: StructuralTarget::parse(target, None).expect("valid target"),
            step: step.to_owned(),
        }
    }

    /// Build a `remove` structural delta from a `workflow:<id>#<step-id>` ref.
    fn remove_delta(target: &str) -> StructuralDelta {
        StructuralDelta::Remove {
            target: StructuralTarget::parse(target, None).expect("valid target"),
        }
    }

    /// A `creates-task: false` `router`-style fixture pack over a two-step
    /// `flow` workflow (`[locate, implement]`) — enough surface to apply a
    /// structural delta to its include list and compose the result without a mint.
    fn structural_pack() -> FixturePack {
        FixturePack::with(vec![
            (
                PackResourceKind::Config,
                "defaults",
                "pack-id: dev\ndefault-workflow: flow\n",
            ),
            (PackResourceKind::Config, "commands", "commands: []\n"),
            (
                PackResourceKind::Workflows,
                "flow",
                "---\nwhen: x\ncreates-task: false\n---\n{{ include: step:locate }}\n{{ include: step:implement }}\n",
            ),
            (PackResourceKind::Steps, "locate", "locate body\n"),
            (PackResourceKind::Steps, "implement", "implement body\n"),
        ])
    }

    /// T5 wire-through (a): `compose_core` runs its `workflow-refs` gate over the
    /// **post-phase-4** include list, so an orphaned-anchor delta — `remove #locate`
    /// then `insert --after locate` in the same manifest — fails the compose with the
    /// routed `structural-anchor-resolves` block (`overrides.md` → Within-layer
    /// manifest order). The orphan surfaces on the real CLI path, never a panic.
    #[test]
    fn compose_core_orphaned_anchor_delta_fails_with_route() {
        let repo = TempDir::new("t5-orphan");
        let pack = structural_pack();
        let source = PackStepSource { pack: &pack };
        let deltas = vec![
            remove_delta("workflow:flow#locate"),
            insert_delta(
                "workflow:flow",
                AnchorSpec::After("locate".to_owned()),
                "lint",
            ),
        ];

        let err = compose_core(
            repo.path(),
            "anything",
            &pack,
            "flow",
            &source,
            &ComposeOverrides::structural(deltas),
        )
        .expect_err("the insert's anchor was removed by the earlier delta");

        let msg = err.to_string();
        assert!(
            msg.contains("orphaned") && msg.contains("locate"),
            "the orphaned-anchor failure must name the cause + the anchor; got: {msg}",
        );
        assert!(
            msg.contains("route:") && msg.contains("jigc config"),
            "the orphan surfaces its repair route on the CLI path; got: {msg}",
        );
    }

    /// T5 wire-through (b): a `replace-step` whose replacement step re-includes a
    /// step that loops back introduces an include cycle the gate catches at phase 6
    /// over the post-phase-4 list — the compose fails with `include-cycle-absent`,
    /// not a panic (`overrides.md` → No silent cycle handling).
    #[test]
    fn compose_core_delta_introduced_cycle_fails_at_phase_6() {
        let repo = TempDir::new("t5-cycle");
        let pack = FixturePack::with(vec![
            (
                PackResourceKind::Config,
                "defaults",
                "pack-id: dev\ndefault-workflow: flow\n",
            ),
            (PackResourceKind::Config, "commands", "commands: []\n"),
            (
                PackResourceKind::Workflows,
                "flow",
                "---\nwhen: x\ncreates-task: false\n---\n{{ include: step:locate }}\n{{ include: step:implement }}\n",
            ),
            (PackResourceKind::Steps, "locate", "locate body\n"),
            (PackResourceKind::Steps, "implement", "implement body\n"),
            // The replacement loops: looping -> back -> looping.
            (
                PackResourceKind::Steps,
                "looping",
                "house rule\n{{ include: step:back }}\n",
            ),
            (
                PackResourceKind::Steps,
                "back",
                "{{ include: step:looping }}\n",
            ),
        ]);
        let source = PackStepSource { pack: &pack };
        let deltas = vec![replace_delta("workflow:flow#implement", "looping")];

        let err = compose_core(
            repo.path(),
            "anything",
            &pack,
            "flow",
            &source,
            &ComposeOverrides::structural(deltas),
        )
        .expect_err("the delta introduces an include cycle");

        assert!(
            err.to_string().contains("include cycle"),
            "a delta-introduced cycle must fail the gate at phase 6; got: {err}",
        );
    }

    /// T5 wire-through (c): flow 3a's clean re-include — `replace #implement →
    /// project-implement` where `project-implement` re-includes the pack `implement`
    /// (a **different** id) — composes cleanly with no false cycle, and the composed
    /// view carries both the pack `implement` body and the house rule
    /// (`worked-examples.md` → 3a). The deltas flow through the real CLI compose path.
    #[test]
    fn compose_core_flow_3a_re_include_composes_clean() {
        let repo = TempDir::new("t5-3a");
        let pack = FixturePack::with(vec![
            (
                PackResourceKind::Config,
                "defaults",
                "pack-id: dev\ndefault-workflow: flow\n",
            ),
            (PackResourceKind::Config, "commands", "commands: []\n"),
            (
                PackResourceKind::Workflows,
                "flow",
                "---\nwhen: x\ncreates-task: false\n---\n{{ include: step:locate }}\n{{ include: step:implement }}\n",
            ),
            (PackResourceKind::Steps, "locate", "locate body\n"),
            (
                PackResourceKind::Steps,
                "implement",
                "pack implement body\n",
            ),
            // The project step re-includes the pack `implement` (a different id).
            (
                PackResourceKind::Steps,
                "project-implement",
                "{{ include: step:implement }}\nhouse rule: run the lint probe\n",
            ),
        ]);
        let source = PackStepSource { pack: &pack };
        let deltas = vec![replace_delta(
            "workflow:flow#implement",
            "project-implement",
        )];

        let composed = compose_core(
            repo.path(),
            "anything",
            &pack,
            "flow",
            &source,
            &ComposeOverrides::structural(deltas),
        )
        .expect("flow 3a's different-id re-include composes clean");

        assert!(
            composed.text.contains("pack implement body")
                && composed.text.contains("house rule: run the lint probe"),
            "the re-include must pull the pack body in then the house rule; got:\n{}",
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
        let source = PackStepSource { pack: &pack };
        let composed = compose_core(
            repo.path(),
            "Add rate limiter",
            &pack,
            "single-task",
            &source,
            &ComposeOverrides::structural(Vec::new()),
        )
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
        let source = PackStepSource { pack: &pack };
        let err = compose_core(
            repo.path(),
            "Add rate limiter",
            &pack,
            "does-not-exist",
            &source,
            &ComposeOverrides::structural(Vec::new()),
        )
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

        let (layer, _deltas, _slot_fills, _forks) =
            load_project_layer(cfg.path()).expect("manifest loads");
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

        let (layer, _deltas, _slot_fills, _forks) =
            load_project_layer(cfg.path()).expect("manifest loads");
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

        let (layer, _deltas, _slot_fills, _forks) =
            load_project_layer(cfg.path()).expect("manifest loads");
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

        let (layer, _deltas, _slot_fills, _forks) =
            load_project_layer(cfg.path()).expect("missing manifest is not an error");
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

        let (layer, _deltas, _slot_fills, _forks) =
            load_project_layer(cfg.path()).expect("manifest loads");
        let resolved = engine::cascade::resolve(&base, None, Some(&layer)).expect("resolves");

        assert_eq!(resolved.scalar("default-workflow"), Some("router"));
        assert_eq!(
            resolved.scalar("validation.file-state.severity"),
            Some("warning"),
            "every `scalar:` entry must be recorded, not just the first",
        );
    }

    /// The read-side determinism invariant, end-to-end through the compose read:
    /// the bare front door resolves the cascade then reads `default-workflow` via
    /// `scalar_required`, so a pack whose `config/knobs` **does not declare**
    /// `default-workflow` fails **loudly** (the `UndeclaredComposeRead` hard error)
    /// rather than silently composing nothing. Pins that the wired compose read
    /// never falls back to a raw `None` for an undeclared key.
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

        // The live front-door read: resolve the cascade, then read the key through
        // `scalar_required` (the exact two steps `compose_in_repo` runs).
        let (resolved, _overrides) = resolve_cascade(&pack, cfg.path()).expect("cascade resolves");
        let err = resolved
            .scalar_required(DEFAULT_WORKFLOW_KEY)
            .map(str::to_owned)
            .map_err(anyhow::Error::from)
            .expect_err("an undeclared compose-read key must fail loudly");
        let msg = err.to_string();
        assert!(
            msg.contains("default-workflow") && msg.contains("undeclared"),
            "the read of an undeclared key must name it and the closed-surface rule; got: {msg}",
        );
    }

    /// The T3 done-criterion: a [`CascadeStepSource`] consults
    /// [`cascade::Resolved::file_owner`] so a project-owned step id reads the
    /// **project** body from `.jigc/config/steps/<id>.yaml`, while an id the
    /// project does not own reads the **pack** body — the highest-precedence
    /// layer that owns the id wins (`overrides.md` → Resolution algorithm phase 2).
    #[test]
    fn cascade_step_source_reads_project_body_when_project_owns_the_id() {
        let cfg = TempDir::new("cascade-step");
        let steps = cfg.path().join("steps");
        fs::create_dir_all(&steps).expect("mk steps/");
        // The project layer ships its own `implement` step file; `noop` it does not.
        fs::write(steps.join("implement.yaml"), "project implement body\n").expect("w");

        // The pack ships both `implement` and `noop`.
        let pack = FixturePack::with(vec![
            (
                PackResourceKind::Steps,
                "implement",
                "pack implement body\n",
            ),
            (PackResourceKind::Steps, "noop", "pack noop body\n"),
        ]);

        // The resolved cascade: pack ships both ids, project shadows `implement`.
        let base = engine::cascade::PackDefaultLayer::new(
            "dev",
            "0.0.0",
            BTreeMap::new(),
            vec!["implement".to_owned(), "noop".to_owned()],
        );
        let project = OverrideLayer::empty().shadow_file("implement");
        let resolved = engine::cascade::resolve(&base, None, Some(&project)).expect("resolves");

        let source = CascadeStepSource::new(&pack, &resolved, cfg.path());

        // Project owns `implement` → the project file's body is read.
        let implement = source.step("implement").expect("project step resolves");
        assert_eq!(
            implement.body, "project implement body\n",
            "a project-owned id must read the project layer's body",
        );
        assert!(
            source.take_error().is_none(),
            "a clean read records no located error",
        );

        // `noop` is pack-owned → the pack body is read.
        let noop = source.step("noop").expect("pack step resolves");
        assert_eq!(
            noop.body, "pack noop body\n",
            "an id the project does not own must read the pack body",
        );
    }

    /// The T3 located-error half: a `file_owner` of `Project` whose on-disk file
    /// is **missing** is a clear located error — `step()` returns `None` (no
    /// silent pack fall-through past the owning layer) and the source records a
    /// located [`Finding`] naming the absent file, drained via `take_error`.
    #[test]
    fn cascade_step_source_missing_project_file_is_a_located_error() {
        let cfg = TempDir::new("cascade-step-missing");
        fs::create_dir_all(cfg.path().join("steps")).expect("mk steps/");
        // No `implement.yaml` written — the project owns the id but ships no file.

        let pack = FixturePack::with(vec![(
            PackResourceKind::Steps,
            "implement",
            "pack implement body\n",
        )]);
        let base = engine::cascade::PackDefaultLayer::new(
            "dev",
            "0.0.0",
            BTreeMap::new(),
            vec!["implement".to_owned()],
        );
        let project = OverrideLayer::empty().shadow_file("implement");
        let resolved = engine::cascade::resolve(&base, None, Some(&project)).expect("resolves");

        let source = CascadeStepSource::new(&pack, &resolved, cfg.path());

        // The project owns the id but its file is missing → `None`, never a silent
        // fall-through to the pack body.
        assert!(
            source.step("implement").is_none(),
            "a project-owned id whose file is missing must not fall through to the pack",
        );
        let finding = source
            .take_error()
            .expect("a missing project-owned file records a located finding");
        assert!(
            finding.location.is_some(),
            "the missing-file fault must be located; got {finding:?}",
        );
        assert!(
            finding.message.contains("implement"),
            "the located error must name the absent step id; got: {}",
            finding.message,
        );
    }

    /// The CLI `--explain` tree seam over a real cascade: it reads the pack
    /// workflow's include list, tags each step against the resolved
    /// `file_owner` (phase-2 by-id shadowing), and threads any scoped
    /// `structural-op` delta into the engine builder. The project shadows the
    /// `implement` step file, so the no-delta tree tags `implement` **project**
    /// and `locate` **pack-default** with `overrides_applied: 0` — the
    /// file-owner provenance the CLI half of T1 wires (`overrides.md` →
    /// Resolution algorithm phase 2; `workflow-dialect.md` → `--explain`).
    #[test]
    fn cli_resolution_tree_tags_each_step_by_resolved_file_owner() {
        let pack = FixturePack::with(vec![
            (
                PackResourceKind::Workflows,
                "wf",
                "---\nwhen: x\ncreates-task: true\n---\n{{ include: step:locate }}\n{{ include: step:implement }}\n",
            ),
            (PackResourceKind::Steps, "locate", "pack locate body\n"),
            (
                PackResourceKind::Steps,
                "implement",
                "pack implement body\n",
            ),
        ]);
        // The cascade: pack owns both steps + the workflow; the project shadows the
        // `implement` step file (phase-2 by-id shadowing), so `file_owner` reports
        // it project-owned.
        let base = engine::cascade::PackDefaultLayer::new(
            "dev",
            "0.0.0",
            BTreeMap::new(),
            vec!["wf".to_owned(), "locate".to_owned(), "implement".to_owned()],
        );
        let project = OverrideLayer::empty().shadow_file("implement");
        let resolved = engine::cascade::resolve(&base, None, Some(&project)).expect("resolves");

        let tree = build_resolution_tree(&pack, &resolved, &[], "wf").expect("tree builds");

        assert_eq!(tree.workflow, "wf");
        assert_eq!(tree.overrides_applied, 0);
        let by_owner: Vec<(&str, engine::cascade::LayerKind)> = tree
            .steps
            .iter()
            .map(|s| (s.id.as_str(), s.layer))
            .collect();
        assert_eq!(
            by_owner,
            vec![
                ("locate", engine::cascade::LayerKind::PackDefault),
                ("implement", engine::cascade::LayerKind::Project),
            ],
            "each step is tagged by its resolved file owner, in include order",
        );
        assert!(
            tree.steps.iter().all(|s| s.replaces.is_none()),
            "a no-delta tree carries no replace annotation",
        );
    }

    /// The CLI seam threads a **scoped** `replace-step` delta through to the
    /// engine builder: a project `replace-step workflow:wf#implement →
    /// step:project-implement` yields `overrides_applied: 1`, the replacing step
    /// tagged project at the same position with the `replaces implement at
    /// position 2` annotation, while a same-manifest delta scoped to *another*
    /// workflow is filtered out (`scoped_deltas`).
    #[test]
    fn cli_resolution_tree_threads_scoped_replace_delta() {
        let pack = FixturePack::with(vec![
            (
                PackResourceKind::Workflows,
                "wf",
                "---\nwhen: x\ncreates-task: true\n---\n{{ include: step:locate }}\n{{ include: step:implement }}\n",
            ),
            (PackResourceKind::Steps, "locate", "pack locate body\n"),
            (
                PackResourceKind::Steps,
                "implement",
                "pack implement body\n",
            ),
        ]);
        let base = engine::cascade::PackDefaultLayer::new(
            "dev",
            "0.0.0",
            BTreeMap::new(),
            vec![
                "wf".to_owned(),
                "locate".to_owned(),
                "implement".to_owned(),
                "project-implement".to_owned(),
            ],
        );
        let project = OverrideLayer::empty().shadow_file("project-implement");
        let resolved = engine::cascade::resolve(&base, None, Some(&project)).expect("resolves");

        let deltas = vec![
            StructuralDelta::Replace {
                target: StructuralTarget::parse("workflow:wf#implement", None).expect("target"),
                step: "project-implement".to_owned(),
            },
            // A delta for a different workflow — `scoped_deltas` must drop it, so it
            // neither applies nor inflates `overrides_applied`.
            StructuralDelta::Remove {
                target: StructuralTarget::parse("workflow:other#locate", None).expect("target"),
            },
        ];

        let tree = build_resolution_tree(&pack, &resolved, &deltas, "wf").expect("tree builds");

        assert_eq!(
            tree.overrides_applied, 1,
            "only the `wf`-scoped delta counts; the `other`-scoped delta is filtered",
        );
        let replacing = &tree.steps[1];
        assert_eq!(replacing.id, "project-implement");
        assert_eq!(replacing.layer, engine::cascade::LayerKind::Project);
        assert_eq!(
            replacing.replaces,
            Some(engine::result::Replacement {
                replaced: "implement".to_owned(),
                position: 2,
            }),
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

        let (layer, _deltas, _slot_fills, _forks) =
            load_project_layer(cfg.path()).expect("manifest loads");
        let resolved = engine::cascade::resolve(&base, None, Some(&layer)).expect("resolves");

        assert_eq!(
            resolved.scalar("some-flag"),
            Some("true"),
            "a bool scalar must record as its YAML string form",
        );
    }

    /// A manifest carrying a hand-authored `tracked-fork` entry **alongside** a
    /// `scalar:` block and a `slot-fill` delta round-trips: the loader parses the
    /// fork back to the T1 [`TrackedForkDelta`] (target + `base-version` +
    /// `base-hash`) without erroring, and the other blocks survive its presence —
    /// the `scalar:` flips the resolved value and the `slot-fill` delta is still
    /// returned (`overrides.md` → Delta representation; `storage.md` → Config
    /// layout: one `deltas:` list, every kind).
    #[test]
    fn manifest_tracked_fork_entry_round_trips_alongside_scalar_and_slot_fill() {
        let cfg = TempDir::new("manifest-tracked-fork");
        fs::write(
            cfg.path().join("manifest.yaml"),
            "scalar:\n  \
             default-workflow: router\n\
             deltas:\n  \
             - kind: slot-fill\n    \
             target: step:implement#extra-guidance\n    \
             content: fills/extra-guidance.md\n  \
             - kind: tracked-fork\n    \
             target: workflow:single-task#implement\n    \
             base-version: v1\n    \
             base-hash: a3f9deadbeef\n",
        )
        .expect("write manifest");

        let (layer, deltas, slot_fills, forks) =
            load_project_layer(cfg.path()).expect("a tracked-fork entry must not error the load");

        // The fork round-trips to the T1 delta.
        assert_eq!(
            forks,
            vec![TrackedForkDelta {
                target: StructuralTarget {
                    workflow_id: "single-task".to_owned(),
                    anchor: Anchor::At("implement".to_owned()),
                },
                base_version: "v1".to_owned(),
                base_hash: "a3f9deadbeef".to_owned(),
            }],
            "the `tracked-fork` entry must parse back to the recorded T1 delta",
        );

        // The other blocks survive the fork entry's presence.
        let resolved =
            engine::cascade::resolve(&base_layer(), None, Some(&layer)).expect("resolves");
        assert_eq!(
            resolved.scalar("default-workflow"),
            Some("router"),
            "the `scalar:` block must still flip the resolved value",
        );
        assert_eq!(
            slot_fills,
            vec![SlotFillDelta {
                target: SlotFillTarget::parse("step:implement#extra-guidance")
                    .expect("valid slot-fill target"),
                content_id: "extra-guidance".to_owned(),
            }],
            "the `slot-fill` delta must survive the fork entry's presence",
        );
        assert!(
            deltas.is_empty(),
            "the fork is not a structural-op, so no structural delta is recorded",
        );
    }

    /// The determinism guard: the structural-op and slot-fill vectors are
    /// **byte-identical** whether or not a `tracked-fork` entry sits in the same
    /// `deltas:` list — the fork is recording surface, never fed to compose, so it
    /// must not perturb the phase-4 / phase-5 inputs.
    #[test]
    fn tracked_fork_entry_does_not_perturb_structural_or_slot_fill_vectors() {
        let without = "deltas:\n  \
             - kind: insert-step\n    \
             target: workflow:single-task\n    \
             after: implement\n    \
             with: step:project-implement\n  \
             - kind: slot-fill\n    \
             target: step:implement#extra-guidance\n    \
             content: fills/extra-guidance.md\n";
        let with = format!(
            "{without}  \
             - kind: tracked-fork\n    \
             target: workflow:single-task#implement\n    \
             base-version: v1\n    \
             base-hash: a3f9deadbeef\n"
        );

        let cfg_without = TempDir::new("fork-guard-without");
        fs::write(cfg_without.path().join("manifest.yaml"), without).expect("write manifest");
        let cfg_with = TempDir::new("fork-guard-with");
        fs::write(cfg_with.path().join("manifest.yaml"), with).expect("write manifest");

        let (_l0, deltas0, slot_fills0, forks0) =
            load_project_layer(cfg_without.path()).expect("loads");
        let (_l1, deltas1, slot_fills1, forks1) =
            load_project_layer(cfg_with.path()).expect("loads");

        assert_eq!(
            deltas0, deltas1,
            "the structural-op vector must be identical with or without the fork entry",
        );
        assert_eq!(
            slot_fills0, slot_fills1,
            "the slot-fill vector must be identical with or without the fork entry",
        );
        assert!(
            forks0.is_empty(),
            "the fork-free manifest records no fork delta",
        );
        assert_eq!(
            forks1.len(),
            1,
            "the fork-bearing manifest records exactly its one fork delta",
        );
    }

    /// A malformed `tracked-fork` entry — `base-hash` missing — is a **clear,
    /// located error**, not a panic: the recorded basis is the pinned ancestor the
    /// (M5) compare reads, so an entry that omits it cannot round-trip (`overrides.md`
    /// → `tracked-fork` hash basis).
    #[test]
    fn tracked_fork_missing_base_hash_is_a_clear_error_not_a_panic() {
        let cfg = TempDir::new("fork-no-hash");
        fs::write(
            cfg.path().join("manifest.yaml"),
            "deltas:\n  \
             - kind: tracked-fork\n    \
             target: workflow:single-task#implement\n    \
             base-version: v1\n",
        )
        .expect("write manifest");

        // The 4-tuple's `OverrideLayer` is not `Debug`, so match the error out
        // rather than `expect_err` the whole `Ok` value.
        let msg = match load_project_layer(cfg.path()) {
            Ok(_) => panic!("a `tracked-fork` missing `base-hash` must be an error"),
            Err(e) => e.to_string(),
        };
        assert!(
            msg.contains("base-hash"),
            "the error must name the missing `base-hash` field; got: {msg}",
        );
    }
}
