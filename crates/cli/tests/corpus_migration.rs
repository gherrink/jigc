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

/// A **stale** `changelog` sitting at its literal root `CHANGELOG.md` **placement home** but
/// still carrying the v1 shape (the `# changelog` slug-H1) and the `schema-version: 1` stamp
/// — the state an operator lands in after hand-moving the doc (`git mv`), or after any partial
/// migration. Carries a populated change-group so byte-preservation is *provable*, not
/// vacuous.
const STALE_ROOT_CHANGELOG_V1: &str = "\
---
schema-version: 1
---

# changelog

## Unreleased Changes

### added  {#added}

- OAuth login button on the sign-in page.

## Releases
";

/// The bytes the migration must produce from [`STALE_ROOT_CHANGELOG_V1`]: the stamp
/// value-bumped 1→2, the H1 fixed to the `display-title` (`# Changelog`), **every other byte
/// preserved**.
const MIGRATED_ROOT_CHANGELOG_V2: &str = "\
---
schema-version: 2
---

# Changelog

## Unreleased Changes

### added  {#added}

- OAuth login button on the sign-in page.

## Releases
";

/// Commit `body` at the literal root `CHANGELOG.md` placement home.
fn commit_root_changelog(repo: &Path, body: &str) {
    fs::write(repo.join("CHANGELOG.md"), body).expect("write CHANGELOG.md");
    git(repo, &["add", "."]);
    git(repo, &["commit", "-q", "-m", "seed root changelog"]);
}

/// **M42 Inc-1 T1 — the corpus walk's placement branches become a UNION.** Pre-fix,
/// `candidate_docs` keyed the two placement branches on whether the prior snapshot carried a
/// `location:` — and `changelog.v1.yaml` always does, so the relocation walk ran over an
/// (empty) `docs/changelog/` and the in-place branch was **dead code for this doctype**. A
/// stale root `CHANGELOG.md` was therefore **invisible**: `jigc migrate-corpus` reported
/// `0 migrated, 0 already current, 0 blocked` and the stamp stayed at 1 forever — which made
/// the family-5 detector's `migrate` route point at a verb that does nothing.
///
/// The union walks the prior home **and** the placement file: the stale doc is FOUND, migrated
/// in place (`Relocated` is a content no-op; `DisplayTitleChanged` fixes the H1), re-stamped 2
/// — and an immediate re-run is idempotent.
#[test]
fn migrate_corpus_finds_a_stale_changelog_at_its_placement_home() {
    let repo = TempDir::new("stale-placement");
    let home = TempDir::new("home");
    setup_repo(repo.path(), home.path());
    commit_root_changelog(repo.path(), STALE_ROOT_CHANGELOG_V1);

    let migrate = jigc(repo.path(), home.path(), &["migrate-corpus"]);
    let out = String::from_utf8_lossy(&migrate.stdout);
    assert_ok(
        &migrate,
        "`jigc migrate-corpus` over a stale placement-home doc",
    );
    assert!(
        out.contains("1 migrated") && out.contains("CHANGELOG.md"),
        "the stale placement-home changelog is FOUND and migrated; stdout:\n{out}",
    );

    // Golden: the stamp value-bumped 1→2, the H1 fixed to the display title, every other
    // byte preserved (the populated change-group survives verbatim).
    let after = fs::read_to_string(repo.path().join("CHANGELOG.md")).expect("read CHANGELOG.md");
    assert_eq!(
        after, MIGRATED_ROOT_CHANGELOG_V2,
        "the migrated placement-home changelog is byte-exact",
    );

    // Idempotent: the re-run finds it already current and rewrites nothing.
    let again = jigc(repo.path(), home.path(), &["migrate-corpus"]);
    let out2 = String::from_utf8_lossy(&again.stdout);
    assert_ok(&again, "the `jigc migrate-corpus` re-run");
    assert!(
        out2.contains("1 already current") && out2.contains("0 migrated"),
        "the re-run is idempotent (already current, nothing migrated); stdout:\n{out2}",
    );
    let after2 = fs::read_to_string(repo.path().join("CHANGELOG.md")).expect("read CHANGELOG.md");
    assert_eq!(after2, after, "the re-run leaves the doc byte-identical");
}

