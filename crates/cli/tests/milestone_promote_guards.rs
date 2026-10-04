//! The rc.24 fix pass, `(R6, D-1)` — **the milestone commit boundary never promotes a
//! `created` doc over an entry already at its destination** (`design/finalize.md` → 4.
//! Promote; `design/storage.md` → The by-task-id join, rule 4).
//!
//! `jigc task finalize` has refused that state by name since M25
//! (`finalize.promote-clobber`). `jigc milestone finalize` ran the same promote sweep and
//! never the guard after it, so a sub-task's fresh doc landed on whatever sat at its home at
//! exit 0 with `findings` empty: an untracked file there was overwritten and is in no git
//! object; a committed one was replaced under its own id
//! (`completions/artifacts/M55/per-axis-review-rc24/tier1-verification/R6-D-1.md`).
//!
//! **The axis is the class, not the report.** The record reached the defect through the
//! join's collision suffix landing on an occupied `<slug>-2`; the verification showed the
//! suffix is one way in and not the mechanism — an occupant at an *unsuffixed* home is
//! overwritten the same way. So the matrix below is
//!
//! - **landing** — the join suffixed the doc onto the occupied id · the doc kept its own id;
//! - **occupant** — a committed managed doc · an untracked conformant doc · an untracked
//!   foreign file · a file staged and never committed;
//! - **`finalize.fan-out.squash`** — both commit models, since the planner is shared;
//! - **doctype** — a methodology create-or-update doctype (`idea`), a dev-pack one (`adr`),
//!   and a create-only one (`inconsistency`, `new: true`);
//!
//! and every refused cell is then **driven out through the route it printed**, as emitted:
//! the milestone lands, every sub-task's doc is in the commit, and the occupant's bytes are
//! still the bytes that were there. One cell of the cross product does not exist and is named
//! where it is skipped: a *committed* occupant at an *unsuffixed* home (a create over a
//! committed doc copies it in, so the doc is not `created`).
//!
//! The controls are the two states the guard must not touch: a suffix landing on a free id,
//! and a sub-task's update of a committed doc (`edited-from-base`, which re-promotes over
//! its own home by design).

use crate::support;

use std::fs;
use std::path::PathBuf;
use std::process::Output;

use support::trial_corpus::{State, TrialCorpus};

const CLOBBER: &str = "finalize.promote-clobber";
/// The milestone every cell mints, and the id the binary slugs it to.
const MILESTONE_TITLE: &str = "file findings";
const MILESTONE: &str = "file-findings";
/// The title two sub-tasks share, and the slug it mints.
const SHARED_TITLE: &str = "Shared Finding";
const SHARED_SLUG: &str = "shared-finding";
/// The bytes only the occupant carries — what a clobber would erase.
const OCCUPANT_MARKER: &str = "OCCUPANT-MARKER the earlier finding nobody addressed.";

/// The doctype axis: which pack, which create grant, which home.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Kind {
    /// Methodology pack, create-or-update (`park-idea`).
    Idea,
    /// Dev pack, create-or-update — the default fan-out target, `sub-task`.
    Adr,
    /// Methodology pack, **create-only** (`report-inconsistency`, `new: true`).
    Inconsistency,
}

impl Kind {
    const ALL: [Kind; 3] = [Kind::Idea, Kind::Adr, Kind::Inconsistency];

