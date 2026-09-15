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
//! **A row with no cell is a hard panic, never a skip**: the cells are generated for
//! every registered row, the driven set is compared back to the registry, and the count
//! is asserted, so a door cannot join the registry and quietly run nothing.
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
use engine::slug::is_slug;
use engine::state::MINT_DOORS;

/// The blocking finding a malformed work-unit id earns at every door (T1).
const MALFORMED_CODE: &str = "work-unit.malformed-id";

/// The well-formed id no work unit in the fixture carries — the fourth cell's token.
const UNKNOWN_ID: &str = "no-such-work-unit";

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

/// **The axis** — every registered door, over the four-token cell space.
///
/// Each malformed cell must block with [`MALFORMED_CODE`], name the token as typed, and
/// carry exactly one route; the unknown cell must give the family's unchanged roster
/// answer and must **not** carry the grammar refusal. After every cell the fixture's tree
/// is re-asserted, because at `caa137e~` the *text* of one of these cells was already
/// correct over a repository that had just been deleted.
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
    let absolute = fixture.path().to_string_lossy().into_owned();
    let mints = mint_leaf_verbs();

    let cells: Vec<(String, Expect)> = vec![
        (String::new(), Expect::Malformed),
        ("../..".to_string(), Expect::Malformed),
        (absolute, Expect::Malformed),
        (UNKNOWN_ID.to_string(), Expect::Unknown),
    ];

    let mut driven: BTreeSet<Vec<&str>> = BTreeSet::new();
    for row in WORK_UNIT_ID_DOORS {
        let family = family_of(row.arg);
        assert_eq!(
            cells.len(),
            4,
            "`jigc {}`: every row is driven over the whole cell space",
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
}
