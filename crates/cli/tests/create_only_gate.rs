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
    assert_already_exists(
        &out,
        "create",
        &landed.address,
        "a create minting the occupied id",
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
