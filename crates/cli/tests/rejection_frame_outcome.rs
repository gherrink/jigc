//! **The committing-door axis × the rollback's OUTCOME** — the fifth sweep of the
//! `*.commit-rejected` frame (M52 completion audit, e2e Finding 2; `settle-record.md` → D1
//! as amended by §1, §2, §19; `design/finalize.md` → 6. Commit, the survivable frame).
//!
//! # The defect this suite exists for
//!
//! `cli::task::RejectionFrame`'s state-truth clause is a per-door **constant**, composed
//! before the transaction ran and therefore before its rollback did. M52 Increment 5 minted
//! exactly the outcomes in which that constant is false — `<door>.rollback-conflict` (the
//! `FileCas` swap declined to overwrite a racer's bytes, so jigc's own write survives) and
//! `<door>.foreign-bytes` (the `MintedSet` unwind declined to remove an area holding a third
//! party's file, so the area survives) — and printed them **beside** the constant, inside one
//! document. Driven at `c80b3f8f`:
//!
//! ```text
//! nothing was committed — the record write and the milestone workbench were both rolled
//! back, so nothing of milestone:text-arm-probe survives. Fix the hook's complaint, then
//! re-run `jigc milestone create 'Text Arm Probe'`.
//! blocking · milestone.rollback-conflict — `docs/milestone-records/text-arm-probe.md`
//!   changed while this milestone-op was running, so the rollback did not restore it …
//! ```
//!
//! …with `docs/milestone-records/text-arm-probe.md` on disk, and the re-run that sentence
//! prescribes refused by `milestone.record-exists`. The sibling cell was worse: at
//! `jigc milestone add-task` the frame said *"the record append and the sub-task mint were
//! both rolled back, so milestone:wave-one is unchanged"* while `milestone.foreign-bytes`
//! named the surviving area two lines below, and the prescribed re-run was refused by
//! `task.serial-collision`.
//!
//! # The axis, and how it is derived
//!
//! The clause is a **function of the rollback's outcome**, so the axis is
//! `<the committing doors that have a conflict-capable rollback population> ×
//! {a surviving path, a completed rollback} × {agent-text, --format json}`.
//!
//! Neither factor is hand-listed. The door set is a **derivation stated as one**
//! ([`conflict_capable_doors`]): the members of
//! [`COMMITTING_DOORS`](cli::invocation_log::COMMITTING_DOORS) — the doors that build a
//! frame — that also appear on some [`ROLLBACK_POPULATIONS`] row whose discipline can leave
//! a path standing ([`Discipline::FileCas`] or [`Discipline::MintedSet`]). A population
//! added for a door, or a door added to the frame family, reddens
//! [`the_cells_are_the_conflict_capable_doors`] rather than passing silently — which is the
//! failure mode this whole wave is about.
//!
//! Exactly one `COMMITTING_DOORS` member is **out**, and the derivation is what puts it out:
//! `jigc migrate-corpus` is on no population, because `commit_migration` runs no rollback at
//! all — which is why its own frame passes an empty conflict slice. The two population rows
//! whose doors are not committing doors — `created-doc-staged-write` (`jigc doc author`,
//! `Declared`) and `config-root-relocation` (`jigc config set`) — are out by the same
//! derivation, because a door that builds no frame can print no frame that lies.
//!
//! # What it adds over the per-population suites
//!
//! `record_rollback_conflict.rs`, `rename_rollback_conflict.rs`, `record_flip_rollback.rs`
//! and `milestone_record_rollback.rs` each own **one population's** conflict cell and assert
//! what the *rollback* did — whose bytes survived, where the pre-image was parked, which
//! code was raised. None of them reads the **frame** printed beside those findings. This
//! suite asserts the one fact none of them can: that the sentence the door prints about the
//! repository agrees with the repository, in the cell where the two could disagree.

use std::collections::BTreeSet;
use std::fs;
use std::path::{Path, PathBuf};

