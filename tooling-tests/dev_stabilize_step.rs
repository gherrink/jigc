//! **`dev/stabilize-step` — every git act of a stabilization run's harness, as ONE command**
//! ([DECISIONS.md](../DECISIONS.md) → *2026-10-06 — Every git act of the stabilization
//! harness is one command*; the repair plan's change C0,
//! [repair-plan.md](../completions/artifacts/M55/stabilization-build/repair-plan.md)).
//!
//! The harness has no shell, so a git act of a run is a step it hands to a `build-git`
//! agent. Until the tool, a step was a numbered list of shell commands the agent followed,
//! and nothing could execute one in a test. Now the agent runs one command and relays the
//! one line it prints — and this suite is where an act is executed: **against real git**,
//! in a throwaway clone with a bare remote, a fixed identity and fixed dates, hooks off.
//!
//! **Three things are held here.**
//!
//! - *Each act does what its list did.* Every act is driven once to its end, and each
//!   refusal its list had is driven too — a tree that is not clean, a pushed branch that is
//!   ahead, a branch that is not the one the step names, a merge whose tree differs, and
//!   the rest ([`REFUSALS`]). For the acts that change something the git commands the tool
//!   ran are **traced** and held to the list, command for command: the rig's `git` is a
//!   shim that writes down each call the tool itself makes and then runs the real one. The
//!   tool was built to change nothing a step checks; the trace is how that is read.
//! - *The tool can do nothing an agent may not.* [`offences`] reads every git call out of
//!   the tool's source — the call sites are derived, never listed — and holds each to the
//!   flags its subcommand may carry, each name a changing command takes to an argument
//!   whose grammar requires a branch prefix, and `main` to the one act that fetches it.
//!   The spellings of a forced push are read from the definition of the agent that runs
//!   the steps. [`a_planted_offender_reddens_the_scan`] plants each forbidden act in a copy
//!   of the source and requires the scan to name it.
//! - *The harness and the tool are one contract.* Where `node` is installed, the harness's
//!   own prompt functions compose each step, the one command is taken out of the prompt and
//!   run **verbatim**, and the harness's own `readStep` reads the line back
//!   ([`every_command_the_harness_composes_is_taken_by_the_tool_and_read_back`]). The close's
//!   sync act is held to the build harness's sync step by the commands it ran
//!   ([`the_sync_act_runs_the_build_harnesss_sync_step`]) — the half of
//!   [`merge_logs_fence`](super::merge_logs_fence)'s arm (k) that can no longer be read off
//!   two prompts.
//!
//! **Every arm runs under a shell-hostile root** — a space, a `'`, a `"` and a `#` in the
//! repository's path — because the rig has no other kind.
//!
//! **The rig is the bed the simulation of a stage is built on**
//! ([`stabilize_simulation`](super::stabilize_simulation)). [`StepRig`] is built in two
//! steps — [`StepRig::unopened`], then the opening — so that a suite can open the run
//! itself, and lends its environment; [`node_or_skip`] is what every test that runs the
//! harness under `node` asks first. That suite's agents are functions of its runtime's
//! stand-in, so [`harness_pure`] (the harness's pure functions, called under `node`) and
//! [`git_agent`] (a `build-git` agent as a function: the one command of a step's prompt,
//! run, and its line relayed) serve this suite's own contract arm.
//!
//! **Preserved on purpose, and marked where it is driven.** The tool repairs nothing. A
//! landing that halted after its merge is still answered `landed-before` by the next call,
//! unchecked and unpushed (the harness review's `H5`); the tree a stage starts from still
//! admits no untracked file but a report, the round's scope file included; and the
//! path-class assert is the list and the subject rule it was (`M8`). Each belongs to a
//! later task of the repair, and each is an assertion here that task will turn.

use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::io::Write as _;
use std::os::unix::fs::PermissionsExt;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

use serde_json::{Value, json};

use crate::support::child_stdin;
use crate::support::scratch::ScratchDir;

const TOOL: &str = "dev/stabilize-step";
const RECORD: &str = "dev/stabilize-record";
const MERGE_LOGS: &str = "dev/merge-logs";
const HARNESS: &str = ".claude/workflows/stabilize.js";
const BUILD_HARNESS: &str = ".claude/workflows/milestone-build.js";
const GIT_DEFINITION: &str = ".claude/agents/build-git.md";

/// The run every rig holds, and the names the harness would mint for it.
pub(crate) const RUN: &str = "rc24";
pub(crate) const LOOP: &str = "fix/rc24";
pub(crate) const ROUNDS: &str = "fix/rc24-r";
pub(crate) const RUN_DIR: &str = "completions/artifacts/rc24";

/// A space, both quotes and a `#`: a path that breaks any command that is not quoted.
const HOSTILE_ROOT: &str = "it's a \"repo\" #1";

/// The word a refusal opens with, and its exit status.
const REFUSALS: &[(&str, i32)] = &[
    ("usage", 2),
    ("wrong-branch", 3),
    ("dirty", 4),
    ("remote-ahead", 5),
    ("product-path", 6),
    ("foreign-merge", 7),
    ("merge-in-round", 8),
    ("conflict", 9),
    ("merge-logs", 10),
    ("unresolved", 11),
    ("merge-differs", 12),
    ("pick-stopped", 13),
    ("carry-differs", 14),
    ("push-rejected", 15),
    ("remote-differs", 16),
    ("record", 17),
    ("git", 18),
    ("gate-red", 19),
    ("no-batch", 20),
    ("position", 21),
];

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .ancestors()
        .nth(2)
        .expect("repo root is two levels above crates/cli")
        .to_path_buf()
}

fn read(rel: &str) -> String {
    fs::read_to_string(repo_root().join(rel)).unwrap_or_else(|e| panic!("read `{rel}`: {e}"))
}

/// The quoted entries of the harness's one-line list `const NAME = [ … ]`.
fn harness_list(name: &str) -> Vec<String> {
    let source = read(HARNESS);
    let declaration = format!("const {name} = [");
    let line = source
        .lines()
        .find(|line| line.starts_with(&declaration))
        .unwrap_or_else(|| panic!("the harness declares `{name}`"));
    line[declaration.len()..]
        .split('\'')
        .skip(1)
        .step_by(2)
        .map(str::to_owned)
        .collect()
}

/// The product paths and the append-only logs, as the harness hands them to the tool: its
/// own lists, never a copy of this suite's.
fn product() -> Vec<String> {
    harness_list("PRODUCT_PATHS")
}
fn logs() -> Vec<String> {
    harness_list("SYNC_LOGS")
}

// ---------------------------------------------------------------------------
// The rig
// ---------------------------------------------------------------------------

/// The rig's `git`: it writes down a call the tool itself made — never one of a script
/// the tool ran, nor one of the fixture's — and runs the real git.
const GIT_SHIM: &str = r#"#!/bin/sh
if [ -n "${STEP_TRACE:-}" ]; then
case "$(ps -o command= -p "$PPID")" in
*dev/stabilize-step*) printf '%s\n' "$*" >>"$STEP_TRACE" ;;
esac
fi
exec "@GIT@" "$@"
"#;

/// The real `git` on the ambient `PATH`.
fn real_git() -> PathBuf {
    let path = std::env::var("PATH").expect("the suite runs with a UTF-8 PATH");
    path.split(':')
        .map(|dir| Path::new(dir).join("git"))
        .find(|candidate| candidate.is_file())
        .expect("git is on PATH")
}

/// A throwaway repository under a shell-hostile path, with a bare remote beside it: the
/// tool's own bytes and the two scripts it runs, committed; `main` pushed; and the loop
/// branch of one opened run checked out and pushed, its tip the commit that added the
/// opening record.
pub(crate) struct StepRig {
    dir: ScratchDir,
    pub(crate) root: PathBuf,
    pub(crate) origin: PathBuf,
    /// The scratch root a `state` act is handed: outside the repository, of plain segments.
    pub(crate) scratch: PathBuf,
    home: PathBuf,
    trace: PathBuf,
    path: String,
}

/// What a call of the tool left: its exit status, the one line it printed — held to the
/// hash it ends with before it is read — its stderr, and the git commands it ran.
pub(crate) struct Stepped {
    pub(crate) code: i32,
    pub(crate) raw: String,
    pub(crate) line: Value,
    pub(crate) stderr: String,
    pub(crate) trace: Vec<String>,
}

impl Stepped {
    /// The act ran to its end and says `status`.
    pub(crate) fn done(&self, act: &str, status: &str) -> &Value {
        assert_eq!(
            self.code, 0,
            "`{act}` exits 0\nline: {}\nstderr: {}",
            self.raw, self.stderr
        );
        assert_eq!(
            self.line["act"], act,
            "the line names its act: {}",
            self.raw
        );
        assert_eq!(self.line["status"], status, "the line: {}", self.raw);
        assert!(
            self.stderr.is_empty(),
            "an act that is done prints nothing on stderr: {}",
            self.stderr
        );
        &self.line
    }

    /// The act refused with `word`: its status, one line on stderr that opens with the
    /// word, and a line on stdout that carries the same word and a halt report.
    pub(crate) fn refused(&self, word: &str) -> &Value {
        let status = REFUSALS
            .iter()
            .find(|(known, _)| *known == word)
            .map(|(_, status)| *status)
            .unwrap_or_else(|| panic!("`{word}` is a refusal of the tool"));
        assert_eq!(
            self.code, status,
            "refused `{word}` exits {status}\nline: {}\nstderr: {}",
            self.raw, self.stderr
        );
        let opening = format!("stabilize-step: refused {word}: ");
        assert!(
            self.stderr.starts_with(&opening) && self.stderr.trim_end().lines().count() == 1,
            "a refusal is one line on stderr that opens `{opening}`: {}",
            self.stderr
        );
        assert_eq!(self.line["status"], "halted", "the line: {}", self.raw);
        assert_eq!(self.line["refused"], word, "the line: {}", self.raw);
        let halt = &self.line["halt"];
        assert!(
            halt["root_cause"]
                .as_str()
                .is_some_and(|cause| cause.starts_with(&format!("{word}: "))),
            "the halt report's root cause opens with the word: {}",
            self.raw
        );
        for field in ["evidence", "tree_state", "recommendation"] {
            assert!(halt[field].is_string(), "the halt report has `{field}`");
        }
        &self.line
    }
}

fn sha256(text: &str) -> String {
    let mut child = Command::new("python3")
        .args([
            "-c",
            "import hashlib, sys; sys.stdout.write(hashlib.sha256(sys.stdin.buffer.read()).hexdigest())",
        ])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("spawn python3");
    child_stdin::feed(&mut child, text);
    let out = child.wait_with_output().expect("python3 exits");
    assert!(out.status.success(), "python3 hashes the text");
    String::from_utf8(out.stdout).expect("a hex digest")
}

/// The one line of a call's stdout, as the object it is — after the line is held to the
/// sha256 it ends with, which is the sha256 of the line without that field.
fn line_of(stdout: &str) -> Value {
    assert!(
        stdout.ends_with('\n') && stdout.trim_end().lines().count() == 1,
        "the tool prints ONE line on stdout: {stdout:?}"
    );
    let line = stdout.trim_end_matches('\n');
    let marker = ", \"sha256\": \"";
    let at = line
        .rfind(marker)
        .unwrap_or_else(|| panic!("the line ends with its sha256: {line}"));
    let hash = &line[at + marker.len()..];
    let hash = hash
        .strip_suffix("\"}")
        .unwrap_or_else(|| panic!("the sha256 is the line's last field: {line}"));
    assert_eq!(
        sha256(&format!("{}}}", &line[..at])),
        hash,
        "the line's sha256 is that of the line without it: {line}"
    );
    serde_json::from_str(line).unwrap_or_else(|e| panic!("the line is JSON ({e}): {line}"))
}

impl StepRig {
    /// The rig with its one run opened: the loop branch checked out and pushed, its tip the
    /// commit that added the opening record.
    pub(crate) fn new(label: &str) -> Self {
        let rig = Self::unopened(label);
        rig.git(&["switch", "-q", "-c", LOOP]);
        rig.write(&format!("{RUN_DIR}/opening.md"), "# the opening record\n");
        rig.commit("docs(record): the run is opened");
        rig.git(&["push", "-q", "origin", LOOP]);
        rig
    }

    /// The rig before any run is opened: `main` checked out and pushed, and nothing else —
    /// for a suite that opens the run itself.
    pub(crate) fn unopened(label: &str) -> Self {
        let dir = ScratchDir::new(&format!("stabilize-step-{label}"));
        let root = dir.path().join(HOSTILE_ROOT);
        let origin = dir.path().join("origin.git");
        let scratch = dir.path().join("scratch");
        let home = dir.path().join("home");
        let bin = dir.path().join("bin");
        for made in [&root.join("dev"), &scratch, &home, &bin] {
            fs::create_dir_all(made).expect("create a rig directory");
        }
        let rig = StepRig {
            trace: dir.path().join("trace"),
            path: format!(
                "{}:{}",
                bin.display(),
                std::env::var("PATH").expect("the suite runs with a UTF-8 PATH")
            ),
            dir,
            root,
            origin,
            scratch,
            home,
        };
        rig.on_path(
            "git",
            &GIT_SHIM.replace("@GIT@", &real_git().display().to_string()),
        );

        // The base: the tool and what it runs, a product path, the two logs, a file that
        // is neither.
        for script in [TOOL, RECORD, MERGE_LOGS] {
            fs::copy(repo_root().join(script), rig.root.join(script))
                .unwrap_or_else(|e| panic!("copy `{script}` into the rig: {e}"));
        }
        rig.write("Cargo.toml", "[workspace]\n");
        rig.write("crates/a.txt", "a\n");
        rig.write("README.md", "a readme\n");
        rig.write(
            "DECISIONS.md",
            "# Decisions\n\nThe log.\n\n## 2026-01-01 — the base\n\nThe base entry.\n",
        );
        rig.write(
            "implementation/project-history.md",
            "# History\n\nThe record.\n",
        );
        rig.git(&["init", "-q", "-b", "main"]);
        rig.git(&["config", "core.hooksPath", "/dev/null"]);
        rig.commit("chore: the base");
        let bare = rig
            .hermetic(Command::new("git"))
            .args(["init", "-q", "--bare"])
            .arg(&rig.origin)
            .output()
            .expect("run git init --bare");
        assert!(bare.status.success(), "init the bare remote: {bare:?}");
        rig.git(&["remote", "add", "origin", &rig.origin.display().to_string()]);
        rig.git(&["push", "-q", "origin", "main"]);
        rig
    }

