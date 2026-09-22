//! M53 Increment 3 / T4 — **the mint names the leftover instead of claiming a live work
//! unit** (`completions/artifacts/M53/settle-record.md` → D3, *What the mint answers*;
//! `implementation/roadmap.md` → M53 Increment 3).
//!
//! **The two lies this suite retires**, both driven at `bde8a643` over a bare `mkdir` (the
//! debug binary, through `dev/jigc-rig`):
//!
//! - `jigc start --slug stray-alpha "<intent>"` over `.jigc/tasks/stray-alpha/` answered
//!   *"task `stray-alpha` is already active"* and routed at `jigc start --task stray-alpha`
//!   or `jigc task discard stray-alpha --force`. T3 made **both of those arms** answer
//!   `finalize.no-task` — *"no task `stray-alpha`"* — so the door stated a falsehood and
//!   then sent the agent two hops to discover it.
//! - `jigc milestone create "Stray mile"` over `.jigc/milestones/stray-mile/` answered
//!   *"milestone `stray-mile` already exists"* and routed at
//!   `jigc milestone add-task stray-mile "<intent>"`, which answered **`milestone.area-io`**
//!   — *"a disk or permissions problem"* — for a state that is neither. A lie whose route
//!   dead-ended in a second lie. (The route's end is closed too, one commit later: the eight
//!   milestone by-id doors resolve through `engine::milestone::require_milestone_area`, so
//!   `add-task` now answers the residual with `milestone.unknown` and the leftover's own
//!   route — `work_unit_unknown_envelope.rs`' milestone cells.)
//!
//! **What does not move, and why it is asserted rather than assumed.** The code and the
//! stable `(code, target)` key stay put at both doors: the state really is *the id is taken
//! on disk*, and a driver keying on `task.serial-collision` / `milestone.serial-collision`
//! must not have to learn a second pair to see the residual cell. Only the **sentence** and
//! the **route** change — and the route is the one the by-id seams already hand out, from
//! the single home `engine::state` owns, because there is exactly one recovery for a
//! leftover directory.
//!
//! **The over-firing control is half the suite.** Every cell below has a live twin: a
//! colliding mint over a real, jigc-minted area must still answer today's bytes, unchanged.
//! Without that, a predicate that answered *residual* over live work would ship green.

use crate::support;

use std::fs;
use std::path::Path;
use support::trial_corpus::{State, TrialCorpus};

// ───────────────────────────── the three residual shapes ─────────────────────────────

/// What a residual directory holds — D3's axis, the same three shapes
/// `residual_area_roster.rs` iterates at the *enumerating* doors.
///
/// The pin is absent in all three; they differ in what else is in there, which is what the
/// **destroying** doors key on. The mint keys on the pin alone, so all three must answer
/// identically here — and that identity is the point: a mint that started discriminating on
/// the contents would be deciding whose bytes those are, which is not the mint's question.
#[derive(Clone, Copy, Debug)]
enum Shape {
    /// Nothing at all — the bare `mkdir`, and the shape an interrupted teardown leaves.
    EmptyDir,
    /// One file jigc did not write, at the area root — the destroying doors' subject.
    ForeignFile,
    /// One file under `docs/` wearing a **staged-instance name** (`<type>:<slug>.md`): the
    /// only shape jigc's own writer could have produced, so the foreign-bytes probe does
    /// not claim it and the pin is all that distinguishes it from a live task's staged area.
    ForeignStagedName,
}

impl Shape {
    /// Every shape. A sweep that iterates this picks up a fourth with no edit.
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