use cli::invocation_log::COMMITTING_DOORS;
use cli::rollback::{Discipline, ROLLBACK_POPULATIONS};

use crate::support::committing_doors::{
    DoorCase, HOOK_MARKER, drive, install_rejecting_hook, jigc,
};

/// The line the racing hook writes at the path it races — the third party's bytes, present
/// on disk when the run exits however the rollback goes.
const RACE_LINE: &str = "<!-- raced by a concurrent editor -->";

/// The file the planting hook drops into a minted working area — the third party's byte that
/// makes the `MintedSet` unwind decline to remove it.
const PLANTED: &str = "third-party.md";

/// The phrase the frame owes whenever the rollback left something standing: the door's own
/// clause **scoped** by the exception, stated before the claim rather than after it, so no
/// absolute stands unqualified.
const SCOPED_PREFIX: &str = "apart from the ";
const SCOPED_TAIL: &str = " named below, which survived the rollback: ";

/// The phrase the recovery sentence owes in the same cell: the re-run is prescribed only
/// *after* the surviving paths' own routes are followed, because at every driven cell below
/// the bare re-run is refused by the door's own identity guard.
const FOLLOW_ROUTES: &str = " and follow each surviving path's route below, then re-run ";

/// The recovery instruction the **completed-rollback** cell keeps, byte-for-byte as M42
/// shipped it and M51 gave its non-hook sibling.
const BARE_RERUN_HOOK: &str = ". Fix the hook's complaint, then re-run ";
const BARE_RERUN_NON_HOOK: &str = ". Resolve the cause above, then re-run ";

