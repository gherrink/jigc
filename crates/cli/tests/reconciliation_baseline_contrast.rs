//! M46 Increment 1 / T4, restaged by the rc.24 fix pass `(R3, F7)` — **the baseline
//! contrast**: what the `file-state` baseline is *worth*, and what its absence costs,
//! pinned as arms of one fixture rather than described.
//!
//! `CLAUDE.md` → Architectural invariants states that out-of-band edits are
//! *"detected and routed — conformant non-conflicts absorbed, conflicts blocked and
//! routed to a human, **never silently merged**"*. That guarantee is a property of the
//! reconciliation state machine **plus a recorded baseline**:
//! `engine::file_state::reconcile_committed` reaches its `DRIFTED + TOUCHED →
//! conflict-block` arm only from `record.get(path) == Some(_)`.
//!
//! The arms are one function with **one branch**: identical repo, identical committed
//! ADR, identical out-of-band human edit (made before the task's first touch),
//! identical in-task write over the same doc. The only difference is whether — and
//! *when* — the ADR's key is missing from `.jigc/state/file-state.json`.
//!
//! | arm | the ADR's key | outcome |
//! |---|---|---|
//! | **A** | present throughout | `reconciliation.conflict-block`, exit 3 ([`cli::task::EXIT_VALIDATION_BLOCKED`]), **no commit** |
//! | **B** | absent **when the task first touches the doc** | exit 0, a commit lands carrying **both** sides' bytes — the declared merge order |
//! | **C** | lost **after** the task's first touch | `reconciliation.conflict-block`, exit 3, **no commit** — the base-pin backstop |
//!
//! **Arm B is the cost `design/storage.md` → Concurrent writers declares, and it is a
//! merge, never a loss.** The task's copy-in carries the human's edit and records it as
//! the doc's baseline (`engine::file_state::read_for_copy_in`), so the finalize lands
//! both sides. Its fixture is not defect-only: `.jigc/` is gitignored, so a teammate's
//! fresh clone has no record at all; `jigc unmanage` forgets a key by design; a deleted
//! cache directory is sanctioned.
//!
//! **Arm C is the arm this suite used to pin as the silent merge.** Until the rc.24 fix
//! pass a key lost at *any* point took the `UNKNOWN → baseline-adopt` arm — an advisory —
//! and the finalize landed at exit 0. For this order of the edit that was a merge; for
//! the other order (the hand edit *after* the task's first write) it was an exit-0
//! **overwrite** of the human's bytes
//! (`completions/artifacts/M55/per-axis-review-rc24/tier1-verification/R3-F7.md`), and
//! with no record the staged copy cannot say which order it is in. So a touched doc with
//! no record is decided by its base pin, and bytes that differ from the pin's blob block.
//! The class is iterated in `copy_in_baseline.rs`; this suite keeps the three-arm
//! contrast.
//!
//! The arms drop the key through the engine API itself (`load` → `forget` → `save`),
//! never by hand-editing bytes.
//!
//! Real binary throughout (`CARGO_BIN_EXE_jigc`), a real `git init` repo, a
//! self-cleaning `TempDir`. No external test crates.

use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

/// The committed ADR both arms drift, at its canonical path / file-state key.
const ADR_PATH: &str = "docs/decisions/single-node-cache.md";

/// The prose the *human* writes out of band, straight into the committed file.
const HUMAN_PROSE: &str = "A cold node loses its sessions; clients re-authenticate.";

/// The prose the *task* writes through the CLI, into a different slot of the same doc.
const TASK_PROSE: &str = "A single in-memory node keeps lookups fast, for now.";

/// A throwaway directory that removes itself on drop.
struct TempDir(PathBuf);

