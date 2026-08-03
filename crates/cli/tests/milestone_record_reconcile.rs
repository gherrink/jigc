//! M39 Increment 4 / T6 — the **reconcile preflight** before every `set: on-transition`
//! record overwrite (`design/team-ready-state.md` → No-silent-overwrite discipline (F3);
//! `design/reconciliation.md`). The `milestone-record` is a **machine-owned** structural
//! record: because every leaf is CLI-owned, an out-of-band human edit to a machine-set
//! field cannot be *merged* — but it must not be silently *clobbered* either. So each
//! milestone-op that overwrites the committed record (`add-task` — append; `join`/finalize
//! — the status-flip) runs a reconcile preflight first and **conflict-blocks on drift**,
//! leaving the record untouched and routing the human to reconcile.
//!
//! Three proofs, driving the REAL binary against throwaway `[dev ▸ methodology]` git repos:
//!
//!   (RED-iv-a) **add-task site.** create → add-task, then hand-edit the committed record's
//!              header `status` OUT-OF-BAND (to a *valid* enum value, so it is genuine drift
//!              of a machine field, not a malformed-parse block) → the next `add-task`
//!              **blocks** (non-zero, a routed reconcile finding naming the record), and the
//!              record is **NOT overwritten** (the OOB edit survives, the new task never lands).
//!
//!   (RED-iv-b) **flip / finalize site.** Same OOB drift, then `finalize` → **blocks** the
//!              same way, the record NOT flipped/overwritten (both overwrite sites guarded
//!              uniformly).
//!
//!   (GREEN)    **Clean stays green.** With no OOB edit, create → add-task → add-task
//!              succeeds — the additive guard is a no-op on a clean record and does not break
//!              the T3/T4 gate.

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

/// A throwaway directory that removes itself on drop.
struct TempDir(PathBuf);

