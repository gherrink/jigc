//! M55 Increment 11 / T3 — **flow 57 (C): overwrite refused, on the shipped report workflow**
//! (`design/worked-examples.md` → 57; `design/findings-channel.md` §4, §11 → C).
//!
//! Every write a task makes here is a line its composed workflow emitted — or the command a
//! refusal's route handed back — run verbatim but for its `<…>` fills (`findings_workflows`'
//! helpers, reused, never copied):
//!
//! 1. **The report arms, one corpus.** A finding lands through the shipped
//!    `report-jigc-feedback`. A second report task then re-files it four ways — the first's
//!    title and a different title slugging onto the first's id, each through the emitted
//!    `doc create` and the emitted `doc author` — and every one is refused
//!    `create.already-exists` at exit 1 keyed at the first finding, never
//!    `write.title-ignored`, with nothing staged and the committed finding byte-unchanged.
//!    The create refusal's route, followed with a `--slug`, lands the second finding beside
//!    the first, and both read back.
//! 2. **The general case** is the shipped `park-idea`, whose entry carries no `new: true`:
//!    a different title slugging onto a committed idea is `write.title-ignored`, routed at a
//!    distinct title or `--slug` and never at renaming the existing doc, and that route,
//!    followed, lands beside it; the same title is the create-or-update it always was.
//! 3. **The report shadows.** A committed project shadow of `report-jigc-feedback` dropping
//!    `new: true` turns the same-title create into the copy-in — so the refusal of arm 1 is
//!    the key's — and one misspelling it `nwe: true` is refused at load, naming the key, by
//!    `jigc start` and by the `doc create` of a task minted before the typo.
//!
//! **What this adds over `create_only_gate` and `findings_workflows`.** The first proves
//! every arm on a project shadow of `park-idea` carrying `new: true`, through hand-built
//! commands; the second's arm (d) drives the same-title `doc create` alone on the shipped
//! report. This flow re-drives the doc-author, different-title, `--slug` and misspelt-key arms
//! on the shipped report workflow (§13's last row). It claims no first proof.
//!
//! **Red** is the shipped `report-jigc-feedback` with `new: true` dropped from its entry:
//! the same-title create acks `copied in for update` at exit 0, and arm 1 fails.

use crate::findings_workflows::{
    Composed, TITLE, argv, author_jigc_feedback, emitted_line, fill_commit, finalize, listed_row,
    run_emitted, shown, slug_of, start, text,
};
use crate::support::trial_corpus::{State, TrialCorpus};

use std::fs;
use std::process::Output;

const ALREADY_EXISTS: &str = "create.already-exists";
const TITLE_IGNORED: &str = "write.title-ignored";
const COPIED_IN: &str = "already existed — copied in for update";

/// The shipped report workflow — every shadow is these bytes with one entry changed.
const SHIPPED_REPORT: &str =
    include_str!("../packs/methodology/workflows/report-jigc-feedback.yaml");
const NEW_ENTRY: &str = "{ type: jigc-feedback, as: feedback, new: true }";
const SHADOW: &str = ".jigc/config/workflows/report-jigc-feedback.yaml";

/// [`TITLE`] retitled so that it slugs onto the same id.
const DIFFERENT: &str = "Finalize sweeps a staged path!";

/// Run the emitted line of `composed` beginning with `prefix`, its placeholders filled,
/// `stdin` fed when given, whatever its exit.
fn run_raw(
    corpus: &TrialCorpus,
    composed: &Composed,
    prefix: &str,
    fills: &[(&str, &str)],
    stdin: Option<&str>,
) -> Output {
    let words = argv(&emitted_line(&composed.text, prefix), fills);
    let args: Vec<&str> = words[1..].iter().map(String::as_str).collect();
    corpus.jigc_stdin_from(&composed.cwd, &args, stdin.unwrap_or(""))
}

/// The `doc author` payload minting `title`, its required slot filled.
fn payload(title: &str) -> String {
    format!(
        "title: {title}\nsections:\n  - id: description\n    set:\n      description: |\n        \
         <<What jigc did.>>\n"
    )
}

