//! Acceptance — the spec `derived-from → prd` forward-ref resolves at finalize
//! (M33 Increment 2, T1).
//!
//! The prd→spec edge of the frozen doctype graph is modeled **spec-side**
//! (settled M9 — `design/project-setup.md`, `design/document-type-schema.md`): the
//! forward `derived-from` ref is stored on the spec; the PRD's `has-specs` inverse
//! is derived, never stored. This drives the built `jigc` binary through the same
//! finalize forward-ref gate the `adr.supersedes` (`superseding_decision`) and
//! `commit.implements` (`implement_from_spec`) paths prove:
//!
//! - **Resolves** — a `plan` task authors `spec:<slug>` with `derived-from: prd:<slug>`
//!   pointing at a committed `docs/prds/<slug>.md`; finalize's edge walk finds the
//!   target in the committed store, so the spec is promoted in exactly one commit and
//!   the committed spec carries its forward `derived-from` edge.
//! - **Dangling** — `derived-from` points at a PRD in neither surface; the
//!   `schema-conformance.ref-resolves` walk blocks finalize non-zero, naming the
//!   dangling target + the three routing options (fix / create-in-task / drop), and
//!   creates NO commit.
//!
//! The `derived-from` ref carries `card: "0..1"` — the deadlock guard (`DECISIONS.md`
//! 2026-06-24): a PRD-less spec stays valid (the field is never author-required), so a
//! spec authored WITHOUT `derived-from` still finalizes; the edge is only walked when
//! present.
//!
//! No external test crates: the binary path comes from `CARGO_BIN_EXE_jigc`, the temp
//! repo is a real `git init`, and a self-cleaning `TempDir` keeps the test off the
//! developer's repo.

use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

/// A throwaway directory that removes itself on drop.
struct TempDir(PathBuf);