    fn doctype(self) -> &'static str {
        match self {
            Kind::Idea => "idea",
            Kind::Adr => "adr",
            Kind::Inconsistency => "inconsistency",
        }
    }

    /// The doctype's home directory under the corpus's docs-root.
    fn home(self) -> &'static str {
        match self {
            Kind::Idea => "docs/ideas",
            Kind::Adr => "docs/decisions",
            Kind::Inconsistency => "docs/inconsistencies",
        }
    }

    /// The workflow a sub-task is added under — `None` is the default fan-out target.
    fn sub_task_workflow(self) -> Option<&'static str> {
        match self {
            Kind::Idea => Some("park-idea"),
            Kind::Adr => None,
            Kind::Inconsistency => Some("report-inconsistency"),
        }
    }

    /// The workflow that lands one doc of this kind through the single-task door.
    fn standalone_workflow(self) -> &'static str {
        match self {
            Kind::Idea => "park-idea",
            Kind::Adr => "record-decision",
            Kind::Inconsistency => "report-inconsistency",
        }
    }

    /// Author every leaf the boundary's conformance gate requires, carrying `prose`.
    fn fill(self, corpus: &TrialCorpus, address: &str, task: &str, prose: &str) {
        match self {
            Kind::Idea => {
                corpus.set_field(&format!("{address}#trigger"), task, "a report comes back");
                corpus.set_slot(&format!("{address}#description"), task, prose);
            }
            Kind::Adr => {
                for slot in ["context", "decision", "consequences"] {
                    corpus.set_slot(&format!("{address}#{slot}"), task, prose);
                }
            }
            Kind::Inconsistency => {
                corpus.set_field(&format!("{address}#meta/kind"), task, "doc-doc");
                corpus.set_slot(&format!("{address}#description"), task, prose);
            }
        }
    }
}

/// What already sits at the promote destination.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Occupant {
    /// A managed doc landed through its own task before the milestone pinned its base.
    Committed,
    /// A conformant doc written by hand at the home and never `git add`ed.
    UntrackedConformant,
    /// A file with no front-matter and no schema — nothing jigc ever saw.
    UntrackedForeign,
    /// A conformant doc `git add`ed and never committed.
    Staged,
}

impl Occupant {
    const ALL: [Occupant; 4] = [
        Occupant::Committed,
        Occupant::UntrackedConformant,
        Occupant::UntrackedForeign,
        Occupant::Staged,
    ];
}

/// How the sub-task's doc arrives at the occupied home.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Landing {
    /// Two sub-tasks mint one slug; the join suffixes the second onto the occupied `-2`.
    Suffixed,
    /// The doc keeps its own id, and the occupant appears at that id after the create.
    Bare,
}

fn text(out: &Output) -> String {
    format!(
        "exit {:?}\n--- stdout ---\n{}\n--- stderr ---\n{}",
        out.status.code(),
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr),
    )
}

/// Take the `squash: false` commit model, committed, before any milestone pins a base.
fn squash_false(corpus: &TrialCorpus) {
    corpus.jigc_ok(&["config", "set", "finalize.fan-out.squash", "false"]);
    if !corpus.git(&["status", "--porcelain"]).is_empty() {
        corpus.git(&["add", "--", ".jigc/config"]);
        corpus.git(&["commit", "-q", "-m", "chore: per-sub-task commits"]);
    }
}

/// Land one doc of `kind` titled `title` through the single-task door, carrying `prose`.
fn land_standalone(corpus: &TrialCorpus, kind: Kind, title: &str, prose: &str) -> String {
    let task = corpus.start_workflow(kind.standalone_workflow(), &format!("land {title}"));
    let address = create(corpus, kind, title, &task);
    kind.fill(corpus, &address, &task, prose);
    corpus.finalize(&task, kind.doctype(), &format!("land {title}"), false);
    address
}

/// `jigc doc create <doctype> --title <title> --task <task>`, returning the emitted address.
fn create(corpus: &TrialCorpus, kind: Kind, title: &str, task: &str) -> String {
    corpus
        .jigc_ok(&[
            "doc",
            "create",
            kind.doctype(),
            "--title",
            title,
            "--task",
            task,
        ])
        .trim()
        .to_string()
}

