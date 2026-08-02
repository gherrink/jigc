//! Integration test for the write-time `--task`-scoped **barrier** (M8 Increment 3,
//! T2 — `design/write-commands.md` → The write-time `--task`-scoped barrier;
//! `design/storage.md` → The by-task-id join: the write-time complement to the
//! join-time isolation check).
//!
//! Drives the built `jigc` binary against a throwaway temp git repo holding a
//! milestone with **two** write-ready sub-task areas. A staging write whose
//! resolved destination would land **outside** the named `tasks/<id>/docs/` (an
//! address slug that path-escapes its own area) must be **refused at write-time**:
//! non-zero exit, a blocking finding, and nothing written outside the area — while
//! the same verb's **within-area** write of the identical doc succeeds. The barrier
//! refuses the mis-directed staging write up front, so a sub-agent physically
//! cannot stage into a sibling's area (the #7-barrier face is T5; this is the seam
//! that the destination-containment predicate is wired into the production path).

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
            "jigc-doc-barrier-{tag}-{}-{:?}",
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

/// Run `jigc <args>` with `cwd = repo` and `$HOME = home`.
fn run(repo: &Path, home: &Path, args: &[&str]) -> std::process::Output {
    Command::new(env!("CARGO_BIN_EXE_jigc"))
        .args(args)
        .current_dir(repo)
        .env("HOME", home)
        .output()
        .expect("run the jigc binary")
}

/// Run `jigc doc <args>` with `cwd = repo`, piping `stdin`.
fn run_doc_stdin(repo: &Path, home: &Path, args: &[&str], stdin: &[u8]) -> std::process::Output {
    let mut command = Command::new(env!("CARGO_BIN_EXE_jigc"));
    command.arg("doc").args(args);
    command.current_dir(repo).env("HOME", home);
    command
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());
    let mut child = command.spawn().expect("spawn the jigc binary");
    child
        .stdin
        .take()
        .expect("stdin piped")
        .write_all(stdin)
        .expect("write stdin");
    child.wait_with_output().expect("wait for jigc")
}

/// Initialize a real git repo with one commit and the `.jigc/config/` project
/// layer so the cascade resolves (milestone mint reads HEAD).
fn init_repo(root: &Path) {
    let git = |args: &[&str]| {
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
    };
    git(&["init", "-q"]);
    git(&["config", "user.email", "test@example.com"]);
    git(&["config", "user.name", "Test"]);
    fs::write(root.join("README.md"), "hello\n").expect("write file");
    git(&["add", "."]);
    git(&["commit", "-q", "-m", "initial"]);
    fs::create_dir_all(root.join(".jigc").join("config")).expect("create project layer");
}

/// A sub-task's `docs/` working area: `.jigc/tasks/<sub>/docs/`.
fn docs_area(repo: &Path, sub: &str) -> PathBuf {
    repo.join(".jigc").join("tasks").join(sub).join("docs")
}

/// A byte-faithful snapshot of an area: every file's repo-relative path → bytes,
/// sorted so equality is order-invariant. Used to prove a refused out-of-bounds
/// write leaves the sibling area **unchanged on disk** (the #7-barrier discipline:
/// the escape attempt must be a genuine no-op against real staged state).
fn snapshot_area(area: &Path) -> Vec<(PathBuf, Vec<u8>)> {
    let mut entries = Vec::new();
    let mut stack = vec![area.to_path_buf()];
    while let Some(dir) = stack.pop() {
        for entry in fs::read_dir(&dir).expect("read area dir") {
            let path = entry.expect("dir entry").path();
            if path.is_dir() {
                stack.push(path);
            } else {
                let rel = path
                    .strip_prefix(area)
                    .expect("path under area")
                    .to_path_buf();
                let bytes = fs::read(&path).expect("read area file");
                entries.push((rel, bytes));
            }
        }
    }
    entries.sort_by(|a, b| a.0.cmp(&b.0));
    entries
}

