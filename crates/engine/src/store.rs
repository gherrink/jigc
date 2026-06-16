//! The committed-store reader — resolve a `<type>:<slug>` address to its canonical
//! path, parse the committed `.md` against its schema, and slice a `#fragment` to
//! its content (a section slot's prose, or a repeatable section's rendered items).
//!
//! This is the byte-read the compose-path [`Resolution::Content`](crate::data_value::Resolution::Content)
//! handle deferred and the superseding-decision flow mandates
//! ([worked-examples.md](../../../design/worked-examples.md) → Superseding decision):
//! reaching `{{@task.decision.supersedes#decision}}` re-reads the committed
//! `decisions/single-node-cache.md` and parses it losslessly to extract the section.
//!
//! ## Identity is the path ([storage.md](../../../design/storage.md) → Identity is the path)
//!
//! A managed doc's identity *is* its path: `decisions/single-node-cache.md` → type
//! `adr`, id `single-node-cache`. So resolving `adr:single-node-cache` to a file is
//! purely `<repo-root>/<schema.location>/<slug>.md` — no redundant id is read from
//! the file. A type with **no `location:`** (a transient sink type like `commit`,
//! whose sink is the git message) has no committed path and is not store-readable.
//!
//! ## The round-trip on a committed file ([parsing.md](../../../implementation/parsing.md) → Round-trip guarantees)
//!
//! The slice is taken over the committed bytes via [`crate::parse::parse_sections`]
//! and the recorded opaque [`Span`](crate::parse::Span) — the same byte-stable read
//! path the writer is the inverse of. Slicing re-reads the source over the span, so
//! the returned prose is byte-for-byte the committed file's slot bytes. A
//! missing/unparseable target is a blocking [`Finding`] carrying a route — never a
//! panic, never a silent empty (the settled block-payload shape, `DECISIONS.md`
//! 2026-05-31).

use std::collections::BTreeMap;
use std::path::{Component, Path, PathBuf};

use crate::address::{Address, Fragment};
use crate::finding::{Finding, Location, Severity};
use crate::parse::{self, Document, ParsedSection};
use crate::schema::Schema;

/// The canonical on-disk path a committed `<type>:<slug>` instance lives at, when
/// its schema declares a persisted `location:`.
///
/// `<repo_root>/<location>/<slug>.md` — identity is the path ([storage.md](../../../design/storage.md)).
/// A type with **no `location:`** (a transient sink type) has no committed path, so
/// this returns `None`.
pub fn canonical_path(repo_root: &Path, schema: &Schema, slug: &str) -> Option<PathBuf> {
    let location = schema.location.as_deref()?;
    Some(repo_root.join(location).join(format!("{slug}.md")))
}

/// Lexically normalize a path — drop `.` components and resolve `..` against the
/// accumulated prefix — **without touching the filesystem**.
///
/// The path-collision guards (`design/auto-migration.md` → Path-collision guard)
/// compare a recorded foreign source path against a canonical managed path; this
/// pass makes those comparisons spelling-insensitive (review F2/C1) so a path
/// recorded as `./changelog/changelog.md` (or with redundant `..` components) still
/// compares equal to the clean canonical `changelog/changelog.md`. Shared so the
/// retire-side (finalize) and create-side (state) guards normalize identically.
pub fn lexical_normalize(p: &Path) -> PathBuf {
    let mut out = PathBuf::new();
    for component in p.components() {
        match component {
            Component::CurDir => {}
            Component::ParentDir => {
                out.pop();
            }
            other => out.push(other.as_os_str()),
        }
    }
    out
}

