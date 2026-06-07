//! `jigc describe` — the self-description surface (M11 Increment 2, T2).
//!
//! Locate → build the **pack-only** resolved definitions → assemble → (the
//! dispatch then renders the projection as free prose through the global
//! `--format`). Mirrors the `run_ingest` / `run_orient` shape: locate the repo +
//! project layer, [`make_pack`], then build the resolved definitions **pack-only**
//! (the *unfiltered* workflow set — every workflow, not the
//! `creates-task && selectable` catalog — plus the full doctype set + the command
//! catalog) and hand them to the engine's [`Description::assemble`] whole-menu
//! projection assembler (`design/introspection.md` → Command surface).
//!
//! Read-only by construction: it locates, reads the embedded pack, and assembles —
//! it composes nothing, mints nothing, and writes nothing. Cascade reflection
//! (reading workflow/doctype prose through the project layer) is a later increment;
//! M11 reads pack-only, so the project layer is required only as the "is this
//! project set up" gate, exactly as `jigc ingest` requires it.

use anyhow::{Context, Result, bail};
use std::path::{Path, PathBuf};

use engine::compose::{WorkflowDef, load_workflow_def};
use engine::finding::Finding;
use engine::introspect::Description;
use engine::packsource::{PackResourceKind, PackSource};
use engine::schema::Schema;

use crate::pack::make_pack;
use crate::start::load_catalog;

/// Assemble the whole-menu [`Description`] projection for the repo `describe` is run
/// from — **pack-only**. Locates the repo + project layer (the setup gate), reads
/// the embedded pack's unfiltered workflows + all doctypes + the command catalog,
/// and runs the engine's [`Description::assemble`]. Presentation-free — the dispatch
/// maps the result to the free-prose surface via `Format → render`.
pub(crate) fn run(cwd: &Path) -> Result<Description> {
    require_project_layer(cwd)?;
    let pack = make_pack();
    let pack = pack.as_ref();

    // The unfiltered workflow set — every workflow, not the `creates-task &&
    // selectable` catalog (`introspection.md` → enumeration is over the unfiltered
    // set). Each id paired with its parsed definition, so the assembler reads the
    // authored `description:` / `usage:` fields.
    let workflows: Vec<(String, WorkflowDef)> = load_workflow_defs(pack)?;
    let schemas: Vec<Schema> = load_schemas(pack)?;
    let catalog = load_catalog(pack)?;

    Ok(Description::assemble(
        workflows.iter().map(|(id, def)| (id.as_str(), def)),
        schemas.iter(),
        &catalog,
    ))
}

/// Load every shipped workflow definition from the embedded pack — the **unfiltered**
/// set (every workflow id the pack lists, parsed for its front-matter). The assembler
/// skips any that carry neither authored field, so no filtering happens here.
fn load_workflow_defs(pack: &dyn PackSource) -> Result<Vec<(String, WorkflowDef)>> {
    let mut out = Vec::new();
    for id in pack.list(PackResourceKind::Workflows) {
        let bytes = pack
            .read(PackResourceKind::Workflows, &id)
            .with_context(|| format!("the `{}` workflow reads back", id.as_str()))?;
        let def = load_workflow_def(&bytes).map_err(finding_to_err)?;
        out.push((id.as_str().to_owned(), def));
    }
    Ok(out)
}

/// Load every shipped doctype schema from the embedded pack — the full doctype set
/// the projection narrates (the engine stays domain-empty; the CLI feeds the pack in).
fn load_schemas(pack: &dyn PackSource) -> Result<Vec<Schema>> {
    let mut out = Vec::new();
    for id in pack.list(PackResourceKind::Schemas) {
        let bytes = pack
            .read(PackResourceKind::Schemas, &id)
            .with_context(|| format!("the `{}` schema reads back", id.as_str()))?;
        let schema = crate::pack::load_pack_schema(pack, &bytes)
            .with_context(|| format!("the `{}` schema parses", id.as_str()))?;
        out.push(schema);
    }
    Ok(out)
}

/// Locate the repo root and its `.jigc/config/` project layer — the same setup gate
/// `jigc ingest` uses. Errors with routed messages when the repo or the project layer
/// is absent.
fn require_project_layer(cwd: &Path) -> Result<PathBuf> {
    let repo_root = discover_repo_root(cwd)
        .with_context(|| format!("not inside a git repository (from {})", cwd.display()))?;
    let project_config = repo_root.join(".jigc").join("config");
    if !project_config.is_dir() {
        bail!(
            "this project isn't set up — run `jigc setup` (no `.jigc/config/` cascade layer found)"
        );
    }
    Ok(repo_root)
}

/// Walk up from `start` to the directory holding `.git` (the repo root).
fn discover_repo_root(start: &Path) -> Option<PathBuf> {
    start
        .ancestors()
        .find(|dir| dir.join(".git").exists())
        .map(PathBuf::from)
}

/// Map an engine [`Finding`] to an `anyhow` error carrying its message + route — the
/// same envelope the rest of the CLI surfaces blocking findings through.
fn finding_to_err(finding: Finding) -> anyhow::Error {
    let route = finding
        .route
        .map(|r| format!("\n  route: {r}"))
        .unwrap_or_default();
    anyhow::anyhow!("{}{route}", finding.message)
}
