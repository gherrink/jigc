//! M55 Increment 2 — **the create-only gate** (`design/findings-channel.md` §4, §10;
//! `design/workflow-dialect.md` → On-disk definition format).
//!
//! **T1 — strict `allows-create` entry keys (O3).** An entry key outside the closed set is
//! refused when the workflow loads, naming the key, under the front-matter envelope every
//! load door already refuses with (`workflow-refs.malformed-front-matter`, M55 pin P1).
//! Driven through the real binary on a committed project-layer shadow of `park-idea`
//! whose entry carries the misspelt `nwe: true`:
//!
//! - `jigc start --workflow park-idea` exits 1 naming `nwe` (it minted a task before);
//! - `jigc validate` lists the finding at `workflow:park-idea` naming `nwe`, and keeps its
//!   store-scope exit 0 (M55 pin P2; it reported no findings before);
//! - a task minted from the clean shadow, whose entry is misspelt afterwards, has
//!   `jigc doc create idea` refused at exit 1 naming `nwe` — the `doc` doors load the
//!   workflow too.
//!
//! **The omitting context** is the clean shadow, identical but for the typo: it loads and
//! mints, so the refusal is the key's and nothing else's.
//!
//! **T2 — the `new` key and `create.already-exists`.** On a shadow whose entry carries
//! `new: true`, over one `idea` landed first through the shipped `park-idea`: a create
//! whose minted identity is already on disk at the doctype's home is refused before
//! anything is copied in — by `doc create` and by `doc author` — keyed at the doc's URI,
//! exit 1, nothing staged, no role bound (M55 pins P3–P6). The task's own fresh mint is
//! not *existing*, so a re-run stays idempotent; `--slug` lands beside; the refusal
//! outranks `write.title-ignored` and `write.identity-change`; a committed doc already
//! copied in, and an untracked file at the home, are refused too; fan-out suffixing is
//! unchanged. **The omitting context** is the shipped `park-idea`, whose entry carries no
//! `new`: create-or-update, unchanged.
//!
//! **T3 — the `write.title-ignored` route (F3).** Under the shipped `park-idea`, a different
//! title that slugs onto a *committed* idea is still `write.title-ignored`, but its route
//! names a distinct title or `--slug` (P5's builder) instead of `jigc doc rename` of someone
//! else's doc — followed, it lands a second idea. The staged arm, whose subject is the
//! task's own doc, keeps its in-task rename route, and that route runs.
//!
//! **T4 — the window (the rc.24 review's `(R6, K-1)`).** T2 drives the doors sequentially,
//! so the home is always occupied before the pre-check asks. T4 occupies it *after* that
//! ask — at a state of the door, never a moment on a clock — before each of the two later
//! looks a minting door takes at the home (the pre-check's incumbent probe; the create's
//! own probe), and each answers `create.already-exists`: under `new: true` no create
//! reaches the copy-in. Both doors × every source of the minted id; the route followed;
//! `task finalize` landing beside an occupant whose bytes are still there. Two controls
//! prove the harness acts where it says, the second being the **omitting context**: the
//! same plants under the shipped entry get create-or-update's answers.

use crate::support;

use std::fs;
use std::process::Output;

use support::run_then_parse::{stderr_json, stdout_json};
use support::trial_corpus::{State, TrialCorpus};

/// The shipped `park-idea` definition — the shadow is these bytes with one entry changed,
/// so the only difference the binary can see is the one under test.
const SHIPPED_PARK_IDEA: &str = include_str!("../packs/methodology/workflows/park-idea.yaml");

/// The shipped entry, and the misspelt one the strict key set refuses.
const CLEAN_ENTRY: &str = "{ type: idea, as: idea }";
const MISSPELT_ENTRY: &str = "{ type: idea, as: idea, nwe: true }";
/// The create-only entry (T2).
const NEW_ENTRY: &str = "{ type: idea, as: idea, new: true }";
const ALREADY_EXISTS: &str = "create.already-exists";
/// The idea every T2 arm lands first, through the shipped `park-idea`.
const FIRST_TITLE: &str = "A Parked Thought";
const COPIED_IN: &str = "already existed — copied in for update";

const SHADOW: &str = ".jigc/config/workflows/park-idea.yaml";
const MALFORMED: &str = "workflow-refs.malformed-front-matter";

/// `park-idea` with its entry replaced by `entry`.
fn park_idea_with(entry: &str) -> String {
    assert!(
        SHIPPED_PARK_IDEA.contains(CLEAN_ENTRY),
        "the premise: the shipped park-idea grants `{CLEAN_ENTRY}`; got:\n{SHIPPED_PARK_IDEA}",
    );
    SHIPPED_PARK_IDEA.replace(CLEAN_ENTRY, entry)
}

/// Write the project-layer shadow of `park-idea` carrying `entry`, and commit it.
fn commit_shadow(corpus: &TrialCorpus, entry: &str) {
    let path = corpus.repo().join(SHADOW);
    fs::create_dir_all(path.parent().expect("a parent")).expect("mk the workflows dir");
    fs::write(&path, park_idea_with(entry)).expect("write the shadow");
    corpus.git(&["add", "--", SHADOW]);
    corpus.git(&["commit", "-q", "-m", "chore: shadow park-idea"]);
}

fn text(out: &Output) -> String {
    format!(
        "--- stdout ---\n{}\n--- stderr ---\n{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr),
    )
}

/// Assert `out` is a load refusal: exit 1, the malformed-front-matter code, naming `nwe`.
fn assert_refused_naming_the_key(out: &Output, what: &str) {
    let shown = text(out);
    assert_eq!(
        out.status.code(),
        Some(1),
        "{what} is refused at exit 1; {shown}"
    );
    assert!(
        shown.contains(MALFORMED),
        "{what} is refused with `{MALFORMED}`; {shown}"
    );
    assert!(
        shown.contains("nwe"),
        "{what}'s refusal names the unknown key `nwe`; {shown}"
    );
}

/// **(1)** `jigc start --workflow park-idea` over the misspelt shadow exits 1 naming `nwe`,
/// and mints nothing.
#[test]
fn start_refuses_a_misspelt_entry_key_naming_it() {
    let corpus = TrialCorpus::build(State::Fresh);
    commit_shadow(&corpus, MISSPELT_ENTRY);

    let out = corpus.jigc(&["start", "--workflow", "park-idea", "park one idea"]);
    assert_refused_naming_the_key(&out, "`jigc start --workflow park-idea`");
    assert!(
        !String::from_utf8_lossy(&out.stdout).contains("task minted:"),
        "nothing is minted; {}",
        text(&out),
    );
}

/// **(2)** `jigc validate` lists the finding at `workflow:park-idea` naming `nwe`, and keeps
/// its report-only store-scope exit 0 (P2).
#[test]
fn validate_reports_a_misspelt_entry_key_and_exits_zero() {
    let corpus = TrialCorpus::build(State::Fresh);
    commit_shadow(&corpus, MISSPELT_ENTRY);

    let out = corpus.jigc(&["validate", "--format", "json"]);
    assert_eq!(
        out.status.code(),
        Some(0),
        "store-scope validate stays report-only, exit 0; {}",
        text(&out),
    );
    let report: serde_json::Value = stdout_json(&out, &[0], "`jigc validate --format json`");
    let findings = report["findings"]
        .as_array()
        .unwrap_or_else(|| panic!("the report carries a `findings` array; {}", text(&out)));
    let hits: Vec<&serde_json::Value> = findings
        .iter()
        .filter(|f| f["code"] == MALFORMED && f["key"]["target"] == "workflow:park-idea")
        .collect();
    assert_eq!(
        hits.len(),
        1,
        "one `{MALFORMED}` finding at `workflow:park-idea`; findings:\n{findings:#?}",
    );
    let message = hits[0]["message"].as_str().unwrap_or_default();
    assert!(
        message.contains("nwe"),
        "the finding names the unknown key `nwe`; got: {message}",
    );
}

