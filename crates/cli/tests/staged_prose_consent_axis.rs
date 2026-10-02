//! M50 Increment 3 / T2 — **the destroying-door consent set, driven over one workbench**
//! (`completions/artifacts/M50/settle-record.md` → D1; `design/team-ready-state.md` → The
//! lifecycle · `jigc task discard <sub-id>`; `design/project-setup.md` → the two states
//! `uninstall` refuses).
//!
//! Driven at `954ce0f`, both halves on one state: `jigc uninstall` exited **1** with
//! `blocking · uninstall.staged-prose` over an open task's staged docs and removed nothing,
//! while `jigc task discard <that same id>` exited **0** and removed those very bytes —
//! two doors answering opposite ways about the identical files, with the refusing one's own
//! route pointing at the destroying one. That is not two bugs; it is one probe missing from
//! one door, which is why M50's D1 settles it as a *shared* guard rather than a second copy
//! ([`cli::task::staged_task_prose`], landed in T1).
//!
//! **The four arms are the consent axis, not the reported repro:**
//!
//! - **(a) the contradiction, closed.** Both doors refuse the *same* staged bytes, each
//!   under **its own** code — `uninstall.staged-prose` names the install it will not remove,
//!   `task-discard.staged-prose` names the task it will not throw away — each carrying
//!   exactly one route, and each route naming `--force`, the single consent. The staged
//!   files are byte-intact after both refusals.
//! - **(b) the consent performs its own act.** `--force` is not one behaviour: `uninstall
//!   --force` removes `.jigc/`; `task discard --force` removes that one working area and
//!   still acks the dropped set with its transient marks. A shared guard must not collapse
//!   two doors into one.
//! - **(c) the guard is keyed on staged bytes, not on being a task.** A `milestone add-task`
//!   sub-task stages nothing at mint, so it discards at exit 0 **without** `--force` and its
//!   committed record still settles — the omitting context, which is where a blanket
//!   refusal would show.
//! - **(d) the stated behaviour change, asserted rather than discovered.** `jigc start`
//!   itself stages `commit:<task>.md`, so from the moment a task is minted its ordinary
//!   discard needs the consent. That is bigger than D1's rationale anticipated and it is
//!   pinned here, on purpose, so a later narrowing of the predicate reddens instead of
//!   quietly re-opening the pair's disagreement (`DECISIONS.md` → 2026-09-05 M50 Increment 3
//!   planning, declared bounds (i) and (ii)).
//!
//! **The M50 completion audit added the third door** (finding 4). `jigc milestone discard`
//! read the *same* probe for **narration** and never as a **guard**, so it destroyed a
//! sub-task's staged prose at exit 0 while `jigc task discard` refused over the identical
//! bytes — D1's own contradiction, one door over. Its warrant was that *"the refusal it does
//! carry is the worktree one"* (`design/team-ready-state.md` → The workbench is actually
//! removed), and that clause is **false in the only cell where bytes die**: an agent
//! authoring through jigc writes into `.jigc/tasks/<sub-id>/docs/`, which is **not inside the
//! worktree**, so `git status --porcelain` reads clean over authored prose. The arms below
//! are the `{staged prose} × {dirty worktree}` 2×2's cells that this door owns, plus the
//! machine-surface fact the audit's scope item 6 turned on:
//!
//! - **(e) cell D — staged prose, clean worktree.** The loss cell. The door refuses under
//!   **its own** code `milestone.staged-prose`, names its sub-task's staged identity, does
//!   not name a task outside the milestone, and takes nothing.
//! - **(f) the consent performs this door's act** — and `--force` never suppresses the
//!   narration M46 pinned, so the loss is still named as it is taken.
//! - **(g) the guard is keyed on staged bytes, not on being a milestone.** `milestone
//!   add-task` and `milestone provision` stage nothing, so a milestone abandoned before
//!   anyone re-entered a sub-task still discards at exit 0 with no consent — the omitting
//!   context that makes this guard safe rather than a `--force` trainer.
//! - **(h) the loss is moved, not withheld.** The narration is a *computed* side channel, so
//!   `text_json_parity_axis`' shared *"nothing is computed, printed, and withheld"* was false
//!   of this verb; it now carries its own disposition and cites this arm, which drives the
//!   forced abandon under `--format json` and finds one undiluted JSON value on stdout with
//!   the loss on stderr — `milestone execute`'s shipped stream-discipline shape.
//!
//! Cells B and C of that 2×2 (dirty worktree, prose or not) are unchanged and stay where
//! they already live: `milestone_discard.rs` and `flow49_acceptance.rs`. The new guard sits
//! **after** the worktree one inside the same `!force` block, so where both hold the door
//! still answers `milestone.dirty-worktree`.

use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

/// A throwaway directory that removes itself on drop.
struct TempDir(PathBuf);

impl TempDir {
    fn new(tag: &str) -> Self {
        let mut path = std::env::temp_dir();
        path.push(format!(
            "jigc-staged-prose-consent-{tag}-{}-{:?}",
            std::process::id(),
            engine::tempname::unique_nanos(),
        ));
        fs::create_dir_all(&path).expect("create temp dir");
        TempDir(path)
    }

    fn path(&self) -> &Path {
        &self.0
    }
}

impl Drop for TempDir {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}

/// Run `git <args>` in `repo`, asserting success.
fn git(repo: &Path, args: &[&str]) {
    let out = Command::new("git")
        .args(args)
        .current_dir(repo)
        .output()
        .expect("run git");
    assert!(
        out.status.success(),
        "git {args:?} failed: {}",
        String::from_utf8_lossy(&out.stderr),
    );
}

/// Run `jigc <args>` with `cwd = repo` and `$HOME = home`, against the embedded packs.
fn jigc(repo: &Path, home: &Path, args: &[&str]) -> std::process::Output {
    Command::new(env!("CARGO_BIN_EXE_jigc"))
        .args(args)
        .current_dir(repo)
        .env("HOME", home)
        .env_remove("JIGC_PACK_DIR")
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("spawn the jigc binary")
        .wait_with_output()
        .expect("wait for jigc")
}

/// Run `jigc <args>` asserting exit 0, returning stdout.
fn ok(repo: &Path, home: &Path, args: &[&str], what: &str) -> String {
    let out = jigc(repo, home, args);
    assert!(
        out.status.success(),
        "`{what}` must exit 0; stdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr),
    );
    String::from_utf8_lossy(&out.stdout).into_owned()
}

/// Both streams of a run — a refusal's identity and its route are rendered on stderr,
/// the acks on stdout, and no arm here cares which.
fn both_streams(out: &std::process::Output) -> String {
    format!(
        "{}{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr),
    )
}

/// The `route:` lines of a rendered findings surface — the route floor's unit, so an arm
/// can assert *exactly one* rather than "contains a route somewhere".
fn route_lines(text: &str) -> Vec<String> {
    text.lines()
        .filter(|line| line.trim_start().starts_with("route:"))
        .map(|line| line.trim().to_string())
        .collect()
}

