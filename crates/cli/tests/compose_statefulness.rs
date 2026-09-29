//! M43 Increment 7, T3 (B3+B4) — the composed statefulness lines + the finalize
//! step names what's-left.
//!
//! Measured before the fix: a minted compose hands the agent a stateful loop —
//! the task id scopes every subsequent write, `jigc start --task <id>` recovers
//! the workflow after context loss, and `jigc task validate <id>` previews what
//! finalize will gate on — but **no composed surface states any of it**. The
//! affordances exist; nothing names them (`design/surface-contract.md` → law 2:
//! resume and what's-left are named by the surfaces producing the state; the B3
//! statement: `--task` is an explicit override over a single-active-task default).
//!
//! The fix is **presentation**, in the CLI frontend (`render.rs` → `composed`):
//! three task-state lines join the `task minted:` / `create-gates:` /
//! `ROUTING_FOOTER` mold on agent/human text —
//!
//!   resume: `jigc start --task <id>`
//!   what's-left: `jigc task validate <id>`
//!   task scope: … single active task … `--task <id>` … explicit …
//!
//! — keyed on the **task id's presence** (they state standing affordances of the
//! active-task state, unlike the `task minted:` header, which states an
//! invocation fact), so a resume carries them too and only the id-less contexts
//! (the router) stay inert. **And** the finalize step's composed text itself
//! names `jigc task validate` (B4's pack half), so the what's-left affordance
//! also survives into the id-carrying re-compose surfaces.
//!
//! The pinned composed-output JSON contract (`design/command-output-contract.md`
//! §1) stays **exactly** `{task, text}` — the lines are presentation and must
//! never reach the JSON projection.
//!
//! Everything here drives the **real binary** and asserts the **emitted bytes**.

use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

/// The one-line routing footer every agent-text composed view ends with
/// (`crates/cli/src/render.rs` → `ROUTING_FOOTER`, verbatim).
const ROUTING_FOOTER: &str =
    "— jigc · run `jigc start` for orientation; all writes through `jigc`.";

/// A throwaway directory that removes itself on drop.
struct TempDir(PathBuf);

