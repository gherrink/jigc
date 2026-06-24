//! The doctype-set **freeze manifest** — the versioned/hashed declaration of the
//! frozen v1 doctype set, and the engine-side check that recomputes each shipped
//! doctype's schema-hash against it.
//!
//! M33 declares the persisted doctype set + the schema-definition format a
//! **frozen v1** and makes the freeze *enforced, not asserted*: a schema-shape
//! change that bumps no version + ships no migration must be **blocked**, not
//! caught by review (`design/corpus-migration.md` → The freeze — declared *and*
//! enforced; A versioned, hashed doctype-set manifest).
//!
//! This module owns the model + the mechanism only — it ships **no** doctype
//! content (the engine-empty invariant): the enumerated `doctype → schema-version
//! + schema-hash` artifact rides in the pack and is fed in. Three pieces:
//!
//! - [`schema_hash`] — a doctype definition's deterministic content hash, the
//!   `blake3` hex of its canonical JSON. [`Schema`] carries only ordered `Vec`s
//!   (no maps), so `serde_json` emits stable bytes; the hash is stable across a
//!   `load → serialize → reload` round-trip.
//! - [`Manifest`] / [`ManifestEntry`] — the deserializable on-disk declaration.
//! - [`check`] — recomputes each shipped schema's hash and compares it to its
//!   manifest entry, and requires the declared set and the shipped set to be
//!   exactly equal, returning the first violation in sorted doctype order.
//!
//! The **firing surface** is the pack-load / build-time assertion (the
//! intrinsic-floor-assertion sibling — `design/corpus-migration.md` → review
//! Finding 3), *not* the report-only `validate` store sweep. This module supplies
//! the mechanism that assertion calls; wiring it at pack-load is a sibling task.

use crate::file_state::hash_bytes;
use crate::schema::Schema;
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};
use thiserror::Error;

/// The deterministic **schema-hash** of a doctype definition: the lowercase-hex
/// `blake3` digest of the schema's canonical JSON serialization.
///
/// [`Schema`] (and its whole transitive model) carries only ordered `Vec`s — no
/// maps — so `serde_json::to_vec` emits the same bytes every run, making the hash
/// a stable fingerprint of the schema's *shape*. Serialization of the schema
/// model is infallible (no `Serialize` impl in the model can error), so the
/// (unreachable) error is surfaced as a panic rather than silently swallowed.
pub fn schema_hash(schema: &Schema) -> String {
    let bytes = serde_json::to_vec(schema).expect("Schema serializes to JSON infallibly");
    hash_bytes(&bytes)
}

/// One manifest entry: a doctype's declared schema-version + frozen schema-hash.
///
/// The version is **per-doctype** (the doc's own doctype version, matched against
/// this entry — `design/corpus-migration.md` → The schema-version stamp), not a
/// global doctype-set version. The hash is the [`schema_hash`] of the doctype's
/// definition as frozen at that version.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ManifestEntry {
    /// The doctype name (e.g. `adr`, `commit`).
    #[serde(rename = "type")]
    pub ty: String,

    /// The doctype's declared schema version (v1 at the freeze).
    #[serde(rename = "schema-version")]
    pub schema_version: u32,

    /// The frozen [`schema_hash`] of the doctype's definition at that version.
    #[serde(rename = "schema-hash")]
    pub schema_hash: String,
}

/// The doctype-set manifest: the enumerated, ordered `doctype → version + hash`
/// declaration that makes the freeze self-enforcing.
///
/// An ordered `Vec` (no map) for the same determinism reason the schema model is
/// map-free — the on-disk form is surface-visible and order-stable.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Manifest {
    /// The frozen doctype entries, in declaration order.
    #[serde(default)]
    pub doctypes: Vec<ManifestEntry>,
}

