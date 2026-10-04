//! The rc.24 fix pass, `(R6, D-7)` — **a promote lands a regular file at exactly its
//! canonical path, or the transaction refuses before anything is written**
//! (`design/finalize.md` → 4. Promote; `design/team-ready-state.md` → Shape is part of
//! membership: *jigc writes regular files and real directories and never a link*).
//!
//! The record's instance was one cell: a **dangling symlink** at a minted home passed the
//! create-gate, and `jigc task finalize` exited 0 saying `promoted … 1 file committed` over a
//! commit that held the *link* — the prose had been written **through** it into an untracked
//! file nobody named. The verification
//! (`completions/artifacts/M55/per-axis-review-rc24/tier1-verification/R6-D-7.md`) showed the
//! mechanism is not the create-only gate and not a doctype: every occupancy probe asked
//! `is_file()`, which follows links, and the promote's copy followed them too. So the class
//! is *any directory entry at a home that is not a regular file*, at *every door that
//! promotes*, and the axis below is the class and not the report:
//!
//! - **entry** — a dangling link (target inside the repository · outside it), a link to a
//!   device, a live link to a regular file, a directory;
//! - **tracked** — the link untracked, or committed at the home before the task existed;
//! - **door** — `jigc task finalize` (and its `--dry-run`), `jigc milestone finalize` under
//!   both commit models, and a migration's landing;
//! - **home** — a `location:` doctype (`adr`, `idea`) and a `placement:` singleton (`vision`);
//! - **provenance** — a doc the unit minted (`created`), and a committed doc the unit copied
//!   in through a live link and edited (`edited-from-base`, the live-link edit path).
//!
//! Every cell refuses with `finalize.promote-clobber` keyed at the destination, commits
//! nothing, writes nothing — at the home, through it, or outside the repository — leaves the
//! entry exactly as it stood, and keeps the unit's staged work. Then **the printed route is
//! driven as printed** and the unit lands a regular file at its home.
//!
//! **One shape is declared out of this suite, and it is not a gap in the guard.** A *special
//! file* at a home (a FIFO, a device node) is refused by the planner and by the sink like the
//! rest — the engine's `finalize` unit axis and `cli::task`'s sink test plant one — but no
//! committing door reaches its planner over a FIFO: the validate phase that precedes it reads
//! every file under a doctype's directory, and a read of a FIFO with no writer blocks. That is
//! a read seam with its own fix (driven while writing this suite: `jigc task finalize` parks
//! in `engine::target_surface::enumerate_target_surface`), so the cell would hang here rather
//! than assert anything about the promote.
//!
//! The create-only gate's half (`create.already-exists` over the same entries) is
//! `create_only_gate::an_entry_that_is_not_a_regular_file_is_an_occupied_home`; the in-task
//! re-slug's is `doc_rename_in_task::the_reslug_destination_guard_refuses_a_home_that_is_not_a_regular_file`.
//! The last test here is the one **other** door that mints a managed doc at its home without
//! promoting it — `jigc milestone create`, whose record write shared the mechanism.

#![cfg(unix)]

use crate::support;

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Output;

use support::trial_corpus::{State, TrialCorpus};

const CLOBBER: &str = "finalize.promote-clobber";
/// The prose only the unit's own doc carries — what a write-through would put somewhere.
const MARKER: &str = "PROMOTE-SHAPE-MARKER the decision this task recorded.";
const TITLE: &str = "Cache Strategy";
const ADR: &str = "adr:cache-strategy";
const ADR_HOME: &str = "docs/decisions/cache-strategy.md";

fn text(out: &Output) -> String {
    format!(
        "exit {:?}\n--- stdout ---\n{}\n--- stderr ---\n{}",
        out.status.code(),
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr),
    )
}

/// What a third party put at the doc's home.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Entry {
    /// A link whose target does not exist, beside the home.
    DanglingInRepo,
    /// A link whose target does not exist, in a directory outside the repository.
    DanglingOutside,
    /// A link to a device — the write-through that swallowed the prose outright.
    DeviceLink,
    /// A link to an existing regular file elsewhere in the repository.
    LiveLink,
    /// A directory wearing the doc's file name.
    Directory,
}

impl Entry {
    /// Every entry a fresh mint can face (a live link under a create-or-update entry is
    /// copied in instead — the live-link edit path, driven on its own below).
    const BLANK_MINT: [Entry; 4] = [
        Entry::DanglingInRepo,
        Entry::DanglingOutside,
        Entry::DeviceLink,
        Entry::Directory,
    ];

    /// How the refusal names it.
    fn noun(self) -> &'static str {
        match self {
            Entry::DanglingInRepo
            | Entry::DanglingOutside
            | Entry::DeviceLink
            | Entry::LiveLink => "a symbolic link",
            Entry::Directory => "a directory",
        }
    }
}

/// A planted entry, and everything a write-through could have touched.
struct Planted {
    /// The home, absolute.
    home: PathBuf,
    /// The path a write through the entry would create or change, when it has one.
    through: Option<PathBuf>,
    /// The bytes at `through` before the door ran (`None`: it did not exist).
    through_before: Option<Vec<u8>>,
    /// The entry's own shape and link text before the door ran.
    fingerprint: String,
}

