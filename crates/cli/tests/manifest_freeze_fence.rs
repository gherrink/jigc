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
//! unactionable across two files, and the **named escape** (T2, `escape::…`) is
//! per-entity, so the entity is the identity the report is keyed by.
//!
//! **The escape is a table too.** A re-pin is sometimes legitimate, so a violation is
//! excused **iff** a commit message in the inspected range carries a line-leading
//! `Manifest-Repin:` trailer naming **that exact entity** — and `escape::SHAPES`
//! enumerates every shape the token can appear in against its verdict, refusals
//! included, so no shape is entertained without a stated row. Blanket and empty values
//! excuse **nothing**, deliberately: a blanket escape would restore *"re-pin the hash"*
//! as one keystroke with two meanings, merely renamed.

use engine::manifest::Manifest;
use std::collections::{BTreeMap, BTreeSet};
use std::fmt;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

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

impl Violation {
    /// The frozen entity this violation is about — the identity the per-entity escape
    /// is keyed by. `None` for [`Violation::Unparseable`], which is about a *text*: it
    /// names no entity, so no named escape can reach it.
    fn entity(&self) -> Option<&str> {
        match self {
            Violation::HashMovedWithoutVersion { entity, .. } => Some(entity),
            Violation::Unparseable { .. } => None,
        }
    }
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

/// The escape's trailer key (M48 Increment 11, T2).
///
/// A re-pin is sometimes **legitimate** — the schema-hash presentation projection at
/// M47 Increment 1 re-pinned all 16 doctype hashes at unchanged versions, and both
/// manifest headers name it *"the declared genesis exemption and the ONLY one."* A
/// fence with no way to say that is a fence that gets disabled, so the escape ships
/// with it — as a **commit-message trailer**: per-commit, reviewed with the diff that
/// needs it, and not retro-addable without rewriting history
/// (`completions/artifacts/M48/settle-record.md` → the fence's sub-decision *(i)*,
/// which is also why the fence is CI-only: the local four-command gate runs *before*
/// the commit message exists).
pub const ESCAPE_TRAILER_KEY: &str = "Manifest-Repin:";

/// The entities excused by a range's commit messages.
///
/// **Per entity, never blanket.** A value is matched against the violating entity by
/// **exact equality** — no glob, no `all`, no pattern syntax is interpreted anywhere —
/// so a blanket or empty value names nothing and excuses nothing. That refusal is
/// deliberate and is the point of the whole increment: a blanket escape would restore
/// *"re-pin the hash"* as one keystroke with two meanings, merely renamed, voiding what
/// the fence proves.
///
/// **A line-leading token only.** The key is read off the **raw** line, so an indented
/// line and a mid-prose mention are both prose — the same shape rule the `commit`
/// doctype's own trailer-key guard states (`engine::validate` → the commit-trailer
/// key-shape rule: a git trailer token carries no whitespace and no colon, on its own
/// line). The key is matched **case-sensitively**: the fence fails **closed**, so a
/// mis-typed escape leaves the violation standing rather than silently excusing it.
///
/// Declared bound: position within the message is **not** checked (git recognizes
/// trailers only in the last paragraph). Being permissive about *where* a deliberate,
/// human-written declaration sits — while strict about its shape and its value — costs
/// the fence nothing it protects.
///
/// The messages are a **parameter**: this stays a pure function of text, so its whole
/// axis runs with no git at all (T3 supplies the real range's messages).
pub fn excused_entities(messages: &[&str]) -> BTreeSet<String> {
    messages
        .iter()
        .flat_map(|message| message.lines())
        .filter_map(|line| line.strip_prefix(ESCAPE_TRAILER_KEY))
        .map(str::trim)
        .filter(|entity| !entity.is_empty())
        .map(str::to_string)
        .collect()
}

/// Drop the violations a range's commit messages name — and only those.
///
/// A violation with no entity ([`Violation::Unparseable`]) is **never** excusable: the
/// escape is per-entity, and an unreadable manifest names none. Order is preserved, so
/// the surviving report is the same bytes it would have been unexcused.
pub fn excuse(violations: Vec<Violation>, messages: &[&str]) -> Vec<Violation> {
    let excused = excused_entities(messages);
    violations
        .into_iter()
        .filter(|violation| match violation.entity() {
            Some(entity) => !excused.contains(entity),
            None => true,
        })
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

/// The named escape (M48 Increment 11, T2) — `Manifest-Repin: <entity>`.
///
/// A violation is excused **iff** some commit message in the inspected range carries a
/// **line-leading** `Manifest-Repin:` trailer whose value names **that exact entity**.
/// Everything else about the escape is a refusal, and each refusal is an arm below
/// rather than a comment: the escape exists so a legitimate re-pin can ship, and it is
/// worth nothing if it can also wave the illegitimate one through.
mod escape {
    use super::*;

    /// A violation over one named entity, to hold the escape against.
    fn violation(entity: &str) -> Violation {
        Violation::HashMovedWithoutVersion {
            manifest: DEV_MANIFEST.to_string(),
            entity: entity.to_string(),
            version: 2,
            base_hash: HASH_A.to_string(),
            head_hash: HASH_B.to_string(),
        }
    }

    /// One shape a commit message can carry the escape in, and whether it excuses the
    /// entity `adr`.
    struct Shape {
        /// Why this row exists — printed on failure, so a red arm names its own rule.
        rule: &'static str,
        /// The whole commit message, subject line included.
        message: &'static str,
        /// Whether `adr`'s violation is excused by it.
        excuses: bool,
    }

    /// **The escape's axis, as a table.** Every shape the token can appear in — the one
    /// that excuses, and every one that must not — enumerated in one place, so a shape
    /// cannot be entertained without a row stating its verdict.
    const SHAPES: &[Shape] = &[
        Shape {
            rule: "a line-leading trailer naming the exact entity is the escape",
            message: "fix(engine): re-pin adr after the projection change\n\n\
                      Manifest-Repin: adr\n",
            excuses: true,
        },
        Shape {
            rule: "git accepts `token:value`; the value is trimmed, not required to be spaced",
            message: "fix(engine): x\n\nManifest-Repin:adr\n",
            excuses: true,
        },
        Shape {
            rule: "surrounding whitespace in the value is trimmed, not part of the name",
            message: "fix(engine): x\n\nManifest-Repin:   adr  \n",
            excuses: true,
        },
        Shape {
            rule: "the trailer need not be the last line of the message",
            message: "fix(engine): x\n\nManifest-Repin: adr\nCo-Authored-By: Someone <a@b>\n",
            excuses: true,
        },
        Shape {
            rule: "a trailer naming a DIFFERENT entity excuses nothing — the escape is \
                   keyed on the entity, not on the presence of the token",
            message: "fix(engine): x\n\nManifest-Repin: changelog\n",
            excuses: false,
        },
        Shape {
            rule: "an EMPTY value names no entity and excuses nothing",
            message: "fix(engine): x\n\nManifest-Repin:\n",
            excuses: false,
        },
        Shape {
            rule: "a whitespace-only value names no entity and excuses nothing",
            message: "fix(engine): x\n\nManifest-Repin:    \n",
            excuses: false,
        },
        Shape {
            rule: "`*` is not an entity: no pattern syntax is interpreted, so a BLANKET \
                   value excuses nothing — refused deliberately, since a blanket escape \
                   restores the one-keystroke-two-meanings ambiguity this fence ends",
            message: "fix(engine): x\n\nManifest-Repin: *\n",
            excuses: false,
        },
        Shape {
            rule: "`all` is matched by exact equality like any other name, and no entity \
                   is called `all` — the second blanket shape, refused the same way",
            message: "fix(engine): x\n\nManifest-Repin: all\n",
            excuses: false,
        },
        Shape {
            rule: "a comma-joined list names no single entity — one trailer per entity, \
                   so a legitimate multi-entity re-pin writes the names out",
            message: "fix(engine): x\n\nManifest-Repin: adr, changelog\n",
            excuses: false,
        },
        Shape {
            rule: "a MID-PROSE mention is not a trailer: the token is read line-leading, \
                   the shape rule the commit doctype's own trailer-key guard states",
            message: "fix(engine): x\n\nWe considered Manifest-Repin: adr and decided against it.\n",
            excuses: false,
        },
        Shape {
            rule: "an indented line is not a trailer either — the raw line is read, so a \
                   quoted or fenced block cannot smuggle an escape in",
            message: "fix(engine): x\n\n    Manifest-Repin: adr\n",
            excuses: false,
        },
        Shape {
            rule: "a longer key that merely STARTS with the token is a different key",
            message: "fix(engine): x\n\nManifest-Repinned: adr\n",
            excuses: false,
        },
        Shape {
            rule: "the key is matched case-sensitively — the fence fails closed, so a \
                   mis-typed escape leaves the violation standing",
            message: "fix(engine): x\n\nmanifest-repin: adr\n",
            excuses: false,
        },
        Shape {
            rule: "a message carrying no escape at all excuses nothing",
            message: "fix(engine): re-pin adr\n\nNo trailer here.\n",
            excuses: false,
        },
    ];

    #[test]
    fn every_shape_the_token_can_appear_in_has_a_stated_verdict() {
        for Shape {
            rule,
            message,
            excuses,
        } in SHAPES
        {
            let survivors = excuse(vec![violation("adr")], &[message]);
            assert_eq!(
                survivors.is_empty(),
                *excuses,
                "{rule}\n  message: {message:?}\n  survivors: {survivors:?}",
            );
        }
    }

    #[test]
    fn the_named_entity_is_excused_and_its_neighbour_is_not() {
        let survivors = excuse(
            vec![violation("adr"), violation(SLUG_RULE_ENTITY)],
            &["fix(engine): re-pin adr\n\nManifest-Repin: adr\n"],
        );
        assert_eq!(
            survivors,
            vec![violation(SLUG_RULE_ENTITY)],
            "the escape is per entity: naming one leaves every other violation from the \
             same commit standing",
        );
    }

    #[test]
    fn several_entities_are_excused_across_several_commits_of_one_range() {
        let messages = [
            "fix(engine): re-pin adr\n\nManifest-Repin: adr\n",
            "chore: unrelated\n",
            "fix(engine): re-pin the slug rule and the changelog\n\n\
             Manifest-Repin: slug-rule\n\
             Manifest-Repin: changelog\n",
        ];
        let survivors = excuse(
            vec![
                violation("adr"),
                violation("changelog"),
                violation(SLUG_RULE_ENTITY),
                violation("spec"),
            ],
            &messages,
        );
        assert_eq!(
            survivors,
            vec![violation("spec")],
            "the whole inspected range is read, and one commit may name more than one \
             entity — one trailer each",
        );
    }

    #[test]
    fn an_unparseable_manifest_is_never_excused() {
        let unparseable = Violation::Unparseable {
            manifest: DEV_MANIFEST.to_string(),
            side: Side::Base,
            error: "did not parse".to_string(),
        };
        let survivors = excuse(
            vec![unparseable.clone()],
            &[
                "chore: x\n\nManifest-Repin: adr\n",
                "chore: y\n\nManifest-Repin: slug-rule\n",
            ],
        );
        assert_eq!(
            survivors,
            vec![unparseable],
            "the escape is per entity and an unreadable text names none: no set of \
             trailers can excuse the fail-closed verdict",
        );
    }

    #[test]
    fn the_escape_composes_over_the_comparator_it_excuses() {
        let base = with_doctypes(&[("adr", 2, HASH_A)]);
        let head = with_doctypes(&[("adr", 2, HASH_B)]);
        let found = compare_manifest(DEV_MANIFEST, Some(&base), &head);
        assert_eq!(
            found.len(),
            1,
            "the fixture must genuinely violate: {found:?}"
        );
        assert_eq!(
            excuse(found, &["fix(engine): x\n\nManifest-Repin: adr\n"]),
            vec![],
            "the escape excuses the comparator's own violations, keyed by the entity \
             the comparator names",
        );
    }

    /// The excused set is a set, not a log: the same messages fed in **two divergent
    /// orders** must leave byte-identical output, or which entity survived would depend
    /// on the order git happened to list the range in.
    #[test]
    fn the_surviving_report_is_byte_identical_under_divergent_message_orders() {
        let forward = [
            "fix: a\n\nManifest-Repin: adr\n",
            "fix: b\n\nManifest-Repin: changelog\n",
        ];
        let reverse = [forward[1], forward[0]];
        let violations = || {
            vec![
                violation("adr"),
                violation("changelog"),
                violation(SLUG_RULE_ENTITY),
            ]
        };

        let in_order = excuse(violations(), &forward);
        let reversed = excuse(violations(), &reverse);
        assert_eq!(in_order.len(), 1, "two of three excused: {in_order:?}");
        assert_eq!(
            rendered(&in_order),
            rendered(&reversed),
            "the surviving report must not depend on the order the range's messages \
             arrive in",
        );
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// The window (M48 Increment 11, T3) — the pushed range, and its fallback ladder.
// ─────────────────────────────────────────────────────────────────────────────
//
// The comparator above is a pure function of two texts; **which two texts** is the
// question this section answers, and it is the question the increment-11 plan halt
// (2026-08-15) re-opened. The settled shape compared `HEAD~1` against the working
// copy — but GitHub Actions fires **one run per push, at the tip**, and this repo
// pushes in large batches. `6e81d53` (M47 Increment 1) re-pinned **all 16 doctype
// hashes at unchanged `schema-version`s, in both manifests, in one commit** — the
// exact shape this fence exists to catch — and it landed **34 first-parent commits
// from its push tip**, where a `HEAD~1` window is provably clean. A fence that
// cannot see the only real instance in the repo's history is not a fence
// (`implementation/roadmap.md` → M48 Increment 11; `completions/artifacts/M48/
// settle-record.md` → *CORRECTED at the increment-11 plan halt*).
//
// So the base is the **base of the pushed range**, resolved by a ladder, and the
// ladder lives **here, in Rust, never in the workflow YAML**: the workflow's whole job
// is to forward each raw event field to its own variable and choose nothing among
// them. A `${{ … || … }}` expression in YAML would put the very decision the halt
// found broken back where nothing tests it.
//
// The rungs, in order:
//
// | event | candidate | when the candidate is unusable |
// |---|---|---|
// | `push` | `github.event.before` | fall to the floor |
// | `pull_request` / `pull_request_target` | the PR base sha | fall to the floor |
// | anything else (`workflow_dispatch`, `schedule`, …) | none | the floor |
//
// *Unusable* means absent, empty, all-zeros (GitHub's new-branch sentinel), or **not
// resolvable in this clone** (a force-pushed range whose base object is gone). The
// **floor is `HEAD~1`** — the originally-settled shape, kept as the floor rather than
// replaced, so widening the window can only ever add reach. A root commit has no
// `HEAD~1` and therefore nothing to compare: clean, not an error.

/// Both manifests the fence compares, in the order it reports them.
pub const MANIFESTS: [&str; 2] = [DEV_MANIFEST, METHODOLOGY_MANIFEST];

/// The raw GitHub Actions event fields the ladder chooses between.
///
/// Both candidates are carried **verbatim and separately**: the choice among them is
/// made by [`Event::declared_base`], in Rust, so the ladder is a tested function
/// rather than an untested workflow expression.
#[derive(Clone, Copy, Debug)]
pub struct Event<'a> {
    /// `GITHUB_EVENT_NAME` — `push`, `pull_request`, `workflow_dispatch`, …
    pub name: &'a str,
    /// `github.event.before` — the commit the pushed range starts after. All-zeros on
    /// a new branch; absent on every non-push event.
    pub push_before: &'a str,
    /// `github.event.pull_request.base.sha` — absent on every non-PR event.
    pub pr_base: &'a str,
}

impl Event<'_> {
    /// The base this event *declares*, before it is checked against the clone.
    ///
    /// The event's kind picks the field: a `push` never reads a PR base and a
    /// `pull_request` never reads `before`, so a stale or cross-wired variable cannot
    /// silently widen or narrow the window.
    pub fn declared_base(&self) -> Option<&str> {
        match self.name {
            "push" => usable(self.push_before),
            "pull_request" | "pull_request_target" => usable(self.pr_base),
            _ => None,
        }
    }
}

/// A candidate ref that names something, or `None`.
///
/// Empty, whitespace, and **all-zeros of any length** are all *nothing named*: GitHub
/// writes the zero sha for a branch that did not exist before this push, and a
/// zero-sha `git rev-parse` would simply fail — the ladder says so explicitly rather
/// than relying on that.
fn usable(value: &str) -> Option<&str> {
    let value = value.trim();
    (!value.is_empty() && !value.chars().all(|c| c == '0')).then_some(value)
}

/// Which end of the ladder the base came from — and therefore how much of the push
/// the fence can see.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Window {
    /// The base of the pushed range, as the event declared it and the clone resolved
    /// it — the whole push is in view.
    Declared(String),
    /// The floor: `HEAD~1`. Only the tip commit is in view — the originally-settled
    /// shape, kept as a floor.
    Floor(String),
    /// Nothing to compare: a root commit has no prior state.
    Nothing,
}

impl Window {
    /// The resolved base sha, or `None` when there is nothing to compare.
    pub fn base(&self) -> Option<&str> {
        match self {
            Window::Declared(sha) | Window::Floor(sha) => Some(sha),
            Window::Nothing => None,
        }
    }
}

/// Walk the ladder against a real clone: the declared base if it resolves here, the
/// `HEAD~1` floor otherwise, and nothing at a root commit.
pub fn resolve_window(repo: &Path, event: Event<'_>) -> Window {
    if let Some(sha) = event.declared_base().and_then(|r| rev_parse(repo, r)) {
        return Window::Declared(sha);
    }
    match rev_parse(repo, "HEAD~1") {
        Some(sha) => Window::Floor(sha),
        None => Window::Nothing,
    }
}

/// Resolve a rev to a commit sha, or `None` when this clone cannot reach it.
///
/// `^{commit}` so a ref that exists but names no commit is unusable too, and
/// `--quiet` so an unresolvable rev is an answer rather than noise on stderr.
fn rev_parse(repo: &Path, rev: &str) -> Option<String> {
    let out = Command::new("git")
        .args([
            "rev-parse",
            "--verify",
            "--quiet",
            &format!("{rev}^{{commit}}"),
        ])
        .current_dir(repo)
        .output()
        .expect("run git rev-parse");
    if !out.status.success() {
        return None;
    }
    let sha = String::from_utf8(out.stdout).ok()?.trim().to_string();
    (!sha.is_empty()).then_some(sha)
}

/// One file's bytes at one rev, or `None` when the path does not exist there.
///
/// `None` is the comparator's *manifest absent at base* cell — a pack whose manifest
/// was first added inside the inspected range declares no prior pin.
pub fn show_at(repo: &Path, rev: &str, path: &str) -> Option<String> {
    let out = Command::new("git")
        .args(["show", &format!("{rev}:{path}")])
        .current_dir(repo)
        .output()
        .expect("run git show");
    out.status
        .success()
        .then(|| String::from_utf8(out.stdout).expect("a manifest is utf-8"))
}

/// Every commit message in `base..head`, whole — subject and body — feeding the
/// per-entity escape.
///
/// The **whole range**, not the tip: the escape is written on the commit that does
/// the re-pin, and in a batched push that commit is rarely the last one.
pub fn messages_in(repo: &Path, base: &str, head: &str) -> Vec<String> {
    let out = Command::new("git")
        .args(["log", "--format=%B%x00", &format!("{base}..{head}")])
        .current_dir(repo)
        .output()
        .expect("run git log");
    assert!(
        out.status.success(),
        "git log {base}..{head} failed: {}",
        String::from_utf8_lossy(&out.stderr),
    );
    String::from_utf8(out.stdout)
        .expect("a commit message is utf-8")
        .split('\0')
        // `%B%x00` leaves git's own inter-commit newline glued to the front of the
        // next message. Only that separator is stripped — never the message's own
        // leading bytes, since the escape is read line-leading and trimming would
        // promote an indented first line into a trailer.
        .map(|m| m.strip_prefix('\n').unwrap_or(m).to_string())
        .filter(|m| !m.trim().is_empty())
        .collect()
}

/// The text a manifest **absent at the head side** is read as: no pins at all.
///
/// Written out rather than left to an empty string's deserialization, so the reading
/// is a stated choice. It makes every entity read as *removed* — the comparator's
/// clean cell — which is the honest verdict here and not a hole: shipping **no**
/// `config/schema-manifest.yaml` is the freeze's own **wholesale, pack-level opt-out**
/// (`crates/cli/src/pack.rs` → a manifest-less pack stays on skip-on-absent;
/// `engine::manifest::Manifest` → *"Opting out of the freeze stays a pack-level,
/// wholesale act"*). A per-entity fence does not get to overrule that by re-deciding
/// it one entity at a time.
const NO_DECLARATION: &str = "doctypes: []\n";

/// The whole fence over one window of one clone: both manifests at either end,
/// compared, then excused by the range's own commit messages.
pub fn fence_over(repo: &Path, base: &str, head: &str) -> Vec<Violation> {
    let texts: Vec<(&str, Option<String>, String)> = MANIFESTS
        .iter()
        .map(|path| {
            (
                *path,
                show_at(repo, base, path),
                show_at(repo, head, path).unwrap_or_else(|| NO_DECLARATION.to_string()),
            )
        })
        .collect();
    let pairs: Vec<(&str, Option<&str>, &str)> = texts
        .iter()
        .map(|(path, base, head)| (*path, base.as_deref(), head.as_str()))
        .collect();
    let messages = messages_in(repo, base, head);
    let messages: Vec<&str> = messages.iter().map(String::as_str).collect();
    excuse(compare_all(&pairs), &messages)
}

/// A self-cleaning temp dir — the shipped suite idiom (pid + nanos, `Drop`-removed),
/// so a throwaway `git init` never lands in the developer's tree.
struct TempDir(PathBuf);

impl TempDir {
    fn new(tag: &str) -> Self {
        let mut path = std::env::temp_dir();
        path.push(format!(
            "jigc-fence-{tag}-{}-{:?}",
            std::process::id(),
            engine::tempname::unique_nanos(),
        ));
        fs::create_dir_all(&path).expect("create temp dir");
        TempDir(path)
    }

