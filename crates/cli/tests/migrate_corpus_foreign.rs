//! Acceptance — **`jigc migrate-corpus` stops claiming a file that is not its subject**
//! (M46 Increment 3, T2; `design/corpus-migration.md` → The corpus walk /
//! `design/validation.md` → The managed-vs-foreign discriminator).
//!
//! The verb upgrades the **managed** corpus. A never-adopted **foreign** file squatting at a
//! managed doctype's home — a brownfield repo's own Keep-a-Changelog `CHANGELOG.md`, its
//! hand-written `VISION.md`, an MADR-shaped `docs/decisions/*.md` — is the *adoption* case,
//! and M42 shipped the one discriminator that says so: `engine::validate::is_unadopted_foreign`.
//! The store door asks it. This door did not, so the same file drew
//! `schema-conformance.unadopted-instance` (advisory, routed at `jigc ingest`) from `jigc
//! validate` and `migrate-corpus.prose-needed` (blocking, routed at *"author the prose, then
//! re-run"* — a route that changes nothing) from `jigc migrate-corpus`, at exit 1. Two
//! surfaces, one file, two stories.
//!
//! The fix asks the shipped discriminator **before the fold**: the foreign file never enters
//! the fold, lands in **neither** `blocked` nor `migrated` nor `already_current`, and stops
//! holding the run's exit — and it is **reported, never silently skipped**, as the store
//! door's own advisory, **verbatim**, from the one producer (`AdoptionInputs::unadopted`).
//! Verbatim is the whole shape: a second constructor in the CLI is exactly how the two
//! surfaces would disagree again.
//!
//! # The subject set is DERIVED, and this is a derivation — not a code-side registry
//!
//! There is no table in the codebase enumerating *"the surfaces that classify a committed
//! file"*, and this suite does not pretend otherwise (`storage.md`'s placement census
//! enumerates `location`/`placement` **consumers**, which is a different set). The honest
//! subject set is derived: **`is_unadopted_foreign`'s call sites × the two home kinds**. Its
//! **five** direct call sites at the time of writing are `doc.rs` ×2 (the read-side reroute
//! and `doc list`'s registration state), `file_state.rs` (the task-scope reconciler) and
//! `validate.rs` ×2 (the store sweep's fifth family, and the `AdoptionInputs::unadopted`
//! seam). `migrate_corpus.rs` is **not** a sixth: it reaches the discriminator **through**
//! that seam — `AdoptionInputs::new` and `::unadopted` — which is the whole point of the
//! fix, since a second constructor in the CLI is exactly how the two surfaces would disagree
//! again. The subject set this suite drives is therefore *the five call sites plus the
//! seam's consumer*. The home kinds are the two a managed doc can have: a **placement** file
//! (`CHANGELOG.md`, `VISION.md` — `location: None`, a literal path) and a **located** home
//! (`docs/decisions/` — resolved through the `docs-root` knob). This suite iterates the
//! seam's consumer over both home kinds; the direct call sites carry their own suites.
//!
//! # The axis, and why it takes two fixture worlds
//!
//! `{placement, located} × {first-blocked, deferred}`. Only **one** doc can be first-blocked
//! per run — the fold halts at the first blocker and every later doc is `deferred` — and the
//! candidate list is **path-sorted**, so a root placement file (`CHANGELOG.md`) always takes
//! that slot ahead of anything under `docs/`. A located foreign file can therefore only reach
//! `first-blocked` in a world with **no** placement squatter in it. Hence two worlds:
//!
//! - **world A** — a foreign `CHANGELOG.md` (placement) + a foreign `VISION.md` (placement) +
//!   a foreign `docs/decisions/0002-use-postgres.md` (located), which reaches
//!   `placement × first-blocked`, `placement × deferred` **and** `located × deferred` in one
//!   run. It also carries the **control**: a genuinely managed, **below-version** (v1-stamped)
//!   ADR, which the exclusion must not swallow — and which, at HEAD, sat `deferred` behind a
//!   foreign first-blocker;
//! - **world B** — the located foreign ADR alone, which reaches `located × first-blocked`.
//!
//! Both worlds are built **local to this suite** on purpose: a `support::trial_corpus::State`
//! member is iterated by the compose goldens, so adding one there would force a golden
//! regeneration this task does not own.