/// Mint the milestone and `count` sub-tasks of `kind`, returning the sub-task ids the binary
/// printed, in the order they were added (which is their task-id order: `one` < `three` < `two`
/// would not be, so the intents are chosen to sort as they are added).
fn fan_out(corpus: &TrialCorpus, kind: Kind, count: usize) -> Vec<String> {
    corpus.jigc_ok(&["milestone", "create", MILESTONE_TITLE]);
    let names = ["alpha", "bravo", "charlie"];
    let mut subs = Vec::new();
    for name in &names[..count] {
        let intent = format!("{name} files a finding");
        let mut args = vec!["milestone", "add-task", MILESTONE, intent.as_str()];
        if let Some(workflow) = kind.sub_task_workflow() {
            args.extend(["--workflow", workflow]);
        }
        let ack = corpus.jigc_ok(&args);
        let (_, rest) = ack
            .split_once("added task:")
            .unwrap_or_else(|| panic!("`milestone add-task` names its task; got:\n{ack}"));
        subs.push(
            rest.split_whitespace()
                .next()
                .expect("the sub-task id")
                .to_owned(),
        );
    }
    let mut sorted = subs.clone();
    sorted.sort();
    assert_eq!(subs, sorted, "the premise: add order is task-id order");
    subs
}

/// The prose only `sub`'s doc carries — how a landed doc is attributed to its sub-task.
fn body_of(sub: &str) -> String {
    format!("From {sub}.")
}

/// Have `sub` create and author one doc of `kind` titled `title`; returns its address.
fn author(corpus: &TrialCorpus, kind: Kind, title: &str, sub: &str) -> String {
    let address = create(corpus, kind, title, sub);
    kind.fill(corpus, &address, sub, &body_of(sub));
    address
}

/// The bytes `sub` staged for `address` — a conformant body to plant an occupant from.
fn staged_bytes(corpus: &TrialCorpus, sub: &str, address: &str) -> String {
    let path = corpus
        .repo()
        .join(".jigc/tasks")
        .join(sub)
        .join("docs")
        .join(format!("{address}.md"));
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read {}: {e}", path.display()))
}

/// Put `occupant` at `rel` (the non-committed members; the committed one is landed before
/// the milestone exists). `conformant` is a body of the doctype to derive the bytes from.
fn plant(corpus: &TrialCorpus, occupant: Occupant, rel: &str, conformant: &str, its_body: &str) {
    let path = corpus.repo().join(rel);
    fs::create_dir_all(path.parent().expect("a home dir")).expect("mk the home dir");
    let bytes = match occupant {
        Occupant::UntrackedForeign => format!("a hand-written note\n\n{OCCUPANT_MARKER}\n"),
        Occupant::UntrackedConformant | Occupant::Staged => {
            assert!(
                conformant.contains(its_body),
                "the premise: the body to derive from carries `{its_body}`; got:\n{conformant}",
            );
            conformant.replace(its_body, OCCUPANT_MARKER)
        }
        Occupant::Committed => unreachable!("a committed occupant is landed, not planted"),
    };
    fs::write(&path, bytes).expect("plant the occupant");
    if occupant == Occupant::Staged {
        corpus.git(&["add", "--", rel]);
    }
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

/// Run an emitted command through a real `sh` split, so its quoting is adjudicated by the
/// thing that will parse it.
fn run_emitted(corpus: &TrialCorpus, command: &str) -> Output {
    let argv = support::shell_words(command, &corpus.repo(), &corpus.home());
    assert_eq!(argv.first().map(String::as_str), Some("jigc"), "{command}");
    let args: Vec<&str> = argv.iter().skip(1).map(String::as_str).collect();
    corpus.jigc(&args)
}

/// `jigc milestone finalize --format json`, asserted **blocked at exit 3**; returns the
/// findings array.
fn blocked(corpus: &TrialCorpus, what: &str) -> Vec<serde_json::Value> {
    let out = corpus.jigc(&["milestone", "finalize", MILESTONE, "--format", "json"]);
    assert_eq!(
        out.status.code(),
        Some(3),
        "{what}: the boundary is blocked at exit 3 — a `created` doc never promotes over an \
         entry already at its destination; {}",
        text(&out),
    );
    let envelope: serde_json::Value = serde_json::from_slice(&out.stdout).unwrap_or_else(|e| {
        panic!(
            "{what}: the findings envelope on stdout ({e}); {}",
            text(&out)
        )
    });
    envelope["findings"]
        .as_array()
        .unwrap_or_else(|| panic!("{what}: a `findings` array; {}", text(&out)))
        .iter()
        .filter(|f| f["severity"] == "blocking")
        .cloned()
        .collect()
}

/// The one clobber finding keyed at `destination`.
fn clobber_at<'f>(
    findings: &'f [serde_json::Value],
    destination: &str,
    what: &str,
) -> &'f serde_json::Value {
    let mut matching = findings.iter().filter(|f| {
        f["key"]["code"] == CLOBBER && f["key"]["target"].as_str() == Some(destination)
    });
    let finding = matching
        .next()
        .unwrap_or_else(|| panic!("{what}: `{CLOBBER}` keyed at `{destination}`; {findings:#?}"));
    assert!(matching.next().is_none(), "{what}: one per destination");
    finding
}

