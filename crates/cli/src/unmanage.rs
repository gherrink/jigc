//! `jigc unmanage <path>` — drop a single managed doc from jigc's index/state, the
//! inverse of `ingest`'s register-only `adopt` (M21 Increment 4, T1).
//!
//! The store-scope teardown verb (`project-setup.md` → Flow 2 hardening → Teardown /
//! cleanup (G5), bullet (a) un-manage a doc). Locates the repo + project layer, loads
//! the persisted schemas + the committed edge index ([`index::load_committed`]) + the
//! file-state record ([`FileStateRecord::load`]), re-derives the target's
//! `<type>:<slug>` identity from the path's location-match, and runs the engine's
//! [`engine::ingest::unmanage`] — dropping the doc's forward edges + its file-state
//! baseline, **leaving the file bytes on disk** (this verb never writes a managed
//! file). Idempotent: a re-run on an already-unmanaged doc is a clean no-op (exit 0,
//! nothing persisted).

use anyhow::{Context, Result};
use std::path::Path;

use engine::file_state::FileStateRecord;
use engine::index;
use engine::schema::Schema;

use crate::ingest::{load_schemas, require_project_layer};
use crate::pack::make_pack;
use crate::task::git_head;

/// The outcome of an un-manage run: the repo-relative path acted on, its re-derived
/// `<type>:<slug>` identity (when the path sits under a schema `location:`), and
/// whether anything was actually dropped (`false` = an already-unmanaged no-op).
#[derive(Clone, Debug, serde::Serialize)]
pub struct UnmanageReport {
    /// The forward-slash repo-relative path the verb un-managed.
    pub path: String,
    /// The doc's `<type>:<slug>` identity, when the path sits under a persisted
    /// schema's `location:`; `None` for a path under no schema location (only the
    /// file-state entry is dropped — there are no edges to drop).
    pub identity: Option<String>,
    /// Whether either surface (edges / baseline) actually changed. `false` on a
    /// re-run / a path that was never managed — a clean no-op.
    pub dropped: bool,
}

/// Run `jigc unmanage <rel_path>` against `cwd`: load the un-manage substrate (the
/// schemas, the committed index, the file-state record), re-derive the doc's identity,
/// drop both its forward edges and its file-state baseline via
/// [`engine::ingest::unmanage`], and persist the mutated surfaces **iff** something
/// changed (register-only — no managed file is ever written). A re-run on an
/// already-unmanaged doc reports a no-op drop and persists nothing.
pub(crate) fn run(cwd: &Path, rel_path: &str) -> Result<UnmanageReport> {
    let repo_root = require_project_layer(cwd)?;
    let pack = make_pack();
    let resolved =
        crate::start::resolve_severity_cascade(pack.as_ref(), &repo_root.join(".jigc/config"))?;
    let schemas = load_schemas(pack.as_ref(), &resolved)?;

    let jigc_root = repo_root.join(".jigc");
    let head = git_head(&repo_root)?;
    let schema_map: std::collections::BTreeMap<String, Schema> =
        schemas.iter().map(|s| (s.ty.clone(), s.clone())).collect();
    let mut index = index::load_committed(&repo_root, &jigc_root, &schema_map, &head);
    let mut record = FileStateRecord::load(&jigc_root)
        .with_context(|| format!("could not load the file-state record under {jigc_root:?}"))?;

    // Re-derive the `<type>:<slug>` identity from the path's location-match — symmetric
    // with `adopt`'s slug-from-stem, keyed off the schema whose `location:` the path
    // sits under. A path under no schema location has no edges to drop, so the identity
    // is `None` and `unmanage` drops only the file-state entry.
    let identity = identity_of(rel_path, &schemas);
    // The `from` argument: the identity when known, else the bare path (which matches no
    // edge — only the file-state baseline drops).
    let from = identity.clone().unwrap_or_else(|| rel_path.to_string());

    let dropped = engine::ingest::unmanage(&mut record, &mut index, &from, rel_path);

    // Persist only the surfaces a real drop mutated — a no-op writes nothing, so a
    // re-run leaves the on-disk index + record byte-identical.
    if dropped {
        index
            .save(&jigc_root)
            .with_context(|| format!("could not save the edge index under {jigc_root:?}"))?;
        record
            .save(&jigc_root)
            .with_context(|| format!("could not save the file-state record under {jigc_root:?}"))?;
    }

    Ok(UnmanageReport {
        path: rel_path.to_string(),
        identity,
        dropped,
    })
}

/// The `<type>:<slug>` identity for a managed-doc path — the persisted schema whose
/// `location:` dir the path sits directly under, joined with the filename stem (the
/// classify location-match, mirrored from the engine). `None` when no schema owns the
/// path's location.
fn identity_of(rel_path: &str, schemas: &[Schema]) -> Option<String> {
    let schema = schemas.iter().find(|s| under_location(rel_path, s))?;
    let slug = rel_path.rsplit('/').next().unwrap_or(rel_path);
    let slug = slug.strip_suffix(".md").unwrap_or(slug);
    Some(format!("{}:{slug}", schema.ty))
}

/// Whether `rel_path` sits directly under `schema`'s declared `location:` dir — the
/// classify location discriminator, mirrored from `engine::ingest`. A transient
/// (location-less) schema is never a home.
fn under_location(rel_path: &str, schema: &Schema) -> bool {
    let Some(location) = schema.location.as_deref() else {
        return false;
    };
    let dir = location.trim_end_matches('/');
    !dir.is_empty() && rel_path.starts_with(&format!("{dir}/"))
}
