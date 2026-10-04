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

// ─────────────────────────────────────────────────────────────────────────────────────────
// A home git holds and the disk does not (the completion audit's CPL-5)
// ─────────────────────────────────────────────────────────────────────────────────────────
//
// The guard above asked the disk, and a home with nothing on disk is not thereby free: `HEAD`
// may hold a committed doc whose file was deleted from the worktree, and the index a file
// staged and then taken out of it. In a checkout that holds no `file-state` key for the doc —
// every fresh clone — a doc *minted* at that id landed under it at exit 0, at `jigc task
// finalize` and `jigc milestone finalize`, and `jigc milestone create` minted a record over a
// committed one. The axis is
//
// - **holder** — `HEAD` (a committed doc, deleted uncommitted) · the index alone (staged,
//   never committed, then taken out of the worktree);
// - **door** — `jigc task finalize` · `jigc milestone finalize` · `jigc milestone create`;
// - **identity** — one its author chooses (`adr`) · a fixed one (`vision`), whose only exit
//   is dropping the mint;
//
// each refused cell driven out through the route it printed, as emitted. The MUST NOT REFUSE
// cells are the states the question must not touch: a home nobody holds, and the update of a
// committed doc, under every line-ending setting the question could have met — it reads no
// byte, and `linked_worktree_doc_home`'s layout matrix (a bare repository's worktree, a
// `--separate-git-dir` checkout, a submodule) lands a created doc through this same door.

const VISION: &str = "vision:vision";
const VISION_HOME: &str = "VISION.md";

/// Where git holds the occupant the worktree does not show.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Holder {
    Head,
    Index,
}

/// Author every commit-doc leaf `task` needs, without finalizing.
fn fill_commit(corpus: &TrialCorpus, task: &str, scope: &str) {
    corpus.set_field(&format!("commit:{task}#type"), task, "docs");
    corpus.set_field(&format!("commit:{task}#scope"), task, scope);
    corpus.set_slot(&format!("commit:{task}#summary"), task, "file a finding");
    corpus.set_slot(&format!("commit:{task}#body"), task, "Driven by the suite.");
}

/// Create and author the `vision` singleton in `task`; returns the create's ack.
fn author_vision(corpus: &TrialCorpus, task: &str, prose: &str) -> String {
    let ack = corpus.jigc_ok(&[
        "doc", "create", "vision", "--title", "Vision", "--task", task,
    ]);
    for slot in ["thesis", "invariants", "open-questions"] {
        corpus.set_slot(&format!("{VISION}#{slot}"), task, prose);
    }
    ack.trim().to_owned()
}

/// Land the `vision` singleton through its own task, carrying `prose`.
fn land_vision(corpus: &TrialCorpus, prose: &str) {
    let task = corpus.start_workflow("form-vision", "form the vision");
    author_vision(corpus, &task, prose);
    corpus.finalize(&task, "vision", "form the vision", false);
}

/// `jigc task finalize <task> --format json`, asserted **blocked at exit 3**; returns the
/// blocking findings.
fn task_blocked(corpus: &TrialCorpus, task: &str, what: &str) -> Vec<serde_json::Value> {
    let out = corpus.jigc(&["task", "finalize", task, "--format", "json"]);
    assert_eq!(
        out.status.code(),
        Some(3),
        "{what}: the task door is blocked at exit 3; {}",
        text(&out),
    );
    let envelope: serde_json::Value = serde_json::from_slice(&out.stdout)
        .unwrap_or_else(|e| panic!("{what}: the findings envelope ({e}); {}", text(&out)));
    envelope["findings"]
        .as_array()
        .unwrap_or_else(|| panic!("{what}: a `findings` array; {}", text(&out)))
        .iter()
        .filter(|f| f["severity"] == "blocking")
        .cloned()
        .collect()
}

/// Run an emitted span — `jigc …` against the corpus binary, `git …` as git — through a real
/// `sh` split, and return its output.
fn run_span(corpus: &TrialCorpus, command: &str) -> Output {
    let argv = support::shell_words(command, &corpus.repo(), &corpus.home());
    let args: Vec<&str> = argv.iter().skip(1).map(String::as_str).collect();
    match argv.first().map(String::as_str) {
        Some("jigc") => corpus.jigc(&args),
        Some("git") => std::process::Command::new("git")
            .args(&args)
            .current_dir(corpus.repo())
            .env("HOME", corpus.home())
            .output()
            .expect("spawn git"),
        other => panic!("an emitted span runs `jigc` or `git`; got {other:?} in `{command}`"),
    }
}

/// The one span of `route` opening with `lead`, run as printed and asserted to succeed.
fn run_the_span(corpus: &TrialCorpus, route: &str, lead: &str, what: &str) -> Output {
    let found = spans(route, lead);
    assert_eq!(
        found.len(),
        1,
        "{what}: exactly one `{lead} …` span; got: {route}"
    );
    let out = run_span(corpus, found[0]);
    assert!(
        out.status.success(),
        "{what}: `{}` runs as printed; {}",
        found[0],
        text(&out),
    );
    out
}

/// Take a committed doc's file out of the worktree in a checkout that holds no `file-state`
/// key for it — the fresh-clone shape, where no baseline blocks on `reconciliation.rename`.
fn delete_uncommitted(corpus: &TrialCorpus, rel: &str) {
    corpus.fresh_clone_shape();
    fs::remove_file(corpus.repo().join(rel)).expect("take the committed file out");
    assert_eq!(
        corpus.git(&["status", "--porcelain", "--", rel]),
        format!("D {rel}"),
        "the premise: `{rel}` is deleted in the worktree and nowhere else",
    );
}

/// `git show HEAD:<rel>`.
fn at_head(corpus: &TrialCorpus, rel: &str) -> String {
    corpus.git(&["show", &format!("HEAD:{rel}")])
}

