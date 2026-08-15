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
