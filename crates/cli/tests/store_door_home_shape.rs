//! The rc.24 fix pass — **the `(R6, D-7)` contract at the doors that write a committed doc
//! where it stands** (`design/finalize.md` → 4. Promote, Declared bounds;
//! `design/write-commands.md` → `jigc rename`; `design/team-ready-state.md` → The lifecycle).
//!
//! `(R6, D-7)` settled that *a managed doc lands as a regular file at its home, never
//! through a link*, and closed it at every door that **promotes**. It left the doors that
//! write a committed doc **in place** — no working area, no promote sink — declared and
//! undriven, and its fixer drove one cell of them: `jigc rename` of a doc whose home is a
//! live link exits 0, moves the link, writes the retitle *through* it and commits the link
//! move alone. Driven on that tree for this suite, the cell was one of a class:
//!
//! - **`jigc rename`, three homes.** The doc's **own** home (a re-slug wrote through the
//!   moved link; a retitle-only wrote through it and then acked `no-op … nothing committed`);
//!   the **destination** (a dangling link there reached git's `fatal: destination exists`, a
//!   directory a bare `Is a directory (os error 21)`, a live link the regular-file arm's
//!   *"a different doc already exists"*); and every **referrer** (the repoint was written
//!   through a linked referrer at exit 0 — `repointed 1 referrer(s)` — over a commit whose
//!   referrer still named the old id).
//! - **The relocation primitive** (`jigc config set docs-root`, `config set placement-root`,
//!   `jigc relocate`): `git mv` carried a relative link to a depth where it dangled, at exit
//!   0, after which `jigc doc list` could not enumerate the store.
//! - **The milestone record's later writers** (`milestone add-task`, `milestone discard`,
//!   `milestone finalize`, a sub-task's `task discard`): over a record replaced by a link,
//!   `add-task` exited 0 with a record commit that held the **link** and the record's new
//!   body in the link's untracked target; over a committed link every one of them wrote
//!   through, failed its own commit and reported *a hook's complaint*.
//!
//! One contract, three seams. Each door asks the home's own directory entry, without
//! following a link, **before** `git mv` and before any write; an entry that is not a
//! regular file refuses, nothing is written or moved, and the printed route — the regular
//! file put back at the home — lands the same command.
//!
//! **One code, at every seam** (the human's ruling of 2026-10-06 on the fix pass's items 7
//! and 8). As built, each door answered under the code it already had for *something is in
//! the way* — three codes in this suite alone: `finalize.promote-clobber` at the doc's own
//! home, a referrer's and the movers, `write.already-present` at a rename's destination,
//! `reconciliation.conflict-block` at the record doors. They all answer
//! `store.home-not-regular-file` now, keyed at the entry's path;
//! `home_shape_one_code.rs` iterates the commands. The corpus migration is the one in-place writer left as it was, and the last
//! test pins why: it writes a temp file and renames it over the home, which never writes
//! through whatever stood there.

#![cfg(unix)]

use crate::support;

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Output;

use support::trial_corpus::{State, TrialCorpus};

/// The code every door raises over a home that is not a regular file — the store's own,
/// through its one constructor (`design/command-output-contract.md` → the file row).
const SHAPE: &str = engine::store::HOME_NOT_REGULAR_FILE;
/// The codes this suite's doors answered that state with until 2026-10-06, each of which
/// has gone back to its own meaning: a rename's destination that holds *a doc*, and a
/// record that was *edited*. No cell here may answer either.
const RETIRED_HERE: [&str; 3] = [
    "finalize.promote-clobber",
    "write.already-present",
    "reconciliation.conflict-block",
];

const ADR: &str = "adr:cache-strategy";
const ADR_HOME: &str = "docs/decisions/cache-strategy.md";
/// Prose only the fixture's docs carry — what a write-through would move somewhere.
const MARKER: &str = "STORE-DOOR-SHAPE-MARKER the decision as it was recorded.";

fn text(out: &Output) -> String {
    format!(
        "exit {:?}\n--- stdout ---\n{}\n--- stderr ---\n{}",
        out.status.code(),
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr),
    )
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