    fn path(&self) -> &Path {
        &self.0
    }
}

impl Drop for TempDir {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}

/// Run git in a throwaway repo, asserting success — the fixtures' own plumbing, kept
/// separate from [`rev_parse`], whose whole job is to tolerate failure.
fn git(repo: &Path, args: &[&str]) -> String {
    let out = Command::new("git")
        .args(args)
        .current_dir(repo)
        .output()
        .expect("run git");
    assert!(
        out.status.success(),
        "git {args:?} failed: {}",
        String::from_utf8_lossy(&out.stderr),
    );
    String::from_utf8(out.stdout)
        .expect("utf-8")
        .trim()
        .to_string()
}

/// A throwaway repo with git identity configured and no commits yet.
fn fresh_repo(tag: &str) -> TempDir {
    let repo = TempDir::new(tag);
    git(repo.path(), &["init", "-q"]);
    git(repo.path(), &["config", "user.email", "fence@example.com"]);
    git(repo.path(), &["config", "user.name", "Fence"]);
    repo
}

/// Commit every change in the throwaway repo under one message.
fn commit_all(repo: &Path, message: &str) -> String {
    git(repo, &["add", "-A"]);
    git(repo, &["commit", "-q", "--allow-empty", "-m", message]);
    git(repo, &["rev-parse", "HEAD"])
}

/// Write one manifest text at its repo-relative path, creating the pack dirs.
fn write_manifest(repo: &Path, path: &str, text: &str) {
    let target = repo.join(path);
    fs::create_dir_all(target.parent().expect("a manifest has a parent"))
        .expect("create pack dirs");
    fs::write(target, text).expect("write manifest");
}

/// The base-ref ladder: which commit the fence compares against, and why.
mod base_ref {
    use super::*;