/// **(3)** A task minted from the clean shadow — the omitting context, which loads and
/// mints — has `jigc doc create idea` refused at exit 1 naming `nwe` once the entry is
/// misspelt: the `doc` doors load the workflow too.
#[test]
fn doc_create_refuses_a_misspelt_entry_key_naming_it() {
    let corpus = TrialCorpus::build(State::Fresh);
    commit_shadow(&corpus, CLEAN_ENTRY);
    let task = corpus.start_workflow("park-idea", "park one idea");

    commit_shadow(&corpus, MISSPELT_ENTRY);
    let out = corpus.jigc(&[
        "doc",
        "create",
        "idea",
        "--title",
        "A Misspelt Gate",
        "--task",
        &task,
    ]);
    assert_refused_naming_the_key(&out, "`jigc doc create idea`");
}

// ---------------------------------------------------------------------------------------
// T2 — the `new` key and `create.already-exists`.
// ---------------------------------------------------------------------------------------

/// The landed first idea: its address as the binary emitted it, and its home.
struct Landed {
    address: String,
    path: String,
}

impl Landed {
    fn slug(&self) -> &str {
        self.address
            .strip_prefix("idea:")
            .unwrap_or_else(|| panic!("an idea address; got {}", self.address))
    }
}

/// Mint a `park-idea` task, create `title` (with `extra` args), fill the idea and the
/// commit doc, and finalize; returns the emitted address and the doc's home.
fn land_idea(corpus: &TrialCorpus, task: &str, title: &str, extra: &[&str]) -> Landed {
    let mut args = vec!["doc", "create", "idea", "--title", title];
    args.extend_from_slice(extra);
    args.extend_from_slice(&["--task", task]);
    let address = corpus.jigc_ok(&args).trim().to_string();
    fill_and_finalize(corpus, task, address)
}

/// Fill the staged idea at `address` and the commit doc, and finalize `task`.
fn fill_and_finalize(corpus: &TrialCorpus, task: &str, address: String) -> Landed {
    corpus.set_field(&format!("{address}#trigger"), task, "a report comes back");
    corpus.set_slot(
        &format!("{address}#description"),
        task,
        "One parked thought.",
    );
    corpus.finalize(task, "ideas", "park an idea", false);
    let slug = address
        .strip_prefix("idea:")
        .unwrap_or_else(|| panic!("an idea address; got {address}"));
    // `idea`'s `location: ideas/` resolves under the `docs-root` knob, `docs/` here.
    let path = format!("docs/ideas/{slug}.md");
    Landed { address, path }
}

/// A fresh corpus holding one idea landed through the **shipped** `park-idea`, then — when
/// `entry` is given — a committed shadow carrying it.
fn arrange(entry: Option<&str>) -> (TrialCorpus, Landed) {
    let corpus = TrialCorpus::build(State::Fresh);
    let first = corpus.start_workflow("park-idea", "park the first thought");
    let landed = land_idea(&corpus, &first, FIRST_TITLE, &[]);
    assert!(
        corpus.repo().join(&landed.path).is_file(),
        "the premise: the first idea landed at {}",
        landed.path,
    );
    if let Some(entry) = entry {
        commit_shadow(&corpus, entry);
    }
    (corpus, landed)
}

/// The payload `doc author` mints `title` from.
fn payload(title: &str) -> String {
    format!(
        "title: {title}\nsections:\n  - id: description\n    set:\n      description: |\n        <<An authored thought.>>\n"
    )
}

/// Run one minting verb on `title` in `task`, with `--format json`.
fn mint(corpus: &TrialCorpus, verb: &str, title: &str, task: &str) -> Output {
    match verb {
        "create" => corpus.jigc(&[
            "doc", "create", "idea", "--title", title, "--task", task, "--format", "json",
        ]),
        "author" => corpus.jigc_stdin(
            &[
                "doc",
                "author",
                "idea",
                "--from-file",
                "-",
                "--task",
                task,
                "--format",
                "json",
            ],
            &payload(title),
        ),
        other => panic!("not a minting verb: {other}"),
    }
}

/// The one finding a refused `--format json` write carries on stderr, at exit 1.
fn refusal(out: &Output, what: &str) -> serde_json::Value {
    let envelope: serde_json::Value = stderr_json(out, &[1], what);
    let findings = envelope["findings"]
        .as_array()
        .unwrap_or_else(|| panic!("{what}: a `findings` array; {}", text(out)));
    assert_eq!(findings.len(), 1, "{what}: one finding; {findings:#?}");
    findings[0].clone()
}

/// Assert `out` is the create-only refusal at `target`, with the verb's own route.
fn assert_already_exists(out: &Output, verb: &str, target: &str, what: &str) {
    let finding = refusal(out, what);
    assert_eq!(
        (
            finding["key"]["code"].as_str(),
            finding["key"]["target"].as_str()
        ),
        (Some(ALREADY_EXISTS), Some(target)),
        "{what}: keyed {{{ALREADY_EXISTS}, {target}}}; {finding:#}",
    );
    assert_eq!(
        finding["severity"], "blocking",
        "{what}: blocking; {finding:#}"
    );
    assert_distinct_identity_route(&finding, verb, what);
}

/// Assert `finding`'s route is P5's distinct-identity route for `verb`: `create` names a
/// distinct `--title` or `--slug`, `author` the payload's `title:` and never a `--slug`;
/// neither routes at renaming the existing doc.
fn assert_distinct_identity_route(finding: &serde_json::Value, verb: &str, what: &str) {
    let route = finding["route"].as_str().unwrap_or_default();
    match verb {
        "create" => assert!(
            route.contains("--title") && route.contains("--slug"),
            "{what}: `create` routes at a distinct `--title` or `--slug`; got: {route}",
        ),
        _ => assert!(
            route.contains("title:") && !route.contains("--slug"),
            "{what}: `author` routes at the payload's `title:` and never at a `--slug` it \
             does not take; got: {route}",
        ),
    }
    assert!(
        !route.contains("doc rename"),
        "{what}: never routed at renaming the existing doc; got: {route}",
    );
}

/// The idea files staged in `task`'s working area.
fn staged_ideas(corpus: &TrialCorpus, task: &str) -> Vec<String> {
    let docs = corpus.repo().join(".jigc/tasks").join(task).join("docs");
    let mut names: Vec<String> = fs::read_dir(&docs)
        .map(|entries| {
            entries
                .filter_map(Result::ok)
                .map(|e| e.file_name().to_string_lossy().into_owned())
                .filter(|name| name.starts_with("idea:"))
                .collect()
        })
        .unwrap_or_default();
    names.sort();
    names
}

/// The address `task`'s `idea` role is bound to, if any.
fn bound_idea(corpus: &TrialCorpus, task: &str) -> Option<String> {
    let path = corpus
        .repo()
        .join(".jigc/tasks")
        .join(task)
        .join("roles.json");
    let bytes = fs::read_to_string(path).ok()?;
    let roles: serde_json::Value = serde_json::from_str(&bytes).expect("roles.json parses");
    roles["roles"]["idea"].as_str().map(str::to_owned)
}

/// **(a)** Re-using the committed idea's title is refused by `doc create` and by `doc
/// author` alike: exit 1, keyed `{create.already-exists, idea:<slug>}`, nothing staged, no
/// role bound, the committed file byte-unchanged.
#[test]
fn a_reused_title_is_refused_by_both_doors_with_nothing_staged() {
    let (corpus, landed) = arrange(Some(NEW_ENTRY));
    let committed = fs::read(corpus.repo().join(&landed.path)).expect("read the committed idea");
    for verb in ["create", "author"] {
        let task = corpus.start_workflow("park-idea", &format!("re-file by {verb}"));
        let out = mint(&corpus, verb, FIRST_TITLE, &task);
        let what = format!("`doc {verb}` of the committed title");
        assert_already_exists(&out, verb, &landed.address, &what);
        assert_eq!(
            staged_ideas(&corpus, &task),
            Vec::<String>::new(),
            "{what}: nothing staged"
        );
        assert_eq!(bound_idea(&corpus, &task), None, "{what}: no role bound");
        assert_eq!(
            fs::read(corpus.repo().join(&landed.path)).expect("re-read"),
            committed,
            "{what}: the committed idea is byte-unchanged",
        );
        corpus.jigc_ok(&["task", "discard", &task, "--force"]);
    }
}

