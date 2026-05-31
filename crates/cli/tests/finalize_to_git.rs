//! End-to-end integration test for the `jigc task finalize <id>` commit boundary —
//! the one-git-commit close of flows #1 + #4 (`design/worked-examples.md` → the seven
//! phases; `design/finalize.md` → 5. Stage / 6. Commit / 7. Post-commit).
//!
//! Drives the built `jigc` binary against a throwaway temp git repo: `start` →
//! fill the commit doc (`doc set-field`/`set-slot`) + make a code change →
//! `task finalize <id>`. The clean path must:
//!
//! - exit 0,
//! - grow `git log` by exactly ONE commit whose message equals the rendered commit
//!   doc (the commit type's sink is the VCS message — `design/finalize.md` →
//!   Commit-doc rendering),
//! - stage the working-tree code change (it lands in the commit),
//! - remove `.jigc/tasks/<id>/` (phase 7),
//! - advance the file-state hashes (phase 7 best-effort).
//!
//! The abort path: if HEAD moves after `start` (a new commit), `finalize` aborts
//! non-zero with the divergence route (`design/finalize.md` → 1. Preflight) and
//! creates NO commit.
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
            "jigc-finalize-{tag}-{}-{:?}",
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

/// Initialize a real git repo with one commit and the `.jigc/config/` project layer,
/// then `jigc start "<intent>"`. Returns the repo + `$HOME` temp dirs.
fn started_repo(intent: &str) -> (TempDir, TempDir) {
    let repo = TempDir::new("repo");
    let home = TempDir::new("home");
    git(repo.path(), &["init", "-q"]);
    git(repo.path(), &["config", "user.email", "test@example.com"]);
    git(repo.path(), &["config", "user.name", "Test"]);
    fs::write(repo.path().join("README.md"), "hello\n").expect("write file");
    git(repo.path(), &["add", "."]);
    git(repo.path(), &["commit", "-q", "-m", "initial"]);
    fs::create_dir_all(repo.path().join(".jigc").join("config")).expect("create project layer");

    let out = Command::new(env!("CARGO_BIN_EXE_jigc"))
        .args(["start", intent])
        .current_dir(repo.path())
        .env("HOME", home.path())
        .output()
        .expect("run jigc start");
    assert!(
        out.status.success(),
        "`jigc start` must provision the task; stderr:\n{}",
        String::from_utf8_lossy(&out.stderr)
    );
    (repo, home)
}