    /// The directory the rig lies in — the repository, its remote and the scratch root are
    /// its children — for what a suite keeps beside them.
    pub(crate) fn dir(&self) -> &Path {
        self.dir.path()
    }

    /// An executable placed first on the `PATH` of every child of the rig.
    pub(crate) fn on_path(&self, name: &str, script: &str) {
        let tool = self.dir.path().join("bin").join(name);
        fs::write(&tool, script).unwrap_or_else(|e| panic!("write the rig's `{name}`: {e}"));
        fs::set_permissions(&tool, fs::Permissions::from_mode(0o755))
            .unwrap_or_else(|e| panic!("chmod the rig's `{name}`: {e}"));
    }

    /// The environment every child of the rig runs in: its own home, no git configuration
    /// of the machine's, a fixed identity and fixed dates.
    pub(crate) fn hermetic(&self, mut command: Command) -> Command {
        command
            .env("PATH", &self.path)
            .env("HOME", &self.home)
            .env("GIT_CONFIG_NOSYSTEM", "1")
            .env("GIT_CONFIG_GLOBAL", "/dev/null")
            .env("GIT_TERMINAL_PROMPT", "0")
            .env("GIT_AUTHOR_NAME", "The Rig")
            .env("GIT_AUTHOR_EMAIL", "rig@example.invalid")
            .env("GIT_AUTHOR_DATE", "2026-01-01T00:00:00Z")
            .env("GIT_COMMITTER_NAME", "The Rig")
            .env("GIT_COMMITTER_EMAIL", "rig@example.invalid")
            .env("GIT_COMMITTER_DATE", "2026-01-01T00:00:00Z")
            .env("PYTHONPYCACHEPREFIX", self.dir.path().join("pycache"))
            .env_remove("GIT_DIR")
            .env_remove("GIT_WORK_TREE")
            .env_remove("GIT_INDEX_FILE")
            .env_remove("STEP_TRACE");
        command
    }

    fn git_at(&self, at: &Path, args: &[&str]) -> std::process::Output {
        self.hermetic(Command::new("git"))
            .args(args)
            .current_dir(at)
            .output()
            .expect("run git")
    }

    /// A git command of the fixture's, in the work clone: it must succeed. Its stdout,
    /// without the last line break.
    pub(crate) fn git(&self, args: &[&str]) -> String {
        let out = self.git_at(&self.root, args);
        assert!(
            out.status.success(),
            "`git {}` in the rig: {}",
            args.join(" "),
            String::from_utf8_lossy(&out.stderr)
        );
        String::from_utf8_lossy(&out.stdout)
            .trim_end_matches('\n')
            .to_owned()
    }

    /// Whether a git command of the fixture's succeeds.
    pub(crate) fn git_ok(&self, args: &[&str]) -> bool {
        self.git_at(&self.root, args).status.success()
    }

    pub(crate) fn write(&self, rel: &str, text: &str) {
        let path = self.root.join(rel);
        fs::create_dir_all(path.parent().expect("a file has a directory"))
            .expect("create the file's directory");
        fs::write(&path, text).unwrap_or_else(|e| panic!("write `{rel}` in the rig: {e}"));
    }

    pub(crate) fn read(&self, rel: &str) -> String {
        fs::read_to_string(self.root.join(rel))
            .unwrap_or_else(|e| panic!("read `{rel}` in the rig: {e}"))
    }

    /// Everything in the tree committed; the commit's sha.
    pub(crate) fn commit(&self, subject: &str) -> String {
        self.git(&["add", "-A"]);
        self.git(&["commit", "-q", "-m", subject]);
        self.rev("HEAD")
    }

    /// One file written and committed.
    pub(crate) fn change(&self, rel: &str, text: &str, subject: &str) -> String {
        self.write(rel, text);
        self.commit(subject)
    }

    pub(crate) fn rev(&self, name: &str) -> String {
        self.git(&["rev-parse", name])
    }

    pub(crate) fn branch(&self) -> String {
        self.git(&["branch", "--show-current"])
    }

    /// What `git status --porcelain --untracked-files=all` prints.
    pub(crate) fn status(&self) -> String {
        self.git(&["status", "--porcelain", "--untracked-files=all"])
    }

    /// The sha the remote holds for a branch, or none.
    pub(crate) fn remote(&self, branch: &str) -> Option<String> {
        let out = self.git_at(
            &self.origin,
            &[
                "rev-parse",
                "--verify",
                "--quiet",
                &format!("refs/heads/{branch}"),
            ],
        );
        out.status
            .success()
            .then(|| String::from_utf8_lossy(&out.stdout).trim().to_owned())
    }

    /// A commit somebody else pushed: made in a second clone, so that the remote's branch
    /// has a commit the work clone lacks.
    pub(crate) fn pushed_by_another(&self, branch: &str, rel: &str, text: &str) -> String {
        let other = self
            .dir
            .path()
            .join(format!("other-{}", branch.replace('/', "-")));
        if !other.exists() {
            let cloned = self
                .hermetic(Command::new("git"))
                .args(["clone", "-q", "--branch", branch])
                .arg(&self.origin)
                .arg(&other)
                .output()
                .expect("run git clone");
            assert!(cloned.status.success(), "clone the remote: {cloned:?}");
        }
        fs::create_dir_all(other.join(rel).parent().expect("a directory"))
            .expect("create the file's directory");
        fs::write(other.join(rel), text).expect("write the other clone's file");
        for args in [
            vec!["add", "-A"],
            vec!["commit", "-q", "-m", "docs: a commit somebody else pushed"],
            vec!["push", "-q", "origin", branch],
        ] {
            let out = self.git_at(&other, &args);
            assert!(
                out.status.success(),
                "`git {}` elsewhere: {out:?}",
                args.join(" ")
            );
        }
        String::from_utf8_lossy(&self.git_at(&other, &["rev-parse", "HEAD"]).stdout)
            .trim()
            .to_owned()
    }

    /// A hook of the bare remote — the clone's own hooks are off.
    fn remote_hook(&self, name: &str, body: &str) {
        let hook = self.origin.join("hooks").join(name);
        fs::create_dir_all(hook.parent().expect("the hooks directory")).expect("create hooks/");
        fs::write(&hook, format!("#!/bin/sh\n{body}\n")).expect("write the remote's hook");
        fs::set_permissions(&hook, fs::Permissions::from_mode(0o755)).expect("chmod the hook");
    }

    fn ran(&self, command: Command) -> Stepped {
        fs::write(&self.trace, "").expect("empty the trace");
        let out = self
            .hermetic(command)
            .env("STEP_TRACE", &self.trace)
            .stdin(Stdio::null())
            .output()
            .expect("run the tool");
        let raw = String::from_utf8_lossy(&out.stdout).into_owned();
        Stepped {
            code: out.status.code().expect("the tool exits"),
            line: line_of(&raw),
            raw,
            stderr: String::from_utf8_lossy(&out.stderr).into_owned(),
            trace: fs::read_to_string(&self.trace)
                .expect("read the trace")
                .lines()
                .map(str::to_owned)
                .collect(),
        }
    }

    /// The tool, called by its path from a directory that is not the repository: it finds
    /// its repository from where it lies.
    pub(crate) fn step(&self, args: &[&str]) -> Stepped {
        let mut command = Command::new(self.root.join(TOOL));
        command.args(args).current_dir(self.dir.path());
        self.ran(command)
    }

    /// A command line exactly as a step's prompt gives it, run as an agent runs it: by a
    /// shell, from the repository's root.
    pub(crate) fn shell(&self, command_line: &str) -> Stepped {
        let mut command = Command::new("sh");
        command.args(["-c", command_line]).current_dir(&self.root);
        self.ran(command)
    }

    /// The round's branch opened from the loop branch and checked out, by plain git.
    pub(crate) fn round(&self, number: u32) -> String {
        let branch = format!("{ROUNDS}{number}");
        self.git(&["switch", "-q", LOOP]);
        self.git(&["switch", "-q", "--no-track", "-c", &branch, LOOP]);
        branch
    }
}

fn with<'a>(head: &[&'a str], lists: &[(&'a str, &'a [String])]) -> Vec<&'a str> {
    let mut args = head.to_vec();
    for (flag, values) in lists {
        for value in *values {
            args.push(flag);
            args.push(value.as_str());
        }
    }
    args
}

fn lines(list: &[&str]) -> Vec<String> {
    list.iter().map(|line| (*line).to_owned()).collect()
}

// ---------------------------------------------------------------------------
// git-state
// ---------------------------------------------------------------------------

fn git_state<'a>(stage: &'a str, product: &'a [String]) -> Vec<&'a str> {
    with(
        &[
            "git-state",
            "--stage",
            stage,
            "--loop",
            LOOP,
            "--rounds",
            ROUNDS,
            "--run-dir",
            RUN_DIR,
        ],
        &[("--product", product)],
    )
}

#[test]
fn git_state_reads_the_state_a_stage_starts_from() {
    let rig = StepRig::new("git-state");
    let product = product();
    let opening = rig.rev("HEAD");
    // What a stage wrote through the record script and no record step has committed is
    // what a tree may hold: a report, and the round's scope — each at the one path the
    // record script names for it (`pending`). A stage that stopped after its scope step
    // left exactly these.
    let report = format!("{RUN_DIR}/r1/reports/test/scope.a1.md");
    let scope = format!("{RUN_DIR}/r1/scope.md");
    rig.write(&report, "a report\n");
    rig.write(&scope, "the scope\n");

    let seen = rig.step(&git_state("test", &product));
    let line = seen.done("git-state", "ready");
    assert_eq!(
        *line,
        json!({
            "act": "git-state", "status": "ready", "branch": LOOP, "head": opening,
            "loop_head": opening, "opening": opening, "untracked": [report, scope],
            "pending": null, "sha256": line["sha256"],
        })
    );
    assert_eq!(
        seen.trace,
        lines(&[
            "branch --show-current",
            "status --porcelain --untracked-files=all",
            &format!("ls-remote --exit-code --heads origin {LOOP}"),
            &format!("fetch origin {LOOP}"),
            &format!("merge-base --is-ancestor origin/{LOOP} {LOOP}"),
            &format!("log --diff-filter=A --format=%H -n 1 {LOOP} -- {RUN_DIR}/opening.md"),
            &format!(
                "log --first-parent --no-merges --format=%H {opening}..{LOOP} -- {}",
                product.join(" ")
            ),
            &format!("log --first-parent --merges --format=%s {opening}..{LOOP}"),
            "rev-parse HEAD",
            &format!("rev-parse {LOOP}"),
        ]),
        "the commands of the step's list, in its order"
    );
}

#[test]
fn git_state_holds_the_branch_to_the_stage() {
    let rig = StepRig::new("git-state-branch");
    let product = product();
    let branch = rig.round(1);
    // The `test` stage works on the loop branch; the `fix` stage on it or on a round's.
    rig.step(&git_state("test", &product))
        .refused("wrong-branch");
    let line = rig.step(&git_state("fix", &product));
    let line = line.done("git-state", "ready");
    assert_eq!(line["branch"], branch.as_str());
    assert_eq!(line["loop_head"], rig.rev(LOOP).as_str());
    // A branch that is neither is refused for both.
    rig.git(&["switch", "-q", "main"]);
    for stage in ["test", "fix"] {
        let seen = rig.step(&git_state(stage, &product));
        seen.refused("wrong-branch");
        assert_eq!(seen.trace[0], "branch --show-current");
        assert!(
            seen.line["halt"]["tree_state"]
                .as_str()
                .is_some_and(|tree| tree.contains("on `main`")),
            "a halt says what is checked out: {}",
            seen.raw
        );
    }
}

/// What is done to a clean tree before a step is asked for.
type Soil<'a> = &'a dyn Fn(&StepRig);

#[test]
fn git_state_refuses_a_tree_that_holds_more_than_reports() {
    let product = product();
    let dirty: [(&str, Soil); 6] = [
        ("a tracked file modified", &|rig| {
            rig.write("README.md", "changed\n")
        }),
        ("an untracked file outside the run", &|rig| {
            rig.write("notes.txt", "x\n")
        }),
        // A file under the run's directory is pending only at a path the record script
        // names: a note beside the tables, and a file beside the reports that is no
        // report's name, are a tree no stage starts from.
        ("an untracked file of the run that is no record's", &|rig| {
            rig.write(&format!("{RUN_DIR}/notes.md"), "a note\n");
        }),
        (
            "an untracked file beside the reports that is no report",
            &|rig| {
                rig.write(&format!("{RUN_DIR}/r1/reports/test/notes.txt"), "x\n");
            },
        ),
        // And a table nobody applied as a batch is a table changed by hand.
        (
            "a tracked file of the run modified, with no batch applied",
            &|rig| {
                rig.write(&format!("{RUN_DIR}/opening.md"), "# another opening\n");
            },
        ),
        ("a staged file", &|rig| {
            rig.write(&format!("{RUN_DIR}/r1/reports/test/x.a1.md"), "x\n");
            rig.git(&["add", "-A"]);
        }),
    ];
    for (what, soil) in dirty {
        let rig = StepRig::new("git-state-dirty");
        soil(&rig);
        let before = rig.status();
        let seen = rig.step(&git_state("test", &product));
        seen.refused("dirty");
        assert_eq!(
            rig.status(),
            before,
            "{what}: a refusal leaves the tree as it was"
        );
        assert_eq!(
            seen.trace.len(),
            4,
            "{what}: nothing runs past the check but the halt's two reads: {:?}",
            seen.trace
        );
    }
}