/// **The task door.** A doc minted over a home git holds is refused, keyed at the home, with
/// nothing committed; the rename the route prints lands the task's doc beside what git holds,
/// and the restore it prints brings that file back, byte for byte.
#[test]
fn a_home_git_holds_and_the_disk_does_not_is_occupied_at_the_task_door() {
    let kind = Kind::Adr;
    let destination = format!("{}/{SHARED_SLUG}.md", kind.home());
    for holder in [Holder::Head, Holder::Index] {
        let what = format!("task door · {holder:?}");
        let corpus = TrialCorpus::build(State::Fresh);
        if holder == Holder::Head {
            land_standalone(&corpus, kind, SHARED_TITLE, OCCUPANT_MARKER);
            delete_uncommitted(&corpus, &destination);
        }
        let task = corpus.start_workflow("record-decision", "file it again");
        let minted = create(&corpus, kind, SHARED_TITLE, &task);
        assert_eq!(
            minted,
            format!("adr:{SHARED_SLUG}"),
            "{what}: the premise — the create saw a free home and minted, copying nothing in",
        );
        kind.fill(&corpus, &minted, &task, &body_of(&task));
        fill_commit(&corpus, &task, "adr");
        if holder == Holder::Index {
            let conformant = staged_bytes(&corpus, &task, &minted);
            plant(
                &corpus,
                Occupant::Staged,
                &destination,
                &conformant,
                &body_of(&task),
            );
            fs::remove_file(corpus.repo().join(&destination)).expect("take the staged file out");
        }
        let status = corpus.git(&["status", "--porcelain", "--", &destination]);
        let head = corpus.git(&["rev-parse", "HEAD"]);

        let findings = task_blocked(&corpus, &task, &what);
        assert_eq!(findings.len(), 1, "{what}: one finding; {findings:#?}");
        let finding = clobber_at(&findings, &destination, &what);
        let message = finding["message"].as_str().expect("a message");
        let names = match holder {
            Holder::Head => "`HEAD`",
            Holder::Index => "index",
        };
        assert!(
            message.contains("missing from the worktree") && message.contains(names),
            "{what}: the refusal says where the occupant is; got: {message}",
        );
        assert_eq!(
            corpus.git(&["rev-parse", "HEAD"]),
            head,
            "{what}: nothing committed"
        );
        assert_eq!(
            corpus.git(&["status", "--porcelain", "--", &destination]),
            status,
            "{what}: what git holds at the home is untouched",
        );

        // ── the route, as printed ──
        let route = finding["route"].as_str().expect("a route");
        let rename = spans(route, "jigc doc rename");
        assert_eq!(rename.len(), 1, "{what}: one rename; got: {route}");
        let renamed = run_span(&corpus, &rename[0].replace("\"<title>\"", "'Moved Aside'"));
        assert!(renamed.status.success(), "{what}: {}", text(&renamed));
        run_the_span(&corpus, route, "jigc task finalize", &what);
        assert_landed(&corpus, kind, &task, &what);
        if holder == Holder::Head {
            assert!(
                at_head(&corpus, &destination).contains(OCCUPANT_MARKER),
                "{what}: the committed doc is still the committed doc — not replaced under \
                 its id",
            );
        }
        run_the_span(&corpus, route, "git -C", &what);
        assert!(
            support::trial_corpus::read(&corpus.repo(), &destination).contains(OCCUPANT_MARKER),
            "{what}: the restore the route prints brings the file git held back",
        );
    }
}

/// **A fixed identity has no other id**, so the route prints no rename — it would refuse —
/// and hands back the drop of the mint: the task is read, discarded by consent, the committed
/// doc restored, and the next create copies it in for update.
#[test]
fn a_fixed_identity_doc_minted_over_a_home_git_holds_routes_at_dropping_the_mint() {
    let what = "task door · fixed identity";
    let corpus = TrialCorpus::build(State::Fresh);
    land_vision(&corpus, OCCUPANT_MARKER);
    delete_uncommitted(&corpus, VISION_HOME);
    let task = corpus.start_workflow("form-vision", "form it again");
    assert_eq!(
        author_vision(&corpus, &task, "A second vision."),
        VISION,
        "{what}: the premise — the create minted, copying nothing in",
    );
    fill_commit(&corpus, &task, "vision");
    let head = corpus.git(&["rev-parse", "HEAD"]);

    let findings = task_blocked(&corpus, &task, what);
    let route = clobber_at(&findings, VISION_HOME, what)["route"]
        .as_str()
        .expect("a route")
        .to_owned();
    assert!(
        spans(&route, "jigc doc rename").is_empty(),
        "{what}: a singleton cannot be renamed, and a route never hands back a command that \
         refuses; got: {route}",
    );
    assert_eq!(corpus.git(&["rev-parse", "HEAD"]), head, "{what}");

    let shown = run_the_span(&corpus, &route, "jigc doc show", what);
    assert!(
        String::from_utf8_lossy(&shown.stdout).contains("A second vision."),
        "{what}: the read the route prints shows what the drop would take",
    );
    run_the_span(&corpus, &route, "jigc task discard", what);
    run_the_span(&corpus, &route, "git -C", what);
    assert!(
        support::trial_corpus::read(&corpus.repo(), VISION_HOME).contains(OCCUPANT_MARKER),
        "{what}: the committed vision is back, as committed",
    );
    assert!(
        corpus.git(&["status", "--porcelain"]).is_empty(),
        "{what}: the tree is clean — nothing was committed and nothing is left over",
    );
    let again = corpus.start_workflow("form-vision", "form it once more");
    assert!(
        author_vision(&corpus, &again, "An update.").contains("copied in for update"),
        "{what}: started again, the create copies the committed doc in instead of minting",
    );
}