/// **M42 Inc-1 T1 — the both-homes state, the CONFLICTING arm.** The union makes it reachable
/// for the first time that a doc sits at the prior home *and* a **different** doc sits at the
/// relocation destination. The destination is never silently overwritten (No-data-loss): the
/// prior-home strand is **blocked with a route**, nothing is written to the destination on its
/// behalf, and nothing is removed.
#[test]
fn migrate_corpus_never_overwrites_a_different_doc_at_the_placement_home() {
    let repo = TempDir::new("both-homes-conflict");
    let home = TempDir::new("home");
    setup_repo(repo.path(), home.path());
    // A genuinely different doc at each home: the root one carries the OAuth entry, the
    // old-home one carries the empty KaC sections (`commit_v1_changelog`).
    commit_root_changelog(repo.path(), STALE_ROOT_CHANGELOG_V1);
    commit_v1_changelog(repo.path());

    let migrate = jigc(repo.path(), home.path(), &["migrate-corpus"]);
    let out = String::from_utf8_lossy(&migrate.stdout);
    assert_ok(
        &migrate,
        "`jigc migrate-corpus` over the both-homes conflict",
    );

    // The destination holds ITS OWN migrated content — never the strand's.
    let dest = fs::read_to_string(repo.path().join("CHANGELOG.md")).expect("read CHANGELOG.md");
    assert_eq!(
        dest, MIGRATED_ROOT_CHANGELOG_V2,
        "the destination doc migrated in place and was NOT overwritten by the strand; \
         stdout:\n{out}",
    );

    // The strand survives on disk — blocked, never deleted (No-data-loss).
    let strand = repo.path().join("docs/changelog/changelog.md");
    assert!(
        strand.is_file(),
        "the conflicting prior-home strand is NOT removed; stdout:\n{out}",
    );
    assert!(
        out.contains("1 blocked") && out.contains("docs/changelog/changelog.md"),
        "the conflicting strand is blocked with a route; stdout:\n{out}",
    );
}

/// **M42 Inc-1 T1 — the both-homes state, the SAME-DOC (interrupted-move) arm.** The
/// relocation is write-before-remove, so an abort between the two halves strands the source at
/// the old home while the destination already holds that doc's migrated bytes. The union sees
/// both; the re-run **completes** the move — the destination stays v2 and the old-home strand
/// is removed. Built from the tool's own output (a real relocation, then the strand restored),
/// so the "same doc" is genuine, not hand-forged.
#[test]
fn migrate_corpus_completes_an_interrupted_relocation() {
    let repo = TempDir::new("both-homes-interrupted");
    let home = TempDir::new("home");
    setup_repo(repo.path(), home.path());
    commit_v1_changelog(repo.path());

    // Run 1 — the real relocation: `docs/changelog/changelog.md` → `CHANGELOG.md`.
    let first = jigc(repo.path(), home.path(), &["migrate-corpus"]);
    assert_ok(&first, "`jigc migrate-corpus` (the relocation)");
    let dest_after_move =
        fs::read_to_string(repo.path().join("CHANGELOG.md")).expect("read CHANGELOG.md");

    // Re-strand the source at the old home: exactly the state an abort *between* the
    // destination write and the source remove leaves behind.
    commit_v1_changelog(repo.path());

    // Run 2 — the union sees both homes and COMPLETES the interrupted move.
    let second = jigc(repo.path(), home.path(), &["migrate-corpus"]);
    let out = String::from_utf8_lossy(&second.stdout);
    assert_ok(
        &second,
        "`jigc migrate-corpus` (completing the interrupted move)",
    );
    assert!(
        out.contains("1 migrated") && out.contains("CHANGELOG.md"),
        "the completed move is reported once; stdout:\n{out}",
    );
    assert!(
        !repo.path().join("docs/changelog/changelog.md").exists(),
        "the old-home strand is removed once the move completes; stdout:\n{out}",
    );
    assert_eq!(
        fs::read_to_string(repo.path().join("CHANGELOG.md")).expect("read CHANGELOG.md"),
        dest_after_move,
        "the destination keeps its v2 bytes — the completion is byte-identical",
    );
}

/// The repo's current `HEAD` sha.
fn head(repo: &Path) -> String {
    git(repo, &["rev-parse", "HEAD"])
}

/// `git status --porcelain` — empty iff the tree (and index) is clean.
fn porcelain(repo: &Path) -> String {
    git(repo, &["status", "--porcelain"])
}

/// The commits between `base` and `HEAD`, one sha per line.
fn commits_since(repo: &Path, base: &str) -> Vec<String> {
    git(repo, &["rev-list", &format!("{base}..HEAD")])
        .lines()
        .map(str::to_string)
        .filter(|l| !l.is_empty())
        .collect()
}

