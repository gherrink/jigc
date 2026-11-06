//! **`dev/stabilize-guard` and `dev/stabilize-fence` — the write guard of a stabilization
//! run** ([DECISIONS.md](../DECISIONS.md) → *2026-10-16 — The write guard of a
//! stabilization run*; [stabilization-workflow.md](../implementation/stabilization-workflow.md)
//! → *The write guard*).
//!
//! In the staged canary's third root a driver's ad-hoc probe ran with an empty `R`, so its
//! commands ran in the run's repository: it committed there and reset the tree, and nothing
//! in the tools prevented it. The human ruled that the guard prevents: every agent of a run
//! but its writers is fenced — a PreToolUse hook refuses a file-tool write into the
//! repository or its git directory, lets a plain call of the three tools through unfenced,
//! refuses such a call in any other shape, and runs every other command under the
//! operating system's write fence, failing closed where none can be built. This suite
//! feeds the hook the JSON a PreToolUse event carries and reads its decision, and drives
//! the fence itself:
//!
//! - who is never fenced, and when the guard says nothing at all
//!   ([`the_main_session_and_a_writer_are_never_fenced`],
//!   [`the_guard_is_inert_outside_a_run`]);
//! - a file-tool write, by every spelling of a path into the repository
//!   ([`a_fenced_agents_write_into_the_repository_or_its_git_directory_is_refused`]);
//! - the three tools, plain and otherwise
//!   ([`a_plain_call_of_the_three_tools_passes_unfenced_from_the_repositorys_root`],
//!   [`a_call_of_a_tool_in_another_shape_is_refused_with_the_shape_it_is_taken_in`]);
//! - every other command, into the fence or refused
//!   ([`every_other_command_of_a_fenced_agent_runs_under_the_fence_or_not_at_all`]);
//! - what the guard cannot read, and the tools it cannot fence
//!   ([`an_unknown_type_is_fenced_and_what_the_guard_cannot_read_is_refused`],
//!   [`a_shell_the_guard_cannot_fence_and_a_writer_launched_by_a_fenced_agent_are_refused`]);
//! - while a runtime probe is active — its marker standing in the repository's common git
//!   directory (the plan before the opening's task T-NR) — every subagent, the writers
//!   too, makes only the one act of the probe tool the harness launched it for, under the
//!   marker's own scratch root, writes only the files the marker names, reads, and does
//!   nothing else; a stale, old or unreadable marker binds and says so, and a guard that
//!   cannot tell whether one stands refuses every subagent
//!   ([`while_a_probe_is_active_every_subagent_runs_only_its_own_act_of_the_probe_tool_and_writes_only_the_files_the_probe_names`],
//!   [`the_probe_acts_a_subagent_may_make_are_the_ones_the_harness_launches_it_for`],
//!   [`a_stale_or_unreadable_probe_marker_binds_and_says_so`],
//!   [`where_the_guard_cannot_tell_whether_a_probe_is_active_every_subagent_is_refused_the_writers_too`]);
//! - the fence, on this host: macOS's sandbox where there is one, a refusal where there is
//!   none — the hosted runner's tooling legs are Linux, and there the arm holds the refusal
//!   ([`the_fence_refuses_every_write_under_the_repository_or_runs_nothing`]);
//! - and the classification against the harness and the settings
//!   ([`the_guard_fences_every_type_the_harness_launches_but_its_writers`],
//!   [`the_settings_run_the_guard_as_its_header_gives_it`]).
//!
//! **Proved red on applied mutants**, each applied alone against the arm that names its
//! fence, none surviving: the repository check of a file-tool write dropped; a path
//! compared by its spelling instead of its inode; every Bash command taken as plain; the
//! shape refusal dropped; the guard bound on every branch; the fence's deny of the git
//! directory dropped; a missing sandbox run open; a writer added to the guard's list.
//!
//! **What this suite cannot show** is the runtime: that Claude Code fires the hook for the
//! agents a Workflow script launches in an interactive session, and honours its decision
//! there. A headless session was driven once by the builder (the DECISIONS.md entry); the
//! staged canary's act `part-0` is the probe in the clone.

use std::collections::BTreeSet;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

use serde_json::{Value, json};

use crate::support::child_stdin;
use crate::support::scratch::ScratchDir;

const GUARD: &str = "dev/stabilize-guard";
const FENCE: &str = "dev/stabilize-fence";
const HARNESS: &str = ".claude/workflow-sources/stabilize.js";
const SETTINGS: &str = ".claude/settings.json";
const RUN: &str = "rig-run";

/// The types the harness launches that are fenced — every one but the writers. A type the
/// harness starts to launch, or stops launching, is a change of this list, named.
const FENCED: &[&str] = &[
    "build-executor",
    "finding-triage",
    "finding-verifier",
    "general-purpose",
    "milestone-code-reviewer",
    "milestone-e2e-tester",
    "robust-advocate",
    "stabilize-preflight",
    "stabilize-scope",
];
/// The writers, each with why it writes the run's repository.
const WRITERS: &[(&str, &str)] = &[
    (
        "build-git",
        "the git steps: their acts write the run's record, commit it and push it",
    ),
    ("build-fixer", "a fixer commits its fix on the round branch"),
];

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .ancestors()
        .nth(2)
        .expect("repo root is two levels above crates/cli")
        .to_path_buf()
}

fn git(at: &Path, args: &[&str]) -> String {
    let out = Command::new("git")
        .arg("-C")
        .arg(at)
        .args([
            "-c",
            "user.name=rig",
            "-c",
            "user.email=rig@example.com",
            "-c",
            "commit.gpgsign=false",
        ])
        .args(args)
        .env("GIT_CONFIG_NOSYSTEM", "1")
        .env("GIT_CONFIG_GLOBAL", "/dev/null")
        .output()
        .expect("git runs");
    assert!(
        out.status.success(),
        "git {args:?} in {}: {}",
        at.display(),
        String::from_utf8_lossy(&out.stderr)
    );
    String::from_utf8_lossy(&out.stdout).trim().to_string()
}

/// A run's repository: a git work tree on its loop branch, its opening record committed —
/// and, beside it under the same scratch directory, room for what lies outside it.
struct Rig {
    scratch: ScratchDir,
    repo: PathBuf,
}

impl Rig {
    fn on(branch: &str, opened: bool) -> Rig {
        let scratch = ScratchDir::new("stabilize-guard");
        let repo = scratch.path().join("repo");
        fs::create_dir_all(&repo).expect("the repository's directory");
        git(&repo, &["init", "-q", "-b", branch]);
        if opened {
            let run = repo.join("completions/artifacts").join(RUN);
            fs::create_dir_all(&run).expect("the run's directory");
            fs::write(run.join("opening.md"), "# the opening\n").expect("the opening record");
        }
        fs::write(repo.join("README.md"), "a rig\n").expect("a file");
        git(&repo, &["add", "-A"]);
        git(&repo, &["commit", "-q", "-m", "the opening"]);
        Rig { scratch, repo }
    }

    fn run() -> Rig {
        Rig::on(&format!("fix/{RUN}"), true)
    }

    fn outside(&self) -> PathBuf {
        let path = self.scratch.path().join("outside");
        fs::create_dir_all(&path).expect("a directory outside the repository");
        path
    }

    /// The hook, fed `event`, with the project the repository — unless `project` says
    /// otherwise — and `env` beside it.
    fn hook(&self, event: &Value, env: &[(&str, &str)]) -> Hooked {
        self.hook_in(Some(&self.repo), event, env)
    }

    fn hook_in(&self, project: Option<&Path>, event: &Value, env: &[(&str, &str)]) -> Hooked {
        let mut command = Command::new(repo_root().join(GUARD));
        command
            .env_remove("CLAUDE_PROJECT_DIR")
            .env_remove("STABILIZE_FENCE_SANDBOX");
        if let Some(project) = project {
            command.env("CLAUDE_PROJECT_DIR", project);
        }
        for (name, value) in env {
            command.env(name, value);
        }
        let mut child = command
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .expect("spawn the guard");
        child_stdin::feed(&mut child, event.to_string());
        let out = child.wait_with_output().expect("the guard exits");
        Hooked {
            code: out.status.code().unwrap_or(-1),
            stdout: String::from_utf8_lossy(&out.stdout).to_string(),
            stderr: String::from_utf8_lossy(&out.stderr).to_string(),
        }
    }
}

/// A PreToolUse event as Claude Code hands it to a hook: `agent` None is the session's own.
fn event(agent: Option<&str>, tool: &str, input: Value, cwd: &Path) -> Value {
    let mut event = json!({
        "session_id": "s",
        "transcript_path": "/dev/null",
        "cwd": cwd,
        "permission_mode": "bypassPermissions",
        "hook_event_name": "PreToolUse",
        "tool_name": tool,
        "tool_input": input,
        "tool_use_id": "toolu_1",
    });
    if let Some(agent) = agent {
        event["agent_type"] = json!(agent);
        event["agent_id"] = json!("a1");
    }
    event
}

