//! Acceptance — **version-aware migrate-vs-corrupt routing** for the store-scope
//! schema-conformance detector (M34 Increment 3, T3).
//!
//! The Inc-1 fifth family detects a committed doc made non-conformant by a schema-shape
//! change; this task makes the detector **route** each such finding by the doc's
//! schema-version stamp vs the doctype's manifest version (`design/validation.md` →
//! Store-scope schema-conformance → Version-aware routing; `design/corpus-migration.md` →
//! the schema-version stamp):
//!
//! - **below-version / stamp-absent ⇒ `migrate`** — a known-old-version (or unstamped v0)
//!   doc the transform can upgrade.
//! - **at-version + non-conformant ⇒ `corrupt`** — a doc already at the current schema
//!   version that still does not conform, so it needs human review, not a version bump.
//!
//! Both routes ride the existing `schema-conformance.required-field-present` finding's
//! `route` field — **no new check id, knob, or inventory row** (`result.rs` →
//! `check_inventory_membership_count_is_stable` stays green). Report-only at store scope:
//! `jigc validate` still exits 0; the *blocking* counterpart is the migration-upgrade gate
//! at the transform transaction boundary, not this read-only sweep.
//!
//! Drives the built `jigc` binary through the real store-scope `jigc validate` so the
//! asserted bytes are the ones an operator actually sees (the route lines the renderer
//! emits), never a reconstructed equivalent. The freeze-gate is respected: the schema-shape
//! change is a **project-layer schema shadow**, never a pack-schema edit.

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
            "jigc-conformance-routing-{tag}-{}-{:?}",
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
/// `schema-version` stamp line in its header (`None` = the unstamped v0 state). Its prose
/// is conformant; only the schema (the shadow) changes around it.
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

/// Install a **project-layer schema shadow** of the `adr` doctype that adds a required
/// `owner` header field (the simulated schema-shape change — never a pack-schema edit). The
/// shadow omits the `schema-version` stamp on purpose; the pack loader injects it uniformly.
fn shadow_adr_schema(repo: &Path) {
    let dir = repo.join(".jigc").join("config").join("schemas");
    fs::create_dir_all(&dir).expect("mk .jigc/config/schemas/");
    let shadow = "\
type: adr
location: decisions/
id-from: title
sections:
  - id: status
    header: true
    fields:
      - { id: status, type: enum, of: [proposed, accepted, superseded], default: proposed }
      - { id: date, type: date, set: on-create }
      - { id: supersedes, type: ref, to: adr, card: \"0..*\", inverse: superseded-by }
      - { id: cites-code, type: code-anchor }
      - { id: owner, type: string }
  - id: context
    slot: { hint: Forces. }
  - id: decision
    slot: { hint: What. }
  - id: consequences
    slot: { hint: Effects. }
";
    fs::write(dir.join("adr.yaml"), shadow).expect("write adr schema shadow");
    git(repo, &["add", "."]);
    git(repo, &["commit", "-q", "-m", "shadow adr schema"]);
}

/// Count non-overlapping occurrences of `needle` in `haystack`.
fn count(haystack: &str, needle: &str) -> usize {
    haystack.matches(needle).count()
}

/// (routing) Under a schema shadow that adds a required field, three committed ADRs — one
/// unstamped (v0), one stamped below the current version (0 < 1), one at the current version
/// (1) — each surface a `required-field-present` break. The two below-current/absent docs
/// route `migrate`; the at-version doc routes `corrupt`. `jigc validate` still exits 0.
#[test]
fn store_sweep_routes_below_version_migrate_and_at_version_corrupt() {
    let repo = TempDir::new("route");
    let home = TempDir::new("home");
    setup_repo(repo.path(), home.path());

    // stamp-absent (the v0 corpus state) ⇒ migrate.
    commit_adr(repo.path(), "alpha-decision", "Alpha decision", None);
    // stamped below the current manifest version (0 < 1) ⇒ migrate.
    commit_adr(repo.path(), "beta-decision", "Beta decision", Some(0));
    // stamped at the current manifest version (1) but non-conformant ⇒ corrupt.
    commit_adr(repo.path(), "gamma-decision", "Gamma decision", Some(1));

    shadow_adr_schema(repo.path());

    let out = jigc(repo.path(), home.path(), &["validate"]);
    let stdout = String::from_utf8_lossy(&out.stdout);
    let stderr = String::from_utf8_lossy(&out.stderr);

    // Report-only: a content-only sweep exits 0 even with blocking-severity findings.
    assert!(
        out.status.success(),
        "a schema-conformance break must not gate `jigc validate` (exit 0); \
         stdout:\n{stdout}\nstderr:\n{stderr}",
    );

    // Each of the three non-conformant ADRs surfaces exactly one required-field-present.
    assert_eq!(
        count(&stdout, "schema-conformance.required-field-present"),
        3,
        "each committed ADR lacking the now-required `owner` field must surface one \
         required-field-present finding; stdout:\n{stdout}",
    );

    // The two below-current / stamp-absent docs route `migrate`; the at-version doc routes
    // `corrupt`. The route lines are the emitted bytes an operator reads.
    assert_eq!(
        count(&stdout, "route: migrate"),
        2,
        "the unstamped (v0) and below-version (0) ADRs must each route `migrate`; \
         stdout:\n{stdout}",
    );
    assert_eq!(
        count(&stdout, "route: corrupt"),
        1,
        "the at-version (1) non-conformant ADR must route `corrupt`; stdout:\n{stdout}",
    );
}

/// (conformant) With **no** schema shadow the committed ADRs conform — no schema-conformance
/// finding, no route line, exit 0. The false-positive guard for the routing path.
#[test]
fn store_sweep_clean_and_unrouted_on_conformant_store() {
    let repo = TempDir::new("clean");
    let home = TempDir::new("home");
    setup_repo(repo.path(), home.path());
    commit_adr(repo.path(), "alpha-decision", "Alpha decision", Some(1));

    let out = jigc(repo.path(), home.path(), &["validate"]);
    let stdout = String::from_utf8_lossy(&out.stdout);
    let stderr = String::from_utf8_lossy(&out.stderr);

    assert!(
        out.status.success(),
        "a conformant store must exit 0; stdout:\n{stdout}\nstderr:\n{stderr}",
    );
    assert!(
        !stdout.contains("schema-conformance"),
        "a fully conformant committed ADR must surface NO schema-conformance finding; \
         stdout:\n{stdout}",
    );
    assert_eq!(
        count(&stdout, "route: migrate") + count(&stdout, "route: corrupt"),
        0,
        "a conformant store must carry no migrate/corrupt route line; stdout:\n{stdout}",
    );
}
