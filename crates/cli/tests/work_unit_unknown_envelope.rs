//! M51 Increment 6 / T1+T2 — **a well-formed work-unit id that names nothing answers the
//! findings envelope, at every door that takes one.**
//!
//! Cited as the `pinned-by:` for the rc.14 trial's
//! [findings-verification](../../../completions/artifacts/RC-rc14/findings-verification.md)
//! row **F-5** (*the unknown-id column: 22 doors refuse correctly, with a route, and no
//! code*), whose earlier `UNPINNED` reason — *"no arm asserts anything about the
//! unknown-id cell"* — is the sentence the sweep below falsifies (M51 Increment 11 / T3,
//! the conversion ledger).
//!
//! ## The class this closes
//!
//! `design/command-output-contract.md` declares a **work-unit** target form —
//! `task:<id>` / `milestone:<id>` — and lists **`finalize.no-task`** under it, i.e. as a
//! finding that *projects a key*. Driven at HEAD before this increment it projected none:
//! every one of the task-family doors answered `--format json` with the flattened
//! single-key `{"error": "no task `<id>` — list live tasks with `jigc task list`"}`, and a
//! code inside a message is not a key. The milestone family was the same defect one
//! family over, and **two wire shapes for one code** besides: five of its eight doors
//! answered the flattened `{"error": "milestone `<id>` does not exist…"}` with no code at
//! all, while the other three flattened a real `milestone.unknown` finding into the same
//! single key. A driver keying on `(code, target)` — the pair the contract pins as a
//! finding's stable identity — got an answer at **none** of the twenty-five.
//!
//! ## The subject is a registry, filtered to one cell of its own axis
//!
//! [`WORK_UNIT_ID_DOORS`] is derived from the clap tree by the argument ids that carry a
//! work-unit id and fenced `⇔` against it
//! (`cli_parse::every_work_unit_id_door_is_registered`), so a door the binary grows
//! cannot ship without joining it. This suite takes **every** row — the filter that held
//! the milestone rows out for the length of T1 is gone, and the sweep's closing assert is
//! a **set** equality against the registry, not a count — and drives each with a
//! well-formed id no work unit carries.
//!
//! Its sibling `work_unit_id_axis.rs` keeps the other three cells of the same axis (the
//! malformed tokens) and the mint-side half; nothing here re-states them.
//!
//! ## What each row must answer
//!
//! The declared reject shape is read off the production registry rather than written
//! down: [`ENVELOPE_ARMS`]' cross-cutting `Reject::Findings` row names the top-level key
//! set, the stream (stderr, stdout empty) and the exit posture, and this suite asserts
//! the **driven** document against it. Then the key itself — the pair the row's **family**
//! declares, `(finalize.no-task, task:<id>)` or
//! `(milestone.unknown, milestone:<id>)`, both read from the producing constructor's own
//! crate — and the route floor: exactly one finding, carrying exactly one route.
//!
//! One condition, one answer, and the **route** is still the door's: an absent milestone
//! routes to minting it everywhere except `discard`, where routing an operator at *create*
//! would answer a teardown with a mint. That split has its own arm below.
//!
//! ## The second absence shape — the **residual** cell (M53 Increment 3 / T3)
//!
//! `completions/artifacts/M53/settle-record.md` → D3 as amended by §9 and §14: *a task is
//! an area that carries its base pin.* A directory under `.jigc/tasks/` that holds no
//! `base.json` is a leftover — a bare `mkdir`, or the tail of a teardown that faulted —
//! and until this increment the by-id doors resolved it as a **live** task, because their
//! only question was `dir.is_dir()`. Driven at `766f32ef`: `jigc task validate <residual>`
//! answered *"no findings — the task validates clean"* at **exit 0**, `jigc task discard`
//! reached `task-discard.foreign-bytes` (a refusal *about bytes in a task*), `jigc doc show
//! … --task <residual>` answered `store.not-staged` (a statement *about a task's staged
//! set*), and `jigc start --task <residual>` fell through to a code-less
//! `could not read the base pin for task …`. Four doors, four different answers, none of
//! them the truth: there is no such task.
//!
//! So the absence the sweep above drives has a **second shape**, and T3 gives it the same
//! identity: the shipped `finalize.no-task`, key unchanged at `(finalize.no-task,
//! task:<id>)`, with the residual's own sentence and a `Human` route naming the
//! repo-relative path to clear by hand. The subject is the same registry, filtered to its
//! **task family** — the milestone rows resolve a milestone from its *committed record*,
//! not from a working area, so the milestone-area twin of this predicate is `jigc rename`'s
//! (T2) and not a cell of these doors.
//!
//! **The axis, one named test per cell** — `{empty dir · a foreign file · a foreign
//! docs/<ty>:<slug>.md} × {a plain id · a sub-task of an open milestone · a sub-task of a
//! joined milestone}` — each driven over every task-family row of the registry, in both the
//! printed and the machine arm, with the fixture's own host root asserted **absent** from
//! every emitted byte (`design/surface-contract.md` law 1).
//!
//! **`jigc task discard` is not an exception**, and that is the point of
//! [`the_discard_door_over_a_joined_milestones_residual_commits_nothing`]: a residual is a
//! task at *no* door, so the false-flip path D2.6 guards at the record is closed one layer
//! earlier, at resolution — and the record, the commit and the directory are all left
//! exactly as they were.