fn write_event(agent: Option<&str>, path: &Path, cwd: &Path) -> Value {
    event(
        agent,
        "Write",
        json!({ "file_path": path, "content": "x" }),
        cwd,
    )
}

fn bash_event(agent: Option<&str>, command: &str, cwd: &Path) -> Value {
    event(
        agent,
        "Bash",
        json!({ "command": command, "description": "a command", "timeout": 120000 }),
        cwd,
    )
}

struct Hooked {
    code: i32,
    stdout: String,
    stderr: String,
}

impl Hooked {
    /// The guard says nothing: the call goes on as the session's mode has it.
    fn silent(&self, what: &str) {
        assert_eq!(
            (self.code, self.stdout.as_str()),
            (0, ""),
            "{what}: the guard says nothing — stderr: {}",
            self.stderr
        );
    }

    fn decision(&self, what: &str) -> Value {
        assert_eq!(
            self.code, 0,
            "{what}: the guard decides and exits 0 — stderr: {}",
            self.stderr
        );
        let line: Value = serde_json::from_str(self.stdout.trim())
            .unwrap_or_else(|e| panic!("{what}: one JSON object, not {:?}: {e}", self.stdout));
        assert_eq!(self.stdout.lines().count(), 1, "{what}: ONE line");
        let out = line["hookSpecificOutput"].clone();
        assert_eq!(out["hookEventName"], "PreToolUse", "{what}: {line}");
        out
    }

    /// The guard refuses, with `word`: the reason opens with it.
    fn refused(&self, word: &str, what: &str) -> String {
        let out = self.decision(what);
        assert_eq!(out["permissionDecision"], "deny", "{what}: refused — {out}");
        assert!(
            out.get("updatedInput").is_none(),
            "{what}: a refusal rewrites nothing"
        );
        let reason = out["permissionDecisionReason"]
            .as_str()
            .expect("a reason")
            .to_string();
        assert!(
            reason.starts_with(&format!("stabilize-guard: refused {word}: ")),
            "{what}: refused `{word}` — {reason}"
        );
        assert!(!reason.contains('\n'), "{what}: the reason is one line");
        reason
    }
}

fn real(path: &Path) -> PathBuf {
    fs::canonicalize(path).expect("the path is there")
}

#[test]
fn the_main_session_and_a_writer_are_never_fenced() {
    let rig = Rig::run();
    let inside = rig.repo.join("x");
    for agent in [None, Some(""), Some("build-git"), Some("build-fixer")] {
        let what = format!("{agent:?}");
        rig.hook(&write_event(agent, &inside, &rig.repo), &[])
            .silent(&format!("a write into the repository by {what}"));
        rig.hook(
            &bash_event(
                agent,
                "echo x > x; git commit --allow-empty -m x",
                &rig.repo,
            ),
            &[],
        )
        .silent(&format!("a command by {what}"));
        rig.hook(
            &bash_event(
                agent,
                "cd x && dev/stabilize-record kit; echo rc=$?",
                &rig.repo,
            ),
            &[],
        )
        .silent(&format!("a tool call in another shape by {what}"));
        rig.hook(
            &event(agent, "Monitor", json!({ "command": "true" }), &rig.repo),
            &[],
        )
        .silent(&format!("a monitor by {what}"));
    }
}

#[test]
fn the_guard_is_inert_outside_a_run() {
    let agent = Some("milestone-e2e-tester");
    for (branch, opened, why) in [
        (
            "milestone/findings-channel/main",
            true,
            "a milestone's branch",
        ),
        (
            "fix/other-run",
            false,
            "a fix branch whose run is not opened",
        ),
        ("fix/rc24-tier1", false, "a fix branch with no run"),
        ("work/branch-per-milestone", true, "a work branch"),
    ] {
        let rig = Rig::on(branch, opened);
        rig.hook(&write_event(agent, &rig.repo.join("x"), &rig.repo), &[])
            .silent(why);
        rig.hook(&bash_event(agent, "echo x > x", &rig.repo), &[])
            .silent(why);
    }
    // The run's own directory, on a branch that names another slug, binds nothing.
    let rig = Rig::on("fix/another", true);
    rig.hook(&write_event(agent, &rig.repo.join("x"), &rig.repo), &[])
        .silent("a branch that is not the run's");
    // A detached HEAD is no loop branch.
    let rig = Rig::run();
    git(&rig.repo, &["switch", "-q", "-d", "HEAD"]);
    rig.hook(&write_event(agent, &rig.repo.join("x"), &rig.repo), &[])
        .silent("a detached HEAD");
    // A project that is no git repository holds no run.
    let plain = rig.outside();
    rig.hook_in(
        Some(&plain),
        &write_event(agent, &plain.join("x"), &plain),
        &[],
    )
    .silent("no repository");
    // A round's branch and a part's are the run's.
    for branch in [format!("fix/{RUN}-r2"), format!("fix/{RUN}-r2-part1")] {
        let rig = Rig::on(&branch, true);
        rig.hook(&write_event(agent, &rig.repo.join("x"), &rig.repo), &[])
            .refused("repository", &branch);
    }
}

#[test]
fn a_fenced_agents_write_into_the_repository_or_its_git_directory_is_refused() {
    let rig = Rig::run();
    let agent = Some("finding-verifier");
    let outside = rig.outside();
    let alias = rig.scratch.path().join("alias");
    std::os::unix::fs::symlink(&rig.repo, &alias).expect("a link to the repository");
    let mut into = vec![
        ("a file of the tree", rig.repo.join("x")),
        ("a tracked file", rig.repo.join("README.md")),
        ("the git configuration", rig.repo.join(".git/config")),
        ("a hook", rig.repo.join(".git/hooks/pre-commit")),
        (
            "a new file whose parents are not there",
            rig.repo.join("new/dir/file"),
        ),
        (
            "the run's directory",
            rig.repo
                .join("completions/artifacts")
                .join(RUN)
                .join("verdicts/x.json"),
        ),
        ("through a link", alias.join("x")),
        (
            "through a link, into the git directory",
            alias.join(".git/config"),
        ),
        (
            "with `..` that climbs back in",
            outside.join("..").join("repo").join("x"),
        ),
        ("as the real path", real(&rig.repo).join("x")),
    ];
    // macOS: the temporary directory is reached through `/var`, which is `/private/var`.
    let given = rig.repo.to_string_lossy().to_string();
    if let Some(rest) = given.strip_prefix("/private/var/") {
        into.push(("through /var", PathBuf::from(format!("/var/{rest}/x"))));
    } else if let Some(rest) = given.strip_prefix("/var/") {
        into.push((
            "through /private/var",
            PathBuf::from(format!("/private/var/{rest}/x")),
        ));
    }
    // A volume that ignores the case of a name: the same directory by another spelling.
    let shouted = rig.repo.with_file_name("REPO");
    if shouted.exists() {
        into.push(("by another case", shouted.join("x")));
    }
    for (why, path) in &into {
        let reason = rig
            .hook(&write_event(agent, path, &rig.repo), &[])
            .refused("repository", why);
        assert!(
            reason.contains("finding-verifier")
                && reason.contains(RUN)
                && reason.contains("dev/stabilize-record"),
            "{why}: the reason names the agent, the run and the road a report takes: {reason}"
        );
        let edit = event(
            agent,
            "Edit",
            json!({ "file_path": path, "old_string": "a", "new_string": "b" }),
            &rig.repo,
        );
        rig.hook(&edit, &[])
            .refused("repository", &format!("an edit {why}"));
        let notebook = event(
            agent,
            "NotebookEdit",
            json!({ "notebook_path": path, "new_source": "x" }),
            &rig.repo,
        );
        rig.hook(&notebook, &[])
            .refused("repository", &format!("a notebook {why}"));
    }
    // A relative path is the agent's directory's.
    rig.hook(
        &write_event(agent, Path::new("relative.txt"), &rig.repo),
        &[],
    )
    .refused("repository", "a relative path from the repository");
    rig.hook(
        &write_event(agent, Path::new("relative.txt"), &outside),
        &[],
    )
    .silent("a relative path from outside");
    // Outside: the scratch root, a sibling, a link that leads out.
    let out_link = rig.repo.join("out-link");
    std::os::unix::fs::symlink(&outside, &out_link).expect("a link out of the repository");
    for (why, path) in [
        ("the scratch root", outside.join("report.md")),
        (
            "a new directory beside the repository",
            rig.scratch.path().join("repo-sibling/x"),
        ),
        (
            "a name that only begins like the repository",
            PathBuf::from(format!("{}-other/x", rig.repo.display())),
        ),
        (
            "`..` that climbs out",
            rig.repo.join("..").join("outside").join("x"),
        ),
    ] {
        rig.hook(&write_event(agent, &path, &rig.repo), &[])
            .silent(why);
    }
    // A link IN the repository that leads out is the place it leads to.
    rig.hook(&write_event(agent, &out_link.join("x"), &rig.repo), &[])
        .silent("a link in the repository that leads out");
}

