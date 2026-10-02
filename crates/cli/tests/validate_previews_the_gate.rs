//! M47 Increment 4 / T1 — **the carryover gate previews at `jigc task validate`**, and
//! `--carry-staged` gives the preview the consent the gate accepts
//! (`DECISIONS.md` → 2026-07-26 M47 Settle, Decision 1 + the cross-model review's
//! blocker 3; `design/command-output-contract.md`:321 — the exit-code table).
//!
//! Until M47 `jigc task validate` exited **0** over a state `jigc task finalize`
//! refuses at **3**: the carryover decision ran on the committing path only, so the
//! surface that every composed step calls *"see what's left before committing"*
//! could not see the wave's #1-ranked v1 gate. The preview now runs the same pure
//! [`engine::finalize::decide_carryover`] over the same mint-time snapshot — **no
//! staging, no mutation** — and the same blocking `finalize.carried-staged` per
//! carried path, keyed at the file path.
//!
//! Two invariants this suite pins alongside the fix:
//!
//! * **The new door speaks for itself** (surface-contract law 1): the finding is read
//!   from `task validate`, so a route text presuming the finalize invocation would be
//!   a lie. The preview carries the `TaskPreview` wording; the **finalize** door's
//!   bytes are pinned here verbatim and are unchanged.
//! * **`--carry-staged` joins the preview** — the cross-model review's scoping of the
//!   non-breaking argument: validate newly exits 3 only on states **default**
//!   `finalize` refuses, never on the ones `finalize --carry-staged` accepts, so a
//!   driver that always intends to carry is not handed a permanently-red preview.
//!
//! Plus the two no-fire bounds: a task with nothing pre-staged validates clean, and a
//! snapshot-less (pre-M43) task validates clean — the declared fail-open bound holds
//! at the new door exactly as it does at the committing one.
//!
//! Every arm drives the real binary and asserts on the emitted findings envelope.

use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

/// A throwaway directory that removes itself on drop.
struct TempDir(PathBuf);

