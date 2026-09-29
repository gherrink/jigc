//! Acceptance — **version-mismatch is itself a surfaced break** for the store-scope
//! schema-conformance detector (M34 Increment 3, the Inc-3 T4 build-halt resolution).
//!
//! The T3 routing path *labels* `schema-conformance.*` findings the detector already
//! produced — but a committed persisted doc whose only problem is an absent / below-current
//! schema-version stamp is **otherwise fully conformant**, so a labeler-only path produces no
//! finding to annotate and `jigc validate` stays silent (exit 0). That silent case is the
//! M34 dogfood's own headline: a **pure-stamp v0 corpus** carries no *other* non-conformance
//! (`design/validation.md` → Version-mismatch is itself a surfaced break; DECISIONS
//! 2026-06-25 → M34 Inc-3 T4 build halt resolved).
//!
//! This drives the **built `jigc` binary** through the real store-scope `jigc validate` over a
//! committed corpus with **no schema shadow** (the bytes an operator actually sees), and
//! asserts:
//!
//! - an **unstamped (v0)**, otherwise-conformant ADR is **reported** with a
//!   `schema-conformance.schema-version-current` finding routed at `jigc migrate-corpus`, and
//!   `jigc validate` **exits non-zero** — the M42 Increment-4 exit flip, keyed on that code
//!   (`design/validation.md` → Exit semantics — the third exception): an unmigrated corpus means
//!   every other family adjudicated docs against a schema they were never written to, so the
//!   sweep could not produce a trustworthy result. *(Until M42 this arm asserted exit **0** — the
//!   false green a CI pipeline would have bound as "unmigrated == green".)*
//! - a **current-stamped** store surfaces **no** version finding and exits 0 (the
//!   false-positive guard — and the proof the flip is keyed on the version break, not on the
//!   sweep running at all).
//!
//! **The check id is pinned here (M42).** The M34 build wrote the break as a *reuse* of
//! `schema-conformance.field-value-conformant` over the stamp field; M42 retracts that — a stale
//! stamp and an invalid enum value are different facts, and the reuse makes the version break
//! indistinguishable to any machine consumer (`design/validation.md` → the retraction). The
//! headline detect case below is unchanged in *behaviour*; what it now pins is **which fact** the
//! report states.

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

/// A throwaway directory that removes itself on drop.
struct TempDir(PathBuf);

