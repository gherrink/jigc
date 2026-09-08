//! M50 Increment 2 / T2 — **the `--slug` override, at every door that takes one.**
//!
//! ## The class this closes
//!
//! A `--slug` value drives a minted identity **verbatim** — that is the point of the flag
//! — and a doc's slug *is* its path component (`<docs-root>/<location>/<slug>.md`). Five
//! doors have refused a value that is not a slug since M39. The sixth, `jigc rename`, did
//! not, and driven at `b32def1`:
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
//! ## The three cells
//!
//! The **empty string** (the cell that produced the law-1 lie), a `../..` **traversal**,
//! and an **absolute path that really exists** (the fixture's own repository root). Each
//! must block non-zero, name the token the caller typed, and **state the grammar** — the
//! one shipped sentence, so six doors give one answer rather than six.
//!
//! ## Why the tree arm exists
//!
//! The text alone passed at `b32def1`'s exit 0 for the sixth door, so one arm asserts the
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
        // physically) and the tree arm proves nothing — at `b32def1` with `src/` present
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
    let cells: Vec<String> = vec![String::new(), "../..".to_string(), absolute];

    let mut driven: BTreeSet<Vec<&str>> = BTreeSet::new();
    for row in SLUG_DOORS {
        for token in &cells {
            let argv: Vec<&str> = row
                .argv
                .iter()
                .map(|arg| {
                    if *arg == SLUG_OVERRIDE_SLOT {
                        token.as_str()
                    } else {
                        *arg
                    }
                })
                .collect();
            let shown = format!("jigc {} [{token}]", row.door.join(" "));
            let out = fixture.run(&argv);
            let text = surface(&out);
            assert_ne!(
                out.status.code(),
                Some(101),
                "{shown}: the refusal must be a finding or an error, never a panic\n{text}",
            );
            assert!(
                !out.status.success(),
                "{shown}: an override that is not a slug names a path nobody can address \
                 — the door must block non-zero\n{text}",
            );
            assert!(
                text.contains(&format!("{token:?}")),
                "{shown}: must name the token the caller typed\n{text}",
            );
            assert!(
                text.contains(GRAMMAR),
                "{shown}: must state the one shipped grammar sentence\n{text}",
            );
            // The tree, not only the text: at `b32def1` the sixth door's traversal cell
            // committed the doc out of the store while printing a success line.
            assert!(
                fixture.path().join(KEEPER).is_file(),
                "{shown}: the committed doc must stay at its own slug\n{text}",
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
