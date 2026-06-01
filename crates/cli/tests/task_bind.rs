//! End-to-end integration test for `jigc task bind <role> <addr> <id>` — the
//! cross-task binding verb (`design/write-commands.md` → Binding a context role:
//! the five-step enforcement; `workflow-dialect.md` → `reads`).
//!
//! Drives the built `jigc` binary against a throwaway temp git repo. A task is
//! minted over the `bind-spec` fixture workflow (which declares
//! `reads: [{role: spec, type: spec}]`), a committed `spec` fixture lives at
//! `specs/<slug>.md`, and `jigc task bind` is exercised across every rejection of
//! the five-step enforcement plus the success + re-bind (last-write-wins) path.
//! The observable proof is the persisted `.jigc/tasks/<id>/roles.json` mapping
//! `spec -> spec:<slug>`.
//!
//! The fixture pack workflow (`bind-spec`) is a test surface only — the real pack
//! workflows do not yet declare `reads` (this increment retires the binding
//! mechanism in isolation before they ride on it).
//!
//! No external test crates: the binary path comes from Cargo's
//! `CARGO_BIN_EXE_jigc`, the temp repo is a real `git init`, and a self-cleaning
//! `TempDir` keeps the test off the developer's real repo / `~/.config`.

use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};

/// A throwaway directory that removes itself on drop.
struct TempDir(PathBuf);