/// The entry at `path` as `lstat` sees it, with the link's own text when it is one.
fn fingerprint(path: &Path) -> String {
    match fs::symlink_metadata(path) {
        Err(err) => format!("absent ({})", err.kind()),
        Ok(shape) if shape.file_type().is_symlink() => format!(
            "symlink -> {}",
            fs::read_link(path).expect("read the link").display()
        ),
        Ok(shape) if shape.is_dir() => "directory".to_owned(),
        Ok(shape) if shape.is_file() => "regular file".to_owned(),
        Ok(_) => "other".to_owned(),
    }
}

/// Put `entry` at `rel` in the corpus's repository.
fn plant(corpus: &TrialCorpus, entry: Entry, rel: &str) -> Planted {
    let home = corpus.repo().join(rel);
    let dir = home.parent().expect("a home directory").to_path_buf();
    fs::create_dir_all(&dir).expect("mk the home directory");
    let link = |target: &Path| {
        std::os::unix::fs::symlink(target, &home)
            .unwrap_or_else(|e| panic!("symlink {} -> {}: {e}", home.display(), target.display()));
    };
    let through = match entry {
        Entry::DanglingInRepo => {
            link(Path::new("nowhere.md"));
            Some(dir.join("nowhere.md"))
        }
        Entry::DanglingOutside => {
            // Beside the repository, never inside it: the corpus's own home directory.
            let outside = corpus.home().join("outside-the-repository");
            fs::create_dir_all(&outside).expect("mk the outside directory");
            let target = outside.join("landed-here.md");
            link(&target);
            Some(target)
        }
        Entry::DeviceLink => {
            link(Path::new("/dev/null"));
            None
        }
        Entry::LiveLink => {
            let target = corpus.repo().join("elsewhere/the-real-file.md");
            fs::create_dir_all(target.parent().expect("a parent")).expect("mk elsewhere/");
            fs::write(&target, "a file somebody else owns\n").expect("write the link's target");
            link(&target);
            Some(target)
        }
        Entry::Directory => {
            fs::create_dir(&home).expect("mk a directory at the home");
            None
        }
    };
    let through_before = through.as_ref().and_then(|path| fs::read(path).ok());
    Planted {
        fingerprint: fingerprint(&home),
        home,
        through,
        through_before,
    }
}

impl Planted {
    /// Assert the entry stands exactly as planted and nothing was written through it.
    fn assert_untouched(&self, what: &str) {
        assert_eq!(
            fingerprint(&self.home),
            self.fingerprint,
            "{what}: the entry at the home is exactly the entry that was there",
        );
        if let Some(through) = &self.through {
            assert_eq!(
                fs::read(through).ok(),
                self.through_before,
                "{what}: nothing was written through the entry — `{}` holds what it held \
                 before (absent stays absent)",
                through.display(),
            );
        }
    }

    /// Take the entry out of the doc's home — the route's *move it out* exit, as a person
    /// would do it. It lands beside the repository, so it is in no later commit's way.
    fn move_out(&self, corpus: &TrialCorpus) {
        let aside = corpus
            .home()
            .join(format!("moved-out-{}", engine::tempname::unique_nanos()));
        fs::rename(&self.home, &aside).expect("move the entry out of the doc's home");
    }
}

/// Author the three prose slots an `adr` needs, each carrying [`MARKER`].
fn fill_adr(corpus: &TrialCorpus, address: &str, task: &str) {
    for slot in ["context", "decision", "consequences"] {
        corpus.set_slot(&format!("{address}#{slot}"), task, MARKER);
    }
}

/// Author the task's `commit` doc, so the only thing left to decide is the finalize.
fn fill_commit(corpus: &TrialCorpus, task: &str) {
    corpus.set_field(&format!("commit:{task}#type"), task, "docs");
    corpus.set_field(&format!("commit:{task}#scope"), task, "shape");
    corpus.set_slot(&format!("commit:{task}#summary"), task, "record a decision");
    corpus.set_slot(
        &format!("commit:{task}#body"),
        task,
        "Driven by the shape axis.",
    );
}

/// Whether `task`'s working area still stages `address` carrying [`MARKER`].
fn staged_carries_marker(corpus: &TrialCorpus, task: &str, address: &str) -> bool {
    let path = corpus
        .repo()
        .join(".jigc/tasks")
        .join(task)
        .join("docs")
        .join(format!("{address}.md"));
    fs::read_to_string(path).is_ok_and(|body| body.contains(MARKER))
}

/// The blocking findings of a refused `--format json` finalize, asserted at **exit 3**.
fn blocked(out: &Output, what: &str) -> Vec<serde_json::Value> {
    assert_eq!(
        out.status.code(),
        Some(3),
        "{what}: the door is blocked at exit 3 — a promote lands a regular file at exactly \
         its home or refuses; {}",
        text(out),
    );
    let envelope: serde_json::Value = serde_json::from_slice(&out.stdout).unwrap_or_else(|e| {
        panic!(
            "{what}: the findings envelope on stdout ({e}); {}",
            text(out)
        )
    });
    envelope["findings"]
        .as_array()
        .unwrap_or_else(|| panic!("{what}: a `findings` array; {}", text(out)))
        .iter()
        .filter(|f| f["severity"] == "blocking")
        .cloned()
        .collect()
}

