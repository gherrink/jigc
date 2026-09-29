//! The registration guard for the consolidated test targets.
//!
//! `crates/cli/Cargo.toml` sets `autotests = false` so the ~250 suite files under
//! `crates/cli/tests/` are compiled into 12 group targets instead of one cargo
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
//!
//! The chain is only as strong as its first link, so the fence reads the group roots
//! from the **`[[test]]` list cargo actually compiles**, never from `tests/groups/`
//! on disk: a root dropped into that directory without its manifest entry looks like
//! a legitimate owner while compiling nowhere — the very hole this file exists to
//! close, one level up. The three arms after the originals fence the three places the
//! chain was trusted rather than checked: the manifest ↔ disk bijection over the
//! roots, the aggregator ↔ disk bijection over submodule directories like
//! `pinned_facts/`, and the count both this file and the manifest state in prose.

use crate::support::root_walk;
use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::{Path, PathBuf};

fn crate_dir() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).to_path_buf()
}

fn tests_dir() -> PathBuf {
    crate_dir().join("tests")
}

fn manifest_path() -> PathBuf {
    crate_dir().join("Cargo.toml")
}

/// The `[[test]]` targets the manifest declares, as `(name, path)` in declaration
/// order — the authority on what `cargo test` compiles.
fn declared_test_targets() -> Vec<(String, String)> {
    let body = fs::read_to_string(manifest_path()).expect("crates/cli/Cargo.toml is readable");
    let mut out: Vec<(String, String)> = Vec::new();
    let mut current: Option<(Option<String>, Option<String>)> = None;
    fn flush(cur: Option<(Option<String>, Option<String>)>, out: &mut Vec<(String, String)>) {
        if let Some((name, path)) = cur {
            out.push((
                name.expect("a [[test]] target declares a name"),
                path.expect("a [[test]] target declares a path"),
            ));
        }
    }
    for line in body.lines() {
        let line = line.trim();
        if line.starts_with('#') {
            continue;
        }
        if line.starts_with('[') {
            flush(current.take(), &mut out);
            if line == "[[test]]" {
                current = Some((None, None));
            }
            continue;
        }
        if let Some((name, path)) = current.as_mut()
            && let Some((key, value)) = line.split_once('=')
        {
            let value = value.trim().trim_matches('"').to_string();
            match key.trim() {
                "name" => *name = Some(value),
                "path" => *path = Some(value),
                _ => {}
            }
        }
    }
    flush(current.take(), &mut out);
    out
}

/// Every `tests/*.rs` file — the suites that must each be owned by exactly one group.
fn suite_files() -> Vec<String> {
    let mut v: Vec<String> = root_walk::files_in(&tests_dir(), root_walk::ext("rs"))
        .iter()
        .filter_map(|p| Some(p.file_stem()?.to_str()?.to_string()))
        .collect();
    v.sort();
    v
}

