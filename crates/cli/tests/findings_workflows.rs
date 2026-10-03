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
//!
//! **Red** is the mutant the doc-only step exists against: with `report-jigc-feedback`'s body
//! on `step:finalize` instead of `step:finalize-doc-only`, the report's commit sweeps the
//! foreign staged path in and (a)'s one-doc assertion fails.

use crate::support;

use std::fs;
use std::process::Output;

use serde_json::{Value, json};
use support::run_then_parse::stdout_json;
use support::trial_corpus::{State, TrialCorpus};

/// The three workflows the router catalog leaves out, each reached by name (§2).
const HIDDEN: [&str; 3] = [
    "report-jigc-feedback",
    "triage-jigc-feedback",
    "triage-inconsistency",
];

/// The router-visible report workflow (§2).
const VISIBLE: &str = "report-inconsistency";

/// The jigc-feedback finding every arm files first.
const TITLE: &str = "Finalize sweeps a staged path";

/// A path another task staged in the same checkout — never the report's to commit.
const FOREIGN: &str = "src/foreign.rs";

/// A fenced repro whose `#`-led line would be refused unfenced (the author step says so).
const REPRO: &str =
    "```sh\n# stage a file, then finalize the report\ngit add src/foreign.rs\n```\n";

/// The three sides of the inconsistency: `(title, --slug, says)`.
const SIDES: [(&str, &str, &str); 3] = [
    ("src/cache.rs", "code", "The cache evicts the oldest entry."),
    (
        "adr:cache-strategy#decision",
        "adr",
        "The cache evicts the least-recently-used entry.",
    ),
    ("docs/cache.md", "guide", "The cache never evicts."),
];

fn text(out: &Output) -> String {
    format!(
        "--- stdout ---\n{}\n--- stderr ---\n{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr),
    )
}

/// A composed work-workflow: the task id the binary printed it minted, and the composed text.
struct Composed {
    task: String,
    text: String,
}

/// The id `stdout` says was minted — read off the emitted `task minted:` line.
fn minted(stdout: &str) -> String {
    stdout
        .lines()
        .find_map(|line| line.strip_prefix("task minted: "))
        .unwrap_or_else(|| panic!("a mint prints `task minted: <id>`; got:\n{stdout}"))
        .trim()
        .to_owned()
}

/// `jigc start --workflow <workflow> <intent>` — the by-name door.
fn start(corpus: &TrialCorpus, workflow: &str, intent: &str) -> Composed {
    let text = corpus.jigc_ok(&["start", "--workflow", workflow, intent]);
    Composed {
        task: minted(&text),
        text,
    }
}

