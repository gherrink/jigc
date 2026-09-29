//! Acceptance — the store-scope **fifth content family**: per-doc schema-conformance
//! re-parse over the committed store (M34 Increment 1, T1; the detect-half).
//!
//! The first four store families re-check *anchors*, *workflow refs*, *file-state hashes*,
//! and *cross-doc edges*, but **none re-parses a committed doc against its schema** — so a
//! doc made non-conformant by a *schema-shape change* with its **bytes unchanged** is
//! invisible to `jigc validate` (the file↔CLI-state family is hash-only; bytes match
//! baseline ⇒ no finding). That is the exact M34 case — a v1 corpus under a v2 schema —
//! and the fifth family closes it (`design/validation.md` → Store-scope schema-conformance;
//! `design/corpus-migration.md` → the detect-half).
//!
//! This drives the built `jigc` binary through the real store-scope `jigc validate` (the
//! M10 invocation-path-masking lesson: drive the bytes an operator would actually run):
//!
//! - **(non-conformant under a reshaped schema)** — a committed ADR conformant under the
//!   shipped adr schema, read against a pack whose adr schema gained a required `owner`
//!   header field (the schema change), surfaces a
//!   `schema-conformance.required-field-present` finding at store scope **with its bytes
//!   unchanged**; `jigc validate` still exits 0 (report-only — the intrinsic-blocking
//!   severity is a finalize verdict, *listed* under the read-only store sweep).
//! - **(conformant store)** — the same committed ADR under the **shipped** schema
//!   surfaces **no** schema-conformance finding and exits 0.
//!
//! The freeze gate is **satisfied, not dodged** (M49 Increment 3). This suite used to
//! simulate the schema change with a project-layer `.jigc/config/schemas/adr.yaml`
//! shadow, on the recorded rationale that *"the M33 pack-load freeze assertion is
//! respected"* — which was false: the gate hashed each pack's own schemas and never the
//! cascade-resolved one, so the project layer reshaped a frozen doctype in silence. That
//! is now blocked, and the device moves to where a shape change legitimately lives: an
//! on-disk pack copy whose `adr` schema gains the field **and whose manifest entry is
//! re-pinned** (`support::frozen_pack`), which is the act a pack author actually
//! performs.
//!
//! No external test crates: the binary path comes from `CARGO_BIN_EXE_jigc`, the temp repo
//! is a real `git init` + `jigc setup`, the probe is the real `doc-code` (so the
//! `jigc validate` pre-flight resolves), and a self-cleaning `TempDir` keeps the test off
//! the developer's repo.

use crate::support::frozen_pack;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

/// A throwaway directory that removes itself on drop.
struct TempDir(PathBuf);