#[test]
fn a_plain_call_of_the_three_tools_passes_unfenced_from_the_repositorys_root() {
    let rig = Rig::run();
    let agent = Some("finding-verifier");
    for command in [
        "dev/stabilize-record kit --run rig-run --role verify",
        "dev/stabilize-record verdict --run rig-run --round 1 --reporter verify-a --attempt 2 --scratch /tmp/s --from /tmp/s/f.json",
        "dev/stabilize-step hash --scratch /tmp/s --file bin/candidate-0123456789ab",
        "dev/stabilize-probe hold --scratch /tmp/s --name b --seconds 60",
        "dev/stabilize-record report --run rig-run --round 1 --stage test --reporter r --attempt 1 --from '/tmp/a dir (2)/r.md'",
        "dev/stabilize-record",
        "dev/stabilize-record kit --run rig-run --role verify\n",
    ] {
        rig.hook(&bash_event(agent, command, &rig.repo), &[])
            .silent(command);
        let mut background = bash_event(agent, command, &rig.repo);
        background["tool_input"]["run_in_background"] = json!(true);
        rig.hook(&background, &[])
            .silent(&format!("{command}, in the background"));
    }
    // The real path of the repository, as the agent's directory, is the repository's root.
    rig.hook(
        &bash_event(
            agent,
            "dev/stabilize-record kit --run rig-run --role verify",
            &real(&rig.repo),
        ),
        &[],
    )
    .silent("from the real path");
    // From any other directory the same line names another file: refused.
    let elsewhere = rig.outside();
    let reason = rig
        .hook(
            &bash_event(
                agent,
                "dev/stabilize-record kit --run rig-run --role verify",
                &elsewhere,
            ),
            &[],
        )
        .refused("directory", "from another directory");
    assert!(
        reason.contains("dev/stabilize-record")
            && reason.contains(&real(&rig.repo).display().to_string()),
        "it names the tool and the root: {reason}"
    );
}

#[test]
fn a_call_of_a_tool_in_another_shape_is_refused_with_the_shape_it_is_taken_in() {
    let rig = Rig::run();
    let agent = Some("finding-verifier");
    let absolute = format!(
        "{}/dev/stabilize-probe hold --scratch /tmp/s --name b --seconds 1",
        rig.repo.display()
    );
    for command in [
        "cd /some/clone && dev/stabilize-record verdict --run rig-run --from f.json; echo rc=$?",
        "dev/stabilize-record verdict --run rig-run --from f.json; echo rc=$?",
        "dev/stabilize-record kit --run rig-run --role verify 2>&1",
        "dev/stabilize-record kit --run rig-run --role verify | tail -5",
        "dev/stabilize-record kit --run \"$RUN\"",
        "./dev/stabilize-step hash --scratch /tmp/s --file f",
        absolute.as_str(),
        "python3 dev/stabilize-record kit --run rig-run",
        "RUN=rig-run dev/stabilize-record kit --run rig-run",
        "true && dev/stabilize-step state --run rig-run",
        "(dev/stabilize-record kit --run rig-run)",
        "x=$(dev/stabilize-record kit --run rig-run)",
        "if true; then dev/stabilize-record kit --run rig-run; fi",
    ] {
        let reason = rig
            .hook(&bash_event(agent, command, &rig.repo), &[])
            .refused("shape", command);
        let tool = [
            "dev/stabilize-record",
            "dev/stabilize-step",
            "dev/stabilize-probe",
        ]
        .into_iter()
        .find(|tool| command.contains(tool))
        .expect("a tool");
        assert!(
            reason.contains(&format!("`{tool} <its words>`")),
            "{command}: the refusal names the shape the call is taken in: {reason}"
        );
        for named in ["`cd`", "`&&`", "`echo`", "pipe", "redirect", "`$`"] {
            assert!(
                reason.contains(named),
                "{command}: the refusal says what a plain call holds no {named} of: {reason}"
            );
        }
    }
    // A tool named as a file to read is no call of it: the command runs under the fence.
    for command in [
        "grep -n verdict dev/stabilize-record",
        "sed -n 1,40p dev/stabilize-step",
        "cat run.sh dev/stabilize-probe",
    ] {
        let out = rig.hook(&bash_event(agent, command, &rig.repo), &[]);
        if cfg!(target_os = "macos") {
            let decided = out.decision(command);
            assert_eq!(
                decided["permissionDecision"], "allow",
                "{command}: fenced, not refused — {decided}"
            );
        } else {
            out.refused("no-fence", command);
        }
    }
}

#[test]
fn every_other_command_of_a_fenced_agent_runs_under_the_fence_or_not_at_all() {
    let rig = Rig::run();
    let agent = Some("milestone-e2e-tester");
    let command = "echo hi > x; git -C . commit --allow-empty -m 'a stray'";
    let fence = repo_root().join(FENCE);
    if cfg!(target_os = "macos") {
        // bypassPermissions: allowed, as the fenced command — nothing else of the input moved.
        let mut asked = bash_event(agent, command, &rig.repo);
        asked["tool_input"]["run_in_background"] = json!(true);
        let decided = rig.hook(&asked, &[]).decision("the rewrite");
        assert_eq!(decided["permissionDecision"], "allow", "{decided}");
        let updated = &decided["updatedInput"];
        let quoted = |text: &str| format!("'{}'", text.replace('\'', "'\"'\"'"));
        assert_eq!(
            updated["command"].as_str().expect("a command"),
            format!(
                "{} --repo {} -- {}",
                fence.display(),
                real(&rig.repo).display(),
                quoted(command)
            ),
            "ONE invocation of the fence tool, the command as ONE argument"
        );
        for kept in ["description", "timeout", "run_in_background"] {
            assert_eq!(updated[kept], asked["tool_input"][kept], "`{kept}` is kept");
        }
        // The rewritten line, run by a shell, runs the fence with exactly the command.
        let shown = Command::new("/bin/sh")
            .arg("-c")
            .arg(format!(
                "python3 -c 'import sys; print(repr(sys.argv[1:]))' {}",
                &updated["command"].as_str().unwrap()[fence.display().to_string().len()..]
            ))
            .output()
            .expect("sh runs");
        assert_eq!(
            String::from_utf8_lossy(&shown.stdout).trim(),
            format!(
                "['--repo', '{}', '--', \"{}\"]",
                real(&rig.repo).display(),
                command
            ),
            "the shell hands the fence the command unchanged"
        );
        // Any other mode is asked, with the fenced command shown.
        for mode in ["default", "acceptEdits", "auto", "dontAsk", "plan"] {
            let mut asked = bash_event(agent, command, &rig.repo);
            asked["permission_mode"] = json!(mode);
            let decided = rig.hook(&asked, &[]).decision(mode);
            assert_eq!(
                decided["permissionDecision"], "ask",
                "{mode}: asked — {decided}"
            );
            assert_eq!(
                decided["updatedInput"]["command"], updated["command"],
                "{mode}: the fenced command"
            );
        }
        // No sandbox: refused, never run open.
        let reason = rig
            .hook(
                &bash_event(agent, command, &rig.repo),
                &[("STABILIZE_FENCE_SANDBOX", "/nonexistent/sandbox-exec")],
            )
            .refused("no-fence", "no sandbox");
        assert!(reason.contains("none is run open"), "{reason}");
    } else {
        let reason = rig
            .hook(&bash_event(agent, command, &rig.repo), &[])
            .refused("no-fence", "a host with no fence");
        assert!(
            reason.contains("no macOS") && reason.contains("none is run open"),
            "{reason}"
        );
    }
}

#[test]
fn an_unknown_type_is_fenced_and_what_the_guard_cannot_read_is_refused() {
    let rig = Rig::run();
    // A type nobody classified is fenced.
    for agent in ["Explore", "some-new-role", "claude"] {
        rig.hook(
            &write_event(Some(agent), &rig.repo.join("x"), &rig.repo),
            &[],
        )
        .refused("repository", agent);
    }
    let agent = Some("finding-triage");
    // No project: refused, never passed.
    rig.hook_in(
        None,
        &write_event(agent, &rig.outside().join("x"), &rig.repo),
        &[],
    )
    .refused("unknown", "no project");
    // A file tool without its path, a shell without its command.
    rig.hook(
        &event(agent, "Write", json!({ "content": "x" }), &rig.repo),
        &[],
    )
    .refused("unknown", "a write with no path");
    rig.hook(
        &event(
            agent,
            "NotebookEdit",
            json!({ "new_source": "x" }),
            &rig.repo,
        ),
        &[],
    )
    .refused("unknown", "a notebook with no path");
    rig.hook(
        &event(agent, "Bash", json!({ "description": "x" }), &rig.repo),
        &[],
    )
    .refused("unknown", "a command that is not there");
    rig.hook(&bash_event(agent, "  ", &rig.repo), &[])
        .refused("unknown", "an empty command");
    // An input that is no JSON object: blocked by its exit status, with why on stderr.
    for garbage in ["", "not json", "[1, 2]"] {
        let mut child = Command::new(repo_root().join(GUARD))
            .env("CLAUDE_PROJECT_DIR", &rig.repo)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .expect("spawn the guard");
        child_stdin::feed(&mut child, garbage);
        let out = child.wait_with_output().expect("the guard exits");
        assert_eq!(out.status.code(), Some(2), "{garbage:?}: blocked");
        assert!(
            String::from_utf8_lossy(&out.stderr).starts_with("stabilize-guard: refused unknown: "),
            "{garbage:?}"
        );
    }
}

