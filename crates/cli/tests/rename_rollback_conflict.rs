//! M52 Increment 5 / T6 — **`rollback_rename`'s two unguarded arms, and
//! `rename.rollback-conflict`** (`completions/artifacts/M52/settle-record.md` → D1.2 as
//! amended by §1 and §14;
//! [baseline-rollback.md](../../../completions/artifacts/M52/baseline-rollback.md) §1
//! population 6, §2.3b; `design/validation.md` → The M52 registrations — Increment 5).
//!
//! # What was broken
//!
//! `rollback_rename` has three arms and exactly one of them was guarded. The
//! `tracked_restore` loop restores the old doc + every referrer **from HEAD**, and the door
//! refuses to run at all over a dirty tree — so HEAD is what the worktree held and there are
//! no third-party bytes for it to take (`ROLLBACK_POPULATIONS`' `rename-head-restore` row,
//! `DoorGuard("rename.dirty-tree")`). The other two took bytes unconditionally:
//!
//!   * the **landing path** — `remove_file` on the path the move landed at, whenever it was
//!     not at HEAD. The user's `pre-commit` hook runs *inside* the interval between that
//!     write and this removal, so a hook that writes there had its bytes deleted at exit 1,
//!     named by nothing (baseline §2.3b drove it: `[ -f docs/research/axis-four-research.md ]`
//!     → NO, `rollback-conflict` in output: 0).
//!   * the **gitignored `.jigc/state/file-state.json`** — a write-or-remove from a captured
//!     pre-image, under no condition at all. A third party's edit inside the same interval
//!     went the same way, and the *absent* cell was worse than an overwrite: a `file-state.json`
//!     this transaction never reached (a failure before the move) was removed anyway.
//!
//! # The two cells, and why the control is half the test
//!
//! 1. [`a_raced_landing_path_survives_and_is_named`] / [`a_raced_file_state_record_survives_and_names_both_copies`]
//!    — a `pre-commit` hook that writes at the arm's own path and exits 1. The racer's bytes
//!    are on disk when the run exits, the door's own blocking `rename.rollback-conflict`
//!    names the live path, and — where there was a pre-image to keep — jigc's copy is parked
//!    under `.jigc/displaced/rename/` rather than discarded.
//! 2. [`an_unraced_rejected_rename_reports_no_conflict_and_restores_byte_identically`] — the
//!    same door under the plain rejecting hook, which speaks and touches nothing. A
//!    compare-and-swap that fired here would be a law-1 lie on every ordinary rejected run,
//!    and the restore must still be byte-exact: the old doc, the file-state record, and the
//!    landing path's *absence*.
//!
//! [`the_machine_arm_carries_the_conflict_in_the_reject_document`] adds the driver's half:
//! under `--format json` the conflict rides **inside** the reject document, so the stream a
//! driver reads still parses as exactly one JSON value.

use std::fs;
use std::path::{Path, PathBuf};

use cli::rollback::{Discipline, RENAME_DOOR, ROLLBACK_POPULATIONS};

use crate::support::committing_doors::{
    DoorCase, HOOK_MARKER, drive, install_rejecting_hook, jigc, log_records, record_for,
    remove_hook,
};

/// The bytes the racing hook writes — the third party's, which must be on disk when the run
/// exits however the rollback goes.
const RACE_LINE: &str = "<!-- raced by a concurrent editor -->";

/// The gitignored cache the rename re-keys — `rename-worktree`'s second arm, and the one the
/// HEAD-sourced arm cannot cover because git does not track it.
const FILE_STATE: &str = ".jigc/state/file-state.json";

/// The `rename-worktree` population's row, as the registry states it.
fn population() -> &'static cli::rollback::Population {
    ROLLBACK_POPULATIONS
        .iter()
        .find(|p| p.id == "rename-worktree")
        .expect("`ROLLBACK_POPULATIONS` carries the `rename-worktree` population")
}

/// Replace the fixture's rejecting hook with one that **writes at `target` first** — the racer
/// running exactly where it can run: inside the transaction, between jigc's write and the
/// rollback.
fn install_racing_hook(repo: &Path, target: &str) {
    let hook = repo.join(".git").join("hooks").join("pre-commit");
    fs::create_dir_all(hook.parent().expect("hooks dir")).expect("mk hooks dir");
    let path = repo.join(target);
    fs::write(
        &hook,
        format!(
            "#!/bin/sh\nmkdir -p {parent:?}\necho '{RACE_LINE}' >> {path:?}\n\
             echo '{HOOK_MARKER}' 1>&2\nexit 1\n",
            parent = path.parent().expect("the raced path has a parent"),
        ),
    )
    .expect("write the racing pre-commit hook");
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        fs::set_permissions(&hook, fs::Permissions::from_mode(0o755)).expect("chmod hook");
    }
}