/// **The milestone boundary asks the same question**, over the same predicate: a sub-task's
/// minted doc over a home git holds is refused, and each identity's route lands the
/// milestone with the committed doc still the committed doc.
#[test]
fn the_milestone_boundary_asks_git_about_a_home_the_disk_shows_free() {
    // An id the author chooses: the in-task rename, then this milestone's boundary.
    {
        let (kind, what) = (Kind::Adr, "milestone door · adr");
        let destination = format!("{}/{SHARED_SLUG}.md", kind.home());
        let corpus = TrialCorpus::build(State::Fresh);
        land_standalone(&corpus, kind, SHARED_TITLE, OCCUPANT_MARKER);
        delete_uncommitted(&corpus, &destination);
        let subs = fan_out(&corpus, kind, 1);
        assert_eq!(
            author(&corpus, kind, SHARED_TITLE, &subs[0]),
            format!("adr:{SHARED_SLUG}"),
            "{what}: the premise — minted, not copied in",
        );
        let head = corpus.git(&["rev-parse", "HEAD"]);
        let findings = blocked(&corpus, what);
        assert_eq!(findings.len(), 1, "{what}: {findings:#?}");
        let route = clobber_at(&findings, &destination, what)["route"]
            .as_str()
            .expect("a route")
            .to_owned();
        assert_eq!(corpus.git(&["rev-parse", "HEAD"]), head, "{what}");
        follow_the_routes(&corpus, &findings, what);
        assert_landed(&corpus, kind, &subs[0], what);
        assert!(
            at_head(&corpus, &destination).contains(OCCUPANT_MARKER),
            "{what}: the committed doc was not replaced under its id",
        );
        run_the_span(&corpus, &route, "git -C", what);
        assert!(
            support::trial_corpus::read(&corpus.repo(), &destination).contains(OCCUPANT_MARKER),
            "{what}: the restore brings the committed doc back",
        );
    }
    // A fixed identity: the sub-task that minted it is dropped, and the rest lands.
    {
        let what = "milestone door · vision";
        let corpus = TrialCorpus::build(State::Fresh);
        land_vision(&corpus, OCCUPANT_MARKER);
        delete_uncommitted(&corpus, VISION_HOME);
        corpus.jigc_ok(&["milestone", "create", MILESTONE_TITLE]);
        corpus.jigc_ok(&[
            "milestone",
            "add-task",
            MILESTONE,
            "alpha forms the vision",
            "--workflow",
            "form-vision",
        ]);
        corpus.jigc_ok(&["milestone", "add-task", MILESTONE, "bravo decides"]);
        let (alpha, bravo) = ("alpha-forms-the-vision", "bravo-decides");
        assert_eq!(author_vision(&corpus, alpha, "A second vision."), VISION);
        author(&corpus, Kind::Adr, "Bravo Decision", bravo);

        let findings = blocked(&corpus, what);
        assert_eq!(findings.len(), 1, "{what}: {findings:#?}");
        let finding = clobber_at(&findings, VISION_HOME, what);
        let route = finding["route"].as_str().expect("a route");
        assert!(
            spans(route, "jigc doc rename").is_empty(),
            "{what}: no rename for a singleton; got: {route}",
        );
        run_the_span(&corpus, route, "jigc doc show", what);
        let discard = run_the_span(&corpus, route, "jigc task discard", what);
        assert!(
            text(&discard).contains(alpha),
            "{what}: the drop names the sub-task that minted the doc",
        );
        run_the_span(&corpus, route, "jigc milestone finalize", what);
        assert_landed(&corpus, Kind::Adr, bravo, what);
        assert!(
            at_head(&corpus, VISION_HOME).contains(OCCUPANT_MARKER),
            "{what}: the committed vision was not replaced under its id",
        );
        run_the_span(&corpus, route, "git -C", what);
        assert!(
            support::trial_corpus::read(&corpus.repo(), VISION_HOME).contains(OCCUPANT_MARKER),
            "{what}: the restore brings the committed vision back",
        );
    }
}

/// **The third door of the class: `jigc milestone create`.** A committed record deleted from
/// the worktree, in a checkout with no workbench for the milestone, read as a free id and was
/// minted over. It refuses under the id-is-taken code, and the restore it prints lets the
/// milestone be continued.
#[test]
fn milestone_create_refuses_an_id_whose_record_git_still_holds() {
    let corpus = TrialCorpus::build(State::Fresh);
    let subs = fan_out(&corpus, Kind::Adr, 1);
    let record = format!("docs/milestone-records/{MILESTONE}.md");
    let committed = at_head(&corpus, &record);
    // A clone's shape: the committed record, and none of the gitignored workbench.
    fs::remove_dir_all(corpus.repo().join(".jigc/milestones").join(MILESTONE))
        .expect("drop the milestone area");
    fs::remove_dir_all(corpus.repo().join(".jigc/tasks").join(&subs[0]))
        .expect("drop the sub-task area");
    delete_uncommitted(&corpus, &record);
    let head = corpus.git(&["rev-parse", "HEAD"]);

    let out = corpus.jigc(&["milestone", "create", MILESTONE_TITLE]);
    let said = text(&out);
    assert_eq!(out.status.code(), Some(1), "the create is refused; {said}");
    assert!(
        said.contains("milestone.record-exists") && said.contains("missing from the worktree"),
        "the id belongs to the record git holds; {said}",
    );
    assert_eq!(
        corpus.git(&["rev-parse", "HEAD"]),
        head,
        "nothing committed"
    );
    assert!(
        !corpus.repo().join(&record).exists(),
        "nothing written at the record's home",
    );

    let stderr = String::from_utf8_lossy(&out.stderr).into_owned();
    run_the_span(&corpus, &stderr, "git -C", "milestone create");
    assert_eq!(
        support::trial_corpus::read(&corpus.repo(), &record).trim(),
        committed,
        "the restore brings the committed record back",
    );
    corpus.jigc_ok(&["milestone", "add-task", MILESTONE, "bravo files a finding"]);
    // MUST NOT REFUSE: an id no record holds mints as before.
    corpus.jigc_ok(&["milestone", "create", "another milestone"]);
}

/// The line-ending settings the question could have met. It reads no byte, so none of them
/// may change its answer.
#[derive(Clone, Copy, Debug)]
enum Conversion {
    None,
    AutocrlfTrue,
    AutocrlfInput,
    /// An uncommitted `.gitattributes` carrying `* text=auto`.
    TextAuto,
    /// A clean/smudge filter over every Markdown file.
    Filter,
}

