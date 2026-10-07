//! **One code for a managed home that is not an ordinary file** — the human's ruling of
//! 2026-10-06 on the rc.24 fix pass's items 7 and 8 (`DECISIONS.md`; `design/finalize.md` →
//! 4. Promote; `design/command-output-contract.md` → the form table, the file row).
//!
//! The rc.24 fix pass taught every door that writes, moves or mints a managed doc to ask
//! what the entry at the doc's home *is* (`engine::store::home_entry`) instead of following
//! a link. Each fixer then answered the shape — a symbolic link, a directory, a special
//! file — under the code its door already had for *something is in the way*, and the pass
//! ended with **one state under six codes at fifteen commands** — fourteen as the ruling
//! counted them, `doc author` sharing `doc create`'s gate:
//!
//! | code, as built | commands |
//! |---|---|
//! | `finalize.promote-clobber` | `task finalize`, `milestone finalize`, `rename` (the doc's own home, a referrer's), `relocate`, `config set docs-root`, `config set placement-root` |
//! | `write.already-present` | `rename` (the destination), `doc rename` |
//! | `create.already-exists` | `doc create`, `doc author` under a create-only entry |
//! | `reconciliation.conflict-block` | `milestone add-task`, `milestone add-from-spec`, `milestone discard`, a sub-task's `task discard`, `milestone finalize` (the record) |
//! | `milestone.record-exists` | `milestone create` |
//! | `migrate-corpus.destination-collision` | `migrate-corpus` (a relocation's destination) |
//!
//! They all answer **`store.home-not-regular-file`** now, keyed at the entry's path, through
//! one constructor (`engine::store::home_shape_refusal`), and each of the six is back to the
//! state it was minted for.
//!
//! ## What is fenced, and by what kind of set
//!
//! 1. **Every production caller of the constructor is a row** ([`SITES`]), found by scanning
//!    the source, and each row names the commands that reach it. A further caller reddens
//!    until it is a row.
//! 2. **Every command a row names is driven** ([`CELLS`]) on the built binary, with a link at
//!    the home: it refuses, under the one code, naming the entry, with none of the six
//!    retired codes anywhere in what it says, and the entry stands as it stood. A command
//!    with no cell reddens, and so does a cell for a command no row names.
//! 3. **Every production caller of the probe is dispositioned** ([`OBSERVERS`]) — of
//!    `engine::store::home_entry`, and of `foreign_home_entry`, its accessor for the one
//!    shape question: it reaches the constructor, or it is out with its reason. This is the
//!    arm that catches a command which asks the probe and then answers *something else* —
//!    which is how one state came to have six codes. The subject is the probe's callers and
//!    not a pattern they might match on, because a caller that compares the answer with
//!    `RegularFile` learns the same thing as one that matches on `Foreign`.
//! 4. **The code is spelled once in production** — the constant beside the constructor.
//!
//! Arms 1, 3 and 4 are scans, so they are finders held to a table in both directions; arm 2
//! is the guarantee, and what it iterates is derived from arm 1's rows.
//!
//! **The must-not-refuse cells** sit in the last test: an ordinary file at the home, a doc
//! legitimately absent, and a placement home — none of which may meet this code.
//!
//! **`jigc migrate-corpus` meets the shape at two homes** — a relocation's destination, and
//! the home of a doc it would rewrite in place or move (the fix pass's item 9, ruled on
//! 2026-10-07: until then the second replaced the link at exit 0). Each refuses that one
//! doc; the run's other docs still migrate
//! (`store_door_home_shape::the_corpus_migration_refuses_a_linked_doc_and_migrates_the_rest`).

#![cfg(unix)]

use crate::support;

use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Output, Stdio};

use support::rust_source;
use support::trial_corpus::{State, TrialCorpus};

/// The one code.
const CODE: &str = engine::store::HOME_NOT_REGULAR_FILE;

/// The six codes the state was answered under until 2026-10-06. Each still exists, for the
/// state it was minted for; none may appear in a refusal over a home that is not a file.
const RETIRED: [&str; 6] = [
    "finalize.promote-clobber",
    "write.already-present",
    "create.already-exists",
    "reconciliation.conflict-block",
    "milestone.record-exists",
    "migrate-corpus.destination-collision",
];

// ── arm 1: the constructor's callers ─────────────────────────────────────────────────