/// Run `jigc doc <args>` with `cwd = repo`, optionally piping `stdin`.
fn run_doc(repo: &Path, home: &Path, args: &[&str], stdin: Option<&[u8]>) -> std::process::Output {
    let mut command = Command::new(env!("CARGO_BIN_EXE_jigc"));
    command.arg("doc");
    command.args(args);
    command.current_dir(repo).env("HOME", home);
    if stdin.is_some() {
        command.stdin(Stdio::piped());
    }
    command.stdout(Stdio::piped()).stderr(Stdio::piped());
    let mut child = command.spawn().expect("spawn the jigc binary");
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

/// Run `jigc task <args>` with `cwd = repo`.
fn run_task(repo: &Path, home: &Path, args: &[&str]) -> std::process::Output {
    Command::new(env!("CARGO_BIN_EXE_jigc"))
        .arg("task")
        .args(args)
        .current_dir(repo)
        .env("HOME", home)
        .output()
        .expect("run jigc task")
}

/// Fill every author-required field/slot of the provisioned commit doc so a
/// `finalize` over it validates clean: `type` + `scope` fields, `summary` + `body`
/// slots.
fn make_commit_conformant(repo: &Path, home: &Path, task: &str) {
    let set_field = |addr: &str, value: &str| {
        let out = run_doc(repo, home, &["set-field", addr, "--value", value], None);
        assert!(
            out.status.success(),
            "set-field {addr}={value} must succeed; stderr:\n{}",
            String::from_utf8_lossy(&out.stderr)
        );
    };
    let set_slot = |addr: &str, prose: &[u8]| {
        let out = run_doc(
            repo,
            home,
            &["set-slot", addr, "--from-file", "-"],
            Some(prose),
        );
        assert!(
            out.status.success(),
            "set-slot {addr} must succeed; stderr:\n{}",
            String::from_utf8_lossy(&out.stderr)
        );
    };
    set_field(&format!("commit:{task}#type"), "feat");
    set_field(&format!("commit:{task}#scope"), "gateway");
    set_slot(
        &format!("commit:{task}#summary"),
        b"add a per-client rate limiter\n",
    );
    set_slot(
        &format!("commit:{task}#body"),
        b"Throttle abusive clients at the gateway.\n",
    );
}

#[test]
fn finalize_makes_one_commit_with_the_rendered_message_and_cleans_up() {
    let (repo, home) = started_repo("add rate limiter");
    let task = "add-rate-limiter";

    // The agent authors a code change in the working tree (the task's work).
    fs::write(
        repo.path().join("limiter.rs"),
        "// a per-client rate limiter\n",
    )
    .expect("write code change");

    // Fill the commit doc so validate is clean.
    make_commit_conformant(repo.path(), home.path(), task);

    let log_before = git(repo.path(), &["rev-list", "--count", "HEAD"]);

    let out = run_task(repo.path(), home.path(), &["finalize", task]);
    assert!(
        out.status.success(),
        "`jigc task finalize` must exit 0; stdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr)
    );

    // Exactly one new commit.
    let log_after = git(repo.path(), &["rev-list", "--count", "HEAD"]);
    let before: u32 = log_before.parse().unwrap();
    let after: u32 = log_after.parse().unwrap();
    assert_eq!(
        after,
        before + 1,
        "finalize must produce exactly ONE new commit"
    );

    // The commit message equals the rendered commit doc.
    let message = git(repo.path(), &["log", "-1", "--format=%B"]);
    assert_eq!(
        message.trim_end(),
        "feat(gateway): add a per-client rate limiter\n\nThrottle abusive clients at the gateway.",
        "the commit message must equal the rendered commit doc; got:\n{message}"
    );

    // The code change is in the commit (staged in phase 5).
    let files = git(repo.path(), &["show", "--name-only", "--format=", "HEAD"]);
    assert!(
        files.lines().any(|l| l == "limiter.rs"),
        "the working-tree code change must land in the commit; files:\n{files}"
    );

    // The working area is gone (phase 7).
    let area = repo.path().join(".jigc").join("tasks").join(task);
    assert!(
        !area.exists(),
        "finalize must remove `.jigc/tasks/<id>/`; it still exists at {area:?}"
    );

    // The file-state hashes advanced (phase 7): the record file exists.
    let record = repo
        .path()
        .join(".jigc")
        .join("state")
        .join("file-state.json");
    assert!(
        record.exists(),
        "finalize must advance the file-state record at {record:?}"
    );
}

/// Flow #5 [4 promote] (`design/worked-examples.md` → Superseding decision, setup):
/// a task creates an ADR via the create-gate, then `finalize` **promotes** it to
/// `decisions/<slug>.md` and commits it — the persisted managed doc the MVP's
/// differentiators ride on. Asserts the committed canonical bytes equal the staged
/// ADR (the byte-stable copy) and that finalize advanced the `file-state` hash for
/// the promoted path (`design/finalize.md` → 4. Promote / 7. Post-commit).
#[test]
fn finalize_promotes_a_created_adr_to_decisions() {
    let (repo, home) = started_repo("cache sessions in a single in-memory node");
    let task = "cache-sessions-in-a-single-in-memory-node";

    // The agent creates the ADR in-task (the create-gate binds it to task.decision).
    let create = run_doc(
        repo.path(),
        home.path(),
        &["create", "adr", "--title", "Single-node cache"],
        None,
    );
    assert!(
        create.status.success(),
        "`jigc doc create adr` must succeed; stderr:\n{}",
        String::from_utf8_lossy(&create.stderr)
    );
    let adr_addr = String::from_utf8(create.stdout)
        .expect("utf-8")
        .trim()
        .to_string();
    assert_eq!(adr_addr, "adr:single-node-cache");

    // Fill the ADR's author-required slots so validate is clean.
    let set_slot = |addr: &str, prose: &[u8]| {
        let out = run_doc(
            repo.path(),
            home.path(),
            &["set-slot", addr, "--from-file", "-"],
            Some(prose),
        );
        assert!(
            out.status.success(),
            "set-slot {addr} must succeed; stderr:\n{}",
            String::from_utf8_lossy(&out.stderr)
        );
    };
    set_slot(
        "adr:single-node-cache#context",
        b"Session lookups must stay sub-millisecond.\n",
    );
    set_slot(
        "adr:single-node-cache#decision",
        b"A single in-memory node keeps session lookups sub-millisecond.\n",
    );
    set_slot(
        "adr:single-node-cache#consequences",
        b"A cold node loses its sessions; clients re-authenticate.\n",
    );

    // Fill the commit doc so the whole task validates clean.
    make_commit_conformant(repo.path(), home.path(), task);

    // The staged ADR's canonical bytes (the byte-stable source the copy must match).
    let staged_adr = repo
        .path()
        .join(".jigc")
        .join("tasks")
        .join(task)
        .join("docs")
        .join("adr:single-node-cache.md");
    let staged_bytes = fs::read(&staged_adr).expect("read the staged ADR");

    let out = run_task(repo.path(), home.path(), &["finalize", task]);
    assert!(
        out.status.success(),
        "`jigc task finalize` must exit 0; stdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr)
    );

    // The ADR is committed at its canonical path with the byte-stable promoted bytes.
    let committed = Command::new("git")
        .args(["show", "HEAD:decisions/single-node-cache.md"])
        .current_dir(repo.path())
        .output()
        .expect("run git show");
    assert!(
        committed.status.success(),
        "`git show HEAD:decisions/single-node-cache.md` must succeed; stderr:\n{}",
        String::from_utf8_lossy(&committed.stderr)
    );
    assert_eq!(
        committed.stdout, staged_bytes,
        "the promoted ADR's committed bytes must equal the staged source (copy is byte-stable)"
    );

    // Phase 7: the file-state record carries the promoted doc's hash.
    let record = repo
        .path()
        .join(".jigc")
        .join("state")
        .join("file-state.json");
    let record_json = fs::read_to_string(&record).expect("read the file-state record");
    assert!(
        record_json.contains("decisions/single-node-cache.md"),
        "finalize must record the promoted ADR's file-state hash; got:\n{record_json}"
    );

    // The working area is gone (phase 7).
    let area = repo.path().join(".jigc").join("tasks").join(task);
    assert!(
        !area.exists(),
        "finalize must remove `.jigc/tasks/<id>/`; it still exists at {area:?}"
    );
}

/// Flow #5 → The dangling variant (`design/worked-examples.md` → The dangling variant
/// — finalize blocks): a task creates an ADR whose `supersedes` points at a target that
/// exists in **neither** the committed store nor this task's working area. The
/// `schema-conformance.ref-resolves` forward-ref integrity check (now part of the
/// `validate_task` sweep `finalize` gates on) blocks the commit non-zero, surfacing the
/// three routing options (fix / create-in-task / drop) and creating NO commit.
#[test]
fn finalize_blocks_on_a_dangling_supersedes_ref() {
    let (repo, home) = started_repo("supersede the cache decision");
    let task = "supersede-the-cache-decision";

    // Create the ADR in-task and fill its author-required slots.
    let create = run_doc(
        repo.path(),
        home.path(),
        &["create", "adr", "--title", "Shared redis session cache"],
        None,
    );
    assert!(
        create.status.success(),
        "`jigc doc create adr` must succeed; stderr:\n{}",
        String::from_utf8_lossy(&create.stderr)
    );

    let set_slot = |addr: &str, prose: &[u8]| {
        let out = run_doc(
            repo.path(),
            home.path(),
            &["set-slot", addr, "--from-file", "-"],
            Some(prose),
        );
        assert!(
            out.status.success(),
            "set-slot {addr} must succeed; stderr:\n{}",
            String::from_utf8_lossy(&out.stderr)
        );
    };
    set_slot(
        "adr:shared-redis-session-cache#context",
        b"A single node is a single point of failure.\n",
    );
    set_slot(
        "adr:shared-redis-session-cache#decision",
        b"Replicate the session cache across nodes.\n",
    );
    set_slot(
        "adr:shared-redis-session-cache#consequences",
        b"Slightly higher write latency for resilience.\n",
    );

    // Point `supersedes` at a target that exists nowhere reachable — the dangling ref.
    // The optional `supersedes` ref is absent from the created skeleton; inject it into
    // the staged ADR's front-matter directly (generating an absent optional field line
    // through `set-field` is a separate write-path concern — this test's concern is the
    // finalize forward-ref gate). The injected line is the canonical `key: value` form.
    let staged_adr = repo
        .path()
        .join(".jigc")
        .join("tasks")
        .join(task)
        .join("docs")
        .join("adr:shared-redis-session-cache.md");
    let body = fs::read_to_string(&staged_adr).expect("read the staged ADR");
    let with_supersedes = body.replacen(
        "---\n---\n",
        "---\nsupersedes: adr:typo-nonexistent\n---\n",
        1,
    );
    assert_ne!(
        body, with_supersedes,
        "the staged ADR carries an empty front-matter block to inject the ref into"
    );
    fs::write(&staged_adr, &with_supersedes).expect("inject the dangling supersedes ref");

    // Fill the commit doc so the *only* possible block is the dangling forward-ref.
    make_commit_conformant(repo.path(), home.path(), task);

    let log_before = git(repo.path(), &["rev-list", "--count", "HEAD"]);

    let out = run_task(repo.path(), home.path(), &["finalize", task]);
    assert!(
        !out.status.success(),
        "a dangling forward-ref must make finalize exit non-zero; stdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr)
    );
    let rendered = format!(
        "{}{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr)
    );
    assert!(
        rendered.contains("adr:typo-nonexistent"),
        "the block names the dangling target; got:\n{rendered}"
    );
    // The three routing options (fix / create-in-task / drop).
    assert!(
        rendered.contains("fix")
            && rendered.contains("create the target in this task")
            && rendered.contains("drop"),
        "the block surfaces the three routing options; got:\n{rendered}"
    );

    // No commit was created.
    let log_after = git(repo.path(), &["rev-list", "--count", "HEAD"]);
    assert_eq!(
        log_before, log_after,
        "a dangling-ref block must create no commit"
    );

    // The ADR was NOT promoted to decisions/ (the transaction aborted before commit).
    assert!(
        !repo
            .path()
            .join("decisions")
            .join("shared-redis-session-cache.md")
            .exists(),
        "a blocked finalize promotes nothing"
    );
}

#[test]
fn finalize_aborts_with_no_commit_when_head_moved() {
    let (repo, home) = started_repo("add rate limiter");
    let task = "add-rate-limiter";

    fs::write(repo.path().join("limiter.rs"), "// rate limiter\n").expect("write code change");
    make_commit_conformant(repo.path(), home.path(), task);

    // HEAD moves after `start`: a human lands another commit, diverging the base pin.
    fs::write(repo.path().join("other.txt"), "unrelated\n").expect("write");
    git(repo.path(), &["add", "other.txt"]);
    git(repo.path(), &["commit", "-q", "-m", "unrelated work"]);

    let log_before = git(repo.path(), &["rev-list", "--count", "HEAD"]);

    let out = run_task(repo.path(), home.path(), &["finalize", task]);
    assert!(
        !out.status.success(),
        "a base mismatch (HEAD moved) must exit non-zero"
    );
    let rendered = format!(
        "{}{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr)
    );
    assert!(
        rendered.contains("base-mismatch") || rendered.to_lowercase().contains("base"),
        "the abort must surface the divergence route; got:\n{rendered}"
    );

    // No new commit was created by finalize.
    let log_after = git(repo.path(), &["rev-list", "--count", "HEAD"]);
    assert_eq!(
        log_before, log_after,
        "a base-mismatch abort must create no commit"
    );
}
