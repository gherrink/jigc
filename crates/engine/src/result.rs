//! The engine's public result types — the renderer / JSON / future-MCP contract.
//!
//! A versioned semantic API (explicit serde projection + schema-version marker),
//! not incidental serde output. See `implementation/module-layout.md` → Renderers.
//!
//! The JSON projection of these types is the stable surface the three renderers,
//! external JSON consumers, and a future `mcp` frontend all bind to. Field names
//! are pinned with explicit `serde` attributes and the projection carries a
//! [`SCHEMA_VERSION`] marker, so a rename is an intentional, versioned change —
//! never an accident of `#[derive(Serialize)]`.

use serde::{Deserialize, Serialize};

use crate::cascade::{LayerKind, Resolved};
use crate::finding::{Finding, Severity};

/// The result-contract schema version. Bumped only when the JSON projection of a
/// public result type changes in a way an external consumer must notice.
pub const SCHEMA_VERSION: u32 = 2;

/// One workflow as it appears in orientation's catalog: a stable `id` and its
/// selection-guidance `when` line (the [`design/bootstrap.md`] orientation example
/// renders these as `single-task — Implement one well-scoped change…`).
///
/// [`design/bootstrap.md`]: ../../../design/bootstrap.md
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct CatalogEntry {
    /// Stable workflow id (e.g. `single-task`).
    pub id: String,
    /// One-line selection guidance shown next to the id.
    pub when: String,
}

impl CatalogEntry {
    /// Build a catalog entry from its id and selection-guidance line.
    pub fn new(id: impl Into<String>, when: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            when: when.into(),
        }
    }
}

/// The catalog of available workflows. Serializes transparently as a JSON array of
/// [`CatalogEntry`], so it projects to `workflows[]` when held by [`Orientation`].
#[derive(Clone, Debug, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Catalog {
    entries: Vec<CatalogEntry>,
}

impl Catalog {
    /// Build a catalog from its entries.
    pub fn new(entries: Vec<CatalogEntry>) -> Self {
        Self { entries }
    }

    /// Borrow the catalog entries in order.
    pub fn entries(&self) -> &[CatalogEntry] {
        &self.entries
    }
}

/// The read-only result of `jigc start` orientation: the version marker plus the
/// catalog of workflows the agent can choose from. This is the versioned contract
/// the renderers read — its JSON keys (`schema_version`, `workflows`) are stable.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Orientation {
    /// The result-contract schema version (see [`SCHEMA_VERSION`]).
    pub schema_version: u32,
    /// Available workflows, in display order.
    pub workflows: Catalog,
}

impl Orientation {
    /// Build an orientation result over a workflow catalog, stamping the current
    /// [`SCHEMA_VERSION`].
    pub fn new(workflows: Catalog) -> Self {
        Self {
            schema_version: SCHEMA_VERSION,
            workflows,
        }
    }
}

/// The full orientation result of bare `jigc start` — the versioned contract a
/// renderer maps over. A `state`-tagged sum of the two increment-1 orientation
/// states (`design/bootstrap.md` → Orientation output examples): `unset-project`
/// (no project cascade layer) and `clean` (cascade resolved, no active task).
///
/// Presentation-free by construction: it carries the *data* each state renders
/// from (the clean state's provenance `header` + workflow catalog), never the
/// rendered text — text formatting lives in `cli::render`, the engine stays
/// presentation-free. The `state` discriminator + explicit serde attributes pin
/// the JSON projection as the stable surface the renderers and JSON consumers
/// bind to (the `Orientation`/`Catalog` stance, applied to the whole result).
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "state", rename_all = "kebab-case")]
pub enum OrientationView {
    /// No project cascade layer is present — route the agent to `jigc setup`.
    UnsetProject {
        /// The result-contract schema version (see [`SCHEMA_VERSION`]).
        schema_version: u32,
    },
    /// Cascade resolved, no active task — carry the provenance `header` and the
    /// workflow catalog the clean-state renderer prints.
    Clean {
        /// The result-contract schema version (see [`SCHEMA_VERSION`]).
        schema_version: u32,
        /// The cascade/provenance header line (`Pack: … · Project config: …`).
        header: String,
        /// Available workflows, in display order.
        workflows: Catalog,
    },
}