    /// The sentinel the rung table writes where the fixture's real base sha goes — the
    /// table states *shapes*, and the fixture substitutes the one value it cannot know
    /// statically.
    const THE_PUSH_BASE: &str = "<the push base>";

    /// A well-formed sha that resolves nowhere — a force-pushed range whose base object
    /// this clone never received.
    const GONE: &str = "deadbeefdeadbeefdeadbeefdeadbeefdeadbeef";

    /// Which rung the ladder must land on.
    #[derive(Debug, PartialEq, Eq)]
    enum Expect {
        /// The declared base of the pushed range.
        Declared,
        /// The `HEAD~1` floor.
        Floor,
    }

    /// One event shape, and the rung it lands on.
    struct Rung {
        /// Why the row exists — printed on failure, so a red arm names its own rule.
        rule: &'static str,
        name: &'static str,
        push_before: &'static str,
        pr_base: &'static str,
        expect: Expect,
    }

    /// **The ladder, as a table.** Every event shape CI can hand the fence against the
    /// rung it lands on — so a shape cannot be entertained without a stated verdict, and
    /// the floor cannot quietly become the ceiling again.
    const RUNGS: &[Rung] = &[
        Rung {
            rule: "a push declares the base of its range, and the whole push is in view",
            name: "push",
            push_before: THE_PUSH_BASE,
            pr_base: "",
            expect: Expect::Declared,
        },
        Rung {
            rule: "the all-zeros sentinel of a NEW BRANCH names nothing — the floor holds",
            name: "push",
            push_before: "0000000000000000000000000000000000000000",
            pr_base: "",
            expect: Expect::Floor,
        },
        Rung {
            rule: "the zero sentinel is refused by SHAPE, not by length — a sha-256 repo \
                   writes 64 of them",
            name: "push",
            push_before: "0000000000000000000000000000000000000000000000000000000000000000",
            pr_base: "",
            expect: Expect::Floor,
        },
        Rung {
            rule: "an ABSENT `before` (the variable never set) falls to the floor",
            name: "push",
            push_before: "",
            pr_base: "",
            expect: Expect::Floor,
        },
        Rung {
            rule: "a whitespace-only value names nothing either",
            name: "push",
            push_before: "   ",
            pr_base: "",
            expect: Expect::Floor,
        },
        Rung {
            rule: "a base this clone cannot resolve (FORCE-PUSH: the object is gone) \
                   falls to the floor rather than failing the run",
            name: "push",
            push_before: GONE,
            pr_base: "",
            expect: Expect::Floor,
        },
        Rung {
            rule: "a push never reads a PR base — a cross-wired variable cannot widen \
                   the window",
            name: "push",
            push_before: "",
            pr_base: THE_PUSH_BASE,
            expect: Expect::Floor,
        },
        Rung {
            rule: "a pull request declares its base sha",
            name: "pull_request",
            push_before: "",
            pr_base: THE_PUSH_BASE,
            expect: Expect::Declared,
        },
        Rung {
            rule: "`pull_request_target` is the same event shape and reads the same field",
            name: "pull_request_target",
            push_before: "",
            pr_base: THE_PUSH_BASE,
            expect: Expect::Declared,
        },
        Rung {
            rule: "a pull request with no base sha falls to the floor",
            name: "pull_request",
            push_before: "",
            pr_base: "",
            expect: Expect::Floor,
        },
        Rung {
            rule: "a pull request never reads `before` — the cross-wiring refusal runs \
                   both ways",
            name: "pull_request",
            push_before: THE_PUSH_BASE,
            pr_base: "",
            expect: Expect::Floor,
        },
        Rung {
            rule: "`workflow_dispatch` declares neither field, so it reads NEITHER — even \
                   when a stale value is present in the environment",
            name: "workflow_dispatch",
            push_before: THE_PUSH_BASE,
            pr_base: THE_PUSH_BASE,
            expect: Expect::Floor,
        },
        Rung {
            rule: "any other event (`schedule`, …) is the floor by the same rule",
            name: "schedule",
            push_before: "",
            pr_base: "",
            expect: Expect::Floor,
        },
    ];