use crate::support::run_then_parse::stdout_json;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

/// A throwaway directory that removes itself on drop.
struct TempDir(PathBuf);

impl TempDir {
    fn new(tag: &str) -> Self {
        let mut path = std::env::temp_dir();
        path.push(format!(
            "jigc-migrate-corpus-foreign-{tag}-{}-{:?}",
            std::process::id(),
            engine::tempname::unique_nanos(),
        ));
        fs::create_dir_all(&path).expect("create temp dir");
        TempDir(path)
    }

    fn path(&self) -> &Path {
        &self.0
    }
}

impl Drop for TempDir {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}

/// Run a `git` command in `cwd`, asserting success.
fn git(cwd: &Path, args: &[&str]) {
    let out = Command::new("git")
        .args(args)
        .current_dir(cwd)
        .output()
        .expect("run git");
    assert!(
        out.status.success(),
        "git {args:?} failed: {}",
        String::from_utf8_lossy(&out.stderr)
    );
}

/// A **real** Keep-a-Changelog file — the stock brownfield `CHANGELOG.md`. No schema-version
/// stamp, and it parses against no shipped `changelog` version: **foreign**, at the
/// `changelog` doctype's **placement** home.
const KEEP_A_CHANGELOG: &str = "\
# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/).

## [Unreleased]

### Added

- A new thing.

## [0.1.0] - 2026-01-01

### Added

- The first thing.
";

/// A hand-written product vision — the second **placement** squatter (`VISION.md`), which the
/// methodology pack's `vision` doctype homes at. Foreign: no stamp, no shipped shape it parses
/// against.
const HAND_WRITTEN_VISION: &str = "\
# Product vision

We want a dashboard a team can ask a question of and get an answer from in under
ten seconds.

## Why now

Every competitor ships a report builder and calls it analytics.

## What we will not do

We will not build a query language.
";

/// An MADR-shaped decision record — the **located** squatter, at the `adr` doctype's
/// `docs/decisions/` home. Foreign: no stamp, and its sections match neither the current (v2)
/// `adr` shape nor the shipped v1 snapshot.
const MADR_DECISION: &str = "\
# 2. Use Postgres

Date: 2026-01-04

## Status

Accepted

## Context and problem

We need a relational store, and we need it before the pilot.

## Chosen option

Postgres, because the team already runs it.
";

/// The **control** — a genuinely managed, **below-version** ADR: the shipped **prior (v1)**
/// shape, stamped `schema-version: 1` while the `adr` manifest is at 2. It is the corpus
/// migration's actual subject, and the exclusion must not swallow it. At HEAD it sat
/// `deferred` behind a foreign first-blocker, so the fix is what lets it migrate at all.
const ADR_STALE_V1: &str = "\
---
status: accepted
date: 2026-06-25
schema-version: 1
---

# Cache sessions in memory

## Context

Session lookups must stay sub-millisecond.

## Decision

Keep sessions in a single in-memory node.

## Consequences

A cold node loses its sessions.
";

/// The control doc's committed path.
const CONTROL_DOC: &str = "docs/decisions/cache-sessions-in-memory.md";

/// The two fixture worlds — see the module header for why one cannot reach all four cells.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum World {
    /// Two placement squatters + a located one + the managed control.
    A,
    /// The located foreign ADR alone.
    B,
}

/// The home kind a managed doctype declares — the second factor of the axis.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Home {
    /// `placement: { file: … }` — a literal path at the repo root, `location: None`,
    /// `docs-root` never applied (`design/storage.md` → Placement).
    Placement,
    /// `location: <dir>/` — resolved through the cascade's `docs-root` knob.
    Located,
}

/// Where the doc stood in the pre-fix report — the first factor of the axis. The fold halts
/// at its first blocker, so exactly one doc per run can be `FirstBlocked`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Slot {
    /// It took the halt: `migrate-corpus.prose-needed`.
    FirstBlocked,
    /// It sat behind the halt, untouched: `migrate-corpus.deferred`.
    Deferred,
}

