//! M50 Increment 2 / T2 — **the `--slug` override, at every door that takes one.**
//!
//! ## The class this closes
//!
//! A `--slug` value drives a minted identity **verbatim** — that is the point of the flag
//! — and a doc's slug *is* its path component (`<docs-root>/<location>/<slug>.md`). Five
//! doors have refused a value that is not a slug since M39. The sixth, `jigc rename`, did
//! not, and driven at `f04fea1`:
//!
//!   * `jigc rename adr:keeper --to "New Title" --slug '../../src/pwned'` exits **0** and
//!     commits `docs/decisions/keeper.md => src/pwned.md` — after which `jigc doc list`
//!     names no such doc, and `jigc validate` says nothing about it;
//!   * `jigc rename adr:keeper --to "New Title" --slug ''` blocks with
//!     `write.unslugable-title` — *"`--to \"New Title\"` slugs to nothing"* — which is
//!     false (that title slugs fine; the emptiness came from `--slug ""`), and routes the
//!     caller to *name the id yourself with `--slug`*, i.e. to repeat what just failed.
//!
//! (`DECISIONS.md` → 2026-09-05 M50 Increment 2 planning; `completions/artifacts/M50/`.)
//!
//! ## The axis is the shipped registry, not a list
//!
//! The subject is [`SLUG_DOORS`], **derived** from the clap tree by [`slug_arg_ids`] and
//! fenced ⇔ against it by `cli_parse::every_slug_door_is_registered`, so a seventh
//! `--slug` door cannot ship without joining it. Six doors reaching **four** separate
//! guards (`start`'s mint, `migrate`'s adoption, `doc.rs`'s shared reject over
//! `create`/`add-item`/`rename`, and — as of this task — `rename.rs`'s own), which is
//! exactly why the door set and not the function is the acceptance: a guard at any one of
//! them covers a quarter of the class.
//!
//! **A row is driven or the suite fails** — the driven set is compared back to the
//! registry and the count asserted, so a row cannot join and quietly run nothing.
//!
//! ## The cells
//!
//! Three ask *is this a slug at all*: the **empty string** (the cell that produced the
//! law-1 lie), a `../..` **traversal**, and an **absolute path that really exists** (the
//! fixture's own repository root). Each must block non-zero, name the token the caller
//! typed, and **state the grammar** — the one shipped sentence, so six doors give one
//! answer rather than six.
//!
//! The fourth asks the question the grammar cannot: a **300-byte slug** (M51 Increment 9 /
//! T3, EC-28). It *is* a slug — the grammar has nothing to say about it — and a slug is
//! what jigc turns into a filesystem path component, so three doors hit the OS name
//! ceiling and told the caller a story about a disk. Driven at `422032b6`, per door in its
//! own fresh fixture:
//!
//!   * `jigc start --workflow single-task "x" --slug <300>` → `blocking ·
//!     task.working-area-io … File name too long (os error 63)`, routed *"resolve the
//!     underlying I/O condition (a disk or permissions problem on the `.jigc/` task
//!     working area)"* — **there is no disk or permissions problem**;
//!   * `jigc doc create adr --title Axis --slug <300>` → the same code, the same false
//!     route, keyed at `task:adr:<300>` (an address, not a task id);
//!   * `jigc doc rename adr:keeper --to Axis --slug <300>` → the OS error bare, with no
//!     code and no route at all (`completions/artifacts/M51/baseline-tokens.md` → row 12);
//!   * `jigc migrate foreign-changelog.md --as changelog --slug <300>` → **exit 0**, a
//!     task minted, the override inert because the target is a singleton — the cell the
//!     baseline recorded as *masked*, and the reason the ceiling is asked at **every** row
//!     and not at the three that fail loudly.
//!
//! So the ceiling cell asserts three things at every row: the code
//! [`NAME_CEILING`], a route naming the ceiling, and that **no** door answers
//! [`FALSE_IO_CODE`] — plus, on every cell of every row, that the working area is
//! **unchanged**, which is what catches a door that refuses by minting first.
//!
//! ## Why the tree arm exists
//!
//! The text alone passed at `f04fea1`'s exit 0 for the sixth door, so one arm asserts the
//! **tree**: a traversal override must land no commit and leave the doc at its own slug,
//! addressable by the surfaces that name it.

use std::collections::BTreeSet;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

use cli::cli::{SLUG_DOOR_SOURCE, SLUG_DOORS, SLUG_OVERRIDE_SLOT, slug_arg_ids};