/// Read the committed managed doc named by `address` and slice its `#fragment` to
/// the section slot's prose bytes.
///
/// Resolves the address's `type` to a [`Schema`] in `schemas`, computes the
/// canonical path under `repo_root`, reads and parses the committed file against the
/// schema, and slices the `#unit` fragment to that section's content — a slot's
/// opaque prose (byte-exact over the committed source), or a repeatable section's
/// rendered items. The address **must** carry a `#unit` fragment naming a slot
/// section (flow #5's `#decision`) or a repeatable section (the spec's `#criteria`).
///
/// Every failure is a blocking [`Finding`] carrying a `route`: an unknown type, a
/// transient (location-less) type, a missing file, an unparseable file, or a missing
/// or unsliceable fragment.
pub fn read_slice(
    repo_root: &Path,
    schemas: &BTreeMap<String, Schema>,
    address: &Address,
) -> Result<String, Finding> {
    let address_str = address.to_string();
    let type_name = address.r#type.as_str();
    let slug = address.slug.as_str();

    // Resolve the type to its schema.
    let Some(schema) = schemas.get(type_name) else {
        return Err(block(
            "store.unknown-type",
            format!("unknown doctype `{type_name}` for `{address_str}`"),
            &address_str,
            "list the available doctypes with `jigc doc types`".to_string(),
        ));
    };

    // A `#unit` fragment naming a slot section is required (the MVP store-read target).
    let Some(fragment) = &address.fragment else {
        return Err(block(
            "store.no-fragment",
            format!("`{address_str}` names no `#section` to slice"),
            &address_str,
            "add a `#section` fragment naming a slot section".to_string(),
        ));
    };

    // Identity is the path: `<repo_root>/<location>/<slug>.md`. A transient
    // (location-less) type has no committed path and is not store-readable.
    let Some(path) = canonical_path(repo_root, schema, slug) else {
        return Err(block(
            "store.transient-type",
            format!(
                "doctype `{type_name}` is transient (no `location:`); `{address_str}` is not committed"
            ),
            &address_str,
            "the referenced doctype has no committed location".to_string(),
        ));
    };

    // Read the committed bytes; a missing file is a located block, not a panic.
    let mut source = std::fs::read_to_string(&path).map_err(|err| {
        block(
            "store.not-found",
            format!(
                "could not read `{address_str}` at `{}`: {err}",
                path.display()
            ),
            &address_str,
            "create the referenced doc, or fix the reference to an existing one".to_string(),
        )
    })?;
    // Tolerate a leading BOM on read (Windows-editor edits) before parse + slice.
    parse::strip_leading_bom(&mut source);

    // Parse the committed file against its schema; conformance failures surface the
    // first blocking finding (re-located onto the address for the caller).
    let doc = parse::parse_sections(schema, &source).map_err(|findings| {
        let why = findings
            .first()
            .map(|f| f.message.clone())
            .unwrap_or_else(|| "unparseable".to_string());
        block(
            "store.unparseable",
            format!(
                "`{address_str}` at `{}` does not parse: {why}",
                path.display()
            ),
            &address_str,
            "fix the committed file so it conforms to its schema".to_string(),
        )
    })?;

    slice_fragment(&doc, &source, fragment, &address_str)
}

/// Build a blocking store-read [`Finding`] with a located message and a route.
fn block(code: &str, message: String, address: &str, route: String) -> Finding {
    Finding::graded(
        Severity::Blocking,
        code,
        message,
        Some(Location::addressed(address, 1, 1)),
        Some(route),
    )
}

/// Slice the `#unit` fragment of a parsed `doc` to its section content over
/// `source`, or a blocking finding when the fragment names no sliceable section.
///
/// Two section shapes slice. A **slot** section (flow #5's `#decision`) returns its
/// slot prose byte-for-byte over the recorded opaque [`Span`](crate::parse::Span). A
/// **repeatable** section (no slot — e.g. the spec's `#criteria`) returns its items
/// rendered as a Content list: each item's `### <title>` followed by its statement
/// slot prose (see [`render_repeatable_items`]), so `{{@task.spec#criteria}}` resolves
/// to the criteria the implementer reads (`worked-examples.md` → flow 6). Zero items
/// yields empty content (the absent-value case, not an error).
///
/// Deeper fragment shapes (`#unit/leaf`, `#unit/item[/leaf]`) are not store-read
/// targets at this scope and surface as a located block rather than a panic.
fn slice_fragment(
    doc: &Document,
    source: &str,
    fragment: &Fragment,
    address: &str,
) -> Result<String, Finding> {
    let Fragment::Unit(unit) = fragment else {
        return Err(block(
            "store.unsliceable-fragment",
            format!("`{address}` does not name a `#section` slot"),
            address,
            "slice a `#section` whose body is a slot".to_string(),
        ));
    };

    let Some(section) = doc.sections.iter().find(|s| s.id == unit.as_str()) else {
        return Err(block(
            "store.no-such-section",
            format!("`{address}` names no section `{unit}`"),
            address,
            "name a section that exists in the committed doc".to_string(),
        ));
    };

    // A slot section slices to its prose span; a repeatable section (no slot) slices
    // to its rendered items — the read-path inverse for each non-deref shape.
    match &section.slot {
        Some(span) => Ok(span.slice(source).to_string()),
        None => Ok(render_repeatable_items(section, source)),
    }
}