/// Each group root's `mod` registrations, keyed suite -> owning group.
///
/// The roots come from the manifest's `[[test]]` list, not from `tests/groups/`, so a
/// suite whose only owner is an undeclared root reads as unregistered here — as it in
/// fact is. `every_group_root_is_declared_as_a_test_target` names the stray root.
fn registrations() -> BTreeMap<String, Vec<String>> {
    let mut owners: BTreeMap<String, Vec<String>> = BTreeMap::new();
    for (group, rel) in declared_test_targets() {
        let body = fs::read_to_string(crate_dir().join(&rel)).unwrap_or_else(|e| {
            panic!("declared [[test]] target `{group}` names {rel}, which is unreadable: {e}")
        });
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

// NOTE — no suite builds the `doc-code` probe, so there is no builder to fence here.
// A suite that needs the real probe drives the real `jigc` with the `JIGC_DOC_CODE_PROBE`
// override removed, so the probe resolves through the production path; the negative legs
// point the override at an absent path or a stub the suite compiles itself. The race the
// consolidation exposed — one suite rebuilding the shared probe executable while a
// co-resident suite ran it — therefore has no rebuilding suite left
// (`DECISIONS.md` → M54 Settle, S1). The probe's own unit tests run as the `jigc` bin's
// unit tests, and the probe any suite spawns is `jigc` itself.

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

/// The fence above reads its group roots from the manifest, which leaves one hole open
/// at the level below: a root file that exists in `tests/groups/` while `Cargo.toml`
/// declares no `[[test]]` target for it. Nothing compiles it, so every suite it lists
/// is silently never run — the exact failure `autotests = false` bought, displaced one
/// level up. The reverse is the sibling: a declared target whose root file is gone
/// fails the build for everyone, but naming it here says *why* in one line.
#[test]
fn every_group_root_is_declared_as_a_test_target() {
    let declared: BTreeSet<String> = declared_test_targets()
        .into_iter()
        .map(|(_, path)| path)
        .collect();
    let on_disk: BTreeSet<String> =
        root_walk::files_in(&tests_dir().join("groups"), root_walk::ext("rs"))
            .iter()
            .filter_map(|p| Some(format!("tests/groups/{}", p.file_name()?.to_str()?)))
            .collect();

    let undeclared: Vec<&String> = on_disk.difference(&declared).collect();
    assert!(
        undeclared.is_empty(),
        "these group roots exist on disk but no `[[test]]` target in crates/cli/Cargo.toml \
         names them, so cargo compiles neither the root nor any suite it registers — add a \
         `[[test]]` entry for each, or delete the file: {undeclared:?}"
    );

    let missing: Vec<&String> = declared.difference(&on_disk).collect();
    assert!(
        missing.is_empty(),
        "these `[[test]]` targets in crates/cli/Cargo.toml name a group root that does not \
         exist: {missing:?}"
    );
}

/// Directories under `tests/` that a sibling `<name>.rs` aggregates — today only
/// `pinned_facts/`. Derived rather than listed, so a second such directory joins the
/// fence by existing. `support/` (a `mod.rs` directory) and the data directories have
/// no `.rs` sibling and are not submodule aggregations.
fn aggregated_dirs() -> Vec<String> {
    let mut v: Vec<String> = fs::read_dir(tests_dir())
        .expect("tests/ is readable")
        .filter_map(|e| {
            let p = e.expect("dir entry").path();
            if !p.is_dir() {
                return None;
            }
            let name = p.file_name()?.to_str()?.to_string();
            tests_dir()
                .join(format!("{name}.rs"))
                .is_file()
                .then_some(name)
        })
        .collect();
    v.sort();
    v
}

/// The same bijection one level deeper. `tests/pinned_facts.rs` declares its repro
/// blocks by hand with `#[path = "pinned_facts/<name>.rs"]`, and `suite_files()` scans
/// only `tests/*.rs` — so until now a repro block dropped into that directory compiled
/// nowhere and reddened nothing, which is precisely the class of silence the pinned
/// facts exist to prevent.
#[test]
fn every_aggregated_submodule_is_declared_by_its_aggregator() {
    let dirs = aggregated_dirs();
    assert!(
        !dirs.is_empty(),
        "no `tests/<dir>/` has a sibling `tests/<dir>.rs` aggregator any more — if that idiom \
         is genuinely gone, delete this fence; if it was renamed, teach `aggregated_dirs()` \
         the new shape rather than leaving a fence that checks nothing"
    );

    for dir in dirs {
        let on_disk: BTreeSet<String> =
            root_walk::files_in(&tests_dir().join(&dir), root_walk::ext("rs"))
                .iter()
                .filter_map(|p| Some(p.file_stem()?.to_str()?.to_string()))
                .collect();

        let body = fs::read_to_string(tests_dir().join(format!("{dir}.rs")))
            .expect("aggregator is readable");
        let prefix = format!("#[path = \"{dir}/");
        let declared: BTreeSet<String> = body
            .lines()
            .filter_map(|line| {
                line.trim()
                    .strip_prefix(&prefix)?
                    .strip_suffix(".rs\"]")
                    .map(str::to_string)
            })
            .collect();

        let undeclared: Vec<&String> = on_disk.difference(&declared).collect();
        assert!(
            undeclared.is_empty(),
            "these files under tests/{dir}/ are declared by no `#[path = \"{dir}/<name>.rs\"] \
             mod <name>;` line in tests/{dir}.rs, so they compile nowhere and never run: \
             {undeclared:?}"
        );

        let dangling: Vec<&String> = declared.difference(&on_disk).collect();
        assert!(
            dangling.is_empty(),
            "tests/{dir}.rs declares these submodules, but no such file exists under \
             tests/{dir}/: {dangling:?}"
        );
    }
}

/// A count stated in prose is a receipt, and a receipt outlives its contract: both
/// `Cargo.toml`'s header and this file's module doc said **ten** group binaries while
/// twelve `[[test]]` targets were declared — the same drift the wave's other fences
/// close on the pack surface, here in our own harness. So the number is asserted where
/// it is written: each file must contain the current count spelled as a decimal, and
/// the needle is built at run time from the manifest, so an edit to either statement
/// reddens instead of quietly disagreeing with what cargo compiles.
#[test]
fn the_stated_target_count_matches_the_declared_targets() {
    let needle = format!("{} group targets", declared_test_targets().len());
    for rel in ["Cargo.toml", "tests/test_target_registration.rs"] {
        let body = fs::read_to_string(crate_dir().join(rel)).expect("stating file is readable");
        assert!(
            body.contains(&needle),
            "crates/cli/{rel} no longer states the current test-target count: it must spell \
             out `{needle}` somewhere in its prose, so the number a reader trusts is the \
             number cargo compiles"
        );
    }
}
