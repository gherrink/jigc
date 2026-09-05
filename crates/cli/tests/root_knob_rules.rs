//! M50 Increment 4, T1 — **the two root knobs get one home** (`settle-record.md` → D8;
//! `roadmap.md` → Milestone 50, Increment 4).
//!
//! `docs-root` and `placement-root` are the two knobs that re-point where every managed
//! doc lives, and both then **move** the committed docs the re-point strands. Every rule
//! that binds one binds the other, and until now that fact lived twice, hand-written, as
//! a `matches!` over the two key literals — the shape M45's complete-fix lens is named
//! for: a third rule (T2's home predicate, T3's value predicate) applied at one arm and
//! not the other is a rule that is not applied at all.
//!
//! Two arms, each naming the kind of set it iterates:
//!
//!   * **The source fence** — a *derivation over one file's text*: the hand-written pair
//!     literal appears **nowhere** in `crates/cli/src/config.rs`, in either ordering, so a
//!     future per-knob rule cannot be re-spelled inline beside the registry that exists to
//!     carry it. The fence lives in a different file from the one it scans: a fence whose
//!     own assertion text carries the forbidden literal could never go green.
//!   * **The declaration derivation** — every [`cli::config::ROOT_KNOBS`] member is a
//!     **declared knob** in the pack's `config/knobs.yaml`, read through the same
//!     `engine::knobs::load_knobs` the cascade seeds itself from (never a hand list here).
//!     A registry member that is not on the closed surface would name a key `jigc config
//!     set` rejects before any root rule could reach it.

use engine::knobs::load_knobs;
use engine::packsource::{PackResourceKind, PackSource, ResourceId};

/// The module the registry governs, read at compile time so the fence cannot drift from
/// the file it is about.
const CONFIG_RS: &str = include_str!("../src/config.rs");

/// Both spellings of the hand-written pair — the fence is about the *shape* (a per-call-site
/// `matches!` over the two root-knob keys), not about one authoring order.
fn pair_literals() -> [String; 2] {
    let docs = format!("{:?}", "docs-root");
    let placement = format!("{:?}", "placement-root");
    [
        format!("{docs} | {placement}"),
        format!("{placement} | {docs}"),
    ]
}

/// **The source fence.** The two root-knob keys are never matched as a hand-written pair
/// inside `config.rs` — the registry is the one home, so a rule added at one knob is added
/// at both by construction.
#[test]
fn config_rs_carries_no_hand_written_root_knob_pair() {
    let hits: usize = pair_literals()
        .iter()
        .map(|lit| CONFIG_RS.matches(lit.as_str()).count())
        .sum();
    assert_eq!(
        hits, 0,
        "`crates/cli/src/config.rs` still matches the two root-knob keys as a hand-written \
         pair ({hits} occurrence(s)) — every root-knob rule reads the one registry \
         (`cli::config::ROOT_KNOBS`) so a rule added at one knob is added at both",
    );
}

/// **The declaration derivation.** Every [`cli::config::ROOT_KNOBS`] member is a declared
/// knob on the pack's closed surface, read through the same `engine::knobs::load_knobs` the
/// cascade seeds itself from — never a hand list here, which would prove only that this file
/// agrees with itself. A member that is not declared would name a key `jigc config set`
/// rejects with `config.undeclared-key` before any root rule could reach it, so the registry
/// would carry a rule for a knob nobody can set.
#[test]
fn every_root_knob_is_a_declared_knob() {
    let pack = cli::pack::EmbeddedPack::new();
    let bytes = pack
        .read(PackResourceKind::Config, &ResourceId::from("knobs"))
        .expect("the embedded pack ships `config/knobs`");
    let knobs = load_knobs(&bytes).expect("`config/knobs` parses");
    let declared: Vec<&str> = knobs.keys().collect();

    assert!(
        !cli::config::ROOT_KNOBS.is_empty(),
        "the root-knob registry is the subject of this derivation — an empty one would make \
         every arm below vacuous",
    );
    for key in cli::config::ROOT_KNOBS {
        assert!(
            knobs.field(key).is_some(),
            "`{key}` is a `ROOT_KNOBS` member but not a declared knob in the pack's \
             `config/knobs.yaml`; declared: {declared:?}",
        );
    }
}
