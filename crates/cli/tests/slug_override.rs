//! End-to-end integration test for the `--slug` override on `jigc start` and
//! `jigc doc create` (M39 Increment 7, T2).
//!
//! Drives the built `jigc` binary against a throwaway temp git repo and asserts
//! (`DECISIONS.md` 2026-07-06 M39 planning → Slug (G6); `design/write-commands.md`
//! → `jigc rename`'s `--slug` precedent): an explicit `--slug` **drives** the minted
//! id verbatim (never the capped-intent slug, never silently re-slugified); a
//! colliding override rejects through the settled serial-collision / instance-
//! collision route; a malformed override is rejected with a route; and the override
//! is **inert** on a non-minting compose path (the router), never an error.
//!
//! No external test crates: the binary path comes from Cargo's `CARGO_BIN_EXE_jigc`,
//! the temp repo is a real `git init` (minting reads HEAD), and a self-cleaning
//! `TempDir` keeps the test off the developer's real repo / `~/.config`.

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

/// A throwaway directory that removes itself on drop.
struct TempDir(PathBuf);

impl TempDir {
    fn new(tag: &str) -> Self {
        let mut path = std::env::temp_dir();
        let unique = format!(
            "jigc-slug-override-{tag}-{}-{:?}",
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

/// A real git repo with one commit and the `.jigc/config/` project layer (so the
/// cascade resolves), plus a `$HOME` temp dir. No task started yet.
fn init_repo() -> (TempDir, TempDir) {
    let repo = TempDir::new("repo");
    let home = TempDir::new("home");
    let git = |args: &[&str]| {
        let out = Command::new("git")
            .args(args)
            .current_dir(repo.path())
            .output()
            .expect("run git");
        assert!(
            out.status.success(),
            "git {args:?} failed: {}",
            String::from_utf8_lossy(&out.stderr)
        );
    };
    git(&["init", "-q"]);
    git(&["config", "user.email", "test@example.com"]);
    git(&["config", "user.name", "Test"]);
    fs::write(repo.path().join("README.md"), "hello\n").expect("write file");
    git(&["add", "."]);
    git(&["commit", "-q", "-m", "initial"]);
    fs::create_dir_all(repo.path().join(".jigc").join("config")).expect("create project layer");
    (repo, home)
}

/// Run `jigc <args>` with `cwd = repo`, capturing stdout+stderr+status.
fn jigc(repo: &Path, home: &Path, args: &[&str]) -> std::process::Output {
    Command::new(env!("CARGO_BIN_EXE_jigc"))
        .args(args)
        .current_dir(repo)
        .env("HOME", home)
        .output()
        .expect("run jigc")
}

fn stderr(out: &std::process::Output) -> String {
    String::from_utf8_lossy(&out.stderr).into_owned()
}

fn stdout(out: &std::process::Output) -> String {
    String::from_utf8_lossy(&out.stdout).into_owned()
}

/// (1) `jigc start "<long intent>" --slug <explicit>` mints the task under
/// `<explicit>` — never the capped-intent slug the bare intent would produce.
#[test]
fn start_slug_override_mints_under_the_explicit_id() {
    let (repo, home) = init_repo();
    // A long intent whose bare slug caps at five words (slug.rs → `MAX_WORDS`):
    // "move the session cache to a shared redis cluster" -> "move-the-session-cache-to".
    let long = "move the session cache to a shared redis cluster and tune eviction";
    let out = jigc(
        repo.path(),
        home.path(),
        &[
            "start",
            "--workflow",
            "single-task",
            long,
            "--slug",
            "redis-cache",
        ],
    );
    assert!(
        out.status.success(),
        "`jigc start --slug` must mint; stderr:\n{}",
        stderr(&out)
    );
    // Minted under the explicit slug …
    assert!(
        repo.path().join(".jigc/tasks/redis-cache").is_dir(),
        "task must be minted under the explicit --slug id"
    );
    // … and NOT under the capped-intent slug.
    assert!(
        !repo
            .path()
            .join(".jigc/tasks/move-the-session-cache-to")
            .exists(),
        "the capped-intent slug must not be minted when --slug overrides it"
    );
    // The composed Run lines the agent would execute carry the override id verbatim.
    let view = stdout(&out);
    assert!(
        view.contains("--task redis-cache"),
        "the emitted composed view must drive the override id: {view}"
    );
}

/// (2) A `--slug` equal to an active task rejects via the serial-collision
/// reject/resume route (the same route a bare-intent id-collision takes).
#[test]
fn start_slug_override_collision_rejects_via_serial_route() {
    let (repo, home) = init_repo();
    let first = jigc(
        repo.path(),
        home.path(),
        &[
            "start",
            "--workflow",
            "single-task",
            "first intent",
            "--slug",
            "shared-id",
        ],
    );
    assert!(first.status.success(), "first mint: {}", stderr(&first));

    let dup = jigc(
        repo.path(),
        home.path(),
        &[
            "start",
            "--workflow",
            "single-task",
            "a different intent",
            "--slug",
            "shared-id",
        ],
    );
    assert!(
        !dup.status.success(),
        "a --slug colliding with an active task must reject"
    );
    let err = stderr(&dup);
    assert!(
        err.contains("shared-id"),
        "the block must name the colliding id: {err}"
    );
    assert!(
        err.contains("jigc start --task shared-id") || err.contains("jigc task discard shared-id"),
        "the block must carry the resume/discard route: {err}"
    );
}

/// A malformed `--slug` is **rejected with a route**, never silently re-slugified
/// (the settled discipline: an explicit identity is taken verbatim or refused).
#[test]
fn start_slug_override_malformed_rejects_without_reslugifying() {
    let (repo, home) = init_repo();
    let out = jigc(
        repo.path(),
        home.path(),
        &[
            "start",
            "--workflow",
            "single-task",
            "an intent",
            "--slug",
            "Bad Slug!",
        ],
    );
    assert!(
        !out.status.success(),
        "a malformed --slug must reject, never silently re-slugify"
    );
    let err = stderr(&out);
    assert!(
        err.contains("slug"),
        "the message names the bad slug: {err}"
    );
    // Never minted under a silently-corrected form.
    assert!(!repo.path().join(".jigc/tasks/bad-slug").exists());
    assert!(!repo.path().join(".jigc/tasks/an-intent").exists());
}

/// The override is **inert** on a non-minting compose path — bare `jigc start
/// "<intent>"` composes the cascade default (the router, `creates-task: false`),
/// mints nothing, so `--slug` is silently ignored, never an error (the omitting-
/// context guard: the knob a command path reads must be inert where no mint happens).
#[test]
fn start_slug_override_is_inert_when_no_task_is_minted() {
    let (repo, home) = init_repo();
    let out = jigc(
        repo.path(),
        home.path(),
        &["start", "an intent", "--slug", "unused-id"],
    );
    assert!(
        out.status.success(),
        "a --slug on a non-minting (router) compose must be inert, not an error:\n{}",
        stderr(&out)
    );
    assert!(
        !repo.path().join(".jigc/tasks/unused-id").exists(),
        "the router mints no task, so --slug materializes nothing"
    );
}

/// (3) `jigc doc create adr --title "<X>" --slug <explicit>` mints `adr:<explicit>`
/// (id decoupled from the title), and a second `--slug` over the same-identity staged
/// instance is the agent-initiated create-gate: it acks `existed` and binds the role
/// (copied-in for update), never rejects — the repairing action stays open (M45 Inc 5
/// T2; the gated serial-collision routed the agent away from the only repairing action).
#[test]
fn doc_create_slug_override_mints_and_recreate_acks_existed() {
    let (repo, home) = init_repo();
    let started = jigc(
        repo.path(),
        home.path(),
        &[
            "start",
            "--workflow",
            "single-task",
            "add a cache",
            "--slug",
            "cache-task",
        ],
    );
    assert!(started.status.success(), "start: {}", stderr(&started));

    let created = jigc(
        repo.path(),
        home.path(),
        &[
            "doc",
            "create",
            "adr",
            "--title",
            "Some Long Decision Title Here",
            "--slug",
            "my-adr",
            "--task",
            "cache-task",
        ],
    );
    assert!(created.status.success(), "create: {}", stderr(&created));
    assert_eq!(
        stdout(&created).trim(),
        "adr:my-adr",
        "create prints the explicit-slug address (id decoupled from the title)"
    );
    assert!(
        repo.path()
            .join(".jigc/tasks/cache-task/docs/adr:my-adr.md")
            .is_file(),
        "the adr must be staged under the explicit slug"
    );

    // A second create with the same --slug is the agent-initiated create-gate over a
    // same-identity staged copy: it acks `existed` and binds the role (copied-in for
    // update), never rejects — the repairing action stays open (M45 Inc 5 T2).
    let dup = jigc(
        repo.path(),
        home.path(),
        &[
            "doc",
            "create",
            "adr",
            "--title",
            "Another",
            "--slug",
            "my-adr",
            "--task",
            "cache-task",
        ],
    );
    assert!(
        dup.status.success(),
        "a --slug re-create over a staged instance acks existed, not reject: {}",
        stderr(&dup)
    );
    let out = stdout(&dup);
    assert!(
        out.contains("adr:my-adr") && out.contains("already existed"),
        "the ack names the address and says it already existed: {out}"
    );
}