use std::collections::BTreeSet;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

use cli::cli::{WORK_UNIT_ID_DOOR_PAYLOAD, WORK_UNIT_ID_DOORS, WORK_UNIT_ID_SLOT};
use cli::render::{ArmOutcome, ArmShape, ENVELOPE_ARMS};

/// The well-formed id no work unit in the fixture carries.
const UNKNOWN_ID: &str = "no-such-work-unit";

/// The code the engine mints for a task working area that is not there
/// (`cli::render::FINALIZE_FAMILY`'s `finalize.no-task`, producer `engine::finalize`).
const NO_TASK_CODE: &str = "finalize.no-task";

/// The registry arm whose declaration these doors must satisfy.
const REJECT_ARM: &str = "Reject::Findings";

/// Which **family** a [`WORK_UNIT_ID_DOORS`] row's id belongs to — read off the clap
/// argument the id arrives through, never off the verb path, so a milestone verb that
/// grew a `--task` scope would be classified by what it actually takes.
#[derive(Clone, Copy, Debug)]
enum Family {
    /// The id names a task working area — `.jigc/tasks/<id>/`.
    Task,
    /// The id names a milestone workbench — `.jigc/milestones/<id>/`.
    Milestone,
}

fn family_of(arg: &str) -> Family {
    match arg {
        "task" | "id" => Family::Task,
        "milestone_id" => Family::Milestone,
        other => panic!(
            "`{other}` is a work-unit-id argument with no family — a fourth member of \
             `work_unit_id_arg_ids()` owes this suite the answer its doors give"
        ),
    }
}

impl Family {
    /// The stable `(code, target)` pair an absent work unit of this family projects — the
    /// **work-unit target form** `design/command-output-contract.md` declares, with the
    /// code taken from the crate that mints it rather than re-spelled here.
    fn key(self, id: &str) -> serde_json::Value {
        let (code, target) = match self {
            Family::Task => (NO_TASK_CODE, format!("task:{id}")),
            Family::Milestone => (
                engine::milestone::UNKNOWN_MILESTONE_CODE,
                format!("milestone:{id}"),
            ),
        };
        serde_json::json!({ "code": code, "target": target })
    }
}

/// A throwaway directory that removes itself on drop.
struct TempDir(PathBuf);