/// `git show --name-status --no-renames HEAD` — the landed commit's file set with the two
/// halves of a move kept **apart** (`D old` / `A new`), so both are assertable.
fn head_name_status(repo: &Path) -> String {
    git(
        repo,
        &["show", "--name-status", "--no-renames", "--format=", "HEAD"],
    )
}

/// **M42 Inc-1 T2 — the commit boundary.** `migrate-corpus` was git-free, so it left the
/// operator holding an unstaged delete + an untracked relocated file (` D
/// docs/changelog/changelog.md` + `?? CHANGELOG.md`) — landable only through the raw
/// `git add -A && git commit` the adapter contract forbids, which is exactly what the
/// adoption trial's agent was forced into. The verb now **lands its own migration**: a
/// pathspec-limited self-commit carrying **both halves** of the relocation move, leaving the
/// tree clean, and naming the commit in the report.
#[test]
fn migrate_corpus_commits_the_relocation_and_leaves_the_tree_clean() {
    let repo = TempDir::new("self-commit");
    let home = TempDir::new("home");
    setup_repo(repo.path(), home.path());
    commit_v1_changelog(repo.path());
    let base = head(repo.path());

    let migrate = jigc(repo.path(), home.path(), &["migrate-corpus"]);
    let out = String::from_utf8_lossy(&migrate.stdout);
    assert_ok(&migrate, "`jigc migrate-corpus`");

    // The tree is CLEAN — the verb landed its own migration; nothing is left for the
    // operator to `git add`.
    assert_eq!(
        porcelain(repo.path()),
        "",
        "the migration leaves a clean tree — it commits its own writes; stdout:\n{out}",
    );

    // Exactly ONE new commit, carrying BOTH halves of the move.
    let landed = commits_since(repo.path(), &base);
    assert_eq!(
        landed.len(),
        1,
        "the migration lands exactly one commit; got {landed:?}\nstdout:\n{out}",
    );
    let names = head_name_status(repo.path());
    assert!(
        names.contains("A\tCHANGELOG.md"),
        "the commit carries the ADD half of the move; got:\n{names}",
    );
    assert!(
        names.contains("D\tdocs/changelog/changelog.md"),
        "the commit carries the DELETE half of the move; got:\n{names}",
    );

    // The report names the commit it landed.
    let sha = git(repo.path(), &["rev-parse", "--short", "HEAD"]);
    assert!(
        out.contains(&sha),
        "the report names the landed commit `{sha}`; stdout:\n{out}",
    );
}

/// The **`--format json`** half of the same contract: the report carries the landed commit
/// as a machine-readable field (`null` when nothing was committed).
#[test]
fn migrate_corpus_names_the_landed_commit_in_json() {
    let repo = TempDir::new("self-commit-json");
    let home = TempDir::new("home");
    setup_repo(repo.path(), home.path());
    commit_v1_changelog(repo.path());

    let migrate = jigc(
        repo.path(),
        home.path(),
        &["migrate-corpus", "--format", "json"],
    );
    let out = String::from_utf8_lossy(&migrate.stdout);
    assert_ok(&migrate, "`jigc migrate-corpus --format json`");

    let sha = git(repo.path(), &["rev-parse", "--short", "HEAD"]);
    let report: serde_json::Value = serde_json::from_str(&out).expect("the report is JSON");
    assert_eq!(
        report["commit"].as_str(),
        Some(sha.as_str()),
        "the JSON report names the landed commit; stdout:\n{out}",
    );

    // A clean re-run commits nothing — the field is `null`, not a stale sha.
    let again = jigc(
        repo.path(),
        home.path(),
        &["migrate-corpus", "--format", "json"],
    );
    assert_ok(&again, "the `jigc migrate-corpus --format json` re-run");
    let out2 = String::from_utf8_lossy(&again.stdout);
    let report2: serde_json::Value =
        serde_json::from_str(&out2).expect("the re-run report is JSON");
    assert!(
        report2["commit"].is_null(),
        "a re-run that commits nothing carries a null commit; stdout:\n{out2}",
    );
}

