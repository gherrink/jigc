//! Acceptance spine (M9 Increment 3, T1) — `jigc ingest` discover → classify →
//! adopt → triage report, end-to-end through the built binary. Adopt is
//! **register-only**: it records conformant candidates into the edge index +
//! file-state baseline but never moves or rewrites a candidate file.
//!
//! Drives the real `jigc` binary against a throwaway git repo seeded with the four
//! flow-12 candidates (after `jigc setup`):
//!
//!   decisions/rate-limit.md   — a CONFORMANT adr at its `decisions/` location → adoptable
//!   decisions/auth-choice.md  — a NON-CONFORMANT near-miss in `decisions/`     → needs-reconcile
//!   docs/old-adr.md           — a CONFORMANT adr at the WRONG location (`docs/`) → needs-reconcile
//!   docs/notes.md             — freeform, outside every location dir            → unmanaged
//!
//! and asserts: (a) all four candidates listed in sorted candidate order with the
//! verdicts adoptable / needs-reconcile / needs-reconcile / unmanaged, and (b) at
//! least one `needs-reconcile` row renders a routed finding — blocking severity,
//! a located message, and a route. Adopt is register-only: the candidate files are
//! byte-unchanged on disk after the scan (the adopt touches only the edge index +
//! file-state baseline, never a candidate file).
//!
//! See `design/worked-examples.md` → flow 12; `design/project-setup.md` → Flow 2.
//!
//! No external test crates: the binary path comes from `CARGO_BIN_EXE_jigc`, the
//! temp repo is a real `git init`, and a self-cleaning `TempDir` keeps the test off
//! the developer's repo.

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

/// A throwaway directory that removes itself on drop.
struct TempDir(PathBuf);

impl TempDir {
    fn new(tag: &str) -> Self {
        let mut path = std::env::temp_dir();
        let unique = format!(
            "jigc-ingest-{tag}-{}-{:?}",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos(),
        );
        path.push(unique);
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

/// Initialize a real git repo with one commit + the `.jigc/config/` project layer.
fn init_repo(repo: &Path) {
    git(repo, &["init", "-q"]);
    git(repo, &["config", "user.email", "test@example.com"]);
    git(repo, &["config", "user.name", "Test"]);
    fs::write(repo.join("README.md"), "hello\n").expect("write file");
    git(repo, &["add", "."]);
    git(repo, &["commit", "-q", "-m", "initial"]);
    fs::create_dir_all(repo.join(".jigc").join("config")).expect("create project layer");
}

/// Run `jigc <args>` with `cwd = repo`, `$HOME = home`, capturing output.
fn jigc(repo: &Path, home: &Path, args: &[&str]) -> std::process::Output {
    Command::new(env!("CARGO_BIN_EXE_jigc"))
        .args(args)
        .current_dir(repo)
        .env("HOME", home)
        .output()
        .expect("run the jigc binary")
}

/// Write a candidate file at `rel`, creating parent dirs as needed.
fn write(repo: &Path, rel: &str, body: &str) {
    let path = repo.join(rel);
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).expect("mk parent dir");
    }
    fs::write(path, body).expect("write candidate");
}

/// A conformant ADR body — the human-editable bytes the committed-store fixtures
/// use (front-matter `status`/`date` + the optional `supersedes` ref, the three
/// required prose sections). The `supersedes` edge is what the adopt path must
/// populate in the index on adoption.
const CONFORMANT_ADR: &str = "\
---
status: accepted
date: 2026-05-23
supersedes: adr:naive-throttle
---

# Rate limiting

## Context
The gateway must shed load under burst traffic.

## Decision
A token bucket per client keeps the gateway fair under burst.

## Consequences
A misbehaving client is throttled, not the whole gateway.
";

/// A near-miss freeform doc dropped into `decisions/`: no front-matter and none of
/// the required ADR sections, so it fails parse/conformance against `adr`.
const NON_CONFORMANT_NEAR_MISS: &str = "\
# Auth choice

A few loose thoughts that are not an ADR at all.
";

