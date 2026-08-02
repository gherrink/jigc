//! M42 inc-11 / T5 — **finalize says what it is about to leave out, and says a rejected
//! commit is survivable** (`design/finalize.md` → "The `left-out` advisory prints BEFORE
//! the commit too" and → 6. Commit, "The message must say the recoverability this bullet
//! declares").
//!
//! Two message truths, both driven through the **real** `jigc` binary against throwaway
//! git repos (never a reconstruction — the emitted bytes are the contract):
//!
//! - **(a)** a finalize with one **staged** file and one **unstaged** file prints the
//!   `left-out` advisory **naming the unstaged file BEFORE** the `finalized <hash>` line —
//!   *before* the commit is history, not only after — and still **lands** the commit (the
//!   block stays reserved for the empty-index case; a print refuses nothing);
//! - **(b)** against a repo with a **rejecting `pre-commit` hook**, finalize exits
//!   non-zero with git's **verbatim** hook stderr **and** the sentence `task <id> is
//!   intact` — and the claim is *proved*, not just printed: removing the hook and
//!   re-running the **same** `jigc task finalize <id>` lands the commit;
//! - **(c)** the forecast is *checked against reality*: with jigc's **own** config layer
//!   dirty (`jigc config set …`), the left-out set the **dry-run** forecasts, the set the
//!   **pre-commit** advisory names, and the **landed residual** are one and the same — a
//!   path the finalize itself stages is never named "left-out (… git add to include)".

use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

/// A throwaway directory that removes itself on drop.
struct TempDir(PathBuf);

