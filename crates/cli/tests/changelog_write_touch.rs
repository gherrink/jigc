//! M46 Increment 6, T1 — the changelog-gate advisory stops **counting items** and
//! becomes the **write-touch** its design already declares
//! ([validation.md](../../../design/validation.md) → The changelog-gate advisory:
//! *"It keys on the gate, never on the diff"*).
//!
//! The shipped predicate compared the staged changelog's repeatable-**item** count
//! against the committed one, so three ordinary authoring writes recorded an entry the
//! check refused to see: re-writing a committed category's `notes`, correcting a
//! release `date`, and retitling a release all leave the item count exactly where it
//! was. The trial's F-1 is that finding — `changelog-recording.gate-granted-unused`
//! fired on a task that *did* record an entry.
//!
//! The fix is a **write-touch**: the staged changelog is compared against its
//! **un-authored baseline** — the bytes the staging primitive materialized at first
//! touch (the committed doc copied in, or the pristine create skeleton when nothing is
//! committed). Any staged write that lands bytes suppresses the advisory; a create
//! that only materializes that baseline, and a write that is **refused**, do not.
//!
//! **This suite iterates the axis, not the repro.** The cells below are keyed on the
//! `["doc", …]` members of [`VERB_KINDS`](cli::cli::VERB_KINDS) that carry
//! [`VerbKind::Write`] — the code-side registry of jigc's write surface — and
//! [`every_doc_write_verb_has_a_cell`] fences the table against it, so a ninth `doc`
//! write verb cannot ship without an answer here. Every cell is driven through the
//! **real binary** against the embedded (shipped) packs.

use cli::cli::{VERB_KINDS, VerbKind};
use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

/// The advisory's finding code — the inventory row this suite drives.
const CODE: &str = "changelog-recording.gate-granted-unused";

/// A throwaway directory that removes itself on drop.
struct TempDir(PathBuf);