/// **Never a blanket `git add -A`.** An ambient dirty tree — an unrelated **untracked** file
/// and an unrelated **unstaged** edit to a tracked file — is *not* swept into the migration's
/// commit: the staging is pathspec-limited to the paths the migration itself touched, so both
/// are still dirty afterwards (the M30 index-as-change-manifest discipline).
#[test]
fn migrate_corpus_never_sweeps_an_ambient_dirty_tree() {
    let repo = TempDir::new("ambient-dirt");
    let home = TempDir::new("home");
    setup_repo(repo.path(), home.path());
    commit_v1_changelog(repo.path());

    // Ambient dirt, unrelated to the migration.
    fs::write(repo.path().join("scratch.md"), "wip\n").expect("write untracked");
    fs::write(repo.path().join("README.md"), "hello, edited\n").expect("edit tracked");

    let migrate = jigc(repo.path(), home.path(), &["migrate-corpus"]);
    let out = String::from_utf8_lossy(&migrate.stdout);
    assert_ok(&migrate, "`jigc migrate-corpus` over an ambient dirty tree");

    // Both ambient changes survive, untouched and uncommitted.
    let status = porcelain(repo.path());
    assert!(
        status.contains("?? scratch.md"),
        "the unrelated untracked file is still untracked; status:\n{status}\nstdout:\n{out}",
    );
    assert!(
        status.contains("M README.md"),
        "the unrelated unstaged edit is still unstaged; status:\n{status}\nstdout:\n{out}",
    );
    let names = head_name_status(repo.path());
    assert!(
        !names.contains("scratch.md") && !names.contains("README.md"),
        "neither ambient path rides the migration commit; got:\n{names}",
    );
    // The migration itself still landed.
    assert!(
        names.contains("A\tCHANGELOG.md") && names.contains("D\tdocs/changelog/changelog.md"),
        "the migration's own move still landed; got:\n{names}",
    );
}

/// Idempotence at the commit boundary: a re-run over an already-migrated corpus stages
/// nothing, so it commits nothing — `HEAD` is unchanged and there is no empty commit.
#[test]
fn migrate_corpus_re_run_commits_nothing() {
    let repo = TempDir::new("re-run-commit");
    let home = TempDir::new("home");
    setup_repo(repo.path(), home.path());
    commit_v1_changelog(repo.path());

    let first = jigc(repo.path(), home.path(), &["migrate-corpus"]);
    assert_ok(&first, "`jigc migrate-corpus` (the first run)");
    let after_first = head(repo.path());

    let second = jigc(repo.path(), home.path(), &["migrate-corpus"]);
    let out = String::from_utf8_lossy(&second.stdout);
    assert_ok(&second, "the `jigc migrate-corpus` re-run");
    assert!(
        out.contains("0 migrated"),
        "the re-run migrates nothing; stdout:\n{out}",
    );
    assert_eq!(
        head(repo.path()),
        after_first,
        "the re-run commits nothing — HEAD is unchanged; stdout:\n{out}",
    );
    assert_eq!(
        porcelain(repo.path()),
        "",
        "the re-run leaves the tree clean; stdout:\n{out}",
    );
}

/// The user's hooks are **policy** (the `finalize`/`rename` posture — never `--no-verify`):
/// a `pre-commit` hook that rejects the commit makes the verb **fail loudly non-zero**,
/// surfacing git's stderr verbatim, never a silent skip behind a success banner.
#[test]
fn migrate_corpus_fails_loudly_when_a_pre_commit_hook_rejects() {
    let repo = TempDir::new("hook-rejects");
    let home = TempDir::new("home");
    setup_repo(repo.path(), home.path());
    commit_v1_changelog(repo.path());
    let base = head(repo.path());

    // A rejecting hook, replacing the warn-only one `jigc setup` installed.
    let hook = repo.path().join(".git").join("hooks").join("pre-commit");
    fs::write(
        &hook,
        "#!/bin/sh\necho 'policy: no corpus commits' >&2\nexit 1\n",
    )
    .expect("write pre-commit hook");
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        fs::set_permissions(&hook, fs::Permissions::from_mode(0o755)).expect("chmod hook");
    }

    let migrate = jigc(repo.path(), home.path(), &["migrate-corpus"]);
    let stdout = String::from_utf8_lossy(&migrate.stdout);
    let stderr = String::from_utf8_lossy(&migrate.stderr);
    assert!(
        !migrate.status.success(),
        "a rejected commit fails the verb loudly; stdout:\n{stdout}\nstderr:\n{stderr}",
    );
    assert!(
        stderr.contains("policy: no corpus commits"),
        "git's (the hook's) own rejection is surfaced verbatim; stderr:\n{stderr}",
    );
    assert_eq!(
        commits_since(repo.path(), &base).len(),
        0,
        "the rejected commit did not land",
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
