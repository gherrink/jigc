//! M42 Increment 11, T3 — the **granted-and-unused changelog gate** becomes a
//! finalize advisory ([validation.md](../../../design/validation.md) → The
//! changelog-gate advisory, M42 Settle fork 6).
//!
//! The adoption trial finalized an unambiguously user-facing fix bundle with **no
//! changelog entry** and asked for the call: *"if it's optional, I'll skip it; if it
//! matters, gate it."* The answer is a finalize-scope **advisory** —
//! `changelog-recording.gate-granted-unused` — that **surfaces without refusing**:
//!
//!   - it keys on the **gate** (the task's workflow `allows-create`s `changelog`),
//!     never on the CLI adjudicating whether a diff is "user-facing" (that is
//!     judgment, and the determinism boundary forbids it);
//!   - its `target` is the **task id** (`task:<id>` — the work-unit ref form,
//!     [command-output-contract.md](../../../design/command-output-contract.md) → the
//!     form table): a `null` target would collapse every skipped changelog in the
//!     corpus onto one `(code, target)` key, in the very wave that fixes exactly that
//!     for the `write.*` family;
//!   - it is **tunable upward** — one project cascade line (`jigc config set
//!     validation.changelog-recording.gate-granted-unused.severity blocking`) turns
//!     the trial's *"if it matters, gate it"* into a gate, with no code change.
//!
//! Driven through the **real binary** against the embedded (shipped) packs, over the
//! granting context (`single-task`) *and* the omitting one (`implement-from-spec`,
//! which grants no changelog gate — the advisory must be inert there, never a false
//! fire), plus the created-but-unauthored middle case (a create alone is not an
//! entry).

use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

/// The advisory's finding code — the inventory row this suite pins.
const CODE: &str = "changelog-recording.gate-granted-unused";

/// A throwaway directory that removes itself on drop.
struct TempDir(PathBuf);