impl Conversion {
    const ALL: [Conversion; 5] = [
        Conversion::None,
        Conversion::AutocrlfTrue,
        Conversion::AutocrlfInput,
        Conversion::TextAuto,
        Conversion::Filter,
    ];

    fn apply(self, corpus: &TrialCorpus) {
        match self {
            Conversion::None => {}
            Conversion::AutocrlfTrue => {
                corpus.git(&["config", "core.autocrlf", "true"]);
            }
            Conversion::AutocrlfInput => {
                corpus.git(&["config", "core.autocrlf", "input"]);
            }
            Conversion::TextAuto => {
                fs::write(corpus.repo().join(".gitattributes"), "* text=auto\n")
                    .expect("write .gitattributes");
            }
            Conversion::Filter => {
                corpus.git(&["config", "filter.keep.clean", "cat"]);
                corpus.git(&["config", "filter.keep.smudge", "cat"]);
                fs::write(corpus.repo().join(".gitattributes"), "*.md filter=keep\n")
                    .expect("write .gitattributes");
            }
        }
    }
}

/// **MUST NOT REFUSE.** Under every conversion setting, in a plain checkout and in the
/// fresh-clone shape: a doc minted at a home nobody holds lands at the task door; the update
/// of a committed doc — a home `HEAD` and the index *do* hold, with the file on disk — lands;
/// and two sub-tasks' minted docs land at the milestone boundary, suffix and all.
#[test]
fn a_home_nobody_holds_and_an_update_land_under_every_conversion_setting() {
    let kind = Kind::Adr;
    for conversion in Conversion::ALL {
        let what = format!("{conversion:?}");
        let corpus = TrialCorpus::build(State::Fresh);
        conversion.apply(&corpus);

        // A minted doc at a free home — the task door.
        let address = land_standalone(&corpus, kind, "First Finding", "The first version.");
        // The update of that committed doc, in the fresh-clone shape.
        corpus.fresh_clone_shape();
        let task = corpus.start_workflow(kind.standalone_workflow(), "update it");
        let ack = create(&corpus, kind, "First Finding", &task);
        assert!(
            ack.starts_with(&address) && ack.contains("copied in for update"),
            "{what}: the premise — an update, not a mint; got: {ack}",
        );
        kind.fill(&corpus, &address, &task, &body_of(&task));
        let out = {
            fill_commit(&corpus, &task, "adr");
            corpus.jigc(&["task", "finalize", &task])
        };
        assert!(
            out.status.success(),
            "{what}: the update of a committed doc lands; {}",
            text(&out),
        );
        assert_landed(&corpus, kind, &task, &what);

        // Two minted docs at the milestone boundary, one suffixed onto a free id.
        let subs = fan_out(&corpus, kind, 2);
        for sub in &subs {
            author(&corpus, kind, SHARED_TITLE, sub);
        }
        let out = corpus.jigc(&["milestone", "finalize", MILESTONE]);
        assert!(
            out.status.success(),
            "{what}: the boundary lands minted docs at free homes; {}",
            text(&out),
        );
        for sub in &subs {
            assert_landed(&corpus, kind, sub, &what);
        }
    }
}

// ─────────────────────────────────────────────────────────────────────────────────────────
// A fixed identity over an occupied home (the completion audit's CPL-3)
// ─────────────────────────────────────────────────────────────────────────────────────────
//
// The matrix at the top of this file has no placement singleton in it, and that missing axis
// member hid a dead end: the file arm routes at giving the doc another id, and a singleton
// has none — `jigc doc rename vision:vision …` answers `write.identity-change`. So the
// doctype axis gains its fixed-identity member, at both committing doors, over
//
// - **occupant** — an untracked hand-written file · a file staged and never committed · the
//   committed doc itself (reachable only through the absent-home state above);
// - **exit** — the home freed, with no commit · the mint dropped;
//
// and each exit is driven as printed to a landed end state with every byte accounted for.

/// The first span of `route` opening with `lead`, run as printed and asserted to succeed —
/// for a route that names one command under two exits.
fn run_first_span(corpus: &TrialCorpus, route: &str, lead: &str, what: &str) -> Output {
    let found = spans(route, lead);
    let span = found
        .first()
        .unwrap_or_else(|| panic!("{what}: a `{lead} …` span; got: {route}"));
    let out = run_span(corpus, span);
    assert!(
        out.status.success(),
        "{what}: `{span}` runs as printed; {}",
        text(&out),
    );
    out
}

/// A hand-written file at the vision's home — nothing jigc ever saw.
fn plant_hand_written_vision(corpus: &TrialCorpus, staged: bool) {
    fs::write(
        corpus.repo().join(VISION_HOME),
        format!("# Our direction\n\n{OCCUPANT_MARKER}\n"),
    )
    .expect("plant the occupant");
    if staged {
        corpus.git(&["add", "--", VISION_HOME]);
    }
}

/// A milestone whose `alpha` sub-task mints the vision and whose `bravo` sub-task mints an
/// ADR; returns `(alpha, bravo)`.
fn vision_fan_out(corpus: &TrialCorpus) -> (&'static str, &'static str) {
    corpus.jigc_ok(&["milestone", "create", MILESTONE_TITLE]);
    corpus.jigc_ok(&[
        "milestone",
        "add-task",
        MILESTONE,
        "alpha forms the vision",
        "--workflow",
        "form-vision",
    ]);
    corpus.jigc_ok(&["milestone", "add-task", MILESTONE, "bravo decides"]);
    let (alpha, bravo) = ("alpha-forms-the-vision", "bravo-decides");
    assert_eq!(author_vision(corpus, alpha, &body_of(alpha)), VISION);
    author(corpus, Kind::Adr, "Bravo Decision", bravo);
    (alpha, bravo)
}

