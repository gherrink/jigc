//! What the published `jigc` crate carries — its `include` allowlist, read off the
//! listing cargo itself would package (M54 S11; [release.md](../../../implementation/release.md)
//! → What the package carries).
//!
//! `jigc` publishes an allowlist, never its directory: `src/`, the packs, the adapter
//! profiles, the guides, the root `README.md` and the license pair. **`tests/` is not
//! published** — its ~1,100 files are this repository's suites, not the product. The
//! fence reads `cargo package --list -p jigc`, the exact set `cargo publish` would upload,
//! so it pins the emitted package rather than a reading of the manifest, and it holds the
//! listing *equal* to the allowlist from both sides: nothing outside it is listed, and
//! every git-tracked file inside its four directories is.
//!
//! Proved red on the parent (before the allowlist existed: `tests/` listed, no README,
//! no license pair) and on an applied mutant adding `"tests/**"` to `include`
//! (recorded in DECISIONS.md → *M54 Inc 6 T4*).

use std::collections::BTreeSet;
use std::process::Command;

/// The directories the allowlist publishes whole, relative to `crates/cli/`.
const PUBLISHED_DIRS: [&str; 4] = ["src", "packs", "adapters", "guides"];

/// The single files the allowlist publishes at the package root: the readme cargo copies
/// in from `readme = "../../README.md"`, and the license pair.
const PUBLISHED_FILES: [&str; 3] = ["README.md", "LICENSE-MIT", "LICENSE-APACHE"];

/// The files cargo writes into every package itself, whatever the allowlist says.
const CARGO_GENERATED: [&str; 4] = [
    ".cargo_vcs_info.json",
    "Cargo.lock",
    "Cargo.toml",
    "Cargo.toml.orig",
];

fn run(program: &str, args: &[&str], label: &str) -> String {
    let out = Command::new(program)
        .args(args)
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .output()
        .unwrap_or_else(|err| panic!("{label}: could not run: {err}"));
    assert!(
        out.status.success(),
        "{label}: exited {}\nstderr:\n{}\nstdout:\n{}",
        out.status,
        String::from_utf8_lossy(&out.stderr),
        String::from_utf8_lossy(&out.stdout),
    );
    String::from_utf8(out.stdout)
        .unwrap_or_else(|err| panic!("{label}: stdout is not UTF-8: {err}"))
}

/// Every path `cargo publish` would upload for `jigc`, relative to the package root.
fn listing() -> BTreeSet<String> {
    run(
        env!("CARGO"),
        &[
            "package",
            "--list",
            "-p",
            "jigc",
            "--offline",
            "--allow-dirty",
        ],
        "`cargo package --list -p jigc`",
    )
    .lines()
    .map(str::to_string)
    .collect()
}

fn under_a_published_dir(path: &str) -> bool {
    PUBLISHED_DIRS.iter().any(|dir| {
        path.strip_prefix(dir)
            .is_some_and(|rest| rest.starts_with('/'))
    })
}

#[test]
fn nothing_under_tests_is_published() {
    let leaked: Vec<String> = listing()
        .into_iter()
        .filter(|path| path.starts_with("tests/"))
        .collect();
    assert!(
        leaked.is_empty(),
        "the `jigc` package lists {} file(s) under `tests/` — the suites are not the product \
         and are never published; first few: {:?}",
        leaked.len(),
        &leaked[..leaked.len().min(5)],
    );
}

#[test]
fn every_listed_path_is_on_the_allowlist() {
    let outside: Vec<String> = listing()
        .into_iter()
        .filter(|path| {
            !under_a_published_dir(path)
                && !PUBLISHED_FILES.contains(&path.as_str())
                && !CARGO_GENERATED.contains(&path.as_str())
        })
        .collect();
    assert!(
        outside.is_empty(),
        "the `jigc` package lists paths outside its allowlist ({PUBLISHED_DIRS:?}, \
         {PUBLISHED_FILES:?}, cargo's own {CARGO_GENERATED:?}): {outside:?}",
    );
}

#[test]
fn the_readme_and_the_license_pair_are_published() {
    let listed = listing();
    let missing: Vec<&str> = PUBLISHED_FILES
        .into_iter()
        .filter(|file| !listed.contains(*file))
        .collect();
    assert!(
        missing.is_empty(),
        "the `jigc` package must carry the root README and the license pair at its root; \
         missing: {missing:?}",
    );
}

#[test]
fn every_tracked_file_under_the_published_dirs_is_published() {
    let listed = listing();
    let mut args = vec!["ls-files", "--"];
    args.extend(PUBLISHED_DIRS);
    let tracked = run(
        "git",
        &args,
        "`git ls-files` under the published directories",
    );
    let unlisted: Vec<&str> = tracked
        .lines()
        .filter(|path| !listed.contains(*path))
        .collect();
    assert!(
        !tracked.is_empty(),
        "`git ls-files` found nothing under {PUBLISHED_DIRS:?} — the fence would be vacuous",
    );
    assert!(
        unlisted.is_empty(),
        "git tracks files under {PUBLISHED_DIRS:?} that the `jigc` package does not carry: \
         {unlisted:?}",
    );
}
