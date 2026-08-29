//! A **legally re-shaped** on-disk dev-pack copy — the device a suite reaches for when
//! it needs a doctype whose shape differs from the shipped, frozen one.
//!
//! Until M49 three suites simulated a schema-shape change with a project-layer
//! whole-file shadow at `.jigc/config/schemas/<ty>.yaml`, two of them recording in their
//! doc-comments that this is how *"the M33 pack-load freeze assertion is respected"*.
//! It was not respected — it was **bypassed**: the freeze hashed each pack's own shipped
//! schemas and never the cascade-resolved one, so the project layer could reshape a
//! frozen doctype at every surface while the gate stayed silent. That is the hole M49
//! Increment 3 closes, and the rationale dies with it.
//!
//! So the device moves to where a shape change legitimately lives — **the pack**, with
//! its manifest entry re-pinned, which is exactly the act a pack author performs. The
//! re-pinned hash is **recomputed here from the production loader**
//! ([`cli::pack::load_pack_schema`] + [`engine::manifest::schema_hash`]), never
//! hard-coded, so an edit to the shipped schema can never leave a suite asserting over a
//! stale copy.

use std::fs;
use std::path::{Path, PathBuf};

/// The embedded dev pack's on-disk source tree (`crates/cli/pack`).
pub fn dev_pack_tree() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("pack")
}

/// The embedded **methodology** pack's on-disk source tree (`packs/methodology`) — the dev
/// pack's manifest-governed sibling, and the home of the work-doc doctypes whose item blocks
/// a migration reshapes.
pub fn methodology_pack_tree() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("..")
        .join("packs")
        .join("methodology")
}

/// Recursively copy `src` into `dst` (both directories), creating `dst`.
fn copy_tree(src: &Path, dst: &Path) {
    fs::create_dir_all(dst).expect("create copy target dir");
    for entry in fs::read_dir(src).expect("read pack dir") {
        let entry = entry.expect("dir entry");
        let from = entry.path();
        let to = dst.join(entry.file_name());
        if from.is_dir() {
            copy_tree(&from, &to);
        } else {
            fs::copy(&from, &to).expect("copy pack file");
        }
    }
}

/// Copy the dev pack into `root` (created), leaving it byte-identical to the shipped
/// tree — the base every helper below mutates.
pub fn copy_dev_pack(root: &Path) {
    copy_tree(&dev_pack_tree(), root);
}

/// Copy the **methodology** pack into `root` (created), byte-identical to the shipped tree —
/// the base a suite mutates when the doctype it reshapes is a methodology one.
pub fn copy_methodology_pack(root: &Path) {
    copy_tree(&methodology_pack_tree(), root);
}

/// Copy the dev pack into `root`, rewrite doctype `ty`'s schema through `reshape`, and
/// **re-pin** that doctype's `schema-hash` in the copy's `config/schema-manifest.yaml`
/// so the pack passes its own freeze gate. The declared `schema-version` is left
/// untouched: pack-load compares hashes only, and holding the version still is what
/// keeps a consuming suite's stamp fixtures (`at-version` vs `below-version`) meaning
/// what they meant before.
pub fn reshaped_dev_pack(root: &Path, ty: &str, reshape: impl FnOnce(&str) -> String) {
    copy_dev_pack(root);

    let schema_path = root.join("schemas").join(format!("{ty}.yaml"));
    let body = fs::read_to_string(&schema_path).expect("read the copied schema");
    let reshaped = reshape(&body);
    assert_ne!(
        body, reshaped,
        "the reshape must actually change `{ty}.yaml` — a device that reshapes nothing \
         proves nothing"
    );
    fs::write(&schema_path, &reshaped).expect("write the reshaped schema");

    repin_manifest_hash(root, ty);
}

/// Recompute doctype `ty`'s [`engine::manifest::schema_hash`] over the pack at `root`
/// **through the production loader** (field types + the schema-version stamp resolved
/// exactly as the freeze gate resolves them) and write it into that pack's
/// `config/schema-manifest.yaml`.
///
/// Pack-agnostic: it reads whatever pack sits at `root`, so the methodology pack re-pins
/// through the same one implementation the dev pack does — which is the point of computing
/// the hash rather than hard-coding it.
pub fn repin_manifest_hash(root: &Path, ty: &str) {
    let pack = cli::pack::FilesystemPack::new(root.to_path_buf());
    let bytes = fs::read(root.join("schemas").join(format!("{ty}.yaml")))
        .expect("read the reshaped schema");
    let schema = cli::pack::load_pack_schema(&pack, &bytes).expect("the reshaped schema loads");
    let hash = engine::manifest::schema_hash(&schema);

    let path = root.join("config").join("schema-manifest.yaml");
    let manifest = fs::read_to_string(&path).expect("read the copied schema-manifest.yaml");
    let mut out = String::new();
    let mut in_entry = false;
    let mut hit = false;
    for line in manifest.lines() {
        if line.starts_with("  - type: ") {
            in_entry = line.trim_end() == format!("  - type: {ty}");
        }
        if in_entry && line.starts_with("    schema-hash: ") {
            out.push_str(&format!("    schema-hash: {hash}"));
            hit = true;
            in_entry = false;
        } else {
            out.push_str(line);
        }
        out.push('\n');
    }
    assert!(hit, "the manifest must declare a `{ty}` entry to re-pin");
    fs::write(&path, out).expect("write the re-pinned schema-manifest.yaml");
}

/// Copy the dev pack into `root` and **drop its freeze manifest** — the pack-level,
/// wholesale opt-out (`design/corpus-migration.md` → The freeze): a pack that declares
/// nothing frozen freezes nothing, at either layer, so a project-layer whole-file schema
/// shadow of one of its doctypes is honoured with no freeze event.
pub fn manifest_less_dev_pack(root: &Path) {
    copy_dev_pack(root);
    fs::remove_file(root.join("config").join("schema-manifest.yaml"))
        .expect("drop the freeze manifest");
}