/// Every regular file under `.jigc/displaced/<noun>/`, recursively — the parked pre-images
/// this door left behind.
fn parked(repo: &Path, noun: &str) -> Vec<PathBuf> {
    fn walk(dir: &Path, out: &mut Vec<PathBuf>) {
        let Ok(entries) = fs::read_dir(dir) else {
            return;
        };
        let mut paths: Vec<PathBuf> = entries.flatten().map(|e| e.path()).collect();
        paths.sort();
        for path in paths {
            if path.is_dir() {
                walk(&path, out);
            } else {
                out.push(path);
            }
        }
    }
    let mut out = Vec::new();
    walk(&repo.join(".jigc").join("displaced").join(noun), &mut out);
    out
}

/// The bytes at `rel`, `None` when it is absent — so *absent* is compared as absent rather
/// than as the empty string.
fn bytes_at(repo: &Path, rel: &str) -> Option<Vec<u8>> {
    fs::read(repo.join(rel)).ok()
}

/// Land one successful rename in the fixture, so `.jigc/state/file-state.json` **exists**
/// before the raced run — its pre-image is then a file to park rather than an absence, which
/// is the cell that proves nothing is discarded.
///
/// Driven through the door itself rather than hand-written: the record is jigc's own cache,
/// and a fixture that forges it would not be capturing what the binary writes.
fn land_a_rename(case: &DoorCase, from: &str, to: &str) {
    remove_hook(case.repo.path());
    let out = jigc(
        case.repo.path(),
        case.home.path(),
        &["rename", from, "--to", to],
        None,
    );
    assert!(
        out.status.success(),
        "the fixture's first rename must land; stderr:\n{}",
        String::from_utf8_lossy(&out.stderr),
    );
    assert!(
        case.repo.path().join(FILE_STATE).is_file(),
        "a landed rename must leave the file-state record on disk — the raced cell needs a \
         pre-image to park",
    );
}

/// The population this suite drives declares the discipline it drives — a row demoted to
/// another discipline owes a different suite, not a silent pass.
#[test]
fn the_population_declares_the_discipline_this_suite_drives() {
    let row = population();
    assert_eq!(
        row.discipline,
        Discipline::FileCas,
        "the `rename-worktree` population declares `FileCas`",
    );
    let doors: Vec<Vec<&str>> = row.doors.iter().map(|d| d.to_vec()).collect();
    assert_eq!(
        doors,
        vec![vec!["rename"]],
        "the population is reached by `jigc rename` and nothing else",
    );
    assert_eq!(
        RENAME_DOOR.code, "rename.rollback-conflict",
        "the door's conflict identity is the one registered in `design/validation.md`",
    );
}

