//! The **committing-door axis, rejecting side** (M47 Inc 3 T7; `DECISIONS.md` →
//! 2026-07-26 M47 the Settle, Decision 6; `design/finalize.md` → 6. Commit, the survivable
//! frame; `design/surface-contract.md` → The error-code namespace).
//!
//! M42 gave **one** door — `jigc task finalize` — the frame a rejected commit always
//! deserved: git's bytes verbatim, a sentence saying what survived, and the door's own
//! re-run. The other **eight** committing doors printed a bare `{err:#}` with no
//! recoverability statement, no route, and no error identity in the invocation log —
//! and the two `milestone finalize` arms logged the *task* door's
//! `finalize.commit-rejected`, a lying code on the surface M42 built to stop the log
//! lying (law 1).
//!
//! This suite is the **rejecting sibling** of `tests/hook_output_axis.rs` (the same nine
//! doors, the same one exclusion — `jigc setup`'s install commit passes `--no-verify` by
//! recorded design, so no hook runs and there is nothing to reject). It does **not**
//! enumerate the doors itself: it iterates
//! [`cli::invocation_log::COMMITTING_DOORS`](cli::invocation_log::COMMITTING_DOORS), the
//! one code-side table `registry_mirrors_the_declared_members` also derives
//! `ERROR_CODE_REGISTRY` from — one list, two consumers, so a door added without a code
//! (or a code minted for no door) reddens rather than silently logging the wrong verb.
//! A member this suite has no arm for is a hard `panic!`, never a skip.
//!
//! Per door, through the **real binary** under a rejecting `pre-commit` hook:
//!
//! 1. the run exits **non-zero**;
//! 2. git's (the hook's) own bytes are **verbatim and unwrapped** — the correction signal
//!    is never edited (`design/finalize.md` → 6. Commit, M40 refinement 3);
//! 3. a **state-truth** sentence names what survived the rejection;
//! 4. the printed re-run argv is **that door's own**, not the task door's;
//! 5. the invocation log's `error_code` is **that door's** identity — read back from the
//!    log record, which is what closes the **release hole**: the `Outcome::error`
//!    membership check is a `debug_assert!`, compiled out of the release build the
//!    acceptance trial actually runs;
//! 6. the printed argv is **lifted verbatim** out of the message, and after the hook is
//!    removed that exact command line **exits 0** — so "a re-run after any rejection
//!    recovers" is proven, not printed.
//!
//! **Scope note on the two `milestone finalize` members.** They are the two commit-model
//! arms (`finalize.fan-out.squash` true/false), each with its own `Err` arm and its own
//! identity. This suite drives both through the docs-only fixture, which reaches each
//! arm's commit phase in seconds; the `squash: false` arm's *provisioned-worktree* chain
//! rejection — one rejection per cause, with the code surviving — is iterated over its
//! whole rejection-cause axis by `tests/milestone_abort_survives.rs` (M47 Inc 3 T1).

use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

use cli::invocation_log::COMMITTING_DOORS;

/// The distinctive bytes the rejecting hook speaks — any appearance in an output stream
/// can only have come from the hook, so it witnesses "verbatim and unwrapped".
const HOOK_MARKER: &str = "policy: COMMIT-REJECTED-AXIS-MARKER";

/// A throwaway directory that removes itself on drop.
struct TempDir(PathBuf);