/// Stand up a milestone with two write-ready sub-task areas (each minted
/// `--workflow single-task`, then re-entered once so its commit doc is
/// provisioned). Returns `(repo, home, subA, subB)`.
fn milestone_with_two_subtasks() -> (TempDir, TempDir, &'static str, &'static str) {
    let repo = TempDir::new("repo");
    let home = TempDir::new("home");
    init_repo(repo.path());

    let created = run(
        repo.path(),
        home.path(),
        &["milestone", "create", "Cache rework"],
    );
    assert!(
        created.status.success(),
        "`milestone create` must exit 0; stderr:\n{}",
        String::from_utf8_lossy(&created.stderr),
    );

    let sub_a = "move-cache-to-redis";
    let sub_b = "evict-stale-keys";
    for intent in ["Move cache to redis", "Evict stale keys"] {
        let added = run(
            repo.path(),
            home.path(),
            &[
                "milestone",
                "add-task",
                "cache-rework",
                intent,
                "--workflow",
                "single-task",
            ],
        );
        assert!(
            added.status.success(),
            "`add-task {intent}` must exit 0; stderr:\n{}",
            String::from_utf8_lossy(&added.stderr),
        );
    }

    for sub in [sub_a, sub_b] {
        let entered = run(
            repo.path(),
            home.path(),
            &["workflow", "single-task", "--task", sub],
        );
        assert!(
            entered.status.success(),
            "first re-entry of `{sub}` must provision its commit doc; stderr:\n{}",
            String::from_utf8_lossy(&entered.stderr),
        );
    }

    (repo, home, sub_a, sub_b)
}

#[test]
fn within_area_write_succeeds_but_an_escaping_destination_is_refused() {
    let (repo, home, sub_a, sub_b) = milestone_with_two_subtasks();

    // The within-area write of the verb succeeds: it lands inside subA's `docs/`.
    let within = run_doc_stdin(
        repo.path(),
        home.path(),
        &[
            "set-slot",
            &format!("commit:{sub_a}#summary"),
            "--from-file",
            "-",
            "--task",
            sub_a,
        ],
        b"Move the cache backend to redis.\n",
    );
    assert!(
        within.status.success(),
        "the within-area `set-slot --task {sub_a}` must exit 0; stderr:\n{}",
        String::from_utf8_lossy(&within.stderr),
    );

    // The escaping write: an address slug that path-escapes subA's own `docs/`,
    // resolving to a destination under subB's real area. (The `<type>:` filename
    // prefix absorbs one leading `..`, so reaching `tasks/<subB>/docs/` from
    // `tasks/<subA>/docs/` takes three `..`.) The barrier must refuse it.
    let escape_slug = format!("../../../{sub_b}/docs/commit:pwned");
    let sentinel = b"PWNED-CROSS-AREA-PROSE\n";
    let escaping = run_doc_stdin(
        repo.path(),
        home.path(),
        &[
            "set-slot",
            &format!("commit:{escape_slug}#summary"),
            "--from-file",
            "-",
            "--task",
            sub_a,
        ],
        sentinel,
    );
    assert!(
        !escaping.status.success(),
        "a staged-doc destination outside `tasks/{sub_a}/docs/` must be refused (non-zero exit)",
    );
    let stderr = String::from_utf8_lossy(&escaping.stderr);
    assert!(
        stderr.contains("outside") && stderr.contains(sub_a),
        "the rejection must name the barrier (destination outside the sub-area); got:\n{stderr}",
    );

    // Nothing was written outside subA's area: no escaping byte reached subB, and
    // no stray file landed anywhere under subB's `docs/`.
    let escaped_path = docs_area(repo.path(), sub_b).join("commit:pwned.md");
    assert!(
        !escaped_path.exists(),
        "the barrier must write nothing at the escaped destination `{}`",
        escaped_path.display(),
    );
    for entry in fs::read_dir(docs_area(repo.path(), sub_b)).expect("read subB docs") {
        let body = fs::read(entry.expect("dir entry").path()).expect("read subB doc");
        assert!(
            !body.windows(sentinel.len()).any(|w| w == sentinel),
            "no sentinel prose may reach any doc under subB's area",
        );
    }
}

