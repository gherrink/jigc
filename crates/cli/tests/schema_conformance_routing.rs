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
//! emits), never a reconstructed equivalent.
//!
//! The freeze gate is **satisfied, not dodged** (M49 Increment 3). The schema-shape change
//! used to be a project-layer `.jigc/config/schemas/adr.yaml` shadow, described here as
//! *"never a pack-schema edit"* as though that respected the freeze; it did not — the gate
//! hashed each pack's own schemas and never the cascade-resolved one, so the project layer
//! reshaped a frozen doctype in silence. That is now blocked, and the device is an on-disk
//! pack copy whose `adr` schema gains the field **with its manifest entry re-pinned**
//! (`support::frozen_pack`) — the declared `schema-version` held at 2, so the stamp
//! fixtures below (absent / 0 / 2) keep meaning v0, below-version and at-version.

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
            "jigc-conformance-routing-{tag}-{}-{:?}",
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
    jigc_with_pack(repo, home, None, args)
}

/// [`jigc`], with `pack` selecting an on-disk pack tree via `JIGC_PACK_DIR` — the seam the
/// reshaped-schema arms drive (`support::frozen_pack`). `None` scrubs the variable, so a
/// developer's stray `JIGC_PACK_DIR` never reaches the shipped-pack arms.
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

/// Build an on-disk dev-pack copy at `root` whose `adr` schema gains a required `owner`
/// header field — the simulated schema-shape change, with the copy's manifest entry
/// **re-pinned** so the pack passes its own freeze gate and its declared
/// `schema-version` held at 2. The field is spliced into the **shipped** `adr.yaml`, so
/// the reshaped schema differs from the frozen one in exactly that field.
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

/// (routing) Under a reshaped schema that adds a required field, three committed ADRs — one
/// unstamped (v0), one stamped below the current version (0 < 2), one at the current version
/// (2) — each surface a `required-field-present` break. The two below-current/absent docs
/// route `migrate`; the at-version doc routes `corrupt`. `jigc validate` still exits 0.
///
/// **The two below-version docs each also carry the version-currency break** (M42 Inc-3 T2):
/// `schema-conformance.schema-version-current` is emitted **unconditionally** on a below-version
/// managed doc, not only on an otherwise-clean one — the old *"only when the doc has no other
/// findings"* guard existed solely to stop the reused `field-value-conformant` id from
/// double-reporting, and it left exactly this doc (stale **and** structurally broken — the
/// commonest kind) with no machine-readable staleness fact. The at-version doc carries none: its
/// break is corruption, not staleness.
#[test]
fn store_sweep_routes_below_version_migrate_and_at_version_corrupt() {
    let repo = TempDir::new("route");
    let home = TempDir::new("home");
    let pack = TempDir::new("route-pack");
    setup_repo(repo.path(), home.path());

    // stamp-absent (the v0 corpus state) ⇒ migrate.
    commit_adr(repo.path(), "alpha-decision", "Alpha decision", None);
    // stamped below the current manifest version (0 < 2) ⇒ migrate.
    commit_adr(repo.path(), "beta-decision", "Beta decision", Some(0));
    // stamped at the current manifest version (2) but non-conformant ⇒ corrupt.
    commit_adr(repo.path(), "gamma-decision", "Gamma decision", Some(2));

    reshape_adr_pack(pack.path());

    let out = jigc_with_pack(repo.path(), home.path(), Some(pack.path()), &["validate"]);
    let stdout = String::from_utf8_lossy(&out.stdout);
    let stderr = String::from_utf8_lossy(&out.stderr);

    // M42: two of the three ADRs are below their manifest version, so the corpus is unmigrated
    // and the sweep exits **non-zero** (the third exit-flipping exception — `design/validation.md`
    // → Exit semantics). The *structural* breaks below remain report-only: they contribute no
    // exit flip of their own (the `gamma` doc is at-version and blocking, and the migrated-corpus
    // arm in `managed_vs_foreign.rs` pins that such a break alone still exits 0).
    assert!(
        !out.status.success(),
        "an unmigrated corpus flips `jigc validate`'s exit; stdout:\n{stdout}\nstderr:\n{stderr}",
    );

    // Each of the three non-conformant ADRs surfaces exactly one required-field-present.
    assert_eq!(
        count(&stdout, "schema-conformance.required-field-present"),
        3,
        "each committed ADR lacking the now-required `owner` field must surface one \
         required-field-present finding; stdout:\n{stdout}",
    );

    // Each below-version doc ALSO carries its version-currency break — emitted unconditionally,
    // alongside the structural one; the at-version doc carries none (corruption, not staleness).
    assert_eq!(
        count(&stdout, "schema-conformance.schema-version-current"),
        2,
        "the unstamped (v0) and below-version (0) ADRs must each surface the version-currency \
         break even though they already carry a structural break; stdout:\n{stdout}",
    );

    // The two below-current / stamp-absent docs route `migrate` — on both of their findings;
    // the at-version doc routes `corrupt`. The route lines are the emitted bytes an operator
    // reads.
    assert_eq!(
        count(&stdout, "route: migrate"),
        4,
        "the unstamped (v0) and below-version (0) ADRs must route `migrate` on each of their \
         two findings (the structural break + the version-currency break); stdout:\n{stdout}",
    );
    assert_eq!(
        count(&stdout, "route: corrupt"),
        1,
        "the at-version (2) non-conformant ADR must route `corrupt`; stdout:\n{stdout}",
    );
}