#[test]
fn git_state_refuses_a_pushed_loop_branch_that_is_ahead() {
    let rig = StepRig::new("git-state-ahead");
    let head = rig.rev("HEAD");
    rig.pushed_by_another(LOOP, &format!("{RUN_DIR}/note.md"), "a record\n");
    rig.step(&git_state("test", &product()))
        .refused("remote-ahead");
    assert_eq!(
        rig.rev("HEAD"),
        head,
        "the local branch is never moved to the remote's"
    );
}

#[test]
fn git_state_asserts_that_product_paths_move_only_through_a_rounds_merge() {
    let product = product();

    // A round's merge may carry a product change.
    let rig = StepRig::new("git-state-merge");
    let branch = rig.round(1);
    rig.change("crates/a.txt", "fixed\n", "fix: a finding");
    rig.git(&["switch", "-q", LOOP]);
    rig.git(&["merge", "-q", "--no-ff", "--no-edit", &branch]);
    rig.git(&["push", "-q", "origin", LOOP]);
    rig.step(&git_state("test", &product))
        .done("git-state", "ready");

    // A commit made directly on the loop branch may not.
    let direct = rig.change("crates/a.txt", "tuned\n", "chore: a direct change");
    let seen = rig.step(&git_state("test", &product));
    let line = seen.refused("product-path");
    assert!(
        line["halt"]["root_cause"]
            .as_str()
            .is_some_and(|cause| cause.contains(&direct))
            && line["halt"]["evidence"]
                .as_str()
                .is_some_and(|shown| shown.contains("crates/a.txt")),
        "the commit and the path it changed are named: {}",
        seen.raw
    );

    // A direct commit that touches no product path is no concern of the assert's — and a
    // merge that is not a round's is refused whatever it carries.
    let rig = StepRig::new("git-state-foreign");
    rig.change("README.md", "tuned\n", "docs: a direct change");
    rig.step(&git_state("test", &product))
        .done("git-state", "ready");
    rig.git(&["switch", "-q", "-c", "work/side"]);
    rig.change("notes.md", "x\n", "docs: a side note");
    rig.git(&["switch", "-q", LOOP]);
    rig.git(&["merge", "-q", "--no-ff", "--no-edit", "work/side"]);
    rig.step(&git_state("test", &product))
        .refused("foreign-merge");

    // Before the opening record is committed the assert does not run.
    let rig = StepRig::new("git-state-unopened");
    rig.git(&["switch", "-q", "-c", "fix/later", "main"]);
    rig.change("crates/a.txt", "direct\n", "chore: before any opening");
    let seen = rig.step(&with(
        &[
            "git-state",
            "--stage",
            "test",
            "--loop",
            "fix/later",
            "--rounds",
            "fix/later-r",
            "--run-dir",
            "completions/artifacts/later",
        ],
        &[("--product", &product)],
    ));
    assert_eq!(seen.done("git-state", "ready")["opening"], "");
    assert!(
        !seen
            .trace
            .iter()
            .any(|call| call.contains("--first-parent")),
        "without an opening the path-class assert does not run: {:?}",
        seen.trace
    );
}

// ---------------------------------------------------------------------------
// state · check-reports
// ---------------------------------------------------------------------------

#[test]
fn state_relays_the_record_scripts_document_whole() {
    let rig = StepRig::new("state");
    let scratch = rig.scratch.display().to_string();
    let seen = rig.step(&[
        "state",
        "--run",
        RUN,
        "--scratch",
        &scratch,
        "--tag",
        "test-1",
    ]);
    let line = seen.done("state", "read");
    assert_eq!(line["branch"], LOOP);
    assert_eq!(line["state"]["run"], RUN);
    assert_eq!(line["state"]["opened"], true);
    assert_eq!(
        line["state"]["next"], "not-ready",
        "the document is the record script's"
    );
    // The document is kept where the step says, as the record script printed it.
    let kept = rig.scratch.join("state/test-1.json");
    assert_eq!(line["file"], kept.display().to_string().as_str());
    let kept: Value = serde_json::from_str(&fs::read_to_string(kept).expect("read the kept state"))
        .expect("the kept state is JSON");
    assert_eq!(kept, line["state"]);
    assert_eq!(seen.trace, lines(&["branch --show-current"]));

    // A run that was never opened is an answer, as it is the record script's.
    let seen = rig.step(&[
        "state",
        "--run",
        "never",
        "--scratch",
        &scratch,
        "--tag",
        "test-2",
    ]);
    assert_eq!(
        seen.done("state", "read")["state"],
        json!({"run": "never", "opened": false})
    );

    // A record the script refuses to read is the step's refusal, with that script's line.
    rig.write(&format!("{RUN_DIR}/ledger.md"), "not a table\n");
    let seen = rig.step(&[
        "state",
        "--run",
        RUN,
        "--scratch",
        &scratch,
        "--tag",
        "test-3",
    ]);
    let line = seen.refused("record");
    assert!(
        line["halt"]["evidence"]
            .as_str()
            .is_some_and(|shown| shown.contains("stabilize-record: refused corrupt")),
        "the record script's refusal is the evidence: {}",
        seen.raw
    );
    assert!(!rig.scratch.join("state/test-3.json").exists());
}

#[test]
fn check_reports_relays_the_check_and_a_failed_check_is_its_answer() {
    let rig = StepRig::new("check-reports");
    let check = [
        "check-reports",
        "--run",
        RUN,
        "--round",
        "1",
        "--stage",
        "test",
        "--attempt",
        "1",
        "--",
        "scope",
    ];
    let seen = rig.step(&check);
    let line = seen.done("check-reports", "checked");
    assert_eq!(line["check"]["ok"], false);
    assert_eq!(line["check"]["missing"], json!(["scope"]));
    assert!(
        seen.trace.is_empty(),
        "the check runs no git: {:?}",
        seen.trace
    );

    rig.write(
        &format!("{RUN_DIR}/r1/reports/test/scope.a1.md"),
        "a report\n",
    );
    assert_eq!(
        rig.step(&check).done("check-reports", "checked")["check"]["ok"],
        true
    );

    // The fix stage's flags reach the record script too.
    let seen = rig.step(&[
        "check-reports",
        "--run",
        RUN,
        "--round",
        "1",
        "--stage",
        "fix",
        "--cycle",
        "2",
        "--attempt",
        "1",
        "--",
        "audit-review",
    ]);
    assert_eq!(
        seen.done("check-reports", "checked")["check"]["dir"],
        format!("{RUN_DIR}/r1/reports/fix/c2").as_str()
    );

    // What the record script refuses is the step's refusal.
    let seen = rig.step(&[
        "check-reports",
        "--run",
        RUN,
        "--round",
        "1",
        "--stage",
        "test",
        "--attempt",
        "1",
        "--",
        "Not A Slug",
    ]);
    seen.refused("record");
}

// ---------------------------------------------------------------------------
// find-round · open-round · push
// ---------------------------------------------------------------------------

#[test]
fn find_round_lists_the_rounds_branches_here_and_on_origin() {
    let rig = StepRig::new("find-round");
    let prefix = format!("{ROUNDS}1");
    let find = ["find-round", "--prefix", prefix.as_str()];
    let line = rig.step(&find);
    let line = line.done("find-round", "ready");
    assert_eq!(line["local"], json!([]));
    assert_eq!(line["remote"], json!([]));

    for name in [
        "fix/rc24-r1",
        "fix/rc24-r1-part1",
        "fix/rc24-r10",
        "fix/rc24-r2",
    ] {
        rig.git(&["branch", name]);
    }
    rig.git(&["push", "-q", "origin", "fix/rc24-r1", "fix/rc24-r2"]);
    let seen = rig.step(&find);
    let line = seen.done("find-round", "ready");
    assert_eq!(line["branch"], LOOP);
    assert_eq!(line["head"], rig.rev("HEAD").as_str());
    assert_eq!(
        line["local"],
        json!(["fix/rc24-r1", "fix/rc24-r1-part1", "fix/rc24-r10"]),
        "every local branch the prefix opens, and no other round's"
    );
    assert_eq!(line["remote"], json!(["fix/rc24-r1"]));
    assert_eq!(
        seen.trace,
        lines(&[
            &format!("branch --list {prefix}* --format=%(refname:short)"),
            &format!("ls-remote --heads origin {prefix}*"),
            "branch --show-current",
            "rev-parse HEAD",
        ])
    );
}

#[test]
fn open_round_opens_the_branch_from_the_loop_branch_or_switches_to_it() {
    let rig = StepRig::new("open-round");
    let branch = format!("{ROUNDS}1");
    let open = [
        "open-round",
        "--loop",
        LOOP,
        "--branch",
        branch.as_str(),
        "--run-dir",
        RUN_DIR,
    ];
    rig.write(
        &format!("{RUN_DIR}/r1/reports/test/scope.a1.md"),
        "a report\n",
    );

    let seen = rig.step(&open);
    let line = seen.done("open-round", "ready");
    assert_eq!(line["created"], true);
    assert_eq!(line["branch"], branch.as_str());
    assert_eq!(line["head"], rig.rev(LOOP).as_str());
    assert_eq!(rig.branch(), branch);
    assert!(
        !rig.git_ok(&[
            "rev-parse",
            "--verify",
            "--quiet",
            &format!("{branch}@{{upstream}}")
        ]),
        "a round's branch is opened without an upstream"
    );
    assert_eq!(
        seen.trace,
        lines(&[
            "status --porcelain --untracked-files=all",
            &format!("rev-parse --verify --quiet refs/heads/{branch}"),
            &format!("switch {LOOP}"),
            &format!("switch --no-track -c {branch} {LOOP}"),
            "branch --show-current",
            "rev-parse HEAD",
        ])
    );

    // A branch that exists is switched to, wherever the tree stands.
    let tip = rig.change("crates/a.txt", "fixed\n", "fix: a finding");
    rig.git(&["switch", "-q", LOOP]);
    let seen = rig.step(&open);
    let line = seen.done("open-round", "ready");
    assert_eq!(line["created"], false);
    assert_eq!(line["head"], tip.as_str());
    assert_eq!(
        seen.trace,
        lines(&[
            "status --porcelain --untracked-files=all",
            &format!("rev-parse --verify --quiet refs/heads/{branch}"),
            &format!("switch {branch}"),
            "branch --show-current",
            "rev-parse HEAD",
        ])
    );

    // A tree that holds more than reports opens nothing.
    let rig = StepRig::new("open-round-dirty");
    rig.write("README.md", "changed\n");
    rig.step(&open).refused("dirty");
    assert!(!rig.git_ok(&[
        "rev-parse",
        "--verify",
        "--quiet",
        &format!("refs/heads/{branch}")
    ]));
    assert_eq!(rig.branch(), LOOP);
}

#[test]
fn push_pushes_the_branch_by_name_and_reads_the_remote_back() {
    let rig = StepRig::new("push");
    let branch = rig.round(1);
    let push = ["push", "--branch", branch.as_str()];
    let first = rig.change("crates/a.txt", "fixed\n", "fix: a finding");

    let seen = rig.step(&push);
    let line = seen.done("push", "ready");
    assert_eq!(
        *line,
        json!({
            "act": "push", "status": "ready", "remote_head": first, "branch": branch,
            "head": first, "sha256": line["sha256"],
        })
    );
    assert_eq!(rig.remote(&branch), Some(first));
    assert_eq!(
        seen.trace,
        lines(&[
            "branch --show-current",
            &format!("ls-remote --exit-code --heads origin {branch}"),
            &format!("push origin {branch}"),
            &format!("ls-remote --exit-code --heads origin {branch}"),
            &format!("rev-parse {branch}"),
            "rev-parse HEAD",
        ]),
        "a branch that is not pushed yet has nothing to compare"
    );

    // A branch that is pushed is first held to its pushed tip.
    let second = rig.change("crates/a.txt", "fixed again\n", "fix: the same finding");
    let seen = rig.step(&push);
    assert_eq!(seen.done("push", "ready")["remote_head"], second.as_str());
    assert_eq!(
        seen.trace[1..4],
        lines(&[
            &format!("ls-remote --exit-code --heads origin {branch}"),
            &format!("fetch origin {branch}"),
            &format!("merge-base --is-ancestor origin/{branch} {branch}"),
        ])
    );
}

