//! Acceptance — **`ValueRemapped` × the field's `id-from` role × the item loci** (M52
//! Increment 7 / T3; `completions/artifacts/M52/settle-record.md` → D4.4;
//! `completions/artifacts/M52/gap-findings.md` → G-19; `baseline-freeze.md` §2.3 rows
//! D3d/D3b and §4 L-4).
//!
//! A repeatable block's `id-from` leaf is **not a committed bullet**: the parser consumes it
//! as the item's heading, and the item's `{#id}` anchor is slugged from it. So an enum rename
//! made on *that* leaf is an identity change, and the fold has no splice for it — which is
//! exactly what `transform.rs` asserted in a comment (*"the enum field is never the id-from —
//! an enum id-from item is reslug-refused"*) and exactly what the binary did not do. Driven
//! (baseline §2.3): the remap loop looked for a bullet, found none on any item, wrote **zero
//! bytes**, and handed the byte-identical buffer to the conformance gate — which broke on the
//! now-foreign heading and routed the doc at `migrate-corpus.prose-needed`: *"author the new
//! required prose … then re-run"*. There is no prose to author, so the re-run reproduced the
//! refusal byte for byte, at **both** reachable loci.
//!
//! **The axis this suite iterates is the field's ROLE crossed with the locus**, because that
//! is the discriminator the baseline measured: the two *plain*-field cells at the same two
//! loci take the correct arm (the map gap, `migrate_corpus_map_gap.rs`), so the locus is not
//! the axis and the role is. One committed document is the subject of all four cells, and each
//! arm renames the members of exactly one enum leaf at exactly one of the two change-group
//! sites the shipped `changelog` declares — the staging block (`unreleased-changes`, locus 2)
//! and the nested one (`releases/changes`, locus 3).
//!
//! The byte-correct *fold* of a plain-field remap at both loci is the engine's claim, driven
//! with a covering map in `crates/engine/src/transform.rs` →
//! `the_value_remap_axis_discriminates_the_field_role_at_every_item_locus`; a CLI arm cannot
//! make it, because `authored_remap` is a fixed table in jigc's own source and holds no entry
//! for a manufactured pack. What this suite adds on the plain cells is that they still reach
//! **their own** refusal — the map gap, naming the table entry to declare — so the role, not
//! the kind and not the locus, is what selects the cause.

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
            "jigc-id-from-remap-{tag}-{}-{:?}",
            std::process::id(),
            engine::tempname::unique_nanos(),
            // A `/` in the tag would nest this under a parent the drop never removes.
            tag = tag.replace('/', "-"),
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

/// The shipped change-group's `id-from` leaf members, and the same set with the one member
/// the committed document carries at **both** loci renamed away. Slug-form throughout: an
/// `id-from` heading's committed value is `slug(title)`, so a `CamelCase` member would be a
/// conformance break in the fixture rather than a member the corpus carries.
const CATEGORY_MEMBERS: &str = "added, changed, deprecated, removed, fixed, security";
const CATEGORY_RENAMED: &str = "added, revised, deprecated, removed, fixed, security";

/// The **plain** enum leaf's members — a committed `- status: draft` bullet, never a heading.
const STATUS_MEMBERS: &str = "draft, final";
const STATUS_RENAMED: &str = "Draft, Final";

const NOTES: &str = "{ id: notes, slot: { hint: \"One bullet per change in this category.\" } }";

/// A change-group block declared explicitly at `include`'s own indentation — read off the
/// anchor, never restated, so a re-indent of the shipped schema cannot silently point a
/// fixture at the other locus.
fn block(include: &str, category: &str, status: &str) -> String {
    let indent = &include[..include.len() - include.trim_start().len()];
    [
        format!("{{ id: category, type: enum, of: [{category}] }}"),
        format!("{{ id: status, type: enum, of: [{status}] }}"),
        NOTES.to_string(),
    ]
    .iter()
    .map(|leaf| format!("{indent}- {leaf}\n"))
    .collect()
}

/// The shipped `changelog.yaml` with **both** change-group includes replaced by explicit
/// blocks carrying the given member sets, `(category, status)` per site.
fn declared(shipped: &str, staging: (&str, &str), nested: (&str, &str)) -> String {
    let out = shipped
        .replacen(
            STAGING_INCLUDE,
            &block(STAGING_INCLUDE, staging.0, staging.1),
            1,
        )
        .replacen(
            NESTED_INCLUDE,
            &block(NESTED_INCLUDE, nested.0, nested.1),
            1,
        );
    assert!(
        !out.contains("include: change-group"),
        "both change-group sites must be declared explicitly; got:\n{out}",
    );
    out
}

/// The field role an arm renames the members of — the axis.
#[derive(Clone, Copy)]
enum Role {
    /// `status`: a committed `- status: draft` bullet. Nothing about the item's identity.
    Plain,
    /// `category`: the block's `id-from`, consumed as the item's `### changed  {#changed}`
    /// heading and slugged into its anchor.
    IdFrom,
}

impl Role {
    fn tag(self) -> &'static str {
        match self {
            Role::Plain => "plain",
            Role::IdFrom => "id-from",
        }
    }

    /// The `(category, status)` member sets this role's rename produces at the moved site.
    fn renamed(self) -> (&'static str, &'static str) {
        match self {
            Role::Plain => (CATEGORY_MEMBERS, STATUS_RENAMED),
            Role::IdFrom => (CATEGORY_RENAMED, STATUS_MEMBERS),
        }
    }
}

