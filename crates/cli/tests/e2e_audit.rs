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
//! committed store (`docs/decisions/*.md`) and routes drift — a conformant OOB edit
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
    // Insert the `supersedes` line before the closing front-matter fence (the created
    // ADR now carries materialized `status`/`date` header lines, not an empty fence).
    let with = body.replacen("\n---\n", &format!("\nsupersedes: {target}\n---\n"), 1);
    assert_ne!(
        body, with,
        "the staged ADR carries a front-matter block to inject the ref into"
    );
    fs::write(&staged, &with).expect("inject supersedes ref");
}

// ─────────────────── scenario 1: setup idempotency ───────────────────

#[test]
fn scenario_1_setup_is_idempotent() {
    let repo = TempDir::new("setup");
    let home = TempDir::new("home");
    // `jigc setup` now installs a `pre-commit` hook, which resolves the repo's real
    // hooks dir via git — so the repo must be a real `git init`, not a bare `.git`
    // marker (as the real `jigc setup` always runs inside a git repo).
    git(repo.path(), &["init", "-q"]);
    // `jigc setup` commits its own install footprint (M30 audit finding 1) — mint needs
    // a usable identity + signing off.
    git(repo.path(), &["config", "user.email", "test@example.com"]);
    git(repo.path(), &["config", "user.name", "Test"]);
    git(repo.path(), &["config", "commit.gpgsign", "false"]);

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
        settings1.contains("\"Bash(jigc:*)\""),
        "setup must allowlist `Bash(jigc:*)`; got:\n{settings1}"
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

/// The profile deny-floor patterns, parsed from the shipped adapter profile so the
/// e2e assertion tracks the source of truth (the shipped `claude-code.yaml`), not a
/// hand-copied list that could drift from what `jigc setup` actually merges.
fn floor_patterns() -> Vec<String> {
    let yaml = include_str!("../adapters/claude-code.yaml");
    let mut out = Vec::new();
    let mut in_deny = false;
    for line in yaml.lines() {
        if line.trim_start().starts_with("deny:") {
            in_deny = true;
            continue;
        }
        if in_deny {
            match line.trim_start().strip_prefix("- ") {
                Some(entry) => out.push(entry.trim().trim_matches('"').to_string()),
                None => break, // end of the indented deny list block
            }
        }
    }
    assert!(
        !out.is_empty(),
        "the shipped profile must carry a deny floor"
    );
    out
}

/// The `deny` safety floor is **merged, never clobbered** into a pre-existing
/// `.claude/settings.json`: a foreign `permissions.deny` entry seeded before setup
/// survives, every profile floor pattern lands, and a second setup is byte-identical
/// (`design/assistant-adapter.md` → the `deny` safety floor — mirror `inject_allowlist`).
#[test]
fn scenario_1b_deny_floor_merges_never_clobbers() {
    let repo = TempDir::new("deny-floor");
    let home = TempDir::new("home");
    git(repo.path(), &["init", "-q"]);
    git(repo.path(), &["config", "user.email", "test@example.com"]);
    git(repo.path(), &["config", "user.name", "Test"]);
    git(repo.path(), &["config", "commit.gpgsign", "false"]);

    // Pre-seed a FOREIGN `permissions.deny` entry before setup (the merge-never-clobber
    // case): the human already denied something of their own.
    fs::create_dir_all(repo.path().join(".claude")).expect("create .claude");
    fs::write(
        repo.path().join(".claude/settings.json"),
        "{\n  \"permissions\": {\n    \"deny\": [\n      \"Bash(shutdown:*)\"\n    ]\n  }\n}\n",
    )
    .expect("seed foreign deny");

    let out = jigc(repo.path(), home.path(), &["setup"]);
    assert_ok(&out, "`jigc setup` over a pre-seeded foreign deny");

    let settings =
        fs::read_to_string(repo.path().join(".claude/settings.json")).expect("settings written");
    // The foreign entry survives (never clobbered).
    assert!(
        settings.contains("\"Bash(shutdown:*)\""),
        "the foreign deny entry must survive the merge; got:\n{settings}"
    );
    // Every profile floor pattern landed.
    for p in floor_patterns() {
        assert!(
            settings.contains(&format!("\"{p}\"")),
            "the floor pattern `{p}` must be merged into permissions.deny; got:\n{settings}"
        );
    }

    // Second setup: byte-identical (the deny merge is an idempotent, byte-stable no-op).
    let out2 = jigc(repo.path(), home.path(), &["setup"]);
    assert_ok(&out2, "second `jigc setup`");
    let settings2 =
        fs::read_to_string(repo.path().join(".claude/settings.json")).expect("settings present");
    assert_eq!(
        settings, settings2,
        "settings.json must be byte-identical after a re-run (deny merge is idempotent)"
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

    // Mint + compose (post-flip the cascade default is the `router`, so minting
    // goes through Form D `--workflow single-task`).
    let start = jigc(
        repo.path(),
        home.path(),
        &["start", "--workflow", "single-task", "add rate limiter"],
    );
    assert_ok(&start, "`jigc start --workflow single-task \"<intent>\"`");
    let composed = String::from_utf8(start.stdout).expect("utf-8");
    let task = "add-rate-limiter";

    // The composed workflow embeds the intent + the four-class write directives.
    assert!(
        composed.contains("add rate limiter") && composed.contains("<<author:"),
        "composed view must embed intent + `<<author:` directive; got:\n{composed}"
    );

    // The agent authors a code change directly in the working tree.
    fs::write(repo.path().join("limiter.rs"), "// rate limiter\n").expect("write code");
    git(repo.path(), &["add", "limiter.rs"]);

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
        &[
            "start",
            "--workflow",
            "single-task",
            "cache sessions in a single in-memory node",
        ],
    );
    assert_ok(&out, "`jigc start` (prior-ADR task)");
    let task = "cache-sessions-in-a-single";
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
            &["cat-file", "-e", "HEAD:docs/decisions/single-node-cache.md"]
        )
        .is_empty(),
        "prior ADR must be committed at docs/decisions/single-node-cache.md"
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

    // Task 2: supersede it. The trailing stopword `to` the 5-word cap exposes is
    // dropped by the F5 edge-stopword rule, so the minted task id ends at `cache`.
    let task = "move-the-session-cache";
    let start = jigc(
        repo.path(),
        home.path(),
        &[
            "start",
            "--workflow",
            "single-task",
            "move the session cache to a shared redis cluster",
        ],
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
                "HEAD:docs/decisions/shared-redis-session-cache.md"
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
        &[
            "start",
            "--workflow",
            "single-task",
            "supersede the cache decision",
        ],
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
            .join("docs")
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

    let start = jigc(
        repo.path(),
        home.path(),
        &["start", "--workflow", "single-task", "add rate limiter"],
    );
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
fn scenario_5b_finalize_aborts_on_overlapping_base_divergence() {
    let repo = TempDir::new("base-mismatch");
    let home = TempDir::new("home");
    init_repo(repo.path());

    let start = jigc(
        repo.path(),
        home.path(),
        &["start", "--workflow", "single-task", "add rate limiter"],
    );
    assert_ok(&start, "`jigc start`");
    let task = "add-rate-limiter";

    // HEAD moves after start ON the task's own path: a human commits `limiter.rs`
    // while the task's working tree edits it too — the parallel-hand-editing overlap
    // (`design/finalize.md` → Parallel hand-editing; disjoint moved history would
    // auto-re-pin instead, per the 2026-06-12 phase-1 amendment).
    fs::write(repo.path().join("limiter.rs"), "// the human's\n").expect("write");
    git(repo.path(), &["add", "limiter.rs"]);
    git(repo.path(), &["commit", "-q", "-m", "human edit"]);
    fs::write(repo.path().join("limiter.rs"), "// the task's\n").expect("write code");
    fill_commit(repo.path(), home.path(), task);

    let before: u32 = git(repo.path(), &["rev-list", "--count", "HEAD"])
        .parse()
        .unwrap();
    let fin = jigc(repo.path(), home.path(), &["task", "finalize", task]);
    assert!(
        !fin.status.success(),
        "an overlapping base divergence must make finalize exit non-zero; streams:\n{}",
        streams(&fin)
    );
    assert!(
        streams(&fin).contains("overlaps the task's work on `limiter.rs`"),
        "the abort must name the overlapping path; got:\n{}",
        streams(&fin)
    );
    let after: u32 = git(repo.path(), &["rev-list", "--count", "HEAD"])
        .parse()
        .unwrap();
    assert_eq!(
        before, after,
        "a base-divergence abort must create no commit"
    );
}

#[test]
fn scenario_5c_discard_cleans_the_working_area() {
    let repo = TempDir::new("discard");
    let home = TempDir::new("home");
    init_repo(repo.path());

    let start = jigc(
        repo.path(),
        home.path(),
        &["start", "--workflow", "single-task", "add rate limiter"],
    );
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
// committed store (`docs/decisions/*.md`) and routes drift. `design/reconciliation.md` →
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
    let adr_path = repo
        .path()
        .join("docs")
        .join("decisions")
        .join("single-node-cache.md");
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
    let start = jigc(
        repo.path(),
        home.path(),
        &["start", "--workflow", "single-task", "a fresh second task"],
    );
    assert_ok(&start, "`jigc start` after an OOB edit");
    // The leading stopword `a` is dropped by the F5 edge-stopword rule.
    let task = "fresh-second-task";
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
    let adr_path = repo
        .path()
        .join("docs")
        .join("decisions")
        .join("single-node-cache.md");
    let before = fs::read_to_string(&adr_path).expect("read committed ADR");
    // Rename a required section heading — a structural nonconformance the parser
    // rejects (the conformance-block case).
    let edited = before.replace("## Decision", "## Decisionz");
    assert_ne!(before, edited, "the OOB edit must change the file");
    fs::write(&adr_path, &edited).expect("OOB edit");

    // A fresh task that touches nothing about the ADR; finalize must still block on the
    // un-reconciled committed-doc drift (the full sweep reaches the committed store).
    let start = jigc(
        repo.path(),
        home.path(),
        &["start", "--workflow", "single-task", "unrelated change"],
    );
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

// ───── scenario 7: cross-kind override acceptance (M4 headline) ─────
//
// The M4 acceptance path (`design/worked-examples.md` → flow 3; roadmap Increment 6
// Proves): a project composes *differently from the pack default* via recorded
// deltas across **all four delta kinds** (`scalar-set` · `structural-op` ·
// `slot-fill` · `tracked-fork`), the cascade applied **live** through the real
// binary — while the **no-override** read path stays byte-identical to the pre-M4
// baseline (the determinism boundary intact). This scenario DRIVES the inc-1..5
// `jigc config <verb>` write surface (it does not build it) end-to-end:
//
//   - each verb shifts the composed bytes vs the no-override compose,
//   - the three closed-surface rejections (undeclared-key, wrong-type enum,
//     hand-authored orphan slot-fill) exit non-zero with their routes,
//   - `--explain` reflects an applied override (`overrides applied: N` tracks reality),
//   - a fresh no-override repo composes single-task/router byte-identically to the
//     pre-M4 baseline golden (the headline determinism assertion through the binary).

/// The **no-override** bare-`jigc start "<intent>"` router composition, byte for byte
/// (`crates/cli/tests/start_compose.rs::NO_OVERRIDE_ROUTER_GOLDEN` — the recorded
/// pre-M4 baseline). Re-asserted here through the audit binary so the headline
/// determinism guard rides on this independent end-to-end harness too.
const NO_OVERRIDE_ROUTER_GOLDEN: &str = "\
These are the selectable work-workflows, each with the situation it fits:

- architecture-documentation — document the architecture of a part of the system, tying its components to the code that implements them
- implement-from-spec — a committed spec already covers the intent, with acceptance criteria to build against
- plan — draft the specification for upcoming work before writing any code
- project-setup — bootstrap a brand-new project by developing the idea into its first product requirements
- quick-fix — apply a small commit-only fix that touches no documented code and records no decision
- single-task — implement one scoped change end-to-end, recording its decisions as ADRs

Pick the workflow whose situation best fits the intent, then re-run with that
choice and the original intent:

jigc start --workflow <chosen> \"<intent>\"
— jigc · run `jigc start` for orientation; all writes through `jigc`.
";

/// The pack `implement` body's leading marker — present iff `step:implement`
/// composes (from the pack default or a project shadow). Bounds the include-list /
/// fork / fill assertions to the relevant region.
const IMPLEMENT_MARKER: &str = "Implement the change directly in the working tree.";

/// Compose `single-task` over the project layer as it stands, returning stdout.
/// (`--workflow single-task` makes the work-workflow body compose regardless of the
/// `default-workflow` knob, so the per-kind delta is isolated from the scalar flip.)
/// `single-task` is `creates-task: true`, so this mints under `.jigc/tasks/<slug>/`;
/// the task is discarded immediately so a re-compose of the same intent (the
/// no-override → overridden comparison) does not hit the serial-collision guard.
fn compose_single_task(repo: &Path, home: &Path) -> String {
    let out = jigc(
        repo,
        home,
        &["start", "--workflow", "single-task", "add rate limiter"],
    );
    assert_ok(&out, "`jigc start --workflow single-task` compose");
    let discard = jigc(repo, home, &["task", "discard", "add-rate-limiter"]);
    assert_ok(&discard, "`jigc task discard` after a compose");
    String::from_utf8(out.stdout).expect("utf-8 stdout")
}

/// A fresh project repo: a real git repo + the `.jigc/config/` project layer, with
/// no manifest (no delta) — the no-override base every per-kind case forks from.
fn fresh_project(tag: &str) -> (TempDir, TempDir) {
    let repo = TempDir::new(tag);
    let home = TempDir::new("home");
    init_repo(repo.path());
    (repo, home)
}

#[test]
fn scenario_7_no_override_compose_is_byte_identical_to_the_pre_m4_baseline() {
    // The headline determinism assertion, re-asserted through the audit binary: a
    // fresh no-override repo composes the cascade default (the router) byte-identical
    // to the recorded pre-M4 baseline golden. No delta is recorded, so the cascade
    // resolves the pack default and the composed bytes must equal the golden exactly.
    let (repo, home) = fresh_project("xkind-baseline");

    let out = jigc(repo.path(), home.path(), &["start", "add rate limiter"]);
    assert_ok(&out, "the no-override bare compose");
    let stdout = String::from_utf8(out.stdout).expect("utf-8 stdout");
    assert_eq!(
        stdout, NO_OVERRIDE_ROUTER_GOLDEN,
        "the no-override compose must stay byte-identical to the pre-M4 baseline golden",
    );
}

#[test]
fn scenario_7_scalar_set_shifts_composed_bytes_vs_no_override() {
    // `scalar-set` (kind 1): `jigc config set default-workflow single-task` flips the
    // bare-intent compose from the no-mint router catalog to the single-task body —
    // the composed bytes shift vs the no-override compose (the router golden).
    let (repo, home) = fresh_project("xkind-set");

    let no_override = jigc(repo.path(), home.path(), &["start", "add rate limiter"]);
    let no_override = String::from_utf8(no_override.stdout).expect("utf-8");

    let set = jigc(
        repo.path(),
        home.path(),
        &["config", "set", "default-workflow", "single-task"],
    );
    assert_ok(&set, "`jigc config set default-workflow single-task`");

    let overridden = jigc(repo.path(), home.path(), &["start", "add rate limiter"]);
    let overridden = String::from_utf8(overridden.stdout).expect("utf-8");
    assert_ne!(
        no_override, overridden,
        "a scalar-set must shift the composed bytes vs the no-override compose",
    );
    assert!(
        overridden.contains(IMPLEMENT_MARKER) && !overridden.contains("selectable work-workflows"),
        "the flipped compose must be the single-task body, not the router catalog; got:\n{overridden}",
    );
}

#[test]
fn scenario_7_structural_op_shifts_composed_bytes_vs_no_override() {
    // `structural-op` (kind 2): an `insert-step --after implement` splices a project
    // step into the include list; the inserted body composes where no-override has
    // nothing — the composed bytes shift.
    let (repo, home) = fresh_project("xkind-structural");
    let config = repo.path().join(".jigc").join("config");

    let no_override = compose_single_task(repo.path(), home.path());
    const HOUSE_RULE: &str = "Run the project lint probe before you finalize.";
    assert!(
        !no_override.contains(HOUSE_RULE),
        "the no-override compose must not carry the project step body",
    );

    fs::write(config.join("extra.yaml"), format!("{HOUSE_RULE}\n")).expect("write source step");
    let insert = jigc(
        repo.path(),
        home.path(),
        &[
            "config",
            "insert-step",
            "--workflow",
            "single-task",
            "--after",
            "implement",
            ".jigc/config/extra.yaml",
        ],
    );
    assert_ok(&insert, "`jigc config insert-step`");

    let overridden = compose_single_task(repo.path(), home.path());
    assert_ne!(
        no_override, overridden,
        "an insert-step structural-op must shift the composed bytes vs no-override",
    );
    assert!(
        overridden.contains(HOUSE_RULE),
        "the inserted step body must compose into the include list; got:\n{overridden}",
    );
}

#[test]
fn scenario_7_slot_fill_shifts_composed_bytes_vs_no_override() {
    // `slot-fill` (kind 3): `jigc config fill step:implement#extra-guidance` injects
    // content into the pack `implement` step's `{{fill: extra-guidance}}` point; the
    // fill content composes where no-override emits nothing — the bytes shift.
    let (repo, home) = fresh_project("xkind-fill");
    let config = repo.path().join(".jigc").join("config");

    let no_override = compose_single_task(repo.path(), home.path());
    const FILL: &str = "Confirm a changelog entry exists for any user-facing change.";
    assert!(
        !no_override.contains(FILL),
        "the no-override compose must not carry the fill content",
    );

    fs::write(config.join("guidance.txt"), format!("{FILL}\n")).expect("write fill content");
    let fill = jigc(
        repo.path(),
        home.path(),
        &[
            "config",
            "fill",
            "step:implement#extra-guidance",
            "--from-file",
            ".jigc/config/guidance.txt",
        ],
    );
    assert_ok(&fill, "`jigc config fill step:implement#extra-guidance`");

    let overridden = compose_single_task(repo.path(), home.path());
    assert_ne!(
        no_override, overridden,
        "a slot-fill must shift the composed bytes vs no-override",
    );
    assert!(
        overridden.contains(FILL),
        "the fill content must compose into the implement body; got:\n{overridden}",
    );
}

#[test]
fn scenario_7_tracked_fork_shifts_composed_bytes_vs_no_override() {
    // `tracked-fork` (kind 4): `jigc config fork workflow:single-task#implement` copies
    // the resolved `implement` body into a project shadow (a *faithful copy* — still
    // byte-identical), then editing that shadow diverges the compose. The shift vs
    // no-override is the post-edit divergence (the fork is the mechanism that lets the
    // project own + edit the unit).
    let (repo, home) = fresh_project("xkind-fork");

    let no_override = compose_single_task(repo.path(), home.path());

    let fork = jigc(
        repo.path(),
        home.path(),
        &["config", "fork", "workflow:single-task#implement"],
    );
    assert_ok(&fork, "`jigc config fork workflow:single-task#implement`");

    // The fork is a faithful copy: the implement region composes byte-identical until
    // the shadow is edited.
    let native = repo
        .path()
        .join(".jigc")
        .join("config")
        .join("steps")
        .join("implement.yaml");
    let mut body = fs::read(&native).expect("the forked native step");
    const HOUSE_RULE: &str = "House rule: run the project lint probe before you finalize.";
    body.extend_from_slice(format!("\n{HOUSE_RULE}\n").as_bytes());
    fs::write(&native, &body).expect("edit the forked shadow");

    let overridden = compose_single_task(repo.path(), home.path());
    assert_ne!(
        no_override, overridden,
        "an edited tracked-fork must shift the composed bytes vs no-override",
    );
    assert!(
        overridden.contains(HOUSE_RULE) && overridden.contains(IMPLEMENT_MARKER),
        "the edited fork shadow must compose its divergence over the copied pack body; got:\n{overridden}",
    );
}

#[test]
fn scenario_7_undeclared_key_rejection_exits_non_zero_with_its_route() {
    // Rejection 1 (closed-surface): a key the pack does not declare is rejected
    // non-zero with the routed finding, before any write.
    let (repo, home) = fresh_project("xkind-undeclared");

    let set = jigc(
        repo.path(),
        home.path(),
        &["config", "set", "not-a-knob", "anything"],
    );
    assert!(
        !set.status.success(),
        "an undeclared key must exit non-zero; streams:\n{}",
        streams(&set),
    );
    let surfaced = streams(&set);
    assert!(
        surfaced.contains("not-a-knob") && surfaced.contains("route:"),
        "the rejection must name the undeclared key and carry a route; got:\n{surfaced}",
    );
    assert!(
        !repo
            .path()
            .join(".jigc")
            .join("config")
            .join("manifest.yaml")
            .exists(),
        "an undeclared-key rejection must write no manifest",
    );
}

#[test]
fn scenario_7_wrong_type_enum_rejection_exits_non_zero_with_its_route() {
    // Rejection 2 (wrong-type enum): a value outside the knob's declared enum is
    // rejected non-zero with the routed finding, before any write.
    let (repo, home) = fresh_project("xkind-wrong-type");

    let set = jigc(
        repo.path(),
        home.path(),
        &["config", "set", "default-workflow", "not-a-workflow"],
    );
    assert!(
        !set.status.success(),
        "a wrong-type enum value must exit non-zero; streams:\n{}",
        streams(&set),
    );
    let surfaced = streams(&set);
    assert!(
        surfaced.contains("not-a-workflow") && surfaced.contains("route:"),
        "the rejection must name the rejected value and carry a route; got:\n{surfaced}",
    );
    assert!(
        !repo
            .path()
            .join(".jigc")
            .join("config")
            .join("manifest.yaml")
            .exists(),
        "a wrong-type rejection must write no manifest",
    );
}

#[test]
fn scenario_7_orphan_slot_fill_rejection_blocks_compose_with_its_route() {
    // Rejection 3 (hand-authored orphan slot-fill): a `slot-fill` targeting a
    // `<fill-id>` no resolved step body declares is a blocking `slot-fill-orphan`
    // finding. Hand-author the manifest (smuggling past the verb) so the gate fires
    // at compose — the closed-surface check runs live, exits non-zero with its route.
    let (repo, home) = fresh_project("xkind-orphan");
    let config = repo.path().join(".jigc").join("config");

    fs::write(
        config.join("manifest.yaml"),
        "scalar:\n  default-workflow: single-task\n\
         deltas:\n\
         \x20 - kind: slot-fill\n\
         \x20   target: step:implement#nonesuch\n\
         \x20   content: fills/nonesuch.md\n",
    )
    .expect("write a manifest with an orphaned slot-fill delta");
    fs::create_dir_all(config.join("fills")).expect("mk fills/");
    fs::write(config.join("fills").join("nonesuch.md"), "Some guidance.\n")
        .expect("write the native fill content");

    let out = jigc(repo.path(), home.path(), &["start", "add rate limiter"]);
    assert!(
        !out.status.success(),
        "an orphaned slot-fill must block compose non-zero; streams:\n{}",
        streams(&out),
    );
    let surfaced = streams(&out);
    assert!(
        surfaced.contains("nonesuch") && surfaced.contains("route:"),
        "the orphan block must name the fill point and carry a route; got:\n{surfaced}",
    );
}

#[test]
fn scenario_7_explain_overrides_applied_tracks_reality() {
    // `--explain` reflects an applied override: with no delta it reports `overrides
    // applied: none`; after a `replace-step` it reports `overrides applied: 1` with
    // the `← replaces … at position` annotation. The count tracks reality (it is not
    // an inert constant) — proven on the emitted bytes through the binary.
    let (repo, home) = fresh_project("xkind-explain");
    let config = repo.path().join(".jigc").join("config");

    let clean = jigc(
        repo.path(),
        home.path(),
        &[
            "start",
            "--explain",
            "--workflow",
            "single-task",
            "add rate limiter",
        ],
    );
    assert_ok(&clean, "`jigc start --explain` over the unmodified pack");
    let clean = String::from_utf8(clean.stdout).expect("utf-8");
    assert!(
        clean.contains("overrides applied: none"),
        "the unmodified pack must report `overrides applied: none`; got:\n{clean}",
    );

    // Record a replace-step, then re-explain — the count must move to 1.
    fs::write(
        config.join("manifest.yaml"),
        "deltas:\n\
         \x20 - kind: replace-step\n\
         \x20   target: workflow:single-task#implement\n\
         \x20   with: step:project-implement\n",
    )
    .expect("write a manifest with a replace-step delta");
    fs::create_dir_all(config.join("steps")).expect("mk steps/");
    fs::write(
        config.join("steps").join("project-implement.yaml"),
        "{{ include: step:implement }}\n",
    )
    .expect("write the project-implement shadow");

    let overridden = jigc(
        repo.path(),
        home.path(),
        &[
            "start",
            "--explain",
            "--workflow",
            "single-task",
            "add rate limiter",
        ],
    );
    assert_ok(&overridden, "`jigc start --explain` after a replace-step");
    let overridden = String::from_utf8(overridden.stdout).expect("utf-8");
    assert!(
        overridden.contains("overrides applied: 1"),
        "a single replace-step must move the count to `overrides applied: 1`; got:\n{overridden}",
    );
    assert!(
        overridden.contains("replaces step:implement at position 2"),
        "the replace-step slot must carry the `← replaces … at position` annotation; got:\n{overridden}",
    );
}
