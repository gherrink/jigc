//! The **embed seam** — the one module that knows where the built-in packs sit (M54 S2).
//!
//! It owns both `include_dir!` embeds and the [`EmbeddedPack`] `PackSource` impl over
//! them, and it reads **no** version of its own: the pack version is an explicit input to
//! each constructor. `jigc` supplies its own release version at one site,
//! `crate::pack`'s `EmbeddedPack::new` / `EmbeddedPack::methodology`, because the
//! built-in packs version with the binary (override-reconciliation keys on it —
//! `design/multi-pack.md` → Version ties to the binary). So the day a second frontend
//! lands, this module becomes the `pack-builtin` crate unchanged, and the frontend that
//! depends on it still hands in *its* version, not the crate's.
//!
//! Embed mechanism is `include_dir` (decided 2026-05-31; always-embedded, so what you test
//! is what ships). Each pack tree holds one sub-directory per [`PackResourceKind`]
//! (`workflows/`, `schemas/`, `steps/`, `config/`, `schema-snapshots/`); a resource's
//! [`ResourceId`] is its file stem.
//!
//! **No production code outside this module names a pack directory** —
//! `crates/cli/tests/pack_path_fence.rs` holds it. See `implementation/module-layout.md`
//! → *The seam is prepared now, the crate still is not*.

use crate::pack::kind_dir;
use engine::packsource::{PackError, PackResourceKind, PackSource, ResourceId};
use include_dir::{Dir, include_dir};

/// The on-disk path of a built-in pack, or of a file inside one, for the **tests** of
/// this package — beside the two embeds below, the one place that knows where a pack
/// sits (M54 S2). Every pack path a `cli` test names comes from here.
///
/// - **Literal form**, `pack_path!(<pack-id>, "<pack-relative path>")`: one `concat!`
///   literal, so it serves wherever `include_*!` needs one —
///   `include_bytes!(cli::pack_path!(dev, "schemas/adr.yaml"))`.
/// - **Root form**, `pack_path!(<pack-id>)`: the pack's root directory, no trailing
///   separator — the runtime root a test copies, lists in `packs.yaml` or hands to
///   `JIGC_PACK_DIR`: `Path::new(cli::pack_path!(methodology))`.
///
/// Arms are keyed by the pack's `pack-id` (`dev`, `methodology`), and each pack's root is
/// written once, in its root arm. The expansion reads `CARGO_MANIFEST_DIR` **where it is
/// expanded**, so the macro means something only inside this package (its lib's unit
/// tests and its integration suites); nothing in the shipped binary expands it.
/// `each_root_form_is_the_tree_its_embed_carries` proves each root arm against its embed.
#[macro_export]
macro_rules! pack_path {
    (dev) => {
        concat!(env!("CARGO_MANIFEST_DIR"), "/packs/dev")
    };
    (methodology) => {
        concat!(env!("CARGO_MANIFEST_DIR"), "/packs/methodology")
    };
    ($pack:ident, $rel:literal) => {
        concat!($crate::pack_path!($pack), "/", $rel)
    };
}

/// The built-in dev pack, embedded at compile time.
static DEV: Dir<'static> = include_dir!("$CARGO_MANIFEST_DIR/packs/dev");

/// The methodology pack (the M12 second pack — `roadmap`/`planning`/`completion`/…),
/// embedded at compile time by a **second** `include_dir!`. Pure-YAML data (no `target/`
/// build-tree, so the M20 bloat lesson does not apply); composed in-binary, never
/// extracted. See `design/multi-pack.md` → Embedded second pack; `module-layout.md` →
/// Pack distribution.
static METHODOLOGY: Dir<'static> = include_dir!("$CARGO_MANIFEST_DIR/packs/methodology");

/// `PackSource` over a binary-embedded pack tree. **Field-carrying:** the selected
/// `&'static Dir` is the dev base or the methodology tree, so the two in-binary packs can
/// be composed, and the version each reports is the one it was handed.
pub struct EmbeddedPack {
    dir: &'static Dir<'static>,
    version: &'static str,
}

impl EmbeddedPack {
    /// The dev base pack (the lowest-precedence foundation), reporting `version`.
    pub fn dev_at(version: &'static str) -> Self {
        EmbeddedPack { dir: &DEV, version }
    }

    /// The methodology pack — the second embedded tree, composed dev-highest at
    /// `jigc setup` behind the `compose-embedded-methodology` marker — reporting
    /// `version`.
    pub fn methodology_at(version: &'static str) -> Self {
        EmbeddedPack {
            dir: &METHODOLOGY,
            version,
        }
    }
}

impl PackSource for EmbeddedPack {
    fn pack_version(&self) -> String {
        self.version.to_owned()
    }

    fn list(&self, kind: PackResourceKind) -> Vec<ResourceId> {
        let Some(dir) = self.dir.get_dir(kind_dir(kind)) else {
            return Vec::new();
        };
        let mut ids: Vec<ResourceId> = dir
            .files()
            .filter_map(|f| f.path().file_stem())
            .filter_map(|stem| stem.to_str())
            .map(ResourceId::from)
            .collect();
        ids.sort();
        ids
    }