/// **(b)** With `--slug` the second idea lands beside the first, and after finalize both
/// are readable through `jigc doc show`.
#[test]
fn a_slug_lands_the_second_idea_beside_the_first() {
    let (corpus, landed) = arrange(Some(NEW_ENTRY));
    let task = corpus.start_workflow("park-idea", "re-file under a slug");
    let beside = format!("{}-again", landed.slug());
    let second = land_idea(&corpus, &task, FIRST_TITLE, &["--slug", &beside]);
    assert_eq!(second.address, format!("idea:{beside}"));
    for address in [&landed.address, &second.address] {
        corpus.jigc_ok(&["doc", "show", address]);
    }
}

/// **(c)** A different title that slugs onto the committed id gets the same gate refusal,
/// never `write.title-ignored` — by both doors.
#[test]
fn a_different_title_onto_the_same_id_is_the_gate_refusal_not_title_ignored() {
    let (corpus, landed) = arrange(Some(NEW_ENTRY));
    let different = "Parked Thought!";
    for verb in ["create", "author"] {
        let task = corpus.start_workflow("park-idea", &format!("retitle by {verb}"));
        let out = mint(&corpus, verb, different, &task);
        let what = format!(
            "`doc {verb}` of a different title onto `{}`",
            landed.address
        );
        assert_already_exists(&out, verb, &landed.address, &what);
        assert!(
            !text(&out).contains("write.title-ignored"),
            "{what}: never `write.title-ignored`; {}",
            text(&out),
        );
        corpus.jigc_ok(&["task", "discard", &task, "--force"]);
    }
}

/// **(d)** The task's own staged copy is not *existing*: re-running a fresh create in its
/// own task acks `copied in for update` at exit 0, and a re-run `doc author` lands on the
/// same doc at exit 0.
#[test]
fn a_rerun_over_the_tasks_own_fresh_mint_stays_idempotent() {
    let (corpus, _landed) = arrange(Some(NEW_ENTRY));
    let task = corpus.start_workflow("park-idea", "file a fresh thought");
    let first = corpus.jigc_ok(&[
        "doc",
        "create",
        "idea",
        "--title",
        "Fresh Thought",
        "--task",
        &task,
    ]);
    let again = corpus.jigc_ok(&[
        "doc",
        "create",
        "idea",
        "--title",
        "Fresh Thought",
        "--task",
        &task,
    ]);
    assert!(
        !first.contains(COPIED_IN) && again.contains(COPIED_IN),
        "the re-run acks `{COPIED_IN}`; first:\n{first}\nagain:\n{again}",
    );

    let task = corpus.start_workflow("park-idea", "author a fresh thought");
    for run in ["first", "re-run"] {
        let out = mint(&corpus, "author", "Authored Thought", &task);
        let ack: serde_json::Value = stdout_json(&out, &[0], &format!("the {run} `doc author`"));
        assert_eq!(
            ack["target"]["slug"], "authored-thought",
            "the {run} lands on one doc"
        );
    }
    assert_eq!(
        staged_ideas(&corpus, &task),
        vec!["idea:authored-thought.md".to_owned()]
    );
}

/// **(e)** When the bound role and the occupied id both hold, the answer is
/// `create.already-exists`, not `write.identity-change` (M55 pin P4).
#[test]
fn an_occupied_id_outranks_the_bound_role() {
    let (corpus, landed) = arrange(Some(NEW_ENTRY));
    let task = corpus.start_workflow("park-idea", "bind then re-file");
    corpus.jigc_ok(&[
        "doc",
        "create",
        "idea",
        "--title",
        "Fresh Thought",
        "--task",
        &task,
    ]);
    assert_eq!(
        bound_idea(&corpus, &task).as_deref(),
        Some("idea:fresh-thought")
    );

    let out = mint(&corpus, "create", FIRST_TITLE, &task);
    let finding = refusal(&out, "a create minting the occupied id");
    assert_eq!(
        (
            finding["key"]["code"].as_str(),
            finding["key"]["target"].as_str()
        ),
        (Some(ALREADY_EXISTS), Some(landed.address.as_str())),
        "the occupied id answers first; {finding:#}",
    );
    assert!(
        !text(&out).contains("write.identity-change"),
        "the occupied id answers first; {}",
        text(&out),
    );
    assert_eq!(
        bound_idea(&corpus, &task).as_deref(),
        Some("idea:fresh-thought")
    );
}

/// **(e′) The route under a bound role (M55 audit O23).** Once this task holds its doc,
/// every distinct identity is a *second* doc, refused `write.identity-change` — so
/// `create.already-exists` may not route at a distinct `--title` / `--slug` / payload
/// `title:`, which followed would refuse again. It names what is true instead: this task
/// already holds its doc, finish or abandon it, and file the next one in its own task. By
/// both minting doors. **Followed verbatim** — the discard, then the `jigc start` with its
/// `<intent>` filled — the next task's create of the same title meets the gate with *no*
/// role bound, so its route is the distinct identity, and the `--slug` it names lands.
#[test]
fn a_bound_role_routes_an_occupied_id_at_the_next_task_not_a_distinct_identity() {
    let (corpus, landed) = arrange(Some(NEW_ENTRY));
    for verb in ["create", "author"] {
        let task = corpus.start_workflow("park-idea", &format!("bind then re-file by {verb}"));
        corpus.jigc_ok(&[
            "doc",
            "create",
            "idea",
            "--title",
            "Fresh Thought",
            "--task",
            &task,
        ]);
        let what = format!("`doc {verb}` of the occupied id under a bound role");
        let out = mint(&corpus, verb, FIRST_TITLE, &task);
        let finding = refusal(&out, &what);
        assert_eq!(
            finding["key"]["code"].as_str(),
            Some(ALREADY_EXISTS),
            "{what}: {finding:#}",
        );
        let route = finding["route"].as_str().unwrap_or_default();
        assert!(
            route.contains("already holds `idea:fresh-thought`"),
            "{what}: the route names the doc this task already holds; got: {route}",
        );
        for dead in [
            "jigc doc create",
            "jigc doc author",
            "jigc doc rename",
            "<slug>",
        ] {
            assert!(
                !route.contains(dead),
                "{what}: no in-task mint or rename is offered (`{dead}`) — each would \
                 meet `write.identity-change`; got: {route}",
            );
        }
        for exit in [
            format!("jigc task finalize {task}"),
            format!("jigc task discard {task} --force"),
            "jigc start --workflow park-idea \"<intent>\"".to_string(),
        ] {
            assert!(
                route.contains(&format!("`{exit}`")),
                "{what}: the route names `{exit}`; got: {route}"
            );
        }

        // Follow it: abandon this task, then start the next one as emitted.
        let discard = backticked_command(route, "jigc task discard", &what);
        let discarded = run_emitted(&corpus, discard, None);
        assert_eq!(discarded.status.code(), Some(0), "{}", text(&discarded));
        let start = backticked_command(route, "jigc start", &what)
            .replace("<intent>", &format!("file the next thought by {verb}"));
        let started = run_emitted(&corpus, &start, None);
        assert_eq!(started.status.code(), Some(0), "{}", text(&started));
        let next = migrate_task(&String::from_utf8_lossy(&started.stdout));

        // The next task's same-title mint has no role bound, so it routes at a distinct
        // identity — and `create`'s `--slug` lands beside the committed idea.
        let again = mint(&corpus, verb, FIRST_TITLE, &next);
        assert_already_exists(&again, verb, &landed.address, &what);
        let beside = format!("{}-{verb}", landed.slug());
        let landed_beside = corpus.jigc_ok(&[
            "doc",
            "create",
            "idea",
            "--title",
            FIRST_TITLE,
            "--slug",
            &beside,
            "--task",
            &next,
        ]);
        assert_eq!(landed_beside.trim(), format!("idea:{beside}"));
        corpus.jigc_ok(&["task", "discard", &next, "--force"]);
    }
}

