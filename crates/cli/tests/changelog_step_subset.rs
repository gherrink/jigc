//! M42 Increment 11, T2 — the changelog paragraph becomes its own step, included
//! only where the create-gate is granted, and the false "lists the gates" sentence
//! is retired.
//!
//! The rc.5 adoption trial found `step:implement` instructing
//! `jigc doc create changelog` under **every** workflow that includes it — while
//! only `single-task` grants the `{type: changelog, as: change}` create-gate. Under
//! `implement-from-spec` and `sub-task` the binary **refuses** the very command the
//! composed prompt tells the agent to run (`create.gate-blocked`), and the prompt's
//! one discovery hint — *"bare `jigc start` lists the gates it grants"* — is false
//! (bare `start` prints the workflow catalog and names no gate anywhere).
//!
//! The fix is **step-subset inclusion** ([workflow-dialect.md](../../../design/workflow-dialect.md)),
//! never a conditional (the dialect bars them): the paragraph moves to
//! `step:record-changelog`, included by `single-task` alone.
//!
//! Proven on the **emitted bytes** through the real binary against the shipped dev
//! pack (`JIGC_PACK_DIR` = the tree that ships):
//!   - `single-task` composes exactly **one** `jigc doc create changelog` line, and
//!     that emitted line — run **verbatim** — is *admitted* by the gate (exit 0);
//!   - `implement-from-spec` and `sub-task` compose **zero** such lines (the
//!     omitting contexts: the step is absent where the gate is absent);
//!   - no composed output — and no source byte of **either** shipped pack — carries
//!     the retired `lists the gates` sentence.
//!
//! The gate's own admit/refuse behaviour is unchanged and stays pinned by
//! `single_task_changelog_gate.rs`; this suite pins *what the pack tells the agent
//! to run*.
//!
//! **M47 Increment 8, T3 (N9)** extends that claim from *which* instruction composes
//! to *whether following it produces anything*: the step now solicits through the
//! `{{schema:changelog}}` seam, and the third arm below drives its emitted lines end
//! to end to a real, committed changelog entry.

use crate::support::root_walk;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

/// A throwaway directory that removes itself on drop.
struct TempDir(PathBuf);