/// The one blocking finding, asserted to be the clobber refusal keyed at `destination` and
/// naming the entry's shape. Returns its route.
fn the_shape_refusal(
    findings: &[serde_json::Value],
    destination: &str,
    entry: Entry,
    what: &str,
) -> String {
    assert_eq!(
        findings.len(),
        1,
        "{what}: one blocking finding; {findings:#?}"
    );
    let finding = &findings[0];
    assert_eq!(
        (
            finding["key"]["code"].as_str(),
            finding["key"]["target"].as_str()
        ),
        (Some(CLOBBER), Some(destination)),
        "{what}: keyed {{{CLOBBER}, {destination}}} — the code and key of every other \
         occupant; {finding:#}",
    );
    let message = finding["message"].as_str().expect("a message");
    assert!(
        message.contains(entry.noun()) && message.contains(destination),
        "{what}: the refusal names what is at the home (`{}`) and the home; got: {message}",
        entry.noun(),
    );
    let route = finding["route"]
        .as_str()
        .unwrap_or_else(|| panic!("{what}: a blocking finding carries a route; {finding:#}"))
        .to_owned();
    for surface in [message, route.as_str()] {
        let lower = surface.to_lowercase();
        assert!(
            !lower.contains("remove") && !lower.contains("delete") && !surface.contains("git rm"),
            "{what}: the clobber refusal never teaches raw removal (RC-lacon A4/A7); got: \
             {surface}",
        );
        assert!(
            !surface.contains(&corpus_root_marker()),
            "{what}: repo-relative paths only; got: {surface}",
        );
    }
    route
}

/// A string no repo-relative surface carries: the temp root every corpus lives under.
fn corpus_root_marker() -> String {
    std::env::temp_dir().to_string_lossy().into_owned()
}

/// The backticked spans of `route` that open with `lead`, as emitted.
fn spans<'a>(route: &'a str, lead: &str) -> Vec<&'a str> {
    route
        .split('`')
        .skip(1)
        .step_by(2)
        .filter(|span| span.starts_with(lead))
        .collect()
}

/// Run an emitted command through a real `sh` split.
fn run_emitted(corpus: &TrialCorpus, command: &str) -> Output {
    let argv = support::shell_words(command, &corpus.repo(), &corpus.home());
    assert_eq!(argv.first().map(String::as_str), Some("jigc"), "{command}");
    let args: Vec<&str> = argv.iter().skip(1).map(String::as_str).collect();
    corpus.jigc(&args)
}

/// Run the route's one `lead` span **as printed** and assert it exits 0.
fn follow(corpus: &TrialCorpus, route: &str, lead: &str, what: &str) -> Output {
    let found = spans(route, lead);
    assert_eq!(
        found.len(),
        1,
        "{what}: the route names one `{lead} …` to run; got: {route}",
    );
    let out = run_emitted(corpus, found[0]);
    assert!(
        out.status.success(),
        "{what}: the emitted `{}` runs as printed; {}",
        found[0],
        text(&out),
    );
    out
}

/// The git mode and blob at `rel` in `HEAD` (`None` when the path is not in the commit).
fn head_entry(corpus: &TrialCorpus, rel: &str) -> Option<(String, String)> {
    let line = corpus.git(&["ls-tree", "HEAD", "--", rel]);
    let mut fields = line.split_whitespace();
    let mode = fields.next()?.to_owned();
    let _kind = fields.next()?;
    let blob = fields.next()?.to_owned();
    Some((mode, blob))
}

/// Assert `HEAD` holds a **regular file** at `rel` carrying [`MARKER`], and that the
/// worktree's entry there is a regular file too.
fn assert_landed_regular(corpus: &TrialCorpus, rel: &str, what: &str) {
    let (mode, blob) = head_entry(corpus, rel)
        .unwrap_or_else(|| panic!("{what}: `{rel}` is in the landed commit"));
    assert_eq!(
        mode, "100644",
        "{what}: the commit holds a regular file at the doc's home — never a `120000` link",
    );
    let body = corpus.git(&["cat-file", "-p", &blob]);
    assert!(
        body.contains(MARKER),
        "{what}: the committed doc carries the prose the unit authored; got:\n{body}",
    );
    assert_eq!(
        fingerprint(&corpus.repo().join(rel)),
        "regular file",
        "{what}: a regular file stands at the home",
    );
}

/// Start a `record-decision` task, mint [`ADR`] and author it and the commit doc.
fn decision_task(corpus: &TrialCorpus, intent: &str) -> String {
    let task = corpus.start_workflow("record-decision", intent);
    let address = corpus.jigc_ok(&["doc", "create", "adr", "--title", TITLE, "--task", &task]);
    assert_eq!(address.trim(), ADR, "the premise: a fresh mint of {ADR}");
    fill_adr(corpus, ADR, &task);
    fill_commit(corpus, &task);
    task
}

