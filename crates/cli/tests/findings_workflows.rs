//! M55 Increment 8 / T2 — **Flow A's report arm and flow C's first arm, on the shipped
//! report workflows** (`design/findings-channel.md` §2, §4, §11 → A and C).
//!
//! Every write a report task makes here is a line the composed workflow **emitted**, run
//! verbatim but for its `<…>` placeholders — the create, each field and slot, each side's
//! `add-item --slug`, the commit doc's leaves and the finalize — so what is proven is what an
//! agent following the composed text would run, never a command rebuilt in this file.
//!
//! - **(a)** `report-jigc-feedback`, reached by name, lands **exactly its doc** in one commit:
//!   a foreign path staged after the report started stays staged and out of the commit
//!   (`step:finalize-doc-only`, §3). `doc list --format json` and `doc show --format json`
//!   carry the `title` and `fields.status == "open"`, and a hand-deleted, committed `status:`
//!   still reads `"open"` on both (§5).
//! - **(b)** The router catalog lists `report-inconsistency` and none of the three hidden
//!   workflows — `report-jigc-feedback`, `triage-jigc-feedback`, `triage-inconsistency` —
//!   each of which is callable by name.
//! - **(c)** `report-inconsistency` is **reached from the catalog** — the catalog line's id
//!   filled into the router's emitted `jigc start --workflow <chosen>` line — and lands its
//!   doc alone with three sides, each added by `add-item --slug`.
//! - **(d)** Flow C's first arm: a second report with the first's title is refused
//!   `create.already-exists` at exit 1 by the create gate's `new: true` entry, nothing staged
//!   and the committed finding byte-unchanged (§4).
//! - **(e)** `triage-jigc-feedback`, on a finding filed through (a)'s arm and named in plain
//!   words, sets `resolved` + `pinned-by` + `resolution` — the first write acking its copy-in
//!   — and lands that doc alone, every filed line kept but `status:`. Its composed text names
//!   the four leaves, says nothing else changes, and names the intent form (§1.5, §2 → R5).
//! - **(f)** `triage-inconsistency` does the same for a filed record: `intended` + a
//!   `resolution`, its three sides kept, the two leaves and the intent form stated.
//! - **(g)** A triage task allows no create: a `doc create` in one is `create.gate-blocked`.
//! - **(h)** Flow D (§11 → D, §6 S2): a milestone with three report sub-tasks. Each `Spawn:`
//!   line is run verbatim through `sub_task_composition`'s `jigc` shim; each composed text
//!   names no `jigc task finalize`, asks for no `commit:<sub>` write, carries no line of
//!   `step:finalize-doc-only` or `step:author-commit`, and ends in the trailer naming
//!   `jigc milestone finalize <m>`. Each sub-task files its doc in its own worktree, and the
//!   join lands one commit holding exactly the three docs and the milestone record.
//! - **(i)** (h)'s discriminating control: the same checks over `report-jigc-feedback`
//!   composed as an ordinary task fail, every one of them.
//!
//! **Red** is the mutant the doc-only step exists against: with `report-jigc-feedback`'s body
//! on `step:finalize` instead of `step:finalize-doc-only`, the report's commit sweeps the
//! foreign staged path in and (a)'s one-doc assertion fails. For (e), the triage step
//! shadowed without its four-leaves sentence fails the composed-text assertion.

use crate::support;

use crate::sub_task_composition::commit_writes;
#[cfg(unix)]
use crate::sub_task_composition::{install_jigc_shim, run_span, spawn_spans};

use std::collections::BTreeSet;
use std::fs;
use std::path::PathBuf;
use std::process::Output;

use serde_json::{Value, json};
use support::run_then_parse::stdout_json;
use support::trial_corpus::{State, TrialCorpus};

/// The three workflows the router catalog leaves out, each reached by name (§2).
pub(crate) const HIDDEN: [&str; 3] = [
    "report-jigc-feedback",
    "triage-jigc-feedback",
    "triage-inconsistency",
];

/// The router-visible report workflow (§2).
pub(crate) const VISIBLE: &str = "report-inconsistency";

/// The jigc-feedback finding every arm files first.
pub(crate) const TITLE: &str = "Finalize sweeps a staged path";

/// A path another task staged in the same checkout — never the report's to commit.
pub(crate) const FOREIGN: &str = "src/foreign.rs";

/// A fenced repro whose `#`-led line would be refused unfenced (the author step says so).
const REPRO: &str =
    "```sh\n# stage a file, then finalize the report\ngit add src/foreign.rs\n```\n";

/// The inconsistency every arm files.
const INCONSISTENCY: &str = "Cache eviction disagrees";

/// The three sides of the inconsistency: `(title, --slug, says)`.
pub(crate) const SIDES: [(&str, &str, &str); 3] = [
    ("src/cache.rs", "code", "The cache evicts the oldest entry."),
    (
        "adr:cache-strategy#decision",
        "adr",
        "The cache evicts the least-recently-used entry.",
    ),
    ("docs/cache.md", "guide", "The cache never evicts."),
];

pub(crate) fn text(out: &Output) -> String {
    format!(
        "--- stdout ---\n{}\n--- stderr ---\n{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr),
    )
}

/// A composed work-workflow: the task id the binary printed it minted, the composed text,
/// and the checkout its emitted lines run in — the repository for an ordinary task, the
/// sub-task's own worktree for a fan-out sub-task.
pub(crate) struct Composed {
    pub(crate) task: String,
    pub(crate) text: String,
    pub(crate) cwd: PathBuf,
}

/// The id `stdout` says was minted — read off the emitted `task minted:` line.
pub(crate) fn minted(stdout: &str) -> String {
    stdout
        .lines()
        .find_map(|line| line.strip_prefix("task minted: "))
        .unwrap_or_else(|| panic!("a mint prints `task minted: <id>`; got:\n{stdout}"))
        .trim()
        .to_owned()
}