/// How a cell produces its surviving path.
#[derive(Clone, Copy)]
enum Racer {
    /// A [`Discipline::FileCas`] population: the hook appends at a path the door's rollback
    /// would restore, so the compare-and-swap declines and jigc's write survives.
    Cas(&'static str),
    /// A [`Discipline::MintedSet`] population: the hook plants a file inside every area
    /// under this parent, so the non-recursive unwind declines and the area survives.
    Mint(&'static str),
}

/// One driven cell.
struct Cell {
    /// The `COMMITTING_DOORS` verb key — the fixture `drive` builds, and the door whose
    /// frame is under assertion.
    verb: &'static str,
    /// How the surviving path is produced.
    racer: Racer,
    /// The blocking code the door raises over it — the witness that the cell reached the
    /// outcome it claims, rather than passing vacuously.
    code: &'static str,
    /// Anything the fixture owes before the racer runs.
    prepare: Option<fn(&Path)>,
}

/// **Every conflict-capable committing door, driven at each shape it can reach** — fenced ⇔
/// against [`conflict_capable_doors`] by [`the_cells_are_the_conflict_capable_doors`].
///
/// A door with conflict-capable populations of **both** disciplines is driven twice, because
/// the two leave a path standing for opposite reasons: a `FileCas` row declines to
/// *overwrite*, a `MintedSet` row declines to *remove*, and a clause honest about one can
/// still lie about the other. It did — the audit's two repros are one door apart and are
/// exactly these two shapes.
const CELLS: &[Cell] = &[
    // `config-layer-worktree` — the amend `finalize` makes to the user's co-owned ignore
    // file. The prep puts both config-layer files one upgrade behind, which is the only
    // shape in which that amend writes at all, and therefore the only one with a pre-image.
    Cell {
        verb: "jigc task finalize",
        racer: Racer::Cas(".jigc/.gitignore"),
        code: "finalize.rollback-conflict",
        prepare: Some(stale_config_layer),
    },
    // `fan-out-record-flip` — the boundary's active → joined mutation of the record. The
    // guard carries the **milestone-op** conflict identity rather than `finalize`'s, because
    // it rewrites the same record file the four record-only doors do and a shared code would
    // be unkeyable exactly where they collide (`record_flip_rollback.rs` reads the same
    // door value).
    Cell {
        verb: "jigc milestone finalize (squash: true)",
        racer: Racer::Cas("docs/milestone-records/cache-rework.md"),
        code: "milestone.rollback-conflict",
        prepare: None,
    },
    Cell {
        verb: "jigc milestone finalize (squash: false)",
        racer: Racer::Cas("docs/milestone-records/cache-rework.md"),
        code: "milestone.rollback-conflict",
        prepare: None,
    },
    // `rename-worktree` — the landing path, one of the two worktree paths HEAD cannot
    // answer for (`rename_rollback_conflict.rs` owns the same racer for the rollback's half).
    Cell {
        verb: "jigc rename",
        racer: Racer::Cas("docs/decisions/beta-decision.md"),
        code: "rename.rollback-conflict",
        prepare: None,
    },
    // `milestone-record` at its five doors, and `milestone-mint-area` /
    // `unrecorded-seed-areas` at the three of them that mint.
    Cell {
        verb: "jigc milestone create",
        racer: Racer::Cas("docs/milestone-records/cache-rework.md"),
        code: "milestone.rollback-conflict",
        prepare: None,
    },
    Cell {
        verb: "jigc milestone create",
        racer: Racer::Mint(".jigc/milestones"),
        code: "milestone.foreign-bytes",
        prepare: None,
    },
    Cell {
        verb: "jigc milestone add-task",
        racer: Racer::Cas("docs/milestone-records/cache-rework.md"),
        code: "milestone.rollback-conflict",
        prepare: None,
    },
    Cell {
        verb: "jigc milestone add-task",
        racer: Racer::Mint(".jigc/tasks"),
        code: "milestone.foreign-bytes",
        prepare: None,
    },
    Cell {
        verb: "jigc milestone add-from-spec",
        racer: Racer::Cas("docs/milestone-records/rate-limit.md"),
        code: "milestone.rollback-conflict",
        prepare: None,
    },
    Cell {
        verb: "jigc milestone add-from-spec",
        racer: Racer::Mint(".jigc/tasks"),
        code: "milestone.foreign-bytes",
        prepare: None,
    },
    Cell {
        verb: "jigc milestone discard",
        racer: Racer::Cas("docs/milestone-records/cache-rework.md"),
        code: "milestone.rollback-conflict",
        prepare: None,
    },
    Cell {
        verb: "jigc task discard",
        racer: Racer::Cas("docs/milestone-records/cache-rework.md"),
        code: "task-discard.rollback-conflict",
        prepare: None,
    },
];

/// Put the config layer one upgrade behind, so `finalize`'s amend genuinely writes — the
/// same preparation `flow53_acceptance.rs` → `stale_config_layer` makes for the same row.
fn stale_config_layer(repo: &Path) {
    let jigc_dir = repo.join(".jigc");
    fs::write(
        jigc_dir.join(".gitignore"),
        "tasks/\nindex/\nstate/\nmilestones/\nworktrees/\nlogs/\n",
    )
    .expect("write the one-entry-short ignore file");
    fs::write(jigc_dir.join("version"), "0.0.0-fixture-stale-stamp\n")
        .expect("write the stale version stamp");
}

/// **The derivation**: the committing doors that can reach a rollback outcome the frame's
/// constant would lie about — the `COMMITTING_DOORS` members named by some
/// `ROLLBACK_POPULATIONS` row whose discipline can leave a path standing.
///
/// The population rows spell a door as its `VERB_KINDS` leaf path (`["milestone",
/// "finalize"]`); `COMMITTING_DOORS` spells the same door as the verb a user types, and
/// splits `milestone finalize` into its two commit-model arms. The join is therefore on the
/// verb **prefix**, which is what makes both arms members of the one population's door set.
fn conflict_capable_doors() -> BTreeSet<&'static str> {
    let standing: Vec<String> = ROLLBACK_POPULATIONS
        .iter()
        .filter(|row| {
            matches!(
                row.discipline,
                Discipline::FileCas | Discipline::MintedSet(_)
            )
        })
        .flat_map(|row| row.doors.iter())
        .map(|door| format!("jigc {}", door.join(" ")))
        .collect();
    COMMITTING_DOORS
        .iter()
        .map(|door| door.verb)
        .filter(|verb| {
            standing
                .iter()
                .any(|spelled| verb.starts_with(spelled.as_str()))
        })
        .collect()
}

/// This door's own route-exempt error identity, as the registry declares it — the code the
/// `--format json` reject document keys its framed refusal under.
fn frame_code(verb: &str) -> &'static str {
    COMMITTING_DOORS
        .iter()
        .find(|door| door.verb == verb)
        .unwrap_or_else(|| panic!("`{verb}` is a `COMMITTING_DOORS` member"))
        .error_code
}