/// Render a repeatable section's items as a Content list for the store-read path:
/// each item is its `### <title>` heading and, when the item block carries a slot
/// (e.g. the spec criterion's `statement`), the slot prose beneath it. Items are
/// joined by a blank line, mirroring their on-disk order. The composer wraps the
/// whole string as a `> ` Content blockquote, so `{{@task.spec#criteria}}` reads as
/// the titled, prose-carrying criteria list the implementer needs.
fn render_repeatable_items(section: &ParsedSection, source: &str) -> String {
    section
        .items
        .iter()
        .map(|item| {
            if !item.slots.is_empty() {
                // A multi-slot item: the `### <title>` heading then each slot under its
                // `#### <Leaf-Title>` sub-heading (the writer's form), so the read view
                // mirrors the on-disk shape.
                let mut out = format!("### {}", item.title.trim());
                for (leaf_id, span) in &item.slots {
                    out.push_str(&format!(
                        "\n\n#### {}\n\n{}",
                        title_case_label(leaf_id),
                        span.slice(source).trim()
                    ));
                }
                out
            } else {
                match &item.slot {
                    Some(span) => {
                        format!("### {}\n\n{}", item.title.trim(), span.slice(source).trim())
                    }
                    None => format!("### {}", item.title.trim()),
                }
            }
        })
        .collect::<Vec<_>>()
        .join("\n\n")
}

