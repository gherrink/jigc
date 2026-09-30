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

use semver::{Version, VersionReq};

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
