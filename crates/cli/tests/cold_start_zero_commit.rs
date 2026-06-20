//! Flow-A cold-start: the literal first commands of a new project must run on a
//! **fresh `git init` with ZERO commits** (`design/project-setup.md` → Flow 2
//! hardening — zero-commit; `implementation/roadmap.md` → M21 Increment 2).
//!
//! Today `jigc start` (and finalize) shell out to `git rev-parse HEAD`, which a repo
//! with no commits cannot resolve — the mint crashes before opening the working area.
//! The zero-commit sentinel detects an **unborn** HEAD distinctly (`git rev-parse
//! --verify -q HEAD` exits 1, vs 128 for a genuinely broken/missing git) and pins the
//! task to the canonical **empty-tree** SHA, so every `creates-task` workflow runs
//! pre-first-commit and the first finalize diffs against the empty tree and commits
//! cleanly as the repo's first commit.
//!
//! Four real-binary (`CARGO_BIN_EXE_jigc`) assertions over a throwaway `git init`:
//!   (i)   on a zero-commit repo, `jigc start --workflow single-task "<intent>"` (run
//!         WITHOUT setup, so HEAD stays unborn) mints against the empty-tree sentinel —
//!         exit 0, the working area + a `base.json` pinning the empty-tree SHA appear —
//!         where today it crashes;
//!   (ii)  the cold-start install footprint lands in git: `jigc setup` on an unborn
//!         HEAD MINTS the repo's first commit carrying jigc's own install files
//!         (`CLAUDE.md` / `.claude/settings.json` / `.jigc/AGENT.md`), so they are
//!         tracked before any work commit (M30 audit finding 1 — setup owns committing
//!         its install regardless of HEAD state; finalize since M30 stages only the
//!         task's change-set and would otherwise leave the footprint untracked);
//!   (iii) after that setup commit, `jigc start` + a conformant commit-doc fill + a code
//!         change → `jigc task finalize` lands a SECOND commit carrying the code change +
//!         promoted docs, exit 0 — no empty-commit-guard false-abort;
//!   (iv)  an existing repo WITH a seed commit still pins/diffs against the real HEAD
//!         SHA — the sentinel never fires when HEAD resolves.
//!
//! No external test crates: the binary path comes from `CARGO_BIN_EXE_jigc`, the temp
//! repo is a real `git init`, and a self-cleaning `TempDir` keeps the test off the
//! developer's repo.

use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

/// The canonical git empty-tree SHA — the sentinel base an unborn HEAD pins to.
const EMPTY_TREE_SHA: &str = "4b825dc642cb6eb9a060e54bf8d69288fbee4904";

/// A throwaway directory that removes itself on drop.
struct TempDir(PathBuf);