/// `jigc start --workflow <workflow> <intent>` — the by-name door.
pub(crate) fn start(corpus: &TrialCorpus, workflow: &str, intent: &str) -> Composed {
    let text = corpus.jigc_ok(&["start", "--workflow", workflow, intent]);
    Composed {
        task: minted(&text),
        text,
        cwd: corpus.repo(),
    }
}

/// The one emitted command line of `text` that begins with `prefix`, its `Run: \`…\``
/// decoration stripped. A heredoc-opening example line (`… <<'EOF'`) is the illustrated form
/// of a line the step also emits bare, so it is skipped; the bare line is the one run.
pub(crate) fn emitted_line(text: &str, prefix: &str) -> String {
    let lines: Vec<&str> = text
        .lines()
        .map(str::trim)
        .map(|line| {
            line.strip_prefix("Run: `")
                .and_then(|rest| rest.strip_suffix('`'))
                .unwrap_or(line)
        })
        .filter(|line| line.starts_with(prefix) && !line.ends_with("<<'EOF'"))
        .collect();
    assert_eq!(
        lines.len(),
        1,
        "the composed text emits exactly one `{prefix}…` line; got {lines:?} in:\n{text}",
    );
    lines[0].to_owned()
}

/// Split an emitted line into argv the way a shell would for these lines — whitespace
/// separates, a double-quoted run is one word — then fill every `<…>` placeholder from
/// `fills`. An unfilled placeholder panics: a line is run only once nothing is left to fill.
pub(crate) fn argv(line: &str, fills: &[(&str, &str)]) -> Vec<String> {
    let mut words = Vec::new();
    let mut word = String::new();
    let mut quoted = false;
    let mut started = false;
    for ch in line.chars() {
        match ch {
            '"' => {
                quoted = !quoted;
                started = true;
            }
            c if c.is_whitespace() && !quoted => {
                if started {
                    words.push(std::mem::take(&mut word));
                    started = false;
                }
            }
            c => {
                word.push(c);
                started = true;
            }
        }
    }
    assert!(!quoted, "an emitted line closes its quotes: `{line}`");
    if started {
        words.push(word);
    }
    let words: Vec<String> = words
        .into_iter()
        .map(|word| {
            fills
                .iter()
                .fold(word, |word, (hole, value)| word.replace(hole, value))
        })
        .collect();
    assert!(
        !words.iter().any(|word| word.contains('<')),
        "every placeholder of `{line}` is filled before it runs; got {words:?}",
    );
    assert_eq!(
        words.first().map(String::as_str),
        Some("jigc"),
        "an emitted line is a `jigc` invocation: `{line}`",
    );
    words
}

/// Run the emitted line beginning with `prefix`, its placeholders filled, `stdin` fed when
/// given; asserts it succeeded and returns its stdout.
pub(crate) fn run_emitted(
    corpus: &TrialCorpus,
    composed: &Composed,
    prefix: &str,
    fills: &[(&str, &str)],
    stdin: Option<&str>,
) -> String {
    let out = run_emitted_raw(corpus, composed, prefix, fills, stdin);
    assert!(
        out.status.success(),
        "the emitted `{prefix}…` line runs at exit 0; {}",
        text(&out),
    );
    String::from_utf8_lossy(&out.stdout).into_owned()
}