/// The `route:` of the one rendered finding in `said`.
fn route_of(said: &str, what: &str) -> String {
    let routes: Vec<&str> = said
        .lines()
        .map(str::trim_start)
        .filter_map(|line| line.strip_prefix("route:"))
        .collect();
    assert_eq!(routes.len(), 1, "{what}: exactly one route; {said}");
    routes[0].trim().to_owned()
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

/// Run an emitted `jigc …` command through a real `sh` split.
fn run_emitted(corpus: &TrialCorpus, command: &str) -> Output {
    let argv = support::shell_words(command, &corpus.repo(), &corpus.home());
    assert_eq!(argv.first().map(String::as_str), Some("jigc"), "{command}");
    let args: Vec<&str> = argv.iter().skip(1).map(String::as_str).collect();
    corpus.jigc(&args)
}

/// Run an emitted `git …` command through a real `sh`, as printed.
fn run_emitted_git(corpus: &TrialCorpus, command: &str, what: &str) {
    let out = std::process::Command::new("sh")
        .arg("-c")
        .arg(command)
        .current_dir(corpus.home())
        .env("HOME", corpus.home())
        .output()
        .expect("spawn sh");
    assert!(
        out.status.success(),
        "{what}: the emitted `{command}` runs as printed, from any directory; {}",
        text(&out),
    );
}

/// The git mode at `rel` in `HEAD` (`None` when the path is not in the commit).
fn head_mode(corpus: &TrialCorpus, rel: &str) -> Option<String> {
    let line = corpus.git(&["ls-tree", "HEAD", "--", rel]);
    line.split_whitespace().next().map(str::to_owned)
}

/// The bytes `HEAD` holds at `rel`.
fn head_body(corpus: &TrialCorpus, rel: &str) -> String {
    corpus.git(&["show", &format!("HEAD:{rel}")])
}

/// Land an `adr` titled `title` through a real task, optionally superseding `supersedes`.
/// Returns its `<type>:<slug>` address.
fn land_adr(corpus: &TrialCorpus, title: &str, supersedes: Option<&str>) -> String {
    let slug = engine::slug::slugify(title);
    let task = corpus.start_workflow("record-decision", &format!("land {slug}"));
    let address = corpus
        .jigc_ok(&["doc", "create", "adr", "--title", title, "--task", &task])
        .trim()
        .to_owned();
    assert_eq!(address, format!("adr:{slug}"), "the premise: a fresh mint");
    for slot in ["context", "decision", "consequences"] {
        corpus.set_slot(&format!("{address}#{slot}"), &task, MARKER);
    }
    if let Some(target) = supersedes {
        corpus.set_field(&format!("{address}#supersedes"), &task, target);
    }
    corpus.finalize(&task, "adr", &format!("land {slug}"), false);
    address
}

/// A committed doc whose home a third party turned into a link.
struct Linked {
    /// The home, absolute.
    home: PathBuf,
    /// The file the link points at, absolute.
    target: PathBuf,
    /// The link's own entry before the door ran.
    fingerprint: String,
    /// The target's bytes before the door ran.
    target_before: Vec<u8>,
}

impl Linked {
    /// Assert the link stands as planted and nothing was written through it.
    fn assert_untouched(&self, what: &str) {
        assert_eq!(
            fingerprint(&self.home),
            self.fingerprint,
            "{what}: the entry at the home is exactly the entry that was there",
        );
        assert_eq!(
            fs::read(&self.target).expect("the link's target is still there"),
            self.target_before,
            "{what}: nothing was written through the link — its target holds what it held",
        );
    }

    /// **The route's exit, as a person takes it**: the regular file itself at the home — a
    /// copy of the file the link points at, in the link's place.
    fn regularize(&self) {
        fs::remove_file(&self.home).expect("take the link out of the doc's home");
        fs::write(&self.home, &self.target_before).expect("put the regular file in its place");
    }
}

/// Move the doc at `rel` to `elsewhere/` and leave a relative link to it at its home —
/// uncommitted. The link's target holds exactly the bytes the doc held.
fn link_out(corpus: &TrialCorpus, rel: &str) -> Linked {
    let repo = corpus.repo();
    let home = repo.join(rel);
    let name = Path::new(rel).file_name().expect("a file name");
    let target = repo.join("elsewhere").join(name);
    fs::create_dir_all(target.parent().expect("a parent")).expect("mk elsewhere/");
    fs::rename(&home, &target).expect("move the doc out from under its home");
    let up = "../".repeat(Path::new(rel).components().count() - 1);
    let link_text = format!("{up}elsewhere/{}", name.to_string_lossy());
    std::os::unix::fs::symlink(&link_text, &home).expect("link the home at the moved doc");
    Linked {
        fingerprint: fingerprint(&home),
        target_before: fs::read(&target).expect("read the link's target"),
        home,
        target,
    }
}

/// Commit whatever the fixture just changed.
fn commit_all(corpus: &TrialCorpus, message: &str) {
    corpus.git(&["add", "-A"]);
    corpus.git(&["commit", "-q", "-m", message]);
}

// ── `jigc rename`: the doc's own home ────────────────────────────────────────────────

/// **A rename never writes through, or moves, a link at the doc's own home** — the cell the
/// `(R6, D-7)` fixer drove and left, on both arms of the door.
///
/// The re-slug moved the link with `git mv` and wrote the new `# H1` through it: exit 0,
/// the link's target modified and uncommitted, the rename commit holding the link move
/// alone. The retitle-only arm wrote the `# H1` through the link, found nothing staged and
/// acked *"no-op … nothing renamed, nothing committed"* over a file it had just rewritten.
/// A **dangling** link at the home answered `store.not-found` — a doc that is not there,
/// under a route at `jigc describe` — where what stands in the way is an entry with a name.
///
/// Refused before `git mv` and before any write, under the code every committing door
/// raises for the same state of the same doc. The route's exit — the regular file put where
/// the link was, committed — then lands the command the refusal printed, as printed.
/// **The refusal reaches a `--format json` driver with its key** (the rc.24 fix pass, the
/// completion audit's CPL-7). `design/command-output-contract.md` lists the code under the
/// file-path target form — and until this cell the door answered
/// `{"error": "blocking · <code> — …"}`: the code inside a message, no `findings`, no
/// `key`. Re-run with `--format json`, the same refusal must be the reject arm every
/// committing door gives the state: stdout empty, exit 1, and on stderr the findings
/// envelope holding one blocking finding keyed `(store.home-not-regular-file, home)`, with
/// its route beside it and nothing written.
fn assert_keyed_on_the_wire(corpus: &TrialCorpus, argv: &[&str], home: &str, what: &str) {
    let mut json: Vec<&str> = argv.to_vec();
    json.extend(["--format", "json"]);
    let before = corpus.git(&["status", "--porcelain"]);
    let out = corpus.jigc(&json);
    assert_eq!(
        out.status.code(),
        Some(1),
        "{what}: the same refusal, the same exit; {}",
        text(&out),
    );
    assert!(
        out.stdout.is_empty(),
        "{what}: a reject leaves stdout empty; {}",
        text(&out),
    );
    let envelope: serde_json::Value = serde_json::from_slice(&out.stderr)
        .unwrap_or_else(|e| panic!("{what}: stderr is one JSON document ({e}); {}", text(&out)));
    assert!(
        envelope.get("error").is_none(),
        "{what}: not the flattened `{{\"error\": …}}` — a code inside a message is no key; \
         got: {envelope:#}",
    );
    // The pinned reject arm, read off the registry rather than restated: no key moves.
    let declared = cli::render::ENVELOPE_ARMS
        .iter()
        .find(|arm| arm.path.is_empty() && arm.arm == "Reject::Findings")
        .map(|arm| match arm.shape {
            cli::render::ArmShape::Object(keys) => {
                let mut keys: Vec<&str> = keys.to_vec();
                keys.sort_unstable();
                keys
            }
            _ => panic!("`Reject::Findings` declares an object key set"),
        })
        .expect("the cross-cutting `Reject::Findings` row is declared");
    let mut got: Vec<&str> = envelope
        .as_object()
        .unwrap_or_else(|| panic!("{what}: an object; got: {envelope:#}"))
        .keys()
        .map(String::as_str)
        .collect();
    got.sort_unstable();
    assert_eq!(
        got, declared,
        "{what}: exactly the top-level keys `ENVELOPE_ARMS` pins for the reject-with-findings \
         arm; got: {envelope:#}",
    );
    let findings = envelope["findings"]
        .as_array()
        .unwrap_or_else(|| panic!("{what}: the findings envelope; got: {envelope:#}"));
    assert_eq!(findings.len(), 1, "{what}: one refusal, one finding");
    let finding = &findings[0];
    assert_eq!(finding["severity"], "blocking", "{what}");
    assert_eq!(
        (
            finding["key"]["code"].as_str(),
            finding["key"]["target"].as_str()
        ),
        (Some(SHAPE), Some(home)),
        "{what}: keyed at the home, as the contract row says; got: {finding:#}",
    );
    assert!(
        finding["route"]
            .as_str()
            .is_some_and(|route| !spans(route, "jigc rename").is_empty()),
        "{what}: the route rides the finding; got: {finding:#}",
    );
    assert_eq!(
        corpus.git(&["status", "--porcelain"]),
        before,
        "{what}: the JSON run wrote nothing either",
    );
}

#[test]
fn a_rename_refuses_a_home_of_the_doc_that_is_not_a_regular_file() {
    for (arm, title, landed) in [
        (
            "re-slug",
            "Cache Strategy Two",
            "docs/decisions/cache-strategy-two.md",
        ),
        ("retitle-only", "Cache strategy!", ADR_HOME),
    ] {
        for dangling in [false, true] {
            let what = format!("jigc rename · {arm} · dangling={dangling}");
            let corpus = TrialCorpus::build(State::Fresh);
            land_adr(&corpus, "Cache Strategy", None);
            let linked = link_out(&corpus, ADR_HOME);
            if dangling {
                // The link stays, its target goes: nothing reachable through the home.
                fs::rename(&linked.target, corpus.home().join("taken-away.md"))
                    .expect("take the link's target away");
            }
            commit_all(&corpus, "chore: the adr's home is a link");
            let head = corpus.git(&["rev-parse", "HEAD"]);

            let out = corpus.jigc(&["rename", ADR, "--to", title]);
            let said = text(&out);
            assert_eq!(
                out.status.code(),
                Some(1),
                "{what}: the door refuses — it never moves a link or writes through one; {said}",
            );
            assert!(
                said.contains(&format!("· {SHAPE} — ")),
                "{what}: under the code a committing door raises for a home that is not a \
                 regular file; {said}",
            );
            assert!(
                said.contains("a symbolic link") && said.contains(ADR_HOME),
                "{what}: the refusal names the entry and the home; {said}",
            );
            assert!(
                !said.contains("no-op") && !said.contains("store.not-found"),
                "{what}: neither a no-op ack nor a doc that is not there; {said}",
            );
            assert_eq!(
                corpus.git(&["rev-parse", "HEAD"]),
                head,
                "{what}: nothing was committed",
            );
            assert_eq!(
                corpus.git(&["status", "--porcelain"]),
                "",
                "{what}: nothing was moved, staged or written",
            );
            assert_eq!(
                fingerprint(&linked.home),
                linked.fingerprint,
                "{what}: the link stands where it stood",
            );
            let route = route_of(&said, &what);
            let rerun = spans(&route, "jigc rename");
            assert_eq!(
                rerun.len(),
                1,
                "{what}: the route names the one command to re-run; got: {route}",
            );
            assert_keyed_on_the_wire(&corpus, &["rename", ADR, "--to", title], ADR_HOME, &what);
            if dangling {
                continue;
            }
            linked.assert_untouched(&what);

            // The exit, then the emitted command as printed.
            linked.regularize();
            commit_all(&corpus, "chore: the adr itself, where the link was");
            let renamed = run_emitted(&corpus, rerun[0]);
            assert!(
                renamed.status.success(),
                "{what}: with the regular file at the home, the emitted `{}` lands; {}",
                rerun[0],
                text(&renamed),
            );
            assert_eq!(
                head_mode(&corpus, landed).as_deref(),
                Some("100644"),
                "{what}: the rename commit holds a regular file at `{landed}`",
            );
            let body = head_body(&corpus, landed);
            assert!(
                body.contains(&format!("# {title}")) && body.contains(MARKER),
                "{what}: the committed doc carries the new title and its prose; got:\n{body}",
            );
            assert_eq!(
                corpus.git(&["status", "--porcelain"]),
                "",
                "{what}: the rename left nothing uncommitted behind",
            );
            assert_eq!(
                fs::read(&linked.target).expect("the old target"),
                linked.target_before,
                "{what}: the file the link pointed at is not the doc and was never rewritten",
            );
        }
    }
}

// ── `jigc rename`: the destination ───────────────────────────────────────────────────

/// What a third party put at the home the rename would land on.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Occupant {
    DanglingInRepo,
    DanglingOutside,
    LiveLink,
    Directory,
}

