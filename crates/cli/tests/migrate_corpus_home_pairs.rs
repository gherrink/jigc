//! Acceptance — **the corpus walk keys on the `from` home, over every prior home of every
//! kind** (M52 Increment 7 / T1; `design/corpus-migration.md` → Relocation: the walk keys on
//! the from home / The corpus walk — the placement branches become a union).
//!
//! A doctype's home can move in four ways — `location:`→`location:`, `location:`→`placement:`,
//! `placement:`→`placement:`, `placement:`→`location:` — and `migrate-corpus` must find the
//! committed instance at whichever home the *prior* schema declared, wherever that is, and land
//! it at the current one. Through HEAD it found **one** of the four: `candidate_docs` returned
//! early on the `location:` branch loading no snapshot at all, and the placement branch's
//! `.filter_map(|prior| prior.location)` dropped every prior `placement:` home by construction
//! — while its own doc-comment claimed the union is *"every home the doctype has ever
//! declared"*. Three of four cells were invisible: `0 migrated, 0 already current, 0 blocked`
//! at exit 0, the stamp stuck below current forever, and the version-aware detector's `migrate`
//! route pointing at a verb that did nothing.
//!
//! So the axis here is **the home-pair**, crossed with the bump kind, because the bump kind is
//! *not* the discriminator: `completions/artifacts/M52/baseline-freeze.md` §2.1 drove W2 and W5
//! — the same two relocations carrying an added optional section — and they behaved identically
//! to the relocation-only W1/W4. Eight cells, one loop, one pack per cell manufactured from the
//! shipped dev pack (`support::frozen_pack`), which is exactly the act a pack author performs:
//! snapshot the shipped shape at its declared version, move the home in the current shape, bump
//! the manifest, re-pin the hash from the production loader.
//!
//! **What the pre-migration `validate` says is keyed on the stranded home, not on the bump** —
//! the baseline's *"the amplifier is the home, not the kind"*, driven again here. A doc stranded
//! inside `orphan::Territory` (the `docs/` tree) is at least **red**, with the wrong diagnosis
//! (`schema-conformance.orphaned-instance`); a doc stranded at the **repo root** — the
//! `CHANGELOG.md` shape — is named by **no** surface at all: `validate` says *"validates
//! clean"* at exit 0. That is why the increment's done-criterion sentence *"`validate` moves
//! from exit 1 (`schema-conformance.schema-version-current`) to exit 0"* is carried here as
//! **two** driven pre-states rather than one: `schema-version-current` is the *content-only*
//! control's answer (a doc at its declared home, stamped below current), and neither relocated
//! from-home cell reaches it. Both pre-states are asserted; what is uniform across all eight
//! cells is the **post**-state — landed at the current home, stamp bumped, prose byte-preserved,
//! `validate` exit 0 with neither diagnosis left standing.

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

use crate::support::frozen_pack;

// ---------------------------------------------------------------------------------------------
// Fixture plumbing (the shipped `migrate_corpus_map_gap.rs` mechanism).
// ---------------------------------------------------------------------------------------------

/// A throwaway directory that removes itself on drop.
struct TempDir(PathBuf);