impl TempDir {
    fn new(tag: &str) -> Self {
        let mut path = std::env::temp_dir();
        path.push(format!(
            "jigc-finalize-truth-{tag}-{}-{:?}",
            std::process::id(),
            engine::tempname::unique_nanos(),
        ));
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

/// The embedded dev pack tree on disk — selected via `JIGC_PACK_DIR`.
fn dev_pack() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("pack")
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
        .trim_end_matches('\n')
        .to_string()
}

/// Initialize a real git repo with one commit.
fn init_repo(repo: &Path) {
    git(repo, &["init", "-q"]);
    git(repo, &["config", "user.email", "test@example.com"]);
    git(repo, &["config", "user.name", "Test"]);
    fs::write(repo.join("README.md"), "hello\n").expect("write README.md");
    git(repo, &["add", "."]);
    git(repo, &["commit", "-q", "-m", "initial"]);
}

/// Run `jigc <args>` with `cwd = repo`, `$HOME = home`, `JIGC_PACK_DIR = pack`, optionally
/// piping `stdin`, capturing output.
fn run_jigc(repo: &Path, home: &Path, args: &[&str], stdin: Option<&[u8]>) -> std::process::Output {
    let mut command = Command::new(env!("CARGO_BIN_EXE_jigc"));
    command
        .args(args)
        .current_dir(repo)
        .env("HOME", home)
        .env("JIGC_PACK_DIR", dev_pack())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());
    if stdin.is_some() {
        command.stdin(Stdio::piped());
    }
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

/// Run `jigc <args>` (no stdin), asserting exit 0, returning stdout.
fn ok_stdout(repo: &Path, home: &Path, args: &[&str], what: &str) -> String {
    let out = run_jigc(repo, home, args, None);
    assert!(
        out.status.success(),
        "`{what}` must exit 0; stdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr),
    );
    String::from_utf8(out.stdout).expect("utf-8 stdout")
}

/// Mint a `single-task` and fill its commit doc's author-required fields/slots.
fn seed_task(repo: &Path, home: &Path, intent: &str) -> String {
    ok_stdout(repo, home, &["setup"], "jigc setup");
    ok_stdout(
        repo,
        home,
        &["start", "--workflow", "single-task", intent],
        "jigc start",
    );
    let task = intent.replace(' ', "-");
    let set_field = |addr: &str, value: &str| {
        let out = run_jigc(
            repo,
            home,
            &["doc", "set-field", addr, "--value", value],
            None,
        );
        assert!(out.status.success(), "set-field {addr} must succeed");
    };
    let set_slot = |addr: &str, prose: &[u8]| {
        let out = run_jigc(
            repo,
            home,
            &["doc", "set-slot", addr, "--from-file", "-"],
            Some(prose),
        );
        assert!(out.status.success(), "set-slot {addr} must succeed");
    };
    set_field(&format!("commit:{task}#type"), "feat");
    set_field(&format!("commit:{task}#scope"), "cache");
    set_slot(&format!("commit:{task}#summary"), b"tell the truth\n");
    set_slot(&format!("commit:{task}#body"), b"A message-truth change.\n");
    task
}

/// The left-out paths a finalize surface names: the shared `left_out_lines` section is a
/// header line followed by one 4-space-indented path per entry (an *included* manifest line
/// carries only 2), so the block reads off any of the three surfaces identically.
fn left_out_paths(surface: &str) -> Vec<String> {
    surface
        .lines()
        .skip_while(|line| !line.trim_start().starts_with("left-out ("))
        .skip(1)
        .take_while(|line| line.starts_with("    "))
        .map(|line| line.trim().to_string())
        .collect()
}

/// (a) The `left-out` advisory prints **before** the commit (M42): one staged file, one
/// unstaged tracked edit → the advisory names the unstaged file ahead of the
/// `finalized <hash>` line, and the commit still lands (a print refuses nothing).
#[test]
fn left_out_is_named_before_the_commit_and_the_commit_still_lands() {
    let repo = TempDir::new("left-out");
    let home = TempDir::new("home-left-out");
    init_repo(repo.path());
    let task = seed_task(repo.path(), home.path(), "surface the left out");

    // The agent stages its own code edit (the agent-stage contract) …
    fs::write(repo.path().join("staged.txt"), "the task's work\n").expect("write staged.txt");
    git(repo.path(), &["add", "staged.txt"]);
    // … and leaves an unstaged tracked edit behind.
    fs::write(repo.path().join("README.md"), "hello\nunstaged WIP\n").expect("edit README.md");

    let stdout = ok_stdout(
        repo.path(),
        home.path(),
        &["task", "finalize", &task],
        "jigc task finalize",
    );

    let landed_at = stdout
        .find("finalized ")
        .unwrap_or_else(|| panic!("the finalize must land and say so; stdout:\n{stdout}"));
    let before_commit = &stdout[..landed_at];
    assert!(
        before_commit.contains("README.md"),
        "the pre-commit advisory must NAME the unstaged file before the `finalized` line; \
         bytes before it:\n{before_commit}\n---\nfull stdout:\n{stdout}",
    );
    assert!(
        before_commit.contains("left-out"),
        "the pre-commit advisory must say what it leaves out; bytes before the `finalized` \
         line:\n{before_commit}",
    );

    // The commit landed, carrying the staged file and NOT the unstaged edit.
    let committed = git(repo.path(), &["show", "--name-only", "--format=", "HEAD"]);
    assert!(
        committed.lines().any(|line| line == "staged.txt"),
        "the staged file must ride the commit; files:\n{committed}",
    );
    assert!(
        !committed.lines().any(|line| line == "README.md"),
        "the unstaged edit must stay OUT of the commit; files:\n{committed}",
    );
    let status = git(repo.path(), &["status", "--porcelain"]);
    assert!(
        status.lines().any(|line| line == " M README.md"),
        "the unstaged edit survives in the working tree; status:\n{status}",
    );
}

/// (b) A rejecting `pre-commit` hook: finalize exits non-zero with git's **verbatim**
/// stderr AND jigc's `task <id> is intact` sentence — and the sentence is TRUE: removing
/// the hook and re-running the same `jigc task finalize <id>` lands the commit.
#[test]
fn hook_rejection_says_the_task_is_intact_and_the_rerun_lands() {
    let repo = TempDir::new("hook");
    let home = TempDir::new("home-hook");
    init_repo(repo.path());
    let task = seed_task(repo.path(), home.path(), "survive the hook");

    fs::write(repo.path().join("code.txt"), "the task's work\n").expect("write code.txt");
    git(repo.path(), &["add", "code.txt"]);
    // HEAD as the task sees it (`jigc setup` lands its own config commit ahead of us).
    let head_before = git(repo.path(), &["rev-parse", "HEAD"]);

    // A rejecting hook whose stderr is the correction signal.
    let hook = repo.path().join(".git").join("hooks").join("pre-commit");
    fs::create_dir_all(hook.parent().expect("hooks dir")).expect("create hooks dir");
    fs::write(
        &hook,
        "#!/bin/sh\necho 'lint: trailing whitespace' 1>&2\nexit 1\n",
    )
    .expect("write pre-commit hook");
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        fs::set_permissions(&hook, fs::Permissions::from_mode(0o755)).expect("chmod hook");
    }

    let rejected = run_jigc(repo.path(), home.path(), &["task", "finalize", &task], None);
    assert!(
        !rejected.status.success(),
        "a hook rejection must exit non-zero; stdout:\n{}",
        String::from_utf8_lossy(&rejected.stdout),
    );
    let stderr = String::from_utf8(rejected.stderr).expect("utf-8 stderr");
    assert!(
        stderr.contains("lint: trailing whitespace"),
        "the hook's own stderr stays VERBATIM (it IS the correction signal); stderr:\n{stderr}",
    );
    assert!(
        stderr.contains(&format!("task {task} is intact")),
        "the rejection must say the task survives; stderr:\n{stderr}",
    );
    assert!(
        stderr.contains(&format!("jigc task finalize {task}")),
        "the rejection must route back to the same re-run; stderr:\n{stderr}",
    );
    // No commit landed.
    assert_eq!(
        git(repo.path(), &["rev-parse", "HEAD"]),
        head_before,
        "a rejected commit must leave HEAD untouched",
    );

