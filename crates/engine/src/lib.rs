//! `engine` — the neutral, domain-empty core of jigc.
//!
//! Owns cascade resolution, the document/schema model, parsing & serialization,
//! the doc registry, workflow composition, the validation engine, task/staging
//! state, and the edge index. Depends on no frontend and no domain content, and
//! makes no LLM calls. See `implementation/module-layout.md` for the topology and
//! `VISION.md` for the determinism boundary these modules enforce.
//!
//! Each module below is a stub home for an increment to fill; the names are taken
//! verbatim from the module-layout dependency graph.

/// The path of a file (or directory) inside a built-in pack, for the engine's **tests**
/// — the one place in this crate that knows where a pack sits (M54 S2;
/// `implementation/module-layout.md` → *The seam is prepared now, the crate still is
/// not*). The engine ships empty of content, so only its tests read pack bytes, and they
/// take every pack path from here: `include_bytes!(pack_path!(dev, "schemas/adr.yaml"))`,
/// or `pack_path!(methodology, "schemas")` as a runtime root.
///
/// Each arm is keyed by the pack's `pack-id` and expands to one absolute `concat!`
/// literal, so it serves wherever `include_*!` needs a literal. The two packs sit at
/// different roots until they share one home, which is why each arm carries its own.
/// `crates/cli/tests/pack_path_fence.rs` leaves this macro alone because it is
/// `#[cfg(test)]`.
#[cfg(test)]
macro_rules! pack_path {
    (dev, $rel:literal) => {
        concat!(env!("CARGO_MANIFEST_DIR"), "/../cli/pack/", $rel)
    };
    (methodology, $rel:literal) => {
        concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../packs/methodology/",
            $rel
        )
    };
}

pub mod address;
pub mod cascade;
pub mod catalog;
pub mod compose;
pub mod data_value;
pub mod field_block;
pub mod file_state;
pub mod finalize;
pub mod finding;
pub mod index;
pub mod ingest;
pub mod introspect;
pub mod knobs;
pub mod manifest;
pub mod milestone;
pub mod override_default;
pub mod parse;
pub mod path;
pub mod registry;
pub mod schema;
pub mod schema_diff;
pub mod slug;
pub mod state;
pub mod store;
pub mod target_surface;
pub mod tempname;
pub mod transform;
pub mod validate;
pub mod write;

pub mod packsource;
pub mod probe;
pub mod result;

#[cfg(test)]
mod root_walk;

#[cfg(test)]
mod pack_path_tests {
    /// Each `pack_path!` arm reaches the pack its name says: the root it expands to
    /// declares that `pack-id`. A swapped or stale arm fails here by identity, not by a
    /// missing-file compile error that would say nothing about which pack it found.
    #[test]
    fn each_arm_reaches_the_pack_it_names() {
        for (arm, defaults) in [
            ("dev", include_str!(pack_path!(dev, "config/defaults.yaml"))),
            (
                "methodology",
                include_str!(pack_path!(methodology, "config/defaults.yaml")),
            ),
        ] {
            let value: serde_yaml_ng::Value =
                serde_yaml_ng::from_str(defaults).expect("defaults.yaml parses");
            assert_eq!(
                value["pack-id"].as_str(),
                Some(arm),
                "`pack_path!({arm}, …)` reaches a pack whose pack-id is {:?}",
                value["pack-id"],
            );
        }
    }
}