    /// A repo of four commits, so the declared base and the `HEAD~1` floor are **two
    /// different commits** — a fixture where they coincide proves nothing about either.
    fn four_commit_repo() -> (TempDir, String, String) {
        let repo = fresh_repo("ladder");
        commit_all(repo.path(), "one");
        let base = commit_all(repo.path(), "two — the base of the pushed range");
        let floor = commit_all(repo.path(), "three — what HEAD~1 reaches");
        commit_all(repo.path(), "four — the tip");
        (repo, base, floor)
    }

    #[test]
    fn every_event_shape_lands_on_a_stated_rung() {
        let (repo, base, floor) = four_commit_repo();
        let subst = |value: &str| {
            if value == THE_PUSH_BASE {
                base.clone()
            } else {
                value.to_string()
            }
        };
        for rung in RUNGS {
            let push_before = subst(rung.push_before);
            let pr_base = subst(rung.pr_base);
            let found = resolve_window(
                repo.path(),
                Event {
                    name: rung.name,
                    push_before: &push_before,
                    pr_base: &pr_base,
                },
            );
            let expected = match rung.expect {
                Expect::Declared => Window::Declared(base.clone()),
                Expect::Floor => Window::Floor(floor.clone()),
            };
            assert_eq!(
                found, expected,
                "{}\n  event: {} before={push_before:?} pr_base={pr_base:?}",
                rung.rule, rung.name,
            );
        }
    }

