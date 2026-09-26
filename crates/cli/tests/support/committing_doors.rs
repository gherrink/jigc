//! The **committing-door fixture harness** — one home for the ten-door axis's
//! rejecting-cell fixtures, shared by the suites that drive it
//! ([pinning.md](../../../../implementation/pinning.md) §4, the shared-module shape).
//!
//! It was `tests/commit_rejected_axis.rs`'s private preamble until M52 Increment 1 / T1,
//! when a second suite — `tests/reject_document_axis.rs`, which asserts the **machine**
//! arm of the same ten cells — needed the identical fixtures. Copying 600 lines of
//! per-door setup would have given the axis two homes that drift; the axis itself is
//! `cli::invocation_log::COMMITTING_DOORS` either way, and this module is the fixtures
//! that reach each of its members' commit phase.
//!
//! Nothing here asserts anything about the emitted surface: each consumer suite owns its
//! own claims. [`drive`] builds one door's fixture, installs the rejecting `pre-commit`
//! hook, and hands back the argv to drive plus the two facts every consumer needs (the
//! door's own re-run and its state-truth clause).

use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

/// The distinctive bytes the rejecting hook speaks — any appearance in an output stream
/// can only have come from the hook, so it witnesses "verbatim and unwrapped".
pub const HOOK_MARKER: &str = "policy: COMMIT-REJECTED-AXIS-MARKER";

/// A throwaway directory that removes itself on drop.
pub struct TempDir(PathBuf);

impl TempDir {
    pub fn new(tag: &str) -> Self {
        let mut path = std::env::temp_dir();
        path.push(format!(
            "jigc-commit-rejected-axis-{tag}-{}-{:?}",
            std::process::id(),
            engine::tempname::unique_nanos(),
        ));
        fs::create_dir_all(&path).expect("create temp dir");
        TempDir(path)
    }

    pub fn path(&self) -> &Path {
        &self.0
    }
}

impl Drop for TempDir {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}

