//! The **manifest-hash fence** — the comparator (M48 Increment 11, T1).
//!
//! The freeze is *declared* by both `schema-manifest.yaml` files and *enforced* at
//! pack-load by `engine::manifest::check` — but that assert only ever sees **one**
//! state of the tree, so it cannot tell a legitimate re-pin from a silent freeze
//! breach: *"re-pin the hash"* is one keystroke with two meanings, and the wrong
//! one ships an un-migrated structural change past every gate at exit 0
//! (`design/corpus-migration.md` → The freeze — declared *and* enforced → **the
//! successor rule, stated forward**; `design/storage.md` → Identity → *The slug
//! rule is itself a versioned rule*).
//!
//! This suite is the mechanism that makes the successor rule checkable: a **pure
//! function over two manifest texts** — the base of the pushed range and the
//! working copy — emitting one violation per entity whose **hash moved while its
//! co-located version stayed equal** (`schema-hash` with `schema-version` per
//! doctype; `slug-rule.hash` with `slug-rule.version`).
//!
//! **No product surface ships.** `Manifest`, `ManifestEntry` and `SlugRule` are
//! already `pub` with `Deserialize`, so the comparator lives entirely here — no
//! engine or CLI function is added for it
//! (`completions/artifacts/M48/settle-record.md` → The manifest-hash fence).
//!
//! **The verdict axis is a table, not a list of remembered cases** — `verdict::…`
//! carries one named arm per cell, and the `hash-moved × version-moved` product is
//! enumerated mechanically so a cell cannot be added without a row:
//!
//! | cell | verdict |
//! |---|---|
//! | hash moved, version equal | **violation** |
//! | hash moved, version moved | clean |
//! | version moved, hash equal | **clean, declared** — the successor rule is one-directional; re-declaring an identical shape at a new version is not this fence's subject |
//! | neither moved | clean |
//! | entity absent at base (added) | clean |
//! | entity absent at head (removed) | clean — strict set-equality is the pack-load assert's job |
//! | manifest absent at base | clean — nothing to compare |
//! | manifest text unparseable | **violation, fail closed** (either side; this wave's re-settled F3 precedent) |
//!
//! A violation names **the manifest path and the entity**: a bare count is
//! unactionable across two files, and the named escape a later task adds is
//! per-entity, so the entity is the identity the report is keyed by.

use engine::manifest::Manifest;
use std::collections::BTreeMap;
use std::fmt;

/// The dev pack's frozen doctype-set manifest — one of the two texts the fence
/// compares.
pub const DEV_MANIFEST: &str = "crates/cli/pack/config/schema-manifest.yaml";

/// The methodology pack's frozen doctype-set manifest — the second (M40 A1: each
/// pack declares its own frozen set, so a fence over only one is half a fence).
pub const METHODOLOGY_MANIFEST: &str = "packs/methodology/config/schema-manifest.yaml";

/// The entity name the `slug-rule:` singleton is reported and escaped under — the
/// one frozen entity that is not a doctype.
pub const SLUG_RULE_ENTITY: &str = "slug-rule";

/// Which of the two compared texts a parse failure came from.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Side {
    /// The base of the pushed range — the recorded prior state.
    Base,
    /// The working copy — the state being pushed.
    Head,
}

impl fmt::Display for Side {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Side::Base => f.write_str("base"),
            Side::Head => f.write_str("head"),
        }
    }
}

/// One reason a manifest diff fails the fence.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Violation {
    /// A frozen entity's hash moved while its **co-located** version stayed equal —
    /// the silent freeze breach the fence exists to catch.
    HashMovedWithoutVersion {
        /// The repo-relative manifest path the entity is declared in.
        manifest: String,
        /// The entity: a doctype name, or [`SLUG_RULE_ENTITY`].
        entity: String,
        /// The version that did **not** move.
        version: u32,
        /// The hash at the base of the pushed range.
        base_hash: String,
        /// The hash in the working copy.
        head_hash: String,
    },
    /// A manifest text did not deserialize. **Fail closed:** an unreadable prior
    /// state is not evidence of a clean one.
    Unparseable {
        /// The repo-relative manifest path.
        manifest: String,
        /// Which side failed to parse.
        side: Side,
        /// The deserializer's own message.
        error: String,
    },
}

