//! Acceptance — **the rest of `item_field`'s filter** (M49 Increment 4, T2;
//! `design/corpus-migration.md` → :188 *the kind must exist rather than ride the backstop* ·
//! The empty-diff backstop; `implementation/doctype-authoring.md` → the `OptionalRelaxed` row).
//!
//! T1 opened the item locus for an **added** slot leaf. Three item-block deltas were still
//! dropped on the floor by the same filter, and each is dropped **silently** the moment it
//! rides alongside any classified change — the residual's own declared bound (a non-empty diff
//! never reaches the backstop):
//!
//! - a **removed** item slot — the item-locus twin of `RemovedField`, whose
//!   recorded pick is *refuse, not strip*, and whose message must name a **slot**: the prose it
//!   would destroy is not a field line;
//! - a **nested repeatable** leaf added, removed, or reshaped — no kind is built for it (T1-2's
//!   `AddedNestedRepeatable` stays deferred, its trigger untouched), so all this change does is
//!   make the delta **name itself**: it emits the backstop kind explicitly instead of vanishing
//!   into a non-empty diff;
//! - an item slot's own **`optional:`** delta — the leaf-kind the shared rule never reached,
//!   which is what made `doctype-authoring.md`'s `OptionalRelaxed` row ("at both loci … and at
//!   the slot level") false.
//!
//! **Where each arm drives its delta is chosen so the arm is red at HEAD**, which is the only
//! way the test proves anything. For the two *dropped-change* classes (a removed slot, a nested
//! repeatable) that is **alongside a classified change** — an `AddedItemField` in the same bump
//! — because that is exactly where HEAD is silent: it migrates and restamps the doc. For the
//! two `optional:` deltas the drop leaves **no** classified kind behind, so HEAD's observable
//! is the residual's refusal (`unclassified-change`, routed at a file no adopter can edit) and
//! the delta is driven **alone**; riding it alongside a companion is behaviour-identical to the
//! fixed build by construction (both directions fold to the same bytes there), so it would be a
//! green-at-HEAD test that proves nothing.
//!
//! Every arm drives the **shipped binary** over a manufactured `JIGC_PACK_DIR` methodology pack
//! whose reshaped doctype is re-pinned through the production loader (`support::frozen_pack`) —
//! the act a pack author performs — and asserts the **bytes on disk** the run left.

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

use crate::support::frozen_pack;

// ---------------------------------------------------------------------------------------------
// Fixture plumbing (the shipped `migrate_corpus_halt_causes.rs` / `migrate_corpus_item_slot.rs`
// mechanism).
// ---------------------------------------------------------------------------------------------

/// A throwaway directory that removes itself on drop.
struct TempDir(PathBuf);