/// Assert a refusal's route hands a fixed-identity doc no command that refuses.
fn assert_no_rename(route: &str, what: &str) {
    assert!(
        spans(route, "jigc doc rename").is_empty() && !route.contains("--slug"),
        "{what}: a singleton cannot be renamed or re-slugged, and a route never hands back a \
         command that refuses; got: {route}",
    );
}

/// **The milestone boundary, the home freed.** The file is moved out of the doc's home — no
/// commit — and the boundary's own command lands every sub-task's work, the vision at its
/// home, with the moved file's bytes intact.
#[test]
fn the_boundary_routes_a_fixed_identity_at_freeing_the_home() {
    for staged in [false, true] {
        let what = format!("milestone door · vision · free the home · staged={staged}");
        let corpus = TrialCorpus::build(State::Fresh);
        let (alpha, bravo) = vision_fan_out(&corpus);
        plant_hand_written_vision(&corpus, staged);
        let head = corpus.git(&["rev-parse", "HEAD"]);

        let findings = blocked(&corpus, &what);
        assert_eq!(findings.len(), 1, "{what}: {findings:#?}");
        let route = clobber_at(&findings, VISION_HOME, &what)["route"]
            .as_str()
            .expect("a route")
            .to_owned();
        assert_no_rename(&route, &what);
        assert!(
            route.contains("move the file out of the doc's home"),
            "{what}: the route names the exit that needs no commit; got: {route}",
        );
        assert_eq!(corpus.git(&["rev-parse", "HEAD"]), head, "{what}");

        // The move is the file's owner's act: by hand, or with `git mv` for a staged one.
        if staged {
            corpus.git(&["mv", VISION_HOME, "VISION.hand.md"]);
        } else {
            fs::rename(
                corpus.repo().join(VISION_HOME),
                corpus.repo().join("VISION.hand.md"),
            )
            .expect("move the file out of the doc's home");
        }
        run_first_span(&corpus, &route, "jigc milestone finalize", &what);
        assert!(
            at_head(&corpus, VISION_HOME).contains(&body_of(alpha)),
            "{what}: the sub-task's vision landed at its home",
        );
        assert_landed(&corpus, Kind::Adr, bravo, &what);
        assert!(
            support::trial_corpus::read(&corpus.repo(), "VISION.hand.md").contains(OCCUPANT_MARKER),
            "{what}: the moved file's bytes are the bytes that were there",
        );
    }
}

/// **The milestone boundary, the mint dropped.** The file stays; the sub-task that minted
/// the doc is read, then discarded by consent; the boundary lands the rest; and the file is
/// brought under management by the two commands the route prints for after the landing.
#[test]
fn the_boundary_routes_a_fixed_identity_at_dropping_the_mint() {
    let what = "milestone door · vision · drop the mint";
    let corpus = TrialCorpus::build(State::Fresh);
    let (alpha, bravo) = vision_fan_out(&corpus);
    plant_hand_written_vision(&corpus, false);
    let before = fs::read(corpus.repo().join(VISION_HOME)).expect("the occupant");

    let findings = blocked(&corpus, what);
    let route = clobber_at(&findings, VISION_HOME, what)["route"]
        .as_str()
        .expect("a route")
        .to_owned();
    let shown = run_the_span(&corpus, &route, "jigc doc show", what);
    assert!(
        String::from_utf8_lossy(&shown.stdout).contains(&body_of(alpha)),
        "{what}: the read the route prints shows what the drop would take",
    );
    run_the_span(&corpus, &route, "jigc task discard", what);
    // The second `jigc milestone finalize` span is this exit's — the same bytes as the first.
    run_first_span(&corpus, &route, "jigc milestone finalize", what);
    assert_landed(&corpus, Kind::Adr, bravo, what);
    assert_eq!(
        fs::read(corpus.repo().join(VISION_HOME)).expect("the occupant survives"),
        before,
        "{what}: the file at the home is byte-identical after the landing",
    );
    // …and after the landing, the adoption the route names, as printed.
    run_the_span(&corpus, &route, "git -C", what);
    let minted = run_the_span(&corpus, &route, "jigc migrate", what);
    assert!(
        String::from_utf8_lossy(&minted.stdout).contains("task minted:"),
        "{what}: `jigc migrate` mints the task that adopts the file; {}",
        text(&minted),
    );
}

/// **A committed occupant cannot leave its home without a commit**, so the route offers the
/// drop alone — never a move that the next run would refuse.
#[test]
fn a_committed_occupant_is_never_routed_at_moving_out() {
    let what = "milestone door · vision · committed occupant";
    let corpus = TrialCorpus::build(State::Fresh);
    land_vision(&corpus, OCCUPANT_MARKER);
    // The one way a minted singleton faces its own committed doc: created while the file
    // was out of the worktree, which is then restored.
    delete_uncommitted(&corpus, VISION_HOME);
    let (_, bravo) = vision_fan_out(&corpus);
    corpus.git(&["checkout", "HEAD", "--", VISION_HOME]);

    let findings = blocked(&corpus, what);
    let route = clobber_at(&findings, VISION_HOME, what)["route"]
        .as_str()
        .expect("a route")
        .to_owned();
    assert_no_rename(&route, what);
    assert!(
        !route.contains("move the file out") && spans(&route, "jigc migrate").is_empty(),
        "{what}: a committed file stays where it is, and is already managed; got: {route}",
    );
    run_the_span(&corpus, &route, "jigc task discard", what);
    run_the_span(&corpus, &route, "jigc milestone finalize", what);
    assert_landed(&corpus, Kind::Adr, bravo, what);
    assert!(
        at_head(&corpus, VISION_HOME).contains(OCCUPANT_MARKER),
        "{what}: the committed vision is still the committed vision",
    );
}