impl TempDir {
    fn new(tag: &str) -> Self {
        let mut path = std::env::temp_dir();
        let unique = format!(
            "jigc-compose-statefulness-{tag}-{}-{:?}",
            std::process::id(),
            engine::tempname::unique_nanos(),
        );
        path.push(unique);
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

/// The embedded dev pack tree on disk — selected via `JIGC_PACK_DIR` so the binary
/// composes the exact bytes it ships.
fn dev_pack() -> PathBuf {
    Path::new(cli::pack_path!(dev)).to_path_buf()
}

/// The methodology pack tree at the repo root — the second shipped pack, whose
/// `finalize` step owes the same what's-left sentence.
fn methodology_pack() -> PathBuf {
    Path::new(cli::pack_path!(methodology)).to_path_buf()
}

/// Initialize a real git repo with one commit plus the `.jigc/config/` project layer.
fn init_repo(root: &Path) {
    let git = |args: &[&str]| {
        let out = Command::new("git")
            .args(args)
            .current_dir(root)
            .output()
            .expect("run git");
        assert!(
            out.status.success(),
            "git {args:?} failed: {}",
            String::from_utf8_lossy(&out.stderr)
        );
    };
    git(&["init", "-q"]);
    git(&["config", "user.email", "test@example.com"]);
    git(&["config", "user.name", "Test"]);
    fs::write(root.join("README.md"), "hello\n").expect("write file");
    git(&["add", "."]);
    git(&["commit", "-q", "-m", "initial"]);
    fs::create_dir_all(root.join(".jigc").join("config")).expect("create project layer");
}

/// Run a `jigc` subcommand with `cwd = repo`, `$HOME = home`, `JIGC_PACK_DIR = pack`.
fn run_jigc(repo: &Path, home: &Path, pack: &Path, args: &[&str]) -> std::process::Output {
    Command::new(env!("CARGO_BIN_EXE_jigc"))
        .args(args)
        .current_dir(repo)
        .env("HOME", home)
        .env("JIGC_PACK_DIR", pack)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()
        .expect("run the jigc binary")
}

/// The stdout of a successful `jigc` invocation, or a panic carrying both streams.
fn ok_stdout(out: std::process::Output, what: &str) -> String {
    assert!(
        out.status.success(),
        "`{what}` must exit 0; stdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr),
    );
    String::from_utf8(out.stdout).expect("utf-8 stdout")
}

/// A fresh repo with the adapter installed — the composing ground for every arm.
fn ground(repo: &Path, home: &Path, pack: &Path) {
    init_repo(repo);
    ok_stdout(run_jigc(repo, home, pack, &["setup"]), "jigc setup");
}

/// Assert the three task-state lines are present for `id`, and sit in the
/// presentation stack (below the composed text, above the routing footer).
fn assert_task_state_lines(composed: &str, id: &str, surface: &str) {
    let resume = format!("resume: `jigc start --task {id}`");
    let whats_left = format!("what's-left: `jigc task validate {id}`");
    for needle in [resume.as_str(), whats_left.as_str(), "task scope:"] {
        assert!(
            composed.contains(needle),
            "{surface} must carry the `{needle}` line \
             (`design/surface-contract.md` → law 2); got:\n{composed}",
        );
    }
    // The B3 statement: `--task` is explicit; writes default to the single
    // active task; explicit wins.
    let scope_line = composed
        .lines()
        .find(|l| l.starts_with("task scope:"))
        .unwrap_or_else(|| panic!("{surface} carries a task-scope line; got:\n{composed}"));
    assert!(
        scope_line.contains("single active task")
            && scope_line.contains(&format!("--task {id}"))
            && scope_line.contains("wins"),
        "the task-scope line states the single-active-task default and that the \
         explicit `--task` wins; got:\n{scope_line}",
    );
    // Presentation stack order: the lines sit after the composed text's finalize
    // step and before the footer.
    let footer_at = composed
        .find(ROUTING_FOOTER)
        .unwrap_or_else(|| panic!("{surface} ends with the routing footer; got:\n{composed}"));
    let resume_at = composed.find(&resume).expect("resume line present");
    assert!(
        resume_at < footer_at,
        "the task-state lines precede the footer; got:\n{composed}",
    );
}

/// The minting arm (B3+B4): a work-minting compose names resume, what's-left, and
/// the `--task` scoping contract — the standing affordances of the state it just
/// produced — as presentation lines beside the `task minted:` header's mold.
#[test]
fn a_minted_compose_names_resume_whats_left_and_task_scope() {
    let repo = TempDir::new("mint");
    let home = TempDir::new("home");
    let pack = dev_pack();
    ground(repo.path(), home.path(), &pack);

    let composed = ok_stdout(
        run_jigc(
            repo.path(),
            home.path(),
            &pack,
            &[
                "--format",
                "agent",
                "start",
                "--workflow",
                "single-task",
                "add a rate limiter",
            ],
        ),
        "jigc start --workflow single-task",
    );

    assert_task_state_lines(&composed, "add-a-rate-limiter", "a minted compose");

    // Human renders the same lines.
    let human = ok_stdout(
        run_jigc(
            repo.path(),
            home.path(),
            &pack,
            &[
                "--format",
                "human",
                "start",
                "--workflow",
                "single-task",
                "add a cache layer",
            ],
        ),
        "jigc --format human start --workflow single-task",
    );
    assert_task_state_lines(&human, "add-a-cache-layer", "a human-format minted compose");
}

/// The omitting arm: the router (`creates-task: false`) composes with **no task
/// id** — there is no state to resume, validate, or scope, so the feature is
/// inert. No line renders, never a placeholder.
#[test]
fn a_router_compose_carries_no_task_state_lines() {
    let repo = TempDir::new("router");
    let home = TempDir::new("home");
    let pack = dev_pack();
    ground(repo.path(), home.path(), &pack);

    let composed = ok_stdout(
        run_jigc(
            repo.path(),
            home.path(),
            &pack,
            &["--format", "agent", "start", "add a rate limiter"],
        ),
        "jigc start \"<intent>\"",
    );

    for needle in ["resume: `jigc start --task", "what's-left:", "task scope:"] {
        assert!(
            !composed.contains(needle),
            "the router composes no task — the `{needle}` line must be omitted \
             (inert, never an error); got:\n{composed}",
        );
    }
    assert!(
        composed.trim_end().ends_with(ROUTING_FOOTER),
        "the composed view still ends with the routing footer; got:\n{composed}",
    );
}

/// The resume arm: the lines state **standing affordances of the active-task
/// state**, not invocation facts — unlike `task minted:`, they are as true (and as
/// needed: a resume happens exactly when context was lost) on a re-compose, so the
/// id-carrying resume surface carries them too.
#[test]
fn a_resume_carries_the_standing_affordances() {
    let repo = TempDir::new("resume");
    let home = TempDir::new("home");
    let pack = dev_pack();
    ground(repo.path(), home.path(), &pack);

    ok_stdout(
        run_jigc(
            repo.path(),
            home.path(),
            &pack,
            &["start", "--workflow", "single-task", "add a rate limiter"],
        ),
        "jigc start --workflow single-task",
    );

    let resumed = ok_stdout(
        run_jigc(
            repo.path(),
            home.path(),
            &pack,
            &["--format", "agent", "start", "--task", "add-a-rate-limiter"],
        ),
        "jigc start --task add-a-rate-limiter",
    );

    assert!(
        !resumed.contains("task minted:"),
        "a resume still announces no mint; got:\n{resumed}",
    );
    assert_task_state_lines(&resumed, "add-a-rate-limiter", "a resume");
}

/// The B4 pack half + the pinned contract: the composed **finalize step text**
/// itself names `jigc task validate <id>` (what's-left survives into the `text`
/// every format carries), and `--format json` stays **exactly** `{task, text}` —
/// no new key, and the presentation lines never leak into the `text` bytes.
#[test]
fn the_finalize_step_names_task_validate_and_the_json_contract_holds() {
    let repo = TempDir::new("json");
    let home = TempDir::new("home");
    let pack = dev_pack();
    ground(repo.path(), home.path(), &pack);

    let out = ok_stdout(
        run_jigc(
            repo.path(),
            home.path(),
            &pack,
            &[
                "--format",
                "json",
                "start",
                "--workflow",
                "single-task",
                "add a rate limiter",
            ],
        ),
        "jigc --format json start --workflow single-task",
    );
    let value: serde_json::Value = serde_json::from_str(&out).expect("valid JSON");
    let object = value.as_object().expect("an object");

    let mut keys: Vec<&str> = object.keys().map(String::as_str).collect();
    keys.sort_unstable();
    assert_eq!(
        keys,
        ["task", "text"],
        "the composed-output contract is pinned at exactly `{{task, text}}` \
         (`design/command-output-contract.md` §1) — the task-state lines are \
         presentation and add NO key; got:\n{out}",
    );

    let text = object["text"].as_str().expect("`text` is a string");
    // B4: the finalize step's own composed prose names the what's-left command,
    // with the id resolved — so the affordance rides `text` in every format.
    assert!(
        text.contains("jigc task validate add-a-rate-limiter"),
        "the composed finalize step text names `jigc task validate <id>` \
         (`design/surface-contract.md` → law 2); got:\n{text}",
    );
    // The presentation lines never reach the JSON projection.
    for needle in [
        "resume: `jigc start --task",
        "what's-left: `jigc",
        "task scope:",
    ] {
        assert!(
            !text.contains(needle),
            "the task-state lines are frontend presentation — `text` must not \
             carry `{needle}`; got:\n{text}",
        );
    }
    assert!(!text.contains(ROUTING_FOOTER));
}

/// The methodology pack's finalize step owes the same what's-left sentence — the
/// second shipped pack is not the un-named sibling (the M42 census lesson).
#[test]
fn the_methodology_finalize_step_names_task_validate() {
    let repo = TempDir::new("methodology");
    let home = TempDir::new("home");
    let pack = methodology_pack();
    ground(repo.path(), home.path(), &pack);

    let out = ok_stdout(
        run_jigc(
            repo.path(),
            home.path(),
            &pack,
            &[
                "--format",
                "json",
                "start",
                "--workflow",
                "dev-task",
                "add a rate limiter",
            ],
        ),
        "jigc --format json start --workflow dev-task (methodology)",
    );
    let value: serde_json::Value = serde_json::from_str(&out).expect("valid JSON");
    let text = value["text"].as_str().expect("`text` is a string");
    assert!(
        text.contains("jigc task validate add-a-rate-limiter"),
        "the methodology finalize step's composed text names `jigc task validate \
         <id>` too; got:\n{text}",
    );
}

/// Run a `jigc` subcommand with `cwd = at`, `$HOME = home`, and **no**
/// `JIGC_PACK_DIR` — the marker-composed `[dev ▸ methodology]` set a normal
/// `jigc setup` installs. The milestone arm below needs both shipped packs at once:
/// `jigc milestone create` writes the methodology pack's `milestone-record`, while
/// the sub-task's minting workflow (`sub-task`) is the dev pack's, so the state is
/// reachable in neither pack alone.
fn run_jigc_composed(at: &Path, home: &Path, args: &[&str]) -> std::process::Output {
    Command::new(env!("CARGO_BIN_EXE_jigc"))
        .args(args)
        .current_dir(at)
        .env("HOME", home)
        .env_remove("JIGC_PACK_DIR")
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()
        .expect("run the jigc binary")
}

/// The leading backticked command of a rendered affordance line
/// (`` <label>: `<cmd>`   — <gloss> ``) — the **emitted** span, lifted verbatim.
fn backticked(line: &str) -> &str {
    line.split('`')
        .nth(1)
        .unwrap_or_else(|| panic!("an affordance line carries a backticked command; got: {line}"))
}

/// M51 Increment 9, T1 — a **milestone sub-task**'s `resume:` line names the door
/// that provisions its working area, not one that composes over an area that is not
/// there (`design/surface-contract.md` → law 1; `design/worked-examples.md` → the
/// fan-out, which already states that `jigc workflow <W> --task <sub>` *"provisions
/// its write-ready area on first entry"*).
///
/// **Driven before the fix**, on exactly this sequence: `jigc start --task <sub>` from
/// the provisioned worktree exits **0** while `.jigc/tasks/<sub>/docs/` does not
/// exist — so the composed body's own `jigc doc set-field commit:<sub>#type …` line
/// answers *"no staged instance … task `<sub>`'s workflow provisions its `commit` doc
/// at compose"*, a sentence the door that just composed had falsified, and its route
/// `jigc doc list --task <sub>` prints an empty index. The `resume:` line pointed the
/// reader back through that same non-provisioning door.
///
/// The arm drives the **emitted artifact**: it lifts the backticked span out of the
/// composed `resume:` line and runs *that*, from the worktree the line names, then
/// asserts the sub-task's `commit` doc is staged and writable.
#[test]
fn a_sub_tasks_resume_names_the_door_that_provisions_its_area() {
    let repo = TempDir::new("sub-task-resume");
    let home = TempDir::new("home");
    init_repo(repo.path());
    ok_stdout(
        run_jigc_composed(repo.path(), home.path(), &["setup"]),
        "jigc setup",
    );

    ok_stdout(
        run_jigc_composed(
            repo.path(),
            home.path(),
            &["milestone", "create", "rework the cache"],
        ),
        "jigc milestone create",
    );
    ok_stdout(
        run_jigc_composed(
            repo.path(),
            home.path(),
            &["milestone", "add-task", "rework-the-cache", "tune eviction"],
        ),
        "jigc milestone add-task",
    );
    ok_stdout(
        run_jigc_composed(
            repo.path(),
            home.path(),
            &["milestone", "provision", "rework-the-cache"],
        ),
        "jigc milestone provision",
    );

    let worktree = repo
        .path()
        .join(".jigc")
        .join("worktrees")
        .join("tune-eviction");
    assert!(
        worktree.is_dir(),
        "`milestone provision` must cut the sub-task's worktree at {worktree:?}",
    );
    let docs = repo
        .path()
        .join(".jigc")
        .join("tasks")
        .join("tune-eviction")
        .join("docs");
    assert!(
        !docs.exists(),
        "the state under test is the *unprovisioned* area; got a docs dir at {docs:?}",
    );

    // The compose the sub-agent actually meets: from its own worktree, at exit 0.
    let composed = ok_stdout(
        run_jigc_composed(
            &worktree,
            home.path(),
            &["--format", "agent", "start", "--task", "tune-eviction"],
        ),
        "jigc start --task tune-eviction (from the worktree)",
    );
    let resume = composed
        .lines()
        .find(|l| l.starts_with("resume:"))
        .unwrap_or_else(|| panic!("a sub-task compose carries a resume line; got:\n{composed}"));

    // Law 1: the named door is the one that provisions, and the line says where it
    // runs — the worktree, which the `Spawn:` line names and this one did not.
    assert_eq!(
        backticked(resume),
        "jigc workflow sub-task --task tune-eviction",
        "a sub-task's resume names the re-entry door that provisions its area; got:\n{resume}",
    );
    assert!(
        resume.contains(".jigc/worktrees/tune-eviction"),
        "the resume line names the worktree the sub-task's work happens in; got:\n{resume}",
    );

    // Follow the **emitted** span, verbatim, from the worktree the line names.
    let argv: Vec<&str> = backticked(resume).split_whitespace().collect();
    assert_eq!(
        argv.first(),
        Some(&"jigc"),
        "the span runs the binary itself"
    );
    ok_stdout(
        run_jigc_composed(&worktree, home.path(), &argv[1..]),
        "the emitted resume span",
    );

    // ...and it left the sub-task's `commit` doc staged, so the composed body's own
    // write lines now land instead of answering `no staged instance`.
    let commit_doc = docs.join("commit:tune-eviction.md");
    assert!(
        commit_doc.is_file(),
        "following the resume span provisions the sub-task's commit doc at {commit_doc:?}",
    );
    ok_stdout(
        run_jigc_composed(
            &worktree,
            home.path(),
            &[
                "doc",
                "set-field",
                "commit:tune-eviction#type",
                "--value",
                "feat",
                "--task",
                "tune-eviction",
            ],
        ),
        "jigc doc set-field commit:tune-eviction#type",
    );
}

/// M52 Increment 10, T4 (the per-axis review's A6-1) — a **milestone sub-task**'s
/// `resume:` line discriminates the **posture it is composed in**
/// (`design/surface-contract.md` → law 1: nothing lies).
///
/// Measured at the M52 baseline (`completions/artifacts/M52/baseline-surfaces.md` §2.1):
/// the sub-task compose is **byte-identical** in both provisioned postures — inside the
/// sub-task's own worktree, and in the shared checkout standing at the milestone's pin —
/// because `render::task_state_lines` keys on `Composition::sub_task_of` alone and has no
/// posture value in scope. So the one line shipped, *"run it from this sub-task's own
/// worktree at `.jigc/worktrees/<id>`"*, tells the reader who is **already standing in
/// that worktree** to go somewhere it already is, and says nothing to the reader in the
/// shared checkout about being in the wrong one.
///
/// The arm drives the **real binary** through the door the fan-out's sub-agents take
/// (`jigc workflow <W> --task <sub>`) from both provisioned postures, and asserts the two
/// emitted `resume:` lines **differ** and each names its own posture — while the
/// backticked span, which is the same door in both, stays the same.
#[test]
fn a_sub_tasks_resume_line_discriminates_the_posture_it_is_composed_in() {
    let repo = TempDir::new("sub-task-posture");
    let home = TempDir::new("home");
    init_repo(repo.path());
    ok_stdout(
        run_jigc_composed(repo.path(), home.path(), &["setup"]),
        "jigc setup",
    );
    ok_stdout(
        run_jigc_composed(
            repo.path(),
            home.path(),
            &["milestone", "create", "rework the cache"],
        ),
        "jigc milestone create",
    );
    ok_stdout(
        run_jigc_composed(
            repo.path(),
            home.path(),
            &["milestone", "add-task", "rework-the-cache", "tune eviction"],
        ),
        "jigc milestone add-task",
    );
    ok_stdout(
        run_jigc_composed(
            repo.path(),
            home.path(),
            &["milestone", "provision", "rework-the-cache"],
        ),
        "jigc milestone provision",
    );

    let worktree = repo
        .path()
        .join(".jigc")
        .join("worktrees")
        .join("tune-eviction");
    assert!(
        worktree.is_dir(),
        "`milestone provision` must cut the sub-task's worktree at {worktree:?}",
    );

    // The shared checkout must stand **at the milestone's pin** to reach the second
    // provisioned posture — `milestone create` / `add-task` each commit their record, so
    // the branch has moved past it. The pin is what the worktree was cut at, so git
    // itself answers for it rather than a second read of the milestone's base file.
    let pin = String::from_utf8(
        Command::new("git")
            .args(["rev-parse", "HEAD"])
            .current_dir(&worktree)
            .output()
            .expect("run git rev-parse in the sub-task worktree")
            .stdout,
    )
    .expect("git prints utf-8");
    let detach = Command::new("git")
        .args(["checkout", "--detach", pin.trim()])
        .current_dir(repo.path())
        .output()
        .expect("run git checkout --detach");
    assert!(
        detach.status.success(),
        "the shared checkout must reach the milestone's pin; stderr:\n{}",
        String::from_utf8_lossy(&detach.stderr),
    );

    let argv = [
        "--format",
        "agent",
        "workflow",
        "sub-task",
        "--task",
        "tune-eviction",
    ];
    let in_worktree = ok_stdout(
        run_jigc_composed(&worktree, home.path(), &argv),
        "jigc workflow sub-task --task tune-eviction (from the sub-task's worktree)",
    );
    let in_shared = ok_stdout(
        run_jigc_composed(repo.path(), home.path(), &argv),
        "jigc workflow sub-task --task tune-eviction (from the shared checkout at the pin)",
    );

    let resume_of = |composed: &str| -> String {
        composed
            .lines()
            .find(|l| l.starts_with("resume:"))
            .unwrap_or_else(|| panic!("a sub-task compose carries a resume line; got:\n{composed}"))
            .to_string()
    };
    let worktree_resume = resume_of(&in_worktree);
    let shared_resume = resume_of(&in_shared);

    // The door is the same in both postures — only what the line says about *where the
    // reader is standing* may differ.
    assert_eq!(
        backticked(&worktree_resume),
        "jigc workflow sub-task --task tune-eviction",
        "the re-entry door is the same in both postures; got:\n{worktree_resume}",
    );
    assert_eq!(
        backticked(&shared_resume),
        backticked(&worktree_resume),
        "the re-entry door is the same in both postures; got:\n{shared_resume}",
    );

    // The claim: the two lines are not the same bytes — the baseline measured them
    // byte-identical.
    assert_ne!(
        worktree_resume, shared_resume,
        "the sub-task resume line must discriminate the posture it is composed in",
    );

    // ...and each names *its own* posture, not the other's.
    assert!(
        worktree_resume.contains("you are in this sub-task's worktree")
            && worktree_resume.contains(".jigc/worktrees/tune-eviction"),
        "composed inside the worktree, the line says the reader is already there; \
         got:\n{worktree_resume}",
    );
    assert!(
        !worktree_resume.contains("run it from this sub-task's own worktree"),
        "composed inside the worktree, the line must not send the reader where it already \
         stands; got:\n{worktree_resume}",
    );
    assert!(
        shared_resume.contains("this checkout is not that worktree")
            && shared_resume.contains(
                "run it from this sub-task's own worktree at `.jigc/worktrees/tune-eviction`"
            ),
        "composed outside it, the line names the worktree to run in; got:\n{shared_resume}",
    );
}
