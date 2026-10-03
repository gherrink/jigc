//! M55 Increment 11 / T4 — **flow 58 (D): several reporters into one store, through one join**
//! (`design/worked-examples.md` → 58; `design/findings-channel.md` §6 → S2, §7, §11 → D).
//!
//! One milestone carries three report sub-tasks — two `report-jigc-feedback`, one
//! `report-inconsistency` — added with `milestone add-task --workflow`, provisioned and
//! executed. Each `Spawn:` line `milestone execute` printed is run **verbatim** through
//! `sub_task_composition`'s `jigc` shim, and each sub-task files its doc in its own worktree
//! through the composed text's emitted lines (`findings_workflows`' helpers, reused, never
//! copied):
//!
//! 1. **The composition.** Each sub-task's composed text carries no line of any step in the
//!    **derived** omission set — read from `sub_task_composition::derive_embedded` under the
//!    `finalize.fan-out.squash` value `jigc config get` resolves in the corpus, never a hand
//!    list — names no `jigc task finalize`, asks for no `commit:<sub>` write, and carries the
//!    `task scope:` trailer naming `jigc milestone finalize <m>` as its only boundary.
//! 2. **The join.** `jigc milestone finalize` exits 0 with exactly one commit holding every
//!    filed doc and the milestone record, and `doc list` lists every filed doc. The two
//!    `jigc-feedback` reporters mint **the same id** (titles differing only by a `!`), so the
//!    join's colliding-new-instance rule is exercised: the lower task id keeps the bare slug,
//!    the higher takes `-2`, each carrying its own reporter's title.
//! 3. **By task id, not completion order.** The walk runs twice, the sub-tasks filing in
//!    task-id order and in reverse, and the landed docs are byte-identical path for path.
//! 4. **The seed fence's predicate over the flow's own output.** The landed docs, copied into
//!    a fresh corpus under its `docs-root`, `jigc ingest` then `jigc validate --format json`,
//!    carry zero findings and list exactly as managed (`seed_fence::assert_adoptable`).
//!
//! **Red** is the discriminating control: the same composition checks over each report
//! workflow composed as an **ordinary** task fail every one — the per-task door, the
//! commit-doc writes, each derived member's own text, and the trailer.
//!
//! **What this adds over `findings_workflows` (h)/(i)**, which drive three report sub-tasks to
//! one join commit against a hand-listed pair of omitted steps: the omission set is the
//! production derivation; the reporters collide on one id and the join resolves it by task id
//! under two filing orders; and the join's output meets the seed's adoptability predicate.

use crate::findings_workflows::{
    Composed, TITLE, commit_count, file_inconsistency, file_jigc_feedback, head_files, listed_row,
    start, text,
};
use crate::seed_fence::assert_adoptable;
use crate::sub_task_composition::{commit_writes, derive_embedded};
#[cfg(unix)]
use crate::sub_task_composition::{install_jigc_shim, run_span, spawn_spans};
use crate::support::run_then_parse::stdout_json;
use crate::support::trial_corpus::{State, TrialCorpus};

use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::PathBuf;

use serde_json::Value;

/// The milestone every walk fans out under.
const MILESTONE_TITLE: &str = "File the findings together";

/// The three reporters: `(workflow, intent, title)`. The two `jigc-feedback` titles slug onto
/// one id; the inconsistency's title is `findings_workflows`' own.
const REPORTERS: [(&str, &str, &str); 3] = [
    ("report-jigc-feedback", "Report the staged sweep", TITLE),
    (
        "report-jigc-feedback",
        "Report the sweep again",
        "Finalize sweeps a staged path!",
    ),
    (
        "report-inconsistency",
        "Report the eviction disagreement",
        "",
    ),
];

/// The two doc homes under the corpus's `docs-root`.
const HOMES: [&str; 2] = ["docs/jigc-feedback", "docs/inconsistencies"];

/// The `finalize.fan-out.squash` value the corpus resolves — the knob the derivation keys on.
fn squash(corpus: &TrialCorpus) -> bool {
    let got: Value = stdout_json(
        &corpus.jigc(&[
            "config",
            "get",
            "finalize.fan-out.squash",
            "--format",
            "json",
        ]),
        &[0],
        "`jigc config get finalize.fan-out.squash`",
    );
    match got["value"].as_str() {
        Some("true") => true,
        Some("false") => false,
        _ => panic!("the knob resolves to a boolean; got:\n{got:#}"),
    }
}

