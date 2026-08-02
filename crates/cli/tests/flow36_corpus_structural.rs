//! Acceptance — the **v1->v2 structural** corpus migration runs end-to-end on the shipped
//! `jigc` binary, sourcing each committed doc's prior shape from the versioned snapshot
//! store (M34 Increment 4, T3; `design/corpus-migration.md` → Prior-schema sourcing +
//! Acceptance flows (the migrate flow); `design/worked-examples.md` → flow 36).
//!
//! Flow 35 (`tests/corpus_migration.rs`) proved the v0->v1 add-field **stamp** dogfood; this
//! is the headline structural proof the snapshot store makes reachable through the real
//! verb: the **reconstructed M25 `prd.requirements` fixed-slot->repeatable reshape**. A
//! `FilesystemPack` fixture (selected via `JIGC_PACK_DIR`) ships
//!   - the **current** `prd` schema at **manifest version 2** (the repeatable shape), and
//!   - `schema-snapshots/prd.v1.yaml` — the **fixed-slot prior shape**.
//!
//! A committed corpus of **v1-stamped fixed-slot** `prd` docs migrates through the compiled
//! `jigc migrate-corpus`: the verb diffs the snapshot `from` against the current `to`,
//! applies the `fixed-slot->repeatable-with-default` structural splice (the old slot prose
//! preserved as the default first item), value-bumps the stamp `1->2`, gates each doc on
//! conformance, and writes it back byte-stable — **deterministic, CLI-owned, no LLM in the
//! structural path** (the determinism boundary).
//!
//! Asserted over the **bytes an operator actually sees** (the migrated files on disk + the
//! emitted report/validate lines), never a reconstructed equivalent:
//!   - **prd** migrates byte-stable: slot prose preserved as the default item; stamp
//!     value-bumped `1->2`; v2-conformant (a re-validate finds no schema-conformance
//!     finding); round-trip identical (a re-run is a byte-untouched no-op — idempotent);
//!   - **deterministic across id-order AND reverse commit orders** — two repos seeded in
//!     opposite commit orders migrate to byte-identical output (increment-workflow #7);
//!   - a **widened-cardinality** secondary (`adr.supersedes` `0..1`->`0..*`) migrates
//!     byte-identical-except-stamp (the widening is a byte no-op; only the stamp bumps);
//!   - a **below-version stamped** doc is **migrated** (never `already-current`) and the
//!     **detector and verb agree**: `jigc validate` routes the v1 prd docs `migrate` and
//!     `jigc migrate-corpus` migrates those same docs (DECISIONS.md → audit Finding 2).

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::OnceLock;

/// A throwaway directory that removes itself on drop.
struct TempDir(PathBuf);

