//! M52 Increment 5 / T4 — **the milestone record's five doors, and the two codes they split
//! into** (`completions/artifacts/M52/settle-record.md` → D1.2/D1.3 as amended by §14 and §19;
//! [baseline-rollback.md](../../../completions/artifacts/M52/baseline-rollback.md) §2.1a–2.1c;
//! `design/finalize.md` → Rollback discipline, the record-only-door row).
//!
//! # What was broken
//!
//! The record axis restored **unconditionally**. Every record-only door writes the committed
//! `milestone-record`, `git add`s it and commits only it — and the hook that can refuse that
//! commit runs *inside* the interval between jigc's write and the rollback. So a third party
//! who wrote at the record path in that interval lost their bytes at exit 1, named by nothing:
//!
//!   * the **present** arm rewrote jigc's captured pre-image over whatever the file now held;
//!   * the **absent** arm — `milestone create`, whose pre-image is `(bytes: absent, index:
//!     absent)` — called `remove_file` on a path a racer had written, so the racer's file was
//!     *deleted* (driven at the wave's base, §2.1a: `ls docs/milestone-records/` → empty,
//!     `rollback-conflict` in output: 0).
//!
//! Its sibling population one file over — the config layer's — already had the right rule, and
//! the two disagreed about the same cell inside one binary. `cli::rollback::ROLLBACK_POPULATIONS`
//! is the registry that made the disagreement visible; this suite is what binds the
//! `milestone-record` row to the `FileCas` discipline it declares.
//!
//! # The axis, and where it comes from
//!
//! The doors are **read from that registry row**, never hand-listed here: [`CELLS`] is fenced
//! ⇔ against `ROLLBACK_POPULATIONS`' `milestone-record` row, so a door added to (or dropped
//! from) the population without a driven cell reddens. That direction matters for this class
//! specifically — `design/finalize.md`'s enumeration said **four** doors while the code had
//! **five**, and the missing one (`jigc task discard <sub-task>`, which runs its own
//! record-only commit through `settle_discarded_sub_task`) is exactly the door a fix cut over
//! the design row would have missed (§19).
//!
//! Five doors, **two** codes: the four milestone ops answer with `milestone.rollback-conflict`
//! and a sub-task discard with `task-discard.rollback-conflict`. They are not one code because
//! the stable `(code, target)` key is what a driver branches on, and a raced `jigc task discard`
//! and a raced `jigc milestone discard` rewrite the **same record file** — one code would make
//! the two indistinguishable at exactly the path where they collide.
//!
//! # The two cells, and why the control is half the test
//!
//! 1. [`every_record_door_preserves_a_raced_record_and_names_both_copies`] — a `pre-commit`
//!    hook that **edits the record and exits 1**. Per door: the run fails, the racer's line is
//!    still in the file (for `create`, the file still *exists* — the cell that used to delete
//!    it), the door's own code is rendered blocking and names the live path, the pre-image is
//!    parked under `.jigc/displaced/<door noun>/` with the bytes it captured, and the code is
//!    in the invocation log's `finding_codes`.
//! 2. [`no_record_door_reports_a_conflict_when_nothing_raced`] — the same five doors under a
//!    hook that refuses and **touches nothing**. A compare-and-swap that fired here would be a
//!    law-1 lie on every ordinary rejected run, and the restore must still be byte-exact: the
//!    record's post-run bytes are asserted identical to its pre-run bytes, *absent included*.
//!
//! [`the_machine_arm_carries_the_conflict_in_the_reject_document`] adds the driver's half for
//! one cell: under `--format json` the conflict is *in* the findings envelope on stderr, not
//! printed beside it, so `json.loads(stderr)` still sees exactly one document.

use std::fs;
use std::path::{Path, PathBuf};

use cli::rollback::{
    ConflictDoor, Discipline, MILESTONE_DOOR, ROLLBACK_POPULATIONS, TASK_DISCARD_DOOR,
};

use crate::support::committing_doors::{
    DoorCase, HOOK_MARKER, drive, jigc, log_records, record_for,
};

/// The line the racing hook writes into the record — the third party's bytes, which must be
/// on disk when the run exits however the rollback goes.
const RACE_LINE: &str = "<!-- raced by a concurrent editor -->";

/// One driven cell: a door of the `milestone-record` population, the fixture verb
/// `support::committing_doors::drive` builds it from, the record that fixture's run writes,
/// and the door value whose code and park noun the conflict must carry.
struct Cell {
    /// The door's `cli::cli::VERB_KINDS` leaf path, as the registry row spells it.
    door: &'static [&'static str],
    /// The verb key `drive` builds the fixture under.
    verb: &'static str,
    /// The record the fixture's run writes, repo-relative.
    record: &'static str,
    /// The door's conflict identity and park noun.
    conflict: ConflictDoor,
}