/// **Follow the routes as printed** — every `jigc doc rename` span each finding carries
/// (deduplicated: two findings of one collision group name overlapping sub-tasks), its
/// `"<title>"` placeholder filled with a title of the caller's, then the `jigc milestone
/// finalize` span verbatim — and assert the milestone lands at exit 0.
fn follow_the_routes(corpus: &TrialCorpus, findings: &[serde_json::Value], what: &str) {
    let mut renames: Vec<String> = Vec::new();
    let mut finalize: Option<String> = None;
    for finding in findings {
        let route = finding["route"]
            .as_str()
            .unwrap_or_else(|| panic!("{what}: a blocking finding carries a route; {finding:#}"));
        let these = spans(route, "jigc doc rename");
        assert!(
            !these.is_empty(),
            "{what}: the route names the in-task rename that moves the doc; got: {route}",
        );
        for span in these {
            if !renames.iter().any(|r| r == span) {
                renames.push(span.to_owned());
            }
        }
        let boundary = spans(route, "jigc milestone finalize");
        assert_eq!(
            boundary,
            vec![format!("jigc milestone finalize {MILESTONE}").as_str()],
            "{what}: the route ends at this milestone's own boundary; got: {route}",
        );
        finalize = Some(boundary[0].to_owned());
        assert!(
            !route.contains("jigc task finalize"),
            "{what}: a sub-task has no boundary of its own — the route must not name one; \
             got: {route}",
        );
        assert!(
            spans(route, "jigc migrate").is_empty() && spans(route, "jigc start").is_empty(),
            "{what}: adopting the occupant in its own task before the boundary moves HEAD \
             and blocks this milestone on `finalize.base-mismatch` — the route must not hand \
             back a command that walks into that; got: {route}",
        );
    }
    for (n, span) in renames.iter().enumerate() {
        assert!(
            span.contains("\"<title>\""),
            "{what}: the new title is the caller's to write; got: {span}",
        );
        let command = span.replace("\"<title>\"", &format!("'Moved Aside {n}'"));
        let out = run_emitted(corpus, &command);
        assert!(
            out.status.success(),
            "{what}: the emitted rename runs as printed (`{command}`); {}",
            text(&out),
        );
    }
    let out = run_emitted(corpus, &finalize.expect("a boundary span"));
    assert!(
        out.status.success(),
        "{what}: with the routes followed, the milestone lands; {}",
        text(&out),
    );
}

/// Assert `sub`'s authored doc is in `HEAD` under `kind`'s home.
fn assert_landed(corpus: &TrialCorpus, kind: Kind, sub: &str, what: &str) {
    let out = std::process::Command::new("git")
        .args(["grep", "-l", "-F", &body_of(sub), "HEAD", "--", kind.home()])
        .current_dir(corpus.repo())
        .output()
        .expect("spawn git grep");
    assert!(
        out.status.success(),
        "{what}: `{sub}`'s doc is in the landed commit — no sub-task's work is lost; \
         `git grep` found nothing under {}",
        kind.home(),
    );
}

