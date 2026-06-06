//! Target-surface enumeration — the `(target-address, anchor-value, check-id)`
//! pairs a task must adjudicate, named directly over its **effective-state docs**.
//!
//! See `design/validation.md` → The `doc-code` probe (M10) → Target surface (the
//! design-review B1 fix) and `implementation/doctype-map.md` (a `code-anchor` is a
//! **field, not an edge**).
//!
//! ## Why direct enumeration (never the edge index)
//!
//! A `code-anchor` is a probe-checked *field*, not a structural *edge*, so it is
//! **not** in the edge index, and the inbound-edge blast-radius walk
//! ([`crate::index`]) cannot reach a citing doc from changed code. `doc-code`
//! therefore **enumerates `code-anchor` leaves directly** over the task's
//! effective-state docs (`validation.md` → Target surface):
//!
//! - **created/edited** — the task's staged doc instances (`<task_dir>/docs/*.md`,
//!   the same `staged_instances` surface [`crate::validate`] sweeps), covering both
//!   a `created` (minted-in-task) and an `edited-from-base` (copied-in) doc;
//! - **bound** — the docs bound into the task's read roles
//!   (`<task_dir>/roles.json`), each resolved to its **committed** bytes at
//!   `<repo_root>/<location>/<slug>.md` (the proven path the CLI's `{{@task.spec}}`
//!   slice reads — [`crate::store::canonical_path`]).
//!
//! A committed doc **neither edited nor bound** is *not* in effective state and
//! contributes **zero** pairs — the per-task gate blocks only on anchors the task's
//! own state carries, never on pre-existing drift in unrelated committed docs
//! (`validation.md` → Target surface; the masking-trap guard, hardening #5).
//!
//! ## The pairs
//!
//! Each `code-anchor` leaf yields one [`TargetAnchor`]:
//!
//! - **`address`** — the leaf's URI-shaped address ([`crate::address`]): a header /
//!   simple-section field is `<type>:<slug>#<section>/<field>`; a repeatable-item
//!   leaf is `<type>:<slug>#<section>/<item>/<field>`.
//! - **`anchor_value`** — the opaque `code-anchor` text the agent authored
//!   (`<path>#<symbol>`), untouched (the schema, not this layer, interprets it).
//! - **`check_id`** — `symbol-exists` for a header / simple-section anchor (the
//!   `adr.cites-code` floor), `criterion-maps-to-test` for a repeatable-item anchor
//!   (the `spec` `criteria/<id>/maps-to-test` headline). The position *is* the
//!   discriminator: a criterion's anchor is the only one carrying the test predicate
//!   (`validation.md` → What it checks).
//!
//! The list is **deterministically address-sorted** — the stable order the
//! serializable snapshot ([`crate::validate`]'s T3 materialization) carries.

use crate::field_block::Value;
use crate::parse::{ParsedSection, parse_sections};
use crate::schema::{FieldType, Schema, SectionBody};
use crate::state::{DOCS_DIR, RolesRecord};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::path::Path;

/// The pack-declared field-type name whose leaves are the target surface.
const CODE_ANCHOR: &str = "code-anchor";

/// The check id for a header / simple-section `code-anchor` (the `adr.cites-code`
/// floor): *does the cited code still exist?*
const SYMBOL_EXISTS: &str = "symbol-exists";

/// The check id for a repeatable-item `code-anchor` (the `spec`
/// `criteria/<id>/maps-to-test` headline): *does the criterion map to a real test?*
const CRITERION_MAPS_TO_TEST: &str = "criterion-maps-to-test";

/// One enumerated target-surface anchor: the leaf's [`address`](Self::address), the
/// opaque [`anchor_value`](Self::anchor_value) the agent authored, and the
/// [`check_id`](Self::check_id) the `doc-code` probe applies to it.
///
/// Serializable so the T3 snapshot can carry the enumerated set verbatim to a
/// subprocess probe (`validation.md` → What the snapshot must carry for `doc-code`).
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct TargetAnchor {
    /// The leaf's URI-shaped address (`<type>:<slug>#<section>/<field>` for a
    /// header field, `<type>:<slug>#<section>/<item>/<field>` for an item leaf).
    pub address: String,
    /// The opaque `code-anchor` value the agent authored (`<path>#<symbol>`).
    pub anchor_value: String,
    /// `symbol-exists` (header/simple anchor) or `criterion-maps-to-test` (item anchor).
    pub check_id: String,
}