/// **The task door has the same dead end and the same exits.** Its file arm routed a
/// singleton at a retitle and an explicit `--slug`; both refuse.
#[test]
fn the_task_door_routes_a_fixed_identity_at_exits_it_has() {
    // The home freed.
    {
        let what = "task door · vision · free the home";
        let corpus = TrialCorpus::build(State::Fresh);
        let task = corpus.start_workflow("form-vision", "form the vision");
        assert_eq!(author_vision(&corpus, &task, &body_of(&task)), VISION);
        fill_commit(&corpus, &task, "vision");
        plant_hand_written_vision(&corpus, false);

        let findings = task_blocked(&corpus, &task, what);
        let route = clobber_at(&findings, VISION_HOME, what)["route"]
            .as_str()
            .expect("a route")
            .to_owned();
        assert_no_rename(&route, what);
        fs::rename(
            corpus.repo().join(VISION_HOME),
            corpus.repo().join("VISION.hand.md"),
        )
        .expect("move the file out of the doc's home");
        run_the_span(&corpus, &route, "jigc task finalize", what);
        assert!(
            at_head(&corpus, VISION_HOME).contains(&body_of(&task)),
            "{what}: the task's vision landed at its home",
        );
        assert!(
            support::trial_corpus::read(&corpus.repo(), "VISION.hand.md").contains(OCCUPANT_MARKER),
            "{what}: the moved file's bytes are the bytes that were there",
        );
    }
    // The mint dropped, and the file adopted.
    {
        let what = "task door · vision · drop the mint";
        let corpus = TrialCorpus::build(State::Fresh);
        let task = corpus.start_workflow("form-vision", "form the vision");
        author_vision(&corpus, &task, &body_of(&task));
        fill_commit(&corpus, &task, "vision");
        plant_hand_written_vision(&corpus, false);
        let before = fs::read(corpus.repo().join(VISION_HOME)).expect("the occupant");

        let findings = task_blocked(&corpus, &task, what);
        let route = clobber_at(&findings, VISION_HOME, what)["route"]
            .as_str()
            .expect("a route")
            .to_owned();
        run_the_span(&corpus, &route, "jigc doc show", what);
        run_the_span(&corpus, &route, "jigc task discard", what);
        assert_eq!(
            fs::read(corpus.repo().join(VISION_HOME)).expect("the occupant survives"),
            before,
            "{what}: the file at the home is untouched",
        );
        run_the_span(&corpus, &route, "git -C", what);
        let minted = run_the_span(&corpus, &route, "jigc migrate", what);
        assert!(
            String::from_utf8_lossy(&minted.stdout).contains("task minted:"),
            "{what}: `jigc migrate` mints the task that adopts the file; {}",
            text(&minted),
        );
    }
}

// ─────────────────────────────────────────────────────────────────────────────────────────
// Two creators of one home in one fan-out (the completion audit's CPL-1)
// ─────────────────────────────────────────────────────────────────────────────────────────
//
// The guard asks what a promotion would land on *before the promote*; two promotions to one
// path are each other's occupant only *during* it. A placement doctype's home does not carry
// the slug, so two sub-tasks that each mint the `vision` (or the `changelog`) are joined as
// `<type>:<type>` and `<type>:<type>-2`, both promote to one file, and one sub-task's doc
// was in no commit and on no disk at exit 0. The axis is the shipped doctype × the commit
// model; the refused cell is driven out through its route, which reads the doc it drops
// before dropping it.

/// A shipped doctype with one fixed home, and how a sub-task mints and authors it.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Singleton {
    Vision,
    Changelog,
}

impl Singleton {
    fn workflow(self) -> &'static str {
        match self {
            Singleton::Vision => "form-vision",
            Singleton::Changelog => "record-change",
        }
    }

    fn home(self) -> &'static str {
        match self {
            Singleton::Vision => VISION_HOME,
            Singleton::Changelog => "CHANGELOG.md",
        }
    }

    fn address(self) -> &'static str {
        match self {
            Singleton::Vision => VISION,
            Singleton::Changelog => "changelog:changelog",
        }
    }

    /// Mint and author the singleton in `sub`, carrying [`body_of`]`(sub)`.
    fn author(self, corpus: &TrialCorpus, sub: &str) {
        match self {
            Singleton::Vision => {
                assert_eq!(author_vision(corpus, sub, &body_of(sub)), VISION);
            }
            Singleton::Changelog => {
                let ack = corpus.jigc_ok(&[
                    "doc",
                    "create",
                    "changelog",
                    "--title",
                    "Changelog",
                    "--task",
                    sub,
                ]);
                assert_eq!(ack.trim(), self.address(), "minted, not copied in");
                let group = corpus.add_item("changelog:changelog#unreleased-changes", "added", sub);
                corpus.set_slot(
                    &format!("{group}/notes"),
                    sub,
                    &format!("- {}", body_of(sub)),
                );
            }
        }
    }
}

/// `jigc milestone add-task <milestone> <intent> [--workflow <w>]`; returns the minted id.
fn add_sub_task(corpus: &TrialCorpus, intent: &str, workflow: Option<&str>) -> String {
    let mut args = vec!["milestone", "add-task", MILESTONE, intent];
    if let Some(workflow) = workflow {
        args.extend(["--workflow", workflow]);
    }
    let ack = corpus.jigc_ok(&args);
    let (_, rest) = ack
        .split_once("added task:")
        .unwrap_or_else(|| panic!("`milestone add-task` names its task; got:\n{ack}"));
    rest.split_whitespace()
        .next()
        .expect("the sub-task id")
        .to_owned()
}