/// One cell of the matrix: build it, assert the boundary refuses it and commits nothing,
/// follow the printed route, assert the milestone lands with everybody's bytes.
fn drive_cell(kind: Kind, landing: Landing, occupant: Occupant, squash: bool) {
    let what = format!("{kind:?} · {landing:?} · {occupant:?} · squash={squash}");
    let corpus = TrialCorpus::build(State::Fresh);
    if !squash {
        squash_false(&corpus);
    }
    // The occupied id: the suffix the join will mint, or the doc's own.
    let occupied_slug = match landing {
        Landing::Suffixed => format!("{SHARED_SLUG}-2"),
        Landing::Bare => SHARED_SLUG.to_owned(),
    };
    let destination = format!("{}/{occupied_slug}.md", kind.home());
    if occupant == Occupant::Committed {
        let landed = land_standalone(&corpus, kind, &format!("{SHARED_TITLE} 2"), OCCUPANT_MARKER);
        assert_eq!(
            landed,
            format!("{}:{occupied_slug}", kind.doctype()),
            "{what}: the premise — the committed occupant holds the id the suffix will mint",
        );
    }

    let subs = fan_out(&corpus, kind, 2);
    let minted = format!("{}:{SHARED_SLUG}", kind.doctype());
    // The sub-task whose doc lands on the occupant, and the other one.
    let (victim, other_title) = match landing {
        Landing::Suffixed => (&subs[1], SHARED_TITLE),
        Landing::Bare => (&subs[0], "Other Finding"),
    };
    assert_eq!(
        author(&corpus, kind, SHARED_TITLE, &subs[0]),
        minted,
        "{what}"
    );
    author(&corpus, kind, other_title, &subs[1]);
    if occupant != Occupant::Committed {
        let conformant = staged_bytes(&corpus, &subs[0], &minted);
        plant(
            &corpus,
            occupant,
            &destination,
            &conformant,
            &body_of(&subs[0]),
        );
    }

    let occupant_path: PathBuf = corpus.repo().join(&destination);
    let before = fs::read(&occupant_path).expect("the occupant is there");
    assert!(
        String::from_utf8_lossy(&before).contains(OCCUPANT_MARKER),
        "{what}: the before-control — the occupant carries its marker",
    );
    let head = corpus.git(&["rev-parse", "HEAD"]);

    // ── the boundary refuses ──
    let findings = blocked(&corpus, &what);
    assert_eq!(
        findings.len(),
        1,
        "{what}: one blocking finding; {findings:#?}"
    );
    let finding = clobber_at(&findings, &destination, &what);
    let message = finding["message"].as_str().expect("a message");
    assert!(
        message.contains(victim.as_str()) && message.contains(&minted),
        "{what}: the refusal names the sub-task and the doc it staged (`{victim}`, \
         `{minted}`) — the address the rename takes; got: {message}",
    );
    assert_eq!(
        corpus.git(&["rev-parse", "HEAD"]),
        head,
        "{what}: nothing was committed",
    );
    assert_eq!(
        fs::read(&occupant_path).expect("the occupant is still there"),
        before,
        "{what}: the occupant's bytes are untouched",
    );
    assert!(
        corpus
            .git(&["status", "--porcelain", "--", "docs/milestone-records"])
            .is_empty(),
        "{what}: the record's `active → joined` flip was restored — the milestone stays \
         finalizable",
    );

    // ── the route, as printed, lands the milestone ──
    follow_the_routes(&corpus, &findings, &what);
    for sub in &subs {
        assert_landed(&corpus, kind, sub, &what);
    }
    assert_eq!(
        fs::read(&occupant_path).expect("the occupant survives the landing"),
        before,
        "{what}: the landed milestone left the occupant's bytes exactly as they were",
    );
}

/// The reachable occupants of a landing. A **committed** occupant at an **unsuffixed** home
/// is not a cell: a create over a committed doc copies it in (`edited-from-base`) or, under
/// `new: true`, is refused — the doc is never `created`, and a commit that put the occupant
/// there afterwards moves `HEAD`, which the base guard answers first.
fn occupants_of(landing: Landing) -> Vec<Occupant> {
    Occupant::ALL
        .into_iter()
        .filter(|o| !(landing == Landing::Bare && *o == Occupant::Committed))
        .collect()
}