/// Why a shipped doctype-set failed its freeze-manifest check.
///
/// Each variant names the offending `doctype` — the doctype [`check`] surfaces
/// first in sorted name order (a given doctype hits at most one variant: a hash
/// either matches, or the doctype is extra, or it is missing).
#[derive(Clone, Debug, PartialEq, Eq, Error)]
pub enum ManifestError {
    /// A shipped doctype's recomputed schema-hash differs from its manifest
    /// entry — a schema-shape change that bumped no version (the freeze breach
    /// the gate exists to block).
    #[error(
        "doctype `{doctype}`: schema-hash mismatch (manifest declares `{expected}`, recomputed `{actual}`)"
    )]
    HashMismatch {
        /// The doctype whose shape drifted from its declared hash.
        doctype: String,
        /// The hash the manifest declares.
        expected: String,
        /// The hash recomputed from the shipped schema.
        actual: String,
    },

    /// A shipped doctype has **no** manifest entry — an undeclared doctype added
    /// to the set without being frozen.
    #[error("doctype `{doctype}` is shipped but absent from the freeze manifest")]
    ExtraEntry {
        /// The undeclared shipped doctype.
        doctype: String,
    },

    /// A manifest entry has **no** shipped doctype — a declared doctype removed
    /// from the set without bumping the manifest.
    #[error("doctype `{doctype}` is declared in the freeze manifest but no schema ships it")]
    MissingEntry {
        /// The declared-but-unshipped doctype.
        doctype: String,
    },
}

