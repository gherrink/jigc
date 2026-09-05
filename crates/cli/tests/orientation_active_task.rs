//! M50 Increment 5 / T2 — **bare `jigc start` over a live task**
//! (`design/bootstrap.md` → Orientation output examples, states 3 and 4 collapsed;
//! M50 → the Settle, D3).
//!
//! **The lie this suite retires:** `jigc start --format json` emitted
//! `"state": "clean"` — declared in `engine::result` as *no active task* — over a
//! repo holding one, and the `refs-post-hoc` compose golden pinned that falsehood as
//! expected output. Law 1 on a versioned machine contract.
//!
//! **What it pins**, driving the real binary against the shared fixture corpus:
//!   * the `active-task` tag carries the six keys the Settle enumerated
//!     (`id`, `workflow`, `intent`, `base`, `staged`, `findings`) — plus the seventh
//!     the route floor forced, `milestone` — and the word `clean` reaches the envelope
//!     nowhere;
//!   * the agent text names the task, its workflow, its base and its staged doc
//!     identities, and carries the four `Run:` directives — the discard one bearing
//!     the **consent** (`--force`) exactly where D1's guard demands it — and the
//!     sub-task arm pins both of the two state-dependent directives' other halves:
//!     the commit directive naming the milestone door that runs, and the abandon
//!     directive omitting a consent the door does not ask for;
//!   * a task carrying a blocking finding renders it with its code **and its route**,
//!     at **exit 0** — orientation reports, it does not gate;
//!   * two live tasks render as **two rows under one tag** (the variant carries the
//!     active *set*, not one task);
//!   * a project with no live task still renders `Clean` — proven **byte-identical**
//!     by discarding the live task and comparing against a task-less corpus's
//!     orientation with the repo path normalized away, so the new variant is confined
//!     to the condition that produces it — and the **catalog** the active view carries
//!     beneath its work in progress is that clean view's own bytes, which is what keeps
//!     the shipped `create.gate-blocked` route (*"`jigc start` lists the catalog"*)
//!     landing where it claims in the one state that can print it;
//!   * and the whole active block, **byte-for-byte** — the pin the compose sweep
//!     cannot hold for this state, because the base pin is a per-run commit sha
//!     (`compose_goldens::EXCLUSIONS` carries the measurement). This suite can supply
//!     the varying token from the fixture's own `base.json`, so it pins these bytes
//!     harder than a golden does rather than trading them away.

use crate::support;

use serde_json::Value;
use support::trial_corpus::{State, TrialCorpus};

/// Bare `jigc start` in the given format, asserted successful, returning stdout.
fn orient(corpus: &TrialCorpus, format: Option<&str>) -> String {
    let out = match format {
        Some(format) => corpus.jigc(&["start", "--format", format]),
        None => corpus.jigc(&["start"]),
    };
    assert!(
        out.status.success(),
        "bare `jigc start` must report a live task at exit 0, not gate on it ({}):\n{}\n{}",
        out.status,
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr),
    );
    String::from_utf8(out.stdout).expect("utf-8 stdout")
}

/// The orientation envelope as JSON.
fn orient_json(corpus: &TrialCorpus) -> Value {
    serde_json::from_str(&orient(corpus, Some("json"))).expect("orientation is one JSON value")
}