/// A step file's body lines, its front-matter dropped, trimmed and non-empty.
fn body_lines(raw: &str) -> Vec<String> {
    let body = raw
        .strip_prefix("---\n")
        .and_then(|rest| rest.split_once("\n---\n"))
        .map_or(raw, |(_, body)| body);
    body.lines()
        .map(str::trim)
        .filter(|line| !line.is_empty())
        .map(str::to_owned)
        .collect()
}

/// The `{{ include: step:<id> }}` ids of `lines`.
fn includes(lines: &[String]) -> Vec<String> {
    lines
        .iter()
        .filter_map(|line| {
            line.strip_prefix("{{ include: step:")
                .and_then(|rest| rest.strip_suffix(" }}"))
                .map(str::to_owned)
        })
        .collect()
}

/// Every step `workflow` reaches, closed under inclusion, each with its body lines — read off
/// the methodology pack's raw files, the pack both report workflows ship in.
fn reachable_steps(workflow: &str) -> BTreeMap<String, Vec<String>> {
    let pack = PathBuf::from(cli::pack_path!(methodology));
    let read = |path: PathBuf| {
        fs::read_to_string(&path).unwrap_or_else(|e| panic!("read {}: {e}", path.display()))
    };
    let mut pending = includes(&body_lines(&read(
        pack.join("workflows").join(format!("{workflow}.yaml")),
    )));
    let mut steps = BTreeMap::new();
    while let Some(id) = pending.pop() {
        if steps.contains_key(&id) {
            continue;
        }
        let lines = body_lines(&read(pack.join("steps").join(format!("{id}.yaml"))));
        pending.extend(includes(&lines));
        steps.insert(id, lines);
    }
    steps
}

/// Each derived member's **own** lines in `workflow`, composed for `task`: its body lines,
/// `{{task.id}}` filled, but any line carrying a `{{` ref (it composes to a `Run:` line the
/// door and commit-write checks own) and any line a surviving step also carries. Every member
/// must have at least one, or its absence would prove nothing.
fn own_lines(
    workflow: &str,
    members: &BTreeSet<String>,
    task: &str,
) -> BTreeMap<String, BTreeSet<String>> {
    let steps = reachable_steps(workflow);
    let fill = |lines: &Vec<String>| -> BTreeSet<String> {
        lines
            .iter()
            .map(|line| line.replace("{{task.id}}", task))
            .filter(|line| !line.contains("{{"))
            .collect()
    };
    let survivors: BTreeSet<String> = steps
        .iter()
        .filter(|(id, _)| !members.contains(*id))
        .flat_map(|(_, lines)| fill(lines))
        .collect();
    members
        .iter()
        .map(|member| {
            let lines = steps.get(member).unwrap_or_else(|| {
                panic!("`{workflow}` reaches its derived member `{member}`; reached {steps:#?}")
            });
            let own: BTreeSet<String> = fill(lines)
                .into_iter()
                .filter(|line| !survivors.contains(line))
                .collect();
            assert!(
                !own.is_empty(),
                "`{member}` has a line of its own in `{workflow}`, so its absence is checkable",
            );
            (member.clone(), own)
        })
        .collect()
}

/// Every way `text`, composed for `task` from `workflow`, falls short of a sub-task of
/// `milestone` under the derived omission set `members`, keyed by the check it fails.
fn sub_task_violations(
    text: &str,
    task: &str,
    workflow: &str,
    milestone: &str,
    members: &BTreeSet<String>,
) -> BTreeMap<String, String> {
    let mut found = BTreeMap::new();
    if text.contains("jigc task finalize") {
        found.insert(
            "per-task door".to_owned(),
            "names `jigc task finalize`".to_owned(),
        );
    }
    let writes = commit_writes(text, task, None);
    if !writes.is_empty() {
        found.insert("commit-doc write".to_owned(), format!("{writes:?}"));
    }
    for (member, own) in own_lines(workflow, members, task) {
        let carried: Vec<&str> = text
            .lines()
            .map(str::trim)
            .filter(|line| own.contains(*line))
            .collect();
        if !carried.is_empty() {
            found.insert(format!("step:{member}"), format!("{carried:#?}"));
        }
    }
    let boundary = format!("`jigc milestone finalize {milestone}` is its only commit boundary");
    if !text
        .lines()
        .any(|line| line.starts_with("task scope: ") && line.contains(&boundary))
    {
        found.insert(
            "milestone trailer".to_owned(),
            format!("no `task scope:` line naming {boundary}"),
        );
    }
    found
}

/// The derived omission set of the methodology workflow `workflow` under `squash`.
fn derived(workflow: &str, squash: bool) -> BTreeSet<String> {
    let set = derive_embedded(squash)
        .remove(&format!("methodology:{workflow}"))
        .unwrap_or_else(|| panic!("the derivation covers `methodology:{workflow}`"));
    assert!(
        !set.is_empty(),
        "`{workflow}` derives a non-empty omission set"
    );
    set
}