/// Run `ty`'s emitted create (`verb == "create"`) or emitted batch author with `title`.
fn mint(corpus: &TrialCorpus, composed: &Composed, ty: &str, verb: &str, title: &str) -> Output {
    match verb {
        "create" => run_raw(
            corpus,
            composed,
            &format!("jigc doc create {ty} "),
            &[("<TITLE>", title)],
            None,
        ),
        _ => run_raw(
            corpus,
            composed,
            &format!("jigc doc author {ty} "),
            &[],
            Some(&payload(title)),
        ),
    }
}

/// The `route:` of the one finding `out` printed.
fn route(out: &Output) -> String {
    String::from_utf8_lossy(&out.stderr)
        .lines()
        .find_map(|line| line.trim().strip_prefix("route: "))
        .unwrap_or_else(|| panic!("a refusal carries a `route:`; {}", text(out)))
        .to_owned()
}

/// Assert `out` is refused `code` at exit 1, keyed at `address`, its route naming a distinct
/// identity for `verb` — `create` a `--title` or a `--slug`, `author` the payload's `title:` —
/// and never renaming the existing doc.
fn assert_refused(out: &Output, code: &str, address: &str, verb: &str, what: &str) {
    assert_eq!(out.status.code(), Some(1), "{what}: exit 1; {}", text(out));
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert!(
        stderr.contains(&format!("· {code} —")) && stderr.contains(&format!("at: {address}\n")),
        "{what}: refused `{code}` at `{address}`; {}",
        text(out),
    );
    let route = route(out);
    let names_identity = match verb {
        "create" => route.contains("--title") && route.contains("--slug"),
        _ => route.contains("title:"),
    };
    assert!(
        names_identity && !route.contains("doc rename"),
        "{what}: routed at a distinct identity, never at renaming the existing doc; got: {route}",
    );
}

/// The backticked `jigc doc create …` command `out`'s route hands back, its `<title>` and
/// `<slug>` filled — the argv a reporter following the route runs.
fn followed_route(out: &Output, title: &str, slug: &str) -> Vec<String> {
    let route = route(out);
    let command = route
        .split('`')
        .skip(1)
        .step_by(2)
        .find(|span| span.starts_with("jigc doc create "))
        .unwrap_or_else(|| panic!("the route hands back a `jigc doc create`; got: {route}"));
    argv(command, &[("<title>", title), ("<slug>", slug)])
}

/// The files of `ty` staged in `task`'s working area.
fn staged(corpus: &TrialCorpus, task: &str, ty: &str) -> Vec<String> {
    let docs = corpus.repo().join(".jigc/tasks").join(task).join("docs");
    fs::read_dir(docs)
        .map(|entries| {
            entries
                .filter_map(Result::ok)
                .map(|entry| entry.file_name().to_string_lossy().into_owned())
                .filter(|name| name.starts_with(&format!("{ty}:")))
                .collect()
        })
        .unwrap_or_default()
}

/// Finalize `composed` through its emitted line; returns the committed path of `address`.
fn land(corpus: &TrialCorpus, composed: &Composed, ty: &str, address: &str) -> String {
    let out = finalize(corpus, composed);
    assert!(
        out.status.success(),
        "`{}` lands; {}",
        composed.task,
        text(&out)
    );
    listed_row(corpus, ty, address)["path"]
        .as_str()
        .expect("a row has a path")
        .to_owned()
}

/// Write `report-jigc-feedback`'s project shadow with its entry replaced by `entry`, and
/// commit it.
fn commit_shadow(corpus: &TrialCorpus, entry: &str) {
    assert!(
        SHIPPED_REPORT.contains(NEW_ENTRY),
        "the premise: the shipped report grants `{NEW_ENTRY}`; got:\n{SHIPPED_REPORT}",
    );
    let path = corpus.repo().join(SHADOW);
    fs::create_dir_all(path.parent().expect("a parent")).expect("mk the workflows dir");
    fs::write(&path, SHIPPED_REPORT.replace(NEW_ENTRY, entry)).expect("write the shadow");
    corpus.git(&["add", "--", SHADOW]);
    corpus.git(&["commit", "-q", "-m", "chore: shadow report-jigc-feedback"]);
}

