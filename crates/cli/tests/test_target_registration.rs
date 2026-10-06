//! The registration guard for the consolidated test targets.
//!
//! `crates/cli/Cargo.toml` sets `autotests = false` so the ~250 suite files under
//! `crates/cli/tests/` — and, since 2026-10-06, the repository's own tooling suites
//! outside `crates/` — are compiled into 13 group targets instead of one cargo
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
//!
//! **The suite homes are read off the same list** ([`test_homes`]): a home is the
//! directory above the `groups/` a declared root sits in, so every arm below runs over
//! each home there is — the crate's `tests/` and the tooling suites' directory today, a
//! third by its manifest entry. A suite is keyed by its **file**, and a registration is
//! resolved to the file its `#[path]` names from the root that carries it, so a file
//! registered once in each of two homes' roots is one file with two owners. Until
//! 2026-10-06 the fence took `crates/cli/tests/` for the only home, and a suite moved out
//! of it would have been fenced by nothing ([dev-workflow.md](../../../implementation/dev-workflow.md)
//! → Gate, *Where the tooling's own tests live*).

use crate::support::root_walk;
use crate::support::test_homes;
use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::{Path, PathBuf};

/// The repository root — two levels above the `jigc` package — as the homes spell it.
fn repo_root() -> PathBuf {
    test_homes::normalize(&Path::new(env!("CARGO_MANIFEST_DIR")).join("../.."))
        .canonicalize()
        .expect("the repository root is reachable from the cli crate")
}

/// `path` as a reader would type it: relative to the repository root.
fn shown(path: &Path) -> String {
    path.strip_prefix(repo_root())
        .unwrap_or(path)
        .display()
        .to_string()
}

/// Every `<home>/*.rs` file of every suite home — the suites that must each be owned by
/// exactly one group.
fn suite_files() -> BTreeSet<PathBuf> {
    test_homes::suite_homes()
        .iter()
        .flat_map(|home| root_walk::files_in(home, root_walk::ext("rs")))
        .collect()
}

/// Each group root's `mod` registrations, keyed suite file -> owning groups.
///
/// The roots come from the manifest's `[[test]]` list, not from a `groups/` directory, so
/// a suite whose only owner is an undeclared root reads as unregistered here — as it in
/// fact is. `every_group_root_is_declared_as_a_test_target` names the stray root.
///
/// A registration is whatever file a root's `#[path = "…"]` resolves to, read from that
/// root's own directory. One that lands **directly in a suite home** names a suite,
/// whichever home the root itself belongs to; one that lands deeper — the shared
/// `support/mod.rs` — is not a suite and is skipped.
fn registrations() -> BTreeMap<PathBuf, Vec<String>> {
    let homes = test_homes::suite_homes();
    let mut owners: BTreeMap<PathBuf, Vec<String>> = BTreeMap::new();
    for target in test_homes::declared_test_targets() {
        let body = fs::read_to_string(&target.root).unwrap_or_else(|e| {
            panic!(
                "declared [[test]] target `{}` names {}, which is unreadable: {e}",
                target.name,
                shown(&target.root),
            )
        });
        let root_dir = target.root.parent().expect("a group root has a parent");
        for line in body.lines() {
            if let Some(rel) = line
                .trim()
                .strip_prefix("#[path = \"")
                .and_then(|rest| rest.strip_suffix("\"]"))
            {
                let file = test_homes::normalize(&root_dir.join(rel));
                if file
                    .parent()
                    .is_some_and(|dir| homes.iter().any(|h| h == dir))
                {
                    owners.entry(file).or_default().push(target.name.clone());
                }
            }
        }
    }
    owners
}