/// **Every production caller of `engine::store::home_shape_refusal`**, as
/// `(file, enclosing fn, the commands that reach it)`.
///
/// The commands are the leaf verbs a user types; `config set` is spelled with the knob,
/// because the two roots move different homes. A row's commands are what arm 2 drives.
const SITES: &[(&str, &str, &[&str])] = &[
    (
        // The committing doors' planner at both boundaries, the promote sink behind them,
        // and the store doors that rewrite or move a committed doc where it stands
        // (`store_home_refusal`: `jigc rename` at the doc's own home and at a referrer's,
        // the relocation primitive behind `relocate` and both root knobs, and
        // `jigc migrate-corpus` at the home of a doc it would rewrite or move).
        "crates/engine/src/finalize.rs",
        "shape_refusal",
        &[
            "task finalize",
            "milestone finalize",
            "rename",
            "relocate",
            "config set docs-root",
            "config set placement-root",
            "migrate-corpus",
        ],
    ),
    (
        "crates/cli/src/rename.rs",
        "foreign_destination",
        &["rename"],
    ),
    ("crates/cli/src/doc.rs", "free_destination", &["doc rename"]),
    (
        "crates/engine/src/state.rs",
        "create_only_home_refusal",
        &["doc create", "doc author"],
    ),
    (
        // The one preflight the five record doors share.
        "crates/cli/src/milestone.rs",
        "record_shape_block",
        &[
            "milestone add-task",
            "milestone add-from-spec",
            "milestone discard",
            "task discard",
            "milestone finalize",
        ],
    ),
    (
        "crates/engine/src/milestone.rs",
        "record_home_taken_finding",
        &["milestone create"],
    ),
    (
        "crates/cli/src/migrate_corpus.rs",
        "destination_collision_finding",
        &["migrate-corpus"],
    ),
];

// ── arm 3: the probe's observers ─────────────────────────────────────────────────────

