//! M52 Increment 5 / T5 — **the fan-out record flip, both `squash` arms**
//! (`completions/artifacts/M52/settle-record.md` → D1.2 (`RecordFlipGuard::Drop`) as amended
//! by §1; [baseline-rollback.md](../../../completions/artifacts/M52/baseline-rollback.md) §1
//! table A row 5; `design/finalize.md` → Rollback discipline, the fan-out record row).
//!
//! # What was broken
//!
//! `jigc milestone finalize` flips the committed `milestone-record` to `joined` **before** the
//! boundary commits, and folds that write into the commit. A boundary that does not land must
//! put the record back — and the guard that does it was a destructor holding an unconditional
//! `fs::write` of its captured pre-image. The interval it writes across is the widest one in
//! the binary: promotion, retirement, the live-index stage, the combine, and the user's own
//! `pre-commit` hook all run inside it. So a third party who wrote at the record path in that
//! interval had their bytes rewritten at exit 1 with nothing on any surface saying so — the
//! same loss the `milestone-record` population had at its own five doors (T4), through the
//! sixth write of the same file.
//!
//! `cli::rollback::ROLLBACK_POPULATIONS`' `fan-out-record-flip` row declares `FileCas`. This
//! suite is what binds it: the restore is the shared compare-and-swap, so the racer's bytes
//! stand, the pre-image is parked under `.jigc/displaced/milestone-op/`, and the door's own
//! `milestone.rollback-conflict` names both copies beside the refusal it did not replace.
//!
//! # The axis, and where it comes from
//!
//! One door — `jigc milestone finalize`, read from the registry row rather than written down
//! here ([`the_cells_are_the_fan_out_flip_population`]) — crossed with the **`squash` knob's
//! two values**, because the boundary has two commit channels (`chain_commit` and
//! `combine_commit`) and a flip proven restored on one proves nothing about the other: the
//! guard's conflicts have to reach the refusal each arm composes, and the two arms compose
//! their own. That crossing is the axis M47's index-half sibling iterates for the same reason
//! (`milestone_record_ff_rollback.rs`).
//!
//! # Three cells, and why two of them are controls
//!
//! 1. [`a_raced_record_flip_preserves_the_racers_bytes_and_names_both_copies`] — a `pre-commit`
//!    hook that **edits the record and exits 1**.
//! 2. [`no_flip_reports_a_conflict_when_nothing_raced`] — the same two arms under a hook that
//!    refuses and touches nothing. A swap that fired here would be a law-1 lie on every
//!    ordinary rejected fan-out, and the restore must still be byte-exact.
//! 3. [`a_landed_boundary_disarms_and_reports_nothing`] — no hook at all. The boundary lands,
//!    the guard disarms, the committed record stays `joined`, and nothing is parked or
//!    reported: a compare-and-swap in a destructor must not run on the success path.

use std::fs;
use std::path::{Path, PathBuf};

use cli::rollback::{Discipline, MILESTONE_DOOR, ROLLBACK_POPULATIONS};

use crate::support::committing_doors::{
    DoorCase, HOOK_MARKER, drive, jigc, log_records, record_for, remove_hook,
};

/// The line the racing hook writes into the record — the third party's bytes, which must be
/// on disk when the run exits however the rollback goes.
const RACE_LINE: &str = "<!-- raced by a concurrent editor -->";

/// The committed record the `milestone finalize` fixture flips, repo-relative.
const RECORD: &str = "docs/milestone-records/cache-rework.md";

/// One driven cell: the `squash` value, and the verb key `support::committing_doors::drive`
/// builds that arm's fan-out fixture under.
struct Cell {
    /// The knob value this arm runs under — the axis the door set does not carry.
    squash: bool,
    /// The verb key `drive` builds the fixture under.
    verb: &'static str,
}

/// The `squash` knob's two values — the boundary's two commit channels.
const CELLS: &[Cell] = &[
    Cell {
        squash: true,
        verb: "jigc milestone finalize (squash: true)",
    },
    Cell {
        squash: false,
        verb: "jigc milestone finalize (squash: false)",
    },
];

/// The `fan-out-record-flip` population's row, as the registry states it.
fn population() -> &'static cli::rollback::Population {
    ROLLBACK_POPULATIONS
        .iter()
        .find(|p| p.id == "fan-out-record-flip")
        .expect("`ROLLBACK_POPULATIONS` carries the `fan-out-record-flip` population")
}

