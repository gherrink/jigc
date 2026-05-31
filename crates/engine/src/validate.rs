//! The validation engine — read-only probes over a scope, severity via cascade,
//! the `finalize` gate.
//!
//! See `design/validation.md`. MVP engine-native probes: `workflow-refs`,
//! `file-state`, plus the synthetic `schema-conformance` integrity checks.
//!
//! ## `schema-conformance` (synthetic category)
//!
//! `schema-conformance` is a **synthetic** validate category (`validation.md` →
//! Synthetic categories): not a literal `check(target, ctx)` probe, but a namespace
//! for schema-driven integrity checks the engine runs over an already-parsed
//! instance. Inc-4 ships three of the four (`DECISIONS.md` 2026-05-31 → inc-4
//! planning pins → schema-conformance scope); **`ref-resolves` (forward-ref / edge
//! index) is explicitly inc-5** and is *not* run here.
//!
//! - **`required-slot-present`** — every declared body slot must hold non-empty
//!   prose. The parser already requires the section *heading* (a missing heading is
//!   a `conformance.section-missing` parse error), so this check fires on the
//!   present-but-empty slot: the LLM left it unfilled. (`finalize.md` → phase 2:
//!   "validate blocks on required-slot presence".)
//! - **`required-field-present`** — every **author-required** declared field must be
//!   present in the instance. A field is author-required iff it carries neither a
//!   `default:` nor a `set:` — those are CLI-derived / defaulted, never the author's
//!   obligation. (`document-type-schema.md` → Field provenance.)
//! - **`field-value-conformant`** — every present field's value must pass its
//!   declared type, reusing [`crate::write::check_value`] (the same write-time
//!   adjudicator). A malformed date / non-member enum / empty string blocks.
//!
//! Each violation is one intrinsic **blocking** `schema-conformance.*` [`Finding`]
//! (the inventory default severity; cascade tuning is not wired into engine probes
//! yet, paralleling `workflow-refs` / `file-state`). A fully-conformant instance
//! yields none.

use crate::field_block::Field;
use crate::file_state::{FileStateRecord, file_state};
use crate::finding::{Finding, Location, Severity};
use crate::parse::{Document, ParsedSection, parse_sections};
use crate::result::ValidationReport;
use crate::schema::{Field as SchemaField, FieldType, Schema, Section, SectionBody};
use std::collections::BTreeMap;
use std::path::Path;

/// The working-area sub-directory holding the task's staged doc instances
/// (`DECISIONS.md` 2026-05-31 → Task working-area on-disk layout: a staged instance
/// lives at `.jigc/tasks/<id>/docs/<type>:<slug>.md`).
const DOCS_DIR: &str = "docs";

/// Validate one task working area — the single engine both `task validate` and
/// `finalize` phase 2 call (`validation.md` → How it gates `finalize`: one engine,
/// two entry points, so what `validate` reports and what `finalize` blocks on can
/// never diverge). No git, no commit.
///
/// Resolves the working area's staged doc instances (`<dir>/docs/*.md`, each named
/// `<type>:<slug>.md`), then over each instance runs the two MVP task-scope probes
/// and aggregates their severity-classified findings into a [`ValidationReport`]:
///
/// - **`file-state`** — hashes the staged bytes against `record` (baseline-adopt on
///   first encounter, blocking drift on a recorded mismatch). The probe advances the
///   record on adopt; the caller persists it.
/// - **`schema-conformance`** — parses each instance against its (caller-supplied)
///   schema. A *parse-level* conformance failure (missing/renamed heading, malformed
///   anchor, …) surfaces those findings directly; a clean parse then runs the
///   [`schema_conformance`] checks (required slot/field present, field-value
///   conformant) over the instance.
///
/// `schemas` maps a doc-type name to its resolved [`Schema`] — the engine stays
/// **domain-empty** (`CLAUDE.md` → engine ships empty of domain content): the caller
/// (CLI) resolves the cascade and feeds the schemas in. A staged file whose type
/// prefix has no schema in `schemas` raises a blocking `schema-conformance.unknown-type`
/// finding (the working area references a type the resolved cascade does not define).
///
/// Findings aggregate in a stable sweep order: docs by path-sorted filename, and
/// within each doc `file-state` before `schema-conformance`. `ref-resolves`
/// (forward-ref / edge index) is **not** run — it is inc-5 (no edge index yet).
pub fn validate_task(
    dir: &Path,
    schemas: &BTreeMap<String, Schema>,
    record: &mut FileStateRecord,
) -> std::io::Result<ValidationReport> {
    let mut findings = Vec::new();
    for entry in staged_instances(dir)? {
        let StagedInstance { rel_key, filename } = entry;
        let bytes = std::fs::read(dir.join(DOCS_DIR).join(&filename))?;

        // `file-state` over this one instance, keyed by its docs-relative path.
        findings.extend(file_state(record, &[(rel_key.as_str(), &bytes)]));

        // `schema-conformance` over the instance, resolving its type from the
        // `<type>:<slug>.md` filename. A non-UTF-8 instance can't be a managed
        // Markdown doc; the parser owns that, so we require a UTF-8 read here.
        let source = String::from_utf8_lossy(&bytes);
        findings.extend(conformance_for(&filename, schemas, &rel_key, &source));
    }
    Ok(ValidationReport::new(findings))
}

