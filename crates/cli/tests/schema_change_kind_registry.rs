//! **The schema-change kind space becomes a set** (M50 Increment 6 / T1).
//!
//! [`engine::schema_diff::SchemaChange`] carries data on every variant, so it can never
//! have an `ALL` of its own — which is why the classifier's kind space was, until this
//! task, **not enumerable**: nothing could iterate it, and the wire names the transform
//! driver puts on the surface were five hand-written string literals with no relation to
//! the variants they claimed to name. This suite is the fence that makes the space a set,
//! in the `flow37_rename.rs::every_refusal_kind_is_declared` shape
//! (`completions/artifacts/M50/settle-record.md` → D6; *Ordering constraints* #2).
//!
//! Three claims, each mechanical:
//!
//! 1. **The set is closed and counted** — an exhaustive match over
//!    [`SchemaChangeKind::ALL`] (the compiler's own completeness check) plus its length,
//!    so a nineteenth kind cannot compile without joining `ALL`, and `ALL` cannot silently
//!    lose a member.
//! 2. **Every classified change names its kind** — the discriminant projection
//!    `From<&SchemaChange>` is exhaustive by construction, but an arm wired to the *wrong*
//!    kind still compiles; the mapping is therefore driven, one constructed change per
//!    member.
//! 3. **The wire names come off the registry** — every
//!    [`engine::transform::TransformError::Unsupported`] construction site in the driver's
//!    production source reads its `kind` from [`SchemaChangeKind::as_str`], never from a
//!    literal. This arm is a **source read** because the strings a refusal can carry are
//!    not enumerable at runtime from the type (`kind` is a `&'static str` by the shipped
//!    1.0 shape): the drift the fence exists to catch is a literal typed at a construction
//!    site, and that site is exactly what it reads.
//!
//! **Bound, stated:** arm 3 reads the driver's **production** region only — the source
//! above `#[cfg(test)]`. The module's own unit tests do spell wire names literally, and
//! deliberately: each is an *assertion about* the value a production site emits, so a drift
//! between the two reddens that test on its own. A literal in the production region has no
//! such counterweight, which is the whole reason for this arm.
//!
//! The kind space itself — which shapes classify to which kind at which locus — is T2's
//! subject; this file asserts only that the space is a **set** T2's completeness fence can
//! iterate.

use engine::schema_diff::{SchemaChange, SchemaChangeKind};
use std::collections::{BTreeMap, BTreeSet};
use std::path::PathBuf;

/// **The set is closed, counted, and its wire names are distinct.**
///
/// The match is exhaustive, so a kind added to the enum without joining `ALL` fails to
/// compile here; the length assertion catches the opposite drift (a member dropped from
/// `ALL` while its variant lives on). Distinctness matters because `as_str` is what the
/// refusal surface prints and what the CLI's value-remap route compares against: two kinds
/// sharing a spelling would make that route fire for the wrong one.
#[test]
fn every_schema_change_kind_is_declared() {
    for kind in SchemaChangeKind::ALL {
        match kind {
            SchemaChangeKind::AddedOptionalField
            | SchemaChangeKind::WidenedCardinality
            | SchemaChangeKind::NarrowedCardinality
            | SchemaChangeKind::EnumWidened
            | SchemaChangeKind::ValueRemapped
            | SchemaChangeKind::AddedItemField
            | SchemaChangeKind::AddedItemSlot
            | SchemaChangeKind::AddedOptionalSection
            | SchemaChangeKind::AddedRepeatableSection
            | SchemaChangeKind::FixedSlotToRepeatable
            | SchemaChangeKind::ProseNeeding
            | SchemaChangeKind::Relocated
            | SchemaChangeKind::DisplayTitleChanged
            | SchemaChangeKind::OptionalRelaxed
            | SchemaChangeKind::RemovedField
            | SchemaChangeKind::RemovedItemSlot
            | SchemaChangeKind::PresentationOnly
            | SchemaChangeKind::Unclassified => {}
        }
    }
    assert_eq!(
        SchemaChangeKind::ALL.len(),
        18,
        "a new `SchemaChangeKind` joins `ALL` (and T2's `kind x locus` disposition gains its cells)",
    );
    let spellings: BTreeSet<&str> = SchemaChangeKind::ALL.iter().map(|k| k.as_str()).collect();
    assert_eq!(
        spellings.len(),
        SchemaChangeKind::ALL.len(),
        "two kinds share a wire name; the refusal surface and the value-remap route both key on it",
    );
}