/// Enumerate the task's target surface: every `code-anchor` leaf over the task's
/// effective-state docs (created/edited ∪ bound), as a deterministically
/// address-sorted list of [`TargetAnchor`]s.
///
/// `task_dir` is the working area (`<jigc_root>/tasks/<id>/`); `repo_root` is the
/// committed-store root the bound read-roles resolve their `<location>/<slug>.md`
/// against; `schemas` maps a doctype name to its resolved [`Schema`] (the engine
/// stays domain-empty — the caller feeds the cascade-resolved set in).
///
/// Effective state is the union of two surfaces:
/// - **created/edited** — the staged `<task_dir>/docs/*.md` instances;
/// - **bound** — the `<task_dir>/roles.json` addresses, each resolved to its
///   committed bytes via [`crate::store::canonical_path`].
///
/// A staged instance whose type prefix has no schema, an unparseable instance, a
/// bound role whose address is unparseable / type-less / has no committed file is
/// skipped (best-effort, mirroring [`crate::index::overlay_working`]): conformance
/// is the `schema-conformance` gate's concern, not enumeration's. The output is
/// address-sorted so it is reproducible regardless of directory-read order.
pub fn enumerate_target_surface(
    task_dir: &Path,
    repo_root: &Path,
    schemas: &BTreeMap<String, Schema>,
) -> std::io::Result<Vec<TargetAnchor>> {
    let mut anchors = Vec::new();

    // Surface a — created/edited: the staged `docs/*.md` instances.
    let docs = task_dir.join(DOCS_DIR);
    match std::fs::read_dir(&docs) {
        Ok(read) => {
            for entry in read {
                let entry = entry?;
                if !entry.file_type()?.is_file() {
                    continue;
                }
                let name = entry.file_name().to_string_lossy().into_owned();
                let Some(stem) = name.strip_suffix(".md") else {
                    continue;
                };
                // The filename stem is the `<type>:<slug>` address slug.
                let Some((ty, slug)) = stem.split_once(':') else {
                    continue;
                };
                let Some(schema) = schemas.get(ty) else {
                    continue; // unknown type: not the enumeration's gate.
                };
                let bytes = std::fs::read(entry.path())?;
                let source = String::from_utf8_lossy(&bytes);
                collect_from_source(schema, ty, slug, &source, &mut anchors);
            }
        }
        Err(err) if err.kind() == std::io::ErrorKind::NotFound => {}
        Err(err) => return Err(err),
    }

    // Surface b — bound: the read-roles resolved to their committed bytes.
    let roles = RolesRecord::load(task_dir)?;
    for address in roles.roles.values() {
        let Some((ty, slug)) = address.split_once(':') else {
            continue;
        };
        // Drop any in-container fragment — a bound role names a whole doc.
        let slug = slug.split('#').next().unwrap_or(slug);
        let Some(schema) = schemas.get(ty) else {
            continue;
        };
        let Some(committed) = crate::store::canonical_path(repo_root, schema, slug) else {
            continue; // a transient (location-less) type has no committed file.
        };
        let Ok(bytes) = std::fs::read(&committed) else {
            continue; // not present at base: nothing bound-resolvable to enumerate.
        };
        let mut source = String::from_utf8_lossy(&bytes).into_owned();
        crate::parse::strip_leading_bom(&mut source);
        collect_from_source(schema, ty, slug, &source, &mut anchors);
    }

    anchors.sort_by(|a, b| a.address.cmp(&b.address));
    anchors.dedup();
    Ok(anchors)
}

/// Parse one doc against its schema and collect its `code-anchor` leaves into
/// `anchors`. An unparseable instance contributes nothing (best-effort).
fn collect_from_source(
    schema: &Schema,
    ty: &str,
    slug: &str,
    source: &str,
    anchors: &mut Vec<TargetAnchor>,
) {
    let Ok(doc) = parse_sections(schema, source) else {
        return;
    };
    for section in &schema.sections {
        let Some(parsed) = doc.sections.iter().find(|s| s.id == section.id) else {
            continue;
        };
        match &section.body {
            SectionBody::Simple { .. } => {
                collect_simple(section, parsed, ty, slug, anchors);
            }
            SectionBody::Repeatable { repeatable } => {
                collect_repeatable(repeatable, parsed, section, ty, slug, anchors);
            }
        }
    }
}