#[test]
fn ingest_classifies_the_four_candidates_in_sorted_order_with_routed_findings() {
    let repo = TempDir::new("repo");
    let home = TempDir::new("home");
    init_repo(repo.path());

    // ── on-ramp: wire jigc into the fresh repo ───────────────────────────────────
    let out = jigc(repo.path(), home.path(), &["setup"]);
    assert!(
        out.status.success(),
        "`jigc setup` must succeed; stderr:\n{}",
        String::from_utf8_lossy(&out.stderr),
    );

    // ── seed the four flow-12 candidates ─────────────────────────────────────────
    write(repo.path(), "decisions/rate-limit.md", CONFORMANT_ADR); // adoptable
    write(
        repo.path(),
        "decisions/auth-choice.md",
        NON_CONFORMANT_NEAR_MISS,
    ); // needs-reconcile (near-miss)
    write(repo.path(), "docs/old-adr.md", CONFORMANT_ADR); // needs-reconcile (wrong location)
    write(repo.path(), "docs/notes.md", NON_CONFORMANT_NEAR_MISS); // unmanaged

    // Snapshot the candidate bytes — adopt is register-only: no candidate file is rewritten.
    let before: Vec<(String, Vec<u8>)> = [
        "decisions/rate-limit.md",
        "decisions/auth-choice.md",
        "docs/old-adr.md",
        "docs/notes.md",
    ]
    .iter()
    .map(|rel| {
        (
            rel.to_string(),
            fs::read(repo.path().join(rel)).expect("read"),
        )
    })
    .collect();

    // ── run the scan ─────────────────────────────────────────────────────────────
    let out = jigc(repo.path(), home.path(), &["ingest"]);
    assert!(
        out.status.success(),
        "`jigc ingest` must succeed; stdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr),
    );
    let report = String::from_utf8(out.stdout).expect("utf-8");

    // (a) All four candidates appear in sorted candidate order, each with its verdict.
    // README.md (the initial commit) is also a root candidate — it must be present and
    // classified unmanaged, but the four flow-12 candidates must read in sorted order
    // with the expected verdicts.
    let verdict_for = |rel: &str| -> &str {
        let line = report
            .lines()
            .find(|l| l.contains(rel))
            .unwrap_or_else(|| panic!("`{rel}` must appear in the report:\n{report}"));
        if line.contains("adoptable") {
            "adoptable"
        } else if line.contains("needs-reconcile") {
            "needs-reconcile"
        } else if line.contains("unmanaged") {
            "unmanaged"
        } else {
            panic!("`{rel}` row carries no recognized verdict:\n{line}")
        }
    };
    assert_eq!(
        verdict_for("decisions/rate-limit.md"),
        "adoptable",
        "report:\n{report}"
    );
    assert_eq!(
        verdict_for("decisions/auth-choice.md"),
        "needs-reconcile",
        "report:\n{report}"
    );
    assert_eq!(
        verdict_for("docs/old-adr.md"),
        "needs-reconcile",
        "report:\n{report}"
    );
    assert_eq!(
        verdict_for("docs/notes.md"),
        "unmanaged",
        "report:\n{report}"
    );

    // Sorted candidate order: the four flow-12 rows appear in lexicographic path order.
    let pos = |rel: &str| {
        report
            .find(rel)
            .unwrap_or_else(|| panic!("`{rel}` in report:\n{report}"))
    };
    assert!(
        pos("decisions/auth-choice.md") < pos("decisions/rate-limit.md"),
        "rows must be in sorted candidate order:\n{report}"
    );
    assert!(
        pos("decisions/rate-limit.md") < pos("docs/notes.md"),
        "rows must be in sorted candidate order:\n{report}"
    );
    assert!(
        pos("docs/notes.md") < pos("docs/old-adr.md"),
        "rows must be in sorted candidate order:\n{report}"
    );

    // (b) At least one needs-reconcile row carries a routed finding — blocking
    // severity, a located message, and a route.
    assert!(
        report.contains("blocking"),
        "a needs-reconcile row must render a blocking finding:\n{report}"
    );
    assert!(
        report.contains("route:") || report.contains("route"),
        "a needs-reconcile row must render its route:\n{report}"
    );

    // Register-only adopt: the candidate bytes are unchanged on disk (adopt records
    // into the edge index + file-state baseline; it never moves or rewrites a candidate).
    for (rel, bytes) in &before {
        let after = fs::read(repo.path().join(rel)).expect("re-read");
        assert_eq!(
            *bytes, after,
            "ingest adopt must be register-only — it must not rewrite `{rel}`"
        );
    }
}

