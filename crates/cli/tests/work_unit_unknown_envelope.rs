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