/// Everything that must still be true after a refused `task finalize`.
fn assert_nothing_happened(
    corpus: &TrialCorpus,
    planted: &Planted,
    task: &str,
    address: &str,
    head: &str,
    status: &str,
    what: &str,
) {
    assert_eq!(
        corpus.git(&["rev-parse", "HEAD"]),
        head,
        "{what}: nothing was committed"
    );
    planted.assert_untouched(what);
    assert!(
        staged_carries_marker(corpus, task, address),
        "{what}: the task's staged doc is intact, prose and all",
    );
    assert_eq!(
        corpus.git(&["status", "--porcelain"]),
        status,
        "{what}: the worktree and the index are as they were",
    );
}

/// **The record's cell and its class, at `jigc task finalize`.** A fresh `adr` whose home
/// is any entry that is not a regular file: the dry-run forecast and the committing run
/// both refuse, nothing is written anywhere, and each of the route's two exits — the
/// in-task rename, and moving the entry out — lands a regular file.
#[test]
fn a_fresh_doc_never_promotes_onto_an_entry_that_is_not_a_regular_file() {
    for entry in Entry::BLANK_MINT {
        for exit in ["rename", "move-out"] {
            let what = format!("task finalize · {entry:?} · exit by {exit}");
            let corpus = TrialCorpus::build(State::Fresh);
            let planted = plant(&corpus, entry, ADR_HOME);
            let task = decision_task(&corpus, "record the cache decision");
            let head = corpus.git(&["rev-parse", "HEAD"]);
            let status = corpus.git(&["status", "--porcelain"]);

            // The forecast refuses what the committing run will — never a manifest that
            // says `promoted` over a home it cannot land on.
            let forecast =
                corpus.jigc(&["task", "finalize", &task, "--dry-run", "--format", "json"]);
            let findings = blocked(&forecast, &format!("{what} (--dry-run)"));
            the_shape_refusal(&findings, ADR_HOME, entry, &format!("{what} (--dry-run)"));

            let out = corpus.jigc(&["task", "finalize", &task, "--format", "json"]);
            let findings = blocked(&out, &what);
            let route = the_shape_refusal(&findings, ADR_HOME, entry, &what);
            assert_nothing_happened(&corpus, &planted, &task, ADR, &head, &status, &what);

            match exit {
                "rename" => {
                    let rename = spans(&route, "jigc doc rename");
                    assert_eq!(rename.len(), 1, "{what}: one in-task rename; got: {route}");
                    assert!(
                        rename[0].contains(ADR) && rename[0].contains(&task),
                        "{what}: the rename is addressed at this task's doc; got: {}",
                        rename[0],
                    );
                    let command = rename[0].replace("\"<title>\"", "'Cache Strategy Revised'");
                    let renamed = run_emitted(&corpus, &command);
                    assert!(
                        renamed.status.success(),
                        "{what}: the emitted rename runs as printed (`{command}`); {}",
                        text(&renamed),
                    );
                    follow(&corpus, &route, "jigc task finalize", &what);
                    assert_landed_regular(
                        &corpus,
                        "docs/decisions/cache-strategy-revised.md",
                        &what,
                    );
                    planted.assert_untouched(&format!("{what}, after the landing"));
                    assert_eq!(
                        head_entry(&corpus, ADR_HOME),
                        None,
                        "{what}: the entry jigc refused is in no commit of jigc's",
                    );
                }
                _ => {
                    assert!(
                        route.contains("out of the doc's home"),
                        "{what}: the route names the other exit — the entry is the user's to \
                         move; got: {route}",
                    );
                    planted.move_out(&corpus);
                    follow(&corpus, &route, "jigc task finalize", &what);
                    assert_landed_regular(&corpus, ADR_HOME, &what);
                }
            }
        }
    }
}

/// **The write never leaves the repository.** The out-of-repository cell on its own, with
/// the claim spelled out: before the fix the door created a file in another directory at
/// exit 0, with nothing naming where the prose had gone.
#[test]
fn a_link_out_of_the_repository_writes_nothing_outside_it() {
    let corpus = TrialCorpus::build(State::Fresh);
    let planted = plant(&corpus, Entry::DanglingOutside, ADR_HOME);
    let outside = planted.through.clone().expect("the link's target");
    assert!(
        !outside.starts_with(corpus.repo()),
        "the premise: the target is outside the repository",
    );
    let task = decision_task(&corpus, "record the cache decision");
    let out = corpus.jigc(&["task", "finalize", &task, "--format", "json"]);
    blocked(&out, "a link out of the repository");
    assert!(
        fs::symlink_metadata(&outside).is_err(),
        "no file was created outside the repository at {}",
        outside.display(),
    );
    let siblings: Vec<_> = fs::read_dir(outside.parent().expect("a parent"))
        .expect("list the outside directory")
        .filter_map(Result::ok)
        .map(|e| e.file_name())
        .collect();
    assert!(
        siblings.is_empty(),
        "the outside directory is still empty; got {siblings:?}"
    );
}