impl TempDir {
    fn new(tag: &str) -> Self {
        let mut path = std::env::temp_dir();
        let unique = format!(
            "jigc-task-bind-{tag}-{}-{:?}",
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

fn git(root: &Path, args: &[&str]) {
    let out = Command::new("git")
        .args(args)
        .current_dir(root)
        .output()
        .expect("run git");
    assert!(
        out.status.success(),
        "git {args:?} failed: {}",
        String::from_utf8_lossy(&out.stderr)
    );
}

/// Initialize a real git repo with one commit and the `.jigc/config/` project
/// layer so the cascade resolves (composition mints, which reads HEAD).
fn init_repo(root: &Path) {
    git(root, &["init", "-q"]);
    git(root, &["config", "user.email", "test@example.com"]);
    git(root, &["config", "user.name", "Test"]);
    fs::write(root.join("README.md"), "hello\n").expect("write file");
    git(root, &["add", "."]);
    git(root, &["commit", "-q", "-m", "initial"]);
    fs::create_dir_all(root.join(".jigc").join("config")).expect("create project layer");
}

/// Commit a `spec` fixture at its canonical path `specs/<slug>.md` so the bind's
/// committed-store resolve (step 3) finds it. The body is a minimal conforming
/// spec — bind's resolve only needs the canonical file to exist.
fn commit_spec(root: &Path, slug: &str) {
    let specs = root.join("specs");
    fs::create_dir_all(&specs).expect("create specs/");
    fs::write(
        specs.join(format!("{slug}.md")),
        "# Cache the session store\n\n## Goal\n\nMove sessions to redis.\n",
    )
    .expect("write spec fixture");
    git(root, &["add", "."]);
    git(root, &["commit", "-q", "-m", "add spec"]);
}

/// Commit an `adr` fixture at its canonical path `decisions/<slug>.md`. Used to
/// give the doctype-mismatch step (4) a target that resolves in the committed
/// store (step 3) yet has the wrong doctype for the `spec` role.
fn commit_adr(root: &Path, slug: &str) {
    let decisions = root.join("decisions");
    fs::create_dir_all(&decisions).expect("create decisions/");
    fs::write(
        decisions.join(format!("{slug}.md")),
        "# Some decision\n\n## Decision\n\nWe decided.\n",
    )
    .expect("write adr fixture");
    git(root, &["add", "."]);
    git(root, &["commit", "-q", "-m", "add adr"]);
}

/// Run `jigc <args>` with `cwd = repo` and `$HOME = home`.
fn run(repo: &Path, home: &Path, args: &[&str]) -> Output {
    Command::new(env!("CARGO_BIN_EXE_jigc"))
        .args(args)
        .current_dir(repo)
        .env("HOME", home)
        .output()
        .expect("run the jigc binary")
}

fn stderr_of(out: &Output) -> String {
    String::from_utf8_lossy(&out.stderr).into_owned()
}

#[test]
fn task_bind_enforces_the_five_steps_and_records_the_binding() {
    let repo = TempDir::new("repo");
    init_repo(repo.path());
    let home = TempDir::new("home");
    let slug = "cache-the-session-store";

    // Mint a task over the `bind-spec` fixture workflow (declares
    // `reads: [{role: spec, type: spec}]`).
    let mint = run(
        repo.path(),
        home.path(),
        &["start", "--workflow", "bind-spec", "implement the spec"],
    );
    assert!(
        mint.status.success(),
        "the bind-spec mint must succeed; stderr:\n{}",
        stderr_of(&mint),
    );
    let task_id = "implement-the-spec";

    // Rejection 1 — no active task: bind against an id with no working area.
    let no_task = run(
        repo.path(),
        home.path(),
        &[
            "task",
            "bind",
            "spec",
            "spec:cache-the-session-store",
            "ghost",
        ],
    );
    assert!(
        !no_task.status.success(),
        "binding against a nonexistent task must fail",
    );
    assert!(
        stderr_of(&no_task).contains("ghost"),
        "the no-active-task rejection must name the missing task; got:\n{}",
        stderr_of(&no_task),
    );

    // Rejection 2 — role not declared in the workflow's `reads`: the rejection
    // lists the declared roles so the agent sees what is bindable.
    let bad_role = run(
        repo.path(),
        home.path(),
        &[
            "task",
            "bind",
            "design",
            "spec:cache-the-session-store",
            task_id,
        ],
    );
    assert!(
        !bad_role.status.success(),
        "binding an undeclared role must fail",
    );
    let bad_role_err = stderr_of(&bad_role);
    assert!(
        bad_role_err.contains("design") && bad_role_err.contains("spec"),
        "the undeclared-role rejection must name the bad role and list the declared ones (spec); \
         got:\n{bad_role_err}",
    );

    // Rejection 3 — `<addr>` does not resolve in the committed store (no spec
    // committed yet): `no such doc <addr>`.
    let no_doc = run(
        repo.path(),
        home.path(),
        &[
            "task",
            "bind",
            "spec",
            "spec:cache-the-session-store",
            task_id,
        ],
    );
    assert!(
        !no_doc.status.success(),
        "binding an addr absent from the committed store must fail",
    );
    let no_doc_err = stderr_of(&no_doc);
    assert!(
        no_doc_err.contains("no such doc") && no_doc_err.contains("spec:cache-the-session-store"),
        "the unresolved-addr rejection must read `no such doc <addr>`; got:\n{no_doc_err}",
    );

    // Now commit the spec fixture so the store resolve succeeds for the rest, plus
    // an ADR fixture so the doctype-mismatch step (4) is reached — its addr must
    // resolve in the committed store (step 3 passes) yet carry the wrong doctype.
    commit_spec(repo.path(), slug);
    commit_adr(repo.path(), "some-decision");
    // The task is pinned to its base; bind reads the committed store at HEAD, but
    // committing the spec advanced HEAD. Re-mint the task on the new HEAD so the
    // store resolve sees the committed spec. (bind itself does not touch the base
    // pin — the doctype/store checks read the committed store, not the task base.)
    run(repo.path(), home.path(), &["task", "discard", task_id]);
    let remint = run(
        repo.path(),
        home.path(),
        &["start", "--workflow", "bind-spec", "implement the spec"],
    );
    assert!(
        remint.status.success(),
        "the re-mint on the spec-bearing HEAD must succeed; stderr:\n{}",
        stderr_of(&remint),
    );

    // Rejection 4 — doctype mismatch: the role `spec` declares type `spec`, but
    // the addr names an `adr`. The committed-store resolve and the declared-type
    // check disagree → reject with the mismatch.
    let mismatch = run(
        repo.path(),
        home.path(),
        &["task", "bind", "spec", "adr:some-decision", task_id],
    );
    assert!(
        !mismatch.status.success(),
        "binding an addr of the wrong doctype must fail",
    );
    let mismatch_err = stderr_of(&mismatch);
    assert!(
        mismatch_err.contains("adr") && mismatch_err.contains("spec"),
        "the doctype-mismatch rejection must name both the target type (adr) and the declared \
         one (spec); got:\n{mismatch_err}",
    );

    // Success — a committed spec at the declared role + matching doctype binds.
    let ok = run(
        repo.path(),
        home.path(),
        &[
            "task",
            "bind",
            "spec",
            "spec:cache-the-session-store",
            task_id,
        ],
    );
    assert!(
        ok.status.success(),
        "binding a committed spec to the declared `spec` role must succeed; stderr:\n{}",
        stderr_of(&ok),
    );

    // The binding landed on disk: roles.json maps `spec -> spec:<slug>`.
    let roles_path = repo
        .path()
        .join(".jigc")
        .join("tasks")
        .join(task_id)
        .join("roles.json");
    let roles = fs::read_to_string(&roles_path).expect("roles.json written by bind");
    assert!(
        roles.contains("\"spec\": \"spec:cache-the-session-store\""),
        "roles.json must bind spec -> the committed spec; got:\n{roles}",
    );

    // Re-bind overwrites (last-write-wins). Commit a second spec, re-mint on that
    // HEAD, and re-bind the same role to the new addr.
    commit_spec(repo.path(), "rate-limit-the-api");
    run(repo.path(), home.path(), &["task", "discard", task_id]);
    run(
        repo.path(),
        home.path(),
        &["start", "--workflow", "bind-spec", "implement the spec"],
    );
    // First bind to spec A, then re-bind to spec B.
    let first = run(
        repo.path(),
        home.path(),
        &[
            "task",
            "bind",
            "spec",
            "spec:cache-the-session-store",
            task_id,
        ],
    );
    assert!(first.status.success(), "first bind must succeed");
    let rebind = run(
        repo.path(),
        home.path(),
        &["task", "bind", "spec", "spec:rate-limit-the-api", task_id],
    );
    assert!(
        rebind.status.success(),
        "the re-bind must succeed; stderr:\n{}",
        stderr_of(&rebind),
    );
    let roles = fs::read_to_string(&roles_path).expect("roles.json after re-bind");
    assert!(
        roles.contains("\"spec\": \"spec:rate-limit-the-api\""),
        "the re-bind must overwrite (last-write-wins); got:\n{roles}",
    );
    assert!(
        !roles.contains("cache-the-session-store"),
        "the prior binding must be gone after the overwrite; got:\n{roles}",
    );
}