    #[test]
    fn the_floor_is_the_settled_shape_kept_as_a_floor_never_the_ceiling() {
        let (repo, base, floor) = four_commit_repo();
        let declared = resolve_window(
            repo.path(),
            Event {
                name: "push",
                push_before: &base,
                pr_base: "",
            },
        );
        assert_eq!(
            declared.base(),
            Some(base.as_str()),
            "the declared window reaches past HEAD~1 — {floor} is what the settled \
             shape would have compared against",
        );
        assert_ne!(base, floor, "the fixture must genuinely differ");
    }

    #[test]
    fn a_root_commit_has_nothing_to_compare() {
        let repo = fresh_repo("root");
        commit_all(repo.path(), "the root commit");
        assert_eq!(
            resolve_window(
                repo.path(),
                Event {
                    name: "workflow_dispatch",
                    push_before: "",
                    pr_base: "",
                },
            ),
            Window::Nothing,
            "a repo whose only commit is its root has no prior state: clean, not an \
             error — the fence reports nothing rather than failing the run",
        );
    }

    #[test]
    fn a_root_commit_with_an_unresolvable_declared_base_still_has_nothing_to_compare() {
        let repo = fresh_repo("root-declared");
        commit_all(repo.path(), "the root commit");
        assert_eq!(
            resolve_window(
                repo.path(),
                Event {
                    name: "push",
                    push_before: GONE,
                    pr_base: "",
                },
            ),
            Window::Nothing,
            "the ladder falls all the way through: an unresolvable declaration, then no \
             floor to land on",
        );
    }