/// Every cell, one per door of the `milestone-record` population. Fenced ⇔ against that row
/// by [`the_cells_are_the_registrys_record_population`], so this table cannot drift from the
/// registry in either direction.
const CELLS: &[Cell] = &[
    Cell {
        door: &["milestone", "create"],
        verb: "jigc milestone create",
        record: "docs/milestone-records/cache-rework.md",
        conflict: MILESTONE_DOOR,
    },
    Cell {
        door: &["milestone", "add-task"],
        verb: "jigc milestone add-task",
        record: "docs/milestone-records/cache-rework.md",
        conflict: MILESTONE_DOOR,
    },
    Cell {
        door: &["milestone", "add-from-spec"],
        verb: "jigc milestone add-from-spec",
        record: "docs/milestone-records/rate-limit.md",
        conflict: MILESTONE_DOOR,
    },
    Cell {
        door: &["milestone", "discard"],
        verb: "jigc milestone discard",
        record: "docs/milestone-records/cache-rework.md",
        conflict: MILESTONE_DOOR,
    },
    Cell {
        door: &["task", "discard"],
        verb: "jigc task discard",
        record: "docs/milestone-records/cache-rework.md",
        conflict: TASK_DISCARD_DOOR,
    },
];

/// The `milestone-record` population's row, as the registry states it.
fn population() -> &'static cli::rollback::Population {
    ROLLBACK_POPULATIONS
        .iter()
        .find(|p| p.id == "milestone-record")
        .expect("`ROLLBACK_POPULATIONS` carries the `milestone-record` population")
}