/// The cell table **is** the derived door set, in both directions.
#[test]
fn the_cells_are_the_conflict_capable_doors() {
    let derived = conflict_capable_doors();
    let driven: BTreeSet<&str> = CELLS.iter().map(|cell| cell.verb).collect();
    assert_eq!(
        driven, derived,
        "every committing door with a conflict-capable rollback population owes a driven \
         cell here, and this table may claim no door the derivation does not give — a class \
         nobody enumerates is a class a fix is cut short of",
    );
    // …and the member that is deliberately out is out *by the derivation*, not by being
    // forgotten: `jigc migrate-corpus` runs no rollback, which is why its own frame passes
    // an empty conflict slice.
    let all: BTreeSet<&str> = COMMITTING_DOORS.iter().map(|d| d.verb).collect();
    let excluded: Vec<&str> = all.difference(&derived).copied().collect();
    assert_eq!(
        excluded,
        vec!["jigc migrate-corpus"],
        "the only committing door outside the rollback family is `jigc migrate-corpus`; a \
         second exclusion is a population nobody wrote down",
    );
}

/// Replace the fixture's rejecting hook with the cell's racer, which runs exactly where a
/// racer can run: inside the transaction, between jigc's write and its rollback.
fn install_racer(repo: &Path, racer: Racer) {
    let script = match racer {
        Racer::Cas(target) => {
            let path = repo.join(target);
            format!(
                "#!/bin/sh\nmkdir -p {parent:?}\necho '{RACE_LINE}' >> {path:?}\n\
                 echo '{HOOK_MARKER}' 1>&2\nexit 1\n",
                parent = path.parent().expect("the raced path has a parent"),
            )
        }
        Racer::Mint(parent) => {
            let dir = repo.join(parent);
            format!(
                "#!/bin/sh\nfor d in {dir:?}/*/; do [ -d \"$d\" ] && \
                 echo '{RACE_LINE}' > \"$d/{PLANTED}\"; done\n\
                 echo '{HOOK_MARKER}' 1>&2\nexit 1\n",
            )
        }
    };
    write_hook(repo, &script);
}

/// Write `script` as the fixture's `pre-commit` hook.
fn write_hook(repo: &Path, script: &str) {
    let hook = repo.join(".git").join("hooks").join("pre-commit");
    fs::create_dir_all(hook.parent().expect("hooks dir")).expect("mk hooks dir");
    fs::write(&hook, script).expect("write the pre-commit hook");
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        fs::set_permissions(&hook, fs::Permissions::from_mode(0o755)).expect("chmod hook");
    }
}