impl OrientationView {
    /// The **unset-project** orientation view, stamping the current
    /// [`SCHEMA_VERSION`].
    pub fn unset_project() -> Self {
        Self::UnsetProject {
            schema_version: SCHEMA_VERSION,
        }
    }

    /// The **clean, no active task** orientation view over its provenance header
    /// and workflow catalog, stamping the current [`SCHEMA_VERSION`].
    pub fn clean(header: impl Into<String>, workflows: Catalog) -> Self {
        Self::Clean {
            schema_version: SCHEMA_VERSION,
            header: header.into(),
            workflows,
        }
    }
}

/// The aggregated result of a `validate` scope sweep — the versioned report both
/// `task validate` and `finalize` phase 2 read (`validation.md` → How it gates
/// `finalize`: one engine, two entry points, so what `validate` reports and what
/// `finalize` blocks on can never diverge).
///
/// It carries the severity-classified [`Finding`]s the probes raised over the
/// scope, in sweep order. [`ValidationReport::has_blocking`] is the gate predicate
/// `finalize` consults: it blocks iff any finding is [`Severity::Blocking`]
/// (advisory findings are surfaced but never stop the commit). Presentation-free —
/// the renderers in `cli::render` format the findings; the engine carries the data.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ValidationReport {
    /// The result-contract schema version (see [`SCHEMA_VERSION`]).
    pub schema_version: u32,
    /// Every finding the probes raised over the scope, in sweep order.
    pub findings: Vec<Finding>,
}

/// The MVP check inventory as the `(probe, check)` **membership set** — the keyed
/// (tunable + floored) surface the M6 post-pass matches on, the engine constant
/// mirroring `validation.md` → MVP check inventory. A `Finding` whose `(probe,
/// check)` is *not* a row here is **exempt** from the post-pass and keeps its
/// emitted severity (`validation.md` → Inventory = the keyed surface, not every
/// finding the engine emits; matched by membership, never code-prefix).
///
/// Membership is all this milestone's post-pass needs: severity is re-graded **only**
/// on an explicit `scalar-set` (override-only-on-explicit-delta) and otherwise kept
/// at the *emitted* severity, so the table never has to carry a declared default that
/// mirrors the code's literal. The per-check default severity + `floor` land in
/// `knobs.yaml` in the next increment (where `check_value` / `--explain` consume
/// them); the membership set is the engine knowledge this increment requires.
///
/// The five built `override-default` codes collapse onto three checks
/// (`target-exists` / `target-unchanged` / `basis-recorded`) via the `Finding.check`
/// field T1 already canonicalizes (`validation.md` → Code-id reconciliation), so the
/// inventory carries the three checks, not the five codes.
const CHECK_INVENTORY: &[(&str, &str)] = &[
    ("workflow-refs", "placeholder-resolves"),
    ("workflow-refs", "include-resolves"),
    ("workflow-refs", "command-ref-resolves"),
    ("workflow-refs", "include-cycle-absent"),
    ("workflow-refs", "at-marker-on-non-scalar"),
    ("workflow-refs", "run-marker-not-shadowed"),
    ("workflow-refs", "body-include-only"),
    ("schema-conformance", "ref-resolves"),
    ("schema-conformance", "required-slot-present"),
    ("schema-conformance", "required-field-present"),
    ("schema-conformance", "field-value-conformant"),
    ("pack-probe-integrity", "timeout"),
    ("pack-probe-integrity", "crash"),
    ("pack-probe-integrity", "malformed-output"),
    ("file-state", "hash-matches"),
    ("schema-completeness", "inverse-cardinality"),
    ("override-default", "target-exists"),
    ("override-default", "target-unchanged"),
    ("override-default", "basis-recorded"),
    ("commit-rendering", "line-limit-subject"),
    ("commit-rendering", "line-limit-body"),
];

/// Whether a `(probe, check)` is a keyed inventory row — the membership test that
/// gates the post-pass (a non-member finding is exempt and keeps its emitted
/// severity).
fn is_inventory_check(probe: &str, check: &str) -> bool {
    CHECK_INVENTORY
        .iter()
        .any(|(p, c)| *p == probe && *c == check)
}

