//! Acceptance — the adr **v1->v2** structural corpus migration runs end-to-end on the
//! shipped `jigc` binary with **no `JIGC_PACK_DIR`** (the real embedded dev pack),
//! sourcing the prior shape from the embedded `schema-snapshots/adr.v1.yaml` — the
//! **first-ever** `load_prior_schema` over the `EmbeddedPack` (M36 Increment 3, T3;
//! `design/corpus-migration.md` → Acceptance flows (the adr v1->v2 flow) + Prior-schema
//! sourcing; `design/worked-examples.md`).
//!
//! Flow 36 (`tests/flow36_corpus_structural.rs`) proved the structural path over a
//! `FilesystemPack` fixture (the reconstructed M25 `prd` reshape). This is the **first
//! live** frozen-schema shape change and the **first embedded-snapshot** migration: the
//! adr `options` optional slot shipped as a real v1->v2 bump (adr `schema-version` 1->2 +
//! recomputed hash in the manifest, T2; the `AddedOptionalSection` schema-diff kind +
//! empty-heading-insert transform, T1). A fixture repo carries a committed **v1-stamped**
//! ADR (no `## Options`); the shipped binary migrates it through `jigc migrate-corpus`
//! sourcing the embedded `adr.v1` snapshot — **deterministic, CLI-owned, no LLM in the
//! structural path** (the determinism boundary).
//!
//! Asserted over the **bytes an operator actually sees** (the migrated file on disk + the
//! emitted report/validate lines), never a reconstructed equivalent:
//!   - `jigc validate` routes the below-version v1 ADR `migrate` (report-only, exit 0);
//!   - `jigc migrate-corpus` **migrates** it (never blocks it with the missing-snapshot
//!     route — the RED obligation: without the shipped `adr.v1` snapshot the doc strands);
//!   - the empty `## Options` heading is spliced at its schema-ordered home, the stamp is
//!     value-bumped `1->2`, and **every prior slot value is byte-preserved**;
//!   - a re-validate finds the migrated ADR v2-conformant (no schema-conformance finding,
//!     no migrate route);
//!   - a re-run is a **byte-untouched no-op** (idempotent).

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
            "jigc-adr-options-{tag}-{}-{:?}",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos(),
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