#[test]
fn push_refuses_where_its_list_halted() {
    // A branch that is not the one the step names.
    let rig = StepRig::new("push-branch");
    let branch = rig.round(1);
    rig.git(&["switch", "-q", LOOP]);
    let seen = rig.step(&["push", "--branch", &branch]);
    seen.refused("wrong-branch");
    assert_eq!(rig.remote(&branch), None, "nothing was pushed");

    // A pushed branch with commits the local one lacks: never pulled, merged or rebased.
    let rig = StepRig::new("push-ahead");
    let head = rig.rev("HEAD");
    let theirs = rig.pushed_by_another(LOOP, &format!("{RUN_DIR}/note.md"), "a record\n");
    let seen = rig.step(&["push", "--branch", LOOP]);
    seen.refused("remote-ahead");
    assert_eq!(rig.rev("HEAD"), head);
    assert_eq!(rig.remote(LOOP), Some(theirs));
    assert!(
        !seen.trace.iter().any(|call| call.starts_with("push ")),
        "no push is tried: {:?}",
        seen.trace
    );

    // A push the remote does not take: never forced.
    let rig = StepRig::new("push-rejected");
    let pushed = rig.rev("HEAD");
    rig.change(
        &format!("{RUN_DIR}/note.md"),
        "a record\n",
        "docs(record): a note",
    );
    rig.remote_hook("pre-receive", "exit 1");
    let seen = rig.step(&["push", "--branch", LOOP]);
    seen.refused("push-rejected");
    assert_eq!(rig.remote(LOOP), Some(pushed.clone()));
    assert_eq!(
        seen.trace
            .iter()
            .filter(|call| call.starts_with("push "))
            .count(),
        1,
        "one push, by name, and no second try: {:?}",
        seen.trace
    );

    // A remote that does not stand at the local head after the push.
    let rig = StepRig::new("push-differs");
    let pushed = rig.rev("HEAD");
    rig.change(
        &format!("{RUN_DIR}/note.md"),
        "a record\n",
        "docs(record): a note",
    );
    rig.remote_hook(
        "post-receive",
        &format!("git update-ref refs/heads/{LOOP} {pushed}"),
    );
    rig.step(&["push", "--branch", LOOP])
        .refused("remote-differs");
}

// ---------------------------------------------------------------------------
// land
// ---------------------------------------------------------------------------

fn land<'a>(branch: &'a str, product: &'a [String], logs: &'a [String]) -> Vec<&'a str> {
    with(
        &[
            "land",
            "--loop",
            LOOP,
            "--branch",
            branch,
            "--run-dir",
            RUN_DIR,
        ],
        &[("--product", product), ("--log", logs)],
    )
}

/// A round with one fix on a product path and one record commit, checked out.
fn a_round(rig: &StepRig) -> String {
    let branch = rig.round(1);
    rig.change("crates/a.txt", "fixed\n", "fix: a finding");
    rig.change(
        &format!("{RUN_DIR}/r1/round.md"),
        "the round's record\n",
        "docs(record): the fix cycle's record",
    );
    branch
}

#[test]
fn land_merges_the_round_with_a_merge_commit_and_pushes_the_loop_branch() {
    let rig = StepRig::new("land");
    let (product, logs) = (product(), logs());
    let branch = a_round(&rig);
    let (pre, round_head) = (rig.rev(LOOP), rig.rev(&branch));
    let outside = format!(":(exclude){RUN_DIR}");

    let seen = rig.step(&land(&branch, &product, &logs));
    let line = seen.done("land", "merged");
    let merge = rig.rev(LOOP);
    assert_eq!(
        *line,
        json!({
            "act": "land", "status": "merged", "tip_moved": false, "moved_outside": [],
            "resolved_logs": [], "merge_commit": merge, "remote_head": merge, "head": merge,
            "sha256": line["sha256"],
        })
    );
    assert_eq!(rig.branch(), LOOP);
    assert_eq!(rig.remote(LOOP), Some(merge.clone()));
    assert_eq!(
        rig.git(&["rev-list", "--parents", "-n", "1", "HEAD"]),
        format!("{merge} {pre} {round_head}"),
        "a merge commit: the loop branch's old tip, then the round's"
    );
    assert_eq!(
        rig.git(&["log", "-1", "--format=%s"]),
        format!("Merge branch '{branch}' into {LOOP}"),
        "the subject the path-class assert reads"
    );
    assert_eq!(
        seen.trace,
        lines(&[
            "status --porcelain --untracked-files=all",
            &format!("rev-parse {branch}"),
            &format!("switch {LOOP}"),
            "rev-parse HEAD",
            &format!("ls-remote --exit-code --heads origin {LOOP}"),
            &format!("fetch origin {LOOP}"),
            &format!("merge-base --is-ancestor origin/{LOOP} {LOOP}"),
            &format!("merge-base --is-ancestor {branch} {LOOP}"),
            &format!("merge-base --is-ancestor {LOOP} {branch}"),
            &format!("merge-base {LOOP} {branch}"),
            &format!("diff --name-only {pre} {LOOP} -- . {outside}"),
            &format!("merge --no-ff --no-edit {branch}"),
            "rev-parse HEAD",
            "rev-list --parents -n 1 HEAD",
            "status --porcelain",
            &format!("diff --quiet HEAD {branch} -- {}", product.join(" ")),
            &format!("diff --quiet HEAD {branch} -- . {outside}"),
            &format!("push origin {LOOP}"),
            &format!("ls-remote --exit-code --heads origin {LOOP}"),
            &format!("rev-parse {LOOP}"),
        ]),
        "the commands of the land step's list, in its order"
    );

    // A round that landed before is said to have, and nothing else runs.
    rig.git(&["switch", "-q", &branch]);
    let seen = rig.step(&land(&branch, &product, &logs));
    assert_eq!(seen.done("land", "landed-before")["head"], merge.as_str());
    assert_eq!(
        seen.trace.last().map(String::as_str),
        Some(format!("merge-base --is-ancestor {branch} {LOOP}").as_str())
    );
}

#[test]
fn land_takes_a_loop_branch_that_moved_in_the_runs_directory_only() {
    let rig = StepRig::new("land-moved");
    let (product, logs) = (product(), logs());
    let branch = a_round(&rig);
    rig.git(&["switch", "-q", LOOP]);
    rig.change(
        &format!("{RUN_DIR}/r1/rulings.md"),
        "a ruling\n",
        "docs(record): the human's rulings",
    );
    rig.git(&["push", "-q", "origin", LOOP]);

    let seen = rig.step(&land(&branch, &product, &logs));
    let line = seen.done("land", "merged");
    assert_eq!(line["tip_moved"], true);
    assert_eq!(line["moved_outside"], json!([]));
    assert_eq!(rig.read("crates/a.txt"), "fixed\n");
    assert_eq!(rig.read(&format!("{RUN_DIR}/r1/rulings.md")), "a ruling\n");
}

/// Both sides add an entry to the decisions log at the same place.
fn both_add_a_decision(rig: &StepRig, branch: &str, theirs: &str) {
    let entry = |date: &str, who: &str| {
        format!(
            "# Decisions\n\nThe log.\n\n## {date} — {who}\n\nAn entry of {who}.\n\n## 2026-01-01 — the base\n\nThe base entry.\n"
        )
    };
    rig.git(&["switch", "-q", branch]);
    rig.change(
        "DECISIONS.md",
        &entry("2026-02-02", "ours"),
        "docs: a decision of ours",
    );
    rig.git(&["switch", "-q", theirs]);
    rig.change(
        "DECISIONS.md",
        &entry("2026-02-01", "theirs"),
        "docs: a decision of theirs",
    );
}

#[test]
fn land_resolves_a_conflict_in_the_append_only_logs_and_no_other() {
    let (product, logs) = (product(), logs());

    // Both branches appended to the decisions log: both entries stay, by dev/merge-logs.
    let rig = StepRig::new("land-logs");
    let branch = a_round(&rig);
    both_add_a_decision(&rig, &branch, LOOP);
    rig.git(&["push", "-q", "origin", LOOP]);
    let seen = rig.step(&land(&branch, &product, &logs));
    let line = seen.done("land", "merged");
    assert_eq!(line["resolved_logs"], json!(["DECISIONS.md"]));
    assert_eq!(line["moved_outside"], json!(["DECISIONS.md"]));
    assert_eq!(line["tip_moved"], true);
    let log = rig.read("DECISIONS.md");
    assert!(
        log.contains("An entry of ours.") && log.contains("An entry of theirs."),
        "both sides' entries are kept: {log}"
    );
    let merging: Vec<&str> = seen
        .trace
        .iter()
        .map(String::as_str)
        .skip_while(|call| !call.starts_with("merge --no-ff"))
        .take(5)
        .collect();
    assert_eq!(
        merging,
        [
            format!("merge --no-ff --no-edit {branch}").as_str(),
            "diff --name-only --diff-filter=U",
            "diff --name-only --diff-filter=U",
            "commit --no-edit",
            "rev-parse HEAD",
        ],
        "the conflict is listed, resolved by dev/merge-logs, listed again, and concluded"
    );
    assert!(
        !seen
            .trace
            .iter()
            .any(|call| call.contains(&format!(":(exclude){RUN_DIR}"))
                && call.starts_with("diff --quiet")),
        "where a log was resolved the merged tree is not held to the round's outside the run: {:?}",
        seen.trace
    );

    // A conflict anywhere else is the human's: the merge is aborted.
    let rig = StepRig::new("land-conflict");
    let branch = a_round(&rig);
    rig.change(
        "README.md",
        "the round's readme\n",
        "docs: the round's readme",
    );
    rig.git(&["switch", "-q", LOOP]);
    let pre = rig.change(
        "README.md",
        "the loop's readme\n",
        "docs: the loop's readme",
    );
    rig.git(&["push", "-q", "origin", LOOP]);
    let seen = rig.step(&land(&branch, &product, &logs));
    let line = seen.refused("conflict");
    assert!(
        line.get("merge_commit").is_none(),
        "no merge commit exists: {}",
        seen.raw
    );
    assert_eq!(rig.rev("HEAD"), pre, "the merge is aborted");
    assert_eq!(rig.status(), "");
    assert!(!rig.git_ok(&["rev-parse", "--verify", "--quiet", "MERGE_HEAD"]));
    assert!(seen.trace.contains(&"merge --abort".to_owned()));

    // A conflict in a log that is not a pure append is refused by dev/merge-logs, and
    // the merge is aborted.
    let rig = StepRig::new("land-merge-logs");
    let branch = a_round(&rig);
    let reworded = |how: &str| {
        format!("# Decisions\n\nThe log.\n\n## 2026-01-01 — the base\n\nThe base entry, {how}.\n")
    };
    rig.change(
        "DECISIONS.md",
        &reworded("reworded by the round"),
        "docs: a rewording",
    );
    rig.git(&["switch", "-q", LOOP]);
    let pre = rig.change(
        "DECISIONS.md",
        &reworded("reworded on the loop"),
        "docs: another",
    );
    rig.git(&["push", "-q", "origin", LOOP]);
    let seen = rig.step(&land(&branch, &product, &logs));
    seen.refused("merge-logs");
    assert_eq!(rig.rev("HEAD"), pre);
    assert_eq!(rig.status(), "");

    // A dev/merge-logs that says it resolved and left the path conflicted: the list said
    // "must print nothing" and named no abort, so the merge is left in progress.
    let rig = StepRig::new("land-unresolved");
    rig.change(
        MERGE_LOGS,
        "#!/bin/sh\nexit 0\n",
        "chore: a merge-logs that resolves nothing",
    );
    let branch = a_round(&rig);
    both_add_a_decision(&rig, &branch, LOOP);
    rig.git(&["push", "-q", "origin", LOOP]);
    rig.step(&land(&branch, &product, &logs))
        .refused("unresolved");
    assert!(
        rig.git_ok(&["rev-parse", "--verify", "--quiet", "MERGE_HEAD"]),
        "the merge is still in progress, as the step left it"
    );
}

#[test]
fn land_refuses_a_merge_whose_tree_is_not_the_rounds_and_undoes_nothing() {
    let rig = StepRig::new("land-differs");
    let (product, logs) = (product(), logs());
    let branch = a_round(&rig);
    // The loop branch took a product change while the round was out: the merge is clean,
    // and what it holds on the product paths is not what the round's audit read.
    rig.git(&["switch", "-q", LOOP]);
    let pre = rig.change("crates/b.txt", "b\n", "chore: a direct change");
    rig.git(&["push", "-q", "origin", LOOP]);
    let round_head = rig.rev(&branch);

    let seen = rig.step(&land(&branch, &product, &logs));
    let line = seen.refused("merge-differs");
    let merge = rig.rev("HEAD");
    assert_eq!(
        line["merge_commit"],
        merge.as_str(),
        "the merge commit is reported"
    );
    assert_eq!(line["moved_outside"], json!(["crates/b.txt"]));
    assert_eq!(
        rig.git(&["rev-list", "--parents", "-n", "1", "HEAD"]),
        format!("{merge} {pre} {round_head}"),
        "nothing is undone: the merge is on the local branch"
    );
    assert_eq!(rig.remote(LOOP), Some(pre), "and nothing is pushed");
    assert!(!seen.trace.iter().any(|call| call.starts_with("push ")));
}

#[test]
fn land_refuses_where_its_list_halted_before_the_merge() {
    let (product, logs) = (product(), logs());

    // The tree: clean, untracked files included — a report is not admitted here.
    let rig = StepRig::new("land-dirty");
    let branch = a_round(&rig);
    rig.write(
        &format!("{RUN_DIR}/r1/reports/fix/c1/x.a1.md"),
        "a report\n",
    );
    let seen = rig.step(&land(&branch, &product, &logs));
    seen.refused("dirty");
    assert_eq!(rig.branch(), branch, "nothing was switched");
    assert_eq!(seen.trace[0], "status --porcelain --untracked-files=all");

    // The pushed loop branch is ahead.
    let rig = StepRig::new("land-ahead");
    let branch = a_round(&rig);
    let pre = rig.rev(LOOP);
    rig.pushed_by_another(LOOP, &format!("{RUN_DIR}/note.md"), "a record\n");
    let seen = rig.step(&land(&branch, &product, &logs));
    seen.refused("remote-ahead");
    assert_eq!(rig.rev(LOOP), pre, "nothing was merged");
    assert_eq!(
        rig.branch(),
        LOOP,
        "the step had switched, and leaves everything as it is"
    );

    // A command that fails where the list expects none to: a round's branch that is not
    // there.
    let rig = StepRig::new("land-no-branch");
    let seen = rig.step(&land("fix/rc24-r9", &product, &logs));
    let line = seen.refused("git");
    assert!(
        line["halt"]["evidence"]
            .as_str()
            .is_some_and(|shown| shown.contains("`git rev-parse fix/rc24-r9` exited")),
        "the command and what it printed are the evidence: {}",
        seen.raw
    );
}