/// The grammar sentence every `--slug` refusal states — the **one** shipped literal
/// (`crate::task::WORK_UNIT_ID_GRAMMAR`, fenced against `design/structural-grammar.md` by
/// `malformed_work_unit_id::the_design_doc_states_the_shipped_grammar_verbatim`). Spelled
/// out rather than imported: a test comparing emitted bytes against the constant that
/// produced them proves only that the constant equals itself.
const GRAMMAR: &str =
    "use lowercase letters, digits, and single hyphens (no leading, trailing, or doubled `-`)";

/// The code the **ceiling** cell answers with at every row (M51 Increment 9 / T3, EC-28).
const NAME_CEILING: &str = "write.slug-name-ceiling";

/// The code no row may answer any longer: the mint I/O fault whose route blames *"a disk or
/// permissions problem"*, which is a law-1 lie about a value the door could have adjudicated
/// itself. Asserted absent on **every** cell of **every** row, not only the three that
/// produced it, because the whole point of a door predicate is that the I/O never happens.
const FALSE_IO_CODE: &str = "task.working-area-io";

/// The ceiling itself, in bytes — `cli::cli::SLUG_NAME_CEILING`, spelled out rather than
/// imported for [`GRAMMAR`]'s reason: a test comparing emitted bytes against the constant
/// that produced them proves only that the constant equals itself. A change to the
/// derivation is a change to what every `--slug` door accepts, so it must be read here, not
/// absorbed.
const CEILING_BYTES: usize = 165;

/// The committed `adr` the `rename` / `doc rename` / `add-item` rows address.
const KEEPER: &str = "docs/decisions/keeper.md";

/// A throwaway directory that removes itself on drop.
struct TempDir(PathBuf);