/// What a site that observes a foreign entry does with it.
#[derive(Clone, Copy)]
enum Observed {
    /// It refuses through the constructor — directly, or by handing the shape to the named
    /// [`SITES`] row.
    Refuses(&'static str),
    /// It does not answer the one code, for the stated reason.
    Out(&'static str),
}

/// **Every production fn that calls `engine::store::home_entry` or `foreign_home_entry`**
/// — the sites that can learn an entry is not a regular file. Not every one asks it of a
/// managed doc's home: the probe is also the install's no-follow read, and those rows say
/// so.
const OBSERVERS: &[(&str, &str, Observed)] = &[
    (
        "crates/engine/src/store.rs",
        "home_entry",
        Observed::Out("the probe itself: it produces the answer every other row reads"),
    ),
    (
        "crates/engine/src/store.rs",
        "rewrite_home",
        Observed::Out(
            "the write's own backstop behind a door that has already asked and refused: it \
             fires only where the entry changed between the door's question and the write, \
             and answers an I/O error the door wraps — no finding is minted here",
        ),
    ),
    (
        "crates/engine/src/finalize.rs",
        "refused_promotions",
        Observed::Refuses("shape_refusal"),
    ),
    (
        "crates/engine/src/state.rs",
        "foreign_home_entry",
        Observed::Out(
            "the accessor for the one question *is this home's entry not a regular file?* — \
             it answers its callers, and each of them is a row",
        ),
    ),
    (
        "crates/engine/src/state.rs",
        "occupied_home",
        Observed::Refuses(
            "create_only_home_refusal — under a create-only entry; a plain create reads a \
             live link's body and mints fresh over the rest, and is refused where the write \
             would happen (`design/finalize.md` → 4. Promote, declared bound 4)",
        ),
    ),
    (
        "crates/cli/src/task.rs",
        "promote",
        Observed::Refuses("shape_refusal"),
    ),
    (
        "crates/cli/src/rename.rs",
        "run",
        Observed::Refuses("shape_refusal · foreign_destination"),
    ),
    (
        "crates/cli/src/relocate.rs",
        "refuse_foreign_source",
        Observed::Refuses("shape_refusal"),
    ),
    (
        "crates/cli/src/milestone.rs",
        "reconcile_record_preflight",
        Observed::Refuses("record_shape_block"),
    ),
    (
        "crates/cli/src/milestone.rs",
        "guard_record_free",
        Observed::Refuses("record_home_taken_finding"),
    ),
    (
        "crates/cli/src/migrate_corpus.rs",
        "destination_occupant",
        Observed::Refuses("destination_collision_finding"),
    ),
    (
        // The one seam through which a doc becomes a doc the run will write.
        "crates/cli/src/migrate_corpus.rs",
        "queue_or_refuse",
        Observed::Refuses("shape_refusal"),
    ),
    (
        "crates/cli/src/migrate_corpus.rs",
        "unlanded_paths",
        Observed::Out(
            "the recovery audit: it asks whether the worktree holds an earlier run's \
             migrated result, and an entry that is not a regular file is not one — it is \
             left out of the run's commit, and nothing is refused, because nothing of the \
             doc's is written",
        ),
    ),
    (
        "crates/cli/src/doc.rs",
        "free_destination",
        Observed::Refuses("free_destination"),
    ),
    (
        "crates/cli/src/doc.rs",
        "title_pre_check",
        Observed::Refuses("create_only_home_refusal"),
    ),
    (
        "crates/cli/src/relocate.rs",
        "displace_foreign_squatter",
        Observed::Out(
            "a relocation's **destination**: whatever squats it — a link included — is \
             parked in the gitignored workbench as the entry it is, and the doc lands. A \
             displacement the ack names, not a refusal (`design/reconciliation.md` → \
             Relocation collisions)",
        ),
    ),
    (
        "crates/cli/src/setup.rs",
        "stamp_standing",
        Observed::Out(
            "`.jigc/version`, jigc's own stamp — an install member, not a managed doc's home",
        ),
    ),
    (
        "crates/cli/src/setup.rs",
        "version_stamp_is_jigcs",
        Observed::Out("`.jigc/version` again: the stamp is jigc's only as a regular file"),
    ),
    (
        "crates/cli/src/setup.rs",
        "read_settings_record",
        Observed::Out(
            "`.jigc/settings-entries.json`, the install's record — read only as the regular \
             file jigc wrote, and anything else is no claim",
        ),
    ),
    (
        "crates/cli/src/setup.rs",
        "recognises",
        Observed::Out(
            "an install member's path at teardown: jigc's own bytes are recognised in a \
             regular file only",
        ),
    ),
    (
        "crates/cli/src/regular_file.rs",
        "blocker",
        Observed::Out(
            "an install member's path (`.jigc/AGENT.md`, `.jigc/version` …), not a managed \
             doc's home: `jigc setup`'s replacing writers refuse it under their own \
             `setup.*` codes, and `--force` does not pass it",
        ),
    ),
    (
        "crates/cli/src/setup.rs",
        "guide_ownership",
        Observed::Out(
            "an installed skill artifact's path, not a managed doc's home: a link there is \
             the user's entry, left alone and reported like an edited copy",
        ),
    ),
];

fn workspace_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(Path::parent)
        .expect("crates/cli sits two levels under the workspace root")
        .to_path_buf()
}

/// Every production `(file, enclosing fn)` in which one of `needles` occurs as code.
fn production_sites(needles: &[&str]) -> BTreeSet<(String, String)> {
    let root = workspace_root();
    let mut sites = BTreeSet::new();
    for dir in ["crates/engine/src", "crates/cli/src"] {
        for path in rust_source::rust_files(&root.join(dir)) {
            let body = fs::read_to_string(&path).expect("read a source file");
            let code = rust_source::code_only(&body);
            let regions = rust_source::cfg_test_regions(&code);
            let rel = path
                .strip_prefix(&root)
                .expect("under the workspace")
                .to_string_lossy()
                .into_owned();
            for needle in needles {
                for (at, _) in code.match_indices(needle) {
                    if rust_source::is_test_domain(&path, &regions, at) {
                        continue;
                    }
                    let Some(enclosing) = rust_source::enclosing_fn(&code, at) else {
                        continue;
                    };
                    sites.insert((rel.clone(), enclosing.to_owned()));
                }
            }
        }
    }
    sites
}

/// **Arm 1.** The constructor's production callers are exactly [`SITES`].
#[test]
fn every_production_caller_of_the_constructor_is_a_row() {
    let mut scanned = production_sites(&["home_shape_refusal("]);
    // The definition is not a call of itself.
    scanned.remove(&(
        "crates/engine/src/store.rs".to_owned(),
        "home_shape_refusal".to_owned(),
    ));
    let declared: BTreeSet<(String, String)> = SITES
        .iter()
        .map(|(file, enclosing, _)| ((*file).to_owned(), (*enclosing).to_owned()))
        .collect();
    assert_eq!(
        scanned, declared,
        "`engine::store::home_shape_refusal` is the one refusal over a managed home that is \
         not a regular file. A caller the table does not carry is a command nobody drives; \
         a row with no caller is a claim about code that is gone. Add the row, with the \
         commands that reach it, and a cell for each in `CELLS`.",
    );
    for (file, enclosing, commands) in SITES {
        assert!(
            !commands.is_empty(),
            "{file}::{enclosing} names the commands that reach it"
        );
    }
}

/// **Arm 3.** Every production caller of the probe is dispositioned — and one that refuses
/// names a row of [`SITES`]. (`foreign_home_entry(` contains the needle, so the accessor's
/// callers are found by it too.)
#[test]
fn every_caller_of_the_probe_is_dispositioned() {
    let scanned = production_sites(&["home_entry("]);
    let declared: BTreeSet<(String, String)> = OBSERVERS
        .iter()
        .map(|(file, enclosing, _)| ((*file).to_owned(), (*enclosing).to_owned()))
        .collect();
    assert_eq!(
        scanned, declared,
        "a site that asks what the entry at a path is either refuses a managed home that is \
         not a regular file through `engine::store::home_shape_refusal`, or says why it \
         does not. One state came to have six codes because each new site answered with \
         the code it had to hand; a site the table does not carry is where a seventh would \
         start.",
    );
    let rows: BTreeSet<&str> = SITES.iter().map(|(_, enclosing, _)| *enclosing).collect();
    for (file, enclosing, observed) in OBSERVERS {
        match observed {
            Observed::Refuses(through) => assert!(
                rows.iter().any(|row| through.contains(row)),
                "{file}::{enclosing} says it refuses through `{through}`, which names no \
                 row of `SITES`",
            ),
            Observed::Out(reason) => assert!(
                !reason.trim().is_empty(),
                "{file}::{enclosing} is out of the class with no stated reason",
            ),
        }
    }
}

/// **Arm 4.** The code is spelled once in production code — the constant beside the
/// constructor. Every other producer reads the constant, so a renamed code cannot leave a
/// registry quietly matching nothing, and no door can mint the string around the
/// constructor.
#[test]
fn the_code_is_spelled_once_in_production() {
    let root = workspace_root();
    let mut spelled = Vec::new();
    for dir in ["crates/engine/src", "crates/cli/src"] {
        for path in rust_source::rust_files(&root.join(dir)) {
            let body = fs::read_to_string(&path).expect("read a source file");
            let code = rust_source::code_and_strings(&body);
            let regions = rust_source::cfg_test_regions(&code);
            for (at, _) in code.match_indices(&format!("\"{CODE}\"")) {
                if !rust_source::is_test_domain(&path, &regions, at) {
                    spelled.push(
                        path.strip_prefix(&root)
                            .expect("under the workspace")
                            .to_string_lossy()
                            .into_owned(),
                    );
                }
            }
        }
    }
    assert_eq!(
        spelled,
        vec!["crates/engine/src/store.rs".to_owned()],
        "`{CODE}` is a string literal in one place: `engine::store::HOME_NOT_REGULAR_FILE`",
    );
}

// ── arm 2: the commands, driven ──────────────────────────────────────────────────────

fn text(out: &Output) -> String {
    format!(
        "exit {:?}\n--- stdout ---\n{}\n--- stderr ---\n{}",
        out.status.code(),
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr),
    )
}

