//! Acceptance — **`AddedItemSlot` at the third locus** (M50 Increment 7 / T4;
//! `completions/artifacts/M50/settle-record.md` → D6 — the three carry-forwards).
//!
//! `crates/cli/tests/migrate_corpus_item_slot.rs` drives this kind's arity axis at the
//! **item** locus: 0→1, 1→2 optional, 1→2 required, 2→3, and the per-item re-run guard.
//! Until this task that whole cell-set stopped one level down — the classifier reached
//! `AddedItemSlot` at locus 3 (Increment 6) and the driver refused it, because
//! [`engine::write::insert_item_slot`] addressed an item at the second locus only and
//! splicing it at the wrong depth is worse than refusing. So `changelog.releases/changes`
//! — the one nested repeatable the shipped doctype set carries, and the shape a
//! changelog-like doctype actually evolves in — was frozen at its birth shape, its route a
//! file in this workspace.
//!
//! The axis here is **the same cell-set, one locus down**, driven through the shipped
//! binary over the shipped `changelog`: the nested `changes` block is declared explicitly at
//! the arity each cell needs, and every claim is read off the bytes the binary left on disk.
//! Two facts belong to this locus and to no other, and each has its own arm:
//!
//! - the sub-labels render at `#####`, not `####` — a nested item sits at `####`, and its
//!   leaves one level deeper (`engine::write::render_item_at`);
//! - `#### added` under two different releases is **two legal items**, so a refusal that
//!   named the bare anchor would not say which one — the located cause carries the whole
//!   chain (`releases/1-0-0/changes/added`), which is the address a reader follows.

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

use crate::support::frozen_pack;

// ---------------------------------------------------------------------------------------------
// Fixture plumbing (the shipped `migrate_locus_axis.rs` mechanism, over a dev-pack copy).
// ---------------------------------------------------------------------------------------------

/// A throwaway directory that removes itself on drop.
struct TempDir(PathBuf);