/// The engine-owned severity post-pass (`validation.md` → Severity assignment — the
/// M6 post-pass): for each finding **whose `(probe, check)` is an inventory row**,
/// resolve its severity by the three-step lookup — per-check key
/// (`validation.<probe>.<check>.severity`), then per-probe key
/// (`validation.<probe>.severity`), then leave the emitted severity. A finding is
/// re-graded **only** when the resolved cascade actually carries a `scalar-set` for
/// one of those keys (override-only-on-explicit-delta); absent any delta it keeps the
/// emitted default, so the no-override path is byte-identical. Findings with no
/// inventory row are exempt entirely — matched by membership, never code-prefix.
fn assign_severity(findings: &mut [Finding], resolved: &Resolved) {
    for finding in findings {
        if !is_inventory_check(&finding.probe, &finding.check) {
            // Not a keyed row → exempt: keep the emitted severity.
            continue;
        }
        let per_check = format!("validation.{}.{}.severity", finding.probe, finding.check);
        let per_probe = format!("validation.{}.severity", finding.probe);
        if let Some(severity) = resolved
            .overridden_scalar(&per_check)
            .or_else(|| resolved.overridden_scalar(&per_probe))
            .and_then(Severity::from_token)
        {
            finding.severity = severity;
        }
    }
}

impl ValidationReport {
    /// Build a report over the aggregated `findings`, running the engine-owned
    /// severity post-pass against the resolved cascade before stamping the current
    /// [`SCHEMA_VERSION`] — the single construction point, so `has_blocking()` and
    /// every downstream consumer read *post-pass* severities (`validation.md` →
    /// Assigned once, at report construction — before the gate). On a no-delta
    /// cascade the post-pass is a no-op and output is byte-identical.
    pub fn new(mut findings: Vec<Finding>, resolved: &Resolved) -> Self {
        assign_severity(&mut findings, resolved);
        Self {
            schema_version: SCHEMA_VERSION,
            findings,
        }
    }

    /// Whether the scope holds at least one [`Severity::Blocking`] finding — the
    /// predicate `finalize` blocks on (`validation.md` → How it gates `finalize`).
    /// Advisory / warning findings do not count.
    pub fn has_blocking(&self) -> bool {
        self.findings
            .iter()
            .any(|f| f.severity == Severity::Blocking)
    }
}

/// Why a step sits at its position in the resolved include list: a `replace-step`
/// delta swapped the pack id at this slot for the resolving one. Carries the
/// replaced step id and the slot's **1-based** position in the post-phase-4 list,
/// so the renderer can emit the `← replaces <id> at position N` annotation
/// (`design/worked-examples.md` → 3a; `design/workflow-dialect.md` → `--explain`).
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Replacement {
    /// The step id this slot's entry replaced (the pack-default id swapped out).
    pub replaced: String,
    /// The 1-based position of this slot in the resolved include list.
    pub position: usize,
}

/// One step in the resolution tree: its resolved id, the cascade layer that owns
/// its file after phase-2 shadowing ([`LayerKind`]), and — when a `replace-step`
/// delta placed it — the [`Replacement`] annotation.
///
/// This is layer 2 of the `--explain` output contract (the include-expansion
/// tree, one entry per post-phase-4 include in order). The `source` path string
/// is **framing** the CLI renderer derives from `(id, layer)`; the engine model
/// carries the structural fact — which layer won — not the displayed path.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ResolvedStep {
    /// The resolved step id at this include-list slot.
    pub id: String,
    /// The cascade layer that owns this step's file (phase-2 by-id shadowing).
    pub layer: LayerKind,
    /// The `replace-step` annotation, when an override placed this id here.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replaces: Option<Replacement>,
}

/// One applied scalar-key override in the `--explain` tree's layer-1 provenance:
/// the knob key, its resolved value, and the cascade layer that won it by applying
/// a `scalar-set` (`design/workflow-dialect.md` → `--explain` output contract:
/// "any scalar-key overrides applied with their source layer";
/// `design/worked-examples.md` → 3b). A knob left at the pack-default base carries
/// no entry — only an overridden key appears here.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ScalarOverride {
    /// The scalar knob key a higher layer set (e.g. `default-workflow`).
    pub key: String,
    /// The resolved value the winning layer set for the knob.
    pub value: String,
    /// The cascade layer that won the knob (the layer that applied the `scalar-set`).
    pub layer: LayerKind,
}