/// Replace the fixture's rejecting hook with one that **edits the record first** — the racer
/// `design/finalize.md` names, running exactly where it can run: inside the transaction,
/// between jigc's write and the rollback.
fn install_record_racing_hook(repo: &Path, record: &str) {
    let hook = repo.join(".git").join("hooks").join("pre-commit");
    fs::create_dir_all(hook.parent().expect("hooks dir")).expect("mk hooks dir");
    let target = repo.join(record);
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

/// The record's bytes, `None` when it is absent — the value the control arm compares, so that
/// *absent* is asserted as absent rather than as the empty string.
fn record_bytes(repo: &Path, record: &str) -> Option<Vec<u8>> {
    fs::read(repo.join(record)).ok()
}

/// The cell table is the registry row's door set, in both directions.
#[test]
fn the_cells_are_the_registrys_record_population() {
    let row = population();
    assert_eq!(
        row.discipline,
        Discipline::FileCas,
        "the `milestone-record` population declares `FileCas`, which is what this suite drives \
         — a row demoted to another discipline owes a different suite, not a silent pass",
    );

    let declared: Vec<Vec<&str>> = row.doors.iter().map(|d| d.to_vec()).collect();
    let driven: Vec<Vec<&str>> = CELLS.iter().map(|c| c.door.to_vec()).collect();
    assert_eq!(
        driven, declared,
        "every door of the `milestone-record` population owes a driven cell here, and this \
         table may claim no door the population does not have — the four-door enumeration in \
         `design/finalize.md` was short by exactly this kind of drift (settle-record §19)",
    );
}

/// The conflict cell: a hook that edits the record and refuses. The racer's bytes survive at
/// every door, and every door names the pair.
#[test]
fn every_record_door_preserves_a_raced_record_and_names_both_copies() {
    for cell in CELLS {
        let verb = cell.verb;
        let case: DoorCase = drive(verb);
        let repo = case.repo.path();
        let home = case.home.path();
        let driven: Vec<&str> = case.driven.iter().map(String::as_str).collect();

        // What the record held before the door ran — absent at `create`, which is the arm
        // that used to delete a racer's file outright.
        let before = record_bytes(repo, cell.record);
        install_record_racing_hook(repo, cell.record);

        let rejected = jigc(repo, home, &driven, None);
        let stdout = String::from_utf8_lossy(&rejected.stdout).into_owned();
        let stderr = String::from_utf8_lossy(&rejected.stderr).into_owned();

        assert!(
            !rejected.status.success(),
            "[{verb}] a rejected commit must exit non-zero; stdout:\n{stdout}\nstderr:\n{stderr}",
        );

        // (1) **The third party's bytes are on disk.** This is the whole claim: at `create`
        // the file exists at all, and at the four present-pre-image doors it still carries
        // the racer's line rather than jigc's captured pre-image.
        let after = record_bytes(repo, cell.record).unwrap_or_else(|| {
            panic!(
                "[{verb}] the raced record `{}` must survive the rollback — the pre-image \
                 arm may not delete a file a third party wrote; stderr:\n{stderr}",
                cell.record,
            )
        });
        assert!(
            String::from_utf8_lossy(&after).contains(RACE_LINE),
            "[{verb}] the racer's own line must still be in `{}` after the rollback; the \
             file now reads:\n{}",
            cell.record,
            String::from_utf8_lossy(&after),
        );

        // (2) the door's OWN identity, rendered blocking through the house finding line, and
        // naming the live path — the key a driver branches on is `(code, target)`.
        let code = cell.conflict.code;
        assert!(
            stderr.contains(&format!("blocking · {code}")),
            "[{verb}] a raced restore must raise this door's own blocking `{code}`; \
             stderr:\n{stderr}",
        );
        assert!(
            stderr.contains(cell.record),
            "[{verb}] the conflict must name the live path `{}`; stderr:\n{stderr}",
            cell.record,
        );

        // (3) the hook's own bytes are still verbatim beside it — the conflict ADDS to the
        // door's frame and replaces nothing.
        assert!(
            stderr.contains(HOOK_MARKER),
            "[{verb}] the hook's own bytes must survive verbatim beside the conflict; \
             stderr:\n{stderr}",
        );

        // (4) the pre-image is preserved, not discarded — parked under this door's noun when
        // there was one to park, and honestly reported as *never existed* at `create`.
        let parked_copies = parked(repo, cell.conflict.noun);
        match &before {
            Some(bytes) => {
                assert_eq!(
                    parked_copies.len(),
                    1,
                    "[{verb}] exactly one pre-image must be parked under \
                     `.jigc/displaced/{}/`; found {parked_copies:?}",
                    cell.conflict.noun,
                );
                assert_eq!(
                    fs::read(&parked_copies[0]).expect("read the parked pre-image"),
                    *bytes,
                    "[{verb}] the parked copy must hold the record's pre-write bytes",
                );
            }
            None => {
                assert!(
                    parked_copies.is_empty(),
                    "[{verb}] `{}` did not exist before the run, so there is no pre-image to \
                     park; found {parked_copies:?}",
                    cell.record,
                );
                assert!(
                    stderr.contains("did not exist before this"),
                    "[{verb}] the absent-pre-image route must say the file did not exist \
                     before the run and that the rollback did not delete it; stderr:\n{stderr}",
                );
            }
        }

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

/// The zero-false-fire control: the same five doors, a hook that refuses and touches nothing.
/// No conflict, and the restore is byte-exact — *absent* included.
#[test]
fn no_record_door_reports_a_conflict_when_nothing_raced() {
    for cell in CELLS {
        let verb = cell.verb;
        // `drive` installs the plain rejecting hook: it speaks on stderr and exits 1 without
        // touching a byte.
        let case: DoorCase = drive(verb);
        let repo = case.repo.path();
        let home = case.home.path();
        let driven: Vec<&str> = case.driven.iter().map(String::as_str).collect();

        let before = record_bytes(repo, cell.record);
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
            record_bytes(repo, cell.record),
            before,
            "[{verb}] the record must be restored byte-identically (and stay ABSENT where it \
             was absent); stderr:\n{stderr}",
        );
        assert!(
            parked(repo, cell.conflict.noun).is_empty(),
            "[{verb}] nothing raced, so nothing is parked",
        );

        let records = log_records(repo);
        let record = record_for(&records, &case.driven).unwrap_or_else(|| {
            panic!("[{verb}] the refused run must be logged; records:\n{records:#?}")
        });
        assert_eq!(
            record["finding_codes"].as_array().map(Vec::len),
            Some(0),
            "[{verb}] an unraced rejection is an operational error and carries no finding; \
             got {record}",
        );
    }
}

/// The driver's half, on one cell: the conflict rides **inside** the reject document rather
/// than being printed beside it, so the stream a driver reads still parses as exactly one
/// JSON value.
#[test]
fn the_machine_arm_carries_the_conflict_in_the_reject_document() {
    let case = drive("jigc milestone add-task");
    let repo = case.repo.path();
    let home = case.home.path();
    install_record_racing_hook(repo, "docs/milestone-records/cache-rework.md");

    let mut driven: Vec<&str> = case.driven.iter().map(String::as_str).collect();
    driven.extend_from_slice(&["--format", "json"]);
    let rejected = jigc(repo, home, &driven, None);
    let stderr = String::from_utf8_lossy(&rejected.stderr).into_owned();

    let document: serde_json::Value = serde_json::from_str(stderr.trim()).unwrap_or_else(|err| {
        panic!("the reject stream must be exactly one JSON document ({err}); stderr:\n{stderr}")
    });
    let codes: Vec<&str> = document["findings"]
        .as_array()
        .map(|a| a.iter().filter_map(|f| f["code"].as_str()).collect())
        .unwrap_or_default();
    assert!(
        codes.contains(&MILESTONE_DOOR.code),
        "the reject document must carry `{}` beside the refusal; document:\n{document:#}",
        MILESTONE_DOOR.code,
    );
}
