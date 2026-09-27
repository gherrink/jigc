//! M50 Increment 1 / T2 — **the work-unit-id axis, every door the clap tree has × the
//! whole token space.**
//!
//! ## The class this closes
//!
//! `.jigc/tasks/<id>/` and `.jigc/milestones/<id>/` are built by joining a
//! caller-supplied token onto a path. Until M50 no door asked whether that token was an
//! id, and driven at `caa137e~`, `jigc task discard "../.."` resolved the **repository
//! root** as a working area and removed it — `.git`, `.jigc`, every tracked file — at
//! **exit 0** (`DECISIONS.md` → 2026-09-05; the trial finding in
//! `completions/artifacts/RC-m50/findings-verification.md`).
//!
//! T1 shipped the guard at the five **resolve seams**. A seam is not a door: the seams
//! were counted by reading the source, and *"a malformed work-unit id is refused at every
//! door"* is a claim about the doors. Four waves running, a fix applied where its wave
//! pointed rather than over its class's axis has been this repo's most expensive shape
//! (M49's own centrepiece; `implementation/pinning.md`). So this suite's subject is
//! [`WORK_UNIT_ID_DOORS`] — **derived from the clap tree** by the argument ids that carry
//! a work-unit id ([`work_unit_id_arg_ids`]) and fenced ⇔ against it by
//! `cli_parse::every_work_unit_id_door_is_registered`, the shipped `DOCTYPE_DOORS` mold.
//! Twenty-five doors, and a door the binary grows cannot ship without joining them.
//!
//! ## The four cells, and why the fourth is not decoration
//!
//! Three **malformed** tokens — `""`, the traversal `../..`, and an **absolute path that
//! really exists** (the fixture's own repository root, the exact value whose resolution
//! was the data loss) — each of which must block with `work-unit.malformed-id`, name the
//! token the caller typed, and carry exactly one route.
//!
//! The fourth is a **well-formed but unknown** id, and it is the cell that catches the
//! guard over-firing: a grammar refusal handed to someone who merely mistyped a live id
//! would be a law-1 misdirection, and it is the failure this guard is one line away from
//! at all times. Its answer is asserted **unchanged**: every task door names the id as
//! *no task*, every milestone door as *does not exist*, and the route points at a verb
//! that either **reads** the roster or **mints** a work unit of that family — both sides
//! read off shipped registries (`VERB_KINDS`, `engine::state::MINT_DOORS`) rather than a
//! remembered list of acceptable strings.
//!
//! The **fifth** is a well-formed id whose working area — `.jigc/tasks/<id>/` or
//! `.jigc/milestones/<id>/` — is a **residual**, a directory carrying no base pin (M53
//! Increment 3). It is the same absence as the fourth and carries the same code **of its
//! own family**, and it is here rather than only in `work_unit_unknown_envelope.rs`'s named
//! cells because it is the cell most likely to be lost to the *other* guard: a leftover id
//! is well-formed, so a grammar refusal over it would be the same law-1 misdirection one
//! state along. It ran task-family-only for one commit, on the premise that a milestone id
//! resolves from its committed record and not from a working area — driven false by one
//! `mkdir`, with the datum on [`Expect::Residual`].
//!
//! **A row with no cell is a hard panic, never a skip**: the cells are generated for
//! every registered row from its own family, the driven set is compared back to the
//! registry, the per-row width is asserted against that family, and the count is asserted,
//! so a door cannot join the registry and quietly run nothing.
//!
//! ## The mint-side half
//!
//! The guard's one dangerous failure mode is not over-firing on a typo but **stranding a
//! live work unit**: if any production mint could produce an id `engine::slug::is_slug`
//! rejects, every door would refuse the unit that jigc itself created. That is a claim
//! about the mints, so it is fenced against the mints — [`MINT_DOORS`], driven with
//! deliberately hostile input, each member dispatched by name with a hard panic for a
//! member this suite does not drive.

use std::collections::BTreeSet;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

use cli::cli::{
    VERB_KINDS, VerbKind, WORK_UNIT_ID_DOOR_PAYLOAD, WORK_UNIT_ID_DOORS, WORK_UNIT_ID_SLOT,
    work_unit_id_arg_ids,
};
use cli::render::ROUTING_FOOTER;
use engine::milestone::UNKNOWN_MILESTONE_CODE;
use engine::slug::is_slug;
use engine::state::MINT_DOORS;

/// The blocking finding a malformed work-unit id earns at every door (T1).
const MALFORMED_CODE: &str = "work-unit.malformed-id";

/// The well-formed id no work unit in the fixture carries — the fourth cell's token.
const UNKNOWN_ID: &str = "no-such-work-unit";

/// The code the engine mints for a task working area that is not a task — absent, or
/// present-but-pin-less (`cli::render::FINALIZE_FAMILY`'s `finalize.no-task`).
const NO_TASK_CODE: &str = "finalize.no-task";

/// The well-formed id whose `.jigc/tasks/` directory is a **residual** — the task family's
/// fifth-cell token, planted as a bare `mkdir` by the sweep.
const RESIDUAL_TASK: &str = "leftover-area";

/// The same, one family over: the id whose `.jigc/milestones/` directory is a residual that
/// **no committed record names**, which is what makes it a leftover rather than a cache the
/// fresh-clone re-seed would rebuild.
const RESIDUAL_MILESTONE: &str = "leftover-mile";

/// The fixture's one open task, so no cell depends on active-task resolution.
const LIVE_TASK: &str = "axis";

/// A throwaway directory that removes itself on drop.
struct TempDir(PathBuf);