/// The top-level task: minted by `jigc start`, which stages `commit:<id>.md` at mint.
const TOP_INTENT: &str = "Fix the retry cap";
const TOP_TASK: &str = "fix-the-retry-cap";

/// The milestone and its sub-task — the cell that stages nothing.
const MILESTONE_TITLE: &str = "Cache rework";
const MILESTONE_ID: &str = "cache-rework";
const SUB_INTENT: &str = "Warm the read cache";
const SUB_TASK: &str = "warm-the-read-cache";

/// The prose an operator typed into the staged commit doc — bytes no commit has a copy of,
/// which is the whole subject of both guards.
const AUTHORED: &str = "Cap the retry budget at three attempts.";

/// **The one workbench state every arm drives**: a real install (`jigc setup`), a milestone
/// carrying one sub-task that stages nothing, and one `jigc start`-minted top-level task
/// whose staged commit doc carries authored prose.
fn workbench(tag: &str) -> (TempDir, TempDir) {
    let repo = TempDir::new(tag);
    let home = TempDir::new(&format!("{tag}-home"));
    git(repo.path(), &["init", "-q"]);
    git(repo.path(), &["config", "user.email", "test@example.com"]);
    git(repo.path(), &["config", "user.name", "Test"]);
    fs::write(repo.path().join("README.md"), "hello\n").expect("write README");
    git(repo.path(), &["add", "."]);
    git(repo.path(), &["commit", "-q", "-m", "initial"]);

    ok(repo.path(), home.path(), &["setup"], "jigc setup");
    ok(
        repo.path(),
        home.path(),
        &["milestone", "create", MILESTONE_TITLE],
        "milestone create",
    );
    ok(
        repo.path(),
        home.path(),
        &["milestone", "add-task", MILESTONE_ID, SUB_INTENT],
        "milestone add-task",
    );
    ok(
        repo.path(),
        home.path(),
        &["start", "--workflow", "quick-fix", TOP_INTENT],
        "start --workflow quick-fix",
    );
    let write = Command::new(env!("CARGO_BIN_EXE_jigc"))
        .args([
            "doc",
            "set-slot",
            &format!("commit:{TOP_TASK}#summary"),
            "--task",
            TOP_TASK,
            "--from-file",
            "-",
        ])
        .current_dir(repo.path())
        .env("HOME", home.path())
        .env_remove("JIGC_PACK_DIR")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .and_then(|mut child| {
            crate::support::child_stdin::feed(&mut child, AUTHORED.as_bytes());
            child.wait_with_output()
        })
        .expect("run doc set-slot");
    assert!(
        write.status.success(),
        "the authored slot must land; stderr:\n{}",
        String::from_utf8_lossy(&write.stderr),
    );
    (repo, home)
}

/// The staged commit doc of `task` in `repo`.
fn staged_doc(repo: &Path, task: &str) -> PathBuf {
    repo.join(".jigc")
        .join("tasks")
        .join(task)
        .join("docs")
        .join(format!("commit:{task}.md"))
}

/// **Arm (a)** — one state, both doors, one shared probe: each refuses the same staged
/// bytes under its own code, with exactly one route, and each route names the consent.
#[test]
fn both_doors_refuse_the_same_staged_bytes_each_under_its_own_code() {
    let (repo, home) = workbench("pair");
    let (repo, home) = (repo.path(), home.path());
    let staged = staged_doc(repo, TOP_TASK);
    let before = fs::read(&staged).expect("the staged commit doc is on disk");
    assert!(
        String::from_utf8_lossy(&before).contains(AUTHORED),
        "the fixture's authored prose is in the staged bytes",
    );

    // The install door.
    let refused_install = jigc(repo, home, &["uninstall"]);
    let install_text = both_streams(&refused_install);
    assert!(
        !refused_install.status.success(),
        "`jigc uninstall` must refuse over an open task's staged docs; got:\n{install_text}",
    );
    assert!(
        install_text.contains("blocking · uninstall.staged-prose"),
        "the install door refuses under its own code; got:\n{install_text}",
    );
    let install_routes = route_lines(&install_text);
    assert_eq!(
        install_routes.len(),
        1,
        "the install refusal carries exactly one route; got:\n{install_text}",
    );
    assert!(
        install_routes[0].contains("jigc uninstall --force"),
        "the install door's route names its own consent; got:\n{}",
        install_routes[0],
    );

    // The task door — the same bytes, the other door.
    let refused_discard = jigc(repo, home, &["task", "discard", TOP_TASK]);
    let discard_text = both_streams(&refused_discard);
    assert!(
        !refused_discard.status.success(),
        "`jigc task discard` must refuse over its own staged docs; got:\n{discard_text}",
    );
    assert!(
        discard_text.contains("blocking · task-discard.staged-prose"),
        "the task door refuses under its OWN code, never the install door's — the two \
         doors name different subjects; got:\n{discard_text}",
    );
    let discard_routes = route_lines(&discard_text);
    assert_eq!(
        discard_routes.len(),
        1,
        "the task refusal carries exactly one route; got:\n{discard_text}",
    );
    assert!(
        discard_routes[0].contains(&format!("jigc task discard {TOP_TASK} --force")),
        "the task door's route names its own consent, substituted with the real id; got:\n{}",
        discard_routes[0],
    );

    // Both refusals name the same staged identity — one probe, one subject.
    for text in [&install_text, &discard_text] {
        assert!(
            text.contains(&format!("commit:{TOP_TASK}")),
            "each refusal names the staged doc it is refusing over; got:\n{text}",
        );
    }

    // And neither took a byte.
    assert_eq!(
        fs::read(&staged).expect("the staged commit doc survives both refusals"),
        before,
        "a refusal removes nothing",
    );
    assert!(
        repo.join(".jigc").is_dir(),
        "the refused uninstall left `.jigc/` in place",
    );
}

/// **Arm (b)** — the shared guard must not collapse the two doors: each `--force`
/// performs that door's own act.
#[test]
fn each_doors_force_performs_its_own_post_condition() {
    // The install door's consent takes the install.
    let (repo, home) = workbench("force-uninstall");
    let (repo, home) = (repo.path(), home.path());
    ok(repo, home, &["uninstall", "--force"], "uninstall --force");
    assert!(
        !repo.join(".jigc").exists(),
        "`jigc uninstall --force` removes `.jigc/`",
    );

    // The task door's consent takes one working area — and still says what it dropped.
    let (repo, home) = workbench("force-discard");
    let (repo, home) = (repo.path(), home.path());
    let ack = ok(
        repo,
        home,
        &["task", "discard", TOP_TASK, "--force"],
        "task discard --force",
    );
    assert!(
        ack.contains(&format!("commit:{TOP_TASK} (transient)")),
        "the forced discard still acks the dropped set with its transient marks; got:\n{ack}",
    );
    assert!(
        !repo.join(".jigc").join("tasks").join(TOP_TASK).exists(),
        "`jigc task discard --force` removes the working area",
    );
    assert!(
        repo.join(".jigc").is_dir(),
        "and takes nothing else — the install is untouched",
    );
}

