//! The **committing-door axis, rejecting side** (M47 Inc 3 T7; `DECISIONS.md` →
//! 2026-07-26 M47 the Settle, Decision 6; `design/finalize.md` → 6. Commit, the survivable
//! frame; `design/surface-contract.md` → The error-code namespace).
//!
//! M42 gave **one** door — `jigc task finalize` — the frame a rejected commit always
//! deserved: git's bytes verbatim, a sentence saying what survived, and the door's own
//! re-run. The other committing doors **[Corrected 2026-09-15 (M51 Increment 7, T3):** *the
//! other **eight** committing doors* — the axis carried nine at M42; it carries **ten** since
//! M49 Increment 2 T3, and the M42-state figure is kept rather than re-pinned.**]** printed a bare `{err:#}` with no
//! recoverability statement, no route, and no error identity in the invocation log —
//! and the two `milestone finalize` arms logged the *task* door's
//! `finalize.commit-rejected`, a lying code on the surface M42 built to stop the log
//! lying (law 1).
//!
//! This suite is the **rejecting sibling** of `tests/hook_output_axis.rs` (the same door
//! set, the same one exclusion — `jigc setup`'s install commit passes `--no-verify` by
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
//!
//! ## The second sweep: the same axis × the **empty-commit** outcome (M48 Increment 8 T2)
//!
//! `git_commit_capture` types **any** non-zero `git commit` exit as `CommitRejected`, and git
//! refuses a commit that would record nothing with exactly that shape — so a door that commits
//! unconditionally frames *"`git commit` was rejected"* over a run nobody rejected, routes to a
//! re-run that can only fail identically, logs a `*.commit-rejected` identity, and relays git's
//! unrelated untracked-file listing as if it were the cause. That is a **law-1 lie on the very
//! surface the sweep above exists to keep honest**: the rejecting sweep proves the frame is told
//! whenever a hook speaks, and this one proves it is *not* told when nobody spoke.
//!
//! [`no_committing_door_dresses_an_empty_commit_as_a_rejection`] therefore iterates the **same**
//! [`COMMITTING_DOORS`] table over the complementary cell — each door driven into the state
//! where the commit it would make records nothing — and per door asserts that it either
//! **refuses/skips earlier with its own true diagnosis** or **reaches the end and acks the
//! no-op**, with the frame's assertion, git's own empty-commit prose, and any `*.commit-rejected`
//! log identity absent in every cell. The two sweeps share one fixture vocabulary and one axis;
//! neither hand-lists a door, and an unclassified member is a hard `panic!` in both.
//!
//! **What "the empty-commit state" is per door**, established live rather than assumed:
//!
//! - the two **content** doors reach the emptiness as a *no-op*: `rename` over a title the doc
//!   already holds (M48 Inc 8 T1's guard) and `migrate-corpus` over an all-current corpus both
//!   discriminate it before committing and ack it at exit 0;
//! - the two **gate** doors refuse ahead of the boundary with a routed blocking finding —
//!   `task finalize` on the engine's `finalize.empty-commit`, `milestone finalize` (both commit
//!   models) on `milestone.zero-contribution`. The milestone pair's fixture is a
//!   `[dev ▸ methodology]` project, where the zero-work refusal keeps precedence over the
//!   planner's own empty-commit block; a dev-only milestone reports the latter instead
//!   (`tests/milestone_zero_contribution.rs`, arm (a2)) — both are the door's own true
//!   diagnosis, and neither is git's refusal;
//! - of the four **record-only** doors, `add-from-spec` is resumable and answers a fully-seeded
//!   re-run with `seeded 0 sub-task(s)`, committing nothing; the other three cannot reach an
//!   empty record commit at all, because the only route to a record write that changes nothing
//!   is the repeated call, and each refuses at its **identity** guard first
//!   (`milestone.record-exists` · `milestone.sub-task-collision` · `milestone.terminal`). Their
//!   cells therefore assert the unreachability *as observed behaviour* — the door's own
//!   diagnosis, nothing committed — which is what makes a future change that moves the write
//!   ahead of the guard redden here rather than ship a lying frame.
//!
//! ## The third sweep: the same axis × the **non-hook** failure (M51 Increment 2 / T5 — N20)
//!
//! `surface_commit_rejection` framed only on the [`CommitRejected`] downcast and fell through
//! to the plain operational envelope for everything else — so a failure **inside** a door's
//! commit transaction that no hook caused discarded a `RejectionFrame` its caller had already
//! built in full: `error_code: null` in the invocation log, no route, and no word about what
//! survived. The headline is the milestone boundary's closing `git merge --ff-only` refusing to
//! overwrite ordinary untracked main-checkout WIP (charter N20), and EC-37's correction to its
//! recorded scope is that the loss is **knob-independent** — both commit models reach the one
//! seam that lands the commit on the live checkout, so both are driven here.
//!
//! [`every_committing_door_keeps_its_frame_when_no_hook_spoke`] iterates the **same**
//! [`COMMITTING_DOORS`] table over that cell, with **no hook installed in any fixture**, and
//! per door asserts: the run exits non-zero and HEAD is untouched, the frame carries the door's
//! state clause and **the arm verifies that clause against the repository's actual
//! post-refusal state**, exactly **one** route is printed and it is this door's own re-run
//! lifted verbatim out of the emitted bytes, the invocation log carries **this door's**
//! `error_code` (read off the record, never the printed text), and nothing blames a hook.
//!
//! **The state clause is not reusable on faith** (G-47): the axis is `COMMITTING_DOORS ×
//! {hook rejection, non-hook failure} × {is the clause true?}`, and one cell answers *no* —
//! `migrate-corpus`' hook-cell clause says the migrated bytes are *"written and staged"*, and
//! in this cell **the stage is exactly what failed**. That door therefore states a second
//! clause (`RejectionFrame::survived_non_hook`) and this arm asserts *that* one, against a
//! driven `git diff --cached` showing nothing of it staged.
//!
//! **The causes are ordinary, not exotic**: a stale `.git/index.lock` (the residue of a killed
//! git) meeting the stage's `git add` or the rename's `git mv`; a promote destination that is a
//! regular file; the fan-out's `--ff-only` over colliding untracked WIP. Each fails after its
//! door's rollback has run, which is what makes a state clause statable here at all.
//!
//! **Out of this increment's scope, named rather than left silent:** a [`CommitRejected`] whose
//! cause is **git itself** rather than a hook (`core.bare`, a partial commit during a merge)
//! still renders the first sweep's hook sentence. It carries no charter row, and its driven
//! instance at `milestone create` is closed from the other side by this increment's
//! operation-in-progress refusal, which never lets that run reach a commit.
//!
//! **Non-vacuity is proven by applied mutation, not by construction**: reverting T1's
//! pre-commit emptiness discriminator in `crates/cli/src/rename.rs` reddens the `jigc rename`
//! cell on four clauses at once (exit, ack, the forbidden assertion, the log identity).

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
///
/// The span is **code-fenced the CommonMark way**: the opening run of backticks is as long as
/// it needs to be to exceed any backtick run inside the command line, so a re-run embedding an
/// author's backticked prose is still delimited unambiguously. The lift reads the run length
/// off the emitted bytes rather than assuming one backtick.
fn lift_rerun(stderr: &str) -> String {
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
fn shell_split(line: &str) -> Vec<String> {
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

/// The axis sweep: every code-side committing door, driven through the real binary under a
/// rejecting `pre-commit` hook.
#[test]
fn every_committing_door_frames_its_rejection_names_itself_and_recovers() {
    assert_eq!(
        COMMITTING_DOORS.len(),
        10,
        "the axis is 10 doors + `jigc setup` excluded by its recorded `--no-verify` reason",
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

/// The **author-prose axis** of clause (6) — the input class the prose-carrying doors are
/// designed to receive. Titles and intents are LLM-written by the determinism boundary (the
/// CLI owns structure, the model owns prose), so `$`, a backtick and `;` are ordinary input,
/// while every fixture in the sweep above feeds prose whose only special character is
/// whitespace. This arm drives the three doors whose re-run **embeds author prose** — the
/// milestone title, the sub-task intent, the rename target — with one title carrying all
/// three metacharacters, and then executes the lifted line **through a real `sh`** (a `jigc`
/// shim on `PATH`), because that is what an operator or agent does with a printed command
/// line. A re-run that expands `$HOME`, substitutes a backticked span, or runs the tail after
/// `;` as a second command can still **exit 0** — silently creating a *different* artifact
/// than the one the frame promised to recover.
const MESSY_PROSE: &str = "Cache $HOME `rework`; drop";

/// Every regular file under `docs/`, concatenated — the committed record surface the recovery
/// re-run must land the author's prose into, byte-for-byte.
fn docs_text(repo: &Path) -> String {
    fn walk(dir: &Path, out: &mut String) {
        let Ok(entries) = fs::read_dir(dir) else {
            return;
        };
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                walk(&path, out);
            } else if let Ok(body) = fs::read_to_string(&path) {
                out.push_str(&body);
                out.push('\n');
            }
        }
    }
    let mut out = String::new();
    walk(&repo.join("docs"), &mut out);
    out
}

/// A directory holding a `jigc` shim that execs the binary under test, so a lifted command
/// line starting with the bare word `jigc` runs as printed under a real shell.
fn shim_dir(home: &Path) -> PathBuf {
    let dir = home.join("shim-bin");
    fs::create_dir_all(&dir).expect("mk the shim dir");
    let shim = dir.join("jigc");
    fs::write(
        &shim,
        format!("#!/bin/sh\nexec {:?} \"$@\"\n", env!("CARGO_BIN_EXE_jigc")),
    )
    .expect("write the jigc shim");
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        fs::set_permissions(&shim, fs::Permissions::from_mode(0o755)).expect("chmod the shim");
    }
    dir
}

#[test]
fn the_printed_re_run_survives_a_real_shell_over_author_owned_prose() {
    for door in [
        "jigc milestone create",
        "jigc milestone add-task",
        "jigc rename",
    ] {
        let tag = door.rsplit(' ').next().expect("a door label");
        let (repo, home) = base_repo(&format!("prose-{tag}"), None);
        // `witness`: the bytes the recovered artifact must carry — the one the *promised*
        // artifact carries and an expanded / substituted / severed prose cannot.
        let (driven, witness): (Vec<String>, String) = match door {
            // The milestone record's identity is minted **from the title bytes**, so the
            // recovered record's own H1 is the witness: `$HOME` expanded mints a different id.
            "jigc milestone create" => (
                owned(&["milestone", "create", MESSY_PROSE]),
                format!("# {}", engine::milestone::mint_id(MESSY_PROSE)),
            ),
            // The sub-task row carries the intent prose verbatim.
            "jigc milestone add-task" => {
                jigc_ok(
                    repo.path(),
                    home.path(),
                    &["milestone", "create", "Cache rework"],
                    "`jigc milestone create`",
                );
                (
                    owned(&["milestone", "add-task", "cache-rework", MESSY_PROSE]),
                    format!("intent: {MESSY_PROSE}"),
                )
            }
            // The renamed doc's H1 becomes the new title verbatim.
            _ => {
                commit_adr(repo.path(), "alpha-decision", "Alpha decision", Some(2));
                (
                    owned(&["rename", "adr:alpha-decision", "--to", MESSY_PROSE]),
                    format!("# {MESSY_PROSE}"),
                )
            }
        };

        install_rejecting_hook(repo.path());
        let args: Vec<&str> = driven.iter().map(String::as_str).collect();
        let rejected = jigc(repo.path(), home.path(), &args, None);
        let stderr = String::from_utf8_lossy(&rejected.stderr).into_owned();
        assert!(
            !rejected.status.success(),
            "[{door}] the rejected run must fail; stderr:\n{stderr}",
        );

        // The lift is the contract: the printed span must still be delimited even though the
        // author's prose carries backticks of its own.
        let lifted = lift_rerun(&stderr);
        assert_eq!(
            shell_split(&lifted),
            {
                let mut want = vec!["jigc".to_string()];
                want.extend(driven.iter().cloned());
                want
            },
            "[{door}] the printed line must re-say the door's own argv, prose intact; \
             printed `{lifted}`",
        );

        // …and the emitted bytes, pasted into a shell exactly as printed, must recover.
        remove_hook(repo.path());
        let head_before = git(repo.path(), &["rev-parse", "HEAD"]);
        let path = format!(
            "{}:{}",
            shim_dir(home.path()).display(),
            std::env::var("PATH").unwrap_or_default(),
        );
        let recovered = Command::new("sh")
            .arg("-c")
            .arg(&lifted)
            .current_dir(repo.path())
            .env("HOME", home.path())
            .env("PATH", path)
            .env_remove("JIGC_PACK_DIR")
            .output()
            .expect("run the printed line through sh");
        assert!(
            recovered.status.success(),
            "[{door}] the printed re-run must recover under a real shell; printed `{lifted}`\n\
             stdout:\n{}\nstderr:\n{}",
            String::from_utf8_lossy(&recovered.stdout),
            String::from_utf8_lossy(&recovered.stderr),
        );
        assert_ne!(
            git(repo.path(), &["rev-parse", "HEAD"]),
            head_before,
            "[{door}] the recovery re-run must land its commit; printed `{lifted}`",
        );

        // The artifact it created is the one the frame promised. An expansion, a command
        // substitution or a `;`-severed tail all still exit 0 — while creating a *different*
        // artifact, which is exactly the silent wrong outcome this witness catches.
        let docs = docs_text(repo.path());
        assert!(
            docs.contains(&witness),
            "[{door}] the recovery must produce the promised artifact (expected \
             {witness:?} in the committed docs); printed `{lifted}`\ndocs:\n{docs}",
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

/// **The task door's state-truth clause, read on the branch git cannot corroborate** (M46
/// Increment 8 / T1; `design/surface-contract.md:136` — a statement about a surface is
/// quantified over what that surface actually serves, and the repair is **scope**, never a
/// behaviour change; the RC-1.0-gate finding B1-1).
///
/// The sweep above drives `jigc task finalize` on a fixture that `git add`s `code.txt`
/// first — the branch on which *"your staged changes are still staged"* is true in git's own
/// vocabulary. A **docs-only** task never touches git's index at all: its work lives in
/// `.jigc/tasks/<id>/docs/`, `git diff --cached` prints nothing at the moment of rejection,
/// and one sentence was covering both mechanisms with one word. That is the branch a
/// doc-review hook rejects most often, so it is the branch this arm drives: the clause must
/// name the area the task's work actually survives in, and the assertion is paired with the
/// git state the sentence is printed over — established live here, never assumed, because the
/// refused finalize runs four scoped rollback axes over the index before this text is emitted.
#[test]
fn the_task_door_names_its_own_staged_docs_over_an_empty_git_index() {
    let (repo, home) = base_repo("task-finalize-docs-only", None);
    let task = seed_task(repo.path(), home.path(), "record the eviction policy");
    // The task's whole contribution: one created-in-task ADR, staged in the task's working
    // area. Nothing is `git add`-ed, so git's index stays exactly as `jigc setup` left it.
    jigc_ok(
        repo.path(),
        home.path(),
        &["doc", "create", "adr", "--title", "Eviction policy"],
        "`jigc doc create adr`",
    );
    for slot in ["context", "decision", "consequences"] {
        let out = jigc(
            repo.path(),
            home.path(),
            &[
                "doc",
                "set-slot",
                &format!("adr:eviction-policy#{slot}"),
                "--from-file",
                "-",
            ],
            Some(format!("Prose for {slot}.\n").as_bytes()),
        );
        assert!(
            out.status.success(),
            "`jigc doc set-slot adr:eviction-policy#{slot}` must exit 0; stderr:\n{}",
            String::from_utf8_lossy(&out.stderr),
        );
    }

    let index_before = git(repo.path(), &["diff", "--cached", "--name-only"]);
    assert!(
        index_before.trim().is_empty(),
        "the fixture is docs-only: git's index must be empty going in; index:\n{index_before}",
    );

    install_rejecting_hook(repo.path());
    let rejected = jigc(repo.path(), home.path(), &["task", "finalize", &task], None);
    let stderr = String::from_utf8_lossy(&rejected.stderr).into_owned();
    assert!(
        !rejected.status.success(),
        "a hook rejection must exit non-zero; stdout:\n{}\nstderr:\n{stderr}",
        String::from_utf8_lossy(&rejected.stdout),
    );

    // The git state the clause is printed over — DRIVEN, not assumed: the finalize staged
    // its promotion and rolled the index back, so `git diff --cached` is empty here and a
    // reader who checks git for "their staged changes" finds nothing.
    let index_after = git(repo.path(), &["diff", "--cached", "--name-only"]);
    assert!(
        index_after.trim().is_empty(),
        "git's index is empty at the moment the frame is printed; index:\n{index_after}",
    );

    // The clause itself, read off the emitted bytes.
    let clause = stderr
        .lines()
        .find(|line| line.contains(&format!("task {task} is intact")))
        .unwrap_or_else(|| panic!("the frame must still say the task survives; stderr:\n{stderr}"));
    let area = format!(".jigc/tasks/{task}/docs/");
    assert!(
        clause.contains(&area),
        "the clause must name the task's OWN staged-doc area (`{area}`), the only place this \
         task's work survives; clause:\n{clause}",
    );
    assert!(
        !clause.contains("your staged changes are still staged"),
        "the clause must not borrow git's word for a state git's index does not hold; \
         clause:\n{clause}",
    );

    // …and the area it names genuinely holds the task's staged docs.
    let docs = repo
        .path()
        .join(".jigc")
        .join("tasks")
        .join(&task)
        .join("docs");
    let mut names: Vec<String> = fs::read_dir(&docs)
        .expect("read the task's staged-doc area")
        .flatten()
        .map(|entry| entry.file_name().to_string_lossy().into_owned())
        .collect();
    names.sort();
    assert!(
        names.iter().any(|n| n == "adr:eviction-policy.md"),
        "the named area must hold the staged ADR the rejection preserved; area holds {names:?}",
    );
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

// ── The second sweep: the same axis × the empty-commit outcome (M48 Increment 8 T2) ─────────

/// The **frame's assertion** — the clause that claims a rejection happened. Its appearance in a
/// cell where nobody rejected anything is the defect this sweep exists to catch, so it is
/// matched as the frame emits it rather than by a looser token that ordinary prose could carry.
const REJECTION_ASSERTION: &str = "was rejected (no commit was made)";

/// git's **own** empty-commit prose. It arrives on the seam as the same non-zero exit a hook
/// rejection does, so a door that relays it has not merely worded the frame badly — it has
/// mistaken git's refusal to record nothing for someone rejecting the run.
const GIT_EMPTY_COMMIT_PROSE: &str = "nothing to commit";

/// Settle the fixture: commit everything outstanding, so the cell's tree is clean and the door's
/// own commit is the only one that could record anything. A tree that is **already** clean is
/// left alone (`commit_adr` sweeps the whole tree on its way past), because git refuses the
/// empty commit that would otherwise be attempted here — the very refusal this sweep is about.
/// The post-condition is asserted either way, since a cell that starts dirty tests a different
/// question than the one it claims to.
fn commit_everything(repo: &Path, message: &str) {
    git(repo, &["add", "-A"]);
    if !git(repo, &["status", "--porcelain"]).trim().is_empty() {
        git(repo, &["commit", "-q", "-m", message]);
    }
    assert!(
        git(repo, &["status", "--porcelain"]).trim().is_empty(),
        "the empty-commit fixture must start from a clean tree",
    );
}

/// One door's **empty-commit cell**: the fixture state in which the commit that door would make
/// records nothing, the argv that drives it there, and what the door must do *instead of*
/// meeting git's refusal — its exit status, the substring of its own true diagnosis, and (where
/// the diagnosis is a routed finding rather than an inline code) the finding code the log carries.
struct EmptyCase {
    repo: TempDir,
    home: TempDir,
    driven: Vec<String>,
    /// The process exit code the cell must produce — `0` for the two doors that ack a no-op,
    /// the blocked/failure code for the doors that refuse.
    exit: i32,
    /// A substring of the door's **own** diagnosis, which must appear in what it printed.
    diagnosis: String,
    /// The finding code the invocation log must carry, for the cells whose refusal is a routed
    /// finding whose code the message itself does not spell.
    finding: Option<&'static str>,
}

/// Build each door's **empty-commit** fixture and drive the door at it. No rejecting hook is
/// installed anywhere here: the whole point is that nothing rejects these runs.
fn drive_empty(verb: &str) -> EmptyCase {
    match verb {
        // Nothing staged over a clean tree: the task validates and produces no diff, so the
        // engine's own guard blocks ahead of the boundary. (A *dirty* tree with an empty index
        // is the different, CLI-side `finalize.nothing-staged` block, which is why this fixture
        // settles the tree first.)
        "jigc task finalize" => {
            let (repo, home) = base_repo("empty-task-finalize", None);
            commit_everything(repo.path(), "settle the fixture");
            let task = seed_task(repo.path(), home.path(), "produce no diff");
            EmptyCase {
                driven: owned(&["task", "finalize", &task]),
                exit: 3,
                diagnosis: "finalize.empty-commit".to_string(),
                finding: Some("finalize.empty-commit"),
                repo,
                home,
            }
        }
        // A milestone with a sub-task that contributed neither a merged doc nor staged code:
        // the boundary would commit only jigc's own bookkeeping, and the zero-work refusal
        // stops it — the same seam for both commit models, above the `squash` branch.
        "jigc milestone finalize (squash: true)" | "jigc milestone finalize (squash: false)" => {
            let squash = if verb.ends_with("true)") {
                "true"
            } else {
                "false"
            };
            let (repo, home) = base_repo(&format!("empty-ms-finalize-{squash}"), Some(squash));
            commit_everything(repo.path(), "settle the fixture");
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
            EmptyCase {
                driven: owned(&["milestone", "finalize", "cache-rework"]),
                exit: 3,
                diagnosis: "would land no work".to_string(),
                finding: Some("milestone.zero-contribution"),
                repo,
                home,
            }
        }
        // The idempotent rename (M48 Inc 8 T1): a `--to` that slugs to the doc's own current
        // slug AND matches the H1 it already carries. An unrelated untracked file rides along,
        // because git's empty-commit refusal *lists* it — so an unguarded door does not merely
        // assert a rejection, it names a file that has nothing to do with the run.
        "jigc rename" => {
            let (repo, home) = base_repo("empty-rename", None);
            commit_adr(repo.path(), "alpha-decision", "Alpha decision", Some(2));
            commit_everything(repo.path(), "settle the fixture");
            fs::write(repo.path().join("scratch.txt"), "unrelated\n").expect("write scratch.txt");
            EmptyCase {
                driven: owned(&["rename", "adr:alpha-decision", "--to", "Alpha decision"]),
                exit: 0,
                diagnosis: "nothing renamed, nothing committed".to_string(),
                finding: None,
                repo,
                home,
            }
        }
        // An all-current corpus: the migration writes nothing back, so its staged-diff check
        // skips the commit and the run reports what it found.
        "jigc migrate-corpus" => {
            let (repo, home) = base_repo("empty-migrate-corpus", None);
            commit_adr(repo.path(), "alpha-decision", "Alpha decision", Some(2));
            commit_everything(repo.path(), "settle the fixture");
            EmptyCase {
                driven: owned(&["migrate-corpus"]),
                exit: 0,
                diagnosis: "0 migrated".to_string(),
                finding: None,
                repo,
                home,
            }
        }
        // The repeated `create` — the only route to a record write that would change nothing.
        // The identity guard refuses ahead of the write, so the empty commit is unreachable.
        "jigc milestone create" => {
            let (repo, home) = base_repo("empty-ms-create", None);
            commit_everything(repo.path(), "settle the fixture");
            jigc_ok(
                repo.path(),
                home.path(),
                &["milestone", "create", "Cache rework"],
                "`jigc milestone create`",
            );
            EmptyCase {
                driven: owned(&["milestone", "create", "Cache rework"]),
                exit: 1,
                diagnosis: "milestone.record-exists".to_string(),
                finding: None,
                repo,
                home,
            }
        }
        // The repeated `add-task`, same shape: the sub-task collision guard refuses ahead of
        // the append that would have re-written the record's own bytes.
        "jigc milestone add-task" => {
            let (repo, home) = base_repo("empty-ms-add-task", None);
            commit_everything(repo.path(), "settle the fixture");
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
            EmptyCase {
                driven: owned(&["milestone", "add-task", "cache-rework", "Area low"]),
                exit: 1,
                diagnosis: "milestone.sub-task-collision".to_string(),
                finding: None,
                repo,
                home,
            }
        }
        // The resumable door, re-run once every criterion is already seeded: it seeds nothing,
        // commits nothing, and says so — the record-only family's one genuinely reachable
        // empty-commit state.
        "jigc milestone add-from-spec" => {
            let (repo, home) = base_repo("empty-ms-add-from-spec", None);
            let specs = repo.path().join("docs").join("specs");
            fs::create_dir_all(&specs).expect("mk docs/specs/");
            fs::write(specs.join("rate-limit.md"), TWO_CRITERIA_SPEC).expect("write the spec");
            commit_everything(repo.path(), "settle the fixture");
            jigc_ok(
                repo.path(),
                home.path(),
                &["milestone", "create", "Rate limit"],
                "`jigc milestone create`",
            );
            jigc_ok(
                repo.path(),
                home.path(),
                &[
                    "milestone",
                    "add-from-spec",
                    "rate-limit",
                    "spec:rate-limit",
                ],
                "`jigc milestone add-from-spec`",
            );
            EmptyCase {
                driven: owned(&[
                    "milestone",
                    "add-from-spec",
                    "rate-limit",
                    "spec:rate-limit",
                ]),
                exit: 0,
                diagnosis: "seeded 0 sub-task(s)".to_string(),
                finding: None,
                repo,
                home,
            }
        }
        // The repeated `discard`: the milestone is already settled, so the terminal guard
        // refuses ahead of the status flip that would have re-written identical bytes.
        "jigc milestone discard" => {
            let (repo, home) = base_repo("empty-ms-discard", None);
            commit_everything(repo.path(), "settle the fixture");
            jigc_ok(
                repo.path(),
                home.path(),
                &["milestone", "create", "Cache rework"],
                "`jigc milestone create`",
            );
            jigc_ok(
                repo.path(),
                home.path(),
                &["milestone", "discard", "cache-rework"],
                "`jigc milestone discard`",
            );
            EmptyCase {
                driven: owned(&["milestone", "discard", "cache-rework"]),
                exit: 1,
                diagnosis: "milestone.terminal".to_string(),
                finding: None,
                repo,
                home,
            }
        }
        // The repeated sub-task `discard`: the first one settled the record item AND removed
        // the working area, so the repeat is refused at task resolution — ahead of the splice
        // that would have re-written identical bytes. The `milestone discard` cell's shape,
        // one level down.
        "jigc task discard" => {
            let (repo, home) = base_repo("empty-task-discard", None);
            commit_everything(repo.path(), "settle the fixture");
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
            jigc_ok(
                repo.path(),
                home.path(),
                &["task", "discard", "area-low"],
                "`jigc task discard`",
            );
            EmptyCase {
                driven: owned(&["task", "discard", "area-low"]),
                exit: 1,
                diagnosis: "no task `area-low`".to_string(),
                finding: None,
                repo,
                home,
            }
        }
        other => panic!(
            "`{other}` is a committing door with no **empty-commit** cell in this suite — the \
             axis is the code-side `COMMITTING_DOORS` table, so a door added there owes its \
             cell here as well as its rejection arm",
        ),
    }
}

/// The empty-commit sweep: every code-side committing door, driven through the real binary into
/// the state where the commit it would make records nothing, with **no hook installed anywhere**.
#[test]
fn no_committing_door_dresses_an_empty_commit_as_a_rejection() {
    assert_eq!(
        COMMITTING_DOORS.len(),
        10,
        "the axis is 10 doors + `jigc setup` excluded by its recorded `--no-verify` reason",
    );

    for door in COMMITTING_DOORS {
        let verb = door.verb;
        let case = drive_empty(verb);
        let repo = case.repo.path();
        let home = case.home.path();
        let driven: Vec<&str> = case.driven.iter().map(String::as_str).collect();

        let head_before = git(repo, &["rev-parse", "HEAD"]);
        let out = jigc(repo, home, &driven, None);
        let stdout = String::from_utf8_lossy(&out.stdout).into_owned();
        let stderr = String::from_utf8_lossy(&out.stderr).into_owned();
        let printed = format!("{stdout}{stderr}");

        // (1) the door lands where its own design says it lands — exit 0 for the two doors that
        // ack a no-op, the blocked/failure code for the doors that refuse ahead of the boundary.
        assert_eq!(
            out.status.code(),
            Some(case.exit),
            "[{verb}] the empty-commit cell must exit {}; stdout:\n{stdout}\nstderr:\n{stderr}",
            case.exit,
        );

        // (2) nothing was committed — in every cell, whichever side of the boundary it stopped on.
        assert_eq!(
            git(repo, &["rev-parse", "HEAD"]),
            head_before,
            "[{verb}] the empty-commit cell must leave HEAD untouched; printed:\n{printed}",
        );

        // (3) the door states its OWN true diagnosis.
        assert!(
            printed.contains(&case.diagnosis),
            "[{verb}] the door must state its own diagnosis (expected {:?}); printed:\n{printed}",
            case.diagnosis,
        );

        // (4) …and never claims someone rejected the run, nor relays git's refusal to record
        // nothing as if it were that rejection.
        assert!(
            !printed.contains(REJECTION_ASSERTION),
            "[{verb}] nobody rejected this run — the frame's assertion must be absent; \
             printed:\n{printed}",
        );
        assert!(
            !printed.contains(GIT_EMPTY_COMMIT_PROSE),
            "[{verb}] git's own empty-commit prose must not be relayed as a rejection; \
             printed:\n{printed}",
        );
        assert!(
            !printed.contains(door.error_code),
            "[{verb}] the rejection identity `{}` must not appear over a run nobody rejected; \
             printed:\n{printed}",
            door.error_code,
        );

        // (5) the log carries no rejection identity either — the release-build-visible half of
        // the claim, since the `Outcome::error` membership check is a compiled-out `debug_assert!`.
        let records = log_records(repo);
        let record = record_for(&records, &case.driven)
            .unwrap_or_else(|| panic!("[{verb}] the run must be logged; records:\n{records:#?}"));
        let logged = record["error_code"].as_str();
        assert!(
            !logged.is_some_and(|code| code.ends_with(".commit-rejected")),
            "[{verb}] the log must carry no `*.commit-rejected` identity over a run nobody \
             rejected; got {record}",
        );
        assert_ne!(
            logged,
            Some(door.error_code),
            "[{verb}] the log must not carry this door's rejection identity; got {record}",
        );
        assert_eq!(
            record["exit_code"].as_i64(),
            Some(i64::from(case.exit)),
            "[{verb}] the logged exit must be the one the door actually took; got {record}",
        );
        if let Some(code) = case.finding {
            let codes: Vec<&str> = record["finding_codes"]
                .as_array()
                .map(|a| a.iter().filter_map(serde_json::Value::as_str).collect())
                .unwrap_or_default();
            assert!(
                codes.contains(&code),
                "[{verb}] the refusal must be logged as its own finding `{code}`; got {record}",
            );
        }
    }
}

// ── The third sweep: the same axis × the NON-HOOK failure (M51 Increment 2 / T5 — N20) ──────

/// The frame's **hook diagnosis** — the one sentence `render::commit_rejected` hard-codes. In a
/// cell where no hook was installed at all it is a law-1 lie *and* a dead-end route ("satisfy a
/// hook that never spoke"), so its absence is what this sweep's cells assert.
const HOOK_DIAGNOSIS: &str = "Fix the hook's complaint";

/// Plant a **stale `.git/index.lock`** — the ordinary residue of a crashed or killed git, and
/// the one non-hook cause every door that stages before it commits meets identically: `git add`
/// (and `git mv`) fail with git's own `Unable to create '…/.git/index.lock'`, inside the door's
/// commit transaction and after its rollback has run.
///
/// It is **not** a hook: no hook is installed in any cell of this sweep, so nothing in the
/// repository can reject anything — which is exactly what makes the frame's hook sentence a lie
/// here and the door's *identity* the thing that must survive.
fn plant_stale_index_lock(repo: &Path) {
    fs::write(repo.join(".git").join("index.lock"), "").expect("plant a stale .git/index.lock");
}

/// Stage a sub-task's authored `commit:<sub>` doc — the prose the `squash: false` per-sub-task
/// render reads, and the doc a provisioned sub-task owes before the boundary will commit it.
fn stage_subtask_commit(repo: &Path, sub: &str, summary: &str) {
    stage_subtask_doc(
        repo,
        sub,
        &format!("commit:{sub}"),
        &format!(
            "---\ntype: feat\n---\n\n# {sub}\n\n## Summary\n\n{summary}\n\n## Body\n\n\n\n\
             ## Trailers\n"
        ),
    );
}

/// The **headline repro**'s fixture (charter N20 / EC-37): a provisioned one-sub-task fan-out
/// whose worktree holds staged code at `src/low.rs`, plus **ordinary untracked WIP** at that
/// same path in the main checkout — so the boundary's closing `git merge --ff-only` refuses to
/// overwrite it (the carry-or-refuse contract) with **no hook anywhere**. Driven at both
/// `finalize.fan-out.squash` settings, because EC-37's correction to N20's recorded scope is
/// that the frame loss is **knob-independent**: both arms reach the one seam
/// (`overlay_docs_commit_and_ff`) that lands the commit on the live checkout.
fn seed_ff_refused_fan_out(repo: &Path, home: &Path) {
    jigc_ok(
        repo,
        home,
        &["milestone", "create", "Cache rework"],
        "`jigc milestone create`",
    );
    jigc_ok(
        repo,
        home,
        &["milestone", "add-task", "cache-rework", "Area low"],
        "`jigc milestone add-task`",
    );
    stage_subtask_doc(
        repo,
        "area-low",
        "adr:low-policy",
        &adr_body("Low policy", None),
    );
    stage_subtask_commit(repo, "area-low", "rework the low cache path");
    jigc_ok(
        repo,
        home,
        &["milestone", "provision", "cache-rework"],
        "`jigc milestone provision`",
    );
    // The sub-agent's staged code, in its isolated worktree.
    let worktree = repo.join(".jigc").join("worktrees").join("area-low");
    fs::create_dir_all(worktree.join("src")).expect("mk the worktree's src/");
    fs::write(worktree.join("src").join("low.rs"), "pub fn low() {}\n")
        .expect("write the worktree's code");
    git(&worktree, &["add", "src/low.rs"]);
    // The human's untracked WIP at the same path in the MAIN checkout — the collision.
    fs::create_dir_all(repo.join("src")).expect("mk src/");
    fs::write(repo.join("src").join("low.rs"), "// untracked human WIP\n")
        .expect("seed the colliding untracked WIP");
}

/// The committed milestone record's repo-relative path (docs-root-nested by the dev
/// `docs-root` knob the `[dev ▸ methodology]` composition resolves).
fn record_rel(milestone_id: &str) -> String {
    format!("docs/milestone-records/{milestone_id}.md")
}

/// One door's **non-hook cell**: the fixture it was driven in, the argv that drove it, the
/// re-run its frame must print, the state clause it must carry, and the **independent** check
/// of that clause against the repository's actual post-refusal state.
///
/// The independent check is the point of G-47: a clause is reusable across the hook and
/// non-hook cells only where it is *true* in both, and the only way to know is to look at the
/// repo rather than at the sentence. Where the shipped clause is false for a non-hook failure
/// (`migrate-corpus`: the stage is exactly what failed, so nothing is staged), the door owes a
/// second clause and this arm asserts *that* one.
struct NonHookCase {
    repo: TempDir,
    home: TempDir,
    driven: Vec<String>,
    expected_rerun: Vec<String>,
    survived: String,
    /// Assert the clause against the repo's real state after the refusal. Takes the repo root.
    verify: Box<dyn Fn(&Path)>,
}

/// Build each door's **non-hook** fixture and drive the door at it. **No hook is installed in
/// any cell** — every failure here is git's own or jigc's own, never a rejection.
fn drive_non_hook(verb: &str) -> NonHookCase {
    match verb {
        // The promote destination is a regular FILE, so phase 4's `create_dir_all` fails inside
        // the commit closure — after the plan validated, before anything is staged or committed.
        "jigc task finalize" => {
            let (repo, home) = base_repo("nonhook-task-finalize", None);
            let task = seed_task(repo.path(), home.path(), "record the eviction policy");
            jigc_ok(
                repo.path(),
                home.path(),
                &["doc", "create", "adr", "--title", "Eviction policy"],
                "`jigc doc create adr`",
            );
            for slot in ["context", "decision", "consequences"] {
                let out = jigc(
                    repo.path(),
                    home.path(),
                    &[
                        "doc",
                        "set-slot",
                        &format!("adr:eviction-policy#{slot}"),
                        "--from-file",
                        "-",
                    ],
                    Some(format!("Prose for {slot}.\n").as_bytes()),
                );
                assert!(out.status.success(), "`jigc doc set-slot` must exit 0");
            }
            fs::create_dir_all(repo.path().join("docs")).expect("mk docs/");
            fs::write(
                repo.path().join("docs").join("decisions"),
                "not a directory\n",
            )
            .expect("block the promote destination with a regular file");
            let area = repo.path().join(".jigc").join("tasks").join(&task);
            NonHookCase {
                survived: format!("task {task} is intact"),
                expected_rerun: owned(&["jigc", "task", "finalize", &task]),
                driven: owned(&["task", "finalize", &task]),
                verify: Box::new(move |_repo| {
                    assert!(
                        area.join("docs").join("adr:eviction-policy.md").is_file(),
                        "the task's staged ADR must still be in its working area",
                    );
                }),
                repo,
                home,
            }
        }
        // The headline repro, at both commit models: `git merge --ff-only` refuses over ordinary
        // untracked main-checkout WIP, with no hook installed anywhere.
        "jigc milestone finalize (squash: true)" | "jigc milestone finalize (squash: false)" => {
            let squash = if verb.ends_with("true)") {
                "true"
            } else {
                "false"
            };
            let (repo, home) = base_repo(&format!("nonhook-ms-finalize-{squash}"), Some(squash));
            seed_ff_refused_fan_out(repo.path(), home.path());
            NonHookCase {
                survived: "milestone:cache-rework is intact".to_string(),
                expected_rerun: owned(&["jigc", "milestone", "finalize", "cache-rework"]),
                driven: owned(&["milestone", "finalize", "cache-rework"]),
                verify: Box::new(|repo| {
                    // EC-37's state half: the record is left `active`, never a `joined` record
                    // for a milestone that never finalized.
                    let record = fs::read_to_string(repo.join(record_rel("cache-rework")))
                        .expect("the committed record survives the refusal");
                    assert!(
                        record.contains("status: active"),
                        "the record must be left `active`; record:\n{record}",
                    );
                    // …and the provisioned worktree still holds its staged code.
                    let staged = git(
                        &repo.join(".jigc").join("worktrees").join("area-low"),
                        &["diff", "--cached", "--name-only"],
                    );
                    assert!(
                        staged.lines().any(|l| l == "src/low.rs"),
                        "the provisioned worktree must still hold its staged code; staged:\n\
                         {staged}",
                    );
                    // …and the merged doc was rolled back out of the live checkout.
                    assert!(
                        !repo
                            .join("docs")
                            .join("decisions")
                            .join("low-policy.md")
                            .exists(),
                        "the merged docs must be rolled back",
                    );
                }),
                repo,
                home,
            }
        }
        // `git mv` meets the stale lock, inside the atomic rename transaction.
        "jigc rename" => {
            let (repo, home) = base_repo("nonhook-rename", None);
            commit_adr(repo.path(), "alpha-decision", "Alpha decision", Some(2));
            plant_stale_index_lock(repo.path());
            NonHookCase {
                survived: "adr:alpha-decision".to_string(),
                expected_rerun: owned(&[
                    "jigc",
                    "rename",
                    "adr:alpha-decision",
                    "--to",
                    "Beta decision",
                ]),
                driven: owned(&["rename", "adr:alpha-decision", "--to", "Beta decision"]),
                verify: Box::new(|repo| {
                    let decisions = repo.join("docs").join("decisions");
                    assert!(
                        decisions.join("alpha-decision.md").is_file(),
                        "the doc must still hold its original identity",
                    );
                    assert!(
                        !decisions.join("beta-decision.md").exists(),
                        "the rename must have been rolled back",
                    );
                }),
                repo,
                home,
            }
        }
        // The stage is exactly what fails — so this door's hook-cell clause ("written and
        // staged") is FALSE here, and the cell asserts the door's own non-hook clause instead.
        "jigc migrate-corpus" => {
            let (repo, home) = base_repo("nonhook-migrate-corpus", None);
            commit_adr(repo.path(), "alpha-decision", "Alpha decision", None);
            plant_stale_index_lock(repo.path());
            NonHookCase {
                survived: "the migrated bytes are written to disk".to_string(),
                expected_rerun: owned(&["jigc", "migrate-corpus"]),
                driven: owned(&["migrate-corpus"]),
                verify: Box::new(|repo| {
                    let path = repo
                        .join("docs")
                        .join("decisions")
                        .join("alpha-decision.md");
                    let bytes = fs::read_to_string(&path).expect("the migrated doc is on disk");
                    assert!(
                        bytes.contains("schema-version:"),
                        "the migrated bytes must be written to disk; doc:\n{bytes}",
                    );
                    let staged = git(repo, &["diff", "--cached", "--name-only"]);
                    assert!(
                        !staged
                            .lines()
                            .any(|l| l == "docs/decisions/alpha-decision.md"),
                        "the stage is what failed, so nothing of it may be staged; staged:\n\
                         {staged}",
                    );
                }),
                repo,
                home,
            }
        }
        "jigc milestone create" => {
            let (repo, home) = base_repo("nonhook-ms-create", None);
            plant_stale_index_lock(repo.path());
            NonHookCase {
                survived: "nothing of milestone:cache-rework survives".to_string(),
                expected_rerun: owned(&["jigc", "milestone", "create", "Cache rework"]),
                driven: owned(&["milestone", "create", "Cache rework"]),
                verify: Box::new(|repo| {
                    assert!(
                        !repo.join(record_rel("cache-rework")).exists(),
                        "the record write must have been rolled back",
                    );
                    assert!(
                        !repo
                            .join(".jigc")
                            .join("milestones")
                            .join("cache-rework")
                            .exists(),
                        "the minted workbench must have been unwound",
                    );
                }),
                repo,
                home,
            }
        }
        "jigc milestone add-task" => {
            let (repo, home) = base_repo("nonhook-ms-add-task", None);
            jigc_ok(
                repo.path(),
                home.path(),
                &["milestone", "create", "Cache rework"],
                "`jigc milestone create`",
            );
            let before = fs::read_to_string(repo.path().join(record_rel("cache-rework")))
                .expect("the record exists before the append");
            plant_stale_index_lock(repo.path());
            NonHookCase {
                survived: "milestone:cache-rework is unchanged".to_string(),
                expected_rerun: owned(&[
                    "jigc",
                    "milestone",
                    "add-task",
                    "cache-rework",
                    "Area low",
                ]),
                driven: owned(&["milestone", "add-task", "cache-rework", "Area low"]),
                verify: Box::new(move |repo| {
                    let after = fs::read_to_string(repo.join(record_rel("cache-rework")))
                        .expect("the record survives");
                    assert_eq!(
                        after, before,
                        "the record append must have been rolled back"
                    );
                    assert!(
                        !repo.join(".jigc").join("tasks").join("area-low").exists(),
                        "the sub-task mint must have been unwound",
                    );
                }),
                repo,
                home,
            }
        }
        "jigc milestone add-from-spec" => {
            let (repo, home) = base_repo("nonhook-ms-add-from-spec", None);
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
            let before = fs::read_to_string(repo.path().join(record_rel("rate-limit")))
                .expect("the record exists before the seed");
            plant_stale_index_lock(repo.path());
            NonHookCase {
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
                verify: Box::new(move |repo| {
                    let after = fs::read_to_string(repo.join(record_rel("rate-limit")))
                        .expect("the record survives");
                    assert_eq!(
                        after, before,
                        "the refused append must have been rolled back, so the record still \
                         names exactly the sub-tasks it named",
                    );
                    assert!(
                        !repo
                            .join(".jigc")
                            .join("tasks")
                            .join("rejects-burst")
                            .exists(),
                        "the un-recorded mint must have been unwound",
                    );
                }),
                repo,
                home,
            }
        }
        "jigc milestone discard" => {
            let (repo, home) = base_repo("nonhook-ms-discard", None);
            jigc_ok(
                repo.path(),
                home.path(),
                &["milestone", "create", "Cache rework"],
                "`jigc milestone create`",
            );
            let before = fs::read_to_string(repo.path().join(record_rel("cache-rework")))
                .expect("the record exists before the discard");
            plant_stale_index_lock(repo.path());
            NonHookCase {
                survived: "milestone:cache-rework's record is still at its pre-discard state"
                    .to_string(),
                expected_rerun: owned(&["jigc", "milestone", "discard", "cache-rework"]),
                driven: owned(&["milestone", "discard", "cache-rework"]),
                verify: Box::new(move |repo| {
                    let after = fs::read_to_string(repo.join(record_rel("cache-rework")))
                        .expect("the record survives");
                    assert_eq!(after, before, "the record must be at its pre-discard state");
                    assert!(
                        repo.join(".jigc")
                            .join("milestones")
                            .join("cache-rework")
                            .exists(),
                        "the workbench must be untouched",
                    );
                }),
                repo,
                home,
            }
        }
        "jigc task discard" => {
            let (repo, home) = base_repo("nonhook-task-discard", None);
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
            plant_stale_index_lock(repo.path());
            NonHookCase {
                survived: "the task's working area is intact".to_string(),
                expected_rerun: owned(&["jigc", "task", "discard", "area-low", "--force"]),
                driven: owned(&["task", "discard", "area-low", "--force"]),
                verify: Box::new(|repo| {
                    assert!(
                        repo.join(".jigc").join("tasks").join("area-low").exists(),
                        "the task's working area must be intact",
                    );
                }),
                repo,
                home,
            }
        }
        other => panic!(
            "`{other}` is a committing door with no NON-HOOK arm in this suite — the axis is \
             the code-side `COMMITTING_DOORS` table, so a door added there owes its arm here",
        ),
    }
}

/// The non-hook sweep (N20): every code-side committing door, driven through the real binary
/// into a failure **inside its commit transaction that no hook caused** — the class
/// `surface_commit_rejection` used to drop on the floor, falling through to the plain
/// operational envelope and discarding a `RejectionFrame` its caller had already built in full
/// (`error_code: null`, no route, no state clause; `completions/artifacts/M51/charter.md` → N20,
/// `gap-findings.md` → G-47).
///
/// Per door: the run fails loudly and commits nothing, the log carries **that door's** identity
/// (read off the record, never the printed text — the `Outcome::error` membership check is a
/// compiled-out `debug_assert!` in the release build the trials run), the frame states what
/// survived and the arm **verifies that clause against the repository itself**, exactly **one**
/// route is printed and it is this door's own re-run, and nothing anywhere blames a hook —
/// because no hook exists in any cell of this sweep.
#[test]
fn every_committing_door_keeps_its_frame_when_no_hook_spoke() {
    assert_eq!(
        COMMITTING_DOORS.len(),
        10,
        "the axis is 10 doors + `jigc setup` excluded by its recorded `--no-verify` reason",
    );

    for door in COMMITTING_DOORS {
        let verb = door.verb;
        let case = drive_non_hook(verb);
        let repo = case.repo.path();
        let home = case.home.path();
        let driven: Vec<&str> = case.driven.iter().map(String::as_str).collect();

        let head_before = git(repo, &["rev-parse", "HEAD"]);
        let failed = jigc(repo, home, &driven, None);
        let stdout = String::from_utf8_lossy(&failed.stdout).into_owned();
        let stderr = String::from_utf8_lossy(&failed.stderr).into_owned();
        let printed = format!("{stdout}{stderr}");

        // (1) the run fails loudly, and nothing landed.
        assert!(
            !failed.status.success(),
            "[{verb}] a non-hook commit-transaction failure must exit non-zero; \
             stdout:\n{stdout}\nstderr:\n{stderr}",
        );
        assert_eq!(
            git(repo, &["rev-parse", "HEAD"]),
            head_before,
            "[{verb}] nothing was committed, so HEAD must be untouched; printed:\n{printed}",
        );

        // (2) NO hook is blamed — none was installed, so the hook diagnosis is both a law-1
        // lie and a route that cannot be followed.
        assert!(
            !printed.contains(HOOK_DIAGNOSIS),
            "[{verb}] no hook spoke in this cell — the hook diagnosis must be absent; \
             printed:\n{printed}",
        );
        assert!(
            !printed.contains(REJECTION_ASSERTION),
            "[{verb}] nobody rejected this run — the rejection assertion must be absent; \
             printed:\n{printed}",
        );

        // (3) the state clause, and the state it claims — checked against the repo, not the
        // sentence (G-47: a clause written for the hook cell can be false in this one).
        assert!(
            stderr.contains(&case.survived),
            "[{verb}] the frame must state what survived (expected to name `{}`); \
             stderr:\n{stderr}",
            case.survived,
        );
        (case.verify)(repo);

        // (4) exactly ONE route, and it is this door's own re-run — lifted verbatim out of the
        // emitted bytes, never rebuilt in the test.
        assert_eq!(
            stderr.matches("then re-run ").count(),
            1,
            "[{verb}] the frame must print exactly one route; stderr:\n{stderr}",
        );
        let lifted = lift_rerun(&stderr);
        assert_eq!(
            shell_split(&lifted),
            case.expected_rerun,
            "[{verb}] the frame must name the door's OWN re-run; printed `{lifted}`",
        );

        // (5) the door's identity is in the INVOCATION LOG — the release-visible half, and the
        // whole of N20: the frame was built, and then thrown away before it could be recorded.
        let records = log_records(repo);
        let record = record_for(&records, &case.driven).unwrap_or_else(|| {
            panic!("[{verb}] the failed run must be logged; records:\n{records:#?}")
        });
        assert_eq!(
            record["error_code"].as_str(),
            Some(door.error_code),
            "[{verb}] the log must carry THIS door's identity; got {record}",
        );
        assert_eq!(
            record["finding_codes"].as_array().map(Vec::len),
            Some(0),
            "[{verb}] a non-hook commit failure is an operational error, not a Finding; \
             got {record}",
        );
    }
}