/// **A link committed at the home before the task existed** (the verification's V3 and
/// V13). Tracked, the link made the stage step a no-op, so the door either blamed a hook
/// that does not exist or — with code staged — landed the code at exit 0 with the doc
/// silently absent. Both refuse now, with the staged code still staged.
#[test]
fn a_tracked_link_at_the_home_commits_nothing_with_or_without_staged_code() {
    for with_code in [false, true] {
        let what = format!("a tracked dangling link · staged code: {with_code}");
        let corpus = TrialCorpus::build(State::Fresh);
        let planted = plant(&corpus, Entry::DanglingInRepo, ADR_HOME);
        corpus.git(&["add", "--", ADR_HOME]);
        corpus.git(&["commit", "-q", "-m", "chore: a stale alias"]);
        assert_eq!(
            head_entry(&corpus, ADR_HOME)
                .map(|(mode, _)| mode)
                .as_deref(),
            Some("120000"),
            "{what}: the premise — the link is tracked",
        );

        let workflow = if with_code {
            "single-task"
        } else {
            "record-decision"
        };
        let task = corpus.start_workflow(workflow, "record the cache decision");
        corpus.jigc_ok(&["doc", "create", "adr", "--title", TITLE, "--task", &task]);
        fill_adr(&corpus, ADR, &task);
        fill_commit(&corpus, &task);
        if with_code {
            fs::create_dir_all(corpus.repo().join("src")).expect("mk src/");
            fs::write(corpus.repo().join("src/cache.rs"), "pub fn cache() {}\n").expect("code");
            corpus.git(&["add", "--", "src/cache.rs"]);
        }
        let head = corpus.git(&["rev-parse", "HEAD"]);
        let status = corpus.git(&["status", "--porcelain"]);

        let out = corpus.jigc(&["task", "finalize", &task, "--format", "json"]);
        let findings = blocked(&out, &what);
        let route = the_shape_refusal(&findings, ADR_HOME, Entry::DanglingInRepo, &what);
        assert_nothing_happened(&corpus, &planted, &task, ADR, &head, &status, &what);

        // The entry is the user's: taken out of the doc's home, the doc lands as a regular
        // file where the link was — and the staged code rides the same commit.
        planted.move_out(&corpus);
        follow(&corpus, &route, "jigc task finalize", &what);
        assert_landed_regular(&corpus, ADR_HOME, &what);
        if with_code {
            assert!(
                head_entry(&corpus, "src/cache.rs").is_some(),
                "{what}: the staged code landed with the doc",
            );
        }
    }
}

/// **The live-link edit path.** A committed doc whose home is a *live* link to a regular
/// file is read through the link at copy-in — reading is not the harm — and edited in the
/// task. Before the fix the promote wrote the edit **through** the link into the target,
/// staged the unchanged link, and the commit was rejected for having nothing in it: an edit
/// that could never land, reported as a hook's complaint. Now the door refuses by name, the
/// target is untouched, and the route's exit — the regular file itself at the home — lands
/// the edit.
#[test]
fn an_edit_of_a_doc_whose_home_is_a_live_link_is_refused_not_written_through() {
    let corpus = TrialCorpus::build(State::Fresh);
    // Land the doc, then turn its home into a tracked link to the file moved elsewhere.
    let first = corpus.start_workflow("record-decision", "land the decision");
    corpus.jigc_ok(&["doc", "create", "adr", "--title", TITLE, "--task", &first]);
    for slot in ["context", "decision", "consequences"] {
        corpus.set_slot(&format!("{ADR}#{slot}"), &first, "The first version.");
    }
    corpus.finalize(&first, "adr", "land the decision", false);
    let target_rel = "elsewhere/cache.md";
    fs::create_dir_all(corpus.repo().join("elsewhere")).expect("mk elsewhere/");
    corpus.git(&["mv", ADR_HOME, target_rel]);
    std::os::unix::fs::symlink("../../elsewhere/cache.md", corpus.repo().join(ADR_HOME))
        .expect("link the home at the moved file");
    corpus.git(&["add", "--", ADR_HOME]);
    corpus.git(&["commit", "-q", "-m", "chore: keep the decision elsewhere"]);
    let target = corpus.repo().join(target_rel);
    let target_before = fs::read(&target).expect("the link's target");
    let link_before = fingerprint(&corpus.repo().join(ADR_HOME));

    let task = corpus.start_workflow("record-decision", "revise the decision");
    corpus.set_slot(&format!("{ADR}#decision"), &task, MARKER);
    fill_commit(&corpus, &task);
    let head = corpus.git(&["rev-parse", "HEAD"]);

    let what = "an edit through a live link";
    let out = corpus.jigc(&["task", "finalize", &task, "--format", "json"]);
    let findings = blocked(&out, what);
    let route = the_shape_refusal(&findings, ADR_HOME, Entry::LiveLink, what);
    assert_eq!(
        corpus.git(&["rev-parse", "HEAD"]),
        head,
        "{what}: nothing was committed"
    );
    assert_eq!(
        fs::read(&target).expect("the target is still there"),
        target_before,
        "{what}: the link's target holds the bytes it held — the edit was not written through",
    );
    assert_eq!(
        fingerprint(&corpus.repo().join(ADR_HOME)),
        link_before,
        "{what}: the link stands",
    );
    assert!(
        staged_carries_marker(&corpus, &task, ADR),
        "{what}: the staged edit is intact",
    );
    assert!(
        spans(&route, "jigc doc rename").is_empty(),
        "{what}: a committed identity is not re-slugged in a task — the route must not hand \
         back a rename that refuses; got: {route}",
    );
    assert!(
        route.contains("regular file"),
        "{what}: the route names the exit — the regular file itself at the home; got: {route}",
    );

    // The exit, as a person takes it: the file the link points at, in the link's place.
    fs::remove_file(corpus.repo().join(ADR_HOME)).expect("take the link away");
    fs::copy(&target, corpus.repo().join(ADR_HOME)).expect("put the regular file at the home");
    follow(&corpus, &route, "jigc task finalize", what);
    assert_landed_regular(&corpus, ADR_HOME, what);
    assert_eq!(
        fs::read(&target).expect("the old target"),
        target_before,
        "{what}: the file the link pointed at is still what it was",
    );
}