#[test]
fn every_suite_file_is_registered_in_exactly_one_group() {
    let suites = suite_files();
    let owners = registrations();

    let unregistered: Vec<String> = suites
        .iter()
        .filter(|s| !owners.contains_key(*s))
        .map(|s| shown(s))
        .collect();
    assert!(
        unregistered.is_empty(),
        "these suite files are compiled into NO test target, so they never run — add each to a \
         group root under its own directory's `groups/` as `#[path = \"../<name>.rs\"] mod \
         <name>;`: {unregistered:?}"
    );

    let duplicated: Vec<(String, &Vec<String>)> = owners
        .iter()
        .filter(|(_, g)| g.len() > 1)
        .map(|(s, g)| (shown(s), g))
        .collect();
    assert!(
        duplicated.is_empty(),
        "these suite files are registered more than once, so their tests run twice under \
         two binary names: {duplicated:?}"
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
        if suite
            .file_name()
            .is_some_and(|name| name == "test_target_registration.rs")
        {
            continue;
        }
        let body = fs::read_to_string(&suite).expect("registered suite is readable");
        if !body.contains("env::set_var") && !body.contains("env::remove_var") {
            continue;
        }
        for g in groups {
            let n = group_size.get(&g).copied().unwrap_or(0);
            if n > 1 {
                violations.push(format!(
                    "{} mutates process env but shares `{g}` with {} other suite(s)",
                    shown(&suite),
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
    let dangling: Vec<(String, &Vec<String>)> = owners
        .iter()
        .filter(|(s, _)| !suites.contains(*s))
        .map(|(s, g)| (shown(s), g))
        .collect();
    assert!(
        dangling.is_empty(),
        "these group registrations name a suite file that no longer exists — delete the \
         `mod` line with the file: {dangling:?}"
    );
}

/// The fence above reads its group roots from the manifest, which leaves one hole open
/// at the level below: a root file that exists in a home's `groups/` while `Cargo.toml`
/// declares no `[[test]]` target for it. Nothing compiles it, so every suite it lists
/// is silently never run — the exact failure `autotests = false` bought, displaced one
/// level up. The reverse is the sibling: a declared target whose root file is gone
/// fails the build for everyone, but naming it here says *why* in one line.
///
/// Read per home, and a home is found through a declared root — so a `groups/` directory
/// no declared root sits in is a directory this arm never opens. That bound is the
/// derivation's: a directory of suites becomes a home by its first manifest entry.
#[test]
fn every_group_root_is_declared_as_a_test_target() {
    let declared: BTreeSet<PathBuf> = test_homes::declared_test_targets()
        .into_iter()
        .map(|target| target.root)
        .collect();
    let on_disk: BTreeSet<PathBuf> = test_homes::suite_homes()
        .iter()
        .flat_map(|home| root_walk::files_in(&home.join("groups"), root_walk::ext("rs")))
        .collect();

    let undeclared: Vec<String> = on_disk.difference(&declared).map(|p| shown(p)).collect();
    assert!(
        undeclared.is_empty(),
        "these group roots exist on disk but no `[[test]]` target in crates/cli/Cargo.toml \
         names them, so cargo compiles neither the root nor any suite it registers — add a \
         `[[test]]` entry for each, or delete the file: {undeclared:?}"
    );

    let missing: Vec<String> = declared.difference(&on_disk).map(|p| shown(p)).collect();
    assert!(
        missing.is_empty(),
        "these `[[test]]` targets in crates/cli/Cargo.toml name a group root that does not \
         exist: {missing:?}"
    );
}

/// Directories under a suite home that a sibling `<name>.rs` aggregates — today only the
/// crate's `pinned_facts/`. Derived rather than listed, so a second such directory joins
/// the fence by existing, in either home. `support/` (a `mod.rs` directory), `groups/` and
/// the data directories have no `.rs` sibling and are not submodule aggregations.
fn aggregated_dirs(home: &Path) -> Vec<String> {
    let mut v: Vec<String> = fs::read_dir(home)
        .expect("a suite home is readable")
        .filter_map(|e| {
            let p = e.expect("dir entry").path();
            if !p.is_dir() {
                return None;
            }
            let name = p.file_name()?.to_str()?.to_string();
            home.join(format!("{name}.rs")).is_file().then_some(name)
        })
        .collect();
    v.sort();
    v
}

/// The same bijection one level deeper. `tests/pinned_facts.rs` declares its repro
/// blocks by hand with `#[path = "pinned_facts/<name>.rs"]`, and `suite_files()` scans
/// only `<home>/*.rs` — so until now a repro block dropped into that directory compiled
/// nowhere and reddened nothing, which is precisely the class of silence the pinned
/// facts exist to prevent.
#[test]
fn every_aggregated_submodule_is_declared_by_its_aggregator() {
    let homes = test_homes::suite_homes();
    assert!(
        homes.iter().any(|home| !aggregated_dirs(home).is_empty()),
        "no suite home has a `<dir>/` with a sibling `<dir>.rs` aggregator any more — if that \
         idiom is genuinely gone, delete this fence; if it was renamed, teach \
         `aggregated_dirs()` the new shape rather than leaving a fence that checks nothing"
    );

    for home in &homes {
        for dir in aggregated_dirs(home) {
            let at = shown(&home.join(&dir));
            let on_disk: BTreeSet<String> =
                root_walk::files_in(&home.join(&dir), root_walk::ext("rs"))
                    .iter()
                    .filter_map(|p| Some(p.file_stem()?.to_str()?.to_string()))
                    .collect();

            let body =
                fs::read_to_string(home.join(format!("{dir}.rs"))).expect("aggregator is readable");
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
                "these files under {at}/ are declared by no `#[path = \"{dir}/<name>.rs\"] \
                 mod <name>;` line in {at}.rs, so they compile nowhere and never run: \
                 {undeclared:?}"
            );

            let dangling: Vec<&String> = declared.difference(&on_disk).collect();
            assert!(
                dangling.is_empty(),
                "{at}.rs declares these submodules, but no such file exists under {at}/: \
                 {dangling:?}"
            );
        }
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
    let needle = format!(
        "{} group targets",
        test_homes::declared_test_targets().len()
    );
    let crate_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    for rel in ["Cargo.toml", "tests/test_target_registration.rs"] {
        let body = fs::read_to_string(crate_dir.join(rel)).expect("stating file is readable");
        assert!(
            body.contains(&needle),
            "crates/cli/{rel} no longer states the current test-target count: it must spell \
             out `{needle}` somewhere in its prose, so the number a reader trusts is the \
             number cargo compiles"
        );
    }
}