/// **Arm (c)** — the omitting context: a sub-task that stages nothing discards at exit 0
/// with no consent, and its committed record still settles.
#[test]
fn a_sub_task_with_nothing_staged_discards_without_the_consent() {
    let (repo, home) = workbench("sub-task");
    let (repo, home) = (repo.path(), home.path());
    assert!(
        !repo
            .join(".jigc")
            .join("tasks")
            .join(SUB_TASK)
            .join("docs")
            .exists(),
        "the premise: `milestone add-task` stages nothing at mint",
    );

    ok(
        repo,
        home,
        &["task", "discard", SUB_TASK],
        "task discard <sub-id> with nothing staged",
    );
    assert!(
        !repo.join(".jigc").join("tasks").join(SUB_TASK).exists(),
        "the sub-task's area is gone",
    );

    let record: serde_json::Value = serde_json::from_str(&ok(
        repo,
        home,
        &[
            "doc",
            "show",
            &format!("milestone-record:{MILESTONE_ID}"),
            "--format",
            "json",
        ],
        "doc show milestone-record --format json",
    ))
    .expect("the pinned read contract parses");
    let settled: Vec<(&str, &str)> = record["sections"]["tasks"]
        .as_array()
        .expect("the record's `tasks` section")
        .iter()
        .map(|item| {
            (
                item["task-id"].as_str().expect("item `task-id`"),
                item["status"].as_str().expect("item `status`"),
            )
        })
        .collect();
    assert_eq!(
        settled,
        vec![(SUB_TASK, "discarded")],
        "the record settles the abandoned sub-task rather than going on calling it active",
    );
}

/// **Arm (d)** — the stated behaviour change: `jigc start` stages the commit doc itself, so
/// an immediate discard of a freshly minted task needs the consent.
#[test]
fn a_freshly_minted_task_needs_the_consent_from_the_moment_it_exists() {
    let (repo, home) = workbench("mint");
    let (repo, home) = (repo.path(), home.path());
    ok(
        repo,
        home,
        &[
            "start",
            "--workflow",
            "record-decision",
            "Adopt the retry cap",
        ],
        "start --workflow record-decision",
    );
    let minted = "adopt-the-retry-cap";
    assert!(
        staged_doc(repo, minted).exists(),
        "the premise: the mint itself stages `commit:{minted}.md`",
    );

    let refused = jigc(repo, home, &["task", "discard", minted]);
    let text = both_streams(&refused);
    assert!(
        !refused.status.success(),
        "an ordinary discard of a just-minted task refuses — the intended behaviour change, \
         not an accident of the fixture; got:\n{text}",
    );
    assert!(
        text.contains("blocking · task-discard.staged-prose")
            && text.contains(&format!("commit:{minted}")),
        "and it names the pristine skeleton it will not destroy without consent; got:\n{text}",
    );
    assert!(
        repo.join(".jigc").join("tasks").join(minted).is_dir(),
        "nothing was removed",
    );

    // The consent it printed runs as printed.
    ok(
        repo,
        home,
        &["task", "discard", minted, "--force"],
        "the printed consent",
    );
    assert!(
        !repo.join(".jigc").join("tasks").join(minted).exists(),
        "and it takes the area",
    );
}

/// The milestone door's own refusal code — its own, never the task door's
/// (`task-discard.staged-prose`) or the install door's (`uninstall.staged-prose`).
const MILESTONE_STAGED_PROSE: &str = "blocking · milestone.staged-prose";

/// Put the milestone's sub-task into **cell D**: provision its worktree, then compose its
/// recorded workflow *from that worktree* — which is exactly how a fanned sub-agent reaches
/// its area, and it stages `commit:<sub-id>.md` while leaving the worktree itself clean.
///
/// Driving the mint rather than writing the file is the point of the cell: the bytes the
/// door destroys are the ones the product itself put there, and the worktree probe reads
/// clean over them because `.jigc/tasks/<sub-id>/docs/` is **not inside the worktree**.
fn enter_sub_task(repo: &Path, home: &Path) -> PathBuf {
    ok(
        repo,
        home,
        &["milestone", "provision", MILESTONE_ID],
        "milestone provision",
    );
    let worktree = repo.join(".jigc").join("worktrees").join(SUB_TASK);
    let composed = jigc(
        &worktree,
        home,
        &["workflow", "sub-task", "--task", SUB_TASK],
    );
    assert!(
        composed.status.success(),
        "composing the sub-task's recorded workflow from its worktree must exit 0; \
         stdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&composed.stdout),
        String::from_utf8_lossy(&composed.stderr),
    );
    assert!(
        staged_doc(repo, SUB_TASK).exists(),
        "the premise of cell D: re-entering the sub-task stages `commit:{SUB_TASK}.md`",
    );
    // …and the worktree the *other* guard probes is clean, so cell D is genuinely reached
    // rather than shadowed by `milestone.dirty-worktree`.
    let status = Command::new("git")
        .args(["status", "--porcelain"])
        .current_dir(&worktree)
        .output()
        .expect("run git status in the sub-task worktree");
    assert!(
        String::from_utf8_lossy(&status.stdout).trim().is_empty(),
        "the premise of cell D: the worktree is CLEAN — the worktree refusal structurally \
         cannot see prose staged outside it; got:\n{}",
        String::from_utf8_lossy(&status.stdout),
    );
    worktree
}

/// **Arm (e)** — cell D of the 2×2: staged prose, clean worktree. The abandon refuses under
/// its own code, names its own sub-task's staged doc, names no task it would not touch, and
/// takes nothing.
#[test]
fn the_milestone_door_refuses_its_sub_tasks_staged_bytes_under_its_own_code() {
    let (repo, home) = workbench("milestone-cell-d");
    let (repo, home) = (repo.path(), home.path());
    enter_sub_task(repo, home);
    let staged = staged_doc(repo, SUB_TASK);
    let before = fs::read(&staged).expect("the sub-task's staged commit doc is on disk");

    let refused = jigc(repo, home, &["milestone", "discard", MILESTONE_ID]);
    let text = both_streams(&refused);
    assert!(
        !refused.status.success(),
        "`jigc milestone discard` must refuse over a sub-task's staged docs, as \
         `jigc task discard` does over the identical bytes; got:\n{text}",
    );
    assert!(
        text.contains(MILESTONE_STAGED_PROSE),
        "the abandon door refuses under its OWN code — a shared identity would put the wrong \
         door's re-run in front of the operator; got:\n{text}",
    );
    let routes = route_lines(&text);
    assert_eq!(
        routes.len(),
        1,
        "the refusal carries exactly one route; got:\n{text}",
    );
    for exit in [
        "jigc doc show <address> --task <sub-task-id>".to_string(),
        format!("jigc milestone finalize {MILESTONE_ID}"),
        format!("jigc milestone discard {MILESTONE_ID} --force"),
    ] {
        assert!(
            routes[0].contains(&exit),
            "the route names its three exits — read, land, consent — each at THIS door's \
             unit kind (`jigc task finalize` would be the wrong one: a sub-task's only \
             commit boundary is the milestone's); `{exit}` is missing from:\n{}",
            routes[0],
        );
    }
    assert!(
        text.contains(&format!("{SUB_TASK}: commit:{SUB_TASK}")),
        "the refusal names the sub-task and the staged identity it will not destroy — which \
         is what makes the route's `<sub-task-id>` placeholder substitutable; got:\n{text}",
    );
    assert!(
        !text.contains(TOP_TASK),
        "and names nothing outside the milestone — this door never removes an unrelated \
         open task's area, so claiming it would be a destruction it does not perform; \
         got:\n{text}",
    );

    // Nothing moved: the bytes, the workbench, and the record are all as they were.
    assert_eq!(
        fs::read(&staged).expect("the staged doc survives the refusal"),
        before,
        "a refusal removes nothing",
    );
    assert!(
        repo.join(".jigc")
            .join("milestones")
            .join(MILESTONE_ID)
            .is_dir(),
        "the milestone workbench is untouched",
    );
    let record: serde_json::Value = serde_json::from_str(&ok(
        repo,
        home,
        &[
            "doc",
            "show",
            &format!("milestone-record:{MILESTONE_ID}"),
            "--format",
            "json",
        ],
        "doc show milestone-record --format json",
    ))
    .expect("the pinned read contract parses");
    assert_eq!(
        record["fields"]["status"].as_str(),
        Some("active"),
        "the refusal ran before the settle — the record must not claim the milestone was \
         abandoned; got:\n{record}",
    );
}