impl Occupant {
    const ALL: [Occupant; 4] = [
        Occupant::DanglingInRepo,
        Occupant::DanglingOutside,
        Occupant::LiveLink,
        Occupant::Directory,
    ];

    fn noun(self) -> &'static str {
        match self {
            Occupant::Directory => "a directory",
            _ => "a symbolic link",
        }
    }

    /// Put it at `rel`; returns the path a write through it would create or change.
    fn plant(self, corpus: &TrialCorpus, rel: &str) -> Option<PathBuf> {
        let home = corpus.repo().join(rel);
        let dir = home.parent().expect("a home directory");
        fs::create_dir_all(dir).expect("mk the home directory");
        let link = |target: &Path| {
            std::os::unix::fs::symlink(target, &home).expect("plant the link");
        };
        match self {
            Occupant::DanglingInRepo => {
                link(Path::new("nowhere.md"));
                Some(dir.join("nowhere.md"))
            }
            Occupant::DanglingOutside => {
                let outside = corpus.home().join("outside-the-repository");
                fs::create_dir_all(&outside).expect("mk the outside directory");
                let target = outside.join("landed-here.md");
                link(&target);
                Some(target)
            }
            Occupant::LiveLink => {
                let target = corpus.home().join("somebody-elses-notes.md");
                fs::write(&target, "a file somebody else owns\n").expect("write the target");
                link(&target);
                Some(target)
            }
            Occupant::Directory => {
                fs::create_dir(&home).expect("mk a directory at the home");
                None
            }
        }
    }
}