    /// Plant this shape at `area`, replacing whatever is there. Works for either area
    /// family — the `docs/` sub-tree is a task shape, but a directory with a foreign file
    /// in it is the same residual under `.jigc/milestones/` as under `.jigc/tasks/`.
    fn plant(self, area: &Path) {
        let _ = fs::remove_dir_all(area);
        fs::create_dir_all(area).expect("plant the residual area");
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

// ───────────────────────────── shared assertions ─────────────────────────────

/// Both streams of a refusal, joined — the refusal's printed surface, whichever stream
/// carries it.
fn refusal(corpus: &TrialCorpus, args: &[&str]) -> String {
    let out = corpus.jigc(args);
    assert!(
        !out.status.success(),
        "`jigc {}` must refuse; got exit {:?}\nstdout:\n{}\nstderr:\n{}",
        args.join(" "),
        out.status.code(),
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr),
    );
    format!(
        "{}{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr),
    )
}

/// Everything a residual refusal owes, at either mint door: the shipped code, the residual
/// sentence, the repo-relative path, no host-absolute byte anywhere, and each dead-end
/// route asserted gone **by its argv**.
fn assert_residual_refusal(
    corpus: &TrialCorpus,
    text: &str,
    code: &str,
    listed: &str,
    dead_argv: &[&str],
    cell: &str,
) {
    assert!(
        text.contains(code),
        "[{cell}] the code and the key do not move — the id really is taken on disk; got:\n{text}",
    );
    assert!(
        text.contains("carrying no base pin"),
        "[{cell}] the refusal says what the directory is; got:\n{text}",
    );
    assert!(
        text.contains(listed),
        "[{cell}] and names `{listed}`, so the agent can go look; got:\n{text}",
    );
    for dead in dead_argv {
        assert!(
            !text.contains(dead),
            "[{cell}] `{dead}` is the dead end this cell retires — it must not come back \
             wearing new prose; got:\n{text}",
        );
    }
    // Law 1: the printed path is repo-relative. Both spellings of the fixture's own root,
    // because a canonicalized temp root differs from the one the harness handed us.
    let repo = corpus.repo();
    for root in [repo.clone(), repo.canonicalize().unwrap_or(repo)] {
        let host = root.to_string_lossy().into_owned();
        assert!(
            !text.contains(&host),
            "[{cell}] no host-absolute path reaches the surface; `{host}` in:\n{text}",
        );
    }
}

// ───────────────────────────── the task mint ─────────────────────────────

/// **A same-slug mint over a task residual, in each of the three shapes.**
///
/// The cell is driven through `jigc start --slug <id>` because that is the door whose id is
/// caller-supplied, so the collision is exact rather than a guess about what a given intent
/// slugs to — and it is the same `engine::state::mint_task` guard every other task-minting
/// form reaches.
#[test]
fn a_same_slug_mint_over_a_task_residual_names_the_leftover() {
    let corpus = TrialCorpus::build(State::CommittedSingletons);
    let area = corpus
        .repo()
        .join(".jigc")
        .join("tasks")
        .join("stray-alpha");

    for shape in Shape::ALL {
        shape.plant(&area);
        let before = sorted_entries(&area);

        let text = refusal(
            &corpus,
            &[
                "start",
                "--workflow",
                "quick-fix",
                "--slug",
                "stray-alpha",
                "Rework the cache",
            ],
        );

        assert_residual_refusal(
            &corpus,
            &text,
            "task.serial-collision",
            ".jigc/tasks/stray-alpha",
            &[
                "jigc start --task stray-alpha",
                "jigc task discard stray-alpha --force",
            ],
            shape.label(),
        );
        assert!(
            !text.contains("is already active"),
            "[{}] a directory with no pin is a task at no door, so it is not active; got:\n{text}",
            shape.label(),
        );
        assert_eq!(
            sorted_entries(&area),
            before,
            "[{}] the refusal precedes every write — nothing was changed",
            shape.label(),
        );
        assert!(
            !area.join("base.json").exists(),
            "[{}] and above all no pin was written into somebody else's directory",
            shape.label(),
        );
    }
}

/// The **over-firing control** for the task door: a colliding mint over a live task still
/// answers today's bytes, resume route and all.
///
/// This is what makes the change above a narrowing rather than a rewrite. The route it
/// asserts is the one the residual cell must *not* carry, so the two tests are each other's
/// converse.
#[test]
fn a_same_slug_mint_over_a_live_task_answers_the_shipped_collision() {
    let corpus = TrialCorpus::build(State::CommittedSingletons);
    corpus.jigc_ok(&[
        "start",
        "--workflow",
        "quick-fix",
        "--slug",
        "live-alpha",
        "Rework the cache",
    ]);
    assert!(
        corpus
            .repo()
            .join(".jigc")
            .join("tasks")
            .join("live-alpha")
            .join("base.json")
            .is_file(),
        "the control's area must be a real one, pin and all",
    );

    let text = refusal(
        &corpus,
        &[
            "start",
            "--workflow",
            "quick-fix",
            "--slug",
            "live-alpha",
            "Rework the cache again",
        ],
    );

    assert!(
        text.contains("task `live-alpha` is already active"),
        "the live cell's sentence is untouched; got:\n{text}",
    );
    assert!(
        text.contains("resume with `jigc start --task live-alpha`")
            && text.contains("`jigc task discard live-alpha --force`"),
        "and so is its route — over live work the resume it names really does resume; \
         got:\n{text}",
    );
    assert!(
        !text.contains("carrying no base pin"),
        "the residual sentence must not fire over a live area; got:\n{text}",
    );
}

// ───────────────────────────── the milestone mint ─────────────────────────────

/// **`jigc milestone create` over a record-less milestone residual.**
///
/// *Record-less* is what makes this the mint's cell rather than
/// [`crate::milestone_record_create`]'s: the committed record owns a milestone's identity,
/// so `run_create` refuses at `guard_record_free` first when one exists. Here no record
/// does — a directory appeared at the id, by a faulted teardown or by hand — and the refusal
/// is the workbench's.
#[test]
fn a_milestone_mint_over_a_record_less_residual_names_the_leftover() {
    let corpus = TrialCorpus::build(State::CommittedSingletons);
    let area = corpus
        .repo()
        .join(".jigc")
        .join("milestones")
        .join("stray-mile");
    let head_before = corpus.git(&["rev-parse", "HEAD"]);

    for shape in Shape::ALL {
        shape.plant(&area);

        let text = refusal(&corpus, &["milestone", "create", "Stray mile"]);

        assert_residual_refusal(
            &corpus,
            &text,
            "milestone.serial-collision",
            ".jigc/milestones/stray-mile",
            &["jigc milestone add-task stray-mile"],
            shape.label(),
        );
        assert!(
            !text.contains("already exists"),
            "[{}] a directory with no pin is a milestone at no door; got:\n{text}",
            shape.label(),
        );
        assert!(
            !area.join("base.json").exists() && !area.join("tasks.json").exists(),
            "[{}] nothing of jigc's landed in that directory",
            shape.label(),
        );
    }

    assert_eq!(
        corpus.git(&["rev-parse", "HEAD"]),
        head_before,
        "a refused create lands no record commit",
    );
}

/// The **over-firing control** for the milestone door.
///
/// Reaching a *live* `milestone.serial-collision` needs an area that carries its pin at an
/// id the committed record does not own — `guard_record_free` refuses first otherwise, one
/// door up. So the control copies an area jigc's own mint just wrote, which is the state a
/// faulted record commit leaves behind (the workbench half survives, the record half does
/// not — `crates/cli/tests/milestone_record_rollback.rs` drives that arc).
#[test]
fn a_milestone_mint_over_a_live_area_answers_the_shipped_collision() {
    let corpus = TrialCorpus::build(State::CommittedSingletons);
    corpus.jigc_ok(&["milestone", "create", "Cache rework"]);

    let minted = corpus
        .repo()
        .join(".jigc")
        .join("milestones")
        .join("cache-rework");
    let live = corpus
        .repo()
        .join(".jigc")
        .join("milestones")
        .join("stray-mile");
    fs::create_dir_all(&live).expect("open the control's area");
    for name in ["base.json", "tasks.json"] {
        fs::copy(minted.join(name), live.join(name)).expect("copy the minted member");
    }
    assert!(
        live.join("base.json").is_file(),
        "the control's area carries its pin, which is what makes it live",
    );

    let text = refusal(&corpus, &["milestone", "create", "Stray mile"]);

    assert!(
        text.contains("milestone `stray-mile` already exists"),
        "the live cell's sentence is untouched; got:\n{text}",
    );
    assert!(
        text.contains("`jigc milestone add-task stray-mile \"<intent>\"`"),
        "and so is its route — over a live area the add-task it names really does work; \
         got:\n{text}",
    );
    assert!(
        !text.contains("carrying no base pin"),
        "the residual sentence must not fire over a live area; got:\n{text}",
    );
}

// ───────────────────────────── helpers ─────────────────────────────

/// Every entry under `dir`, sorted — the "nothing was changed" witness.
fn sorted_entries(dir: &Path) -> Vec<String> {
    let mut names: Vec<String> = fs::read_dir(dir)
        .expect("read the area")
        .filter_map(|entry| entry.ok()?.file_name().into_string().ok())
        .collect();
    names.sort();
    names
}
