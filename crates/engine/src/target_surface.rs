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
//! - **`check_id`** — the resolved field-type's predicate, `field.check` if the
//!   schema field declares one else the pack-declared field-type's `check`
//!   (`field.check ?? field_type.check`; M13's per-field-type predicate selector).
//!   So `adr.cites-code` / `arch-doc.components/implemented-by` (bare `code-anchor`)
//!   inherit `symbol-exists`, while `spec.criteria/maps-to-test` (an explicit
//!   `check: criterion-maps-to-test`) keeps the test predicate. **Position is not
//!   the discriminator** — a repeatable anchor may resolve to `symbol-exists`
//!   (`architecture-documentation.md` → The per-field-type predicate selector;
//!   `validation.md` → What it checks).
//!
//! The list is **deterministically address-sorted** — the stable order the
//! serializable snapshot ([`crate::validate`]'s T3 materialization) carries.

use crate::field_block::Value;
use crate::finding::{Finding, Location, Severity};
use crate::parse::{ParsedSection, parse_sections};
use crate::schema::{FieldType, Schema, SectionBody};
use crate::state::{DOCS_DIR, RolesRecord};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::path::Path;

/// The pack-declared field-type name whose leaves are the target surface.
const CODE_ANCHOR: &str = "code-anchor";

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
    /// The resolved predicate the `doc-code` probe applies: `field.check` if the
    /// schema field declares one, else the field-type's pack-declared `check`
    /// (M13's selector — *not* chosen by section position).
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
///
/// Returns `(anchors, guard_findings)`: the second element carries any **multi-valued
/// non-silent guard** findings ([`collect_from_source`]) — a `code-anchor` field that
/// parsed to a [`Value::List`] is not an anchor shape this projection enumerates, but
/// it is *not silently dropped* either, so a future `0..*` code-anchor can never pass
/// unchecked-but-green (`validation.md` → Store-scope re-validation, "Multi-valued
/// anchors get a non-silent guard"). No shipped `code-anchor` is list-valued, so over
/// the shipped pack this is always empty.
pub fn enumerate_target_surface(
    task_dir: &Path,
    repo_root: &Path,
    schemas: &BTreeMap<String, Schema>,
) -> std::io::Result<(Vec<TargetAnchor>, Vec<Finding>)> {
    let mut anchors = Vec::new();
    let mut guard_findings = Vec::new();

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
                collect_from_source(schema, ty, slug, &source, &mut anchors, &mut guard_findings);
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
        collect_from_source(schema, ty, slug, &source, &mut anchors, &mut guard_findings);
    }

    anchors.sort_by(|a, b| a.address.cmp(&b.address));
    anchors.dedup();
    Ok((anchors, guard_findings))
}

/// Parse one doc against its schema and collect its `code-anchor` leaves into
/// `anchors`. An unparseable instance contributes nothing (best-effort).
///
/// `guard_findings` accumulates the **multi-valued non-silent guard** (`validation.md`
/// → Store-scope re-validation): a `code-anchor` field that parsed to a
/// [`Value::List`] is not enumerated as an anchor, but is surfaced as one loud
/// blocking finding rather than silently dropped. The store walk ([T2]) lifts this
/// projection **verbatim**, so the guard is a property of the projection, shared by
/// both the task-scope and store-scope callers.
pub fn collect_from_source(
    schema: &Schema,
    ty: &str,
    slug: &str,
    source: &str,
    anchors: &mut Vec<TargetAnchor>,
    guard_findings: &mut Vec<Finding>,
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
                collect_simple(section, parsed, ty, slug, anchors, guard_findings);
            }
            SectionBody::Repeatable { repeatable } => {
                collect_repeatable(
                    repeatable,
                    parsed,
                    section,
                    ty,
                    slug,
                    anchors,
                    guard_findings,
                );
            }
        }
    }
}