impl TempDir {
    fn new(tag: &str) -> Self {
        let mut path = std::env::temp_dir();
        path.push(format!(
            "jigc-work-unit-id-axis-{tag}-{}-{:?}",
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

/// A set-up repo with one open task and the payload file the two `--from-file` rows read.
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

    fn path(&self) -> &Path {
        self.repo.path()
    }
}

/// What a cell's token is, and therefore which answer the door owes.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Expect {
    /// The token is not an id at all — the grammar refusal.
    Malformed,
    /// The token is a well-formed id naming nothing — the roster answer, unchanged.
    Unknown,
    /// The token is a well-formed id whose working area is a **residual**: a directory
    /// carrying no base pin (M53 Increment 3 / T3). The same absence as [`Expect::Unknown`]
    /// and therefore the same code and key — with the leftover's own sentence, and a route
    /// that names a **path**, because jigc mints no verb that clears one.
    ///
    /// **Both families**, each under its own code — `finalize.no-task` at a task door,
    /// `milestone.unknown` at a milestone door. The first pass held the milestone rows out
    /// of this cell on the written premise that *"a milestone door resolves its unit from
    /// the committed record, not from a working area"*; the premise is false and one
    /// `mkdir` falsifies it. Driven at `3f22150b`, a bare `mkdir .jigc/milestones/<id>`
    /// turned the family's keyed `(milestone.unknown, milestone:<id>)` into five code-less
    /// `could not read the task list …` errors, two **false** `milestone.area-io` refusals
    /// and one answer about a spec — while the identical id with *no* directory answered
    /// correctly at all eight. What is genuinely record-shaped is the opposite case: a
    /// pin-less area a record **does** name is a cache the fresh-clone re-seed rebuilds, and
    /// that control lives in `work_unit_unknown_envelope.rs`.
    Residual,
}

/// Which family a door's id belongs to, **derived** from the clap argument the id arrives
/// through rather than declared a second time.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Family {
    Task,
    Milestone,
}

fn family_of(arg: &str) -> Family {
    match arg {
        "id" | "task" => Family::Task,
        "milestone_id" => Family::Milestone,
        other => panic!(
            "`{other}` is a work-unit-id argument with no family — a fourth member of \
             work_unit_id_arg_ids() owes this suite the answer its doors give"
        ),
    }
}

/// The backticked spans of a surface that name a `jigc` command — the routes a reader
/// would paste.
///
/// **The universal routing footer is not a route and is dropped first** (M51 Increment 6 /
/// T1). `cli::render::ROUTING_FOOTER` ends every composed reading surface with *"run
/// `jigc start` for orientation; all writes through `jigc`"* — two backticked `jigc`
/// spans that are an orientation reminder, not this refusal's recovery. They arrived here
/// when the unknown-id refusal became a `Finding` rendered through the house findings
/// funnel, and counting them would read one route as three. The footer is dropped by its
/// own production constant, never by a copy of its text.
fn jigc_commands(text: &str) -> Vec<String> {
    text.replace(ROUTING_FOOTER, "")
        .split('`')
        .skip(1)
        .step_by(2)
        .filter(|span| *span == "jigc" || span.starts_with("jigc "))
        .map(str::to_owned)
        .collect()
}

/// The leaf verb path of a route command (`jigc milestone list-tasks <milestone-id>` →
/// `["milestone", "list-tasks"]`) — the longest [`VERB_KINDS`] prefix of its tokens.
fn leaf_verb(command: &str) -> Vec<String> {
    let tokens: Vec<&str> = command.split_whitespace().skip(1).collect();
    VERB_KINDS
        .iter()
        .map(|(path, _)| *path)
        .filter(|path| path.len() <= tokens.len() && path.iter().zip(&tokens).all(|(a, b)| a == b))
        .max_by_key(|path| path.len())
        .unwrap_or_else(|| panic!("`{command}` names no leaf verb of the clap tree"))
        .iter()
        .map(|token| (*token).to_string())
        .collect()
}

/// The leaf verbs an operator reaches a **work-unit mint** by, read off
/// [`MINT_DOORS`] — the half of the fourth cell's route rule that is not *read the
/// roster*. Members whose `door` is a prose description of an internal re-seed rather
/// than an operator-typed argv contribute nothing, which is the honest reading: they are
/// not doors anyone is routed to.
fn mint_leaf_verbs() -> BTreeSet<Vec<String>> {
    MINT_DOORS
        .iter()
        .filter(|door| door.door.starts_with("jigc "))
        .map(|door| leaf_verb(door.door))
        .collect()
}

/// **The axis** — every registered door, over its family's whole token cell space.
///
/// Each malformed cell must block with [`MALFORMED_CODE`], name the token as typed, and
/// carry exactly one route; the unknown cell must give the family's unchanged roster
/// answer and must **not** carry the grammar refusal. After every cell the fixture's tree
/// is re-asserted, because at `caa137e~` the *text* of one of these cells was already
/// correct over a repository that had just been deleted.
///
/// **The fifth cell is every row's** ([`Expect::Residual`], M53 Increment 3): a *well-formed
/// id whose working area is a leftover directory*. Its **token** is built per row from the
/// row's family — each family has its own area root and its own planted leftover — and the
/// per-row length is asserted, so a row cannot silently run four cells where five are owed.
#[test]
fn every_work_unit_id_door_answers_the_whole_token_axis() {
    let fixture = Fixture::new("axis");
    fixture.ok(&[
        "start",
        "--workflow",
        "single-task",
        "axis intent",
        "--slug",
        LIVE_TASK,
    ]);
    // The leftovers the fifth cell asks about: a bare `mkdir` under each family's area
    // root, the state one command reaches on the shipped binary.
    for (area, id) in [("tasks", RESIDUAL_TASK), ("milestones", RESIDUAL_MILESTONE)] {
        fs::create_dir_all(fixture.path().join(".jigc").join(area).join(id))
            .expect("plant the residual working area");
    }
    let absolute = fixture.path().to_string_lossy().into_owned();
    let mints = mint_leaf_verbs();

    let shared: Vec<(String, Expect)> = vec![
        (String::new(), Expect::Malformed),
        ("../..".to_string(), Expect::Malformed),
        (absolute, Expect::Malformed),
        (UNKNOWN_ID.to_string(), Expect::Unknown),
    ];

    let mut driven: BTreeSet<Vec<&str>> = BTreeSet::new();
    for row in WORK_UNIT_ID_DOORS {
        let family = family_of(row.arg);
        let mut cells = shared.clone();
        cells.push((
            match family {
                Family::Task => RESIDUAL_TASK.to_string(),
                Family::Milestone => RESIDUAL_MILESTONE.to_string(),
            },
            Expect::Residual,
        ));
        assert_eq!(
            cells.len(),
            5,
            "`jigc {}`: every row is driven over its family's whole cell space",
            row.door.join(" "),
        );
        for (token, expect) in &cells {
            let argv: Vec<&str> = row
                .argv
                .iter()
                .map(|arg| {
                    if *arg == WORK_UNIT_ID_SLOT {
                        token.as_str()
                    } else {
                        *arg
                    }
                })
                .collect();
            let shown = format!("jigc {} [{token}]", row.door.join(" "));
            let out = fixture.run(&argv);
            assert!(
                !out.status.success(),
                "{shown}: an id that names no live work unit must block non-zero",
            );
            let stderr = String::from_utf8_lossy(&out.stderr).into_owned();
            let routes = jigc_commands(&stderr);

            match expect {
                Expect::Malformed => {
                    assert!(
                        stderr.contains(&format!("· {MALFORMED_CODE} — ")),
                        "{shown}: must carry the `{MALFORMED_CODE}` finding code\n{stderr}",
                    );
                    assert!(
                        stderr.contains(&format!("{token:?}")),
                        "{shown}: must name the token the caller typed\n{stderr}",
                    );
                    assert_eq!(
                        stderr.matches("route:").count(),
                        1,
                        "{shown}: must carry exactly one route\n{stderr}",
                    );
                }
                Expect::Unknown => {
                    assert!(
                        !stderr.contains(MALFORMED_CODE),
                        "{shown}: a well-formed id names a work unit that is merely \
                         absent — the grammar refusal here is a law-1 misdirection\
                         \n{stderr}",
                    );
                    let clause = match family {
                        Family::Task => format!("no task `{token}`"),
                        Family::Milestone => format!("milestone `{token}` does not exist"),
                    };
                    assert!(
                        stderr.contains(&clause),
                        "{shown}: must carry the family's identity clause `{clause}`\n{stderr}",
                    );
                    assert_eq!(
                        routes.len(),
                        1,
                        "{shown}: must name exactly one runnable jigc command\n{stderr}",
                    );
                    let verb = leaf_verb(&routes[0]);
                    let kind = VERB_KINDS
                        .iter()
                        .find(|(path, _)| path.iter().eq(verb.iter()))
                        .map(|(_, kind)| *kind)
                        .expect("the route's leaf verb is classified");
                    assert!(
                        kind == VerbKind::Read || mints.contains(&verb),
                        "{shown}: the route must point at a verb that reads the roster or \
                         mints one of this family — `jigc {}` does neither\n{stderr}",
                        verb.join(" "),
                    );
                }
                Expect::Residual => {
                    let (code, area) = match family {
                        Family::Task => (NO_TASK_CODE, "tasks"),
                        Family::Milestone => (UNKNOWN_MILESTONE_CODE, "milestones"),
                    };
                    assert!(
                        !stderr.contains(MALFORMED_CODE),
                        "{shown}: a leftover directory's id is a well-formed id — the \
                         grammar refusal here is a law-1 misdirection\n{stderr}",
                    );
                    assert!(
                        stderr.contains(&format!("· {code} — ")),
                        "{shown}: a residual is the same absence as an unknown id, so it \
                         carries the same code\n{stderr}",
                    );
                    assert!(
                        stderr.contains(&format!("`.jigc/{area}/{token}`")),
                        "{shown}: the leftover's sentence names the repo-relative \
                         directory the operator has to clear\n{stderr}",
                    );
                    assert_eq!(
                        stderr.matches("route:").count(),
                        1,
                        "{shown}: must carry exactly one route\n{stderr}",
                    );
                    assert!(
                        routes.is_empty(),
                        "{shown}: jigc mints no verb that clears a leftover working area, \
                         so the route names a path and no command — a `jigc` span here \
                         would be a command that deletes someone else's bytes\n{stderr}",
                    );
                    assert!(
                        fixture.path().join(".jigc").join(area).join(token).is_dir(),
                        "{shown}: a refusal destroys nothing — the leftover is still on \
                         disk",
                    );
                }
            }

            // The tree, not only the text: nothing a refused id names may be touched.
            assert!(
                fixture.path().join(".git").is_dir(),
                "{shown}: the repository must survive a refused id",
            );
            assert!(
                fixture.path().join("README.md").is_file(),
                "{shown}: the tracked tree must survive a refused id",
            );
            assert!(
                fixture
                    .path()
                    .join(".jigc")
                    .join("tasks")
                    .join(LIVE_TASK)
                    .is_dir(),
                "{shown}: the live task's working area must survive a refused id",
            );
        }
        driven.insert(row.door.to_vec());
    }

    assert_eq!(
        driven.len(),
        WORK_UNIT_ID_DOORS.len(),
        "every registered door is driven exactly once — a row with no cell is a failure, \
         never a skip",
    );
    assert!(
        !work_unit_id_arg_ids().is_empty(),
        "the derivation vocabulary must be non-empty, else the ⇔ fence is vacuous",
    );
}

// ─────────────────────────── the mint-side half ───────────────────────────

/// Hostile input for a door that slugifies: non-ASCII, punctuation, doubled separators
/// and a trailing stopword — every shape that could leave a leading, trailing or doubled
/// hyphen behind.
const HOSTILE: &str = "  ÄÖÜ Straße —— re-DO the *WHOLE* thing!!  ";

/// A foreign source whose **path** is the hostile part: `jigc migrate` passes its id to
/// the mint through the `slug_override` bypass, so it is the one door where a token that
/// is not `is_slug` would reach disk unnormalized.
const HOSTILE_SOURCE_DIR: &str = "Docs Folder";
const HOSTILE_SOURCE_FILE: &str = "Odd Name (v2).md";

/// The **degenerate** half of the mint axis (M53 Increment 5 / T3): titles that carry
/// nothing an id can be built from, so `engine::slug::slugify` folds each to the empty
/// string and the mint's empty→type-name fallback would hand the work unit an identity
/// nobody typed.
///
/// **The cells are ordinary, not adversarial.** Only the first three read as abuse; the
/// last two are a title in another script and a title of stopwords alone, and both are
/// things a person types on purpose. That is why the refusal's sentence has to be true of
/// all five, and why an assertion on it is not decoration.
const UNSLUGABLE_TITLES: &[&str] = &["", "   ", "!!!", "日本語", "the of a"];

/// The mint class's refusal, raised through `engine::state::reject_unslugable_title` from
/// the one producer private to that module (M53 Increment 5).
const UNSLUGABLE_CODE: &str = "write.unslugable-title";

/// The half of that refusal's sentence that is **about the caller's title rather than
/// this door**, and therefore the half a per-door wording would quietly drop.
const UNSLUGABLE_SENTENCE: &str = "ids are built from ASCII letters and digits, so a \
     title in another script, or of stopwords only, yields none";

/// Where a [`MINT_DOORS`] row's id comes from — the axis the degenerate cell iterates.
///
/// It is **not** a second remembered list: every row is dispatched by `site` below with a
/// hard panic for an unclassified member, so a new mint door has to answer this question
/// before it can ship, exactly as it already has to answer `Snapshot`.
enum IdSource {
    /// The id is slugged from the caller's prose, so this row owes the degenerate cell:
    /// the door refuses `write.unslugable-title` **before any write**.
    Prose,
    /// The id is derived from something that is not a title, with the reason stated on
    /// the registry's own `Snapshot::Exempt` mold. There is no degenerate *title* to feed
    /// such a door; what it owes instead is that its own id still mints, which the cell
    /// drives rather than asserting.
    Exempt(&'static str),
}

/// Every working-area name currently on disk, task areas and milestone areas alike.
fn area_names(repo: &Path) -> Vec<(String, String)> {
    let mut out = Vec::new();
    for (kind, dir) in [
        ("task", repo.join(".jigc").join("tasks")),
        ("milestone", repo.join(".jigc").join("milestones")),
    ] {
        let Ok(entries) = fs::read_dir(&dir) else {
            continue;
        };
        for entry in entries.flatten() {
            if entry.path().is_dir() {
                out.push((
                    kind.to_string(),
                    entry.file_name().to_string_lossy().into_owned(),
                ));
            }
        }
    }
    out.sort();
    out
}

/// Run `git <args>` in `repo`, asserting it succeeded.
fn git_ok(repo: &Path, args: &[&str]) {
    let out = Command::new("git")
        .args(args)
        .current_dir(repo)
        .output()
        .expect("run git");
    assert!(
        out.status.success(),
        "git {args:?} failed: {}",
        String::from_utf8_lossy(&out.stderr),
    );
}

/// The commit a door that refuses before any write must not have moved.
fn head_sha(repo: &Path) -> String {
    let out = Command::new("git")
        .args(["rev-parse", "HEAD"])
        .current_dir(repo)
        .output()
        .expect("run git rev-parse");
    assert!(out.status.success(), "`git rev-parse HEAD` must succeed");
    String::from_utf8_lossy(&out.stdout).trim().to_string()
}

/// `git status --porcelain` — for the degenerate cells, empty *is* the assertion.
fn porcelain(repo: &Path) -> String {
    let out = Command::new("git")
        .args(["status", "--porcelain"])
        .current_dir(repo)
        .output()
        .expect("run git status");
    assert!(
        out.status.success(),
        "`git status --porcelain` must succeed"
    );
    String::from_utf8_lossy(&out.stdout).into_owned()
}

/// A fixture standing on a **clean** tree, so *"the refused door left `git status` clean"*
/// is a statement about the door rather than about the fixture.
///
/// `jigc setup` already writes the `compose-embedded-methodology` marker, so this project
/// is `[dev ▸ methodology]` and the milestone doors do materialize a committed record and
/// land record-only commits — without which *"landed no record commit"* would pass because
/// there is no record home to land in.
fn clean_fixture(tag: &str) -> Fixture {
    let fixture = Fixture::new(tag);
    git_ok(fixture.path(), &["add", "-A"]);
    git_ok(
        fixture.path(),
        &["commit", "-q", "-m", "the fixture's own bytes"],
    );
    assert!(
        porcelain(fixture.path()).is_empty(),
        "the degenerate cells start from a clean tree",
    );
    fixture
}

/// Drive one prose [`MINT_DOORS`] row over the **whole** degenerate title set and assert
/// the refusal is *before any write* (M53 Increment 5 / T3, D5 as amended by §12).
///
/// Three things are checked per title, and the last two are the ones a guard placed after
/// the door's first write would fail: the surface carries the class's identity **and** the
/// half of its sentence that is about the caller's title rather than this door; and HEAD,
/// the working-area set and the working tree are exactly what they were.
fn refuses_before_any_write(fixture: &Fixture, site: &str, argv: &dyn Fn(&str) -> Vec<String>) {
    for title in UNSLUGABLE_TITLES {
        let before_head = head_sha(fixture.path());
        let before_areas = area_names(fixture.path());
        let owned = argv(title);
        let args: Vec<&str> = owned.iter().map(String::as_str).collect();
        let out = fixture.run(&args);
        let surface = format!(
            "{}{}",
            String::from_utf8_lossy(&out.stdout),
            String::from_utf8_lossy(&out.stderr),
        );
        assert!(
            !out.status.success(),
            "`{site}` must refuse the title {title:?} — a title that slugs to nothing would \
             otherwise mint the work unit at a fabricated id\n{surface}",
        );
        assert!(
            surface.contains(UNSLUGABLE_CODE),
            "`{site}` must refuse {title:?} under `{UNSLUGABLE_CODE}`\n{surface}",
        );
        assert!(
            surface.contains(UNSLUGABLE_SENTENCE),
            "`{site}` owes the class's sentence, which is true for a title in any script — \
             a per-door wording is how {title:?} gets told it has no letters\n{surface}",
        );
        assert_eq!(
            head_sha(fixture.path()),
            before_head,
            "`{site}` moved HEAD while refusing {title:?} — the refusal must precede every \
             write, the record commit included\n{surface}",
        );
        assert_eq!(
            area_names(fixture.path()),
            before_areas,
            "`{site}` left a working area behind while refusing {title:?}\n{surface}",
        );
        assert_eq!(
            porcelain(fixture.path()),
            "",
            "`{site}` left the working tree dirty while refusing {title:?}\n{surface}",
        );
    }
}

/// Plant the foreign source `jigc migrate` adopts, committed so it is not a candidate
/// fault of its own.
fn plant_hostile_source(fixture: &Fixture) -> String {
    let dir = fixture.path().join(HOSTILE_SOURCE_DIR);
    fs::create_dir_all(&dir).expect("create the foreign directory");
    fs::write(
        dir.join(HOSTILE_SOURCE_FILE),
        "# Change Log\n\n## v1\n\n- did a thing\n",
    )
    .expect("write the foreign source");
    for args in [
        vec!["add", "-A"],
        vec![
            "-c",
            "user.email=t@e",
            "-c",
            "user.name=T",
            "commit",
            "-qm",
            "foreign",
        ],
    ] {
        let out = Command::new("git")
            .args(&args)
            .current_dir(fixture.path())
            .output()
            .expect("run git");
        assert!(out.status.success(), "git {args:?} failed");
    }
    format!("{HOSTILE_SOURCE_DIR}/{HOSTILE_SOURCE_FILE}")
}

/// A milestone with one sub-task, both minted through hostile input.
fn hostile_milestone(fixture: &Fixture) -> String {
    let out = fixture.ok(&["milestone", "create", HOSTILE]);
    let id = out
        .lines()
        .find_map(|line| line.split_once("milestone minted: "))
        .map(|(_, id)| id.trim().to_string())
        .unwrap_or_else(|| {
            area_names(fixture.path())
                .into_iter()
                .find(|(kind, _)| kind == "milestone")
                .map(|(_, id)| id)
                .expect("`jigc milestone create` leaves a milestone area")
        });
    fixture.ok(&["milestone", "add-task", &id, HOSTILE]);
    id
}

/// **The mint-side fence** — every id a production mint door produces is a well-formed
/// work-unit id, and therefore survives the guard T1 put at every door.
///
/// This is the one claim whose falsity would **strand a live work unit**: an id the mint
/// wrote and the doors refuse is a work unit nobody can resume, validate, finalize or
/// discard. So it is driven, not argued — [`MINT_DOORS`] is dispatched by `site` with a
/// hard panic for an undriven member, each door is fed deliberately hostile input, and
/// every area name it leaves on disk is checked **twice**: against
/// [`engine::slug::is_slug`], and by feeding the id back through a real door and
/// requiring the answer not to be the grammar refusal.
///
/// **The degenerate half** (M53 Increment 5 / T3): the same registry, crossed with the
/// titles that yield *no* id at all. The hostile half proves a door's id survives the
/// grammar; this half proves the door never invents one. Driven at `92ed1957~`,
/// `jigc milestone create "日本語"` minted `milestone:milestone` and **committed a record**
/// for it at exit 0, and `jigc milestone add-task <m> "日本語"` committed `task:task` — an
/// identity nobody typed, that the next such call then serial-collides. Each of the three
/// rows whose id is slugged from prose now refuses `write.unslugable-title` before any
/// write; the two rows that derive their id from something else are `Exempt(reason)` on
/// the registry's own mold, and each **drives** its stated reason rather than asserting it.
#[test]
fn every_mint_door_produces_an_id_every_door_accepts() {
    for (index, door) in MINT_DOORS.iter().enumerate() {
        let fixture = Fixture::new(&format!("mint{index}"));
        match door.site {
            "crates/cli/src/start.rs::mint_in_repo" => {
                fixture.ok(&["start", "--workflow", "single-task", HOSTILE]);
            }
            "crates/cli/src/start.rs::mint_migration_in_repo" => {
                let source = plant_hostile_source(&fixture);
                fixture.ok(&["migrate", &source, "--as", "changelog"]);
            }
            // `jigc task amend`'s intent is optional, so its id has **two** sources and the
            // hostile half has to reach the one the grammar can break: a present intent
            // slugs like every other prose door's, and an absent one takes the
            // `amend-<sha7>` fallback, which is jigc's own bytes and cannot be hostile.
            // Driven on the prose source, which is the cell this fence exists for.
            "crates/cli/src/start.rs::mint_amend_in_repo" => {
                fixture.ok(&["task", "amend", HOSTILE]);
            }
            "crates/cli/src/milestone.rs::run_create" => {
                fixture.ok(&["milestone", "create", HOSTILE]);
            }
            "crates/engine/src/milestone.rs::add_task" => {
                hostile_milestone(&fixture);
            }
            "crates/engine/src/milestone.rs::reseed_sub_task_areas" => {
                let milestone = hostile_milestone(&fixture);
                // The fresh clone: the gitignored workbench is gone, the committed
                // record remains, and the next operating verb rebuilds the areas from it.
                fs::remove_dir_all(fixture.path().join(".jigc").join("tasks"))
                    .expect("drop the task areas");
                fs::remove_dir_all(fixture.path().join(".jigc").join("milestones"))
                    .expect("drop the milestone cache");
                fixture.ok(&["milestone", "add-task", &milestone, "a second sub-task"]);
            }
            other => panic!(
                "`MINT_DOORS` member `{other}` ({}) is not driven here — a new mint door \
                 owes this fence a cell, or the guard's mint-side safety is unproven for \
                 it",
                door.door,
            ),
        }

        let areas = area_names(fixture.path());
        assert!(
            !areas.is_empty(),
            "`{}` must leave a working area to check",
            door.site,
        );
        for (kind, id) in areas {
            assert!(
                is_slug(&id),
                "`{}` minted `{id}`, which `is_slug` rejects — every door would refuse \
                 the work unit jigc itself created",
                door.site,
            );
            let probe: Vec<&str> = match kind.as_str() {
                "task" => vec!["task", "validate", &id],
                _ => vec!["milestone", "list-tasks", &id],
            };
            let out = fixture.run(&probe);
            let surface = format!(
                "{}{}",
                String::from_utf8_lossy(&out.stdout),
                String::from_utf8_lossy(&out.stderr),
            );
            assert!(
                !surface.contains(MALFORMED_CODE),
                "`{}` minted `{id}`, which `jigc {}` then refuses as malformed\n{surface}",
                door.site,
                probe.join(" "),
            );
        }
    }

    // ───────── the degenerate half: a title that yields no id (T3) ─────────

    let mut prose = 0usize;
    let mut exempt = 0usize;
    for (index, door) in MINT_DOORS.iter().enumerate() {
        let source = match door.site {
            "crates/cli/src/start.rs::mint_in_repo" => {
                let fixture = clean_fixture(&format!("degen{index}"));
                refuses_before_any_write(&fixture, door.site, &|title| {
                    vec![
                        "start".into(),
                        "--workflow".into(),
                        "single-task".into(),
                        title.into(),
                    ]
                });
                IdSource::Prose
            }
            "crates/cli/src/milestone.rs::run_create" => {
                let fixture = clean_fixture(&format!("degen{index}"));
                refuses_before_any_write(&fixture, door.site, &|title| {
                    vec!["milestone".into(), "create".into(), title.into()]
                });
                IdSource::Prose
            }
            "crates/engine/src/milestone.rs::add_task" => {
                let fixture = clean_fixture(&format!("degen{index}"));
                fixture.ok(&["milestone", "create", "Cache rework"]);
                refuses_before_any_write(&fixture, door.site, &|title| {
                    vec![
                        "milestone".into(),
                        "add-task".into(),
                        "cache-rework".into(),
                        title.into(),
                    ]
                });
                IdSource::Prose
            }
            // `jigc task amend` is **prose**, and the reason it is not exempt is the whole
            // point of the cell: its intent is *optional*, so it looks like a door that
            // always has a fallback id to fall back on. It does not. The fallback is keyed
            // on the intent being **absent**, which is what keeps a caller who typed a
            // degenerate title from silently getting a task named after the commit instead.
            // Driven at this fence's sibling (`flow54_acceptance`'s arm 5) with the branch
            // keyed on emptiness rather than `Option`: `jigc task amend "   "` minted
            // `amend-<sha7>` at exit 0.
            "crates/cli/src/start.rs::mint_amend_in_repo" => {
                let fixture = clean_fixture(&format!("degen{index}"));
                refuses_before_any_write(&fixture, door.site, &|title| {
                    vec!["task".into(), "amend".into(), title.into()]
                });
                IdSource::Prose
            }
            "crates/cli/src/start.rs::mint_migration_in_repo" => {
                // The reason, driven: a source path whose every character slugs away is
                // exactly the input that would break a title-slugging door, and this door
                // mints from it anyway — because the id is the path's hash, not its prose
                // (`start.rs::migration_task_id`, whose `slug_override` bypasses the mint's
                // re-slugify entirely).
                let fixture = clean_fixture(&format!("degen{index}"));
                fs::write(
                    fixture.path().join("日本語.md"),
                    "# Change Log\n\n## v1\n\n- did a thing\n",
                )
                .expect("write the unslugable source path");
                git_ok(fixture.path(), &["add", "-A"]);
                git_ok(fixture.path(), &["commit", "-q", "-m", "foreign source"]);
                fixture.ok(&["migrate", "日本語.md", "--as", "changelog"]);
                let minted: Vec<String> = area_names(fixture.path())
                    .into_iter()
                    .filter(|(kind, _)| kind == "task")
                    .map(|(_, id)| id)
                    .collect();
                assert_eq!(
                    minted.len(),
                    1,
                    "`{}` must mint exactly one task from an unslugable source path",
                    door.site,
                );
                assert!(
                    minted[0].starts_with("migrate-changelog-") && is_slug(&minted[0]),
                    "`{}` minted `{}` — the id is the doctype plus the source path's hash, \
                     which is why no title guard applies to this row",
                    door.site,
                    minted[0],
                );
                IdSource::Exempt(
                    "the id is `migrate-<doctype>-<blake3(source path)>`, fed to the mint \
                     verbatim as `slug_override` — this door is handed a path, never a \
                     title, and slugs no prose at all",
                )
            }
            "crates/engine/src/milestone.rs::reseed_sub_task_areas" => {
                // The reason, driven over the id that makes it load-bearing: the milestone
                // is seeded with the title `Task`, whose legitimate slug is `task` — the
                // **exact** id the fallback fabricates, so the record now reads as a
                // pre-guard corpus does at the leaf this door actually consults. The
                // workbench is then wiped and a door that re-seeds is run: `task` comes
                // back, because `reseed_sub_task_areas` hands `mint_task` the record's own
                // `item.id` as `slug_override` and asks no title at all.
                //
                // Driven and not pinned: the *intent*-side twin of this corpus cannot be
                // manufactured through any door. Hand-editing a committed record's `intent`
                // leaf and re-running is refused by `reconciliation.conflict-block` — a
                // machine-maintained record is never merged — so the honest subject here is
                // the recorded **id**, which is the leaf this door reads.
                let fixture = clean_fixture(&format!("degen{index}"));
                fixture.ok(&["milestone", "create", "Cache rework"]);
                fixture.ok(&["milestone", "add-task", "cache-rework", "Task"]);
                assert!(
                    area_names(fixture.path()).contains(&("task".into(), "task".into())),
                    "the fixture's premise: the recorded sub-task id is `task`, the id the \
                     fallback would have fabricated",
                );
                // The fresh clone: the gitignored workbench is gone, the committed
                // record remains, and the next operating verb rebuilds the areas from it.
                fs::remove_dir_all(fixture.path().join(".jigc").join("tasks"))
                    .expect("drop the task areas");
                fs::remove_dir_all(fixture.path().join(".jigc").join("milestones"))
                    .expect("drop the milestone cache");
                fixture.ok(&["milestone", "add-task", "cache-rework", "a second sub-task"]);
                assert!(
                    area_names(fixture.path()).contains(&("task".into(), "task".into())),
                    "`{}` must rebuild the recorded sub-task area at the recorded id — a \
                     guard reaching this door would brick every milestone whose record \
                     already names such a sub-task",
                    door.site,
                );
                IdSource::Exempt(
                    "the id is read back from the committed record (`item.id`, passed to \
                     the mint as `slug_override`), so this door slugs no prose and a record \
                     that already names such a sub-task re-seeds rather than bricking",
                )
            }
            other => panic!(
                "`MINT_DOORS` member `{other}` ({}) is unclassified on the id-source axis — \
                 a new mint door owes this fence either the degenerate cell or a stated \
                 exemption, exactly as it already owes `Snapshot` one",
                door.door,
            ),
        };
        match source {
            IdSource::Prose => prose += 1,
            IdSource::Exempt(reason) => {
                assert!(
                    !reason.trim().is_empty(),
                    "`{}`'s exemption must state its reason",
                    door.site,
                );
                exempt += 1;
            }
        }
    }
    // Non-vacuity, without a remembered count: every row is classified (the match is
    // exhaustive or it panics), and **both** kinds were actually observed — a classification
    // where one side is empty proves nothing about the other.
    assert_eq!(
        prose + exempt,
        MINT_DOORS.len(),
        "every mint door is classified on the id-source axis",
    );
    assert!(
        prose > 0 && exempt > 0,
        "both kinds must be observed — {prose} prose, {exempt} exempt",
    );
}

/// `jigc milestone add-from-spec` over a spec carrying a **degenerate criterion** refuses,
/// and commits no record for it (M53 Increment 5 / T3; `settle-record.md` → D5, the third
/// committing door the charter did not name).
///
/// It is its own arm because the seam is its own: `engine::milestone::add_from_spec`
/// computes `mint_sub_id(intent)` and consults the **resume skip set** *before* it calls
/// `add_task`, so a degenerate criterion whose fabricated `task` id the milestone already
/// carries was skipped at exit 0 and never reached `add_task`'s guard at all (driven at
/// `423d8a58`). The guard therefore sits ahead of the skip check, and this arm drives the
/// door rather than the function.
///
/// The degenerate criterion is deliberately the **second** one: the pass mints the first,
/// then aborts, and the door's mid-loop unwind must carry that mint out — so *"no record
/// commit"* here is a statement about the whole call, not about an abort that happened to
/// come first.
#[test]
fn add_from_spec_refuses_a_criterion_that_yields_no_id() {
    let fixture = clean_fixture("from-spec");
    let specs = fixture.path().join("docs").join("specs");
    fs::create_dir_all(&specs).expect("mk docs/specs/");
    fs::write(
        specs.join("rate-limit.md"),
        "# Rate limit\n\n## Goal\n\nBound per-client request volume.\n\n## Context\n\n\
         Downstream services enforced limits ad hoc.\n\n## Criteria\n\n\
         ### Rejects the 101st request  {#rejects-burst}\n\n\
         The gateway rejects the 101st request in a rolling 60s window.\n\n\
         ### 日本語  {#in-another-script}\n\n\
         A criterion whose text carries nothing an id can be built from.\n",
    )
    .expect("write the spec");
    git_ok(fixture.path(), &["add", "-A"]);
    git_ok(
        fixture.path(),
        &["commit", "-q", "-m", "a two-criteria spec"],
    );

    fixture.ok(&["milestone", "create", "Rate limit"]);
    let before_head = head_sha(fixture.path());
    let before_areas = area_names(fixture.path());

    let out = fixture.run(&[
        "milestone",
        "add-from-spec",
        "rate-limit",
        "spec:rate-limit",
    ]);
    let surface = format!(
        "{}{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr),
    );
    assert!(
        !out.status.success(),
        "a degenerate criterion must refuse the seeding pass\n{surface}",
    );
    assert!(
        surface.contains(UNSLUGABLE_CODE) && surface.contains(UNSLUGABLE_SENTENCE),
        "the pass refuses under the mint class's identity and sentence\n{surface}",
    );
    assert_eq!(
        head_sha(fixture.path()),
        before_head,
        "the refused pass lands no record commit — not for the degenerate criterion, and \
         not for the ordinary one it had already minted\n{surface}",
    );
    assert_eq!(
        area_names(fixture.path()),
        before_areas,
        "the refused pass leaves no sub-task area — the mid-loop unwind carries out what \
         it minted before the abort\n{surface}",
    );
    assert_eq!(
        porcelain(fixture.path()),
        "",
        "the refused pass leaves the working tree clean\n{surface}",
    );
}

/// Outside a repository, `jigc milestone create ""` still answers the **one** not-in-repo
/// answer (M53 Increment 5 / T3; `settle-record.md` → **§12**).
///
/// This is the cell that decides *where* the guard sits. Placed literally at the top of
/// `run_create` — as D5 first wrote it — a caller standing in the wrong directory would be
/// told their title has no letters, which is a true sentence about a fact that is not their
/// problem and buries M49's converged precondition. The guard therefore sits after
/// `discover_repo_root` and `jigc_home_or_repo` (both reads) and before the first write,
/// and this arm is what keeps it there.
#[test]
fn outside_a_repository_the_degenerate_title_still_answers_not_in_repo() {
    let outside = TempDir::new("outside");
    let home = TempDir::new("outside-home");
    let out = Command::new(env!("CARGO_BIN_EXE_jigc"))
        .args(["milestone", "create", ""])
        .current_dir(outside.path())
        .env("HOME", home.path())
        .env_remove("JIGC_PACK_DIR")
        .output()
        .expect("spawn the jigc binary");
    let surface = format!(
        "{}{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr),
    );
    assert!(
        !out.status.success(),
        "outside a repository this refuses\n{surface}"
    );
    assert!(
        surface.contains("not inside a git repository"),
        "the precondition every door outside a repository shares is the answer\n{surface}",
    );
    assert!(
        !surface.contains(UNSLUGABLE_CODE),
        "the title guard must not pre-empt the not-in-repo answer — it sits after the two \
         reads that establish where jigc is standing\n{surface}",
    );
}

/// **A mint door whose title is optional routes at the exit that omits it** (M53, the rc.20
/// per-axis review `(3, F-C)`).
///
/// `write.unslugable-title` is one producer for the whole mint class, and its route —
/// *re-run with a title carrying ASCII letters or digits* — is the complete exit set at
/// every [`MINT_DOORS`] row whose title is **required**. Then the registry grew a sixth
/// member whose title is not: `jigc task amend ["<intent>"]`, where omitting the intent
/// names the task after the commit it rewrites (`amend-<sha7>`). The shared route was not
/// re-derived over the widened set, so the one door with a second exit was the one door
/// that never named it — M45's complete-fix lens turned on M53's own new code, with the
/// registry as the axis.
///
/// **The axis is read off the registry, not listed here.** A row's `door` spelling is where
/// the optionality is already written down (the brackets), so a seventh door with an
/// optional title joins this cell by existing, and a row that drops its brackets reddens
/// the count. The control is the sibling immediately beside it: `jigc milestone create ""`
/// raises the identical code and must **not** grow the clause, because there is no fallback
/// there to name.
#[test]
fn a_mint_door_whose_title_is_optional_routes_at_the_exit_that_omits_it() {
    let optional: Vec<&str> = MINT_DOORS
        .iter()
        .map(|row| row.door)
        .filter(|door| door.contains('['))
        .collect();
    assert_eq!(
        optional,
        vec!["jigc task amend [\"<intent>\"]"],
        "the axis is the rows whose registry spelling makes the title optional — a new one \
         owes this cell, and a row that loses its brackets owes an explanation",
    );

    let fixture = clean_fixture("optional-title-route");
    let short = {
        let out = Command::new("git")
            .args(["rev-parse", "--short=7", "HEAD"])
            .current_dir(fixture.path())
            .output()
            .expect("read HEAD");
        String::from_utf8_lossy(&out.stdout).trim().to_string()
    };

    for title in UNSLUGABLE_TITLES {
        let out = fixture.run(&["task", "amend", title]);
        let surface = format!(
            "{}{}",
            String::from_utf8_lossy(&out.stdout),
            String::from_utf8_lossy(&out.stderr),
        );
        assert!(
            surface.contains(UNSLUGABLE_CODE) && surface.contains("amend-<sha7>"),
            "the refusal must name the exit only this door has, for the title {title:?}\
             \n{surface}",
        );
    }

    // The control: the sibling with no fallback must not acquire the clause.
    let refused = fixture.run(&["milestone", "create", "日本語"]);
    let surface = format!(
        "{}{}",
        String::from_utf8_lossy(&refused.stdout),
        String::from_utf8_lossy(&refused.stderr),
    );
    assert!(
        surface.contains(UNSLUGABLE_CODE) && !surface.contains("amend-<sha7>"),
        "a door with no second exit must not be given one\n{surface}",
    );

    // …and the exit the route names is real: run it, and the mint lands at that id.
    let minted = fixture.run(&["task", "amend"]);
    assert!(
        minted.status.success(),
        "the routed exit must run; output:\n{}{}",
        String::from_utf8_lossy(&minted.stdout),
        String::from_utf8_lossy(&minted.stderr),
    );
    assert!(
        area_names(fixture.path())
            .iter()
            .any(|(kind, name)| kind == "task" && *name == format!("amend-{short}")),
        "the fallback the route names is the id the mint takes; areas: {:?}",
        area_names(fixture.path()),
    );
}