/// **Arm (f)** — the consent performs *this* door's act, and it does not buy silence:
/// `--force` still narrates the loss it takes (M46 pinned the narration deliberately).
#[test]
fn the_milestone_doors_force_takes_the_workbench_and_still_narrates_the_loss() {
    let (repo, home) = workbench("milestone-force");
    let (repo, home) = (repo.path(), home.path());
    enter_sub_task(repo, home);

    let forced = jigc(
        repo,
        home,
        &["milestone", "discard", MILESTONE_ID, "--force"],
    );
    let stderr = String::from_utf8_lossy(&forced.stderr).into_owned();
    assert!(
        forced.status.success(),
        "the printed consent runs as printed; stdout:\n{}\nstderr:\n{stderr}",
        String::from_utf8_lossy(&forced.stdout),
    );
    assert!(
        stderr.contains(&format!("{SUB_TASK}: commit:{SUB_TASK}"))
            && stderr.contains("not recoverable"),
        "`--force` is consent, never silence — the narration survives it verbatim; \
         stderr:\n{stderr}",
    );
    assert!(
        !repo.join(".jigc").join("tasks").join(SUB_TASK).exists(),
        "the forced abandon really takes the sub-task area (visible AND consented)",
    );
    assert!(
        staged_doc(repo, TOP_TASK).exists(),
        "and takes only its own sub-tasks — the unrelated open task's prose survives",
    );
}

/// **Arm (g)** — the omitting context, and the property that makes this guard safe: the
/// milestone doors that *precede* authoring stage nothing, so a milestone abandoned before
/// anyone re-entered a sub-task discards at exit 0 with no consent at all.
///
/// Without this cell the guard would be indistinguishable from one that fires on every
/// abandon — which is the `--force`-into-reflex failure M46 refused on measured evidence.
#[test]
fn a_milestone_whose_sub_tasks_stage_nothing_discards_without_the_consent() {
    let (repo, home) = workbench("milestone-bare");
    let (repo, home) = (repo.path(), home.path());
    ok(
        repo,
        home,
        &["milestone", "provision", MILESTONE_ID],
        "milestone provision",
    );
    assert!(
        !repo
            .join(".jigc")
            .join("tasks")
            .join(SUB_TASK)
            .join("docs")
            .exists(),
        "the premise: `milestone add-task` and `milestone provision` stage nothing",
    );

    let discarded = jigc(repo, home, &["milestone", "discard", MILESTONE_ID]);
    let text = both_streams(&discarded);
    assert!(
        discarded.status.success(),
        "a milestone holding no staged prose must still abandon with no consent; got:\n{text}",
    );
    assert!(
        !text.contains("milestone.staged-prose"),
        "and the guard must stay silent where it has nothing to refuse over; got:\n{text}",
    );
    assert!(
        !repo
            .join(".jigc")
            .join("milestones")
            .join(MILESTONE_ID)
            .exists(),
        "the abandon really ran",
    );
    assert!(
        staged_doc(repo, TOP_TASK).exists(),
        "and left the unrelated open task's staged prose alone",
    );
}

/// **The parity disposition's citation** (M50 completion audit, finding 4, scope item 6).
///
/// `crates/cli/tests/text_json_parity_axis.rs` dispositioned `milestone discard` as
/// *"nothing is computed, printed, and withheld"*. Driven, the loss narration **is**
/// computed and printed and is not in the envelope — so the rationale was corrected rather
/// than the envelope grown (the pre-1.0 additive-key window closed at M48): the warning
/// rides **stderr** under `--format json` exactly as `milestone execute`'s partial-provision
/// advisory does, so the pinned document stays one JSON value and the driver still receives
/// the loss on its own stream.
#[test]
fn the_forced_abandons_loss_narration_rides_stderr_under_format_json() {
    let (repo, home) = workbench("milestone-json");
    let (repo, home) = (repo.path(), home.path());
    enter_sub_task(repo, home);

    let forced = jigc(
        repo,
        home,
        &[
            "milestone",
            "discard",
            MILESTONE_ID,
            "--force",
            "--format",
            "json",
        ],
    );
    let stdout = String::from_utf8_lossy(&forced.stdout).into_owned();
    let stderr = String::from_utf8_lossy(&forced.stderr).into_owned();
    assert!(
        forced.status.success(),
        "the forced abandon exits 0; stdout:\n{stdout}\nstderr:\n{stderr}",
    );

    let envelope: serde_json::Value =
        serde_json::from_str(&stdout).expect("stdout is ONE pinned JSON value, undiluted");
    assert!(
        envelope["text"]
            .as_str()
            .expect("`text` carries the whole agent summary")
            .contains("workbench removed"),
        "the summary rides `text` whole, as the shared disposition says; got:\n{stdout}",
    );
    assert!(
        !stdout.contains(&format!("commit:{SUB_TASK}")),
        "the loss is NOT a key of the envelope — the additive-key window closed at M48; \
         got:\n{stdout}",
    );
    assert!(
        stderr.contains(&format!("{SUB_TASK}: commit:{SUB_TASK}")),
        "…and it is not withheld either: stream discipline puts it on stderr under the \
         machine format too; got:\n{stderr}",
    );
}