    /// The plumbing the comparator eats: one file's bytes at a rev.
    #[test]
    fn a_manifest_is_read_at_its_rev_and_a_path_absent_there_is_none() {
        let repo = fresh_repo("show");
        write_manifest(
            repo.path(),
            DEV_MANIFEST,
            &with_doctypes(&[("adr", 2, HASH_A)]),
        );
        let base = commit_all(repo.path(), "the dev manifest only");
        write_manifest(
            repo.path(),
            METHODOLOGY_MANIFEST,
            &with_doctypes(&[("idea", 1, HASH_B)]),
        );
        commit_all(repo.path(), "the methodology manifest joins");

        assert_eq!(
            show_at(repo.path(), &base, DEV_MANIFEST).as_deref(),
            Some(with_doctypes(&[("adr", 2, HASH_A)]).as_str()),
            "the base text is the file's own bytes at that commit",
        );
        assert_eq!(
            show_at(repo.path(), &base, METHODOLOGY_MANIFEST),
            None,
            "a manifest first added inside the range is absent at the base — the \
             comparator's *absent at base* cell, fed from git rather than fabricated",
        );
    }

    /// The plumbing the escape eats: every message in the range, whole.
    #[test]
    fn the_ranges_whole_messages_are_read_and_nothing_outside_it_is() {
        let repo = fresh_repo("messages");
        commit_all(repo.path(), "before the range\n\nManifest-Repin: adr\n");
        let base = commit_all(repo.path(), "the base");
        commit_all(repo.path(), "inside\n\nManifest-Repin: changelog\n");
        commit_all(repo.path(), "the tip\n\nManifest-Repin: slug-rule\n");

        let messages = messages_in(repo.path(), &base, "HEAD");
        let messages: Vec<&str> = messages.iter().map(String::as_str).collect();
        assert_eq!(messages.len(), 2, "two commits in the range: {messages:?}");
        assert_eq!(
            excused_entities(&messages),
            ["changelog".to_string(), SLUG_RULE_ENTITY.to_string()]
                .into_iter()
                .collect(),
            "the body of every commit in the range is read — and a trailer written \
             BEFORE the base is outside the window, so it excuses nothing",
        );
    }