/// One axis cell: a foreign file, its home kind, and the slot it occupied in the pre-fix
/// blocking set.
struct Cell {
    world: World,
    path: &'static str,
    doctype: &'static str,
    home: Home,
    slot: Slot,
}

/// `{placement, located} × {first-blocked, deferred}`, spread over the two worlds the module
/// header explains. Every cell is a file `migrate-corpus` blocked at HEAD and must not name at
/// all now.
const AXIS: &[Cell] = &[
    Cell {
        world: World::A,
        path: "CHANGELOG.md",
        doctype: "changelog",
        home: Home::Placement,
        slot: Slot::FirstBlocked,
    },
    Cell {
        world: World::A,
        path: "VISION.md",
        doctype: "vision",
        home: Home::Placement,
        slot: Slot::Deferred,
    },
    Cell {
        world: World::A,
        path: "docs/decisions/0002-use-postgres.md",
        doctype: "adr",
        home: Home::Located,
        slot: Slot::Deferred,
    },
    Cell {
        world: World::B,
        path: "docs/decisions/0002-use-postgres.md",
        doctype: "adr",
        home: Home::Located,
        slot: Slot::FirstBlocked,
    },
];

/// A built fixture world: the repo, its isolated `$HOME`, and the temp root that owns both.
struct Fixture {
    _root: TempDir,
    repo: PathBuf,
    home: PathBuf,
}

impl Fixture {
    /// Build `world`: a real git repo, `jigc setup`, then the world's committed squatters.
    fn build(world: World) -> Self {
        let root = TempDir::new(match world {
            World::A => "world-a",
            World::B => "world-b",
        });
        let repo = root.path().join("repo");
        let home = root.path().join("home");
        fs::create_dir_all(&repo).expect("mk repo");
        fs::create_dir_all(&home).expect("mk home");

        git(&repo, &["init", "-q"]);
        git(&repo, &["config", "user.email", "test@example.com"]);
        git(&repo, &["config", "user.name", "Test"]);
        fs::write(repo.join("README.md"), "hello\n").expect("write README");
        git(&repo, &["add", "."]);
        git(&repo, &["commit", "-q", "-m", "initial"]);

        let fixture = Fixture {
            _root: root,
            repo,
            home,
        };
        let out = fixture.jigc(&["setup"]);
        assert!(
            out.status.success(),
            "`jigc setup` must succeed; stderr:\n{}",
            String::from_utf8_lossy(&out.stderr),
        );

        let decisions = fixture.repo.join("docs").join("decisions");
        fs::create_dir_all(&decisions).expect("mk docs/decisions/");
        fs::write(decisions.join("0002-use-postgres.md"), MADR_DECISION).expect("write madr");
        if world == World::A {
            fs::write(fixture.repo.join("CHANGELOG.md"), KEEP_A_CHANGELOG)
                .expect("write changelog");
            fs::write(fixture.repo.join("VISION.md"), HAND_WRITTEN_VISION).expect("write vision");
            fs::write(fixture.repo.join(CONTROL_DOC), ADR_STALE_V1).expect("write the control adr");
        }
        git(&fixture.repo, &["add", "-A"]);
        git(&fixture.repo, &["commit", "-q", "-m", "seed the corpus"]);
        fixture
    }

    /// Run `jigc <args>` in the fixture, with the real `doc-code` probe and a scrubbed
    /// `JIGC_PACK_DIR` (a suite that inherits one reads a pack it never claimed to sweep).
    fn jigc(&self, args: &[&str]) -> std::process::Output {
        Command::new(env!("CARGO_BIN_EXE_jigc"))
            .args(args)
            .current_dir(&self.repo)
            .env("HOME", &self.home)
            .env_remove("JIGC_DOC_CODE_PROBE")
            .env_remove("JIGC_PACK_DIR")
            .output()
            .expect("run the jigc binary")
    }

