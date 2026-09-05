//! Acceptance — **a repeatable item block can gain prose** (M49 Increment 4, T1;
//! `design/corpus-migration.md` → The classifier's holes · The deterministic transform ·
//! Prose routing (Framing A); `completions/artifacts/M49/settle-record.md` → D3(B), D10).
//!
//! Before this increment there was **no legal path from a scalar item field to item prose at
//! any arity**: the item-block diff loop classified `Field` leaves only, so an added slot leaf
//! diffed to the empty-diff residual and `jigc migrate-corpus` refused the doc at
//! `migrate-corpus.unclassified-change`, with a route naming
//! `crates/engine/src/transform.rs` — **a file no adopter can edit**. That is not a bound on
//! an adopter's lock-in cost; it is the absence of one. `schema_diff.rs`'s own doc-comment
//! blessed the residual for slots while the locked record had already settled the principle
//! the other way for `RemovedField` (`corpus-migration.md`:188 — *"the kind must exist rather
//! than ride the backstop"*).
//!
//! Every arm drives the **shipped binary** over a manufactured `JIGC_PACK_DIR` methodology
//! pack — the real freeze machinery, with the reshaped doctype's `schema-hash` re-pinned
//! through the production loader (`support::frozen_pack`), which is exactly the act a pack
//! author performs — and asserts the **bytes on disk** the migration left, never a
//! reconstruction of them.
//!
//! **The axis is arity × requiredness**, and the cells are the ones a real bump lands in:
//!
//! - **0 → 1, optional** (`completion-record` + a `detail` slot — D10's settled shape, and the
//!   one this suite's cells now manufacture by **stripping** the slot the bump shipped): the
//!   schema-version stamp is the migration's **only** byte delta, because a lone slot renders
//!   *bare*, under no sub-heading, so a slot-less item already carries the v2 bytes.
//! - **1 → 2, optional** (`roadmap` + an optional second slot): the committed bare item prose
//!   rides **verbatim** under the v1 slot's `#### <Leaf-Title>` and the new sub-label mints
//!   empty at its schema-ordered offset — asserted as exact bytes *and* as
//!   `render(parse(x)) == x` over those emitted bytes.
//! - **1 → 2, required** (`roadmap` + `decomposition`, the **shipped** shape): the fold mints
//!   the empty required leaf and the per-doc conformance gate refuses the doc, so the route is
//!   the doc-authorable `migrate-corpus.prose-needed` — never `unclassified-change`, never a
//!   build instruction — with the bytes rolled back and the stamp unmoved.
//! - **2 → 3**: the new sub-label is minted with every existing sub-label byte-untouched. This
//!   is the cell that pins the idempotency guard's key: an item at this arity already carries
//!   *some* declared sub-label while still needing the added one, so a guard keyed on "any
//!   sub-label" skips exactly the items the change exists to reshape.
//! - **the re-run**, over one doc holding an item that already carries the added sub-label
//!   **beside** one that does not — a re-run after a refused commit, or a hand-authored entry.
//!   The first is left byte-identical, which is why the guard is per item: a whole-change
//!   filter in `per_doc_changes` cannot express it.

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

use crate::support::frozen_pack;

// ---------------------------------------------------------------------------------------------
// Fixture plumbing (the shipped `migrate_corpus_halt_causes.rs` mechanism).
// ---------------------------------------------------------------------------------------------

/// A throwaway directory that removes itself on drop.
struct TempDir(PathBuf);