/// (a) The envelope tags the state as `active-task` and populates every key the
/// Settle enumerated — and the `clean` claim is gone from the document entirely.
#[test]
fn the_json_envelope_tags_the_active_set_and_no_longer_claims_clean() {
    let corpus = TrialCorpus::build(State::RefsPostHoc);
    let task = corpus
        .live_task()
        .expect("refs-post-hoc leaves a task live");
    let raw = orient(&corpus, Some("json"));
    let view: Value = serde_json::from_str(&raw).expect("orientation is one JSON value");

    assert_eq!(
        view["state"], "active-task",
        "a repo with a live task must not tag as anything else; got:\n{view:#}"
    );
    assert!(
        !raw.contains("\"clean\""),
        "the `clean` tag must not survive anywhere in the envelope; got:\n{raw}"
    );
    assert!(
        view["header"].as_str().is_some_and(|h| h.contains("Pack:")),
        "the provenance header rides the active view too; got:\n{view:#}"
    );

    let tasks = view["tasks"].as_array().expect("`tasks` is an array");
    assert_eq!(tasks.len(), 1, "one live task, one row; got:\n{view:#}");
    let row = &tasks[0];
    assert_eq!(row["id"], task);
    assert_eq!(row["workflow"], "form-vision");
    assert_eq!(row["intent"], "ground the vision in research");
    // Projected even when absent: which commit boundary a task has is a fact a driver
    // reads, never a key it has to notice is missing.
    assert!(
        row.get("milestone").is_some_and(Value::is_null),
        "a top-level task carries `milestone: null`, not no key; got:\n{row:#}"
    );
    assert!(
        row["base"]["sha"].as_str().is_some_and(|s| s.len() == 40),
        "the base pin carries the full sha; got:\n{row:#}"
    );
    assert!(
        row["base"]["short"].as_str().is_some_and(|s| !s.is_empty()),
        "the base pin carries the short sha the text renders; got:\n{row:#}"
    );
    let staged: Vec<&str> = row["staged"]
        .as_array()
        .expect("`staged` is an array")
        .iter()
        .map(|v| v.as_str().expect("staged ids are strings"))
        .collect();
    assert_eq!(
        staged,
        vec![format!("commit:{task}").as_str(), "vision:vision"],
        "the staged set is this task's own docs, sorted; got:\n{row:#}"
    );
    let findings = row["findings"]
        .as_array()
        .expect("`findings` is an array when the sweep ran");
    assert!(
        findings
            .iter()
            .any(|f| f["severity"] == "blocking" && f["code"].as_str().is_some()),
        "the sweep's findings ride the envelope as data; got:\n{row:#}"
    );
}

/// (b) The agent text names the task, its workflow, its base and its staged ids, and
/// carries the four `Run:` directives — the discard one carrying its consent.
#[test]
fn the_agent_text_names_the_task_and_routes_with_the_discard_consent() {
    let corpus = TrialCorpus::build(State::RefsPostHoc);
    let task = corpus
        .live_task()
        .expect("refs-post-hoc leaves a task live");
    let text = orient(&corpus, None);

    for token in [
        format!("Active task: {task}"),
        String::from("form-vision"),
        String::from("ground the vision in research"),
        format!("commit:{task}"),
        String::from("vision:vision"),
    ] {
        assert!(
            text.contains(&token),
            "orientation must name `{token}`; got:\n{text}"
        );
    }
    let base = std::fs::read_to_string(corpus.repo().join(format!(".jigc/tasks/{task}/base.json")))
        .expect("the live task has a base pin");
    let pin: Value = serde_json::from_str(&base).expect("the pin is JSON");
    let short = pin["short"].as_str().expect("the pin carries a short sha");
    assert!(
        text.contains(short),
        "orientation must name the base the task is pinned to (`{short}`); got:\n{text}"
    );

    for run in [
        format!("Run: `jigc start --task {task}`"),
        format!("Run: `jigc task validate {task}`"),
        format!("Run: `jigc task finalize {task}`"),
        format!("Run: `jigc task discard {task} --force`"),
    ] {
        assert!(
            text.contains(&run),
            "orientation must carry `{run}`; got:\n{text}"
        );
    }
    // The consent is not decoration: `jigc task discard <id>` without it refuses over
    // exactly the staged docs this view just listed, so a route printed without it
    // would be a route the wave's own guard blocks.
    let refused = corpus.jigc(&["task", "discard", task]);
    assert!(
        !refused.status.success(),
        "D1's guard must refuse the consent-less discard this view would otherwise route to"
    );

    // **The emitted bytes are the contract**: two of the four directives are extracted
    // from the render and run **verbatim**, rather than rebuilt here — a directive
    // reconstructed in test code can pass while the bytes an agent would paste are
    // broken. (`task validate` is deliberately not run: it exits 3 over this task's
    // blocking findings, which is that door's correct answer, not a broken route. The
    // commit directive is not run either — it would land a commit.)
    for directive in ["start --task", "task discard"] {
        let argv = emitted_argv(&text, directive);
        let out = corpus.jigc(&argv.iter().map(String::as_str).collect::<Vec<_>>());
        assert!(
            out.status.success(),
            "the emitted `{directive}` directive must run exactly as printed ({}):\n{}",
            out.status,
            String::from_utf8_lossy(&out.stderr),
        );
    }
}