impl TempDir {
    fn new(tag: &str) -> Self {
        let mut path = std::env::temp_dir();
        let unique = format!(
            "jigc-derived-from-{tag}-{}-{:?}",
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

/// Run `jigc doc <args>`, optionally piping `stdin`, capturing output.
fn jigc_doc(repo: &Path, home: &Path, args: &[&str], stdin: Option<&[u8]>) -> std::process::Output {
    let mut command = Command::new(env!("CARGO_BIN_EXE_jigc"));
    command.arg("doc").args(args);
    command.current_dir(repo).env("HOME", home);
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

/// A minimal, conformant `prd` body (no front-matter — `prd` has no header section):
/// a `vision` slot, an empty `requirements` repeatable, a `context` slot.
const PRD_BODY: &str = "\
# Payments platform

## Vision

A unified platform for accepting payments.

## Requirements

## Context

We must accept multiple payment methods without per-method rework.
";

/// Commit a conformant PRD at its canonical `docs/prds/payments-platform.md` so the
/// spec's `derived-from` ref has a target to resolve to (surface a — the committed
/// store). Returns the prd's `<type>:<slug>` address.
fn commit_prd(repo: &Path) -> &'static str {
    let dir = repo.join("docs").join("prds");
    fs::create_dir_all(&dir).expect("mk docs/prds/");
    fs::write(dir.join("payments-platform.md"), PRD_BODY).expect("write prd");
    git(repo, &["add", "."]);
    git(repo, &["commit", "-q", "-m", "seed prd"]);
    "prd:payments-platform"
}

/// Inject a `derived-from: <target>` line into the staged spec's empty front-matter
/// block. The optional `derived-from` ref (card `0..1`) is absent from the created
/// skeleton; injecting the canonical `key: value` line directly keeps this test's
/// concern the finalize forward-ref gate, not the write-path field-generation concern
/// (mirrors `superseding_decision::inject_supersedes`).
fn inject_derived_from(repo: &Path, task: &str, slug: &str, target: &str) {
    let staged = repo
        .join(".jigc")
        .join("tasks")
        .join(task)
        .join("docs")
        .join(format!("spec:{slug}.md"));
    let body = fs::read_to_string(&staged).expect("read staged spec");
    // Insert `derived-from` before the closing fence of the (empty) header block the
    // schema's new `meta` header section renders.
    let with = body.replacen("\n---\n", &format!("\nderived-from: {target}\n---\n"), 1);
    assert_ne!(
        body, with,
        "the staged spec carries a front-matter block to inject the ref into"
    );
    fs::write(&staged, &with).expect("inject derived-from ref");
}

/// Author a `plan` task that creates `spec:auth-spec`, fills its slots + the commit
/// doc, and points `derived-from` at `target`. Returns the finalize `Output`.
fn author_spec_deriving_from(repo: &Path, home: &Path, target: &str) -> std::process::Output {
    let task = "draft-the-auth-spec";
    let out = jigc(
        repo,
        home,
        &["start", "--workflow", "plan", "draft the auth spec"],
    );
    assert_ok(&out, "`jigc start --workflow plan`");

    let create = jigc_doc(
        repo,
        home,
        &["create", "spec", "--title", "Auth spec"],
        None,
    );
    assert_ok(&create, "`jigc doc create spec`");
    let spec = String::from_utf8(create.stdout)
        .expect("utf-8")
        .trim()
        .to_string();
    assert_eq!(spec, "spec:auth-spec");

    let set_slot = |addr: &str, prose: &[u8]| {
        let out = jigc_doc(
            repo,
            home,
            &["set-slot", addr, "--from-file", "-"],
            Some(prose),
        );
        assert_ok(&out, &format!("set-slot {addr}"));
    };
    set_slot("spec:auth-spec#goal", b"Authenticate users via OAuth.\n");
    set_slot(
        "spec:auth-spec#context",
        b"Sessions must survive node restarts.\n",
    );

    // The derived-from edge → the target PRD.
    inject_derived_from(repo, task, "auth-spec", target);

    let set_field = |addr: &str, value: &str| {
        let out = jigc_doc(repo, home, &["set-field", addr, "--value", value], None);
        assert_ok(&out, &format!("set-field {addr}"));
    };
    set_field(&format!("commit:{task}#type"), "docs");
    set_field(&format!("commit:{task}#scope"), "auth");
    set_slot(&format!("commit:{task}#summary"), b"draft the auth spec\n");
    set_slot(
        &format!("commit:{task}#body"),
        b"Capture the auth requirements before coding.\n",
    );

    jigc(repo, home, &["task", "finalize", task])
}

#[test]
fn spec_derived_from_resolves_when_prd_committed() {
    let repo = TempDir::new("pass-repo");
    let home = TempDir::new("pass-home");
    init_repo(repo.path());
    let prd = commit_prd(repo.path());

    let before: u32 = git(repo.path(), &["rev-list", "--count", "HEAD"])
        .parse()
        .unwrap();
    let out = author_spec_deriving_from(repo.path(), home.path(), prd);
    assert_ok(
        &out,
        "`jigc task finalize` — derived-from resolves to the committed PRD",
    );

    // Exactly one new commit on HEAD (the promoted spec).
    let after: u32 = git(repo.path(), &["rev-list", "--count", "HEAD"])
        .parse()
        .unwrap();
    assert_eq!(
        after,
        before + 1,
        "a resolving derived-from must land exactly ONE commit"
    );

    // The spec is committed at its canonical path, carrying its forward derived-from edge
    // (the prd's `has-specs` inverse is derived on read, never stored).
    let committed = Command::new("git")
        .args(["show", "HEAD:docs/specs/auth-spec.md"])
        .current_dir(repo.path())
        .output()
        .expect("git show");
    assert!(
        committed.status.success(),
        "finalize must promote + commit docs/specs/auth-spec.md; stderr:\n{}",
        String::from_utf8_lossy(&committed.stderr),
    );
    let body = String::from_utf8(committed.stdout).expect("utf-8");
    assert!(
        body.contains("derived-from: prd:payments-platform"),
        "the committed spec carries the forward derived-from edge; got:\n{body}",
    );
}

#[test]
fn dangling_derived_from_blocks_finalize_with_the_three_routing_options() {
    let repo = TempDir::new("dangle-repo");
    let home = TempDir::new("dangle-home");
    init_repo(repo.path());
    // No committed PRD: the derived-from target resolves in neither surface.

    let before: u32 = git(repo.path(), &["rev-list", "--count", "HEAD"])
        .parse()
        .unwrap();
    let out = author_spec_deriving_from(repo.path(), home.path(), "prd:nonexistent-prd");
    assert!(
        !out.status.success(),
        "a dangling derived-from must make finalize exit non-zero; stdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr),
    );
    let rendered = format!(
        "{}{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr),
    );
    assert!(
        rendered.contains("ref-resolves"),
        "the block is a schema-conformance.ref-resolves finding; got:\n{rendered}",
    );
    assert!(
        rendered.contains("prd:nonexistent-prd"),
        "the block names the dangling target; got:\n{rendered}",
    );
    // The three routing options: fix the ref / create the target in this task / drop it.
    assert!(
        rendered.contains("fix")
            && rendered.contains("create the target in this task")
            && rendered.contains("drop"),
        "the block surfaces the three routing options; got:\n{rendered}",
    );

    // No commit was created; nothing promoted.
    let after: u32 = git(repo.path(), &["rev-list", "--count", "HEAD"])
        .parse()
        .unwrap();
    assert_eq!(before, after, "a dangling-ref block must create no commit");
    assert!(
        !repo
            .path()
            .join("docs")
            .join("specs")
            .join("auth-spec.md")
            .exists(),
        "a blocked finalize promotes nothing",
    );
}