/// The frame's recovery sentence and the number of findings printed **beside** it — read off
/// what the binary wrote, on whichever arm it wrote it.
///
/// On the agent-text arm the sentence is the paragraph after git's verbatim relay and the
/// conflicts are the house `blocking · <code>` lines under it; under `--format json` the
/// sentence is the framed refusal's `route` and the conflicts are its siblings in the one
/// findings envelope. Both are the *same* composed string reaching two carriers, which is
/// why one lifter serves both.
fn frame_recovery(stderr: &str, verb: &str, json: bool) -> (String, usize) {
    if json {
        let doc: serde_json::Value = serde_json::from_str(stderr.trim()).unwrap_or_else(|err| {
            panic!(
                "[{verb}] the reject stream must be exactly one JSON document ({err}):\n{stderr}"
            )
        });
        let findings = doc["findings"]
            .as_array()
            .unwrap_or_else(|| {
                panic!("[{verb}] the reject document carries a findings array:\n{stderr}")
            })
            .clone();
        let code = frame_code(verb);
        let route = findings
            .iter()
            .find(|f| f["code"].as_str() == Some(code))
            .and_then(|f| f["route"].as_str())
            .unwrap_or_else(|| {
                panic!("[{verb}] the reject document carries `{code}` with its route:\n{stderr}")
            })
            .to_owned();
        // Everything that is not the door's own framed refusal is a path the rollback left
        // standing — the count the clause has to agree with.
        let beside = findings
            .iter()
            .filter(|f| f["code"].as_str() != Some(code))
            .count();
        return (route, beside);
    }
    let sentence = stderr
        .lines()
        .find(|line| line.contains(", then re-run "))
        .unwrap_or_else(|| panic!("[{verb}] the frame prints its recovery sentence:\n{stderr}"))
        .to_owned();
    let beside = stderr
        .lines()
        .filter(|line| line.starts_with("blocking · "))
        .count();
    (sentence, beside)
}

/// The path the cell's racer leaves standing.
fn surviving_path(repo: &Path, racer: Racer) -> PathBuf {
    match racer {
        Racer::Cas(target) => repo.join(target),
        Racer::Mint(parent) => {
            let dir = repo.join(parent);
            let mut areas: Vec<PathBuf> = fs::read_dir(&dir)
                .unwrap_or_else(|err| panic!("read `{}`: {err}", dir.display()))
                .flatten()
                .map(|entry| entry.path())
                .filter(|path| path.join(PLANTED).is_file())
                .collect();
            areas.sort();
            areas
                .pop()
                .unwrap_or_else(|| panic!("an area under `{}` survived", dir.display()))
        }
    }
}

/// **The claim.** At every conflict-capable door, on both surfaces: when the rollback left a
/// path standing, the frame says so *before* it states what the door put back, states a
/// count the repository can be checked against, and prescribes the re-run only behind the
/// survivors' own routes.
#[test]
fn the_frame_states_what_the_rollback_actually_left() {
    for cell in CELLS {
        for json in [false, true] {
            let arm = if json { "--format json" } else { "agent-text" };
            let verb = cell.verb;
            let case: DoorCase = drive(verb);
            let repo = case.repo.path();
            let home = case.home.path();
            if let Some(prepare) = cell.prepare {
                prepare(repo);
            }
            install_racer(repo, cell.racer);

            let mut argv: Vec<&str> = case.driven.iter().map(String::as_str).collect();
            if json {
                argv.extend(["--format", "json"]);
            }
            let out = jigc(repo, home, &argv, None);
            let stderr = String::from_utf8_lossy(&out.stderr).into_owned();
            assert!(
                !out.status.success(),
                "[{verb} · {arm}] a rejected commit must exit non-zero; stderr:\n{stderr}",
            );

            // (0) the cell genuinely reached the outcome it claims — the door raised its own
            // conflict identity. Without this the assertions below could pass vacuously over
            // a cell whose racer stopped racing.
            assert!(
                stderr.contains(cell.code),
                "[{verb} · {arm}] this cell must reach `{}` — a cell that no longer leaves a \
                 path standing proves nothing about the clause; stderr:\n{stderr}",
                cell.code,
            );

            // (1) the surviving path is on disk. The clause is checked against the
            // repository, never against another sentence.
            let survivor = surviving_path(repo, cell.racer);
            assert!(
                survivor.exists(),
                "[{verb} · {arm}] `{}` must be on disk — it is what the clause has to be \
                 true about; stderr:\n{stderr}",
                survivor.display(),
            );

            let (sentence, beside) = frame_recovery(&stderr, verb, json);

            // (2) the clause is SCOPED, and the scope opens it, so the door's absolute never
            // stands before its own contradiction.
            let scope_at = sentence.find(SCOPED_PREFIX).unwrap_or_else(|| {
                panic!(
                    "[{verb} · {arm}] the rollback left a path standing, so the frame owes \
                     the exception BEFORE the door's clause; it printed:\n{sentence}"
                )
            });
            let tail_at = sentence.find(SCOPED_TAIL).unwrap_or_else(|| {
                panic!("[{verb} · {arm}] the scope names what it excepts:\n{sentence}")
            });
            assert!(
                scope_at < tail_at,
                "[{verb} · {arm}] the exception opens the clause:\n{sentence}",
            );

            // (3) the count is the number of findings printed beside the frame — one per
            // path the rollback could not put back, which is what the registry's own
            // contract promises. A clause that counted differently would be a second,
            // disagreeing statement about the same event.
            let noun = if beside == 1 { "path" } else { "paths" };
            assert!(
                sentence.contains(&format!("{SCOPED_PREFIX}{beside} {noun}{SCOPED_TAIL}")),
                "[{verb} · {arm}] {beside} finding(s) name a surviving path, so the clause \
                 states that count:\n{sentence}",
            );

            // (4) the re-run is not prescribed bare. At every cell here the identical re-run
            // is refused by the door's own identity guard until the survivor is dealt with,
            // so a bare `then re-run` is a route the state forbids.
            assert!(
                sentence.contains(FOLLOW_ROUTES),
                "[{verb} · {arm}] the re-run may be prescribed only behind the surviving \
                 paths' own routes:\n{sentence}",
            );
            assert!(
                !sentence.contains(BARE_RERUN_HOOK) && !sentence.contains(BARE_RERUN_NON_HOOK),
                "[{verb} · {arm}] the completed-rollback recovery instruction may not print \
                 over a rollback that did not complete:\n{sentence}",
            );

            // (5) the door's OWN clause is still there — the scope qualifies it, it does not
            // replace it. What the rollback *did* put back stays stated.
            assert!(
                sentence.contains(case.survived.as_str()),
                "[{verb} · {arm}] the door's own state clause (`{}`) is scoped, not \
                 discarded:\n{sentence}",
                case.survived,
            );
        }
    }
}