impl TempDir {
    fn new(tag: &str) -> Self {
        let mut path = std::env::temp_dir();
        path.push(format!(
            "jigc-commit-rejected-axis-{tag}-{}-{:?}",
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

/// Run a `git` command in `cwd`, asserting success, returning stdout.
fn git(cwd: &Path, args: &[&str]) -> String {
    let out = Command::new("git")
        .args(args)
        .current_dir(cwd)
        .output()
        .expect("run git");
    assert!(
        out.status.success(),
        "git {args:?} in {cwd:?} failed: {}",
        String::from_utf8_lossy(&out.stderr),
    );
    String::from_utf8(out.stdout).expect("utf-8 git stdout")
}

/// Run `jigc <args>` with `cwd = repo` and `$HOME = home`, never inheriting a harness
/// `JIGC_PACK_DIR` (the `[dev ▸ methodology]` compose the milestone doors need requires it
/// absent), optionally piping `stdin`.
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

/// Run `jigc <args>`, asserting exit 0.
fn jigc_ok(repo: &Path, home: &Path, args: &[&str], what: &str) -> std::process::Output {
    let out = jigc(repo, home, args, None);
    assert!(
        out.status.success(),
        "{what} must exit 0; stdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr),
    );
    out
}

/// A real git repo with one commit, `jigc setup` run over it (which also wires the
/// `[dev ▸ methodology]` compose marker), and the **invocation-log knob ON** in the project
/// cascade — the log record is where each door's error identity is read back from.
fn base_repo(tag: &str, squash: Option<&str>) -> (TempDir, TempDir) {
    let repo = TempDir::new(tag);
    let home = TempDir::new(&format!("home-{tag}"));
    git(repo.path(), &["init", "-q"]);
    git(repo.path(), &["config", "user.email", "test@example.com"]);
    git(repo.path(), &["config", "user.name", "Test"]);
    fs::write(repo.path().join("README.md"), "hello\n").expect("write README.md");
    git(repo.path(), &["add", "."]);
    git(repo.path(), &["commit", "-q", "-m", "initial"]);

    jigc_ok(repo.path(), home.path(), &["setup"], "`jigc setup`");

    let mut manifest = String::from("scalar:\n  invocation-log: true\n");
    if let Some(value) = squash {
        manifest.push_str(&format!("  finalize.fan-out.squash: {value}\n"));
    }
    fs::write(
        repo.path()
            .join(".jigc")
            .join("config")
            .join("manifest.yaml"),
        manifest,
    )
    .expect("write the project scalar layer");
    (repo, home)
}

/// Install an executable `pre-commit` hook that **rejects every commit**, speaking
/// [`HOOK_MARKER`] on stderr (replacing the warn-only one `jigc setup` installed).
fn install_rejecting_hook(repo: &Path) {
    let hook = repo.join(".git").join("hooks").join("pre-commit");
    fs::create_dir_all(hook.parent().expect("hooks dir")).expect("mk hooks dir");
    fs::write(
        &hook,
        format!("#!/bin/sh\necho '{HOOK_MARKER}' 1>&2\nexit 1\n"),
    )
    .expect("write the rejecting pre-commit hook");
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        fs::set_permissions(&hook, fs::Permissions::from_mode(0o755)).expect("chmod hook");
    }
}

/// The operator's repair between the refused run and the recovery re-run.
fn remove_hook(repo: &Path) {
    let _ = fs::remove_file(repo.join(".git").join("hooks").join("pre-commit"));
}

/// A `pre-commit` hook that lets the first `pass` commits through and rejects every one
/// after — the only way to drive a **partially landed** door, whose earlier commits are
/// history by design and therefore cannot be claimed away.
fn install_counting_hook(repo: &Path, pass: u32) {
    let counter = repo.join(".git").join("axis-commit-count");
    let hook = repo.join(".git").join("hooks").join("pre-commit");
    fs::create_dir_all(hook.parent().expect("hooks dir")).expect("mk hooks dir");
    fs::write(
        &hook,
        format!(
            "#!/bin/sh\nC='{}'\nn=$(cat \"$C\" 2>/dev/null || echo 0)\nn=$((n+1))\n\
             echo \"$n\" > \"$C\"\nif [ \"$n\" -le {pass} ]; then exit 0; fi\n\
             echo '{HOOK_MARKER}' 1>&2\nexit 1\n",
            counter.display(),
        ),
    )
    .expect("write the counting pre-commit hook");
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        fs::set_permissions(&hook, fs::Permissions::from_mode(0o755)).expect("chmod hook");
    }
}

/// The number of commits reachable from HEAD.
fn commit_count(repo: &Path) -> u32 {
    git(repo, &["rev-list", "--count", "HEAD"])
        .trim()
        .parse()
        .expect("the commit count parses")
}

/// A conformant `adr` body, optionally stamped (`None` = the unstamped v0 state
/// `migrate-corpus` lifts).
fn adr_body(title: &str, stamp: Option<u32>) -> String {
    let stamp_line = match stamp {
        Some(v) => format!("schema-version: {v}\n"),
        None => String::new(),
    };
    format!(
        "---\nstatus: accepted\ndate: 2026-07-26\n{stamp_line}---\n\n# {title}\n\n## Context\n\n\
         Session lookups must stay sub-millisecond.\n\n## Options\n\nA distributed cache was \
         weighed and rejected on latency.\n\n## Decision\n\nKeep sessions in one node.\n\n\
         ## Consequences\n\nA cold node loses its sessions.\n"
    )
}

/// Write + commit an `adr` at its canonical `docs/decisions/<slug>.md`.
fn commit_adr(repo: &Path, slug: &str, title: &str, stamp: Option<u32>) {
    let dir = repo.join("docs").join("decisions");
    fs::create_dir_all(&dir).expect("mk docs/decisions/");
    fs::write(dir.join(format!("{slug}.md")), adr_body(title, stamp)).expect("write adr");
    git(repo, &["add", "."]);
    git(repo, &["commit", "-q", "-m", "seed adr"]);
}

/// A committed 2-criteria `spec` — the `add-from-spec` seed substrate.
const TWO_CRITERIA_SPEC: &str = "\
# Rate limit

## Goal

Bound per-client request volume.

## Context

Downstream services enforced limits ad hoc.

## Criteria

### Rejects the 101st request  {#rejects-burst}

The gateway rejects the 101st request in a rolling 60s window.

### Admits within the window  {#admits-within}

Requests under the cap are admitted unchanged.
";

/// Stage a doc body + its provenance bit into a sub-task's `tasks/<sub>/docs/` area — the
/// merged-doc contribution the milestone boundary promotes.
fn stage_subtask_doc(repo: &Path, sub: &str, address: &str, body: &str) {
    let docs = repo.join(".jigc").join("tasks").join(sub).join("docs");
    fs::create_dir_all(&docs).expect("mk the sub-task docs area");
    fs::write(docs.join(format!("{address}.md")), body).expect("write the staged body");
    let manifest = docs.join("provenance.json");
    let mut record: serde_json::Value = match fs::read_to_string(&manifest) {
        Ok(s) => serde_json::from_str(&s).expect("the provenance manifest parses"),
        Err(_) => serde_json::json!({ "docs": {} }),
    };
    record["docs"][address] = serde_json::Value::String("created".to_string());
    fs::write(
        &manifest,
        serde_json::to_string_pretty(&record).expect("serialize the manifest"),
    )
    .expect("write the provenance manifest");
}

/// Mint a `single-task` and fill its commit doc's author-required fields/slots, returning
/// the task id.
fn seed_task(repo: &Path, home: &Path, intent: &str) -> String {
    jigc_ok(
        repo,
        home,
        &["start", "--workflow", "single-task", intent],
        "`jigc start`",
    );
    let task = intent.replace(' ', "-");
    for (addr, value) in [
        (format!("commit:{task}#type"), "feat"),
        (format!("commit:{task}#scope"), "cache"),
    ] {
        jigc_ok(
            repo,
            home,
            &["doc", "set-field", &addr, "--value", value],
            "`jigc doc set-field`",
        );
    }
    for (addr, prose) in [
        (format!("commit:{task}#summary"), "survive the rejection\n"),
        (format!("commit:{task}#body"), "An axis-sweep change.\n"),
    ] {
        let out = jigc(
            repo,
            home,
            &["doc", "set-slot", &addr, "--from-file", "-"],
            Some(prose.as_bytes()),
        );
        assert!(
            out.status.success(),
            "`jigc doc set-slot {addr}` must exit 0; stderr:\n{}",
            String::from_utf8_lossy(&out.stderr),
        );
    }
    task
}

/// The parsed JSONL invocation-log records at `.jigc/logs/invocations.jsonl`.
fn log_records(repo: &Path) -> Vec<serde_json::Value> {
    let path = repo.join(".jigc").join("logs").join("invocations.jsonl");
    match fs::read_to_string(&path) {
        Ok(body) => body
            .lines()
            .filter(|l| !l.trim().is_empty())
            .map(|l| serde_json::from_str(l).expect("each log line is valid JSON"))
            .collect(),
        Err(_) => Vec::new(),
    }
}

/// The **last** logged record whose `argv` is exactly `argv` — the door's own run, never an
/// earlier setup call that happens to share a token.
fn record_for<'a>(
    records: &'a [serde_json::Value],
    argv: &[String],
) -> Option<&'a serde_json::Value> {
    records.iter().rev().find(|r| {
        r["argv"].as_array().is_some_and(|a| {
            a.len() == argv.len()
                && a.iter()
                    .zip(argv)
                    .all(|(v, want)| v.as_str() == Some(want.as_str()))
        })
    })
}