/// T5 — the #7-barrier face (hardening). Drives a write **through** the barrier from
/// the binary and attempts a write **outside** the named sub-area, asserting the
/// out-of-bounds attempt is refused with the sibling area left byte-identical on
/// disk. The discipline that makes this a genuine red→green and not a no-op: the
/// areas are provisioned **only** via `jigc workflow … --task` + the `jigc doc`
/// write path (NEVER `fs::write` into `docs/`), and the sibling holds **real staged
/// prose** (written through the binary) before the escape, so a barrier regression
/// that let the escaping write land would mutate the captured snapshot and fail.
#[test]
fn the_barrier_face_refuses_an_out_of_bounds_write_leaving_the_sibling_unchanged() {
    let (repo, home, sub_a, sub_b) = milestone_with_two_subtasks();

    // Both areas were provisioned ONLY through `jigc workflow … --task` (the
    // re-entry in the fixture) — never an `fs::write` into `docs/`. Pre-stage
    // nothing here either: the sibling's real staged state is built through the
    // binary's write path, so the barrier is exercised against genuine content.
    let prose_a = b"In-bounds: move the cache backend to redis.\n";
    let in_bounds = run_doc_stdin(
        repo.path(),
        home.path(),
        &[
            "set-slot",
            &format!("commit:{sub_a}#summary"),
            "--from-file",
            "-",
            "--task",
            sub_a,
        ],
        prose_a,
    );
    assert!(
        in_bounds.status.success(),
        "the in-bounds `set-slot --task {sub_a}` must exit 0; stderr:\n{}",
        String::from_utf8_lossy(&in_bounds.stderr),
    );

    // The in-bounds write landed ONLY inside `tasks/<subA>/docs/`: subA's commit doc
    // carries the prose, and no byte of it escaped into subB's area.
    let landed = docs_area(repo.path(), sub_a).join(format!("commit:{sub_a}.md"));
    let landed_body = fs::read_to_string(&landed).expect("read subA's commit doc");
    assert!(
        landed_body.contains("In-bounds: move the cache backend to redis."),
        "the in-bounds write must land inside `tasks/{sub_a}/docs/`; got:\n{landed_body}",
    );

    // Give subB its own genuine staged prose — through the binary — so the escape
    // attempt below has real sibling state to (fail to) corrupt.
    let prose_b = b"Sibling: evict stale keys on a TTL sweep.\n";
    let sibling = run_doc_stdin(
        repo.path(),
        home.path(),
        &[
            "set-slot",
            &format!("commit:{sub_b}#summary"),
            "--from-file",
            "-",
            "--task",
            sub_b,
        ],
        prose_b,
    );
    assert!(
        sibling.status.success(),
        "seeding subB's own staged prose must exit 0; stderr:\n{}",
        String::from_utf8_lossy(&sibling.stderr),
    );

    // Capture subB's full byte-state — the sibling whose area the escape targets.
    let before = snapshot_area(&docs_area(repo.path(), sub_b));
    assert!(
        before
            .iter()
            .any(|(_, bytes)| bytes.windows(prose_b.len()).any(|w| w == prose_b)),
        "subB's snapshot must hold its own staged prose before the escape attempt",
    );

    // The out-of-bounds attempt: a `--task <subA>` write whose address slug
    // path-escapes subA's `docs/` and resolves under subB's real area. (The
    // `<type>:` filename prefix absorbs one leading `..`, so reaching
    // `tasks/<subB>/docs/` from `tasks/<subA>/docs/` takes three `..`.)
    let escape_slug = format!("../../../{sub_b}/docs/commit:pwned");
    let sentinel = b"PWNED-BARRIER-FACE-PROSE\n";
    let escaping = run_doc_stdin(
        repo.path(),
        home.path(),
        &[
            "set-slot",
            &format!("commit:{escape_slug}#summary"),
            "--from-file",
            "-",
            "--task",
            sub_a,
        ],
        sentinel,
    );
    assert!(
        !escaping.status.success(),
        "an out-of-bounds `--task {sub_a}` write must be refused (non-zero exit)",
    );
    let stderr = String::from_utf8_lossy(&escaping.stderr);
    assert!(
        stderr.contains("outside") && stderr.contains(sub_a),
        "the blocking finding must name the barrier (destination outside `{sub_a}`'s area); got:\n{stderr}",
    );

    // The sibling area is byte-identical to before — the escaping write was a
    // genuine no-op, not merely \"the sentinel didn't reach a known file\". No new
    // file appeared, no existing byte changed.
    let after = snapshot_area(&docs_area(repo.path(), sub_b));
    assert_eq!(
        before, after,
        "the refused out-of-bounds write must leave subB's area byte-identical on disk",
    );
    assert!(
        after
            .iter()
            .all(|(_, bytes)| !bytes.windows(sentinel.len()).any(|w| w == sentinel)),
        "no sentinel byte may reach any doc under subB's area",
    );
}