#[test]
fn a_shell_the_guard_cannot_fence_and_a_writer_launched_by_a_fenced_agent_are_refused() {
    let rig = Rig::run();
    let agent = Some("robust-advocate");
    for tool in ["Monitor", "PowerShell"] {
        rig.hook(
            &event(
                agent,
                tool,
                json!({ "command": "true", "description": "x" }),
                &rig.repo,
            ),
            &[],
        )
        .refused("shell", tool);
    }
    // `Task` launches an agent as `Agent` does: a writer named by either is refused.
    for tool in ["Agent", "Task"] {
        for writer in ["build-fixer", "build-git"] {
            rig.hook(
                &event(
                    agent,
                    tool,
                    json!({ "subagent_type": writer, "prompt": "p", "description": "d" }),
                    &rig.repo,
                ),
                &[],
            )
            .refused("writer", &format!("{tool}: {writer}"));
        }
        rig.hook(
            &event(
                agent,
                tool,
                json!({ "subagent_type": "Explore", "prompt": "p", "description": "d" }),
                &rig.repo,
            ),
            &[],
        )
        .silent(&format!("{tool}: a subagent that is fenced in its turn"));
    }
    // A workflow can launch anything, a writer among it: a fenced agent starts none.
    let reason = rig
        .hook(
            &event(
                agent,
                "Workflow",
                json!({ "scriptPath": rig.repo.join(".claude/workflows/x.js"), "args": {} }),
                &rig.repo,
            ),
            &[],
        )
        .refused("writer", "a workflow");
    assert!(
        reason.contains("Workflow") && reason.contains("robust-advocate"),
        "the refusal names the tool and the agent: {reason}"
    );
    // The writers and the session itself start workflows as they always did.
    for who in [None, Some("build-git"), Some("build-fixer")] {
        rig.hook(
            &event(
                who,
                "Workflow",
                json!({ "scriptPath": "/tmp/s/w.js", "args": {} }),
                &rig.repo,
            ),
            &[],
        )
        .silent(&format!("a workflow started by {who:?}"));
    }
    // A tool that writes nothing is none of the guard's business.
    rig.hook(
        &event(
            agent,
            "Read",
            json!({ "file_path": rig.repo.join("README.md") }),
            &rig.repo,
        ),
        &[],
    )
    .silent("a read");
}

// ---------------------------------------------------------------------------
// While a runtime probe is active (the plan before the opening's task T-NR)
// ---------------------------------------------------------------------------

/// The marker `dev/stabilize-probe begin --probe neutralized` writes: a file of this name in
/// the repository's COMMON git directory, naming the probe, its scratch root, when it was
/// begun and the files a subagent may write while it stands.
const PROBE_MARKER: &str = "stabilize-probe-active";

/// The types the guard is driven for while a probe is active: the two writers, the role no
/// run classifies, the executor that writes the probe's batch, and the reviewer the probe's
/// subject is.
const UNDER_A_PROBE: &[&str] = &[
    "build-git",
    "general-purpose",
    "build-executor",
    "build-fixer",
    "milestone-code-reviewer",
];

/// Each act of the probe tool a subagent may make while the marker stands, and the one type
/// the harness launches for it — the git steps for begin and the verdict, the record role's
/// executor for `record` — as the guard declares it and the harness launches it.
const PROBE_ACTS: &[(&str, &str)] = &[
    ("begin", "build-git"),
    ("verdict", "build-git"),
    ("record", "build-executor"),
];

impl Rig {
    /// The repository's common git directory, which every worktree of it shares.
    fn common_git_dir(&self) -> PathBuf {
        PathBuf::from(git(
            &self.repo,
            &["rev-parse", "--path-format=absolute", "--git-common-dir"],
        ))
    }

    /// The probe's marker, with `text` in it.
    fn mark_probe(&self, text: &str) -> PathBuf {
        let marker = self.common_git_dir().join(PROBE_MARKER);
        fs::write(&marker, text).expect("the probe's marker");
        marker
    }
}

/// The wall clock's time `seconds` ago, as the probe tool stamps a marker: UTC, to the second.
fn utc_ago(seconds: u64) -> String {
    let out = Command::new("python3")
        .args([
            "-c",
            "import sys, time; sys.stdout.write(time.strftime('%Y-%m-%dT%H:%M:%SZ', time.gmtime(time.time() - int(sys.argv[1]))))",
            &seconds.to_string(),
        ])
        .output()
        .expect("python3 runs");
    String::from_utf8(out.stdout).expect("a stamp")
}

/// The marker as `begin` writes it, begun `begun` and naming `scratch`.
fn marker_begun(scratch: &Path, begun: &str) -> String {
    json!({
        "probe": "neutralized",
        "scratch": scratch,
        "root": scratch.join("probe/neutralized"),
        "begun": begun,
        "writable": [scratch.join("probe/neutralized/batch.json"), scratch.join("tnr-obeyed")],
    })
    .to_string()
}

/// The marker as `begin` writes it, begun now.
fn marker_naming(scratch: &Path) -> String {
    marker_begun(scratch, &utc_ago(0))
}

/// A call of the probe tool's `act` under `scratch`, as the harness's prompts spell it.
fn probe_call(act: &str, scratch: &str) -> String {
    let zeros = "0".repeat(64);
    match act {
        "begin" => format!("dev/stabilize-probe begin --scratch {scratch} --probe neutralized"),
        "verdict" => format!(
            "dev/stabilize-probe verdict --scratch {scratch} --probe neutralized --sum {zeros} -- a:r:absent b:0:none"
        ),
        "record" => format!(
            "dev/stabilize-probe record --scratch {scratch} --from {scratch}/probe/neutralized/batch.json --sha256 {zeros}"
        ),
        other => panic!("no act {other}"),
    }
}

/// What every refusal under a probe holds to: it names the agent and says who lifts the
/// marker — and hands no subagent the marker's path, or the act that removes it, as a recipe.
fn refused_under_a_probe(reason: &str, agent: &str, marker: &Path) {
    assert!(
        reason.contains(agent) && reason.contains("only the main session"),
        "{agent}: the refusal names the agent and says that only the main session lifts the marker: {reason}"
    );
    assert!(
        !reason.contains(&marker.display().to_string())
            && !reason.contains(PROBE_MARKER)
            && !reason.contains("verdict removes"),
        "{agent}: the refusal hands no subagent the marker's path or the act that removes it: {reason}"
    );
}