/// An entry planted at a home: where it is, and what it was when it was planted.
struct Planted {
    /// The home, repo-relative — the refusal's key.
    home: String,
    /// The entry, absolute.
    entry: PathBuf,
    /// The link's own text.
    link: PathBuf,
}

/// A **dangling** link at `rel` — nothing reachable through the home.
fn dangling(repo: &Path, rel: &str) -> Planted {
    let entry = repo.join(rel);
    fs::create_dir_all(entry.parent().expect("a parent")).expect("mk the home's directory");
    std::os::unix::fs::symlink("nowhere.md", &entry).expect("plant a dangling link");
    Planted {
        home: rel.to_owned(),
        link: fs::read_link(&entry).expect("the link"),
        entry,
    }
}

/// Move the doc at `rel` to `elsewhere/` and leave a relative **live** link to it at its
/// home — the state a third party leaves a committed doc's home in.
fn link_out(repo: &Path, rel: &str) -> Planted {
    let entry = repo.join(rel);
    let name = Path::new(rel).file_name().expect("a file name");
    let target = repo.join("elsewhere").join(name);
    fs::create_dir_all(target.parent().expect("a parent")).expect("mk elsewhere/");
    fs::rename(&entry, &target).expect("move the doc out from under its home");
    let up = "../".repeat(Path::new(rel).components().count() - 1);
    std::os::unix::fs::symlink(format!("{up}elsewhere/{}", name.to_string_lossy()), &entry)
        .expect("link the home at the moved doc");
    Planted {
        home: rel.to_owned(),
        link: fs::read_link(&entry).expect("the link"),
        entry,
    }
}

/// What a cell hands back: proof that its command was asked, and answered the one code.
/// Only [`answers_the_one_code`] builds one, so a cell cannot pass without asserting.
struct Answered(());

/// **The assertion every cell ends in.** `out` is the refused command's output.
///
/// - it **refused**: a non-zero exit — or, at a triage door that reports a blocked row and
///   classifies the rest (`at_exit_zero`), the row;
/// - under **the one code**, and **naming the entry**: on the refusal's own `at:` line
///   where the door prints the finding itself (`keyed`), in the text where it folds the
///   finding into a report of its own;
/// - with **none of the six retired codes** anywhere in what it said;
/// - and the entry **stands as it was planted** — a link, pointing where it pointed.
fn answers_the_one_code(what: &str, out: &Output, planted: &Planted, shape: Shape) -> Answered {
    let said = text(out);
    if shape.at_exit_zero {
        assert!(
            said.contains("blocked"),
            "{what}: the triage door reports the row as blocked; {said}"
        );
    } else {
        assert!(!out.status.success(), "{what}: the command refuses; {said}");
    }
    assert!(
        said.contains(CODE),
        "{what}: under `{CODE}`, the store's one code for the state; {said}",
    );
    if shape.keyed {
        assert!(
            said.contains(&format!("· {CODE} — "))
                && said.contains(&format!("at: {}", planted.home)),
            "{what}: the refusal is the finding itself, keyed at the entry's path \
             (`at: {}`); {said}",
            planted.home,
        );
    } else {
        assert!(
            said.contains(&planted.home),
            "{what}: the refusal names the entry's path, `{}`; {said}",
            planted.home,
        );
    }
    assert!(
        said.contains("a symbolic link"),
        "{what}: it says what stands there; {said}"
    );
    for retired in RETIRED {
        assert!(
            !said.contains(retired),
            "{what}: `{retired}` is another state's code — it answered this one until \
             2026-10-06 and may not again; {said}",
        );
    }
    assert!(
        fs::symlink_metadata(&planted.entry).is_ok_and(|meta| meta.file_type().is_symlink()),
        "{what}: the entry is still a link — nothing replaced it",
    );
    assert_eq!(
        fs::read_link(&planted.entry).ok().as_deref(),
        Some(planted.link.as_path()),
        "{what}: …pointing where it pointed",
    );
    assert!(
        !planted
            .entry
            .parent()
            .expect("a parent")
            .join("nowhere.md")
            .exists(),
        "{what}: nothing was written through it",
    );
    Answered(())
}

