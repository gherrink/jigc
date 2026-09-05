//! Acceptance — **the migration's third locus** (M50 Increment 6 / T2;
//! `completions/artifacts/M50/settle-record.md` → D6 — the full cut).
//!
//! A repeatable item block may itself declare a repeatable
//! ([`engine::schema::MAX_NESTING_DEPTH`] `= 2`, derived), so the classifier has **three**
//! loci: a section's own fields and slot, its item block's leaves, and a **nested** item
//! block's leaves. Until this task it iterated two: `diff_item_fields` compared a nested
//! `Leaf::Repeatable` **wholesale** and pushed the empty-diff backstop kind on any
//! inequality, so every one of the ~10 kinds locus 2 carries answered
//! `migrate-corpus.unclassified-change` at locus 3 — a route into this workspace that no
//! adopter can follow, over the one shape a `changelog`-like doctype actually evolves in.
//!
//! Two claims, and the file is split on them:
//!
//! 1. **Every locus-3 shape change classifies.** The `kind × locus` disposition is a
//!    production table ([`engine::schema_diff::locus_disposition`]) the driver itself
//!    consults, and the locus-3 column is driven kind by kind against a manufactured
//!    nested schema pair — so *no cell reads `Unclassified` for a kind that has one*
//!    is a measurement, not a claim.
//! 2. **The fold resolves a nested leaf at its own locus.** A nested leaf whose id also
//!    exists in the outer block is the shipped collision (`date` on `changelog.releases`),
//!    and a locus-blind fold resolves it against the **outer** declaration — takes that
//!    field's no-op branch, writes nothing, and restamps the doc. Driven on exactly that
//!    pair through the shipped binary.

use std::collections::BTreeSet;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

use crate::support::frozen_pack;

use engine::schema::{MAX_NESTING_DEPTH, load_schema};
use engine::schema_diff::{
    LOCI, LocusDisposition, SchemaChangeKind, locus_disposition, schema_diff,
};

// ---------------------------------------------------------------------------------------------
// The manufactured nested schema pair — one edit per kind, all at locus 3.
// ---------------------------------------------------------------------------------------------

