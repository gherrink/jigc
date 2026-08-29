//! M47 Increment 6, T1 — **the write-verb × miss-shape axis, one row per cell.**
//!
//! The class: *which miss a write reject claims to be*. An **item-id miss** — the
//! addressed item was never minted — is not a shape question: the schema names the
//! *shape*, never the corpus's real item ids, so `jigc doc schema <doctype>` is a dead
//! end there. Before this suite the diagnosis was decided per call site, and four of the
//! eight cells got it wrong (`set-field --value`, `set-field --unset` at an undeclared
//! field, `retitle-item`, and nested `add-item` under an absent parent), while the two
//! already-correct cells (`set-slot`, `remove-item`) had no axis-level fence keeping the
//! rest with them (`DECISIONS.md` → 2026-07-26 M47 Settle, Decision 8 — sweep the axis,
//! not the instance; `baseline.md` §4e).
//!
//! The axis is enumerated as **data**, one row per cell, so a write verb added later is
//! covered by adding a row rather than by remembering this file exists:
//!
//!   * the **item-id misses** → `write.not-present`
//!     (`set-slot` · `remove-item` · `set-field --value` · `set-field --unset` at a
//!     declared field · `set-field --unset` at an *undeclared* field — the item miss
//!     outranks the field question — · `retitle-item` · nested `add-item` under an
//!     absent parent, the eighth cell the baseline census under-counted · and, added
//!     with the route column, `retitle-item` at an absent **nested** item under a real
//!     parent, the deepest strip);
//!   * the **undeclared-section** misses → `write.unknown-section`, over the same six
//!     item-addressing verbs plus the bare `add-item` (see the column note below);
//!   * a **genuine declared-shape defect** (`add-item` into a non-repeatable section) →
//!     `write.wrong-shape`, which the flip deliberately leaves standing.
//!
//! Every cell asserts the same three things through the real binary: the write **blocks
//! non-zero**, the emitted `--format json` finding carries the expected `code`, and the
//! **staged bytes are byte-identical** to before the call (a rejected write persists
//! nothing).
//!
//! **T2 adds the route column** — the same axis, one hop further: naming the miss
//! correctly is worthless if the route the agent is handed is still the schema dead end.
//! Every item-id-miss cell must emit
//! `jigc doc show <type>:<slug>#<top-section> --task <id>` carrying the **real** slug and
//! the **resolved** task id (never the `<address>` / `<task-id>` placeholders of the
//! engine's defensive fallback), and the emitted argv is **run verbatim** — it must exit 0
//! and print the section's live item ids, so the recovery is followable and not merely
//! well-worded (`design/surface-contract.md` law 2: nothing hides;
//! `design/validation.md` → the `write.*` route split). A **nested** cell strips to the
//! top *showable* section — the M44 N2 pin, re-asserted here at the four verbs the CLI
//! enrichment did not reach (`set-field --value` / `--unset` · `retitle-item` ·
//! `add-item`).
//!
//! **T3 closes the id-from-leaf strip.** Naming the miss and routing it are worth nothing
//! at a cell that never reaches either, and two *schema-only* CLI pre-checks —
//! `write.id-from-field` (`set-field` at an id-from leaf) and `write.identity-change`
//! (`retitle-item` under an **enum** id-from), plus the engine's own `retitle_item`
//! id-from re-validation — answered from the doctype alone and so out-ranked presence:
//! they asserted what a **nonexistent** item derives its id from, and routed to a
//! `retitle-item` / `remove-item` / `set-field` that blocks on the same absence. Four rows
//! were added over the two verbs × the two depths, and — the reason the one pre-existing
//! nested row greened over a live defect — **the fixture's id-from topology was corrected
//! to the shipped dev pack's**: its change-group `category` was declared a plain `string`,
//! the one shape that sidesteps the enum arm, while `crates/cli/pack/schemas/changelog.yaml`
//! declares it an **enum**. A fixture authored in the case the implementation handles
//! proves nothing about the case the corpus actually has.
//!
//! **The undeclared-SECTION column is swept like the item-id one.** The matrix is verb ×
//! miss-shape, and this column shipped with **one** cell (bare `add-item`) against the
//! item-id column's twelve — so the same miss came back four different ways underneath it:
//! `set-slot` / `remove-item` answered `write.not-present` (and, enriched, handed back a
//! `jigc doc show <doc>#<undeclared-section>` whose **verbatim run exits 1** — the
//! un-followable route this suite's T2 property exists to forbid), `retitle-item` / nested
//! `add-item` answered `write.wrong-shape`, and `set-field --unset` answered
//! `write.unknown-field` about a field on an item in a section that does not exist. Six
//! rows join the column, and the shape cells' route is now **run verbatim** too: it must be
//! `jigc doc schema <doctype>` — the read that genuinely answers a shape question — and it
//! must exit 0 naming the doctype's declared sections. Engine-side the fix is one ranked
//! predicate (`engine::write::section_undeclared`, rank 1 of shape → presence → leaf)
//! opening every item-addressing door.
//!
//! **M48 adds the title-miss column.** Every column above is an *address* miss; a
//! **doc-minting** verb (`create` / `author`) can miss one hop earlier — on the title it
//! was handed — and until M48 neither shape was a miss at all: both exited 0, one of them
//! committing a second document nobody authored. The two rows split on whether the
//! identity moves, because the recovery does not: a call that would mint a **different**
//! identity beside the one the gate's `as:` role already binds is `write.identity-change`
//! (a second document, not a correction), while a call landing on the **same** identity
//! that merely drops the title is identity-**stable** (`design/storage.md` → Identity)
//! and earns `write.title-ignored`. Both route at `jigc doc rename` — a route that is
//! itself a write, so it is run verbatim against a **private** fixture in the same state.
//!
//! **M49 adds the collision column.** Every column above is a *miss* — an address that
//! names nothing. This one is the opposite: the address is right, the shape is right, and
//! the **minted id is taken**. Two genuinely different titles can slug alike (`1-3-0` and
//! `1.3.0` mint the same id), and until M49 the reject's route was a `Route::human` —
//! *"the target already exists — edit it in place"* — which is wrong for a second, distinct
//! entry, and is not runnable at all, let alone from inside the `doc author` batch it
//! rejects whole. With `--slug` on `add-item` (T4 of this increment) the recovery **is** a
//! command, so the route is mechanical: the same mint under a free id. Two rows, because
//! the collision reaches an agent through two doors that must not disagree — the per-leaf
//! verb and the batch — and both inherit the one CLI-seam enrichment at
//! `apply_add_item_target`. The route is run **verbatim** and must *land the second item*,
//! not merely exit 0.
//!
//! **Declared bound.** The two *section-level* address forms — `set-slot` / `set-field` at
//! `#<undeclared-section>` with no item hop — are **not** cells of this matrix: the CLI
//! address resolver refuses to resolve a slot/field target inside an undeclared section and
//! emits the `{"error": …}` envelope, so they never reach a write door and never mint a
//! `write.*` finding. That is a different seam (target resolution, not the write-reject
//! taxonomy) and is left as it is.

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

