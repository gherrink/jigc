//! End-to-end acceptance (M9 Increment 3, T4 / worked-examples.md flow 12) — the
//! brownfield arc driven through the **real `jigc` binary** on a throwaway repo,
//! wrapping T1 (discover → classify → triage), T2 (schema-gated adopt) and T3
//! (the `ingest-existing` orient/route workflow) into one end-to-end proof.
//!
//! The mandated M9 existing-project acceptance path. A four-candidate repo:
//!
//!   docs/decisions/rate-limit.md   — a CONFORMANT adr at its `docs/decisions/` location → adoptable
//!   docs/decisions/auth-choice.md  — a NON-CONFORMANT near-miss in `docs/decisions/`     → needs-reconcile
//!   docs/old-adr.md           — a CONFORMANT adr at the WRONG location (`docs/`) → needs-reconcile
//!   docs/notes.md             — freeform, outside every location dir            → unmanaged
//!
//! is wired in with `jigc setup`, oriented with
//! `jigc start --workflow ingest-existing "<intent>"`, and ingested by **running
//! the workflow's own emitted `Run:` line verbatim** — the emitted-bytes contract
//! (`increment-workflow.md` → #4): the composed view's single backticked launch
//! span is extracted and executed as the `jigc` argv, never a hand-written
//! `["ingest"]`. A workflow that emitted a broken launch line would fail here even
//! though a reconstruction would pass.
//!
//! Asserts (the flow-12 acceptance bar):
//!   (a) the conformant-at-location `rate-limit.md` adopts — its `supersedes` edge
//!       lands in the persisted index, its rel-path gains a file-state baseline,
//!       and **no file is moved** (register-only);
//!   (b) the non-conformant near-miss + the wrong-location adr route
//!       (needs-reconcile, NOT adopted — absent from index + baseline);
//!   (c) the freeform doc is left untouched (unmanaged, not adopted);
//!   (d) nothing is adopted without a schema check at its correct location — the
//!       near-miss under a location dir routes, never adopts (the hole closed);
//!   (e) no prose is rewritten — every candidate file is byte-identical pre/post.
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
            "jigc-flow12-{tag}-{}-{:?}",
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

/// Assert a `jigc` invocation succeeded, surfacing its streams on failure.
fn assert_ok(out: &std::process::Output, what: &str) {
    assert!(
        out.status.success(),
        "{what} must succeed; stdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr),
    );
}

/// Write a candidate file at `rel`, creating parent dirs as needed.
fn write(repo: &Path, rel: &str, body: &str) {
    let path = repo.join(rel);
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).expect("mk parent dir");
    }
    fs::write(path, body).expect("write candidate");
}

/// Lift the single backticked launch span the emitted `Run:` line carries — the
/// rendered command the agent would actually run, verbatim from the composed view.
/// The contract is the emitted bytes, so the argv is extracted, never reconstructed.
fn run_line_command(composed: &str) -> Vec<String> {
    let line = composed
        .lines()
        .find(|l| l.trim_start().starts_with("Run:"))
        .unwrap_or_else(|| panic!("the ingest-existing view must emit a `Run:` line:\n{composed}"));
    let mut spans = line.split('`');
    let _before = spans.next();
    let command = spans
        .next()
        .unwrap_or_else(|| panic!("the `Run:` line must carry a backticked command:\n{line}"));
    assert!(
        spans.next().is_some() && spans.next().is_none(),
        "the `Run:` line must carry exactly one backticked span; got:\n{line}",
    );
    command.split_whitespace().map(str::to_owned).collect()
}

/// A conformant ADR body — front-matter `status`/`date` + the optional `supersedes`
/// ref + the three required prose sections. The `supersedes` edge is what the adopt
/// path must populate in the index on adoption.
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

/// A genuinely freeform doc outside every location dir — plain prose, no schema.
const FREEFORM_NOTES: &str = "\
# Scratch notes

Random reminders that belong to no managed doc-type.
";