impl TempDir {
    fn new(tag: &str) -> Self {
        let mut path = std::env::temp_dir();
        path.push(format!(
            "jigc-unknown-envelope-{tag}-{}-{:?}",
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

/// A real git repo with one commit — the ground `jigc setup` installs into.
fn init_repo(root: &Path) {
    let git = |args: &[&str]| {
        let out = Command::new("git")
            .args(args)
            .current_dir(root)
            .output()
            .expect("run git");
        assert!(
            out.status.success(),
            "git {args:?} failed: {}",
            String::from_utf8_lossy(&out.stderr)
        );
    };
    git(&["init", "-q"]);
    git(&["config", "user.email", "test@example.com"]);
    git(&["config", "user.name", "Test"]);
    fs::write(root.join("README.md"), "hello\n").expect("write file");
    git(&["add", "."]);
    git(&["commit", "-q", "-m", "initial"]);
}

/// A set-up repo plus the payload file the two `--from-file` rows read — so those rows
/// fault on the **id** and never on a missing file.
struct Fixture {
    repo: TempDir,
    home: TempDir,
}

impl Fixture {
    fn new(tag: &str) -> Self {
        let repo = TempDir::new(&format!("repo-{tag}"));
        let home = TempDir::new(&format!("home-{tag}"));
        init_repo(repo.path());
        let fixture = Fixture { repo, home };
        fs::write(
            fixture.repo.path().join(WORK_UNIT_ID_DOOR_PAYLOAD),
            "title: Axis\nsections: []\n",
        )
        .expect("write payload");
        fixture.ok(&["setup"]);
        fixture
    }

    fn run(&self, args: &[&str]) -> std::process::Output {
        Command::new(env!("CARGO_BIN_EXE_jigc"))
            .args(args)
            .current_dir(self.repo.path())
            .env("HOME", self.home.path())
            .env_remove("JIGC_PACK_DIR")
            .output()
            .expect("spawn the jigc binary")
    }

    fn ok(&self, args: &[&str]) -> String {
        let out = self.run(args);
        assert!(
            out.status.success(),
            "`jigc {}` must succeed: {}",
            args.join(" "),
            String::from_utf8_lossy(&out.stderr),
        );
        String::from_utf8_lossy(&out.stdout).into_owned()
    }

    /// The fixture repo's `HEAD` sha — the observable a refused door must leave alone
    /// (M53 Increment 3 / T3: a residual reaches no record commit).
    fn git_head(&self) -> String {
        let out = Command::new("git")
            .args(["rev-parse", "HEAD"])
            .current_dir(self.repo.path())
            .output()
            .expect("run git rev-parse");
        assert!(out.status.success(), "git rev-parse HEAD must succeed");
        String::from_utf8_lossy(&out.stdout).trim().to_owned()
    }
}

/// The `Reject::Findings` row of the production registry — the declaration these doors
/// are asserted against, read rather than restated.
fn reject_findings_keys() -> Vec<&'static str> {
    let arm = ENVELOPE_ARMS
        .iter()
        .find(|arm| arm.path.is_empty() && arm.arm == REJECT_ARM)
        .expect("the cross-cutting `Reject::Findings` row is declared");
    assert!(
        matches!(arm.outcome, ArmOutcome::Reject),
        "`{REJECT_ARM}` is a reject arm — stdout empty, the document on stderr",
    );
    match arm.shape {
        ArmShape::Object(keys) => keys.to_vec(),
        _ => panic!("`{REJECT_ARM}` declares an object key set"),
    }
}

/// **The sweep** — every registered door, driven with a well-formed id that names
/// nothing, answers the findings envelope carrying its family's stable `(code, target)`
/// pair.
#[test]
fn every_work_unit_id_door_answers_the_findings_envelope_for_an_unknown_id() {
    let fixture = Fixture::new("sweep");
    // One live task, so no row's answer depends on active-task resolution.
    fixture.ok(&[
        "start",
        "--workflow",
        "single-task",
        "envelope intent",
        "--slug",
        "live",
    ]);

    let declared = reject_findings_keys();
    let mut driven: BTreeSet<Vec<&str>> = BTreeSet::new();

    for row in WORK_UNIT_ID_DOORS {
        let family = family_of(row.arg);
        let mut argv: Vec<&str> = row
            .argv
            .iter()
            .map(|arg| {
                if *arg == WORK_UNIT_ID_SLOT {
                    UNKNOWN_ID
                } else {
                    *arg
                }
            })
            .collect();
        argv.extend(["--format", "json"]);
        let shown = format!("jigc {} [{UNKNOWN_ID}] --format json", row.door.join(" "));

        let out = fixture.run(&argv);
        assert!(
            out.stdout.is_empty(),
            "{shown}: a reject leaves stdout empty; stdout:\n{}",
            String::from_utf8_lossy(&out.stdout),
        );
        assert_eq!(
            out.status.code(),
            Some(1),
            "{shown}: an unknown work-unit id rejects at exit 1; stderr:\n{}",
            String::from_utf8_lossy(&out.stderr),
        );

        let stderr = String::from_utf8(out.stderr).expect("utf-8 stderr");
        let value: serde_json::Value = serde_json::from_str(&stderr).unwrap_or_else(|err| {
            panic!("{shown}: stderr must parse as the reject envelope ({err}); got:\n{stderr}")
        });
        let object = value
            .as_object()
            .unwrap_or_else(|| panic!("{shown}: the envelope is a JSON object; got:\n{stderr}"));
        let mut keys: Vec<&str> = object.keys().map(String::as_str).collect();
        keys.sort_unstable();
        let mut want = declared.clone();
        want.sort_unstable();
        assert_eq!(
            keys, want,
            "{shown}: the envelope's top-level keys must be exactly what `ENVELOPE_ARMS`' \
             `{REJECT_ARM}` row declares — not the flattened `{{\"error\": …}}`, whose \
             code lives inside a message and projects no key; got:\n{stderr}",
        );

        let findings = value["findings"]
            .as_array()
            .unwrap_or_else(|| panic!("{shown}: `findings` is an array; got:\n{stderr}"));
        assert_eq!(
            findings.len(),
            1,
            "{shown}: one absent work unit is one finding; got:\n{stderr}",
        );
        let finding = &findings[0];
        assert!(
            !finding["key"].is_null(),
            "{shown}: a finding on the wire projects a key; got:\n{stderr}",
        );
        assert_eq!(
            finding["key"],
            family.key(UNKNOWN_ID),
            "{shown}: the stable key is this family's work-unit pair — `{family:?}`, the \
             target form the contract declares for an absent work unit; got:\n{stderr}",
        );
        let route = finding["route"].as_str().unwrap_or_else(|| {
            panic!("{shown}: a blocking finding carries a route; got:\n{stderr}")
        });
        assert_eq!(
            route.split('`').skip(1).step_by(2).count(),
            1,
            "{shown}: exactly one route — one backticked command to run; got `{route}`",
        );

        driven.insert(row.door.to_vec());
    }

    // **The set, not the count** — a door driven twice and one never reached is a count
    // that passes, and the registry is the subject, so the equality is against its own
    // rows.
    let registered: BTreeSet<Vec<&str>> = WORK_UNIT_ID_DOORS
        .iter()
        .map(|row| row.door.to_vec())
        .collect();
    assert_eq!(
        driven, registered,
        "every registered door is driven exactly once — a row with no cell is a failure, \
         never a skip",
    );
    assert!(
        registered.len() >= 25,
        "`WORK_UNIT_ID_DOORS` is {} doors — a sweep over a shrunken registry would pass \
         vacuously",
        registered.len(),
    );
}

/// **`discard`'s route is a teardown's, and stays one** — the one door of the milestone
/// family whose recovery is *not* minting the milestone.
///
/// Converging the eight doors on one condition and one answer is what T2 does; converging
/// their **routes** is what it must not do. `jigc milestone discard <id>` is a teardown,
/// and an operator who typed the wrong id there is asking to remove something — answering
/// that with *"create it first"* would route them at the one act they did not ask for, on
/// a door that has just told them nothing was discarded. So the shared identity carries
/// this door's own route, and this arm is what would notice if a later convergence took
/// it.
#[test]
fn the_discard_door_routes_at_the_roster_and_never_at_create() {
    let fixture = Fixture::new("discard-route");

    let out = fixture.run(&["milestone", "discard", UNKNOWN_ID, "--format", "json"]);
    let stderr = String::from_utf8(out.stderr).expect("utf-8 stderr");
    let value: serde_json::Value = serde_json::from_str(&stderr)
        .unwrap_or_else(|err| panic!("the reject envelope must parse ({err}); got:\n{stderr}"));
    let finding = &value["findings"][0];
    assert_eq!(
        finding["key"],
        Family::Milestone.key(UNKNOWN_ID),
        "`discard` carries the family's shared identity; got:\n{stderr}",
    );

    let route = finding["route"]
        .as_str()
        .unwrap_or_else(|| panic!("a blocking finding carries a route; got:\n{stderr}"));
    assert!(
        route.contains("`jigc milestone list-tasks <milestone-id>`"),
        "`discard`'s route names the roster a wrong id is checked against; got `{route}`",
    );
    assert!(
        !route.contains("jigc milestone create"),
        "a teardown door may not route at a mint — an operator who named the wrong \
         milestone to discard is not asking to create one; got `{route}`",
    );
    assert!(
        route.contains("nothing was discarded"),
        "`discard`'s route says what did NOT happen — the state truth this door owes; \
         got `{route}`",
    );
}

// ───────────────────────── the residual cell (M53 Increment 3 / T3) ─────────────────────

/// What a **residual** working area holds. The base pin is absent in all three — that is
/// what makes them residuals; the shapes differ in what *else* is in there, and they differ
/// in what the *destroying* doors think of the same directory, which is why one shape is not
/// the axis.
#[derive(Clone, Copy, Debug)]
enum Shape {
    /// Nothing at all — the bare `mkdir`, and the shape an interrupted teardown leaves.
    EmptyDir,
    /// One file jigc did not write, at the area root. Driven at `766f32ef` this shape is
    /// what took `jigc task discard` to `task-discard.foreign-bytes` — a refusal composed
    /// *about a task*, over a directory that is not one.
    ForeignFile,
    /// One file under `docs/` wearing a **staged-instance name** (`<type>:<slug>.md`): the
    /// only shape here that jigc's own writer could have produced, so
    /// `engine::state::foreign_area_paths` does not claim it and the pin is the only thing
    /// that tells it from a live task's staged area.
    ForeignStagedName,
}

impl Shape {
    /// Every shape. The cell tests below are named one per `(shape, placement)` pair; a
    /// fourth shape is a fourth row of named tests, never a silently unswept variant.
    const ALL: &'static [Shape] = &[
        Shape::EmptyDir,
        Shape::ForeignFile,
        Shape::ForeignStagedName,
    ];

    /// The shape's label, for a failure message that says which cell fell over.
    fn label(self) -> &'static str {
        match self {
            Shape::EmptyDir => "empty dir",
            Shape::ForeignFile => "a foreign file",
            Shape::ForeignStagedName => "a foreign docs/<ty>:<slug>.md",
        }
    }

    /// Plant this shape at `.jigc/tasks/<id>/`, replacing whatever is there.
    ///
    /// The area is **removed first**, so a placement that mints a real sub-task area and
    /// then residualizes it cannot leave the pin behind and pass by accident.
    fn plant(self, repo: &Path, id: &str) {
        let area = repo.join(".jigc").join("tasks").join(id);
        let _ = fs::remove_dir_all(&area);
        fs::create_dir_all(&area).expect("plant the residual area");
        match self {
            Shape::EmptyDir => {}
            Shape::ForeignFile => {
                fs::write(area.join("notes.txt"), "a third party's bytes\n")
                    .expect("plant the foreign file");
            }
            Shape::ForeignStagedName => {
                let docs = area.join("docs");
                fs::create_dir_all(&docs).expect("plant the residual docs/ tree");
                fs::write(docs.join("adr:leftover.md"), "# Leftover\n")
                    .expect("plant the staged-looking body");
            }
        }
        assert!(
            !area.join("base.json").exists(),
            "a planted residual must carry no base pin, or this suite proves nothing",
        );
    }
}

/// Where the residual's **id** comes from — the second axis. The id's provenance decides
/// what else in the repository has an opinion about it: a plain id is known to nothing, an
/// open milestone's record still lists it `active`, and a joined milestone's record has
/// already settled it.
#[derive(Clone, Copy, Debug)]
enum Placement {
    /// A directory under `.jigc/tasks/` belonging to no milestone — the cell one `mkdir`
    /// reaches on the shipped binary.
    PlainId,
    /// A sub-task of a milestone whose record is still `active`.
    OpenMilestone,
    /// A sub-task of a milestone whose boundary **landed**: the record is terminal, jigc
    /// itself removed the area, and a directory then reappeared at its id.
    JoinedMilestone,
}

/// The milestone the two milestone placements mint, and the id `jigc milestone create`
/// slugs that title to.
const MILESTONE_TITLE: &str = "Cache rework";
const MILESTONE_ID: &str = "cache-rework";

/// The id the plain placement plants at — well-formed, and named by nothing.
const PLAIN_RESIDUAL_ID: &str = "stray-leftover";

impl Placement {
    /// Every placement — the second factor of the shape space
    /// [`the_named_cells_are_the_whole_residual_shape_space`] fences the named tests
    /// against.
    const ALL: &'static [Placement] = &[
        Placement::PlainId,
        Placement::OpenMilestone,
        Placement::JoinedMilestone,
    ];