/// A throwaway directory that removes itself on drop.
struct TempDir(PathBuf);

impl TempDir {
    fn new(tag: &str) -> Self {
        let mut path = std::env::temp_dir();
        let unique = format!(
            "jigc-miss-shape-axis-{tag}-{}-{:?}",
            std::process::id(),
            engine::tempname::unique_nanos(),
        );
        path.push(unique);
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

/// Recursively copy `from` into `to` (both directories).
fn copy_tree(from: &Path, to: &Path) {
    fs::create_dir_all(to).expect("create dest dir");
    for entry in fs::read_dir(from).expect("read src dir") {
        let entry = entry.expect("dir entry");
        let src = entry.path();
        let dst = to.join(entry.file_name());
        if src.is_dir() {
            copy_tree(&src, &dst);
        } else {
            fs::copy(&src, &dst).expect("copy file");
        }
    }
}

/// The fixture `changelog` schema — a two-level repeatable (so a nested `add-item`
/// under an absent parent has a home), carrying a `summary` slot and an **optional**
/// `link` field (the only unset-eligible shape: a required or defaulted field is
/// refused by the eligibility guard before the item is ever adjudicated), plus a
/// **non-repeatable** `overview` section so the genuine shape question has a target.
///
/// **The id-from topology mirrors the shipped dev pack's changelog, deliberately** (M47
/// Increment 6, T3 — the fixture-topology mask): the two CLI-side, schema-only
/// pre-checks that outranked item presence (`write.id-from-field` /
/// `write.identity-change`) are reached only through an id-from leaf, and the
/// **`write.identity-change`** arm only through an **enum** id-from. A fixture that
/// declares its change-group `category` as a plain `string` — as this one first did —
/// sidesteps that arm entirely and greens a cell the shipped pack false-fails. So both
/// `category` blocks are enums (`crates/cli/pack/schemas/changelog.yaml`), and a
/// single-level `staged` repeatable mirrors the pack's `unreleased-changes` so the
/// **top-level** enum id-from is on the axis too, not only the nested one.
///
/// It declares a **`location:`** for the same class of reason (M48): the title-miss rows
/// route at `jigc doc rename`, which refuses a doctype with no committed home as a
/// *transient sink* — so a home-less fixture would emit a route it cannot follow, and the
/// new column would green on a dead end. The nine shipped slug-identity doctypes those
/// rows model all declare one.
const CHANGELOG_SCHEMA: &str = "\
type: changelog
location: changelogs/
id-from: title
sections:
  - id: overview
    slot: { hint: \"What this changelog covers.\", optional: true }
  - id: staged
    repeatable:
      id-from: category
      block:
        - { id: category, type: enum, of: [added, changed, deprecated, removed, fixed, security] }
        - { id: notes, slot: { hint: \"One bullet per staged change.\" } }
  - id: releases
    repeatable:
      id-from: version
      block:
        - { id: version, type: string }
        - { id: link, type: string, optional: true }
        - { id: summary, slot: { hint: \"One-line release summary.\" } }
        - id: changes
          repeatable:
            id-from: category
            block:
              - { id: category, type: enum, of: [added, changed, deprecated, removed, fixed, security] }
              - { id: notes, slot: { hint: \"One bullet per change.\" } }
";

/// Build a throwaway pack: the embedded dev pack tree copied to a temp dir, plus the
/// fixture `changelog` schema and a `log-change` workflow whose create-gate admits it.
fn fixture_pack() -> TempDir {
    let pack = TempDir::new("pack");
    let dev_pack = Path::new(env!("CARGO_MANIFEST_DIR")).join("pack");
    copy_tree(&dev_pack, pack.path());
    // The fixture ships a deliberately divergent `changelog` shape, so it is NOT the
    // frozen dev pack — drop the copied freeze manifest (the pack-load gate would
    // otherwise block the un-bumped shape change). A manifest-less pack is unchecked.
    fs::remove_file(pack.path().join("config").join("schema-manifest.yaml"))
        .expect("drop the copied freeze manifest");

    fs::write(
        pack.path().join("schemas").join("changelog.yaml"),
        CHANGELOG_SCHEMA,
    )
    .expect("write changelog schema");

    fs::write(
        pack.path().join("workflows").join("log-change.yaml"),
        "\
---
when: record a release's changes in the changelog
description: Author the changelog for a release.
usage: a release's changes need recording in the changelog.
creates-task: true
allows-create: [{type: changelog, as: changelog}]
---
{{ include: step:finalize }}
",
    )
    .expect("write log-change workflow");

    pack
}

/// Initialize a real git repo with one commit (composition reads HEAD) plus the
/// `.jigc/config/` project layer the cascade expects.
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
    fs::create_dir_all(root.join(".jigc").join("config")).expect("create project layer");
}

/// The held provisioning: the temp guards must stay alive (dropping them removes the
/// repo), so they are returned to the caller.
struct Fixture {
    repo: TempDir,
    home: TempDir,
    pack: TempDir,
    slug: String,
}

impl Fixture {
    fn run(&self, args: &[&str], stdin: Option<&[u8]>) -> std::process::Output {
        use std::io::Write;
        use std::process::Stdio;
        let mut command = Command::new(env!("CARGO_BIN_EXE_jigc"));
        command
            .args(args)
            .current_dir(self.repo.path())
            .env("HOME", self.home.path())
            .env("JIGC_PACK_DIR", self.pack.path())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped());
        if stdin.is_some() {
            command.stdin(Stdio::piped());
        }
        let mut child = command.spawn().expect("spawn the jigc binary");
        if let Some(bytes) = stdin {
            child
                .stdin
                .take()
                .expect("stdin piped")
                .write_all(bytes)
                .expect("write stdin");
        }
        child.wait_with_output().expect("wait for jigc")
    }

    /// The staged working copy of the fixture changelog — the bytes a rejected write
    /// must leave untouched.
    fn staged(&self) -> PathBuf {
        self.repo
            .path()
            .join(".jigc")
            .join("tasks")
            .join(TASK_ID)
            .join("docs")
            .join(format!("changelog:{}.md", self.slug))
    }

    /// Run a route's emitted command **verbatim** (shell-split, the leading `jigc`
    /// dropped), returning its trimmed stdout — the followability proof.
    fn run_route(&self, cmd: &str, what: &str) -> String {
        let argv = shell_split(cmd, self.repo.path(), self.home.path());
        assert_eq!(
            argv.first().map(String::as_str),
            Some("jigc"),
            "`{what}`: the route leads `jigc`"
        );
        let args: Vec<&str> = argv.iter().skip(1).map(String::as_str).collect();
        ok_stdout(self.run(&args, None), what)
    }
}

/// Split an emitted command into argv **through a real `sh`** — the bytes an agent pastes
/// are parsed by the thing that will actually parse them, expansions included. A
/// hand-rolled splitter understands one quoting form and expands nothing, which is exactly
/// how a route carrying live `$` or `$( … )` in a title passes a suite and rewrites the
/// document in a terminal (M48 inc-2 triage). `set --` is the shell's own word splitter.
/// (Kept local: this suite's group root carries no `support` module, and pulling one in
/// would compile the whole shared fixture substrate into this target.)
fn shell_split(cmd: &str, cwd: &Path, home: &Path) -> Vec<String> {
    let script = format!("set -- {cmd}\nfor w in \"$@\"; do printf '%s\\0' \"$w\"; done");
    let out = Command::new("sh")
        .arg("-c")
        .arg(&script)
        .current_dir(cwd)
        .env("HOME", home)
        .output()
        .expect("spawn sh");
    assert!(
        out.status.success(),
        "the emitted command line must parse as shell words; got `{cmd}`\nstderr:\n{}",
        String::from_utf8_lossy(&out.stderr),
    );
    let mut words: Vec<String> = out
        .stdout
        .split(|b| *b == 0)
        .map(|w| String::from_utf8_lossy(w).into_owned())
        .collect();
    words.pop(); // the trailing NUL of the last word yields one empty tail element
    words
}

/// The task id `jigc start "log the release"` mints — the id the enriched route must
/// carry (never the `<task-id>` placeholder).
const TASK_ID: &str = "log-the-release";

/// The leading backticked command of a route string (`` `<cmd>`<tail> ``).
fn backticked<'a>(route: &'a str, what: &str) -> &'a str {
    route
        .strip_prefix('`')
        .and_then(|rest| rest.split('`').next())
        .unwrap_or_else(|| panic!("`{what}`: the route carries a backticked command; got: {route}"))
}

/// The trimmed stdout of a successful `jigc` invocation, or a panic carrying stderr.
fn ok_stdout(out: std::process::Output, what: &str) -> String {
    assert!(
        out.status.success(),
        "`{what}` must exit 0; stderr:\n{}",
        String::from_utf8_lossy(&out.stderr)
    );
    String::from_utf8(out.stdout)
        .expect("utf-8 stdout")
        .trim_end_matches('\n')
        .to_owned()
}

/// Provision a `changelog` task carrying release `1-3-0` with a nested `Added`
/// change-group, returning the live fixture.
fn provision() -> Fixture {
    let repo = TempDir::new("repo");
    let home = TempDir::new("home");
    let pack = fixture_pack();
    init_repo(repo.path());

    let mut fx = Fixture {
        repo,
        home,
        pack,
        slug: String::new(),
    };

    ok_stdout(fx.run(&["setup"], None), "jigc setup");
    ok_stdout(
        fx.run(
            &["start", "--workflow", "log-change", "log the release"],
            None,
        ),
        "jigc start --workflow log-change",
    );

    let created = ok_stdout(
        fx.run(
            &["doc", "create", "changelog", "--title", "Changelog"],
            None,
        ),
        "jigc doc create changelog",
    );
    fx.slug = created
        .strip_prefix("changelog:")
        .expect("created address is changelog:<slug>")
        .to_owned();

    let release = ok_stdout(
        fx.run(
            &[
                "doc",
                "add-item",
                &format!("changelog:{}#releases", fx.slug),
                "--title",
                "1-3-0",
            ],
            None,
        ),
        "add-item release",
    );
    ok_stdout(
        fx.run(
            &[
                "doc",
                "add-item",
                &format!("{release}/changes"),
                "--title",
                "Added",
            ],
            None,
        ),
        "add-item nested change-group",
    );
    // One live item in the top-level **enum** id-from section, so the `staged` cells'
    // emitted route has something real to reveal when it is run verbatim.
    ok_stdout(
        fx.run(
            &[
                "doc",
                "add-item",
                &format!("changelog:{}#staged", fx.slug),
                "--title",
                "Changed",
            ],
            None,
        ),
        "add-item staged change-group",
    );

    fx
}

/// One cell of the write-verb × miss-shape matrix.
struct Cell {
    /// What the cell is, for the assertion messages.
    what: &'static str,
    /// The `jigc` argv after the binary, with `{addr}` standing for the doc address.
    args: &'static [&'static str],
    /// Optional stdin payload (the `--from-file -` cells).
    stdin: Option<&'static [u8]>,
    /// The finding `code` the reject must carry.
    code: &'static str,
    /// How the emitted route is adjudicated — see [`RouteCheck`].
    route: RouteCheck,
}

/// What a cell's emitted route must be, and how it is proven followable.
enum RouteCheck {
    /// An **item-id miss**: the top showable section the route must show
    /// (`jigc doc show <type>:<slug>#<section> --task <id>`), run verbatim on the shared
    /// fixture — a read moves nothing.
    Show(&'static str),
    /// A **shape / declaredness** question, which the schema answers: `jigc doc schema
    /// <doctype>`, run verbatim on the shared fixture.
    Schema,
    /// A route that is itself a **write** (`{addr}` substituted): proven followable on a
    /// **private** fixture in the same state, because running it on the shared one would
    /// move the very bytes the next row diffs against.
    MutatingWrite(&'static str),
    /// A **collision** (M49): the minted item id is already taken, so the recovery is the
    /// same mint under a distinct `--slug`. Proven on a **private** fixture in the same
    /// state, and proven to *land* — a route that exits 0 without minting would pass a
    /// bare followability check while leaving the agent exactly where it was.
    Collides {
        /// The minted id the reject's message must name — which of the payload's items
        /// collided, the question the batch door left unanswered.
        minted: &'static str,
        /// The expected route argv (`{addr}` substituted).
        argv: &'static str,
        /// The section the landing is read back from …
        section: &'static str,
        /// … and the `{#id}` anchor the second item must carry there afterwards.
        landed: &'static str,
    },
    /// The collision column's **omitting context**: a destination whose `id-from` is an
    /// **enum**, where the heading IS the member and `--slug` is refused as an identity
    /// change. The collision must stay on its shipped human route — "edit it in place",
    /// which is the true answer when the id *is* the category — so this asserts the whole
    /// route string verbatim. A mechanical `--slug` route here would name a command that
    /// blocks when run, which is the un-followable route this suite exists to forbid.
    StaysHuman(&'static str),
}

/// The item [`provision`] mints in each showable section — what the emitted route, run
/// verbatim, must reveal. A property of the fixture (not of the cell), so a new row only
/// declares *which* section its route must strip to.
fn live_item(section: &str) -> &'static str {
    match section {
        "releases" => "1-3-0",
        "staged" => "Changed",
        other => panic!("no live item provisioned in section `{other}`"),
    }
}

/// **The axis.** Six item-id misses, the undeclared-section miss, and the one genuine
/// declared-shape defect the flip leaves standing.
const CELLS: &[Cell] = &[
    Cell {
        what: "set-slot at a nonexistent item",
        args: &[
            "doc",
            "set-slot",
            "{addr}#releases/9-9-9/summary",
            "--from-file",
            "-",
        ],
        stdin: Some(b"A summary.\n"),
        code: "write.not-present",
        route: RouteCheck::Show("releases"),
    },
    Cell {
        what: "remove-item at a nonexistent item",
        args: &["doc", "remove-item", "{addr}#releases/9-9-9"],
        stdin: None,
        code: "write.not-present",
        route: RouteCheck::Show("releases"),
    },
    Cell {
        what: "set-field --value at a nonexistent item",
        args: &[
            "doc",
            "set-field",
            "{addr}#releases/9-9-9/link",
            "--value",
            "https://x",
        ],
        stdin: None,
        code: "write.not-present",
        route: RouteCheck::Show("releases"),
    },
    Cell {
        what: "set-field --unset at a nonexistent item, declared field",
        args: &["doc", "set-field", "{addr}#releases/9-9-9/link", "--unset"],
        stdin: None,
        code: "write.not-present",
        route: RouteCheck::Show("releases"),
    },
    Cell {
        what: "set-field --unset at a nonexistent item, undeclared field",
        args: &["doc", "set-field", "{addr}#releases/9-9-9/bogus", "--unset"],
        stdin: None,
        code: "write.not-present",
        route: RouteCheck::Show("releases"),
    },
    Cell {
        what: "retitle-item at a nonexistent item",
        args: &[
            "doc",
            "retitle-item",
            "{addr}#releases/9-9-9",
            "--title",
            "9.9.9",
        ],
        stdin: None,
        code: "write.not-present",
        route: RouteCheck::Show("releases"),
    },
    Cell {
        what: "nested add-item under an absent parent item",
        args: &[
            "doc",
            "add-item",
            "{addr}#releases/9-9-9/changes",
            "--title",
            "Added",
        ],
        stdin: None,
        code: "write.not-present",
        route: RouteCheck::Show("releases"),
    },
    Cell {
        // The **deep** strip at one of the newly-enriched verbs: a real parent release
        // (`1-3-0`), an absent nested change-group — the route must still strip past two
        // hops to the top *showable* section (the N2 pin, re-asserted at `retitle-item`).
        what: "retitle-item at a nonexistent nested item under a real parent",
        args: &[
            "doc",
            "retitle-item",
            "{addr}#releases/1-3-0/changes/no-such-group",
            "--title",
            "Fixed",
        ],
        stdin: None,
        code: "write.not-present",
        route: RouteCheck::Show("releases"),
    },
    // ---- The **id-from leaf** strip (M47 Increment 6, T3). Two CLI-side, schema-only
    // pre-checks sit in front of the engine's presence adjudication at two of the six
    // enriched dispatch sites — `write.id-from-field` (`set-field` at an id-from leaf)
    // and `write.identity-change` (`retitle-item` under an **enum** id-from). Both
    // assert a property of an item and hand back a route whose first verb blocks at an
    // item that was never minted, so item presence must outrank them: the same
    // shape → presence → leaf order the engine's own item-field doors already keep.
    Cell {
        what: "set-field --value at a nonexistent item's id-from leaf",
        args: &[
            "doc",
            "set-field",
            "{addr}#releases/9-9-9/version",
            "--value",
            "9.9.9",
        ],
        stdin: None,
        code: "write.not-present",
        route: RouteCheck::Show("releases"),
    },
    Cell {
        what: "set-field --value at a nonexistent item's enum id-from leaf",
        args: &[
            "doc",
            "set-field",
            "{addr}#staged/no-such-group/category",
            "--value",
            "fixed",
        ],
        stdin: None,
        code: "write.not-present",
        route: RouteCheck::Show("staged"),
    },
    Cell {
        what: "set-field --value at a nonexistent nested item's enum id-from leaf",
        args: &[
            "doc",
            "set-field",
            "{addr}#releases/1-3-0/changes/no-such-group/category",
            "--value",
            "fixed",
        ],
        stdin: None,
        code: "write.not-present",
        route: RouteCheck::Show("releases"),
    },
    Cell {
        what: "retitle-item at a nonexistent item under an enum id-from",
        args: &[
            "doc",
            "retitle-item",
            "{addr}#staged/no-such-group",
            "--title",
            "Fixed",
        ],
        stdin: None,
        code: "write.not-present",
        route: RouteCheck::Show("staged"),
    },
    // ---- The **undeclared-section** column. The row below (`add-item` at a bare
    // undeclared section) was for a long time the column's only cell, so the matrix had
    // 1 of N there while the item-id-miss column had every verb — and five of the six
    // item-addressing verbs disagreed underneath it: `set-slot` / `remove-item` claimed
    // `write.not-present` (and handed back a `jigc doc show <doc>#<undeclared>` that
    // **exits 1** — a route that does not answer), `retitle-item` / nested `add-item`
    // claimed `write.wrong-shape`, and `set-field --unset` claimed `write.unknown-field`
    // about a field on an item in a section that does not exist. Shape outranks presence
    // and presence outranks the leaf (`design/write-commands.md` → Adjudication order),
    // so an undeclared section is `write.unknown-section` at **every** door, whose route
    // is the schema read that genuinely answers it.
    Cell {
        what: "set-slot at an item in an undeclared section",
        args: &[
            "doc",
            "set-slot",
            "{addr}#no-such-section/9-9-9/summary",
            "--from-file",
            "-",
        ],
        stdin: Some(b"A summary.\n"),
        code: "write.unknown-section",
        route: RouteCheck::Schema,
    },
    Cell {
        what: "remove-item at an item in an undeclared section",
        args: &["doc", "remove-item", "{addr}#no-such-section/9-9-9"],
        stdin: None,
        code: "write.unknown-section",
        route: RouteCheck::Schema,
    },
    Cell {
        what: "set-field --value at an item in an undeclared section",
        args: &[
            "doc",
            "set-field",
            "{addr}#no-such-section/9-9-9/link",
            "--value",
            "https://x",
        ],
        stdin: None,
        code: "write.unknown-section",
        route: RouteCheck::Schema,
    },
    Cell {
        what: "set-field --unset at an item in an undeclared section",
        args: &[
            "doc",
            "set-field",
            "{addr}#no-such-section/9-9-9/link",
            "--unset",
        ],
        stdin: None,
        code: "write.unknown-section",
        route: RouteCheck::Schema,
    },
    Cell {
        what: "retitle-item at an item in an undeclared section",
        args: &[
            "doc",
            "retitle-item",
            "{addr}#no-such-section/9-9-9",
            "--title",
            "9.9.9",
        ],
        stdin: None,
        code: "write.unknown-section",
        route: RouteCheck::Schema,
    },
    Cell {
        what: "nested add-item under an item in an undeclared section",
        args: &[
            "doc",
            "add-item",
            "{addr}#no-such-section/9-9-9/changes",
            "--title",
            "Added",
        ],
        stdin: None,
        code: "write.unknown-section",
        route: RouteCheck::Schema,
    },
    Cell {
        what: "add-item into an undeclared section",
        args: &[
            "doc",
            "add-item",
            "{addr}#no-such-section",
            "--title",
            "Added",
        ],
        stdin: None,
        code: "write.unknown-section",
        route: RouteCheck::Schema,
    },
    Cell {
        what: "add-item into a non-repeatable section (the genuine shape question)",
        args: &["doc", "add-item", "{addr}#overview", "--title", "Added"],
        stdin: None,
        code: "write.wrong-shape",
        route: RouteCheck::Schema,
    },
    // ---- The **title-miss** column (M48 Increment 2, T2). The matrix's other columns are
    // all *address* misses; these two are the miss a **doc-minting** verb can make, and
    // until M48 neither was a miss at all — both exited 0. They split on whether the
    // identity moves, because the recovery does not: a call that would mint a DIFFERENT
    // identity beside the one this task holds is `write.identity-change` (a second
    // document, not a correction), while a call that lands on the SAME identity and
    // merely drops the title is identity-**stable** (`design/storage.md` → Identity) and
    // earns `write.title-ignored`. Both route at `jigc doc rename`, the in-task title
    // change — a route that is itself a write, hence `MutatingWrite`.
    Cell {
        what: "create a second identity while the gate's role is already bound",
        args: &["doc", "create", "changelog", "--title", "Release Log"],
        stdin: None,
        code: "write.identity-change",
        route: RouteCheck::MutatingWrite(
            "jigc doc rename {addr} --to 'Release Log' --task log-the-release",
        ),
    },
    Cell {
        what: "create at the held identity with a title that would be dropped",
        args: &["doc", "create", "changelog", "--title", "Changelog!"],
        stdin: None,
        code: "write.title-ignored",
        route: RouteCheck::MutatingWrite(
            "jigc doc rename {addr} --to 'Changelog!' --task log-the-release",
        ),
    },
    // ---- The **collision** column (M49 Increment 5, T5). Not a miss: the section is
    // declared, the shape is right, and the id the title mints is simply taken — the
    // fixture already holds release `1-3-0`, and the distinct title `1.3.0` mints the same
    // id. The shipped route said *"edit it in place"*, which is the answer for a
    // correction and the wrong answer for a second, distinct entry — and is not runnable,
    // so from inside a `doc author` payload (rejected whole, nothing staged) it terminated
    // nowhere. Both doors carry the same row because both funnel through the one
    // `apply_add_item_target` enrichment; a fix at only one of them leaves the other lying.
    Cell {
        what: "add-item at a title that mints a taken id",
        args: &["doc", "add-item", "{addr}#releases", "--title", "1.3.0"],
        stdin: None,
        code: "write.already-present",
        route: RouteCheck::Collides {
            minted: "1-3-0",
            argv: "jigc doc add-item {addr}#releases --title 1.3.0 --slug 1-3-0-2 \
                   --task log-the-release",
            section: "releases",
            landed: "1-3-0-2",
        },
    },
    Cell {
        what: "doc author batch carrying an item title that mints a taken id",
        args: &["doc", "author", "changelog", "--from-file", "-"],
        stdin: Some(
            b"title: Changelog\nsections:\n  - id: releases\n    items:\n      - title: \"1.3.0\"\n",
        ),
        code: "write.already-present",
        route: RouteCheck::Collides {
            minted: "1-3-0",
            argv: "jigc doc add-item {addr}#releases --title 1.3.0 --slug 1-3-0-2 \
                   --task log-the-release",
            section: "releases",
            landed: "1-3-0-2",
        },
    },
    Cell {
        // The omitting context, driven rather than reasoned about: `#staged`'s `id-from`
        // is an **enum**, so the same slug-alike collision (`Changed` is live; `changed`
        // mints the same id) must NOT be handed a `--slug` — that override is refused as
        // an identity change, so the route would block when run.
        what: "add-item at a taken enum id-from member (the omitting context)",
        args: &["doc", "add-item", "{addr}#staged", "--title", "changed"],
        stdin: None,
        code: "write.already-present",
        route: RouteCheck::StaysHuman(
            "the target already exists — edit it in place (`set-field`/`set-slot`) \
             instead of re-creating it",
        ),
    },
];

#[test]
fn every_write_miss_names_its_own_miss_and_routes_the_recovery() {
    let fx = provision();
    let addr = format!("changelog:{}", fx.slug);
    let staged = fx.staged();
    let before = fs::read_to_string(&staged).expect("read the staged changelog");
    // Every cell is adjudicated, then the whole axis is reported at once — a per-cell
    // panic would hide the rest of the matrix behind the first broken row.
    let mut broken: Vec<String> = Vec::new();

    for cell in CELLS {
        let args: Vec<String> = cell
            .args
            .iter()
            .map(|arg| arg.replace("{addr}", &addr))
            .chain(["--format".to_owned(), "json".to_owned()])
            .collect();
        let out = fx.run(
            &args.iter().map(String::as_str).collect::<Vec<&str>>(),
            cell.stdin,
        );

        assert!(
            !out.status.success(),
            "`{}` must block (non-zero exit); stdout:\n{}\nstderr:\n{}",
            cell.what,
            String::from_utf8_lossy(&out.stdout),
            String::from_utf8_lossy(&out.stderr),
        );
        let stderr = String::from_utf8_lossy(&out.stderr);
        let report: serde_json::Value = serde_json::from_str(stderr.trim())
            .unwrap_or_else(|e| panic!("`{}` stderr is JSON: {e}; got:\n{stderr}", cell.what));
        let got = report["findings"][0]["code"].as_str().unwrap_or("<absent>");
        let named_its_miss = got == cell.code;

        // A **collision** must name the colliding minted id in its own message: an
        // `already-present` inside a batch is otherwise silent about WHICH of the
        // payload's items collided, and the whole payload is what got rejected.
        if let RouteCheck::Collides { minted, .. } = &cell.route {
            let message = report["findings"][0]["message"].as_str().unwrap_or("");
            assert!(
                message.contains(&format!("{minted:?}")),
                "`{}`: the reject names the colliding minted id {minted:?}; got:\n{message}",
                cell.what,
            );
        }
        if !named_its_miss {
            broken.push(format!(
                "  {}: expected `{}`, got `{}` (route: {})",
                cell.what,
                cell.code,
                got,
                report["findings"][0]["route"].as_str().unwrap_or("<none>"),
            ));
        }

        // **The route column.** An item-id miss must hand back the containing section's
        // `doc show`, with the doc's real slug and the resolved task id substituted — the
        // engine's `<address>` / `<task-id>` fallback reaching an agent is the dead end
        // this axis exists to close. Only adjudicated once the cell named its own miss:
        // a wrong-code cell has already been recorded, and its foreign route shape must
        // not panic the run and hide the rest of the matrix.
        //
        // The complement of the item-id column: a **shape** cell must route at the schema
        // read that answers a shape question — never at a `jigc doc show` of the very
        // section the address got wrong, which is the un-followable route (it exits 1) an
        // unranked declaredness check hands back. The CLI seam resolves the `<doctype>`
        // placeholder, so that route is run **verbatim** too: it must exit 0 and name the
        // doctype's real sections. And a **title-miss** cell's route is itself a write, so
        // it is proven followable on a private fixture in the same state (running it here
        // would move the bytes the next row diffs against).
        if named_its_miss {
            let route = report["findings"][0]["route"]
                .as_str()
                .unwrap_or_else(|| panic!("`{}` carries a route; got:\n{stderr}", cell.what));
            // The one non-command row shape: the omitting context, whose route must stay
            // the shipped human direction. Adjudicated on the WHOLE route string, before
            // `backticked` — which would panic on it, since a human route carries no
            // leading backticked command (that panic is exactly the red this column
            // opened with, and it must not fire on the row that is correct as it stands).
            if let RouteCheck::StaysHuman(expected) = cell.route {
                if route != expected {
                    broken.push(format!(
                        "  {}: expected the human route `{}`, got `{}`",
                        cell.what, expected, route
                    ));
                }
                let after = fs::read_to_string(&staged).expect("read the staged changelog");
                assert_eq!(
                    after, before,
                    "`{}` persisted nothing — the staged bytes are unchanged",
                    cell.what,
                );
                continue;
            }
            let cmd = backticked(route, cell.what);
            let expected = match cell.route {
                RouteCheck::Show(section) => {
                    format!("jigc doc show {addr}#{section} --task {TASK_ID}")
                }
                RouteCheck::Schema => "jigc doc schema changelog".to_owned(),
                RouteCheck::MutatingWrite(argv) => argv.replace("{addr}", &addr),
                RouteCheck::Collides { argv, .. } => argv.replace("{addr}", &addr),
                RouteCheck::StaysHuman(_) => unreachable!("adjudicated above"),
            };
            if cmd != expected {
                broken.push(format!(
                    "  {}: expected route `{}`, got `{}`",
                    cell.what, expected, cmd
                ));
            } else {
                match cell.route {
                    RouteCheck::Show(section) => {
                        // Followability: the emitted argv, run **verbatim**, must exit 0
                        // and print the section's live item ids — the address the agent
                        // should have used.
                        let shown = fx.run_route(cmd, cell.what);
                        let live = live_item(section);
                        assert!(
                            shown.contains(live),
                            "`{}`: the emitted route reveals the section's live item ids; got:\n{shown}",
                            cell.what,
                        );
                    }
                    RouteCheck::Schema => {
                        let shown = fx.run_route(cmd, cell.what);
                        assert!(
                            shown.contains("releases"),
                            "`{}`: the emitted route reveals the doctype's declared sections; got:\n{shown}",
                            cell.what,
                        );
                    }
                    RouteCheck::MutatingWrite(_) => {
                        provision().run_route(cmd, cell.what);
                    }
                    RouteCheck::Collides {
                        section, landed, ..
                    } => {
                        assert!(
                            cmd.contains(" --slug "),
                            "`{}`: the collision route names `--slug` — the flag that \
                             decouples the id from the title; got `{cmd}`",
                            cell.what,
                        );
                        // Run verbatim on a private fixture in the same state, then read
                        // the section back: the second item must have LANDED beside the
                        // first, not merely exited 0.
                        let private = provision();
                        private.run_route(cmd, cell.what);
                        let shown = private.run_route(
                            &format!("jigc doc show {addr}#{section} --task {TASK_ID}"),
                            cell.what,
                        );
                        for anchor in [live_item(section), landed] {
                            assert!(
                                shown.contains(&format!("{{#{anchor}}}")),
                                "`{}`: after the route ran, `{{#{anchor}}}` stands in \
                                 `#{section}`; got:\n{shown}",
                                cell.what,
                            );
                        }
                    }
                    RouteCheck::StaysHuman(_) => unreachable!("adjudicated above"),
                }
            }
        }

        let after = fs::read_to_string(&staged).expect("read the staged changelog");
        assert_eq!(
            after, before,
            "`{}` persisted nothing — the staged bytes are unchanged",
            cell.what,
        );
    }

    assert!(
        broken.is_empty(),
        "{} defect(s) across {} cells (the miss named, or the route handed back):\n{}",
        broken.len(),
        CELLS.len(),
        broken.join("\n"),
    );
}
