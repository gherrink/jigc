//! Acceptance — the store-scope `schema-completeness.inverse-cardinality` check
//! (M33 Increment 2, T2): "a PRD must have ≥ 1 spec."
//!
//! The spec→prd edge of the frozen doctype graph is modeled **spec-side** (the
//! forward `derived-from` ref is stored on the spec, `inverse: has-specs,
//! inverse-card: "1..*"`; the PRD's `has-specs` inverse is derived, never stored —
//! `design/project-setup.md`, `design/document-type-schema.md`). Inverse / minimum-
//! cardinality is **completeness, not integrity** — a PRD's specs are *other tasks'*
//! jobs — so it is **advisory by default and hard-enforced only at store / milestone
//! scope, never the per-task `finalize` gate** (`design/document-type-schema.md` →
//! Inverse-cardinality and orphan obligations; `design/validation.md` → Integrity vs
//! completeness). PRD-first / spec-later authoring must never deadlock.
//!
//! This drives the built `jigc` binary through the real store-scope `jigc validate`
//! and the per-task `jigc task finalize` (the M10 invocation-path-masking lesson:
//! drive the bytes an operator would actually run, never a reconstructed equivalent):
//!
//! - **(below minimum)** — a committed PRD with **zero** inbound `derived-from` edges
//!   surfaces **exactly one** advisory `schema-completeness.inverse-cardinality`
//!   finding at store scope, naming the deficient PRD (advisory, report-only — it never
//!   flips the sweep's exit; the fixture corpus is unstamped, so the exit *does* flip on the
//!   M42 version-currency break, asserted below so the two are never conflated).
//! - **(at minimum)** — a committed PRD with **≥1** inbound `derived-from` edge (a
//!   committed spec deriving from it) surfaces **none**.
//! - **(never a per-task gate)** — with a deficient PRD committed, a `jigc task
//!   finalize` over an unrelated task (which runs `validate_task`) emits **no**
//!   inverse-cardinality finding and finalizes clean — the completeness obligation is
//!   never the per-task gate.
//!
//! No external test crates: the binary path comes from `CARGO_BIN_EXE_jigc`, the temp
//! repo is a real `git init` + `jigc setup`, the probe is the real built `doc-code`
//! (so the `jigc validate` pre-flight resolves), and a self-cleaning `TempDir` keeps
//! the test off the developer's repo.

use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::sync::OnceLock;

/// A throwaway directory that removes itself on drop.
struct TempDir(PathBuf);

