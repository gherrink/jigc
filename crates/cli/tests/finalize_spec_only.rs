//! Headline acceptance (M3 Increment 3, T3) — a spec-only, **code-less** task
//! finalizes cleanly into exactly ONE `docs` commit.
//!
//! Proves the doc-creation differentiator *beyond* `adr`: an agent-authored,
//! persisted `spec` born via the create-gate, and that a task that produces NO
//! code change still finalizes (the promoted spec is a non-empty diff, so the
//! empty-commit guard does NOT reject it). Drives the built `jigc` binary against
//! a throwaway temp git repo through the whole `plan` loop:
//!
//!   `jigc start --workflow plan "<intent>"`
//!     → `jigc doc create spec --title "…"`   (the create-gate mints `spec:<slug>`)
//!     → set-slot goal/context                 (a criteria-less-but-valid spec)
//!     → set-field commit#type=docs + set-slot commit#summary
//!     → (NO code change)
//!     → `jigc task finalize <id>`
//!
//! and asserts: (a) finalize exits 0, (b) exactly one new commit on HEAD,
//! (c) `docs/specs/<slug>.md` is committed at its canonical path (promotion),
//! (d) the rendered commit message carries `docs`, (e) the empty-commit guard
//! does NOT reject the code-less task.
//!
//! See `design/finalize.md` → 4. Promote / Empty commit; `design/worked-examples.md`
//! → flow 6 Task 1; `implementation/roadmap.md` → M3 Increment 3.
//!
//! No external test crates: the binary path comes from `CARGO_BIN_EXE_jigc`, the
//! temp repo is a real `git init`, and a self-cleaning `TempDir` keeps the test off
//! the developer's repo.

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
            "jigc-spec-only-{tag}-{}-{:?}",
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

#[test]
fn spec_only_code_less_task_finalizes_into_one_docs_commit() {
    let repo = TempDir::new("repo");
    let home = TempDir::new("home");
    init_repo(repo.path());

    // ── plan loop: mint a planning task ──────────────────────────────────────────
    let out = jigc(
        repo.path(),
        home.path(),
        &["start", "--workflow", "plan", "draft the auth spec"],
    );
    assert_ok(&out, "`jigc start --workflow plan`");
    let task = "draft-the-auth-spec";

    // ── create the spec via the create-gate (spec is born ONLY in `plan`) ─────────
    let create = jigc_doc(
        repo.path(),
        home.path(),
        &["create", "spec", "--title", "Auth spec"],
        None,
    );
    assert_ok(&create, "`jigc doc create spec`");
    let spec = String::from_utf8(create.stdout)
        .expect("utf-8")
        .trim()
        .to_string();
    assert_eq!(spec, "spec:auth-spec");

    // ── author a criteria-less-but-valid spec (goal + context slots only) ─────────
    let set_slot = |addr: &str, prose: &[u8]| {
        let out = jigc_doc(
            repo.path(),
            home.path(),
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

    // ── fill the commit doc — `type: docs` for a doc-only change ──────────────────
    let set_field = |addr: &str, value: &str| {
        let out = jigc_doc(
            repo.path(),
            home.path(),
            &["set-field", addr, "--value", value],
            None,
        );
        assert_ok(&out, &format!("set-field {addr}"));
    };
    set_field(&format!("commit:{task}#type"), "docs");
    set_field(&format!("commit:{task}#scope"), "auth");
    set_slot(&format!("commit:{task}#summary"), b"draft the auth spec\n");
    set_slot(
        &format!("commit:{task}#body"),
        b"Capture the auth requirements before coding.\n",
    );

    // ── NO code change: the only diff is the promoted spec (the empty-commit guard
    //    is satisfied by the untracked-but-staged spec, not a code edit). ──────────
    let before: u32 = git(repo.path(), &["rev-list", "--count", "HEAD"])
        .parse()
        .unwrap();

    let out = jigc(repo.path(), home.path(), &["task", "finalize", task]);
    // (a) finalize exits zero — the empty-commit guard does NOT reject the code-less task.
    assert_ok(
        &out,
        "`jigc task finalize` — a code-less spec task must finalize cleanly",
    );

    // (b) exactly one new commit on HEAD.
    let after: u32 = git(repo.path(), &["rev-list", "--count", "HEAD"])
        .parse()
        .unwrap();
    assert_eq!(
        after,
        before + 1,
        "a spec-only finalize must land exactly ONE commit"
    );

    // (c) the spec is committed at its canonical path (promotion).
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
    let files = git(repo.path(), &["show", "--name-only", "--format=", "HEAD"]);
    assert!(
        files.lines().any(|l| l == "docs/specs/auth-spec.md"),
        "the promoted spec must be in the commit; files:\n{files}",
    );

    // (d) the rendered commit message carries `docs`.
    let message = git(repo.path(), &["log", "-1", "--format=%s"]);
    assert!(
        message.starts_with("docs"),
        "the rendered commit subject must carry the `docs` type; got:\n{message}",
    );

    // The working area is gone (the transaction completed through post-commit).
    let area = repo.path().join(".jigc").join("tasks").join(task);
    assert!(
        !area.exists(),
        "finalize must remove `.jigc/tasks/<id>/`; it still exists at {area:?}",
    );
}
