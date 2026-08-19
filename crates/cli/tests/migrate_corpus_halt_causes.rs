//! Real-binary regression — **a halted doc is told what actually halted it** (M46 Inc-4 T3;
//! `design/command-output-contract.md` → the `migrate-corpus.*` sub-table;
//! `design/corpus-migration.md` → Prose routing).
//!
//! The per-doc transaction collapsed **three** distinct causes into one `None` — the transform
//! driver's refusal, a parse failure under the new schema, and the conformance gate's own
//! findings — and the CLI rendered every one of them as `migrate-corpus.prose-needed`:
//!
//! > `<path>`'s migration **mints a new required prose slot** … author the new required prose,
//! > then re-run
//!
//! Driven at HEAD, all three cells below printed that byte-identically. Two of them are lies: a
//! doc whose *pre-existing* slot is empty had **nothing minted**, and an uncovered enum-member
//! rename needs an **authored old→new map** in the pack tooling — no prose an author writes into
//! that doc, and no number of re-runs, changes either outcome. The route was a **permanent dead
//! end**, followed exactly — the same class Increment 4's fold-side fix closes.
//!
//! So the halt now carries its cause ([`HaltReason`]), and the CLI renders it through an
//! **exhaustive** match:
//!
//! - a **gate** halt keeps the contract-pinned `migrate-corpus.prose-needed` waypoint and its
//!   route, and **relays the gate's own findings** as its message rather than asserting a mint;
//! - every **non-gate** halt takes `migrate-corpus.fold-refused`, whose route is a
//!   **schema-authoring** repair — the treatment `unclassified-change` / `narrowed-cardinality` /
//!   `removed-field` already get pre-fold, on the file's own recorded reason that the fold's
//!   halt route *"would be a lie"*.

use engine::transform::{HaltReason, TransformError};
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

// ---------------------------------------------------------------------------------------------
// The variant axis, read from the code side.
// ---------------------------------------------------------------------------------------------

/// How this suite proves one [`HaltReason`] variant. The match is **exhaustive on purpose**: a
/// fourth cause cannot be added without deciding what it says and where that is proven — the
/// same compile-time registry the CLI's rendering match is.
///
/// Two variants are reached by a **dev-pack fixture through the real binary** and are proven
/// here. The third is not reachable from any shipped doctype pair — a fold whose output does not
/// parse under the new schema needs a hand-built change list — so it is proven at the **engine
/// seam** instead, and this suite says so rather than implying a binary cell exists for it.
fn proof_of(reason: &HaltReason) -> &'static str {
    match reason {
        HaltReason::Gate(_) => {
            "this suite: `a_pre_existing_empty_slot_is_never_described_as_a_mint` (nothing minted) \
             + `a_genuinely_minted_required_slot_keeps_the_framing_a_waypoint` (a real mint)"
        }
        HaltReason::Transform(_) => {
            "this suite: `an_uncovered_enum_rename_routes_at_the_authored_map`"
        }
        HaltReason::Parse(_) => {
            "the engine seam: `engine::transform::tests::\
             a_fold_whose_output_does_not_parse_under_the_new_schema_halts_as_a_parse_failure` — \
             no dev-pack doctype pair reaches it through the binary"
        }
    }
}

/// **Every halt cause is proven somewhere, and the somewhere is named.** One value per variant,
/// each carrying a non-empty, distinct proof — so a new cause added to the engine either lands a
/// cell here or declares its engine-seam proof, and cannot arrive silently.
#[test]
fn every_halt_cause_names_where_it_is_proven() {
    let causes = [
        HaltReason::Gate(Vec::new()),
        HaltReason::Transform(TransformError::Unclassified),
        HaltReason::Parse(Vec::new()),
    ];
    let proofs: Vec<&str> = causes.iter().map(proof_of).collect();
    for (cause, proof) in causes.iter().zip(&proofs) {
        assert!(
            !proof.is_empty(),
            "the halt cause {cause:?} names no proof — give it a cell here, or name the engine \
             test that proves it",
        );
    }
    let mut distinct = proofs.clone();
    distinct.sort_unstable();
    distinct.dedup();
    assert_eq!(
        distinct.len(),
        proofs.len(),
        "two halt causes claim the same proof — one of them is unproven; proofs: {proofs:?}",
    );
}

// ---------------------------------------------------------------------------------------------
// Fixture plumbing (the shipped `migrate_corpus_set_fields.rs` mechanism).
// ---------------------------------------------------------------------------------------------

/// A throwaway directory that removes itself on drop.
struct TempDir(PathBuf);