/// One staged doc instance under `<dir>/docs/`: its `docs/<filename>` record key
/// and the bare `filename` (`<type>:<slug>.md`).
struct StagedInstance {
    rel_key: String,
    filename: String,
}

/// The task's staged doc instances under `<dir>/docs/`, in **path-sorted** filename
/// order (the stable sweep order). A working area with no `docs/` dir (nothing
/// staged yet) yields an empty list, not an error. Only `*.md` files are instances.
fn staged_instances(dir: &Path) -> std::io::Result<Vec<StagedInstance>> {
    let docs = dir.join(DOCS_DIR);
    let read = match std::fs::read_dir(&docs) {
        Ok(read) => read,
        Err(err) if err.kind() == std::io::ErrorKind::NotFound => return Ok(Vec::new()),
        Err(err) => return Err(err),
    };
    let mut names: Vec<String> = Vec::new();
    for entry in read {
        let entry = entry?;
        if !entry.file_type()?.is_file() {
            continue;
        }
        let name = entry.file_name().to_string_lossy().into_owned();
        if name.ends_with(".md") {
            names.push(name);
        }
    }
    names.sort();
    Ok(names
        .into_iter()
        .map(|filename| StagedInstance {
            rel_key: format!("{DOCS_DIR}/{filename}"),
            filename,
        })
        .collect())
}

/// Run `schema-conformance` over one staged instance: resolve its type from the
/// `<type>:<slug>.md` filename, parse against the schema, and surface parse-level
/// findings or the [`schema_conformance`] checks. A type with no schema in the
/// resolved cascade raises a blocking `schema-conformance.unknown-type`.
fn conformance_for(
    filename: &str,
    schemas: &BTreeMap<String, Schema>,
    rel_key: &str,
    source: &str,
) -> Vec<Finding> {
    let ty = filename.split(':').next().unwrap_or(filename);
    let Some(schema) = schemas.get(ty) else {
        return vec![Finding {
            severity: Severity::Blocking,
            code: "schema-conformance.unknown-type".to_string(),
            message: format!(
                "staged doc `{rel_key}` has type `{ty}`, which the resolved cascade does not define"
            ),
            location: Some(Location::addressed(rel_key, 1, 1)),
            route: None,
        }];
    };
    match parse_sections(schema, source) {
        Ok(doc) => schema_conformance(schema, source, &doc),
        Err(parse_findings) => parse_findings,
    }
}

/// Run the synthetic `schema-conformance` checks over a parsed instance: every
/// declared body slot is non-empty (`required-slot-present`), every author-required
/// field is present (`required-field-present`), and every present field's value is
/// type-conformant (`field-value-conformant`). Returns one blocking [`Finding`] per
/// violation, in section-document order; a conformant instance yields an empty `Vec`.
///
/// `ref-resolves` (forward-ref / edge-index integrity) is **not** run here — it is
/// inc-5 (no edge index exists yet). `source` is needed to slice the opaque slot
/// spans the parser recorded.
pub fn schema_conformance(schema: &Schema, source: &str, doc: &Document) -> Vec<Finding> {
    let mut findings = Vec::new();
    for section in &schema.sections {
        let Some(parsed) = doc.sections.iter().find(|s| s.id == section.id) else {
            // A section the parser did not map (header sections carry no `ParsedSection`
            // slot, and a repeatable section is out of MVP conformance scope) — nothing
            // slot/field-shaped to adjudicate here.
            continue;
        };
        let SectionBody::Simple {
            slot: declared_slot,
            fields: declared_fields,
        } = &section.body
        else {
            // Repeatable sections are not adjudicated by these MVP checks.
            continue;
        };

        // required-slot-present: a declared body slot must hold non-empty prose.
        if declared_slot.is_some() {
            check_slot_present(section, parsed, source, &mut findings);
        }
        // required-field-present + field-value-conformant over the declared fields.
        for declared in declared_fields {
            check_field(section, declared, parsed, &mut findings);
        }
    }
    findings
}