/// The order the sub-tasks file their docs in, against their task ids.
#[derive(Clone, Copy, Debug)]
enum Filing {
    ByTaskId,
    Reverse,
}

/// What one walk landed: the corpus, HEAD's file list, and every doc under [`HOMES`] by
/// repo-relative path.
struct Landed {
    corpus: TrialCorpus,
    head: Vec<String>,
    docs: BTreeMap<String, String>,
}

/// One walk of flow D, the sub-tasks filing in `filing` order.
#[cfg(unix)]
fn walk(filing: Filing) -> Landed {
    let corpus = TrialCorpus::build(State::Fresh);
    let squash = squash(&corpus);

    let created = corpus.jigc_ok(&["milestone", "create", MILESTONE_TITLE]);
    let milestone = created
        .lines()
        .find_map(|line| line.strip_prefix("minted milestone:"))
        .and_then(|rest| rest.split_whitespace().next())
        .unwrap_or_else(|| panic!("`milestone create` names its id; got:\n{created}"))
        .to_owned();
    let record = created
        .lines()
        .find_map(|line| line.strip_prefix("record: "))
        .and_then(|rest| rest.split_whitespace().next())
        .unwrap_or_else(|| panic!("`milestone create` names its record; got:\n{created}"))
        .to_owned();
    let mut subs: Vec<(String, &str, &str)> = REPORTERS
        .iter()
        .map(|(workflow, intent, title)| {
            let ack = corpus.jigc_ok(&[
                "milestone",
                "add-task",
                &milestone,
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
    corpus.jigc_ok(&["milestone", "provision", &milestone]);
    let executed = corpus.jigc_ok(&["milestone", "execute", &milestone]);
    let spans = spawn_spans(&executed);
    assert_eq!(
        spans.len(),
        REPORTERS.len(),
        "one `Spawn:` line per sub-task; got:\n{executed}",
    );

    subs.sort();
    if let Filing::Reverse = filing {
        subs.reverse();
    }
    let shim_bin = install_jigc_shim(&corpus.home());
    let mut filed: Vec<(&str, String, String, &str)> = Vec::new();
    for (sub, workflow, title) in &subs {
        let span = spans
            .iter()
            .find(|span| span.ends_with(&format!(" --task {sub}")))
            .unwrap_or_else(|| panic!("a `Spawn:` line for `{sub}`; got:\n{executed}"));
        let out = run_span(&corpus.repo(), &corpus.home(), &shim_bin, span);
        assert!(
            out.status.success(),
            "the span `{span}` runs at exit 0; {}",
            text(&out),
        );
        let composed = Composed {
            task: sub.clone(),
            text: String::from_utf8(out.stdout).expect("utf-8 composed text"),
            cwd: corpus.repo().join(".jigc/worktrees").join(sub),
        };
        let violations = sub_task_violations(
            &composed.text,
            sub,
            workflow,
            &milestone,
            &derived(workflow, squash),
        );
        assert!(
            violations.is_empty(),
            "the `{workflow}` sub-task `{sub}` composes no step of its derived omission set, \
             no per-task door and no commit-doc write, and names the milestone's boundary; \
             violations {violations:#?} in:\n{}",
            composed.text,
        );
        let (ty, address) = match *workflow {
            "report-jigc-feedback" => (
                "jigc-feedback",
                file_jigc_feedback(&corpus, &composed, title),
            ),
            _ => ("inconsistency", file_inconsistency(&corpus, &composed)),
        };
        filed.push((ty, address, sub.clone(), title));
    }

    // The join: one commit, every filed doc and the record.
    let before = commit_count(&corpus);
    let out = corpus.jigc(&["milestone", "finalize", &milestone]);
    assert_eq!(
        out.status.code(),
        Some(0),
        "the join lands every report; {}",
        text(&out)
    );
    assert_eq!(commit_count(&corpus), before + 1, "exactly one commit");

    // The two feedback reporters minted one id; the join keeps it for the lower task id and
    // suffixes the higher one, each doc carrying its own reporter's title.
    let mut feedback: Vec<(String, String, &str)> = filed
        .iter()
        .filter(|(ty, ..)| *ty == "jigc-feedback")
        .map(|(_, address, sub, title)| (sub.clone(), address.clone(), *title))
        .collect();
    feedback.sort();
    assert_eq!(
        feedback[0].1, feedback[1].1,
        "the premise: both feedback reporters minted one id; got {feedback:?}",
    );
    let mut landed_ids: Vec<(String, String)> = Vec::new();
    for (n, (sub, address, title)) in feedback.iter().enumerate() {
        let id = if n == 0 {
            address.clone()
        } else {
            format!("{address}-{}", n + 1)
        };
        let row = listed_row(&corpus, "jigc-feedback", &id);
        assert_eq!(
            row["title"].as_str(),
            Some(*title),
            "`{id}` is `{sub}`'s finding — by task id, filed {filing:?}; row:\n{row:#}",
        );
        landed_ids.push(("jigc-feedback".to_owned(), id));
    }
    landed_ids.extend(
        filed
            .iter()
            .filter(|(ty, ..)| *ty == "inconsistency")
            .map(|(ty, address, ..)| ((*ty).to_owned(), address.clone())),
    );

    let mut expected: Vec<String> = landed_ids
        .iter()
        .map(|(ty, id)| {
            listed_row(&corpus, ty, id)["path"]
                .as_str()
                .expect("a row has a path")
                .to_owned()
        })
        .chain([record])
        .collect();
    expected.sort();
    let mut head = head_files(&corpus);
    head.sort();
    assert_eq!(
        head,
        expected,
        "the join's commit holds exactly every filed doc and the milestone record; {}",
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
        let want: BTreeSet<&str> = landed_ids
            .iter()
            .filter(|(t, _)| t == ty)
            .map(|(_, id)| id.as_str())
            .collect();
        assert_eq!(listed, want, "`doc list {ty}` lists every landed doc");
    }

    assert_eq!(
        corpus.git(&["status", "--porcelain", "--", "docs"]),
        "",
        "the landed docs on disk are the committed bytes",
    );
    let docs = HOMES
        .iter()
        .flat_map(|home| {
            let dir = corpus.repo().join(home);
            fs::read_dir(&dir)
                .unwrap_or_else(|e| panic!("read {}: {e}", dir.display()))
                .map(|entry| {
                    let path = entry.expect("a dir entry").path();
                    let rel = path
                        .strip_prefix(corpus.repo())
                        .expect("under the repo")
                        .to_string_lossy()
                        .into_owned();
                    (rel, fs::read_to_string(&path).expect("read a landed doc"))
                })
                .collect::<Vec<_>>()
        })
        .collect();
    Landed { corpus, head, docs }
}

/// Flow D: three reporters, one join, by task id in either filing order, and the output
/// adoptable as the seed is.
#[cfg(unix)]
#[test]
fn several_reporters_land_through_one_join_by_task_id_in_either_filing_order() {
    let by_id = walk(Filing::ByTaskId);
    let reverse = walk(Filing::Reverse);
    assert_eq!(
        by_id.docs.keys().collect::<Vec<_>>(),
        reverse.docs.keys().collect::<Vec<_>>(),
        "the same docs land whatever order the sub-tasks filed in",
    );
    for (path, bytes) in &by_id.docs {
        assert_eq!(
            Some(bytes),
            reverse.docs.get(path),
            "`{path}` is byte-identical whatever order the sub-tasks filed in",
        );
    }
    assert_eq!(by_id.head, reverse.head, "the join commits the same paths");
    assert_eq!(
        by_id.docs.len(),
        REPORTERS.len(),
        "every reporter's doc landed"
    );

    assert_adoptable(&by_id.corpus.repo().join("docs"));
}

/// **Red — the discriminating control.** Each report workflow composed as an **ordinary**
/// task fails every composition check of the walk: the per-task door, the commit-doc writes,
/// every derived member's own text, and the milestone trailer.
#[test]
fn an_ordinary_report_task_fails_every_sub_task_composition_check() {
    let corpus = TrialCorpus::build(State::Fresh);
    let squash = squash(&corpus);
    for workflow in ["report-jigc-feedback", "report-inconsistency"] {
        let composed = start(&corpus, workflow, &format!("compose {workflow} as a task"));
        let members = derived(workflow, squash);
        let failed: BTreeSet<String> = sub_task_violations(
            &composed.text,
            &composed.task,
            workflow,
            "file-the-findings-together",
            &members,
        )
        .into_keys()
        .collect();
        let every: BTreeSet<String> = ["per-task door", "commit-doc write", "milestone trailer"]
            .into_iter()
            .map(str::to_owned)
            .chain(members.iter().map(|member| format!("step:{member}")))
            .collect();
        assert_eq!(
            failed, every,
            "an ordinary `{workflow}` task fails every sub-task check; got:\n{}",
            composed.text,
        );
    }
}