impl TempDir {
    fn new(tag: &str) -> Self {
        let mut path = std::env::temp_dir();
        path.push(format!(
            "jigc-home-pairs-{tag}-{}-{:?}",
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

/// Run a `git` command in `repo`, asserting success.
fn git(repo: &Path, args: &[&str]) {
    let out = Command::new("git")
        .args(args)
        .current_dir(repo)
        .output()
        .expect("run git");
    assert!(
        out.status.success(),
        "git {args:?} failed: {}",
        String::from_utf8_lossy(&out.stderr)
    );
}

/// Run `jigc <args>` in `repo` against the manufactured `pack`.
fn jigc(repo: &Path, home: &Path, pack: &Path, args: &[&str]) -> std::process::Output {
    Command::new(env!("CARGO_BIN_EXE_jigc"))
        .args(args)
        .current_dir(repo)
        .env("HOME", home)
        .env("JIGC_PACK_DIR", pack)
        .output()
        .expect("run the jigc binary")
}

/// `jigc <args>`'s stdout + whether it exited 0, with stderr folded in on a failure so a
/// pack-load refusal never reads as a test bug.
fn run(repo: &Path, home: &Path, pack: &Path, args: &[&str]) -> (String, bool) {
    let out = jigc(repo, home, pack, args);
    let stdout = String::from_utf8_lossy(&out.stdout).into_owned();
    let stderr = String::from_utf8_lossy(&out.stderr).into_owned();
    (format!("{stdout}{stderr}"), out.status.success())
}

// ---------------------------------------------------------------------------------------------
// The manufactured pack: one doctype whose home moves at a real version bump.
// ---------------------------------------------------------------------------------------------

/// The doctype's **shipped** manifest `schema-version` — read from the copied manifest rather
/// than written down, so the fixture keeps meaning what it means after the next real bump.
fn shipped_version(root: &Path, ty: &str) -> u32 {
    let manifest = fs::read_to_string(root.join("config").join("schema-manifest.yaml"))
        .expect("read the copied schema-manifest.yaml");
    let mut in_entry = false;
    for line in manifest.lines() {
        if let Some(rest) = line.strip_prefix("  - type: ") {
            in_entry = rest.trim_end() == ty;
        }
        if in_entry && let Some(v) = line.strip_prefix("    schema-version: ") {
            return v.trim().parse().expect("a numeric schema-version");
        }
    }
    panic!("the manifest must declare a `{ty}` entry");
}

/// Rewrite doctype `ty`'s manifest `schema-version` to `to`.
fn set_manifest_version(root: &Path, ty: &str, to: u32) {
    let path = root.join("config").join("schema-manifest.yaml");
    let manifest = fs::read_to_string(&path).expect("read the copied schema-manifest.yaml");
    let mut out = String::new();
    let mut in_entry = false;
    let mut hit = false;
    for line in manifest.lines() {
        if let Some(rest) = line.strip_prefix("  - type: ") {
            in_entry = rest.trim_end() == ty;
        }
        if in_entry && line.starts_with("    schema-version: ") {
            out.push_str(&format!("    schema-version: {to}"));
            hit = true;
            in_entry = false;
        } else {
            out.push_str(line);
        }
        out.push('\n');
    }
    assert!(hit, "the manifest must declare a `{ty}` entry to bump");
    fs::write(&path, out).expect("write the bumped schema-manifest.yaml");
}

/// A dev-pack copy in which doctype `ty` **bumps one version**: `prior` shapes the snapshot
/// stored at the shipped version (`schema-snapshots/<ty>.v<shipped>.yaml`, the shape the
/// committed corpus was authored under) and `current` shapes `schemas/<ty>.yaml`. The manifest
/// version is bumped and the hash re-pinned **through the production loader**
/// ([`frozen_pack::repin_manifest_hash`]), so the pack passes its own freeze gate — the act a
/// pack author performs, never a project-layer shadow that bypasses the freeze.
///
/// Returns the pack dir and the version the corpus is stamped at.
fn bumped_pack(
    tag: &str,
    ty: &str,
    prior: impl FnOnce(&str) -> String,
    current: impl FnOnce(&str) -> String,
) -> (TempDir, u32) {
    let dir = TempDir::new(tag);
    frozen_pack::copy_dev_pack(dir.path());

    let schema_path = dir.path().join("schemas").join(format!("{ty}.yaml"));
    let shipped = fs::read_to_string(&schema_path).expect("read the copied schema");
    let from = shipped_version(dir.path(), ty);

    let prior_yaml = prior(&shipped);
    let current_yaml = current(&shipped);
    assert_ne!(
        prior_yaml, current_yaml,
        "the bump must actually move `{ty}.yaml` — a pack that changes nothing proves nothing",
    );
    fs::write(
        dir.path()
            .join("schema-snapshots")
            .join(format!("{ty}.v{from}.yaml")),
        prior_yaml,
    )
    .expect("write the prior-version snapshot");
    fs::write(&schema_path, current_yaml).expect("write the reshaped schema");

    set_manifest_version(dir.path(), ty, from + 1);
    frozen_pack::repin_manifest_hash(dir.path(), ty);
    (dir, from)
}

// ---------------------------------------------------------------------------------------------
// The schema edits: the four home moves + the content rider.
// ---------------------------------------------------------------------------------------------

/// The shipped `adr`'s declared home line.
const ADR_LOCATION: &str = "location: decisions/\n";
/// The shipped `changelog`'s declared home line.
const CHANGELOG_PLACEMENT: &str = "placement: { file: CHANGELOG.md }\n";

/// Replace `needle` with `replacement` exactly once, asserting it was there.
fn swap(body: &str, needle: &str, replacement: &str) -> String {
    let out = body.replacen(needle, replacement, 1);
    assert_ne!(body, out, "the schema must carry `{}`", needle.trim_end());
    out
}

/// Append an **optional** prose section to the schema's `sections:` list — the content half of
/// a `Relocated` + content bump (an `AddedOptionalSection`, which the transform splices as an
/// empty `## Rollout` at its schema-ordered offset and which needs no authored prose).
fn with_optional_section(body: &str) -> String {
    format!("{body}  - id: rollout\n    slot: {{ optional: true, hint: \"How it rolls out.\" }}\n")
}

// ---------------------------------------------------------------------------------------------
// The corpora.
// ---------------------------------------------------------------------------------------------

/// A conformant `adr` body, stamped at `version` — the `adr_schema_conformance_store.rs`
/// fixture, which is driven clean against the shipped schema.
fn adr_body(version: u32) -> String {
    format!(
        "\
---
status: accepted
date: 2026-06-25
schema-version: {version}
---

# Cache sessions in memory

## Context

Session lookups must stay sub-millisecond.

## Options

A distributed cache was weighed and rejected on latency.

## Decision

Keep sessions in a single in-memory node.

## Consequences

A cold node loses its sessions.
"
    )
}

/// A conformant `changelog` body, stamped at `version` — a staged group and one cut release.
fn changelog_body(version: u32) -> String {
    format!(
        "\
---
schema-version: {version}
---

# Changelog

## Unreleased Changes

### changed  {{#changed}}

- the staging group

## Releases

### 1.0.0  {{#1-0-0}}

<!-- fields -->
- date: 2026-06-14

#### added  {{#added}}

- the nested group
"
    )
}

/// A git repo with `jigc setup` run over it against `pack`, ready to take a committed corpus.
fn set_up_repo(tag: &str, home: &Path, pack: &Path) -> TempDir {
    let dir = TempDir::new(tag);
    let root = dir.path();
    git(root, &["init", "-q"]);
    git(root, &["config", "user.email", "test@example.com"]);
    git(root, &["config", "user.name", "Test"]);
    fs::write(root.join("README.md"), "hello\n").expect("write file");
    git(root, &["add", "."]);
    git(root, &["commit", "-q", "-m", "initial"]);

    let out = jigc(root, home, pack, &["setup"]);
    assert!(
        out.status.success(),
        "`jigc setup` must succeed; stdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr),
    );
    dir
}

/// Commit `body` at the repo-relative `path`.
fn commit_doc(repo: &Path, path: &str, body: &str) {
    let full = repo.join(path);
    if let Some(parent) = full.parent() {
        fs::create_dir_all(parent).expect("mk the doc's home");
    }
    fs::write(&full, body).expect("write the committed doc");
    git(repo, &["add", "-A"]);
    git(repo, &["commit", "-q", "-m", "seed the corpus"]);
}

// ---------------------------------------------------------------------------------------------
// The axis.
// ---------------------------------------------------------------------------------------------

/// What `jigc validate` says about the corpus **before** the migration — keyed on whether the
/// stranded from-home lies inside `orphan::Territory`, which is the whole discriminator
/// (`completions/artifacts/M52/baseline-freeze.md` §2.1: *"the amplifier is the home, not the
/// kind"*).
#[derive(Clone, Copy, Debug)]
enum PreState {
    /// Stranded inside the `docs/` tree: red, with the wrong diagnosis.
    Orphaned,
    /// Stranded at the repo root: **no** surface mentions the document — exit 0, "validates
    /// clean", which is what makes the walk hole silent rather than merely wrong.
    SilentlyClean,
}

/// One `{from-home kind, to-home kind}` cell.
struct Cell {
    tag: &'static str,
    ty: &'static str,
    /// The current schema's home edit (applied to the shipped body).
    move_home: fn(&str) -> String,
    /// The committed instance's from-home.
    source: &'static str,
    /// Where the migration must land it.
    destination: &'static str,
    /// The doc body, stamped at the shipped version.
    body: fn(u32) -> String,
    /// A distinctive line of the committed prose, which the migration must preserve.
    prose: &'static str,
    pre: PreState,
}

/// The four home-pair cells. The pair is the axis; the *doctypes* are the two shipped carriers
/// of the two home kinds (`adr` declares `location:`, `changelog` declares `placement:`), so
/// each is driven in both directions.
const CELLS: &[Cell] = &[
    Cell {
        tag: "location-to-location",
        ty: "adr",
        move_home: |body| swap(body, ADR_LOCATION, "location: adrs/\n"),
        source: "docs/decisions/cache-sessions-in-memory.md",
        destination: "docs/adrs/cache-sessions-in-memory.md",
        body: adr_body,
        prose: "Session lookups must stay sub-millisecond.",
        pre: PreState::Orphaned,
    },
    Cell {
        tag: "location-to-placement",
        ty: "adr",
        move_home: |body| swap(body, ADR_LOCATION, "placement: { file: DECISIONS.md }\n"),
        source: "docs/decisions/cache-sessions-in-memory.md",
        destination: "DECISIONS.md",
        body: adr_body,
        prose: "Session lookups must stay sub-millisecond.",
        pre: PreState::Orphaned,
    },
    Cell {
        tag: "placement-to-placement",
        ty: "changelog",
        move_home: |body| {
            swap(
                body,
                CHANGELOG_PLACEMENT,
                "placement: { file: HISTORY.md }\n",
            )
        },
        source: "CHANGELOG.md",
        destination: "HISTORY.md",
        body: changelog_body,
        prose: "- the staging group",
        pre: PreState::SilentlyClean,
    },
    Cell {
        tag: "placement-to-location",
        ty: "changelog",
        move_home: |body| swap(body, CHANGELOG_PLACEMENT, "location: changelog/\n"),
        source: "CHANGELOG.md",
        // A fixed-identity doctype's slug is its type id (`engine::index::instance_slug`'s
        // placement rule, and `store.fixed-identity`'s one address), so the identity survives
        // the move into a folder home as the file **stem**.
        destination: "docs/changelog/changelog.md",
        body: changelog_body,
        prose: "- the staging group",
        pre: PreState::SilentlyClean,
    },
];

/// The bump kinds the cell axis is crossed with — `Relocated` alone, and `Relocated` carrying a
/// content change — because the **bump kind is not the discriminator** (baseline §2.1: W2 and
/// W5 behave identically to W1 and W4). `None` is the relocation-only arm.
struct Rider {
    tag: &'static str,
    /// Whether the bump also adds an optional section (the content half).
    content: bool,
}

const RIDERS: &[Rider] = &[
    Rider {
        tag: "relocated-only",
        content: false,
    },
    Rider {
        tag: "with-content",
        content: true,
    },
];

/// Parse a `--format json` report, surfacing the streams on a parse failure.
fn report(repo: &Path, home: &Path, pack: &Path, args: &[&str]) -> (serde_json::Value, bool) {
    let out = jigc(repo, home, pack, args);
    let stdout = String::from_utf8_lossy(&out.stdout);
    let value: serde_json::Value = serde_json::from_str(&stdout).unwrap_or_else(|err| {
        panic!(
            "`jigc {}` must emit JSON ({err}); stdout:\n{stdout}\nstderr:\n{}",
            args.join(" "),
            String::from_utf8_lossy(&out.stderr),
        )
    });
    (value, out.status.success())
}

/// The doc's `schema-version:` stamp, read off its front matter.
fn stamp(body: &str) -> Option<u32> {
    body.lines()
        .take_while(|line| *line != "---" || body.starts_with("---\n"))
        .find_map(|line| line.strip_prefix("schema-version: "))
        .and_then(|v| v.trim().parse().ok())
}

/// **A below-version doc at any prior home of any kind is seen and landed at the current home**
/// — all four `{from, to}` home-pair cells × both bump kinds (M52 Increment 7 / T1).
///
/// RED at HEAD in three of the four cells (and, with the rider, in six of the eight):
/// `migrate-corpus` reported `0 migrated, 0 already current, 0 blocked` at exit 0 and the doc
/// stayed stranded at its old home, invisible to every surface.
#[test]
fn a_below_version_doc_at_every_prior_home_kind_is_seen_and_landed() {
    for cell in CELLS {
        for rider in RIDERS {
            let tag = format!("{}-{}", cell.tag, rider.tag);
            let move_home = cell.move_home;
            let (pack, from_version) = bumped_pack(
                &tag,
                cell.ty,
                |shipped| shipped.to_string(),
                |shipped| {
                    let moved = move_home(shipped);
                    if rider.content {
                        with_optional_section(&moved)
                    } else {
                        moved
                    }
                },
            );
            let home = TempDir::new(&format!("home-{tag}"));
            let repo = set_up_repo(&tag, home.path(), pack.path());
            let body = (cell.body)(from_version);
            commit_doc(repo.path(), cell.source, &body);

            // The pre-state, keyed on the stranded home (not on the bump).
            let (before, before_ok) = run(repo.path(), home.path(), pack.path(), &["validate"]);
            match cell.pre {
                PreState::Orphaned => {
                    assert!(
                        !before_ok && before.contains("schema-conformance.orphaned-instance"),
                        "{tag}: a doc stranded inside the docs tree is red with the wrong \
                         diagnosis; validate said:\n{before}",
                    );
                }
                PreState::SilentlyClean => {
                    assert!(
                        before_ok && before.contains("validates clean"),
                        "{tag}: a doc stranded at the repo root is named by no surface — that \
                         silence is what the walk hole costs; validate said:\n{before}",
                    );
                }
            }

            // The walk sees it, and the migration lands it at the current home.
            let (report, ok) = report(
                repo.path(),
                home.path(),
                pack.path(),
                &["migrate-corpus", "--format", "json"],
            );
            assert!(ok, "{tag}: the migration runs clean; report:\n{report:#}");
            assert_eq!(
                report["migrated"],
                serde_json::json!([cell.destination]),
                "{tag}: the below-version doc at the prior home is migrated to the current \
                 home; report:\n{report:#}",
            );
            assert!(
                report["blocked"].as_array().is_some_and(|b| b.is_empty()),
                "{tag}: nothing is blocked; report:\n{report:#}",
            );

            // The bytes: landed at the destination, stamp bumped, prose preserved, source gone.
            let landed = fs::read_to_string(repo.path().join(cell.destination))
                .unwrap_or_else(|err| panic!("{tag}: the destination must hold the doc: {err}"));
            assert_eq!(
                stamp(&landed),
                Some(from_version + 1),
                "{tag}: the stamp flips last, to the current version; landed:\n{landed}",
            );
            assert!(
                landed.contains(cell.prose),
                "{tag}: the committed prose survives the move; landed:\n{landed}",
            );
            if !rider.content {
                assert_eq!(
                    landed,
                    body.replacen(
                        &format!("schema-version: {from_version}"),
                        &format!("schema-version: {}", from_version + 1),
                        1,
                    ),
                    "{tag}: a relocation-only bump moves the bytes and the stamp, nothing else",
                );
            }
            assert!(
                !repo.path().join(cell.source).exists(),
                "{tag}: the prior home is vacated — write-before-remove completes the move",
            );

            // And the store validates clean afterwards, with neither diagnosis left standing.
            let (after, after_ok) = run(repo.path(), home.path(), pack.path(), &["validate"]);
            assert!(
                after_ok,
                "{tag}: the migrated store validates clean; validate said:\n{after}",
            );
            for stale in [
                "schema-conformance.orphaned-instance",
                "schema-conformance.schema-version-current",
            ] {
                assert!(
                    !after.contains(stale),
                    "{tag}: `{stale}` must not survive the migration; validate said:\n{after}",
                );
            }
        }
    }
}

/// **A prior `placement:` home is re-rooted through `placement-root` before it is walked.**
///
/// The knob re-roots a *declared* home carrying a leading directory component, and the snapshot
/// store holds **declarations** — so the walk has to apply the knob to the prior declaration
/// exactly as schema resolution applies it to the current one. Applying it to an
/// already-resolved home instead is the documented trap in
/// [`cli::start::reroot_placement_file`]: under a root the resolved home has no leading
/// component left, so the re-root is a silent no-op.
///
/// Baseline §2.1 W9, driven: `docs/history.md` → `docs/story.md` under `placement-root: site`
/// left the committed `site/history.md` at `0 migrated` with `validate` calling it orphaned.
#[test]
fn a_prior_placement_home_is_rerooted_through_placement_root() {
    let (pack, from_version) = bumped_pack(
        "rerooted",
        "changelog",
        |shipped| {
            swap(
                shipped,
                CHANGELOG_PLACEMENT,
                "placement: { file: docs/history.md }\n",
            )
        },
        |shipped| {
            swap(
                shipped,
                CHANGELOG_PLACEMENT,
                "placement: { file: docs/story.md }\n",
            )
        },
    );
    let home = TempDir::new("home-rerooted");
    let repo = set_up_repo("rerooted", home.path(), pack.path());

    let out = jigc(
        repo.path(),
        home.path(),
        pack.path(),
        &["config", "set", "placement-root", "site"],
    );
    assert!(
        out.status.success(),
        "`config set placement-root` must succeed; stdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr),
    );
    git(repo.path(), &["add", "-A"]);
    git(repo.path(), &["commit", "-q", "-m", "set placement-root"]);

    // The instance sits at the PRIOR declaration re-rooted — `site/history.md`, not
    // `docs/history.md`.
    commit_doc(
        repo.path(),
        "site/history.md",
        &changelog_body(from_version),
    );

    let (report, ok) = report(
        repo.path(),
        home.path(),
        pack.path(),
        &["migrate-corpus", "--format", "json"],
    );
    assert!(ok, "the migration runs clean; report:\n{report:#}");
    assert_eq!(
        report["migrated"],
        serde_json::json!(["site/story.md"]),
        "the prior placement home re-roots through the knob, and the destination does too; \
         report:\n{report:#}",
    );
    assert!(
        !repo.path().join("site/history.md").exists(),
        "the re-rooted prior home is vacated",
    );
    assert!(
        !repo.path().join("docs/story.md").exists(),
        "nothing lands at the UNrooted declaration — re-rooting the resolved home instead of \
         the declaration is the silent no-op this guards",
    );
}

/// **Both homes populated: the widened walk reports the collision, it does not clobber.**
///
/// Widening the walk widens the collision surface (the increment's declared bound), and the
/// answer is the shipped destination-collision rule: a relocating doc whose destination already
/// holds *other* bytes is blocked with a route, nothing written and nothing removed.
#[test]
fn a_destination_already_populated_blocks_instead_of_clobbering() {
    let (pack, from_version) = bumped_pack(
        "collision",
        "changelog",
        |shipped| shipped.to_string(),
        |shipped| {
            swap(
                shipped,
                CHANGELOG_PLACEMENT,
                "placement: { file: HISTORY.md }\n",
            )
        },
    );
    let home = TempDir::new("home-collision");
    let repo = set_up_repo("collision", home.path(), pack.path());

    // The stranded instance at the prior home, and a *different* document already sitting at
    // the destination, already current.
    let stranded = changelog_body(from_version);
    let squatter = changelog_body(from_version + 1).replace("- the staging group", "- the tenant");
    fs::write(repo.path().join("CHANGELOG.md"), &stranded).expect("write the stranded doc");
    fs::write(repo.path().join("HISTORY.md"), &squatter).expect("write the squatter");
    git(repo.path(), &["add", "-A"]);
    git(repo.path(), &["commit", "-q", "-m", "seed both homes"]);

    let (report, ok) = report(
        repo.path(),
        home.path(),
        pack.path(),
        &["migrate-corpus", "--format", "json"],
    );
    assert!(
        !ok,
        "a blocked doc holds the exit non-zero; report:\n{report:#}"
    );
    let blocked = report["blocked"].as_array().expect("a `blocked[]` array");
    assert_eq!(
        blocked.len(),
        1,
        "exactly one doc blocks; report:\n{report:#}"
    );
    assert_eq!(
        blocked[0]["code"], "migrate-corpus.destination-collision",
        "the widened walk's own hazard answers with the shipped collision rule; \
         finding:\n{:#}",
        blocked[0],
    );
    assert_eq!(
        fs::read_to_string(repo.path().join("HISTORY.md")).expect("read the destination"),
        squatter,
        "the occupant is byte-untouched — No-data-loss",
    );
    assert_eq!(
        fs::read_to_string(repo.path().join("CHANGELOG.md")).expect("read the source"),
        stranded,
        "and the stranded source is byte-untouched too — nothing is half-moved",
    );
}