/// **(1) The report arms.** Same title and different title, by `doc create` and by `doc
/// author`: each refused `create.already-exists` before copy-in. The create route, followed
/// with a `--slug`, lands a second finding beside the first.
#[test]
fn a_second_report_onto_a_filed_id_is_refused_by_both_doors_and_a_slug_lands_beside() {
    let corpus = TrialCorpus::build(State::Fresh);
    let (first, address) = author_jigc_feedback(&corpus, TITLE);
    let path = land(&corpus, &first, "jigc-feedback", &address);
    let committed = fs::read(corpus.repo().join(&path)).expect("read the finding");
    let status = corpus.git(&["status", "--porcelain"]);

    let second = start(&corpus, "report-jigc-feedback", "the same thing again");
    let mut same_title_create = None;
    for title in [TITLE, DIFFERENT] {
        for verb in ["create", "author"] {
            let what = format!("`doc {verb}` of `{title}` onto `{address}`");
            let out = mint(&corpus, &second, "jigc-feedback", verb, title);
            assert_refused(&out, ALREADY_EXISTS, &address, verb, &what);
            assert!(
                !text(&out).contains(TITLE_IGNORED),
                "{what}: the gate refusal comes first, never `{TITLE_IGNORED}`; {}",
                text(&out),
            );
            assert_eq!(
                staged(&corpus, &second.task, "jigc-feedback"),
                Vec::<String>::new(),
                "{what}: nothing staged",
            );
            assert_eq!(
                corpus.git(&["status", "--porcelain"]),
                status,
                "{what}: the checkout is untouched",
            );
            assert_eq!(
                fs::read(corpus.repo().join(&path)).expect("re-read"),
                committed,
                "{what}: the committed finding is byte-unchanged",
            );
            if (title, verb) == (TITLE, "create") {
                same_title_create = Some(out);
            }
        }
    }

    // Follow the same-title create's route, keeping the title and passing a `--slug`.
    let refused = same_title_create.expect("the same-title create ran");
    let beside = format!("{}-again", slug_of(&address));
    let words = followed_route(&refused, TITLE, &beside);
    let args: Vec<&str> = words[1..].iter().map(String::as_str).collect();
    let created = corpus.jigc_ok(&args);
    let again = format!("jigc-feedback:{beside}");
    assert_eq!(
        created.trim(),
        again,
        "the followed route mints the `--slug`"
    );
    for (leaf, hole, value) in [
        ("kind", "<bug|inconvenience|feedback>", "bug"),
        ("found-in", "<kind>:<label>", "milestone:findings-channel"),
        ("jigc-version", "<version>", "1.0.0-rc.23"),
    ] {
        run_emitted(
            &corpus,
            &second,
            &format!("jigc doc set-field jigc-feedback:<slug>#meta/{leaf} "),
            &[("<slug>", &beside), (hole, value)],
            None,
        );
    }
    run_emitted(
        &corpus,
        &second,
        "jigc doc set-slot jigc-feedback:<slug>#description ",
        &[("<slug>", &beside)],
        Some("Filed again, beside the first."),
    );
    fill_commit(&corpus, &second, "feedback");
    land(&corpus, &second, "jigc-feedback", &again);
    for filed in [&address, &again] {
        assert_eq!(
            shown(&corpus, filed)["title"],
            TITLE,
            "`{filed}` reads back with the shared title",
        );
    }
    assert_eq!(
        fs::read(corpus.repo().join(&path)).expect("re-read"),
        committed,
        "the first finding is byte-unchanged after the second landed",
    );
}