/// A doctype whose item block declares a **nested** repeatable — the third locus, carrying
/// one leaf per shape the classifier can be asked about there.
const PROBE_V1: &str = "\
type: probe
location: probes/
id-from: title
description: A probe doctype whose item block nests a second repeatable.
usage: the classifier is asked about a leaf at the third locus.
sections:
  - id: entries
    repeatable:
      id-from: title
      block:
        - { id: title, type: string }
        - id: notes
          repeatable:
            id-from: key
            block:
              - { id: key, type: string }
              - { id: tally, type: string, card: \"0..1\", optional: true }
              - { id: mood, type: enum, of: [glad, sad] }
              - { id: stamped, type: string, set: on-create, optional: true }
              - { id: body, slot: { hint: \"The note.\" } }
              - { id: aside, slot: { optional: true, hint: \"An aside.\" } }
";

/// One nested-block leaf line, as it appears in [`PROBE_V1`].
fn nested(line: &str) -> String {
    format!("              - {line}\n")
}

/// [`PROBE_V1`] with `needle` replaced by `replacement` — both nested-block leaf lines.
fn edited(needle: &str, replacement: &str) -> String {
    let out = PROBE_V1.replacen(&nested(needle), replacement, 1);
    assert_ne!(out, PROBE_V1, "the probe schema must declare `{needle}`");
    out
}

/// Replace a nested-block leaf line with another.
fn swap(needle: &str, replacement: &str) -> String {
    edited(needle, &nested(replacement))
}

/// Append a leaf line to the nested block (after its last declared leaf).
fn add(line: &str) -> String {
    let anchor = nested("{ id: aside, slot: { optional: true, hint: \"An aside.\" } }");
    let out = PROBE_V1.replacen(&anchor, &format!("{anchor}{}", nested(line)), 1);
    assert_ne!(out, PROBE_V1, "the probe schema must declare `aside`");
    out
}

/// Drop a leaf line from the nested block.
fn drop_leaf(needle: &str) -> String {
    edited(needle, "")
}

/// Every locus-3 edit, keyed by the kind it must classify as.
fn locus_3_edits() -> Vec<(SchemaChangeKind, String)> {
    vec![
        (
            SchemaChangeKind::WidenedCardinality,
            swap(
                "{ id: tally, type: string, card: \"0..1\", optional: true }",
                "{ id: tally, type: string, card: \"0..*\", optional: true }",
            ),
        ),
        (
            SchemaChangeKind::NarrowedCardinality,
            swap(
                "{ id: tally, type: string, card: \"0..1\", optional: true }",
                "{ id: tally, type: string, card: \"1\", optional: true }",
            ),
        ),
        (
            SchemaChangeKind::EnumWidened,
            swap(
                "{ id: mood, type: enum, of: [glad, sad] }",
                "{ id: mood, type: enum, of: [glad, sad, meh] }",
            ),
        ),
        (
            SchemaChangeKind::ValueRemapped,
            swap(
                "{ id: mood, type: enum, of: [glad, sad] }",
                "{ id: mood, type: enum, of: [glad, blue] }",
            ),
        ),
        (
            SchemaChangeKind::AddedItemField,
            add("{ id: ticket, type: string, optional: true }"),
        ),
        (
            SchemaChangeKind::AddedItemSlot,
            add("{ id: extra, slot: { optional: true, hint: \"More.\" } }"),
        ),
        (
            SchemaChangeKind::ProseNeeding,
            add("{ id: owner, type: string }"),
        ),
        (
            SchemaChangeKind::OptionalRelaxed,
            swap(
                "{ id: body, slot: { hint: \"The note.\" } }",
                "{ id: body, slot: { optional: true, hint: \"The note.\" } }",
            ),
        ),
        (
            SchemaChangeKind::PresentationOnly,
            swap(
                "{ id: stamped, type: string, set: on-create, optional: true }",
                "{ id: stamped, type: string, set: on-create }",
            ),
        ),
        (
            SchemaChangeKind::RemovedField,
            drop_leaf("{ id: mood, type: enum, of: [glad, sad] }"),
        ),
        (
            SchemaChangeKind::RemovedItemSlot,
            drop_leaf("{ id: aside, slot: { optional: true, hint: \"An aside.\" } }"),
        ),
    ]
}

/// **Every locus-3 shape change classifies — none of them reads the backstop.**
///
/// One edit per kind, each confined to the **nested** block, each diffed through the real
/// classifier. Red at HEAD on all eleven: `diff_item_fields` compared the nested leaf
/// wholesale, so every edit answered `[Unclassified]`.
#[test]
fn every_locus_3_shape_change_classifies() {
    let mut driven: BTreeSet<&str> = BTreeSet::new();
    for (expected, v2) in locus_3_edits() {
        let old = load_schema(PROBE_V1.as_bytes()).expect("the probe v1 schema loads");
        let new = load_schema(v2.as_bytes())
            .unwrap_or_else(|err| panic!("the `{expected:?}` probe v2 schema loads: {err}\n{v2}"));
        let changes = schema_diff(&old, &new);
        let kinds: BTreeSet<&str> = changes
            .iter()
            .map(|c| SchemaChangeKind::from(c).as_str())
            .collect();
        assert_eq!(
            kinds,
            BTreeSet::from([expected.as_str()]),
            "a nested-block `{expected:?}` edit must classify as itself, at the third locus; \
             got {changes:?}",
        );
        // The kind alone is not the claim — a locus-blind classification would satisfy it and
        // then splice at the wrong depth. Every change that HAS a locus must name the nested
        // one; `PresentationOnly` names a delta rather than a place and has none.
        for change in &changes {
            if let Some(locus) = change.locus() {
                assert_eq!(
                    (locus.to_string(), locus.index()),
                    ("entries/notes".to_string(), LOCI),
                    "`{expected:?}` must be classified AT the nested locus; got {change:?}",
                );
            }
        }
        driven.insert(expected.as_str());
    }
    assert_eq!(
        driven,
        reachable_at(LOCI),
        "every cell the disposition table calls reachable at the deepest locus is driven here, \
         and nothing is driven that the table calls unreachable",
    );
}

/// **A nested block's `id-from` is a non-leaf key inside the projection, and it must not
/// reach nothing.** The recursion diffs the nested block's *leaves*; the block's own
/// `id-from` — which names every nested item's anchor — is not one of them, so re-keying it
/// would be invisible to a leaf-only recursion. It rides the backstop (no transform kind
/// re-slugs a nested item), and it must ride it **explicitly**: alongside a classified
/// change the residual never fires.
#[test]
fn a_nested_id_from_flip_names_itself_beside_a_classified_change() {
    let old = load_schema(PROBE_V1.as_bytes()).expect("the probe v1 schema loads");
    let reflowed = PROBE_V1
        .replacen(
            "            id-from: key\n",
            "            id-from: title\n",
            1,
        )
        .replacen(
            &nested("{ id: key, type: string }"),
            &nested("{ id: title, type: string }"),
            1,
        );
    assert_ne!(
        reflowed, PROBE_V1,
        "the nested block declares `id-from: key`"
    );
    // Alongside a classified sibling, so the empty-diff residual is unreachable.
    let v2 = reflowed.replacen(
        &nested("{ id: mood, type: enum, of: [glad, sad] }"),
        &nested("{ id: mood, type: enum, of: [glad, sad, meh] }"),
        1,
    );
    let new = load_schema(v2.as_bytes()).expect("the re-keyed probe v2 schema loads");

    let changes = schema_diff(&old, &new);
    let kinds: BTreeSet<&str> = changes
        .iter()
        .map(|c| SchemaChangeKind::from(c).as_str())
        .collect();
    assert!(
        kinds.contains(SchemaChangeKind::Unclassified.as_str()),
        "a nested `id-from` flip must name itself, or it vanishes into a non-empty diff; \
         got {changes:?}",
    );
    assert!(
        kinds.contains(SchemaChangeKind::EnumWidened.as_str()),
        "the classified sibling still classifies; got {changes:?}",
    );
}

// ---------------------------------------------------------------------------------------------
// The completeness fence — `SchemaChangeKind::ALL x loci`, with the locus count derived.
// ---------------------------------------------------------------------------------------------

/// The loci a kind is **reachable** at, per the production disposition table.
fn reachable_at(locus: usize) -> BTreeSet<&'static str> {
    SchemaChangeKind::ALL
        .iter()
        .filter(|kind| {
            !matches!(
                locus_disposition(**kind, locus),
                LocusDisposition::Unreachable | LocusDisposition::DoctypeLevel
            )
        })
        .map(|kind| kind.as_str())
        .collect()
}

/// **Every `kind × locus` cell is dispositioned, and the locus count is derived.**
///
/// The iteration is `SchemaChangeKind::ALL × 1..=LOCI` with `LOCI` read from
/// [`MAX_NESTING_DEPTH`] — never a literal, because a literal three here would re-enact
/// M45's *statement == constant* failure the day the address grammar grows a hop pair
/// (`completions/artifacts/M50/settle-record.md` → D6: *any increment hardcoding "three
/// loci" re-enacts it*).
///
/// Four claims over the table, each of which a wrong cell breaks:
///
/// - **no kind is dead** — every kind is reachable at some locus, or is doctype-level at all
///   of them;
/// - **doctype-level is all-or-nothing** — a kind that sits at no locus says so at every
///   index, so `Relocated` can never be half a locus kind;
/// - **loci 1 and 2 are fully built** — no `Unbuilt` cell there; the un-built arms this wave
///   leaves behind are the nested ones and only those;
/// - **the locus-3 column is the locus-2 column minus the backstop** — which is precisely
///   what *"no cell reads `Unclassified` for a kind that has one"* means: every kind the item
///   locus carries, the nested item locus carries too, and only the backstop drops out (a
///   repeatable nested inside a nested repeatable is refused at schema load).
#[test]
fn every_kind_x_locus_cell_is_dispositioned() {
    assert_eq!(
        LOCI,
        MAX_NESTING_DEPTH + 1,
        "the locus count is derived from the nesting cap, never written down",
    );

    for kind in SchemaChangeKind::ALL {
        let cells: Vec<LocusDisposition> = (1..=LOCI).map(|l| locus_disposition(kind, l)).collect();
        let doctype_level = cells
            .iter()
            .filter(|c| **c == LocusDisposition::DoctypeLevel)
            .count();
        assert!(
            doctype_level == 0 || doctype_level == LOCI,
            "`{kind:?}` is doctype-level at some loci and not others; cells: {cells:?}",
        );
        assert!(
            cells.iter().any(|c| *c != LocusDisposition::Unreachable),
            "`{kind:?}` is unreachable at every locus — a kind nothing can emit; cells: {cells:?}",
        );
    }

    for locus in [1, 2] {
        let unbuilt: Vec<&str> = SchemaChangeKind::ALL
            .iter()
            .filter(|k| locus_disposition(**k, locus) == LocusDisposition::Unbuilt)
            .map(|k| k.as_str())
            .collect();
        assert!(
            unbuilt.is_empty(),
            "locus {locus} is fully built; got un-built cells {unbuilt:?}",
        );
    }

    let mut expected = reachable_at(2);
    assert!(
        expected.remove(SchemaChangeKind::Unclassified.as_str()),
        "the backstop is reachable at the item locus (a nested block added, dropped or re-keyed)",
    );
    assert_eq!(
        reachable_at(LOCI),
        expected,
        "the deepest locus must carry every kind the one above it carries, minus the backstop — \
         a kind missing here is a shape that answers `unclassified-change` one level down",
    );
}

// ---------------------------------------------------------------------------------------------
// Fixture plumbing for the binary-driven arms (the shipped `migrate_corpus_set_fields.rs`
// mechanism, over a dev-pack copy).
// ---------------------------------------------------------------------------------------------

/// A throwaway directory that removes itself on drop.
struct TempDir(PathBuf);

impl TempDir {
    fn new(tag: &str) -> Self {
        let mut path = std::env::temp_dir();
        path.push(format!(
            "jigc-locus3-{tag}-{}-{:?}",
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

/// The shipped `changelog` nested item block's one declared leaf line — the anchor a
/// locus-3 leaf is appended after. Its indentation is what distinguishes the **nested**
/// change-group include from the staging one, which is the same text six columns out.
const NESTED_INCLUDE: &str = "              - include: change-group\n";

/// A dev-pack copy bumping `changelog` **2 → 3** with `added` appended to its **nested**
/// `changes` block: `schema-snapshots/changelog.v2.yaml` is the shipped shape (the prior),
/// `schemas/changelog.yaml` carries the added leaf, and the manifest carries version 3 with
/// its `schema-hash` re-pinned through the production loader.
fn pack_adding_a_nested_leaf(tag: &str, added: &str) -> TempDir {
    let dir = TempDir::new(tag);
    frozen_pack::copy_dev_pack(dir.path());

    let schema_path = dir.path().join("schemas").join("changelog.yaml");
    let shipped = fs::read_to_string(&schema_path).expect("read the copied changelog.yaml");
    fs::write(
        dir.path()
            .join("schema-snapshots")
            .join("changelog.v2.yaml"),
        &shipped,
    )
    .expect("write the changelog.v2 snapshot");

    // The nested block's indentation is READ off the anchor rather than restated: a literal
    // run of spaces here is both a fence violation and a fact that would drift the day the
    // schema is re-indented.
    let indent = &NESTED_INCLUDE[..NESTED_INCLUDE.len() - NESTED_INCLUDE.trim_start().len()];
    let reshaped = shipped.replacen(
        NESTED_INCLUDE,
        &format!("{NESTED_INCLUDE}{indent}- {added}\n"),
        1,
    );
    assert_ne!(
        reshaped, shipped,
        "changelog.yaml must declare a nested change-group include",
    );
    // The staging block's include is the FIRST occurrence at its own indentation and must
    // stay untouched: the whole point of the arm is a leaf that exists at locus 3 only.
    assert_eq!(
        reshaped
            .lines()
            .filter(|line| *line == "        - include: change-group")
            .count(),
        1,
        "the staging include must be left alone",
    );
    fs::write(&schema_path, &reshaped).expect("write the reshaped changelog.yaml");

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

/// A conformant, **v2-stamped** `CHANGELOG.md` carrying a **nested** change-group — the
/// committed shape the locus-3 fold runs over.
const CHANGELOG_V2: &str = "\
---
schema-version: 2
---

# Changelog

## Unreleased Changes

### changed  {#changed}

- the fixture builder gained shape-class coverage

## Releases

### 1.0.0  {#1-0-0}

<!-- fields -->
- date: 2026-06-14
- link: https://example.com/compare/0.9.0...1.0.0

#### added  {#added}

- the trial-shaped fixture builder
";

/// A real git repo carrying the project cascade layer, one commit, and [`CHANGELOG_V2`] at
/// its literal placement home.
fn repo_with_changelog() -> TempDir {
    let dir = TempDir::new("repo");
    let root = dir.path();
    git(root, &["init", "-q"]);
    git(root, &["config", "user.email", "test@example.com"]);
    git(root, &["config", "user.name", "Test"]);
    fs::write(root.join("README.md"), "hello\n").expect("write file");
    fs::create_dir_all(root.join(".jigc").join("config")).expect("create project layer");
    fs::write(root.join("CHANGELOG.md"), CHANGELOG_V2).expect("write the committed changelog");
    git(root, &["add", "."]);
    git(root, &["commit", "-q", "-m", "seed the corpus"]);
    dir
}

/// Run `jigc <args>` with `cwd = repo`, `$HOME = home`, `JIGC_PACK_DIR = pack`.
fn jigc(repo: &Path, home: &Path, pack: &Path, args: &[&str]) -> std::process::Output {
    Command::new(env!("CARGO_BIN_EXE_jigc"))
        .args(args)
        .current_dir(repo)
        .env("HOME", home)
        .env("JIGC_PACK_DIR", pack)
        .output()
        .expect("run the jigc binary")
}

/// `jigc migrate-corpus --format json`, parsed — with the streams surfaced on a parse
/// failure so a pack-load refusal never reads as a test bug.
fn migrate_report(repo: &Path, home: &Path, pack: &Path) -> (serde_json::Value, bool) {
    let out = jigc(repo, home, pack, &["migrate-corpus", "--format", "json"]);
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

/// **The headline: a nested optional leaf migrates as the byte no-op it is.** An item that
/// lacks the bullet already conforms, so the fold places nothing and the stamp is the run's
/// only byte delta — at the third locus exactly as at the second.
///
/// Red at HEAD: `0 migrated / 1 blocked`, exit 1, `migrate-corpus.unclassified-change`,
/// routed at `crates/engine/src/schema_diff.rs`.
#[test]
fn a_nested_optional_leaf_migrates_as_a_byte_no_op() {
    let home = TempDir::new("home");
    let pack =
        pack_adding_a_nested_leaf("optional", "{ id: ticket, type: string, optional: true }");
    let repo = repo_with_changelog();

    let (report, ok) = migrate_report(repo.path(), home.path(), pack.path());
    assert!(ok, "a clean migration exits 0; report:\n{report:#}");
    assert_eq!(
        report["migrated"].as_array().map(|m| m.len()),
        Some(1),
        "the changelog migrates; report:\n{report:#}",
    );
    assert_eq!(
        fs::read_to_string(repo.path().join("CHANGELOG.md")).expect("read the changelog"),
        CHANGELOG_V2.replace("schema-version: 2", "schema-version: 3"),
        "a nested optional add is a byte no-op: the stamp is the migration's only delta",
    );
    let validated = jigc(repo.path(), home.path(), pack.path(), &["validate"]);
    assert!(
        validated.status.success(),
        "the migrated corpus validates clean; stdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&validated.stdout),
        String::from_utf8_lossy(&validated.stderr),
    );
}

/// **A required nested leaf routes the author, not the source tree.** The classifier reaches
/// `ProseNeeding` at the third locus, the section heading is already there so nothing is
/// minted, and the per-doc conformance gate adjudicates the committed nested item that now
/// lacks the leaf — which is the Framing-A handoff, in the doc's own words.
///
/// Red at HEAD: `migrate-corpus.unclassified-change`, routed at
/// `crates/engine/src/schema_diff.rs` — a file no adopter can edit, over a doc whose own prose
/// is the repair.
#[test]
fn a_required_nested_leaf_routes_the_author_not_the_source_tree() {
    let home = TempDir::new("home");
    let pack = pack_adding_a_nested_leaf("required", "{ id: owner, type: string }");
    let repo = repo_with_changelog();

    let (report, ok) = migrate_report(repo.path(), home.path(), pack.path());
    let finding = sole_blocked(&report, ok);
    assert_eq!(
        finding["code"], "migrate-corpus.prose-needed",
        "a required nested leaf is adjudicated by the per-doc gate; finding:\n{finding:#}",
    );
    let route = finding["route"].as_str().expect("a route");
    assert!(
        route.contains("CHANGELOG.md") && !route.contains("crates/engine"),
        "the route names the doc whose prose clears it, not a file no adopter can edit; \
         route: {route}",
    );
    assert_eq!(
        fs::read_to_string(repo.path().join("CHANGELOG.md")).expect("read the changelog"),
        CHANGELOG_V2,
        "the refused doc rolls back byte-identical, its stamp unmoved",
    );
}

/// **A nested leaf whose id also exists in the outer block is resolved at its own locus.**
///
/// `changelog.releases` declares `date` (`set: on-create`, no `default:`); this bump adds a
/// **nested** `date` that *does* carry a `default:`. A locus-blind fold resolves the leaf by
/// section id alone, finds the **outer** declaration, takes its no-default branch, writes
/// nothing — and restamps the doc: silent corruption at exit 0, a corpus stamped 3 whose
/// nested items never got the value the schema declares.
///
/// Resolved at its own locus the declaration carries a `default:`, so the cell is the
/// byte-writing one — un-built at the third locus (M50 Increment 7) — and the fold refuses,
/// naming the kind and the locus path instead of splicing at the wrong depth.
#[test]
fn a_nested_leaf_colliding_with_an_outer_id_resolves_at_its_own_locus() {
    let home = TempDir::new("home");
    let pack = pack_adding_a_nested_leaf(
        "collision",
        "{ id: date, type: date, default: \"1970-01-01\" }",
    );
    let repo = repo_with_changelog();

    let (report, ok) = migrate_report(repo.path(), home.path(), pack.path());
    let finding = sole_blocked(&report, ok);
    let message = finding["message"].as_str().expect("a message");
    assert_eq!(
        finding["code"], "migrate-corpus.fold-refused",
        "the nested declaration carries a value source, so the cell is the byte-writing one; \
         finding:\n{finding:#}",
    );
    assert!(
        message.contains("added-item-field") && message.contains("releases/changes"),
        "the refusal names the kind AND the locus path it sits at; message: {message}",
    );
    let route = finding["route"].as_str().expect("a route");
    assert!(
        route.contains("releases/changes"),
        "the route names the locus path too; route: {route}",
    );
    assert_eq!(
        fs::read_to_string(repo.path().join("CHANGELOG.md")).expect("read the changelog"),
        CHANGELOG_V2,
        "the refusal leaves the committed bytes — and the stamp — untouched",
    );
}