impl TempDir {
    fn new(tag: &str) -> Self {
        let mut path = std::env::temp_dir();
        path.push(format!(
            "jigc-version-mismatch-{tag}-{}-{:?}",
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
        .env_remove("JIGC_DOC_CODE_PROBE")
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
/// `schema-version` stamp line in its header (`None` = the unstamped v0 state). The prose is
/// conformant under the real (un-shadowed) schema — the *only* signal under test is the stamp.
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

/// Count non-overlapping occurrences of `needle` in `haystack`.
fn count(haystack: &str, needle: &str) -> usize {
    haystack.matches(needle).count()
}

/// (the headline case) An ingested + baselined v0 corpus ADR (**no** schema-version stamp,
/// **no** schema shadow, otherwise conformant) is *reported* by `jigc validate` with a
/// `schema-conformance.schema-version-current` break routed at `jigc migrate-corpus` — emission,
/// not annotation — and (M42 Increment 4) the sweep **exits non-zero**. This is the case a
/// labeler-only path leaves silent (its own dogfood).
#[test]
fn store_sweep_reports_unstamped_v0_doc_as_migrate_and_exits_nonzero() {
    let repo = TempDir::new("v0");
    let home = TempDir::new("home");
    setup_repo(repo.path(), home.path());

    // The v0 corpus state: a committed ADR with NO schema-version stamp, fully conformant
    // under the un-shadowed pack schema.
    commit_adr(repo.path(), "alpha-decision", "Alpha decision", None);

    let out = jigc(repo.path(), home.path(), &["validate"]);
    let stdout = String::from_utf8_lossy(&out.stdout);
    let stderr = String::from_utf8_lossy(&out.stderr);

    // M42: the version break is the **third exit-flipping exception** — an unmigrated corpus
    // makes the whole sweep untrustworthy, so it exits non-zero. (Ordinary content findings
    // stay report-only / exit 0 — pinned in `managed_vs_foreign.rs`.)
    assert!(
        !out.status.success(),
        "an unmigrated corpus flips `jigc validate`'s exit non-zero; \
         stdout:\n{stdout}\nstderr:\n{stderr}",
    );

    // The unstamped doc surfaces exactly one schema-conformance break (the version break),
    // emitted even though the doc is otherwise structurally clean — and it is the version-
    // currency code, not a reused value-conformance one (M42: the fact must be distinguishable).
    assert_eq!(
        count(&stdout, "schema-conformance."),
        1,
        "an unstamped v0 ADR must surface exactly one schema-conformance break; \
         stdout:\n{stdout}",
    );
    assert_eq!(
        count(&stdout, "schema-conformance.schema-version-current"),
        1,
        "the v0 break is the version-currency break, under its own check id; stdout:\n{stdout}",
    );
    // And it routes `migrate`, naming the verb that fixes it — the emitted bytes an operator
    // reads.
    assert_eq!(
        count(&stdout, "route: migrate"),
        1,
        "the unstamped (v0) ADR's version break must route `migrate`; stdout:\n{stdout}",
    );
    assert_eq!(
        count(&stdout, "jigc migrate-corpus"),
        2,
        "the verb that upgrades a managed corpus is named verbatim twice — once on the finding's \
         route, once in the M42 exit-flipping trailer that explains the non-zero exit; \
         stdout:\n{stdout}",
    );
}

/// (the above-current arm — the confidence-audit sibling-hunt item 1, 2026-07-24) An
/// otherwise-conformant ADR stamped **above** its doctype's manifest version — an OOB-planted
/// or foreign-future stamp — is a *different fact* from a stale one (the M42 retraction
/// rationale: a machine consumer must be able to tell them apart, because a stale stamp is
/// fixed by `jigc migrate-corpus` and a future stamp **cannot** be), so it surfaces under its
/// **own** check id `schema-conformance.schema-version-ahead`, with a Human-shaped route (no
/// mechanical fix exists: `set-field` refuses the machine-maintained stamp — upgrade jigc, or
/// restore the stamp from git history), and the sweep **exits non-zero** on the same
/// untrustworthy-sweep criterion as the below-version break (the doc was written to a schema
/// this binary does not know). Before the fix this case was **silent**: no finding, exit 0 —
/// the fixed failure through an unfixed door.
///
/// And `jigc migrate-corpus` must **never** report it `already-current` (the permanent silent
/// migrate-skip): it blocks the doc with a route, mirroring the missing-snapshot arm.
#[test]
fn store_sweep_reports_above_current_stamp_and_migrate_corpus_blocks_it() {
    let repo = TempDir::new("ahead");
    let home = TempDir::new("home");
    setup_repo(repo.path(), home.path());

    // An otherwise-conformant committed ADR stamped far above the current manifest version.
    commit_adr(repo.path(), "alpha-decision", "Alpha decision", Some(99));

    let out = jigc(repo.path(), home.path(), &["validate"]);
    let stdout = String::from_utf8_lossy(&out.stdout);
    let stderr = String::from_utf8_lossy(&out.stderr);

    // The above-current stamp meets the same exit-flipping criterion as the below-version
    // break: the doc was written to a schema this binary does not know, so the sweep could
    // not adjudicate it.
    assert!(
        !out.status.success(),
        "an above-current stamp flips `jigc validate`'s exit non-zero; \
         stdout:\n{stdout}\nstderr:\n{stderr}",
    );
    // Exactly one break, under the AHEAD code — never the below-version code (the two facts
    // must stay distinguishable to a machine consumer keying on the code).
    assert_eq!(
        count(&stdout, "schema-conformance.schema-version-ahead"),
        1,
        "an above-current ADR must surface exactly one version-ahead break; stdout:\n{stdout}",
    );
    assert_eq!(
        count(&stdout, "schema-conformance.schema-version-current"),
        0,
        "a future stamp is not a stale stamp — the below-version code must not fire; \
         stdout:\n{stdout}",
    );
    // The route is honest and Human-shaped: no verb fixes a future stamp, so it names the
    // two real repairs — upgrade jigc, or restore the stamp from git history.
    assert!(
        stdout.contains("upgrade jigc") && stdout.contains("git history"),
        "the ahead break's route must name the two human repairs (upgrade jigc / restore \
         from git history); stdout:\n{stdout}",
    );

    // `jigc migrate-corpus` must not silently skip the doc as already-current: it blocks it
    // with a route (the missing-snapshot precedent — never a silent skip).
    let out = jigc(repo.path(), home.path(), &["migrate-corpus", "--dry-run"]);
    let stdout = String::from_utf8_lossy(&out.stdout);
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert!(
        !out.status.success(),
        "an above-current doc blocks the corpus migration (exit non-zero); \
         stdout:\n{stdout}\nstderr:\n{stderr}",
    );
    assert_eq!(
        count(&stdout, "current    docs/decisions/alpha-decision.md"),
        0,
        "an above-current doc must NOT report `already current`; stdout:\n{stdout}",
    );
    assert_eq!(
        count(&stdout, "blocked    docs/decisions/alpha-decision.md"),
        1,
        "the above-current doc is blocked, with its path named; stdout:\n{stdout}",
    );
    assert_eq!(
        count(&stdout, "migrate-corpus.schema-version-ahead"),
        1,
        "the refusal carries its own stable code; stdout:\n{stdout}",
    );
}

/// (the false-positive guard) A current-stamped store surfaces **no** version finding, no
/// route line, and exits 0 — a doc stamped at the manifest version is clean.
#[test]
fn store_sweep_clean_on_current_stamped_doc() {
    let repo = TempDir::new("current");
    let home = TempDir::new("home");
    setup_repo(repo.path(), home.path());
    commit_adr(repo.path(), "alpha-decision", "Alpha decision", Some(2));

    let out = jigc(repo.path(), home.path(), &["validate"]);
    let stdout = String::from_utf8_lossy(&out.stdout);
    let stderr = String::from_utf8_lossy(&out.stderr);

    assert!(
        out.status.success(),
        "a current-stamped store must exit 0; stdout:\n{stdout}\nstderr:\n{stderr}",
    );
    assert!(
        !stdout.contains("schema-conformance"),
        "a current-stamped, conformant committed ADR must surface NO schema-conformance \
         finding; stdout:\n{stdout}",
    );
    assert_eq!(
        count(&stdout, "route: migrate"),
        0,
        "a current-stamped store must carry no migrate route line; stdout:\n{stdout}",
    );
}