/// **A rename never lands on an entry that is not a regular file** — and says so itself.
///
/// The collision gate asked `is_file()`, which follows links, so a dangling link at the new
/// home read as free and the refusal was git's own `fatal: destination exists` — exit 1,
/// code-less and route-less, rolled back — where a regular-file occupant gets
/// `write.already-present`. A directory answered a bare `Is a directory (os error 21)`, and
/// a live link was called *"a different doc"*. All four now refuse at the gate, before the
/// transaction, naming what is there — under the store's code for the state, since
/// 2026-10-06, where they first rode the occupancy code; both exits the route names land.
#[test]
fn a_rename_refuses_a_destination_that_is_not_a_regular_file() {
    let taken = "docs/decisions/taken.md";
    for (nth, occupant) in Occupant::ALL.into_iter().enumerate() {
        let what = format!("jigc rename onto · {occupant:?}");
        let corpus = TrialCorpus::build(State::Fresh);
        land_adr(&corpus, "Cache Strategy", None);
        let through = occupant.plant(&corpus, taken);
        let through_before = through.as_ref().and_then(|path| fs::read(path).ok());
        let entry = fingerprint(&corpus.repo().join(taken));
        let head = corpus.git(&["rev-parse", "HEAD"]);
        let status = corpus.git(&["status", "--porcelain"]);

        let out = corpus.jigc(&["rename", ADR, "--to", "Taken"]);
        let said = text(&out);
        assert_eq!(out.status.code(), Some(1), "{what}: refused; {said}");
        assert!(
            said.contains(&format!("· {SHAPE} — ")) && said.contains(&format!("at: {taken}")),
            "{what}: under the store's code for an entry that is no file, keyed at the \
             entry — never git's `fatal`, never a bare OS error; {said}",
        );
        for retired in RETIRED_HERE {
            assert!(
                !said.contains(retired),
                "{what}: `{retired}` says a doc is there, and none is; {said}",
            );
        }
        assert!(
            said.contains(occupant.noun()) && said.contains(taken),
            "{what}: the refusal names the entry and the home it holds; {said}",
        );
        assert!(
            !said.contains("fatal:") && !said.contains("os error"),
            "{what}: the refusal is the door's, taken before the transaction; {said}",
        );
        assert_eq!(
            corpus.git(&["rev-parse", "HEAD"]),
            head,
            "{what}: nothing was committed",
        );
        assert_eq!(
            corpus.git(&["status", "--porcelain"]),
            status,
            "{what}: nothing was moved or staged",
        );
        assert_eq!(
            fingerprint(&corpus.repo().join(taken)),
            entry,
            "{what}: the entry stands as it stood",
        );
        if let Some(through) = &through {
            assert_eq!(
                fs::read(through).ok(),
                through_before,
                "{what}: nothing was written through the entry",
            );
        }

        let route = route_of(&said, &what);
        let renames = spans(&route, "jigc rename");
        let (other_slug, rerun): (Vec<&str>, Vec<&str>) = renames
            .iter()
            .partition(|span| span.contains("<other-slug>"));
        assert_eq!(
            (other_slug.len(), rerun.len()),
            (1, 1),
            "{what}: the route names both exits — another id, and the freed home; got: {route}",
        );
        let landed = if nth % 2 == 0 {
            // Exit 1: an id whose home is free. The entry is left exactly as it is.
            let command = other_slug[0].replace("<other-slug>", "taken-instead");
            let renamed = run_emitted(&corpus, &command);
            assert!(
                renamed.status.success(),
                "{what}: the emitted `{command}` lands; {}",
                text(&renamed),
            );
            assert_eq!(
                fingerprint(&corpus.repo().join(taken)),
                entry,
                "{what}: the entry was not touched by the rename that went around it",
            );
            "docs/decisions/taken-instead.md"
        } else {
            // Exit 2: the entry moved out of the doc's home (it is untracked here).
            fs::rename(
                corpus.repo().join(taken),
                corpus.home().join(format!("moved-out-{nth}")),
            )
            .expect("move the entry out of the doc's home");
            let renamed = run_emitted(&corpus, rerun[0]);
            assert!(
                renamed.status.success(),
                "{what}: with the home free, the emitted `{}` lands; {}",
                rerun[0],
                text(&renamed),
            );
            taken
        };
        assert_eq!(
            head_mode(&corpus, landed).as_deref(),
            Some("100644"),
            "{what}: the rename commit holds a regular file at `{landed}`",
        );
        assert!(
            head_body(&corpus, landed).contains("# Taken"),
            "{what}: the committed doc carries the new title",
        );
    }
}

