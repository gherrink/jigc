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
//! 2. **The fold resolves a nested leaf at its own locus — and writes there.** A nested leaf
//!    whose id also exists in the outer block is the shipped collision (`date` on
//!    `changelog.releases`), and there are two ways to get it wrong: a locus-blind fold
//!    resolves it against the **outer** declaration, takes that field's no-op branch, writes
//!    nothing and restamps the doc (Increment 6); and a fold that resolves it correctly but
//!    splices through the second-locus primitive lands the bullet on the **release**, over
//!    the release's own committed `date` (Increment 7 / T3). Driven on exactly that pair
//!    through the shipped binary, over a corpus whose two releases each nest two
//!    change-groups — two of them sharing an anchor.

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
/// - **no locus has a half-built cell** — the `Unbuilt` set is empty over the whole of
///   `1..=LOCI`, asserted rather than remarked. Increment 6 left three byte-writing arms open
///   at locus 3 and this loop covered loci 1 and 2 only, so the exemption lived in a comment;
///   M50 Increment 7 folded all three (T3–T5), and with the range widened it is the *test*
///   that refuses the next half-built cell rather than a reader noticing the comment went
///   stale ([`engine::schema_diff::LocusDisposition::Unbuilt`] is kept for that next kind);
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

    for locus in 1..=LOCI {
        let unbuilt: Vec<&str> = SchemaChangeKind::ALL
            .iter()
            .filter(|k| locus_disposition(**k, locus) == LocusDisposition::Unbuilt)
            .map(|k| k.as_str())
            .collect();
        assert!(
            unbuilt.is_empty(),
            "locus {locus} has a half-built cell: {unbuilt:?}. Every `kind × locus` cell folds \
             or refuses by design at HEAD, so a cell landing back on `Unbuilt` means an arm was \
             removed, or a kind was added without one — build it before the bump ships.",
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

/// A real git repo carrying the project cascade layer, one commit, and `changelog` at the
/// `changelog` doctype's literal placement home.
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

/// [`repo_with`] over the one-release [`CHANGELOG_V2`] fixture.
fn repo_with_changelog() -> TempDir {
    repo_with(CHANGELOG_V2)
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

/// A conformant, **v2-stamped** `CHANGELOG.md` carrying **two** releases, each nesting **two**
/// change-groups — so *every nested change-group* is a real claim rather than a sample of one,
/// and the two releases make the outer hop of the walk real too. The staging section's own
/// change-group is the control: it is an item block at locus **2 of another section**, so a
/// locus-3 fold that sprayed by section-shape rather than by locus path would land there.
///
/// `#### added` appears under **both** releases on purpose: same-anchor nested items under
/// different parents are legal, and a walk that addressed a nested item by its anchor alone
/// would write one of them twice and the other never.
const CHANGELOG_V2_NESTED: &str = "\
---
schema-version: 2
---

# Changelog

## Unreleased Changes

### changed  {#changed}

- the fixture builder gained shape-class coverage

## Releases

### 1.1.0  {#1-1-0}

<!-- fields -->
- date: 2026-07-01

#### added  {#added}

- the group-count axis

#### fixed  {#fixed}

- the anchor-collision arm

### 1.0.0  {#1-0-0}

<!-- fields -->
- date: 2026-06-14
- link: https://example.com/compare/0.9.0...1.0.0

#### added  {#added}

- the trial-shaped fixture builder

#### changed  {#changed}

- a nested group under the older release
";

/// The value the nested `date` leaf declares as its `default:` — the deterministic bytes the
/// fold owes every nested change-group.
const NESTED_DEFAULT: &str = "1970-01-01";

/// The canonical **v3** form of [`CHANGELOG_V2_NESTED`]: the stamp, and one
/// `- date: 1970-01-01` field group on each of the four nested change-groups. Pinned as bytes
/// because *where* the bullet lands is the claim — a group's field group renders after its
/// prose, one level inside the group, and nowhere near the release's own.
const CHANGELOG_V3_NESTED: &str = "\
---
schema-version: 3
---

# Changelog

## Unreleased Changes

### changed  {#changed}

- the fixture builder gained shape-class coverage

## Releases

### 1.1.0  {#1-1-0}

<!-- fields -->
- date: 2026-07-01

#### added  {#added}

- the group-count axis

<!-- fields -->
- date: 1970-01-01

#### fixed  {#fixed}

- the anchor-collision arm

<!-- fields -->
- date: 1970-01-01

### 1.0.0  {#1-0-0}

<!-- fields -->
- date: 2026-06-14
- link: https://example.com/compare/0.9.0...1.0.0

#### added  {#added}

- the trial-shaped fixture builder

<!-- fields -->
- date: 1970-01-01

#### changed  {#changed}

- a nested group under the older release

<!-- fields -->
- date: 1970-01-01
";

/// **A nested leaf whose id also exists in the outer block is resolved — and SPLICED — at its
/// own locus.** (M50 Increment 7 / T3.)
///
/// `changelog.releases` declares `date` (`set: on-create`, no `default:`); this bump adds a
/// **nested** `date` that *does* carry a `default:`. Two ways to get this wrong, and the arm
/// catches both: a locus-blind fold resolves the leaf by section id alone, finds the **outer**
/// declaration, takes its no-default branch, writes nothing and restamps the doc (M50
/// Increment 6 closed that half); and a fold that resolves the declaration correctly but
/// splices through the second-locus primitive writes the bullet onto the **release**, over the
/// release's own committed `date`.
///
/// Red at HEAD: `0 migrated / 1 blocked`, exit 1, `migrate-corpus.fold-refused` naming
/// `added-item-field` at `releases/changes` — the byte-writing arm the third locus did not have.
#[test]
fn a_nested_leaf_colliding_with_an_outer_id_splices_at_its_own_locus() {
    let home = TempDir::new("home");
    let pack = pack_adding_a_nested_leaf(
        "collision",
        &format!("{{ id: date, type: date, default: \"{NESTED_DEFAULT}\" }}"),
    );
    let repo = repo_with(CHANGELOG_V2_NESTED);
    let changelog = repo.path().join("CHANGELOG.md");

    let (report, ok) = migrate_report(repo.path(), home.path(), pack.path());
    assert!(ok, "the locus-3 fold migrates cleanly; report:\n{report:#}");
    assert_eq!(
        report["migrated"].as_array().map(|m| m.len()),
        Some(1),
        "the changelog migrates; report:\n{report:#}",
    );

    let migrated = fs::read_to_string(&changelog).expect("read the migrated changelog");

    // (a) EVERY nested change-group gained the declared default — four of them, under two
    //     releases, two of which share an anchor.
    let nested_groups = CHANGELOG_V2_NESTED.matches("\n#### ").count();
    assert_eq!(nested_groups, 4, "the fixture nests four change-groups");
    assert_eq!(
        migrated
            .matches(&format!("- date: {NESTED_DEFAULT}\n"))
            .count(),
        nested_groups,
        "every nested change-group gains the declared default; migrated:\n{migrated}",
    );

    // (b) The RELEASE's own `date` — the colliding outer leaf — is byte-identical. A splice
    //     at the second locus would have overwritten it with the nested declaration's default.
    for release_date in ["- date: 2026-07-01\n", "- date: 2026-06-14\n"] {
        assert!(
            migrated.contains(release_date),
            "the release's own committed date survives; `{release_date:?}` is gone from:\n{migrated}",
        );
    }

    // (c) The staging section is an item block too, at locus 2 of another section — untouched.
    let staging = |doc: &str| {
        let start = doc
            .find("## Unreleased Changes")
            .expect("the staging section");
        let end = doc.find("## Releases").expect("the releases section");
        doc[start..end].to_string()
    };
    assert_eq!(
        staging(&migrated),
        staging(CHANGELOG_V2_NESTED),
        "the staging change-group is at another locus and gains nothing",
    );

    // (d) …and the whole file, byte for byte: the four bullets and the stamp are the ONLY delta.
    assert_eq!(
        migrated, CHANGELOG_V3_NESTED,
        "the migrated bytes are the canonical v3 form",
    );

    // (e) The migrated corpus validates clean at its new stamp.
    let validated = jigc(repo.path(), home.path(), pack.path(), &["validate"]);
    assert!(
        validated.status.success(),
        "the migrated corpus validates clean; stdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&validated.stdout),
        String::from_utf8_lossy(&validated.stderr),
    );

    // (f) …and a second run is a byte no-op: the fold converges rather than re-splicing.
    let (again, ok_again) = migrate_report(repo.path(), home.path(), pack.path());
    assert!(ok_again, "the second run exits 0; report:\n{again:#}");
    assert_eq!(
        fs::read_to_string(&changelog).expect("read the changelog"),
        migrated,
        "a second `migrate-corpus` writes no bytes; report:\n{again:#}",
    );
}

// ---------------------------------------------------------------------------------------------
// The unfilled advisory at the ITEM LOCI — a route whose emitted commands RUN.
// ---------------------------------------------------------------------------------------------

/// The doc identity the corpus's one `changelog` answers to.
const DOC_ADDRESS: &str = "changelog:changelog";
/// The value a reader supplies where the emitted write says `<value>`.
const READER_VALUE: &str = "v1";
/// The leaf both arms add — `set:`-derived with no `default:`, which is exactly what the fold
/// places no bytes for and the loudness rider reports.
const UNFILLED_LEAF: &str = "{ id: stamped, type: string, set: on-create, optional: true }";

/// The shipped `changelog` **outer** item block's optional-field line — the anchor a locus-2 leaf
/// is appended after. Its indentation, six columns *shallower* than the nested block's, is what
/// makes the appended leaf land at locus 2 rather than 3.
const OUTER_ANCHOR: &str = "        - { id: link, type: string, optional: true }\n";

/// A dev-pack copy bumping `changelog` **2 → 3** with `added` appended to its **outer** `releases`
/// item block — [`pack_adding_a_nested_leaf`]'s locus-2 twin, so the two arms below differ only in
/// the locus the leaf lands at.
fn pack_adding_an_outer_leaf(tag: &str, added: &str) -> TempDir {
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

    let indent = &OUTER_ANCHOR[..OUTER_ANCHOR.len() - OUTER_ANCHOR.trim_start().len()];
    let reshaped = shipped.replacen(
        OUTER_ANCHOR,
        &format!("{OUTER_ANCHOR}{indent}- {added}\n"),
        1,
    );
    assert_ne!(
        reshaped, shipped,
        "changelog.yaml must declare the outer block's optional `link` field",
    );
    // The anchor must name exactly one line: a second match would put the leaf somewhere the arm
    // does not claim, and the whole point of the pair is a leaf at a KNOWN locus.
    assert_eq!(
        shipped
            .lines()
            .filter(|line| format!("{line}\n") == OUTER_ANCHOR)
            .count(),
        1,
        "the outer-block anchor must be unique",
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

/// The backtick-closed span of `route` that starts at `verb` — the emitted command, exactly as a
/// reader copies it out.
fn emitted_command<'a>(route: &'a str, verb: &str) -> &'a str {
    let start = route
        .find(verb)
        .unwrap_or_else(|| panic!("the route emits a `{verb}` command; route: {route}"));
    let rest = &route[start..];
    let end = rest.find('`').unwrap_or_else(|| {
        panic!("the emitted `{verb}` command is backtick-closed; route: {route}")
    });
    &rest[..end]
}

/// One emitted address, with **every hole a reader fills in** replaced by this corpus's real
/// values: `<doc-address>` by the doc identity, and each item-id hole — whatever it is spelled —
/// by the next real id in `ids`, outermost first.
///
/// The item substitution is **positional**, not name-keyed: the claim is *a reader who fills each
/// id hole reaches something*, which must hold for whatever placeholder vocabulary the route
/// picks, not only the one shipped today.
fn reader_filled_address(address: &str, ids: &mut std::slice::Iter<'_, &str>) -> String {
    let raw = address.replace("<doc-address>", DOC_ADDRESS);
    let (head, fragment) = raw
        .split_once('#')
        .unwrap_or_else(|| panic!("the emitted address carries a fragment; address: {raw}"));
    let hops: Vec<String> = fragment
        .split('/')
        .map(|hop| {
            if hop.starts_with('<') && hop.ends_with('>') && hop.contains("item-id") {
                (*ids
                    .next()
                    .expect("the route names no more id hops than the corpus has"))
                .to_string()
            } else {
                hop.to_string()
            }
        })
        .collect();
    format!("{head}#{}", hops.join("/"))
}

/// An emitted `jigc …` command turned into the argv a reader actually runs: the leading `jigc`
/// dropped, every placeholder filled from this corpus. **Nothing else is rewritten** — the verb,
/// the flags and their order are the route's own bytes, which is the only way an argv the CLI
/// does not accept can be caught.
fn reader_filled_argv(command: &str, ids: &[&str], task: &str) -> Vec<String> {
    let mut ids = ids.iter();
    command
        .split_whitespace()
        .skip(1)
        .map(|token| match token {
            "<task-id>" => task.to_string(),
            "<value>" => READER_VALUE.to_string(),
            _ if token.contains('#') => reader_filled_address(token, &mut ids),
            _ => token.to_string(),
        })
        .collect()
}

/// Run `jigc <argv>` — the owned-`String` form of [`jigc`], for an argv assembled at runtime.
fn jigc_argv(repo: &Path, home: &Path, pack: &Path, argv: &[String]) -> std::process::Output {
    let args: Vec<&str> = argv.iter().map(String::as_str).collect();
    jigc(repo, home, pack, &args)
}

/// Install the adapter and mint one task, returning its id — the `--task <task-id>` the emitted
/// write needs, which the migration report itself does not hold.
fn task_in(repo: &Path, home: &Path, pack: &Path) -> String {
    let setup = jigc(repo, home, pack, &["setup"]);
    assert!(
        setup.status.success(),
        "setup installs; stdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&setup.stdout),
        String::from_utf8_lossy(&setup.stderr),
    );
    let started = jigc(
        repo,
        home,
        pack,
        &[
            "start",
            "stamp the change group",
            "--workflow",
            "single-task",
        ],
    );
    let out = String::from_utf8_lossy(&started.stdout);
    assert!(
        started.status.success(),
        "a work-workflow mints a task; stdout:\n{out}\nstderr:\n{}",
        String::from_utf8_lossy(&started.stderr),
    );
    let marker = "task minted: ";
    let start = out
        .find(marker)
        .unwrap_or_else(|| panic!("the mint acks its id; stdout:\n{out}"))
        + marker.len();
    out[start..]
        .split_whitespace()
        .next()
        .expect("the ack names an id")
        .to_string()
}

/// Drive the unfilled advisory's emitted route over `pack`: fill each id hole from `ids` and run
/// **the whole argv**, then read the value back at `read_back`.
fn the_emitted_route_runs(pack: &TempDir, ids: &[&str], read_back: &str) {
    let home = TempDir::new("home");
    let repo = repo_with_changelog();

    let (report, ok) = migrate_report(repo.path(), home.path(), pack.path());
    assert!(
        ok,
        "an unfilled `set:` leaf is reported, never blocking; report:\n{report:#}"
    );
    let unfilled = report["unfilled"]
        .as_array()
        .unwrap_or_else(|| panic!("the envelope carries `unfilled[]`; got:\n{report:#}"));
    assert_eq!(
        unfilled.len(),
        1,
        "exactly one leaf was left unfilled; report:\n{report:#}",
    );
    let finding = &unfilled[0];
    assert_eq!(finding["code"], "migrate-corpus.set-field-unfilled");
    let route = finding["route"].as_str().expect("a route");

    let task = task_in(repo.path(), home.path(), pack.path());

    // The read: the reader's only source of the ids the write needs, so it must both run and
    // render every anchor the write address asks for.
    let show = reader_filled_argv(emitted_command(route, "jigc doc show "), ids, &task);
    let read = jigc_argv(repo.path(), home.path(), pack.path(), &show);
    let read_out = String::from_utf8_lossy(&read.stdout);
    assert!(
        read.status.success(),
        "the emitted read must run; `{show:?}` gave:\n{read_out}\n{}",
        String::from_utf8_lossy(&read.stderr),
    );
    for id in ids {
        assert!(
            read_out.contains(&format!("{{#{id}}}")),
            "the emitted read hands back every id hop the write needs — `{id}` is missing from \
             `{show:?}`'s render:\n{read_out}",
        );
    }

    // The write: run it verbatim, its id holes filled from the read above.
    let set = reader_filled_argv(emitted_command(route, "jigc doc set-field "), ids, &task);
    let wrote = jigc_argv(repo.path(), home.path(), pack.path(), &set);
    assert!(
        wrote.status.success(),
        "the emitted write must run; `{set:?}` gave:\n{}\n{}",
        String::from_utf8_lossy(&wrote.stdout),
        String::from_utf8_lossy(&wrote.stderr),
    );

    // And it landed on the item the advisory said the leaf was missing from.
    let back = jigc(
        repo.path(),
        home.path(),
        pack.path(),
        &["doc", "show", read_back, "--task", &task],
    );
    let back_out = String::from_utf8_lossy(&back.stdout);
    assert!(
        back.status.success() && back_out.contains(&format!("stamped: {READER_VALUE}")),
        "the value lands on the item the advisory named; read-back of `{read_back}`:\n{back_out}\n{}",
        String::from_utf8_lossy(&back.stderr),
    );
}

/// **The unfilled advisory's emitted commands RUN at the deepest item locus** — verbatim, with
/// only the reader's own values filled in. (M50 Increment 6 audit.)
///
/// The cell is new: before this increment `diff_item_fields` pushed the backstop for any nested
/// delta, so no nested `set:` leaf ever reached the loudness rider. Reaching it, the route
/// interpolated the locus's **`Display` path** (`releases/changes`) into both verb positions — and
/// that path is a *diagnostic* locus, not an address: it omits the outer item hop, so no
/// substitution of the single `<item-id>` it offered could make either command resolve.
///
/// Red at HEAD, on the bytes themselves: the emitted read exits 1 on `store.no-such-item`
/// (`changes` read as an item of `releases`); with the read's address hand-repaired, the emitted
/// write's address exits 1 on `store.no-such-section`.
///
/// Green: the read names the **resolvable outer section**, whose one render hands the reader both
/// anchors (`{#1-0-0}` and `{#added}`); the write carries **one id hole per item hop**, so filling
/// them addresses the leaf's own home and the value lands there.
#[test]
fn the_nested_unfilled_advisory_emits_commands_that_run() {
    the_emitted_route_runs(
        &pack_adding_a_nested_leaf("unfilled-nested", UNFILLED_LEAF),
        &["1-0-0", "added"],
        "changelog:changelog#releases/1-0-0/changes/added",
    );
}

/// **The same claim at the item locus above it** — the axis, not the reported repro.
///
/// The advisory's route is one code path over both item loci, so the two breaks the nested arm
/// exposed were never nested-only: the emitted write passed its value as a **bare positional**
/// (`jigc doc set-field` answers `error: unexpected argument`), and the emitted read carried
/// `--task <task-id>` against a doc the run had just **committed** and no task had staged
/// (`store.not-staged`). Both were shipped at this locus since M46 and reported by nobody; a test
/// that iterated only the locus the finding named would have shipped them again.
#[test]
fn the_item_locus_unfilled_advisory_emits_commands_that_run() {
    the_emitted_route_runs(
        &pack_adding_an_outer_leaf("unfilled-outer", UNFILLED_LEAF),
        &["1-0-0"],
        "changelog:changelog#releases/1-0-0",
    );
}