/// **Arm (i)** — a **directory** named `<type>:<slug>.md` under a task's `docs/` is a staged
/// identity at no surface (M52 Increment 4 / T1, defect L-3 —
/// `completions/artifacts/M52/baseline-destroying.md` §4).
///
/// `cli::task::staged_doc_ids` is the one probe every one of these surfaces reads, and it
/// asked a **name** question — `strip_suffix(".md")` — and no shape question at all. Driven
/// at `4572ca7c` with `mkdir '.jigc/tasks/<id>/docs/fake:thing.md'`: `jigc task discard`
/// refused naming *"stages 1 doc(s) … : fake:thing"* and routed at
/// `jigc doc show fake:thing --task <id>`, which dead-ends at `store.unknown-type`; the
/// forced discard acked *"dropped staged edits to: fake:thing"*; and `jigc doc list --task`
/// — a **1.0-pinned** contract — listed it `managed`. Four surfaces claiming a staged doc
/// that no doc read can open.
///
/// Low severity in bytes (the door **over**-refused, so nothing died) and a law-1 claim the
/// bytes do not support on a pinned surface, which is why it is pinned here rather than left
/// to the guard that owns it. **The subject boundary is the point:** what a non-file entry
/// there *is* — a byte jigc did not write into a task area — is the destroying doors'
/// question about foreign bytes, not this probe's question about staged prose, and a probe
/// that answers the wrong one of those answers it wrongly.
///
/// The real staged doc is planted beside it as the control: an arm that passed by the
/// surfaces going quiet would prove nothing.
#[test]
fn a_directory_named_like_a_staged_doc_is_an_identity_at_no_surface() {
    let (repo, home) = workbench("shaped-like-a-doc");
    let (repo, home) = (repo.path(), home.path());
    let squatter = repo
        .join(".jigc")
        .join("tasks")
        .join(TOP_TASK)
        .join("docs")
        .join("fake:thing.md");
    fs::create_dir(&squatter).expect("plant a directory shaped like a staged doc");
    let real = format!("commit:{TOP_TASK}");

    // The pinned index read.
    let listed = jigc(repo, home, &["doc", "list", "--task", TOP_TASK]);
    let listed_text = both_streams(&listed);
    assert!(
        listed.status.success(),
        "`jigc doc list --task` must still answer; got:\n{listed_text}",
    );
    assert!(
        listed_text.contains(&real),
        "the control: the task's real staged doc is still listed; got:\n{listed_text}",
    );
    assert!(
        !listed_text.contains("fake:thing"),
        "a directory is not a staged doc, and the 1.0-pinned index read must not call it \
         `managed`; got:\n{listed_text}",
    );

    // The refusal. Since T5 the subject boundary this arm names is *performed* by the
    // binary: a non-file entry under `docs/` is a byte jigc did not write, so the door that
    // answers it is the foreign-byte guard, naming the **path**, and not the staged-prose
    // probe naming an **identity** `jigc doc show` cannot open.
    let refused = jigc(repo, home, &["task", "discard", TOP_TASK]);
    let refused_text = both_streams(&refused);
    let as_path = format!(".jigc/tasks/{TOP_TASK}/docs/fake:thing.md");
    assert!(
        !refused.status.success(),
        "the door still refuses over it; got:\n{refused_text}",
    );
    assert!(
        refused_text.contains("blocking · task-discard.foreign-bytes")
            && refused_text.contains(&as_path),
        "…under the code whose subject it really is, naming it as a path; got:\n{refused_text}",
    );
    assert!(
        !refused_text.replace(&as_path, "").contains("fake:thing"),
        "and NOWHERE as a staged identity — the claim `jigc doc show` cannot honour and \
         whose route dead-ends at `store.unknown-type`; got:\n{refused_text}",
    );

    // The ack. The consent takes it, and the two streams keep the two claims apart: the
    // stdout ack says what staged docs it dropped; the stderr narration says what foreign
    // bytes it took.
    let dropped = jigc(repo, home, &["task", "discard", TOP_TASK, "--force"]);
    let ack = String::from_utf8_lossy(&dropped.stdout).into_owned();
    let narration = String::from_utf8_lossy(&dropped.stderr).into_owned();
    assert!(
        dropped.status.success(),
        "the consent performs the discard; got:\n{ack}{narration}",
    );
    assert!(
        ack.contains(&real),
        "the control: the ack still names the staged doc it dropped; got:\n{ack}",
    );
    assert!(
        !ack.contains("fake:thing"),
        "the ack must not claim it dropped staged edits to a directory; got:\n{ack}",
    );
    assert!(
        narration.contains(&as_path),
        "…and the bytes are not withheld either: the door names what it took, as a path; \
         got:\n{narration}",
    );
}

// ---------------------------------------------------------------------------------------
// M52 Increment 4 / T5 — **the foreign-byte consent axis**: the same three doors, over the
// OTHER population of a working area (`settle-record.md` → D3.2 as amended by §6/§8/§14;
// `design/team-ready-state.md` → The working area's two populations).
//
// A working area holds two populations: the files jigc wrote (`engine::state`'s registry,
// T2) and everything else — bytes an agent or a human put there, in **no commit, no index
// and no git object at all**, since the whole tree is gitignored. Driven at `4f311cb1`, all
// three doors took that second population whole, at exit 0, named by nothing: `jigc task
// discard` removed a `NOTES.md`, an `analysis/perf.txt` and a `docs/notes.txt`; `jigc
// milestone discard` did the same across every sub-task area *and* its own milestone area;
// and `jigc uninstall` took all of it plus everything parked under `.jigc/displaced/`.
//
// The arms below are the consent axis of the **staged-prose** arms above, one subject over —
// `{refuse without --force, narrate with it}` × the three doors — plus the `displaced/` row
// (§8) and the zero-false-fire control T2's registry earns.
// ---------------------------------------------------------------------------------------

/// The operator's own bytes: in no commit, no index, and no git object — `.jigc/` is
/// gitignored whole, so the working area is their only copy.
const MINE: &str = "the operator's own bytes — nothing else has a copy\n";

/// Plant the done-criterion's three foreign cells in `area`: a file at the area root, one a
/// directory deep, and one beside the staged prose under `docs/`.
///
/// Returns the paths **as the complement names them**: a foreign *directory* is one entry,
/// moved and removed whole, so `analysis/perf.txt` is named by its `analysis` parent (the
/// unit T3's displacement primitive already established). The bytes are asserted separately.
fn plant_foreign(area: &Path) -> Vec<String> {
    fs::create_dir_all(area.join("docs")).expect("the docs dir");
    fs::write(area.join("NOTES.md"), MINE).expect("plant the root file");
    fs::create_dir_all(area.join("analysis")).expect("plant the nested dir");
    fs::write(area.join("analysis").join("perf.txt"), MINE).expect("plant the nested file");
    fs::write(area.join("docs").join("notes.txt"), MINE).expect("plant the docs file");
    vec![
        "NOTES.md".to_string(),
        "analysis".to_string(),
        "docs/notes.txt".to_string(),
    ]
}

/// The three planted files, by absolute path — what "every byte intact" is asserted over.
fn planted_bytes(area: &Path) -> Vec<PathBuf> {
    vec![
        area.join("NOTES.md"),
        area.join("analysis").join("perf.txt"),
        area.join("docs").join("notes.txt"),
    ]
}