#[test]
fn flow12_four_candidate_repo_classified_and_routed_through_the_binary() {
    let repo = TempDir::new("repo");
    let home = TempDir::new("home");
    init_repo(repo.path());

    // ── on-ramp: wire jigc into the existing repo ────────────────────────────────
    let out = jigc(repo.path(), home.path(), &["setup"]);
    assert_ok(&out, "`jigc setup`");

    // ── seed the four flow-12 candidates ─────────────────────────────────────────
    write(repo.path(), "docs/decisions/rate-limit.md", CONFORMANT_ADR); // adoptable
    write(
        repo.path(),
        "docs/decisions/auth-choice.md",
        NON_CONFORMANT_NEAR_MISS,
    ); // needs-reconcile (near-miss, under a location dir)
    write(repo.path(), "docs/old-adr.md", CONFORMANT_ADR); // needs-reconcile (wrong location)
    write(repo.path(), "docs/notes.md", FREEFORM_NOTES); // unmanaged

    // Snapshot every candidate's bytes — the whole arc is detect-and-route, never
    // a rewrite or a move.
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

    // ── orient: compose `ingest-existing`, which emits the ingest launch line ────
    let oriented = jigc(
        repo.path(),
        home.path(),
        &[
            "start",
            "--workflow",
            "ingest-existing",
            "bring this repo under jigc management",
        ],
    );
    assert_ok(&oriented, "`jigc start --workflow ingest-existing`");
    let composed = String::from_utf8(oriented.stdout).expect("utf-8");

    // `ingest-existing` is `creates-task: false` — orient/route only, no mint.
    assert!(
        !repo.path().join(".jigc").join("tasks").exists(),
        "the ingest-existing orient view must mint no task; composed:\n{composed}",
    );

    // The composed view emits `Run: `jigc ingest`` — the launch line is extracted
    // verbatim and run as the `jigc` argv (the emitted-bytes contract).
    let command = run_line_command(&composed);
    assert_eq!(
        command,
        vec!["jigc".to_string(), "ingest".to_string()],
        "the emitted `Run:` line must launch the real `jigc ingest` verb; composed:\n{composed}",
    );

    // ── ingest: run the EMITTED launch line verbatim (drop the `jigc` program token)
    let out = jigc(
        repo.path(),
        home.path(),
        &command[1..].iter().map(String::as_str).collect::<Vec<_>>(),
    );
    assert_ok(&out, "the emitted `jigc ingest` launch line");
    let report = String::from_utf8(out.stdout).expect("utf-8");

    // The four candidates each carry their verdict in the report.
    let verdict_for = |rel: &str| -> &str {
        let line = report
            .lines()
            .find(|l| l.contains(rel) && !l.trim_start().starts_with("route:"))
            .unwrap_or_else(|| panic!("`{rel}` must appear in the report:\n{report}"));
        if line.contains("adoptable") || line.contains("adopted") {
            "adopted"
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
        "adopted",
        "{report}"
    );
    assert_eq!(
        verdict_for("docs/decisions/auth-choice.md"),
        "needs-reconcile",
        "{report}"
    );
    assert_eq!(
        verdict_for("docs/old-adr.md"),
        "needs-reconcile",
        "{report}"
    );
    assert_eq!(verdict_for("docs/notes.md"), "unmanaged", "{report}");

    // A needs-reconcile row renders a routed finding — blocking + a route.
    assert!(
        report.contains("blocking"),
        "a needs-reconcile row must render a blocking finding:\n{report}"
    );
    assert!(
        report.contains("route"),
        "a needs-reconcile row must render its route:\n{report}"
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

    // ── (b)+(c)+(d) the routed + unmanaged docs are NOT adopted (hole closed) ─────
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
    // The wrong-location doc shares the conformant bytes (slug `old-adr`); the
    // near-miss/freeform slugs must all be absent — only `rate-limit` registered.
    for slug in ["adr:old-adr", "adr:auth-choice", "adr:notes"] {
        assert!(
            !edges.contains(&format!("\"from\": \"{slug}\"")),
            "no routed/unmanaged candidate (`{slug}`) enters the index:\n{edges}",
        );
    }
    // (d) the non-conformant near-miss under a location dir routes, never adopts —
    // nothing adopted without a schema check at its correct location.
    assert!(
        report.contains("needs-reconcile docs/decisions/auth-choice.md"),
        "the non-conformant near-miss routes, never adopts:\n{report}",
    );

    // ── (e) no prose rewritten: every candidate's bytes are byte-identical ────────
    for (rel, bytes) in &before {
        let after = fs::read(repo.path().join(rel)).expect("re-read");
        assert_eq!(
            *bytes, after,
            "ingest must not rewrite `{rel}` (detect-and-route only)"
        );
    }
    // No file relocated/duplicated: `docs/decisions/` still holds exactly its two seeds.
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
}
