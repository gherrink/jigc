//! Ship the dev pack's `doc-code` probe into the build tree **alongside the `jigc`
//! binary**, so the production probe-resolution path
//! ([`invoke::doc_code_program`](src/invoke.rs) → `<jigc-bin-dir>/doc-code`) finds a
//! runnable probe after a normal `cargo build` — **no `JIGC_DOC_CODE_PROBE` override
//! required**.
//!
//! **Why a build script and not a workspace member / second `[[bin]]`.** The probe is a
//! standalone executable that lives **outside** the workspace by invariant
//! ([module-layout.md](../../implementation/module-layout.md) → Probe boundary): its
//! tree-sitter deps must never enter the engine/cli lock graph. Making it a member or a
//! `cli` `[[bin]]` would pull tree-sitter into that lock. So we build the probe as its
//! **own detached cargo workspace** (a nested `cargo build --manifest-path …`) and copy
//! the resulting executable next to `jigc`. The engine stays shell-free — this is a
//! CLI-side build concern, not a runtime one.
//!
//! **Where it lands.** Cargo places the final `jigc` binary at `<target>/<profile>/jigc`
//! and hands this script `OUT_DIR = <target>/<profile>/build/cli-<hash>/out`. Walking
//! three parents up from `OUT_DIR` reaches `<target>/<profile>/` — the directory the
//! built `jigc` resolves its sibling probe against (`current_exe().parent()`), and the
//! same directory `CARGO_BIN_EXE_jigc` points integration tests at. This is the standard
//! build-script idiom for "place an artifact next to the final binary"; it tracks
//! cross-compilation automatically since `OUT_DIR` sits under the active profile dir.

use std::path::{Path, PathBuf};
use std::process::Command;

fn main() {
    let manifest_dir =
        PathBuf::from(std::env::var_os("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR"));
    let probe_manifest = manifest_dir.join("probes/doc-code/Cargo.toml");

    // Rerun only when the probe's own sources change — not on every `cli` rebuild.
    let probe_src = manifest_dir.join("probes/doc-code/src");
    println!("cargo::rerun-if-changed={}", probe_manifest.display());
    println!("cargo::rerun-if-changed={}", probe_src.display());

    let profile = std::env::var("PROFILE").expect("PROFILE");
    // The probe builds in its own target dir (its detached workspace), then we copy the
    // produced binary out. Pin its profile to ours so a release `jigc` ships a release
    // probe.
    let probe_target = manifest_dir.join("probes/doc-code/target");
    let mut cargo = Command::new(std::env::var_os("CARGO").unwrap_or_else(|| "cargo".into()));
    cargo
        .args(["build", "--quiet", "--manifest-path"])
        .arg(&probe_manifest)
        .arg("--target-dir")
        .arg(&probe_target);
    if profile == "release" {
        cargo.arg("--release");
    }
    let status = cargo
        .status()
        .expect("invoke cargo build for the doc-code probe");
    assert!(status.success(), "building the doc-code probe failed");

    let probe_bin = probe_target.join(&profile).join(exe_name("doc-code"));
    assert!(
        probe_bin.is_file(),
        "doc-code probe binary missing at {probe_bin:?} after build"
    );

    // `OUT_DIR = <target>/<profile>/build/cli-<hash>/out`; three parents up is the
    // directory holding the final `jigc` binary.
    let out_dir = PathBuf::from(std::env::var_os("OUT_DIR").expect("OUT_DIR"));
    let bin_dir = out_dir
        .parent()
        .and_then(Path::parent)
        .and_then(Path::parent)
        .expect("resolve the jigc binary directory from OUT_DIR");
    let dest = bin_dir.join(exe_name("doc-code"));
    std::fs::copy(&probe_bin, &dest)
        .unwrap_or_else(|e| panic!("copy doc-code probe to {dest:?}: {e}"));
}

/// The platform's executable file name for `stem` (`.exe` on Windows, bare elsewhere).
fn exe_name(stem: &str) -> String {
    if cfg!(windows) {
        format!("{stem}.exe")
    } else {
        stem.to_string()
    }
}