/// The argv of the emitted `Run:` directive whose command begins with `verb`, taken
/// **off the rendered bytes** — so a directive is executed as printed rather than
/// rebuilt from what the test believes was printed.
fn emitted_argv(text: &str, verb: &str) -> Vec<String> {
    text.lines()
        .find_map(|line| {
            let inner = line.strip_prefix("Run: `")?;
            let (command, _) = inner.split_once('`')?;
            let rest = command.strip_prefix("jigc ")?;
            rest.starts_with(verb)
                .then(|| rest.split_whitespace().map(str::to_owned).collect())
        })
        .unwrap_or_else(|| panic!("no `Run:` directive for `{verb}` in:\n{text}"))
}

/// (c) A blocking finding renders with its code and its route — at exit 0. Orientation
/// reports what `jigc task validate` would block on; it is not itself a gate.
#[test]
fn a_blocking_finding_renders_with_code_and_route_at_exit_zero() {
    let corpus = TrialCorpus::build(State::RefsPostHoc);
    let task = corpus
        .live_task()
        .expect("refs-post-hoc leaves a task live");
    let out = corpus.jigc(&["start"]);
    assert_eq!(
        out.status.code(),
        Some(0),
        "orientation reports a blocked task at exit 0"
    );
    let text = String::from_utf8(out.stdout).expect("utf-8 stdout");

    assert!(
        text.contains("blocking · schema-conformance.required-slot-present"),
        "the blocking finding renders with its severity and code; got:\n{text}"
    );
    assert!(
        text.contains(&format!(
            "route: `jigc doc set-slot commit:{task}#summary --task {task} --from-file -`"
        )),
        "the route floor holds on the orientation surface too; got:\n{text}"
    );
    assert!(
        text.contains("findings:"),
        "the per-task summary states the finding tally; got:\n{text}"
    );
}

/// (d) Two live tasks render as two rows under one tag — the variant carries the
/// active set, so N > 1 has a shape rather than a guess.
#[test]
fn two_live_tasks_render_two_rows_under_one_tag() {
    let corpus = TrialCorpus::build(State::CommittedSingletons);
    let first = corpus.start_workflow("quick-fix", "tidy the loader");
    let second = corpus.start_workflow("quick-fix", "widen the reader");

    let view = orient_json(&corpus);
    assert_eq!(view["state"], "active-task");
    let ids: Vec<&str> = view["tasks"]
        .as_array()
        .expect("`tasks` is an array")
        .iter()
        .map(|t| t["id"].as_str().expect("each row carries an id"))
        .collect();
    assert_eq!(
        ids,
        vec![first.as_str(), second.as_str()]
            .into_iter()
            .collect::<std::collections::BTreeSet<_>>()
            .into_iter()
            .collect::<Vec<_>>(),
        "both live tasks ride one tag, in the enumerator's stable order; got:\n{view:#}"
    );

    let text = orient(&corpus, None);
    assert_eq!(
        text.matches("Active task: ").count(),
        2,
        "two rows, one tag; got:\n{text}"
    );
}