    /// `jigc migrate-corpus [--dry-run] --format json` — the whole envelope plus the exit code.
    fn migrate_corpus_json(&self, dry_run: bool) -> (i32, serde_json::Value) {
        let mut args = vec!["migrate-corpus"];
        if dry_run {
            args.push("--dry-run");
        }
        args.extend(["--format", "json"]);
        let out = self.jigc(&args);
        let json = stdout_json(&out, &[0, 1], "`jigc migrate-corpus --format json`");
        (out.status.code().expect("an exit code"), json)
    }

    /// The default (agent-format) `jigc migrate-corpus` stdout plus its exit code.
    fn migrate_corpus_text(&self) -> (i32, String) {
        let out = self.jigc(&["migrate-corpus"]);
        (
            out.status.code().expect("an exit code"),
            String::from_utf8(out.stdout).expect("utf-8 stdout"),
        )
    }

    /// The `findings[]` of `jigc validate --format json` — the store door's own report.
    fn validate_findings(&self) -> Vec<serde_json::Value> {
        let out = self.jigc(&["validate", "--format", "json"]);
        let json: serde_json::Value = stdout_json(&out, &[0, 1], "`jigc validate --format json`");
        json["findings"]
            .as_array()
            .expect("the store envelope carries a `findings` array")
            .clone()
    }
}

/// The array under `key`, as a `Vec` of values.
fn array<'a>(report: &'a serde_json::Value, key: &str) -> &'a Vec<serde_json::Value> {
    report[key].as_array().unwrap_or_else(|| {
        panic!("the corpus-migration envelope carries a `{key}` array; got:\n{report:#}")
    })
}

/// The string members of the array under `key`.
fn paths(report: &serde_json::Value, key: &str) -> Vec<String> {
    array(report, key)
        .iter()
        .map(|v| v.as_str().expect("a path string").to_string())
        .collect()
}

/// The single finding in `findings` addressed at `path` and coded `code`, or a panic naming
/// what was there instead.
fn finding_at<'a>(
    findings: &'a [serde_json::Value],
    code: &str,
    path: &str,
) -> &'a serde_json::Value {
    let matches: Vec<&serde_json::Value> = findings
        .iter()
        .filter(|f| {
            f["code"].as_str() == Some(code) && f["location"]["address"].as_str() == Some(path)
        })
        .collect();
    assert_eq!(
        matches.len(),
        1,
        "expected exactly one `{code}` addressed at `{path}`; the set was:\n{findings:#?}",
    );
    matches[0]
}