/// (file-attribution, M36) Two committed ADRs made non-conformant by the same reshaped schema
/// each surface one `required-field-present` break — and each **rendered finding line names
/// its own doc address** (`decisions/<slug>.md`), never the sibling's. Pre-attribution the two
/// message lines were byte-identical and named no doc at all, so the adoption ingest→validate
/// path could not attribute a finding to its doc. The assertion isolates the `blocking · code —
/// message` line (which the `route:` line — that already carries the doc — never matches), so the
/// doc address must come from the *message* the engine attributed.
#[test]
fn store_sweep_attributes_each_conformance_finding_to_its_own_doc() {
    let repo = TempDir::new("attribute");
    let home = TempDir::new("home");
    let pack = TempDir::new("attribute-pack");
    setup_repo(repo.path(), home.path());
    commit_adr(repo.path(), "alpha-decision", "Alpha decision", Some(1));
    commit_adr(repo.path(), "beta-decision", "Beta decision", Some(1));
    reshape_adr_pack(pack.path());

    let out = jigc_with_pack(repo.path(), home.path(), Some(pack.path()), &["validate"]);
    let stdout = String::from_utf8_lossy(&out.stdout);
    let stderr = String::from_utf8_lossy(&out.stderr);
    // Both ADRs are stamped v1 under the v2 manifest, so this corpus is unmigrated and the sweep
    // exits non-zero (M42's third exit-flipping exception); attribution — what this arm owns —
    // is unaffected.
    assert!(
        !out.status.success(),
        "an unmigrated corpus flips `jigc validate`'s exit; stdout:\n{stdout}\nstderr:\n{stderr}",
    );

    // The `blocking · code — message` message lines (the route line, which already names the
    // doc, does not contain the code, so it is filtered out): each must name its own doc.
    let message_lines: Vec<&str> = stdout
        .lines()
        .filter(|l| l.contains("schema-conformance.required-field-present"))
        .collect();
    assert_eq!(
        message_lines.len(),
        2,
        "one required-field-present message line per committed ADR; stdout:\n{stdout}",
    );
    let alpha = message_lines
        .iter()
        .find(|l| l.contains("alpha-decision.md"))
        .unwrap_or_else(|| panic!("alpha's finding line must name its own doc; stdout:\n{stdout}"));
    let beta = message_lines
        .iter()
        .find(|l| l.contains("beta-decision.md"))
        .unwrap_or_else(|| panic!("beta's finding line must name its own doc; stdout:\n{stdout}"));
    assert!(
        !alpha.contains("beta-decision"),
        "alpha's finding line must not name the sibling `beta`: {alpha}",
    );
    assert!(
        !beta.contains("alpha-decision"),
        "beta's finding line must not name the sibling `alpha`: {beta}",
    );
}

/// (conformant) Under the **shipped** schema the committed ADRs conform — no
/// schema-conformance finding, no route line, exit 0. The false-positive guard for the
/// routing path.
#[test]
fn store_sweep_clean_and_unrouted_on_conformant_store() {
    let repo = TempDir::new("clean");
    let home = TempDir::new("home");
    setup_repo(repo.path(), home.path());
    commit_adr(repo.path(), "alpha-decision", "Alpha decision", Some(2));

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
