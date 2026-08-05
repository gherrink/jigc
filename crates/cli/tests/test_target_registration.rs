//! The registration guard for the consolidated test targets.
//!
//! `crates/cli/Cargo.toml` sets `autotests = false` so the ~250 suite files under
//! `crates/cli/tests/` are compiled into ~10 group binaries instead of one cargo
//! target each (a source change relinked 256 binaries — a measured 7.7-minute link
//! wave — before the consolidation). The cost of turning auto-discovery off is that
//! a **newly added suite file is silently never run** unless a group root pulls it
//! in: the file compiles nowhere, so nothing reddens.
//!
//! This test is the fence. It asserts a **bijection** between the suite files on
//! disk and the `#[path = "../<name>.rs"] mod <name>;` lines across the group roots
//! — every suite registered exactly once, and every registration pointing at a file
//! that exists. It is the same shape as the pack/describe inventory assertions: the
//! surface is generated, so the count is enforced rather than trusted.

use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};

fn tests_dir() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("tests")
}

/// Every `tests/*.rs` file — the suites that must each be owned by exactly one group.
fn suite_files() -> Vec<String> {
    let mut v: Vec<String> = fs::read_dir(tests_dir())
        .expect("tests/ is readable")
        .filter_map(|e| {
            let p = e.expect("dir entry").path();
            if !p.is_file() {
                return None;
            }
            let name = p.file_name()?.to_str()?.to_string();
            name.strip_suffix(".rs").map(str::to_string)
        })
        .collect();
    v.sort();
    v
}

/// Each group root's `mod` registrations, keyed suite -> owning group.
fn registrations() -> BTreeMap<String, Vec<String>> {
    let mut owners: BTreeMap<String, Vec<String>> = BTreeMap::new();
    let groups = tests_dir().join("groups");
    for entry in fs::read_dir(&groups).expect("tests/groups/ exists") {
        let path = entry.expect("dir entry").path();
        if path.extension().and_then(|e| e.to_str()) != Some("rs") {
            continue;
        }
        let group = path
            .file_stem()
            .and_then(|s| s.to_str())
            .expect("group stem")
            .to_string();
        let body = fs::read_to_string(&path).expect("group root is readable");
        for line in body.lines() {
            let line = line.trim();
            // `#[path = "../<suite>.rs"]` — the support root uses `../support/mod.rs`,
            // which is not a suite and is skipped by the `../` + `.rs` shape below.
            if let Some(rest) = line.strip_prefix("#[path = \"../")
                && let Some(file) = rest.strip_suffix(".rs\"]")
                && !file.contains('/')
            {
                owners
                    .entry(file.to_string())
                    .or_default()
                    .push(group.clone());
            }
        }
    }
    owners
}

#[test]
fn every_suite_file_is_registered_in_exactly_one_group() {
    let suites = suite_files();
    let owners = registrations();

    let unregistered: Vec<&String> = suites.iter().filter(|s| !owners.contains_key(*s)).collect();
    assert!(
        unregistered.is_empty(),
        "these suite files are compiled into NO test target, so they never run — add each to a \
         group root under crates/cli/tests/groups/ as `#[path = \"../<name>.rs\"] mod <name>;`: \
         {unregistered:?}"
    );

    let duplicated: Vec<(&String, &Vec<String>)> =
        owners.iter().filter(|(_, g)| g.len() > 1).collect();
    assert!(
        duplicated.is_empty(),
        "these suite files are registered in more than one group, so their tests run twice \
         under two binary names: {duplicated:?}"
    );
}

// NOTE — the shared `doc-code` probe is deliberately NOT fenced here. Dozens of suites
// shell out to cargo against the detached `crates/cli/probes/doc-code` workspace, and
// they all drive the same `target/` directory, so "at most one builder per group" is not
// a rule this tree can satisfy without undoing the consolidation. The race — one suite
// replacing `target/debug/doc-code` while a co-resident suite executes it — is closed at
// the source instead, by building the probe **once before** the suite runs (the gate's
// prebuild step, `implementation/dev-workflow.md` → Gate): after that every in-test
// `cargo build` for the same manifest is an up-to-date no-op that rewrites nothing.

/// A suite that mutates **process-global** environment state is only sound while it is
/// the sole test in its process. Before the M47 consolidation every suite owned its own
/// binary, so this held by construction and two suites relied on it in writing —
/// `store_sweep_acceptance` documented itself as *"the sole in-process writer"* of
/// `JIGC_DOC_CODE_PROBE`, and `trial_corpus_states` poisons `JIGC_PACK_DIR` in the
/// parent on purpose. Consolidation falsified that assumption for free, and the first
/// full run caught it: `severity_tuning`, co-resident for the first time, picked up a
/// deliberately-crashing probe path and failed.
///
/// So the property is fenced where membership is decided rather than left to the
/// `g_solo_` naming convention: whichever suites mutate the environment, their group
/// must hold exactly one suite. A new env-mutating suite dropped into a shared group
/// reddens here instead of corrupting a neighbour months later.
#[test]
fn an_env_mutating_suite_is_alone_in_its_target() {
    let group_size: BTreeMap<String, usize> =
        registrations()
            .values()
            .flatten()
            .fold(BTreeMap::new(), |mut acc, g| {
                *acc.entry(g.clone()).or_default() += 1;
                acc
            });

    let mut violations = Vec::new();
    for (suite, groups) in registrations() {
        // This file names the two calls it searches for, in this function and in the
        // doc comment above it, so a literal scan matches the scanner. It mutates no
        // environment itself — it only reads the tree.
        if suite == "test_target_registration" {
            continue;
        }
        let body = fs::read_to_string(tests_dir().join(format!("{suite}.rs")))
            .expect("registered suite is readable");
        if !body.contains("env::set_var") && !body.contains("env::remove_var") {
            continue;
        }
        for g in groups {
            let n = group_size.get(&g).copied().unwrap_or(0);
            if n > 1 {
                violations.push(format!(
                    "{suite} mutates process env but shares `{g}` with {} other suite(s)",
                    n - 1
                ));
            }
        }
    }
    assert!(
        violations.is_empty(),
        "process-global env mutation leaks to every co-resident suite in the same test \
         binary — give each of these its own `[[test]]` target (see \
         crates/cli/tests/groups/g_solo_*.rs): {violations:#?}"
    );
}

#[test]
fn every_registration_points_at_a_file_that_exists() {
    let suites = suite_files();
    let owners = registrations();
    let dangling: Vec<(&String, &Vec<String>)> =
        owners.iter().filter(|(s, _)| !suites.contains(s)).collect();
    assert!(
        dangling.is_empty(),
        "these group registrations name a suite file that no longer exists — delete the \
         `mod` line with the file: {dangling:?}"
    );
}