impl TempDir {
    fn new(tag: &str) -> Self {
        let mut path = std::env::temp_dir();
        let unique = format!(
            "jigc-coldstart-{tag}-{}-{:?}",
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

/// A real `git init` repo with identity configured but **no seed commit** (the
/// zero-commit precondition) — the `finalize_to_git.rs` harness minus its
/// `git commit -q -m initial`. Returns the repo + a `$HOME` temp dir.
fn fresh_unborn_repo() -> (TempDir, TempDir) {
    let repo = TempDir::new("repo");
    let home = TempDir::new("home");
    git(repo.path(), &["init", "-q"]);
    git(repo.path(), &["config", "user.email", "test@example.com"]);
    git(repo.path(), &["config", "user.name", "Test"]);
    fs::create_dir_all(repo.path().join(".jigc").join("config")).expect("create project layer");
    (repo, home)
}

/// Run `jigc <args>` with `cwd = repo` and a temp `$HOME`.
fn run_jigc(repo: &Path, home: &Path, args: &[&str]) -> std::process::Output {
    Command::new(env!("CARGO_BIN_EXE_jigc"))
        .args(args)
        .current_dir(repo)
        .env("HOME", home)
        .output()
        .expect("run jigc")
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

/// Fill every author-required field/slot of the provisioned commit doc so a `finalize`
/// over it validates clean (the `finalize_to_git.rs` idiom).
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

/// Assert the repo's HEAD is unborn (no commits) — the zero-commit precondition.
fn assert_unborn(repo: &Path) {
    assert!(
        !Command::new("git")
            .args(["rev-parse", "--verify", "-q", "HEAD"])
            .current_dir(repo)
            .output()
            .expect("run git")
            .status
            .success(),
        "the repo must start with an unborn HEAD (no commits)"
    );
}

/// (i) The sentinel proper: on a zero-commit repo, `jigc start` (run WITHOUT setup, so
/// HEAD stays unborn) must mint against the empty-tree sentinel rather than crashing on
/// `git rev-parse HEAD`.
#[test]
fn zero_commit_repo_starts_against_the_empty_tree_sentinel() {
    let (repo, home) = fresh_unborn_repo();
    let task = "add-rate-limiter";
    assert_unborn(repo.path());

    let start = run_jigc(
        repo.path(),
        home.path(),
        &["start", "--workflow", "single-task", "add rate limiter"],
    );
    assert!(
        start.status.success(),
        "`jigc start` must mint on a zero-commit repo (no `git rev-parse HEAD` crash); \
         stdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&start.stdout),
        String::from_utf8_lossy(&start.stderr)
    );

    // The working area appeared, and base.json pins the empty-tree sentinel SHA.
    let area = repo.path().join(".jigc").join("tasks").join(task);
    assert!(area.exists(), "mint must open the working area at {area:?}");
    let base_json = fs::read_to_string(area.join("base.json")).expect("base pin written");
    assert!(
        base_json.contains(EMPTY_TREE_SHA),
        "the base pin must record the empty-tree sentinel SHA on a zero-commit repo; got:\n{base_json}"
    );
}

/// (ii) + (iii): the cold-start install footprint lands in git. On a zero-commit repo,
/// `jigc setup` MINTS the repo's first commit carrying jigc's own install files, so
/// `CLAUDE.md` / `.claude/settings.json` / `.jigc/AGENT.md` are tracked before any work
/// commit — then `jigc start` + a conformant fill + a code change → `jigc task finalize`
/// lands a SECOND commit with the code change (M30 audit finding 1).
#[test]
fn cold_start_setup_commits_its_install_footprint_then_finalize_lands_the_work() {
    let (repo, home) = fresh_unborn_repo();
    let task = "add-rate-limiter";
    assert_unborn(repo.path());

    // `jigc setup` installs the adapter — must run pre-first-commit.
    let setup = run_jigc(repo.path(), home.path(), &["setup"]);
    assert!(
        setup.status.success(),
        "`jigc setup` must run on a zero-commit repo; stderr:\n{}",
        String::from_utf8_lossy(&setup.stderr)
    );

    // (ii) setup minted the repo's first commit and jigc's install footprint is TRACKED
    // in git (not left untracked for a manual `git add`).
    let count_after_setup: u32 = git(repo.path(), &["rev-list", "--count", "HEAD"])
        .parse()
        .unwrap();
    assert_eq!(
        count_after_setup, 1,
        "`jigc setup` on an unborn HEAD must mint the repo's first commit (the install)"
    );
    let tracked = git(repo.path(), &["ls-files"]);
    for f in ["CLAUDE.md", ".claude/settings.json", ".jigc/AGENT.md"] {
        assert!(
            tracked.lines().any(|l| l == f),
            "setup's install footprint `{f}` must be tracked in git after a cold-start setup; \
             tracked:\n{tracked}"
        );
    }

    // (iii) `jigc start --workflow single-task` mints over the install commit.
    let start = run_jigc(
        repo.path(),
        home.path(),
        &["start", "--workflow", "single-task", "add rate limiter"],
    );
    assert!(
        start.status.success(),
        "`jigc start` must mint after a cold-start setup; stdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&start.stdout),
        String::from_utf8_lossy(&start.stderr)
    );
    let area = repo.path().join(".jigc").join("tasks").join(task);
    assert!(area.exists(), "mint must open the working area at {area:?}");

    // A conformant commit-doc fill + a code change → finalize lands the work commit.
    fs::write(
        repo.path().join("limiter.rs"),
        "// a per-client rate limiter\n",
    )
    .expect("write code change");
    git(repo.path(), &["add", "limiter.rs"]);
    make_commit_conformant(repo.path(), home.path(), task);

    let finalize = run_jigc(repo.path(), home.path(), &["task", "finalize", task]);
    assert!(
        finalize.status.success(),
        "the cold-start `jigc task finalize` must exit 0; stdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&finalize.stdout),
        String::from_utf8_lossy(&finalize.stderr)
    );

    // Two commits exist now: the setup install commit + the finalize work commit.
    let count: u32 = git(repo.path(), &["rev-list", "--count", "HEAD"])
        .parse()
        .unwrap();
    assert_eq!(
        count, 2,
        "a cold start lands TWO commits: setup's install commit, then the finalize work commit"
    );

    // The code change rode in the finalize (HEAD) commit.
    let files = git(repo.path(), &["show", "--name-only", "--format=", "HEAD"]);
    assert!(
        files.lines().any(|l| l == "limiter.rs"),
        "the code change must land in the finalize commit; files:\n{files}"
    );

    // The install footprint is still tracked after finalize.
    let tracked_after = git(repo.path(), &["ls-files"]);
    for f in ["CLAUDE.md", ".claude/settings.json", ".jigc/AGENT.md"] {
        assert!(
            tracked_after.lines().any(|l| l == f),
            "the install footprint `{f}` must remain tracked after finalize; tracked:\n{tracked_after}"
        );
    }

    // The working area is gone (phase 7).
    assert!(
        !area.exists(),
        "finalize must remove the working area at {area:?}"
    );
}

/// M30 audit finding 2 (re-verify, resolved by finding 1) — on a cold start the `finalize`
/// **dry-run forecast** and the **landed manifest** must agree (G3 dry-run/landed
/// symmetry). Finding 1's fix (setup commits the `.jigc/config` layer on the unborn HEAD)
/// makes the config layer already-tracked by the first finalize, so it leaks into NEITHER
/// surface's `left_out`. This guards the symmetry so a future staging change cannot
/// silently reintroduce the asymmetry the audit flagged.
#[test]
fn cold_start_dry_run_and_landed_manifests_agree_on_the_config_layer() {
    let (repo, home) = fresh_unborn_repo();
    let task = "add-rate-limiter";
    assert_unborn(repo.path());

    let setup = run_jigc(repo.path(), home.path(), &["setup"]);
    assert!(
        setup.status.success(),
        "`jigc setup` must run on a cold start"
    );
    let start = run_jigc(
        repo.path(),
        home.path(),
        &["start", "--workflow", "single-task", "add rate limiter"],
    );
    assert!(start.status.success(), "`jigc start` must mint after setup");

    fs::write(
        repo.path().join("limiter.rs"),
        "// a per-client rate limiter\n",
    )
    .expect("write code change");
    git(repo.path(), &["add", "limiter.rs"]);
    make_commit_conformant(repo.path(), home.path(), task);

    // The dry-run forecast (commits nothing) ...
    let dry = run_jigc(
        repo.path(),
        home.path(),
        &["--format", "json", "task", "finalize", task, "--dry-run"],
    );
    assert!(dry.status.success(), "the dry-run must exit 0");
    let dry: serde_json::Value =
        serde_json::from_str(&String::from_utf8(dry.stdout).expect("utf-8")).expect("dry JSON");

    // ... then the real landed finalize.
    let landed = run_jigc(
        repo.path(),
        home.path(),
        &["--format", "json", "task", "finalize", task],
    );
    assert!(landed.status.success(), "the landed finalize must exit 0");
    let landed: serde_json::Value =
        serde_json::from_str(&String::from_utf8(landed.stdout).expect("utf-8"))
            .expect("landed JSON");
    let committed = &landed["committed"];

    // Dry-run and landed agree on both the included manifest and the left-out residual.
    assert_eq!(
        dry["manifest"], committed["manifest"],
        "dry-run and landed must forecast the same included set (G3 symmetry)",
    );
    assert_eq!(
        dry["left_out"], committed["left_out"],
        "dry-run and landed must forecast the same left-out set (G3 symmetry)",
    );

    // The config layer leaks into NEITHER left_out (setup already committed it — finding 1).
    let mentions_config = |v: &serde_json::Value| {
        v.as_array().is_some_and(|arr| {
            arr.iter().any(|e| {
                e["path"]
                    .as_str()
                    .is_some_and(|p| p.starts_with(".jigc/config") || p == ".jigc/.gitignore")
            })
        })
    };
    assert!(
        !mentions_config(&dry["left_out"]),
        "the config layer must not be forecast as left-out (it is already tracked); dry:\n{dry}",
    );
    assert!(
        !mentions_config(&committed["left_out"]),
        "the config layer must not land as left-out; committed:\n{committed}",
    );
}

/// (iv) The hardening #5 omitting-context guard: an existing repo WITH a seed commit
/// must still pin against the **real** HEAD SHA — the sentinel must never fire when
/// HEAD resolves.
#[test]
fn repo_with_a_seed_commit_pins_against_the_real_head_not_the_sentinel() {
    let (repo, home) = fresh_unborn_repo();
    fs::write(repo.path().join("README.md"), "hello\n").expect("write file");
    git(repo.path(), &["add", "."]);
    git(repo.path(), &["commit", "-q", "-m", "initial"]);
    let head_sha = git(repo.path(), &["rev-parse", "HEAD"]);

    let start = run_jigc(
        repo.path(),
        home.path(),
        &["start", "--workflow", "single-task", "add rate limiter"],
    );
    assert!(
        start.status.success(),
        "`jigc start` must mint on a seeded repo; stderr:\n{}",
        String::from_utf8_lossy(&start.stderr)
    );

    let base_json = fs::read_to_string(
        repo.path()
            .join(".jigc")
            .join("tasks")
            .join("add-rate-limiter")
            .join("base.json"),
    )
    .expect("base pin written");
    assert!(
        base_json.contains(&head_sha),
        "the base pin must record the REAL HEAD SHA when HEAD resolves; got:\n{base_json}"
    );
    assert!(
        !base_json.contains(EMPTY_TREE_SHA),
        "the sentinel must never fire when HEAD resolves; got:\n{base_json}"
    );
}