/// Lift the re-run command line **verbatim** out of the frame's closing sentence
/// (``… then re-run `<argv>`.``) — the emitted bytes are the contract, so the recovery run
/// re-executes what the door printed, never a reconstruction.
fn lift_rerun(stderr: &str) -> String {
    let marker = "then re-run `";
    let start = stderr
        .rfind(marker)
        .unwrap_or_else(|| panic!("the frame must route back to a re-run; stderr:\n{stderr}"))
        + marker.len();
    let rest = &stderr[start..];
    let end = rest.find('`').unwrap_or_else(|| {
        panic!("the re-run command line must be closed by a backtick; stderr:\n{stderr}")
    });
    rest[..end].to_string()
}

/// Split a printed command line into argv the way a shell would for the one quoting form the
/// frame emits: bare tokens, plus double-quoted runs with `\"` / `\\` escapes.
fn shell_split(line: &str) -> Vec<String> {
    let mut argv = Vec::new();
    let mut current = String::new();
    let mut open = false;
    let mut quoted = false;
    let mut chars = line.chars();
    while let Some(c) = chars.next() {
        match c {
            '\\' if quoted => current.push(chars.next().unwrap_or('\\')),
            '"' => {
                quoted = !quoted;
                open = true;
            }
            c if c.is_whitespace() && !quoted => {
                if open {
                    argv.push(std::mem::take(&mut current));
                    open = false;
                }
            }
            c => {
                current.push(c);
                open = true;
            }
        }
    }
    if open {
        argv.push(current);
    }
    argv
}

