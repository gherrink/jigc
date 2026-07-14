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

/// The declared **slug rule** — the identity-derivation rule's version + the
/// fingerprint of its behaviour ([`crate::slug::SLUG_RULE_VERSION`] /
/// [`crate::slug::rule_fingerprint`]).
///
/// `slugify` mints every id in every corpus yet rides in no `schema-hash`, so the
/// freeze assert was blind to a change in it — and no transform kind can re-mint
/// an id, so such a change **cannot be migrated after the fact** (a corpus that
/// spans one carries two id generations permanently). This block is what makes the
/// change a *declared* event: a pack that ships it is held to it at pack-load
/// ([`check`]; `design/storage.md` → Identity → *The slug rule is itself a
/// versioned rule (M42)*).
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SlugRule {
    /// The slug rule version this pack was frozen against.
    pub version: u32,

    /// The [`crate::slug::rule_fingerprint`] of the rule at that version.
    pub hash: String,
}

/// The doctype-set manifest: the enumerated, ordered `doctype → version + hash`
/// declaration that makes the freeze self-enforcing, plus the declared slug rule
/// the ids inside those doctypes are minted by.
///
/// An ordered `Vec` (no map) for the same determinism reason the schema model is
/// map-free — the on-disk form is surface-visible and order-stable.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Manifest {
    /// The declared slug rule. **Absent = unchecked** — the manifest-less
    /// precedent (a seeded / composed pack that opts out stays inert, never
    /// errors); a pack that declares it opts into the gate.
    #[serde(rename = "slug-rule", default, skip_serializing_if = "Option::is_none")]
    pub slug_rule: Option<SlugRule>,

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

    /// The **slug rule** changed: the recomputed [`crate::slug::rule_fingerprint`]
    /// differs from the manifest's declared `slug-rule.hash`. This is the
    /// identity-mint sibling of [`HashMismatch`](ManifestError::HashMismatch) — a
    /// change to the function that names every id in every corpus, which **no
    /// migration can repair** (no transform kind re-mints an id), so it must be
    /// declared, never slipped in.
    #[error(
        "the slug rule changed: the manifest declares slug-rule.hash `{expected}`, the engine recomputed `{actual}` — bump slug-rule-version + re-pin the hash in every shipped manifest (a slug-rule change cannot be migrated: it splits the corpus into two permanent id generations)"
    )]
    SlugRuleHashMismatch {
        /// The fingerprint the manifest declares.
        expected: String,
        /// The fingerprint recomputed from the shipped rule.
        actual: String,
    },

    /// The manifest's declared `slug-rule.version` is not the version the engine
    /// ships ([`crate::slug::SLUG_RULE_VERSION`]) — the **anti-silencing** arm: a
    /// rule change re-pinned into the hash alone (without the declared bump the
    /// change owes) still blocks, and a bumped rule leaves every un-re-pinned
    /// manifest loud.
    #[error(
        "slug-rule version mismatch: the manifest declares slug-rule.version {declared}, the engine ships slug-rule version {shipped} — bump slug-rule-version + re-pin the hash in every shipped manifest"
    )]
    SlugRuleVersionMismatch {
        /// The version the manifest declares.
        declared: u32,
        /// The version the engine's rule ships at.
        shipped: u32,
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
///
/// The declared **slug rule** ([`SlugRule`]) is checked **first**, before any
/// doctype: it is the rule the ids *inside* every doctype are minted by, so a
/// drift there is the more fundamental breach (and, unlike a schema-shape change,
/// one no migration can repair). An absent `slug-rule:` block is unchecked.
pub fn check(manifest: &Manifest, schemas: &BTreeMap<String, Schema>) -> Result<(), ManifestError> {
    check_slug_rule(manifest.slug_rule.as_ref())?;

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

/// Verify a declared [`SlugRule`] against the rule the engine actually ships.
///
/// Two arms, both loud, both routing to the same fix (*bump `slug-rule-version` +
/// re-pin the hash*):
///
/// - the declared **hash** must equal the recomputed [`crate::slug::rule_fingerprint`]
///   — the drift the gate exists to catch;
/// - the declared **version** must equal [`crate::slug::SLUG_RULE_VERSION`] — the
///   *anti-silencing* arm, so re-pinning the hash alone cannot quiet the gate: the
///   rule change still owes its declared bump, in every shipped manifest.
///
/// `None` (no `slug-rule:` block) is **unchecked** — the manifest-less precedent:
/// a pack opts into the gate by declaring the rule.
fn check_slug_rule(declared: Option<&SlugRule>) -> Result<(), ManifestError> {
    let Some(rule) = declared else {
        return Ok(());
    };
    if rule.version != crate::slug::SLUG_RULE_VERSION {
        return Err(ManifestError::SlugRuleVersionMismatch {
            declared: rule.version,
            shipped: crate::slug::SLUG_RULE_VERSION,
        });
    }
    let actual = crate::slug::rule_fingerprint();
    if rule.hash != actual {
        return Err(ManifestError::SlugRuleHashMismatch {
            expected: rule.hash.clone(),
            actual: actual.to_string(),
        });
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

    /// A manifest whose entries' hashes match the supplied schemas (version 1),
    /// declaring **no** slug rule (the unchecked, opted-out shape).
    fn manifest_for(schemas: &[Schema]) -> Manifest {
        Manifest {
            slug_rule: None,
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

    /// The same manifest, declaring the slug rule the engine actually ships (the
    /// shape both production packs carry).
    fn manifest_with_slug_rule(schemas: &[Schema]) -> Manifest {
        Manifest {
            slug_rule: Some(SlugRule {
                version: crate::slug::SLUG_RULE_VERSION,
                hash: crate::slug::rule_fingerprint().to_string(),
            }),
            ..manifest_for(schemas)
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

    /// The declared slug rule matches the shipped one → the gate is inert.
    #[test]
    fn matching_slug_rule_passes() {
        let schemas = vec![widget(), gadget()];
        let manifest = manifest_with_slug_rule(&schemas);
        assert_eq!(check(&manifest, &schema_map(schemas)), Ok(()));
    }

    /// The headline: a manifest whose `slug-rule.hash` differs from the recomputed
    /// [`crate::slug::rule_fingerprint`] **blocks loudly** — naming the recomputed
    /// hash and routing to the declared bump. This is the fence the whole increment
    /// rests on: `slugify` mints every id in every corpus, sits in no `schema-hash`,
    /// and **cannot be migrated after the fact**.
    #[test]
    fn slug_rule_hash_drift_blocks_naming_the_recomputed_hash() {
        let schemas = vec![widget(), gadget()];
        let mut manifest = manifest_with_slug_rule(&schemas);
        manifest.slug_rule = Some(SlugRule {
            version: crate::slug::SLUG_RULE_VERSION,
            hash: "0".repeat(64),
        });

        let err = check(&manifest, &schema_map(schemas)).expect_err("a drifted rule must block");
        assert_eq!(
            err,
            ManifestError::SlugRuleHashMismatch {
                expected: "0".repeat(64),
                actual: crate::slug::rule_fingerprint().to_string(),
            }
        );
        // The message names the recomputed hash and the route (the agent/human
        // reading stderr must know what to do; `design/storage.md` → the slug rule).
        let msg = err.to_string();
        assert!(
            msg.contains(crate::slug::rule_fingerprint()),
            "the message must name the recomputed hash; got: {msg}"
        );
        assert!(
            msg.contains("bump slug-rule-version + re-pin"),
            "the message must route to the declared bump; got: {msg}"
        );
    }

    /// The **anti-silencing** arm: re-pinning the hash alone cannot quiet the gate.
    /// A manifest whose declared `slug-rule.version` is not the engine's
    /// [`crate::slug::SLUG_RULE_VERSION`] blocks with the same route — so a rule
    /// change must carry its *declared bump*, in every shipped manifest, not just a
    /// freshly-regenerated digest.
    #[test]
    fn slug_rule_version_mismatch_blocks_even_with_a_correct_hash() {
        let schemas = vec![widget(), gadget()];
        let mut manifest = manifest_with_slug_rule(&schemas);
        let stale = crate::slug::SLUG_RULE_VERSION + 1;
        manifest.slug_rule = Some(SlugRule {
            version: stale,
            hash: crate::slug::rule_fingerprint().to_string(), // the hash is CORRECT
        });

        let err =
            check(&manifest, &schema_map(schemas)).expect_err("a version mismatch must block");
        assert_eq!(
            err,
            ManifestError::SlugRuleVersionMismatch {
                declared: stale,
                shipped: crate::slug::SLUG_RULE_VERSION,
            }
        );
        assert!(
            err.to_string().contains("bump slug-rule-version + re-pin"),
            "the message must route to the declared bump; got: {err}"
        );
    }

    /// The omitting context (the manifest-less precedent, applied within the
    /// manifest): a manifest with **no** `slug-rule:` block is **unchecked** — inert,
    /// never an error — so every pack that has not opted in (and every fixture)
    /// stays green.
    #[test]
    fn absent_slug_rule_block_is_unchecked() {
        let schemas = vec![widget(), gadget()];
        let manifest = manifest_for(&schemas); // no slug-rule block
        assert_eq!(manifest.slug_rule, None);
        assert_eq!(check(&manifest, &schema_map(schemas)), Ok(()));
    }

    /// The on-disk key spelling is part of the contract: `slug-rule: { version, hash }`
    /// deserializes into the model (both shipped manifests declare it, and
    /// `deny_unknown_fields` means a key/field drift breaks *both* packs loudly).
    #[test]
    fn slug_rule_block_deserializes_from_the_on_disk_spelling() {
        let yaml = "\
slug-rule:
  version: 1
  hash: deadbeef
doctypes: []
";
        let manifest: Manifest = serde_yaml_ng::from_str(yaml).expect("the manifest parses");
        assert_eq!(
            manifest.slug_rule,
            Some(SlugRule {
                version: 1,
                hash: "deadbeef".into()
            })
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