/// **Two sub-tasks, one singleton.** The boundary refuses the pair, keyed at the one home,
/// with nothing committed and both staged docs intact; the route reads the doc it is about
/// to drop, drops that sub-task by consent, and the boundary lands the other.
#[test]
fn two_sub_tasks_minting_one_singleton_never_overwrite_each_other() {
    for (singleton, squash) in [
        (Singleton::Vision, true),
        (Singleton::Vision, false),
        (Singleton::Changelog, true),
    ] {
        let what = format!("{singleton:?} · squash={squash}");
        let corpus = TrialCorpus::build(State::Fresh);
        if !squash {
            squash_false(&corpus);
        }
        corpus.jigc_ok(&["milestone", "create", MILESTONE_TITLE]);
        let alpha = add_sub_task(&corpus, "alpha writes it", Some(singleton.workflow()));
        let bravo = add_sub_task(&corpus, "bravo writes it", Some(singleton.workflow()));
        for sub in [&alpha, &bravo] {
            singleton.author(&corpus, sub);
            assert!(
                staged_bytes(&corpus, sub, singleton.address()).contains(&body_of(sub)),
                "{what}: the before-control — `{sub}` staged its own doc",
            );
        }
        let head = corpus.git(&["rev-parse", "HEAD"]);

        let findings = blocked(&corpus, &what);
        assert_eq!(
            findings.len(),
            1,
            "{what}: one finding for the one contested home; {findings:#?}"
        );
        let finding = clobber_at(&findings, singleton.home(), &what);
        let message = finding["message"].as_str().expect("a message");
        assert!(
            message.contains(alpha.as_str()) && message.contains(bravo.as_str()),
            "{what}: the refusal names both sub-tasks; got: {message}",
        );
        assert_eq!(corpus.git(&["rev-parse", "HEAD"]), head, "{what}");
        assert!(
            !corpus.repo().join(singleton.home()).exists(),
            "{what}: nothing was written at the home",
        );
        for sub in [&alpha, &bravo] {
            assert!(
                staged_bytes(&corpus, sub, singleton.address()).contains(&body_of(sub)),
                "{what}: `{sub}`'s staged doc is intact",
            );
        }

        // ── the route, as printed: read, drop, land ──
        let route = finding["route"].as_str().expect("a route");
        assert_no_rename(route, &what);
        let shown = run_the_span(&corpus, route, "jigc doc show", &what);
        assert!(
            String::from_utf8_lossy(&shown.stdout).contains(&body_of(&bravo)),
            "{what}: the route reads the doc it is about to drop — nothing is lost unseen",
        );
        let discard = run_the_span(&corpus, route, "jigc task discard", &what);
        assert!(text(&discard).contains(bravo.as_str()), "{what}");
        run_the_span(&corpus, route, "jigc milestone finalize", &what);
        let landed = at_head(&corpus, singleton.home());
        assert!(
            landed.contains(&body_of(&alpha)) && !landed.contains(&body_of(&bravo)),
            "{what}: the doc the route said keeps the home is the one that landed; got:\n\
             {landed}",
        );
    }
}

/// **MUST NOT REFUSE — one creator per home.** Two singletons of different doctypes in one
/// fan-out have two homes; the boundary lands both.
#[test]
fn one_creator_per_singleton_home_lands() {
    let corpus = TrialCorpus::build(State::Fresh);
    corpus.jigc_ok(&["milestone", "create", MILESTONE_TITLE]);
    let mut subs = Vec::new();
    for (name, singleton) in [
        ("alpha", Singleton::Vision),
        ("bravo", Singleton::Changelog),
    ] {
        let sub = add_sub_task(
            &corpus,
            &format!("{name} writes it"),
            Some(singleton.workflow()),
        );
        singleton.author(&corpus, &sub);
        subs.push((sub, singleton));
    }
    corpus.jigc_ok(&["milestone", "finalize", MILESTONE]);
    for (sub, singleton) in &subs {
        assert!(
            at_head(&corpus, singleton.home()).contains(&body_of(sub)),
            "{singleton:?}: `{sub}`'s doc landed at its own home",
        );
    }
}

// ─────────────────────────────────────────────────────────────────────────────────────────
// A promote destination a sub-task worktree has staged (the completion audit's CPL-2)
// ─────────────────────────────────────────────────────────────────────────────────────────
//
// The boundary lands two channels in one commit — the code each sub-task staged in its
// worktree, and the merged docs, added over the combined code tree — and nothing compared
// them: a file a sub-agent staged at a doc's home was replaced by the doc at exit 0, in no
// commit, while the manifest counted `1 code file`. The axis is
//
// - **the doc** — minted by another sub-task · minted by the same sub-task · another
//   sub-task's edit of a committed doc (`edited-from-base`, which the file arm lets through);
// - **`finalize.fan-out.squash`** — both commit models;
// - **exit** — the doc renamed, so both land · the staged file taken into git's stash;
//
// with the staged file's bytes accounted for at the end of every cell. Every cell also stages
// a file at a path no doc promotes to, which must land (MUST NOT REFUSE), and the cells walk
// the line-ending settings on the diagonal: the question is which paths a worktree staged,
// asked of git, and reads no byte.

/// The bytes only the worktree's staged file carries.
const WORKTREE_MARKER: &str = "WORKTREE-MARKER a sub-agent's own file, staged in its worktree.";

/// `sub`'s provisioned fan-out worktree.
fn worktree(corpus: &TrialCorpus, sub: &str) -> PathBuf {
    corpus.repo().join(".jigc/worktrees").join(sub)
}

/// Run `git <args>` in `sub`'s worktree, assert success, return trimmed stdout.
fn worktree_git(corpus: &TrialCorpus, sub: &str, args: &[&str]) -> String {
    let out = std::process::Command::new("git")
        .args(args)
        .current_dir(worktree(corpus, sub))
        .env("HOME", corpus.home())
        .output()
        .expect("spawn git");
    assert!(
        out.status.success(),
        "git {args:?} in `{sub}`'s worktree: {}",
        String::from_utf8_lossy(&out.stderr),
    );
    String::from_utf8_lossy(&out.stdout).trim().to_owned()
}

/// Write `bytes` at `rel` in `sub`'s worktree and stage it there.
fn stage_in_worktree(corpus: &TrialCorpus, sub: &str, rel: &str, bytes: &str) {
    let path = worktree(corpus, sub).join(rel);
    fs::create_dir_all(path.parent().expect("a parent dir")).expect("mk the dir");
    fs::write(&path, bytes).expect("write the worktree file");
    worktree_git(corpus, sub, &["add", "--", rel]);
}