impl fmt::Display for Violation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Violation::HashMovedWithoutVersion {
                manifest,
                entity,
                version,
                base_hash,
                head_hash,
            } => write!(
                f,
                "{manifest}: `{entity}` — hash moved ({base_hash} -> {head_hash}) \
                 while its version stayed {version}: a frozen shape moves only \
                 together with its co-located version",
            ),
            Violation::Unparseable {
                manifest,
                side,
                error,
            } => write!(
                f,
                "{manifest}: the {side} text does not parse as a freeze manifest \
                 ({error}): the fence fails closed rather than reporting clean",
            ),
        }
    }
}

/// One entity's frozen pin: its version and the hash frozen at that version.
#[derive(Clone, Debug, PartialEq, Eq)]
struct Pin {
    version: u32,
    hash: String,
}

/// Every frozen entity a manifest declares, keyed by entity name — the doctypes by
/// their `type`, plus the `slug-rule` singleton when declared.
///
/// A `BTreeMap` keyed by the entity name, so the comparison (and therefore the
/// emitted report) is **order-invariant** over declaration order in either text.
fn pins(manifest: &Manifest) -> BTreeMap<String, Pin> {
    let mut out: BTreeMap<String, Pin> = manifest
        .doctypes
        .iter()
        .map(|e| {
            (
                e.ty.clone(),
                Pin {
                    version: e.schema_version,
                    hash: e.schema_hash.clone(),
                },
            )
        })
        .collect();
    if let Some(rule) = &manifest.slug_rule {
        out.insert(
            SLUG_RULE_ENTITY.to_string(),
            Pin {
                version: rule.version,
                hash: rule.hash.clone(),
            },
        );
    }
    out
}

/// Compare one manifest's base text against its head text.
///
/// `base: None` is the *manifest absent at base* cell — a manifest that did not
/// exist there declares no prior pin, so there is nothing to compare.
pub fn compare_manifest(manifest: &str, base: Option<&str>, head: &str) -> Vec<Violation> {
    let Some(base) = base else {
        return Vec::new();
    };
    let base = match serde_yaml_ng::from_str::<Manifest>(base) {
        Ok(m) => m,
        Err(e) => return vec![unparseable(manifest, Side::Base, &e)],
    };
    let head = match serde_yaml_ng::from_str::<Manifest>(head) {
        Ok(m) => m,
        Err(e) => return vec![unparseable(manifest, Side::Head, &e)],
    };

    let head_pins = pins(&head);
    pins(&base)
        .into_iter()
        .filter_map(|(entity, was)| {
            // An entity absent on either side is clean: nothing to have moved from
            // (added), and strict set-equality is the pack-load assert's job
            // (removed).
            let now = head_pins.get(&entity)?;
            (now.hash != was.hash && now.version == was.version).then(|| {
                Violation::HashMovedWithoutVersion {
                    manifest: manifest.to_string(),
                    entity,
                    version: was.version,
                    base_hash: was.hash,
                    head_hash: now.hash.clone(),
                }
            })
        })
        .collect()
}

/// The fail-closed violation for a text that does not deserialize.
fn unparseable(manifest: &str, side: Side, error: &serde_yaml_ng::Error) -> Violation {
    Violation::Unparseable {
        manifest: manifest.to_string(),
        side,
        error: error.to_string(),
    }
}

/// Compare every `(manifest path, base text, head text)` the caller hands over —
/// the fence's whole subject is **both** manifests, so violations from either must
/// reach the report, each naming its own file.
pub fn compare_all(manifests: &[(&str, Option<&str>, &str)]) -> Vec<Violation> {
    manifests
        .iter()
        .flat_map(|(path, base, head)| compare_manifest(path, *base, head))
        .collect()
}