#[test]
fn a_suffix_landing_on_an_occupied_home_blocks_and_its_route_lands_the_milestone() {
    for occupant in occupants_of(Landing::Suffixed) {
        drive_cell(Kind::Idea, Landing::Suffixed, occupant, true);
    }
}

#[test]
fn an_occupant_at_an_unsuffixed_created_home_blocks_and_its_route_lands_the_milestone() {
    for occupant in occupants_of(Landing::Bare) {
        drive_cell(Kind::Idea, Landing::Bare, occupant, true);
    }
}

#[test]
fn the_per_sub_task_commit_model_blocks_the_suffixed_landing_too() {
    for occupant in occupants_of(Landing::Suffixed) {
        drive_cell(Kind::Idea, Landing::Suffixed, occupant, false);
    }
}

#[test]
fn the_per_sub_task_commit_model_blocks_the_unsuffixed_landing_too() {
    for occupant in occupants_of(Landing::Bare) {
        drive_cell(Kind::Idea, Landing::Bare, occupant, false);
    }
}

/// The doctype axis: another pack's doctype, and a create-only (`new: true`) one — over
/// both landings, since the guard keys on provenance and a home, never on a doctype.
#[test]
fn every_doctype_with_a_home_is_guarded_at_the_milestone_boundary() {
    for kind in Kind::ALL {
        if kind == Kind::Idea {
            continue; // the four tests above are its cells.
        }
        drive_cell(kind, Landing::Suffixed, Occupant::Committed, true);
        drive_cell(kind, Landing::Bare, Occupant::UntrackedForeign, true);
    }
}

/// **The record's own repro**, three reporters: a committed `<slug>-2` and an untracked
/// `<slug>-3`, both under the suffixes the join mints. One finding per occupied home, each
/// naming its own sub-task; the routes followed land all three docs and keep both occupants.
#[test]
fn every_occupied_suffix_is_reported_and_the_routes_land_all_three_sub_tasks() {
    let kind = Kind::Inconsistency;
    let corpus = TrialCorpus::build(State::Fresh);
    land_standalone(&corpus, kind, &format!("{SHARED_TITLE} 2"), OCCUPANT_MARKER);
    let subs = fan_out(&corpus, kind, 3);
    let minted = format!("{}:{SHARED_SLUG}", kind.doctype());
    for sub in &subs {
        assert_eq!(author(&corpus, kind, SHARED_TITLE, sub), minted);
    }
    let second = format!("{}/{SHARED_SLUG}-2.md", kind.home());
    let third = format!("{}/{SHARED_SLUG}-3.md", kind.home());
    let conformant = staged_bytes(&corpus, &subs[0], &minted);
    plant(
        &corpus,
        Occupant::UntrackedConformant,
        &third,
        &conformant,
        &body_of(&subs[0]),
    );
    let before: Vec<Vec<u8>> = [&second, &third]
        .iter()
        .map(|rel| fs::read(corpus.repo().join(rel)).expect("an occupant"))
        .collect();
    let head = corpus.git(&["rev-parse", "HEAD"]);

    let what = "three reporters over an occupied -2 and -3";
    let findings = blocked(&corpus, what);
    assert_eq!(
        findings.len(),
        2,
        "{what}: one per occupied home; {findings:#?}"
    );
    for (destination, sub) in [(&second, &subs[1]), (&third, &subs[2])] {
        let message = clobber_at(&findings, destination, what)["message"]
            .as_str()
            .expect("a message")
            .to_owned();
        assert!(
            message.contains(sub.as_str()),
            "{what}: `{destination}` is `{sub}`'s landing, by task id; got: {message}",
        );
    }
    assert_eq!(
        corpus.git(&["rev-parse", "HEAD"]),
        head,
        "{what}: nothing committed"
    );

    follow_the_routes(&corpus, &findings, what);
    for sub in &subs {
        assert_landed(&corpus, kind, sub, what);
    }
    for (rel, bytes) in [&second, &third].iter().zip(&before) {
        assert_eq!(
            &fs::read(corpus.repo().join(rel)).expect("the occupant survives"),
            bytes,
            "{what}: `{rel}` is byte-identical after the landing",
        );
    }
}