/// How a door presents the refusal.
#[derive(Clone, Copy)]
struct Shape {
    /// The door prints the finding itself, with its `at:` line.
    keyed: bool,
    /// The door is a triage report: the refusal is a blocked row, whatever the exit.
    at_exit_zero: bool,
}

const FINDING: Shape = Shape {
    keyed: true,
    at_exit_zero: false,
};
/// The finding inside the door's own refusal (`config.repoint-failed`) or its own report
/// (`migrate-corpus`'s `blocked` list), at a non-zero exit.
const CARRIED: Shape = Shape {
    keyed: false,
    at_exit_zero: false,
};
/// The finding as a blocked row of a triage report.
const ROW: Shape = Shape {
    keyed: false,
    at_exit_zero: true,
};

const ADR: &str = "adr:cache-strategy";
const ADR_HOME: &str = "docs/decisions/cache-strategy.md";
const MILESTONE: &str = "ship-it";
const RECORD_HOME: &str = "docs/milestone-records/ship-it.md";

/// Land an `adr` titled `title` through a real task, optionally superseding `supersedes`.
fn land_adr(corpus: &TrialCorpus, title: &str, supersedes: Option<&str>) -> String {
    let task = corpus.start_workflow("record-decision", &format!("land {title}"));
    let address = stage_adr(corpus, title, &task);
    if let Some(target) = supersedes {
        corpus.set_field(&format!("{address}#supersedes"), &task, target);
    }
    corpus.finalize(&task, "adr", &format!("land {title}"), false);
    address
}

/// Create and author an `adr` titled `title` in `task`, without finalizing.
fn stage_adr(corpus: &TrialCorpus, title: &str, task: &str) -> String {
    let address = corpus
        .jigc_ok(&["doc", "create", "adr", "--title", title, "--task", task])
        .trim()
        .to_owned();
    for slot in ["context", "decision", "consequences"] {
        corpus.set_slot(
            &format!("{address}#{slot}"),
            task,
            "The decision, recorded.",
        );
    }
    address
}

fn commit_all(corpus: &TrialCorpus, message: &str) {
    corpus.git(&["add", "-A"]);
    corpus.git(&["commit", "-q", "-m", message]);
}

/// A milestone with one sub-task, its record committed by the doors that made them.
fn milestone_with_a_sub_task(corpus: &TrialCorpus) {
    corpus.jigc_ok(&["milestone", "create", "ship it"]);
    corpus.jigc_ok(&["milestone", "add-task", MILESTONE, "first task"]);
}

// The cells. Each builds the smallest corpus in which its command meets a link at a managed
// home, runs the command, and ends in [`answers_the_one_code`].

fn task_finalize() -> Answered {
    let corpus = TrialCorpus::build(State::Fresh);
    let task = corpus.start_workflow("record-decision", "record the cache strategy");
    stage_adr(&corpus, "Cache Strategy", &task);
    corpus.set_field(&format!("commit:{task}#type"), &task, "docs");
    corpus.set_field(&format!("commit:{task}#scope"), &task, "adr");
    corpus.set_slot(&format!("commit:{task}#summary"), &task, "record it");
    corpus.set_slot(
        &format!("commit:{task}#body"),
        &task,
        "Driven by the suite.",
    );
    let planted = dangling(&corpus.repo(), ADR_HOME);
    let out = corpus.jigc(&["task", "finalize", &task]);
    answers_the_one_code("task finalize", &out, &planted, FINDING)
}

fn milestone_finalize_a_promotion() -> Answered {
    let corpus = TrialCorpus::build(State::Fresh);
    milestone_with_a_sub_task(&corpus);
    stage_adr(&corpus, "Cache Strategy", "first-task");
    let planted = dangling(&corpus.repo(), ADR_HOME);
    let out = corpus.jigc(&["milestone", "finalize", MILESTONE]);
    answers_the_one_code("milestone finalize · a promotion", &out, &planted, FINDING)
}

fn rename_own_home() -> Answered {
    let corpus = TrialCorpus::build(State::Fresh);
    land_adr(&corpus, "Cache Strategy", None);
    let planted = link_out(&corpus.repo(), ADR_HOME);
    commit_all(&corpus, "chore: the adr's home is a link");
    let out = corpus.jigc(&["rename", ADR, "--to", "Cache Plan"]);
    answers_the_one_code("rename · the doc's own home", &out, &planted, FINDING)
}

fn rename_referrer() -> Answered {
    let corpus = TrialCorpus::build(State::Fresh);
    land_adr(&corpus, "Cache Strategy", None);
    land_adr(&corpus, "Cache Strategy Revised", Some(ADR));
    let planted = link_out(&corpus.repo(), "docs/decisions/cache-strategy-revised.md");
    commit_all(&corpus, "chore: the referrer's home is a link");
    let out = corpus.jigc(&["rename", ADR, "--to", "Cache Plan"]);
    answers_the_one_code("rename · a referrer's home", &out, &planted, FINDING)
}