fn run_emitted_raw(
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

/// Run the emitted create with `title`, returning the address the binary acked.
fn create(corpus: &TrialCorpus, composed: &Composed, ty: &str, title: &str) -> String {
    let address = run_emitted(
        corpus,
        composed,
        &format!("jigc doc create {ty} "),
        &[("<TITLE>", title)],
        None,
    )
    .trim()
    .to_owned();
    assert!(
        address.starts_with(&format!("{ty}:")),
        "the create acks a `{ty}:` address; got `{address}`",
    );
    address
}

/// The `<slug>` half of a `type:slug` address.
pub(crate) fn slug_of(address: &str) -> String {
    address
        .split_once(':')
        .unwrap_or_else(|| panic!("a `type:slug` address; got `{address}`"))
        .1
        .to_owned()
}

/// Fill the task's commit doc through the commit step's emitted lines.
pub(crate) fn fill_commit(corpus: &TrialCorpus, composed: &Composed, scope: &str) {
    let commit = format!("jigc doc set-field commit:{}", composed.task);
    run_emitted(
        corpus,
        composed,
        &format!("{commit}#type "),
        &[("<TYPE>", "docs")],
        None,
    );
    run_emitted(
        corpus,
        composed,
        &format!("{commit}#scope "),
        &[("<SCOPE>", scope)],
        None,
    );
    let slot = format!("jigc doc set-slot commit:{}", composed.task);
    run_emitted(
        corpus,
        composed,
        &format!("{slot}#summary "),
        &[],
        Some("file one finding"),
    );
    run_emitted(
        corpus,
        composed,
        &format!("{slot}#body "),
        &[],
        Some("One finding, filed through its report workflow."),
    );
}

/// Author a jigc-feedback finding titled `title` in a fresh `report-jigc-feedback` task,
/// every write through the composed text's emitted lines; the commit doc is filled too, so
/// only the finalize is left. Returns the composed task and the finding's address.
pub(crate) fn author_jigc_feedback(corpus: &TrialCorpus, title: &str) -> (Composed, String) {
    let composed = start(
        corpus,
        "report-jigc-feedback",
        "the finalize sweeps a staged path",
    );
    let address = file_jigc_feedback(corpus, &composed, title);
    fill_commit(corpus, &composed, "feedback");
    (composed, address)
}

/// Create and fill a jigc-feedback finding titled `title` in the composed report task,
/// every write — and the read-back — through its emitted lines. Returns its address.
pub(crate) fn file_jigc_feedback(corpus: &TrialCorpus, composed: &Composed, title: &str) -> String {
    let address = create(corpus, composed, "jigc-feedback", title);
    let slug = slug_of(&address);
    let fills = [("<slug>", slug.as_str())];
    for (leaf, hole, value) in [
        ("kind", "<bug|inconvenience|feedback>", "bug"),
        ("found-in", "<kind>:<label>", "milestone:findings-channel"),
        ("jigc-version", "<version>", "1.0.0-rc.23"),
        ("about", "<surface>", "jigc task finalize"),
    ] {
        run_emitted(
            corpus,
            composed,
            &format!("jigc doc set-field jigc-feedback:<slug>#meta/{leaf} "),
            &[fills[0], (hole, value)],
            None,
        );
    }
    run_emitted(
        corpus,
        composed,
        "jigc doc set-slot jigc-feedback:<slug>#description ",
        &fills,
        Some("The report's finalize committed a path another task had staged."),
    );
    run_emitted(
        corpus,
        composed,
        "jigc doc set-slot jigc-feedback:<slug>#repro ",
        &fills,
        Some(REPRO),
    );
    let read_back = run_emitted(
        corpus,
        composed,
        "jigc doc show jigc-feedback:<slug> ",
        &fills,
        None,
    );
    assert!(
        read_back.contains(REPRO.trim_end()),
        "the emitted read-back serves the staged, fenced repro; got:\n{read_back}",
    );
    address
}

/// Author the inconsistency titled [`INCONSISTENCY`] with its three [`SIDES`] in the
/// composed `report-inconsistency` task, every write through the composed text's emitted
/// lines; the commit doc is filled too, so only the finalize is left. Returns its address.
pub(crate) fn author_inconsistency(corpus: &TrialCorpus, composed: &Composed) -> String {
    let address = file_inconsistency(corpus, composed);
    fill_commit(corpus, composed, "inconsistencies");
    address
}

/// Create and fill the inconsistency titled [`INCONSISTENCY`] with its three [`SIDES`] in
/// the composed report task, every write through its emitted lines. Returns its address.
pub(crate) fn file_inconsistency(corpus: &TrialCorpus, composed: &Composed) -> String {
    let address = create(corpus, composed, "inconsistency", INCONSISTENCY);
    let slug = slug_of(&address);
    run_emitted(
        corpus,
        composed,
        "jigc doc set-field inconsistency:<slug>#meta/kind ",
        &[("<slug>", &slug), ("<code-doc|doc-doc>", "code-doc")],
        None,
    );
    for (side_title, side, says) in SIDES {
        run_emitted(
            corpus,
            composed,
            "jigc doc add-item inconsistency:<slug>#sides ",
            &[
                ("<slug>", &slug),
                ("<path or address>", side_title),
                ("<side>", side),
            ],
            None,
        );
        run_emitted(
            corpus,
            composed,
            "jigc doc set-slot inconsistency:<slug>#sides/<side>/says ",
            &[("<slug>", &slug), ("<side>", side)],
            Some(says),
        );
    }
    run_emitted(
        corpus,
        composed,
        "jigc doc set-slot inconsistency:<slug>#description ",
        &[("<slug>", &slug)],
        Some("The code, the decision and the guide each state a different eviction rule."),
    );
    run_emitted(
        corpus,
        composed,
        "jigc doc set-slot inconsistency:<slug>#evidence ",
        &[("<slug>", &slug)],
        Some("Read all three side by side."),
    );
    address
}

/// Run the composed text's emitted finalize line.
pub(crate) fn finalize(corpus: &TrialCorpus, composed: &Composed) -> Output {
    run_emitted_raw(
        corpus,
        composed,
        &format!("jigc task finalize {}", composed.task),
        &[],
        None,
    )
}

/// The paths HEAD's commit changed.
pub(crate) fn head_files(corpus: &TrialCorpus) -> Vec<String> {
    corpus
        .git(&["show", "--name-only", "--pretty=format:", "HEAD"])
        .lines()
        .filter(|line| !line.is_empty())
        .map(str::to_owned)
        .collect()
}

pub(crate) fn commit_count(corpus: &TrialCorpus) -> usize {
    corpus
        .git(&["rev-list", "--count", "HEAD"])
        .parse()
        .expect("a count")
}

/// The committed `doc list <ty> --format json` row for `address`.
pub(crate) fn listed_row(corpus: &TrialCorpus, ty: &str, address: &str) -> Value {
    let listing: Value = stdout_json(
        &corpus.jigc(&["doc", "list", ty, "--format", "json"]),
        &[0],
        &format!("`doc list {ty}`"),
    );
    listing["docs"]
        .as_array()
        .expect("`docs` is an array")
        .iter()
        .find(|row| row["id"] == json!(address))
        .cloned()
        .unwrap_or_else(|| panic!("a committed row for `{address}`; listing:\n{listing:#}"))
}

/// The committed `doc show <address> --format json` serve.
pub(crate) fn shown(corpus: &TrialCorpus, address: &str) -> Value {
    stdout_json(
        &corpus.jigc(&["doc", "show", address, "--format", "json"]),
        &[0],
        &format!("`doc show {address}`"),
    )
}

/// Assert both read surfaces carry `title` and project `fields.status == "open"`.
pub(crate) fn assert_reads_open(
    corpus: &TrialCorpus,
    ty: &str,
    address: &str,
    title: &str,
    when: &str,
) {
    let row = listed_row(corpus, ty, address);
    let show = shown(corpus, address);
    for (surface, value) in [("`doc list` row", &row), ("`doc show`", &show)] {
        assert_eq!(
            (&value["title"], &value["fields"]["status"]),
            (&json!(title), &json!("open")),
            "{when}: the {surface} of `{address}` carries its title and status `open`; \
             got:\n{value:#}",
        );
    }
}

/// The router catalog `jigc start "<intent>"` composes — every `- <id> — <when>` line's id.
pub(crate) fn catalog(corpus: &TrialCorpus) -> (String, Vec<String>) {
    let text = corpus.jigc_ok(&["start", "a disagreement turned up mid-work"]);
    let ids: Vec<String> = text
        .lines()
        .filter_map(|line| line.strip_prefix("- "))
        .filter_map(|line| line.split_once(" — "))
        .map(|(id, _)| id.to_owned())
        .collect();
    assert!(
        ids.iter().any(|id| id == "park-idea"),
        "the premise: the router composed its catalog; got:\n{text}",
    );
    (text, ids)
}

/// **(a)** `report-jigc-feedback`, by name, lands exactly its doc — a foreign path staged
/// after the report started stays staged — and both read surfaces carry the title and
/// status `open`, still `open` once the committed `status:` line is hand-deleted.
#[test]
fn report_jigc_feedback_lands_its_doc_alone_and_reads_back_open() {
    let corpus = TrialCorpus::build(State::Fresh);
    let (composed, address) = author_jigc_feedback(&corpus, TITLE);

    fs::create_dir_all(corpus.repo().join("src")).expect("mk src/");
    fs::write(corpus.repo().join(FOREIGN), "pub fn other_task() {}\n").expect("foreign file");
    corpus.git(&["add", "--", FOREIGN]);

    let before = commit_count(&corpus);
    let out = finalize(&corpus, &composed);
    assert!(
        out.status.success(),
        "the report's emitted finalize lands at exit 0; {}",
        text(&out),
    );
    assert_eq!(commit_count(&corpus), before + 1, "exactly one commit");
    let path = listed_row(&corpus, "jigc-feedback", &address)["path"]
        .as_str()
        .expect("a row has a path")
        .to_owned();
    assert_eq!(
        head_files(&corpus),
        vec![path.clone()],
        "the report's commit holds its finding alone; {}",
        text(&out),
    );
    assert_eq!(
        corpus.git(&["diff", "--cached", "--name-only"]),
        FOREIGN,
        "the foreign path stays staged for the task it belongs to",
    );
    assert_reads_open(&corpus, "jigc-feedback", &address, TITLE, "as filed");

    // Hand-delete the committed `status:` line and commit just that file.
    let committed = fs::read_to_string(corpus.repo().join(&path)).expect("read the finding");
    let statusless: String = committed
        .split_inclusive('\n')
        .filter(|line| !line.starts_with("status:"))
        .collect();
    assert_ne!(statusless, committed, "`{path}` carried a `status:` line");
    fs::write(corpus.repo().join(&path), &statusless).expect("hand edit");
    corpus.git(&["add", "--", &path]);
    corpus.git(&[
        "commit",
        "-q",
        "-m",
        "drop the status line by hand",
        "--",
        &path,
    ]);
    assert_reads_open(
        &corpus,
        "jigc-feedback",
        &address,
        TITLE,
        "status hand-deleted",
    );
    assert_eq!(
        fs::read_to_string(corpus.repo().join(&path)).expect("re-read"),
        statusless,
        "the reads project the default and write nothing",
    );
}

/// **(b)** The catalog lists `report-inconsistency` and none of the three hidden workflows,
/// and each hidden one is callable by name.
#[test]
fn the_catalog_lists_report_inconsistency_and_none_of_the_hidden_three() {
    let corpus = TrialCorpus::build(State::Fresh);
    let (text, ids) = catalog(&corpus);
    assert!(
        ids.iter().any(|id| id == VISIBLE),
        "the catalog lists `{VISIBLE}`; got:\n{text}",
    );
    for hidden in HIDDEN {
        assert!(
            !ids.iter().any(|id| id == hidden),
            "the catalog leaves `{hidden}` out; got:\n{text}",
        );
        let composed = start(&corpus, hidden, &format!("reach {hidden} by name"));
        assert!(
            composed
                .text
                .contains(&format!("jigc task finalize {}", composed.task)),
            "`{hidden}` composes by name to a finalizing task; got:\n{}",
            composed.text,
        );
    }
}

/// **(c)** `report-inconsistency` is reached from the catalog and lands its doc alone, three
/// sides added by `add-item --slug` and each addressable by that slug.
#[test]
fn report_inconsistency_is_reached_from_the_catalog_with_three_sides() {
    let corpus = TrialCorpus::build(State::Fresh);
    let (router, ids) = catalog(&corpus);
    let chosen = ids
        .iter()
        .find(|id| *id == VISIBLE)
        .unwrap_or_else(|| panic!("the catalog lists `{VISIBLE}`; got:\n{router}"));
    let intent = "the cache docs disagree on eviction";
    let start_line = argv(
        &emitted_line(&router, "jigc start --workflow <chosen>"),
        &[("<chosen>", chosen), ("<intent>", intent)],
    );
    let args: Vec<&str> = start_line[1..].iter().map(String::as_str).collect();
    let reached = corpus.jigc_ok(&args);
    let composed = Composed {
        task: minted(&reached),
        text: reached,
        cwd: corpus.repo(),
    };
    assert!(
        composed.text.contains("jigc doc create inconsistency "),
        "the router's re-run line composed `{VISIBLE}`; got:\n{}",
        composed.text,
    );

    let address = author_inconsistency(&corpus, &composed);

    let before = commit_count(&corpus);
    let out = finalize(&corpus, &composed);
    assert!(
        out.status.success(),
        "the emitted finalize lands at exit 0; {}",
        text(&out),
    );
    assert_eq!(commit_count(&corpus), before + 1, "exactly one commit");
    let row = listed_row(&corpus, "inconsistency", &address);
    let path = row["path"].as_str().expect("a row has a path").to_owned();
    assert_eq!(
        head_files(&corpus),
        vec![path],
        "the commit holds the inconsistency alone",
    );
    assert_eq!(
        (&row["item-count"], &row["fields"]["status"]),
        (&json!(3), &json!("open")),
        "three sides, filed open; row:\n{row:#}",
    );
    for (side_title, side, says) in SIDES {
        let served: Value = stdout_json(
            &corpus.jigc(&[
                "doc",
                "show",
                &format!("{address}#sides/{side}/says"),
                "--format",
                "json",
            ]),
            &[0],
            &format!("the `{side}` side's `says`"),
        );
        assert!(
            served.to_string().contains(says),
            "the side `{side}` (`{side_title}`) is addressable by its `--slug` and says \
             `{says}`; got:\n{served:#}",
        );
    }
}

/// **(d)** Flow C's first arm: a second `report-jigc-feedback` with the first's title is
/// refused `create.already-exists` at exit 1 — nothing staged, the finding unchanged.
#[test]
fn a_same_title_second_report_is_refused_with_nothing_staged() {
    let corpus = TrialCorpus::build(State::Fresh);
    let (first, address) = author_jigc_feedback(&corpus, TITLE);
    let out = finalize(&corpus, &first);
    assert!(
        out.status.success(),
        "the first report lands; {}",
        text(&out)
    );
    let path = listed_row(&corpus, "jigc-feedback", &address)["path"]
        .as_str()
        .expect("a row has a path")
        .to_owned();
    let committed = fs::read(corpus.repo().join(&path)).expect("read the finding");
    let status_before = corpus.git(&["status", "--porcelain"]);

    let second = start(&corpus, "report-jigc-feedback", "the same thing again");
    let out = run_emitted_raw(
        &corpus,
        &second,
        "jigc doc create jigc-feedback ",
        &[("<TITLE>", TITLE)],
        None,
    );
    assert_eq!(
        out.status.code(),
        Some(1),
        "the same-title create is refused at exit 1; {}",
        text(&out),
    );
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert!(
        stderr.contains("create.already-exists") && stderr.contains(&format!("at: {address}")),
        "refused `create.already-exists` at `{address}`; {}",
        text(&out),
    );
    let docs = corpus
        .repo()
        .join(".jigc/tasks")
        .join(&second.task)
        .join("docs");
    let staged: Vec<String> = fs::read_dir(&docs)
        .map(|entries| {
            entries
                .filter_map(Result::ok)
                .map(|entry| entry.file_name().to_string_lossy().into_owned())
                .filter(|name| name.starts_with("jigc-feedback:"))
                .collect()
        })
        .unwrap_or_default();
    assert_eq!(staged, Vec::<String>::new(), "nothing staged in the task");
    assert_eq!(
        corpus.git(&["status", "--porcelain"]),
        status_before,
        "the checkout is untouched",
    );
    assert_eq!(
        fs::read(corpus.repo().join(&path)).expect("re-read"),
        committed,
        "the committed finding is byte-unchanged",
    );
}

/// The test `triage-jigc-feedback` names as pinning the fix it resolves.
const PINNED_BY: &str =
    "findings_workflows::report_jigc_feedback_lands_its_doc_alone_and_reads_back_open";

/// The `resolution` a triage authors.
pub(crate) const RESOLUTION: &str = "Fixed: the report's finalize commits its doc alone.";

/// `text` with every whitespace run folded to one space — composed prose wraps its lines,
/// so a sentence is asserted on its words, not on where the step file broke it.
fn prose(text: &str) -> String {
    text.split_whitespace().collect::<Vec<_>>().join(" ")
}

/// Assert the composed triage text states the leaves it changes and the intent form for `ty`.
pub(crate) fn assert_triage_text(composed: &Composed, ty: &str, leaves: &str) {
    let words = prose(&composed.text);
    for sentence in [
        format!("This step changes {leaves} — and nothing else"),
        format!(
            "Name the {} in the intent in plain words — its slug, without the `{ty}:` prefix \
             and its colon.",
            match ty {
                "jigc-feedback" => "finding",
                _ => "record",
            },
        ),
        format!(
            "the address, `{ty}:<slug>`, goes to the `doc` verbs below and never to `jigc start`."
        ),
    ] {
        assert!(
            words.contains(&sentence),
            "the composed triage text says `{sentence}`; got:\n{}",
            composed.text,
        );
    }
}

/// Start `workflow` the way its step says to — the finding named in plain words, its slug
/// alone — and assert the task id is minted from that slug, unmangled.
pub(crate) fn start_triage(corpus: &TrialCorpus, workflow: &str, address: &str) -> Composed {
    let slug = slug_of(address);
    let composed = start(corpus, workflow, &slug);
    assert!(
        composed.task.starts_with(&slug),
        "a plain-words intent mints the task id from the slug `{slug}`; got `{}`",
        composed.task,
    );
    composed
}

/// Finalize the report task `composed` through its emitted line and return the path of
/// the doc it filed at `address` (in `ty`'s store), read back off `doc list`.
fn land_report(corpus: &TrialCorpus, composed: &Composed, ty: &str, address: &str) -> String {
    let out = finalize(corpus, composed);
    assert!(out.status.success(), "the report lands; {}", text(&out));
    listed_row(corpus, ty, address)["path"]
        .as_str()
        .expect("a row has a path")
        .to_owned()
}

/// Stage a foreign path, run the triage task's emitted finalize, and assert it lands one
/// commit holding `path` alone, the foreign path left staged.
pub(crate) fn assert_lands_alone(corpus: &TrialCorpus, composed: &Composed, path: &str) {
    fs::create_dir_all(corpus.repo().join("src")).expect("mk src/");
    fs::write(corpus.repo().join(FOREIGN), "pub fn other_task() {}\n").expect("foreign file");
    corpus.git(&["add", "--", FOREIGN]);
    let before = commit_count(corpus);
    let out = finalize(corpus, composed);
    assert!(
        out.status.success(),
        "the triage's emitted finalize lands at exit 0; {}",
        text(&out),
    );
    assert_eq!(commit_count(corpus), before + 1, "exactly one commit");
    assert_eq!(
        head_files(corpus),
        vec![path.to_owned()],
        "the triage's commit holds the finding alone; {}",
        text(&out),
    );
    assert_eq!(
        corpus.git(&["diff", "--cached", "--name-only"]),
        FOREIGN,
        "the foreign path stays staged for the task it belongs to",
    );
}

/// The lines of `filed` the triage removed — append-only means only the `status:` line.
fn removed_lines(filed: &str, triaged: &str) -> Vec<String> {
    filed
        .lines()
        .filter(|line| !triaged.lines().any(|kept| kept == *line))
        .map(str::to_owned)
        .collect()
}

/// **(e)** `triage-jigc-feedback` moves a filed finding off `open` — `resolved`, a
/// `pinned-by` and a `resolution`, the first write acking its copy-in — and lands that doc
/// alone, every other line as filed. Its composed text names the four leaves, says nothing
/// else changes, and names the plain-words intent form.
#[test]
fn triage_jigc_feedback_resolves_a_filed_finding_and_lands_it_alone() {
    let corpus = TrialCorpus::build(State::Fresh);
    let (report, address) = author_jigc_feedback(&corpus, TITLE);
    let path = land_report(&corpus, &report, "jigc-feedback", &address);
    let filed = fs::read_to_string(corpus.repo().join(&path)).expect("read the finding");

    let triage = start_triage(&corpus, "triage-jigc-feedback", &address);
    assert_triage_text(
        &triage,
        "jigc-feedback",
        "four leaves — `status`, `duplicate-of`, `pinned-by` and `resolution`",
    );

    let slug = slug_of(&address);
    let first = run_emitted(
        &corpus,
        &triage,
        "jigc doc set-field jigc-feedback:<slug>#meta/status ",
        &[
            ("<slug>", &slug),
            ("<resolved|declined|duplicate|refuted>", "resolved"),
        ],
        None,
    );
    assert!(
        first.contains("copied in for update"),
        "the first write acks the committed finding's copy-in; got:\n{first}",
    );
    let pin = run_emitted(
        &corpus,
        &triage,
        "jigc doc set-field jigc-feedback:<slug>#meta/pinned-by ",
        &[("<slug>", &slug), ("<module>::<test_name>", PINNED_BY)],
        None,
    );
    assert!(
        !pin.contains("copied in for update"),
        "a later write edits the staged copy; got:\n{pin}",
    );
    run_emitted(
        &corpus,
        &triage,
        "jigc doc set-slot jigc-feedback:<slug>#resolution ",
        &[("<slug>", &slug)],
        Some(RESOLUTION),
    );
    let read_back = run_emitted(
        &corpus,
        &triage,
        "jigc doc show jigc-feedback:<slug> ",
        &[("<slug>", &slug)],
        None,
    );
    assert!(
        read_back.contains(RESOLUTION),
        "the emitted read-back serves the staged resolution; got:\n{read_back}",
    );
    fill_commit(&corpus, &triage, "feedback");

    assert_lands_alone(&corpus, &triage, &path);
    let row = listed_row(&corpus, "jigc-feedback", &address);
    assert_eq!(
        (&row["fields"]["status"], &row["fields"]["pinned-by"]),
        (&json!("resolved"), &json!(PINNED_BY)),
        "the committed row is resolved and pinned; row:\n{row:#}",
    );
    assert!(
        shown(&corpus, &address).to_string().contains(RESOLUTION),
        "the committed finding carries its resolution",
    );
    let triaged = fs::read_to_string(corpus.repo().join(&path)).expect("re-read");
    assert_eq!(
        removed_lines(&filed, &triaged),
        vec!["status: open".to_owned()],
        "only the status line left the filed finding; triaged:\n{triaged}",
    );
}

/// **(f)** `triage-inconsistency` moves a filed record off `open` — `intended` and a
/// `resolution` — and lands that doc alone, its three sides as filed. Its composed text
/// names the two leaves and the plain-words intent form.
#[test]
fn triage_inconsistency_marks_a_filed_record_intended_and_lands_it_alone() {
    let corpus = TrialCorpus::build(State::Fresh);
    let report = start(&corpus, VISIBLE, "the cache docs disagree on eviction");
    let address = author_inconsistency(&corpus, &report);
    let path = land_report(&corpus, &report, "inconsistency", &address);
    let filed = fs::read_to_string(corpus.repo().join(&path)).expect("read the record");

    let triage = start_triage(&corpus, "triage-inconsistency", &address);
    assert_triage_text(
        &triage,
        "inconsistency",
        "two leaves — `status` and `resolution`",
    );

    let slug = slug_of(&address);
    let first = run_emitted(
        &corpus,
        &triage,
        "jigc doc set-field inconsistency:<slug>#meta/status ",
        &[
            ("<slug>", &slug),
            ("<resolved|intended|refuted>", "intended"),
        ],
        None,
    );
    assert!(
        first.contains("copied in for update"),
        "the first write acks the committed record's copy-in; got:\n{first}",
    );
    let resolution = "Intended: the guide describes the planned eviction rule.";
    run_emitted(
        &corpus,
        &triage,
        "jigc doc set-slot inconsistency:<slug>#resolution ",
        &[("<slug>", &slug)],
        Some(resolution),
    );
    let read_back = run_emitted(
        &corpus,
        &triage,
        "jigc doc show inconsistency:<slug> ",
        &[("<slug>", &slug)],
        None,
    );
    assert!(
        read_back.contains(resolution),
        "the emitted read-back serves the staged resolution; got:\n{read_back}",
    );
    fill_commit(&corpus, &triage, "inconsistencies");

    assert_lands_alone(&corpus, &triage, &path);
    let row = listed_row(&corpus, "inconsistency", &address);
    assert_eq!(
        (&row["fields"]["status"], &row["item-count"]),
        (&json!("intended"), &json!(3)),
        "the committed record is intended, its three sides kept; row:\n{row:#}",
    );
    assert!(
        shown(&corpus, &address).to_string().contains(resolution),
        "the committed record carries its resolution",
    );
    let triaged = fs::read_to_string(corpus.repo().join(&path)).expect("re-read");
    assert_eq!(
        removed_lines(&filed, &triaged),
        vec!["status: open".to_owned()],
        "only the status line left the filed record; triaged:\n{triaged}",
    );
}

/// **(g)** A triage task allows no create: its composed text emits no `doc create` line,
/// and a `doc create` of its own doctype inside it is refused `create.gate-blocked`, with
/// nothing staged.
#[test]
fn a_create_inside_a_triage_task_is_gate_blocked() {
    let corpus = TrialCorpus::build(State::Fresh);
    for (workflow, ty) in [
        ("triage-jigc-feedback", "jigc-feedback"),
        ("triage-inconsistency", "inconsistency"),
    ] {
        let triage = start(&corpus, workflow, &format!("no create under {workflow}"));
        assert!(
            !triage.text.contains("jigc doc create"),
            "`{workflow}` emits no create; got:\n{}",
            triage.text,
        );
        let out = corpus.jigc(&[
            "doc",
            "create",
            ty,
            "--title",
            "A new finding",
            "--task",
            &triage.task,
        ]);
        assert!(
            !out.status.success()
                && String::from_utf8_lossy(&out.stderr).contains("create.gate-blocked"),
            "a `{ty}` create in a `{workflow}` task is refused `create.gate-blocked`; {}",
            text(&out),
        );
        let docs = corpus
            .repo()
            .join(".jigc/tasks")
            .join(&triage.task)
            .join("docs");
        let staged: Vec<String> = fs::read_dir(&docs)
            .map(|entries| {
                entries
                    .filter_map(Result::ok)
                    .map(|entry| entry.file_name().to_string_lossy().into_owned())
                    .filter(|name| name.starts_with(&format!("{ty}:")))
                    .collect()
            })
            .unwrap_or_default();
        assert_eq!(staged, Vec::<String>::new(), "`{workflow}`: nothing staged");
    }
}

/// The milestone flow D fans out under, and the id `milestone create` mints from it.
const MILESTONE_TITLE: &str = "Seed the findings store";
const MILESTONE: &str = "seed-the-findings-store";

/// Flow D's three report sub-tasks: `(workflow, intent, title)` — two findings into one store,
/// one inconsistency beside them.
const REPORTERS: [(&str, &str, &str); 3] = [
    ("report-jigc-feedback", "Report the staged sweep", TITLE),
    (
        "report-jigc-feedback",
        "Report the dropped title",
        "Doc show drops the title",
    ),
    (
        "report-inconsistency",
        "Report the eviction disagreement",
        INCONSISTENCY,
    ),
];

/// The own lines of a step a sub-task omits — `step:finalize-doc-only` and
/// `step:author-commit`, the methodology pack's — read off the raw step files: every body
/// line, `{{task.id}}` filled with `task`, but a `{{ cli.… }}` ref (it composes to a `Run:`
/// line the door and commit-write checks own) and any line a surviving report author step
/// also carries (the shared heredoc and read-back wording, which is not the omitted step's).
fn omitted_step_lines(task: &str) -> BTreeSet<String> {
    let steps = PathBuf::from(cli::pack_path!(methodology)).join("steps");
    let lines = |id: &str| -> BTreeSet<String> {
        let raw = fs::read_to_string(steps.join(format!("{id}.yaml")))
            .unwrap_or_else(|e| panic!("read step:{id}: {e}"));
        let body = raw
            .strip_prefix("---\n")
            .and_then(|rest| rest.split_once("\n---\n"))
            .map_or(raw.as_str(), |(_, body)| body);
        body.lines()
            .map(|line| line.trim().replace("{{task.id}}", task))
            .filter(|line| !line.is_empty() && !line.contains("{{"))
            .collect()
    };
    let survivors: BTreeSet<String> = ["author-jigc-feedback", "author-inconsistency"]
        .into_iter()
        .flat_map(lines)
        .collect();
    let own: BTreeSet<String> = ["finalize-doc-only", "author-commit"]
        .into_iter()
        .flat_map(lines)
        .filter(|line| !survivors.contains(line))
        .collect();
    assert!(
        own.contains(
            "Land this task's docs as exactly one commit. This finalize commits path-scoped:"
        ) && own.contains(
            "finalize renders the commit doc; it does not fill it, so set its header and prose"
        ),
        "the omitted steps' own lines are read off their files; got {own:#?}",
    );
    own
}

/// Every way `text`, composed for `task`, falls short of a sub-task of `milestone`, keyed by
/// the check it fails: it names the refused per-task door, asks for a `commit:<task>` write,
/// carries a line of an omitted step, or lacks the trailer naming the milestone's boundary.
fn sub_task_violations(text: &str, task: &str, milestone: &str) -> Vec<(&'static str, String)> {
    let mut found = Vec::new();
    if text.contains("jigc task finalize") {
        found.push(("per-task door", "names `jigc task finalize`".to_owned()));
    }
    let writes = commit_writes(text, task, None);
    if !writes.is_empty() {
        found.push(("commit-doc write", format!("{writes:?}")));
    }
    let own = omitted_step_lines(task);
    let carried: Vec<&str> = text
        .lines()
        .map(str::trim)
        .filter(|line| own.contains(*line))
        .collect();
    if !carried.is_empty() {
        found.push(("omitted step text", format!("{carried:#?}")));
    }
    let boundary = format!("`jigc milestone finalize {milestone}` is its only commit boundary");
    if !text
        .lines()
        .any(|line| line.starts_with("task scope: ") && line.contains(&boundary))
    {
        found.push((
            "milestone trailer",
            format!("no `task scope:` line naming {boundary}"),
        ));
    }
    found
}

/// **(h)** Flow D: a milestone with three report sub-tasks. Each `Spawn:` line `milestone
/// execute` prints is run verbatim; each composed text is free of the derived omission set
/// and ends in the trailer naming `jigc milestone finalize`; each sub-task files its doc in
/// its own worktree through its emitted lines; the join lands one commit holding exactly
/// the three docs and the milestone record, and `doc list` lists them all.
#[cfg(unix)]
#[test]
fn flow_d_three_report_sub_tasks_land_every_doc_at_the_join() {
    let corpus = TrialCorpus::build(State::Fresh);
    let created = corpus.jigc_ok(&["milestone", "create", MILESTONE_TITLE]);
    let record = created
        .lines()
        .find_map(|line| line.strip_prefix("record: "))
        .and_then(|rest| rest.split_whitespace().next())
        .unwrap_or_else(|| panic!("`milestone create` names its record; got:\n{created}"))
        .to_owned();
    let subs: Vec<(String, &str, &str)> = REPORTERS
        .iter()
        .map(|(workflow, intent, title)| {
            let ack = corpus.jigc_ok(&[
                "milestone",
                "add-task",
                MILESTONE,
                intent,
                "--workflow",
                workflow,
            ]);
            let sub = ack
                .split_once("task:")
                .and_then(|(_, rest)| rest.split_whitespace().next())
                .unwrap_or_else(|| panic!("the add-task ack names the sub-task; got:\n{ack}"))
                .to_owned();
            (sub, *workflow, *title)
        })
        .collect();
    corpus.jigc_ok(&["milestone", "provision", MILESTONE]);
    let executed = corpus.jigc_ok(&["milestone", "execute", MILESTONE]);
    let spans = spawn_spans(&executed);
    assert_eq!(
        spans.len(),
        REPORTERS.len(),
        "one `Spawn:` line per sub-task; got:\n{executed}",
    );

    let shim_bin = install_jigc_shim(&corpus.home());
    let mut filed = Vec::new();
    for (sub, workflow, title) in &subs {
        let span = spans
            .iter()
            .find(|span| span.ends_with(&format!(" --task {sub}")))
            .unwrap_or_else(|| panic!("a `Spawn:` line for `{sub}`; got:\n{executed}"));
        let out = run_span(&corpus.repo(), &corpus.home(), &shim_bin, span);
        assert!(
            out.status.success(),
            "the span `{span}` runs at exit 0; {}",
            text(&out)
        );
        let composed = Composed {
            task: sub.clone(),
            text: String::from_utf8(out.stdout).expect("utf-8 composed text"),
            cwd: corpus.repo().join(".jigc/worktrees").join(sub),
        };
        let violations = sub_task_violations(&composed.text, sub, MILESTONE);
        assert!(
            violations.is_empty(),
            "the `{workflow}` sub-task `{sub}`'s composed text carries no step of the omission \
             set and ends in the milestone trailer; violations {violations:#?} in:\n{}",
            composed.text,
        );
        let (ty, address) = match *workflow {
            "report-jigc-feedback" => (
                "jigc-feedback",
                file_jigc_feedback(&corpus, &composed, title),
            ),
            _ => ("inconsistency", file_inconsistency(&corpus, &composed)),
        };
        filed.push((ty, address));
    }

    let before = commit_count(&corpus);
    let out = corpus.jigc(&["milestone", "finalize", MILESTONE]);
    assert_eq!(
        out.status.code(),
        Some(0),
        "the join lands every report; {}",
        text(&out)
    );
    assert_eq!(commit_count(&corpus), before + 1, "exactly one commit");
    let mut expected: Vec<String> = filed
        .iter()
        .map(|(ty, address)| {
            listed_row(&corpus, ty, address)["path"]
                .as_str()
                .expect("a row has a path")
                .to_owned()
        })
        .chain([record])
        .collect();
    expected.sort();
    let mut landed = head_files(&corpus);
    landed.sort();
    assert_eq!(
        landed,
        expected,
        "the join's commit holds exactly the three docs and the milestone record; {}",
        text(&out),
    );
    for ty in ["jigc-feedback", "inconsistency"] {
        let listing: Value = stdout_json(
            &corpus.jigc(&["doc", "list", ty, "--format", "json"]),
            &[0],
            &format!("`doc list {ty}`"),
        );
        let listed: BTreeSet<&str> = listing["docs"]
            .as_array()
            .expect("`docs` is an array")
            .iter()
            .filter_map(|row| row["id"].as_str())
            .collect();
        let want: BTreeSet<&str> = filed
            .iter()
            .filter(|(t, _)| *t == ty)
            .map(|(_, address)| address.as_str())
            .collect();
        assert_eq!(listed, want, "`doc list {ty}` lists every filed doc");
    }
}

/// **(i)** The discriminating control for (h): `report-jigc-feedback` composed as an
/// **ordinary** task fails every one of (h)'s composition checks — it carries the per-task
/// door, the commit-doc writes, both omitted steps' text, and no milestone trailer.
#[test]
fn an_ordinary_report_task_fails_every_sub_task_composition_check() {
    let corpus = TrialCorpus::build(State::Fresh);
    let composed = start(&corpus, "report-jigc-feedback", "report it as a task");
    let failed: BTreeSet<&str> = sub_task_violations(&composed.text, &composed.task, MILESTONE)
        .into_iter()
        .map(|(check, _)| check)
        .collect();
    assert_eq!(
        failed,
        BTreeSet::from([
            "per-task door",
            "commit-doc write",
            "omitted step text",
            "milestone trailer",
        ]),
        "an ordinary report task fails every sub-task check; got:\n{}",
        composed.text,
    );
}