/// Run a `git` command in `cwd`, asserting success, returning stdout.
pub fn git(cwd: &Path, args: &[&str]) -> String {
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
pub fn jigc(repo: &Path, home: &Path, args: &[&str], stdin: Option<&[u8]>) -> std::process::Output {
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
pub fn jigc_ok(repo: &Path, home: &Path, args: &[&str], what: &str) -> std::process::Output {
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
pub fn base_repo(tag: &str, squash: Option<&str>) -> (TempDir, TempDir) {
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
pub fn install_rejecting_hook(repo: &Path) {
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
pub fn remove_hook(repo: &Path) {
    let _ = fs::remove_file(repo.join(".git").join("hooks").join("pre-commit"));
}

/// A `pre-commit` hook that lets the first `pass` commits through and rejects every one
/// after — the only way to drive a **partially landed** door, whose earlier commits are
/// history by design and therefore cannot be claimed away.
pub fn install_counting_hook(repo: &Path, pass: u32) {
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
pub fn commit_count(repo: &Path) -> u32 {
    git(repo, &["rev-list", "--count", "HEAD"])
        .trim()
        .parse()
        .expect("the commit count parses")
}

/// A conformant `adr` body, optionally stamped (`None` = the unstamped v0 state
/// `migrate-corpus` lifts).
pub fn adr_body(title: &str, stamp: Option<u32>) -> String {
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
pub fn commit_adr(repo: &Path, slug: &str, title: &str, stamp: Option<u32>) {
    let dir = repo.join("docs").join("decisions");
    fs::create_dir_all(&dir).expect("mk docs/decisions/");
    fs::write(dir.join(format!("{slug}.md")), adr_body(title, stamp)).expect("write adr");
    git(repo, &["add", "."]);
    git(repo, &["commit", "-q", "-m", "seed adr"]);
}

/// A committed 2-criteria `spec` — the `add-from-spec` seed substrate.
pub const TWO_CRITERIA_SPEC: &str = "\
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
pub fn stage_subtask_doc(repo: &Path, sub: &str, address: &str, body: &str) {
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
pub fn seed_task(repo: &Path, home: &Path, intent: &str) -> String {
    jigc_ok(
        repo,
        home,
        &["start", "--workflow", "single-task", intent],
        "`jigc start`",
    );
    let task = intent.replace(' ', "-");
    fill_commit_doc(repo, home, &task);
    task
}

/// Author a task's provisioned `commit` doc to the point its finalize reaches the commit
/// phase — the two required leaves plus the two optional ones. Shared by [`seed_task`] and
/// the amend cell, which mints through a different door and needs the identical fill.
pub fn fill_commit_doc(repo: &Path, home: &Path, task: &str) {
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
}

/// The parsed JSONL invocation-log records at `.jigc/logs/invocations.jsonl`.
pub fn log_records(repo: &Path) -> Vec<serde_json::Value> {
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
pub fn record_for<'a>(
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
///
/// The span is **code-fenced the CommonMark way**: the opening run of backticks is as long as
/// it needs to be to exceed any backtick run inside the command line, so a re-run embedding an
/// author's backticked prose is still delimited unambiguously. The lift reads the run length
/// off the emitted bytes rather than assuming one backtick.
pub fn lift_rerun(stderr: &str) -> String {
    let marker = "then re-run ";
    let start = stderr
        .rfind(marker)
        .unwrap_or_else(|| panic!("the frame must route back to a re-run; stderr:\n{stderr}"))
        + marker.len();
    let rest = &stderr[start..];
    let fence: String = rest.chars().take_while(|c| *c == '`').collect();
    assert!(
        !fence.is_empty(),
        "the re-run command line must be code-fenced; stderr:\n{stderr}",
    );
    let body = &rest[fence.len()..];
    let end = body.find(&fence).unwrap_or_else(|| {
        panic!("the re-run command line must be closed by its fence; stderr:\n{stderr}")
    });
    body[..end].to_string()
}

/// Split a printed command line into argv the way a shell would for the quoting forms the
/// frame emits: bare tokens, single-quoted runs (POSIX-literal, `'` written `'\''`), plus
/// double-quoted runs with `\"` / `\\` escapes.
pub fn shell_split(line: &str) -> Vec<String> {
    #[derive(PartialEq)]
    enum Quote {
        None,
        Single,
        Double,
    }
    let mut argv = Vec::new();
    let mut current = String::new();
    let mut open = false;
    let mut state = Quote::None;
    let mut chars = line.chars();
    while let Some(c) = chars.next() {
        match (&state, c) {
            (Quote::Single, '\'') => state = Quote::None,
            (Quote::Single, c) => current.push(c),
            (Quote::Double, '\\') => current.push(chars.next().unwrap_or('\\')),
            (Quote::Double, '"') => state = Quote::None,
            (Quote::Double, c) => current.push(c),
            (Quote::None, '\\') => {
                current.push(chars.next().unwrap_or('\\'));
                open = true;
            }
            (Quote::None, '\'') => {
                state = Quote::Single;
                open = true;
            }
            (Quote::None, '"') => {
                state = Quote::Double;
                open = true;
            }
            (Quote::None, c) if c.is_whitespace() => {
                if open {
                    argv.push(std::mem::take(&mut current));
                    open = false;
                }
            }
            (Quote::None, c) => {
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
pub struct DoorCase {
    pub repo: TempDir,
    pub home: TempDir,
    pub driven: Vec<String>,
    pub expected_rerun: Vec<String>,
    pub survived: String,
}

pub fn owned(args: &[&str]) -> Vec<String> {
    args.iter().map(|s| (*s).to_string()).collect()
}

/// Build each door's fixture, install the rejecting hook, and drive the door.
pub fn drive(verb: &str) -> DoorCase {
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
        // The amend arm (F-10): the same leaf, minted through `jigc task amend` so the task
        // carries the marker that selects the second commit model. Nothing is `git add`-ed —
        // this arm refuses over a non-empty index, so a staged file here would reach the
        // dirty-index gate instead of the hook.
        "jigc task finalize (amend)" => {
            let (repo, home) = base_repo("task-finalize-amend", None);
            jigc_ok(
                repo.path(),
                home.path(),
                &["task", "amend", "repair the install message"],
                "`jigc task amend`",
            );
            let task = "repair-the-install-message";
            fill_commit_doc(repo.path(), home.path(), task);
            install_rejecting_hook(repo.path());
            DoorCase {
                survived: "`HEAD` is unchanged".to_string(),
                expected_rerun: owned(&["jigc", "task", "finalize", task]),
                driven: owned(&["task", "finalize", task]),
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
        "jigc task discard" => {
            let (repo, home) = base_repo("task-discard", None);
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
            install_rejecting_hook(repo.path());
            DoorCase {
                survived: "the task's working area is intact".to_string(),
                // The frame echoes the consent the run carried (M50 Inc 3 / T2): since the
                // staged-prose guard, a bare re-run refuses in exactly the state that
                // printed the frame whenever the area stages a doc, so a re-run line that
                // dropped the flag would not reach the commit phase the frame promises.
                expected_rerun: owned(&["jigc", "task", "discard", "area-low", "--force"]),
                driven: owned(&["task", "discard", "area-low", "--force"]),
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