// ── `jigc rename`: a referrer ────────────────────────────────────────────────────────

/// **A rename never repoints a referrer through a link** — the third home the transaction
/// writes, which no report named.
///
/// The repoint was `fs::write` at the referrer's home. With that home a live link, the
/// rename exited 0 saying `repointed 1 referrer(s)`: the new id went into the link's target,
/// uncommitted, and the rename commit moved the doc while the committed referrer still
/// named the id that no longer existed — the dangling cross-reference this verb exists to
/// make impossible, produced by the verb. Refused before anything moves; with the referrer
/// itself at its home, the emitted command lands the move and the repoint in one commit.
#[test]
fn a_rename_refuses_a_referrer_whose_home_is_a_link() {
    let what = "jigc rename · a linked referrer";
    let referrer_home = "docs/decisions/cache-strategy-revised.md";
    let corpus = TrialCorpus::build(State::Fresh);
    land_adr(&corpus, "Cache Strategy", None);
    land_adr(&corpus, "Cache Strategy Revised", Some(ADR));
    let linked = link_out(&corpus, referrer_home);
    commit_all(&corpus, "chore: the referrer's home is a link");
    let head = corpus.git(&["rev-parse", "HEAD"]);

    let out = corpus.jigc(&["rename", ADR, "--to", "Cache Plan"]);
    let said = text(&out);
    assert_eq!(out.status.code(), Some(1), "{what}: refused; {said}");
    assert!(
        said.contains(&format!("· {SHAPE} — "))
            && said.contains("a symbolic link")
            && said.contains(referrer_home),
        "{what}: the refusal names the referrer's home and what is there; {said}",
    );
    assert!(
        said.contains("adr:cache-strategy-revised"),
        "{what}: and the referrer it could not repoint; {said}",
    );
    assert_eq!(
        corpus.git(&["rev-parse", "HEAD"]),
        head,
        "{what}: nothing was committed — the doc did not move out from under its referrer",
    );
    assert_eq!(corpus.git(&["status", "--porcelain"]), "", "{what}");
    linked.assert_untouched(what);
    assert_eq!(
        fingerprint(&corpus.repo().join(ADR_HOME)),
        "regular file",
        "{what}: the doc is still at its home",
    );
    assert_keyed_on_the_wire(
        &corpus,
        &["rename", ADR, "--to", "Cache Plan"],
        referrer_home,
        what,
    );

    let route = route_of(&said, what);
    let rerun = spans(&route, "jigc rename");
    assert_eq!(
        rerun.len(),
        1,
        "{what}: one command to re-run; got: {route}"
    );
    linked.regularize();
    commit_all(&corpus, "chore: the referrer itself, where the link was");
    let renamed = run_emitted(&corpus, rerun[0]);
    assert!(
        renamed.status.success(),
        "{what}: the emitted `{}` lands; {}",
        rerun[0],
        text(&renamed),
    );
    assert_eq!(
        head_mode(&corpus, referrer_home).as_deref(),
        Some("100644"),
        "{what}: the referrer is a regular file in the rename commit",
    );
    assert!(
        head_body(&corpus, referrer_home).contains("supersedes: adr:cache-plan"),
        "{what}: and the commit holds the repoint — no committed reference dangles",
    );
    assert_eq!(
        head_mode(&corpus, "docs/decisions/cache-plan.md").as_deref(),
        Some("100644"),
        "{what}: beside the moved doc",
    );
    assert_eq!(corpus.git(&["status", "--porcelain"]), "", "{what}");
}