impl TempDir {
    fn new(tag: &str) -> Self {
        let mut path = std::env::temp_dir();
        path.push(format!(
            "jigc-corpus-structural-{tag}-{}-{:?}",
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

/// Build the pack's `doc-code` probe once (process-wide) and return its binary path — the
/// real tree-sitter subprocess the `jigc validate` pre-flight resolves.
fn doc_code_probe() -> &'static Path {
    static PROBE: OnceLock<PathBuf> = OnceLock::new();
    PROBE.get_or_init(|| {
        let manifest = Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("probes")
            .join("doc-code")
            .join("Cargo.toml");
        let out = Command::new(env!("CARGO"))
            .args(["build", "--quiet", "--manifest-path"])
            .arg(&manifest)
            .output()
            .expect("invoke cargo build for doc-code");
        assert!(
            out.status.success(),
            "building the doc-code probe failed:\n{}",
            String::from_utf8_lossy(&out.stderr),
        );
        let bin = manifest
            .parent()
            .unwrap()
            .join("target")
            .join("debug")
            .join("doc-code");
        assert!(bin.is_file(), "doc-code binary missing at {bin:?}");
        bin
    })
}

/// Recursively copy `src` into `dst` (creating `dst`), used to clone the embedded dev pack
/// into a writable fixture root before overlaying the snapshot store + manifest bumps.
fn copy_dir_all(src: &Path, dst: &Path) {
    fs::create_dir_all(dst).expect("mk dst dir");
    for entry in fs::read_dir(src).expect("read src dir") {
        let entry = entry.expect("dir entry");
        let from = entry.path();
        let to = dst.join(entry.file_name());
        if entry.file_type().expect("file type").is_dir() {
            copy_dir_all(&from, &to);
        } else {
            fs::copy(&from, &to).expect("copy file");
        }
    }
}

/// Build the **v1->v2 fixture pack** in `pack_dir`: clone the shipped dev pack (so setup +
/// every command path behaves exactly like the real pack), overlay the two prior-schema
/// snapshots (`prd.v1` fixed-slot, `adr.v1` narrower-cardinality), and bump `prd` + `adr` to
/// **manifest version 2** so their committed v1-stamped docs read as below-version. The
/// current `prd`/`adr` schemas in the clone are already the v2 shapes (repeatable
/// `requirements`; `supersedes` at `0..*`), so the snapshots are the only prior shapes to add.
///
/// Since M42 Inc 6 the pack-load freeze gate fires inside the pack-source factory, so it
/// runs on **every** door this fixture drives (setup / migrate-corpus / validate), not just
/// compose. A fixture pack must therefore be **freeze-consistent**: the overlaid `adr` shape
/// below re-syncs its manifest `schema-hash` ([`resync_adr_manifest_hash`]) rather than
/// riding a stale one. The version map (`schema-version`) is still the authority the
/// detector routes against; only the hash is recomputed.
fn build_fixture_pack(pack_dir: &Path) {
    let embedded = Path::new(env!("CARGO_MANIFEST_DIR")).join("pack");
    copy_dir_all(&embedded, pack_dir);

    let snaps_src = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("fixtures")
        .join("corpus-structural")
        .join("schema-snapshots");
    let snaps_dst = pack_dir.join("schema-snapshots");
    copy_dir_all(&snaps_src, &snaps_dst);

    // Pin this fixture's **current** `adr` shape to the pre-M36 no-`options` `supersedes: 0..*`
    // form. The real shipped adr is now v2-with-`options` (M36 inc-3), but flow36's synthetic
    // scenario is the *widened-cardinality* secondary — a byte-inert `0..1 -> 0..*` migration —
    // so its `adr` current shape must stay optionless to keep that migration byte-identical
    // except the stamp bump. The real options-slot v1->v2 migration is proven by T3's own flow,
    // over the embedded `adr.v1` snapshot; this test stays decoupled from it.
    let adr_no_options = "\
type: adr
location: decisions/
id-from: title
description: A dated architectural decision record, capturing the context a choice was made in, the choice itself, and its consequences, with an optional link to the decision it supersedes.
usage: a choice is worth preserving with its rationale, so a later reader can recover why the call was made or supersede it on the record.

sections:
  - id: status
    header: true
    fields:
      - { id: status, type: enum, of: [proposed, accepted, superseded], default: proposed }
      - { id: date, type: date, set: on-create }
      - { id: supersedes, type: ref, to: adr, card: \"0..*\", inverse: superseded-by }
      - { id: cites-code, type: code-anchor }
  - id: context
    slot: { hint: \"Why a decision was needed — the forces at play.\" }
  - id: decision
    slot: { hint: \"What we decided, in a sentence or two.\" }
  - id: consequences
    slot: { hint: \"Tradeoffs and follow-on effects.\" }
";
    fs::write(pack_dir.join("schemas").join("adr.yaml"), adr_no_options)
        .expect("overlay the no-options adr schema into the fixture pack");

    // Bump `prd` and `adr` to manifest version 2 (every other doctype stays v1). Each
    // `- type: <name>` block is unique, so the targeted version-line replacement is
    // unambiguous.
    let manifest_path = pack_dir.join("config").join("schema-manifest.yaml");
    let manifest = fs::read_to_string(&manifest_path).expect("read fixture manifest");
    let manifest = manifest.replace(
        "  - type: prd\n    schema-version: 1",
        "  - type: prd\n    schema-version: 2",
    );
    let manifest = manifest.replace(
        "  - type: adr\n    schema-version: 1",
        "  - type: adr\n    schema-version: 2",
    );
    assert!(
        manifest.contains("  - type: prd\n    schema-version: 2")
            && manifest.contains("  - type: adr\n    schema-version: 2"),
        "the fixture manifest must declare prd + adr at version 2; got:\n{manifest}",
    );
    fs::write(&manifest_path, resync_adr_manifest_hash(pack_dir, manifest))
        .expect("write bumped manifest");
}

/// Re-sync the fixture manifest's `adr` `schema-hash` to the shape the fixture pack **actually
/// ships** (the overlaid no-options `adr` above). The pack-load freeze gate recomputes each
/// doctype's hash over the pack's own schemas at *every* door (M42 Inc 6), so a fixture that
/// hand-edits a shape must recompute its hash or be blocked before the migration it exists to
/// prove ever runs. Hashed exactly as the gate does: the pack's field-type declarations
/// resolved in, then the engine's `schema-version` stamp injected (`adr` is persisted +
/// manifest-frozen), then [`engine::manifest::schema_hash`].
fn resync_adr_manifest_hash(pack_dir: &Path, manifest: String) -> String {
    let field_types = fs::read_to_string(pack_dir.join("config").join("field-types.yaml"))
        .expect("the fixture pack ships config/field-types.yaml");
    let decls: Vec<engine::schema::PackTypeDecl> =
        serde_yaml_ng::from_str(&field_types).expect("the pack's field-type declarations parse");
    let bytes = fs::read(pack_dir.join("schemas").join("adr.yaml")).expect("read the fixture adr");
    let mut schema = engine::schema::load_schema_with_types(&bytes, &decls)
        .expect("the overlaid adr schema loads");
    engine::schema::inject_schema_version_stamp(&mut schema);
    let hash = engine::manifest::schema_hash(&schema);

    let needle = "  - type: adr\n    schema-version: 2\n    schema-hash: ";
    let at = manifest
        .find(needle)
        .expect("the fixture manifest declares a v2 adr entry with a hash");
    let start = at + needle.len();
    let end = start + hash.len();
    let mut manifest = manifest;
    assert!(
        manifest[start..end].chars().all(|c| c.is_ascii_hexdigit()),
        "the spliced span must be the adr entry's hex digest; got `{}`",
        &manifest[start..end],
    );
    manifest.replace_range(start..end, &hash);
    manifest
}

/// Run a `git` command in `repo`, asserting success, returning trimmed stdout.
fn git(repo: &Path, args: &[&str]) -> String {
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
    String::from_utf8(out.stdout)
        .expect("utf-8")
        .trim()
        .to_string()
}

/// Run `jigc <args>` with `cwd = repo`, `$HOME = home`, the fixture `pack`, and the real
/// `doc-code` probe.
fn jigc(repo: &Path, home: &Path, pack: &Path, args: &[&str]) -> std::process::Output {
    Command::new(env!("CARGO_BIN_EXE_jigc"))
        .args(args)
        .current_dir(repo)
        .env("HOME", home)
        .env("JIGC_PACK_DIR", pack)
        .env("JIGC_DOC_CODE_PROBE", doc_code_probe())
        .output()
        .expect("run the jigc binary")
}

/// Assert a `jigc` invocation succeeded, surfacing its streams on failure.
fn assert_ok(out: &std::process::Output, what: &str) {
    assert!(
        out.status.success(),
        "{what} must succeed; stdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr),
    );
}

/// Make `root` a real git repo with identity, then run `jigc setup` over it (with the fixture
/// pack).
fn setup_repo(repo: &Path, home: &Path, pack: &Path) {
    git(repo, &["init", "-q"]);
    git(repo, &["config", "user.email", "test@example.com"]);
    git(repo, &["config", "user.name", "Test"]);
    fs::write(repo.join("README.md"), "hello\n").expect("write file");
    git(repo, &["add", "."]);
    git(repo, &["commit", "-q", "-m", "initial"]);

    let out = jigc(repo, home, pack, &["setup"]);
    assert_ok(&out, "`jigc setup`");
}

/// A conformant **fixed-slot (v1)** `prd` body, stamped `schema-version: 1` in its injected
/// `meta` header — the byte form a committed v1 prd had before the M25 repeatable reshape.
fn prd_v1_body(title: &str, requirement: &str) -> String {
    format!(
        "\
---
schema-version: 1
---

# {title}

## Vision

A fast, predictable system.

## Requirements

{requirement}

## Context

Latency budgets are tight and the team is small.
"
    )
}

/// A conformant **v1** `adr` body (the `supersedes` `0..1` prior shape — it carries no
/// `supersedes`, so it is valid under both `0..1` and the widened `0..*`), stamped
/// `schema-version: 1`.
fn adr_v1_body(title: &str) -> String {
    format!(
        "\
---
status: accepted
date: 2026-06-25
schema-version: 1
---

# {title}

## Context

Session lookups must stay sub-millisecond.

## Decision

Keep sessions in a single in-memory node.

## Consequences

A cold node loses its sessions.
"
    )
}

/// Write + git-commit a `prd` at `docs/prds/<slug>.md`.
fn commit_prd(repo: &Path, slug: &str, title: &str, requirement: &str) {
    let dir = repo.join("docs").join("prds");
    fs::create_dir_all(&dir).expect("mk docs/prds/");
    fs::write(
        dir.join(format!("{slug}.md")),
        prd_v1_body(title, requirement),
    )
    .expect("write prd");
    git(repo, &["add", "."]);
    git(repo, &["commit", "-q", "-m", "seed prd"]);
}

/// Write + git-commit an `adr` at `docs/decisions/<slug>.md`.
fn commit_adr(repo: &Path, slug: &str, title: &str) {
    let dir = repo.join("docs").join("decisions");
    fs::create_dir_all(&dir).expect("mk docs/decisions/");
    fs::write(dir.join(format!("{slug}.md")), adr_v1_body(title)).expect("write adr");
    git(repo, &["add", "."]);
    git(repo, &["commit", "-q", "-m", "seed adr"]);
}

fn prd_path(repo: &Path, slug: &str) -> PathBuf {
    repo.join("docs").join("prds").join(format!("{slug}.md"))
}

fn adr_path(repo: &Path, slug: &str) -> PathBuf {
    repo.join("docs")
        .join("decisions")
        .join(format!("{slug}.md"))
}

/// Count non-overlapping occurrences of `needle` in `haystack`.
fn count(haystack: &str, needle: &str) -> usize {
    haystack.matches(needle).count()
}

/// The headline T3 proof: a v1-stamped fixed-slot `prd` corpus + a v1 `adr` migrates through
/// the compiled `jigc migrate-corpus`, sourcing each prior shape from the snapshot store.
#[test]
fn structural_v1_to_v2_migration_runs_through_the_real_binary() {
    let pack = TempDir::new("pack");
    build_fixture_pack(pack.path());

    let repo = TempDir::new("repo");
    let home = TempDir::new("home");
    setup_repo(repo.path(), home.path(), pack.path());

    // The v1 corpus: two fixed-slot prd docs (the structural case) + one adr (the
    // widened-cardinality secondary), all stamped below the bumped manifest version.
    commit_prd(
        repo.path(),
        "cache-prd",
        "Cache Prd",
        "The cache must answer in under a millisecond.",
    );
    commit_prd(
        repo.path(),
        "queue-prd",
        "Queue Prd",
        "The queue must never drop an enqueued job.",
    );
    commit_adr(repo.path(), "alpha-decision", "Alpha Decision");

    let prd_before = fs::read_to_string(prd_path(repo.path(), "cache-prd")).expect("read prd");
    let adr_before = fs::read_to_string(adr_path(repo.path(), "alpha-decision")).expect("read adr");

    // 1. DETECT — the version-aware store sweep routes every below-version doc `migrate`, and
    //    (M42) **exits non-zero**: an unmigrated corpus is the third exit-flipping exception
    //    (`design/validation.md` → Exit semantics). Both fixed-slot prd docs are structurally
    //    non-conformant under v2 and route `migrate`. The widened adr is otherwise structurally
    //    conformant, but it is stamped v1 under a manifest bumped to v2, so the version-currency
    //    break flags it too (`doc_findings.is_empty() && stamp < current` → below-version): all
    //    THREE docs route `migrate` here, and the verb migrates each of them below.
    let detect = jigc(repo.path(), home.path(), pack.path(), &["validate"]);
    let detect_out = String::from_utf8_lossy(&detect.stdout);
    assert!(
        !detect.status.success(),
        "`jigc validate` over a below-version corpus exits non-zero; \
         stdout:\n{detect_out}",
    );
    assert_eq!(
        count(&detect_out, "route: migrate"),
        3,
        "every below-version doc is routed `migrate` — both fixed-slot prd docs AND the \
         v1-stamped widened adr (flagged by the version-mismatch break); stdout:\n{detect_out}",
    );
    // The adr's version-mismatch break specifically must route `migrate`: a regression that
    // stopped flagging the below-version-but-structurally-conformant adr turns this red.
    assert!(
        detect_out.contains("route: migrate — `docs/decisions/alpha-decision.md`"),
        "the v1-stamped widened adr is routed `migrate` by the version-mismatch break; \
         stdout:\n{detect_out}",
    );

    // 2. MIGRATE — the verb sources each prior shape from the snapshot store, applies the
    //    structural splice, value-bumps the stamp, and writes back byte-stable, exit 0.
    let migrate = jigc(repo.path(), home.path(), pack.path(), &["migrate-corpus"]);
    let migrate_out = String::from_utf8_lossy(&migrate.stdout);
    assert_ok(&migrate, "`jigc migrate-corpus`");
    // Detector/verb AGREE: the docs the detector routed `migrate` are migrated by the verb
    // (never reported `already-current`) — DECISIONS.md → audit Finding 2.
    for name in [
        "docs/prds/cache-prd.md",
        "docs/prds/queue-prd.md",
        "docs/decisions/alpha-decision.md",
    ] {
        assert_eq!(
            count(&migrate_out, name),
            1,
            "{name} is named once in the migration report; stdout:\n{migrate_out}",
        );
    }
    assert!(
        !migrate_out.contains("already current\n    "),
        "nothing is reported already-current; stdout:\n{migrate_out}",
    );

    // --- the prd structural migration landed byte-stable ---
    let prd_after = fs::read_to_string(prd_path(repo.path(), "cache-prd")).expect("read prd");
    // The stamp is value-bumped 1->2 (the v1->v2 transition, distinct from the v0->v1 add).
    assert!(
        prd_after.contains("schema-version: 2") && !prd_after.contains("schema-version: 1"),
        "the prd stamp is value-bumped 1->2; got:\n{prd_after}",
    );
    // The old fixed-slot prose is preserved as the default first item of the now-repeatable
    // `requirements` section.
    assert!(
        prd_after.contains("The cache must answer in under a millisecond."),
        "the v1 slot prose survives as the default requirement item; got:\n{prd_after}",
    );
    // The structure genuinely changed — the v1 fixed-slot form is gone.
    assert_ne!(prd_before, prd_after, "the prd was structurally rewritten");

    // --- the adr widened-cardinality secondary: byte-identical EXCEPT the stamp bump ---
    let adr_after = fs::read_to_string(adr_path(repo.path(), "alpha-decision")).expect("read adr");
    assert_eq!(
        adr_after,
        adr_before.replace("schema-version: 1", "schema-version: 2"),
        "the widened-cardinality adr migrates byte-identical except the stamp value-bump",
    );

    // 3. RE-VALIDATE — the migrated corpus is v2-conformant: no schema-conformance finding,
    //    no migrate route, exit 0.
    let reval = jigc(repo.path(), home.path(), pack.path(), &["validate"]);
    let reval_out = String::from_utf8_lossy(&reval.stdout);
    assert!(
        reval.status.success(),
        "re-validate exits 0; stdout:\n{reval_out}"
    );
    assert!(
        !reval_out.contains("schema-conformance"),
        "the migrated corpus surfaces NO schema-conformance finding; stdout:\n{reval_out}",
    );
    assert_eq!(
        count(&reval_out, "route: migrate"),
        0,
        "the migrated corpus carries no migrate route; stdout:\n{reval_out}",
    );

    // 4. IDEMPOTENT — a re-run is a byte-untouched no-op (round-trip identical): the now-v2
    //    docs read at-version and are skipped.
    let rerun = jigc(repo.path(), home.path(), pack.path(), &["migrate-corpus"]);
    assert_ok(&rerun, "`jigc migrate-corpus` re-run");
    assert_eq!(
        fs::read_to_string(prd_path(repo.path(), "cache-prd")).expect("read prd"),
        prd_after,
        "the re-run leaves the migrated prd byte-untouched (idempotent)",
    );
    assert_eq!(
        fs::read_to_string(adr_path(repo.path(), "alpha-decision")).expect("read adr"),
        adr_after,
        "the re-run leaves the migrated adr byte-untouched (idempotent)",
    );
    let rerun_out = String::from_utf8_lossy(&rerun.stdout);
    assert_eq!(
        count(&rerun_out, "migrated  "),
        0,
        "the re-run migrates nothing; stdout:\n{rerun_out}",
    );
}

/// Seed a fresh repo over `pack` with the two prd docs committed in `order`, migrate it, and
/// return `(cache-prd bytes, queue-prd bytes)` after migration. The `home`/`repo` temp dirs
/// are dropped by the caller via the returned guards.
fn migrate_under_order(pack: &Path, order: [&str; 2]) -> (TempDir, TempDir, String, String) {
    let repo = TempDir::new("order-repo");
    let home = TempDir::new("order-home");
    setup_repo(repo.path(), home.path(), pack);

    for slug in order {
        match slug {
            "cache-prd" => commit_prd(
                repo.path(),
                "cache-prd",
                "Cache Prd",
                "The cache must answer in under a millisecond.",
            ),
            "queue-prd" => commit_prd(
                repo.path(),
                "queue-prd",
                "Queue Prd",
                "The queue must never drop an enqueued job.",
            ),
            other => panic!("unknown slug {other}"),
        }
    }

    let migrate = jigc(repo.path(), home.path(), pack, &["migrate-corpus"]);
    assert_ok(&migrate, "`jigc migrate-corpus` (ordered seed)");

    let cache = fs::read_to_string(prd_path(repo.path(), "cache-prd")).expect("read cache prd");
    let queue = fs::read_to_string(prd_path(repo.path(), "queue-prd")).expect("read queue prd");
    (repo, home, cache, queue)
}

/// Determinism (increment-workflow #7): the same two-prd corpus committed in **id-order** and
/// in **reverse** migrates to **byte-identical** output — the verb keys output on the
/// path-sorted corpus, never on commit order.
#[test]
fn structural_migration_is_byte_identical_across_commit_orders() {
    let pack = TempDir::new("pack");
    build_fixture_pack(pack.path());

    // id-order: cache-prd then queue-prd.
    let (_r1, _h1, cache_fwd, queue_fwd) =
        migrate_under_order(pack.path(), ["cache-prd", "queue-prd"]);
    // reverse: queue-prd then cache-prd.
    let (_r2, _h2, cache_rev, queue_rev) =
        migrate_under_order(pack.path(), ["queue-prd", "cache-prd"]);

    assert_eq!(
        cache_fwd, cache_rev,
        "cache-prd migrates byte-identically regardless of commit order",
    );
    assert_eq!(
        queue_fwd, queue_rev,
        "queue-prd migrates byte-identically regardless of commit order",
    );
    // Guard: the two distinct docs really did migrate to distinct content (the fixture forces
    // genuine, non-trivial output — not two empty strings compared equal).
    assert_ne!(
        cache_fwd, queue_fwd,
        "the two prd docs migrate to genuinely distinct content",
    );
    assert!(
        cache_fwd.contains("schema-version: 2"),
        "the migrated prd is genuinely v2-stamped; got:\n{cache_fwd}",
    );
}