    fn label(self) -> &'static str {
        match self {
            Placement::PlainId => "a plain id",
            Placement::OpenMilestone => "a sub-task of an open milestone",
            Placement::JoinedMilestone => "a sub-task of a joined milestone",
        }
    }

    /// Build a fixture holding one residual of `shape` in this placement, and hand back the
    /// id the by-id doors will be asked about.
    fn build(self, shape: Shape, tag: &str) -> (Fixture, String) {
        let fixture = Fixture::new(tag);
        let id = match self {
            Placement::PlainId => PLAIN_RESIDUAL_ID.to_string(),
            Placement::OpenMilestone => {
                fixture.ok(&["milestone", "create", MILESTONE_TITLE]);
                add_sub_task(&fixture, "Shard the index")
            }
            Placement::JoinedMilestone => {
                fixture.ok(&["milestone", "create", MILESTONE_TITLE]);
                // One sub-task contributes a promotable doc, so the boundary is not a
                // `milestone.zero-contribution` refusal; the other is the one this cell
                // residualizes.
                let contributor = add_sub_task(&fixture, "Warm the read cache");
                let settled = add_sub_task(&fixture, "Shard the index");
                stage_a_promotable_adr(&fixture, &contributor);
                fixture.ok(&["milestone", "finalize", MILESTONE_ID]);
                let record = fixture.ok(&["doc", "show", "milestone-record:cache-rework"]);
                assert!(
                    record.contains("status: joined"),
                    "the milestone must really be joined, or this placement is the open \
                     one again; got:\n{record}",
                );
                assert!(
                    !task_area(&fixture, &settled).exists(),
                    "the boundary removes its sub-task areas — the residual below must be \
                     planted, not left",
                );
                settled
            }
        };
        shape.plant(fixture.repo.path(), &id);
        (fixture, id)
    }
}