/// Collect header / simple-section `code-anchor` fields, each carrying the
/// predicate the M13 selector resolves (`field.check ?? field_type.check`).
pub fn collect_simple(
    section: &crate::schema::Section,
    parsed: &ParsedSection,
    ty: &str,
    slug: &str,
    anchors: &mut Vec<TargetAnchor>,
    guard_findings: &mut Vec<Finding>,
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
        let address = format!("{ty}:{slug}#{}/{}", section.id, declared.id);
        if let Some(value) = scalar(&present.value, &address, guard_findings) {
            anchors.push(TargetAnchor {
                address,
                anchor_value: value,
                check_id: resolve_check_id(declared),
            });
        }
    }
}

/// Collect repeatable-item `code-anchor` leaves, each carrying the predicate the
/// M13 selector resolves (`field.check ?? field_type.check`) — *not* chosen by
/// position, so a repeatable anchor may resolve to `symbol-exists`.
pub fn collect_repeatable(
    repeatable: &crate::schema::Repeatable,
    parsed: &ParsedSection,
    section: &crate::schema::Section,
    ty: &str,
    slug: &str,
    anchors: &mut Vec<TargetAnchor>,
    guard_findings: &mut Vec<Finding>,
) {
    // The block's `code-anchor` leaves (a field, never the id-source / a slot).
    let anchor_fields: Vec<&crate::schema::Field> = repeatable
        .block
        .iter()
        .filter_map(|leaf| match leaf {
            crate::schema::Leaf::Field(f) if is_code_anchor(&f.ty) => Some(f.as_ref()),
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
            let address = format!("{ty}:{slug}#{}/{}/{}", section.id, item.id, declared.id);
            if let Some(value) = scalar(&present.value, &address, guard_findings) {
                anchors.push(TargetAnchor {
                    address,
                    anchor_value: value,
                    check_id: resolve_check_id(declared),
                });
            }
        }
    }
}

/// The `doc-code` predicate for one `code-anchor` field: the field's own `check:`
/// override if it declares one, else the resolved field-type's pack-declared
/// `check` (M13's per-field-type predicate selector — position no longer
/// discriminates). Only ever called on a field [`is_code_anchor`] accepted, so the
/// type is a resolved [`FieldType::Pack`] carrying `check: Some(_)` (the post-T1
/// invariant); a `None` there is a structurally-impossible mis-resolved schema and
/// is surfaced as a panic, never a silent default (the absent-default trap).
pub fn resolve_check_id(field: &crate::schema::Field) -> String {
    if let Some(check) = &field.check {
        return check.clone();
    }
    let FieldType::Pack(pack) = &field.ty else {
        unreachable!(
            "resolve_check_id is only called on a code-anchor (a resolved pack field type)"
        );
    };
    pack.check.clone().expect(
        "a resolved pack field type carries check: Some(_) (the post-T1 invariant); \
         a None here is a mis-resolved schema, not a default to silently fill",
    )
}

/// Whether a field's resolved type is the pack-declared `code-anchor`.
pub fn is_code_anchor(ty: &FieldType) -> bool {
    matches!(ty, FieldType::Pack(p) if p.name == CODE_ANCHOR)
}