impl TempDir {
    fn new(tag: &str) -> Self {
        let mut path = std::env::temp_dir();
        path.push(format!(
            "jigc-conformance-store-{tag}-{}-{:?}",
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

/// Run `jigc <args>` with `cwd = repo`, `$HOME = home`, and the real `doc-code` probe
/// resolved through the production path (no `JIGC_DOC_CODE_PROBE` override). The shipped
/// (embedded) pack — a developer's stray `JIGC_PACK_DIR` is scrubbed.
fn jigc(repo: &Path, home: &Path, args: &[&str]) -> std::process::Output {
    jigc_with_pack(repo, home, None, args)
}

/// [`jigc`], with `pack` selecting an on-disk pack tree via `JIGC_PACK_DIR` — the seam
/// the reshaped-schema arm drives (`support::frozen_pack`).
fn jigc_with_pack(
    repo: &Path,
    home: &Path,
    pack: Option<&Path>,
    args: &[&str],
) -> std::process::Output {
    let mut command = Command::new(env!("CARGO_BIN_EXE_jigc"));
    command
        .args(args)
        .current_dir(repo)
        .env("HOME", home)
        .env_remove("JIGC_DOC_CODE_PROBE");
    match pack {
        Some(dir) => command.env("JIGC_PACK_DIR", dir),
        None => command.env_remove("JIGC_PACK_DIR"),
    };
    command.output().expect("run the jigc binary")
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

/// Make `root` a real git repo with identity, then run `jigc setup` over it (the project
/// layer). Returns once the store is a clean, set-up repo.
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

/// A conformant `adr` body under the **pack** adr schema (status/date header + the four
/// prose slots incl. the M36 optional `## Options`; **no `owner` field**). It carries
/// `schema-version: 2` — the at-version stamp under the adr v1→v2 options-slot bump, so a
/// *fully conformant* committed doc is genuinely clean (an absent/below stamp is itself a
/// surfaced version-mismatch break, M34 Inc-3). Its bytes never change across this test —
/// only the resolved schema does.
const ADR_BODY: &str = "\
---
status: accepted
date: 2026-06-25
schema-version: 2
---

# Cache sessions in memory

## Context

Session lookups must stay sub-millisecond.

## Options

A distributed cache was weighed and rejected on latency.

## Decision

Keep sessions in a single in-memory node.

## Consequences

A cold node loses its sessions.
";

/// Commit the conformant ADR at its canonical `docs/decisions/cache-sessions-in-memory.md`
/// (the `adr` doctype's `location: decisions/` nested under the resolved `docs/` root).
fn commit_adr(repo: &Path) {
    let dir = repo.join("docs").join("decisions");
    fs::create_dir_all(&dir).expect("mk docs/decisions/");
    fs::write(dir.join("cache-sessions-in-memory.md"), ADR_BODY).expect("write adr");
    git(repo, &["add", "."]);
    git(repo, &["commit", "-q", "-m", "seed adr"]);
}

/// Build an on-disk dev-pack copy at `root` whose `adr` schema gains a required `owner`
/// header field — the simulated schema-shape change, with the copy's manifest entry
/// **re-pinned** so the pack passes its own freeze gate (the version is held at 2, so the
/// committed fixture's `schema-version: 2` stamp stays *at-version*).
///
/// The added field is spliced into the **shipped** `adr.yaml`, so the reshaped schema
/// differs from the frozen one in exactly that field — a hand-copied body would drift
/// from the pack the moment `adr` changed.
fn reshape_adr_pack(root: &Path) {
    frozen_pack::reshaped_dev_pack(root, "adr", |body| {
        let out = body.replacen(
            "      - { id: cites-code, type: code-anchor }\n",
            "      - { id: cites-code, type: code-anchor }\n      - { id: owner, type: string }\n",
            1,
        );
        assert_ne!(body, out, "adr.yaml must declare the cites-code field");
        out
    });
}

/// Count non-overlapping occurrences of `needle` in `haystack`.
fn count(haystack: &str, needle: &str) -> usize {
    haystack.matches(needle).count()
}

/// (non-conformant under a reshaped schema) A committed ADR conformant under the shipped
/// schema, read against a pack whose `adr` gained a required `owner` field, surfaces
/// exactly one `schema-conformance.required-field-present` finding at store scope **with
/// its bytes unchanged**; `jigc validate` still exits 0 (report-only).
#[test]
fn store_sweep_surfaces_schema_conformance_break_under_reshaped_schema() {
    let repo = TempDir::new("break");
    let home = TempDir::new("home");
    let pack = TempDir::new("break-pack");
    setup_repo(repo.path(), home.path());
    commit_adr(repo.path());
    reshape_adr_pack(pack.path());

    let out = jigc_with_pack(repo.path(), home.path(), Some(pack.path()), &["validate"]);
    let stdout = String::from_utf8_lossy(&out.stdout);
    let stderr = String::from_utf8_lossy(&out.stderr);

    // Report-only: a content-only sweep exits 0 even with a blocking-severity finding.
    assert!(
        out.status.success(),
        "a schema-conformance break must not gate `jigc validate` (exit 0); \
         stdout:\n{stdout}\nstderr:\n{stderr}",
    );
    assert_eq!(
        count(&stdout, "schema-conformance.required-field-present"),
        1,
        "a committed doc made non-conformant by the reshaped schema (a now-required `owner` \
         field its unchanged bytes lack) must surface EXACTLY ONE required-field-present \
         finding at store scope; stdout:\n{stdout}",
    );
}

/// (conformant store) The same committed ADR under the **shipped** schema surfaces **no**
/// schema-conformance finding and exits 0 — the false-positive guard for the fifth family.
#[test]
fn store_sweep_clean_on_conformant_store() {
    let repo = TempDir::new("clean");
    let home = TempDir::new("home");
    setup_repo(repo.path(), home.path());
    commit_adr(repo.path());

    let out = jigc(repo.path(), home.path(), &["validate"]);
    let stdout = String::from_utf8_lossy(&out.stdout);
    let stderr = String::from_utf8_lossy(&out.stderr);

    assert!(
        out.status.success(),
        "a conformant store must exit 0; stdout:\n{stdout}\nstderr:\n{stderr}",
    );
    assert!(
        !stdout.contains("schema-conformance"),
        "a fully conformant committed doc must surface NO schema-conformance finding; \
         stdout:\n{stdout}",
    );
}