/// The one emitted command line of `text` that begins with `prefix`, its `Run: \`…\``
/// decoration stripped. A heredoc-opening example line (`… <<'EOF'`) is the illustrated form
/// of a line the step also emits bare, so it is skipped; the bare line is the one run.
fn emitted_line(text: &str, prefix: &str) -> String {
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
fn argv(line: &str, fills: &[(&str, &str)]) -> Vec<String> {
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
fn run_emitted(
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
    match stdin {
        Some(payload) => corpus.jigc_stdin(&args, payload),
        None => corpus.jigc(&args),
    }
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
fn slug_of(address: &str) -> String {
    address
        .split_once(':')
        .unwrap_or_else(|| panic!("a `type:slug` address; got `{address}`"))
        .1
        .to_owned()
}

/// Fill the task's commit doc through the commit step's emitted lines.
fn fill_commit(corpus: &TrialCorpus, composed: &Composed, scope: &str) {
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
fn author_jigc_feedback(corpus: &TrialCorpus, title: &str) -> (Composed, String) {
    let composed = start(
        corpus,
        "report-jigc-feedback",
        "the finalize sweeps a staged path",
    );
    let address = create(corpus, &composed, "jigc-feedback", title);
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
            &composed,
            &format!("jigc doc set-field jigc-feedback:<slug>#meta/{leaf} "),
            &[fills[0], (hole, value)],
            None,
        );
    }
    run_emitted(
        corpus,
        &composed,
        "jigc doc set-slot jigc-feedback:<slug>#description ",
        &fills,
        Some("The report's finalize committed a path another task had staged."),
    );
    run_emitted(
        corpus,
        &composed,
        "jigc doc set-slot jigc-feedback:<slug>#repro ",
        &fills,
        Some(REPRO),
    );
    let read_back = run_emitted(
        corpus,
        &composed,
        "jigc doc show jigc-feedback:<slug> ",
        &fills,
        None,
    );
    assert!(
        read_back.contains(REPRO.trim_end()),
        "the emitted read-back serves the staged, fenced repro; got:\n{read_back}",
    );
    fill_commit(corpus, &composed, "feedback");
    (composed, address)
}

/// Run the composed text's emitted finalize line.
fn finalize(corpus: &TrialCorpus, composed: &Composed) -> Output {
    run_emitted_raw(
        corpus,
        composed,
        &format!("jigc task finalize {}", composed.task),
        &[],
        None,
    )
}

/// The paths HEAD's commit changed.
fn head_files(corpus: &TrialCorpus) -> Vec<String> {
    corpus
        .git(&["show", "--name-only", "--pretty=format:", "HEAD"])
        .lines()
        .filter(|line| !line.is_empty())
        .map(str::to_owned)
        .collect()
}

fn commit_count(corpus: &TrialCorpus) -> usize {
    corpus
        .git(&["rev-list", "--count", "HEAD"])
        .parse()
        .expect("a count")
}

/// The committed `doc list <ty> --format json` row for `address`.
fn listed_row(corpus: &TrialCorpus, ty: &str, address: &str) -> Value {
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
fn shown(corpus: &TrialCorpus, address: &str) -> Value {
    stdout_json(
        &corpus.jigc(&["doc", "show", address, "--format", "json"]),
        &[0],
        &format!("`doc show {address}`"),
    )
}

/// Assert both read surfaces carry `title` and project `fields.status == "open"`.
fn assert_reads_open(corpus: &TrialCorpus, ty: &str, address: &str, title: &str, when: &str) {
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
fn catalog(corpus: &TrialCorpus) -> (String, Vec<String>) {
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
    };
    assert!(
        composed.text.contains("jigc doc create inconsistency "),
        "the router's re-run line composed `{VISIBLE}`; got:\n{}",
        composed.text,
    );

    let address = create(
        &corpus,
        &composed,
        "inconsistency",
        "Cache eviction disagrees",
    );
    let slug = slug_of(&address);
    run_emitted(
        &corpus,
        &composed,
        "jigc doc set-field inconsistency:<slug>#meta/kind ",
        &[("<slug>", &slug), ("<code-doc|doc-doc>", "code-doc")],
        None,
    );
    for (side_title, side, says) in SIDES {
        run_emitted(
            &corpus,
            &composed,
            "jigc doc add-item inconsistency:<slug>#sides ",
            &[
                ("<slug>", &slug),
                ("<path or address>", side_title),
                ("<side>", side),
            ],
            None,
        );
        run_emitted(
            &corpus,
            &composed,
            "jigc doc set-slot inconsistency:<slug>#sides/<side>/says ",
            &[("<slug>", &slug), ("<side>", side)],
            Some(says),
        );
    }
    run_emitted(
        &corpus,
        &composed,
        "jigc doc set-slot inconsistency:<slug>#description ",
        &[("<slug>", &slug)],
        Some("The code, the decision and the guide each state a different eviction rule."),
    );
    run_emitted(
        &corpus,
        &composed,
        "jigc doc set-slot inconsistency:<slug>#evidence ",
        &[("<slug>", &slug)],
        Some("Read all three side by side."),
    );
    fill_commit(&corpus, &composed, "inconsistencies");

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
