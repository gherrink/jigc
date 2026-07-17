//! Acceptance spine (M9 Increment 3, T1) — `jigc ingest` discover → classify →
//! adopt → triage report, end-to-end through the built binary. Adopt is
//! **register-only**: it records conformant candidates into the edge index +
//! file-state baseline but never moves or rewrites a candidate file.
//!
//! Drives the real `jigc` binary against a throwaway git repo seeded with the four
//! flow-12 candidates (after `jigc setup`):
//!
//!   docs/decisions/rate-limit.md   — a CONFORMANT adr at its `docs/decisions/` location → adoptable
//!   docs/decisions/auth-choice.md  — a NON-CONFORMANT near-miss in `docs/decisions/`     → needs-reconcile
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

## Options
Alternatives were weighed and rejected.

## Decision
A token bucket per client keeps the gateway fair under burst.

## Consequences
A misbehaving client is throttled, not the whole gateway.
";

/// A near-miss freeform doc dropped into `docs/decisions/`: no front-matter and none of
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
    write(repo.path(), "docs/decisions/rate-limit.md", CONFORMANT_ADR); // adoptable
    write(
        repo.path(),
        "docs/decisions/auth-choice.md",
        NON_CONFORMANT_NEAR_MISS,
    ); // needs-reconcile (near-miss)
    write(repo.path(), "docs/old-adr.md", CONFORMANT_ADR); // needs-reconcile (wrong location)
    write(repo.path(), "docs/notes.md", NON_CONFORMANT_NEAR_MISS); // unmanaged

    // Snapshot the candidate bytes — adopt is register-only: no candidate file is rewritten.
    let before: Vec<(String, Vec<u8>)> = [
        "docs/decisions/rate-limit.md",
        "docs/decisions/auth-choice.md",
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
        verdict_for("docs/decisions/rate-limit.md"),
        "adoptable",
        "report:\n{report}"
    );
    assert_eq!(
        verdict_for("docs/decisions/auth-choice.md"),
        "needs-reconcile",
        "report:\n{report}"
    );
    assert_eq!(
        verdict_for("docs/old-adr.md"),
        "needs-reconcile",
        "report:\n{report}"
    );
    // The unmanaged candidate collapses into a per-directory count line (V9, M41 Inc
    // 8) — no per-file row — so its verdict reads from the collapsed `docs/` summary.
    assert!(
        report.contains("unmanaged docs/ — 1 file(s)"),
        "the unmanaged `docs/notes.md` must collapse into a `docs/` count line:\n{report}"
    );

    // Sorted candidate order: the itemized rows appear in lexicographic path order,
    // and the collapsed unmanaged summary follows every itemized row.
    let pos = |rel: &str| {
        report
            .find(rel)
            .unwrap_or_else(|| panic!("`{rel}` in report:\n{report}"))
    };
    assert!(
        pos("docs/decisions/auth-choice.md") < pos("docs/decisions/rate-limit.md"),
        "rows must be in sorted candidate order:\n{report}"
    );
    assert!(
        pos("docs/decisions/rate-limit.md") < pos("docs/old-adr.md"),
        "rows must be in sorted candidate order:\n{report}"
    );
    assert!(
        pos("docs/old-adr.md") < pos("unmanaged docs/ — 1 file(s)"),
        "the collapsed unmanaged summary follows the itemized rows:\n{report}"
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

/// Recursive reach (G3): a doc in a nested subdirectory (`docs/sub/x.md`) — invisible
/// to M9's top-level-only scan — now appears in `jigc ingest` output. Drives the real
/// binary so the recursion is proven end-to-end, not just at the engine unit level.
#[test]
fn ingest_discovers_a_nested_subdirectory_doc() {
    let repo = TempDir::new("nested");
    let home = TempDir::new("home");
    init_repo(repo.path());

    let out = jigc(repo.path(), home.path(), &["setup"]);
    assert!(
        out.status.success(),
        "`jigc setup` must succeed; stderr:\n{}",
        String::from_utf8_lossy(&out.stderr),
    );

    // A freeform doc nested two levels deep — reachable only by recursing from the root.
    write(repo.path(), "docs/sub/x.md", NON_CONFORMANT_NEAR_MISS);

    let out = jigc(repo.path(), home.path(), &["ingest"]);
    assert!(
        out.status.success(),
        "`jigc ingest` must succeed; stdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr),
    );
    let report = String::from_utf8(out.stdout).expect("utf-8");

    // The nested doc is unmanaged (freeform near-miss outside every location) → it
    // collapses into a `docs/sub/` per-directory count (V9); the directory's presence
    // proves the scan recursed two levels deep to reach it.
    assert!(
        report.contains("unmanaged docs/sub/ — 1 file(s)"),
        "a nested docs/sub/ must appear in `jigc ingest` output (recursion reached it):\n{report}",
    );
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
    write(repo.path(), "docs/decisions/rate-limit.md", CONFORMANT_ADR); // adoptable
    write(
        repo.path(),
        "docs/decisions/auth-choice.md",
        NON_CONFORMANT_NEAR_MISS,
    ); // needs-reconcile (near-miss, under a location dir)
    write(repo.path(), "docs/old-adr.md", CONFORMANT_ADR); // needs-reconcile (wrong location)
    write(repo.path(), "docs/notes.md", NON_CONFORMANT_NEAR_MISS); // unmanaged

    // Snapshot every candidate's bytes — adopt is register-only: nothing rewritten.
    let candidates = [
        "docs/decisions/rate-limit.md",
        "docs/decisions/auth-choice.md",
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
        row("docs/decisions/rate-limit.md").contains("adopted"),
        "the conformant-at-location row renders the adopt-confirmation marker:\n{report}",
    );
    assert!(
        !report.contains("docs/decisions/auth-choice.md")
            || !row("docs/decisions/auth-choice.md").contains("adopted"),
        "the near-miss row must NOT render an adopt-confirmation:\n{report}",
    );
    assert!(
        !row("docs/old-adr.md").contains("adopted"),
        "the wrong-location row must NOT render an adopt-confirmation:\n{report}",
    );
    // The unmanaged doc collapses into a `docs/` count line (V9) — never adopted; the
    // collapsed summary carries no adopt-confirmation marker (the structural
    // non-adoption is re-proven against the index + baseline below).
    assert!(
        report.contains(
            "unmanaged docs/ — 1 file(s) parse against no schema (left untouched — fine to stay plain)"
        ),
        "the unmanaged row must collapse into a `docs/` count with no adopt-confirmation:\n{report}",
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
        baseline.contains("\"docs/decisions/rate-limit.md\""),
        "the adopted adr's rel-path must gain a file-state baseline:\n{baseline}",
    );

    // ── (b) the two needs-reconcile docs + the unmanaged doc are NOT adopted ──────
    // Absent from both the index (no forward edge) and the baseline (no hash).
    for rel in [
        "docs/decisions/auth-choice.md",
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
    let decisions: Vec<_> = fs::read_dir(repo.path().join("docs").join("decisions"))
        .expect("read docs/decisions/")
        .flatten()
        .map(|e| e.file_name())
        .collect();
    assert_eq!(
        decisions.len(),
        2,
        "adopt moves/creates no file in docs/decisions/: {decisions:?}",
    );

    // ── (d) nothing adopted without a schema check: the non-conformant near-miss ──
    // under a location dir (`docs/decisions/auth-choice.md`) is never adopted (the hole
    // closed end-to-end) — already asserted absent from index + baseline above, and
    // it carries the routed finding rather than an adopt-confirmation.
    assert!(
        report.contains("needs-reconcile docs/decisions/auth-choice.md"),
        "the non-conformant near-miss routes, never adopts:\n{report}",
    );
}

/// M40 / F4 (the ingest half) — **adopt-time triage annotations**, row-carried in
/// ALL output formats (`design/project-setup.md` → Flow 2, the M40 triage-annotation
/// vocabulary; `design/validation.md` → Hollow and surplus adoption). Adopting a
/// zero-item roadmap (`docs/roadmap.md`, the placement singleton with an empty
/// `milestones` repeatable) and a conformant adr carrying one surplus trailing H2
/// annotates each triage row — the pinned shapes *"adopted — structurally empty:
/// 0 milestones"* / *"adopted — 1 surplus trailing sections"* — in BOTH the human
/// render and the `--format json` projection. Fixed-advisory: the annotated rows are
/// still marked adopted (verdict unflipped, adoption not blocked) and the scan still
/// exits 0.
#[test]
fn ingest_annotates_hollow_and_surplus_adoptions_in_human_and_json_output() {
    let repo = TempDir::new("annotate");
    let home = TempDir::new("annotate-home");
    init_repo(repo.path());

    let out = jigc(repo.path(), home.path(), &["setup"]);
    assert!(
        out.status.success(),
        "`jigc setup` must succeed; stderr:\n{}",
        String::from_utf8_lossy(&out.stderr),
    );

    // A structurally hollow roadmap at its literal placement home — exactly what the
    // adoption trial adopted silent (the `roadmap#milestones` token is NOT exempt).
    write(
        repo.path(),
        "docs/roadmap.md",
        "# roadmap\n\n## Milestones\n",
    );
    // A conformant adr with one surplus trailing H2 — the positional parse never
    // visits it, so the doc still classifies adoptable.
    let surplus_adr = format!(
        "{CONFORMANT_ADR}\n## Legacy planning notes\n\nOld notes, carried byte-faithful.\n"
    );
    write(repo.path(), "docs/decisions/legacy.md", &surplus_adr);
    // A clean conformant adr — adopted with NO annotation (the omitting row).
    write(repo.path(), "docs/decisions/rate-limit.md", CONFORMANT_ADR);

    // ── the human render carries the pinned annotation shapes on adopted rows ─────
    let out = jigc(repo.path(), home.path(), &["ingest", "--format", "human"]);
    assert!(
        out.status.success(),
        "annotations are fixed-advisory — the scan still exits 0; stdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr),
    );
    let report = String::from_utf8(out.stdout).expect("utf-8");
    let row = |rel: &str| -> &str {
        report
            .lines()
            .find(|l| l.contains(rel))
            .unwrap_or_else(|| panic!("`{rel}` row must appear in the report:\n{report}"))
    };
    // Annotated rows are still adopted — the annotation never flips a verdict.
    assert!(
        row("docs/roadmap.md").contains("adoptable") && row("docs/roadmap.md").contains("adopted"),
        "the hollow roadmap still adopts:\n{report}",
    );
    assert!(
        row("docs/decisions/legacy.md").contains("adoptable")
            && row("docs/decisions/legacy.md").contains("adopted"),
        "the surplus-trailing adr still adopts:\n{report}",
    );
    assert!(
        report.contains("adopted — structurally empty: 0 milestones"),
        "the hollow-adoption annotation renders with its pinned shape:\n{report}",
    );
    assert!(
        report.contains("adopted — 1 surplus trailing sections"),
        "the surplus-adoption annotation renders with its pinned shape:\n{report}",
    );

    // ── the JSON projection carries the same annotations on the serialized rows ───
    let out = jigc(repo.path(), home.path(), &["ingest", "--format", "json"]);
    assert!(
        out.status.success(),
        "the json scan still exits 0; stderr:\n{}",
        String::from_utf8_lossy(&out.stderr),
    );
    let json: serde_json::Value =
        serde_json::from_slice(&out.stdout).expect("`--format json` emits parseable JSON");
    let rows = json["rows"].as_array().expect("a rows array");
    let json_row = |rel: &str| -> &serde_json::Value {
        rows.iter()
            .find(|r| r["file"] == rel)
            .unwrap_or_else(|| panic!("`{rel}` row must be serialized:\n{json}"))
    };
    let roadmap = json_row("docs/roadmap.md");
    assert_eq!(
        roadmap["adopted"], true,
        "the annotated roadmap row is still marked adopted:\n{roadmap}",
    );
    assert_eq!(
        roadmap["annotations"],
        serde_json::json!(["adopted — structurally empty: 0 milestones"]),
        "the hollow annotation is row-carried in the JSON projection:\n{roadmap}",
    );
    let adr = json_row("docs/decisions/legacy.md");
    assert_eq!(
        adr["adopted"], true,
        "the annotated adr row is still marked adopted:\n{adr}",
    );
    assert_eq!(
        adr["annotations"],
        serde_json::json!(["adopted — 1 surplus trailing sections"]),
        "the surplus annotation is row-carried in the JSON projection:\n{adr}",
    );
    // An un-annotated adopted row carries an empty annotations array (shape-uniform).
    let clean = json_row("docs/decisions/rate-limit.md");
    assert_eq!(
        clean["annotations"],
        serde_json::json!([]),
        "a clean adopted row serializes an empty annotations array:\n{clean}",
    );
}

/// M40 / F8 — the candidate set is CLI-computed from `git ls-files --cached --others
/// --exclude-standard -- '*.md'`: a **gitignored** `.md` (the adoption trial's
/// `node_modules` funnel poison — 91% of the logged triage output) is absent from the
/// triage report, while an **untracked non-ignored** `.md` still appears
/// (`design/project-setup.md` → Flow-2 discovery, the M40 re-base paragraph).
#[test]
fn ingest_excludes_gitignored_candidates_and_includes_untracked_markdown() {
    let repo = TempDir::new("gitignored");
    let home = TempDir::new("gitignored-home");
    init_repo(repo.path());

    let out = jigc(repo.path(), home.path(), &["setup"]);
    assert!(
        out.status.success(),
        "`jigc setup` must succeed; stderr:\n{}",
        String::from_utf8_lossy(&out.stderr),
    );

    // Gitignore `node_modules/` (appending — setup may have seeded a root
    // `.gitignore`), then seed a gitignored candidate + an untracked non-ignored one.
    let gitignore = repo.path().join(".gitignore");
    let mut ignore_body = fs::read_to_string(&gitignore).unwrap_or_default();
    ignore_body.push_str("node_modules/\n");
    fs::write(&gitignore, ignore_body).expect("write .gitignore");
    write(
        repo.path(),
        "node_modules/pkg/README.md",
        "# pkg\n\nA dependency's readme — never a triage candidate.\n",
    );
    write(
        repo.path(),
        "docs/untracked-note.md",
        NON_CONFORMANT_NEAR_MISS,
    );

    // Read the full-row JSON contract: the text arm collapses unmanaged rows into
    // per-directory counts (V9), so per-file inclusion/exclusion reads from JSON.
    let out = jigc(repo.path(), home.path(), &["ingest", "--format", "json"]);
    assert!(
        out.status.success(),
        "`jigc ingest` must succeed; stdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr),
    );
    let report = String::from_utf8(out.stdout).expect("utf-8");

    assert!(
        !report.contains("node_modules/pkg/README.md"),
        "a gitignored candidate must be absent from the triage report:\n{report}"
    );
    assert!(
        report.contains("docs/untracked-note.md"),
        "an untracked non-ignored candidate must appear in the triage report:\n{report}"
    );
}

/// M40 / F8 hardening — a **non-ASCII** `.md` filename (tracked or untracked) must not
/// abort the scan. git's default `core.quotepath=true` C-quotes such paths in
/// `ls-files` porcelain output (`"docs/r\303\251sum\303\251.md"`); consuming that
/// quoted line as a literal path hard-fails the whole verb with zero verdicts. The
/// candidate listing is NUL-terminated (`ls-files -z`), so both files classify as
/// ordinary candidates under their real names.
#[test]
fn ingest_survives_non_ascii_candidate_filenames() {
    let repo = TempDir::new("quotepath");
    let home = TempDir::new("quotepath-home");
    init_repo(repo.path());

    let out = jigc(repo.path(), home.path(), &["setup"]);
    assert!(
        out.status.success(),
        "`jigc setup` must succeed; stderr:\n{}",
        String::from_utf8_lossy(&out.stderr),
    );

    // One tracked (--cached arm) and one untracked (--others arm) non-ASCII filename —
    // both quoted by `core.quotepath=true` porcelain output.
    write(
        repo.path(),
        "docs/café.md",
        "# café\n\nTracked prose with a non-ASCII name.\n",
    );
    git(repo.path(), &["add", "docs/café.md"]);
    git(repo.path(), &["commit", "-q", "-m", "add café note"]);
    write(
        repo.path(),
        "docs/résumé.md",
        "# résumé\n\nUntracked prose with a non-ASCII name.\n",
    );

    // Read the full-row JSON contract: the text arm collapses these unmanaged rows
    // into a per-directory count (V9), so the per-file real-name check reads from JSON.
    let out = jigc(repo.path(), home.path(), &["ingest", "--format", "json"]);
    assert!(
        out.status.success(),
        "`jigc ingest` must survive non-ASCII candidate filenames; stdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr),
    );
    let report = String::from_utf8(out.stdout).expect("utf-8");

    assert!(
        report.contains("docs/café.md"),
        "the tracked non-ASCII candidate must appear under its real name:\n{report}"
    );
    assert!(
        report.contains("docs/résumé.md"),
        "the untracked non-ASCII candidate must appear under its real name:\n{report}"
    );
    assert!(
        !report.contains("\\303"),
        "no C-quoted octal-escaped path may leak into the report:\n{report}"
    );
}