impl TempDir {
    fn new(tag: &str) -> Self {
        let mut path = std::env::temp_dir();
        path.push(format!(
            "jigc-changelog-gate-advisory-{tag}-{}-{:?}",
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

/// Run a `git` command in `repo`, asserting success.
fn git(repo: &Path, args: &[&str]) -> String {
    let out = Command::new("git")
        .args(args)
        .current_dir(repo)
        .output()
        .expect("run git");
    assert!(
        out.status.success(),
        "git {args:?} failed: {}",
        String::from_utf8_lossy(&out.stderr)
    );
    String::from_utf8(out.stdout)
        .expect("utf-8")
        .trim_end_matches('\n')
        .to_string()
}

/// Initialize a real git repo with one commit (a tracked `README.md`).
fn init_repo(repo: &Path) {
    git(repo, &["init", "-q"]);
    git(repo, &["config", "user.email", "test@example.com"]);
    git(repo, &["config", "user.name", "Test"]);
    fs::write(repo.join("README.md"), "hello\n").expect("write README");
    git(repo, &["add", "README.md"]);
    git(repo, &["commit", "-q", "-m", "initial"]);
}

/// Run `jigc <args>` against the **embedded** packs (the shipped bytes).
fn jigc(repo: &Path, home: &Path, args: &[&str], stdin: Option<&[u8]>) -> std::process::Output {
    let mut command = Command::new(env!("CARGO_BIN_EXE_jigc"));
    command
        .args(args)
        .current_dir(repo)
        .env("HOME", home)
        .env_remove("JIGC_PACK_DIR")
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());
    if stdin.is_some() {
        command.stdin(Stdio::piped());
    }
    let mut child = command.spawn().expect("spawn the jigc binary");
    if let Some(bytes) = stdin {
        child
            .stdin
            .take()
            .expect("stdin piped")
            .write_all(bytes)
            .expect("write stdin");
    }
    child.wait_with_output().expect("wait for jigc")
}

/// Run `jigc <args>` (no stdin), asserting exit 0 and returning stdout.
fn ok(repo: &Path, home: &Path, args: &[&str], what: &str) -> String {
    let out = jigc(repo, home, args, None);
    assert!(
        out.status.success(),
        "`{what}` must exit 0; stdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr),
    );
    String::from_utf8(out.stdout)
        .expect("utf-8 stdout")
        .trim_end_matches('\n')
        .to_string()
}

/// Fill every author-required field/slot of the provisioned commit doc.
fn fill_commit(repo: &Path, home: &Path, task: &str) {
    ok(
        repo,
        home,
        &[
            "doc",
            "set-field",
            &format!("commit:{task}#type"),
            "--value",
            "feat",
        ],
        "set-field commit#type",
    );
    ok(
        repo,
        home,
        &[
            "doc",
            "set-field",
            &format!("commit:{task}#scope"),
            "--value",
            "api",
        ],
        "set-field commit#scope",
    );
    for (leaf, prose) in [
        ("summary", &b"add the rate limiter\n"[..]),
        ("body", &b"Bound per-client request volume.\n"[..]),
    ] {
        let out = jigc(
            repo,
            home,
            &[
                "doc",
                "set-slot",
                &format!("commit:{task}#{leaf}"),
                "--from-file",
                "-",
            ],
            Some(prose),
        );
        assert!(
            out.status.success(),
            "set-slot commit#{leaf} must succeed; stderr:\n{}",
            String::from_utf8_lossy(&out.stderr),
        );
    }
}

/// Stage a real code change so the finalize has a diff to commit.
fn stage_code(repo: &Path) {
    fs::write(repo.join("limiter.txt"), "rate limiter\n").expect("write code change");
    git(repo, &["add", "limiter.txt"]);
}

/// The findings of a **landed** finalize (`--format json` on stdout, exit 0).
fn landed_findings(out: &std::process::Output, what: &str) -> Vec<serde_json::Value> {
    assert!(
        out.status.success(),
        "`{what}` must LAND (exit 0 — the advisory never refuses); stdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr),
    );
    let stdout = String::from_utf8_lossy(&out.stdout);
    let value: serde_json::Value = serde_json::from_str(&stdout)
        .unwrap_or_else(|e| panic!("`{what}` emits the pinned envelope ({e}); got:\n{stdout}"));
    value["findings"]
        .as_array()
        .unwrap_or_else(|| panic!("the envelope carries a `findings` array; got:\n{stdout}"))
        .clone()
}

/// The one finding carrying `CODE`, or `None`.
fn advisory(findings: &[serde_json::Value]) -> Option<&serde_json::Value> {
    findings.iter().find(|f| f["code"] == CODE)
}

/// Start a `single-task` (the one shipped workflow granting `{type: changelog, as:
/// change}`), returning the task id.
fn start_single_task(repo: &Path, home: &Path) -> String {
    ok(
        repo,
        home,
        &["start", "--workflow", "single-task", "add a rate limiter"],
        "jigc start --workflow single-task",
    );
    "add-a-rate-limiter".to_string()
}

/// The id of the single active task — read from the working area, so a slug-minting
/// rule (the M41 word cap) never has to be re-derived in test code.
fn active_task(repo: &Path) -> String {
    let mut ids: Vec<String> = fs::read_dir(repo.join(".jigc").join("tasks"))
        .expect("the tasks dir exists")
        .map(|entry| {
            entry
                .expect("task entry")
                .file_name()
                .to_string_lossy()
                .into_owned()
        })
        .collect();
    assert_eq!(ids.len(), 1, "exactly one active task; got {ids:?}");
    ids.pop().expect("the one task id")
}

/// A repo + home with `jigc setup` run.
fn setup(tag: &str) -> (TempDir, TempDir) {
    let repo = TempDir::new(tag);
    let home = TempDir::new(&format!("{tag}-home"));
    init_repo(repo.path());
    ok(repo.path(), home.path(), &["setup"], "jigc setup");
    (repo, home)
}

/// The granting context, gate unused: the advisory fires, keys at the **task id**,
/// carries a route — and the commit **lands** (exit 0; it surfaces, it never refuses).
#[test]
fn a_granted_but_unused_changelog_gate_advises_at_the_task_and_still_lands() {
    let (repo, home) = setup("unused");
    let task = start_single_task(repo.path(), home.path());
    fill_commit(repo.path(), home.path(), &task);
    stage_code(repo.path());

    let before: u32 = git(repo.path(), &["rev-list", "--count", "HEAD"])
        .parse()
        .unwrap();
    let out = jigc(
        repo.path(),
        home.path(),
        &["task", "finalize", &task, "--format", "json"],
        None,
    );
    let findings = landed_findings(&out, "jigc task finalize (gate granted, unused)");
    let finding = advisory(&findings).unwrap_or_else(|| {
        panic!(
            "`single-task` grants the changelog create-gate and this task authored no \
             entry — the `{CODE}` advisory must ride the envelope; got:\n{findings:#?}"
        )
    });

    assert_eq!(
        finding["severity"], "advisory",
        "the default severity is advisory — it surfaces, it never refuses",
    );
    assert_eq!(
        finding["key"]["target"],
        serde_json::json!(format!("task:{task}")),
        "the subject is the WORK UNIT: a null target would collapse every skipped \
         changelog in the corpus onto one key; got:\n{finding:#?}",
    );
    assert!(
        finding["route"].is_string(),
        "every advisory carries a route (the universal advisory-route floor); got:\n{finding:#?}",
    );

    // M43 pre-trial surface polish A6 — the landed-state option — **revised at M46
    // Inc 6 / T3**: A6 correctly saw that this advisory prints after the commit lands,
    // where the in-task `--task <id>` verbs are dead, and then offered them anyway as
    // the *"before finalize"* option, on the premise that they were live on the
    // `task validate` preview. They were not: the advisory did not preview at all
    // until T3. So the landing door now prints the landed-state form **alone** — the
    // in-task verbs moved to the doors where the working area is still open (the
    // preview, and a finalize the promoted gate refuses; `validate_previews_the_gate`
    // drives both, running every printed argv verbatim).
    let route = finding["route"].as_str().expect("route is a string");
    assert!(
        route.contains("jigc start --workflow record-change"),
        "the landing door carries the landed-state form; got: {route}"
    );
    assert!(
        !route.contains("--task"),
        "the commit has landed and `.jigc/tasks/<id>/` is gone — an in-task argv \
         printed here answers ``no task `<id>` `` at exit 1; got: {route}"
    );

    let after: u32 = git(repo.path(), &["rev-list", "--count", "HEAD"])
        .parse()
        .unwrap();
    assert_eq!(after, before + 1, "the advisory still lands the commit");
}

/// The gate used: an authored unreleased change-group silences the advisory.
#[test]
fn an_authored_unreleased_change_group_emits_no_advisory() {
    let (repo, home) = setup("authored");
    let task = start_single_task(repo.path(), home.path());

    ok(
        repo.path(),
        home.path(),
        &["doc", "create", "changelog", "--title", "Changelog"],
        "jigc doc create changelog",
    );
    let group = ok(
        repo.path(),
        home.path(),
        &[
            "doc",
            "add-item",
            "changelog:changelog#unreleased-changes",
            "--title",
            "Added",
        ],
        "jigc doc add-item (unreleased)",
    );
    let notes = jigc(
        repo.path(),
        home.path(),
        &[
            "doc",
            "set-slot",
            &format!("{group}/notes"),
            "--from-file",
            "-",
        ],
        Some(b"Per-client rate limiting at the gateway.\n"),
    );
    assert!(notes.status.success(), "set-slot notes must succeed");

    fill_commit(repo.path(), home.path(), &task);
    stage_code(repo.path());

    let out = jigc(
        repo.path(),
        home.path(),
        &["task", "finalize", &task, "--format", "json"],
        None,
    );
    let findings = landed_findings(&out, "jigc task finalize (gate used)");
    assert!(
        advisory(&findings).is_none(),
        "the task authored a changelog entry — no `{CODE}` advisory may fire; got:\n{findings:#?}",
    );
}

/// The middle case: a `doc create changelog` alone is **not an entry** — the skeleton
/// carries no item, so the advisory still fires (a create-and-abandon is exactly the
/// silent skip this check exists to surface).
#[test]
fn a_created_but_unauthored_changelog_is_not_an_entry() {
    let (repo, home) = setup("created-only");
    let task = start_single_task(repo.path(), home.path());

    ok(
        repo.path(),
        home.path(),
        &["doc", "create", "changelog", "--title", "Changelog"],
        "jigc doc create changelog",
    );
    fill_commit(repo.path(), home.path(), &task);
    stage_code(repo.path());

    let out = jigc(
        repo.path(),
        home.path(),
        &["task", "finalize", &task, "--format", "json"],
        None,
    );
    let findings = landed_findings(&out, "jigc task finalize (created, unauthored)");
    assert!(
        advisory(&findings).is_some(),
        "an empty changelog skeleton carries no entry — the `{CODE}` advisory must \
         still fire; got:\n{findings:#?}",
    );
}

/// The **omitting context**: `implement-from-spec` grants no changelog gate, so the
/// check is inert there — never a false fire, never an error.
#[test]
fn a_workflow_without_the_changelog_gate_emits_no_advisory() {
    let (repo, home) = setup("gateless");
    ok(
        repo.path(),
        home.path(),
        &[
            "start",
            "--workflow",
            "implement-from-spec",
            "build from the rate limit spec",
        ],
        "jigc start --workflow implement-from-spec",
    );
    let task = active_task(repo.path());
    fill_commit(repo.path(), home.path(), &task);
    stage_code(repo.path());

    let out = jigc(
        repo.path(),
        home.path(),
        &["task", "finalize", &task, "--format", "json"],
        None,
    );
    let findings = landed_findings(&out, "jigc task finalize (no changelog gate)");
    assert!(
        advisory(&findings).is_none(),
        "`implement-from-spec` grants no changelog create-gate — the check keys on the \
         GATE, so it must be inert here; got:\n{findings:#?}",
    );
}

/// *"If it matters, gate it"* — one project cascade line promotes the tunable key to
/// `blocking`, and the same unused gate now **blocks** finalize (no commit lands).
#[test]
fn a_project_cascade_line_promotes_the_advisory_to_a_gate() {
    let (repo, home) = setup("promoted");
    ok(
        repo.path(),
        home.path(),
        &[
            "config",
            "set",
            "validation.changelog-recording.gate-granted-unused.severity",
            "blocking",
        ],
        "jigc config set (promote to blocking)",
    );

    let task = start_single_task(repo.path(), home.path());
    fill_commit(repo.path(), home.path(), &task);
    stage_code(repo.path());

    let before: u32 = git(repo.path(), &["rev-list", "--count", "HEAD"])
        .parse()
        .unwrap();
    let out = jigc(
        repo.path(),
        home.path(),
        &["task", "finalize", &task, "--format", "json"],
        None,
    );
    assert!(
        !out.status.success(),
        "the promoted key must BLOCK finalize; stdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr),
    );
    let stdout = String::from_utf8_lossy(&out.stdout);
    let value: serde_json::Value = serde_json::from_str(&stdout)
        .unwrap_or_else(|e| panic!("blocked envelope ({e}):\n{stdout}"));
    let findings = value["findings"]
        .as_array()
        .unwrap_or_else(|| panic!("the blocked envelope carries findings; got:\n{stdout}"))
        .clone();
    let finding = advisory(&findings)
        .unwrap_or_else(|| panic!("the promoted `{CODE}` finding blocks; got:\n{findings:#?}"));
    assert_eq!(
        finding["severity"], "blocking",
        "the M6 post-pass promotes the finding through the cascade — no code change",
    );

    let after: u32 = git(repo.path(), &["rev-list", "--count", "HEAD"])
        .parse()
        .unwrap();
    assert_eq!(after, before, "a blocked finalize lands nothing");
}