/// **A placement singleton's home** has no slug to coincide and no other id to take: a
/// `vision` whose root `VISION.md` is a link refuses, the route offers no rename (the
/// in-task rename refuses a fixed identity), and the entry moved out lands the file.
#[test]
fn a_placement_home_that_is_a_link_is_refused() {
    let what = "a placement home that is a dangling link";
    let corpus = TrialCorpus::build(State::Fresh);
    fs::create_dir_all(corpus.repo().join("docs")).expect("mk docs/ — the link's target dir");
    let home = corpus.repo().join("VISION.md");
    std::os::unix::fs::symlink("docs/VISION-moved.md", &home).expect("link VISION.md");
    let planted = Planted {
        fingerprint: fingerprint(&home),
        home,
        through: Some(corpus.repo().join("docs/VISION-moved.md")),
        through_before: None,
    };

    let task = corpus.start_workflow("form-vision", "form the project vision");
    corpus.jigc_ok(&[
        "doc", "create", "vision", "--title", "Vision", "--task", &task,
    ]);
    for slot in ["thesis", "invariants", "open-questions"] {
        corpus.set_slot(&format!("vision:vision#{slot}"), &task, MARKER);
    }
    fill_commit(&corpus, &task);
    let head = corpus.git(&["rev-parse", "HEAD"]);
    let status = corpus.git(&["status", "--porcelain"]);

    let out = corpus.jigc(&["task", "finalize", &task, "--format", "json"]);
    let findings = blocked(&out, what);
    let route = the_shape_refusal(&findings, "VISION.md", Entry::DanglingInRepo, what);
    assert_nothing_happened(
        &corpus,
        &planted,
        &task,
        "vision:vision",
        &head,
        &status,
        what,
    );
    assert!(
        spans(&route, "jigc doc rename").is_empty(),
        "{what}: a singleton has one id — the route must not hand back a rename that \
         refuses; got: {route}",
    );

    planted.move_out(&corpus);
    follow(&corpus, &route, "jigc task finalize", what);
    assert_landed_regular(&corpus, "VISION.md", what);
}

/// The id a `jigc migrate` printed as `task minted: <id>`.
fn migrate_task(stdout: &str) -> String {
    stdout
        .lines()
        .find_map(|line| line.strip_prefix("task minted: "))
        .unwrap_or_else(|| panic!("`jigc migrate` prints `task minted: <id>`; got:\n{stdout}"))
        .trim()
        .to_string()
}