/// **The route lands in one pass.** The join numbers a collision group in task-id order, so
/// renaming only the blocked doc out of a group of three moves the *next* one onto the same
/// occupied suffix — a route naming one rename would be followed into a second refusal. With
/// only `<slug>-2` occupied, the one finding names every sub-task from that suffix on, and
/// following it lands the milestone on the first re-run.
#[test]
fn a_blocked_suffix_names_every_later_sub_task_of_its_group_so_one_pass_lands() {
    let kind = Kind::Idea;
    let corpus = TrialCorpus::build(State::Fresh);
    land_standalone(&corpus, kind, &format!("{SHARED_TITLE} 2"), OCCUPANT_MARKER);
    let subs = fan_out(&corpus, kind, 3);
    for sub in &subs {
        author(&corpus, kind, SHARED_TITLE, sub);
    }
    let what = "three ideas, only -2 occupied";
    let findings = blocked(&corpus, what);
    assert_eq!(findings.len(), 1, "{what}: `-3` is free; {findings:#?}");
    let route = findings[0]["route"].as_str().expect("a route");
    let renames = spans(route, "jigc doc rename");
    for sub in &subs[1..] {
        assert!(
            renames
                .iter()
                .any(|span| span.ends_with(&format!("--task {sub}"))),
            "{what}: the route names `{sub}`'s rename — it is at or after the occupied \
             suffix; got: {route}",
        );
    }
    assert!(
        !renames
            .iter()
            .any(|span| span.ends_with(&format!("--task {}", subs[0]))),
        "{what}: the first sub-task keeps the bare id and is not asked to move; got: {route}",
    );
    follow_the_routes(&corpus, &findings, what);
    for sub in &subs {
        assert_landed(&corpus, kind, sub, what);
    }
}

/// **No sub-task's staged code is lost either.** The real fan-out shape: provisioned
/// worktrees, each sub-task staging code beside its doc. A blocked boundary leaves every
/// worktree's index as it was, and the route followed lands code and docs in one commit.
#[test]
fn a_blocked_boundary_keeps_each_worktrees_staged_code_and_the_route_lands_it() {
    let kind = Kind::Adr;
    let corpus = TrialCorpus::build(State::Fresh);
    let subs = fan_out(&corpus, kind, 2);
    corpus.jigc_ok(&["milestone", "provision", MILESTONE]);
    let minted = format!("{}:{SHARED_SLUG}", kind.doctype());
    for sub in &subs {
        author(&corpus, kind, SHARED_TITLE, sub);
        let worktree = corpus.repo().join(".jigc/worktrees").join(sub);
        fs::create_dir_all(worktree.join("src")).expect("mk src/");
        fs::write(worktree.join(format!("src/{sub}.rs")), "pub fn work() {}\n").expect("code");
        let out = std::process::Command::new("git")
            .args(["add", "--", &format!("src/{sub}.rs")])
            .current_dir(&worktree)
            .output()
            .expect("spawn git add");
        assert!(out.status.success(), "stage `{sub}`'s code");
    }
    let destination = format!("{}/{SHARED_SLUG}-2.md", kind.home());
    let conformant = staged_bytes(&corpus, &subs[0], &minted);
    plant(
        &corpus,
        Occupant::UntrackedConformant,
        &destination,
        &conformant,
        &body_of(&subs[0]),
    );
    let before = fs::read(corpus.repo().join(&destination)).expect("the occupant");

    let what = "provisioned worktrees with staged code";
    let findings = blocked(&corpus, what);
    clobber_at(&findings, &destination, what);
    for sub in &subs {
        let worktree = corpus.repo().join(".jigc/worktrees").join(sub);
        let out = std::process::Command::new("git")
            .args(["diff", "--cached", "--name-only"])
            .current_dir(&worktree)
            .output()
            .expect("spawn git diff");
        assert_eq!(
            String::from_utf8_lossy(&out.stdout).trim(),
            format!("src/{sub}.rs"),
            "{what}: `{sub}`'s staged code is still staged in its worktree",
        );
    }

    follow_the_routes(&corpus, &findings, what);
    for sub in &subs {
        assert_landed(&corpus, kind, sub, what);
        corpus.git(&["cat-file", "-e", &format!("HEAD:src/{sub}.rs")]);
    }
    assert_eq!(
        fs::read(corpus.repo().join(&destination)).expect("the occupant survives"),
        before,
        "{what}: the occupant is byte-identical after the landing",
    );
}