/// Arm 1 — the landing path. A hook that writes there and refuses leaves those bytes
/// standing, and the door says so instead of removing them.
#[test]
fn a_raced_landing_path_survives_and_is_named() {
    let case: DoorCase = drive("jigc rename");
    let repo = case.repo.path();
    let landing = "docs/decisions/beta-decision.md";
    install_racing_hook(repo, landing);

    let driven: Vec<&str> = case.driven.iter().map(String::as_str).collect();
    let rejected = jigc(repo, case.home.path(), &driven, None);
    let stderr = String::from_utf8_lossy(&rejected.stderr).into_owned();
    assert!(
        !rejected.status.success(),
        "a rejected pre-commit hook must fail the rename; stderr:\n{stderr}",
    );

    // (1) **The third party's bytes are on disk.** This is the whole claim: the arm that
    // used to `remove_file` this path took them silently.
    let after = bytes_at(repo, landing).unwrap_or_else(|| {
        panic!(
            "the raced landing path `{landing}` must survive the rollback — the removal arm \
             may not delete a file a third party wrote; stderr:\n{stderr}",
        )
    });
    assert!(
        String::from_utf8_lossy(&after).contains(RACE_LINE),
        "the racer's own line must still be in `{landing}` after the rollback; it now reads:\n{}",
        String::from_utf8_lossy(&after),
    );

    // (2) the door's own identity, rendered blocking through the house finding line, naming
    // the live path — `(code, target)` is the key a driver branches on.
    assert!(
        stderr.contains(&format!("blocking · {}", RENAME_DOOR.code)),
        "a raced restore must raise this door's own blocking `{}`; stderr:\n{stderr}",
        RENAME_DOOR.code,
    );
    assert!(
        stderr.contains(landing),
        "the conflict must name the live path `{landing}`; stderr:\n{stderr}",
    );

    // (3) the hook's own bytes are still verbatim beside it — the conflict ADDS to the door's
    // frame and replaces nothing.
    assert!(
        stderr.contains(HOOK_MARKER),
        "the hook's own bytes must survive verbatim beside the conflict; stderr:\n{stderr}",
    );

    // (4) the landing path did not exist before this rename, so there is no pre-image to
    // park — and the route says so rather than naming a copy that is not there.
    assert!(
        parked(repo, RENAME_DOOR.noun).is_empty(),
        "`{landing}` did not exist before the run, so nothing is parked",
    );
    assert!(
        stderr.contains("did not exist before this"),
        "the absent-pre-image route must say the file did not exist before the run and that \
         the rollback did not delete it; stderr:\n{stderr}",
    );

    // (5) the old doc is still restored byte-identically — the guarded arm is untouched by
    // this change, and the conflict is additive.
    assert!(
        repo.join("docs/decisions/alpha-decision.md").is_file(),
        "the HEAD-sourced arm still restores the old doc; stderr:\n{stderr}",
    );

    // (6) the code reaches the invocation log, read off the record.
    let records = log_records(repo);
    let record = record_for(&records, &case.driven)
        .unwrap_or_else(|| panic!("the refused run must be logged; records:\n{records:#?}"));
    let codes: Vec<&str> = record["finding_codes"]
        .as_array()
        .map(|a| a.iter().filter_map(|v| v.as_str()).collect())
        .unwrap_or_default();
    assert!(
        codes.contains(&RENAME_DOOR.code),
        "the log must carry `{}` in `finding_codes`; got {record}",
        RENAME_DOOR.code,
    );
}

/// Arm 2 — the gitignored file-state record. It is the cell with a **present** pre-image, so
/// it is the one that proves nothing is discarded: both copies survive and both are named.
#[test]
fn a_raced_file_state_record_survives_and_names_both_copies() {
    let case: DoorCase = drive("jigc rename");
    let repo = case.repo.path();
    land_a_rename(&case, "adr:alpha-decision", "Gamma decision");

    let before = bytes_at(repo, FILE_STATE).expect("the landed rename wrote the file-state record");
    install_racing_hook(repo, FILE_STATE);

    let rejected = jigc(
        repo,
        case.home.path(),
        &["rename", "adr:gamma-decision", "--to", "Beta decision"],
        None,
    );
    let stderr = String::from_utf8_lossy(&rejected.stderr).into_owned();
    assert!(
        !rejected.status.success(),
        "a rejected pre-commit hook must fail the rename; stderr:\n{stderr}",
    );

    // (1) the racer's bytes stand.
    let after = bytes_at(repo, FILE_STATE).unwrap_or_else(|| {
        panic!("the raced `{FILE_STATE}` must survive the rollback; stderr:\n{stderr}")
    });
    assert!(
        String::from_utf8_lossy(&after).contains(RACE_LINE),
        "the racer's own line must still be in `{FILE_STATE}`; it now reads:\n{}",
        String::from_utf8_lossy(&after),
    );

    // (2) the conflict names the live path.
    assert!(
        stderr.contains(&format!("blocking · {}", RENAME_DOOR.code)),
        "a raced restore must raise `{}`; stderr:\n{stderr}",
        RENAME_DOOR.code,
    );
    assert!(
        stderr.contains(FILE_STATE),
        "the conflict must name the live path `{FILE_STATE}`; stderr:\n{stderr}",
    );

    // (3) **both copies**: jigc's pre-image is parked under this door's noun, holding exactly
    // the bytes the capture took, not discarded.
    let parked_copies = parked(repo, RENAME_DOOR.noun);
    assert_eq!(
        parked_copies.len(),
        1,
        "exactly one pre-image must be parked under `.jigc/displaced/{}/`; found \
         {parked_copies:?}",
        RENAME_DOOR.noun,
    );
    assert_eq!(
        fs::read(&parked_copies[0]).expect("read the parked pre-image"),
        before,
        "the parked copy must hold the record's pre-write bytes",
    );
    let parked_label = parked_copies[0]
        .strip_prefix(repo)
        .expect("the park lives in the repo")
        .to_string_lossy()
        .replace('\\', "/");
    assert!(
        stderr.contains(&parked_label),
        "the route must name the parked copy at `{parked_label}`; stderr:\n{stderr}",
    );
}