/// **The zero-false-fire control, over the whole frame family.** A rejection whose rollback
/// completed keeps the door's clause exactly as M42 through M51 pinned it: no exception, no
/// route detour, the bare re-run. Iterated over every `COMMITTING_DOORS` member — including
/// `jigc migrate-corpus`, which has no rollback at all and must therefore never see the
/// scope.
#[test]
fn a_completed_rollback_states_the_doors_own_clause_unqualified() {
    for door in COMMITTING_DOORS {
        let verb = door.verb;
        let case: DoorCase = drive(verb);
        let repo = case.repo.path();
        let home = case.home.path();
        // The plain rejecting hook: it refuses and races nothing, so every restore the door
        // makes completes.
        install_rejecting_hook(repo);
        let argv: Vec<&str> = case.driven.iter().map(String::as_str).collect();
        let out = jigc(repo, home, &argv, None);
        let stderr = String::from_utf8_lossy(&out.stderr).into_owned();
        assert!(
            !out.status.success(),
            "[{verb}] a rejected commit must exit non-zero; stderr:\n{stderr}",
        );
        let (sentence, beside) = frame_recovery(&stderr, verb, false);
        assert_eq!(
            beside, 0,
            "[{verb}] this control cell races nothing, so nothing may survive the \
             rollback:\n{stderr}",
        );
        assert!(
            !sentence.contains(SCOPED_PREFIX) && !sentence.contains(FOLLOW_ROUTES),
            "[{verb}] nothing survived this rollback, so the frame must not invent an \
             exception:\n{sentence}",
        );
        assert!(
            sentence.contains(BARE_RERUN_HOOK),
            "[{verb}] a completed rollback keeps the frame's own recovery instruction, \
             unchanged:\n{sentence}",
        );
        assert!(
            sentence.contains(case.survived.as_str()),
            "[{verb}] the door's own state clause still prints:\n{sentence}",
        );
    }
}
