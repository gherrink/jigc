//! M53 Increment 3 / T1 — **the residual rule's load-bearing property, made a standing
//! fence**: every legitimate working area carries its base pin at the first moment any
//! door can observe it, and a directory wearing the pin's name is not the pin
//! (`completions/artifacts/M53/settle-record.md` → D3;
//! `acceptance-design.md` → Spikes owed, increment 3;
//! `baseline-residual-area-census.md` §2 and §2.1;
//! `implementation/roadmap.md` → M53 Increment 3).
//!
//! **The claim this increment rests on.** Increment 3 makes
//! `symlink_metadata(<area>/base.json).is_file()` the predicate that decides whether a
//! directory under `.jigc/tasks/` (or `.jigc/milestones/`) is a work unit at all — at the
//! shared enumerator, at one shared by-id resolve predicate, and at `jigc rename`'s
//! milestone twin. A predicate is only as truthful as its converse: if **one** production
//! mint could leave an area pin-less at a moment a door observes it, the fix would answer
//! *no such task* over a real, live work unit, and the resume route it names would be the
//! dead end it replaced.
//!
//! So the property is driven here, per `engine::state::MINT_DOORS` row, through the real
//! binary — including the two `Snapshot::Exempt` rows and the fresh-clone re-seed, which
//! is driven over a **real `git clone`** of a real origin rather than a simulated one
//! (`milestone_record_fresh_clone.rs`'s discipline: a fixture that reaches inside
//! `.jigc/tasks/` would be asserting the state the tool is supposed to rebuild).
//!
//! **Why a fence and not a one-off drive.** Both halves were driven at planning and are on
//! the record; a drive that happened once proves the sha it ran on. The predicate ships
//! into five doors, so the property it rests on is checked where a sixth mint door would
//! have to join it: `MINT_DOORS` is the subject, a member with no cell is a **hard panic**
//! rather than a skip, and [`every_mint_door_row_has_a_driven_pin_cell`] asserts the driven
//! set back against the registry in both directions.
//!
//! **The shape leg.** `engine::state::unwind_area`'s doc states *"Shape is part of
//! membership … a directory named `base.json` … none of those is something jigc wrote, so
//! none is removed, and the area then survives as `Foreign`"* — the census read that branch
//! and recorded it **unexercised** (§2.1, *"cheaper shape-hole"*). It is the reason the
//! predicate is `symlink_metadata(…).is_file()` and not `.exists()`: an existence-keyed
//! predicate would call such a survivor a live task forever. It is driven below **with its
//! own control** — the identical area carrying a regular-file pin unwinds to `Removed` — so
//! the leg cannot pass by being inert.
//!
//! **Declared bound, carried from D3.** The mid-mint window (`create_dir_all` →
//! `fs::write(base.json)`, two syscalls) is narrowed, not closed: a door racing a mint can
//! still observe the directory before the pin. It flickers in the safe direction (a
//! just-minted task is briefly absent from the roster rather than a finalized one briefly
//! present), and jigc's one designed racer — the hook (`design/storage.md` → Concurrent
//! writers) — does not run inside `mint_task`. Nothing here drives a racer; each cell
//! observes the area after the mint returned, which is the moment every production door
//! observes it.

use engine::state::{MILESTONE_AREA_FILES, MINT_DOORS, Snapshot, TASK_AREA_FILES};
use std::collections::BTreeSet;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

// ───────────────────────────── the fixture primitives ─────────────────────────────

/// A throwaway directory that removes itself on drop.
struct TempDir(PathBuf);

