//! Headline acceptance — the superseding-decision flow end-to-end (flows #5 + #2).
//!
//! The mandated differentiator proof (`CLAUDE.md` → MVP scope; `design/worked-examples.md`
//! → 5. Superseding decision; `implementation/roadmap.md` → Increment 5 Proves). Drives
//! the built `jigc` binary against a throwaway temp git repo through the WHOLE inc-5
//! stack — the committed-store reader, the edge index (committed rebuild + working
//! overlay), the context-slice over a persisted ADR, the create-gate + promotion, and
//! the forward-ref integrity walk at finalize:
//!
//! - **Setup (task 1)** creates + finalizes `adr:single-node-cache`, the ADR that gets
//!   superseded — asserts `docs/decisions/single-node-cache.md` is committed at its canonical
//!   path.
//! - **Supersede + re-compose (task 2)** creates `adr:shared-redis-session-cache`, sets
//!   `supersedes: adr:single-node-cache`, and re-composes (`jigc start --task <id>`): the
//!   `superseded-context` step's `{{@task.decision.supersedes#decision}}` resolves to the
//!   **prior decision prose** — re-read from the committed `docs/decisions/single-node-cache.md`
//!   and sliced losslessly — emitted as a multi-line `> ` blockquote (NOT the bare address
//!   handle). This is the round-trip on a committed, human-editable file (inc-2 byte-stable
//!   splice) + the context-slice over the persisted ADR (`worked-examples.md` → Task 2).
//! - **Finalize — the edge walk** passes: the overlaid edge index finds the supersedes
//!   target in the committed store (the two reachable surfaces), so finalize lands exactly
//!   one commit that promotes the new ADR to `docs/decisions/`.
//! - **The dangling variant** points `supersedes` at a target in neither surface; the
//!   `schema-conformance.ref-resolves` walk blocks finalize non-zero, naming the dangling
//!   target + the three routing options (fix / create-in-task / drop), and creates NO
//!   commit (`worked-examples.md` → The dangling variant).
//!
//! No external test crates: the binary path comes from `CARGO_BIN_EXE_jigc`, the temp repo
//! is a real `git init`, and a self-cleaning `TempDir` keeps the test off the dev's repo.

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
            "jigc-supersede-{tag}-{}-{:?}",
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

/// Fill every author-required field/slot of the provisioned commit doc so a finalize
/// over it validates clean.
fn fill_commit(repo: &Path, home: &Path, task: &str) {
    let set_field = |addr: &str, value: &str| {
        let out = jigc_doc(repo, home, &["set-field", addr, "--value", value], None);
        assert_ok(&out, &format!("set-field {addr}"));
    };
    let set_slot = |addr: &str, prose: &[u8]| {
        let out = jigc_doc(
            repo,
            home,
            &["set-slot", addr, "--from-file", "-"],
            Some(prose),
        );
        assert_ok(&out, &format!("set-slot {addr}"));
    };
    set_field(&format!("commit:{task}#type"), "feat");
    set_field(&format!("commit:{task}#scope"), "cache");
    set_slot(&format!("commit:{task}#summary"), b"change the cache\n");
    set_slot(&format!("commit:{task}#body"), b"A cache change.\n");
}

/// Fill an ADR's author-required prose slots.
fn fill_adr_slots(repo: &Path, home: &Path, slug: &str, context: &[u8], decision: &[u8]) {
    let set_slot = |addr: &str, prose: &[u8]| {
        let out = jigc_doc(
            repo,
            home,
            &["set-slot", addr, "--from-file", "-"],
            Some(prose),
        );
        assert_ok(&out, &format!("set-slot {addr}"));
    };
    set_slot(&format!("adr:{slug}#context"), context);
    set_slot(&format!("adr:{slug}#decision"), decision);
    set_slot(
        &format!("adr:{slug}#consequences"),
        b"A cold node loses its sessions; clients re-authenticate.\n",
    );
}

