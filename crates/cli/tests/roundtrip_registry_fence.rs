//! # The round-trip completeness fence — registry-derived
//!
//! Replaces the engine's **circular** completeness assert
//! `every_persisted_methodology_doctype_has_a_deterministic_fixture`, which compared the
//! hand-authored `methodology_fixtures()` table against the hand-authored
//! `PERSISTED_METHODOLOGY_DOCTYPES` const — **two hand lists in one file**, so a doctype
//! that fell out of *both* was invisible, and a persisted doctype added to the shipped
//! pack (but not the const) silently escaped the fence. The requirement side now comes
//! from the **live composed registry** ([pinning.md](../../../implementation/pinning.md)
//! §1: *enumeration comes from the registries, never a hand list*; §2: the commit
//! round-trip repair — *the exemption was the pinning hole*).
//!
//! **What round-trips on disk, and therefore must carry a byte-round-trip proof:**
//!  - every **persisted** doctype (a `location:` folder home or a literal `placement:`
//!    file) round-trips through that home — the methodology proofs live in
//!    `engine::write`'s `methodology_canonical_fixtures_*`, the dev `spec`/`arch-doc`
//!    proofs in `engine::write::spec_roundtrip`;
//!  - the transient **`commit`** round-trips through
//!    `.jigc/tasks/<id>/docs/commit:<slug>.md` (`render` ↔ `instance_from_source`),
//!    proven finalize-path in `commit_trailer_roundtrip.rs`. This is exactly the doctype
//!    the false *"[commit] has no on-disk round-trip to fence"* rationale exempted — and
//!    the exemption is where the dead-trailer bug (M45 Inc 4) lived unfenced.
//!
//! The fence is **awareness, not proof** (`pinning.md` §1: *goldens are for noticing*):
//! it holds the registry-derived required set equal to the reviewed snapshot below, so a
//! persisted doctype **removed/renamed** — or **`commit` re-exempted** (the `|| commit`
//! classification clause dropped, or `commit` struck from the snapshot) — reddens here
//! and forces the reviewer to add both the snapshot entry and the proof, rather than the
//! doctype silently falling out. The byte proofs themselves stay in their suites; this
//! fence guards that none is missing.

use std::collections::BTreeSet;

use cli::pack::{CompositePack, EmbeddedPack, load_pack_schema};
use engine::packsource::{PackResourceKind, PackSource};

/// The production composition, built the **CWD-free** way (never `make_pack()`, which
/// resolves against the process CWD and is a hazard under parallel tests): `[dev ▸
/// methodology]`, dev highest-precedence (`design/multi-pack.md` → Embedded second
/// pack). This is the exact pack-set the shipped binary composes.
fn composite() -> CompositePack {
    CompositePack::new(vec![
        Box::new(EmbeddedPack::new()),
        Box::new(EmbeddedPack::methodology()),
    ])
}

/// The registry-derived set of doctypes that round-trip on disk and therefore **must**
/// carry a byte-round-trip proof. Read from the **real shipped schemas** — every schema
/// the composite enumerates, loaded through the production [`load_pack_schema`] path, and
/// classified by its own `location`/`placement`, never a hand list. A doctype is required
/// iff it is **persisted** (a `location:`/`placement:` home) **or** it is `commit` (the
/// transient doctype that round-trips through the task staging copy).
fn round_trip_required(pack: &dyn PackSource) -> BTreeSet<String> {
    pack.list(PackResourceKind::Schemas)
        .iter()
        .filter_map(|id| {
            let bytes = pack
                .read(PackResourceKind::Schemas, id)
                .unwrap_or_else(|e| panic!("schema `{}` reads back: {e}", id.as_str()));
            let schema = load_pack_schema(pack, &bytes)
                .unwrap_or_else(|e| panic!("schema `{}` loads: {e:?}", id.as_str()));
            let persisted = schema.location.is_some() || schema.placement.is_some();
            // `|| commit`: the transient `commit` has no `location`/`placement` home, but
            // it round-trips on disk through `.jigc/tasks/<id>/docs/commit:<slug>.md`.
            // Dropping this clause re-exempts it — the fence below reddens.
            (persisted || schema.ty == "commit").then(|| schema.ty.clone())
        })
        .collect()
}

/// The reviewed snapshot: the 14 persisted doctypes both embedded packs ship (six dev,
/// nine methodology, with `commit` shared) plus the transient `commit`. Sorted, so a
/// diff is legible. A persisted doctype removed/renamed moves `round_trip_required` off
/// this list; `commit` re-exempted moves it off too.
const EXPECTED_ROUND_TRIP_REQUIRED: &[&str] = &[
    "adr",
    "arch-doc",
    "changelog",
    "commit",
    "completion-record",
    "decisions-log",
    "deferral-ledger",
    "dogfood-record",
    "idea",
    "milestone-record",
    "prd",
    "research",
    "roadmap",
    "spec",
    "vision",
];

/// **The fence.** The set of doctypes that round-trip on disk — derived live from the
/// composed registry's real shipped schemas — is exactly the reviewed snapshot. A
/// persisted doctype removed/renamed, or `commit` re-exempted, reddens here (the
/// registry no longer matches the snapshot) rather than the doctype silently escaping
/// the byte-round-trip suites.
#[test]
fn round_trip_required_set_matches_the_reviewed_snapshot() {
    let pack = composite();
    let required = round_trip_required(&pack);
    let expected: BTreeSet<String> = EXPECTED_ROUND_TRIP_REQUIRED
        .iter()
        .map(|s| s.to_string())
        .collect();
    assert_eq!(
        required, expected,
        "the round-trip-required doctype set (persisted ∪ {{commit}}, read from the shipped \
         schemas) drifted from the reviewed snapshot — a persisted doctype was added, removed, \
         or renamed, or `commit` was re-exempted; add its byte-round-trip proof and update \
         EXPECTED_ROUND_TRIP_REQUIRED, or revert the schema change"
    );
}

/// The commit half, called out on its own so a re-exemption names the exact regression:
/// `commit` round-trips through `.jigc/tasks/<id>/docs/commit:<slug>.md` (`render` ↔
/// `instance_from_source`), so it must be round-trip-required, never exempt. The false
/// *"no on-disk round-trip to fence"* rationale that hid the dead-trailer bug lives here
/// as a standing assertion.
#[test]
fn commit_is_round_trip_required_not_exempt() {
    let pack = composite();
    assert!(
        round_trip_required(&pack).contains("commit"),
        "`commit` round-trips on disk through .jigc/tasks/<id>/docs/commit:<slug>.md — it must \
         stay round-trip-required (proven in commit_trailer_roundtrip.rs); the false \
         `no on-disk round-trip to fence` rationale is exactly what hid the dead-trailer bug"
    );
}