impl TempDir {
    fn new(tag: &str) -> Self {
        let mut path = std::env::temp_dir();
        path.push(format!(
            "jigc-mint-pin-{tag}-{}-{:?}",
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

/// Run `git` in `cwd`, asserting success, returning trimmed stdout.
fn git(cwd: &Path, args: &[&str]) -> String {
    let out = Command::new("git")
        .args(args)
        .current_dir(cwd)
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

/// Run `jigc <args>` with `cwd = repo`, `$HOME = home`, against the embedded packs.
fn jigc(repo: &Path, home: &Path, args: &[&str]) -> std::process::Output {
    Command::new(env!("CARGO_BIN_EXE_jigc"))
        .args(args)
        .current_dir(repo)
        .env("HOME", home)
        .env_remove("JIGC_PACK_DIR")
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()
        .expect("spawn the jigc binary")
}

/// Run `jigc <args>`, assert exit 0, return stdout.
fn jigc_ok(repo: &Path, home: &Path, args: &[&str]) -> String {
    let out = jigc(repo, home, args);
    assert!(
        out.status.success(),
        "`jigc {args:?}` must exit 0; stdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr),
    );
    String::from_utf8(out.stdout).expect("utf-8 jigc stdout")
}

/// The id a mint **printed**, read off the real output — never reconstructed from the
/// intent, because the slug rule is the binary's and a test-side copy of it drifts.
fn minted_task(stdout: &str) -> String {
    stdout
        .lines()
        .find_map(|line| line.strip_prefix("task minted: "))
        .unwrap_or_else(|| panic!("a mint must print `task minted: <id>`; got:\n{stdout}"))
        .trim()
        .to_string()
}

/// A real git repo with one commit under `<workdir>/repo`.
fn init_repo(workdir: &Path) -> PathBuf {
    let repo = workdir.join("repo");
    fs::create_dir_all(&repo).expect("mk the repo dir");
    git(&repo, &["init", "-q"]);
    git(&repo, &["config", "user.email", "test@example.com"]);
    git(&repo, &["config", "user.name", "Test"]);
    fs::write(repo.join("README.md"), "hello\n").expect("write README");
    git(&repo, &["add", "README.md"]);
    git(&repo, &["commit", "-q", "-m", "initial"]);
    repo
}

/// The `[dev ▸ methodology]` compose marker — a milestone needs the record's home.
fn write_compose_marker(repo: &Path) {
    fs::create_dir_all(repo.join(".jigc").join("config")).expect("mk project config");
    fs::write(
        repo.join(".jigc").join("config").join("packs.yaml"),
        "compose-embedded-methodology: true\n",
    )
    .expect("write compose marker");
}

/// `.jigc/tasks/<id>` under `repo`.
fn task_area(repo: &Path, id: &str) -> PathBuf {
    repo.join(".jigc").join("tasks").join(id)
}

/// `.jigc/milestones/<id>` under `repo`.
fn milestone_area(repo: &Path, id: &str) -> PathBuf {
    repo.join(".jigc").join("milestones").join(id)
}

// ───────────────────────────── the pin, as the registry names it ─────────────────────

/// The pin's name, taken from the writer registry rather than written down — D3 keys the
/// predicate on `TASK_AREA_FILES[0]` / `MILESTONE_AREA_FILES[0]`, and a positional citation
/// that nothing checks goes stale the first time somebody sorts a row.
fn pin_name(kind: engine::state::WorkArea) -> &'static str {
    kind.jigc_written()[0]
}

/// Both registry rows lead with the same member, and it is the pin — so the two areas'
/// predicates are one predicate, and the positional citation in D3 is true rather than
/// remembered.
#[test]
fn both_writer_registry_rows_lead_with_the_base_pin() {
    assert_eq!(
        TASK_AREA_FILES[0], "base.json",
        "the residual predicate keys on the FIRST member of the task row; a reorder must \
         redden here rather than silently changing what every cell below checks",
    );
    assert_eq!(
        MILESTONE_AREA_FILES[0], "base.json",
        "…and on the first member of the milestone row, so `jigc rename`'s milestone twin \
         asks the same question the task seams ask",
    );
}

/// The one measurement every cell makes: the area holds its pin **as a regular file**,
/// read through `symlink_metadata` so a symlink wearing the name is not mistaken for it.
///
/// This is the exact predicate Increment 3 ships into the enumerator, the shared resolve
/// seam and `rename`'s milestone twin — asked here of what the production mint produced.
fn assert_carries_its_pin(area: &Path, kind: engine::state::WorkArea, door: &str) {
    assert!(
        area.is_dir(),
        "`{door}` must have minted a working area at `{}`",
        area.display(),
    );
    let pin = area.join(pin_name(kind));
    let shape = fs::symlink_metadata(&pin).unwrap_or_else(|err| {
        panic!(
            "`{door}` minted `{}` with NO base pin ({err}) — the residual predicate would \
             answer `finalize.no-task` over a live work unit, and its route would dead-end. \
             The area held {:?}",
            area.display(),
            entry_names(area),
        )
    });
    assert!(
        shape.is_file(),
        "`{door}`'s pin at `{}` must be a REGULAR FILE — the predicate is \
         `symlink_metadata(…).is_file()`, not `.exists()`, so a directory or a symlink \
         wearing the name is not a work unit",
        pin.display(),
    );
}

/// Every entry name directly under `area`, sorted — evidence for a red assertion, so a
/// failure says what the area actually held.
fn entry_names(area: &Path) -> Vec<String> {
    let mut names: Vec<String> = match fs::read_dir(area) {
        Ok(entries) => entries
            .map(|entry| {
                entry
                    .expect("a working-area entry")
                    .file_name()
                    .to_string_lossy()
                    .into_owned()
            })
            .collect(),
        Err(_) => Vec::new(),
    };
    names.sort();
    names
}

// ───────────────────────────── the cells, one per registry row ───────────────────────

/// What one cell reports: every working area its door minted, with the registry row each
/// belongs to. A door that mints more than one area (the re-seed rebuilds every sub-task
/// the record names) returns them all — the claim is about *every* legitimate area, not a
/// representative one.
type Minted = Vec<(PathBuf, engine::state::WorkArea)>;

/// **Cell — `jigc start "<intent>"`** (`crates/cli/src/start.rs::mint_in_repo`).
/// `jigc task amend` — the amend task's own area. Its pin is load-bearing twice over: it is
/// the residual predicate's subject like every other area's, **and** it is what the finalize
/// arm compares HEAD against before it rewrites a commit (`finalize.base-mismatch`).
fn cell_amend(workdir: &Path, home: &Path) -> Minted {
    let repo = init_repo(workdir);
    jigc_ok(&repo, home, &["setup"]);
    let id = minted_task(&jigc_ok(
        &repo,
        home,
        &["task", "amend", "carry the base pin"],
    ));
    vec![(task_area(&repo, &id), engine::state::WorkArea::Task)]
}

fn cell_start(workdir: &Path, home: &Path) -> Minted {
    let repo = init_repo(workdir);
    jigc_ok(&repo, home, &["setup"]);
    let id = minted_task(&jigc_ok(
        &repo,
        home,
        &["start", "--workflow", "single-task", "carry the base pin"],
    ));
    vec![(task_area(&repo, &id), engine::state::WorkArea::Task)]
}

/// **Cell — `jigc migrate <path> --as <doctype>`**
/// (`crates/cli/src/start.rs::mint_migration_in_repo`).
fn cell_migrate(workdir: &Path, home: &Path) -> Minted {
    let repo = init_repo(workdir);
    fs::write(
        repo.join("HISTORY.md"),
        "# Changelog\n\n## [0.1.0] - 2021-03-09\n### Added\n- First public release.\n",
    )
    .expect("write HISTORY.md");
    git(&repo, &["add", "HISTORY.md"]);
    git(&repo, &["commit", "-q", "-m", "track HISTORY.md"]);
    jigc_ok(&repo, home, &["setup"]);

    let id = minted_task(&jigc_ok(
        &repo,
        home,
        &["migrate", "HISTORY.md", "--as", "changelog"],
    ));
    vec![(task_area(&repo, &id), engine::state::WorkArea::Task)]
}

/// The milestone id every milestone cell shares, as `jigc milestone create` slugs it.
const MILESTONE: &str = "cache-rework";

/// Build a `[dev ▸ methodology]` repo carrying a created milestone with two sub-tasks,
/// each landing its own record-only commit — so a clone of it carries the record.
fn drive_milestone_origin(workdir: &Path, home: &Path) -> PathBuf {
    let repo = init_repo(workdir);
    write_compose_marker(&repo);
    git(&repo, &["add", "."]);
    git(
        &repo,
        &["commit", "-q", "-m", "compose the methodology pack"],
    );
    jigc_ok(&repo, home, &["milestone", "create", "Cache rework"]);
    for intent in ["Warm the read cache", "Evict cold entries"] {
        jigc_ok(&repo, home, &["milestone", "add-task", MILESTONE, intent]);
    }
    repo
}

/// **Cell — `jigc milestone create "<title>"`** (`crates/cli/src/milestone.rs::run_create`)
/// — the one door that mints a **milestone** area.
fn cell_milestone_create(workdir: &Path, home: &Path) -> Minted {
    let repo = init_repo(workdir);
    write_compose_marker(&repo);
    jigc_ok(&repo, home, &["milestone", "create", "Cache rework"]);
    vec![(
        milestone_area(&repo, MILESTONE),
        engine::state::WorkArea::Milestone,
    )]
}

/// **Cell — `jigc milestone add-task`** (`crates/engine/src/milestone.rs::add_task`), a
/// `Snapshot::Exempt` row: exempt from the *carryover snapshot*, never from the pin.
fn cell_add_task(workdir: &Path, home: &Path) -> Minted {
    let repo = drive_milestone_origin(workdir, home);
    ["warm-the-read-cache", "evict-cold-entries"]
        .iter()
        .map(|sub| (task_area(&repo, sub), engine::state::WorkArea::Task))
        .collect()
}

/// **Cell — the record-driven re-seed on a fresh clone**
/// (`crates/engine/src/milestone.rs::reseed_sub_task_areas`), the second `Snapshot::Exempt`
/// row — driven over a **real `git clone`**, which is the only shape that carries exactly
/// the tracked bytes a teammate gets: the committed record, and none of the gitignored
/// workbench the re-seed exists to rebuild.
fn cell_reseed(workdir: &Path, home: &Path) -> Minted {
    let origin = drive_milestone_origin(workdir, home);
    let clone = workdir.join("clone");
    git(
        workdir,
        &[
            "clone",
            "-q",
            &origin.display().to_string(),
            &clone.display().to_string(),
        ],
    );
    git(&clone, &["config", "user.email", "test@example.com"]);
    git(&clone, &["config", "user.name", "Test"]);
    assert!(
        !clone.join(".jigc").join("tasks").exists(),
        "a fresh clone carries no sub-task working areas — that absence is what the \
         re-seed rebuilds from the committed record",
    );
    assert!(
        !clone.join(".jigc").join("milestones").exists(),
        "…and no milestone workbench either",
    );

    // An *operating* milestone op re-seeds; `list-tasks` is `VerbKind::Read` and rebuilds
    // nothing (M49 Inc 2 T4), so it would measure the wrong door.
    jigc_ok(&clone, home, &["milestone", "provision", MILESTONE]);

    ["warm-the-read-cache", "evict-cold-entries"]
        .iter()
        .map(|sub| (task_area(&clone, sub), engine::state::WorkArea::Task))
        .collect()
}

// ───────────────────────────── the dispatch, by registry site ────────────────────────

/// One cell, driven.
type Cell = fn(&Path, &Path) -> Minted;

/// The cell that drives one `MINT_DOORS` member, **dispatched by its declared site**.
///
/// A member with no cell is a hard panic, not a skip: a sixth mint door that silently ran
/// no test would leave the residual predicate resting on a property nobody measured — which
/// is the remembered list `MINT_DOORS` replaced, wearing a table's clothes.
fn cell_for(site: &str, door: &str) -> Cell {
    match site {
        "crates/cli/src/start.rs::mint_in_repo" => cell_start,
        "crates/cli/src/start.rs::mint_amend_in_repo" => cell_amend,
        "crates/cli/src/start.rs::mint_migration_in_repo" => cell_migrate,
        "crates/cli/src/milestone.rs::run_create" => cell_milestone_create,
        "crates/engine/src/milestone.rs::add_task" => cell_add_task,
        "crates/engine/src/milestone.rs::reseed_sub_task_areas" => cell_reseed,
        other => panic!(
            "`MINT_DOORS` member `{other}` ({door}) has no pin cell — a new mint door owes \
             one here, or M53 Increment 3's residual predicate rests on a property that \
             door was never measured against",
        ),
    }
}

/// Every site this suite drives — the set [`every_mint_door_row_has_a_driven_pin_cell`]
/// asserts back against the registry, in both directions.
const DRIVEN_SITES: &[&str] = &[
    "crates/cli/src/start.rs::mint_in_repo",
    "crates/cli/src/start.rs::mint_migration_in_repo",
    "crates/cli/src/start.rs::mint_amend_in_repo",
    "crates/cli/src/milestone.rs::run_create",
    "crates/engine/src/milestone.rs::add_task",
    "crates/engine/src/milestone.rs::reseed_sub_task_areas",
];

/// Drive one registry row's cell and assert every area it minted carries its pin.
fn drive(site: &str) {
    let door = MINT_DOORS
        .iter()
        .find(|entry| entry.site == site)
        .unwrap_or_else(|| {
            panic!(
                "`{site}` is no longer an `engine::state::MINT_DOORS` member — a moved or \
                 deleted mint leaves this cell measuring a door nobody can reach",
            )
        });

    let workdir = TempDir::new("cell");
    let home = TempDir::new("home");
    let minted = cell_for(door.site, door.door)(workdir.path(), home.path());
    assert!(
        !minted.is_empty(),
        "`{}` must report the areas it minted, or the cell asserts nothing",
        door.site,
    );
    for (area, kind) in minted {
        assert_carries_its_pin(&area, kind, door.door);
    }
}

// ───────────────────────────── one test per registry row ─────────────────────────────

/// `jigc start "<intent>"` — the ordinary mint.
#[test]
fn the_start_door_mints_an_area_carrying_its_base_pin() {
    drive(DRIVEN_SITES[0]);
}

/// `jigc migrate <path> --as <doctype>` — the migration task's own area.
#[test]
fn the_migrate_door_mints_an_area_carrying_its_base_pin() {
    drive(DRIVEN_SITES[1]);
}

/// `jigc task amend ["<intent>"]` — the amend task's area.
#[test]
fn the_amend_door_mints_an_area_carrying_its_base_pin() {
    drive(DRIVEN_SITES[2]);
}

/// `jigc milestone create "<title>"` — the milestone area, whose pin is what
/// `jigc rename`'s `first_dir_name` twin will key on.
#[test]
fn the_milestone_create_door_mints_an_area_carrying_its_base_pin() {
    drive(DRIVEN_SITES[3]);
}

/// `jigc milestone add-task` — a `Snapshot::Exempt` row. The exemption is from the
/// carryover snapshot; the pin is not exempt, and a sub-task area that lacked one would be
/// answered *no such task* by its own `jigc start --task`.
#[test]
fn the_add_task_door_mints_an_area_carrying_its_base_pin() {
    drive(DRIVEN_SITES[4]);
}

/// The record-driven re-seed on a **real** fresh clone — the second `Snapshot::Exempt` row,
/// and the only mint whose areas nobody typed a command for.
#[test]
fn the_fresh_clone_reseed_rebuilds_areas_carrying_their_base_pin() {
    drive(DRIVEN_SITES[5]);
}

/// **The set is the registry's** — no undriven member, no cell for a door that is gone.
///
/// The per-row tests above name a site each; this asserts that naming is exhaustive in both
/// directions and that every member resolves to a cell (the hard panic in [`cell_for`]), so
/// a sixth mint door reddens here rather than joining the class unmeasured.
#[test]
fn every_mint_door_row_has_a_driven_pin_cell() {
    let declared: BTreeSet<&str> = MINT_DOORS.iter().map(|door| door.site).collect();
    assert_eq!(
        declared.len(),
        MINT_DOORS.len(),
        "each `MINT_DOORS` entry names its own call site",
    );
    let driven: BTreeSet<&str> = DRIVEN_SITES.iter().copied().collect();
    assert_eq!(
        driven.len(),
        DRIVEN_SITES.len(),
        "each per-row test drives its own site",
    );
    assert_eq!(
        driven, declared,
        "every `MINT_DOORS` row owes a driven pin cell, and a cell owes a live row — the \
         residual predicate answers `no task` over any area a door leaves pin-less",
    );
    for door in MINT_DOORS {
        // The hard panic is the assertion: a member the dispatch does not name aborts here.
        let _ = cell_for(door.site, door.door);
    }
    // Both dispositions are represented — the two `Exempt` rows are exactly the ones a
    // sweep keyed on "does this door snapshot?" would have skipped.
    assert!(
        MINT_DOORS
            .iter()
            .any(|door| matches!(door.snapshot, Snapshot::Exempt(_))),
        "the exempt rows are driven here too — the pin is not the snapshot",
    );
    assert!(
        MINT_DOORS
            .iter()
            .any(|door| matches!(door.snapshot, Snapshot::Written)),
        "…and so are the written ones",
    );
}

// ───────────────────────────── the shape leg, with its control ───────────────────────

/// **A directory wearing the pin's name is not the pin** — and the control proves the leg
/// is live rather than inert.
///
/// `engine::state::unwind_area` removes a registry member only when its shape matches
/// (`else if shape.is_file()`), so a *directory* named `base.json` is nobody's-of-jigc's: it
/// survives, the area's non-recursive `remove_dir` then refuses, and the caller is told
/// `AreaUnwind::Foreign`. The census read that branch and recorded it **unexercised**
/// (§2.1) — it is the reason D3's predicate is `symlink_metadata(…).is_file()` rather than
/// `.exists()`, because such a survivor would otherwise be a live task at every door,
/// forever.
///
/// The **control** is the same area one shape different: with a regular-file pin it unwinds
/// to `Removed`. Without it the subject arm would pass over an `unwind_area` that removed
/// nothing at all.
#[test]
fn a_directory_wearing_the_pins_name_survives_the_unwind_as_foreign() {
    let workdir = TempDir::new("shape");
    let home = TempDir::new("shape-home");
    let repo = init_repo(workdir.path());
    jigc_ok(&repo, home.path(), &["setup"]);

    // Two real, minted task areas — the subject and its control, from the same door.
    let subject = task_area(
        &repo,
        &minted_task(&jigc_ok(
            &repo,
            home.path(),
            &["start", "--workflow", "single-task", "the shape subject"],
        )),
    );
    let control = task_area(
        &repo,
        &minted_task(&jigc_ok(
            &repo,
            home.path(),
            &["start", "--workflow", "single-task", "the shape control"],
        )),
    );

    // The control: nothing but jigc's own writes, pin included, unwinds whole.
    assert_eq!(
        engine::state::unwind_area(&control, engine::state::WorkArea::Task)
            .expect("the control unwind reports"),
        engine::state::AreaUnwind::Removed,
        "the control area holds nothing but jigc's own writes and must unwind whole — \
         without this the subject arm below would pass over an unwind that removes nothing",
    );
    assert!(!control.exists(), "…and the control area is gone");

    // The subject: the pin's name, worn by a directory.
    let pin = subject.join(pin_name(engine::state::WorkArea::Task));
    fs::remove_file(&pin).expect("take the real pin away");
    fs::create_dir(&pin).expect("put a DIRECTORY at the pin's name");
    fs::write(pin.join("inside.txt"), "a third party's\n").expect("write inside it");

    assert_eq!(
        engine::state::unwind_area(&subject, engine::state::WorkArea::Task)
            .expect("the subject unwind reports"),
        engine::state::AreaUnwind::Foreign,
        "a DIRECTORY named `{}` is not something jigc wrote, so the unwind must leave it \
         and say so; the area held {:?}",
        pin_name(engine::state::WorkArea::Task),
        entry_names(&subject),
    );
    assert!(
        pin.is_dir(),
        "…and the directory itself is still there, with its contents",
    );
    assert!(
        pin.join("inside.txt").is_file(),
        "…including the bytes inside it — the survivor is a third party's, not a pin",
    );
    assert!(
        !fs::symlink_metadata(&pin)
            .expect("the survivor's shape reads")
            .is_file(),
        "…and the residual predicate must not read it as a pin: this is exactly the area \
         an `.exists()`-keyed predicate would call a live task forever",
    );
}