impl TempDir {
    fn new(tag: &str) -> Self {
        let mut path = std::env::temp_dir();
        path.push(format!(
            "jigc-nested-slot-{tag}-{}-{:?}",
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

/// The shipped `changelog` **nested** `changes` block's one declared leaf line — the anchor
/// every fixture below replaces. Its indentation is what distinguishes it from the staging
/// change-group include, which is the same text six columns out.
const NESTED_INCLUDE: &str = "              - include: change-group\n";

/// The staging block's include — the **control**: an item block at locus 2 of another
/// section, left untouched by every reshape here, so a fold that sprayed by section shape
/// rather than by locus path would show up in its bytes.
const STAGING_INCLUDE: &str = "        - include: change-group";

/// The nested block's `id-from` leaf. Every fixture arity keeps it: it is what names the
/// item, so a block without it would not load.
const CATEGORY: &str =
    "{ id: category, type: enum, of: [added, changed, deprecated, removed, fixed, security] }";

/// The shipped `notes` slot leaf, spelled out — the v1 leaf whose committed bare prose the
/// reshape re-keys under a `##### Notes` sub-label.
const NOTES: &str = "{ id: notes, slot: { hint: \"One bullet per change in this category.\" } }";

/// The added **optional** slot leaf: the one that mints empty.
const IMPACT: &str = "{ id: impact, slot: { optional: true, hint: \"Who this affects.\" } }";

/// The added **required** slot leaf — same relabel, adjudicated by the per-doc gate.
const IMPACT_REQUIRED: &str = "{ id: impact, slot: { hint: \"Who this affects.\" } }";

/// A third optional slot leaf, for the 2→3 arity.
const MORE: &str = "{ id: more, slot: { optional: true, hint: \"Still more.\" } }";

/// An optional **field** leaf — the shape that gives a nested item a `<!-- fields -->` group,
/// and so the one committed shape that can carry prose the parse does not model.
const TICKET: &str = "{ id: ticket, type: string, optional: true }";

/// The nested `changes` block's leaves, rendered at the include's **own** indentation — read
/// off the anchor, never restated, so a re-indent of the shipped schema cannot silently make
/// these fixtures test a different block.
fn nested_block(leaves: &[&str]) -> String {
    let indent = &NESTED_INCLUDE[..NESTED_INCLUDE.len() - NESTED_INCLUDE.trim_start().len()];
    leaves
        .iter()
        .map(|leaf| format!("{indent}- {leaf}\n"))
        .collect()
}

/// A dev-pack copy bumping `changelog` **2 → 3**, with its nested `changes` block declared
/// explicitly at two arities: `prior` is what `schema-snapshots/changelog.v2.yaml` carries
/// (the shape every committed instance conforms to), `current` what `schemas/changelog.yaml`
/// does. The manifest takes version 3 with its `schema-hash` re-pinned through the production
/// loader.
fn pack_reshaping_the_nested_block(tag: &str, prior: &[&str], current: &[&str]) -> TempDir {
    let dir = TempDir::new(tag);
    frozen_pack::copy_dev_pack(dir.path());

    let schema_path = dir.path().join("schemas").join("changelog.yaml");
    let shipped = fs::read_to_string(&schema_path).expect("read the copied changelog.yaml");
    let with = |leaves: &[&str]| {
        let out = shipped.replacen(NESTED_INCLUDE, &nested_block(leaves), 1);
        assert_ne!(
            out, shipped,
            "changelog.yaml must declare a nested change-group include",
        );
        assert_eq!(
            out.lines().filter(|line| *line == STAGING_INCLUDE).count(),
            1,
            "the staging include must be left alone — the reshape is at locus 3 only",
        );
        out
    };
    fs::write(
        dir.path()
            .join("schema-snapshots")
            .join("changelog.v2.yaml"),
        with(prior),
    )
    .expect("write the changelog.v2 snapshot");
    fs::write(&schema_path, with(current)).expect("write the reshaped changelog.yaml");

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

/// A real git repo carrying the project cascade layer, one commit, and `changelog` at the
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

/// `jigc migrate-corpus --format json`, parsed — with the streams surfaced on a parse failure
/// so a pack-load refusal never reads as a test bug.
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

/// The one migrated target of a clean run, with the exit asserted.
fn sole_migrated(report: &serde_json::Value, ok: bool) -> String {
    assert!(ok, "a clean migration exits 0; report:\n{report:#}");
    let migrated = report["migrated"]
        .as_array()
        .unwrap_or_else(|| panic!("the envelope carries `migrated[]`; got:\n{report:#}"));
    assert_eq!(
        migrated.len(),
        1,
        "exactly one doc migrated; report:\n{report:#}",
    );
    migrated[0]
        .as_str()
        .map(str::to_string)
        .unwrap_or_else(|| panic!("the migrated entry is a path; got:\n{report:#}"))
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

/// `jigc validate` over the migrated corpus — the migrated doc must gate clean at store scope.
fn assert_validates_clean(repo: &Path, home: &Path, pack: &Path) {
    let out = jigc(repo, home, pack, &["validate"]);
    assert!(
        out.status.success(),
        "the migrated corpus validates clean; stdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr),
    );
}

/// `render(parse(x)) == x` over the bytes the **binary wrote**, under the reshaped pack's own
/// `changelog` schema, loaded through the production loader.
///
/// The assertion is on the emitted artifact, never on a reconstruction of it: the file the
/// migration left on disk is fed straight back through the parser and the canonical writer —
/// so a literal below that happened to match a *non-canonical* reshape would still redden.
fn assert_byte_stable(pack: &Path, bytes: &str) {
    let source = cli::pack::FilesystemPack::new(pack.to_path_buf());
    let schema_bytes =
        fs::read(pack.join("schemas").join("changelog.yaml")).expect("read the reshaped schema");
    let schema =
        cli::pack::load_pack_schema(&source, &schema_bytes).expect("the reshaped schema loads");
    let instance = engine::write::instance_from_source(&schema, bytes)
        .expect("the migrated bytes re-parse under the new schema");
    assert_eq!(
        engine::write::render(&schema, &instance),
        bytes,
        "the migrated bytes must round-trip byte-identical under the new schema",
    );
}

/// The committed `CHANGELOG.md` as it stands.
fn changelog(repo: &Path) -> String {
    fs::read_to_string(repo.join("CHANGELOG.md")).expect("read the changelog")
}

// ---------------------------------------------------------------------------------------------
// Cell 1 — 0 → 1, optional.
// ---------------------------------------------------------------------------------------------

/// A conformant, **v2-stamped** changelog whose nested change-groups are **slot-less** — a
/// heading and nothing else, the shape a nested block declaring only its `id-from` leaf
/// commits to.
const NESTED_SLOTLESS: &str = "\
---
schema-version: 2
---

# Changelog

## Unreleased Changes

### changed  {#changed}

- the staging group is the control

## Releases

### 1.0.0  {#1-0-0}

<!-- fields -->
- date: 2026-06-14

#### added  {#added}
";

/// **0 → 1, optional — the stamp is the only byte delta, at the third locus too.** A lone
/// declared slot renders **bare**, under no sub-heading, and an item whose one slot is empty
/// renders identically to a slot-less one, so the committed bytes already *are* the v3 bytes.
#[test]
fn nested_zero_to_one_optional_migrates_with_the_stamp_as_its_only_delta() {
    let home = TempDir::new("home");
    let pack = pack_reshaping_the_nested_block(
        "zero-to-one",
        &[CATEGORY],
        &[
            CATEGORY,
            "{ id: notes, slot: { optional: true, hint: \"One bullet per change.\" } }",
        ],
    );
    let repo = repo_with(NESTED_SLOTLESS);

    let (report, ok) = migrate_report(repo.path(), home.path(), pack.path());
    assert_eq!(sole_migrated(&report, ok), "CHANGELOG.md");
    assert_eq!(
        changelog(repo.path()),
        NESTED_SLOTLESS.replace("schema-version: 2", "schema-version: 3"),
        "below two declared slots the fold writes no content bytes, one locus down as at the \
         one above",
    );
    assert_validates_clean(repo.path(), home.path(), pack.path());
}

// ---------------------------------------------------------------------------------------------
// Cells 2 and 3 — 1 → 2, optional and required. The headline.
// ---------------------------------------------------------------------------------------------

/// A conformant, **v2-stamped** changelog carrying **two** releases, each nesting **two**
/// change-groups whose prose is the **bare body** of the one declared slot — the shape every
/// committed instance of a single-slot nested block is in.
///
/// `#### added` appears under **both** releases on purpose: same-anchor nested items under
/// different parents are legal, so a walk that addressed one by its anchor alone would
/// reshape one of them twice and the other never. The staging change-group is the control.
const NESTED_BARE: &str = "\
---
schema-version: 2
---

# Changelog

## Unreleased Changes

### changed  {#changed}

- the staging group is the control

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

/// **1 → 2, optional — the headline.** Every nested change-group's committed prose rides
/// verbatim under its v2 leaf's `##### Notes` sub-label and the added leaf mints empty at its
/// schema-ordered offset — at `#####`, because a nested item sits at `####` and its leaves
/// render one level deeper. All four nested groups are reshaped, both `#### added` items are
/// reached under their own parents, and the staging section's own change-group — an item block
/// at locus 2 of another section — is byte-untouched.
#[test]
fn nested_one_to_two_optional_carries_the_committed_prose_under_its_sub_label() {
    let home = TempDir::new("home");
    let pack = pack_reshaping_the_nested_block(
        "one-to-two",
        &[CATEGORY, NOTES],
        &[CATEGORY, NOTES, IMPACT],
    );
    let repo = repo_with(NESTED_BARE);

    let (report, ok) = migrate_report(repo.path(), home.path(), pack.path());
    assert_eq!(sole_migrated(&report, ok), "CHANGELOG.md");

    let after = changelog(repo.path());
    assert_eq!(
        after,
        "\
---
schema-version: 3
---

# Changelog

## Unreleased Changes

### changed  {#changed}

- the staging group is the control

## Releases

### 1.1.0  {#1-1-0}

<!-- fields -->
- date: 2026-07-01

#### added  {#added}

##### Notes

- the group-count axis

##### Impact

#### fixed  {#fixed}

##### Notes

- the anchor-collision arm

##### Impact

### 1.0.0  {#1-0-0}

<!-- fields -->
- date: 2026-06-14
- link: https://example.com/compare/0.9.0...1.0.0

#### added  {#added}

##### Notes

- the trial-shaped fixture builder

##### Impact

#### changed  {#changed}

##### Notes

- a nested group under the older release

##### Impact
",
        "each nested group's prose rides verbatim under `##### Notes`, `##### Impact` mints \
         empty, and the staging group is untouched",
    );
    assert_byte_stable(pack.path(), &after);
    assert_validates_clean(repo.path(), home.path(), pack.path());

    // The re-run: the corpus is at the current version, so a second pass has nothing to fold
    // and leaves the bytes exactly as the first left them.
    let (again, ok_again) = migrate_report(repo.path(), home.path(), pack.path());
    assert!(
        ok_again,
        "a re-run over a migrated corpus exits 0; report:\n{again:#}"
    );
    assert_eq!(
        changelog(repo.path()),
        after,
        "the re-run is a byte no-op over the migrated corpus",
    );
}

/// **1 → 2, required — the doc is routed at the author, not at the source tree.** One kind
/// carries both requirednesses because the relabel is the same byte work either way; the
/// **per-doc conformance gate** adjudicates the empty required leaf, so the refusal is the
/// doc-authorable `migrate-corpus.prose-needed` and the bytes roll back with the stamp
/// unmoved — one locus down exactly as at the item locus.
#[test]
fn nested_one_to_two_required_routes_at_the_author_never_at_the_source_tree() {
    let home = TempDir::new("home");
    let pack = pack_reshaping_the_nested_block(
        "required",
        &[CATEGORY, NOTES],
        &[CATEGORY, NOTES, IMPACT_REQUIRED],
    );
    let repo = repo_with(NESTED_BARE);

    let (report, ok) = migrate_report(repo.path(), home.path(), pack.path());
    let finding = sole_blocked(&report, ok);
    assert_eq!(
        finding["code"], "migrate-corpus.prose-needed",
        "a required added nested slot is adjudicated by the gate, so its route is the \
         doc-authorable one; finding:\n{finding:#}",
    );
    let route = finding["route"].as_str().expect("a route");
    assert!(
        route.contains("CHANGELOG.md") && !route.contains("crates/engine"),
        "the route names the doc whose prose clears it, not a file no adopter can edit; \
         route: {route}",
    );
    assert_eq!(
        changelog(repo.path()),
        NESTED_BARE,
        "a refused doc rolls back byte-identical — the stamp never moves",
    );
}

// ---------------------------------------------------------------------------------------------
// Cell 4 — 2 → 3.
// ---------------------------------------------------------------------------------------------

/// A conformant, **v2-stamped** changelog whose nested change-group already carries **both**
/// declared sub-labels — the shape the 2→3 arity migrates from.
const NESTED_TWO_SLOTS: &str = "\
---
schema-version: 2
---

# Changelog

## Unreleased Changes

### changed  {#changed}

- the staging group is the control

## Releases

### 1.0.0  {#1-0-0}

<!-- fields -->
- date: 2026-06-14

#### added  {#added}

##### Notes

- the trial-shaped fixture builder

##### Impact

Everyone who cuts a release.
";

/// **2 → 3 — the new sub-label is minted and every existing one is byte-untouched.** This is
/// the arity that keys the per-item idempotency guard on the **added** leaf rather than on
/// "any declared sub-label": the item already carries sub-labels while still needing the new
/// one, so an any-sub-label guard would skip exactly the item the change exists to reshape.
#[test]
fn nested_two_to_three_mints_the_new_sub_label_with_the_existing_ones_untouched() {
    let home = TempDir::new("home");
    let pack = pack_reshaping_the_nested_block(
        "two-to-three",
        &[CATEGORY, NOTES, IMPACT],
        &[CATEGORY, NOTES, IMPACT, MORE],
    );
    let repo = repo_with(NESTED_TWO_SLOTS);

    let (report, ok) = migrate_report(repo.path(), home.path(), pack.path());
    assert_eq!(sole_migrated(&report, ok), "CHANGELOG.md");
    let after = changelog(repo.path());
    assert_eq!(
        after,
        NESTED_TWO_SLOTS.replace("schema-version: 2", "schema-version: 3") + "\n##### More\n",
        "the existing sub-labels and their prose survive byte-for-byte; only the new leaf's \
         empty sub-label is added, at its schema-ordered offset",
    );
    assert_byte_stable(pack.path(), &after);
    assert_validates_clean(repo.path(), home.path(), pack.path());
}

// ---------------------------------------------------------------------------------------------
// Cell 5 — the re-run, over a doc holding nested items in both states.
// ---------------------------------------------------------------------------------------------

/// A **v2-stamped** changelog whose first nested change-group already renders the v3
/// sub-labels (an earlier run whose commit was refused, or a hand-authored entry) while the
/// second is still bare.
const NESTED_HALF_RESHAPED: &str = "\
---
schema-version: 2
---

# Changelog

## Unreleased Changes

### changed  {#changed}

- the staging group is the control

## Releases

### 1.0.0  {#1-0-0}

<!-- fields -->
- date: 2026-06-14

#### added  {#added}

##### Notes

- the trial-shaped fixture builder

##### Impact

Already reshaped by an earlier run.

#### fixed  {#fixed}

- the anchor-collision arm
";

/// **The re-run guard is per nested item.** One doc can hold a nested item that already
/// carries the added sub-label beside one that does not, so the guard cannot live in a
/// whole-change filter: the already-reshaped group is left **byte-identical** — its authored
/// `impact` prose is not re-keyed, re-nested or lost — while the bare one is reshaped in the
/// same run.
#[test]
fn a_re_run_leaves_an_already_reshaped_nested_item_byte_identical() {
    let home = TempDir::new("home");
    let pack =
        pack_reshaping_the_nested_block("re-run", &[CATEGORY, NOTES], &[CATEGORY, NOTES, IMPACT]);
    let repo = repo_with(NESTED_HALF_RESHAPED);

    let (report, ok) = migrate_report(repo.path(), home.path(), pack.path());
    assert_eq!(sole_migrated(&report, ok), "CHANGELOG.md");

    let after = changelog(repo.path());
    let untouched = NESTED_HALF_RESHAPED
        .split("#### fixed")
        .next()
        .expect("the already-reshaped group's bytes")
        .replace("schema-version: 2", "schema-version: 3");
    assert!(
        after.starts_with(&untouched),
        "the already-reshaped nested group keeps its bytes verbatim; got:\n{after}",
    );
    assert!(
        after.ends_with(
            "#### fixed  {#fixed}\n\n##### Notes\n\n- the anchor-collision arm\n\n##### Impact\n"
        ),
        "the still-bare nested group is reshaped in the same run; got:\n{after}",
    );
    assert_byte_stable(pack.path(), &after);
    assert_validates_clean(repo.path(), home.path(), pack.path());
}

// ---------------------------------------------------------------------------------------------
// The located refusal — same-anchor nested items under different parents.
// ---------------------------------------------------------------------------------------------

/// A **v2-stamped** changelog whose two releases each nest a `#### added` group, and whose
/// **older** release's group carries a paragraph hand-appended after its `<!-- fields -->`
/// group — where "append a sentence to this entry" lands, since the canonical order is
/// slots-then-fields. The parse does not model it: `doc show` renders the slot without it and
/// `jigc validate` reports nothing about it.
const NESTED_UNMODELLED: &str = "\
---
schema-version: 2
---

# Changelog

## Unreleased Changes

### changed  {#changed}

- the staging group is the control

## Releases

### 1.1.0  {#1-1-0}

<!-- fields -->
- date: 2026-07-01

#### added  {#added}

- the group-count axis

<!-- fields -->
- ticket: T-1

### 1.0.0  {#1-0-0}

<!-- fields -->
- date: 2026-06-14

#### added  {#added}

- the trial-shaped fixture builder

<!-- fields -->
- ticket: T-2

THE COMMITTED PROSE THAT MUST SURVIVE.
";

/// **The refusal names the whole chain, because the bare anchor names two legal items.**
/// The reshape replaces a nested item's **whole committed region**, so a byte the parse does
/// not model would be destroyed by it — **No-data-loss** is a declared property of this pair,
/// so the fold checks its pre-image and refuses instead.
///
/// At this locus the located cause has to say *which* `#### added`: both releases carry one,
/// and they are two legal items. So the message and the route carry
/// `releases/1-0-0/changes/added` — the item id, the nested-section hop and the nested item id
/// — and not the conformant sibling under the other release.
#[test]
fn the_unmodelled_refusal_names_the_nested_item_by_its_whole_chain() {
    let home = TempDir::new("home");
    let pack = pack_reshaping_the_nested_block(
        "unmodelled",
        &[CATEGORY, NOTES, TICKET],
        &[CATEGORY, NOTES, TICKET, IMPACT],
    );
    let repo = repo_with(NESTED_UNMODELLED);

    let (report, ok) = migrate_report(repo.path(), home.path(), pack.path());
    let finding = sole_blocked(&report, ok);
    assert_eq!(
        finding["code"], "migrate-corpus.item-unmodelled-content",
        "the refusal names its own cause; finding:\n{finding:#}",
    );
    let message = finding["message"].as_str().expect("a message");
    assert!(
        message.contains("releases/1-0-0/changes/added"),
        "the cause locates the nested item by its whole chain — the address a reader \
         follows; message: {message}",
    );
    assert!(
        !message.contains("releases/1-1-0/changes/added"),
        "…and not the conformant same-anchor sibling under the other release; \
         message: {message}",
    );
    let route = finding["route"].as_str().expect("a route");
    assert!(
        !route.contains("crates/engine"),
        "the repair is in the doc, not in a file no adopter can edit; route: {route}",
    );

    assert_eq!(
        changelog(repo.path()),
        NESTED_UNMODELLED,
        "a refused doc rolls back byte-identical — the committed prose survives and the stamp \
         never moves",
    );
}