/// **A migration's landing** promotes through the same sink. A foreign note adopted as an
/// `idea` whose home is a dangling link refuses at the task's finalize — before the review
/// hold, so `--approve` is never offered a landing that cannot happen — the foreign source
/// is not retired, and with the entry moved out the route's own continuation lands it.
#[test]
fn a_migration_never_lands_through_a_link_at_its_home() {
    let what = "a migration landing onto a dangling link";
    let corpus = TrialCorpus::build(State::Fresh);
    let source = "notes/foreign.md";
    fs::create_dir_all(corpus.repo().join("notes")).expect("mk notes/");
    fs::write(corpus.repo().join(source), "# Some Foreign Note\n\nbody\n").expect("the note");
    corpus.git(&["add", "--", source]);
    corpus.git(&["commit", "-q", "-m", "chore: a foreign note"]);
    let home = "docs/ideas/some-foreign-note.md";
    let planted = plant(&corpus, Entry::DanglingInRepo, home);

    let task = migrate_task(&corpus.jigc_ok(&["migrate", source, "--as", "idea"]));
    let address = "idea:some-foreign-note";
    let payload = format!(
        "title: Some Foreign Note\nsections:\n  - id: description\n    set:\n      \
         description: |\n        <<{MARKER}>>\n"
    );
    corpus.jigc_stdin_ok(
        &["doc", "author", "idea", "--from-file", "-", "--task", &task],
        &payload,
    );
    corpus.set_field(&format!("{address}#trigger"), &task, "a report comes back");
    fill_commit(&corpus, &task);
    let head = corpus.git(&["rev-parse", "HEAD"]);
    let status = corpus.git(&["status", "--porcelain"]);

    for approve in [false, true] {
        let mut args = vec!["task", "finalize", task.as_str(), "--format", "json"];
        if approve {
            args.push("--approve");
        }
        let out = corpus.jigc(&args);
        let cell = format!("{what} · --approve: {approve}");
        let findings = blocked(&out, &cell);
        the_shape_refusal(&findings, home, Entry::DanglingInRepo, &cell);
        assert_nothing_happened(&corpus, &planted, &task, address, &head, &status, &cell);
        assert!(
            corpus.repo().join(source).is_file(),
            "{cell}: the foreign source is not retired by a landing that did not happen",
        );
    }

    let out = corpus.jigc(&["task", "finalize", &task, "--format", "json"]);
    let route = the_shape_refusal(&blocked(&out, what), home, Entry::DanglingInRepo, what);
    assert!(
        spans(&route, "jigc task finalize")
            .iter()
            .any(|span| span.ends_with("--approve")),
        "{what}: the route ends at the migration's own continuation; got: {route}",
    );
    planted.move_out(&corpus);
    let approve = spans(&route, "jigc task finalize")
        .into_iter()
        .find(|span| span.ends_with("--approve"))
        .expect("the --approve span")
        .to_owned();
    let landed = run_emitted(&corpus, &approve);
    assert!(
        landed.status.success(),
        "{what}: with the entry moved out, the emitted `{approve}` lands; {}",
        text(&landed),
    );
    assert_landed_regular(&corpus, home, what);
    assert!(
        !corpus.repo().join(source).exists(),
        "{what}: the landing retired the foreign source, as a migration's does",
    );
}

const MILESTONE_TITLE: &str = "record decisions";
const MILESTONE: &str = "record-decisions";

/// Mint the milestone and two default (`sub-task`) sub-tasks; returns their ids.
fn fan_out(corpus: &TrialCorpus) -> Vec<String> {
    corpus.jigc_ok(&["milestone", "create", MILESTONE_TITLE]);
    ["alpha", "bravo"]
        .iter()
        .map(|name| {
            let intent = format!("{name} records a decision");
            let ack = corpus.jigc_ok(&["milestone", "add-task", MILESTONE, &intent]);
            let (_, rest) = ack
                .split_once("added task:")
                .unwrap_or_else(|| panic!("`milestone add-task` names its task; got:\n{ack}"));
            rest.split_whitespace()
                .next()
                .expect("the sub-task id")
                .to_owned()
        })
        .collect()
}

/// **The milestone commit boundary shares the promote, so it shares the refusal** — under
/// both commit models, for a doc a sub-task minted (the in-task rename and the move-out
/// both land it) and for a committed doc a sub-task edited through a live link.
#[test]
fn the_milestone_boundary_refuses_an_entry_that_is_not_a_regular_file() {
    for squash in [true, false] {
        for exit in ["rename", "move-out"] {
            let what = format!("milestone finalize · squash={squash} · exit by {exit}");
            let corpus = TrialCorpus::build(State::Fresh);
            if !squash {
                corpus.jigc_ok(&["config", "set", "finalize.fan-out.squash", "false"]);
                if !corpus.git(&["status", "--porcelain"]).is_empty() {
                    corpus.git(&["add", "--", ".jigc/config"]);
                    corpus.git(&["commit", "-q", "-m", "chore: per-sub-task commits"]);
                }
            }
            let planted = plant(&corpus, Entry::DanglingInRepo, ADR_HOME);
            let subs = fan_out(&corpus);
            corpus.jigc_ok(&["doc", "create", "adr", "--title", TITLE, "--task", &subs[0]]);
            fill_adr(&corpus, ADR, &subs[0]);
            corpus.jigc_ok(&[
                "doc",
                "create",
                "adr",
                "--title",
                "Other Decision",
                "--task",
                &subs[1],
            ]);
            fill_adr(&corpus, "adr:other-decision", &subs[1]);
            let head = corpus.git(&["rev-parse", "HEAD"]);

            let out = corpus.jigc(&["milestone", "finalize", MILESTONE, "--format", "json"]);
            let findings = blocked(&out, &what);
            let route = the_shape_refusal(&findings, ADR_HOME, Entry::DanglingInRepo, &what);
            let message = findings[0]["message"].as_str().expect("a message");
            assert!(
                message.contains(subs[0].as_str()) && message.contains(ADR),
                "{what}: the refusal names the sub-task and the doc it staged; got: {message}",
            );
            assert_eq!(
                corpus.git(&["rev-parse", "HEAD"]),
                head,
                "{what}: nothing was committed"
            );
            planted.assert_untouched(&what);
            assert!(
                staged_carries_marker(&corpus, &subs[0], ADR),
                "{what}: the sub-task's staged doc is intact",
            );
            assert!(
                !route.contains("jigc task finalize") && spans(&route, "jigc migrate").is_empty(),
                "{what}: a sub-task has no boundary of its own, and nothing is adopted \
                 before this one; got: {route}",
            );

            let landed_at = match exit {
                "rename" => {
                    let rename = spans(&route, "jigc doc rename");
                    assert_eq!(rename.len(), 1, "{what}: one in-task rename; got: {route}");
                    let command = rename[0].replace("\"<title>\"", "'Cache Strategy Revised'");
                    let renamed = run_emitted(&corpus, &command);
                    assert!(
                        renamed.status.success(),
                        "{what}: the emitted rename runs as printed (`{command}`); {}",
                        text(&renamed),
                    );
                    "docs/decisions/cache-strategy-revised.md"
                }
                _ => {
                    planted.move_out(&corpus);
                    ADR_HOME
                }
            };
            follow(&corpus, &route, "jigc milestone finalize", &what);
            assert_landed_regular(&corpus, landed_at, &what);
            assert!(
                head_entry(&corpus, "docs/decisions/other-decision.md").is_some(),
                "{what}: the other sub-task's doc landed too — no sub-task's work is lost",
            );
            if exit == "rename" {
                planted.assert_untouched(&format!("{what}, after the landing"));
            }
        }
    }
}

