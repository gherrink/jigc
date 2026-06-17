//! Headline acceptance (M9 Increment 1, T5 / worked-examples.md flow 11) — the
//! new-project arc end-to-end: a fresh repo develops its idea into its first
//! managed `prd`, and a **code-less** finalize promotes it into `prds/` as one
//! `docs(prd)` commit.
//!
//! Proves the full create-gate → code-less-finalize → promote substrate for the
//! M9 `prd` doctype (pure pack data): an agent-authored, persisted `prd` born via
//! the `project-setup` create-gate, and that a task that produces NO code change
//! still finalizes (the promoted prd is a non-empty diff, so the empty-commit
//! guard does NOT reject it). Drives the built `jigc` binary against a throwaway
//! temp git repo through the whole `project-setup` loop:
//!
//!   `jigc setup`
//!     → `jigc start --workflow project-setup "<idea>"`
//!     → `jigc doc create prd --title "…"`   (the create-gate mints `prd:<slug>`)
//!     → set-slot vision/context (the two fixed prose slots)
//!     → add-item + set-slot per requirement (the repeatable `requirements` section)
//!     → set-field commit#type=docs + set-slot commit#summary/body
//!     → (NO code change)
//!     → `jigc task finalize <id>`
//!
//! and asserts: (a) finalize exits 0, (b) exactly one new commit on HEAD,
//! (c) `prds/<slug>.md` is committed at its canonical path (promotion),
//! (d) the rendered commit subject carries `docs`, (e) `.jigc/tasks/<id>/` is
//! removed (the transaction completed through post-commit).
//!
//! See `design/worked-examples.md` → flow 11; `design/finalize.md` → 4. Promote /
//! Empty commit guard; `implementation/roadmap.md` → M9 Increment 1, scope bullet 5.
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
            "jigc-flow11-{tag}-{}-{:?}",
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

