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
use crate::schema::{Leaf, Schema, SectionBody};
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};
use thiserror::Error;

/// The deterministic **schema-hash** of a doctype definition: the lowercase-hex
/// `blake3` digest of the canonical JSON of its **presentation projection**
/// ([`erase_presentation`]).
///
/// [`Schema`] (and its whole transitive model) carries only ordered `Vec`s — no
/// maps — so `serde_json::to_vec` emits the same bytes every run, making the hash
/// a stable fingerprint of the schema's *shape*. Serialization of the schema
/// model is infallible (no `Serialize` impl in the model can error), so the
/// (unreachable) error is surfaced as a panic rather than silently swallowed.
///
/// **The hash is taken over the projection, not the whole struct (M47).** Three
/// authored-prose keys — `Schema.description`, `Schema.usage`, `Slot.hint` — are
/// erased first, so a typo fix in an authoring hint is not a frozen-schema event.
/// Everything else stays in, the six *semantics* keys (`default` / `set` /
/// `inverse` / `inverse-card` / `check` / `title-names-symbol`) included: they
/// change what the tool **writes and adjudicates**, which is a declared contract
/// change even though no committed doc's bytes can violate it.
pub fn schema_hash(schema: &Schema) -> String {
    let projected = erase_presentation(schema);
    let bytes = serde_json::to_vec(&projected).expect("Schema serializes to JSON infallibly");
    hash_bytes(&bytes)
}

/// The schema's **presentation projection** — a clone with the three authored-prose
/// keys erased: `Schema.description`, `Schema.usage`, and every `Slot.hint` (at both
/// loci — a simple section's slot **and** a slot leaf inside a repeatable item block,
/// recursing into nested repeatables).
///
/// **An erase-list, deliberately — and deliberately *not* a call into
/// [`crate::schema_diff`]'s `erase_out_of_projection`.** The two functions answer two
/// different questions: *can a committed doc's bytes violate this?* (the migration
/// classifier, whose erase set is designed to **grow**) versus *is this a declared
/// change to the contract?* (this one). Reusing the classifier's set would stop the
/// hash moving on `default:` / `set:` — keys that change what the tool writes into new
/// documents — and would silently narrow the freeze every time that set grew.
///
/// The `Schema` destructure below is **exhaustive on purpose**: a field added to the
/// model breaks this build rather than silently falling out of the hash — the safe
/// direction (a compile error the author sees) instead of a silent freeze narrowing
/// nobody sees (`DECISIONS.md` → 2026-07-26 the Settle, Decision 3 + the pre-decompose
/// review; `design/corpus-migration.md` → The freeze).
fn erase_presentation(schema: &Schema) -> Schema {
    // Exhaustive: every `Schema` field is named here, so adding one is a build error.
    let Schema {
        ty,
        location,
        id_from,
        description: _,
        usage: _,
        display_title,
        placement,
        singleton,
        sections,
    } = schema;
    let mut out = Schema {
        ty: ty.clone(),
        location: location.clone(),
        id_from: id_from.clone(),
        description: None,
        usage: None,
        display_title: display_title.clone(),
        placement: placement.clone(),
        singleton: *singleton,
        sections: sections.clone(),
    };
    for section in &mut out.sections {
        match &mut section.body {
            SectionBody::Simple { slot, .. } => {
                if let Some(slot) = slot {
                    slot.hint = None;
                }
            }
            SectionBody::Repeatable { repeatable } => erase_block_hints(&mut repeatable.block),
        }
    }
    out
}