    fn read(&self, kind: PackResourceKind, id: &ResourceId) -> Result<Vec<u8>, PackError> {
        let dir = self.dir.get_dir(kind_dir(kind));
        let bytes = dir.and_then(|dir| {
            dir.files()
                .find(|f| f.path().file_stem().and_then(|s| s.to_str()) == Some(id.as_str()))
                .map(|f| f.contents().to_vec())
        });
        bytes.ok_or_else(|| PackError::NotFound {
            kind,
            id: id.clone(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// **The version is an input, never read here.** Each selector reports exactly the
    /// version it was handed — so the frontend, not this module, decides what a built-in
    /// pack's version is.
    #[test]
    fn each_selector_reports_the_version_it_was_handed() {
        assert_eq!(
            EmbeddedPack::dev_at("9.8.7-seam").pack_version(),
            "9.8.7-seam"
        );
        assert_eq!(
            EmbeddedPack::methodology_at("0.0.1-seam").pack_version(),
            "0.0.1-seam",
        );
    }

    /// Every file `embed` carries, as `(pack-relative path, bytes)`, sorted by path.
    fn embedded_files(embed: &Dir<'static>) -> Vec<(String, Vec<u8>)> {
        fn walk(dir: &Dir<'static>, out: &mut Vec<(String, Vec<u8>)>) {
            for file in dir.files() {
                let rel = file.path().to_str().expect("a UTF-8 pack path").to_owned();
                out.push((rel, file.contents().to_vec()));
            }
            for sub in dir.dirs() {
                walk(sub, out);
            }
        }
        let mut out = Vec::new();
        walk(embed, &mut out);
        out.sort();
        out
    }

    /// **`pack_path!` reaches the tree the binary embeds, per pack.** Each root form is
    /// read file by file against its `include_dir!` embed: every embedded file sits at
    /// the macro's root with the same bytes, and the literal form's
    /// `config/defaults.yaml` names the pack the arm is keyed by. So a test that takes a
    /// pack path from the macro reads exactly what ships, and an arm pointed at the
    /// wrong pack cannot pass.
    #[test]
    fn each_root_form_is_the_tree_its_embed_carries() {
        let arms: [(&str, &str, &Dir<'static>, &str); 2] = [
            (
                "dev",
                crate::pack_path!(dev),
                &DEV,
                crate::pack_path!(dev, "config/defaults.yaml"),
            ),
            (
                "methodology",
                crate::pack_path!(methodology),
                &METHODOLOGY,
                crate::pack_path!(methodology, "config/defaults.yaml"),
            ),
        ];
        for (pack_id, root, embed, defaults) in arms {
            let files = embedded_files(embed);
            assert!(!files.is_empty(), "the `{pack_id}` embed carries no files");
            for (rel, bytes) in &files {
                let on_disk =
                    std::fs::read(std::path::Path::new(root).join(rel)).unwrap_or_else(|e| {
                        panic!("`pack_path!({pack_id})` = {root} lacks embedded `{rel}`: {e}")
                    });
                assert!(
                    on_disk == *bytes,
                    "`pack_path!({pack_id})` = {root}: `{rel}` differs from the embed",
                );
            }
            let text = std::fs::read_to_string(defaults).expect("read the arm's defaults.yaml");
            let declared: serde_yaml_ng::Value =
                serde_yaml_ng::from_str(&text).expect("parse the arm's defaults.yaml");
            assert_eq!(
                declared["pack-id"].as_str(),
                Some(pack_id),
                "`pack_path!({pack_id}, …)` reaches a pack with another pack-id",
            );
        }
    }

    /// The embedded dev pack carries **only** real pack content — never the `doc-code`
    /// probe's source tree. The probe's sources live outside the `include_dir!` root (the
    /// `jigc` bin's `src/doc_code_probe/`), so the embed sweeps no `probes/` directory.
    /// See module-layout.md → Probe distribution (the de-bloat site).
    #[test]
    fn embedded_pack_carries_no_probes_directory() {
        assert!(
            DEV.get_dir("probes").is_none(),
            "the embedded dev pack must not carry a `probes/` entry — the probe source \
             lives outside the include_dir! root",
        );
    }

    /// The embedded methodology `Dir` carries **only** the resource dirs (`workflows/`,
    /// `schemas/`, `steps/`, `config/`, and — since the M41 F4 v1→v2 `deferral-ledger`
    /// rename — `schema-snapshots/`) — never a `target/` / build subtree. The methodology
    /// tree is pure YAML data, so an embed that swept a build tree would re-introduce the
    /// M20 bloat. Mirrors `embedded_pack_carries_no_probes_directory` for the dev pack.
    #[test]
    fn methodology_dir_carries_no_build_subtree() {
        assert!(
            METHODOLOGY.get_dir("target").is_none(),
            "the embedded methodology Dir must not carry a `target/` build subtree",
        );
        let top_level: Vec<&str> = METHODOLOGY
            .dirs()
            .filter_map(|d| d.path().file_name().and_then(|n| n.to_str()))
            .collect();
        for name in &top_level {
            assert!(
                matches!(
                    *name,
                    "workflows" | "schemas" | "steps" | "config" | "schema-snapshots"
                ),
                "the methodology Dir must hold only the resource dirs; saw `{name}` \
                 among {top_level:?}",
            );
        }
    }
}