/// **Control — a free suffix still lands.** The guard is about an occupied home; rule 4's
/// suffix onto a free id is the join working, and stays exit 0.
#[test]
fn a_suffix_landing_on_a_free_id_still_lands() {
    for kind in Kind::ALL {
        let corpus = TrialCorpus::build(State::Fresh);
        let subs = fan_out(&corpus, kind, 2);
        for sub in &subs {
            author(&corpus, kind, SHARED_TITLE, sub);
        }
        corpus.jigc_ok(&["milestone", "finalize", MILESTONE]);
        for (slug, sub) in [
            (SHARED_SLUG.to_owned(), &subs[0]),
            (format!("{SHARED_SLUG}-2"), &subs[1]),
        ] {
            let body =
                support::trial_corpus::read(&corpus.repo(), &format!("{}/{slug}.md", kind.home()));
            assert!(
                body.contains(&body_of(sub)),
                "{kind:?}: `{slug}` carries `{sub}`'s body, by task id; got:\n{body}",
            );
        }
    }
}

/// **Control — an update of a committed doc still lands.** A sub-task whose create copied a
/// committed doc in holds it `edited-from-base`; re-promoting it over its own home is the
/// update path, never a clobber.
#[test]
fn a_sub_task_update_of_a_committed_doc_still_lands() {
    for kind in [Kind::Idea, Kind::Adr] {
        let corpus = TrialCorpus::build(State::Fresh);
        let address = land_standalone(&corpus, kind, SHARED_TITLE, "The first version.");
        let subs = fan_out(&corpus, kind, 1);
        let ack = create(&corpus, kind, SHARED_TITLE, &subs[0]);
        assert!(
            ack.starts_with(&address) && ack.contains("copied in for update"),
            "{kind:?}: the premise — the create copies the committed doc in; got: {ack}",
        );
        kind.fill(&corpus, &address, &subs[0], &body_of(&subs[0]));
        corpus.jigc_ok(&["milestone", "finalize", MILESTONE]);
        let body = support::trial_corpus::read(
            &corpus.repo(),
            &format!("{}/{SHARED_SLUG}.md", kind.home()),
        );
        assert!(
            body.contains(&body_of(&subs[0])),
            "{kind:?}: the sub-task's update of the committed doc landed; got:\n{body}",
        );
    }
}

/// **The statement, where the door is described.** The boundary's refusal is stated in the
/// two helps an agent reads before it runs the fan-out's last two commands, and in the
/// design sentence that used to claim the guard was inherited everywhere.
#[test]
fn the_helps_and_the_design_state_the_boundary_refusal() {
    let corpus = TrialCorpus::build(State::Fresh);
    let finalize = corpus.jigc_ok(&["milestone", "finalize", "--help"]);
    assert!(
        finalize.contains(CLOBBER),
        "`jigc milestone finalize --help` names the refusal; got:\n{finalize}",
    );
    let join = corpus.jigc_ok(&["milestone", "join", "--help"]);
    assert!(
        join.contains(CLOBBER),
        "`jigc milestone join --help` says its suffix does not consult the store and names \
         the boundary's refusal; got:\n{join}",
    );
    for (doc, needle) in [
        ("design/finalize.md", "milestone boundary"),
        ("design/storage.md", CLOBBER),
    ] {
        let body = fs::read_to_string(format!("{}/../../{doc}", env!("CARGO_MANIFEST_DIR")))
            .unwrap_or_else(|e| panic!("read {doc}: {e}"));
        assert!(
            body.contains(CLOBBER) && body.contains(needle),
            "{doc} states the milestone boundary's clobber refusal",
        );
    }
}