/// Replace the fixture's rejecting hook with one that **edits the record first** — the racer
/// running exactly where it can run: inside the transaction, between the flip and the
/// rollback. The path is absolute because the boundary's commit runs in a **dedicated
/// worktree**, so the hook's own `cwd` is not the checkout the record lives in.
fn install_record_racing_hook(repo: &Path) {
    let hook = repo.join(".git").join("hooks").join("pre-commit");
    fs::create_dir_all(hook.parent().expect("hooks dir")).expect("mk hooks dir");
    let target = repo.join(RECORD);
    fs::write(
        &hook,
        format!(
            "#!/bin/sh\necho '{RACE_LINE}' >> {target:?}\necho '{HOOK_MARKER}' 1>&2\nexit 1\n",
        ),
    )
    .expect("write the record-racing pre-commit hook");
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

/// The record's bytes.
fn record_bytes(repo: &Path) -> Vec<u8> {
    fs::read(repo.join(RECORD)).expect("the committed milestone record exists")
}

/// The cell table is the registry row's door set crossed with the knob, in both directions.
#[test]
fn the_cells_are_the_fan_out_flip_population() {
    let row = population();
    assert_eq!(
        row.discipline,
        Discipline::FileCas,
        "the `fan-out-record-flip` population declares `FileCas`, which is what this suite \
         drives — a row demoted to another discipline owes a different suite, not a silent \
         pass",
    );

    let declared: Vec<Vec<&str>> = row.doors.iter().map(|d| d.to_vec()).collect();
    assert_eq!(
        declared,
        vec![vec!["milestone", "finalize"]],
        "the flip is reached by exactly one door; a door added to the population owes a \
         driven cell here",
    );
    // The knob is the axis the door set cannot carry: two commit channels, two arms.
    let squashes: Vec<bool> = CELLS.iter().map(|c| c.squash).collect();
    assert_eq!(
        squashes,
        vec![true, false],
        "both `finalize.fan-out.squash` values owe a cell — the boundary has two commit \
         channels and the guard's conflicts have to reach the refusal each one composes",
    );
}

/// The conflict cell: a hook that edits the record and refuses. The racer's bytes survive on
/// both arms, and both arms name the pair.
#[test]
fn a_raced_record_flip_preserves_the_racers_bytes_and_names_both_copies() {
    for cell in CELLS {
        let verb = cell.verb;
        let case: DoorCase = drive(verb);
        let repo = case.repo.path();
        let home = case.home.path();
        let driven: Vec<&str> = case.driven.iter().map(String::as_str).collect();

        // What the record held before the boundary flipped it — the bytes the guard captured.
        let before = record_bytes(repo);
        assert!(
            String::from_utf8_lossy(&before).contains("active"),
            "[{verb}] the fixture's record must be `active` before the finalize",
        );
        install_record_racing_hook(repo);

        let rejected = jigc(repo, home, &driven, None);
        let stdout = String::from_utf8_lossy(&rejected.stdout).into_owned();
        let stderr = String::from_utf8_lossy(&rejected.stderr).into_owned();

        assert!(
            !rejected.status.success(),
            "[{verb}] a rejected boundary must exit non-zero; stdout:\n{stdout}\nstderr:\n{stderr}",
        );

        // (1) **The third party's bytes are on disk.** This is the whole claim: the guard's
        // unconditional rewrite is what used to take them.
        let after = record_bytes(repo);
        assert!(
            String::from_utf8_lossy(&after).contains(RACE_LINE),
            "[{verb}] the racer's own line must still be in `{RECORD}` after the rollback; \
             the file now reads:\n{}",
            String::from_utf8_lossy(&after),
        );

        // (2) the door's own identity, rendered blocking through the house finding line, and
        // naming the live path — the key a driver branches on is `(code, target)`.
        let code = MILESTONE_DOOR.code;
        assert!(
            stderr.contains(&format!("blocking · {code}")),
            "[{verb}] a raced flip restore must raise `{code}`; stderr:\n{stderr}",
        );
        assert!(
            stderr.contains(RECORD),
            "[{verb}] the conflict must name the live path `{RECORD}`; stderr:\n{stderr}",
        );

        // (3) the hook's own bytes are still verbatim beside it — the conflict ADDS to the
        // boundary's frame and replaces nothing.
        assert!(
            stderr.contains(HOOK_MARKER),
            "[{verb}] the hook's own bytes must survive verbatim beside the conflict; \
             stderr:\n{stderr}",
        );

        // (4) the pre-flip bytes are preserved, not discarded — parked under the door's noun.
        let parked_copies = parked(repo, MILESTONE_DOOR.noun);
        assert_eq!(
            parked_copies.len(),
            1,
            "[{verb}] exactly one pre-image must be parked under `.jigc/displaced/{}/`; \
             found {parked_copies:?}",
            MILESTONE_DOOR.noun,
        );
        assert_eq!(
            fs::read(&parked_copies[0]).expect("read the parked pre-image"),
            before,
            "[{verb}] the parked copy must hold the record's pre-flip bytes",
        );

        // (5) the code reaches the invocation log — read off the record, so a release build's
        // compiled-out `debug_assert!` cannot hide it.
        let records = log_records(repo);
        let record = record_for(&records, &case.driven).unwrap_or_else(|| {
            panic!("[{verb}] the refused run must be logged; records:\n{records:#?}")
        });
        let codes: Vec<&str> = record["finding_codes"]
            .as_array()
            .map(|a| a.iter().filter_map(|v| v.as_str()).collect())
            .unwrap_or_default();
        assert!(
            codes.contains(&code),
            "[{verb}] the log must carry `{code}` in `finding_codes`; got {record}",
        );
    }
}

/// The zero-false-fire control: the same two arms, a hook that refuses and touches nothing.
/// No conflict, and the flip is restored byte-exact.
#[test]
fn no_flip_reports_a_conflict_when_nothing_raced() {
    for cell in CELLS {
        let verb = cell.verb;
        // `drive` installs the plain rejecting hook: it speaks on stderr and exits 1 without
        // touching a byte.
        let case: DoorCase = drive(verb);
        let repo = case.repo.path();
        let home = case.home.path();
        let driven: Vec<&str> = case.driven.iter().map(String::as_str).collect();

        let before = record_bytes(repo);
        let rejected = jigc(repo, home, &driven, None);
        let stderr = String::from_utf8_lossy(&rejected.stderr).into_owned();

        assert!(
            !rejected.status.success(),
            "[{verb}] the control cell must still fail — it is the same rejection; \
             stderr:\n{stderr}",
        );
        assert!(
            !stderr.contains("rollback-conflict"),
            "[{verb}] nothing raced, so no conflict may be reported; stderr:\n{stderr}",
        );
        assert_eq!(
            record_bytes(repo),
            before,
            "[{verb}] the record must be restored to its pre-flip bytes; stderr:\n{stderr}",
        );
        assert!(
            parked(repo, MILESTONE_DOOR.noun).is_empty(),
            "[{verb}] nothing raced, so nothing is parked",
        );
    }
}

/// The success control: no hook at all. The boundary lands, the guard **disarms**, and the
/// committed record keeps the `joined` bytes the commit carried — a compare-and-swap in a
/// destructor must not run on the path that succeeded.
#[test]
fn a_landed_boundary_disarms_and_reports_nothing() {
    for cell in CELLS {
        let verb = cell.verb;
        let case: DoorCase = drive(verb);
        let repo = case.repo.path();
        let home = case.home.path();
        let driven: Vec<&str> = case.driven.iter().map(String::as_str).collect();
        remove_hook(repo);

        let landed = jigc(repo, home, &driven, None);
        let stdout = String::from_utf8_lossy(&landed.stdout).into_owned();
        let stderr = String::from_utf8_lossy(&landed.stderr).into_owned();
        assert!(
            landed.status.success(),
            "[{verb}] the boundary must land; stdout:\n{stdout}\nstderr:\n{stderr}",
        );

        let after = String::from_utf8_lossy(&record_bytes(repo)).into_owned();
        assert!(
            after.contains("joined"),
            "[{verb}] the landed boundary's `joined` record must not be reverted; the file \
             reads:\n{after}",
        );
        assert!(
            !stderr.contains("rollback-conflict") && !stdout.contains("rollback-conflict"),
            "[{verb}] a landed boundary rolls nothing back and reports nothing; \
             stdout:\n{stdout}\nstderr:\n{stderr}",
        );
        assert!(
            parked(repo, MILESTONE_DOOR.noun).is_empty(),
            "[{verb}] a landed boundary parks nothing",
        );
    }
}