/// Assert every planted byte is still exactly where it was, with its content.
fn assert_intact(area: &Path, what: &str) {
    for path in planted_bytes(area) {
        assert_eq!(
            fs::read_to_string(&path)
                .unwrap_or_else(|err| panic!("{path:?} survives {what}: {err}")),
            MINE,
            "{what} must not touch a byte jigc did not write",
        );
    }
}

/// **T5 arm (j)** — the task door refuses over the bytes it did not write, names every one,
/// and takes nothing.
///
/// Driven at `4f311cb1`: exit **0**, all three destroyed, named by nothing.
///
/// **The foreign subject is asked BEFORE the staged-prose one** (this fixture stages
/// `commit:<id>.md`, as every `jigc start` does). Asked second it would be inert on the
/// dominant cell — a guard that structurally cannot fire at its own door — while asked first
/// it costs the staged-prose refusal nothing on any state whose complement is empty, which
/// T2 drove to be the ordinary path.
#[test]
fn the_task_door_refuses_over_bytes_jigc_did_not_write_and_names_every_one() {
    let (repo, home) = workbench("foreign-task");
    let (repo, home) = (repo.path(), home.path());
    let area = repo.join(".jigc").join("tasks").join(TOP_TASK);
    let planted = plant_foreign(&area);

    let refused = jigc(repo, home, &["task", "discard", TOP_TASK]);
    let text = both_streams(&refused);
    assert!(
        !refused.status.success(),
        "`jigc task discard` must refuse over bytes jigc did not write; got:\n{text}",
    );
    assert!(
        text.contains("blocking · task-discard.foreign-bytes"),
        "the task door refuses under its OWN foreign-byte code; got:\n{text}",
    );
    for entry in &planted {
        assert!(
            text.contains(&format!(".jigc/tasks/{TOP_TASK}/{entry}")),
            "the refusal names every foreign entry it would take — `{entry}` is missing \
             from:\n{text}",
        );
    }
    let routes = route_lines(&text);
    assert_eq!(
        routes.len(),
        1,
        "the refusal carries exactly one route; got:\n{text}",
    );
    assert!(
        routes[0].contains(&format!("jigc task discard {TOP_TASK} --force")),
        "and the route names this door's own consent, carrying the real id; got:\n{}",
        routes[0],
    );
    assert_intact(&area, "a refusal");
    assert!(area.is_dir(), "and the working area itself survives");
}

/// **T5 arm (k)** — `--force` is consent, never silence: the door takes the foreign bytes and
/// names each one it took, keyed on the outcome.
#[test]
fn the_task_doors_force_narrates_every_foreign_path_it_took() {
    let (repo, home) = workbench("foreign-task-force");
    let (repo, home) = (repo.path(), home.path());
    let area = repo.join(".jigc").join("tasks").join(TOP_TASK);
    let planted = plant_foreign(&area);

    let forced = jigc(repo, home, &["task", "discard", TOP_TASK, "--force"]);
    let stderr = String::from_utf8_lossy(&forced.stderr).into_owned();
    assert!(
        forced.status.success(),
        "the printed consent runs as printed; stdout:\n{}\nstderr:\n{stderr}",
        String::from_utf8_lossy(&forced.stdout),
    );
    for entry in &planted {
        assert!(
            stderr.contains(&format!(".jigc/tasks/{TOP_TASK}/{entry}")),
            "`--force` narrates every foreign path it took — `{entry}` is missing \
             from:\n{stderr}",
        );
    }
    assert!(
        stderr.contains("not recoverable"),
        "and says what that costs; got:\n{stderr}",
    );
    assert!(!area.exists(), "the consented discard really took the area");
}

/// **T5 arm (l)** — the abandon door, over **both** of its area kinds: a sub-task's working
/// area and the milestone area itself. One refusal names both; `--force` narrates both.
#[test]
fn the_milestone_door_refuses_over_foreign_bytes_in_both_area_kinds() {
    let (repo, home) = workbench("foreign-milestone");
    let (repo, home) = (repo.path(), home.path());
    let sub_area = repo.join(".jigc").join("tasks").join(SUB_TASK);
    fs::create_dir_all(&sub_area).expect("the sub-task area exists from add-task");
    let planted = plant_foreign(&sub_area);
    let milestone_area = repo.join(".jigc").join("milestones").join(MILESTONE_ID);
    fs::write(milestone_area.join("scratch.txt"), MINE).expect("plant in the milestone area");

    let refused = jigc(repo, home, &["milestone", "discard", MILESTONE_ID]);
    let text = both_streams(&refused);
    assert!(
        !refused.status.success(),
        "`jigc milestone discard` must refuse over bytes jigc did not write; got:\n{text}",
    );
    assert!(
        text.contains("blocking · milestone.foreign-bytes"),
        "the abandon door refuses under its OWN foreign-byte code; got:\n{text}",
    );
    for entry in &planted {
        assert!(
            text.contains(&format!(".jigc/tasks/{SUB_TASK}/{entry}")),
            "the refusal names the sub-task area's foreign entries — `{entry}` is missing \
             from:\n{text}",
        );
    }
    assert!(
        text.contains(&format!(".jigc/milestones/{MILESTONE_ID}/scratch.txt")),
        "…and the milestone area's own, which is a SECOND registry row and was reachable \
         through no guard at all; got:\n{text}",
    );
    assert!(
        !text.contains(TOP_TASK),
        "and names nothing outside the milestone — this door removes no unrelated task's \
         area, so claiming one would be a destruction it does not perform; got:\n{text}",
    );
    let routes = route_lines(&text);
    assert_eq!(routes.len(), 1, "exactly one route; got:\n{text}");
    assert!(
        routes[0].contains(&format!("jigc milestone discard {MILESTONE_ID} --force")),
        "carrying this door's own consent; got:\n{}",
        routes[0],
    );
    assert_intact(&sub_area, "a refusal");
    assert!(
        milestone_area.join("scratch.txt").exists(),
        "and the milestone area's foreign byte too",
    );

    // The consent performs this door's act, and narrates both area kinds as it takes them.
    let forced = jigc(
        repo,
        home,
        &["milestone", "discard", MILESTONE_ID, "--force"],
    );
    let stderr = String::from_utf8_lossy(&forced.stderr).into_owned();
    assert!(
        forced.status.success(),
        "the printed consent runs as printed; stdout:\n{}\nstderr:\n{stderr}",
        String::from_utf8_lossy(&forced.stdout),
    );
    for named in [
        format!(".jigc/tasks/{SUB_TASK}/NOTES.md"),
        format!(".jigc/milestones/{MILESTONE_ID}/scratch.txt"),
    ] {
        assert!(
            stderr.contains(&named),
            "`--force` narrates what it took in both area kinds — `{named}` is missing \
             from:\n{stderr}",
        );
    }
    assert!(!sub_area.exists(), "the consented abandon took the areas");
}