impl TempDir {
    fn new(tag: &str) -> Self {
        let mut path = std::env::temp_dir();
        let unique = format!(
            "jigc-changelog-step-subset-{tag}-{}-{:?}",
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

/// Run a `jigc` subcommand exactly as [`run_jigc`], feeding `stdin` to the process
/// — the `--from-file -` shape every authoring verb the step emits reads its payload
/// through.
fn run_jigc_stdin(
    repo: &Path,
    home: &Path,
    pack: &Path,
    args: &[&str],
    stdin: &str,
) -> std::process::Output {
    use std::io::Write;

    let mut child = Command::new(env!("CARGO_BIN_EXE_jigc"))
        .args(args)
        .current_dir(repo)
        .env("HOME", home)
        .env("JIGC_PACK_DIR", pack)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("spawn the jigc binary");
    child
        .stdin
        .as_mut()
        .expect("piped stdin")
        .write_all(stdin.as_bytes())
        .expect("write the payload to stdin");
    child.wait_with_output().expect("collect jigc output")
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

/// Compose a workflow through the real binary — **through whichever door composes it**.
///
/// `jigc start --workflow <w> <intent>` for an ordinary workflow; for a **verb-routed**
/// one, the door its `suppressed:` block declares, because the compose-by-name doors
/// refuse it since M52 Increment 9 / T2. `sub-task` is the member this suite reaches:
/// its door is the fanned sub-agent's re-entry, which needs a provisioned worktree —
/// [`fanned_sub_task`] stands one up. Composing it any other way would mean dropping
/// the one omitting context whose body the claim is about.
fn compose(repo: &Path, home: &Path, pack: &Path, workflow: &str, intent: &str) -> String {
    if declares_a_door(pack, workflow) {
        let (sub, worktree) = fanned_sub_task(repo, home, pack, intent);
        return ok_stdout(
            run_jigc(
                &worktree,
                home,
                pack,
                &["workflow", workflow, "--task", &sub],
            ),
            &format!("jigc workflow {workflow} --task {sub}"),
        );
    }
    ok_stdout(
        run_jigc(repo, home, pack, &["start", "--workflow", workflow, intent]),
        &format!("jigc start --workflow {workflow}"),
    )
}

/// Whether `workflow`'s `suppressed:` block declares the `door:` it is reached through
/// — read from the pack under test, so this helper and the binary cannot disagree.
fn declares_a_door(pack: &Path, workflow: &str) -> bool {
    let path = pack.join("workflows").join(format!("{workflow}.yaml"));
    let Ok(bytes) = fs::read(&path) else {
        return false;
    };
    engine::compose::load_workflow_def(&bytes)
        .unwrap_or_else(|f| panic!("`{workflow}` must load: {}", f.message))
        .suppressed
        .and_then(|s| s.door)
        .is_some()
}

/// A milestone with one provisioned sub-task — the state `sub-task`'s door needs. The
/// worktree is where a fanned sub-agent re-enters from: the main checkout's HEAD moves
/// ahead of the shared base pin when the milestone record commits, by design.
fn fanned_sub_task(repo: &Path, home: &Path, pack: &Path, intent: &str) -> (String, PathBuf) {
    ok_stdout(
        run_jigc(repo, home, pack, &["milestone", "create", "Door fixture"]),
        "jigc milestone create",
    );
    let added = ok_stdout(
        run_jigc(
            repo,
            home,
            pack,
            &[
                "milestone",
                "add-task",
                "door-fixture",
                intent,
                "--workflow",
                "sub-task",
            ],
        ),
        "jigc milestone add-task",
    );
    ok_stdout(
        run_jigc(
            repo,
            home,
            pack,
            &["milestone", "provision", "door-fixture"],
        ),
        "jigc milestone provision",
    );
    // The id the binary printed, never a test-side re-slug of the intent: the ack is
    // `added task:<id> to milestone:<m>`.
    let sub = added
        .lines()
        .find_map(|line| line.trim().strip_prefix("added task:"))
        .and_then(|rest| rest.split_whitespace().next())
        .expect("`milestone add-task` acks the task it added")
        .to_owned();
    let worktree = repo.join(".jigc").join("worktrees").join(&sub);
    (sub, worktree)
}

/// The command a composed line instructs, stripped of the command-ref frame the
/// renderer wraps a `{{cli.<id>}}` reference in (``Run: `<cmd>` ``) — a step may
/// solicit either through the ref or as a bare literal, and both are the same
/// instruction to the agent that reads it.
fn instructed_command(line: &str) -> &str {
    let line = line.trim();
    let line = line.strip_prefix("Run: ").unwrap_or(line);
    line.strip_prefix('`')
        .and_then(|rest| rest.strip_suffix('`'))
        .unwrap_or(line)
}

/// Every emitted line that instructs a `jigc doc create changelog`, verbatim.
fn changelog_create_lines(composed: &str) -> Vec<&str> {
    composed
        .lines()
        .map(instructed_command)
        .filter(|line| line.starts_with("jigc doc create changelog"))
        .collect()
}

/// The core done-criterion: the changelog instruction composes **only** where the
/// create-gate is granted, and the emitted line is the one the binary admits.
#[test]
fn the_changelog_instruction_composes_only_where_the_gate_is_granted() {
    let repo = TempDir::new("repo");
    let home = TempDir::new("home");
    let pack = dev_pack();
    init_repo(repo.path());

    ok_stdout(
        run_jigc(repo.path(), home.path(), &pack, &["setup"]),
        "jigc setup",
    );

    // ── The granting context: `single-task` alone carries `{type: changelog, as: change}`.
    let single = compose(
        repo.path(),
        home.path(),
        &pack,
        "single-task",
        "add a rate limiter",
    );
    let lines = changelog_create_lines(&single);
    assert_eq!(
        lines.len(),
        1,
        "`single-task` grants the changelog gate — it must compose exactly one \
         `jigc doc create changelog` line; got {lines:?} in:\n{single}",
    );

    // The EMITTED line is the contract: run it verbatim. A prompt whose literal
    // command the binary refuses is the defect this task exists to kill.
    let emitted = lines[0];
    let argv: Vec<&str> = emitted.split_whitespace().collect();
    assert_eq!(argv[0], "jigc", "the emitted line invokes the binary");
    let created = run_jigc(repo.path(), home.path(), &pack, &argv[1..]);
    assert!(
        created.status.success(),
        "the emitted line `{emitted}` must be ADMITTED by the gate it is composed \
         under; stderr:\n{}",
        String::from_utf8_lossy(&created.stderr),
    );

    // ── The omitting contexts: neither workflow grants the gate, so neither may
    // instruct the create at all (today both do — and the binary refuses it).
    for workflow in ["implement-from-spec", "sub-task"] {
        let composed = compose(
            repo.path(),
            home.path(),
            &pack,
            workflow,
            &format!("work the {workflow} arm"),
        );
        let lines = changelog_create_lines(&composed);
        assert!(
            lines.is_empty(),
            "`{workflow}` does not grant the changelog create-gate — it must compose \
             ZERO `jigc doc create changelog` lines (the binary refuses them with \
             `create.gate-blocked`); got {lines:?} in:\n{composed}",
        );
    }
}

/// The false discovery hint is **retired**, not rewritten: bare `jigc start` prints
/// the workflow catalog and names no gate anywhere, and a step included only where
/// the gate exists needs no hint. Pinned on the emitted bytes of the three
/// `step:implement` workflows *and* exhaustively over both shipped pack trees — the
/// only bytes from which the sentence could ever reach a composed view.
#[test]
fn no_composed_workflow_and_no_pack_source_claims_start_lists_the_gates() {
    let repo = TempDir::new("repo");
    let home = TempDir::new("home");
    let pack = dev_pack();
    init_repo(repo.path());

    ok_stdout(
        run_jigc(repo.path(), home.path(), &pack, &["setup"]),
        "jigc setup",
    );

    for workflow in ["single-task", "implement-from-spec", "sub-task"] {
        let composed = compose(
            repo.path(),
            home.path(),
            &pack,
            workflow,
            &format!("compose the {workflow} view"),
        );
        assert!(
            !composed.contains("lists the gates"),
            "`{workflow}` must not compose the false gates-discovery sentence; got:\n{composed}",
        );
    }

    let files: Vec<(PathBuf, String)> = [cli::pack_path!(dev), cli::pack_path!(methodology)]
        .iter()
        .flat_map(|tree| root_walk::files(Path::new(tree), |_| true))
        .filter_map(|path| fs::read_to_string(&path).ok().map(|text| (path, text)))
        .collect();
    let offenders: Vec<&PathBuf> = files
        .iter()
        .filter(|(_, text)| text.contains("lists the gates"))
        .map(|(path, _)| path)
        .collect();
    assert!(
        offenders.is_empty(),
        "no shipped pack source may claim bare `jigc start` lists the gates it grants \
         (it does not); offenders: {offenders:?}",
    );
}

// ─────────────────────────────────────────────────────────────────────────────
// M47 Increment 8, T3 (N9) — the step yields a REAL changelog, not an empty one
// that validates clean.
//
// At HEAD the step was four lines naming only `doc create`, over a nested
// repeatable: an agent following it verbatim staged the bare skeleton — no
// change-group, no notes — and `jigc task validate` said exit 0, so the first
// changelog a greenfield operator wrote recorded nothing. The fix routes the
// authoring through the `{{schema:changelog}}` seam (schema + fillable batch
// payload, generated — never hand-written address grammar, the thing M43
// Increment 3 deleted from twelve templates) and states the copy-in/append
// constraint above the solicit, declaring `create.singleton-copy-in`.
//
// Proven on the EMITTED BYTES: the composed step's own `doc create` line and its
// own `doc author` invocation are extracted and run verbatim, and the payload is
// the emitted skeleton with only the `<…>` values filled in — the prose half is
// the agent's, the structure is the pack's. A test that hand-built the payload
// would pass over a step that emits nothing at all.
// ─────────────────────────────────────────────────────────────────────────────

/// The emitted batch-author invocation and its emitted payload skeleton: the
/// `jigc doc author changelog …` line (minus the shell heredoc marker) plus the
/// body the step prints between `<<'EOF'` and `EOF`.
fn emitted_author_call(composed: &str) -> (Vec<String>, String) {
    let lines: Vec<&str> = composed.lines().collect();
    let idx = lines
        .iter()
        .position(|line| instructed_command(line).starts_with("jigc doc author changelog"))
        .unwrap_or_else(|| {
            panic!(
                "the composed `single-task` view must emit a `jigc doc author changelog` \
                 invocation — an agent following the step has to be able to WRITE the \
                 entry, not just create the empty doc; got:\n{composed}"
            )
        });
    let argv: Vec<String> = instructed_command(lines[idx])
        .split_whitespace()
        .take_while(|token| !token.starts_with("<<"))
        .map(str::to_owned)
        .collect();
    let body = idx + 1;
    let end = body
        + lines[body..]
            .iter()
            .position(|line| line.trim_end() == "EOF")
            .expect("the emitted heredoc must terminate at a bare `EOF` line");
    (argv, lines[body..end].join("\n"))
}

/// Fill the emitted payload the way the step directs an agent to: keep the
/// `unreleased-changes` entry (a routine change is staged, not cut), omit the
/// `releases` entry, and replace the `<…>` placeholders with real values. Nothing
/// structural is authored here — every key, indent and `<<…>>` marker is the
/// emitted skeleton's.
fn fill_emitted_payload(payload: &str, note: &str) -> String {
    let cut = payload.find("\n  - id: releases").unwrap_or_else(|| {
        panic!("the emitted payload must carry the `releases` entry to omit; got:\n{payload}")
    });
    let filled = payload[..cut]
        .replace(
            "\"<added | changed | deprecated | removed | fixed | security>\"",
            "\"added\"",
        )
        .replace(
            "<<One bullet per change in this category.>>",
            &format!("<<{note}>>"),
        );
    assert!(
        !filled.contains("\"<") && !filled.contains("<<One bullet"),
        "every `<…>` placeholder in the emitted skeleton must be fillable by literal \
         substitution; left over in:\n{filled}"
    );
    format!("{filled}\n")
}

/// The `--task <id>` value the emitted lines carry — the task the composed view was
/// minted for, read off the emitted bytes rather than re-slugged in test code.
fn emitted_task_id(argv: &[String]) -> String {
    let at = argv
        .iter()
        .position(|token| token == "--task")
        .expect("the emitted invocation must carry `--task`");
    argv[at + 1].clone()
}

/// The done-criterion: follow the composed step's own emitted lines and end with a
/// real `unreleased-changes` group carrying notes, a clean `jigc task validate`, and
/// a finalize that lands `CHANGELOG.md` with that entry.
#[test]
fn following_the_composed_changelog_step_yields_a_real_changelog_entry() {
    let repo = TempDir::new("real-entry");
    let home = TempDir::new("real-entry-home");
    let pack = dev_pack();
    init_repo(repo.path());

    ok_stdout(
        run_jigc(repo.path(), home.path(), &pack, &["setup"]),
        "jigc setup",
    );

    let composed = compose(
        repo.path(),
        home.path(),
        &pack,
        "single-task",
        "rate-limit the public API",
    );

    // 1 — the emitted create line, run verbatim.
    let create = changelog_create_lines(&composed);
    assert_eq!(create.len(), 1, "exactly one create line; got {create:?}");
    let create_argv: Vec<&str> = create[0].split_whitespace().collect();
    let created = run_jigc(repo.path(), home.path(), &pack, &create_argv[1..]);
    assert!(
        created.status.success(),
        "the emitted create line `{}` must be admitted; stderr:\n{}",
        create[0],
        String::from_utf8_lossy(&created.stderr),
    );

    // 2 — the emitted author invocation + its emitted payload, run verbatim.
    let (author_argv, skeleton) = emitted_author_call(&composed);
    let task = emitted_task_id(&author_argv);
    let note = "- The public API now rate-limits by client key.";
    let payload = fill_emitted_payload(&skeleton, note);
    let args: Vec<&str> = author_argv[1..].iter().map(String::as_str).collect();
    let authored = run_jigc_stdin(repo.path(), home.path(), &pack, &args, &payload);
    assert!(
        authored.status.success(),
        "the emitted author invocation `{}` must be admitted under `single-task`'s \
         changelog create-gate; payload:\n{payload}\nstderr:\n{}",
        author_argv.join(" "),
        String::from_utf8_lossy(&authored.stderr),
    );

    // 3 — the staged doc carries a REAL group with notes, not the empty skeleton.
    let staged = fs::read_to_string(
        repo.path()
            .join(".jigc")
            .join("tasks")
            .join(&task)
            .join("docs")
            .join("changelog:changelog.md"),
    )
    .expect("the task stages the changelog it authored");
    for expected in ["## Unreleased Changes", "{#added}", note] {
        assert!(
            staged.contains(expected),
            "the staged changelog must carry a real unreleased group WITH notes — \
             missing {expected:?} in:\n{staged}"
        );
    }

    // 4 — the task validates clean. (The commit doc is the workflow's other
    // required write; filling it is fixture work, not the claim under test.)
    ok_stdout(
        run_jigc(
            repo.path(),
            home.path(),
            &pack,
            &[
                "doc",
                "set-field",
                &format!("commit:{task}#header/type"),
                "--value",
                "feat",
                "--task",
                &task,
            ],
        ),
        "set the commit type",
    );
    let summary = run_jigc_stdin(
        repo.path(),
        home.path(),
        &pack,
        &[
            "doc",
            "set-slot",
            &format!("commit:{task}#summary"),
            "--from-file",
            "-",
            "--task",
            &task,
        ],
        "rate-limit the public API",
    );
    assert!(summary.status.success(), "set the commit summary");

    let validated = run_jigc(
        repo.path(),
        home.path(),
        &pack,
        &["task", "validate", &task],
    );
    assert!(
        validated.status.success(),
        "`jigc task validate {task}` must be clean after following the step; stdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&validated.stdout),
        String::from_utf8_lossy(&validated.stderr),
    );

    // 5 — finalize lands `CHANGELOG.md` with that entry, committed.
    let finalized = run_jigc(
        repo.path(),
        home.path(),
        &pack,
        &["task", "finalize", &task],
    );
    assert!(
        finalized.status.success(),
        "`jigc task finalize {task}` must land the changelog; stdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&finalized.stdout),
        String::from_utf8_lossy(&finalized.stderr),
    );
    let committed = Command::new("git")
        .args(["show", "HEAD:CHANGELOG.md"])
        .current_dir(repo.path())
        .output()
        .expect("run git show");
    let committed = String::from_utf8_lossy(&committed.stdout).into_owned();
    assert!(
        committed.contains(note) && committed.contains("{#added}"),
        "the committed `CHANGELOG.md` must carry the authored entry; got:\n{committed}"
    );

    // The N9 second half: `record-change`'s suppression reason claims routine change
    // recording "already rides `single-task`'s record-changelog step". The claim is
    // true of the step this test just drove end-to-end — and it stays anchored to it:
    // the reason must keep naming the step, byte-unchanged.
    let reason = fs::read_to_string(cli::pack_path!(dev, "workflows/record-change.yaml"))
        .expect("read the record-change workflow");
    assert!(
        reason.contains(
            "routine change recording already rides `single-task`'s record-changelog step"
        ),
        "the suppression reason must keep naming the step whose truth this test proves; got:\n{reason}"
    );
}