fn rename_destination() -> Answered {
    let corpus = TrialCorpus::build(State::Fresh);
    land_adr(&corpus, "Cache Strategy", None);
    let planted = dangling(&corpus.repo(), "docs/decisions/taken.md");
    let out = corpus.jigc(&["rename", ADR, "--to", "Taken"]);
    answers_the_one_code("rename · the destination", &out, &planted, FINDING)
}

fn config_set_docs_root() -> Answered {
    let corpus = TrialCorpus::build(State::Fresh);
    land_adr(&corpus, "Cache Strategy", None);
    let planted = link_out(&corpus.repo(), ADR_HOME);
    commit_all(&corpus, "chore: the adr's home is a link");
    let out = corpus.jigc(&["config", "set", "docs-root", "handbook/sub"]);
    answers_the_one_code("config set docs-root", &out, &planted, CARRIED)
}

fn config_set_placement_root() -> Answered {
    // The one shipped placement home with a leading directory, so the one this knob moves.
    let corpus = TrialCorpus::build(State::CommittedSingletons);
    let planted = link_out(&corpus.repo(), "docs/roadmap.md");
    commit_all(&corpus, "chore: the roadmap's home is a link");
    let out = corpus.jigc(&["config", "set", "placement-root", "handbook"]);
    answers_the_one_code("config set placement-root", &out, &planted, CARRIED)
}

/// `jigc relocate` moves a **freeze-exempt** doctype's stranded instances, and every
/// shipped doctype is manifest-governed — so the cell manufactures the smallest pack that
/// has one (the `relocate` suite's own `note`).
fn relocate() -> Answered {
    let root = support::trial_corpus::unique_root("home-shape-relocate");
    let (repo, home) = (root.join("repo"), root.join("home"));
    fs::create_dir_all(&repo).expect("mk the repo");
    fs::create_dir_all(&home).expect("mk the home");
    let git = |args: &[&str]| {
        let out = Command::new("git")
            .args(args)
            .current_dir(&repo)
            .env("HOME", &home)
            .output()
            .expect("spawn git");
        assert!(out.status.success(), "git {args:?}: {}", text(&out));
    };
    git(&["init", "-q"]);
    git(&["config", "user.email", "test@example.com"]);
    git(&["config", "user.name", "Test"]);
    let pack = repo.join(".jigc").join("note-pack");
    fs::create_dir_all(pack.join("schemas")).expect("mk the pack");
    fs::write(
        pack.join("schemas").join("note.yaml"),
        "type: note\nlocation: notes/\nid-from: title\nsections:\n  - id: body\n    slot: { hint: \"The note.\" }\n",
    )
    .expect("write the note schema");
    fs::create_dir_all(repo.join(".jigc").join("config")).expect("mk the project layer");
    fs::write(
        repo.join(".jigc").join("config").join("packs.yaml"),
        format!("packs:\n  - {}\n", pack.display()),
    )
    .expect("name the pack");
    // A committed link under the prior home, to a file that is no doc's home.
    fs::create_dir_all(repo.join("linked")).expect("mk the prior home");
    fs::create_dir_all(repo.join("elsewhere")).expect("mk the link's target dir");
    fs::write(
        repo.join("elsewhere/pointer.md"),
        "# Pointer\n\n## Body\n\nprose\n",
    )
    .expect("write the link's target");
    std::os::unix::fs::symlink("../elsewhere/pointer.md", repo.join("linked/pointer.md"))
        .expect("strand a link");
    git(&["add", "-A"]);
    git(&["commit", "-q", "-m", "strand a link"]);
    let planted = Planted {
        home: "linked/pointer.md".to_owned(),
        entry: repo.join("linked/pointer.md"),
        link: PathBuf::from("../elsewhere/pointer.md"),
    };
    let out = Command::new(env!("CARGO_BIN_EXE_jigc"))
        .args(["relocate", "note", "--from", "linked"])
        .current_dir(&repo)
        .env("HOME", &home)
        .env_remove("JIGC_PACK_DIR")
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()
        .expect("run jigc");
    let answered = answers_the_one_code("relocate", &out, &planted, ROW);
    let _ = fs::remove_dir_all(&root);
    answered
}

fn doc_rename() -> Answered {
    let corpus = TrialCorpus::build(State::Fresh);
    let planted = dangling(&corpus.repo(), "docs/decisions/taken-home.md");
    let task = corpus.start_workflow("single-task", "pick the broker");
    corpus.jigc_ok(&[
        "doc",
        "create",
        "adr",
        "--title",
        "Pick a broker",
        "--task",
        &task,
    ]);
    let out = corpus.jigc(&[
        "doc",
        "rename",
        "adr:pick-a-broker",
        "--to",
        "Taken Home",
        "--task",
        &task,
    ]);
    answers_the_one_code("doc rename", &out, &planted, FINDING)
}

/// The shipped create-only entry: `report-inconsistency` grants `inconsistency` with
/// `new: true`.
const CREATE_ONLY_HOME: &str = "docs/inconsistencies/taken-home.md";

fn doc_create() -> Answered {
    let corpus = TrialCorpus::build(State::Fresh);
    let planted = dangling(&corpus.repo(), CREATE_ONLY_HOME);
    let task = corpus.start_workflow("report-inconsistency", "report a disagreement");
    let out = corpus.jigc(&[
        "doc",
        "create",
        "inconsistency",
        "--title",
        "Taken Home",
        "--task",
        &task,
    ]);
    answers_the_one_code("doc create · a create-only entry", &out, &planted, FINDING)
}