/// **The axis, iterated.** Every cell of `{placement, located} × {first-blocked, deferred}`:
/// the foreign file is named by **no** `migrate-corpus.*` finding, appears in **neither**
/// `migrated` nor `already_current`, does not hold the run's exit — and is reported as the
/// store door's advisory, **verbatim**.
#[test]
fn every_axis_cell_reports_the_foreign_file_as_the_store_doors_advisory_and_blocks_nothing() {
    let world_a = Fixture::build(World::A);
    let world_b = Fixture::build(World::B);

    // The dry-run triage first, while both worlds are untouched: the discriminator is pure, so
    // `--dry-run` must print the identical adoption set an applying run prints.
    let dry: Vec<(World, serde_json::Value)> = vec![
        (World::A, world_a.migrate_corpus_json(true).1),
        (World::B, world_b.migrate_corpus_json(true).1),
    ];

    let mut reports = Vec::new();
    for (world, fixture) in [(World::A, &world_a), (World::B, &world_b)] {
        let (code, report) = fixture.migrate_corpus_json(false);
        assert_eq!(
            code, 0,
            "world {world:?}: a corpus whose only unmigratable files are foreign has nothing \
             blocked, so the run exits 0; got:\n{report:#}",
        );
        assert!(
            array(&report, "blocked").is_empty(),
            "world {world:?}: not one foreign file may hold the blocking set; got:\n{report:#}",
        );
        reports.push((world, report, fixture.validate_findings()));
    }

    for cell in AXIS {
        let (_, report, store) = reports
            .iter()
            .find(|(world, _, _)| *world == cell.world)
            .expect("both worlds ran");
        let path = cell.path;

        // (1) NOT ITS SUBJECT. No `migrate-corpus.*` finding names the file — neither the
        // halt's `prose-needed` (whose route, followed exactly, changes nothing for a file
        // jigc never wrote) nor `deferred`.
        for finding in array(report, "blocked") {
            assert_ne!(
                finding["location"]["address"].as_str(),
                Some(path),
                "{:?} × {:?} ({path}): a never-adopted foreign file is not the corpus \
                 migration's subject and may not appear in its blocking set; got:\n{finding:#}",
                cell.home,
                cell.slot,
            );
        }

        // (2) NOT SILENTLY SWALLOWED EITHER. It is in neither outcome list — reporting it
        // `already current` is the silent-skip this file's own rule forbids, and `migrated`
        // would claim a write that never happened.
        for key in ["migrated", "already_current"] {
            assert!(
                !paths(report, key).contains(&path.to_string()),
                "{:?} × {:?} ({path}): the excluded file lands in neither outcome list, and \
                 `{key}` claims it; got:\n{report:#}",
                cell.home,
                cell.slot,
            );
        }

        // (3) REPORTED, VERBATIM. The advisory the run prints is the store door's own — one
        // producer, so the two surfaces cannot tell two stories about one file again.
        let unadopted = array(report, "unadopted");
        let ours = finding_at(unadopted, "schema-conformance.unadopted-instance", path);
        let theirs = finding_at(store, "schema-conformance.unadopted-instance", path);
        for field in ["code", "severity", "message", "route"] {
            assert_eq!(
                ours[field], theirs[field],
                "{:?} × {:?} ({path}): `{field}` must be byte-identical to `jigc validate`'s \
                 for the same file — a second constructor in the CLI is exactly how the two \
                 doors disagreed;\nmigrate-corpus: {:#}\nvalidate:       {:#}",
                cell.home, cell.slot, ours[field], theirs[field],
            );
        }
        assert_eq!(
            ours["location"]["address"], theirs["location"]["address"],
            "{:?} × {:?} ({path}): the advisory is addressed at the FILE PATH on both doors — a \
             foreign file has no managed identity to claim",
            cell.home, cell.slot,
        );
        assert_eq!(
            ours, theirs,
            "{:?} × {:?} ({path}): the whole finding comes from the one producer, so it is \
             equal field for field",
            cell.home, cell.slot,
        );

        // (4) The route is the adoption route, and it names the doctype-directed verb for a
        // doctype that ships one — never `jigc migrate-corpus`, which would do nothing for it.
        let route = ours["route"].as_str().unwrap_or_else(|| {
            panic!("{path}: the advisory carries a route (the universal route floor)")
        });
        assert!(
            route.contains("jigc ingest"),
            "{path}: the adoption front door is always named; got: {route}",
        );
        assert!(
            route.contains(&format!("--as {}", cell.doctype)),
            "{path}: the `{}` doctype ships `migrate-{}`, so the doctype-directed verb is \
             named too; got: {route}",
            cell.doctype,
            cell.doctype,
        );
        assert!(
            // The check names the VERB, not the substring: since the adoption route's
            // operand became absolute (M53 post-review-fix review, HIGH 2), a fixture whose
            // own temp directory is named `…migrate-corpus-foreign…` matched a bare
            // `contains("migrate-corpus")` through the host path.
            !route.contains("jigc migrate-corpus"),
            "{path}: the route may not name the verb that just declined to act on it; got: \
             {route}",
        );
    }

    // (5) `--dry-run` is the identical triage — the check is pure, so the preview and the
    // applying run agree on the adoption set, member for member.
    for (world, preview) in &dry {
        let (_, applied, _) = reports
            .iter()
            .find(|(w, _, _)| w == world)
            .expect("both worlds ran");
        assert_eq!(
            array(preview, "unadopted"),
            array(applied, "unadopted"),
            "world {world:?}: the discriminator is pure, so `--dry-run` prints the identical \
             adoption triage",
        );
    }
}