/// Collect header / simple-section `code-anchor` fields → `symbol-exists`.
fn collect_simple(
    section: &crate::schema::Section,
    parsed: &ParsedSection,
    ty: &str,
    slug: &str,
    anchors: &mut Vec<TargetAnchor>,
) {
    let SectionBody::Simple { fields, .. } = &section.body else {
        return;
    };
    for declared in fields {
        if !is_code_anchor(&declared.ty) {
            continue;
        }
        let Some(present) = parsed.fields.iter().find(|f| f.key == declared.id) else {
            continue; // optional + absent: no anchor to adjudicate.
        };
        if let Some(value) = scalar(&present.value) {
            anchors.push(TargetAnchor {
                address: format!("{ty}:{slug}#{}/{}", section.id, declared.id),
                anchor_value: value,
                check_id: SYMBOL_EXISTS.to_string(),
            });
        }
    }
}

/// Collect repeatable-item `code-anchor` leaves → `criterion-maps-to-test`.
fn collect_repeatable(
    repeatable: &crate::schema::Repeatable,
    parsed: &ParsedSection,
    section: &crate::schema::Section,
    ty: &str,
    slug: &str,
    anchors: &mut Vec<TargetAnchor>,
) {
    // The block's `code-anchor` leaves (a field, never the id-source / a slot).
    let anchor_fields: Vec<&crate::schema::Field> = repeatable
        .block
        .iter()
        .filter_map(|leaf| match leaf {
            crate::schema::Leaf::Field(f) if is_code_anchor(&f.ty) => Some(f),
            _ => None,
        })
        .collect();
    if anchor_fields.is_empty() {
        return;
    }
    for item in &parsed.items {
        for declared in &anchor_fields {
            let Some(present) = item.fields.iter().find(|f| f.key == declared.id) else {
                continue;
            };
            if let Some(value) = scalar(&present.value) {
                anchors.push(TargetAnchor {
                    address: format!("{ty}:{slug}#{}/{}/{}", section.id, item.id, declared.id),
                    anchor_value: value,
                    check_id: CRITERION_MAPS_TO_TEST.to_string(),
                });
            }
        }
    }
}

/// Whether a field's resolved type is the pack-declared `code-anchor`.
fn is_code_anchor(ty: &FieldType) -> bool {
    matches!(ty, FieldType::Pack(p) if p.name == CODE_ANCHOR)
}

/// The scalar text of a field value (an anchor is a single opaque scalar; a list
/// value is not an anchor shape and is skipped).
fn scalar(value: &Value) -> Option<String> {
    match value {
        Value::Scalar(s) => Some(s.clone()),
        Value::List(_) => None,
    }
}

#[cfg(test)]
mod tests {
    //! The enumeration's done-criterion (`validation.md` → Target surface): over a
    //! task whose effective state is a **created** `adr` (a working delta with a
    //! `cites-code` header anchor) ∪ a **bound** `spec` (a committed read-role doc
    //! with a `maps-to-test` criterion anchor), enumeration finds **both** anchors;
    //! a committed `adr`/`spec` that is **neither edited nor bound** contributes
    //! **zero** pairs (the masking-trap guard, hardening #5); a doc with **no**
    //! `code-anchor` contributes zero pairs.

    use super::*;
    use crate::schema::{dev_pack_field_types, load_schema_with_types};
    use std::path::PathBuf;

    const ADR_YAML: &[u8] = include_bytes!("../../cli/pack/schemas/adr.yaml");
    const SPEC_YAML: &[u8] = include_bytes!("../../cli/pack/schemas/spec.yaml");

    /// A throwaway directory tree that removes itself on drop.
    struct TempRoot(PathBuf);