/// Who stages the file, and how the doc that promotes onto it is held.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Staged {
    /// Another sub-task's worktree stages a new file at the home of a doc `bravo` minted.
    ByAnother,
    /// The sub-task that minted the doc staged the file in its own worktree.
    ByItself,
    /// Another sub-task's worktree stages a hand edit of a committed doc `bravo` updates.
    EditOfCommitted,
}

/// How the cell is driven out.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Exit {
    Rename,
    Stash,
}

fn drive_staged_cell(staged: Staged, squash: bool, exit: Exit, conversion: Conversion) {
    let what = format!("{staged:?} · squash={squash} · {exit:?} · {conversion:?}");
    let kind = Kind::Adr;
    let destination = format!("{}/{SHARED_SLUG}.md", kind.home());
    let corpus = TrialCorpus::build(State::Fresh);
    conversion.apply(&corpus);
    if !squash {
        squash_false(&corpus);
    }
    if staged == Staged::EditOfCommitted {
        land_standalone(&corpus, kind, SHARED_TITLE, "The first version.");
    }
    let subs = fan_out(&corpus, kind, 2);
    corpus.jigc_ok(&["milestone", "provision", MILESTONE]);
    let (alpha, bravo) = (&subs[0], &subs[1]);
    // The sub-task whose worktree stages the file, and the one whose doc promotes onto it.
    let (stager, author_of_doc) = match staged {
        Staged::ByItself => (bravo, bravo),
        _ => (alpha, bravo),
    };
    let staged_bytes_of_file = match staged {
        Staged::EditOfCommitted => {
            format!("{}\n{WORKTREE_MARKER}\n", at_head(&corpus, &destination))
        }
        _ => format!("{WORKTREE_MARKER}\n"),
    };
    stage_in_worktree(&corpus, stager, &destination, &staged_bytes_of_file);
    // …and a file nobody's doc promotes to: the MUST NOT REFUSE half of every cell.
    stage_in_worktree(
        &corpus,
        stager,
        &format!("{}/a-neighbour.md", kind.home()),
        "a staged file at a path no doc promotes to\n",
    );
    if !squash {
        // The per-sub-task commit model renders the commit doc of every sub-task that
        // carries code, which its re-entry provisions.
        let entered = corpus.jigc_stdin_from(
            &worktree(&corpus, stager),
            &["workflow", "sub-task", "--task", stager],
            "",
        );
        assert!(entered.status.success(), "{what}: {}", text(&entered));
        fill_commit(&corpus, stager, "adr");
    }
    let ack = create(&corpus, kind, SHARED_TITLE, author_of_doc);
    assert_eq!(
        ack.contains("copied in for update"),
        staged == Staged::EditOfCommitted,
        "{what}: the premise — how the doc is held; got: {ack}",
    );
    kind.fill(
        &corpus,
        &format!("adr:{SHARED_SLUG}"),
        author_of_doc,
        &body_of(author_of_doc),
    );
    let head = corpus.git(&["rev-parse", "HEAD"]);

    // ── the boundary refuses ──
    let findings = blocked(&corpus, &what);
    assert_eq!(findings.len(), 1, "{what}: one finding; {findings:#?}");
    let finding = clobber_at(&findings, &destination, &what);
    let message = finding["message"].as_str().expect("a message");
    assert!(
        message.contains(&format!("`{stager}`")) && message.contains("worktree"),
        "{what}: the refusal names the worktree that staged the file; got: {message}",
    );
    assert_eq!(corpus.git(&["rev-parse", "HEAD"]), head, "{what}");
    assert!(
        worktree_git(&corpus, stager, &["diff", "--cached", "--name-only"])
            .lines()
            .any(|path| path == destination),
        "{what}: the staged file is still staged in its worktree",
    );

    // ── the route, as printed ──
    let route = finding["route"].as_str().expect("a route");
    assert_eq!(
        spans(route, "jigc doc rename").is_empty(),
        staged == Staged::EditOfCommitted,
        "{what}: a rename is offered exactly where the doc was minted; got: {route}",
    );
    match exit {
        Exit::Rename => {
            follow_the_routes(&corpus, &findings, &what);
            assert!(
                at_head(&corpus, &destination).contains(WORKTREE_MARKER),
                "{what}: the staged file landed at its path, as the code it is",
            );
        }
        Exit::Stash => {
            run_the_span(&corpus, route, "git -C", &what);
            run_the_span(&corpus, route, "jigc milestone finalize", &what);
            assert!(
                at_head(&corpus, &destination).contains(&body_of(author_of_doc)),
                "{what}: the doc landed at its home",
            );
            assert!(
                corpus
                    .git(&["stash", "show", "-p", "stash@{0}"])
                    .contains(WORKTREE_MARKER),
                "{what}: the staged file's bytes are in the stash the route named",
            );
        }
    }
    assert_landed(&corpus, kind, author_of_doc, &what);
    corpus.git(&[
        "cat-file",
        "-e",
        &format!("HEAD:{}/a-neighbour.md", kind.home()),
    ]);
}

/// **A minted doc over another sub-task's staged file**, under both commit models and out
/// through both exits.
#[test]
fn a_promote_never_replaces_a_file_another_sub_task_staged() {
    drive_staged_cell(Staged::ByAnother, true, Exit::Rename, Conversion::None);
    drive_staged_cell(
        Staged::ByAnother,
        false,
        Exit::Stash,
        Conversion::AutocrlfInput,
    );
}

/// **The two cells the file arm's discriminators would have let through**: the sub-task's
/// own staged file, and another sub-task's staged edit of a committed doc this one updates.
#[test]
fn a_promote_never_replaces_a_staged_file_whoever_holds_the_doc() {
    drive_staged_cell(
        Staged::ByItself,
        true,
        Exit::Rename,
        Conversion::AutocrlfTrue,
    );
    drive_staged_cell(Staged::EditOfCommitted, true, Exit::Stash, Conversion::None);
}