/// Title-case a single-word slot leaf id for its `#### <Leaf-Title>` sub-heading in
/// the store-read view (mirrors the writer's single-word heading rendering).
fn title_case_label(id: &str) -> String {
    let mut chars = id.chars();
    match chars.next() {
        Some(first) => first.to_ascii_uppercase().to_string() + chars.as_str(),
        None => String::new(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const ADR_YAML: &[u8] = include_bytes!("../../cli/pack/schemas/adr.yaml");
    const COMMIT_YAML: &[u8] = include_bytes!("../../cli/pack/schemas/commit.yaml");
    const SPEC_YAML: &[u8] = include_bytes!("../../cli/pack/schemas/spec.yaml");

    #[test]
    fn lexical_normalize_collapses_curdir_and_parentdir() {
        // The shared path-collision normalization (review C1): a `./`-prefixed or
        // redundant-`..` spelling of the canonical path collapses to the clean form,
        // so retire-side and create-side guards compare equal regardless of spelling.
        assert_eq!(
            lexical_normalize(Path::new("./changelog/changelog.md")),
            PathBuf::from("changelog/changelog.md"),
        );
        assert_eq!(
            lexical_normalize(Path::new("changelog/../changelog/changelog.md")),
            PathBuf::from("changelog/changelog.md"),
        );
    }

    /// A throwaway directory that removes itself on drop — keeps store-read tests
    /// off any real repo tree.
    struct TempRoot(PathBuf);

    impl TempRoot {
        fn new(tag: &str) -> Self {
            let mut path = std::env::temp_dir();
            let unique = format!(
                "jigc-store-{tag}-{}-{:?}",
                std::process::id(),
                std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap()
                    .as_nanos(),
            );
            path.push(unique);
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
            crate::schema::load_schema_with_types(ADR_YAML, &crate::schema::dev_pack_field_types())
                .expect("adr.yaml loads"),
        );
        m.insert(
            "commit".to_string(),
            crate::schema::load_schema(COMMIT_YAML).expect("commit.yaml loads"),
        );
        m.insert(
            "spec".to_string(),
            crate::schema::load_schema_with_types(
                SPEC_YAML,
                &crate::schema::dev_pack_field_types(),
            )
            .expect("spec.yaml loads"),
        );
        m
    }

    /// A committed `spec` whose `criteria` is a **repeatable** section (the shipped
    /// shape) — the read target `implement-from-spec`'s `locate-from-spec` step reads
    /// via `{{@task.spec#criteria}}`. Human-editable, conformant bytes.
    const COMMITTED_SPEC: &str = "\
# Gateway rate limiting

## Goal

Bound per-client request volume at the gateway.

## Context

Downstream services were each enforcing limits ad hoc.

## Criteria

### Rejects the 101st request  {#rejects-burst}

The gateway rejects the 101st request in a rolling 60s window.

### Recovers after the window  {#recovers}

The next window admits requests again.
";

    /// The canonical committed-ADR fixture flow #5 re-reads
    /// (`decisions/single-node-cache.md`). Human-editable, conformant bytes.
    const COMMITTED_ADR: &str = "\
---
status: accepted
date: 2026-05-23
---

# Single-node session cache

## Context
Session lookups must stay sub-millisecond.

## Decision
A single in-memory node keeps session lookups sub-millisecond and avoids a
network hop; acceptable because sessions are cheap to reconstruct on a cold node.

## Consequences
A cold node loses its sessions; clients re-authenticate.
";

    /// Write the committed ADR fixture to its canonical path under `repo_root`.
    fn write_committed_adr(repo_root: &Path) {
        let path = repo_root.join("decisions").join("single-node-cache.md");
        std::fs::create_dir_all(path.parent().unwrap()).expect("mk decisions/");
        std::fs::write(&path, COMMITTED_ADR).expect("write committed ADR");
    }

    /// GOLDEN: a committed ADR at `decisions/single-node-cache.md` is resolved from
    /// `adr:single-node-cache#decision`, parsed against the schema, and its
    /// `#decision` section slot sliced to its prose — byte-exact over the committed
    /// file. The snapshot pins the sliced prose bytes.
    #[test]
    fn store_reads_and_slices_a_committed_adr_section() {
        let root = TempRoot::new("read-slice");
        write_committed_adr(root.path());

        let address = Address::parse("adr:single-node-cache#decision").expect("valid address");
        let prose =
            read_slice(root.path(), &schemas(), &address).expect("committed ADR slice resolves");

        insta::assert_snapshot!(prose, @r"
        A single in-memory node keeps session lookups sub-millisecond and avoids a
        network hop; acceptable because sessions are cheap to reconstruct on a cold node.
        ");
    }

    /// GOLDEN: a committed `spec`'s **repeatable** `criteria` section is sliced via
    /// `spec:gateway-rate-limiting#criteria` — the section has no slot, so the slice
    /// renders its items (each `### <title>` + its `statement` slot prose) as the
    /// Content list `{{@task.spec#criteria}}` reads (`worked-examples.md` → flow 6).
    /// This is the read path the shipped repeatable `criteria` needs; the slot-only
    /// path (flow #5's `#decision`) never exercised it.
    #[test]
    fn store_slices_a_committed_spec_repeatable_criteria_section() {
        let root = TempRoot::new("spec-criteria");
        let path = root.path().join("specs").join("gateway-rate-limiting.md");
        std::fs::create_dir_all(path.parent().unwrap()).expect("mk specs/");
        std::fs::write(&path, COMMITTED_SPEC).expect("write committed spec");

        let address = Address::parse("spec:gateway-rate-limiting#criteria").expect("valid address");
        let rendered = read_slice(root.path(), &schemas(), &address)
            .expect("committed spec criteria slice resolves");

        insta::assert_snapshot!(rendered, @r"
        ### Rejects the 101st request

        The gateway rejects the 101st request in a rolling 60s window.

        ### Recovers after the window

        The next window admits requests again.
        ");
    }

    /// Read-tolerance: a committed ADR saved with a leading UTF-8 BOM (common from
    /// Windows editors — the file is an editable channel) still reads and slices,
    /// per `parsing.md` → Round-trip guarantees ("BOM — tolerate on read (skip)").
    /// The BOM is stripped before both parse and slice, so the prose stays aligned.
    #[test]
    fn store_tolerates_a_leading_bom_on_a_committed_adr() {
        let root = TempRoot::new("bom");
        let path = root.path().join("decisions").join("single-node-cache.md");
        std::fs::create_dir_all(path.parent().unwrap()).expect("mk decisions/");
        std::fs::write(&path, format!("\u{feff}{COMMITTED_ADR}")).expect("write BOM'd ADR");

        let address = Address::parse("adr:single-node-cache#decision").expect("valid address");
        let prose = read_slice(root.path(), &schemas(), &address)
            .expect("a BOM-prefixed committed ADR still resolves");
        assert!(
            prose.starts_with("A single in-memory node"),
            "sliced prose is byte-aligned despite the BOM: {prose:?}"
        );
    }

    /// A missing target file yields a blocking [`Finding`] with a located message
    /// and a route — never a panic, never a silent empty.
    #[test]
    fn store_read_of_a_missing_file_is_a_blocking_finding() {
        let root = TempRoot::new("missing");
        // No file written.
        let address = Address::parse("adr:does-not-exist#decision").expect("valid address");
        let err = read_slice(root.path(), &schemas(), &address)
            .expect_err("a missing committed file blocks");

        assert_eq!(err.severity, Severity::Blocking);
        assert_eq!(err.code, "store.not-found");
        assert!(
            err.message.contains("adr:does-not-exist"),
            "block names the unresolved address: {err:?}"
        );
        assert!(err.location.is_some(), "the block is located");
        assert!(err.route.is_some(), "the block carries a route");
    }
}

#[cfg(test)]
mod prop_tests {
    use super::*;
    use crate::write::{self, Instance, SectionContent};
    use proptest::prelude::*;

    const ADR_YAML: &[u8] = include_bytes!("../../cli/pack/schemas/adr.yaml");

    fn adr_schema() -> Schema {
        crate::schema::load_schema_with_types(ADR_YAML, &crate::schema::dev_pack_field_types())
            .expect("adr.yaml loads")
    }

    fn schemas() -> BTreeMap<String, Schema> {
        let mut m = BTreeMap::new();
        m.insert("adr".to_string(), adr_schema());
        m
    }

    /// A throwaway directory that removes itself on drop.
    struct TempRoot(PathBuf);

    impl TempRoot {
        fn new() -> Self {
            let mut path = std::env::temp_dir();
            let unique = format!(
                "jigc-store-prop-{}-{:?}",
                std::process::id(),
                std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap()
                    .as_nanos(),
            );
            path.push(unique);
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

    /// Heading-free slot prose: arbitrary paragraphs and `####`-deep headings, but
    /// never a `##`/`###` ATX heading or a Setext underline (the ceiling), and never
    /// a `<!-- fields -->` sentinel. Mirrors the parse-side slot strategy so the
    /// generated ADR is conformant.
    fn slot_prose_strategy() -> impl Strategy<Value = String> {
        let line = prop_oneof![
            "[a-zA-Z][a-zA-Z0-9 .,]{0,40}",
            "#### [a-zA-Z][a-zA-Z0-9 ]{0,20}",
        ];
        prop::collection::vec(line, 1..5).prop_map(|lines| lines.join("\n\n"))
    }

    proptest! {
        /// PARSE→SLICE round-trip: for any conformant ADR built via [`write::render`],
        /// a store-read of each slot fragment slices **byte-for-byte** to that slot's
        /// source prose. The committed bytes are written to the canonical path under a
        /// repo root, then `read_slice` re-reads and re-parses them over the recorded
        /// span — the round-trip on a committed, human-editable file
        /// ([parsing.md](../../../implementation/parsing.md) → Round-trip guarantees).
        #[test]
        fn read_slice_round_trips_every_slot_byte_for_byte(
            context in slot_prose_strategy(),
            decision in slot_prose_strategy(),
            consequences in slot_prose_strategy(),
        ) {
            let schema = adr_schema();
            let instance = Instance {
                title: "Single-node session cache".to_string(),
                sections: vec![
                    SectionContent { id: "status".to_string(), ..Default::default() },
                    SectionContent {
                        id: "context".to_string(),
                        slot: Some(context.clone()),
                        ..Default::default()
                    },
                    SectionContent {
                        id: "decision".to_string(),
                        slot: Some(decision.clone()),
                        ..Default::default()
                    },
                    SectionContent {
                        id: "consequences".to_string(),
                        slot: Some(consequences.clone()),
                        ..Default::default()
                    },
                ],
            };
            let bytes = write::render(&schema, &instance);

            // Commit it at its canonical path.
            let root = TempRoot::new();
            let path = root.path().join("decisions").join("single-node-cache.md");
            std::fs::create_dir_all(path.parent().unwrap()).expect("mk decisions/");
            std::fs::write(&path, &bytes).expect("write committed ADR");

            let schemas = schemas();
            for (section, expected) in [
                ("context", &context),
                ("decision", &decision),
                ("consequences", &consequences),
            ] {
                let address = Address::parse(&format!("adr:single-node-cache#{section}"))
                    .expect("valid address");
                let got = read_slice(root.path(), &schemas, &address)
                    .expect("conformant ADR slice resolves");
                // The writer `trim_end`s prose and the parser trims the span, so the
                // recorded slot bytes are the trimmed prose; the canonical render
                // places it contiguously, so trimmed == the prose itself.
                prop_assert_eq!(got, expected.trim());
            }
        }
    }
}
