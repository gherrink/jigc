//! Acceptance — the **map-gap refusal at every item locus** (M50 Increment 7 / T5;
//! `design/corpus-migration.md` → the value-remap kind, the structural-auto /
//! value-semantic-authored distinction).
//!
//! An enum rename is unrecoverable from the schema pair alone, so the old→new map is a
//! CLI-**authored** migration input ([`migrate_corpus::authored_remap`]) and a committed value
//! the map does not cover blocks the doc. `migrate_corpus_halt_causes.rs` drives that refusal
//! at locus 1 (an `adr`'s header `status`) and `migrate_corpus_value_remap.rs` drives the
//! *covered* path at locus 2. What neither could reach is the **item loci as an axis**, and
//! until T5 the third one answered a different question entirely: `halt_finding`'s map-gap
//! branch was guarded `&& !locus.is_nested()`, so the moment the nested arm existed an
//! uncovered nested value would have fallen through to the *un-built arm* text — *"build the
//! nested-item-block arm of `value-remapped` in `crates/engine/src/transform.rs`"* — which is
//! **false** (the arm is built) and **unfollowable** (the operator does not have this
//! workspace). The repair that exists is a table entry, and the refusal has to name it.
//!
//! So the axis here is **the item loci, in one loop, over one committed document**: the
//! shipped `changelog` declares a change-group at both — the staging block (`unreleased-changes`,
//! locus 2) and the nested one (`releases/changes`, locus 3) — and each arm renames the enum
//! members at exactly one of them. Both must answer with the same cause, the same code, and a
//! route naming **that block's own locus path**, which is the key
//! [`migrate_corpus::authored_remap`] is looked up by.

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

use crate::support::frozen_pack;

// ---------------------------------------------------------------------------------------------
// Fixture plumbing (the shipped `migrate_corpus_nested_item_slot.rs` mechanism).
// ---------------------------------------------------------------------------------------------

/// A throwaway directory that removes itself on drop.
struct TempDir(PathBuf);