/// **(e″) `write.identity-change` under a create-only gate (M55 audit O23).** With the role
/// bound and the minted id *free*, a distinct `--title` — and the committed title under a
/// distinct `--slug` — are a second doc, refused `write.identity-change`; its route, the
/// in-task rename of the doc this task holds, is no dead end under `new: true`: run
/// verbatim, it exits 0 and the held doc carries the asked-for identity.
#[test]
fn under_new_the_identity_change_route_runs() {
    let (corpus, landed) = arrange(Some(NEW_ENTRY));
    let beside = format!("{}-again", landed.slug());
    for (args, moved_to) in [
        (
            vec!["--title", "Something Else"],
            "idea:something-else".to_string(),
        ),
        (
            vec!["--title", FIRST_TITLE, "--slug", beside.as_str()],
            format!("idea:{beside}"),
        ),
    ] {
        let task = corpus.start_workflow("park-idea", "bind then mint another");
        corpus.jigc_ok(&[
            "doc",
            "create",
            "idea",
            "--title",
            "Fresh Thought",
            "--task",
            &task,
        ]);
        let mut argv = vec!["doc", "create", "idea"];
        argv.extend(args.iter().copied());
        argv.extend(["--task", task.as_str(), "--format", "json"]);
        let what = format!("`doc create {}` under a bound role", args.join(" "));
        let finding = refusal(&corpus.jigc(&argv), &what);
        assert_eq!(
            finding["key"]["code"].as_str(),
            Some("write.identity-change"),
            "{what}: {finding:#}",
        );
        let route = finding["route"].as_str().unwrap_or_default();
        let rename = backticked_command(route, "jigc doc rename", &what);
        let followed = run_emitted(&corpus, rename, None);
        assert_eq!(
            followed.status.code(),
            Some(0),
            "{what}: the route runs; {rename}\n{}",
            text(&followed),
        );
        assert_eq!(
            bound_idea(&corpus, &task).as_deref(),
            Some(moved_to.as_str())
        );
        corpus.jigc_ok(&["task", "discard", &task, "--force"]);
    }
}

/// **(f)** A committed doc this task already copied in — by `set-slot` — is still on disk
/// at its home, so the create is refused (M55 pin P3).
#[test]
fn a_committed_doc_copied_in_by_set_slot_is_refused() {
    let (corpus, landed) = arrange(Some(NEW_ENTRY));
    let committed = fs::read(corpus.repo().join(&landed.path)).expect("read the committed idea");
    let task = corpus.start_workflow("park-idea", "touch then re-file");
    corpus.set_slot(
        &format!("{}#description", landed.address),
        &task,
        "An overwrite.",
    );
    assert_eq!(
        staged_ideas(&corpus, &task),
        vec![format!("{}.md", landed.address)]
    );

    let out = mint(&corpus, "create", FIRST_TITLE, &task);
    assert_already_exists(
        &out,
        "create",
        &landed.address,
        "a create over the copied-in doc",
    );
    assert_eq!(
        fs::read(corpus.repo().join(&landed.path)).expect("re-read"),
        committed,
        "the committed idea is byte-unchanged",
    );
}

/// **(g)** An untracked file hand-placed at the doc's home is on disk, so it is refused.
#[test]
fn an_untracked_file_at_the_home_is_refused() {
    let (corpus, landed) = arrange(Some(NEW_ENTRY));
    let body = fs::read_to_string(corpus.repo().join(&landed.path)).expect("read");
    let placed = body.replace(&format!("# {FIRST_TITLE}"), "# Hand Placed");
    assert_ne!(placed, body, "the premise: the H1 was rewritten");
    fs::write(corpus.repo().join("docs/ideas/hand-placed.md"), placed).expect("place");
    let task = corpus.start_workflow("park-idea", "file over a hand-placed file");
    let out = mint(&corpus, "create", "Hand Placed", &task);
    assert_already_exists(
        &out,
        "create",
        "idea:hand-placed",
        "a create over an untracked file",
    );
    assert_eq!(
        staged_ideas(&corpus, &task),
        Vec::<String>::new(),
        "nothing staged"
    );
}

/// **(h) The omitting context.** Under the shipped `park-idea`, whose entry carries no
/// `new`, the same title over the committed idea is a create-or-update: copied in, exit 0.
#[test]
fn without_new_the_create_or_update_is_unchanged() {
    let (corpus, landed) = arrange(None);
    let task = corpus.start_workflow("park-idea", "update the thought");
    let ack = corpus.jigc_ok(&[
        "doc",
        "create",
        "idea",
        "--title",
        FIRST_TITLE,
        "--task",
        &task,
    ]);
    assert!(
        ack.contains(&landed.address) && ack.contains(COPIED_IN),
        "create-or-update acks `{COPIED_IN}`; got:\n{ack}",
    );
}

/// **(i)** Fan-out suffixing is unchanged: two sub-tasks minted from the shadow each create
/// one fresh title at exit 0 — neither sees the other's isolated mint — and `jigc milestone
/// finalize` lands `<slug>` and `<slug>-2`.
#[test]
fn fan_out_sub_tasks_still_join_with_a_suffix() {
    let (corpus, _landed) = arrange(Some(NEW_ENTRY));
    corpus.jigc_ok(&["milestone", "create", "file findings"]);
    let mut subs = Vec::new();
    for n in ["one", "two"] {
        let ack = corpus.jigc_ok(&[
            "milestone",
            "add-task",
            "file-findings",
            &format!("file finding {n}"),
            "--workflow",
            "park-idea",
        ]);
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
    for sub in &subs {
        let address = corpus
            .jigc_ok(&[
                "doc",
                "create",
                "idea",
                "--title",
                "Shared Finding",
                "--task",
                sub,
            ])
            .trim()
            .to_string();
        assert_eq!(address, "idea:shared-finding");
        corpus.set_field(&format!("{address}#trigger"), sub, "a report comes back");
        corpus.set_slot(
            &format!("{address}#description"),
            sub,
            &format!("From {sub}."),
        );
    }
    corpus.jigc_ok(&["milestone", "finalize", "file-findings"]);
    for (slug, sub) in [("shared-finding", &subs[0]), ("shared-finding-2", &subs[1])] {
        let body = fs::read_to_string(corpus.repo().join(format!("docs/ideas/{slug}.md")))
            .unwrap_or_else(|e| panic!("`{slug}` landed: {e}"));
        assert!(
            body.contains(&format!("From {sub}.")),
            "`{slug}` carries `{sub}`'s body, by task id; got:\n{body}",
        );
    }
}

/// **The contract names the new member of the URI form.** `create.already-exists` keys at
/// the doc's `<type>:<slug>`, so `design/command-output-contract.md`'s URI-form row must
/// list it — the `commit_seam_posture` (i) mold: what is fenced is the one statement this
/// commit makes, not a general closure.
#[test]
fn the_contract_lists_already_exists_under_the_uri_form() {
    let doc = std::fs::read_to_string(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../design/command-output-contract.md"
    ))
    .expect("read design/command-output-contract.md");
    let row = doc
        .lines()
        .find(|line| line.starts_with("| a managed doc, or a node inside one"))
        .expect("the URI form's row");
    assert!(
        row.contains(ALREADY_EXISTS),
        "the URI form's Members list must name `{ALREADY_EXISTS}`; row:\n{row}",
    );
}

// ───────────────────── T3 — the `write.title-ignored` route (F3) ─────────────────────

const TITLE_IGNORED: &str = "write.title-ignored";

/// The first backticked span in `route` that starts with `lead` — the command the route
/// hands back, as emitted.
fn backticked_command<'a>(route: &'a str, lead: &str, what: &str) -> &'a str {
    route
        .split('`')
        .skip(1)
        .step_by(2)
        .find(|span| span.starts_with(lead))
        .unwrap_or_else(|| panic!("{what}: the route carries a backticked `{lead}…`; got: {route}"))
}

/// Run an emitted command through a real `sh` split (`support::shell_words`), so its own
/// quoting is adjudicated by the thing that will parse it.
fn run_emitted(corpus: &TrialCorpus, command: &str, stdin: Option<&str>) -> Output {
    let argv = support::shell_words(command, &corpus.repo(), &corpus.home());
    assert_eq!(argv.first().map(String::as_str), Some("jigc"), "{command}");
    let args: Vec<&str> = argv.iter().skip(1).map(String::as_str).collect();
    match stdin {
        Some(input) => corpus.jigc_stdin(&args, input),
        None => corpus.jigc(&args),
    }
}