// ── the relocation primitive ─────────────────────────────────────────────────────────

/// **A relocation never carries a link to a new home.** `jigc config set docs-root` moves
/// every committed doc under the old root with `git mv`, which moves a link as the entry it
/// is — relative target and all. Driven: a re-point one directory deeper exited 0 saying
/// `relocating 2 committed doc(s)`, the link dangled at its new home, and `jigc doc list`
/// then answered a code-less exit 1 for the **whole** store while `jigc validate` routed at
/// dropping the doc.
///
/// The move primitive asks the source's entry before `git mv`, so the door refuses with
/// nothing moved and the knob unchanged; with the regular file at the home, the re-point
/// the refusal printed lands both docs.
#[test]
fn a_root_repoint_refuses_to_carry_a_link_to_the_new_home() {
    let what = "jigc config set docs-root · a linked doc";
    let corpus = TrialCorpus::build(State::Fresh);
    land_adr(&corpus, "Cache Strategy", None);
    land_adr(&corpus, "Keeper", None);
    let linked = link_out(&corpus, ADR_HOME);
    commit_all(&corpus, "chore: the adr's home is a link");
    let head = corpus.git(&["rev-parse", "HEAD"]);

    let out = corpus.jigc(&["config", "set", "docs-root", "handbook/sub"]);
    let said = text(&out);
    assert_eq!(out.status.code(), Some(1), "{what}: refused; {said}");
    assert!(
        said.contains("config.repoint-failed")
            && said.contains(SHAPE)
            && said.contains("a symbolic link")
            && said.contains(ADR_HOME),
        "{what}: the re-point's own refusal, naming the entry it would not carry; {said}",
    );
    assert_eq!(corpus.git(&["rev-parse", "HEAD"]), head, "{what}");
    assert_eq!(
        corpus.git(&["status", "--porcelain"]),
        "",
        "{what}: no doc moved and the knob was not written",
    );
    linked.assert_untouched(what);
    assert!(
        corpus.jigc(&["doc", "list"]).status.success(),
        "{what}: the store still enumerates",
    );

    let repoint = spans(&said, "jigc config set docs-root");
    assert!(
        !repoint.is_empty(),
        "{what}: the route names the re-point to re-run; {said}",
    );
    linked.regularize();
    commit_all(&corpus, "chore: the adr itself, where the link was");
    let landed = run_emitted(&corpus, repoint[0]);
    assert!(
        landed.status.success(),
        "{what}: with the regular file at the home, the emitted `{}` lands; {}",
        repoint[0],
        text(&landed),
    );
    for moved in ["cache-strategy", "keeper"] {
        assert_eq!(
            fingerprint(
                &corpus
                    .repo()
                    .join(format!("handbook/sub/decisions/{moved}.md"))
            ),
            "regular file",
            "{what}: `{moved}` is a regular file at its new home",
        );
    }
    let listed = corpus.jigc(&["doc", "list"]);
    assert!(
        listed.status.success()
            && String::from_utf8_lossy(&listed.stdout).contains("adr:cache-strategy"),
        "{what}: and the store enumerates it there; {}",
        text(&listed),
    );
}