/// Erase the slot hints of a repeatable item block's leaves — the **second locus**
/// (the item block is where the work-doc family's prose lives), recursing into a
/// nested repeatable so no slot escapes the projection at any depth.
fn erase_block_hints(block: &mut [Leaf]) {
    for leaf in block {
        match leaf {
            Leaf::Slot { slot, .. } => slot.hint = None,
            Leaf::Repeatable { repeatable, .. } => erase_block_hints(&mut repeatable.block),
            Leaf::Field(_) => {}
        }
    }
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
    /// The declared slug rule. **Required** — an absent block is
    /// [`ManifestError::SlugRuleUndeclared`], not a silent opt-out (M42 audit): a
    /// manifest that freezes doctype *shapes* is held to the rule the ids inside them
    /// are *minted* by. `Option` in the *model* only, so the omission surfaces as that
    /// routed error rather than an unroutable parse failure — and so this struct's
    /// `deny_unknown_fields` (which already blocks a *misspelled* key) does not reward
    /// the omission it punishes the typo for. Opting out of the freeze stays a
    /// pack-level, wholesale act: ship no `schema-manifest.yaml`.
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

    /// The manifest declares **no** `slug-rule:` block at all — the *third* silencer,
    /// and the cheapest one. [`SlugRuleHashMismatch`](ManifestError::SlugRuleHashMismatch)
    /// catches the drift and [`SlugRuleVersionMismatch`](ManifestError::SlugRuleVersionMismatch)
    /// stops a re-pinned hash from buying silence — but *deleting three lines* bought
    /// the same silence outright, and a third-party manifest authored without the key
    /// inherited no fence at all.
    ///
    /// Opting out of the freeze is a **pack-level, wholesale** act (ship no
    /// `schema-manifest.yaml` — that pack is skipped entirely). A manifest that freezes
    /// doctype *shapes* may not quietly decline to declare the rule the ids inside
    /// those doctypes are *minted* by: that is the one change no migration can repair,
    /// so an absent declaration is precisely the case that must block rather than wave
    /// through. The route carries the block to paste.
    #[error(
        "the freeze manifest declares no `slug-rule:` block — a manifest that freezes doctype shapes must also declare the identity-mint rule the ids inside them are derived by (`engine::slug::slugify`), because a slug-rule change cannot be migrated after the fact: it splits the corpus into two permanent id generations. Declare the rule this engine ships:\n\nslug-rule:\n  version: {shipped_version}\n  hash: {shipped_hash}\n\n(the values of `engine::slug::SLUG_RULE_VERSION` + `engine::slug::rule_fingerprint()`; a pack that ships no `config/schema-manifest.yaml` at all opts out of the freeze entirely)"
    )]
    SlugRuleUndeclared {
        /// The slug-rule version the engine ships — the value the manifest owes.
        shipped_version: u32,
        /// The fingerprint the engine recomputed — the value the manifest owes.
        shipped_hash: String,
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
/// one no migration can repair). Its declaration is **required** — an absent
/// `slug-rule:` block blocks ([`ManifestError::SlugRuleUndeclared`]); a pack opts out
/// of the freeze by shipping no manifest, never by omitting a key from one.
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
/// `None` (no `slug-rule:` block) is the **third** arm and blocks too
/// ([`ManifestError::SlugRuleUndeclared`]) — otherwise the two arms above are
/// silenceable by deleting them, and a manifest authored without the key inherits no
/// fence at all. The pack-level opt-out (ship no manifest) is untouched; a manifest
/// that freezes doctype shapes is held to the rule that mints the ids inside them.
fn check_slug_rule(declared: Option<&SlugRule>) -> Result<(), ManifestError> {
    let Some(rule) = declared else {
        return Err(ManifestError::SlugRuleUndeclared {
            shipped_version: crate::slug::SLUG_RULE_VERSION,
            shipped_hash: crate::slug::rule_fingerprint().to_string(),
        });
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
    use crate::schema::{Field, Placement, Slot, load_schema};

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

    /// A fixture carrying **every prose locus** the hash projection erases — the
    /// doctype-level `description:` / `usage:`, a simple section's slot `hint:`, an
    /// item-block slot `hint:`, and a **nested** item-block slot `hint:` — alongside
    /// the semantics keys the projection keeps (`of` / `default` / `set` / `card` /
    /// `inverse` / `inverse-card` / `optional`).
    const PROSE_YAML: &str = "\
type: prosey
location: prosey/
id-from: title
description: A prosey doctype.
usage: Reach for it sometimes.
sections:
  - id: header
    header: true
    fields:
      - { id: title, type: string }
      - { id: status, type: enum, of: [open, closed], default: open }
      - { id: stamped, type: date, set: on-create }
      - id: supersedes
        type: ref
        to: prosey
        card: \"0..1\"
        inverse: superseded-by
        inverse-card: \"0..1\"
  - id: body
    slot: { hint: \"What the prosey doc says.\" }
  - id: parts
    repeatable:
      id-from: name
      block:
        - { id: name, type: string }
        - id: notes
          slot: { hint: \"Notes on the part.\" }
        - id: subparts
          repeatable:
            id-from: label
            block:
              - { id: label, type: string }
              - id: detail
                slot: { hint: \"Detail of the subpart.\" }
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

    /// A **well-formed** manifest: entries' hashes match the supplied schemas
    /// (version 1), and the slug rule the engine actually ships is declared — the
    /// shape every production manifest carries, and the only shape [`check`] passes.
    fn manifest_for(schemas: &[Schema]) -> Manifest {
        Manifest {
            slug_rule: Some(SlugRule {
                version: crate::slug::SLUG_RULE_VERSION,
                hash: crate::slug::rule_fingerprint().to_string(),
            }),
            ..manifest_without_slug_rule(schemas)
        }
    }

    /// The same manifest with the `slug-rule:` block **omitted** — the shape that
    /// used to be silently unchecked and now blocks
    /// ([`absent_slug_rule_block_blocks_loudly`]).
    fn manifest_without_slug_rule(schemas: &[Schema]) -> Manifest {
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

    #[test]
    fn matching_set_passes() {
        let schemas = vec![widget(), gadget()];
        let manifest = manifest_for(&schemas);
        assert_eq!(check(&manifest, &schema_map(schemas)), Ok(()));
    }

    /// A real **shape** drift with no version bump blocks.
    ///
    /// The mutation is a widened `enum` — a genuine structural change a committed doc
    /// can be adjudicated against. **It used to be a slot-`hint` reword**, which is
    /// exactly the conflation M47's presentation projection ends: this test claimed to
    /// witness "a schema-shape change" while its dev-pack sibling eight lines up drifted
    /// `location:`, and once prose left the hash the old mutation stopped moving it at
    /// all. The invariance half is its own arm ([`a_prose_reword_passes_the_freeze_check`]).
    #[test]
    fn shape_mutated_without_bump_is_hash_mismatch() {
        let manifest = manifest_for(&[widget(), gadget()]);
        let expected = schema_hash(&widget());

        // Mutate the widget's *shape* (widen its `status` enum) without bumping its version.
        let mut mutated = widget();
        if let crate::schema::SectionBody::Simple { fields, .. } = &mut mutated.sections[0].body {
            let status = fields
                .iter_mut()
                .find(|f| f.id == "status")
                .expect("the fixture header declares `status`");
            status.of = Some(vec!["open".into(), "closed".into(), "parked".into()]);
        } else {
            panic!("fixture header section is simple");
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

    /// The **invariance arm beside it**: the very mutation
    /// [`shape_mutated_without_bump_is_hash_mismatch`] used to make — a slot-`hint`
    /// reword — now passes the freeze check against an **unbumped, un-re-pinned**
    /// manifest. That is the M47 deliverable at the gate: a typo fix in an authoring
    /// hint is no longer a frozen-schema event, so the freeze becomes fenceable (the
    /// two changes stop being the same keystroke).
    #[test]
    fn a_prose_reword_passes_the_freeze_check() {
        let manifest = manifest_for(&[widget(), gadget()]);

        let mut reworded = widget();
        if let crate::schema::SectionBody::Simple { slot, .. } = &mut reworded.sections[1].body {
            slot.as_mut().expect("the fixture body has a slot").hint =
                Some("a different hint".into());
        } else {
            panic!("fixture body section is simple");
        }
        assert_ne!(
            reworded,
            widget(),
            "the reword must really change the schema"
        );

        assert_eq!(
            check(&manifest, &schema_map([reworded, gadget()])),
            Ok(()),
            "a prose reword must pass the freeze check with no bump and no re-pin",
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
        let manifest = manifest_for(&schemas);
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
        let mut manifest = manifest_for(&schemas);
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
        let mut manifest = manifest_for(&schemas);
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

    /// The **third silencer**, and the one the other two arms left open: a manifest
    /// that simply **omits** the `slug-rule:` block must block loudly, not wave
    /// through. The hash arm catches a drift and the version arm stops a re-pinned
    /// hash from buying silence — but *deleting three lines* bought the same silence
    /// more cheaply, and it is the cheapest "fix" an agent staring at a
    /// [`SlugRuleHashMismatch`](ManifestError::SlugRuleHashMismatch) can reach for.
    /// A pack opts out of the freeze by shipping **no manifest**; a manifest that
    /// freezes doctype shapes is held to the rule the ids inside them are minted by.
    #[test]
    fn absent_slug_rule_block_blocks_loudly() {
        let schemas = vec![widget(), gadget()];
        let manifest = manifest_without_slug_rule(&schemas);
        assert_eq!(manifest.slug_rule, None);

        let err =
            check(&manifest, &schema_map(schemas)).expect_err("an undeclared slug rule must block");
        assert_eq!(
            err,
            ManifestError::SlugRuleUndeclared {
                shipped_version: crate::slug::SLUG_RULE_VERSION,
                shipped_hash: crate::slug::rule_fingerprint().to_string(),
            }
        );

        // The route must be *actionable*: name the key, the version, and the hash —
        // the pack author reading stderr has the block to paste, not a research task.
        let msg = err.to_string();
        assert!(
            msg.contains("slug-rule:"),
            "the message must name the missing key; got: {msg}"
        );
        assert!(
            msg.contains(&format!("version: {}", crate::slug::SLUG_RULE_VERSION)),
            "the message must name the shipped version; got: {msg}"
        );
        assert!(
            msg.contains(crate::slug::rule_fingerprint()),
            "the message must name the shipped fingerprint; got: {msg}"
        );
    }

    /// The **omission is not a typo**: `deny_unknown_fields` already blocks a
    /// *misspelled* `slug_rule:` key loudly. Before this fix the strict-shape posture
    /// punished the typo and rewarded the omission — the asymmetry that made "absent =
    /// unchecked" incoherent. Both now block; this pins the pair.
    #[test]
    fn a_misspelled_and_an_omitted_slug_rule_key_both_block() {
        // Misspelled → a loud *parse* failure (deny_unknown_fields).
        let misspelled = "\
slug_rule:
  version: 2
  hash: deadbeef
doctypes: []
";
        serde_yaml_ng::from_str::<Manifest>(misspelled)
            .expect_err("a misspelled slug-rule key must not deserialize");

        // Omitted → parses (the key is optional in the *model*), and blocks at `check`.
        let omitted: Manifest = serde_yaml_ng::from_str("doctypes: []\n").expect("parses");
        assert!(
            check(&omitted, &BTreeMap::new()).is_err(),
            "an omitted slug-rule block must block at the gate"
        );
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

    /// A named schema mutation: a label plus the edit it applies in place. Used by the
    /// two projection tests below as `(what, apply)` tables.
    type Mutation = (&'static str, fn(&mut Schema));

    fn prosey() -> Schema {
        load_schema(PROSE_YAML.as_bytes()).expect("prosey fixture loads")
    }

    /// The fixture's header field `id` — the locus every semantics-key mutation below
    /// reaches for.
    fn header_field<'a>(schema: &'a mut Schema, id: &str) -> &'a mut Field {
        match &mut schema.sections[0].body {
            SectionBody::Simple { fields, .. } => fields
                .iter_mut()
                .find(|f| f.id == id)
                .unwrap_or_else(|| panic!("the fixture header declares `{id}`")),
            SectionBody::Repeatable { .. } => panic!("the fixture header is a simple section"),
        }
    }

    /// The **first** slot-hint locus: a simple body section's slot.
    fn body_slot(schema: &mut Schema) -> &mut Slot {
        match &mut schema.sections[1].body {
            SectionBody::Simple { slot, .. } => slot.as_mut().expect("the fixture body has a slot"),
            SectionBody::Repeatable { .. } => panic!("the fixture body is a simple section"),
        }
    }

    /// The **second** slot-hint locus: a slot leaf inside a repeatable item block.
    fn item_slot(schema: &mut Schema) -> &mut Slot {
        match &mut schema.sections[2].body {
            SectionBody::Repeatable { repeatable } => match &mut repeatable.block[1] {
                Leaf::Slot { slot, .. } => slot,
                _ => panic!("the fixture item block's second leaf is a slot"),
            },
            SectionBody::Simple { .. } => panic!("the fixture `parts` section is repeatable"),
        }
    }

    /// The second locus **at depth**: a slot leaf inside a *nested* repeatable's block.
    fn nested_item_slot(schema: &mut Schema) -> &mut Slot {
        match &mut schema.sections[2].body {
            SectionBody::Repeatable { repeatable } => match &mut repeatable.block[2] {
                Leaf::Repeatable { repeatable, .. } => match &mut repeatable.block[1] {
                    Leaf::Slot { slot, .. } => slot,
                    _ => panic!("the nested block's second leaf is a slot"),
                },
                _ => panic!("the fixture item block's third leaf is a nested repeatable"),
            },
            SectionBody::Simple { .. } => panic!("the fixture `parts` section is repeatable"),
        }
    }

    /// The M47 Decision 3 headline — **the hash is a presentation projection**: a
    /// reword of authored *prose* leaves [`schema_hash`] byte-identical, so a typo fix
    /// in a `description:` / `usage:` / slot `hint:` is no longer a frozen-schema
    /// event. While prose sat inside the hash the freeze was **unfenceable**: a hint
    /// reword and an enum widening were the same keystroke — re-pin, green — guarded
    /// only by a comment (`DECISIONS.md` → 2026-07-26 the Settle, Decision 3).
    ///
    /// Every prose locus is swept, not the one being thought about: both doctype-level
    /// keys, a simple section's slot, an item-block slot, and a **nested** item-block
    /// slot.
    #[test]
    fn a_prose_reword_leaves_the_schema_hash_invariant() {
        let base = prosey();
        let baseline = schema_hash(&base);

        let rewords: [Mutation; 5] = [
            ("description", |s| {
                s.description = Some("A prosier doctype.".into())
            }),
            ("usage", |s| s.usage = Some("Reach for it often.".into())),
            ("body slot hint", |s| {
                body_slot(s).hint = Some("What the prosey doc states.".into())
            }),
            ("item-block slot hint", |s| {
                item_slot(s).hint = Some("Notes about the part.".into())
            }),
            ("nested item-block slot hint", |s| {
                nested_item_slot(s).hint = Some("Details of the subpart.".into())
            }),
        ];

        for (what, reword) in &rewords {
            let mut reworded = base.clone();
            reword(&mut reworded);
            assert_ne!(
                reworded, base,
                "the {what} reword must really change the schema (else the arm is vacuous)"
            );
            assert_eq!(
                schema_hash(&reworded),
                baseline,
                "rewording the {what} must leave the schema-hash invariant"
            );
        }

        // …and all five at once, so the invariance is not an artifact of one-at-a-time.
        let mut all = base.clone();
        for (_, reword) in &rewords {
            reword(&mut all);
        }
        assert_ne!(all, base);
        assert_eq!(
            schema_hash(&all),
            baseline,
            "rewording every prose locus at once must leave the schema-hash invariant"
        );
    }

    /// The projection's **other side**, swept key by key: three keys left the hash and
    /// **nothing else did**. The six semantics keys stay in deliberately — `default:` /
    /// `set:` change what the tool writes into new documents, and `inverse:` /
    /// `inverse-card:` / `check:` / `title-names-symbol:` change what it adjudicates —
    /// so they are declared contract changes even though no committed doc's bytes can
    /// violate them (the argument that kept `erase_presentation` a separate function
    /// from `schema_diff::erase_out_of_projection`, whose erase set is designed to grow).
    #[test]
    fn every_semantics_key_still_moves_the_schema_hash() {
        let base = prosey();
        let baseline = schema_hash(&base);

        let mutations: [Mutation; 14] = [
            ("default", |s| {
                header_field(s, "status").default = Some("closed".into())
            }),
            ("set", |s| {
                header_field(s, "stamped").set = Some("on-transition".into())
            }),
            ("inverse", |s| {
                header_field(s, "supersedes").inverse = Some("replaced-by".into())
            }),
            ("inverse-card", |s| {
                header_field(s, "supersedes").inverse_card = Some("0..*".into())
            }),
            ("check", |s| {
                header_field(s, "title").check = Some("symbol-exists".into())
            }),
            ("title-names-symbol", |s| {
                header_field(s, "title").title_names_symbol = true
            }),
            ("of", |s| {
                header_field(s, "status").of =
                    Some(vec!["open".into(), "closed".into(), "parked".into()])
            }),
            ("card", |s| {
                header_field(s, "supersedes").card = Some("0..*".into())
            }),
            ("optional (header field)", |s| {
                header_field(s, "status").optional = true
            }),
            ("optional (body slot)", |s| body_slot(s).optional = true),
            ("optional (item-block slot)", |s| {
                item_slot(s).optional = true
            }),
            ("location", |s| s.location = Some("prosier/".into())),
            ("placement", |s| {
                s.location = None;
                s.placement = Some(Placement {
                    file: "PROSEY.md".into(),
                });
            }),
            ("display-title", |s| s.display_title = Some("Prosey".into())),
        ];

        for (what, mutate) in mutations {
            let mut mutated = base.clone();
            mutate(&mut mutated);
            assert_ne!(
                schema_hash(&mutated),
                baseline,
                "a `{what}` change must still move the schema-hash"
            );
        }
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

        // A real *shape* drift, not a `description` reword: since M47 prose is outside
        // the hash, so the old mutation would leave `widget`'s hash intact and this test
        // would silently stop testing the tie-break its comment claims (it asserts
        // `ExtraEntry`, so it would have stayed green either way).
        let mut drifted = widget();
        drifted.location = Some("drifted-widgets/".into());

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