/// A dev-pack copy bumping `changelog` **2 → 3** by renaming `role`'s enum members at
/// **exactly one** of the two change-group sites: `schema-snapshots/changelog.v2.yaml` carries
/// both blocks unmoved, `schemas/changelog.yaml` renames the one at `site`.
fn pack_renaming(tag: &str, role: Role, site: &str) -> TempDir {
    let dir = TempDir::new(tag);
    frozen_pack::copy_dev_pack(dir.path());

    let schema_path = dir.path().join("schemas").join("changelog.yaml");
    let shipped = fs::read_to_string(&schema_path).expect("read the copied changelog.yaml");
    let base = (CATEGORY_MEMBERS, STATUS_MEMBERS);
    let prior = declared(&shipped, base, base);
    let current = match site {
        STAGING_INCLUDE => declared(&shipped, role.renamed(), base),
        NESTED_INCLUDE => declared(&shipped, base, role.renamed()),
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

/// A conformant, **v2-stamped** changelog whose change-group at **both** loci is `changed`
/// (the `id-from` member the id-from arms rename away) and carries `status: draft` (the plain
/// member the plain arms rename away) — so one document is genuinely the subject of all four
/// cells, and no cell passes because its own leaf happened to be untouched.
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

#### changed  {#changed}

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
// The axis: {plain, id-from} × {locus 2, locus 3}.
// ---------------------------------------------------------------------------------------------

/// **A `ValueRemapped` naming the block's `id-from` refuses for its own reason — at every item
/// locus — and the identity it names is not touched.** (M52 Increment 7 / T3.)
///
/// Four cells over one committed document. Both *plain* cells reach the map-gap refusal (the
/// shipped, correct arm: the repair is a table entry). Both *id-from* cells must reach
/// `migrate-corpus.fold-refused` with a cause that names the `id-from` role — never
/// `prose-needed`, whose route asks for prose that does not exist and whose re-run reproduces
/// the refusal byte for byte, and never the map-gap cause either, which asserts a missing table
/// entry that would not help if it were declared.
///
/// Red at HEAD on the two id-from cells: `migrate-corpus.prose-needed`, message relaying the
/// gate's `id-from field \`category\` in item …: \`changed\` is not an enum member`, route
/// *"author the new required prose … then re-run"*.
#[test]
fn a_value_remap_on_an_id_from_enum_refuses_at_every_item_locus() {
    for (site, locus) in [
        (STAGING_INCLUDE, "unreleased-changes"),
        (NESTED_INCLUDE, "releases/changes"),
    ] {
        for role in [Role::Plain, Role::IdFrom] {
            let cell = format!("{}@{locus}", role.tag());
            let home = TempDir::new("home");
            let pack = pack_renaming(&format!("{}-{locus}", role.tag()), role, site);
            let repo = repo_with(COMMITTED);

            let (report, ok) = migrate_report(repo.path(), home.path(), pack.path());
            let finding = sole_blocked(&report, ok);

            // (a) EVERY cell takes the non-gate code: the fold refused, so no prose an author
            //     writes into the doc clears it.
            assert_eq!(
                finding["code"], "migrate-corpus.fold-refused",
                "{cell}: a halt the gate never saw takes the non-gate code; finding:\n{finding:#}",
            );
            assert!(
                !report.to_string().contains("prose-needed"),
                "{cell}: no arm routes at prose that does not exist; report:\n{report:#}",
            );

            let message = finding["message"].as_str().expect("a message");
            let route = finding["route"].as_str().expect("a route");
            assert!(
                message.contains(&format!("`{locus}`")),
                "{cell}: the cause names the block the rename was made in; message: {message}",
            );

            // (b) The ROLE selects the cause — the whole axis.
            match role {
                Role::Plain => {
                    assert!(
                        message.contains("the authored old→new map does not cover"),
                        "{cell}: a committed bullet's rename is the map gap; message: {message}",
                    );
                    assert!(
                        route.contains("authored_remap"),
                        "{cell}: the route names the table entry to declare; route: {route}",
                    );
                }
                Role::IdFrom => {
                    assert!(
                        message.contains("`id-from`"),
                        "{cell}: the cause names the role that makes the remap inapplicable; \
                         message: {message}",
                    );
                    assert!(
                        !message.contains("the authored old→new map does not cover"),
                        "{cell}: the map gap is a false diagnosis here — declaring the entry \
                         would change nothing; message: {message}",
                    );
                    assert!(
                        !route.contains("authored_remap"),
                        "{cell}: the route must not send the reader at a table entry that \
                         cannot help; route: {route}",
                    );
                }
            }

            // (c) The document is untouched, identity included: the run refused before the
            //     fold, so every byte — both `{#changed}` anchors among them — is as committed.
            let on_disk = fs::read_to_string(repo.path().join("CHANGELOG.md"))
                .expect("read the changelog back");
            assert_eq!(
                on_disk, COMMITTED,
                "{cell}: a refused doc is rolled back byte-identical",
            );
            assert_eq!(
                on_disk.matches("{#changed}").count(),
                2,
                "{cell}: both item anchors are the ones the corpus committed",
            );
            assert_eq!(
                report["migrated"].as_array().map(Vec::len),
                Some(0),
                "{cell}: nothing migrated; report:\n{report:#}",
            );
        }
    }
}