/// One door's driven rejection: the repo it ran in, the argv it was driven with, the argv its
/// frame must print, and the state-truth substring its frame must carry.
struct DoorCase {
    repo: TempDir,
    home: TempDir,
    driven: Vec<String>,
    expected_rerun: Vec<String>,
    survived: String,
}

fn owned(args: &[&str]) -> Vec<String> {
    args.iter().map(|s| (*s).to_string()).collect()
}

/// Build each door's fixture, install the rejecting hook, and drive the door.
fn drive(verb: &str) -> DoorCase {
    match verb {
        "jigc task finalize" => {
            let (repo, home) = base_repo("task-finalize", None);
            let task = seed_task(repo.path(), home.path(), "survive the rejection");
            fs::write(repo.path().join("code.txt"), "the task's work\n").expect("write code.txt");
            git(repo.path(), &["add", "code.txt"]);
            install_rejecting_hook(repo.path());
            let driven = owned(&["task", "finalize", &task]);
            DoorCase {
                survived: format!("task {task} is intact"),
                expected_rerun: owned(&["jigc", "task", "finalize", &task]),
                driven,
                repo,
                home,
            }
        }
        "jigc milestone finalize (squash: true)" | "jigc milestone finalize (squash: false)" => {
            let squash = if verb.ends_with("true)") {
                "true"
            } else {
                "false"
            };
            let (repo, home) = base_repo(&format!("ms-finalize-{squash}"), Some(squash));
            jigc_ok(
                repo.path(),
                home.path(),
                &["milestone", "create", "Cache rework"],
                "`jigc milestone create`",
            );
            jigc_ok(
                repo.path(),
                home.path(),
                &["milestone", "add-task", "cache-rework", "Area low"],
                "`jigc milestone add-task`",
            );
            // A merged-doc contribution, so the boundary reaches its commit phase rather
            // than the zero-contribution refusal.
            stage_subtask_doc(
                repo.path(),
                "area-low",
                "adr:eviction-policy",
                &adr_body("Eviction policy", None),
            );
            install_rejecting_hook(repo.path());
            DoorCase {
                survived: "milestone:cache-rework is intact".to_string(),
                expected_rerun: owned(&["jigc", "milestone", "finalize", "cache-rework"]),
                driven: owned(&["milestone", "finalize", "cache-rework"]),
                repo,
                home,
            }
        }
        "jigc rename" => {
            let (repo, home) = base_repo("rename", None);
            commit_adr(repo.path(), "alpha-decision", "Alpha decision", Some(2));
            install_rejecting_hook(repo.path());
            DoorCase {
                survived: "adr:alpha-decision".to_string(),
                expected_rerun: owned(&[
                    "jigc",
                    "rename",
                    "adr:alpha-decision",
                    "--to",
                    "Beta decision",
                ]),
                driven: owned(&["rename", "adr:alpha-decision", "--to", "Beta decision"]),
                repo,
                home,
            }
        }
        "jigc migrate-corpus" => {
            let (repo, home) = base_repo("migrate-corpus", None);
            // An unstamped (v0) committed ADR — the stamp add-field migration writes it back
            // and the commit boundary lands it, so the hook genuinely fires.
            commit_adr(repo.path(), "alpha-decision", "Alpha decision", None);
            install_rejecting_hook(repo.path());
            DoorCase {
                survived: "migrated bytes".to_string(),
                expected_rerun: owned(&["jigc", "migrate-corpus"]),
                driven: owned(&["migrate-corpus"]),
                repo,
                home,
            }
        }
        "jigc milestone create" => {
            let (repo, home) = base_repo("ms-create", None);
            install_rejecting_hook(repo.path());
            DoorCase {
                survived: "milestone:cache-rework".to_string(),
                expected_rerun: owned(&["jigc", "milestone", "create", "Cache rework"]),
                driven: owned(&["milestone", "create", "Cache rework"]),
                repo,
                home,
            }
        }
        "jigc milestone add-task" => {
            let (repo, home) = base_repo("ms-add-task", None);
            jigc_ok(
                repo.path(),
                home.path(),
                &["milestone", "create", "Cache rework"],
                "`jigc milestone create`",
            );
            install_rejecting_hook(repo.path());
            DoorCase {
                survived: "milestone:cache-rework".to_string(),
                expected_rerun: owned(&[
                    "jigc",
                    "milestone",
                    "add-task",
                    "cache-rework",
                    "Area low",
                ]),
                driven: owned(&["milestone", "add-task", "cache-rework", "Area low"]),
                repo,
                home,
            }
        }
        "jigc milestone add-from-spec" => {
            let (repo, home) = base_repo("ms-add-from-spec", None);
            let specs = repo.path().join("docs").join("specs");
            fs::create_dir_all(&specs).expect("mk docs/specs/");
            fs::write(specs.join("rate-limit.md"), TWO_CRITERIA_SPEC).expect("write the spec");
            git(repo.path(), &["add", "."]);
            git(repo.path(), &["commit", "-q", "-m", "add spec"]);
            jigc_ok(
                repo.path(),
                home.path(),
                &["milestone", "create", "Rate limit"],
                "`jigc milestone create`",
            );
            install_rejecting_hook(repo.path());
            DoorCase {
                survived: "milestone:rate-limit's task list names exactly what its record names"
                    .to_string(),
                expected_rerun: owned(&[
                    "jigc",
                    "milestone",
                    "add-from-spec",
                    "rate-limit",
                    "spec:rate-limit",
                ]),
                driven: owned(&[
                    "milestone",
                    "add-from-spec",
                    "rate-limit",
                    "spec:rate-limit",
                ]),
                repo,
                home,
            }
        }
        "jigc milestone discard" => {
            let (repo, home) = base_repo("ms-discard", None);
            jigc_ok(
                repo.path(),
                home.path(),
                &["milestone", "create", "Cache rework"],
                "`jigc milestone create`",
            );
            install_rejecting_hook(repo.path());
            DoorCase {
                survived: "milestone:cache-rework".to_string(),
                expected_rerun: owned(&["jigc", "milestone", "discard", "cache-rework"]),
                driven: owned(&["milestone", "discard", "cache-rework"]),
                repo,
                home,
            }
        }
        other => panic!(
            "`{other}` is a committing door with no arm in this suite — the axis is the \
             code-side `COMMITTING_DOORS` table, so a door added there owes its arm here",
        ),
    }
}

