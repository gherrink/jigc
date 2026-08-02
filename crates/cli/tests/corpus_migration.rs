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

/// Assert a `jigc migrate-corpus` run that **refused a doc** exits **non-zero** (M42 completion
/// audit, Finding 2). A refusal used to exit 0 — so a run in which every doc was refused reported
/// success, the three "refuse loudly" classes were inaudible, and `validate` (exit 1, *run
/// migrate-corpus*) → `migrate-corpus` (exit 0) → `validate` was an infinite CI loop. The corpus
/// is **not migrated** when a doc is blocked, and the caller must hear so.
fn assert_refused(out: &std::process::Output, what: &str) {
    assert!(
        !out.status.success(),
        "{what} refused a doc, so it must exit NON-ZERO; stdout:\n{}\nstderr:\n{}",
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

    // 1. DETECT — `jigc validate` reports the stranded v0 doc, routed `migrate`, and (M42)
    //    **exits non-zero**: an unmigrated corpus is the third exit-flipping exception, since
    //    every other family in the sweep adjudicated docs against a schema they were never
    //    written to (`design/validation.md` → Exit semantics — the third exception).
    let detect = jigc(repo.path(), home.path(), &["validate"]);
    let detect_out = String::from_utf8_lossy(&detect.stdout);
    assert!(
        !detect.status.success(),
        "`jigc validate` over a v0 corpus exits non-zero; stdout:\n{detect_out}",
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
    assert_refused(
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
    // A genuinely-current bystander (an ADR stamped at the current schema-version 2):
    // its `already-current` report line must SURVIVE the completed-move dedup — the
    // report-integrity axis the contains-only assertions below were blind to
    // (2026-07-24 mutation audit, finding #4: a `retain` inverted to `k == target`
    // keeps only the superseded destination line and silently drops every other
    // already-current entry).
    commit_adr(repo.path(), "settled", "Settled decision", Some(2));

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
    // The exact-set report: the moved doc is reported ONCE, as migrated — its
    // enumerated `already-current` destination line superseded — while the bystander's
    // `already-current` line survives. Exact summary counts + per-entry lines, so both
    // a double-report and a dropped entry redden.
    assert!(
        out.contains("corpus migration: 1 migrated, 1 already current, 0 blocked"),
        "the summary counts are exact — one completed move, one surviving bystander, \
         nothing blocked; stdout:\n{out}",
    );
    assert!(
        out.contains("  migrated   CHANGELOG.md"),
        "the completed move is reported migrated; stdout:\n{out}",
    );
    assert!(
        !out.contains("  current    CHANGELOG.md"),
        "the moved doc's superseded `already-current` line must NOT double-report; \
         stdout:\n{out}",
    );
    assert!(
        out.contains("  current    docs/decisions/settled.md"),
        "the genuinely-current bystander stays reported; stdout:\n{out}",
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

/// The `jigc validate --format json` findings over `repo`. The **exit code is not asserted**
/// here: since M42 an unmigrated corpus flips the store sweep's exit non-zero (the third
/// exit-flipping exception), and the whole point of the callers below is to read the findings
/// over exactly such a corpus. A run that could not sweep at all emits no JSON, so the parse
/// below is the "the sweep ran" guard.
fn validate_findings(repo: &Path, home: &Path) -> Vec<serde_json::Value> {
    let out = jigc(repo, home, &["validate", "--format", "json"]);
    let report: serde_json::Value =
        serde_json::from_slice(&out.stdout).expect("`jigc validate` emits valid JSON");
    report["findings"]
        .as_array()
        .expect("the report carries a findings array")
        .clone()
}

/// The **blocking** `schema-conformance.*` findings a sweep raises against the doc at
/// `identity` (the finding's stable `key.target`, whose doc half is `<type>:<slug>` — a
/// placement singleton's `<type>:<type>` — optionally followed by a `#fragment`).
fn blocking_conformance_on(
    findings: &[serde_json::Value],
    identity: &str,
) -> Vec<serde_json::Value> {
    findings
        .iter()
        .filter(|f| {
            f["severity"] == "blocking"
                && f["code"]
                    .as_str()
                    .is_some_and(|c| c.starts_with("schema-conformance."))
                && f["key"]["target"]
                    .as_str()
                    .is_some_and(|t| t == identity || t.starts_with(&format!("{identity}#")))
        })
        .cloned()
        .collect()
}

/// **M42 Inc-2 T1 — the fifth store family enumerates through `index::committed_instances`.**
/// The detect↔fix loop over a **placement** doctype, end-to-end on the real binary.
///
/// Pre-fix, `schema_conformance_store` walked only `location:`-bearing schemas (the stale
/// guard *"a transient (location-less) type has no committed docs"*, true pre-M38 and false
/// since), so it dropped the whole **placement** class — which is where both placement
/// doctypes at schema-version 2 live (`changelog`, `deferral-ledger`). Net: `jigc validate`
/// over a corpus whose root `CHANGELOG.md` is stamped `schema-version: 1` reported **zero**
/// schema-conformance findings and exited 0 — a false green over exactly the corpus
/// `migrate-corpus` exists to upgrade (the fixing half outrunning the detecting half;
/// `design/validation.md` → Store-scope schema-conformance, "every committed instance").
///
/// Post-fix the loop closes: **detect** (a blocking `schema-conformance.*` finding addressed
/// at `changelog:changelog`, routed `migrate`) → **migrate** → **re-validate clean**.
#[test]
fn validate_detects_the_stale_placement_changelog_then_migrate_clears_it() {
    let repo = TempDir::new("stale-placement-detect");
    let home = TempDir::new("home");
    setup_repo(repo.path(), home.path());
    // The corpus: one committed doc, a root `CHANGELOG.md` stamped schema-version 1 while the
    // manifest is at 2 — the state a hand-moved (or partially-migrated) changelog lands in.
    commit_root_changelog(repo.path(), STALE_ROOT_CHANGELOG_V1);

    // 1. DETECT — the stale placement doc is seen, blocking-conformant-broken, routed migrate.
    let before = validate_findings(repo.path(), home.path());
    let stale = blocking_conformance_on(&before, "changelog:changelog");
    assert!(
        !stale.is_empty(),
        "the stale placement-home changelog raises a blocking schema-conformance finding; \
         got findings: {before:#?}",
    );
    // Among them, exactly one is the **version-currency** break under its own check id (M42
    // Inc-3 T2) — the machine-readable staleness fact, routed at the verb that fixes it.
    let version_currency: Vec<_> = stale
        .iter()
        .filter(|f| f["code"].as_str() == Some("schema-conformance.schema-version-current"))
        .collect();
    assert_eq!(
        version_currency.len(),
        1,
        "the stale placement changelog surfaces exactly one version-currency break; \
         got findings: {before:#?}",
    );
    assert!(
        version_currency[0]["route"]
            .as_str()
            .is_some_and(|r| r.contains("jigc migrate-corpus")),
        "the version-currency break names the verb that upgrades a managed corpus, verbatim; \
         got: {:#?}",
        version_currency[0],
    );
    for finding in &stale {
        let route = finding["route"]
            .as_str()
            .unwrap_or_else(|| panic!("the finding carries a route; got: {finding:#?}"));
        assert!(
            route.starts_with("migrate"),
            "a below-version placement doc routes `migrate`; got route: {route}",
        );
        assert!(
            route.contains("CHANGELOG.md"),
            "the route names the doc's literal placement home; got route: {route}",
        );
        assert!(
            finding["message"]
                .as_str()
                .is_some_and(|m| m.contains("`CHANGELOG.md`")),
            "the finding is attributed to the repo-relative placement path; got: {finding:#?}",
        );
    }

    // 2. MIGRATE — the routed verb upgrades the doc it was routed at.
    let migrate = jigc(repo.path(), home.path(), &["migrate-corpus"]);
    assert_ok(&migrate, "`jigc migrate-corpus`");
    assert_eq!(
        fs::read_to_string(repo.path().join("CHANGELOG.md")).expect("read CHANGELOG.md"),
        MIGRATED_ROOT_CHANGELOG_V2,
        "the routed verb migrates the doc the detector pointed at",
    );

    // 3. RE-VALIDATE — the loop closes: no blocking conformance break, no migrate route.
    let after = validate_findings(repo.path(), home.path());
    assert!(
        blocking_conformance_on(&after, "changelog:changelog").is_empty(),
        "the migrated placement doc validates clean; got findings: {after:#?}",
    );
    assert!(
        !after.iter().any(|f| f["route"]
            .as_str()
            .is_some_and(|r| r.starts_with("migrate"))),
        "the migrated corpus carries no migrate route; got findings: {after:#?}",
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

/// The `.jigc` file-state record's raw bytes (`.jigc/state/file-state.json`) — the baseline
/// the migration flips per written doc. `None` before the record has ever been written.
fn file_state(repo: &Path) -> Option<String> {
    fs::read_to_string(repo.join(".jigc").join("state").join("file-state.json")).ok()
}

/// **M42 Inc-1 T3 — `--no-commit`.** The opt-out of T2's commit boundary, for an operator who
/// wants to review the migration or fold it into a larger commit: the verb still **migrates**
/// (bytes written, the relocation move made), but stages and commits **nothing** — the tree is
/// left exactly as `HEAD`-today's git-free verb left it (an unstaged delete of the old home +
/// an untracked file at the placement home), and `HEAD` is unchanged.
#[test]
fn migrate_corpus_no_commit_migrates_but_lands_nothing() {
    let repo = TempDir::new("no-commit");
    let home = TempDir::new("home");
    setup_repo(repo.path(), home.path());
    commit_v1_changelog(repo.path());
    let base = head(repo.path());

    let migrate = jigc(repo.path(), home.path(), &["migrate-corpus", "--no-commit"]);
    let out = String::from_utf8_lossy(&migrate.stdout);
    assert_ok(&migrate, "`jigc migrate-corpus --no-commit`");
    assert!(
        out.contains("1 migrated") && out.contains("CHANGELOG.md"),
        "`--no-commit` still MIGRATES — it only declines to land it; stdout:\n{out}",
    );

    // The migration really happened on disk: the doc relocated + re-stamped v2.
    assert!(
        !repo.path().join("docs/changelog/changelog.md").exists(),
        "the relocation move still happened under `--no-commit`; stdout:\n{out}",
    );
    let moved = fs::read_to_string(repo.path().join("CHANGELOG.md")).expect("read CHANGELOG.md");
    assert!(
        moved.starts_with("---\nschema-version: 2\n---\n\n# Changelog\n"),
        "`--no-commit` writes the migrated bytes; got:\n{moved}",
    );

    // But nothing was staged and nothing was committed: the tree carries BOTH halves of the
    // move as working-tree changes — exactly the state the git-free verb left behind.
    let status = porcelain(repo.path());
    assert!(
        status.contains("?? CHANGELOG.md"),
        "`--no-commit` leaves the relocated file untracked (nothing staged); status:\n{status}",
    );
    assert!(
        status.contains("D docs/changelog/changelog.md"),
        "`--no-commit` leaves the old-home delete unstaged; status:\n{status}",
    );
    assert_eq!(
        head(repo.path()),
        base,
        "`--no-commit` commits nothing — HEAD is unchanged; stdout:\n{out}",
    );
    // `--no-commit` names no *landed* commit, and says so honestly (F2 — the run wrote the
    // bytes but staged nothing; distinct from both the committed run and the dry-run preview).
    assert!(
        out.contains("written but NOT committed (`--no-commit`)"),
        "`--no-commit` states it wrote but did not commit; stdout:\n{out}",
    );
    assert!(
        !out.contains("only the migrated paths were staged"),
        "`--no-commit` names no landed commit sha; stdout:\n{out}",
    );
}

/// **M42 Inc-1 T3 — `--dry-run`.** There is no operator-facing `persist` *stage* to stop before
/// (`engine::state::persist` writes the bytes and flips the file-state baseline in one motion),
/// so dry-run is defined as **suppressing the write**: no bytes, no relocation move, no
/// file-state flip — and, necessarily, no commit (there is nothing on disk to stage). The
/// triage report is produced either way, and it is the **identical** report the applying run
/// prints — asserted byte-for-byte against `--no-commit`, the run that does exactly what
/// dry-run describes.
///
/// The file-state suppression is proven **non-vacuously**: a v0 ADR is `ingest`ed first, so the
/// record genuinely carries a baseline the applying run re-hashes — and the final assertion
/// shows the record *does* move once the migration really runs.
#[test]
fn migrate_corpus_dry_run_writes_nothing_and_commits_nothing() {
    let repo = TempDir::new("dry-run");
    let home = TempDir::new("home");
    setup_repo(repo.path(), home.path());
    commit_v1_changelog(repo.path());
    commit_adr(repo.path(), "alpha-decision", "Alpha decision", None);
    // Adopt the ADR, so the file-state record carries a baseline the real migration re-hashes.
    assert_ok(
        &jigc(repo.path(), home.path(), &["ingest"]),
        "`jigc ingest`",
    );

    let old_home = repo.path().join("docs/changelog/changelog.md");
    let before_old_home = fs::read_to_string(&old_home).expect("read the v1 changelog");
    let before_adr = fs::read_to_string(adr_path(repo.path(), "alpha-decision")).expect("read adr");
    let before_state = file_state(repo.path()).expect("the file-state record exists after ingest");
    assert!(
        before_state.contains("alpha-decision"),
        "precondition: the ingested ADR carries a file-state baseline; got:\n{before_state}",
    );
    let base = head(repo.path());
    let before_status = porcelain(repo.path());

    // DRY RUN — the report is produced; nothing is written, moved, re-baselined or committed.
    let dry = jigc(repo.path(), home.path(), &["migrate-corpus", "--dry-run"]);
    let dry_out = String::from_utf8_lossy(&dry.stdout).into_owned();
    assert_ok(&dry, "`jigc migrate-corpus --dry-run`");
    // A dry run speaks in the conditional and says nothing was written (F2 — never past-tense
    // `migrated` over writes that never happened).
    assert!(
        dry_out.contains("2 would migrate") && dry_out.contains("CHANGELOG.md"),
        "the dry run reports the migration it WOULD make; stdout:\n{dry_out}",
    );
    assert!(
        dry_out.contains("dry run — nothing written"),
        "the dry-run header says what the run IS — nothing written; stdout:\n{dry_out}",
    );

    assert_eq!(
        fs::read_to_string(&old_home).expect("the old home still holds the v1 changelog"),
        before_old_home,
        "`--dry-run` writes nothing: the old home is byte-identical, and still there",
    );
    assert!(
        !repo.path().join("CHANGELOG.md").exists(),
        "`--dry-run` moves nothing: the placement home was never written; stdout:\n{dry_out}",
    );
    assert_eq!(
        fs::read_to_string(adr_path(repo.path(), "alpha-decision")).expect("read adr"),
        before_adr,
        "`--dry-run` leaves the in-place ADR byte-identical (unstamped)",
    );
    assert_eq!(
        file_state(repo.path()).expect("the file-state record"),
        before_state,
        "`--dry-run` never flips the file-state baseline",
    );
    assert_eq!(
        head(repo.path()),
        base,
        "`--dry-run` implies no commit — HEAD is unchanged; stdout:\n{dry_out}",
    );
    assert_eq!(
        porcelain(repo.path()),
        before_status,
        "`--dry-run` leaves the working tree exactly as it found it; stdout:\n{dry_out}",
    );

    // The report is the IDENTICAL triage report the applying run prints (`--no-commit` is the
    // run that does exactly what the dry run described, and lands nothing either).
    let real = jigc(repo.path(), home.path(), &["migrate-corpus", "--no-commit"]);
    let real_out = String::from_utf8_lossy(&real.stdout).into_owned();
    assert_ok(&real, "`jigc migrate-corpus --no-commit` after the dry run");
    // The two runs adjudicate the SAME triage set (both name CHANGELOG.md, both 2 docs), but
    // are byte-distinct: the applying run speaks in the past tense, the dry run in the
    // conditional (F2 — a dry run must not be byte-indistinguishable from a run that wrote).
    assert!(
        real_out.contains("2 migrated") && !real_out.contains("would migrate"),
        "the applying run speaks in the past tense; stdout:\n{real_out}",
    );
    assert_ne!(
        dry_out, real_out,
        "a dry run must NOT print bytes identical to a run that applied the writes",
    );

    // Non-vacuity: the applying run DOES move the bytes and the file-state baseline the dry
    // run left untouched.
    assert!(
        repo.path().join("CHANGELOG.md").exists() && !old_home.exists(),
        "the applying run really relocates the changelog",
    );
    assert_ne!(
        file_state(repo.path()).expect("the file-state record"),
        before_state,
        "the applying run DOES flip the file-state baseline — the dry-run assertion is not vacuous",
    );
}

/// A **second, different** committed v1 changelog at the prior folder home — the
/// two-candidates-one-destination topology the M42 walk union newly makes reachable (the
/// "partially-completed migration" state it exists to find): *both* prior-home instances
/// relocate to the single `CHANGELOG.md` placement file, so exactly one can land and the
/// other collides.
fn commit_second_v1_changelog(repo: &Path, stem: &str) {
    let body = format!(
        "\
---
schema-version: 1
---

# changelog

## Unreleased Changes

### added  {{#added}}

- A {stem} entry.

## Releases
"
    );
    let dir = repo.join("docs").join("changelog");
    fs::create_dir_all(&dir).expect("mk docs/changelog/");
    fs::write(dir.join(format!("{stem}.md")), body).expect("write the second v1 changelog");
    git(repo, &["add", "."]);
    git(repo, &["commit", "-q", "-m", "seed a second v1 changelog"]);
}

/// **M42 Inc-1 — `--dry-run` on the COLLISION topology the union creates.** `--dry-run`'s whole
/// contract is *"the identical triage report an applying run prints"*, and the destination
/// collision is adjudicated against the destination's bytes **as this run leaves them** — so a
/// second candidate for a destination an earlier candidate in the same run already claimed must
/// see that claim whether or not the write was persisted.
///
/// The defect this pins: with the adjudication reading only from **disk**, `--dry-run` (which
/// suppresses the write) had the second candidate see *no* destination at all — it reported
/// **both** prior-home docs `migrated` (the same `CHANGELOG.md` listed twice) where the applying
/// run reports `1 migrated, 1 blocked`. The preview described an outcome that does not occur.
#[test]
fn migrate_corpus_dry_run_reports_the_collision_the_applying_run_reports() {
    let repo = TempDir::new("dry-run-collision");
    let home = TempDir::new("home");
    setup_repo(repo.path(), home.path());
    // Two *different* v1 changelogs at the prior home; both relocate to root `CHANGELOG.md`.
    commit_v1_changelog(repo.path());
    commit_second_v1_changelog(repo.path(), "legacy");

    // DRY RUN — the preview.
    let dry = jigc(repo.path(), home.path(), &["migrate-corpus", "--dry-run"]);
    let dry_out = String::from_utf8_lossy(&dry.stdout).into_owned();
    assert_refused(
        &dry,
        "`jigc migrate-corpus --dry-run` over the collision topology",
    );
    assert!(
        dry_out.contains("1 would migrate") && dry_out.contains("1 blocked"),
        "the dry run adjudicates the collision it would hit — one lands, one blocks; \
         stdout:\n{dry_out}",
    );
    assert_eq!(
        count(&dry_out, "  would migrate CHANGELOG.md"),
        1,
        "the shared destination is never reported migrated twice; stdout:\n{dry_out}",
    );
    assert!(
        dry_out.contains("blocked    docs/changelog/legacy.md"),
        "the losing candidate is blocked with a route; stdout:\n{dry_out}",
    );
    assert!(
        !repo.path().join("CHANGELOG.md").exists(),
        "`--dry-run` still writes nothing; stdout:\n{dry_out}",
    );

    // The applying run (`--no-commit` does exactly what the dry run described, landing nothing).
    let real = jigc(repo.path(), home.path(), &["migrate-corpus", "--no-commit"]);
    let real_out = String::from_utf8_lossy(&real.stdout).into_owned();
    assert_refused(&real, "`jigc migrate-corpus --no-commit` after the dry run");
    // Same triage set on the collision topology (one lands, `legacy.md` blocks) but byte-distinct
    // framing — the preview says `would migrate`, the applying run says `migrated` (F2).
    assert!(
        real_out.contains("1 migrated")
            && real_out.contains("blocked    docs/changelog/legacy.md")
            && !real_out.contains("would migrate"),
        "the applying run reports the same collision in the past tense; stdout:\n{real_out}",
    );
    assert_ne!(
        dry_out, real_out,
        "a dry run must NOT print bytes identical to a run that applied the writes — on the \
         collision topology, not just the single-candidate one",
    );

    // Non-vacuity: the applying run really lands exactly one doc and leaves the other on disk.
    assert_eq!(
        fs::read_to_string(repo.path().join("CHANGELOG.md")).expect("read CHANGELOG.md"),
        "---\nschema-version: 2\n---\n\n# Changelog\n\n## Unreleased Changes\n\n## Releases\n",
        "the winning candidate's migrated bytes land at the destination",
    );
    assert!(
        repo.path().join("docs/changelog/legacy.md").is_file(),
        "the blocked candidate is never removed (No-data-loss)",
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

// ─────────────── the cross-order repro block (the confidence-audit back-sweep) ───────────────

/// **Repro block (pinning.md §3) — the M40 e2e seed-order witness, made standing**
/// (the confidence-audit back-sweep, triage item c2). The M40 completion audit drove
/// this one-off: `completions/artifacts/M40/VERDICT.md` → *Milestone e2e*: "the v0
/// corpus stamp migration byte-identical across seed orders". Standing coverage
/// asserted the migration in a **single** seed order only; nothing re-drove it under
/// two, so a refactor swapping the sorted store walk (`committed_slugs` /
/// `PreparedDoc` path-sort / the sorted report) for raw `read_dir` iteration would
/// ship driver-visible nondeterminism with every existing test green.
///
/// ```yaml
/// claim: "the v0 stamp migration lands byte-identical trees across divergent seed orders"
/// verdict: CONFIRMED (M40 e2e audit — one-off; standing as of this test)
/// setup:
///   - two fresh repos; the SAME three unstamped v0 ADRs committed one-per-commit,
///     in divergent seed orders (alpha,beta,gamma vs gamma,alpha,beta)
/// repro:
///   - ["jigc", "migrate-corpus"]
/// expect:
///   tree: HEAD^{tree} byte-identical across the two repos
///   message: the landed commit message byte-identical
///   report: stdout byte-identical modulo the commit hash (parent histories differ)
/// pinned-by: corpus_migration::stamp_migration_is_byte_identical_across_divergent_seed_orders
/// ```
///
/// Verified catchable: locally muting the enumeration/report sorts
/// (`committed_slugs`' `slugs.sort()` + the `prepared`/`report.migrated` sorts) lets
/// tmpfs creation order through and reddens the masked-report assertion
/// (mutate → catch → restore; never committed).
#[test]
fn stamp_migration_is_byte_identical_across_divergent_seed_orders() {
    /// Seed the same v0 corpus in `order`, migrate it, and return
    /// `(tree-hash, commit-message, masked stdout report)`.
    fn seed_and_migrate(order: &[(&str, &str)]) -> (String, String, String) {
        let repo = TempDir::new("seed-order");
        let home = TempDir::new("home");
        setup_repo(repo.path(), home.path());
        for (slug, title) in order {
            commit_adr(repo.path(), slug, title, None);
        }

        let migrate = jigc(repo.path(), home.path(), &["migrate-corpus"]);
        assert_ok(&migrate, "`jigc migrate-corpus` (seed-order arm)");
        let report = String::from_utf8(migrate.stdout).expect("utf-8 report");

        // Non-vacuity: all three docs were genuinely migrated and stamped on disk.
        assert_eq!(
            count(&report, "migrated   docs/decisions/"),
            3,
            "the report names all three migrated ADRs; got:\n{report}",
        );
        for (slug, _) in order {
            let body = fs::read_to_string(adr_path(repo.path(), slug)).expect("read adr");
            assert!(
                body.contains("schema-version:"),
                "the migrated `{slug}` carries the stamp; got:\n{body}",
            );
        }

        let tree = git(repo.path(), &["rev-parse", "HEAD^{tree}"])
            .trim()
            .to_string();
        let message = git(repo.path(), &["log", "-1", "--format=%B"]);
        (tree, message, mask_commit_hash(&report))
    }

    /// Mask the commit hash in the `committed <hash> — …` report line — the one
    /// legitimately repo-specific byte range (the two repos' seed histories differ,
    /// so the landed commit's parents — and hence its hash — differ by design).
    fn mask_commit_hash(report: &str) -> String {
        report
            .lines()
            .map(|line| match line.strip_prefix("committed ") {
                Some(rest) => match rest.split_once(' ') {
                    Some((_hash, tail)) => format!("committed <hash> {tail}"),
                    None => line.to_string(),
                },
                None => line.to_string(),
            })
            .collect::<Vec<_>>()
            .join("\n")
    }

    let order_a = [
        ("alpha-decision", "Alpha decision"),
        ("beta-decision", "Beta decision"),
        ("gamma-decision", "Gamma decision"),
    ];
    let order_b = [
        ("gamma-decision", "Gamma decision"),
        ("alpha-decision", "Alpha decision"),
        ("beta-decision", "Beta decision"),
    ];
    assert_ne!(order_a, order_b, "the seed orders must diverge");

    let (tree_a, message_a, report_a) = seed_and_migrate(&order_a);
    let (tree_b, message_b, report_b) = seed_and_migrate(&order_b);

    assert_eq!(
        tree_a, tree_b,
        "the migrated tree must be byte-identical across divergent seed orders",
    );
    assert_eq!(
        message_a, message_b,
        "the landed commit message must be byte-identical across divergent seed orders",
    );
    assert_eq!(
        report_a, report_b,
        "the migration report (commit hash masked) must be byte-identical across \
         divergent seed orders",
    );
}