/// (e) With no live task the view is `Clean` — **byte-identical** to a task-less
/// corpus's, once the repo path (the harness's one normalization) is taken out. The
/// active variant is confined to the condition that produces it.
#[test]
fn a_task_less_project_still_renders_clean_byte_identical() {
    let live = TrialCorpus::build(State::RefsPostHoc);
    let task = live.live_task().expect("refs-post-hoc leaves a task live");
    live.jigc_ok(&["task", "discard", task, "--force"]);

    let quiet = TrialCorpus::build(State::CommittedSingletons);

    let normalize = |text: &str, corpus: &TrialCorpus| {
        text.replace(&corpus.repo().display().to_string(), "<REPO>")
    };
    let quiet_text = orient(&quiet, None);
    assert_eq!(
        normalize(&orient(&live, None), &live),
        normalize(&quiet_text, &quiet),
        "a project whose last task was discarded renders the clean view unchanged"
    );
    assert_eq!(orient_json(&live)["state"], "clean");

    // And the catalog the ACTIVE view carries beneath its work-in-progress is the clean
    // view's own bytes — the block `create.gate-blocked` routes to, which that refusal
    // can only ever print while a task is live. Pinned by equality with the surface the
    // compose goldens hold over five states, rather than restated here.
    let with_task = TrialCorpus::build(State::RefsPostHoc);
    let catalog = |text: &str| {
        let (_, tail) = text
            .split_once("\nAvailable workflows:")
            .expect("both views carry the catalog");
        format!("Available workflows:{tail}")
    };
    assert_eq!(
        catalog(&orient(&with_task, None)),
        catalog(&quiet_text),
        "the active view's catalog must be the clean view's, byte for byte"
    );
}

/// The active block, **byte-for-byte** — the compensating pin for the two goldens
/// `compose_goldens::EXCLUSIONS` declares out (the base pin is a per-run commit sha).
///
/// Compared from `Active task:` onward: the provenance header above it is the *clean*
/// view's line too, and is still goldened over the five task-less states. The one
/// generated fragment — the `task validate` coverage clause — comes from the shipped
/// generator, because that clause is fenced across its eight surfaces by
/// `crate::gate_coverage` and restating it here would pin a copy instead of the rule.
#[test]
fn the_active_block_renders_byte_for_byte() {
    let corpus = TrialCorpus::build(State::RefsPostHoc);
    let task = corpus
        .live_task()
        .expect("refs-post-hoc leaves a task live");
    let text = orient(&corpus, None);

    let pin: Value = serde_json::from_str(
        &std::fs::read_to_string(corpus.repo().join(format!(".jigc/tasks/{task}/base.json")))
            .expect("the live task has a base pin"),
    )
    .expect("the pin is JSON");
    let short = pin["short"].as_str().expect("the pin carries a short sha");
    let coverage = cli::gate_coverage::whats_left_coverage();

    let expected = format!(
        "Active task: {task}
  workflow: form-vision
  intent:   ground the vision in research
  base:     {short}
  staged:   commit:{task}, vision:vision
  findings: 2 blocking, 1 advisory
blocking · schema-conformance.field-value-conformant — `commit:{task}`: field `type` in section `header`: \"\" is not a member of enum \"type\" (allowed: feat, fix, docs, style, refactor, perf, test, build, ci, chore, revert)
  at: commit:{task}#header/type
  route: `jigc doc set-field commit:{task}#header/type --task {task} --value <value>` to correct the value
blocking · schema-conformance.required-slot-present — `commit:{task}`: required slot in section `summary` is empty
  at: commit:{task}#summary · line 9
  route: `jigc doc set-slot commit:{task}#summary --task {task} --from-file -` to fill the empty slot
advisory · file-state.staged-copy — staged copy of `VISION.md` — this task's in-flight version of the doc
  at: VISION.md
  route: no action needed — the staged copy is validated in-task and baselined when its finalize lands

Run: `jigc start --task {task}`   — resume: re-composes this task's own workflow where it left off
Run: `jigc task validate {task}`   — {coverage}
Run: `jigc task finalize {task}`   — validate + commit
Run: `jigc task discard {task} --force`   — abandon: removes the working area and the doc(s) staged in it, which no commit has a copy of; `--force` is the consent this door refuses without
"
    );
    assert_eq!(
        active_block(&text),
        expected,
        "the active block moved; look at the diff before accepting it"
    );
}