/// The axis sweep: every code-side committing door, driven through the real binary under a
/// rejecting `pre-commit` hook.
#[test]
fn every_committing_door_frames_its_rejection_names_itself_and_recovers() {
    assert_eq!(
        COMMITTING_DOORS.len(),
        9,
        "the axis is 9 doors + `jigc setup` excluded by its recorded `--no-verify` reason",
    );

    for door in COMMITTING_DOORS {
        let verb = door.verb;
        let case = drive(verb);
        let repo = case.repo.path();
        let home = case.home.path();
        let driven: Vec<&str> = case.driven.iter().map(String::as_str).collect();

        let head_before = git(repo, &["rev-parse", "HEAD"]);
        let rejected = jigc(repo, home, &driven, None);
        let stdout = String::from_utf8_lossy(&rejected.stdout).into_owned();
        let stderr = String::from_utf8_lossy(&rejected.stderr).into_owned();

        // (1) the run fails loudly.
        assert!(
            !rejected.status.success(),
            "[{verb}] a rejected commit must exit non-zero; stdout:\n{stdout}\nstderr:\n{stderr}",
        );
        // …and nothing landed.
        assert_eq!(
            git(repo, &["rev-parse", "HEAD"]),
            head_before,
            "[{verb}] a rejected commit must leave HEAD untouched",
        );

        // (2) git's bytes are verbatim and unwrapped — the hook IS the correction signal.
        assert!(
            stderr.contains(HOOK_MARKER),
            "[{verb}] the hook's own bytes must survive verbatim; stderr:\n{stderr}",
        );
        assert!(
            stderr.contains("`git commit` was rejected (no commit was made):"),
            "[{verb}] git's rejection line must stay unwrapped; stderr:\n{stderr}",
        );

        // (3) a state-truth sentence names what survived.
        assert!(
            stderr.contains(&case.survived),
            "[{verb}] the frame must state what survived (expected to name `{}`); stderr:\n{stderr}",
            case.survived,
        );

        // (4) the printed re-run is THIS door's own argv — lifted verbatim, never rebuilt.
        let lifted = lift_rerun(&stderr);
        let lifted_argv = shell_split(&lifted);
        assert_eq!(
            lifted_argv, case.expected_rerun,
            "[{verb}] the frame must name the door's OWN re-run; printed `{lifted}`",
        );

        // (5) the log names THIS door — read off the record, so the release build's
        // compiled-out `debug_assert!` cannot hide a wrong or missing identity.
        let records = log_records(repo);
        let record = record_for(&records, &case.driven).unwrap_or_else(|| {
            panic!("[{verb}] the rejected run must be logged; records:\n{records:#?}")
        });
        assert_eq!(
            record["error_code"].as_str(),
            Some(door.error_code),
            "[{verb}] the log must carry THIS door's identity; got {record}",
        );
        assert_eq!(
            record["finding_codes"].as_array().map(Vec::len),
            Some(0),
            "[{verb}] a hook rejection is an operational error, not a Finding; got {record}",
        );

        // (6) the claim is TRUE: remove the hook, run the printed line verbatim, it lands.
        // The argv executed here is the one **lifted out of the emitted message**, never
        // `expected_rerun` — a reconstruction can pass while the bytes an operator would
        // actually paste are broken.
        remove_hook(repo);
        assert_eq!(
            lifted_argv.first().map(String::as_str),
            Some("jigc"),
            "[{verb}] the printed line must be a runnable `jigc …` command; printed `{lifted}`",
        );
        let rerun_argv: Vec<&str> = lifted_argv[1..].iter().map(String::as_str).collect();
        let recovered = jigc(repo, home, &rerun_argv, None);
        assert!(
            recovered.status.success(),
            "[{verb}] the printed re-run must recover; stdout:\n{}\nstderr:\n{}",
            String::from_utf8_lossy(&recovered.stdout),
            String::from_utf8_lossy(&recovered.stderr),
        );
    }
}