/// **T5 arm (m)** — the install door, over the same bytes. `uninstall` removes `.jigc/`
/// whole, so its subject is every task area and every milestone area **on disk** — never a
/// registered list, which a copied repo does not have.
#[test]
fn the_install_door_refuses_over_foreign_bytes_under_the_workbench() {
    let (repo, home) = workbench("foreign-uninstall");
    let (repo, home) = (repo.path(), home.path());
    let area = repo.join(".jigc").join("tasks").join(TOP_TASK);
    let planted = plant_foreign(&area);

    let refused = jigc(repo, home, &["uninstall"]);
    let text = both_streams(&refused);
    assert!(
        !refused.status.success(),
        "`jigc uninstall` must refuse over bytes jigc did not write; got:\n{text}",
    );
    assert!(
        text.contains("blocking · uninstall.foreign-bytes"),
        "the install door refuses under its OWN foreign-byte code; got:\n{text}",
    );
    for entry in &planted {
        assert!(
            text.contains(&format!(".jigc/tasks/{TOP_TASK}/{entry}")),
            "naming every foreign entry the teardown would take — `{entry}` is missing \
             from:\n{text}",
        );
    }
    let routes = route_lines(&text);
    assert_eq!(routes.len(), 1, "exactly one route; got:\n{text}");
    assert!(
        routes[0].contains("jigc uninstall --force"),
        "carrying this door's own consent; got:\n{}",
        routes[0],
    );
    assert_intact(&area, "a refusal");
    assert!(repo.join(".jigc").is_dir(), "and the install survives");

    let forced = jigc(repo, home, &["uninstall", "--force"]);
    let stderr = String::from_utf8_lossy(&forced.stderr).into_owned();
    assert!(
        forced.status.success(),
        "the printed consent runs as printed; stderr:\n{stderr}",
    );
    assert!(
        stderr.contains(&format!(".jigc/tasks/{TOP_TASK}/NOTES.md")),
        "`--force` narrates what it took; got:\n{stderr}",
    );
    assert!(!repo.join(".jigc").exists(), "and takes the install");
}

/// **T5 arm (n)** — the `displaced/` row (§8). `.jigc/displaced/` is the parking home for
/// bytes jigc moved **aside rather than destroy** (`relocate::relocate_stranded`, and since
/// T3 every displacing door) — and until now `jigc uninstall` took the lot at exit 0, named
/// by nothing, because the row sits inside `gitignore::ENTRIES` and so outside
/// `uninstall.untracked-workbench-file`'s subject, while no other door owns it at all
/// (`setup.rs`' `workbench_paths` said so in as many words, as a declared gap).
///
/// The row **does not discriminate** a pre-image jigc parked from a foreign byte — both are
/// the same claim, that nothing else has a copy — which is the decided answer to review A3,
/// and why clearing it is the human's own `rm` and not a new verb.
#[test]
fn a_non_empty_displaced_workbench_refuses_the_uninstall_and_narrates_under_force() {
    let (repo, home) = workbench("displaced-row");
    let (repo, home) = (repo.path(), home.path());
    // Planted, not driven: the bytes are the same bytes either producer parks there, and
    // the row's whole point is that it does not ask which one did.
    let parked = repo
        .join(".jigc")
        .join("displaced")
        .join("tidy-the-readme")
        .join("NOTES.md");
    fs::create_dir_all(parked.parent().expect("parent")).expect("the parking home");
    fs::write(&parked, MINE).expect("park a file");

    let refused = jigc(repo, home, &["uninstall"]);
    let text = both_streams(&refused);
    assert!(
        !refused.status.success(),
        "a non-empty `.jigc/displaced/` must refuse the teardown; got:\n{text}",
    );
    assert!(
        text.contains("blocking · uninstall.foreign-bytes")
            && text.contains(".jigc/displaced/tidy-the-readme/NOTES.md"),
        "naming every entry under it; got:\n{text}",
    );
    let routes = route_lines(&text);
    assert_eq!(routes.len(), 1, "exactly one route; got:\n{text}");
    assert!(
        routes[0].contains("rm ") && routes[0].contains("jigc uninstall --force"),
        "the route names the human's own `rm` — jigc mints no verb that clears this row — \
         and the consent; got:\n{}",
        routes[0],
    );
    assert_eq!(
        fs::read_to_string(&parked).expect("the parked file survives the refusal"),
        MINE,
    );

    let forced = jigc(repo, home, &["uninstall", "--force"]);
    let stderr = String::from_utf8_lossy(&forced.stderr).into_owned();
    assert!(
        forced.status.success(),
        "the printed consent runs as printed; stderr:\n{stderr}",
    );
    assert!(
        stderr.contains(".jigc/displaced/tidy-the-readme/NOTES.md"),
        "and `--force` narrates the parked bytes it took; got:\n{stderr}",
    );
    assert!(!parked.exists(), "which it really did");
}

/// **T5 arm (o)** — T2's zero-false-fire control, re-driven at **all three doors**.
///
/// A complement guard cut one member short does not fail safe: it refuses over jigc's own
/// file and the operator cannot get past it except with `--force`, which is the consent
/// reflex M46 refused to train. T2 proved the complement empty over a full lifecycle; this
/// proves the three doors that *read* it stay silent over the same state.
#[test]
fn no_door_fires_the_foreign_guard_over_jigcs_own_files() {
    let (repo, home) = workbench("zero-false-fire");
    let (repo, home) = (repo.path(), home.path());
    // Reach the members a shorter registry would have missed — `roles.json` (the ADR create)
    // and `renames.json` (the ordinary in-task retitle) — plus the probe snapshots.
    let task = "record-a-decision-about-caching";
    ok(
        repo,
        home,
        &[
            "start",
            "--workflow",
            "record-decision",
            "record a decision about caching",
        ],
        "jigc start",
    );
    ok(
        repo,
        home,
        &[
            "doc",
            "create",
            "adr",
            "--title",
            "Cache the thing",
            "--task",
            task,
        ],
        "jigc doc create",
    );
    ok(
        repo,
        home,
        &[
            "doc",
            "rename",
            "adr:cache-the-thing",
            "--to",
            "Cache the other thing",
            "--task",
            task,
        ],
        "jigc doc rename",
    );
    jigc(repo, home, &["task", "validate", task]);
    let area = repo.join(".jigc").join("tasks").join(task);
    let on_disk: Vec<String> = fs::read_dir(&area)
        .expect("read the area")
        .map(|e| e.expect("entry").file_name().to_string_lossy().into_owned())
        .collect();
    assert!(
        on_disk.contains(&"roles.json".to_string())
            && on_disk.contains(&"renames.json".to_string()),
        "the control must actually reach `roles.json` and `renames.json`, else it proves \
         nothing; the area holds: {on_disk:?}",
    );

    for (what, argv) in [
        ("task discard", vec!["task", "discard", task]),
        (
            "milestone discard",
            vec!["milestone", "discard", MILESTONE_ID],
        ),
        ("uninstall", vec!["uninstall"]),
    ] {
        let out = jigc(repo, home, &argv);
        let text = both_streams(&out);
        assert!(
            !text.contains("foreign-bytes"),
            "`jigc {what}` must not call one of jigc's own files foreign — the area holds \
             {on_disk:?}; got:\n{text}",
        );
    }
}