/// Render a violation set the way a report does — the byte form the order-invariance
/// arm compares.
fn rendered(violations: &[Violation]) -> String {
    violations
        .iter()
        .map(Violation::to_string)
        .collect::<Vec<_>>()
        .join("\n")
}

/// Fabricate a manifest text: an optional `slug-rule:` block plus doctype entries in
/// the declaration order given (the order is an input, so the order-invariance arm
/// can feed the same set two ways).
fn manifest_text(slug: Option<(u32, &str)>, doctypes: &[(&str, u32, &str)]) -> String {
    let mut out = String::new();
    if let Some((version, hash)) = slug {
        out.push_str(&format!(
            "slug-rule:\n  version: {version}\n  hash: {hash}\n"
        ));
    }
    if doctypes.is_empty() {
        out.push_str("doctypes: []\n");
        return out;
    }
    out.push_str("doctypes:\n");
    for (ty, version, hash) in doctypes {
        out.push_str(&format!(
            "  - type: {ty}\n    schema-version: {version}\n    schema-hash: {hash}\n"
        ));
    }
    out
}

const HASH_A: &str = "1111111111111111111111111111111111111111111111111111111111111111";
const HASH_B: &str = "2222222222222222222222222222222222222222222222222222222222222222";
const SLUG_HASH_A: &str = "3333333333333333333333333333333333333333333333333333333333333333";
const SLUG_HASH_B: &str = "4444444444444444444444444444444444444444444444444444444444444444";

/// A manifest declaring the slug rule at `(3, SLUG_HASH_A)` and nothing else moved —
/// the neutral backdrop the doctype arms vary one entity against.
fn with_doctypes(doctypes: &[(&str, u32, &str)]) -> String {
    manifest_text(Some((3, SLUG_HASH_A)), doctypes)
}

/// A manifest declaring one unmoving doctype and the slug rule as given — the
/// backdrop the slug-rule arms vary the singleton against.
fn with_slug(version: u32, hash: &str) -> String {
    manifest_text(Some((version, hash)), &[("adr", 2, HASH_A)])
}

/// The fence's verdict for one compared entity.
#[derive(Debug, PartialEq, Eq)]
enum Verdict {
    Clean,
    Violation,
}

/// The verdict table over the `hash-moved × version-moved` product, as an
/// **exhaustive match** — the product is enumerated by the arms below, so a cell
/// cannot be added without a row here and a row cannot be silently dropped.
fn expected(hash_moved: bool, version_moved: bool) -> Verdict {
    match (hash_moved, version_moved) {
        // The subject of the fence: a shape moved with nothing declaring it.
        (true, false) => Verdict::Violation,
        // A declared, versioned shape change — exactly what the freeze asks for.
        (true, true) => Verdict::Clean,
        // CLEAN, DECLARED. The successor rule is one-directional: it constrains a
        // moving hash, not a moving version. Re-declaring an identical shape at a
        // new version is not this fence's subject and is not invented into one.
        (false, true) => Verdict::Clean,
        // Nothing moved.
        (false, false) => Verdict::Clean,
    }
}

fn verdict_of(violations: &[Violation]) -> Verdict {
    if violations.is_empty() {
        Verdict::Clean
    } else {
        Verdict::Violation
    }
}

mod verdict {
    use super::*;

    /// The doctype entity — `schema-hash` against its co-located `schema-version`.
    mod doctype {
        use super::*;

        #[test]
        fn hash_moved_version_equal_is_a_violation() {
            let base = with_doctypes(&[("adr", 2, HASH_A)]);
            let head = with_doctypes(&[("adr", 2, HASH_B)]);
            let found = compare_manifest(DEV_MANIFEST, Some(&base), &head);
            assert_eq!(
                found,
                vec![Violation::HashMovedWithoutVersion {
                    manifest: DEV_MANIFEST.to_string(),
                    entity: "adr".to_string(),
                    version: 2,
                    base_hash: HASH_A.to_string(),
                    head_hash: HASH_B.to_string(),
                }],
                "a doctype hash that moves at an unchanged schema-version is the \
                 silent freeze breach this fence exists to catch",
            );
        }