impl TempDir {
    fn new(tag: &str) -> Self {
        let mut path = std::env::temp_dir();
        path.push(format!(
            "jigc-baseline-contrast-{tag}-{}-{:?}",
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

/// Run a `git` command in `repo`, asserting success.
fn git(repo: &Path, args: &[&str]) -> String {
    let out = Command::new("git")
        .args(args)
        .current_dir(repo)
        .output()
        .expect("run git");
    assert!(
        out.status.success(),
        "git {args:?} failed: {}",
        String::from_utf8_lossy(&out.stderr),
    );
    String::from_utf8_lossy(&out.stdout).to_string()
}

/// The number of commits reachable from HEAD.
fn commit_count(repo: &Path) -> u32 {
    git(repo, &["rev-list", "--count", "HEAD"])
        .trim()
        .parse()
        .expect("parse the commit count")
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
        crate::support::child_stdin::feed(&mut child, bytes);
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

/// Fill every author-required field/slot of the provisioned commit doc so the task
/// can reach the reconcile gate rather than dying on its own conformance.
fn fill_commit(repo: &Path, home: &Path, task: &str, summary: &str) {
    let set_field = |addr: &str, value: &str| {
        assert_ok(
            &jigc_doc(repo, home, &["set-field", addr, "--value", value], None),
            &format!("set-field {addr}"),
        );
    };
    let set_slot = |addr: &str, prose: &[u8]| {
        assert_ok(
            &jigc_doc(
                repo,
                home,
                &["set-slot", addr, "--from-file", "-"],
                Some(prose),
            ),
            &format!("set-slot {addr}"),
        );
    };
    set_field(&format!("commit:{task}#type"), "feat");
    set_field(&format!("commit:{task}#scope"), "cache");
    set_slot(
        &format!("commit:{task}#summary"),
        format!("{summary}\n").as_bytes(),
    );
    set_slot(&format!("commit:{task}#body"), b"A cache change.\n");
}

/// Set one slot of a managed doc, asserting success.
fn set_slot(repo: &Path, home: &Path, addr: &str, prose: &str) {
    assert_ok(
        &jigc_doc(
            repo,
            home,
            &["set-slot", addr, "--from-file", "-"],
            Some(format!("{prose}\n").as_bytes()),
        ),
        &format!("set-slot {addr}"),
    );
}

/// Which arm is being driven — the set's **only** input difference.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Arm {
    /// The committed ADR's file-state baseline is present throughout.
    Present,
    /// The key is absent when the task first touches the doc — a fresh clone, an
    /// `unmanage`, a deleted cache.
    AbsentAtFirstTouch,
    /// The key is lost after the task's first touch — the same causes, later; or a task
    /// copied in by a binary that recorded nothing at the copy-in.
    LostAfterFirstTouch,
}

impl Arm {
    fn tag(self) -> &'static str {
        match self {
            Arm::Present => "armA",
            Arm::AbsentAtFirstTouch => "armB",
            Arm::LostAfterFirstTouch => "armC",
        }
    }
}

/// Drop the ADR's key through the engine API that owns the record, so the record's byte
/// form stays canonical and the drop is the shape a real forgetting writer produces.
fn drop_the_baseline(repo: &Path) {
    let jigc_root = repo.join(".jigc");
    let mut record =
        engine::file_state::FileStateRecord::load(&jigc_root).expect("load the record");
    assert!(
        record.forget(ADR_PATH),
        "the drop must remove a key that was really there",
    );
    record.save(&jigc_root).expect("persist the lost baseline");
}

/// Whether the ADR's key is in the persisted record.
fn baseline_recorded(repo: &Path) -> bool {
    engine::file_state::FileStateRecord::load(&repo.join(".jigc"))
        .expect("load the record")
        .get(ADR_PATH)
        .is_some()
}

/// What one arm observed. Everything asserted about the contrast is here, so the
/// two arms are compared as data rather than as two hand-written scripts.
struct Outcome {
    /// The process exit code of `jigc task finalize`.
    exit: i32,
    /// The finding codes in the finalize envelope, in emission order.
    codes: Vec<String>,
    /// Commit count immediately before finalize.
    commits_before: u32,
    /// Commit count immediately after finalize.
    commits_after: u32,
    /// The committed ADR's bytes after finalize.
    committed_adr: String,
    /// Whether the ADR key was in the persisted record when the task first touched the
    /// doc, and when finalize ran — the fixture's own witness that the arms differ where
    /// they claim to.
    baseline_at_first_touch: bool,
    baseline_at_finalize: bool,
}

/// Drive one arm end to end. **One branch on `arm`** — the baseline drop — so the
/// contrast the test asserts cannot come from anywhere else.
fn drive(arm: Arm) -> Outcome {
    let repo = TempDir::new(arm.tag());
    let home = TempDir::new("home");
    let repo = repo.path();
    let home = home.path();
    init_repo(repo);

    // ── Task 0: create + finalize the ADR. Its landed finalize posts the committed
    //    baseline (finalize phase 7) — the precondition arm B removes.
    assert_ok(
        &jigc(
            repo,
            home,
            &[
                "start",
                "--workflow",
                "single-task",
                "cache sessions in a single in-memory node",
            ],
        ),
        "`jigc start` (task 0)",
    );
    let task0 = "cache-sessions-in-a-single";
    assert_ok(
        &jigc_doc(
            repo,
            home,
            &["create", "adr", "--title", "Single-node cache"],
            None,
        ),
        "`jigc doc create adr` (task 0)",
    );
    set_slot(
        repo,
        home,
        "adr:single-node-cache#context",
        "Session lookups must stay sub-millisecond.",
    );
    set_slot(
        repo,
        home,
        "adr:single-node-cache#decision",
        "A single in-memory node keeps lookups fast.",
    );
    set_slot(
        repo,
        home,
        "adr:single-node-cache#consequences",
        "A cold node loses its sessions.",
    );
    fill_commit(repo, home, task0, "record the cache decision");
    assert_ok(
        &jigc(repo, home, &["task", "finalize", task0]),
        "`jigc task finalize` (task 0) — records the committed baseline FIRST",
    );
    assert!(
        repo.join(ADR_PATH).exists(),
        "task 0 must promote {ADR_PATH}"
    );

    // ── The warm task is minted over the committed ADR.
    assert_ok(
        &jigc(
            repo,
            home,
            &[
                "start",
                "--workflow",
                "single-task",
                "revise the cache decision",
            ],
        ),
        "`jigc start` (warm task)",
    );
    let warm = "revise-the-cache-decision";

    // ── The human channel: an out-of-band, conformant, prose-only edit to the
    //    committed ADR, written on disk AFTER the warm task is minted and left
    //    uncommitted. HEAD stays at the task's base (a `finalize.base-mismatch` would
    //    pre-empt the reconcile gate and the arms would prove nothing about
    //    reconciliation), and the edit is not the base pin's blob — a change made during
    //    the task, which M55 Increment 4's pulled-edit absorb leaves to this arm's
    //    conflict-block. It lands BEFORE the task's first touch, so the copy-in below
    //    carries it: arm B's merge is a commit carrying both sides' prose.
    let path = repo.join(ADR_PATH);
    let body = fs::read_to_string(&path).expect("read the committed ADR");
    let edited = body.replacen("A cold node loses its sessions.", HUMAN_PROSE, 1);
    assert_ne!(body, edited, "the OOB edit must change the committed ADR");
    fs::write(&path, edited).expect("apply the OOB edit");

    // ── The one branch, first half: arm B has no key when the task first touches the
    //    doc.
    if arm == Arm::AbsentAtFirstTouch {
        drop_the_baseline(repo);
    }
    let baseline_at_first_touch = baseline_recorded(repo);

    // ── The warm task touches the SAME doc through the CLI (copy-in for update), so
    //    both sides have moved: the committed file and this task's staged writes.
    set_slot(repo, home, "adr:single-node-cache#decision", TASK_PROSE);
    fill_commit(repo, home, warm, "revise the decision");

    // ── The one branch, second half: arm C loses the key after that touch.
    if arm == Arm::LostAfterFirstTouch {
        drop_the_baseline(repo);
    }
    let baseline_at_finalize = baseline_recorded(repo);

    let commits_before = commit_count(repo);
    let out = jigc(repo, home, &["task", "finalize", warm, "--format", "json"]);
    let exit = out.status.code().expect("finalize exits, never signalled");
    let stdout = String::from_utf8(out.stdout).expect("utf-8 stdout");
    let envelope: serde_json::Value = serde_json::from_str(&stdout).unwrap_or_else(|err| {
        panic!("{arm:?}: finalize must emit the JSON envelope ({err}); got:\n{stdout}")
    });
    let codes = envelope["findings"]
        .as_array()
        .unwrap_or_else(|| {
            panic!("{arm:?}: the envelope carries a `findings` array; got:\n{stdout}")
        })
        .iter()
        .map(|f| f["code"].as_str().unwrap_or_default().to_string())
        .collect();

    Outcome {
        exit,
        codes,
        commits_before,
        commits_after: commit_count(repo),
        committed_adr: fs::read_to_string(repo.join(ADR_PATH)).expect("read the committed ADR"),
        baseline_at_first_touch,
        baseline_at_finalize,
    }
}

/// Assert `arm` blocked: the conflict finding, the validation exit, no commit, the
/// human's bytes on disk and the task's nowhere near the file.
fn assert_conflict_blocked(name: &str, arm: &Outcome) {
    assert_eq!(
        arm.exit,
        i32::from(cli::task::EXIT_VALIDATION_BLOCKED),
        "{name} blocks at the validation exit code; findings were {:?}",
        arm.codes,
    );
    assert!(
        arm.codes
            .iter()
            .any(|c| c == "reconciliation.conflict-block"),
        "{name}'s block is `reconciliation.conflict-block`; got {:?}",
        arm.codes,
    );
    assert_eq!(
        arm.commits_before, arm.commits_after,
        "{name}: a conflict-block creates no commit",
    );
    assert!(
        arm.committed_adr.contains(HUMAN_PROSE),
        "{name} leaves the human's bytes on disk untouched; got:\n{}",
        arm.committed_adr,
    );
    assert!(
        !arm.committed_adr.contains(TASK_PROSE),
        "{name} promotes nothing — the task's staged write must not reach the file; got:\n{}",
        arm.committed_adr,
    );
}

/// **The contrast.** The same out-of-band edit over the same committed managed doc,
/// touched by the same task. With the baseline recorded the conflict is detected and
/// blocked. With no key at the task's first touch the copy-in adopts the human's bytes
/// and the two sides land merged — the declared cost of a missing baseline. With the key
/// lost after that touch the base pin blocks, where it used to merge silently.
#[test]
fn the_baseline_decides_between_a_block_and_the_declared_merge() {
    let arm_a = drive(Arm::Present);
    let arm_b = drive(Arm::AbsentAtFirstTouch);
    let arm_c = drive(Arm::LostAfterFirstTouch);

    // The fixture's own witness: the arms diverge exactly where they claim to.
    assert!(
        arm_a.baseline_at_first_touch && arm_a.baseline_at_finalize,
        "arm A holds the ADR's recorded baseline throughout",
    );
    assert!(
        !arm_b.baseline_at_first_touch,
        "arm B reaches the task's first touch WITHOUT the ADR's recorded baseline",
    );
    assert!(
        arm_b.baseline_at_finalize,
        "arm B's copy-in recorded the baseline — the bytes it copied, the human's edit \
         included",
    );
    assert!(
        arm_c.baseline_at_first_touch && !arm_c.baseline_at_finalize,
        "arm C loses the ADR's recorded baseline AFTER the task's first touch",
    );

    // ── Arm A — baseline present ⇒ detected, blocked, routed, nothing committed.
    assert_conflict_blocked("arm A", &arm_a);

    // ── Arm B — no key at the first touch ⇒ the copy-in carries the human's edit and
    //    adopts it as the baseline; nothing has moved since, so the finalize lands and
    //    the promoted doc carries BOTH sides' prose. A merge, by the declared order.
    assert_eq!(
        arm_b.exit,
        i32::from(cli::task::EXIT_SUCCESS),
        "arm B succeeds — nothing blocks; findings were {:?}",
        arm_b.codes,
    );
    assert!(
        !arm_b
            .codes
            .iter()
            .any(|c| c == "reconciliation.conflict-block"),
        "arm B never reaches the conflict arm; got {:?}",
        arm_b.codes,
    );
    assert_eq!(
        arm_b.commits_after,
        arm_b.commits_before + 1,
        "arm B lands a commit",
    );
    assert!(
        arm_b.committed_adr.contains(HUMAN_PROSE) && arm_b.committed_adr.contains(TASK_PROSE),
        "arm B merges both sides into the committed doc; got:\n{}",
        arm_b.committed_adr,
    );

    // ── Arm C — the key lost after the first touch ⇒ no record says what the task
    //    started from, so the base pin does: the on-disk bytes differ from its blob,
    //    and the door blocks rather than adopt them.
    assert_conflict_blocked("arm C", &arm_c);
}
