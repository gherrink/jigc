//! Independent END-TO-END AUDIT of the jigc MVP loop, driving the REAL `jigc`
//! binary (`CARGO_BIN_EXE_jigc`) against throwaway temp git repos. Written by an
//! independent test author — NOT the builders — to verify the full loop works,
//! mirroring `design/worked-examples.md` and the MVP scope in `CLAUDE.md`.
//!
//! Each `#[test]` is one audit scenario. The harness (TempDir + git + jigc helpers)
//! deliberately re-implements the established pattern from `finalize_to_git.rs` /
//! `superseding_decision.rs` / `task_lifecycle.rs` so this file is self-contained
//! and does not depend on the builders' helpers.
//!
//! `scenario_6_*` proves the committed-store OOB reconciliation classifier
//! (`design/reconciliation.md` → The state machine: absorb / conformance-block /
//! conflict-block / rename) is wired into the real loop: `task validate` (and
//! therefore `finalize`, which gates on exactly what validate reports) sweeps the
//! committed store (`decisions/*.md`) and routes drift — a conformant OOB edit
//! **absorbs** (advisory, non-blocking), a nonconformant one **conformance-blocks**.

use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

// ───────────────────────────── harness ─────────────────────────────

/// A throwaway directory that removes itself on drop.
struct TempDir(PathBuf);