impl TempDir {
    fn new(tag: &str) -> Self {
        let mut path = std::env::temp_dir();
        path.push(format!(
            "jigc-itemslot-{tag}-{}-{:?}",
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

/// Pin `ty`'s manifest `schema-version` to `to` in the pack copy at `pack`, **whatever it
/// shipped at**. The **hash** is re-pinned separately by [`frozen_pack::repin_manifest_hash`]
/// whenever the *current* schema is the half that moved; a version pin alone is what an author
/// does when only the prior snapshot is being introduced.
///
/// It reads the shipped version rather than asserting one, so a doctype this suite reshapes can
/// be bumped for real in the pack (M49 Inc-9 shipped `completion-record` at **2**) without every
/// cell below going red on an incidental number — the fixture pairs the cells manufacture are
/// what the axis is made of, not the version they are labelled with.
fn pin_manifest_version(pack: &Path, ty: &str, to: u32) {
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
    pinned.push_str(&to.to_string());
    pinned.push_str(&manifest[end..]);
    fs::write(&manifest_path, pinned).expect("write the pinned manifest");
}

/// A methodology-pack copy whose doctype `ty` is bumped `1 → 2`:
///
/// - `schema-snapshots/<ty>.v1.yaml` — the prior shape, produced by `prior` from the shipped
///   body (the state every committed instance conforms to);
/// - `schemas/<ty>.yaml` — the current shape, produced by `current` from the shipped body;
/// - `config/schema-manifest.yaml` — the version bumped and, since the current shape may have
///   moved, the `schema-hash` **re-pinned through the production loader**.
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

    pin_manifest_version(dir.path(), ty, 2);
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

/// `render(parse(x)) == x` over the bytes the **binary wrote**, under the reshaped pack's own
/// `ty` schema, loaded through the production loader.
///
/// The assertion is on the emitted artifact, never on a reconstruction of it: the file the
/// migration left on disk is fed straight back through the parser and the canonical writer.
fn assert_byte_stable(pack: &Path, ty: &str, bytes: &str) {
    let source = cli::pack::FilesystemPack::new(pack.to_path_buf());
    let schema_bytes = fs::read(pack.join("schemas").join(format!("{ty}.yaml")))
        .expect("read the reshaped schema");
    let schema =
        cli::pack::load_pack_schema(&source, &schema_bytes).expect("the reshaped schema loads");
    let instance = engine::write::instance_from_source(&schema, bytes)
        .expect("the migrated bytes re-parse under the new schema");
    assert_eq!(
        engine::write::render(&schema, &instance),
        bytes,
        "the migrated bytes must round-trip byte-identical under the new schema"
    );
}

// ---------------------------------------------------------------------------------------------
// The reshapes, as text edits over the shipped schema bodies.
// ---------------------------------------------------------------------------------------------

/// The shipped `roadmap` item block's second slot leaf — the line the 1→2 fixtures remove to
/// manufacture a single-slot prior shape.
const ROADMAP_DECOMPOSITION: &str = "        - { id: decomposition, slot: { hint: \"The increments, as prose: each increment's deliverable and the tasks that build it, one level.\" } }\n";

/// The shipped `completion-record` item block's **one** slot leaf (M49 Inc-9 T1 — the `detail`
/// prose leaf that made the 0→1 arity real). The cells below strip it to manufacture the
/// slot-less prior shape, and append after it to manufacture the higher arities.
const RECORD_DETAIL: &str = "        - { id: detail, slot: { optional: true, hint: \"The finding in prose — what it is, how it was reproduced, and what its disposition rests on. Leave it empty when the one-line `evidence` says everything.\" } }\n";

/// Drop the shipped `detail` slot leaf: `completion-record`'s finding block becomes the
/// **slot-less** shape every record committed before that bump carries.
fn without_detail(body: &str) -> String {
    let out = body.replacen(RECORD_DETAIL, "", 1);
    assert_ne!(
        out, body,
        "completion-record.yaml must declare the `detail` slot"
    );
    out
}

/// Drop the `decomposition` slot leaf: the shipped 2-slot `roadmap` item block becomes the
/// **single-slot** shape a committed v1 instance carries as bare prose.
fn without_decomposition(body: &str) -> String {
    let out = body.replacen(ROADMAP_DECOMPOSITION, "", 1);
    assert_ne!(
        out, body,
        "roadmap.yaml must declare the `decomposition` slot"
    );
    out
}

/// Append an **optional** `detail` slot leaf to the item block, after `anchor`.
fn with_optional_detail(body: &str, anchor: &str) -> String {
    let out = body.replacen(
        anchor,
        &format!(
            "{anchor}        - {{ id: detail, slot: {{ optional: true, hint: \"The entry in prose.\" }} }}\n"
        ),
        1,
    );
    assert_ne!(out, body, "the anchor line must be present to append after");
    out
}

// ---------------------------------------------------------------------------------------------
// Cell 1 — 0 → 1, optional.
// ---------------------------------------------------------------------------------------------

/// A conformant, **v1-stamped** `completion-record` whose one finding item is **slot-less** —
/// a heading, its `<!-- fields -->` group, and nothing else.
// The committed record is seeded at `completions/m1.md` — the **slug** the mint
// derives from the title `M1`, never the title itself. A hand-written `M1.md` is an
// identity no jigc operation produces, and since M50 Inc 2 / T1 the store sweep says
// so (`schema-conformance.unadopted-instance`), which is what a fixture naming it that
// way was quietly relying on not happening.
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

/// **0 → 1, optional — the stamp is the only byte delta.** `completion-record`'s finding block
/// gains an optional `detail` slot (D10's settled shape, which is *why* this arity had to be
/// reachable: converting the scalar `evidence` would be a `RemovedField`, refused by design).
///
/// A lone declared slot renders **bare**, under no sub-heading, and an item whose one slot is
/// empty renders identically to a slot-less one — so the committed bytes already *are* the v2
/// bytes and the fold writes none. Red at HEAD: `migrate-corpus.unclassified-change`, exit 1.
#[test]
fn zero_to_one_optional_migrates_with_the_stamp_as_its_only_byte_delta() {
    let home = TempDir::new("home");
    let pack = pack_bumping("pack", "completion-record", without_detail, |shipped| {
        shipped.to_string()
    });
    let repo = repo_with(&[
        ("completions/m1.md", RECORD_V1),
        ("completions/artifacts/M1/VERDICT.md", "the verdict\n"),
    ]);

    let (report, ok) = migrate_report(repo.path(), home.path(), pack.path());
    assert_eq!(
        sole_migrated(&report, ok),
        "completions/m1.md",
        "the record migrates; report:\n{report:#}"
    );

    let after = fs::read_to_string(repo.path().join("completions").join("m1.md"))
        .expect("read the migrated record");
    assert_eq!(
        after,
        RECORD_V1.replace("schema-version: 1", "schema-version: 2"),
        "the stamp is the migration's only byte delta at the 0→1 arity"
    );
    assert_byte_stable(pack.path(), "completion-record", &after);
    assert_validates_clean(repo.path(), home.path(), pack.path());
}

// ---------------------------------------------------------------------------------------------
// Cells 2 and 3 — 1 → 2, optional and required.
// ---------------------------------------------------------------------------------------------

/// A conformant, **v1-stamped** `roadmap` whose two milestone items each carry their prose as
/// the **bare body** of the single declared slot — the shape every committed instance of a
/// one-slot item block is in.
const ROADMAP_V1_BARE: &str = "\
---
schema-version: 1
---

# Roadmap

## Milestones

### First milestone  {#first-milestone}

The loop closes.

### Second milestone  {#second-milestone}

The fan-out joins.
";

/// **1 → 2, optional — the committed prose rides verbatim under the v1 leaf's sub-label.**
/// `roadmap`'s item block gains an optional second slot, so both leaves now render under
/// `#### <Leaf-Title>` sub-headings: the committed bare prose is re-keyed under the leaf that
/// owned it, and the new one mints **empty** at its schema-ordered offset.
///
/// The bytes are asserted exactly, and then fed back through the parser and the canonical
/// writer — `render(parse(x)) == x` over the artifact the binary actually emitted.
#[test]
fn one_to_two_optional_carries_the_committed_prose_under_its_sub_label() {
    let home = TempDir::new("home");
    let pack = pack_bumping("pack", "roadmap", without_decomposition, |shipped| {
        with_optional_detail(
            &without_decomposition(shipped),
            "        - { id: proves, slot: { hint: \"What this milestone proves — the differentiator it converts from supported to demonstrated.\" } }\n",
        )
    });
    let repo = repo_with(&[("docs/roadmap.md", ROADMAP_V1_BARE)]);

    let (report, ok) = migrate_report(repo.path(), home.path(), pack.path());
    assert_eq!(sole_migrated(&report, ok), "docs/roadmap.md");

    let after =
        fs::read_to_string(repo.path().join("docs").join("roadmap.md")).expect("read the roadmap");
    assert_eq!(
        after,
        "\
---
schema-version: 2
---

# Roadmap

## Milestones

### First milestone  {#first-milestone}

#### Proves

The loop closes.

#### Detail

### Second milestone  {#second-milestone}

#### Proves

The fan-out joins.

#### Detail
",
        "each item's committed prose rides verbatim under `#### Proves`, and `#### Detail` \
         mints empty at its schema-ordered offset"
    );
    assert_byte_stable(pack.path(), "roadmap", &after);
    assert_validates_clean(repo.path(), home.path(), pack.path());
}

/// **1 → 2, required — the doc is routed at the author, not at the jigc source tree.** This is
/// the **shipped** `roadmap` shape (`proves` + a required `decomposition`) reached from a
/// single-slot prior, so it is the bump a real adopter meets.
///
/// One kind carries both requirednesses: the relabel is the same byte work either way, and the
/// **per-doc conformance gate** adjudicates the empty required leaf. So the refusal is
/// `migrate-corpus.prose-needed` — followable, and the one halt cause an author can clear from
/// the doc — never `unclassified-change` and never a build instruction. Red at HEAD: the
/// mutual dead end (`validate` says *migrate*, `migrate-corpus` says *edit the engine*).
#[test]
fn one_to_two_required_routes_at_the_author_never_at_the_source_tree() {
    let home = TempDir::new("home");
    let pack = pack_bumping("pack", "roadmap", without_decomposition, |shipped| {
        shipped.to_string()
    });
    let repo = repo_with(&[("docs/roadmap.md", ROADMAP_V1_BARE)]);

    let (report, ok) = migrate_report(repo.path(), home.path(), pack.path());
    let finding = sole_blocked(&report, ok);
    assert_eq!(
        finding["code"], "migrate-corpus.prose-needed",
        "a required added item slot is adjudicated by the gate, so its route is the \
         doc-authorable one; finding:\n{finding:#}",
    );
    let route = finding["route"].as_str().expect("a route");
    assert!(
        !route.contains("crates/engine"),
        "the route must not name a file no adopter can edit; route: {route}",
    );
    assert!(
        route.contains("author the new required prose"),
        "the route is the Framing-A handoff; route: {route}",
    );
    let message = finding["message"].as_str().expect("a message");
    assert!(
        message.contains("required slot `decomposition` in item `milestones/first-milestone`"),
        "the message relays the gate's own findings, which name the minted leaf; \
         message: {message}",
    );

    let after =
        fs::read_to_string(repo.path().join("docs").join("roadmap.md")).expect("read the roadmap");
    assert_eq!(
        after, ROADMAP_V1_BARE,
        "a refused doc rolls back byte-identical — the stamp never moves"
    );
}

// ---------------------------------------------------------------------------------------------
// Cell 4 — 2 → 3.
// ---------------------------------------------------------------------------------------------

/// A conformant, **v1-stamped** `roadmap` in the shipped **two-slot** shape — every item
/// already carries both declared sub-labels.
const ROADMAP_V1_TWO_SLOTS: &str = "\
---
schema-version: 1
---

# Roadmap

## Milestones

### First milestone  {#first-milestone}

#### Proves

The loop closes.

#### Decomposition

Two increments.
";

/// **2 → 3 — the new sub-label is minted and every existing one is byte-untouched.**
///
/// This cell is what keys the per-item idempotency guard on the **added** leaf rather than on
/// "any declared sub-label": at this arity every item already carries sub-labels while still
/// needing the new one, so an any-sub-label guard skips exactly the items the change exists to
/// reshape — and the fold's output then fails to parse under the new schema.
#[test]
fn two_to_three_mints_the_new_sub_label_with_the_existing_ones_untouched() {
    let home = TempDir::new("home");
    let pack = pack_bumping(
        "pack",
        "roadmap",
        |shipped| shipped.to_string(),
        |shipped| with_optional_detail(shipped, ROADMAP_DECOMPOSITION),
    );
    let repo = repo_with(&[("docs/roadmap.md", ROADMAP_V1_TWO_SLOTS)]);

    let (report, ok) = migrate_report(repo.path(), home.path(), pack.path());
    assert_eq!(sole_migrated(&report, ok), "docs/roadmap.md");

    let after =
        fs::read_to_string(repo.path().join("docs").join("roadmap.md")).expect("read the roadmap");
    assert_eq!(
        after,
        ROADMAP_V1_TWO_SLOTS.replace("schema-version: 1", "schema-version: 2") + "\n#### Detail\n",
        "the existing sub-labels and their prose survive byte-for-byte; only the new leaf's \
         empty sub-label is added, at its schema-ordered offset"
    );
    assert_byte_stable(pack.path(), "roadmap", &after);
    assert_validates_clean(repo.path(), home.path(), pack.path());
}

// ---------------------------------------------------------------------------------------------
// Cell 5 — the re-run, over a doc holding both item states.
// ---------------------------------------------------------------------------------------------

/// A **v1-stamped** `roadmap` whose first item already renders the v2 sub-labels (an earlier
/// run whose commit was refused, or a hand-authored entry) while its second is still bare.
const ROADMAP_HALF_RESHAPED: &str = "\
---
schema-version: 1
---

# Roadmap

## Milestones

### First milestone  {#first-milestone}

#### Proves

The loop closes.

#### Detail

Already reshaped by an earlier run.

### Second milestone  {#second-milestone}

The fan-out joins.
";

/// **The re-run guard is per item.** One doc can hold an item that already carries the added
/// sub-label beside one that does not, so the guard cannot live in `per_doc_changes`' whole-
/// change filter: the already-reshaped item is left **byte-identical** (its authored `detail`
/// prose is not re-keyed, re-nested, or lost) while the bare one is reshaped.
#[test]
fn a_re_run_leaves_an_already_reshaped_item_byte_identical() {
    let home = TempDir::new("home");
    let pack = pack_bumping("pack", "roadmap", without_decomposition, |shipped| {
        with_optional_detail(
            &without_decomposition(shipped),
            "        - { id: proves, slot: { hint: \"What this milestone proves — the differentiator it converts from supported to demonstrated.\" } }\n",
        )
    });
    let repo = repo_with(&[("docs/roadmap.md", ROADMAP_HALF_RESHAPED)]);

    let (report, ok) = migrate_report(repo.path(), home.path(), pack.path());
    assert_eq!(sole_migrated(&report, ok), "docs/roadmap.md");

    let after =
        fs::read_to_string(repo.path().join("docs").join("roadmap.md")).expect("read the roadmap");
    let untouched = ROADMAP_HALF_RESHAPED
        .split("### Second milestone")
        .next()
        .expect("the already-reshaped item's bytes")
        .replace("schema-version: 1", "schema-version: 2");
    assert!(
        after.starts_with(&untouched),
        "the already-reshaped item keeps its bytes verbatim; got:\n{after}"
    );
    assert!(
        after.ends_with("### Second milestone  {#second-milestone}\n\n#### Proves\n\nThe fan-out joins.\n\n#### Detail\n"),
        "the still-bare item is reshaped in the same run; got:\n{after}"
    );
    assert_byte_stable(pack.path(), "roadmap", &after);
    assert_validates_clean(repo.path(), home.path(), pack.path());
}

// ---------------------------------------------------------------------------------------------
// Cells 6–8 — the **field-bearing** item block, the shape that can hold unmodelled bytes.
//
// Every cell above rewrites bytes over `roadmap`, whose item block declares **no fields** — so
// no `<!-- fields -->` group exists in it, and the one committed shape that can carry prose the
// parse does not model was never migrated at an arity that rewrites anything (`completion-record`
// appears only at 0→1, where the fold writes zero bytes by construction). These cells close that
// hole on the axis the kind claims: **arity × requiredness × whether the item block bears
// fields**.
// ---------------------------------------------------------------------------------------------

/// Append a second **optional** `extra` slot leaf after the shipped `detail` one — the 1→2
/// arity over a field-bearing block, and the base the 2→3 cell appends a third onto.
fn with_optional_extra(body: &str) -> String {
    let anchor = RECORD_DETAIL;
    let out = body.replacen(
        anchor,
        &format!(
            "{anchor}        - {{ id: extra, slot: {{ optional: true, hint: \"The rest.\" }} }}\n"
        ),
        1,
    );
    assert_ne!(
        out, body,
        "the `detail` leaf must be present to append after"
    );
    out
}

/// A conformant, **v1-stamped** `completion-record` whose finding item carries its prose as the
/// bare body of the single declared slot **and** a trailing `<!-- fields -->` group — jigc's own
/// canonical order, slots-then-fields, for a field-bearing item block.
const RECORD_V1_PROSE: &str = "\
---
verdict: green
owner-artifact: completions/artifacts/M1/VERDICT.md
schema-version: 1
---

# M1

## Findings

### A stray finding  {#a-stray-finding}

The finding, in prose.

<!-- fields -->
- severity: advisory
- disposition: fixed
- evidence: the audit log
";

/// The same committed doc with **one hand-appended sentence after the field group** — where
/// "append a sentence to the end of this entry" lands, because the canonical order puts the
/// fields last. The parse does not model it: `doc show` renders the slot without it and
/// `jigc validate` reports nothing about it.
const RECORD_V1_TRAILING_PROSE: &str = "\
---
verdict: green
owner-artifact: completions/artifacts/M1/VERDICT.md
schema-version: 1
---

# M1

## Findings

### A stray finding  {#a-stray-finding}

The finding, in prose.

<!-- fields -->
- severity: advisory
- disposition: fixed
- evidence: the audit log

THE COMMITTED PROSE THAT MUST SURVIVE.
";

/// The 1→2 reshape over `completion-record`'s **field-bearing** finding block: prior = the
/// shipped block (whose one slot leaf is `detail`); current = plus an optional `extra` slot.
fn record_one_to_two(tag: &str) -> TempDir {
    pack_bumping(
        tag,
        "completion-record",
        |shipped| shipped.to_string(),
        with_optional_extra,
    )
}

/// **Cell 6 — 1 → 2 over a field-bearing item block.** The committed bare prose rides under
/// `#### Detail`, the new leaf mints empty at its schema-ordered offset, and the item's
/// `<!-- fields -->` group survives byte-for-byte **after** both sub-labels: the canonical
/// slots-then-fields order the writer emits.
#[test]
fn one_to_two_over_a_field_bearing_item_keeps_the_field_group() {
    let home = TempDir::new("home");
    let pack = record_one_to_two("pack");
    let repo = repo_with(&[
        ("completions/m1.md", RECORD_V1_PROSE),
        ("completions/artifacts/M1/VERDICT.md", "the verdict\n"),
    ]);

    let (report, ok) = migrate_report(repo.path(), home.path(), pack.path());
    assert_eq!(sole_migrated(&report, ok), "completions/m1.md");

    let after = fs::read_to_string(repo.path().join("completions").join("m1.md"))
        .expect("read the migrated record");
    assert_eq!(
        after,
        "\
---
verdict: green
owner-artifact: completions/artifacts/M1/VERDICT.md
schema-version: 2
---

# M1

## Findings

### A stray finding  {#a-stray-finding}

#### Detail

The finding, in prose.

#### Extra

<!-- fields -->
- severity: advisory
- disposition: fixed
- evidence: the audit log
",
        "the committed prose is re-keyed under its leaf, the new leaf mints empty, and the \
         field group survives verbatim"
    );
    assert_byte_stable(pack.path(), "completion-record", &after);
    assert_validates_clean(repo.path(), home.path(), pack.path());
}

/// **Cell 7 — the guard: an item carrying bytes the parse does not model is REFUSED, never
/// rewritten.** The reshape is a whole-item re-render, so every committed byte the parse does
/// not model would be silently destroyed by it — and prose after the `<!-- fields -->` group is
/// reachable by hand-editing, invisible to `doc show` and to `jigc validate`.
///
/// **No-data-loss** is a declared property of this pair — `transform.rs` invokes it by name to
/// refuse `RemovedItemSlot` for exactly these bytes — so the fold checks its own pre-image
/// first: re-render the item under the OLD template and compare it to the committed region.
/// Unequal means the parse does not model the region, and the doc is blocked with a located
/// cause and its bytes rolled back untouched.
#[test]
fn an_item_carrying_unmodelled_bytes_is_refused_never_rewritten() {
    let home = TempDir::new("home");
    let pack = record_one_to_two("pack");
    let repo = repo_with(&[
        ("completions/m1.md", RECORD_V1_TRAILING_PROSE),
        ("completions/artifacts/M1/VERDICT.md", "the verdict\n"),
    ]);

    let (report, ok) = migrate_report(repo.path(), home.path(), pack.path());
    let finding = sole_blocked(&report, ok);
    assert_eq!(
        finding["code"], "migrate-corpus.item-unmodelled-content",
        "the refusal names its own cause; finding:\n{finding:#}",
    );
    let message = finding["message"].as_str().expect("a message");
    assert!(
        message.contains("findings/a-stray-finding"),
        "the cause locates the item it refused; message: {message}",
    );
    let route = finding["route"].as_str().expect("a route");
    assert!(
        !route.contains("crates/engine"),
        "the repair is in the doc, not in a file no adopter can edit; route: {route}",
    );

    let after = fs::read_to_string(repo.path().join("completions").join("m1.md"))
        .expect("read the refused record");
    assert_eq!(
        after, RECORD_V1_TRAILING_PROSE,
        "a refused doc rolls back byte-identical — the committed prose survives and the stamp \
         never moves"
    );
}

/// **Cell 8 — the guard holds at the next arity too.** 2→3 over the same field-bearing block:
/// the committed item is already multi-slot, and the trailing prose after its field group is
/// still unmodelled — so the same refusal fires. The arity axis is iterated, not sampled.
#[test]
fn the_unmodelled_bytes_guard_holds_at_two_to_three() {
    let home = TempDir::new("home");
    let pack = pack_bumping(
        "pack",
        "completion-record",
        with_optional_extra,
        |shipped| {
            with_optional_extra(shipped).replacen(
                "        - { id: extra, slot: { optional: true, hint: \"The rest.\" } }\n",
                "        - { id: extra, slot: { optional: true, hint: \"The rest.\" } }\n        - { id: more, slot: { optional: true, hint: \"Still more.\" } }\n",
                1,
            )
        },
    );
    let record = "\
---
verdict: green
owner-artifact: completions/artifacts/M1/VERDICT.md
schema-version: 1
---

# M1

## Findings

### A stray finding  {#a-stray-finding}

#### Detail

The finding, in prose.

#### Extra

The rest of it.

<!-- fields -->
- severity: advisory
- disposition: fixed
- evidence: the audit log

THE COMMITTED PROSE THAT MUST SURVIVE.
";
    let repo = repo_with(&[
        ("completions/m1.md", record),
        ("completions/artifacts/M1/VERDICT.md", "the verdict\n"),
    ]);

    let (report, ok) = migrate_report(repo.path(), home.path(), pack.path());
    let finding = sole_blocked(&report, ok);
    assert_eq!(
        finding["code"], "migrate-corpus.item-unmodelled-content",
        "the same guard adjudicates the 2→3 arity; finding:\n{finding:#}",
    );

    let after = fs::read_to_string(repo.path().join("completions").join("m1.md"))
        .expect("read the refused record");
    assert_eq!(
        after, record,
        "a refused doc rolls back byte-identical at every arity"
    );
}
