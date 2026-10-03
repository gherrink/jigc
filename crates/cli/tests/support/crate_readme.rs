//! **The crate README's transform, read from the one place it is defined** (M55 S15;
//! [findings-channel.md](../../../../design/findings-channel.md) → 8).
//!
//! `crates/cli/README.md` is generated from the root `README.md` by
//! [`dev/crate-readme`](../../../../dev/crate-readme), which rewrites every relative link to
//! the workspace `repository` URL + `/blob/HEAD/<root path>`. Every fence that speaks about
//! the generated file reads the transform **by running the script**, never by a second
//! implementation in test code: a respelled transform can agree with a fence while the
//! script a maintainer runs writes something else.

use std::path::{Path, PathBuf};
use std::process::Command;

/// The generator, repository-relative.
pub const SCRIPT: &str = "dev/crate-readme";

/// The repository root, two levels above `crates/cli`.
pub fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .ancestors()
        .nth(2)
        .expect("repo root is two levels above crates/cli")
        .to_path_buf()
}

/// The committed, generated crate README.
pub fn crate_readme_path() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("README.md")
}

/// What `dev/crate-readme --stdout` prints for the root README as it stands.
///
/// # Panics
///
/// When the script cannot run, exits non-zero (its stderr is in the message), or prints
/// bytes that are not UTF-8.
pub fn generated_crate_readme() -> String {
    let out = Command::new(repo_root().join(SCRIPT))
        .arg("--stdout")
        .output()
        .unwrap_or_else(|e| panic!("run {SCRIPT} --stdout: {e}"));
    assert!(
        out.status.success(),
        "{SCRIPT} --stdout exited {:?}: {}",
        out.status.code(),
        String::from_utf8_lossy(&out.stderr),
    );
    String::from_utf8(out.stdout).unwrap_or_else(|e| panic!("{SCRIPT} printed non-UTF-8: {e}"))
}