        #[test]
        fn hash_moved_version_moved_is_clean() {
            let base = with_doctypes(&[("adr", 2, HASH_A)]);
            let head = with_doctypes(&[("adr", 3, HASH_B)]);
            assert_eq!(
                compare_manifest(DEV_MANIFEST, Some(&base), &head),
                vec![],
                "a hash moving together with its co-located schema-version is the \
                 declared, versioned change the freeze asks for",
            );
        }

        #[test]
        fn version_moved_hash_equal_is_clean_declared() {
            let base = with_doctypes(&[("adr", 2, HASH_A)]);
            let head = with_doctypes(&[("adr", 3, HASH_A)]);
            assert_eq!(
                compare_manifest(DEV_MANIFEST, Some(&base), &head),
                vec![],
                "the successor rule is one-directional — it constrains a moving hash, \
                 not a moving version",
            );
        }

        #[test]
        fn neither_moved_is_clean() {
            let base = with_doctypes(&[("adr", 2, HASH_A)]);
            let head = with_doctypes(&[("adr", 2, HASH_A)]);
            assert_eq!(compare_manifest(DEV_MANIFEST, Some(&base), &head), vec![]);
        }

        #[test]
        fn absent_at_base_is_clean() {
            let base = with_doctypes(&[("adr", 2, HASH_A)]);
            let head = with_doctypes(&[("adr", 2, HASH_A), ("idea", 1, HASH_B)]);
            assert_eq!(
                compare_manifest(DEV_MANIFEST, Some(&base), &head),
                vec![],
                "an entity added in this range has no prior pin to have moved from",
            );
        }

        #[test]
        fn absent_at_head_is_clean() {
            let base = with_doctypes(&[("adr", 2, HASH_A), ("idea", 1, HASH_B)]);
            let head = with_doctypes(&[("adr", 2, HASH_A)]);
            assert_eq!(
                compare_manifest(DEV_MANIFEST, Some(&base), &head),
                vec![],
                "a removed entity is the pack-load assert's strict set-equality job, \
                 not this fence's",
            );
        }

        #[test]
        fn the_move_product_is_a_table_with_no_unnamed_cell() {
            for hash_moved in [false, true] {
                for version_moved in [false, true] {
                    let base = with_doctypes(&[("adr", 2, HASH_A)]);
                    let head = with_doctypes(&[(
                        "adr",
                        if version_moved { 3 } else { 2 },
                        if hash_moved { HASH_B } else { HASH_A },
                    )]);
                    let found = compare_manifest(DEV_MANIFEST, Some(&base), &head);
                    assert_eq!(
                        verdict_of(&found),
                        expected(hash_moved, version_moved),
                        "doctype cell (hash_moved={hash_moved}, version_moved={version_moved}) \
                         must match the table; found {found:?}",
                    );
                }
            }
        }
    }

    /// The `slug-rule:` singleton — `hash` against its co-located `version`. The same
    /// axis, a different pair of keys: a fence that walks only the doctype list is
    /// blind to the one rule that names every id in every corpus.
    mod slug_rule {
        use super::*;

        #[test]
        fn hash_moved_version_equal_is_a_violation() {
            let base = with_slug(3, SLUG_HASH_A);
            let head = with_slug(3, SLUG_HASH_B);
            assert_eq!(
                compare_manifest(METHODOLOGY_MANIFEST, Some(&base), &head),
                vec![Violation::HashMovedWithoutVersion {
                    manifest: METHODOLOGY_MANIFEST.to_string(),
                    entity: SLUG_RULE_ENTITY.to_string(),
                    version: 3,
                    base_hash: SLUG_HASH_A.to_string(),
                    head_hash: SLUG_HASH_B.to_string(),
                }],
                "a slug-rule change cannot be migrated after the fact — it is a \
                 declared event or it is a corpus split",
            );
        }