/// Run `jigc <args>` with `cwd = repo`, `$HOME = home`, the real `doc-code` probe, and
/// **no `JIGC_PACK_DIR`** — the shipped embedded dev pack (with the embedded `adr.v1`
/// snapshot), exactly the production install.
fn jigc(repo: &Path, home: &Path, args: &[&str]) -> std::process::Output {
    Command::new(env!("CARGO_BIN_EXE_jigc"))
        .args(args)
        .current_dir(repo)
        .env("HOME", home)
        .env_remove("JIGC_PACK_DIR")
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

/// Make `root` a real git repo with identity, then run `jigc setup` over it (the embedded
/// pack — no `JIGC_PACK_DIR`).
fn setup_repo(repo: &Path, home: &Path) {
    git(repo, &["init", "-q"]);
    git(repo, &["config", "user.email", "test@example.com"]);
    git(repo, &["config", "user.name", "Test"]);
    fs::write(repo.join("README.md"), "hello\n").expect("write file");
    git(repo, &["add", "."]);
    git(repo, &["commit", "-q", "-m", "initial"]);

    let out = jigc(repo, home, &["setup"]);
    assert_ok(&out, "`jigc setup`");
}

/// A conformant **v1** `adr` body (the pre-`options` shape: `status` header +
/// `context`/`decision`/`consequences` slots, no `## Options`), stamped
/// `schema-version: 1` — the byte form a committed ADR had before the M36 v1->v2 bump.
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

/// Write + git-commit a v1 `adr` at `docs/decisions/<slug>.md`.
fn commit_adr(repo: &Path, slug: &str, title: &str) {
    let dir = repo.join("docs").join("decisions");
    fs::create_dir_all(&dir).expect("mk docs/decisions/");
    fs::write(dir.join(format!("{slug}.md")), adr_v1_body(title)).expect("write adr");
    git(repo, &["add", "."]);
    git(repo, &["commit", "-q", "-m", "seed adr"]);
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

/// The headline T3 proof: a committed **v1-stamped** ADR migrates through the shipped
/// `jigc migrate-corpus`, sourcing the prior shape from the **embedded** `adr.v1` snapshot
/// (the first-ever `load_prior_schema` over the `EmbeddedPack`) — the empty `## Options`
/// spliced, the stamp value-bumped `1->2`, every prior slot value byte-preserved.
#[test]
fn adr_options_v1_to_v2_migration_runs_through_the_shipped_binary() {
    let repo = TempDir::new("repo");
    let home = TempDir::new("home");
    setup_repo(repo.path(), home.path());

    commit_adr(repo.path(), "alpha-decision", "Alpha Decision");
    let before = fs::read_to_string(adr_path(repo.path(), "alpha-decision")).expect("read adr");
    // Precondition: the committed v1 ADR genuinely lacks the v2 `## Options` section, so the
    // migration has real structural work (not a stamp-only bump).
    assert!(
        !before.contains("## Options") && before.contains("schema-version: 1"),
        "the seed ADR is v1-stamped and carries no Options section; got:\n{before}",
    );

    // 1. DETECT — the version-aware store sweep routes the below-version ADR `migrate`
    //    (report-only, exit 0). The doc is otherwise structurally conformant (an absent
    //    optional section conforms), but it is stamped v1 under a manifest bumped to v2, so
    //    the version-mismatch break flags it.
    let detect = jigc(repo.path(), home.path(), &["validate"]);
    let detect_out = String::from_utf8_lossy(&detect.stdout);
    assert!(
        detect.status.success(),
        "`jigc validate` over a below-version corpus stays report-only (exit 0); \
         stdout:\n{detect_out}",
    );
    assert!(
        detect_out.contains("route: migrate — `docs/decisions/alpha-decision.md`"),
        "the v1-stamped ADR is routed `migrate` by the version-mismatch break; \
         stdout:\n{detect_out}",
    );

    // 2. MIGRATE — the verb sources the prior shape from the EMBEDDED `adr.v1` snapshot,
    //    classifies the delta as exactly `[AddedOptionalSection]`, splices the empty
    //    `## Options`, value-bumps the stamp, and writes back byte-stable, exit 0.
    //
    //    RED obligation: without the shipped `schema-snapshots/adr.v1.yaml`,
    //    `load_prior_schema` errs and the verb BLOCKS this doc with the missing-snapshot
    //    route (the corpus-corruption the migration prevents) — the `migrated` assertion
    //    below fails until the snapshot ships.
    let migrate = jigc(repo.path(), home.path(), &["migrate-corpus"]);
    let migrate_out = String::from_utf8_lossy(&migrate.stdout);
    assert_ok(&migrate, "`jigc migrate-corpus`");
    assert_eq!(
        count(&migrate_out, "migrated   docs/decisions/alpha-decision.md"),
        1,
        "the v1 ADR is migrated (not blocked with the missing-snapshot route); \
         stdout:\n{migrate_out}",
    );
    assert!(
        !migrate_out.contains("no prior-schema snapshot"),
        "the embedded adr.v1 snapshot resolves — no missing-snapshot block; \
         stdout:\n{migrate_out}",
    );

    // --- the adr structural migration landed byte-stable ---
    let after = fs::read_to_string(adr_path(repo.path(), "alpha-decision")).expect("read adr");
    // The empty optional `## Options` heading is spliced at its schema-ordered home.
    assert!(
        after.contains("## Options"),
        "the empty Options section is spliced in; got:\n{after}",
    );
    // The stamp is value-bumped 1->2 (the v1->v2 transition, distinct from the v0->v1 add).
    assert!(
        after.contains("schema-version: 2") && !after.contains("schema-version: 1"),
        "the adr stamp is value-bumped 1->2; got:\n{after}",
    );
    // Every prior slot value is byte-preserved verbatim — the header fields and each of the
    // three prose blocks survive unchanged (only the stamp value moved + Options inserted).
    assert!(
        after.contains("status: accepted\ndate: 2026-06-25\n"),
        "the header fields are byte-preserved; got:\n{after}",
    );
    assert!(
        after.contains("# Alpha Decision\n"),
        "the title is byte-preserved; got:\n{after}",
    );
    assert!(
        after.contains("## Context\n\nSession lookups must stay sub-millisecond.\n"),
        "the context prose is byte-preserved; got:\n{after}",
    );
    assert!(
        after.contains("## Decision\n\nKeep sessions in a single in-memory node.\n"),
        "the decision prose is byte-preserved; got:\n{after}",
    );
    assert!(
        after.contains("## Consequences\n\nA cold node loses its sessions.\n"),
        "the consequences prose is byte-preserved; got:\n{after}",
    );
    // The structure genuinely changed — the v1 form is gone.
    assert_ne!(before, after, "the adr was structurally rewritten");

    // 3. RE-VALIDATE — the migrated ADR is v2-conformant: no schema-conformance finding, no
    //    migrate route, exit 0.
    let reval = jigc(repo.path(), home.path(), &["validate"]);
    let reval_out = String::from_utf8_lossy(&reval.stdout);
    assert!(
        reval.status.success(),
        "re-validate exits 0; stdout:\n{reval_out}",
    );
    assert!(
        !reval_out.contains("schema-conformance"),
        "the migrated ADR surfaces NO schema-conformance finding; stdout:\n{reval_out}",
    );
    assert_eq!(
        count(&reval_out, "route: migrate"),
        0,
        "the migrated ADR carries no migrate route; stdout:\n{reval_out}",
    );

    // 4. IDEMPOTENT — a re-run is a byte-untouched no-op (round-trip identical): the now-v2
    //    ADR reads at-version and is skipped.
    let rerun = jigc(repo.path(), home.path(), &["migrate-corpus"]);
    assert_ok(&rerun, "`jigc migrate-corpus` re-run");
    assert_eq!(
        fs::read_to_string(adr_path(repo.path(), "alpha-decision")).expect("read adr"),
        after,
        "the re-run leaves the migrated ADR byte-untouched (idempotent)",
    );
    let rerun_out = String::from_utf8_lossy(&rerun.stdout);
    assert_eq!(
        count(&rerun_out, "migrated   "),
        0,
        "the re-run migrates nothing; stdout:\n{rerun_out}",
    );
}