/// **T5 arm (p)** — the three codes **log the identity they print**.
///
/// A door refusal joins neither `engine::result::CHECK_INVENTORY` nor the error-code
/// registry (it is not a probe result and not a commit-phase rejection), so the only place
/// it becomes legible to a driver or a trial's log analysis is the invocation record — and
/// the carrier that makes that work (`render::BlockedFinding`) is something a door has to
/// actually use. Driven here rather than assumed, for all three doors at once, alongside the
/// route-floor unit each refusal already carries.
#[test]
fn each_foreign_byte_refusal_logs_the_code_it_printed() {
    let (repo, home) = workbench("foreign-log");
    let (repo, home) = (repo.path(), home.path());
    ok(
        repo,
        home,
        &["config", "set", "invocation-log", "true"],
        "config set invocation-log true",
    );
    let sub_area = repo.join(".jigc").join("tasks").join(SUB_TASK);
    fs::create_dir_all(&sub_area).expect("the sub-task area");
    plant_foreign(&sub_area);
    plant_foreign(&repo.join(".jigc").join("tasks").join(TOP_TASK));

    let log = repo.join(".jigc").join("logs").join("invocations.jsonl");
    for (code, argv) in [
        (
            "task-discard.foreign-bytes",
            vec!["task", "discard", SUB_TASK],
        ),
        (
            "milestone.foreign-bytes",
            vec!["milestone", "discard", MILESTONE_ID],
        ),
        ("uninstall.foreign-bytes", vec!["uninstall"]),
    ] {
        let before = fs::read_to_string(&log).unwrap_or_default().lines().count();
        let out = jigc(repo, home, &argv);
        let text = both_streams(&out);
        assert!(
            !out.status.success() && text.contains(code),
            "`jigc {}` must refuse under `{code}`; got:\n{text}",
            argv.join(" "),
        );
        assert_eq!(
            route_lines(&text).len(),
            1,
            "…carrying exactly one route (the route floor's unit); got:\n{text}",
        );
        let records: Vec<serde_json::Value> = fs::read_to_string(&log)
            .expect("the invocation log")
            .lines()
            .filter(|line| !line.trim().is_empty())
            .map(|line| serde_json::from_str(line).expect("each record is JSON"))
            .collect();
        assert_eq!(
            records.len(),
            before + 1,
            "`jigc {}` appends exactly one record",
            argv.join(" "),
        );
        let codes: Vec<&str> = records.last().expect("one record")["finding_codes"]
            .as_array()
            .expect("finding_codes is an array")
            .iter()
            .map(|value| value.as_str().expect("a code is a string"))
            .collect();
        assert!(
            codes.contains(&code),
            "a refusal that names itself on the surface must name itself in the log — \
             `jigc {}` printed `{code}` and logged {codes:?}",
            argv.join(" "),
        );
    }
}

/// **T5 arm (q)** — the three codes answer on the **declared machine arm**.
///
/// The increment registers all three of them on the **findings arm**
/// (`implementation/roadmap.md` → Increment 4, *Codes it registers*), and two of them did not
/// get there: the task door and the abandon door raised their refusal through the flattened
/// `{"error": …}` carrier, so a driver that parsed `--format json` got one prose string with
/// the code inside it — and a code inside a message is not a key
/// (`design/command-output-contract.md` → The membership test). The third door, `uninstall`,
/// answered the envelope, so **one registration shipped two wire shapes**, which is the fact
/// that makes this an arm defect rather than a preference: a driver cannot discriminate the
/// family it was handed.
///
/// Driven at `43c00034`: `task discard` and `milestone discard` each emitted
/// `{"error": "blocking · <code> — …"}` with no `schema_version`, no `findings` and no `key`.
///
/// The target half of the key is **not** asserted: every code in this family is a declared
/// singleton — each door is fail-fast, so two instances of one code cannot coexist in one
/// output and there is nothing for a target to discriminate
/// (`design/command-output-contract.md` → The declared singleton exception). The shipped
/// `uninstall` cell keys at `target: null` for exactly that reason.
#[test]
fn each_foreign_byte_refusal_answers_on_the_declared_findings_arm() {
    let (repo, home) = workbench("foreign-arm");
    let (repo, home) = (repo.path(), home.path());
    let sub_area = repo.join(".jigc").join("tasks").join(SUB_TASK);
    fs::create_dir_all(&sub_area).expect("the sub-task area");
    plant_foreign(&sub_area);
    plant_foreign(&repo.join(".jigc").join("tasks").join(TOP_TASK));

    for (code, argv) in [
        (
            "task-discard.foreign-bytes",
            vec!["task", "discard", SUB_TASK, "--format", "json"],
        ),
        (
            "milestone.foreign-bytes",
            vec!["milestone", "discard", MILESTONE_ID, "--format", "json"],
        ),
        (
            "uninstall.foreign-bytes",
            vec!["uninstall", "--format", "json"],
        ),
    ] {
        let what = argv.join(" ");
        let out = jigc(repo, home, &argv);
        let stdout = String::from_utf8_lossy(&out.stdout).into_owned();
        let stderr = String::from_utf8_lossy(&out.stderr).into_owned();
        assert!(
            !out.status.success(),
            "`jigc {what}` must refuse; stdout:\n{stdout}\nstderr:\n{stderr}",
        );
        assert!(
            stdout.trim().is_empty(),
            "a reject puts nothing on stdout; `jigc {what}` printed:\n{stdout}",
        );
        let doc: serde_json::Value = serde_json::from_str(stderr.trim()).unwrap_or_else(|err| {
            panic!("`jigc {what}` must emit exactly one JSON document ({err});\n{stderr}")
        });
        assert!(
            doc.get("code").is_none() && doc.get("severity").is_none(),
            "`jigc {what}`: a reject document is never a bare `Finding` at the root; \
             got:\n{doc:#}",
        );
        assert!(
            doc.get("schema_version").is_some(),
            "`jigc {what}` answers on the declared findings arm, which carries \
             `schema_version` — the flattened `{{\"error\": …}}` arm is the one this code was \
             NOT registered on; got:\n{doc:#}",
        );
        let findings = doc["findings"]
            .as_array()
            .unwrap_or_else(|| {
                panic!("`jigc {what}`: the envelope carries `findings`; got:\n{doc:#}")
            })
            .clone();
        let hit = findings
            .iter()
            .find(|f| f["code"].as_str() == Some(code))
            .unwrap_or_else(|| {
                panic!("`jigc {what}` must carry `{code}` as a finding; got:\n{doc:#}")
            });
        assert_eq!(
            hit["key"]["code"].as_str(),
            Some(code),
            "`jigc {what}`: `{code}` projects its own `key.code`; got:\n{hit:#}",
        );
        assert_eq!(
            hit["severity"].as_str(),
            Some("blocking"),
            "`jigc {what}`: the refusal is blocking; got:\n{hit:#}",
        );
        assert!(
            hit["route"].is_object() || hit["route"].is_string(),
            "`jigc {what}`: the refusal carries its route as data; got:\n{hit:#}",
        );
    }
}