/// One soft-rejected `scalar-set` in the `--explain` tree's layer-1 provenance:
/// a demotion a knob's `floor` dropped at cascade resolution — the attempted
/// value, the floor it ranked below, and the source layer that tried to set it
/// (`design/workflow-dialect.md` → `--explain` output contract: "any **rejected**
/// scalar-sets … shown distinctly from applied overrides with the attempted value,
/// the floor, and its source layer"; `design/overrides.md` → Soft-rejection). The
/// delta was **logged, not applied** — so the knob's value resolved from the
/// remaining layers; this records *why* the attempted demotion did not win,
/// distinct from the applied [`ScalarOverride`] lines.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct RejectedDemotion {
    /// The floored knob key the rejected `scalar-set` targeted.
    pub key: String,
    /// The below-floor value the layer attempted to set (dropped, not applied).
    pub attempted: String,
    /// The knob's `floor` the attempted value ranked below.
    pub floor: String,
    /// The cascade layer that attempted the below-floor `scalar-set`.
    pub layer: LayerKind,
}

/// The structural slice of the `--explain` resolution tree — layers 1–2 of the
/// output contract (`design/workflow-dialect.md` → `--explain` output contract):
/// the workflow's cascade provenance (`overrides applied: N` plus the per-knob
/// scalar-override lines) and the resolved include-expansion tree (each step
/// tagged with its source [`LayerKind`] and any `replace-step` annotation), in
/// post-phase-4 composed include order.
///
/// **Derived, never persisted** — re-computed each call from the same
/// `(definition + cascade)` (`workflow-dialect.md`: the tree is re-computed on
/// demand). A pure function of the resolved cascade; carries no presentation
/// (path strings, indentation) — `cli::render` frames it.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ResolutionTree {
    /// The result-contract schema version (see [`SCHEMA_VERSION`]).
    pub schema_version: u32,
    /// The workflow id whose resolution this tree explains.
    pub workflow: String,
    /// The cascade layer the workflow definition file resolved to.
    pub workflow_layer: LayerKind,
    /// The **total** applied-override count — `structural-op` deltas to this
    /// workflow's include list **plus** scalar-key overrides — i.e. exactly the
    /// `overrides applied: N` header total (so the JSON field and the agent-text
    /// header agree). The structural-only count is `overrides_applied -
    /// scalar_overrides.len()`; the scalar breakdown is `scalar_overrides`.
    pub overrides_applied: usize,
    /// The applied scalar-key overrides, each with its winning layer — layer 1 of
    /// the `--explain` output contract. Empty when every knob resolved from the
    /// pack-default base. Their count is included in `overrides_applied`.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub scalar_overrides: Vec<ScalarOverride>,
    /// The `scalar-set` demotions a knob's `floor` soft-rejected at cascade
    /// resolution — layer 1 of the `--explain` output contract, **distinct** from
    /// the applied `scalar_overrides` (logged, not applied). Empty when no floored
    /// key was set below its floor. Not counted in `overrides_applied` (a rejected
    /// delta did not become an override).
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub rejected_demotions: Vec<RejectedDemotion>,
    /// The resolved include steps, in post-phase-4 composed order.
    pub steps: Vec<ResolvedStep>,
}