/// Inject a `supersedes: <target>` line into the staged ADR's empty front-matter block.
/// The optional `supersedes` ref is absent from the created skeleton; injecting the
/// canonical `key: value` line directly keeps this test's concern the finalize forward-ref
/// gate, not the write-path field-generation concern (mirrors `finalize_to_git.rs`).
fn inject_supersedes(repo: &Path, task: &str, slug: &str, target: &str) {
    let staged = repo
        .join(".jigc")
        .join("tasks")
        .join(task)
        .join("docs")
        .join(format!("adr:{slug}.md"));
    let body = fs::read_to_string(&staged).expect("read staged ADR");
    // Insert the `supersedes` line before the closing front-matter fence (the created
    // ADR now carries materialized `status`/`date` header lines, not an empty fence).
    let with = body.replacen("\n---\n", &format!("\nsupersedes: {target}\n---\n"), 1);
    assert_ne!(
        body, with,
        "the staged ADR carries a front-matter block to inject the ref into"
    );
    fs::write(&staged, &with).expect("inject supersedes ref");
}

/// The committed prose of the prior ADR's `#decision` slice — the bytes the
/// `superseded-context` step must re-read from `docs/decisions/single-node-cache.md`,
/// trimmed of its trailing newline (the slice is the section's prose).
const PRIOR_DECISION: &str =
    "A single in-memory node keeps session lookups sub-millisecond and avoids a network hop.";

/// Setup — task 1 creates + finalizes the ADR that will be superseded. Returns the
/// repo's HEAD after the commit (task 2 pins to it).
fn commit_prior_adr(repo: &Path, home: &Path) {
    let out = jigc(
        repo,
        home,
        &[
            "start",
            "--workflow",
            "single-task",
            "cache sessions in a single in-memory node",
        ],
    );
    assert_ok(&out, "`jigc start` (task 1)");
    let task = "cache-sessions-in-a-single-in-memory-node";

    let create = jigc_doc(
        repo,
        home,
        &["create", "adr", "--title", "Single-node cache"],
        None,
    );
    assert_ok(&create, "`jigc doc create adr` (task 1)");
    let adr = String::from_utf8(create.stdout)
        .expect("utf-8")
        .trim()
        .to_string();
    assert_eq!(adr, "adr:single-node-cache");

    fill_adr_slots(
        repo,
        home,
        "single-node-cache",
        b"Session lookups must stay sub-millisecond.\n",
        format!("{PRIOR_DECISION}\n").as_bytes(),
    );
    fill_commit(repo, home, task);

    let out = jigc(repo, home, &["task", "finalize", task]);
    assert_ok(&out, "`jigc task finalize` (task 1)");

    // The prior ADR is committed at its canonical path (the persisted differentiator).
    let committed = Command::new("git")
        .args(["show", "HEAD:docs/decisions/single-node-cache.md"])
        .current_dir(repo)
        .output()
        .expect("git show");
    assert!(
        committed.status.success(),
        "task 1 must commit docs/decisions/single-node-cache.md; stderr:\n{}",
        String::from_utf8_lossy(&committed.stderr),
    );
}