fn doc_author() -> Answered {
    let corpus = TrialCorpus::build(State::Fresh);
    let planted = dangling(&corpus.repo(), CREATE_ONLY_HOME);
    let task = corpus.start_workflow("report-inconsistency", "report a disagreement");
    let out = corpus.jigc_stdin(
        &[
            "doc",
            "author",
            "inconsistency",
            "--from-file",
            "-",
            "--task",
            &task,
        ],
        "title: Taken Home\nsections:\n  - id: description\n    set:\n      description: |\n        <<Two docs disagree.>>\n",
    );
    answers_the_one_code("doc author · a create-only entry", &out, &planted, FINDING)
}

fn milestone_create() -> Answered {
    let corpus = TrialCorpus::build(State::Fresh);
    let planted = dangling(&corpus.repo(), RECORD_HOME);
    let out = corpus.jigc(&["milestone", "create", "ship it"]);
    answers_the_one_code("milestone create", &out, &planted, FINDING)
}

/// The five record doors share one preflight; each is driven over a record whose home a
/// third party turned into a link to a byte-identical copy — the cell that read as *in
/// sync* and was written through.
fn record_door(what: &str, state: State, prepare: fn(&TrialCorpus), argv: &[&str]) -> Answered {
    let corpus = TrialCorpus::build(state);
    prepare(&corpus);
    let planted = link_out(&corpus.repo(), RECORD_HOME);
    let out = corpus.jigc(argv);
    answers_the_one_code(what, &out, &planted, FINDING)
}

fn milestone_add_task() -> Answered {
    record_door(
        "milestone add-task",
        State::Fresh,
        |corpus| {
            corpus.jigc_ok(&["milestone", "create", "ship it"]);
        },
        &["milestone", "add-task", MILESTONE, "second task"],
    )
}

fn milestone_add_from_spec() -> Answered {
    // The one named state that carries a committed spec with a criterion to seed from.
    record_door(
        "milestone add-from-spec",
        State::Vendored,
        |corpus| {
            corpus.jigc_ok(&["milestone", "create", "ship it"]);
        },
        &["milestone", "add-from-spec", MILESTONE, "spec:padding"],
    )
}

fn milestone_discard() -> Answered {
    record_door(
        "milestone discard",
        State::Fresh,
        milestone_with_a_sub_task,
        &["milestone", "discard", MILESTONE],
    )
}

fn sub_task_discard() -> Answered {
    record_door(
        "task discard · a sub-task",
        State::Fresh,
        milestone_with_a_sub_task,
        &["task", "discard", "first-task", "--force"],
    )
}

fn milestone_finalize_the_record() -> Answered {
    record_door(
        "milestone finalize · the record",
        State::Fresh,
        |corpus| {
            milestone_with_a_sub_task(corpus);
            stage_adr(corpus, "Cache Strategy", "first-task");
        },
        &["milestone", "finalize", MILESTONE],
    )
}

/// `jigc migrate-corpus` over a doctype whose home moved at a version bump, with a link at
/// the home the stranded doc relocates to — the `migrate_corpus_home_pairs` fixture.
fn migrate_corpus_destination() -> Answered {
    use crate::migrate_corpus_home_pairs as fixture;
    let (pack, from_version) = fixture::bumped_pack(
        "one-code",
        "changelog",
        |shipped| shipped.to_string(),
        |shipped| {
            fixture::swap(
                shipped,
                fixture::CHANGELOG_PLACEMENT,
                "placement: { file: HISTORY.md }\n",
            )
        },
    );
    let home = fixture::TempDir::new("one-code-home");
    let repo = fixture::set_up_repo("one-code", home.path(), pack.path());
    fixture::commit_doc(
        repo.path(),
        "CHANGELOG.md",
        &fixture::changelog_body(from_version),
    );
    let planted = dangling(repo.path(), "HISTORY.md");
    let out = fixture::jigc(repo.path(), home.path(), pack.path(), &["migrate-corpus"]);
    answers_the_one_code(
        "migrate-corpus · a relocation's destination",
        &out,
        &planted,
        CARRIED,
    )
}

/// `jigc migrate-corpus` over a doc it would rewrite **in place** whose home is a link: the
/// v0 corpus state — a committed doc with its `schema-version:` stamp taken off — behind a
/// committed link.
fn migrate_corpus_in_place() -> Answered {
    let corpus = TrialCorpus::build(State::Fresh);
    land_adr(&corpus, "Cache Strategy", None);
    let home = corpus.repo().join(ADR_HOME);
    let unstamped: String = fs::read_to_string(&home)
        .expect("read the adr")
        .lines()
        .filter(|line| !line.starts_with("schema-version:"))
        .map(|line| format!("{line}\n"))
        .collect();
    fs::write(&home, unstamped).expect("strip the stamp");
    let planted = link_out(&corpus.repo(), ADR_HOME);
    commit_all(&corpus, "chore: an unstamped adr whose home is a link");
    let out = corpus.jigc(&["migrate-corpus"]);
    answers_the_one_code("migrate-corpus · a doc's own home", &out, &planted, CARRIED)
}