    impl TempRoot {
        fn new(tag: &str) -> Self {
            let mut path = std::env::temp_dir();
            path.push(format!(
                "jigc-target-surface-{tag}-{}-{:?}",
                std::process::id(),
                std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap()
                    .as_nanos(),
            ));
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

    fn schemas() -> BTreeMap<String, Schema> {
        let mut m = BTreeMap::new();
        m.insert(
            "adr".to_string(),
            load_schema_with_types(ADR_YAML, &dev_pack_field_types()).expect("adr.yaml loads"),
        );
        m.insert(
            "spec".to_string(),
            load_schema_with_types(SPEC_YAML, &dev_pack_field_types()).expect("spec.yaml loads"),
        );
        m
    }

    /// A `created` ADR staged in the task `docs/` area, carrying a `cites-code`
    /// header anchor.
    const ADR_WITH_ANCHOR: &str = "\
---
status: accepted
date: 2026-05-23
cites-code: crates/engine/src/validate.rs#validate_task
---

# Single-node session cache

## Context
Forces.

## Decision
A single node.

## Consequences
Cold node loses sessions.
";

    /// A committed SPEC (the bound read-role doc) with one criterion carrying a
    /// `maps-to-test` anchor.
    const SPEC_WITH_ANCHOR: &str = "\
---
title: Rate limiting
---

# Rate limiting

## Goal
Limit requests.

## Context
Bursts overwhelm the gateway.

## Criteria

### Rate limit holds at 100/min  {#rate-limit}
The gateway rejects the 101st request.

<!-- fields -->
- maps-to-test: crates/engine/src/validate.rs#validate_task
";

    /// A committed ADR carrying its own `cites-code` anchor — present in the store
    /// but **neither edited nor bound** by the task, so it must contribute zero pairs.
    const COMMITTED_UNRELATED_ADR: &str = "\
---
status: accepted
date: 2026-05-01
cites-code: crates/engine/src/store.rs#canonical_path
---

# Unrelated decision

## Context
Forces.

## Decision
Decided.

## Consequences
Effects.
";

    /// A `created` ADR with **no** `cites-code` anchor — contributes zero pairs.
    const ADR_NO_ANCHOR: &str = "\
---
status: accepted
date: 2026-05-23
---

# No-anchor decision

## Context
Forces.

## Decision
Decided.

## Consequences
Effects.
";

    /// Stage a doc at `<task_dir>/docs/<type>:<slug>.md`.
    fn stage(task_dir: &Path, addr: &str, body: &str) {
        let docs = task_dir.join(DOCS_DIR);
        std::fs::create_dir_all(&docs).expect("mk docs/");
        std::fs::write(docs.join(format!("{addr}.md")), body).expect("stage");
    }

    /// Commit a doc at `<repo_root>/<location>/<slug>.md`.
    fn commit(repo_root: &Path, location: &str, slug: &str, body: &str) {
        let dir = repo_root.join(location);
        std::fs::create_dir_all(&dir).expect("mk location");
        std::fs::write(dir.join(format!("{slug}.md")), body).expect("commit");
    }

    #[test]
    fn enumerates_created_and_bound_anchors_skips_unrelated_committed() {
        let repo = TempRoot::new("repo");
        let jigc = repo.path().join(".jigc");
        let task_dir = jigc.join("tasks").join("rate-limit");

        // Effective state — surface a (created/edited): a staged ADR with an anchor,
        // and a staged ADR with NO anchor (must contribute zero).
        stage(&task_dir, "adr:single-node-cache", ADR_WITH_ANCHOR);
        stage(&task_dir, "adr:no-anchor", ADR_NO_ANCHOR);

        // Effective state — surface b (bound): a SPEC committed to specs/, bound to
        // the `spec` read-role in roles.json.
        commit(repo.path(), "specs", "rate-limiting", SPEC_WITH_ANCHOR);
        let mut roles = RolesRecord::new();
        roles.bind("spec", "spec:rate-limiting");
        roles.save(&task_dir).expect("save roles");

        // NOT in effective state: a committed ADR with its own anchor, neither
        // staged nor bound — the masking-trap guard. It must yield zero pairs.
        commit(
            repo.path(),
            "decisions",
            "unrelated",
            COMMITTED_UNRELATED_ADR,
        );

        let anchors =
            enumerate_target_surface(&task_dir, repo.path(), &schemas()).expect("enumerates");

        // Exactly the two effective-state anchors, address-sorted.
        assert_eq!(
            anchors,
            vec![
                TargetAnchor {
                    address: "adr:single-node-cache#status/cites-code".to_string(),
                    anchor_value: "crates/engine/src/validate.rs#validate_task".to_string(),
                    check_id: "symbol-exists".to_string(),
                },
                TargetAnchor {
                    address: "spec:rate-limiting#criteria/rate-limit/maps-to-test".to_string(),
                    anchor_value: "crates/engine/src/validate.rs#validate_task".to_string(),
                    check_id: "criterion-maps-to-test".to_string(),
                },
            ],
            "found both the created ADR's header anchor and the bound SPEC's item \
             anchor, and skipped the unrelated committed ADR + the no-anchor doc",
        );

        // The unrelated committed ADR's anchor never appears.
        assert!(
            !anchors
                .iter()
                .any(|a| a.anchor_value.contains("store.rs#canonical_path")),
            "an unrelated committed doc (neither edited nor bound) must contribute \
             zero pairs, got {anchors:?}",
        );
    }
}
