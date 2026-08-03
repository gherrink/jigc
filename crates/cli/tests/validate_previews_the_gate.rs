//! M47 Increment 4 / T1 — **the carryover gate previews at `jigc task validate`**, and
//! `--carry-staged` gives the preview the consent the gate accepts
//! (`DECISIONS.md` → 2026-07-26 M47 Settle, Decision 1 + the cross-model review's
//! blocker 3; `design/command-output-contract.md`:321 — the exit-code table).
//!
//! Until M47 `jigc task validate` exited **0** over a state `jigc task finalize`
//! refuses at **3**: the carryover decision ran on the committing path only, so the
//! surface that every composed step calls *"see what's left before committing"*
//! could not see the wave's #1-ranked v1 gate. The preview now runs the same pure
//! [`engine::finalize::decide_carryover`] over the same mint-time snapshot — **no
//! staging, no mutation** — and the same blocking `finalize.carried-staged` per
//! carried path, keyed at the file path.
//!
//! Two invariants this suite pins alongside the fix:
//!
//! * **The new door speaks for itself** (surface-contract law 1): the finding is read
//!   from `task validate`, so a route text presuming the finalize invocation would be
//!   a lie. The preview carries the `TaskPreview` wording; the **finalize** door's
//!   bytes are pinned here verbatim and are unchanged.
//! * **`--carry-staged` joins the preview** — the cross-model review's scoping of the
//!   non-breaking argument: validate newly exits 3 only on states **default**
//!   `finalize` refuses, never on the ones `finalize --carry-staged` accepts, so a
//!   driver that always intends to carry is not handed a permanently-red preview.
//!
//! Plus the two no-fire bounds: a task with nothing pre-staged validates clean, and a
//! snapshot-less (pre-M43) task validates clean — the declared fail-open bound holds
//! at the new door exactly as it does at the committing one.
//!
//! Every arm drives the real binary and asserts on the emitted findings envelope.

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
            "jigc-validate-preview-{tag}-{}-{:?}",
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

/// Initialize a real git repo with one commit (a tracked `README.md`).
fn init_repo(repo: &Path) {
    git(repo, &["init", "-q"]);
    git(repo, &["config", "user.email", "test@example.com"]);
    git(repo, &["config", "user.name", "Test"]);
    fs::write(repo.join("README.md"), "hello\n").expect("write README");
    git(repo, &["add", "README.md"]);
    git(repo, &["commit", "-q", "-m", "initial"]);
}