impl TempDir {
    fn new(tag: &str) -> Self {
        let mut path = std::env::temp_dir();
        let unique = format!(
            "jigc-e2e-audit-{tag}-{}-{:?}",
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

/// A real git repo with one commit + the `.jigc/config/` project layer.
fn init_repo(repo: &Path) {
    git(repo, &["init", "-q"]);
    git(repo, &["config", "user.email", "test@example.com"]);
    git(repo, &["config", "user.name", "Test"]);
    fs::write(repo.join("README.md"), "hello\n").expect("write file");
    git(repo, &["add", "."]);
    git(repo, &["commit", "-q", "-m", "initial"]);
    fs::create_dir_all(repo.join(".jigc").join("config")).expect("create project layer");
}

/// Run `jigc <args>` with `cwd = repo`, `$HOME = home`.
fn jigc(repo: &Path, home: &Path, args: &[&str]) -> std::process::Output {
    Command::new(env!("CARGO_BIN_EXE_jigc"))
        .args(args)
        .current_dir(repo)
        .env("HOME", home)
        .output()
        .expect("run the jigc binary")
}

/// Run `jigc doc <args>`, optionally piping `stdin`.
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

fn assert_ok(out: &std::process::Output, what: &str) {
    assert!(
        out.status.success(),
        "{what} must succeed; stdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr),
    );
}

fn streams(out: &std::process::Output) -> String {
    format!(
        "{}{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr)
    )
}

/// Fill every author-required field/slot of the provisioned commit doc.
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
    set_field(&format!("commit:{task}#scope"), "gateway");
    set_slot(
        &format!("commit:{task}#summary"),
        b"add a per-client rate limiter\n",
    );
    set_slot(
        &format!("commit:{task}#body"),
        b"Throttle abusive clients.\n",
    );
}

/// Fill an ADR's author-required prose slots.
fn fill_adr(repo: &Path, home: &Path, slug: &str, decision: &[u8]) {
    let set_slot = |addr: &str, prose: &[u8]| {
        let out = jigc_doc(
            repo,
            home,
            &["set-slot", addr, "--from-file", "-"],
            Some(prose),
        );
        assert_ok(&out, &format!("set-slot {addr}"));
    };
    set_slot(&format!("adr:{slug}#context"), b"Some context here.\n");
    set_slot(&format!("adr:{slug}#decision"), decision);
    set_slot(&format!("adr:{slug}#consequences"), b"Some consequences.\n");
}

/// Inject a `supersedes: <target>` line into the staged ADR's empty front-matter.
fn inject_supersedes(repo: &Path, task: &str, slug: &str, target: &str) {
    let staged = repo
        .join(".jigc")
        .join("tasks")
        .join(task)
        .join("docs")
        .join(format!("adr:{slug}.md"));
    let body = fs::read_to_string(&staged).expect("read staged ADR");
    let with = body.replacen(
        "---\n---\n",
        &format!("---\nsupersedes: {target}\n---\n"),
        1,
    );
    assert_ne!(
        body, with,
        "the staged ADR carries an empty front-matter block"
    );
    fs::write(&staged, &with).expect("inject supersedes ref");
}

// ─────────────────── scenario 1: setup idempotency ───────────────────

#[test]
fn scenario_1_setup_is_idempotent() {
    let repo = TempDir::new("setup");
    let home = TempDir::new("home");
    // A bare `.git` marker is enough for repo-root discovery.
    fs::create_dir_all(repo.path().join(".git")).expect("git marker");

    let first = jigc(repo.path(), home.path(), &["setup"]);
    assert_ok(&first, "first `jigc setup`");

    let claude1 = fs::read_to_string(repo.path().join("CLAUDE.md")).expect("CLAUDE.md written");
    let agent1 =
        fs::read_to_string(repo.path().join(".jigc/AGENT.md")).expect(".jigc/AGENT.md written");
    let settings1 =
        fs::read_to_string(repo.path().join(".claude/settings.json")).expect("settings written");

    // The bootstrap reference (no marker comments) + managed file + allowlist landed.
    assert!(
        claude1.contains("@.jigc/AGENT.md") && !claude1.contains("<!-- jigc:bootstrap"),
        "setup must inject the bare `@.jigc/AGENT.md` reference, not a marker block; got:\n{claude1}"
    );
    assert!(
        agent1.contains("`jigc` is your interface to this project"),
        ".jigc/AGENT.md must carry the bootstrap sentence; got:\n{agent1}"
    );
    assert!(
        settings1.contains("\"jigc *\""),
        "setup must allowlist `jigc *`; got:\n{settings1}"
    );

    // Second run: byte-identical (no duplicate reference, no duplicate permit).
    let second = jigc(repo.path(), home.path(), &["setup"]);
    assert_ok(&second, "second `jigc setup`");
    let claude2 = fs::read_to_string(repo.path().join("CLAUDE.md")).expect("CLAUDE.md present");
    let agent2 =
        fs::read_to_string(repo.path().join(".jigc/AGENT.md")).expect(".jigc/AGENT.md present");
    let settings2 =
        fs::read_to_string(repo.path().join(".claude/settings.json")).expect("settings present");
    assert_eq!(
        claude1, claude2,
        "CLAUDE.md must be byte-identical after re-run"
    );
    assert_eq!(
        agent1, agent2,
        ".jigc/AGENT.md must be byte-identical after re-run"
    );
    assert_eq!(
        settings1, settings2,
        "settings.json must be byte-identical after re-run"
    );

    // Exactly one bootstrap reference (not two).
    assert_eq!(
        claude2.matches("@.jigc/AGENT.md").count(),
        1,
        "exactly one bootstrap reference must exist; got:\n{claude2}"
    );
}

// ───────────────────── scenario 2: orientation ─────────────────────

#[test]
fn scenario_2a_orientation_unset_routes_to_setup() {
    let repo = TempDir::new("orient-unset");
    let home = TempDir::new("home");
    fs::create_dir_all(repo.path().join(".git")).expect("git marker");
    // No `.jigc/config/` → unset project.

    let out = jigc(repo.path(), home.path(), &["start"]);
    assert_ok(&out, "bare `jigc start` (unset)");
    let stdout = String::from_utf8(out.stdout).expect("utf-8");
    assert!(
        stdout.contains("isn't set up") && stdout.contains("Run: `jigc setup`"),
        "unset orientation must route to `jigc setup`; got:\n{stdout}"
    );
}

#[test]
fn scenario_2b_orientation_setup_shows_catalog_and_footer() {
    let repo = TempDir::new("orient-set");
    let home = TempDir::new("home");
    fs::create_dir_all(repo.path().join(".git")).expect("git marker");
    fs::create_dir_all(repo.path().join(".jigc").join("config")).expect("project layer");

    let out = jigc(repo.path(), home.path(), &["start"]);
    assert_ok(&out, "bare `jigc start` (set up)");
    let stdout = String::from_utf8(out.stdout).expect("utf-8");
    assert!(
        stdout.contains("Pack: "),
        "must print provenance header; got:\n{stdout}"
    );
    assert!(
        stdout.contains("single-task") && stdout.contains("implement one scoped change end-to-end"),
        "must list the workflow catalog with `when` hints; got:\n{stdout}"
    );
    assert!(
        stdout
            .trim_end()
            .ends_with("— jigc · run `jigc start` for orientation; all writes through `jigc`."),
        "human output must end with the routing footer; got:\n{stdout}"
    );
}

#[test]
fn scenario_2c_orientation_json_is_valid_and_footerless() {
    let repo = TempDir::new("orient-json");
    let home = TempDir::new("home");
    fs::create_dir_all(repo.path().join(".git")).expect("git marker");
    fs::create_dir_all(repo.path().join(".jigc").join("config")).expect("project layer");

    let out = jigc(repo.path(), home.path(), &["start", "--format", "json"]);
    assert_ok(&out, "bare `jigc start --format json`");
    let stdout = String::from_utf8(out.stdout).expect("utf-8");
    assert!(
        !stdout.contains("— jigc · run `jigc start`"),
        "JSON must carry no routing footer; got:\n{stdout}"
    );
    let value: serde_json::Value = serde_json::from_str(&stdout)
        .unwrap_or_else(|e| panic!("must be valid JSON ({e}); got:\n{stdout}"));
    assert_eq!(value["state"], serde_json::json!("clean"), "got:\n{stdout}");
}

// ─────────────── scenario 3: single-task happy path ────────────────

#[test]
fn scenario_3_single_task_happy_path_one_commit() {
    let repo = TempDir::new("happy");
    let home = TempDir::new("home");
    init_repo(repo.path());

    // Mint + compose.
    let start = jigc(repo.path(), home.path(), &["start", "add rate limiter"]);
    assert_ok(&start, "`jigc start \"<intent>\"`");
    let composed = String::from_utf8(start.stdout).expect("utf-8");
    let task = "add-rate-limiter";

    // The composed workflow embeds the intent + the four-class write directives.
    assert!(
        composed.contains("add rate limiter") && composed.contains("<<author:"),
        "composed view must embed intent + `<<author:` directive; got:\n{composed}"
    );

    // The agent authors a code change directly in the working tree.
    fs::write(repo.path().join("limiter.rs"), "// rate limiter\n").expect("write code");

    // Fill the commit doc via the real write verbs.
    fill_commit(repo.path(), home.path(), task);

    // validate is clean.
    let val = jigc(repo.path(), home.path(), &["task", "validate", task]);
    assert_ok(&val, "`jigc task validate` on a conformant task");

    let before: u32 = git(repo.path(), &["rev-list", "--count", "HEAD"])
        .parse()
        .unwrap();

    // finalize → exactly one commit, message = rendered commit doc, code staged.
    let fin = jigc(repo.path(), home.path(), &["task", "finalize", task]);
    assert_ok(&fin, "`jigc task finalize`");
    let after: u32 = git(repo.path(), &["rev-list", "--count", "HEAD"])
        .parse()
        .unwrap();
    assert_eq!(after, before + 1, "finalize must land EXACTLY ONE commit");

    let message = git(repo.path(), &["log", "-1", "--format=%B"]);
    assert_eq!(
        message.trim_end(),
        "feat(gateway): add a per-client rate limiter\n\nThrottle abusive clients.",
        "commit message must equal the rendered commit doc; got:\n{message}"
    );
    let files = git(repo.path(), &["show", "--name-only", "--format=", "HEAD"]);
    assert!(
        files.lines().any(|l| l == "limiter.rs"),
        "the code change must be staged into the commit; files:\n{files}"
    );
    assert!(
        !repo.path().join(".jigc").join("tasks").join(task).exists(),
        "finalize must remove the working area"
    );
}

// ─────────────── scenario 4: superseding decision ─────────────────

/// Setup helper: task 1 creates + finalizes the ADR to be superseded.
fn commit_prior_adr(repo: &Path, home: &Path, decision: &str) {
    let out = jigc(
        repo,
        home,
        &["start", "cache sessions in a single in-memory node"],
    );
    assert_ok(&out, "`jigc start` (prior-ADR task)");
    let task = "cache-sessions-in-a-single-in-memory-node";
    let create = jigc_doc(
        repo,
        home,
        &["create", "adr", "--title", "Single-node cache"],
        None,
    );
    assert_ok(&create, "`jigc doc create adr` (prior)");
    let addr = String::from_utf8(create.stdout)
        .expect("utf-8")
        .trim()
        .to_string();
    assert_eq!(addr, "adr:single-node-cache");
    fill_adr(
        repo,
        home,
        "single-node-cache",
        format!("{decision}\n").as_bytes(),
    );
    fill_commit(repo, home, task);
    let fin = jigc(repo, home, &["task", "finalize", task]);
    assert_ok(&fin, "`jigc task finalize` (prior)");
    assert!(
        git(
            repo,
            &["cat-file", "-e", "HEAD:decisions/single-node-cache.md"]
        )
        .is_empty(),
        "prior ADR must be committed at decisions/single-node-cache.md"
    );
}

#[test]
fn scenario_4_superseding_decision_resolves_slice_and_passes_edge_walk() {
    let repo = TempDir::new("supersede-pass");
    let home = TempDir::new("home");
    init_repo(repo.path());

    const PRIOR_DECISION: &str =
        "A single in-memory node keeps session lookups sub-millisecond and avoids a network hop.";
    commit_prior_adr(repo.path(), home.path(), PRIOR_DECISION);

    // Task 2: supersede it.
    let task = "move-the-session-cache-to-a-shared-redis-cluster";
    let start = jigc(
        repo.path(),
        home.path(),
        &["start", "move the session cache to a shared redis cluster"],
    );
    assert_ok(&start, "`jigc start` (task 2)");
    let create = jigc_doc(
        repo.path(),
        home.path(),
        &["create", "adr", "--title", "Shared Redis session cache"],
        None,
    );
    assert_ok(&create, "`jigc doc create adr` (task 2)");
    fill_adr(
        repo.path(),
        home.path(),
        "shared-redis-session-cache",
        b"Replicate the session cache across nodes.\n",
    );
    inject_supersedes(
        repo.path(),
        task,
        "shared-redis-session-cache",
        "adr:single-node-cache",
    );

    // Re-compose: the superseded-context step slices the PRIOR committed decision prose.
    let resume = jigc(repo.path(), home.path(), &["start", "--task", task]);
    assert_ok(&resume, "`jigc start --task <id>` re-compose");
    let composed = String::from_utf8(resume.stdout).expect("utf-8");
    assert!(
        composed.contains(&format!("> {PRIOR_DECISION}")),
        "the context-slice must resolve the prior committed decision prose as a `> ` \
         blockquote, NOT the bare address handle; got:\n{composed}"
    );
    assert!(
        !composed.contains("> adr:single-node-cache#decision"),
        "the slice must dereference to prose, not emit the address handle; got:\n{composed}"
    );

    // Finalize: the forward-ref walk passes (target in the committed store).
    fill_commit(repo.path(), home.path(), task);
    let before: u32 = git(repo.path(), &["rev-list", "--count", "HEAD"])
        .parse()
        .unwrap();
    let fin = jigc(repo.path(), home.path(), &["task", "finalize", task]);
    assert_ok(&fin, "`jigc task finalize` (task 2) — edge walk must pass");
    let after: u32 = git(repo.path(), &["rev-list", "--count", "HEAD"])
        .parse()
        .unwrap();
    assert_eq!(
        after,
        before + 1,
        "task 2 finalize must land exactly one commit"
    );
    assert!(
        git(
            repo.path(),
            &[
                "cat-file",
                "-e",
                "HEAD:decisions/shared-redis-session-cache.md"
            ]
        )
        .is_empty(),
        "the superseding ADR must be promoted + committed"
    );
}

#[test]
fn scenario_4_dangling_supersedes_blocks_finalize() {
    let repo = TempDir::new("supersede-dangle");
    let home = TempDir::new("home");
    init_repo(repo.path());

    commit_prior_adr(repo.path(), home.path(), "keep it on one node.");

    let task = "supersede-the-cache-decision";
    let start = jigc(
        repo.path(),
        home.path(),
        &["start", "supersede the cache decision"],
    );
    assert_ok(&start, "`jigc start` (dangling task)");
    let create = jigc_doc(
        repo.path(),
        home.path(),
        &["create", "adr", "--title", "Shared redis session cache"],
        None,
    );
    assert_ok(&create, "`jigc doc create adr` (dangling task)");
    fill_adr(
        repo.path(),
        home.path(),
        "shared-redis-session-cache",
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
    let fin = jigc(repo.path(), home.path(), &["task", "finalize", task]);
    assert!(
        !fin.status.success(),
        "a dangling supersedes must make finalize exit NON-ZERO; streams:\n{}",
        streams(&fin)
    );
    let rendered = streams(&fin);
    assert!(
        rendered.contains("adr:typo-nonexistent"),
        "the block must name the dangling target; got:\n{rendered}"
    );
    assert!(
        rendered.contains("fix")
            && rendered.contains("create the target in this task")
            && rendered.contains("drop"),
        "the block must surface the three routing options; got:\n{rendered}"
    );
    let after: u32 = git(repo.path(), &["rev-list", "--count", "HEAD"])
        .parse()
        .unwrap();
    assert_eq!(before, after, "a blocked finalize must create no commit");
    assert!(
        !repo
            .path()
            .join("decisions")
            .join("shared-redis-session-cache.md")
            .exists(),
        "a blocked finalize promotes nothing"
    );
}

// ─────────────── scenario 5: failure / abort paths ─────────────────

#[test]
fn scenario_5a_validate_and_finalize_block_on_missing_required_field() {
    let repo = TempDir::new("missing-field");
    let home = TempDir::new("home");
    init_repo(repo.path());

    let start = jigc(repo.path(), home.path(), &["start", "add rate limiter"]);
    assert_ok(&start, "`jigc start`");
    let task = "add-rate-limiter";

    // Leave the commit doc's required slots/fields EMPTY.
    let val = jigc(repo.path(), home.path(), &["task", "validate", task]);
    assert!(
        !val.status.success(),
        "validate on an unfilled commit doc must exit non-zero; streams:\n{}",
        streams(&val)
    );
    assert!(
        streams(&val).contains("schema-conformance."),
        "validate must surface a `schema-conformance.*` finding; got:\n{}",
        streams(&val)
    );

    // finalize must also block, and land NO commit.
    let before: u32 = git(repo.path(), &["rev-list", "--count", "HEAD"])
        .parse()
        .unwrap();
    let fin = jigc(repo.path(), home.path(), &["task", "finalize", task]);
    assert!(
        !fin.status.success(),
        "finalize on an unfilled commit doc must exit non-zero; streams:\n{}",
        streams(&fin)
    );
    let after: u32 = git(repo.path(), &["rev-list", "--count", "HEAD"])
        .parse()
        .unwrap();
    assert_eq!(before, after, "a blocked finalize must land no commit");
}

#[test]
fn scenario_5b_finalize_aborts_on_base_mismatch() {
    let repo = TempDir::new("base-mismatch");
    let home = TempDir::new("home");
    init_repo(repo.path());

    let start = jigc(repo.path(), home.path(), &["start", "add rate limiter"]);
    assert_ok(&start, "`jigc start`");
    let task = "add-rate-limiter";
    fs::write(repo.path().join("limiter.rs"), "// x\n").expect("write code");
    fill_commit(repo.path(), home.path(), task);

    // HEAD moves after start: a human lands another commit, diverging the base pin.
    fs::write(repo.path().join("other.txt"), "unrelated\n").expect("write");
    git(repo.path(), &["add", "other.txt"]);
    git(repo.path(), &["commit", "-q", "-m", "unrelated work"]);

    let before: u32 = git(repo.path(), &["rev-list", "--count", "HEAD"])
        .parse()
        .unwrap();
    let fin = jigc(repo.path(), home.path(), &["task", "finalize", task]);
    assert!(
        !fin.status.success(),
        "a base mismatch must make finalize exit non-zero; streams:\n{}",
        streams(&fin)
    );
    assert!(
        streams(&fin).to_lowercase().contains("base"),
        "the abort must surface the base divergence; got:\n{}",
        streams(&fin)
    );
    let after: u32 = git(repo.path(), &["rev-list", "--count", "HEAD"])
        .parse()
        .unwrap();
    assert_eq!(before, after, "a base-mismatch abort must create no commit");
}

#[test]
fn scenario_5c_discard_cleans_the_working_area() {
    let repo = TempDir::new("discard");
    let home = TempDir::new("home");
    init_repo(repo.path());

    let start = jigc(repo.path(), home.path(), &["start", "add rate limiter"]);
    assert_ok(&start, "`jigc start`");
    let task = "add-rate-limiter";
    let area = repo.path().join(".jigc").join("tasks").join(task);
    assert!(area.is_dir(), "the started working area must exist");

    let discard = jigc(repo.path(), home.path(), &["task", "discard", task]);
    assert_ok(&discard, "`jigc task discard`");
    assert!(!area.exists(), "discard must remove the working area");
}

// ─────────────── scenario 6: out-of-band edit ─────────────────
//
// The committed-store OOB reconciliation classifier is wired through the real loop:
// `task validate` (and `finalize`, which gates on what validate reports) sweeps the
// committed store (`decisions/*.md`) and routes drift. `design/reconciliation.md` →
// Detection timing names `jigc task validate` as the full sweep; the classifier is
// `engine::file_state::reconcile_committed`. A conformant OOB edit **absorbs**
// (advisory `reconciliation.absorb`, non-blocking); a nonconformant one
// **conformance-blocks** (`reconciliation.conformance-block`, blocking). Files are
// truth: the human's edit is never silently discarded.

#[test]
fn scenario_6_conformant_oob_edit_to_committed_adr_absorbs() {
    let repo = TempDir::new("oob");
    let home = TempDir::new("home");
    init_repo(repo.path());

    // Commit a prior ADR (task 1).
    commit_prior_adr(repo.path(), home.path(), "keep it on one node.");
    let adr_path = repo.path().join("decisions").join("single-node-cache.md");
    assert!(adr_path.is_file(), "the committed ADR exists on disk");

    // Hand-edit the committed managed doc OUTSIDE jigc (an out-of-band edit that
    // stays schema-conformant — only the `## Decision` slot prose changes).
    let before = fs::read_to_string(&adr_path).expect("read committed ADR");
    let edited = before.replace("keep it on one node.", "keep it on TWO nodes (OOB EDIT).");
    assert_ne!(before, edited, "the OOB edit must change the file");
    fs::write(&adr_path, &edited).expect("hand-edit the committed ADR");

    // A fresh second task; `task validate` runs the full sweep, which now reaches the
    // committed store (reconciliation.md → Detection timing: the `task validate` full
    // sweep). The conformant drift absorbs.
    let start = jigc(repo.path(), home.path(), &["start", "a fresh second task"]);
    assert_ok(&start, "`jigc start` after an OOB edit");
    let task = "a-fresh-second-task";
    fill_commit(repo.path(), home.path(), task);

    let val = jigc(repo.path(), home.path(), &["task", "validate", task]);
    assert_ok(
        &val,
        "a conformant committed-store OOB edit absorbs (advisory), so validate stays clean",
    );
    let surfaced = streams(&val);
    assert!(
        surfaced.contains("external edit absorbed") && surfaced.contains("single-node-cache"),
        "the conformant OOB edit must surface an `external edit absorbed` line naming the \
         committed doc; got:\n{surfaced}"
    );

    // The on-disk OOB edit is NOT lost (files are truth) — it remains as the human left it.
    let after = fs::read_to_string(&adr_path).expect("read committed ADR after validate");
    assert_eq!(after, edited, "the OOB edit must not be silently discarded");
}

#[test]
fn scenario_6_nonconformant_oob_edit_to_committed_adr_blocks_finalize() {
    // A nonconformant committed-store OOB edit conformance-blocks: validate and the
    // later task's finalize both gate on it (finalize ≡ validate + commit).
    let repo = TempDir::new("oob-finalize");
    let home = TempDir::new("home");
    init_repo(repo.path());

    commit_prior_adr(repo.path(), home.path(), "keep it on one node.");
    let adr_path = repo.path().join("decisions").join("single-node-cache.md");
    let before = fs::read_to_string(&adr_path).expect("read committed ADR");
    // Rename a required section heading — a structural nonconformance the parser
    // rejects (the conformance-block case).
    let edited = before.replace("## Decision", "## Decisionz");
    assert_ne!(before, edited, "the OOB edit must change the file");
    fs::write(&adr_path, &edited).expect("OOB edit");

    // A fresh task that touches nothing about the ADR; finalize must still block on the
    // un-reconciled committed-doc drift (the full sweep reaches the committed store).
    let start = jigc(repo.path(), home.path(), &["start", "unrelated change"]);
    assert_ok(&start, "`jigc start`");
    let task = "unrelated-change";
    fs::write(repo.path().join("code.rs"), "// x\n").expect("write code");
    fill_commit(repo.path(), home.path(), task);

    let before_count: u32 = git(repo.path(), &["rev-list", "--count", "HEAD"])
        .parse()
        .unwrap();
    let fin = jigc(repo.path(), home.path(), &["task", "finalize", task]);
    assert!(
        !fin.status.success(),
        "a nonconformant committed-store OOB edit must block finalize; streams:\n{}",
        streams(&fin)
    );
    let surfaced = streams(&fin);
    assert!(
        surfaced.contains("conformance-block") || surfaced.contains("nonconformant"),
        "the block must surface a reconciliation conformance-block; got:\n{surfaced}"
    );
    let after_count: u32 = git(repo.path(), &["rev-list", "--count", "HEAD"])
        .parse()
        .unwrap();
    assert_eq!(
        before_count, after_count,
        "a blocked finalize must land no commit"
    );
}