/// Assert `out` is `write.title-ignored` at `target`, blocking; returns the finding.
fn assert_title_ignored(out: &Output, target: &str, what: &str) -> serde_json::Value {
    let finding = refusal(out, what);
    assert_eq!(
        (
            finding["key"]["code"].as_str(),
            finding["key"]["target"].as_str()
        ),
        (Some(TITLE_IGNORED), Some(target)),
        "{what}: keyed {{{TITLE_IGNORED}, {target}}}; {finding:#}",
    );
    assert_eq!(
        finding["severity"], "blocking",
        "{what}: blocking; {finding:#}"
    );
    finding
}

/// **The committed arm, `doc create` (F3).** Under the shipped `park-idea`, a different
/// title that slugs onto the committed idea is `write.title-ignored`, exit 1, keyed at the
/// idea — and its route names a distinct `--title` or `--slug`, never `jigc doc rename` of
/// someone else's doc. Following it with a `--slug` lands a second idea beside the first.
#[test]
fn without_new_a_different_title_onto_a_committed_id_routes_at_a_distinct_identity() {
    let (corpus, landed) = arrange(None);
    let different = "Parked Thought!";
    let task = corpus.start_workflow("park-idea", "retitle the thought");
    let out = mint(&corpus, "create", different, &task);
    let what = format!(
        "`doc create` of a different title onto `{}`",
        landed.address
    );
    let finding = assert_title_ignored(&out, &landed.address, &what);
    assert_distinct_identity_route(&finding, "create", &what);
    assert_eq!(
        staged_ideas(&corpus, &task),
        Vec::<String>::new(),
        "{what}: nothing staged"
    );

    // Follow the emitted route, filling only its two author-owned placeholders.
    let route = finding["route"].as_str().unwrap_or_default();
    let beside = format!("{}-retitled", landed.slug());
    let command = backticked_command(route, "jigc doc create", &what)
        .replace("<title>", &format!("'{different}'"))
        .replace("<slug>", &beside);
    let followed = run_emitted(&corpus, &command, None);
    assert_eq!(
        followed.status.code(),
        Some(0),
        "the followed route lands; {command}\n{}",
        text(&followed),
    );
    let second = fill_and_finalize(
        &corpus,
        &task,
        String::from_utf8_lossy(&followed.stdout).trim().to_string(),
    );
    assert_eq!(second.address, format!("idea:{beside}"));
    for address in [&landed.address, &second.address] {
        corpus.jigc_ok(&["doc", "show", address]);
    }
}

/// **The committed arm, `doc author` (F3).** The same case through `doc author` routes at
/// the payload's `title:` and never at a `--slug` the verb does not take; re-running the
/// emitted command with a distinct `title:` lands.
#[test]
fn without_new_author_onto_a_committed_id_routes_at_the_payload_title() {
    let (corpus, landed) = arrange(None);
    let task = corpus.start_workflow("park-idea", "re-author the thought");
    let out = mint(&corpus, "author", "Parked Thought!", &task);
    let what = format!(
        "`doc author` of a different title onto `{}`",
        landed.address
    );
    let finding = assert_title_ignored(&out, &landed.address, &what);
    assert_distinct_identity_route(&finding, "author", &what);

    let route = finding["route"].as_str().unwrap_or_default();
    let command = backticked_command(route, "jigc doc author", &what).replace("<payload>", "-");
    let followed = run_emitted(&corpus, &command, Some(&payload("Distinct Thought")));
    assert_eq!(
        followed.status.code(),
        Some(0),
        "the re-run with a distinct `title:` lands; {command}\n{}",
        text(&followed),
    );
    assert_eq!(
        staged_ideas(&corpus, &task),
        vec!["idea:distinct-thought.md".to_owned()]
    );
}

/// **The staged arm keeps its route.** Its subject is the task's own doc, so a different
/// title slugging onto it still routes `jigc doc rename … --task <id>` — and that route,
/// run verbatim, lands the requested title.
#[test]
fn the_staged_arm_still_routes_at_the_in_task_rename() {
    let (corpus, _landed) = arrange(None);
    let task = corpus.start_workflow("park-idea", "file and retitle");
    corpus.jigc_ok(&[
        "doc",
        "create",
        "idea",
        "--title",
        "Fresh Thought",
        "--task",
        &task,
    ]);
    let out = mint(&corpus, "create", "Fresh Thought!", &task);
    let what = "`doc create` of a different title onto the task's own staged idea";
    let finding = assert_title_ignored(&out, "idea:fresh-thought", what);
    let route = finding["route"].as_str().unwrap_or_default();
    let command = backticked_command(route, "jigc doc rename", what);
    assert!(
        command.starts_with("jigc doc rename idea:fresh-thought ")
            && command.ends_with(&format!("--task {task}")),
        "{what}: routed at the in-task rename of the task's own doc; got: {route}",
    );
    let followed = run_emitted(&corpus, command, None);
    assert_eq!(
        followed.status.code(),
        Some(0),
        "the rename route lands; {command}\n{}",
        text(&followed),
    );
    let shown = corpus.jigc_ok(&["doc", "show", "idea:fresh-thought", "--task", &task]);
    assert!(
        shown.contains("# Fresh Thought!"),
        "the route's rename lands the requested title; got:\n{shown}",
    );
}

// ──────── The distinct-identity route follows where the id comes from (M55 inc 2 fix) ────────
//
// P5's route names a distinct *title* — correct only while the id is the title's slug. Three
// other sources exist, and a route at the title under any of them is one that cannot be
// satisfied: followed verbatim, it hands back the same refusal forever. Each test below
// follows the emitted route and proves it lands, or (a fixed identity) that it names no
// create to loop on.

/// The foreign note a migration adopts as an idea.
const FOREIGN_NOTE: &str = "notes/foreign.md";
const FOREIGN_TITLE: &str = "Some Foreign Note";

/// The id a `jigc migrate` printed as `task minted: <id>`.
fn migrate_task(stdout: &str) -> String {
    stdout
        .lines()
        .find_map(|line| line.strip_prefix("task minted: "))
        .unwrap_or_else(|| panic!("`jigc migrate` prints `task minted: <id>`; got:\n{stdout}"))
        .trim()
        .to_string()
}

/// **The migration's recorded `--slug` (`doc author`).** A migration minted with `--slug`
/// onto a committed idea's id authors under that override whatever the payload's `title:`
/// says, so the route must not send the agent back to the payload title — it did, and
/// following it re-raised the same `write.title-ignored` at the same id. The route is
/// discarding the task and re-migrating under a distinct slug; run verbatim, the author
/// then lands at that slug.
#[test]
fn a_migration_slug_onto_a_committed_id_routes_at_a_re_migrate_not_the_payload_title() {
    let (corpus, landed) = arrange(None);
    let foreign = corpus.repo().join(FOREIGN_NOTE);
    fs::create_dir_all(foreign.parent().expect("a parent")).expect("mk notes/");
    fs::write(&foreign, format!("# {FOREIGN_TITLE}\n\nbody\n")).expect("write the note");
    corpus.git(&["add", "--", FOREIGN_NOTE]);
    corpus.git(&["commit", "-q", "-m", "chore: a foreign note"]);
    let task = migrate_task(&corpus.jigc_ok(&[
        "migrate",
        FOREIGN_NOTE,
        "--as",
        "idea",
        "--slug",
        landed.slug(),
    ]));

    let author = [
        "doc",
        "author",
        "idea",
        "--from-file",
        "-",
        "--task",
        &task,
        "--format",
        "json",
    ];
    let out = corpus.jigc_stdin(&author, &payload(FOREIGN_TITLE));
    let what = format!("a migration's `--slug` authoring onto `{}`", landed.address);
    let finding = assert_title_ignored(&out, &landed.address, &what);
    let route = finding["route"].as_str().unwrap_or_default().to_owned();
    assert!(
        !route.contains("set the payload's `title:`") && !route.contains("jigc doc author"),
        "{what}: the id is the override, so no payload title changes it — the route must \
         not send the agent back to the payload; got: {route}",
    );

    // Follow the route verbatim: the discard, then the re-migrate with its one placeholder.
    let discard = backticked_command(&route, "jigc task discard", &what);
    let discarded = run_emitted(&corpus, discard, None);
    assert_eq!(
        discarded.status.code(),
        Some(0),
        "the route's discard runs; {discard}\n{}",
        text(&discarded),
    );
    let distinct = format!("{}-migrated", landed.slug());
    let remigrate = backticked_command(&route, "jigc migrate", &what).replace("<slug>", &distinct);
    let remigrated = run_emitted(&corpus, &remigrate, None);
    assert_eq!(
        remigrated.status.code(),
        Some(0),
        "the route's re-migrate runs; {remigrate}\n{}",
        text(&remigrated),
    );
    let task = migrate_task(&String::from_utf8_lossy(&remigrated.stdout));
    let author = [
        "doc",
        "author",
        "idea",
        "--from-file",
        "-",
        "--task",
        &task,
        "--format",
        "json",
    ];
    let landed_again = corpus.jigc_stdin(&author, &payload(FOREIGN_TITLE));
    let ack: serde_json::Value = stdout_json(&landed_again, &[0], "the re-run `doc author`");
    assert_eq!(
        ack["target"]["slug"], distinct,
        "the author lands at the distinct slug"
    );
}