    /// A throwaway repo shaped like one of this project's real pushes: the re-pin sits
    /// in the **middle** of the batch, never at the tip.
    fn batched_push_repo(tag: &str, repin_message: &str) -> (TempDir, String) {
        let repo = fresh_repo(tag);
        write_manifest(
            repo.path(),
            DEV_MANIFEST,
            &with_doctypes(&[("adr", 2, HASH_A)]),
        );
        let base = commit_all(repo.path(), "the pinned base");
        write_manifest(
            repo.path(),
            DEV_MANIFEST,
            &with_doctypes(&[("adr", 2, HASH_B)]),
        );
        commit_all(repo.path(), repin_message);
        commit_all(repo.path(), "an unrelated commit");
        commit_all(repo.path(), "another unrelated commit — the push tip");
        (repo, base)
    }

    /// **The mechanism, proven on a synthetic push before it is proven on the real
    /// one.** The declared window sees a breach the settled floor is provably clean
    /// over — the halt's finding, reduced to four commits.
    #[test]
    fn the_declared_window_catches_a_mid_batch_repin_the_floor_cannot_see() {
        let (repo, base) = batched_push_repo("batch", "re-pin adr with nothing declaring it");

        let over_the_push = fence_over(repo.path(), &base, "HEAD");
        assert_eq!(
            over_the_push,
            vec![Violation::HashMovedWithoutVersion {
                manifest: DEV_MANIFEST.to_string(),
                entity: "adr".to_string(),
                version: 2,
                base_hash: HASH_A.to_string(),
                head_hash: HASH_B.to_string(),
            }],
            "the base of the pushed range sees the whole batch",
        );

        let floor = rev_parse(repo.path(), "HEAD~1").expect("a floor exists");
        assert_eq!(
            fence_over(repo.path(), &floor, "HEAD"),
            vec![],
            "the settled HEAD~1 window is PROVABLY CLEAN over the very push that \
             carries the breach — which is why the floor is a floor and not the fence",
        );
    }

    /// The escape reaches the whole window too: it is written on the commit that
    /// re-pins, which in a batch is not the commit CI runs at.
    #[test]
    fn a_mid_batch_escape_excuses_the_mid_batch_repin() {
        let (repo, base) = batched_push_repo(
            "batch-escaped",
            "re-pin adr, declared\n\nManifest-Repin: adr\n",
        );
        assert_eq!(
            fence_over(repo.path(), &base, "HEAD"),
            vec![],
            "a legitimate re-pin declares itself on its own commit — and the fence \
             reads the range, not the tip",
        );
    }

    /// A manifest deleted inside the range reads as *every entity removed* — clean,
    /// and stated rather than left to an accident of deserialization.
    ///
    /// **This is not the fence looking away.** Deleting the file is the freeze's own
    /// wholesale, pack-level opt-out, declared as such where the freeze is defined; a
    /// per-entity fence that re-decided it one entity at a time would be overruling a
    /// shipped decision from a test file. The arm exists so the reading is *chosen*.
    #[test]
    fn a_manifest_deleted_inside_the_range_reads_as_removed_not_moved() {
        let repo = fresh_repo("deleted");
        write_manifest(
            repo.path(),
            DEV_MANIFEST,
            &with_doctypes(&[("adr", 2, HASH_A)]),
        );
        let base = commit_all(repo.path(), "the pinned base");
        fs::remove_file(repo.path().join(DEV_MANIFEST)).expect("remove the manifest");
        commit_all(repo.path(), "opt the pack out of the freeze, wholesale");

        assert_eq!(
            fence_over(repo.path(), &base, "HEAD"),
            vec![],
            "no hash moved — the declaration itself is gone, which the freeze permits \
             at pack level and this fence does not re-adjudicate per entity",
        );
    }
}

/// **The window's acceptance is this repo's own history.**
///
/// The halt's finding is not left as a paragraph: the commit it turned on is the
/// fixture. `6e81d53` — M47 Increment 1, *"the schema-hash becomes a presentation
/// projection"* — re-pinned **all 16 doctype hashes at unchanged `schema-version`s in
/// both manifests**, and both manifest headers name it *"the declared genesis exemption
/// and the ONLY one."* Over the pushed range the fence flags every one of the 16; over
/// the `HEAD~1`-shaped window at that push's tip it flags none.
///
/// The window is stable by construction: **no commit after `6e81d53` touches either
/// manifest** (`git log 6e81d53..HEAD -- <both>` is empty), so the flagged set is
/// exactly the 16 and stays so.
///
/// **These arms fail loudly rather than skip.** A shallow clone that cannot reach the
/// fixture makes the fence's own acceptance unverifiable, and a skipped fence is the
/// green nobody earned — which is why `.github/workflows/ci.yml` checks out with
/// `fetch-depth: 0`.
mod historical {
    use super::*;

