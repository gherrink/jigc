//! M51 Increment 6 / T1 — **a well-formed work-unit id that names nothing answers the
//! findings envelope, at every task-family door.**
//!
//! ## The class this closes
//!
//! `design/command-output-contract.md` lists **`finalize.no-task`** under the work-unit
//! **target form** — i.e. as a finding that *projects a key*, `(code, target)` with
//! `target: "task:<id>"`. Driven at HEAD before this increment it projected none: every
//! one of the task-family doors answered `--format json` with the flattened single-key
//! `{"error": "no task `<id>` — list live tasks with `jigc task list`"}`, and a code
//! inside a message is not a key. A driver keying on `(code, target)` — the pair the
//! contract pins as a finding's stable identity — got an answer at **none** of them.
//!
//! ## The subject is a registry, filtered to one cell of its own axis
//!
//! [`WORK_UNIT_ID_DOORS`] is derived from the clap tree by the argument ids that carry a
//! work-unit id and fenced `⇔` against it
//! (`cli_parse::every_work_unit_id_door_is_registered`), so a door the binary grows
//! cannot ship without joining it. This suite takes the **task-family** rows — the ones
//! whose id arrives through `--task` / the positional `<id>` — and drives each with a
//! well-formed id no work unit carries. The **milestone** rows are excluded by
//! [`is_task_family`], and the exclusion names the task that removes it: M51 Increment 6
//! **T2** joins the eight milestone doors to the same answer, at which point the filter
//! goes and this suite drives all twenty-five.
//!
//! Its sibling `work_unit_id_axis.rs` keeps the other three cells of the same axis (the
//! malformed tokens) and the mint-side half; nothing here re-states them.
//!
//! ## What each row must answer
//!
//! The declared reject shape is read off the production registry rather than written
//! down: [`ENVELOPE_ARMS`]' cross-cutting `Reject::Findings` row names the top-level key
//! set, the stream (stderr, stdout empty) and the exit posture, and this suite asserts
//! the **driven** document against it. Then the key itself — `finalize.no-task` at
//! `task:<id>` — and the route floor: exactly one finding, carrying exactly one route.

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

/// Whether a [`WORK_UNIT_ID_DOORS`] row belongs to the **task** family — read off the
/// clap argument the id arrives through, never off the verb path.
///
/// **The milestone rows are excluded here and only here.** M51 Increment 6 / T2 joins the
/// eight `milestone_id` doors to this same answer; when it lands, this function and its
/// two call sites go, and the sweep below drives every registered row.
fn is_task_family(arg: &str) -> bool {
    match arg {
        "task" | "id" => true,
        "milestone_id" => false,
        other => panic!(
            "`{other}` is a work-unit-id argument with no family — a fourth member of \
             `work_unit_id_arg_ids()` owes this suite the answer its doors give"
        ),
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

/// **The sweep** — every task-family door, driven with a well-formed id that names
/// nothing, answers the findings envelope carrying the stable `(code, target)` pair.
#[test]
fn every_task_family_door_answers_the_findings_envelope_for_an_unknown_id() {
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
        if !is_task_family(row.arg) {
            continue;
        }
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
        assert_eq!(
            finding["key"],
            serde_json::json!({
                "code": NO_TASK_CODE,
                "target": format!("task:{UNKNOWN_ID}"),
            }),
            "{shown}: the stable key is `(finalize.no-task, task:<id>)` — the work-unit \
             target form the contract declares for this code; got:\n{stderr}",
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

    let registered = WORK_UNIT_ID_DOORS
        .iter()
        .filter(|row| is_task_family(row.arg))
        .count();
    assert_eq!(
        driven.len(),
        registered,
        "every registered task-family door is driven exactly once — a row with no cell is \
         a failure, never a skip",
    );
    assert!(
        registered >= 17,
        "the task-family slice of `WORK_UNIT_ID_DOORS` is {registered} doors — a sweep \
         over a shrunken registry would pass vacuously",
    );
}