/// **Every classified change names its own kind.**
///
/// `From<&SchemaChange>` is exhaustive by construction — a nineteenth `SchemaChange`
/// variant does not compile without an arm — but an arm pointing at the *wrong* kind
/// compiles perfectly, and a mis-wired projection would hand T2's completeness fence a
/// silently wrong locus. So the mapping is driven: one constructed change per member,
/// checked both ways (every kind is reached, and each change projects to the kind that
/// names it).
#[test]
fn every_change_names_its_kind() {
    let section = || "notes".to_string();
    let cases: Vec<(SchemaChangeKind, SchemaChange)> = vec![
        (
            SchemaChangeKind::AddedOptionalField,
            SchemaChange::AddedOptionalField {
                section: section(),
                field: "status".into(),
            },
        ),
        (
            SchemaChangeKind::WidenedCardinality,
            SchemaChange::WidenedCardinality {
                section: section(),
                field: "status".into(),
            },
        ),
        (
            SchemaChangeKind::NarrowedCardinality,
            SchemaChange::NarrowedCardinality {
                section: section(),
                field: "status".into(),
            },
        ),
        (
            SchemaChangeKind::EnumWidened,
            SchemaChange::EnumWidened {
                section: section(),
                field: "status".into(),
            },
        ),
        (
            SchemaChangeKind::ValueRemapped,
            SchemaChange::ValueRemapped {
                section: section(),
                field: "status".into(),
                map: BTreeMap::new(),
            },
        ),
        (
            SchemaChangeKind::AddedItemField,
            SchemaChange::AddedItemField {
                section: section(),
                field: "status".into(),
            },
        ),
        (
            SchemaChangeKind::AddedItemSlot,
            SchemaChange::AddedItemSlot {
                section: section(),
                leaf: "detail".into(),
            },
        ),
        (
            SchemaChangeKind::AddedOptionalSection,
            SchemaChange::AddedOptionalSection { section: section() },
        ),
        (
            SchemaChangeKind::AddedRepeatableSection,
            SchemaChange::AddedRepeatableSection { section: section() },
        ),
        (
            SchemaChangeKind::FixedSlotToRepeatable,
            SchemaChange::FixedSlotToRepeatable { section: section() },
        ),
        (
            SchemaChangeKind::ProseNeeding,
            SchemaChange::ProseNeeding {
                section: section(),
                leaf: None,
            },
        ),
        (
            SchemaChangeKind::Relocated,
            SchemaChange::Relocated {
                from: "docs".into(),
                to: "docs/decisions".into(),
            },
        ),
        (
            SchemaChangeKind::DisplayTitleChanged,
            SchemaChange::DisplayTitleChanged {
                to: "Changelog".into(),
            },
        ),
        (
            SchemaChangeKind::OptionalRelaxed,
            SchemaChange::OptionalRelaxed {
                section: section(),
                leaf: Some("status".into()),
            },
        ),
        (
            SchemaChangeKind::RemovedField,
            SchemaChange::RemovedField {
                section: section(),
                field: "status".into(),
            },
        ),
        (
            SchemaChangeKind::RemovedItemSlot,
            SchemaChange::RemovedItemSlot {
                section: section(),
                leaf: "detail".into(),
            },
        ),
        (
            SchemaChangeKind::PresentationOnly,
            SchemaChange::PresentationOnly,
        ),
        (SchemaChangeKind::Unclassified, SchemaChange::Unclassified),
    ];

    for (kind, change) in &cases {
        assert_eq!(
            SchemaChangeKind::from(change),
            *kind,
            "`{change:?}` projects to the wrong kind",
        );
    }
    let reached: BTreeSet<&str> = cases.iter().map(|(kind, _)| kind.as_str()).collect();
    let declared: BTreeSet<&str> = SchemaChangeKind::ALL.iter().map(|k| k.as_str()).collect();
    assert_eq!(
        reached, declared,
        "every declared kind needs a constructed change here, and no case may name a kind twice",
    );
}

/// **The wire names come off the registry, at every production construction site.**
///
/// Reads `crates/engine/src/transform.rs` above its `#[cfg(test)]` line and requires every
/// `kind:` field initializer to be `SchemaChangeKind::<Variant>.as_str()`, with `<Variant>`
/// a declared member. Before this task the same read found five hand-written literals
/// (`"narrowed-cardinality"`, `"removed-field"`, `"removed-item-slot"`, `"prose-needing"`,
/// `"added-optional-field"`, `"added-item-field"`) plus a parallel `VALUE_REMAP_KIND`
/// const — six spellings the classifier's variants had no mechanical relation to, so a
/// renamed kind kept its old name on the refusal surface at exit 0.
///
/// The one initializer that is *not* a construction site — the enum field's own
/// `kind: &'static str,` declaration — is excluded by name rather than by position, so a
/// site moving above or below it is still read.
#[test]
fn every_unsupported_wire_name_comes_off_the_registry() {
    let by_variant: BTreeMap<String, &str> = SchemaChangeKind::ALL
        .iter()
        .map(|kind| (format!("{kind:?}"), kind.as_str()))
        .collect();

    let sites = production_kind_initializers();
    assert!(
        !sites.is_empty(),
        "no `kind:` initializer found in the driver's production source — the read is broken, \
         not the code",
    );
    for site in &sites {
        let variant = site
            .strip_prefix("SchemaChangeKind::")
            .and_then(|rest| rest.strip_suffix(".as_str()"))
            .unwrap_or_else(|| {
                panic!(
                    "`kind: {site}` is not read off the registry — every `Unsupported` wire name \
                     must be a `SchemaChangeKind::<Variant>.as_str()` output",
                )
            });
        assert!(
            by_variant.contains_key(variant),
            "`kind: {site}` names `{variant}`, which is not a declared `SchemaChangeKind`",
        );
    }
}

/// Every `kind:` field initializer in the transform driver's **production** source (above
/// `#[cfg(test)]`), with the enum's own field declaration dropped. Returns the initializer
/// text, comma stripped.
fn production_kind_initializers() -> Vec<String> {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../engine/src/transform.rs");
    let source = std::fs::read_to_string(&path)
        .unwrap_or_else(|err| panic!("read {}: {err}", path.display()));
    let production = source
        .split("\n#[cfg(test)]\n")
        .next()
        .expect("the driver source has a production region");
    production
        .lines()
        .filter_map(|line| line.trim().strip_prefix("kind:"))
        .map(|value| value.trim().trim_end_matches(',').trim().to_string())
        // The `TransformError::Unsupported` field declaration itself, not a construction site.
        .filter(|value| value != "&'static str")
        .collect()
}