impl TempDir {
    fn new(tag: &str) -> Self {
        let mut path = std::env::temp_dir();
        path.push(format!(
            "jigc-slug-override-axis-{tag}-{}-{:?}",
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

/// A conformant committed `adr` — the doc every address-carrying row names.
const ADR_KEEPER: &str = "\
---
status: accepted
date: 2026-06-28
schema-version: 1
---

# Keeper

## Context

Context.

## Options

Alternatives were weighed and rejected.

## Decision

Decided.

## Consequences

Effects.
";

/// The foreign source `jigc migrate` adopts, so that row's door answers about the
/// override and not about a missing file.
const FOREIGN_CHANGELOG: &str = "# Change Log\n\n## v1\n\n- did a thing\n";

/// A set-up repo carrying a committed `adr`, the foreign migrate source, and one open
/// task (the `doc` rows run under the shipped single-active-task default).
struct Fixture {
    repo: TempDir,
    home: TempDir,
}

impl Fixture {
    fn new(tag: &str) -> Self {
        let fixture = Fixture {
            repo: TempDir::new(&format!("repo-{tag}")),
            home: TempDir::new(&format!("home-{tag}")),
        };
        let repo = fixture.repo.path();
        fixture.git(&["init", "-q"]);
        fixture.git(&["config", "user.email", "test@example.com"]);
        fixture.git(&["config", "user.name", "Test"]);
        fs::create_dir_all(repo.join("docs/decisions")).expect("mk decisions");
        fs::write(repo.join(KEEPER), ADR_KEEPER).expect("write the keeper adr");
        fs::write(repo.join(SLUG_DOOR_SOURCE), FOREIGN_CHANGELOG).expect("write the source");
        fs::write(repo.join("README.md"), "hello\n").expect("write README");
        // Driven fixture fact: the traversal's destination directory must exist on disk,
        // or the escape is refused for an unrelated reason (macOS resolves `..`
        // physically) and the tree arm proves nothing — at `f04fea1` with `src/` present
        // the same call exits 0 and commits `docs/decisions/keeper.md => src/pwned.md`.
        fs::create_dir_all(repo.join("src")).expect("mk src");
        fs::write(repo.join("src/module.md"), "source-tree prose\n").expect("write src");
        fixture.git(&["add", "."]);
        fixture.git(&["commit", "-q", "-m", "initial"]);
        fixture.ok(&["setup"]);
        fixture.ok(&["start", "--workflow", "single-task", "axis intent"]);
        fixture
    }

    fn git(&self, args: &[&str]) -> String {
        let out = Command::new("git")
            .args(args)
            .current_dir(self.repo.path())
            .output()
            .expect("run git");
        assert!(
            out.status.success(),
            "git {args:?} failed: {}",
            String::from_utf8_lossy(&out.stderr),
        );
        String::from_utf8_lossy(&out.stdout).trim().to_string()
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

/// **What a cell asks**, and therefore what the door owes back. The two questions are
/// genuinely different — one is *is this a slug*, the other *is this slug short enough to
/// be a filename* — so a single `contains(GRAMMAR)` over both would let the ceiling cell
/// pass on a grammar refusal that is false about the value.
enum Cell {
    /// A token the slug grammar rejects. The door owes the one shipped grammar sentence.
    NotASlug(String),
    /// A well-formed slug longer than the ceiling. The door owes [`NAME_CEILING`] and a
    /// route naming the ceiling.
    OverCeiling(String),
}

impl Cell {
    fn token(&self) -> &str {
        match self {
            Cell::NotASlug(t) | Cell::OverCeiling(t) => t.as_str(),
        }
    }

    /// The cell's spelling in a failure message — the 300-byte token is elided, because a
    /// 300-character assertion header hides the message it is meant to introduce.
    fn shown(&self) -> String {
        match self {
            Cell::NotASlug(t) => format!("{t:?}"),
            Cell::OverCeiling(t) => format!("<{} bytes>", t.len()),
        }
    }
}

/// Every path under the task working area, relative to it — the state a refusal must leave
/// untouched. Empty when `.jigc/tasks` does not exist.
fn working_area(fixture: &Fixture) -> BTreeSet<PathBuf> {
    let root = fixture.path().join(".jigc").join("tasks");
    let mut seen = BTreeSet::new();
    let mut stack = vec![root.clone()];
    while let Some(dir) = stack.pop() {
        let Ok(entries) = fs::read_dir(&dir) else {
            continue;
        };
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                stack.push(path.clone());
            }
            if let Ok(rel) = path.strip_prefix(&root) {
                seen.insert(rel.to_path_buf());
            }
        }
    }
    seen
}

/// Both streams of one invocation — the surface a reader meets.
fn surface(out: &std::process::Output) -> String {
    format!(
        "{}{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr),
    )
}

/// **The axis** — every registered `--slug` door, over the whole malformed-override cell
/// space.
#[test]
fn every_slug_door_refuses_a_malformed_override() {
    let fixture = Fixture::new("axis");
    let absolute = fixture.path().to_string_lossy().into_owned();
    let cells: Vec<Cell> = vec![
        Cell::NotASlug(String::new()),
        Cell::NotASlug("../..".to_string()),
        Cell::NotASlug(absolute),
        // A well-formed slug, 300 bytes long — the cell the grammar has nothing to say
        // about. 300 rather than `CEILING_BYTES + 1` on purpose: it is the token the
        // baseline drove, and it clears the OS ceiling by enough that a door which let it
        // through would fail on the filesystem, which is the failure this cell forbids.
        Cell::OverCeiling("a".repeat(300)),
    ];

    let mut driven: BTreeSet<Vec<&str>> = BTreeSet::new();
    for row in SLUG_DOORS {
        for cell in &cells {
            let token = cell.token();
            let argv: Vec<&str> = row
                .argv
                .iter()
                .map(|arg| {
                    if *arg == SLUG_OVERRIDE_SLOT {
                        token
                    } else {
                        *arg
                    }
                })
                .collect();
            let shown = format!("jigc {} [{}]", row.door.join(" "), cell.shown());
            // The working area before the call — a refusal that mints first is a refusal
            // that leaves a task dir to resume from, which the `migrate` row did at exit 0.
            let before = working_area(&fixture);
            let out = fixture.run(&argv);
            let text = surface(&out);
            assert_ne!(
                out.status.code(),
                Some(101),
                "{shown}: the refusal must be a finding or an error, never a panic\n{text}",
            );
            assert!(
                !out.status.success(),
                "{shown}: an override jigc cannot mint an identity from must block \
                 non-zero\n{text}",
            );
            // Asked before the echo assertion below: when a door still answers the I/O
            // story, that is the finding to read, not the spelling of the token in it.
            assert!(
                !text.contains(FALSE_IO_CODE),
                "{shown}: `{FALSE_IO_CODE}` blames a disk or permissions problem for a \
                 value the door could adjudicate itself — no row may answer it\n{text}",
            );
            assert!(
                text.contains(&format!("{token:?}")),
                "{shown}: must name the token the caller typed\n{text}",
            );
            match cell {
                Cell::NotASlug(_) => assert!(
                    text.contains(GRAMMAR),
                    "{shown}: must state the one shipped grammar sentence\n{text}",
                ),
                Cell::OverCeiling(_) => {
                    assert!(
                        text.contains(NAME_CEILING),
                        "{shown}: an over-long slug is a slug — it must answer \
                         `{NAME_CEILING}`, never the grammar and never an I/O story\n{text}",
                    );
                    assert!(
                        text.contains(&CEILING_BYTES.to_string()),
                        "{shown}: the route must name the ceiling ({CEILING_BYTES} bytes), \
                         so the caller knows what to re-run with\n{text}",
                    );
                }
            }
            // The tree, not only the text: at `f04fea1` the sixth door's traversal cell
            // committed the doc out of the store while printing a success line.
            assert!(
                fixture.path().join(KEEPER).is_file(),
                "{shown}: the committed doc must stay at its own slug\n{text}",
            );
            assert_eq!(
                working_area(&fixture),
                before,
                "{shown}: a refused override must leave the working area untouched — no \
                 task dir minted, nothing staged\n{text}",
            );
        }
        driven.insert(row.door.to_vec());
    }

    assert_eq!(
        driven.len(),
        SLUG_DOORS.len(),
        "every registered `--slug` door is driven exactly once — a row with no cell is a \
         failure, never a skip",
    );
    assert_eq!(
        SLUG_DOORS.len(),
        6,
        "the registry ships six `--slug` doors; a change to that count is a change to this \
         class's axis and must be read, not absorbed",
    );
    assert!(
        !slug_arg_ids().is_empty(),
        "the derivation vocabulary must be non-empty, else the ⇔ fence is vacuous",
    );
}

/// **The tree arm.** A traversal override at `jigc rename` — the committing door, and the
/// one that shipped the defect — must land no commit and leave the store addressable.
#[test]
fn a_traversal_override_commits_nothing_out_of_the_docs_root() {
    let fixture = Fixture::new("tree");
    // The rename door refuses a dirty tree, and the axis fixture's `jigc start` leaves the
    // working area gitignored but the task in flight — discard it so this arm reaches the
    // slug guard rather than the in-flight guard.
    fixture.ok(&["task", "discard", "axis-intent", "--force"]);
    let before = fixture.git(&["rev-parse", "HEAD"]);

    let out = fixture.run(&[
        "rename",
        "adr:keeper",
        "--to",
        "Pwned",
        "--slug",
        "../../src/pwned",
    ]);
    let text = surface(&out);
    assert!(!out.status.success(), "the traversal must block\n{text}");
    assert_eq!(
        fixture.git(&["rev-parse", "HEAD"]),
        before,
        "a refused rename must land no commit\n{text}",
    );
    assert!(
        fixture.path().join(KEEPER).is_file(),
        "the doc must still be at its own slug\n{text}",
    );
    assert!(
        !fixture.path().join("src").join("pwned.md").exists(),
        "nothing may be written outside the docs root\n{text}",
    );
    let listing = fixture.ok(&["doc", "list", "adr"]);
    assert!(
        listing.contains("adr:keeper"),
        "the doc must stay addressable by the surfaces that name it\n{listing}",
    );
}

/// **The ceiling is a boundary, not a blanket.** A slug of exactly [`CEILING_BYTES`] bytes
/// must still mint — and the write it drives must actually land on disk, which is the half
/// a refusal-only axis cannot prove: the number is derived from what jigc wraps around a
/// minted id, so a derivation that reserved too little would pass every refusal assertion
/// above and fail here with the very `os error 63` the predicate exists to prevent.
///
/// Driven at one door (`jigc doc create adr`) rather than six: the claim is about the
/// *number*, and the number is one constant every door reads.
#[test]
fn a_slug_at_the_ceiling_still_mints_and_one_byte_over_does_not() {
    let fixture = Fixture::new("boundary");
    let at_ceiling = "a".repeat(CEILING_BYTES);
    let over = "a".repeat(CEILING_BYTES + 1);

    let ok = fixture.run(&[
        "doc",
        "create",
        "adr",
        "--title",
        "Axis",
        "--slug",
        &at_ceiling,
    ]);
    let ok_text = surface(&ok);
    assert!(
        ok.status.success(),
        "a {CEILING_BYTES}-byte slug is at the ceiling, not over it — the door must mint \
         it\n{ok_text}",
    );
    assert!(
        fixture
            .path()
            .join(".jigc/tasks/axis-intent/docs")
            .join(format!("adr:{at_ceiling}.md"))
            .is_file(),
        "the staged instance must exist on disk — the ceiling reserves room for everything \
         jigc wraps around the slug, including the atomic write's temp sibling\n{ok_text}",
    );

    // A fresh fixture for the over cell: the create above bound this task's `decision`
    // role, and a second create in the same task refuses on *that* (`write.identity-change`)
    // before any slug is looked at — an arm that shared one fixture would assert the
    // ceiling against a refusal the ceiling had nothing to do with.
    let second = Fixture::new("boundary-over");
    let refused = second.run(&["doc", "create", "adr", "--title", "Axis", "--slug", &over]);
    let refused_text = surface(&refused);
    assert!(
        !refused.status.success(),
        "one byte over the ceiling must block\n{refused_text}",
    );
    assert!(
        refused_text.contains(NAME_CEILING),
        "one byte over the ceiling must answer `{NAME_CEILING}`\n{refused_text}",
    );
}