/// **The state-truth clause is checked at its hard case, not only at k = 0.** The axis sweep
/// above rejects the *first* commit of every door, which is the state-truth clause's easy
/// arm: nothing had landed, so "nothing survives" is trivially true. `add-from-spec` is the
/// one door whose seeding is **resumable** (M47 Inc 2 T3) — it lands one record-only commit
/// per seeded sub-task, so a rejection at the k-th leaves the k−1 earlier commits as history
/// that no rollback can take back. A clause claiming "the milestone is unchanged" would
/// therefore be a **law-1 lie for every k > 1**, and no sweep that rejects the first commit
/// could ever see it. This arm drives the k > 0 state directly with a counting hook.
#[test]
fn add_from_spec_states_the_partial_truth_when_an_earlier_sub_task_already_landed() {
    let (repo, home) = base_repo("ms-add-from-spec-partial", None);
    let specs = repo.path().join("docs").join("specs");
    fs::create_dir_all(&specs).expect("mk docs/specs/");
    fs::write(specs.join("rate-limit.md"), TWO_CRITERIA_SPEC).expect("write the spec");
    git(repo.path(), &["add", "."]);
    git(repo.path(), &["commit", "-q", "-m", "add spec"]);
    jigc_ok(
        repo.path(),
        home.path(),
        &["milestone", "create", "Rate limit"],
        "`jigc milestone create`",
    );

    // The FIRST seeded sub-task's record commit lands; the second is refused.
    install_counting_hook(repo.path(), 1);
    let before = commit_count(repo.path());
    let argv = [
        "milestone",
        "add-from-spec",
        "rate-limit",
        "spec:rate-limit",
    ];
    let rejected = jigc(repo.path(), home.path(), &argv, None);
    let stderr = String::from_utf8_lossy(&rejected.stderr).into_owned();
    assert!(
        !rejected.status.success(),
        "the refused k-th record commit must fail the run; stderr:\n{stderr}",
    );
    assert_eq!(
        commit_count(repo.path()),
        before + 1,
        "exactly the first sub-task's record commit landed; stderr:\n{stderr}",
    );

    // The frame must not claim away the commit that DID land.
    assert!(
        stderr.contains("any sub-task this run already recorded stayed committed"),
        "the frame must state the partial truth; stderr:\n{stderr}",
    );
    assert!(
        !stderr.contains("is unchanged"),
        "the frame must NOT claim the milestone is unchanged — a landed record commit is \
         history; stderr:\n{stderr}",
    );

    // …and the printed re-run seeds exactly the remainder.
    remove_hook(repo.path());
    let lifted = shell_split(&lift_rerun(&stderr));
    let rerun: Vec<&str> = lifted[1..].iter().map(String::as_str).collect();
    jigc_ok(repo.path(), home.path(), &rerun, "the lifted re-run");
    let listed = String::from_utf8_lossy(
        &jigc_ok(
            repo.path(),
            home.path(),
            &["milestone", "list-tasks", "rate-limit"],
            "`jigc milestone list-tasks`",
        )
        .stdout,
    )
    .into_owned();
    let record = fs::read_to_string(
        repo.path()
            .join("docs")
            .join("milestone-records")
            .join("rate-limit.md"),
    )
    .expect("read the committed milestone record");
    // `milestone:<id> tasks (N): <id>, <id>` — the id-sorted listing line.
    let ids: Vec<&str> = listed
        .lines()
        .find(|l| l.contains(" tasks ("))
        .and_then(|l| l.split_once("): "))
        .map(|(_, tail)| tail.split(',').map(str::trim).collect())
        .unwrap_or_default();
    assert_eq!(
        ids.len(),
        2,
        "the recovered milestone carries both criteria; listing:\n{listed}",
    );
    for id in ids {
        assert!(
            record.contains(id),
            "the committed record must name `{id}` after the recovery; record:\n{record}",
        );
    }
}