/// The rendered bytes from `Active task:` up to the catalog beneath it — the region
/// this task ships. The header above it and the catalog below it are the *clean* view's
/// own bytes, goldened over the five task-less states, and arm (e) pins that they are
/// the same bytes here.
fn active_block(text: &str) -> String {
    let (_, tail) = text
        .split_once("Active task: ")
        .expect("the active view names its task");
    let (block, _) = tail
        .split_once("\nAvailable workflows:")
        .expect("the catalog rides the active view too");
    format!("Active task: {block}")
}

/// A **milestone sub-task** is routed to the door that runs. Its area is an ordinary
/// `.jigc/tasks/<id>/`, so the active set lists it — but `jigc task finalize <sub>`
/// refuses outright (the milestone door is a sub-task's only commit boundary), and a
/// route this binary's own guard blocks is a route-floor defect. The same row exercises
/// the abandon directive's **other** arm: a sub-task at `add-task` time stages nothing,
/// so the consent is not printed over a door that does not ask for it.
#[test]
fn a_sub_task_is_routed_to_the_milestone_door_and_omits_the_unneeded_consent() {
    let corpus = TrialCorpus::build(State::CommittedSingletons);
    corpus.jigc_ok(&["milestone", "create", "Cache rework"]);
    corpus.jigc_ok(&[
        "milestone",
        "add-task",
        "cache-rework",
        "Warm the read cache",
    ]);

    let view = orient_json(&corpus);
    let row = &view["tasks"][0];
    let sub = row["id"]
        .as_str()
        .expect("the sub-task rides the active set");
    assert_eq!(
        row["milestone"], "cache-rework",
        "the envelope carries which milestone owns the task; got:\n{row:#}"
    );
    assert_eq!(
        row["staged"].as_array().map(Vec::len),
        Some(0),
        "a sub-task at add-task time stages nothing; got:\n{row:#}"
    );

    let text = orient(&corpus, None);
    assert!(
        text.contains("Run: `jigc milestone finalize cache-rework`"),
        "the commit directive must name the door that runs; got:\n{text}"
    );
    assert!(
        !text.contains(&format!("Run: `jigc task finalize {sub}`")),
        "the door that refuses must not be routed to; got:\n{text}"
    );
    assert!(
        text.contains(&format!("Run: `jigc task discard {sub}`   — abandon")),
        "with nothing staged the consent is not printed; got:\n{text}"
    );

    // The refusal the route steers around is real, not assumed: drive it.
    let refused = corpus.jigc(&["task", "finalize", sub]);
    assert!(
        !refused.status.success(),
        "`jigc task finalize <sub-task>` must refuse, or this route's whole reason is gone"
    );
    assert!(
        String::from_utf8_lossy(&refused.stderr).contains("cache-rework"),
        "and its refusal names the milestone the orientation routes to"
    );
    // And the directive orientation printed instead is one the binary accepts: extracted
    // off the rendered bytes and run verbatim, it reaches the milestone door's own gate
    // rather than a parse or lookup failure.
    let argv = emitted_argv(&text, "milestone finalize");
    let reached = corpus.jigc(&argv.iter().map(String::as_str).collect::<Vec<_>>());
    let stderr = String::from_utf8_lossy(&reached.stderr).into_owned();
    assert!(
        !stderr.contains("no milestone") && !stderr.contains("unexpected argument"),
        "the emitted directive must reach the milestone door; got:\n{stderr}"
    );

    // The consent-free abandon directive runs as printed — the other half of B9's
    // disjunction, proven on the emitted bytes rather than on the branch that wrote them.
    let argv = emitted_argv(&text, "task discard");
    let out = corpus.jigc(&argv.iter().map(String::as_str).collect::<Vec<_>>());
    assert!(
        out.status.success(),
        "the emitted abandon directive must run exactly as printed ({}):\n{}",
        out.status,
        String::from_utf8_lossy(&out.stderr),
    );
}