        #[test]
        fn hash_moved_version_moved_is_clean() {
            let base = with_slug(3, SLUG_HASH_A);
            let head = with_slug(4, SLUG_HASH_B);
            assert_eq!(
                compare_manifest(METHODOLOGY_MANIFEST, Some(&base), &head),
                vec![],
            );
        }

        #[test]
        fn version_moved_hash_equal_is_clean_declared() {
            let base = with_slug(3, SLUG_HASH_A);
            let head = with_slug(4, SLUG_HASH_A);
            assert_eq!(
                compare_manifest(METHODOLOGY_MANIFEST, Some(&base), &head),
                vec![],
            );
        }

        #[test]
        fn neither_moved_is_clean() {
            let base = with_slug(3, SLUG_HASH_A);
            let head = with_slug(3, SLUG_HASH_A);
            assert_eq!(
                compare_manifest(METHODOLOGY_MANIFEST, Some(&base), &head),
                vec![],
            );
        }

        #[test]
        fn absent_at_base_is_clean() {
            let base = manifest_text(None, &[("adr", 2, HASH_A)]);
            let head = with_slug(3, SLUG_HASH_A);
            assert_eq!(
                compare_manifest(METHODOLOGY_MANIFEST, Some(&base), &head),
                vec![],
                "a slug rule first declared in this range has no prior pin",
            );
        }

        #[test]
        fn absent_at_head_is_clean() {
            let base = with_slug(3, SLUG_HASH_A);
            let head = manifest_text(None, &[("adr", 2, HASH_A)]);
            assert_eq!(
                compare_manifest(METHODOLOGY_MANIFEST, Some(&base), &head),
                vec![],
                "an undeclared slug rule blocks at pack-load — this fence does not \
                 duplicate that verdict",
            );
        }

        #[test]
        fn the_move_product_is_a_table_with_no_unnamed_cell() {
            for hash_moved in [false, true] {
                for version_moved in [false, true] {
                    let base = with_slug(3, SLUG_HASH_A);
                    let head = with_slug(
                        if version_moved { 4 } else { 3 },
                        if hash_moved { SLUG_HASH_B } else { SLUG_HASH_A },
                    );
                    let found = compare_manifest(METHODOLOGY_MANIFEST, Some(&base), &head);
                    assert_eq!(
                        verdict_of(&found),
                        expected(hash_moved, version_moved),
                        "slug-rule cell (hash_moved={hash_moved}, version_moved={version_moved}) \
                         must match the table; found {found:?}",
                    );
                }
            }
        }
    }

    /// The cells about the *texts* rather than the entities inside them.
    mod presence {
        use super::*;

        #[test]
        fn manifest_absent_at_base_is_clean() {
            let head = with_doctypes(&[("adr", 2, HASH_A)]);
            assert_eq!(
                compare_manifest(DEV_MANIFEST, None, &head),
                vec![],
                "a manifest that did not exist at the base declares no prior pin",
            );
        }

        #[test]
        fn base_text_unparseable_is_a_violation_fail_closed() {
            let head = with_doctypes(&[("adr", 2, HASH_A)]);
            let found = compare_manifest(DEV_MANIFEST, Some("doctypes: [ {{{"), &head);
            assert_eq!(found.len(), 1, "found {found:?}");
            match &found[0] {
                Violation::Unparseable {
                    manifest,
                    side,
                    error,
                } => {
                    assert_eq!(manifest, DEV_MANIFEST);
                    assert_eq!(*side, Side::Base);
                    assert!(!error.is_empty(), "the deserializer's own message is kept");
                }
                other => panic!("an unparseable base must fail closed, got {other:?}"),
            }
        }