impl TempDir {
    fn new(tag: &str) -> Self {
        let mut path = std::env::temp_dir();
        path.push(format!(
            "jigc-map-gap-{tag}-{}-{:?}",
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

/// The **locus-2** change-group site: the staging block's include.
const STAGING_INCLUDE: &str = "        - include: change-group\n";

/// The **locus-3** change-group site: the nested block's include, the same text six columns
/// out — which is the only thing that distinguishes the two.
const NESTED_INCLUDE: &str = "              - include: change-group\n";

/// The shipped change-group's two leaves, spelled out so the fragment can be replaced by an
/// explicit block at one site without moving the other.
const CATEGORY: &str =
    "{ id: category, type: enum, of: [added, changed, deprecated, removed, fixed, security] }";
const NOTES: &str = "{ id: notes, slot: { hint: \"One bullet per change in this category.\" } }";

/// The enum leaf the reshape renames the members of. Deliberately **not** the block's
/// `id-from`: `category` renders as the item's own heading rather than a committed `- key:
/// value` bullet, so renaming *it* would move the item's identity, which is a different
/// change from rewriting a committed value.
fn status(members: &str) -> String {
    format!("{{ id: status, type: enum, of: [{members}] }}")
}

/// A change-group block declared explicitly at `include`'s own indentation — read off the
/// anchor, never restated, so a re-indent of the shipped schema cannot silently point a
/// fixture at the other locus.
fn block(include: &str, members: &str) -> String {
    let indent = &include[..include.len() - include.trim_start().len()];
    [CATEGORY.to_string(), status(members), NOTES.to_string()]
        .iter()
        .map(|leaf| format!("{indent}- {leaf}\n"))
        .collect()
}

/// The shipped `changelog.yaml` with **both** change-group includes replaced by explicit
/// blocks carrying the given enum members.
fn declared(shipped: &str, staging: &str, nested: &str) -> String {
    let out = shipped
        .replacen(STAGING_INCLUDE, &block(STAGING_INCLUDE, staging), 1)
        .replacen(NESTED_INCLUDE, &block(NESTED_INCLUDE, nested), 1);
    assert!(
        !out.contains("include: change-group"),
        "both change-group sites must be declared explicitly; got:\n{out}",
    );
    out
}

/// A dev-pack copy bumping `changelog` **2 → 3** by renaming the `status` enum members at
/// **exactly one** of the two change-group sites: `schema-snapshots/changelog.v2.yaml` carries
/// both at `[draft, final]`, `schemas/changelog.yaml` renames the one at `site`.
fn pack_renaming_status_at(tag: &str, site: &str) -> TempDir {
    let dir = TempDir::new(tag);
    frozen_pack::copy_dev_pack(dir.path());

    let schema_path = dir.path().join("schemas").join("changelog.yaml");
    let shipped = fs::read_to_string(&schema_path).expect("read the copied changelog.yaml");
    let renamed = "Draft, Final";
    let prior = declared(&shipped, "draft, final", "draft, final");
    let current = match site {
        STAGING_INCLUDE => declared(&shipped, renamed, "draft, final"),
        NESTED_INCLUDE => declared(&shipped, "draft, final", renamed),
        other => panic!("no such change-group site: {other:?}"),
    };
    assert_ne!(
        prior, current,
        "the reshape must move exactly one of the two blocks",
    );
    fs::write(
        dir.path()
            .join("schema-snapshots")
            .join("changelog.v2.yaml"),
        prior,
    )
    .expect("write the changelog.v2 snapshot");
    fs::write(&schema_path, current).expect("write the reshaped changelog.yaml");

    let manifest_path = dir.path().join("config").join("schema-manifest.yaml");
    let manifest = fs::read_to_string(&manifest_path).expect("read the copied manifest");
    let bumped = manifest.replacen(
        "- type: changelog\n    schema-version: 2",
        "- type: changelog\n    schema-version: 3",
        1,
    );
    assert_ne!(
        manifest, bumped,
        "the manifest must carry `changelog` at schema-version 2",
    );
    fs::write(&manifest_path, bumped).expect("write the bumped manifest");
    frozen_pack::repin_manifest_hash(dir.path(), "changelog");
    dir
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

/// A conformant, **v2-stamped** changelog carrying a committed `status: draft` at **both**
/// change-group loci — so one document is the subject of both arms, and each arm's refusal
/// has to name the block its own schema moved rather than the other one.
const COMMITTED: &str = "\
---
schema-version: 2
---

# Changelog

## Unreleased Changes

### changed  {#changed}

- the staging group

<!-- fields -->
- status: draft

## Releases

### 1.0.0  {#1-0-0}

<!-- fields -->
- date: 2026-06-14

#### added  {#added}

- the nested group

<!-- fields -->
- status: draft
";

/// A real git repo carrying the project cascade layer, one commit, and the changelog at the
/// doctype's literal placement home.
fn repo_with(changelog: &str) -> TempDir {
    let dir = TempDir::new("repo");
    let root = dir.path();
    git(root, &["init", "-q"]);
    git(root, &["config", "user.email", "test@example.com"]);
    git(root, &["config", "user.name", "Test"]);
    fs::write(root.join("README.md"), "hello\n").expect("write file");
    fs::create_dir_all(root.join(".jigc").join("config")).expect("create project layer");
    fs::write(root.join("CHANGELOG.md"), changelog).expect("write the committed changelog");
    git(root, &["add", "."]);
    git(root, &["commit", "-q", "-m", "seed the corpus"]);
    dir
}

/// `jigc migrate-corpus --format json`, parsed — with the streams surfaced on a parse failure
/// so a pack-load refusal never reads as a test bug.
fn migrate_report(repo: &Path, home: &Path, pack: &Path) -> (serde_json::Value, bool) {
    let out = Command::new(env!("CARGO_BIN_EXE_jigc"))
        .args(["migrate-corpus", "--format", "json"])
        .current_dir(repo)
        .env("HOME", home)
        .env("JIGC_PACK_DIR", pack)
        .output()
        .expect("run the jigc binary");
    let stdout = String::from_utf8_lossy(&out.stdout);
    let report: serde_json::Value = serde_json::from_str(&stdout).unwrap_or_else(|err| {
        panic!(
            "the report is JSON ({err}); stdout:\n{stdout}\nstderr:\n{}",
            String::from_utf8_lossy(&out.stderr)
        )
    });
    (report, out.status.success())
}

/// The sole `blocked[]` member of a refused run, with the refusal's exit asserted.
fn sole_blocked(report: &serde_json::Value, ok: bool) -> serde_json::Value {
    assert!(
        !ok,
        "a corpus with an un-migratable doc holds the exit; report:\n{report:#}"
    );
    let blocked = report["blocked"]
        .as_array()
        .unwrap_or_else(|| panic!("the envelope carries `blocked[]`; got:\n{report:#}"));
    assert_eq!(
        blocked.len(),
        1,
        "exactly one doc halted the run; report:\n{report:#}",
    );
    blocked[0].clone()
}

// ---------------------------------------------------------------------------------------------
// The axis.
// ---------------------------------------------------------------------------------------------

/// **An uncovered enum rename routes at the authored map — at every item locus, naming the
/// block's own locus path.** (M50 Increment 7 / T5.)
///
/// The cause is the same at both loci and so is the repair: the old→new map is a *migration
/// input*, so nothing an author writes into the doc and no re-run clears it, and the route
/// names the table entry that has to be declared. What the locus changes is the **key**:
/// `unreleased-changes` at locus 2, `releases/changes` at locus 3 — the block the rename was
/// made in, and the key [`authored_remap`] is looked up by.
///
/// Red at HEAD on the nested arm on both halves: the fold refused before the arm ran
/// (`locus_disposition(ValueRemapped, 3)` was `Unbuilt`), and `halt_finding`'s map-gap branch
/// was guarded `&& !locus.is_nested()`, so the refusal that *did* surface told the operator to
/// build an arm in `crates/engine/src/transform.rs`.
#[test]
fn an_uncovered_enum_rename_routes_at_the_authored_map_at_every_item_locus() {
    for (tag, site, locus) in [
        ("staging", STAGING_INCLUDE, "unreleased-changes"),
        ("nested", NESTED_INCLUDE, "releases/changes"),
    ] {
        let home = TempDir::new("home");
        let pack = pack_renaming_status_at(tag, site);
        let repo = repo_with(COMMITTED);

        let (report, ok) = migrate_report(repo.path(), home.path(), pack.path());
        let finding = sole_blocked(&report, ok);
        assert_eq!(
            finding["code"], "migrate-corpus.fold-refused",
            "{tag}: a halt the gate never saw takes the non-gate code; finding:\n{finding:#}",
        );

        let message = finding["message"].as_str().expect("a message");
        assert!(
            message.contains("the authored old→new map does not cover"),
            "{tag}: the cause is the map gap, not an un-built arm; message: {message}",
        );
        assert!(
            message.contains(&format!("`{locus}`")),
            "{tag}: the cause names the block the rename was made in; message: {message}",
        );

        let route = finding["route"].as_str().expect("a route");
        assert!(
            route.contains("authored_remap"),
            "{tag}: the route names the authored map — the repair that actually exists; \
             route: {route}",
        );
        assert!(
            route.contains(&format!("`{locus}`")),
            "{tag}: the route names the key `authored_remap` is looked up by, which is the \
             locus **path** and not the bare section id; route: {route}",
        );
        // The three texts this refusal must never wear: the un-built-arm instruction (the arm
        // is built, and the operator has no workspace to build it in), the schema-authoring
        // route, and the gate's author-the-prose one.
        for lie in [
            "build the nested-item-block arm",
            "crates/engine/src/transform.rs",
            "crates/engine/src/schema_diff.rs",
            "prose",
        ] {
            assert!(
                !route.contains(lie),
                "{tag}: the route must not say {lie:?} — the repair is a table entry; \
                 route: {route}",
            );
        }

        assert_eq!(
            fs::read_to_string(repo.path().join("CHANGELOG.md")).expect("read the changelog"),
            COMMITTED,
            "{tag}: a refused doc rolls back whole — the stamp never moves",
        );
    }
}