/// The zero-false-fire control: the same door, a hook that refuses and touches nothing. No
/// conflict, and the restore is byte-exact on all three arms — the old doc, the file-state
/// record, and the landing path's *absence*.
#[test]
fn an_unraced_rejected_rename_reports_no_conflict_and_restores_byte_identically() {
    let case: DoorCase = drive("jigc rename");
    let repo = case.repo.path();
    land_a_rename(&case, "adr:alpha-decision", "Gamma decision");
    install_rejecting_hook(repo);

    let old_doc = "docs/decisions/gamma-decision.md";
    let landing = "docs/decisions/beta-decision.md";
    let before_doc = bytes_at(repo, old_doc).expect("the renamed doc is on disk");
    let before_state = bytes_at(repo, FILE_STATE).expect("the landed rename wrote the record");

    let rejected = jigc(
        repo,
        case.home.path(),
        &["rename", "adr:gamma-decision", "--to", "Beta decision"],
        None,
    );
    let stderr = String::from_utf8_lossy(&rejected.stderr).into_owned();
    assert!(
        !rejected.status.success(),
        "the control cell must still fail — it is the same rejection; stderr:\n{stderr}",
    );
    assert!(
        !stderr.contains("rollback-conflict"),
        "nothing raced, so no conflict may be reported; stderr:\n{stderr}",
    );
    assert_eq!(
        bytes_at(repo, old_doc).as_deref(),
        Some(before_doc.as_slice()),
        "the old doc must be restored byte-identically; stderr:\n{stderr}",
    );
    assert_eq!(
        bytes_at(repo, FILE_STATE).as_deref(),
        Some(before_state.as_slice()),
        "the file-state record must be restored byte-identically; stderr:\n{stderr}",
    );
    assert_eq!(
        bytes_at(repo, landing),
        None,
        "the landing path must be gone — jigc's own copy is removed while it still holds \
         jigc's bytes; stderr:\n{stderr}",
    );
    assert!(
        parked(repo, RENAME_DOOR.noun).is_empty(),
        "nothing raced, so nothing is parked",
    );

    let records = log_records(repo);
    let record = record_for(
        &records,
        &["rename", "adr:gamma-decision", "--to", "Beta decision"]
            .iter()
            .map(|s| (*s).to_string())
            .collect::<Vec<String>>(),
    )
    .unwrap_or_else(|| panic!("the refused run must be logged; records:\n{records:#?}"));
    assert_eq!(
        record["finding_codes"].as_array().map(Vec::len),
        Some(0),
        "an unraced rejection is an operational error and carries no finding; got {record}",
    );
}

/// The driver's half: the conflict rides **inside** the reject document rather than being
/// printed beside it, so the stream a driver reads still parses as exactly one JSON value.
#[test]
fn the_machine_arm_carries_the_conflict_in_the_reject_document() {
    let case: DoorCase = drive("jigc rename");
    let repo = case.repo.path();
    install_racing_hook(repo, "docs/decisions/beta-decision.md");

    let mut driven: Vec<&str> = case.driven.iter().map(String::as_str).collect();
    driven.extend_from_slice(&["--format", "json"]);
    let rejected = jigc(repo, case.home.path(), &driven, None);
    let stderr = String::from_utf8_lossy(&rejected.stderr).into_owned();

    let document: serde_json::Value = serde_json::from_str(stderr.trim()).unwrap_or_else(|err| {
        panic!("the reject stream must be exactly one JSON document ({err}); stderr:\n{stderr}")
    });
    let codes: Vec<&str> = document["findings"]
        .as_array()
        .map(|a| a.iter().filter_map(|f| f["code"].as_str()).collect())
        .unwrap_or_default();
    assert!(
        codes.contains(&RENAME_DOOR.code),
        "the reject document must carry `{}` beside the refusal; document:\n{document:#}",
        RENAME_DOOR.code,
    );
}