        #[test]
        fn head_text_unparseable_is_a_violation_fail_closed() {
            let base = with_doctypes(&[("adr", 2, HASH_A)]);
            let found = compare_manifest(DEV_MANIFEST, Some(&base), "doctypes: [ {{{");
            assert_eq!(found.len(), 1, "found {found:?}");
            match &found[0] {
                Violation::Unparseable { side, .. } => assert_eq!(*side, Side::Head),
                other => panic!("an unparseable head must fail closed, got {other:?}"),
            }
        }

        #[test]
        fn an_unknown_key_is_unparseable_not_ignored() {
            let head = with_doctypes(&[("adr", 2, HASH_A)]);
            let base = format!("{}surprise: 1\n", with_doctypes(&[("adr", 2, HASH_A)]));
            let found = compare_manifest(DEV_MANIFEST, Some(&base), &head);
            assert!(
                matches!(found.as_slice(), [Violation::Unparseable { .. }]),
                "the manifest model denies unknown fields; a text the freeze gate \
                 itself would reject is not a trustworthy prior state — found {found:?}",
            );
        }
    }

    /// What the report has to say to be actionable, and the properties of the set it
    /// emits.
    mod reporting {
        use super::*;

        #[test]
        fn a_violation_names_the_manifest_path_and_the_entity() {
            let base = with_doctypes(&[("adr", 2, HASH_A)]);
            let head = with_doctypes(&[("adr", 2, HASH_B)]);
            let text = rendered(&compare_manifest(DEV_MANIFEST, Some(&base), &head));
            assert!(text.contains(DEV_MANIFEST), "names its manifest: {text}");
            assert!(text.contains("`adr`"), "names its entity: {text}");
        }

        #[test]
        fn both_manifests_are_compared_and_each_violation_names_its_own_file() {
            let dev_base = with_doctypes(&[("adr", 2, HASH_A)]);
            let dev_head = with_doctypes(&[("adr", 2, HASH_B)]);
            let meth_base = with_slug(3, SLUG_HASH_A);
            let meth_head = with_slug(3, SLUG_HASH_B);
            let found = compare_all(&[
                (DEV_MANIFEST, Some(&dev_base), &dev_head),
                (METHODOLOGY_MANIFEST, Some(&meth_base), &meth_head),
            ]);
            assert_eq!(found.len(), 2, "found {found:?}");
            let text = rendered(&found);
            assert!(text.contains(DEV_MANIFEST), "{text}");
            assert!(text.contains(METHODOLOGY_MANIFEST), "{text}");
            assert!(text.contains("`adr`"), "{text}");
            assert!(text.contains("`slug-rule`"), "{text}");
        }

        /// The report is keyed by entity, not by declaration order: the same set of
        /// moved pins fed in **two divergent orders** must render byte-identically,
        /// or a manifest reshuffle would churn the fence's output and hide which
        /// entity actually moved.
        #[test]
        fn the_report_is_byte_identical_under_divergent_declaration_orders() {
            let forward = [("adr", 2, HASH_A), ("changelog", 2, HASH_A)];
            let reverse = [("changelog", 2, HASH_A), ("adr", 2, HASH_A)];
            let forward_head = [("adr", 2, HASH_B), ("changelog", 2, HASH_B)];
            let reverse_head = [("changelog", 2, HASH_B), ("adr", 2, HASH_B)];

            let in_order = compare_manifest(
                DEV_MANIFEST,
                Some(&manifest_text(Some((3, SLUG_HASH_A)), &forward)),
                &manifest_text(Some((3, SLUG_HASH_B)), &forward_head),
            );
            let reversed = compare_manifest(
                DEV_MANIFEST,
                Some(&manifest_text(Some((3, SLUG_HASH_A)), &reverse)),
                &manifest_text(Some((3, SLUG_HASH_B)), &reverse_head),
            );

            assert_eq!(
                in_order.len(),
                3,
                "the fixture must genuinely overlap — three entities moved: {in_order:?}",
            );
            assert_eq!(
                rendered(&in_order),
                rendered(&reversed),
                "the same moved set must render byte-identically under either \
                 declaration order",
            );
        }
    }
}