/// The `--format json` arm: the framed text rides the `operational_error` envelope, so a
/// tooling consumer parses the same three halves the agent-text surface prints (git's bytes,
/// the state-truth sentence, the door's own re-run) instead of raw text on stderr. One arm
/// suffices — the wrap is `render::commit_rejected`'s single `Format::Json` branch, shared by
/// every door on the axis.
#[test]
fn the_json_arm_keeps_the_framed_text_in_the_operational_error_envelope() {
    let case = drive("jigc task finalize");
    let mut argv = vec!["--format".to_string(), "json".to_string()];
    argv.extend(case.driven.iter().cloned());
    let args: Vec<&str> = argv.iter().map(String::as_str).collect();

    let rejected = jigc(case.repo.path(), case.home.path(), &args, None);
    assert!(
        !rejected.status.success(),
        "the JSON rejection must exit non-zero; stdout:\n{}",
        String::from_utf8_lossy(&rejected.stdout),
    );
    let stderr = String::from_utf8_lossy(&rejected.stderr).into_owned();
    let envelope: serde_json::Value = serde_json::from_str(stderr.trim()).unwrap_or_else(|err| {
        panic!("the JSON rejection must be a single `operational_error` envelope ({err}); stderr:\n{stderr}")
    });
    let framed = envelope["error"].as_str().unwrap_or_else(|| {
        panic!("the envelope must carry the framed text at `error`; got:\n{stderr}")
    });
    assert!(
        framed.contains(HOOK_MARKER),
        "the envelope must carry the hook's verbatim bytes; got:\n{framed}",
    );
    assert!(
        framed.contains(&case.survived),
        "the envelope must carry the state-truth sentence; got:\n{framed}",
    );
    assert_eq!(
        shell_split(&lift_rerun(framed)),
        case.expected_rerun,
        "the envelope must carry the door's own re-run; got:\n{framed}",
    );
}