/// The scalar text of a `code-anchor` field value, or `None` (with a **loud guard
/// finding** pushed) when the value is a [`Value::List`].
///
/// An anchor is a single opaque scalar. A list-valued `code-anchor` is not an anchor
/// shape this projection enumerates — no shipped field is list-valued, so no
/// list-element walk is built (generality for a non-existent case). But it must **not**
/// be silently dropped: at store scope a dropped list would read as a false "all
/// clear." So a list value yields **no** anchor and **one** blocking guard finding
/// addressed at the offending field — a future `0..*` code-anchor can never pass
/// unchecked-but-green (`validation.md` → Store-scope re-validation, "Multi-valued
/// anchors get a non-silent guard"). Coordinate is `(1, 1)` — the field's exact source
/// position is not carried at this projection layer (the `doc-code.symbol-exists`
/// convention).
fn scalar(value: &Value, address: &str, guard_findings: &mut Vec<Finding>) -> Option<String> {
    match value {
        Value::Scalar(s) => Some(s.clone()),
        Value::List(_) => {
            guard_findings.push(Finding::graded(
                Severity::Blocking,
                "doc-code.multi-valued-anchor",
                format!(
                    "code-anchor `{address}` is list-valued; multi-valued anchors are \
                     not enumerated (no list-element check is built) — resolve to a \
                     single anchor or add list-element support"
                ),
                Some(Location::addressed(address.to_string(), 1, 1)),
                None,
            ));
            None
        }
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
    //!
    //! Each anchor's `check_id` is the **M13 selector** result (`field.check ??
    //! field_type.check`), *not* a section-position choice: the `adr.cites-code`
    //! header anchor (bare `code-anchor`) inherits `symbol-exists`; the `spec`
    //! `maps-to-test` criterion anchor (an explicit `check: criterion-maps-to-test`)
    //! keeps the test predicate; and a **repeatable** anchor with no field override
    //! resolves to the type's `symbol-exists` — the position-independence proof
    //! (`architecture-documentation.md` → The per-field-type predicate selector).

    use super::*;
    use crate::schema::{dev_pack_field_types, load_schema_with_types};
    use std::path::PathBuf;

    const ADR_YAML: &[u8] = include_bytes!("../../cli/pack/schemas/adr.yaml");

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

    /// A `spec` schema whose `criteria/maps-to-test` carries an **explicit**
    /// `check: criterion-maps-to-test` override — the shipped predicate, now chosen
    /// by `check:` not position (the post-T2 selector; T3 lands the same override on
    /// the shipped `spec.yaml`). Inline here so this increment's enumeration proof
    /// doesn't depend on T3's file edit.
    const SPEC_WITH_CHECK_OVERRIDE: &[u8] = b"\
type: spec
location: specs/
id-from: title
sections:
  - id: goal
    slot: { hint: One sentence. }
  - id: context
    slot: { hint: Forces. }
  - id: criteria
    repeatable:
      id-from: title
      block:
        - { id: title, type: string }
        - { id: statement, slot: { hint: Testably phrased. } }
        - { id: maps-to-test, type: code-anchor, check: criterion-maps-to-test }
";

    fn schemas() -> BTreeMap<String, Schema> {
        let mut m = BTreeMap::new();
        m.insert(
            "adr".to_string(),
            load_schema_with_types(ADR_YAML, &dev_pack_field_types()).expect("adr.yaml loads"),
        );
        m.insert(
            "spec".to_string(),
            load_schema_with_types(SPEC_WITH_CHECK_OVERRIDE, &dev_pack_field_types())
                .expect("spec fixture loads"),
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

    /// A `created` ADR whose `cites-code` field parses to a [`Value::List`] (the
    /// inline-flow `[a, b]` form). No shipped `code-anchor` is list-valued, so the
    /// projection builds no list-element enumeration — but it must not *silently
    /// drop* the value either (a false "all clear"). It emits a loud guard finding.
    const ADR_WITH_LIST_ANCHOR: &str = "\
---
status: accepted
date: 2026-05-23
cites-code: [crates/engine/src/a.rs#one, crates/engine/src/b.rs#two]
---

# List-valued anchor

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

        let (anchors, guard_findings) =
            enumerate_target_surface(&task_dir, repo.path(), &schemas()).expect("enumerates");

        // No list-valued anchor in this fixture: zero guard findings.
        assert!(
            guard_findings.is_empty(),
            "no list-valued code-anchor here, so no guard finding: {guard_findings:?}",
        );

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

    /// (M13, the position-independence proof) A doctype whose **repeatable**
    /// section carries a **bare** `code-anchor` leaf (no field `check:` override)
    /// enumerates to `check_id: symbol-exists` — the type-level `check`, inherited
    /// via the M13 selector. Pre-T2 the positional constant forced *every*
    /// repeatable-item anchor to `criterion-maps-to-test`; this is the regression
    /// that proves the predicate is pack-declared and position-independent (the
    /// `arch-doc.components/implemented-by` shape — `architecture-documentation.md`
    /// → The per-field-type predicate selector).
    #[test]
    fn repeatable_anchor_inherits_type_level_symbol_exists() {
        // A doctype whose repeatable items each carry a bare `code-anchor`.
        const ARCH_SCHEMA: &[u8] = b"\
type: arch-doc
location: architecture/
id-from: title
sections:
  - id: components
    repeatable:
      id-from: name
      block:
        - { id: name, type: string }
        - { id: implemented-by, type: code-anchor }
";
        // One created instance with a single component item carrying the anchor.
        // No front-matter — the schema declares no header section.
        const ARCH_INSTANCE: &str = "\
# The index

## Components

### The edge index  {#edge-index}

<!-- fields -->
- implemented-by: crates/engine/src/index.rs#overlay_working
";

        let repo = TempRoot::new("repeatable-symbol-exists");
        let task_dir = repo.path().join(".jigc").join("tasks").join("arch");
        stage(&task_dir, "arch-doc:the-index", ARCH_INSTANCE);

        let mut schemas = BTreeMap::new();
        schemas.insert(
            "arch-doc".to_string(),
            load_schema_with_types(ARCH_SCHEMA, &dev_pack_field_types())
                .expect("arch-doc fixture loads"),
        );

        let (anchors, guard_findings) =
            enumerate_target_surface(&task_dir, repo.path(), &schemas).expect("enumerates");

        assert!(
            guard_findings.is_empty(),
            "no list anchor: {guard_findings:?}"
        );

        // The repeatable-item anchor resolves to the *type-level* `symbol-exists`,
        // not the deleted positional `criterion-maps-to-test`.
        assert_eq!(
            anchors,
            vec![TargetAnchor {
                address: "arch-doc:the-index#components/edge-index/implemented-by".to_string(),
                anchor_value: "crates/engine/src/index.rs#overlay_working".to_string(),
                check_id: "symbol-exists".to_string(),
            }],
            "a repeatable `code-anchor` with no field override inherits the \
             type-level `symbol-exists` — position is not the discriminator",
        );
    }

    /// (M18, the multi-valued non-silent guard — `validation.md` → Store-scope
    /// re-validation, "Multi-valued anchors get a non-silent guard.") A
    /// `code-anchor` field whose value parses to a [`Value::List`] is **not** an
    /// anchor shape the projection enumerates (no shipped field is list-valued, so
    /// no list-element enumeration is built). The contract is that it is **not
    /// silently dropped** — the projection emits a loud guard `Finding` so a future
    /// `0..*` code-anchor can never pass unchecked-but-green. Here a staged ADR's
    /// `cites-code` carries the inline-flow list `[a, b]`: the projection yields
    /// **zero** anchors for that field and **one** blocking guard finding addressed
    /// at the field. (The mechanism lands here; its firing at *store* scope over a
    /// committed list fixture is proven in T2.)
    #[test]
    fn list_valued_anchor_emits_guard_finding_not_a_dropped_anchor() {
        let repo = TempRoot::new("list-anchor-guard");
        let task_dir = repo.path().join(".jigc").join("tasks").join("guard");
        stage(&task_dir, "adr:list-anchor", ADR_WITH_LIST_ANCHOR);

        let (anchors, guard_findings) =
            enumerate_target_surface(&task_dir, repo.path(), &schemas()).expect("enumerates");

        // The list value is NOT enumerated as an anchor (no list-element walk built).
        assert!(
            anchors.is_empty(),
            "a list-valued code-anchor yields no enumerated anchor, got {anchors:?}",
        );

        // It is NOT silently dropped: exactly one loud guard finding, addressed at
        // the offending field, blocking.
        assert_eq!(
            guard_findings.len(),
            1,
            "the list value emits exactly one guard finding (not a silent drop): \
             {guard_findings:?}",
        );
        let f = &guard_findings[0];
        assert_eq!(f.severity, crate::finding::Severity::Blocking);
        assert_eq!(f.code, "doc-code.multi-valued-anchor");
        assert_eq!(
            f.location.as_ref().and_then(|l| l.address.as_deref()),
            Some("adr:list-anchor#status/cites-code"),
            "the guard finding points at the offending field's address",
        );
    }
}
