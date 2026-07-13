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
//! A committed doc **neither edited nor bound** is *not* in this `enumerate_target_surface`
//! pairing — the task's *own* effective state carries only its authored/edited/bound docs,
//! never unrelated committed ones (the masking-trap guard, hardening #5). **But the
//! per-task `finalize` gate is not limited to this surface:** the **code-anchor blast
//! radius** ([`crate::validate::schedule_doc_code`]) additionally drags in *committed*
//! anchors whose target file the task changed, and blocks on ones that **newly** dangle
//! (resolved at `HEAD`, gone at the staged index) — so a task that renames a symbol cannot
//! commit a dangling citation in a committed doc it never opened, while pre-existing drift
//! in a touched file is *not* re-attributed to it (`validation.md` → Scope = effective
//! state, the code-anchor blast radius). This module owns only the task's-own-state
//! enumeration; the blast radius is the floor's separate, deliberate surface.
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

/// Enumerate the **committed store's** target surface: every `code-anchor` leaf over
/// every committed doc in every active schema `location:`, as a deterministically
/// address-sorted list of [`TargetAnchor`]s. This is the **store-scope** sweep
/// (`validation.md` → Store-scope re-validation) — **task-less and read-only**: it
/// has no working area, opens **no `FileStateRecord` write**, and touches **no edge
/// index**.
///
/// `repo_root` is the committed-store root; `schemas` maps a doctype name to its
/// resolved [`Schema`] (the engine stays domain-empty — the caller feeds the
/// cascade-resolved set in). For each schema declaring a persisted `location:`, this
/// globs `<repo_root>/<location>/*.md` (its **own** deterministic, address-sorted
/// read-dir + `.md`-filter walk, reusing the proven sorted-enumeration idiom of the
/// CLI's `committed_store` / [`crate::store::canonical_path`]), parses each doc, and
/// collects its `code-anchor` leaves via the shared [`collect_from_source`]
/// projection (so the predicate selector + multi-valued guard apply identically to
/// the task scope). A **placement** doctype (`storage.md` → Placement) has no location
/// dir: its one committed instance is the literal `placement.file` at the fixed
/// `<type>:<type>` singleton identity, enumerated through the census's shared
/// [`crate::index::committed_instances`] (M42 — before it, the whole placement class fell
/// through the transient arm and the family reported a clean store over anchors it never
/// read: `storage.md` → The census). A transient (location-less, non-placement) doctype
/// contributes nothing.
///
/// The located glob keeps its **own** `read_dir` walk rather than routing through the
/// shared enumerator too: `committed_instances` *swallows* a directory-read error (an
/// unreadable `decisions/` reads as "no committed docs"), while this walk propagates it —
/// and a store-scope sweep that cannot read a managed location must be **loud**, never a
/// clean-looking empty. Sharing the enumerator for the placement arm (where the only I/O
/// is an `exists()` on one literal path) costs nothing in loudness and buys the census its
/// single home resolution.
///
/// This walk **deliberately surfaces unrelated committed docs** — the inversion of
/// the task-scope masking-trap guard ([`enumerate_target_surface`]): at task scope a
/// committed doc neither edited nor bound is excluded so a task can't be blocked by
/// drift it didn't cause; at store scope surfacing exactly that pre-existing drift is
/// the command's entire purpose. A future reader must **not** "fix" this to honor the
/// task-gate exclusion (`validation.md` → Store-scope re-validation, the inversion).
///
/// It **must not** route through [`crate::file_state::reconcile_committed_store`] —
/// that path mutates the file-state record + edge index (re-baselining drifted-clean
/// docs via *absorb*, keyed off a `task_dir` a task-less sweep lacks), so reusing it
/// would *silently re-baseline genuine drift* the sweep exists to surface. This is a
/// pure read.
///
/// One doctype per `location:` is **assumed** (true for the three code-anchor
/// doctypes today — `adr`/`decisions`, `arch-doc`/`architecture`, `spec`/`specs`).
/// The walk claims files by `location:`, so a future pack sharing a `location:`
/// across doctypes would need the walk to disambiguate by parsed type; no
/// disambiguation is built (`validation.md` → Scope of the sweep).
///
/// Returns `(anchors, guard_findings)`: the second element carries the **multi-valued
/// non-silent guard** findings ([`collect_from_source`]) — a committed `code-anchor`
/// that parsed to a [`Value::List`] is not enumerated but is surfaced as one loud
/// blocking finding rather than silently dropped (a false store-wide "all clear").
pub fn enumerate_committed_surface(
    repo_root: &Path,
    schemas: &BTreeMap<String, Schema>,
) -> std::io::Result<(Vec<TargetAnchor>, Vec<Finding>)> {
    let mut anchors = Vec::new();
    let mut guard_findings = Vec::new();

    for schema in schemas.values() {
        // A **placement** doctype (`storage.md` → Placement) has no `location:` dir to
        // glob — its single committed instance is the literal repo-root-relative
        // `placement.file`, addressed by the fixed `<type>:<type>` singleton slug. It is
        // enumerated through the census's shared placement-aware enumerator
        // ([`crate::index::committed_instances`] — the sibling family 5 and the file-state
        // twin already route through), never a fourth hand-rolled home resolution: the
        // identity comes from the enumerator, since a case-preserved literal home
        // (`VISION.md`) does not round-trip through slug derivation. Keyed on
        // `schema.location` alone (as this walk was until M42), the whole placement class
        // dropped through the *transient* arm below and the family reported a clean store
        // over anchors it never read (`storage.md` → The census).
        if schema.placement.is_some() {
            for (identity, path) in crate::index::committed_instances(repo_root, &schema.ty, schema)
            {
                let Some((_, slug)) = identity.split_once(':') else {
                    continue;
                };
                let bytes = std::fs::read(&path)?;
                let mut source = String::from_utf8_lossy(&bytes).into_owned();
                crate::parse::strip_leading_bom(&mut source);
                collect_from_source(
                    schema,
                    &schema.ty,
                    slug,
                    &source,
                    &mut anchors,
                    &mut guard_findings,
                );
            }
            continue;
        }
        let Some(location) = schema.location.as_deref() else {
            continue; // a transient (location-less) type has no committed instances.
        };
        let dir = repo_root.join(location);
        let read = match std::fs::read_dir(&dir) {
            Ok(read) => read,
            Err(err) if err.kind() == std::io::ErrorKind::NotFound => continue,
            Err(err) => return Err(err),
        };
        // Collect the location's `.md` stems and sort them, so the per-doc walk is
        // deterministic regardless of directory-read order (the `committed_store`
        // sorted-enumeration idiom).
        let mut slugs: Vec<String> = Vec::new();
        for entry in read {
            let entry = entry?;
            if !entry.file_type()?.is_file() {
                continue;
            }
            let name = entry.file_name().to_string_lossy().into_owned();
            if let Some(stem) = name.strip_suffix(".md") {
                slugs.push(stem.to_owned());
            }
        }
        slugs.sort();
        for slug in &slugs {
            let path = dir.join(format!("{slug}.md"));
            let bytes = std::fs::read(&path)?;
            let mut source = String::from_utf8_lossy(&bytes).into_owned();
            crate::parse::strip_leading_bom(&mut source);
            collect_from_source(
                schema,
                &schema.ty,
                slug,
                &source,
                &mut anchors,
                &mut guard_findings,
            );
        }
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
                // Title↔symbol consistency (pack-declared opt-in). The prose blind
                // spot the `symbol-exists` anchor check alone misses: a capable model
                // fixes the anchor to clear the gate and leaves the heading naming the
                // *old* symbol (the long-horizon study's Opus failure). We flag a
                // title that carries a **compound code-identifier** (a camelCase /
                // PascalCase token — a hump like `…tB…`/`…kP…`) which is **not** the
                // anchored symbol: that token is a stale/wrong symbol name. A purely
                // descriptive title (`Session store`, `Cache Layer`) has no such token
                // and never trips — so descriptive headings stay legal. Pure string
                // work (no code resolution), emitted as a `doc-code.*` guard so the
                // floor/hook catch it. Bound: a compound *tech word* in a title
                // (`WebSocket`) that is not the symbol would flag — acceptable and rare.
                // Emitted **advisory** (M40 — keyed as
                // `validation.doc-code.title-names-symbol.severity`, re-promotable by
                // a project scalar-set): the brand-name false-positive class
                // (`WordPress`, `PostgreSQL`) has no valid remedy under blocking —
                // renaming the heading would be *wrong* — which wedged the agent
                // (`validation.md` → the M40 knob + demotion block).
                if let Some((_, symbol)) = value.split_once('#')
                    && declared.title_names_symbol
                    && !symbol.is_empty()
                {
                    let title_symbols: Vec<&str> = item
                        .title
                        .split(|c: char| !c.is_alphanumeric() && c != '_')
                        .filter(|t| is_compound_identifier(t))
                        .collect();
                    if !title_symbols.is_empty() && !title_symbols.contains(&symbol) {
                        let title_address = format!(
                            "{ty}:{slug}#{}/{}/{}",
                            section.id, item.id, repeatable.id_from
                        );
                        guard_findings.push(Finding::graded(
                            Severity::Advisory,
                            "doc-code.title-names-symbol",
                            format!(
                                "component title `{}` names symbol(s) {title_symbols:?} \
                                 but its anchor implements `{symbol}` (`{value}`) — the \
                                 heading still names a renamed/removed symbol; update the \
                                 title to match the code",
                                item.title
                            ),
                            Some(Location::addressed(title_address, 1, 1)),
                            Some(format!(
                                "start a task (`jigc start \"<intent>\"`), then run \
                                 `jigc doc retitle-item {ty}:{slug}#{}/{} --title \
                                 \"<new title>\"` within it — `retitle-item` needs an \
                                 active task, and the heading retitles with its `{{#id}}` \
                                 anchor frozen (a descriptive title that drops the stale \
                                 symbol also clears this)",
                                section.id, item.id
                            )),
                        ));
                    }
                }
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

/// Whether `token` reads as a **compound code identifier** — a multi-word symbol
/// name. This is the [`title-names-symbol`](crate::schema::Field::title_names_symbol)
/// discriminator: a heading token shaped like a symbol is treated as a symbol name
/// (so a stale one is caught), while a plain descriptive word is left alone, so
/// descriptive titles stay legal. Two shapes count, covering both casing idioms:
///
/// - a **camelCase hump** — a lower→upper transition (`CommentBlockParser`, `myVar`,
///   `WebSocket`);
/// - an **acronym-then-word** — two+ uppercase then lowercase (`UIDoc`, `HTTPServer`,
///   `IOError`), which has no lower→upper hump but is still plainly a symbol.
///
/// A single capitalized word (`Session`), an all-lower word (`store`), and a bare
/// acronym (`HTTP`, `API`) match neither and are left alone. Bound: a compound tech
/// word in a title (`WebSocket`, `OAuth`) that is not the symbol would flag — rare.
fn is_compound_identifier(token: &str) -> bool {
    let b = token.as_bytes();
    let camel_hump = b
        .windows(2)
        .any(|w| w[0].is_ascii_lowercase() && w[1].is_ascii_uppercase());
    let acronym_then_word = b.windows(3).any(|w| {
        w[0].is_ascii_uppercase() && w[1].is_ascii_uppercase() && w[2].is_ascii_lowercase()
    });
    camel_hump || acronym_then_word
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

## Options
Alternatives were weighed and rejected.

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

## Options
Alternatives were weighed and rejected.

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

## Options
Alternatives were weighed and rejected.

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

## Options
Alternatives were weighed and rejected.

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

    /// A `code-anchor` field carrying `title-names-symbol: true` flags when the
    /// repeatable item's **title** does not contain the symbol the anchor names —
    /// the long-horizon study's Opus prose blind spot (the agent fixed the anchor
    /// to `CommentTagParser` to clear the symbol-exists gate but left the heading
    /// `CommentBlockParser`). The opt-in is pack-declared so a doctype whose item
    /// titles are prose (spec criteria) is unaffected. Emitted **advisory** by
    /// default (M40 — the brand-name false-positive class has no valid remedy
    /// under blocking; re-promotable via the cascade knob), with a route naming
    /// the verb that performs the repair, `jigc doc retitle-item`.
    #[test]
    fn title_names_symbol_flags_advisory_when_title_does_not_name_the_anchored_symbol() {
        const ARCH_SCHEMA: &[u8] = b"\
type: arch-doc
location: architecture/
id-from: title
sections:
  - id: components
    repeatable:
      id-from: title
      block:
        - { id: title, type: string }
        - { id: implemented-by, type: code-anchor, title-names-symbol: true }
";
        // The anchor was updated to the renamed symbol; the title was NOT (the
        // exact Opus failure shape). Anchor symbol = `CommentTagParser`, title =
        // `CommentBlockParser` → the title does not name its symbol.
        const ARCH_INSTANCE: &str = "\
# Core public API

## Components

### CommentBlockParser  {#commentblockparser}

<!-- fields -->
- implemented-by: packages/core/src/CommentTagParser.ts#CommentTagParser
";

        let repo = TempRoot::new("title-names-symbol-drift");
        let task_dir = repo.path().join(".jigc").join("tasks").join("arch");
        stage(&task_dir, "arch-doc:core-public-api", ARCH_INSTANCE);

        let mut schemas = BTreeMap::new();
        schemas.insert(
            "arch-doc".to_string(),
            load_schema_with_types(ARCH_SCHEMA, &dev_pack_field_types())
                .expect("arch-doc fixture loads"),
        );

        let (anchors, guard_findings) =
            enumerate_target_surface(&task_dir, repo.path(), &schemas).expect("enumerates");

        // The anchor itself is still enumerated (symbol-exists runs against the code).
        assert_eq!(anchors.len(), 1, "the code-anchor is still enumerated");
        // …and the title-mismatch raises exactly one advisory guard finding.
        assert_eq!(
            guard_findings.len(),
            1,
            "one title finding: {guard_findings:?}"
        );
        let f = &guard_findings[0];
        assert_eq!(f.code, "doc-code.title-names-symbol");
        assert_eq!(
            f.severity,
            Severity::Advisory,
            "advisory-by-default (M40 demotion)"
        );
        // The route names the verb that performs the repair, with the concrete
        // item address — not a retitle no verb could perform.
        let route = f.route.as_deref().expect("the finding carries a route");
        assert!(
            route.contains(
                "jigc doc retitle-item arch-doc:core-public-api#components/commentblockparser"
            ),
            "the route names `jigc doc retitle-item` at the item address; got: {route}"
        );
        // (V8) The check fires task-less on `jigc validate`, but `retitle-item`
        // bails "no active task" — so the route must first carry the minting step
        // (`jigc start`), else it commands a verb no context can perform.
        assert!(
            route.contains("jigc start"),
            "the route carries the task-minting step so the retitle is performable; got: {route}"
        );

        // Control: when the title DOES name the symbol, no finding.
        const CONSISTENT: &str = "\
# Core public API

## Components

### CommentTagParser  {#commenttagparser}

<!-- fields -->
- implemented-by: packages/core/src/CommentTagParser.ts#CommentTagParser
";
        let repo2 = TempRoot::new("title-names-symbol-clean");
        let task_dir2 = repo2.path().join(".jigc").join("tasks").join("arch");
        stage(&task_dir2, "arch-doc:core-public-api", CONSISTENT);
        let (_a2, g2) =
            enumerate_target_surface(&task_dir2, repo2.path(), &schemas).expect("enumerates");
        assert!(g2.is_empty(), "consistent title raises no finding: {g2:?}");

        // Control 2: a purely DESCRIPTIVE title (no compound identifier) raises no
        // finding even though it does not literally contain the symbol — descriptive
        // component headings stay legal (the false-positive the camel-hump rule avoids).
        const DESCRIPTIVE: &str = "\
# Core public API

## Components

### Session store  {#session-store}

<!-- fields -->
- implemented-by: packages/core/src/SessionStore.ts#SessionStore
";
        let repo3 = TempRoot::new("title-descriptive-ok");
        let task_dir3 = repo3.path().join(".jigc").join("tasks").join("arch");
        stage(&task_dir3, "arch-doc:core-public-api", DESCRIPTIVE);
        let (_a3, g3) =
            enumerate_target_surface(&task_dir3, repo3.path(), &schemas).expect("enumerates");
        assert!(g3.is_empty(), "descriptive title raises no finding: {g3:?}");
    }

    /// The arch-doc schema fixture + a staged brand-name-titled component whose
    /// anchor names a different symbol — the M40 knob tests' shared drift shape
    /// (`WordPress` is a compound identifier by the camel-hump rule, but renaming
    /// the heading to the symbol would be *wrong*: the false-positive class the
    /// demotion exists for).
    fn wordpress_guard_findings(tag: &str) -> Vec<Finding> {
        const ARCH_SCHEMA: &[u8] = b"\
type: arch-doc
location: architecture/
id-from: title
sections:
  - id: components
    repeatable:
      id-from: title
      block:
        - { id: title, type: string }
        - { id: implemented-by, type: code-anchor, title-names-symbol: true }
";
        const ARCH_INSTANCE: &str = "\
# Integrations

## Components

### WordPress adapter  {#wordpress-adapter}

<!-- fields -->
- implemented-by: src/wp/adapter.rs#WpAdapter
";
        let repo = TempRoot::new(tag);
        let task_dir = repo.path().join(".jigc").join("tasks").join("arch");
        stage(&task_dir, "arch-doc:integrations", ARCH_INSTANCE);

        let mut schemas = BTreeMap::new();
        schemas.insert(
            "arch-doc".to_string(),
            load_schema_with_types(ARCH_SCHEMA, &dev_pack_field_types())
                .expect("arch-doc fixture loads"),
        );
        let (_anchors, guard_findings) =
            enumerate_target_surface(&task_dir, repo.path(), &schemas).expect("enumerates");
        guard_findings
    }

    /// The resolved cascade over the **shipped** knob surface (the embedded
    /// `knobs.yaml` bytes, production floor-wiring), with an optional project layer.
    fn shipped_cascade(
        project: Option<&crate::cascade::OverrideLayer>,
    ) -> crate::cascade::Resolved {
        let knobs = crate::knobs::load_knobs(include_bytes!("../../cli/pack/config/knobs.yaml"))
            .expect("knobs.yaml loads");
        let pack =
            crate::cascade::PackDefaultLayer::new("dev", "0.1.0", knobs.base_scalars(), Vec::new())
                .with_floors(knobs.floors().clone());
        crate::cascade::resolve(&pack, None, project).expect("resolves")
    }

    /// The WordPress proof (M40): a brand-name compound identifier in a component
    /// title over a differently-named symbol raises `doc-code.title-names-symbol`
    /// at its **advisory** default, and the report built over the *shipped* knob
    /// surface (no cascade delta) carries no blocking finding — the task
    /// gate/finalize passes. Under the old hardcoded blocking there was no valid
    /// remedy (renaming the heading would be wrong), which wedged the agent
    /// (`validation.md` → doc-code.title-names-symbol, the M40 demotion block).
    #[test]
    fn brand_name_title_is_advisory_and_passes_the_gate_over_the_shipped_knobs() {
        let findings = wordpress_guard_findings("wordpress-advisory");
        assert_eq!(findings.len(), 1, "one title finding: {findings:?}");
        assert_eq!(findings[0].code, "doc-code.title-names-symbol");

        let resolved = shipped_cascade(None);
        let report = crate::result::ValidationReport::new(findings, &resolved);
        assert_eq!(report.findings[0].severity, Severity::Advisory);
        assert!(
            !report.has_blocking(),
            "an advisory-only title finding does not flip the gate — the \
             WordPress-titled component finalizes",
        );
    }

    /// A strict project re-promotes with one cascade line: a project-layer
    /// `scalar-set validation.doc-code.title-names-symbol.severity blocking`
    /// re-grades the emitted advisory to **blocking** through the severity
    /// post-pass — the check is a keyed `CHECK_INVENTORY` row over a declared,
    /// tunable (unfloored) knob, not an exempt un-keyed code
    /// (`validation.md` → doc-code.title-names-symbol: "a strict project
    /// re-promotes it with one cascade line").
    #[test]
    fn title_names_symbol_repromotes_to_blocking_via_a_project_scalar_set() {
        let findings = wordpress_guard_findings("wordpress-repromote");
        assert_eq!(findings.len(), 1, "one title finding: {findings:?}");

        let project = crate::cascade::OverrideLayer::empty().scalar_set(
            "validation.doc-code.title-names-symbol.severity",
            "blocking",
        );
        let resolved = shipped_cascade(Some(&project));
        let report = crate::result::ValidationReport::new(findings, &resolved);
        assert_eq!(
            report.findings[0].severity,
            Severity::Blocking,
            "the project scalar-set re-grades the finding through assign_severity",
        );
        assert!(report.has_blocking(), "the re-promoted finding gates again");
    }

    #[test]
    fn is_compound_identifier_flags_symbols_not_prose() {
        assert!(is_compound_identifier("CommentBlockParser"));
        assert!(is_compound_identifier("myVar"));
        assert!(is_compound_identifier("WebSocket"));
        // Acronym-prefixed identifiers (no lower→upper hump) still count — the gap the
        // Opus re-test surfaced when `UIDoc` slipped through the camel-hump-only rule.
        assert!(is_compound_identifier("UIDoc"));
        assert!(is_compound_identifier("HTTPServer"));
        assert!(is_compound_identifier("IOError"));
        assert!(!is_compound_identifier("Session")); // single Capitalized word
        assert!(!is_compound_identifier("store")); // all lower
        assert!(!is_compound_identifier("HTTP")); // bare acronym, no trailing word
        assert!(!is_compound_identifier("API")); // bare acronym
        assert!(!is_compound_identifier("present_symbol")); // snake_case, no hump
    }

    /// Recursively collect every file path under `root` (relative to it), sorted —
    /// the before/after snapshot that proves the store walk wrote nothing.
    fn file_set(root: &Path) -> Vec<PathBuf> {
        let mut out = Vec::new();
        fn walk(dir: &Path, base: &Path, out: &mut Vec<PathBuf>) {
            let Ok(read) = std::fs::read_dir(dir) else {
                return;
            };
            for entry in read.flatten() {
                let path = entry.path();
                if path.is_dir() {
                    walk(&path, base, out);
                } else {
                    out.push(path.strip_prefix(base).unwrap().to_path_buf());
                }
            }
        }
        walk(root, root, &mut out);
        out.sort();
        out
    }

    /// (M18, the net-new read-only committed-store walk — `validation.md` →
    /// Store-scope re-validation.) [`enumerate_committed_surface`] sweeps every
    /// active schema `location:` over the committed store and collects its
    /// `code-anchor` leaves, with **no** working area and **no** task scope. Three
    /// things it must do that the task-scope walk does not:
    ///
    /// - **Surface unrelated committed docs.** The task gate deliberately *excludes*
    ///   a committed doc that is neither edited nor bound (hardening #5). Store scope
    ///   **inverts** that: surfacing pre-existing drift across the whole store is the
    ///   command's entire purpose, so a committed ADR no task ever touched IS in the
    ///   set.
    /// - **Fire the multi-valued guard at store scope.** A committed doc whose
    ///   `code-anchor` parses to a [`Value::List`] yields no anchor and one loud
    ///   guard finding (the store-scope half of the non-silent-guard contract).
    /// - **Mutate nothing.** It must not route through `reconcile_committed_store`
    ///   (which absorbs drift into the file-state record + edge index), so a
    ///   before/after file-set snapshot under the repo root is byte-identical — no
    ///   `file-state.json` / `edges.json` appears.
    ///
    /// The seeded store carries a **valid** anchor, a **dangling** anchor (a
    /// well-formed `<path>#<symbol>` whose target need not exist — enumeration does
    /// not resolve, the probe does), a **list-valued** anchor, and an **unrelated**
    /// committed doc. The returned set is exactly the scalar anchors, address-sorted.
    #[test]
    fn enumerate_committed_surface_walks_store_surfaces_unrelated_fires_guard_mutates_nothing() {
        let repo = TempRoot::new("committed-surface");

        // A committed ADR with a valid scalar anchor.
        commit(
            repo.path(),
            "decisions",
            "single-node-cache",
            ADR_WITH_ANCHOR,
        );
        // A committed ADR with a *dangling* anchor (well-formed, target absent —
        // enumeration is shape-only; the probe resolves it later).
        const ADR_DANGLING: &str = "\
---
status: accepted
date: 2026-05-23
cites-code: crates/engine/src/gone.rs#vanished_symbol
---

# Dangling decision

## Context
Forces.

## Options
Alternatives were weighed and rejected.

## Decision
Decided.

## Consequences
Effects.
";
        commit(repo.path(), "decisions", "dangling", ADR_DANGLING);
        // A committed ADR whose anchor is list-valued — fires the guard, no anchor.
        commit(
            repo.path(),
            "decisions",
            "list-anchor",
            ADR_WITH_LIST_ANCHOR,
        );
        // A committed SPEC with a criterion anchor — a *different* location/doctype,
        // proving the per-location glob covers every active schema.
        commit(repo.path(), "specs", "rate-limiting", SPEC_WITH_ANCHOR);

        let before = file_set(repo.path());

        let (anchors, guard_findings) =
            enumerate_committed_surface(repo.path(), &schemas()).expect("enumerates store");

        // Exactly the three scalar anchors (valid + dangling + the bound-spec item),
        // address-sorted. The list-valued ADR contributes no anchor.
        assert_eq!(
            anchors,
            vec![
                TargetAnchor {
                    address: "adr:dangling#status/cites-code".to_string(),
                    anchor_value: "crates/engine/src/gone.rs#vanished_symbol".to_string(),
                    check_id: "symbol-exists".to_string(),
                },
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
            "store walk collects every committed doc's scalar code-anchor, \
             address-sorted, across all active locations",
        );

        // The store-scope inversion: an unrelated committed ADR (no task touched it)
        // IS present — store scope surfaces exactly the drift the task gate excludes.
        assert!(
            anchors
                .iter()
                .any(|a| a.address == "adr:single-node-cache#status/cites-code"),
            "an unrelated committed doc must be surfaced at store scope (the \
             task-gate inversion), got {anchors:?}",
        );

        // The list-valued committed anchor fired the guard at store scope.
        assert_eq!(
            guard_findings.len(),
            1,
            "the committed list-valued code-anchor emits one guard finding at store \
             scope (not a silent drop): {guard_findings:?}",
        );
        let f = &guard_findings[0];
        assert_eq!(f.severity, crate::finding::Severity::Blocking);
        assert_eq!(f.code, "doc-code.multi-valued-anchor");
        assert_eq!(
            f.location.as_ref().and_then(|l| l.address.as_deref()),
            Some("adr:list-anchor#status/cites-code"),
            "the guard finding points at the offending committed field's address",
        );

        // Mutated nothing: no file-state.json, no edges.json, no record write — the
        // file set under the repo root is byte-identical before and after.
        let after = file_set(repo.path());
        assert_eq!(
            before, after,
            "the read-only store walk must write no FileStateRecord / edge-index \
             file (it must NOT route through reconcile_committed_store)",
        );
    }

    /// (M42, the placement census — `storage.md` → The census, row
    /// `target_surface::enumerate_committed_surface`.) The store walk keyed on
    /// `schema.location` alone, so a **placement** doctype (`location: None`, homed at a
    /// literal repo-root `placement.file` — `VISION.md`, `CHANGELOG.md`, `docs/roadmap.md`)
    /// dropped through the *transient* arm and contributed **zero** anchors: the store-scope
    /// `doc-code` family would report a clean store over anchors it never read. The hole was
    /// **latent** (no shipped placement doctype declares a `code-anchor`) but it opened on a
    /// bare schema edit, so the proof is a **synthetic** placement schema that declares one.
    ///
    /// Its committed literal file present, the walk must yield its `code-anchor` leaves
    /// addressed at the fixed `<type>:<type>` singleton identity (never a slug re-derived
    /// from the case-preserved literal path), and the **multi-valued non-silent guard** must
    /// fire over the placement doc exactly as it does over a located one — the projection is
    /// shared, so the class cannot be enumerated-but-unguarded.
    #[test]
    fn enumerate_committed_surface_reaches_a_placement_doctypes_literal_home() {
        // A synthetic placement doctype (no shipped one declares a `code-anchor`) whose
        // header carries both a scalar anchor and a list-valued one.
        const PINBOARD_SCHEMA: &[u8] = b"\
type: pinboard
placement: { file: PINBOARD.md }
id-from: title
sections:
  - id: status
    header: true
    fields:
      - { id: cites-code, type: code-anchor }
      - { id: also-cites, type: code-anchor }
  - id: notes
    slot: { hint: Standing notes. }
";
        const PINBOARD_INSTANCE: &str = "\
---
cites-code: crates/engine/src/index.rs#committed_instances
also-cites: [crates/engine/src/a.rs#one, crates/engine/src/b.rs#two]
---

# Pinboard

## Notes
Standing notes.
";

        let repo = TempRoot::new("placement-surface");
        // The committed instance lives at its literal repo-root file — never under a
        // `<location>/` dir (a placement doctype has none).
        std::fs::write(repo.path().join("PINBOARD.md"), PINBOARD_INSTANCE).expect("commit");

        let mut schemas = BTreeMap::new();
        schemas.insert(
            "pinboard".to_string(),
            load_schema_with_types(PINBOARD_SCHEMA, &dev_pack_field_types())
                .expect("placement fixture loads"),
        );

        let before = file_set(repo.path());
        let (anchors, guard_findings) =
            enumerate_committed_surface(repo.path(), &schemas).expect("enumerates store");

        // The scalar anchor, at the fixed `<type>:<type>` singleton identity.
        assert_eq!(
            anchors,
            vec![TargetAnchor {
                address: "pinboard:pinboard#status/cites-code".to_string(),
                anchor_value: "crates/engine/src/index.rs#committed_instances".to_string(),
                check_id: "symbol-exists".to_string(),
            }],
            "a placement doctype's committed literal home is walked, and its anchor is \
             addressed by the fixed `<type>:<type>` singleton slug, got {anchors:?}",
        );

        // The multi-valued guard fires over the placement doc too — the class is never
        // enumerated-but-unguarded (a silent drop would read as a false store-wide all-clear).
        assert_eq!(
            guard_findings.len(),
            1,
            "the list-valued anchor in the placement doc fires the guard: {guard_findings:?}",
        );
        let f = &guard_findings[0];
        assert_eq!(f.severity, Severity::Blocking);
        assert_eq!(f.code, "doc-code.multi-valued-anchor");
        assert_eq!(
            f.location.as_ref().and_then(|l| l.address.as_deref()),
            Some("pinboard:pinboard#status/also-cites"),
            "the guard names the offending placement doc's field address",
        );

        // Still a pure read: no record, no index, no file written.
        assert_eq!(
            before,
            file_set(repo.path()),
            "the store walk writes nothing"
        );
    }

    /// An **absent** placement singleton contributes nothing and is not an error — the
    /// literal-home walk lists only what exists, exactly as the located `<location>/*.md`
    /// glob does (a repo that has not authored its `VISION.md` yet is not a broken store).
    #[test]
    fn enumerate_committed_surface_over_an_absent_placement_singleton_is_empty_not_an_error() {
        const PINBOARD_SCHEMA: &[u8] = b"\
type: pinboard
placement: { file: PINBOARD.md }
id-from: title
sections:
  - id: status
    header: true
    fields:
      - { id: cites-code, type: code-anchor }
";
        let repo = TempRoot::new("placement-absent");
        let mut schemas = BTreeMap::new();
        schemas.insert(
            "pinboard".to_string(),
            load_schema_with_types(PINBOARD_SCHEMA, &dev_pack_field_types())
                .expect("placement fixture loads"),
        );

        let (anchors, guard_findings) = enumerate_committed_surface(repo.path(), &schemas)
            .expect("an absent home is not an error");
        assert!(anchors.is_empty(), "no committed singleton, no anchors");
        assert!(
            guard_findings.is_empty(),
            "and no findings: {guard_findings:?}"
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