/// **(2) The general case.** Under the shipped `park-idea`, whose entry carries no `new:
/// true`, a different title onto a committed idea is `write.title-ignored`, routed at a
/// distinct `--title` or `--slug`; the route, followed, lands beside it. The same title is
/// the create-or-update it always was.
#[test]
fn under_park_idea_a_different_title_onto_a_committed_idea_routes_at_a_distinct_identity() {
    let corpus = TrialCorpus::build(State::Fresh);
    let park = |intent: &str| start(&corpus, "park-idea", intent);
    let fill_and_land = |composed: &Composed, slug: &str| -> String {
        run_emitted(
            &corpus,
            composed,
            "jigc doc set-field idea:<slug>#trigger ",
            &[
                ("<slug>", slug),
                ("<what would resurface it>", "a report comes back"),
            ],
            None,
        );
        run_emitted(
            &corpus,
            composed,
            "jigc doc set-slot idea:<slug>#description ",
            &[("<slug>", slug)],
            Some("One parked thought."),
        );
        fill_commit(&corpus, composed, "ideas");
        land(&corpus, composed, "idea", &format!("idea:{slug}"))
    };

    let first = park("park the first thought");
    let address = run_emitted(
        &corpus,
        &first,
        "jigc doc create idea ",
        &[("<TITLE>", "A Parked Thought")],
        None,
    )
    .trim()
    .to_owned();
    let slug = slug_of(&address);
    let path = fill_and_land(&first, &slug);
    let committed = fs::read(corpus.repo().join(&path)).expect("read the idea");

    let second = park("retitle the thought");
    let different = "A Parked Thought!";
    let out = mint(&corpus, &second, "idea", "create", different);
    let what = format!("`doc create` of `{different}` onto `{address}` under park-idea");
    assert_refused(&out, TITLE_IGNORED, &address, "create", &what);
    assert_eq!(
        staged(&corpus, &second.task, "idea"),
        Vec::<String>::new(),
        "{what}: nothing staged",
    );
    let beside = format!("{slug}-retitled");
    let words = followed_route(&out, different, &beside);
    let args: Vec<&str> = words[1..].iter().map(String::as_str).collect();
    assert_eq!(
        corpus.jigc_ok(&args).trim(),
        format!("idea:{beside}"),
        "{what}: the followed route mints the `--slug`",
    );
    fill_and_land(&second, &beside);
    for filed in [address.clone(), format!("idea:{beside}")] {
        corpus.jigc_ok(&["doc", "show", &filed]);
    }
    assert_eq!(
        fs::read(corpus.repo().join(&path)).expect("re-read"),
        committed,
        "{what}: the existing idea is byte-unchanged",
    );

    let update = park("update the thought");
    let out = mint(&corpus, &update, "idea", "create", "A Parked Thought");
    assert!(
        out.status.success() && String::from_utf8_lossy(&out.stdout).contains(COPIED_IN),
        "the same title under park-idea is the create-or-update: `{COPIED_IN}`, exit 0; {}",
        text(&out),
    );
}

/// **(3) The report shadows.** Dropping `new: true` turns the same-title create into the
/// copy-in at exit 0 — so arm 1's refusal is the key's; misspelling it `nwe: true` is
/// refused at load, naming the key, by `jigc start` and by a minted task's `doc create`.
#[test]
fn a_report_shadow_without_new_copies_in_and_one_misspelling_it_is_refused_at_load() {
    let corpus = TrialCorpus::build(State::Fresh);
    let (first, address) = author_jigc_feedback(&corpus, TITLE);
    land(&corpus, &first, "jigc-feedback", &address);

    commit_shadow(&corpus, "{ type: jigc-feedback, as: feedback }");
    let unguarded = start(&corpus, "report-jigc-feedback", "file it without the key");
    let out = mint(&corpus, &unguarded, "jigc-feedback", "create", TITLE);
    assert!(
        out.status.success()
            && String::from_utf8_lossy(&out.stdout).contains(&format!("{address} ({COPIED_IN})")),
        "without `new: true` the same title copies the filed finding in, exit 0; {}",
        text(&out),
    );

    commit_shadow(&corpus, "{ type: jigc-feedback, as: feedback, nwe: true }");
    let started = corpus.jigc(&[
        "start",
        "--workflow",
        "report-jigc-feedback",
        "file a third",
    ]);
    let created = mint(
        &corpus,
        &unguarded,
        "jigc-feedback",
        "create",
        "A third finding",
    );
    for (out, what) in [
        (&started, "`jigc start --workflow report-jigc-feedback`"),
        (&created, "the minted task's emitted `doc create`"),
    ] {
        assert_eq!(out.status.code(), Some(1), "{what}: exit 1; {}", text(out));
        assert!(
            text(out).contains("workflow-refs.malformed-front-matter")
                && text(out).contains("unknown field `nwe`"),
            "{what}: refused at load, naming the misspelt key `nwe`; {}",
            text(out),
        );
    }
    assert!(
        !String::from_utf8_lossy(&started.stdout).contains("task minted:"),
        "the misspelt shadow mints nothing; {}",
        text(&started),
    );
}