#[test]
fn new_project_arc_finalizes_into_one_docs_prd_commit() {
    let repo = TempDir::new("repo");
    let home = TempDir::new("home");
    init_repo(repo.path());

    // ── on-ramp: wire jigc into the fresh repo ───────────────────────────────────
    let out = jigc(repo.path(), home.path(), &["setup"]);
    assert_ok(&out, "`jigc setup`");

    // ── project-setup loop: mint the bootstrap task ──────────────────────────────
    let out = jigc(
        repo.path(),
        home.path(),
        &[
            "start",
            "--workflow",
            "project-setup",
            "build a habit tracker",
        ],
    );
    assert_ok(&out, "`jigc start --workflow project-setup`");
    let task = "build-a-habit-tracker";

    // ── create the prd via the create-gate (prd is born ONLY in `project-setup`) ──
    let create = jigc_doc(
        repo.path(),
        home.path(),
        &["create", "prd", "--title", "Habit tracker"],
        None,
    );
    assert_ok(&create, "`jigc doc create prd`");
    let prd = String::from_utf8(create.stdout)
        .expect("utf-8")
        .trim()
        .to_string();
    assert_eq!(prd, "prd:habit-tracker");

    // ── author the prd — the two fixed prose slots ───────────────────────────────
    let set_slot = |addr: &str, prose: &[u8]| {
        let out = jigc_doc(
            repo.path(),
            home.path(),
            &["set-slot", addr, "--from-file", "-"],
            Some(prose),
        );
        assert_ok(&out, &format!("set-slot {addr}"));
    };
    // Deliberately NO trailing newline on a non-terminal slot (requirements + context
    // follow it): the canonical writer must still separate the prose from the next `##`
    // heading, so flow-11 can't silently mask a fuse-onto-heading regression.
    set_slot(
        "prd:habit-tracker#vision",
        b"A tracker that turns intentions into daily streaks.",
    );

    // ── author the prd — the repeatable `requirements` section (M25 Inc 5) ────────
    // Mint each requirement, then fill its `statement` slot at the EMITTED item
    // address verbatim (the addr the agent runs next — never a reconstructed one).
    let add_requirement = |title: &str| -> String {
        let out = jigc_doc(
            repo.path(),
            home.path(),
            &[
                "add-item",
                "prd:habit-tracker#requirements",
                "--title",
                title,
            ],
            None,
        );
        assert_ok(&out, &format!("add-item requirements --title {title:?}"));
        let addr = String::from_utf8(out.stdout)
            .expect("utf-8")
            .trim_end_matches('\n')
            .to_string();
        assert!(
            addr.starts_with("prd:habit-tracker#requirements/"),
            "add-item must emit the minted requirement item address; got {addr:?}",
        );
        addr
    };
    let req_one = add_requirement("Log a habit in one tap");
    let req_two = add_requirement("Show the current streak");
    // Deliberately NO trailing newline on req_one's statement: it is NON-TERMINAL
    // (req_two's `### …` heading follows it), so the writer must still separate the
    // statement prose from the next item heading — the fuse-onto-heading masking guard
    // on the new repeatable shape.
    set_slot(
        &format!("{req_one}/statement"),
        b"Logging a habit takes a single tap from the home screen.",
    );
    set_slot(
        &format!("{req_two}/statement"),
        b"The current streak is shown front and center.\n",
    );

    set_slot(
        "prd:habit-tracker#context",
        b"Built for solo users who abandon heavyweight planners.\n",
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
    set_field(&format!("commit:{task}#scope"), "prd");
    set_slot(
        &format!("commit:{task}#summary"),
        b"capture the habit tracker prd\n",
    );
    set_slot(
        &format!("commit:{task}#body"),
        b"Develop the new-project idea into its first managed prd.\n",
    );

    // ── NO code change: the only diff is the promoted prd (the empty-commit guard
    //    is satisfied by the untracked-but-staged prd, not a code edit). ──────────
    let before: u32 = git(repo.path(), &["rev-list", "--count", "HEAD"])
        .parse()
        .unwrap();

    let out = jigc(repo.path(), home.path(), &["task", "finalize", task]);
    // (a) finalize exits zero — the empty-commit guard does NOT reject the code-less task.
    assert_ok(
        &out,
        "`jigc task finalize` — a code-less prd task must finalize cleanly",
    );

    // (b) exactly one new commit on HEAD.
    let after: u32 = git(repo.path(), &["rev-list", "--count", "HEAD"])
        .parse()
        .unwrap();
    assert_eq!(
        after,
        before + 1,
        "a prd-only finalize must land exactly ONE commit"
    );

    // (c) the prd is committed at its canonical path (promotion).
    let committed = Command::new("git")
        .args(["show", "HEAD:prds/habit-tracker.md"])
        .current_dir(repo.path())
        .output()
        .expect("git show");
    assert!(
        committed.status.success(),
        "finalize must promote + commit prds/habit-tracker.md; stderr:\n{}",
        String::from_utf8_lossy(&committed.stderr),
    );
    let files = git(repo.path(), &["show", "--name-only", "--format=", "HEAD"]);
    assert!(
        files.lines().any(|l| l == "prds/habit-tracker.md"),
        "the promoted prd must be in the commit; files:\n{files}",
    );

    // (c′) the committed prd carries the per-requirement repeatable items + their
    // statements (M25 Inc 5): the `## Requirements` section holds both minted
    // requirements as `### …` items, each followed by its own statement prose.
    let prd_body = String::from_utf8(committed.stdout).expect("utf-8 prd body");
    assert!(
        prd_body.contains("### Log a habit in one tap"),
        "the committed prd must carry the first requirement item; got:\n{prd_body}",
    );
    assert!(
        prd_body.contains("Logging a habit takes a single tap from the home screen."),
        "the committed prd must carry the first requirement's statement; got:\n{prd_body}",
    );
    assert!(
        prd_body.contains("### Show the current streak"),
        "the committed prd must carry the second requirement item; got:\n{prd_body}",
    );
    assert!(
        prd_body.contains("The current streak is shown front and center."),
        "the committed prd must carry the second requirement's statement; got:\n{prd_body}",
    );

    // (d) the rendered commit subject carries `docs`.
    let message = git(repo.path(), &["log", "-1", "--format=%s"]);
    assert!(
        message.starts_with("docs"),
        "the rendered commit subject must carry the `docs` type; got:\n{message}",
    );

    // (e) the working area is gone (the transaction completed through post-commit).
    let area = repo.path().join(".jigc").join("tasks").join(task);
    assert!(
        !area.exists(),
        "finalize must remove `.jigc/tasks/<id>/`; it still exists at {area:?}",
    );
}