impl TempDir {
    fn new(tag: &str) -> Self {
        let mut path = std::env::temp_dir();
        path.push(format!(
            "jigc-haltcause-{tag}-{}-{:?}",
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

/// The embedded dev pack tree on disk — the faithful source a mutated copy mirrors.
fn embedded_pack_tree() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("pack")
}

/// Recursively copy `src` into `dst` (both directories), creating `dst`.
fn copy_tree(src: &Path, dst: &Path) {
    fs::create_dir_all(dst).expect("create copy target dir");
    for entry in fs::read_dir(src).expect("read pack dir") {
        let entry = entry.expect("dir entry");
        let from = entry.path();
        let to = dst.join(entry.file_name());
        if from.is_dir() {
            copy_tree(&from, &to);
        } else {
            fs::copy(&from, &to).expect("copy pack file");
        }
    }
}

/// Bump `ty`'s manifest `schema-version` `from → to` in the copied pack at `pack`.
fn bump_manifest(pack: &Path, ty: &str, from: u32, to: u32) {
    let manifest_path = pack.join("config").join("schema-manifest.yaml");
    let manifest = fs::read_to_string(&manifest_path).expect("read the copied manifest");
    let bumped = manifest.replacen(
        &format!("- type: {ty}\n    schema-version: {from}"),
        &format!("- type: {ty}\n    schema-version: {to}"),
        1,
    );
    assert_ne!(
        manifest, bumped,
        "the manifest must carry `{ty}` at schema-version {from}",
    );
    fs::write(&manifest_path, bumped).expect("write the bumped manifest");
}

/// Initialize a real git repo with one commit plus the `.jigc/config/` project layer the cascade
/// expects (the `docs-root` knob stays at its shipped default, `docs/`).
fn init_repo(root: &Path) {
    let git = |args: &[&str]| {
        let out = Command::new("git")
            .args(args)
            .current_dir(root)
            .output()
            .expect("run git");
        assert!(
            out.status.success(),
            "git {args:?} failed: {}",
            String::from_utf8_lossy(&out.stderr)
        );
    };
    git(&["init", "-q"]);
    git(&["config", "user.email", "test@example.com"]);
    git(&["config", "user.name", "Test"]);
    fs::write(root.join("README.md"), "hello\n").expect("write file");
    git(&["add", "."]);
    git(&["commit", "-q", "-m", "initial"]);
    fs::create_dir_all(root.join(".jigc").join("config")).expect("create project layer");
}

/// Commit `source` at `docs/decisions/alpha.md` — the one-doc corpus every cell migrates.
fn commit_adr(repo: &Path, source: &str) {
    let dir = repo.join("docs").join("decisions");
    fs::create_dir_all(&dir).expect("mk docs/decisions/");
    fs::write(dir.join("alpha.md"), source).expect("write the adr");
    for args in [
        &["add", "."][..],
        &["commit", "-q", "-m", "seed an adr"][..],
    ] {
        let out = Command::new("git")
            .args(args)
            .current_dir(repo)
            .output()
            .expect("run git");
        assert!(out.status.success(), "git {args:?} failed");
    }
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

/// The sole `blocked[]` member of a `--format json` run, with the refusal's exit asserted: a
/// corpus with a doc the verb could not migrate is **not migrated**, and the caller hears so.
fn sole_blocked(out: &std::process::Output) -> serde_json::Value {
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(
        !out.status.success(),
        "a halted doc holds the exit; stdout:\n{stdout}\nstderr:\n{}",
        String::from_utf8_lossy(&out.stderr),
    );
    let report: serde_json::Value =
        serde_json::from_str(&stdout).unwrap_or_else(|err| panic!("the report is JSON: {err}"));
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

/// The shipped Framing-A route, byte-identical: a gate halt is the one cause an author *can*
/// clear from the doc, so its route is unchanged — and still followable.
fn framing_a_route(rel_key: &str) -> String {
    format!(
        "author the new required prose in `{rel_key}` through the write verbs, then re-run \
         `jigc migrate-corpus` (the schema-version stamp flips only once it gates clean)"
    )
}

// ---------------------------------------------------------------------------------------------
// Cell A — the gate, with nothing minted (the stock pack: no fixture schema at all).
// ---------------------------------------------------------------------------------------------

/// A **v0** (pre-stamp) ADR whose `## Consequences` heading is present with **no prose**. The
/// migration's only change is the schema-version stamp; the empty slot predates it entirely.
const V0_ADR_EMPTY_CONSEQUENCES: &str = "\
---
status: accepted
---

# Alpha decision

## Context

Session lookups must stay sub-millisecond.

## Options

A distributed cache was weighed and rejected on latency.

## Decision

Keep sessions in a single in-memory node.

## Consequences

";

/// **The message stops claiming a mint where nothing was minted.** Driven over the **stock**
/// pack — this is an ordinary brownfield state, not a mutated-schema fixture: a v0 ADR with an
/// authored-but-empty `## Consequences`. The fold adds the stamp, the gate breaks on the slot
/// that was empty before the run started, and the report used to say the migration *"mints a new
/// **required** prose slot"* — a claim about the migration that is simply false.
///
/// It now **relays the gate's own words**, and the route is unchanged: authoring that prose and
/// re-running is exactly what clears this doc.
#[test]
fn a_pre_existing_empty_slot_is_never_described_as_a_mint() {
    let repo = TempDir::new("repo");
    let home = TempDir::new("home");
    let pack = embedded_pack_tree();
    init_repo(repo.path());
    commit_adr(repo.path(), V0_ADR_EMPTY_CONSEQUENCES);

    let out = jigc(
        repo.path(),
        home.path(),
        &pack,
        &["migrate-corpus", "--format", "json"],
    );
    let finding = sole_blocked(&out);
    assert_eq!(
        finding["code"], "migrate-corpus.prose-needed",
        "a gate halt keeps the contract-pinned Framing-A waypoint; finding:\n{finding:#}",
    );
    let message = finding["message"].as_str().expect("a message");
    assert!(
        !message.contains("mints a new"),
        "nothing was minted — the empty slot predates the run; message: {message}",
    );
    assert!(
        message.contains("required slot in section `consequences` is empty"),
        "the message relays the gate's own finding, never a re-wording of it; message: {message}",
    );
    assert_eq!(
        finding["route"],
        serde_json::Value::String(framing_a_route("docs/decisions/alpha.md")),
        "the route is unchanged and still followable; finding:\n{finding:#}",
    );
}

// ---------------------------------------------------------------------------------------------
// Cell B — not the gate: an uncovered enum-member rename.
// ---------------------------------------------------------------------------------------------

/// A dev-pack copy in which `adr` is bumped **2 → 3** by a **renamed set of `status` enum
/// members** for which no old→new map is authored:
///
/// - `schema-snapshots/adr.v2.yaml` — the prior shape, whose `status` field declares
///   `of: [proposed, acc, sup]`;
/// - `schemas/adr.yaml` — **shipped verbatim** (the pack-load freeze gate stays quiet);
/// - `config/schema-manifest.yaml` — the `adr` entry's `schema-version` bumped `2 → 3`.
///
/// The classifier emits `ValueRemapped` with an **empty** map (which old member becomes which new
/// one is unrecoverable from the schema pair), `authored_remap` carries no `adr` entry, and the
/// committed `status: accepted` is therefore uncovered — the driver refuses the doc.
fn pack_renaming_the_adr_status_members(tag: &str) -> TempDir {
    let dir = TempDir::new(tag);
    copy_tree(&embedded_pack_tree(), dir.path());

    let current = fs::read_to_string(dir.path().join("schemas").join("adr.yaml"))
        .expect("read the copied adr.yaml");
    let prior = current.replacen(
        "of: [proposed, accepted, superseded]",
        "of: [proposed, acc, sup]",
        1,
    );
    assert_ne!(
        current, prior,
        "adr.yaml must declare the `status` enum members",
    );
    fs::write(
        dir.path().join("schema-snapshots").join("adr.v2.yaml"),
        prior,
    )
    .expect("write the adr.v2 snapshot");

    bump_manifest(dir.path(), "adr", 2, 3);
    dir
}

/// A conformant, **v2-stamped** ADR carrying `status: accepted` — the committed value the empty
/// remap table cannot cover.
const V2_ADR: &str = "\
---
status: accepted
schema-version: 2
---

# Alpha decision

## Context

Session lookups must stay sub-millisecond.

## Options

A distributed cache was weighed and rejected on latency.

## Decision

Keep sessions in a single in-memory node.

## Consequences

A cold node loses its sessions.
";

/// **A non-gate halt routes at the repair that exists.** An uncovered enum rename is a
/// *schema-authoring* gap: the old→new map is a CLI-supplied migration input, so nothing an
/// author writes **into this doc** and no re-run clears it. It takes the non-gate code, its route
/// names the authored map, and it never instructs anyone to author prose.
#[test]
fn an_uncovered_enum_rename_routes_at_the_authored_map() {
    let repo = TempDir::new("repo");
    let home = TempDir::new("home");
    let pack = pack_renaming_the_adr_status_members("pack");
    init_repo(repo.path());
    commit_adr(repo.path(), V2_ADR);

    let out = jigc(
        repo.path(),
        home.path(),
        pack.path(),
        &["migrate-corpus", "--format", "json"],
    );
    let finding = sole_blocked(&out);
    assert_eq!(
        finding["code"], "migrate-corpus.fold-refused",
        "a halt the gate never saw takes the non-gate code; finding:\n{finding:#}",
    );
    let route = finding["route"].as_str().expect("a route");
    assert!(
        route.contains("authored_remap"),
        "the route names the authored old→new map — the repair that actually exists; \
         route: {route}",
    );
    assert!(
        !route.contains("prose"),
        "no prose an author writes into this doc changes the outcome; route: {route}",
    );
    let message = finding["message"].as_str().expect("a message");
    assert!(
        !message.contains("mints a new"),
        "the fold minted nothing — it never produced bytes at all; message: {message}",
    );

    // And the doc is byte-untouched: a refused doc rolls back whole.
    let after = fs::read_to_string(repo.path().join("docs").join("decisions").join("alpha.md"))
        .expect("read the adr");
    assert_eq!(after, V2_ADR, "a refused doc keeps its committed bytes");
}

// ---------------------------------------------------------------------------------------------
// Cell C — the gate, over a slot the migration genuinely minted.
// ---------------------------------------------------------------------------------------------

/// A dev-pack copy in which `adr` is bumped **2 → 3** by **adding back its required
/// `## Consequences` section**: the prior snapshot is the shipped `adr.yaml` with that section
/// deleted, the current shape ships verbatim, and the manifest bumps `2 → 3`. A committed doc
/// that carries no `## Consequences` heading therefore has one **minted empty** by the fold —
/// the Framing-A handoff in its genuine form.
fn pack_adding_the_required_consequences_section(tag: &str) -> TempDir {
    let dir = TempDir::new(tag);
    copy_tree(&embedded_pack_tree(), dir.path());

    let current = fs::read_to_string(dir.path().join("schemas").join("adr.yaml"))
        .expect("read the copied adr.yaml");
    let prior = current.replacen(
        "  - id: consequences\n    slot: { hint: \"Tradeoffs and follow-on effects.\" }\n",
        "",
        1,
    );
    assert_ne!(
        current, prior,
        "adr.yaml must declare the required `consequences` section",
    );
    fs::write(
        dir.path().join("schema-snapshots").join("adr.v2.yaml"),
        prior,
    )
    .expect("write the adr.v2 snapshot");

    bump_manifest(dir.path(), "adr", 2, 3);
    dir
}

/// A conformant, **v2-stamped** ADR with **no** `## Consequences` heading — conformant under the
/// prior shape, and the doc the v3 fold mints the empty required slot into.
const V2_ADR_WITHOUT_CONSEQUENCES: &str = "\
---
status: accepted
schema-version: 2
---

# Alpha decision

## Context

Session lookups must stay sub-millisecond.

## Options

A distributed cache was weighed and rejected on latency.

## Decision

Keep sessions in a single in-memory node.
";

/// **The genuine mint keeps the waypoint it always had.** This is the case `prose-needed` was
/// written for: the fold minted an empty required slot, the gate blocks on it, and the route —
/// author the prose, then re-run — is exactly right. The code and the route are byte-identical to
/// what shipped; only the *message* changed, and only because it now says what the gate said
/// instead of asserting the mint on the gate's behalf.
#[test]
fn a_genuinely_minted_required_slot_keeps_the_framing_a_waypoint() {
    let repo = TempDir::new("repo");
    let home = TempDir::new("home");
    let pack = pack_adding_the_required_consequences_section("pack");
    init_repo(repo.path());
    commit_adr(repo.path(), V2_ADR_WITHOUT_CONSEQUENCES);

    let out = jigc(
        repo.path(),
        home.path(),
        pack.path(),
        &["migrate-corpus", "--format", "json"],
    );
    let finding = sole_blocked(&out);
    assert_eq!(
        finding["code"], "migrate-corpus.prose-needed",
        "a minted required slot is the Framing-A handoff; finding:\n{finding:#}",
    );
    assert_eq!(
        finding["route"],
        serde_json::Value::String(framing_a_route("docs/decisions/alpha.md")),
        "the route is byte-identical to the one that shipped; finding:\n{finding:#}",
    );
    let message = finding["message"].as_str().expect("a message");
    assert!(
        message.contains("required slot in section `consequences` is empty"),
        "the message relays the gate — the same relay as the un-minted cell, because the two \
         surfaces must not disagree about one break; message: {message}",
    );

    // The rollback is whole: the minted heading never reaches disk.
    let after = fs::read_to_string(repo.path().join("docs").join("decisions").join("alpha.md"))
        .expect("read the adr");
    assert_eq!(
        after, V2_ADR_WITHOUT_CONSEQUENCES,
        "a halted doc's scratch buffer is discarded, never written",
    );
}