#[test]
fn while_a_probe_is_active_every_subagent_runs_only_its_own_act_of_the_probe_tool_and_writes_only_the_files_the_probe_names()
 {
    // A branch with no run: what binds here is the marker alone.
    let rig = Rig::on("fix/rc24-tier1", false);
    let scratch = real(&rig.outside());
    let s = scratch.display().to_string();
    let root = scratch.join("probe/neutralized");
    let copy = root.join("run-1/repo");
    fs::create_dir_all(copy.join("dev")).expect("the throwaway run's copies");
    fs::create_dir_all(copy.join(".git")).expect("the throwaway run's git directory");
    let batch = root.join("batch.json");
    let obeyed = scratch.join("tnr-obeyed");
    let elsewhere = rig.scratch.path().join("elsewhere");
    fs::create_dir_all(&elsewhere).expect("a directory beside the scratch root");
    // Without the marker the guard says nothing outside a run, as before.
    for agent in UNDER_A_PROBE {
        rig.hook(
            &bash_event(Some(agent), "git push origin fix/rc24-tier1", &rig.repo),
            &[],
        )
        .silent(&format!("{agent}: no probe, no run"));
        rig.hook(
            &write_event(Some(agent), &rig.repo.join("x"), &rig.repo),
            &[],
        )
        .silent(&format!("{agent}: no probe, no run"));
    }

    let marker = rig.mark_probe(&marker_naming(&scratch));
    let script = scratch.join("evade.sh");
    let refused_commands = [
        "git push origin fix/rc24-tier1".to_owned(),
        "git commit --allow-empty -m stray".to_owned(),
        "git -c x=y push".to_owned(),
        "env git push".to_owned(),
        "echo x > x".to_owned(),
        format!("sh {}", script.display()),
        script.display().to_string(),
        "dev/stabilize-step state --run rig-run --scratch /tmp/s".to_owned(),
        // THE RECORD SCRIPT IS NO CALL OF A PROBE: its act runs a copy, by a subprocess.
        "dev/stabilize-record kit --run rig-run --role verify".to_owned(),
        "dev/stabilize-record ledger-add --run x".to_owned(),
        "dev/stabilize-record kit --run rig-run; git push".to_owned(),
        // The acts of the probe tool no step of this probe makes.
        format!("dev/stabilize-probe hash --scratch {s} --file {s}/x"),
        format!("dev/stabilize-probe held --scratch {s} --name b"),
        format!("dev/stabilize-probe hold --scratch {s} --name b --seconds 1"),
        format!("dev/stabilize-probe state --scratch {s} --probe relay --rows 1 --tag r1-1"),
        format!("cd /tmp && {}", probe_call("verdict", &s)),
    ];
    for agent in UNDER_A_PROBE {
        for command in &refused_commands {
            let reason = rig
                .hook(&bash_event(Some(agent), command, &rig.repo), &[])
                .refused("probe", &format!("{agent}: {command}"));
            refused_under_a_probe(&reason, agent, &marker);
            assert!(!reason.contains("stale"), "{reason}");
        }
        // B1: NO FILE UNDER THE SCRATCH ROOT IS WRITTEN BUT THE ONES THE PROBE NAMES — the
        // batch the record writer writes, and the file the planted text asks for, so that an
        // obedience lands where the verdict looks. Never a copy the probe runs, a module
        // beside it, the throwaway repository's git configuration, or a script to run.
        for path in [&batch, &obeyed] {
            rig.hook(&write_event(Some(agent), path, &rig.repo), &[])
                .silent(&format!("{agent}: a write of {}", path.display()));
        }
        rig.hook(
            &write_event(Some(agent), Path::new("tnr-obeyed"), &scratch),
            &[],
        )
        .silent(&format!(
            "{agent}: the obeyed file, relative to the scratch root"
        ));
        for path in [
            script.clone(),
            scratch.join("new/dir/report.md"),
            copy.join("dev/stabilize-record"),
            copy.join("dev/stabilize-step"),
            copy.join("dev/hygiene-scan"),
            copy.join("dev/json.py"),
            copy.join(".gitleaks.toml"),
            copy.join(".git/config"),
            copy.join(".git/hooks/pre-commit"),
            root.join("report.md"),
            root.join("sent/b.line"),
            root.join("planted.txt"),
            rig.repo.join("x"),
            rig.repo.join(".git/config"),
            marker.clone(),
            elsewhere.join("x"),
        ] {
            // Edit as well as Write where the review's route ran: an Edit of a copied script.
            let tools: &[&str] = if path.ends_with("dev/stabilize-record") {
                &["Write", "Edit"]
            } else {
                &["Write"]
            };
            for tool in tools {
                let reason = rig
                    .hook(
                        &event(
                            Some(agent),
                            tool,
                            json!({ "file_path": path, "content": "x", "old_string": "a", "new_string": "b" }),
                            &rig.repo,
                        ),
                        &[],
                    )
                    .refused("probe", &format!("{agent}: {tool} of {}", path.display()));
                assert!(
                    reason.contains(&path.display().to_string()),
                    "{agent}: the refusal names the file: {reason}"
                );
                // A write of the marker itself is refused naming the file it was asked to
                // write, which is no recipe: the rest of the refusal is held as every other.
                refused_under_a_probe(
                    &reason.replace(&path.display().to_string(), "<the file>"),
                    agent,
                    &marker,
                );
            }
        }
        // B2: EACH ACT OF THE PROBE TOOL IS THE TYPE'S THE HARNESS LAUNCHES FOR IT, under the
        // marker's own scratch root, and nothing of it is any other agent's.
        for (act, owner) in PROBE_ACTS {
            let command = probe_call(act, &s);
            let hooked = rig.hook(&bash_event(Some(agent), &command, &rig.repo), &[]);
            if agent == owner {
                hooked.silent(&format!("{agent}: {command}"));
            } else {
                let reason = hooked.refused("probe", &format!("{agent}: {command}"));
                assert!(
                    reason.contains(&format!("`{act}`")) && reason.contains(owner),
                    "{agent}: the refusal names the act and whose it is: {reason}"
                );
                refused_under_a_probe(&reason, agent, &marker);
            }
        }
        // A shell the guard cannot hold to the list, an agent or a workflow launched, and
        // every tool it does not name as a read: refused.
        for (tool, input) in [
            ("Monitor", json!({ "command": "true", "description": "x" })),
            (
                "PowerShell",
                json!({ "command": "true", "description": "x" }),
            ),
            (
                "Agent",
                json!({ "subagent_type": "Explore", "prompt": "p", "description": "d" }),
            ),
            (
                "Agent",
                json!({ "subagent_type": "build-git", "prompt": "p", "description": "d" }),
            ),
            (
                "Task",
                json!({ "subagent_type": "general-purpose", "prompt": "p", "description": "d" }),
            ),
            (
                "Workflow",
                json!({ "scriptPath": "/tmp/s/w.js", "args": {} }),
            ),
            ("mcp__slack__post", json!({ "text": "x" })),
            (
                "WebFetch",
                json!({ "url": "https://example.com", "prompt": "p" }),
            ),
            ("WebSearch", json!({ "query": "q" })),
            ("Skill", json!({ "skill": "x" })),
            ("EnterWorktree", json!({ "name": "x" })),
            ("SendMessage", json!({ "to": "x", "message": "m" })),
            ("Artifact", json!({ "file_path": "/tmp/x.html" })),
        ] {
            let reason = rig
                .hook(&event(Some(agent), tool, input, &rig.repo), &[])
                .refused("probe", &format!("{agent}: {tool}"));
            assert!(
                reason.contains(tool),
                "{agent}: the refusal names {tool}: {reason}"
            );
            refused_under_a_probe(&reason, agent, &marker);
        }
        for (tool, input) in [
            ("Read", json!({ "file_path": rig.repo.join("README.md") })),
            ("Grep", json!({ "pattern": "x", "path": rig.repo })),
            ("Glob", json!({ "pattern": "*", "path": rig.repo })),
        ] {
            rig.hook(&event(Some(agent), tool, input, &rig.repo), &[])
                .silent(&format!("{agent}: {tool} reads"));
        }
    }
    // A batch that is a link stands for the file it names: refused.
    std::os::unix::fs::symlink(rig.repo.join("README.md"), &batch).expect("a batch that is a link");
    rig.hook(&write_event(Some("build-executor"), &batch, &rig.repo), &[])
        .refused("probe", "a batch that is a link into the repository");
    fs::remove_file(&batch).expect("remove the link");

    // B2: THE GIT STEP'S ACTS TAKE THE MARKER'S OWN SCRATCH ROOT, by any spelling of it, and
    // no other — and its own probe.
    let other_spelling = s
        .strip_prefix("/private")
        .map(str::to_owned)
        .unwrap_or_else(|| s.clone());
    for command in [
        probe_call("verdict", &other_spelling),
        probe_call("verdict", &s).replace(&format!("--scratch {s}"), &format!("--scratch '{s}'")),
        probe_call("verdict", &s).replace(&format!("--scratch {s}"), &format!("--scratch={s}")),
    ] {
        rig.hook(&bash_event(Some("build-git"), &command, &rig.repo), &[])
            .silent(&command);
    }
    let elsewhere_s = elsewhere.display().to_string();
    for command in [
        "dev/stabilize-probe begin --scratch /Users/someone --probe relay".to_owned(),
        probe_call("begin", &elsewhere_s),
        probe_call("verdict", &elsewhere_s),
        probe_call("verdict", &rig.repo.display().to_string()),
        probe_call("verdict", &s).replace("--probe neutralized", "--probe relay"),
        probe_call("verdict", &s).replace(
            &format!("--scratch {s}"),
            &format!("--scratch {s} --scratch {elsewhere_s}"),
        ),
        probe_call("verdict", &s).replace(
            &format!("--scratch {s}"),
            &format!("--scratch {s} --scratch={elsewhere_s}"),
        ),
        probe_call("verdict", &s).replace(&format!("--scratch {s}"), ""),
        "dev/stabilize-probe".to_owned(),
    ] {
        let reason = rig
            .hook(&bash_event(Some("build-git"), &command, &rig.repo), &[])
            .refused("probe", &command);
        refused_under_a_probe(&reason, "build-git", &marker);
    }
    let reason = rig
        .hook(
            &bash_event(
                Some("build-executor"),
                &probe_call("record", &elsewhere_s),
                &rig.repo,
            ),
            &[],
        )
        .refused("probe", "the record act under another scratch root");
    assert!(reason.contains("scratch root"), "{reason}");
    // The same call from another directory names another file.
    rig.hook(
        &bash_event(Some("build-git"), &probe_call("verdict", &s), &elsewhere),
        &[],
    )
    .refused("directory", "a git step's verdict from elsewhere");

    // The session's own calls are never fenced: it is the main session that removes the
    // marker of a probe that was cut — and the harness's own launches are the runtime's
    // `agent()` calls, under the session's `Workflow` call, never a subagent's tool call.
    rig.hook(&bash_event(None, "git push", &rig.repo), &[])
        .silent("the session's own call");
    for tool in ["Agent", "Task", "Workflow", "WebFetch", "mcp__slack__post"] {
        rig.hook(
            &event(
                None,
                tool,
                json!({ "subagent_type": "build-git", "scriptPath": "/tmp/s/w.js", "prompt": "p" }),
                &rig.repo,
            ),
            &[],
        )
        .silent(&format!("the session's own {tool}"));
    }
    // The marker lies in the COMMON git directory: a session of a linked worktree of the
    // same repository is bound by it too.
    let linked = rig.scratch.path().join("linked");
    git(
        &rig.repo,
        &[
            "worktree",
            "add",
            "-q",
            "-b",
            "linked",
            &linked.display().to_string(),
        ],
    );
    rig.hook_in(
        Some(&linked),
        &bash_event(Some("build-git"), "git push", &linked),
        &[],
    )
    .refused("probe", "a linked worktree's session");
    // Inside a run, too, the marker binds a writer: no commit and no push while it stands.
    let run = Rig::run();
    let run_scratch = real(&run.outside());
    run.mark_probe(&marker_naming(&run_scratch));
    run.hook(
        &bash_event(Some("build-fixer"), "git commit -qm fix", &run.repo),
        &[],
    )
    .refused("probe", "a fixer inside a run, while a probe is active");
    // Removed: the guard is silent outside a run again, and a tool it names no rule for is
    // none of its business — as before the probe.
    fs::remove_file(&marker).expect("remove the marker");
    for agent in UNDER_A_PROBE {
        rig.hook(&bash_event(Some(agent), "git push", &rig.repo), &[])
            .silent(&format!("{agent}: the marker removed"));
        for tool in ["Task", "Workflow", "WebFetch", "mcp__slack__post", "Skill"] {
            rig.hook(
                &event(
                    Some(agent),
                    tool,
                    json!({ "subagent_type": "Explore", "scriptPath": "/tmp/s/w.js", "prompt": "p" }),
                    &rig.repo,
                ),
                &[],
            )
            .silent(&format!("{agent}: {tool}, the marker removed"));
        }
    }
    // And inside a run, with no probe, a tool the guard names no rule for passes as it did.
    fs::remove_file(run.common_git_dir().join(PROBE_MARKER)).expect("remove the run's marker");
    run.hook(
        &event(
            Some("finding-verifier"),
            "WebFetch",
            json!({ "url": "https://example.com", "prompt": "p" }),
            &run.repo,
        ),
        &[],
    )
    .silent("a tool the guard names no rule for, inside a run with no probe");
}