/// `required-slot-present`: the section declares a slot, so its prose must be
/// non-empty. The parser records the slot span even when empty (a present heading
/// with no prose under it), so an all-whitespace slice is the unfilled-slot case.
fn check_slot_present(
    section: &Section,
    parsed: &ParsedSection,
    source: &str,
    findings: &mut Vec<Finding>,
) {
    let filled = parsed
        .slot
        .as_ref()
        .map(|span| !span.slice(source).trim().is_empty())
        .unwrap_or(false);
    if !filled {
        findings.push(blocking_conformance(
            "schema-conformance.required-slot-present",
            format!("required slot in section `{}` is empty", section.id),
            slot_location(parsed),
        ));
    }
}

/// `required-field-present` + `field-value-conformant` for one declared field.
///
/// A field is **author-required** iff it carries neither a `default:` nor a `set:`
/// (those are CLI-derived / defaulted, never the author's obligation). An absent
/// author-required field blocks; a present field whose value fails its declared type
/// (via [`crate::write::check_value`]) blocks. A present field is *not* also reported
/// absent, and an absent field is *not* value-checked.
fn check_field(
    section: &Section,
    declared: &SchemaField,
    parsed: &ParsedSection,
    findings: &mut Vec<Finding>,
) {
    match parsed.fields.iter().find(|f| f.key == declared.id) {
        Some(present) => check_field_value(section, declared, present, findings),
        None => {
            if is_author_required(declared) {
                findings.push(blocking_conformance(
                    "schema-conformance.required-field-present",
                    format!(
                        "required field `{}` is missing from section `{}`",
                        declared.id, section.id
                    ),
                    None,
                ));
            }
        }
    }
}

/// `field-value-conformant`: a present field's value must pass its declared type.
fn check_field_value(
    section: &Section,
    declared: &SchemaField,
    present: &Field,
    findings: &mut Vec<Finding>,
) {
    if let Err(why) = crate::write::check_value(declared, &present.value) {
        findings.push(blocking_conformance(
            "schema-conformance.field-value-conformant",
            format!("field `{}` in section `{}`: {why}", declared.id, section.id),
            None,
        ));
    }
}

/// A field is author-required iff the author must supply it — no `default:`, no
/// `set:` (CLI-derived), and not an **optional `ref`** (forward `card:` with a minimum
/// of 0, e.g. the ADR `supersedes` relation's `0..1`). An optional ref carries no
/// author obligation: its presence is the agent's choice and its *resolution*, not its
/// presence, is what `ref-resolves` adjudicates at finalize (`design/validation.md` →
/// the synthetic `ref-resolves` check; `document-type-schema.md` → `ref` cardinality).
fn is_author_required(field: &SchemaField) -> bool {
    if field.ty == FieldType::Ref && ref_min_cardinality_zero(field) {
        return false;
    }
    field.default.is_none() && field.set.is_none()
}

/// Whether a `ref` field's forward cardinality has a minimum of 0 (it is optional).
/// `card:` defaults to `"0..1"` when omitted (`schema.rs` → `SchemaField::card`); an
/// explicit form is optional iff it starts with `"0"` (`"0..1"` / `"0..*"`).
fn ref_min_cardinality_zero(field: &SchemaField) -> bool {
    match field.card.as_deref() {
        None => true,
        Some(card) => card.trim_start().starts_with('0'),
    }
}

/// The located coordinate for a slot finding: the slot span's start line when known.
fn slot_location(parsed: &ParsedSection) -> Option<Location> {
    parsed
        .slot
        .as_ref()
        .map(|span| Location::at(span.start_line, 1))
}

/// Build a blocking `schema-conformance.*` [`Finding`] (the inventory default
/// severity). These intrinsic checks carry no `route` — the agent fills the slot /
/// field directly.
fn blocking_conformance(code: &str, message: String, location: Option<Location>) -> Finding {
    Finding {
        severity: Severity::Blocking,
        code: code.to_string(),
        message,
        location,
        route: None,
    }
}

