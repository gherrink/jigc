//! What the published `jigc` crate carries — its `include` allowlist, read off the
//! listing cargo itself would package (M54 S11; [release.md](../../../implementation/release.md)
//! → What the package carries).
//!
//! `jigc` publishes an allowlist, never its directory: `src/`, the packs, the adapter
//! profiles, the guides, the crate `README.md` generated from the root's, and the license
//! pair. **`tests/` is not published** — its ~1,100 files are this repository's suites,
//! not the product. The fence reads `cargo package --list -p jigc`, the exact set
//! `cargo publish` would upload, so it pins the emitted package rather than a reading of
//! the manifest, and it holds the listing *equal* to the allowlist from both sides: nothing
//! outside it is listed, and every git-tracked file inside its four directories is. One arm
//! reads the packaged bytes themselves: the README in the `jigc` tarball is the generated
//! crate README (M55 S15).
//!
//! Proved red on the parent (before the allowlist existed: `tests/` listed, no README,
//! no license pair) and on an applied mutant adding `"tests/**"` to `include`
//! (recorded in DECISIONS.md → *M54 Inc 6 T4*); the packaged-README arm on its parent
//! (`readme = "../../README.md"` packaged the root's bytes) and on a one-byte mutant of
//! `crates/cli/README.md` (DECISIONS.md → *M55 Increment 10 / T2*).

use std::collections::BTreeSet;
use std::fs;
use std::path::Path;
use std::process::Command;

use crate::support::crate_readme::{SCRIPT, crate_readme_path, generated_crate_readme};
use crate::support::scratch::ScratchDir;

/// The directories the allowlist publishes whole, relative to `crates/cli/`.
const PUBLISHED_DIRS: [&str; 4] = ["src", "packs", "adapters", "guides"];

/// The single files the allowlist publishes at the package root: the generated crate
/// README `readme = "README.md"` names, and the license pair.
const PUBLISHED_FILES: [&str; 3] = ["README.md", "LICENSE-MIT", "LICENSE-APACHE"];

/// The files cargo writes into every package itself, whatever the allowlist says.
const CARGO_GENERATED: [&str; 4] = [
    ".cargo_vcs_info.json",
    "Cargo.lock",
    "Cargo.toml",
    "Cargo.toml.orig",
];

/// Run `cmd` from `crates/cli/`, assert it exits 0, and return its stdout bytes.
fn stdout_of(mut cmd: Command, label: &str) -> Vec<u8> {
    let out = cmd
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
    out.stdout
}

fn run(program: &str, args: &[&str], label: &str) -> String {
    let mut cmd = Command::new(program);
    cmd.args(args);
    String::from_utf8(stdout_of(cmd, label))
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
        "the `jigc` package must carry the crate README and the license pair at its root; \
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

/// The one `.crate` in `dir` whose file name starts with `<name>-` followed by a digit —
/// so `jigc-` never matches `jigc-engine-…`.
fn the_crate_file(dir: &Path, name: &str) -> std::path::PathBuf {
    let prefix = format!("{name}-");
    let found: Vec<_> = fs::read_dir(dir)
        .unwrap_or_else(|err| panic!("read {}: {err}", dir.display()))
        .map(|entry| entry.expect("a readable directory entry").path())
        .filter(|path| {
            path.file_name()
                .and_then(|n| n.to_str())
                .and_then(|n| n.strip_prefix(&prefix))
                .is_some_and(|rest| {
                    rest.ends_with(".crate") && rest.starts_with(|c: char| c.is_ascii_digit())
                })
        })
        .collect();
    assert_eq!(
        found.len(),
        1,
        "expected exactly one `{name}` .crate in {}, found {found:?}",
        dir.display(),
    );
    found.into_iter().next().expect("one crate file")
}

/// **The README crates.io shows for `jigc` is the generated crate README, byte for byte**
/// (M55 S15; [findings-channel.md](../../../design/findings-channel.md) → 8). The arm reads
/// the packaged bytes, not the manifest: it packages both crates into a target directory it
/// owns and extracts `README.md` from the `jigc` tarball. Both crates, because `-p jigc`
/// alone exits 101 against an index that lacks the `=`-pinned `jigc-engine`. `--no-verify`
/// skips the build — what is under test is what the tarball carries. The bytes are compared
/// twice: with the committed `crates/cli/README.md`, which proves `readme` names it, and with
/// what `dev/crate-readme --stdout` prints for today's root, which is what makes a drifted
/// committed file red here. `--allow-dirty` packages the working tree, so the first
/// comparison alone moves with an edit to the file and could never see one. `jigc-engine`
/// stays readme-less.
#[test]
fn the_packaged_readme_is_the_generated_one() {
    let target = ScratchDir::new("package-contents-readme");
    let mut package = Command::new(env!("CARGO"));
    package
        .args([
            "package",
            "--no-verify",
            "-p",
            "jigc-engine",
            "-p",
            "jigc",
            "--offline",
            "--allow-dirty",
        ])
        .env("CARGO_TARGET_DIR", target.path());
    stdout_of(
        package,
        "`cargo package --no-verify -p jigc-engine -p jigc`",
    );
    let packaged = target.path().join("package");

    let jigc = format!("jigc-{}", env!("CARGO_PKG_VERSION"));
    let jigc_crate = packaged.join(format!("{jigc}.crate"));
    let mut extract = Command::new("tar");
    extract
        .arg("-xzOf")
        .arg(&jigc_crate)
        .arg(format!("{jigc}/README.md"));
    let readme = stdout_of(extract, "`tar -xzOf` the packaged `jigc` README");
    let committed = fs::read(crate_readme_path()).unwrap_or_else(|err| {
        panic!(
            "{} must exist — run `{SCRIPT}`: {err}",
            crate_readme_path().display()
        )
    });
    assert!(
        readme == committed,
        "the README packaged in {} is not byte-equal to the committed crate README {} \
         ({} vs {} bytes) — `readme` in crates/cli/Cargo.toml must name `README.md`, the \
         file `{SCRIPT}` writes",
        jigc_crate.display(),
        crate_readme_path().display(),
        readme.len(),
        committed.len(),
    );
    assert!(
        readme == generated_crate_readme().into_bytes(),
        "the README packaged in {} is the committed {}, but that file is not what `{SCRIPT}` \
         generates from the root README — regenerate it with `{SCRIPT}`",
        jigc_crate.display(),
        crate_readme_path().display(),
    );

    let engine_crate = the_crate_file(&packaged, "jigc-engine");
    let mut list = Command::new("tar");
    list.arg("-tzf").arg(&engine_crate);
    let listing = String::from_utf8(stdout_of(list, "`tar -tzf` the `jigc-engine` crate"))
        .expect("a UTF-8 tar listing");
    let readmes: Vec<&str> = listing
        .lines()
        .filter(|path| path.ends_with("/README.md") && path.matches('/').count() == 1)
        .collect();
    assert!(
        readmes.is_empty(),
        "`jigc-engine` carries no readme — the root README describes `jigc` — yet {} lists \
         {readmes:?}",
        engine_crate.display(),
    );
}