/// Run `jigc <args>` with `cwd = repo`, `$HOME = home`, against the embedded packs.
fn jigc(repo: &Path, home: &Path, args: &[&str], stdin: Option<&[u8]>) -> std::process::Output {
    let mut command = Command::new(env!("CARGO_BIN_EXE_jigc"));
    command
        .args(args)
        .current_dir(repo)
        .env("HOME", home)
        .env_remove("JIGC_PACK_DIR")
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

/// Run `jigc <args>` (no stdin), asserting exit 0.
fn ok(repo: &Path, home: &Path, args: &[&str], what: &str) {
    let out = jigc(repo, home, args, None);
    assert!(
        out.status.success(),
        "`{what}` must exit 0; stdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr),
    );
}

/// Fill every author-required field/slot of the provisioned commit doc.
fn fill_commit(repo: &Path, home: &Path, task: &str) {
    ok(
        repo,
        home,
        &[
            "doc",
            "set-field",
            &format!("commit:{task}#type"),
            "--value",
            "feat",
        ],
        "doc set-field type",
    );
    ok(
        repo,
        home,
        &[
            "doc",
            "set-field",
            &format!("commit:{task}#scope"),
            "--value",
            "gate",
        ],
        "doc set-field scope",
    );
    let set_slot = |addr: &str, prose: &[u8]| {
        let out = jigc(
            repo,
            home,
            &["doc", "set-slot", addr, "--from-file", "-"],
            Some(prose),
        );
        assert!(out.status.success(), "set-slot {addr} must succeed");
    };
    set_slot(&format!("commit:{task}#summary"), b"preview the gate\n");
    set_slot(&format!("commit:{task}#body"), b"An M47 change.\n");
}

/// Mint a `single-task` task and do its work: write + `git add` a task edit
/// (post-mint staging — the task's own), fill the commit doc.
fn mint_and_work(repo: &Path, home: &Path, intent: &str, task: &str, edit: &str) {
    ok(
        repo,
        home,
        &["start", "--workflow", "single-task", intent],
        "jigc start",
    );
    fs::write(repo.join(edit), "pub fn work() {}\n").expect("write task edit");
    git(repo, &["add", edit]);
    fill_commit(repo, home, task);
}

const TASK: &str = "preview-the-carryover-gate";
const INTENT: &str = "preview the carryover gate";

/// The corpus every arm shares: a repo with jigc set up, an optional foreign
/// pre-mint staged plant, and a minted, worked `single-task`.
fn corpus(tag: &str, plant: bool) -> (TempDir, TempDir) {
    let repo = TempDir::new(tag);
    let home = TempDir::new(&format!("{tag}-home"));
    init_repo(repo.path());
    ok(repo.path(), home.path(), &["setup"], "jigc setup");
    if plant {
        // The foreign pre-staged adds — staged BEFORE the task exists.
        fs::write(repo.path().join("foreign-a.txt"), "not this task's work\n").expect("write a");
        fs::write(repo.path().join("foreign-b.txt"), "also not\n").expect("write b");
        git(repo.path(), &["add", "foreign-a.txt", "foreign-b.txt"]);
    }
    mint_and_work(repo.path(), home.path(), INTENT, TASK, "feature.rs");
    (repo, home)
}

/// The `findings` array of a `--format json` envelope on stdout.
fn findings(out: &std::process::Output, what: &str) -> Vec<serde_json::Value> {
    let stdout = String::from_utf8_lossy(&out.stdout);
    let value: serde_json::Value = serde_json::from_str(&stdout).unwrap_or_else(|e| {
        panic!("`{what}` emits the pinned findings envelope on stdout ({e}); got:\n{stdout}")
    });
    value["findings"]
        .as_array()
        .unwrap_or_else(|| panic!("the envelope carries a `findings` array; got:\n{stdout}"))
        .clone()
}

/// The `finalize.carried-staged` findings in an envelope.
fn carried_staged(findings: &[serde_json::Value]) -> Vec<serde_json::Value> {
    findings
        .iter()
        .filter(|f| f["code"] == "finalize.carried-staged")
        .cloned()
        .collect()
}

/// Assert the exit code, with both streams in the failure message.
fn assert_exit(out: &std::process::Output, code: i32, what: &str) {
    assert_eq!(
        out.status.code(),
        Some(code),
        "`{what}` must exit {code}; stdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr),
    );
}

/// **The preview fires**: a pre-staged-before-mint plant is *reported at
/// `jigc task validate`* rather than discovered at `jigc task finalize` — exit 3 with
/// one blocking `finalize.carried-staged` per carried path, keyed at the file path,
/// routed at both exits. The task's own post-mint staged edit never fires.
///
/// The finding speaks for **its own door** (law 1): it names the `jigc task finalize`
/// that will refuse, and never claims a finalize was run.
#[test]
fn a_pre_staged_before_mint_plant_is_reported_at_validate() {
    let (repo, home) = corpus("fires", true);
    let before = git(repo.path(), &["rev-list", "--count", "HEAD"]);

    let out = jigc(
        repo.path(),
        home.path(),
        &["task", "validate", TASK, "--format", "json"],
        None,
    );
    assert_exit(&out, 3, "jigc task validate (pre-mint staged plant)");
    let findings = findings(&out, "jigc task validate");
    let carried = carried_staged(&findings);
    assert_eq!(
        carried.len(),
        2,
        "exactly ONE finding per carried path (two foreign paths staged); got:\n{findings:#?}"
    );
    for (finding, path) in carried.iter().zip(["foreign-a.txt", "foreign-b.txt"]) {
        assert_eq!(
            finding["severity"], "blocking",
            "the preview is blocking — it previews a block; got: {finding}"
        );
        assert_eq!(
            finding["key"]["target"], path,
            "the finding keys at the carried file path; got: {finding}"
        );
        let message = finding["message"].as_str().expect("message is a string");
        let route = finding["route"].as_str().expect("route is a string");
        assert!(
            message.contains("jigc task finalize"),
            "the preview names the door that WILL refuse; got: {message}"
        );
        assert!(
            !message.contains("refusing to let"),
            "the preview never claims a finalize refused (law 1); got: {message}"
        );
        assert!(
            route.contains(&format!("git restore --staged -- {path}"))
                && route.contains("--carry-staged"),
            "the route names both exits (unstage, or declare); got: {route}"
        );
    }
    assert!(
        !carried
            .iter()
            .any(|f| f["key"]["target"] == "feature.rs" || f["key"]["target"] == "README.md"),
        "the task's own post-mint staged edit is never carried; got:\n{findings:#?}"
    );

    // A pure reader: no commit, and the plant is still staged exactly as it was.
    assert_eq!(
        git(repo.path(), &["rev-list", "--count", "HEAD"]),
        before,
        "`task validate` commits nothing"
    );
    let staged = git(repo.path(), &["diff", "--cached", "--name-only"]);
    let mut names: Vec<&str> = staged.lines().collect();
    names.sort_unstable();
    assert!(
        names.contains(&"foreign-a.txt") && names.contains(&"foreign-b.txt"),
        "the preview stages and unstages nothing; index:\n{staged}"
    );
}

/// **`--carry-staged` gives the preview the consent the gate accepts**: the identical
/// state validates at exit 0 with no carryover finding — so a driver that always
/// intends to carry gets a preview of *its own* finalize, not a permanently-red one.
#[test]
fn carry_staged_gives_the_preview_the_consent_the_gate_accepts() {
    let (repo, home) = corpus("consent", true);

    let out = jigc(
        repo.path(),
        home.path(),
        &[
            "task",
            "validate",
            TASK,
            "--carry-staged",
            "--format",
            "json",
        ],
        None,
    );
    assert_exit(&out, 0, "jigc task validate --carry-staged");
    let findings = findings(&out, "jigc task validate --carry-staged");
    assert!(
        carried_staged(&findings).is_empty(),
        "a declared carry-over is not a finding; got:\n{findings:#?}"
    );
}

/// **No false fire**: a task with nothing pre-staged validates clean, exactly as it
/// did before the preview existed.
#[test]
fn a_task_with_nothing_pre_staged_validates_clean() {
    let (repo, home) = corpus("clean", false);

    let out = jigc(
        repo.path(),
        home.path(),
        &["task", "validate", TASK, "--format", "json"],
        None,
    );
    assert_exit(&out, 0, "jigc task validate (nothing pre-staged)");
    assert!(
        carried_staged(&findings(&out, "jigc task validate")).is_empty(),
        "post-mint staging never trips the preview"
    );
}

/// **The fail-open bound holds at the new door**: a snapshot-less task (minted before
/// the M43 gate existed) validates clean — the same declared bound `decide_carryover`
/// honours on the committing path, unchanged by the new caller.
#[test]
fn a_snapshot_less_task_validates_clean() {
    let (repo, home) = corpus("fail-open", true);

    // Reproduce the pre-M43 working area: no mint-time staged snapshot.
    let snapshot = repo
        .path()
        .join(".jigc/tasks")
        .join(TASK)
        .join("staged-snapshot.json");
    assert!(snapshot.is_file(), "the mint writes {snapshot:?}");
    fs::remove_file(&snapshot).expect("remove the snapshot");

    let out = jigc(
        repo.path(),
        home.path(),
        &["task", "validate", TASK, "--format", "json"],
        None,
    );
    assert_exit(&out, 0, "jigc task validate (snapshot-less task)");
    assert!(
        carried_staged(&findings(&out, "jigc task validate")).is_empty(),
        "a snapshot-less task fails open at the preview door too"
    );
}

/// **Finalize is untouched** — the hard invariant. Over the identical planted state
/// the committing door still blocks at exit 3 with its own verbatim message and
/// route (pinned byte-for-byte here: the preview must not have been wired through a
/// shared seam that re-words or re-positions the refusal), and
/// `finalize --carry-staged` still lands the declared carry-over in the commit.
#[test]
fn the_finalize_door_is_byte_identical() {
    let (repo, home) = corpus("finalize", true);
    let before = git(repo.path(), &["rev-list", "--count", "HEAD"]);

    let out = jigc(
        repo.path(),
        home.path(),
        &["task", "finalize", TASK, "--format", "json"],
        None,
    );
    assert_exit(&out, 3, "jigc task finalize (pre-mint staged plant)");
    let carried = carried_staged(&findings(&out, "jigc task finalize"));
    assert_eq!(carried.len(), 2, "one finding per carried path");
    for (finding, path) in carried.iter().zip(["foreign-a.txt", "foreign-b.txt"]) {
        assert_eq!(
            finding["message"],
            format!(
                "`{path}` was already staged before this task existed — refusing to let a \
                 pre-task staged change silently ride this task's commit"
            ),
            "the committing door's message is unchanged"
        );
        assert_eq!(
            finding["route"],
            format!(
                "unstage it (`git restore --staged -- {path}`) if it is not this task's work, \
                 or re-run the finalize with `--carry-staged` to declare the carry-over \
                 deliberate"
            ),
            "the committing door's route is unchanged"
        );
    }
    assert_eq!(
        git(repo.path(), &["rev-list", "--count", "HEAD"]),
        before,
        "the blocked finalize commits nothing"
    );

    ok(
        repo.path(),
        home.path(),
        &["task", "finalize", TASK, "--carry-staged"],
        "jigc task finalize --carry-staged",
    );
    let landed = git(repo.path(), &["show", "--name-only", "--format=", "HEAD"]);
    assert!(
        landed.lines().any(|l| l == "foreign-a.txt")
            && landed.lines().any(|l| l == "foreign-b.txt")
            && landed.lines().any(|l| l == "feature.rs"),
        "the declared carry-over still rides the whole-index commit; files:\n{landed}"
    );
}