/// **The fixture's shape is what makes the four cells distinct** — asserted, not asserted-in-
/// prose. Exactly one cell per world claims `first-blocked` (the fold halts once), and it is
/// the **path-minimum** of that world's foreign set, because the candidate list is path-sorted:
/// which is precisely why a *located* file can only reach that slot in a world with no
/// placement squatter above it.
#[test]
fn the_axis_takes_two_worlds_because_one_run_has_one_first_blocked_slot() {
    for world in [World::A, World::B] {
        let cells: Vec<&Cell> = AXIS.iter().filter(|c| c.world == world).collect();
        let first: Vec<&&Cell> = cells
            .iter()
            .filter(|c| c.slot == Slot::FirstBlocked)
            .collect();
        assert_eq!(
            first.len(),
            1,
            "world {world:?}: the fold halts at exactly one doc, so exactly one cell may claim \
             the first-blocked slot",
        );
        let minimum = cells
            .iter()
            .map(|c| c.path)
            .min()
            .expect("each world plants at least one foreign file");
        assert_eq!(
            first[0].path, minimum,
            "world {world:?}: the candidate list is path-sorted, so the first-blocked slot goes \
             to the path-minimum — this is why the located cell needs a world of its own",
        );
    }
    assert!(
        AXIS.iter()
            .any(|c| c.home == Home::Placement && c.slot == Slot::FirstBlocked)
            && AXIS
                .iter()
                .any(|c| c.home == Home::Placement && c.slot == Slot::Deferred)
            && AXIS
                .iter()
                .any(|c| c.home == Home::Located && c.slot == Slot::FirstBlocked)
            && AXIS
                .iter()
                .any(|c| c.home == Home::Located && c.slot == Slot::Deferred),
        "all four cells of `{{placement, located}} × {{first-blocked, deferred}}` are covered",
    );
}

/// **The control the exclusion must not swallow.** A genuinely managed, below-version doc in
/// world A still migrates — and *now* migrates rather than sitting `deferred` behind a foreign
/// first-blocker, which is the fix's other half: the verb's real subject was being held
/// hostage by a file that was never its subject.
#[test]
fn the_managed_below_version_doc_still_migrates_past_the_foreign_squatters() {
    let world_a = Fixture::build(World::A);
    let (code, report) = world_a.migrate_corpus_json(false);

    assert_eq!(
        code, 0,
        "nothing blocks, so the run exits 0; got:\n{report:#}"
    );
    assert!(
        paths(&report, "migrated").contains(&CONTROL_DOC.to_string()),
        "the managed v1-stamped ADR is the corpus migration's actual subject and must migrate; \
         got:\n{report:#}",
    );
    let migrated = fs::read_to_string(world_a.repo.join(CONTROL_DOC)).expect("read the control");
    assert!(
        migrated.contains("schema-version: 2"),
        "the control's stamp is value-bumped to the manifest version; got:\n{migrated}",
    );
    assert_eq!(
        array(&report, "unadopted").len(),
        3,
        "the three foreign files are still reported — the exclusion is loud, never silent; \
         got:\n{report:#}",
    );
}

/// **The text surface says it too.** The advisory is not a JSON-only fact: the agent view
/// counts the excluded files in its headline and prints each one with its code and its route,
/// because a file the verb saw and never mentioned is the silent skip `migrate_corpus.rs`' own
/// *"never a silent already-current"* rule exists to prevent.
#[test]
fn the_agent_surface_names_every_excluded_file_with_its_code_and_route() {
    let world_a = Fixture::build(World::A);
    let (code, text) = world_a.migrate_corpus_text();

    assert_eq!(code, 0, "nothing blocks, so the run exits 0; got:\n{text}");
    assert!(
        text.contains("3 not adopted"),
        "the headline counts what the run declined to claim; got:\n{text}",
    );
    for path in [
        "CHANGELOG.md",
        "VISION.md",
        "docs/decisions/0002-use-postgres.md",
    ] {
        assert!(
            text.contains(&format!("unadopted  {path}")),
            "the excluded file is listed by path; got:\n{text}",
        );
    }
    assert!(
        text.contains("schema-conformance.unadopted-instance"),
        "each excluded file carries the store door's own code; got:\n{text}",
    );
    assert!(
        text.contains("jigc ingest"),
        "each excluded file carries the adoption route; got:\n{text}",
    );
    assert!(
        text.contains("migrated   docs/decisions/cache-sessions-in-memory.md"),
        "and the managed control still migrates on the same surface; got:\n{text}",
    );
}
