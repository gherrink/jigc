//! Orphan detection — committed managed-looking docs stranded **outside** the resolved
//! doctype roots after a `docs-root` re-point (M36; `design/validation.md` → Orphan
//! detection; `design/storage.md` → docs-root).
//!
//! Two consumers share the location-basename predicate here (CLI-side because the engine
//! ships empty by invariant and never shells to git for tracked status):
//!
//! - [`orphaned_docs`] backs the store-scope `file-state.orphaned-doc` advisory pushed by
//!   `jigc validate` — a committed `.md` whose immediate-parent dir basename matches a
//!   doctype's resolved `location:` basename yet falls outside that doctype's *current*
//!   resolved root.
//! - [`docs_root_would_orphan`] backs the pre-write `jigc config set docs-root` warning —
//!   the live docs a re-point to a *different* root would strand.
//!
//! **The match predicate (pinned).** Doctypes carry no filename pattern (`id-from: title`
//! mints an arbitrary slug), so the only stable discriminator is the `location:`
//! **directory** basename (`decisions`, `specs`, …), which survives a `docs-root` change.
//! Matching by the `.md` extension alone is explicitly wrong — it would flag `README.md`
//! and every stray markdown file (a front-door false-positive). Enumeration is over
//! `git ls-files` (committed truth that survives a fresh clone, where the finalize-gated
//! file-state map does not). A `git` failure yields none — a best-effort advisory, never
//! an error.

use std::path::Path;

use engine::schema::Schema;

/// The `location:` directory **basename** of a persisted schema (`decisions` for a
/// resolved `docs/decisions/`), or `None` for a transient (location-less) schema. Stable
/// across a `docs-root` change — the discriminator the predicate keys on.
pub(crate) fn location_basename(schema: &Schema) -> Option<&str> {
    let dir = schema.location.as_deref()?.trim_end_matches('/');
    Path::new(dir).file_name()?.to_str()
}

/// Whether `rel` (a repo-relative path) sits directly under `schema`'s **resolved**
/// `location:` dir — mirrors `unmanage::under_location`. A transient (location-less)
/// schema is never a home.
pub(crate) fn under_location(rel: &str, schema: &Schema) -> bool {
    let Some(dir) = schema.location.as_deref().map(|l| l.trim_end_matches('/')) else {
        return false;
    };
    !dir.is_empty() && rel.starts_with(&format!("{dir}/"))
}

/// The immediate-parent directory basename of a repo-relative `.md` path — `decisions`
/// for `docs/decisions/foo.md`. `None` for a root-level file (`README.md`), which is never
/// a managed doc — the false-positive a bare extension match would trip.
fn parent_basename(rel: &str) -> Option<&str> {
    let (parent, _file) = rel.rsplit_once('/')?;
    Some(parent.rsplit('/').next().unwrap_or(parent))
}

/// Enumerate `git ls-files` under `repo_root`, returning the committed `.md` paths,
/// address-sorted. A `git` failure yields an empty listing (best-effort advisory).
fn committed_markdown(repo_root: &Path) -> Vec<String> {
    let Ok(listing) = crate::task::git_capture(repo_root, &["ls-files"]) else {
        return Vec::new();
    };
    let mut out: Vec<String> = listing
        .lines()
        .filter(|l| l.ends_with(".md"))
        .map(str::to_string)
        .collect();
    out.sort();
    out
}

/// The committed `.md`s that **look like** managed docs (their immediate-parent dir
/// basename matches some persisted doctype's resolved `location:` basename) yet fall
/// **outside** that doctype's current resolved root — the `docs-root`-changed orphans.
/// `schemas` must carry the `docs-root`-applied `location:` (the resolved roots).
/// Address-sorted.
pub(crate) fn orphaned_docs<'a>(
    repo_root: &Path,
    schemas: impl IntoIterator<Item = &'a Schema>,
) -> Vec<String> {
    let schemas: Vec<&Schema> = schemas.into_iter().collect();
    let mut hits = Vec::new();
    for rel in committed_markdown(repo_root) {
        let Some(base) = parent_basename(&rel) else {
            continue; // a root-level file — never a managed doc.
        };
        // The doctype whose resolved location basename matches — at most one (basenames
        // are unique per doctype). No match → ordinary prose, never flagged.
        let Some(schema) = schemas.iter().find(|s| location_basename(s) == Some(base)) else {
            continue;
        };
        // Directly under the CURRENT resolved root → a live managed doc, not an orphan.
        if !under_location(&rel, schema) {
            hits.push(rel);
        }
    }
    hits
}

/// The committed managed docs currently under an **old** resolved root that a `docs-root`
/// re-point to a *different* root would strand. `old_schemas` must carry the *current*
/// (`old_docs_root`-applied) `location:`. When the normalized root does not actually
/// change, nothing orphans (an empty list). Address-sorted.
pub(crate) fn docs_root_would_orphan<'a>(
    repo_root: &Path,
    old_schemas: impl IntoIterator<Item = &'a Schema>,
    old_docs_root: &str,
    new_docs_root: &str,
) -> Vec<String> {
    // Normalize both to `apply_docs_root`'s rule: strip surrounding slashes; `""` / `.`
    // are the flat repo-root layout.
    let norm = |v: &str| {
        let trimmed = v.trim_matches('/');
        if trimmed == "." { "" } else { trimmed }.to_string()
    };
    if norm(old_docs_root) == norm(new_docs_root) {
        return Vec::new(); // the root does not move — nothing orphans.
    }
    let old_schemas: Vec<&Schema> = old_schemas.into_iter().collect();
    let mut hits = Vec::new();
    for rel in committed_markdown(repo_root) {
        // A live managed doc under a current resolved root moves out from under it when
        // the root re-points → it would orphan.
        if old_schemas.iter().any(|s| under_location(&rel, s)) {
            hits.push(rel);
        }
    }
    hits
}

#[cfg(test)]
mod tests {
    use super::*;

    fn adr_schema(location: &str) -> Schema {
        let yaml = format!(
            "type: adr\nlocation: {location}\nid-from: title\nsections:\n  - id: body\n    slot: {{ hint: x }}\n"
        );
        engine::schema::load_schema(yaml.as_bytes()).expect("adr schema loads")
    }

    #[test]
    fn location_basename_is_the_trailing_dir_regardless_of_docs_root() {
        assert_eq!(
            location_basename(&adr_schema("decisions/")),
            Some("decisions")
        );
        assert_eq!(
            location_basename(&adr_schema("docs/decisions/")),
            Some("decisions")
        );
        assert_eq!(
            location_basename(&adr_schema("archive/decisions/")),
            Some("decisions")
        );
    }

    #[test]
    fn parent_basename_is_none_for_a_root_level_file() {
        assert_eq!(parent_basename("README.md"), None);
        assert_eq!(parent_basename("docs/guide.md"), Some("docs"));
        assert_eq!(
            parent_basename("docs/decisions/cache.md"),
            Some("decisions")
        );
    }
}
