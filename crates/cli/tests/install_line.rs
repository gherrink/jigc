//! The install line's version requirement, proved on the library cargo matches with
//! (M54 S13 + O6; [release.md](../../../implementation/release.md) → Installing).
//!
//! The one install line is `cargo install jigc --version '^1.0.0-rc.1' --locked`, and it
//! is static: no release rewrites it. That holds only if the requirement admits every
//! `1.0.0-rc.N` prerelease and every later `1.x`, and never the `0.0.0` placeholder that
//! held the crate name. Prerelease matching is the part people get wrong — a caret
//! requirement without a prerelease tag admits no prerelease at all — so the build
//! verifies it here, on the `semver` crate, **before any doc relies on it**. This is the
//! first of two halves: the real registry install after the first publish is the full
//! proof (release.md → Verifying a publish).
//!
//! Proved red on an applied mutant: with `INSTALL_REQUIREMENT` set to `>=0.0.0`, the
//! `0.0.0` arm fails (and so does the `1.0.0-rc.22` arm, which a plain range never admits).

use std::path::PathBuf;

use semver::{Version, VersionReq};

use crate::support::install_line::{install_line_in, quickstart_install_line};

/// The requirement the install line carries, held once.
const INSTALL_REQUIREMENT: &str = "^1.0.0-rc.1";

fn requirement() -> VersionReq {
    VersionReq::parse(INSTALL_REQUIREMENT).expect("the install requirement parses")
}

fn admits(version: &str) -> bool {
    requirement().matches(&Version::parse(version).expect("the probe version parses"))
}

#[test]
fn the_requirement_admits_every_release_candidate_and_every_later_one_x() {
    for version in ["1.0.0-rc.22", "1.0.0", "1.3.0"] {
        assert!(
            admits(version),
            "`{INSTALL_REQUIREMENT}` must match {version}, or the install line stops \
             resolving that release"
        );
    }
}

#[test]
fn the_requirement_never_admits_the_name_holding_placeholder() {
    assert!(
        !admits("0.0.0"),
        "`{INSTALL_REQUIREMENT}` must not match 0.0.0 — the placeholder that held the \
         crate name is not jigc"
    );
}

/// The requirement this suite proves is the one the owner of the line carries. The
/// constant above is proved on `semver`; this arm closes the gap between that proof and
/// the bytes an adopter copies — `QUICKSTART.md`'s own `--version` argument, read through
/// the one extractor every install-line fence shares.
#[test]
fn quickstart_carries_the_requirement_this_suite_proves() {
    let owned = quickstart_install_line();
    assert_eq!(
        owned.requirement, INSTALL_REQUIREMENT,
        "QUICKSTART.md's install line (`{}`) names a requirement other than the one proved \
         here — move them together, and re-prove the matching rule",
        owned.line,
    );
}

/// The root README carries **the one allowed copy** of the line (S13): byte-identical to
/// QUICKSTART's, read out of both docs through the same extractor, so each must hold
/// exactly one install line under `## Install` and the two must agree to the byte. A
/// one-byte drift in the copy turns this red (recorded in DECISIONS.md → *M54 Inc 6 T3*).
#[test]
fn the_readme_carries_quickstarts_line_byte_identical() {
    let readme = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../README.md");
    let text = std::fs::read_to_string(&readme)
        .unwrap_or_else(|err| panic!("{} must be readable: {err}", readme.display()));
    let copy = install_line_in(&text, "README.md");
    let owned = quickstart_install_line();
    assert_eq!(
        copy.line, owned.line,
        "README.md's install line is the one allowed copy of QUICKSTART.md's and must match \
         it byte-for-byte — change the line in QUICKSTART.md and copy it here",
    );
}