#[test]
fn a_landing_that_halted_after_its_merge_is_answered_landed_before_as_the_list_did() {
    // PRESERVED ON PURPOSE — the harness review's H5. The land step's list stopped at
    // "the round landed before" without checking the merge and without pushing; so does
    // the act. A later task of the repair turns the last three assertions.
    let rig = StepRig::new("land-halted");
    let (product, logs) = (product(), logs());
    let branch = a_round(&rig);
    let pre = rig.rev(LOOP);
    rig.remote_hook("pre-receive", "exit 1");
    let seen = rig.step(&land(&branch, &product, &logs));
    let line = seen.refused("push-rejected");
    let merge = rig.rev(LOOP);
    assert_ne!(merge, pre);
    assert_eq!(
        line["merge_commit"],
        merge.as_str(),
        "a land that halts after its merge still reports the merge commit"
    );
    assert_eq!(rig.remote(LOOP), Some(pre.clone()));

    fs::remove_file(rig.origin.join("hooks/pre-receive")).expect("lift the remote's refusal");
    let seen = rig.step(&land(&branch, &product, &logs));
    let line = seen.done("land", "landed-before");
    assert_eq!(line["head"], merge.as_str());
    assert!(line.get("remote_head").is_none(), "no remote is read");
    assert_eq!(
        rig.remote(LOOP),
        Some(pre),
        "and the merge is still not pushed"
    );
}

// ---------------------------------------------------------------------------
// round-commits · carry
// ---------------------------------------------------------------------------

#[test]
fn round_commits_lists_the_rounds_commits_and_which_touch_the_runs_directory() {
    let rig = StepRig::new("round-commits");
    let branch = rig.round(1);
    let fix = rig.change("crates/a.txt", "fixed\n", "fix: a finding");
    let record = rig.change(
        &format!("{RUN_DIR}/r1/round.md"),
        "r\n",
        "docs(record): a cycle",
    );
    rig.write("crates/a.txt", "fixed twice\n");
    let mixed = rig.change(
        &format!("{RUN_DIR}/r1/note.md"),
        "n\n",
        "chore: both at once",
    );
    let commits = [
        "round-commits",
        "--loop",
        LOOP,
        "--branch",
        branch.as_str(),
        "--run-dir",
        RUN_DIR,
    ];

    let seen = rig.step(&commits);
    let line = seen.done("round-commits", "listed");
    assert_eq!(line["all"], json!([fix, record, mixed]));
    assert_eq!(line["inside"], json!([record, mixed]));
    assert_eq!(line["outside"], json!([fix, mixed]));
    let between = format!("{LOOP}..{branch}");
    assert_eq!(
        seen.trace,
        lines(&[
            &format!("rev-list --merges --count {between}"),
            &format!("log --reverse --format=%H {between}"),
            &format!("log --reverse --format=%H {between} -- {RUN_DIR}"),
            &format!("log --reverse --format=%H {between} -- . :(exclude){RUN_DIR}"),
        ])
    );

    // A merge among a round's commits is refused: no carry-over can take it.
    rig.git(&["switch", "-q", "-c", "work/side", LOOP]);
    rig.change("notes.md", "x\n", "docs: a side note");
    rig.git(&["switch", "-q", &branch]);
    rig.git(&["merge", "-q", "--no-ff", "--no-edit", "work/side"]);
    rig.step(&commits).refused("merge-in-round");
}