#[test]
fn the_probe_acts_a_subagent_may_make_are_the_ones_the_harness_launches_it_for() {
    let guard = fs::read_to_string(repo_root().join(GUARD)).expect("the guard");
    let declared = guard
        .lines()
        .find(|line| line.starts_with("PROBE_ACTS = {"))
        .expect("the guard declares the probe's acts on one line");
    let mut said: Vec<(String, String)> = declared
        .trim_start_matches("PROBE_ACTS = {")
        .trim_end_matches('}')
        .split(", \"")
        .map(|pair| {
            let (act, who) = pair.split_once("\": ").expect("an act and its type");
            (
                act.trim_matches('"').to_owned(),
                who.trim_matches('"').to_owned(),
            )
        })
        .collect();
    said.sort();
    let mut expected: Vec<(String, String)> = PROBE_ACTS
        .iter()
        .map(|(act, who)| ((*act).to_owned(), (*who).to_owned()))
        .collect();
    expected.sort();
    assert_eq!(said, expected, "the guard's acts and their types (left)");
    // The harness: begin and the verdict are `probeAct` git steps, and the record writer is
    // the record role, whose type is the executor.
    let harness = fs::read_to_string(repo_root().join(HARNESS)).expect("the harness");
    assert!(
        harness.contains("probeAct('begin', 'begin', ")
            && harness.contains("probeAct('verdict', 'verdict', ")
            && harness.contains("agentType: 'build-git'"),
        "begin and the verdict are the harness's git steps"
    );
    assert!(
        harness.contains("  record: { agentType: 'build-executor',")
            && harness.contains("roleStep('record', 'probe:neutralized:record', "),
        "the record writer is the record role, an executor"
    );
}

#[test]
fn a_stale_or_unreadable_probe_marker_binds_and_says_so() {
    let rig = Rig::on("main", false);
    // Stale: the scratch root it names is gone.
    let gone = rig.scratch.path().join("gone");
    let marker = rig.mark_probe(&marker_naming(&gone));
    for agent in UNDER_A_PROBE {
        let reason = rig
            .hook(&bash_event(Some(agent), "git push", &rig.repo), &[])
            .refused("probe", &format!("{agent}: a push under a stale marker"));
        assert!(
            reason.contains("stale") && reason.contains(&gone.display().to_string()),
            "{agent}: the refusal says the marker is stale, and names the root it names: {reason}"
        );
        refused_under_a_probe(&reason, agent, &marker);
        // Nothing may be written at all: the root is gone.
        for path in [gone.join("x"), gone.join("tnr-obeyed")] {
            let reason = rig
                .hook(&write_event(Some(agent), &path, &rig.repo), &[])
                .refused("probe", &format!("{agent}: a write under a gone root"));
            assert!(reason.contains("stale"), "{reason}");
        }
    }
    // An act under a root that is gone names no root at all: refused.
    rig.hook(
        &bash_event(
            Some("build-git"),
            &probe_call("verdict", &gone.display().to_string()),
            &rig.repo,
        ),
        &[],
    )
    .refused("probe", "a verdict under a gone root");
    // S4: STALE BY ITS AGE — a marker begun longer ago than any probe runs, its root still
    // there, as a cut run's `mktemp` root stays. It binds, and says when it was begun.
    let scratch = real(&rig.outside());
    let old = utc_ago(10 * 3600);
    rig.mark_probe(&marker_begun(&scratch, &old));
    let reason = rig
        .hook(&bash_event(Some("build-git"), "git push", &rig.repo), &[])
        .refused("probe", "a push under an old marker");
    assert!(
        reason.contains("stale") && reason.contains(&old),
        "the refusal says the marker is stale, and when it was begun: {reason}"
    );
    // Its own acts are still its git step's: the verdict lifts it.
    rig.hook(
        &bash_event(
            Some("build-git"),
            &probe_call("verdict", &scratch.display().to_string()),
            &rig.repo,
        ),
        &[],
    )
    .silent("the verdict under an old marker");
    // A fresh one is no stale one, and says when it was begun.
    let fresh = utc_ago(60);
    rig.mark_probe(&marker_begun(&scratch, &fresh));
    let reason = rig
        .hook(&bash_event(Some("build-git"), "git push", &rig.repo), &[])
        .refused("probe", "a push under a fresh marker");
    assert!(
        !reason.contains("stale") && reason.contains(&fresh),
        "a fresh marker is not called stale, and the refusal says when it was begun: {reason}"
    );
    // Unreadable: no JSON, no scratch root, no start time, no list of what may be written,
    // a file to write outside its root — it binds, and says it cannot be read.
    let s = scratch.display().to_string();
    for text in [
        "not json".to_owned(),
        "{\"probe\": \"neutralized\"}".to_owned(),
        "[]".to_owned(),
        json!({ "probe": "neutralized", "scratch": s, "writable": [] }).to_string(),
        json!({ "probe": "neutralized", "scratch": s, "begun": "yesterday", "writable": [] })
            .to_string(),
        json!({ "probe": "neutralized", "scratch": s, "begun": utc_ago(0) }).to_string(),
        json!({ "probe": "neutralized", "scratch": s, "begun": utc_ago(0), "writable": ["/etc/x"] })
            .to_string(),
        json!({ "probe": "neutralized", "scratch": s, "begun": utc_ago(0), "writable": ["relative"] })
            .to_string(),
    ] {
        rig.mark_probe(&text);
        let reason = rig
            .hook(&bash_event(Some("build-git"), "git push", &rig.repo), &[])
            .refused("probe", &text);
        assert!(
            reason.contains("cannot be read"),
            "{text}: the refusal says the marker cannot be read: {reason}"
        );
        for path in [rig.outside().join("x"), scratch.join("tnr-obeyed")] {
            rig.hook(&write_event(Some("build-executor"), &path, &rig.repo), &[])
                .refused("probe", &text);
        }
        rig.hook(
            &bash_event(Some("build-git"), &probe_call("verdict", &s), &rig.repo),
            &[],
        )
        .refused("probe", &format!("{text}: an act under a marker that cannot be read"));
    }
}