/// **The one other door that mints a managed doc at its home: `jigc milestone create`.**
/// The `milestone-record` is not promoted — the door writes it directly and lands a
/// record-only commit — and its id-is-taken guard asked `exists()`, which follows links. So
/// a dangling link at the record's home read as a free id: the door exited 0 with a record
/// commit that held the **link**, and the record itself sat in an untracked file. The same
/// mechanism as the promote's, at a different sink, so the same contract: the home's entry
/// is read without following a link, an entry that is not a regular file refuses under the
/// door's existing `milestone.record-exists`, and the write creates the file exclusively.
/// Nothing is written or minted; with the entry moved out, the same command lands the record
/// as a regular file.
#[test]
fn the_milestone_record_is_never_created_through_a_link_at_its_home() {
    let home = "docs/milestone-records/record-decisions.md";
    for entry in [
        Entry::DanglingInRepo,
        Entry::DanglingOutside,
        Entry::LiveLink,
        Entry::Directory,
    ] {
        let what = format!("milestone create · {entry:?}");
        let corpus = TrialCorpus::build(State::Fresh);
        let planted = plant(&corpus, entry, home);
        let head = corpus.git(&["rev-parse", "HEAD"]);
        let status = corpus.git(&["status", "--porcelain"]);

        let out = corpus.jigc(&["milestone", "create", MILESTONE_TITLE]);
        let said = text(&out);
        assert!(
            !out.status.success() && said.contains("milestone.record-exists"),
            "{what}: the door refuses under its own occupied-id code; {said}",
        );
        assert!(
            said.contains(entry.noun()) && said.contains(home),
            "{what}: the refusal names the entry and the record's home; {said}",
        );
        assert!(
            !said.contains("add-task"),
            "{what}: there is no milestone to continue — the route must not offer one; {said}",
        );
        assert_eq!(
            corpus.git(&["rev-parse", "HEAD"]),
            head,
            "{what}: nothing was committed"
        );
        planted.assert_untouched(&what);
        assert_eq!(
            corpus.git(&["status", "--porcelain"]),
            status,
            "{what}: the worktree and the index are as they were",
        );
        assert!(
            !corpus
                .repo()
                .join(".jigc/milestones")
                .join(MILESTONE)
                .exists(),
            "{what}: no milestone area was minted by a create that refused",
        );

        planted.move_out(&corpus);
        let created = corpus.jigc(&["milestone", "create", MILESTONE_TITLE]);
        assert!(
            created.status.success(),
            "{what}: with the entry moved out, the same create lands; {}",
            text(&created),
        );
        assert_eq!(
            head_entry(&corpus, home).map(|(mode, _)| mode).as_deref(),
            Some("100644"),
            "{what}: the record commit holds a regular file at the record's home",
        );
        assert_eq!(
            fingerprint(&corpus.repo().join(home)),
            "regular file",
            "{what}"
        );
    }
}

/// **Controls — what the guard must not touch.** A free home lands; an update of a
/// committed regular file re-promotes over its own home. Both at exit 0, both a `100644`.
#[test]
fn a_free_home_and_an_update_of_a_regular_file_still_land() {
    let corpus = TrialCorpus::build(State::Fresh);
    let task = decision_task(&corpus, "record the cache decision");
    let out = corpus.jigc(&["task", "finalize", &task, "--format", "json"]);
    assert!(out.status.success(), "a free home lands; {}", text(&out));
    assert_landed_regular(&corpus, ADR_HOME, "a free home");

    let again = corpus.start_workflow("record-decision", "revise the decision");
    corpus.set_slot(&format!("{ADR}#consequences"), &again, "A second thought.");
    fill_commit(&corpus, &again);
    let out = corpus.jigc(&["task", "finalize", &again, "--format", "json"]);
    assert!(
        out.status.success(),
        "an update of a committed regular file re-promotes over its own home; {}",
        text(&out),
    );
    assert_landed_regular(&corpus, ADR_HOME, "an update of a regular file");
    let body = corpus.git(&["show", &format!("HEAD:{ADR_HOME}")]);
    assert!(
        body.contains("A second thought."),
        "the update is in the commit:\n{body}"
    );
}