#[cfg(test)]
mod schema_conformance_tests {
    //! `schema-conformance.*` over a single parsed instance: required-slot-present,
    //! required-field-present, field-value-conformant. One blocking finding per
    //! violation; a conformant instance yields none.

    use super::*;
    use crate::parse::parse_sections;
    use crate::schema::load_schema;

    /// A purpose-built schema with the three things the checks adjudicate: a header
    /// with one author-required enum field (`kind`, no default/set) plus one
    /// CLI-defaulted enum field (`status`, with `default`), and one body slot section
    /// (`body`). Minimal — one of each lever, so a fixture can violate exactly one.
    fn schema() -> Schema {
        let yaml = b"\
type: note
id-from: title
sections:
  - id: meta
    header: true
    fields:
      - { id: title, type: string }
      - { id: kind, type: enum, of: [memo, brief] }
      - { id: status, type: enum, of: [open, closed], default: open }
  - id: body
    slot: { hint: \"The note body.\" }
";
        load_schema(yaml).expect("note schema loads")
    }

    /// Parse a fixture against the schema, panicking (with the parse findings) if it
    /// does not cleanly map — the conformance checks run *over a parsed instance*, so
    /// the fixture must parse first.
    fn parse(source: &str) -> Document {
        parse_sections(&schema(), source)
            .unwrap_or_else(|f| panic!("fixture must parse; got conformance findings: {f:?}"))
    }

    /// A fully-conformant instance: the slot is filled, both author-required fields
    /// present, every value type-conformant. Yields **zero** findings.
    const CONFORMANT: &str = "\
---
title: A note
kind: memo
status: open
---

# A note

## Body

The note body prose.
";

    /// Missing required slot: the `## Body` heading is present (so it parses) but the
    /// slot prose is empty. Exactly one `schema-conformance.required-slot-present`.
    const MISSING_SLOT: &str = "\
---
title: A note
kind: memo
status: open
---

# A note

## Body
";

    /// Missing required field: the author-required `kind` enum (no default) is absent
    /// from the front-matter. Exactly one `schema-conformance.required-field-present`.
    /// (`status` is absent too but is CLI-defaulted, so it is *not* author-required.)
    const MISSING_FIELD: &str = "\
---
title: A note
---

# A note

## Body

The note body prose.
";

    /// Malformed field value: `kind` is present but not an enum member. Exactly one
    /// `schema-conformance.field-value-conformant`.
    const MALFORMED_VALUE: &str = "\
---
title: A note
kind: wormhole
status: open
---

# A note

## Body

The note body prose.
";

    /// The clean fixture yields no findings; each minimal negative fixture yields
    /// exactly one matching blocking `schema-conformance.{…}` finding. The table is
    /// the done-criterion: one violation each, asserted by `code` + `severity`.
    #[test]
    fn schema_conformance_flags_missing_and_malformed() {
        let schema = schema();

        // The conformant instance: zero findings.
        let doc = parse(CONFORMANT);
        let findings = schema_conformance(&schema, CONFORMANT, &doc);
        assert!(
            findings.is_empty(),
            "a fully-conformant instance must yield no findings, got {findings:?}"
        );

        // Each negative fixture: exactly one finding with the expected code + severity.
        let cases: &[(&str, &str)] = &[
            (MISSING_SLOT, "schema-conformance.required-slot-present"),
            (MISSING_FIELD, "schema-conformance.required-field-present"),
            (MALFORMED_VALUE, "schema-conformance.field-value-conformant"),
        ];
        for (source, expected_code) in cases {
            let doc = parse(source);
            let findings = schema_conformance(&schema, source, &doc);
            assert_eq!(
                findings.len(),
                1,
                "fixture for {expected_code} must yield exactly one finding, got {findings:?}"
            );
            let finding = &findings[0];
            assert_eq!(finding.code, *expected_code, "finding code");
            assert_eq!(
                finding.severity,
                Severity::Blocking,
                "{expected_code} must be blocking"
            );
        }
    }
}

#[cfg(test)]
mod validate_task_tests {
    //! The task-scope sweep: `validate_task` resolves a working area's staged doc
    //! instances, runs `file-state` + `schema-conformance` over them, and aggregates
    //! the severity-classified findings into one report exposing `has_blocking()`.

    use super::*;
    use crate::file_state::hash_bytes;
    use crate::schema::load_schema;
    use std::path::PathBuf;

    /// A throwaway working-area root that removes itself on drop.
    struct TempArea(PathBuf);