/// **A `--slug` in force (`doc create`).** The id is the slug, so the route must not offer
/// a distinct `--title` as a correction; its command, followed with a distinct `--slug`
/// and the same title, lands beside the committed idea.
#[test]
fn a_create_slug_onto_an_occupied_id_routes_at_a_distinct_slug() {
    let (corpus, landed) = arrange(Some(NEW_ENTRY));
    let task = corpus.start_workflow("park-idea", "re-file under a taken slug");
    let title = "Totally Distinct Title";
    let out = corpus.jigc(&[
        "doc",
        "create",
        "idea",
        "--title",
        title,
        "--slug",
        landed.slug(),
        "--task",
        &task,
        "--format",
        "json",
    ]);
    let what = format!("`doc create --slug {}`", landed.slug());
    assert_already_exists(&out, "create", &landed.address, &what);
    let finding = refusal(&out, &what);
    let route = finding["route"].as_str().unwrap_or_default();
    assert!(
        !route.contains("distinct `--title`"),
        "{what}: the id is the `--slug`, so a distinct `--title` changes nothing; got: {route}",
    );
    let beside = format!("{}-beside", landed.slug());
    let command = backticked_command(route, "jigc doc create", &what)
        .replace("<title>", &format!("'{title}'"))
        .replace("<slug>", &beside);
    let followed = run_emitted(&corpus, &command, None);
    assert_eq!(
        followed.status.code(),
        Some(0),
        "the followed route lands; {command}\n{}",
        text(&followed),
    );
    assert_eq!(
        String::from_utf8_lossy(&followed.stdout).trim(),
        format!("idea:{beside}")
    );
}

/// **A fixed identity (a singleton under `new: true`).** `vision` mints `vision:vision`
/// whatever the title or `--slug`, so a distinct identity does not exist: the route must say
/// so and name no `--title`/`--slug` create — every such command re-raises this refusal.
#[test]
fn a_singleton_under_new_routes_at_no_distinct_identity() {
    const FORM_VISION: &str = include_str!("../packs/methodology/workflows/form-vision.yaml");
    const VISION_ENTRY: &str = "{ type: vision, as: vision }";
    assert!(
        FORM_VISION.contains(VISION_ENTRY),
        "the premise: form-vision grants `{VISION_ENTRY}`"
    );
    let corpus = TrialCorpus::build(State::CommittedSingletons);
    let shadow = ".jigc/config/workflows/form-vision.yaml";
    let path = corpus.repo().join(shadow);
    fs::create_dir_all(path.parent().expect("a parent")).expect("mk the workflows dir");
    fs::write(
        &path,
        FORM_VISION.replace(VISION_ENTRY, "{ type: vision, as: vision, new: true }"),
    )
    .expect("write the shadow");
    corpus.git(&["add", "--", shadow]);
    corpus.git(&["commit", "-q", "-m", "chore: shadow form-vision"]);

    let task = corpus.start_workflow("form-vision", "revise the vision");
    for (args, what) in [
        (
            vec!["--title", "Vision"],
            "`doc create vision` under `new: true`",
        ),
        (
            vec!["--title", "Vision Two", "--slug", "vision-two"],
            "`doc create vision --slug` under `new: true`",
        ),
    ] {
        let mut argv = vec!["doc", "create", "vision"];
        argv.extend(args);
        argv.extend(["--task", &task, "--format", "json"]);
        let out = corpus.jigc(&argv);
        let finding = refusal(&out, what);
        assert_eq!(
            (
                finding["key"]["code"].as_str(),
                finding["key"]["target"].as_str()
            ),
            (Some(ALREADY_EXISTS), Some("vision:vision")),
            "{what}: keyed at the singleton; {finding:#}",
        );
        let route = finding["route"].as_str().unwrap_or_default();
        assert!(
            route.contains("fixed identity") && !route.contains("jigc doc create"),
            "{what}: the route states that no distinct identity exists and names no create \
             that would refuse again; got: {route}",
        );
    }
}

/// **The staged arm under a `--slug`.** The in-task rename route must carry the slug in
/// force: a bare `--to` re-slugs the task's doc from the title, and the route's own promise —
/// *re-run this write unchanged* — then met `write.identity-change`, because the re-run still
/// mints the slug. Followed verbatim, the rename keeps the id and the re-run lands.
#[test]
fn the_staged_arm_under_a_slug_keeps_the_id_and_the_rerun_lands() {
    let (corpus, _landed) = arrange(None);
    let task = corpus.start_workflow("park-idea", "file under a slug and retitle");
    let create = |title: &str| {
        corpus.jigc(&[
            "doc", "create", "idea", "--title", title, "--slug", "keep-me", "--task", &task,
            "--format", "json",
        ])
    };
    let first = create("First Title");
    assert_eq!(first.status.code(), Some(0), "{}", text(&first));
    let out = create("Second Title");
    let what = "`doc create --slug keep-me` of a different title onto the task's own staged idea";
    let finding = assert_title_ignored(&out, "idea:keep-me", what);
    let route = finding["route"].as_str().unwrap_or_default();
    let command = backticked_command(route, "jigc doc rename", what);
    let followed = run_emitted(&corpus, command, None);
    assert_eq!(
        followed.status.code(),
        Some(0),
        "the rename route runs; {command}\n{}",
        text(&followed),
    );
    let rerun = create("Second Title");
    let ack: serde_json::Value = stdout_json(&rerun, &[0], "the unchanged re-run");
    assert_eq!(
        ack["target"]["slug"], "keep-me",
        "the re-run lands on the same id"
    );
    assert_eq!(
        staged_ideas(&corpus, &task),
        vec!["idea:keep-me.md".to_owned()]
    );
}

// ---------------------------------------------------------------------------------------
// T4 — the window: a home that becomes occupied AFTER the pre-check's create-only ask
// (the rc.24 review's `(R6, K-1)`).
// ---------------------------------------------------------------------------------------

/// The marker a planted occupant carries in a field no minting door writes.
const OCCUPANT: &str = "OCCUPANT-MARKER";

/// The conformant `idea` another hand lands at `docs/ideas/<slug>.md`: the first landed
/// idea's own bytes, retitled, its `trigger` carrying [`OCCUPANT`].
fn occupant_body(landed_body: &str, title: &str) -> String {
    let body = landed_body
        .replace(&format!("# {FIRST_TITLE}"), &format!("# {title}"))
        .replace("a report comes back", OCCUPANT);
    assert!(
        body.contains(&format!("# {title}")) && body.contains(OCCUPANT),
        "the premise: the occupant carries its title and its marker:\n{body}",
    );
    body
}