#[test]
fn ingest_adopts_only_the_conformant_at_location_doc_and_persists_register_only() {
    let repo = TempDir::new("adopt");
    let home = TempDir::new("home");
    init_repo(repo.path());

    // ── on-ramp: wire jigc into the fresh repo ───────────────────────────────────
    let out = jigc(repo.path(), home.path(), &["setup"]);
    assert!(
        out.status.success(),
        "`jigc setup` must succeed; stderr:\n{}",
        String::from_utf8_lossy(&out.stderr),
    );

    // ── seed the four flow-12 candidates ─────────────────────────────────────────
    write(repo.path(), "decisions/rate-limit.md", CONFORMANT_ADR); // adoptable
    write(
        repo.path(),
        "decisions/auth-choice.md",
        NON_CONFORMANT_NEAR_MISS,
    ); // needs-reconcile (near-miss, under a location dir)
    write(repo.path(), "docs/old-adr.md", CONFORMANT_ADR); // needs-reconcile (wrong location)
    write(repo.path(), "docs/notes.md", NON_CONFORMANT_NEAR_MISS); // unmanaged

    // Snapshot every candidate's bytes — adopt is register-only: nothing rewritten.
    let candidates = [
        "decisions/rate-limit.md",
        "decisions/auth-choice.md",
        "docs/old-adr.md",
        "docs/notes.md",
    ];
    let before: Vec<(String, Vec<u8>)> = candidates
        .iter()
        .map(|rel| {
            (
                rel.to_string(),
                fs::read(repo.path().join(rel)).expect("read"),
            )
        })
        .collect();

    // ── run the scan (adopt path) ────────────────────────────────────────────────
    let out = jigc(repo.path(), home.path(), &["ingest"]);
    assert!(
        out.status.success(),
        "`jigc ingest` must succeed; stdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr),
    );
    let report = String::from_utf8(out.stdout).expect("utf-8");

    // The adoptable row renders the adopt-confirmation marker; the routed/unmanaged
    // rows do not (only the schema-checked, at-location doc is adopted).
    let row = |rel: &str| -> &str {
        report
            .lines()
            .find(|l| l.contains(rel) && !l.trim_start().starts_with("route:"))
            .unwrap_or_else(|| panic!("`{rel}` row must appear in the report:\n{report}"))
    };
    assert!(
        row("decisions/rate-limit.md").contains("adopted"),
        "the conformant-at-location row renders the adopt-confirmation marker:\n{report}",
    );
    assert!(
        !report.contains("decisions/auth-choice.md")
            || !row("decisions/auth-choice.md").contains("adopted"),
        "the near-miss row must NOT render an adopt-confirmation:\n{report}",
    );
    assert!(
        !row("docs/old-adr.md").contains("adopted"),
        "the wrong-location row must NOT render an adopt-confirmation:\n{report}",
    );
    assert!(
        !row("docs/notes.md").contains("adopted"),
        "the unmanaged row must NOT render an adopt-confirmation:\n{report}",
    );

    // ── (a) the conformant-at-location adr adopts: index + baseline gain it ───────
    let edges = fs::read_to_string(repo.path().join(".jigc/index/edges.json"))
        .expect("the edge index is persisted after an adopt");
    assert!(
        edges.contains("\"from\": \"adr:rate-limit\"")
            && edges.contains("\"relation\": \"supersedes\"")
            && edges.contains("\"to\": \"adr:naive-throttle\""),
        "the adopted adr's supersedes forward edge must be persisted in the index:\n{edges}",
    );

    let baseline = fs::read_to_string(repo.path().join(".jigc/state/file-state.json"))
        .expect("the file-state record is persisted after an adopt");
    assert!(
        baseline.contains("\"decisions/rate-limit.md\""),
        "the adopted adr's rel-path must gain a file-state baseline:\n{baseline}",
    );

    // ── (b) the two needs-reconcile docs + the unmanaged doc are NOT adopted ──────
    // Absent from both the index (no forward edge) and the baseline (no hash).
    for rel in [
        "decisions/auth-choice.md",
        "docs/old-adr.md",
        "docs/notes.md",
    ] {
        assert!(
            !baseline.contains(rel),
            "`{rel}` must NOT gain a file-state baseline (not adopted):\n{baseline}",
        );
    }
    // The wrong-location doc shares the conformant bytes (slug `old-adr`); its edge
    // must be absent — only the at-location `rate-limit` was registered.
    assert!(
        !edges.contains("\"from\": \"adr:old-adr\""),
        "the wrong-location adr must NOT enter the index (not adopted):\n{edges}",
    );
    assert!(
        !edges.contains("\"from\": \"adr:auth-choice\"")
            && !edges.contains("\"from\": \"adr:notes\""),
        "no routed/unmanaged candidate enters the index:\n{edges}",
    );

    // ── (c) register-only: every candidate's bytes are byte-identical, no move ────
    for (rel, bytes) in &before {
        let after = fs::read(repo.path().join(rel)).expect("re-read");
        assert_eq!(
            *bytes, after,
            "adopt must not rewrite `{rel}` (register-only)"
        );
    }
    // No file was relocated/duplicated: each candidate's directory holds the same set.
    let decisions: Vec<_> = fs::read_dir(repo.path().join("decisions"))
        .expect("read decisions/")
        .flatten()
        .map(|e| e.file_name())
        .collect();
    assert_eq!(
        decisions.len(),
        2,
        "adopt moves/creates no file in decisions/: {decisions:?}",
    );

    // ── (d) nothing adopted without a schema check: the non-conformant near-miss ──
    // under a location dir (`decisions/auth-choice.md`) is never adopted (the hole
    // closed end-to-end) — already asserted absent from index + baseline above, and
    // it carries the routed finding rather than an adopt-confirmation.
    assert!(
        report.contains("needs-reconcile decisions/auth-choice.md"),
        "the non-conformant near-miss routes, never adopts:\n{report}",
    );
}