/// `.jigc/tasks/<id>` in the fixture's repo.
fn task_area(fixture: &Fixture, id: &str) -> PathBuf {
    fixture.repo.path().join(".jigc").join("tasks").join(id)
}

/// `jigc milestone add-task <milestone> "<intent>"`, returning the **minted** sub-task id
/// read off the ack rather than re-slugged in test code.
fn add_sub_task(fixture: &Fixture, intent: &str) -> String {
    let ack = fixture.ok(&["milestone", "add-task", MILESTONE_ID, intent]);
    let (_, rest) = ack
        .split_once("added task:")
        .unwrap_or_else(|| panic!("`milestone add-task` names the task it minted; got:\n{ack}"));
    rest.split_whitespace()
        .next()
        .expect("the ack's task id is one token")
        .to_string()
}

/// Stage one promotable `adr` in `sub_id`'s working area, the way a fanned-out sub-agent
/// leaves it — the body plus its provenance bit — so the milestone boundary has something
/// to commit.
fn stage_a_promotable_adr(fixture: &Fixture, sub_id: &str) {
    let docs = task_area(fixture, sub_id).join("docs");
    fs::create_dir_all(&docs).expect("mk the contributing sub-task's docs/");
    fs::write(
        docs.join("adr:warm-policy.md"),
        "---\nstatus: accepted\ndate: 2026-06-04\n---\n\n# Warm policy\n\n## Context\n\n\
         Forces.\n\n## Options\n\nAlternatives.\n\n## Decision\n\nDo the thing.\n\n\
         ## Consequences\n\nTradeoffs.\n",
    )
    .expect("stage the contributed doc");
    fs::write(
        docs.join("provenance.json"),
        "{\"docs\":{\"adr:warm-policy\":\"created\"}}",
    )
    .expect("stage the provenance bit");
}