impl TempDir {
    fn new(tag: &str) -> Self {
        let mut path = std::env::temp_dir();
        path.push(format!(
            "jigc-validate-preview-{tag}-{}-{:?}",
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

/// Run a `git` command in `repo`, asserting success, returning trimmed stdout.
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

/// Run `jigc <args>` with `cwd = repo`, `$HOME = home`, against the embedded packs.
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
        crate::support::child_stdin::feed(&mut child, bytes);
    }
    child.wait_with_output().expect("wait for jigc")
}

/// Run `jigc <args>` (no stdin), asserting exit 0.
fn ok(repo: &Path, home: &Path, args: &[&str], what: &str) {
    let out = jigc(repo, home, args, None);
    assert!(
        out.status.success(),
        "`{what}` must exit 0; stdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr),
    );
}

/// Run `jigc <args>` (no stdin), asserting exit 0 and returning trimmed stdout.
fn ok_stdout(repo: &Path, home: &Path, args: &[&str], what: &str) -> String {
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
        "doc set-field type",
    );
    ok(
        repo,
        home,
        &[
            "doc",
            "set-field",
            &format!("commit:{task}#scope"),
            "--value",
            "gate",
        ],
        "doc set-field scope",
    );
    let set_slot = |addr: &str, prose: &[u8]| {
        let out = jigc(
            repo,
            home,
            &["doc", "set-slot", addr, "--from-file", "-"],
            Some(prose),
        );
        assert!(out.status.success(), "set-slot {addr} must succeed");
    };
    set_slot(&format!("commit:{task}#summary"), b"preview the gate\n");
    set_slot(&format!("commit:{task}#body"), b"An M47 change.\n");
}

/// Mint a `single-task` task and do its work: write + `git add` a task edit
/// (post-mint staging — the task's own), fill the commit doc.
fn mint_and_work(repo: &Path, home: &Path, intent: &str, task: &str, edit: &str) {
    ok(
        repo,
        home,
        &["start", "--workflow", "single-task", intent],
        "jigc start",
    );
    fs::write(repo.join(edit), "pub fn work() {}\n").expect("write task edit");
    git(repo, &["add", edit]);
    fill_commit(repo, home, task);
}

const TASK: &str = "preview-the-carryover-gate";
const INTENT: &str = "preview the carryover gate";

/// The corpus every arm shares: a repo with jigc set up, an optional foreign
/// pre-mint staged plant, and a minted, worked `single-task`.
fn corpus(tag: &str, plant: bool) -> (TempDir, TempDir) {
    let repo = TempDir::new(tag);
    let home = TempDir::new(&format!("{tag}-home"));
    init_repo(repo.path());
    ok(repo.path(), home.path(), &["setup"], "jigc setup");
    if plant {
        // The foreign pre-staged adds — staged BEFORE the task exists.
        fs::write(repo.path().join("foreign-a.txt"), "not this task's work\n").expect("write a");
        fs::write(repo.path().join("foreign-b.txt"), "also not\n").expect("write b");
        git(repo.path(), &["add", "foreign-a.txt", "foreign-b.txt"]);
    }
    mint_and_work(repo.path(), home.path(), INTENT, TASK, "feature.rs");
    (repo, home)
}

/// The `findings` array of a `--format json` envelope on stdout.
fn findings(out: &std::process::Output, what: &str) -> Vec<serde_json::Value> {
    let stdout = String::from_utf8_lossy(&out.stdout);
    let value: serde_json::Value = serde_json::from_str(&stdout).unwrap_or_else(|e| {
        panic!("`{what}` emits the pinned findings envelope on stdout ({e}); got:\n{stdout}")
    });
    value["findings"]
        .as_array()
        .unwrap_or_else(|| panic!("the envelope carries a `findings` array; got:\n{stdout}"))
        .clone()
}

/// The `finalize.carried-staged` findings in an envelope.
fn carried_staged(findings: &[serde_json::Value]) -> Vec<serde_json::Value> {
    findings
        .iter()
        .filter(|f| f["code"] == "finalize.carried-staged")
        .cloned()
        .collect()
}

/// Assert the exit code, with both streams in the failure message.
fn assert_exit(out: &std::process::Output, code: i32, what: &str) {
    assert_eq!(
        out.status.code(),
        Some(code),
        "`{what}` must exit {code}; stdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr),
    );
}

/// **The preview fires**: a pre-staged-before-mint plant is *reported at
/// `jigc task validate`* rather than discovered at `jigc task finalize` — exit 3 with
/// one blocking `finalize.carried-staged` per carried path, keyed at the file path,
/// routed at both exits. The task's own post-mint staged edit never fires.
///
/// The finding speaks for **its own door** (law 1): it names the `jigc task finalize`
/// that will refuse, and never claims a finalize was run.
#[test]
fn a_pre_staged_before_mint_plant_is_reported_at_validate() {
    let (repo, home) = corpus("fires", true);
    let before = git(repo.path(), &["rev-list", "--count", "HEAD"]);

    let out = jigc(
        repo.path(),
        home.path(),
        &["task", "validate", TASK, "--format", "json"],
        None,
    );
    assert_exit(&out, 3, "jigc task validate (pre-mint staged plant)");
    let findings = findings(&out, "jigc task validate");
    let carried = carried_staged(&findings);
    assert_eq!(
        carried.len(),
        2,
        "exactly ONE finding per carried path (two foreign paths staged); got:\n{findings:#?}"
    );
    for (finding, path) in carried.iter().zip(["foreign-a.txt", "foreign-b.txt"]) {
        assert_eq!(
            finding["severity"], "blocking",
            "the preview is blocking — it previews a block; got: {finding}"
        );
        assert_eq!(
            finding["key"]["target"], path,
            "the finding keys at the carried file path; got: {finding}"
        );
        let message = finding["message"].as_str().expect("message is a string");
        let route = finding["route"].as_str().expect("route is a string");
        assert!(
            message.contains("jigc task finalize"),
            "the preview names the door that WILL refuse; got: {message}"
        );
        assert!(
            !message.contains("refusing to let"),
            "the preview never claims a finalize refused (law 1); got: {message}"
        );
        assert!(
            route.contains(&format!(" restore --staged -- {path}"))
                && route.contains("git -C /")
                && route.contains("--carry-staged"),
            "the route names both exits (unstage, or declare); got: {route}"
        );
    }
    assert!(
        !carried
            .iter()
            .any(|f| f["key"]["target"] == "feature.rs" || f["key"]["target"] == "README.md"),
        "the task's own post-mint staged edit is never carried; got:\n{findings:#?}"
    );

    // A pure reader: no commit, and the plant is still staged exactly as it was.
    assert_eq!(
        git(repo.path(), &["rev-list", "--count", "HEAD"]),
        before,
        "`task validate` commits nothing"
    );
    let staged = git(repo.path(), &["diff", "--cached", "--name-only"]);
    let mut names: Vec<&str> = staged.lines().collect();
    names.sort_unstable();
    assert!(
        names.contains(&"foreign-a.txt") && names.contains(&"foreign-b.txt"),
        "the preview stages and unstages nothing; index:\n{staged}"
    );
}

/// **`--carry-staged` gives the preview the consent the gate accepts**: the identical
/// state validates at exit 0 with no carryover finding — so a driver that always
/// intends to carry gets a preview of *its own* finalize, not a permanently-red one.
#[test]
fn carry_staged_gives_the_preview_the_consent_the_gate_accepts() {
    let (repo, home) = corpus("consent", true);

    let out = jigc(
        repo.path(),
        home.path(),
        &[
            "task",
            "validate",
            TASK,
            "--carry-staged",
            "--format",
            "json",
        ],
        None,
    );
    assert_exit(&out, 0, "jigc task validate --carry-staged");
    let findings = findings(&out, "jigc task validate --carry-staged");
    assert!(
        carried_staged(&findings).is_empty(),
        "a declared carry-over is not a finding; got:\n{findings:#?}"
    );
}

/// **No false fire**: a task with nothing pre-staged validates clean, exactly as it
/// did before the preview existed.
#[test]
fn a_task_with_nothing_pre_staged_validates_clean() {
    let (repo, home) = corpus("clean", false);

    let out = jigc(
        repo.path(),
        home.path(),
        &["task", "validate", TASK, "--format", "json"],
        None,
    );
    assert_exit(&out, 0, "jigc task validate (nothing pre-staged)");
    assert!(
        carried_staged(&findings(&out, "jigc task validate")).is_empty(),
        "post-mint staging never trips the preview"
    );
}

/// **The fail-open bound holds at the new door**: a snapshot-less task (minted before
/// the M43 gate existed) validates clean — the same declared bound `decide_carryover`
/// honours on the committing path, unchanged by the new caller.
#[test]
fn a_snapshot_less_task_validates_clean() {
    let (repo, home) = corpus("fail-open", true);

    // Reproduce the pre-M43 working area: no mint-time staged snapshot.
    let snapshot = repo
        .path()
        .join(".jigc/tasks")
        .join(TASK)
        .join("staged-snapshot.json");
    assert!(snapshot.is_file(), "the mint writes {snapshot:?}");
    fs::remove_file(&snapshot).expect("remove the snapshot");

    let out = jigc(
        repo.path(),
        home.path(),
        &["task", "validate", TASK, "--format", "json"],
        None,
    );
    assert_exit(&out, 0, "jigc task validate (snapshot-less task)");
    assert!(
        carried_staged(&findings(&out, "jigc task validate")).is_empty(),
        "a snapshot-less task fails open at the preview door too"
    );
}

/// **Finalize is untouched** — the hard invariant. Over the identical planted state
/// the committing door still blocks at exit 3 with its own verbatim message and
/// route (pinned byte-for-byte here: the preview must not have been wired through a
/// shared seam that re-words or re-positions the refusal), and
/// `finalize --carry-staged` still lands the declared carry-over in the commit.
#[test]
fn the_finalize_door_is_byte_identical() {
    let (repo, home) = corpus("finalize", true);
    let before = git(repo.path(), &["rev-list", "--count", "HEAD"]);

    let out = jigc(
        repo.path(),
        home.path(),
        &["task", "finalize", TASK, "--format", "json"],
        None,
    );
    assert_exit(&out, 3, "jigc task finalize (pre-mint staged plant)");
    let carried = carried_staged(&findings(&out, "jigc task finalize"));
    assert_eq!(carried.len(), 2, "one finding per carried path");
    for (finding, path) in carried.iter().zip(["foreign-a.txt", "foreign-b.txt"]) {
        assert_eq!(
            finding["message"],
            format!(
                "`{path}` was already staged before this task existed — refusing to let a \
                 pre-task staged change silently ride this task's commit"
            ),
            "the committing door's message is unchanged"
        );
        assert_eq!(
            finding["route"],
            format!(
                "unstage it (`git -C {home} restore --staged -- {path}`) if it is not this \
                 task's work, or re-run the finalize with `--carry-staged` to declare the \
                 carry-over deliberate",
                // The one part of the pinned bytes that is a fact about the machine: the
                // checkout the unstage runs in (M53 — the cwd census, C1-01). Everything
                // else is still compared byte for byte, which is what this arm is for.
                // Canonicalized, because the door resolves its root from `current_dir()` and
                // on macOS the fixture's `/var/…` is a symlink to `/private/var/…`.
                home = repo
                    .path()
                    .canonicalize()
                    .unwrap_or_else(|_| repo.path().to_path_buf())
                    .display(),
            ),
            "the committing door's route is unchanged"
        );
    }
    assert_eq!(
        git(repo.path(), &["rev-list", "--count", "HEAD"]),
        before,
        "the blocked finalize commits nothing"
    );

    ok(
        repo.path(),
        home.path(),
        &["task", "finalize", TASK, "--carry-staged"],
        "jigc task finalize --carry-staged",
    );
    let landed = git(repo.path(), &["show", "--name-only", "--format=", "HEAD"]);
    assert!(
        landed.lines().any(|l| l == "foreign-a.txt")
            && landed.lines().any(|l| l == "foreign-b.txt")
            && landed.lines().any(|l| l == "feature.rs"),
        "the declared carry-over still rides the whole-index commit; files:\n{landed}"
    );
}

// ---------------------------------------------------------------------------
// M46 Increment 6 / T3 — the **changelog-gate advisory joins the previewed set**,
// and each door names verbs that run at that door.
// ---------------------------------------------------------------------------

/// The changelog advisory's finding code — the member joining [`Tier::Previewed`].
const CHANGELOG_CODE: &str = "changelog-recording.gate-granted-unused";

/// The one finding carrying [`CHANGELOG_CODE`], or `None`.
fn changelog_advisory(findings: &[serde_json::Value]) -> Option<serde_json::Value> {
    findings
        .iter()
        .find(|f| f["code"] == CHANGELOG_CODE)
        .cloned()
}

/// The route text of the changelog advisory in an envelope.
fn advisory_route(findings: &[serde_json::Value], what: &str) -> String {
    changelog_advisory(findings)
        .unwrap_or_else(|| panic!("`{what}` carries the `{CHANGELOG_CODE}` advisory"))["route"]
        .as_str()
        .unwrap_or_else(|| panic!("`{what}`'s advisory carries a route string"))
        .to_owned()
}

/// Split a backticked command into argv, honouring the double-quoted intent the route
/// spells (`jigc start --workflow record-change "<what changed>"` is **three** args
/// plus the quoted one, not five).
fn argv_of(command: &str) -> Vec<String> {
    let mut argv = Vec::new();
    let mut current = String::new();
    let mut quoted = false;
    for ch in command.chars() {
        match ch {
            '"' => quoted = !quoted,
            c if c.is_whitespace() && !quoted => {
                if !current.is_empty() {
                    argv.push(std::mem::take(&mut current));
                }
            }
            c => current.push(c),
        }
    }
    if !current.is_empty() {
        argv.push(current);
    }
    argv
}

/// **The emitted argvs**, extracted from a route's backticked spans — the bytes an
/// agent would actually run, never a reconstruction in test code.
fn route_argvs(route: &str) -> Vec<Vec<String>> {
    route
        .split('`')
        .skip(1)
        .step_by(2)
        .filter(|span| span.starts_with("jigc "))
        .map(argv_of)
        .collect()
}

/// Run **every** argv the route prints, verbatim and in order — the door-truth check:
/// a route may not name a verb that is dead at the door that printed it.
///
/// Two assertions, and the split is the surface's own placeholder grammar rather than
/// a convenience. **Every** argv must reach the task: an answer of ``no task `<id>` ``
/// is the failure this task exists to kill, and it is a *door* failure — it does not
/// depend on what an author would substitute. An argv carrying **no** `<…>`
/// placeholder is additionally copy-runnable and must exit 0. A placeholder-bearing
/// one is not claimed to be runnable as printed (`--title <category>` is an enum
/// member the author picks — the CLI choosing it would be judgment inside the
/// determinism boundary), so it is held to the door check alone.
fn every_route_argv_runs(repo: &Path, home: &Path, route: &str, door: &str) {
    let argvs = route_argvs(route);
    assert!(
        !argvs.is_empty(),
        "{door}: the route prints no runnable command; got: {route}"
    );
    for argv in argvs {
        assert_eq!(argv[0], "jigc", "{door}: the route's command is `jigc`");
        let args: Vec<&str> = argv[1..].iter().map(String::as_str).collect();
        let out = jigc(repo, home, &args, None);
        let rendered = format!(
            "{}{}",
            String::from_utf8_lossy(&out.stdout),
            String::from_utf8_lossy(&out.stderr),
        );
        assert!(
            !rendered.contains("no task `"),
            "{door}: `{}` is dead at this door — it answers `no task`:\n{rendered}",
            argv.join(" "),
        );
        if !argv.iter().any(|arg| arg.contains('<')) {
            assert!(
                out.status.success(),
                "{door}: `{}` carries no placeholder, so it must run verbatim; got exit \
                 {:?}:\n{rendered}",
                argv.join(" "),
                out.status.code(),
            );
        }
    }
}

/// Author a real changelog entry in `task` — the write-touch T1 keys the advisory on.
fn record_a_change(repo: &Path, home: &Path, task: &str) {
    ok(
        repo,
        home,
        &[
            "doc",
            "create",
            "changelog",
            "--title",
            "Changelog",
            "--task",
            task,
        ],
        "jigc doc create changelog",
    );
    let group = ok_stdout(
        repo,
        home,
        &[
            "doc",
            "add-item",
            "changelog:changelog#unreleased-changes",
            "--title",
            "Added",
            "--task",
            task,
        ],
        "jigc doc add-item (unreleased)",
    );
    let notes = jigc(
        repo,
        home,
        &[
            "doc",
            "set-slot",
            &format!("{group}/notes"),
            "--from-file",
            "-",
            "--task",
            task,
        ],
        Some(b"Per-client rate limiting at the gateway.\n"),
    );
    assert!(
        notes.status.success(),
        "set-slot notes must succeed; stderr:\n{}",
        String::from_utf8_lossy(&notes.stderr),
    );
}

/// **The advisory previews.** A `single-task` grants the `changelog` create-gate; a
/// task that wrote no entry draws the advisory at `jigc task validate` — exit **0**
/// (it surfaces, it never refuses), keyed at the work unit, carrying a route.
///
/// And the route is **this door's**: the working area is open here, so the in-task
/// verbs lead — and every argv it prints runs verbatim.
#[test]
fn a_granted_but_unused_changelog_gate_is_previewed_at_validate() {
    let (repo, home) = corpus("changelog-unused", false);

    let out = jigc(
        repo.path(),
        home.path(),
        &["task", "validate", TASK, "--format", "json"],
        None,
    );
    assert_exit(&out, 0, "jigc task validate (changelog gate unused)");
    let findings = findings(&out, "jigc task validate");
    let finding = changelog_advisory(&findings).unwrap_or_else(|| {
        panic!(
            "`single-task` grants the changelog create-gate and this task recorded no \
             entry — the advisory must preview at `task validate`; got:\n{findings:#?}"
        )
    });
    assert_eq!(
        finding["severity"], "advisory",
        "the previewed member keeps its default severity; got: {finding}"
    );
    assert_eq!(
        finding["key"]["target"],
        serde_json::json!(format!("task:{TASK}")),
        "the subject is the work unit; got: {finding}"
    );

    let route = advisory_route(&findings, "jigc task validate");
    assert!(
        route.contains("jigc doc create changelog"),
        "the preview door's route leads with the in-task form — the verbs are live \
         here; got: {route}"
    );
    every_route_argv_runs(repo.path(), home.path(), &route, "the `task validate` door");
}

/// **The write-touch suppresses it at the preview door too.** A task that recorded an
/// entry draws no advisory — the same predicate the committing door reads (T1), read
/// through the new one.
#[test]
fn a_task_that_recorded_a_change_draws_no_advisory_at_validate() {
    let (repo, home) = corpus("changelog-recorded", false);
    record_a_change(repo.path(), home.path(), TASK);

    let out = jigc(
        repo.path(),
        home.path(),
        &["task", "validate", TASK, "--format", "json"],
        None,
    );
    assert_exit(&out, 0, "jigc task validate (changelog recorded)");
    let findings = findings(&out, "jigc task validate");
    assert!(
        changelog_advisory(&findings).is_none(),
        "the task recorded an entry — no advisory may fire at either door; got:\n{findings:#?}"
    );
}

/// **The omitting context stays inert at the new door.** `implement-from-spec` grants
/// no changelog gate, so the preview mints nothing — the check keys on the *gate*.
#[test]
fn a_workflow_without_the_changelog_gate_previews_no_advisory() {
    let repo = TempDir::new("changelog-gateless");
    let home = TempDir::new("changelog-gateless-home");
    init_repo(repo.path());
    ok(repo.path(), home.path(), &["setup"], "jigc setup");
    ok(
        repo.path(),
        home.path(),
        &["start", "--workflow", "implement-from-spec", INTENT],
        "jigc start --workflow implement-from-spec",
    );
    fill_commit(repo.path(), home.path(), TASK);

    let out = jigc(
        repo.path(),
        home.path(),
        &["task", "validate", TASK, "--format", "json"],
        None,
    );
    assert_exit(&out, 0, "jigc task validate (no changelog gate)");
    assert!(
        changelog_advisory(&findings(&out, "jigc task validate")).is_none(),
        "`implement-from-spec` grants no changelog create-gate — the preview is inert"
    );
}

/// **One cascade line, and the preview exits 3 on the state finalize refuses.** The
/// promoted key blocks at *both* doors — which is the whole point of previewing it —
/// and the refusing route names verbs that run while the task is open, including the
/// exit for a change that is not user-facing.
#[test]
fn the_promoted_gate_blocks_at_the_preview_and_at_finalize() {
    let repo = TempDir::new("changelog-promoted");
    let home = TempDir::new("changelog-promoted-home");
    init_repo(repo.path());
    ok(repo.path(), home.path(), &["setup"], "jigc setup");
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
    mint_and_work(repo.path(), home.path(), INTENT, TASK, "feature.rs");

    let out = jigc(
        repo.path(),
        home.path(),
        &["task", "validate", TASK, "--format", "json"],
        None,
    );
    assert_exit(&out, 3, "jigc task validate (promoted changelog gate)");
    let previewed = findings(&out, "jigc task validate");
    let finding = changelog_advisory(&previewed)
        .unwrap_or_else(|| panic!("the promoted member previews; got:\n{previewed:#?}"));
    assert_eq!(
        finding["severity"], "blocking",
        "the cascade promotion reaches the preview door; got: {finding}"
    );

    // The same state at the committing door: it refuses, and lands nothing.
    let before = git(repo.path(), &["rev-list", "--count", "HEAD"]);
    let refused = jigc(
        repo.path(),
        home.path(),
        &["task", "finalize", TASK, "--format", "json"],
        None,
    );
    assert_exit(&refused, 3, "jigc task finalize (promoted changelog gate)");
    assert_eq!(
        git(repo.path(), &["rev-list", "--count", "HEAD"]),
        before,
        "a refused finalize lands nothing"
    );

    // The refusing door's route: the task is still open, so it names the in-task
    // verbs — plus the exit a change that is *not* user-facing needs, since "no
    // action" is not an exit from a blocking finding.
    let route = advisory_route(
        &findings(&refused, "jigc task finalize"),
        "jigc task finalize",
    );
    assert!(
        route.contains("jigc doc create changelog"),
        "a refused finalize leaves the task open — the in-task verbs run; got: {route}"
    );
    assert!(
        route.contains(&format!(
            "jigc config set validation.{CHANGELOG_CODE}.severity advisory"
        )),
        "a blocking finding names a recovery for the not-user-facing case; got: {route}"
    );
    every_route_argv_runs(
        repo.path(),
        home.path(),
        &route,
        "the refusing `task finalize` door",
    );
}

/// **The landing door names no verb that died with the task.** Over a finalize that
/// lands, the advisory rides the exit-0 envelope — and by then `.jigc/tasks/<id>/` is
/// gone, so the in-task form is *not* offered: every argv the route prints still runs.
///
/// This is the arm that was red before T3: the shipped route offered
/// `jigc doc create changelog --task <id>` here, and running it answered
/// `no task <id>` at exit 1.
#[test]
fn the_landing_finalize_door_prints_no_dead_argv() {
    let (repo, home) = corpus("changelog-landed", false);

    let out = jigc(
        repo.path(),
        home.path(),
        &["task", "finalize", TASK, "--format", "json"],
        None,
    );
    assert_exit(
        &out,
        0,
        "jigc task finalize (advisory rides the landed commit)",
    );
    let findings = findings(&out, "jigc task finalize");
    let route = advisory_route(&findings, "jigc task finalize");
    assert!(
        !route.contains("--task"),
        "the commit landed and the working area is gone — no in-task argv may be \
         printed here; got: {route}"
    );
    every_route_argv_runs(
        repo.path(),
        home.path(),
        &route,
        "the landing `task finalize` door",
    );
}