impl TempDir {
    fn new(tag: &str) -> Self {
        let mut path = std::env::temp_dir();
        path.push(format!(
            "jigc-changelog-write-touch-{tag}-{}-{:?}",
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

/// Run a `git` command in `repo`, asserting success.
fn git(repo: &Path, args: &[&str]) -> String {
    let out = Command::new("git")
        .args(args)
        .current_dir(repo)
        .output()
        .expect("run git");
    assert!(
        out.status.success(),
        "git {args:?} failed: {}",
        String::from_utf8_lossy(&out.stderr)
    );
    String::from_utf8(out.stdout)
        .expect("utf-8")
        .trim_end_matches('\n')
        .to_string()
}

/// Run `jigc <args>` against the **embedded** packs (the shipped bytes).
fn jigc(repo: &Path, home: &Path, args: &[&str], stdin: Option<&[u8]>) -> std::process::Output {
    let mut command = Command::new(env!("CARGO_BIN_EXE_jigc"));
    command
        .args(args)
        .current_dir(repo)
        .env("HOME", home)
        .env_remove("JIGC_PACK_DIR")
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());
    if stdin.is_some() {
        command.stdin(Stdio::piped());
    }
    let mut child = command.spawn().expect("spawn the jigc binary");
    if let Some(bytes) = stdin {
        crate::support::child_stdin::feed(&mut child, bytes);
    }
    child.wait_with_output().expect("wait for jigc")
}

/// Run `jigc <args>` (no stdin), asserting exit 0.
fn ok(repo: &Path, home: &Path, args: &[&str], what: &str) {
    let out = jigc(repo, home, args, None);
    assert!(
        out.status.success(),
        "`{what}` must exit 0; stdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr),
    );
}

/// Run `jigc <args>`, asserting it **refuses** (non-zero) and that the refusal says
/// `quoted` — the inapplicable / rejected cells state the refusal they rest on rather
/// than asserting a bare non-zero exit.
fn refuses(repo: &Path, home: &Path, args: &[&str], quoted: &str, what: &str) {
    let out = jigc(repo, home, args, None);
    let printed = format!(
        "{}{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr),
    );
    assert!(
        !out.status.success(),
        "`{what}` must refuse; it exited 0 with:\n{printed}",
    );
    assert!(
        printed.contains(quoted),
        "`{what}` must refuse with {quoted:?}; got:\n{printed}",
    );
}

/// Pipe `prose` into a `set-slot`, asserting success.
fn set_slot(repo: &Path, home: &Path, addr: &str, prose: &[u8]) {
    let out = jigc(
        repo,
        home,
        &["doc", "set-slot", addr, "--from-file", "-"],
        Some(prose),
    );
    assert!(
        out.status.success(),
        "`jigc doc set-slot {addr}` must exit 0; stderr:\n{}",
        String::from_utf8_lossy(&out.stderr),
    );
}

/// Fill every author-required field/slot of the provisioned commit doc.
fn fill_commit(repo: &Path, home: &Path, task: &str) {
    for (leaf, value) in [("type", "feat"), ("scope", "api")] {
        ok(
            repo,
            home,
            &[
                "doc",
                "set-field",
                &format!("commit:{task}#{leaf}"),
                "--value",
                value,
            ],
            &format!("set-field commit#{leaf}"),
        );
    }
    set_slot(
        repo,
        home,
        &format!("commit:{task}#summary"),
        b"add the rate limiter\n",
    );
    set_slot(
        repo,
        home,
        &format!("commit:{task}#body"),
        b"Bound per-client request volume.\n",
    );
}

/// Stage a real code change so the finalize has a diff to commit. The filename carries
/// the `tag` so a second finalize in the same fixture repo stages a fresh path.
fn stage_code(repo: &Path, tag: &str) {
    let name = format!("{tag}.txt");
    fs::write(repo.join(&name), "rate limiter\n").expect("write code change");
    git(repo, &["add", &name]);
}

/// Initialize a real git repo with one commit and run `jigc setup`.
fn setup(tag: &str) -> (TempDir, TempDir) {
    let repo = TempDir::new(tag);
    let home = TempDir::new(&format!("{tag}-home"));
    git(repo.path(), &["init", "-q"]);
    git(repo.path(), &["config", "user.email", "test@example.com"]);
    git(repo.path(), &["config", "user.name", "Test"]);
    fs::write(repo.path().join("README.md"), "hello\n").expect("write README");
    git(repo.path(), &["add", "README.md"]);
    git(repo.path(), &["commit", "-q", "-m", "initial"]);
    ok(repo.path(), home.path(), &["setup"], "jigc setup");
    (repo, home)
}

/// Start a `single-task` — the one shipped workflow granting `{type: changelog, as:
/// change}` — with `intent`, returning the minted task id (its frozen content slug).
fn start(repo: &Path, home: &Path, intent: &str, task: &str) -> String {
    ok(
        repo,
        home,
        &["start", "--workflow", "single-task", intent],
        "jigc start --workflow single-task",
    );
    assert!(
        repo.join(".jigc").join("tasks").join(task).is_dir(),
        "the minted task id must be `{task}`",
    );
    task.to_string()
}

/// Land a real **committed** `CHANGELOG.md`: one unreleased `Added` group carrying
/// prose, and one cut `1.0.0` release carrying a `date` and a nested `Fixed` group.
/// This is the corpus state the three regressed cells need — every one of them edits
/// content that is *already there*, leaving the item count untouched.
fn commit_a_changelog(repo: &Path, home: &Path) {
    let task = start(repo, home, "seed the changelog", "seed-the-changelog");
    ok(
        repo,
        home,
        &["doc", "create", "changelog", "--title", "Changelog"],
        "jigc doc create changelog",
    );
    ok(
        repo,
        home,
        &[
            "doc",
            "add-item",
            "changelog:changelog#unreleased-changes",
            "--title",
            "Added",
        ],
        "jigc doc add-item (unreleased)",
    );
    set_slot(
        repo,
        home,
        "changelog:changelog#unreleased-changes/added/notes",
        b"Per-client rate limiting at the gateway.\n",
    );
    ok(
        repo,
        home,
        &[
            "doc",
            "add-item",
            "changelog:changelog#releases",
            "--title",
            "1.0.0",
        ],
        "jigc doc add-item (releases)",
    );
    ok(
        repo,
        home,
        &[
            "doc",
            "add-item",
            "changelog:changelog#releases/1-0-0/changes",
            "--title",
            "Fixed",
        ],
        "jigc doc add-item (nested changes)",
    );
    set_slot(
        repo,
        home,
        "changelog:changelog#releases/1-0-0/changes/fixed/notes",
        b"A leak on the retry path.\n",
    );
    fill_commit(repo, home, &task);
    stage_code(repo, "seed");
    ok(
        repo,
        home,
        &["task", "finalize", &task],
        "jigc task finalize (seed the changelog)",
    );
    assert!(
        repo.join("CHANGELOG.md").is_file(),
        "the seeded changelog must be committed at its placement literal",
    );
}

/// Whether a **landed** finalize (`--format json`, exit 0) carried the advisory.
fn advisory_fired(out: &std::process::Output, what: &str) -> bool {
    assert!(
        out.status.success(),
        "`{what}` must LAND (exit 0 — the advisory never refuses); stdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr),
    );
    let stdout = String::from_utf8_lossy(&out.stdout);
    let value: serde_json::Value = serde_json::from_str(&stdout)
        .unwrap_or_else(|e| panic!("`{what}` emits the pinned envelope ({e}); got:\n{stdout}"));
    value["findings"]
        .as_array()
        .unwrap_or_else(|| panic!("the envelope carries a `findings` array; got:\n{stdout}"))
        .iter()
        .any(|f| f["code"] == CODE)
}

/// What a cell's write must do to the advisory.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Outcome {
    /// The write landed bytes over the un-authored baseline — the gate was used, so
    /// the advisory is suppressed.
    Suppressed,
    /// Nothing was authored (the baseline was only materialized, or the write was
    /// refused, or the verb is inapplicable to this doctype) — the advisory fires.
    StillFires,
}

/// One cell of the **write-verb axis**: a `["doc", …]` [`VerbKind::Write`] member,
/// driven against the managed changelog inside a gate-granting task, and what the
/// advisory must do at that task's finalize.
struct Cell {
    /// The [`VERB_KINDS`] path this cell drives — the axis coordinate.
    verb: &'static [&'static str],
    /// Temp-dir tag and failure label.
    tag: &'static str,
    /// Whether the fixture repo carries a **committed** `CHANGELOG.md` before the
    /// cell's task starts (the copy-in baseline arm vs. the create-skeleton arm).
    committed: bool,
    /// What the advisory must do.
    outcome: Outcome,
    /// Drives this cell's verb inside the started task.
    drive: fn(&Path, &Path),
    /// Why the cell reads the way it does — printed on failure, so a redness explains
    /// itself without a trip back to this table.
    why: &'static str,
}

/// The matrix. Every `["doc", …]` write verb appears at least once;
/// [`every_doc_write_verb_has_a_cell`] fences that against [`VERB_KINDS`].
const CELLS: &[Cell] = &[
    Cell {
        verb: &["doc", "create"],
        tag: "create-copy-in",
        committed: true,
        outcome: Outcome::StillFires,
        drive: drive_create,
        why: "a create over a committed changelog stages a byte-identical COPY of it — \
              every committed item rides along, and nothing was authored",
    },
    Cell {
        verb: &["doc", "create"],
        tag: "create-skeleton",
        committed: false,
        outcome: Outcome::StillFires,
        drive: drive_create,
        why: "a create with nothing committed stages the pristine schema skeleton — \
              create-and-abandon is exactly the silent skip this check exists to surface",
    },
    Cell {
        verb: &["doc", "add-item"],
        tag: "add-item",
        committed: true,
        outcome: Outcome::Suppressed,
        drive: drive_add_item,
        why: "cutting a new release is an authored entry — and it is the add-item \
              that reaches finalize alone, since a new change-group would leave its \
              required `notes` slot empty and block",
    },
    Cell {
        verb: &["doc", "remove-item"],
        tag: "remove-item",
        committed: true,
        outcome: Outcome::Suppressed,
        drive: drive_remove_item,
        why: "retracting a staged entry is a write to the changelog — it lands bytes, \
              and the item count moving DOWN must not read as `no entry recorded`",
    },
    Cell {
        verb: &["doc", "retitle-item"],
        tag: "retitle-item",
        committed: true,
        outcome: Outcome::Suppressed,
        drive: drive_retitle_item,
        why: "renaming a cut release is a changelog write; the item count is unchanged, \
              which is precisely what the counting predicate could not see",
    },
    Cell {
        verb: &["doc", "set-field"],
        tag: "set-field-date",
        committed: true,
        outcome: Outcome::Suppressed,
        drive: drive_set_field,
        why: "correcting a release date is a changelog write; the item count is unchanged",
    },
    Cell {
        verb: &["doc", "set-field"],
        tag: "set-field-refused",
        committed: true,
        outcome: Outcome::StillFires,
        drive: drive_set_field_refused,
        why: "a REFUSED write copies the committed doc in and then rejects, leaving a \
              staged copy byte-identical to its baseline — nothing was authored",
    },
    Cell {
        verb: &["doc", "set-slot"],
        tag: "set-slot",
        committed: true,
        outcome: Outcome::Suppressed,
        drive: drive_set_slot,
        why: "rewriting a committed category's notes is a changelog write; the item \
              count is unchanged",
    },
    Cell {
        verb: &["doc", "author"],
        tag: "author",
        committed: true,
        outcome: Outcome::Suppressed,
        drive: drive_author,
        why: "the batch verb lands its payload's leaves over the copied-in doc",
    },
    Cell {
        verb: &["doc", "rename"],
        tag: "rename-refused",
        committed: true,
        outcome: Outcome::StillFires,
        drive: drive_rename_refused,
        why: "INAPPLICABLE — `changelog` is a fixed-title singleton, so the in-task \
              title change refuses outright and stages nothing",
    },
];

fn drive_create(repo: &Path, home: &Path) {
    ok(
        repo,
        home,
        &["doc", "create", "changelog", "--title", "Changelog"],
        "jigc doc create changelog",
    );
}

fn drive_add_item(repo: &Path, home: &Path) {
    ok(
        repo,
        home,
        &[
            "doc",
            "add-item",
            "changelog:changelog#releases",
            "--title",
            "1.0.1",
        ],
        "jigc doc add-item",
    );
}

fn drive_remove_item(repo: &Path, home: &Path) {
    ok(
        repo,
        home,
        &[
            "doc",
            "remove-item",
            "changelog:changelog#unreleased-changes/added",
        ],
        "jigc doc remove-item",
    );
}

fn drive_retitle_item(repo: &Path, home: &Path) {
    ok(
        repo,
        home,
        &[
            "doc",
            "retitle-item",
            "changelog:changelog#releases/1-0-0",
            "--title",
            "1.0.1",
        ],
        "jigc doc retitle-item",
    );
}

fn drive_set_field(repo: &Path, home: &Path) {
    ok(
        repo,
        home,
        &[
            "doc",
            "set-field",
            "changelog:changelog#releases/1-0-0/date",
            "--value",
            "2026-01-31",
        ],
        "jigc doc set-field",
    );
}

fn drive_set_field_refused(repo: &Path, home: &Path) {
    refuses(
        repo,
        home,
        &[
            "doc",
            "set-field",
            "changelog:changelog#releases/9-9-9/date",
            "--value",
            "2026-01-31",
        ],
        r#"write rejected: item "9-9-9" in section "releases" not present"#,
        "jigc doc set-field (absent item)",
    );
}

fn drive_set_slot(repo: &Path, home: &Path) {
    set_slot(
        repo,
        home,
        "changelog:changelog#unreleased-changes/added/notes",
        b"Per-client rate limiting at the gateway, now with a burst allowance.\n",
    );
}

fn drive_author(repo: &Path, home: &Path) {
    let payload = repo.join("changelog-payload.yaml");
    fs::write(
        &payload,
        "sections:\n  - id: unreleased-changes\n    items:\n      - title: Changed\n        \
         set:\n          notes: \"<<The gateway now answers 429 with a Retry-After.>>\"\n",
    )
    .expect("write the author payload");
    ok(
        repo,
        home,
        &[
            "doc",
            "author",
            "changelog",
            "--from-file",
            payload.to_str().expect("utf-8 payload path"),
        ],
        "jigc doc author changelog",
    );
    fs::remove_file(&payload).expect("remove the author payload");
}

fn drive_rename_refused(repo: &Path, home: &Path) {
    refuses(
        repo,
        home,
        &["doc", "rename", "changelog", "--to", "Change Log"],
        "`changelog` is a singleton — its slug IS the type id and its `# H1` is supplied \
         by the schema",
        "jigc doc rename changelog",
    );
}

/// Drive one cell end to end and return whether the advisory fired at its finalize.
fn run_cell(cell: &Cell) -> bool {
    let (repo, home) = setup(cell.tag);
    if cell.committed {
        commit_a_changelog(repo.path(), home.path());
    }
    let task = start(
        repo.path(),
        home.path(),
        "add a rate limiter",
        "add-a-rate-limiter",
    );
    (cell.drive)(repo.path(), home.path());
    fill_commit(repo.path(), home.path(), &task);
    stage_code(repo.path(), cell.tag);
    let out = jigc(
        repo.path(),
        home.path(),
        &["task", "finalize", &task, "--format", "json"],
        None,
    );
    advisory_fired(&out, &format!("jigc task finalize ({})", cell.tag))
}

/// **The axis.** Every cell, driven through the real binary: a write that lands bytes
/// over the un-authored baseline suppresses the advisory; a create that only
/// materializes that baseline, a refused write, and an inapplicable verb do not.
#[test]
fn every_doc_write_verb_reads_as_a_write_touch() {
    // Every cell runs before the verdict: a per-iteration assert would report the
    // first wrong cell and hide the rest of the axis, which is the whole point here.
    let mut wrong = Vec::new();
    for cell in CELLS {
        let fired = run_cell(cell);
        let expected = cell.outcome == Outcome::StillFires;
        if fired != expected {
            wrong.push(format!(
                "  `jigc {}` / {}: the advisory must {} — {}",
                cell.verb.join(" "),
                cell.tag,
                if expected { "FIRE" } else { "be SUPPRESSED" },
                cell.why,
            ));
        }
    }
    assert!(
        wrong.is_empty(),
        "the `{CODE}` advisory misread {} of {} write-verb cells:\n{}",
        wrong.len(),
        CELLS.len(),
        wrong.join("\n"),
    );
}

/// The fence: the table above is keyed on the **code-side** write surface, so a ninth
/// `doc` write verb cannot ship without an answer here.
#[test]
fn every_doc_write_verb_has_a_cell() {
    let surface: Vec<&[&str]> = VERB_KINDS
        .iter()
        .filter(|(path, kind)| path.first() == Some(&"doc") && *kind == VerbKind::Write)
        .map(|(path, _)| *path)
        .collect();
    assert_eq!(
        surface.len(),
        8,
        "the `doc` write surface is the eight-verb axis this suite iterates; got: {surface:?}",
    );
    for verb in &surface {
        assert!(
            CELLS.iter().any(|cell| cell.verb == *verb),
            "`jigc {}` is a `doc` write verb with no cell — every write verb must state \
             whether it suppresses the `{CODE}` advisory, or why it cannot apply",
            verb.join(" "),
        );
    }
    for cell in CELLS {
        assert!(
            surface.contains(&cell.verb),
            "cell `{}` names `jigc {}`, which is not a `doc` write verb in VERB_KINDS",
            cell.tag,
            cell.verb.join(" "),
        );
    }
}