/// The **task-family** rows of the registry — the doors whose id names a task working
/// area, and therefore the doors the shared resolve predicate moves.
///
/// Read off [`family_of`], never off the verb path, so a milestone verb that grew a
/// `--task` scope would join this sweep by being what it is.
fn task_family_rows() -> Vec<&'static cli::cli::WorkUnitIdDoor> {
    WORK_UNIT_ID_DOORS
        .iter()
        .filter(|row| matches!(family_of(row.arg), Family::Task))
        .collect()
}

/// Every task-family door, asked about `id` whose area is a residual, must answer the one
/// identity: the findings envelope carrying exactly one finding keyed
/// `(finalize.no-task, task:<id>)`, exactly one route, the residual's own sentence naming
/// the repo-relative area — and **no host-absolute path** on any stream, in either arm.
fn assert_every_task_door_answers_the_residual(fixture: &Fixture, id: &str, cell: &str) {
    let declared = reject_findings_keys();
    let listed = format!(".jigc/tasks/{id}");
    // Both spellings of the fixture's own root: the cwd the binary was handed, and what it
    // canonicalizes to (on macOS `/var/folders/…` → `/private/var/folders/…`). Law 1's rule
    // is that neither reaches a surface.
    let host_roots: Vec<String> = {
        let raw = fixture.repo.path().to_path_buf();
        let mut roots = vec![raw.to_string_lossy().into_owned()];
        if let Ok(real) = raw.canonicalize() {
            let real = real.to_string_lossy().into_owned();
            if !roots.contains(&real) {
                roots.push(real);
            }
        }
        roots
    };

    let mut driven: BTreeSet<Vec<&str>> = BTreeSet::new();
    let rows = task_family_rows();
    assert!(
        !rows.is_empty(),
        "the task family of `WORK_UNIT_ID_DOORS` is non-empty — a sweep over none of it \
         would pass vacuously",
    );

    for row in &rows {
        let base: Vec<&str> = row
            .argv
            .iter()
            .map(|arg| if *arg == WORK_UNIT_ID_SLOT { id } else { *arg })
            .collect();

        // ── the printed arm ──
        let shown = format!("[{cell}] jigc {} [{id}]", row.door.join(" "));
        let out = fixture.run(&base);
        let printed = format!(
            "{}{}",
            String::from_utf8_lossy(&out.stdout),
            String::from_utf8_lossy(&out.stderr),
        );
        assert_eq!(
            out.status.code(),
            Some(1),
            "{shown}: a residual is an absent work unit — it rejects at exit 1, never \
             validates clean; got:\n{printed}",
        );
        assert!(
            printed.contains("· finalize.no-task — "),
            "{shown}: the printed refusal carries the shipped absent-work-unit code; \
             got:\n{printed}",
        );
        assert!(
            printed.contains(&format!("`{listed}`")),
            "{shown}: the refusal names the leftover directory the operator has to clear; \
             got:\n{printed}",
        );
        for root in &host_roots {
            assert!(
                !printed.contains(root.as_str()),
                "{shown}: law 1 — no host-absolute path on any stream; `{root}` \
                 appears in:\n{printed}",
            );
        }

        // ── the machine arm ──
        let mut argv = base.clone();
        argv.extend(["--format", "json"]);
        let shown = format!("{shown} --format json");
        let out = fixture.run(&argv);
        assert!(
            out.stdout.is_empty(),
            "{shown}: a reject leaves stdout empty; stdout:\n{}",
            String::from_utf8_lossy(&out.stdout),
        );
        assert_eq!(
            out.status.code(),
            Some(1),
            "{shown}: a residual rejects at exit 1; stderr:\n{}",
            String::from_utf8_lossy(&out.stderr),
        );

        let stderr = String::from_utf8(out.stderr).expect("utf-8 stderr");
        for root in &host_roots {
            assert!(
                !stderr.contains(root.as_str()),
                "{shown}: law 1 — no host-absolute path on the machine arm either; \
                 `{root}` appears in:\n{stderr}",
            );
        }
        let value: serde_json::Value = serde_json::from_str(&stderr).unwrap_or_else(|err| {
            panic!("{shown}: stderr must parse as the reject envelope ({err}); got:\n{stderr}")
        });
        let object = value
            .as_object()
            .unwrap_or_else(|| panic!("{shown}: the envelope is a JSON object; got:\n{stderr}"));
        let mut keys: Vec<&str> = object.keys().map(String::as_str).collect();
        keys.sort_unstable();
        let mut want = declared.clone();
        want.sort_unstable();
        assert_eq!(
            keys, want,
            "{shown}: the envelope's top-level keys are exactly what `ENVELOPE_ARMS`' \
             `{REJECT_ARM}` row declares; got:\n{stderr}",
        );

        let findings = value["findings"]
            .as_array()
            .unwrap_or_else(|| panic!("{shown}: `findings` is an array; got:\n{stderr}"));
        assert_eq!(
            findings.len(),
            1,
            "{shown}: a residual is one condition and one finding — not a task's \
             foreign-bytes refusal, not its staged-set report; got:\n{stderr}",
        );
        let finding = &findings[0];
        assert_eq!(
            finding["key"],
            Family::Task.key(id),
            "{shown}: the key is unchanged — the residual is the same absence, so it \
             carries the same stable pair; got:\n{stderr}",
        );
        let message = finding["message"]
            .as_str()
            .unwrap_or_else(|| panic!("{shown}: a finding carries a message; got:\n{stderr}"));
        assert!(
            message.contains(&format!("`{listed}`")),
            "{shown}: the residual sentence names the repo-relative directory; got \
             `{message}`",
        );
        let route = finding["route"].as_str().unwrap_or_else(|| {
            panic!("{shown}: a blocking finding carries a route; got:\n{stderr}")
        });
        assert_eq!(
            route.split('`').skip(1).step_by(2).count(),
            1,
            "{shown}: exactly one route, naming exactly one thing — the path to clear; \
             got `{route}`",
        );
        assert!(
            route.contains(&listed),
            "{shown}: the route names the path by hand, because jigc mints no verb that \
             clears it; got `{route}`",
        );

        driven.insert(row.door.to_vec());
    }

    let registered: BTreeSet<Vec<&str>> = rows.iter().map(|row| row.door.to_vec()).collect();
    assert_eq!(
        driven, registered,
        "[{cell}] every task-family row is driven exactly once — a row with no cell is a \
         failure, never a skip",
    );
}