    // The sentence is TRUE — fix the hook's complaint, re-run the SAME command, it lands.
    fs::remove_file(&hook).expect("remove the hook");
    ok_stdout(
        repo.path(),
        home.path(),
        &["task", "finalize", &task],
        "the re-run after removing the hook",
    );
    assert_eq!(
        git(repo.path(), &["log", "-1", "--pretty=format:%s"]),
        "feat(cache): tell the truth",
        "the re-run lands the task's commit",
    );
    let committed = git(repo.path(), &["show", "--name-only", "--format=", "HEAD"]);
    assert!(
        committed.lines().any(|line| line == "code.txt"),
        "the still-staged code rides the re-run's commit; files:\n{committed}",
    );
}

/// (c) The forecast must not lie about the paths **jigc itself stages**. With jigc's own
/// git-tracked config layer dirty (`jigc config set docs-root docs` writes an untracked
/// `.jigc/config/manifest.yaml`, which `stage_index_honoring` unconditionally `git add`s),
/// the three surfaces must name **one** left-out set: the `--dry-run` forecast, the
/// pre-commit advisory, and the landed residual. The config file belongs in the *commit*,
/// never under "left-out (unstaged/untracked — git add to include)" — an instruction to
/// `git add` a path the CLI owns and stages. The genuine unstaged user file still appears.
#[test]
fn the_pre_commit_left_out_equals_the_landed_residual_when_jigcs_config_is_dirty() {
    let repo = TempDir::new("config-dirty");
    let home = TempDir::new("home-config-dirty");
    init_repo(repo.path());
    let task = seed_task(repo.path(), home.path(), "own the config layer");

    // jigc's own config layer goes dirty — the CLI owns it, and finalize stages it.
    ok_stdout(
        repo.path(),
        home.path(),
        &["config", "set", "docs-root", "docs"],
        "jigc config set docs-root",
    );
    let config = "  .jigc/config/manifest.yaml";
    assert!(
        git(
            repo.path(),
            &["status", "--porcelain", "--untracked-files=all"]
        )
        .lines()
        .any(|line| line.ends_with(".jigc/config/manifest.yaml")),
        "the premise: jigc's config layer is dirty going into the finalize",
    );

    // The agent stages its code and leaves a genuine unstaged edit behind.
    fs::write(repo.path().join("staged.txt"), "the task's work\n").expect("write staged.txt");
    git(repo.path(), &["add", "staged.txt"]);
    fs::write(repo.path().join("README.md"), "hello\nunstaged WIP\n").expect("edit README.md");

    let dry_run = ok_stdout(
        repo.path(),
        home.path(),
        &["task", "finalize", &task, "--dry-run"],
        "jigc task finalize --dry-run",
    );
    let stdout = ok_stdout(
        repo.path(),
        home.path(),
        &["task", "finalize", &task],
        "jigc task finalize",
    );
    let landed_at = stdout
        .find("finalized ")
        .unwrap_or_else(|| panic!("the finalize must land and say so; stdout:\n{stdout}"));

    let forecast = left_out_paths(&dry_run);
    let pre_commit = left_out_paths(&stdout[..landed_at]);
    let landed = left_out_paths(&stdout[landed_at..]);

    // One truth, three surfaces — the forecast is what the commit actually leaves behind.
    assert_eq!(
        pre_commit, landed,
        "the pre-commit left-out set must BE the landed residual; dry-run:\n{dry_run}\n---\n\
         finalize:\n{stdout}",
    );
    assert_eq!(
        forecast, pre_commit,
        "the --dry-run forecast must name the same set; dry-run:\n{dry_run}\n---\n\
         finalize:\n{stdout}",
    );
    assert!(
        !pre_commit
            .iter()
            .any(|path| path == ".jigc/config/manifest.yaml"),
        "jigc must never tell the agent to `git add` the config layer IT stages; left-out:\n\
         {pre_commit:?}\nfinalize:\n{stdout}",
    );
    assert!(
        pre_commit.iter().any(|path| path == "README.md"),
        "the genuine unstaged user edit is still named; left-out:\n{pre_commit:?}",
    );

    // …because the commit carries it: the config layer landed, the unstaged edit did not.
    let committed = git(repo.path(), &["show", "--name-only", "--format=", "HEAD"]);
    assert!(
        committed
            .lines()
            .any(|line| line == ".jigc/config/manifest.yaml"),
        "the config layer rides the commit (which is WHY it is not left out); files:\n{committed}",
    );
    assert!(
        stdout[landed_at..].contains(config.trim()),
        "the landed manifest names the config layer as an included member; stdout:\n{stdout}",
    );
    assert!(
        !committed.lines().any(|line| line == "README.md"),
        "the unstaged edit stays out of the commit; files:\n{committed}",
    );
}