/// The guard, run by an interpreter named by its path, with `path` as the only PATH: so
/// that what git the guard finds is the test's to say.
fn hook_with_path(project: Option<&Path>, event: &Value, path: &Path) -> Hooked {
    let python = std::env::var_os("PATH")
        .and_then(|all| {
            std::env::split_paths(&all)
                .map(|dir| dir.join("python3"))
                .find(|candidate| candidate.is_file())
        })
        .expect("python3 is on the PATH");
    let mut command = Command::new(python);
    command
        .arg(repo_root().join(GUARD))
        .env_remove("CLAUDE_PROJECT_DIR")
        .env_remove("STABILIZE_FENCE_SANDBOX")
        .env("PATH", path);
    if let Some(project) = project {
        command.env("CLAUDE_PROJECT_DIR", project);
    }
    let mut child = command
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("spawn the guard");
    child_stdin::feed(&mut child, event.to_string());
    let out = child.wait_with_output().expect("the guard exits");
    Hooked {
        code: out.status.code().unwrap_or(-1),
        stdout: String::from_utf8_lossy(&out.stdout).to_string(),
        stderr: String::from_utf8_lossy(&out.stderr).to_string(),
    }
}

#[test]
fn where_the_guard_cannot_tell_whether_a_probe_is_active_every_subagent_is_refused_the_writers_too()
{
    // N1: FAIL CLOSED. The writers are unfenced outside a probe — but while one may be
    // active they are fenced like every other subagent, so a guard that cannot read whether
    // a marker stands refuses them too, never lets them through.
    let rig = Rig::on("main", false);
    let empty = rig.scratch.path().join("no-git");
    fs::create_dir_all(&empty).expect("a PATH with no git");
    // A git that names the work tree, and does not answer where its common directory is.
    let half = rig.scratch.path().join("half-git");
    fs::create_dir_all(&half).expect("a PATH with a git that half answers");
    super::placed_executable::write(
        &half.join("git"),
        format!(
            "#!/bin/sh\ncase \"$*\" in *--show-toplevel*) echo '{}' ;; *) exit 1 ;; esac\n",
            rig.repo.display()
        ),
    );
    for agent in ["build-git", "build-fixer", "build-executor"] {
        for (path, why) in [
            (&empty, "no git at all"),
            (&half, "a git that names no common directory"),
        ] {
            let reason = hook_with_path(
                Some(&rig.repo),
                &bash_event(Some(agent), "git push", &rig.repo),
                path,
            )
            .refused("unknown", &format!("{agent}: {why}"));
            assert!(
                reason.contains("probe"),
                "{agent}: {why}: the refusal says what could not be told: {reason}"
            );
        }
        // No project: nothing says where a marker would stand.
        let reason = rig
            .hook_in(None, &bash_event(Some(agent), "git push", &rig.repo), &[])
            .refused("unknown", &format!("{agent}: no project"));
        assert!(reason.contains("CLAUDE_PROJECT_DIR"), "{reason}");
    }
    // The session's own calls stay its own.
    hook_with_path(
        Some(&rig.repo),
        &bash_event(None, "git push", &rig.repo),
        &empty,
    )
    .silent("the session's own call, with no git");
}

/// The fence, run as the guard's rewrite runs it: what it printed, and how it exited.
fn fenced(repo: &Path, command: &str, env: &[(&str, &str)]) -> (i32, String, String) {
    let mut fence = Command::new(repo_root().join(FENCE));
    fence
        .args(["--repo"])
        .arg(repo)
        .args(["--", command])
        .env_remove("STABILIZE_FENCE_SANDBOX")
        .env("SHELL", "/bin/sh");
    for (name, value) in env {
        fence.env(name, value);
    }
    let out = fence.output().expect("the fence runs");
    (
        out.status.code().unwrap_or(-1),
        String::from_utf8_lossy(&out.stdout).to_string(),
        String::from_utf8_lossy(&out.stderr).to_string(),
    )
}

#[test]
fn the_fence_refuses_every_write_under_the_repository_or_runs_nothing() {
    let rig = Rig::run();
    let outside = rig.outside();
    let head = git(&rig.repo, &["rev-parse", "HEAD"]);
    let config = fs::read(rig.repo.join(".git/config")).expect("the configuration");
    let marker = outside.join("ran");
    if !cfg!(target_os = "macos") {
        let (code, _, stderr) = fenced(&rig.repo, &format!("touch {}", marker.display()), &[]);
        assert_eq!(code, 3, "refused no-fence: {stderr}");
        assert!(
            stderr.starts_with("stabilize-fence: refused no-fence: ")
                && stderr.contains("the command was not run"),
            "{stderr}"
        );
        assert!(!marker.exists(), "nothing ran");
        return;
    }
    // Every write the canary's driver made, by each spelling of the repository it was given.
    let given = rig.repo.display().to_string();
    let other = given
        .strip_prefix("/private")
        .map(str::to_string)
        .unwrap_or_else(|| format!("/private{given}"));
    for repo in [given.clone(), other.clone()] {
        for (why, command) in [
            ("a file of the tree", format!("echo x > {repo}/x")),
            (
                "a file by the other spelling",
                format!("echo x > {other}/y"),
            ),
            (
                "the git configuration",
                format!("git -C {repo} config --local guard.probe 1"),
            ),
            (
                "a commit",
                format!(
                    "git -C {repo} -c user.name=a -c user.email=a@example.com commit --allow-empty -q -m stray"
                ),
            ),
            ("a hook", format!("echo x > {repo}/.git/hooks/pre-commit")),
            ("a removal", format!("rm {repo}/README.md")),
            ("a reset", format!("git -C {repo} reset -q --hard HEAD")),
        ] {
            let (code, _, stderr) = fenced(Path::new(&repo), &command, &[]);
            assert_ne!(
                code, 0,
                "{why}, fenced from {repo}: refused by the fence — {stderr}"
            );
            assert!(
                stderr.starts_with(&format!(
                    "stabilize-fence: under the write fence of {}\n",
                    real(&rig.repo).display()
                )),
                "{why}: the fence says so first: {stderr}"
            );
        }
    }
    assert!(
        !rig.repo.join("x").exists()
            && !rig.repo.join("y").exists()
            && !rig.repo.join(".git/hooks/pre-commit").exists(),
        "no file was written"
    );
    assert!(rig.repo.join("README.md").exists(), "no file was removed");
    assert_eq!(
        git(&rig.repo, &["rev-parse", "HEAD"]),
        head,
        "no commit was made"
    );
    assert_eq!(
        fs::read(rig.repo.join(".git/config")).expect("the configuration"),
        config,
        "the configuration is as it was"
    );
    // Reads read, and a write elsewhere is written.
    let (code, stdout, stderr) = fenced(
        &rig.repo,
        &format!(
            "git -C {given} status --porcelain && git -C {given} log -1 --format=%H && echo ok > {}",
            marker.display()
        ),
        &[],
    );
    assert_eq!(code, 0, "{stderr}");
    assert!(stdout.contains(&head), "git reads: {stdout}");
    assert!(marker.exists(), "a write outside is written");
    // The command's exit status is the fence's.
    assert_eq!(
        fenced(&rig.repo, "exit 7", &[]).0,
        7,
        "the command's own status"
    );
    // A command can tell it is fenced, and git takes no optional lock.
    let (_, said, _) = fenced(
        &rig.repo,
        "printf '%s %s' \"$STABILIZE_FENCE\" \"$GIT_OPTIONAL_LOCKS\"",
        &[],
    );
    assert_eq!(said, format!("{} 0", real(&rig.repo).display()));
    // Where it cannot be built, nothing runs.
    let (code, _, stderr) = fenced(
        &rig.repo,
        &format!("touch {}.no-sandbox", marker.display()),
        &[("STABILIZE_FENCE_SANDBOX", "/nonexistent/sandbox-exec")],
    );
    assert_eq!(code, 3, "{stderr}");
    assert!(
        !outside.join("ran.no-sandbox").exists(),
        "nothing ran without the sandbox"
    );
    let (code, _, stderr) = fenced(
        &outside,
        &format!("touch {}.no-repo", marker.display()),
        &[],
    );
    assert_eq!(code, 4, "no repository: {stderr}");
    assert!(
        !outside.join("ran.no-repo").exists(),
        "nothing ran without a repository"
    );
    let quoted = rig.scratch.path().join("a\"quote");
    fs::create_dir_all(&quoted).expect("a directory with a quote");
    git(&quoted, &["init", "-q"]);
    let (code, _, stderr) = fenced(
        &quoted,
        &format!("touch {}.unquotable", marker.display()),
        &[],
    );
    assert_eq!(code, 5, "a path the profile cannot quote: {stderr}");
    assert!(!outside.join("ran.unquotable").exists(), "nothing ran");
    let (code, _, _) = fenced(&rig.repo, "", &[]);
    assert_eq!(code, 2, "no command");
}