impl TempDir {
    fn new(tag: &str) -> Self {
        let mut path = std::env::temp_dir();
        path.push(format!(
            "jigc-milestone-record-reconcile-{tag}-{}-{:?}",
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
    String::from_utf8(out.stdout).expect("utf-8 git stdout")
}

/// Initialize a real git repo with one commit (mint reads HEAD via `git rev-parse`).
fn init_repo(repo: &Path) {
    git(repo, &["init", "-q"]);
    git(repo, &["config", "user.email", "test@example.com"]);
    git(repo, &["config", "user.name", "Test"]);
    fs::write(repo.join("README.md"), "hello\n").expect("write file");
    git(repo, &["add", "README.md"]);
    git(repo, &["commit", "-q", "-m", "initial"]);
}

/// Write the `[dev ▸ methodology]` compose marker — the exact key `make_pack` reads to
/// assemble the composition dev-highest (so the dev `docs-root` knob applies → `docs/`).
fn write_compose_marker(repo: &Path) {
    fs::create_dir_all(repo.join(".jigc").join("config")).expect("mk project config");
    fs::write(
        repo.join(".jigc").join("config").join("packs.yaml"),
        "compose-embedded-methodology: true\n",
    )
    .expect("write compose marker");
}

/// Run `jigc milestone <args>` with `cwd = repo` and `$HOME = home`, never inheriting a
/// harness `JIGC_PACK_DIR` (the compose-marker path requires it ABSENT).
fn run_milestone(repo: &Path, home: &Path, args: &[&str]) -> std::process::Output {
    Command::new(env!("CARGO_BIN_EXE_jigc"))
        .arg("milestone")
        .args(args)
        .current_dir(repo)
        .env("HOME", home)
        .env_remove("JIGC_PACK_DIR")
        .output()
        .expect("run the jigc binary")
}

/// Assert a `jigc milestone` invocation exited 0, surfacing stderr on failure.
fn assert_ok(out: &std::process::Output, what: &str) {
    assert!(
        out.status.success(),
        "{what} must exit 0; got {:?}\nstderr:\n{}",
        out.status,
        String::from_utf8_lossy(&out.stderr),
    );
}

/// The committed record path under docs-root for milestone `cache-rework`.
fn record_path(repo: &Path) -> PathBuf {
    repo.join("docs")
        .join("milestone-records")
        .join("cache-rework.md")
}

/// Hand-edit the committed record's header `status: active` → `status: joined` out of
/// band (a *valid* enum value — so the write is genuine drift of a machine-set field the
/// preflight must catch on its own, NOT a malformed-parse conformance block). Returns the
/// edited bytes for the not-overwritten assertion.
fn oob_edit_status(repo: &Path) -> String {
    let path = record_path(repo);
    let before = fs::read_to_string(&path).expect("read record before OOB edit");
    assert!(
        before.contains("status: active"),
        "the record must carry `status: active` before the OOB edit; got:\n{before}",
    );
    let after = before.replacen("status: active", "status: joined", 1);
    assert_ne!(after, before, "the OOB edit must change the record bytes");
    fs::write(&path, &after).expect("write OOB-edited record");
    after
}

/// The first `<…>`-shaped span left unsubstituted in `text` — an all-lowercase
/// `<word-with-dashes>` token, the shape a route placeholder takes. `None` when the text
/// carries none.
fn unsubstituted_placeholder(text: &str) -> Option<String> {
    let mut rest = text;
    while let Some(open) = rest.find('<') {
        rest = &rest[open + 1..];
        if let Some(close) = rest.find('>') {
            let span = &rest[..close];
            if !span.is_empty()
                && span
                    .chars()
                    .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '-')
            {
                return Some(format!("<{span}>"));
            }
        }
    }
    None
}

/// The **record-seam conflict contract** (M47 inc-2 / T4). The conflict-block route
/// belongs to the caller, and at this door the caller is a milestone-record op with **no
/// task at all** — so the emitted block names the *record*, and carries neither the
/// inapplicable `jigc task discard` (there is no task to discard) nor an unsubstituted
/// `<…>` placeholder. Both are what the M43 route floor exists to prevent on a
/// **blocking** finding (`design/surface-contract.md` → The route fence).
fn assert_record_conflict_contract(stderr: &str, door: &str) {
    assert!(
        stderr.contains("reconciliation.conflict-block")
            && stderr.contains("docs/milestone-records/cache-rework.md"),
        "door `{door}`: the block must be a routed `reconciliation.conflict-block` naming \
         the record; got stderr:\n{stderr}",
    );
    assert!(
        stderr.contains("milestone record"),
        "door `{door}`: the message names the record, not a task; got stderr:\n{stderr}",
    );
    assert!(
        !stderr.contains("this task's staged writes"),
        "door `{door}`: no task staged anything here — the message must not claim one did; \
         got stderr:\n{stderr}",
    );
    assert!(
        !stderr.contains("jigc task discard"),
        "door `{door}`: `jigc task discard` is inapplicable at a record-only door (no task \
         exists); got stderr:\n{stderr}",
    );
    assert_eq!(
        unsubstituted_placeholder(stderr),
        None,
        "door `{door}`: a blocking finding's route carries no unsubstituted placeholder; \
         got stderr:\n{stderr}",
    );
    assert!(
        stderr.contains("route:"),
        "door `{door}`: a blocking finding always routes; got stderr:\n{stderr}",
    );
}

/// create → add-task, establishing the committed record + its file-state baseline.
fn create_and_seed(repo: &Path, home: &Path) {
    init_repo(repo);
    write_compose_marker(repo);
    assert_ok(
        &run_milestone(repo, home, &["create", "Cache rework"]),
        "`[dev ▸ methodology]` `jigc milestone create`",
    );
    assert_ok(
        &run_milestone(
            repo,
            home,
            &["add-task", "cache-rework", "Warm the read cache"],
        ),
        "add-task #1 (seeds the baseline)",
    );
}

/// (RED-iv-a) An OOB edit to the committed record conflict-blocks the next `add-task`,
/// leaving the record untouched.
#[test]
fn oob_edit_conflict_blocks_the_next_add_task() {
    let repo = TempDir::new("add-task");
    let home = TempDir::new("home");
    create_and_seed(repo.path(), home.path());

    let edited = oob_edit_status(repo.path());
    let commits_before = git(repo.path(), &["rev-list", "--count", "HEAD"]);

    let out = run_milestone(
        repo.path(),
        home.path(),
        &["add-task", "cache-rework", "Evict cold entries"],
    );
    assert!(
        !out.status.success(),
        "add-task over an OOB-drifted record must block (non-zero); got success\nstdout:\n{}",
        String::from_utf8_lossy(&out.stdout),
    );
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert_record_conflict_contract(&stderr, "add-task");

    // The record was NOT overwritten: the OOB edit survives and the second task never landed.
    let now = fs::read_to_string(record_path(repo.path())).expect("record after blocked add-task");
    assert_eq!(
        now, edited,
        "a blocked add-task must leave the OOB-edited record byte-identical (not overwritten)",
    );
    assert!(
        !now.contains("Evict cold entries") && !now.contains("evict-cold-entries"),
        "the blocked second sub-task must not appear in the record:\n{now}",
    );
    // No record-only commit was made by the blocked op.
    assert_eq!(
        git(repo.path(), &["rev-list", "--count", "HEAD"]),
        commits_before,
        "a blocked add-task makes no commit",
    );
}

/// (RED-iv-b) The same OOB drift conflict-blocks `finalize` (the status-flip overwrite
/// site), leaving the record un-flipped.
#[test]
fn oob_edit_conflict_blocks_finalize_flip() {
    let repo = TempDir::new("finalize");
    let home = TempDir::new("home");
    create_and_seed(repo.path(), home.path());

    let edited = oob_edit_status(repo.path());

    let out = run_milestone(repo.path(), home.path(), &["finalize", "cache-rework"]);
    assert!(
        !out.status.success(),
        "finalize over an OOB-drifted record must block (non-zero); got success\nstdout:\n{}",
        String::from_utf8_lossy(&out.stdout),
    );
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert_record_conflict_contract(&stderr, "finalize");

    // The record was NOT flipped/overwritten: the OOB edit survives byte-identical.
    let now = fs::read_to_string(record_path(repo.path())).expect("record after blocked finalize");
    assert_eq!(
        now, edited,
        "a blocked finalize must leave the OOB-edited record byte-identical (not flipped)",
    );
}

/// (GREEN) A clean record is a no-op for the guard — sequential `add-task`s still succeed
/// (the additive guard does not false-block the T3 gate).
#[test]
fn clean_record_add_task_stays_green() {
    let repo = TempDir::new("clean");
    let home = TempDir::new("home");
    create_and_seed(repo.path(), home.path());

    // No OOB edit — the second append must still succeed and land its record commit.
    let before = git(repo.path(), &["rev-list", "--count", "HEAD"]);
    assert_ok(
        &run_milestone(
            repo.path(),
            home.path(),
            &["add-task", "cache-rework", "Evict cold entries"],
        ),
        "a clean sequential add-task must succeed (guard inert on a clean record)",
    );
    let now = fs::read_to_string(record_path(repo.path())).expect("record after clean add-task");
    assert!(
        now.contains("evict-cold-entries"),
        "the clean second sub-task must land in the record:\n{now}",
    );
    let after: u32 = git(repo.path(), &["rev-list", "--count", "HEAD"])
        .trim()
        .parse()
        .unwrap();
    assert_eq!(
        after,
        before.trim().parse::<u32>().unwrap() + 1,
        "the clean add-task lands exactly one record commit",
    );
}
