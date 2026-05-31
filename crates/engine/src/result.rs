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

use crate::finding::{Finding, Severity};

/// The result-contract schema version. Bumped only when the JSON projection of a
/// public result type changes in a way an external consumer must notice.
pub const SCHEMA_VERSION: u32 = 1;

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

impl ValidationReport {
    /// Build a report over the aggregated `findings`, stamping the current
    /// [`SCHEMA_VERSION`].
    pub fn new(findings: Vec<Finding>) -> Self {
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

#[cfg(test)]
mod tests {
    use super::*;

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
                "schema_version": 1,
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
            serde_json::json!({ "state": "unset-project", "schema_version": 1 })
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
                "schema_version": 1,
                "header": "Pack: dev/v0.3.0 · Project config: .jigc/config",
                "workflows": [
                    { "id": "single-task", "when": "Implement one well-scoped change." }
                ]
            })
        );
    }

    /// `ValidationReport` projects to the stable `{schema_version, findings}` shape
    /// and `has_blocking()` is true iff any finding is blocking — the gate predicate
    /// `finalize` consults. An advisory-only report is non-blocking.
    #[test]
    fn validation_report_projection_and_has_blocking() {
        use crate::finding::Location;

        // Empty report: non-blocking, version-stamped, empty findings array.
        let empty = ValidationReport::new(Vec::new());
        assert!(!empty.has_blocking(), "empty report does not block");
        assert_eq!(
            serde_json::to_value(&empty).expect("serializes"),
            serde_json::json!({ "schema_version": 1, "findings": [] })
        );

        // Advisory-only: surfaced but does not block.
        let advisory = Finding {
            severity: Severity::Advisory,
            code: "file-state.baseline-adopt".into(),
            message: "baseline adopted".into(),
            location: Some(Location::addressed("docs/note:ok.md", 1, 1)),
            route: None,
        };
        let report = ValidationReport::new(vec![advisory.clone()]);
        assert!(
            !report.has_blocking(),
            "an advisory-only report does not block"
        );

        // A blocking finding flips the gate.
        let blocking = Finding {
            severity: Severity::Blocking,
            code: "schema-conformance.required-slot-present".into(),
            message: "required slot is empty".into(),
            location: None,
            route: None,
        };
        let report = ValidationReport::new(vec![advisory, blocking]);
        assert!(
            report.has_blocking(),
            "any blocking finding blocks the gate"
        );
    }
}