    /// M47 Increment 1 — the re-pin of all 16, and the only declared genesis exemption.
    const GENESIS: &str = "6e81d53";

    /// The base of the window that contains it.
    const GENESIS_PARENT: &str = "6e81d53~1";

    /// The tip of the push that carried it — pinned as a sha rather than an offset.
    const PUSH_TIP: &str = "d0b8728";

    /// How far the genesis commit landed from its push tip, first-parent.
    const DISTANCE_FROM_TIP: usize = 34;

    /// The dev pack's frozen doctypes at the genesis commit.
    const DEV_DOCTYPES: [&str; 6] = ["commit", "adr", "spec", "prd", "arch-doc", "changelog"];

    /// The methodology pack's, at the same commit.
    const METHODOLOGY_DOCTYPES: [&str; 10] = [
        "commit",
        "completion-record",
        "decisions-log",
        "deferral-ledger",
        "dogfood-record",
        "idea",
        "milestone-record",
        "research",
        "roadmap",
        "vision",
    ];

    /// This repo — the fixture is its own history, so the arms run against the checkout.
    fn this_repo() -> PathBuf {
        Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../..")
            .canonicalize()
            .expect("the workspace root is reachable from the cli crate")
    }

    /// Fail loudly on a clone that cannot reach the fixture.
    fn require_reachable(repo: &Path, rev: &str) {
        assert!(
            rev_parse(repo, rev).is_some(),
            "{rev} is unreachable in this clone, so the fence's own acceptance cannot \
             run. This arm FAILS rather than skips: a skipped fence is a green nobody \
             earned. Check out with full history (`fetch-depth: 0` in CI, \
             `git fetch --unshallow` locally).",
        );
    }

    #[test]
    fn the_fixture_commits_are_reachable_and_a_whole_push_apart() {
        let repo = this_repo();
        require_reachable(&repo, GENESIS);
        require_reachable(&repo, GENESIS_PARENT);
        require_reachable(&repo, PUSH_TIP);
        let distance = git(
            &repo,
            &[
                "rev-list",
                "--first-parent",
                "--count",
                &format!("{GENESIS}..{PUSH_TIP}"),
            ],
        );
        assert_eq!(
            distance.parse::<usize>().expect("a count"),
            DISTANCE_FROM_TIP,
            "the genesis re-pin landed {DISTANCE_FROM_TIP} first-parent commits from \
             its push tip — the distance is the whole point: the settled window could \
             see one of them",
        );
    }

    #[test]
    fn the_repin_of_all_sixteen_is_flagged_over_the_pushed_range() {
        let repo = this_repo();
        require_reachable(&repo, GENESIS_PARENT);
        require_reachable(&repo, PUSH_TIP);

        let found = fence_over(&repo, GENESIS_PARENT, PUSH_TIP);
        let flagged: BTreeSet<(String, String)> = found
            .iter()
            .map(|violation| match violation {
                Violation::HashMovedWithoutVersion {
                    manifest, entity, ..
                } => (manifest.clone(), entity.clone()),
                other => panic!("the genesis re-pin is a hash move, got {other:?}"),
            })
            .collect();

        let expected: BTreeSet<(String, String)> = DEV_DOCTYPES
            .iter()
            .map(|ty| (DEV_MANIFEST.to_string(), ty.to_string()))
            .chain(
                METHODOLOGY_DOCTYPES
                    .iter()
                    .map(|ty| (METHODOLOGY_MANIFEST.to_string(), ty.to_string())),
            )
            .collect();
        assert_eq!(
            flagged, expected,
            "over the pushed range the fence flags all 16 doctype entities across both \
             manifests — the exact event it exists to catch, and the one the repo's own \
             history carries",
        );
        assert_eq!(found.len(), 16, "one violation each: {found:?}");
        assert!(
            !flagged.iter().any(|(_, entity)| entity == SLUG_RULE_ENTITY),
            "the genesis commit re-pinned doctype hashes only — the slug rule did not \
             move, and the fence does not invent a violation over it",
        );
    }

    #[test]
    fn the_head_tilde_one_window_at_the_push_tip_is_clean() {
        let repo = this_repo();
        require_reachable(&repo, PUSH_TIP);
        assert_eq!(
            fence_over(&repo, &format!("{PUSH_TIP}~1"), PUSH_TIP),
            vec![],
            "the settled `HEAD~1` shape, evaluated where CI would actually have \
             evaluated it, is PROVABLY CLEAN over the push that carried the re-pin of \
             all 16 — the fence as settled would have missed the exact event it exists \
             to catch",
        );
    }
}