/// **A state of a minting door** at which the harness acts — never a moment on a clock.
///
/// Both doors run the shared title pre-check and then the create, and between them they
/// look at the minted identity's home three times: the pre-check's create-only ask
/// (rank 0), the pre-check's incumbent probe (rank 3's input), and the create's own probe.
/// Each variant names a file the door reads *between* two of those looks; the harness
/// stops the door at that read ([`at_the_door`]).
///
/// That each read sits where its variant says is **proved by this suite, not assumed**:
/// [`the_window_harness_acts_after_the_pre_checks_ask`] (an occupant *removed* at either
/// state is still refused, so the ask had already seen it) and
/// [`without_new_the_same_plants_get_create_or_updates_answers`] (under a plain entry a
/// plant at the first state is seen by the incumbent probe, and one at the second is not
/// but is still copied in — so each lands before the look it claims to precede).
#[derive(Clone, Copy, Debug, PartialEq)]
enum DoorState {
    /// After the create-only ask, before the incumbent probe: the pre-check reads the
    /// task's `source-path` (*is this a migration onto its own home?*) on its way to that
    /// probe. `doc author` has already read the file once, to learn whether it is a
    /// migration at all, so there it is the second read.
    BetweenThePreChecksLooks,
    /// After every look the pre-check takes, before the create's: the pre-check reads the
    /// task's bound roles last (rank 2's premise), and the create probes the home next.
    BetweenThePreCheckAndTheCreate,
}

const DOOR_STATES: [DoorState; 2] = [
    DoorState::BetweenThePreChecksLooks,
    DoorState::BetweenThePreCheckAndTheCreate,
];

impl DoorState {
    /// The file whose read is the state, which of the door's reads of it, and the bytes
    /// that read is answered with — each the file's own *nothing recorded* form, so the
    /// door goes on exactly as it would have with the file absent.
    fn read(self, verb: &str) -> (&'static str, usize, &'static [u8]) {
        match self {
            DoorState::BetweenThePreChecksLooks => {
                ("source-path", if verb == "author" { 2 } else { 1 }, b"\n")
            }
            DoorState::BetweenThePreCheckAndTheCreate => ("roles.json", 1, b"{\"roles\":{}}\n"),
        }
    }
}

fn mkfifo(path: &std::path::Path) {
    let made = std::process::Command::new("mkfifo")
        .arg(path)
        .status()
        .expect("spawn mkfifo");
    assert!(made.success(), "mkfifo {}", path.display());
}

/// Run `door` (one minting `jigc doc …` call on `task`) and run `act` with the door
/// stopped at `state`.
///
/// **How.** The state's file is absent on a task that has recorded nothing; the harness
/// puts a FIFO there. The door's read blocks in `open` until a writer arrives, and the
/// helper's write-`open` returns exactly then — so `act` runs with the door stopped at
/// that line and the door cannot go on until `act` has finished. The helper then unlinks
/// the FIFO, restoring *absent* for every later read, and answers the blocked read. When
/// the state is the door's *second* read of the file, the first is answered the same way
/// after a fresh FIFO has been renamed over the path — while the door is still blocked in
/// that first read, so its next one stops too. No sleep, no retry, no race.
///
/// Returns the door's output and whether the door ever reached the state.
fn at_the_door(
    corpus: &TrialCorpus,
    task: &str,
    verb: &str,
    state: DoorState,
    act: impl FnOnce() + Send,
    door: impl FnOnce() -> Output,
) -> (Output, bool) {
    use std::io::Write as _;
    use std::sync::atomic::{AtomicBool, Ordering};

    let (file, nth, answer) = state.read(verb);
    let path = corpus.repo().join(".jigc/tasks").join(task).join(file);
    assert!(
        !path.exists(),
        "the premise: a fresh task has no `{file}`; found {}",
        path.display(),
    );
    // One FIFO per read to stop at: the first at the path, the rest parked outside the
    // task (same filesystem — they are renamed into place) until their turn.
    let spares: Vec<std::path::PathBuf> = (2..=nth)
        .map(|k| corpus.home().join(format!("door-{task}-{file}-{k}.fifo")))
        .collect();
    mkfifo(&path);
    for spare in &spares {
        mkfifo(spare);
    }

    let fired = AtomicBool::new(false);
    let released = AtomicBool::new(false);
    let mut act = Some(act);
    let out = std::thread::scope(|scope| {
        let helper = scope.spawn(|| {
            for k in 1..=nth {
                // Returns when a reader has the FIFO open: the door, stopped at its read.
                let mut pipe = fs::OpenOptions::new()
                    .write(true)
                    .open(&path)
                    .expect("open the FIFO for writing");
                if released.load(Ordering::SeqCst) {
                    return;
                }
                if k < nth {
                    fs::rename(&spares[k - 1], &path).expect("arm the door's next read");
                } else {
                    fired.store(true, Ordering::SeqCst);
                    (act.take().expect("the state is reached once"))();
                    fs::remove_file(&path).expect("unlink the FIFO — the file is absent again");
                }
                pipe.write_all(answer).expect("answer the blocked read");
            }
        });
        let out = door();
        if !fired.load(Ordering::SeqCst) {
            // The door exited before the state: rendezvous with the helper's blocked
            // `open` so it returns, and tell it to do nothing.
            released.store(true, Ordering::SeqCst);
            let reader = fs::File::open(&path).expect("release the helper");
            helper.join().expect("the helper exits");
            drop(reader);
            fs::remove_file(&path).expect("unlink the unused FIFO");
        }
        out
    });
    for spare in &spares {
        let _ = fs::remove_file(spare);
    }
    (out, fired.load(Ordering::SeqCst))
}

/// One way a minting door comes to mint the occupant's identity.
struct WindowArm {
    what: &'static str,
    verb: &'static str,
    /// The title the door is handed.
    title: &'static str,
    /// The occupant's `# H1`.
    occupant_title: &'static str,
    /// Whether the id comes from a `--slug` (`doc create` only) rather than the title.
    by_slug: bool,
    /// The identity both resolve to.
    slug_minted: &'static str,
}

/// The axis: both doors × where the minted id comes from (the title, a different title
/// slugging onto the same id, a `--slug`).
const WINDOW_ARMS: &[WindowArm] = &[
    WindowArm {
        what: "`doc create`, the occupant's title",
        verb: "create",
        title: "Window One",
        occupant_title: "Window One",
        by_slug: false,
        slug_minted: "window-one",
    },
    WindowArm {
        what: "`doc create`, a different title onto the occupant's id",
        verb: "create",
        title: "Window  Two!",
        occupant_title: "Window Two",
        by_slug: false,
        slug_minted: "window-two",
    },
    WindowArm {
        what: "`doc create`, a `--slug` naming the occupant",
        verb: "create",
        title: "Something Else Entirely",
        occupant_title: "Window Three",
        by_slug: true,
        slug_minted: "window-three",
    },
    WindowArm {
        what: "`doc author`, the occupant's title",
        verb: "author",
        title: "Window Four",
        occupant_title: "Window Four",
        by_slug: false,
        slug_minted: "window-four",
    },
    WindowArm {
        what: "`doc author`, a different title onto the occupant's id",
        verb: "author",
        title: "Window  Five!",
        occupant_title: "Window Five",
        by_slug: false,
        slug_minted: "window-five",
    },
];

/// Run one minting door on `task`, `--format json` — with `--slug` when the id comes
/// from one.
fn mint_door(
    corpus: &TrialCorpus,
    verb: &str,
    title: &str,
    slug: Option<&str>,
    task: &str,
) -> Output {
    match slug {
        None => mint(corpus, verb, title, task),
        Some(slug) => corpus.jigc(&[
            "doc", "create", "idea", "--title", title, "--slug", slug, "--task", task, "--format",
            "json",
        ]),
    }
}

