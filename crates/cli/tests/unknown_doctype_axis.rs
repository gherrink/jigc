//! M49 Increment 11 / T1 — **the unknown-doctype axis, one row per door that takes a
//! doctype.**
//!
//! The class: *what jigc says when the doctype you named does not exist*. At HEAD it said
//! it five different ways across fifteen doors. Three were routed findings
//! (`doc create`/`doc author` → `create.unknown-doctype`; `doc show`/`doc schema`/`doc
//! list` → `store.unknown-type`). The rest were not surfaces at all but leaked internals:
//! the six `doc` write verbs answered
//!
//! ```text
//! unknown doctype `x`: the embedded pack is missing `x`: no pack resource of kind Schemas with id `x`
//! ```
//!
//! — no finding code, no route, and `Schemas` is `engine::packsource::PackResourceKind`'s
//! **`{:?}`**, a Rust enum rendered at a user; `jigc rename` answered a bare `unknown
//! doctype \`x\``; `jigc task bind` answered ``no such doc `x:y` `` (naming the *doc* when
//! the *doctype* is the fault); `jigc migrate --as` carried a route but no code; and
//! `jigc relocate` carried neither (`DECISIONS.md` → 2026-08-31 M49 Increment 11).
//!
//! **The door set is derived, not remembered.** [`DOCTYPE_DOORS`] is fenced ⇔ against the
//! real clap tree by the argument ids that carry a doctype
//! ([`doctype_arg_ids`]; `cli_parse::every_doctype_door_is_registered`), so this suite
//! enumerates the binary's doors rather than the seven an audit happened to walk — and the
//! roadmap's "7 doors" is corrected to **15** on that derivation.
//!
//! **Two answers, and the split is the door's *kind*, not its verb** (kept, not collapsed —
//! `DECISIONS.md` → 2026-08-31):
//!
//!   * a **create-gate** door (`doc create` / `doc author`) is naming a doctype to *mint*,
//!     so it blocks `create.unknown-doctype` and names the authorable set;
//!   * every other door is naming a doctype that must already **exist** in the resolved
//!     cascade, so it blocks `store.unknown-type` — one fault, one code, whether the id
//!     arrived bare (`jigc relocate x`) or as an address head (`jigc doc set-slot x:y#z`).
//!
//! Every cell is driven through the **real binary** and asserts: the call blocks non-zero;
//! the surface carries the door's declared finding code and names the doctype the caller
//! typed; it carries **exactly one** `route:` line; that route's command **runs verbatim
//! at exit 0**; and its stderr contains **no `{:?}` rendering of a
//! [`PackResourceKind`]** — the leak's own enum, matched exhaustively here so a sixth
//! variant cannot ship without an author coming to this suite.

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

use cli::cli::{DOCTYPE_DOORS, DoctypeArg, doctype_arg_ids};
use engine::packsource::PackResourceKind;

/// The doctype id no pack ships — the axis's single input.
const UNKNOWN: &str = "nosuch";

/// The fixture's one open task — every in-task cell scopes to it explicitly, so no cell
/// depends on active-task resolution.
const TASK: &str = "axis";

/// The fixture's one milestone — `milestone add-from-spec` reads the named spec only after
/// the milestone resolves, so the cell needs a **real** one or the door would answer about
/// the milestone instead of about the doctype.
const MILESTONE: &str = "axis-m";

/// The two answers the axis is allowed to give (the create-gate / already-exists split).
const CREATE_CODE: &str = "create.unknown-doctype";
const STORE_CODE: &str = "store.unknown-type";

/// A throwaway directory that removes itself on drop.
struct TempDir(PathBuf);

