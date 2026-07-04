//! Acceptance — the **managed-corpus schema migration verb** runs the full M34
//! detect→block→migrate loop on the shipped `jigc` binary over a dev-pack corpus
//! (M34 Increment 3, T4; `design/corpus-migration.md` → Acceptance flows;
//! `design/worked-examples.md` → flows 35–37).
//!
//! The headline **dogfood**: a committed v0 ADR corpus (no schema-version stamp) is
//!   1. **detected** — `jigc validate` reports it `route: migrate` (T3, exit 0);
//!   2. **migrated** — `jigc migrate-corpus` adds the schema-version stamp **byte-stable
//!      via the real `added-optional-field` transform** (the live add-field e2e: the
//!      schema-diff classifier over the genuine corpus), writing each doc back only on a
//!      clean conformance gate;
//!   3. **re-validated** — `jigc validate` now finds the corpus conformant + stamped v1
//!      (no `schema-conformance` finding, exit 0).
//!
//! This drives the **built binary** end-to-end against the real store-scope sweep — the
//! bytes an operator actually sees.

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
            "jigc-corpus-migration-{tag}-{}-{:?}",
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

/// Run `jigc <args>` with `cwd = repo`, `$HOME = home`, and the real `doc-code` probe.
fn jigc(repo: &Path, home: &Path, args: &[&str]) -> std::process::Output {
    Command::new(env!("CARGO_BIN_EXE_jigc"))
        .args(args)
        .current_dir(repo)
        .env("HOME", home)
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

/// Make `root` a real git repo with identity, then run `jigc setup` over it.
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

/// A conformant `adr` body under the **pack** adr schema, optionally carrying a
/// `schema-version` stamp line in its header (`None` = the unstamped v0 state).
fn adr_body(title: &str, stamp: Option<u32>) -> String {
    let stamp_line = match stamp {
        Some(v) => format!("schema-version: {v}\n"),
        None => String::new(),
    };
    format!(
        "\
---
status: accepted
date: 2026-06-25
{stamp_line}---

# {title}

## Context

Session lookups must stay sub-millisecond.

## Options

A distributed cache was weighed and rejected on latency.

## Decision

Keep sessions in a single in-memory node.

## Consequences

A cold node loses its sessions.
"
    )
}

/// Commit an `adr` at its canonical `docs/decisions/<slug>.md`.
fn commit_adr(repo: &Path, slug: &str, title: &str, stamp: Option<u32>) {
    let dir = repo.join("docs").join("decisions");
    fs::create_dir_all(&dir).expect("mk docs/decisions/");
    fs::write(dir.join(format!("{slug}.md")), adr_body(title, stamp)).expect("write adr");
    git(repo, &["add", "."]);
    git(repo, &["commit", "-q", "-m", "seed adr"]);
}

/// The committed ADR's on-disk path.
fn adr_path(repo: &Path, slug: &str) -> PathBuf {
    repo.join("docs")
        .join("decisions")
        .join(format!("{slug}.md"))
}

/// Count non-overlapping occurrences of `needle` in `haystack`.
fn count(haystack: &str, needle: &str) -> usize {
    haystack.matches(needle).count()
}

/// The full M34 dogfood loop on the real binary: **detect → migrate → re-validate**.
///
/// A committed unstamped (v0) ADR corpus is reported `route: migrate` by `jigc validate`;
/// `jigc migrate-corpus` stamps it (the live add-field transform); a re-validate shows the
/// corpus conformant + stamped v1.
#[test]
fn migrate_corpus_stamps_the_v0_dogfood_then_revalidates_clean() {
    let repo = TempDir::new("dogfood");
    let home = TempDir::new("home");
    setup_repo(repo.path(), home.path());

    // The v0 corpus: a committed ADR with NO schema-version stamp, otherwise conformant.
    commit_adr(repo.path(), "alpha-decision", "Alpha decision", None);

    // 1. DETECT — `jigc validate` reports the stranded v0 doc, routed `migrate`, exit 0.
    let detect = jigc(repo.path(), home.path(), &["validate"]);
    let detect_out = String::from_utf8_lossy(&detect.stdout);
    assert!(
        detect.status.success(),
        "`jigc validate` over a v0 corpus stays report-only (exit 0); stdout:\n{detect_out}",
    );
    assert_eq!(
        count(&detect_out, "route: migrate"),
        1,
        "the unstamped v0 ADR is detected and routed `migrate`; stdout:\n{detect_out}",
    );

    // Precondition: the on-disk ADR genuinely carries no stamp before the migration.
    let before = fs::read_to_string(adr_path(repo.path(), "alpha-decision")).expect("read adr");
    assert!(
        !before.contains("schema-version:"),
        "the v0 ADR must be unstamped before migration; got:\n{before}",
    );

    // 2. MIGRATE — `jigc migrate-corpus` stamps the corpus byte-stable, exit 0.
    let migrate = jigc(repo.path(), home.path(), &["migrate-corpus"]);
    let migrate_out = String::from_utf8_lossy(&migrate.stdout);
    assert_ok(&migrate, "`jigc migrate-corpus`");
    assert_eq!(
        count(&migrate_out, "docs/decisions/alpha-decision.md"),
        1,
        "the migrated ADR is named in the report; stdout:\n{migrate_out}",
    );

    // The stamp landed on disk — the live add-field transform spliced `schema-version: 1`
    // into the existing header, the prior fields preserved.
    let after = fs::read_to_string(adr_path(repo.path(), "alpha-decision")).expect("read adr");
    assert!(
        after.contains("schema-version: 2"),
        "the migrated ADR carries the current schema-version stamp; got:\n{after}",
    );
    assert!(
        after.contains("status: accepted") && after.contains("date: 2026-06-25"),
        "the migration preserves the prior header fields; got:\n{after}",
    );
    assert!(
        after.contains("## Context") && after.contains("A cold node loses its sessions."),
        "the migration preserves the body prose; got:\n{after}",
    );

    // 3. RE-VALIDATE — the corpus is now conformant + stamped: no schema-conformance
    // finding, no migrate route, exit 0.
    let revalidate = jigc(repo.path(), home.path(), &["validate"]);
    let revalidate_out = String::from_utf8_lossy(&revalidate.stdout);
    assert!(
        revalidate.status.success(),
        "re-validate exits 0; stdout:\n{revalidate_out}",
    );
    assert!(
        !revalidate_out.contains("schema-conformance"),
        "the migrated corpus surfaces NO schema-conformance finding; stdout:\n{revalidate_out}",
    );
    assert_eq!(
        count(&revalidate_out, "route: migrate"),
        0,
        "the migrated corpus carries no migrate route; stdout:\n{revalidate_out}",
    );
}

/// A conformant **v1** (pre-relocation) `changelog` at the OLD folder home — the byte form
/// a real committed `docs/changelog/changelog.md` had before the M38 v1→v2 relocation: the
/// `location: changelog/` folder home under `docs-root`, a lowercase-slug `# changelog` H1
/// (no `display-title`), the `schema-version: 1` stamp, and the two empty KaC sections.
fn commit_v1_changelog(repo: &Path) {
    let body = "\
---
schema-version: 1
---

# changelog

## Unreleased Changes

## Releases
";
    let dir = repo.join("docs").join("changelog");
    fs::create_dir_all(&dir).expect("mk docs/changelog/");
    fs::write(dir.join("changelog.md"), body).expect("write v1 changelog");
    git(repo, &["add", "."]);
    git(repo, &["commit", "-q", "-m", "seed v1 changelog"]);
}

/// **Regression (M38 completion audit, HIGH):** the `migrate-corpus` verb must relocate a
/// committed **v1** `changelog` to its root `CHANGELOG.md` **placement** home through the
/// real binary. The doctype ships `placement: { file: CHANGELOG.md }` with `location: None`,
/// so the pre-fix `migrate_in_repo` job-list gate (`location.as_deref()` = `Some`) dropped
/// it — the relocation move-arm was never reached and a real project's stranded v1 changelog
/// migrated NOTHING, a permanent stuck state (the version detector kept flagging it). This
/// drives the **shipped binary** end-to-end (no `migrate_committed_corpus` shortcut), so it
/// fails on the gate defect and passes once the gate admits the placement doctype.
#[test]
fn migrate_corpus_relocates_the_v1_changelog_to_root_placement_home() {
    let repo = TempDir::new("changelog-relocate");
    let home = TempDir::new("home");
    setup_repo(repo.path(), home.path());

    // A committed v1 changelog at the pre-relocation folder home (`docs/changelog/`).
    commit_v1_changelog(repo.path());
    assert!(
        repo.path().join("docs/changelog/changelog.md").exists(),
        "precondition: the v1 changelog sits at its old folder home",
    );

    // MIGRATE — the real verb must relocate it to root `CHANGELOG.md`, exit 0.
    let migrate = jigc(repo.path(), home.path(), &["migrate-corpus"]);
    let migrate_out = String::from_utf8_lossy(&migrate.stdout);
    assert_ok(&migrate, "`jigc migrate-corpus`");
    assert_eq!(
        count(&migrate_out, "CHANGELOG.md"),
        1,
        "the relocated changelog is named in the report; stdout:\n{migrate_out}",
    );

    // The doc moved to the literal root placement home, byte-faithful: H1 fixed to
    // `# Changelog` (display-title), stamp value-bumped 1→2, KaC sections preserved.
    assert!(
        !repo.path().join("docs/changelog/changelog.md").exists(),
        "the old folder home is emptied after the relocation move; stdout:\n{migrate_out}",
    );
    let moved = fs::read_to_string(repo.path().join("CHANGELOG.md")).expect("read CHANGELOG.md");
    assert!(
        moved.starts_with("---\nschema-version: 2\n---\n\n# Changelog\n"),
        "the relocated changelog is v2-stamped with the `# Changelog` H1; got:\n{moved}",
    );
    assert!(
        moved.contains("## Unreleased Changes") && moved.contains("## Releases"),
        "the relocation preserves the two KaC sections; got:\n{moved}",
    );
}

/// The false-positive guard: a corpus already stamped at the current version is a clean
/// no-op — `jigc migrate-corpus` migrates nothing and reports the doc already current.
#[test]
fn migrate_corpus_is_a_no_op_on_an_already_current_corpus() {
    let repo = TempDir::new("current");
    let home = TempDir::new("home");
    setup_repo(repo.path(), home.path());
    commit_adr(repo.path(), "beta-decision", "Beta decision", Some(2));

    let before = fs::read_to_string(adr_path(repo.path(), "beta-decision")).expect("read adr");

    let migrate = jigc(repo.path(), home.path(), &["migrate-corpus"]);
    assert_ok(&migrate, "`jigc migrate-corpus` on a current corpus");

    // Byte-identical: a current doc is never rewritten.
    let after = fs::read_to_string(adr_path(repo.path(), "beta-decision")).expect("read adr");
    assert_eq!(
        before, after,
        "an already-current ADR is left byte-identical by the migration",
    );
}