/// **The contract, by construction: under a `new: true` entry an occupied home is
/// `create.already-exists` at every look a minting door takes at it, and no create
/// reaches the copy-in.** The home is free when the pre-check asks its create-only
/// question and occupied by a later look — the pre-check's incumbent probe, or the
/// create's own. Over both later looks × both doors × every source of the minted id:
/// exit 1, `create.already-exists` keyed at the occupant, the verb's own route, nothing
/// staged, no role bound, the occupant's bytes untouched. Then the door's own next step
/// is driven: the task files its finding under a distinct identity, as the route names,
/// and **`task finalize` lands it beside the occupant, whose bytes are still there** — the
/// loss the window used to end in.
///
/// Before the fix the gate was the pre-check's first look alone. An occupant found by
/// the create was copied in at exit 0 (the engine's create took the gate entry and never
/// read its key), and one found by the incumbent probe was answered `write.title-ignored`
/// — *"would be copied in for update"* — when its title differed, and copied in when it
/// did not. [`without_new_the_same_plants_get_create_or_updates_answers`] is that
/// behaviour, which is a plain entry's to keep.
#[test]
fn a_home_occupied_after_the_create_only_ask_is_still_refused_and_never_copied_in() {
    let (corpus, landed) = arrange(Some(NEW_ENTRY));
    let landed_body = fs::read_to_string(corpus.repo().join(&landed.path)).expect("read");

    for (round, state) in DOOR_STATES.into_iter().enumerate() {
        for arm in WINDOW_ARMS {
            let what = format!("{} ({state:?})", arm.what);
            let what = what.as_str();
            let slug = format!("{}-{round}", arm.slug_minted);
            let title = format!("{} {round}", arm.title);
            let occupant_title = format!("{} {round}", arm.occupant_title);
            let home = corpus.repo().join(format!("docs/ideas/{slug}.md"));
            let address = format!("idea:{slug}");
            let occupant = occupant_body(&landed_body, &occupant_title);
            let task = corpus.start_workflow("park-idea", &format!("file {slug}"));
            assert!(!home.exists(), "{what}: the premise — the home starts free");

            let by_slug = arm.by_slug.then_some(slug.as_str());
            let (out, fired) = at_the_door(
                &corpus,
                &task,
                arm.verb,
                state,
                || fs::write(&home, &occupant).expect("another hand lands the occupant"),
                || mint_door(&corpus, arm.verb, &title, by_slug, &task),
            );
            assert!(fired, "{what}: the door reached the state; {}", text(&out));

            assert_already_exists(&out, arm.verb, &address, what);
            assert_eq!(
                staged_ideas(&corpus, &task),
                Vec::<String>::new(),
                "{what}: nothing staged",
            );
            assert_eq!(bound_idea(&corpus, &task), None, "{what}: no role bound");
            assert_eq!(
                fs::read_to_string(&home).expect("re-read the occupant"),
                occupant,
                "{what}: the occupant's bytes are untouched",
            );

            // The door's next step, as its route names it: a distinct identity.
            let finding = refusal(&out, what);
            let route = finding["route"].as_str().unwrap_or_default();
            let beside = format!("{slug}-beside");
            let followed = match arm.verb {
                "create" => {
                    let command = backticked_command(route, "jigc doc create", what)
                        .replace("<title>", "'A Distinct Thought'")
                        .replace("<slug>", &beside);
                    run_emitted(&corpus, &command, None)
                }
                _ => {
                    let command = backticked_command(route, "jigc doc author", what)
                        .replace("<payload>", "-");
                    run_emitted(&corpus, &command, Some(&payload(&beside.replace('-', " "))))
                }
            };
            assert_eq!(
                followed.status.code(),
                Some(0),
                "{what}: the followed route lands; route: {route}\n{}",
                text(&followed),
            );
            let second = fill_and_finalize(&corpus, &task, format!("idea:{beside}"));
            assert!(
                corpus.repo().join(&second.path).is_file(),
                "{what}: the task's own finding landed at {}",
                second.path,
            );
            assert_eq!(
                fs::read_to_string(&home).expect("re-read the occupant after finalize"),
                occupant,
                "{what}: after `task finalize` the occupant's bytes are still at its home",
            );
        }
    }
}

/// **The harness's first premise: both states are after the pre-check's create-only
/// ask.** The inverse plant — the home is occupied from the start and the occupant is
/// **removed** at the state. Were a state ahead of the ask, the ask would find the home
/// free and the create would mint at exit 0; it is refused, so the ask had already seen
/// the home. Without this arm the test above could go vacuous unnoticed: a reordered door
/// would plant before the ask and be refused by it, proving nothing about the later looks.
#[test]
fn the_window_harness_acts_after_the_pre_checks_ask() {
    let (corpus, landed) = arrange(Some(NEW_ENTRY));
    let landed_body = fs::read_to_string(corpus.repo().join(&landed.path)).expect("read");
    for (round, state) in DOOR_STATES.into_iter().enumerate() {
        for verb in ["create", "author"] {
            let what = format!("`doc {verb}` over an occupant removed at {state:?}");
            let title = format!("Removed {verb} {round}");
            let slug = format!("removed-{verb}-{round}");
            let home = corpus.repo().join(format!("docs/ideas/{slug}.md"));
            fs::write(&home, occupant_body(&landed_body, &title)).expect("occupy the home");
            let task = corpus.start_workflow("park-idea", &format!("file {slug}"));

            let (out, _fired) = at_the_door(
                &corpus,
                &task,
                verb,
                state,
                || fs::remove_file(&home).expect("the occupant is removed"),
                || mint(&corpus, verb, &title, &task),
            );
            assert_already_exists(&out, verb, &format!("idea:{slug}"), &what);
            assert_eq!(
                staged_ideas(&corpus, &task),
                Vec::<String>::new(),
                "{what}: nothing staged",
            );
        }
    }
}

/// **The harness's second premise, and the omitting context: each state is before the
/// look it claims to precede.** Under the shipped `park-idea`, whose entry carries no
/// `new`, the same plants get create-or-update's answers, by both doors:
///
/// - planted **between the pre-check's looks**, a *different*-titled occupant is seen by
///   the incumbent probe — `write.title-ignored`, exit 1, nothing staged — so that state
///   is before that probe;
/// - planted **between the pre-check and the create**, a same-titled occupant is
///   **copied in** at exit 0 — the staged copy carries its marker — so that state is
///   before the create's probe, and what the `new: true` arm refuses is exactly this
///   copy-in.
///
/// Both are a plain entry's declared behaviour over an on-disk doc and are unchanged.
#[test]
fn without_new_the_same_plants_get_create_or_updates_answers() {
    let (corpus, landed) = arrange(None);
    let landed_body = fs::read_to_string(corpus.repo().join(&landed.path)).expect("read");
    for verb in ["create", "author"] {
        // Between the pre-check's looks: the incumbent probe sees the occupant.
        let what = format!("`doc {verb}` under a plain entry, a different-titled occupant");
        let slug = format!("plain-seen-{verb}");
        let home = corpus.repo().join(format!("docs/ideas/{slug}.md"));
        let occupant = occupant_body(&landed_body, &format!("Plain Seen {verb}"));
        let task = corpus.start_workflow("park-idea", &format!("update {slug}"));
        let (out, fired) = at_the_door(
            &corpus,
            &task,
            verb,
            DoorState::BetweenThePreChecksLooks,
            || fs::write(&home, &occupant).expect("another hand lands the occupant"),
            || mint(&corpus, verb, &format!("Plain  Seen {verb}!"), &task),
        );
        assert!(fired, "{what}: the door reached the state; {}", text(&out));
        assert_title_ignored(&out, &format!("idea:{slug}"), &what);
        assert_eq!(
            staged_ideas(&corpus, &task),
            Vec::<String>::new(),
            "{what}: nothing staged",
        );

        // Between the pre-check and the create: the create copies the occupant in.
        let what = format!("`doc {verb}` under a plain entry, an occupant the create finds");
        let slug = format!("plain-copied-{verb}");
        let title = format!("Plain Copied {verb}");
        let home = corpus.repo().join(format!("docs/ideas/{slug}.md"));
        let occupant = occupant_body(&landed_body, &title);
        let task = corpus.start_workflow("park-idea", &format!("update {slug}"));
        let (out, fired) = at_the_door(
            &corpus,
            &task,
            verb,
            DoorState::BetweenThePreCheckAndTheCreate,
            || fs::write(&home, &occupant).expect("another hand lands the occupant"),
            || mint(&corpus, verb, &title, &task),
        );
        assert!(fired, "{what}: the door reached the state; {}", text(&out));
        assert_eq!(
            out.status.code(),
            Some(0),
            "{what}: create-or-update, exit 0; {}",
            text(&out),
        );
        let staged = corpus
            .repo()
            .join(".jigc/tasks")
            .join(&task)
            .join(format!("docs/idea:{slug}.md"));
        let staged = fs::read_to_string(&staged)
            .unwrap_or_else(|err| panic!("{what}: a staged copy; {err}; {}", text(&out)));
        assert!(
            staged.contains(OCCUPANT),
            "{what}: the staged copy is the occupant, copied in:\n{staged}",
        );
        corpus.jigc_ok(&["task", "discard", &task, "--force"]);
    }
}