impl ResolutionTree {
    /// Build a resolution tree over its workflow provenance + resolved steps,
    /// stamping the current [`SCHEMA_VERSION`].
    pub fn new(
        workflow: impl Into<String>,
        workflow_layer: LayerKind,
        overrides_applied: usize,
        scalar_overrides: Vec<ScalarOverride>,
        rejected_demotions: Vec<RejectedDemotion>,
        steps: Vec<ResolvedStep>,
    ) -> Self {
        Self {
            schema_version: SCHEMA_VERSION,
            workflow: workflow.into(),
            workflow_layer,
            overrides_applied,
            scalar_overrides,
            rejected_demotions,
            steps,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The `--explain` `ResolutionTree` projects under the **bumped** schema version
    /// (`SCHEMA_VERSION == 2`) and carries the new `rejected_demotions` field — one
    /// entry per soft-rejected below-floor `scalar-set`, each with its `key`,
    /// `attempted` value, `floor`, and source `layer`. The field is the stable surface
    /// `--explain` reads (`design/workflow-dialect.md` → `--explain` output contract;
    /// `design/overrides.md` → Soft-rejection). A tree with no rejection omits the key
    /// entirely (`skip_serializing_if`), so the no-rejection projection is unchanged
    /// but for the version bump.
    #[test]
    fn resolution_tree_projects_rejected_demotions_under_bumped_schema_version() {
        // The bump is intentional + versioned.
        assert_eq!(
            SCHEMA_VERSION, 2,
            "the projection change bumps the schema version"
        );

        let tree = ResolutionTree::new(
            "single-task",
            LayerKind::PackDefault,
            0,
            Vec::new(),
            vec![RejectedDemotion {
                key: "validation.workflow-refs.placeholder-resolves.severity".to_owned(),
                attempted: "advisory".to_owned(),
                floor: "blocking".to_owned(),
                layer: LayerKind::Project,
            }],
            Vec::new(),
        );

        let json = serde_json::to_value(&tree).expect("serializes");
        assert_eq!(json["schema_version"], serde_json::json!(2));
        let rejected = json["rejected_demotions"]
            .as_array()
            .expect("rejected_demotions is an array");
        assert_eq!(rejected.len(), 1);
        assert_eq!(
            rejected[0]["key"],
            serde_json::json!("validation.workflow-refs.placeholder-resolves.severity")
        );
        assert_eq!(rejected[0]["attempted"], serde_json::json!("advisory"));
        assert_eq!(rejected[0]["floor"], serde_json::json!("blocking"));
        assert_eq!(rejected[0]["layer"], serde_json::json!("project"));

        // A tree with no rejection omits the key entirely (skip_serializing_if).
        let clean = ResolutionTree::new(
            "single-task",
            LayerKind::PackDefault,
            0,
            Vec::new(),
            Vec::new(),
            Vec::new(),
        );
        let clean_json = serde_json::to_value(&clean).expect("serializes");
        assert!(
            clean_json.get("rejected_demotions").is_none(),
            "an empty rejected_demotions is omitted; got:\n{clean_json:#}",
        );
    }

    /// Constructing an `Orientation` over a two-entry catalog and serializing it
    /// must project to the documented, stable keys: `schema_version` at the root
    /// and `workflows[].id` / `workflows[].when` per entry. A field rename breaks
    /// this — that is the contract being locked.
    #[test]
    fn orientation_json_projection_is_the_stable_contract() {
        let orientation = Orientation::new(Catalog::new(vec![
            CatalogEntry::new("single-task", "Implement one well-scoped change."),
            CatalogEntry::new(
                "project-setup",
                "Set up the development pack on a fresh repo.",
            ),
        ]));

        let json = serde_json::to_value(&orientation).expect("serializes");

        // Root marker.
        assert_eq!(json["schema_version"], serde_json::json!(SCHEMA_VERSION));

        // The catalog projects under `workflows` as an array.
        let workflows = json["workflows"].as_array().expect("workflows is an array");
        assert_eq!(workflows.len(), 2);

        // Each entry exposes stable `id` / `when` keys.
        assert_eq!(workflows[0]["id"], serde_json::json!("single-task"));
        assert_eq!(
            workflows[0]["when"],
            serde_json::json!("Implement one well-scoped change.")
        );
        assert_eq!(workflows[1]["id"], serde_json::json!("project-setup"));

        // The full projection is exactly the documented shape — no stray keys.
        assert_eq!(
            json,
            serde_json::json!({
                "schema_version": 2,
                "workflows": [
                    { "id": "single-task", "when": "Implement one well-scoped change." },
                    { "id": "project-setup", "when": "Set up the development pack on a fresh repo." }
                ]
            })
        );
    }

    /// The two `OrientationView` states project to a `state`-tagged JSON shape:
    /// `unset-project` carries only the version marker; `clean` carries the
    /// provenance header + the catalog under `workflows`. The discriminator and
    /// keys are the stable contract a renderer / JSON consumer binds to.
    #[test]
    fn orientation_view_state_tagged_projection_is_the_stable_contract() {
        let unset = serde_json::to_value(OrientationView::unset_project()).expect("serializes");
        assert_eq!(
            unset,
            serde_json::json!({ "state": "unset-project", "schema_version": 2 })
        );

        let clean = serde_json::to_value(OrientationView::clean(
            "Pack: dev/v0.3.0 · Project config: .jigc/config",
            Catalog::new(vec![CatalogEntry::new(
                "single-task",
                "Implement one well-scoped change.",
            )]),
        ))
        .expect("serializes");
        assert_eq!(
            clean,
            serde_json::json!({
                "state": "clean",
                "schema_version": 2,
                "header": "Pack: dev/v0.3.0 · Project config: .jigc/config",
                "workflows": [
                    { "id": "single-task", "when": "Implement one well-scoped change." }
                ]
            })
        );
    }

    /// `ValidationReport` projects to the stable `{schema_version, findings}` shape
    /// and `has_blocking()` is true iff any finding is blocking — the gate predicate
    /// `finalize` consults. An advisory-only report is non-blocking. The reports here
    /// are built over a **no-delta** cascade, so the post-pass leaves every emitted
    /// severity untouched.
    #[test]
    fn validation_report_projection_and_has_blocking() {
        use crate::finding::Location;

        let resolved = no_delta_resolved();

        // Empty report: non-blocking, version-stamped, empty findings array.
        let empty = ValidationReport::new(Vec::new(), &resolved);
        assert!(!empty.has_blocking(), "empty report does not block");
        assert_eq!(
            serde_json::to_value(&empty).expect("serializes"),
            serde_json::json!({ "schema_version": 2, "findings": [] })
        );

        // Advisory-only: surfaced but does not block.
        let advisory = Finding::graded(
            Severity::Advisory,
            "file-state.baseline-adopt",
            "baseline adopted",
            Some(Location::addressed("docs/note:ok.md", 1, 1)),
            None,
        );
        let report = ValidationReport::new(vec![advisory.clone()], &resolved);
        assert!(
            !report.has_blocking(),
            "an advisory-only report does not block"
        );

        // A blocking finding flips the gate.
        let blocking = Finding::graded(
            Severity::Blocking,
            "schema-conformance.required-slot-present",
            "required slot is empty",
            None,
            None,
        );
        let report = ValidationReport::new(vec![advisory, blocking], &resolved);
        assert!(
            report.has_blocking(),
            "any blocking finding blocks the gate"
        );
    }

    /// A cascade with no `scalar-set` deltas over a pack declaring every per-check /
    /// per-probe severity knob the post-pass might read — so a finding's emitted
    /// severity is its assigned severity (the no-override path is byte-identical).
    fn no_delta_resolved() -> crate::cascade::Resolved {
        crate::cascade::resolve(&severity_pack(), None, None).expect("resolves")
    }

    /// A pack-default layer declaring the per-check and per-probe severity knobs the
    /// post-pass three-step lookup reads — the closed surface an `OverrideLayer` may
    /// `scalar-set` against (knobs.yaml gains these in increment 2; the post-pass is
    /// proven here over a synthetic surface, the planner's "fed a synthetic
    /// resolved-override map" form).
    fn severity_pack() -> crate::cascade::PackDefaultLayer {
        use std::collections::BTreeMap;
        let mut scalars = BTreeMap::new();
        for key in [
            "validation.file-state.hash-matches.severity",
            "validation.file-state.severity",
            "validation.commit-rendering.line-limit-subject.severity",
        ] {
            scalars.insert(key.to_owned(), "blocking".to_owned());
        }
        crate::cascade::PackDefaultLayer::new("dev-pack", "0.1.0", scalars, Vec::new())
    }

    /// The M6 post-pass (T2): severity is assigned over the aggregated findings at
    /// `ValidationReport::new`, overriding **only** when the resolved cascade carries
    /// a `scalar-set` for the finding's `(probe, check)` key. A per-check `scalar-set`
    /// flips a blocking finding to advisory *and* an advisory finding to blocking, and
    /// `has_blocking()` reflects the **assigned** severities (`validation.md` →
    /// Severity assignment — the M6 post-pass; the per-check, three-step lookup).
    #[test]
    fn post_pass_flips_severity_on_a_per_check_scalar_set() {
        // `file-state.hash-matches` (emitted blocking) is demoted to advisory;
        // `commit-rendering.line-limit-subject` (emitted advisory) is promoted to
        // blocking — both per-check `scalar-set`s on the closed knob surface.
        let project = crate::cascade::OverrideLayer::empty()
            .scalar_set("validation.file-state.hash-matches.severity", "advisory")
            .scalar_set(
                "validation.commit-rendering.line-limit-subject.severity",
                "blocking",
            );
        let resolved =
            crate::cascade::resolve(&severity_pack(), None, Some(&project)).expect("resolves");

        let drift = Finding::graded(
            Severity::Blocking,
            "file-state.hash-matches",
            "on-disk content drifted",
            None,
            None,
        );
        let line_limit = Finding::graded(
            Severity::Advisory,
            "commit-rendering.line-limit-subject",
            "subject is 73 chars",
            None,
            None,
        );

        let report = ValidationReport::new(vec![drift, line_limit], &resolved);

        // The demoted blocking check is now advisory; the promoted advisory check is
        // now blocking — assignment is keyed on the finding's `(probe, check)`.
        let by_check = |check: &str| {
            report
                .findings
                .iter()
                .find(|f| f.check == check)
                .map(|f| f.severity)
        };
        assert_eq!(
            by_check("hash-matches"),
            Some(Severity::Advisory),
            "a per-check `scalar-set` demotes the blocking finding"
        );
        assert_eq!(
            by_check("line-limit-subject"),
            Some(Severity::Blocking),
            "a per-check `scalar-set` promotes the advisory finding"
        );

        // The gate reflects the *assigned* severities: the promoted line-limit now
        // blocks (and the demoted drift no longer would on its own).
        assert!(
            report.has_blocking(),
            "has_blocking reflects the post-pass severities, got {:?}",
            report.findings
        );
    }

    /// No-delta byte-identity: over a cascade carrying no `scalar-set`, every finding
    /// keeps the **emitted** severity the probe produced — the determinism guard the
    /// milestone's #1 risk turns on (`validation.md` → the no-override path is
    /// byte-identical). A blocking inventory finding stays blocking; an advisory one
    /// stays advisory; the gate is unchanged.
    #[test]
    fn post_pass_no_delta_leaves_emitted_severity() {
        let resolved = no_delta_resolved();

        let drift = Finding::graded(
            Severity::Blocking,
            "file-state.hash-matches",
            "on-disk content drifted",
            None,
            None,
        );
        let line_limit = Finding::graded(
            Severity::Advisory,
            "commit-rendering.line-limit-subject",
            "subject is 73 chars",
            None,
            None,
        );

        let report = ValidationReport::new(vec![drift.clone(), line_limit.clone()], &resolved);

        assert_eq!(
            report.findings,
            vec![drift, line_limit],
            "a no-delta cascade leaves every finding byte-identical"
        );
        assert!(
            report.has_blocking(),
            "the emitted blocking finding still blocks"
        );
    }

    /// Exempt findings are untouched. A finding whose `(probe, check)` is **not** an
    /// inventory row (`schema-conformance.unknown-type` — a synthetic sibling with no
    /// row) is exempt from the post-pass even when a per-probe `scalar-set` exists for
    /// its probe: matching is by inventory membership, never code-prefix
    /// (`validation.md` → Matched by inventory membership, never by code-prefix). The
    /// per-probe key must not catch a determinism-boundary code that has no row.
    #[test]
    fn post_pass_exempts_a_non_inventory_check_under_a_keyed_probe() {
        use std::collections::BTreeMap;
        // Declare a per-probe key for `schema-conformance` and demote it.
        let mut scalars = BTreeMap::new();
        scalars.insert(
            "validation.schema-conformance.severity".to_owned(),
            "blocking".to_owned(),
        );
        let pack = crate::cascade::PackDefaultLayer::new("dev-pack", "0.1.0", scalars, Vec::new());
        let project = crate::cascade::OverrideLayer::empty()
            .scalar_set("validation.schema-conformance.severity", "advisory");
        let resolved = crate::cascade::resolve(&pack, None, Some(&project)).expect("resolves");

        // `unknown-type` is not an inventory row → exempt from the per-probe fallback.
        let exempt = Finding::graded(
            Severity::Blocking,
            "schema-conformance.unknown-type",
            "staged doc has an undefined type",
            None,
            None,
        );
        let report = ValidationReport::new(vec![exempt], &resolved);

        assert_eq!(
            report.findings[0].severity,
            Severity::Blocking,
            "a non-inventory check is exempt — the per-probe key never catches it"
        );
    }
}
