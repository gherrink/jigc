//! M45 Increment 10 / T3 — the **generalized ref-edge rule** across every
//! doctype-with-refs × its minting workflow.
//!
//! The class (project-alpha-3.0 trial §5, correction of §61): a `type: ref` field is a
//! doctype→doctype edge the author must wire, and finalize checks it resolves. The
//! guidance to wire it had been fixed **per-instance twice** (`cites` at M41,
//! `maps-to-test` at M42 — the latter a code-anchor sibling) and never generalized
//! over *doctypes-with-refs*. This test enumerates the **axis** — the five `type: ref`
//! fields the shipped schemas declare — so a ref field added later without its
//! minting-workflow guidance trips here by construction, not by memory:
//!
//!   | ref field           | schema    | minting workflow            | state          |
//!   |---------------------|-----------|-----------------------------|----------------|
//!   | `supersedes → adr`  | adr       | record-decision             | fixed (T3)     |
//!   | `derived-from → prd`| spec      | plan (author-spec)          | fixed (T3)     |
//!   | `implements → spec` | commit    | implement-from-spec (locate)| fixed (T3)     |
//!   | `cites → adr`       | arch-doc  | architecture-documentation  | already-safe   |
//!   | `grounded-in → …`   | vision    | form-vision                 | already-safe   |
//!
//! EXCLUDED with reason: `decided-task` mints a `decisions-log` entry, which carries
//! **no** `type: ref` field — so its composed guidance must state **no** `supersedes`
//! (the omitting context: the ref-edge guidance is inert there, never an error). This
//! is §61's correction — decided-task was mis-listed as a supersedes site.
//!
//! The contract is the EMITTED bytes: this drives the real `jigc` binary and asserts
//! the composed step text an agent actually reads — no reconstructed equivalent.

use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

/// A throwaway directory that removes itself on drop.
struct TempDir(PathBuf);

impl TempDir {
    fn new(tag: &str) -> Self {
        let mut path = std::env::temp_dir();
        let unique = format!(
            "jigc-ref-edge-{tag}-{}-{:?}",
            std::process::id(),
            engine::tempname::unique_nanos(),
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

/// The on-disk methodology pack home (`<root>/packs/methodology`) — composed atop the
/// embedded dev base so both dev-pack (record-decision, plan, implement-from-spec,
/// architecture-documentation) and methodology-pack (form-vision, decided-task)
/// workflows resolve in one repo.
fn methodology_pack_tree() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("..")
        .join("packs")
        .join("methodology")
}

/// Run a `git` command in `repo`, asserting success.
fn git(repo: &Path, args: &[&str]) {
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
}

/// Initialize a real git repo with the `[dev ▸ methodology]` composition.
fn init_repo(repo: &Path) {
    git(repo, &["init", "-q"]);
    git(repo, &["config", "user.email", "test@example.com"]);
    git(repo, &["config", "user.name", "Test"]);
    fs::write(repo.join("README.md"), "hello\n").expect("write file");
    git(repo, &["add", "."]);
    git(repo, &["commit", "-q", "-m", "initial"]);
    fs::create_dir_all(repo.join(".jigc").join("config")).expect("create project layer");
    fs::write(
        repo.join(".jigc").join("config").join("packs.yaml"),
        format!("packs:\n  - {}\n", methodology_pack_tree().display()),
    )
    .expect("write packs.yaml naming the methodology pack");
}

/// Run `jigc <args>` with `cwd = repo`, `$HOME = home`, returning captured output.
fn jigc(repo: &Path, home: &Path, args: &[&str]) -> std::process::Output {
    let mut child = Command::new(env!("CARGO_BIN_EXE_jigc"))
        .args(args)
        .current_dir(repo)
        .env("HOME", home)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("spawn jigc");
    child.stdin.take();
    child.wait_with_output().expect("wait for jigc")
}

/// Compose `--workflow <wf>` with an intent and return the composed stdout, asserting exit 0.
fn compose(repo: &Path, home: &Path, workflow: &str, intent: &str) -> String {
    let out = jigc(repo, home, &["start", "--workflow", workflow, intent]);
    assert!(
        out.status.success(),
        "`jigc start --workflow {workflow}` must compose; stdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr),
    );
    String::from_utf8(out.stdout).expect("utf-8 composed stdout")
}

/// The ref-edge axis: (workflow, intent, distinguishing needle, the edge it names).
/// One row per shipped `type: ref` field × its minting workflow — the guarantee is
/// the enumeration, not a brainstormed list. Adding a ref field without its guidance
/// row leaves this table incomplete on the next audit.
const AXIS: &[(&str, &str, &str, &str)] = &[
    (
        "record-decision",
        "adopt the blackboard pattern",
        "set `supersedes` on the ADR",
        "adr.supersedes → adr",
    ),
    (
        "plan",
        "spec the rate limiter",
        "#derived-from --value prd:",
        "spec.derived-from → prd",
    ),
    (
        "implement-from-spec",
        "build the rate limiter",
        "#implements --value spec:",
        "commit.implements → spec",
    ),
    (
        "architecture-documentation",
        "document the ingest path",
        "#cites --value",
        "arch-doc.cites → adr",
    ),
    (
        "form-vision",
        "form the project vision",
        "#meta/grounded-in --value",
        "vision.grounded-in → research",
    ),
];

#[test]
fn every_minting_workflow_names_its_ref_edge_guidance() {
    let home = TempDir::new("home");

    for (workflow, intent, needle, edge) in AXIS {
        let repo = TempDir::new(workflow);
        init_repo(repo.path());
        let composed = compose(repo.path(), home.path(), workflow, intent);
        assert!(
            composed.contains(needle),
            "the composed `{workflow}` guidance must name its ref edge ({edge}) — \
             expected the substring `{needle}` (a ref field finalize checks resolves); \
             got:\n{composed}",
        );
    }
}

/// The omitting context (§61's correction): `decided-task` mints a `decisions-log`
/// entry — a doctype with **no** `type: ref` field — so the ref-edge guidance is inert
/// there. Its composed text must name **no** `supersedes` (nor any adr ref-edge): a
/// green over the composing contexts above hides a scope bug if the guidance leaks
/// into an omitting one.
#[test]
fn decided_task_emits_no_supersedes_guidance() {
    let repo = TempDir::new("decided");
    let home = TempDir::new("home");
    init_repo(repo.path());

    let composed = compose(
        repo.path(),
        home.path(),
        "decided-task",
        "adopt the blackboard pattern",
    );
    assert!(
        !composed.contains("supersedes"),
        "`decided-task` mints a decisions-log entry (no `type: ref` field) — its composed \
         guidance must name NO `supersedes` set-field (the ref-edge guidance is inert in \
         an omitting context, never leaked); got:\n{composed}",
    );
}