impl TempDir {
    fn new(tag: &str) -> Self {
        let mut path = std::env::temp_dir();
        path.push(format!(
            "jigc-itemleaf-{tag}-{}-{:?}",
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

/// Pin `ty`'s manifest `schema-version` to **2** in the pack copy at `pack`, whatever it
/// shipped at — the version the fixture pairs below are built against (the `v1` snapshot is
/// the prior shape, so a doc stamped 1 migrates to 2).
///
/// It reads the shipped version rather than asserting `1`, so a doctype this suite uses as a
/// host can be bumped for real in the pack (M49 Inc-9 shipped `completion-record` at **2**)
/// without these arms going red on an incidental number: what they assert is the *pair* they
/// manufacture, not the version it is labelled with.
fn bump_manifest(pack: &Path, ty: &str) {
    let manifest_path = pack.join("config").join("schema-manifest.yaml");
    let manifest = fs::read_to_string(&manifest_path).expect("read the copied manifest");
    let anchor = format!("- type: {ty}\n    schema-version: ");
    let at = manifest
        .find(&anchor)
        .unwrap_or_else(|| panic!("the manifest must declare a `{ty}` entry"))
        + anchor.len();
    let end = at
        + manifest[at..]
            .find('\n')
            .expect("the schema-version line terminates");
    let mut pinned = String::with_capacity(manifest.len());
    pinned.push_str(&manifest[..at]);
    pinned.push('2');
    pinned.push_str(&manifest[end..]);
    fs::write(&manifest_path, pinned).expect("write the pinned manifest");
}

/// A methodology-pack copy whose doctype `ty` is bumped `1 → 2`: the `v1` snapshot is `prior`'s
/// output over the shipped body, the current schema is `current`'s, and the manifest carries the
/// bumped version with its `schema-hash` re-pinned through the production loader.
fn pack_bumping(
    tag: &str,
    ty: &str,
    prior: impl FnOnce(&str) -> String,
    current: impl FnOnce(&str) -> String,
) -> TempDir {
    let dir = TempDir::new(tag);
    frozen_pack::copy_methodology_pack(dir.path());

    let schema_path = dir.path().join("schemas").join(format!("{ty}.yaml"));
    let shipped = fs::read_to_string(&schema_path).expect("read the copied schema");

    let prior_body = prior(&shipped);
    fs::write(
        dir.path()
            .join("schema-snapshots")
            .join(format!("{ty}.v1.yaml")),
        &prior_body,
    )
    .expect("write the v1 snapshot");

    let current_body = current(&shipped);
    assert_ne!(
        prior_body, current_body,
        "the bump must actually reshape `{ty}` — a fixture that reshapes nothing proves nothing",
    );
    fs::write(&schema_path, &current_body).expect("write the current schema");

    bump_manifest(dir.path(), ty);
    frozen_pack::repin_manifest_hash(dir.path(), ty);
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

/// A real git repo carrying the `.jigc/config/` project layer the cascade expects, one initial
/// commit, and the committed managed docs `docs` names (path → bytes).
fn repo_with(docs: &[(&str, &str)]) -> TempDir {
    let dir = TempDir::new("repo");
    let root = dir.path();
    git(root, &["init", "-q"]);
    git(root, &["config", "user.email", "test@example.com"]);
    git(root, &["config", "user.name", "Test"]);
    fs::write(root.join("README.md"), "hello\n").expect("write file");
    fs::create_dir_all(root.join(".jigc").join("config")).expect("create project layer");
    for (rel, body) in docs {
        let path = root.join(rel);
        fs::create_dir_all(path.parent().expect("a parent dir")).expect("mk doc dir");
        fs::write(&path, body).expect("write the managed doc");
    }
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
    migrated[0].as_str().expect("a target path").to_string()
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
        "the migrated corpus must validate clean; stdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr),
    );
}

/// The doc's bytes on disk, read back from `repo`.
fn on_disk(repo: &Path, rel: &str) -> String {
    fs::read_to_string(repo.join(rel)).expect("read the doc")
}

// ---------------------------------------------------------------------------------------------
// The reshapes, as text edits over the shipped schema bodies.
// ---------------------------------------------------------------------------------------------

/// The shipped `roadmap` item block's second slot leaf.
const ROADMAP_DECOMPOSITION: &str = "        - { id: decomposition, slot: { hint: \"The increments, as prose: each increment's deliverable and the tasks that build it, one level.\" } }\n";

/// The shipped `roadmap` item block's first slot leaf — the anchor a companion field is
/// appended after.
const ROADMAP_PROVES: &str = "        - { id: proves, slot: { hint: \"What this milestone proves — the differentiator it converts from supported to demonstrated.\" } }\n";

/// The shipped `completion-record` item block's last field — the anchor a companion field or a
/// nested repeatable leaf is appended after.
const RECORD_EVIDENCE: &str = "        - { id: evidence, type: string }\n";

/// Append `added` after `anchor` (which must be present).
fn append_after(body: &str, anchor: &str, added: &str) -> String {
    let out = body.replacen(anchor, &format!("{anchor}{added}"), 1);
    assert_ne!(out, body, "the anchor line must be present to append after");
    out
}

/// **The companion classified change** every dropped-delta arm rides alongside: an added
/// *optional* item field with no value source, which classifies `AddedItemField` and folds to
/// **zero bytes** (an item without the bullet already conforms). It contributes nothing to the
/// migrated bytes and everything to the premise — a non-empty diff, which is precisely the state
/// the `Unclassified` residual never sees.
fn with_companion_field(body: &str, anchor: &str) -> String {
    append_after(
        body,
        anchor,
        "        - { id: owner, type: string, optional: true }\n",
    )
}

/// Drop the `decomposition` slot leaf from the shipped `roadmap` item block.
fn without_decomposition(body: &str) -> String {
    let out = body.replacen(ROADMAP_DECOMPOSITION, "", 1);
    assert_ne!(
        out, body,
        "roadmap.yaml must declare the `decomposition` slot"
    );
    out
}

/// Relax the shipped `roadmap` item block's `decomposition` slot to `optional: true`.
fn optional_decomposition(body: &str) -> String {
    let out = body.replacen(
        "- { id: decomposition, slot: { hint:",
        "- { id: decomposition, slot: { optional: true, hint:",
        1,
    );
    assert_ne!(
        out, body,
        "roadmap.yaml must declare the `decomposition` slot"
    );
    out
}

/// Append a **nested repeatable** `notes` leaf to `completion-record`'s finding block. `extra`
/// is appended inside the nested block, so the *reshaped* fixture differs from the plain one
/// inside the nested leaf and nowhere else.
fn with_nested_notes(body: &str, extra: &str) -> String {
    append_after(
        body,
        RECORD_EVIDENCE,
        &format!(
            "        - id: notes\n\
             \x20         repeatable:\n\
             \x20           id-from: title\n\
             \x20           block:\n\
             \x20             - {{ id: title, type: string }}\n\
             \x20             - {{ id: body, slot: {{ hint: \"The note.\" }} }}\n{extra}"
        ),
    )
}

// ---------------------------------------------------------------------------------------------
// The committed v1 corpora.
// ---------------------------------------------------------------------------------------------

/// A conformant, **v1-stamped** `roadmap` whose one milestone carries both declared slots.
const ROADMAP_V1_BOTH: &str = "\
---
schema-version: 1
---

# Roadmap

## Milestones

### First milestone  {#first-milestone}

#### Proves

The loop closes.

#### Decomposition

Three increments.
";

/// A conformant, **v1-stamped** `roadmap` whose milestone leaves `decomposition` **empty** —
/// the doc an `optional:` delta on that slot is actually about.
const ROADMAP_V1_EMPTY_DECOMPOSITION: &str = "\
---
schema-version: 1
---

# Roadmap

## Milestones

### First milestone  {#first-milestone}

#### Proves

The loop closes.

#### Decomposition
";

/// A conformant, **v1-stamped** `completion-record` with one finding item.
const RECORD_V1: &str = "\
---
verdict: green
owner-artifact: completions/artifacts/M1/VERDICT.md
schema-version: 1
---

# M1

## Findings

### A stray finding  {#a-stray-finding}

<!-- fields -->
- severity: advisory
- disposition: fixed
- evidence: the audit log
";

/// The seeded `completion-record` corpus: the record plus the owned artifact its header names.
fn record_corpus() -> TempDir {
    repo_with(&[
        ("completions/M1.md", RECORD_V1),
        ("completions/artifacts/M1/VERDICT.md", "the verdict\n"),
    ])
}

// ---------------------------------------------------------------------------------------------
// Arm 1 — a removed item slot refuses, and its message names a SLOT.
// ---------------------------------------------------------------------------------------------

/// **A removed item slot is refused with a schema-authoring route that names a slot.** The
/// recorded pick for a dropped leaf is *refuse, not strip* (`DECISIONS.md` → 2026-07-13 M42
/// Inc-5 T5), and it holds at the item locus for a sharper reason than at the simple one: what a
/// strip arm would destroy here is **authored prose**, not a field line.
///
/// The removal rides alongside an added optional item field, so the diff is non-empty and the
/// residual never fires — which is exactly where HEAD is silent. Red at HEAD: the run migrates
/// and restamps the doc while it still carries prose under a sub-label the schema has dropped.
#[test]
fn a_removed_item_slot_refuses_with_a_slot_naming_route() {
    let home = TempDir::new("home");
    let pack = pack_bumping(
        "pack",
        "roadmap",
        |shipped| shipped.to_string(),
        |shipped| with_companion_field(&without_decomposition(shipped), ROADMAP_PROVES),
    );
    let repo = repo_with(&[("docs/roadmap.md", ROADMAP_V1_BOTH)]);

    let (report, ok) = migrate_report(repo.path(), home.path(), pack.path());
    let finding = sole_blocked(&report, ok);
    assert_eq!(
        finding["code"], "migrate-corpus.removed-item-slot",
        "a dropped item slot names itself rather than riding the residual; finding:\n{finding:#}",
    );
    let message = finding["message"].as_str().expect("a message");
    assert!(
        message.contains("milestones.decomposition") && message.contains("slot"),
        "the message names the dropped SLOT; message: {message}",
    );
    assert!(
        !message.contains("field"),
        "the leaf that was dropped is a prose slot, not a field; message: {message}",
    );
    let route = finding["route"].as_str().expect("a route");
    assert!(
        route.contains("restore") && route.contains("milestones.decomposition"),
        "the repair is a schema-authoring one, naming the slot; route: {route}",
    );

    assert_eq!(
        on_disk(repo.path(), "docs/roadmap.md"),
        ROADMAP_V1_BOTH,
        "the refusal leaves the committed bytes — and the stamp — untouched",
    );
}

// ---------------------------------------------------------------------------------------------
// Arm 2 — a nested-repeatable delta names itself (the backstop kind, emitted explicitly).
// ---------------------------------------------------------------------------------------------

/// Drive one nested-repeatable bump of `completion-record` and assert the run refuses the doc at
/// the backstop's code with its bytes untouched. Each caller supplies the prior/current pair;
/// every one of them rides the companion `AddedItemField`, so the residual is unreachable and
/// HEAD migrates the doc silently.
fn assert_nested_delta_blocks(
    tag: &str,
    prior: impl FnOnce(&str) -> String,
    current: impl FnOnce(&str) -> String,
) {
    let home = TempDir::new("home");
    let pack = pack_bumping(tag, "completion-record", prior, current);
    let repo = record_corpus();

    let (report, ok) = migrate_report(repo.path(), home.path(), pack.path());
    let finding = sole_blocked(&report, ok);
    assert_eq!(
        finding["code"], "migrate-corpus.unclassified-change",
        "a nested-repeatable delta names itself instead of vanishing into a non-empty diff; \
         finding:\n{finding:#}",
    );
    assert_eq!(
        on_disk(repo.path(), "completions/M1.md"),
        RECORD_V1,
        "the refusal leaves the committed bytes — and the stamp — untouched",
    );
}

/// **An added nested repeatable leaf blocks.** No kind is built for it (T1-2's
/// `AddedNestedRepeatable` stays deferred), so the whole change is that the delta *names itself*
/// — the explicit backstop kind — instead of being dropped by the item filter.
#[test]
fn an_added_nested_repeatable_leaf_blocks_the_run() {
    assert_nested_delta_blocks(
        "nested-added",
        |shipped| shipped.to_string(),
        |shipped| with_companion_field(&with_nested_notes(shipped, ""), RECORD_EVIDENCE),
    );
}

/// **A removed nested repeatable leaf blocks** — the removed pass reads the leaf kind the added
/// pass now reads, so a nested leaf cannot disappear from a doctype silently either.
#[test]
fn a_removed_nested_repeatable_leaf_blocks_the_run() {
    assert_nested_delta_blocks(
        "nested-removed",
        |shipped| with_nested_notes(shipped, ""),
        |shipped| with_companion_field(shipped, RECORD_EVIDENCE),
    );
}

/// **A reshaped nested repeatable leaf blocks** — the leaf is present on both sides and its own
/// block moved inside the conformance-relevant projection (its inner item block gains a field),
/// which is a change no committed nested item satisfies by accident.
#[test]
fn a_reshaped_nested_repeatable_leaf_blocks_the_run() {
    assert_nested_delta_blocks(
        "nested-reshaped",
        |shipped| with_nested_notes(shipped, ""),
        |shipped| {
            with_companion_field(
                &with_nested_notes(shipped, "              - { id: weight, type: string }\n"),
                RECORD_EVIDENCE,
            )
        },
    );
}

// ---------------------------------------------------------------------------------------------
// Arm 3 — an item slot's `optional:` delta joins the shared rule.
// ---------------------------------------------------------------------------------------------

/// **`optional: false → true` on an item slot migrates clean, as the declared byte no-op.** A
/// relaxation admits every doc the strict rule admitted, so nothing can have become
/// non-conformant and the stamp is the migration's only byte delta —
/// `doctype-authoring.md`'s `OptionalRelaxed` row claims exactly this "at both loci … and at the
/// slot level", and the item filter is what made the claim false.
///
/// Red at HEAD: the delta is dropped, nothing else classifies, and the residual refuses the one
/// direction that cannot break a single doc — `migrate-corpus.unclassified-change`, routed at
/// the jigc source tree.
#[test]
fn an_item_slot_relaxation_migrates_as_a_byte_no_op() {
    let home = TempDir::new("home");
    let pack = pack_bumping(
        "relaxed",
        "roadmap",
        |shipped| shipped.to_string(),
        optional_decomposition,
    );
    let repo = repo_with(&[("docs/roadmap.md", ROADMAP_V1_EMPTY_DECOMPOSITION)]);

    let (report, ok) = migrate_report(repo.path(), home.path(), pack.path());
    assert_eq!(sole_migrated(&report, ok), "docs/roadmap.md");
    assert_eq!(
        on_disk(repo.path(), "docs/roadmap.md"),
        ROADMAP_V1_EMPTY_DECOMPOSITION.replace("schema-version: 1", "schema-version: 2"),
        "a relaxation is a byte no-op: the stamp is the migration's only delta",
    );
    assert_validates_clean(repo.path(), home.path(), pack.path());
}

/// **`optional: true → false` on an item slot routes the author.** A slot carries no value
/// source, so a tightening always makes the leaf author-required: a doc that legitimately left
/// it empty now breaks its own conformance gate, and the refusal is the doc-authorable
/// `migrate-corpus.prose-needed` — never a build instruction.
///
/// Red at HEAD: the delta is dropped and the residual answers instead, with
/// `unclassified-change` and a route naming `crates/engine` — a file no adopter can edit, over a
/// doc whose own prose is the actual repair.
#[test]
fn an_item_slot_tightening_routes_the_author() {
    let home = TempDir::new("home");
    let pack = pack_bumping("tightened", "roadmap", optional_decomposition, |shipped| {
        shipped.to_string()
    });
    let repo = repo_with(&[("docs/roadmap.md", ROADMAP_V1_EMPTY_DECOMPOSITION)]);

    let (report, ok) = migrate_report(repo.path(), home.path(), pack.path());
    let finding = sole_blocked(&report, ok);
    assert_eq!(
        finding["code"], "migrate-corpus.prose-needed",
        "a tightened item slot is adjudicated by the per-doc gate, so its route is the \
         doc-authorable one; finding:\n{finding:#}",
    );
    let route = finding["route"].as_str().expect("a route");
    assert!(
        route.contains("docs/roadmap.md") && !route.contains("crates/engine"),
        "the route names the doc whose prose clears it, not a file no adopter can edit; \
         route: {route}",
    );
    assert_eq!(
        on_disk(repo.path(), "docs/roadmap.md"),
        ROADMAP_V1_EMPTY_DECOMPOSITION,
        "the refused doc rolls back byte-identical, its stamp unmoved",
    );
}