// ── the milestone record's later writers ─────────────────────────────────────────────

const MILESTONE: &str = "ship-it";
const RECORD_HOME: &str = "docs/milestone-records/ship-it.md";

/// A door that rewrites a `milestone-record` that already exists.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum RecordDoor {
    AddTask,
    Discard,
    TaskDiscard,
    Finalize,
}

impl RecordDoor {
    const ALL: [RecordDoor; 4] = [
        RecordDoor::AddTask,
        RecordDoor::Discard,
        RecordDoor::TaskDiscard,
        RecordDoor::Finalize,
    ];

    fn argv(self) -> Vec<&'static str> {
        match self {
            RecordDoor::AddTask => vec!["milestone", "add-task", MILESTONE, "second task"],
            RecordDoor::Discard => vec!["milestone", "discard", MILESTONE],
            RecordDoor::TaskDiscard => vec!["task", "discard", "first-task", "--force"],
            RecordDoor::Finalize => vec!["milestone", "finalize", MILESTONE],
        }
    }

    /// What the landed record says once the door has run.
    fn landed(self) -> &'static str {
        match self {
            RecordDoor::AddTask => "{#second-task}",
            RecordDoor::Discard | RecordDoor::TaskDiscard => "status: discarded",
            RecordDoor::Finalize => "status: joined",
        }
    }
}

/// How the record's home came to hold a link.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum RecordLink {
    /// A live link to a byte-identical copy, in the worktree only: `HEAD` still holds the
    /// record as a file. The record's own drift check reads the same bytes through it.
    Uncommitted,
    /// The same link, committed.
    Committed,
    /// A link whose target does not exist, in the worktree only.
    Dangling,
}

/// **A milestone record is never rewritten through a link at its home.** `milestone create`
/// was closed by `(R6, D-7)`; the doors that write the record *afterwards* were reasoned
/// about and not driven. Driven: with the record replaced by a link to a byte-identical
/// copy, the drift check read the copy through the link and passed, and
/// `jigc milestone add-task` exited **0** — a record commit holding the **link**, the
/// record's new body in the link's untracked target, so every other clone was handed a
/// record that is not there. With the link committed, each door wrote through it, failed
/// its own commit for holding nothing and reported *a hook's complaint*.
///
/// The record's home is asked for its own entry, without following a link, at the one
/// preflight every one of those doors runs; an entry that is not a regular file refuses
/// there — the record is jigc's to write, as a regular file — under the store's code for
/// that state (the door's own conflict-block until 2026-10-06, which is an edit's code).
/// Nothing is written, minted or committed, and the route's restore lands the same command.
#[test]
fn a_milestone_record_is_never_rewritten_through_a_link_at_its_home() {
    for door in RecordDoor::ALL {
        for link in [
            RecordLink::Uncommitted,
            RecordLink::Committed,
            RecordLink::Dangling,
        ] {
            // The milestone boundary's base guard admits record-only commits and nothing
            // else, so a fixture commit that also carries the link's target would block it
            // for a reason this suite is not about; the dangling cell is driven once.
            if door == RecordDoor::Finalize && link == RecordLink::Committed
                || link == RecordLink::Dangling && door != RecordDoor::AddTask
            {
                continue;
            }
            let what = format!("{door:?} · {link:?}");
            let corpus = TrialCorpus::build(State::Fresh);
            corpus.jigc_ok(&["milestone", "create", "ship it"]);
            if door != RecordDoor::AddTask {
                corpus.jigc_ok(&["milestone", "add-task", MILESTONE, "first task"]);
            }
            if door == RecordDoor::Finalize {
                corpus.jigc_ok(&[
                    "doc",
                    "create",
                    "adr",
                    "--title",
                    "Cache Strategy",
                    "--task",
                    "first-task",
                ]);
                for slot in ["context", "decision", "consequences"] {
                    corpus.set_slot(&format!("{ADR}#{slot}"), "first-task", MARKER);
                }
            }
            let linked = link_out(&corpus, RECORD_HOME);
            match link {
                RecordLink::Uncommitted => {}
                RecordLink::Committed => commit_all(&corpus, "chore: the record's home is a link"),
                RecordLink::Dangling => {
                    fs::rename(&linked.target, corpus.home().join("taken-away.md"))
                        .expect("take the link's target away");
                }
            }
            let head = corpus.git(&["rev-parse", "HEAD"]);
            let status = corpus.git(&["status", "--porcelain"]);

            let out = corpus.jigc(&door.argv());
            let said = text(&out);
            assert!(
                !out.status.success(),
                "{what}: the door refuses — it never writes the record through a link; {said}",
            );
            assert!(
                said.contains(&format!("· {SHAPE} — "))
                    && said.contains("a symbolic link")
                    && said.contains(RECORD_HOME),
                "{what}: the store's refusal, naming the entry and the record's home; {said}",
            );
            for retired in RETIRED_HERE {
                assert!(
                    !said.contains(retired),
                    "{what}: `{retired}` is an edit's code, and nobody edited the record; \
                     {said}",
                );
            }
            assert!(
                !said.contains("hook"),
                "{what}: nobody's hook complained — the door refused before it wrote; {said}",
            );
            assert_eq!(
                corpus.git(&["rev-parse", "HEAD"]),
                head,
                "{what}: nothing was committed",
            );
            assert_eq!(
                corpus.git(&["status", "--porcelain"]),
                status,
                "{what}: the worktree and the index are as they were",
            );
            assert_eq!(
                fingerprint(&linked.home),
                linked.fingerprint,
                "{what}: the link stands where it stood",
            );
            if link != RecordLink::Dangling {
                linked.assert_untouched(&what);
            }
            if door == RecordDoor::AddTask {
                assert!(
                    !corpus.repo().join(".jigc/tasks/second-task").exists(),
                    "{what}: no sub-task was minted by an add that refused",
                );
            }

            // The route: the record back as the regular file `HEAD` holds, where it does —
            // the emitted git command, as printed — or the regular file itself, committed.
            let route = route_of(&said, &what);
            match link {
                RecordLink::Uncommitted | RecordLink::Dangling => {
                    let restore = spans(&route, "git ");
                    assert_eq!(
                        restore.len(),
                        1,
                        "{what}: the route names the one restore; got: {route}",
                    );
                    run_emitted_git(&corpus, restore[0], &what);
                }
                RecordLink::Committed => {
                    linked.regularize();
                    commit_all(&corpus, "chore: the record itself, where the link was");
                }
            }
            assert_eq!(
                fingerprint(&linked.home),
                "regular file",
                "{what}: the route's exit puts a regular file at the record's home",
            );
            let landed = corpus.jigc(&door.argv());
            assert!(
                landed.status.success(),
                "{what}: with the record a regular file again, the same command lands; {}",
                text(&landed),
            );
            assert_eq!(
                head_mode(&corpus, RECORD_HOME).as_deref(),
                Some("100644"),
                "{what}: the record commit holds a regular file at the record's home",
            );
            assert!(
                head_body(&corpus, RECORD_HOME).contains(door.landed()),
                "{what}: and the committed record carries what the door wrote (`{}`); got:\n{}",
                door.landed(),
                head_body(&corpus, RECORD_HOME),
            );
            if link != RecordLink::Dangling {
                assert_eq!(
                    fs::read(&linked.target).expect("the old target"),
                    linked.target_before,
                    "{what}: the file the link pointed at was never rewritten",
                );
            }
        }
    }
}