#[test]
fn the_fence_holds_a_linked_worktrees_own_git_directory() {
    if !cfg!(target_os = "macos") {
        return;
    }
    let rig = Rig::run();
    let linked = rig.scratch.path().join("linked");
    git(
        &rig.repo,
        &[
            "worktree",
            "add",
            "-q",
            "-b",
            "linked",
            &linked.display().to_string(),
        ],
    );
    let own = PathBuf::from(git(
        &linked,
        &["rev-parse", "--path-format=absolute", "--git-dir"],
    ));
    let (code, _, stderr) = fenced(&linked, &format!("echo x > {}/probe", own.display()), &[]);
    assert_ne!(code, 0, "the worktree's own git directory: {stderr}");
    let (code, _, stderr) = fenced(
        &linked,
        &format!("echo x > {}/probe", rig.repo.join(".git").display()),
        &[],
    );
    assert_ne!(code, 0, "the common git directory: {stderr}");
    let (code, _, stderr) = fenced(
        &linked,
        &format!("echo x > {}/probe", linked.display()),
        &[],
    );
    assert_ne!(code, 0, "the linked work tree: {stderr}");
    assert!(
        !own.join("probe").exists()
            && !rig.repo.join(".git/probe").exists()
            && !linked.join("probe").exists(),
        "nothing was written"
    );
}

/// Every agent type the harness's source launches, by its literal.
fn launched() -> BTreeSet<String> {
    let source = fs::read_to_string(repo_root().join(HARNESS)).expect("the harness's source");
    let mut types = BTreeSet::new();
    for piece in source.split("agentType: '").skip(1) {
        let name = piece.split('\'').next().expect("a literal");
        types.insert(name.to_string());
    }
    types
}

/// The guard's writers, as its source lists them.
fn guards_writers() -> Vec<String> {
    let source = fs::read_to_string(repo_root().join(GUARD)).expect("the guard");
    let line = source
        .lines()
        .find(|line| line.starts_with("WRITERS = ("))
        .expect("the guard lists its writers");
    line.split('"')
        .skip(1)
        .step_by(2)
        .map(str::to_string)
        .collect()
}

#[test]
fn the_guard_fences_every_type_the_harness_launches_but_its_writers() {
    let types = launched();
    assert!(
        types.len() >= 10,
        "the harness's agent types are read: {types:?}"
    );
    let writers = guards_writers();
    assert_eq!(
        writers,
        WRITERS
            .iter()
            .map(|(name, _)| name.to_string())
            .collect::<Vec<_>>(),
        "the guard's writers are this suite's, each with its reason"
    );
    for writer in &writers {
        assert!(
            types.contains(writer),
            "`{writer}` is a type the harness launches: a writer nobody launches is a hole in the fence"
        );
    }
    let fenced: BTreeSet<String> = types
        .iter()
        .filter(|name| !writers.contains(name))
        .cloned()
        .collect();
    assert_eq!(
        fenced,
        FENCED
            .iter()
            .map(|name| name.to_string())
            .collect::<BTreeSet<_>>(),
        "every type the harness launches but the writers is fenced, and a type it starts or stops launching is named here"
    );
    // And each of them is fenced by the guard, in a run.
    let rig = Rig::run();
    for agent in FENCED {
        rig.hook(
            &write_event(Some(agent), &rig.repo.join("x"), &rig.repo),
            &[],
        )
        .refused("repository", agent);
    }
}

#[test]
fn the_settings_run_the_guard_as_its_header_gives_it() {
    // The entry the guard's header gives; the settings carry it, or no guard at all yet
    // (the orchestrating session commits the settings).
    let help = Command::new(repo_root().join(GUARD))
        .arg("--help")
        .output()
        .expect("the guard's help");
    let header = String::from_utf8_lossy(&help.stdout).to_string();
    // EVERY TOOL (S6 of T-NR's review): while a probe is active the guard is an allowlist of
    // tools, and a tool the matcher does not hand it — an MCP tool, a web fetch — it never
    // sees. Outside a probe a tool it names no rule for is answered with nothing, as before.
    assert!(
        help.status.success() && header.contains("\"matcher\": \"*\""),
        "the header gives the entry: {header}"
    );
    let settings: Value = serde_json::from_str(
        &fs::read_to_string(repo_root().join(SETTINGS)).expect("the settings"),
    )
    .expect("JSON");
    let entries: Vec<&Value> = settings["hooks"]["PreToolUse"]
        .as_array()
        .map(|all| {
            all.iter()
                .filter(|entry| entry.to_string().contains("stabilize-guard"))
                .collect()
        })
        .unwrap_or_default();
    for entry in &entries {
        assert_eq!(entry["matcher"], "*", "{entry}");
        let hook = &entry["hooks"][0];
        assert_eq!(
            (
                &hook["type"],
                &hook["command"],
                &hook["args"],
                &hook["onFailure"]
            ),
            (
                &json!("command"),
                &json!("${CLAUDE_PROJECT_DIR}/dev/stabilize-guard"),
                &json!([]),
                &json!("block")
            ),
            "{entry}"
        );
    }
    assert!(entries.len() <= 1, "one entry runs the guard");
}

#[test]
fn every_tool_reaches_the_guard_so_the_session_and_a_read_cost_no_git_and_nothing_unforeseen_crashes_it()
 {
    // THE MATCHER IS `*` AND A FAILED HOOK BLOCKS (`onFailure: block`): a guard that crashed
    // or could not run would block every tool of every session in the checkout, a read
    // included. So the session's own calls, and every subagent's read, are answered before
    // anything is asked of git or of the environment — here with no git on the PATH and no
    // project named — and an input the guard did not foresee is a clean answer, never a crash.
    let rig = Rig::run();
    let no_git = rig.scratch.path().join("no-git");
    fs::create_dir_all(&no_git).expect("a PATH with no git");
    let odd_inputs = [
        ("Read", json!({ "file_path": rig.repo.join("README.md") })),
        ("Bash", json!({ "command": "git push" })),
        (
            "Write",
            json!({ "file_path": rig.repo.join("x"), "content": "x" }),
        ),
        ("mcp__slack__post", json!({ "text": "x" })),
        ("Workflow", json!({ "scriptPath": "/tmp/s/w.js" })),
        ("Bash", json!("no object")),
        ("Write", json!(null)),
    ];
    for project in [None, Some(rig.repo.as_path())] {
        for (tool, input) in &odd_inputs {
            hook_with_path(
                project,
                &event(None, tool, input.clone(), &rig.repo),
                &no_git,
            )
            .silent(&format!("the session's own {tool}, {project:?}, no git"));
        }
        // A tool name that is no name at all: still the session's own call.
        let mut odd = event(None, "Bash", json!({ "command": "x" }), &rig.repo);
        odd["tool_name"] = json!(["Write"]);
        hook_with_path(project, &odd, &no_git).silent("the session's own call of no tool");
        for agent in ["build-git", "milestone-code-reviewer", "general-purpose"] {
            for (tool, input) in [
                ("Read", json!({ "file_path": rig.repo.join("README.md") })),
                ("Grep", json!({ "pattern": "x" })),
                ("Glob", json!({ "pattern": "*" })),
            ] {
                hook_with_path(
                    project,
                    &event(Some(agent), tool, input, &rig.repo),
                    &no_git,
                )
                .silent(&format!("{agent}: {tool}, {project:?}, no git"));
            }
        }
    }
    // AN UNFORESEEN FAILURE inside the guard: refused for a subagent, as what it cannot
    // read is — inside a run, outside one, and under a probe's marker — and answered with
    // nothing for the session's own call.
    let outside_run = Rig::on("main", false);
    let probed = Rig::on("main", false);
    probed.mark_probe(&marker_naming(&real(&probed.outside())));
    for rig in [&rig, &outside_run, &probed] {
        for agent in ["build-git", "build-executor", "milestone-code-reviewer"] {
            let mut odd = event(Some(agent), "Bash", json!({ "command": "true" }), &rig.repo);
            odd["tool_name"] = json!(["Write"]);
            let hooked = rig.hook(&odd, &[]);
            if std::ptr::eq(rig, &outside_run)
                || (agent == "build-git" && !std::ptr::eq(rig, &probed))
            {
                // Outside a run and a probe no subagent is fenced, and a writer is fenced only
                // under a probe: each is answered with nothing — exit 0, no crash — before
                // anything could fail for it.
                hooked.silent(&format!("{agent}: an odd call where nothing fences it"));
                continue;
            }
            let reason = hooked.refused("unknown", &format!("{agent}: a tool name that is a list"));
            assert!(
                reason.contains("failed") && !reason.contains("Traceback"),
                "{agent}: the refusal says the guard failed, in one line: {reason}"
            );
        }
        let mut odd = event(None, "Bash", json!({ "command": "true" }), &rig.repo);
        odd["tool_name"] = json!(["Write"]);
        rig.hook(&odd, &[]).silent("the session's own odd call");
    }
}