/// Drive one `(shape, placement)` cell of the residual axis.
fn drive_cell(shape: Shape, placement: Placement, tag: &str) {
    let (fixture, id) = placement.build(shape, tag);
    let cell = format!("{} · {}", placement.label(), shape.label());
    assert_every_task_door_answers_the_residual(&fixture, &id, &cell);
    assert!(
        task_area(&fixture, &id).is_dir(),
        "[{cell}] a refusal destroys nothing — the leftover is still on disk",
    );
}

/// **The nine named cells, and the registry they generate.**
///
/// Each arm expands to one `#[test]` naming its cell, and the same list expands to
/// [`AXIS`] — so the registry the fence below compares against the shape space is built
/// **from the tests themselves**. A cell that has no named test cannot appear in `AXIS`,
/// and a cell that is not in the shape space cannot appear in either: the two directions
/// of *"a named test per cell"* are checked rather than counted.
macro_rules! residual_cells {
    ($($name:ident => ($shape:expr, $placement:expr, $tag:literal);)+) => {
        $(
            #[test]
            fn $name() {
                drive_cell($shape, $placement, $tag);
            }
        )+

        /// The `(shape, placement)` pairs the named tests above drive — generated from
        /// that list, never written beside it.
        const AXIS: &[(Shape, Placement)] = &[$(($shape, $placement)),+];
    };
}