// ── the corpus migration: already safe, and pinned as such ───────────────────────────

/// **The corpus migration never writes through a link** — the one in-place writer of a
/// committed home this pass leaves as it was, driven rather than read.
///
/// `jigc migrate-corpus` rewrites a doc by writing a sibling temp file and renaming it over
/// the home. A rename replaces the directory entry it lands on; it does not open it. So over
/// a home that is a live link the migrated bytes land as a **regular file at exactly the
/// home**, the migration's own commit records that file, and the link's target keeps the
/// bytes it had. What it does not do is refuse: the link is replaced, not named — the
/// declared difference from the doors above (`design/finalize.md` → 4. Promote, Declared
/// bounds).
#[test]
fn the_corpus_migration_lands_a_regular_file_and_writes_through_nothing() {
    let what = "jigc migrate-corpus · a linked doc";
    let corpus = TrialCorpus::build(State::Fresh);
    land_adr(&corpus, "Cache Strategy", None);
    // The v0 corpus state: the doc as it was before the stamp existed.
    let home = corpus.repo().join(ADR_HOME);
    let stamped = fs::read_to_string(&home).expect("read the adr");
    let unstamped: String = stamped
        .lines()
        .filter(|line| !line.starts_with("schema-version:"))
        .map(|line| format!("{line}\n"))
        .collect();
    assert_ne!(stamped, unstamped, "the premise: the adr carried a stamp");
    fs::write(&home, &unstamped).expect("strip the stamp");
    let linked = link_out(&corpus, ADR_HOME);
    commit_all(&corpus, "chore: an unstamped adr whose home is a link");

    let out = corpus.jigc(&["migrate-corpus"]);
    assert!(
        out.status.success(),
        "{what}: the migration lands; {}",
        text(&out)
    );
    assert_eq!(
        fs::read(&linked.target).expect("the link's target"),
        linked.target_before,
        "{what}: nothing was written through the link — its target is still unstamped",
    );
    assert_eq!(
        fingerprint(&home),
        "regular file",
        "{what}: the migrated doc is a regular file at exactly its home",
    );
    assert_eq!(
        head_mode(&corpus, ADR_HOME).as_deref(),
        Some("100644"),
        "{what}: and the migration's commit holds that file, not the link",
    );
    assert!(
        head_body(&corpus, ADR_HOME).contains("schema-version:"),
        "{what}: carrying the stamp the migration added",
    );
    assert_eq!(corpus.git(&["status", "--porcelain"]), "", "{what}");
}