/// One cell: a corpus in which a command meets a link at a managed home, driven to its
/// refusal.
type Cell = fn() -> Answered;

/// **The commands, each with the cells that drive it.** A command may have more than one
/// cell where it meets the shape at more than one home.
const CELLS: &[(&str, Cell)] = &[
    ("task finalize", task_finalize),
    ("milestone finalize", milestone_finalize_a_promotion),
    ("milestone finalize", milestone_finalize_the_record),
    ("rename", rename_own_home),
    ("rename", rename_referrer),
    ("rename", rename_destination),
    ("relocate", relocate),
    ("config set docs-root", config_set_docs_root),
    ("config set placement-root", config_set_placement_root),
    ("doc rename", doc_rename),
    ("doc create", doc_create),
    ("doc author", doc_author),
    ("milestone add-task", milestone_add_task),
    ("milestone add-from-spec", milestone_add_from_spec),
    ("milestone discard", milestone_discard),
    ("task discard", sub_task_discard),
    ("milestone create", milestone_create),
    ("migrate-corpus", migrate_corpus_destination),
    ("migrate-corpus", migrate_corpus_in_place),
];

/// **Arm 2.** Every command a [`SITES`] row names is driven, and every one answers the one
/// code. The set iterated is the rows' own, so a command added to a row with no cell — or a
/// further caller of the constructor, which arm 1 forces into a row — reddens here.
#[test]
fn every_command_that_meets_the_shape_answers_the_one_code() {
    let named: BTreeSet<&str> = SITES
        .iter()
        .flat_map(|(_, _, commands)| commands.iter().copied())
        .collect();
    let mut cells: BTreeMap<&str, Vec<Cell>> = BTreeMap::new();
    for (command, cell) in CELLS {
        assert!(
            named.contains(command),
            "`{command}` has a cell and no row of `SITES` names it — a cell for a command \
             nothing says reaches the constructor proves nothing about the class",
        );
        cells.entry(command).or_default().push(*cell);
    }
    for command in &named {
        let driven = cells.get(command).unwrap_or_else(|| {
            panic!(
                "`jigc {command}` reaches `engine::store::home_shape_refusal` and no cell \
                 drives it: add one to `CELLS` — a corpus in which the command meets a link \
                 at a managed home, ending in `answers_the_one_code`",
            )
        });
        for cell in driven {
            let Answered(()) = cell();
        }
    }
}

// ── the cells that must not refuse ───────────────────────────────────────────────────

/// **An ordinary file, a doc legitimately absent and a placement home never meet this
/// code.** The doors above are driven again over the three states the guard must not
/// touch: each lands, and nothing it prints names the code.
#[test]
fn an_ordinary_file_an_absent_doc_and_a_placement_home_are_not_refused() {
    let clean = |what: &str, out: &Output| {
        let said = text(out);
        assert!(out.status.success(), "{what}: lands; {said}");
        assert!(
            !said.contains(CODE),
            "{what}: an ordinary home never meets `{CODE}`; {said}",
        );
    };

    // An absent doc: a free home takes a fresh mint, at the task door and at a rename.
    let corpus = TrialCorpus::build(State::Fresh);
    land_adr(&corpus, "Cache Strategy", None);
    clean(
        "a rename onto a free home",
        &corpus.jigc(&["rename", ADR, "--to", "Cache Plan"]),
    );
    // An ordinary file: the committed doc is updated in place by a task, and a referrer is
    // repointed by a rename.
    land_adr(&corpus, "Cache Plan Revised", Some("adr:cache-plan"));
    clean(
        "a rename that repoints an ordinary referrer",
        &corpus.jigc(&["rename", "adr:cache-plan", "--to", "Cache Design"]),
    );
    clean(
        "a root re-point over ordinary files",
        &corpus.jigc(&["config", "set", "docs-root", "handbook"]),
    );
    // The record doors over a record that is the regular file jigc wrote.
    clean(
        "milestone create at a free id",
        &corpus.jigc(&["milestone", "create", "ship it"]),
    );
    clean(
        "milestone add-task over the record as written",
        &corpus.jigc(&["milestone", "add-task", MILESTONE, "first task"]),
    );
    clean(
        "milestone discard over the record as written",
        &corpus.jigc(&["milestone", "discard", MILESTONE]),
    );

    // A placement home: the singletons at their literal files — one at the repository
    // root, one under a directory — are ordinary files, moved and updated as such.
    let singletons = TrialCorpus::build(State::CommittedSingletons);
    for home in ["VISION.md", "docs/roadmap.md"] {
        assert!(
            fs::symlink_metadata(singletons.repo().join(home)).is_ok_and(|meta| meta.is_file()),
            "the premise: `{home}` is a regular file at a placement home",
        );
    }
    clean(
        "a placement-root re-point over ordinary singletons",
        &singletons.jigc(&["config", "set", "placement-root", "handbook"]),
    );
    clean(
        "the store sweep afterwards",
        &singletons.jigc(&["validate"]),
    );
}