#[test]
fn superseding_decision_slices_the_prior_committed_adr_and_passes_the_edge_walk() {
    let repo = TempDir::new("pass");
    let home = TempDir::new("home");
    init_repo(repo.path());

    // ── Setup: task 1 commits adr:single-node-cache ──────────────────────────────
    commit_prior_adr(repo.path(), home.path());

    // ── Task 2: supersede it ─────────────────────────────────────────────────────
    let task = "move-the-session-cache-to-a-shared-redis-cluster";
    let out = jigc(
        repo.path(),
        home.path(),
        &[
            "start",
            "--workflow",
            "single-task",
            "move the session cache to a shared redis cluster",
        ],
    );
    assert_ok(&out, "`jigc start` (task 2)");

    let create = jigc_doc(
        repo.path(),
        home.path(),
        &["create", "adr", "--title", "Shared Redis session cache"],
        None,
    );
    assert_ok(&create, "`jigc doc create adr` (task 2)");
    let adr = String::from_utf8(create.stdout)
        .expect("utf-8")
        .trim()
        .to_string();
    assert_eq!(adr, "adr:shared-redis-session-cache");

    fill_adr_slots(
        repo.path(),
        home.path(),
        "shared-redis-session-cache",
        b"A single node is a single point of failure.\n",
        b"Replicate the session cache across nodes.\n",
    );
    // The supersedes edge → the committed adr:single-node-cache.
    inject_supersedes(
        repo.path(),
        task,
        "shared-redis-session-cache",
        "adr:single-node-cache",
    );

    // ── Re-compose: the superseded-context step slices the PRIOR committed decision ──
    let resume = jigc(repo.path(), home.path(), &["start", "--task", task]);
    assert_ok(&resume, "`jigc start --task <id>` re-compose (task 2)");
    let composed = String::from_utf8(resume.stdout).expect("utf-8");
    // The prior decision's PROSE is emitted as a `> ` blockquote — re-read + sliced from
    // the committed ADR (NOT the bare address handle).
    assert!(
        composed.contains(&format!("> {PRIOR_DECISION}")),
        "the superseded-context step must slice the prior committed decision as a `> ` \
         blockquote; got:\n{composed}",
    );
    assert!(
        !composed.contains("> adr:single-node-cache#decision"),
        "the slice must dereference to prose, NOT emit the bare address handle; got:\n{composed}",
    );

    // ── Finalize: the edge walk passes (target in the committed store) ────────────
    fill_commit(repo.path(), home.path(), task);
    let before: u32 = git(repo.path(), &["rev-list", "--count", "HEAD"])
        .parse()
        .unwrap();
    let out = jigc(repo.path(), home.path(), &["task", "finalize", task]);
    assert_ok(
        &out,
        "`jigc task finalize` (task 2) — the edge walk must pass",
    );
    let after: u32 = git(repo.path(), &["rev-list", "--count", "HEAD"])
        .parse()
        .unwrap();
    assert_eq!(
        after,
        before + 1,
        "task 2 finalize must land exactly ONE commit"
    );

    // The superseding ADR is committed at its canonical path.
    let committed = Command::new("git")
        .args(["show", "HEAD:docs/decisions/shared-redis-session-cache.md"])
        .current_dir(repo.path())
        .output()
        .expect("git show");
    assert!(
        committed.status.success(),
        "task 2 must promote + commit docs/decisions/shared-redis-session-cache.md; stderr:\n{}",
        String::from_utf8_lossy(&committed.stderr),
    );
    // The committed superseding ADR still carries its forward supersedes edge (the
    // inverse `superseded-by` on the prior ADR is derived on read, never stored).
    let body = String::from_utf8(committed.stdout).expect("utf-8");
    assert!(
        body.contains("supersedes: adr:single-node-cache"),
        "the committed ADR carries the forward supersedes edge; got:\n{body}",
    );
}

#[test]
fn dangling_supersedes_blocks_finalize_with_the_three_routing_options() {
    let repo = TempDir::new("dangle");
    let home = TempDir::new("home");
    init_repo(repo.path());

    // The committed prior ADR exists, but the dangling ref points elsewhere.
    commit_prior_adr(repo.path(), home.path());

    let task = "supersede-the-cache-decision";
    let out = jigc(
        repo.path(),
        home.path(),
        &[
            "start",
            "--workflow",
            "single-task",
            "supersede the cache decision",
        ],
    );
    assert_ok(&out, "`jigc start` (dangling task)");

    let create = jigc_doc(
        repo.path(),
        home.path(),
        &["create", "adr", "--title", "Shared redis session cache"],
        None,
    );
    assert_ok(&create, "`jigc doc create adr` (dangling task)");

    fill_adr_slots(
        repo.path(),
        home.path(),
        "shared-redis-session-cache",
        b"A single node is a single point of failure.\n",
        b"Replicate the session cache across nodes.\n",
    );
    // Point supersedes at a target in NEITHER surface — the dangling ref.
    inject_supersedes(
        repo.path(),
        task,
        "shared-redis-session-cache",
        "adr:typo-nonexistent",
    );
    fill_commit(repo.path(), home.path(), task);

    let before: u32 = git(repo.path(), &["rev-list", "--count", "HEAD"])
        .parse()
        .unwrap();
    let out = jigc(repo.path(), home.path(), &["task", "finalize", task]);
    assert!(
        !out.status.success(),
        "a dangling supersedes must make finalize exit non-zero; stdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr),
    );
    let rendered = format!(
        "{}{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr),
    );
    assert!(
        rendered.contains("adr:typo-nonexistent"),
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
            .join("decisions")
            .join("shared-redis-session-cache.md")
            .exists(),
        "a blocked finalize promotes nothing",
    );
}