residual_cells! {
    an_empty_residual_at_a_plain_id_is_a_task_at_no_by_id_door
        => (Shape::EmptyDir, Placement::PlainId, "res-plain-empty");
    a_foreign_file_residual_at_a_plain_id_is_a_task_at_no_by_id_door
        => (Shape::ForeignFile, Placement::PlainId, "res-plain-foreign");
    a_staged_named_residual_at_a_plain_id_is_a_task_at_no_by_id_door
        => (Shape::ForeignStagedName, Placement::PlainId, "res-plain-staged");
    an_empty_residual_of_an_open_milestone_is_a_task_at_no_by_id_door
        => (Shape::EmptyDir, Placement::OpenMilestone, "res-open-empty");
    a_foreign_file_residual_of_an_open_milestone_is_a_task_at_no_by_id_door
        => (Shape::ForeignFile, Placement::OpenMilestone, "res-open-foreign");
    a_staged_named_residual_of_an_open_milestone_is_a_task_at_no_by_id_door
        => (Shape::ForeignStagedName, Placement::OpenMilestone, "res-open-staged");
    an_empty_residual_of_a_joined_milestone_is_a_task_at_no_by_id_door
        => (Shape::EmptyDir, Placement::JoinedMilestone, "res-joined-empty");
    a_foreign_file_residual_of_a_joined_milestone_is_a_task_at_no_by_id_door
        => (Shape::ForeignFile, Placement::JoinedMilestone, "res-joined-foreign");
    a_staged_named_residual_of_a_joined_milestone_is_a_task_at_no_by_id_door
        => (Shape::ForeignStagedName, Placement::JoinedMilestone, "res-joined-staged");
}

/// **The axis is the whole shape space** — `Shape::ALL × Placement::ALL`, with no cell
/// driven twice.
///
/// This is what makes the nine tests above an *axis* rather than nine instances: a fourth
/// residual shape, or a fourth placement, reddens here until it has its own named cell
/// (`implementation/pinning.md` → a fix is complete over its class's axis).
#[test]
fn the_named_cells_are_the_whole_residual_shape_space() {
    let named: BTreeSet<(&str, &str)> = AXIS
        .iter()
        .map(|(shape, placement)| (shape.label(), placement.label()))
        .collect();
    let space: BTreeSet<(&str, &str)> = Shape::ALL
        .iter()
        .flat_map(|shape| {
            Placement::ALL
                .iter()
                .map(move |placement| (shape.label(), placement.label()))
        })
        .collect();
    assert_eq!(
        named, space,
        "every cell of `{{shape}} × {{placement}}` has a named test, and every named test \
         names a cell of it",
    );
    assert_eq!(
        AXIS.len(),
        Shape::ALL.len() * Placement::ALL.len(),
        "the set equality above would also pass with a cell driven twice and another \
         missing — the count is what rules that out",
    );
}

/// **The named cell** (`settle-record.md` → D3, *"`jigc task discard` is not an
/// exception"*; D2.6 and §8): a bare `mkdir .jigc/tasks/<sub>` under a **joined** milestone,
/// then `jigc task discard <sub>`.
///
/// Driven at `766f32ef` this door reached the record — before M53 Increment 2's settled-item
/// guard it committed a false `- status: discarded` over an item the record had already
/// settled, and with that guard it refused at the record with `milestone.terminal`. Either
/// way the door had already accepted the directory **as a task** and gone looking for its
/// item. After T3 it never gets there: the resolve seam answers first, so the false-flip
/// path is closed one layer earlier than the guard that watches it.
///
/// What this arm asserts is the *absence of an act*, which is the only part a message
/// cannot tell you: `HEAD` unmoved, the record's item byte-identical, and the directory
/// still on disk.
#[test]
fn the_discard_door_over_a_joined_milestones_residual_commits_nothing() {
    let (fixture, id) = Placement::JoinedMilestone.build(Shape::EmptyDir, "res-discard");

    let head_before = fixture.git_head();
    let record_before = fixture.ok(&["doc", "show", "milestone-record:cache-rework"]);

    let out = fixture.run(&["task", "discard", &id]);
    let stderr = String::from_utf8_lossy(&out.stderr).into_owned();
    assert_eq!(
        out.status.code(),
        Some(1),
        "a residual is not a task, so there is nothing to discard; got:\n{stderr}",
    );
    assert!(
        stderr.contains("· finalize.no-task — "),
        "the teardown door answers the same identity as the other task-family doors — a \
         residual is a task at NO door; got:\n{stderr}",
    );
    assert!(
        !stderr.contains("milestone.terminal"),
        "the record guard is the layer behind this one: after T3 the door never accepts \
         the directory as a task, so it never reaches the item; got:\n{stderr}",
    );
    assert!(
        !stderr.contains("task-discard."),
        "and it never composes a refusal ABOUT a task's bytes over a directory that is \
         not a task; got:\n{stderr}",
    );

    assert_eq!(
        fixture.git_head(),
        head_before,
        "no record commit — a refused discard moves HEAD not at all",
    );
    assert_eq!(
        fixture.ok(&["doc", "show", "milestone-record:cache-rework"]),
        record_before,
        "the record's item is byte-identical: the sub-task stays `joined`, never flipped \
         to `discarded`",
    );
    assert!(
        task_area(&fixture, &id).is_dir(),
        "and the leftover is still on disk — the operator clears it, jigc does not",
    );
}