impl TempDir {
    fn new(tag: &str) -> Self {
        let mut path = std::env::temp_dir();
        path.push(format!(
            "jigc-unknown-doctype-axis-{tag}-{}-{:?}",
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

/// A set-up repo with one open task, a foreign file for `jigc migrate` to point at, and a
/// payload file for the two `--from-file` doors. The task is minted on
/// `implement-from-spec` because `jigc task bind` needs a workflow that declares a `reads`
/// role — the role check fires before the address is adjudicated, so a role-less task
/// would never reach this axis's fault.
struct Fixture {
    repo: TempDir,
    home: TempDir,
}

impl Fixture {
    fn new() -> Self {
        let repo = TempDir::new("repo");
        let home = TempDir::new("home");
        init_repo(repo.path());
        let fixture = Fixture { repo, home };
        fs::write(fixture.repo.path().join("STATE.md"), "# State\n").expect("write foreign file");
        fs::write(
            fixture.repo.path().join("payload.yaml"),
            "title: X\nsections: []\n",
        )
        .expect("write payload");
        let setup = fixture.run(&["setup"]);
        assert!(
            setup.status.success(),
            "`jigc setup` must succeed: {}",
            String::from_utf8_lossy(&setup.stderr),
        );
        let start = fixture.run(&[
            "start",
            "--workflow",
            "implement-from-spec",
            "axis intent",
            "--slug",
            TASK,
        ]);
        assert!(
            start.status.success(),
            "minting the fixture task must succeed: {}",
            String::from_utf8_lossy(&start.stderr),
        );
        let milestone = fixture.run(&["milestone", "create", "Axis M"]);
        assert!(
            milestone.status.success(),
            "minting the fixture milestone must succeed: {}",
            String::from_utf8_lossy(&milestone.stderr),
        );
        assert!(
            String::from_utf8_lossy(&milestone.stdout).contains(MILESTONE),
            "the fixture milestone must be `{MILESTONE}`; got: {}",
            String::from_utf8_lossy(&milestone.stdout),
        );
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

/// One cell: a [`DOCTYPE_DOORS`] path, the argv that carries [`UNKNOWN`] into it, and the
/// finding code that door's kind owes.
struct Cell {
    door: &'static [&'static str],
    argv: &'static [&'static str],
    code: &'static str,
}

/// **The axis** — every door of [`DOCTYPE_DOORS`], plus the second `set-field` arm
/// (`--unset` reaches the schema through its own code path, so it is driven, not assumed).
fn axis() -> Vec<Cell> {
    vec![
        Cell {
            door: &["migrate"],
            argv: &["migrate", "STATE.md", "--as", UNKNOWN],
            code: STORE_CODE,
        },
        Cell {
            door: &["relocate"],
            argv: &["relocate", UNKNOWN, "--from", "docs/"],
            code: STORE_CODE,
        },
        Cell {
            door: &["rename"],
            argv: &["rename", "nosuch:thing", "--to", "New Title"],
            code: STORE_CODE,
        },
        Cell {
            door: &["doc", "create"],
            argv: &["doc", "create", UNKNOWN, "--title", "X", "--task", TASK],
            code: CREATE_CODE,
        },
        Cell {
            door: &["doc", "author"],
            argv: &[
                "doc",
                "author",
                UNKNOWN,
                "--from-file",
                "payload.yaml",
                "--task",
                TASK,
            ],
            code: CREATE_CODE,
        },
        Cell {
            door: &["doc", "schema"],
            argv: &["doc", "schema", UNKNOWN],
            code: STORE_CODE,
        },
        Cell {
            door: &["doc", "list"],
            argv: &["doc", "list", UNKNOWN],
            code: STORE_CODE,
        },
        Cell {
            door: &["doc", "show"],
            argv: &["doc", "show", "nosuch:thing"],
            code: STORE_CODE,
        },
        Cell {
            door: &["doc", "set-field"],
            argv: &[
                "doc",
                "set-field",
                "nosuch:thing#status",
                "--value",
                "accepted",
                "--task",
                TASK,
            ],
            code: STORE_CODE,
        },
        Cell {
            door: &["doc", "set-field"],
            argv: &[
                "doc",
                "set-field",
                "nosuch:thing#status",
                "--unset",
                "--task",
                TASK,
            ],
            code: STORE_CODE,
        },
        Cell {
            door: &["doc", "set-slot"],
            argv: &[
                "doc",
                "set-slot",
                "nosuch:thing#context",
                "--from-file",
                "payload.yaml",
                "--task",
                TASK,
            ],
            code: STORE_CODE,
        },
        Cell {
            door: &["doc", "add-item"],
            argv: &[
                "doc",
                "add-item",
                "nosuch:thing#entries",
                "--title",
                "X",
                "--task",
                TASK,
            ],
            code: STORE_CODE,
        },
        Cell {
            door: &["doc", "remove-item"],
            argv: &[
                "doc",
                "remove-item",
                "nosuch:thing#entries/x",
                "--task",
                TASK,
            ],
            code: STORE_CODE,
        },
        Cell {
            door: &["doc", "retitle-item"],
            argv: &[
                "doc",
                "retitle-item",
                "nosuch:thing#entries/x",
                "--title",
                "Y",
                "--task",
                TASK,
            ],
            code: STORE_CODE,
        },
        Cell {
            door: &["doc", "rename"],
            argv: &[
                "doc",
                "rename",
                "nosuch:thing",
                "--to",
                "New Title",
                "--task",
                TASK,
            ],
            code: STORE_CODE,
        },
        Cell {
            door: &["task", "bind"],
            argv: &["task", "bind", "spec", "nosuch:thing", TASK],
            code: STORE_CODE,
        },
        Cell {
            door: &["milestone", "add-from-spec"],
            argv: &["milestone", "add-from-spec", MILESTONE, "nosuch:thing"],
            code: STORE_CODE,
        },
    ]
}

/// Every [`PackResourceKind`] variant — matched **exhaustively**, so a sixth variant
/// cannot compile without an author coming here, and the Debug-leak fence below cannot
/// fall behind the enum it fences.
fn pack_resource_kind_debug_spellings() -> Vec<String> {
    [
        PackResourceKind::Schemas,
        PackResourceKind::Workflows,
        PackResourceKind::Steps,
        PackResourceKind::Config,
        PackResourceKind::SchemaSnapshots,
    ]
    .iter()
    .map(|kind| {
        // The exhaustive arm is the fence; the value it produces is the `{:?}` spelling.
        match kind {
            PackResourceKind::Schemas
            | PackResourceKind::Workflows
            | PackResourceKind::Steps
            | PackResourceKind::Config
            | PackResourceKind::SchemaSnapshots => format!("{kind:?}"),
        }
    })
    .collect()
}

/// The `route: …` lines of a rendered surface.
fn route_lines(text: &str) -> Vec<String> {
    text.lines()
        .map(str::trim_start)
        .filter(|line| line.starts_with("route:"))
        .map(|line| line.trim_start_matches("route:").trim().to_owned())
        .collect()
}

/// The first backticked command span of a route line — the argv a reader would paste.
fn route_command(route: &str) -> String {
    let (_, rest) = route
        .split_once('`')
        .unwrap_or_else(|| panic!("a route must name a command in backticks: {route}"));
    let (command, _) = rest
        .split_once('`')
        .unwrap_or_else(|| panic!("a route's command span must close: {route}"));
    command.to_string()
}

/// **The axis.** Every door, driven through the real binary with one unknown doctype: one
/// blocking answer, carrying the door kind's code, the doctype the caller typed, exactly
/// one route — which runs verbatim at exit 0 — and no leaked engine `{:?}`.
#[test]
fn every_doctype_door_answers_an_unknown_doctype_the_same_way() {
    let fixture = Fixture::new();
    let debug_spellings = pack_resource_kind_debug_spellings();

    for cell in axis() {
        let name = cell.argv.join(" ");
        let out = fixture.run(cell.argv);
        assert!(
            !out.status.success(),
            "`jigc {name}`: an unknown doctype must block non-zero",
        );
        let stderr = String::from_utf8_lossy(&out.stderr).into_owned();

        assert!(
            stderr.contains(&format!("· {} — ", cell.code)),
            "`jigc {name}`: the surface must carry the `{}` finding code\n{stderr}",
            cell.code,
        );
        assert!(
            stderr.contains(&format!("`{UNKNOWN}`")),
            "`jigc {name}`: the surface must name the doctype the caller typed\n{stderr}",
        );

        for spelling in &debug_spellings {
            assert!(
                !stderr.contains(spelling),
                "`jigc {name}`: stderr leaks `PackResourceKind::{spelling}` — a Rust enum's \
                 `{{:?}}` rendered at a reader\n{stderr}",
            );
        }

        let routes = route_lines(&stderr);
        assert_eq!(
            routes.len(),
            1,
            "`jigc {name}`: the surface must carry exactly one route\n{stderr}",
        );
        let command = route_command(&routes[0]);
        let argv: Vec<&str> = command.split_whitespace().collect();
        assert_eq!(
            argv.first().copied(),
            Some("jigc"),
            "`jigc {name}`: the route's command must be a jigc invocation: {command}",
        );
        let followed = fixture.run(&argv[1..]);
        assert!(
            followed.status.success(),
            "`jigc {name}`: the emitted route `{command}` must run verbatim at exit 0\n{}",
            String::from_utf8_lossy(&followed.stderr),
        );

        assert!(
            matches!(cell.code, CREATE_CODE | STORE_CODE),
            "`jigc {name}`: the axis has two answers, not three",
        );
    }
}

/// **The table is bound to the door registry**, which is itself bound to the clap tree:
/// every registered door is driven by at least one cell, and no cell drives a path the
/// registry does not carry. Without this a door could be added to the binary, declared in
/// `DOCTYPE_DOORS`, and never exercised here.
#[test]
fn the_axis_drives_every_registered_doctype_door() {
    let cells = axis();
    for (door, _) in DOCTYPE_DOORS {
        assert!(
            cells.iter().any(|cell| cell.door == *door),
            "`jigc {}` is a registered doctype door with no cell in this axis",
            door.join(" "),
        );
    }
    for cell in &cells {
        assert!(
            DOCTYPE_DOORS.iter().any(|(door, _)| *door == cell.door),
            "`jigc {}` drives a path DOCTYPE_DOORS does not carry",
            cell.door.join(" "),
        );
    }
    assert!(
        !doctype_arg_ids().is_empty(),
        "the derivation vocabulary must be non-empty, else the ⇔ fence is vacuous",
    );
}

/// **An address-headed door always names a doc that must already exist**, so it can never
/// answer with the create-gate's code — the half of the split that is checkable from the
/// registry rather than declared cell by cell.
#[test]
fn every_address_headed_door_answers_with_the_store_code() {
    for cell in axis() {
        let shape = DOCTYPE_DOORS
            .iter()
            .find(|(door, _)| *door == cell.door)
            .map(|(_, shape)| *shape)
            .expect("every cell names a registered door");
        if shape == DoctypeArg::Address {
            assert_eq!(
                cell.code,
                STORE_CODE,
                "`jigc {}` takes a doc address, so its doctype names an existing doc",
                cell.door.join(" "),
            );
        }
    }
}