impl TempDir {
    fn new(tag: &str) -> Self {
        let mut path = std::env::temp_dir();
        path.push(format!(
            "jigc-inverse-card-{tag}-{}-{:?}",
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

/// Build the pack's `doc-code` probe once (process-wide) and return its binary path —
/// the **real** tree-sitter subprocess the engine/CLI seam drives, so the `jigc
/// validate` probe pre-flight resolves (the `validate_envelope.rs` idiom).
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

/// Run `jigc <args>` with `cwd = repo`, `$HOME = home`, and the real `doc-code` probe
/// selected via `JIGC_DOC_CODE_PROBE` so the validate pre-flight resolves.
fn jigc(repo: &Path, home: &Path, args: &[&str]) -> std::process::Output {
    Command::new(env!("CARGO_BIN_EXE_jigc"))
        .args(args)
        .current_dir(repo)
        .env("HOME", home)
        .env("JIGC_DOC_CODE_PROBE", doc_code_probe())
        .output()
        .expect("run the jigc binary")
}

/// Run `jigc doc <args>`, optionally piping `stdin`, capturing output.
fn jigc_doc(repo: &Path, home: &Path, args: &[&str], stdin: Option<&[u8]>) -> std::process::Output {
    let mut command = Command::new(env!("CARGO_BIN_EXE_jigc"));
    command.arg("doc").args(args);
    command
        .current_dir(repo)
        .env("HOME", home)
        .env("JIGC_DOC_CODE_PROBE", doc_code_probe());
    if stdin.is_some() {
        command.stdin(Stdio::piped());
    }
    command.stdout(Stdio::piped()).stderr(Stdio::piped());
    let mut child = command.spawn().expect("spawn jigc");
    if let Some(bytes) = stdin {
        child
            .stdin
            .take()
            .expect("stdin piped")
            .write_all(bytes)
            .expect("write stdin");
    }
    child.wait_with_output().expect("wait for jigc")
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

/// Make `root` a real git repo with identity, then run `jigc setup` over it (the
/// project layer + probe extraction). Returns once the store is a clean, set-up repo.
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

/// A minimal, conformant `prd` body (no front-matter — `prd` has no header section).
const PRD_BODY: &str = "\
# Payments platform

## Vision

A unified platform for accepting payments.

## Requirements

## Context

We must accept multiple payment methods without per-method rework.
";

/// Commit a conformant PRD at its canonical `docs/prds/payments-platform.md`. Returns
/// its `<type>:<slug>` identity. With no committed spec deriving from it, the PRD has
/// **zero** inbound `derived-from` edges (below the `1..*` inverse-card minimum).
fn commit_prd(repo: &Path) -> &'static str {
    let dir = repo.join("docs").join("prds");
    fs::create_dir_all(&dir).expect("mk docs/prds/");
    fs::write(dir.join("payments-platform.md"), PRD_BODY).expect("write prd");
    git(repo, &["add", "."]);
    git(repo, &["commit", "-q", "-m", "seed prd"]);
    "prd:payments-platform"
}

/// Commit a conformant `spec` at `docs/specs/auth-spec.md` carrying a forward
/// `derived-from: <prd>` edge, so the target PRD gains exactly one inbound edge. The
/// store-sweep edge rebuild parses this committed spec against the resolved schema and
/// extracts its `derived-from` edge — so a malformed body (no extracted edge) would
/// leave the PRD deficient and trip the finding, making this fixture self-checking.
fn commit_spec_deriving_from(repo: &Path, prd: &str) {
    let dir = repo.join("docs").join("specs");
    fs::create_dir_all(&dir).expect("mk docs/specs/");
    let body = format!(
        "---\n\
         derived-from: {prd}\n\
         ---\n\
         \n\
         # Auth spec\n\
         \n\
         ## Goal\n\
         \n\
         Authenticate users via OAuth.\n\
         \n\
         ## Context\n\
         \n\
         Sessions must survive node restarts.\n\
         \n\
         ## Criteria\n",
    );
    fs::write(dir.join("auth-spec.md"), body).expect("write spec");
    git(repo, &["add", "."]);
    git(repo, &["commit", "-q", "-m", "seed spec"]);
}

/// Count non-overlapping occurrences of `needle` in `haystack`.
fn count(haystack: &str, needle: &str) -> usize {
    haystack.matches(needle).count()
}

/// (below minimum) A committed PRD with zero inbound `derived-from` edges surfaces
/// **exactly one** advisory `schema-completeness.inverse-cardinality` finding at store
/// scope, naming the deficient PRD. (The sweep exits non-zero on the unstamped fixture corpus
/// — the M42 version-currency break — never on the advisory; both are asserted below.)
#[test]
fn store_sweep_surfaces_inverse_cardinality_for_prd_with_no_specs() {
    let repo = TempDir::new("below");
    let home = TempDir::new("home");
    setup_repo(repo.path(), home.path());
    let prd = commit_prd(repo.path());

    let out = jigc(repo.path(), home.path(), &["validate"]);
    let stdout = String::from_utf8_lossy(&out.stdout);
    let stderr = String::from_utf8_lossy(&out.stderr);

    // The exit is non-zero — the hand-committed fixture PRD is unstamped, so the corpus is
    // *unmigrated*: the M42 third exit-flipping exception (`design/validation.md` → Exit
    // semantics), never the advisory below. That an advisory content finding does not flip the
    // exit on its own is pinned over a *current* corpus in `managed_vs_foreign.rs`.
    assert_eq!(
        count(&stdout, "schema-conformance.schema-version-current"),
        1,
        "the v0-era fixture corpus is unmigrated — the reason the exit flips; \
         stdout:\n{stdout}\nstderr:\n{stderr}",
    );
    assert_eq!(
        count(&stdout, "schema-completeness.inverse-cardinality"),
        1,
        "a PRD with zero inbound derived-from edges must surface EXACTLY ONE advisory \
         inverse-cardinality finding at store scope; stdout:\n{stdout}",
    );
    assert!(
        stdout.contains(prd),
        "the finding must name the deficient PRD `{prd}`; stdout:\n{stdout}",
    );
}

/// (at minimum) A committed PRD with ≥1 inbound `derived-from` edge (a committed spec
/// deriving from it) surfaces **no** inverse-cardinality finding.
#[test]
fn store_sweep_emits_no_inverse_cardinality_when_prd_has_a_spec() {
    let repo = TempDir::new("at-min");
    let home = TempDir::new("home");
    setup_repo(repo.path(), home.path());
    let prd = commit_prd(repo.path());
    commit_spec_deriving_from(repo.path(), prd);

    let out = jigc(repo.path(), home.path(), &["validate"]);
    let stdout = String::from_utf8_lossy(&out.stdout);
    let stderr = String::from_utf8_lossy(&out.stderr);

    // As above, the exit flips on the unmigrated fixture corpus (two unstamped docs now), never
    // on a completeness finding — of which this arm must surface exactly zero.
    assert_eq!(
        count(&stdout, "schema-conformance.schema-version-current"),
        2,
        "the v0-era fixture corpus is unmigrated — the reason the exit flips; \
         stdout:\n{stdout}\nstderr:\n{stderr}",
    );
    assert!(
        !stdout.contains("schema-completeness.inverse-cardinality"),
        "a PRD with an inbound derived-from edge meets its inverse-card minimum, so NO \
         inverse-cardinality finding may surface; stdout:\n{stdout}",
    );
}

/// (never a per-task gate) With a deficient PRD committed, a `jigc task finalize` over
/// an unrelated task (which runs `validate_task`) emits **no** inverse-cardinality
/// finding and finalizes clean — the completeness obligation is never the per-task
/// gate (`design/document-type-schema.md` → never a per-task `finalize` gate).
#[test]
fn task_finalize_never_emits_inverse_cardinality() {
    let repo = TempDir::new("per-task");
    let home = TempDir::new("home");
    setup_repo(repo.path(), home.path());
    // A committed PRD below its inverse-card minimum is present throughout this task.
    commit_prd(repo.path());

    // An unrelated `single-task` that creates + finalizes an ADR — `validate_task` runs
    // over its working area, the deficient PRD sitting in the committed store.
    let out = jigc(
        repo.path(),
        home.path(),
        &[
            "start",
            "--workflow",
            "single-task",
            "cache sessions in a single in-memory node",
        ],
    );
    assert_ok(&out, "`jigc start`");
    let task = "cache-sessions-in-a-single";

    let create = jigc_doc(
        repo.path(),
        home.path(),
        &["create", "adr", "--title", "Single-node cache"],
        None,
    );
    assert_ok(&create, "`jigc doc create adr`");

    let set_slot = |addr: &str, prose: &[u8]| {
        let out = jigc_doc(
            repo.path(),
            home.path(),
            &["set-slot", addr, "--from-file", "-"],
            Some(prose),
        );
        assert_ok(&out, &format!("set-slot {addr}"));
    };
    let set_field = |addr: &str, value: &str| {
        let out = jigc_doc(
            repo.path(),
            home.path(),
            &["set-field", addr, "--value", value],
            None,
        );
        assert_ok(&out, &format!("set-field {addr}"));
    };
    set_slot(
        "adr:single-node-cache#context",
        b"Session lookups must stay sub-millisecond.\n",
    );
    set_slot(
        "adr:single-node-cache#decision",
        b"Keep sessions in a single in-memory node.\n",
    );
    set_slot(
        "adr:single-node-cache#consequences",
        b"A cold node loses its sessions.\n",
    );
    set_field(&format!("commit:{task}#type"), "feat");
    set_field(&format!("commit:{task}#scope"), "cache");
    set_slot(&format!("commit:{task}#summary"), b"cache sessions\n");
    set_slot(&format!("commit:{task}#body"), b"A cache change.\n");

    let out = jigc(repo.path(), home.path(), &["task", "finalize", task]);
    let stdout = String::from_utf8_lossy(&out.stdout);
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert_ok(
        &out,
        "`jigc task finalize` over a task with a deficient PRD committed",
    );
    assert!(
        !stdout.contains("schema-completeness.inverse-cardinality")
            && !stderr.contains("schema-completeness.inverse-cardinality"),
        "the per-task finalize gate (`validate_task`) must NEVER emit an inverse-cardinality \
         finding — completeness is a store-scope concern; stdout:\n{stdout}\nstderr:\n{stderr}",
    );
}