fn carry<'a>(product: &'a [String], shas: &[&'a str]) -> Vec<&'a str> {
    let mut args = with(
        &["carry", "--loop", LOOP, "--run-dir", RUN_DIR],
        &[("--product", product)],
    );
    args.push("--");
    args.extend_from_slice(shas);
    args
}

#[test]
fn carry_takes_a_dropped_rounds_record_commits_over_to_the_loop_branch() {
    let rig = StepRig::new("carry");
    let product = product();
    let branch = rig.round(1);
    rig.change("crates/a.txt", "fixed\n", "fix: a finding");
    let first = rig.change(
        &format!("{RUN_DIR}/r1/round.md"),
        "r\n",
        "docs(record): a cycle",
    );
    let second = rig.change(
        &format!("{RUN_DIR}/r1/drop.md"),
        "d\n",
        "docs(record): dropped",
    );
    let pre = rig.rev(LOOP);

    let seen = rig.step(&carry(&product, &[first.as_str(), second.as_str()]));
    let line = seen.done("carry", "carried");
    let made: Vec<String> = rig
        .git(&["log", "--reverse", "--format=%H", &format!("{pre}..{LOOP}")])
        .lines()
        .map(str::to_owned)
        .collect();
    assert_eq!(made.len(), 2);
    assert_eq!(
        line["picked"],
        json!([{"from": first, "to": made[0]}, {"from": second, "to": made[1]}])
    );
    assert_eq!(line["branch"], LOOP);
    assert_eq!(line["head"], made[1].as_str());
    assert_eq!(line["remote_head"], made[1].as_str());
    assert_eq!(rig.remote(LOOP), Some(made[1].clone()));
    assert_eq!(
        rig.read("crates/a.txt"),
        "a\n",
        "the round's fix is not carried"
    );
    assert!(
        rig.git(&["log", "-1", "--format=%B"])
            .contains(&format!("(cherry picked from commit {second})")),
        "a carried commit names the commit it was picked from"
    );
    assert_eq!(
        rig.rev(&branch),
        second,
        "the round's branch is left as it is"
    );
    assert_eq!(
        seen.trace,
        lines(&[
            "status --porcelain --untracked-files=all",
            &format!("switch {LOOP}"),
            "rev-parse HEAD",
            &format!("ls-remote --exit-code --heads origin {LOOP}"),
            &format!("fetch origin {LOOP}"),
            &format!("merge-base --is-ancestor origin/{LOOP} {LOOP}"),
            &format!("cherry-pick -x {first}"),
            &format!("cherry-pick -x {second}"),
            &format!("rev-list --count {pre}..HEAD"),
            &format!("diff --name-only {pre} HEAD"),
            &format!("diff --quiet {pre} HEAD -- {}", product.join(" ")),
            &format!("push origin {LOOP}"),
            &format!("ls-remote --exit-code --heads origin {LOOP}"),
            &format!("rev-parse {LOOP}"),
            "branch --show-current",
            "rev-parse HEAD",
            &format!("log --reverse --format=%H {pre}..HEAD"),
        ]),
        "the commands of the carry step's list, in its order"
    );
}

#[test]
fn carry_refuses_where_its_list_halted() {
    let product = product();

    // A commit that changed a path outside the run's directory is carried, found, and
    // left: nothing is undone, and nothing is pushed.
    let rig = StepRig::new("carry-differs");
    rig.round(1);
    let fix = rig.change("crates/a.txt", "fixed\n", "fix: a finding");
    let pre = rig.rev(LOOP);
    let seen = rig.step(&carry(&product, &[fix.as_str()]));
    seen.refused("carry-differs");
    assert_eq!(
        rig.git(&["rev-list", "--count", &format!("{pre}..{LOOP}")]),
        "1"
    );
    assert_eq!(rig.remote(LOOP), Some(pre));

    // The list's third check — no product path changed — is its own: a product path that
    // lies inside the run's directory passes the check before it and is caught here.
    let rig = StepRig::new("carry-product");
    rig.round(1);
    let record = rig.change(
        &format!("{RUN_DIR}/r1/round.md"),
        "r\n",
        "docs(record): a cycle",
    );
    let inside = vec![format!("{RUN_DIR}/r1")];
    let seen = rig.step(&carry(&inside, &[record.as_str()]));
    assert!(
        seen.refused("carry-differs")["halt"]["root_cause"]
            .as_str()
            .is_some_and(|cause| cause.contains("a product path")),
        "the product paths are held on their own: {}",
        seen.raw
    );

    // A pick that stops is aborted; the commits carried before it stay.
    let rig = StepRig::new("carry-stopped");
    rig.round(1);
    let first = rig.change(
        &format!("{RUN_DIR}/r1/round.md"),
        "r\n",
        "docs(record): a cycle",
    );
    let second = rig.change(
        &format!("{RUN_DIR}/r1/note.md"),
        "theirs\n",
        "docs(record): a note",
    );
    rig.git(&["switch", "-q", LOOP]);
    rig.change(
        &format!("{RUN_DIR}/r1/note.md"),
        "ours\n",
        "docs(record): another note",
    );
    rig.git(&["push", "-q", "origin", LOOP]);
    let pre = rig.rev(LOOP);
    let seen = rig.step(&carry(&product, &[first.as_str(), second.as_str()]));
    seen.refused("pick-stopped");
    assert_eq!(
        rig.git(&["rev-list", "--count", &format!("{pre}..{LOOP}")]),
        "1"
    );
    assert_eq!(rig.status(), "", "the stopped pick is aborted");
    assert!(!rig.git_ok(&["rev-parse", "--verify", "--quiet", "CHERRY_PICK_HEAD"]));
    assert_eq!(rig.remote(LOOP), Some(pre));

    // A tree that is not clean carries nothing; and a commit is named by its full sha.
    let rig = StepRig::new("carry-dirty");
    let head = rig.rev("HEAD");
    rig.write("notes.txt", "x\n");
    rig.step(&carry(&product, &[head.as_str()]))
        .refused("dirty");
    let seen = rig.step(&carry(&product, &[&head[..12]]));
    seen.refused("usage");
    assert!(seen.trace.is_empty());
}

// ---------------------------------------------------------------------------
// sync-main
// ---------------------------------------------------------------------------

fn sync(logs: &[String]) -> Vec<&str> {
    with(&["sync-main", "--loop", LOOP], &[("--log", logs)])
}

/// The four ways the sync act ends, each with the git commands it ran.
fn sync_runs() -> Vec<(&'static str, Vec<String>)> {
    let logs = logs();
    let mut runs = Vec::new();

    // The branch holds main already.
    let rig = StepRig::new("sync-up-to-date");
    let head = rig.rev("HEAD");
    let seen = rig.step(&sync(&logs));
    let line = seen.done("sync-main", "up-to-date");
    assert_eq!(line["head"], head.as_str());
    assert_eq!(line["origin_main"], rig.rev("origin/main").as_str());
    assert_eq!(
        seen.trace,
        lines(&[
            "branch --show-current",
            "status --porcelain",
            "rev-parse HEAD",
            "fetch origin main",
            "rev-parse origin/main",
            &format!("merge-base --is-ancestor origin/main {LOOP}"),
            "rev-parse HEAD",
        ])
    );
    runs.push(("up to date", seen.trace));

    // main moved: a merge commit, and nothing pushed.
    let rig = StepRig::new("sync-merged");
    let pre = rig.rev("HEAD");
    let theirs = rig.pushed_by_another("main", "notes.md", "from main\n");
    let seen = rig.step(&sync(&logs));
    let line = seen.done("sync-main", "merged");
    let merge = rig.rev("HEAD");
    assert_eq!(line["head"], merge.as_str());
    assert_eq!(line["origin_main"], theirs.as_str());
    assert_eq!(line["resolved_logs"], json!([]));
    assert_eq!(
        rig.git(&["rev-list", "--parents", "-n", "1", "HEAD"]),
        format!("{merge} {pre} {theirs}")
    );
    assert_eq!(rig.remote(LOOP), Some(pre), "the sync pushes nothing");
    assert_eq!(rig.remote("main"), Some(theirs));
    assert_eq!(
        seen.trace,
        lines(&[
            "branch --show-current",
            "status --porcelain",
            "rev-parse HEAD",
            "fetch origin main",
            "rev-parse origin/main",
            &format!("merge-base --is-ancestor origin/main {LOOP}"),
            "merge --no-ff --no-edit origin/main",
            "rev-list --parents -n 1 HEAD",
            "rev-parse origin/main",
            "status --porcelain",
            "rev-parse HEAD",
        ]),
        "the commands of the sync step's list, in its order"
    );
    runs.push(("merged", seen.trace));

    // Both appended to the decisions log.
    let rig = StepRig::new("sync-logs");
    let entry = |date: &str, who: &str| {
        format!(
            "# Decisions\n\nThe log.\n\n## {date} — {who}\n\nAn entry of {who}.\n\n## 2026-01-01 — the base\n\nThe base entry.\n"
        )
    };
    rig.change(
        "DECISIONS.md",
        &entry("2026-02-02", "the run"),
        "docs: a decision of the run's",
    );
    rig.pushed_by_another("main", "DECISIONS.md", &entry("2026-02-01", "main"));
    let seen = rig.step(&sync(&logs));
    assert_eq!(
        seen.done("sync-main", "merged")["resolved_logs"],
        json!(["DECISIONS.md"])
    );
    let log = rig.read("DECISIONS.md");
    assert!(log.contains("An entry of the run.") && log.contains("An entry of main."));
    runs.push(("a log resolved", seen.trace));

    // A conflict outside the logs: aborted, and the human's.
    let rig = StepRig::new("sync-conflict");
    let pre = rig.change("README.md", "the run's readme\n", "docs: the run's readme");
    rig.pushed_by_another("main", "README.md", "main's readme\n");
    let seen = rig.step(&sync(&logs));
    seen.refused("conflict");
    assert_eq!(rig.rev("HEAD"), pre);
    assert_eq!(rig.status(), "");
    runs.push(("a conflict", seen.trace));

    runs
}

#[test]
fn sync_main_merges_origin_main_into_the_loop_branch_and_pushes_nothing() {
    sync_runs();

    // The branch and the tree the step starts from.
    let logs = logs();
    let rig = StepRig::new("sync-branch");
    rig.round(1);
    rig.step(&sync(&logs)).refused("wrong-branch");
    rig.git(&["switch", "-q", LOOP]);
    rig.write("notes.txt", "x\n");
    rig.step(&sync(&logs)).refused("dirty");
}

/// The git commands a harness's step lists: every `` `git …` `` between backticks in the
/// numbered lines of its prompt function, the branch's variable filled in.
fn listed_git_commands(
    harness: &str,
    step: &str,
    variable: &str,
    branch: &str,
) -> BTreeSet<String> {
    let at = harness
        .find(&format!("\nfunction {step}("))
        .unwrap_or_else(|| panic!("the harness has the step `{step}`"));
    let body = &harness[at..];
    let body = &body[..body.find("\n}\n").expect("the function closes")];
    let mut listed = BTreeSet::new();
    for line in body.lines().filter(|line| {
        let line = line.trim_start();
        line.starts_with('\'') && line[1..].starts_with(|c: char| c.is_ascii_digit())
    }) {
        for command in line.split('`').skip(1).step_by(2) {
            if let Some(command) = command.strip_prefix("git ") {
                listed.insert(command.replace(&format!("' + {variable} + '"), branch));
            }
        }
    }
    listed
}

#[test]
fn the_sync_act_runs_the_build_harnesss_sync_step() {
    // The close's sync is one step, whichever harness returns it. The build harness lists
    // it as commands; the stabilization harness hands it to the tool. So the two are held
    // together by what the act RAN, over the four ways it ends: every git command it ran
    // is one the build harness's step lists, and every command that step lists was run.
    let listed = listed_git_commands(&read(BUILD_HARNESS), "syncMainPrompt", "m", LOOP);
    assert!(
        listed.len() >= 10 && listed.contains("merge --no-ff --no-edit origin/main"),
        "the scan found the build harness's sync step: {listed:#?}"
    );
    let ran: BTreeSet<String> = sync_runs()
        .into_iter()
        .flat_map(|(_, trace)| trace)
        .collect();
    assert_eq!(
        ran, listed,
        "the git commands the sync act ran (left) are the commands {BUILD_HARNESS}'s sync step lists (right)"
    );
}

// ---------------------------------------------------------------------------
// The names an act takes
// ---------------------------------------------------------------------------

/// The acts of the tool and the flags each requires, read from its own parser.
fn acts() -> BTreeMap<String, Vec<String>> {
    let source = read(TOOL);
    let mut acts = BTreeMap::new();
    for line in source
        .lines()
        .filter_map(|line| line.strip_prefix("    act(\""))
    {
        let name = &line[..line.find('"').expect("the act's name closes")];
        let flags = &line[line.find('[').expect("an act lists its flags") + 1..];
        let flags = &flags[..flags.find(']').expect("the list closes")];
        acts.insert(
            name.to_owned(),
            flags
                .split('"')
                .skip(1)
                .step_by(2)
                .map(str::to_owned)
                .collect(),
        );
    }
    assert!(acts.len() >= 10, "the scan found the tool's acts: {acts:?}");
    acts
}

/// A value each required flag takes.
fn valid(flag: &str, scratch: &str) -> String {
    match flag {
        "loop" => LOOP.to_owned(),
        "branch" | "prefix" => format!("{ROUNDS}1"),
        "rounds" => ROUNDS.to_owned(),
        "run-dir" => RUN_DIR.to_owned(),
        "run" => RUN.to_owned(),
        "tag" => "test-1".to_owned(),
        "scratch" => scratch.to_owned(),
        "stage" => "test".to_owned(),
        "round" | "attempt" | "calls" | "checks" => "1".to_owned(),
        "reporter" => "attempt".to_owned(),
        "commit" => "c".repeat(40),
        "gate" => format!("{scratch}/gate.txt"),
        other => panic!("the suite has no value for the flag `--{other}`: give it one"),
    }
}

#[test]
fn no_act_takes_main_or_any_branch_without_a_prefix() {
    let rig = StepRig::new("names");
    let scratch = rig.scratch.display().to_string();
    let (main, head) = (rig.remote("main"), rig.rev("HEAD"));
    let mut driven = 0;
    for (act, flags) in acts() {
        for naming in flags
            .iter()
            .filter(|flag| ["loop", "branch", "rounds", "prefix"].contains(&flag.as_str()))
        {
            for hostile in ["main", "HEAD", "+fix/rc24", "fix/rc24:main", "-f", "fix/"] {
                let mut args = vec![act.clone()];
                for flag in &flags {
                    args.push(format!("--{flag}"));
                    args.push(if flag == naming {
                        hostile.to_owned()
                    } else {
                        valid(flag, &scratch)
                    });
                }
                if act == "carry" {
                    args.push(head.clone());
                }
                let args: Vec<&str> = args.iter().map(String::as_str).collect();
                let seen = rig.step(&args);
                seen.refused("usage");
                assert!(
                    seen.trace.is_empty(),
                    "`{act} --{naming} {hostile}` ran git: {:?}",
                    seen.trace
                );
                driven += 1;
            }
        }
    }
    assert!(
        driven >= 70,
        "every naming flag of every act was driven: {driven}"
    );
    assert_eq!(rig.remote("main"), main, "main is where it was");
    assert_eq!(rig.rev("HEAD"), head);
    assert_eq!(rig.branch(), LOOP);
}

#[test]
fn the_tools_header_its_statuses_and_this_suites_table_agree() {
    let source = read(TOOL);
    // The header's table: `#   NN  word   …`.
    let header: BTreeMap<String, i32> = source
        .lines()
        .take_while(|line| line.starts_with('#'))
        .filter_map(|line| {
            let mut words = line.trim_start_matches('#').split_whitespace();
            let status: i32 = words.next()?.parse().ok()?;
            let word = words.next()?;
            (status >= 2).then(|| (word.to_owned(), status))
        })
        .collect();
    // The code's: `    "word": NN,`.
    let at = source.find("\nSTATUS = {\n").expect("the tool's statuses");
    let table = &source[at..];
    let table = &table[..table.find("\n}\n").expect("the table closes")];
    let code: BTreeMap<String, i32> = table
        .lines()
        .filter_map(|line| line.trim().strip_prefix('"'))
        .map(|line| {
            let (word, status) = line.split_once("\": ").expect("a word and its status");
            (
                word.to_owned(),
                status.trim_end_matches(',').parse().expect("a status"),
            )
        })
        .collect();
    let suite: BTreeMap<String, i32> = REFUSALS
        .iter()
        .map(|(word, status)| ((*word).to_owned(), *status))
        .collect();
    assert_eq!(
        header, code,
        "the header's refusals (left) and the code's (right)"
    );
    assert_eq!(
        code, suite,
        "the code's refusals (left) and this suite's (right)"
    );
    let statuses: BTreeSet<i32> = code.values().copied().collect();
    assert_eq!(
        statuses.len(),
        code.len(),
        "a status per refusal, and no two alike"
    );

    // `--help` prints that header, and an act that is none is refused in one line.
    let rig = StepRig::new("usage");
    let help = rig
        .hermetic(Command::new(rig.root.join(TOOL)))
        .arg("--help")
        .output()
        .expect("run the tool");
    assert!(help.status.success());
    assert!(String::from_utf8_lossy(&help.stdout).starts_with("\ndev/stabilize-step — "));
    rig.step(&["rebase"]).refused("usage");
    rig.step(&[]).refused("usage");
}

// ---------------------------------------------------------------------------
// The tool can do nothing an agent may not: the source scan
// ---------------------------------------------------------------------------

/// The flags a git subcommand of the tool may carry. A subcommand that is not here is one
/// the tool does not run — `rebase`, `reset`, `pull`, `stash`, `tag`, `checkout` — and a
/// flag that is not here is one it does not pass: `--force`, `-D`, `--delete`, `--hard`.
const ALLOWED: &[(&str, &[&str])] = &[
    (
        "branch",
        &["--show-current", "--list", "--format=%(refname:short)"],
    ),
    ("status", &["--porcelain", "--untracked-files=all"]),
    ("ls-remote", &["--exit-code", "--heads"]),
    ("fetch", &[]),
    ("merge-base", &["--is-ancestor"]),
    (
        "log",
        &[
            "--diff-filter=A",
            "--format=%H",
            "--format=%s",
            "-n",
            "--first-parent",
            "--no-merges",
            "--merges",
            "--reverse",
            "--",
        ],
    ),
    ("show", &["--stat", "--format=%s", "--"]),
    ("rev-parse", &["--verify", "--quiet"]),
    ("rev-list", &["--merges", "--count", "--parents", "-n"]),
    ("diff", &["--name-only", "--diff-filter=U", "--quiet", "--"]),
    ("switch", &["--no-track", "-c"]),
    ("push", &[]),
    ("merge", &["--no-ff", "--no-edit", "--abort"]),
    ("commit", &["--no-edit", "-q", "-m"]),
    ("add", &["--"]),
    ("cherry-pick", &["-x", "--abort"]),
];

/// The subcommands that change a branch, a commit or a remote, and every shape the tool
/// may give one: a literal stands for itself, `<…>` for one of the names below.
const CHANGING: &[(&str, &[&str])] = &[
    ("fetch", &["fetch origin <branch>", "fetch origin main"]),
    (
        "switch",
        &["switch <branch>", "switch --no-track -c <branch> <branch>"],
    ),
    ("push", &["push origin <branch>"]),
    (
        "merge",
        &["merge --no-ff --no-edit <target>", "merge --abort"],
    ),
    ("commit", &["commit --no-edit", "commit -q -m <subject>"]),
    ("add", &["add -- <paths>"]),
    (
        "cherry-pick",
        &["cherry-pick -x <sha>", "cherry-pick --abort"],
    ),
];

/// The names a changing command may take: an argument of the branch grammar, or the
/// parameter of a helper whose every caller passes one.
const BRANCH_NAMES: [&str; 3] = ["args.loop", "args.branch", "branch"];

/// The pieces of text the tool may join a name to, inside a git call.
const JOINED: [&str; 5] = ["origin/", "refs/heads/", "*", "/opening.md", ":(exclude)"];

#[derive(Debug, Clone, PartialEq)]
enum Arg {
    /// One string literal, and nothing else.
    Literal(String),
    /// Anything else, as written.
    Expression(String),
}

#[derive(Debug)]
struct Call {
    /// The top-level function the call stands in.
    within: String,
    name: String,
    args: Vec<Arg>,
}

/// The tool's code: no comment line.
fn python_code(source: &str) -> String {
    source
        .lines()
        .filter(|line| !line.trim_start().starts_with('#'))
        .map(|line| format!("{line}\n"))
        .collect()
}

/// The string literals of a piece of python: double-quoted, as the tool writes them.
fn literals(text: &str) -> Vec<String> {
    let mut found = Vec::new();
    let mut chars = text.chars();
    while let Some(c) = chars.next() {
        if c == '"' {
            let mut literal = String::new();
            while let Some(c) = chars.next() {
                match c {
                    '\\' => literal.extend(chars.next()),
                    '"' => break,
                    c => literal.push(c),
                }
            }
            found.push(literal);
        }
    }
    found
}

/// Every call of a function named in `names`, with its arguments split at the top level.
fn calls(code: &str, names: &[&str]) -> Vec<Call> {
    let mut found = Vec::new();
    let mut within = String::new();
    let mut offset = 0;
    for line in code.split_inclusive('\n') {
        if let Some(rest) = line.strip_prefix("def ") {
            within = rest[..rest.find('(').expect("a def has parameters")].to_owned();
        }
        for name in names {
            let needle = format!("{name}(");
            let mut from = 0;
            while let Some(at) = line[from..].find(&needle) {
                let start = from + at;
                from = start + needle.len();
                let before = line[..start].chars().next_back();
                if before.is_some_and(|c| c.is_alphanumeric() || c == '_' || c == '.')
                    || line[..start].ends_with("def ")
                {
                    continue;
                }
                // The arguments: up to the parenthesis that closes the call, which may
                // lie on a later line.
                let rest = &code[offset + from..];
                let mut args = Vec::new();
                let mut current = String::new();
                let (mut depth, mut quoted, mut escaped) = (1, false, false);
                for c in rest.chars() {
                    if quoted {
                        current.push(c);
                        if escaped {
                            escaped = false;
                        } else if c == '\\' {
                            escaped = true;
                        } else if c == '"' {
                            quoted = false;
                        }
                        continue;
                    }
                    match c {
                        '"' => quoted = true,
                        '(' | '[' | '{' => depth += 1,
                        ')' | ']' | '}' => depth -= 1,
                        _ => {}
                    }
                    if depth == 0 {
                        break;
                    }
                    if c == ',' && depth == 1 {
                        args.push(std::mem::take(&mut current));
                    } else {
                        current.push(c);
                    }
                }
                args.push(current);
                let args = args
                    .iter()
                    .map(|arg| arg.trim())
                    .filter(|arg| !arg.is_empty())
                    .map(|arg| {
                        let inner = literals(arg);
                        let whole = arg.starts_with('"')
                            && arg.ends_with('"')
                            && inner.len() == 1
                            && arg.len() == inner[0].len() + 2;
                        if whole {
                            Arg::Literal(inner[0].clone())
                        } else {
                            Arg::Expression(arg.to_owned())
                        }
                    })
                    .collect();
                found.push(Call {
                    within: within.clone(),
                    name: (*name).to_owned(),
                    args,
                });
            }
        }
        offset += line.len();
    }
    found
}

/// The spellings of a forced push, as the definition of the agent that runs the steps
/// names them: the code spans of its *never forced (…)*.
fn forced_spellings() -> Vec<String> {
    let definition = read(GIT_DEFINITION);
    let at = definition
        .find("never forced (")
        .expect("build-git.md says what a forced push is spelled like");
    let list = &definition[at..];
    let list = &list[..list.find(')').expect("the list closes")];
    let spellings: Vec<String> = list
        .split('`')
        .skip(1)
        .step_by(2)
        .map(str::to_owned)
        .collect();
    assert!(
        spellings.contains(&"--force".to_owned()) && spellings.contains(&"+".to_owned()),
        "the scan found the spellings: {spellings:?}"
    );
    spellings
}

/// Everything in the tool's source that an agent may not do — or nothing.
fn offences(source: &str) -> Vec<String> {
    let code = python_code(source);
    let mut found = Vec::new();
    let allowed: BTreeMap<&str, &[&str]> = ALLOWED.iter().copied().collect();
    let changing: BTreeMap<&str, &[&str]> = CHANGING.iter().copied().collect();

    // A forced push, in any spelling, in any string of the tool.
    let forced = forced_spellings();
    for literal in literals(&code) {
        for spelling in &forced {
            let hit = if spelling.len() == 1 {
                literal.starts_with(spelling.as_str())
            } else if spelling.starts_with("--") {
                literal.contains(spelling.as_str())
            } else {
                literal == *spelling
            };
            if hit {
                found.push(format!(
                    "the string \"{literal}\" spells a forced push (`{spelling}`)"
                ));
            }
        }
    }

    // Every git call: its subcommand, its flags, what it joins a name to, and — for a
    // command that changes something — its whole shape.
    for call in calls(&code, &["git", "said"]) {
        let shown = format!(
            "`{}({})` in `{}`",
            call.name,
            shape(&call.args),
            call.within
        );
        // `said` is `git` with a check on its exit status: it hands its own arguments on.
        if call.within == "said" && call.args == [Arg::Expression("*argv".to_owned())] {
            continue;
        }
        let Some(Arg::Literal(subcommand)) = call.args.first() else {
            found.push(format!(
                "{shown}: a git call whose subcommand is not a literal"
            ));
            continue;
        };
        let Some(flags) = allowed.get(subcommand.as_str()) else {
            found.push(format!("{shown}: the tool runs `git {subcommand}`"));
            continue;
        };
        for arg in &call.args[1..] {
            match arg {
                Arg::Literal(text) if text.starts_with('-') => {
                    if !flags.contains(&text.as_str()) {
                        found.push(format!("{shown}: `git {subcommand}` is passed `{text}`"));
                    }
                }
                Arg::Literal(text) if text.contains("main") => {
                    let fetched = subcommand == "fetch" && text == "main";
                    let read = text == "origin/main"
                        && ["rev-parse", "merge-base"].contains(&subcommand.as_str());
                    if call.within != "sync_main" || !(fetched || read) {
                        found.push(format!("{shown}: `main` outside the sync act's fetch and its reads of `origin/main`"));
                    }
                }
                Arg::Literal(_) => {}
                Arg::Expression(text) => {
                    for piece in literals(text) {
                        if !JOINED.contains(&piece.as_str()) {
                            found.push(format!("{shown}: a name is joined to \"{piece}\""));
                        }
                    }
                }
            }
        }
        if let Some(shapes) = changing.get(subcommand.as_str()) {
            let fits = shapes.iter().any(|allowed| {
                let allowed: Vec<&str> = allowed.split(' ').collect();
                allowed.len() == call.args.len()
                    && allowed
                        .iter()
                        .zip(&call.args)
                        .all(|(want, got)| match (*want, got) {
                            ("<branch>", Arg::Expression(name)) => {
                                BRANCH_NAMES.contains(&name.as_str())
                            }
                            ("<target>", Arg::Expression(name)) => name == "target",
                            ("<sha>", Arg::Expression(name)) => name == "sha",
                            // The record's commit: the subject its batch carries, and
                            // the run's pending paths, each by name.
                            ("<subject>", Arg::Expression(name)) => name == "subject",
                            ("<paths>", Arg::Expression(name)) => name == "*paths",
                            (literal, Arg::Literal(text)) => text == literal,
                            _ => false,
                        })
            });
            if !fits {
                found.push(format!(
                    "{shown}: `git {subcommand}` in a shape the tool does not have"
                ));
            }
        }
    }

    // A helper that takes a branch, or a merge's target, is handed a checked name by every
    // caller — the helpers derived from their `def` lines.
    let helpers: Vec<(String, bool)> = code
        .lines()
        .filter_map(|line| line.strip_prefix("def "))
        .filter_map(|rest| {
            let (name, parameters) = rest.split_once('(')?;
            let first = parameters.split([',', ')']).next()?.trim();
            ["branch", "target"]
                .contains(&first)
                .then(|| (name.to_owned(), first == "target"))
        })
        .collect();
    if helpers.len() < 3 {
        found.push(format!(
            "the scan found the helpers that take a branch: {helpers:?}"
        ));
    }
    for (helper, merges) in &helpers {
        for call in calls(&code, &[helper.as_str()]) {
            let handed = match call.args.first() {
                Some(Arg::Expression(text)) => {
                    ["args.loop", "args.branch"].contains(&text.as_str())
                }
                Some(Arg::Literal(text)) => {
                    *merges && text == "origin/main" && call.within == "sync_main"
                }
                None => false,
            };
            if !handed {
                found.push(format!(
                    "`{helper}({})` in `{}`: a name nobody checked",
                    shape(&call.args),
                    call.within
                ));
            }
        }
    }
    // A sha that is picked is one of the act's own arguments.
    if code.matches("for sha in ").count() != 2
        || !code.contains("    for sha in args.shas:\n")
        || !code.contains(" for sha in direct]")
    {
        found.push("`sha` is bound somewhere else than over the carry's arguments and the path-class assert's listing".to_owned());
    }
    // And the grammar those arguments are checked against requires a prefix.
    for required in [
        "BRANCH = re.compile(r\"[a-z0-9]+(?:-[a-z0-9]+)*(?:/[a-z0-9]+(?:-[a-z0-9]+)*)+\")\n",
        "branch_name = grammar(BRANCH, ",
        "    \"loop\": branch_name,\n",
        "    \"branch\": branch_name,\n",
        "full_sha = grammar(FULL_SHA, ",
        "names=(\"shas\", argparse.ONE_OR_MORE, full_sha)",
        "            sub.add_argument(\"--\" + flag, required=True, type=FLAGS[flag])\n",
    ] {
        if !code.contains(required) {
            found.push(format!(
                "the grammar of a name: the tool no longer holds `{}`",
                required.trim()
            ));
        }
    }

    // One road to a process, and three programs on it: git, and the two scripts.
    for (n, line) in code.lines().enumerate() {
        for forbidden in [
            "os.system",
            "os.popen",
            "os.exec",
            "os.spawn",
            "shell=True",
            "pty.",
            "shutil",
            "os.remove",
            "os.unlink",
            "os.rmdir",
            "os.rename",
            "os.replace",
            "os.chmod",
            "eval(",
            "exec(",
            "__import__",
            "importlib",
        ] {
            if line.contains(forbidden) {
                found.push(format!("line {}: `{forbidden}`: {}", n + 1, line.trim()));
            }
        }
    }
    let spawning: Vec<&str> = code
        .lines()
        .filter(|line| line.contains("subprocess") && !line.starts_with("import "))
        .collect();
    if spawning.len() != 1
        || !spawning[0]
            .trim_start()
            .starts_with("done = subprocess.run(argv, cwd=ROOT, ")
    {
        found.push(format!(
            "a process is started somewhere else than in `run`: {spawning:#?}"
        ));
    }
    for call in calls(&code, &["run"]) {
        let program = match call.args.first() {
            Some(Arg::Expression(text)) => text.clone(),
            other => format!("{other:?}"),
        };
        let known = (call.within == "git" && program == "[\"git\"] + list(argv)")
            || program == "[MERGE_LOGS]"
            || program.starts_with("[RECORD, \"state\", ")
            || program.starts_with("[RECORD, \"check-reports\"] + ")
            || program.starts_with("[RECORD, \"pending\", ")
            || program.starts_with("[RECORD, \"gate-check\", ")
            || program.starts_with("[RECORD, \"settle\", ")
            // The one write the tool makes through the record script: an attempt's marker,
            // by the act that begins the attempt, under the reporter it was handed.
            || (call.within == "begin"
                && program.starts_with(
                    "[RECORD, \"report\"] + stage_flags(args) + [\"--reporter\", args.reporter, ",
                ));
        if !known {
            found.push(format!(
                "`run({program})` in `{}`: a program the tool does not run",
                call.within
            ));
        }
    }
    let writes: Vec<&str> = code.lines().filter(|line| line.contains("open(")).collect();
    if writes.len() != 2 || writes.iter().filter(|line| line.contains("\"w\"")).count() != 1 {
        found.push(format!(
            "the tool opens a file somewhere else than its help and the kept state: {writes:#?}"
        ));
    }
    found
}

/// A call's arguments as one line: a literal as itself, anything else as written.
fn shape(args: &[Arg]) -> String {
    args.iter()
        .map(|arg| match arg {
            Arg::Literal(text) | Arg::Expression(text) => text.as_str(),
        })
        .collect::<Vec<_>>()
        .join(" ")
}

#[test]
fn the_tool_runs_no_git_command_an_agent_may_not() {
    let source = read(TOOL);
    assert_eq!(
        offences(&source),
        Vec::<String>::new(),
        "{TOOL} does something an agent may not — or the scan no longer reads it"
    );
    // The scan read the tool: every subcommand it may run is one it does run, and the
    // calls are found in the acts.
    let found = calls(&python_code(&source), &["git", "said"]);
    assert!(
        found.len() >= 60,
        "the scan found the tool's git calls: {}",
        found.len()
    );
    let run: BTreeSet<String> = found
        .iter()
        .filter_map(|call| match call.args.first() {
            Some(Arg::Literal(subcommand)) => Some(subcommand.clone()),
            _ => None,
        })
        .collect();
    let allowed: BTreeSet<String> = ALLOWED.iter().map(|(name, _)| (*name).to_owned()).collect();
    assert_eq!(
        run, allowed,
        "the subcommands the tool runs (left) are the ones it may (right)"
    );
    let pushes: Vec<String> = found
        .iter()
        .filter(|call| call.args.first() == Some(&Arg::Literal("push".to_owned())))
        .map(|call| format!("{} in {}", shape(&call.args), call.within))
        .collect();
    assert_eq!(
        pushes,
        ["push origin branch in push"],
        "one push, by name, in one helper"
    );
}

#[test]
fn a_planted_offender_reddens_the_scan() {
    let source = read(TOOL);
    let anchor = "def push_branch(args, facts):\n";
    assert_eq!(
        source.matches(anchor).count(),
        1,
        "the act the offenders are planted in"
    );
    // Each line is planted alone, as the first statement of an act.
    let planted: &[(&str, &str)] = &[
        (
            "git(\"push\", \"--force\", \"origin\", args.branch)",
            "forced push",
        ),
        (
            "git(\"push\", \"-f\", \"origin\", args.branch)",
            "forced push",
        ),
        (
            "git(\"push\", \"--force-with-lease\", \"origin\", args.branch)",
            "forced push",
        ),
        (
            "git(\"push\", \"origin\", \"+\" + args.branch)",
            "forced push",
        ),
        (
            "git(\"push\", \"origin\", \"--delete\", args.branch)",
            "`--delete`",
        ),
        (
            "git(\"push\", \"origin\", \":\" + args.branch)",
            "joined to \":\"",
        ),
        (
            "git(\"push\", \"origin\", args.branch + \":main\")",
            "joined to \":main\"",
        ),
        (
            "git(\"push\", \"origin\", \"main\")",
            "`main` outside the sync act",
        ),
        (
            "git(\"push\", \"origin\", args.rounds)",
            "a shape the tool does not have",
        ),
        ("git(\"push\", \"--all\", \"origin\")", "`--all`"),
        ("git(\"push\", \"--tags\", \"origin\")", "`--tags`"),
        ("git(\"rebase\", args.branch)", "`git rebase`"),
        ("git(\"reset\", \"--hard\", \"HEAD\")", "`git reset`"),
        ("git(\"pull\", \"origin\", args.branch)", "`git pull`"),
        ("git(\"pull\")", "`git pull`"),
        ("git(\"stash\")", "`git stash`"),
        ("git(\"branch\", \"-D\", args.branch)", "`-D`"),
        ("git(\"branch\", \"-d\", args.branch)", "`-d`"),
        ("git(\"branch\", \"--delete\", args.branch)", "`--delete`"),
        ("git(\"tag\", \"-d\", \"v1\")", "`git tag`"),
        ("git(\"checkout\", \"main\")", "`git checkout`"),
        ("said(\"switch\", \"main\")", "`main` outside the sync act"),
        ("said(\"switch\", \"-C\", args.branch, args.loop)", "`-C`"),
        (
            "git(\"merge\", \"--no-ff\", \"--no-edit\", \"main\")",
            "`main` outside the sync act",
        ),
        (
            "git(\"merge\", \"--no-ff\", \"--no-edit\", args.branch)",
            "a shape the tool does not have",
        ),
        (
            "git(\"merge\", \"--ff-only\", \"origin/main\")",
            "`--ff-only`",
        ),
        (
            "said(\"fetch\", \"origin\", \"main:main\")",
            "`main` outside the sync act",
        ),
        (
            "said(\"fetch\", \"origin\", \"main\")",
            "`main` outside the sync act",
        ),
        ("git(\"commit\", \"--amend\", \"--no-edit\")", "`--amend`"),
        ("git(\"commit\", \"--no-edit\", \"-a\")", "`-a`"),
        (
            "git(\"cherry-pick\", \"-x\", args.branch)",
            "a shape the tool does not have",
        ),
        ("git(args.branch)", "subcommand is not a literal"),
        ("push(\"main\", facts)", "a name nobody checked"),
        ("push(args.rounds, facts)", "a name nobody checked"),
        ("merge(\"main\", [], facts)", "a name nobody checked"),
        ("merge(\"origin/main\", [], facts)", "a name nobody checked"),
        (
            "subprocess.run([\"git\", \"push\", \"--force\"])",
            "a process is started somewhere else",
        ),
        ("os.system(\"git reset --hard\")", "`os.system`"),
        (
            "run([\"git\", \"rebase\", \"main\"])",
            "a program the tool does not run",
        ),
        (
            "run([\"sh\", \"-c\", \"git pull\"])",
            "a program the tool does not run",
        ),
        ("os.remove(args.branch)", "`os.remove`"),
    ];
    for (line, names) in planted {
        let mutant = source.replacen(anchor, &format!("{anchor}    {line}\n"), 1);
        let found = offences(&mutant);
        assert!(
            found.iter().any(|offence| offence.contains(names)),
            "the scan must name `{line}` ({names}); it found: {found:#?}"
        );
    }
    // And the grammar a name is checked against: a branch without a prefix, or a name
    // that is no longer checked at all.
    for (from, to) in [
        (
            "(?:/[a-z0-9]+(?:-[a-z0-9]+)*)+\")",
            "(?:/[a-z0-9]+(?:-[a-z0-9]+)*)*\")",
        ),
        ("    \"loop\": branch_name,\n", "    \"loop\": str,\n"),
        ("required=True, type=FLAGS[flag])", "required=True)"),
        (
            "names=(\"shas\", argparse.ONE_OR_MORE, full_sha)",
            "names=(\"shas\", argparse.ONE_OR_MORE, str)",
        ),
    ] {
        assert_eq!(source.matches(from).count(), 1, "`{from}` stands once");
        let found = offences(&source.replacen(from, to, 1));
        assert!(
            found
                .iter()
                .any(|offence| offence.contains("the grammar of a name")),
            "the scan must name the grammar without `{from}`; it found: {found:#?}"
        );
    }
}

// ---------------------------------------------------------------------------
// The harness and the tool: one contract (where node is installed)
// ---------------------------------------------------------------------------

/// Calls the harness's pure functions — everything above its first statement that runs,
/// which the harness's own fence holds to launching nothing — and prints what they return.
const PURE: &str = r#"
import { readFileSync } from 'node:fs'
const [script, callsJson] = process.argv.slice(2)
const source = readFileSync(script, 'utf8').replace(/^export const meta = /m, 'const meta = ')
const pure = source.slice(0, source.indexOf('\n// ---- args: parsed, and refused before any agent ----\n'))
const calls = JSON.parse(callsJson)
const names = [...new Set(calls.map((call) => call[0]))]
const made = new Function(pure + '\nreturn { ' + names.join(', ') + ' }')()
console.log(JSON.stringify(calls.map(([name, args]) => made[name](...args))))
"#;

/// The file the gate names for the tests that pass without running what they test
/// (`dev/gate`'s header: *PASSED WITHOUT RUNNING*): one line per test, which the gate
/// counts and prints above its verdict.
const GATE_SKIPS: &str = "JIGC_GATE_SKIPS";

/// Whether `node` is there to run the harness under — and, where it is not, what a test
/// that needs it does: `unrun` says what was not run on this machine.
///
/// **Under CI it fails.** GitHub's hosted runners ship `node`, so a runner without it is a
/// job that changed, and a suite that passed there by running nothing would be the gap the
/// suite exists to close. **Anywhere else it returns false, and the caller passes** — the
/// gate gains no dependency — **but not silently**: a test runner shows a passing test's
/// stderr to nobody, so beside the line on stderr the skip is written to the file the gate
/// names, and the gate prints it in its own summary.
pub(crate) fn node_or_skip(unrun: &str) -> bool {
    let answers = Command::new("node")
        .arg("--version")
        .output()
        .is_ok_and(|out| out.status.success());
    if answers {
        return true;
    }
    assert!(
        std::env::var_os("CI").is_none_or(|ci| ci.is_empty()),
        "`node` is not on PATH, and the environment says this is CI: {unrun}. A hosted \
         runner ships `node`; a job that lacks it installs it (`actions/setup-node`) — \
         the suite is never left to pass there without running"
    );
    let skipped = format!("SKIPPED: `node` is not on PATH, so {unrun}");
    eprintln!("{skipped}");
    if let Some(skips) = std::env::var_os(GATE_SKIPS) {
        let mut file = fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(&skips)
            .unwrap_or_else(|e| panic!("open the gate's skip file: {e}"));
        writeln!(file, "{skipped}").expect("write the skip to the gate's file");
    }
    false
}

/// The harness's own pure functions, called under `node`: one result per `[name, args]`.
pub(crate) fn harness_pure(calls: &Value) -> Vec<Value> {
    let scratch = ScratchDir::new("stabilize-step-pure");
    let driver = scratch.path().join("pure.mjs");
    fs::write(&driver, PURE).expect("write the driver");
    let out = Command::new("node")
        .arg(&driver)
        .arg(repo_root().join(HARNESS))
        .arg(calls.to_string())
        .output()
        .expect("run node");
    assert!(
        out.status.success(),
        "the harness's pure functions under {calls}: {}",
        String::from_utf8_lossy(&out.stderr)
    );
    serde_json::from_slice(&out.stdout).expect("the driver prints one JSON list")
}

/// The one command of a git step's prompt: the code span of its first numbered line.
pub(crate) fn command_of(prompt: &str) -> &str {
    let line = prompt
        .lines()
        .find(|line| line.starts_with("1. `"))
        .unwrap_or_else(|| panic!("a git step's prompt lists its command first: {prompt}"));
    let command = &line["1. `".len()..];
    &command[..command.rfind('`').expect("the code span closes")]
}

/// A `build-git` agent, as a function: the one command of the step's prompt, run as
/// written from the repository's root, and its one line relayed — what the agent returns.
pub(crate) fn git_agent(rig: &StepRig, prompt: &str) -> Value {
    json!({"status": "ran", "line": rig.shell(command_of(prompt)).raw})
}

#[test]
fn every_command_the_harness_composes_is_taken_by_the_tool_and_read_back() {
    if !node_or_skip(&format!(
        "the commands {HARNESS} composes were not run against {TOOL}"
    )) {
        return;
    }
    let rig = StepRig::new("harness");
    let scratch = rig.scratch.display().to_string();
    let v = |stage: &str| json!({"run": RUN, "stage": stage, "scratch": scratch});
    let branch = format!("{ROUNDS}1");
    let dropped = format!("{ROUNDS}2");
    let ctx = json!({"run": RUN, "round": 1, "stage": "test", "attempt": 1, "scratch": scratch});

    // One step: the harness composes it, the agent runs its one command, the harness
    // reads the line — each by its own code.
    let step = |act: &str, function: &str, args: Value| -> Value {
        let prompt = harness_pure(&json!([[function, args]]))
            .remove(0)
            .as_str()
            .unwrap_or_else(|| panic!("`{function}` returns a prompt"))
            .to_owned();
        assert!(
            command_of(&prompt).starts_with(&format!("{TOOL} {act} ")),
            "`{function}` composes ONE command of the tool: {prompt}"
        );
        let relayed = git_agent(&rig, &prompt);
        let read = harness_pure(&json!([["readStep", [relayed, act]]])).remove(0);
        assert!(
            read.get("relay").is_none() && read["act"] == act,
            "`readStep` reads the line `{function}`'s command printed: {read}"
        );
        read
    };

    let state = step("git-state", "gitStatePrompt", json!([v("test")]));
    assert_eq!(state["status"], "ready", "{state}");
    assert_eq!(state["branch"], LOOP);
    let read = step("state", "statePrompt", json!([v("test"), "test-1"]));
    assert_eq!(read["status"], "read", "{read}");
    assert_eq!(read["state"]["opened"], true);
    // The step that begins an attempt: the tool takes the command as composed, and its
    // answer is read back — here its refusal, because this rig's run has an opening record
    // and no fact, and no stage begins before the opening is done. (The act writes through
    // the record script, whose scanners this rig lacks: it is driven to its end in
    // [`stabilize_simulation`](super::stabilize_simulation).)
    let begun = step("begin", "beginPrompt", json!([ctx, rig.rev("HEAD")]));
    assert_eq!(
        json!([begun["status"], begun["refused"]]),
        json!(["halted", "position"]),
        "{begun}"
    );
    let check = step(
        "check-reports",
        "checkReportsPrompt",
        json!([ctx, ["scope", "preflight"]]),
    );
    assert_eq!(check["status"], "checked", "{check}");
    assert_eq!(check["check"]["missing"], json!(["scope", "preflight"]));
    let found = step("find-round", "findRoundPrompt", json!([v("fix"), 1]));
    assert_eq!(found["local"], json!([]), "{found}");
    let opened = step(
        "open-round",
        "openRoundPrompt",
        json!([v("fix"), 1, branch]),
    );
    assert_eq!(opened["created"], true, "{opened}");
    rig.change("crates/a.txt", "fixed\n", "fix: a finding");
    let pushed = step("push", "pushPrompt", json!([v("fix"), branch]));
    assert_eq!(pushed["remote_head"], pushed["head"], "{pushed}");
    let listed = step(
        "round-commits",
        "roundCommitsPrompt",
        json!([v("fix"), 1, branch]),
    );
    assert_eq!(listed["all"], json!([rig.rev(&branch)]), "{listed}");
    let landed = step("land", "landPrompt", json!([v("fix"), 1, branch]));
    assert_eq!(landed["status"], "merged", "{landed}");
    assert_eq!(landed["remote_head"], landed["merge_commit"]);
    let again = step("land", "landPrompt", json!([v("fix"), 1, branch]));
    assert_eq!(again["status"], "landed-before", "{again}");
    let after = step("git-state", "gitStatePrompt", json!([v("fix")]));
    assert_eq!(
        after["status"], "ready",
        "a landed round passes the path-class assert: {after}"
    );

    // A second round, dropped: its record commit is carried over.
    step(
        "open-round",
        "openRoundPrompt",
        json!([v("fix"), 2, dropped]),
    );
    let record = rig.change(
        &format!("{RUN_DIR}/r2/round.md"),
        "r\n",
        "docs(record): dropped",
    );
    let carried = step("carry", "carryPrompt", json!([v("fix"), 2, [record], null]));
    assert_eq!(carried["status"], "carried", "{carried}");
    assert_eq!(carried["picked"][0]["from"], record.as_str());
    let synced = step("sync-main", "syncMainPrompt", json!([RUN]));
    assert_eq!(synced["status"], "up-to-date", "{synced}");

    // A refusal is read back as a halted step that says why — and a line that was
    // retyped on its way is not the tool's.
    rig.write("notes.txt", "x\n");
    let refused = step("git-state", "gitStatePrompt", json!([v("test")]));
    assert_eq!(refused["status"], "halted", "{refused}");
    assert_eq!(refused["refused"], "dirty");
    assert!(
        refused["halt"]["root_cause"]
            .as_str()
            .is_some_and(|cause| cause.starts_with("dirty: "))
    );
    let line = rig.shell(&format!("{TOOL} push --branch {LOOP}")).raw;
    let head = rig.rev("HEAD");
    let retyped = line.replacen(&head, &head.chars().rev().collect::<String>(), 1);
    assert_ne!(retyped, line);
    for (what, relayed, act) in [
        (
            "a sha retyped",
            json!({"status": "ran", "line": retyped}),
            "push",
        ),
        (
            "another act's line",
            json!({"status": "ran", "line": line}),
            "land",
        ),
        ("no line", json!({"status": "ran"}), "push"),
        (
            "no JSON",
            json!({"status": "ran", "line": "pushed"}),
            "push",
        ),
    ] {
        let read = harness_pure(&json!([["readStep", [relayed, act]]])).remove(0);
        assert!(
            read["status"] == "halted" && read["relay"].is_string(),
            "{what}: `readStep` says the line is not the command's: {read}"
        );
    }

    // UNCONVERTED ON PURPOSE: the re-cut of a part is still a list of commands the agent
    // follows — the exit is ruled to be rebuilt as a revert (DECISIONS.md → 2026-10-06,
    // *a part lands by revert*), and a mechanism about to be deleted is not ported.
    let part = harness_pure(&json!([[
        "carryPrompt",
        [v("fix"), 2, [record], format!("{dropped}-part1")]
    ]]))
    .remove(0);
    let part = part.as_str().expect("a prompt");
    assert!(
        part.starts_with("GIT STEP — re-cut a part of round 2")
            && part.contains("`git cherry-pick -x <sha>`")
            && !part.contains(TOOL),
        "the part's re-cut is the list it was: {part}"
    );
}