    impl TempArea {
        fn new(tag: &str) -> Self {
            let mut path = std::env::temp_dir();
            path.push(format!(
                "jigc-validate-task-{tag}-{}-{:?}",
                std::process::id(),
                std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap()
                    .as_nanos(),
            ));
            std::fs::create_dir_all(path.join(DOCS_DIR)).expect("create docs dir");
            TempArea(path)
        }

        fn dir(&self) -> &Path {
            &self.0
        }

        /// Stage a doc instance at `docs/<filename>` and return its docs-relative
        /// path key (the form the `file-state` record is keyed by).
        fn stage(&self, filename: &str, bytes: &[u8]) -> String {
            let rel = format!("{DOCS_DIR}/{filename}");
            std::fs::write(self.0.join(&rel), bytes).expect("stage instance");
            rel
        }
    }

    impl Drop for TempArea {
        fn drop(&mut self) {
            let _ = std::fs::remove_dir_all(&self.0);
        }
    }

    /// The `note` schema used by inc-4's conformance fixtures: one author-required
    /// enum field plus one body slot — enough to make an instance conformance-broken
    /// by leaving the slot empty.
    fn note_schema() -> Schema {
        let yaml = b"\
type: note
id-from: title
sections:
  - id: meta
    header: true
    fields:
      - { id: title, type: string }
      - { id: kind, type: enum, of: [memo, brief] }
  - id: body
    slot: { hint: \"The note body.\" }
";
        load_schema(yaml).expect("note schema loads")
    }

    /// A fully-conformant `note` instance: slot filled, required field present.
    const CONFORMANT: &str = "\
---
title: A note
kind: memo
---

# A note

## Body

The note body prose.
";

    /// A conformance-broken `note` instance: the `## Body` heading parses but the
    /// slot is empty — one `schema-conformance.required-slot-present`.
    const BROKEN: &str = "\
---
title: A note
kind: memo
---

# A note

## Body
";

    fn schemas() -> BTreeMap<String, Schema> {
        let mut m = BTreeMap::new();
        m.insert("note".to_string(), note_schema());
        m
    }

    /// The done-criterion. Over a working area with **one drifted file** and **one
    /// conformance-broken instance**, `validate_task` returns *both* findings and
    /// `has_blocking() == true`; over a **clean** area it returns an empty report and
    /// `has_blocking() == false`.
    #[test]
    fn validate_task_aggregates_probe_findings() {
        // --- The broken area: a drifted instance + a conformance-broken instance.
        let area = TempArea::new("broken");

        // `note:drift.md` was committed with the conformant bytes (its hash is in the
        // record), but the working area now holds *different* bytes → file-state drift.
        let drift_rel = area.stage("note:drift.md", BROKEN.as_bytes());
        let mut record = FileStateRecord::new();
        record.record(drift_rel.clone(), hash_bytes(CONFORMANT.as_bytes()));

        // `note:broken.md` is a fresh instance (no recorded hash → baseline-adopt,
        // advisory) whose slot is empty → schema-conformance blocks.
        area.stage("note:broken.md", BROKEN.as_bytes());

        let report = validate_task(area.dir(), &schemas(), &mut record).expect("sweep runs");

        // Both blocking findings are present in the aggregate.
        let codes: Vec<&str> = report.findings.iter().map(|f| f.code.as_str()).collect();
        assert!(
            codes.contains(&"file-state.hash-matches"),
            "the drifted file must surface a file-state drift finding, got {codes:?}"
        );
        assert!(
            codes.contains(&"schema-conformance.required-slot-present"),
            "the broken instance must surface a conformance finding, got {codes:?}"
        );
        assert!(
            report.has_blocking(),
            "a drifted + conformance-broken area must block, got {:?}",
            report.findings
        );

        // --- The clean area: a single conformant instance, freshly baselined.
        let clean = TempArea::new("clean");
        clean.stage("note:ok.md", CONFORMANT.as_bytes());
        let mut clean_record = FileStateRecord::new();

        let clean_report =
            validate_task(clean.dir(), &schemas(), &mut clean_record).expect("clean sweep runs");

        // A fresh file baselines (advisory, non-blocking) and conforms → no blocker.
        assert!(
            !clean_report.has_blocking(),
            "a clean area must not block, got {:?}",
            clean_report.findings
        );
        assert!(
            clean_report
                .findings
                .iter()
                .all(|f| f.severity != Severity::Blocking),
            "no blocking findings over a clean area, got {:?}",
            clean_report.findings
        );
    }
}