/// Verify a shipped doctype-set against the freeze manifest.
///
/// `schemas` is keyed by doctype name (a [`BTreeMap`], so iteration is sorted and
/// the check is independent of any insertion order). For each doctype in the
/// **union** of the declared and the shipped sets, taken in sorted name order:
///
/// - declared **and** shipped → recompute its [`schema_hash`] and compare
///   ([`ManifestError::HashMismatch`] on drift — the freeze breach),
/// - shipped but **not** declared → [`ManifestError::ExtraEntry`] (an undeclared
///   doctype added to the set),
/// - declared but **not** shipped → [`ManifestError::MissingEntry`] (a declared
///   doctype removed from the set).
///
/// The declared set and the shipped set must therefore be **exactly equal**, each
/// hash matching. The **first** violation in sorted doctype order is returned, so
/// the result is a pure function of the manifest + the schema *content* — the
/// loud, deterministic failure the pack-load freeze gate fires on.
pub fn check(manifest: &Manifest, schemas: &BTreeMap<String, Schema>) -> Result<(), ManifestError> {
    let declared: BTreeMap<&str, &ManifestEntry> = manifest
        .doctypes
        .iter()
        .map(|entry| (entry.ty.as_str(), entry))
        .collect();

    // The union of declared + shipped doctype names, sorted (a BTreeSet), so the
    // single returned violation is deterministic regardless of input order.
    let mut names: BTreeSet<&str> = BTreeSet::new();
    names.extend(declared.keys().copied());
    names.extend(schemas.keys().map(String::as_str));

    for name in names {
        match (declared.get(name), schemas.get(name)) {
            (Some(entry), Some(schema)) => {
                let actual = schema_hash(schema);
                if actual != entry.schema_hash {
                    return Err(ManifestError::HashMismatch {
                        doctype: name.to_string(),
                        expected: entry.schema_hash.clone(),
                        actual,
                    });
                }
            }
            (None, Some(_)) => {
                return Err(ManifestError::ExtraEntry {
                    doctype: name.to_string(),
                });
            }
            (Some(_), None) => {
                return Err(ManifestError::MissingEntry {
                    doctype: name.to_string(),
                });
            }
            // Unreachable: every name came from one of the two maps.
            (None, None) => unreachable!("name drawn from the union of both sets"),
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::schema::load_schema;

    const WIDGET_YAML: &str = "\
type: widget
location: widgets/
id-from: title
sections:
  - id: header
    header: true
    fields:
      - { id: title, type: string }
      - { id: status, type: enum, of: [open, closed], default: open }
  - id: body
    slot: { hint: \"What the widget does.\" }
";

    const GADGET_YAML: &str = "\
type: gadget
location: gadgets/
id-from: title
sections:
  - id: header
    header: true
    fields:
      - { id: title, type: string }
  - id: detail
    slot: { hint: \"The gadget detail.\" }
";

    fn widget() -> Schema {
        load_schema(WIDGET_YAML.as_bytes()).expect("widget fixture loads")
    }

    fn gadget() -> Schema {
        load_schema(GADGET_YAML.as_bytes()).expect("gadget fixture loads")
    }

    /// The doctype-keyed schema map `check` consumes.
    fn schema_map(schemas: impl IntoIterator<Item = Schema>) -> BTreeMap<String, Schema> {
        schemas.into_iter().map(|s| (s.ty.clone(), s)).collect()
    }

    /// A manifest whose entries' hashes match the supplied schemas (version 1).
    fn manifest_for(schemas: &[Schema]) -> Manifest {
        Manifest {
            doctypes: schemas
                .iter()
                .map(|s| ManifestEntry {
                    ty: s.ty.clone(),
                    schema_version: 1,
                    schema_hash: schema_hash(s),
                })
                .collect(),
        }
    }

    #[test]
    fn matching_set_passes() {
        let schemas = vec![widget(), gadget()];
        let manifest = manifest_for(&schemas);
        assert_eq!(check(&manifest, &schema_map(schemas)), Ok(()));
    }

    #[test]
    fn shape_mutated_without_bump_is_hash_mismatch() {
        let manifest = manifest_for(&[widget(), gadget()]);
        let expected = schema_hash(&widget());

        // Mutate the widget's *shape* (its body slot) without bumping its version.
        let mut mutated = widget();
        if let crate::schema::SectionBody::Simple { slot, .. } = &mut mutated.sections[1].body {
            *slot = Some(crate::schema::Slot {
                hint: Some("a different hint".into()),
                optional: false,
            });
        } else {
            panic!("fixture body section is simple");
        }
        let actual = schema_hash(&mutated);
        assert_ne!(actual, expected, "the mutation must change the hash");

        let err = check(&manifest, &schema_map([mutated, gadget()])).expect_err("must fail");
        assert_eq!(
            err,
            ManifestError::HashMismatch {
                doctype: "widget".into(),
                expected,
                actual,
            }
        );
    }

    #[test]
    fn added_doctype_is_extra_entry() {
        // Manifest knows only `widget`; `gadget` is shipped but undeclared.
        let manifest = manifest_for(&[widget()]);
        let err = check(&manifest, &schema_map([widget(), gadget()])).expect_err("must fail");
        assert_eq!(
            err,
            ManifestError::ExtraEntry {
                doctype: "gadget".into()
            }
        );
    }

    #[test]
    fn removed_doctype_is_missing_entry() {
        // Manifest declares both; only `widget` ships.
        let manifest = manifest_for(&[widget(), gadget()]);
        let err = check(&manifest, &schema_map([widget()])).expect_err("must fail");
        assert_eq!(
            err,
            ManifestError::MissingEntry {
                doctype: "gadget".into()
            }
        );
    }

    #[test]
    fn schema_hash_stable_across_load_serialize_reload() {
        let schema = widget();
        let first = schema_hash(&schema);

        // serialize -> reload -> rehash must reproduce the same digest.
        let json = serde_json::to_vec(&schema).expect("serialize");
        let reloaded: Schema = serde_json::from_slice(&json).expect("reload");
        let second = schema_hash(&reloaded);
        assert_eq!(first, second);

        // a fresh parse of the same source bytes is also identical.
        let reparsed = widget();
        assert_eq!(first, schema_hash(&reparsed));
    }

    #[test]
    fn first_violation_is_deterministic_in_sorted_doctype_order() {
        // Both a hash drift (widget) and an extra doctype (gadget) are present;
        // `check` returns the violation whose doctype sorts first (`gadget`),
        // independent of how the schemas/entries were ordered on input. A
        // BTreeMap normalizes key order, so insertion order cannot perturb it.
        let manifest = manifest_for(&[widget()]); // declares only widget

        let mut drifted = widget();
        drifted.description = Some("a shape change".into());

        let from_widget_first = check(&manifest, &schema_map([drifted.clone(), gadget()]));
        let from_gadget_first = check(&manifest, &schema_map([gadget(), drifted]));
        assert_eq!(
            from_widget_first, from_gadget_first,
            "the surfaced violation must not depend on input order"
        );
        assert_eq!(
            from_widget_first,
            Err(ManifestError::ExtraEntry {
                doctype: "gadget".into()
            })
        );
    }
}
